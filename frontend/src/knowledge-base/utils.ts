const WATERMARK_KEYWORDS = [
  "淘宝店铺：", "淘宝：", "掌柜旺旺：", "认准淘宝店铺：",
  "叮当考研", "谈辰图书", "光速考研工作室", "学海无涯教育",
];

const GARBLED_PATTERN = /^[!"#$%&'()*+,-\s]+$/;

export function isGarbledTitle(title: string): boolean {
  if (!title) return true;
  if (WATERMARK_KEYWORDS.some((kw) => title.includes(kw))) return true;
  if (GARBLED_PATTERN.test(title.trim())) return true;
  return false;
}

export function cleanTitle(item: { title: string; source?: string }): string {
  if (!isGarbledTitle(item.title)) return item.title;
  if (item.source) {
    const name = item.source.split(/[\\/]/).pop() || item.source;
    return name.replace(/\.[^.]+$/, "");
  }
  return "未命名资料";
}

export function fallbackSummary(item: { summary: string; content: string }): string {
  if (item.summary.trim()) return item.summary.trim();
  if (item.content.trim()) {
    const text = item.content.trim().replace(/\s+/g, " ");
    return text.length > 120 ? text.slice(0, 120) + "…" : text;
  }
  return "";
}
