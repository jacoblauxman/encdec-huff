use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

fn main() -> Result<(), std::io::Error> {
    let file_path = PathBuf::from("le-mis.txt");
    let char_counts = count_characters(&file_path).expect("Should return char_counts");
    print_char_counts(&char_counts);

    Ok(())
}

fn count_characters(input: &PathBuf) -> Result<HashMap<u8, usize>, String> {
    let file = File::open(input).expect("Failed to read file input");

    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut buffer = [0; 64 * 1024];
    let mut char_counts = HashMap::new();

    loop {
        let bytes_read = reader.read(&mut buffer).map_err(|e| e.to_string())?;
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

    println!("Characer Counts:");
    for (&byte, &count) in chars.iter() {
        let display_c = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            byte as char
        } else {
            '�' // replacement-char
        };

        println!("{} (0x{:02X}): {}", display_c, byte, count);
    }
}
