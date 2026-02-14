use rand::Rng;
use super::CompanyResult;

struct GenericCompanyInfo {
    code: &'static str,
    country_name: &'static str,
    name: &'static str,
    prefix: &'static str,
    length: u8,
    numeric_only: bool,
}

static GENERIC_COMPANIES: &[GenericCompanyInfo] = &[
    // ── Europe ──
    GenericCompanyInfo { code: "AT", country_name: "Austria", name: "UID", prefix: "ATU", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "BE", country_name: "Belgium", name: "TVA", prefix: "BE0", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "CH", country_name: "Switzerland", name: "UID", prefix: "CHE", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "CZ", country_name: "Czech Republic", name: "DIC", prefix: "CZ", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "DK", country_name: "Denmark", name: "CVR", prefix: "DK", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "EE", country_name: "Estonia", name: "KMKR", prefix: "EE", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "FI", country_name: "Finland", name: "ALV nro", prefix: "FI", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "GR", country_name: "Greece", name: "AFM", prefix: "EL", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "HR", country_name: "Croatia", name: "OIB", prefix: "HR", length: 11, numeric_only: true },
    GenericCompanyInfo { code: "HU", country_name: "Hungary", name: "ANUM", prefix: "HU", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "IE", country_name: "Ireland", name: "VAT", prefix: "IE", length: 8, numeric_only: false },
    GenericCompanyInfo { code: "LT", country_name: "Lithuania", name: "PVM", prefix: "LT", length: 12, numeric_only: true },
    GenericCompanyInfo { code: "LU", country_name: "Luxembourg", name: "TVA", prefix: "LU", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "LV", country_name: "Latvia", name: "PVN", prefix: "LV", length: 11, numeric_only: true },
    GenericCompanyInfo { code: "MT", country_name: "Malta", name: "VAT", prefix: "MT", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "NL", country_name: "Netherlands", name: "BTW", prefix: "NL", length: 12, numeric_only: false },
    GenericCompanyInfo { code: "NO", country_name: "Norway", name: "MVA", prefix: "NO", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "PL", country_name: "Poland", name: "NIP", prefix: "PL", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "PT", country_name: "Portugal", name: "NIPC", prefix: "PT", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "RO", country_name: "Romania", name: "CIF", prefix: "RO", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "SE", country_name: "Sweden", name: "VAT", prefix: "SE", length: 12, numeric_only: true },
    GenericCompanyInfo { code: "SI", country_name: "Slovenia", name: "DDV", prefix: "SI", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "SK", country_name: "Slovakia", name: "DPH", prefix: "SK", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "TR", country_name: "Turkey", name: "VKN", prefix: "", length: 10, numeric_only: true },
    // ── Americas ──
    GenericCompanyInfo { code: "US", country_name: "United States", name: "EIN", prefix: "", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "CA", country_name: "Canada", name: "BN", prefix: "", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "BR", country_name: "Brazil", name: "CNPJ", prefix: "", length: 14, numeric_only: true },
    GenericCompanyInfo { code: "MX", country_name: "Mexico", name: "RFC", prefix: "", length: 12, numeric_only: false },
    GenericCompanyInfo { code: "AR", country_name: "Argentina", name: "CUIT", prefix: "", length: 11, numeric_only: true },
    GenericCompanyInfo { code: "CL", country_name: "Chile", name: "RUT", prefix: "", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "CO", country_name: "Colombia", name: "NIT", prefix: "", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "PE", country_name: "Peru", name: "RUC", prefix: "", length: 11, numeric_only: true },
    // ── Asia-Pacific ──
    GenericCompanyInfo { code: "AU", country_name: "Australia", name: "ABN", prefix: "", length: 11, numeric_only: true },
    GenericCompanyInfo { code: "NZ", country_name: "New Zealand", name: "NZBN", prefix: "", length: 13, numeric_only: true },
    GenericCompanyInfo { code: "CN", country_name: "China", name: "USCI", prefix: "", length: 18, numeric_only: false },
    GenericCompanyInfo { code: "IN", country_name: "India", name: "GSTIN", prefix: "29", length: 13, numeric_only: false },
    GenericCompanyInfo { code: "JP", country_name: "Japan", name: "Corporate Number", prefix: "", length: 13, numeric_only: true },
    GenericCompanyInfo { code: "KR", country_name: "South Korea", name: "BRN", prefix: "", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "SG", country_name: "Singapore", name: "UEN", prefix: "", length: 10, numeric_only: false },
    GenericCompanyInfo { code: "HK", country_name: "Hong Kong", name: "BR Number", prefix: "", length: 8, numeric_only: true },
    GenericCompanyInfo { code: "MY", country_name: "Malaysia", name: "Business Reg", prefix: "", length: 12, numeric_only: true },
    GenericCompanyInfo { code: "TH", country_name: "Thailand", name: "Tax ID", prefix: "", length: 13, numeric_only: true },
    GenericCompanyInfo { code: "VN", country_name: "Vietnam", name: "MST", prefix: "", length: 10, numeric_only: true },
    GenericCompanyInfo { code: "PH", country_name: "Philippines", name: "TIN", prefix: "", length: 12, numeric_only: true },
    GenericCompanyInfo { code: "ID", country_name: "Indonesia", name: "NPWP", prefix: "", length: 15, numeric_only: true },
    // ── Africa / Middle East ──
    GenericCompanyInfo { code: "ZA", country_name: "South Africa", name: "VAT", prefix: "4", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "NG", country_name: "Nigeria", name: "TIN", prefix: "", length: 12, numeric_only: true },
    GenericCompanyInfo { code: "EG", country_name: "Egypt", name: "Tax Card", prefix: "", length: 9, numeric_only: true },
    GenericCompanyInfo { code: "IL", country_name: "Israel", name: "Company Number", prefix: "51", length: 7, numeric_only: true },
    GenericCompanyInfo { code: "AE", country_name: "United Arab Emirates", name: "TRN", prefix: "100", length: 12, numeric_only: true },
    GenericCompanyInfo { code: "SA", country_name: "Saudi Arabia", name: "VAT", prefix: "3", length: 14, numeric_only: true },
];

fn find_info(code: &str) -> Option<&'static GenericCompanyInfo> {
    GENERIC_COMPANIES.iter().find(|c| c.code == code)
}

pub fn generate(code: &str, rng: &mut impl Rng) -> Option<CompanyResult> {
    let info = find_info(code).or_else(|| {
        // Absolute fallback for any unknown country code
        if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
            Some(&GenericCompanyInfo {
                code: "??", // doesn't matter, we'll use the passed code
                country_name: "Unknown",
                name: "Business ID",
                prefix: "",
                length: 10,
                numeric_only: false,
            })
        } else {
            None
        }
    })?;

    let suffix_len = info.length as usize;
    let suffix: String = if info.numeric_only {
        (0..suffix_len)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect()
    } else {
        (0..suffix_len)
            .map(|_| {
                if rng.gen_bool(0.6) {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                } else {
                    (b'A' + rng.gen_range(0..26u8)) as char
                }
            })
            .collect()
    };

    let full_code = format!("{}{}", info.prefix, suffix);

    Some(CompanyResult {
        country_code: code.to_string(),
        country_name: if info.code == "??" { format!("Country ({})", code) } else { info.country_name.to_string() },
        name: info.name.to_string(),
        code: full_code,
        valid: true,
    })
}

pub fn validate(code: &str, input: &str) -> bool {
    let info = match find_info(code) {
        Some(i) => i,
        None => {
            // Absolute fallback for unknown countries
            return input.len() >= 8 && input.len() <= 15;
        }
    };

    if !input.starts_with(info.prefix) {
        return false;
    }

    let suffix = &input[info.prefix.len()..];
    if suffix.len() != info.length as usize {
        return false;
    }

    if info.numeric_only {
        suffix.chars().all(|c| c.is_ascii_digit())
    } else {
        suffix.chars().all(|c| c.is_ascii_alphanumeric())
    }
}

pub fn list_countries() -> Vec<(&'static str, &'static str, &'static str)> {
    GENERIC_COMPANIES
        .iter()
        .map(|c| (c.code, c.country_name, c.name))
        .collect()
}
