#![forbid(unsafe_code)]
//! Primitive graph traversal helpers.
//!
//! The crate provides deterministic breadth-first and depth-first traversals
//! that follow the existing adjacency list order.
//!
//! # Examples
//!
//! ```rust
//! use use_graph_traversal::{breadth_first_order, connected_component, reachable};
//!
//! let adjacency = vec![vec![1, 2], vec![3], vec![], vec![]];
//!
//! assert_eq!(breadth_first_order(&adjacency, 0).unwrap(), vec![0, 1, 2, 3]);
//! assert!(reachable(&adjacency, 0, 3).unwrap());
//! assert_eq!(connected_component(&adjacency, 0).unwrap(), vec![0, 1, 2, 3]);
//! ```

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalError {
    InvalidStartNode,
    InvalidTargetNode,
    InvalidNeighborIndex,
}

fn validate_start(adjacency: &[Vec<usize>], start: usize) -> Result<(), TraversalError> {
    if start >= adjacency.len() {
        Err(TraversalError::InvalidStartNode)
    } else {
        Ok(())
    }
}

fn validate_target(adjacency: &[Vec<usize>], target: usize) -> Result<(), TraversalError> {
    if target >= adjacency.len() {
        Err(TraversalError::InvalidTargetNode)
    } else {
        Ok(())
    }
}

fn validate_neighbor(adjacency: &[Vec<usize>], neighbor: usize) -> Result<(), TraversalError> {
    if neighbor >= adjacency.len() {
        Err(TraversalError::InvalidNeighborIndex)
    } else {
        Ok(())
    }
}

pub fn breadth_first_order(
    adjacency: &[Vec<usize>],
    start: usize,
) -> Result<Vec<usize>, TraversalError> {
    validate_start(adjacency, start)?;

    let mut visited = vec![false; adjacency.len()];
    let mut queue = VecDeque::new();
    let mut order = Vec::with_capacity(adjacency.len());

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);

        for &neighbor in &adjacency[node] {
            validate_neighbor(adjacency, neighbor)?;

            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    Ok(order)
}

pub fn depth_first_order(
    adjacency: &[Vec<usize>],
    start: usize,
) -> Result<Vec<usize>, TraversalError> {
    validate_start(adjacency, start)?;

    let mut visited = vec![false; adjacency.len()];
    let mut stack = vec![start];
    let mut order = Vec::with_capacity(adjacency.len());

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }

        visited[node] = true;
        order.push(node);

        for &neighbor in adjacency[node].iter().rev() {
            validate_neighbor(adjacency, neighbor)?;

            if !visited[neighbor] {
                stack.push(neighbor);
            }
        }
    }

    Ok(order)
}

pub fn reachable(
    adjacency: &[Vec<usize>],
    start: usize,
    target: usize,
) -> Result<bool, TraversalError> {
    validate_start(adjacency, start)?;
    validate_target(adjacency, target)?;

    if start == target {
        return Ok(true);
    }

    let mut visited = vec![false; adjacency.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adjacency[node] {
            validate_neighbor(adjacency, neighbor)?;

            if neighbor == target {
                return Ok(true);
            }

            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    Ok(false)
}

pub fn connected_component(
    adjacency: &[Vec<usize>],
    start: usize,
) -> Result<Vec<usize>, TraversalError> {
    breadth_first_order(adjacency, start)
}

#[cfg(test)]
mod tests {
    use super::{
        breadth_first_order, connected_component, depth_first_order, reachable, TraversalError,
    };

    #[test]
    fn performs_breadth_first_traversal() {
        let adjacency = vec![vec![1, 2], vec![3], vec![3], vec![]];

        assert_eq!(
            breadth_first_order(&adjacency, 0).unwrap(),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn performs_depth_first_traversal_in_adjacency_order() {
        let adjacency = vec![vec![1, 2], vec![3], vec![], vec![]];

        assert_eq!(depth_first_order(&adjacency, 0).unwrap(), vec![0, 1, 3, 2]);
    }

    #[test]
    fn checks_reachability_and_connected_components() {
        let adjacency = vec![vec![1], vec![2], vec![], vec![4], vec![]];

        assert!(reachable(&adjacency, 0, 2).unwrap());
        assert!(!reachable(&adjacency, 0, 4).unwrap());
        assert_eq!(connected_component(&adjacency, 3).unwrap(), vec![3, 4]);
    }

    #[test]
    fn rejects_invalid_indexes() {
        let adjacency = vec![vec![1], vec![]];

        assert_eq!(
            breadth_first_order(&adjacency, 2),
            Err(TraversalError::InvalidStartNode)
        );
        assert_eq!(
            reachable(&adjacency, 0, 2),
            Err(TraversalError::InvalidTargetNode)
        );
    }

    #[test]
    fn rejects_invalid_neighbor_indexes_in_input_adjacency() {
        let adjacency = vec![vec![3], vec![]];

        assert_eq!(
            depth_first_order(&adjacency, 0),
            Err(TraversalError::InvalidNeighborIndex)
        );
    }
}
