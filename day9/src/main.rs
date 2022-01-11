use std::{error::Error, env, fs::File, io::{BufReader, BufRead}, collections::{VecDeque}};

use itertools::{Itertools, process_results};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let heights: Vec<Vec<u32>> =
        process_results(
            BufReader::new(file).lines(),
            |iter| {
                iter.map(|s| -> Result<Vec<_>, _> {
                    s.chars()
                     .map(|c| c.to_digit(10).ok_or("not a digit"))
                     .try_collect()
                })
                .try_collect()
            }
        )??;
    
    // Part 1
    let ymax = heights.len();
    let xmax = heights[0].len();
    let mut minima: Vec<(usize, usize, u32)> = vec![];

    let get_neighbors = |x, y| {
        let mut neighbors: Vec<(usize, usize)> = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < xmax - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < ymax - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    };

    for (y, hs) in heights.iter().enumerate() {
        for (x, &h) in hs.iter().enumerate() {
            let m =
                get_neighbors(x, y)
                    .iter()
                    .map(|&(x, y)| heights[y][x])
                    .min()
                    .ok_or("no min")?;
            if h < m {
                minima.push((x, y, h));
            }
        }
    }

    let total_risk: u32 = minima.iter().map(|&(_, _, h)| h + 1).sum();

    println!("Total Risk: {}", total_risk);

    // Part 2
    let mut visited: Vec<Vec<bool>> = vec![vec![false; xmax]; ymax];
    let mut flood = |x: usize, y: usize| {
        let mut count = 0;
        let mut working = VecDeque::new();
        working.push_back((x, y));

        while let Some((nx, ny)) = working.pop_front() {
            if visited[ny][nx] || heights[ny][nx] == 9 {
                continue
            }

            visited[ny][nx] = true;
            count += 1;

            get_neighbors(nx, ny).iter().for_each(|&t| working.push_back(t));
        }

        count
    };

    let best: u32 =
        minima.iter()
              .map(|&(x, y, _)| flood(x, y))
              .sorted()
              .rev()
              .take(3)
              .product();
    
    println!("Basin product: {}", best);

    Ok(())
}
