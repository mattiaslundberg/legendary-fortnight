use std::fs::File;
use std::io::{BufRead, BufReader};

fn one() -> std::io::Result<()> {
    let file = File::open(format!("src/day3/input.txt"))?;
    for line in BufReader::new(file).lines() {}

    println!("{}", "TODO");
    Ok(())
}

fn two() -> std::io::Result<()> {
    let file = File::open(format!("src/day3/input.txt"))?;
    for line in BufReader::new(file).lines() {}

    println!("{}", "TODO");
    Ok(())
}

fn main() -> std::io::Result<()> {
    one()?;
    two()?;
    Ok(())
}
