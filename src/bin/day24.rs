use permutohedron::Heap;
use std::cmp::{max, min, Ordering};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn parse_routemap(path: &str) -> Result<HashMap<(usize, usize), char>> {
    let data: Vec<String> = read_to_string(path)?
        .lines()
        .map(|s| s.to_owned())
        .collect();
    let mut result: HashMap<(usize, usize), char> = HashMap::new();
    for y in 0..data.len() {
        let chars: Vec<char> = data[y].chars().collect();
        for x in 0..chars.len() {
            if chars[x] != '#' {
                result.insert((x, y), chars[x]);
            }
        }
    }
    Ok(result)
}

fn all_routes(route_map: &HashMap<(usize, usize), char>) -> Result<Vec<Vec<(usize, usize)>>> {
    let mut locations: Vec<(usize, usize)> = route_map
        .iter()
        .filter(|(_, val)| *val != &'.')
        .map(|(key, _)| key)
        .cloned()
        .collect();
    let heap = Heap::new(&mut locations);
    let permutations: Vec<Vec<(usize, usize)>> = heap.collect();
    let result: Vec<Vec<(usize, usize)>> = permutations
        .iter()
        .filter(|v| route_map.get(v.first().unwrap()).unwrap() == &'0')
        .cloned()
        .collect();
    Ok(result)
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Location {
    steps: usize,
    xy: (usize, usize),
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.xy.cmp(&other.xy))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbours(
    route_map: &HashMap<(usize, usize), char>,
    xy: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    let keys = vec![
        (xy.0, xy.1 - 1),
        (xy.0, xy.1 + 1),
        (xy.0 - 1, xy.1),
        (xy.0 + 1, xy.1),
    ];
    for key in keys.iter() {
        if route_map.contains_key(&key) {
            result.push(*key);
        }
    }
    result
}

fn shortest_path_len(
    route_map: &HashMap<(usize, usize), char>,
    a: (usize, usize),
    b: (usize, usize),
) -> usize {
    let mut seen = HashSet::new();
    let mut unvisited = BinaryHeap::new();
    unvisited.push(Location { steps: 0, xy: a });
    loop {
        let location = unvisited.pop().unwrap();
        if location.xy == b {
            return location.steps;
        }
        seen.insert(location.xy);
        for neighbour in neighbours(&route_map, &location.xy) {
            if !seen.contains(&neighbour) {
                let l = Location {
                    xy: neighbour,
                    steps: location.steps + 1,
                };
                seen.insert(l.xy);
                unvisited.push(l);
            }
        }
    }
}

fn solve(
    route_map: &HashMap<(usize, usize), char>,
    routes: &Vec<Vec<(usize, usize)>>,
) -> Result<usize> {
    let mut result = usize::max_value();
    let mut len_memo: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    for route in routes {
        let mut route_len = 0;
        for edge in route.windows(2) {
            if let Some(len) = len_memo.get(&(min(edge[0], edge[1]), max(edge[0], edge[1]))) {
                route_len += len;
            } else {
                let l = shortest_path_len(&route_map, edge[0], edge[1]);
                len_memo.insert((min(edge[0], edge[1]), max(edge[0], edge[1])), l);
                route_len += l;
            }
        }
        if route_len < result {
            result = route_len;
        }
    }
    Ok(result)
}

fn main() {
    let route_map = parse_routemap("resources/day24-input.txt").unwrap();
    let routes = all_routes(&route_map).unwrap();
    let start_pos = routes.first().unwrap().first().unwrap();
    println!("Part 1: {}", solve(&route_map, &routes).unwrap());

    let mut routes2: Vec<Vec<(usize, usize)>> = routes.clone();
    for route in routes2.iter_mut() {
        route.push(start_pos.clone());
    }
    println!("Part 2: {}", solve(&route_map, &routes2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let route_map = parse_routemap("resources/day24-test-input.txt").unwrap();
        let routes = all_routes(&route_map).unwrap();
        assert_eq!(14, solve(&route_map, &routes).unwrap());
    }
}
