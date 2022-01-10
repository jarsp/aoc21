use std::{error::Error, env, fs::File, str::FromStr, io::{BufReader, BufRead}, collections::{HashMap, HashSet}};

use itertools::{Itertools, process_results};

//   0
// 1   2
//   3
// 4   5
//   6

const SEGCOUNTS: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
const UNIQUES: [usize; 4] = [SEGCOUNTS[1], SEGCOUNTS[4], SEGCOUNTS[7], SEGCOUNTS[8]];
const NUMMAP: [&str; 10] = [
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",
    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg",
];

#[derive(Debug)]
struct Signal {
    digits: HashMap<usize, Vec<HashSet<char>>>,
    outputs: Vec<String>,
}

impl FromStr for Signal {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vs: Vec<Vec<String>> =
            s.split(" | ")
             .map(|t| {
                 t.split(' ')
                  .map(|u| u.into())
                  .collect_vec()
             })
             .collect_vec();

        let digits_map =
            vs[0].iter()
                 .map(|s| HashSet::from_iter(s.chars())) 
                 .into_group_map_by(|s| s.len());
        
        Ok(Signal {digits: digits_map, outputs: vs[1].clone()})
    }
}

impl Signal {
    fn deduce(&self) -> Result<usize, Box<dyn Error>> {
        let mut deduced: HashMap<char, char> = HashMap::new();

        // Identify 1, 4, 7, 8
        let uniques =
            UNIQUES.iter()
                   .map(|l| {
                       self.digits
                           .get(l)
                           .ok_or("unique not found")
                           .and_then(|v| {
                               if v.len() != 1 {
                                   Err("unique wrong number")
                               } else {
                                   // NB: A bit lame, but I cannot return Ok(&v[0]) here because
                                   // apparently rust thinks this ends up leaking a reference to
                                   // self.digits to outside of deduce, and then you need to
                                   // put lifetime specifiers
                                   Ok(v[0].clone())
                               }
                           })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
        
        // Deduce a from 1 and 7
        let a = *uniques[2].difference(&uniques[0]).next().unwrap();
        deduced.insert(a, 'a');

        // Deduce b, d from 4 and 0
        let d_cands = uniques[1].difference(&uniques[0]).cloned().collect::<HashSet<_>>();
        let d = 
            self.digits
                .get(&SEGCOUNTS[0])
                .ok_or("could not find 0")?
                .iter()
                .map(|digit| d_cands.difference(digit).cloned().collect_vec())
                .filter(|v| v.len() == 1)
                .next()
                .unwrap()[0];
        let b = *d_cands.iter().filter(|&&cand| cand != d).next().unwrap();
        deduced.insert(b, 'b');
        deduced.insert(d, 'd');

        // Deduce c, f from 1 and 6
        let c =
            self.digits
                .get(&SEGCOUNTS[6])
                .ok_or("could not find 6")?
                .iter()
                .map(|digit| uniques[0].difference(digit).cloned().collect_vec())
                .filter(|v| v.len() == 1)
                .next()
                .unwrap()[0];
        let f = *uniques[0].iter().filter(|&&cand| cand != c).next().unwrap();
        deduced.insert(c, 'c');
        deduced.insert(f, 'f');

        // Deduce g from 9
        let mut found: HashSet<char> = HashSet::from_iter(deduced.keys().cloned());
        let g =
            self.digits
                .get(&SEGCOUNTS[9])
                .ok_or("could not find 9")?
                .iter()
                .map(|digit| digit.difference(&found).cloned().collect_vec())
                .filter(|v| v.len() == 1)
                .next()
                .unwrap()[0];
        deduced.insert(g, 'g');

        // Deduce e
        found.insert(g);
        let all: HashSet<char> = HashSet::from_iter("abcdefg".chars());
        let e = all.difference(&found).cloned().next().unwrap();
        deduced.insert(e, 'e');

        // Calculate output
        let output =
            self.outputs
                .iter()
                .map(|s| {
                    s.chars().map(|ch| deduced.get(&ch).unwrap()).sorted().collect::<String>()
                })
                .map(|s| NUMMAP.iter().position(|&r| r == s).unwrap())
                .fold(
                    0,
                    |acc, x| acc * 10 + x
                );

        Ok(output)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let signals =
        process_results(
            BufReader::new(file).lines(),
            |iter| {
                iter.map(|s| s.parse::<Signal>())
                    .collect::<Result<Vec<_>, _>>()
            }
        )??;
    
    // Part 1
    // 1, 4, 7, 8 unique segcounts
    let num_uniques =
        signals
            .iter()
            .map(|sig| {
                sig.outputs
                   .iter()
                   .map(&String::len)
                   .filter(|n| UNIQUES.contains(n))
                   .count()
            })
            .sum::<usize>();
    
    println!("Number of uniques: {}", num_uniques);

    // Part 2
    let output_sum = 
        signals
            .iter()
            .map(&Signal::deduce)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum::<usize>();

    println!("Output sum: {}", output_sum);

    Ok(())
}
