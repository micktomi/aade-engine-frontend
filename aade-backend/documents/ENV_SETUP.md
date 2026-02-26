# Environment Setup Guide

This project uses a single `.env` file for local development. For production
(Render/Vercel), set environment variables in the provider dashboard.

## `.env` (git ignored)
Create `aade/.env` with local values:
```bash
DATABASE_URL=postgres://user:password@localhost:5432/aade_db
SERVER_ADDR=127.0.0.1:3000
ENVIRONMENT=development
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:5173,http://localhost:5174
RUST_LOG=info
```

## Docker (optional)
`docker-compose.yml` already sets env vars, so no extra `.env` file is required.

## Production (Render)
Set these in Render → Environment:
```bash
DATABASE_URL=<Internal-Database-URL>
ENVIRONMENT=production
SERVER_ADDR=0.0.0.0:3000
PORT=3000
CORS_ALLOWED_ORIGINS=https://your-frontend.vercel.app
RUST_LOG=info
```

## Frontend (Vercel)
Set this in Vercel → Environment:
```bash
VITE_API_URL=https://your-backend.onrender.com
```

## Security
`.env` is ignored by git and should never include real production secrets in the repo.
