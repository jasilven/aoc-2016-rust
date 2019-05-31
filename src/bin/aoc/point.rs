use std::cmp::Ordering;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
pub fn manh_dist(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    #[allow(dead_code)]
    pub fn manh_dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        let origin = Point { x: 0, y: 0 };
        self.manh_dist(&origin).cmp(&other.manh_dist(&origin))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_cmp() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        let p3 = Point::new(-1, -1);
        assert_eq!(Ordering::Less, p1.cmp(&p2));
        assert_eq!(Ordering::Equal, p1.cmp(&p3));
    }

    #[test]
    fn test_manh_dist() {
        let origin = Point { x: 0, y: 0 };
        let p1 = Point { x: 1, y: 1 };
        assert_eq!(2, origin.manh_dist(&p1));
        assert_eq!(4, p1.manh_dist(&Point { x: -1, y: -1 }));
        assert_eq!(3, origin.manh_dist(&Point { x: 3, y: 0 }));
    }

}
