#![allow(dead_code, unused_variables)]
use crate::core::parse_url::regex_url;
use crate::core::path::ProjPaths;
use anyhow::{Context, Result};
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

pub fn client_main(date: &HashMap<&str, u32>, cmdt_code: &String) {
    if let Err(e) = process_client(date, cmdt_code) {
        panic!("{e}");
    }
}
fn process_client(date: &HashMap<&str, u32>, cmdt_code: &String) -> Result<()> {
    let raw_url = get_url()?;
    let url = regex_url(&raw_url, date, cmdt_code)?;
    let get_header = get_header()?;
    let header = build_header(&get_header)?;
    let client = build_client(header)?;
    let make_req = process_req(&client, &url)?;
    let _ = save_resp(make_req)?;
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
fn build_header(value: &serde_json::Value) -> Result<HeaderMap> {
    let mut header = HeaderMap::new();
    let v = value.as_object();
    if let Some(kv) = v {
        for (key, value) in kv {
            let bytes_key = key.as_bytes();
            let str_val = value
                .as_str()
                .with_context(|| format!("Failed converting header value to &str"))?;
            header.insert(
                HeaderName::from_bytes(bytes_key)?,
                HeaderValue::from_str(str_val)?,
            );
        }
    }
    Ok(header)
}
fn build_client(headers: HeaderMap) -> Result<blocking::Client> {
    let c = blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(c)
}
fn process_req(client: &blocking::Client, url: &String) -> Result<blocking::Response> {
    let resp = client.get(url).send()?;
    let st_code = resp.status().as_u16();
    if st_code != 200 {
        panic!("Non-200 status code");
    }
    Ok(resp)
}
fn save_resp(resp: blocking::Response) -> Result<()> {
    let text = resp.text()?;
    let mut filepath = ProjPaths::res_path()?;
    filepath.push("result.json");
    let f = fs::File::create(&filepath)?;
    let mut writer = BufWriter::new(&f);
    let _ = writer.write(&text.as_bytes())?;
    Ok(())
}
