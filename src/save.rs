use std::io::{self, Write};
use log::info;

pub fn save_or_print_wordlist(wordlist: impl Iterator<Item = String>, output_file: Option<String>) -> io::Result<()> {
    match output_file {
        Some(file) => {
            let mut output = std::fs::File::create(&file)?;
            for word in wordlist {
                writeln!(output, "{}", word)?;
            }
            info!("Wordlist saved to {}", file);
        },
        None => {
            for word in wordlist {
                println!("{}", word);
            }
            info!("Wordlist printed to console");
        }
    }
    Ok(())
}
