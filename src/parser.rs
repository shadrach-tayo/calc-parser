use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, char};
use nom::IResult;
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, tuple};

#[derive(Debug)]
pub enum ParsedStatement<'a> {
    Declaration(&'a str),
    InputOperation(&'a str),
    OutputOperation(ParsedExpr<'a>),
    Assignment(&'a str, ParsedExpr<'a>),
}

pub type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);
pub type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

#[derive(Debug, PartialEq)]
pub enum ParsedFactor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<ParsedExpr<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermOperator { Multiply, Divide }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExprOperator { Add, Subtract }

pub type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;

pub fn parse_program(input: &str) -> IResult<&str, ParsedProgram> {
    many0(
        preceded(
            skip_spaces,
            alt((
                parse_declaration,
                parse_input_statement,
                parse_output_statement,
                parse_assignment,
            )),
        )
    )(input)
}

pub fn parse_declaration(input: &str) -> IResult<&str, ParsedStatement> {
    // preceded(char('@'), many1(is_alphabetic)(input.bytes().into()).map(|output|)
    // todo!()
    tuple((char('@'), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::Declaration(output.2)))
}

pub fn parse_input_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('>'), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::InputOperation(output.2)))
}

pub fn parse_output_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('<'), skip_spaces, parse_expr))(input)
        .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.2)))
}

pub fn parse_assignment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((parse_identifier, skip_spaces, tag(":="), skip_spaces, parse_expr))(input)
        .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

pub fn parse_expr(input: &str) -> IResult<&str, ParsedExpr> {
    tuple((
        parse_term,
        many0(
            tuple((
                preceded(
                    skip_spaces,
                    alt((
                        map(char('+'), |_| ExprOperator::Add),
                        map(char('-'), |_| ExprOperator::Subtract),
                    )),
                ),
                parse_term
            ))
        )
    ))(input)
}


pub fn parse_term(input: &str) -> IResult<&str, ParsedTerm> {
    tuple((
        parse_factor,
        many0(
            tuple((
                preceded(
                    skip_spaces,
                    alt((
                        map(char('/'), |_| TermOperator::Divide),
                        map(char('*'), |_| TermOperator::Multiply),
                    )),
                ),
                parse_factor
            ))
        )
    ))(input)
}

pub fn parse_factor(input: &str) -> IResult<&str, ParsedFactor> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, ParsedFactor::Identifier),
            map(double, ParsedFactor::Literal),
            map(parse_subexpr, |expr| ParsedFactor::SubExpression(Box::new(expr))),
        )),
    )(input)
}

pub fn parse_subexpr(input: &str) -> IResult<&str, ParsedExpr> {
    delimited(
        preceded(skip_spaces, char('(')),
        parse_expr,
        preceded(skip_spaces, char(')'))
    )(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

pub fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |ch| chars.contains(ch))(input)
}