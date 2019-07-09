use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Arg {
    Reg(usize),
    Value(i32),
}

impl FromStr for Arg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Arg::Reg(0)),
            "b" => Ok(Arg::Reg(1)),
            "c" => Ok(Arg::Reg(2)),
            "d" => Ok(Arg::Reg(3)),
            s => match s.parse::<i32>() {
                Ok(v) => Ok(Arg::Value(v)),
                Err(_) => Err("argument parse error".to_string()),
            },
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpCode {
    INC(Arg),
    DEC(Arg),
    CPY(Arg, Arg),
    JNZ(Arg, Arg),
    TGL(Arg),
}

pub fn parse_assembunny(fname: &str) -> Result<Vec<OpCode>, Box<Error>> {
    let mut result: Vec<OpCode> = vec![];

    for line in BufReader::new(File::open(fname)?).lines() {
        let line = line?;
        let mut words = line.split(' ');
        let opcode = words.next().ok_or("opcode parse error")?;
        let a: Arg = words.next().ok_or("arg1 parse error")?.parse()?;
        let b = words.next().ok_or("arg2 parse error");
        let oc = match opcode {
            "inc" => OpCode::INC(a),
            "dec" => OpCode::DEC(a),
            "cpy" => OpCode::CPY(a, b?.parse()?),
            "jnz" => OpCode::JNZ(a, b?.parse()?),
            "tgl" => OpCode::TGL(a),
            s => return Err(format!("invalid opcode: {}", s).into()),
        };
        result.push(oc);
    }

    Ok(result)
}

pub fn execute_assembunny(prog: &mut [OpCode], regs: &mut [i32]) -> Result<(), Box<Error>> {
    let mut pc: i32 = 0;
    let end = prog.len() as i32;
    while (pc < end) && (pc >= 0) {
        match prog[pc as usize] {
            OpCode::INC(Arg::Reg(r)) => regs[r] += 1,
            OpCode::DEC(Arg::Reg(r)) => regs[r] -= 1,
            OpCode::CPY(Arg::Value(i), Arg::Reg(r)) => regs[r] = i,
            OpCode::CPY(Arg::Reg(r), Arg::Reg(rr)) => regs[rr] = regs[r],
            OpCode::JNZ(Arg::Value(i), Arg::Value(ii)) => {
                if i != 0 {
                    pc += ii;
                    continue;
                }
            }
            OpCode::JNZ(Arg::Value(i), Arg::Reg(r)) => {
                if i != 0 {
                    pc += regs[r];
                    continue;
                }
            }
            OpCode::JNZ(Arg::Reg(r), Arg::Value(i)) => {
                if regs[r] != 0 {
                    pc += i;
                    continue;
                }
            }
            OpCode::JNZ(Arg::Reg(r), Arg::Reg(rr)) => {
                if regs[r] != 0 {
                    pc += regs[rr];
                    continue;
                }
            }
            OpCode::TGL(a) => {
                let target = pc
                    + match a {
                        Arg::Value(i) => i,
                        Arg::Reg(r) => regs[r],
                    };

                if (target < end) && (target >= 0) {
                    prog[target as usize] = match &prog[target as usize] {
                        OpCode::INC(a) => OpCode::DEC(*a),
                        OpCode::DEC(a) => OpCode::INC(*a),
                        OpCode::TGL(a) => OpCode::INC(*a),
                        OpCode::JNZ(a, b) => OpCode::CPY(*a, *b),
                        OpCode::CPY(a, b) => OpCode::JNZ(*a, *b),
                    };
                }
            }
            OpCode::DEC(Arg::Value(_))
            | OpCode::INC(Arg::Value(_))
            | OpCode::CPY(Arg::Value(_), Arg::Value(_))
            | OpCode::CPY(Arg::Reg(_), Arg::Value(_)) => {}
        }
        pc += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_day12() {
        let prog = parse_assembunny("resources/day12-test-input.txt").unwrap();
        let v = vec![
            OpCode::CPY(Arg::Value(41), Arg::Reg(0)),
            OpCode::INC(Arg::Reg(0)),
            OpCode::INC(Arg::Reg(0)),
            OpCode::DEC(Arg::Reg(0)),
            OpCode::JNZ(Arg::Reg(0), Arg::Value(2)),
            OpCode::DEC(Arg::Reg(0)),
        ];
        assert_eq!(v, prog);
    }

    #[test]
    fn test_parse_day23() {
        let prog = parse_assembunny("resources/day23-test-input.txt").unwrap();
        let v = vec![
            OpCode::CPY(Arg::Value(2), Arg::Reg(0)),
            OpCode::TGL(Arg::Reg(0)),
            OpCode::TGL(Arg::Reg(0)),
            OpCode::TGL(Arg::Reg(0)),
            OpCode::CPY(Arg::Value(1), Arg::Reg(0)),
            OpCode::DEC(Arg::Reg(0)),
            OpCode::DEC(Arg::Reg(0)),
        ];
        assert_eq!(v, prog);
    }

    #[test]
    fn test_execute_day12() {
        let mut regs = [0i32; 4];
        let mut prog = parse_assembunny("resources/day12-test-input.txt").unwrap();
        execute_assembunny(&mut prog, &mut regs).unwrap();
        assert_eq!(42, regs[0]);
    }

    #[test]
    fn test_execute_day23() {
        let mut regs = [0i32; 4];
        let mut prog = parse_assembunny("resources/day23-test-input.txt").unwrap();
        execute_assembunny(&mut prog, &mut regs).unwrap();
        assert_eq!(3, regs[0]);
    }
}
