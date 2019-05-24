extern crate reqwest;
extern crate rust_decimal;
extern crate serde;

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Rates {
    // btc: Decimal,
    eth: Decimal,
}

#[derive(Deserialize)]
struct Data {
    // currency: String,
    rates: Rates,
}

#[derive(Deserialize)]
struct Body {
    data: Data,
}

fn main() -> reqwest::Result<()> {
    const URL: &str = "https://api.coinbase.com/v2/exchange-rates";
    let body: Body = reqwest::get(URL)?.json()?;
    let rates = body.data.rates;
    println!("ETC Rate: {}", rates.eth);
    let rate = (Decimal::from(1) / rates.eth).round_dp(2);
    println!("Dollars Per Ether: {}", rate);
    Ok(())
}
