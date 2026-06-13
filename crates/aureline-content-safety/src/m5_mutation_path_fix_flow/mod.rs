//! Auditable suppressions, preview-first bidi/invisible/confusable fix flows, and
//! no-silent-byte-rewrite guards across the new M5 mutation paths.
//!
//! Save, format, organize-imports, and AI-apply are *mutation paths*: they write
//! bytes back to the new M5 mutation-bearing artifact families (notebooks, docs
//! pages, AI-evidence artifacts, pipeline/structured artifacts, and generated
//! artifacts). When the content they touch carries bidi-control, invisible, or
//! mixed-script confusable bytes, two things must hold:
//!
//! 1. Any fix that rewrites those suspicious bytes routes through a *previewable
//!    diff* or a *review sheet* before bytes change — never a silent rewrite.
//! 2. If the user suppresses the warning instead of fixing, the suppression is a
//!    scope-aware, auditable object (actor, reason, timestamp, scope, optional
//!    expiry, reachable audit log) — not hidden per-pane state that disappears.
//!
//! This lane sits on the same shared content-integrity policy library as its
//! siblings: it runs the shared suspicious-content detector
//! ([`crate::detect_suspicious_content`]) over the content each mutation path
//! touches and derives the shared safe-inspection escape
//! ([`crate::escape_for_safe_inspection`]) rather than inventing a parallel
//! detector. The frozen content-integrity matrix in
//! [`crate::freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`]
//! locks the static qualification each surface may claim, and the
//! suspicious-text detector parity lane in
//! [`crate::m5_suspicious_text_detector_parity`] keeps the warning vocabulary
//! shared. This lane covers the orthogonal *mutation-path* gap they leave open:
//! a save/format/organize-imports/AI-apply must never quietly normalize
//! suspicious bytes, and the choice to suppress a warning must be auditable.
//!
//! Every mutation path resolves to a fix-flow mode that requires a preview
//! before bytes change ([`M5FixFlowMode::PreviewableDiff`] for the local-edit
//! paths, [`M5FixFlowMode::ReviewSheet`] for AI-apply), and every path blocks
//! silent byte rewrites. Suspicious bytes are never normalized away, and the
//! escaped inspection excerpt never masquerades as the raw bytes.
//!
//! The packet is metadata only: it carries opaque refs to the artifact and its
//! raw bytes plus the shared detector's escaped excerpt, never the raw artifact
//! bodies, credentials, or provider payloads, so nothing unsafe crosses the
//! export boundary.
//!
//! The boundary schema is
//! [`schemas/security/m5-mutation-path-fix-flow.schema.json`](../../../../schemas/security/m5-mutation-path-fix-flow.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_mutation_path_fix_flow.md`](../../../../docs/security/m5/m5_mutation_path_fix_flow.md).
//! The protected fixture directory is
//! [`fixtures/security/m5/m5_mutation_path_fix_flow/`](../../../../fixtures/security/m5/m5_mutation_path_fix_flow/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::detector::{detect_suspicious_content, escape_for_safe_inspection};

/// Stable record-kind tag carried by [`M5MutationPathFixFlowPacket`].
pub const M5_MUTATION_PATH_FIX_FLOW_RECORD_KIND: &str = "m5_mutation_path_fix_flow_packet";

/// Integer schema version for the M5 mutation-path fix-flow packet.
pub const M5_MUTATION_PATH_FIX_FLOW_SCHEMA_VERSION: u32 = 1;

/// Stable packet id minted by [`frozen_m5_mutation_path_fix_flow_packet`].
pub const M5_MUTATION_PATH_FIX_FLOW_PACKET_ID: &str = "m5-mutation-path-fix-flow:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_MUTATION_PATH_FIX_FLOW_SCHEMA_REF: &str =
    "schemas/security/m5-mutation-path-fix-flow.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MUTATION_PATH_FIX_FLOW_DOC_REF: &str = "docs/security/m5/m5_mutation_path_fix_flow.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MUTATION_PATH_FIX_FLOW_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5_mutation_path_fix_flow";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MUTATION_PATH_FIX_FLOW_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_mutation_path_fix_flow/support_export.json";

/// Repo-relative path of the sibling suspicious-text detector-parity contract.
pub const M5_MUTATION_PATH_FIX_FLOW_SUSPICIOUS_TEXT_CONTRACT_REF: &str =
    "schemas/security/m5-suspicious-text-detector-parity.schema.json";

/// Repo-relative path of the frozen content-integrity matrix contract.
pub const M5_MUTATION_PATH_FIX_FLOW_CONTENT_INTEGRITY_MATRIX_CONTRACT_REF: &str =
    "schemas/security/freeze-the-m5-suspicious-content-safe-preview-and-representation-copy-export-matrix.schema.json";

/// A mutation path that can write bytes back to a new M5 artifact family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MutationPath {
    /// Persisting an edited artifact to disk.
    Save,
    /// Reformatting an artifact in place.
    Format,
    /// Reordering/removing imports in a code-bearing artifact.
    OrganizeImports,
    /// Applying an AI-proposed change set.
    AiApply,
}

impl M5MutationPath {
    /// Every mutation path, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Save,
        Self::Format,
        Self::OrganizeImports,
        Self::AiApply,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Save => "save",
            Self::Format => "format",
            Self::OrganizeImports => "organize_imports",
            Self::AiApply => "ai_apply",
        }
    }

    /// Human-readable name of this mutation path.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Save => "Save",
            Self::Format => "Format",
            Self::OrganizeImports => "Organize Imports",
            Self::AiApply => "AI Apply",
        }
    }

    /// The fix-flow route a fix on this path must travel before bytes change.
    ///
    /// AI-apply proposes a change set reviewed in a review sheet; the local-edit
    /// paths show a previewable diff of raw vs proposed bytes.
    pub const fn preview_route(self) -> M5FixFlowMode {
        match self {
            Self::Save | Self::Format | Self::OrganizeImports => M5FixFlowMode::PreviewableDiff,
            Self::AiApply => M5FixFlowMode::ReviewSheet,
        }
    }
}

/// The previewable route a suspicious-byte fix travels before bytes change.
///
/// There is deliberately no `SilentRewrite` variant: a mutation path can never
/// rewrite suspicious bytes without a preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FixFlowMode {
    /// A previewable diff of raw vs proposed bytes shown before the write.
    PreviewableDiff,
    /// A review sheet listing the proposed edits shown before the write.
    ReviewSheet,
}

impl M5FixFlowMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewableDiff => "previewable_diff",
            Self::ReviewSheet => "review_sheet",
        }
    }

    /// Human-readable label for the fix-flow affordance.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PreviewableDiff => "Preview fix diff",
            Self::ReviewSheet => "Review proposed change",
        }
    }
}

/// A class of suspicious-byte fix a preview-first fix flow can offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FixKind {
    /// Fix a bidi-control codepoint.
    Bidi,
    /// Fix an invisible/zero-width formatting codepoint.
    Invisible,
    /// Fix a mixed-script confusable identifier.
    Confusable,
}

impl M5FixKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bidi => "bidi",
            Self::Invisible => "invisible",
            Self::Confusable => "confusable",
        }
    }

    /// Maps a shared-detector threat-class token to a fix kind, if any.
    pub fn from_detector_class(class: &str) -> Option<Self> {
        match class {
            "bidi_control" => Some(Self::Bidi),
            "invisible_formatting" => Some(Self::Invisible),
            "mixed_script_confusable" | "whole_script_confusable" => Some(Self::Confusable),
            _ => None,
        }
    }
}

/// Scope at which a suspicious-content suppression applies.
///
/// A suppression is always recorded at one of these scopes, never as hidden
/// per-pane state, so it is auditable and can be narrowed or revoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuppressionScope {
    /// This single occurrence only.
    Occurrence,
    /// Every occurrence in this file/artifact.
    File,
    /// Every occurrence in the workspace.
    Workspace,
    /// Every occurrence governed by an admin policy.
    AdminPolicy,
}

impl M5SuppressionScope {
    /// Every suppression scope, narrowest first.
    pub const ALL: [Self; 4] = [
        Self::Occurrence,
        Self::File,
        Self::Workspace,
        Self::AdminPolicy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Occurrence => "occurrence",
            Self::File => "file",
            Self::Workspace => "workspace",
            Self::AdminPolicy => "admin_policy",
        }
    }
}

/// A scope-aware, auditable suppression of a suspicious-content warning.
///
/// Every field that makes a suppression auditable is recorded: who suppressed it
/// ([`Self::actor_ref`]), why ([`Self::reason`]), when ([`Self::recorded_at`]),
/// at what scope ([`Self::scope`]), which fix kinds it covers
/// ([`Self::suppressed_fix_kinds`]), an optional expiry that narrows it
/// ([`Self::expires_at`]), and a reachable audit-log ref ([`Self::audit_log_ref`]).
/// [`Self::hidden_per_pane_state`] is always `false`: a suppression is a recorded
/// object, never transient per-pane state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuppressionRecord {
    /// Stable suppression id.
    pub suppression_id: String,
    /// Scope at which the suppression applies.
    pub scope: M5SuppressionScope,
    /// Scope token, redundant with [`Self::scope`] for export readers.
    pub scope_token: String,
    /// Fix kinds this suppression covers, sorted.
    pub suppressed_fix_kinds: Vec<String>,
    /// Opaque ref of the actor who recorded the suppression.
    pub actor_ref: String,
    /// Human-readable reason recorded with the suppression.
    pub reason: String,
    /// Timestamp the suppression was recorded (RFC 3339).
    pub recorded_at: String,
    /// Optional expiry that automatically narrows the suppression (RFC 3339).
    pub expires_at: Option<String>,
    /// Opaque ref to the reachable audit-log entry for this suppression.
    pub audit_log_ref: String,
    /// Opaque ref to the artifact the suppressed finding lives in.
    pub finding_artifact_ref: String,
    /// Whether the suppression is hidden per-pane state (always false).
    pub hidden_per_pane_state: bool,
}

impl M5SuppressionRecord {
    /// Whether this suppression records every field that makes it auditable.
    pub fn is_auditable(&self) -> bool {
        !self.actor_ref.trim().is_empty()
            && !self.reason.trim().is_empty()
            && !self.recorded_at.trim().is_empty()
            && !self.audit_log_ref.trim().is_empty()
            && !self.finding_artifact_ref.trim().is_empty()
    }
}

/// The capabilities of the suppression-audit contract on a mutation path.
///
/// Distinct from a concrete [`M5SuppressionRecord`]: this block states *that* a
/// suppression on this path is always scope-aware and auditable, regardless of
/// whether one was recorded in this case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuppressionAudit {
    /// Suppressions on this path are scope-aware.
    pub scope_aware: bool,
    /// Scopes a suppression may be recorded at, narrowest first.
    pub allowed_scopes: Vec<String>,
    /// The suppression records the actor.
    pub records_actor: bool,
    /// The suppression records a reason.
    pub records_reason: bool,
    /// The suppression records a timestamp.
    pub records_timestamp: bool,
    /// The suppression records its scope.
    pub records_scope: bool,
    /// The suppression supports an optional expiry that narrows it.
    pub supports_expiry: bool,
    /// The suppression's audit-log entry is reachable.
    pub audit_log_reachable: bool,
    /// Suppressions are never hidden per-pane state (always false).
    pub hidden_per_pane_state: bool,
}

impl M5SuppressionAudit {
    /// The frozen, all-capabilities-present audit block.
    pub fn frozen() -> Self {
        Self {
            scope_aware: true,
            allowed_scopes: M5SuppressionScope::ALL
                .iter()
                .map(|s| s.as_str().to_owned())
                .collect(),
            records_actor: true,
            records_reason: true,
            records_timestamp: true,
            records_scope: true,
            supports_expiry: true,
            audit_log_reachable: true,
            hidden_per_pane_state: false,
        }
    }

    fn is_sound(&self) -> bool {
        self.scope_aware
            && self.allowed_scopes.len() == M5SuppressionScope::ALL.len()
            && self.records_actor
            && self.records_reason
            && self.records_timestamp
            && self.records_scope
            && self.supports_expiry
            && self.audit_log_reachable
            && !self.hidden_per_pane_state
    }
}

/// The preview-first fix flow resolved for a mutation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FixFlow {
    /// Preview route a fix travels before bytes change.
    pub mode: M5FixFlowMode,
    /// Mode token, redundant with [`Self::mode`] for export readers.
    pub mode_token: String,
    /// Human-readable label for the fix-flow affordance.
    pub label: String,
    /// Whether a preview is required before any byte changes (always true).
    pub preview_required: bool,
    /// Whether the preview shows raw bytes alongside the proposed fix.
    pub shows_raw_and_proposed: bool,
    /// Fix kinds offered, derived from detected suspicious classes, sorted.
    pub fix_kinds_offered: Vec<String>,
}

/// A typed suppression seed used to record a concrete suppression in a case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5SuppressionSeed<'a> {
    /// Stable suppression id.
    pub suppression_id: &'a str,
    /// Scope at which the suppression applies.
    pub scope: M5SuppressionScope,
    /// Opaque ref of the actor recording the suppression.
    pub actor_ref: &'a str,
    /// Human-readable reason.
    pub reason: &'a str,
    /// Timestamp the suppression was recorded (RFC 3339).
    pub recorded_at: &'a str,
    /// Optional expiry that narrows the suppression (RFC 3339).
    pub expires_at: Option<&'a str>,
    /// Opaque ref to the reachable audit-log entry.
    pub audit_log_ref: &'a str,
}

/// Inputs describing one mutation path's content and optional suppression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5MutationPathInput<'a> {
    /// Mutation path this input describes.
    pub path: M5MutationPath,
    /// Opaque ref of the artifact being mutated.
    pub artifact_ref: &'a str,
    /// Opaque ref to the raw bytes the open-raw affordance targets.
    pub raw_content_ref: &'a str,
    /// Representative excerpt fed to the shared detector. Only its escaped form
    /// and finding summary are exported.
    pub content_excerpt: &'a str,
    /// Optional auditable suppression recorded for this path's findings.
    pub suppression: Option<M5SuppressionSeed<'a>>,
}

/// Inputs needed to project the M5 mutation-path fix-flow packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5MutationPathFixFlowSeed<'a> {
    /// Stable case id shared by all path projections.
    pub case_id: &'a str,
    /// Per-path inputs, one per [`M5MutationPath::ALL`].
    pub path_inputs: [M5MutationPathInput<'a>; 4],
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: &'a str,
}

/// The resolved preview-first fix flow and suppression audit for one path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationPathProjection {
    /// Mutation path this projection describes.
    pub path: M5MutationPath,
    /// Path token, redundant with [`Self::path`] for export readers.
    pub path_token: String,
    /// Human-readable mutation path name.
    pub path_display_name: String,
    /// Opaque artifact ref being mutated.
    pub artifact_ref: String,
    /// Opaque ref the open-raw affordance targets for the raw bytes.
    pub raw_content_ref: String,
    /// The shared detector's escaped, safe-inspection excerpt of the content.
    pub suspicious_excerpt_escaped: String,
    /// Whether the shared detector flagged suspicious bytes in the content.
    pub findings_present: bool,
    /// Number of suspicious findings the shared detector reported.
    pub finding_count: usize,
    /// Distinct detector threat-class tokens present, sorted.
    pub threat_classes: Vec<String>,
    /// Shared-detector outcome token for this content.
    pub detector_outcome_token: String,
    /// The preview-first fix flow resolved for this path.
    pub fix_flow: M5FixFlow,
    /// Whether silent suspicious-byte rewrites are blocked (always true).
    pub silent_byte_rewrite_blocked: bool,
    /// Whether bytes change only after a preview (always true).
    pub bytes_change_only_after_preview: bool,
    /// The suppression-audit contract on this path.
    pub suppression_audit: M5SuppressionAudit,
    /// A concrete auditable suppression, when one was recorded for this case.
    pub recorded_suppression: Option<M5SuppressionRecord>,
    /// Whether the guard is preserved in product.
    pub preserved_in_product: bool,
    /// Whether the guard is preserved in exported review packets.
    pub preserved_in_exported_review_packet: bool,
    /// Whether the guard is preserved in support handoff artifacts.
    pub preserved_in_support_handoff: bool,
    /// Human-readable rationale for the resolved fix flow and guard.
    pub rationale: String,
}

impl M5MutationPathProjection {
    /// Whether the fix flow requires a preview before bytes change.
    pub fn preview_required(&self) -> bool {
        self.fix_flow.preview_required && self.fix_flow.shows_raw_and_proposed
    }

    /// Whether the offered fix kinds cover every detected suspicious class.
    pub fn fix_kinds_cover_findings(&self) -> bool {
        let detected: BTreeSet<&str> = self
            .threat_classes
            .iter()
            .filter_map(|c| M5FixKind::from_detector_class(c).map(M5FixKind::as_str))
            .collect();
        detected
            .iter()
            .all(|kind| self.fix_flow.fix_kinds_offered.iter().any(|o| o == kind))
    }

    /// Whether the guard is preserved across all carriers.
    pub fn preserved_everywhere(&self) -> bool {
        self.preserved_in_product
            && self.preserved_in_exported_review_packet
            && self.preserved_in_support_handoff
    }
}

/// Mutation-path fix-flow review block; every field encodes a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationPathFixFlowReview {
    /// One shared content-integrity policy library governs every path.
    pub one_shared_policy_library_governs_all_paths: bool,
    /// Save, format, organize-imports, and AI-apply are all covered.
    pub save_format_organize_imports_and_ai_apply_covered: bool,
    /// Every mutation path blocks silent suspicious-byte rewrites.
    pub all_mutation_paths_block_silent_byte_rewrite: bool,
    /// Every fix routes through a previewable diff or review sheet first.
    pub fixes_route_through_preview_before_bytes_change: bool,
    /// Fix flows offer bidi, invisible, and confusable fixes as detected.
    pub fix_flows_offer_bidi_invisible_confusable: bool,
    /// Suspicious bytes are surfaced, never normalized away.
    pub suspicious_bytes_never_normalized_away: bool,
    /// The escaped excerpt never masquerades as the raw bytes.
    pub escaped_excerpt_never_masquerades_as_raw: bool,
    /// Suppressions are scope-aware and auditable.
    pub suppressions_are_scope_aware_and_auditable: bool,
    /// Suppressions are never hidden per-pane state.
    pub suppressions_never_hidden_per_pane_state: bool,
    /// The guard is preserved in product, export, and handoff.
    pub guard_preserved_in_product_export_and_handoff: bool,
}

impl M5MutationPathFixFlowReview {
    /// The frozen, all-invariants-hold review block.
    pub const fn frozen() -> Self {
        Self {
            one_shared_policy_library_governs_all_paths: true,
            save_format_organize_imports_and_ai_apply_covered: true,
            all_mutation_paths_block_silent_byte_rewrite: true,
            fixes_route_through_preview_before_bytes_change: true,
            fix_flows_offer_bidi_invisible_confusable: true,
            suspicious_bytes_never_normalized_away: true,
            escaped_excerpt_never_masquerades_as_raw: true,
            suppressions_are_scope_aware_and_auditable: true,
            suppressions_never_hidden_per_pane_state: true,
            guard_preserved_in_product_export_and_handoff: true,
        }
    }

    fn all_hold(&self) -> bool {
        self.one_shared_policy_library_governs_all_paths
            && self.save_format_organize_imports_and_ai_apply_covered
            && self.all_mutation_paths_block_silent_byte_rewrite
            && self.fixes_route_through_preview_before_bytes_change
            && self.fix_flows_offer_bidi_invisible_confusable
            && self.suspicious_bytes_never_normalized_away
            && self.escaped_excerpt_never_masquerades_as_raw
            && self.suppressions_are_scope_aware_and_auditable
            && self.suppressions_never_hidden_per_pane_state
            && self.guard_preserved_in_product_export_and_handoff
    }
}

/// Cross-path mutation-path fix-flow and suppression-audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationPathFixFlowPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Number of mutation paths projected.
    pub path_count: usize,
    /// Number of paths whose content carried suspicious bytes.
    pub paths_with_findings_count: usize,
    /// Number of paths that recorded an auditable suppression.
    pub suppressed_path_count: usize,
    /// Distinct fix-flow mode tokens across paths, sorted.
    pub fix_flow_modes: Vec<String>,
    /// Distinct suppression scope tokens recorded across paths, sorted.
    pub recorded_suppression_scopes: Vec<String>,
    /// Whether projection normalized or stripped any source (always false).
    pub normalization_applied: bool,
    /// Per-path resolved projections.
    pub paths: Vec<M5MutationPathProjection>,
    /// Mutation-path fix-flow review block.
    pub review: M5MutationPathFixFlowReview,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MutationPathFixFlowPacket {
    /// Returns true when every required mutation path is present exactly once.
    pub fn covers_all_paths(&self) -> bool {
        let present: BTreeSet<_> = self.paths.iter().map(|p| p.path).collect();
        M5MutationPath::ALL.iter().all(|p| present.contains(p))
            && present.len() == M5MutationPath::ALL.len()
    }

    /// Returns true when every path blocks silent byte rewrites.
    pub fn all_block_silent_byte_rewrite(&self) -> bool {
        self.paths
            .iter()
            .all(|p| p.silent_byte_rewrite_blocked && p.bytes_change_only_after_preview)
    }

    /// Returns true when every path requires a preview before bytes change.
    pub fn all_require_preview(&self) -> bool {
        self.paths
            .iter()
            .all(M5MutationPathProjection::preview_required)
    }

    /// Returns true when every path's offered fix kinds cover its findings.
    pub fn fix_kinds_cover_findings_everywhere(&self) -> bool {
        self.paths
            .iter()
            .all(M5MutationPathProjection::fix_kinds_cover_findings)
    }

    /// Returns true when every recorded suppression is scope-aware, auditable,
    /// and not hidden per-pane state.
    pub fn suppressions_auditable_everywhere(&self) -> bool {
        self.paths.iter().all(|p| {
            if !p.suppression_audit.is_sound() {
                return false;
            }
            match &p.recorded_suppression {
                None => true,
                Some(s) => s.is_auditable() && !s.hidden_per_pane_state,
            }
        })
    }

    /// Returns true when the guard is preserved across carriers everywhere.
    pub fn preserved_everywhere(&self) -> bool {
        self.paths
            .iter()
            .all(M5MutationPathProjection::preserved_everywhere)
    }

    /// Validates the mutation-path fix-flow invariants.
    pub fn validate(&self) -> Vec<M5MutationPathFixFlowViolation> {
        use M5MutationPathFixFlowViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_MUTATION_PATH_FIX_FLOW_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_MUTATION_PATH_FIX_FLOW_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.case_id.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(V::MissingSourceContracts);
        }
        if self.normalization_applied {
            violations.push(V::NormalizationApplied);
        }
        if !self.covers_all_paths() {
            violations.push(V::MutationPathMissing);
        }
        if !self.all_block_silent_byte_rewrite() {
            violations.push(V::SilentRewriteNotBlocked);
        }
        if !self.all_require_preview() {
            violations.push(V::PreviewNotRequired);
        }
        if !self.fix_kinds_cover_findings_everywhere() {
            violations.push(V::FixKindsMissing);
        }
        if !self.suppressions_auditable_everywhere() {
            violations.push(V::SuppressionNotAuditable);
        }
        if !self.preserved_everywhere() {
            violations.push(V::GuardNotPreserved);
        }
        if self.path_count != self.paths.len() {
            violations.push(V::PathCountMismatch);
        }
        if self.paths_with_findings_count != self.declared_findings_count() {
            violations.push(V::FindingsCountMismatch);
        }
        if self.suppressed_path_count != self.declared_suppressed_count() {
            violations.push(V::SuppressedCountMismatch);
        }
        for path in &self.paths {
            // The fix flow's preview route must match the path's declared route.
            if path.fix_flow.mode != path.path.preview_route()
                || path.fix_flow.mode_token != path.fix_flow.mode.as_str()
            {
                violations.push(V::FixFlowModeMismatch);
                break;
            }
        }
        for path in &self.paths {
            // A path with findings must offer at least one fix kind; a clean
            // path must offer none, and may not carry a recorded suppression.
            let has_kinds = !path.fix_flow.fix_kinds_offered.is_empty();
            if path.findings_present != has_kinds {
                violations.push(V::FixKindsMissing);
                break;
            }
            if !path.findings_present && path.recorded_suppression.is_some() {
                violations.push(V::SuppressionWithoutFinding);
                break;
            }
        }
        for path in &self.paths {
            if let Some(s) = &path.recorded_suppression {
                let malformed = s.scope_token != s.scope.as_str()
                    || s.suppressed_fix_kinds.is_empty()
                    || s.hidden_per_pane_state
                    || !s
                        .suppressed_fix_kinds
                        .iter()
                        .all(|k| path.fix_flow.fix_kinds_offered.iter().any(|o| o == k));
                if malformed {
                    violations.push(V::SuppressionRecordMalformed);
                    break;
                }
            }
        }
        if !self.review.all_hold() {
            violations.push(V::ReviewIncomplete);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 mutation-path fix-flow packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    fn declared_findings_count(&self) -> usize {
        self.paths.iter().filter(|p| p.findings_present).count()
    }

    fn declared_suppressed_count(&self) -> usize {
        self.paths
            .iter()
            .filter(|p| p.recorded_suppression.is_some())
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 mutation-path fix-flow packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Mutation-Path Fix Flows And Auditable Suppressions\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Case: `{}`\n", self.case_id));
        out.push_str(&format!(
            "- Paths: {} · with findings: {} · suppressed: {}\n",
            self.path_count, self.paths_with_findings_count, self.suppressed_path_count
        ));
        out.push_str(&format!(
            "- Fix-flow modes: {}\n",
            self.fix_flow_modes.join(", ")
        ));
        out.push_str("\n## Mutation paths\n\n");
        for path in &self.paths {
            out.push_str(&format!(
                "- **{}** → fix flow `{}` ({})\n",
                path.path.as_str(),
                path.fix_flow.mode.as_str(),
                path.fix_flow.label,
            ));
            out.push_str(&format!(
                "  - Findings: {} · silent rewrite blocked: {} · preview required: {}\n",
                path.findings_present,
                path.silent_byte_rewrite_blocked,
                path.fix_flow.preview_required,
            ));
            if !path.fix_flow.fix_kinds_offered.is_empty() {
                out.push_str(&format!(
                    "  - Fix kinds: {}\n",
                    path.fix_flow.fix_kinds_offered.join(", ")
                ));
            }
            if let Some(s) = &path.recorded_suppression {
                out.push_str(&format!(
                    "  - Suppression: scope `{}`, actor `{}`, audit `{}`\n",
                    s.scope.as_str(),
                    s.actor_ref,
                    s.audit_log_ref,
                ));
            }
        }
        out
    }
}

/// Validation failures emitted by [`M5MutationPathFixFlowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MutationPathFixFlowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Projection normalized or stripped the source bytes.
    NormalizationApplied,
    /// A required mutation path is missing.
    MutationPathMissing,
    /// A path did not block silent suspicious-byte rewrites.
    SilentRewriteNotBlocked,
    /// A fix flow did not require a preview before bytes change.
    PreviewNotRequired,
    /// A path's offered fix kinds did not cover its findings.
    FixKindsMissing,
    /// A recorded suppression was not scope-aware or auditable.
    SuppressionNotAuditable,
    /// A clean path carried a suppression for a non-existent finding.
    SuppressionWithoutFinding,
    /// A recorded suppression record is malformed.
    SuppressionRecordMalformed,
    /// A path did not preserve the guard across carriers.
    GuardNotPreserved,
    /// The declared path count does not match the projections.
    PathCountMismatch,
    /// The declared findings count does not match the projections.
    FindingsCountMismatch,
    /// The declared suppressed-path count does not match the projections.
    SuppressedCountMismatch,
    /// A fix flow's mode does not match its path's declared preview route.
    FixFlowModeMismatch,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5MutationPathFixFlowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NormalizationApplied => "normalization_applied",
            Self::MutationPathMissing => "mutation_path_missing",
            Self::SilentRewriteNotBlocked => "silent_rewrite_not_blocked",
            Self::PreviewNotRequired => "preview_not_required",
            Self::FixKindsMissing => "fix_kinds_missing",
            Self::SuppressionNotAuditable => "suppression_not_auditable",
            Self::SuppressionWithoutFinding => "suppression_without_finding",
            Self::SuppressionRecordMalformed => "suppression_record_malformed",
            Self::GuardNotPreserved => "guard_not_preserved",
            Self::PathCountMismatch => "path_count_mismatch",
            Self::FindingsCountMismatch => "findings_count_mismatch",
            Self::SuppressedCountMismatch => "suppressed_count_mismatch",
            Self::FixFlowModeMismatch => "fix_flow_mode_mismatch",
            Self::ReviewIncomplete => "review_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in mutation-path fix-flow export.
#[derive(Debug)]
pub enum M5MutationPathFixFlowExportError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MutationPathFixFlowViolation>),
}

impl std::fmt::Display for M5MutationPathFixFlowExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 mutation-path fix-flow export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 mutation-path fix-flow export failed validation: {tokens}"
                )
            }
        }
    }
}

impl std::error::Error for M5MutationPathFixFlowExportError {}

/// Resolves a single mutation path's preview-first fix flow and suppression
/// audit by running the shared suspicious-content detector over the content.
fn resolve_path(input: &M5MutationPathInput<'_>) -> M5MutationPathProjection {
    let path = input.path;
    let detection = detect_suspicious_content(input.content_excerpt);
    let escaped = escape_for_safe_inspection(input.content_excerpt);

    let threat_classes = distinct_tokens(detection.findings.iter().map(|f| f.class.as_str()));
    let findings_present = detection.has_findings();

    // Fix kinds offered mirror the detected suspicious classes exactly: a path
    // with no findings offers no fix, a path with findings offers a fix for each
    // distinct detected class.
    let fix_kinds_offered =
        distinct_tokens(detection.findings.iter().filter_map(|f| {
            M5FixKind::from_detector_class(f.class.as_str()).map(M5FixKind::as_str)
        }));

    let mode = path.preview_route();
    let fix_flow = M5FixFlow {
        mode,
        mode_token: mode.as_str().to_owned(),
        label: mode.label().to_owned(),
        preview_required: true,
        shows_raw_and_proposed: true,
        fix_kinds_offered: fix_kinds_offered.clone(),
    };

    let recorded_suppression = input
        .suppression
        .filter(|_| findings_present)
        .map(|seed| build_suppression(&seed, &fix_kinds_offered, input.artifact_ref));

    let rationale = build_rationale(
        path,
        findings_present,
        &threat_classes,
        &recorded_suppression,
    );

    M5MutationPathProjection {
        path,
        path_token: path.as_str().to_owned(),
        path_display_name: path.display_name().to_owned(),
        artifact_ref: input.artifact_ref.to_owned(),
        raw_content_ref: input.raw_content_ref.to_owned(),
        suspicious_excerpt_escaped: escaped,
        findings_present,
        finding_count: detection.findings.len(),
        threat_classes,
        detector_outcome_token: detection.outcome.as_str().to_owned(),
        fix_flow,
        silent_byte_rewrite_blocked: true,
        bytes_change_only_after_preview: true,
        suppression_audit: M5SuppressionAudit::frozen(),
        recorded_suppression,
        preserved_in_product: true,
        preserved_in_exported_review_packet: true,
        preserved_in_support_handoff: true,
        rationale,
    }
}

fn build_suppression(
    seed: &M5SuppressionSeed<'_>,
    fix_kinds_offered: &[String],
    artifact_ref: &str,
) -> M5SuppressionRecord {
    M5SuppressionRecord {
        suppression_id: seed.suppression_id.to_owned(),
        scope: seed.scope,
        scope_token: seed.scope.as_str().to_owned(),
        suppressed_fix_kinds: fix_kinds_offered.to_vec(),
        actor_ref: seed.actor_ref.to_owned(),
        reason: seed.reason.to_owned(),
        recorded_at: seed.recorded_at.to_owned(),
        expires_at: seed.expires_at.map(str::to_owned),
        audit_log_ref: seed.audit_log_ref.to_owned(),
        finding_artifact_ref: artifact_ref.to_owned(),
        hidden_per_pane_state: false,
    }
}

fn build_rationale(
    path: M5MutationPath,
    findings_present: bool,
    threat_classes: &[String],
    suppression: &Option<M5SuppressionRecord>,
) -> String {
    let route = path.preview_route().as_str();
    if findings_present {
        let suppression_note = match suppression {
            Some(s) => format!(
                " A {} suppression was recorded by {} with a reachable audit log.",
                s.scope.as_str(),
                s.actor_ref
            ),
            None => String::new(),
        };
        format!(
            "{} touches content the detector flagged ({}); the fix routes through a {} before any bytes change and the silent-rewrite guard stays on.{}",
            path.display_name(),
            threat_classes.join(", "),
            route,
            suppression_note
        )
    } else {
        format!(
            "{} touches clean content, but the silent-rewrite guard stays on and a fix would still route through a {} before any bytes change.",
            path.display_name(),
            route
        )
    }
}

fn distinct_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> Vec<String> {
    let set: BTreeSet<&str> = tokens.collect();
    set.into_iter().map(str::to_owned).collect()
}

/// Projects preview-first fix flows, auditable suppressions, and the
/// no-silent-byte-rewrite guard across every new M5 mutation path.
pub fn project_m5_mutation_path_fix_flow(
    seed: &M5MutationPathFixFlowSeed<'_>,
) -> M5MutationPathFixFlowPacket {
    let paths: Vec<_> = seed.path_inputs.iter().map(resolve_path).collect();

    let paths_with_findings_count = paths.iter().filter(|p| p.findings_present).count();
    let suppressed_path_count = paths
        .iter()
        .filter(|p| p.recorded_suppression.is_some())
        .count();
    let fix_flow_modes = distinct_tokens(paths.iter().map(|p| p.fix_flow.mode.as_str()));
    let recorded_suppression_scopes = distinct_tokens(
        paths
            .iter()
            .filter_map(|p| p.recorded_suppression.as_ref().map(|s| s.scope.as_str())),
    );

    M5MutationPathFixFlowPacket {
        record_kind: M5_MUTATION_PATH_FIX_FLOW_RECORD_KIND.to_owned(),
        schema_version: M5_MUTATION_PATH_FIX_FLOW_SCHEMA_VERSION,
        packet_id: M5_MUTATION_PATH_FIX_FLOW_PACKET_ID.to_owned(),
        case_id: seed.case_id.to_owned(),
        path_count: paths.len(),
        paths_with_findings_count,
        suppressed_path_count,
        fix_flow_modes,
        recorded_suppression_scopes,
        normalization_applied: false,
        paths,
        review: M5MutationPathFixFlowReview::frozen(),
        source_contract_refs: vec![
            M5_MUTATION_PATH_FIX_FLOW_SCHEMA_REF.to_owned(),
            M5_MUTATION_PATH_FIX_FLOW_DOC_REF.to_owned(),
            M5_MUTATION_PATH_FIX_FLOW_SUSPICIOUS_TEXT_CONTRACT_REF.to_owned(),
            M5_MUTATION_PATH_FIX_FLOW_CONTENT_INTEGRITY_MATRIX_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: seed.minted_at.to_owned(),
    }
}

/// Builds the canonical frozen mutation-path fix-flow packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_MUTATION_PATH_FIX_FLOW_ARTIFACT_REF`]; the bin emits this packet and a
/// test asserts the checked-in artifact deserializes back to it unchanged. The
/// scenario exercises a save over a bidi control suppressed at file scope, a
/// format over an invisible zero-width suppressed at workspace scope, an
/// organize-imports over a mixed-script confusable suppressed at occurrence
/// scope, and an AI-apply over content carrying all three suspicious classes
/// suppressed at admin-policy scope with an expiry.
pub fn frozen_m5_mutation_path_fix_flow_packet() -> M5MutationPathFixFlowPacket {
    use M5MutationPath as Path;

    let path_inputs = [
        // Save over a doc page carrying a right-to-left override (U+202E).
        M5MutationPathInput {
            path: Path::Save,
            artifact_ref: "docs:page:runbook",
            raw_content_ref: "docs:page:runbook:raw",
            content_excerpt: "title: Allow \u{202E}egress",
            suppression: Some(M5SuppressionSeed {
                suppression_id: "suppression:save:0001",
                scope: M5SuppressionScope::File,
                actor_ref: "actor:user:dana",
                reason: "Reviewed: the override is intentional sample text in a runbook.",
                recorded_at: "2026-06-10T00:00:00Z",
                expires_at: None,
                audit_log_ref: "audit:suppression:save:0001",
            }),
        },
        // Format over a notebook cell hiding a zero-width space (U+200B).
        M5MutationPathInput {
            path: Path::Format,
            artifact_ref: "notebook:cell:7",
            raw_content_ref: "notebook:cell:7:raw",
            content_excerpt: "value = build\u{200B}_id",
            suppression: Some(M5SuppressionSeed {
                suppression_id: "suppression:format:0001",
                scope: M5SuppressionScope::Workspace,
                actor_ref: "actor:user:dana",
                reason: "Reviewed: the zero-width byte is preserved for an upstream fixture.",
                recorded_at: "2026-06-10T00:00:00Z",
                expires_at: Some("2026-09-10T00:00:00Z"),
                audit_log_ref: "audit:suppression:format:0001",
            }),
        },
        // Organize-imports over a structured artifact with a Cyrillic confusable.
        M5MutationPathInput {
            path: Path::OrganizeImports,
            artifact_ref: "structured:artifact:config",
            raw_content_ref: "structured:artifact:config:raw",
            content_excerpt: "import p\u{0430}yload",
            suppression: Some(M5SuppressionSeed {
                suppression_id: "suppression:organize:0001",
                scope: M5SuppressionScope::Occurrence,
                actor_ref: "actor:user:dana",
                reason: "Reviewed: this single confusable identifier is expected here.",
                recorded_at: "2026-06-10T00:00:00Z",
                expires_at: None,
                audit_log_ref: "audit:suppression:organize:0001",
            }),
        },
        // AI-apply over AI-evidence text carrying bidi, invisible, and confusable.
        M5MutationPathInput {
            path: Path::AiApply,
            artifact_ref: "ai:evidence:patch-1",
            raw_content_ref: "ai:evidence:patch-1:raw",
            content_excerpt: "fix \u{202E}p\u{0430}yload\u{200B} now",
            suppression: Some(M5SuppressionSeed {
                suppression_id: "suppression:ai-apply:0001",
                scope: M5SuppressionScope::AdminPolicy,
                actor_ref: "actor:admin:policy-owner",
                reason:
                    "Admin policy allows this evidence excerpt under review until the audit closes.",
                recorded_at: "2026-06-10T00:00:00Z",
                expires_at: Some("2026-12-10T00:00:00Z"),
                audit_log_ref: "audit:suppression:ai-apply:0001",
            }),
        },
    ];

    project_m5_mutation_path_fix_flow(&M5MutationPathFixFlowSeed {
        case_id: "case:m5-mutation-path-fix-flow:stable",
        path_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

/// Reads and validates the checked-in mutation-path fix-flow support export.
pub fn current_m5_mutation_path_fix_flow_export(
) -> Result<M5MutationPathFixFlowPacket, M5MutationPathFixFlowExportError> {
    let packet: M5MutationPathFixFlowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/m5_mutation_path_fix_flow/support_export.json"
    )))
    .map_err(M5MutationPathFixFlowExportError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MutationPathFixFlowExportError::Validation(violations))
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
