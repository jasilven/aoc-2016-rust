mod aoc;
use aoc::point::Point;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

struct Map {
    favnum: i32,
    coords: HashMap<Point, bool>,
}

impl Map {
    pub fn new(favnum: i32) -> Map {
        let coords = HashMap::new();
        Map { favnum, coords }
    }

    fn update_coords(&mut self, p: &Point) {
        let n = p.x * p.x + 3 * p.x + 2 * p.x * p.y + p.y + p.y * p.y;
        let bin = format!("{:b}", n + self.favnum);
        let ones = bin.chars().filter(|ch| ch == &'1').count();
        match ones % 2 {
            0 => self.coords.insert(p.clone(), true),
            1 => self.coords.insert(p.clone(), false),
            _ => panic!("update_coords failed"),
        };
    }

    pub fn is_open(&mut self, p: &Point) -> bool {
        if self.coords.contains_key(&p) {
            self.coords[&p]
        } else {
            self.update_coords(p);
            self.is_open(p)
        }
    }

    pub fn neighbours(&mut self, p: &Point) -> HashSet<Point> {
        let result: HashSet<Point> = [
            Point::new(p.x, p.y - 1),
            Point::new(p.x, p.y + 1),
            Point::new(p.x - 1, p.y),
            Point::new(p.x + 1, p.y),
        ]
        .iter()
        .filter(|&p| (p.x >= 0) && (p.y >= 0))
        .filter(|&p| self.is_open(&p))
        .cloned()
        .collect();
        result
    }

    #[allow(dead_code)]
    pub fn write_map<T: Write>(&mut self, writer: &mut T, mx: i32, my: i32) {
        for y in 0..=my {
            for x in 0..=mx {
                match self.is_open(&Point::new(x, y)) {
                    true => write!(writer, ".").unwrap(),
                    false => write!(writer, "#").unwrap(),
                };
            }
            writeln!(writer).unwrap();
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Location {
    point: Point,
    dist: i32,
}

impl Location {
    fn new(point: Point, dist: i32) -> Location {
        Location { point, dist }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

// use dijkstra
fn solve1(origin: &Point, target: &Point, map: &mut Map) -> i32 {
    let mut visited: HashMap<Point, Location> = HashMap::new();
    let mut unvisited: BinaryHeap<Location> = BinaryHeap::new();
    unvisited.push(Location::new(origin.clone(), 0));

    while !visited.contains_key(&target) {
        let loc: Location = unvisited.pop().unwrap();
        for p in map.neighbours(&loc.point).iter() {
            if !visited.contains_key(&p) {
                unvisited.push(Location::new(p.clone(), loc.dist + 1));
            }
        }
        visited.insert(loc.point.clone(), loc);
    }
    return visited.get(target).unwrap().dist;
}

fn solve2(origin: &Point, limit: i32, map: &mut Map) -> usize {
    let mut visited: HashMap<Point, Location> = HashMap::new();
    let mut unvisited: BinaryHeap<Location> = BinaryHeap::new();
    unvisited.push(Location::new(origin.clone(), 0));

    while !unvisited.is_empty() {
        let loc: Location = unvisited.pop().unwrap();
        visited.insert(loc.point.clone(), loc.clone());
        for p in map.neighbours(&loc.point).iter() {
            if !visited.contains_key(&p) && (&loc.dist < &limit) {
                unvisited.push(Location::new(p.clone(), &loc.dist + 1));
            }
        }
    }
    return visited.len();
}

fn main() {
    let mut map = Map::new(1358);
    let origin = Point::new(1, 1);
    println!("Part 1: {}", solve1(&origin, &Point::new(31, 39), &mut map));
    // correct answer: 96
    println!("Part 2: {}", solve2(&origin, 50, &mut map));
    // correct answer: 141
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut map = Map::new(10);
        assert_eq!(11, solve1(&Point::new(1, 1), &Point::new(7, 4), &mut map));
    }

    #[test]
    fn test_location_cmp() {
        let l1 = Location::new(Point::new(1, 1), 2);
        let l2 = Location::new(Point::new(2, 2), 4);
        let l3 = Location::new(Point::new(-1, -1), 2);
        assert_eq!(Ordering::Greater, l1.cmp(&l2));
        assert_eq!(Ordering::Equal, l1.cmp(&l3));
    }

    #[test]
    fn test_neighbours() {
        let mut map = Map::new(10);
        assert_eq!(4, map.neighbours(&Point::new(3, 2)).len());
        assert_eq!(0, map.neighbours(&Point::new(5, 3)).len());
    }

    #[test]
    fn test_map() {
        let maps = ".#.####.##
..#..#...#
#....##...
###.#.###.
.##..#..#.
..##....#.
#...##.###
";
        let mut buf: Vec<u8> = vec![];
        let mut map = Map::new(10);
        map.write_map(&mut buf, 9, 6);
        assert_eq!(maps, String::from_utf8(buf).unwrap());
    }
}
