// Think like y = mx + b, but in N dimensions.

use crate::{curve::ParametricCurve, Domain, Point};

pub struct Line<const N: usize> {
    components: [f64; N],
}

impl<const N: usize> ParametricCurve<N> for Line<N> {
    fn get_domain(&self) -> Domain {
        Domain::new(0.0, 1.0)
    }

    fn point(&self, t: f64) -> Point<N> {
        let mut coords = [0.0; N];
        for i in 0..N {
            coords[i] = self.components[i] * t;
        }
        Point { coords }
    }
}
