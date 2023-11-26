use pyo3::prelude::*;

mod stations;
use stations::Renfe;
mod timetable;
use timetable::{print_timetable, search_timetable};
mod cli;
use cli::main;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn renfe_cli(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Renfe>()?;
    m.add_function(wrap_pyfunction!(search_timetable, m)?)?;
    m.add_function(wrap_pyfunction!(print_timetable, m)?)?;
    m.add_function(wrap_pyfunction!(main, m)?)?;

    Ok(())
}
