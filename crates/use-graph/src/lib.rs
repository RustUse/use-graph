#![forbid(unsafe_code)]
//! Thin facade for the `use-graph` workspace.
//!
//! The crate reexports the focused graph crates directly so consumers can opt
//! into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_graph::*;
//!
//! let adjacency = build_directed_adjacency(4, &[(0, 1), (1, 2), (0, 3)]).unwrap();
//! let path = shortest_path_unweighted(&adjacency, 0, 2).unwrap().unwrap();
//! let edge = WeightedEdge::new(0, 1, 1.5).unwrap();
//!
//! assert_eq!(path.nodes(), &[0, 1, 2]);
//! assert_eq!(path_weight(&[edge]).unwrap(), 1.5);
//! assert_eq!(max_degree(&adjacency), Some(2));
//! ```

pub use use_adjacency;
pub use use_adjacency::*;
pub use use_edge;
pub use use_edge::*;
pub use use_graph_metrics;
pub use use_graph_metrics::*;
pub use use_graph_path;
pub use use_graph_path::*;
pub use use_graph_traversal;
pub use use_graph_traversal::*;
pub use use_node;
pub use use_node::*;
pub use use_weighted_graph;
pub use use_weighted_graph::*;

#[cfg(test)]
mod tests {
    use super::{
        build_directed_adjacency, max_degree, node_ids, path_weight, shortest_path_unweighted,
        WeightedEdge,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let nodes = node_ids(4);
        assert_eq!(nodes.len(), 4);

        let adjacency = build_directed_adjacency(4, &[(0, 1), (1, 2), (0, 3)]).unwrap();
        let path = shortest_path_unweighted(&adjacency, 0, 2).unwrap().unwrap();
        assert_eq!(path.nodes(), &[0, 1, 2]);

        let edge = WeightedEdge::new(0, 1, 1.5).unwrap();
        assert_eq!(path_weight(&[edge]).unwrap(), 1.5);
        assert_eq!(max_degree(&adjacency), Some(2));
    }
}
