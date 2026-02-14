import time
from stdnum import iban

IBAN = "DE47508562162522867909"
ITERATIONS = 100_000

start = time.time()
for _ in range(ITERATIONS):
    iban.is_valid(IBAN)
end = time.time()

print(f"python-stdnum: {ITERATIONS} iterations in {end - start:.4f} seconds")
print(f"Throughput: {ITERATIONS / (end - start):.2f} ops/sec")
