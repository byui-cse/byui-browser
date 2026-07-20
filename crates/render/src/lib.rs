//! GPU rendering, layer composition, and compositor.
//!
//! **Owning team**: Layout & Rendering Team
//!
//! Consumes display lists / layer trees from `paint` and produces pixels
//! (or GPU command streams). Runs in the GPU / Compositor process.

#![forbid(unsafe_code)]

// TODO(render): Implement compositor and GPU backend (wgpu).
