// imports

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*imports][imports:1]]
use crate::base::*;
// imports:1 ends here

// core

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*core][core:1]]
impl Neighborhood {
    pub(crate) fn search_neighbors_aperiodic(
        &self,
        pt: Point,
        radius: f64,
    ) -> impl Iterator<Item = Neighbor> + '_ {
        self.tree
            .as_ref()
            .expect("octree not ready")
            .search(pt, radius)
            .into_iter()
            .map(move |(index, distance)| {
                let (&node, _) = self.points.get_index(index).expect("invalid index");
                Neighbor {
                    node,
                    distance,
                    image: None,
                }
            })
    }
}
// core:1 ends here
