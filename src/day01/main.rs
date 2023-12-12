#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST1: &str = include_str!("input-test1.txt");
const INPUT_TEST2: &str = include_str!("input-test2.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST1));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST2));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_number = None;
        let mut last_number = None;
        for c in line.chars() {
            let number = c.to_digit(10);
            if number.is_some() {
                if first_number == None {
                    first_number = number;
                }
                last_number = number;
            }
        }
        if first_number.is_some() {
            sum += first_number.unwrap() * 10 + last_number.unwrap();
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut buffer = CycleBuffer::new();
    let mut sum = 0;
    for line in input.lines() {
        let mut first_number = None;
        let mut last_number = None;
        for c in line.chars() {
            buffer.push(c);
            let number = c.to_digit(10).or_else(|| buffer.parse());
            if number.is_some() {
                if first_number == None {
                    first_number = number;
                }
                last_number = number;
            }
        }
        if first_number.is_some() {
            sum += first_number.unwrap() * 10 + last_number.unwrap();
        }
    }
    sum
}

struct CycleBuffer {
    buffer: [char; 5],
    index: usize,
}

impl CycleBuffer {
    fn new() -> Self {
        Self {
            buffer: ['a'; 5],
            index: 0,
        }
    }

    fn push(&mut self, c: char) {
        self.buffer[self.index as usize] = c;
        self.index = CycleBuffer::get_next_index(self.index);
    }

    fn parse(&self) -> Option<u32> {
        let mut i = CycleBuffer::get_prev_index(self.index);
        let c1 = self.buffer[i];
        i = CycleBuffer::get_prev_index(i);
        let c2 = self.buffer[i];
        i = CycleBuffer::get_prev_index(i);
        let c3 = self.buffer[i];
        if c3 == 'o' && c2 == 'n' && c1 == 'e' { return Some(1); }
        if c3 == 't' && c2 == 'w' && c1 == 'o' { return Some(2); }
        if c3 == 's' && c2 == 'i' && c1 == 'x' { return Some(6); }
        i = CycleBuffer::get_prev_index(i);
        let c4 = self.buffer[i];
        if c4 == 'f' && c3 == 'o' && c2 == 'u' && c1 == 'r' { return Some(4); }
        if c4 == 'f' && c3 == 'i' && c2 == 'v' && c1 == 'e' { return Some(5); }
        if c4 == 'n' && c3 == 'i' && c2 == 'n' && c1 == 'e' { return Some(9); }
        i = CycleBuffer::get_prev_index(i);
        let c5 = self.buffer[i];
        if c5 == 't' && c4 == 'h' && c3 == 'r' && c2 == 'e' && c1 == 'e' { return Some(3); }
        if c5 == 's' && c4 == 'e' && c3 == 'v' && c2 == 'e' && c1 == 'n' { return Some(7); }
        if c5 == 'e' && c4 == 'i' && c3 == 'g' && c2 == 'h' && c1 == 't' { return Some(8); }
        None
    }

    fn get_next_index(i : usize) -> usize {
        (i + 1) % 5
    }

    fn get_prev_index(i : usize) -> usize {
        (i + 4) % 5
    }
}

extern crate test;
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); } // 28,467 ns/iter (+/- 5,411)
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); } // 138,505 ns/iter (+/- 6,980)
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST1), 142); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST2), 281); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 55029); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 55686); }
