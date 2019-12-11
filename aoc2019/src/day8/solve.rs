use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn count(layer: Vec<Vec<u8>>, find: u8) -> u32 {
    let mut found = 0;
    for r in layer {
        for c in r {
            if c == find {
                found += 1;
            }
        }
    }
    found
}

fn one() -> Result<()> {
    let file = File::open(format!("src/day8/input.txt"))?;
    let line = BufReader::new(file).lines().next();
    let st = line.unwrap().unwrap();
    let l: Vec<&str> = st.split("").collect();

    let mut layers: Vec<Vec<Vec<u8>>> = vec![];

    let mut layer = 0;
    let mut row = 0;
    let mut col = 0;
    for n in l {
        if let Ok(num) = n.parse::<u8>() {
            if layers.len() < layer + 1 {
                layers.push(vec![]);
            }
            if layers[layer].len() < row + 1 {
                layers[layer].push(vec![]);
            }
            layers[layer][row].push(num);
            col += 1;
            if col > 24 {
                col = 0;
                row += 1;
            }
            if row > 5 {
                row = 0;
                layer += 1;
            }
        }
    }

    let mut min_count = 10000000;
    let mut min_layer = 0;
    let mut index = 0;
    for layer in layers.clone() {
        let zeroes = count(layer, 0);
        if zeroes < min_count {
            min_count = zeroes;
            min_layer = index;
        }
        index += 1;
    }

    let layer = &layers[min_layer];
    let ones = count(layer.to_vec(), 1);
    let twos = count(layer.to_vec(), 2);
    println!("{}", ones * twos);

    Ok(())
}

fn two() -> Result<()> {
    let file = File::open(format!("src/day8/input.txt"))?;
    let line = BufReader::new(file).lines().next();
    let st = line.unwrap().unwrap();
    let l: Vec<&str> = st.split("").collect();

    let mut layers: Vec<Vec<Vec<u8>>> = vec![];

    let mut layer = 0;
    let mut row = 0;
    let mut col = 0;
    for n in l {
        if let Ok(num) = n.parse::<u8>() {
            if layers.len() < layer + 1 {
                layers.push(vec![]);
            }
            if layers[layer].len() < row + 1 {
                layers[layer].push(vec![]);
            }
            layers[layer][row].push(num);
            col += 1;
            if col > 24 {
                col = 0;
                row += 1;
            }
            if row > 5 {
                row = 0;
                layer += 1;
            }
        }
    }

    let mut message: Vec<Vec<u8>> = vec![];
    for c in 0..6 {
        message.push(vec![]);
        for _r in 0..25 {
            message[c].push(0);
        }
    }

    for col in 0..25 {
        for row in 0..6 {
            for layer in layers.clone() {
                if layer[row][col] != 2 {
                    message[row][col] = layer[row][col];
                    break;
                }
            }
        }
    }

    for l in message {
        for x in l {
            print!("{}", if x == 1 { "X" } else { " " });
        }
        println!();
    }

    Ok(())
}

fn main() -> Result<()> {
    one()?;
    two()?;
    Ok(())
}
