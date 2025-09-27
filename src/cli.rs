use clap::{Parser, ValueEnum};
/// Format wyjścia wyników.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Output {
    /// Ładny tekst na stdout
    Pretty,
    /// JSON (do dalszej obróbki)
    Json,
}


/// Parametry linii poleceń.
#[derive(Debug, Parser)]
#[command(
name = "radiation-sim",
author,
version,
about = "Symulator promieniowania ciała czarnego (Stefan–Boltzmann)",
long_about = None
)]
pub struct Cli {
    /// Temperatura [K]
     #[arg(short = 'T', long = "temp", value_name = "K")]
pub temp: f64,


/// Powierzchnia [m^2]
#[arg(short = 'A', long = "area", value_name = "m2")]
pub area: f64,


/// Czas [s]
#[arg(short = 't', long = "time", value_name = "s")]
pub time: f64,


/// Emisyjność ε w [0,1] (domyślnie 1.0)
#[arg(short = 'e', long = "emis", default_value_t = 1.0)]
pub emissivity: f64,


/// Liczba miejsc po przecinku w wydruku (0..=10)
#[arg(short = 'p', long = "precision", default_value_t = 3)]
pub precision: u32,


/// Format wyjścia: pretty | json
#[arg(long = "output", value_enum, default_value_t = Output::Pretty)]
pub output: Output,
}