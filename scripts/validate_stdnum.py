import sys
import json
from stdnum import br, mx, us, ar, cl, co, ec, ee, fi, it, za, fr, gb, de, es, nz, bic
# in is a keyword in python, so stdnum uses in_
from stdnum import in_ as in_module

def validate(item, category):
    code = item.get('raw') or item.get('code') or item.get('number') or item.get('iban') or item.get('account_number')
    cc = item.get('country_code') or item.get('country')
    
    # Clean code for stdnum (some modules need it)
    clean_code = code.replace(' ', '').replace('-', '').replace('.', '') if code else None
    
    try:
        if category == 'id':
            if cc == 'BR': from stdnum.br import cpf; return cpf.is_valid(clean_code)
            if cc == 'IN': from stdnum.in_ import aadhaar; return aadhaar.is_valid(clean_code)
            if cc == 'MX': from stdnum.mx import curp; return curp.is_valid(clean_code)
            if cc == 'US': from stdnum.us import ssn; return ssn.is_valid(clean_code)
            if cc == 'AR': from stdnum.ar import cui; return cui.is_valid(clean_code)
            if cc == 'EE': from stdnum.ee import ik; return ik.is_valid(clean_code)
            if cc == 'FI': from stdnum.fi import hetu; return hetu.is_valid(clean_code)
            if cc == 'IT': from stdnum.it import codicefiscale; return codicefiscale.is_valid(clean_code)
            if cc == 'ZA': from stdnum.za import id as za_id; return za_id.is_valid(clean_code)
            if cc == 'FR': from stdnum.fr import nir; return nir.is_valid(clean_code)
        
        elif category == 'company':
            if cc == 'DE': from stdnum.de import vat; return vat.is_valid(clean_code)
            if cc == 'GB': from stdnum.gb import vat; return vat.is_valid(clean_code)
            if cc == 'FR': from stdnum.fr import tva; return tva.is_valid(clean_code)
            if cc == 'IT': from stdnum.it import iva; return iva.is_valid(clean_code)
            if cc == 'ES': from stdnum.es import nif; return nif.is_valid(clean_code)
            if cc == 'US': from stdnum.us import ein; return ein.is_valid(clean_code)
            
        elif category == 'swift':
            from stdnum import bic; return bic.is_valid(clean_code)
            
        elif category == 'account':
            if cc == 'AR': from stdnum.ar import cbu; return cbu.is_valid(clean_code)
            if cc == 'NZ': from stdnum.nz import bankaccount; return bankaccount.is_valid(clean_code)
            
    except Exception as e:
        return f"Error: {e}"
    
    return "Skipped (no stdnum check)"

if __name__ == "__main__":
    category = sys.argv[1]
    data = json.load(sys.stdin)
    results = []
    for item in data:
        valid = validate(item, category)
        results.append({
            "code": item.get('code') or item.get('number') or item.get('iban') or item.get('account_number'),
            "country": item.get('country_code') or item.get('country'),
            "stdnum_valid": valid
        })
    print(json.dumps(results, indent=2))
