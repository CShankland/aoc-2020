#[macro_use]
extern crate lazy_static;

use std::fs;
use regex::Regex;
use std::str::Lines;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::AocError::ParseError;

pub enum AocError
{
    ParseIntError(ParseIntError),
    ParseError,
}

impl From<ParseIntError> for AocError
{
    fn from(error: ParseIntError) -> Self
    {
        AocError::ParseIntError(error)
    }
}

pub struct PassportParser<'a>
{
    lines: Lines<'a>
}

pub struct Height
{
    units: String,
    value: i32
}

fn invalid_height() -> Height
{
    Height { units: String::from("none"), value: 0 }
}

impl FromStr for Height
{
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        lazy_static!
        {
            static ref RE: Regex = Regex::new(r"(?P<value>\d+)(?P<units>.+)").unwrap();
        }

        if let Some(captures) = RE.captures(s)
        {
            let value = captures["value"].to_string().parse::<i32>()?;
            let units = captures["units"].to_string();

            Ok(Height{ units, value })
        }
        else
        {
            Err(ParseError)
        }
    }
}

pub struct PassportData
{
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

fn parse_to_passports(input: &String) -> PassportParser
{
    PassportParser{ lines: input.lines() }
}

fn populate_passport_field(passport: &mut PassportData, field_data: &str)
{
    if let Some(colon_index) = field_data.find(":")
    {
        let field = &field_data[..colon_index];
        let value: String = field_data[colon_index+1..].to_string();

        println!("Parsing field {} : {}", field, value);

        match field
        {
            "byr" => passport.byr = value.parse::<i32>().unwrap_or(0),
            "iyr" => passport.iyr = value.parse::<i32>().unwrap_or(0),
            "eyr" => passport.eyr = value.parse::<i32>().unwrap_or(0),
            "hgt" => passport.hgt = value.parse::<Height>().unwrap_or(invalid_height()),
            "hcl" => passport.hcl = value,
            "ecl" => passport.ecl = value,
            "pid" => passport.pid = value,
            "cid" => passport.cid = value,

            _ => println!("Unknown field {}", field)
        }
    }
    else
    {
        println!("No colon in data {}", field_data);
    }
}

impl Iterator for PassportParser<'_>
{
    type Item = PassportData;

    fn next(&mut self) -> Option<PassportData>
    {
        let mut maybe_line = self.lines.next();
        if maybe_line.is_none()
        {
            return None;
        }

        let mut result: PassportData = PassportData {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: invalid_height(),
            hcl: String::from(""),
            ecl: String::from(""),
            pid: String::from(""),
            cid: String::from("")
        };

        while let Some(line) = maybe_line
        {
            println!("Read line {}", line);

            if line.is_empty()
            {
                println!("Passport data complete.");
                break;
            }

            line.split_ascii_whitespace()
                .for_each(|key_value| populate_passport_field(&mut result, key_value));

            maybe_line = self.lines.next();
        }

        return Some(result);
    }
}

fn between(value: i32, min: i32, max: i32) -> bool
{
    return value >= min && value <= max;
}

fn valid_height(height: &Height) -> bool
{
    match height.units.as_ref()
    {
        "cm" => between(height.value, 150, 193),
        "in" => between(height.value, 59, 76),
        _ => false
    }
}

fn valid_hair_color(value: &str) -> bool
{
    lazy_static!
    {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    RE.is_match(value)
}

fn valid_eye_color(value: &str) -> bool
{
    match value
    {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn valid_passport_id(value: &str) -> bool
{
    lazy_static!
    {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    RE.is_match(value)
}

fn is_valid(passport: &PassportData) -> bool
{
    let mut valid: bool = true;

    valid = valid && between(passport.byr, 1920, 2002);
    valid = valid && between(passport.iyr, 2010, 2020);
    valid = valid && between(passport.eyr, 2020, 2030);
    valid = valid && valid_height(&passport.hgt);
    valid = valid && valid_hair_color(&passport.hcl);
    valid = valid && valid_eye_color(&passport.ecl);
    valid = valid && valid_passport_id(&passport.pid);

    return valid;
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day4.txt")).unwrap();
    let valid_passports = parse_to_passports(&input)
        .filter(is_valid)
        .count();

    println!("Valid passports: {}", valid_passports);
}
