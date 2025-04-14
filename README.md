# Aliquot Sequence Classifier 🔢

[![CC0 License](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://creativecommons.org/publicdomain/zero/1.0/)

A multithreaded Rust program that calculates and classifies [Aliquot Sequences](https://en.wikipedia.org/wiki/Aliquot_sequence) with CSV output.

## Features ✨
- **Fast classification** using parallel processing
- **Resume capability** - continues where you left off
- **CSV output** with full sequence history
- **Six classifications**:
  - Perfect (6 → 6 → 6...)
  - Amicable (220 ↔ 284 ↔ 220...)
  - Sociable (long cycles)
  - Aspiring (ends in perfect number)
  - Terminating (reaches zero)
  - Non-terminating (no pattern detected)

## Quick Start 🚀
```bash
git clone git@github.com:yourusername/aliquot-classifier.git
cd aliquot-classifier
cargo run --release
