use std::fs;
use std::path::Path;

pub fn main(args: Vec<String>) {
    let path: &Path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        if let Some(file_name) = dir_entry.file_name().to_str() {
                            println!("{}", file_name);
                        }
                    }
                    Err(e) => eprintln!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}
