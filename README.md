# radiation-sim

A terminal CLI that simulates blackbody thermal radiation using the **Stefan–Boltzmann law**. The app computes:
- **Radiated power**: $P = \sigma\,\varepsilon\,A\,T^{4}$ $[\mathrm{W}]$
- **Energy emitted over time**: $E = P \cdot t$ $[\mathrm{J}]$

> $\sigma = 5.670\,374\,419\times 10^{-8}\ \mathrm{W\,m^{-2}\,K^{-4}}$ — Stefan–Boltzmann constant (SI).  
> $\varepsilon$ — emissivity in $[0,1]$: 1 for a perfect blackbody; lower for real materials.

---

## Table of Contents
- [Requirements](#requirements)
- [Install & Run](#install--run)
- [CLI Usage](#cli-usage)
- [Examples](#examples)
- [Output: JSON vs Pretty](#output-json-vs-pretty)
- [Validation & Units](#validation--units)
- [What the App Computes (Step by Step)](#what-the-app-computes-step-by-step)
- [Project Structure](#project-structure)
- [Tests & Benchmarks](#tests--benchmarks)
- [Limitations & Roadmap](#limitations--roadmap)
- [FAQ](#faq)
- [License](#license)

---

## Requirements
- **Rust** 1.70+ (edition 2021)
- OS: Linux / macOS / Windows (x86_64)

---

## Install & Run

Clone the repo and build:

```bash
cargo build
```

Run in dev mode:

```bash
cargo run -- -T 1000 -A 1.0 -t 5
```

Release build + run the binary:

```bash
cargo build --release
# Linux/macOS:
./target/release/radiation-sim -T 800 -A 0.5 -t 10 -e 0.9
# Windows (PowerShell / CMD):
.\target\release\radiation-sim.exe -T 800 -A 0.5 -t 10 -e 0.9
```

---

## CLI Usage

```text
Usage: radiation-sim [OPTIONS] --temp <K> --area <m2> --time <s>

Options:
  -T, --temp <K>          Temperature in Kelvin (K) > 0
  -A, --area <m2>         Surface area in m² ≥ 0
  -t, --time <s>          Time in seconds ≥ 0
  -e, --emis <ε>          Emissivity in [0,1] (default: 1.0)
  -p, --precision <n>     Decimal places (0..=10, default: 3)
      --output <format>   Output format: pretty | json (default: pretty)
  -h, --help              Show help
  -V, --version           Show version
```

Quick help:
```bash
cargo run -- --help
```

---

## Examples

1) **Basic case** (pretty print):
```bash
cargo run -- -T 800 -A 0.5 -t 10 -e 0.9 -p 4
```
Sample output:
```
=== Stefan–Boltzmann Radiation ===
Input:
  T [K]:      800.0000
  A [m²]:     0.5000
  ε [-]:      0.9000
  t [s]:      10.0000
Results:
  P [W]:      10451.6341
  E [J]:      104516.3413
```

2) **JSON output** (easy for scripting):
```bash
cargo run -- -T 800 -A 0.5 -t 10 -e 0.9 --output json
```
Result:
```json
{
  "power_w": 10451.6341291008,
  "energy_j": 104516.341291008
}
```

3) **Astrophysics example** (treating a star as a blackbody):
```bash
cargo run --example star
```

---

## Output: JSON vs Pretty

- `--output pretty` — human-friendly text (default).
- `--output json` — stable structure for automation/CI.

Decimal precision in `pretty` mode is controlled by `-p/--precision`.  
In `json` we print the full `f64` to avoid precision loss downstream.

---

## Validation & Units

- **Temperature**: $[\mathrm{K}] > 0$ — *always in Kelvin!*  
  Convert from °C: $T[\mathrm{K}] = T[^{\circ}\mathrm{C}] + 273.15$
- **Area**: $[\mathrm{m}^2] \ge 0$
- **Time**: $[\mathrm{s}] \ge 0$
- **Emissivity**: $\varepsilon \in [0,1]$

Output units:
- **Power** $P$: watts $[\mathrm{W}]$
- **Energy** $E$: joules $[\mathrm{J}]$ (remember: $1\,\mathrm{Wh} = 3600\,\mathrm{J}$)

If inputs are out of range, the app prints a clear error and aborts.

---

## What the App Computes (Step by Step)

Given $T$, $A$, $\varepsilon$, and $t$:

1. **Power density** per $1\,\mathrm{m^2}$: $\sigma\,T^{4}$
2. **Material effect**: $\sigma\,\varepsilon\,T^{4}$
3. **Total power** for area $A$:  
   $$ P = \sigma\,\varepsilon\,A\,T^{4} $$
4. **Energy over time** $t$:  
   $$ E = P \cdot t $$

Implications:
- $P$ scales as **$T^{4}$** (e.g., doubling $T$ ⇒ $\sim 16\times$ larger $P$)
- $P$ is **linear** in $A$ and $\varepsilon$
- $E$ is **linear** in $t$ if $T$ (and thus $P$) is constant

---

## Project Structure

```
radiation-sim/
├─ Cargo.toml
├─ README.md
├─ .gitignore
├─ src/
│  ├─ main.rs        # CLI entry: parse -> validate -> sim -> report
│  ├─ lib.rs         # re-exports for tests/examples/benches
│  ├─ cli.rs         # Clap arguments
│  ├─ physics.rs     # σ constant, formulas: P=σ ε A T^4, E=P·t
│  ├─ sim.rs         # wiring computations; returns result
│  ├─ report.rs      # pretty text & JSON printers
│  └─ validate.rs    # input validation & normalization
├─ tests/
│  ├─ smoke_cli.rs   # CLI smoke tests
│  └─ physics_spec.rs# formula tests (against SIGMA)
├─ examples/
│  └─ star.rs        # star luminosity example
└─ benches/
   └─ perf.rs        # micro-benchmarks (criterion)
```

---

## Tests & Benchmarks

Run all tests:
```bash
cargo test
```

Benchmarks (require `criterion`):
```bash
cargo bench
```

Quick sanity check (the $T^{4}$ scaling):
```bash
cargo run -- -T 800  -A 0.5 -t 10 -e 0.9
cargo run -- -T 1600 -A 0.5 -t 10 -e 0.9  # power ≈ 16× larger
```

---

## Limitations & Roadmap

The current model assumes **constant temperature** during the calculation (no cooling/heating dynamics). This deliberate simplification means:
- instant computations,
- energy over time is simply $E = P \cdot t$.

**Potential extensions:**
- `--sweep` (scan over $T$ and/or $A$; CSV export)
- `--explain` (show intermediate math steps)
- basic cooling model:  
  $$ m c \frac{dT}{dt} = -\sigma\,\varepsilon\,A\,T^{4} $$
- more output formats (CSV, YAML)
- material presets (typical $\varepsilon$ values)
- unit converters (e.g., °C → K)

---

## FAQ

**Can I enter temperature in °C?**  
Not directly. Convert to Kelvin: $T[\mathrm{K}] = T[^{\circ}\mathrm{C}] + 273.15$.

**Results look too big/small — what to check?**  
Units first: $T$ in **K**, $A$ in **m²**, $t$ in **s**, $\varepsilon \in [0,1]$.  
Remember: $P \sim T^{4}$ — small $T$ changes have large effects.

**Do you account for surroundings/absorption?**  
No. Simple model: isolated body radiating to space.

**Why does JSON print full `f64`?**  
To avoid precision loss in downstream processing. In `pretty` mode use `-p` to control decimals.

---

## License

Dual-licensed: **MIT** OR **Apache-2.0**.  
Choose either license to suit your needs.
