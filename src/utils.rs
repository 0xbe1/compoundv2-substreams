use substreams::Hex;
use tiny_keccak::{Hasher, Keccak};

pub fn address_pretty(input: &[u8]) -> String {
    format!("0x{}", Hex::encode(input))
}

pub fn decode_uint32(input: &[u8]) -> u32 {
    let as_array: [u8; 4] = input[28..32].try_into().unwrap();
    u32::from_be_bytes(as_array)
}

pub fn decode_string(input: &[u8]) -> String {
    if input.len() < 96 {
        panic!("input length too small: {}", input.len());
    }

    let next = decode_uint32(&input[0..32]);
    if next != 32 {
        panic!("invalid input, first part should be 32");
    };

    let size: usize = decode_uint32(&input[32..64]) as usize;
    let end: usize = (size) + 64;

    if end > input.len() {
        panic!(
            "invalid input: end {:?}, length: {:?}, next: {:?}, size: {:?}, whole: {:?}",
            end,
            input.len(),
            next,
            size,
            Hex::encode(&input[32..64])
        );
    }

    String::from_utf8_lossy(&input[64..end]).to_string()
}

// "name()" -> "06fdde03"
// Same effect as: printf "name()" | keccak256 --no-0x | cut -c 1-8
pub fn method_signature(method: &str) -> Vec<u8> {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(&Vec::from(method));
    keccak.finalize(&mut output);
    return output[..8].to_vec();
}