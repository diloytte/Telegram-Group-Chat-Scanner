use ethereum::extract_ethereum_address;
use solana::extract_solana_address;
use tron::extract_tron_address;

mod ethereum;
mod solana;
mod tron;

//TODO: These do not full extract ERC20/SPL token address. It just extracts the "address". Fix required.

pub static FUNCTIONS: &[fn(&str) -> Option<String>] = &[
    extract_ethereum_address,
    extract_solana_address,
    extract_tron_address,
];

pub fn extract_token_address_from_message_text(text: &str) -> Option<String> {
    let mut final_token_address: Option<String> = None;
    let len = FUNCTIONS.iter().len();

    for i in 0..len {
        let extractor_function = FUNCTIONS.get(i);
        let extracted_address_option = extractor_function.unwrap()(text);
        final_token_address = extracted_address_option;

        if final_token_address.is_some() {
            break;
        }
    }
    
    final_token_address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ethereum_address() {
        let ethereum_text =
            "Check this Ethereum address 0x5f8F8E1dbB5bF65E3aF5F5dF8F8F8F8F8F8F8F8F test test test";
        let expected_address = Some("0x5f8F8E1dbB5bF65E3aF5F5dF8F8F8F8F8F8F8F8F".to_string());

        let result = extract_token_address_from_message_text(ethereum_text);

        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_extract_solana_address() {
        let solana_text =
            "Solana address: 3KZs8bDozYngwMY52VrggD2nzAeNGiDTzKpHWZ4gN1Fq test test test";
        let expected_address = Some("3KZs8bDozYngwMY52VrggD2nzAeNGiDTzKpHWZ4gN1Fq".to_string());

        let result = extract_token_address_from_message_text(solana_text);

        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_extract_tron_address() {
        let tron_text = "Tron address: TQ7r4rYjTnso3j7KwhZZfnZZG2eDZ6fvB7 test test test";
        let expected_address = Some("TQ7r4rYjTnso3j7KwhZZfnZZG2eDZ6fvB7".to_string());

        let result = extract_token_address_from_message_text(tron_text);

        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_extract_token_address_from_message_text_no_address() {
        let text = "No valid address here!";

        let result = extract_token_address_from_message_text(text);

        assert_eq!(result, None);
    }
}
