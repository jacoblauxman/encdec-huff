mod cli;
mod huff;

use clap::Parser;
use cli::{Cli, Commands};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

fn main() -> Result<(), std::io::Error> {
    let input = Cli::parse();

    match &input.command {
        Commands::CharCount { input, output } => {
            println!("Count characters from {:?}...\n", input);
            let char_counts = count_characters(&input)?;
            print_char_counts(&char_counts);

            if let Some(output_path) = output {
                write_char_counts(&char_counts, &output_path)?;
            }
        }
    }

    Ok(())
}

fn count_characters(input: &PathBuf) -> Result<HashMap<u8, usize>, std::io::Error> {
    let file = File::open(input).expect("Failed to read file input");

    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut buffer = [0; 64 * 1024];
    let mut char_counts = HashMap::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for &byte in &buffer[..bytes_read] {
            *char_counts.entry(byte).or_insert(0) += 1;
        }
    }

    Ok(char_counts)
}

fn print_char_counts(char_counts: &HashMap<u8, usize>) {
    let mut chars: Vec<(&u8, &usize)> = char_counts.iter().collect();
    chars.sort_by(|a, b| b.1.cmp(a.1));

    println!("Characer Counts ({} in total):", chars.len());
    for (&byte, &count) in chars.iter() {
        let display_c = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            byte as char
        } else {
            '�' // replacement-char
        };

        println!("{} (0x{:02X}): {}", display_c, byte, count);
    }
}

fn write_char_counts(
    char_counts: &HashMap<u8, usize>,
    output_path: &PathBuf,
) -> Result<(), std::io::Error> {
    let mut file = File::create(output_path)?;

    for (&byte, &count) in char_counts.iter() {
        writeln!(
            file,
            "{} (0x{:02X}): {}",
            if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                byte as char
            } else {
                '�'
            },
            byte,
            count
        )?;
    }

    Ok(())
}
