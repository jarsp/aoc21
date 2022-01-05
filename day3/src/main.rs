use std::{collections::HashSet, env, error::Error, fs::File, io::{BufRead, BufReader}};

// Part 1
fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let file = File::open(input)?;
    let mut lines = 0;
    let mut counts: Vec<u32> = Vec::new();
    for line in BufReader::new(file).lines() {
        if let Ok(bin) = line {
            if counts.len() != bin.len() {
                counts.resize(bin.len(), 0);
            }

            for (i, c) in bin.chars().enumerate() {
                let d = c.to_digit(10).unwrap();
                counts[i] += d;
            }

            lines += 1;
        }
    }

    let gamma = counts.iter()
                          .map(|c| *c > lines/2)
                          .fold(0, |acc, c| (acc << 1) + (c as i32));
    let epsilon = (1 << counts.len()) - 1 - gamma;
    
    println!("gamma: {} epsilon: {} product: {}", gamma, epsilon, gamma * epsilon);

    Ok(())
}

fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let file = File::open(input)?;
    let mut gamma_candidates: HashSet<String> =
        BufReader::new(file).lines()
                                  .map(|x| x.unwrap())
                                  .collect();

    let file = File::open(input)?;
    let mut epsilon_candidates: HashSet<String> =
        BufReader::new(file).lines()
                                  .map(|x| x.unwrap())
                                  .collect();

    let n = gamma_candidates.iter().nth(0).unwrap().len();
    assert!(n > 0);

    fn candidate_bit(v: &HashSet<String>, index: usize, is_gamma: bool) -> char {
        let count: usize =
            v.iter()
             .map(|s| s.chars().nth(index).unwrap().to_digit(10).unwrap() as usize)
             .sum();
        let total = v.len();
        if count == 0 {
            '0'
        } else if count == total {
            '1'
        } else if is_gamma {
            if count >= total/2 {
                '1'
            } else {
                '0'
            }
        } else {
            if count >= total/2 {
                '0'
            } else {
                '1'
            }
        }
    }

    for i in 0..n {
        let gc = candidate_bit(&gamma_candidates, i, true);
        gamma_candidates.retain(|s| s.chars().nth(i).unwrap() == gc);

        let ec = candidate_bit(&epsilon_candidates, i, false);
        epsilon_candidates.retain(|s| s.chars().nth(i).unwrap() == ec);
    }

    assert!(gamma_candidates.len() == 1);
    assert!(epsilon_candidates.len() == 1);

    let gamma = i32::from_str_radix(gamma_candidates.iter().nth(0).unwrap().as_str(), 2)?;
    let epsilon = i32::from_str_radix(epsilon_candidates.iter().nth(0).unwrap().as_str(), 2)?;

    println!("gamma: {} epsilon: {} product: {}", gamma, epsilon, gamma * epsilon);

    Ok (())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    part1(&args[1])?;
    part2(&args[1])?;

    Ok(())
}