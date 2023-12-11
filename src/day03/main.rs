#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        let current_line = lines[i].as_bytes();
        let prev_line = if i > 0 { Some(lines[i - 1].as_bytes()) } else { None };
        let next_line = if i < lines.len() - 1 { Some(lines[i + 1].as_bytes()) } else { None };

        for j in 0..current_line.len() {
            let c = current_line[j] as char;
            if c.is_digit(10) || c == '.' {
                continue;
            }

            let numbers = find_numbers(current_line, prev_line, next_line, i, j);
            sum += numbers.iter().sum::<u32>();
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        let current_line = lines[i].as_bytes();
        let prev_line = if i > 0 { Some(lines[i - 1].as_bytes()) } else { None };
        let next_line = if i < lines.len() - 1 { Some(lines[i + 1].as_bytes()) } else { None };

        for j in 0..current_line.len() {
            let c = current_line[j] as char;
            if c == '*' {
                let numbers = find_numbers(current_line, prev_line, next_line, i, j);
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }
    sum
}

fn find_numbers(current_line: &[u8], prev_line: Option<&[u8]>, next_line: Option<&[u8]>, i: usize, j: usize) -> Vec<u32> {
    let left_top = if i > 0 && j > 0 { (prev_line.unwrap()[j - 1] as char).is_digit(10) } else { false };
    let top = if i > 0 { (prev_line.unwrap()[j] as char).is_digit(10) } else { false };
    let right_top = if i > 0 && j < current_line.len() - 1 { (prev_line.unwrap()[j + 1] as char).is_digit(10) } else { false };
    let left = if j > 0 { (current_line[j - 1] as char).is_digit(10) } else { false };
    let right = if j < current_line.len() - 1 { (current_line[j + 1] as char).is_digit(10) } else { false };
    let left_bottom = if next_line.is_some() && j > 0 { (next_line.unwrap()[j - 1] as char).is_digit(10) } else { false };
    let bottom = if next_line.is_some() { (next_line.unwrap()[j] as char).is_digit(10) } else { false };
    let right_bottom = if next_line.is_some() && j < current_line.len() - 1 { (next_line.unwrap()[j + 1] as char).is_digit(10) } else { false };

    let mut numbers = Vec::new();

    // top
    if left_top && top && right_top { numbers.push(s_right(prev_line.unwrap(), j-1)); }
    else if left_top && top { numbers.push( s_left(prev_line.unwrap(), j)); }
    else if top && right_top { numbers.push(s_right(prev_line.unwrap(), j)); }
    else if left_top && right_top { numbers.push(s_left(prev_line.unwrap(), j-1)); numbers.push(s_right(prev_line.unwrap(), j+1)); }
    else if left_top { numbers.push(s_left(prev_line.unwrap(), j-1)); }
    else if right_top { numbers.push(s_right(prev_line.unwrap(), j+1)); }
    else if top { numbers.push(s_right(prev_line.unwrap(), j)); }

    // left
    if left { numbers.push(s_left(current_line, j-1)); }

    // right
    if right { numbers.push(s_right(current_line, j+1)); }

    // bottom
    if left_bottom && bottom && right_bottom { numbers.push(s_right(next_line.unwrap(), j-1)); }
    else if left_bottom && bottom { numbers.push(s_left(next_line.unwrap(), j)); }
    else if bottom && right_bottom { numbers.push(s_right(next_line.unwrap(), j)); }
    else if left_bottom && right_bottom { numbers.push(s_left(next_line.unwrap(), j-1)); numbers.push(s_right(next_line.unwrap(), j+1)); }
    else if left_bottom { numbers.push(s_left(next_line.unwrap(), j-1)); }
    else if right_bottom { numbers.push(s_right(next_line.unwrap(), j+1)); }
    else if bottom { numbers.push(s_right(next_line.unwrap(), j)); }

    numbers
}

fn s_left(line: &[u8], i: usize) -> u32 {
    let mut number = 0;
    let mut j = i;
    let mut multiplier = 1;
    loop {
        let c = line[j] as char;
        if c.is_digit(10) {
            number += c.to_digit(10).unwrap() * multiplier;
            multiplier *= 10;
        } else {
            break;
        }
        if j == 0 {
            break;
        }
        j -= 1;
    }
    number
}

fn s_right(line: &[u8], i: usize) -> u32 {
    let mut number = 0;
    let mut j = i;
    while j < line.len() {
        let c = line[j] as char;
        if c.is_digit(10) {
            number *= 10;
            number += c.to_digit(10).unwrap();
        } else {
            break;
        }
        j += 1;
    }
    number
}

extern crate test;
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); }
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); }
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 4361); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 467835); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 556057); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 82824352); }
