use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let mut expense_entries = Vec::new();

    let filename = format!("{}\\{}", DATA_PATH, "day1.txt");
    let lines_iter = read_lines(&filename);
    if lines_iter.is_err()
    {
        println!("Failed to open {}: {}", &filename, lines_iter.unwrap_err().to_string());
    }
    else
    {
        for line in lines_iter.unwrap()
        {
            if let Ok(entry) = line
            {
                expense_entries.push(entry.parse::<i32>().unwrap());
            }
        }
    }

    let mut outer = expense_entries.iter();
    while let Some(a) = outer.next()
    {
        let mut middle = outer.clone();
        while let Some(b) = middle.next()
        {
            let inner = middle.clone();
            for c in inner
            {
                if a + b + c == 2020
                {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
