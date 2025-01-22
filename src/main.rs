mod alphanumeric_mode;
mod bit_block;
mod code_matrix;
mod data_codewords;
mod data_mask;
mod error_correction;
mod format_info;
mod gf_256;
mod pattern_scoring;
mod symbol_matrix;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(
    author = "Ged Dackys <ged@onegood.dev>",
    version = "0.1",
    about = "Generates a M4-L version micro QR code from an input string encoded in alphanumeric mode. Outputs 210x210 pixels image file."
)]
struct Args {
    /// Input string (max 21 chars, alphanumeric character set only)
    #[arg(short, long, value_parser = validate_input)]
    input: String,

    /// Output file name (e.g. my_qrc.png)
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let input = args.input;
    let output = args.output;

    let data_codewords = data_codewords::generate(&input);
    let symbol_matrix = symbol_matrix::generate(&data_codewords);

    // TODO: write to an image file
}

fn validate_input(s: &str) -> Result<String, String> {
    if s.len() > 21 {
        return Err(String::from("Input must not exceed 21 characters"));
    }

    let re = Regex::new(r"^[0-9A-Z $%*+\-./:]*$").unwrap();

    if !re.is_match(s) {
        return Err(String::from("Input contains invalid characters. Only alphanumeric (0-9, A-Z) and special characters ( $%*+-./:) are allowed"));
    }

    Ok(s.to_string())
}
