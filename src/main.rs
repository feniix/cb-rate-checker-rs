extern crate requests;
extern crate bigdecimal;
extern crate json;

// adds method .json() to the Response class
use requests::ToJson;

use std::str::FromStr;
use bigdecimal::BigDecimal;

//use requests::{Codes, Request, Response, StatusCode};

fn main() {

    const URL: &str = "https://api.coinbase.com/v2/exchange-rates";
    let res = requests::get(URL).unwrap();
    let data = res.json().unwrap();
    println!("JsonValue({})", data["data"]["rates"]["ETH"]);
    println!("{:?}", data["data"]["rates"]["ETH"]);
    let btc = BigDecimal::from_str(&data["data"]["rates"]["ETH"].to_string()).unwrap();
    let one = BigDecimal::from_str("1").unwrap();
    let rate: BigDecimal = one / btc;
    println!("{:?}", rate.with_scale(2).to_string());

}
