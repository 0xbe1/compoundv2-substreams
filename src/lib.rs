#[rustfmt::skip]
mod abi;
#[rustfmt::skip]
mod pb;
mod rpc;
mod utils;

use crate::utils::exponent_to_big_decimal;
use bigdecimal::BigDecimal;
use pb::compound;
use std::ops::{Div, Mul};
use std::str::FromStr;
use substreams::{proto, store, Hex};
use substreams_ethereum::pb::eth::v1 as eth;
use substreams_ethereum::NULL_ADDRESS;

#[substreams::handlers::map]
fn map_accrue_interest(
    blk: eth::Block,
) -> Result<compound::AccrueInterestList, substreams::errors::Error> {
    let mut accrue_interest_list: Vec<compound::AccrueInterest> = vec![];
    for trx in blk.transaction_traces {
        accrue_interest_list.extend(trx.receipt.unwrap().logs.iter().filter_map(|log| {
            if !abi::ctoken::events::AccrueInterest::match_log(log) {
                return None;
            }

            let accrue_interest = abi::ctoken::events::AccrueInterest::must_decode(log);

            Some(compound::AccrueInterest {
                interest_accumulated: accrue_interest.interest_accumulated.to_string(),
                borrow_index: accrue_interest.borrow_index.to_string(),
                total_borrows: accrue_interest.total_borrows.to_string(),
                address: log.address.clone(),
                block_number: blk.number,
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
    store_token: store::StoreGet,
    store_price: store::StoreGet,
) -> Result<compound::MintList, substreams::errors::Error> {
    let mut mint_list = compound::MintList { mint_list: vec![] };
    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs.iter() {
            if !abi::ctoken::events::Mint::match_log(log) {
                continue;
            }

            let market_address = &log.address;
            let mint_event = abi::ctoken::events::Mint::must_decode(log);
            let underlying_price_res = store_price.get_last(&format!(
                "market:{}:underlying:price",
                Hex::encode(market_address)
            ));
            let underlying_res = store_token.get_last(&format!(
                "market:{}:underlying",
                Hex::encode(market_address)
            ));
            if let (Some(underlying_token), Some(underlying_price)) =
                (underlying_res, underlying_price_res)
            {
                let price = utils::string_to_bigdecimal(underlying_price.as_ref());
                let underlying_token: compound::Token = proto::decode(&underlying_token).unwrap();
                let mint = compound::Mint {
                    id: format!("{}-{}", Hex::encode(&trx.hash), log.index).into_bytes(),
                    timestamp: blk
                        .header
                        .as_ref()
                        .unwrap()
                        .timestamp
                        .as_ref()
                        .unwrap()
                        .seconds,
                    minter: mint_event.minter,
                    mint_amount: mint_event.mint_amount.to_string(),
                    mint_tokens: mint_event.mint_tokens.to_string(),
                    mint_amount_usd: BigDecimal::from_str(
                        mint_event.mint_amount.to_string().as_str(),
                    )
                    .unwrap()
                    .div(utils::exponent_to_big_decimal(underlying_token.decimals))
                    .mul(price)
                    .to_string(),
                };
                mint_list.mint_list.push(mint);
            }
        }
    }
    Ok(mint_list)
}

#[substreams::handlers::map]
fn map_market_listed(
    blk: eth::Block,
) -> Result<compound::MarketListedList, substreams::errors::Error> {
    let mut market_listed_list = compound::MarketListedList {
        market_listed_list: vec![],
    };
    for trx in blk.transaction_traces {
        for call in trx.calls.iter() {
            for log in call.logs.iter() {
                if !abi::comptroller::events::MarketListed::match_log(log) {
                    continue;
                }
                let market_listed = abi::comptroller::events::MarketListed::must_decode(log);
                market_listed_list
                    .market_listed_list
                    .push(compound::MarketListed {
                        ctoken: market_listed.c_token,
                    });
            }
        }
    }
    Ok(market_listed_list)
}

#[substreams::handlers::store]
fn store_mint(mint_list: compound::MintList, output: store::StoreSet) {
    for mint in mint_list.mint_list {
        output.set(
            0,
            String::from_utf8(mint.id.clone()).unwrap(),
            &proto::encode(&mint).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_token(market_listed_list: compound::MarketListedList, output: store::StoreSet) {
    for market_listed in market_listed_list.market_listed_list {
        let ctoken_id = market_listed.ctoken;
        // handle eth and sai differently
        // because eth and sai (89d24a6b4ccb1b6faa2625fe562bdd9a23260359) are NOT ERC20 tokens
        let is_ceth = ctoken_id == Hex::decode("4ddc2d193948926d02f9b1fe9e1daa0718270ed5").unwrap();
        let is_csai = ctoken_id == Hex::decode("f5dce57282a584d2746faf1593d3121fcac444dc").unwrap();

        let ctoken_res = rpc::fetch_token(ctoken_id.clone());
        if ctoken_res.is_err() {
            continue;
        }
        let ctoken = ctoken_res.unwrap();
        let underlying_token_res = if is_ceth {
            Ok(compound::Token {
                id: NULL_ADDRESS.to_vec(),
                name: "Ether".to_string(),
                symbol: "ETH".to_string(),
                decimals: 18,
            })
        } else if is_csai {
            Ok(compound::Token {
                id: Hex::decode("89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap(),
                name: "Sai Stablecoin v1.0 (SAI)".to_string(),
                symbol: "SAI".to_string(),
                decimals: 18,
            })
        } else {
            rpc::fetch(rpc::RpcCallParams {
                to: ctoken_id.clone(),
                method: "underlying()".to_string(),
                args: vec![],
            })
            .map(|x| x[12..32].to_vec())
            .and_then(rpc::fetch_token)
        };
        if underlying_token_res.is_err() {
            continue;
        }
        let underlying_token = underlying_token_res.unwrap();
        output.set(
            0,
            format!("market:{}:ctoken", Hex::encode(ctoken_id.clone())),
            &proto::encode(&ctoken).unwrap(),
        );
        output.set(
            0,
            format!("market:{}:underlying", Hex::encode(ctoken_id.clone())),
            &proto::encode(&underlying_token).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_mint_count(mint_list: compound::MintList, output: store::StoreAddInt64) {
    for mint in mint_list.mint_list {
        output.add(
            0,
            format!("mint:count:{}", mint.timestamp / (24 * 60 * 60)),
            1,
        )
    }
}

#[substreams::handlers::store]
fn store_market_count(
    market_listed_list: compound::MarketListedList,
    output: store::StoreAddInt64,
) {
    for _ in market_listed_list.market_listed_list {
        output.add(0, "market:count".to_string(), 1)
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

// TODO: interest accumulated -> protocol side revenue, supply side revenue

#[substreams::handlers::store]
fn store_price(
    accrue_interest_list: compound::AccrueInterestList,
    store_oracle: store::StoreGet,
    store_token: store::StoreGet,
    output: store::StoreSet,
) {
    for accrue_interest in accrue_interest_list.accrue_interest_list {
        let market_address = accrue_interest.address;
        let oracle_res = store_oracle.get_last(&"protocol:oracle".to_string());
        let underlying_res = store_token.get_last(&format!(
            "market:{}:underlying",
            Hex::encode(&market_address)
        ));
        if let (Some(oracle), Some(underlying)) = (oracle_res, underlying_res) {
            let underlying_token: compound::Token = proto::decode(&underlying).unwrap();
            let price_usd_res = utils::get_underlying_price_usd(
                market_address.clone(),
                underlying_token.id,
                oracle,
                accrue_interest.block_number,
                underlying_token.decimals,
            );
            if price_usd_res.is_err() {
                continue;
            }
            output.set(
                0,
                format!("market:{}:underlying:price", Hex::encode(&market_address)),
                &Vec::from(price_usd_res.unwrap().to_string()),
            )
        }
    }
}

#[substreams::handlers::store]
fn store_tvl(
    accrue_interest_list: compound::AccrueInterestList,
    store_token: store::StoreGet,
    store_price: store::StoreGet,
    output: store::StoreSet,
) {
    for accrue_interest in accrue_interest_list.accrue_interest_list {
        let market_address = accrue_interest.address;
        let underlying_res: Option<compound::Token> = store_token
            .get_last(&format!(
                "market:{}:underlying",
                Hex::encode(&market_address)
            ))
            .map(|x| proto::decode(&x).unwrap());
        let underlying_price_res = store_price
            .get_last(&format!(
                "market:{}:underlying:price",
                Hex::encode(&market_address)
            ))
            .map(|x| utils::string_to_bigdecimal(x.as_ref()));
        let ctoken_supply_res = rpc::fetch(rpc::RpcCallParams {
            to: market_address.clone(),
            method: "totalSupply()".to_string(),
            args: vec![],
        })
        .map(|x| utils::bytes_to_bigdecimal(x.as_ref()));
        let ctoken_exchange_rate_res = rpc::fetch(rpc::RpcCallParams {
            to: market_address.clone(),
            method: "exchangeRateStored()".to_string(),
            args: vec![],
        })
        .map(|x| utils::bytes_to_bigdecimal(x.as_ref()));
        if let (
            Some(underlying),
            Some(underlying_price),
            Ok(ctoken_supply),
            Ok(ctoken_exchange_rate),
        ) = (
            underlying_res,
            underlying_price_res,
            ctoken_supply_res,
            ctoken_exchange_rate_res,
        ) {
            let total_value_locked = ctoken_supply
                .mul(ctoken_exchange_rate)
                .div(exponent_to_big_decimal(
                    utils::MANTISSA_FACTOR + underlying.decimals,
                ))
                .mul(underlying_price);
            output.set(
                0,
                format!("market:{}:tvl", Hex::encode(&market_address)),
                &Vec::from(total_value_locked.to_string()),
            )
        }
    }
}
