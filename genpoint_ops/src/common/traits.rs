pub trait CaulatingDistance {
    fn distance_from_origin(&self) -> f64;
}

pub trait DisplayPoint {
    fn display(&self);
}
