struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    fn perimeter(&self) -> f64 {
        3.14 * self.radius * self.radius
    }

    fn area(&self) -> f64 {
        2.0f64 * 3.14f64 * self.radius
    }

    fn contains(&self, other: Circle) -> bool {
        self.radius > other.radius
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn circle_contains_another_circle() {
        let c1 = Circle::new(3.0f64);
        let c2 = Circle::new(2.0f64);
        assert!(c1.contains(c2));
    }

    #[test]
    fn circle_does_not_contain_another_circle() {
        let c2 = Circle::new(3.0f64);
        let c1 = Circle::new(2.0f64);
        assert!(!c1.contains(c2));
    }
}
