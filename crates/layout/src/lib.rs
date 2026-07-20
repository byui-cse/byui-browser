//! Box tree construction and layout (block, inline, flex, grid).
//!
//! **Owning team**: Layout & Rendering Team
//!
//! Planned interface (architecture §3.2):
//! `layout_tree(styled_dom, viewport) -> LayoutTree`
//!
//! Layout never talks directly to the GPU.

#![forbid(unsafe_code)]

// TODO(layout): Implement box tree and layout algorithms.
