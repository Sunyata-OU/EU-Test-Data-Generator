use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct SwiftResult {
    pub code: String,
    pub bank: String,
    pub country: String,
    pub location: String,
    pub branch: Option<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub country: Option<String>,
}

pub struct Registry;

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> SwiftResult {
        let country = opts.country.as_deref().unwrap_or_else(|| {
            let countries = ["US", "GB", "DE", "FR", "IN", "CN", "JP", "AU", "CA", "CH", "IT", "ES", "NL", "SG", "HK"];
            countries[rng.gen_range(0..countries.len())]
        }).to_uppercase();

        let bank = match country.as_str() {
            "US" => ["CHAS", "CITI", "BOFA", "JPMC", "WFCU"].choose(rng),
            "GB" => ["BARC", "LLOY", "HSBC", "NWRS", "RBOS"].choose(rng),
            "DE" => ["DEUT", "COBA", "DABA", "DRE2", "DZAD"].choose(rng),
            "FR" => ["BNPA", "SOGE", "CRLY", "BCIT", "BCEP"].choose(rng),
            _ => None,
        }.map(|s| s.to_string()).unwrap_or_else(|| {
            (0..4).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect()
        });

        let location = format!("{}{}", 
            rng.gen_range(b'A'..=b'Z') as char,
            rng.gen_range(b'2'..=b'9') as char // Often a digit or letter
        );

        let branch = if rng.gen_bool(0.3) {
            Some((0..3).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect())
        } else if rng.gen_bool(0.2) {
            Some("XXX".to_string())
        } else {
            None
        };

        let code = format!("{}{}{}{}", bank, country, location, branch.as_deref().unwrap_or(""));

        SwiftResult {
            code,
            bank,
            country,
            location,
            branch,
            valid: true,
        }
    }

    pub fn validate(&self, code: &str) -> bool {
        let len = code.len();
        if len != 8 && len != 11 {
            return false;
        }
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
        // Country code should be letters
        if !code[4..6].chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }
        true
    }
}

trait ChooseExt<T> {
    fn choose(&self, rng: &mut impl Rng) -> Option<&T>;
}

impl<T> ChooseExt<T> for [T] {
    fn choose(&self, rng: &mut impl Rng) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.gen_range(0..self.len())])
        }
    }
}
