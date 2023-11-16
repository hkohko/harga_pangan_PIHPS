use std::io;
use anyhow::Result;
pub fn input_main() {
    let _ = sanitize();
}
pub fn sanitize() -> Result<String> {
    // accepted date: dd mmm yyyy
    let i = input()?;
    let trim = i.trim();
    Ok(trim.to_string())
}
fn input() -> Result<String>{
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    Ok(x)
}