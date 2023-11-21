use crate::core::path::ProjPaths;
use anyhow::{Context, Result};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

pub fn data_choices(date: &HashMap<&str, u32>, raw_url: &String, cmdt_code: &String) -> Result<()> {
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
    let regex_url = |url: &String| -> Result<()> {
        let d = date.get("d").unwrap().to_string();
        let m = get_month()?.to_string();
        let y = date.get("y").unwrap().to_string();

        let date_pattern = r"(?<dq>tanggal=)\d+%20\w+%20\d+";
        let commodity_pattern = r"(?<cmdt>commodity=)\w+&";

        let date_regex = Regex::new(date_pattern)?;
        let cmdt_regex = Regex::new(commodity_pattern)?;
        let date_re_fmt = format!("${{dq}}{d}%20{m}%20{y}");
        let cmdt_re_fmt = format!("${{cmdt}}{cmdt_code}&");

        let date_r = date_regex.replace(url, date_re_fmt.as_str());
        let cmdt_r = cmdt_regex.replace(&date_r, cmdt_re_fmt.as_str());
        dbg!(cmdt_r);
        Ok(())
    };
    let r = regex_url(raw_url)?;

    Ok(())
}
