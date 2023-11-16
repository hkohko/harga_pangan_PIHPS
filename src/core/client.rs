use anyhow::{Context, Result};
use reqwest::blocking;
use serde_json;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use crate::core::path::ProjPaths;

pub fn client_main() {
    if let Err(e) = process_client() {
        println!("{e}");
    }
}
fn process_client() -> Result<()> {
    let url = get_url()?;
    let header = get_header()?;
    dbg!(url);
    dbg!(header);
    Ok(())
}
fn get_url() -> Result<String> {
    let filename = "url.txt";
    let mut url = String::new();
    let mut filepath = ProjPaths::res_path()?;
    filepath.push(filename);
    let f = fs::File::open(filepath).with_context(|| format!("Failed to open {}", filename))?;
    let mut reader = BufReader::new(&f);
    reader.read_to_string(&mut url)?;
    Ok(url)
}
fn get_header() -> Result<serde_json::Value> {
    let filename = "header.json";
    let mut header: Vec<u8> = Vec::new();
    let mut filepath = ProjPaths::res_path()?;
    filepath.push(filename);
    let f = fs::File::open(filepath).with_context(|| format!("Failed to open {}", filename))?;
    let mut reader = BufReader::new(f);
    reader.read_to_end(&mut header)?;
    let v =
        serde_json::from_slice(&header).with_context(|| format!("Error parsing header to json"))?;
    Ok(v)
}
