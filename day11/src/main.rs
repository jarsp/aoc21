use std::{env, fs::File, error::Error, io::{BufReader, BufRead}, collections::VecDeque};

use itertools::{Itertools, process_results};
use utils::Grid;

fn step(grid: &mut Grid<u8>) -> i32 {
    let (w, h) = grid.size();

    // Add 1 to each
    grid.iter_mut_coords()
        .flatten()
        .for_each(|(v, _)| *v += 1);
    
    // Flash each octopus
    let mut num_flashed = 0;
    let mut flashed = vec![vec![false; w]; h];
    let mut working: VecDeque<_> =
        grid.iter_coords()
            .flatten()
            .filter_map(|t| {
                if *t.0 > 9 {
                    Some(t.1)
                } else {
                    None
                }
            })
            .collect();
            
    while let Some((x, y)) = working.pop_front() {
        if flashed[y][x] {
            continue
        }

        flashed[y][x] = true;
        num_flashed += 1;
        *grid.get_mut(x, y).unwrap() = 0;

        grid.iter_neighbors(x, y)
            .for_each(|(nx, ny)| {
                let n = grid.get_mut(nx, ny).unwrap();
                if !flashed[ny][nx] {
                    *n += 1;
                    if *n > 9 {
                        working.push_back((nx, ny));
                    }
                }
            });
    }

    num_flashed
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Could not read input");
    }

    let file = File::open(&args[1])?;
    let mut grid: Grid<u8> =
        process_results(
            BufReader::new(file).lines(),
            |iter| Grid::parse_grid(iter, "")
        )??;

    // Part 1
    let flashed_totals =
        (0..100)
            .map(|_| step(&mut grid))
            .collect_vec();
    
    println!("Flashed: {}", flashed_totals.iter().sum::<i32>());

    // Part 2
    let ttl = (grid.size().0 * grid.size().1) as i32;
    if let Some(p) = flashed_totals.iter().position(|&v| v == ttl) {
        println!("Simultaneous: {}", p + 1);
    } else {
        let mut simul = 101;
        loop {
            if step(&mut grid) == ttl {
                println!("Simultaneous: {}", simul);
                break;
            } else {
                simul += 1;
            }
        }
    }
    
    Ok(())
}
