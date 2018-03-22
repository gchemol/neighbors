// [[file:~/Workspace/Programming/neighbors/neighbors.note::921fcb19-eb9a-4b00-9c8b-4f3af18a8f58][921fcb19-eb9a-4b00-9c8b-4f3af18a8f58]]
use std::ops::Mul;
use std::collections::HashMap;

use periodic::UnitCell;
use octree::Octree;

use cgmath::{Vector3, Matrix};
use cgmath::prelude::*;

/// search neighbors for periodic system
pub fn neighbors_for_periodic
    (
        positions: &Vec<[f64; 3]>,
        cell: UnitCell, cutoff: f64
    )
    -> HashMap<usize, Vec<(usize, f64, Vector3<f64>)>>
{
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
                    neighbors.push((index, distance, -1.*image))
                }
            }
        }
        kneighbors.insert(current, neighbors);
    }

    kneighbors
}
// 921fcb19-eb9a-4b00-9c8b-4f3af18a8f58 ends here
