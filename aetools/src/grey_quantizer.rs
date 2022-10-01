use crate::implicit_tree::ImplicitTree;

/// Quantize colors in an 8-bit greyscale image.
///
/// See https://dl.acm.org/doi/10.5555/90767.90833,
/// "A simple method for color quantization: octree quantization"
/// This version is adapted to greyscale,
/// uses an implicit binary tree instead of an explicit octree,
/// and breaks ties by merging the colors with the smallest numbers of pixels.
/// Okay, it's more "inspired by" than actually based on the original.
///
/// For a survey of other algorithms, see https://arxiv.org/abs/1101.0395.
/// For a practical implementation of another algorithm, see https://pngquant.org/#algorithm.
///
/// This algorithm doesn't necessarily produce an optimal palette in terms of minimizing
/// [MSE](https://en.wikipedia.org/wiki/Mean_squared_error),
/// [PSNR](https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio),
/// or [SSIM](https://en.wikipedia.org/wiki/Structural_similarity,
/// but it might be good enough.
/// Given the smallish color space, better but more expensive algorithms may be practical.
pub struct GreyQuantizer(ImplicitTree<ColorNode>);

impl GreyQuantizer {
    pub fn new() -> Self {
        Self(ImplicitTree::new())
    }

    /// Record one pixel with this color.
    pub fn count_pixel(&mut self, color: u8) {
        self.0.count_pixel(color)
    }

    /// Get total number of pixels counted with a given color.
    pub fn total_pixels_with_color(&self, color: u8) -> usize {
        self.0.total_pixels_with_color(color)
    }

    /// Get the current number of colors.
    pub fn num_colors(&self) -> usize {
        self.0.num_colors()
    }

    /// Merge colors down to a target number of colors.
    pub fn reduce(&mut self, num_colors: usize) {
        self.0.reduce(num_colors)
    }

    /// Palette will be in sorted order.
    /// Table maps input colors to palette indexes,
    /// and may not be valid for colors not in the input image.
    pub fn palette_and_mapping_table(&mut self) -> (Vec<u8>, [Option<u8>; 256]) {
        self.0.palette_and_mapping_table()
    }
}

#[derive(Default)]
struct ColorNode {
    /// Number of pixels with this color (okay, grey level).
    count: usize,
    /// True if this represents actual pixels, false if this is an interior node.
    /// Note that we need to keep the original colors around to generate a mapping table,
    /// so "leaves" will often have other leaves under them.
    /// The real table boundary is the end of the node vector.
    leaf: bool,
}

struct ReduceState {
    /// Level of the tree being considered for reducible nodes.
    depth: usize,
    /// Current number of colors.
    num_colors: usize,
}

impl ImplicitTree<ColorNode> {
    fn new() -> Self {
        Self::new_full(8)
    }

    fn count_pixel(&mut self, color: u8) {
        let index = Self::first_at_depth(8) + color as usize;
        self[index].leaf = true;
        self[index].count += 1;
    }

    fn total_pixels_with_color(&self, color: u8) -> usize {
        let index = Self::first_at_depth(8) + color as usize;
        self[index].count
    }

    fn num_colors(&self) -> usize {
        self.num_colors_under(0)
    }

    fn num_colors_under(&self, index: usize) -> usize {
        if index >= self.len() {
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

    fn reduce(&mut self, num_colors: usize) {
        let mut reduce_state = ReduceState {
            depth: 7,
            num_colors: self.num_colors(),
        };
        while reduce_state.num_colors > num_colors {
            self.reduce_once(&mut reduce_state);
        }
    }

    fn palette_and_mapping_table(&mut self) -> (Vec<u8>, [Option<u8>; 256]) {
        let mut palette = vec![0u8; 0];
        let mut table = [None; 256];

        let mut visit_colors = |index| {
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
            let replacement_color = (color_total / count_total as f64).round() as u8;
            palette.push(replacement_color);

            // In the reduction table, use the replacement color for all colors in the range merged to produce it.
            let palette_index = Some((palette.len() - 1) as u8);
            for color in color_min..=color_max {
                table[color] = palette_index;
            }

            // Don't visit nodes lower than this one.
            false
        };
        self.dfs(&mut visit_colors, 0);

        (palette, table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Should pass thru the existing colors.
    #[test]
    fn test_1234_passthru() {
        let mut quantizer = GreyQuantizer::new();
        assert_eq!(0, quantizer.num_colors());
        for c in 1..=4 {
            quantizer.count_pixel(c);
        }
        assert_eq!(4, quantizer.num_colors());
        let (palette, table) = quantizer.palette_and_mapping_table();
        assert_eq!(vec![1, 2, 3, 4], palette);
        for c in 1..=4 {
            assert!(palette.contains(&table[c]));
        }
    }

    /// Should also pass thru the existing colors.
    #[test]
    fn test_1234_reduce_4() {
        let mut quantizer = GreyQuantizer::new();
        for c in 1..=4 {
            quantizer.count_pixel(c);
        }
        quantizer.reduce(4);
        assert_eq!(4, quantizer.num_colors());
        let (palette, table) = quantizer.palette_and_mapping_table();
        assert_eq!(vec![1, 2, 3, 4], palette);
        for c in 1..=4 {
            assert!(palette.contains(&table[c]));
        }
    }

    /// Should yield 2 colors.
    #[test]
    fn test_1234_reduce_2() {
        let mut quantizer = GreyQuantizer::new();
        for c in 1..=4 {
            quantizer.count_pixel(c);
        }
        quantizer.reduce(2);
        assert_eq!(2, quantizer.num_colors());
        let (palette, table) = quantizer.palette_and_mapping_table();
        assert_eq!(vec![2, 4], palette);
        for c in 1..=4 {
            assert!(palette.contains(&table[c]));
        }
    }

    /// Should yield 3 colors.
    #[test]
    fn test_1234_reduce_3() {
        let mut quantizer = GreyQuantizer::new();
        for c in 1..=4 {
            quantizer.count_pixel(c);
        }
        quantizer.reduce(3);
        assert_eq!(3, quantizer.num_colors());
        let (palette, table) = quantizer.palette_and_mapping_table();
        assert_eq!(vec![1, 3, 4], palette);
        for c in 1..=4 {
            assert!(palette.contains(&table[c]));
        }
    }

    /// Should yield 2 colors but one should be close to the most common input color.
    /// Resulting palette is legal but not optimal.
    #[test]
    fn test_11111234_reduce_2() {
        let mut quantizer = GreyQuantizer::new();
        for c in 1..=4 {
            quantizer.count_pixel(1);
            quantizer.count_pixel(c);
        }
        assert_eq!(4, quantizer.num_colors());
        quantizer.reduce(2);
        assert_eq!(2, quantizer.num_colors());
        let (palette, table) = quantizer.palette_and_mapping_table();
        assert_eq!(vec![1, 4], palette);
        for c in 1..=4 {
            assert!(palette.contains(&table[c]));
        }
    }
}
