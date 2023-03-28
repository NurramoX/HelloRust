// reverse_words.rs
pub fn reverse_words(s: &str) -> String {
    let help: String = s.chars().rev().collect();
    let help2: Vec<String> = help.split(' ').rev().map(|x| x.to_string()).collect();
    help2.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_reverse_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(reverse_words(input), expected);
    }

    #[test]
    fn test_reverse_single_character() {
        let input = "a";
        let expected = "a";
        assert_eq!(reverse_words(input), expected);
    }

    #[test]
    fn test_reverse_multiple_characters() {
        let input = "hello";
        let expected = "olleh";
        assert_eq!(reverse_words(input), expected);
    }

    #[test]
    fn test_reverse_multiple_words() {
        let input = "hello world";
        let expected = "olleh dlrow";
        assert_eq!(reverse_words(input), expected);
    }
}
