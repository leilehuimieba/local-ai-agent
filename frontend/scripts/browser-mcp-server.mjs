import { createServer } from "node:http"
import { basename } from "node:path"
import { chromium } from "@playwright/test"

const port = readPort()
const pages = new Map()
let browser = null
let context = null
let nextPageID = 1

const server = createServer(async (req, res) => {
  try {
    await route(req, res)
  } catch (error) {
    writeJSON(res, 500, rpcError(null, -32603, errorMessage(error)))
  }
})

server.listen(port, "127.0.0.1", () => {
  console.log(`[browser-mcp] listening on http://127.0.0.1:${port}/mcp`)
})

process.on("SIGINT", shutdown)
process.on("SIGTERM", shutdown)

async function route(req, res) {
  if (req.method === "GET" && req.url === "/health") {
    return writeJSON(res, 200, { ok: true, port })
  }
  if (req.method === "POST" && req.url === "/mcp") {
    return handleRPC(req, res)
  }
  writeJSON(res, 404, rpcError(null, -32601, "not found"))
}

async function handleRPC(req, res) {
  const payload = await readJSON(req)
  const method = payload?.method ?? ""
  if (method === "initialize") {
    return writeJSON(res, 200, rpcOK(payload.id, initializeResult()))
  }
  if (method === "tools/list") {
    return writeJSON(res, 200, rpcOK(payload.id, { tools: listTools() }))
  }
  if (method === "tools/call") {
    return writeJSON(res, 200, rpcOK(payload.id, await callTool(payload.params)))
  }
  writeJSON(res, 200, rpcError(payload?.id ?? null, -32601, `method not found: ${method}`))
}

function initializeResult() {
  return {
    protocolVersion: "2024-11-05",
    serverInfo: { name: "local-browser-mcp", version: "0.1.0" },
    capabilities: { tools: {} },
  }
}

function listTools() {
  return [openPageTool(), readPageTool(), clickTool(), typeTool(), selectTool(), submitTool(), uploadTool()]
}

function openPageTool() {
  return {
    name: "open_page",
    description: "Open a page and return navigation metadata.",
    inputSchema: {
      type: "object",
      properties: {
        url: { type: "string" },
        wait_until: { type: "string", enum: ["domcontentloaded", "load", "networkidle"] },
        timeout_ms: { type: "integer", minimum: 1000, maximum: 30000 },
      },
      required: ["url"],
    },
  }
}

function readPageTool() {
  return {
    name: "read_page",
    description: "Read the current page into structured text.",
    inputSchema: {
      type: "object",
      properties: {
        page_id: { type: "string" },
        format: { type: "string", enum: ["markdown", "text"] },
        max_chars: { type: "integer", minimum: 1000, maximum: 20000 },
      },
      required: ["page_id"],
    },
  }
}

function clickTool() {
  return {
    name: "click",
    description: "Click a visible element on the page.",
    inputSchema: {
      type: "object",
      properties: {
        page_id: { type: "string" },
        selector: { type: "string" },
        timeout_ms: { type: "integer", minimum: 1000, maximum: 30000 },
      },
      required: ["page_id", "selector"],
    },
  }
}

function typeTool() {
  return {
    name: "type",
    description: "Type text into a visible input or textarea.",
    inputSchema: {
      type: "object",
      properties: {
        page_id: { type: "string" },
        selector: { type: "string" },
        text: { type: "string" },
        submit: { type: "boolean" },
        timeout_ms: { type: "integer", minimum: 1000, maximum: 30000 },
      },
      required: ["page_id", "selector", "text"],
    },
  }
}

function selectTool() {
  return {
    name: "select",
    description: "Select an option in a visible select element.",
    inputSchema: {
      type: "object",
      properties: {
        page_id: { type: "string" },
        selector: { type: "string" },
        value: { type: "string" },
        timeout_ms: { type: "integer", minimum: 1000, maximum: 30000 },
      },
      required: ["page_id", "selector", "value"],
    },
  }
}

function submitTool() {
  return {
    name: "submit",
    description: "Submit a visible form trigger such as a button.",
    inputSchema: {
      type: "object",
      properties: {
        page_id: { type: "string" },
        selector: { type: "string" },
        timeout_ms: { type: "integer", minimum: 1000, maximum: 30000 },
      },
      required: ["page_id", "selector"],
    },
  }
}

function uploadTool() {
  return {
    name: "upload",
    description: "Upload one or more local files into a visible file input.",
    inputSchema: {
      type: "object",
      properties: {
        page_id: { type: "string" },
        selector: { type: "string" },
        file_paths: { type: "array", items: { type: "string" }, minItems: 1 },
        timeout_ms: { type: "integer", minimum: 1000, maximum: 30000 },
      },
      required: ["page_id", "selector", "file_paths"],
    },
  }
}

async function callTool(params) {
  const name = params?.name ?? ""
  if (name === "open_page") {
    return openPage(params?.arguments)
  }
  if (name === "read_page") {
    return readPage(params?.arguments)
  }
  if (name === "click") {
    return clickPage(params?.arguments)
  }
  if (name === "type") {
    return typeIntoPage(params?.arguments)
  }
  if (name === "select") {
    return selectInPage(params?.arguments)
  }
  if (name === "submit") {
    return submitPage(params?.arguments)
  }
  if (name === "upload") {
    return uploadToPage(params?.arguments)
  }
  return businessError("browser_tool_not_found", `tool not found: ${name}`)
}

async function openPage(args) {
  const url = normalizeURL(args?.url)
  if (!url) {
    return businessError("browser_invalid_url", "url is required")
  }
  const started = Date.now()
  const page = await createPage()
  try {
    const response = await page.goto(url, gotoOptions(args))
    const pageID = registerPage(page)
    return {
      ok: true,
      page_id: pageID,
      requested_url: url,
      final_url: page.url(),
      title: await page.title(),
      status_code: response?.status() ?? 0,
      loaded: true,
      elapsed_ms: Date.now() - started,
    }
  } catch (error) {
    await closePage(page)
    return navigationError(error)
  }
}

async function readPage(args) {
  const pageID = stringValue(args?.page_id)
  const page = pageID ? pages.get(pageID) : null
  if (!page) {
    return businessError("browser_page_not_found", `page_id ${pageID || "unknown"} not found`)
  }
  const started = Date.now()
  try {
    const format = readFormat(args?.format)
    const snapshot = await pageSnapshot(page)
    const content = renderContent(snapshot, format)
    if (!content) {
      return businessError("browser_empty_document", "page content is empty")
    }
    const clipped = clipText(content, readMaxChars(args?.max_chars))
    return readResult(pageID, page.url(), snapshot.title, format, clipped, started)
  } catch (error) {
    return businessError("browser_read_failed", errorMessage(error))
  }
}

async function clickPage(args) {
  const resolved = resolvePageAction(args)
  if (resolved.error) {
    return resolved.error
  }
  const started = Date.now()
  try {
    const locator = await readyLocator(resolved.page, resolved.selector, actionTimeout(args))
    await locator.click({ timeout: actionTimeout(args) })
    return await actionResult(resolved, started, { clicked: true })
  } catch (error) {
    return actionError("click", error)
  }
}

async function typeIntoPage(args) {
  const resolved = resolvePageAction(args)
  const text = stringValue(args?.text)
  if (resolved.error) {
    return resolved.error
  }
  if (!text) {
    return businessError("browser_text_required", "text is required")
  }
  const started = Date.now()
  try {
    const locator = await readyLocator(resolved.page, resolved.selector, actionTimeout(args))
    await locator.fill(text, { timeout: actionTimeout(args) })
    await submitLocator(locator, args)
    return await actionResult(resolved, started, typedMeta(text, args))
  } catch (error) {
    return actionError("type", error)
  }
}

async function selectInPage(args) {
  const resolved = resolvePageAction(args)
  const value = stringValue(args?.value)
  if (resolved.error) {
    return resolved.error
  }
  if (!value) {
    return businessError("browser_value_required", "value is required")
  }
  const started = Date.now()
  try {
    const locator = await readyLocator(resolved.page, resolved.selector, actionTimeout(args))
    await locator.selectOption(value, { timeout: actionTimeout(args) })
    return await actionResult(resolved, started, { selected_value: value })
  } catch (error) {
    return actionError("select", error)
  }
}

async function submitPage(args) {
  const resolved = resolvePageAction(args)
  if (resolved.error) {
    return resolved.error
  }
  const started = Date.now()
  try {
    const locator = await readyLocator(resolved.page, resolved.selector, actionTimeout(args))
    await locator.click({ timeout: actionTimeout(args) })
    return await actionResult(resolved, started, { submitted: true })
  } catch (error) {
    return actionError("submit", error)
  }
}

async function uploadToPage(args) {
  const resolved = resolvePageAction(args)
  const filePaths = readUploadPaths(args?.file_paths)
  if (resolved.error) {
    return resolved.error
  }
  if (filePaths.length === 0) {
    return businessError("browser_file_required", "file_paths is required")
  }
  const started = Date.now()
  try {
    const locator = await readyLocator(resolved.page, resolved.selector, actionTimeout(args))
    await locator.setInputFiles(filePaths, { timeout: actionTimeout(args) })
    return await actionResult(resolved, started, uploadMeta(filePaths))
  } catch (error) {
    return actionError("upload", error)
  }
}

function gotoOptions(args) {
  return {
    waitUntil: readWaitUntil(args?.wait_until),
    timeout: boundedInt(args?.timeout_ms, 15000, 1000, 30000),
  }
}

function readResult(pageID, url, title, format, clipped, started) {
  return {
    ok: true,
    page_id: pageID,
    url,
    title,
    content_format: format,
    content: clipped.value,
    truncated: clipped.truncated,
    returned_char_count: clipped.value.length,
    elapsed_ms: Date.now() - started,
  }
}

function resolvePageAction(args) {
  const pageID = stringValue(args?.page_id)
  const selector = stringValue(args?.selector)
  const page = pageID ? pages.get(pageID) : null
  if (!page) {
    return { error: businessError("browser_page_not_found", `page_id ${pageID || "unknown"} not found`) }
  }
  if (!selector) {
    return { error: businessError("browser_selector_required", "selector is required") }
  }
  return { pageID, page, selector }
}

function actionTimeout(args) {
  return boundedInt(args?.timeout_ms, 12000, 1000, 30000)
}

async function actionResult(resolved, started, extra) {
  return {
    ok: true,
    page_id: resolved.pageID,
    url: resolved.page.url(),
    title: await resolved.page.title(),
    selector: resolved.selector,
    elapsed_ms: Date.now() - started,
    ...extra,
  }
}

function typedMeta(text, args) {
  return {
    typed_char_count: text.length,
    submitted: Boolean(args?.submit),
  }
}

function uploadMeta(filePaths) {
  return {
    uploaded_file_count: filePaths.length,
    uploaded_files: filePaths.map(fileLabel),
  }
}

function readUploadPaths(value) {
  if (!Array.isArray(value)) {
    return []
  }
  return value.map(stringValue).filter(Boolean)
}

function fileLabel(value) {
  return basename(value)
}

async function readyLocator(page, selector, timeout) {
  const locator = page.locator(selector).first()
  try {
    await locator.waitFor({ state: "visible", timeout })
    return locator
  } catch {
    throw new Error(`selector_not_found:${selector}`)
  }
}

async function submitLocator(locator, args) {
  if (!args?.submit) {
    return
  }
  await locator.press("Enter", { timeout: actionTimeout(args) })
}

function actionError(kind, error) {
  const message = errorMessage(error)
  if (message.startsWith("selector_not_found:")) {
    return businessError("browser_selector_not_found", message.replace("selector_not_found:", "selector not found: "))
  }
  if (message.toLowerCase().includes("timeout")) {
    return businessError("browser_action_timeout", message)
  }
  return businessError(`browser_${kind}_failed`, message)
}

function readWaitUntil(value) {
  return ["domcontentloaded", "networkidle"].includes(value) ? value : "load"
}

function readFormat(value) {
  return value === "text" ? "text" : "markdown"
}

function readMaxChars(value) {
  return boundedInt(value, 12000, 1000, 20000)
}

function boundedInt(value, fallback, min, max) {
  const parsed = Number(value)
  if (!Number.isFinite(parsed)) {
    return fallback
  }
  return Math.min(max, Math.max(min, Math.trunc(parsed)))
}

function normalizeURL(raw) {
  const value = stringValue(raw)
  if (!value) {
    return ""
  }
  return parseURL(value) ?? parseURL(`http://${value}`) ?? ""
}

function parseURL(value) {
  try {
    return new URL(value).toString()
  } catch {
    return null
  }
}

async function createPage() {
  const activeContext = await ensureContext()
  return activeContext.newPage()
}

async function ensureContext() {
  if (context) {
    return context
  }
  browser = await launchBrowser()
  context = await browser.newContext()
  return context
}

async function launchBrowser() {
  const channels = process.platform === "win32" ? ["msedge", "chrome", "chromium"] : ["chromium"]
  for (const channel of channels) {
    const launched = await tryLaunch(channel)
    if (launched) {
      return launched
    }
  }
  throw new Error("no browser launch candidate available")
}

async function tryLaunch(channel) {
  try {
    const options = channel === "chromium" ? { headless: true } : { channel, headless: true }
    return await chromium.launch(options)
  } catch {
    return null
  }
}

function registerPage(page) {
  const pageID = `page_${String(nextPageID).padStart(2, "0")}`
  nextPageID += 1
  pages.set(pageID, page)
  page.once("close", () => pages.delete(pageID))
  return pageID
}

async function pageSnapshot(page) {
  return page.evaluate(() => ({
    title: document.title || "",
    text: document.body?.innerText || "",
  }))
}

function renderContent(snapshot, format) {
  const text = normalizeText(snapshot?.text ?? "")
  if (!text) {
    return ""
  }
  return format === "text" ? text : markdownText(snapshot?.title ?? "", text)
}

function normalizeText(value) {
  return String(value).replace(/\r/g, "").replace(/\n{3,}/g, "\n\n").trim()
}

function markdownText(title, text) {
  const heading = stringValue(title)
  return heading ? `# ${heading}\n\n${text}` : text
}

function clipText(value, limit) {
  if (value.length <= limit) {
    return { value, truncated: false }
  }
  return { value: value.slice(0, limit), truncated: true }
}

function navigationError(error) {
  const message = errorMessage(error)
  if (message.toLowerCase().includes("timeout")) {
    return businessError("browser_navigation_timeout", message)
  }
  return businessError("browser_navigation_failed", message)
}

function businessError(code, message) {
  return { ok: false, error_code: code, error_message: message }
}

function rpcOK(id, result) {
  return { jsonrpc: "2.0", id, result }
}

function rpcError(id, code, message) {
  return { jsonrpc: "2.0", id, error: { code, message } }
}

function writeJSON(res, status, body) {
  res.statusCode = status
  res.setHeader("Content-Type", "application/json")
  res.end(JSON.stringify(body))
}

function readJSON(req) {
  return new Promise((resolve, reject) => {
    const chunks = []
    req.on("data", chunk => chunks.push(chunk))
    req.on("end", () => {
      try {
        resolve(JSON.parse(Buffer.concat(chunks).toString("utf8") || "{}"))
      } catch (error) {
        reject(error)
      }
    })
    req.on("error", reject)
  })
}

function readPort() {
  return boundedInt(process.env.LOCAL_AGENT_BROWSER_MCP_PORT, 3345, 1024, 65535)
}

function stringValue(value) {
  return typeof value === "string" ? value.trim() : ""
}

function errorMessage(error) {
  return error instanceof Error ? error.message : String(error)
}

async function closePage(page) {
  try {
    await page.close()
  } catch {}
}

async function shutdown() {
  for (const page of pages.values()) {
    await closePage(page)
  }
  pages.clear()
  if (context) {
    await context.close().catch(() => {})
  }
  if (browser) {
    await browser.close().catch(() => {})
  }
  process.exit(0)
}
