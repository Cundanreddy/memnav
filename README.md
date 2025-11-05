# memnav

A Rust crate for analyzing `.map` files and reporting per-symbol, per-function, and per-section memory usage.

### Usage (CLI)
```bash
memnav analyze app.map --csv report.csv --json report.json
```

# Analyze and export both CSV + JSON
cargo run -- analyze myproject.map --csv report.csv --json report.json

# Install globally after publishing
cargo install memnav

# Use globally
memnav analyze app.map --csv out.csv
