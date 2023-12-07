#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day07/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    //part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut hands_with_bids = Vec::new();
    for line in lines {
        let (hand_str, bid_str) = line.split_at(5);
        let hand = Hand::new(hand_str);
        let bid = bid_str[1..].parse::<u32>().unwrap();
        let hand_with_bid = HandWithBid { hand, bid };
        hands_with_bids.push(hand_with_bid);
    }
    hands_with_bids.sort();
    let mut sum = 0;
    let mut rank = 1;
    for hand_with_bid in hands_with_bids {
        sum += hand_with_bid.bid * rank;
        rank += 1;
    }
    println!("Part 1: {}", sum);
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct HandWithBid {
    hand: Hand,
    bid: u32,
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
}

impl Hand {
    fn new(hand_str: &str) -> Hand {
        let mut cards = Vec::new();
        for c in hand_str.chars() {
            let card = match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Invalid card"),
            };
            cards.push(card);
        }
        let hand_type = Hand::get_type(&cards);
        Hand { cards, hand_type }
    }

    fn get_type(cards: &Vec<u32>) -> HandType {
        let mut counts = [0; 13];
        for card in cards {
            counts[(*card - 2) as usize] += 1;
        }

        let max_count = *counts.iter().max().unwrap();

        match max_count {
            1 => HandType::HighCard,
            2 => {
                if counts.iter().filter(|c| **c == 2).count() == 1 {
                    HandType::OnePair
                } else {
                    HandType::TwoPairs
                }
            },
            3 => {
                if counts.iter().filter(|c| **c == 2).count() == 1 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            },
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        }
        for (a, b) in self.cards.iter().zip(&other.cards) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                non_eq => return Some(non_eq),
            }
        }
        self.cards.len().partial_cmp(&other.cards.len())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

// fn part2(lines: &Vec<&str>) {
//     todo!()
// }

extern crate test;

use std::cmp::Ordering;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day07/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}

// #[bench]
// fn test_part2(b: &mut Bencher) {
//     let input = std::fs::read_to_string("src/day07/input.txt").unwrap();
//     let lines: Vec<&str> = input.lines().collect();
//     b.iter(|| part2(&lines));
// }
