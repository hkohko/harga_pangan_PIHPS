#![allow(dead_code, unused_variables)]
use anyhow::Result;
use get_prices::core::client::client_main;
use get_prices::core::parse_date::parser;
use get_prices::input::handle_input::sanitize;
fn main() {
    let _ = main_2();
}
fn main_2() -> Result<()> {
    let input = sanitize()?;
    let date = parser(input.as_str())?;
    let client = client_main(&date);
    println!("{:?}", date);
    Ok(())
}
