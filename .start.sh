#!/usr/bin/env bash
set -e

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "==> Starting AADE Validation Engine (local)"

# Backend
echo "==> Backend (Rust)"
cd "$ROOT_DIR/aade-backend"

if [ ! -d "target" ]; then
  echo "Building backend..."
  cargo build
fi

cargo run &

BACKEND_PID=$!

# Frontend
echo "==> Frontend (Vite)"
cd "$ROOT_DIR/aade-frontend"

if [ ! -d "node_modules" ]; then
  echo "Installing frontend deps..."
  npm install
fi

npm run dev &

FRONTEND_PID=$!

echo ""
echo "Backend PID:  $BACKEND_PID"
echo "Frontend PID: $FRONTEND_PID"
echo ""
echo "Local URLs:"
echo "  Backend:  http://localhost:3600"
echo "  Frontend: http://localhost:5173"
echo ""
echo "Press Ctrl+C to stop both."

trap "echo 'Stopping...'; kill $BACKEND_PID $FRONTEND_PID" SIGINT SIGTERM

wait
