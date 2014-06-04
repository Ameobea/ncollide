use nalgebra::na;
use nalgebra::na::{Cast, Vec3};
use procedural::{TriMesh, SplitIndexBuffer};
use procedural::utils;

/// Generates a cone with a given height and diameter.
pub fn cone<N: FloatMath + Cast<f64>>(diameter: N, height: N, nsubdiv: u32) -> TriMesh<N, Vec3<N>> {
    let mut cone = unit_cone(nsubdiv);

    cone.scale_by(&Vec3::new(diameter, height, diameter));

    cone
}

/// Generates a cone with unit height and diameter.
pub fn unit_cone<N: FloatMath + Cast<f64>>(nsubdiv: u32) -> TriMesh<N, Vec3<N>> {
    let two_pi: N   = Float::two_pi();
    let dtheta      = two_pi / na::cast(nsubdiv as f64);
    let mut coords  = Vec::new();
    let mut indices = Vec::new();
    let mut normals;

    utils::push_circle(na::cast(0.5), nsubdiv, dtheta, na::cast(-0.5), &mut coords);

    normals = coords.clone();

    coords.push(Vec3::new(na::zero(), na::cast(0.5), na::zero()));

    utils::push_degenerate_top_ring_indices(0, coords.len() as u32 - 1, nsubdiv, &mut indices);
    utils::push_closed_circle_indices(0, nsubdiv, &mut indices);

    /*
     * Normals.
     */
    let mut indices = utils::split_index_buffer(indices.as_slice());

    // adjust the normals:
    let shift: N = na::cast(0.05 / 0.475);
    let div = (shift * shift + na::cast(0.25)).sqrt();
    for n in normals.mut_iter() {
        n.y = n.y + shift;
        // FIXME: n / div does not work?
        n.x = n.x / div;
        n.y = n.y / div;
        n.z = n.z / div;
    }

    // normal for the basis
    normals.push(Vec3::new(na::zero(), -na::one::<N>(), na::zero()));

    let ilen = indices.len();
    let nlen = normals.len() as u32;
    for (id, i) in indices.mut_slice_to(ilen - (nsubdiv as uint - 2)).mut_iter().enumerate() {
        i.y.z = id as u32;
    }

    for i in indices.mut_slice_from(ilen - (nsubdiv as uint - 2)).mut_iter() {
        i.x.z = nlen - 1;
        i.y.z = nlen - 1;
        i.z.z = nlen - 1;
    }

    // normal for the body

    TriMesh::new(coords, Some(normals), None, Some(SplitIndexBuffer(indices)))

    // XXX: uvs
}
