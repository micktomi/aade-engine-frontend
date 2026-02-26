use axum::{
    extract::{State, Multipart},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use crate::xml::parser::AadeBook;
use crate::xml::normalizer::Normalizer;
use crate::validation::business_rules::BusinessRules;
use crate::validation::result::ValidationReport;
use crate::persistence::validation_log;
use crate::state::AppState;
use crate::utils::hash;
use quick_xml::de::from_str;

#[derive(Serialize)]
pub struct BatchFileResult {
    filename: String,
    status: String, // "success" (parsed ok) or "error" (xml parse failed)
    reports: Vec<ValidationReport>,
    error_message: Option<String>,
}

pub async fn validate_invoice(
    State(state): State<AppState>,
    body: String
) -> impl IntoResponse {
    match process_xml_content(&state, &body).await {
        Ok(reports) => (StatusCode::OK, Json(reports)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "XML Parse Failed",
            "details": e
        }))).into_response(),
    }
}

pub async fn validate_batch(
    State(state): State<AppState>,
    mut multipart: Multipart
) -> impl IntoResponse {
    let mut results = Vec::new();

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid multipart payload",
                    "details": e.to_string()
                }))).into_response();
            }
        };

        let filename = field.file_name().unwrap_or("unknown.xml").to_string();

        // Read the field data as text
        match field.text().await {
            Ok(content) => match process_xml_content(&state, &content).await {
                Ok(reports) => {
                    results.push(BatchFileResult {
                        filename,
                        status: "success".to_string(),
                        reports,
                        error_message: None,
                    });
                }
                Err(e) => {
                    results.push(BatchFileResult {
                        filename,
                        status: "error".to_string(),
                        reports: vec![],
                        error_message: Some(e),
                    });
                }
            },
            Err(e) => {
                results.push(BatchFileResult {
                    filename,
                    status: "error".to_string(),
                    reports: vec![],
                    error_message: Some(format!("Failed to read file content: {}", e)),
                });
            }
        }
    }

    Json(results).into_response()
}

// Helper function to process a single XML string (used by both single and batch endpoints)
async fn process_xml_content(state: &AppState, content: &str) -> Result<Vec<ValidationReport>, String> {
    let xml_hash = hash::calculate_hash(content);
    let book: AadeBook = from_str(content).map_err(|e| e.to_string())?;

    let mut all_results = Vec::new();

    for xml_inv in book.invoices {
        match Normalizer::normalize(xml_inv) {
            Ok(invoice) => {
                    let res = BusinessRules::validate(&invoice);
                    
                    // Log to Database
                    if let Err(e) = validation_log::log_validation(&state.db, &invoice, &xml_hash, &res).await {
                        tracing::error!("Failed to log validation: {}", e);
                    }

                    all_results.push(res);
            },
            Err(e) => {
                // If normalization fails for one invoice, we currently treat it as a hard error for that invoice context
                // But since we return Vec<ValidationReport>, we might need a way to represent a "Failed Normalization" report.
                // For now, we'll return an error string to the caller, which might fail the whole file.
                // IMPROVEMENT: Create a ValidationReport with a System Error for this specific failure.
                return Err(format!("Normalization Failed: {}", e));
            }
        }
    }

    Ok(all_results)
}
