use crate::{Expr, Binary, With, Binding, Id};

// A type that implements Substitutable can propagate or effect a With
// replacement.
pub(crate) trait Substitutable {
    type Substituted;

    // subst is recursively called on the input of a With in order to enact the
    // replacement described by the With.
    fn subst(self, binding: &Binding) -> Self::Substituted;

    // replace traverses the ast until a With expression is found, and then
    // calls subst.
    fn replace(self) -> Self::Substituted;
}

impl Substitutable for Expr {
    type Substituted = Expr;

    fn subst(self, binding: &Binding) -> Expr {
        return match self {
            Expr::Binary(expr) => expr.subst(binding),
            Expr::With(expr) => expr.subst(binding),
            Expr::Id(expr) if expr.should_replace(binding) => binding.replace.clone().into(),
            _ => self,
        }
    }

    fn replace(self) -> Expr {
        return match self {
            Expr::Binary(expr) => expr.replace(),
            Expr::With(expr) => expr.replace(),
            _ => self,
        }
    }
}

impl Substitutable for Binary {
    type Substituted = Expr;

    fn subst(self, binding: &Binding) -> Expr {
        let new_left: Expr = self.left.subst(binding);
        let new_right: Expr = self.right.subst(binding);
        Binary{ op: self.op, left: new_left, right: new_right }.into()
    }

    fn replace(self) -> Expr {
        let new_left: Expr = self.left.replace();
        let new_right: Expr = self.right.replace();
        Binary{ op: self.op, left: new_left, right: new_right }.into()
    }
}

impl Substitutable for With {
    type Substituted = Expr;

    fn subst(self, binding: &Binding) -> Expr {
        // First effect substitution on this With's binding using the given
        // binding:
        //   (With ([x 1] (With ([y (* x 2)]) (<expr>)))
        //   =>
        //   ((With ([x 1] (With ([y (* 1 2)]) (<expr>)))
        let subst_binding: &Binding = &self.binding.subst(binding);

        // Effect the replacement described by this With's binding.
        let first_replace: Expr = self.input.subst(subst_binding);

        // Now propagate the previous With replacement into the input. Replace
        // this With with its input.
        first_replace.subst(binding)
    }

    fn replace(self) -> Expr {
        self.input.subst(&self.binding.replace())
    }
}

impl Substitutable for Binding {
    type Substituted = Binding;

    fn subst(self, binding: &Binding) -> Binding {
        Binding{ identifier: self.identifier, replace: self.replace.subst(binding) }.into()
    }

    fn replace(self) -> Binding {
        Binding{ identifier: self.identifier, replace: self.replace.replace() }.into()
    }
}

impl Id {
    fn should_replace(&self, binding: &Binding) -> bool {
        self.val == binding.identifier.val
    }
}