use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Green,  // All good
    Yellow, // Warnings exist, but valid
    Red,    // Critical errors, invalid
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainableError {
    pub code: String,
    pub field: Option<String>,
    pub value_found: Option<String>,
    pub reason: String,
    pub allowed_values: Option<Vec<String>>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixHint {
    pub field: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub status: ValidationStatus,
    pub risk_score: u8,
    pub summary: String,
    pub errors: Vec<ExplainableError>,
    pub suggestions: Vec<FixHint>,
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self {
            status: ValidationStatus::Green,
            risk_score: 0,
            summary: "Επιτυχής έλεγχος. Έτοιμο για υποβολή.".to_string(),
            errors: vec![],
            suggestions: vec![],
        }
    }
}

impl ValidationReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_error(
        &mut self,
        code: &str,
        message: &str,
        field: Option<&str>,
        value: Option<&str>,
    ) {
        self.status = ValidationStatus::Red;
        self.risk_score = 100; // Critical error maxes out risk
        self.summary = "Απέτυχε ο έλεγχος. Διορθώστε τα σφάλματα.".to_string();
        
        self.errors.push(ExplainableError {
            code: code.to_string(),
            field: field.map(|s| s.to_string()),
            value_found: value.map(|s| s.to_string()),
            reason: message.to_string(),
            allowed_values: None,
            severity: Severity::Error,
        });
    }

    pub fn add_warning(&mut self, code: &str, message: &str) {
        if self.status == ValidationStatus::Green {
            self.status = ValidationStatus::Yellow;
            self.summary = "Προσοχή. Υπάρχουν επισημάνσεις.".to_string();
            self.risk_score = std::cmp::max(self.risk_score, 30);
        }

        self.errors.push(ExplainableError {
            code: code.to_string(),
            field: None,
            value_found: None,
            reason: message.to_string(),
            allowed_values: None,
            severity: Severity::Warning,
        });
    }

    pub fn add_info(
        &mut self,
        code: &str,
        message: &str,
        field: Option<&str>,
        value: Option<&str>,
    ) {
        if self.status == ValidationStatus::Green {
            self.summary = "Ο έλεγχος ολοκληρώθηκε με ενημερωτικές παρατηρήσεις.".to_string();
        }

        self.errors.push(ExplainableError {
            code: code.to_string(),
            field: field.map(|s| s.to_string()),
            value_found: value.map(|s| s.to_string()),
            reason: message.to_string(),
            allowed_values: None,
            severity: Severity::Info,
        });
    }
}
