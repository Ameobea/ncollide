extern crate nalgebra as na;
extern crate ncollide2d;

use na::{Isometry2, Vector2};
use ncollide2d::shape::Ball;
use ncollide2d::bounding_volume;
use ncollide2d::broad_phase::{BroadPhase, DBVTBroadPhase};

fn main() {
    /*
     * Create the objects.
     */
    let poss = [
        Isometry2::new(Vector2::new(0.0, 0.0), na::zero()),
        Isometry2::new(Vector2::new(0.0, 0.5), na::zero()),
        Isometry2::new(Vector2::new(0.5, 0.0), na::zero()),
        Isometry2::new(Vector2::new(0.5, 0.5), na::zero()),
    ];

    // We will use the same shape for the four objects.
    let ball = Ball::new(0.5);

    /*
     * Create the broad phase.
     */
    let mut bf = DBVTBroadPhase::new(0.2);

    // First parameter: the object bounding box.
    // Second parameter:  some data (here, the id that identify each object).
    let proxy1 = bf.create_proxy(bounding_volume::aabb(&ball, &poss[0]), 0);
    let proxy2 = bf.create_proxy(bounding_volume::aabb(&ball, &poss[1]), 1);
    let _ = bf.create_proxy(bounding_volume::aabb(&ball, &poss[2]), 2);
    let _ = bf.create_proxy(bounding_volume::aabb(&ball, &poss[3]), 3);

    // Update the broad phase.
    // The collision filter (first closure) prevents self-collision.
    bf.update(&mut |a, b| *a != *b, &mut |_, _, _| {});

    assert!(bf.num_interferences() == 6);

    // Remove two objects.
    bf.remove(&[proxy1, proxy2], &mut |_, _| {});

    // Update the broad phase.
    // The collision filter (first closure) prevents self-collision.
    bf.update(&mut |a, b| *a != *b, &mut |_, _, _| {});

    assert!(bf.num_interferences() == 1)
}
