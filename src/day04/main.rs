#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day04/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let card = Card::new(line);
        let matches = card.matches();
        if matches == 0 {
            continue;
        }

        sum += 2u32.pow(matches - 1);
    }
    println!("Part 1: {}", sum);
}

fn part2(lines: &Vec<&str>) {
    let mut dict = std::collections::HashMap::new();
    for i in 0..lines.len() {
        dict.insert(i, 1u32);
    }

    let mut sum = 0;
    for i in 0..lines.len() {
        let card = Card::new(lines[i]);
        let current_card_amount = dict.get(&i).unwrap().clone();
        sum += current_card_amount;
        let matches = card.matches();
        if matches == 0 {
            continue;
        }

        for j in i+1..i+1+(matches as usize) {
            let next_card_amount = dict.get_mut(&j).unwrap();
            *next_card_amount += current_card_amount;
        }
    }

    println!("Part 2: {}", sum);
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let (id_part, game_part) = line.split_at(line.find(':').unwrap());
        let id = id_part[5..].trim_start().parse::<u32>().unwrap();
        let (left_part, right_part) = game_part.split_at(game_part.find('|').unwrap());
        let winning_numbers = left_part[1..].split(' ').filter(|n| *n != "").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let your_numbers = right_part[1..].split(' ').filter(|n| *n != "").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Card {
            id,
            winning_numbers,
            your_numbers,
        }
    }

    fn matches(&self) -> u32 {
        let mut matches = 0;
        for number in &self.your_numbers {
            if self.winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        matches
    }
}

extern crate test;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day04/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}

#[bench]
fn test_part2(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day04/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part2(&lines));
}
