use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn calc(mut ops: Vec<i32>, input: i32) -> std::io::Result<i32> {
    let mut location = 0;
    loop {
        let op = ops[location];
        let op_code = op % 100;
        let m1 = (op / 100) % 10;
        let m2 = (op / 1000) % 10;

        location = match op_code {
            1 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                let l2 = if m2 > 0 {
                    ops[location + 2]
                } else {
                    ops[usize::try_from(ops[location + 2]).ok().unwrap()]
                };
                let l3 = ops[location + 3];
                ops[usize::try_from(l3).ok().unwrap()] = (l1 + l2).try_into().unwrap();
                location + 4
            }
            2 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                let l2 = if m2 > 0 {
                    ops[location + 2]
                } else {
                    ops[usize::try_from(ops[location + 2]).ok().unwrap()]
                };
                let l3 = ops[location + 3];
                ops[usize::try_from(l3).ok().unwrap()] = (l1 * l2).try_into().unwrap();
                location + 4
            }
            3 => {
                let l = usize::try_from(ops[location + 1]).ok().unwrap();
                ops[l] = input;
                location + 2
            }
            4 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                println!("Output: {}", l1);
                location + 2
            }
            5 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                let l2 = if m2 > 0 {
                    ops[location + 2]
                } else {
                    ops[usize::try_from(ops[location + 2]).ok().unwrap()]
                };
                if l1 != 0 {
                    l2.try_into().unwrap()
                } else {
                    location + 3
                }
            }
            6 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                let l2 = if m2 > 0 {
                    ops[location + 2]
                } else {
                    ops[usize::try_from(ops[location + 2]).ok().unwrap()]
                };
                if l1 == 0 {
                    l2.try_into().unwrap()
                } else {
                    location + 3
                }
            }
            7 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                let l2 = if m2 > 0 {
                    ops[location + 2]
                } else {
                    ops[usize::try_from(ops[location + 2]).ok().unwrap()]
                };
                let l3 = ops[location + 3];
                ops[usize::try_from(l3).ok().unwrap()] = if l1 < l2 { 1 } else { 0 };
                location + 4
            }
            8 => {
                let l1 = if m1 > 0 {
                    ops[location + 1]
                } else {
                    ops[usize::try_from(ops[location + 1]).ok().unwrap()]
                };
                let l2 = if m2 > 0 {
                    ops[location + 2]
                } else {
                    ops[usize::try_from(ops[location + 2]).ok().unwrap()]
                };
                let l3 = ops[location + 3];
                ops[usize::try_from(l3).ok().unwrap()] = if l1 == l2 { 1 } else { 0 };
                location + 4
            }
            99 => break,
            a => {
                println!("Failed: {:?}", a);
                break;
            }
        };
    }
    Ok(ops[0])
}

fn one() -> Result<()> {
    let file = File::open(format!("src/day5/input.txt"))?;
    for line in BufReader::new(file).lines() {
        let l = line?;
        let data: Vec<&str> = l.split(",").collect();
        let ops: Vec<i32> = data.iter().map(|d| d.parse::<i32>().unwrap()).collect();
        let r = calc(ops, 1)?;
        println!("{:?}", r);
    }

    Ok(())
}

fn two() -> Result<()> {
    let file = File::open(format!("src/day5/input.txt"))?;
    for line in BufReader::new(file).lines() {
        let l = line?;
        let data: Vec<&str> = l.split(",").collect();
        let ops: Vec<i32> = data.iter().map(|d| d.parse::<i32>().unwrap()).collect();
        let r = calc(ops, 5)?;
        println!("{:?}", r);
    }
    Ok(())
}

fn main() -> Result<()> {
    one()?;
    two()?;
    Ok(())
}
