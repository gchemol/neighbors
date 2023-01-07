// [[file:../neighbors.note::*import][import:1]]

// import:1 ends here

// [[file:../neighbors.note::*mods][mods:1]]
mod aperiodic;
mod periodic;
// mods:1 ends here

// [[file:../neighbors.note::*base][base:1]]
mod base {
    use lattice::Lattice;
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

// [[file:../neighbors.note::46a178d4][46a178d4]]
mod api {
    use crate::base::*;
    use lattice::Lattice;
    use octree::Octree;

    impl Neighborhood {
        /// Constructs a neighborhood detector using the given `cutoff` distance.
        pub fn new() -> Self {
            Self { ..Default::default() }
        }

        /// Automatically build and update Neighborhood with contents of an
        /// iterator.
        ///
        /// The position of a point is associated with a permanent key in type
        /// of `usize`.
        pub fn update<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = (usize, Point)>,
        {
            // update data points
            for (k, v) in iter {
                self.points.insert(k, v);
            }
            // FIXME: how to reduce octree building
            let points: Vec<_> = self.points.values().copied().collect();
            let mut tree = Octree::new(points);
            let bucket_size = 100;
            tree.build(bucket_size);
            self.tree = Some(tree);
        }

        #[cfg(feature = "adhoc")]
        /// Reset internal data.
        pub fn clear(&mut self) {
            self.points.clear();
            self.lattice = None;
            self.tree = None;
        }

        /// Return an iterator of the nodes connected to the node `n`.
        ///
        /// Parameters
        /// ----------
        /// * n: the key of host node for searching neighbors
        /// * radius: cutoff radius distance
        pub fn neighbors(&self, n: usize, radius: f64) -> impl Iterator<Item = Neighbor> + '_ {
            // the index of host node `n` in point list.
            let (_, _, &pt) = self.points.get_full(&n).expect("invalid key");

            // FIXME: think twice
            // excluding self from the list
            let epsilon = 1e-6;
            self.search(pt, radius).filter_map(move |m| {
                if m.node == n && m.distance < epsilon {
                    return None;
                }
                Some(m)
            })
        }

        /// Return the position of node `n`. Return None if there is no
        /// associated point with node `n`.
        pub fn get_position(&self, n: usize) -> Option<&Point> {
            self.points.get(&n)
        }

        /// Return neighbors of a particle `pt` within distance cutoff `radius`.
        pub fn search(&self, pt: Point, radius: f64) -> impl Iterator<Item = Neighbor> + '_ {
            // inspired by: https://stackoverflow.com/a/54728634
            let mut iter_periodic = None;
            let mut iter_aperiodic = None;
            match self.lattice {
                Some(lattice) => {
                    let iter = self.search_neighbors_periodic(pt, radius, lattice);
                    iter_periodic = Some(iter);
                }
                None => {
                    let iter = self.search_neighbors_aperiodic(pt, radius);
                    iter_aperiodic = Some(iter);
                }
            }
            iter_periodic
                .into_iter()
                .flatten()
                .chain(iter_aperiodic.into_iter().flatten())
        }

        /// Return current number of points.
        pub fn npoints(&self) -> usize {
            self.points.len()
        }

        // FIXME: generic `mat`
        /// Set lattice for applying periodic boundary conditions
        pub fn set_lattice(&mut self, mat: [[f64; 3]; 3]) {
            let lat = Lattice::new(mat);
            self.lattice = Some(lat);
        }
    }
}
// 46a178d4 ends here

// [[file:../neighbors.note::*pub][pub:1]]
pub use crate::api::*;
pub use crate::base::*;
// pub:1 ends here
