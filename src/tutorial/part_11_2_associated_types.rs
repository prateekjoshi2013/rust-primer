//

#[derive(Debug)]
struct Mh {
    value: i32,
}

#[derive(Debug)]
struct KmH {
    value: i32,
}

trait DistanceThreeHours {
    type Output;

    fn distance(&self) -> Self::Output;
}

impl DistanceThreeHours for KmH {
    type Output = Km;

    fn distance(&self) -> Self::Output {
        Km {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mh {
    type Output = M;

    fn distance(&self) -> Self::Output {
        return M {
            value: self.value * 3,
        };
    }
}

#[derive(Debug)]
struct Km {
    value: i32,
}

#[derive(Debug)]
struct M {
    value: i32,
}

// when we have implementation for multiple types
// we use generic types

// if implementation of a trait is only for one type
// we use associated types
#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

trait Add<T> {
    type Output;
    fn add(&self, other: T) -> Self::Output;
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for i32 {
    type Output = i32;
    fn add(&self, other: i32) -> i32 {
        self + other
    }
}

pub fn main() {
    let km: KmH = KmH { value: 20 };
    println!("{:?}", km.distance());

    let mh = Mh { value: 30 };
    println!("{:?}", mh.distance());

    assert_eq!(
        (Point { x: 5, y: 4 }).add(Point { x: 4, y: 5 }),
        Point { x: 9, y: 9 }
    );

    assert_eq!(5 + 5, 10);
}
