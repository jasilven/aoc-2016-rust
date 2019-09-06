
fn solve() -> usize {
    return usize::from_str_radix("101010101010", 2).unwrap() - (4 * 633);
}

fn main() {
    println!("Part 1: {}", solve());
}