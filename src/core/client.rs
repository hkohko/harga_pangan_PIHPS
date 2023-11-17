#![allow(dead_code, unused_variables)]
use crate::core::path::ProjPaths;
use anyhow::{Context, Result};
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json;
use serde_json::Value;
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
    let retval_month = || -> Result<Value> {
        let mut month_list = String::new();
        let mut file_month_path = ProjPaths::res_path()?;
        file_month_path.push("month.json");
        let f_month = fs::File::open(file_month_path)
            .with_context(|| format!("Failed to open month.json"))?;
        let mut reader = BufReader::new(f_month);
        reader.read_to_string(&mut month_list)?;
        let month_as_serde_val: Value = serde_json::from_str(month_list.as_str())?;
        Ok(month_as_serde_val)
    };
    let to_month_dict = |m_val: &Value| -> Result<Value> {
        let month_obj = m_val.as_object().unwrap();
        let month_names = month_obj.get("month_date").unwrap();
        Ok(month_names.clone())
    };
    let get_month = || -> Result<String> {
        let month_obj_serde: Value = retval_month()?;
        let month_array_serde: Value = to_month_dict(&month_obj_serde)?;
        let input_month = date.get("m").unwrap();
        let own_input_month =
            usize::try_from(input_month.clone()).expect("can't convert input_month to usize");
        let month_array = month_array_serde.as_array().unwrap();
        let month_name = &month_array[own_input_month - 1];
        let month_name_asstr = month_name.as_str().unwrap();
        let name_as_vec: Vec<char> = month_name_asstr.chars().collect();
        let first3 = &name_as_vec[0..3];
        let w: &String = &first3.iter().collect();
        Ok(w.to_owned())
    };
    let r = get_month()?;
    dbg!(r);
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
