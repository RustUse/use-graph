#![forbid(unsafe_code)]
//! Primitive edge helpers.
//!
//! The crate keeps edge handling explicit with small structs for generic,
//! directed, and undirected edges.
//!
//! # Examples
//!
//! ```rust
//! use use_edge::{Edge, UndirectedEdge, dedupe_edges, has_self_loop};
//!
//! let deduped = dedupe_edges(&[Edge::new(0, 1), Edge::new(0, 1), Edge::new(1, 1)]);
//! let undirected = UndirectedEdge::new(2, 1);
//!
//! assert_eq!(deduped, vec![Edge::new(0, 1), Edge::new(1, 1)]);
//! assert_eq!(undirected.source(), 1);
//! assert!(has_self_loop(Edge::new(1, 1)));
//! ```

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirectedEdge {
    pub source: usize,
    pub target: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UndirectedEdge {
    pub a: usize,
    pub b: usize,
}

impl Edge {
    #[must_use]
    pub const fn new(source: usize, target: usize) -> Self {
        Self { source, target }
    }

    #[must_use]
    pub const fn source(&self) -> usize {
        self.source
    }

    #[must_use]
    pub const fn target(&self) -> usize {
        self.target
    }

    #[must_use]
    pub const fn contains(&self, node: usize) -> bool {
        self.source == node || self.target == node
    }

    #[must_use]
    pub const fn reversed(&self) -> Self {
        Self::new(self.target, self.source)
    }
}

impl DirectedEdge {
    #[must_use]
    pub const fn new(source: usize, target: usize) -> Self {
        Self { source, target }
    }

    #[must_use]
    pub const fn source(&self) -> usize {
        self.source
    }

    #[must_use]
    pub const fn target(&self) -> usize {
        self.target
    }

    #[must_use]
    pub const fn contains(&self, node: usize) -> bool {
        self.source == node || self.target == node
    }

    #[must_use]
    pub const fn reversed(&self) -> Self {
        Self::new(self.target, self.source)
    }
}

impl UndirectedEdge {
    #[must_use]
    pub fn new(a: usize, b: usize) -> Self {
        if a <= b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    #[must_use]
    pub const fn source(&self) -> usize {
        self.a
    }

    #[must_use]
    pub const fn target(&self) -> usize {
        self.b
    }

    #[must_use]
    pub const fn contains(&self, node: usize) -> bool {
        self.a == node || self.b == node
    }

    #[must_use]
    pub fn reversed(&self) -> Self {
        Self::new(self.b, self.a)
    }
}

#[must_use]
pub fn dedupe_edges(edges: &[Edge]) -> Vec<Edge> {
    let mut seen = HashSet::with_capacity(edges.len());
    let mut unique = Vec::with_capacity(edges.len());

    for &edge in edges {
        if seen.insert(edge) {
            unique.push(edge);
        }
    }

    unique
}

#[must_use]
pub const fn has_self_loop(edge: Edge) -> bool {
    edge.source == edge.target
}

#[cfg(test)]
mod tests {
    use super::{DirectedEdge, Edge, UndirectedEdge, dedupe_edges, has_self_loop};

    #[test]
    fn constructs_and_reverses_edges() {
        let edge = Edge::new(0, 1);
        let directed = DirectedEdge::new(1, 2);

        assert_eq!(edge.source(), 0);
        assert_eq!(edge.target(), 1);
        assert_eq!(edge.reversed(), Edge::new(1, 0));
        assert!(directed.contains(2));
        assert_eq!(directed.reversed(), DirectedEdge::new(2, 1));
    }

    #[test]
    fn normalizes_undirected_edges() {
        let first = UndirectedEdge::new(1, 2);
        let second = UndirectedEdge::new(2, 1);

        assert_eq!(first, second);
        assert_eq!(first.source(), 1);
        assert_eq!(first.target(), 2);
        assert_eq!(first.reversed(), first);
    }

    #[test]
    fn deduplicates_edges_and_detects_self_loops() {
        let edges = [Edge::new(0, 1), Edge::new(0, 1), Edge::new(1, 1)];

        assert_eq!(dedupe_edges(&edges), vec![Edge::new(0, 1), Edge::new(1, 1)]);
        assert!(has_self_loop(Edge::new(1, 1)));
        assert!(!has_self_loop(Edge::new(0, 1)));
    }
}
