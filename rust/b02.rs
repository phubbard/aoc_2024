use std::io::{self, BufRead, BufReader};

use std::fmt;

#[derive(Debug)]
enum State {
    Safe,
    Unsafe,
}

#[derive(Debug)]
struct Report {
    number: usize,
    levels: Vec<i32>,
    state: State,
}

impl Report {
    pub fn new(number: usize, levels: Vec<i32>) -> Self {
        let mut state: State = State::Safe;

        if levels[0] == levels[1] {
            state = State::Unsafe;
        }

        let multiplier = if levels[0] >= levels[1] { -1 } else { 1 };

        for i in 1..levels.len() {
            let delta = multiplier * (levels[i] - levels[i - 1]);
            if delta < 1 || delta > 3 {
                state = State::Unsafe;
            }
        }

        Report {
            number: number,
            levels: levels,
            state: state,
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "REPORT#{}: {:#?} is {:?}",
            self.number, self.levels, self.state
        )
    }
}

#[derive(Debug)]
struct ProblemSet {
    file_name: &'static str,
    expected_answer_p1: usize,
    expected_answer_p2: usize,
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

        let report = Report::new(line_number, nums);

        reports.push(report);
    }

    let accum_p1 = reports
        .iter()
        .filter(|report| matches!(report.state, State::Safe))
        .count();

    let accum_p2 = 0;

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
            expected_answer_p1: 2,
            expected_answer_p2: 0,
        },
        ProblemSet {
            file_name: "../data/02.txt",
            expected_answer_p1: 299,
            expected_answer_p2: 0,
        },
    ];

    for problem_set in &problem_sets {
        try_set(problem_set)?;
    }

    Ok(())
}
