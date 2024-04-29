use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<char, bool>,
    results: HashMap<String, bool>,
}

impl Environment {
    pub fn new(variables: HashMap<char, bool>) -> Self {
        Self {
            variables,
            results: HashMap::new(),
        }
    }

    pub fn get(&self, name: &char) -> bool {
        self.variables[name]
    }

    pub fn insert(&mut self, expr: String, value: bool) {
        self.results.insert(expr, value);
    }

    pub fn into_results(self) -> impl Iterator<Item = (String, bool)> {
        self.results.into_iter()
    }
}
