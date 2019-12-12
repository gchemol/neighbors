// import

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*import][import:1]]
#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
#[macro_use]
extern crate timeit;
// import:1 ends here

// mods

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*mods][mods:1]]
mod aperiodic;
mod periodic;
// mods:1 ends here

// base

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*base][base:1]]
mod base {
    use gchemol_lattice::Lattice;
    use indexmap::IndexMap;
    use octree::Octree;
    use vecfx::Vector3f;

    pub type Point = [f64; 3];

    /// Helper struct for neighbors search result.
    #[derive(Debug, Clone, Copy)]
    pub struct Neighbor {
        /// The node connected to the host point.
        pub node: usize,
        /// The distance to the host point.
        pub distance: f64,
        /// Scaled displacment vector relative to origin cell if PBC enabled.
        pub image: Option<Vector3f>,
    }

    /// Neighborhood is a neighboring nodes detector, for given cutoff distance.
    #[derive(Debug, Clone, Default)]
    pub struct Neighborhood {
        /// particle coordinates
        pub(crate) points: IndexMap<usize, Point>,

        /// Octree object
        pub(crate) tree: Option<Octree>,

        /// Periodic lattice.
        pub(crate) lattice: Option<Lattice>,
    }
}
// base:1 ends here

// api

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*api][api:1]]
mod api {
    use crate::base::*;
    use octree::Octree;

    impl Neighborhood {
        /// Constructs a neighborhood detector using the given `cutoff` distance.
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        /// Automatically build and update Neighborhood with contents of an
        /// iterator.
        pub fn update<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = (usize, Point)>,
        {
            self.points = iter.into_iter().collect();
            let points: Vec<_> = self.points.values().copied().collect();
            let mut tree = Octree::new(points);
            let bucket_size = 10;
            tree.build(bucket_size);
            self.tree = Some(tree);
        }

        /// Return a list of the nodes connected to the node n.
        ///
        /// Parameters
        /// ----------
        /// - n: the key of host node for searching neighbors
        /// - radius: cutoff radius distance
        pub fn neighbors(&self, n: &usize, radius: f64) -> Vec<Neighbor> {
            if let Some(lattice) = self.lattice {
                // FIXME: remove clone, remove mut
                let lattice = lattice.clone();
                self.neighbors_periodic(n, radius, lattice)
            } else {
                self.neighbors_aperiodic(n, radius)
            }
        }
    }
}
// api:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*pub][pub:1]]
pub use crate::api::*;
pub use crate::base::*;
// pub:1 ends here
