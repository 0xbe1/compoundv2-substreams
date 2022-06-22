use substreams::Hex;
use substreams_ethereum::{pb::eth, rpc};

use crate::{
    pb::compound::Token,
    utils::{address_pretty, decode_string, decode_uint32},
};

// TODO: return Result

pub fn fetch_token(addr: &Vec<u8>) -> Token {
    let decimals = Hex::decode("313ce567").unwrap();
    let name = Hex::decode("06fdde03").unwrap();
    let symbol = Hex::decode("95d89b41").unwrap();
    let rpc_calls = eth::rpc::RpcCalls {
        calls: vec![
            eth::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: decimals,
            },
            eth::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: name,
            },
            eth::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: symbol,
            },
        ],
    };

    let responses = rpc::eth_call(&rpc_calls).responses;

    if responses[0].failed || responses[1].failed || responses[2].failed {
        panic!("not a token because of a failure: {}", address_pretty(addr))
    };

    if responses[0].raw.len() != 32 || responses[1].raw.len() < 96 || responses[2].raw.len() < 96 {
        panic!(
            "not a token because response length: {}",
            address_pretty(addr),
        )
    };

    let decoded_decimals = decode_uint32(responses[0].raw.as_ref());
    let decoded_name = decode_string(responses[1].raw.as_ref());
    let decoded_symbol = decode_string(responses[2].raw.as_ref());

    Token {
        id: address_pretty(addr),
        name: decoded_name,
        symbol: decoded_symbol,
        decimals: decoded_decimals as u64,
    }
}

pub fn fetch_underlying(addr: &Vec<u8>) -> Vec<u8> {
    let underlying = Hex::decode("6f307dc3").unwrap();
    let rpc_calls = eth::rpc::RpcCalls {
        calls: vec![eth::rpc::RpcCall {
            to_addr: Vec::from(addr.clone()),
            method_signature: underlying,
        }],
    };

    let responses = rpc::eth_call(&rpc_calls).responses;

    if responses[0].failed {
        panic!("not a token because of a failure: {}", address_pretty(addr))
    };

    if responses[0].raw.len() != 32 {
        panic!(
            "not a token because response length: {}",
            address_pretty(addr)
        )
    };
    return responses[0].raw[12..32].to_vec();
}
