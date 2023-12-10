#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day08/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let (instructions, graph) = parse_input(lines);
    let instructions = instructions.as_bytes();
    let mut pointer = 0;
    let mut steps = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let (left, right) = graph.get(current_node).unwrap();
        let instruction = instructions[pointer] as char;
        current_node = match instruction {
            'L' => left,
            'R' => right,
            _ => panic!("Invalid instruction"),
        };
        pointer += 1;
        steps += 1;
        if pointer >= instructions.len() {
            pointer = 0;
        }
    }
    println!("Part 1: {}", steps);
}

fn parse_input<'a>(lines: &Vec<&'a str>) -> (&'a str, HashMap<&'a str, (&'a str, &'a str)>) {
    let instructions = lines[0];
    let mut graph = HashMap::new();
    for line in lines.iter().skip(2) {
        let node_name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        graph.insert(node_name, (left, right));
    }
    (instructions, graph)
}

extern crate test;

use std::collections::HashMap;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day08/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}
