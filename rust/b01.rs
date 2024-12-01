use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("../data/01-s.txt")?;
    let reader = BufReader::new(file);

    let mut numbers1: Vec<i32> = Vec::new();
    let mut numbers2: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        // Split the line by whitespace and collect into a vector
        let nums: Vec<&str> = line.split_whitespace().collect();
        
        if nums.len() >= 2 {
            // Parse the numbers and add them to their respective arrays
            if let Ok(num1) = nums[0].parse::<i32>() {
                if let Ok(num2) = nums[1].parse::<i32>() {
                    numbers1.push(num1);
                    numbers2.push(num2);
                }
            }
        }
    }

    // Print out the arrays
    println!("First array: {:?}", numbers1);
    println!("Second array: {:?}", numbers2);

    Ok(())
}

