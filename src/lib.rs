//! # idsmith
//!
//! Forge valid, checksum-correct IBANs, personal IDs, and bank accounts
//! for 252 countries. Every generated code passes mod-97 (IBAN) and
//! national checksum validation.
//!
//! ## Quick Start
//!
//! ```rust
//! use rand::thread_rng;
//! use idsmith::{iban, personal_id};
//!
//! let mut rng = thread_rng();
//!
//! // Generate a German IBAN
//! let code = iban::generate_iban(Some("DE"), &mut rng).unwrap();
//! println!("{}", iban::format_iban(&code));
//!
//! // Generate an Estonian personal ID
//! let registry = personal_id::Registry::new();
//! let opts = personal_id::GenOptions::default();
//! let id = registry.generate("EE", &opts, &mut rng).unwrap();
//! println!("{}", id);
//! ```

//! # idsmith
//!
//! A comprehensive **Generator** and **Validator** for valid, checksum-correct
//! identifiers across 252 countries. Supports IBANs, Personal IDs, Bank Accounts,
//! Credit Cards, SWIFT/BIC, and Company IDs.
//!
//! ## Quick Start (Generation)
//!
//! ```rust
//! use rand::thread_rng;
//! use idsmith::{bank_accounts, personal_ids};
//!
//! let mut rng = thread_rng();
//!
//! // Generate a German IBAN
//! let code = idsmith::iban::generate_iban(Some("DE"), &mut rng).unwrap();
//!
//! // Generate an Estonian personal ID
//! let id = personal_ids().generate("EE", &Default::default(), &mut rng).unwrap();
//! ```
//!
//! ## Quick Start (Validation)
//!
//! ```rust
//! use idsmith::{credit_cards, personal_ids};
//!
//! // Validate a credit card number
//! let is_valid = credit_cards().validate("4152839405126374");
//!
//! // Validate a US Social Security Number
//! let is_ssn_valid = personal_ids().validate("US", "446-72-2445").unwrap_or(false);
//! ```

pub mod bank_account;
pub mod company_id;
pub mod credit_card;
pub mod iban;
pub mod personal_id;
pub mod swift;

#[cfg(feature = "csv")]
pub mod csv;

use std::sync::OnceLock;

/// Global registry for bank accounts.
/// Provides methods to generate, validate, and format bank account numbers for 252 countries.
pub fn bank_accounts() -> &'static bank_account::Registry {
    static REGISTRY: OnceLock<bank_account::Registry> = OnceLock::new();
    REGISTRY.get_or_init(bank_account::Registry::new)
}

/// Global registry for personal IDs.
/// Provides methods to generate, validate, and parse national ID numbers (SSN, CPF, Aadhaar, etc.) for 252 countries.
pub fn personal_ids() -> &'static personal_id::Registry {
    static REGISTRY: OnceLock<personal_id::Registry> = OnceLock::new();
    REGISTRY.get_or_init(personal_id::Registry::new)
}

/// Global registry for credit cards.
/// Provides methods to generate and validate credit card numbers for major brands (Visa, Mastercard, Amex, etc.).
pub fn credit_cards() -> &'static credit_card::Registry {
    static REGISTRY: OnceLock<credit_card::Registry> = OnceLock::new();
    REGISTRY.get_or_init(credit_card::Registry::new)
}

/// Global registry for company IDs.
/// Provides methods to generate and validate business identifiers (VAT, EIN, CIF) for all countries.
pub fn company_ids() -> &'static company_id::Registry {
    static REGISTRY: OnceLock<company_id::Registry> = OnceLock::new();
    REGISTRY.get_or_init(company_id::Registry::new)
}

/// Global registry for SWIFT codes.
/// Provides methods to generate and validate 8 and 11 character SWIFT/BIC codes.
pub fn swift_codes() -> &'static swift::Registry {
    static REGISTRY: OnceLock<swift::Registry> = OnceLock::new();
    REGISTRY.get_or_init(swift::Registry::new)
}
