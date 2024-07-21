pub mod days;
pub mod solution;

use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    time::Instant,
};

use anyhow::Result;

fn main() -> Result<()> {
    let mut day = String::new();
    print!("Enter day: ");
    stdout().flush()?;
    stdin().read_line(&mut day)?;

    let day = day.trim().parse::<u8>()?;

    let mut part = String::new();
    print!("Enter part: ");
    stdout().flush()?;
    stdin().read_line(&mut part)?;

    let part = part.trim().parse::<u8>()?;

    let input = read_to_string(format!("./input/day{:02}.txt", day))?;

    let solution = match day {
        1 => days::day01::SOLUTION,
        _ => unimplemented!(),
    };

    let start = Instant::now();

    let ans = match part {
        1 => solution.part1,
        2 => solution.part2,
        _ => unimplemented!(),
    }
    .unwrap()(&input);

    let elapsed = start.elapsed();

    println!();
    println!("{ans}");
    println!();
    println!("Executed in {:.2?}", elapsed);

    Ok(())
}
