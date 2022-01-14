use std::{env, fs::File, error::Error, io::{BufReader, BufRead}, collections::{HashMap, HashSet}, str::FromStr, rc::Rc};

use itertools::{Itertools, process_results};

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Clone)]
enum Cave {
    Big(Rc<String>),
    Small(Rc<String>),
    Start,
    End,
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            n if n == n.to_ascii_uppercase() => Ok(Cave::Big(s.to_string().into())),
            _ => Ok(Cave::Small(s.to_string().into())),
        }
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    fn parse_system<I: IntoIterator<Item = String>>(iter: I) -> Result<Self, &'static str> {
        let mut caves: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for s in iter {
            let v: Vec<Cave> = s.split('-').map(&str::parse::<Cave>).try_collect()?;
            if v.len() != 2 {
                return Err("Not a pair")
            }

            caves.entry(v[0].clone()).or_insert_with(Vec::new).push(v[1].clone());
            caves.entry(v[1].clone()).or_insert_with(Vec::new).push(v[0].clone());
        }

        Ok(CaveSystem {caves})
    }

    fn paths(&self, quota: u32) -> i32 {
        self.paths_(Cave::Start, &mut HashSet::new(), quota)
    }

    fn paths_(&self, node: Cave, visited: &mut HashSet<Cave>, quota: u32) -> i32 {
        if node == Cave::End {
            return 1
        }

        let is_start = node == Cave::Start;
        let is_small = matches!(node, Cave::Small(_));

        let seen = visited.contains(&node);
        let mut new_quota = quota;
        if seen {
            if is_start {
                return 0
            } else if is_small {
                if quota == 0 {
                    return 0
                }
                new_quota -= 1;
            }
        }

        if is_start || is_small {
            visited.insert(node.clone());
        }

        let p: i32 =
            self.caves
                .get(&node)
                .unwrap()
                .iter()
                .map(|n| self.paths_(n.clone(), visited, new_quota))
                .sum();
        
        if (is_start || is_small) && new_quota == quota {
            visited.remove(&node);
        }

        p
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect_vec();
    if args.len() != 2 {
        panic!("Could not read input");
    }

    let file = File::open(&args[1])?;
    let graph =
        process_results(
            BufReader::new(file).lines(),
            |iter| CaveSystem::parse_system(iter)
        )??;
    
    // Part 1
    println!("Paths: {}", graph.paths(0));

    // Part 2
    println!("Paths with quota 1: {}", graph.paths(1));
    
    Ok(())
}
