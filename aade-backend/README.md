# AADE Validation Engine Backend

## Περιγραφή

Το **AADE Validation Engine** είναι ένα high-performance backend σύστημα γραμμένο σε **Rust** που επικυρώνει XML παραστατικά myDATA της ΑΑΔΕ (Ανεξάρτητη Αρχή Δημοσίων Εσόδων). Το σύστημα ελέγχει τη συμμόρφωση των τιμολογίων με τους κανόνες της ελληνικής φορολογικής νομοθεσίας πριν την υποβολή τους στο myDATA.

### Βασικά Χαρακτηριστικά

- ✅ **Πλήρης Επικύρωση myDATA v1.0**: Υποστηρίζει όλους τους τύπους παραστατικών (1.1-11.5)
- 🚀 **Υψηλή Απόδοση**: Γραμμένο σε Rust για μέγιστη ταχύτητα και ασφάλεια μνήμης
- 📋 **Batch Processing**: Επεξεργασία πολλαπλών XML αρχείων ταυτόχρονα
- 🔧 **Δυναμικοί Κανόνες**: Οι κανόνες επικύρωσης ορίζονται σε YAML αρχεία (rules/mydata_v1.yaml)
- 🔒 **Rate Limiting**: Προστασία από abuse (100 requests/15min ανά IP)
- 📊 **Logging σε PostgreSQL**: Προαιρετική καταγραφή επικυρώσεων για analytics
- 🌐 **REST API**: Εύκολη ενσωμάτωση με οποιοδήποτε frontend ή ERP σύστημα

---

## Αρχιτεκτονική

### Τεχνολογίες

- **Rust 1.91+**: Core language
- **Axum**: Async web framework
- **Tokio**: Async runtime
- **SQLx**: PostgreSQL driver (προαιρετικό)
- **quick-xml**: XML parsing
- **serde**: Serialization/Deserialization
- **rust_decimal**: Ακριβείς υπολογισμοί χρηματικών ποσών

### Δομή Project

```
aade-backend/
├── src/
│   ├── main.rs                    # Entry point, server setup
│   ├── api/
│   │   ├── validate.rs            # Validation endpoints
│   │   └── health.rs              # Health check endpoints
│   ├── domain/
│   │   ├── invoice.rs             # Domain models (Invoice, Header, Line)
│   │   ├── enums.rs               # InvoiceType, VatCategory enums
│   │   ├── totals.rs              # InvoiceTotals struct
│   │   ├── vat.rs                 # VAT breakdown
│   │   └── classification.rs     # Income classifications
│   ├── xml/
│   │   ├── parser.rs              # XML → Raw structs
│   │   └── normalizer.rs          # Raw → Domain models
│   ├── validation/
│   │   ├── business_rules.rs     # Hardcoded validation logic
│   │   ├── rules_engine.rs       # YAML-based rules engine
│   │   ├── result.rs             # ValidationReport structure
│   │   └── diff.rs               # XML comparison utilities
│   ├── persistence/
│   │   ├── db.rs                 # Database connection
│   │   └── validation_log.rs     # Logging validation results
│   ├── utils/
│   │   ├── afm.rs                # Greek VAT (AFM) validation
│   │   └── hash.rs               # XML hashing for deduplication
│   ├── config.rs                 # Environment configuration
│   ├── state.rs                  # Application state
│   └── error.rs                  # Error types
├── rules/
│   └── mydata_v1.yaml            # Validation rules (25KB, 200+ rules)
├── migrations/
│   └── 0001_init.sql             # Database schema
├── tests/
│   ├── validation_tests.rs       # Unit tests
│   └── samples/                  # Test XML files
├── Cargo.toml                    # Dependencies
├── Dockerfile                    # Multi-stage build
└── docker-compose.yml            # Local development setup
```

---

## Λειτουργία του Engine

### 1. XML Parsing & Normalization

```
XML Input → Parser → XmlInvoice (raw) → Normalizer → Invoice (domain model)
```

- **Parser** (`xml/parser.rs`): Διαβάζει το XML και το μετατρέπει σε Rust structs
- **Normalizer** (`xml/normalizer.rs`): Καθαρίζει τα δεδομένα, μετατρέπει τύπους (String → Enum, f64 → Decimal)

### 2. Validation Pipeline

```
Invoice → BusinessRules::validate() → ValidationReport
                ↓
    ┌───────────┴───────────┐
    │                       │
Static Rules          Dynamic Rules
(Hardcoded)          (YAML Engine)
```

#### Static Rules (business_rules.rs)

Σύνθετοι έλεγχοι που απαιτούν προγραμματιστική λογική:

- **BR-001**: Έλεγχος αθροίσματος καθαρής αξίας γραμμών
- **BR-002**: Έλεγχος αθροίσματος ΦΠΑ γραμμών
- **BR-007**: Έλεγχος τελικής αξίας (Net + VAT + Fees + Stamp - Deductions - Withheld)
- **BR-003/004**: Επικύρωση ΑΦΜ εκδότη/λήπτη (αλγόριθμος Luhn)
- **BR-005**: Έλεγχος ημερομηνίας (δεν μπορεί να είναι μελλοντική)
- **BR-VAT-CALC**: Έλεγχος υπολογισμού ΦΠΑ ανά γραμμή
- **BR-CLS-TOTAL**: Έλεγχος αθροίσματος χαρακτηρισμών εσόδων

#### Dynamic Rules (rules_engine.rs + mydata_v1.yaml)

Δηλωτικοί κανόνες που ορίζονται σε YAML (200+ rules):

**Τύποι Κανόνων:**

1. **LineValueAllowed**: Επιτρεπτές τιμές πεδίων γραμμής
   ```yaml
   - id: "VAT-001"
     logic:
       type: "LineValueAllowed"
       field_path: "vat_category"
       allowed_values: ["1", "2", "3", "4", "5", "6", "7", "8"]
   ```

2. **HeaderDependencyLine**: Εξαρτήσεις header → line
   ```yaml
   - id: "VAT-002"
     logic:
       type: "HeaderDependencyLine"
       header_field: "invoice_type"
       header_value: "1.1"
       line_check_field: "vat_category"
       allowed_values: ["1", "2", "3"]
   ```

3. **CounterpartRequired**: Υποχρεωτικός λήπτης για B2B
4. **ClassificationRequired**: Υποχρεωτικοί χαρακτηρισμοί
5. **CurrencyExchangeRate**: Έλεγχος ισοτιμίας για ξένο νόμισμα
6. **NegativeAmountsOnly**: Αρνητικά ποσά (πιστωτικά)
7. **PaymentMethodValueAllowed**: Επιτρεπτοί τρόποι πληρωμής
8. **WithheldTaxPercentage**: Έλεγχος παρακράτησης φόρου (20%)
9. **StampDutyPercentage**: Έλεγχος χαρτοσήμου (3.6%)

### 3. Validation Report

Το αποτέλεσμα είναι ένα `ValidationReport` με:

```json
{
  "valid": false,
  "errors": [
    {
      "code": "BR-001",
      "message": "Το υπολογισμένο Καθαρό Ποσό (100.00) δεν συμφωνεί με το σύνολο (105.00)",
      "field": "totalNetValue",
      "value": "105.00"
    }
  ],
  "warnings": [
    {
      "code": "W-001",
      "message": "Η γραμμή 1 έχει μηδενική καθαρή αξία"
    }
  ],
  "info": []
}
```

**Severity Levels:**
- **Error**: Κρίσιμο σφάλμα, το παραστατικό θα απορριφθεί από myDATA
- **Warning**: Πιθανό πρόβλημα, αλλά μπορεί να γίνει αποδεκτό
- **Info**: Πληροφοριακό μήνυμα

---

## API Endpoints

### 1. Single Validation

```http
POST /validate
Content-Type: text/xml

<InvoicesDoc>
  <invoice>
    <!-- XML content -->
  </invoice>
</InvoicesDoc>
```

**Response:**
```json
[
  {
    "valid": true,
    "errors": [],
    "warnings": [],
    "info": []
  }
]
```

### 2. Batch Validation

```http
POST /validate/batch
Content-Type: multipart/form-data

file1: invoice1.xml
file2: invoice2.xml
```

**Response:**
```json
[
  {
    "filename": "invoice1.xml",
    "status": "success",
    "reports": [{ "valid": true, ... }],
    "error_message": null
  },
  {
    "filename": "invoice2.xml",
    "status": "error",
    "reports": [],
    "error_message": "XML Parse Failed: unexpected element"
  }
]
```

### 3. Health Checks

```http
GET /health/ready   # Readiness probe (DB connection check)
GET /health/live    # Liveness probe (always returns 200)
```

---

## Εγκατάσταση & Εκτέλεση

### Προαπαιτούμενα

- **Rust 1.91+**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **PostgreSQL 15+** (προαιρετικό): Για logging
- **Docker & Docker Compose** (προαιρετικό): Για containerized deployment

### Local Development

#### 1. Clone & Setup

```bash
git clone <repository-url>
cd aade-backend
cp .env.example .env  # Επεξεργασία .env με τις ρυθμίσεις σας
```

#### 2. Ρύθμιση .env

```bash
DATABASE_URL=postgres://user:password@localhost:5432/aade_db
ENVIRONMENT=development
PORT=3000
CORS_ALLOWED_ORIGINS=http://localhost:5173
```

#### 3. Εκτέλεση με Docker Compose (Recommended)

```bash
docker-compose up --build
```

Το API θα είναι διαθέσιμο στο `http://localhost:3000`

#### 4. Εκτέλεση Native (χωρίς Docker)

```bash
# Εγκατάσταση dependencies
cargo build --release

# Εκτέλεση migrations (αν χρησιμοποιείτε DB)
cargo install sqlx-cli
sqlx migrate run

# Εκκίνηση server
cargo run --release
```

### Production Deployment

#### Docker Build

```bash
docker build -t aade-validator:latest .
docker run -p 3000:3000 \
  -e DATABASE_URL=postgres://user:pass@db:5432/aade_db \
  -e ENVIRONMENT=production \
  -e CORS_ALLOWED_ORIGINS=https://yourdomain.com \
  aade-validator:latest
```

#### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | No | - | PostgreSQL connection string (logging disabled if missing) |
| `ENVIRONMENT` | No | `development` | `development` or `production` (affects CORS) |
| `PORT` | No | `3000` | Server port |
| `CORS_ALLOWED_ORIGINS` | No | `*` (dev) | Comma-separated list of allowed origins |

---

## Testing

### Unit Tests

```bash
# Εκτέλεση όλων των tests
cargo test

# Εκτέλεση με verbose output
cargo test -- --nocapture

# Εκτέλεση συγκεκριμένου test
cargo test test_vat_calculation
```

### Integration Tests

```bash
# Validation tests με sample XML
cargo test --test validation_tests

# Diff tests (XML comparison)
cargo test --test diff_tests
```

### Test Coverage

Το project περιλαμβάνει:
- **Unit tests** στο `rules_engine.rs` (10+ tests)
- **Integration tests** στο `tests/` directory
- **Sample XML files** στο `tests/samples/`

---

## Κανόνες Επικύρωσης

### Κατηγορίες Κανόνων

#### 1. Counterpart Validation (CP-001 έως CP-005)
- Υποχρεωτικός λήπτης για B2B τιμολόγια
- Έλεγχος χώρας λήπτη (ενδοκοινοτικές, τρίτες χώρες)

#### 2. VAT Validation (VAT-001 έως VAT-020)
- Επιτρεπτές κατηγορίες ΦΠΑ ανά τύπο παραστατικού
- Υποχρεωτικός λόγος απαλλαγής για 0% ΦΠΑ
- Έλεγχος υπολογισμού ΦΠΑ

#### 3. Classification Validation (CLS-001 έως CLS-010)
- Υποχρεωτικοί χαρακτηρισμοί εσόδων
- Έλεγχος τύπων χαρακτηρισμών (E3_561_001, E3_880_001, κλπ)

#### 4. Payment Method Validation (PAY-001 έως PAY-005)
- Επιτρεπτοί τρόποι πληρωμής (1=Μετρητά, 3=Κάρτα, 5=Επί Πιστώσει, κλπ)

#### 5. Amount Validation (AMT-001 έως AMT-010)
- Έλεγχος αρνητικών ποσών (πιστωτικά)
- Έλεγχος μη-αρνητικών πεδίων (παρακρατήσεις, χαρτόσημο)
- Έλεγχος συνολικών ποσών

#### 6. Currency & Exchange Rate (CUR-001 έως CUR-003)
- Υποχρεωτική ισοτιμία για ξένο νόμισμα

#### 7. Correlated Invoices (CRN-001 έως CRN-003)
- Υποχρεωτική συσχέτιση για πιστωτικά τιμολόγια

### Προσθήκη Νέων Κανόνων

Επεξεργασία του `rules/mydata_v1.yaml`:

```yaml
- id: "CUSTOM-001"
  description: "Περιγραφή κανόνα"
  severity: "Error"  # Error | Warning | Info
  logic:
    type: "LineValueAllowed"
    field_path: "vat_category"
    allowed_values: ["1", "2"]
  error_message: "Μήνυμα σφάλματος με {placeholders}"
```

**Placeholders:**
- `{invoice_type}`: Τύπος παραστατικού
- `{currency}`: Νόμισμα
- `{line}`: Αριθμός γραμμής
- `{value_a}`, `{value_b}`: Τιμές για comparisons

---

## Performance & Scalability

### Benchmarks

- **Single Validation**: ~5-10ms (μέσος όρος)
- **Batch 100 files**: ~500ms-1s
- **Memory Usage**: ~50MB (idle), ~200MB (under load)
- **Throughput**: ~1000 validations/sec (single core)

### Rate Limiting

- **Default**: 100 requests per 15 minutes per IP
- **Burst**: 100 requests (allows temporary spikes)
- **Configurable**: Μπορεί να τροποποιηθεί στο `main.rs`

### Database Logging

Το logging είναι **προαιρετικό** και **non-blocking**:
- Αν η DB δεν είναι διαθέσιμη, το API συνεχίζει να λειτουργεί
- Timeout: 2 seconds για DB connection
- Χρήσιμο για analytics και audit trails

---

## Troubleshooting

### Συνήθη Προβλήματα

#### 1. "Database connection timeout"

**Λύση**: Το API λειτουργεί χωρίς DB. Αν θέλετε logging:
```bash
# Ελέγξτε αν τρέχει η PostgreSQL
docker-compose ps

# Ελέγξτε το DATABASE_URL στο .env
echo $DATABASE_URL
```

#### 2. "CORS error" στο frontend

**Λύση**: Προσθέστε το frontend URL στο `.env`:
```bash
CORS_ALLOWED_ORIGINS=http://localhost:5173,https://yourdomain.com
```

#### 3. "XML Parse Failed"

**Λύση**: Ελέγξτε ότι το XML είναι valid myDATA format:
```bash
# Validate XML structure
xmllint --noout your-invoice.xml
```

#### 4. "Rate limit exceeded"

**Λύση**: Περιμένετε 15 λεπτά ή αυξήστε το limit στο `main.rs`:
```rust
.per_second(2)  // 2 req/sec instead of 1
.burst_size(200) // 200 burst instead of 100
```

### Logs

```bash
# Docker logs
docker-compose logs -f app

# Native logs (με RUST_LOG)
RUST_LOG=debug cargo run
```

---

## Roadmap

### Planned Features

- [ ] **WebSocket support**: Real-time validation για live editing
- [ ] **Caching layer**: Redis για repeated validations
- [ ] **Advanced analytics**: Dashboard για validation statistics
- [ ] **Multi-tenant support**: API keys για διαφορετικούς clients
- [ ] **PDF generation**: Δημιουργία PDF από validated XML
- [ ] **AADE API integration**: Άμεση υποβολή στο myDATA
- [ ] **Rule versioning**: Υποστήριξη πολλαπλών versions κανόνων

### Known Limitations

- Δεν υποστηρίζει όλα τα optional πεδία του myDATA (π.χ. `movePurpose`, `fuelCode`)
- Δεν ελέγχει digital signatures (ADES)
- Δεν επικοινωνεί με το AADE API (μόνο validation)

---

## Contributing

### Development Workflow

1. Fork το repository
2. Δημιουργία feature branch: `git checkout -b feature/new-rule`
3. Commit changes: `git commit -am 'Add new validation rule'`
4. Push to branch: `git push origin feature/new-rule`
5. Δημιουργία Pull Request

### Code Style

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run tests before commit
cargo test
```

---

## License

[Προσθέστε το license σας εδώ]

---

## Support

Για τεχνική υποστήριξη:
- **Issues**: [GitHub Issues](https://github.com/your-repo/issues)
- **Email**: support@yourdomain.com
- **Documentation**: [Wiki](https://github.com/your-repo/wiki)

---

## Credits

Developed with ❤️ using Rust

**Key Dependencies:**
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Database driver
- [quick-xml](https://github.com/tafia/quick-xml) - XML parser
- [rust_decimal](https://github.com/paupino/rust-decimal) - Decimal arithmetic
