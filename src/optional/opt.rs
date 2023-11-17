#![allow(dead_code, unused_imports, unused_variables)]
use crate::core::path::ProjPaths;
use anyhow::Result;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

fn write_months_to_json_file() -> Result<()> {
    #[derive(Debug, serde::Serialize)]
    struct Months {
        month_date: Vec<String>,
    }
    let m = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut content: Vec<String> = Vec::new();
    for n in &m {
        let _ = &content.push(format!("{n} "));
    }
    let month_struct = Months {
        month_date: content,
    };
    let json = serde_json::to_string(&month_struct)?;
    write_things(json)?;
    Ok(())
}

fn write_things(data: String) -> Result<()> {
    let mut res_path = ProjPaths::res_path().unwrap();
    res_path.push("month.json");
    let f = std::fs::File::create(&res_path)?;
    let mut writer = BufWriter::new(f);
    writer.write(data.as_bytes())?;
    Ok(())
}
