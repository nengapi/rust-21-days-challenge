use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn main(args: Vec<String>) {
    let path: &Path = Path::new(&args[1]);
    let file: File = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            std::process::exit(1);
        }
    };
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
