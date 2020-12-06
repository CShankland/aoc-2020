use std::fs;
use std::str::Lines;

pub struct SeatParser<'a>
{
    lines: Lines<'a>
}

pub struct SeatData
{
    row: i32,
    column: i32
}

fn parse_to_seat(input: &String) -> SeatParser
{
    SeatParser{ lines: input.lines() }
}

fn compute_bit(letter: char) -> i32
{
    match letter
    {
        'B' => 1,
        'R' => 1,
        _ => 0
    }
}

impl Iterator for SeatParser<'_>
{
    type Item = SeatData;

    fn next(&mut self) -> Option<SeatData>
    {
        self.lines.next()
            .map(|line| line.chars().fold(0, |acc, c| (acc << 1) + compute_bit(c)))
            .map(|value| SeatData{ row: value >> 3, column: value & 0x7 })
    }
}

fn compute_seat_id(data: &SeatData) -> i32
{
    data.row * 8 + data.column
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day5.txt")).unwrap();
    let mut seats: Vec<i32> = parse_to_seat(&input)
        .map(|seat| compute_seat_id(&seat))
        .collect();
    seats.sort();
    let mut seat_iter = seats.iter().peekable();
    while let Some(seat) = seat_iter.next()
    {
        if let Some(next_seat) = seat_iter.peek()
        {
            if **next_seat == *seat + 2
            {
                println!("Available seat: {}", *seat + 1);
            }
        }
    }
}

#[test]
fn check_parser()
{
    let input = String::from("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");
    let seats: Vec<SeatData> = parse_to_seat(&input).collect();
    assert_eq!(3, seats.len());

    let seat_1 = &seats[0];
    assert_eq!(seat_1.row, 70);
    assert_eq!(seat_1.column, 7);
    assert_eq!(compute_seat_id(seat_1), 567);

    let seat_2 = &seats[1];
    assert_eq!(seat_2.row, 14);
    assert_eq!(seat_2.column, 7);
    assert_eq!(compute_seat_id(seat_2), 119);

    let seat_3 = &seats[2];
    assert_eq!(seat_3.row, 102);
    assert_eq!(seat_3.column, 4);
    assert_eq!(compute_seat_id(seat_3), 820);
}
