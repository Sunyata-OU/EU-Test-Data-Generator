import time
import subprocess

ITERATIONS = 50
IBAN = "DE47508562162522867909"

def bench_cli(name, command_args):
    start = time.time()
    for _ in range(ITERATIONS):
        subprocess.run(command_args, capture_output=True)
    end = time.time()
    avg = (end - start) / ITERATIONS
    print(f"{name} (CLI): {avg:.4f}s avg per call")

print(f"Benchmarking CLI startup over {ITERATIONS} calls...")
bench_cli("idsmith (Rust)", ["./target/release/idsmith", "validate", "iban", IBAN])
# We don't have a direct CLI for ibantools/stdnum in a standard way easily, 
# but we can use a wrapper script for stdnum to simulate its CLI startup.
bench_cli("python-stdnum (via wrapper)", ["./benchmarks/venv/bin/python3", "-c", "import sys; from stdnum import iban; iban.is_valid(sys.argv[1])", IBAN])
