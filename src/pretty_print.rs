use crate::ast::{Expr, Number, Binary, Operator, With, Binding, Id};
use std::fmt::{Display, Formatter};
use std::fmt;

pub fn pretty_print(expr: &Expr) {
    fn pretty_print<T: Printable + ?Sized>(expr: &Box<T>, prefix: String, child_prefix: String) {
        println!("{}{}", prefix, expr);

        if expr.child_count() > 0 {
            let last_child = expr.child_count() - 1;

            for (i, child) in expr.children().iter().enumerate() {
                let new_prefix: String;
                let new_child_prefix: String;
                if i == last_child {
                    new_prefix = format!("{}{}", child_prefix, "└── ");
                    new_child_prefix = format!("{}{}", child_prefix, "    ");
                } else {
                    new_prefix = format!("{}{}", child_prefix, "├── ");
                    new_child_prefix = format!("{}{}", child_prefix, "│   ");
                }
                pretty_print(child, new_prefix, new_child_prefix);
            }
        }
    }

    pretty_print(&Box::new(expr.clone()), "".to_string(), "".to_string());
    println!()
}

trait Printable: Display {
    fn child_count(&self) -> usize { 0 }
    fn children(&self) -> Vec<Box<dyn Printable>> { Vec::new() }
    fn name(&self) -> String;
}

impl Printable for Number {
    fn name(&self) -> String {
        "Number".to_string()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.val)
    }
}

impl Printable for Binary {
    fn child_count(&self) -> usize { 2 }
    fn children(&self) -> Vec<Box<dyn Printable>> {
        vec!(Box::new(self.left.clone()), Box::new(self.right.clone()))
    }

    fn name(&self) -> String {
        "Binary".to_string()
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.op.name())
    }
}

impl Printable for Operator {
    fn name(&self) -> String {
        return match self {
            Operator::Add => "Add".to_string(),
            Operator::Sub => "Subtract".to_string(),
            Operator::Mul => "Multiply".to_string(),
            Operator::Div => "Divide".to_string(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Printable for With {
    fn child_count(&self) -> usize { 2 }
    fn children(&self) -> Vec<Box<dyn Printable>> {
        vec!(Box::new(self.binding.clone()), Box::new(self.input.clone()))
    }

    fn name(&self) -> String {
        "With".to_string()
    }
}

impl Display for With {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Printable for Binding {
    fn child_count(&self) -> usize { 2 }
    fn children(&self) -> Vec<Box<dyn Printable>> {
        vec!(self.identifier.clone(), Box::new(self.replace.clone()))
    }

    fn name(&self) -> String {
        "Binding".to_string()
    }
}

impl Display for Binding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Printable for Id {
    fn name(&self) -> String {
        "Id".to_string()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.val)
    }
}

impl Printable for Expr {
    fn child_count(&self) -> usize {
        return match self {
            Expr::Number(expr) => expr.child_count(),
            Expr::Binary(expr) => expr.child_count(),
            Expr::With(expr) => expr.child_count(),
            Expr::Id(expr) => expr.child_count(),
        }
    }

    fn children(&self) -> Vec<Box<dyn Printable>> {
        return match self {
            Expr::Number(expr) => expr.children(),
            Expr::Binary(expr) => expr.children(),
            Expr::With(expr) => expr.children(),
            Expr::Id(expr) => expr.children(),
        }
    }

    fn name(&self) -> String {
        return match self {
            Expr::Number(expr) => expr.name(),
            Expr::Binary(expr) => expr.name(),
            Expr::With(expr) => expr.name(),
            Expr::Id(expr) => expr.name(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            Expr::Number(expr) => expr.fmt(f),
            Expr::Binary(expr) => expr.fmt(f),
            Expr::With(expr) => expr.fmt(f),
            Expr::Id(expr) => expr.fmt(f),
        }
    }
}
