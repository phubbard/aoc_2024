use std::io::{self, BufRead, BufReader};

use std::fmt;

#[derive(Debug)]
struct Report {
    number: usize,
    levels: Vec<i32>,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "REPORT#{}: {:#?}", self.number, self.levels,)
    }
}

#[derive(Debug)]
struct ProblemSet {
    file_name: &'static str,
    expected_answer_p1: i32,
    expected_answer_p2: i32,
}

fn try_set(problem_set: &ProblemSet) -> Result<(), std::io::Error> {
    println!("trying open with {}", problem_set.file_name);
    let file = std::fs::File::open(problem_set.file_name)?;
    println!("Good open with {}", problem_set.file_name);

    let reader = BufReader::new(file);

    let mut reports: Vec<Report> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        let nums = line
            .split_whitespace()
            .map(|s| {
                s.parse::<i32>().map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let report = Report {
            number: line_number,
            levels: nums,
        };

        reports.push(report);
    }

    let accum_p1 = -1;
    let accum_p2 = -1;

    if problem_set.expected_answer_p1 != accum_p1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Part one expected {} but saw {}",
                problem_set.expected_answer_p1, accum_p1
            ),
        ));
    }

    if problem_set.expected_answer_p2 != accum_p2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Part two expected {} but saw {}",
                problem_set.expected_answer_p2, accum_p1
            ),
        ));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let problem_sets: [ProblemSet; 2] = [
        ProblemSet {
            file_name: "../data/02s.txt",
            expected_answer_p1: 11,
            expected_answer_p2: 31,
        },
        ProblemSet {
            file_name: "../data/02.txt",
            expected_answer_p1: 3569916,
            expected_answer_p2: 26407426,
        },
    ];

    for problem_set in &problem_sets {
        try_set(problem_set)?;
    }

    Ok(())
}
