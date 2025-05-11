@echo off
echo Stopping any running backend processes...
taskkill /F /IM backend.exe 2>nul
timeout /t 1 /nobreak >nul

echo Cleaning backend directory...
cd backend
cargo clean
timeout /t 1 /nobreak >nul

echo Building backend...
cargo build

echo Done!
cd .. 