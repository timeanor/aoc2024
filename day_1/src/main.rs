use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();    
    let file_path = Path::new(&args[1]);
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let mut array_a: Vec<i32> = Vec::new();
    let mut array_b: Vec<i32> = Vec::new();

    
    for (_ln, line_result) in reader.lines().enumerate(){
        let l = line_result?;

        let parts: Vec<&str> = l.splitn(2, "   ").collect();
        
        let left = parts[0].trim();
        let right = parts[1].trim();

        if let Ok(num) = left.parse::<i32>(){
            array_a.push(num); 
        }
        if let Ok(num) = right.parse::<i32>(){
            array_b.push(num); 
        }
    }

    let mut sorted_a = array_a.clone();
    sorted_a.sort();
    
    let mut sorted_b = array_b.clone();
    sorted_b.sort();

    
    let part1_ans= part1(&sorted_a, &sorted_b);

    println!("Answer: {part1_ans}");
    Ok(())
}

fn part1(vec_a: &[i32], vec_b: &[i32]) -> i32 { 
    vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part1(vec_a: &[i32], vec_b: &[i32]) -> i32 { 
    
}

