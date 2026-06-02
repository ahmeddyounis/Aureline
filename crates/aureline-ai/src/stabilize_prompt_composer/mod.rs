//! Stable prompt-composer stabilization records.
//!
//! This module promotes the beta prompt-composer conformance lane to the stable
//! line. It does not re-derive composer truth: the beta
//! [`crate::prompt_composer::PromptComposerConformancePacket`] remains canonical
//! for intent, mentions, attachments, slash commands, budget, draft retention,
//! and evidence lineage. The stable packet references that conformance packet by
//! id and adds the review-pass deltas required before any send-capable composer
//! row may claim Stable: typed attachment/mention origin-trust-freshness
//! semantics, **Pinned but stale** truth, omitted-context inspectability that
//! survives send, forked-thread lineage, compare-answer same-versus-different
//! context truth, context-drift banners, and a thread header that previews what
//! a Remember/Save affordance retains, where it lives, and who can reuse it.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes, and
//! review labels only; raw prompt bodies, source file bodies, provider payloads,
//! endpoint URLs, credentials, raw token counts, exact prices, and billing
//! account ids stay outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::context_inspector::{ContextFreshnessClass, ContextOmissionReasonClass};
use crate::prompt_composer::PromptComposerConformancePacket;
use crate::SourceClass;

/// Stable record-kind tag carried by [`PromptComposerStabilizationPacket`].
pub const PROMPT_COMPOSER_STABILIZATION_RECORD_KIND: &str = "prompt_composer_stabilization_packet";

/// Schema version for prompt-composer stabilization packets.
pub const PROMPT_COMPOSER_STABILIZATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the stabilization boundary schema.
pub const PROMPT_COMPOSER_STABILIZATION_SCHEMA_REF: &str =
    "schemas/ai/prompt_composer_stabilization.schema.json";

/// Repo-relative path of the stable prompt-composer contract doc.
pub const PROMPT_COMPOSER_STABILIZATION_AI_DOC_REF: &str =
    "docs/ai/m4/stabilize_prompt_composer.md";

/// Repo-relative path of the frozen AI prompt-composer contract.
pub const PROMPT_COMPOSER_STABILIZATION_BASE_CONTRACT_REF: &str =
    "docs/ai/prompt_composer_contract.md";

/// Repo-relative path of the beta conformance export this lane promotes.
pub const PROMPT_COMPOSER_STABILIZATION_BETA_ARTIFACT_REF: &str =
    "artifacts/ai/m3/prompt_composer_conformance/support_export.json";

/// Repo-relative path of the protected stabilization fixture directory.
pub const PROMPT_COMPOSER_STABILIZATION_FIXTURE_DIR: &str =
    "fixtures/ai/m4/prompt_composer_stabilization";

/// Repo-relative path of the checked stabilization export.
pub const PROMPT_COMPOSER_STABILIZATION_ARTIFACT_REF: &str =
    "artifacts/ai/m4/prompt_composer_stabilization/support_export.json";

/// Repo-relative path of the checked stabilization Markdown summary.
pub const PROMPT_COMPOSER_STABILIZATION_SUMMARY_REF: &str =
    "artifacts/ai/m4/prompt_composer_stabilization/summary.md";

/// Typed source classes the stable composer attaches with explicit semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableAttachmentSourceClass {
    /// A workspace file or file slice.
    WorkspaceFile,
    /// A workspace symbol or graph node.
    Symbol,
    /// A docs/knowledge-pack reference.
    DocsReference,
    /// A diagnostic record.
    Diagnostic,
    /// A test or check result.
    TestResult,
    /// A terminal or tool output capture.
    TerminalToolOutput,
    /// Free external text pasted or imported by the operator.
    ExternalText,
}

impl StableAttachmentSourceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceFile => "workspace_file",
            Self::Symbol => "symbol",
            Self::DocsReference => "docs_reference",
            Self::Diagnostic => "diagnostic",
            Self::TestResult => "test_result",
            Self::TerminalToolOutput => "terminal_tool_output",
            Self::ExternalText => "external_text",
        }
    }

    /// Source classes that must be covered before the lane claims Stable.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::WorkspaceFile,
            Self::Symbol,
            Self::DocsReference,
            Self::Diagnostic,
            Self::TestResult,
            Self::TerminalToolOutput,
            Self::ExternalText,
        ]
    }
}

/// Trust or taint class shown on each attached object before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentTaintClass {
    /// Authored or owned first-party workspace content.
    TrustedFirstParty,
    /// Derived from workspace content but not directly authored.
    WorkspaceDerived,
    /// External content of unknown provenance.
    UntrustedExternal,
    /// Tainted content held behind a quarantine fence.
    TaintedQuarantined,
}

impl AttachmentTaintClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedFirstParty => "trusted_first_party",
            Self::WorkspaceDerived => "workspace_derived",
            Self::UntrustedExternal => "untrusted_external",
            Self::TaintedQuarantined => "tainted_quarantined",
        }
    }
}

/// Current inclusion posture for an attached object before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InclusionPostureClass {
    /// Included as raw content.
    IncludedRaw,
    /// Pinned and included; cannot be silently dropped.
    IncludedPinned,
    /// Included as a summary fallback.
    Summarized,
    /// Included as a trimmed slice.
    Trimmed,
    /// Omitted but inspectable before and after send.
    OmittedInspectable,
    /// Blocked behind a quarantine or policy fence.
    BlockedQuarantined,
}

impl InclusionPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedRaw => "included_raw",
            Self::IncludedPinned => "included_pinned",
            Self::Summarized => "summarized",
            Self::Trimmed => "trimmed",
            Self::OmittedInspectable => "omitted_inspectable",
            Self::BlockedQuarantined => "blocked_quarantined",
        }
    }
}

/// One typed attachment with origin, trust, freshness, and inclusion posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableAttachmentSemanticRow {
    /// Attachment id shared with the conformance packet.
    pub attachment_id: String,
    /// Stable object identity that survives display-label changes.
    pub stable_object_ref: String,
    /// Review-safe origin label.
    pub origin_label: String,
    /// Typed source class.
    pub source_class: StableAttachmentSourceClass,
    /// Trust or taint class.
    pub taint_class: AttachmentTaintClass,
    /// Freshness class shown on the pill.
    pub freshness_class: ContextFreshnessClass,
    /// Current inclusion posture before send.
    pub inclusion_posture: InclusionPostureClass,
    /// True when keyboard users can focus and act on the pill.
    pub keyboard_reachable: bool,
    /// Screen-reader narration label.
    pub screen_reader_label: String,
}

/// Freshness state for a pinned context object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinnedFreshnessStateClass {
    /// Pinned and still matching the underlying object.
    PinnedFresh,
    /// Pinned object changed on disk, in Git, or in live state.
    PinnedButStale,
    /// Pinned object is being refreshed.
    PinnedRefreshing,
}

impl PinnedFreshnessStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedFresh => "pinned_fresh",
            Self::PinnedButStale => "pinned_but_stale",
            Self::PinnedRefreshing => "pinned_refreshing",
        }
    }
}

/// What changed underneath a pinned or previously reviewed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftSourceClass {
    /// The backing file changed on disk.
    OnDisk,
    /// The backing object changed in Git.
    InGit,
    /// A live runtime/tool object changed.
    LiveState,
    /// A docs or knowledge source changed.
    DocsSource,
    /// A test or check result changed.
    TestState,
}

impl DriftSourceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDisk => "on_disk",
            Self::InGit => "in_git",
            Self::LiveState => "live_state",
            Self::DocsSource => "docs_source",
            Self::TestState => "test_state",
        }
    }
}

/// One pinned context row that must surface staleness before reuse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedContextRow {
    /// Stable pin id.
    pub pin_id: String,
    /// Stable object identity of the pinned object.
    pub stable_object_ref: String,
    /// Review-safe display label.
    pub display_label: String,
    /// Pinned freshness state.
    pub freshness_state: PinnedFreshnessStateClass,
    /// What changed underneath the pin, when stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_source: Option<DriftSourceClass>,
    /// Refresh action ref.
    pub refresh_action_ref: String,
    /// Remove action ref.
    pub remove_action_ref: String,
    /// True when a stale pin blocks send until refreshed or removed.
    pub blocks_send_until_resolved: bool,
    /// True when keyboard users can focus, refresh, and remove the pin.
    pub keyboard_reachable: bool,
}

/// One omitted source that stays inspectable before and after send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmittedContextReviewRow {
    /// Stable object identity of the omitted source.
    pub source_ref: String,
    /// Source class token.
    pub source_class: SourceClass,
    /// Why the source was omitted.
    pub omission_reason_class: ContextOmissionReasonClass,
    /// True when the omitted source remains inspectable after send.
    pub inspectable_after_send: bool,
    /// Inspect action ref.
    pub inspect_action_ref: String,
    /// True when replay, support, and audit flows can explain the exclusion.
    pub replay_explains_exclusion: bool,
    /// True when keyboard users can reach the omitted-context review row.
    pub keyboard_reachable: bool,
}

/// Forked-thread lineage preserved by compare-answer and replay flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForkedThreadLineage {
    /// Stable thread id.
    pub thread_id: String,
    /// True when this thread forked from a parent thread or run.
    pub is_forked: bool,
    /// Parent thread ref when forked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_thread_ref: Option<String>,
    /// Parent run ref when forked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_run_ref: Option<String>,
    /// Inherited context snapshot ref captured at the fork.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited_context_snapshot_ref: Option<String>,
    /// Divergence point ref where this thread left the parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divergence_point_ref: Option<String>,
}

/// Whether compared answers reused the same context or diverged on drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextParityClass {
    /// Both answers used the same context snapshot.
    SameContextSnapshot,
    /// Answers differ because of hidden context drift.
    DifferentContextDrift,
    /// Answers differ because of an intentional, disclosed context change.
    DifferentContextIntentional,
}

impl ContextParityClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameContextSnapshot => "same_context_snapshot",
            Self::DifferentContextDrift => "different_context_drift",
            Self::DifferentContextIntentional => "different_context_intentional",
        }
    }
}

/// One compare-answer row that preserves same-versus-different context truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareAnswerRow {
    /// Stable comparison id.
    pub comparison_id: String,
    /// Left run ref.
    pub left_run_ref: String,
    /// Right run ref.
    pub right_run_ref: String,
    /// Left context snapshot ref.
    pub left_context_snapshot_ref: String,
    /// Right context snapshot ref.
    pub right_context_snapshot_ref: String,
    /// Context parity class.
    pub context_parity_class: ContextParityClass,
    /// Provider/model delta label when the runs used different routes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_model_delta_label: Option<String>,
    /// Instruction-stack delta label when the runs used different instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction_stack_delta_label: Option<String>,
    /// True when hidden context drift must be flagged to the operator.
    pub hidden_drift_warning: bool,
}

/// One context-drift banner shown when a reviewed composer state changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextDriftBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// What changed underneath the reviewed state.
    pub drift_source: DriftSourceClass,
    /// Stable object identity affected by the drift.
    pub affected_object_ref: String,
    /// Context snapshot ref the operator previously reviewed.
    pub previously_reviewed_snapshot_ref: String,
    /// True when rerun/resend must not imply the earlier snapshot still applies.
    pub requires_rereview: bool,
    /// Review-safe explanation label.
    pub explanation_label: String,
    /// True when keyboard users can reach the banner.
    pub keyboard_reachable: bool,
    /// Screen-reader narration label.
    pub screen_reader_label: String,
}

/// Composer surface the stable contract must stay consistent across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerSurfaceClass {
    /// Composer attached inline in the editor.
    EditorAttached,
    /// Composer docked in the sidebar.
    Sidebar,
    /// Detached/floating composer window.
    Detached,
}

impl ComposerSurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorAttached => "editor_attached",
            Self::Sidebar => "sidebar",
            Self::Detached => "detached",
        }
    }

    /// Surfaces that must stay consistent before the lane claims Stable.
    pub const fn required_coverage() -> [Self; 3] {
        [Self::EditorAttached, Self::Sidebar, Self::Detached]
    }
}

/// One surface-consistency row proving cross-surface keyboard/SR reachability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceConsistencyRow {
    /// Composer surface class.
    pub surface_class: ComposerSurfaceClass,
    /// True when attachment pills are keyboard reachable on this surface.
    pub attachment_pills_keyboard_reachable: bool,
    /// True when mention rows are screen-reader describable on this surface.
    pub mention_rows_screen_reader_describable: bool,
    /// True when omitted-context review is reachable on this surface.
    pub omitted_context_review_reachable: bool,
    /// True when forked-thread comparison is reachable on this surface.
    pub forked_thread_comparison_reachable: bool,
    /// True when context-drift banners are reachable on this surface.
    pub context_drift_banner_reachable: bool,
}

impl SurfaceConsistencyRow {
    fn is_fully_reachable(&self) -> bool {
        self.attachment_pills_keyboard_reachable
            && self.mention_rows_screen_reader_describable
            && self.omitted_context_review_reachable
            && self.forked_thread_comparison_reachable
            && self.context_drift_banner_reachable
    }
}

/// Retention mode shown in the thread header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreadRetentionModeClass {
    /// Thread and drafts stay local to the device/workspace.
    LocalOnly,
    /// Thread is shared at the repo scope.
    RepoShared,
    /// Thread is shared at the org scope.
    OrgShared,
    /// Nothing durable is retained.
    EphemeralNoRetention,
}

impl ThreadRetentionModeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::RepoShared => "repo_shared",
            Self::OrgShared => "org_shared",
            Self::EphemeralNoRetention => "ephemeral_no_retention",
        }
    }

    fn shares_beyond_device(self) -> bool {
        matches!(self, Self::RepoShared | Self::OrgShared)
    }
}

/// Where a remembered object will live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionLocusClass {
    /// Local device or workspace only.
    LocalDevice,
    /// Repo-scoped memory store.
    RepoScoped,
    /// Org-scoped memory store.
    OrgScoped,
}

impl RetentionLocusClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDevice => "local_device",
            Self::RepoScoped => "repo_scoped",
            Self::OrgScoped => "org_scoped",
        }
    }
}

/// Who can reuse a remembered object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReuseAudienceClass {
    /// Only the current operator.
    OnlyMe,
    /// Repo collaborators.
    RepoCollaborators,
    /// Org members.
    OrgMembers,
    /// Nobody; nothing is retained.
    Nobody,
}

impl ReuseAudienceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnlyMe => "only_me",
            Self::RepoCollaborators => "repo_collaborators",
            Self::OrgMembers => "org_members",
            Self::Nobody => "nobody",
        }
    }
}

/// Preview of what a Remember/Save affordance retains, where, and for whom.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberPreview {
    /// Review-safe summary of what will be retained.
    pub retained_summary_label: String,
    /// Where the retained object will live.
    pub retention_locus_class: RetentionLocusClass,
    /// Who can reuse the retained object.
    pub reuse_audience_class: ReuseAudienceClass,
    /// Memory class token shared with the AI memory model.
    pub memory_class_token: String,
    /// Preview action ref.
    pub preview_action_ref: String,
}

/// Thread header showing scope, route, retention, and memory access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadHeaderRow {
    /// Stable thread id.
    pub thread_id: String,
    /// Current scope label.
    pub current_scope_label: String,
    /// Selected provider label.
    pub provider_label: String,
    /// Selected model label.
    pub model_label: String,
    /// Retention mode for the thread.
    pub retention_mode_class: ThreadRetentionModeClass,
    /// Memory class token shared with the AI memory model.
    pub memory_class_token: String,
    /// Save-memory action ref.
    pub save_memory_action_ref: String,
    /// Delete action ref.
    pub delete_action_ref: String,
    /// Export action ref.
    pub export_action_ref: String,
    /// Remember/Save preview.
    pub remember_preview: RememberPreview,
}

/// Constructor input for [`PromptComposerStabilizationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptComposerStabilizationInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label.
    pub display_label: String,
    /// Conformance packet id this lane promotes.
    pub composer_conformance_packet_ref: String,
    /// Conformance context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Conformance session ref.
    pub composer_session_ref: String,
    /// Conformance draft ref.
    pub composer_draft_ref: String,
    /// Thread header row.
    pub thread_header: ThreadHeaderRow,
    /// Typed attachment semantic rows.
    pub attachment_semantic_rows: Vec<StableAttachmentSemanticRow>,
    /// Pinned context rows.
    pub pinned_context_rows: Vec<PinnedContextRow>,
    /// Omitted-context review rows.
    pub omitted_context_review_rows: Vec<OmittedContextReviewRow>,
    /// Forked-thread lineage.
    pub forked_thread_lineage: ForkedThreadLineage,
    /// Compare-answer rows.
    pub compare_answer_rows: Vec<CompareAnswerRow>,
    /// Context-drift banners.
    pub context_drift_banners: Vec<ContextDriftBanner>,
    /// Surface-consistency rows.
    pub surface_consistency_rows: Vec<SurfaceConsistencyRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe prompt-composer stabilization packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptComposerStabilizationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label.
    pub display_label: String,
    /// Conformance packet id this lane promotes.
    pub composer_conformance_packet_ref: String,
    /// Conformance context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Conformance session ref.
    pub composer_session_ref: String,
    /// Conformance draft ref.
    pub composer_draft_ref: String,
    /// Thread header showing scope, route, retention, and memory access.
    pub thread_header: ThreadHeaderRow,
    /// Typed attachment semantic rows.
    pub attachment_semantic_rows: Vec<StableAttachmentSemanticRow>,
    /// Pinned context rows that surface staleness before reuse.
    pub pinned_context_rows: Vec<PinnedContextRow>,
    /// Omitted-context review rows that survive send.
    pub omitted_context_review_rows: Vec<OmittedContextReviewRow>,
    /// Forked-thread lineage.
    pub forked_thread_lineage: ForkedThreadLineage,
    /// Compare-answer rows.
    pub compare_answer_rows: Vec<CompareAnswerRow>,
    /// Context-drift banners.
    pub context_drift_banners: Vec<ContextDriftBanner>,
    /// Surface-consistency rows.
    pub surface_consistency_rows: Vec<SurfaceConsistencyRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PromptComposerStabilizationPacket {
    /// Builds a stabilization packet from the stable-lane input.
    pub fn new(input: PromptComposerStabilizationInput) -> Self {
        Self {
            record_kind: PROMPT_COMPOSER_STABILIZATION_RECORD_KIND.to_owned(),
            schema_version: PROMPT_COMPOSER_STABILIZATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            composer_conformance_packet_ref: input.composer_conformance_packet_ref,
            composer_context_snapshot_ref: input.composer_context_snapshot_ref,
            composer_session_ref: input.composer_session_ref,
            composer_draft_ref: input.composer_draft_ref,
            thread_header: input.thread_header,
            attachment_semantic_rows: input.attachment_semantic_rows,
            pinned_context_rows: input.pinned_context_rows,
            omitted_context_review_rows: input.omitted_context_review_rows,
            forked_thread_lineage: input.forked_thread_lineage,
            compare_answer_rows: input.compare_answer_rows,
            context_drift_banners: input.context_drift_banners,
            surface_consistency_rows: input.surface_consistency_rows,
            source_contract_refs: input.source_contract_refs,
            json_export_ref: input.json_export_ref,
            markdown_summary_ref: input.markdown_summary_ref,
            minted_at: input.minted_at,
        }
    }

    /// Validates the stabilization packet and the conformance packet it promotes.
    pub fn validate(
        &self,
        conformance: &PromptComposerConformancePacket,
    ) -> Vec<PromptComposerStabilizationViolation> {
        let mut violations = self.validate_self();
        if !conformance.validate().is_empty()
            || conformance.packet_id != self.composer_conformance_packet_ref
            || conformance.composer_context_snapshot_ref != self.composer_context_snapshot_ref
            || conformance.composer_session_id != self.composer_session_ref
            || conformance.composer_draft_id != self.composer_draft_ref
        {
            violations.push(PromptComposerStabilizationViolation::EmbeddedConformanceInvalid);
        }
        violations
    }

    /// Validates only the stabilization packet's own stable-line invariants.
    pub fn validate_self(&self) -> Vec<PromptComposerStabilizationViolation> {
        let mut violations = Vec::new();
        if self.record_kind != PROMPT_COMPOSER_STABILIZATION_RECORD_KIND {
            violations.push(PromptComposerStabilizationViolation::WrongRecordKind);
        }
        if self.schema_version != PROMPT_COMPOSER_STABILIZATION_SCHEMA_VERSION {
            violations.push(PromptComposerStabilizationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.composer_conformance_packet_ref.trim().is_empty()
            || self.composer_context_snapshot_ref.trim().is_empty()
            || self.composer_session_ref.trim().is_empty()
            || self.composer_draft_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PromptComposerStabilizationViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_thread_header(self, &mut violations);
        validate_attachment_semantics(self, &mut violations);
        validate_pinned_context(self, &mut violations);
        validate_omitted_context(self, &mut violations);
        validate_forked_thread(self, &mut violations);
        validate_compare_answers(self, &mut violations);
        validate_context_drift(self, &mut violations);
        validate_surface_consistency(self, &mut violations);
        if self.json_export_ref.trim().is_empty() || self.markdown_summary_ref.trim().is_empty() {
            violations.push(PromptComposerStabilizationViolation::ExportRefsMissing);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("prompt composer stabilization packet serializes"),
        ) {
            violations.push(PromptComposerStabilizationViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("prompt composer stabilization packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Prompt Composer Stabilization\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Promotes conformance: `{}`\n",
            self.composer_conformance_packet_ref
        ));
        out.push_str(&format!(
            "- Thread: `{}` / scope `{}` / route `{} / {}`\n",
            self.thread_header.thread_id,
            self.thread_header.current_scope_label,
            self.thread_header.provider_label,
            self.thread_header.model_label
        ));
        out.push_str(&format!(
            "- Retention: `{}` (remember -> `{}` / `{}`)\n",
            self.thread_header.retention_mode_class.as_str(),
            self.thread_header
                .remember_preview
                .retention_locus_class
                .as_str(),
            self.thread_header
                .remember_preview
                .reuse_audience_class
                .as_str()
        ));
        out.push_str(&format!(
            "- Attachments / pinned / omitted-review: {} / {} / {}\n",
            self.attachment_semantic_rows.len(),
            self.pinned_context_rows.len(),
            self.omitted_context_review_rows.len()
        ));
        out.push_str(&format!(
            "- Forked thread: `{}` (forked: {})\n",
            self.forked_thread_lineage.thread_id, self.forked_thread_lineage.is_forked
        ));
        out.push_str(&format!(
            "- Compare-answer rows / drift banners / surfaces: {} / {} / {}\n",
            self.compare_answer_rows.len(),
            self.context_drift_banners.len(),
            self.surface_consistency_rows.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in stabilization export.
#[derive(Debug)]
pub enum PromptComposerStabilizationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PromptComposerStabilizationViolation>),
}

impl fmt::Display for PromptComposerStabilizationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "prompt composer stabilization export parse failed: {error}"
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
                    "prompt composer stabilization export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PromptComposerStabilizationArtifactError {}

/// Validation failures emitted by [`PromptComposerStabilizationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptComposerStabilizationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The promoted conformance packet is invalid or mismatched.
    EmbeddedConformanceInvalid,
    /// Thread header scope, route, retention, or memory access is incomplete.
    ThreadHeaderIncomplete,
    /// A Remember/Save affordance does not preview retention, locus, or audience.
    RememberPreviewIncomplete,
    /// An attachment row lacks origin, trust, freshness, posture, or narration.
    AttachmentSemanticIncomplete,
    /// Required typed source classes are not covered.
    AttachmentSourceClassCoverageMissing,
    /// A pinned-but-stale row does not surface staleness before reuse.
    PinnedStaleNotSurfaced,
    /// An omitted source is not inspectable after send.
    OmittedContextNotInspectable,
    /// Forked-thread lineage is incomplete.
    ForkedThreadLineageIncomplete,
    /// Compare-answer truth is missing same-versus-different context state.
    CompareAnswerTruthMissing,
    /// A context-drift banner is incomplete or implies the old snapshot applies.
    ContextDriftBannerIncomplete,
    /// Cross-surface keyboard/screen-reader consistency is not proven.
    SurfaceConsistencyMissing,
    /// Export refs are missing.
    ExportRefsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PromptComposerStabilizationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::EmbeddedConformanceInvalid => "embedded_conformance_invalid",
            Self::ThreadHeaderIncomplete => "thread_header_incomplete",
            Self::RememberPreviewIncomplete => "remember_preview_incomplete",
            Self::AttachmentSemanticIncomplete => "attachment_semantic_incomplete",
            Self::AttachmentSourceClassCoverageMissing => {
                "attachment_source_class_coverage_missing"
            }
            Self::PinnedStaleNotSurfaced => "pinned_stale_not_surfaced",
            Self::OmittedContextNotInspectable => "omitted_context_not_inspectable",
            Self::ForkedThreadLineageIncomplete => "forked_thread_lineage_incomplete",
            Self::CompareAnswerTruthMissing => "compare_answer_truth_missing",
            Self::ContextDriftBannerIncomplete => "context_drift_banner_incomplete",
            Self::SurfaceConsistencyMissing => "surface_consistency_missing",
            Self::ExportRefsMissing => "export_refs_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in prompt-composer stabilization export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_prompt_composer_stabilization_export(
) -> Result<PromptComposerStabilizationPacket, PromptComposerStabilizationArtifactError> {
    let packet: PromptComposerStabilizationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/prompt_composer_stabilization/support_export.json"
    )))
    .map_err(PromptComposerStabilizationArtifactError::SupportExport)?;
    let violations = packet.validate_self();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PromptComposerStabilizationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for required in [
        PROMPT_COMPOSER_STABILIZATION_AI_DOC_REF,
        PROMPT_COMPOSER_STABILIZATION_BASE_CONTRACT_REF,
        PROMPT_COMPOSER_STABILIZATION_SCHEMA_REF,
        PROMPT_COMPOSER_STABILIZATION_BETA_ARTIFACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(PromptComposerStabilizationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_thread_header(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    let header = &packet.thread_header;
    if header.thread_id.trim().is_empty()
        || header.current_scope_label.trim().is_empty()
        || header.provider_label.trim().is_empty()
        || header.model_label.trim().is_empty()
        || header.memory_class_token.trim().is_empty()
        || header.save_memory_action_ref.trim().is_empty()
        || header.delete_action_ref.trim().is_empty()
        || header.export_action_ref.trim().is_empty()
    {
        violations.push(PromptComposerStabilizationViolation::ThreadHeaderIncomplete);
    }
    let preview = &header.remember_preview;
    let summary_describes_retention = !preview.retained_summary_label.trim().is_empty();
    let audience_matches_mode = match header.retention_mode_class {
        ThreadRetentionModeClass::EphemeralNoRetention => {
            preview.retention_locus_class == RetentionLocusClass::LocalDevice
                && preview.reuse_audience_class == ReuseAudienceClass::Nobody
        }
        ThreadRetentionModeClass::LocalOnly => {
            preview.retention_locus_class == RetentionLocusClass::LocalDevice
        }
        _ => {
            header.retention_mode_class.shares_beyond_device()
                && preview.reuse_audience_class != ReuseAudienceClass::Nobody
        }
    };
    if !summary_describes_retention
        || preview.preview_action_ref.trim().is_empty()
        || preview.memory_class_token.trim().is_empty()
        || !audience_matches_mode
    {
        violations.push(PromptComposerStabilizationViolation::RememberPreviewIncomplete);
    }
}

fn validate_attachment_semantics(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for row in &packet.attachment_semantic_rows {
        if row.attachment_id.trim().is_empty()
            || row.stable_object_ref.trim().is_empty()
            || row.origin_label.trim().is_empty()
            || row.screen_reader_label.trim().is_empty()
            || !row.keyboard_reachable
        {
            violations.push(PromptComposerStabilizationViolation::AttachmentSemanticIncomplete);
            break;
        }
    }
    for required in StableAttachmentSourceClass::required_coverage() {
        if !packet
            .attachment_semantic_rows
            .iter()
            .any(|row| row.source_class == required)
        {
            violations
                .push(PromptComposerStabilizationViolation::AttachmentSourceClassCoverageMissing);
            break;
        }
    }
}

fn validate_pinned_context(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for row in &packet.pinned_context_rows {
        let stale = row.freshness_state == PinnedFreshnessStateClass::PinnedButStale;
        if row.pin_id.trim().is_empty()
            || row.stable_object_ref.trim().is_empty()
            || row.refresh_action_ref.trim().is_empty()
            || row.remove_action_ref.trim().is_empty()
            || !row.keyboard_reachable
        {
            violations.push(PromptComposerStabilizationViolation::PinnedStaleNotSurfaced);
            return;
        }
        // A pinned object that drifted underneath must read as Pinned but stale
        // and block silent reuse until the operator refreshes or removes it.
        if row.drift_source.is_some() && !stale {
            violations.push(PromptComposerStabilizationViolation::PinnedStaleNotSurfaced);
            return;
        }
        if stale && (row.drift_source.is_none() || !row.blocks_send_until_resolved) {
            violations.push(PromptComposerStabilizationViolation::PinnedStaleNotSurfaced);
            return;
        }
    }
}

fn validate_omitted_context(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for row in &packet.omitted_context_review_rows {
        if row.source_ref.trim().is_empty()
            || row.inspect_action_ref.trim().is_empty()
            || !row.inspectable_after_send
            || !row.replay_explains_exclusion
            || !row.keyboard_reachable
        {
            violations.push(PromptComposerStabilizationViolation::OmittedContextNotInspectable);
            break;
        }
    }
}

fn validate_forked_thread(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    let lineage = &packet.forked_thread_lineage;
    if lineage.thread_id.trim().is_empty() {
        violations.push(PromptComposerStabilizationViolation::ForkedThreadLineageIncomplete);
        return;
    }
    if lineage.is_forked {
        let has_parent = lineage
            .parent_thread_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
            || lineage
                .parent_run_ref
                .as_deref()
                .is_some_and(|reference| !reference.trim().is_empty());
        let has_inherited = lineage
            .inherited_context_snapshot_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        let has_divergence = lineage
            .divergence_point_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        if !has_parent || !has_inherited || !has_divergence {
            violations.push(PromptComposerStabilizationViolation::ForkedThreadLineageIncomplete);
        }
    }
}

fn validate_compare_answers(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for row in &packet.compare_answer_rows {
        if row.comparison_id.trim().is_empty()
            || row.left_run_ref.trim().is_empty()
            || row.right_run_ref.trim().is_empty()
            || row.left_context_snapshot_ref.trim().is_empty()
            || row.right_context_snapshot_ref.trim().is_empty()
        {
            violations.push(PromptComposerStabilizationViolation::CompareAnswerTruthMissing);
            break;
        }
        match row.context_parity_class {
            ContextParityClass::SameContextSnapshot => {
                if row.left_context_snapshot_ref != row.right_context_snapshot_ref
                    || row.hidden_drift_warning
                {
                    violations
                        .push(PromptComposerStabilizationViolation::CompareAnswerTruthMissing);
                    break;
                }
            }
            ContextParityClass::DifferentContextDrift => {
                if row.left_context_snapshot_ref == row.right_context_snapshot_ref
                    || !row.hidden_drift_warning
                {
                    violations
                        .push(PromptComposerStabilizationViolation::CompareAnswerTruthMissing);
                    break;
                }
            }
            ContextParityClass::DifferentContextIntentional => {
                if row.left_context_snapshot_ref == row.right_context_snapshot_ref {
                    violations
                        .push(PromptComposerStabilizationViolation::CompareAnswerTruthMissing);
                    break;
                }
            }
        }
    }
}

fn validate_context_drift(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for banner in &packet.context_drift_banners {
        if banner.banner_id.trim().is_empty()
            || banner.affected_object_ref.trim().is_empty()
            || banner.previously_reviewed_snapshot_ref.trim().is_empty()
            || banner.explanation_label.trim().is_empty()
            || banner.screen_reader_label.trim().is_empty()
            || !banner.requires_rereview
            || !banner.keyboard_reachable
        {
            violations.push(PromptComposerStabilizationViolation::ContextDriftBannerIncomplete);
            break;
        }
    }
}

fn validate_surface_consistency(
    packet: &PromptComposerStabilizationPacket,
    violations: &mut Vec<PromptComposerStabilizationViolation>,
) {
    for required in ComposerSurfaceClass::required_coverage() {
        let covered = packet
            .surface_consistency_rows
            .iter()
            .find(|row| row.surface_class == required);
        match covered {
            Some(row) if row.is_fully_reachable() => {}
            _ => {
                violations.push(PromptComposerStabilizationViolation::SurfaceConsistencyMissing);
                return;
            }
        }
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}

#[cfg(test)]
mod tests;
