@echo off
echo Starting AI Paraphrasing Tool in development mode...

start cmd /k "cd frontend && npm run dev"
start cmd /k "cd backend && cargo run"

echo Both servers started.
echo Frontend: http://localhost:5173
echo Backend: http://localhost:8080 