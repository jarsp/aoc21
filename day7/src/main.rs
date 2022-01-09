use std::{env, error::Error, fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;

fn l1(y: i32, xs: &[i32]) -> i32 {
    xs.iter()
      .map(|&x| (y - x).abs())
      .sum()
}

fn not_l2(y: i32, xs: &[i32]) -> i32 {
    xs.iter()
      .map(|&x| (y - x).abs() * ((y - x).abs() + 1))
      .sum::<i32>()/2
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let mut crabs =
        BufReader::new(file)
            .lines()
            .exactly_one()??
            .split(',')
            .map(&str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()?;

    crabs.sort();

    // Part 1
    let median = crabs[crabs.len()/2];

    println!("L1 for crab median: {}", l1(median, &crabs));

    // Part 2
    let mean = crabs.iter().sum::<i32>()/(crabs.len() as i32);
    let range =
        if median < mean {
            median..mean + 2
        } else {
            mean..median + 1
        };
    let best =
        range
            .map(|y| not_l2(y, &crabs))
            .min()
            .ok_or("rude")?;
    
    println!("Mean: {} Median: {}", mean, median);
    println!("Best not l2: {}", best);

    Ok(())
}
