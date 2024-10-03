use renfe_cli::cli;

fn main() -> Result<(), pyo3::PyErr> {
    // Initialize the Python interpreter required
    pyo3::prepare_freethreaded_python();

    cli::main()?;

    Ok(())
}
