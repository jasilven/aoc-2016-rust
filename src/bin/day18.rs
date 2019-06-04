use std::fs::read_to_string;

fn map_tile(triple: &str) -> char {
    match triple {
        "^^." | ".^^" | "^.." | "..^" => '^',
        _ => '.',
    }
}

fn count_safe_tiles(row: &str) -> usize {
    row.chars().filter(|ch| ch == &'.').count()
}

fn solve(row: &str, row_cnt: usize) -> usize {
    let mut result = count_safe_tiles(row);
    let mut cur_row = format!(".{}.", row);
    for _ in 0..row_cnt - 1 {
        let mut next_row = String::from("");
        for i in 0..=(cur_row.len() - 3) {
            next_row.push(map_tile(&cur_row[i..(i + 3)]));
        }
        result += count_safe_tiles(&next_row);
        cur_row = format!(".{}.", next_row);
    }
    result
}

fn main() {
    let first_row = read_to_string("resources/day18-input.txt").unwrap();
    println!("Part 1: {}", &solve(&first_row.trim(), 40));
    // correct answer: 1956
    println!("Part 2: {}", &solve(&first_row.trim(), 400_000));
    // correct answer: 19995121
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe_tiles() {
        assert_eq!(2, count_safe_tiles(".^."));
        assert_eq!(0, count_safe_tiles("^"));
        assert_eq!(0, count_safe_tiles(""));
        assert_eq!(1, count_safe_tiles("."));
        assert_eq!(3, count_safe_tiles("..."));
        assert_eq!(3, count_safe_tiles("^.^.^.^"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(6, solve("..^^.", 3));
        assert_eq!(38, solve(".^^.^.^^^^", 10));
    }
}
