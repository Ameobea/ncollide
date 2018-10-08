use test;
use test::Bencher;

use na::Isometry3;
use ncollide3d::bounding_volume::AABB;
use ncollide3d::partitioning::{DBVTLeaf, DBVT};

fn create_entity() -> DBVTLeaf<f32, (), AABB<f32>> {
    unimplemented!()
}

fn populate_dbvt(entity_count: usize) -> DBVT<f32, (), AABB<f32>> {
    let mut dbvt = DBVT::new();
    for _ in 0..entity_count {
        let leaf = create_entity();
        dbvt.insert(leaf);
    }
    unimplemented!()
}

#[bench]
fn dbvt_insert_remove_small(bh: &mut Bencher) {}

#[bench]
fn dbvt_insert_remove_medium(bh: &mut Bencher) {}

#[bench]
fn dbvt_insert_remove_large(bh: &mut Bencher) {}

#[bench]
fn dbvt_insert_remove_huge(bh: &mut Bencher) {}
