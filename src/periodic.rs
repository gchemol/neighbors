// src

// [[file:~/Workspace/Programming/neighbors/neighbors.note::*src][src:1]]
use cgmath::prelude::*;
use cgmath::{Deg, Matrix3, Point3, Vector3};

/// An UnitCell defines how periodic boundary conditions are applied
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitCell {
    /// Unit cell matrix
    pub matrix: Matrix3<f64>,

    /// Inverse of the unit cell matrix
    inverse: Matrix3<f64>,

    lengths: [f64; 3],
    angles: [f64; 3],

    /// Volume of the unit cell.
    volume: f64,

    /// The perpendicular widths of the unit cell on each direction,
    /// i.e. the distance between opposite faces of the unit cell
    widths: [f64; 3],
}

impl UnitCell {
    pub fn new(tvs: [[f64; 3]; 3]) -> Self {
        let va = Vector3::from(tvs[0]);
        let vb = Vector3::from(tvs[1]);
        let vc = Vector3::from(tvs[2]);

        let volume = va.dot(vb.cross(vc));

        let wa = volume / (vb.magnitude() * vc.magnitude());
        let wb = volume / (vc.magnitude() * va.magnitude());
        let wc = volume / (va.magnitude() * vb.magnitude());

        UnitCell {
            matrix: Matrix3::from_cols(va, vb, vc),
            inverse: Matrix3::zero(),

            lengths: [0.0; 3],
            angles: [0.0; 3],
            volume: volume,
            widths: [wa, wb, wc],
        }
    }

    pub fn to_frac(&self, coordinates: Vec<Vector3<f64>>) -> Vec<Vector3<f64>> {
        let mut fractional = Vec::new();
        let inv = self.matrix.transpose().invert().unwrap();
        for v in coordinates {
            fractional.push(inv * v);
        }

        fractional
    }

    /// minimal images for neighborhood search
    pub fn relevant_images(&self, radius: f64) -> Vec<Vector3<f64>> {
        let ns = self.n_min_images(radius);
        let na = ns[0] as isize;
        let nb = ns[1] as isize;
        let nc = ns[2] as isize;

        let mut images = vec![];
        for i in (-na)..(na + 1) {
            for j in (-nb)..(nb + 1) {
                for k in (-nc)..(nc + 1) {
                    let v = Vector3::from([i as f64, j as f64, k as f64]);
                    images.push(v);
                }
            }
        }

        images
    }

    /// Return the minimal number of images for neighborhood search on each cell direction
    fn n_min_images(&self, radius: f64) -> [usize; 3] {
        let mut ns = [0; 3];

        for (i, &w) in self.widths.iter().enumerate() {
            let n = (radius / w).ceil();
            ns[i] = n as usize;
        }

        ns
    }
}

fn cart_to_frac(matrix: Matrix3<f64>, coordinates: Vec<Vector3<f64>>) -> Vec<Vector3<f64>> {
    let mut fractional = Vec::new();
    let inv = matrix.transpose().invert().unwrap();
    for v in coordinates {
        fractional.push(inv * v);
    }

    fractional
}
// src:1 ends here



// #+name: 9e56d65c-e674-41e3-96a3-aec630d7f374

// [[file:~/Workspace/Programming/neighbors/neighbors.note::9e56d65c-e674-41e3-96a3-aec630d7f374][9e56d65c-e674-41e3-96a3-aec630d7f374]]
#[test]
fn test_cell() {
    let cell = UnitCell::new(
        [[ 18.256,   0.   ,   0.   ],
         [  0.   ,  20.534,   0.   ],
         [  0.   ,   0.   ,  15.084]]
    );

    assert_eq!([1, 1, 1], cell.n_min_images(9.));
    assert_eq!([2, 1, 2], cell.n_min_images(19.));
    assert_eq!([2, 1, 2], cell.n_min_images(20.));
    assert_eq!([2, 2, 2], cell.n_min_images(20.6));

    let expected = [
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new(-1.0, -1.0,  0.0),
        Vector3::new(-1.0, -1.0,  1.0),
        Vector3::new(-1.0,  0.0, -1.0),
        Vector3::new(-1.0,  0.0,  0.0),
        Vector3::new(-1.0,  0.0,  1.0),
        Vector3::new(-1.0,  1.0, -1.0),
        Vector3::new(-1.0,  1.0,  0.0),
        Vector3::new(-1.0,  1.0,  1.0),
        Vector3::new( 0.0, -1.0, -1.0),
        Vector3::new( 0.0, -1.0,  0.0),
        Vector3::new( 0.0, -1.0,  1.0),
        Vector3::new( 0.0,  0.0, -1.0),
        Vector3::new( 0.0,  0.0,  0.0),
        Vector3::new( 0.0,  0.0,  1.0),
        Vector3::new( 0.0,  1.0, -1.0),
        Vector3::new( 0.0,  1.0,  0.0),
        Vector3::new( 0.0,  1.0,  1.0),
        Vector3::new( 1.0, -1.0, -1.0),
        Vector3::new( 1.0, -1.0,  0.0),
        Vector3::new( 1.0, -1.0,  1.0),
        Vector3::new( 1.0,  0.0, -1.0),
        Vector3::new( 1.0,  0.0,  0.0),
        Vector3::new( 1.0,  0.0,  1.0),
        Vector3::new( 1.0,  1.0, -1.0),
        Vector3::new( 1.0,  1.0,  0.0),
        Vector3::new( 1.0,  1.0,  1.0)];

    let images = cell.relevant_images(3.0);
    assert_eq!(expected.len(), images.len());
    assert_eq!(expected[1][2], images[1][2]);
}
// 9e56d65c-e674-41e3-96a3-aec630d7f374 ends here



// 最近邻镜像原子
// #+name: e51000e4-e9a4-41b0-87e7-e8a832bb1ea4

// [[file:~/Workspace/Programming/neighbors/neighbors.note::e51000e4-e9a4-41b0-87e7-e8a832bb1ea4][e51000e4-e9a4-41b0-87e7-e8a832bb1ea4]]
use std::f64;

fn get_nearest_image(
    cell: Matrix3<f64>,
    position1: Point3<f64>,
    position2: Point3<f64>) -> (Vector3<f64>, f64)
{
    // loop 27 possible point images
    let relevant_images = [-1, 0, 1];
    let mut distance = f64::MAX;
    let mut image = Vector3::from_value(0_f64);
    for x in relevant_images.iter() {
        for y in relevant_images.iter() {
            for z in relevant_images.iter() {
                let p = position2 + (*x as f64)*cell.x + (*y as f64)*cell.y + (*z as f64)*cell.z;
                let d = position1.distance(p);
                if d < distance {
                    distance = d;
                    image.x = *x as f64;
                    image.y = *y as f64;
                    image.z = *z as f64;
                }
            }
        }
    }

    (image, distance)
}

#[test]
fn test_get_nearest_image() {
    let mat1 = Matrix3::new(5.09, 0.00, 0.00,
                            0.00, 6.74, 0.00,
                            0.00, 0.00, 4.53);

    let p1  = Point3::new(0.18324000,   1.68500000,   3.85050000);
    let p13 = Point3::new(4.53010000,   1.68500000,   2.03850000);
    let p10 = Point3::new(0.94674000,   2.94538000,   1.48584000);
    let dp1_13 = 1.95847;
    let dp1_10 = 2.61920;

    let (image, d) = get_nearest_image(mat1, p1, p13);
    assert_relative_eq!(d, dp1_13, epsilon=1e-4);
    assert_relative_eq!(image.x, -1.0, epsilon=1e-4);
    assert_relative_eq!(image.y, 0.0, epsilon=1e-4);
    assert_relative_eq!(image.z, 0.0, epsilon=1e-4);

    let (image, d) = get_nearest_image(mat1, p1, p10);
    assert_relative_eq!(d, dp1_10, epsilon=1e-4);
}
// e51000e4-e9a4-41b0-87e7-e8a832bb1ea4 ends here



// #+name: a2204d33-0e41-4f99-9667-3f825b7039aa

// [[file:~/Workspace/Programming/neighbors/neighbors.note::a2204d33-0e41-4f99-9667-3f825b7039aa][a2204d33-0e41-4f99-9667-3f825b7039aa]]
fn cell_vectors_to_parameters(matrix: Matrix3<f64>) -> (f64, f64, f64, f64, f64, f64) {
    let a = matrix.x.magnitude();
    let b = matrix.y.magnitude();
    let c = matrix.z.magnitude();

    let alpha: Deg<_> = matrix.y.angle(matrix.z).into();
    let beta: Deg<_> = matrix.x.angle(matrix.z).into();
    let gamma: Deg<_> = matrix.x.angle(matrix.y).into();

    (a, b, c, alpha.0, beta.0, gamma.0)
}
// a2204d33-0e41-4f99-9667-3f825b7039aa ends here



// #+name: 46bbc67d-7915-4e1b-b582-75bd706bdaa6

// [[file:~/Workspace/Programming/neighbors/neighbors.note::46bbc67d-7915-4e1b-b582-75bd706bdaa6][46bbc67d-7915-4e1b-b582-75bd706bdaa6]]
#[test]
fn test_cell2() {
    // ovito/tests/files/LAMMPS/multi_sequence_1.dump
    let mat1 = Matrix3::new(5.09, 0.00, 0.00,
                            0.00, 6.74, 0.00,
                            0.00, 0.00, 4.53);

    let v1 = Vector3::new(2.1832, 1.6850, 3.8505);
    let v2 = Vector3::new(6.9068, 5.0550, 0.6795);
    let v3 = Vector3::new(4.3618, 5.0550, 1.5855);

    let fracs = cart_to_frac(mat1, vec![v1, v2, v3]);
    assert_relative_eq!(fracs[0].x, 0.4289, epsilon=1e-3);
    assert_relative_eq!(fracs[0].y, 0.2500, epsilon=1e-3);
    assert_relative_eq!(fracs[0].z, 0.8500, epsilon=1e-3);
    assert_relative_eq!(fracs[1].x, 1.3569, epsilon=1e-3);
    assert_relative_eq!(fracs[2].z, 0.3500, epsilon=1e-3);

    let mat2 = Matrix3::new(15.3643, 0.0, 0.0,
                            4.5807, 15.5026, 0.0,
                            0.0, 0.0, 17.4858);

    let (a, b, c, alpha, beta, gamma) = cell_vectors_to_parameters(mat2);
    assert_relative_eq!(a, 15.3643, epsilon=1e-4);
    assert_relative_eq!(b, 16.1652, epsilon=1e-4);
    assert_relative_eq!(c, 17.4858, epsilon=1e-4);

    assert_relative_eq!(alpha, 90.0, epsilon=1e-4);
    assert_relative_eq!(beta, 90.0, epsilon=1e-4);
    assert_relative_eq!(gamma, 73.5386, epsilon=1e-4);
}
// 46bbc67d-7915-4e1b-b582-75bd706bdaa6 ends here
