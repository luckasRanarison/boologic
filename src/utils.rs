use std::cmp::Ordering;

pub fn num_to_bool(n: u32, size: usize) -> Vec<bool> {
    (0..32)
        .map(|i| (n >> i) & 1 == 1)
        .rev()
        .skip(32 - size)
        .collect()
}

pub fn compare_labels(a: &str, b: &str) -> Ordering {
    match (a.len(), b.len()) {
        (1, 1) => a.cmp(b),  // alphabetic order
        (a, b) => a.cmp(&b), // operation length order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_bool() {
        let expected = vec![true, false, true, false];
        let result = num_to_bool(10, 4);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_label_comparaison() {
        let expected = Ordering::Less;
        let result = compare_labels("p", "q");
        assert_eq!(expected, result);

        let expected = Ordering::Greater;
        let result = compare_labels("p -> q", "q");
        assert_eq!(expected, result);
    }
}
