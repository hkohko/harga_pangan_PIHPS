use crate::core::path::ProjPaths;
use crate::ResultObjects;
use num_format::{Locale, ToFormattedString};
use anyhow::Result;
use serde_json;
use std::fs;
use std::io::{prelude::*, BufReader};

pub fn parse_json_main(prov: &String) {
    if let Err(e) = parse(prov) {
        panic!("{e}");
    }
}
fn parse(prov: &String) -> Result<()> {
    let vec: Vec<ResultObjects> = {
        let mut s = String::new();
        let mut filepath = ProjPaths::res_path()?;
        let _ = &filepath.push("result.json");
        let f = fs::File::open(filepath)?;
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut s).expect("reader failed.");
        serde_json::from_str(&s).expect("serde_json fail")
    };
    let mut s = String::new();
    for obj in vec.iter() {
        if &obj.Provinsi == prov {
            let nilai_formatted = {
                let to_u32 = obj.Nilai as u32;
                to_u32.to_formatted_string(&Locale::en)
            };
            let to_print = format!("{} di {}:\nRp{}", &obj.Komoditas, &obj.Provinsi, &nilai_formatted);
            s.push_str(to_print.as_str())
        }
    }
    if s.len() != 0 {
        println!("{s}")
    } else {
        println!("Tidak ada data.")
    }
    Ok(())
}
