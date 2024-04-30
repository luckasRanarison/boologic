use boologic::{ast::Expr, env::Environment, eval::Eval, parser::parse, utils::num_to_bool};
use std::{
    collections::{HashMap, HashSet},
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        io::stdout().write_all(b"> ")?;
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        match parse(&buffer) {
            Ok(root) => print_table(&buffer, root),
            Err(err) => eprintln!("Error: {err}"),
        };

        buffer.clear();
    }
}

fn print_table(source: &str, root: Expr) {
    let variables = source
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .collect::<HashSet<char>>();
    let variable_count = variables.len();
    let cases = 2u32.pow(variable_count as u32);
    let mut results = HashMap::<String, Vec<bool>>::new();

    for i in 0..cases {
        let booleans = num_to_bool(i, variable_count);
        let values = variables.iter().zip(booleans).map(|(&v, b)| (v, b));
        let mut env = Environment::new(values.collect());
        let _ = root.eval(&mut env);

        for (expr, value) in env.into_results() {
            if let Some(values) = results.get_mut(&expr) {
                values.push(value);
            } else {
                results.insert(expr, vec![value]);
            }
        }
    }

    let mut labels = results.keys().collect::<Vec<_>>();

    labels.sort_by(|a, b| match (a.len(), b.len()) {
        (1, 1) => a.cmp(b),  // alphabetic order
        (a, b) => a.cmp(&b), // operation length order
    });

    for i in 0..cases + 1 {
        for (j, label) in labels.iter().enumerate() {
            let width = label.chars().count() + 1;
            let text = match i > 0 {
                true => (results[*label][i as usize - 1] as u8).to_string(),
                false => label.to_string(),
            };
            print!("{}{text:<width$}", if j > 0 { " | " } else { "  " });
        }
        println!();
    }
}
