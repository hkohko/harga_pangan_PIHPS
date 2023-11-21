use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use crate::data::constants::MONTH;

fn get_month(date: &HashMap<&str, u32>) -> Result<String> {
    let input_month = date.get("m").unwrap();
    // Index `month_array`  with `own_input_month`
    let own_input_month =
        usize::try_from(input_month.clone()).expect("can't convert input_month to usize");
    let month_name = MONTH[own_input_month - 1].to_owned();
    // Get the first 3 letters of the month name "January" -> "Jan"
    Ok(month_name.chars().take(3).collect())
}
pub fn regex_url(url: &String, date: &HashMap<&str, u32>, cmdt_code: &String) -> Result<String> {
    let d = date.get("d").unwrap().to_string();
    let m = get_month(date)?.to_string();
    let y = date.get("y").unwrap().to_string();

    let date_regex = Regex::new(r"(?<dq>tanggal=)\d+%20\w+%20\d+")?;
    let cmdt_regex = Regex::new(r"(?<cmdt>commodity=)\w+&")?;
    let date_re_fmt = format!("${{dq}}{d}%20{m}%20{y}");
    let cmdt_re_fmt = format!("${{cmdt}}{cmdt_code}&");

    let date_r = date_regex.replace(url, date_re_fmt.as_str());
    let cmdt_r = cmdt_regex.replace(&date_r, cmdt_re_fmt.as_str());
    Ok(cmdt_r.to_string())
}
