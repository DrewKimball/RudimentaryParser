use crate::{Expr, Number, Binary, Operator, With, Id};
use crate::subst::Substitutable;

// calc evaluates the given abstract syntax tree and returns the result.
pub fn calc(ast: &Expr) -> Result<f32, String> {
    // First carry out With substitution, then evaluate the resulting AST.
    ast.clone().replace().calc()
}

// A type that implements Calculable can be evaluated for a float result.
pub(crate) trait Calculable {
    // calc evaluates the expression rooted at this node and returns the result.
    fn calc(&self) -> Result<f32, String>;
}

impl Calculable for Expr {
    fn calc(&self) -> Result<f32, String> {
        return match self {
            Expr::Number(expr) => expr.calc(),
            Expr::Binary(expr) => expr.calc(),
            Expr::With(expr) => expr.calc(),
            Expr::Id(expr) => expr.calc(),
        }
    }
}

impl Calculable for Number {
    fn calc(&self) -> Result<f32, String> {
        Ok(self.val)
    }
}

impl Calculable for Binary {
    fn calc(&self) -> Result<f32, String> {
        return match self.op {
            Operator::Add => Ok(self.left.calc()? + self.right.calc()?),
            Operator::Sub => Ok(self.left.calc()? - self.right.calc()?),
            Operator::Mul => Ok(self.left.calc()? * self.right.calc()?),
            Operator::Div => Ok(self.left.calc()? / self.right.calc()?),
        }
    }
}

impl Calculable for With {
    fn calc(&self) -> Result<f32, String> {
        self.input.calc()
    }
}

impl Calculable for Id {
    fn calc(&self) -> Result<f32, String> {
        Err(format!("failed to replace identifier: {}", self.val))
    }
}
