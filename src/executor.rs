use crate::analyzer::{AnalyzedExpr, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement, AnalyzedTerm};
use crate::parser::{ExprOperator, TermOperator};
use crate::symbol_table::SymbolTable;

pub fn execute_program(variables: &mut SymbolTable, program: &AnalyzedProgram) {
    for statement in program {
        execute_statement(variables, statement);
    }
}

fn execute_statement(variables: &mut SymbolTable, statement: &AnalyzedStatement) {
    match statement {
        AnalyzedStatement::Declaration(_handle) => {}
        AnalyzedStatement::Assignment(handle, expr) => {
            let value = evaluate_expr(variables, expr);
            variables.set_value(*handle, value);
        }
        AnalyzedStatement::InputOperation(handle) => {
            let mut text = String::new();
            eprintln!("? ");
            std::io::stdin()
                .read_line(&mut text)
                .expect("Could not read line.");
            let value = text.trim().parse::<f64>().unwrap_or(0.);
            variables.set_value(*handle, value);
        }
        AnalyzedStatement::OutputOperation(expr) => {
            println!("{}", evaluate_expr(variables, expr));
        }
    }
}

fn evaluate_term(variables: &mut SymbolTable, term: &AnalyzedTerm) -> f64 {
    let mut result = evaluate_factor(variables, &term.0);
    for factor in &term.1 {
        match factor.0 {
            TermOperator::Multiply => result *= evaluate_factor(variables, &factor.1),
            TermOperator::Divide => result /= evaluate_factor(variables, &factor.1)
        }
    }
    result
}

fn evaluate_expr(variables: &mut SymbolTable, expr: &AnalyzedExpr) -> f64 {
    let mut result = evaluate_term(variables, &expr.0);
    for term in &expr.1 {
        match term.0 {
            ExprOperator::Add => result += evaluate_term(variables, &term.1),
            ExprOperator::Subtract => result -= evaluate_term(variables, &term.1)
        }
    }
    result
}

fn evaluate_factor(variables: &mut SymbolTable, factor: &AnalyzedFactor) -> f64 {
    match factor {
        AnalyzedFactor::Identifier(handle) => variables.get_value(*handle),
        AnalyzedFactor::Literal(value) => *value,
        AnalyzedFactor::SubExpression(expr) => evaluate_expr(variables, expr)
    }
}