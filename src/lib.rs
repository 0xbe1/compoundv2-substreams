#[rustfmt::skip]
mod abi;
#[rustfmt::skip]
mod pb;
mod rpc;
mod utils;

use crate::utils::address_pretty;
use hex_literal::hex;
use pb::compound;
use substreams::{proto, store, Hex};
use substreams_ethereum::pb::eth::v1 as eth;
use substreams_ethereum::NULL_ADDRESS;

const COMPTROLLER_CONTRACT: [u8; 20] = hex!("3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B");

substreams_ethereum::init!();

// TODO: use ethtokens
#[substreams::handlers::map]
fn map_market_listed(
    blk: eth::Block,
) -> Result<compound::MarketListedList, substreams::errors::Error> {
    let mut market_listed_list: Vec<compound::MarketListed> = vec![];
    for trx in blk.transaction_traces {
        market_listed_list.extend(trx.receipt.unwrap().logs.iter().filter_map(|log| {
            if log.address != COMPTROLLER_CONTRACT {
                return None;
            }

            if !abi::comptroller::events::MarketListed::match_log(log) {
                return None;
            }

            let market_listed = abi::comptroller::events::MarketListed::must_decode(log);

            Some(compound::MarketListed {
                address: log.address.clone(),
                trx_hash: trx.hash.clone(),
                block_number: blk.number,
                ctoken: market_listed.c_token,
            })
        }));
    }

    Ok(compound::MarketListedList { market_listed_list })
}

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
                address: log.address.clone(),
                trx_hash: trx.hash.clone(),
                block_number: blk.number,
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

#[substreams::handlers::store]
fn store_market(market_listed_list: compound::MarketListedList, output: store::StoreSet) {
    for market_listed in market_listed_list.market_listed_list {
        let ctoken_id = market_listed.ctoken;
        let is_ceth = ctoken_id == Hex::decode("4ddc2d193948926d02f9b1fe9e1daa0718270ed5").unwrap();
        let is_csai = ctoken_id == Hex::decode("f5dce57282a584d2746faf1593d3121fcac444dc").unwrap();

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

#[substreams::handlers::store]
fn store_price_oracle(blk: eth::Block, output: store::StoreSet) {
    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs.iter() {
            if log.address != COMPTROLLER_CONTRACT {
                continue;
            }

            if !abi::comptroller::events::NewPriceOracle::match_log(log) {
                continue;
            }

            let new_price_oracle = abi::comptroller::events::NewPriceOracle::must_decode(log);
            output.set(
                log.block_index as u64,
                "protocol:price_oracle".to_string(),
                &new_price_oracle.new_price_oracle,
            );
        }
    }
}
