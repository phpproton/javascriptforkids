use std::cmp::Ordering;
use std::f64::consts::PI;

fn compare<T: Ord + std::fmt::Display>(a: T, b: T) -> Ordering {
    match a.cmp(&b) {
        Ordering::Greater => {
            println!("{a} is greater than {b}");
            Ordering::Greater
        }

        Ordering::Less => {
            println!("{a} is less than {b}");
            Ordering::Less
        }

        Ordering::Equal => {
            println!("{a} and {b} are equal");
            Ordering::Equal
        }
    }
}

enum Shape {
    Square(f64),
    Rectangle(f64, f64),
    Circle(f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Square(s) => s * s,
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => PI * r * r,
    }
}

fn main() {
    {
        let square = Shape::Square(10.0);
        println!("Area: {}", calculate_area(square));
        let rect = Shape::Rectangle(2.0, 3.0);
        println!("Area: {}", calculate_area(rect));
        let circle = Shape::Circle(9.0);
        println!("Area: {}", calculate_area(circle));
    }

    let result = compare(40, 42);
    println!("Result {:?}", result);
}
