//! Framework generators and codemods with preview, diff, rollback, and
//! execution-context reuse.
//!
//! This module locks the canonical, export-safe packet for the framework
//! generator and codemod run lane. Each [`GeneratorRunRow`] binds one generator
//! or codemod run — a scaffold generation, a source-rewriting codemod, a
//! framework migration, a structural refactor, or a config generation — to its
//! pinned generator version, whether a preview was produced, whether the change
//! diff was reviewed, whether the run can be rolled back, whether a warm
//! execution context was reused, how run-fresh it is, the support class on which
//! it may be offered, and its downgrade banner. The generator gallery, preview
//! pane, diff-review, run, rollback/recovery, diagnostics, and support surfaces
//! project the same truth about whether a generator may be run — and on what
//! terms — instead of letting starter convenience outrun provenance, preview, or
//! rollback.
//!
//! The packet is metadata only. Raw source bodies, raw diffs, generated file
//! contents, repository URLs, hostnames, secrets, and user-authored content never
//! cross this boundary; rows carry opaque refs, closed-vocabulary class tokens,
//! short reviewable summaries, structural locators, and export-safe chip labels.
//! It references the upstream template-manifest and framework-pack contracts by
//! ref rather than embedding them, and reuses the prior support-class and
//! downgrade vocabulary instead of inventing parallel terms.
//!
//! [`GeneratorRunPacket::apply_downgrade_automation`] narrows runs whose preview
//! went unavailable, whose change diff could not be produced, whose rollback
//! handle was lost, whose warm execution context could not be reused, whose run
//! record went stale, or whose proof or upstream dependency narrowed — withholding
//! confident display and surfacing a downgrade banner rather than hiding the run,
//! so CI or release tooling narrows a stale or unrunnable generator before it is
//! offered.
//!
//! The boundary schema is
//! [`schemas/templates/implement-framework-generators-or-codemods-with-preview-diff-rollback-and-execution-context-reuse.schema.json`](../../../../schemas/templates/implement-framework-generators-or-codemods-with-preview-diff-rollback-and-execution-context-reuse.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse.md`](../../../../docs/frameworks/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/`](../../../../fixtures/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`GeneratorRunPacket`].
pub const GENERATOR_RUN_RECORD_KIND: &str = "framework_generator_run_preview_diff_rollback_rows";

/// Schema version for generator-run packets.
pub const GENERATOR_RUN_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GENERATOR_RUN_SCHEMA_REF: &str =
    "schemas/templates/implement-framework-generators-or-codemods-with-preview-diff-rollback-and-execution-context-reuse.schema.json";

/// Repo-relative path of the contract doc.
pub const GENERATOR_RUN_DOC_REF: &str =
    "docs/frameworks/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse.md";

/// Repo-relative path of the upstream template-manifest contract this packet references.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream framework-pack contract this packet references.
pub const FRAMEWORK_PACK_CONTRACT_REF: &str =
    "schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json";

/// Repo-relative path of the upstream generation diff-review and recovery contract.
pub const GENERATION_RECOVERY_CONTRACT_REF: &str =
    "schemas/templates/add-generation-diff-review-rollback-or-delete-generated-recovery-and-managed-zone-honesty.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const GENERATOR_RUN_FIXTURE_DIR: &str =
    "fixtures/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse";

/// Repo-relative path of the checked support-export artifact.
pub const GENERATOR_RUN_ARTIFACT_REF: &str =
    "artifacts/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/support_export.json";

/// Which kind of generator or codemod produced a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorKind {
    /// A scaffold generator that emits new files from a template.
    ScaffoldGenerator,
    /// A codemod that rewrites existing source in place.
    Codemod,
    /// A framework-version migration codemod.
    MigrationCodemod,
    /// A structural refactor generator.
    RefactorGenerator,
    /// A configuration or manifest generator.
    ConfigGenerator,
}

impl GeneratorKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScaffoldGenerator => "scaffold_generator",
            Self::Codemod => "codemod",
            Self::MigrationCodemod => "migration_codemod",
            Self::RefactorGenerator => "refactor_generator",
            Self::ConfigGenerator => "config_generator",
        }
    }
}

/// Whether a preview of the change is available — a generator-convenience guardrail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewClass {
    /// A full preview of every effect is computed and shown.
    PreviewAvailable,
    /// A preview is shown but some effects could not be previewed.
    PreviewPartial,
    /// No preview could be produced; the run must be blocked from confident apply.
    PreviewUnavailable,
}

impl PreviewClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewAvailable => "preview_available",
            Self::PreviewPartial => "preview_partial",
            Self::PreviewUnavailable => "preview_unavailable",
        }
    }

    /// Whether no preview could be produced.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::PreviewUnavailable)
    }
}

/// Diff-review state for the change a generator or codemod produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffReviewClass {
    /// The change diff was computed and reviewed before apply.
    DiffReviewed,
    /// The change diff was computed and is awaiting review.
    DiffPending,
    /// No change diff could be produced; the run must be blocked.
    DiffUnavailable,
}

impl DiffReviewClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffReviewed => "diff_reviewed",
            Self::DiffPending => "diff_pending",
            Self::DiffUnavailable => "diff_unavailable",
        }
    }

    /// Whether no diff could be produced.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::DiffUnavailable)
    }
}

/// Whether and how a generator run can be rolled back — a recovery guardrail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackClass {
    /// A rollback handle is captured; the run can be fully undone.
    RollbackAvailable,
    /// Only a partial rollback is possible; some effects are irreversible.
    RollbackPartial,
    /// No rollback handle is available; the run must be blocked from confident apply.
    RollbackUnavailable,
    /// The run was already rolled back; the recovery action was applied.
    RolledBack,
}

impl RollbackClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackAvailable => "rollback_available",
            Self::RollbackPartial => "rollback_partial",
            Self::RollbackUnavailable => "rollback_unavailable",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Whether no rollback handle is available.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::RollbackUnavailable)
    }
}

/// Whether a warm execution context was reused for the run — the central reuse cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContextReuseClass {
    /// A warm execution context was reused for the run.
    ContextReused,
    /// A fresh execution context was created; no reuse occurred.
    ContextFresh,
    /// Reuse was requested but the warm context was unavailable; the run fell back.
    ContextReuseUnavailable,
    /// The execution-context reuse state could not be determined.
    ContextReuseUnknown,
}

impl ExecutionContextReuseClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextReused => "context_reused",
            Self::ContextFresh => "context_fresh",
            Self::ContextReuseUnavailable => "context_reuse_unavailable",
            Self::ContextReuseUnknown => "context_reuse_unknown",
        }
    }

    /// Whether reuse could not be honored or determined and must show a banner.
    pub const fn requires_banner(self) -> bool {
        matches!(
            self,
            Self::ContextReuseUnavailable | Self::ContextReuseUnknown
        )
    }
}

/// Run-record freshness state for a generator run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorFreshnessClass {
    /// Verified fresh against the last run record.
    Fresh,
    /// A newer run is available but the current record is still serviceable.
    RescanAvailable,
    /// Aging; a re-run is recommended.
    Aging,
    /// Stale; the run record is past its freshness window.
    Stale,
    /// Freshness could not be determined.
    FreshnessUnknown,
}

impl GeneratorFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::RescanAvailable => "rescan_available",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// Whether this freshness state blocks presenting the run as current.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Stale | Self::FreshnessUnknown)
    }
}

/// Support class on which a generator run may be offered — keeps bridge/heuristic honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorSupportClass {
    /// Exactly modeled first-party generator.
    ExactlyModeled,
    /// Experimental; may change without notice.
    Experimental,
    /// Bridge behavior: bridged from another generator tool rather than modeled natively.
    BridgeBehavior,
    /// Heuristic mapping; inferred rather than exactly modeled.
    HeuristicMapping,
    /// Explicitly unsupported.
    Unsupported,
    /// Support class unknown.
    SupportUnknown,
}

impl GeneratorSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactlyModeled => "exactly_modeled",
            Self::Experimental => "experimental",
            Self::BridgeBehavior => "bridge_behavior",
            Self::HeuristicMapping => "heuristic_mapping",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }

    /// Whether this class is bridge or heuristic behavior that must be disclosed.
    ///
    /// Bridge and heuristic generators must never be presented as exact first-party
    /// truth without a known issue, a support-class banner, and the matching
    /// disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BridgeBehavior | Self::HeuristicMapping)
    }
}

/// Downgrade banner shown for a generator run — the explicit narrowing cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorDowngradeBannerClass {
    /// No downgrade banner is shown.
    NoBanner,
    /// Freshness banner: the run record is aging, stale, or unverifiable.
    FreshnessBanner,
    /// Preview banner: no preview could be produced.
    PreviewUnavailableBanner,
    /// Diff banner: no change diff could be produced.
    DiffUnavailableBanner,
    /// Rollback banner: no rollback handle is available.
    RollbackUnavailableBanner,
    /// Support-class banner: bridge or heuristic behavior is disclosed.
    SupportClassBanner,
    /// Context-reuse banner: a warm execution context could not be reused.
    ContextReuseBanner,
    /// Policy-block banner: the run is blocked by policy or trust.
    PolicyBlockBanner,
}

impl GeneratorDowngradeBannerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBanner => "no_banner",
            Self::FreshnessBanner => "freshness_banner",
            Self::PreviewUnavailableBanner => "preview_unavailable_banner",
            Self::DiffUnavailableBanner => "diff_unavailable_banner",
            Self::RollbackUnavailableBanner => "rollback_unavailable_banner",
            Self::SupportClassBanner => "support_class_banner",
            Self::ContextReuseBanner => "context_reuse_banner",
            Self::PolicyBlockBanner => "policy_block_banner",
        }
    }

    /// Whether a banner is shown at all.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoBanner)
    }

    /// Whether this banner hard-blocks confident display (not merely a soft cue).
    pub const fn is_hard_block(self) -> bool {
        matches!(
            self,
            Self::PreviewUnavailableBanner
                | Self::DiffUnavailableBanner
                | Self::RollbackUnavailableBanner
                | Self::PolicyBlockBanner
        )
    }
}

/// Downgrade trigger that can narrow a generator run below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// No preview could be produced.
    PreviewUnavailable,
    /// No change diff could be produced.
    DiffUnavailable,
    /// No rollback handle is available.
    RollbackUnavailable,
    /// A warm execution context could not be reused.
    ContextReuseUnavailable,
    /// The pinned generator version could not be verified.
    GeneratorVersionUnverified,
    /// The run record that produced the row went stale.
    RunRecordStale,
    /// Heuristic mapping is disclosed and held from exact-truth claims.
    HeuristicMappingDisclosed,
    /// Bridge behavior is disclosed and held from exact-truth claims.
    BridgeBehaviorDisclosed,
    /// A blocking known issue applies.
    KnownIssueBlocking,
    /// A validation bundle failed.
    ValidationFailed,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl GeneratorDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::PreviewUnavailable,
        Self::DiffUnavailable,
        Self::RollbackUnavailable,
        Self::ContextReuseUnavailable,
        Self::GeneratorVersionUnverified,
        Self::RunRecordStale,
        Self::HeuristicMappingDisclosed,
        Self::BridgeBehaviorDisclosed,
        Self::KnownIssueBlocking,
        Self::ValidationFailed,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::PreviewUnavailable => "preview_unavailable",
            Self::DiffUnavailable => "diff_unavailable",
            Self::RollbackUnavailable => "rollback_unavailable",
            Self::ContextReuseUnavailable => "context_reuse_unavailable",
            Self::GeneratorVersionUnverified => "generator_version_unverified",
            Self::RunRecordStale => "run_record_stale",
            Self::HeuristicMappingDisclosed => "heuristic_mapping_disclosed",
            Self::BridgeBehaviorDisclosed => "bridge_behavior_disclosed",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::ValidationFailed => "validation_failed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a generator run's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorConsumerSurface {
    /// Generator / codemod gallery.
    GeneratorGallery,
    /// Preview pane.
    PreviewPane,
    /// Generation diff-review surface.
    DiffReview,
    /// Scaffold or generator run surface.
    RunSurface,
    /// Rollback / recovery surface.
    RollbackRecovery,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl GeneratorConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::GeneratorGallery,
        Self::PreviewPane,
        Self::DiffReview,
        Self::RunSurface,
        Self::RollbackRecovery,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratorGallery => "generator_gallery",
            Self::PreviewPane => "preview_pane",
            Self::DiffReview => "diff_review",
            Self::RunSurface => "run_surface",
            Self::RollbackRecovery => "rollback_recovery",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One generator-run row: a generator or codemod run and its preview, diff,
/// rollback, execution-context-reuse, and banner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunRow {
    /// Opaque stable row id.
    pub row_id: String,
    /// Which kind of generator or codemod produced this run.
    pub generator_kind: GeneratorKind,
    /// Opaque stable generator id.
    pub generator_id: String,
    /// Display label for the generator.
    pub generator_label: String,
    /// Structural locator for the generator definition.
    pub generator_locator: String,
    /// Pinned generator version — provenance always disclosed.
    pub generator_version: String,
    /// Opaque stable app / project id.
    pub app_id: String,
    /// Opaque framework-pack ref this generator belongs to; a sentinel otherwise.
    pub framework_pack_ref: String,
    /// Short reviewable summary of what the run changes.
    pub target_summary: String,
    /// Whether a preview of the change is available.
    pub preview_class: PreviewClass,
    /// Short reviewable preview summary.
    pub preview_summary: String,
    /// Diff-review state for the change.
    pub diff_review_class: DiffReviewClass,
    /// Short reviewable diff summary.
    pub diff_summary: String,
    /// Export-safe diff-stat chip label.
    pub diff_stat_label: String,
    /// Whether and how the run can be rolled back.
    pub rollback_class: RollbackClass,
    /// Short reviewable rollback summary.
    pub rollback_summary: String,
    /// Opaque rollback-handle refs that ground the recovery action.
    pub rollback_handle_refs: Vec<String>,
    /// Whether a warm execution context was reused.
    pub context_reuse_class: ExecutionContextReuseClass,
    /// Short reviewable execution-context-reuse summary.
    pub context_reuse_summary: String,
    /// Opaque execution-context refs the run reused or created.
    pub execution_context_refs: Vec<String>,
    /// Run-record freshness state.
    pub freshness_class: GeneratorFreshnessClass,
    /// Export-safe freshness/run chip label.
    pub freshness_chip_label: String,
    /// RFC 3339 timestamp the generator last ran.
    pub last_run: String,
    /// Support class on which the run may be offered.
    pub support_class: GeneratorSupportClass,
    /// Downgrade banner shown for this run.
    pub downgrade_banner_class: GeneratorDowngradeBannerClass,
    /// Opaque known-issue refs disclosed before the run is offered.
    pub known_issue_refs: Vec<String>,
    /// Whether this run is admitted to be offered as confident active truth.
    pub admitted_for_display: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<GeneratorDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<GeneratorConsumerSurface>,
}

impl GeneratorRunRow {
    /// Whether this row is structurally blocked from confident display.
    pub const fn is_blocked(&self) -> bool {
        self.freshness_class.is_blocking()
            || self.preview_class.is_unavailable()
            || self.diff_review_class.is_unavailable()
            || self.rollback_class.is_unavailable()
            || self.downgrade_banner_class.is_hard_block()
    }
}

/// Review block asserting the lane's honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunReview {
    /// A pinned generator version is disclosed for every run.
    pub generator_version_disclosed_for_every_run: bool,
    /// A preview state is shown for every run.
    pub preview_state_shown_for_every_run: bool,
    /// A diff-review state is shown for every run.
    pub diff_review_state_shown_for_every_run: bool,
    /// A rollback state is shown for every run.
    pub rollback_state_shown_for_every_run: bool,
    /// An execution-context-reuse state is shown for every run.
    pub context_reuse_state_shown_for_every_run: bool,
    /// An unavailable preview blocks any confident apply.
    pub preview_unavailable_blocks_confident_apply: bool,
    /// An unavailable diff blocks any confident apply.
    pub diff_unavailable_blocks_confident_apply: bool,
    /// An unavailable rollback blocks any confident apply.
    pub rollback_unavailable_blocks_confident_apply: bool,
    /// A heuristic or bridged generator is never presented as exact truth.
    pub heuristic_or_bridge_never_presented_as_exact_truth: bool,
    /// A failed context reuse is labeled rather than silently hidden.
    pub context_reuse_failure_labeled_not_hidden: bool,
    /// A stale run record is never presented as current.
    pub stale_run_record_not_presented_as_current: bool,
    /// The support class is visible before a run is offered.
    pub support_class_visible_before_display: bool,
    /// Known issues are disclosed before a run is offered.
    pub known_issues_disclosed_before_display: bool,
    /// No raw source bodies, diffs, or URLs cross the export boundary.
    pub no_raw_source_bodies_or_urls_in_export: bool,
    /// Downgrade narrows the run's claim rather than hiding the run.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or unrunnable rows automatically block promotion.
    pub stale_or_unrunnable_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunConsumerProjection {
    /// Generator gallery shows the generator version.
    pub gallery_shows_generator_version: bool,
    /// Preview pane shows the preview state.
    pub preview_pane_shows_preview_state: bool,
    /// Diff-review shows the change diff.
    pub diff_review_shows_change_diff: bool,
    /// Run surface shows the rollback state.
    pub run_surface_shows_rollback_state: bool,
    /// Run surface shows the execution-context-reuse state.
    pub run_surface_shows_context_reuse_state: bool,
    /// Rollback / recovery surface shows the rollback handle.
    pub rollback_recovery_shows_rollback_handle: bool,
    /// CLI / headless shows generator rows.
    pub cli_headless_shows_generator_rows: bool,
    /// Support export shows generator rows.
    pub support_export_shows_generator_rows: bool,
    /// Diagnostics shows preview, diff, and rollback state.
    pub diagnostics_shows_preview_diff_rollback_state: bool,
    /// Blocked runs are visibly labeled rather than hidden.
    pub blocked_runs_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`GeneratorRunPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorRunRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when a preview of the change is currently available.
    pub preview_available: bool,
    /// True when the change diff is currently available.
    pub diff_available: bool,
    /// True when a rollback handle is currently available.
    pub rollback_available: bool,
    /// True when the warm execution context was reused for the run.
    pub context_reused: bool,
    /// True when the run record is currently fresh.
    pub run_fresh: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`GeneratorRunPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorRunPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Generator-run rows.
    pub rows: Vec<GeneratorRunRow>,
    /// Review block.
    pub review: GeneratorRunReview,
    /// Consumer projection block.
    pub consumer_projection: GeneratorRunConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GeneratorRunProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe generator-run packet with preview, diff, rollback, and
/// execution-context-reuse truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunPacket {
    /// Record kind; must equal [`GENERATOR_RUN_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GENERATOR_RUN_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Generator-run rows.
    pub rows: Vec<GeneratorRunRow>,
    /// Review block.
    pub review: GeneratorRunReview,
    /// Consumer projection block.
    pub consumer_projection: GeneratorRunConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GeneratorRunProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GeneratorRunPacket {
    /// Builds a generator-run packet from stable-row input.
    pub fn new(input: GeneratorRunPacketInput) -> Self {
        Self {
            record_kind: GENERATOR_RUN_RECORD_KIND.to_owned(),
            schema_version: GENERATOR_RUN_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            rows: input.rows,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows runs whose preview went unavailable, whose change diff could not be
    /// produced, whose rollback handle was lost, whose warm execution context could
    /// not be reused, whose run record went stale, or whose proof or upstream
    /// narrowed.
    ///
    /// An unavailable preview is the hardest block: the preview is marked
    /// unavailable, the diff is marked unavailable, the preview-unavailable banner is
    /// raised, and the run loses confident display. A lost rollback handle marks the
    /// rollback unavailable, raises the rollback-unavailable banner, and withdraws
    /// display. An unavailable diff marks the diff unavailable, raises the
    /// diff-unavailable banner, and withdraws display. A failed context reuse narrows
    /// the reuse state to unavailable and raises a context-reuse banner without
    /// withdrawing display, because falling back to a fresh context is honest, not a
    /// block. A stale run record narrows freshness to stale and raises a freshness
    /// banner. Stale proof or a narrowed upstream withholds display until evidence
    /// refreshes. A raised banner is never lowered. Rows without a matching
    /// observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[GeneratorRunRowObservation]) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.preview_available {
                row.preview_class = PreviewClass::PreviewUnavailable;
                row.diff_review_class = DiffReviewClass::DiffUnavailable;
                row.downgrade_banner_class =
                    GeneratorDowngradeBannerClass::PreviewUnavailableBanner;
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    GeneratorDowngradeTrigger::PreviewUnavailable,
                );
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    GeneratorDowngradeTrigger::DiffUnavailable,
                );
                continue;
            }

            if !observation.rollback_available {
                row.rollback_class = RollbackClass::RollbackUnavailable;
                raise_banner(
                    row,
                    GeneratorDowngradeBannerClass::RollbackUnavailableBanner,
                );
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    GeneratorDowngradeTrigger::RollbackUnavailable,
                );
            }

            if !observation.diff_available && !row.diff_review_class.is_unavailable() {
                row.diff_review_class = DiffReviewClass::DiffUnavailable;
                raise_banner(row, GeneratorDowngradeBannerClass::DiffUnavailableBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    GeneratorDowngradeTrigger::DiffUnavailable,
                );
            }

            if !observation.context_reused
                && row.context_reuse_class == ExecutionContextReuseClass::ContextReused
            {
                row.context_reuse_class = ExecutionContextReuseClass::ContextReuseUnavailable;
                raise_banner(row, GeneratorDowngradeBannerClass::ContextReuseBanner);
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    GeneratorDowngradeTrigger::ContextReuseUnavailable,
                );
            }

            if !observation.run_fresh {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = GeneratorFreshnessClass::Stale;
                }
                raise_banner(row, GeneratorDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    GeneratorDowngradeTrigger::RunRecordStale,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.admitted_for_display
            {
                row.admitted_for_display = false;
                let trigger = if observation.proof_fresh {
                    GeneratorDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    GeneratorDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the generator-run invariants.
    pub fn validate(&self) -> Vec<GeneratorRunViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GENERATOR_RUN_RECORD_KIND {
            violations.push(GeneratorRunViolation::WrongRecordKind);
        }
        if self.schema_version != GENERATOR_RUN_SCHEMA_VERSION {
            violations.push(GeneratorRunViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GeneratorRunViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("generator-run packet serializes"),
        ) {
            violations.push(GeneratorRunViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("generator-run packet serializes")
    }

    /// Rows currently admitted to be offered as confident active truth.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &GeneratorRunRow> {
        self.rows.iter().filter(|row| row.admitted_for_display)
    }

    /// Deterministic Markdown summary for diagnostics, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str(
            "# Framework Generators and Codemods with Preview, Diff, Rollback, and Execution-Context Reuse\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Rows: {} ({} admitted for display)\n",
            self.rows.len(),
            admitted
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** `{}` ({}) v{}: {}\n",
                row.generator_label,
                row.generator_locator,
                row.generator_kind.as_str(),
                row.generator_version,
                row.support_class.as_str()
            ));
            out.push_str(&format!("  - Target: {}\n", row.target_summary));
            out.push_str(&format!(
                "  - Preview: {} ({})\n",
                row.preview_summary,
                row.preview_class.as_str()
            ));
            out.push_str(&format!(
                "  - Diff: {} ({}) [{}]\n",
                row.diff_summary,
                row.diff_review_class.as_str(),
                row.diff_stat_label
            ));
            out.push_str(&format!(
                "  - Rollback: {} ({})\n",
                row.rollback_summary,
                row.rollback_class.as_str()
            ));
            out.push_str(&format!(
                "  - Context reuse: {} ({})\n",
                row.context_reuse_summary,
                row.context_reuse_class.as_str()
            ));
            out.push_str(&format!(
                "  - Freshness chip: {} ({})\n",
                row.freshness_chip_label,
                row.freshness_class.as_str()
            ));
            out.push_str(&format!(
                "  - Banner: {}\n",
                row.downgrade_banner_class.as_str()
            ));
            out.push_str(&format!("  - Offered: {}\n", row.admitted_for_display));
        }
        out
    }
}

/// Errors emitted when reading the checked-in generator-run export.
#[derive(Debug)]
pub enum GeneratorRunArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GeneratorRunViolation>),
}

impl fmt::Display for GeneratorRunArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "generator-run export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "generator-run export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GeneratorRunArtifactError {}

/// Validation failures emitted by [`GeneratorRunPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratorRunViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no rows.
    RowsEmpty,
    /// A row is incomplete.
    RowIncomplete,
    /// A preview-unavailable row is missing its preview-unavailable banner.
    PreviewUnavailableBannerMissing,
    /// A diff-unavailable row is missing a downgrade banner.
    DiffUnavailableBannerMissing,
    /// A rollback-unavailable row is missing its rollback-unavailable banner.
    RollbackUnavailableBannerMissing,
    /// A grounded rollback disclosure carries no rollback-handle refs.
    RollbackHandleRefsMissing,
    /// A context-reuse-unavailable or unknown row is missing a downgrade banner.
    ContextReuseBannerMissing,
    /// A bridge/heuristic row is missing a known issue, banner, or disclosure trigger.
    SupportClassUndisclosed,
    /// A stale or unknown-freshness row is missing a downgrade banner.
    FreshnessBannerMissing,
    /// A blocked row is still admitted for confident display.
    BlockedDisplayAdmitted,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl GeneratorRunViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::PreviewUnavailableBannerMissing => "preview_unavailable_banner_missing",
            Self::DiffUnavailableBannerMissing => "diff_unavailable_banner_missing",
            Self::RollbackUnavailableBannerMissing => "rollback_unavailable_banner_missing",
            Self::RollbackHandleRefsMissing => "rollback_handle_refs_missing",
            Self::ContextReuseBannerMissing => "context_reuse_banner_missing",
            Self::SupportClassUndisclosed => "support_class_undisclosed",
            Self::FreshnessBannerMissing => "freshness_banner_missing",
            Self::BlockedDisplayAdmitted => "blocked_display_admitted",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in generator-run export.
///
/// This is the first real consumer of the generator-run lane: a generator
/// gallery, preview-pane, diff-review, run, rollback/recovery, diagnostics, or
/// support-export surface calls it to ingest the canonical packet rather than
/// cloning status text.
///
/// # Errors
///
/// Returns [`GeneratorRunArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_generator_run_export() -> Result<GeneratorRunPacket, GeneratorRunArtifactError> {
    let packet: GeneratorRunPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/support_export.json"
    )))
    .map_err(GeneratorRunArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GeneratorRunArtifactError::Validation(violations))
    }
}

/// Canonical review block with every invariant satisfied.
pub fn canonical_review() -> GeneratorRunReview {
    GeneratorRunReview {
        generator_version_disclosed_for_every_run: true,
        preview_state_shown_for_every_run: true,
        diff_review_state_shown_for_every_run: true,
        rollback_state_shown_for_every_run: true,
        context_reuse_state_shown_for_every_run: true,
        preview_unavailable_blocks_confident_apply: true,
        diff_unavailable_blocks_confident_apply: true,
        rollback_unavailable_blocks_confident_apply: true,
        heuristic_or_bridge_never_presented_as_exact_truth: true,
        context_reuse_failure_labeled_not_hidden: true,
        stale_run_record_not_presented_as_current: true,
        support_class_visible_before_display: true,
        known_issues_disclosed_before_display: true,
        no_raw_source_bodies_or_urls_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_unrunnable_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting generator truth.
pub fn canonical_consumer_projection() -> GeneratorRunConsumerProjection {
    GeneratorRunConsumerProjection {
        gallery_shows_generator_version: true,
        preview_pane_shows_preview_state: true,
        diff_review_shows_change_diff: true,
        run_surface_shows_rollback_state: true,
        run_surface_shows_context_reuse_state: true,
        rollback_recovery_shows_rollback_handle: true,
        cli_headless_shows_generator_rows: true,
        support_export_shows_generator_rows: true,
        diagnostics_shows_preview_diff_rollback_state: true,
        blocked_runs_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every generator-run export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        GENERATOR_RUN_SCHEMA_REF.to_owned(),
        GENERATOR_RUN_DOC_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        FRAMEWORK_PACK_CONTRACT_REF.to_owned(),
        GENERATION_RECOVERY_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical generator-run packet from stable-row truth.
///
/// The rows mirror the checked-in support export and cover the preview, diff,
/// rollback, and execution-context-reuse spectrum: an exact scaffold generator
/// run with a reviewed diff, a captured rollback, and a reused warm context shown
/// active with no banner; an exact codemod run on a fresh context shown active; a
/// heuristic migration codemod held behind its support-class banner with a partial
/// preview, a pending diff, a partial rollback, and a context-reuse fallback; a
/// refactor generator whose rollback handle was lost and is blocked rather than
/// offered; a config generator whose preview could not be produced and is blocked;
/// and a codemod bridged from an external tool that was rolled back and held from
/// exact-truth claims.
pub fn canonical_generator_runs(
    packet_id: String,
    packet_label: String,
    minted_at: String,
    proof_freshness: GeneratorRunProofFreshness,
) -> GeneratorRunPacket {
    GeneratorRunPacket::new(GeneratorRunPacketInput {
        packet_id,
        packet_label,
        rows: canonical_rows(),
        review: canonical_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical rows that match the checked-in support export.
pub fn canonical_rows() -> Vec<GeneratorRunRow> {
    use GeneratorConsumerSurface as Surface;
    use GeneratorDowngradeTrigger as Trigger;

    vec![
        GeneratorRunRow {
            row_id: "generator-run-row:scaffold.resource.exact.reused:2026.06".to_owned(),
            generator_kind: GeneratorKind::ScaffoldGenerator,
            generator_id: "generator:rust.axum.resource_scaffold".to_owned(),
            generator_label: "Resource scaffold".to_owned(),
            generator_locator: "generator:framework_pack/rust.axum/resource".to_owned(),
            generator_version: "1.8.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            target_summary: "Generates a new resource module, route registration, and test stub under the managed zone".to_owned(),
            preview_class: PreviewClass::PreviewAvailable,
            preview_summary: "Every emitted and edited file is previewed before apply".to_owned(),
            diff_review_class: DiffReviewClass::DiffReviewed,
            diff_summary: "The full change diff was reviewed and approved before apply".to_owned(),
            diff_stat_label: "+3 files · +88 / −0".to_owned(),
            rollback_class: RollbackClass::RollbackAvailable,
            rollback_summary: "A rollback handle was captured; the run can be fully undone".to_owned(),
            rollback_handle_refs: vec![
                "rollback-handle:generation/scaffold.resource:2026.06".to_owned(),
            ],
            context_reuse_class: ExecutionContextReuseClass::ContextReused,
            context_reuse_summary: "A warm framework-pack execution context was reused for the run".to_owned(),
            execution_context_refs: vec!["exec-context:rust.axum.warm:2026.06".to_owned()],
            freshness_class: GeneratorFreshnessClass::Fresh,
            freshness_chip_label: "ran · fresh".to_owned(),
            last_run: "2026-06-08T00:00:00Z".to_owned(),
            support_class: GeneratorSupportClass::ExactlyModeled,
            downgrade_banner_class: GeneratorDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::PreviewUnavailable,
                Trigger::RollbackUnavailable,
                Trigger::RunRecordStale,
            ],
            consumer_surfaces: vec![
                Surface::GeneratorGallery,
                Surface::PreviewPane,
                Surface::DiffReview,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        GeneratorRunRow {
            row_id: "generator-run-row:codemod.add_field.exact.fresh:2026.06".to_owned(),
            generator_kind: GeneratorKind::Codemod,
            generator_id: "generator:rust.axum.add_model_field".to_owned(),
            generator_label: "Add model field".to_owned(),
            generator_locator: "generator:framework_pack/rust.axum/add_field".to_owned(),
            generator_version: "1.8.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            target_summary: "Rewrites the model struct and migration to add one field across two files".to_owned(),
            preview_class: PreviewClass::PreviewAvailable,
            preview_summary: "Both edited files are previewed before apply".to_owned(),
            diff_review_class: DiffReviewClass::DiffReviewed,
            diff_summary: "The change diff was reviewed and approved before apply".to_owned(),
            diff_stat_label: "2 files · +14 / −2".to_owned(),
            rollback_class: RollbackClass::RollbackAvailable,
            rollback_summary: "A rollback handle was captured for both edited files".to_owned(),
            rollback_handle_refs: vec![
                "rollback-handle:generation/codemod.add_field:2026.06".to_owned(),
            ],
            context_reuse_class: ExecutionContextReuseClass::ContextFresh,
            context_reuse_summary: "A fresh execution context was created; no warm context was reused".to_owned(),
            execution_context_refs: vec!["exec-context:rust.axum.fresh:2026.06".to_owned()],
            freshness_class: GeneratorFreshnessClass::Fresh,
            freshness_chip_label: "ran · fresh".to_owned(),
            last_run: "2026-06-08T00:00:00Z".to_owned(),
            support_class: GeneratorSupportClass::ExactlyModeled,
            downgrade_banner_class: GeneratorDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::PreviewUnavailable,
                Trigger::DiffUnavailable,
                Trigger::RollbackUnavailable,
            ],
            consumer_surfaces: vec![
                Surface::GeneratorGallery,
                Surface::PreviewPane,
                Surface::DiffReview,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        GeneratorRunRow {
            row_id: "generator-run-row:migration.heuristic.reuse_unavailable:2026.05".to_owned(),
            generator_kind: GeneratorKind::MigrationCodemod,
            generator_id: "generator:rust.axum.framework_migration".to_owned(),
            generator_label: "Framework migration (heuristic)".to_owned(),
            generator_locator: "generator:framework_pack/rust.axum/migrate".to_owned(),
            generator_version: "1.7.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            target_summary: "Migrates router and middleware usage to the new framework version; some rewrites are inferred".to_owned(),
            preview_class: PreviewClass::PreviewPartial,
            preview_summary: "Most edits are previewed, but a few inferred rewrites could not be fully previewed".to_owned(),
            diff_review_class: DiffReviewClass::DiffPending,
            diff_summary: "The change diff is computed and awaiting review before apply".to_owned(),
            diff_stat_label: "7 files · +120 / −86".to_owned(),
            rollback_class: RollbackClass::RollbackPartial,
            rollback_summary: "Only a partial rollback is possible; some manual edits would not be restored".to_owned(),
            rollback_handle_refs: vec![
                "rollback-handle:generation/migration.partial:2026.05".to_owned(),
            ],
            context_reuse_class: ExecutionContextReuseClass::ContextReuseUnavailable,
            context_reuse_summary: "Reuse was requested but the warm context was unavailable; the run fell back to a fresh context".to_owned(),
            execution_context_refs: vec!["exec-context:rust.axum.fresh:2026.05".to_owned()],
            freshness_class: GeneratorFreshnessClass::Aging,
            freshness_chip_label: "ran · aging".to_owned(),
            last_run: "2026-05-20T00:00:00Z".to_owned(),
            support_class: GeneratorSupportClass::HeuristicMapping,
            downgrade_banner_class: GeneratorDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:framework_generators:heuristic_migration_rewrites".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HeuristicMappingDisclosed,
                Trigger::ContextReuseUnavailable,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::GeneratorGallery,
                Surface::PreviewPane,
                Surface::DiffReview,
                Surface::RunSurface,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        GeneratorRunRow {
            row_id: "generator-run-row:refactor.rollback_unavailable.blocked:2026.06".to_owned(),
            generator_kind: GeneratorKind::RefactorGenerator,
            generator_id: "generator:rust.axum.extract_service".to_owned(),
            generator_label: "Extract service".to_owned(),
            generator_locator: "generator:framework_pack/rust.axum/extract_service".to_owned(),
            generator_version: "1.8.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            target_summary: "Extracts a service module from a handler, but no rollback handle could be captured for the move".to_owned(),
            preview_class: PreviewClass::PreviewAvailable,
            preview_summary: "The change was previewed, but it cannot be safely offered without a rollback handle".to_owned(),
            diff_review_class: DiffReviewClass::DiffReviewed,
            diff_summary: "The change diff was computed, but the run is blocked from confident apply".to_owned(),
            diff_stat_label: "4 files · +60 / −40".to_owned(),
            rollback_class: RollbackClass::RollbackUnavailable,
            rollback_summary: "No rollback handle could be captured; the run is blocked rather than offered".to_owned(),
            rollback_handle_refs: vec![],
            context_reuse_class: ExecutionContextReuseClass::ContextFresh,
            context_reuse_summary: "A fresh execution context was created for the run".to_owned(),
            execution_context_refs: vec!["exec-context:rust.axum.fresh:2026.06".to_owned()],
            freshness_class: GeneratorFreshnessClass::Fresh,
            freshness_chip_label: "ran · fresh".to_owned(),
            last_run: "2026-06-08T00:00:00Z".to_owned(),
            support_class: GeneratorSupportClass::SupportUnknown,
            downgrade_banner_class: GeneratorDowngradeBannerClass::RollbackUnavailableBanner,
            known_issue_refs: vec![
                "known-issue:framework_generators:rollback_handle_unavailable".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::RollbackUnavailable,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::PreviewPane,
                Surface::DiffReview,
                Surface::RollbackRecovery,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        GeneratorRunRow {
            row_id: "generator-run-row:config.preview_unavailable.blocked:2026.04".to_owned(),
            generator_kind: GeneratorKind::ConfigGenerator,
            generator_id: "generator:rust.axum.config_scaffold".to_owned(),
            generator_label: "Config scaffold".to_owned(),
            generator_locator: "generator:framework_pack/rust.axum/config".to_owned(),
            generator_version: "1.8.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            target_summary: "Generates framework configuration, but the preview could not be produced for the target environment".to_owned(),
            preview_class: PreviewClass::PreviewUnavailable,
            preview_summary: "No preview could be produced; the run is blocked rather than applied blind".to_owned(),
            diff_review_class: DiffReviewClass::DiffUnavailable,
            diff_summary: "No change diff could be produced without a preview".to_owned(),
            diff_stat_label: "diff unavailable".to_owned(),
            rollback_class: RollbackClass::RollbackAvailable,
            rollback_summary: "A rollback handle would be captured, but the run is blocked before apply".to_owned(),
            rollback_handle_refs: vec![
                "rollback-handle:generation/config.scaffold:2026.04".to_owned(),
            ],
            context_reuse_class: ExecutionContextReuseClass::ContextReuseUnknown,
            context_reuse_summary: "The execution-context-reuse state could not be determined for the blocked run".to_owned(),
            execution_context_refs: vec![],
            freshness_class: GeneratorFreshnessClass::Fresh,
            freshness_chip_label: "ran · fresh".to_owned(),
            last_run: "2026-04-10T00:00:00Z".to_owned(),
            support_class: GeneratorSupportClass::SupportUnknown,
            downgrade_banner_class: GeneratorDowngradeBannerClass::PreviewUnavailableBanner,
            known_issue_refs: vec![
                "known-issue:framework_generators:preview_unavailable".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::PreviewUnavailable,
                Trigger::DiffUnavailable,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::PreviewPane,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        GeneratorRunRow {
            row_id: "generator-run-row:codemod.bridge.rolled_back:2026.06".to_owned(),
            generator_kind: GeneratorKind::Codemod,
            generator_id: "generator:rust.axum.lint_fix_bridge".to_owned(),
            generator_label: "Lint fix codemod (bridged)".to_owned(),
            generator_locator: "generator:framework_pack/rust.axum/lint_bridge".to_owned(),
            generator_version: "0.4.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            target_summary: "Applies fixes bridged from an external codemod tool; the run was rolled back after review".to_owned(),
            preview_class: PreviewClass::PreviewAvailable,
            preview_summary: "The bridged change was previewed before apply".to_owned(),
            diff_review_class: DiffReviewClass::DiffReviewed,
            diff_summary: "The change diff was reviewed, then the run was rolled back".to_owned(),
            diff_stat_label: "5 files · +18 / −22".to_owned(),
            rollback_class: RollbackClass::RolledBack,
            rollback_summary: "The run was rolled back from its captured handle; the working tree was restored".to_owned(),
            rollback_handle_refs: vec![
                "rollback-handle:generation/codemod.bridge:2026.06".to_owned(),
            ],
            context_reuse_class: ExecutionContextReuseClass::ContextFresh,
            context_reuse_summary: "A fresh execution context was created for the bridged run".to_owned(),
            execution_context_refs: vec!["exec-context:rust.axum.fresh:2026.06".to_owned()],
            freshness_class: GeneratorFreshnessClass::RescanAvailable,
            freshness_chip_label: "ran · rescan available".to_owned(),
            last_run: "2026-06-06T00:00:00Z".to_owned(),
            support_class: GeneratorSupportClass::BridgeBehavior,
            downgrade_banner_class: GeneratorDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:framework_generators:external_codemod_bridge".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::BridgeBehaviorDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::GeneratorGallery,
                Surface::DiffReview,
                Surface::RollbackRecovery,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &GeneratorRunPacket,
    violations: &mut Vec<GeneratorRunViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GENERATOR_RUN_SCHEMA_REF,
        GENERATOR_RUN_DOC_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        FRAMEWORK_PACK_CONTRACT_REF,
        GENERATION_RECOVERY_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GeneratorRunViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(packet: &GeneratorRunPacket, violations: &mut Vec<GeneratorRunViolation>) {
    if packet.rows.is_empty() {
        violations.push(GeneratorRunViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.generator_id.trim().is_empty()
            || row.generator_label.trim().is_empty()
            || row.generator_locator.trim().is_empty()
            || row.generator_version.trim().is_empty()
            || row.app_id.trim().is_empty()
            || row.framework_pack_ref.trim().is_empty()
            || row.target_summary.trim().is_empty()
            || row.preview_summary.trim().is_empty()
            || row.diff_summary.trim().is_empty()
            || row.diff_stat_label.trim().is_empty()
            || row.rollback_summary.trim().is_empty()
            || row.context_reuse_summary.trim().is_empty()
            || row.freshness_chip_label.trim().is_empty()
            || row.last_run.trim().is_empty()
        {
            violations.push(GeneratorRunViolation::RowIncomplete);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(GeneratorRunViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(GeneratorRunViolation::ConsumerSurfacesMissing);
        }

        validate_row_banners(row, violations);
    }
}

fn validate_row_banners(row: &GeneratorRunRow, violations: &mut Vec<GeneratorRunViolation>) {
    // An unavailable preview must raise the preview-unavailable banner, so a run is
    // never applied blind.
    if row.preview_class.is_unavailable()
        && row.downgrade_banner_class != GeneratorDowngradeBannerClass::PreviewUnavailableBanner
    {
        violations.push(GeneratorRunViolation::PreviewUnavailableBannerMissing);
    }

    // An unavailable diff must show a downgrade banner. A preview-unavailable run
    // shows the preview banner, which also covers its unavailable diff.
    if row.diff_review_class.is_unavailable() && !row.downgrade_banner_class.is_present() {
        violations.push(GeneratorRunViolation::DiffUnavailableBannerMissing);
    }

    // An unavailable rollback must raise the rollback-unavailable banner, so a run is
    // never applied without a way back.
    if row.rollback_class.is_unavailable()
        && row.downgrade_banner_class != GeneratorDowngradeBannerClass::RollbackUnavailableBanner
    {
        violations.push(GeneratorRunViolation::RollbackUnavailableBannerMissing);
    }

    // A captured or partial rollback must carry at least one rollback-handle ref.
    if matches!(
        row.rollback_class,
        RollbackClass::RollbackAvailable
            | RollbackClass::RollbackPartial
            | RollbackClass::RolledBack
    ) && row.rollback_handle_refs.is_empty()
    {
        violations.push(GeneratorRunViolation::RollbackHandleRefsMissing);
    }

    // A context reuse that could not be honored or determined must show a banner.
    if row.context_reuse_class.requires_banner() && !row.downgrade_banner_class.is_present() {
        violations.push(GeneratorRunViolation::ContextReuseBannerMissing);
    }

    // Bridge/heuristic runs must disclose a known issue, a banner, and the matching trigger.
    if row.support_class.requires_disclosure() {
        let matching_trigger = match row.support_class {
            GeneratorSupportClass::BridgeBehavior => {
                GeneratorDowngradeTrigger::BridgeBehaviorDisclosed
            }
            _ => GeneratorDowngradeTrigger::HeuristicMappingDisclosed,
        };
        if row.known_issue_refs.is_empty()
            || !row.downgrade_banner_class.is_present()
            || !row.downgrade_triggers.contains(&matching_trigger)
        {
            violations.push(GeneratorRunViolation::SupportClassUndisclosed);
        }
    }

    // A stale or unknown-freshness run must show a downgrade banner.
    if row.freshness_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(GeneratorRunViolation::FreshnessBannerMissing);
    }

    // A blocked run cannot be admitted for confident display.
    if row.is_blocked() && row.admitted_for_display {
        violations.push(GeneratorRunViolation::BlockedDisplayAdmitted);
    }
}

fn validate_review(packet: &GeneratorRunPacket, violations: &mut Vec<GeneratorRunViolation>) {
    let review = &packet.review;
    for ok in [
        review.generator_version_disclosed_for_every_run,
        review.preview_state_shown_for_every_run,
        review.diff_review_state_shown_for_every_run,
        review.rollback_state_shown_for_every_run,
        review.context_reuse_state_shown_for_every_run,
        review.preview_unavailable_blocks_confident_apply,
        review.diff_unavailable_blocks_confident_apply,
        review.rollback_unavailable_blocks_confident_apply,
        review.heuristic_or_bridge_never_presented_as_exact_truth,
        review.context_reuse_failure_labeled_not_hidden,
        review.stale_run_record_not_presented_as_current,
        review.support_class_visible_before_display,
        review.known_issues_disclosed_before_display,
        review.no_raw_source_bodies_or_urls_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_unrunnable_blocks_promotion,
    ] {
        if !ok {
            violations.push(GeneratorRunViolation::ReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &GeneratorRunPacket,
    violations: &mut Vec<GeneratorRunViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_generator_version,
        projection.preview_pane_shows_preview_state,
        projection.diff_review_shows_change_diff,
        projection.run_surface_shows_rollback_state,
        projection.run_surface_shows_context_reuse_state,
        projection.rollback_recovery_shows_rollback_handle,
        projection.cli_headless_shows_generator_rows,
        projection.support_export_shows_generator_rows,
        projection.diagnostics_shows_preview_diff_rollback_state,
        projection.blocked_runs_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(GeneratorRunViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &GeneratorRunPacket,
    violations: &mut Vec<GeneratorRunViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(GeneratorRunViolation::ProofFreshnessIncomplete);
    }
}

/// Raises the row's downgrade banner only when none is currently shown, so an
/// already-raised banner is never lowered to a softer cue.
fn raise_banner(row: &mut GeneratorRunRow, banner: GeneratorDowngradeBannerClass) {
    if !row.downgrade_banner_class.is_present() {
        row.downgrade_banner_class = banner;
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<GeneratorDowngradeTrigger>,
    trigger: GeneratorDowngradeTrigger,
) {
    if !triggers.contains(&trigger) {
        triggers.push(trigger);
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
