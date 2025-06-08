use crate::{Domain, Point};

pub trait ParametricCurve<const N: usize> {
    fn get_domain(&self) -> Domain;
    fn point(&self, t: f64) -> Point<N>;
}
