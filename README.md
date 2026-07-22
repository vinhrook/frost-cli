# ❄️ frost-cli

Cold chain monitoring should be dead simple. If a warehouse operator or pharmacy technician needs to verify whether a batch of vaccines or milk is at a safe storage temperature, they shouldn't have to fight with rigid CLI flags like `--unit C --to F`.

`frost-cli` is a lightweight, zero-overhead Rust tool built for real-world temperature conversion and safety auditing across sensitive cargo (vaccines, blood products, dairy, insulin, and meat).

---

## ⚡ How it works

I built `frost-cli` with a dual-mode argument resolver. You can type commands naturally in plain English or use standard flags if you're scripting it inside a automated pipeline.

### 1. Conversational Temperature Conversion

Type commands naturally without memorizing flags:

```bash
# Convert 23°C to Fahrenheit
cargo run -- convert 23 C to F

# Quick shorthand
cargo run -- convert 23 C F

# Traditional flag style (also works)
cargo run -- convert 23 --unit C --to F
```

Output:
```text
23°C -> 73.4°F  Ambient
```

### 2. Cold Chain Safety Auditing

Instantly check if a cargo temperature complies with WHO / cold chain safety limits:

```bash
# Check if 4°C is safe for vaccine storage
cargo run -- check 4 C vaccine

# If unit is omitted, it defaults to Celsius
cargo run -- check 4 vaccine
```

Output:
```text
🛡️ Cold Chain Safety Check
==========================================
Product:     VACCINE
Input Temp:  4.0°C
Normalized:  4.0°C
Safe Range:  2.0°C to 8.0°C
------------------------------------------
Status:      ✅ SAFE (Within Cold Chain Limits)
Note:        Requires strict WHO cold chain standard (2°C - 8°C). Risk of loss of potency if breached.
==========================================
```

If a temperature breach occurs (e.g., checking 12°C for a vaccine):
```text
Status:      🔥 CRITICAL HIGH (TOO HOT / SPOILAGE RISK)
```

Supported product profiles out of the box: `vaccine`, `blood`, `insulin`, `milk`, `meat`, `chocolate`, `ice cream`.

### 3. Persistent History Audit Trail

Every conversion automatically appends to a local `history.json` file with local timestamps so you always have an auditable record of measurements.

```bash
cargo run -- history
```

---

## 🛠️ Under the hood

Built purely in Rust with a clean, light footprint:

- **`clap`** for CLI macro routing & flexible positional argument parsing
- **`serde` + `serde_json`** for local JSON persistence
- **`chrono`** for timestamping
- **`colored`** for terminal hazard highlighting

### Project Anatomy

```text
src/
├── main.rs       # Entry point & natural language argument resolver
├── cli.rs        # Clap enum schemas & subcommand definitions
├── converter.rs  # Math conversions (C / F / K) + thermal zone tagging
├── checker.rs    # Static product safety profiles & boundary checks
└── history.rs    # Serde JSON load & save handling
```
```

---

## 📜 License

MIT License. Free to use, modify, and distribute.
