use std::{env, error::Error, fs::File, io::{BufReader, BufRead}, str::FromStr, collections::{HashSet, VecDeque}};

use itertools::{Itertools, process_results};
use utils::Coord;

#[derive(Debug)]
enum FoldLine {
    FoldX(usize),
    FoldY(usize),
}

impl FromStr for FoldLine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coord: usize =
            s.rsplit_once('=')
             .ok_or("no coord")?.1
             .parse()?;

        if s.starts_with("fold along x") {
            Ok(FoldLine::FoldX(coord))
        } else if s.starts_with("fold along y") {
            Ok(FoldLine::FoldY(coord))
        } else {
            Err("bad dir".into())
        }
    }
}

#[derive(Debug)]
struct Origami {
    grid: HashSet<Coord>,
    folds: VecDeque<FoldLine>,
}

impl Origami {
    fn parse_origami<I>(iter: I)
        -> Result<Self, Box<dyn Error>> 
    where
        I: IntoIterator<Item = String>
    {
        let mut it = iter.into_iter();
        let grid: HashSet<Coord> =
            it.by_ref()
              .take_while(|s| !s.is_empty())
              .map(|s| s.parse::<Coord>())
              .try_collect()?;
        
        let folds: VecDeque<FoldLine> =
            it.map(|s| s.parse::<FoldLine>())
              .try_collect()?;

        Ok(Origami {grid, folds})
    }

    fn fold_one(&mut self) -> Option<usize> {
        if let Some(fold) = self.folds.pop_front() {
            let f: Box<dyn Fn(Coord) -> Coord> =
                match fold {
                    FoldLine::FoldX(l) => Box::new(move |c: Coord| {
                        if c.x <= l {
                            c
                        } else {
                            Coord {x: 2 * l - c.x, y: c.y}
                        }
                    }),
                    FoldLine::FoldY(l) => Box::new(move |c: Coord| {
                        if c.y <= l {
                            c
                        } else {
                            Coord {x: c.x, y: 2 * l - c.y}
                        }
                    }),
                };

            let reflected: HashSet<Coord> =
                self.grid.iter().map(|&c| f(c)).collect();

            self.grid = reflected;

            Some(self.grid.len())
        } else {
            None
        }
    }

    fn fold(&mut self) {
        while self.fold_one().is_some() {}
    }

    fn show(&self) {
        let width = self.grid.iter().map(|c| c.x + 1).max().unwrap_or(0);
        let height = self.grid.iter().map(|c| c.y + 1).max().unwrap_or(0);

        for y in 0..height {
            for x in 0..width {
                if self.grid.get(&Coord {x, y}).is_some() {
                    print!("#");
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let mut origami =
        process_results(
            BufReader::new(file).lines(),
            |iter| Origami::parse_origami(iter)
        )??;

    // Part 1
    let first = origami.fold_one().ok_or("fold failed")?;
    println!("First fold stars left: {}", first);

    // Part 2
    origami.fold();
    origami.show();
    
    Ok(())
}
