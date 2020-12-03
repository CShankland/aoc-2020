#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::num;

enum AocError
{
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for AocError
{
    fn from(error: io::Error) -> Self
    {
        AocError::IoError(error)
    }
}

impl From<num::ParseIntError> for AocError
{
    fn from(error: num::ParseIntError) -> Self
    {
        AocError::ParseError(error)
    }
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

struct PasswordEntry
{
    password: String,
    required: char,
    first_index: usize,
    second_index: usize
}

fn main() {
    let valid_passwords = read_lines(format!("{}\\{}", DATA_PATH, "day2.txt"))
        .unwrap()
        .map(|line| parse_password_entry(&line.unwrap()))
        .filter_map(Result::ok)
        .map(|entry| verify_password(&entry))
        .fold(0, |acc, x| acc + x as i32);

    println!("Valid passwords: {}", valid_passwords);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_password_entry(input: &String) -> Result<PasswordEntry, AocError>
{
    lazy_static!
    {
        // 2-9 c: ccccccccc
        static ref RE: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<req>\w): (?P<pwd>\w+)").unwrap();
    }

    let parse_result = RE.captures(input).unwrap();
    let first_index = parse_result["min"].to_string().parse::<usize>()?;
    let second_index = parse_result["max"].to_string().parse::<usize>()?;

    Ok(PasswordEntry
    {
        password: parse_result["pwd"].to_string(),
        required: parse_result["req"].to_string().chars().nth(0).unwrap(),
        first_index,
        second_index,
    })
}

fn verify_password(password_data: &PasswordEntry) -> bool
{
    let has_first = password_data.password.chars().nth(password_data.first_index - 1).map_or(false, |c| c == password_data.required);
    let has_second = password_data.password.chars().nth(password_data.second_index - 1).map_or(false, |c| c == password_data.required);

    return has_first ^ has_second;
}
