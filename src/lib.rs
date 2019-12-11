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
    use vecfx::Vector3f;
    use octree::Octree;

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
    pub struct Neighborhood<'a> {
        /// particle coordinates
        pub(crate) points: IndexMap<usize, Point>,

        /// Octree object
        pub(crate) tree: Option<Octree<'a>>,

        /// Periodic lattice.
        pub(crate) lattice: Option<Lattice>,
    }
}
// base:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*pub][pub:1]]
// pub use crate::aperiodic::*;
pub use crate::base::*;
// pub use crate::periodic::*;
// pub:1 ends here
