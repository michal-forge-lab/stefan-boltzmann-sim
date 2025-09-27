# radiation-sim

Terminalowy symulator promieniowania ciała czarnego zgodnie z **prawem Stefana–Boltzmanna**. Aplikacja CLI oblicza:
- **moc promieniowania** $P = \sigma\,\varepsilon\,A\,T^{4}$ $[\mathrm{W}]$,
- **energię wypromieniowaną** w zadanym czasie $E = P \cdot t$ $[\mathrm{J}]$.

> $\sigma = 5.670\,374\,419\times 10^{-8}\ \mathrm{W\,m^{-2}\,K^{-4}}$ — stała Stefana–Boltzmanna (SI).  
> $\varepsilon$ — emisyjność w zakresie $[0,1]$: 1 dla ciała doskonale czarnego, mniej dla materiałów rzeczywistych.

---

## Spis treści
- [Wymagania](#wymagania)
- [Instalacja i uruchomienie](#instalacja-i-uruchomienie)
- [Użycie (CLI)](#użycie-cli)
- [Przykłady](#przykłady)
- [Wyjście JSON vs. czytelny tekst](#wyjście-json-vs-czytelny-tekst)
- [Walidacja i jednostki](#walidacja-i-jednostki)
- [Co liczy aplikacja — w prostych krokach](#co-liczy-aplikacja--w-prostych-krokach)
- [Struktura projektu](#struktura-projektu)
- [Testy i benchmarki](#testy-i-benchmarki)
- [Ograniczenia modelu i roadmapa](#ograniczenia-modelu-i-roadmapa)
- [FAQ](#faq)
- [Licencja](#licencja)

---

## Wymagania
- **Rust** 1.70+ (edition 2021)
- System: Linux / macOS / Windows (x86_64)

---

## Instalacja i uruchomienie

Sklonuj repo i zbuduj:

```bash
cargo build
```

Uruchom w trybie deweloperskim:

```bash
cargo run -- -T 1000 -A 1.0 -t 5
```

Build wydaniowy + uruchomienie binarki:

```bash
cargo build --release
# Linux/macOS:
./target/release/radiation-sim -T 800 -A 0.5 -t 10 -e 0.9
# Windows (PowerShell / CMD):
.\target\release\radiation-sim.exe -T 800 -A 0.5 -t 10 -e 0.9
```

---

## Użycie (CLI)

```text
Usage: radiation-sim [OPTIONS] --temp <K> --area <m2> --time <s>

Options:
  -T, --temp <K>          Temperatura w Kelvinach (K) > 0
  -A, --area <m2>         Powierzchnia w m² ≥ 0
  -t, --time <s>          Czas w sekundach ≥ 0
  -e, --emis <ε>          Emisyjność w [0,1] (domyślnie: 1.0)
  -p, --precision <n>     Miejsca po przecinku (0..=10, domyślnie: 3)
      --output <format>  Format wyjścia: pretty | json (domyślnie: pretty)
  -h, --help              Pokaż pomoc
  -V, --version           Wersja programu
```

Szybka pomoc:
```bash
cargo run -- --help
```

---

## Przykłady

1) **Podstawowy przypadek** (pretty-print):
```bash
cargo run -- -T 800 -A 0.5 -t 10 -e 0.9 -p 4
```
Przykładowy wynik:
```
=== Stefan–Boltzmann Radiation ===
Wejście:
  T [K]:      800.0000
  A [m²]:     0.5000
  ε [-]:      0.9000
  t [s]:      10.0000
Wyniki:
  P [W]:      10451.6341
  E [J]:      104516.3413
```

2) **Wyjście w JSON** (łatwe do dalszej obróbki):
```bash
cargo run -- -T 800 -A 0.5 -t 10 -e 0.9 --output json
```
Wynik:
```json
{
  "power_w": 10451.6341291008,
  "energy_j": 104516.341291008
}
```

3) **Przykład astrofizyczny** (moc gwiazdy traktowanej jak ciało czarne):
```bash
cargo run --example star
```

---

## Wyjście JSON vs. czytelny tekst

- `--output pretty` – format przyjazny człowiekowi (domyślny).
- `--output json` – stabilna struktura do parsowania w skryptach/CI.

Precyzję liczb w trybie `pretty` kontroluje `-p/--precision`.  
W trybie `json` drukujemy pełen `f64` (umożliwia to dokładne przeliczenia po stronie użytkownika).

---

## Walidacja i jednostki

- **Temperatura**: $[\mathrm{K}] > 0$ — *zawsze w Kelvinach!*  
  Konwersja z °C: $T[\mathrm{K}] = T[^{\circ}\mathrm{C}] + 273.15$
- **Powierzchnia**: $[\mathrm{m}^2] \ge 0$
- **Czas**: $[\mathrm{s}] \ge 0$
- **Emisyjność**: $\varepsilon \in [0,1]$

Jednostki wyników:
- **Moc** $P$: Waty $[\mathrm{W}]$
- **Energia** $E$: Dżule $[\mathrm{J}]$ (pamiętaj: $1\,\mathrm{Wh} = 3600\,\mathrm{J}$)

W przypadku danych spoza zakresu aplikacja zwraca czytelny błąd i nie wykonuje obliczeń.

---

## Co liczy aplikacja — w prostych krokach

Dla podanych $T$, $A$, $\varepsilon$ i $t$:

1. **Gęstość mocy** na $1\,\mathrm{m^2}$: $\sigma\,T^{4}$
2. **Uwzględnienie materiału**: $\sigma\,\varepsilon\,T^{4}$
3. **Moc całkowita** dla powierzchni $A$: $$ P = \sigma\,\varepsilon\,A\,T^{4} $$
4. **Energia w czasie** $t$: $$ E = P \cdot t $$

Wnioski:
- $P$ rośnie **jak $T^{4}$** (np. 2× wyższa $T$ ⇒ ~16× większe $P$)
- $P$ jest **liniowe** w $A$ i $\varepsilon$
- $E$ jest **liniowe** w czasie $t$, jeśli $T$ (a więc $P$) jest stałe

---

## Struktura projektu

```
radiation-sim/
├─ Cargo.toml
├─ README.md
├─ .gitignore
├─ src/
│  ├─ main.rs        # punkt wejścia CLI: parse -> validate -> sim -> report
│  ├─ lib.rs         # re-eksport modułów dla tests/examples/benches
│  ├─ cli.rs         # definicja argumentów (clap)
│  ├─ physics.rs     # stała σ, wzory: P=σ ε A T^4, E=P·t
│  ├─ sim.rs         # spina obliczenia i zwraca wynik
│  ├─ report.rs      # pretty-print i JSON
│  └─ validate.rs    # walidacja zakresów i normalizacja parametrów
├─ tests/
│  ├─ smoke_cli.rs   # testy uruchomienia CLI
│  └─ physics_spec.rs# testy wzoru (porównanie z SIGMA)
├─ examples/
│  └─ star.rs        # przykład: luminosity gwiazdy
└─ benches/
   └─ perf.rs        # benchmark funkcji obliczeniowych (criterion)
```

---

## Testy i benchmarki

Uruchom wszystkie testy:
```bash
cargo test
```

Benchmark (wymaga `criterion`):
```bash
cargo bench
```

Przykładowe polecenie do sanity-checku (skalowanie $T^{4}$):
```bash
cargo run -- -T 800  -A 0.5 -t 10 -e 0.9
cargo run -- -T 1600 -A 0.5 -t 10 -e 0.9  # moc ≈ 16× większa
```

---

## Ograniczenia modelu i roadmapa

Obecny model zakłada **stałą temperaturę** w trakcie obliczeń (nie modeluje stygnięcia/przyrostu temperatury). To celowe uproszczenie, dzięki któremu:
- obliczenia są natychmiastowe,
- wynik energii w czasie jest po prostu $E = P \cdot t$.

**Potencjalne rozszerzenia:**
- `--sweep` (skanowanie po zakresie $T$ i/lub $A$; eksport CSV)
- tryb `--explain` (druk kolejnych kroków obliczeń)
- prosty model stygnięcia: równanie $$ m c \frac{dT}{dt} = -\sigma\,\varepsilon\,A\,T^{4} $$
- dodatkowe formaty wyjścia (CSV, YAML)
- zakresy i preset-y materiałów (typowe $\varepsilon$ dla metali/ceramik)
- walidacja jednostek wejściowych i przeliczniki (np. °C → K)

---

## FAQ

**Czy mogę podawać temperaturę w °C?**  
Nie bezpośrednio. Przelicz na Kelwiny: $T[\mathrm{K}] = T[^{\circ}\mathrm{C}] + 273.15$.

**Wyjście „dziwnie duże/małe” — co sprawdzić?**  
Najpierw jednostki: $T$ w **K**, $A$ w **m²**, $t$ w **s**, $\varepsilon \in [0,1]$.  
Pamiętaj: $P \sim T^{4}$ — niewielka zmiana $T$ mocno wpływa na wynik.

**Czy uwzględniacie otoczenie/pochłanianie?**  
Nie. To prosty model: izolowane ciało, emisja „w próżnię”.

**Dlaczego JSON drukuje pełne `f64`?**  
By nie tracić informacji przy dalszym przetwarzaniu. W trybie `pretty` możesz kontrolować precyzję `-p`.

---

## Licencja

Dual-licensed: **MIT** OR **Apache-2.0**.  
Wybierz jedną z licencji zgodnie z własnymi potrzebami.
