use std::fmt::Display;


#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    InsufficientDigits,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(err) => write!(f, "{err}"),
            Error::InsufficientDigits => write!(f, "Insufficient digits"),
        }
    }
}

fn day_1_pt1() -> Result<i64, Error> {
    let input = include_str!("input.txt");

    let numbers = input.lines().map(|line| {
        let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
        if digits.is_empty() {
            Err(Error::InsufficientDigits)
        } else {
            match str::parse(&format!("{}{}", digits[0], digits[digits.len() - 1])) {
                Ok(num) => Ok(num),
                Err(err) => Err(Error::Other(Box::new(err))),
            }
        }
    }).collect::<Result<Vec<i64>, Error>>()?;

    Ok(numbers.iter().sum())
}

fn day_1_pt2() -> Result<i64, Error> {
    const NUM_TOKENS: [(&str, &str); 20] = [
        ("0", "0"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("six", "6"),
        ("four", "4"),
        ("zero", "0"),
        ("five", "5"),
        ("nine", "9"),
        ("three", "3"),
        ("seven", "7"),
        ("eight", "8"),
    ];

    let input = include_str!("input.txt");

    let numbers = input.lines().map(|line| {
        let mut digits = Vec::new();
        for start in 0..line.len() {
            for token in NUM_TOKENS {
                if (start + token.0.len()) > line.len() { continue };
                let slice = &line[start..(start + token.0.len())];
                if slice == token.0 {
                    digits.push(token.1);
                    break;
                }
            }
        }
        if digits.is_empty() {
            Err(Error::InsufficientDigits)
        } else {
            match str::parse(&format!("{}{}", digits[0], digits[digits.len() - 1])) {
                Ok(num) => Ok(num),
                Err(err) => Err(Error::Other(Box::new(err))),
            }
        }
    }).collect::<Result<Vec<i64>, Error>>()?;

    Ok(numbers.iter().sum())
}

fn main() {
    println!("Advent of Code 2023");

    let day_1_pt1 = match day_1_pt1() {
        Ok(answer) => answer.to_string(),
        Err(err) => err.to_string(),
    };

    println!("Day 1 (Part 1): {day_1_pt1}");

    let day_1_pt2 = match day_1_pt2() {
        Ok(answer) => answer.to_string(),
        Err(err) => err.to_string(),
    };

    println!("Day 1 (Part 2): {day_1_pt2}");
}
