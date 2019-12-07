use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn rec_path(
    moves: Vec<&str>,
    mut grid: HashSet<(i32, i32)>,
    mut distance: HashMap<(i32, i32), i32>,
) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut x: i32 = 1;
    let mut y: i32 = 1;
    let mut d: i32 = 0;
    for mv in moves {
        let mut dir = mv.to_string();
        let amount = dir.split_off(1).parse::<i32>().unwrap();

        match dir.as_str() {
            "U" => {
                for new_x in (x + 1)..(x + amount + 1) {
                    d += 1;
                    distance.insert((new_x, y), d);
                    grid.insert((new_x, y));
                }
                x += amount;
            }
            "D" => {
                for new_x in (x - amount)..(x) {
                    d += 1;
                    distance.insert((new_x, y), d);
                    grid.insert((new_x, y));
                }
                x -= amount;
            }
            "L" => {
                for new_y in (y - amount)..(y) {
                    d += 1;
                    distance.insert((x, new_y), d);
                    grid.insert((x, new_y));
                }
                y -= amount;
            }
            "R" => {
                for new_y in (y + 1)..(y + amount + 1) {
                    d += 1;
                    distance.insert((x, new_y), d);
                    grid.insert((x, new_y));
                }
                y += amount;
            }
            _ => {}
        }
    }
    (grid, distance)
}

fn one() -> std::io::Result<()> {
    let file = File::open(format!("src/day3/input.txt"))?;
    let mut lines = BufReader::new(file).lines();

    let line1 = lines.next().unwrap().unwrap();
    let g1: HashSet<(i32, i32)> = HashSet::new();
    let map1: HashMap<(i32, i32), i32> = HashMap::new();
    let m1: Vec<&str> = line1.split(",").collect();
    let (moves1, _) = rec_path(m1, g1, map1);

    let line2 = lines.next().unwrap().unwrap();
    let g2: HashSet<(i32, i32)> = HashSet::new();
    let map2: HashMap<(i32, i32), i32> = HashMap::new();
    let m2: Vec<&str> = line2.split(",").collect();
    let (moves2, _) = rec_path(m2, g2, map2);

    let common = moves1.intersection(&moves2);
    println!("{:?}", common);

    let mut min = 10000;
    for (x, y) in common {
        let dist = (1 - x).abs() + (1 - y).abs();
        if dist < min {
            min = dist;
        }
    }
    println!("{}", min);

    Ok(())
}

fn two() -> std::io::Result<()> {
    let file = File::open(format!("src/day3/input.txt"))?;
    let mut lines = BufReader::new(file).lines();

    let line1 = lines.next().unwrap().unwrap();
    let g1: HashSet<(i32, i32)> = HashSet::new();
    let map1: HashMap<(i32, i32), i32> = HashMap::new();
    let m1: Vec<&str> = line1.split(",").collect();
    let (moves1, distance1) = rec_path(m1, g1, map1);

    let line2 = lines.next().unwrap().unwrap();
    let g2: HashSet<(i32, i32)> = HashSet::new();
    let map2: HashMap<(i32, i32), i32> = HashMap::new();
    let m2: Vec<&str> = line2.split(",").collect();
    let (moves2, distance2) = rec_path(m2, g2, map2);

    let common = moves1.intersection(&moves2);

    let mut min = 10000000;
    for loc in common {
        let dist = distance2.get(loc).unwrap() + distance1.get(loc).unwrap();
        if dist < min {
            min = dist;
        }
    }

    println!("{}", min);

    Ok(())
}

fn main() -> std::io::Result<()> {
    one()?;
    two()?;
    Ok(())
}
