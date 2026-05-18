#![forbid(unsafe_code)]
//! Primitive node identifier helpers.
//!
//! The crate keeps node identifiers explicit and lightweight through a small
//! `NodeId` wrapper around `usize`.
//!
//! # Examples
//!
//! ```rust
//! use use_node::{NodeId, contains_node, node_ids, unique_nodes};
//!
//! let nodes = node_ids(3);
//! let unique = unique_nodes(&[NodeId::new(2), NodeId::new(1), NodeId::new(2)]);
//!
//! assert!(contains_node(&nodes, NodeId::new(1)));
//! assert_eq!(unique, vec![NodeId::new(2), NodeId::new(1)]);
//! ```

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(usize);

impl NodeId {
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(&self) -> usize {
        self.0
    }
}

#[must_use]
pub fn node_ids(count: usize) -> Vec<NodeId> {
    (0..count).map(NodeId::new).collect()
}

#[must_use]
pub fn contains_node(nodes: &[NodeId], node: NodeId) -> bool {
    nodes.contains(&node)
}

#[must_use]
pub fn unique_nodes(nodes: &[NodeId]) -> Vec<NodeId> {
    let mut seen = HashSet::with_capacity(nodes.len());
    let mut unique = Vec::with_capacity(nodes.len());

    for &node in nodes {
        if seen.insert(node) {
            unique.push(node);
        }
    }

    unique
}

#[cfg(test)]
mod tests {
    use super::{NodeId, contains_node, node_ids, unique_nodes};

    #[test]
    fn creates_node_ids() {
        let nodes = node_ids(4);

        assert_eq!(
            nodes,
            vec![
                NodeId::new(0),
                NodeId::new(1),
                NodeId::new(2),
                NodeId::new(3)
            ]
        );
        assert_eq!(NodeId::new(7).value(), 7);
    }

    #[test]
    fn checks_containment() {
        let nodes = [NodeId::new(1), NodeId::new(3)];

        assert!(contains_node(&nodes, NodeId::new(3)));
        assert!(!contains_node(&nodes, NodeId::new(2)));
    }

    #[test]
    fn deduplicates_nodes_while_preserving_first_occurrence_order() {
        let unique = unique_nodes(&[
            NodeId::new(2),
            NodeId::new(1),
            NodeId::new(2),
            NodeId::new(1),
            NodeId::new(3),
        ]);

        assert_eq!(unique, vec![NodeId::new(2), NodeId::new(1), NodeId::new(3)]);
    }
}
