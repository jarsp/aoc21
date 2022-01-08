use std::{env, fs::File, io::{BufRead, BufReader}, error::Error};

fn step(ages: &mut [u64; 9]) {
    let regen = ages[0];
    (1..9).for_each(|n| ages[n - 1] = ages[n]);
    ages[8] = 0;
    ages[6] += regen;
    ages[8] += regen;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let mut ages = [0; 9];
    BufReader::new(file)
        .lines()
        .next()
        .ok_or("Read Error")??
        .split(',')
        .map(&str::parse::<usize>)
        .try_for_each(|r| {
            r.and_then(|n| {
                ages[n] += 1;
                Ok(())
            })
        })?;
    
    // Part 1
    (0..80).for_each(|_| {
        step(&mut ages);
    });
    println!("Total fish after 80 days: {}", ages.iter().sum::<u64>());

    // Part 2
    (80..256).for_each(|_| {
        step(&mut ages);
    });
    println!("Total fish after 256 days: {}", ages.iter().sum::<u64>());

    Ok(())
}
