#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    // part2(&lines);
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
    let charge_times: Vec<(i32, i32)> = input.iter().map(|(t, d)| {
        let t = *t as f32;
        let d = *d as f32;
        (
            ((t - (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil() as i32, // min charge time
            ((t + (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).floor() as i32, // max charge time
        )
    }).collect();
    let win_num : Vec<i32> = charge_times.iter().map(|(min, max)| max - min + 1).collect();
    let answer = win_num.iter().fold(1, |acc, x| acc * x);
    println!("Part 1: {}", answer);
}

fn parse_input(lines: &Vec<&str>) -> Vec<(i32, i32)> {
    let time_str = lines[0];
    let distances_str = lines[1];
    let times: Vec<i32> = time_str.split_at(5).1.split(" ").filter(|s| *s != "").map(|s| s.parse::<i32>().unwrap()).collect();
    let distances: Vec<i32> = distances_str.split_at(9).1.split(" ").filter(|s| *s != "").map(|s| s.parse::<i32>().unwrap()).collect();
    times.iter().zip(distances.iter()).map(|(t, d)| (*t, *d)).collect()
}

extern crate test;

use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}