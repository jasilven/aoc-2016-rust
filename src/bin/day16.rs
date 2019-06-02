fn checksum(s: &str) -> String {
    let mut result = String::from("");
    for n in (0..=(s.len() - 2)).step_by(2) {
        let ss = match &s[n..n + 2] {
            "00" | "11" => "1",
            _ => "0",
        };
        result.push_str(ss);
    }
    if result.len() % 2 == 0 {
        return checksum(result.as_str());
    }
    result
}

fn generate(a: &str) -> String {
    let mut result = String::from(a);
    result.push_str("0");
    result.push_str(
        &a.chars()
            .rev()
            .map(|ch| match ch {
                '1' => '0',
                _ => '1',
            })
            .collect::<String>(),
    );
    result
}

fn solve(length: usize, s: &str) -> String {
    let mut gen_s = generate(s);
    while gen_s.len() < length {
        gen_s = generate(&gen_s);
    }
    checksum(&gen_s[..length])
}

fn main() {
    println!("Part 1: {}", &solve(272, "10001110011110000"));
    // correct answer is: 10010101010011101
    println!("Part 2: {}", &solve(35651584, "10001110011110000"));
    // correct answer is: 01100111101101111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!("100", checksum("110010110100"));
    }

    #[test]
    fn test_generate() {
        assert_eq!("100", generate("1"));
        assert_eq!("001", generate("0"));
        assert_eq!("1111000010100101011110000", generate("111100001010"));
        assert_eq!("11111000000", generate("11111"));
        assert_eq!("10000011110", generate("10000"));
    }

    #[test]
    fn test_solve() {
        assert_eq!("01100", solve(20, "10000"));
    }
}
