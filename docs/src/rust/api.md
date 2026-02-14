# Rust API Reference

The full Rust API documentation is auto-generated from source code using `cargo doc`.

**[View the Rust API Reference →](../api/rust/idsmith/index.html)**

## Key Entry Points

| Function | Description |
|----------|-------------|
| `idsmith::bank_accounts()` | Global registry for bank account operations |
| `idsmith::personal_ids()` | Global registry for personal ID operations |
| `idsmith::credit_cards()` | Global registry for credit card operations |
| `idsmith::company_ids()` | Global registry for company ID operations |
| `idsmith::swift_codes()` | Global registry for SWIFT/BIC operations |
| `idsmith::iban::generate_iban()` | Generate a valid IBAN |
| `idsmith::iban::validate_iban()` | Validate an IBAN string |
| `idsmith::iban::format_iban()` | Format an IBAN with spaces |

## Modules

| Module | Description |
|--------|-------------|
| `idsmith::bank_account` | Bank account types and `Registry` |
| `idsmith::personal_id` | Personal ID types, `Registry`, `Gender` enum |
| `idsmith::credit_card` | Credit card types and `Registry` |
| `idsmith::company_id` | Company ID types and `Registry` |
| `idsmith::swift` | SWIFT/BIC types and `Registry` |
| `idsmith::iban` | IBAN generation, validation, and formatting |
