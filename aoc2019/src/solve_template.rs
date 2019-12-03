use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn one() -> Result<()> {
    let file = File::open(format!("src/day{day}/input.txt"))?;
    for line in BufReader::new(file).lines() {}

    println!("{}", "TODO");
    Ok(())
}

fn two() -> Result<()> {
    let file = File::open(format!("src/day{day}/input.txt"))?;
    for line in BufReader::new(file).lines() {}

    println!("{}", "TODO");
    Ok(())
}

fn main() -> Result<()> {
    one()?;
    two()?;
    Ok(())
}
