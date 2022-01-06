use std::{env, error::Error, fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

#[derive(Debug)]
struct Board {
    grid: Vec<i32>,
    marked: [i32; 5],
    scored: bool
}

impl Board {
    pub fn parse_board<
        E: Error,
        I: Iterator<Item = Result<String, E>>
    >(it: I) -> Option<Board> {
        it.flat_map(|r| {
            r.unwrap()
             .split_whitespace()
             .map(&str::parse::<i32>)
             .collect_vec()
          })
          .collect::<Result<Vec<i32>, _>>()
          .map_or(
              None, 
              |v| Some(Board {grid: v, marked: [0, 0, 0, 0, 0], scored: false})
          )
    }

    pub fn mark(&mut self, n: i32) -> Option<i32> {
        match self.grid.iter().position(|&x| x == n) {
            Some(p) => self.marked[p/5] |= 1 << (p % 5),
            None => ()
        }

        let bingo =
            self.marked.contains(&0x1f) ||
            self.marked.iter()
                       .fold(0x1f, |acc, x| acc & x) != 0;

        if bingo {
            self.scored = true;
            let s =
                self.grid.iter()
                        .enumerate()
                        .filter(|&(p, _)| {
                            self.marked[p/5] & (1 << (p % 5)) == 0
                        })
                        .map(|(_, n)| n)
                        .sum::<i32>();
            
            Some(s * n)
        } else {
            None
        }
    }

    pub fn is_playing(&self) -> bool {
        !self.scored
    }

    pub fn reset(&mut self) {
        self.scored = false;
        self.marked.iter_mut().for_each(|x| *x = 0);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let mut file_iter = BufReader::new(file).lines();
    let seq =
        file_iter
            .next()
            .unwrap()?
            .split(',')
            .map(&str::parse::<i32>)
            .collect::<Result<Vec<_>,_>>()?;
    
    let mut boards =
        file_iter
            .chunks(6)
            .into_iter()
            .filter_map(Board::parse_board)
            .collect_vec();
    
    // Part 1
    let mut bingo_score = -1;
    'outer: for &n in &seq {
        for b in &mut boards {
            match b.mark(n) {
                Some(score) => {
                    bingo_score = score;
                    break 'outer
                },
                None => ()
            }
        }
    }

    println!("First bingo score: {}", bingo_score);
    boards.iter_mut().for_each(Board::reset);

    // Part 2
    bingo_score = -1;
    for &n in &seq {
        boards.iter_mut().for_each(|b| {
            bingo_score = b.mark(n).unwrap_or(bingo_score);
        });
        boards.retain(Board::is_playing);
    }
    println!("Last bingo score: {}", bingo_score);

    Ok(())
}
