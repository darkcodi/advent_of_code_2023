#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day09/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let numbers: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let next_number = predict_next_number(&numbers);
        sum += next_number;
    }
    println!("Part 1: {}", sum);
}

fn predict_next_number(numbers: &Vec<i64>) -> i64 {
    let diff = get_diff(numbers);
    if diff.iter().all(|x| *x == 0) {
        return numbers[0]
    }
    let next_diff = predict_next_number(&diff);
    numbers[numbers.len() - 1] + next_diff
}

fn get_diff(numbers: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::with_capacity(numbers.len() - 1);
    for i in 0..numbers.len() - 1 {
        diff.push(numbers[i + 1] - numbers[i]);
    }
    diff
}

extern crate test;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day09/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}
