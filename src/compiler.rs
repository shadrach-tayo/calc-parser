use crate::analyzer::{
    AnalyzedExpr, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement, AnalyzedTerm,
};
use crate::parser::{ExprOperator, TermOperator};
use crate::symbol_table::SymbolTable;

fn translate_to_rust_expr(variables: &SymbolTable, expr: &AnalyzedExpr) -> String {
    let mut result = translate_to_rust_term(variables, &expr.0);
    for factor in &expr.1 {
        match factor.0 {
            ExprOperator::Add => {
                result += &format!(" + {}", &translate_to_rust_term(variables, &factor.1))
            }
            ExprOperator::Subtract => {
                result += &format!(" - {}", &translate_to_rust_term(variables, &factor.1))
            }
        }
    }
    result
}

fn translate_to_rust_term(variables: &SymbolTable, term: &AnalyzedTerm) -> String {
    let mut result = translate_to_rust_factor(variables, &term.0);
    for factor in &term.1 {
        match factor.0 {
            TermOperator::Multiply => {
                result += &format!(" * {}", &translate_to_rust_factor(variables, &factor.1))
            }
            TermOperator::Divide => {
                result += &format!(" / {}", &translate_to_rust_factor(variables, &factor.1))
            }
        }
    }
    result
}

fn translate_to_rust_factor(variables: &SymbolTable, factor: &AnalyzedFactor) -> String {
    match factor {
        AnalyzedFactor::Literal(value) => format!("{}f64", value),
        AnalyzedFactor::Identifier(handle) => variables.get_name(*handle),
        AnalyzedFactor::SubExpression(expr) => translate_to_rust_expr(variables, expr),
    }
}

fn translate_to_rust_statement(variables: &SymbolTable, statement: &AnalyzedStatement) -> String {
    match statement {
        AnalyzedStatement::InputOperation(handle) => {
            format!("_{} = input();", variables.get_name(*handle))
        }
        AnalyzedStatement::Declaration(handle) => {
            format!("let mut _{} = 0.0;", variables.get_name(*handle))
        }
        AnalyzedStatement::Assignment(handle, expr) => format!(
            "_{} = {};",
            variables.get_name(*handle),
            translate_to_rust_expr(variables, expr)
        ),
        AnalyzedStatement::OutputOperation(expr) => format!(
            "println!(\"{}\", {});",
            "{}",
            translate_to_rust_expr(variables, expr)
        ),
    }
}

pub fn translate_to_rust_program(variables: &SymbolTable, program: &AnalyzedProgram) -> String {
    let mut rust_program = String::new();
    rust_program += "use std::io::Write;\n";
    rust_program += "\n";
    rust_program += "#[allow(dead_code)]\n";
    rust_program += "fn input() -> f64 {\n";
    rust_program += "     let mut text = String::new();\n";
    rust_program += "     eprint!(\"? \");\n";
    rust_program += "     std::io::stderr().flush().unwrap();\n";
    rust_program += "     std::io::stdin()\n";
    rust_program += "         .read_line(&mut text)\n";
    rust_program += "         .expect(\"Cannot read line.\");\n";
    rust_program += "     text.trim().parse::<f64>().unwrap_or(0.)\n";
    rust_program += "}\n";
    rust_program += "\n";
    rust_program += "fn main() {\n";
    for statement in program {
        rust_program += "   ";
        rust_program += &translate_to_rust_statement(variables, statement);
        rust_program += "\n";
    }
    rust_program += "}\n";

    rust_program
}
