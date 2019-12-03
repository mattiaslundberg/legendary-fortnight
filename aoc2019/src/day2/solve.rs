use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

fn calc(mut ops: Vec<usize>, noun: usize, verb: usize) -> std::io::Result<usize> {
    ops[1] = noun;
    ops[2] = verb;
    let mut location = 0;
    loop {
        location = match ops[location] {
            1 => {
                let l1 = ops[location + 1];
                let l2 = ops[location + 2];
                let l3 = ops[location + 3];
                ops[l3] = ops[l1] + ops[l2];
                location + 4
            }
            2 => {
                let l1 = ops[location + 1];
                let l2 = ops[location + 2];
                let l3 = ops[location + 3];
                ops[l3] = ops[l1] * ops[l2];
                location + 4
            }
            99 => break,
            a => {
                println!("FAiled {:?}", a);
                break;
            }
        };
    }
    Ok(ops[0])
}

fn one() -> std::io::Result<()> {
    let file = File::open(format!("src/day2/input.txt"))?;
    for line in BufReader::new(file).lines() {
        let l = line?;
        let data: Vec<&str> = l.split(",").collect();
        let ops: Vec<usize> = data.iter().map(|d| d.parse::<usize>().unwrap()).collect();
        let r = calc(ops, 12, 2)?;
        println!("{:?}", r);
    }

    Ok(())
}

fn two() -> std::io::Result<()> {
    let file = File::open(format!("src/day2/input.txt"))?;
    for line in BufReader::new(file).lines() {
        let l = line?;
        let data: Vec<&str> = l.split(",").collect();
        let ops: Vec<usize> = data.iter().map(|d| d.parse::<usize>().unwrap()).collect();
        'outer: for noun in 1..100 {
            for verb in 1..100 {
                let r = match calc(ops.clone(), noun, verb) {
                    Ok(v) => v,
                    _ => 0,
                };
                if r == 19690720 {
                    println!("{}", 100 * noun + verb);
                    break 'outer;
                }
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    one()?;
    two()?;
    Ok(())
}
