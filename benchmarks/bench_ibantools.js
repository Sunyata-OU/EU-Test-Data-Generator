const { isValidIBAN } = require('ibantools');

const IBAN = "DE47508562162522867909";
const ITERATIONS = 100_000;

const start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
    isValidIBAN(IBAN);
}
const end = Date.now();

const durationSeconds = (end - start) / 1000;
console.log(`ibantools: ${ITERATIONS} iterations in ${durationSeconds.toFixed(4)} seconds`);
console.log(`Throughput: ${(ITERATIONS / durationSeconds).toFixed(2)} ops/sec`);
