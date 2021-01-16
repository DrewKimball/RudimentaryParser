mod ast;
mod parse;
mod calc;
mod subst;
mod pretty_print;

use ast::{Expr, Number, Binary, Operator, With, Binding, Id};
use parse::parse;
use calc::calc;
use crate::subst::Substitutable;
use crate::pretty_print::pretty_print;

fn main() {
    test_expr("253354".to_string());
    test_expr("(with ([x (- 23 7)]) (+ (/ x 2) (* 3 4)))".to_string());
    test_expr("(with ([x 1]) (with ([y (* x 2)]) (+ x y)))".to_string());
    test_expr("(with ([x 1]) (+ (with ([x (* x 2)]) x) x))".to_string());
    test_expr("gvtct".to_string());
    test_expr("(* 1 jksef)".to_string());
    test_expr("1vtct".to_string());
    test_expr("(+ 1 2 3)".to_string());
    test_expr("()".to_string());
    test_expr("(+ 1 2".to_string());
    println!("{}\n", "=".repeat(80));
}

fn test_expr(string_rep: String) {
    println!("{}\n", "=".repeat(80));
    println!("Original AST:");
    let mut ast: Expr;
    match parse(string_rep) {
        Ok(expr) => {
            ast = expr;
            pretty_print(&ast)
        },
        Err(msg) => {
            println!("Error: {}\n", msg);
            return
        }
    }
    println!("After With Replacement:");
    ast = ast.replace();
    pretty_print(&ast);
    match calc(&ast) {
        Ok(val) => println!("Result Of Evaluation: {}\n", val),
        Err(msg) => println!("Error: {}\n", msg)
    }
}
