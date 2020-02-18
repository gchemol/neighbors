#[macro_use]
extern crate timeit;

use gchemol_neighbors::Neighborhood;
use rayon::prelude::*;

fn read_points_xyz(txt: &str) -> Vec<[f64; 3]> {
    let mut positions = Vec::new();
    for line in txt.lines().skip(2) {
        let attrs: Vec<_> = line.split_whitespace().collect();
        let (_symbol, position) = attrs.split_first().expect("empty line");
        assert_eq!(position.len(), 3, "{:?}", position);
        let p: Vec<f64> = position.iter().map(|x| x.parse().unwrap()).collect();
        positions.push([p[0], p[1], p[2]]);
    }

    positions
}

fn main() {
    // external xyz file containing 4704 particles
    let stream = include_str!("../data/25/87bf60-cbe7-44bc-a2f0-c369b329f968/CLO.xyz");
    let particles = read_points_xyz(stream);
    let a = 51.712;
    let cell = [[a, 0.0, 0.0], [0.0, a, 0.0], [0.0, 0.0, a]];

    let nloops = 10;
    let cutoff = 1.0;
    let x = timeit_loops!(10, {
        let mut nh = Neighborhood::new();
        nh.update(particles.iter().enumerate().map(|(i, &v)| (i, v)));
        nh.set_lattice(cell);
        let n = particles.len();
        (0..n).into_par_iter().take(n).for_each(|i| {
            let _nodes: Vec<_> = nh.neighbors(i, cutoff).map(|n| n.node).collect();
        })
    });

    println!("{} loops: {} ms", nloops, x * 1000.0);
}
