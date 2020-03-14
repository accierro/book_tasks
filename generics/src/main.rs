struct Point<T> {
    x: T,
    y: T,
}

// Note: we have to declare T just after impl so we can use it to specify
// that we're implemneting methods on the typy Point<T>. In that case Rust
// can identify that the type in the angle brackets in Point is a generic
// type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// That implementation mean that Point<f32> will have function distance_from_origin,
// but other instances of Point<T> where T is not of type f32 will not have this
// method defined
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// We can mixing types in that case function mixup takes itself and otehr
// point as parameters and produces a new point with x value from self
// and y value from another point
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 2.0 };

    let number_list = vec![34, 50, 25, 100, 64];

    let result = largest(&number_list);
    println!("{}", result);

    let char_list = vec!["a", "z", "d"];
    let result = largest(&char_list);

    println!("{}", result)
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
