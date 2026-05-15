//! Hot-path instrumentation, tracing, and metrics primitives.
//!
//! Provides the recording surface used by every other crate to emit spans,
//! counters, and latency samples that feed protected-path traces and the
//! support-bundle export.

#![doc(html_root_url = "https://docs.rs/aureline-telemetry/0.0.0")]

pub mod endpoint_policy;
pub mod hot_path_metrics;
pub mod onboarding;
pub mod trace_event;
