extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum OpCode {
    Cpi(i32, String),    // copy integer to register
    Cpr(String, String), // copy register to register
    Inc(String),         // inc register
    Dec(String),         // dec register
    Jnzr(String, i32),   // jump if register is not zero
    Jnzi(i32, i32),      // jump if value is not zero
}

fn parse_assembunny(fname: &str) -> Vec<OpCode> {
    let mut result = vec![];
    let cpi_re = Regex::new(r"^cpy (-?\d+) ([a-z])$").unwrap();
    let cpr_re = Regex::new(r"^cpy ([a-z]) ([a-z])$").unwrap();
    let inc_re = Regex::new(r"^inc ([a-z])$").unwrap();
    let dec_re = Regex::new(r"^dec ([a-z])$").unwrap();
    let jnzr_re = Regex::new(r"^jnz ([a-z]) (-?\d+)$").unwrap();
    let jnzi_re = Regex::new(r"^jnz (-?\d+) (-?\d+)$").unwrap();
    for line in BufReader::new(File::open(fname).unwrap()).lines() {
        let line = line.unwrap();
        match true {
            true if cpi_re.is_match(&line) => {
                let caps = cpi_re.captures(&line).unwrap();
                result.push(OpCode::Cpi(
                    caps[1].parse::<i32>().unwrap(),
                    caps[2].to_owned(),
                ));
            }
            true if cpr_re.is_match(&line) => {
                let caps = cpr_re.captures(&line).unwrap();
                result.push(OpCode::Cpr(caps[1].to_owned(), caps[2].to_owned()));
            }
            true if inc_re.is_match(&line) => {
                let caps = inc_re.captures(&line).unwrap();
                result.push(OpCode::Inc(caps[1].to_owned()));
            }
            true if dec_re.is_match(&line) => {
                let caps = dec_re.captures(&line).unwrap();
                result.push(OpCode::Dec(caps[1].to_owned()));
            }
            true if jnzr_re.is_match(&line) => {
                let caps = jnzr_re.captures(&line).unwrap();
                result.push(OpCode::Jnzr(
                    caps[1].to_owned(),
                    caps[2].parse::<i32>().unwrap(),
                ));
            }
            true if jnzi_re.is_match(&line) => {
                let caps = jnzi_re.captures(&line).unwrap();
                result.push(OpCode::Jnzi(
                    caps[2].parse::<i32>().unwrap(),
                    caps[2].parse::<i32>().unwrap(),
                ));
            }
            _ => panic!(format!("unknown opcode:{}", line)),
        }
    }
    result
}

fn execute(prog: &Vec<OpCode>, regs: &mut HashMap<String, i32>) {
    let ip = "ip".to_owned();
    while *regs.get(&ip).unwrap() >= 0 && *regs.get(&ip).unwrap() < (prog.len() as i32) {
        match &prog[*regs.get(&ip).unwrap() as usize] {
            OpCode::Cpi(i, r) => regs.insert(r.to_string(), *i),
            OpCode::Cpr(r1, r2) => regs.insert(r2.to_string(), regs.get(r1).unwrap().clone()),
            OpCode::Inc(r) => regs.insert(r.to_string(), regs.get(r).unwrap() + 1),
            OpCode::Dec(r) => regs.insert(r.to_string(), regs.get(r).unwrap() - 1),
            OpCode::Jnzr(r, i) => {
                if regs.get(r).unwrap() != &0 {
                    regs.insert(ip.to_owned(), regs.get(&ip).unwrap() + i - 1);
                };
                None
            }
            OpCode::Jnzi(i1, i2) => {
                if i1 != &0 {
                    regs.insert(ip.to_owned(), regs.get(&ip).unwrap() + i2 - 1);
                };
                None
            }
        };
        regs.insert(ip.to_owned(), regs.get(&ip).unwrap() + 1);
    }
}

fn solve1(prog: &Vec<OpCode>) -> i32 {
    let mut regs: HashMap<String, i32> = HashMap::new();
    for reg in &["a", "b", "c", "d", "ip"] {
        regs.insert(reg.to_string(), 0);
    }

    execute(&prog, &mut regs);
    *regs.get("a").unwrap()
}

fn solve2(prog: &Vec<OpCode>) -> i32 {
    let mut regs: HashMap<String, i32> = HashMap::new();
    for reg in &["a", "b", "c", "d", "ip"] {
        regs.insert(reg.to_string(), 0);
    }
    regs.insert("c".to_string(), 1);

    execute(&prog, &mut regs);
    *regs.get("a").unwrap()
}

fn main() {
    let prog = parse_assembunny("resources/day12-input.txt");
    println!("Part 1: {}", &solve1(&prog));
    // correct answer: 318009
    println!("Part 2: {}", &solve2(&prog));
    // correct answer: 9227663
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let prog = parse_assembunny("resources/day12-test-input.txt");
        assert_eq!(42, solve1(&prog));
    }
}
