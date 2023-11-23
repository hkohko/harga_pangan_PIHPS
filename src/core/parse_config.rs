use std::env;
use anyhow::Result;
use std::fs;
use std::io::{BufReader, prelude::*};

pub fn config_main() {
    if let Err(e) = parse_config() {
        panic!("{e}");
    }
}
pub fn parse_config() -> Result<Vec<String>> {
    let content = {
        let mut path = env::current_dir()?;
        let _ = &path.push(".config");
        let f = fs::File::open(path)?;
        BufReader::new(f)
    };
    let mut res: Vec<String> = Vec::new();
    for pair in content.lines() {
        let to_split = pair?;
        let mut split = to_split.split("=");
        let _ = split.next().expect("Unexpected key in config");
        let value = split.next().expect("Unexpected value in config");
        res.push(value.to_string());
    }
    Ok(res)
}