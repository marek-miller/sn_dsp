use std::{
    alloc::{
        Allocator,
        Global,
    },
    fmt::Debug,
    mem,
    ops::{
        Index,
        IndexMut,
    },
    slice::{
        Iter,
        IterMut,
    },
    vec::IntoIter,
};

use crate::{
    frame::Frame,
    node::Node,
};

pub struct Bus<'a, T, A = Global>
where
    A: Allocator,
{
    nodes: Vec<Box<dyn Node<Frame = T> + 'a, A>, A>,
}

impl<'a, T, A> Bus<'a, T, A>
where
    A: Allocator,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            nodes: Vec::new_in(alloc),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn push(
        &mut self,
        node: Box<dyn Node<Frame = T> + 'a, A>,
    ) {
        self.nodes.push(node);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Node<Frame = T> + 'a, A>> {
        self.nodes.pop()
    }

    /// # Panics
    ///
    /// Panics if `index > len`
    pub fn insert(
        &mut self,
        index: usize,
        node: Box<dyn Node<Frame = T> + 'a, A>,
    ) {
        self.nodes.insert(index, node);
    }

    /// # Panics
    ///
    /// Panics if index is out of bounds.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Box<dyn Node<Frame = T> + 'a, A> {
        self.nodes.remove(index)
    }

    /// # Panics
    ///
    /// Panics if index is out of bounds.
    pub fn replace(
        &mut self,
        index: usize,
        node: Box<dyn Node<Frame = T> + 'a, A>,
    ) -> Box<dyn Node<Frame = T> + 'a, A> {
        mem::replace(&mut self.nodes[index], node)
    }

    pub fn node_push_in<N>(
        &mut self,
        node: N,
        alloc: A,
    ) -> &mut Self
    where
        T: Frame,
        N: Node<Frame = T> + 'a,
    {
        self.push(Box::new_in(node, alloc));
        self
    }

    /// # Panics
    ///
    /// Panics if `index > len`
    pub fn node_insert_in<N>(
        &mut self,
        index: usize,
        node: N,
        alloc: A,
    ) -> &mut Self
    where
        T: Frame,
        N: Node<Frame = T> + 'a,
    {
        self.insert(index, Box::new_in(node, alloc));
        self
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn as_slice(&self) -> &[Box<dyn Node<Frame = T> + 'a, A>] {
        &self.nodes
    }

    pub fn as_mut_slice(&mut self) -> &mut [Box<dyn Node<Frame = T> + 'a, A>] {
        &mut self.nodes
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn Node<Frame = T> + 'a, A>> {
        self.nodes.iter()
    }

    pub fn iter_mut(
        &mut self
    ) -> IterMut<'_, Box<dyn Node<Frame = T> + 'a, A>> {
        self.nodes.iter_mut()
    }
}

impl<'a, T, A> IntoIterator for Bus<'a, T, A>
where
    A: Allocator,
{
    type IntoIter = IntoIter<Box<dyn Node<Frame = T> + 'a, A>, A>;
    type Item = Box<dyn Node<Frame = T> + 'a, A>;

    fn into_iter(self) -> IntoIter<Box<dyn Node<Frame = T> + 'a, A>, A> {
        self.nodes.into_iter()
    }
}

impl<'b, 'a: 'b, T, A> IntoIterator for &'b Bus<'a, T, A>
where
    A: Allocator,
{
    type IntoIter = Iter<'b, Box<dyn Node<Frame = T> + 'a, A>>;
    type Item = &'b Box<dyn Node<Frame = T> + 'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'b, 'a: 'b, T, A> IntoIterator for &'b mut Bus<'a, T, A>
where
    A: Allocator,
{
    type IntoIter = IterMut<'b, Box<dyn Node<Frame = T> + 'a, A>>;
    type Item = &'b mut Box<dyn Node<Frame = T> + 'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T, A> Index<usize> for Bus<'a, T, A>
where
    A: Allocator,
{
    type Output = Box<dyn Node<Frame = T> + 'a, A>;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'a, T, A> IndexMut<usize> for Bus<'a, T, A>
where
    A: Allocator,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl<'a, T, A> Debug for Bus<'a, T, A>
where
    A: Allocator,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("Bus")
            .field("nodes", &format_args!("Vec<Box<dyn Node>>"))
            .finish()
    }
}

impl<'a, T, A> Node for Bus<'a, T, A>
where
    A: Allocator,
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for node in &mut self.nodes {
            node.proc(frames);
        }
    }
}

impl<'a, T> Bus<'a, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    /// Allocates memory on the heap
    pub fn node_push<N>(
        &mut self,
        node: N,
    ) -> &mut Self
    where
        T: Frame,
        N: Node<Frame = T> + 'a,
    {
        self.push(Box::new(node));
        self
    }

    /// Allocates memory on the heap
    ///
    /// # Panics
    ///
    /// Panics if `index > len`
    pub fn node_insert<N>(
        &mut self,
        index: usize,
        node: N,
    ) -> &mut Self
    where
        T: Frame,
        N: Node<Frame = T> + 'a,
    {
        self.insert(index, Box::new(node));
        self
    }
}

impl<'a, T> Default for Bus<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
