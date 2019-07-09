use aoc_2016::{execute_assembunny, parse_assembunny};

fn solve1() -> i32 {
    let mut prog = parse_assembunny("resources/day12-input.txt").unwrap();
    let mut regs = [0i32; 4];
    execute_assembunny(&mut prog, &mut regs).unwrap();
    regs[0]
}

fn solve2() -> i32 {
    let mut prog = parse_assembunny("resources/day12-input.txt").unwrap();
    let mut regs = [0i32; 4];
    regs[2] = 1;
    execute_assembunny(&mut prog, &mut regs).unwrap();
    regs[0]
}

fn main() {
    println!("Part 1: {}", solve1());
    // correct answer: 318009
    println!("Part 2: {}", solve2());
    // correct answer: 9227663
}
