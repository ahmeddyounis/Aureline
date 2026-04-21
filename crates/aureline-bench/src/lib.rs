//! Benchmark harness and protected-path trace fixtures.
//!
//! Hosts the harness that drives latency, throughput, and trace-fidelity
//! measurements for protected-path workflows. Production crates must not depend
//! on this; it pulls them in, never the other way around.

#![doc(html_root_url = "https://docs.rs/aureline-bench/0.0.0")]

pub mod text_stack;
