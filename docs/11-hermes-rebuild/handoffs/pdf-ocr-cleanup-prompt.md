# 提示词：中文PDF乱码修复 + OCR扫描件识别 上下文接续

## 你是谁
你是本项目的Go后端开发助手，当前工作聚焦于 `gateway/internal/knowledge/` 模块的PDF文本提取与OCR能力。

## 项目架构
- **路径**：`D:/newwork/本地智能体`
- **前端**：`frontend/` (React 19 + Vite + TS)，dev 端口 `5174`
- **后端**：`gateway/` (Go 1.25)，端口 `8897`
- **数据库**：`data/storage/main.db`（SQLite，538条记录）
- **poppler**：`gateway/third_party/poppler/poppler-24.08.0/`（已gitignore）
- **百度OCR**：`config/app.json` 已配置，5万次/月额度

## 已完成的代码改动

### 核心文件
- `gateway/internal/knowledge/extract.go` — PDF/TXT/DOCX文本提取主逻辑
- `gateway/internal/knowledge/ocr.go` — 百度OCR封装（access_token + accurate_basic）
- `gateway/internal/config/config.go` — 新增 `OCRConfig` 配置字段
- `gateway/cmd/reimport-ocr/main.go` — 批量OCR重导入脚本（支持 `limit` 参数分批）

### 关键技术方案
1. **PDF提取优先级**：pdftotext `-enc UTF-8 -layout` → `rsc.io/pdf` 回退 → OCR（扫描件回退）
2. **Windows中文路径兼容**：复制PDF到 `%TEMP%` 纯英文路径，提取后清理
3. **poppler-data编码修复**：`POPPLER_PREFIX` 指向 share 目录；若路径含中文，自动在 `%TEMP%` 创建 junction（`poppler-prefix`）避免GBK解析错误
4. **乱码检测**：`isMostlyGarbled` 排除空格+Unicode格式字符(Cf)，阈值30%；控制字符比例>15%强制走OCR
5. **OCR限制**：单PDF最多前8页（`-f 1 -l 8`），150 DPI
6. **空字节过滤**：`strings.ReplaceAll(content, "\x00", "")` 防止sqlite3 stdin截断

## 当前数据库状态

```
total=538
empty=3      ← 3条高频词汇PDF字体映射完全错乱，无法识别
garbled=0    ← 乱码已清零
ok=427       ← 约79%质量良好（title正常、content≥500、无水印、无乱码）
watermark=42 ← 扫描件含淘宝/叮当考研水印文字
short=13     ← 1-2页短文档（作文模板、TED文稿、图表）
filename_title=59 ← title回退为文件名
```

## 常用验证命令

```bash
# DB整体状态
cd gateway; go run ./cmd/reimport-check/main.go

# 质量分级（输出garbled/watermark/short/filename_title等明细）
cd gateway; go run ./cmd/quality-check/main.go

# 批量OCR重导入（limit=每批条数）
cd gateway; go run ./cmd/reimport-ocr/main.go 30
```

## 已知限制与下一步

1. **3条词汇PDF无法识别**：`大学英语四级高频词汇.pdf`、`英语四级高频词汇（submission）.pdf`、`大学英语六级高频词汇.pdf` — 字体嵌入特殊，pdftoppm渲染即乱码，OCR无效。建议尝试Python库（pdfplumber/PyMuPDF）或人工维护。
2. **42条水印记录**：content以"淘宝店铺""叮当考研"等开头。可考虑 `strings.ReplaceAll` 清洗水印前缀，或放宽OCR页数限制。
3. **59条filename_title**：多为TED文稿（"中英双语.pdf"）。可用内容首行替换文件名title。
4. **OCR前8页限制**：30页解析类PDF可能丢失后续内容。若需完整识别，可调整 `ocr.go` 的 `maxPages`。

## 约束
- 新增/修改的Go函数不超过30行
- 不修改未改动的代码行注释
- `commit message` 用中文，不超过50字
- 热点文件（>600行）需评估拆分，拆分前在 `docs/11-hermes-rebuild/changes/` 备案
