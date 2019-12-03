use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn one() -> std::io::Result<()> {
    let file = File::open(format!("src/day1/input.txt"))?;
    let mut total: f32 = 0.0;
    for line in BufReader::new(file).lines() {
        let weight: f32 = line?.parse::<f32>().unwrap();
        let cost = (weight / 3.0).floor() - 2.0;
        total += cost;
    }

    println!("{}", total);
    Ok(())
}

fn two() -> std::io::Result<()> {
    let file = File::open(format!("src/day1/input.txt"))?;
    let mut total: f32 = 0.0;
    for line in BufReader::new(file).lines() {
        let mut cost: f32 = line?.parse::<f32>().unwrap();
        while cost > 0.0 {
            cost = (cost / 3.0).floor() - 2.0;
            if cost > 0.0 {
                total += cost;
            }
        }
    }

    println!("{}", total);
    Ok(())
}

fn main() -> std::io::Result<()> {
    one()?;
    two()?;
    Ok(())
}
