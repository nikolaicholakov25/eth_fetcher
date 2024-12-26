use std::io::Error;

use hex::FromHex;
use rlp::{Rlp, RlpStream};

pub async fn decode_rlp_encoded_list(rlp_hex: &String) -> Result<Vec<String>, Error> {
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

pub async fn encode_hexes_to_rlp(hashes: &Vec<String>) -> String {
    // testing
    // let hashes = vec![
    //     "0xc44abbcefbf85565ec0fa893e3369c513128aa73b30e0926fa0a5825fbfe1fe7",
    //     "0xa13acefb7e2dbe7343f89bdcc08b60f71d77bceeab68903468921fedc899265a",
    // ];
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
            println!("RLP encoding error");
        }
    }

    // Encode the RLP stream into bytes
    let rlp_bytes = stream.out();

    hex::encode(rlp_bytes)
}
