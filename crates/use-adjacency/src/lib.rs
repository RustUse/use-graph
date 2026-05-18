#![forbid(unsafe_code)]
//! Primitive adjacency list helpers.
//!
//! The crate provides small builders for directed and undirected adjacency
//! lists plus basic neighbor and degree queries.
//!
//! # Examples
//!
//! ```rust
//! use use_adjacency::{build_directed_adjacency, degree, has_edge, neighbors};
//!
//! let adjacency = build_directed_adjacency(3, &[(0, 1), (0, 2)]).unwrap();
//!
//! assert_eq!(neighbors(&adjacency, 0), Some(&[1, 2][..]));
//! assert_eq!(degree(&adjacency, 0), Some(2));
//! assert!(has_edge(&adjacency, 0, 2));
//! ```

pub type AdjacencyList = Vec<Vec<usize>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjacencyError {
    InvalidNodeIndex,
}

fn validate_edge(node_count: usize, source: usize, target: usize) -> Result<(), AdjacencyError> {
    if source >= node_count || target >= node_count {
        Err(AdjacencyError::InvalidNodeIndex)
    } else {
        Ok(())
    }
}

pub fn build_directed_adjacency(
    node_count: usize,
    edges: &[(usize, usize)],
) -> Result<AdjacencyList, AdjacencyError> {
    let mut adjacency = vec![Vec::new(); node_count];

    for &(source, target) in edges {
        validate_edge(node_count, source, target)?;
        adjacency[source].push(target);
    }

    Ok(adjacency)
}

pub fn build_undirected_adjacency(
    node_count: usize,
    edges: &[(usize, usize)],
) -> Result<AdjacencyList, AdjacencyError> {
    let mut adjacency = vec![Vec::new(); node_count];

    for &(source, target) in edges {
        validate_edge(node_count, source, target)?;
        adjacency[source].push(target);

        if source != target {
            adjacency[target].push(source);
        }
    }

    Ok(adjacency)
}

#[must_use]
pub fn neighbors(adjacency: &AdjacencyList, node: usize) -> Option<&[usize]> {
    adjacency.get(node).map(Vec::as_slice)
}

#[must_use]
pub fn degree(adjacency: &AdjacencyList, node: usize) -> Option<usize> {
    adjacency.get(node).map(Vec::len)
}

#[must_use]
pub fn has_edge(adjacency: &AdjacencyList, source: usize, target: usize) -> bool {
    adjacency
        .get(source)
        .is_some_and(|targets| targets.contains(&target))
}

#[cfg(test)]
mod tests {
    use super::{
        AdjacencyError, build_directed_adjacency, build_undirected_adjacency, degree, has_edge,
        neighbors,
    };

    #[test]
    fn builds_directed_adjacency() {
        let adjacency = build_directed_adjacency(3, &[(0, 1), (0, 2), (1, 2)]).unwrap();

        assert_eq!(neighbors(&adjacency, 0), Some(&[1, 2][..]));
        assert_eq!(degree(&adjacency, 0), Some(2));
        assert!(has_edge(&adjacency, 1, 2));
        assert!(!has_edge(&adjacency, 2, 1));
    }

    #[test]
    fn builds_undirected_adjacency_and_preserves_duplicates() {
        let adjacency = build_undirected_adjacency(3, &[(0, 1), (1, 2), (0, 1)]).unwrap();

        assert_eq!(neighbors(&adjacency, 0), Some(&[1, 1][..]));
        assert_eq!(neighbors(&adjacency, 1), Some(&[0, 2, 0][..]));
        assert!(has_edge(&adjacency, 2, 1));
    }

    #[test]
    fn handles_self_loops_and_empty_graphs() {
        let adjacency = build_undirected_adjacency(1, &[(0, 0)]).unwrap();

        assert_eq!(neighbors(&adjacency, 0), Some(&[0][..]));
        assert_eq!(
            build_directed_adjacency(0, &[]).unwrap(),
            Vec::<Vec<usize>>::new()
        );
    }

    #[test]
    fn rejects_invalid_node_indexes() {
        assert_eq!(
            build_directed_adjacency(2, &[(0, 2)]),
            Err(AdjacencyError::InvalidNodeIndex)
        );
        assert_eq!(
            build_undirected_adjacency(2, &[(3, 1)]),
            Err(AdjacencyError::InvalidNodeIndex)
        );
    }
}
