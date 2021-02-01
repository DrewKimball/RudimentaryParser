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
    println!("RUNNING RUDIMENTARY INTERPRETER TESTS");

    test_expr("253354", "253354");
    test_expr("(+ 1 2)", "3");
    test_expr("(with ([x (- 23 7)]) (+ (/ x 2) (* 3 4)))", "20");
    test_expr("(with ([x 1]) (with ([y (* x 2)]) (+ x y)))", "3");
    test_expr("(with ([x 1]) (+ (with ([x (* x 2)]) x) x))", "3");
    test_expr("  (+ 1 2)             ", "3");
    test_expr("    (   +  1     2    ) ", "3");
    test_expr(" (     with    (  [  x    (       -     23   7  ) ])    \
    ( +   (  /    x 2)    ( * 3     4) ))", "20");
    test_expr("gvtct", "error");
    test_expr("(* 1 jksef)", "error");
    test_expr("1vtct", "error");
    test_expr("(+ 1 2 3)", "error");
    test_expr("()", "error");
    test_expr("(+ 1 2", "error");
    println!("{}", "=".repeat(80));
}

fn test_expr(string_rep: &str, expected: &str) {
    println!("{}", "=".repeat(80));
    println!("Expression: {}\n", string_rep);
    println!("Test Parse:");
    let mut ast: Expr;
    match parse(string_rep.to_string()) {
        Ok(expr) => {
            ast = expr;
            pretty_print(&ast)
        },
        Err(msg) => {
            println!("Error: {}", msg);
            println!("Expected: {}", expected);
            return
        }
    }
    println!("Test Subst:");
    ast = ast.replace();
    pretty_print(&ast);
    print!("Test Calc: ");
    match calc(&ast) {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("Error: {}", msg)
    }
    println!("Expected: {}", expected)
}
