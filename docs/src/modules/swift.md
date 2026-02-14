# SWIFT/BIC

Generate and validate 8 and 11 character SWIFT/BIC codes with ISO 9362 country positioning.

## Format

A SWIFT code consists of:
- **4 characters** — bank code
- **2 characters** — ISO country code
- **2 characters** — location code
- **3 characters** (optional) — branch code

Example: `CHAS` `US` `U5` `XXX` → `CHASUSU5XXX`

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::swift::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::swift_codes();

// Random country
let result = registry.generate(&GenOptions::default(), &mut rng);

// Specific country
let opts = GenOptions { country: Some("US".to_string()) };
let us = registry.generate(&opts, &mut rng);
// us.code     → "CHASUSU5XXX"
// us.bank     → "CHAS"
// us.country  → "US"
// us.location → "U5"
// us.branch   → Some("XXX")
```

### Python
```python
import idsmith

result = idsmith.Swift.generate()                 # random country
us = idsmith.Swift.generate(country="US")         # specific country
print(us["code"])  # CHASUSU5XXX
```

### JavaScript
```javascript
const { Swift } = require('idsmith');

const result = Swift.generate();      // random country
const us = Swift.generate('US');      // specific country
console.log(us.code);  // CHASUSU5XXX
```

## Validate

### Rust
```rust
idsmith::swift_codes().validate("CHASUSU5XXX");  // true (11 chars)
idsmith::swift_codes().validate("CHASUSU5");     // true (8 chars)
idsmith::swift_codes().validate("INVALID");      // false
```

### Python
```python
idsmith.Swift.validate("CHASUSU5XXX")  # True
idsmith.Swift.validate("CHASUSU5")     # True
```

### JavaScript
```javascript
Swift.validate('CHASUSU5XXX');  // true
Swift.validate('CHASUSU5');     // true
```

## Known Banks

The generator uses real bank codes for major countries:

| Country | Banks |
|---------|-------|
| US | CHAS, CITI, BOFA, JPMC, WFCU |
| GB | BARC, LLOY, HSBC, NWRS, RBOS |
| DE | DEUT, COBA, DABA, DRE2, DZAD |
| FR | BNPA, SOGE, CRLY, BCIT, BCEP |

Other countries generate random 4-letter bank codes.
