# idsmith

Validate and generate checksum-correct **IBANs**, **personal IDs**, **bank accounts**, **credit cards**, **SWIFT/BIC**, and **company IDs** for 252 countries.

Built for developers and QA engineers who need a robust way to validate existing identifiers or create realistic, algorithmically valid test data.

## Features

- **Validator + Generator** — verify existing strings or create new data
- **96 IBAN countries** — full IBAN registry coverage with mod-97-10 checksum validation
- **252 bank account formats** — US ABA routing, MX CLABE, AU BSB, IN IFSC, AR CBU, NG NUBAN, BR mod-11, etc.
- **252 personal ID formats** — 56 checksum-verified (SSN, CPF, Aadhaar, Resident ID) + generic fallbacks
- **252 company ID formats** — VAT numbers, EINs, and Business IDs with major economy checksums
- **Universal Credit Cards** — Visa, Mastercard, Amex, Discover, JCB, and Diners Club with Luhn support
- **SWIFT/BIC** — valid 8 and 11 character codes with ISO country positioning
- **Single binary** — instant startup, no runtime dependencies
- **Library + CLI** — use as a lightweight Rust crate or a standalone tool
- **JSON & CSV export** — built-in support for RFC 4180 CSV and pretty-printed JSON

## Installation

```bash
cargo install --path .
```

## CLI Usage

### 1. Validating Data
Use the `validate` command to check if a code is checksum and format correct.

```bash
# Validate an IBAN
idsmith validate iban DE47508562162522867909

# Validate a National ID (requires country code)
idsmith validate id 446-72-2445 --country US

# Validate a Credit Card
idsmith validate card 5590133141634919
```

### 2. Generating Data
Generate any identifier using subcommands. Use the optional count positional argument.

```bash
# Generate 5 German IBANs
idsmith iban DE 5

# Generate 3 US Bank Accounts in JSON
idsmith account 3 --country US --json -

# Generate a random Credit Card
idsmith card --brand amex
```

## Library (SDK) Usage

Add `idsmith` to your `Cargo.toml`. For a lightweight build, disable default features to exclude the CLI logic.

```toml
[dependencies]
idsmith = { version = "0.4.0", default-features = false }
```

### SDK Validation API

```rust
use idsmith::{credit_cards, personal_ids, bank_accounts, company_ids, swift_codes};

// Global registries provide thread-safe, easy access
let cards = credit_cards();
let ids = personal_ids();

// 1. Validate simple formats (returns bool)
let card_ok = cards.validate("4152839405126374");
let swift_ok = swift_codes().validate("PBIHNLY9XXX");

// 2. Validate country-specific formats (returns Option<bool>)
// None is returned if the country is not supported or structurally invalid
let ssn_ok = ids.validate("US", "446-72-2445").unwrap_or(false);
let vat_ok = company_ids().validate("DE", "DE141158922").unwrap_or(false);
let acc_ok = bank_accounts().validate("MX", "167078019952865929").unwrap_or(false);

// 3. IBAN validation
let iban_ok = idsmith::iban::validate_iban("DE47508562162522867909");
```

### SDK Generation API

```rust
use rand::thread_rng;
let mut rng = thread_rng();

// Generate with default options
let card = credit_cards().generate(&Default::default(), &mut rng).unwrap();
let id = ids.generate("BR", &Default::default(), &mut rng).unwrap();

// Parse an existing ID to extract metadata
let result = ids.parse("FI", "050497-598S").unwrap();
println!("DOB: {:?}, Gender: {:?}", result.dob, result.gender);
```

## Validation Standards

Accuracy is ensured by cross-validating against established industry libraries. Verification scripts are available in the `scripts/` directory.

| Category | Validation Source |
|----------|-------------------|
| IBAN | [ibantools](https://www.npmjs.com/package/ibantools) |
| Personal ID | [python-stdnum](https://github.com/arthurdejong/python-stdnum), [taiwan-id](https://www.npmjs.com/package/taiwan-id) |
| Bank Account | [abavalidator](https://www.npmjs.com/package/abavalidator), [clabe-validator](https://github.com/center-key/clabe-validator) |
| Credit Card | ISO/IEC 7812 (Luhn Algorithm) |
| SWIFT/BIC | ISO 9362 Standards |

## License

MIT