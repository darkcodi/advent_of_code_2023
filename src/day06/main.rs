#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

// x (charge time, variable)
// t (total time, given)
// v (speed, variable) = x
// y (distance, given) = (t - x) * v = (t - x) * x = tx - x^2
// SO...
// y = tx - x^2
// -x^2 + tx - y = 0 (A = -1, B = t, C = -y)
// D = B^2 - 4AC = t^2 - 4(-1)(-y) = t^2 - 4y
// x = (-B +- sqrt(D)) / 2A = (-t +- sqrt(t^2 - 4y)) / -2
// x = (t +- sqrt(t^2 - 4y)) / 2
fn part1(input: &str) -> i64 {
    let input = parse_input(input);
    let answer = calculate_answer(input);
    answer
}

fn part2(input: &str) -> i64 {
    let input = parse_input_kerning(input);
    let answer = calculate_answer(input);
    answer
}

fn calculate_answer(input: Vec<(i64, i64)>) -> i64 {
    let charge_times: Vec<(i64, i64)> = input.iter().map(|(t, d)| {
        let t = *t as f64;
        let d = *d as f64;
        (
            ((t - (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil() as i64, // min charge time
            ((t + (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).floor() as i64, // max charge time
        )
    }).collect();
    let win_num: Vec<i64> = charge_times.iter().map(|(min, max)| max - min + 1).collect();
    let answer = win_num.iter().fold(1, |acc, x| acc * x);
    answer
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let lines: Vec<&str> = input.lines().collect();
    let time_str = lines[0];
    let distances_str = lines[1];
    let times: Vec<i64> = time_str.split_at(5).1.split(" ").filter(|s| *s != "").map(|s| s.parse::<i64>().unwrap()).collect();
    let distances: Vec<i64> = distances_str.split_at(9).1.split(" ").filter(|s| *s != "").map(|s| s.parse::<i64>().unwrap()).collect();
    times.iter().zip(distances.iter()).map(|(t, d)| (*t, *d)).collect()
}

fn parse_input_kerning(input: &str) -> Vec<(i64, i64)> {
    let lines: Vec<&str> = input.lines().collect();
    let time_str = lines[0].split_at(5).1.replace(" ", "");
    let distances_str = lines[1].split_at(9).1.replace(" ", "");
    let val = (time_str.parse::<i64>().unwrap(), distances_str.parse::<i64>().unwrap());
    vec![val]
}

extern crate test;
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); }
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); }
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 288); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 71503); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 449550); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 28360140); }
