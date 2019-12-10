// imports

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*imports][imports:1]]
use crate::base::*;
// imports:1 ends here

// core

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*core][core:1]]
use octree::Octree;

impl Neighborhood {
    /// Constructs a neighborhood detector using the given `cutoff` distance.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Automatically build and update Neighborhood with `points`.
    ///
    /// Parameters
    /// ----------
    /// - points: A list of 3D point to build Neighborhood for.
    pub fn update(&mut self, points: &[Point]) {
        for (i, &p) in points.iter().enumerate() {
            // counts from 1
            self.points.insert(i + 1, p);
        }
        let points: Vec<_> = self.points.values().cloned().collect();
        let mut tree = Octree::new(&points);
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
        // the index of host node `n` in point list.
        let (n_index, _, pt) = self.points.get_full(n).expect("invalid key");

        if let Some(lattice) = self.lattice {
            // FIXME: remove clone, remove mut
            let lattice = lattice.clone();
            self.neighbors_periodic(n, radius, lattice)
        } else {
            self.tree
                .as_ref()
                .expect("octree not ready")
                .search(*pt, radius)
                .into_iter()
                .filter_map(|(index, distance)| {
                    // excluding this node `n` from neighbor list.
                    if index == n_index {
                        None
                    } else {
                        let (&node, _) = self.points.get_index(index).expect("invalid index");
                        let neighbor = Neighbor {
                            node,
                            distance,
                            image: None,
                        };

                        Some(neighbor)
                    }
                })
                .collect()
        }
    }
}
// core:1 ends here
