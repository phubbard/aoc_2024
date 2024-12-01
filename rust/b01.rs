use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::fmt;


#[derive(Debug)]
struct LocationID {
    value: i32,
    position_number: usize,
}


impl fmt::Display for LocationID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {} (position_number: {})", self.value, self.position_number)
    }
}


fn parse_number(num_str: &str, position_number: usize) -> Result<LocationID, io::Error> {
    let num_int = num_str.parse::<i32>().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Line {}: First number invalid: {}", position_number + 1, e)
        )
    })?;

    let location_id = LocationID { value: num_int, position_number: position_number };

    Ok(location_id)
}

fn main() -> io::Result<()> {
    let file = File::open("../data/01-s.txt")?;
    let reader = BufReader::new(file);

    let mut numbers1: Vec<LocationID> = Vec::new();
    let mut numbers2: Vec<LocationID> = Vec::new();

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

    println!("First array:  {:#?}", numbers1);
    println!("Second array: {:#?}", numbers2);

    Ok(())
}

