use na;
use math::{Point, Vector};
use geometry::Proximity;
use entities::shape::Ball;

/// Proximity between balls.
#[inline]
pub fn ball_against_ball<P>(center1: &P, b1: &Ball<<P::Vect as Vector>::Scalar>,
                            center2: &P, b2: &Ball<<P::Vect as Vector>::Scalar>,
                            margin: <P::Vect as Vector>::Scalar)
                            -> Proximity
    where P: Point {
    let r1         = b1.radius();
    let r2         = b2.radius();
    let delta_pos  = *center2 - *center1;
    let distance_squared     = na::norm_squared(&delta_pos);
    let sum_radius = r1 + r2;
    let sum_radius_with_error = sum_radius + margin;

    if distance_squared <= sum_radius_with_error * sum_radius_with_error {
        if distance_squared <= sum_radius * sum_radius {
            Proximity::Intersecting
        }
        else {
            Proximity::WithinMargin
        }
    }
    else {
        Proximity::Disjoint
    }
}