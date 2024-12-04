use std::env;
use std::fs::File;
use std::io::{ BufReader, Read, Result};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();   
    if args.len() < 2 { usage();  std::process::exit(1); };
    let reader = readfile(&args[1])?;
    
    let mut safe_count: i32 = 0; 

    for (_ln, line_result) in reader.lines().enumerate(){
        let line = line_result;
        let report: Vec<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        let report_is_safe = part1(report);
        if report_is_safe { 
            safe_count += 1;
        };
    }   

    
    let part1_ans= safe_count;

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

fn part1(vec_a: Vec<i32>) -> bool { 
    let mut deltas: Vec<i32> = Vec::new();
    let mut asc = true;
    let mut desc = true;    

    for w in vec_a.windows(2) {
        deltas.push((w[0] - w[1]).abs());
        asc  &= w[0] < w[1];
        desc &= w[0] > w[1];
    }
    
    let min = *deltas.iter().min().unwrap();
    let max = *deltas.iter().max().unwrap();
    let rate_safety = min > 0 && max < 3;

   rate_safety && ( asc ^ desc ) 
}

// fn part2(vec_a: &[i32], vec_b: &[i32]) -> i32 { 
//     vec_a.iter()
//         .filter(|&a|)

// }

