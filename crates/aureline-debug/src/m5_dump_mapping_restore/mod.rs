//! Typed dump/core-file/source-map/symbol artifact strips, one shared six-state mapping
//! fidelity vocabulary, and restore-honesty records: the canonical M5 records every
//! debugger, notebook, profiler, incident, support, and AI surface reads to show *which*
//! debug artifact it opened, *how trustworthy* that artifact's source/symbol mapping is,
//! which build it belongs to, and — when a layout is reopened — whether the prior
//! process/session is gone, inspect-only, reconnect-required, or manually relaunchable.
//!
//! The [`symbolication`](crate::symbolication) lane already pins the symbol/source-map
//! manifest, build-match, and four-state user-facing fidelity vocabulary at the artifact
//! provenance level; the
//! [`m5_frame_variable_snapshots`](crate::m5_frame_variable_snapshots) lane pins the
//! four-state frame-mapping fidelity at the stack-frame level. This lane *materializes*
//! the dump/core-file/source-map/symbol *strip* family that those surfaces render at the
//! top of a debug pane, and widens the four-state mapping vocabulary into one shared
//! [`DebugMappingFidelity`] that adds the two artifact-level degradations — `imported`
//! (a bounded-trust side-load) and `mismatched_build` (a mapping attempted against a
//! build that does not match) — so frames, breakpoints, variables, and dump artifacts all
//! read one mapping vocabulary instead of re-expressing fidelity ad hoc.
//!
//! Dump and restore truth stays explicit and replay-safe:
//!
//! - **One artifact strip, one mapping pill.** Every [`DebugArtifactStrip`] carries one
//!   [`DebugArtifactPill`] pinning one [`DebugMappingFidelity`], one [`ArtifactBuildMatch`],
//!   and one [`ArtifactSourceClass`]. A precise source link
//!   ([`shows_exact_source_link`](DebugArtifactPill::shows_exact_source_link)) renders only
//!   for an *exact* mapping backed by an *exact-build* match; an approximate, symbol-only,
//!   unresolved, imported, or build-mismatched strip always
//!   [`requires_disclosure`](DebugArtifactPill::requires_disclosure).
//! - **Entrypoints stay distinct and visible.** Core-file, crash-dump, open-replay, and
//!   open-inspect-only are four distinct [`DebugArtifactEntrypoint`] values that each open
//!   an inspect-only session; importing a symbol or source-map artifact is a fifth,
//!   non-session entrypoint. A strip never flattens which entrypoint opened it.
//! - **Build / artifact identity is always present.** Every strip carries a build id or an
//!   artifact ref, the artifact kind, a capture time, and a source class (workspace,
//!   local, provider, mirror, or imported), so a surface can show current build/artifact
//!   identity and exact-versus-degraded mapping without dropping into support-only
//!   diagnostics.
//! - **Imported and mismatched-build stay honest.** An [`DebugMappingFidelity::Imported`]
//!   strip is always sourced from an import, and a
//!   [`DebugMappingFidelity::MismatchedBuild`] strip always carries a rejected build
//!   match; neither ever renders the exact source link.
//! - **Restored layouts never imply reacquired authority.** Every
//!   [`RestoredLayoutRecord`] carries one [`RestorePill`] whose
//!   [`implies_live_continuity`](RestorePill::implies_live_continuity) and
//!   [`implies_process_authority`](RestorePill::implies_process_authority) are always
//!   false, names one [`RestorePosture`] (gone, inspect-only, reconnect-required, or
//!   manually relaunchable), and shows the exact-build mapping only when it is *still*
//!   verified — so a reopened pane never implies live target continuity or exact-build
//!   mapping when that is no longer true.
//!
//! [`m5_dump_mapping_restore_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`DumpRestoreInvariant`]'s `holds` flag from the
//! built records, so the checked-in fixture and the freeze gate freeze the contract
//! byte-for-byte and an inconsistent edit flips an invariant and fails CI. The record
//! carries no dump bodies, memory contents, source bodies, raw paths, provider payloads,
//! URLs, hostnames, or credentials — only opaque object refs, stable tokens, opaque
//! digests, and short reviewable sentences — so it is safe for support export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_dump_mapping_restore.schema.json`](../../../schemas/debug/m5_dump_mapping_restore.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_dump_mapping_restore/canonical_set.json`](../../../fixtures/debug/m5_dump_mapping_restore/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_dump_mapping_restore.md`](../../../docs/debug/m5_dump_mapping_restore.md).

use serde::{Deserialize, Serialize};

use crate::m5_debug_contracts::DebugConsumer;
use crate::m5_frame_variable_snapshots::FrameMappingFidelity;
use crate::symbolication::{DebugFormatClass, SymbolicationFidelityLabel};

#[cfg(test)]
mod tests;

/// Schema version for the M5 dump/mapping/restore set.
pub const M5_DUMP_MAPPING_RESTORE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 dump/mapping/restore set.
pub const M5_DUMP_MAPPING_RESTORE_SCHEMA_REF: &str =
    "schemas/debug/m5_dump_mapping_restore.schema.json";

/// Stable record-kind tag for the dump/mapping/restore set.
pub const M5_DUMP_MAPPING_RESTORE_RECORD_KIND: &str = "m5_dump_mapping_restore_set";

/// Stable id for the canonical dump/mapping/restore set.
pub const M5_DUMP_MAPPING_RESTORE_SET_ID: &str = "m5-dump-mapping-restore:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_DUMP_MAPPING_RESTORE_AS_OF: &str = "2026-06-26T00:00:00Z";

/// The freeze gate that keeps the dump/mapping/restore set current. Stable promotion runs
/// this gate; it fails when the in-code set drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_DUMP_MAPPING_RESTORE_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_dump_mapping_restore.rs";

/// The checked-in canonical dump/mapping/restore-set fixture.
pub const M5_DUMP_MAPPING_RESTORE_FIXTURE_REF: &str =
    "fixtures/debug/m5_dump_mapping_restore/canonical_set.json";

/// The contract narrative document.
pub const M5_DUMP_MAPPING_RESTORE_DOC_REF: &str = "docs/debug/m5_dump_mapping_restore.md";

/// The human-readable evidence companion artifact.
pub const M5_DUMP_MAPPING_RESTORE_ARTIFACT_REF: &str = "artifacts/debug/m5_dump_mapping_restore.md";

// ---------------------------------------------------------------------------
// Shared mapping fidelity vocabulary.
// ---------------------------------------------------------------------------

/// The single controlled mapping-fidelity vocabulary rendered wherever a frame,
/// breakpoint, variable, or dump artifact invites trust in a source/symbol mapping.
///
/// This is the superset of the four-state frame-mapping fidelity
/// ([`FrameMappingFidelity`]) and the four-state symbolication fidelity
/// ([`SymbolicationFidelityLabel`]); it adds the two artifact-level degradations
/// [`DebugMappingFidelity::Imported`] and [`DebugMappingFidelity::MismatchedBuild`]. Only
/// [`DebugMappingFidelity::Exact`] preserves an authoritative mapping; every other state
/// requires a visible caveat so an imported or build-mismatched mapping is never drawn as
/// a precise source link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugMappingFidelity {
    /// Maps exactly to an authoritative source/symbol against an exact build.
    Exact,
    /// Maps approximately (line-only, drifted, or nearest-span), disclosed as inexact.
    Approximate,
    /// Resolves a symbol / function name only, without authoritative source lines.
    SymbolOnly,
    /// Could not be mapped to source or symbol; an explicit unresolved mapping.
    Unresolved,
    /// Resolved from an imported / side-loaded artifact whose authority is bounded.
    Imported,
    /// A candidate mapping was found but the build it belongs to does not match the
    /// artifact under inspection; the mapping is rejected as untrustworthy.
    MismatchedBuild,
}

impl DebugMappingFidelity {
    /// All fidelity classes, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Exact,
        Self::Approximate,
        Self::SymbolOnly,
        Self::Unresolved,
        Self::Imported,
        Self::MismatchedBuild,
    ];

    /// Stable snake_case token for this fidelity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::SymbolOnly => "symbol_only",
            Self::Unresolved => "unresolved",
            Self::Imported => "imported",
            Self::MismatchedBuild => "mismatched_build",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Approximate => "Approximate",
            Self::SymbolOnly => "Symbol-only",
            Self::Unresolved => "Unresolved",
            Self::Imported => "Imported",
            Self::MismatchedBuild => "Build mismatch",
        }
    }

    /// Short pill fragment for this fidelity.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::SymbolOnly => "symbol-only",
            Self::Unresolved => "unresolved",
            Self::Imported => "imported",
            Self::MismatchedBuild => "build mismatch",
        }
    }

    /// Whether this fidelity preserves an authoritative source mapping.
    pub const fn preserves_exact_source(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Whether this fidelity must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact)
    }

    /// Whether this fidelity offers a source line a reader can navigate to. An imported
    /// mapping still resolves a line (under a bounded-trust caveat); a symbol-only,
    /// unresolved, or build-mismatched mapping does not.
    pub const fn allows_source_navigation(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate | Self::Imported)
    }

    /// Whether this fidelity is the imported / side-loaded state.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::Imported)
    }

    /// Whether this fidelity is the build-mismatch state.
    pub const fn is_build_mismatch(self) -> bool {
        matches!(self, Self::MismatchedBuild)
    }

    /// Widens a four-state frame-mapping fidelity into the shared vocabulary, proving the
    /// frame fidelity is a subset of this one.
    pub const fn from_frame_fidelity(fidelity: FrameMappingFidelity) -> Self {
        match fidelity {
            FrameMappingFidelity::Exact => Self::Exact,
            FrameMappingFidelity::Approximate => Self::Approximate,
            FrameMappingFidelity::SymbolOnly => Self::SymbolOnly,
            FrameMappingFidelity::Unmapped => Self::Unresolved,
        }
    }

    /// Widens a four-state symbolication fidelity label into the shared vocabulary.
    pub const fn from_symbolication_label(label: SymbolicationFidelityLabel) -> Self {
        match label {
            SymbolicationFidelityLabel::Exact => Self::Exact,
            SymbolicationFidelityLabel::Approximate => Self::Approximate,
            SymbolicationFidelityLabel::SymbolOnly => Self::SymbolOnly,
            SymbolicationFidelityLabel::Unresolved => Self::Unresolved,
        }
    }

    /// Narrows the shared vocabulary back to a frame-mapping fidelity, so a debugger frame
    /// stack can render a dump-artifact mapping with its existing pill: imported degrades
    /// to approximate (bounded-trust but navigable) and build-mismatch degrades to
    /// unmapped (untrustworthy).
    pub const fn narrow_to_frame_fidelity(self) -> FrameMappingFidelity {
        match self {
            Self::Exact => FrameMappingFidelity::Exact,
            Self::Approximate => FrameMappingFidelity::Approximate,
            Self::Imported => FrameMappingFidelity::Approximate,
            Self::SymbolOnly => FrameMappingFidelity::SymbolOnly,
            Self::Unresolved => FrameMappingFidelity::Unmapped,
            Self::MismatchedBuild => FrameMappingFidelity::Unmapped,
        }
    }
}

// ---------------------------------------------------------------------------
// Build / artifact match.
// ---------------------------------------------------------------------------

/// Whether the build the artifact maps against matches the artifact under inspection.
/// Mirrors the symbolication and frame-mapping build-match vocabularies so dump, frame,
/// and crash surfaces read one truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactBuildMatch {
    /// Build identity matched field-for-field; an exact-build match.
    ExactBuildVerified,
    /// Only an approximate / candidate build was available, disclosed.
    ApproximateCandidate,
    /// A candidate build was found but rejected because it mismatched.
    MismatchedRejected,
    /// No build identity was available to verify against.
    NoCandidate,
}

impl ArtifactBuildMatch {
    /// All build-match classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ExactBuildVerified,
        Self::ApproximateCandidate,
        Self::MismatchedRejected,
        Self::NoCandidate,
    ];

    /// Stable snake_case token for this build-match class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildVerified => "exact_build_verified",
            Self::ApproximateCandidate => "approximate_candidate",
            Self::MismatchedRejected => "mismatched_rejected",
            Self::NoCandidate => "no_candidate",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactBuildVerified => "Exact build verified",
            Self::ApproximateCandidate => "Approximate build candidate",
            Self::MismatchedRejected => "Build mismatch (rejected)",
            Self::NoCandidate => "No build identity",
        }
    }

    /// Short pill fragment when the match is not an exact-build match.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::ExactBuildVerified => "build verified",
            Self::ApproximateCandidate => "approx build",
            Self::MismatchedRejected => "build mismatch",
            Self::NoCandidate => "no build id",
        }
    }

    /// Whether this state proves an exact-build match.
    pub const fn proves_exact_build(self) -> bool {
        matches!(self, Self::ExactBuildVerified)
    }

    /// Whether this state must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::ExactBuildVerified)
    }
}

// ---------------------------------------------------------------------------
// Artifact kind, entrypoint, and source class.
// ---------------------------------------------------------------------------

/// The kind of debug artifact a strip describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugArtifactKind {
    /// A native core file captured at process exit.
    CoreFile,
    /// A crash dump / minidump captured by a crash handler.
    CrashDump,
    /// An inspect-only debug session opened over a captured artifact.
    InspectOnlySession,
    /// A symbol artifact (PDB, dSYM, split/bundled DWARF) attached to resolve mappings.
    SymbolArtifact,
    /// A source map (JS/TS/CSS) attached to resolve generated-source mappings.
    SourceMap,
    /// A recorded replay capture reconstructed for inspect-only replay.
    ReplayCapture,
}

impl DebugArtifactKind {
    /// All artifact kinds, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::CoreFile,
        Self::CrashDump,
        Self::InspectOnlySession,
        Self::SymbolArtifact,
        Self::SourceMap,
        Self::ReplayCapture,
    ];

    /// Stable snake_case token for this artifact kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreFile => "core_file",
            Self::CrashDump => "crash_dump",
            Self::InspectOnlySession => "inspect_only_session",
            Self::SymbolArtifact => "symbol_artifact",
            Self::SourceMap => "source_map",
            Self::ReplayCapture => "replay_capture",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CoreFile => "Core file",
            Self::CrashDump => "Crash dump",
            Self::InspectOnlySession => "Inspect-only session",
            Self::SymbolArtifact => "Symbol artifact",
            Self::SourceMap => "Source map",
            Self::ReplayCapture => "Replay capture",
        }
    }

    /// Whether opening this artifact yields an inspect-only debug session rather than a
    /// supporting mapping input.
    pub const fn opens_inspect_only_session(self) -> bool {
        matches!(
            self,
            Self::CoreFile | Self::CrashDump | Self::InspectOnlySession | Self::ReplayCapture
        )
    }

    /// Whether this artifact is a symbol/source-map mapping input attached to a session
    /// rather than a session of its own. Such artifacts carry a debug format.
    pub const fn is_mapping_input(self) -> bool {
        matches!(self, Self::SymbolArtifact | Self::SourceMap)
    }
}

/// The distinct entrypoint a debug artifact strip was opened through. Core-file,
/// crash-dump, open-replay, and open-inspect-only are four distinct session entrypoints;
/// importing a symbol or source-map artifact is the fifth, non-session entrypoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugArtifactEntrypoint {
    /// Open a core file.
    OpenCoreFile,
    /// Open a crash dump / minidump.
    OpenCrashDump,
    /// Open a recorded replay capture.
    OpenReplay,
    /// Open an inspect-only session over a captured artifact.
    OpenInspectOnly,
    /// Import a symbol or source-map artifact to resolve mappings.
    ImportSymbolsOrSourceMap,
}

impl DebugArtifactEntrypoint {
    /// All entrypoints, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::OpenCoreFile,
        Self::OpenCrashDump,
        Self::OpenReplay,
        Self::OpenInspectOnly,
        Self::ImportSymbolsOrSourceMap,
    ];

    /// The four distinct session entrypoints that each open an inspect-only session.
    pub const SESSION_ENTRYPOINTS: [Self; 4] = [
        Self::OpenCoreFile,
        Self::OpenCrashDump,
        Self::OpenReplay,
        Self::OpenInspectOnly,
    ];

    /// Stable snake_case token for this entrypoint.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCoreFile => "open_core_file",
            Self::OpenCrashDump => "open_crash_dump",
            Self::OpenReplay => "open_replay",
            Self::OpenInspectOnly => "open_inspect_only",
            Self::ImportSymbolsOrSourceMap => "import_symbols_or_source_map",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenCoreFile => "Open core file",
            Self::OpenCrashDump => "Open crash dump",
            Self::OpenReplay => "Open replay",
            Self::OpenInspectOnly => "Open inspect-only",
            Self::ImportSymbolsOrSourceMap => "Import symbols / source map",
        }
    }

    /// Whether this entrypoint opens an inspect-only session.
    pub const fn opens_inspect_only_session(self) -> bool {
        matches!(
            self,
            Self::OpenCoreFile | Self::OpenCrashDump | Self::OpenReplay | Self::OpenInspectOnly
        )
    }

    /// Whether this entrypoint accepts the given artifact kind, so an entrypoint never
    /// silently mislabels what it opened.
    pub const fn accepts_kind(self, kind: DebugArtifactKind) -> bool {
        matches!(
            (self, kind),
            (Self::OpenCoreFile, DebugArtifactKind::CoreFile)
                | (Self::OpenCrashDump, DebugArtifactKind::CrashDump)
                | (Self::OpenReplay, DebugArtifactKind::ReplayCapture)
                | (Self::OpenInspectOnly, DebugArtifactKind::InspectOnlySession)
                | (
                    Self::ImportSymbolsOrSourceMap,
                    DebugArtifactKind::SymbolArtifact
                )
                | (Self::ImportSymbolsOrSourceMap, DebugArtifactKind::SourceMap)
        )
    }
}

/// Where a debug artifact came from: workspace source, a local artifact, a provider, a
/// mirror, or an explicit import. Mirrors the symbolication source-identity vocabulary so
/// a strip discloses a mirrored or imported origin rather than implying a local-trusted
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactSourceClass {
    /// The project's own workspace source or build output.
    WorkspaceSource,
    /// A local artifact store or local capture on this machine.
    LocalArtifact,
    /// Supplied by a managed provider / service.
    ProviderSupplied,
    /// Supplied by an enterprise or managed mirror.
    MirrorSupplied,
    /// An explicit user import / side-load with bounded trust.
    ImportedAttachment,
}

impl ArtifactSourceClass {
    /// All source classes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::WorkspaceSource,
        Self::LocalArtifact,
        Self::ProviderSupplied,
        Self::MirrorSupplied,
        Self::ImportedAttachment,
    ];

    /// Stable snake_case token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceSource => "workspace_source",
            Self::LocalArtifact => "local_artifact",
            Self::ProviderSupplied => "provider_supplied",
            Self::MirrorSupplied => "mirror_supplied",
            Self::ImportedAttachment => "imported_attachment",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceSource => "Workspace source",
            Self::LocalArtifact => "Local artifact",
            Self::ProviderSupplied => "Provider-supplied",
            Self::MirrorSupplied => "Mirror-supplied",
            Self::ImportedAttachment => "Imported attachment",
        }
    }

    /// Short pill fragment when the source must be disclosed.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::WorkspaceSource => "workspace",
            Self::LocalArtifact => "local",
            Self::ProviderSupplied => "provider",
            Self::MirrorSupplied => "mirror",
            Self::ImportedAttachment => "imported",
        }
    }

    /// Whether the source is mirrored.
    pub const fn is_mirrored(self) -> bool {
        matches!(self, Self::MirrorSupplied)
    }

    /// Whether the source is an explicit import / side-load.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedAttachment)
    }

    /// Whether the source is local to this machine or workspace.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::WorkspaceSource | Self::LocalArtifact)
    }

    /// Whether the provenance must be disclosed: a non-local source (provider, mirror, or
    /// import) never poses as a local-trusted one.
    pub const fn requires_provenance_disclosure(self) -> bool {
        !self.is_local()
    }
}

// ---------------------------------------------------------------------------
// Restore posture.
// ---------------------------------------------------------------------------

/// The honest posture of a reopened debug layout. A restored layout never implies live
/// continuity: it is gone, inspect-only, reconnect-required, or manually relaunchable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePosture {
    /// The prior process / session is gone; the layout is historical only.
    ProcessGone,
    /// Reopened as an inspect-only continuation (a dump or replay), with no live process.
    InspectOnlyContinuation,
    /// A live target may be reattachable, but only after an explicit reconnect.
    ReconnectRequired,
    /// No reconnect is possible; the user must manually relaunch to get a live session.
    ManuallyRelaunchable,
}

impl RestorePosture {
    /// All restore postures, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ProcessGone,
        Self::InspectOnlyContinuation,
        Self::ReconnectRequired,
        Self::ManuallyRelaunchable,
    ];

    /// Stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProcessGone => "process_gone",
            Self::InspectOnlyContinuation => "inspect_only_continuation",
            Self::ReconnectRequired => "reconnect_required",
            Self::ManuallyRelaunchable => "manually_relaunchable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProcessGone => "Prior process gone",
            Self::InspectOnlyContinuation => "Inspect-only continuation",
            Self::ReconnectRequired => "Reconnect required",
            Self::ManuallyRelaunchable => "Manually relaunchable",
        }
    }

    /// Whether this posture implies live target continuity. Always false: a restored
    /// layout never reacquires a live process by being reopened.
    pub const fn implies_live_continuity(self) -> bool {
        false
    }

    /// Whether the prior process is gone.
    pub const fn prior_process_gone(self) -> bool {
        matches!(self, Self::ProcessGone)
    }

    /// Whether this posture is an inspect-only continuation.
    pub const fn is_inspect_only(self) -> bool {
        matches!(self, Self::InspectOnlyContinuation)
    }

    /// Whether the user must take an explicit action (reconnect or relaunch) to regain a
    /// live session.
    pub const fn requires_explicit_action(self) -> bool {
        matches!(self, Self::ReconnectRequired | Self::ManuallyRelaunchable)
    }

    /// Whether an explicit reconnect is offered.
    pub const fn reconnect_available(self) -> bool {
        matches!(self, Self::ReconnectRequired)
    }

    /// Whether a manual relaunch is offered.
    pub const fn relaunch_available(self) -> bool {
        matches!(self, Self::ManuallyRelaunchable)
    }
}

// ---------------------------------------------------------------------------
// Pills.
// ---------------------------------------------------------------------------

/// The single canonical pill every surface renders for a debug artifact strip — one
/// mapping fidelity, one build-match outcome, and one source class, with every disclosure
/// flag derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugArtifactPill {
    /// The mapping fidelity.
    pub fidelity: DebugMappingFidelity,
    /// Stable token for the fidelity.
    pub fidelity_token: String,
    /// The build-match outcome.
    pub build_match: ArtifactBuildMatch,
    /// Stable token for the build-match outcome.
    pub build_match_token: String,
    /// The source class.
    pub source_class: ArtifactSourceClass,
    /// Stable token for the source class.
    pub source_class_token: String,
    /// One reviewable pill label combining fidelity, source disclosure, build match, and
    /// inspect-only posture.
    pub label: String,
    /// Whether the strip may render an unqualified exact source link — true only when the
    /// mapping is exact and the build identity proves an exact build.
    pub shows_exact_source_link: bool,
    /// Whether the strip must render with a visible mapping caveat.
    pub requires_disclosure: bool,
    /// Whether the strip offers a source line a reader can navigate to.
    pub allows_source_navigation: bool,
    /// Whether the source is mirrored.
    pub is_mirrored_source: bool,
    /// Whether the source is an explicit import / side-load.
    pub is_imported_source: bool,
    /// Whether the artifact opens an inspect-only session.
    pub is_inspect_only: bool,
}

impl DebugArtifactPill {
    /// Whether a strip may render the unqualified exact source link: only an exact mapping
    /// backed by an exact-build match. This is the guardrail that keeps a precise source
    /// link from hiding an approximate, symbol-only, unresolved, imported, or
    /// build-mismatched reality.
    pub const fn derive_shows_exact_source_link(
        fidelity: DebugMappingFidelity,
        build_match: ArtifactBuildMatch,
    ) -> bool {
        fidelity.preserves_exact_source() && build_match.proves_exact_build()
    }

    /// Builds the canonical pill for a strip, deriving every flag and the label so the
    /// pill cannot disagree with itself.
    pub fn derive(
        fidelity: DebugMappingFidelity,
        build_match: ArtifactBuildMatch,
        source_class: ArtifactSourceClass,
        opens_inspect_only: bool,
    ) -> Self {
        let shows_exact_source_link = Self::derive_shows_exact_source_link(fidelity, build_match);
        let mut label = fidelity.label().to_owned();
        if source_class.requires_provenance_disclosure() {
            label.push_str(" · ");
            label.push_str(source_class.short_label());
        }
        if build_match.requires_disclosure() && !fidelity.is_build_mismatch() {
            label.push_str(" · ");
            label.push_str(build_match.short_label());
        }
        if opens_inspect_only {
            label.push_str(" · inspect-only");
        }
        Self {
            fidelity,
            fidelity_token: fidelity.as_str().to_owned(),
            build_match,
            build_match_token: build_match.as_str().to_owned(),
            source_class,
            source_class_token: source_class.as_str().to_owned(),
            label,
            shows_exact_source_link,
            requires_disclosure: !shows_exact_source_link,
            allows_source_navigation: fidelity.allows_source_navigation(),
            is_mirrored_source: source_class.is_mirrored(),
            is_imported_source: source_class.is_imported(),
            is_inspect_only: opens_inspect_only,
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        fidelity: DebugMappingFidelity,
        build_match: ArtifactBuildMatch,
        source_class: ArtifactSourceClass,
        opens_inspect_only: bool,
    ) -> bool {
        *self == Self::derive(fidelity, build_match, source_class, opens_inspect_only)
    }
}

/// The single canonical pill every surface renders for a restored debug layout — one
/// posture, one mapping fidelity, with every honesty flag derived. The restore can never
/// imply live continuity or process authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePill {
    /// The restore posture.
    pub posture: RestorePosture,
    /// Stable token for the posture.
    pub posture_token: String,
    /// The mapping fidelity the restored layout currently has.
    pub mapping_fidelity: DebugMappingFidelity,
    /// Stable token for the mapping fidelity.
    pub mapping_fidelity_token: String,
    /// One reviewable pill label combining posture, mapping fidelity, and any required
    /// action.
    pub label: String,
    /// Whether the restored layout implies live target continuity — always false.
    pub implies_live_continuity: bool,
    /// Whether the restored layout implies reacquired process authority — always false.
    pub implies_process_authority: bool,
    /// Whether the restored layout may render an exact-build mapping — true only when the
    /// mapping is exact and the build is still verified.
    pub implies_exact_build_mapping: bool,
    /// Whether the user must take an explicit action to regain a live session.
    pub requires_explicit_action: bool,
    /// Whether an explicit reconnect is offered.
    pub reconnect_available: bool,
    /// Whether a manual relaunch is offered.
    pub relaunch_available: bool,
    /// Whether the restore must render with a visible non-live caveat — always true.
    pub requires_disclosure: bool,
}

impl RestorePill {
    /// Builds the canonical pill for a restored layout, deriving every flag so the pill
    /// cannot disagree with itself.
    pub fn derive(
        posture: RestorePosture,
        mapping_fidelity: DebugMappingFidelity,
        exact_build_still_verified: bool,
    ) -> Self {
        let implies_exact_build_mapping =
            mapping_fidelity.preserves_exact_source() && exact_build_still_verified;
        let mut label = posture.label().to_owned();
        label.push_str(" · mapping ");
        label.push_str(mapping_fidelity.short_label());
        if posture.reconnect_available() {
            label.push_str(" · reconnect to resume");
        } else if posture.relaunch_available() {
            label.push_str(" · relaunch to resume");
        }
        Self {
            posture,
            posture_token: posture.as_str().to_owned(),
            mapping_fidelity,
            mapping_fidelity_token: mapping_fidelity.as_str().to_owned(),
            label,
            implies_live_continuity: posture.implies_live_continuity(),
            implies_process_authority: false,
            implies_exact_build_mapping,
            requires_explicit_action: posture.requires_explicit_action(),
            reconnect_available: posture.reconnect_available(),
            relaunch_available: posture.relaunch_available(),
            requires_disclosure: true,
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        posture: RestorePosture,
        mapping_fidelity: DebugMappingFidelity,
        exact_build_still_verified: bool,
    ) -> bool {
        *self == Self::derive(posture, mapping_fidelity, exact_build_still_verified)
    }
}

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// A typed debug artifact strip: the canonical record every debugger, notebook, profiler,
/// incident, support, and AI surface reads to show one opened core file, crash dump,
/// inspect-only session, symbol artifact, source map, or replay capture, which build it
/// belongs to, how trustworthy its mapping is, and how it was opened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugArtifactStrip {
    /// Stable, namespaced strip id.
    pub strip_id: String,
    /// The artifact kind.
    pub artifact_kind: DebugArtifactKind,
    /// Stable token for the artifact kind.
    pub artifact_kind_token: String,
    /// The distinct entrypoint the strip was opened through.
    pub entrypoint: DebugArtifactEntrypoint,
    /// Stable token for the entrypoint.
    pub entrypoint_token: String,
    /// Whether the entrypoint opens an inspect-only session.
    pub opens_inspect_only_session: bool,
    /// Opaque digest of the build id this artifact belongs to, never a raw path or URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// Export-safe artifact ref for the opened artifact, when one is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    /// The native symbol format or source-map class, present only for mapping inputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_format: Option<DebugFormatClass>,
    /// Timestamp the artifact was captured / opened at.
    pub captured_as_of: String,
    /// The canonical fidelity + build-match + source pill every surface renders.
    pub pill: DebugArtifactPill,
    /// The proof packet that keeps this strip current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the strip.
    pub summary: String,
}

impl DebugArtifactStrip {
    /// Builds a debug artifact strip, deriving every computed token, honesty flag, and the
    /// pill from the typed inputs so the record cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        strip_id: impl Into<String>,
        artifact_kind: DebugArtifactKind,
        entrypoint: DebugArtifactEntrypoint,
        build_id: Option<&str>,
        artifact_ref: Option<&str>,
        debug_format: Option<DebugFormatClass>,
        captured_as_of: impl Into<String>,
        fidelity: DebugMappingFidelity,
        build_match: ArtifactBuildMatch,
        source_class: ArtifactSourceClass,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        let opens_inspect_only_session = entrypoint.opens_inspect_only_session();
        Self {
            strip_id: strip_id.into(),
            artifact_kind,
            artifact_kind_token: artifact_kind.as_str().to_owned(),
            entrypoint,
            entrypoint_token: entrypoint.as_str().to_owned(),
            opens_inspect_only_session,
            build_id: build_id.map(str::to_owned),
            artifact_ref: artifact_ref.map(str::to_owned),
            debug_format,
            captured_as_of: captured_as_of.into(),
            pill: DebugArtifactPill::derive(
                fidelity,
                build_match,
                source_class,
                opens_inspect_only_session,
            ),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The mapping fidelity from the pill.
    pub const fn fidelity(&self) -> DebugMappingFidelity {
        self.pill.fidelity
    }

    /// The build-match outcome from the pill.
    pub const fn build_match(&self) -> ArtifactBuildMatch {
        self.pill.build_match
    }

    /// The source class from the pill.
    pub const fn source_class(&self) -> ArtifactSourceClass {
        self.pill.source_class
    }
}

/// A typed restored-layout record: the canonical record every surface reads when a debug
/// layout is reopened, to say whether the prior process / session is gone, inspect-only,
/// reconnect-required, or manually relaunchable — never implying live continuity or
/// reacquired process authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoredLayoutRecord {
    /// Stable, namespaced restored-layout id.
    pub layout_id: String,
    /// Stable strip id of the artifact strip this layout reopened.
    pub restored_strip_ref: String,
    /// Opaque ref for the prior session that was reopened.
    pub prior_session_ref: String,
    /// Timestamp the layout was restored at.
    pub restored_as_of: String,
    /// Whether the artifact's exact build is still verified after restore.
    pub exact_build_still_verified: bool,
    /// The canonical restore pill every surface renders.
    pub pill: RestorePill,
    /// The proof packet that keeps this restored layout current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the restore.
    pub summary: String,
}

impl RestoredLayoutRecord {
    /// Builds a restored-layout record, deriving the pill from the typed inputs so the
    /// record cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        layout_id: impl Into<String>,
        restored_strip_ref: impl Into<String>,
        prior_session_ref: impl Into<String>,
        restored_as_of: impl Into<String>,
        posture: RestorePosture,
        mapping_fidelity: DebugMappingFidelity,
        exact_build_still_verified: bool,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            layout_id: layout_id.into(),
            restored_strip_ref: restored_strip_ref.into(),
            prior_session_ref: prior_session_ref.into(),
            restored_as_of: restored_as_of.into(),
            exact_build_still_verified,
            pill: RestorePill::derive(posture, mapping_fidelity, exact_build_still_verified),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The restore posture from the pill.
    pub const fn posture(&self) -> RestorePosture {
        self.pill.posture
    }

    /// The mapping fidelity from the pill.
    pub const fn fidelity(&self) -> DebugMappingFidelity {
        self.pill.mapping_fidelity
    }
}

// ---------------------------------------------------------------------------
// Invariants and set.
// ---------------------------------------------------------------------------

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DumpRestoreInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 dump/mapping/restore set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DumpMappingRestoreSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_dump_mapping_restore_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the set current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The boundary schemas this set binds as truth sources.
    pub source_schema_refs: Vec<String>,
    /// The crate modules that already produce this truth.
    pub producer_refs: Vec<String>,
    /// The surfaces that consume the artifact strips and restore records.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The debug artifact strips.
    pub artifacts: Vec<DebugArtifactStrip>,
    /// The restored-layout records.
    pub restored_layouts: Vec<RestoredLayoutRecord>,
    /// The computed invariants.
    pub invariants: Vec<DumpRestoreInvariant>,
    /// Whether raw dump/source/value bodies are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the dump/mapping/restore set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumpMappingRestoreSetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for DumpMappingRestoreSetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m5 dump/mapping/restore set invalid: {}", self.reason)
    }
}

impl std::error::Error for DumpMappingRestoreSetValidationError {}

impl DumpMappingRestoreSet {
    /// Returns the artifact strip with the given id, if present.
    pub fn artifact(&self, strip_id: &str) -> Option<&DebugArtifactStrip> {
        self.artifacts.iter().find(|a| a.strip_id == strip_id)
    }

    /// Returns the restored layout with the given id, if present.
    pub fn restored_layout(&self, layout_id: &str) -> Option<&RestoredLayoutRecord> {
        self.restored_layouts
            .iter()
            .find(|r| r.layout_id == layout_id)
    }

    /// Returns the first artifact strip in the given fidelity, if present.
    pub fn artifact_in_fidelity(
        &self,
        fidelity: DebugMappingFidelity,
    ) -> Option<&DebugArtifactStrip> {
        self.artifacts.iter().find(|a| a.fidelity() == fidelity)
    }

    /// Returns the first restored layout in the given posture, if present.
    pub fn restored_layout_in_posture(
        &self,
        posture: RestorePosture,
    ) -> Option<&RestoredLayoutRecord> {
        self.restored_layouts
            .iter()
            .find(|r| r.posture() == posture)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are excluded
    /// and every ref is a repo-relative object ref, never a URL, host, credential, or
    /// absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_set = self
            .source_schema_refs
            .iter()
            .map(String::as_str)
            .chain(self.producer_refs.iter().map(String::as_str))
            .chain(std::iter::once(self.freeze_gate_ref.as_str()));
        let from_artifacts = self.artifacts.iter().flat_map(|a| {
            std::iter::once(a.proof_packet_ref.as_str()).chain(a.artifact_ref.as_deref())
        });
        let from_restores = self
            .restored_layouts
            .iter()
            .map(|r| r.proof_packet_ref.as_str());
        from_set.chain(from_artifacts).chain(from_restores)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns a [`DumpMappingRestoreSetValidationError`] when an identifier, a ref, a
    /// computed flag, a pill, an entrypoint/kind rule, a fidelity rule, a restore rule, or
    /// an invariant is inconsistent.
    pub fn validate(&self) -> Result<(), DumpMappingRestoreSetValidationError> {
        let fail = |reason: String| Err(DumpMappingRestoreSetValidationError { reason });

        if self.record_kind != M5_DUMP_MAPPING_RESTORE_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_DUMP_MAPPING_RESTORE_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_dump_mapping_restore_schema_version != M5_DUMP_MAPPING_RESTORE_SCHEMA_VERSION {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.artifacts.is_empty() {
            return fail("no artifacts".to_owned());
        }
        if self.restored_layouts.is_empty() {
            return fail("no restored layouts".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.artifacts.iter().map(|a| a.strip_id.as_str())) {
            return fail("artifact strip ids are not unique".to_owned());
        }
        if !all_unique(self.restored_layouts.iter().map(|r| r.layout_id.as_str())) {
            return fail("restored layout ids are not unique".to_owned());
        }

        // The full vocabularies are materialized.
        for fidelity in DebugMappingFidelity::ALL {
            if self.artifact_in_fidelity(fidelity).is_none() {
                return fail(format!(
                    "mapping fidelity {} is not materialized",
                    fidelity.as_str()
                ));
            }
        }
        for kind in DebugArtifactKind::ALL {
            if !self.artifacts.iter().any(|a| a.artifact_kind == kind) {
                return fail(format!(
                    "artifact kind {} is not materialized",
                    kind.as_str()
                ));
            }
        }
        for entrypoint in DebugArtifactEntrypoint::ALL {
            if !self.artifacts.iter().any(|a| a.entrypoint == entrypoint) {
                return fail(format!(
                    "entrypoint {} is not materialized",
                    entrypoint.as_str()
                ));
            }
        }
        for source in ArtifactSourceClass::ALL {
            if !self.artifacts.iter().any(|a| a.source_class() == source) {
                return fail(format!(
                    "source class {} is not materialized",
                    source.as_str()
                ));
            }
        }
        for posture in RestorePosture::ALL {
            if self.restored_layout_in_posture(posture).is_none() {
                return fail(format!(
                    "restore posture {} is not materialized",
                    posture.as_str()
                ));
            }
        }

        // Per-strip structural floor and cross-cutting rules.
        for a in &self.artifacts {
            validate_artifact(a)
                .map_err(|reason| DumpMappingRestoreSetValidationError { reason })?;
        }

        // The four distinct session entrypoints are each present and each open an
        // inspect-only session.
        for entrypoint in DebugArtifactEntrypoint::SESSION_ENTRYPOINTS {
            let matching: Vec<&DebugArtifactStrip> = self
                .artifacts
                .iter()
                .filter(|a| a.entrypoint == entrypoint)
                .collect();
            if matching.is_empty() {
                return fail(format!(
                    "session entrypoint {} is not present",
                    entrypoint.as_str()
                ));
            }
            if matching.iter().any(|a| !a.opens_inspect_only_session) {
                return fail(format!(
                    "session entrypoint {} must open an inspect-only session",
                    entrypoint.as_str()
                ));
            }
        }

        // Per-restore structural floor and cross-cutting rules, plus cross-reference.
        for r in &self.restored_layouts {
            validate_restore(r)
                .map_err(|reason| DumpMappingRestoreSetValidationError { reason })?;
            if self.artifact(&r.restored_strip_ref).is_none() {
                return fail(format!(
                    "restored layout {} references unknown strip {}",
                    r.layout_id, r.restored_strip_ref
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("set is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn validate_artifact(a: &DebugArtifactStrip) -> Result<(), String> {
    if a.strip_id.is_empty() {
        return Err("artifact strip has empty id".to_owned());
    }
    if a.captured_as_of.is_empty() {
        return Err(format!("strip {} has empty capture time", a.strip_id));
    }
    if a.proof_packet_ref.is_empty() {
        return Err(format!("strip {} has no proof packet", a.strip_id));
    }
    if !artifact_flags_consistent(a) {
        return Err(format!(
            "strip {} computed flags or pill disagree with its enums",
            a.strip_id
        ));
    }
    // The entrypoint accepts the artifact kind.
    if !a.entrypoint.accepts_kind(a.artifact_kind) {
        return Err(format!(
            "strip {} entrypoint {} does not accept kind {}",
            a.strip_id,
            a.entrypoint.as_str(),
            a.artifact_kind.as_str()
        ));
    }
    // The inspect-only echo agrees with the entrypoint and the pill.
    if a.opens_inspect_only_session != a.entrypoint.opens_inspect_only_session()
        || a.pill.is_inspect_only != a.opens_inspect_only_session
    {
        return Err(format!(
            "strip {} inspect-only flags disagree with its entrypoint",
            a.strip_id
        ));
    }
    // A debug format is present exactly for mapping inputs (symbol/source-map).
    if a.debug_format.is_some() != a.artifact_kind.is_mapping_input() {
        return Err(format!(
            "strip {} debug format presence must match a mapping-input kind",
            a.strip_id
        ));
    }
    // Build / artifact identity is always present.
    if a.build_id.is_none() && a.artifact_ref.is_none() {
        return Err(format!(
            "strip {} has neither a build id nor an artifact ref",
            a.strip_id
        ));
    }
    // Imported fidelity implies an imported source class.
    if a.fidelity().is_imported() && !a.source_class().is_imported() {
        return Err(format!(
            "strip {} claims imported fidelity without an imported source",
            a.strip_id
        ));
    }
    // Build-mismatch fidelity implies a rejected build match.
    if a.fidelity().is_build_mismatch() && a.build_match() != ArtifactBuildMatch::MismatchedRejected
    {
        return Err(format!(
            "strip {} claims build-mismatch fidelity without a rejected build match",
            a.strip_id
        ));
    }
    // An exact source link implies an exact mapping and an exact-build match.
    if a.pill.shows_exact_source_link
        && !(a.fidelity().preserves_exact_source() && a.build_match().proves_exact_build())
    {
        return Err(format!(
            "strip {} shows an exact source link without an exact mapping and build",
            a.strip_id
        ));
    }
    Ok(())
}

fn validate_restore(r: &RestoredLayoutRecord) -> Result<(), String> {
    if r.layout_id.is_empty() {
        return Err("restored layout has empty id".to_owned());
    }
    if r.restored_strip_ref.is_empty() {
        return Err(format!("restore {} has empty strip ref", r.layout_id));
    }
    if r.prior_session_ref.is_empty() {
        return Err(format!(
            "restore {} has empty prior session ref",
            r.layout_id
        ));
    }
    if r.restored_as_of.is_empty() {
        return Err(format!("restore {} has empty restore time", r.layout_id));
    }
    if r.proof_packet_ref.is_empty() {
        return Err(format!("restore {} has no proof packet", r.layout_id));
    }
    if !restore_flags_consistent(r) {
        return Err(format!(
            "restore {} computed flags or pill disagree with its enums",
            r.layout_id
        ));
    }
    // A restored layout never implies live continuity or process authority.
    if r.pill.implies_live_continuity || r.pill.implies_process_authority {
        return Err(format!(
            "restore {} implies live continuity or process authority",
            r.layout_id
        ));
    }
    // An exact-build mapping is shown only when still verified.
    let expected_exact = r.fidelity().preserves_exact_source() && r.exact_build_still_verified;
    if r.pill.implies_exact_build_mapping != expected_exact {
        return Err(format!(
            "restore {} exact-build mapping flag disagrees with its evidence",
            r.layout_id
        ));
    }
    Ok(())
}

fn artifact_flags_consistent(a: &DebugArtifactStrip) -> bool {
    a.artifact_kind_token == a.artifact_kind.as_str()
        && a.entrypoint_token == a.entrypoint.as_str()
        && a.pill.fidelity_token == a.pill.fidelity.as_str()
        && a.pill.build_match_token == a.pill.build_match.as_str()
        && a.pill.source_class_token == a.pill.source_class.as_str()
        && a.pill.matches_derivation(
            a.pill.fidelity,
            a.pill.build_match,
            a.pill.source_class,
            a.opens_inspect_only_session,
        )
}

fn restore_flags_consistent(r: &RestoredLayoutRecord) -> bool {
    r.pill.posture_token == r.pill.posture.as_str()
        && r.pill.mapping_fidelity_token == r.pill.mapping_fidelity.as_str()
        && r.pill.matches_derivation(
            r.pill.posture,
            r.pill.mapping_fidelity,
            r.exact_build_still_verified,
        )
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque `aureline://`
/// handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical M5 dump/mapping/restore set.
///
/// Deterministic: the same bytes every call. Each invariant's `holds` flag is computed
/// from the built records, so an inconsistent edit flips an invariant rather than silently
/// passing.
pub fn m5_dump_mapping_restore_set() -> DumpMappingRestoreSet {
    let artifacts = build_artifacts();
    let restored_layouts = build_restored_layouts();
    let invariants = compute_invariants(&artifacts, &restored_layouts);

    DumpMappingRestoreSet {
        record_kind: M5_DUMP_MAPPING_RESTORE_RECORD_KIND.to_owned(),
        m5_dump_mapping_restore_schema_version: M5_DUMP_MAPPING_RESTORE_SCHEMA_VERSION,
        schema_ref: M5_DUMP_MAPPING_RESTORE_SCHEMA_REF.to_owned(),
        set_id: M5_DUMP_MAPPING_RESTORE_SET_ID.to_owned(),
        as_of: M5_DUMP_MAPPING_RESTORE_AS_OF.to_owned(),
        freeze_gate_ref: M5_DUMP_MAPPING_RESTORE_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set of M5 dump/core-file/source-map/symbol artifact strips and \
                  restored-layout records. Every strip carries one pill that pins one shared mapping \
                  fidelity (exact, approximate, symbol-only, unresolved, imported, mismatched-build), \
                  one build-match outcome, and one source class (workspace, local, provider, mirror, \
                  imported), so a precise source link renders only for an exact mapping backed by an \
                  exact-build match, an imported or build-mismatched strip never renders it, and \
                  core-file/crash-dump/open-replay/open-inspect-only entrypoints stay distinct from \
                  importing a symbol or source-map artifact. Every restored layout carries one pill \
                  that names whether the prior process/session is gone, inspect-only, \
                  reconnect-required, or manually relaunchable, never implies live continuity or \
                  reacquired process authority, and shows an exact-build mapping only when it is \
                  still verified."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/debug/symbolication_contract.schema.json",
            "schemas/debug/m5_debug_contracts.schema.json",
            "schemas/debug/m5_frame_variable_snapshots.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs",
            "crates/aureline-debug/src/symbolication/mod.rs",
            "crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs",
        ]),
        consumer_surfaces: vec![
            DebugConsumer::CoreDebugger,
            DebugConsumer::NotebookDebug,
            DebugConsumer::Profiler,
            DebugConsumer::IncidentReview,
            DebugConsumer::SupportExport,
            DebugConsumer::AiContext,
            DebugConsumer::ReviewWorkspace,
            DebugConsumer::CliHeadless,
        ],
        artifacts,
        restored_layouts,
        invariants,
        raw_payload_excluded: true,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

const CRASH_MINIDUMP_PROOF: &str =
    "fixtures/debug/mapping_cases/crash_minidump_resolved_with_siblings.json";
const CORE_DUMP_PROOF: &str =
    "fixtures/debug/mapping_cases/crash_core_dump_pending_upload_consent.json";
const SYMBOL_PDB_PROOF: &str = "fixtures/debug/mapping_cases/native_symbol_pdb_resolved.json";
const SYMBOL_DSYM_MISMATCH_PROOF: &str =
    "fixtures/debug/mapping_cases/native_symbol_dsym_mismatch_build_id.json";
const SYMBOL_DWARF_MISSING_PROOF: &str =
    "fixtures/debug/mapping_cases/native_symbol_dwarf_split_symbols_missing.json";
const SOURCE_MAP_STALE_PROOF: &str =
    "fixtures/debug/mapping_cases/source_map_js_stale_mapping.json";
const SOURCE_MAP_OPENAPI_PROOF: &str =
    "fixtures/debug/mapping_cases/generated_source_openapi_resolved.json";
const SOURCE_MAP_UNKNOWN_PROOF: &str =
    "fixtures/debug/mapping_cases/generated_source_spec_unknown.json";
const SYMBOL_ONLY_REPORT_PROOF: &str = "fixtures/debug/symbolication/symbol_only_report.json";
const REPLAY_PROOF: &str = "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json";

fn build_artifacts() -> Vec<DebugArtifactStrip> {
    use ArtifactBuildMatch::*;
    use ArtifactSourceClass::*;
    use DebugArtifactEntrypoint::*;
    use DebugArtifactKind::*;
    use DebugMappingFidelity::*;

    vec![
        // 1. Crash dump resolved exactly against a verified build: the only kind of strip
        //    that renders the unqualified precise source link.
        DebugArtifactStrip::build(
            "debug.artifact:crash_dump_exact:0001",
            CrashDump,
            OpenCrashDump,
            Some("build:digest:aa11bb"),
            Some("aureline://artifact/crash/minidump-0001"),
            None,
            "2026-06-26T09:14:00Z",
            Exact,
            ExactBuildVerified,
            LocalArtifact,
            CRASH_MINIDUMP_PROOF,
            "Crash dump resolved exactly to source against a verified build: the only strip \
             that renders the unqualified precise source link, opened as inspect-only.",
        ),
        // 2. Core file mapped only approximately against an approximate build candidate.
        DebugArtifactStrip::build(
            "debug.artifact:core_file_approx:0002",
            CoreFile,
            OpenCoreFile,
            Some("build:digest:aa11bb"),
            Some("aureline://artifact/core/core-0002"),
            None,
            "2026-06-26T09:02:00Z",
            Approximate,
            ApproximateCandidate,
            WorkspaceSource,
            CORE_DUMP_PROOF,
            "Core file mapped approximately against an approximate build candidate: navigable, \
             but disclosed as approximate rather than drawn exact.",
        ),
        // 3. Replay capture reopened exactly read-only: an inspect-only session entrypoint
        //    distinct from core-file and crash-dump.
        DebugArtifactStrip::build(
            "debug.artifact:replay_exact:0003",
            ReplayCapture,
            OpenReplay,
            Some("build:digest:aa11bb"),
            Some("aureline://artifact/replay/task-run-42"),
            None,
            "2026-06-26T08:40:00Z",
            Exact,
            ExactBuildVerified,
            LocalArtifact,
            REPLAY_PROOF,
            "Recorded replay capture reopened read-only against a verified build: a distinct \
             open-replay entrypoint, always inspect-only.",
        ),
        // 4. Inspect-only session over a symbol-only report: a distinct open-inspect-only
        //    entrypoint resolving symbols without authoritative source lines.
        DebugArtifactStrip::build(
            "debug.artifact:inspect_only_symbol_only:0004",
            InspectOnlySession,
            OpenInspectOnly,
            Some("build:digest:cc33dd"),
            Some("aureline://artifact/inspect/session-0004"),
            None,
            "2026-06-26T08:20:00Z",
            SymbolOnly,
            NoCandidate,
            LocalArtifact,
            SYMBOL_ONLY_REPORT_PROOF,
            "Inspect-only session resolving a symbol name only, with no authoritative source \
             lines: a distinct open-inspect-only entrypoint, never drawn as a precise link.",
        ),
        // 5. PDB symbol artifact resolved exactly: a mapping input (not a session) imported
        //    through the import entrypoint, carrying its debug format.
        DebugArtifactStrip::build(
            "debug.artifact:symbol_pdb_exact:0005",
            SymbolArtifact,
            ImportSymbolsOrSourceMap,
            Some("build:digest:aa11bb"),
            Some("aureline://artifact/symbols/pdb-0005"),
            Some(DebugFormatClass::Pdb),
            "2026-06-26T07:55:00Z",
            Exact,
            ExactBuildVerified,
            LocalArtifact,
            SYMBOL_PDB_PROOF,
            "PDB symbol artifact resolved exactly against a verified build: a mapping input \
             imported to back exact source links, not a session of its own.",
        ),
        // 6. dSYM symbol artifact whose build id does not match: a build-mismatch mapping
        //    that is rejected and never drawn exact.
        DebugArtifactStrip::build(
            "debug.artifact:symbol_dsym_mismatch:0006",
            SymbolArtifact,
            ImportSymbolsOrSourceMap,
            Some("build:digest:ee77ff"),
            Some("aureline://artifact/symbols/dsym-0006"),
            Some(DebugFormatClass::Dsym),
            "2026-06-26T07:40:00Z",
            MismatchedBuild,
            MismatchedRejected,
            LocalArtifact,
            SYMBOL_DSYM_MISMATCH_PROOF,
            "dSYM symbol artifact whose build id does not match the binary under inspection: \
             a build-mismatch mapping, rejected and never rendered as a precise link.",
        ),
        // 7. Provider-supplied DWARF symbols with siblings missing: symbol-only, with the
        //    provider provenance disclosed.
        DebugArtifactStrip::build(
            "debug.artifact:symbol_dwarf_provider:0007",
            SymbolArtifact,
            ImportSymbolsOrSourceMap,
            Some("build:digest:99ff00"),
            Some("aureline://artifact/symbols/dwarf-0007"),
            Some(DebugFormatClass::Dwarf),
            "2026-06-26T07:25:00Z",
            SymbolOnly,
            NoCandidate,
            ProviderSupplied,
            SYMBOL_DWARF_MISSING_PROOF,
            "Provider-supplied DWARF symbols with split siblings missing: symbol-only, with \
             the provider provenance disclosed rather than posing as a local-trusted store.",
        ),
        // 8. Mirror-supplied JavaScript source map with a stale mapping: approximate, with
        //    the mirror provenance disclosed.
        DebugArtifactStrip::build(
            "debug.artifact:source_map_mirror_stale:0008",
            SourceMap,
            ImportSymbolsOrSourceMap,
            Some("build:digest:cc33dd"),
            Some("aureline://artifact/source-maps/js-0008"),
            Some(DebugFormatClass::JavaScriptSourceMap),
            "2026-06-26T07:10:00Z",
            Approximate,
            ApproximateCandidate,
            MirrorSupplied,
            SOURCE_MAP_STALE_PROOF,
            "Mirror-supplied JavaScript source map with a stale mapping: approximate, with the \
             mirror provenance disclosed so it never poses as a local-trusted source.",
        ),
        // 9. Side-loaded TypeScript source map: imported fidelity sourced from an explicit
        //    import with bounded trust.
        DebugArtifactStrip::build(
            "debug.artifact:source_map_imported:0009",
            SourceMap,
            ImportSymbolsOrSourceMap,
            Some("build:digest:cc33dd"),
            Some("aureline://artifact/source-maps/ts-0009"),
            Some(DebugFormatClass::TypeScriptSourceMap),
            "2026-06-26T06:55:00Z",
            Imported,
            ApproximateCandidate,
            ImportedAttachment,
            SOURCE_MAP_OPENAPI_PROOF,
            "Side-loaded TypeScript source map resolved from an explicit import: an imported \
             mapping with bounded trust, never escalated to an exact link.",
        ),
        // 10. Source map for a generated source whose spec is unknown: an explicit
        //     unresolved mapping rather than a guessed location.
        DebugArtifactStrip::build(
            "debug.artifact:source_map_unresolved:0010",
            SourceMap,
            ImportSymbolsOrSourceMap,
            Some("build:digest:cc33dd"),
            Some("aureline://artifact/source-maps/spec-0010"),
            Some(DebugFormatClass::CssSourceMap),
            "2026-06-26T06:40:00Z",
            Unresolved,
            NoCandidate,
            LocalArtifact,
            SOURCE_MAP_UNKNOWN_PROOF,
            "Source map for a generated source whose spec could not be resolved: an explicit \
             unresolved mapping, never shown as a guessed source location.",
        ),
    ]
}

fn build_restored_layouts() -> Vec<RestoredLayoutRecord> {
    use DebugMappingFidelity::*;
    use RestorePosture::*;

    vec![
        // 1. The prior live launch is gone; the reopened layout is historical only.
        RestoredLayoutRecord::build(
            "debug.restore:process_gone:0001",
            "debug.artifact:inspect_only_symbol_only:0004",
            "debug.session:prior-launch:0001",
            M5_DUMP_MAPPING_RESTORE_AS_OF,
            ProcessGone,
            Unresolved,
            false,
            SYMBOL_ONLY_REPORT_PROOF,
            "Reopened layout whose prior live launch is gone: historical only, with an \
             unresolved mapping and no live continuity or process authority.",
        ),
        // 2. A crash dump reopened as an inspect-only continuation: exact mapping is still
        //    verified, but the restore never implies a live process.
        RestoredLayoutRecord::build(
            "debug.restore:inspect_only_continuation:0002",
            "debug.artifact:crash_dump_exact:0001",
            "debug.session:prior-crash-dump:0002",
            M5_DUMP_MAPPING_RESTORE_AS_OF,
            InspectOnlyContinuation,
            Exact,
            true,
            CRASH_MINIDUMP_PROOF,
            "Crash dump reopened as an inspect-only continuation: its exact-build mapping is \
             still verified, yet the layout never implies live continuity or process authority.",
        ),
        // 3. A prior attach session that can be resumed only by an explicit reconnect; the
        //    restored mapping is imported with bounded trust.
        RestoredLayoutRecord::build(
            "debug.restore:reconnect_required:0003",
            "debug.artifact:replay_exact:0003",
            "debug.session:prior-attach:0003",
            M5_DUMP_MAPPING_RESTORE_AS_OF,
            ReconnectRequired,
            Imported,
            false,
            REPLAY_PROOF,
            "Prior attach session reopened reconnect-required: a live target may be \
             reattachable only after an explicit reconnect, with an imported bounded-trust mapping.",
        ),
        // 4. A prior launch against a build that has since changed: must be manually
        //    relaunched, and the restored mapping is a build mismatch.
        RestoredLayoutRecord::build(
            "debug.restore:manually_relaunchable:0004",
            "debug.artifact:symbol_dsym_mismatch:0006",
            "debug.session:prior-launch:0004",
            M5_DUMP_MAPPING_RESTORE_AS_OF,
            ManuallyRelaunchable,
            MismatchedBuild,
            false,
            SYMBOL_DSYM_MISMATCH_PROOF,
            "Prior launch against a build that has since changed: must be manually relaunched, \
             with a build-mismatch mapping and no reacquired process authority.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> DumpRestoreInvariant {
    DumpRestoreInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    artifacts: &[DebugArtifactStrip],
    restored_layouts: &[RestoredLayoutRecord],
) -> Vec<DumpRestoreInvariant> {
    // Every strip carries one pill whose flags equal the derivation from its fidelity,
    // build match, source class, and inspect-only posture.
    let artifact_one_canonical_pill = artifacts.iter().all(|a| {
        a.pill.matches_derivation(
            a.pill.fidelity,
            a.pill.build_match,
            a.pill.source_class,
            a.opens_inspect_only_session,
        ) && a.pill.fidelity_token == a.pill.fidelity.as_str()
            && a.pill.build_match_token == a.pill.build_match.as_str()
            && a.pill.source_class_token == a.pill.source_class.as_str()
    });

    // The full six-state mapping vocabulary is materialized across strips.
    let mapping_vocabulary_complete = DebugMappingFidelity::ALL
        .iter()
        .all(|f| artifacts.iter().any(|a| a.fidelity() == *f));

    // The full artifact-kind and source-class vocabularies are materialized.
    let artifact_kind_vocabulary_complete = DebugArtifactKind::ALL
        .iter()
        .all(|k| artifacts.iter().any(|a| a.artifact_kind == *k));
    let source_class_vocabulary_complete = ArtifactSourceClass::ALL
        .iter()
        .all(|s| artifacts.iter().any(|a| a.source_class() == *s));

    // A precise source link renders only for an exact mapping backed by an exact-build
    // match; every degraded strip discloses, and at least one degraded strip exists.
    let exact_link_never_hides = artifacts.iter().all(|a| {
        a.pill.shows_exact_source_link
            == (a.fidelity().preserves_exact_source() && a.build_match().proves_exact_build())
            && a.pill.requires_disclosure != a.pill.shows_exact_source_link
    }) && artifacts.iter().any(|a| !a.pill.shows_exact_source_link);

    // Imported and build-mismatch are both materialized and stay honest: an imported
    // mapping is always sourced from an import, a build-mismatch always carries a rejected
    // build match, and neither ever renders the exact source link.
    let imported_and_mismatch_honest = artifacts
        .iter()
        .any(|a| a.fidelity() == DebugMappingFidelity::Imported)
        && artifacts
            .iter()
            .any(|a| a.fidelity() == DebugMappingFidelity::MismatchedBuild)
        && artifacts.iter().all(|a| {
            (!a.fidelity().is_imported() || a.source_class().is_imported())
                && (!a.fidelity().is_build_mismatch()
                    || a.build_match() == ArtifactBuildMatch::MismatchedRejected)
                && (!(a.fidelity().is_imported() || a.fidelity().is_build_mismatch())
                    || !a.pill.shows_exact_source_link)
        });

    // The four distinct session entrypoints are each present, each open an inspect-only
    // session, and the import entrypoint is present and never opens a session.
    let entrypoints_distinct_and_visible = DebugArtifactEntrypoint::ALL
        .iter()
        .all(|e| artifacts.iter().any(|a| a.entrypoint == *e))
        && DebugArtifactEntrypoint::SESSION_ENTRYPOINTS
            .iter()
            .all(|e| {
                artifacts
                    .iter()
                    .filter(|a| a.entrypoint == *e)
                    .all(|a| a.opens_inspect_only_session)
                    && artifacts.iter().any(|a| a.entrypoint == *e)
            })
        && artifacts
            .iter()
            .filter(|a| a.entrypoint == DebugArtifactEntrypoint::ImportSymbolsOrSourceMap)
            .all(|a| !a.opens_inspect_only_session);

    // Every strip carries a build/artifact identity, a capture time, and an entrypoint
    // that accepts its kind, so a surface can always show current build/artifact identity.
    let build_artifact_identity_present = artifacts.iter().all(|a| {
        (a.build_id.is_some() || a.artifact_ref.is_some())
            && !a.captured_as_of.is_empty()
            && a.entrypoint.accepts_kind(a.artifact_kind)
            && (a.debug_format.is_some() == a.artifact_kind.is_mapping_input())
    });

    // Mirrored and imported sources are both materialized and disclose their provenance.
    let mirrored_and_imported_disclosed = artifacts.iter().any(|a| a.source_class().is_mirrored())
        && artifacts.iter().any(|a| a.source_class().is_imported())
        && artifacts.iter().all(|a| {
            a.pill.is_mirrored_source == a.source_class().is_mirrored()
                && a.pill.is_imported_source == a.source_class().is_imported()
                && (!a.source_class().requires_provenance_disclosure()
                    || a.pill.label.contains(a.source_class().short_label()))
        });

    // The full restore-posture vocabulary is materialized.
    let restore_posture_vocabulary_complete = RestorePosture::ALL
        .iter()
        .all(|p| restored_layouts.iter().any(|r| r.posture() == *p));

    // Every restored layout carries one pill whose flags equal its derivation and never
    // implies live continuity or process authority.
    let restore_never_implies_authority = restored_layouts.iter().all(|r| {
        r.pill.matches_derivation(
            r.pill.posture,
            r.pill.mapping_fidelity,
            r.exact_build_still_verified,
        ) && !r.pill.implies_live_continuity
            && !r.pill.implies_process_authority
            && r.pill.requires_disclosure
    });

    // A restored layout shows an exact-build mapping only when it is still verified; at
    // least one restored layout carries a degraded mapping.
    let restore_exact_only_when_verified = restored_layouts.iter().all(|r| {
        r.pill.implies_exact_build_mapping
            == (r.fidelity().preserves_exact_source() && r.exact_build_still_verified)
    }) && restored_layouts
        .iter()
        .any(|r| !r.fidelity().preserves_exact_source());

    // Reconnect-required and manually-relaunchable name an explicit action; gone and
    // inspect-only do not require one; at least one action-required and one no-action
    // restore exist.
    let restore_action_named = restored_layouts.iter().all(|r| {
        r.pill.requires_explicit_action == r.posture().requires_explicit_action()
            && r.pill.reconnect_available == (r.posture() == RestorePosture::ReconnectRequired)
            && r.pill.relaunch_available == (r.posture() == RestorePosture::ManuallyRelaunchable)
    }) && restored_layouts
        .iter()
        .any(|r| r.pill.requires_explicit_action)
        && restored_layouts
            .iter()
            .any(|r| !r.pill.requires_explicit_action);

    // The shared mapping vocabulary is a strict superset of the frame-mapping fidelity:
    // each frame fidelity widens and narrows back to itself, and the two extra states
    // (imported, mismatched-build) exist beyond the four frame states.
    let shared_vocabulary_supersets_frame =
        FrameMappingFidelity::ALL.iter().all(|ff| {
            DebugMappingFidelity::from_frame_fidelity(*ff).narrow_to_frame_fidelity() == *ff
        }) && [
            SymbolicationFidelityLabel::Exact,
            SymbolicationFidelityLabel::Approximate,
            SymbolicationFidelityLabel::SymbolOnly,
            SymbolicationFidelityLabel::Unresolved,
        ]
        .iter()
        .all(|label| {
            let widened = DebugMappingFidelity::from_symbolication_label(*label);
            !widened.is_imported() && !widened.is_build_mismatch()
        }) && DebugMappingFidelity::ALL.len() == FrameMappingFidelity::ALL.len() + 2
            && DebugMappingFidelity::ALL
                .iter()
                .filter(|f| f.is_imported() || f.is_build_mismatch())
                .count()
                == 2;

    // Every strip and restore retains its typed tokens and cites an export-safe proof
    // packet, so export never flattens them into rendered chrome.
    let export_retains_state = artifacts.iter().all(|a| {
        !a.pill.fidelity_token.is_empty()
            && !a.proof_packet_ref.is_empty()
            && is_export_safe_ref(&a.proof_packet_ref)
    }) && restored_layouts.iter().all(|r| {
        !r.pill.posture_token.is_empty()
            && !r.proof_packet_ref.is_empty()
            && is_export_safe_ref(&r.proof_packet_ref)
    });

    vec![
        invariant(
            "artifacts.one_canonical_mapping_pill",
            "Every artifact strip carries exactly one pill whose fidelity, build-match, and \
             source-class tokens come from the frozen vocabulary and whose flags equal their \
             derivation.",
            artifact_one_canonical_pill,
        ),
        invariant(
            "artifacts.mapping_vocabulary_complete",
            "Exact, approximate, symbol-only, unresolved, imported, and mismatched-build are all \
             materialized across the strips.",
            mapping_vocabulary_complete,
        ),
        invariant(
            "artifacts.artifact_kind_vocabulary_complete",
            "Core file, crash dump, inspect-only session, symbol artifact, source map, and replay \
             capture are all materialized.",
            artifact_kind_vocabulary_complete,
        ),
        invariant(
            "artifacts.source_class_vocabulary_complete",
            "Workspace, local, provider, mirror, and imported source classes are all materialized.",
            source_class_vocabulary_complete,
        ),
        invariant(
            "artifacts.exact_link_never_hides_degraded_mapping",
            "The unqualified precise source link renders only for an exact mapping backed by an \
             exact-build match; any degraded strip always discloses.",
            exact_link_never_hides,
        ),
        invariant(
            "artifacts.imported_and_mismatch_stay_honest",
            "Imported and build-mismatch are both materialized; an imported mapping is always \
             sourced from an import, a build-mismatch always carries a rejected build match, and \
             neither ever renders the exact source link.",
            imported_and_mismatch_honest,
        ),
        invariant(
            "artifacts.entrypoints_distinct_and_visible",
            "Core-file, crash-dump, open-replay, and open-inspect-only are each present and each \
             open an inspect-only session, distinct from importing a symbol or source-map artifact, \
             which never opens a session.",
            entrypoints_distinct_and_visible,
        ),
        invariant(
            "artifacts.build_artifact_identity_present",
            "Every strip carries a build id or artifact ref, a capture time, an entrypoint that \
             accepts its kind, and a debug format exactly for mapping inputs.",
            build_artifact_identity_present,
        ),
        invariant(
            "artifacts.mirrored_and_imported_sources_disclosed",
            "Mirrored and imported sources are both materialized and disclose their provenance \
             rather than posing as a local-trusted one.",
            mirrored_and_imported_disclosed,
        ),
        invariant(
            "restore.posture_vocabulary_complete",
            "Process-gone, inspect-only-continuation, reconnect-required, and manually-relaunchable \
             are all materialized.",
            restore_posture_vocabulary_complete,
        ),
        invariant(
            "restore.never_implies_live_continuity_or_authority",
            "Every restored layout carries one canonical pill that never implies live target \
             continuity or reacquired process authority and always discloses its non-live posture.",
            restore_never_implies_authority,
        ),
        invariant(
            "restore.exact_build_mapping_only_when_still_verified",
            "A restored layout shows an exact-build mapping only when the mapping is exact and the \
             build is still verified; at least one restored layout carries a degraded mapping.",
            restore_exact_only_when_verified,
        ),
        invariant(
            "restore.required_action_named",
            "Reconnect-required and manually-relaunchable name an explicit action; gone and \
             inspect-only do not; both an action-required and a no-action restore exist.",
            restore_action_named,
        ),
        invariant(
            "set.shared_mapping_vocabulary_supersets_frame_fidelity",
            "The shared mapping vocabulary is a strict superset of the frame-mapping fidelity: each \
             frame fidelity widens and narrows back to itself, and the imported and \
             mismatched-build states exist beyond the four frame states.",
            shared_vocabulary_supersets_frame,
        ),
        invariant(
            "set.export_retains_artifact_and_restore_state",
            "Every strip and restore retains its typed tokens and cites an export-safe proof \
             packet, so support export never flattens it into rendered chrome.",
            export_retains_state,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the dump/mapping/restore set as human-readable lines for CLI/headless and
/// support.
pub fn m5_dump_mapping_restore_lines(set: &DumpMappingRestoreSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 dump/mapping/restore — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Artifacts: {}  Restored layouts: {}  Invariants: {}",
        set.artifacts.len(),
        set.restored_layouts.len(),
        set.invariants.len(),
    ));

    lines.push("Artifacts:".to_owned());
    for a in &set.artifacts {
        lines.push(format!(
            "  - {} [{}] via {} pill={} exact_link={} inspect_only={}",
            a.strip_id,
            a.artifact_kind_token,
            a.entrypoint_token,
            a.pill.label,
            a.pill.shows_exact_source_link,
            a.opens_inspect_only_session,
        ));
        lines.push(format!(
            "      fidelity={} build_match={} source={} build_id={} format={}",
            a.pill.fidelity_token,
            a.pill.build_match_token,
            a.pill.source_class_token,
            a.build_id.as_deref().unwrap_or("-"),
            a.debug_format.map(|f| f.as_str()).unwrap_or("-"),
        ));
        lines.push(format!("      {}", a.summary));
        lines.push(format!("      proof: {}", a.proof_packet_ref));
    }

    lines.push("Restored layouts:".to_owned());
    for r in &set.restored_layouts {
        lines.push(format!(
            "  - {} -> {} pill={} live_continuity={} process_authority={} exact_mapping={}",
            r.layout_id,
            r.restored_strip_ref,
            r.pill.label,
            r.pill.implies_live_continuity,
            r.pill.implies_process_authority,
            r.pill.implies_exact_build_mapping,
        ));
        lines.push(format!(
            "      posture={} mapping={} action_required={} reconnect={} relaunch={}",
            r.pill.posture_token,
            r.pill.mapping_fidelity_token,
            r.pill.requires_explicit_action,
            r.pill.reconnect_available,
            r.pill.relaunch_available,
        ));
        lines.push(format!("      {}", r.summary));
        lines.push(format!("      proof: {}", r.proof_packet_ref));
    }

    lines.push("Invariants:".to_owned());
    for i in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
