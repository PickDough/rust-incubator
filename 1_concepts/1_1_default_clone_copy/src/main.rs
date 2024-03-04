#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new(points: Vec<Point>) -> Self {
        assert!(points.len() > 0, "Polyline type represents a non-empty set of Point");
        Polyline { points }
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
    use crate::{Point, Polyline};

    #[test]
    #[should_panic(expected = "Polyline type represents a non-empty set of Point")]
    fn assert_polyline() {
        let poly = Polyline::new(vec![]);
    }

    #[test]
    fn assert_cloning() {
        let mut poly = Polyline::new(vec![Point::default(), Point {x: 10, ..Default::default()}]);
        let mut poly_clone = poly.clone();

        poly.at(1).x = 15;

        assert_eq!(10, poly_clone.at(1).x);
        assert_ne!(*poly.at(1), *poly_clone.at(1));
    }
}
