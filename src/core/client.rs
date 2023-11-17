#![allow(dead_code, unused_variables)]
use crate::core::path::ProjPaths;
use anyhow::{Context, Result};
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

pub fn client_main(date: &HashMap<&str, u32>) {
    if let Err(e) = process_client(date) {
        panic!("{e}");
    }
}
fn process_client(date: &HashMap<&str, u32>) -> Result<()> {
    let raw_url = get_url()?;
    let url = data_choices(date, &raw_url);
    // let get_header = get_header()?;
    // let header = build_header(&get_header)?;
    // let client = build_client(header)?;
    // let make_req = process_req(&client, &url)?;
    // let save_response = save_resp(make_req)?;
    Ok(())
}
fn data_choices(date: &HashMap<&str, u32>, raw_url: &String) -> Result<()> {
    let commodity: &str;
    let mut file_month_path = ProjPaths::res_path()?;
    file_month_path.push("month.json");

    let retval_month = || -> Result<serde_json::Value> {
        let mut month_list = String::new();
        let f_month = fs::File::open(file_month_path)?;
        let mut reader = BufReader::new(f_month);
        reader.read_to_string(&mut month_list)?;
        let month_as_serde_val: serde_json::Value =
            serde_json::from_str(month_list.as_str()).unwrap();
        Ok(month_as_serde_val)
    };
    let to_month_dict = |m_val: &serde_json::Value| -> Result<serde_json::Value> {
        let month_obj = m_val.as_object().unwrap();
        let month_names = month_obj.get("month_date").unwrap();
        Ok(month_names.clone())
    };
    let month_as_serde_val = retval_month()?;
    let month_dict = to_month_dict(&month_as_serde_val);
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
    // let own_resp = resp.by_ref();
    Ok(resp)
}
