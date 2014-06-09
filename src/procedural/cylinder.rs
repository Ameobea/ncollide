use nalgebra::na;
use nalgebra::na::{Cast, Vec2, Vec3};
use procedural::{TriMesh, SplitIndexBuffer};
use procedural::utils;

/// Generates a cylinder with a given height and diameter.
pub fn cylinder<N: FloatMath + Cast<f64>>(diameter: N, height: N, nsubdiv: u32) -> TriMesh<N, Vec3<N>> {
    let mut cylinder = unit_cylinder(nsubdiv);

    cylinder.scale_by(&Vec3::new(diameter, height, diameter));

    cylinder
}

/// Generates a cylinder with unit height and diameter.
pub fn unit_cylinder<N: FloatMath + Cast<f64>>(nsubdiv: u32) -> TriMesh<N, Vec3<N>> {
    let two_pi: N   = Float::two_pi();
    let invsubdiv   = na::one::<N>() / na::cast(nsubdiv as f64);
    let dtheta      = two_pi * invsubdiv;
    let mut coords  = Vec::new();
    let mut indices = Vec::new();
    let mut normals;

    utils::push_circle(na::cast(0.5), nsubdiv, dtheta, na::cast(-0.5), &mut coords);

    normals = coords.clone();

    utils::push_circle(na::cast(0.5), nsubdiv, dtheta, na::cast(0.5),  &mut coords);

    utils::push_ring_indices(0, nsubdiv, nsubdiv, &mut indices);
    utils::push_filled_circle_indices(0, nsubdiv, &mut indices);
    utils::push_filled_circle_indices(nsubdiv, nsubdiv, &mut indices);

    let len             = indices.len();
    let bottom_start_id = len - (nsubdiv as uint - 2);
    utils::reverse_clockwising(indices.mut_slice_from(bottom_start_id));

    let mut indices = utils::split_index_buffer(indices.as_slice());

    /*
     * Compute uvs.
     */
    // bottom ring uvs
    let mut uvs    = Vec::with_capacity(coords.len());
    let mut curr_u = na::zero::<N>();
    for _ in range(0, nsubdiv) {
        uvs.push(Vec2::new(curr_u.clone(), na::zero()));
        curr_u = curr_u + invsubdiv;
    }
    

    // top ring uvs
    curr_u = na::zero();
    for _ in range(0, nsubdiv) {
        uvs.push(Vec2::new(curr_u.clone(), na::one()));
        curr_u = curr_u + invsubdiv;
    }

    /*
     * Adjust normals.
     */
    for n in normals.mut_iter() {
        n.x = n.x * na::cast(2.0);
        n.y = na::zero();
        n.z = n.z * na::cast(2.0);
    }

    normals.push(Vec3::y());  // top cap
    normals.push(-Vec3::y()); // bottom cap
    let nlen = normals.len() as u32;

    let top_start_id = len - 2 * (nsubdiv as uint - 2);

    for i in indices.mut_slice_to(top_start_id).mut_iter() {
        if i.x.z >= nsubdiv {
            i.x.z = i.x.z - nsubdiv;
        }
        if i.y.z >= nsubdiv {
            i.y.z = i.y.z - nsubdiv;
        }
        if i.z.z >= nsubdiv {
            i.z.z = i.z.z - nsubdiv;
        }
    }

    for i in indices.mut_slice(top_start_id, bottom_start_id).mut_iter() {
        i.x.z = nlen - 2;
        i.y.z = nlen - 2;
        i.z.z = nlen - 2;
    }

    for i in indices.mut_slice_from(bottom_start_id).mut_iter() {
        i.x.z = nlen - 1;
        i.y.z = nlen - 1;
        i.z.z = nlen - 1;
    }

    TriMesh::new(coords, Some(normals), Some(uvs), Some(SplitIndexBuffer(indices)))
}
