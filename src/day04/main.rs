#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let card = Card::new(line);
        let matches = card.matches();
        if matches == 0 {
            continue;
        }

        sum += 2u32.pow(matches - 1);
    }
    sum
}

fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
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
    sum
}

struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let (_, game_part) = line.split_at(line.find(':').unwrap());
        let (left_part, right_part) = game_part.split_at(game_part.find('|').unwrap());
        let winning_numbers = left_part[1..].split(' ').filter(|n| *n != "").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let your_numbers = right_part[1..].split(' ').filter(|n| *n != "").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Card {
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
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); } // 126,210 ns/iter (+/- 16,117)
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); } // 140,675 ns/iter (+/- 8,612)
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 13); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 30); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 18619); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 8063216); }
