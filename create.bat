@echo off
setlocal
pushd "%~dp0"
call npm.cmd run build:msi
if errorlevel 1 (
    popd
    exit /b 1
)
echo MSI generated in src-tauri\target\release\bundle\msi
popd
endlocal
