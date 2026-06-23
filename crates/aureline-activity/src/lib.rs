//! Attention-routing, durable-activity, and cross-client fanout contracts.
//!
//! This crate owns the frozen object model Aureline uses for notifications,
//! durable jobs, badge counts, quiet-hours routing, and fanout truth across the
//! shell activity center, OS notifications, companion/cross-client surfaces, and
//! operator dashboards. It does not deliver notifications; it provides the typed,
//! inspectable matrix that names each governed attention object, freezes its
//! stable identifiers and required fields, pins one controlled vocabulary across
//! them, maps each one to the proof packet that keeps it current, and states the
//! invariants every attention surface must hold — so docs, help, support,
//! activity, and companion packets point at the same object model rather than
//! re-expressing notification truth ad hoc.
//!
//! The records this crate produces are metadata-safe truth packets: they carry no
//! message bodies, credentials, raw provider payloads, hostnames, or absolute
//! paths — only opaque object refs, stable tokens, and short reviewable
//! sentences — so they are safe to embed in a support export verbatim.

#![doc(html_root_url = "https://docs.rs/aureline-activity/0.0.0")]

pub mod m5_activity_objects;
pub mod m5_attention_actions;
pub mod m5_attention_routing;
pub mod m5_envelope_routing;
pub mod m5_fanout_receipts;
pub mod m5_quiet_hours_suppression;
