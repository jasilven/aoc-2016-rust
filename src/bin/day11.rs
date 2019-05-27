fn solve(items: &Vec<usize>) -> usize {
    let mut result: usize = 0;
    for n in 1..items.len() {
        result += items.iter().take(n).sum::<usize>();
    }
    2 * result - (items.len() - 1) * 3
}

fn main() {
    println!("Part 1: {}", &solve(&vec![8, 2, 0, 0]));
    println!("Part 2: {}", &solve(&vec![12, 2, 0, 0]));
}
