use std::io::Error;

use hex::FromHex;
use rlp::{Rlp, RlpStream};

pub fn decode_rlp_encoded_list(rlp_hex: &String) -> Result<Vec<String>, Error> {
    let mut decoded_hashes: Vec<String> = vec![];

    // Convert the hex string to bytes
    let rlp_bytes = Vec::from_hex(rlp_hex).expect("Invalid hex string");

    // Parse the RLP
    let rlp = Rlp::new(&rlp_bytes);

    // Decode the list of transaction hashes
    let transaction_hashes: Vec<Vec<u8>> = rlp.as_list().expect("Failed to decode RLP list");

    // Print the transaction hashes as hex strings
    for (_, hash) in transaction_hashes.iter().enumerate() {
        decoded_hashes.push(hex::encode(hash));
    }

    Ok(decoded_hashes)
}

// used only for testing
#[allow(dead_code)]
pub fn encode_hexes_to_rlp(hashes: &Vec<String>) -> Result<String, String> {
    // Initialize an RLP stream
    let mut stream = RlpStream::new_list(hashes.len());

    // Iterate through each hash, validate, and append to the RLP stream
    for hash in hashes {
        // Remove the "0x" prefix if present
        let hash_trimmed = hash.trim_start_matches("0x");

        // Convert the hash string to bytes
        if let Ok(bytes) = Vec::from_hex(&hash_trimmed) {
            stream.append(&bytes);
        } else {
            return Err("Unable to encode bytes".to_string());
        }
    }

    // Encode the RLP stream into bytes
    let rlp_bytes = stream.out();

    Ok(hex::encode(rlp_bytes))
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::utils::rlp::decode_rlp_encoded_list;

    use super::encode_hexes_to_rlp;
    static SEPOLIA_HASHES: LazyLock<Vec<String>> = LazyLock::new(|| {
        vec![
            "0x8e5484577d7f6bc0dd7d6a7016a55e3e33a43ece50c4c11aad074b3d728a8d35".to_string(),
            "0x7addeb71d33c4824e31b30d92894e0e1d2e0c0a13d8e1020aaad80d5b3ee32ec".to_string(),
            "0x031b20239d55bee927ab7bc0510748628438a6d08dccffbb0da61f3b72bc71ed".to_string(),
        ]
    });

    #[test]
    fn test_rlp_encode() {
        // encoding
        let rlp_hex = encode_hexes_to_rlp(&SEPOLIA_HASHES);
        assert!(rlp_hex.is_ok());
    }

    #[test]
    fn test_rlp_decode() {
        // encoding
        let rlp_hex = encode_hexes_to_rlp(&SEPOLIA_HASHES);
        assert!(rlp_hex.is_ok());

        // decoding
        let decoded_rlp = decode_rlp_encoded_list(&rlp_hex.unwrap());
        assert!(decoded_rlp.is_ok());

        let decoded_result: Vec<String> = decoded_rlp
            .unwrap()
            .iter_mut()
            .map(|hex| format!("0x{}", hex))
            .collect();

        for i in 0..decoded_result.len() {
            assert_eq!(
                decoded_result.get(i).unwrap(),
                SEPOLIA_HASHES.get(i).unwrap()
            );
        }
    }
}
