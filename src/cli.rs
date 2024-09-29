use chrono::{Datelike, Utc};
use getopts::Options;
use pyo3::{exceptions::PyValueError, pyfunction, PyResult};
use std::env;

use crate::renfe::Renfe;

#[pyfunction]
pub fn main() -> PyResult<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let now = Utc::now();
    let opts = set_opts();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            return Err(PyValueError::new_err(f.to_string()));
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    let mut renfe = Renfe::new()?;

    let origin = renfe.filter_station(matches.opt_str("f").expect("Missing origin station"))?;
    let destination =
        renfe.filter_station(matches.opt_str("t").expect("Missing destination station"))?;
    let day = match matches.opt_str("d") {
        Some(day) => day.parse()?,
        None => now.day(),
    };
    let month = match matches.opt_str("m") {
        Some(day) => day.parse()?,
        None => now.month(),
    };
    let year = match matches.opt_str("y") {
        Some(day) => day.parse()?,
        None => now.year(),
    };
    let sorted: bool = matches.opt_present("s");

    println!("Today is: {}-{}-{}", now.year(), now.month(), now.day());
    println!("Searching timetable for date: {}-{}-{}", year, month, day);

    renfe.set_train_schedules(&origin.id, &destination.id, day, month, year, sorted)?;

    println!("Origin station: {}", origin.name);
    println!("Destination station: {}", destination.name);

    renfe.print_timetable();

    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn set_opts() -> Options {
    let mut opts = Options::new();
    opts.optopt("f", "", "Set From origin station", "ORIGIN");
    opts.optopt("t", "", "Set To destination station", "DESTINATION");
    opts.optopt("d", "day", "Set the Day (default: today's day)", "DAY");
    opts.optopt(
        "m",
        "month",
        "Set the Month (default: today's month)",
        "MONTH",
    );
    opts.optopt("y", "year", "Set the Year (default: today's year)", "YEAR");
    opts.optflag("s", "sort", "Option to sort the timetable by Duration");
    opts.optflag("h", "help", "Print this help menu");

    opts
}
