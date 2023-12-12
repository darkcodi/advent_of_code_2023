#![feature(test)]

use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    // println!("Part 1 (test): {}", part1(INPUT_TEST));
    // println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

fn part2(input: &str) -> u32 {
    let mut hands_with_bids = Vec::new();
    for line in input.lines() {
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
    sum
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
                'J' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => panic!("Invalid card"),
            };
            cards.push(card);
        }
        let hand_type = Hand::get_type(&cards);
        Hand { cards, hand_type }
    }

    fn get_type(cards: &Vec<u32>) -> HandType {
        let mut counts = [0; 12];
        let mut joker_count = 0;
        for card in cards {
            if *card == 1 {
                joker_count += 1;
                continue;
            }
            counts[(*card - 2) as usize] += 1;
        }

        let max_count = *counts.iter().max().unwrap();

        if joker_count == 1 {
            if max_count == 4 {
                return HandType::FiveOfAKind;
            }
            if max_count == 3 {
                return HandType::FourOfAKind;
            }
            if max_count == 2 {
                if counts.iter().filter(|c| **c == 2).count() == 1 {
                    return HandType::ThreeOfAKind;
                }
                return HandType::FullHouse;
            }
            return HandType::OnePair; // max_count == 1
        }

        if joker_count == 2 {
            if max_count == 3 {
                return HandType::FiveOfAKind;
            }
            if max_count == 2 {
                return HandType::FourOfAKind;
            }
            return HandType::ThreeOfAKind; // max_count == 1
        }

        if joker_count == 3 {
            if max_count == 2 {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind; // max_count == 1
        }

        if joker_count == 4 || joker_count == 5 {
            return HandType::FiveOfAKind;
        }

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

extern crate test;
//#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); }
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); } // 198,735 ns/iter (+/- 9,940)
//#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 6440); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 5905); }
//#[test] fn part1_answer() { assert_eq!(part1(INPUT), 248113761); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 246285222); }
