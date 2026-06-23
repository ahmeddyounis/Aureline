//! Typed breakpoint specs and mapping-state pills: the canonical M5 record every
//! debugger-capable surface reads to show what a breakpoint requested, where it
//! actually bound, and whether its source mapping is still trustworthy.
//!
//! The [`m5_debug_contracts`](crate::m5_debug_contracts) matrix *names* the debugger
//! object families and freezes their vocabulary; the
//! [`m5_debug_session_descriptors`](crate::m5_debug_session_descriptors) lane
//! materializes the session and attach-target families. This lane *materializes* the
//! [`DebugObjectClass::BreakpointSpec`](crate::m5_debug_contracts::DebugObjectClass::BreakpointSpec)
//! family as concrete, serde-serializable [`BreakpointSpec`] records and freezes a
//! canonical [`BreakpointSpecSet`] that carries the full
//! pending/verified/misaligned/unbound/unsupported/policy-blocked/needs-remap truth.
//!
//! Breakpoint truth stays explicit and replay-safe:
//!
//! - **One canonical pill vocabulary.** Every breakpoint carries one
//!   [`BreakpointPill`] that pins one [`BreakpointVerificationState`] (pending,
//!   verified, unbound, unsupported, policy-blocked) and one
//!   [`BreakpointMappingState`] (exact, misaligned, needs-remap, unmapped). A
//!   breakpoint shown in a gutter, a session header, a list, a notebook cell, a
//!   replay timeline, or an export packet renders the *same* pill, so it always
//!   traces back to one spec and one state vocabulary.
//! - **A green gutter icon never hides a caveat.** The
//!   [`shows_clean_confirmed`](BreakpointPill::shows_clean_confirmed) flag — the only
//!   state allowed to render the unqualified confirmed-stop icon — is derived to be
//!   true only when the breakpoint is verified *and* its mapping is exact *and* it is
//!   not replay-only. An unbound, misaligned, replay-only, or policy-blocked
//!   breakpoint always
//!   [`requires_disclosure`](BreakpointPill::requires_disclosure).
//! - **Identity survives rename/reformat/import, or degrades to needs-remap.** A
//!   [`BreakpointSourceAnchor`] keeps a stable `logical_source_ref` so a breakpoint
//!   survives a rename or reformat where stable source identity exists. When it does
//!   not, the [`BreakpointMappingProvenance::SourceIdentityLost`] provenance forces
//!   [`BreakpointMappingState::NeedsRemap`] — the breakpoint stays visible and asks
//!   for an explicit remap rather than silently disappearing.
//! - **A lexical fallback is never replayed as semantic.** A
//!   [`BreakpointMappingProvenance::LexicalFallback`] mapping can never present as an
//!   exact mapping; it always discloses that it is a textual match, not a semantic
//!   one.
//! - **Notebook and replay views keep stable identity.** A notebook-scoped breakpoint
//!   carries a [`NotebookCellAnchor`] and a replay-scoped breakpoint carries a
//!   [`ReplayFrameAnchor`], so cell and frame identity are never collapsed, and a
//!   replay-scoped breakpoint stays [`is_replay_only`](BreakpointPill::is_replay_only)
//!   and never poses as a live confirmed stop.
//!
//! [`m5_breakpoint_spec_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`BreakpointInvariant`]'s `holds` flag from
//! the built specs, so the checked-in fixture and the freeze gate freeze the contract
//! byte-for-byte and an inconsistent edit flips an invariant and fails CI. The record
//! carries no source bodies, raw paths, provider payloads, URLs, hostnames, or
//! credentials — only opaque object refs, stable tokens, opaque digests, and short
//! reviewable sentences — so it is safe for support export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_breakpoint_specs.schema.json`](../../../schemas/debug/m5_breakpoint_specs.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_breakpoint_specs/canonical_set.json`](../../../fixtures/debug/m5_breakpoint_specs/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_breakpoint_specs.md`](../../../docs/debug/m5_breakpoint_specs.md).

use serde::{Deserialize, Serialize};

use crate::m5_debug_contracts::DebugConsumer;

#[cfg(test)]
mod tests;

/// Schema version for the M5 breakpoint-spec set.
pub const M5_BREAKPOINT_SPECS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 breakpoint-spec set.
pub const M5_BREAKPOINT_SPECS_SCHEMA_REF: &str = "schemas/debug/m5_breakpoint_specs.schema.json";

/// Stable record-kind tag for the breakpoint-spec set.
pub const M5_BREAKPOINT_SPECS_RECORD_KIND: &str = "m5_breakpoint_spec_set";

/// Stable id for the canonical breakpoint-spec set.
pub const M5_BREAKPOINT_SPECS_SET_ID: &str = "m5-breakpoint-specs:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_BREAKPOINT_SPECS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the breakpoint-spec set current. Stable promotion runs
/// this gate; it fails when the in-code set drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_BREAKPOINT_SPECS_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_breakpoint_specs.rs";

/// The checked-in canonical breakpoint-spec-set fixture.
pub const M5_BREAKPOINT_SPECS_FIXTURE_REF: &str =
    "fixtures/debug/m5_breakpoint_specs/canonical_set.json";

/// The contract narrative document.
pub const M5_BREAKPOINT_SPECS_DOC_REF: &str = "docs/debug/m5_breakpoint_specs.md";

/// The human-readable evidence companion artifact.
pub const M5_BREAKPOINT_SPECS_ARTIFACT_REF: &str = "artifacts/debug/m5_breakpoint_specs.md";

// ---------------------------------------------------------------------------
// Breakpoint kind.
// ---------------------------------------------------------------------------

/// The kind of breakpoint requested.
///
/// All kinds but [`BreakpointKindClass::Exception`] target a concrete source
/// location; an exception breakpoint applies to an exception category and carries no
/// authoritative source line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointKindClass {
    /// A plain source-line breakpoint.
    Line,
    /// A source-line breakpoint guarded by a condition expression.
    Conditional,
    /// A logpoint that logs a message instead of stopping.
    Logpoint,
    /// A function / method-entry breakpoint.
    Function,
    /// A data / watchpoint on a memory location or variable.
    Data,
    /// An exception breakpoint scoped to an exception category.
    Exception,
}

impl BreakpointKindClass {
    /// All breakpoint kinds, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Line,
        Self::Conditional,
        Self::Logpoint,
        Self::Function,
        Self::Data,
        Self::Exception,
    ];

    /// Stable snake_case token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Conditional => "conditional",
            Self::Logpoint => "logpoint",
            Self::Function => "function",
            Self::Data => "data",
            Self::Exception => "exception",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Line => "Line",
            Self::Conditional => "Conditional",
            Self::Logpoint => "Logpoint",
            Self::Function => "Function",
            Self::Data => "Data / watchpoint",
            Self::Exception => "Exception",
        }
    }

    /// Whether this kind targets a concrete source location. An exception breakpoint
    /// does not — it applies to an exception category, so an unmapped state is honest
    /// rather than a defect.
    pub const fn targets_source_location(self) -> bool {
        !matches!(self, Self::Exception)
    }
}

// ---------------------------------------------------------------------------
// Enablement.
// ---------------------------------------------------------------------------

/// Whether the user has the breakpoint enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointEnablement {
    /// The breakpoint is enabled and active.
    Enabled,
    /// The breakpoint is disabled and will not stop the target.
    Disabled,
}

impl BreakpointEnablement {
    /// All enablement states, in canonical order.
    pub const ALL: [Self; 2] = [Self::Enabled, Self::Disabled];

    /// Stable snake_case token for this enablement state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }

    /// Whether the breakpoint is active.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

// ---------------------------------------------------------------------------
// Verification state.
// ---------------------------------------------------------------------------

/// The verification state of a breakpoint: whether, and why, it bound at the target.
///
/// Only [`BreakpointVerificationState::Verified`] is a confirmed binding; every other
/// state requires a visible caveat so a breakpoint that never bound is not drawn as a
/// confirmed stop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointVerificationState {
    /// Accepted by Aureline but not yet confirmed bound by the target.
    Pending,
    /// Verified and bound at its requested or adjusted location.
    Verified,
    /// Could not bind at the target and remains unverified / unbound.
    Unbound,
    /// The target or adapter does not support this breakpoint kind.
    Unsupported,
    /// Policy forbids setting this breakpoint on the target.
    PolicyBlocked,
}

impl BreakpointVerificationState {
    /// All verification states, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Pending,
        Self::Verified,
        Self::Unbound,
        Self::Unsupported,
        Self::PolicyBlocked,
    ];

    /// Stable snake_case token for this verification state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Unbound => "unbound",
            Self::Unsupported => "unsupported",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Verified => "Verified",
            Self::Unbound => "Unbound",
            Self::Unsupported => "Unsupported",
            Self::PolicyBlocked => "Policy-blocked",
        }
    }

    /// Whether this state is a confirmed binding at the target.
    pub const fn is_bound(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Whether this state must render with a visible caveat: anything but a confirmed
    /// binding cannot be shown as an unquestioned stop.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Verified)
    }

    /// Whether the breakpoint was actively blocked rather than merely unbound — the
    /// target/adapter refused it or policy forbids it.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Unsupported | Self::PolicyBlocked)
    }
}

// ---------------------------------------------------------------------------
// Mapping state.
// ---------------------------------------------------------------------------

/// The mapping state of a breakpoint: whether its requested source location still maps
/// trustworthily to where it would bind after edits, renames, or imports.
///
/// Only [`BreakpointMappingState::Exact`] preserves the requested location exactly;
/// every other state requires a visible caveat, and
/// [`BreakpointMappingState::NeedsRemap`] is the explicit degrade-rather-than-vanish
/// state when stable source identity is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointMappingState {
    /// The requested location maps exactly to where the breakpoint bound.
    Exact,
    /// The breakpoint bound at an adjusted location, disclosed as relocated.
    Misaligned,
    /// Stable source identity was lost (rename/reformat/import); the breakpoint stays
    /// visible and needs an explicit remap.
    NeedsRemap,
    /// No source mapping is available (e.g. an exception category or a capture with no
    /// source).
    Unmapped,
}

impl BreakpointMappingState {
    /// All mapping states, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Misaligned,
        Self::NeedsRemap,
        Self::Unmapped,
    ];

    /// Stable snake_case token for this mapping state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Misaligned => "misaligned",
            Self::NeedsRemap => "needs_remap",
            Self::Unmapped => "unmapped",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Misaligned => "Misaligned",
            Self::NeedsRemap => "Needs remap",
            Self::Unmapped => "Unmapped",
        }
    }

    /// Short label used as the mapping caveat in a pill (e.g. `Verified · relocated`).
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Misaligned => "relocated",
            Self::NeedsRemap => "needs remap",
            Self::Unmapped => "no source mapping",
        }
    }

    /// Whether this state preserves the requested location exactly.
    pub const fn preserves_exact_location(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Whether this state must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact)
    }

    /// Whether this state asks for an explicit remap rather than a silent jump.
    pub const fn needs_explicit_remap(self) -> bool {
        matches!(self, Self::NeedsRemap)
    }
}

// ---------------------------------------------------------------------------
// Scope.
// ---------------------------------------------------------------------------

/// The scope a breakpoint applies in — which keeps notebook and replay views from
/// collapsing stable cell or frame identity into a generic source breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointScopeClass {
    /// Anchored to a workspace source file and shared across sessions.
    WorkspaceSource,
    /// Scoped to a single live debug session.
    SessionLocal,
    /// Anchored to a notebook cell.
    NotebookCell,
    /// Scoped to a replay capture's timeline.
    ReplayTimeline,
    /// An exception breakpoint scoped to an exception category, not a source location.
    ExceptionCategory,
}

impl BreakpointScopeClass {
    /// All scopes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::WorkspaceSource,
        Self::SessionLocal,
        Self::NotebookCell,
        Self::ReplayTimeline,
        Self::ExceptionCategory,
    ];

    /// Stable snake_case token for this scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceSource => "workspace_source",
            Self::SessionLocal => "session_local",
            Self::NotebookCell => "notebook_cell",
            Self::ReplayTimeline => "replay_timeline",
            Self::ExceptionCategory => "exception_category",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceSource => "Workspace source",
            Self::SessionLocal => "Session-local",
            Self::NotebookCell => "Notebook cell",
            Self::ReplayTimeline => "Replay timeline",
            Self::ExceptionCategory => "Exception category",
        }
    }

    /// Whether breakpoints in this scope are replay-only — observed against a recorded
    /// capture and never a live confirmed stop.
    pub const fn is_replay_only(self) -> bool {
        matches!(self, Self::ReplayTimeline)
    }

    /// Whether breakpoints in this scope must carry a [`NotebookCellAnchor`] so cell
    /// identity is never collapsed.
    pub const fn requires_notebook_anchor(self) -> bool {
        matches!(self, Self::NotebookCell)
    }

    /// Whether breakpoints in this scope must carry a [`ReplayFrameAnchor`] so frame
    /// identity is never collapsed.
    pub const fn requires_replay_anchor(self) -> bool {
        matches!(self, Self::ReplayTimeline)
    }
}

// ---------------------------------------------------------------------------
// Mapping provenance.
// ---------------------------------------------------------------------------

/// How a breakpoint's current mapping was derived — so a textual fallback is never
/// replayed as a semantic match and a lost-identity case is named explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointMappingProvenance {
    /// A stable logical source identity survived a rename or move.
    StableSourceId,
    /// Re-resolved after a reformat using span heuristics over the same source id.
    ReResolvedAfterReformat,
    /// Derived from an imported source map or imported session.
    ImportedSourceMap,
    /// Only a lexical / textual match was available; never an exact semantic mapping.
    LexicalFallback,
    /// Stable source identity could not be recovered; forces an explicit remap.
    SourceIdentityLost,
}

impl BreakpointMappingProvenance {
    /// All provenance classes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::StableSourceId,
        Self::ReResolvedAfterReformat,
        Self::ImportedSourceMap,
        Self::LexicalFallback,
        Self::SourceIdentityLost,
    ];

    /// Stable snake_case token for this provenance class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableSourceId => "stable_source_id",
            Self::ReResolvedAfterReformat => "re_resolved_after_reformat",
            Self::ImportedSourceMap => "imported_source_map",
            Self::LexicalFallback => "lexical_fallback",
            Self::SourceIdentityLost => "source_identity_lost",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StableSourceId => "Stable source identity",
            Self::ReResolvedAfterReformat => "Re-resolved after reformat",
            Self::ImportedSourceMap => "Imported source map",
            Self::LexicalFallback => "Lexical fallback",
            Self::SourceIdentityLost => "Source identity lost",
        }
    }

    /// Whether this provenance is a semantic mapping rather than a textual guess. A
    /// lexical fallback is the only non-semantic class.
    pub const fn is_semantic(self) -> bool {
        !matches!(self, Self::LexicalFallback)
    }

    /// Whether this provenance forces a [`BreakpointMappingState::NeedsRemap`]: a lost
    /// source identity cannot resolve to any trusted location.
    pub const fn forces_needs_remap(self) -> bool {
        matches!(self, Self::SourceIdentityLost)
    }

    /// Whether this provenance must render with a visible caveat. Only an intact stable
    /// source identity needs none.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::StableSourceId)
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// The source location a breakpoint requested — carrying a stable logical source ref
/// that survives a rename, plus an opaque physical-path hint and a line/span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointSourceAnchor {
    /// Stable logical source identity that survives renames and moves, e.g.
    /// `source:logical:service-api/handler.rs#handle_request`.
    pub logical_source_ref: String,
    /// Opaque digest of the resolved physical path, never a raw path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_path_hint: Option<String>,
    /// One-based start line of the requested location.
    pub line: u32,
    /// One-based end line of the requested span, when the breakpoint spans more than
    /// one line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    /// One-based start column, when the location is column-precise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
}

/// The notebook cell a notebook-scoped breakpoint is anchored to, preserving stable
/// cell identity through cell shifts and re-execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCellAnchor {
    /// Stable notebook document ref.
    pub notebook_ref: String,
    /// Stable cell id that survives cell reordering and re-execution.
    pub cell_id: String,
    /// Opaque ref to the cell execution the breakpoint last bound against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_execution_ref: Option<String>,
}

/// The replay frame a replay-scoped breakpoint is anchored to, preserving stable frame
/// identity within a recorded capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayFrameAnchor {
    /// Stable replay capture ref.
    pub capture_ref: String,
    /// Stable timeline position ref within the capture.
    pub timeline_ref: String,
    /// Stable frame ref the breakpoint matched during replay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_ref: Option<String>,
}

/// The condition / log / hit-count payload attached to a breakpoint. Bodies are
/// carried as opaque digests and presence flags only, never as raw expressions, so the
/// record is safe for support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointPayload {
    /// Whether the breakpoint carries a condition expression.
    pub has_condition: bool,
    /// Opaque digest of the condition expression, never the raw source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_digest: Option<String>,
    /// Whether the breakpoint carries a log message (logpoint).
    pub has_log_message: bool,
    /// Opaque digest of the log message template, never the raw source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_message_digest: Option<String>,
    /// Structured hit-count condition such as `>= 3`, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
}

impl BreakpointPayload {
    /// An empty payload: a plain breakpoint with no condition, log message, or hit
    /// count.
    pub fn none() -> Self {
        Self {
            has_condition: false,
            condition_digest: None,
            has_log_message: false,
            log_message_digest: None,
            hit_condition: None,
        }
    }

    /// Whether the presence flags agree with the carried digests.
    pub fn is_consistent(&self) -> bool {
        self.has_condition == self.condition_digest.is_some()
            && self.has_log_message == self.log_message_digest.is_some()
    }
}

/// The single canonical pill every surface renders for a breakpoint — the one
/// verification + mapping state vocabulary a gutter, list, header, notebook cell,
/// replay timeline, or export packet reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointPill {
    /// The verification state.
    pub verification: BreakpointVerificationState,
    /// Stable token for the verification state.
    pub verification_token: String,
    /// The mapping state.
    pub mapping: BreakpointMappingState,
    /// Stable token for the mapping state.
    pub mapping_token: String,
    /// One reviewable pill label combining verification and mapping.
    pub label: String,
    /// Whether the breakpoint is a confirmed binding at the target.
    pub is_confirmed_binding: bool,
    /// Whether the breakpoint may render the unqualified confirmed-stop icon — true
    /// only when verified, exact, and not replay-only.
    pub shows_clean_confirmed: bool,
    /// Whether the breakpoint must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether the breakpoint asks for an explicit remap rather than a silent jump.
    pub needs_explicit_remap: bool,
    /// Whether the breakpoint is replay-only and never a live confirmed stop.
    pub is_replay_only: bool,
}

impl BreakpointPill {
    /// Whether a breakpoint may render the unqualified confirmed-stop icon: only a
    /// verified binding with an exact mapping that is not replay-only. This is the
    /// guardrail that keeps a green gutter icon from hiding an unbound, misaligned,
    /// replay-only, or policy-blocked reality.
    pub const fn derive_shows_clean_confirmed(
        verification: BreakpointVerificationState,
        mapping: BreakpointMappingState,
        scope: BreakpointScopeClass,
    ) -> bool {
        verification.is_bound() && mapping.preserves_exact_location() && !scope.is_replay_only()
    }

    /// Builds the canonical pill for a breakpoint, deriving every flag and the label
    /// from the verification state, mapping state, and scope so the pill cannot
    /// disagree with itself.
    pub fn derive(
        verification: BreakpointVerificationState,
        mapping: BreakpointMappingState,
        scope: BreakpointScopeClass,
    ) -> Self {
        let shows_clean_confirmed =
            Self::derive_shows_clean_confirmed(verification, mapping, scope);
        let mut label = verification.label().to_owned();
        if !mapping.preserves_exact_location() {
            label.push_str(" · ");
            label.push_str(mapping.short_label());
        }
        if scope.is_replay_only() {
            label.push_str(" · replay-only");
        }
        Self {
            verification,
            verification_token: verification.as_str().to_owned(),
            mapping,
            mapping_token: mapping.as_str().to_owned(),
            label,
            is_confirmed_binding: verification.is_bound(),
            shows_clean_confirmed,
            requires_disclosure: !shows_clean_confirmed,
            needs_explicit_remap: mapping.needs_explicit_remap(),
            is_replay_only: scope.is_replay_only(),
        }
    }

    /// Whether this pill equals the canonical derivation for the given inputs.
    pub fn matches_derivation(
        &self,
        verification: BreakpointVerificationState,
        mapping: BreakpointMappingState,
        scope: BreakpointScopeClass,
    ) -> bool {
        *self == Self::derive(verification, mapping, scope)
    }
}

/// A typed breakpoint spec: the canonical record every debugger-capable surface reads
/// to show one requested breakpoint, where it bound, and whether its source mapping is
/// still trustworthy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointSpec {
    /// Stable, namespaced breakpoint id.
    pub breakpoint_id: String,
    /// The breakpoint kind.
    pub kind: BreakpointKindClass,
    /// Stable token for the kind.
    pub kind_token: String,
    /// Whether the kind targets a concrete source location.
    pub kind_targets_source_location: bool,
    /// Whether the user has the breakpoint enabled.
    pub enablement: BreakpointEnablement,
    /// Stable token for the enablement state.
    pub enablement_token: String,
    /// Whether the breakpoint is active.
    pub enablement_is_active: bool,
    /// The requested source location with its stable logical source ref.
    pub source_anchor: BreakpointSourceAnchor,
    /// The notebook cell anchor, when the breakpoint is notebook-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notebook_anchor: Option<NotebookCellAnchor>,
    /// The replay frame anchor, when the breakpoint is replay-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_anchor: Option<ReplayFrameAnchor>,
    /// The condition / log / hit-count payload.
    pub payload: BreakpointPayload,
    /// The scope the breakpoint applies in.
    pub scope: BreakpointScopeClass,
    /// Stable token for the scope.
    pub scope_token: String,
    /// How the current mapping was derived.
    pub mapping_provenance: BreakpointMappingProvenance,
    /// Stable token for the mapping provenance.
    pub mapping_provenance_token: String,
    /// Whether the mapping provenance is a semantic mapping rather than a textual guess.
    pub mapping_provenance_is_semantic: bool,
    /// Whether the mapping provenance must be disclosed.
    pub mapping_provenance_requires_disclosure: bool,
    /// The canonical verification + mapping pill every surface renders.
    pub pill: BreakpointPill,
    /// The proof packet that keeps this breakpoint spec current.
    pub proof_packet_ref: String,
    /// One reviewable export-safe sentence describing the breakpoint.
    pub summary: String,
}

impl BreakpointSpec {
    /// Builds a breakpoint spec, deriving every computed token, honesty flag, and the
    /// pill from the typed enums so the record cannot disagree with itself.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        breakpoint_id: impl Into<String>,
        kind: BreakpointKindClass,
        enablement: BreakpointEnablement,
        source_anchor: BreakpointSourceAnchor,
        notebook_anchor: Option<NotebookCellAnchor>,
        replay_anchor: Option<ReplayFrameAnchor>,
        payload: BreakpointPayload,
        scope: BreakpointScopeClass,
        mapping_provenance: BreakpointMappingProvenance,
        verification: BreakpointVerificationState,
        mapping: BreakpointMappingState,
        proof_packet_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            breakpoint_id: breakpoint_id.into(),
            kind,
            kind_token: kind.as_str().to_owned(),
            kind_targets_source_location: kind.targets_source_location(),
            enablement,
            enablement_token: enablement.as_str().to_owned(),
            enablement_is_active: enablement.is_active(),
            source_anchor,
            notebook_anchor,
            replay_anchor,
            payload,
            scope,
            scope_token: scope.as_str().to_owned(),
            mapping_provenance,
            mapping_provenance_token: mapping_provenance.as_str().to_owned(),
            mapping_provenance_is_semantic: mapping_provenance.is_semantic(),
            mapping_provenance_requires_disclosure: mapping_provenance.requires_disclosure(),
            pill: BreakpointPill::derive(verification, mapping, scope),
            proof_packet_ref: proof_packet_ref.into(),
            summary: summary.into(),
        }
    }

    /// The verification state from the pill.
    pub const fn verification(&self) -> BreakpointVerificationState {
        self.pill.verification
    }

    /// The mapping state from the pill.
    pub const fn mapping(&self) -> BreakpointMappingState {
        self.pill.mapping
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 breakpoint-spec set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointSpecSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_breakpoint_specs_schema_version: u32,
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
    /// The surfaces that consume the breakpoint specs.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The breakpoint specs.
    pub breakpoints: Vec<BreakpointSpec>,
    /// The computed invariants.
    pub invariants: Vec<BreakpointInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the breakpoint-spec set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakpointSpecSetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for BreakpointSpecSetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m5 breakpoint-spec set invalid: {}", self.reason)
    }
}

impl std::error::Error for BreakpointSpecSetValidationError {}

impl BreakpointSpecSet {
    /// Returns the breakpoint spec with the given id, if present.
    pub fn breakpoint(&self, breakpoint_id: &str) -> Option<&BreakpointSpec> {
        self.breakpoints
            .iter()
            .find(|b| b.breakpoint_id == breakpoint_id)
    }

    /// Returns the first breakpoint in the given verification state, if present.
    pub fn in_verification_state(
        &self,
        verification: BreakpointVerificationState,
    ) -> Option<&BreakpointSpec> {
        self.breakpoints
            .iter()
            .find(|b| b.verification() == verification)
    }

    /// Returns the first breakpoint in the given mapping state, if present.
    pub fn in_mapping_state(&self, mapping: BreakpointMappingState) -> Option<&BreakpointSpec> {
        self.breakpoints.iter().find(|b| b.mapping() == mapping)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
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
        let from_breakpoints = self.breakpoints.iter().map(|b| b.proof_packet_ref.as_str());
        from_set.chain(from_breakpoints)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns a [`BreakpointSpecSetValidationError`] when an identifier, a ref, a
    /// computed flag, a pill, a provenance rule, an anchor, or an invariant is
    /// inconsistent.
    pub fn validate(&self) -> Result<(), BreakpointSpecSetValidationError> {
        let fail = |reason: String| Err(BreakpointSpecSetValidationError { reason });

        if self.record_kind != M5_BREAKPOINT_SPECS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_BREAKPOINT_SPECS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_breakpoint_specs_schema_version != M5_BREAKPOINT_SPECS_SCHEMA_VERSION {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.breakpoints.is_empty() {
            return fail("no breakpoints".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.breakpoints.iter().map(|b| b.breakpoint_id.as_str())) {
            return fail("breakpoint ids are not unique".to_owned());
        }

        // The full verification and mapping vocabulary is materialized.
        for state in BreakpointVerificationState::ALL {
            if self.in_verification_state(state).is_none() {
                return fail(format!(
                    "verification state {} is not materialized",
                    state.as_str()
                ));
            }
        }
        for mapping in BreakpointMappingState::ALL {
            if self.in_mapping_state(mapping).is_none() {
                return fail(format!(
                    "mapping state {} is not materialized",
                    mapping.as_str()
                ));
            }
        }
        for scope in BreakpointScopeClass::ALL {
            if !self.breakpoints.iter().any(|b| b.scope == scope) {
                return fail(format!("scope {} is not materialized", scope.as_str()));
            }
        }

        // Per-breakpoint structural floor and cross-cutting rules.
        for bp in &self.breakpoints {
            if bp.breakpoint_id.is_empty() {
                return fail("breakpoint has empty id".to_owned());
            }
            if bp.source_anchor.logical_source_ref.is_empty() {
                return fail(format!(
                    "breakpoint {} has no logical source ref",
                    bp.breakpoint_id
                ));
            }
            if bp.proof_packet_ref.is_empty() {
                return fail(format!(
                    "breakpoint {} has no proof packet",
                    bp.breakpoint_id
                ));
            }
            if !bp.payload.is_consistent() {
                return fail(format!(
                    "breakpoint {} payload flags disagree with its digests",
                    bp.breakpoint_id
                ));
            }
            if !bp_flags_consistent(bp) {
                return fail(format!(
                    "breakpoint {} computed flags or pill disagree with its enums",
                    bp.breakpoint_id
                ));
            }
            // A lost source identity degrades to needs-remap, and a needs-remap state
            // only ever comes from a lost source identity.
            if bp.mapping_provenance.forces_needs_remap()
                != (bp.mapping() == BreakpointMappingState::NeedsRemap)
            {
                return fail(format!(
                    "breakpoint {} needs-remap state must match a lost source identity",
                    bp.breakpoint_id
                ));
            }
            // A lexical fallback is never presented as an exact mapping.
            if bp.mapping_provenance == BreakpointMappingProvenance::LexicalFallback
                && bp.mapping() == BreakpointMappingState::Exact
            {
                return fail(format!(
                    "breakpoint {} presents a lexical fallback as an exact mapping",
                    bp.breakpoint_id
                ));
            }
            // Notebook and replay scopes carry their stable anchors.
            if bp.scope.requires_notebook_anchor() && bp.notebook_anchor.is_none() {
                return fail(format!(
                    "notebook-scoped breakpoint {} has no notebook cell anchor",
                    bp.breakpoint_id
                ));
            }
            if bp.scope.requires_replay_anchor() && bp.replay_anchor.is_none() {
                return fail(format!(
                    "replay-scoped breakpoint {} has no replay frame anchor",
                    bp.breakpoint_id
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

fn bp_flags_consistent(bp: &BreakpointSpec) -> bool {
    bp.kind_token == bp.kind.as_str()
        && bp.kind_targets_source_location == bp.kind.targets_source_location()
        && bp.enablement_token == bp.enablement.as_str()
        && bp.enablement_is_active == bp.enablement.is_active()
        && bp.scope_token == bp.scope.as_str()
        && bp.mapping_provenance_token == bp.mapping_provenance.as_str()
        && bp.mapping_provenance_is_semantic == bp.mapping_provenance.is_semantic()
        && bp.mapping_provenance_requires_disclosure == bp.mapping_provenance.requires_disclosure()
        && bp
            .pill
            .matches_derivation(bp.pill.verification, bp.pill.mapping, bp.scope)
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
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

/// Builds the canonical M5 breakpoint-spec set.
///
/// Deterministic: the same bytes every call. Each invariant's `holds` flag is computed
/// from the built specs, so an inconsistent edit flips an invariant rather than
/// silently passing.
pub fn m5_breakpoint_spec_set() -> BreakpointSpecSet {
    let breakpoints = build_breakpoints();
    let invariants = compute_invariants(&breakpoints);

    BreakpointSpecSet {
        record_kind: M5_BREAKPOINT_SPECS_RECORD_KIND.to_owned(),
        m5_breakpoint_specs_schema_version: M5_BREAKPOINT_SPECS_SCHEMA_VERSION,
        schema_ref: M5_BREAKPOINT_SPECS_SCHEMA_REF.to_owned(),
        set_id: M5_BREAKPOINT_SPECS_SET_ID.to_owned(),
        as_of: M5_BREAKPOINT_SPECS_AS_OF.to_owned(),
        freeze_gate_ref: M5_BREAKPOINT_SPECS_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set of M5 breakpoint specs and mapping-state pills. Every \
                  breakpoint carries one canonical pill that pins one verification state (pending, \
                  verified, unbound, unsupported, policy-blocked) and one mapping state (exact, \
                  misaligned, needs-remap, unmapped), so a breakpoint shown in a gutter, session \
                  header, list, notebook cell, replay timeline, or export packet traces back to one \
                  spec and one state vocabulary. A green confirmed-stop icon renders only when a \
                  breakpoint is verified, exact, and not replay-only; an unbound, misaligned, \
                  replay-only, or policy-blocked breakpoint always discloses. Breakpoint identity \
                  survives rename, reformat, and import where a stable logical source identity \
                  exists, and degrades to an explicit needs-remap rather than vanishing when it \
                  does not; a lexical fallback is never presented as an exact semantic mapping; and \
                  notebook and replay views keep stable cell and frame identity."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json",
            "schemas/debug/m5_debug_contracts.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_breakpoint_specs/mod.rs",
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
        breakpoints,
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

fn anchor(
    logical: &str,
    path_hint: Option<&str>,
    line: u32,
    end_line: Option<u32>,
) -> BreakpointSourceAnchor {
    BreakpointSourceAnchor {
        logical_source_ref: logical.to_owned(),
        physical_path_hint: path_hint.map(str::to_owned),
        line,
        end_line,
        column: None,
    }
}

fn build_breakpoints() -> Vec<BreakpointSpec> {
    use BreakpointEnablement::*;
    use BreakpointKindClass::*;
    use BreakpointMappingProvenance::*;
    use BreakpointMappingState::*;
    use BreakpointScopeClass::*;
    use BreakpointVerificationState::*;

    vec![
        // 1. Clean, verified, exact line breakpoint with a stable source identity: the
        //    only state allowed to render the unqualified green confirmed-stop icon.
        BreakpointSpec::build(
            "debug.breakpoint:line_verified_exact:0001",
            Line,
            Enabled,
            anchor(
                "source:logical:service-api/handler.rs#handle_request",
                Some("path:digest:1a2b3c"),
                142,
                None,
            ),
            None,
            None,
            BreakpointPayload::none(),
            WorkspaceSource,
            StableSourceId,
            Verified,
            Exact,
            LIVE_PROOF_REF,
            "Verified line breakpoint bound exactly at its requested location with a stable source \
             identity — the only state that renders the unqualified confirmed-stop icon.",
        ),
        // 2. Conditional breakpoint that bound at an adjusted line after a reformat:
        //    verified but disclosed as relocated.
        BreakpointSpec::build(
            "debug.breakpoint:conditional_verified_misaligned:0002",
            Conditional,
            Enabled,
            anchor(
                "source:logical:service-api/handler.rs#validate",
                Some("path:digest:4d5e6f"),
                209,
                None,
            ),
            None,
            None,
            BreakpointPayload {
                has_condition: true,
                condition_digest: Some("cond:digest:7a8b9c".to_owned()),
                has_log_message: false,
                log_message_digest: None,
                hit_condition: Some(">= 3".to_owned()),
            },
            SessionLocal,
            ReResolvedAfterReformat,
            Verified,
            Misaligned,
            MAPPING_STALE_PROOF_REF,
            "Conditional breakpoint that bound at an adjusted line after a reformat, re-resolved \
             over the same source id and disclosed as relocated rather than drawn as exact.",
        ),
        // 3. Pending line breakpoint: accepted but not yet confirmed bound.
        BreakpointSpec::build(
            "debug.breakpoint:line_pending:0003",
            Line,
            Enabled,
            anchor(
                "source:logical:worker/queue.rs#drain",
                Some("path:digest:0f1e2d"),
                64,
                None,
            ),
            None,
            None,
            BreakpointPayload::none(),
            WorkspaceSource,
            StableSourceId,
            Pending,
            Exact,
            LIVE_PROOF_REF,
            "Pending line breakpoint accepted by Aureline but not yet confirmed bound by the \
             target, disclosed as pending rather than as a confirmed stop.",
        ),
        // 4. Logpoint whose source file was edited so its stable identity was lost:
        //    unbound and degraded to an explicit needs-remap rather than vanishing.
        BreakpointSpec::build(
            "debug.breakpoint:logpoint_unbound_needs_remap:0004",
            Logpoint,
            Enabled,
            anchor(
                "source:logical:worker/legacy.rs#orphaned",
                Some("path:digest:33aa44"),
                88,
                Some(90),
            ),
            None,
            None,
            BreakpointPayload {
                has_condition: false,
                condition_digest: None,
                has_log_message: true,
                log_message_digest: Some("log:digest:bb55cc".to_owned()),
                hit_condition: None,
            },
            WorkspaceSource,
            SourceIdentityLost,
            Unbound,
            NeedsRemap,
            MAPPING_UNKNOWN_PROOF_REF,
            "Logpoint whose source identity was lost after an edit: it stays visible, unbound, and \
             flagged needs-remap so it asks for an explicit remap instead of silently disappearing.",
        ),
        // 5. Function breakpoint against a replay capture: unsupported by the replay
        //    backend, replay-only, with a stable frame anchor.
        BreakpointSpec::build(
            "debug.breakpoint:function_unsupported_replay:0005",
            Function,
            Enabled,
            anchor(
                "source:logical:task/run.rs#execute",
                None,
                0,
                None,
            ),
            None,
            Some(ReplayFrameAnchor {
                capture_ref: "replay:capture:task-run-42".to_owned(),
                timeline_ref: "replay:timeline:0x0a3f".to_owned(),
                frame_ref: Some("replay:frame:execute".to_owned()),
            }),
            BreakpointPayload::none(),
            ReplayTimeline,
            ImportedSourceMap,
            Unsupported,
            Unmapped,
            REPLAY_PROOF_REF,
            "Function breakpoint against a recorded replay capture: unsupported by the replay \
             backend, replay-only, and never drawn as a live confirmed stop, with stable frame \
             identity preserved.",
        ),
        // 6. Data watchpoint on a write-protected target: blocked by policy.
        BreakpointSpec::build(
            "debug.breakpoint:data_policy_blocked:0006",
            Data,
            Enabled,
            anchor(
                "source:logical:service-api/state.rs#counter",
                Some("path:digest:77ee88"),
                31,
                None,
            ),
            None,
            None,
            BreakpointPayload::none(),
            SessionLocal,
            StableSourceId,
            PolicyBlocked,
            Exact,
            LIVE_PROOF_REF,
            "Data watchpoint refused by a write-protect policy on the target: policy-blocked and \
             disclosed rather than shown as an active stop.",
        ),
        // 7. Exception breakpoint: verified against an exception category, with no
        //    source mapping by nature.
        BreakpointSpec::build(
            "debug.breakpoint:exception_verified:0007",
            Exception,
            Enabled,
            anchor("source:logical:exception-category/uncaught", None, 0, None),
            None,
            None,
            BreakpointPayload::none(),
            ExceptionCategory,
            StableSourceId,
            Verified,
            Unmapped,
            LIVE_PROOF_REF,
            "Exception breakpoint verified against the uncaught-exception category: it binds but \
             has no source line, so it discloses an unmapped location rather than implying one.",
        ),
        // 8. Notebook-cell line breakpoint: verified and exact within a cell, with the
        //    stable cell identity preserved.
        BreakpointSpec::build(
            "debug.breakpoint:notebook_verified_exact:0008",
            Line,
            Enabled,
            anchor("source:logical:notebook/analysis#cell-7", None, 4, None),
            Some(NotebookCellAnchor {
                notebook_ref: "notebook:doc:analysis".to_owned(),
                cell_id: "cell:stable:7".to_owned(),
                cell_execution_ref: Some("cell:exec:7:0003".to_owned()),
            }),
            None,
            BreakpointPayload::none(),
            NotebookCell,
            StableSourceId,
            Verified,
            Exact,
            NOTEBOOK_EXACT_PROOF_REF,
            "Notebook line breakpoint verified and exact inside a stable cell, with the cell \
             identity preserved through re-execution.",
        ),
        // 9. Notebook-cell breakpoint whose line shifted on re-execution: stable cell
        //    identity is kept but the line needs remap, so it is not shown as exact.
        BreakpointSpec::build(
            "debug.breakpoint:notebook_needs_remap:0009",
            Line,
            Enabled,
            anchor("source:logical:notebook/analysis#cell-12", None, 9, None),
            Some(NotebookCellAnchor {
                notebook_ref: "notebook:doc:analysis".to_owned(),
                cell_id: "cell:stable:12".to_owned(),
                cell_execution_ref: None,
            }),
            None,
            BreakpointPayload::none(),
            NotebookCell,
            SourceIdentityLost,
            Unbound,
            NeedsRemap,
            NOTEBOOK_STALE_PROOF_REF,
            "Notebook breakpoint whose in-cell line could not be recovered after re-execution: \
             the stable cell identity is kept while the line is flagged needs-remap rather than \
             pretended exact.",
        ),
        // 10. Imported breakpoint backed only by a lexical match: bound but disclosed as
        //     a textual fallback, never replayed as a semantic mapping.
        BreakpointSpec::build(
            "debug.breakpoint:imported_lexical_fallback:0010",
            Line,
            Disabled,
            anchor(
                "source:logical:imported/vendor.rs#entry",
                Some("path:digest:99cc00"),
                512,
                None,
            ),
            None,
            None,
            BreakpointPayload::none(),
            WorkspaceSource,
            LexicalFallback,
            Verified,
            Misaligned,
            MAPPING_STALE_PROOF_REF,
            "Imported breakpoint matched only by a lexical fallback: it bound but is disclosed as \
             a textual match relocated to the nearest line, never replayed as a semantic mapping.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> BreakpointInvariant {
    BreakpointInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(breakpoints: &[BreakpointSpec]) -> Vec<BreakpointInvariant> {
    // Every breakpoint carries one canonical pill whose flags equal the derivation
    // from its verification state, mapping state, and scope.
    let one_canonical_pill = breakpoints.iter().all(|b| {
        b.pill
            .matches_derivation(b.pill.verification, b.pill.mapping, b.scope)
            && b.pill.verification_token == b.pill.verification.as_str()
            && b.pill.mapping_token == b.pill.mapping.as_str()
    });

    // The full verification vocabulary is materialized.
    let verification_complete = BreakpointVerificationState::ALL
        .iter()
        .all(|state| breakpoints.iter().any(|b| b.verification() == *state));

    // The full mapping vocabulary is materialized.
    let mapping_complete = BreakpointMappingState::ALL
        .iter()
        .all(|state| breakpoints.iter().any(|b| b.mapping() == *state));

    // A green confirmed-stop icon never hides an unbound, misaligned, replay-only, or
    // policy-blocked reality: the clean-confirmed flag is true only for a verified,
    // exact, non-replay breakpoint, and at least one disclosed case exists.
    let green_never_hides = breakpoints.iter().all(|b| {
        b.pill.shows_clean_confirmed
            == (b.verification().is_bound()
                && b.mapping().preserves_exact_location()
                && !b.scope.is_replay_only())
            && b.pill.requires_disclosure != b.pill.shows_clean_confirmed
    }) && breakpoints.iter().any(|b| {
        !b.pill.shows_clean_confirmed
            && (b.verification() == BreakpointVerificationState::Unbound
                || b.verification() == BreakpointVerificationState::PolicyBlocked
                || b.mapping() == BreakpointMappingState::Misaligned
                || b.scope.is_replay_only())
    });

    // A lost source identity degrades to needs-remap (and stays visible), and a
    // needs-remap state only ever comes from a lost source identity.
    let lost_identity_needs_remap = breakpoints.iter().all(|b| {
        b.mapping_provenance.forces_needs_remap()
            == (b.mapping() == BreakpointMappingState::NeedsRemap)
    }) && breakpoints
        .iter()
        .any(|b| b.mapping() == BreakpointMappingState::NeedsRemap);

    // A lexical-fallback provenance is never presented as an exact mapping and always
    // discloses it is not a semantic match.
    let lexical_never_exact = breakpoints.iter().all(|b| {
        if b.mapping_provenance == BreakpointMappingProvenance::LexicalFallback {
            b.mapping() != BreakpointMappingState::Exact && !b.mapping_provenance_is_semantic
        } else {
            true
        }
    }) && breakpoints
        .iter()
        .any(|b| b.mapping_provenance == BreakpointMappingProvenance::LexicalFallback);

    // Every notebook-scoped breakpoint keeps a stable cell anchor and is never shown as
    // a clean confirmed stop while its mapping is not exact.
    let notebook_preserves_identity = breakpoints
        .iter()
        .filter(|b| b.scope == BreakpointScopeClass::NotebookCell)
        .all(|b| {
            b.notebook_anchor
                .as_ref()
                .is_some_and(|a| !a.cell_id.is_empty())
                && (b.mapping().preserves_exact_location() || !b.pill.shows_clean_confirmed)
        })
        && breakpoints
            .iter()
            .any(|b| b.scope == BreakpointScopeClass::NotebookCell);

    // Every replay-scoped breakpoint keeps a stable frame anchor, stays replay-only,
    // and never renders a clean confirmed stop.
    let replay_preserves_identity = breakpoints
        .iter()
        .filter(|b| b.scope == BreakpointScopeClass::ReplayTimeline)
        .all(|b| {
            b.replay_anchor
                .as_ref()
                .is_some_and(|a| !a.timeline_ref.is_empty())
                && b.pill.is_replay_only
                && !b.pill.shows_clean_confirmed
        })
        && breakpoints
            .iter()
            .any(|b| b.scope == BreakpointScopeClass::ReplayTimeline);

    // Every breakpoint retains its verification and mapping state as typed pill fields
    // and cites an export-safe proof packet, so export never flattens it into chrome.
    let export_retains_state = breakpoints.iter().all(|b| {
        !b.pill.verification_token.is_empty()
            && !b.pill.mapping_token.is_empty()
            && !b.proof_packet_ref.is_empty()
            && is_export_safe_ref(&b.proof_packet_ref)
    });

    vec![
        invariant(
            "breakpoints.one_canonical_pill_vocabulary",
            "Every breakpoint carries exactly one pill whose verification and mapping tokens come \
             from the frozen vocabulary and whose flags equal their derivation, so a breakpoint \
             shown anywhere traces back to one spec and one state vocabulary.",
            one_canonical_pill,
        ),
        invariant(
            "breakpoints.verification_vocabulary_complete",
            "Pending, verified, unbound, unsupported, and policy-blocked are all materialized.",
            verification_complete,
        ),
        invariant(
            "breakpoints.mapping_vocabulary_complete",
            "Exact, misaligned, needs-remap, and unmapped are all materialized.",
            mapping_complete,
        ),
        invariant(
            "breakpoints.green_never_hides_unverified_misaligned_replay_or_blocked",
            "The unqualified confirmed-stop icon renders only for a verified, exact, non-replay \
             breakpoint; an unbound, misaligned, replay-only, or policy-blocked breakpoint always \
             discloses.",
            green_never_hides,
        ),
        invariant(
            "breakpoints.lost_identity_degrades_to_needs_remap",
            "A breakpoint whose stable source identity was lost stays visible and is flagged \
             needs-remap rather than vanishing, and a needs-remap state only ever comes from a \
             lost source identity.",
            lost_identity_needs_remap,
        ),
        invariant(
            "breakpoints.lexical_fallback_never_presented_as_exact",
            "A lexical-fallback mapping is never presented as an exact semantic mapping; it always \
             discloses it is a textual match.",
            lexical_never_exact,
        ),
        invariant(
            "breakpoints.notebook_preserves_stable_cell_identity",
            "Every notebook-scoped breakpoint keeps a stable cell anchor and is never shown as a \
             clean confirmed stop while its in-cell mapping is not exact.",
            notebook_preserves_identity,
        ),
        invariant(
            "breakpoints.replay_preserves_stable_frame_identity_and_stays_replay_only",
            "Every replay-scoped breakpoint keeps a stable frame anchor, stays replay-only, and \
             never renders a live confirmed stop.",
            replay_preserves_identity,
        ),
        invariant(
            "breakpoints.export_retains_verification_and_mapping_state",
            "Every breakpoint retains its verification and mapping state as typed pill fields and \
             cites an export-safe proof packet, so support export never flattens it into rendered \
             chrome.",
            export_retains_state,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the breakpoint-spec set as human-readable lines for CLI/headless and
/// support.
pub fn m5_breakpoint_spec_lines(set: &BreakpointSpecSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 breakpoint specs — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Breakpoints: {}  Invariants: {}",
        set.breakpoints.len(),
        set.invariants.len(),
    ));

    lines.push("Breakpoints:".to_owned());
    for bp in &set.breakpoints {
        lines.push(format!(
            "  - {} [{}] scope={} pill={} clean_confirmed={} needs_remap={} replay_only={}",
            bp.breakpoint_id,
            bp.kind_token,
            bp.scope_token,
            bp.pill.label,
            bp.pill.shows_clean_confirmed,
            bp.pill.needs_explicit_remap,
            bp.pill.is_replay_only,
        ));
        lines.push(format!(
            "      verification={} mapping={} provenance={} ({})",
            bp.pill.verification_token,
            bp.pill.mapping_token,
            bp.mapping_provenance_token,
            if bp.mapping_provenance_is_semantic {
                "semantic"
            } else {
                "lexical fallback"
            },
        ));
        lines.push(format!(
            "      source: {}",
            bp.source_anchor.logical_source_ref
        ));
        lines.push(format!("      {}", bp.summary));
        lines.push(format!("      proof: {}", bp.proof_packet_ref));
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
