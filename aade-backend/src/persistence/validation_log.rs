use sqlx::PgPool;
use crate::domain::invoice::Invoice;
use crate::validation::result::{ValidationReport, ValidationStatus};

pub async fn log_validation(
    pool: &Option<PgPool>,
    invoice: &Invoice,
    hash: &str,
    report: &ValidationReport,
) -> anyhow::Result<()> {
    // If no database is available, skip logging
    let Some(pool) = pool else {
        return Ok(());
    };
    // Serialize errors and suggestions for storage
    let report_json = serde_json::to_value(report)?;

    // Determine validity based on status (Green/Yellow are considered valid for submission, Red is invalid)
    let is_valid = report.status != ValidationStatus::Red;

    sqlx::query(
        r#"
        INSERT INTO validation_logs (invoice_hash, issuer_vat, invoice_series, invoice_aa, is_valid, errors_json)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(hash)
    .bind(&invoice.issuer.vat_number)
    .bind(&invoice.header.series)
    .bind(&invoice.header.aa)
    .bind(is_valid)
    .bind(report_json)
    .execute(pool)
    .await?;

    Ok(())
}
