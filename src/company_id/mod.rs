pub mod generic;

use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;
use crate::personal_id::checksum::iso7064_mod11_10;
use crate::bank_account::checksum::luhn_check_digit;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct CompanyResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
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

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> Option<CompanyResult> {
        let country = opts.country.as_deref().unwrap_or_else(|| {
            let countries = self.list_countries();
            countries[rng.gen_range(0..countries.len())].0
        }).to_uppercase();

        // Specific implementation
        if let Some((name, code)) = match country.as_str() {
            "GB" => Some(("VAT Number", self.generate_gb(rng))),
            "DE" => Some(("USt-IdNr", self.generate_de(rng))),
            "FR" => Some(("TVA Intracommunautaire", self.generate_fr(rng))),
            "IT" => Some(("Partita IVA", self.generate_it(rng))),
            "ES" => Some(("CIF", self.generate_es(rng))),
            _ => None,
        } {
            let country_name = match country.as_str() {
                "GB" => "United Kingdom",
                "DE" => "Germany",
                "FR" => "France",
                "IT" => "Italy",
                "ES" => "Spain",
                _ => "Unknown",
            };
            return Some(CompanyResult {
                country_code: country,
                country_name: country_name.to_string(),
                name: name.to_string(),
                code,
                valid: true,
            });
        }

        // Generic fallback
        if let Some(result) = generic::generate(&country, rng) {
            return Some(result);
        }

        None
    }

    pub fn validate(&self, country: &str, code: &str) -> bool {
        match country.to_uppercase().as_str() {
            "GB" => self.validate_gb(code),
            "DE" => self.validate_de(code),
            "FR" => self.validate_fr(code),
            "IT" => self.validate_it(code),
            "ES" => self.validate_es(code),
            _ => generic::validate(country, code),
        }
    }

    fn validate_gb(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 9 { return false; }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..7].iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
        let mut check = (sum % 97) as i32;
        check = 97 - check;
        if check < 0 { check += 97; }
        
        let actual_check: i32 = clean[7..9].parse().unwrap_or(-1);
        check == actual_check
    }

    fn validate_de(&self, code: &str) -> bool {
        let clean: String = if code.starts_with("DE") { code[2..].to_string() } else { code.to_string() };
        if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) { return false; }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        iso7064_mod11_10(&digits[..8]) == digits[8]
    }

    fn validate_fr(&self, code: &str) -> bool {
        let clean: String = if code.starts_with("FR") { code[2..].to_string() } else { code.to_string() };
        if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) { return false; }
        let siren_str = &clean[2..11];
        let siren_val: u64 = siren_str.parse().unwrap_or(0);
        let check = (12 + 3 * (siren_val % 97)) % 97;
        let actual_check: u64 = clean[0..2].parse().unwrap_or(99);
        check == actual_check
    }

    fn validate_it(&self, code: &str) -> bool {
        let clean: String = if code.starts_with("IT") { code[2..].to_string() } else { code.to_string() };
        if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) { return false; }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        luhn_check_digit(&digits[..10]) == digits[10]
    }

    fn validate_es(&self, code: &str) -> bool {
        if code.len() != 9 { return false; }
        let digits: Vec<u8> = code.bytes().skip(1).take(7).map(|b| b - b'0').collect();
        if digits.len() != 7 { return false; }
        
        let mut total: u32 = 0;
        for (i, &d) in digits.iter().enumerate() {
            let mut val = d as u32;
            if i % 2 == 0 {
                val *= 2;
                if val > 9 { val -= 9; }
            }
            total += val;
        }
        let check = (10 - (total % 10)) % 10;
        let actual_check = (code.as_bytes()[8] - b'0') as u32;
        check == actual_check
    }

    fn generate_gb(&self, rng: &mut impl Rng) -> String {
        let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
        let weights = [8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
        let mut check = (sum % 97) as i32;
        check = 97 - check;
        if check < 0 { check += 97; }
        
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        format!("GB{}{:02}", s, check)
    }

    fn generate_de(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        let check = iso7064_mod11_10(&digits);
        digits.push(check);
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        format!("DE{}", s)
    }

    fn generate_fr(&self, rng: &mut impl Rng) -> String {
        let mut siren_digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        let siren_check = luhn_check_digit(&siren_digits);
        siren_digits.push(siren_check);
        
        let siren_str: String = siren_digits.iter().map(|d| (b'0' + d) as char).collect();
        let siren_val: u64 = siren_str.parse().unwrap();
        let check = (12 + 3 * (siren_val % 97)) % 97;
        
        format!("FR{:02}{}", check, siren_str)
    }

    fn generate_it(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..=9)).collect();
        let check = luhn_check_digit(&digits);
        digits.push(check);
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        format!("IT{}", s)
    }

    fn generate_es(&self, rng: &mut impl Rng) -> String {
        let letters = b"ABCDEFGHJPQRSUVNW";
        let letter = letters[rng.gen_range(0..letters.len())] as char;
        let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
        
        // Simplified Spanish CIF checksum (Luhn-based for digits)
        let mut total: u32 = 0;
        for (i, &d) in digits.iter().enumerate() {
            let mut val = d as u32;
            if i % 2 == 0 {
                val *= 2;
                if val > 9 { val -= 9; }
            }
            total += val;
        }
        let check = (10 - (total % 10)) % 10;
        
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        // CIF can have a digit or a letter as check. Using digit for simplicity.
        format!("{}{}{}", letter, s, check)
    }

    pub fn list_countries(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let mut seen = std::collections::HashSet::new();
        let mut result = vec![
            ("GB", "United Kingdom", "VAT Number"),
            ("DE", "Germany", "USt-IdNr"),
            ("FR", "France", "TVA Intracommunautaire"),
            ("IT", "Italy", "Partita IVA"),
            ("ES", "Spain", "CIF"),
        ];
        for r in &result {
            seen.insert(r.0);
        }

        for item in generic::list_countries() {
            if seen.insert(item.0) {
                result.push(item);
            }
        }
        result.sort_by_key(|(code, _, _)| *code);
        result
    }
}
