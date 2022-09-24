use std::ops::{Index, IndexMut};

/// Implicit binary tree.
/// See https://opendatastructures.org/ods-cpp/10_1_Implicit_Binary_Tree.html
pub struct ImplicitTree<Node> {
    nodes: Vec<Node>,
}

impl<Node> ImplicitTree<Node> {
    pub fn left(index: usize) -> usize {
        (index << 1) + 1
    }

    pub fn right(index: usize) -> usize {
        (index << 1) + 2
    }

    pub fn parent(index: usize) -> usize {
        (index - 1) >> 1
    }

    pub fn depth(index: usize) -> usize {
        ((usize::BITS - 1) - (index + 1).leading_zeros()) as usize
    }

    pub fn first_at_depth(depth: usize) -> usize {
        (1 << (depth)) - 1
    }

    pub fn last_at_depth(depth: usize) -> usize {
        (1 << (depth + 1)) - 2
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Preorder traversal (parents before children).
    /// `f` should return `false` if it doesn't want to see nodes deeper than the one it was passed.
    pub fn dfs<F>(&self, f: &mut F, index: usize)
    where
        F: FnMut(usize) -> bool,
    {
        if index >= self.len() {
            return;
        }
        if f(index) {
            self.dfs(f, Self::left(index));
            self.dfs(f, Self::right(index));
        }
    }
}

impl<Node: Default> ImplicitTree<Node> {
    pub fn new_full(depth: usize) -> ImplicitTree<Node> {
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
