// [[file:~/Workspace/Programming/neighbors/neighbors.note::56b7353e-897b-4eb6-b8a7-89104f8b085f][56b7353e-897b-4eb6-b8a7-89104f8b085f]]
extern crate cgmath;
#[macro_use]
extern crate timeit;
extern crate octree;
#[macro_use]
extern crate approx;


use std::collections::HashMap;
use cgmath::{Vector3, Matrix};
use cgmath::prelude::*;
use octree::Octree;

pub type Point = [f64; 3];
pub type Points = Vec<Point>;

mod neighbors;
mod periodic;

#[derive(Clone, Debug, Default)]
pub struct Neighbor {
    /// Particle index in particle list
    pub index: usize,

    /// The periodic image that hosting the this neighbor particle
    pub image: Option<[f64; 3]>,

    /// The distance to the neighboring particle
    pub distance: f64,
}

impl Neighbor {
    pub fn new() -> Self {
        Neighbor::default()
    }
}


use periodic::UnitCell;

pub struct Neighborhood<'a> {
    tree: Octree<'a>,
    cell: Option<UnitCell>,
    kneighbors: HashMap<usize, Vec<(usize, f64, Vector3<f64>)>>,
}

impl<'a> Neighborhood<'a> {
    pub fn new(particles: &'a Points) -> Self {
        Neighborhood{
            tree: Octree::new(&particles),
            cell: None,
            kneighbors: HashMap::new(),
        }
    }

    /// Build the neighbor list
    pub fn build() {
        ;
    }

    /// Return neighbors of particle i.
    pub fn neighbors(&self, i: usize) -> Vec<Neighbor> {
        let mut ns = vec![];

        ns
    }
}
// 56b7353e-897b-4eb6-b8a7-89104f8b085f ends here
