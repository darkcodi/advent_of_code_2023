#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day05/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let almanac = parse_almanac(lines);
    let locations = almanac.seeds.iter().map(|seed| almanac.map(*seed as i64)).collect::<Vec<i64>>();
    let min = locations.iter().min().unwrap();
    println!("Part 1: {}", min);
}

fn parse_almanac(lines: &Vec<&str>) -> Almanac {
    let (first_line, other_lines) = lines.split_first().unwrap();
    let (_, first_line) = first_line.split_at(7); // skip first 7 chars in first line - "seeds: "
    let seeds: Vec<i64> = first_line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();

    let mut maps = Vec::new();
    let mut current_map = None;
    for line in other_lines {
        if *line == "" {
            if current_map.is_some() {
                maps.push(current_map.unwrap());
                current_map = None;
            }
            continue;
        }

        let first_char = line.chars().next().unwrap();
        if !first_char.is_digit(10) {
            let (map_name, _) = line.split_at(line.find(' ').unwrap());
            let map_name_parts = map_name.split('-').collect::<Vec<&str>>();
            let src_name = map_name_parts[0];
            let dst_name = map_name_parts[2];
            current_map = Some(AlmanacMap::new(src_name, dst_name));
            continue;
        }

        let num_parts = line.split(' ').collect::<Vec<&str>>();
        current_map.as_mut().unwrap().add_range(
            num_parts[0].parse::<i64>().unwrap(),
            num_parts[1].parse::<i64>().unwrap(),
            num_parts[2].parse::<i64>().unwrap(),
        );
    }
    if current_map.is_some() {
        maps.push(current_map.unwrap());
    }
    Almanac { seeds, maps }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<AlmanacMap>,
}

#[derive(Debug)]
struct AlmanacMap {
    src_name: String,
    dst_name: String,
    ranges: Vec<AlmanacMapRange>,
}

#[derive(Debug)]
struct AlmanacMapRange {
    src_start: i64,
    src_end: i64,
    offset: i64,
}

impl Almanac {
    fn map(&self, src: i64) -> i64 {
        let mut dst = src;
        for map in &self.maps {
            dst = map.map(dst);
        }
        dst
    }
}

impl AlmanacMap {
    fn new(src_name: &str, dst_name: &str) -> AlmanacMap {
        AlmanacMap {
            src_name: src_name.to_string(),
            dst_name: dst_name.to_string(),
            ranges: Vec::new(),
        }
    }

    fn add_range(&mut self, dst_start: i64, src_start: i64, length: i64) {
        self.ranges.push(AlmanacMapRange::new(dst_start, src_start, length));
    }

    fn map(&self, src: i64) -> i64 {
        for range in &self.ranges {
            if range.contains(src) {
                return range.map(src);
            }
        }
        src
    }
}

impl AlmanacMapRange {
    fn new(dst_start: i64, src_start: i64, length: i64) -> AlmanacMapRange {
        AlmanacMapRange {
            src_start,
            src_end: src_start + length - 1,
            offset: dst_start - src_start,
        }
    }

    fn contains(&self, src: i64) -> bool {
        src >= self.src_start && src <= self.src_end
    }

    fn map(&self, src: i64) -> i64 {
        src + self.offset
    }
}

extern crate test;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day05/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}
