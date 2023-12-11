#![feature(test)]

fn main() {
    let input = std::fs::read_to_string("src/day10/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let graph = build_graph(lines);
    let distances = dijkstra(&graph);
    let max_distance = distances.values().max().unwrap();
    println!("Part 1: {}", max_distance);
}

fn dijkstra(graph: &Graph) -> HashMap<Vec2, usize> {
    let mut distances = HashMap::new();
    let mut queue = Vec::new();
    queue.push((graph.root_position, 0));
    while !queue.is_empty() {
        let (position, distance) = queue.remove(0);
        if distances.contains_key(&position) {
            continue;
        }
        distances.insert(position, distance);
        let node = graph.nodes_map.get(&position).unwrap();
        for (_, neighbour_position) in node.neighbours.iter() {
            if !distances.contains_key(neighbour_position) {
                queue.push((*neighbour_position, distance + 1));
            }
        }
    }
    distances
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
enum PipeType {
    #[default]
    Ground, // '.'
    NorthSouth, // '|'
    EastWest, // '-'
    NorthEast, // 'L'
    NorthWest, // 'J'
    SouthWest, // '7'
    SouthEast, // 'F'
}

static PIPE_TYPES: [PipeType; 7] = [
    PipeType::Ground,
    PipeType::NorthSouth,
    PipeType::EastWest,
    PipeType::NorthEast,
    PipeType::NorthWest,
    PipeType::SouthWest,
    PipeType::SouthEast,
];

impl PipeType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => PipeType::Ground,
            '|' => PipeType::NorthSouth,
            '-' => PipeType::EastWest,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            _ => panic!("Unknown pipe type: {}", c),
        }
    }

    fn to_directions(&self) -> (Direction, Direction) {
        match self {
            PipeType::Ground => (Direction::None, Direction::None),
            PipeType::NorthSouth => (Direction::North, Direction::South),
            PipeType::EastWest => (Direction::East, Direction::West),
            PipeType::NorthEast => (Direction::North, Direction::East),
            PipeType::NorthWest => (Direction::North, Direction::West),
            PipeType::SouthWest => (Direction::South, Direction::West),
            PipeType::SouthEast => (Direction::South, Direction::East),
        }
    }

    fn is_compatible(&self, dir: &Direction) -> bool {
        let my_dirs = self.to_directions();
        dir.is_compatible(&my_dirs.0) || dir.is_compatible(&my_dirs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
enum Direction {
    #[default]
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Direction::None => Direction::None,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn is_compatible(&self, other: &Direction) -> bool {
        match (self, other) {
            (Direction::None, Direction::None) => true,
            (Direction::North, Direction::South) => true,
            (Direction::South, Direction::North) => true,
            (Direction::West, Direction::East) => true,
            (Direction::East, Direction::West) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Node {
    position: Vec2,
    pipe_type: PipeType,
    neighbours: Vec<(Direction, Vec2)>,
}

#[derive(Debug)]
struct Graph {
    nodes_map: HashMap<Vec2, Node>,
    root_position: Vec2,
}

fn build_graph(lines: &Vec<&str>) -> Graph {
    let mut nodes_map = HashMap::new();
    let mut root_position = Vec2::default();
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let position = Vec2 { x, y };
            if c == 'S' {
                root_position = position;
            }
            let pipe_type = if c == 'S' { suggest_pipe_type(lines, &position) } else { PipeType::from_char(c) };
            let neighbours = neighbours_map(lines, &position).iter()
                .filter(|(direction, pos)| {
                    let neighbour = lines[pos.x].as_bytes()[pos.y] as char;
                    neighbour == 'S' || PipeType::from_char(neighbour).is_compatible(direction)
                })
                .map(|(d, p)| (*d, *p))
                .collect::<Vec<(Direction, Vec2)>>();
            let node = Node { position, pipe_type, neighbours };
            nodes_map.insert(position, node);
        }
    }
    Graph { nodes_map, root_position }
}

fn suggest_pipe_type(lines: &Vec<&str>, pos: &Vec2) -> PipeType {
    let all_neighbours = neighbours_map(lines, pos);
    let possible_types = all_neighbours.iter()
        .filter(|(direction, pos)| {
            let neighbour = lines[pos.x].as_bytes()[pos.y] as char;
            PipeType::from_char(neighbour).is_compatible(direction)
        })
        .flat_map(|(d, _)| PIPE_TYPES.iter().filter(|t2| t2.is_compatible(&d.invert())).map(|t2| *t2).collect::<Vec<PipeType>>())
        .collect::<Vec<PipeType>>();
    let most_common_type = possible_types.iter()
        .max_by_key(|t| possible_types.iter().filter(|t2| t2 == t).count())
        .unwrap();
    *most_common_type
}

fn neighbours_map(lines: &Vec<&str>, pos: &Vec2) -> Vec<(Direction, Vec2)> {
    let mut neighbours_map = Vec::new();
    if pos.x > 0 {
        neighbours_map.push((Direction::North, Vec2 { x: pos.x - 1, y: pos.y }));
    }
    if pos.x < lines.len() - 1 {
        neighbours_map.push((Direction::South, Vec2 { x: pos.x + 1, y: pos.y }));
    }
    if pos.y > 0 {
        neighbours_map.push((Direction::West, Vec2 { x: pos.x, y: pos.y - 1 }));
    }
    if pos.y < lines[pos.x].len() - 1 {
        neighbours_map.push((Direction::East, Vec2 { x: pos.x, y: pos.y + 1 }));
    }
    neighbours_map
}

extern crate test;

use std::collections::HashMap;
use test::Bencher;

#[bench]
fn test_part1(b: &mut Bencher) {
    let input = std::fs::read_to_string("src/day10/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    b.iter(|| part1(&lines));
}
