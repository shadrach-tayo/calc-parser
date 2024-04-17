mod parser;

fn main() {
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let source_file = args.next();

    if source_file.is_none() {
        eprintln!("{}: Missing argument <file>.calc", current_program_path);
    }
    else {
        // process file
    }
}
