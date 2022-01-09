use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

pub struct Note {
    pub filename: String,
    pub note: String,
}

impl Note {
    pub fn new(mut args: env::Args) -> Result<Note, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Please enter a file name as the second arg"),
        };

        let note = match args.next() {
            Some(arg) => arg,
            None => return Err("Please enter a note as the third arg"),
        };

        Ok(Note { filename, note })
    }
}

pub fn run(note: Note) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(note.filename)
        .expect("a file");

    let mut line = String::from("\n");
    line.push_str(note.note.as_str());

    if let Err(e) = file.write_all(line.as_bytes()) {
        eprintln!("Failed to write file: {}", e);
        process::exit(1);
    };

    Ok(())
}
