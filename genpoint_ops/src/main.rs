mod common;
mod util;

use std::fmt::Display;

use self::common::traits::{CaulatingDistance, DisplayPoint};
use self::util::round::round;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> CaulatingDistance for Point<T, U>
where
    T: Copy + Into<f64>,
    U: Copy + Into<f64>,
{
    fn distance_from_origin(&self) -> f64 {
        let distance_x: f64 = self.x.into().powi(2);
        let distance_y: f64 = self.y.into().powi(2);
        let euclidean_d = (distance_x + distance_y).sqrt();
        round(euclidean_d, 2)
    }
}

impl<T, U> DisplayPoint for Point<T, U>
where
    T: Display,
    U: Display,
{
    fn display(&self) {
        println!("({}, {})", self.x, self.y)
    }
}

fn swap_coordinates<T, U>(p: &Point<T, U>) -> Point<f64, f64>
where
    T: Into<f64> + Clone,
    U: Into<f64> + Clone,
{
    Point {
        x: p.y.clone().into(),
        y: p.x.clone().into(),
    }
}

fn midpoint<T, U>(point1: Option<&Point<T, U>>, point2: Option<&Point<T, U>>) -> Option<Point<T, T>>
where
    T: Into<f64> + From<f64> + Copy,
    U: Into<f64> + Copy,
{
    match (point1, point2) {
        (Some(p1), Some(p2)) => {
            let x_mid = (p1.x.into() + p2.x.into()) / 2.0;
            let y_mid = (p1.y.into() + p2.y.into()) / 2.0;
            let out_p = Point {
                x: T::from(x_mid),
                y: T::from(y_mid),
            };
            return Some(out_p);
        }
        _ => return None,
    }
}

fn main() {
    let one = Point { x: 2., y: 3. };
    let out = one.distance_from_origin();
    println!("{}", out);
    one.display();
    let re = Point { x: 3.3, y: 1. };
    let second_p = swap_coordinates(&re);
    second_p.display();
    let t = &re.distance_from_origin();
    println!("{}", t);

    let re_s = Some(&re);
    let one_s = Some(&one);
    let mid_point = midpoint(re_s, one_s);

    match mid_point {
        Some(mp) => mp.display(),
        _ => panic!("Not found"),
    }
}
