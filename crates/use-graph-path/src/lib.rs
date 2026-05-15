#![forbid(unsafe_code)]
//! Primitive path helpers.
//!
//! The crate keeps path handling explicit through a small `Path` wrapper and a
//! breadth-first shortest-path helper for unweighted graphs.
//!
//! # Examples
//!
//! ```rust
//! use use_graph_path::{Path, is_valid_path, shortest_path_unweighted};
//!
//! let adjacency = vec![vec![1], vec![2], vec![]];
//! let shortest = shortest_path_unweighted(&adjacency, 0, 2).unwrap().unwrap();
//! let path = Path::new(vec![0, 1, 2]).unwrap();
//!
//! assert_eq!(shortest.nodes(), &[0, 1, 2]);
//! assert!(is_valid_path(&adjacency, path.nodes()).unwrap());
//! ```

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    nodes: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathError {
    EmptyPath,
    InvalidStartNode,
    InvalidTargetNode,
    InvalidNodeIndex,
    InvalidNeighborIndex,
}

impl Path {
    pub fn new(nodes: Vec<usize>) -> Result<Self, PathError> {
        if nodes.is_empty() {
            Err(PathError::EmptyPath)
        } else {
            Ok(Self { nodes })
        }
    }

    #[must_use]
    pub fn nodes(&self) -> &[usize] {
        self.nodes.as_slice()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.nodes.len().saturating_sub(1)
    }

    #[must_use]
    pub fn starts_at(&self, node: usize) -> bool {
        self.nodes.first().copied() == Some(node)
    }

    #[must_use]
    pub fn ends_at(&self, node: usize) -> bool {
        self.nodes.last().copied() == Some(node)
    }
}

fn validate_node(adjacency: &[Vec<usize>], node: usize, error: PathError) -> Result<(), PathError> {
    if node >= adjacency.len() {
        Err(error)
    } else {
        Ok(())
    }
}

fn reconstruct_path(parents: &[Option<usize>], start: usize, target: usize) -> Option<Path> {
    let mut nodes = vec![target];
    let mut current = target;

    while current != start {
        current = parents[current]?;
        nodes.push(current);
    }

    nodes.reverse();
    Path::new(nodes).ok()
}

pub fn shortest_path_unweighted(
    adjacency: &[Vec<usize>],
    start: usize,
    target: usize,
) -> Result<Option<Path>, PathError> {
    validate_node(adjacency, start, PathError::InvalidStartNode)?;
    validate_node(adjacency, target, PathError::InvalidTargetNode)?;

    if start == target {
        return Ok(Some(Path::new(vec![start])?));
    }

    let mut visited = vec![false; adjacency.len()];
    let mut parents = vec![None; adjacency.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adjacency[node] {
            if neighbor >= adjacency.len() {
                return Err(PathError::InvalidNeighborIndex);
            }

            if !visited[neighbor] {
                visited[neighbor] = true;
                parents[neighbor] = Some(node);

                if neighbor == target {
                    return Ok(reconstruct_path(&parents, start, target));
                }

                queue.push_back(neighbor);
            }
        }
    }

    Ok(None)
}

pub fn is_valid_path(adjacency: &[Vec<usize>], path: &[usize]) -> Result<bool, PathError> {
    if path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    for &node in path {
        validate_node(adjacency, node, PathError::InvalidNodeIndex)?;
    }

    Ok(path
        .windows(2)
        .all(|edge| adjacency[edge[0]].contains(&edge[1])))
}

#[cfg(test)]
mod tests {
    use super::{is_valid_path, shortest_path_unweighted, Path, PathError};

    #[test]
    fn constructs_paths_and_reports_basic_properties() {
        let path = Path::new(vec![0, 1, 2]).unwrap();

        assert_eq!(path.nodes(), &[0, 1, 2]);
        assert_eq!(path.len(), 3);
        assert!(!path.is_empty());
        assert_eq!(path.edge_count(), 2);
        assert!(path.starts_at(0));
        assert!(path.ends_at(2));
    }

    #[test]
    fn finds_shortest_unweighted_paths() {
        let adjacency = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let path = shortest_path_unweighted(&adjacency, 0, 3).unwrap().unwrap();

        assert_eq!(path.nodes(), &[0, 1, 3]);
    }

    #[test]
    fn handles_unreachable_and_single_node_paths() {
        let adjacency = vec![vec![1], vec![], vec![]];

        assert_eq!(shortest_path_unweighted(&adjacency, 0, 2).unwrap(), None);
        assert_eq!(
            shortest_path_unweighted(&adjacency, 1, 1)
                .unwrap()
                .unwrap()
                .nodes(),
            &[1]
        );
    }

    #[test]
    fn validates_paths_against_adjacency() {
        let adjacency = vec![vec![1], vec![2], vec![]];

        assert!(is_valid_path(&adjacency, &[0, 1, 2]).unwrap());
        assert!(!is_valid_path(&adjacency, &[0, 2]).unwrap());
    }

    #[test]
    fn rejects_invalid_inputs() {
        let adjacency = vec![vec![3], vec![]];

        assert_eq!(Path::new(vec![]), Err(PathError::EmptyPath));
        assert_eq!(
            shortest_path_unweighted(&adjacency, 2, 0),
            Err(PathError::InvalidStartNode)
        );
        assert_eq!(
            shortest_path_unweighted(&adjacency, 0, 1),
            Err(PathError::InvalidNeighborIndex)
        );
        assert_eq!(is_valid_path(&[vec![]], &[]), Err(PathError::EmptyPath));
        assert_eq!(
            is_valid_path(&[vec![]], &[1]),
            Err(PathError::InvalidNodeIndex)
        );
    }
}
