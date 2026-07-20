//! Networking stack: HTTP/1.1, HTTP/2, HTTP/3, TLS, caching, cookies.
//!
//! **Owning team**: Networking Team
//!
//! Planned high-level API (architecture §3.2): `fetch(request) -> Response`.
//! Renderer processes never open raw sockets; all network goes through
//! the Network process. Cookie jar / cache policy decisions are owned by
//! Security & Storage.

#![forbid(unsafe_code)]

// TODO(net): Implement fetch API, protocols, TLS, and connection pooling.
