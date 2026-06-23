//! Typed frame mappings and variable/watch snapshots: the canonical M5 records every
//! debugger, notebook, replay, and support surface reads to show *which* source a stack
//! frame maps to and *how trustworthy* that mapping is, and whether a value is a live
//! read, a captured snapshot, a stale last-known value, unavailable, or redacted.
//!
//! The [`m5_debug_contracts`](crate::m5_debug_contracts) matrix *names* the debugger
//! object families and freezes their vocabulary; the
//! [`m5_debug_session_descriptors`](crate::m5_debug_session_descriptors) and
//! [`m5_breakpoint_specs`](crate::m5_breakpoint_specs) lanes materialize the session,
//! attach-target, and breakpoint families. This lane *materializes* the
//! [`DebugObjectClass::FrameMapping`](crate::m5_debug_contracts::DebugObjectClass::FrameMapping)
//! and
//! [`DebugObjectClass::VariableWatchSnapshot`](crate::m5_debug_contracts::DebugObjectClass::VariableWatchSnapshot)
//! families as concrete, serde-serializable [`FrameMapping`] and [`ValueSnapshot`]
//! records and freezes a canonical [`FrameVariableSnapshotSet`].
//!
//! Frame and value truth stays explicit and replay-safe:
//!
//! - **One frame-mapping pill, one fidelity vocabulary.** Every frame carries one
//!   [`FrameMappingPill`] pinning one [`FrameMappingFidelity`] (exact, approximate,
//!   symbol-only, unmapped) and one [`BuildMatchClass`]. A frame stack never flattens
//!   exact, approximate, symbol-only, and unresolved frames into one generic location
//!   link: the
//!   [`shows_exact_source_link`](FrameMappingPill::shows_exact_source_link) flag is
//!   derived true only when the mapping is *exact* and the build identity *proves an
//!   exact build*. An approximate, symbol-only, unmapped, or build-mismatched frame
//!   always [`requires_disclosure`](FrameMappingPill::requires_disclosure).
//! - **Current-frame identity is preserved.** Each [`FrameMapping`] keeps stable
//!   session/thread/frame ids and explicit
//!   [`is_current_frame`](FrameMapping::is_current_frame) /
//!   [`is_selected_frame`](FrameMapping::is_selected_frame) flags, so the frame where
//!   execution stopped and the frame the user selected are never collapsed.
//! - **Source-map provenance is never silently flattened.** A
//!   [`FrameMappingProvenance::SourceMap`] mapping always discloses that it came through
//!   a source map; a lost mapping degrades to an explicit
//!   [`FrameMappingFidelity::Unmapped`] rather than a generic guessed location.
//! - **Async boundaries stay visible.** A frame whose parent is an async resumption or
//!   a runtime gap carries [`is_async_boundary`](FrameMapping::is_async_boundary) and
//!   discloses it, so a reconstructed caller is never drawn as a contiguous native one.
//! - **One value-disclosure vocabulary across variables, watches, notebook explorers,
//!   and replay inspectors.** Every [`ValueSnapshot`] — whether a
//!   [`SnapshotEntryKind::Variable`] or a [`SnapshotEntryKind::Watch`], and whether
//!   captured against a live session, a notebook cell, or a replay capture — carries one
//!   [`SnapshotDisclosurePill`] pinning one [`ValueDisclosure`] (live, captured, stale,
//!   unavailable, redacted). A value is shown as a live read only when it truly is one;
//!   an unavailable value carries a [`VariableUnavailableReason`]; a redacted value
//!   withholds its body; and a captured or stale value never implies live authority.
//!
//! [`m5_frame_variable_snapshot_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`SnapshotInvariant`]'s `holds` flag from the
//! built records, so the checked-in fixture and the freeze gate freeze the contract
//! byte-for-byte and an inconsistent edit flips an invariant and fails CI. The record
//! carries no source bodies, value bodies, raw paths, provider payloads, URLs,
//! hostnames, or credentials — only opaque object refs, stable tokens, opaque digests,
//! type/shape summaries, and short reviewable sentences — so it is safe for support
//! export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_frame_variable_snapshots.schema.json`](../../../schemas/debug/m5_frame_variable_snapshots.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_frame_variable_snapshots/canonical_set.json`](../../../fixtures/debug/m5_frame_variable_snapshots/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_frame_variable_snapshots.md`](../../../docs/debug/m5_frame_variable_snapshots.md).

use serde::{Deserialize, Serialize};

use crate::m5_debug_contracts::DebugConsumer;

#[cfg(test)]
mod tests;

/// Schema version for the M5 frame-mapping and variable/watch snapshot set.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 frame-mapping and variable/watch snapshot set.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_REF: &str =
    "schemas/debug/m5_frame_variable_snapshots.schema.json";

/// Stable record-kind tag for the frame/variable snapshot set.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_RECORD_KIND: &str = "m5_frame_variable_snapshot_set";

/// Stable id for the canonical frame/variable snapshot set.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_SET_ID: &str = "m5-frame-variable-snapshots:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the frame/variable snapshot set current. Stable promotion
/// runs this gate; it fails when the in-code set drifts from the checked-in fixture or
/// any invariant flips.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_frame_variable_snapshots.rs";

/// The checked-in canonical frame/variable snapshot-set fixture.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_FIXTURE_REF: &str =
    "fixtures/debug/m5_frame_variable_snapshots/canonical_set.json";

/// The contract narrative document.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_DOC_REF: &str = "docs/debug/m5_frame_variable_snapshots.md";

/// The human-readable evidence companion artifact.
pub const M5_FRAME_VARIABLE_SNAPSHOTS_ARTIFACT_REF: &str =
    "artifacts/debug/m5_frame_variable_snapshots.md";

// ---------------------------------------------------------------------------
// Frame mapping fidelity.
// ---------------------------------------------------------------------------

/// The fidelity of a frame's mapping from instruction to source.
///
/// Only [`FrameMappingFidelity::Exact`] preserves an authoritative source location;
/// every other state requires a visible caveat so a symbol-only or unresolved frame is
/// never drawn as a precise source link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameMappingFidelity {
    /// Maps exactly to an authoritative source line.
    Exact,
    /// Maps approximately (line-only, drifted, or nearest-span), disclosed as inexact.
    Approximate,
    /// Resolves a symbol / function name only, without authoritative source lines.
    SymbolOnly,
    /// Could not be mapped to source or symbol; an explicit unresolved frame.
    Unmapped,
}

impl FrameMappingFidelity {
    /// All fidelity classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Approximate,
        Self::SymbolOnly,
        Self::Unmapped,
    ];

    /// Stable snake_case token for this fidelity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::SymbolOnly => "symbol_only",
            Self::Unmapped => "unmapped",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Approximate => "Approximate",
            Self::SymbolOnly => "Symbol-only",
            Self::Unmapped => "Unmapped",
        }
    }

    /// Short caveat label used in a pill (e.g. `Approximate · source-map`).
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::SymbolOnly => "symbol-only",
            Self::Unmapped => "unmapped",
        }
    }

    /// Whether this fidelity preserves an authoritative source location.
    pub const fn preserves_exact_source(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Whether this fidelity must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact)
    }

    /// Whether this fidelity offers a source line a reader can navigate to.
    pub const fn allows_source_navigation(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate)
    }
}

// ---------------------------------------------------------------------------
// Frame mapping provenance.
// ---------------------------------------------------------------------------

/// How a frame's current mapping was derived — so a source-map mapping is never flattened
/// into a direct exact link and a lost mapping is named explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameMappingProvenance {
    /// Mapped directly from authoritative debug line tables for the running build.
    DirectSourceLine,
    /// Mapped through a source map (transpiled / bundled / generated source).
    SourceMap,
    /// Resolved a symbol from a symbol table without authoritative source lines.
    SymbolTable,
    /// Derived by a line-only heuristic over drifted or partial debug info.
    HeuristicLineOnly,
    /// No mapping could be resolved; forces an explicit unmapped frame.
    Unresolved,
}

impl FrameMappingProvenance {
    /// All provenance classes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::DirectSourceLine,
        Self::SourceMap,
        Self::SymbolTable,
        Self::HeuristicLineOnly,
        Self::Unresolved,
    ];

    /// Stable snake_case token for this provenance class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectSourceLine => "direct_source_line",
            Self::SourceMap => "source_map",
            Self::SymbolTable => "symbol_table",
            Self::HeuristicLineOnly => "heuristic_line_only",
            Self::Unresolved => "unresolved",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DirectSourceLine => "Direct source line",
            Self::SourceMap => "Source map",
            Self::SymbolTable => "Symbol table",
            Self::HeuristicLineOnly => "Heuristic (line-only)",
            Self::Unresolved => "Unresolved",
        }
    }

    /// Short pill fragment for this provenance, when it must be disclosed.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::DirectSourceLine => "direct",
            Self::SourceMap => "source-map",
            Self::SymbolTable => "symbol table",
            Self::HeuristicLineOnly => "line-only",
            Self::Unresolved => "unresolved",
        }
    }

    /// Whether this provenance is a source-map mapping, which must always disclose it.
    pub const fn is_source_map(self) -> bool {
        matches!(self, Self::SourceMap)
    }

    /// Whether this provenance forces a [`FrameMappingFidelity::Unmapped`]: an
    /// unresolved mapping cannot resolve to any authoritative location.
    pub const fn forces_unmapped(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether this provenance must render with a visible caveat. Only a direct
    /// source-line mapping needs none.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::DirectSourceLine)
    }
}

// ---------------------------------------------------------------------------
// Build / artifact match.
// ---------------------------------------------------------------------------

/// Whether the build the frame ran against matches the artifact the mapping was resolved
/// from. Mirrors the symbolication build-match vocabulary so debug, profiler, and
/// crash surfaces read one truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildMatchClass {
    /// Build identity matched field-for-field; an exact-build match.
    ExactBuildVerified,
    /// Only an approximate / candidate build was available, disclosed.
    ApproximateCandidate,
    /// A candidate build was found but rejected because it mismatched.
    MismatchedRejected,
    /// No build identity was available to verify against.
    NoCandidate,
}

impl BuildMatchClass {
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
// Frame continuity / async boundary.
// ---------------------------------------------------------------------------

/// Whether the frame above this one is a contiguous native caller, or a reconstructed
/// boundary — an async resumption or a runtime gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameContinuityClass {
    /// The caller is a contiguous native parent frame.
    Contiguous,
    /// The frame resumed across an async suspension; its caller is reconstructed.
    AsyncResumption,
    /// A runtime / FFI / native boundary separates this frame from its caller.
    RuntimeGap,
}

impl FrameContinuityClass {
    /// All continuity classes, in canonical order.
    pub const ALL: [Self; 3] = [Self::Contiguous, Self::AsyncResumption, Self::RuntimeGap];

    /// Stable snake_case token for this continuity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Contiguous => "contiguous",
            Self::AsyncResumption => "async_resumption",
            Self::RuntimeGap => "runtime_gap",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Contiguous => "Contiguous",
            Self::AsyncResumption => "Async resumption",
            Self::RuntimeGap => "Runtime gap",
        }
    }

    /// Whether this continuity crosses an async / runtime boundary that must be disclosed.
    pub const fn is_async_boundary(self) -> bool {
        matches!(self, Self::AsyncResumption | Self::RuntimeGap)
    }
}

// ---------------------------------------------------------------------------
// Value freshness, disclosure, redaction, and unavailability.
// ---------------------------------------------------------------------------

/// The freshness / availability of a captured value, before redaction is applied.
///
/// Only [`VariableFreshnessState::Live`] is a live read at the current stop. A captured
/// or stale value still carries a body but never implies live authority; an unavailable
/// value carries no body and names a reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableFreshnessState {
    /// Live read at the current stop on a session with live authority.
    Live,
    /// Captured at a prior stop or from a recorded capture; internally consistent at
    /// capture time, but not a live read.
    CapturedSnapshot,
    /// Stale last-known value: the target resumed since it was captured.
    Stale,
    /// Unavailable — optimized out, out of scope, not loaded, or failed to evaluate.
    Unavailable,
}

impl VariableFreshnessState {
    /// All freshness states, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Live,
        Self::CapturedSnapshot,
        Self::Stale,
        Self::Unavailable,
    ];

    /// Stable snake_case token for this freshness state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::CapturedSnapshot => "Captured snapshot",
            Self::Stale => "Stale",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Whether a value in this state carries a value body (a digest/summary). Unavailable
    /// values carry none.
    pub const fn carries_value(self) -> bool {
        matches!(self, Self::Live | Self::CapturedSnapshot | Self::Stale)
    }
}

/// The single value-disclosure vocabulary every variable, watch, notebook explorer, and
/// replay inspector renders. Derived from freshness and redaction so a value can never
/// silently disagree about whether it is live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueDisclosure {
    /// A live read at the current stop.
    Live,
    /// A captured snapshot, not a live read.
    Captured,
    /// A stale last-known value.
    Stale,
    /// Unavailable, with a named reason.
    Unavailable,
    /// Withheld by a redaction policy.
    Redacted,
}

impl ValueDisclosure {
    /// All disclosure classes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::Captured,
        Self::Stale,
        Self::Unavailable,
        Self::Redacted,
    ];

    /// Stable snake_case token for this disclosure class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Captured => "captured",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::Redacted => "redacted",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::Captured => "Captured snapshot",
            Self::Stale => "Stale (last known)",
            Self::Unavailable => "Unavailable",
            Self::Redacted => "Redacted",
        }
    }

    /// Derives the canonical disclosure from a freshness state and a redaction class.
    /// Redaction dominates: a redacted value reads as `redacted` regardless of freshness.
    pub const fn derive(freshness: VariableFreshnessState, redaction: ValueRedactionClass) -> Self {
        if redaction.is_redacted() {
            return Self::Redacted;
        }
        match freshness {
            VariableFreshnessState::Live => Self::Live,
            VariableFreshnessState::CapturedSnapshot => Self::Captured,
            VariableFreshnessState::Stale => Self::Stale,
            VariableFreshnessState::Unavailable => Self::Unavailable,
        }
    }

    /// Whether this disclosure is a live read.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// Whether this disclosure must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Why a value's body is withheld on a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueRedactionClass {
    /// Not redacted; the value body is present (subject to truncation).
    NotRedacted,
    /// Withheld because it matched a secret / credential class.
    SecretRedacted,
    /// Withheld because it matched a personal-data class.
    PiiRedacted,
    /// Withheld by an explicit policy rule on this surface.
    PolicyWithheld,
}

impl ValueRedactionClass {
    /// All redaction classes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::NotRedacted,
        Self::SecretRedacted,
        Self::PiiRedacted,
        Self::PolicyWithheld,
    ];

    /// Stable snake_case token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRedacted => "not_redacted",
            Self::SecretRedacted => "secret_redacted",
            Self::PiiRedacted => "pii_redacted",
            Self::PolicyWithheld => "policy_withheld",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotRedacted => "Not redacted",
            Self::SecretRedacted => "Secret (redacted)",
            Self::PiiRedacted => "Personal data (redacted)",
            Self::PolicyWithheld => "Policy-withheld",
        }
    }

    /// Whether this class withholds the value body.
    pub const fn is_redacted(self) -> bool {
        !matches!(self, Self::NotRedacted)
    }
}

/// Why a value is unavailable. Present exactly when the disclosure is
/// [`ValueDisclosure::Unavailable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableUnavailableReason {
    /// The compiler optimized the value out of the running build.
    OptimizedOut,
    /// The value is out of scope at the current frame.
    OutOfScope,
    /// A lazy value whose body was not loaded and could not be fetched.
    NotLoaded,
    /// The target resumed and the value could not be re-read.
    TargetResumed,
    /// A watch expression failed to evaluate.
    EvaluationError,
    /// The adapter or runtime does not support reading this value.
    Unsupported,
}

impl VariableUnavailableReason {
    /// All unavailable reasons, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::OptimizedOut,
        Self::OutOfScope,
        Self::NotLoaded,
        Self::TargetResumed,
        Self::EvaluationError,
        Self::Unsupported,
    ];

    /// Stable snake_case token for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OptimizedOut => "optimized_out",
            Self::OutOfScope => "out_of_scope",
            Self::NotLoaded => "not_loaded",
            Self::TargetResumed => "target_resumed",
            Self::EvaluationError => "evaluation_error",
            Self::Unsupported => "unsupported",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OptimizedOut => "Optimized out",
            Self::OutOfScope => "Out of scope",
            Self::NotLoaded => "Not loaded",
            Self::TargetResumed => "Target resumed",
            Self::EvaluationError => "Evaluation error",
            Self::Unsupported => "Unsupported",
        }
    }
}

// ---------------------------------------------------------------------------
// Variable scope and entry kind.
// ---------------------------------------------------------------------------

/// The scope a captured value belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableScopeClass {
    /// A local variable in the current frame.
    Local,
    /// A function argument.
    Argument,
    /// A captured closure / upvalue.
    Closure,
    /// A module-level / global binding.
    Global,
    /// A machine register or low-level location.
    Register,
    /// A user watch expression rather than a named binding.
    WatchExpression,
}

impl VariableScopeClass {
    /// All scopes, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Local,
        Self::Argument,
        Self::Closure,
        Self::Global,
        Self::Register,
        Self::WatchExpression,
    ];

    /// Stable snake_case token for this scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Argument => "argument",
            Self::Closure => "closure",
            Self::Global => "global",
            Self::Register => "register",
            Self::WatchExpression => "watch_expression",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Argument => "Argument",
            Self::Closure => "Closure",
            Self::Global => "Global",
            Self::Register => "Register",
            Self::WatchExpression => "Watch expression",
        }
    }

    /// Whether this scope is the watch-expression scope reserved for watch entries.
    pub const fn is_watch_scope(self) -> bool {
        matches!(self, Self::WatchExpression)
    }
}

/// Whether a snapshot is a named variable read or a user watch expression. Both share
/// the same disclosure vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotEntryKind {
    /// A named variable / scope binding.
    Variable,
    /// A user watch expression.
    Watch,
}

impl SnapshotEntryKind {
    /// All entry kinds, in canonical order.
    pub const ALL: [Self; 2] = [Self::Variable, Self::Watch];

    /// Stable snake_case token for this entry kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Variable => "variable",
            Self::Watch => "watch",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Variable => "Variable",
            Self::Watch => "Watch",
        }
    }

    /// Whether this entry is a watch expression.
    pub const fn is_watch(self) -> bool {
        matches!(self, Self::Watch)
    }
}

/// The structural shape of a captured value, for the type/shape/size summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueShapeClass {
    /// A scalar (number, boolean, char, …).
    Scalar,
    /// A string / text value.
    Text,
    /// An ordered collection (array, vector, list, …).
    Collection,
    /// A keyed map / dictionary.
    Map,
    /// A struct / record / object with named fields.
    Struct,
    /// A reference / pointer / handle.
    Reference,
    /// An opaque value whose internals are not modeled.
    Opaque,
}

impl ValueShapeClass {
    /// All shape classes, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::Scalar,
        Self::Text,
        Self::Collection,
        Self::Map,
        Self::Struct,
        Self::Reference,
        Self::Opaque,
    ];

    /// Stable snake_case token for this shape.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Text => "text",
            Self::Collection => "collection",
            Self::Map => "map",
            Self::Struct => "struct",
            Self::Reference => "reference",
            Self::Opaque => "opaque",
        }
    }
}

/// Why a value's rendered representation was truncated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncationReason {
    /// Truncated because the byte size exceeded the budget.
    SizeLimit,
    /// Truncated because the nesting depth exceeded the budget.
    DepthLimit,
    /// Truncated because the element count exceeded the budget.
    ElementCountLimit,
    /// Truncated because the string length exceeded the budget.
    StringLengthLimit,
}

impl TruncationReason {
    /// Stable snake_case token for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SizeLimit => "size_limit",
            Self::DepthLimit => "depth_limit",
            Self::ElementCountLimit => "element_count_limit",
            Self::StringLengthLimit => "string_length_limit",
        }
    }
}

// ---------------------------------------------------------------------------
// Frame-mapping records.
// ---------------------------------------------------------------------------

/// The source / artifact location a frame maps to. A mapped frame carries a logical
/// source ref and a line; an unmapped or symbol-only frame may carry only an artifact
/// ref. No raw paths cross this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameSourceLocation {
    /// Stable logical source identity, present when the frame maps to source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_source_ref: Option<String>,
    /// Export-safe artifact ref for the binary / module the frame ran in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    /// One-based source line, when the mapping resolves one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// One-based end line of the mapped span, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    /// One-based column, when the mapping is column-precise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
}

/// The build / artifact identity a frame's mapping was resolved against, preserved so a
/// frame stack and an exported crash never lose which build produced the mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildArtifactIdentity {
    /// Opaque digest of the build id, never a raw path or URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// Export-safe artifact ref for the build, when one is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    /// Whether the build matched the artifact the mapping came from.
    pub match_state: BuildMatchClass,
    /// Stable token for the match state.
    pub match_state_token: String,
    /// Whether the match state must be disclosed.
    pub match_requires_disclosure: bool,
}

impl BuildArtifactIdentity {
    /// Builds a build/artifact identity, deriving the computed match flags.
    pub fn build(
        build_id: Option<&str>,
        artifact_ref: Option<&str>,
        match_state: BuildMatchClass,
    ) -> Self {
        Self {
            build_id: build_id.map(str::to_owned),
            artifact_ref: artifact_ref.map(str::to_owned),
            match_state,
            match_state_token: match_state.as_str().to_owned(),
            match_requires_disclosure: match_state.requires_disclosure(),
        }
    }

    /// Whether the carried tokens and flags agree with the match state.
    pub fn is_consistent(&self) -> bool {
        self.match_state_token == self.match_state.as_str()
            && self.match_requires_disclosure == self.match_state.requires_disclosure()
    }
}

/// The single canonical pill every surface renders for a frame's mapping — one fidelity
/// and one build-match outcome, with every disclosure flag derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameMappingPill {
    /// The mapping fidelity.
    pub fidelity: FrameMappingFidelity,
    /// Stable token for the fidelity.
    pub fidelity_token: String,
    /// The build-match outcome.
    pub build_match: BuildMatchClass,
    /// Stable token for the build-match outcome.
    pub build_match_token: String,
    /// One reviewable pill label combining fidelity, source-map provenance, build match,
    /// and async boundary.
    pub label: String,
    /// Whether the frame may render an unqualified exact source link — true only when the
    /// mapping is exact and the build identity proves an exact build.
    pub shows_exact_source_link: bool,
    /// Whether the frame must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether the frame offers a source line a reader can navigate to.
    pub allows_source_navigation: bool,
    /// Whether the frame's caller is across an async / runtime boundary.
    pub is_async_boundary: bool,
}

impl FrameMappingPill {
    /// Whether a frame may render the unqualified exact source link: only an exact
    /// mapping backed by an exact-build match. This is the guardrail that keeps a precise
    /// source link from hiding an approximate, symbol-only, unmapped, or build-mismatched
    /// reality.
    pub const fn derive_shows_exact_source_link(
        fidelity: FrameMappingFidelity,
        build_match: BuildMatchClass,
    ) -> bool {
        fidelity.preserves_exact_source() && build_match.proves_exact_build()
    }

    /// Builds the canonical pill for a frame, deriving every flag and the label from the
    /// fidelity, build match, provenance, and continuity so the pill cannot disagree with
    /// itself.
    pub fn derive(
        fidelity: FrameMappingFidelity,
        build_match: BuildMatchClass,
        provenance: FrameMappingProvenance,
        continuity: FrameContinuityClass,
    ) -> Self {
        let shows_exact_source_link = Self::derive_shows_exact_source_link(fidelity, build_match);
        let mut label = fidelity.label().to_owned();
        if provenance.is_source_map() {
            label.push_str(" · source-map");
        }
        if build_match.requires_disclosure() {
            label.push_str(" · ");
            label.push_str(build_match.short_label());
        }
        if continuity.is_async_boundary() {
            label.push_str(" · async boundary");
        }
        Self {
            fidelity,
            fidelity_token: fidelity.as_str().to_owned(),
            build_match,
            build_match_token: build_match.as_str().to_owned(),
            label,
            shows_exact_source_link,
            requires_disclosure: !shows_exact_source_link,
            allows_source_navigation: fidelity.allows_source_navigation(),
            is_async_boundary: continuity.is_async_boundary(),
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        fidelity: FrameMappingFidelity,
        build_match: BuildMatchClass,
        provenance: FrameMappingProvenance,
        continuity: FrameContinuityClass,
    ) -> bool {
        *self == Self::derive(fidelity, build_match, provenance, continuity)
    }
}

/// A typed frame mapping: the canonical record every frame stack, notebook frame view,
/// replay inspector, and exported crash reads to show one stack frame, where it maps in
/// source, and how trustworthy that mapping is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameMapping {
    /// Stable, namespaced frame id.
    pub frame_id: String,
    /// Stable session id the frame belongs to.
    pub session_id: String,
    /// Stable thread id the frame belongs to.
    pub thread_id: String,
    /// Depth of the frame within its thread stack; 0 is the topmost (innermost) frame.
    pub frame_index: u32,
    /// Function / method label for the frame.
    pub function_label: String,
    /// Symbol name (possibly demangled) when one is resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_label: Option<String>,
    /// The source / artifact location the frame maps to.
    pub location: FrameSourceLocation,
    /// The build / artifact identity the mapping was resolved against.
    pub build_identity: BuildArtifactIdentity,
    /// How the current mapping was derived.
    pub mapping_provenance: FrameMappingProvenance,
    /// Stable token for the mapping provenance.
    pub mapping_provenance_token: String,
    /// Whether the mapping provenance must be disclosed.
    pub mapping_provenance_requires_disclosure: bool,
    /// Whether the frame's caller is contiguous, an async resumption, or a runtime gap.
    pub continuity: FrameContinuityClass,
    /// Stable token for the continuity class.
    pub continuity_token: String,
    /// Whether the frame's caller is across an async / runtime boundary.
    pub is_async_boundary: bool,
    /// Whether this is the current frame: the topmost frame where execution stopped.
    pub is_current_frame: bool,
    /// Whether this is the frame the user / inspector has selected.
    pub is_selected_frame: bool,
    /// The canonical fidelity + build-match pill every surface renders.
    pub pill: FrameMappingPill,
    /// The proof packet that keeps this frame mapping current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the frame.
    pub summary: String,
}

impl FrameMapping {
    /// Builds a frame mapping, deriving every computed token, honesty flag, and the pill
    /// from the typed enums so the record cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        frame_id: impl Into<String>,
        session_id: impl Into<String>,
        thread_id: impl Into<String>,
        frame_index: u32,
        function_label: impl Into<String>,
        symbol_label: Option<&str>,
        location: FrameSourceLocation,
        build_identity: BuildArtifactIdentity,
        fidelity: FrameMappingFidelity,
        mapping_provenance: FrameMappingProvenance,
        continuity: FrameContinuityClass,
        is_current_frame: bool,
        is_selected_frame: bool,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            frame_id: frame_id.into(),
            session_id: session_id.into(),
            thread_id: thread_id.into(),
            frame_index,
            function_label: function_label.into(),
            symbol_label: symbol_label.map(str::to_owned),
            location,
            pill: FrameMappingPill::derive(
                fidelity,
                build_identity.match_state,
                mapping_provenance,
                continuity,
            ),
            build_identity,
            mapping_provenance,
            mapping_provenance_token: mapping_provenance.as_str().to_owned(),
            mapping_provenance_requires_disclosure: mapping_provenance.requires_disclosure(),
            continuity,
            continuity_token: continuity.as_str().to_owned(),
            is_async_boundary: continuity.is_async_boundary(),
            is_current_frame,
            is_selected_frame,
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The mapping fidelity from the pill.
    pub const fn fidelity(&self) -> FrameMappingFidelity {
        self.pill.fidelity
    }
}

// ---------------------------------------------------------------------------
// Value snapshot records.
// ---------------------------------------------------------------------------

/// The capture context a value snapshot was read in: the session/thread/frame, the
/// capture timestamp, and the notebook or replay surface it belongs to. Ties a variable
/// to a frame and lets notebook explorers and replay inspectors reuse the same record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotCaptureContext {
    /// Stable session id the value was read in.
    pub session_id: String,
    /// Stable thread id, when the value is bound to a thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// Stable frame id the value belongs to, tying it to a [`FrameMapping`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<String>,
    /// Timestamp the value was captured at.
    pub captured_as_of: String,
    /// The stop sequence number the value was read at, used to detect staleness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_stop_seq: Option<u64>,
    /// Stable notebook cell ref, when the value is shown in a notebook variable explorer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notebook_cell_ref: Option<String>,
    /// Stable replay capture ref, when the value is read from a recorded capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_capture_ref: Option<String>,
}

/// The type / shape / size summary of a captured value. Type names are structural and
/// safe to carry; value bodies are not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeShapeSummary {
    /// The value's type name (a code identifier, not data).
    pub type_name: String,
    /// The structural shape class.
    pub shape: ValueShapeClass,
    /// Stable token for the shape class.
    pub shape_token: String,
    /// Element / field count for collections, maps, and structs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_count: Option<u64>,
    /// Short, reviewable size summary (e.g. `4 KiB`, `1024 elems`), never a value body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_summary: Option<String>,
}

impl TypeShapeSummary {
    /// Builds a type/shape/size summary, deriving the shape token.
    pub fn build(
        type_name: impl Into<String>,
        shape: ValueShapeClass,
        element_count: Option<u64>,
        size_summary: Option<&str>,
    ) -> Self {
        Self {
            type_name: type_name.into(),
            shape,
            shape_token: shape.as_str().to_owned(),
            element_count,
            size_summary: size_summary.map(str::to_owned),
        }
    }

    /// Whether the shape token agrees with the shape.
    pub fn is_consistent(&self) -> bool {
        self.shape_token == self.shape.as_str()
    }
}

/// Whether and why a value's rendered representation was truncated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueTruncation {
    /// Whether the representation was truncated.
    pub is_truncated: bool,
    /// Why it was truncated, when it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<TruncationReason>,
    /// Stable token for the truncation reason, when truncated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_token: Option<String>,
    /// Short note on the shown extent (e.g. `first 256 bytes`), when truncated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shown_extent: Option<String>,
}

impl ValueTruncation {
    /// A complete, untruncated value.
    pub fn complete() -> Self {
        Self {
            is_truncated: false,
            reason: None,
            reason_token: None,
            shown_extent: None,
        }
    }

    /// A truncated value with a reason and a shown-extent note.
    pub fn truncated(reason: TruncationReason, shown_extent: &str) -> Self {
        Self {
            is_truncated: true,
            reason: Some(reason),
            reason_token: Some(reason.as_str().to_owned()),
            shown_extent: Some(shown_extent.to_owned()),
        }
    }

    /// Whether the presence flags agree with the carried reason.
    pub fn is_consistent(&self) -> bool {
        if self.is_truncated {
            self.reason.is_some()
                && self.reason_token.as_deref() == self.reason.map(TruncationReason::as_str)
        } else {
            self.reason.is_none() && self.reason_token.is_none()
        }
    }
}

/// The single canonical disclosure pill every variable, watch, notebook explorer, and
/// replay inspector renders for a value — one freshness state, one derived disclosure,
/// with every honesty flag computed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotDisclosurePill {
    /// The freshness state before redaction.
    pub freshness: VariableFreshnessState,
    /// Stable token for the freshness state.
    pub freshness_token: String,
    /// The derived disclosure: live, captured, stale, unavailable, or redacted.
    pub disclosure: ValueDisclosure,
    /// Stable token for the disclosure.
    pub disclosure_token: String,
    /// One reviewable disclosure label.
    pub label: String,
    /// Whether this is a live read at the current stop.
    pub is_live_read: bool,
    /// Whether this value implies the debugger holds live authority — only a live read.
    pub implies_live_authority: bool,
    /// Whether this value must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether a value body (digest/summary) is present — live, captured, or stale and
    /// not redacted.
    pub value_body_present: bool,
    /// Whether the value body is withheld by redaction.
    pub is_redacted: bool,
}

impl SnapshotDisclosurePill {
    /// Builds the canonical disclosure pill, deriving every flag from the freshness state
    /// and redaction class so the pill cannot disagree with itself.
    pub fn derive(freshness: VariableFreshnessState, redaction: ValueRedactionClass) -> Self {
        let disclosure = ValueDisclosure::derive(freshness, redaction);
        let is_redacted = redaction.is_redacted();
        let value_body_present = !is_redacted && freshness.carries_value();
        let is_live_read = disclosure.is_live();
        let mut label = disclosure.label().to_owned();
        if is_redacted && redaction != ValueRedactionClass::NotRedacted {
            label.push_str(" · ");
            label.push_str(redaction.label());
        }
        Self {
            freshness,
            freshness_token: freshness.as_str().to_owned(),
            disclosure,
            disclosure_token: disclosure.as_str().to_owned(),
            label,
            is_live_read,
            implies_live_authority: is_live_read,
            requires_disclosure: disclosure.requires_disclosure(),
            value_body_present,
            is_redacted,
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        freshness: VariableFreshnessState,
        redaction: ValueRedactionClass,
    ) -> bool {
        *self == Self::derive(freshness, redaction)
    }
}

/// A typed variable / watch snapshot: the canonical record every variables pane, watch
/// list, notebook variable explorer, and replay inspector reads to show one captured
/// value and whether it is a live read, a captured snapshot, a stale last-known value,
/// unavailable, or redacted.
///
/// One struct materializes both the variable-snapshot and watch-snapshot families: a
/// [`SnapshotEntryKind::Watch`] entry carries a watch-expression digest and uses the
/// [`VariableScopeClass::WatchExpression`] scope, while every other field — scope,
/// type/shape/size, freshness, truncation, redaction, capture context — is shared, so
/// variables and watches reuse one disclosure vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSnapshot {
    /// Stable, namespaced snapshot id.
    pub snapshot_id: String,
    /// Whether this is a named variable read or a user watch expression.
    pub entry_kind: SnapshotEntryKind,
    /// Stable token for the entry kind.
    pub entry_kind_token: String,
    /// Display name: the binding name or the watch label (a code identifier, not data).
    pub display_name: String,
    /// Opaque digest of the watch expression, present only for watch entries; never the
    /// raw expression source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watch_expression_digest: Option<String>,
    /// The scope the value belongs to.
    pub scope: VariableScopeClass,
    /// Stable token for the scope.
    pub scope_token: String,
    /// The capture context: session/thread/frame, timestamp, notebook/replay surface.
    pub capture_context: SnapshotCaptureContext,
    /// The type / shape / size summary.
    pub type_shape: TypeShapeSummary,
    /// Whether and why the value representation was truncated.
    pub truncation: ValueTruncation,
    /// Opaque digest of the value representation; present only when a value body is
    /// present (live/captured/stale and not redacted), never the raw value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_repr_digest: Option<String>,
    /// Whether the value is a lazy / expandable handle whose children are not yet loaded.
    pub lazy_loadable: bool,
    /// Why the value is unavailable; present exactly when the disclosure is unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<VariableUnavailableReason>,
    /// Stable token for the unavailable reason, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason_token: Option<String>,
    /// The value redaction class.
    pub redaction: ValueRedactionClass,
    /// Stable token for the redaction class.
    pub redaction_token: String,
    /// The canonical freshness + disclosure pill every surface renders.
    pub disclosure: SnapshotDisclosurePill,
    /// The proof packet that keeps this snapshot current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the snapshot.
    pub summary: String,
}

impl ValueSnapshot {
    /// Builds a value snapshot, deriving every computed token, the disclosure pill, and
    /// the unavailable-reason token from the typed inputs so the record cannot disagree
    /// with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        snapshot_id: impl Into<String>,
        entry_kind: SnapshotEntryKind,
        display_name: impl Into<String>,
        watch_expression_digest: Option<&str>,
        scope: VariableScopeClass,
        capture_context: SnapshotCaptureContext,
        type_shape: TypeShapeSummary,
        truncation: ValueTruncation,
        value_repr_digest: Option<&str>,
        lazy_loadable: bool,
        unavailable_reason: Option<VariableUnavailableReason>,
        redaction: ValueRedactionClass,
        freshness: VariableFreshnessState,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            snapshot_id: snapshot_id.into(),
            entry_kind,
            entry_kind_token: entry_kind.as_str().to_owned(),
            display_name: display_name.into(),
            watch_expression_digest: watch_expression_digest.map(str::to_owned),
            scope,
            scope_token: scope.as_str().to_owned(),
            capture_context,
            type_shape,
            truncation,
            value_repr_digest: value_repr_digest.map(str::to_owned),
            lazy_loadable,
            unavailable_reason,
            unavailable_reason_token: unavailable_reason.map(|r| r.as_str().to_owned()),
            redaction,
            redaction_token: redaction.as_str().to_owned(),
            disclosure: SnapshotDisclosurePill::derive(freshness, redaction),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The derived disclosure from the pill.
    pub const fn disclosure_class(&self) -> ValueDisclosure {
        self.disclosure.disclosure
    }

    /// The freshness state from the pill.
    pub const fn freshness(&self) -> VariableFreshnessState {
        self.disclosure.freshness
    }
}

// ---------------------------------------------------------------------------
// Invariants and set.
// ---------------------------------------------------------------------------

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 frame-mapping and variable/watch snapshot set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameVariableSnapshotSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_frame_variable_snapshots_schema_version: u32,
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
    /// The surfaces that consume the frame mappings and value snapshots.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The frame mappings.
    pub frames: Vec<FrameMapping>,
    /// The variable / watch snapshots.
    pub snapshots: Vec<ValueSnapshot>,
    /// The computed invariants.
    pub invariants: Vec<SnapshotInvariant>,
    /// Whether raw source bodies and value bodies are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the frame/variable snapshot set fails a structural consistency
/// check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameVariableSnapshotSetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for FrameVariableSnapshotSetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m5 frame/variable snapshot set invalid: {}", self.reason)
    }
}

impl std::error::Error for FrameVariableSnapshotSetValidationError {}

impl FrameVariableSnapshotSet {
    /// Returns the frame mapping with the given id, if present.
    pub fn frame(&self, frame_id: &str) -> Option<&FrameMapping> {
        self.frames.iter().find(|f| f.frame_id == frame_id)
    }

    /// Returns the snapshot with the given id, if present.
    pub fn snapshot(&self, snapshot_id: &str) -> Option<&ValueSnapshot> {
        self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id)
    }

    /// Returns the first frame in the given fidelity, if present.
    pub fn frame_in_fidelity(&self, fidelity: FrameMappingFidelity) -> Option<&FrameMapping> {
        self.frames.iter().find(|f| f.fidelity() == fidelity)
    }

    /// Returns the first snapshot in the given disclosure class, if present.
    pub fn snapshot_in_disclosure(&self, disclosure: ValueDisclosure) -> Option<&ValueSnapshot> {
        self.snapshots
            .iter()
            .find(|s| s.disclosure_class() == disclosure)
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
        let from_frames = self.frames.iter().map(|f| f.proof_packet_ref.as_str());
        let from_snapshots = self.snapshots.iter().map(|s| s.proof_packet_ref.as_str());
        from_set.chain(from_frames).chain(from_snapshots)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns a [`FrameVariableSnapshotSetValidationError`] when an identifier, a ref, a
    /// computed flag, a pill, a freshness/redaction rule, a frame-mapping rule, or an
    /// invariant is inconsistent.
    pub fn validate(&self) -> Result<(), FrameVariableSnapshotSetValidationError> {
        let fail = |reason: String| Err(FrameVariableSnapshotSetValidationError { reason });

        if self.record_kind != M5_FRAME_VARIABLE_SNAPSHOTS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_frame_variable_snapshots_schema_version
            != M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_VERSION
        {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.frames.is_empty() {
            return fail("no frames".to_owned());
        }
        if self.snapshots.is_empty() {
            return fail("no snapshots".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.frames.iter().map(|f| f.frame_id.as_str())) {
            return fail("frame ids are not unique".to_owned());
        }
        if !all_unique(self.snapshots.iter().map(|s| s.snapshot_id.as_str())) {
            return fail("snapshot ids are not unique".to_owned());
        }

        // The full fidelity vocabulary is materialized.
        for fidelity in FrameMappingFidelity::ALL {
            if self.frame_in_fidelity(fidelity).is_none() {
                return fail(format!(
                    "frame fidelity {} is not materialized",
                    fidelity.as_str()
                ));
            }
        }
        // The full disclosure vocabulary is materialized.
        for disclosure in ValueDisclosure::ALL {
            if self.snapshot_in_disclosure(disclosure).is_none() {
                return fail(format!(
                    "value disclosure {} is not materialized",
                    disclosure.as_str()
                ));
            }
        }
        // Both entry kinds are materialized.
        for kind in SnapshotEntryKind::ALL {
            if !self.snapshots.iter().any(|s| s.entry_kind == kind) {
                return fail(format!("entry kind {} is not materialized", kind.as_str()));
            }
        }

        // Per-frame structural floor and cross-cutting rules.
        for fr in &self.frames {
            validate_frame(fr)
                .map_err(|reason| FrameVariableSnapshotSetValidationError { reason })?;
        }
        // Exactly one current frame per thread.
        for (session_id, thread_id) in self.distinct_threads() {
            let current = self
                .frames
                .iter()
                .filter(|f| f.session_id == session_id && f.thread_id == thread_id)
                .filter(|f| f.is_current_frame)
                .count();
            if current != 1 {
                return fail(format!(
                    "thread {session_id}/{thread_id} has {current} current frames, expected exactly 1"
                ));
            }
        }

        // Per-snapshot structural floor and cross-cutting rules.
        for sn in &self.snapshots {
            validate_snapshot(sn)
                .map_err(|reason| FrameVariableSnapshotSetValidationError { reason })?;
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

    fn distinct_threads(&self) -> Vec<(String, String)> {
        let mut seen: Vec<(String, String)> = Vec::new();
        for f in &self.frames {
            let key = (f.session_id.clone(), f.thread_id.clone());
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        seen
    }
}

fn validate_frame(fr: &FrameMapping) -> Result<(), String> {
    if fr.frame_id.is_empty() {
        return Err("frame has empty id".to_owned());
    }
    if fr.session_id.is_empty() || fr.thread_id.is_empty() {
        return Err(format!("frame {} has empty session/thread id", fr.frame_id));
    }
    if fr.function_label.is_empty() {
        return Err(format!("frame {} has empty function label", fr.frame_id));
    }
    if fr.proof_packet_ref.is_empty() {
        return Err(format!("frame {} has no proof packet", fr.frame_id));
    }
    if !frame_flags_consistent(fr) {
        return Err(format!(
            "frame {} computed flags or pill disagree with its enums",
            fr.frame_id
        ));
    }
    // Unmapped iff unresolved provenance.
    if fr.mapping_provenance.forces_unmapped() != (fr.fidelity() == FrameMappingFidelity::Unmapped)
    {
        return Err(format!(
            "frame {} unmapped state must match an unresolved provenance",
            fr.frame_id
        ));
    }
    // A source-map mapping always discloses its provenance.
    if fr.mapping_provenance.is_source_map()
        && (!fr.mapping_provenance_requires_disclosure || !fr.pill.label.contains("source-map"))
    {
        return Err(format!(
            "frame {} hides its source-map provenance",
            fr.frame_id
        ));
    }
    // A mapped (exact/approximate) frame carries a source ref and a line.
    if fr.fidelity().allows_source_navigation()
        && (fr.location.logical_source_ref.is_none() || fr.location.line.is_none())
    {
        return Err(format!(
            "frame {} claims a navigable mapping but has no source ref/line",
            fr.frame_id
        ));
    }
    // An exact source link implies an exact-build match.
    if fr.pill.shows_exact_source_link && !fr.build_identity.match_state.proves_exact_build() {
        return Err(format!(
            "frame {} shows an exact source link without an exact-build match",
            fr.frame_id
        ));
    }
    if !fr.build_identity.is_consistent() {
        return Err(format!(
            "frame {} build identity flags disagree with its match state",
            fr.frame_id
        ));
    }
    Ok(())
}

fn validate_snapshot(sn: &ValueSnapshot) -> Result<(), String> {
    if sn.snapshot_id.is_empty() {
        return Err("snapshot has empty id".to_owned());
    }
    if sn.display_name.is_empty() {
        return Err(format!(
            "snapshot {} has empty display name",
            sn.snapshot_id
        ));
    }
    if sn.capture_context.session_id.is_empty() {
        return Err(format!("snapshot {} has empty session id", sn.snapshot_id));
    }
    if sn.capture_context.captured_as_of.is_empty() {
        return Err(format!("snapshot {} has empty timestamp", sn.snapshot_id));
    }
    if sn.proof_packet_ref.is_empty() {
        return Err(format!("snapshot {} has no proof packet", sn.snapshot_id));
    }
    if !snapshot_flags_consistent(sn) {
        return Err(format!(
            "snapshot {} computed flags or pill disagree with its enums",
            sn.snapshot_id
        ));
    }
    if !sn.type_shape.is_consistent() {
        return Err(format!(
            "snapshot {} type/shape token disagrees with its shape",
            sn.snapshot_id
        ));
    }
    if !sn.truncation.is_consistent() {
        return Err(format!(
            "snapshot {} truncation flags disagree with its reason",
            sn.snapshot_id
        ));
    }
    // Watch entries use the watch scope and carry an expression digest; variables do not.
    match sn.entry_kind {
        SnapshotEntryKind::Watch => {
            if !sn.scope.is_watch_scope() || sn.watch_expression_digest.is_none() {
                return Err(format!(
                    "watch snapshot {} must use the watch scope and carry an expression digest",
                    sn.snapshot_id
                ));
            }
        }
        SnapshotEntryKind::Variable => {
            if sn.scope.is_watch_scope() || sn.watch_expression_digest.is_some() {
                return Err(format!(
                    "variable snapshot {} must not use the watch scope or carry an expression digest",
                    sn.snapshot_id
                ));
            }
        }
    }
    // Unavailable iff a reason is present and no value body.
    let is_unavailable = sn.disclosure_class() == ValueDisclosure::Unavailable;
    if is_unavailable != sn.unavailable_reason.is_some() {
        return Err(format!(
            "snapshot {} unavailable disclosure must match an unavailable reason",
            sn.snapshot_id
        ));
    }
    if sn.unavailable_reason_token.as_deref()
        != sn.unavailable_reason.map(VariableUnavailableReason::as_str)
    {
        return Err(format!(
            "snapshot {} unavailable reason token disagrees with its reason",
            sn.snapshot_id
        ));
    }
    // Redaction dominates: a redacted class reads as a redacted disclosure and withholds
    // the value body.
    if sn.redaction.is_redacted() != (sn.disclosure_class() == ValueDisclosure::Redacted) {
        return Err(format!(
            "snapshot {} redaction must match a redacted disclosure",
            sn.snapshot_id
        ));
    }
    // A value body is present exactly when the pill says so.
    if sn.disclosure.value_body_present != sn.value_repr_digest.is_some() {
        return Err(format!(
            "snapshot {} value body presence disagrees with its digest",
            sn.snapshot_id
        ));
    }
    Ok(())
}

fn frame_flags_consistent(fr: &FrameMapping) -> bool {
    fr.mapping_provenance_token == fr.mapping_provenance.as_str()
        && fr.mapping_provenance_requires_disclosure == fr.mapping_provenance.requires_disclosure()
        && fr.continuity_token == fr.continuity.as_str()
        && fr.is_async_boundary == fr.continuity.is_async_boundary()
        && fr.pill.is_async_boundary == fr.continuity.is_async_boundary()
        && fr.build_identity.is_consistent()
        && fr.pill.matches_derivation(
            fr.pill.fidelity,
            fr.build_identity.match_state,
            fr.mapping_provenance,
            fr.continuity,
        )
}

fn snapshot_flags_consistent(sn: &ValueSnapshot) -> bool {
    sn.entry_kind_token == sn.entry_kind.as_str()
        && sn.scope_token == sn.scope.as_str()
        && sn.redaction_token == sn.redaction.as_str()
        && sn
            .disclosure
            .matches_derivation(sn.disclosure.freshness, sn.redaction)
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

/// Builds the canonical M5 frame-mapping and variable/watch snapshot set.
///
/// Deterministic: the same bytes every call. Each invariant's `holds` flag is computed
/// from the built records, so an inconsistent edit flips an invariant rather than
/// silently passing.
pub fn m5_frame_variable_snapshot_set() -> FrameVariableSnapshotSet {
    let frames = build_frames();
    let snapshots = build_snapshots();
    let invariants = compute_invariants(&frames, &snapshots);

    FrameVariableSnapshotSet {
        record_kind: M5_FRAME_VARIABLE_SNAPSHOTS_RECORD_KIND.to_owned(),
        m5_frame_variable_snapshots_schema_version: M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_VERSION,
        schema_ref: M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_REF.to_owned(),
        set_id: M5_FRAME_VARIABLE_SNAPSHOTS_SET_ID.to_owned(),
        as_of: M5_FRAME_VARIABLE_SNAPSHOTS_AS_OF.to_owned(),
        freeze_gate_ref: M5_FRAME_VARIABLE_SNAPSHOTS_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set of M5 frame mappings and variable/watch snapshots. Every \
                  frame carries one pill that pins one mapping fidelity (exact, approximate, \
                  symbol-only, unmapped) and one build-match outcome, so a frame stack never \
                  flattens exact, approximate, symbol-only, and unresolved frames into one generic \
                  location link: a precise source link renders only for an exact mapping backed by \
                  an exact-build match, current-frame identity is preserved, a source-map mapping \
                  always discloses, a lost mapping degrades to an explicit unmapped frame, and an \
                  async/runtime boundary stays visible. Every value snapshot — variable or watch, \
                  live session, notebook cell, or replay capture — carries one disclosure pill that \
                  pins one of live, captured, stale, unavailable, or redacted, so a value is shown \
                  as a live read only when it truly is one, an unavailable value names its reason, \
                  a redacted value withholds its body, and a captured or stale value never implies \
                  live authority."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json",
            "schemas/debug/m5_debug_contracts.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs",
            "crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs",
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
        frames,
        snapshots,
        invariants,
        raw_payload_excluded: true,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

const LIVE_PROOF_REF: &str =
    "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json";
const MAPPING_STALE_PROOF_REF: &str =
    "fixtures/debug/mapping_cases/source_map_js_stale_mapping.json";
const MAPPING_UNKNOWN_PROOF_REF: &str =
    "fixtures/debug/mapping_cases/generated_source_spec_unknown.json";
const REPLAY_PROOF_REF: &str = "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json";
const NOTEBOOK_EXACT_PROOF_REF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_exact_match.json";
const NOTEBOOK_STALE_PROOF_REF: &str =
    "fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_stale.json";

const SESSION_MAIN: &str = "debug.session:local-launch:0001";
const THREAD_MAIN: &str = "debug.thread:main:0001";
const THREAD_WORKER: &str = "debug.thread:worker:0002";

fn source_location(
    logical: &str,
    line: u32,
    end_line: Option<u32>,
    column: Option<u32>,
) -> FrameSourceLocation {
    FrameSourceLocation {
        logical_source_ref: Some(logical.to_owned()),
        artifact_ref: None,
        line: Some(line),
        end_line,
        column,
    }
}

fn artifact_only_location(artifact: &str) -> FrameSourceLocation {
    FrameSourceLocation {
        logical_source_ref: None,
        artifact_ref: Some(artifact.to_owned()),
        line: None,
        end_line: None,
        column: None,
    }
}

fn build_frames() -> Vec<FrameMapping> {
    use BuildMatchClass::*;
    use FrameContinuityClass::*;
    use FrameMappingFidelity::*;
    use FrameMappingProvenance::*;

    vec![
        // 0. Current, top-of-stack frame: exact mapping with a verified build — the only
        //    state that renders the unqualified precise source link.
        FrameMapping::build(
            "debug.frame:main_exact_current:0001",
            SESSION_MAIN,
            THREAD_MAIN,
            0,
            "service_api::handler::handle_request",
            Some("symbol:digest:11aa22"),
            source_location(
                "source:logical:service-api/handler.rs#handle_request",
                142,
                None,
                Some(9),
            ),
            BuildArtifactIdentity::build(
                Some("build:digest:aa11bb"),
                Some("aureline://build/service-api/exact"),
                ExactBuildVerified,
            ),
            Exact,
            DirectSourceLine,
            Contiguous,
            true,
            false,
            LIVE_PROOF_REF,
            "Current top-of-stack frame mapped exactly to its source line against a verified \
             build — the only state that renders the unqualified precise source link.",
        ),
        // 1. Caller frame mapped only by a line-only heuristic against an approximate
        //    build candidate: navigable but disclosed as approximate.
        FrameMapping::build(
            "debug.frame:main_approx_heuristic:0002",
            SESSION_MAIN,
            THREAD_MAIN,
            1,
            "service_api::handler::dispatch",
            Some("symbol:digest:33cc44"),
            source_location(
                "source:logical:service-api/handler.rs#dispatch",
                88,
                Some(96),
                None,
            ),
            BuildArtifactIdentity::build(
                Some("build:digest:aa11bb"),
                Some("aureline://build/service-api/exact"),
                ApproximateCandidate,
            ),
            Approximate,
            HeuristicLineOnly,
            Contiguous,
            false,
            false,
            MAPPING_STALE_PROOF_REF,
            "Caller frame resolved by a line-only heuristic against an approximate build \
             candidate: navigable, but disclosed as approximate rather than drawn exact.",
        ),
        // 2. Selected frame mapped through a source map: an approximate generated-source
        //    mapping that always discloses its source-map provenance.
        FrameMapping::build(
            "debug.frame:main_sourcemap_selected:0003",
            SESSION_MAIN,
            THREAD_MAIN,
            2,
            "bundle::vendor::run",
            Some("symbol:digest:55dd66"),
            source_location("source:logical:web/app/vendor.ts#run", 1204, None, None),
            BuildArtifactIdentity::build(
                Some("build:digest:cc33dd"),
                Some("aureline://build/web-app/bundle"),
                ExactBuildVerified,
            ),
            Approximate,
            SourceMap,
            Contiguous,
            false,
            true,
            MAPPING_STALE_PROOF_REF,
            "User-selected frame mapped through a source map to the original generated source: \
             always disclosed as a source-map mapping, never flattened into a direct exact link.",
        ),
        // 3. Library frame with only a symbol name against a mismatched build: symbol-only,
        //    not navigable, build mismatch disclosed.
        FrameMapping::build(
            "debug.frame:main_symbol_only_mismatch:0004",
            SESSION_MAIN,
            THREAD_MAIN,
            3,
            "libssl::SSL_read",
            Some("symbol:digest:77ee88"),
            artifact_only_location("aureline://build/libssl/stripped"),
            BuildArtifactIdentity::build(
                Some("build:digest:ee77ff"),
                Some("aureline://build/libssl/stripped"),
                MismatchedRejected,
            ),
            SymbolOnly,
            SymbolTable,
            Contiguous,
            false,
            false,
            MAPPING_UNKNOWN_PROOF_REF,
            "Library frame resolved to a symbol name only against a mismatched build: not \
             navigable to source, with the build mismatch disclosed rather than hidden.",
        ),
        // 4. Async-resumption frame whose mapping could not be resolved: an explicit
        //    unmapped frame across an async boundary.
        FrameMapping::build(
            "debug.frame:main_unmapped_async:0005",
            SESSION_MAIN,
            THREAD_MAIN,
            4,
            "runtime::task::poll",
            None,
            artifact_only_location("aureline://build/async-runtime"),
            BuildArtifactIdentity::build(None, None, NoCandidate),
            Unmapped,
            Unresolved,
            AsyncResumption,
            false,
            false,
            MAPPING_UNKNOWN_PROOF_REF,
            "Async-resumption frame whose mapping could not be resolved: an explicit unmapped \
             frame across an async boundary, never drawn as a generic source location.",
        ),
        // 5. Frame at a runtime/FFI gap that still maps exactly to its own source: the
        //    exact link is allowed, but the async/runtime boundary stays disclosed.
        FrameMapping::build(
            "debug.frame:main_exact_runtime_gap:0006",
            SESSION_MAIN,
            THREAD_MAIN,
            5,
            "service_api::main",
            Some("symbol:digest:99ff00"),
            source_location("source:logical:service-api/main.rs#main", 12, None, None),
            BuildArtifactIdentity::build(
                Some("build:digest:aa11bb"),
                Some("aureline://build/service-api/exact"),
                ExactBuildVerified,
            ),
            Exact,
            DirectSourceLine,
            RuntimeGap,
            false,
            false,
            LIVE_PROOF_REF,
            "Entry frame across a runtime/FFI gap that still maps exactly to its own source: the \
             precise link is allowed while the boundary stays disclosed.",
        ),
        // 6. Current frame on a second (worker) thread: proves current-frame identity is
        //    preserved per thread, not flattened across the process.
        FrameMapping::build(
            "debug.frame:worker_exact_current:0007",
            SESSION_MAIN,
            THREAD_WORKER,
            0,
            "worker::queue::drain",
            Some("symbol:digest:abcd12"),
            source_location("source:logical:worker/queue.rs#drain", 64, None, None),
            BuildArtifactIdentity::build(
                Some("build:digest:aa11bb"),
                Some("aureline://build/service-api/exact"),
                ExactBuildVerified,
            ),
            Exact,
            DirectSourceLine,
            Contiguous,
            true,
            false,
            LIVE_PROOF_REF,
            "Current top-of-stack frame on the worker thread: current-frame identity is preserved \
             per thread rather than flattened across the process.",
        ),
    ]
}

fn capture_context(
    thread: Option<&str>,
    frame: Option<&str>,
    stop_seq: Option<u64>,
    notebook: Option<&str>,
    replay: Option<&str>,
) -> SnapshotCaptureContext {
    SnapshotCaptureContext {
        session_id: SESSION_MAIN.to_owned(),
        thread_id: thread.map(str::to_owned),
        frame_id: frame.map(str::to_owned),
        captured_as_of: M5_FRAME_VARIABLE_SNAPSHOTS_AS_OF.to_owned(),
        capture_stop_seq: stop_seq,
        notebook_cell_ref: notebook.map(str::to_owned),
        replay_capture_ref: replay.map(str::to_owned),
    }
}

fn build_snapshots() -> Vec<ValueSnapshot> {
    use SnapshotEntryKind::*;
    use TruncationReason::*;
    use ValueRedactionClass::*;
    use ValueShapeClass::*;
    use VariableFreshnessState::*;
    use VariableScopeClass::*;
    use VariableUnavailableReason::*;

    vec![
        // 1. Live local variable on the current frame: the clean live read.
        ValueSnapshot::build(
            "debug.snapshot:local_live:0001",
            Variable,
            "request_id",
            None,
            Local,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_exact_current:0001"), Some(7), None, None),
            TypeShapeSummary::build("u64", Scalar, None, Some("8 bytes")),
            ValueTruncation::complete(),
            Some("value:digest:1a2b3c"),
            false,
            None,
            NotRedacted,
            Live,
            LIVE_PROOF_REF,
            "Live read of a local scalar on the current frame — the clean live value at the \
             current stop.",
        ),
        // 2. Captured argument read from a replay capture, truncated: a captured snapshot,
        //    not a live read.
        ValueSnapshot::build(
            "debug.snapshot:arg_captured_replay:0002",
            Variable,
            "payload",
            None,
            Argument,
            capture_context(
                Some(THREAD_MAIN),
                Some("debug.frame:main_approx_heuristic:0002"),
                Some(4),
                None,
                Some("replay:capture:task-run-42"),
            ),
            TypeShapeSummary::build("String", Text, None, Some("first 256 of 4096 bytes")),
            ValueTruncation::truncated(StringLengthLimit, "first 256 bytes"),
            Some("value:digest:4d5e6f"),
            false,
            None,
            NotRedacted,
            CapturedSnapshot,
            REPLAY_PROOF_REF,
            "Captured argument read from a recorded replay capture and truncated to its first \
             bytes: a captured snapshot, disclosed as not a live read.",
        ),
        // 3. Stale closure value: the target resumed since it was captured.
        ValueSnapshot::build(
            "debug.snapshot:closure_stale:0003",
            Variable,
            "accumulator",
            None,
            Closure,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_approx_heuristic:0002"), Some(3), None, None),
            TypeShapeSummary::build("Vec<i64>", Collection, Some(12), Some("12 elements")),
            ValueTruncation::complete(),
            Some("value:digest:7a8b9c"),
            false,
            None,
            NotRedacted,
            Stale,
            MAPPING_STALE_PROOF_REF,
            "Last-known closure value captured before the target resumed: disclosed as stale, \
             never presented as the current live value.",
        ),
        // 4. Unavailable local: optimized out of the running build, no value body.
        ValueSnapshot::build(
            "debug.snapshot:local_optimized_out:0004",
            Variable,
            "scratch",
            None,
            Local,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_exact_current:0001"), Some(7), None, None),
            TypeShapeSummary::build("i32", Scalar, None, None),
            ValueTruncation::complete(),
            None,
            false,
            Some(OptimizedOut),
            NotRedacted,
            Unavailable,
            LIVE_PROOF_REF,
            "Local optimized out of the running build: unavailable with an explicit reason and no \
             value body, never shown as a readable value.",
        ),
        // 5. Live but lazy-loadable large collection, truncated by element count: a live
        //    read whose children load on demand.
        ValueSnapshot::build(
            "debug.snapshot:local_live_lazy:0005",
            Variable,
            "rows",
            None,
            Local,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_exact_current:0001"), Some(7), None, None),
            TypeShapeSummary::build("Vec<Row>", Collection, Some(10000), Some("10000 elements")),
            ValueTruncation::truncated(ElementCountLimit, "first 100 elements"),
            Some("value:digest:0f1e2d"),
            true,
            None,
            NotRedacted,
            Live,
            LIVE_PROOF_REF,
            "Live read of a large collection shown as a lazy, expandable handle truncated to its \
             first elements: a live value whose children load on demand.",
        ),
        // 6. Redacted global secret: the value body is withheld by a secret-redaction
        //    policy.
        ValueSnapshot::build(
            "debug.snapshot:global_secret_redacted:0006",
            Variable,
            "api_token",
            None,
            Global,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_exact_current:0001"), Some(7), None, None),
            TypeShapeSummary::build("SecretString", Opaque, None, None),
            ValueTruncation::complete(),
            None,
            false,
            None,
            SecretRedacted,
            Live,
            LIVE_PROOF_REF,
            "Global value matching a secret class: the body is withheld and disclosed as redacted, \
             so a credential never appears in a variables pane or an export.",
        ),
        // 7. Live watch expression shown in a notebook variable explorer: a watch reusing
        //    the same disclosure vocabulary as a variable.
        ValueSnapshot::build(
            "debug.snapshot:watch_live_notebook:0007",
            Watch,
            "df.shape",
            Some("watch:digest:bb55cc"),
            WatchExpression,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_exact_current:0001"), Some(7), Some("notebook:doc:analysis#cell-7"), None),
            TypeShapeSummary::build("tuple", Struct, Some(2), Some("2 fields")),
            ValueTruncation::complete(),
            Some("value:digest:cc66dd"),
            false,
            None,
            NotRedacted,
            Live,
            NOTEBOOK_EXACT_PROOF_REF,
            "Live watch expression evaluated in a notebook variable explorer: a watch reusing the \
             same live disclosure vocabulary as a variable, not a notebook-only truth.",
        ),
        // 8. Watch expression that failed to evaluate in a notebook: unavailable with an
        //    evaluation-error reason.
        ValueSnapshot::build(
            "debug.snapshot:watch_eval_error_notebook:0008",
            Watch,
            "model.predict(x)",
            Some("watch:digest:dd77ee"),
            WatchExpression,
            capture_context(Some(THREAD_MAIN), Some("debug.frame:main_approx_heuristic:0002"), Some(3), Some("notebook:doc:analysis#cell-12"), None),
            TypeShapeSummary::build("<unknown>", Opaque, None, None),
            ValueTruncation::complete(),
            None,
            false,
            Some(EvaluationError),
            NotRedacted,
            Unavailable,
            NOTEBOOK_STALE_PROOF_REF,
            "Watch expression that failed to evaluate in a notebook: unavailable with an \
             evaluation-error reason, sharing the same unavailable disclosure as a variable.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> SnapshotInvariant {
    SnapshotInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    frames: &[FrameMapping],
    snapshots: &[ValueSnapshot],
) -> Vec<SnapshotInvariant> {
    // Every frame carries one pill whose flags equal the derivation from its fidelity,
    // build match, provenance, and continuity.
    let frame_one_canonical_pill = frames.iter().all(|f| {
        f.pill.matches_derivation(
            f.pill.fidelity,
            f.build_identity.match_state,
            f.mapping_provenance,
            f.continuity,
        ) && f.pill.fidelity_token == f.pill.fidelity.as_str()
            && f.pill.build_match_token == f.pill.build_match.as_str()
    });

    // The full fidelity vocabulary is materialized.
    let fidelity_complete = FrameMappingFidelity::ALL
        .iter()
        .all(|fid| frames.iter().any(|f| f.fidelity() == *fid));

    // A precise source link renders only for an exact mapping backed by an exact-build
    // match; an approximate/symbol-only/unmapped or build-mismatched frame discloses, and
    // at least one disclosed frame exists.
    let exact_link_never_hides = frames.iter().all(|f| {
        f.pill.shows_exact_source_link
            == (f.fidelity().preserves_exact_source()
                && f.build_identity.match_state.proves_exact_build())
            && f.pill.requires_disclosure != f.pill.shows_exact_source_link
    }) && frames.iter().any(|f| {
        !f.pill.shows_exact_source_link
            && (f.fidelity() != FrameMappingFidelity::Exact
                || f.build_identity.match_state != BuildMatchClass::ExactBuildVerified)
    });

    // Current-frame identity is preserved: exactly one current frame per thread, and at
    // least one frame carries a distinct selected flag.
    let current_frame_preserved = {
        let mut threads: Vec<(&str, &str)> = Vec::new();
        for f in frames {
            let key = (f.session_id.as_str(), f.thread_id.as_str());
            if !threads.contains(&key) {
                threads.push(key);
            }
        }
        threads.iter().all(|(s, t)| {
            frames
                .iter()
                .filter(|f| f.session_id == *s && f.thread_id == *t && f.is_current_frame)
                .count()
                == 1
        }) && frames
            .iter()
            .any(|f| f.is_selected_frame && !f.is_current_frame)
    };

    // A frame is unmapped exactly when its provenance is unresolved: a lost mapping
    // degrades to an explicit unmapped frame, and a generic location is never shown.
    let unmapped_iff_unresolved = frames.iter().all(|f| {
        f.mapping_provenance.forces_unmapped() == (f.fidelity() == FrameMappingFidelity::Unmapped)
    }) && frames
        .iter()
        .any(|f| f.fidelity() == FrameMappingFidelity::Unmapped);

    // A source-map mapping always discloses its provenance and never flattens into a
    // direct exact link.
    let source_map_always_disclosed =
        frames.iter().all(|f| {
            if f.mapping_provenance.is_source_map() {
                f.mapping_provenance_requires_disclosure && f.pill.label.contains("source-map")
            } else {
                true
            }
        }) && frames.iter().any(|f| f.mapping_provenance.is_source_map());

    // Every frame across an async/runtime boundary discloses it; a contiguous frame never
    // falsely claims a boundary.
    let async_boundary_visible = frames.iter().all(|f| {
        f.is_async_boundary == f.continuity.is_async_boundary()
            && f.pill.is_async_boundary == f.continuity.is_async_boundary()
            && (!f.is_async_boundary || f.pill.label.contains("async boundary"))
    }) && frames.iter().any(|f| f.is_async_boundary);

    // Every frame preserves a build/artifact identity, and a precise source link implies
    // an exact-build match.
    let build_identity_preserved = frames.iter().all(|f| {
        f.build_identity.is_consistent()
            && (!f.pill.shows_exact_source_link
                || f.build_identity.match_state.proves_exact_build())
    });

    // The full disclosure vocabulary is materialized.
    let disclosure_complete = ValueDisclosure::ALL
        .iter()
        .all(|d| snapshots.iter().any(|s| s.disclosure_class() == *d));

    // Every snapshot carries one disclosure pill whose flags equal the derivation from
    // its freshness and redaction.
    let snapshot_one_canonical_pill = snapshots.iter().all(|s| {
        s.disclosure
            .matches_derivation(s.disclosure.freshness, s.redaction)
            && s.disclosure.freshness_token == s.disclosure.freshness.as_str()
            && s.disclosure.disclosure_token == s.disclosure.disclosure.as_str()
    });

    // A value implies live authority only when it is a true live read; captured, stale,
    // unavailable, and redacted values never do.
    let live_only_when_live = snapshots.iter().all(|s| {
        s.disclosure.implies_live_authority == (s.disclosure_class() == ValueDisclosure::Live)
            && s.disclosure.is_live_read == (s.disclosure_class() == ValueDisclosure::Live)
            && s.disclosure.requires_disclosure != s.disclosure.is_live_read
    }) && snapshots.iter().any(|s| {
        matches!(
            s.disclosure_class(),
            ValueDisclosure::Captured | ValueDisclosure::Stale
        ) && !s.disclosure.implies_live_authority
    });

    // Every unavailable snapshot names a reason and carries no value body; every
    // non-unavailable snapshot carries no reason.
    let unavailable_carries_reason = snapshots.iter().all(|s| {
        let is_unavailable = s.disclosure_class() == ValueDisclosure::Unavailable;
        is_unavailable == s.unavailable_reason.is_some()
            && (!is_unavailable || s.value_repr_digest.is_none())
    }) && snapshots
        .iter()
        .any(|s| s.disclosure_class() == ValueDisclosure::Unavailable);

    // Every redacted snapshot withholds its value body and reads as a redacted
    // disclosure; redaction dominates freshness.
    let redacted_withholds_body = snapshots.iter().all(|s| {
        s.redaction.is_redacted() == (s.disclosure_class() == ValueDisclosure::Redacted)
            && (!s.redaction.is_redacted()
                || (s.value_repr_digest.is_none() && s.disclosure.is_redacted))
    }) && snapshots.iter().any(|s| s.redaction.is_redacted());

    // Variables and watches share one disclosure vocabulary: both kinds are materialized,
    // watches use the watch scope with an expression digest, variables do not, and both
    // carry the same disclosure pill type drawn from the same vocabulary.
    let variables_and_watches_share_vocabulary = SnapshotEntryKind::ALL
        .iter()
        .all(|k| snapshots.iter().any(|s| s.entry_kind == *k))
        && snapshots.iter().all(|s| match s.entry_kind {
            SnapshotEntryKind::Watch => {
                s.scope.is_watch_scope() && s.watch_expression_digest.is_some()
            }
            SnapshotEntryKind::Variable => {
                !s.scope.is_watch_scope() && s.watch_expression_digest.is_none()
            }
        })
        && snapshots
            .iter()
            .all(|s| ValueDisclosure::ALL.contains(&s.disclosure_class()));

    // Notebook explorers and replay inspectors reuse the same snapshot vocabulary: a
    // notebook-context and a replay-context snapshot both exist and draw their disclosure
    // from the shared vocabulary rather than inventing surface-only truth.
    let notebook_and_replay_reuse_vocabulary = snapshots
        .iter()
        .any(|s| s.capture_context.notebook_cell_ref.is_some())
        && snapshots
            .iter()
            .any(|s| s.capture_context.replay_capture_ref.is_some())
        && snapshots
            .iter()
            .filter(|s| {
                s.capture_context.notebook_cell_ref.is_some()
                    || s.capture_context.replay_capture_ref.is_some()
            })
            .all(|s| ValueDisclosure::ALL.contains(&s.disclosure_class()));

    // Every frame and snapshot retains its typed tokens and cites an export-safe proof
    // packet, so export never flattens them into rendered chrome.
    let export_retains_state = frames.iter().all(|f| {
        !f.pill.fidelity_token.is_empty()
            && !f.proof_packet_ref.is_empty()
            && is_export_safe_ref(&f.proof_packet_ref)
    }) && snapshots.iter().all(|s| {
        !s.disclosure.disclosure_token.is_empty()
            && !s.proof_packet_ref.is_empty()
            && is_export_safe_ref(&s.proof_packet_ref)
    });

    vec![
        invariant(
            "frames.one_canonical_mapping_pill",
            "Every frame carries exactly one mapping pill whose fidelity and build-match tokens \
             come from the frozen vocabulary and whose flags equal their derivation.",
            frame_one_canonical_pill,
        ),
        invariant(
            "frames.fidelity_vocabulary_complete",
            "Exact, approximate, symbol-only, and unmapped are all materialized.",
            fidelity_complete,
        ),
        invariant(
            "frames.exact_link_never_hides_approximate_symbol_only_unmapped_or_mismatch",
            "The unqualified precise source link renders only for an exact mapping backed by an \
             exact-build match; an approximate, symbol-only, unmapped, or build-mismatched frame \
             always discloses.",
            exact_link_never_hides,
        ),
        invariant(
            "frames.preserve_current_frame_identity_per_thread",
            "Each thread has exactly one current frame, and the selected frame is tracked \
             distinctly, so current-frame identity is never flattened across the stack or process.",
            current_frame_preserved,
        ),
        invariant(
            "frames.lost_mapping_degrades_to_explicit_unmapped",
            "A frame is unmapped exactly when its provenance is unresolved, so a lost mapping \
             becomes an explicit unmapped frame rather than a generic guessed location.",
            unmapped_iff_unresolved,
        ),
        invariant(
            "frames.source_map_provenance_always_disclosed",
            "A source-map mapping always discloses its source-map provenance and is never \
             flattened into a direct exact link.",
            source_map_always_disclosed,
        ),
        invariant(
            "frames.async_boundary_stays_visible",
            "Every frame across an async or runtime boundary discloses it, and a contiguous frame \
             never falsely claims a boundary.",
            async_boundary_visible,
        ),
        invariant(
            "frames.build_artifact_identity_preserved",
            "Every frame preserves a build/artifact identity with a match state, and a precise \
             source link implies an exact-build match.",
            build_identity_preserved,
        ),
        invariant(
            "snapshots.disclosure_vocabulary_complete",
            "Live, captured, stale, unavailable, and redacted are all materialized.",
            disclosure_complete,
        ),
        invariant(
            "snapshots.one_canonical_disclosure_pill",
            "Every snapshot carries one disclosure pill whose freshness and disclosure tokens come \
             from the frozen vocabulary and whose flags equal their derivation.",
            snapshot_one_canonical_pill,
        ),
        invariant(
            "snapshots.live_authority_only_when_truly_live",
            "A value implies live authority only when it is a true live read; captured, stale, \
             unavailable, and redacted values never imply live authority.",
            live_only_when_live,
        ),
        invariant(
            "snapshots.unavailable_names_reason_and_withholds_body",
            "Every unavailable snapshot names a reason and carries no value body, and every \
             available snapshot carries no unavailable reason.",
            unavailable_carries_reason,
        ),
        invariant(
            "snapshots.redacted_withholds_value_body",
            "Every redacted snapshot withholds its value body and reads as a redacted disclosure; \
             redaction dominates freshness.",
            redacted_withholds_body,
        ),
        invariant(
            "snapshots.variables_and_watches_share_one_vocabulary",
            "Variables and watches are both materialized and share one disclosure vocabulary; \
             watches use the watch scope with an expression digest, variables do not.",
            variables_and_watches_share_vocabulary,
        ),
        invariant(
            "snapshots.notebook_and_replay_reuse_snapshot_vocabulary",
            "Notebook explorers and replay inspectors both exist and reuse the shared snapshot \
             disclosure vocabulary instead of inventing notebook-only or replay-only truth.",
            notebook_and_replay_reuse_vocabulary,
        ),
        invariant(
            "set.export_retains_frame_and_value_state",
            "Every frame and snapshot retains its typed fidelity/freshness tokens and cites an \
             export-safe proof packet, so support export never flattens it into rendered chrome.",
            export_retains_state,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the frame/variable snapshot set as human-readable lines for CLI/headless and
/// support.
pub fn m5_frame_variable_snapshot_lines(set: &FrameVariableSnapshotSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 frame mappings & variable/watch snapshots — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Frames: {}  Snapshots: {}  Invariants: {}",
        set.frames.len(),
        set.snapshots.len(),
        set.invariants.len(),
    ));

    lines.push("Frames:".to_owned());
    for fr in &set.frames {
        lines.push(format!(
            "  - {} [{}/{} #{}] pill={} exact_link={} async_boundary={} current={} selected={}",
            fr.frame_id,
            fr.session_id,
            fr.thread_id,
            fr.frame_index,
            fr.pill.label,
            fr.pill.shows_exact_source_link,
            fr.is_async_boundary,
            fr.is_current_frame,
            fr.is_selected_frame,
        ));
        lines.push(format!(
            "      fidelity={} provenance={} build_match={}",
            fr.pill.fidelity_token, fr.mapping_provenance_token, fr.pill.build_match_token,
        ));
        lines.push(format!("      {}", fr.summary));
        lines.push(format!("      proof: {}", fr.proof_packet_ref));
    }

    lines.push("Snapshots:".to_owned());
    for sn in &set.snapshots {
        lines.push(format!(
            "  - {} [{}] {} scope={} disclosure={} live={} value_present={} redacted={} lazy={}",
            sn.snapshot_id,
            sn.entry_kind_token,
            sn.display_name,
            sn.scope_token,
            sn.disclosure.disclosure_token,
            sn.disclosure.is_live_read,
            sn.disclosure.value_body_present,
            sn.disclosure.is_redacted,
            sn.lazy_loadable,
        ));
        lines.push(format!(
            "      type={} shape={} truncated={} unavailable_reason={}",
            sn.type_shape.type_name,
            sn.type_shape.shape_token,
            sn.truncation.is_truncated,
            sn.unavailable_reason_token.as_deref().unwrap_or("-"),
        ));
        lines.push(format!("      {}", sn.summary));
        lines.push(format!("      proof: {}", sn.proof_packet_ref));
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
