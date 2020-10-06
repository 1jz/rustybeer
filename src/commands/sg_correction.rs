extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::AppSubCommand;
pub use crate::calculators::sg_correction::SgCorrection;
use crate::utils::conversions::TemperatureBuilder;

impl AppSubCommand for SgCorrection {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        let ret = app.subcommand(SubCommand::with_name("sg_correction")
            .version("0.1")
            .author("Joseph Russell (josephrussell123@gmail.com)")
            .about("Corrects SG reading according to the difference between the measurement temperature and the calibration temperature")
            .arg(Arg::with_name("sg")
                 .short("s")
                 .long("sg")
                 .value_name("GRAVITY")
                 .help("Specific gravity reading")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("ct")
                 .short("c")
                 .long("ct")
                 .value_name("CALIBRATION TEMPERATURE")
                 .help("Calibration temperature with unit (C, F, K, etc.). Defaults to Celsius.")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("mt")
                 .short("m")
                 .long("mt")
                 .value_name("MEASUREMENT TEMPERATURE")
                 .help("Measurement temperature with unit (C, F, K, etc.). Defaults to Celsius.")
                 .required(true)
                 .takes_value(true)));

        ret
    }

    fn do_matches<'a>(&self, matches: &ArgMatches<'a>) {
        if let Some(ref matches) = matches.subcommand_matches("sg_correction") {
            let sg = value_t!(matches, "sg", f64).unwrap_or_else(|e| e.exit());
            let ct_str = value_t!(matches, "ct", String).unwrap_or_else(|e| e.exit());
            let ct = TemperatureBuilder::from_str(ct_str.clone()).as_fahrenheit();
            let mt_str = value_t!(matches, "mt", String).unwrap_or_else(|e| e.exit());
            let mt = TemperatureBuilder::from_str(mt_str.clone()).as_fahrenheit();

            println!("Measured gravity: {}", sg);
            println!("Calibration temperature: {}", ct_str);
            println!("Measurement temperature: {}", mt_str);
            println!("Corrected gravity: {:.3}", self.correct_sg(sg, ct, mt));
        }
    }
}
