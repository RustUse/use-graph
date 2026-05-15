# use-graph

Composable primitive graph utilities for Rust.

`use-graph` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-simulation`,
`use-control`, `use-signal`, `use-color`, `use-text`, `use-time`, and
`use-units`. It groups small, focused crates for node identifiers, edges,
adjacency lists, traversal, paths, weighted graphs, and simple graph metrics.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `usize`, `f64`, `Vec`, and small structs or enums
- dependencies stay minimal so each crate is easy to audit and adopt

## Workspace crates

- `use-graph`: thin facade crate that reexports the full graph workspace
- `use-node`: primitive node identifiers and node collection helpers
- `use-edge`: primitive directed, undirected, and generic edge helpers
- `use-adjacency`: adjacency list builders and lookup helpers
- `use-graph-traversal`: deterministic breadth-first and depth-first traversal helpers
- `use-graph-path`: path validation and unweighted shortest-path helpers
- `use-weighted-graph`: weighted edges and weighted adjacency builders
- `use-graph-metrics`: simple graph size, degree, and density helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-graph`. It
reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_graph::*;

let adjacency = build_directed_adjacency(4, &[(0, 1), (1, 2), (0, 3)]).unwrap();
let path = shortest_path_unweighted(&adjacency, 0, 2).unwrap().unwrap();
let edge = WeightedEdge::new(0, 1, 1.5).unwrap();

assert_eq!(path.nodes(), &[0, 1, 2]);
assert_eq!(path_weight(&[edge]).unwrap(), 1.5);
assert_eq!(max_degree(&adjacency), Some(2));
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse graph
surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
