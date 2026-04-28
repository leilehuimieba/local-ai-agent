@echo off
chcp 65001 > nul
cd /d "%~dp0"

echo ============================================
echo   本地智能体 - 一键启动
echo ============================================
echo.

:: 加载 .env 环境变量（关键变量直接设置）
set LOCAL_AGENT_API_KEY_DEEPSEEK=sk-ed9edbc6dc574bd099cf9c4d3d43e781
set LOCAL_AGENT_MODEL_ID=deepseek-v4-pro
set LOCAL_AGENT_PROVIDER_ID=deepseek
set LOCAL_AGENT_GATEWAY_PORT=8897
set LOCAL_AGENT_RUNTIME_PORT=8898
set LOCAL_AGENT_DEFAULT_MODE=standard

:: 检查 Go 是否可用
where go >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未找到 Go，请先安装 Go: https://go.dev/dl/
    pause
    exit /b 1
)

:: 检查 Rust/Cargo 是否可用
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未找到 Cargo，请先安装 Rust: https://rustup.rs/
    pause
    exit /b 1
)

:: 检查 Node.js 是否可用
where node >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未找到 Node.js，请先安装: https://nodejs.org/
    pause
    exit /b 1
)

echo [1/3] 检查前端构建...
if not exist "frontend\dist\index.html" (
    echo   首次运行，正在安装前端依赖...
    cd frontend
    call npm install
    echo   正在构建前端...
    call npm run build
    cd ..
) else (
    echo   前端已构建，跳过
)

echo [2/3] 编译 Gateway...
cd gateway
go build -o server.exe ./cmd/server
cd ..

echo [3/3] 启动本地智能体...
echo.
echo   Gateway  : http://127.0.0.1:8897
echo   Runtime  : http://127.0.0.1:8898
echo   模型     : DeepSeek V4 Pro
echo.
echo ============================================
echo   启动中，请稍候...
echo ============================================

cd gateway
go run ./cmd/launcher
cd ..

pause
