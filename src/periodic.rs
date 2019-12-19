// import

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*import][import:1]]
use gchemol_lattice::Lattice;

use crate::base::*;
// import:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*pub][pub:1]]
use vecfx::*;

impl Neighborhood {
    /// Set lattice for applying periodic boundary conditions
    pub fn set_lattice(&mut self, mat: [[f64; 3]; 3]) {
        let lat = Lattice::new(mat);
        self.lattice = Some(lat);
    }

    /// Search neighbors for periodic system.
    pub(crate) fn search_neighbors_periodic(
        &self,
        pt: Point,
        cutoff: f64,
        mut lattice: Lattice,
    ) -> impl Iterator<Item = Neighbor> + '_ {
        let tree = self.tree.as_ref().expect("octree not ready.");
        let images = lattice.relevant_images(cutoff);

        // to avoid octree building for each image we mirror the query points
        // and then mirror back
        let pt: Vector3f = pt.into();
        let pt_images = images.into_iter().map(move |image| {
            let v_disp = lattice.to_cart([image[0], image[1], image[2]]);
            let new_pt = pt + v_disp;
            (new_pt, -image)
        });

        // run queries over all relevant images
        pt_images.flat_map(move |(pt, image): (Vector3f, Vector3f)| {
            tree.search(pt.into(), cutoff)
                .into_iter()
                .map(move |(index, distance)| {
                    let (&node, _) = self.points.get_index(index).expect("invalid index");
                    Neighbor {
                        node,
                        distance,
                        image: Some(image),
                    }
                })
        })
    }
}
// pub:1 ends here
