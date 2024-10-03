pub mod cli;
pub mod renfe;

use pyo3::prelude::*;

use cli::main;
use renfe::{Renfe, Schedule, Station};

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn renfe_cli(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Renfe>()?;
    m.add_class::<Station>()?;
    m.add_class::<Schedule>()?;
    m.add_function(wrap_pyfunction!(main, m)?)?;

    Ok(())
}
