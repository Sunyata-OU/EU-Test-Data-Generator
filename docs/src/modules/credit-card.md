# Credit Card

Generate and validate credit card numbers for 6 major brands with Luhn checksum. Generated cards include CVV (3 digits, 4 for Amex) and a random future expiration date.

## Supported Brands

| Brand | Prefix | Length |
|-------|--------|--------|
| Visa | 4 | 16 |
| Mastercard | 51-55, 2221-2720 | 16 |
| Amex | 34, 37 | 15 |
| Discover | 6011, 65, 644-649 | 16 |
| JCB | 3528-3589 | 16 |
| Diners | 300-305, 36, 38 | 14 |

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::credit_card::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::credit_cards();

// Random brand
let card = registry.generate(&GenOptions::default(), &mut rng).unwrap();

// Specific brand
let opts = GenOptions { brand: Some("visa".to_string()) };
let visa = registry.generate(&opts, &mut rng).unwrap();
// visa.brand     → "VISA"
// visa.number    → "4152839405126374"
// visa.formatted → "4152 8394 0512 6374"
// visa.cvv       → "123"
// visa.expiry    → "09/28"
```

### Python
```python
import idsmith

card = idsmith.CreditCard.generate()           # random brand
visa = idsmith.CreditCard.generate(brand="visa")  # specific brand
print(visa["formatted"])  # 4152 8394 0512 6374
print(visa["cvv"])        # 123
print(visa["expiry"])     # 09/28
```

### JavaScript
```javascript
const { CreditCard } = require('idsmith');

const card = CreditCard.generate();        // random brand
const visa = CreditCard.generate('visa');  // specific brand
console.log(visa.formatted);  // 4152 8394 0512 6374
console.log(visa.cvv);        // 123
console.log(visa.expiry);     // 09/28
```

## Validate

### Rust
```rust
idsmith::credit_cards().validate("4152839405126374");  // true
idsmith::credit_cards().validate("0000000000000000");  // false
```

### Python
```python
idsmith.CreditCard.validate("4152839405126374")  # True
idsmith.CreditCard.validate("0000000000000000")  # False
```

### JavaScript
```javascript
CreditCard.validate('4152839405126374');  // true
CreditCard.validate('0000000000000000');  // false
```

## Format

### Rust
```rust
idsmith::credit_cards().format("visa", "4152839405126374");
// "4152 8394 0512 6374"

idsmith::credit_cards().format("amex", "371449635398431");
// "3714 496353 98431"
```

### Python
```python
idsmith.CreditCard.format("visa", "4152839405126374")
# "4152 8394 0512 6374"
```

### JavaScript
```javascript
CreditCard.format('visa', '4152839405126374');
// '4152 8394 0512 6374'
```
