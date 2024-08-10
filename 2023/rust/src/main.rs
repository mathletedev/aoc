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

    let input = match read_to_string(format!("../input/day{:02}.txt", day)) {
        Ok(input) => input,
        Err(_) => panic!("Input file not found"),
    };

    let solution = match day {
        1 => days::day01::SOLUTION,
        2 => days::day02::SOLUTION,
        3 => days::day03::SOLUTION,
        4 => days::day04::SOLUTION,
        _ => unimplemented!(),
    };

    let solution = match part {
        1 => solution.part1,
        2 => solution.part2,
        _ => unimplemented!(),
    }
    .unwrap();

    let start = Instant::now();

    let ans = solution(&input);

    let elapsed = start.elapsed();

    println!();
    println!("{ans}");
    println!();
    println!("Executed in {:.2?}", elapsed);

    Ok(())
}
