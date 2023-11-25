use getopts::Options;
use pyo3::pyfunction;
use std::env;

use crate::timetable::{print_timetable, search_timetable};

#[pyfunction]
pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "", "Set From origin station", "ORIGIN");
    opts.optopt("t", "", "Set To destination station", "DESTINATION");
    opts.optopt("d", "day", "Set Day to search timetable for", "DAY");
    opts.optopt("m", "month", "Set Month to search timetable for", "MONTH");
    opts.optopt("y", "year", "Set Year to search timetable for", "YEAR");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let origin = matches.opt_str("f");
    let destination = matches.opt_str("t");
    let day = matches.opt_str("d");
    let month = matches.opt_str("m");
    let year = matches.opt_str("y");

    let timetable = search_timetable(
        origin.unwrap(),
        destination.unwrap(),
        day.unwrap(),
        month.unwrap(),
        year.unwrap(),
    );
    print_timetable(timetable);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
