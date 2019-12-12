// update.rs
// :PROPERTIES:
// :header-args: :tangle tests/update.rs
// :END:

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*update.rs][update.rs:1]]
use neighbors::Neighborhood;

#[test]
fn test_neighbors() {
    let positions = vec![[-1.5365e+00,  2.4770e-01,  0.0000e+00],
                         [-4.1670e-01,  2.4770e-01,  0.0000e+00],
                         [-1.8828e+00,  1.3126e+00,  0.0000e+00],
                         [-2.0532e+00, -4.6390e-01,  1.2468e+00],
                         [-1.6724e+00,  6.0500e-02,  2.1630e+00],
                         [-2.0535e+00, -4.6410e-01, -1.2466e+00],
                         [-1.6729e+00,  6.0100e-02, -2.1629e+00],
                         [-1.5637e+00, -1.9090e+00, -1.2451e+00],
                         [-4.4420e-01, -1.9302e+00, -1.2577e+00],
                         [-1.9302e+00, -2.4348e+00, -2.1634e+00],
                         [-1.5634e+00, -1.9089e+00,  1.2454e+00],
                         [-1.9296e+00, -2.4345e+00,  2.1638e+00],
                         [-4.4390e-01, -1.9300e+00,  1.2576e+00],
                         [-2.0803e+00, -2.6231e+00,  3.0000e-04],
                         [-1.7195e+00, -3.6858e+00,  3.0000e-04],
                         [-3.6059e+00, -2.6024e+00,  4.0000e-04],
                         [-3.9927e+00, -3.1350e+00, -9.0540e-01],
                         [-3.9924e+00, -3.1349e+00,  9.0650e-01],
                         [-3.5788e+00, -4.4570e-01,  1.2456e+00],
                         [-3.9456e+00,  6.1220e-01,  1.2580e+00],
                         [-3.9650e+00, -9.5680e-01,  2.1640e+00],
                         [-3.5791e+00, -4.4580e-01, -1.2449e+00],
                         [-3.9458e+00,  6.1220e-01, -1.2572e+00],
                         [-3.9656e+00, -9.5680e-01, -2.1632e+00],
                         [-4.0980e+00, -1.1582e+00,  4.0000e-04],
                         [-5.2201e+00, -1.1442e+00,  6.0000e-04]];

    let mut nh = Neighborhood::new();
    // update internal data points. set point labels counting from 1.
    nh.update(positions.into_iter().enumerate().map(|(i, v)| (i + 1, v)));
    assert_eq!(nh.npoints(), 26);

    let n = nh.neighbors(8, 1.5).count();
    assert_eq!(n, 2);
    let n = nh.neighbors(8, 1.6).count();
    assert_eq!(n, 4);

    // move point 9 to a new location: distance(8--9) = 2.05
    let p9 = [0.4858028, -1.9478115, -1.2681672];
    let new_positions = vec![(9, p9)];
    nh.update(new_positions);
    let n = nh.neighbors(8, 1.5).count();
    assert_eq!(n, 1);
    let n = nh.neighbors(8, 1.6).count();
    assert_eq!(n, 3);

    // search neighboring points near a location
    let n = nh.search(p9, 2.0).count();
    assert_eq!(n, 1);     // point 9
    let n = nh.search(p9, 2.2).count();
    assert_eq!(n, 2);     // point 9 + 8
}
// update.rs:1 ends here
