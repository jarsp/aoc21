use std::{collections::HashMap, env, error::Error, fs::File, io::{BufRead, BufReader}, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct Pos {
    x: i32,
    y: i32
}

impl FromStr for Pos {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<i32> = 
            s.split(',')
             .map(&str::parse::<i32>)
             .try_collect()?;
        
        if p.len() != 2 {
            Err(format!("PosError: {}", s))?
        }

        return Ok(Pos {x: p[0], y: p[1]})
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Line {
    start: Pos,
    end: Pos
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<Pos> =
            s.split(" -> ")
             .map(&str::parse::<Pos>)
             .try_collect()?;

        if p.len() != 2 {
            Err(format!("LineError: {}", s))?
        }

        Ok(Line {start: p[0], end: p[1]})
    }
}

impl Line {
    pub fn is_horz_vert(&self) -> bool {
        self.start.x == self.end.x ||
        self.start.y == self.end.y
    }

    pub fn mark(&self, v: &mut HashMap<Pos, i32>) {
        let dy = self.end.y - self.start.y;
        let dx = self.end.x - self.start.x;
        let (slope_x, slope_y, count) =
            if self.end.x != self.start.x {
                (dx.signum(), dy/dx.abs(), dx.abs() + 1)
            } else if self.end.y != self.start.y {
                (0, dy.signum(), dy.abs() + 1)
            } else {
                (0, 0, 0)
            };

        let mut pos = self.start.clone();
        for _ in 0..count {
            *v.entry(pos).or_insert(0) += 1;
            pos.x += slope_x;
            pos.y += slope_y;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let lines: Vec<Line> =
        BufReader::new(file)
            .lines()
            .map(|r| {
                r.map_err(|e| e.into())
                 .and_then(|s| s.parse::<Line>())
            })
            .try_collect()?;
    
    // Part 1
    let mut counts: HashMap<Pos, i32> = HashMap::new();
    lines.iter()
         .filter(|&l| l.is_horz_vert())
         .for_each(|l| l.mark(&mut counts));
    
    let overlaps =
        counts.iter()
              .filter(|&(_, &count)| count > 1)
              .count();
    
    println!("Overlaps: {}", overlaps);
    
    // Part 2
    counts.clear();
    lines.iter()
         .for_each(|l| l.mark(&mut counts));
    
    let overlaps_diag =
        counts.iter()
              .filter(|&(_, &count)| count > 1)
              .count();
    
    println!("Overlaps_diag: {}", overlaps_diag);

    Ok(())
}
