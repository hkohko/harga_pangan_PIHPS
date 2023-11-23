use anyhow::Result;
use std::io;

pub fn input_main() {}
pub fn input() -> Result<Vec<String>> {
    let mut x = String::new();

    println!("Input date: ");
    io::stdin().read_line(&mut x)?;

    let arg_vec = vec![x.trim().to_owned()];
    Ok(arg_vec)
}
