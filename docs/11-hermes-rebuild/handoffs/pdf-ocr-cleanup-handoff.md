# 交接文档：中文PDF乱码修复 + 百度OCR扫描件识别

## 项目背景

- **项目路径**：`D:/newwork/本地智能体`
- **前端**：`frontend/` (React 19 + Vite + TS)，dev 端口 `5174`
- **后端**：`gateway/` (Go 1.25)，端口 `8897`
- **数据库**：`data/storage/main.db`（SQLite）
- **poppler**：`gateway/third_party/poppler/poppler-24.08.0/`（47MB，已gitignore）
- **百度OCR**：`config/app.json` 已配置，个人额度 5万次/月

## 本轮完成的工作

### 1. 中文PDF文本提取修复
- **根因**：`rsc.io/pdf` v0.1.1 不支持中文CMap/CID字体编码
- **策略**：pdftotext 优先 → 乱码校验清空 → 扫描件OCR回退
- **文件**：`gateway/internal/knowledge/extract.go`

### 2. Windows中文路径兼容
- pdftotext无法处理Unicode路径 → 自动复制PDF到 `%TEMP%` 纯英文临时路径，提取后清理

### 3. poppler-data编码错误修复
- `unknown encoding GBK-EUC-H` → 设置 `POPPLER_PREFIX` 环境变量指向 poppler share 目录
- 若路径含中文，自动在 `%TEMP%` 创建 junction（`poppler-prefix`）指向实际目录，避免poppler内部GBK解析错误

### 4. title选取优化
- 从前5行非空候选中选取不含 `�` 且通过乱码检测的首行，限制≤200字符

### 5. 百度OCR集成
- **文件**：`gateway/internal/knowledge/ocr.go`
- 封装百度API（取access_token + accurate_basic识别）
- `ExtractText` 在PDF为空/乱码时自动OCR
- 限制单PDF最多前8页（`-f 1 -l 8`）、150 DPI，避免超时

### 6. 乱码检测增强
- `isMostlyGarbled`：空格、Unicode格式控制字符(Cf)不计入有效字符，阈值30%
- `extractPdf`：控制字符比例>15%时强制走OCR路径
- 过滤 `\u0000` 空字节，防止sqlite3 stdin截断

### 7. 批量OCR重导入
- **脚本**：`gateway/cmd/reimport-ocr/main.go`
- 支持 `limit` 参数分批处理（默认30条）
- 已处理约200+条早期扫描件真题（2007-2020年）

## 当前数据库状态（538条记录）

| 指标 | 数值 | 说明 |
|------|------|------|
| total | 538 | — |
| empty | 3 | 高频词汇PDF字体映射完全错乱，无法识别 |
| garbled | 0 | 无乱码记录 |
| ok(≥500,无水印,无乱码,非文件名title) | 427 | 约79%质量良好 |
| watermark | 42 | 扫描件含淘宝/叮当考研水印文字 |
| short(<500) | 13 | 1-2页短文档（作文模板、TED文稿、图表） |
| filename_title | 59 | title回退为文件名（如"中英双语.pdf"） |

## 关键代码文件

```
gateway/internal/knowledge/extract.go   # 文本提取主逻辑
gateway/internal/knowledge/ocr.go       # 百度OCR封装
gateway/internal/config/config.go       # OCR配置字段
gateway/cmd/reimport-ocr/main.go        # 批量OCR脚本
gateway/cmd/reimport-check/main.go      # DB状态检查脚本
gateway/cmd/quality-check/main.go       # 质量分级检查脚本
```

## 已知限制

1. **3条词汇PDF无法识别**：
   - `大学英语四级高频词汇.pdf`
   - `英语四级高频词汇（submission）.pdf`
   - `大学英语六级高频词汇.pdf`
   - 原因：字体嵌入方式特殊，pdftoppm渲染出的图片本身为乱码符号，OCR也无法识别

2. **部分扫描件OCR只识别到水印**：约42条记录content以"淘宝店铺""叮当考研""谈辰图书"等水印开头，通常也夹杂部分正文

3. **OCR限制前8页**：完整扫描件PDF（如30页解析）只识别前8页，可能丢失后面内容

4. **百度OCR额度**：个人应用5万次/月，当前已消耗约2000次

## 测试验证方式

```bash
# DB整体状态
cd gateway; go run ./cmd/reimport-check/main.go

# 质量分级
cd gateway; go run ./cmd/quality-check/main.go

# 单条PDF测试
cd gateway; go run ./cmd/test-ocr-direct/main.go

# 批量OCR（limit=N）
cd gateway; go run ./cmd/reimport-ocr/main.go 30
```

## 下一步建议

1. **水印清洗**：对42条watermark记录，尝试用 `strings.ReplaceAll` 去除水印前缀，或增加OCR页数限制
2. **title优化**：59条filename_title中，TED文稿可用内容首行替换当前文件名title
3. **词汇PDF替代方案**：3条empty词汇PDF可尝试用Python库（如pdfplumber/PyMuPDF）提取，或人工维护词汇文本
4. **前端检索验证**：确认知识库搜索功能对中文内容正常返回
