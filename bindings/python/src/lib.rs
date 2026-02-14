#![allow(clippy::useless_conversion)]

use pyo3::conversion::ToPyObject;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rand::thread_rng;

fn account_result_to_dict(py: Python<'_>, r: &idsmith::bank_account::AccountResult) -> PyObject {
    let dict = PyDict::new_bound(py);
    dict.set_item("country_code", &r.country_code).unwrap();
    dict.set_item("country_name", &r.country_name).unwrap();
    dict.set_item("format_name", &r.format_name).unwrap();
    dict.set_item("bank_code", &r.bank_code).unwrap();
    dict.set_item("branch_code", &r.branch_code).unwrap();
    dict.set_item("account_number", &r.account_number).unwrap();
    dict.set_item("check_digits", &r.check_digits).unwrap();
    dict.set_item("formatted", &r.formatted).unwrap();
    dict.set_item("raw", &r.raw).unwrap();
    dict.set_item("iban", &r.iban).unwrap();
    dict.set_item("valid", r.valid).unwrap();
    dict.into()
}

fn id_result_to_dict(py: Python<'_>, r: &idsmith::personal_id::IdResult) -> PyObject {
    let dict = PyDict::new_bound(py);
    dict.set_item("country_code", &r.country_code).unwrap();
    dict.set_item("code", &r.code).unwrap();
    dict.set_item("gender", &r.gender).unwrap();
    dict.set_item("dob", &r.dob).unwrap();
    dict.set_item("valid", r.valid).unwrap();
    dict.into()
}

fn card_result_to_dict(py: Python<'_>, r: &idsmith::credit_card::CardResult) -> PyObject {
    let dict = PyDict::new_bound(py);
    dict.set_item("brand", &r.brand).unwrap();
    dict.set_item("number", &r.number).unwrap();
    dict.set_item("formatted", &r.formatted).unwrap();
    dict.set_item("valid", r.valid).unwrap();
    dict.into()
}

fn company_result_to_dict(py: Python<'_>, r: &idsmith::company_id::CompanyResult) -> PyObject {
    let dict = PyDict::new_bound(py);
    dict.set_item("country_code", &r.country_code).unwrap();
    dict.set_item("country_name", &r.country_name).unwrap();
    dict.set_item("name", &r.name).unwrap();
    dict.set_item("code", &r.code).unwrap();
    dict.set_item("valid", r.valid).unwrap();
    dict.into()
}

fn swift_result_to_dict(py: Python<'_>, r: &idsmith::swift::SwiftResult) -> PyObject {
    let dict = PyDict::new_bound(py);
    dict.set_item("code", &r.code).unwrap();
    dict.set_item("bank", &r.bank).unwrap();
    dict.set_item("country", &r.country).unwrap();
    dict.set_item("location", &r.location).unwrap();
    dict.set_item("branch", &r.branch).unwrap();
    dict.set_item("valid", r.valid).unwrap();
    dict.into()
}

// ── BankAccount ──

#[pyclass]
struct BankAccount;

#[pymethods]
impl BankAccount {
    #[staticmethod]
    #[pyo3(signature = (country, bank_code=None))]
    fn generate(py: Python<'_>, country: &str, bank_code: Option<String>) -> PyResult<PyObject> {
        let mut rng = thread_rng();
        let opts = idsmith::bank_account::GenOptions { bank_code };
        idsmith::bank_accounts()
            .generate(country, &opts, &mut rng)
            .map(|r| account_result_to_dict(py, &r))
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!("Unsupported country: {}", country))
            })
    }

    #[staticmethod]
    fn validate(country: &str, raw: &str) -> PyResult<bool> {
        idsmith::bank_accounts()
            .validate(country, raw)
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!("Unsupported country: {}", country))
            })
    }

    #[staticmethod]
    fn format(country: &str, raw: &str) -> PyResult<String> {
        idsmith::bank_accounts()
            .format(country, raw)
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!("Unsupported country: {}", country))
            })
    }

    #[staticmethod]
    fn list_countries(py: Python<'_>) -> PyResult<PyObject> {
        let countries: Vec<PyObject> = idsmith::bank_accounts()
            .list_countries()
            .iter()
            .map(|(code, name, format_name, has_iban)| {
                let dict = PyDict::new_bound(py);
                dict.set_item("code", code).unwrap();
                dict.set_item("name", name).unwrap();
                dict.set_item("format", format_name).unwrap();
                dict.set_item("has_iban", has_iban).unwrap();
                dict.into()
            })
            .collect();
        Ok(countries.to_object(py))
    }

    #[staticmethod]
    fn is_supported(country: &str) -> bool {
        idsmith::bank_accounts().is_supported(country)
    }
}

// ── PersonalId ──

#[pyclass]
struct PersonalId;

#[pymethods]
impl PersonalId {
    #[staticmethod]
    #[pyo3(signature = (country, gender=None, year=None))]
    fn generate(country: &str, gender: Option<&str>, year: Option<u16>) -> PyResult<String> {
        let mut rng = thread_rng();
        let g = idsmith::personal_id::date::Gender::from_str_opt(gender);
        let opts = idsmith::personal_id::GenOptions { gender: g, year };
        idsmith::personal_ids()
            .generate(country, &opts, &mut rng)
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!("Unsupported country: {}", country))
            })
    }

    #[staticmethod]
    fn validate(country: &str, code: &str) -> PyResult<bool> {
        idsmith::personal_ids()
            .validate(country, code)
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!("Unsupported country: {}", country))
            })
    }

    #[staticmethod]
    fn parse(py: Python<'_>, country: &str, code: &str) -> PyResult<PyObject> {
        idsmith::personal_ids()
            .parse(country, code)
            .map(|r| id_result_to_dict(py, &r))
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!("Unsupported country: {}", country))
            })
    }

    #[staticmethod]
    fn list_countries(py: Python<'_>) -> PyResult<PyObject> {
        let countries: Vec<PyObject> = idsmith::personal_ids()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| {
                let dict = PyDict::new_bound(py);
                dict.set_item("code", code).unwrap();
                dict.set_item("country_name", country_name).unwrap();
                dict.set_item("id_name", id_name).unwrap();
                dict.into()
            })
            .collect();
        Ok(countries.to_object(py))
    }

    #[staticmethod]
    fn is_supported(country: &str) -> bool {
        idsmith::personal_ids().is_supported(country)
    }
}

// ── CreditCard ──

#[pyclass]
struct CreditCard;

#[pymethods]
impl CreditCard {
    #[staticmethod]
    #[pyo3(signature = (brand=None))]
    fn generate(py: Python<'_>, brand: Option<String>) -> PyResult<PyObject> {
        let mut rng = thread_rng();
        let opts = idsmith::credit_card::GenOptions { brand };
        idsmith::credit_cards()
            .generate(&opts, &mut rng)
            .map(|r| card_result_to_dict(py, &r))
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err("Failed to generate credit card")
            })
    }

    #[staticmethod]
    fn validate(number: &str) -> bool {
        idsmith::credit_cards().validate(number)
    }

    #[staticmethod]
    fn format(brand: &str, number: &str) -> String {
        idsmith::credit_cards().format(brand, number)
    }

    #[staticmethod]
    fn list_brands() -> Vec<&'static str> {
        idsmith::credit_cards().list_brands()
    }
}

// ── CompanyId ──

#[pyclass]
struct CompanyId;

#[pymethods]
impl CompanyId {
    #[staticmethod]
    #[pyo3(signature = (country=None))]
    fn generate(py: Python<'_>, country: Option<String>) -> PyResult<PyObject> {
        let mut rng = thread_rng();
        let opts = idsmith::company_id::GenOptions { country };
        idsmith::company_ids()
            .generate(&opts, &mut rng)
            .map(|r| company_result_to_dict(py, &r))
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Failed to generate company ID"))
    }

    #[staticmethod]
    fn validate(country: &str, code: &str) -> bool {
        idsmith::company_ids().validate(country, code)
    }

    #[staticmethod]
    fn list_countries(py: Python<'_>) -> PyResult<PyObject> {
        let countries: Vec<PyObject> = idsmith::company_ids()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| {
                let dict = PyDict::new_bound(py);
                dict.set_item("code", code).unwrap();
                dict.set_item("country_name", country_name).unwrap();
                dict.set_item("id_name", id_name).unwrap();
                dict.into()
            })
            .collect();
        Ok(countries.to_object(py))
    }
}

// ── Swift ──

#[pyclass]
struct Swift;

#[pymethods]
impl Swift {
    #[staticmethod]
    #[pyo3(signature = (country=None))]
    fn generate(py: Python<'_>, country: Option<String>) -> PyObject {
        let mut rng = thread_rng();
        let opts = idsmith::swift::GenOptions { country };
        let r = idsmith::swift_codes().generate(&opts, &mut rng);
        swift_result_to_dict(py, &r)
    }

    #[staticmethod]
    fn validate(code: &str) -> bool {
        idsmith::swift_codes().validate(code)
    }
}

// ── IBAN functions ──

#[pyfunction]
#[pyo3(signature = (country=None))]
fn generate_iban(country: Option<&str>) -> PyResult<String> {
    let mut rng = thread_rng();
    idsmith::iban::generate_iban(country, &mut rng).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn validate_iban(iban: &str) -> bool {
    idsmith::iban::validate_iban(iban)
}

#[pyfunction]
fn format_iban(iban: &str) -> String {
    idsmith::iban::format_iban(iban)
}

#[pyfunction]
fn iban_countries() -> Vec<&'static str> {
    idsmith::iban::supported_countries()
}

// ── Module ──

#[pymodule]
fn _idsmith(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BankAccount>()?;
    m.add_class::<PersonalId>()?;
    m.add_class::<CreditCard>()?;
    m.add_class::<CompanyId>()?;
    m.add_class::<Swift>()?;
    m.add_function(wrap_pyfunction!(generate_iban, m)?)?;
    m.add_function(wrap_pyfunction!(validate_iban, m)?)?;
    m.add_function(wrap_pyfunction!(format_iban, m)?)?;
    m.add_function(wrap_pyfunction!(iban_countries, m)?)?;
    Ok(())
}
