#![forbid(unsafe_code)]
//! Primitive graph metrics.
//!
//! The crate provides small helpers for counting nodes and edges, inspecting
//! degrees, and computing simple directed or undirected densities.
//!
//! # Examples
//!
//! ```rust
//! use use_graph_metrics::{average_degree, density_directed, edge_count_directed, max_degree};
//!
//! let adjacency = vec![vec![1, 2], vec![2], vec![]];
//!
//! assert_eq!(edge_count_directed(&adjacency), 3);
//! assert_eq!(max_degree(&adjacency), Some(2));
//! assert_eq!(average_degree(&adjacency), Some(1.0));
//! assert_eq!(density_directed(&adjacency), Some(0.5));
//! ```

#[must_use]
pub fn node_count(adjacency: &[Vec<usize>]) -> usize {
    adjacency.len()
}

#[must_use]
pub fn edge_count_directed(adjacency: &[Vec<usize>]) -> usize {
    adjacency.iter().map(Vec::len).sum()
}

#[must_use]
pub fn edge_count_undirected(adjacency: &[Vec<usize>]) -> usize {
    let mut self_loops = 0;
    let mut non_self_edges = 0;

    for (node, neighbors) in adjacency.iter().enumerate() {
        for &neighbor in neighbors {
            if neighbor == node {
                self_loops += 1;
            } else {
                non_self_edges += 1;
            }
        }
    }

    self_loops + non_self_edges / 2
}

#[must_use]
pub fn degrees(adjacency: &[Vec<usize>]) -> Vec<usize> {
    adjacency.iter().map(Vec::len).collect()
}

#[must_use]
pub fn max_degree(adjacency: &[Vec<usize>]) -> Option<usize> {
    adjacency.iter().map(Vec::len).max()
}

#[must_use]
pub fn min_degree(adjacency: &[Vec<usize>]) -> Option<usize> {
    adjacency.iter().map(Vec::len).min()
}

#[must_use]
pub fn average_degree(adjacency: &[Vec<usize>]) -> Option<f64> {
    if adjacency.is_empty() {
        None
    } else {
        Some(edge_count_directed(adjacency) as f64 / adjacency.len() as f64)
    }
}

#[must_use]
pub fn density_directed(adjacency: &[Vec<usize>]) -> Option<f64> {
    match adjacency.len() {
        0 => None,
        1 => Some(0.0),
        node_count => {
            Some(edge_count_directed(adjacency) as f64 / (node_count * (node_count - 1)) as f64)
        }
    }
}

#[must_use]
pub fn density_undirected(adjacency: &[Vec<usize>]) -> Option<f64> {
    match adjacency.len() {
        0 => None,
        1 => Some(0.0),
        node_count => Some(
            (2 * edge_count_undirected(adjacency)) as f64 / (node_count * (node_count - 1)) as f64,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        average_degree, degrees, density_directed, density_undirected, edge_count_directed,
        edge_count_undirected, max_degree, min_degree, node_count,
    };

    #[test]
    fn computes_directed_graph_metrics() {
        let adjacency = vec![vec![1, 2], vec![2], vec![]];

        assert_eq!(node_count(&adjacency), 3);
        assert_eq!(edge_count_directed(&adjacency), 3);
        assert_eq!(degrees(&adjacency), vec![2, 1, 0]);
        assert_eq!(max_degree(&adjacency), Some(2));
        assert_eq!(min_degree(&adjacency), Some(0));
        assert_eq!(average_degree(&adjacency), Some(1.0));
        assert_eq!(density_directed(&adjacency), Some(0.5));
    }

    #[test]
    fn computes_undirected_graph_metrics() {
        let adjacency = vec![vec![1, 2], vec![0], vec![0]];

        assert_eq!(edge_count_undirected(&adjacency), 2);
        assert_eq!(density_undirected(&adjacency), Some(2.0 / 3.0));
    }

    #[test]
    fn handles_empty_and_single_node_graphs() {
        assert_eq!(node_count(&[]), 0);
        assert_eq!(average_degree(&[]), None);
        assert_eq!(density_directed(&[]), None);
        assert_eq!(density_undirected(&[]), None);

        let single = vec![Vec::new()];
        assert_eq!(edge_count_directed(&single), 0);
        assert_eq!(edge_count_undirected(&single), 0);
        assert_eq!(average_degree(&single), Some(0.0));
        assert_eq!(density_directed(&single), Some(0.0));
        assert_eq!(density_undirected(&single), Some(0.0));
    }

    #[test]
    fn counts_self_loops_deliberately_in_undirected_graphs() {
        let adjacency = vec![vec![0, 1], vec![0]];

        assert_eq!(edge_count_undirected(&adjacency), 2);
    }
}
