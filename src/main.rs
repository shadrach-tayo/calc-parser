mod parser;
mod symbol_table;
mod analyzer;
mod executor;

fn main() {
    run_interpreter();
}

// fn process_file<'a>(current_program_path: &'a str, source_path: &'a str) -> &'a str {
//     const CALC_PREFIX: &str = ".calc";
//     if !source_path.ends_with(CALC_PREFIX) {
//         eprintln!(
//             "{}: Invalid argument  '{}': It must end with {CALC_PREFIX}",
//             current_program_path, source_path
//         );
//         return ""
//     }
//
//     let source_code = std::fs::read_to_string(source_path);
//     let source_code = source_code.unwrap();
//
//     source_code
// }

fn run_interpreter() {
    eprintln!("* Calc interactive interpreter *");

    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let command = input_command();
        if command.len() == 0 {
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
            trimmed_command => match parser::parse_program(&trimmed_command) {
                Ok((rest, parsed_program)) => {
                    if rest.len() > 0 {
                        eprintln!("Unparsed Input: `{}`.", rest);
                        return;
                    } else {
                        match analyzer::analyze_program(&mut variables, &parsed_program) {
                            Ok(analyzed_program) => {
                                executor::execute_program(&mut variables, &analyzed_program)
                            }
                            Err(err) => eprintln!("Error: {:?}", err)
                        }
                    }

                }
                Err(err) => eprintln!("Error: {:?}", err)
            }
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