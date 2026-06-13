//! Debug-session chronology, replay support class truth, symbolication manifests and reports,
//! and capability descriptor contracts.
//!
//! This crate owns the boundary contract for chronology capture and replay
//! support class qualification across local, remote/helper, container, and
//! notebook-bridge debug lanes. It exposes one canonical
//! [`qualify_chronology_capture_and_replay_support_classes`] module that pins
//! the replay support class truth every debugger UI, support export, and
//! release reviewer reads.
//!
//! It also exposes [`symbolication`] for exact-build symbol and source-map
//! manifests, local or mirrored symbolication reports, and the shared fidelity
//! labels rendered by debug, profiler, preview, browser-runtime, and support
//! surfaces.
//!
//! It also exposes
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
pub mod symbolication;

pub use symbolication::{
    current_symbolication_contract, BuildMatchState, DebugFormatClass, MirrorPolicyRow,
    ResolutionSourceClass, RetentionPostureClass, SourceIdentityClass, SurfaceProjectionRow,
    SymbolManifestRow, SymbolicationContractArtifactError, SymbolicationContractPacket,
    SymbolicationContractSummary, SymbolicationContractViolation, SymbolicationFidelityLabel,
    SymbolicationRedactionClass, SymbolicationReportRow, SymbolicationSourceUsageRow,
    SymbolicationSurfaceKind, SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF,
    SYMBOLICATION_CONTRACT_DOC_REF, SYMBOLICATION_CONTRACT_FIXTURE_DIR,
    SYMBOLICATION_CONTRACT_PACKET_JSON, SYMBOLICATION_CONTRACT_PACKET_PATH,
    SYMBOLICATION_CONTRACT_RECORD_KIND, SYMBOLICATION_CONTRACT_SCHEMA_REF,
    SYMBOLICATION_CONTRACT_SCHEMA_VERSION,
};
