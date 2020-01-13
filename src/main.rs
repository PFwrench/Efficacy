use std::error::Error;
use efficacy::cli;

fn main() -> Result<(), Box<dyn Error>>{
    cli::parse()
}
