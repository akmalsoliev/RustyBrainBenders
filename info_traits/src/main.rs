mod traits;

use std::fmt::Display;
use std::ops::Mul;

use self::traits::information::Information;

fn main() {
    let number = 5;
    println!("{}", number.info());

    let text = "Rust";
    println!("{}", &text.info());

    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T: Copy + Display> Information for Point<T> {
        fn info(&self) -> String {
            format!("({}, {})", self.x, self.y)
        }
    }

    impl<T: Mul<Output = T>> Mul for Point<T> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    let point = Point { x: 3.0, y: 4.0 };
    println!("{}", point.info());

    let another_point = Point { x: 1.0, y: 2.0 };
    let sum_point = point * another_point;
    println!("{}", sum_point.info());
}
