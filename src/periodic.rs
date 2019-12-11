// import

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*import][import:1]]
use gchemol_lattice::Lattice;

use crate::base::*;
// import:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*pub][pub:1]]
impl<'a> Neighborhood<'a> {
    /// Set lattice for applying periodic boundary conditions
    pub fn set_lattice(&mut self, mat: [[f64; 3]; 3]) {
        let lat = Lattice::new(mat);
        self.lattice = Some(lat);
    }

    /// Search neighbors for periodic system.
    pub(crate) fn neighbors_periodic(
        &self,
        n: &usize,
        cutoff: f64,
        mut lattice: Lattice,
    ) -> Vec<Neighbor> {
        // the index of host node `n` in point list.
        let (n_index, _, pt) = self.points.get_full(n).expect("invalid key");

        let tree = self.tree.as_ref().expect("octree not ready.");
        let images = lattice.relevant_images(cutoff);

        // to avoid octree building for each image we mirror the query points
        // and then mirror back
        let [x0, y0, z0] = pt;
        let pt_images = images.into_iter().map(|image| {
            let [dx, dy, dz] = lattice.to_cart([image[0], image[1], image[2]]);
            let new_pt = [x0 + dx, y0 + dy, z0 + dz];
            (new_pt, -image)
        });

        // run queries over all relevant images
        pt_images
            .flat_map(|(pt, image)| {
                tree.search(pt, cutoff)
                    .into_iter()
                    .filter_map(move |(index, distance)| {
                        if distance > 0.0 {
                            let (&node, _) = self.points.get_index(index).expect("invalid index");
                            let nbr = Neighbor {
                                node,
                                distance,
                                image: Some(image),
                            };
                            Some(nbr)
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}
// pub:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*test][test:1]]
#[test]
fn test_periodic_neighbors() {
    use vecfx::*;

    let particles = [[ 0.60421912,  4.2840792 ,  0.67433509],
                     [-0.69258171,  3.9731936 ,  3.49208748],
                     [ 0.32811792,  4.34729737,  6.48343793],
                     [ 4.88477572,  1.81537674,  6.26972558],
                     [ 6.14499816,  1.48505734,  3.37312786],
                     [ 5.12754047,  1.85762907,  0.43572421],
                     [ 2.09507387,  3.66872721,  0.39353504],
                     [ 0.5848138 ,  0.91854645,  0.28564143],
                     [ 0.33364169,  4.10698461,  2.26790994],
                     [-1.14582521,  2.41879964,  3.57784907],
                     [ 0.06571752,  4.4286596 ,  4.80486228],
                     [ 3.78132323,  0.96146537,  0.19503846],
                     [ 3.29078661,  1.21859679,  6.60654731],
                     [ 4.93953611,  3.49170736,  6.71444093],
                     [ 5.15070623,  1.63464631,  4.60290757],
                     [ 6.60903043,  4.89706872,  3.2209702 ],
                     [ 5.36681478,  1.95057166,  2.05143108],
                     [ 1.73241622,  3.38087446,  6.78291188]];
    
    let cell = [[ 8.60700000e+00,  0.00000000e+00,  0.00000000e+00],
                [ 8.64636107e-04,  4.95399992e+00,  0.00000000e+00],
                [-3.14318359e+00,  1.38078488e-02,  6.91625732e+00]];
    let mut nh = Neighborhood::new();
    nh.update(&particles);
    nh.set_lattice(cell);
    let cutoff = 1.8;
    let mut neighbors = nh.neighbors(&1, cutoff);
    // sort by node index
    neighbors.sort_by_key(|n| n.node);
    assert_eq!(4, neighbors.len());
    let nodes: Vec<_> = neighbors.iter().map(|n| n.node).collect();
    assert_eq!(nodes, vec![7, 8, 9, 14]);

    let images: Vec<_> = neighbors.iter().map(|n| n.image.unwrap()).collect();
    let expected = vec![
        [ 0.0, 0.0,  0.0],   // node 7
        [ 0.0, 1.0,  0.0],   // node 8
        [ 0.0, 0.0,  0.0],   // node 9
        [-1.0, 0.0, -1.0],   // node 14
    ];

    for (i, &image) in images.iter().enumerate() {
        let _image = Vector3f::from(expected[i]);
        assert_relative_eq!(image, _image);
    }
}
// test:1 ends here
