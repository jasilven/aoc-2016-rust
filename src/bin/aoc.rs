use std::fs::File;
use std::io::Read;

pub fn slurp(fname: &str) -> Result<String, std::io::Error> {
    let mut data = String::new();
    let mut file = File::open(fname)?;
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn manh_dist(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manh_dist_test() {
        assert_eq!(2, manh_dist((0, 0), (1, 1)));
        assert_eq!(4, manh_dist((-1, -1), (1, 1)));
    }
}
