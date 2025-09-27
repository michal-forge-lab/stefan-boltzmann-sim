use crate::cli::Output;
use crate::sim::SimulationResult;
use crate::validate::Params;


/// Drukuje wynik w zadanym formacie.
pub fn print_result(params: &Params, res: &SimulationResult) {
    match params.output {
        Output::Pretty => print_pretty(params, res),
        Output::Json => print_json(res),
    }
}


fn print_pretty(params: &Params, res: &SimulationResult) {
    let prec = params.precision;
    println!("=== Stefan–Boltzmann Radiation ===");
    println!("Wejście:");
    println!(" T [K]: {:.prec$}", params.temp, prec = prec);
    println!(" A [m²]: {:.prec$}", params.area, prec = prec);
    println!(" ε [-]: {:.prec$}", params.emissivity, prec = prec);
    println!(" t [s]: {:.prec$}", params.time, prec = prec);
    println!("Wyniki:");
    println!(" P [W]: {:.prec$}", res.power_w, prec = prec);
    println!(" E [J]: {:.prec$}", res.energy_j, prec = prec);
}


fn print_json(res: &SimulationResult) {
    let json = serde_json::to_string_pretty(res).expect("serializacja JSON nie powiodła się");
    println!("{}", json);
}