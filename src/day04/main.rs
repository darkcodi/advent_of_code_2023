#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day04/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let card = Card::new(line);
        sum += card.points();
    }
    println!("Part 1: {}", sum);
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

    fn points(&self) -> u32 {
        let mut matches = 0;
        for number in &self.your_numbers {
            if self.winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        if matches == 0 {
            return 0;
        }

        2u32.pow(matches - 1)
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
