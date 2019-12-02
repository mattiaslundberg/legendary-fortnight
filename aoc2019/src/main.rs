use std::fs::{create_dir, read, write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2019")]
struct Opt {
    #[structopt(short, long)]
    generate: bool,

    #[structopt(name = "day", required = true)]
    day: u32,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    if opt.generate {
        create_dir(format!("src/day{}/", opt.day))?;
        let content = read(format!("src/solve_template.rs"))?;
        write(format!("src/day{}/solve.rs", opt.day), content)?;
        write(format!("src/day{}/input.txt", opt.day), "")?;
    }
    Ok(())
}
