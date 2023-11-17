use anyhow::Result;
use chrono::prelude::*;
use dateparser;
use std::collections::HashMap;
pub fn parser_main() {
    let date = "23 sep 2023";
    let p = parser(date);
    if let Err(e) = &p {
        println!("{e}");
    }
    println!("{p:?}");
}
pub fn parser(date: &str) -> Result<HashMap<&str, u32>> {
    let parse_local = dateparser::datetime::Parse::new(&Local, None);
    let r = parse_local.parse(date).unwrap_or_else(|e| panic!("{e}"));
    let mut map = HashMap::new();
    let d = r.day();
    let m = r.month();
    let y = u32::try_from(r.year())
        .unwrap_or_else(|e| panic!("Unable to convert to u32 from {}\n{e}", r.year()));
    map.insert("d", d);
    map.insert("m", m);
    map.insert("y", y);
    Ok(map)
}
