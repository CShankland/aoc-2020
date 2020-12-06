use std::fs;
use std::str::Lines;
use bitvector::BitVector;
pub struct CustomsDeclarationParser<'a>
{
    lines: Lines<'a>
}

pub struct CustomsDeclaration
{
    answers: BitVector
}

fn parse_to_declaration_group(input: &String) -> CustomsDeclarationParser { CustomsDeclarationParser{ lines: input.lines() } }

fn count_yes_answers(declaration: &CustomsDeclaration) -> usize
{
    declaration.answers.iter().map(|v| v.count_ones()).count()
}

impl Iterator for CustomsDeclarationParser<'_>
{
    type Item = CustomsDeclaration;

    fn next(&mut self) -> Option<CustomsDeclaration>
    {
        let mut result = CustomsDeclaration{ answers: BitVector::ones(26) };
        let mut tmp = CustomsDeclaration{ answers: BitVector::new(26) };
        let mut found_entry = false;
        while let Some(line_data) = self.lines.next()
        {
            if line_data.is_empty()
            {
                break;
            }

            found_entry = true;
            for char in line_data.chars()
            {
                tmp.answers.insert(char as usize - 'a' as usize);
            }

            result.answers = result.answers.intersection(&tmp.answers);
            tmp.answers.clear();
        }

        if found_entry
        {
            return Some(result);
        }
        else
        {
            return None;
        }
    }
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day6.txt")).unwrap();
    let entry_sum: usize = parse_to_declaration_group(&input)
        .map(|declaration| count_yes_answers(&declaration))
        .sum();
    println!("Sum of entries: {}", entry_sum);
}

#[test]
fn check_parser()
{
    let input = String::from("abc\n\na\nb\nc\n\nab\nac\n");
    let groups: Vec<CustomsDeclaration> = parse_to_declaration_group(&input).collect();
    assert_eq!(groups.len(), 3);

    let group_1 = &groups[0];
    let mut b1 = BitVector::new(26);
    for i in vec![0, 1, 2] { b1.insert(i); }
    assert_eq!(b1, group_1.answers);
    assert_eq!(count_yes_answers(&group_1), 3);

    let group_2 = &groups[1];
    let b2 = BitVector::new(26);
    assert_eq!(b2, group_2.answers);
    assert_eq!(count_yes_answers(&group_2), 0);
}