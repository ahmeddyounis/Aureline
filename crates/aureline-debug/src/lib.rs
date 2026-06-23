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
//! It also exposes [`m5_debug_contracts`] — the frozen, typed matrix that names
//! the M5 debugger object families (debug session, attach target, breakpoint spec,
//! frame mapping, variable/watch snapshot, evaluate request/result, console
//! emission, chronology capability, replay session, and notebook-debug parity),
//! pins one controlled vocabulary across session modes, breakpoint/mapping states,
//! variable freshness, evaluate purity, mapping fidelity, and restore/reattach
//! posture, and maps each object to the proof packet that keeps it current — so
//! notebook, profiler, incident, support, AI, and core debug surfaces consume one
//! debugger object model instead of re-expressing debug truth ad hoc.
//!
//! It also exposes [`m5_debug_session_descriptors`] — the typed, frozen
//! [`DebugSessionDescriptor`](m5_debug_session_descriptors::DebugSessionDescriptor)
//! and
//! [`AttachTargetDescriptor`](m5_debug_session_descriptors::AttachTargetDescriptor)
//! records that materialize two of those families. The canonical
//! [`DebugSessionDescriptorSet`](m5_debug_session_descriptors::DebugSessionDescriptorSet)
//! holds one descriptor per session mode plus the restore/reattach cases, so every
//! debugger-capable surface can explain what was launched or attached, against which
//! target, with what current authority and adapter posture — launch, attach,
//! core-file, replay, and inspect-only stay distinct; attach-target identity,
//! mutability, privilege, and adapter drift survive from picker to session to export;
//! and a restored layout never silently reattaches.
//!
//! It also exposes [`m5_breakpoint_specs`] — the typed, frozen
//! [`BreakpointSpec`](m5_breakpoint_specs::BreakpointSpec) records and
//! [`BreakpointPill`](m5_breakpoint_specs::BreakpointPill) that materialize the
//! breakpoint-spec family. The canonical
//! [`BreakpointSpecSet`](m5_breakpoint_specs::BreakpointSpecSet) carries the full
//! pending/verified/misaligned/unbound/unsupported/policy-blocked/needs-remap truth,
//! so a breakpoint shown in a gutter, session header, list, notebook cell, replay
//! timeline, or export packet traces back to one spec and one verification/mapping
//! vocabulary: a green confirmed-stop icon renders only when a breakpoint is verified,
//! exact, and not replay-only; identity survives rename/reformat/import or degrades to
//! an explicit needs-remap rather than vanishing; a lexical fallback never poses as an
//! exact semantic mapping; and notebook and replay views keep stable cell and frame
//! identity.
//!
//! The reviewer-facing contract is at
//! [`/docs/m4/qualify-chronology-capture-and-replay-support-classes.md`](../../../docs/m4/qualify-chronology-capture-and-replay-support-classes.md).
//! The cross-tool boundary schema is at
//! [`/schemas/debug/chronology-replay-support.schema.json`](../../../schemas/debug/chronology-replay-support.schema.json).
//! The checked-in stable packet is at
//! [`/artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json`](../../../artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json).

#![doc(html_root_url = "https://docs.rs/aureline-debug/0.0.0")]

pub mod canonical_test_discovery_session_and_watch_truth;
pub mod m5_breakpoint_specs;
pub mod m5_debug_contracts;
pub mod m5_debug_session_descriptors;
pub mod qualify_chronology_capture_and_replay_support_classes;
pub mod symbolication;

pub use m5_debug_contracts::{
    m5_debug_contracts_lines, m5_debug_contracts_matrix, DebugConsumer, DebugContractInvariant,
    DebugContractsValidationError, DebugFieldDef, DebugObjectClass, DebugObjectEntry,
    DebugRedactionClass, DebugSharedVocabulary, DebugStateClass, DebugStateTerm, DebugTokenDef,
    DebugVocabulary, M5DebugContractsMatrix, M5_DEBUG_CONTRACTS_ARTIFACT_REF,
    M5_DEBUG_CONTRACTS_AS_OF, M5_DEBUG_CONTRACTS_DOC_REF, M5_DEBUG_CONTRACTS_FIXTURE_REF,
    M5_DEBUG_CONTRACTS_FREEZE_GATE_REF, M5_DEBUG_CONTRACTS_MATRIX_ID,
    M5_DEBUG_CONTRACTS_RECORD_KIND, M5_DEBUG_CONTRACTS_SCHEMA_REF,
    M5_DEBUG_CONTRACTS_SCHEMA_VERSION,
};

pub use m5_breakpoint_specs::{
    m5_breakpoint_spec_lines, m5_breakpoint_spec_set, BreakpointEnablement, BreakpointInvariant,
    BreakpointKindClass, BreakpointMappingProvenance, BreakpointMappingState, BreakpointPayload,
    BreakpointPill, BreakpointScopeClass, BreakpointSourceAnchor, BreakpointSpec,
    BreakpointSpecSet, BreakpointSpecSetValidationError, BreakpointVerificationState,
    NotebookCellAnchor, ReplayFrameAnchor, M5_BREAKPOINT_SPECS_ARTIFACT_REF,
    M5_BREAKPOINT_SPECS_AS_OF, M5_BREAKPOINT_SPECS_DOC_REF, M5_BREAKPOINT_SPECS_FIXTURE_REF,
    M5_BREAKPOINT_SPECS_FREEZE_GATE_REF, M5_BREAKPOINT_SPECS_RECORD_KIND,
    M5_BREAKPOINT_SPECS_SCHEMA_REF, M5_BREAKPOINT_SPECS_SCHEMA_VERSION, M5_BREAKPOINT_SPECS_SET_ID,
};

pub use m5_debug_session_descriptors::{
    m5_debug_session_descriptor_lines, m5_debug_session_descriptor_set, AdapterDriftClass,
    AttachTargetDescriptor, DebugAdapterRef, DebugEntrypointClass, DebugSessionDescriptor,
    DebugSessionDescriptorSet, DebugSessionModeClass, DebugTargetIdentity, DescriptorInvariant,
    DescriptorSetValidationError, ReentryPosture, SessionRunStateClass, TargetBoundaryClass,
    TargetIdentityEcho, TargetKindClass, TargetMutabilityClass, TargetPrivilegeClass,
    M5_DEBUG_SESSION_DESCRIPTORS_ARTIFACT_REF, M5_DEBUG_SESSION_DESCRIPTORS_AS_OF,
    M5_DEBUG_SESSION_DESCRIPTORS_DOC_REF, M5_DEBUG_SESSION_DESCRIPTORS_FIXTURE_REF,
    M5_DEBUG_SESSION_DESCRIPTORS_FREEZE_GATE_REF, M5_DEBUG_SESSION_DESCRIPTORS_RECORD_KIND,
    M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_REF, M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_VERSION,
    M5_DEBUG_SESSION_DESCRIPTORS_SET_ID,
};

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
