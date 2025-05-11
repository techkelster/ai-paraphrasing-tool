@echo off
echo Rebuilding backend...

REM Kill any running processes on port 8080 (if needed)
for /f "tokens=5" %%a in ('netstat -aon ^| find ":8080" ^| find "LISTENING"') do (
    echo Killing process %%a on port 8080
    taskkill /F /PID %%a
)

REM Build and run the backend
cargo build
if %ERRORLEVEL% == 0 (
    echo Build successful. Starting server...
    start cargo run
    echo Backend server started on http://localhost:8080
) else (
    echo Build failed with error %ERRORLEVEL%
) 