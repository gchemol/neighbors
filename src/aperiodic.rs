// imports

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*imports][imports:1]]
use crate::base::*;
// imports:1 ends here

// core

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*core][core:1]]
impl Neighborhood {
    pub(crate) fn neighbors_aperiodic(&self, n: &usize, radius: f64) -> Vec<Neighbor> {
        // the index of host node `n` in point list.
        let (n_index, _, pt) = self.points.get_full(n).expect("invalid key");

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
// core:1 ends here
