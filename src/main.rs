use renfe_cli::cli;

fn main() -> Result<(), pyo3::PyErr> {
  // Initialize the Python interpreter required
  pyo3::Python::initialize();

  cli::native_main()?;

  Ok(())
}
