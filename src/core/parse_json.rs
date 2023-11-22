use crate::core::path::ProjPaths;
use crate::ResultObjects;
use anyhow::Result;
use serde_json;
use std::fs;
use std::io::{self, prelude::*, BufReader};

pub fn parse_json_main() {
    if let Err(e) = parse() {
        panic!("{e}");
    }
}
fn parse() -> Result<()> {
    let vec: Vec<ResultObjects> = {
        let mut s = String::new();
        let mut filepath = ProjPaths::res_path()?;
        let _ = &filepath.push("result.json");
        let f = fs::File::open(filepath)?;
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut s).expect("reader failed.");
        serde_json::from_str(&s).expect("serde_json fail")
    };
    dbg!(vec);
    Ok(())
}
