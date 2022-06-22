#[rustfmt::skip]
mod abi;
#[rustfmt::skip]
mod pb;
use hex_literal::hex;
use pb::compound;
use substreams::{store, Hex};
use substreams_ethereum::{pb::eth::v1 as eth};

const COMPTROLLER_CONTRACT: [u8; 20] = hex!("3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_market_listed(blk: eth::Block) -> Result<compound::MarketListedList, substreams::errors::Error> {
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
                ctoken: market_listed.c_token
            })
        }));
    }

    Ok(compound::MarketListedList { market_listed_list })
}

#[substreams::handlers::store]
fn store_market_listed(market_listed_list: compound::MarketListedList, s: store::StoreSet) {
    for market_listed in market_listed_list.market_listed_list {
        s.set(1, format!("market:{}", Hex(market_listed.ctoken)), &Vec::from("1"));
    }
}
