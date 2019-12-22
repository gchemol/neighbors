// import

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*import][import:1]]
use crate::base::*;
use gchemol_lattice::Image;
use gchemol_lattice::Lattice;
use octree::Octree;
use vecfx::*;
// import:1 ends here

// pub/algo1

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*pub/algo1][pub/algo1:1]]
impl Neighborhood {
    /// Search neighbors for periodic system.
    pub(crate) fn search_neighbors_periodic(
        &self,
        pt: Point,
        cutoff: f64,
        mut lattice: Lattice,
    ) -> impl Iterator<Item = Neighbor> + '_ {
        // the minimum supercell size ranges
        let cell_sizes: Vec<_> = lattice
            .widths()
            .iter()
            .map(|&w| {
                let rc = cutoff / w;
                let a = (-rc).floor() as isize;
                let b = (1.0 + rc).ceil() as isize;
                [a, b]
            })
            .collect();

        // to avoid octree building for each image we mirror the query points
        // and then mirror back
        let pt: Vector3f = pt.into();
        let pt_images = lattice
            .replicate_images(
                cell_sizes[0][0]..=cell_sizes[0][1],
                cell_sizes[1][0]..=cell_sizes[1][1],
                cell_sizes[2][0]..=cell_sizes[2][1],
            )
            .map(move |image| {
                let tv = image.translation_vector();
                let disp = lattice.to_cart(tv);
                let new_pt = pt + disp;
                (new_pt, -tv)
            });

        // run queries over all relevant images
        let tree = self.tree.as_ref().expect("octree not ready.");
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
// pub/algo1:1 ends here
