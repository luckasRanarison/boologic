pub fn num_to_bool(n: u32, size: usize) -> Vec<bool> {
    (0..32)
        .map(|i| (n >> i) & 1 == 1)
        .rev()
        .skip(32 - size)
        .collect()
}
