# 设计文档

## 前端 apiUtils.ts

```typescript
export async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || `HTTP ${response.status}`;
}
```

统一使用 `` `HTTP ${status}` `` 格式，比 `String(status)` 更语义化。

## Go util/path.go

```go
package util

import "os"

func PathExists(path string) bool {
  if path == "" {
    return false
  }
  _, err := os.Stat(path)
  return err == nil
}
```

导出函数首字母大写，保留空路径防御。

## 引用方式

- 前端：`import { readErrorText } from "../shared/apiUtils"`
- Go：`import "local-agent/gateway/internal/util"`，调用 `util.PathExists(...)`
