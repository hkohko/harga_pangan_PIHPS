use anyhow::{Result, Context};
use std::path::PathBuf;
use std::env;

pub struct ProjPaths {}
impl ProjPaths {
    pub fn get_proj_path() -> Result<PathBuf> {
        let cur_dir = env::current_dir()
            .with_context(|| format!("Cannot access current_dir"))?;
        let proj_path = PathBuf::from(&cur_dir);
        Ok(proj_path)
    }
    pub fn res_path() -> Result<PathBuf>{
        let mut res = ProjPaths::get_proj_path()?;
        let _ = &res.push("res");
        Ok(res)
    }
}