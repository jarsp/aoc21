use std::{env, error::Error, fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Insufficient arguments");
    }

    // Part 1
    let file = File::open(&args[1])?;
    let mut old_depth = 0;
    let mut inc_count = -1;
    for line in BufReader::new(file).lines() {
        if let Ok(nstr) = line {
            let new_depth: i32 = nstr.parse()?;
            if new_depth > old_depth {
                inc_count += 1;
            }
            old_depth = new_depth;
        }
    }

    println!("Increased measurements: {}", inc_count);

    
    // Part 2
    let file = File::open(&args[1])?;
    let mut old_depths = [0, 0, 0];
    let mut inc3_count = -3;
    for line in BufReader::new(file).lines() {
        if let Ok(nstr) = line {
            let new_depth: i32 = nstr.parse()?;
            if new_depth > old_depths[0] {
                inc3_count += 1;
            }
            old_depths[0] = old_depths[1];
            old_depths[1] = old_depths[2];
            old_depths[2] = new_depth;
        }
    }

    println!("Increased window measurements: {}", inc3_count);
    
    Ok(())
}