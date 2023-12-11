#![feature(test)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input-test.txt");

fn main() {
    println!("Part 1 (test): {}", part1(INPUT_TEST));
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2 (test): {}", part2(INPUT_TEST));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    let almanac = parse_almanac(input);
    let locations = almanac.seeds.iter().map(|seed| almanac.map(*seed as i64)).collect::<Vec<i64>>();
    let min = *locations.iter().min().unwrap();
    min
}

fn part2(input: &str) -> i64 {
    let mut almanac = parse_almanac(input);

    // fix seeds
    let mut counts = Vec::with_capacity(almanac.seeds.len() / 2);
    let mut starts = Vec::with_capacity(almanac.seeds.len() / 2);
    while almanac.seeds.len() > 0 {
        let count = almanac.seeds.pop();
        let start = almanac.seeds.pop();
        counts.push(count.unwrap());
        starts.push(start.unwrap());
    }
    counts.reverse();
    starts.reverse();
    let seed_segments = counts.iter().zip(starts.iter())
        .map(|(count, start)| Segment { start: *start, end: *start + *count - 1 })
        .collect::<Vec<Segment>>();

    let mut locations = Vec::new();
    for seed_segment in seed_segments {
        let intersections = almanac.maps.iter()
            .flat_map(|map| map.mappings.iter().map(|mapping| mapping.src.get_intersection(&seed_segment)))
            .filter(|x| x.is_some()).map(|x| x.unwrap())
            .collect::<Vec<Segment>>();
        if intersections.len() == 0 {
            continue;
        }
        let min_location = intersections.iter().map(|segment| almanac.map(segment.start)).min().unwrap();
        locations.push(min_location);
    }

    let min = *locations.iter().min().unwrap();
    min
}

fn parse_almanac(input: &str) -> Almanac {
    let lines = input.lines().collect::<Vec<&str>>();
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
            num_parts[1].parse::<i64>().unwrap(),
            num_parts[0].parse::<i64>().unwrap(),
            num_parts[2].parse::<i64>().unwrap(),
        );
    }
    if current_map.is_some() {
        maps.push(current_map.unwrap());
    }
    let mut almanac = Almanac { seeds, maps };
    almanac.compose_maps();
    almanac
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
    mappings: Vec<SegmentMapping>,
}

#[derive(Debug, Clone)]
struct SegmentMapping {
    src: Segment,
    dst: Segment,
    offset: i64,
}

#[derive(Debug, Clone)]
struct Segment {
    start: i64,
    end: i64,
}

impl Almanac {
    fn map(&self, num: i64) -> i64 {
        let mut num = num;
        for map in &self.maps {
            num = map.map(num);
        }
        num
    }

    fn compose_maps(&mut self) {
        for map in &mut self.maps {
            map.sort_mappings();
        }
        self.maps.reverse();
        let mut composite_map = self.maps.pop().unwrap();
        while self.maps.len() > 0 {
            composite_map = composite_map.compose(&self.maps.pop().unwrap());
        }
        self.maps.push(composite_map);
    }
}

impl AlmanacMap {
    fn new(src_name: &str, dst_name: &str) -> AlmanacMap {
        AlmanacMap {
            src_name: src_name.to_string(),
            dst_name: dst_name.to_string(),
            mappings: Vec::new(),
        }
    }

    fn add_range(&mut self, src_start: i64, dst_start: i64, length: i64) {
        self.mappings.push(SegmentMapping::from_start_points(src_start, dst_start, length));
    }

    fn map(&self, num: i64) -> i64 {
        for mapping in &self.mappings {
            if mapping.contains(num) {
                return mapping.map(num);
            }
        }
        num
    }

    fn sort_mappings(&mut self) {
        self.mappings.sort_by(|a, b| a.src.start.cmp(&b.src.start));
    }

    fn max(&self) -> i64 { self.max_src().max(self.max_dst()) }
    fn max_src(&self) -> i64 { self.mappings.iter().map(|mapping| mapping.src.end).max().unwrap() }
    fn max_dst(&self) -> i64 { self.mappings.iter().map(|mapping| mapping.dst.end).max().unwrap() }

    fn compose(&self, other: &AlmanacMap) -> AlmanacMap {
        assert_eq!(self.dst_name, other.src_name);

        // fill gaps
        let mut start = 0;
        let end = self.max().max(other.max());
        let mut self_mappings = Vec::new();
        for mapping in &self.mappings {
            if mapping.src.start > start {
                let src_filling = Segment { start, end: mapping.src.start - 1 };
                self_mappings.push(SegmentMapping::from_segment(src_filling, 0));
                start = mapping.src.end + 1;
            }
            self_mappings.push(mapping.clone());
        }
        if start <= end {
            let src_filling = Segment { start, end };
            self_mappings.push(SegmentMapping::from_segment(src_filling, 0));
        }

        // compose mappings
        let mut composed_mappings = Vec::new();
        for mapping in &self_mappings {
            composed_mappings.extend(mapping.split(&other.mappings));
        }

        AlmanacMap {
            src_name: self.src_name.clone(),
            dst_name: other.dst_name.clone(),
            mappings: composed_mappings,
        }
    }
}

impl SegmentMapping {
    fn from_start_points(src_start: i64, dst_start: i64, length: i64) -> SegmentMapping {
        SegmentMapping {
            src: Segment { start: src_start, end: src_start + length - 1 },
            dst: Segment { start: dst_start, end: dst_start + length - 1 },
            offset: dst_start - src_start,
        }
    }

    fn from_segment(src: Segment, offset: i64) -> SegmentMapping {
        let dst = src.add_offset(offset);
        SegmentMapping {
            src,
            dst,
            offset,
        }
    }

    fn contains(&self, num: i64) -> bool {
        self.src.contains(num)
    }

    fn map(&self, num: i64) -> i64 {
        num + self.offset
    }

    fn split(&self, other_mappings: &Vec<SegmentMapping>) -> Vec<SegmentMapping> {
        let mut new_mappings = Vec::new();
        let mut dst_point = self.dst.start;
        for other in other_mappings {
            let dst_intersection = self.dst.get_intersection(&other.src);
            if dst_intersection.is_none() {
                continue;
            }

            let dst_intersection = dst_intersection.unwrap();
            if dst_intersection.start > dst_point {
                let dst_filling = Segment { start: dst_point, end: dst_intersection.start - 1 };
                let src_filling = dst_filling.add_offset(-self.offset);
                new_mappings.push(SegmentMapping::from_segment(src_filling, self.offset));
            }

            let src_intersection = dst_intersection.add_offset(-self.offset);
            new_mappings.push(SegmentMapping::from_segment(src_intersection, self.offset + other.offset));

            dst_point = dst_intersection.end + 1;
            if dst_point > self.dst.end {
                break;
            }
        }
        if dst_point <= self.dst.end {
            let dst_filling = Segment { start: dst_point, end: self.dst.end };
            let src_filling = dst_filling.add_offset(-self.offset);
            new_mappings.push(SegmentMapping::from_segment(src_filling, self.offset));
        }
        assert!(new_mappings.len() > 0, "Split has no mappings");
        assert_eq!(new_mappings[0].src.start, self.src.start);
        assert_eq!(new_mappings[new_mappings.len() - 1].src.end, self.src.end);
        new_mappings
    }
}

impl Segment {
    fn contains(&self, num: i64) -> bool {
        num >= self.start && num <= self.end
    }

    fn get_intersection(&self, other: &Segment) -> Option<Segment> {
        if self.start > other.end || self.end < other.start {
            return None;
        }
        let start = if self.start > other.start { self.start } else { other.start };
        let end = if self.end < other.end { self.end } else { other.end };
        Some(Segment { start, end })
    }

    fn add_offset(&self, offset: i64) -> Segment {
        Segment { start: self.start + offset, end: self.end + offset }
    }
}

extern crate test;
#[bench] fn part1_perf(b: &mut test::Bencher) { b.iter(|| part1(INPUT)); }
#[bench] fn part2_perf(b: &mut test::Bencher) { b.iter(|| part2(INPUT)); }
#[test] fn part1_test_answer() { assert_eq!(part1(INPUT_TEST), 35); }
#[test] fn part2_test_answer() { assert_eq!(part2(INPUT_TEST), 46); }
#[test] fn part1_answer() { assert_eq!(part1(INPUT), 825516882); }
#[test] fn part2_answer() { assert_eq!(part2(INPUT), 136096660); }
