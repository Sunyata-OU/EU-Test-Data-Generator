import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import {
  BankAccount,
  PersonalId,
  CreditCard,
  CompanyId,
  Swift,
  generateIban,
  validateIban,
  formatIban,
  ibanCountries,
} from '../index.js';

describe('IBAN', () => {
  it('should generate a valid German IBAN', () => {
    const iban = generateIban('DE');
    assert.ok(iban.startsWith('DE'));
    assert.strictEqual(iban.length, 22);
    assert.ok(validateIban(iban));
  });

  it('should reject invalid IBAN', () => {
    assert.ok(!validateIban('DE00000000000000000000'));
  });

  it('should format IBAN with spaces', () => {
    const iban = generateIban('GB');
    const formatted = formatIban(iban);
    assert.ok(formatted.includes(' '));
  });

  it('should list supported countries', () => {
    const countries = ibanCountries();
    assert.ok(countries.includes('DE'));
    assert.ok(countries.includes('GB'));
    assert.ok(countries.length > 50);
  });
});

describe('BankAccount', () => {
  it('should generate a US bank account', () => {
    const result = BankAccount.generate('US');
    assert.strictEqual(result.countryCode, 'US');
    assert.ok(result.valid);
    assert.ok(result.formatted);
  });

  it('should validate a generated account', () => {
    const result = BankAccount.generate('US');
    assert.ok(BankAccount.validate('US', result.raw));
  });

  it('should list countries', () => {
    const countries = BankAccount.listCountries();
    assert.ok(countries.length > 100);
    const codes = countries.map((c) => c.code);
    assert.ok(codes.includes('US'));
  });

  it('should check support', () => {
    assert.ok(BankAccount.isSupported('US'));
    assert.ok(BankAccount.isSupported('DE'));
  });
});

describe('PersonalId', () => {
  it('should generate a valid Estonian ID', () => {
    const code = PersonalId.generate('EE');
    assert.strictEqual(code.length, 11);
    assert.ok(PersonalId.validate('EE', code));
  });

  it('should generate with gender', () => {
    const code = PersonalId.generate('EE', 'male');
    assert.ok(PersonalId.validate('EE', code));
  });

  it('should parse a generated ID', () => {
    const code = PersonalId.generate('EE');
    const result = PersonalId.parse('EE', code);
    assert.strictEqual(result.countryCode, 'EE');
    assert.ok(result.valid);
    assert.ok(result.dob);
  });

  it('should list countries', () => {
    const countries = PersonalId.listCountries();
    assert.ok(countries.length > 40);
    const codes = countries.map((c) => c.code);
    assert.ok(codes.includes('US'));
    assert.ok(codes.includes('EE'));
  });
});

describe('CreditCard', () => {
  it('should generate a valid card', () => {
    const result = CreditCard.generate();
    assert.ok(result.valid);
    assert.ok(result.number);
    assert.ok(result.brand);
  });

  it('should generate a Visa card', () => {
    const result = CreditCard.generate('visa');
    assert.strictEqual(result.brand, 'VISA');
    assert.ok(result.number.startsWith('4'));
  });

  it('should validate a generated card', () => {
    const result = CreditCard.generate();
    assert.ok(CreditCard.validate(result.number));
  });

  it('should reject invalid card', () => {
    assert.ok(!CreditCard.validate('0000000000000000'));
  });

  it('should list brands', () => {
    const brands = CreditCard.listBrands();
    assert.ok(brands.includes('Visa'));
    assert.ok(brands.includes('Mastercard'));
    assert.strictEqual(brands.length, 6);
  });
});

describe('CompanyId', () => {
  it('should generate a UK company ID', () => {
    const result = CompanyId.generate('GB');
    assert.strictEqual(result.countryCode, 'GB');
    assert.ok(result.valid);
  });

  it('should validate a generated ID', () => {
    const result = CompanyId.generate('GB');
    assert.ok(CompanyId.validate('GB', result.code));
  });

  it('should list countries', () => {
    const countries = CompanyId.listCountries();
    assert.ok(countries.length > 50);
    const codes = countries.map((c) => c.code);
    assert.ok(codes.includes('GB'));
  });
});

describe('Swift', () => {
  it('should generate a SWIFT code', () => {
    const result = Swift.generate();
    assert.ok(result.code);
    assert.ok(result.valid);
  });

  it('should generate for a specific country', () => {
    const result = Swift.generate('US');
    assert.ok(result.code.includes('US'));
  });

  it('should validate a generated code', () => {
    const result = Swift.generate();
    assert.ok(Swift.validate(result.code));
  });
});
