pub fn num_to_bool(n: u64, size: usize) -> Vec<bool> {
    (0..64)
        .map(|i| (n >> i) & 1 == 1)
        .rev()
        .skip(64 - size)
        .collect()
}
