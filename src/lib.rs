use pyo3::prelude::*;

mod renfe;
use renfe::Renfe;
mod cli;
use cli::main;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn renfe_cli(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Renfe>()?;
    m.add_function(wrap_pyfunction!(main, m)?)?;

    Ok(())
}
