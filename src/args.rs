use std::path::PathBuf;

use clap::{Arg, Command, Subcommand, Parser};

// Encode
    // File path
    // Chunk type
    // Message
    // Output file (optional)
// Decode
    // File path
    // Chunk type
// Remove
    // File path
    // Chunk type
// Print
    // File path

#[derive(Subcommand)]
pub enum Subcommands {
    Encode {
        #[arg(short, long = "input")]
        input: PathBuf,
        #[arg(short, long = "type")]
        chunk_type: String,
        #[arg(short, long = "message")]
        message: String,
        #[arg(short, long = "output")]
        output: Option<PathBuf>,
    },
    Decode {
        #[arg(short, long = "path")]
        path: PathBuf,
        #[arg(short, long = "type")]
        chunk_type: String,
    },
    Remove {
        #[arg(short, long = "path")]
        path: PathBuf,
        #[arg(short, long = "type")]
        chunk_type: String,
    },
    Print {
        #[arg(short, long = "path")]
        path: PathBuf,
    },
}

#[derive(Parser)]
#[command(name = "PngMe")]
#[command(version = "1.0")]
#[command(author = "Haoran Wang <ubecwang@gmail.com>")]
#[command(about = "A demo for png hidden information.")]
#[clap(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommands
}

// fn main() {
//     let matches = Command::new("MyApp")
//         .version("1.0")
//         .author("Your Name <your.email@example.com>")
//         .about("An example of using clap")
//         .arg(
//             Arg::new("input")
//                 .short('i')
//                 .long("input")
//                 .value_name("FILE")
//                 .help("Sets the input file to use")
//                 .takes_value(true)
//                 .required(true),
//         )
//         .arg(
//             Arg::new("verbose")
//                 .short('v')
//                 .long("verbose")
//                 .help("Sets the level of verbosity"),
//         )
//         .get_matches();

//     // 获取 "input" 参数的值
//     if let Some(input) = matches.value_of("input") {
//         println!("Using input file: {}", input);
//     }

//     // 检查 "verbose" 参数是否存在
//     if matches.is_present("verbose") {
//         println!("Verbose mode is on");
//     }
// }