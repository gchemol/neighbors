// imports

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*imports][imports:1]]
use guts::prelude::*;
use indexmap::IndexMap;
use vecfx::Vector3f;
// imports:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*pub][pub:1]]
type Point = [f64; 3];

/// Helper struct for neighbors query result.
#[derive(Debug, Clone)]
pub struct Neighbor {
    /// The node connected to the host point.
    pub node: usize,
    /// The distance to the host point.
    pub distance: f64,
    /// Scaled displacment vector relative to origin cell if PBC enabled.
    pub image: Option<Vector3f>,
}

/// Neighborhood is a neighboring nodes detector, given a cutoff distance.
#[derive(Debug, Clone)]
pub struct Neighborhood {
    points: IndexMap<usize, Point>,
    tree: Option<Octree>,
}
// pub:1 ends here

// core

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*core][core:1]]
use octree::Octree;

impl Neighborhood {
    /// Constructs a neighborhood detector using the given `cutoff` distance.
    pub fn new() -> Self {
        Self {
            points: IndexMap::new(),
            tree: None,
        }
    }

    /// Automatically build and update Neighborhood with `points`.
    ///
    /// Parameters
    /// ----------
    /// - points: A list of 3D point to build Neighborhood for.
    pub fn update(&mut self, points: &[Point]) {
        for (i, &p) in points.iter().enumerate() {
            self.points.insert(i, p);
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

// tests

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*tests][tests:1]]
#[test]
fn test_neighbors() {
    let points = vec![
        [6.85877920e+00, 1.21643416e+01, 1.45130706e+01],
        [4.36500960e+00, 1.27783082e+01, 1.50779664e+01],
        [6.85877920e+00, 1.65462972e+01, 1.45123164e+01],
        [5.58085920e+00, 1.17783024e+01, 3.28077000e-01],
        [5.52791680e+00, 1.66448604e+01, 3.29585400e-01],
        [2.26922080e+00, 1.86366584e+01, 3.20007060e+00],
        [4.76299040e+00, 1.80226918e+01, 3.76496640e+00],
        [2.26922080e+00, 1.42547028e+01, 3.19931640e+00],
        [3.54714080e+00, 1.90226976e+01, 1.16410770e+01],
        [3.60008320e+00, 1.41561396e+01, 1.16425854e+01],
        [2.26922080e+00, 1.89734160e+00, 1.18839294e+01],
        [4.76299040e+00, 2.51130820e+00, 1.13190336e+01],
        [2.26922080e+00, 6.27929720e+00, 1.18846836e+01],
        [3.54714080e+00, 1.51130240e+00, 3.44292300e+00],
        [3.60008320e+00, 6.37786040e+00, 3.44141460e+00],
        [6.85877920e+00, 8.36965840e+00, 5.70929400e-01],
        [4.36500960e+00, 7.75569180e+00, 6.03360000e-03],
        [6.85877920e+00, 3.98770280e+00, 5.71683600e-01],
        [5.58085920e+00, 8.75569760e+00, 1.47559230e+01],
        [5.52791680e+00, 3.88913960e+00, 1.47544146e+01],
        [2.26922080e+00, 1.86366584e+01, 5.70929400e-01],
        [4.76299040e+00, 1.80226918e+01, 6.03360000e-03],
        [2.26922080e+00, 1.42547028e+01, 5.71683600e-01],
        [3.54714080e+00, 1.90226976e+01, 1.47559230e+01],
        [3.60008320e+00, 1.41561396e+01, 1.47544146e+01],
        [6.85877920e+00, 1.21643416e+01, 1.18839294e+01],
        [4.36500960e+00, 1.27783082e+01, 1.13190336e+01],
        [6.85877920e+00, 1.65462972e+01, 1.18846836e+01],
        [5.58085920e+00, 1.17783024e+01, 3.44292300e+00],
        [5.52791680e+00, 1.66448604e+01, 3.44141460e+00],
        [6.85877920e+00, 8.36965840e+00, 3.20007060e+00],
        [4.36500960e+00, 7.75569180e+00, 3.76496640e+00],
        [6.85877920e+00, 3.98770280e+00, 3.19931640e+00],
        [5.58085920e+00, 8.75569760e+00, 1.16410770e+01],
        [5.52791680e+00, 3.88913960e+00, 1.16425854e+01],
        [2.26922080e+00, 1.89734160e+00, 1.45130706e+01],
        [4.76299040e+00, 2.51130820e+00, 1.50779664e+01],
        [2.26922080e+00, 6.27929720e+00, 1.45123164e+01],
        [3.54714080e+00, 1.51130240e+00, 3.28077000e-01],
        [3.60008320e+00, 6.37786040e+00, 3.29585400e-01],
        [5.13176160e+00, 1.02670000e+01, 0.00000000e+00],
        [3.99623840e+00, 0.00000000e+00, 1.13130000e+01],
        [3.99623840e+00, 0.00000000e+00, 0.00000000e+00],
        [5.13176160e+00, 1.02670000e+01, 1.13130000e+01],
        [5.96606080e+00, 1.18994530e+01, 1.88550000e+00],
        [5.93867680e+00, 1.66099526e+01, 1.88550000e+00],
        [1.65399360e+00, 1.64456806e+01, 1.88550000e+00],
        [1.54810880e+00, 1.80514394e+01, 1.88550000e+00],
        [1.54810880e+00, 1.48399218e+01, 1.88550000e+00],
        [3.16193920e+00, 1.89015470e+01, 1.31985000e+01],
        [3.18932320e+00, 1.41910474e+01, 1.31985000e+01],
        [7.47400640e+00, 1.43553194e+01, 1.31985000e+01],
        [7.57989120e+00, 1.27495606e+01, 1.31985000e+01],
        [7.57989120e+00, 1.59610782e+01, 1.31985000e+01],
        [3.16193920e+00, 1.63245300e+00, 1.88550000e+00],
        [3.18932320e+00, 6.34295260e+00, 1.88550000e+00],
        [7.47400640e+00, 6.17868060e+00, 1.88550000e+00],
        [7.57989120e+00, 7.78443940e+00, 1.88550000e+00],
        [7.57989120e+00, 4.57292180e+00, 1.88550000e+00],
        [5.96606080e+00, 8.63454700e+00, 1.31985000e+01],
        [5.93867680e+00, 3.92404740e+00, 1.31985000e+01],
        [1.65399360e+00, 4.08831940e+00, 1.31985000e+01],
        [1.54810880e+00, 2.48256060e+00, 1.31985000e+01],
        [1.54810880e+00, 5.69407820e+00, 1.31985000e+01],
        [4.56400000e+00, 1.54005000e+01, 0.00000000e+00],
        [4.56400000e+00, 1.54005000e+01, 1.13130000e+01],
        [4.56400000e+00, 5.13350000e+00, 0.00000000e+00],
        [4.56400000e+00, 5.13350000e+00, 1.13130000e+01],
        [0.00000000e+00, 1.84908670e+01, 1.88550000e+00],
        [0.00000000e+00, 1.44004942e+01, 1.88550000e+00],
        [0.00000000e+00, 2.04313300e+00, 1.31985000e+01],
        [0.00000000e+00, 6.13350580e+00, 1.31985000e+01],
        [1.59867792e+01, 1.89734160e+00, 1.45130706e+01],
        [1.34930096e+01, 2.51130820e+00, 1.50779664e+01],
        [1.59867792e+01, 6.27929720e+00, 1.45123164e+01],
        [1.47088592e+01, 1.51130240e+00, 3.28077000e-01],
        [1.46559168e+01, 6.37786040e+00, 3.29585400e-01],
        [1.13972208e+01, 8.36965840e+00, 3.20007060e+00],
        [1.38909904e+01, 7.75569180e+00, 3.76496640e+00],
        [1.13972208e+01, 3.98770280e+00, 3.19931640e+00],
        [1.26751408e+01, 8.75569760e+00, 1.16410770e+01],
        [1.27280832e+01, 3.88913960e+00, 1.16425854e+01],
        [1.13972208e+01, 1.21643416e+01, 1.18839294e+01],
        [1.38909904e+01, 1.27783082e+01, 1.13190336e+01],
        [1.13972208e+01, 1.65462972e+01, 1.18846836e+01],
        [1.26751408e+01, 1.17783024e+01, 3.44292300e+00],
        [1.27280832e+01, 1.66448604e+01, 3.44141460e+00],
        [1.59867792e+01, 1.86366584e+01, 5.70929400e-01],
        [1.34930096e+01, 1.80226918e+01, 6.03360000e-03],
        [1.59867792e+01, 1.42547028e+01, 5.71683600e-01],
        [1.47088592e+01, 1.90226976e+01, 1.47559230e+01],
        [1.46559168e+01, 1.41561396e+01, 1.47544146e+01],
        [1.13972208e+01, 8.36965840e+00, 5.70929400e-01],
        [1.38909904e+01, 7.75569180e+00, 6.03360000e-03],
        [1.13972208e+01, 3.98770280e+00, 5.71683600e-01],
        [1.26751408e+01, 8.75569760e+00, 1.47559230e+01],
        [1.27280832e+01, 3.88913960e+00, 1.47544146e+01],
        [1.59867792e+01, 1.89734160e+00, 1.18839294e+01],
        [1.34930096e+01, 2.51130820e+00, 1.13190336e+01],
        [1.59867792e+01, 6.27929720e+00, 1.18846836e+01],
        [1.47088592e+01, 1.51130240e+00, 3.44292300e+00],
        [1.46559168e+01, 6.37786040e+00, 3.44141460e+00],
        [1.59867792e+01, 1.86366584e+01, 3.20007060e+00],
        [1.34930096e+01, 1.80226918e+01, 3.76496640e+00],
        [1.59867792e+01, 1.42547028e+01, 3.19931640e+00],
        [1.47088592e+01, 1.90226976e+01, 1.16410770e+01],
        [1.46559168e+01, 1.41561396e+01, 1.16425854e+01],
        [1.13972208e+01, 1.21643416e+01, 1.45130706e+01],
        [1.38909904e+01, 1.27783082e+01, 1.50779664e+01],
        [1.13972208e+01, 1.65462972e+01, 1.45123164e+01],
        [1.26751408e+01, 1.17783024e+01, 3.28077000e-01],
        [1.27280832e+01, 1.66448604e+01, 3.29585400e-01],
        [1.42597616e+01, 0.00000000e+00, 0.00000000e+00],
        [1.31242384e+01, 1.02670000e+01, 1.13130000e+01],
        [1.31242384e+01, 1.02670000e+01, 0.00000000e+00],
        [1.42597616e+01, 0.00000000e+00, 1.13130000e+01],
        [1.50940608e+01, 1.63245300e+00, 1.88550000e+00],
        [1.50666768e+01, 6.34295260e+00, 1.88550000e+00],
        [1.07819936e+01, 6.17868060e+00, 1.88550000e+00],
        [1.06761088e+01, 7.78443940e+00, 1.88550000e+00],
        [1.06761088e+01, 4.57292180e+00, 1.88550000e+00],
        [1.22899392e+01, 8.63454700e+00, 1.31985000e+01],
        [1.23173232e+01, 3.92404740e+00, 1.31985000e+01],
        [1.66020064e+01, 4.08831940e+00, 1.31985000e+01],
        [1.67078912e+01, 2.48256060e+00, 1.31985000e+01],
        [1.67078912e+01, 5.69407820e+00, 1.31985000e+01],
        [1.22899392e+01, 1.18994530e+01, 1.88550000e+00],
        [1.23173232e+01, 1.66099526e+01, 1.88550000e+00],
        [1.66020064e+01, 1.64456806e+01, 1.88550000e+00],
        [1.67078912e+01, 1.80514394e+01, 1.88550000e+00],
        [1.67078912e+01, 1.48399218e+01, 1.88550000e+00],
        [1.50940608e+01, 1.89015470e+01, 1.31985000e+01],
        [1.50666768e+01, 1.41910474e+01, 1.31985000e+01],
        [1.07819936e+01, 1.43553194e+01, 1.31985000e+01],
        [1.06761088e+01, 1.27495606e+01, 1.31985000e+01],
        [1.06761088e+01, 1.59610782e+01, 1.31985000e+01],
        [1.36920000e+01, 5.13350000e+00, 1.13130000e+01],
        [1.36920000e+01, 1.54005000e+01, 1.13130000e+01],
        [1.36920000e+01, 5.13350000e+00, 0.00000000e+00],
        [1.36920000e+01, 1.54005000e+01, 0.00000000e+00],
        [9.12800000e+00, 8.22386700e+00, 1.88550000e+00],
        [9.12800000e+00, 4.13349420e+00, 1.88550000e+00],
        [9.12800000e+00, 1.23101330e+01, 1.31985000e+01],
        [9.12800000e+00, 1.64005058e+01, 1.31985000e+01],
        [6.85877920e+00, 1.21643416e+01, 6.97107060e+00],
        [4.36500960e+00, 1.27783082e+01, 7.53596640e+00],
        [6.85877920e+00, 1.65462972e+01, 6.97031640e+00],
        [5.58085920e+00, 1.17783024e+01, 7.87007700e+00],
        [5.52791680e+00, 1.66448604e+01, 7.87158540e+00],
        [2.26922080e+00, 1.86366584e+01, 1.07420706e+01],
        [4.76299040e+00, 1.80226918e+01, 1.13069664e+01],
        [2.26922080e+00, 1.42547028e+01, 1.07413164e+01],
        [3.54714080e+00, 1.90226976e+01, 4.09907700e+00],
        [3.60008320e+00, 1.41561396e+01, 4.10058540e+00],
        [2.26922080e+00, 1.89734160e+00, 4.34192940e+00],
        [4.76299040e+00, 2.51130820e+00, 3.77703360e+00],
        [2.26922080e+00, 6.27929720e+00, 4.34268360e+00],
        [3.54714080e+00, 1.51130240e+00, 1.09849230e+01],
        [3.60008320e+00, 6.37786040e+00, 1.09834146e+01],
        [6.85877920e+00, 8.36965840e+00, 8.11292940e+00],
        [4.36500960e+00, 7.75569180e+00, 7.54803360e+00],
        [6.85877920e+00, 3.98770280e+00, 8.11368360e+00],
        [5.58085920e+00, 8.75569760e+00, 7.21392300e+00],
        [5.52791680e+00, 3.88913960e+00, 7.21241460e+00],
        [2.26922080e+00, 1.86366584e+01, 8.11292940e+00],
        [4.76299040e+00, 1.80226918e+01, 7.54803360e+00],
        [2.26922080e+00, 1.42547028e+01, 8.11368360e+00],
        [3.54714080e+00, 1.90226976e+01, 7.21392300e+00],
        [3.60008320e+00, 1.41561396e+01, 7.21241460e+00],
        [6.85877920e+00, 1.21643416e+01, 4.34192940e+00],
        [4.36500960e+00, 1.27783082e+01, 3.77703360e+00],
        [6.85877920e+00, 1.65462972e+01, 4.34268360e+00],
        [5.58085920e+00, 1.17783024e+01, 1.09849230e+01],
        [5.52791680e+00, 1.66448604e+01, 1.09834146e+01],
        [6.85877920e+00, 8.36965840e+00, 1.07420706e+01],
        [4.36500960e+00, 7.75569180e+00, 1.13069664e+01],
        [6.85877920e+00, 3.98770280e+00, 1.07413164e+01],
        [5.58085920e+00, 8.75569760e+00, 4.09907700e+00],
        [5.52791680e+00, 3.88913960e+00, 4.10058540e+00],
        [2.26922080e+00, 1.89734160e+00, 6.97107060e+00],
        [4.76299040e+00, 2.51130820e+00, 7.53596640e+00],
        [2.26922080e+00, 6.27929720e+00, 6.97031640e+00],
        [3.54714080e+00, 1.51130240e+00, 7.87007700e+00],
        [3.60008320e+00, 6.37786040e+00, 7.87158540e+00],
        [5.13176160e+00, 1.02670000e+01, 7.54200000e+00],
        [3.99623840e+00, 0.00000000e+00, 3.77100000e+00],
        [3.99623840e+00, 0.00000000e+00, 7.54200000e+00],
        [5.13176160e+00, 1.02670000e+01, 3.77100000e+00],
        [5.96606080e+00, 1.18994530e+01, 9.42750000e+00],
        [5.93867680e+00, 1.66099526e+01, 9.42750000e+00],
        [1.65399360e+00, 1.64456806e+01, 9.42750000e+00],
        [1.54810880e+00, 1.80514394e+01, 9.42750000e+00],
        [1.54810880e+00, 1.48399218e+01, 9.42750000e+00],
        [3.16193920e+00, 1.89015470e+01, 5.65650000e+00],
        [3.18932320e+00, 1.41910474e+01, 5.65650000e+00],
        [7.47400640e+00, 1.43553194e+01, 5.65650000e+00],
        [7.57989120e+00, 1.27495606e+01, 5.65650000e+00],
        [7.57989120e+00, 1.59610782e+01, 5.65650000e+00],
        [3.16193920e+00, 1.63245300e+00, 9.42750000e+00],
        [3.18932320e+00, 6.34295260e+00, 9.42750000e+00],
        [7.47400640e+00, 6.17868060e+00, 9.42750000e+00],
        [7.57989120e+00, 7.78443940e+00, 9.42750000e+00],
        [7.57989120e+00, 4.57292180e+00, 9.42750000e+00],
        [5.96606080e+00, 8.63454700e+00, 5.65650000e+00],
        [5.93867680e+00, 3.92404740e+00, 5.65650000e+00],
        [1.65399360e+00, 4.08831940e+00, 5.65650000e+00],
        [1.54810880e+00, 2.48256060e+00, 5.65650000e+00],
        [1.54810880e+00, 5.69407820e+00, 5.65650000e+00],
        [4.56400000e+00, 1.54005000e+01, 7.54200000e+00],
        [4.56400000e+00, 1.54005000e+01, 3.77100000e+00],
        [4.56400000e+00, 5.13350000e+00, 7.54200000e+00],
        [4.56400000e+00, 5.13350000e+00, 3.77100000e+00],
        [0.00000000e+00, 1.84908670e+01, 9.42750000e+00],
        [0.00000000e+00, 1.44004942e+01, 9.42750000e+00],
        [0.00000000e+00, 2.04313300e+00, 5.65650000e+00],
        [0.00000000e+00, 6.13350580e+00, 5.65650000e+00],
        [1.59867792e+01, 1.89734160e+00, 6.97107060e+00],
        [1.34930096e+01, 2.51130820e+00, 7.53596640e+00],
        [1.59867792e+01, 6.27929720e+00, 6.97031640e+00],
        [1.47088592e+01, 1.51130240e+00, 7.87007700e+00],
        [1.46559168e+01, 6.37786040e+00, 7.87158540e+00],
        [1.13972208e+01, 8.36965840e+00, 1.07420706e+01],
        [1.38909904e+01, 7.75569180e+00, 1.13069664e+01],
        [1.13972208e+01, 3.98770280e+00, 1.07413164e+01],
        [1.26751408e+01, 8.75569760e+00, 4.09907700e+00],
        [1.27280832e+01, 3.88913960e+00, 4.10058540e+00],
        [1.13972208e+01, 1.21643416e+01, 4.34192940e+00],
        [1.38909904e+01, 1.27783082e+01, 3.77703360e+00],
        [1.13972208e+01, 1.65462972e+01, 4.34268360e+00],
        [1.26751408e+01, 1.17783024e+01, 1.09849230e+01],
        [1.27280832e+01, 1.66448604e+01, 1.09834146e+01],
        [1.59867792e+01, 1.86366584e+01, 8.11292940e+00],
        [1.34930096e+01, 1.80226918e+01, 7.54803360e+00],
        [1.59867792e+01, 1.42547028e+01, 8.11368360e+00],
        [1.47088592e+01, 1.90226976e+01, 7.21392300e+00],
        [1.46559168e+01, 1.41561396e+01, 7.21241460e+00],
        [1.13972208e+01, 8.36965840e+00, 8.11292940e+00],
        [1.38909904e+01, 7.75569180e+00, 7.54803360e+00],
        [1.13972208e+01, 3.98770280e+00, 8.11368360e+00],
        [1.26751408e+01, 8.75569760e+00, 7.21392300e+00],
        [1.27280832e+01, 3.88913960e+00, 7.21241460e+00],
        [1.59867792e+01, 1.89734160e+00, 4.34192940e+00],
        [1.34930096e+01, 2.51130820e+00, 3.77703360e+00],
        [1.59867792e+01, 6.27929720e+00, 4.34268360e+00],
        [1.47088592e+01, 1.51130240e+00, 1.09849230e+01],
        [1.46559168e+01, 6.37786040e+00, 1.09834146e+01],
        [1.59867792e+01, 1.86366584e+01, 1.07420706e+01],
        [1.34930096e+01, 1.80226918e+01, 1.13069664e+01],
        [1.59867792e+01, 1.42547028e+01, 1.07413164e+01],
        [1.47088592e+01, 1.90226976e+01, 4.09907700e+00],
        [1.46559168e+01, 1.41561396e+01, 4.10058540e+00],
        [1.13972208e+01, 1.21643416e+01, 6.97107060e+00],
        [1.38909904e+01, 1.27783082e+01, 7.53596640e+00],
        [1.13972208e+01, 1.65462972e+01, 6.97031640e+00],
        [1.26751408e+01, 1.17783024e+01, 7.87007700e+00],
        [1.27280832e+01, 1.66448604e+01, 7.87158540e+00],
        [1.42597616e+01, 0.00000000e+00, 7.54200000e+00],
        [1.31242384e+01, 1.02670000e+01, 3.77100000e+00],
        [1.31242384e+01, 1.02670000e+01, 7.54200000e+00],
        [1.42597616e+01, 0.00000000e+00, 3.77100000e+00],
        [1.50940608e+01, 1.63245300e+00, 9.42750000e+00],
        [1.50666768e+01, 6.34295260e+00, 9.42750000e+00],
        [1.07819936e+01, 6.17868060e+00, 9.42750000e+00],
        [1.06761088e+01, 7.78443940e+00, 9.42750000e+00],
        [1.06761088e+01, 4.57292180e+00, 9.42750000e+00],
        [1.22899392e+01, 8.63454700e+00, 5.65650000e+00],
        [1.23173232e+01, 3.92404740e+00, 5.65650000e+00],
        [1.66020064e+01, 4.08831940e+00, 5.65650000e+00],
        [1.67078912e+01, 2.48256060e+00, 5.65650000e+00],
        [1.67078912e+01, 5.69407820e+00, 5.65650000e+00],
        [1.22899392e+01, 1.18994530e+01, 9.42750000e+00],
        [1.23173232e+01, 1.66099526e+01, 9.42750000e+00],
        [1.66020064e+01, 1.64456806e+01, 9.42750000e+00],
        [1.67078912e+01, 1.80514394e+01, 9.42750000e+00],
        [1.67078912e+01, 1.48399218e+01, 9.42750000e+00],
        [1.50940608e+01, 1.89015470e+01, 5.65650000e+00],
        [1.50666768e+01, 1.41910474e+01, 5.65650000e+00],
        [1.07819936e+01, 1.43553194e+01, 5.65650000e+00],
        [1.06761088e+01, 1.27495606e+01, 5.65650000e+00],
        [1.06761088e+01, 1.59610782e+01, 5.65650000e+00],
        [1.36920000e+01, 5.13350000e+00, 3.77100000e+00],
        [1.36920000e+01, 1.54005000e+01, 3.77100000e+00],
        [1.36920000e+01, 5.13350000e+00, 7.54200000e+00],
        [1.36920000e+01, 1.54005000e+01, 7.54200000e+00],
        [9.12800000e+00, 8.22386700e+00, 9.42750000e+00],
        [9.12800000e+00, 4.13349420e+00, 9.42750000e+00],
        [9.12800000e+00, 1.23101330e+01, 5.65650000e+00],
        [9.12800000e+00, 1.64005058e+01, 5.65650000e+00],
    ];

    let mut nh = Neighborhood::new();
    nh.update(&points);
    let nns = nh.neighbors(&0, 1.7);
    dbg!(nns);

    // let ns = nh.neighbors(0).unwrap();
    // assert_eq!(1, ns.len());
    // assert_eq!(52, ns[0].index);
}
// tests:1 ends here