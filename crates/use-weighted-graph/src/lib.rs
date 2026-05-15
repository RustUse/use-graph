#![forbid(unsafe_code)]
//! Primitive weighted graph helpers.
//!
//! The crate provides a small weighted-edge type plus adjacency builders and a
//! few helpers for summarizing edge weights.
//!
//! # Examples
//!
//! ```rust
//! use use_weighted_graph::{WeightedEdge, build_weighted_directed_adjacency, path_weight};
//!
//! let edge = WeightedEdge::new(0, 1, 2.5).unwrap();
//! let adjacency = build_weighted_directed_adjacency(2, &[edge]).unwrap();
//!
//! assert_eq!(adjacency, vec![vec![(1, 2.5)], vec![]]);
//! assert_eq!(path_weight(&[edge]), Some(2.5));
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedEdge {
    pub source: usize,
    pub target: usize,
    pub weight: f64,
}

pub type WeightedAdjacencyList = Vec<Vec<(usize, f64)>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightedGraphError {
    InvalidNodeIndex,
    InvalidWeight,
}

impl WeightedEdge {
    pub fn new(source: usize, target: usize, weight: f64) -> Result<Self, WeightedGraphError> {
        if !weight.is_finite() {
            return Err(WeightedGraphError::InvalidWeight);
        }

        Ok(Self {
            source,
            target,
            weight,
        })
    }

    #[must_use]
    pub const fn reversed(&self) -> Self {
        Self {
            source: self.target,
            target: self.source,
            weight: self.weight,
        }
    }
}

fn validate_edge(node_count: usize, edge: WeightedEdge) -> Result<(), WeightedGraphError> {
    if !edge.weight.is_finite() {
        return Err(WeightedGraphError::InvalidWeight);
    }

    if edge.source >= node_count || edge.target >= node_count {
        Err(WeightedGraphError::InvalidNodeIndex)
    } else {
        Ok(())
    }
}

pub fn build_weighted_directed_adjacency(
    node_count: usize,
    edges: &[WeightedEdge],
) -> Result<WeightedAdjacencyList, WeightedGraphError> {
    let mut adjacency = vec![Vec::new(); node_count];

    for &edge in edges {
        validate_edge(node_count, edge)?;
        adjacency[edge.source].push((edge.target, edge.weight));
    }

    Ok(adjacency)
}

pub fn build_weighted_undirected_adjacency(
    node_count: usize,
    edges: &[WeightedEdge],
) -> Result<WeightedAdjacencyList, WeightedGraphError> {
    let mut adjacency = vec![Vec::new(); node_count];

    for &edge in edges {
        validate_edge(node_count, edge)?;
        adjacency[edge.source].push((edge.target, edge.weight));

        if edge.source != edge.target {
            let reversed = edge.reversed();
            adjacency[reversed.source].push((reversed.target, reversed.weight));
        }
    }

    Ok(adjacency)
}

#[must_use]
pub fn path_weight(edges: &[WeightedEdge]) -> Option<f64> {
    if edges.is_empty() || edges.iter().any(|edge| !edge.weight.is_finite()) {
        None
    } else {
        Some(edges.iter().map(|edge| edge.weight).sum())
    }
}

#[must_use]
pub fn min_weight_edge(edges: &[WeightedEdge]) -> Option<WeightedEdge> {
    if edges.is_empty() || edges.iter().any(|edge| !edge.weight.is_finite()) {
        None
    } else {
        edges
            .iter()
            .copied()
            .min_by(|left, right| left.weight.total_cmp(&right.weight))
    }
}

#[must_use]
pub fn max_weight_edge(edges: &[WeightedEdge]) -> Option<WeightedEdge> {
    if edges.is_empty() || edges.iter().any(|edge| !edge.weight.is_finite()) {
        None
    } else {
        edges
            .iter()
            .copied()
            .max_by(|left, right| left.weight.total_cmp(&right.weight))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_weighted_directed_adjacency, build_weighted_undirected_adjacency, max_weight_edge,
        min_weight_edge, path_weight, WeightedEdge, WeightedGraphError,
    };

    #[test]
    fn constructs_weighted_edges() {
        let edge = WeightedEdge::new(0, 1, 2.5).unwrap();

        assert_eq!(edge.reversed(), WeightedEdge::new(1, 0, 2.5).unwrap());
    }

    #[test]
    fn builds_weighted_adjacency_lists() {
        let edges = [
            WeightedEdge::new(0, 1, 2.0).unwrap(),
            WeightedEdge::new(1, 2, 3.5).unwrap(),
        ];

        assert_eq!(
            build_weighted_directed_adjacency(3, &edges).unwrap(),
            vec![vec![(1, 2.0)], vec![(2, 3.5)], vec![]]
        );
        assert_eq!(
            build_weighted_undirected_adjacency(3, &edges).unwrap(),
            vec![vec![(1, 2.0)], vec![(0, 2.0), (2, 3.5)], vec![(1, 3.5)]]
        );
    }

    #[test]
    fn summarizes_weights() {
        let edges = [
            WeightedEdge::new(0, 1, 2.0).unwrap(),
            WeightedEdge::new(1, 2, 3.5).unwrap(),
            WeightedEdge::new(2, 3, 1.0).unwrap(),
        ];

        assert_eq!(path_weight(&edges), Some(6.5));
        assert_eq!(
            min_weight_edge(&edges),
            Some(WeightedEdge::new(2, 3, 1.0).unwrap())
        );
        assert_eq!(
            max_weight_edge(&edges),
            Some(WeightedEdge::new(1, 2, 3.5).unwrap())
        );
    }

    #[test]
    fn rejects_invalid_weighted_inputs() {
        assert_eq!(
            WeightedEdge::new(0, 1, f64::NAN),
            Err(WeightedGraphError::InvalidWeight)
        );
        assert_eq!(
            build_weighted_directed_adjacency(
                2,
                &[WeightedEdge {
                    source: 0,
                    target: 2,
                    weight: 1.0,
                }]
            ),
            Err(WeightedGraphError::InvalidNodeIndex)
        );
    }

    #[test]
    fn handles_empty_weight_summaries() {
        assert_eq!(path_weight(&[]), None);
        assert_eq!(min_weight_edge(&[]), None);
        assert_eq!(max_weight_edge(&[]), None);
    }
}
