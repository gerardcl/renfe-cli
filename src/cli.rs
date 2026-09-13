use chrono::{Datelike, Utc};
use getopts::Options;
use pyo3::{PyResult, exceptions::PyValueError, pyfunction};
use std::env;
use std::io::{self, BufRead, Write};

use crate::renfe::{Renfe, Station};

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
    },
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return Ok(());
  }

  let mut renfe = Renfe::new(matches.opt_present("c"))?;

  let origin = select_station(
    &renfe,
    matches.opt_str("f").expect("Missing origin station"),
  )?;
  let destination = select_station(
    &renfe,
    matches.opt_str("t").expect("Missing destination station"),
  )?;
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

fn select_station(renfe: &Renfe, station: String) -> PyResult<Station> {
  let matches = renfe.stations_match(station.clone())?;
  let stdin = io::stdin();
  let stdout = io::stdout();

  select_station_from(&station, matches, &mut stdin.lock(), &mut stdout.lock())
    .map_err(PyValueError::new_err)
}

fn select_station_from(
  station: &str,
  mut matches: Vec<Station>,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<Station, String> {
  matches.sort_by(|left, right| {
    left
      .name
      .cmp(&right.name)
      .then_with(|| left.id.cmp(&right.id))
  });

  match matches.len() {
    0 => {
      return Err(format!(
        "Provided input '{station}' does not match any station name"
      ));
    },
    1 => return Ok(matches.remove(0)),
    _ => {},
  }

  writeln!(output, "Multiple stations match '{station}':").map_err(|error| error.to_string())?;
  for (index, station) in matches.iter().enumerate() {
    writeln!(output, "  {}. {}", index + 1, station.name).map_err(|error| error.to_string())?;
  }

  loop {
    write!(output, "Select a station [1-{}]: ", matches.len())
      .and_then(|_| output.flush())
      .map_err(|error| error.to_string())?;

    let mut selection = String::new();
    if input
      .read_line(&mut selection)
      .map_err(|error| error.to_string())?
      == 0
    {
      return Err(format!("No station was selected for '{station}'"));
    }

    if let Ok(index) = selection.trim().parse::<usize>()
      && let Some(station) = index.checked_sub(1).and_then(|index| matches.get(index))
    {
      return Ok(station.clone());
    }

    writeln!(
      output,
      "Please enter a number between 1 and {}.",
      matches.len()
    )
    .map_err(|error| error.to_string())?;
  }
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
  opts.optflag("c", "cercanias", "Option to search over Renfe Cercanías");
  opts.optflag("h", "help", "Print this help menu");

  opts
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;

  fn station(name: &str, id: &str) -> Station {
    Station {
      name: name.into(),
      id: id.into(),
    }
  }

  #[test]
  fn selects_the_only_station_without_prompting() {
    let mut input = Cursor::new(Vec::new());
    let mut output = Vec::new();

    let selected = select_station_from(
      "girona",
      vec![station("Estación de tren Girona", "79300")],
      &mut input,
      &mut output,
    )
    .unwrap();

    assert_eq!(selected.id, "79300");
    assert!(output.is_empty());
  }

  #[test]
  fn explains_when_no_station_names_match() {
    let mut input = Cursor::new(Vec::new());
    let mut output = Vec::new();

    let error = select_station_from("unknown", vec![], &mut input, &mut output).unwrap_err();

    assert_eq!(
      error,
      "Provided input 'unknown' does not match any station name"
    );
    assert!(output.is_empty());
  }

  #[test]
  fn lists_multiple_matches_and_selects_their_ordinal_number() {
    let mut input = Cursor::new(b"2\n");
    let mut output = Vec::new();

    let selected = select_station_from(
      "madrid",
      vec![
        station("Madrid-Puerta de Atocha", "60000"),
        station("Madrid-Chamartín", "17000"),
      ],
      &mut input,
      &mut output,
    )
    .unwrap();

    assert_eq!(selected.id, "60000");
    assert_eq!(
      String::from_utf8(output).unwrap(),
      concat!(
        "Multiple stations match 'madrid':\n",
        "  1. Madrid-Chamartín\n",
        "  2. Madrid-Puerta de Atocha\n",
        "Select a station [1-2]: "
      )
    );
  }

  #[test]
  fn asks_again_after_an_invalid_selection() {
    let mut input = Cursor::new(b"none\n0\n1\n");
    let mut output = Vec::new();

    let selected = select_station_from(
      "madrid",
      vec![
        station("Madrid-Chamartín", "17000"),
        station("Madrid-Puerta de Atocha", "60000"),
      ],
      &mut input,
      &mut output,
    )
    .unwrap();

    assert_eq!(selected.id, "17000");
    assert_eq!(
      String::from_utf8(output)
        .unwrap()
        .matches("Please enter a number between 1 and 2.")
        .count(),
      2
    );
  }
}
