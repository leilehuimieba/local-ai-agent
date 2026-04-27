# 验证记录

## 第一轮：前端 API + Go pathExists 统一

### O-01~O-05 前端 readErrorText 统一

- [x] `frontend/src/shared/apiUtils.ts` 已创建
- [x] `chat/api.ts`、`settings/api.ts`、`release/api.ts`、`knowledge-base/api.ts` 均已引用 apiUtils
- [x] `npm test -- --run`：**25 files / 74 tests passed**
- [x] `npx tsc --noEmit`：**通过**

### O-06~O-08 Go pathExists 统一

- [x] `gateway/internal/util/path.go` 已创建
- [x] `gateway/internal/api/router.go`、`diagnostics_remediation.go`、`service/diagnostics.go` 已引用 `util.PathExists`
- [x] `go build ./...`：**通过**
- [x] `go test ./internal/service/ ./internal/api/`：**通过**

## 第二轮：workspaceViewModel.tsx 拆分

### O-09~O-12 前端 VM 层拆分

- [x] `frontend/src/shell/workspaceViewModel/types.ts` 已创建（类型 + 常量）
- [x] `frontend/src/shell/workspaceViewModel/renderers.tsx` 已创建（渲染函数）
- [x] `frontend/src/shell/workspaceViewModel/props.ts` 已创建（Props 构建器）
- [x] `frontend/src/shell/workspaceViewModel/components/TaskLeftNav.tsx` 已创建（导航组件 + 逻辑）
- [x] `frontend/src/shell/workspaceViewModel.tsx` 重写为重新导出（509 行 → 5 行）
- [x] `homeModel.ts` 兼容通过
- [x] `npm test -- --run`：**25 files / 74 tests passed**
- [x] `npx tsc --noEmit`：**通过**

## 第三轮：router_release.go 拆 service 层

### O-13~O-17 Release 流程 service 化

- [x] `gateway/internal/service/release.go` 已创建
- [x] `gateway/internal/api/router_release.go` 重写为纯 HTTP 适配（155 行 → 39 行）
- [x] `gateway/internal/service/release_test.go` 已创建
- [x] `gateway/internal/api/router_release_test.go` 精简
- [x] `go build ./...`：**通过**
- [x] `go test ./internal/service/ -run TestRelease`：**通过**
- [x] `go test ./internal/api/ -run TestRelease`：**通过**

## 第四轮：useSettings.ts 拆分

### O-18~O-21 Settings Hook 拆分

- [x] `frontend/src/settings/actionRunner.ts` 已创建
- [x] `frontend/src/settings/useSettings.ts` 重写引用 actionRunner（563 行 → 355 行）
- [x] `npm test -- --run`：**25 files / 74 tests passed**
- [x] `npx tsc --noEmit`：**通过**

## 第五轮：router.go 纯工具函数移入 util 包

### O-22~O-25 工具函数归位

- [x] `gateway/internal/util/file.go` 已创建（CountJSONLLines + CountDirEntries）
- [x] `gateway/internal/api/router.go` 引用 util 函数，移除内联实现
- [x] `go build ./...`：**通过**
- [x] `go test ./internal/service/ ./internal/api/ -run TestRelease`：**通过**

## 第六轮：tmp/ 历史产物清理

### O-26 临时目录治理

- [x] 清理 11 个超过 7 天的 stage 验收目录
- [x] 释放磁盘空间 ~0.96 MB
- [x] 保留 release-wizard、release-wizard-install、e2e-test 等当前使用目录

### 无回归问题

- [x] Rust `cargo check --workspace`：通过
- [x] 未修改业务逻辑
- [x] 未引入新依赖
