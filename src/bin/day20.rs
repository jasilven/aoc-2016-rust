mod aoc;
use aoc::parse_ints;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(fname: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for line in BufReader::new(File::open(fname).unwrap()).lines() {
        let line = line.unwrap().replace("-", " ");
        let ints = parse_ints(&line);
        result.push((ints[0] as usize, ints[1] as usize));
    }
    result
}

fn get_sorted_cands(ranges: &[(usize, usize)], maxip: usize) -> Vec<usize> {
    let mut cands: Vec<usize> = ranges
        .iter()
        .map(|(_, hi)| if *hi < maxip { hi + 1 as usize } else { *hi })
        .collect();
    cands.sort();
    cands
}

fn solve1(ranges: &[(usize, usize)], maxip: usize) -> usize {
    let mut result = 0;
    for cand in get_sorted_cands(&ranges, maxip) {
        if ranges.iter().all(|(lo, hi)| (cand < *lo) | (cand > *hi)) {
            result = cand;
            break;
        }
    }
    result
}

fn split_by_overlap(
    ranges: &[(usize, usize)],
    range: &(usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut overlap = vec![];
    let mut non_overlap = vec![];
    for r in ranges {
        if ((r.0 >= range.0) & (r.0 <= range.1)) | ((r.1 >= range.0) & (r.1 <= range.1)) {
            overlap.push(r.clone());
        } else {
            non_overlap.push(r.clone());
        }
    }
    (overlap, non_overlap)
}

fn combine_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for cur_range in ranges.iter() {
        let (overlap, non_overlap) = split_by_overlap(ranges, cur_range);
        if overlap.len() > 1 {
            let lowest = overlap.iter().min_by(|r1, r2| r1.0.cmp(&r2.0)).unwrap();
            let highest = overlap.iter().max_by(|r1, r2| r1.1.cmp(&r2.1)).unwrap();
            result.push((lowest.0, highest.1));
            for r in non_overlap {
                result.push(r);
            }
            break;
        }
    }
    result
}

fn solve2(ranges: &[(usize, usize)], maxip: usize) -> usize {
    let mut tmp_ranges = ranges.to_owned();
    loop {
        let new_ranges = combine_ranges(&tmp_ranges);
        if new_ranges.is_empty() {
            break;
        } else {
            tmp_ranges = new_ranges;
        }
    }
    let blocked_cnt = tmp_ranges.iter().fold(0, |acc, (lo, hi)| acc + hi - lo + 1);
    maxip - blocked_cnt + 1
}

fn main() {
    let maxip = 4294967295;
    let ranges = parse_input("resources/day20-input.txt");
    println!("Part 1: {:?}", solve1(&ranges, maxip));
    // correct answer: 14975795
    println!("Part 2: {:?}", solve2(&ranges, maxip));
    // correct answer: 101
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let ranges = parse_input("resources/day20-test-input.txt");
        assert_eq!(3, solve1(&ranges, 9));
    }

    #[test]
    fn test_solve2() {
        let ranges = parse_input("resources/day20-test-input.txt");
        assert_eq!(2, solve2(&ranges, 9));
    }
}
