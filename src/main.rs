use boologic::{
    parse,
    utils::{compare_labels, num_to_bool},
    Environment, Eval, Expr,
};
use std::{
    collections::{HashMap, HashSet},
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Boologic REPL, Enter 'help' to see usage");

    loop {
        io::stdout().write_all(b"> ")?;
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        let (cmd, arg) = buffer.split_once(' ').unwrap_or((&buffer.trim_end(), ""));

        match cmd {
            "" => {}
            "exit" => break,
            "help" => print_help(),
            "clear" => clear_screen(),
            "table" => parse_then(arg, print_table),
            _ => println!("Error: Invalid command"),
        }

        buffer.clear();
    }

    Ok(())
}

#[allow(clippy::print_literal)]
fn print_help() {
    println!(
        "{}\n{}\n{}",
        "table <expr>     Print the truth table of a given expression",
        "clear            Clear the screen",
        "exit             Exit the program"
    );
}

fn clear_screen() {
    print!("\x1b[2J\x1b[0;0H") // clear + moveto 0,0
}

fn parse_then<F>(source: &str, callback: F)
where
    F: Fn(Expr),
{
    match parse(source) {
        Ok(root) => callback(root),
        Err(err) => println!("Error: {err}"),
    }
}

fn print_table(root: Expr) {
    let (cases, results) = get_results(root);
    let mut labels = results.keys().collect::<Vec<_>>();

    labels.sort_by(|a, b| compare_labels(a, b));

    for i in 0..cases + 1 {
        for (j, label) in labels.iter().enumerate() {
            let start = if j > 0 { " | " } else { "  " };
            let text = match i > 0 {
                true => (results[*label][i as usize - 1] as u8).to_string(),
                false => label.to_string(),
            };
            let width = label.chars().count() + 1;
            let end = if j == labels.len() - 1 { "\n" } else { "" };

            print!("{start}{text:<width$}{end}");
        }
    }
}

fn get_results(root: Expr) -> (u32, HashMap<String, Vec<bool>>) {
    let variables = root
        .to_string()
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

    (cases, results)
}
