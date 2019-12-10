use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn count(
    orbits: HashMap<String, Vec<String>>,
    childs: Vec<String>,
    mut cache: HashMap<String, i32>,
) -> (i32, HashMap<String, i32>) {
    let mut total = 0;
    for child in childs {
        let val = if let Some(&c) = cache.get(&child) {
            c
        } else {
            let (a, mut c) = count(
                orbits.clone(),
                orbits.get(&child).unwrap_or(&vec![]).to_vec(),
                cache,
            );
            c.insert(child, a);
            cache = c;
            a
        };
        total = total + val + 1;
    }
    (total, cache)
}
fn mindist(
    orbits: HashMap<String, Vec<String>>,
    childs: Vec<String>,
    lookfor: String,
) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for child in childs {
        if child == lookfor {
            return vec![child];
        }
        let path = mindist(
            orbits.clone(),
            orbits.get(&child).unwrap_or(&vec![]).to_vec(),
            lookfor.clone(),
        );
        if path.contains(&lookfor) {
            res = path.clone();
            res.insert(0, child);
        }
    }
    res
}

fn one() -> Result<()> {
    let file = File::open(format!("src/day6/input.txt"))?;
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for line in BufReader::new(file).lines() {
        let l = line?;
        let ll: Vec<String> = l
            .split(")")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.to_string())
            .collect();
        let parent = ll[0].clone();
        let child = ll[1].clone();

        let new_childs: Vec<String> = if let Some(childs) = orbits.get(&parent) {
            let mut c = childs.to_vec().clone();
            c.push(child);
            c
        } else {
            vec![child.clone()]
        };
        orbits.insert(parent.clone(), new_childs);
    }

    let mut total = 0;
    let mut cache: HashMap<String, i32> = HashMap::new();

    for (k, v) in orbits.clone() {
        let (r, c) = count(orbits.clone(), v, cache);
        total += r;
        cache = c;
    }

    println!("{}", total);
    Ok(())
}

fn two() -> Result<()> {
    let file = File::open(format!("src/day6/input.txt"))?;
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for line in BufReader::new(file).lines() {
        let l = line?;
        let ll: Vec<String> = l
            .split(")")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.to_string())
            .collect();
        let parent = ll[0].clone();
        let child = ll[1].clone();

        let new_childs: Vec<String> = if let Some(childs) = orbits.get(&parent) {
            let mut c = childs.to_vec().clone();
            c.push(child);
            c
        } else {
            vec![child.clone()]
        };
        orbits.insert(parent.clone(), new_childs);
    }

    let y = mindist(
        orbits.clone(),
        orbits.get("COM").unwrap().to_vec(),
        String::from("YOU"),
    );
    let s = mindist(
        orbits.clone(),
        orbits.get("COM").unwrap().to_vec(),
        String::from("SAN"),
    );

    let mut common = 0;
    println!("{:?} {:?}", y, s);
    for (yv, sv) in y.iter().zip(s.iter()) {
        println!("{} {}", yv, sv);
        if yv == sv {
            common += 1;
        }
    }
    let res = y.len() - common - 1 + s.len() - common - 1;
    println!("{}", res);

    Ok(())
}

fn main() -> Result<()> {
    one()?;
    two()?;
    Ok(())
}
