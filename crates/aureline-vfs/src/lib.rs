//! Virtual filesystem layer.
//!
//! Owns workspace roots, canonical path identity, file watching, ignore
//! resolution, and the abstraction that lets local, remote, and overlay
//! filesystems present a unified surface to the rest of the system.

#![doc(html_root_url = "https://docs.rs/aureline-vfs/0.0.0")]
