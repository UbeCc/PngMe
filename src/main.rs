// final goals are:
// pngme encode ./dice.png ruSt "This is a secret message!
// pngme decode ./dice.png ruSt
// pngme remove ./dice.png ruSt
// pngme print ./dice.png

// Requirements
// You should have four subcommands each with their own set of parameters.

use clap::{Subcommand, Parser};

mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

use args::{Cli, Subcommands};
use png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Subcommands::Encode { input, chunk_type, message, output } => {
            let input_file = &input;
            let output_file = &output.unwrap_or_else(|| input_file.clone());
            let message = &message;
            let chunk_type = &chunk_type;
            let result = Png::encode(input_file, output_file, message, chunk_type)?;
            println!("Result: {:?}", result);
        }
        Subcommands::Decode { path, chunk_type } => {
            let path = &path;
            let chunk_type = &chunk_type;
            let result = Png::decode(path, chunk_type)?;
            println!("Result: {:?}", result);
        }
        Subcommands::Remove { path, chunk_type } => {
            let path = &path;
            let chunk_type = &chunk_type;
            let result = Png::remove(path, chunk_type)?;
            println!("Result: {:?}", result);
        }
        Subcommands::Print { path } => {
            let path = &path;
            let result = Png::print(path)?;
            println!("Result: {:?}", result);
        }
        _ => {
            println!("Invalid command");
        }
    }
    Ok(())
}
