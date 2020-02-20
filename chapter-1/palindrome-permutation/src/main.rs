use std::collections::HashSet;

fn main() {}

fn palindrome_permutation(s: &str) -> bool {
    let mut odd_letters = HashSet::<char>::new();
    for letter in s
        .chars()
        .filter(|letter| letter.is_alphabetic())
        .map(|letter| letter.to_ascii_lowercase())
    {
        if odd_letters.contains(&letter) {
            odd_letters.remove(&letter);
        } else {
            odd_letters.insert(letter);
        }
    }

    odd_letters.len() <= 1
}

mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(palindrome_permutation("racecar"));
    }

    #[test]
    fn test_empty() {
        assert!(palindrome_permutation(""));
    }

    #[test]
    fn test_single_letter() {
        assert!(palindrome_permutation("I"));
    }

    #[test]
    fn test_non_example() {
        assert!(!palindrome_permutation("Suck it, grass!"));
    }

    #[test]
    fn test_case_insensitivity() {
        assert!(palindrome_permutation("F f"));
    }

    #[test]
    fn test_permutation() {
        assert!(palindrome_permutation("TactCoa"));
    }

    #[test]
    fn test_permutation_with_whitespace() {
        assert!(palindrome_permutation("Tact Coa"));
    }
    
    #[test]
    fn test_whitespace_only() {
        assert!(palindrome_permutation("    \t\n\n\r"));
    }
    
    #[test]
    fn test_palindrome_with_whitespace() {
        assert!(palindrome_permutation("A man, a plan, a canal, Panama!"));
    }
}
