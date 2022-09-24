use anyhow;
use std::ops::{Index, IndexMut};
use std::path::Path;

use image::io::Reader as ImageReader;
use image::Luma;

pub fn convert(
    input_path: &Path,
    tile_width: u32,
    tile_height: u32,
    output_path: &Path,
) -> anyhow::Result<()> {
    let mut img = ImageReader::open(input_path)?.decode()?.into_luma8();
    let mut quantizer = ImplicitTree::<ColorNode>::init();
    for Luma([c]) in img.pixels().cloned() {
        quantizer.count_pixel(c);
    }
    quantizer.reduce(4);
    let (palette, table) = quantizer.palette_and_remapping_table();
    for (i, c) in palette.into_iter().enumerate() {
        println!("{i:<3}: #{c:02x}{c:02x}{c:02x}");
    }
    Ok(())
}

/// See https://opendatastructures.org/ods-cpp/10_1_Implicit_Binary_Tree.html
struct ImplicitTree<Node> {
    nodes: Vec<Node>,
}

impl<Node> ImplicitTree<Node> {
    fn left(index: usize) -> usize {
        (index << 1) + 1
    }

    fn right(index: usize) -> usize {
        (index << 1) + 2
    }

    fn parent(index: usize) -> usize {
        (index - 1) >> 1
    }

    fn depth(index: usize) -> usize {
        ((usize::BITS - 1) - (index + 1).leading_zeros()) as usize
    }

    fn first_at_depth(depth: usize) -> usize {
        (1 << (depth)) - 1
    }

    fn last_at_depth(depth: usize) -> usize {
        (1 << (depth + 1)) - 2
    }

    /// `f` should return `false` if it doesn't want to see nodes under the one it was passed.
    fn dfs<F>(&self, mut f: F)
    where
        F: FnMut(usize) -> bool,
    {
        self.dfs_at(&mut f, 0);
    }

    fn dfs_at<F>(&self, f: &mut F, index: usize)
    where
        F: FnMut(usize) -> bool,
    {
        if Self::first_at_depth(Self::depth(index)) >= self.nodes.len() {
            return;
        }
        if f(index) {
            self.dfs_at(f, Self::left(index));
            self.dfs_at(f, Self::right(index));
        }
    }
}

impl<Node: Default> ImplicitTree<Node> {
    fn new(depth: usize) -> ImplicitTree<Node> {
        let mut nodes = vec![];
        nodes.resize_with(2usize.pow((depth + 1) as u32) - 1, Default::default);
        ImplicitTree { nodes }
    }
}

impl<Node> Index<usize> for ImplicitTree<Node> {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<Node> IndexMut<usize> for ImplicitTree<Node> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

#[derive(Default, Debug)]
struct ColorNode {
    count: usize,
    leaf: bool,
}

impl ImplicitTree<ColorNode> {
    fn init() -> Self {
        Self::new(8)
    }

    fn count_pixel(&mut self, color: u8) {
        let index = Self::first_at_depth(8) + color as usize;
        self[index].leaf = true;
        self[index].count += 1;
    }

    fn num_colors(&self) -> usize {
        self.num_colors_under(0)
    }

    fn num_colors_under(&self, index: usize) -> usize {
        if Self::depth(index) > 8 {
            0
        } else if self[index].leaf {
            1
        } else {
            self.num_colors_under(Self::left(index)) + self.num_colors_under(Self::right(index))
        }
    }

    /// Does this node have two children with pixels?
    fn is_reducible(&self, index: usize) -> bool {
        if self[index].leaf {
            return false;
        }
        self[Self::left(index)].leaf && self[Self::right(index)].leaf
    }

    fn reduce_at(&mut self, index: usize) {
        self[index].leaf = self[Self::left(index)].leaf || self[Self::right(index)].leaf;
        self[index].count = self[Self::left(index)].count + self[Self::right(index)].count;
    }

    /// Look at the search level and merge the two colors with the smallest pixel counts.
    /// If there aren't any reducible colors there, try levels closer to the root.
    fn reduce_once(&mut self, reduce_state: &mut ReduceState) {
        loop {
            let mut smallest_index: Option<usize> = None;
            let mut smallest_count: Option<usize> = None;
            for index in
                Self::first_at_depth(reduce_state.depth)..=Self::last_at_depth(reduce_state.depth)
            {
                if !self.is_reducible(index) {
                    continue;
                }
                let count = self[Self::left(index)].count + self[Self::right(index)].count;
                if smallest_index.is_none() || count < smallest_count.unwrap() {
                    smallest_index = Some(index);
                    smallest_count = Some(count);
                }
            }
            if let Some(index) = smallest_index {
                self.reduce_at(index);
                reduce_state.num_colors -= 1;
                return;
            }

            // If we can't reduce any colors, we can promote nodes with one child to this level, making them candidates for the next round of merges.
            for index in
                Self::first_at_depth(reduce_state.depth)..=Self::last_at_depth(reduce_state.depth)
            {
                if self[index].leaf {
                    // Product of a previous reduction. Skip it.
                    continue;
                }
                // `.reduce_at` can also promote.
                // Reducing a node with one child results in a promotion.
                // Reducing a node with no children results in another empty node.
                self.reduce_at(index);
            }

            // Then we go one level closer to the root and repeat.
            if reduce_state.depth == 0 {
                panic!("Can't reduce to zero colors");
            }
            reduce_state.depth -= 1;
        }
    }

    /// Merge colors down to a target number of colors.
    /// See https://dl.acm.org/doi/10.5555/90767.90833,
    /// "A simple method for color quantization: octree quantization"
    /// This version is adapted to grayscale,
    /// uses an implicit binary tree instead of an explicit octree,
    /// and breaks ties by merging the colors with the smallest numbers of pixels.
    /// Okay, it's more "inspired by" than actually based on the original.
    fn reduce(&mut self, num_colors: usize) {
        let mut reduce_state = ReduceState {
            depth: 7,
            num_colors: self.num_colors(),
        };
        while reduce_state.num_colors > num_colors {
            self.reduce_once(&mut reduce_state);
        }
    }

    /// Note: colors not in the input image will be mapped to 0 in the remapping table. Do not use the remapping table on images other than the input image.
    fn palette_and_remapping_table(&mut self) -> (Vec<u8>, [u8; 256]) {
        let mut palette = vec![0u8; 0];
        let mut table = [0u8; 256];

        let visit_colors = |index| {
            if !self[index].leaf {
                return true;
            }

            // We found a color that survived reduction.
            // Calculate the weighted average of all original colors that were merged into this color.
            // That will be the new color.

            let mut leftmost_deepest_child = index;
            while Self::depth(leftmost_deepest_child) < 8 {
                leftmost_deepest_child = Self::left(leftmost_deepest_child);
            }
            let color_min = leftmost_deepest_child - Self::first_at_depth(8);

            let mut rightmost_deepest_child = index;
            while Self::depth(rightmost_deepest_child) < 8 {
                rightmost_deepest_child = Self::right(rightmost_deepest_child);
            }
            let color_max = rightmost_deepest_child - Self::first_at_depth(8);

            let mut count_total = 0usize;
            let mut color_total = 0f64;
            for child_index in leftmost_deepest_child..=rightmost_deepest_child {
                if !self[child_index].leaf {
                    // No pixels with this color.
                    continue;
                }
                count_total += self[child_index].count;
                let color = child_index - Self::first_at_depth(8);
                color_total += self[child_index].count as f64 * color as f64;
            }
            let replacement_color = (color_total / count_total as f64) as u8;
            palette.push(replacement_color);

            // In the reduction table, use the replacement color for all colors in the range merged to produce it.
            for color in color_min..=color_max {
                table[color] = replacement_color;
            }

            // Don't visit nodes lower than this one.
            false
        };
        self.dfs(visit_colors);

        (palette, table)
    }
}

#[derive(Debug)]
struct ReduceState {
    depth: usize,
    num_colors: usize,
}

mod test {
    use super::*;

    /// Should pass thru the existing colors.
    #[test]
    fn test_1234_passthru() {
        let mut quantizer = ImplicitTree::<ColorNode>::init();
        assert_eq!(0, quantizer.num_colors());
        quantizer.count_pixel(1);
        quantizer.count_pixel(2);
        quantizer.count_pixel(3);
        quantizer.count_pixel(4);
        assert_eq!(4, quantizer.num_colors());
        // Don't call .reduce()
        let palette = quantizer.palette_and_remapping_table().0;
        assert_eq!(vec![1, 2, 3, 4], palette);
    }

    /// Should also pass thru the existing colors.
    #[test]
    fn test_1234_reduce_4() {
        let mut quantizer = ImplicitTree::<ColorNode>::init();
        assert_eq!(0, quantizer.num_colors());
        quantizer.count_pixel(1);
        quantizer.count_pixel(2);
        quantizer.count_pixel(3);
        quantizer.count_pixel(4);
        assert_eq!(4, quantizer.num_colors());
        quantizer.reduce(4);
        assert_eq!(4, quantizer.num_colors());
        let palette = quantizer.palette_and_remapping_table().0;
        assert_eq!(vec![1, 2, 3, 4], palette);
    }

    /// Should yield 2 colors.
    #[test]
    fn test_1234_reduce_2() {
        let mut quantizer = ImplicitTree::<ColorNode>::init();
        assert_eq!(0, quantizer.num_colors());
        quantizer.count_pixel(1);
        quantizer.count_pixel(2);
        quantizer.count_pixel(3);
        quantizer.count_pixel(4);
        assert_eq!(4, quantizer.num_colors());
        quantizer.reduce(2);
        assert_eq!(2, quantizer.num_colors());
        let palette = quantizer.palette_and_remapping_table().0;
        assert_eq!(vec![2, 4], palette);
    }

    /// Should yield 2 colors but one should be close to the most common input color.
    #[test]
    fn test_11111234_reduce_2() {
        let mut quantizer = ImplicitTree::<ColorNode>::init();
        assert_eq!(0, quantizer.num_colors());
        quantizer.count_pixel(1);
        quantizer.count_pixel(1);
        quantizer.count_pixel(1);
        quantizer.count_pixel(1);
        quantizer.count_pixel(1);
        quantizer.count_pixel(2);
        quantizer.count_pixel(3);
        quantizer.count_pixel(4);
        assert_eq!(4, quantizer.num_colors());
        quantizer.reduce(2);
        assert_eq!(2, quantizer.num_colors());
        let palette = quantizer.palette_and_remapping_table().0;
        assert_eq!(vec![1, 3], palette);
    }
}
