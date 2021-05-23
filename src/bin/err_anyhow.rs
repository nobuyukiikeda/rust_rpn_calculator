use anyhow::{Context, Result, bail};

fn get_int_from_file() -> Result<i32> {
  let path = "number.text";
  let num_str = std::fs::read_to_string(path)
    .with_context(|| format!("failed to read string from {}", path))?;
  
  if num_str.len() >= 10 {
    bail!("It may be too large number");
  }

  num_str.trim()
    .parse::<i32>()
    .map(|t| t * 2)
    .context("failed to parse string")
}

fn main() {
  match get_int_from_file() {
    Ok(x) => println!("{}", x),
    Err(e) => println!("{:#?}", e),
  }
}
