# 🚀 Vercel Frontend Deployment (με Render Backend)

Οδηγός για να ανεβάσεις το frontend στο Vercel και το backend στο Render.

---

## 🏗️ Αρχιτεκτονική

```
Frontend (React Vite) → Vercel
Backend (Rust API) → Render
Database (PostgreSQL) → Render
```

---

## Βήμα 1: Deploy Backend στο Render

Ακολούθησε το `RENDER_DEPLOYMENT.md` για:
1. ✅ PostgreSQL Database
2. ✅ Backend Web Service

**ΣΗΜΑΝΤΙΚΟ:** Κράτα το backend URL, π.χ.:
```
https://aade-backend.onrender.com
```

---

## Βήμα 2: Deploy Frontend στο Vercel

### 2.1 Σύνδεση με GitHub

1. Πήγαινε στο [vercel.com](https://vercel.com)
2. **Sign Up / Login** με GitHub
3. **Add New Project**
4. **Import** το `aade-validation engine` repository

### 2.2 Project Settings

#### Framework Preset:
```
Framework: Vite
```

#### Root Directory:
```
Root Directory: aade-ui
```

#### Build Settings:
```
Build Command: npm run build
Output Directory: dist
Install Command: npm install
```

### 2.3 Environment Variables

Πρόσθεσε αυτή τη μεταβλητή:

```bash
VITE_API_URL
Value: https://aade-backend.onrender.com
       (το URL του backend σου από το Render)
```

### 2.4 Deploy

1. Κάνε **Deploy**
2. Περίμενε 1-2 λεπτά
3. Κράτα το Vercel URL, π.χ.: `https://aade-ui.vercel.app`

---

## Βήμα 3: Ενημέρωση CORS στο Backend

### 3.1 Πήγαινε στο Render Dashboard

1. Άνοιξε το **aade-backend** service
2. Πήγαινε στο **Environment**

### 3.2 Ενημέρωση CORS_ALLOWED_ORIGINS

Πρόσθεσε το Vercel URL:

```bash
CORS_ALLOWED_ORIGINS=https://aade-ui.vercel.app
```

**Για πολλαπλά domains** (π.χ. production + custom domain):
```bash
CORS_ALLOWED_ORIGINS=https://aade-ui.vercel.app,https://app.yourdomain.com
```

**Για Vercel Preview Deployments** (προαιρετικό):
```bash
# Προσοχή: Αυτό είναι πιο permissive, χρησιμοποίησε μόνο αν χρειάζεται
CORS_ALLOWED_ORIGINS=https://aade-ui.vercel.app,https://aade-ui-*.vercel.app
```

### 3.3 Save & Redeploy

- **Save Changes**
- Το backend θα κάνει auto-redeploy (30-60 δευτερόλεπτα)

---

## ✅ Verification

### 1. Test Backend Health:
```bash
curl https://aade-backend.onrender.com/health/ready
# Expected: {"status":"ready"}
```

### 2. Test Frontend:
1. Άνοιξε `https://aade-ui.vercel.app`
2. Άνοιξε **Browser DevTools** (F12) → **Console**
3. Upload ένα XML τιμολόγιο
4. Έλεγξε ότι **δεν υπάρχουν CORS errors**

### 3. Test CORS:
```bash
# Από terminal
curl -X OPTIONS https://aade-backend.onrender.com/validate \
  -H "Origin: https://aade-ui.vercel.app" \
  -H "Access-Control-Request-Method: POST" \
  -v

# Κοίτα για:
# < Access-Control-Allow-Origin: https://aade-ui.vercel.app
```

---

## 🔄 Auto-Deploy

### Vercel (Frontend):
- **Production:** Κάθε push στο `main` branch
- **Preview:** Κάθε pull request
- **Instant:** ~30-60 δευτερόλεπτα

### Render (Backend):
- Κάθε push στο `main` branch
- ~5-10 λεπτά rebuild

---

## 📋 Environment Variables Recap

### Backend στο Render:
```bash
DATABASE_URL=<Internal-Database-URL>
ENVIRONMENT=production
SERVER_ADDR=0.0.0.0:3000
PORT=3000
RUST_LOG=info
CORS_ALLOWED_ORIGINS=https://aade-ui.vercel.app
```

### Frontend στο Vercel:
```bash
VITE_API_URL=https://aade-backend.onrender.com
```

---

## 🐛 Troubleshooting

### CORS Error στο Browser:
```
Access to XMLHttpRequest at 'https://aade-backend.onrender.com/validate'
from origin 'https://aade-ui.vercel.app' has been blocked by CORS policy
```

**Fix:**
1. Έλεγξε `CORS_ALLOWED_ORIGINS` στο Render backend
2. Βεβαιώσου ότι το URL είναι **ακριβώς** το ίδιο (με https://)
3. Δεν υπάρχει trailing slash: ❌ `https://aade-ui.vercel.app/` ✅ `https://aade-ui.vercel.app`

### API Calls Fail (404):
```
GET https://aade-backend.onrender.com/validate 404
```

**Fix:**
1. Έλεγξε `VITE_API_URL` στο Vercel
2. Βεβαιώσου ότι το backend είναι UP (visit the URL)
3. Έλεγξε ότι το path είναι σωστό (/validate, όχι /api/validate)

### Backend Cold Start:
```
Request timeout after 30s
```

**Αιτία:** Render free tier κάνει "spin down" μετά 15 λεπτά inactivity

**Fix:**
- Περίμενε 30-60 δευτερόλεπτα για cold start
- Ή upgrade σε Render paid plan ($7/μήνα) για always-on

### Vercel Build Fails:
```
Error: Cannot find module 'vite'
```

**Fix:**
1. Έλεγξε ότι το `Root Directory` είναι `aade-ui`
2. Έλεγξε ότι το `package.json` έχει `vite` στα dependencies

---

## 💰 Costs

### Vercel (Frontend):
- ✅ **Free:** Hobby Plan
  - 100GB bandwidth/μήνα
  - Unlimited websites
  - Automatic HTTPS
  - Global CDN

### Render (Backend + DB):
- **Backend Free:** 750 ώρες/μήνα (με cold starts)
- **Backend Paid:** $7/μήνα (always-on)
- **PostgreSQL:** $7/μήνα μετά 90-day trial

---

## 🌍 Custom Domain

### Frontend (Vercel):
1. Vercel Dashboard → Project → Settings → Domains
2. Πρόσθεσε: `app.yourdomain.com`
3. Ενημέρωσε DNS records (Vercel θα σου δείξει τι να προσθέσεις)
4. **Auto SSL** από το Vercel

### Backend (Render):
1. Render Dashboard → Service → Settings → Custom Domain
2. Πρόσθεσε: `api.yourdomain.com`
3. Ενημέρωσε DNS records
4. **Auto SSL** από το Render

### Update Environment Variables:
```bash
# Vercel
VITE_API_URL=https://api.yourdomain.com

# Render
CORS_ALLOWED_ORIGINS=https://app.yourdomain.com
```

---

## ⚡ Performance Tips

### Vercel:
- ✅ **Edge Caching:** Αυτόματο
- ✅ **Brotli Compression:** Αυτόματο
- ✅ **Image Optimization:** Χρησιμοποίησε `<Image />` από Next.js (αν μεταφέρεις)

### Render:
- Use **Starter Plan** για no cold starts
- Enable **Persistent Disk** αν χρειάζεσαι local storage

---

## 📊 Monitoring

### Vercel Analytics (Optional):
```bash
npm install @vercel/analytics
```

```typescript
// src/main.tsx
import { inject } from '@vercel/analytics';
inject();
```

### Render Logs:
- Real-time logs στο Dashboard
- Search & filter
- Download για debugging

---

## 🔐 Security Checklist

- [ ] Backend `ENVIRONMENT=production`
- [ ] Backend CORS περιορισμένο (όχι `*`)
- [ ] Frontend `VITE_API_URL` points to HTTPS
- [ ] Database `DATABASE_URL` uses Internal URL (όχι External)
- [ ] No `.env` files committed to git
- [ ] Strong database password
- [ ] HTTPS enabled (αυτόματο στο Vercel/Render)

---

## 🎉 Success!

Το app σου είναι τώρα live:

```
Frontend: https://aade-ui.vercel.app
Backend:  https://aade-backend.onrender.com
```

**Next Steps:**
1. Test όλα τα features
2. Share με users για feedback
3. Monitor errors στο Render Logs
4. Optimize performance αν χρειάζεται
