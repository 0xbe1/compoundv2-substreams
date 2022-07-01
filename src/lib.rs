#[rustfmt::skip]
mod abi;
#[rustfmt::skip]
mod pb;
mod rpc;
mod utils;

use crate::utils::address_pretty;
use hex_literal::hex;
use num_bigint::BigUint;
use pb::compound;
use substreams::{log, proto, store, Hex};
use substreams_ethereum::pb::eth::v1 as eth;
use substreams_ethereum::NULL_ADDRESS;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_accrue_interest(
    blk: eth::Block,
) -> Result<compound::AccrueInterestList, substreams::errors::Error> {
    let mut accrue_interest_list: Vec<compound::AccrueInterest> = vec![];
    for trx in blk.transaction_traces {
        accrue_interest_list.extend(trx.receipt.unwrap().logs.iter().filter_map(|log| {
            // TODO: filter by market id

            if !abi::ctoken::events::AccrueInterest::match_log(log) {
                return None;
            }

            let accrue_interest = abi::ctoken::events::AccrueInterest::must_decode(log);

            Some(compound::AccrueInterest {
                meta: Some(compound::EventMeta {
                    address: log.address.clone(),
                    txn_hash: trx.hash.clone(),
                    log_index: log.index,
                    block_number: blk.number,
                    block_timestamp: blk
                        .header
                        .as_ref()
                        .unwrap()
                        .timestamp
                        .as_ref()
                        .unwrap()
                        .seconds,
                }),
                interest_accumulated: accrue_interest.interest_accumulated.to_string(),
                borrow_index: accrue_interest.borrow_index.to_string(),
                total_borrows: accrue_interest.total_borrows.to_string(),
            })
        }));
    }

    Ok(compound::AccrueInterestList {
        accrue_interest_list,
    })
}

#[substreams::handlers::map]
fn map_mint(
    blk: eth::Block,
    input: store::StoreGet,
) -> Result<compound::MintList, substreams::errors::Error> {
    let mut mint_list = compound::MintList { mint_list: vec![] };
    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs.iter() {
            if !abi::ctoken::events::Mint::match_log(log) {
                continue;
            }

            let mint_event = abi::ctoken::events::Mint::must_decode(log);
            let mint = compound::Mint {
                id: format!("{}-{}", Hex::encode(&trx.hash), log.index),
                minter: mint_event.minter,
                mint_amount: mint_event.mint_amount.to_string(),
                mint_tokens: mint_event.mint_tokens.to_string(),
                // TODO: calculate amount usd, it is only price now
                mint_amount_usd: match input
                    .get_last(&format!("token:{}:price", address_pretty(&log.address)))
                {
                    None => BigUint::default(),
                    Some(price) => {
                        // log::debug!("price {}", BigUint::from_bytes_be(&price));
                        BigUint::from_bytes_be(&price)
                    }
                }
                .to_string(),
            };
            mint_list.mint_list.push(mint);
        }
    }
    Ok(mint_list)
}

#[substreams::handlers::store]
fn store_mint(mint_list: compound::MintList, output: store::StoreSet) {
    for mint in mint_list.mint_list {
        output.set(0, mint.id.clone(), &proto::encode(&mint).unwrap());
    }
}

#[substreams::handlers::store]
fn store_market(blk: eth::Block, output: store::StoreSet) {
    for trx in blk.transaction_traces {
        for call in trx.calls.iter() {
            for log in call.logs.iter() {
                if !abi::comptroller::events::MarketListed::match_log(log) {
                    continue;
                }
                let market_listed = abi::comptroller::events::MarketListed::must_decode(log);
                let ctoken_id = market_listed.c_token;
                let is_ceth =
                    ctoken_id == Hex::decode("4ddc2d193948926d02f9b1fe9e1daa0718270ed5").unwrap();
                let is_csai =
                    ctoken_id == Hex::decode("f5dce57282a584d2746faf1593d3121fcac444dc").unwrap();

                let ctoken_res = rpc::fetch_token(&ctoken_id);
                if ctoken_res.is_err() {
                    continue;
                }
                let ctoken = ctoken_res.unwrap();

                let underlying_token_id_res: Result<Vec<u8>, String> = if is_ceth {
                    Ok(NULL_ADDRESS.to_vec())
                } else if is_csai {
                    Ok(Hex::decode("89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap())
                } else {
                    rpc::fetch_underlying(&ctoken_id)
                };
                if underlying_token_id_res.is_err() {
                    continue;
                }
                let underlying_token_id = underlying_token_id_res.unwrap();

                let underlying_token_res = if is_ceth {
                    Ok(compound::Token {
                        id: address_pretty(&NULL_ADDRESS),
                        name: "Ether".to_string(),
                        symbol: "ETH".to_string(),
                        decimals: 18,
                    })
                } else if is_csai {
                    Ok(compound::Token {
                        id: address_pretty(&hex!("89d24a6b4ccb1b6faa2625fe562bdd9a23260359")),
                        name: "Sai Stablecoin v1.0 (SAI)".to_string(),
                        symbol: "SAI".to_string(),
                        decimals: 18,
                    })
                } else {
                    rpc::fetch_token(&underlying_token_id)
                };
                if underlying_token_res.is_err() {
                    continue;
                }
                let underlying_token = underlying_token_res.unwrap();

                let market = compound::Market {
                    id: ctoken.id.clone(),
                    name: ctoken.name.clone(),
                    input_token_id: underlying_token.id.clone(),
                    output_token_id: ctoken.id.clone(),
                };
                output.set(
                    0,
                    format!("token:{}", ctoken.id.clone()),
                    &proto::encode(&ctoken).unwrap(),
                );
                output.set(
                    0,
                    format!("token:{}", underlying_token.id.clone()),
                    &proto::encode(&underlying_token).unwrap(),
                );
                output.set(
                    0,
                    format!("market:{}", ctoken.id.clone()),
                    &proto::encode(&market).unwrap(),
                )
            }
        }
    }
}

#[substreams::handlers::store]
fn store_oracle(blk: eth::Block, output: store::StoreSet) {
    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs.iter() {
            if !abi::comptroller::events::NewPriceOracle::match_log(log) {
                continue;
            }

            let new_price_oracle = abi::comptroller::events::NewPriceOracle::must_decode(log);
            output.set(
                0,
                "protocol:oracle".to_string(),
                &new_price_oracle.new_price_oracle,
            );
        }
    }
}

#[substreams::handlers::store]
fn store_price(
    accrue_interest_list: compound::AccrueInterestList,
    input: store::StoreGet,
    output: store::StoreSet,
) {
    for accrue_interest in accrue_interest_list.accrue_interest_list {
        let market = &accrue_interest.meta.as_ref().unwrap().address;
        match input.get_last(&"protocol:oracle".to_string()) {
            None => continue,
            Some(oracle) => {
                let method = if accrue_interest.meta.as_ref().unwrap().block_number < 7710795 {
                    "getPrice(address)"
                } else {
                    "getUnderlyingPrice(address)"
                };
                let price_res = rpc::fetch_price(&oracle, method, &market);
                if price_res.is_err() {
                    log::info!(price_res.err().unwrap());
                    continue;
                }
                // log::debug!(format!("price {}", price_res.as_ref().unwrap()));
                output.set(
                    0,
                    format!("token:{}:price", address_pretty(&market)),
                    &price_res.unwrap().to_bytes_be(),
                )
            }
        }
    }
}
