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

// triclinic

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*triclinic][triclinic:1]]
#[test]
fn test_triclinic() {
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
    nh.update(particles.iter().enumerate().map(|(i, &v)| (i, v)));
    nh.set_lattice(cell);

    let cutoff = 1.0;
    let nodes: Vec<_> = nh.neighbors(0, cutoff).map(|n| n.node).collect();
    assert!(nodes.is_empty());

    let cutoff = 3.0;
    let mut nodes: Vec<_> = nh.neighbors(0, cutoff).map(|n| n.node).collect();
    nodes.sort();
    assert_eq!(vec![6, 7, 8, 13], nodes);

    let cutoff = 5.0;
    let mut nodes: Vec<_> = nh.neighbors(0, cutoff).map(|n| n.node).collect();
    nodes.sort();
    assert_eq!(
        vec![
            0, 0, 1, 1, 2, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 7, 8, 9, 9, 9, 10, 10, 11, 11, 12, 12, 13,
            13, 14, 14, 15, 15, 16, 16, 17, 17
        ],
        nodes
    );

    let cutoff = 4.0;
    let neighbors = vec![
        vec![1, 2, 3, 3, 6, 7, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vec![0, 2, 4, 4, 6, 7, 8, 9, 9, 10, 13, 14, 14, 15, 16],
        vec![0, 1, 5, 5, 6, 7, 9, 10, 11, 11, 12, 16, 17],
        vec![0, 0, 4, 5, 6, 7, 8, 9, 12, 13, 13, 14, 15, 16, 17],
        vec![1, 1, 3, 5, 7, 8, 8, 9, 10, 11, 12, 14, 15, 15, 16],
        vec![2, 2, 3, 4, 6, 9, 10, 11, 12, 13, 15, 16, 17, 17],
        vec![0, 1, 2, 3, 5, 7, 7, 8, 9, 10, 11, 11, 13, 17],
        vec![0, 0, 1, 2, 3, 4, 6, 6, 8, 8, 10, 11, 12, 13, 13, 14],
        vec![0, 1, 3, 4, 4, 6, 7, 7, 9, 9, 10, 13, 15],
        vec![0, 1, 1, 2, 3, 4, 5, 6, 8, 8, 10, 10, 13, 14, 15, 15, 16],
        vec![0, 1, 2, 4, 5, 6, 7, 8, 9, 9, 11, 15, 17],
        vec![0, 2, 2, 4, 5, 6, 6, 7, 10, 12, 16, 17, 17],
        vec![0, 2, 3, 4, 5, 7, 11, 13, 13, 14, 15, 16, 17, 17],
        vec![0, 1, 3, 3, 5, 6, 7, 7, 8, 9, 12, 12, 14, 14, 15, 16, 17],
        vec![0, 1, 1, 3, 4, 7, 9, 12, 13, 13, 15, 15, 16],
        vec![0, 1, 3, 4, 4, 5, 8, 9, 9, 10, 12, 13, 14, 14, 16, 16],
        vec![1, 2, 3, 4, 5, 9, 11, 12, 13, 14, 15, 15, 17],
        vec![2, 3, 5, 5, 6, 10, 11, 11, 12, 12, 13, 16],
    ];
    for (i, expected) in neighbors.into_iter().enumerate() {
        let mut nodes: Vec<_> = nh.neighbors(i, cutoff).map(|n| n.node).collect();
        nodes.sort();
        assert_eq!(expected, nodes);
    }
}
// triclinic:1 ends here

// orthorhombic

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*orthorhombic][orthorhombic:1]]
#[test]
fn test_orthorhombic() {
    let particles = vec![
        [ 0.        ,  0.        , 23.78162099],
        [ 1.24405   ,  1.24405   , 25.47342157],
        [ 0.        ,  0.        , 20.24425178],
        [ 1.24405   ,  1.24405   , 21.99516685],
        [ 0.        ,  0.        , 16.71356813],
        [ 1.24405   ,  1.24405   , 18.47363187],
        [ 0.        ,  0.        , 13.19203315],
        [ 1.24405   ,  1.24405   , 14.94294822],
        [ 0.        ,  0.        ,  9.71377843],
        [ 1.24405   ,  1.24405   , 11.40557901]
    ];
    
    let cell = [[2.4881, 0.0, 0.0], [0.0, 2.4881, 0.0], [0.0, 0.0, 35.1872]];
    let mut nh = Neighborhood::new();
    nh.update(particles.iter().enumerate().map(|(i, &v)| (i, v)));
    nh.set_lattice(cell);

    let cutoff = 1.0;
    let nodes: Vec<_> = nh.neighbors(0, cutoff).map(|n| n.node).collect();
    assert!(nodes.is_empty());

    let cutoff = 3.0;
    let mut nodes: Vec<_> = nh.neighbors(0, cutoff).map(|n| n.node).collect();
    nodes.sort();
    assert_eq!(vec![0, 0, 0, 0, 1, 1, 1, 1, 3, 3, 3, 3], nodes);

    let cutoff = 5.0;
    let mut nodes: Vec<_> = nh.neighbors(0, cutoff).map(|n| n.node).collect();
    nodes.sort();
    assert_eq!(
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3
        ],
        nodes
    );

    let cutoff = 4.0;
    let neighbors = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 3, 3, 3, 3],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![0, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 5, 5, 5, 5],
        vec![0, 0, 0, 0, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 5],
        vec![2, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 7, 7, 7, 7],
        vec![2, 2, 2, 2, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 7],
        vec![4, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 8, 9, 9, 9, 9],
        vec![4, 4, 4, 4, 5, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 9],
        vec![6, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9],
        vec![6, 6, 6, 6, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9],
    ];
    for (i, expected) in neighbors.into_iter().enumerate() {
        let mut nodes: Vec<_> = nh.neighbors(i, cutoff).map(|n| n.node).collect();
        nodes.sort();
        assert_eq!(expected, nodes);
    }
}
// orthorhombic:1 ends here
