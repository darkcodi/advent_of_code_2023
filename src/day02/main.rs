#![feature(test)]

const RED_CUBES: usize = 12;
const GREEN_CUBES: usize = 13;
const BLUE_CUBES: usize = 14;

fn main() {
    let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
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
    println!("Part 1: {}", sum);
}

fn part2(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
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
    println!("Part 2: {}", sum);
}

extern crate test;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}

#[bench]
fn test_part2(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part2(&lines));
}
