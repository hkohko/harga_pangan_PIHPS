use serde::Deserialize;

pub mod core {
    pub mod client;
    pub mod parse_date;
    pub mod parse_url;
    pub mod parse_json;
    pub mod path;
}
pub mod input {
    pub mod handle_input;
}
pub mod optional {
    pub mod opt;
}
pub mod data {
    pub mod constants;
}
#[derive(Deserialize, Debug)]
pub struct ResultObjects {
    ProvID: i32,
    Provinsi: String,
    Tanggal: String,
    Komoditas: String,
    Nilai: f32,
    NilaiDiff: String,
    SemuaProvinsi: f32,
    Kelompok: i32,
    stdDev: f32,
    Percentage: f32,
    SemuaPercentage: f32,
    stdDevPercentage: Option<String>,
    TanggalLast: String,
    TanggalInflasi: String,
    show: bool,
}
