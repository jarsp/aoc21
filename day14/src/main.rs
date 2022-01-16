use std::{env, fs::File, error::Error, collections::HashMap, io::{BufReader, BufRead}};

use itertools::{Itertools, process_results};

type Counts = HashMap<char, usize>;

trait CountsArith {
    fn plus(&mut self, w: &Self);
}

impl CountsArith for Counts {
    fn plus(&mut self, w: &Counts) {
        w.iter()
            .for_each(|(&c, &n)| *self.entry(c).or_insert(0) += n);
    }
}

#[derive(Debug, Clone)]
struct Polymer {
    template: String,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn parse_polymer<I>(iter: I)
        -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = String>
    {
        let mut it = iter.into_iter();
        let template = it.next().ok_or("error no template")?;

        let rules: HashMap<(char, char), char> =
            it.skip(1)
              .map(|s| {
                  let v = s.split(" -> ").collect_vec();
                  if v.len() != 2 || v[0].len() != 2 || v[1].len() != 1 {
                      Err("bad rule")
                  } else {
                      let mut l = v[0].chars();
                      let l1 = l.next().unwrap();
                      let l2 = l.next().unwrap();
                      let r1 = v[1].chars().next().unwrap();
                      Ok(((l1, l2), r1))
                  }
              })
              .try_collect()?;
        
        Ok(Polymer {template, rules})
    }

    fn counts_table(&self, n: usize) -> HashMap<(char, char), Counts> {
        // F(c, d, n) -> counts of chars appearing after n steps, not counting the last char
        // F(c, d, 0) -> {c |-> 1}
        // F(c, d, i) -> {
        //     if rule cd -> e: F(c, e, i - 1) + F(e, d, i - 1)
        //     else: F(c, d, 0)
        // }

        let mut counts: HashMap<(char, char), Counts> =
            self.rules
                .keys()
                .map(|&t| (t, HashMap::from([(t.0, 1)])))
                .collect();
        
        for _ in 1..n + 1 {
            counts = 
                self.rules
                    .iter()
                    .map(|(&(c, d), &e)| {
                        let mut ce =
                            match counts.get(&(c, e)) {
                                Some(cs) => cs.clone(),
                                None => HashMap::from([(c, 1)]),
                            };
                        let ed =
                            match counts.get(&(e, d)) {
                                Some(cs) => cs.clone(),
                                None => HashMap::from([(e, 1)]),
                            };
                        ce.plus(&ed);
                        ((c, d), ce)
                    })
                    .collect();
        }

        counts
    }

    fn dynamic_count(&self, n: usize) -> Counts {
        let counts_table = self.counts_table(n);
        let mut counts: Counts = HashMap::new();
        self.template
            .chars()
            .zip(self.template.chars().skip(1))
            .for_each(|(c, d)| {
                match counts_table.get(&(c, d)) {
                    Some(cs) => counts.plus(cs),
                    None => counts.plus(&HashMap::from([(c, 1)])),
                }
            });
        counts.plus(&HashMap::from([(self.template.chars().nth_back(0).unwrap(), 1)]));

        counts
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let polymer =
        process_results(
            BufReader::new(file).lines(),
            |iter| Polymer::parse_polymer(iter)
        )??;

    // Part 1
    let stats =
        polymer.dynamic_count(10)
               .into_iter()
               .sorted_by_key(|e| e.1)
               .collect_vec();

    println!("Difference: {}", stats[stats.len() - 1].1 - stats[0].1);

    // Part 2
    let stats40 =
        polymer.dynamic_count(40)
               .into_iter()
               .sorted_by_key(|e| e.1)
               .collect_vec();
    println!("{:?}", stats40);

    println!("Difference 40: {}", stats40[stats40.len() - 1].1 - stats40[0].1);

    Ok(())
}
