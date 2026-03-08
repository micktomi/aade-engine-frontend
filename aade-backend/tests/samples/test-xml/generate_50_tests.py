import os
import random
from datetime import datetime, timedelta

output_dir = "aade-backend/tests/samples/test-xml"
os.makedirs(output_dir, exist_ok=True)

base_xml = """<?xml version='1.0' encoding='UTF-8'?>
<ns0:InvoicesDoc xmlns:ns0="http://www.aade.gr/myDATA/invoice/v1.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.aade.gr/myDATA/invoice/v1.0/InvoicesDoc-v0.6.xsd">
  <ns0:invoice>
    <ns0:issuer>
      <ns0:vatNumber>090000045</ns0:vatNumber>
      <ns0:country>GR</ns0:country>
      <ns0:branch>0</ns0:branch>
    </ns0:issuer>
    <ns0:counterpart>
      <ns0:vatNumber>{counterpart_vat}</ns0:vatNumber>
      <ns0:country>{counterpart_country}</ns0:country>
      <ns0:branch>0</ns0:branch>
      <ns0:name>Test Customer</ns0:name>
    </ns0:counterpart>
    <ns0:invoiceHeader>
      <ns0:series>A</ns0:series>
      <ns0:aa>{aa}</ns0:aa>
      <ns0:issueDate>{issue_date}</ns0:issueDate>
      <ns0:invoiceType>{invoice_type}</ns0:invoiceType>
      <ns0:currency>{currency}</ns0:currency>
    </ns0:invoiceHeader>
    <ns0:paymentMethods>
      <ns0:paymentMethodDetails>
        <ns0:type>{payment_type}</ns0:type>
        <ns0:amount>{gross_value}</ns0:amount>
        <ns0:paymentMethodInfo>Info</ns0:paymentMethodInfo>
      </ns0:paymentMethodDetails>
    </ns0:paymentMethods>
    <ns0:invoiceDetails>
      <ns0:lineNumber>1</ns0:lineNumber>
      <ns0:netValue>{net_value}</ns0:netValue>
      <ns0:vatCategory>{vat_category}</ns0:vatCategory>
      <ns0:vatAmount>{vat_amount}</ns0:vatAmount>
    </ns0:invoiceDetails>
    <ns0:invoiceSummary>
      <ns0:totalNetValue>{net_value}</ns0:totalNetValue>
      <ns0:totalVatAmount>{vat_amount}</ns0:totalVatAmount>
      <ns0:totalWithheldAmount>0.00</ns0:totalWithheldAmount>
      <ns0:totalFeesAmount>0.00</ns0:totalFeesAmount>
      <ns0:totalStampDutyAmount>0.00</ns0:totalStampDutyAmount>
      <ns0:totalDeductionsAmount>0.00</ns0:totalDeductionsAmount>
      <ns0:totalGrossValue>{gross_value}</ns0:totalGrossValue>
      <ns0:incomeClassification>
        <ns0:classificationType>{class_type}</ns0:classificationType>
        <ns0:classificationCategory>{class_cat}</ns0:classificationCategory>
        <ns0:amount>{net_value}</ns0:amount>
      </ns0:incomeClassification>
    </ns0:invoiceSummary>
  </ns0:invoice>
</ns0:InvoicesDoc>
"""

invoice_types = ["1.1", "1.2", "2.1", "3.1", "5.1", "11.1", "11.2", "14.30"]
vat_rates = {
    "1": 0.24,
    "2": 0.13,
    "3": 0.06,
    "4": 0.04,
    "5": 0.03,
    "6": 0.00,
    "7": 0.00 # Without VAT
}
payment_types = ["1", "2", "3", "4", "5"]

# Let's generate 50 XMLs
for i in range(1, 51):
    scenario = "valid" if i <= 30 else "invalid"
    
    inv_type = random.choice(invoice_types)
    net_val = round(random.uniform(10.0, 1000.0), 2)
    vat_cat = random.choice(list(vat_rates.keys()))
    
    vat_amt = round(net_val * vat_rates[vat_cat], 2)
    
    if scenario == "invalid":
        # Introduce an error
        error_type = random.choice(["math", "date", "vat", "currency", "afm"])
        if error_type == "math":
            vat_amt += 1.0  # Incorrect VAT amount
            reason = "math_error"
        elif error_type == "date":
            issue_date = (datetime.now() + timedelta(days=5)).strftime('%Y-%m-%d') # Future date
            reason = "future_date"
        elif error_type == "vat":
            vat_cat = "99" # Invalid vat category
            reason = "invalid_vat_cat"
        elif error_type == "currency":
            currency = "XYZ" # Invalid currency
            reason = "invalid_currency"
        elif error_type == "afm":
            counterpart_vat = "123" # Invalid AFM
            reason = "invalid_afm"
        else:
            reason = "other"
    else:
        issue_date = (datetime.now() - timedelta(days=random.randint(1, 30))).strftime('%Y-%m-%d')
        currency = random.choice(["EUR", "USD", "GBP"])
        counterpart_vat = "090000045"
        reason = "valid"

    if scenario == "valid":
        filename = f"gen_{i:02d}_{scenario}_type_{inv_type.replace('.','_')}.xml"
    else:
        filename = f"gen_{i:02d}_{scenario}_{reason}.xml"

    gross_val = round(net_val + vat_amt, 2)
    
    class_type = "E3_561_001"
    class_cat = "category1_1"

    xml_content = base_xml.format(
        counterpart_vat=counterpart_vat if scenario != "invalid" or error_type != "afm" else "123",
        counterpart_country="GR" if currency == "EUR" else "US",
        aa=100 + i,
        issue_date=issue_date if scenario != "invalid" or error_type != "date" else (datetime.now() + timedelta(days=5)).strftime('%Y-%m-%d'),
        invoice_type=inv_type,
        currency=currency if scenario != "invalid" or error_type != "currency" else "XYZ",
        payment_type=random.choice(payment_types),
        net_value=f"{net_val:.2f}",
        vat_category=vat_cat if scenario != "invalid" or error_type != "vat" else "99",
        vat_amount=f"{vat_amt:.2f}",
        gross_value=f"{gross_val:.2f}",
        class_type=class_type,
        class_cat=class_cat
    )

    with open(os.path.join(output_dir, filename), "w", encoding="utf-8") as f:
        f.write(xml_content)

print("Successfully generated 50 test XML files.")
