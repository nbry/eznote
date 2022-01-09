use std::{env, process};

use eznote::Note;

fn main() {
    let note = Note::new(env::args()).expect("a note");

    if let Err(e) = eznote::run(note) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
