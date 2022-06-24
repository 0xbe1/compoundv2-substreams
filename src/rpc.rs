use num_bigint::BigUint;
use substreams::Hex;
use substreams_ethereum::{pb::eth, rpc};

use crate::utils::rpc_data;
use crate::{
    pb::compound::Token,
    utils::{address_pretty, read_string, read_uint32},
};

pub fn fetch_token(addr: &Vec<u8>) -> Result<Token, String> {
    let rpc_calls = eth::rpc::RpcCalls {
        calls: vec![
            eth::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: rpc_data("decimals()", vec![]),
            },
            eth::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: rpc_data("name()", vec![]),
            },
            eth::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: rpc_data("symbol()", vec![]),
            },
        ],
    };

    let responses = rpc::eth_call(&rpc_calls).responses;
    if responses[0].failed || responses[1].failed || responses[2].failed {
        return Err(format!("contract {} eth_call failed", Hex(addr)));
    };

    let decoded_decimals = read_uint32(responses[0].raw.as_ref());
    if decoded_decimals.is_err() {
        return Err(format!(
            "({}).decimal() decode failed: {}",
            Hex(addr),
            decoded_decimals.err().unwrap()
        ));
    }

    let decoded_name = read_string(responses[1].raw.as_ref());
    if decoded_name.is_err() {
        return Err(format!(
            "({}).name() decode failed: {}",
            Hex(addr),
            decoded_name.err().unwrap()
        ));
    }

    let decoded_symbol = read_string(responses[2].raw.as_ref());
    if decoded_symbol.is_err() {
        return Err(format!(
            "({}).symbol() decode failed: {}",
            Hex(addr),
            decoded_symbol.err().unwrap()
        ));
    }

    return Ok(Token {
        id: address_pretty(addr),
        name: decoded_name.unwrap(),
        symbol: decoded_symbol.unwrap(),
        decimals: decoded_decimals.unwrap() as u64,
    });
}

pub fn fetch_underlying(addr: &Vec<u8>) -> Result<Vec<u8>, String> {
    let rpc_calls = eth::rpc::RpcCalls {
        calls: vec![eth::rpc::RpcCall {
            to_addr: Vec::from(addr.clone()),
            method_signature: rpc_data("underlying()", vec![]),
        }],
    };
    let responses = rpc::eth_call(&rpc_calls).responses;
    if responses[0].failed {
        return Err(format!("contract {} eth_call failed", Hex(addr)));
    };
    return Ok(responses[0].raw[12..32].to_vec());
}

pub fn fetch_price(oracle: &Vec<u8>, method: &str, market: &Vec<u8>) -> Result<BigUint, String> {
    let rpc_calls = eth::rpc::RpcCalls {
        calls: vec![eth::rpc::RpcCall {
            to_addr: Vec::from(oracle.clone()),
            method_signature: rpc_data(method, vec![market]),
        }],
    };

    let responses = rpc::eth_call(&rpc_calls).responses;
    if responses[0].failed {
        return Err(format!("contract {} eth_call failed", Hex(oracle)));
    };
    return Ok(BigUint::from_bytes_be(responses[0].raw.as_ref()));
}
