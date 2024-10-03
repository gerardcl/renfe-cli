use renfe_cli::cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(cli::main()?)
}
