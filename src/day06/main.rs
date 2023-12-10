#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
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
fn part1(lines: &Vec<&str>) {
    let input = parse_input(lines);
    let answer = calculate_answer(input);
    println!("Part 1: {}", answer);
}

fn part2(lines: &Vec<&str>) {
    let input = parse_input_kerning(lines);
    let answer = calculate_answer(input);
    println!("Part 2: {}", answer);
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

fn parse_input(lines: &Vec<&str>) -> Vec<(i64, i64)> {
    let time_str = lines[0];
    let distances_str = lines[1];
    let times: Vec<i64> = time_str.split_at(5).1.split(" ").filter(|s| *s != "").map(|s| s.parse::<i64>().unwrap()).collect();
    let distances: Vec<i64> = distances_str.split_at(9).1.split(" ").filter(|s| *s != "").map(|s| s.parse::<i64>().unwrap()).collect();
    times.iter().zip(distances.iter()).map(|(t, d)| (*t, *d)).collect()
}

fn parse_input_kerning(lines: &Vec<&str>) -> Vec<(i64, i64)> {
    let time_str = lines[0].split_at(5).1.replace(" ", "");
    let distances_str = lines[1].split_at(9).1.replace(" ", "");
    let val = (time_str.parse::<i64>().unwrap(), distances_str.parse::<i64>().unwrap());
    vec![val]
}

extern crate test;

use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}

#[bench]
fn test_part2(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part2(&lines));
}
