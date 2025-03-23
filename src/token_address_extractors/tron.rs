use regex::Regex;

pub fn extract_tron_address(text: &str) -> Option<String> {
    let pattern = Regex::new(r"\bT[a-zA-Z0-9]{33}\b").unwrap();
    pattern.find(text).map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_tron_address() {
        let input = "
            Some random text
            TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7
            more random text
        ";

        let expected_address = Some("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string());

        let result = extract_tron_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "This is a test with no valid Tron addresses!";
        let expected_address = None;

        let result = extract_tron_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_multiple_addresses() {
        let input = "
            TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7
            TPL66VynYgJoq4AuirFx9pPJTtL2PdRsy4
        ";

        // The function should return only the first valid Tron address found.
        let expected_address = Some("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string());

        let result = extract_tron_address(input);
        assert_eq!(result, expected_address);
    }
}
