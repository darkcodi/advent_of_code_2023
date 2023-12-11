#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

const RED_CUBES: usize = 12;
const GREEN_CUBES: usize = 13;
const BLUE_CUBES: usize = 14;

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (id_part, game_part) = line.split_at(line.find(':').unwrap());
        let mut is_possible = true;
        for turn_part in game_part[1..].split(';') {
            for cube_part in turn_part.split(',') {
                let cube_part = cube_part.trim_start();
                let (cube_count, color) = cube_part.split_at(cube_part.find(' ').unwrap());
                let cube_count = cube_count.parse::<usize>().unwrap();
                let enough_cubes = match color.trim_start() {
                    "red" => cube_count <= RED_CUBES,
                    "green" => cube_count <= GREEN_CUBES,
                    "blue" => cube_count <= BLUE_CUBES,
                    _ => false,
                };
                if !enough_cubes {
                    is_possible = false;
                    break;
                }
            }
            if !is_possible {
                break;
            }
        }
        if is_possible {
            let id = id_part[5..].parse::<usize>().unwrap();
            sum += id;
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        let (_, game_part) = line.split_at(line.find(':').unwrap());
        for turn_part in game_part[1..].split(';') {
            for cube_part in turn_part.split(',') {
                let cube_part = cube_part.trim_start();
                let (cube_count, color) = cube_part.split_at(cube_part.find(' ').unwrap());
                let cube_count = cube_count.parse::<usize>().unwrap();
                match color.trim_start() {
                    "red" => if cube_count > max_red { max_red = cube_count; },
                    "green" => if cube_count > max_green { max_green = cube_count; },
                    "blue" => if cube_count > max_blue { max_blue = cube_count; },
                    _ => {}
                };
            }
        }
        let power = max_red * max_green * max_blue;
        sum += power;
    }
    sum
}

extern crate test;
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); }
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); }
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 8); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 2286); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 2207); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 62241); }