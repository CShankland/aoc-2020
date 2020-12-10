use std::fs;
use std::str::Lines;

struct AocParser<'a>
{
    lines: Lines<'a>
}

impl Iterator for AocParser<'_>
{
    type Item = i64;

    fn next(&mut self) -> Option<i64>
    {
        self.lines.next()
            .map(|line| line.trim())
            .map(|str| str.parse::<i64>().unwrap())
    }
}

fn parse_input(input: &String) -> AocParser { AocParser{ lines: input.lines() } }

fn is_sum(target: i64, window: &Vec<i64>) -> bool
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

fn find_min_max(window: &Vec<i64>, start: usize, end: usize) -> (i64, i64)
{
    let mut min = core::i64::MAX;
    let mut max = core::i64::MIN;

    for idx in start..=end
    {
        let element = window[idx];
        min = std::cmp::min(min, element);
        max = std::cmp::max(max, element);
    }

    return (min, max);
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day9.txt")).unwrap();
    let input_values: Vec<i64> = parse_input(&input).collect();
    let mut input_stream = input_values.iter();

    let mut window = Vec::new();
    input_stream.by_ref().take(25).for_each(|val| window.push(*val));

    while let Some(val) = input_stream.next()
    {
        if !is_sum(*val, &window)
        {
            let mut start_idx: usize = 0;
            let mut end_idx: usize = 1;
            let mut sum = input_values[0] + input_values[1];
            loop
            {
                if sum == *val
                {
                    let (min, max) = find_min_max(&input_values, start_idx, end_idx);
                    println!("Result: {}", min + max);
                    return;
                }
                else if sum > *val
                {
                    sum -= input_values[start_idx];
                    start_idx += 1;
                    if start_idx == end_idx
                    {
                        end_idx += 1;
                        if end_idx >= input_values.len()
                        {
                            println!("Failed to find result.");
                            return;
                        }

                        sum += input_values[end_idx];
                    }
                }
                else
                {
                    end_idx += 1;
                    if end_idx >= input_values.len()
                    {
                        println!("Failed to find result.");
                        return;
                    }

                    sum += input_values[end_idx];
                }
            }
        }

        window.remove(0);
        window.push(*val);
    }

    println!("Found nothing.");
}
