//! Read-only Project Doctor alpha probes and support/export projections.
//!
//! This crate owns the first executable Project Doctor alpha lane. It consumes
//! typed, redaction-safe evidence records from the entry, execution-context,
//! search/index, trust, Git, provider/auth, and restore surfaces and emits
//! stable findings with evidence refs, confidence, and exact recovery or
//! escalation paths.

#![doc(html_root_url = "https://docs.rs/aureline-doctor/0.0.0")]

pub mod probes;
