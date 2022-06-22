#[rustfmt::skip]
mod abi;
#[rustfmt::skip]
mod pb;
mod rpc;
mod utils;

use hex_literal::hex;
use pb::compound;
use substreams::{proto, store, Hex};
use substreams_ethereum::pb::eth::v1 as eth;

const COMPTROLLER_CONTRACT: [u8; 20] = hex!("3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B");

substreams_ethereum::init!();

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
                trx_hash: trx.hash.clone(),
                ordinal: log.block_index as u64,
                ctoken: market_listed.c_token,
            })
        }));
    }

    Ok(compound::MarketListedList { market_listed_list })
}

#[substreams::handlers::store]
fn store_token(market_listed_list: compound::MarketListedList, s: store::StoreSet) {
    for market_listed in market_listed_list.market_listed_list {
        let ctoken_id = market_listed.ctoken;
        let ctoken = rpc::fetch_token(&ctoken_id);
        // let underlying_token_id = rpc::fetch_underlying(&ctoken_id);
        // let underlying_token = rpc::fetch_token(&underlying_token_id);
        s.set(
            0,
            format!("token:{}", Hex(&ctoken_id)),
            &proto::encode(&ctoken).unwrap(),
        );
        // s.set(0, format!("token:{}", Hex(&underlying_token_id)), &proto::encode(&underlying_token).unwrap())
    }
}
