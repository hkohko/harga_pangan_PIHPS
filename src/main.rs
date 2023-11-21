#![allow(dead_code, unused_variables)]
use anyhow::Result;
use get_prices::core::client::client_main;
use get_prices::core::parse_date::parser;
use get_prices::input::handle_input::input;
fn main() {
    match stack() {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    }
}
fn stack() -> Result<()> {
    let input = input()?;
    let date_input = &input[0];
    let commodity_code = &input[1];
    let date = parser(date_input)?;
    let client = client_main(&date, commodity_code);
    Ok(())
}
