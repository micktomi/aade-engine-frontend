# 🚀 Render Deployment Guide

Οδηγός για να ανεβάσεις το AADE Validator στο Render (Backend + Frontend).

## 📦 Τι θα δημιουργήσεις στο Render:

1. **PostgreSQL Database** (1x)
2. **Web Service - Backend** (Rust API) (1x)
3. **Static Site - Frontend** (React Vite) (1x)

---

## Βήμα 1: PostgreSQL Database

### Δημιουργία:
1. Render Dashboard → **New** → **PostgreSQL**
2. Όνομα: `aade-db`
3. Region: Διάλεξε το πιο κοντινό (π.χ. Frankfurt για Ευρώπη)
4. Plan: **Free** (για testing)
5. **Create Database**

### Κράτα αυτά:
- ✅ **Internal Database URL** (θα το χρειαστείς για το backend)

---

## Βήμα 2: Backend (Rust API)

### Δημιουργία:
1. Render Dashboard → **New** → **Web Service**
2. Connect το GitHub repository σου
3. Διάλεξε το `aade-validation engine` repo

### Ρυθμίσεις:

#### Basic Settings:
```
Name: aade-backend
Region: Ίδιο με το database
Branch: main
Root Directory: aade
```

#### Build & Deploy:
```
Runtime: Docker
Dockerfile Path: ./Dockerfile
```

#### Environment Variables:
Πρόσθεσε αυτές (Add Environment Variable):

```bash
DATABASE_URL
Value: <paste-το-Internal-Database-URL-από-το-PostgreSQL>

ENVIRONMENT
Value: production

SERVER_ADDR
Value: 0.0.0.0:3000

PORT
Value: 3000

RUST_LOG
Value: info

CORS_ALLOWED_ORIGINS
Value: https://your-frontend-app.onrender.com
       (θα το ενημερώσεις μετά το deploy του frontend)
```

### Health Check:
```
Health Check Path: /health/ready
```

### Deploy:
- Κάνε **Create Web Service**
- Περίμενε 5-10 λεπτά να γίνει build
- Κράτα το URL: `https://aade-backend.onrender.com`

---

## Βήμα 3: Frontend (React Vite)

### Δημιουργία:
1. Render Dashboard → **New** → **Static Site**
2. Connect το ίδιο repository
3. Διάλεξε το `aade-validation engine` repo

### Ρυθμίσεις:

#### Basic Settings:
```
Name: aade-frontend
Branch: main
Root Directory: aade-ui
```

#### Build & Deploy:
```
Build Command: npm install && npm run build
Publish Directory: dist
```

#### Environment Variables:
```bash
VITE_API_URL
Value: https://aade-backend.onrender.com
       (το URL που πήρες από το backend deploy)
```

### Deploy:
- Κάνε **Create Static Site**
- Περίμενε 2-3 λεπτά
- Κράτα το URL: `https://aade-frontend.onrender.com`

---

## Βήμα 4: Τελικές Ρυθμίσεις (CORS)

### Ενημέρωση Backend CORS:
1. Πήγαινε στο **aade-backend** service
2. Environment → Edit `CORS_ALLOWED_ORIGINS`
3. Βάλε το frontend URL:
   ```
   https://aade-frontend.onrender.com
   ```
4. **Save Changes**
5. Το service θα κάνει auto-redeploy

---

## ✅ Verification

### Test το backend:
```bash
curl https://aade-backend.onrender.com/health/ready
# Αναμενόμενο: {"status":"ready"}
```

### Test το frontend:
1. Άνοιξε `https://aade-frontend.onrender.com`
2. Upload ένα XML τιμολόγιο
3. Έλεγξε ότι λειτουργεί η validation

---

## 📋 Checklist Deployment

### Backend:
- [ ] PostgreSQL database created
- [ ] `DATABASE_URL` set σωστά (Internal URL)
- [ ] `ENVIRONMENT=production`
- [ ] `CORS_ALLOWED_ORIGINS` με frontend URL
- [ ] Health check `/health/ready` περνάει
- [ ] Migrations τρέχουν αυτόματα (έλεγξε logs)

### Frontend:
- [ ] `VITE_API_URL` δείχνει στο backend
- [ ] Build succeeds (έλεγξε logs)
- [ ] Static files served
- [ ] API calls λειτουργούν (έλεγξε Network tab)

### Database:
- [ ] Connections: Τουλάχιστον 1 connection από backend
- [ ] Table `validation_logs` δημιουργήθηκε

---

## 🐛 Troubleshooting

### Backend δεν ξεκινάει:
```bash
# Έλεγξε logs στο Render Dashboard
# Πιθανά προβλήματα:
- DATABASE_URL λάθος (έλεγξε ότι είναι το Internal URL)
- Migrations failed (έλεγξε logs για SQL errors)
- Port conflict (έλεγξε SERVER_ADDR=0.0.0.0:3000)
```

### Frontend δεν συνδέεται στο backend:
```bash
# Έλεγξε Browser Console
- CORS error → Ενημέρωσε CORS_ALLOWED_ORIGINS στο backend
- 404 error → Έλεγξε VITE_API_URL στο frontend
- Network timeout → Έλεγξε ότι backend είναι UP
```

### Database connection failed:
```bash
# Έλεγξε:
- Το database είναι στο ίδιο region με το backend
- Χρησιμοποιείς Internal Database URL (όχι External)
- Το database δεν είναι suspended (Free tier)
```

---

## 💰 Costs (Free Tier Limits)

### PostgreSQL:
- ✅ **Free**: 90 ημέρες trial, μετά $7/μήνα
- 256MB RAM, 1GB Storage

### Backend Web Service:
- ✅ **Free**: 750 ώρες/μήνα
- Spins down μετά 15 λεπτά inactivity
- Cold start: 30-60 δευτερόλεπτα

### Frontend Static Site:
- ✅ **Free**: Unlimited
- 100GB bandwidth/μήνα

---

## 🔄 Auto-Deploy

Κάθε `git push` στο `main` branch θα κάνει auto-deploy:
- Backend: ~5-10 λεπτά rebuild
- Frontend: ~2-3 λεπτά rebuild

---

## 📝 URLs Recap

Μετά το deployment θα έχεις:

```
Backend API:
https://aade-backend.onrender.com

Frontend:
https://aade-frontend.onrender.com

Database:
Internal: dpg-xxxxx.oregon-postgres.render.com
```

---

## 🔗 Custom Domain (Optional)

Αν θέλεις δικό σου domain:

1. Render Dashboard → Service → Settings → Custom Domain
2. Πρόσθεσε: `api.yourdomain.com` (backend)
3. Πρόσθεσε: `app.yourdomain.com` (frontend)
4. Ενημέρωσε DNS records στον domain provider σου
5. Ενημέρωσε `CORS_ALLOWED_ORIGINS` και `VITE_API_URL`

---

## ⚡ Performance Tips

### Backend:
- Χρησιμοποίησε **Starter** plan ($7/μήνα) για no cold starts
- Ενεργοποίησε **Persistent Disk** αν χρειάζεσαι local files

### Database:
- Upgrade σε **Standard** plan ($25/μήνα) για production
- Enable **Backups** (auto στα paid plans)

### Frontend:
- Ενεργοποίησε **Brotli Compression** (αυτόματο στο Render)
- Χρησιμοποίησε CDN αν έχεις πολλή κίνηση

---

**Τέλος! Το AADE Validator σου είναι live! 🎉**
