#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: Vec<Point>,
}

struct NonZeroVec<T>(T, Vec<T>);

impl Polyline {
    fn new(points: NonZeroVec<Point>) -> Self {
        let mut v = vec![points.0];
        v.extend(points.1);
        Polyline { points: v }
    }
    
    fn at(&mut self, idx: usize) -> &mut Point {
        &mut self.points[idx]
    }
}


fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use crate::{NonZeroVec, Point, Polyline};

    #[test]
    fn assert_cloning() {
        let mut poly = Polyline::new(NonZeroVec(Point::default(), vec![Point {x: 10, ..Default::default()}]));
        let mut poly_clone = poly.clone();

        poly.at(1).x = 15;

        assert_eq!(10, poly_clone.at(1).x);
        assert_ne!(*poly.at(1), *poly_clone.at(1));
    }
}
