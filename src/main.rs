extern crate reqwest;
extern crate rust_decimal;
extern crate serde;

use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Data {
    // currency: String,
    rates: HashMap<String, Decimal>,
}

#[derive(Deserialize)]
struct Body {
    data: Data,
}

fn main() -> reqwest::Result<()> {
    const URL: &str = "https://api.coinbase.com/v2/exchange-rates";
    let body: Body = reqwest::get(URL)?.json()?;
    // println!("{}", json.data.rates["ETH"]);
    println!("JsonValue({})", body.data.rates["ETH"]);
    println!("{:?}", body.data.rates["ETH"]);
    let btc = body.data.rates["ETH"];
    let rate = (Decimal::from(1) / btc).round_dp(2);
    println!("{:?}", rate);
    Ok(())
}
