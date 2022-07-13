use bigdecimal::{BigDecimal, One};
use num_bigint::BigUint;
use std::ops::Mul;
use std::str;
use std::str::FromStr;
use substreams::Hex;
use tiny_keccak::{Hasher, Keccak};

pub fn read_uint32(input: &[u8]) -> Result<u32, String> {
    if input.len() != 32 {
        return Err(format!("uint32 invalid length: {}", input.len()));
    }
    let as_array: [u8; 4] = input[28..32].try_into().unwrap();
    Ok(u32::from_be_bytes(as_array))
}

pub fn read_string(input: &[u8]) -> Result<String, String> {
    if input.len() < 96 {
        return Err(format!("string invalid length: {}", input.len()));
    }

    let next = read_uint32(&input[0..32])?;
    if next != 32 {
        return Err(format!("invalid string uint32 value: {}", next));
    };

    let size = read_uint32(&input[32..64])?;
    let end: usize = (size as usize) + 64;

    if end > input.len() {
        return Err(format!(
            "invalid input: end {:?}, length: {:?}, next: {:?}, size: {:?}, whole: {:?}",
            end,
            input.len(),
            next,
            size,
            Hex::encode(&input[32..64])
        ));
    }

    Ok(String::from_utf8_lossy(&input[64..end]).to_string())
}

pub fn string_to_bigdecimal(input: &[u8]) -> BigDecimal {
    return BigDecimal::from_str(str::from_utf8(input).unwrap()).unwrap();
}

pub fn bytes_to_bigdecimal(input: &[u8]) -> BigDecimal {
    return BigDecimal::from_str(&BigUint::from_bytes_be(input).to_string()).unwrap();
}

pub fn exponent_to_big_decimal(decimals: u64) -> BigDecimal {
    let mut result = BigDecimal::one();
    let big_decimal_ten: &BigDecimal = &BigDecimal::from(10 as u64);
    for _ in 0..decimals {
        result = result.mul(big_decimal_ten);
    }
    return result;
}

// Construct rpc data according to https://docs.soliditylang.org/en/develop/abi-spec.html#examples
pub fn rpc_data(method: &str, args: &Vec<Vec<u8>>) -> Vec<u8> {
    let method_sig = method_signature(method);
    if args.len() == 0 {
        return method_sig;
    }
    let mut data = Vec::with_capacity(8 + args.len() * 32);
    data.extend(method_sig);
    for arg in args {
        data.extend(vec![0u8].repeat(32 - arg.len()));
        data.extend(arg);
    }
    return data;
}

// "name()" -> "06fdde03"
// Same effect as: printf "name()" | keccak256 --no-0x | cut -c 1-8
fn method_signature(method: &str) -> Vec<u8> {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(&Vec::from(method));
    keccak.finalize(&mut output);
    return output[..4].to_vec();
}
