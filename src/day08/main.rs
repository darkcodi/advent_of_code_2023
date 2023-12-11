#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day08/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
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

fn part2(lines: &Vec<&str>) {
    let (instructions, graph) = parse_input(lines);
    let instructions = instructions.as_bytes();
    let mut current_nodes : Vec<&str> = graph.keys().filter(|x| x.ends_with('A')).map(|x| *x).collect();
    current_nodes.sort();
    let z_list : Vec<usize> = current_nodes.iter().map(|x| find_z_position(x, instructions, &graph)).collect();
    let mut z_list = z_list.iter();
    let mut lcm = *z_list.next().unwrap();
    for z in z_list {
        lcm = find_lcm(lcm, *z);
    }
    println!("Part 2: {}", lcm);
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

fn find_z_position(node: &str, instructions: &[u8], graph: &HashMap<&str, (&str, &str)>) -> usize {
    let mut current_node = node;
    let mut pointer = 0;
    let mut steps = 0;
    let mut z_position = None;
    while z_position.is_none() {
        let (left, right) = graph.get(current_node).unwrap();
        let instruction = instructions[pointer] as char;
        current_node = match instruction {
            'L' => left,
            'R' => right,
            _ => panic!("Invalid instruction"),
        };
        steps += 1;
        pointer += 1;
        if pointer >= instructions.len() {
            pointer = 0;
        }
        if current_node.ends_with('Z') {
            z_position = Some(steps);
        }
    }
    z_position.unwrap()
}

// https://en.wikipedia.org/wiki/Least_common_multiple
fn find_lcm(first: usize, second: usize) -> usize {
    first * second / find_gcd(first, second)
}

// https://en.wikipedia.org/wiki/Greatest_common_divisor
fn find_gcd(first: usize, second: usize) -> usize {
    let mut max = first.max(second);
    let mut min = first.min(second);
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
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

#[bench]
fn test_part2(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day08/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part2(&lines));
}
