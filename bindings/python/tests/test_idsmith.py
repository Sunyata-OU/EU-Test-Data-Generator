import idsmith


def test_generate_iban():
    iban = idsmith.generate_iban("DE")
    assert iban.startswith("DE")
    assert len(iban) == 22
    assert idsmith.validate_iban(iban)


def test_validate_iban_invalid():
    assert not idsmith.validate_iban("DE00000000000000000000")


def test_format_iban():
    iban = idsmith.generate_iban("GB")
    formatted = idsmith.format_iban(iban)
    assert " " in formatted


def test_iban_countries():
    countries = idsmith.iban_countries()
    assert "DE" in countries
    assert "GB" in countries
    assert len(countries) > 50


def test_bank_account_generate():
    result = idsmith.BankAccount.generate("US")
    assert result["country_code"] == "US"
    assert result["valid"] is True
    assert result["formatted"]


def test_bank_account_validate():
    result = idsmith.BankAccount.generate("US")
    assert idsmith.BankAccount.validate("US", result["raw"])


def test_bank_account_list_countries():
    countries = idsmith.BankAccount.list_countries()
    assert len(countries) > 100
    codes = [c["code"] for c in countries]
    assert "US" in codes


def test_bank_account_is_supported():
    assert idsmith.BankAccount.is_supported("US")
    assert idsmith.BankAccount.is_supported("DE")


def test_personal_id_generate():
    code = idsmith.PersonalId.generate("EE")
    assert len(code) == 11
    assert idsmith.PersonalId.validate("EE", code)


def test_personal_id_generate_with_gender():
    code = idsmith.PersonalId.generate("EE", gender="male")
    assert idsmith.PersonalId.validate("EE", code)


def test_personal_id_parse():
    code = idsmith.PersonalId.generate("EE")
    result = idsmith.PersonalId.parse("EE", code)
    assert result["country_code"] == "EE"
    assert result["valid"] is True
    assert result["dob"] is not None


def test_personal_id_list_countries():
    countries = idsmith.PersonalId.list_countries()
    assert len(countries) > 40
    codes = [c["code"] for c in countries]
    assert "US" in codes
    assert "EE" in codes


def test_credit_card_generate():
    result = idsmith.CreditCard.generate()
    assert result["valid"] is True
    assert result["number"]
    assert result["brand"]
    assert result["cvv"]
    assert len(result["cvv"]) in (3, 4)
    assert result["expiry"]
    assert "/" in result["expiry"]


def test_credit_card_generate_visa():
    result = idsmith.CreditCard.generate(brand="visa")
    assert result["brand"] == "VISA"
    assert result["number"].startswith("4")


def test_credit_card_validate():
    result = idsmith.CreditCard.generate()
    assert idsmith.CreditCard.validate(result["number"])


def test_credit_card_validate_invalid():
    assert not idsmith.CreditCard.validate("1234567890123456")


def test_credit_card_list_brands():
    brands = idsmith.CreditCard.list_brands()
    assert "Visa" in brands
    assert "Mastercard" in brands
    assert len(brands) == 6


def test_company_id_generate():
    result = idsmith.CompanyId.generate(country="GB")
    assert result["country_code"] == "GB"
    assert result["valid"] is True


def test_company_id_validate():
    result = idsmith.CompanyId.generate(country="GB")
    assert idsmith.CompanyId.validate("GB", result["code"])


def test_company_id_list_countries():
    countries = idsmith.CompanyId.list_countries()
    assert len(countries) > 50
    codes = [c["code"] for c in countries]
    assert "GB" in codes


def test_swift_generate():
    result = idsmith.Swift.generate()
    assert result["code"]
    assert result["valid"] is True


def test_swift_generate_country():
    result = idsmith.Swift.generate(country="US")
    assert "US" in result["code"]


def test_swift_validate():
    result = idsmith.Swift.generate()
    assert idsmith.Swift.validate(result["code"])
