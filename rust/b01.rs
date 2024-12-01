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


#[derive(Debug)]
struct ProblemSet {
    file_name: &'static str,
    expected_answer: i32
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


fn try_set(problem_set: &ProblemSet) -> Result<(), std::io::Error> {

    let file = std::fs::File::open(problem_set.file_name)?;
    println!("Good open with {}", problem_set.file_name);

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

    // println!("First array:  {:#?}", numbers1);
    // println!("Second array: {:#?}", numbers2);

    numbers1.sort_by_key( |loc| loc.value );
    numbers2.sort_by_key( |loc| loc.value );

    // println!("SORTED First array:  {:#?}", numbers1);
    // println!("SORTED Second array: {:#?}", numbers2);

    let mut accumulator: i32 = 0;

    for i in 0..numbers1.len() {

        let a = numbers1[i].value;
        let b = numbers2[i].value;

        // println!("At: {} considering {} and {}", i, a, b);

        let diff = if a > b 
                    { a - b } else
                        { b - a };

        accumulator = accumulator + diff;
        // println!("At: {} the accumulator is now {}", i, accumulator);
    }

    println!("At end: the accumulator is now {}", accumulator);

    if problem_set.expected_answer != accumulator {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Expected {} but saw {}", problem_set.expected_answer, accumulator)
        ));
    }

    Ok(())
}


fn main() -> io::Result<()> {

    let problem_sets: [ProblemSet; 2] = [
        ProblemSet { file_name: "../data/01-s.txt", expected_answer: 11 },
        ProblemSet { file_name: "../data/01.txt",   expected_answer: 3569916 },
    ];

    for problem_set in &problem_sets {
        try_set(problem_set)?;
    }

    Ok(())
}

