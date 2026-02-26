export enum InvoiceType {
  SalesInvoice = "1.1",
  SalesInvoiceIntra = "1.2",
  ServiceInvoice = "2.1",
  RetailReceipt = "11.1",
  ServiceReceipt = "11.2",
  Unknown = "Unknown"
}

export enum VatCategory {
  Vat24 = "1",
  Vat13 = "2",
  Vat6 = "3",
  Vat17 = "4",
  Vat9 = "5",
  Vat4 = "6",
  Vat0 = "7",
  Excluded = "8"
}

export enum ValidationStatus {
    Green = "Green",
    Yellow = "Yellow",
    Red = "Red"
}

export enum Severity {
    Info = "Info",
    Warning = "Warning",
    Error = "Error"
}

export interface ExplainableError {
    code: string;
    field?: string;
    value_found?: string;
    reason: string;
    allowed_values?: string[];
    severity: Severity;
}

export interface FixHint {
    field: string;
    suggestion: string;
}

export interface ValidationReport {
    status: ValidationStatus;
    risk_score: number;
    summary: string;
    errors: ExplainableError[];
    suggestions: FixHint[];
}

export interface BatchFileResult {
    filename: string;
    status: string; // "success" | "error"
    reports: ValidationReport[];
    error_message?: string;
}