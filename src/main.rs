#[macro_use]
extern crate lazy_static;

use std::fs;
use regex::Regex;
use std::str::Lines;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct BaggageRuleParser<'a>
{
    lines: Lines<'a>
}

pub struct BagCount
{
    bag_type: String,
    count: usize
}

pub struct BaggageRule
{
    outer_bag: String,
    contents: Vec<BagCount>
}

pub struct BaggageContainment
{
    rule: usize,
    contained_in: Vec<usize>
}

fn parse_baggage_rule(input: &str) -> BaggageRule
{
    let mut rule = BaggageRule{ outer_bag: String::from(""), contents: Vec::new() };

    lazy_static!
    {
        static ref RE: Regex = Regex::new(r",? |\.").unwrap();
    }

    let mut rule_iter = RE.split(input);
    rule.outer_bag = (&mut rule_iter).take(2).collect::<Vec<&str>>().join(" ");

    // contain
    (&mut rule_iter).next();
    (&mut rule_iter).next();

    while let Some(count) = (&mut rule_iter).next()
    {
        match count
        {
            "" => break,
            "no" => break,
            _ =>
                {
                    let bag_type: String = (&mut rule_iter).take(2).collect::<Vec<&str>>().join(" ");
                    rule.contents.push(BagCount{ bag_type , count: count.parse::<usize>().unwrap() });
                    (&mut rule_iter).next();
                }
        }
    }

    return rule;
}

impl Iterator for BaggageRuleParser<'_>
{
    type Item = BaggageRule;

    fn next(&mut self) -> Option<BaggageRule>
    {
        self.lines.next()
            .map(parse_baggage_rule)
    }
}

fn parse_to_baggage_rules(input: &String) -> BaggageRuleParser { BaggageRuleParser{ lines: input.lines() } }


const DATA_PATH: &str = "C:\\Development\\aoc-2020\\data";

fn main() {
    let input = fs::read_to_string(format!("{}\\{}", DATA_PATH, "day7.txt")).unwrap();
    let rules: Vec<BaggageRule> = parse_to_baggage_rules(&input)
        .collect();

    let rule_size = rules.len();
    let mut rules_lookup = HashMap::new();
    for idx in 0..rule_size
    {
        rules_lookup.insert(rules[idx].outer_bag.clone(), BaggageContainment{ rule: idx, contained_in: Vec::new() });
    }

    for idx in 0..rule_size
    {
        let rule: &BaggageRule = &rules[idx];
        for item in &rule.contents
        {
            rules_lookup.get_mut(&item.bag_type).unwrap().contained_in.push(idx);
        }
    }

    let mut containing_bags: HashSet<usize> = HashSet::new();
    let mut search: Vec<usize> = rules_lookup["shiny gold"].contained_in.clone();
    while let Some(next) = search.pop()
    {
        containing_bags.insert(next);
        rules_lookup[&rules[next].outer_bag].contained_in.iter().for_each(|item | search.push(*item));
    }

    println!("{} bags contain shiny gold", containing_bags.len());
}

#[test]
fn check_parser()
{
    let input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.");
    let rules: Vec<BaggageRule> = parse_to_baggage_rules(&input).collect();
    assert_eq!(rules.len(), 9);

    let rule_0 = &rules[0];
    assert_eq!(rule_0.outer_bag, "light red");
    assert_eq!(rule_0.contents.len(), 2);
    assert_eq!(rule_0.contents[0].bag_type, "bright white");
    assert_eq!(rule_0.contents[0].count, 1);
    assert_eq!(rule_0.contents[1].bag_type, "muted yellow");
    assert_eq!(rule_0.contents[1].count, 2);
}