use crate::{
    ast::Expr,
    error::{Error, Result},
};

pub fn parse(input: &str) -> Result<Expr> {
    let (root, rest) = parse_or(input)?;

    if !rest.is_empty() {
        Err(Error::InvalidToken(rest.to_owned()))
    } else {
        Ok(root)
    }
}

fn parse_or(input: &str) -> Result<(Expr, &str)> {
    let (lhs, rest) = parse_and(input)?;

    if let Ok(rest) = seq("|", rest) {
        let (rhs, rest) = parse_or(rest)?;
        let node = Expr::Or(Box::new(lhs), Box::new(rhs));
        Ok((node, rest))
    } else {
        Ok((lhs, rest))
    }
}

fn parse_and(input: &str) -> Result<(Expr, &str)> {
    let (lhs, rest) = parse_imp(input)?;

    if let Ok(rest) = seq("&", rest) {
        let (rhs, rest) = parse_and(rest)?;
        let node = Expr::And(Box::new(lhs), Box::new(rhs));
        Ok((node, rest))
    } else {
        Ok((lhs, rest))
    }
}

fn parse_imp(input: &str) -> Result<(Expr, &str)> {
    let (lhs, rest) = parse_eq(input)?;

    if let Ok(rest) = seq("->", rest) {
        let (rhs, rest) = parse_imp(rest)?;
        let node = Expr::Impl(Box::new(lhs), Box::new(rhs));
        Ok((node, rest))
    } else {
        Ok((lhs, rest))
    }
}

fn parse_eq(input: &str) -> Result<(Expr, &str)> {
    let (lhs, rest) = parse_not(input)?;

    if let Ok(rest) = seq("==", rest) {
        let (rhs, rest) = parse_eq(rest)?;
        let node = Expr::Eq(Box::new(lhs), Box::new(rhs));
        Ok((node, rest))
    } else {
        Ok((lhs, rest))
    }
}

fn parse_not(input: &str) -> Result<(Expr, &str)> {
    let rest = input.trim_start();

    if let Ok(rest) = seq("!", rest) {
        let (node, rest) = parse_primary(rest)?;
        let node = Expr::Not(Box::new(node));
        Ok((node, rest))
    } else {
        parse_primary(input)
    }
}

fn parse_primary(input: &str) -> Result<(Expr, &str)> {
    let rest = input.trim_start();
    let (node, rest) = parse_group(rest).or(parse_variable(rest))?;
    let rest = rest.trim_start();
    Ok((node, rest))
}

fn parse_group(input: &str) -> Result<(Expr, &str)> {
    let rest = seq("(", input)?;
    let (node, rest) = parse_or(&rest)?;
    let rest = seq(")", rest)?;
    Ok((node, rest))
}

fn parse_variable(input: &str) -> Result<(Expr, &str)> {
    let ch = input.chars().next().ok_or(Error::UnexpectedEOF)?;
    let node = ch
        .is_alphabetic()
        .then_some(Expr::Var(ch))
        .ok_or(Error::InvalidToken(ch.to_string()))?;
    Ok((node, &input[1..]))
}

fn seq<'a>(sub: &str, input: &'a str) -> Result<&'a str> {
    input
        .starts_with(sub)
        .then(|| &input[sub.len()..]) // make it lazy to avoid out of bound indexing
        .ok_or(Error::ExpectedSequence(sub.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::{ast::Expr, parser::parse};

    #[test]
    fn test_parsing() {
        let source = "(p -> !q) & q";
        let expected = Expr::And(
            Box::new(Expr::Impl(
                Box::new(Expr::Var('p')),
                Box::new(Expr::Not(Box::new(Expr::Var('q')))),
            )),
            Box::new(Expr::Var('q')),
        );

        assert_eq!(expected, parse(source).unwrap());
    }
}
