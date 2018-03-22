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
    pub image: Option<Vector3<f64>>,

    /// The distance to the neighboring particle
    pub distance: f64,
}

impl Neighbor {
    pub fn new() -> Self {
        Neighbor::default()
    }
}

pub use periodic::UnitCell;

pub struct Neighborhood<'a> {
    pub cell: Option<UnitCell>,

    particles: &'a Points,
    tree: Octree<'a>,
    kneighbors: HashMap<usize, Vec<(usize, f64, Vector3<f64>)>>,
}

use neighbors::neighbors_for_periodic;

impl<'a> Neighborhood<'a> {
    pub fn new(particles: &'a Points) -> Self {
        Neighborhood{
            particles: particles,
            tree: Octree::new(&particles),
            cell: None,
            kneighbors: HashMap::new(),
        }
    }

    /// Build the neighbor list
    pub fn build(&mut self, cutoff: f64) -> Result<(), &'static str> {
       self.kneighbors = neighbors_for_periodic(self.particles, self.cell.unwrap(), cutoff);

        Ok(())
    }

    /// Return neighbors of particle i.
    pub fn neighbors(&self, i: usize) -> Result<Vec<Neighbor>, &'static str> {
        let kns = self.kneighbors.get(&i).ok_or("particle index out of bound")?;

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
// 56b7353e-897b-4eb6-b8a7-89104f8b085f ends here
