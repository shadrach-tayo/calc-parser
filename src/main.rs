mod parser;
mod symbol_table;
mod analyzer;

fn main() {
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let source_file = args.next();

    if source_file.is_none() {
        eprintln!("{}: Missing argument <file>.calc", current_program_path);
        return;
    } else {
        process_file(&current_program_path, &source_file.unwrap());
    }
}

fn process_file(current_program_path: &str, source_path: &str) {
    const CALC_PREFIX: &str = ".calc";
    if !source_path.ends_with(CALC_PREFIX) {
        eprintln!(
            "{}: Invalid argument  '{}': It must end with {CALC_PREFIX}",
            current_program_path, source_path
        );
        return;
    }

    let source_code = std::fs::read_to_string(source_path);
    let source_code = source_code.unwrap();

    let parsed_program;
    // process file
    // let syntax_tree = ;
    match parser::parse_program(&source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!("invalid remaining code in '{}': {}", source_path, trimmed_rest);
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path, err);
            return;
        }
    }

    let analyzed_program;
    let mut variables = symbol_table::SymbolTable::new();
    match analyzer::analyze_program(&mut variables, &parsed_program) {
        Ok(analyzed_tree) => {
            analyzed_program = analyzed_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {}", source_path, err);
            return;
        }
    }

    println!("Symbol table: {:#?}", variables);
    println!("Analyzed program: {:#?}", analyzed_program);
}

fn run_interpreter() {
    eprintln!("* Calc interactive interpreter *");
}