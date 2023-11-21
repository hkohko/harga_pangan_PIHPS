use anyhow::Result;
use std::io;

pub fn input_main() {}
pub fn input() -> Result<Vec<String>> {
    let mut x = String::new();
    let mut y = String::new();

    println!("Input date: ");
    io::stdin().read_line(&mut x)?;
    println!("Input commodity code: ");
    io::stdin().read_line(&mut y)?;

    let arg_vec = vec![x.trim().to_owned(), y.trim().to_owned()];
    Ok(arg_vec)
}
