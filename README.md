# myDATA Validation Engine 🇬🇷

Ένα πλήρες σύστημα επικύρωσης XML παραστατικών για το myDATA της ΑΑΔΕ (Ανεξάρτητη Αρχή Δημοσίων Εσόδων).

## 📋 Περιεχόμενα

- [Χαρακτηριστικά](#χαρακτηριστικά)
- [Τεχνολογίες](#τεχνολογίες)
- [Εγκατάσταση](#εγκατάσταση)
- [Χρήση](#χρήση)
- [API Documentation](#api-documentation)
- [Validation Rules](#validation-rules)
- [Development](#development)
- [Production Deployment](#production-deployment)

---

## 🚀 Χαρακτηριστικά

### Backend (Rust)
- ✅ **100% Coverage** - Υποστήριξη όλων των 17 τύπων παραστατικών myDATA
- ✅ **55 Validation Rules** - Πλήρης έλεγχος σύμφωνα με τις προδιαγραφές ΑΑΔΕ
- ✅ **Business Logic** - AFM validation (MOD 11), totals check, classifications
- ✅ **Financial Fields** - Withheld tax (20%), Stamp duty (3.6%), Fees, Deductions
- ✅ **Fast Processing** - Batch validation πολλαπλών αρχείων
- ✅ **Database Logging** - Προαιρετική καταγραφή validations (PostgreSQL)
- ✅ **Rate Limiting** - Προστασία από abuse

### Frontend (React + TypeScript)
- ✅ **Drag & Drop** - Batch upload αρχείων XML
- ✅ **Real-time Validation** - Άμεση επικύρωση και αποτελέσματα
- ✅ **Greek UI** - Πλήρως ελληνική διεπαφή
- ✅ **Error Explanations** - Κατανοητές επεξηγήσεις λαθών με προτεινόμενες λύσεις
- ✅ **Export Results** - Εξαγωγή αποτελεσμάτων σε PDF

### Υποστηριζόμενοι Τύποι Παραστατικών

| Κωδικός | Τύπος Παραστατικού |
|---------|-------------------|
| 1.1 | Τιμολόγιο Πώλησης |
| 1.2 | Τιμολόγιο Πώλησης / Ενδοκοινοτικές Παραδόσεις |
| 1.3 | Τιμολόγιο Πώλησης / Τρίτες Χώρες |
| 1.4 | Τιμολόγιο Πώλησης / Συνδεδεμένη |
| 1.5 | Τιμολόγιο Πώλησης / Διασυνοριακή Λιανική |
| 1.6 | Τιμολόγιο Πώλησης / Ξένο ΦΠΑ |
| 2.1 | Τιμολόγιο Παροχής Υπηρεσιών |
| 2.2 | Τιμολόγιο Παροχής / Ενδοκοινοτική |
| 2.3 | Τιμολόγιο Παροχής / Τρίτες Χώρες |
| 2.4 | Τιμολόγιο Παροχής / Συνδεδεμένη |
| 5.1 | Πιστωτικό Τιμολόγιο |
| 5.2 | Χρεωστικό Τιμολόγιο |
| 11.1 | ΑΛΠ - Απόδειξη Λιανικής Πώλησης |
| 11.2 | ΑΠΥ - Απόδειξη Παροχής Υπηρεσιών |
| 11.3 | Απλοποιημένο Τιμολόγιο |
| 11.4 | Πιστωτικό Στοιχείο Λιανικής |
| 11.5 | Πιστωτικό Στοιχείο Παροχής |

---

## 🛠️ Τεχνολογίες

### Backend
- **Rust** 1.83+ - Γρήγορη και ασφαλής γλώσσα
- **Axum** - Modern web framework
- **SQLx** - Type-safe SQL (PostgreSQL)
- **Serde** - Serialization/Deserialization
- **Quick-XML** - XML parsing

### Frontend
- **React 18** - UI library
- **TypeScript** - Type safety
- **Vite** - Build tool
- **TailwindCSS** - Styling
- **Axios** - HTTP client

---

## 📦 Εγκατάσταση

### Προαπαιτούμενα

```bash
# Rust (backend)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version  # Πρέπει να είναι 1.83+

# Node.js (frontend)
node --version   # Πρέπει να είναι 18+
npm --version    # Πρέπει να είναι 9+

# PostgreSQL (προαιρετικό - για logging)
psql --version   # Προαιρετικό
```

### Clone Repository

```bash
git clone <repository-url>
cd aade-validation-engine
```

### Backend Setup

```bash
cd aade-backend

# Αντιγραφή environment variables
cp .env.example .env

# Επεξεργασία .env (προαιρετικό)
nano .env

# Build
cargo build --release

# Run tests
cargo test

# Start server
cargo run --release
```

Το backend θα τρέχει στο `http://localhost:3000`

**Σημείωση**: Η database είναι **προαιρετική**. Αν δεν υπάρχει PostgreSQL, το backend θα ξεκινήσει κανονικά χωρίς logging.

### Frontend Setup

```bash
cd aade-frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

Το frontend θα τρέχει στο `http://localhost:5173`

### Database Setup (Προαιρετικό)

Αν θέλεις να καταγράφεις τα validations:

```bash
# Δημιουργία database
createdb aade_db

# Ενημέρωση .env
DATABASE_URL=postgres://user:password@localhost:5432/aade_db

# Οι migrations τρέχουν αυτόματα κατά την εκκίνηση
cargo run
```

---

## 💻 Χρήση

### Web Interface (Recommended)

1. Άνοιξε το browser στο `http://localhost:5173`
2. Σύρε XML αρχεία στην περιοχή upload ή κάνε click για επιλογή
3. Περίμενε τα αποτελέσματα
4. Δες λεπτομέρειες κάθε παραστατικού
5. Εξάγωγε αποτελέσματα σε PDF (προαιρετικό)

### API Usage

#### Single Invoice Validation

```bash
curl -X POST http://localhost:3000/validate \
  -H "Content-Type: application/xml" \
  --data-binary @invoice.xml
```

**Response:**
```json
[
  {
    "status": "Green",
    "errors": [],
    "warnings": [],
    "info": []
  }
]
```

#### Batch Validation

```bash
curl -X POST http://localhost:3000/validate/batch \
  -F "files=@invoice1.xml" \
  -F "files=@invoice2.xml" \
  -F "files=@invoice3.xml"
```

**Response:**
```json
[
  {
    "filename": "invoice1.xml",
    "status": "success",
    "reports": [...]
  },
  {
    "filename": "invoice2.xml",
    "status": "success",
    "reports": [...]
  }
]
```

#### Health Check

```bash
# Liveness probe
curl http://localhost:3000/health/live

# Readiness probe
curl http://localhost:3000/health/ready
```

---

## 📚 API Documentation

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/validate` | Validate single XML (raw body) |
| POST | `/validate/batch` | Validate multiple XMLs (multipart) |
| GET | `/health/live` | Liveness probe |
| GET | `/health/ready` | Readiness probe |

### Validation Status

- **🟢 Green**: Έγκυρο - Μπορεί να υποβληθεί
- **🟡 Yellow**: Προειδοποιήσεις - Μπορεί να υποβληθεί αλλά έλεγξε τα warnings
- **🔴 Red**: Άκυρο - Δεν μπορεί να υποβληθεί

### Error Codes

Παραδείγματα validation error codes:

- **BR-001** to **BR-008**: Business Rules (Totals, AFM, Dates)
- **VAT-001** to **VAT-017**: VAT Category Validation
- **CP-001** to **CP-008**: Counterpart Validation
- **CLS-001** to **CLS-006**: Classification Rules
- **NEG-001** to **NEG-004**: Negative Amounts
- **CRN-001**, **DBT-001-002**: Credit/Debit Notes
- **WTH-001-002**: Withheld Tax
- **STM-001-002**: Stamp Duty
- **FEE-001**, **DED-001-002**: Fees & Deductions
- **BRN-001-002**: Branch Validation
- **AMT-001**: Amount Validation

---

## ✅ Validation Rules

### 55 Ενεργοί Κανόνες

#### Business Rules (8)
- ✅ AFM Validation (MOD 11 algorithm)
- ✅ Total Net Amount check (tolerance: €0.05)
- ✅ Total VAT Amount check (tolerance: €0.05)
- ✅ Total Gross Amount check
- ✅ Line VAT calculation consistency
- ✅ Income classification totals
- ✅ Issue date validation (not in future)

#### VAT Rules (17)
- ✅ VAT categories per invoice type (1.1 - 11.5)
- ✅ VAT exemption cause required for 0% VAT
- ✅ Legacy VAT rates warning (17%, 9%, 4%)

#### Counterpart Rules (8)
- ✅ Counterpart required for B2B invoices
- ✅ Intra-EU country validation
- ✅ Third country validation
- ✅ Cross-border retail validation

#### Financial Fields (11)
- ✅ Withheld tax >= 0, 20% rate check
- ✅ Stamp duty >= 0, 3.6% rate check
- ✅ Fees >= 0
- ✅ Deductions >= 0, <= net amount
- ✅ Branch numbers >= 0
- ✅ Line amounts != 0

#### Classification Rules (6)
- ✅ B2B invoices require classifications
- ✅ Intra-EU requires E3_881_003
- ✅ Related party validations

#### Amount Rules (4)
- ✅ Credit notes must have negative amounts
- ✅ Normal invoices cannot have negative amounts
- ✅ Correlated invoice required for credit/debit notes

---

## 🧪 Development

### Backend Development

```bash
cd aade-backend

# Watch mode (recompile on change)
cargo watch -x run

# Run tests
cargo test

# Run specific test
cargo test test_name

# Lint
cargo clippy

# Format
cargo fmt

# Check without building
cargo check
```

### Frontend Development

```bash
cd aade-frontend

# Development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Lint
npm run lint
```

### Adding New Validation Rules

1. Επεξεργασία `aade-backend/rules/mydata_v1.yaml`
2. Προσθήκη νέου rule:

```yaml
- id: "CUSTOM-001"
  description: "My custom rule"
  severity: "Error"  # Error | Warning | Info
  logic:
    type: "LineValueAllowed"
    field_path: "my_field"
    allowed_values: ["value1", "value2"]
  error_message: "Custom error message here"
```

3. Restart backend - οι κανόνες φορτώνονται αυτόματα!

### Adding New RuleLogic Types

Αν χρειάζεσαι νέο τύπο λογικής:

1. Επεξεργασία `src/validation/rules_engine.rs`
2. Πρόσθεσε νέο variant στο `RuleLogic` enum
3. Υλοποίησε τη λογική στο `apply()` method
4. Γράψε unit tests

---

## 🚀 Production Deployment

### Environment Variables

#### Backend (.env)

```bash
# Required
DATABASE_URL=postgres://user:password@host:5432/aade_db

# Optional
ENVIRONMENT=production
PORT=3000
SERVER_ADDR=0.0.0.0:3000
CORS_ALLOWED_ORIGINS=https://your-frontend-domain.com
RUST_LOG=info,aade_validator=debug
```

#### Frontend (.env.production)

```bash
VITE_API_URL=https://your-backend-domain.com
```

### Docker Deployment (Προαιρετικό)

```bash
# Build backend image
cd aade-backend
docker build -t aade-backend .

# Build frontend image
cd aade-frontend
docker build -t aade-frontend .

# Run with docker-compose
docker-compose up -d
```

### Performance Tuning

#### Backend

```bash
# Production build with optimizations
cargo build --release

# Increase worker threads (default: CPU cores)
TOKIO_WORKER_THREADS=8 cargo run --release

# Database connection pool
# Set in code: max_connections(10)
```

#### Frontend

```bash
# Production build
npm run build

# Serve with nginx/caddy
# Output: dist/
```

### Security Checklist

- ✅ Rate limiting enabled (100 requests/15min per IP)
- ✅ CORS configured for production domain
- ✅ File size limits (10MB max)
- ⚠️ Add HTTPS (nginx/caddy reverse proxy)
- ⚠️ Add authentication if needed
- ⚠️ Add input sanitization
- ⚠️ Set up firewall rules

### Monitoring

```bash
# Backend logs (structured JSON)
RUST_LOG=info cargo run --release > app.log 2>&1

# Health checks
curl http://localhost:3000/health/live
curl http://localhost:3000/health/ready

# Metrics (consider adding Prometheus exporter)
```

---

## 📝 Configuration

### Backend Configuration

Edit `aade-backend/.env`:

```bash
# Database (optional)
DATABASE_URL=postgres://user:password@localhost:5432/aade_db

# Server
ENVIRONMENT=development  # development | production
PORT=3000
SERVER_ADDR=0.0.0.0:3000

# CORS (comma-separated)
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:5174

# Logging
RUST_LOG=info,aade_validator=debug,tower_http=debug
```

### Frontend Configuration

Edit `aade-frontend/.env`:

```bash
VITE_API_URL=http://127.0.0.1:3000
```

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Όλα τα tests πρέπει να περνάνε (`cargo test`)
- Clippy πρέπει να είναι clean (`cargo clippy`)
- Code formatting με `cargo fmt`
- Προσθήκη tests για νέα features
- Update documentation

---

## 📄 License

[Specify your license here]

---

## 🆘 Troubleshooting

### Backend δεν ξεκινάει

**Πρόβλημα**: Database connection timeout
**Λύση**: Η database είναι προαιρετική. Το backend θα ξεκινήσει με warning αν δεν βρει PostgreSQL.

**Πρόβλημα**: Port 3000 already in use
**Λύση**: Άλλαξε το `PORT` στο `.env` ή σκότωσε την άλλη εφαρμογή:
```bash
lsof -ti:3000 | xargs kill -9
```

### Frontend δεν συνδέεται με Backend

**Πρόβλημα**: Network Error / ERR_CONNECTION_REFUSED
**Λύση**:
1. Έλεγξε αν τρέχει το backend: `curl http://localhost:3000/health/live`
2. Έλεγξε το `VITE_API_URL` στο `.env`
3. Έλεγξε CORS settings στο backend `.env`

### Validation errors που δεν καταλαβαίνεις

Κάθε error έχει:
- **Code** (π.χ. BR-003)
- **Field** (ποιο πεδίο έχει πρόβλημα)
- **Explanation** (τι σημαίνει το λάθος)
- **Suggested Solution** (πώς να το διορθώσεις)

Αν χρειάζεσαι βοήθεια, άνοιξε issue με το error code.

---

## 📞 Support

- **Issues**: [GitHub Issues](your-repo-url/issues)
- **Documentation**: This README + inline code comments
- **myDATA Specs**: [AADE myDATA Documentation](https://www.aade.gr/mydata)

---

## 📊 Project Stats

- **Lines of Code**: ~15,000
- **Validation Rules**: 55
- **Invoice Types**: 17/17 (100%)
- **Tests**: 12 (all passing)
- **Languages**: Rust (backend), TypeScript (frontend)
- **Build Time**: ~10s (backend), ~30s (frontend)

---

## 🗺️ Roadmap

- [ ] Docker containers για easy deployment
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Integration tests
- [ ] Load testing (1000+ files)
- [ ] Prometheus metrics
- [ ] API key authentication
- [ ] WebSocket για real-time progress
- [ ] Support για PDF parsing
- [ ] Mobile-responsive UI improvements

---

Δημιουργήθηκε με ❤️ για την ελληνική επιχειρηματική κοινότητα
