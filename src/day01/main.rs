#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day01/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mut first_number = None;
        let mut last_number = None;
        for c in line.chars() {
            let number = c.to_digit(10);
            if number.is_some() {
                if first_number == None {
                    first_number = number;
                }
                last_number = number;
            }
        }
        if first_number.is_some() {
            sum += first_number.unwrap() * 10 + last_number.unwrap();
        }
    }
    println!("Part 1: {}", sum);
}

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
fn part2(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mut first_number = None;
        let mut last_number = None;
        for c in line.chars() {
            let number = c.to_digit(10).or_else(|| {
                for (i, number) in NUMBERS.iter().enumerate() {
                    if number.starts_with(c) {
                        return Some((i + 1) as u32);
                    }
                }
                None
            });
            if number.is_some() {
                if first_number == None {
                    first_number = number;
                }
                last_number = number;
            }
        }
        if first_number.is_some() {
            sum += first_number.unwrap() * 10 + last_number.unwrap();
        }
    }
    println!("Part 2: {}", sum);
}

extern crate test;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day01/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}

#[bench]
fn test_part2(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day01/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part2(&lines));
}
