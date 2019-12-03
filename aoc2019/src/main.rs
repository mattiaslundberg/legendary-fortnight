use std::fs::{create_dir, read, write, OpenOptions};
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2019")]
struct Opt {
    #[structopt(name = "day", required = true)]
    day: u32,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    create_dir(format!("src/day{}/", opt.day))?;
    let content = String::from_utf8(read(format!("src/solve_template.rs"))?).unwrap();
    write(
        format!("src/day{}/solve.rs", opt.day),
        content
            .as_str()
            .replace("{day}", format!("{}", opt.day).as_str()),
    )?;
    write(format!("src/day{}/input.txt", opt.day), "")?;

    let mut file = OpenOptions::new().append(true).open("Cargo.toml")?;
    file.write(
        format!(
            "\n[[bin]]\nname = \"day{}\"\n path = \"src/day{}/solve.rs\"\n",
            opt.day, opt.day
        )
        .as_bytes(),
    )?;
    Ok(())
}
