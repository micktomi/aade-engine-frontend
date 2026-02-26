use aade_validator::xml::parser::AadeBook;
use aade_validator::xml::normalizer::Normalizer;
use aade_validator::validation::diff;
use quick_xml::de::from_str;

#[test]
fn test_diff_detection() {
    let xml1 = r#"<?xml version="1.0" encoding="UTF-8"?>
<InvoicesDoc xmlns="http://www.aade.gr/myDATA/invoice/v1.0">
  <invoice>
    <issuer><vatNumber>090000045</vatNumber><country>GR</country><branch>0</branch></issuer>
    <invoiceHeader><series>A</series><aa>1</aa><issueDate>2023-10-27</issueDate><invoiceType>1.1</invoiceType><currency>EUR</currency></invoiceHeader>
    <invoiceDetails><lineNumber>1</lineNumber><netValue>100.00</netValue><vatCategory>1</vatCategory><vatAmount>24.00</vatAmount></invoiceDetails>
    <invoiceSummary>
      <totalNetValue>100.00</totalNetValue>
      <totalVatAmount>24.00</totalVatAmount>
      <totalWithheldAmount>0.00</totalWithheldAmount>
      <totalFeesAmount>0.00</totalFeesAmount>
      <totalStampDutyAmount>0.00</totalStampDutyAmount>
      <totalDeductionsAmount>0.00</totalDeductionsAmount>
      <totalGrossValue>124.00</totalGrossValue>
    </invoiceSummary>
  </invoice>
</InvoicesDoc>"#;

    // XML2 has changed VAT number and Net Value
    let xml2 = r#"<?xml version="1.0" encoding="UTF-8"?>
<InvoicesDoc xmlns="http://www.aade.gr/myDATA/invoice/v1.0">
  <invoice>
    <issuer><vatNumber>999999999</vatNumber><country>GR</country><branch>0</branch></issuer>
    <invoiceHeader><series>A</series><aa>1</aa><issueDate>2023-10-27</issueDate><invoiceType>1.1</invoiceType><currency>EUR</currency></invoiceHeader>
    <invoiceDetails><lineNumber>1</lineNumber><netValue>200.00</netValue><vatCategory>1</vatCategory><vatAmount>48.00</vatAmount></invoiceDetails>
    <invoiceSummary>
      <totalNetValue>200.00</totalNetValue>
      <totalVatAmount>48.00</totalVatAmount>
      <totalWithheldAmount>0.00</totalWithheldAmount>
      <totalFeesAmount>0.00</totalFeesAmount>
      <totalStampDutyAmount>0.00</totalStampDutyAmount>
      <totalDeductionsAmount>0.00</totalDeductionsAmount>
      <totalGrossValue>248.00</totalGrossValue>
    </invoiceSummary>
  </invoice>
</InvoicesDoc>"#;

    let book1: AadeBook = from_str(xml1).unwrap();
    let inv1 = Normalizer::normalize(book1.invoices.into_iter().next().unwrap()).unwrap();

    let book2: AadeBook = from_str(xml2).unwrap();
    let inv2 = Normalizer::normalize(book2.invoices.into_iter().next().unwrap()).unwrap();

    let report = diff::compare(&inv1, &inv2);
    println!("Changes found: {:#?}", report.changes);

    assert!(report.has_changes);
    
    // Check for VAT change
    let vat_change = report.changes.iter().any(|c| c.path == "issuer.vat_number" && c.new_value == "999999999");
    assert!(vat_change, "Should detect issuer VAT change");

    // Check for Net Value change on line 1
    let line_change = report.changes.iter().any(|c| c.path == "line[1].net_value" && c.new_value == "200");
    assert!(line_change, "Should detect net value change on line 1");

    println!("Diff detected successfully: {:#?}", report.changes);
}
