//! Debug-session chronology, replay support class truth, and capability descriptor contracts.
//!
//! This crate owns the boundary contract for chronology capture and replay
//! support class qualification across local, remote/helper, container, and
//! notebook-bridge debug lanes. It exposes one canonical
//! [`qualify_chronology_capture_and_replay_support_classes`] module that pins
//! the replay support class truth every debugger UI, support export, and
//! release reviewer reads. It also exposes
//! [`canonical_test_discovery_session_and_watch_truth`] for stable test
//! discovery/session/watch/quarantine/imported-CI packets shared by runtime,
//! support, and release evidence surfaces.
//!
//! The reviewer-facing contract is at
//! [`/docs/m4/qualify-chronology-capture-and-replay-support-classes.md`](../../../docs/m4/qualify-chronology-capture-and-replay-support-classes.md).
//! The cross-tool boundary schema is at
//! [`/schemas/debug/chronology-replay-support.schema.json`](../../../schemas/debug/chronology-replay-support.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json`](../../../artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json).

#![doc(html_root_url = "https://docs.rs/aureline-debug/0.0.0")]

pub mod canonical_test_discovery_session_and_watch_truth;
pub mod qualify_chronology_capture_and_replay_support_classes;
