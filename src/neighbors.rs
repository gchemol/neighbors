// neighbors.rs
// :PROPERTIES:
// :header-args: :tangle src/neighbors.rs
// :END:

// [[file:~/Workspace/Programming/neighbors/neighbors.note::*neighbors.rs][neighbors.rs:1]]
use std::collections::HashMap;
use std::ops::Mul;

use crate::periodic::UnitCell;
use octree::Octree;

use cgmath::prelude::*;
use cgmath::{Matrix, Vector3};

/// search neighbors for aperiodic system
pub fn neighbors_for_aperiodic(
    positions: &Vec<[f64; 3]>,
    cutoff: f64,
) -> HashMap<usize, Vec<(usize, f64, Vector3<f64>)>> {
    let tree = Octree::new(&positions);

    let mut kneighbors = HashMap::new();
    // run queries over all relevant images
    for (current, &p) in positions.iter().enumerate() {
        let mut neighbors = Vec::new();
        let nps = tree.search(p, cutoff);
        for &(index, distance) in nps.iter() {
            if index != current {
                neighbors.push((index, distance, Vector3::new(0.0, 0.0, 0.0)));
            }
        }
        kneighbors.insert(current, neighbors);
    }

    kneighbors
}

/// search neighbors for periodic system
pub fn neighbors_for_periodic(
    positions: &Vec<[f64; 3]>,
    cell: UnitCell,
    cutoff: f64,
) -> HashMap<usize, Vec<(usize, f64, Vector3<f64>)>> {
    let images = cell.relevant_images(cutoff);

    let tree = Octree::new(&positions);

    let mut kneighbors = HashMap::new();
    // run queries over all relevant images
    for (current, &p) in positions.iter().enumerate() {
        // to avoid octree building for each image
        // we mirror the query points and then mirror back
        let mut neighbors: Vec<(usize, f64, Vector3<f64>)> = Vec::new();
        for &image in images.iter() {
            let disp = cell.matrix * image;
            let vquery = Vector3::from(p) + disp;
            let nps = tree.search(*vquery.as_ref(), cutoff);
            for &(index, distance) in nps.iter() {
                if index != current {
                    neighbors.push((index, distance, -1. * image))
                }
            }
        }
        kneighbors.insert(current, neighbors);
    }

    kneighbors
}
// neighbors.rs:1 ends here
