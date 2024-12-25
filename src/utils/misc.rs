use alloy::primitives::B256;
use hex::FromHex;
use serde::Deserialize;

pub fn parse_b256_from_str(hash_str: &str) -> Result<B256, String> {
    // remove hex prefix
    let trimmed = hash_str.trim_start_matches("0x");
    // turn into bytes
    let bytes = <Vec<u8>>::from_hex(trimmed).map_err(|e| format!("Invalid hex: {}", e))?;
    // ensure 32 bytes length
    let array: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "Hash must be 32 bytes".to_string())?;

    // wrap in `B256`
    Ok(B256::new(array))
}

pub fn comma_separated_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',').map(|item| item.trim().to_string()).collect())
}
