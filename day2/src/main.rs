use std::{env, error::Error, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Failed to read input");
    }

    let file = File::open(&args[1])?;
    let mut pos = 0;
    let mut depth = 0;
    for line in BufReader::new(file).lines() {
        if let Ok(cmdstr) = line {
            let cmd: Vec<&str> = cmdstr.split(' ').collect();
            let amt: i32 = cmd[1].parse()?;
            match cmd[0] {
                "forward" => pos += amt,
                "up" => depth -= amt,
                "down" => depth += amt,
                _ => panic!("Unknown command")
            }
        }
    }

    println!("pos x depth: {}", pos * depth);

    let file = File::open(&args[1])?;
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in BufReader::new(file).lines() {
        if let Ok(cmdstr) = line {
            let cmd: Vec<&str> = cmdstr.split(' ').collect();
            let amt: i32 = cmd[1].parse()?;
            match cmd[0] {
                "forward" => {
                    pos += amt;
                    depth += aim * amt;
                },
                "up" => aim -= amt,
                "down" => aim += amt,
                _ => panic!("Unknown command")
            }
        }
    }

    println!("real pos x depth: {}", pos * depth);

    Ok(())
}