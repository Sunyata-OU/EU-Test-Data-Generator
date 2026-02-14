mod checksum;
mod countries;
mod generate;
mod types;
mod util;

#[cfg(feature = "json")]
use serde::Serialize;

pub(crate) use countries::get_format;
pub use countries::supported_countries;
pub use generate::{format_iban, generate_iban, validate_iban};
pub use types::{BbanField, CharType};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct IbanResult {
    pub country: String,
    pub iban: String,
    pub formatted: String,
    pub valid: bool,
}
