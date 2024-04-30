use crate::{ast::Expr, env::Environment};

pub trait Eval {
    fn eval(&self, env: &mut Environment) -> bool;
}

impl Eval for Expr {
    fn eval(&self, env: &mut Environment) -> bool {
        let hash = self.to_string();

        if let Some(result) = env.get_result(&hash) {
            return result;
        }

        let result = match self {
            Expr::Or(lhs, rhs)
            | Expr::And(lhs, rhs)
            | Expr::Impl(lhs, rhs)
            | Expr::Eq(lhs, rhs) => {
                // non-lazy evaluation
                let lhs = lhs.eval(env);
                let rhs = rhs.eval(env);

                match self {
                    Expr::Or(_, _) => lhs || rhs,
                    Expr::And(_, _) => lhs && rhs,
                    Expr::Impl(_, _) => !lhs || rhs,
                    _ => lhs == rhs,
                }
            }
            Expr::Not(op) => !op.eval(env),
            Expr::Var(name) => env.get_variable(name),
        };

        env.insert(hash, result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Eval;
    use crate::{env::Environment, parser::parse};
    use std::collections::HashMap;

    #[test]
    fn test_eval() {
        let expr = "p | q";
        let root = parse(expr).unwrap();
        let variables = HashMap::from([('p', true), ('q', false)]);
        let mut env = Environment::new(variables);
        let _ = root.eval(&mut env);
        let results = env.into_results().collect::<HashMap<_, _>>();

        assert_eq!(true, results["p"]);
        assert_eq!(false, results["q"]);
        assert_eq!(true, results["(p ∨ q)"]);
    }
}
