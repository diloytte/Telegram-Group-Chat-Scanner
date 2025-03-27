use regex::Regex;

pub fn extract_ethereum_based_address(text: &str) -> Option<String> {
    let pattern = Regex::new(r"\b0x[a-fA-F0-9]{40}\b").unwrap();
    pattern.find(text).map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ethereum_address() {
        let input = "
            Some random text
            0xAb5801a7D398351b8bE11C439e05C5b3259aec9B
            more random text
        ";

        let expected_address = Some("0xAb5801a7D398351b8bE11C439e05C5b3259aec9B".to_string());

        let result = extract_ethereum_based_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "This is a test with no valid Ethereum addresses!";
        let expected_address = None;

        let result = extract_ethereum_based_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_link() {
        let input = "https://dexscreener.com/bsc/0x2c46eb9820f048eeba1ce7b1dcfd302916284dad";
        let expected_address = Some("0x2c46eb9820f048eeba1ce7b1dcfd302916284dad".to_string());

        let result = extract_ethereum_based_address(input);
        assert_eq!(expected_address, result);
    }

    #[test]
    fn test_multiple_addresses() {
        let input = "
            0xAb5801a7D398351b8bE11C439e05C5b3259aec9B
            0x4E9ce36E442e55EcD9025B9a6E0D88485d628A67
        ";

        // The function should return only the first valid Ethereum address found.
        let expected_address = Some("0xAb5801a7D398351b8bE11C439e05C5b3259aec9B".to_string());

        let result = extract_ethereum_based_address(input);
        assert_eq!(result, expected_address);
    }
}
