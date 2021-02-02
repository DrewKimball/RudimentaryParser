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

// Expr is a node in an abstract syntax tree that represents an expression from
// the above grammar.
#[derive(Clone)]
pub enum Expr {
    Number(Box<Number>),
    Binary(Box<Binary>),
    With(Box<With>),
    Id(Box<Id>),
}

macro_rules! into_expr {
    ($id:ident) => {
        impl Into<Expr> for $id {
            fn into(self) -> Expr {
                Expr::$id(Box::new(self))
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Number {
    pub(crate) val: i32,
}

into_expr!(Number);

#[derive(Clone)]
pub struct Binary {
    pub(crate) op:    Operator,
    pub(crate) left:  Expr,
    pub(crate) right: Expr,
}

into_expr!(Binary);

#[derive(Copy, Clone)]
pub enum Operator { Add, Sub, Mul, Div }

#[derive(Clone)]
pub struct With {
    pub(crate) binding: Binding,
    pub(crate) input:   Expr,
}

into_expr!(With);

#[derive(Clone)]
pub struct Binding {
    pub(crate) identifier: Box<Id>,
    pub(crate) replace:    Expr,
}

#[derive(Clone)]
pub struct Id {
    pub(crate) val: String,
}

into_expr!(Id);
