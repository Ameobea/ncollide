//! Utilities useful for various generations tasks.

use std::num::Zero;
use std::mem;
use nalgebra::na;
use nalgebra::na::{Vec3, Dim, Indexable};

// FIXME: remove that in favor of `push_xy_circle` ?
/// Pushes a discretized counterclockwise circle to a buffer.
pub fn push_circle<N: FloatMath>(radius: N, nsubdiv: u32, dtheta: N, y: N, out: &mut Vec<Vec3<N>>) {
    let mut curr_theta: N = na::zero();

    for _ in range(0, nsubdiv) {
        out.push(Vec3::new(curr_theta.cos() * radius, y.clone(), curr_theta.sin() * radius));
        curr_theta = curr_theta + dtheta;
    }
}

/// Pushes a discretized counterclockwise circle to a buffer.
/// The circle is contained on the plane spanned by the `x` and `y` axis.
pub fn push_xy_circle<N: FloatMath, V: Dim + Indexable<uint, N> + Zero>(
                      radius:  N,
                      nsubdiv: u32,
                      dtheta:  N,
                      out:     &mut Vec<V>) {
    assert!(na::dim::<V>() >= 2);

    let mut curr_theta: N = na::zero();

    for _ in range(0, nsubdiv) {
        let mut pt = na::zero::<V>();

        pt.set(0, curr_theta.cos() * radius);
        pt.set(1, curr_theta.sin() * radius);
        out.push(pt);

        curr_theta = curr_theta + dtheta;
    }
}

/// Creates the faces from two circles with the same discretization.
pub fn push_ring_indices(base_lower_circle: u32,
                         base_upper_circle: u32,
                         nsubdiv:           u32,
                         out:               &mut Vec<Vec3<u32>>) {
    push_open_ring_indices(base_lower_circle, base_upper_circle, nsubdiv, out);

    // adjust the last two triangles
    out.push(Vec3::new(base_lower_circle + nsubdiv - 1, base_upper_circle + nsubdiv - 1, base_upper_circle));
    out.push(Vec3::new(base_lower_circle + nsubdiv - 1, base_upper_circle, base_lower_circle));
}

/// Creates the faces from two circles with the same discretization.
/// The created ring is open (the start and end points are not linked together).
pub fn push_open_ring_indices(base_lower_circle: u32,
                              base_upper_circle: u32,
                              nsubdiv:           u32,
                              out:               &mut Vec<Vec3<u32>>) {
    assert!(nsubdiv > 0);

    for i in range(0, nsubdiv - 1) {
        let bli = base_lower_circle + i;
        let bui = base_upper_circle + i;
        out.push(Vec3::new(bli, bui, bui + 1));
        out.push(Vec3::new(bli, bui + 1, bli + 1));
    }
}

/// Creates the faces from a circle and a point that is shared by all triangle.
pub fn push_degenerate_top_ring_indices(base_circle: u32,
                                        point:       u32,
                                        nsubdiv:     u32,
                                        out:         &mut Vec<Vec3<u32>>) {
    push_degenerate_open_top_ring_indices(base_circle, point, nsubdiv, out);

    out.push(Vec3::new(base_circle + nsubdiv - 1, point, base_circle));
}

/// Creates the faces from a circle and a point that is shared by all triangle.
pub fn push_degenerate_open_top_ring_indices(base_circle: u32,
                                             point:       u32,
                                             nsubdiv:     u32,
                                             out:         &mut Vec<Vec3<u32>>) {
    assert!(nsubdiv > 0);

    for i in range(0, nsubdiv - 1) {
        out.push(Vec3::new(base_circle + i, point, base_circle + i + 1));
    }
}

/// Pushes indices so that a circle is filled with triangles. Each triangle will have the
/// `base_circle` point in common.
/// Pushes `nsubdiv - 2` elements to `out`.
pub fn push_closed_circle_indices(base_circle: u32, nsubdiv: u32, out: &mut Vec<Vec3<u32>>) {
    for i in range(base_circle + 1, base_circle + nsubdiv - 1) {
        out.push(Vec3::new(base_circle, i, i + 1));
    }
}

/// Reverses the clockwising of a set of faces.
pub fn reverse_clockwising(indices: &mut [Vec3<u32>]) {
    for i in indices.mut_iter() {
        mem::swap(&mut i.x, &mut i.y);
    }
}

/// Duplicates the indices of each triangle on the given index buffer.
///
/// For example: [ (0.0, 1.0, 2.0) ] becomes: [ (0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (2.0, 2.0, 2.0)].
pub fn split_index_buffer(indices: &[Vec3<u32>]) -> Vec<Vec3<Vec3<u32>>> {
    let mut resi = Vec::new();

    for vertex in indices.iter() {
        resi.push(
            Vec3::new(
                Vec3::new(vertex.x, vertex.x, vertex.x),
                Vec3::new(vertex.y, vertex.y, vertex.y),
                Vec3::new(vertex.z, vertex.z, vertex.z)
                )
            );
    }

    resi
}
