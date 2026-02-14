const ibantools = require('ibantools');
const abavalidator = require('abavalidator');
const clabeValidator = require('clabe-validator');
const taiwanId = require('taiwan-id');

let input = '';
process.stdin.on('data', chunk => { input += chunk; });
process.stdin.on('end', () => {
    if (!input) return;
    const data = JSON.parse(input);
    const category = process.argv[2];
    const results = data.map(item => {
        let code = item.raw || item.code || item.number || item.iban || item.account_number;
        let cc = item.country_code || item.country;
        let valid = false;

        if (category === 'iban') {
            valid = ibantools.isValidIBAN(code);
        } else if (category === 'account') {
            if (cc === 'US') {
                valid = abavalidator.validate(item.bank_code);
            } else if (cc === 'MX') {
                valid = clabeValidator.clabe.validate(code).ok;
            }
        } else if (category === 'id' && cc === 'TW') {
            valid = taiwanId.check(code);
        }

        return { code, country: cc, npm_valid: valid };
    });
    console.log(JSON.stringify(results, null, 2));
});