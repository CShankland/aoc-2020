use std::fs;
use std::str::Lines;

struct AocParser<'a>
{
    lines: Lines<'a>
}

impl Iterator for AocParser<'_>
{
    type Item = i32;

    fn next(&mut self) -> Option<i32>
    {
        self.lines.next()
            .map(|line| line.trim())
            .map(|str| str.parse::<i32>().unwrap())
    }
}

fn parse_input(input: &String) -> AocParser { AocParser{ lines: input.lines() } }

fn is_sum(target: i32, window: &Vec<i32>) -> bool
{
    for val in window.iter()
    {
        if window.contains(&(target - val))
        {
            return true;
        }
    }

    return false;
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day9.txt")).unwrap();
    let mut input_stream = parse_input(&input);

    let mut window = Vec::new();
    input_stream.by_ref().take(25).for_each(|val| window.push(val));

    while let Some(val) = input_stream.next()
    {
        if !is_sum(val, &window)
        {
            println!("Found {}", val);
            return;
        }

        window.remove(0);
        window.push(val);
    }

    println!("Found nothing.");
}
