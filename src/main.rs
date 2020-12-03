use std::fs;
use std::str::Lines;

pub struct TobogganPath<'a>
{
    lines: Lines<'a>,
    offset: usize,
    skip: usize,
    right: usize,
    down: usize,
}

impl Iterator for TobogganPath<'_>
{
    type Item = char;

    fn next(&mut self) -> Option<char>
    {
        let result = self.lines.nth(self.skip)
            .map_or(None, |l| l.chars().cycle().nth(self.offset));
        self.offset += self.right;
        self.skip = self.down - 1;
        result
    }
}

fn compute_toboggan_path(input: &String, right: usize, down: usize) -> TobogganPath
{
    TobogganPath{ lines: input.lines(), offset: 0, skip: 0, right, down }
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn is_tree(terrain: &char) -> bool
{
    *terrain == '#'
}

fn count_trees(input: &String, right: usize, down: usize) -> usize
{
    compute_toboggan_path(&input, right, down)
        .filter(is_tree)
        .count()
}

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day3.txt")).unwrap();
    let trees_hit =
        count_trees(&input, 1, 1) *
        count_trees(&input, 3, 1) *
        count_trees(&input, 5, 1) *
        count_trees(&input, 7, 1) *
        count_trees(&input, 1, 2);

    println!("Trees hit: {}", trees_hit);
}
