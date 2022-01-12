use std::{error::Error, env, fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;

const OPENS: [char; 4] = ['(', '[', '{', '<'];
const CLOSES: [char; 4] = [')', ']', '}', '>'];
const MATCHES: [(char, char); 4] = [
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('<', '>'),
];
const SCORING: [(char, u64); 4] = [
    (')', 3),
    (']', 57),
    ('}', 1197),
    ('>', 25137),
];
const COMPLETE_SCORE: [(char, u64); 4] = [
    ('(', 1),
    ('[', 2),
    ('{', 3),
    ('<', 4),
];

enum Score {
    CompleteScore(u64),
    ErrorScore(u64),
}

fn lookup<T: Copy>(xs: &[(char, T)], c: char) -> Result<T, &'static str> {
    xs.iter().find(|&&(k, _)| k == c).map(|&(_, t)| t).ok_or("bad lookup")
}

fn parse_score(s: &str) -> Result<Score, &'static str> {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        if OPENS.contains(&c) {
            stack.push(c);
        } else if CLOSES.contains(&c) {
            let score = lookup(&SCORING, c).map(Score::ErrorScore);
            match stack.pop() {
                Some(open) => {
                    match lookup(&MATCHES, open) {
                        Ok(close) => if close != c {
                            return score
                        },
                        Err(s) => return Err(s)
                    }
                }
                None => return score
            }
        } else {
            return Err("invalid character");
        }
    }

    let sc: u64 = 
        stack.iter()
             .try_rfold(0, |acc, &x| lookup(&COMPLETE_SCORE, x).map(|v| acc * 5 + v))?;
    
    Ok(Score::CompleteScore(sc))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Failed to read input");
    }
    
    let file = File::open(&args[1])?;
    let lines: Vec<String> = BufReader::new(file).lines().try_collect()?;

    // Part 1
    let scores: Vec<Score> = lines.iter().map(|s| parse_score(s)).try_collect()?;
    let err_score: u64 =
        scores
            .iter()
            .map(|s| if let Score::ErrorScore(e) = s { *e } else { 0 })
            .sum();
    
    println!("Error Score: {}", err_score);

    // Part 2
    let complete_scores =
        scores
            .iter()
            .filter_map(|s| if let Score::CompleteScore(c) = s { Some(*c) } else { None })
            .sorted()
            .collect_vec();

    println!("Completion Score: {}", complete_scores[complete_scores.len()/2]);

    Ok(())
}
