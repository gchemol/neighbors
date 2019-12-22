// import

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*import][import:1]]
use crate::base::*;
use gchemol_lattice::Image;
use gchemol_lattice::Lattice;
use octree::Octree;
use vecfx::*;
// import:1 ends here

// impl

// [[file:~/Workspace/Programming/gchemol-rs/neighbors/neighbors.note::*impl][impl:1]]
impl Neighborhood {
    /// Search neighbors for periodic system.
    pub(crate) fn search_neighbors_periodic(
        &self,
        pt: Point,
        cutoff: f64,
        mut lattice: Lattice,
    ) -> Vec<Neighbor> {
        let [wa, wb, wc] = lattice.widths();
        let safe_ranges = [cutoff / wa, cutoff / wb, cutoff / wc];

        // 将坐标点wrapped到晶胞, 计算分数坐标
        let frac_coords: Vec<_> = self
            .points
            .values()
            .map(|&p| {
                let f = lattice.to_frac(p);
                lattice.wrap_frac(f)
            })
            .collect();

        // the minimum supercell size ranges
        let cell_sizes: Vec<_> = safe_ranges
            .iter()
            .map(|&rc| {
                let a = (-rc).floor() as isize;
                let b = (1.0 + rc).ceil() as isize;
                [a, b]
            })
            .collect();

        // 建立最小需要的超胞, 但仅保留相关的数据点
        let relevant_particle_images: Vec<_> = lattice
            // 以原晶胞为中心, 向三个轴向扩展, 建立超胞
            .replicate_images(
                cell_sizes[0][0]..=cell_sizes[0][1],
                cell_sizes[1][0]..=cell_sizes[1][1],
                cell_sizes[2][0]..=cell_sizes[2][1],
            )
            .flat_map(|image| {
                // 处理晶胞镜像中的数据点, 仅保留相关的
                let loc = image.location();
                // 计算镜像晶胞中的分数坐标, 根据分数坐标确定要保留的点.
                frac_coords.iter().enumerate().filter_map(
                    move |(
                        ipoint, // 中心晶胞粒子序号
                        &f,     // 中心晶胞粒子分数坐标
                    )| {
                        // loop over three lattice vectors
                        for (i, &j) in loc.iter().enumerate() {
                            let rc = safe_ranges[i];
                            if j as f64 + f[i] > 1.0 + rc {
                                return None;
                            }
                            if j as f64 + f[i] < -rc {
                                return None;
                            }
                        }
                        Some((ipoint, image))
                    },
                )
            })
            .collect();

        // 转换为直角坐标
        let cart_coords: Vec<[f64; 3]> = relevant_particle_images
            .iter()
            .map(|&(i, image)| {
                let f = frac_coords[i] + image.translation_vector();
                lattice.to_cart(f).into()
            })
            .collect();

        // 输出为cif, 方便检查结构
        // output_file(&lattice, &cart_coords);

        // update data points
        let mut tree = Octree::new(cart_coords);
        let bucket_size = 100;
        tree.build(bucket_size);

        tree.search(pt, cutoff)
            .map(move |(index, distance)| {
                let (index, image) = relevant_particle_images[index];
                // get user index
                let (&node, _) = self.points.get_index(index).expect("invalid index");
                Neighbor {
                    node,
                    distance,
                    image: Some(image.translation_vector()),
                }
            })
            .collect()
    }
}
// impl:1 ends here
