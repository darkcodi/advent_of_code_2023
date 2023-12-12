#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let next_number = predict_next_number(&numbers);
        sum += next_number;
    }
    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let previous_number = predict_previous_number(&numbers);
        sum += previous_number;
    }
    sum
}

fn predict_next_number(numbers: &Vec<i64>) -> i64 {
    let diff = get_diff(numbers);
    if diff.iter().all(|x| *x == 0) {
        return numbers[0]
    }
    let next_diff = predict_next_number(&diff);
    numbers[numbers.len() - 1] + next_diff
}

fn predict_previous_number(numbers: &Vec<i64>) -> i64 {
    let diff = get_diff(numbers);
    if diff.iter().all(|x| *x == 0) {
        return numbers[0]
    }
    let prev_diff = predict_previous_number(&diff);
    numbers[0] - prev_diff
}

fn get_diff(numbers: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::with_capacity(numbers.len() - 1);
    for i in 0..numbers.len() - 1 {
        diff.push(numbers[i + 1] - numbers[i]);
    }
    diff
}

extern crate test;
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); } // 139,426 ns/iter (+/- 10,814)
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); } // 143,120 ns/iter (+/- 9,015)
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 114); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 2); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 1972648895); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 919); }
