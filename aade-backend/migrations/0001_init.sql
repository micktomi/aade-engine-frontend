CREATE TABLE IF NOT EXISTS validation_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_hash VARCHAR(64) NOT NULL,
    issuer_vat VARCHAR(20) NOT NULL,
    invoice_series VARCHAR(50),
    invoice_aa VARCHAR(50),
    is_valid BOOLEAN NOT NULL,
    errors_json JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_invoice_hash ON validation_logs(invoice_hash);
CREATE INDEX idx_issuer_vat ON validation_logs(issuer_vat);
