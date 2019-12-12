// periodic.rs
// :PROPERTIES:
// :header-args: :tangle tests/periodic.rs
// :END:

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*periodic.rs][periodic.rs:1]]
use neighbors::Neighborhood;

#[test]
fn test_periodic_neighbors() {
    use approx::*;
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
    nh.update(particles.iter().enumerate().map(|(i, &v)| (i + 1, v)));
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
        [0.0, 0.0, 0.0],   // node 7
        [0.0, 1.0, 0.0],   // node 8
        [0.0, 0.0, 0.0],   // node 9
        [-1.0, 0.0, -1.0], // node 14
    ];

    for (i, &image) in images.iter().enumerate() {
        let _image = Vector3f::from(expected[i]);
        assert_relative_eq!(image, _image);
    }
}
// periodic.rs:1 ends here
