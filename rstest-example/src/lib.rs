pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 2, 4)]
    #[case(2, 3, 5)]
    fn it_works(#[case] left: u64, #[case] right: u64, #[case] expected: u64) {
        let result = add(left, right);
        assert_eq!(result, expected, "left: {}, right: {}", left, right);
    }
}
