mod traits;

use std::{f32::consts::PI, ops::Mul};

use self::traits::shape::Shape;

fn main() {
    struct Rectangle {
        length: f64,
        breadth: f64,
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.breadth
        }
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            self.radius.mul(f64::from(PI)).powi(2)
        }
    }

    let rec = Rectangle {
        length: 3.,
        breadth: 4.,
    };
    println!("{}", &rec.area());

    let cir = Circle { radius: 43. };
    println!("{}", cir.area());
}
