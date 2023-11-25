use chrono::{Datelike, Utc};
use getopts::Options;
use pyo3::pyfunction;
use std::env;

use crate::timetable::{print_timetable, search_timetable};

#[pyfunction]
pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let now = Utc::now();
    let opts = set_opts();

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
    let day = matches.opt_str("d").unwrap_or(now.day().to_string());
    let month = matches.opt_str("m").unwrap_or(now.month().to_string());
    let year = matches.opt_str("y").unwrap_or(now.year().to_string());
    let wait = matches
        .opt_str("w")
        .unwrap_or(2.to_string())
        .parse::<u64>()
        .unwrap();

    println!("Today is: {}-{}-{}", now.year(), now.month(), now.day());
    println!("Searching timetable for date: {}-{}-{}", year, month, day);

    let timetable = search_timetable(
        origin.unwrap(),
        destination.unwrap(),
        day,
        month,
        year,
        wait,
    );

    print_timetable(timetable);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn set_opts() -> Options {
    let mut opts = Options::new();
    opts.optopt("f", "", "Set From origin station", "ORIGIN");
    opts.optopt("t", "", "Set To destination station", "DESTINATION");
    opts.optopt(
        "d",
        "day",
        "Set Day to search timetable for (default: today)",
        "DAY",
    );
    opts.optopt(
        "m",
        "month",
        "Set Month to search timetable for (default: today's month)",
        "MONTH",
    );
    opts.optopt(
        "y",
        "year",
        "Set Year to search timetable for (default: today's year)",
        "YEAR",
    );
    opts.optopt(
        "w",
        "wait",
        "Set Wait time in seconds for Renfe search result page (default: 2)",
        "SECONDS",
    );
    opts.optflag("h", "help", "Print this help menu");

    opts
}
