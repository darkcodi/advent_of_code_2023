#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day03/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
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

            let left_top = if i > 0 && j > 0 { (prev_line.unwrap()[j - 1] as char).is_digit(10) } else { false };
            let top = if i > 0 { (prev_line.unwrap()[j] as char).is_digit(10) } else { false };
            let right_top = if i > 0 && j < current_line.len() - 1 { (prev_line.unwrap()[j + 1] as char).is_digit(10) } else { false };
            let left = if j > 0 { (current_line[j - 1] as char).is_digit(10) } else { false };
            let right = if j < current_line.len() - 1 { (current_line[j + 1] as char).is_digit(10) } else { false };
            let left_bottom = if i < lines.len() - 1 && j > 0 { (next_line.unwrap()[j - 1] as char).is_digit(10) } else { false };
            let bottom = if i < lines.len() - 1 { (next_line.unwrap()[j] as char).is_digit(10) } else { false };
            let right_bottom = if i < lines.len() - 1 && j < current_line.len() - 1 { (next_line.unwrap()[j + 1] as char).is_digit(10) } else { false };

            // top
            if left_top && top && right_top {
                sum += read_number_right(prev_line.unwrap(), j-1);
            } else if left_top && top {
                sum += read_number_left(prev_line.unwrap(), j);
            } else if top && right_top {
                sum += read_number_right(prev_line.unwrap(), j);
            } else if left_top && right_top {
                sum += read_number_left(prev_line.unwrap(), j-1);
                sum += read_number_right(prev_line.unwrap(), j+1);
            } else if left_top {
                sum += read_number_left(prev_line.unwrap(), j-1);
            } else if right_top {
                sum += read_number_right(prev_line.unwrap(), j+1);
            } else if top {
                sum += read_number_right(prev_line.unwrap(), j);
            }

            // left
            if left {
                sum += read_number_left(current_line, j-1);
            }

            // right
            if right {
                sum += read_number_right(current_line, j+1);
            }

            // bottom
            if left_bottom && bottom && right_bottom {
                sum += read_number_right(next_line.unwrap(), j-1);
            } else if left_bottom && bottom {
                sum += read_number_left(next_line.unwrap(), j);
            } else if bottom && right_bottom {
                sum += read_number_right(next_line.unwrap(), j);
            } else if left_bottom && right_bottom {
                sum += read_number_left(next_line.unwrap(), j-1);
                sum += read_number_right(next_line.unwrap(), j+1);
            } else if left_bottom {
                sum += read_number_left(next_line.unwrap(), j-1);
            } else if right_bottom {
                sum += read_number_right(next_line.unwrap(), j+1);
            } else if bottom {
                sum += read_number_right(next_line.unwrap(), j);
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn read_number_left(line: &[u8], i: usize) -> u32 {
    let mut number = 0;
    let mut j = i;
    let mut multiplier = 1;
    while j >= 0 {
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

fn read_number_right(line: &[u8], i: usize) -> u32 {
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
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day03/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}
