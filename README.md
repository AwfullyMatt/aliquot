# Aliquot Sequence Classifier 🔢

[![CC0 License](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://creativecommons.org/publicdomain/zero/1.0/)

A multithreaded Rust program that calculates and classifies [Aliquot Sequences](https://en.wikipedia.org/wiki/Aliquot_sequence) with CSV output.

## Features ✨
- **Fast classification** using parallel processing
- **Resume capability** - continues where you left off
- **CSV output** with full sequence history

## Quick Start 🚀
```bash
git clone git@github.com:yourusername/aliquot-classifier.git
cd aliquot-classifier
cargo run --release
```

## How It Works 🔍
Results are saved to aliquot.csv in this format:

```csv
Number,Classification,Sequence
6,perfect,"6"
220,amicable,"220,284"
12,terminating,"12,16,15,9,4,3,1,0"
etc...
```

## Sequence Generation 🪄
For each number, the program:
- Calculates sum of proper divisors
- Builds sequence until it detects:
  - Repeating pattern (cycle)
  - Termination at zero
  - 1000-step limit

## Classification Types 📊
| Type	   | Pattern	                | Example               |
| :------: | :----------------------: | :-------------------: |
| Perfect	 | Immediate repetition	    | 6 → 6 → 6...          |
| Amicable | 2-number cycle	          | 220 ↔ 284 ↔ 220...    |
| Sociable | 3+ number cycle          | 1264460 → 1547860...  |
| Aspiring | Ends at perfect number   |	25 → 6 → 6...         |
| Term     | Reaches zero             | 12 → 16 → ... → 0     |
| Non-Term | No pattern in 1000 steps | 276 → ... (continues) |

## Configuration ⚙️

Modify in src/main.rs:

```rust
const RUN_QUANTITY: u64 = 5000; // Numbers processed per run
```

## License 📄

This project is dedicated to the public domain under CC0 1.0.
No attribution required - use freely for any purpose.

🔢 Number Theory | ⚡ Rust Performance | 📊 Data Export
