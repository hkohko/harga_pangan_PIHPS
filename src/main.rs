#![allow(dead_code, unused_variables)]
use anyhow::Result;
use get_prices::core::client::client_main;
use get_prices::core::parse_config::parse_config;
use get_prices::core::parse_date::parser;
use get_prices::core::parse_json::parse_json_main;
use get_prices::input::handle_input::input;

fn main() {
    match stack() {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    }
}
fn stack() -> Result<()> {
    let mut commodity_code = String::new();
    let cfg = parse_config()?;
    let input = input()?;
    let date_input = &input[0];
    if input[1].len() == 0 {
        commodity_code.push_str(&cfg[1])
    } else {
        commodity_code.push_str(&input[1])
    }
    let date = parser(date_input)?;
    let client = client_main(&date, &commodity_code);
    let _ = parse_json_main(&cfg[0]);
    Ok(())
}
