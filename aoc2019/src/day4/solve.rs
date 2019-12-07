use std::io::Result;

fn has_double_and_incr(n: i32) -> bool {
    let s = n.to_string();
    let strs: Vec<i8> = s
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let mut matches = false;
    for i in 1..strs.len() {
        if strs[i] < strs[i - 1] {
            return false;
        }
        if strs[i] == strs[i - 1] {
            matches = true;
        }
    }

    matches
}
fn has_double_and_incr2(n: i32) -> bool {
    let s = n.to_string();
    let strs: Vec<i8> = s
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let mut matches = false;
    for i in 1..strs.len() {
        if strs[i] < strs[i - 1] {
            return false;
        }
        let mut x = 0;
        if i >= 2 {
            x = strs[i - 2];
        }
        let mut y = 0;
        if i + 1 < strs.len() {
            y = strs[i + 1];
        }
        if strs[i] == strs[i - 1] && x != strs[i] && y != strs[i] {
            matches = true;
        }
    }
    matches
}

fn one() -> Result<()> {
    let range = 123257..647015;
    let mut matches = 0;
    for n in range {
        if has_double_and_incr(n) {
            matches += 1;
        }
    }

    println!("{}", matches);
    Ok(())
}

fn two() -> Result<()> {
    let range = 123257..647015;
    let mut matches = 0;
    for n in range {
        if has_double_and_incr2(n) {
            matches += 1;
        }
    }

    println!("{}", matches);
    Ok(())
}

fn main() -> Result<()> {
    one()?;
    two()?;
    Ok(())
}
