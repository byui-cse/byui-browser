//! Top-level browser binary.
//!
//! **Primary owners**: Browser UX Team (process wiring is cross-team).
//!
//! This binary will eventually spawn Browser, Renderer, Network, GPU,
//! Utility/Storage, and DevTools processes per the architecture process model.
//! Teams own the crates; this crate only composes them.

fn main() {
    // TODO(browser): Wire up multi-process startup and the first vertical slice.
    eprintln!("byui-browser: scaffolding only — engine not implemented yet");
}
