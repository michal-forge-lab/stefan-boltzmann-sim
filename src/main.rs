
use clap::Parser;
use anyhow::Result;
use radiation_sim::{report, sim, validate, cli::Cli};


fn main() -> Result<()> {
    // Parsujemy argumenty CLI.
    let args = Cli::parse();


    // Walidujemy i normalizujemy parametry.
    let params = validate::validate_params(args)?;


    // Uruchamiamy symulację.
    let result = sim::run(&params)?;


    // Drukujemy wyniki w wybranym formacie.
    report::print_result(&params, &result);


    Ok(())
}