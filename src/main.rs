mod analyzer;
mod compiler;
mod executor;
mod parser;
mod symbol_table;

fn main() {
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let source_path = args.next();
    if let Some(source_path) = source_path {
        process_file(&current_program_path, &source_path);
    } else {
        run_interpreter();
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

    let target_path = source_path[0..source_path.len() - CALC_PREFIX.len()].to_string() + ".rs";
    let source_code = std::fs::read_to_string(source_path);
    if source_code.is_err() {
        eprintln!(
            "Failed to read from file {}: ({})",
            source_path,
            source_code.unwrap_err()
        );
        return;
    }
    let source_code = source_code.unwrap();

    // process file
    // let syntax_tree = ;
    let parsed_program = match parser::parse_program(&source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if !trimmed_rest.is_empty() {
                eprintln!(
                    "invalid remaining code in '{}': {}",
                    source_path, trimmed_rest
                );
                return;
            }
            syntax_tree
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path, err);
            return;
        }
    };

    let mut variables = symbol_table::SymbolTable::new();
    let analyzed_program = match analyzer::analyze_program(&mut variables, &parsed_program) {
        Ok(analyzed_tree) => analyzed_tree,
        Err(err) => {
            eprintln!("Invalid code in '{}': {}", source_path, err);
            return;
        }
    };

    println!("Symbol table: {:#?}", variables);
    println!("Analyzed program: {:#?}", analyzed_program);

    match std::fs::write(
        &target_path,
        compiler::translate_to_rust_program(&variables, &analyzed_program),
    ) {
        Ok(_) => eprintln!("Compiled {} to {}.", source_path, target_path),
        Err(err) => eprintln!("Failed to write to file {}: ({})", target_path, err),
    }
}

fn run_interpreter() {
    eprintln!("* Calc interactive interpreter *");

    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let command = input_command();
        if command.is_empty() {
            break;
        }

        match command.trim() {
            "q" => break,
            "c" => {
                variables = symbol_table::SymbolTable::new();
                eprintln!("Cleared variables.");
            }
            "v" => {
                for var in variables.iter() {
                    eprintln!(" {}: {}", var.0, var.1);
                }
            }
            trimmed_command => match parser::parse_program(trimmed_command) {
                Ok((rest, parsed_program)) => {
                    if !rest.is_empty() {
                        eprintln!("Unparsed Input: `{}`.", rest);
                        return;
                    } else {
                        match analyzer::analyze_program(&mut variables, &parsed_program) {
                            Ok(analyzed_program) => {
                                executor::execute_program(&mut variables, &analyzed_program)
                            }
                            Err(err) => eprintln!("Error: {:?}", err),
                        }
                    }
                }
                Err(err) => eprintln!("Error: {:?}", err),
            },
        }
    }
}

fn input_command() -> String {
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Could not read line.");
    text
}
