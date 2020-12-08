#[macro_use]
extern crate lazy_static;

use std::fs;
use regex::Regex;
use std::str::Lines;
use std::collections::HashSet;

#[derive(Debug)]
#[derive(PartialEq)]
enum OpCode
{
    Unknown,
    Noop,
    Accumulate,
    Jump,
}

struct Instruction
{
    operation: OpCode,
    value: i32
}

struct Program
{
    instructions: Vec<Instruction>,
    instruction_pointer: i32,
    accumulator: i32,
}

struct AocParser<'a>
{
    lines: Lines<'a>
}

impl Iterator for AocParser<'_>
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction>
    {
        self.lines.next()
            .map(|line| line.trim())
            .map(parse_instruction)
    }
}

fn parse_instruction(input: &str) -> Instruction
{
    lazy_static!
    {
        static ref RE: Regex = Regex::new(r"(\w+) ([+-]\d+)").unwrap();
    }

    let captures = RE.captures(input).unwrap();
    let operation = match captures.get(1).unwrap().as_str()
    {
        "nop" => OpCode::Noop,
        "acc" => OpCode::Accumulate,
        "jmp" => OpCode::Jump,
        _ => OpCode::Unknown
    };
    let value = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

    return Instruction{ operation, value };
}

fn parse_input(input: &String) -> AocParser { AocParser{ lines: input.lines() } }

fn run_program(instructions: Vec<Instruction>, evaluation_callback: &mut dyn FnMut(&Program) -> bool) -> i32
{
    let mut program = Program{ instructions, instruction_pointer: 0, accumulator: 0 };

    while evaluation_callback(&program)
    {
        let instruction: &Instruction = &program.instructions[program.instruction_pointer as usize];
        match instruction.operation
        {
            OpCode::Noop => program.instruction_pointer += 1,
            OpCode::Accumulate =>
                {
                    program.accumulator += instruction.value;
                    program.instruction_pointer += 1;
                },
            OpCode::Jump =>
                {
                    program.instruction_pointer += instruction.value;
                },
            _ => panic!("Unknown instruction")
        }
    }

    return program.accumulator;
}

const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let instructions: Vec<Instruction> = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day8.txt"))
        .map(|input| parse_input(&input).collect())
        .unwrap();

    let mut evaluated_instructions: HashSet<i32> = HashSet::new();
    let mut prevent_loop = |program: &Program|
    {
        if evaluated_instructions.contains(&program.instruction_pointer)
        {
            return false;
        }

        evaluated_instructions.insert(program.instruction_pointer);
        return true;
    };

    let result = run_program(instructions, &mut prevent_loop);
    println!("Accumulator ended at {}", result);
}

#[test]
fn check_parser()
{
    let input = String::from(r#"nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6"#);

    let instructions: Vec<Instruction> = parse_input(&input).collect();
    assert_eq!(instructions.len(), 9);

    let item_0 = &instructions[0];
    assert_eq!(item_0.operation, OpCode::Noop);
    assert_eq!(item_0.value, 0);

    let item_1 = &instructions[1];
    assert_eq!(item_1.operation, OpCode::Accumulate);
    assert_eq!(item_1.value, 1);

    let item_2 = &instructions[2];
    assert_eq!(item_2.operation, OpCode::Jump);
    assert_eq!(item_2.value, 4);

    let item_3 = &instructions[3];
    assert_eq!(item_3.operation, OpCode::Accumulate);
    assert_eq!(item_3.value, 3);

    let item_4 = &instructions[4];
    assert_eq!(item_4.operation, OpCode::Jump);
    assert_eq!(item_4.value, -3);
}