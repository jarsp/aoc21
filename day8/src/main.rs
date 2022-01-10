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
        let mut vs: Vec<Vec<String>> =
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
        
        Ok(Signal {digits: digits_map, outputs: vs.swap_remove(1)})
    }
}

impl Signal {
    fn deduce(&self) -> Result<usize, Box<dyn Error>> {
        let mut deduced: HashMap<char, char> = HashMap::new();

        // Identify 1, 4, 7, 8
        let uniques: Vec<_> =
            UNIQUES.iter()
                   .map(|l| {
                       self.digits
                           .get(l)
                           .ok_or("unique not found")
                           .and_then(|v| {
                               if v.len() != 1 {
                                   Err("unique wrong number")
                               } else {
                                   Ok(&v[0])
                               }
                           })
                   })
                   .try_collect()?;
        
        fn helper<'a, 'b: 'a, F, I>(digits: &'b HashMap<usize, Vec<HashSet<char>>>, sc: usize, f: F)
            -> Result<char, &'static str>
        where
            F: Fn(&'b HashSet<char>) -> I,
            I: Iterator<Item = &'a char>
        {
            let val =
                digits
                    .get(&SEGCOUNTS[sc])
                    .ok_or("could not find")?
                    .iter()
                    .map(|digit| f(digit).copied().collect_vec())
                    .find(|v| v.len() == 1)
                    .ok_or("could not deduce")?[0];
            Ok(val)
        }

        // Deduce a from 1 and 7
        let a = *uniques[2].difference(&uniques[0]).exactly_one().map_err(|_| "not unique")?;
        deduced.insert(a, 'a');

        // Deduce b, d from 4 and 0
        let d_cands: HashSet<_> = uniques[1].difference(&uniques[0]).copied().collect();
        let d = helper(&self.digits, 0, |digit| d_cands.difference(digit))?;
        let b = *d_cands.iter().find(|&&cand| cand != d).ok_or("could not find")?;
        deduced.insert(b, 'b');
        deduced.insert(d, 'd');

        // Deduce c, f from 1 and 6
        let c = helper(&self.digits, 6, |digit| uniques[0].difference(digit))?;
        let f = *uniques[0].iter().find(|&&cand| cand != c).ok_or("could not find")?;
        deduced.insert(c, 'c');
        deduced.insert(f, 'f');

        // Deduce g from 9
        let mut found: HashSet<char> = HashSet::from_iter(deduced.keys().copied());
        let g = helper(&self.digits, 9, |digit| digit.difference(&found))?;
        deduced.insert(g, 'g');

        // Deduce e
        found.insert(g);
        let all = HashSet::from_iter("abcdefg".chars());
        let e = *all.difference(&found).exactly_one().map_err(|_| "could not deduce e")?;
        deduced.insert(e, 'e');

        // Calculate output
        let output =
            self.outputs
                .iter()
                .map(|s| -> Result<String, _> {
                    s.chars()
                     .map(|ch| deduced.get(&ch).ok_or("char not deduced"))
                     .sorted()
                     .try_collect()
                })
                .map(|rs| {
                    rs.and_then(|s| {
                        NUMMAP.iter()
                              .position(|&r| r == s)
                              .ok_or("could not find number")
                    })
                })
                .fold_ok(
                    0,
                    |acc, x| acc * 10 + x
                )?;

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
            |iter| -> Result<Vec<_>, _> {
                iter.map(|s| s.parse::<Signal>())
                    .try_collect()
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
