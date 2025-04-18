use ::std::fs;
use std::path::Path;

fn main() {
    let directory = Path::new("./maciek");

    println!("Reading directory: {:?}", directory);

    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("Entry: {:?}", entry.path());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory: {}", e)
        }
    }
}
