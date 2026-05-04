@echo off
setlocal
cd /d "%~dp0gateway"
go run .\cmd\launcher
