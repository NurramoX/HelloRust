fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().enumerate().map(|(seq, x)|
        format!("{}: {}", seq + 1, x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input: Vec<&str> = vec![];
        let expected: Vec<String> = vec![];
        assert_eq!(number(&input), expected);
    }

    #[test]
    fn test_single_line_input() {
        let input = vec!["Hello"];
        let expected = vec!["1: Hello"];
        assert_eq!(number(&input), expected);
    }

    #[test]
    fn test_multiple_lines_input() {
        let input = vec!["Hello", "World", "Rust", "is", "awesome!"];
        let expected = vec![
            "1: Hello",
            "2: World",
            "3: Rust",
            "4: is",
            "5: awesome!",
        ];
        assert_eq!(number(&input), expected);
    }

    #[test]
    fn test_blank_lines_input() {
        let input = vec!["", "", "Hello", "", "World"];
        let expected = vec![
            "1: ",
            "2: ",
            "3: Hello",
            "4: ",
            "5: World",
        ];
        assert_eq!(number(&input), expected);
    }
}
