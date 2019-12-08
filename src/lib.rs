// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/neighbors/neighbors.note::*lib.rs][lib.rs:1]]
use cgmath::prelude::*;
use cgmath::{Matrix, Vector3};
use octree::Octree;
use std::collections::HashMap;

type Point = [f64; 3];
type Points = Vec<Point>;

mod neighbors;
mod periodic;

#[derive(Clone, Debug, Default)]
pub struct Neighbor {
    /// Particle index in particle list
    pub index: usize,

    /// The periodic image that hosting the this neighbor particle
    pub image: Option<Vector3<f64>>,

    /// The distance to the neighboring particle
    pub distance: f64,
}

impl Neighbor {
    pub fn new() -> Self {
        Neighbor::default()
    }
}

pub use crate::periodic::UnitCell;

pub struct Neighborhood<'a> {
    pub cell: Option<UnitCell>,

    particles: &'a Points,
    tree: Octree<'a>,
    kneighbors: HashMap<usize, Vec<(usize, f64, Vector3<f64>)>>,
}

use crate::neighbors::{neighbors_for_aperiodic, neighbors_for_periodic};

impl<'a> Neighborhood<'a> {
    /// Construct neighborhood structure from points in 3D space
    pub fn new(particles: &'a Points) -> Self {
        Neighborhood {
            particles: particles,
            tree: Octree::new(&particles),
            cell: None,
            kneighbors: HashMap::new(),
        }
    }

    /// Set unit cell, applying periodic boundary conditions
    pub fn set_cell(&mut self, cell: UnitCell) {
        self.cell = Some(cell);
    }

    /// Build the neighbor list
    pub fn build(&mut self, cutoff: f64) -> Result<(), &'static str> {
        if self.cell.is_some() {
            self.kneighbors = neighbors_for_periodic(self.particles, self.cell.unwrap(), cutoff);
        } else {
            self.kneighbors = neighbors_for_aperiodic(self.particles, cutoff);
        }

        Ok(())
    }

    /// Return neighbors of particle i.
    pub fn neighbors(&self, i: usize) -> Result<Vec<Neighbor>, &'static str> {
        let kns = self
            .kneighbors
            .get(&i)
            .ok_or("particle index out of bound")?;

        let mut ns = vec![];
        for &(index, distance, image) in kns.iter() {
            let mut neighbor = Neighbor::new();
            neighbor.index = index;
            neighbor.distance = distance;
            neighbor.image = Some(image);
            ns.push(neighbor);
        }

        Ok(ns)
    }
}
// lib.rs:1 ends here
