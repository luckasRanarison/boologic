pub fn num_to_bool(n: u32, size: usize) -> Vec<bool> {
    (0..32)
        .map(|i| (n >> i) & 1 == 1)
        .rev()
        .skip(32 - size)
        .collect()
}

pub fn compare_labels(a: &str, b: &str) -> std::cmp::Ordering {
    match (a.len(), b.len()) {
        (1, 1) => a.cmp(b),  // alphabetic order
        (a, b) => a.cmp(&b), // operation length order
    }
}
