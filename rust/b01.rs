use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn parse_number(num_str: &str, line_number: usize) -> Result<i32, io::Error> {
    let num_int = num_str.parse::<i32>().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Line {}: First number invalid: {}", line_number + 1, e)
        )
    })?;

    Ok(num_int)
}

fn main() -> io::Result<()> {
    let file = File::open("../data/01-s.txt")?;
    let reader = BufReader::new(file);

    let mut numbers1: Vec<i32> = Vec::new();
    let mut numbers2: Vec<i32> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let nums: Vec<&str> = line.split_whitespace().collect();
        
        // Check for exactly 2 numbers
        if nums.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Line {} does not contain exactly 2 numbers", line_number + 1)
            ));
        }

        // Parse both numbers, returning error if either fails
        let num1 = parse_number(nums[0], line_number)?;
        
        let num2 = parse_number(nums[1], line_number)?;

        numbers1.push(num1);
        numbers2.push(num2);
    }

    println!("First array: {:?}", numbers1);
    println!("Second array: {:?}", numbers2);

    Ok(())
}

