use std::env;
use std::fs::File;
use std::io::{ BufReader, Read, Result};
use std::path::Path;
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();   
    if args.len() < 2 { usage();  std::process::exit(1); };
    let reader = readfile(&args[1])?;
    

    let re = Regex::new(r"(mul\(\d+,\d+\))|(do(?:n't)?\(\))").unwrap();

    let matches: Vec<&str> = re.find_iter(&reader).map(|m| m.as_str()).collect();

    let part1_ans = part1(matches);

    println!("Answer: {part1_ans}");
    Ok(())
}

fn usage() -> () {
    eprintln!(" invalid number of arguments");   
}

fn readfile(filepath: &str) -> Result<String> {
    let mut reader = open_file_as_reader(filepath)?;
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn open_file_as_reader(filepath: &str) -> Result<BufReader<File>> {
    let file_path = Path::new(filepath);
    let file = File::open(&file_path)?;
    Ok(BufReader::new(file))
}

fn bothparts(matches: Vec<&str>) -> (i32,i32) { 
    let part1: i32 = 0;
    let part2: i32 = 0;

    let enabled: bool = true;

    for m in matches.iter() {
       if m.contains("do()") => enabled = true;
       if m.contains("don't()") => enabled = true;
    }

    (part1, part2)
}

fn multiply(mul: str) -> i32 {
    let mul = Regex::new(r"mul\((?<l>\d+),(?<r>\d+)\)").unwrap();
}
