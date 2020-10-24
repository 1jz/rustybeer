use structopt::StructOpt;
mod commands;

#[derive(Debug, StructOpt)]
#[structopt(name = "RustyBeer", version = "0.1")]
/// RustyBeer Calculators CLI
pub enum RustyBeer {
    Abv(commands::abv::AbvOptions),
    BeerStyle(commands::beer_style::BeerStyleOptions),
    BoilOff(commands::boil_off::BoilOffOptions),
    Diluting(commands::diluting::DilutingOptions),
    NumBottles(commands::num_bottles::NumBottlesOptions),
    Priming(commands::priming::PrimingOptions),
    SgCorrection(commands::sg_correction::SgCorrectionOptions),
}

fn main() {
    let opt = RustyBeer::from_args();
    match opt {
        RustyBeer::Abv(opts) => commands::abv::calculate_and_print(opts),
        RustyBeer::BeerStyle(opts) => commands::beer_style::calculate_and_print(opts),
        RustyBeer::BoilOff(opts) => commands::boil_off::calculate_and_print(opts),
        RustyBeer::Diluting(opts) => commands::diluting::calculate_and_print(opts),
        RustyBeer::NumBottles(opts) => commands::num_bottles::calculate_and_print(opts),
        RustyBeer::Priming(opts) => commands::priming::calculate_and_print(opts),
        RustyBeer::SgCorrection(opts) => commands::sg_correction::calculate_and_print(opts),
    }
}
