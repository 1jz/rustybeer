pub use crate::calculators::calorie_counter::{
    calculate_alcohol_calories, calculate_carbs_calories, calculate_total_calories,
    convert_oz_to_ml,
};
pub use crate::utils::abv_calories::{ABVCalories, Criteria, ABV_CALORIES};
use clap::{value_t, App, Arg, ArgMatches};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("calories")
            .version("0.1")
            .author("Roger Yu (roger.yu27 [at] gmail.com)")
            .about("Calculates calories by volume from original and final gravity or from alcohol by volume")
            .arg(Arg::with_name("og")
                 .short("o")
                 .long("og")
                 .value_name("OG")
                 .help("Original gravity")
                 .required(false)
                 .takes_value(true))
            .arg(Arg::with_name("fg")
                 .short("f")
                 .long("fg")
                 .value_name("FG")
                 .help("Final gravity")
                 .required(false)
                 .takes_value(true))
            .arg(Arg::with_name("abv")
                 .short("a")
                 .long("abv")
                 .value_name("ABV")
                 .help("Alcohol by volume")
                 .required_unless("fg")
                 .takes_value(true))
            .arg(Arg::with_name("cv")
                 .short("c")
                 .long("cv")
                 .value_name("CV")
                 .help("Custom volume")
                 .required(false)
                 .default_value("100")
                 .takes_value(true))
}

fn matches_to_criteria<'a>(matches: &ArgMatches<'a>) -> Criteria {
    Criteria {
        abv: matches.value_of("abv").map(|value| value.parse().unwrap()),
    }
}

pub fn do_matches<'a>(matches: &ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches("calories") {
        if matches.is_present("fg") && matches.is_present("og") {
            let fg = value_t!(matches, "fg", f32).unwrap_or_else(|e| e.exit());
            let og = value_t!(matches, "og", f32).unwrap_or_else(|e| e.exit());
            let cv = value_t!(matches, "cv", f32).unwrap_or_else(|e| e.exit());
            let ac = calculate_alcohol_calories(og, fg, cv);
            let cc = calculate_carbs_calories(og, fg, cv);
            let tc = calculate_total_calories(og, fg, cv);
            println!("{:<8} {:>8} {:>8}", "", "kcal", "kJ");
            println!("{:<8} {:>8.0} {:>8.0}", "Alcohol:", ac, ac * 4.184);
            println!("{:<8} {:>8.0} {:>8.0}", "Carbs:", cc, cc * 4.184);
            println!("{:<8} {:>8.0} {:>8.0}", "Total:", tc, tc * 4.184);
            println!("Per: {:.0} ml", cv);
        } else if matches.is_present("abv") {
            let abv = value_t!(matches, "abv", f32).unwrap_or_else(|e| e.exit());
            let cv = value_t!(matches, "cv", f32).unwrap_or_else(|e| e.exit());
            let criteria = matches_to_criteria(matches);

            for abv in ABV_CALORIES.iter() {
                if criteria.matches(abv) {
                    let lc = convert_oz_to_ml(abv.calories_low, cv);
                    let hc = convert_oz_to_ml(abv.calories_high, cv);
                    println!("Calories: {:.0} to {:.0} kcal", lc, hc);
                    println!("          {:.0} to {:.0} kJ", lc * 4.184, hc * 4.184);
                    println!("Per: {:.0} ml", cv);
                    return;
                }
            }

            println!("Could not find any ABV to calories matching criteria");
            return;
        } else {
            println!("Either specify original gravity and final gravity or just abv!");
            println!("{}", matches.usage());
        }
    }
}
