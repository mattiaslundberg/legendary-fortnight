use std::fs::read;

fn one() -> std::io::Result<()> {
    let data = read(format!("input.txt"))?;

    println!("{}", "TODO");
    Ok(())
}

fn two() -> std::io::Result<()> {
    let data = read(format!("input.txt"))?;

    println!("{}", "TODO");
    Ok(())
}

fn main() -> std::io::Result<()> {
    one()?;
    two()?;
    Ok(())
}
