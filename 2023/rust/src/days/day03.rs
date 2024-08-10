use std::collections::HashSet;

use crate::solution::{Part, Solution};

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// walk back to start of number
fn walk_back(x: i32, schematic: &Vec<Vec<char>>, i: &i32) -> i32 {
    if x > 0 && schematic[*i as usize][x as usize - 1].is_ascii_digit() {
        walk_back(x - 1, schematic, i)
    } else {
        x
    }
}

// walk forward to end of number
fn walk_forward(acc: i32, x: i32, schematic: &Vec<Vec<char>>, i: &i32) -> i32 {
    if x >= schematic[0].len().try_into().unwrap()
        || !schematic[*i as usize][x as usize].is_ascii_digit()
    {
        acc
    } else {
        walk_forward(
            acc * 10 + schematic[*i as usize][x as usize].to_digit(10).unwrap() as i32,
            x + 1,
            schematic,
            i,
        )
    }
}

const PART1: Part = |input| {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let coords = schematic.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            // find all symbols
            .filter(|(_, &c)| !c.is_ascii_digit() && c != '.')
            .map(move |(j, _)| (i.try_into().unwrap(), j.try_into().unwrap()))
            .collect::<Vec<(i32, i32)>>()
    });

    coords
        // move in every direction
        .flat_map(|(i, j)| DIRS.iter().map(move |(di, dj)| (i + di, j + dj)))
        // filter out invalid coordinates
        .filter(|(i, j)| {
            *i >= 0
                && *j >= 0
                && *i < schematic.len().try_into().unwrap()
                && *j < schematic[0].len().try_into().unwrap()
                // only take digits
                && schematic[*i as usize][*j as usize].is_ascii_digit()
        })
        .map(|(i, j)| (i, walk_back(j, &schematic, &i)))
        // filter out duplicates
        .collect::<HashSet<(i32, i32)>>()
        .iter()
        .map(|(i, j)| walk_forward(0, *j, &schematic, i))
        .sum::<i32>()
        .to_string()
};

const PART2: Part = |input| {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let coords = schematic.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &c)| c == '*')
            .map(move |(j, _)| (i.try_into().unwrap(), j.try_into().unwrap()))
            .collect::<Vec<(i32, i32)>>()
    });

    coords
        .map(|(i, j)| {
            DIRS.iter()
                .map(move |(di, dj)| (i + di, j + dj))
                .filter(|(i, j)| {
                    *i >= 0
                        && *j >= 0
                        && *i < schematic.len().try_into().unwrap()
                        && *j < schematic[0].len().try_into().unwrap()
                        && schematic[*i as usize][*j as usize].is_ascii_digit()
                })
                .map(|(i, j)| (i, walk_back(j, &schematic, &i)))
                .collect::<HashSet<(i32, i32)>>()
        })
        // find gears with two unique numbers
        .filter(|s| s.len() == 2)
        .map(|s| {
            s.iter()
                .map(|(i, j)| walk_forward(0, *j, &schematic, i))
                .product::<i32>()
        })
        .sum::<i32>()
        .to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
