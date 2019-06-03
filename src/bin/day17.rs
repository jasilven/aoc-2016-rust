extern crate md5;
mod aoc;
use aoc::point::{move_point, Point};

#[derive(Debug)]
struct Location {
    point: Point,
    code: String,
}

fn is_open(ch: char) -> bool {
    match ch {
        'b' | 'c' | 'd' | 'e' | 'f' => true,
        _ => false,
    }
}

fn avail_locations(cur_loc: &Location) -> Vec<Location> {
    let mut result: Vec<Location> = vec![];
    let hash = format!("{:x}", md5::compute(&cur_loc.code));
    let dirs = ['U', 'D', 'L', 'R'];
    for i in 0..dirs.len() {
        let ch = hash.chars().nth(i).unwrap();
        if is_open(ch) {
            let p = move_point(&cur_loc.point, dirs[i], 1).unwrap();
            if (p.x < 4) & (p.x >= 0) & (p.y < 4) & (p.y >= 0) {
                result.push(Location {
                    point: p,
                    code: format!("{}{}", cur_loc.code, dirs[i]),
                })
            }
        }
    }
    result
}

fn find_paths(start: &Point, target: &Point, code: &str) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    let mut locs: Vec<Location> = vec![Location {
        point: start.clone(),
        code: code.to_owned(),
    }];
    loop {
        if let Some(l) = locs.pop() {
            if &l.point == target {
                let path: String = l.code.chars().filter(|ch| ch.is_uppercase()).collect();
                paths.push(path);
            } else {
                locs.append(&mut avail_locations(&l));
            }
        } else {
            break;
        }
    }
    paths
}

fn solve1(paths: &Vec<String>) -> String {
    paths
        .iter()
        .min_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .clone()
}

fn solve2(paths: &Vec<String>) -> usize {
    paths
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len()
}

fn main() {
    let start = Point::new(0, 0);
    let target = Point::new(3, 3);
    let paths = find_paths(&start, &target, "vwbaicqe");
    if paths.is_empty() {
        panic!("NOT FOUND");
    }
    println!("Part 1: {}", solve1(&paths));
    // correct answer: DRDRULRDRD
    println!("Part 2: {}", solve2(&paths));
    // correct answer: 384
}

#[cfg(test)]
mod tests {
    use super::*;
    const START: Point = Point { x: 0, y: 0 };
    const TARGET: Point = Point { x: 3, y: 3 };

    #[test]
    fn test_solve1() {
        let p1 = find_paths(&START, &TARGET, "ihgpwlah");
        assert_eq!("DDRRRD", &solve1(&p1));

        let p2 = find_paths(&START, &TARGET, "kglvqrro");
        assert_eq!("DDUDRLRRUDRD", &solve1(&p2));

        let p3 = find_paths(&START, &TARGET, "ulqzkmiv");
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", &solve1(&p3));
    }

    #[test]
    fn test_solve2() {
        let p1 = find_paths(&START, &TARGET, "ihgpwlah");
        assert_eq!(370, solve2(&p1));

        let p2 = find_paths(&START, &TARGET, "kglvqrro");
        assert_eq!(492, solve2(&p2));

        let p3 = find_paths(&START, &TARGET, "ulqzkmiv");
        assert_eq!(830, solve2(&p3));
    }
}
