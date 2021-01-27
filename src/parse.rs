// ============================================================================
// GRAMMAR:
// WAE = Number
//     | (+ WAE WAE)
//     | (- WAE WAE)
//     | (* WAE WAE)
//     | (/ WAE WAE)
//     | (With ([x WAE]) WAE)
//     | x
// ============================================================================

use crate::{Expr, Number, Binary, Operator, With, Binding, Id};
use std::num::ParseFloatError;
use std::str::Chars;

// parse returns an abstract syntax tree that represents the expression provided
// by the given string.
pub fn parse(rep: String) -> Result<Expr, String> {
    Expr::parse(rep)
}

// A type that implements Parsable is able to construct an instance of itself
// given a string representation.
pub(crate) trait Parsable {
    type Representation;
    type Parsed;
    
    // parse parses the given input String and returns the expression tree it
    // represents.
    fn parse(input: Self::Representation) -> Result<Self::Parsed, String>;
}

impl Parsable for Expr {
    type Representation = String;
    type Parsed = Expr;

    fn parse(rep: String) -> Result<Expr, String> {
        if rep.is_empty() {
            return Err("expected a non-empty input".to_string())
        }
        let first_char: char;
        let parse_str: String = rep.trim().to_string();
        match parse_str.chars().next() {
            Some(c) => first_char = c,
            None => return Err(format!("unable to parse expression: {}", rep)),
        }
        if first_char.is_ascii_digit() {
            return Number::parse(parse_str);
        }
        if first_char == OPEN_PAREN {
            return parse_paren_expr(parse_str);
        }
        if first_char.is_alphabetic() {
            return Ok(Id::parse(parse_str)?.into());
        }
        Err(format!("unexpected symbol: {}", first_char))
    }
}

impl Parsable for Number {
    type Representation = String;
    type Parsed = Expr;
    
    fn parse(input: String) -> Result<Expr, String> {
        let result_val: Result<f32, ParseFloatError> = input.parse::<f32>();
        return match result_val {
            Ok(val) => Ok(Number{ val }.into()),
            Err(_) => Err("expected a Number".to_string())
        }
    }
}

impl Parsable for Binary {
    type Representation = Vec<String>;
    type Parsed = Expr;
    
    fn parse(tokens: Vec<String>) -> Result<Expr, String> {
        if tokens.len() != 3 {
            return Err("expected an operator type and two inputs for binary expression".to_string())
        }
        let op: Operator = Operator::parse(tokens[0].clone())?;
        let left: Expr = Expr::parse(tokens[1].clone())?;
        let right: Expr = Expr::parse(tokens[2].clone())?;
        Ok(Binary { op, left, right }.into())
    }
}

impl Parsable for Operator {
    type Representation = String;
    type Parsed = Operator;
    
    fn parse(input: String) -> Result<Operator, String> {
        if input.eq(ADD_OP) {
            return Ok(Operator::Add)
        }
        if input.eq(SUB_OP) {
            return Ok(Operator::Sub)
        }
        if input.eq(MUL_OP) {
            return Ok(Operator::Mul)
        }
        if input.eq(DIV_OP) {
            return Ok(Operator::Div)
        }
        Err(format!("unexpected operator: {}", input))
    }
}

impl Parsable for With {
    type Representation = Vec<String>;
    type Parsed = Expr;
    
    fn parse(tokens: Vec<String>) -> Result<Expr, String> {
        if tokens.len() != 3 {
            return Err("expected 'with' symbol, binding, and input for With expression".to_string())
        }
        if !tokens[0].eq(WITH_OP) {
            return Err("expected 'with'".to_string())
        }
        let binding: Binding = Binding::parse(tokens[1].clone())?;
        let input: Expr = Expr::parse(tokens[2].clone())?;
        Ok(With { binding, input }.into())
    }
}

impl Parsable for Binding {
    type Representation = String;
    type Parsed = Binding;
    
    fn parse(input: String) -> Result<Binding, String> {
        let mut stripped: String;
        match strip_ends(input.trim().to_string(), OPEN_PAREN, CLOSE_PAREN) {
            Some(s) => stripped = s,
            None => return Err("expected With binding to be wrapped in parentheses".to_string())
        }
        match strip_ends(stripped.trim().to_string(), OPEN_BRACE, CLOSE_BRACE) {
            Some(s) => stripped = s,
            None => return Err("expected With binding to be wrapped in brackets".to_string())
        }
        if stripped.is_empty() {
            return Err("expected a binding expression for With clause".to_string())
        }
        let tokens: Vec<String> = split_tokens(stripped);
        if tokens.len() != 2 {
            return Err("expected an identifier and bound expression for With clause".to_string())
        }
        let identifier: Box<Id> = Box::new(Id::parse(tokens[0].clone())?);
        let replace: Expr = Expr::parse(tokens[1].clone())?;
        Ok(Binding{ identifier, replace })
    }
}

impl Parsable for Id {
    type Representation = String;
    type Parsed = Id;
    
    fn parse(input: String) -> Result<Id, String> {
        let parse_str: &str = input.trim();
        if parse_str.eq(WITH_OP) {
            return Err("identifier cannot be 'with'".to_string())
        }
        for ch in parse_str.chars() {
            if !ch.is_alphabetic() {
                return Err("expected an alphabetic identifier".to_string())
            }
        }
        Ok(Id { val: parse_str.to_string() })
    }
}

// parse_paren_expr returns the Binary or With expression represented by the 
// given parenthesized string.
fn parse_paren_expr(parse_string: String) -> Result<Expr, String> {
    let stripped: String;
    match strip_ends(parse_string, OPEN_PAREN, CLOSE_PAREN) {
        Some(s) => stripped = s,
        None => return Err("expected opening and closing parentheses".to_string())
    }
    let exprs: Vec<String> = split_tokens(stripped);
    if exprs.len() == 0 {
        return Err("expected an expression within the parentheses".to_string())
    }
    return match &exprs[0][..] {
        ADD_OP | SUB_OP | MUL_OP | DIV_OP => Binary::parse(exprs),
        WITH_OP => With::parse(exprs),
        s @ _ => Err(format!("unexpected parenthesized expression: {}", s)),
    }
}

// split_tokens splits the given input string into a series of parenthesized and
// literal expression tokens.
fn split_tokens(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let mut is_paren_expr: bool = false;
    let mut paren_count: i32 = 0;
    let mut token: String = "".to_string();

    let mut chars: Chars = input.trim().chars();
    while let Some(curr_char) = chars.next() {
        if !is_paren_expr && curr_char.is_ascii_whitespace() {
            if !token.is_empty() {
                tokens.push(token);
                token = String::new()
            }
            if paren_count != 0 || is_paren_expr {
                return Vec::new()
            }
            continue
        }
        if curr_char.eq(&OPEN_PAREN) {
            paren_count += 1;
            if !is_paren_expr {
                is_paren_expr = true
            }
        } else if curr_char.eq(&CLOSE_PAREN) {
            paren_count -= 1;
            if paren_count == 0 {
                is_paren_expr = false
            }
        }
        token.push(curr_char)
    }

    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

// strip_ends strips the given prefix and suffix from the given string.
// Returns None if the prefix or suffix do not exist.
fn strip_ends(input: String, prefix: char, suffix: char) -> Option<String> {
    let stripped: String;
    match input.strip_prefix(prefix) {
        Some(s) => stripped = s.to_string(),
        None => return None,
    }
    return match stripped.strip_suffix(suffix) {
        Some(s) => Some(s.to_string()),
        None => return None,
    }
}

// Constants for use in parsing expressions.
const OPEN_PAREN:  char = '(';
const CLOSE_PAREN: char = ')';
const OPEN_BRACE:  char = '[';
const CLOSE_BRACE: char = ']';
const ADD_OP: &str = "+";
const SUB_OP: &str = "-";
const MUL_OP: &str = "*";
const DIV_OP: &str = "/";
const WITH_OP: &str = "with";
