//! Workflow-bundle review packets for install, update, drift, and rollback.
//!
//! A [`WorkflowBundleReviewRecord`] is the workspace-owned beta packet that
//! composes bundle detail, install/update preview, drift/override review,
//! remove/rollback review, certification truth, mirror/offline posture, CLI
//! parity, diagnostics, and support export into one validated artifact.
//!
//! The packet does not execute bundle changes. It is the review and projection
//! boundary every mutating engine must pass through before durable state is
//! installed, updated, rebased, removed, or rolled back.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for beta workflow-bundle review records.
pub const WORKFLOW_BUNDLE_REVIEW_BETA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for [`WorkflowBundleReviewRecord`].
pub const WORKFLOW_BUNDLE_REVIEW_BETA_RECORD_KIND: &str = "workflow_bundle_review_beta_record";

/// Closed set of bundle classes the review packet projects.
pub const WORKFLOW_BUNDLE_CLASSES: &[&str] = &[
    "launch_bundle",
    "imported_user_bundle",
    "org_approved_bundle",
    "design_partner_bundle",
    "local_draft_bundle",
];

/// Closed set of source classes kept distinct across review surfaces.
pub const WORKFLOW_BUNDLE_SOURCE_CLASSES: &[&str] = &[
    "certified",
    "managed_approved",
    "community",
    "imported",
    "local_draft",
];

/// Closed set of status classes mirrored from workflow-bundle manifests.
pub const WORKFLOW_BUNDLE_STATUS_CLASSES: &[&str] = &[
    "certified_current",
    "certified_retest_pending",
    "managed_approved_current",
    "community_reviewed",
    "community_unreviewed",
    "imported_pending_review",
    "local_draft",
    "deprecated_or_archived",
    "retest_needed",
    "status_unknown",
];

/// Closed set of support classes projected into diagnostics and exports.
pub const WORKFLOW_BUNDLE_SUPPORT_CLASSES: &[&str] = &[
    "officially_supported",
    "community_supported",
    "experimental",
    "legacy_deprecated",
    "unsupported",
    "support_unknown",
];

/// Closed set of effective badge classes after evidence freshness is applied.
pub const WORKFLOW_BUNDLE_EFFECTIVE_BADGE_CLASSES: &[&str] = &[
    "certified",
    "managed_approved",
    "community",
    "imported",
    "local_draft",
    "retest_pending",
    "limited",
    "status_unknown",
];

/// Closed set of evidence freshness classes used by certification review.
pub const WORKFLOW_BUNDLE_EVIDENCE_FRESHNESS_CLASSES: &[&str] = &[
    "fresh_current",
    "aging_within_window",
    "stale_past_window",
    "imported_evidence",
    "evidence_unknown",
];

/// Closed set of certification state classes used by review packets.
pub const WORKFLOW_BUNDLE_CERTIFICATION_STATE_CLASSES: &[&str] = &[
    "certified_current",
    "managed_approved_current",
    "community_unverified",
    "imported_pending_review",
    "local_draft",
    "retest_pending",
    "evidence_stale",
    "status_unknown",
];

/// Closed set of support claim classes used by badge downgrade logic.
pub const WORKFLOW_BUNDLE_SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_launch_wedge_claim",
    "managed_org_claim",
    "community_no_certification_claim",
    "imported_pending_review_claim",
    "local_draft_no_claim",
    "limited_retest_pending_claim",
];

/// Required diff axes for install and update review packets.
pub const WORKFLOW_BUNDLE_REQUIRED_DIFF_AXES: &[&str] = &[
    "extension_set",
    "profile_preset",
    "surface_preset",
    "settings_or_token",
    "task_recipe",
    "launch_recipe",
    "debug_recipe",
    "docs_pack",
    "tour_pack",
    "template_or_scaffold_ref",
    "migration_mapping",
    "certification_target",
];

/// Closed set of install/update diff axes.
pub const WORKFLOW_BUNDLE_DIFF_AXES: &[&str] = &[
    "extension_set",
    "profile_preset",
    "surface_preset",
    "settings_or_token",
    "task_recipe",
    "launch_recipe",
    "debug_recipe",
    "template_or_scaffold_ref",
    "docs_pack",
    "tour_pack",
    "glossary_pack",
    "migration_mapping",
    "certification_target",
    "evidence_link",
    "trust_or_permission",
    "compatibility_or_runtime",
    "side_effect",
];

/// Closed set of change kinds in install/update review.
pub const WORKFLOW_BUNDLE_CHANGE_KINDS: &[&str] = &[
    "added",
    "removed",
    "changed",
    "revision_bumped",
    "unchanged_visible",
    "preserved_local",
    "blocked_pending_review",
    "skipped_no_op",
];

/// Closed set of granular drift subject classes.
pub const WORKFLOW_BUNDLE_DRIFT_SUBJECT_GRANULARITY_CLASSES: &[&str] = &[
    "field",
    "package",
    "task",
    "component",
    "certification_evidence",
    "mirror_or_offline_pack",
];

/// Closed set of asset ownership classes used by drift and removal review.
pub const WORKFLOW_BUNDLE_ASSET_OWNERSHIP_CLASSES: &[&str] = &[
    "bundle_owned",
    "user_owned",
    "shared_user_overlay_on_bundle",
    "mixed_unknown_provenance",
];

/// Closed set of safe-to-remove classes used by removal review.
pub const WORKFLOW_BUNDLE_SAFE_TO_REMOVE_CLASSES: &[&str] = &[
    "safe_to_remove_no_user_data",
    "safe_to_remove_user_overlay_preserved",
    "review_required_user_data_co_resident",
    "not_safe_to_remove_user_owned",
    "not_safe_to_remove_policy_locked",
];

/// Closed set of review action ids used before durable mutation.
pub const WORKFLOW_BUNDLE_REVIEW_ACTION_IDS: &[&str] = &[
    "review.compare",
    "review.confirm",
    "review.cancel",
    "review.set_up_later",
    "review.inspect_change_source",
    "review.create_rollback_checkpoint",
];

/// Closed set of drift resolve actions.
pub const WORKFLOW_BUNDLE_RESOLVE_ACTION_IDS: &[&str] = &[
    "resolve.keep_local",
    "resolve.adopt_bundle",
    "resolve.compare",
    "resolve.rebase_to_bundle",
    "resolve.ignore_this_drift",
];

/// Closed set of action rendering states.
pub const WORKFLOW_BUNDLE_ACTION_RENDERED_STATES: &[&str] = &[
    "enabled",
    "visible_disabled",
    "hidden_not_applicable",
    "preflight_pending",
];

/// Closed set of rollback linkage classes.
pub const WORKFLOW_BUNDLE_ROLLBACK_LINKAGE_CLASSES: &[&str] = &[
    "single_attributable_workspace_checkpoint",
    "single_attributable_profile_checkpoint",
    "single_attributable_user_checkpoint",
    "single_appearance_checkpoint_only_for_visual",
    "paired_workspace_and_appearance_checkpoint",
    "non_reversible_with_justification",
    "non_reversible_pending_review",
];

/// Closed set of rollback path classes.
pub const WORKFLOW_BUNDLE_ROLLBACK_PATH_CLASSES: &[&str] = &[
    "single_checkpoint_revert",
    "surface_reload_then_revert",
    "full_restart_then_revert",
    "manual_recovery_only",
    "not_reversible",
];

/// Closed set of mirror/offline packaging posture classes.
pub const WORKFLOW_BUNDLE_MIRROR_OFFLINE_POSTURE_CLASSES: &[&str] = &[
    "live_origin_only",
    "live_or_mirror",
    "mirror_only",
    "signed_offline_bundle",
    "offline_no_bundle",
    "packaging_posture_unknown",
];

/// Closed set of consumer surfaces that must read the same review packet.
pub const WORKFLOW_BUNDLE_CONSUMER_SURFACES: &[&str] = &[
    "start_center",
    "bundle_detail",
    "cli_headless",
    "diagnostics",
    "support_export",
    "docs_workspace",
];

/// Closed set of redaction classes allowed on support exports.
pub const WORKFLOW_BUNDLE_REDACTION_CLASSES: &[&str] = &[
    "metadata_safe_default",
    "operator_only_restricted",
    "internal_support_restricted",
    "signing_evidence_only",
];

/// Review packet for one workflow-bundle install/update/drift/remove flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleReviewRecord {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record shape.
    pub schema_version: u32,
    /// Stable review packet id.
    pub review_id: String,
    /// Monotonic timestamp when the review packet was minted.
    pub minted_at: String,
    /// Bundle identity and source truth.
    pub bundle_identity: WorkflowBundleIdentity,
    /// Distinct source-class labels every consumer must preserve.
    pub source_class_legend: Vec<WorkflowBundleSourceClassDisclosure>,
    /// Exact contents exposed by the bundle detail page and headless summary.
    pub detail: WorkflowBundleDetail,
    /// Certification and support-claim truth after evidence freshness is applied.
    pub certification: WorkflowBundleCertificationReview,
    /// Install/update diff and rollback checkpoint review.
    pub install_update_review: WorkflowBundleInstallUpdateReview,
    /// Drift banner and local-override review.
    pub drift_override_review: WorkflowBundleDriftOverrideReview,
    /// Remove and rollback review.
    pub remove_rollback_review: WorkflowBundleRemoveRollbackReview,
    /// Mirror, signer, offline, and compatibility posture.
    pub mirror_offline: WorkflowBundleMirrorOfflineReview,
    /// Support, diagnostics, and headless export binding.
    pub support_export: WorkflowBundleSupportExport,
    /// Consumer surfaces bound to this exact packet.
    pub consumer_surfaces: Vec<String>,
    /// Guardrails that prevent hidden trust, egress, policy, or approval widening.
    pub guardrails: WorkflowBundleReviewGuardrails,
    /// Invariants every review packet must claim.
    pub review_invariants: WorkflowBundleReviewInvariants,
}

/// Stable identity for the target workflow bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleIdentity {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Integer bundle revision.
    pub bundle_revision: u32,
    /// Optional semantic version for the bundle revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_revision_semver: Option<String>,
    /// Product row class such as `launch_bundle` or `imported_user_bundle`.
    pub bundle_class: String,
    /// Source class such as `certified`, `imported`, or `local_draft`.
    pub bundle_source_class: String,
    /// Operational status at packet mint time.
    pub bundle_status_class: String,
    /// Support class shown in review and export surfaces.
    pub support_class: String,
    /// Signer source class from the bundle manifest.
    pub signer_source_class: String,
    /// Stable signer reference safe for support export.
    pub signer_ref: String,
    /// Manifest reference safe for review surfaces.
    pub manifest_ref: String,
    /// Manifest digest reference safe for support export.
    pub manifest_digest_ref: String,
    /// Compatible Aureline version range copied to every surface.
    pub compatible_aureline_range: String,
    /// Release channel for this bundle revision.
    pub channel: String,
    /// Imported source reference when the bundle came from a migration handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_source_ref: Option<String>,
}

/// Source-class label and caveat preserved by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleSourceClassDisclosure {
    /// Source class token.
    pub source_class: String,
    /// Display label paired with the token.
    pub display_label: String,
    /// Scope or caveat ref explaining the class.
    pub caveat_ref: String,
}

/// Exact bundle contents rendered before install or update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleDetail {
    /// Extension sets and package groups included by the bundle.
    pub extension_sets: Vec<WorkflowBundleContentItem>,
    /// Profile and surface presets included by the bundle.
    pub presets: Vec<WorkflowBundleContentItem>,
    /// Task, launch, and debug recipes included by the bundle.
    pub task_launch_debug_recipes: Vec<WorkflowBundleContentItem>,
    /// Docs, tour, and glossary packs included by the bundle.
    pub docs_tour_packs: Vec<WorkflowBundleContentItem>,
    /// Template or scaffold references included by the bundle.
    pub template_refs: Vec<WorkflowBundleContentItem>,
    /// Migration mappings included by the bundle.
    pub migration_mappings: Vec<WorkflowBundleContentItem>,
    /// Certification targets and evidence refs included by the bundle.
    pub certification_targets: Vec<WorkflowBundleContentItem>,
}

/// Support-safe item listed in bundle detail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleContentItem {
    /// Stable item reference.
    pub item_ref: String,
    /// Component or content class.
    pub item_class: String,
    /// Ownership class for removal and rollback review.
    pub ownership_class: String,
    /// Source class of the item when stricter than the bundle.
    pub source_class: String,
    /// Summary reference safe for UI, CLI, and support export.
    pub summary_ref: String,
    /// Revision reference for the item.
    pub revision_ref: String,
    /// Whether the item can be mirrored or exported for offline use.
    pub mirrorable: bool,
    /// Whether review surfaces must disclose the item.
    pub disclosure_required: bool,
}

/// Certification review after source, evidence, and compatibility checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleCertificationReview {
    /// Badge class claimed by the bundle source.
    pub source_badge_class: String,
    /// Freshness class of the linked evidence.
    pub evidence_freshness_class: String,
    /// Certification state after freshness and compatibility checks.
    pub certification_state_class: String,
    /// Effective badge class that surfaces may render.
    pub effective_badge_class: String,
    /// Claim class the badge is allowed to imply.
    pub support_claim_class: String,
    /// Reference workspace backing a strong claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_workspace_ref: Option<String>,
    /// Compatibility and evidence refs backing this review.
    pub compatibility_evidence_refs: Vec<String>,
    /// Whether a retest is required before a stronger claim may render.
    pub retest_required: bool,
}

/// Install/update review sheet and rollback checkpoint disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleInstallUpdateReview {
    /// Review state before mutation.
    pub review_state_class: String,
    /// Install, update, or rebase preview refs.
    pub preview_refs: Vec<String>,
    /// Diff entries shown before mutation.
    pub diff_entries: Vec<WorkflowBundleDiffEntry>,
    /// Review actions available on the sheet.
    pub actions: Vec<WorkflowBundleReviewAction>,
    /// Rollback checkpoint created or explained by the review.
    pub rollback_checkpoint: WorkflowBundleRollbackCheckpoint,
    /// Durable side effects disclosed before confirmation.
    pub side_effects: Vec<WorkflowBundleSideEffect>,
}

/// One support-safe diff row in an install/update review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleDiffEntry {
    /// Diff axis, such as `extension_set` or `settings_or_token`.
    pub change_axis: String,
    /// Change kind for this row.
    pub change_kind: String,
    /// Subject kind, such as extension package, setting field, or task.
    pub subject_kind: String,
    /// Stable subject reference.
    pub subject_ref: String,
    /// Ownership class relevant to rollback/removal.
    pub ownership_class: String,
    /// Optional prior revision or value-class reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_ref: Option<String>,
    /// Optional target revision or value-class reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_ref: Option<String>,
    /// Preserved override ref when local state wins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
    /// Destination preview when this row routes to a later apply/rebase review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes_through_preview_ref: Option<String>,
    /// Whether this row must be rendered on review surfaces.
    pub disclosure_required: bool,
    /// Whether the row can be reached by keyboard in UI surfaces.
    pub keyboard_reachable: bool,
}

/// Review action rendered before install, update, or removal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleReviewAction {
    /// Action id from the closed review action set.
    pub action_id: String,
    /// Rendering state for this action.
    pub rendered_state: String,
    /// Destination ref for compare, confirm, defer, inspect, or checkpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_ref: Option<String>,
    /// Disabled reason when the action is visible but unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_code: Option<String>,
    /// Whether the action can be reached by keyboard in UI surfaces.
    pub keyboard_reachable: bool,
}

/// Rollback checkpoint linkage shown before mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleRollbackCheckpoint {
    /// Linkage class for the rollback handle.
    pub linkage_class: String,
    /// Rollback path class shown to the reviewer.
    pub rollback_path_class: String,
    /// Checkpoint reference when the path is reversible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Axes that the checkpoint restores.
    pub restorable_axes: Vec<String>,
    /// Whether the checkpoint is attributable to this review packet.
    pub attributable_to_review: bool,
}

/// Side-effect envelope row disclosed before mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleSideEffect {
    /// Side-effect class.
    pub side_effect_class: String,
    /// Scope class affected by the side effect.
    pub scope_class: String,
    /// Summary ref safe for review and export.
    pub summary_ref: String,
    /// Whether rollback covers this side effect.
    pub reversible_in_rollback: bool,
}

/// Drift banner and local-override review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleDriftOverrideReview {
    /// High-level drift state shown on summary surfaces.
    pub drift_state_class: String,
    /// Field, package, task, component, evidence, or mirror drift rows.
    pub drift_entries: Vec<WorkflowBundleDriftEntry>,
    /// Whether the drift block uses field/package/task granularity.
    pub field_package_task_granular: bool,
}

/// One drift or local-override row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleDriftEntry {
    /// Drift state for this row.
    pub drift_state_class: String,
    /// Drift axis, mirroring install/update change axes.
    pub drift_axis: String,
    /// Granularity class such as field, package, or task.
    pub subject_granularity_class: String,
    /// Stable subject reference.
    pub subject_ref: String,
    /// Asset ownership class.
    pub asset_ownership_class: String,
    /// Claim narrowing caused by the drift row.
    pub claim_narrowing_class: String,
    /// Preserved local override ref when local state diverged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserved_local_override_ref: Option<String>,
    /// Resolve actions visible for this row.
    pub resolve_actions: Vec<WorkflowBundleResolveAction>,
}

/// Resolve action rendered on a drift row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleResolveAction {
    /// Resolve action id.
    pub action_id: String,
    /// Rendering state for this action.
    pub rendered_state: String,
    /// Destination ref for compare, keep-local, adopt, rebase, or ignore.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_ref: Option<String>,
    /// Disabled reason when the action is visible but unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_code: Option<String>,
    /// Whether the action can be reached by keyboard in UI surfaces.
    pub keyboard_reachable: bool,
}

/// Remove-bundle and rollback review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleRemoveRollbackReview {
    /// Remove review lifecycle state.
    pub review_state_class: String,
    /// Stable remove review ref.
    pub remove_review_ref: String,
    /// Rollback target ref.
    pub rollback_target_ref: String,
    /// Rollback checkpoint ref.
    pub rollback_checkpoint_ref: String,
    /// Assets considered by the remove review.
    pub removable_assets: Vec<WorkflowBundleRemovableAsset>,
    /// Local overrides or imported state retained by the review.
    pub retained_local_overrides: Vec<WorkflowBundleRetainedOverride>,
    /// Remove or rollback actions visible on the review.
    pub actions: Vec<WorkflowBundleReviewAction>,
}

/// One asset classified by remove-bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleRemovableAsset {
    /// Stable asset reference.
    pub asset_ref: String,
    /// Asset kind, such as extension set or scaffold output.
    pub asset_kind: String,
    /// Ownership class.
    pub ownership_class: String,
    /// Safe-to-remove class.
    pub safe_to_remove_class: String,
    /// Whether this asset requires extra review before deletion.
    pub review_required: bool,
    /// Whether this asset was explicitly shown in the remove review.
    pub explicit_reviewed: bool,
}

/// Local override or imported artifact retained through removal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleRetainedOverride {
    /// Stable override reference.
    pub override_ref: String,
    /// Retention class.
    pub retained_class: String,
    /// Scope where the retained record remains authoritative.
    pub target_scope_class: String,
}

/// Mirror/offline, signer, and compatibility posture block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleMirrorOfflineReview {
    /// Mirror/offline packaging posture.
    pub posture_class: String,
    /// Source registry or origin ref.
    pub source_registry_ref: String,
    /// Approved mirror ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_ref: Option<String>,
    /// Signed offline pack ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_pack_ref: Option<String>,
    /// Signer ref preserved in offline and mirror flows.
    pub signer_ref: String,
    /// Compatible Aureline range preserved across surfaces.
    pub compatible_aureline_range: String,
    /// Whether retest-needed state is preserved through offline review.
    pub retest_needed_preserved: bool,
    /// Offline restore review ref.
    pub offline_restore_review_ref: String,
}

/// Support export and diagnostics binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleSupportExport {
    /// Support export refs that reconstruct this packet.
    pub export_packet_refs: Vec<String>,
    /// Diagnostic refs that reconstruct this packet.
    pub diagnostics_refs: Vec<String>,
    /// CLI/headless refs that reconstruct this packet.
    pub cli_headless_refs: Vec<String>,
    /// Whether raw secrets may be exported.
    pub raw_secret_export_allowed: bool,
    /// Whether raw user-authored content may be exported.
    pub raw_user_content_export_allowed: bool,
    /// Whether raw local paths may be exported.
    pub raw_paths_export_allowed: bool,
    /// Redaction class for the export.
    pub redaction_class: String,
}

/// Guardrail block preventing silent trust or authority widening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleReviewGuardrails {
    /// Whether provider recommendations remain recommendations only.
    pub providers_recommended_only: bool,
    /// Whether remote-mode recommendations remain recommendations only.
    pub remote_modes_recommended_only: bool,
    /// Whether template recommendations remain recommendations only.
    pub templates_recommended_only: bool,
    /// Whether the bundle silently widens workspace trust.
    pub workspace_trust_widened: bool,
    /// Whether network egress widened without review.
    pub network_egress_widened_without_review: bool,
    /// Whether policy scope widened without review.
    pub policy_scope_widened_without_review: bool,
    /// Whether approval defaults widened without review.
    pub approval_defaults_widened_without_review: bool,
}

/// Review invariants claimed by every packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleReviewInvariants {
    /// Diff review happens before any mutation.
    pub diff_before_apply: bool,
    /// Rollback is reviewed before apply.
    pub rollback_reviewed: bool,
    /// Removal preserves user-owned assets unless separately reviewed.
    pub removal_preserves_user_assets: bool,
    /// CLI/headless output reads the same packet.
    pub cli_headless_parity: bool,
    /// Diagnostics and support export read the same packet.
    pub diagnostics_export_parity: bool,
    /// Mirror/offline posture is preserved by review surfaces.
    pub offline_mirror_truth_preserved: bool,
    /// Hidden imperative install hooks are disallowed.
    pub no_hidden_imperative_hooks: bool,
    /// Raw secret injection is disallowed.
    pub no_raw_secret_injection: bool,
}

/// Compact projection consumed by shell, CLI, diagnostics, and support export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowBundleReviewProjection {
    /// Stable review packet id.
    pub review_id: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Product row class.
    pub bundle_class: String,
    /// Bundle source class.
    pub bundle_source_class: String,
    /// Effective badge class after evidence checks.
    pub effective_badge_class: String,
    /// Required install/update axes present in the diff.
    pub required_diff_axes_present: Vec<String>,
    /// Required install/update axes missing from the diff.
    pub missing_required_diff_axes: Vec<String>,
    /// Number of granular drift rows.
    pub drift_entry_count: usize,
    /// Number of retained local overrides.
    pub retained_override_count: usize,
    /// Number of removable assets.
    pub removable_asset_count: usize,
    /// Review action ids visible before mutation.
    pub review_actions: Vec<String>,
    /// Resolve action ids visible on drift rows.
    pub resolve_actions: Vec<String>,
    /// Support export refs.
    pub support_export_refs: Vec<String>,
    /// Whether any raw export path is enabled.
    pub raw_export_allowed: bool,
    /// Whether guardrails pass.
    pub guardrails_pass: bool,
}

impl WorkflowBundleReviewRecord {
    /// Validates this review packet.
    ///
    /// # Errors
    ///
    /// Returns [`WorkflowBundleReviewValidationError`] when a review invariant,
    /// closed vocabulary, ownership rule, badge rule, or guardrail is violated.
    pub fn validate(&self) -> Result<(), WorkflowBundleReviewValidationError> {
        validate_record(self)
    }

    /// Projects this packet into the compact shared surface row.
    pub fn project(&self) -> WorkflowBundleReviewProjection {
        let required = WORKFLOW_BUNDLE_REQUIRED_DIFF_AXES
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let present_set = self
            .install_update_review
            .diff_entries
            .iter()
            .map(|entry| entry.change_axis.as_str())
            .filter(|axis| required.contains(axis))
            .collect::<BTreeSet<_>>();
        let required_diff_axes_present = present_set
            .iter()
            .map(|axis| (*axis).to_string())
            .collect::<Vec<_>>();
        let missing_required_diff_axes = required
            .difference(&present_set)
            .map(|axis| (*axis).to_string())
            .collect::<Vec<_>>();

        let review_actions = self
            .install_update_review
            .actions
            .iter()
            .chain(self.remove_rollback_review.actions.iter())
            .map(|action| action.action_id.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let resolve_actions = self
            .drift_override_review
            .drift_entries
            .iter()
            .flat_map(|entry| entry.resolve_actions.iter())
            .map(|action| action.action_id.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        WorkflowBundleReviewProjection {
            review_id: self.review_id.clone(),
            bundle_id: self.bundle_identity.bundle_id.clone(),
            bundle_class: self.bundle_identity.bundle_class.clone(),
            bundle_source_class: self.bundle_identity.bundle_source_class.clone(),
            effective_badge_class: self.certification.effective_badge_class.clone(),
            required_diff_axes_present,
            missing_required_diff_axes,
            drift_entry_count: self.drift_override_review.drift_entries.len(),
            retained_override_count: self.remove_rollback_review.retained_local_overrides.len(),
            removable_asset_count: self.remove_rollback_review.removable_assets.len(),
            review_actions,
            resolve_actions,
            support_export_refs: self.support_export.export_packet_refs.clone(),
            raw_export_allowed: self.support_export.raw_secret_export_allowed
                || self.support_export.raw_user_content_export_allowed
                || self.support_export.raw_paths_export_allowed,
            guardrails_pass: guardrails_pass(&self.guardrails),
        }
    }
}

/// Error returned when a workflow-bundle review payload fails validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowBundleReviewValidationError {
    message: String,
}

impl WorkflowBundleReviewValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for WorkflowBundleReviewValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "workflow bundle review validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for WorkflowBundleReviewValidationError {}

/// Error returned when a review payload cannot be parsed or projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowBundleReviewError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the beta review contract.
    Validation(WorkflowBundleReviewValidationError),
}

impl WorkflowBundleReviewError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for WorkflowBundleReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => {
                write!(formatter, "workflow bundle review JSON error: {message}")
            }
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for WorkflowBundleReviewError {}

/// Parses, validates, and projects a workflow-bundle review JSON payload.
///
/// # Errors
///
/// Returns [`WorkflowBundleReviewError`] if JSON parsing or validation fails.
pub fn project_workflow_bundle_review(
    payload: &str,
) -> Result<WorkflowBundleReviewProjection, WorkflowBundleReviewError> {
    let record: WorkflowBundleReviewRecord = serde_json::from_str(payload)
        .map_err(|err| WorkflowBundleReviewError::Json(err.to_string()))?;
    record
        .validate()
        .map_err(WorkflowBundleReviewError::Validation)?;
    Ok(record.project())
}

fn validate_record(
    record: &WorkflowBundleReviewRecord,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_equal(
        "record_kind",
        WORKFLOW_BUNDLE_REVIEW_BETA_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != WORKFLOW_BUNDLE_REVIEW_BETA_SCHEMA_VERSION {
        return Err(WorkflowBundleReviewValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, WORKFLOW_BUNDLE_REVIEW_BETA_SCHEMA_VERSION
        )));
    }
    require_non_empty("review_id", &record.review_id)?;
    require_non_empty("minted_at", &record.minted_at)?;
    validate_identity(&record.bundle_identity)?;
    validate_source_class_legend(&record.source_class_legend)?;
    validate_detail(&record.detail)?;
    validate_certification(&record.bundle_identity, &record.certification)?;
    validate_install_update_review(&record.install_update_review)?;
    validate_drift_override_review(&record.drift_override_review)?;
    validate_remove_rollback_review(&record.remove_rollback_review)?;
    validate_mirror_offline_review(&record.bundle_identity, &record.mirror_offline)?;
    validate_support_export(&record.support_export)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_guardrails(&record.guardrails)?;
    validate_invariants(&record.review_invariants)?;
    Ok(())
}

fn validate_identity(
    identity: &WorkflowBundleIdentity,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_non_empty("bundle_identity.bundle_id", &identity.bundle_id)?;
    if identity.bundle_revision == 0 {
        return Err(WorkflowBundleReviewValidationError::new(
            "bundle_identity.bundle_revision must be greater than zero",
        ));
    }
    require_one_of(
        "bundle_identity.bundle_class",
        WORKFLOW_BUNDLE_CLASSES,
        &identity.bundle_class,
    )?;
    require_one_of(
        "bundle_identity.bundle_source_class",
        WORKFLOW_BUNDLE_SOURCE_CLASSES,
        &identity.bundle_source_class,
    )?;
    require_one_of(
        "bundle_identity.bundle_status_class",
        WORKFLOW_BUNDLE_STATUS_CLASSES,
        &identity.bundle_status_class,
    )?;
    require_one_of(
        "bundle_identity.support_class",
        WORKFLOW_BUNDLE_SUPPORT_CLASSES,
        &identity.support_class,
    )?;
    require_non_empty(
        "bundle_identity.signer_source_class",
        &identity.signer_source_class,
    )?;
    require_non_empty("bundle_identity.signer_ref", &identity.signer_ref)?;
    require_non_empty("bundle_identity.manifest_ref", &identity.manifest_ref)?;
    require_non_empty(
        "bundle_identity.manifest_digest_ref",
        &identity.manifest_digest_ref,
    )?;
    require_non_empty(
        "bundle_identity.compatible_aureline_range",
        &identity.compatible_aureline_range,
    )?;
    require_non_empty("bundle_identity.channel", &identity.channel)?;

    let source = identity.bundle_source_class.as_str();
    let status = identity.bundle_status_class.as_str();
    let valid_pair = match source {
        "certified" => matches!(
            status,
            "certified_current" | "certified_retest_pending" | "retest_needed"
        ),
        "managed_approved" => status == "managed_approved_current",
        "community" => matches!(status, "community_reviewed" | "community_unreviewed"),
        "imported" => status == "imported_pending_review",
        "local_draft" => status == "local_draft",
        _ => false,
    };
    if !valid_pair {
        return Err(WorkflowBundleReviewValidationError::new(format!(
            "bundle_identity source/status pairing is invalid: {source}/{status}"
        )));
    }
    if identity.bundle_class == "imported_user_bundle" && identity.imported_source_ref.is_none() {
        return Err(WorkflowBundleReviewValidationError::new(
            "imported_user_bundle must carry bundle_identity.imported_source_ref",
        ));
    }
    Ok(())
}

fn validate_source_class_legend(
    legend: &[WorkflowBundleSourceClassDisclosure],
) -> Result<(), WorkflowBundleReviewValidationError> {
    let mut classes = BTreeSet::new();
    for disclosure in legend {
        require_one_of(
            "source_class_legend[].source_class",
            WORKFLOW_BUNDLE_SOURCE_CLASSES,
            &disclosure.source_class,
        )?;
        require_non_empty(
            "source_class_legend[].display_label",
            &disclosure.display_label,
        )?;
        require_non_empty("source_class_legend[].caveat_ref", &disclosure.caveat_ref)?;
        if !classes.insert(disclosure.source_class.as_str()) {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "source_class_legend contains duplicate source class {}",
                disclosure.source_class
            )));
        }
    }
    let required = WORKFLOW_BUNDLE_SOURCE_CLASSES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let missing = required
        .difference(&classes)
        .map(|class| (*class).to_string())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(format!(
            "source_class_legend must distinguish every source class; missing {}",
            missing.join(",")
        )));
    }
    Ok(())
}

fn validate_detail(
    detail: &WorkflowBundleDetail,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_non_empty_items("detail.extension_sets", &detail.extension_sets)?;
    require_non_empty_items("detail.presets", &detail.presets)?;
    require_non_empty_items(
        "detail.task_launch_debug_recipes",
        &detail.task_launch_debug_recipes,
    )?;
    require_non_empty_items("detail.docs_tour_packs", &detail.docs_tour_packs)?;
    require_non_empty_items("detail.template_refs", &detail.template_refs)?;
    require_non_empty_items("detail.migration_mappings", &detail.migration_mappings)?;
    require_non_empty_items(
        "detail.certification_targets",
        &detail.certification_targets,
    )?;

    for (label, items) in [
        ("detail.extension_sets", detail.extension_sets.as_slice()),
        ("detail.presets", detail.presets.as_slice()),
        (
            "detail.task_launch_debug_recipes",
            detail.task_launch_debug_recipes.as_slice(),
        ),
        ("detail.docs_tour_packs", detail.docs_tour_packs.as_slice()),
        ("detail.template_refs", detail.template_refs.as_slice()),
        (
            "detail.migration_mappings",
            detail.migration_mappings.as_slice(),
        ),
        (
            "detail.certification_targets",
            detail.certification_targets.as_slice(),
        ),
    ] {
        validate_content_items(label, items)?;
    }
    Ok(())
}

fn validate_certification(
    identity: &WorkflowBundleIdentity,
    certification: &WorkflowBundleCertificationReview,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_one_of(
        "certification.source_badge_class",
        WORKFLOW_BUNDLE_SOURCE_CLASSES,
        &certification.source_badge_class,
    )?;
    require_one_of(
        "certification.evidence_freshness_class",
        WORKFLOW_BUNDLE_EVIDENCE_FRESHNESS_CLASSES,
        &certification.evidence_freshness_class,
    )?;
    require_one_of(
        "certification.certification_state_class",
        WORKFLOW_BUNDLE_CERTIFICATION_STATE_CLASSES,
        &certification.certification_state_class,
    )?;
    require_one_of(
        "certification.effective_badge_class",
        WORKFLOW_BUNDLE_EFFECTIVE_BADGE_CLASSES,
        &certification.effective_badge_class,
    )?;
    require_one_of(
        "certification.support_claim_class",
        WORKFLOW_BUNDLE_SUPPORT_CLAIM_CLASSES,
        &certification.support_claim_class,
    )?;
    if certification.compatibility_evidence_refs.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "certification.compatibility_evidence_refs must not be empty",
        ));
    }
    require_unique(
        "certification.compatibility_evidence_refs",
        &certification.compatibility_evidence_refs,
    )?;
    if identity.bundle_source_class != certification.source_badge_class {
        return Err(WorkflowBundleReviewValidationError::new(
            "certification.source_badge_class must match bundle_identity.bundle_source_class",
        ));
    }
    if matches!(
        certification.evidence_freshness_class.as_str(),
        "stale_past_window" | "evidence_unknown"
    ) || certification.retest_required
    {
        if matches!(
            certification.effective_badge_class.as_str(),
            "certified" | "managed_approved"
        ) {
            return Err(WorkflowBundleReviewValidationError::new(
                "stale or retest-required evidence cannot render certified or managed-approved effective badges",
            ));
        }
    }
    match certification.source_badge_class.as_str() {
        "certified" => {
            if certification.reference_workspace_ref.is_none() {
                return Err(WorkflowBundleReviewValidationError::new(
                    "certified bundle review must carry certification.reference_workspace_ref",
                ));
            }
            if certification.support_claim_class == "stable_launch_wedge_claim"
                && certification.effective_badge_class != "certified"
            {
                return Err(WorkflowBundleReviewValidationError::new(
                    "stable_launch_wedge_claim requires an effective certified badge",
                ));
            }
        }
        "managed_approved" => {
            if certification.support_claim_class != "managed_org_claim" {
                return Err(WorkflowBundleReviewValidationError::new(
                    "managed_approved source must use managed_org_claim",
                ));
            }
        }
        "community" => {
            if certification.support_claim_class != "community_no_certification_claim"
                || certification.effective_badge_class == "certified"
            {
                return Err(WorkflowBundleReviewValidationError::new(
                    "community source cannot imply certification",
                ));
            }
        }
        "imported" => {
            if certification.support_claim_class != "imported_pending_review_claim"
                || certification.effective_badge_class != "imported"
            {
                return Err(WorkflowBundleReviewValidationError::new(
                    "imported source must remain imported pending review",
                ));
            }
        }
        "local_draft" => {
            if certification.support_claim_class != "local_draft_no_claim"
                || certification.effective_badge_class != "local_draft"
            {
                return Err(WorkflowBundleReviewValidationError::new(
                    "local draft source cannot imply support certification",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_install_update_review(
    review: &WorkflowBundleInstallUpdateReview,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_non_empty(
        "install_update_review.review_state_class",
        &review.review_state_class,
    )?;
    if review.preview_refs.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "install_update_review.preview_refs must not be empty",
        ));
    }
    require_unique("install_update_review.preview_refs", &review.preview_refs)?;
    if review.diff_entries.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "install_update_review.diff_entries must not be empty",
        ));
    }
    validate_diff_entries(&review.diff_entries)?;
    validate_review_actions("install_update_review.actions", &review.actions)?;
    validate_rollback_checkpoint(&review.rollback_checkpoint)?;
    if review.side_effects.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "install_update_review.side_effects must not be empty",
        ));
    }
    for side_effect in &review.side_effects {
        require_non_empty(
            "install_update_review.side_effects[].side_effect_class",
            &side_effect.side_effect_class,
        )?;
        require_non_empty(
            "install_update_review.side_effects[].scope_class",
            &side_effect.scope_class,
        )?;
        require_non_empty(
            "install_update_review.side_effects[].summary_ref",
            &side_effect.summary_ref,
        )?;
    }

    let present = review
        .diff_entries
        .iter()
        .map(|entry| entry.change_axis.as_str())
        .collect::<BTreeSet<_>>();
    let missing = WORKFLOW_BUNDLE_REQUIRED_DIFF_AXES
        .iter()
        .copied()
        .filter(|axis| !present.contains(axis))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(format!(
            "install_update_review.diff_entries missing required axes: {}",
            missing.join(",")
        )));
    }
    Ok(())
}

fn validate_diff_entries(
    entries: &[WorkflowBundleDiffEntry],
) -> Result<(), WorkflowBundleReviewValidationError> {
    for entry in entries {
        require_one_of(
            "install_update_review.diff_entries[].change_axis",
            WORKFLOW_BUNDLE_DIFF_AXES,
            &entry.change_axis,
        )?;
        require_one_of(
            "install_update_review.diff_entries[].change_kind",
            WORKFLOW_BUNDLE_CHANGE_KINDS,
            &entry.change_kind,
        )?;
        require_non_empty(
            "install_update_review.diff_entries[].subject_kind",
            &entry.subject_kind,
        )?;
        require_non_empty(
            "install_update_review.diff_entries[].subject_ref",
            &entry.subject_ref,
        )?;
        require_one_of(
            "install_update_review.diff_entries[].ownership_class",
            WORKFLOW_BUNDLE_ASSET_OWNERSHIP_CLASSES,
            &entry.ownership_class,
        )?;
        if !entry.disclosure_required {
            return Err(WorkflowBundleReviewValidationError::new(
                "install_update_review.diff_entries[].disclosure_required must be true",
            ));
        }
        if !entry.keyboard_reachable {
            return Err(WorkflowBundleReviewValidationError::new(
                "install_update_review.diff_entries[].keyboard_reachable must be true",
            ));
        }
        if entry.change_kind == "preserved_local" && entry.local_override_ref.is_none() {
            return Err(WorkflowBundleReviewValidationError::new(
                "preserved_local diff entries must carry local_override_ref",
            ));
        }
    }
    Ok(())
}

fn validate_review_actions(
    label: &str,
    actions: &[WorkflowBundleReviewAction],
) -> Result<(), WorkflowBundleReviewValidationError> {
    if actions.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(format!(
            "{label} must not be empty"
        )));
    }
    let mut seen = BTreeSet::new();
    for action in actions {
        require_one_of(
            &format!("{label}[].action_id"),
            WORKFLOW_BUNDLE_REVIEW_ACTION_IDS,
            &action.action_id,
        )?;
        require_one_of(
            &format!("{label}[].rendered_state"),
            WORKFLOW_BUNDLE_ACTION_RENDERED_STATES,
            &action.rendered_state,
        )?;
        if !seen.insert(action.action_id.as_str()) {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "{label} contains duplicate action {}",
                action.action_id
            )));
        }
        if !action.keyboard_reachable {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "{label}[].keyboard_reachable must be true"
            )));
        }
        if action.rendered_state == "visible_disabled" && action.disabled_reason_code.is_none() {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "{label} visible_disabled action {} must carry disabled_reason_code",
                action.action_id
            )));
        }
    }
    Ok(())
}

fn validate_rollback_checkpoint(
    checkpoint: &WorkflowBundleRollbackCheckpoint,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_one_of(
        "install_update_review.rollback_checkpoint.linkage_class",
        WORKFLOW_BUNDLE_ROLLBACK_LINKAGE_CLASSES,
        &checkpoint.linkage_class,
    )?;
    require_one_of(
        "install_update_review.rollback_checkpoint.rollback_path_class",
        WORKFLOW_BUNDLE_ROLLBACK_PATH_CLASSES,
        &checkpoint.rollback_path_class,
    )?;
    if checkpoint.restorable_axes.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "install_update_review.rollback_checkpoint.restorable_axes must not be empty",
        ));
    }
    require_unique(
        "install_update_review.rollback_checkpoint.restorable_axes",
        &checkpoint.restorable_axes,
    )?;
    if checkpoint.linkage_class != "non_reversible_with_justification"
        && checkpoint.linkage_class != "non_reversible_pending_review"
        && checkpoint.checkpoint_ref.is_none()
    {
        return Err(WorkflowBundleReviewValidationError::new(
            "reversible rollback checkpoint linkages must carry checkpoint_ref",
        ));
    }
    if !checkpoint.attributable_to_review {
        return Err(WorkflowBundleReviewValidationError::new(
            "rollback checkpoint must be attributable_to_review",
        ));
    }
    Ok(())
}

fn validate_drift_override_review(
    review: &WorkflowBundleDriftOverrideReview,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_non_empty(
        "drift_override_review.drift_state_class",
        &review.drift_state_class,
    )?;
    if !review.field_package_task_granular {
        return Err(WorkflowBundleReviewValidationError::new(
            "drift_override_review.field_package_task_granular must be true",
        ));
    }
    for entry in &review.drift_entries {
        require_non_empty(
            "drift_override_review.drift_entries[].drift_state_class",
            &entry.drift_state_class,
        )?;
        require_one_of(
            "drift_override_review.drift_entries[].drift_axis",
            WORKFLOW_BUNDLE_DIFF_AXES,
            &entry.drift_axis,
        )?;
        require_one_of(
            "drift_override_review.drift_entries[].subject_granularity_class",
            WORKFLOW_BUNDLE_DRIFT_SUBJECT_GRANULARITY_CLASSES,
            &entry.subject_granularity_class,
        )?;
        require_non_empty(
            "drift_override_review.drift_entries[].subject_ref",
            &entry.subject_ref,
        )?;
        require_one_of(
            "drift_override_review.drift_entries[].asset_ownership_class",
            WORKFLOW_BUNDLE_ASSET_OWNERSHIP_CLASSES,
            &entry.asset_ownership_class,
        )?;
        require_non_empty(
            "drift_override_review.drift_entries[].claim_narrowing_class",
            &entry.claim_narrowing_class,
        )?;
        if entry.drift_state_class == "local_override"
            && entry.preserved_local_override_ref.is_none()
        {
            return Err(WorkflowBundleReviewValidationError::new(
                "local_override drift entries must preserve a local override ref",
            ));
        }
        validate_resolve_actions(&entry.resolve_actions)?;
    }
    Ok(())
}

fn validate_resolve_actions(
    actions: &[WorkflowBundleResolveAction],
) -> Result<(), WorkflowBundleReviewValidationError> {
    if actions.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "drift_override_review.drift_entries[].resolve_actions must not be empty",
        ));
    }
    let mut seen = BTreeSet::new();
    for action in actions {
        require_one_of(
            "drift_override_review.drift_entries[].resolve_actions[].action_id",
            WORKFLOW_BUNDLE_RESOLVE_ACTION_IDS,
            &action.action_id,
        )?;
        require_one_of(
            "drift_override_review.drift_entries[].resolve_actions[].rendered_state",
            WORKFLOW_BUNDLE_ACTION_RENDERED_STATES,
            &action.rendered_state,
        )?;
        if !seen.insert(action.action_id.as_str()) {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "drift resolve actions contain duplicate action {}",
                action.action_id
            )));
        }
        if !action.keyboard_reachable {
            return Err(WorkflowBundleReviewValidationError::new(
                "drift resolve actions must be keyboard reachable",
            ));
        }
        if action.rendered_state == "visible_disabled" && action.disabled_reason_code.is_none() {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "visible_disabled resolve action {} must carry disabled_reason_code",
                action.action_id
            )));
        }
        if matches!(
            action.action_id.as_str(),
            "resolve.adopt_bundle" | "resolve.rebase_to_bundle"
        ) && action.rendered_state == "enabled"
        {
            let destination = action.destination_ref.as_deref().ok_or_else(|| {
                WorkflowBundleReviewValidationError::new(
                    "enabled adopt/rebase resolve actions must route through bundle_change_preview",
                )
            })?;
            if !destination.contains("bundle_change_preview") {
                return Err(WorkflowBundleReviewValidationError::new(
                    "enabled adopt/rebase resolve actions must route through bundle_change_preview",
                ));
            }
        }
    }
    if !seen.contains("resolve.compare") {
        return Err(WorkflowBundleReviewValidationError::new(
            "drift resolve actions must include resolve.compare",
        ));
    }
    Ok(())
}

fn validate_remove_rollback_review(
    review: &WorkflowBundleRemoveRollbackReview,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_non_empty(
        "remove_rollback_review.review_state_class",
        &review.review_state_class,
    )?;
    require_non_empty(
        "remove_rollback_review.remove_review_ref",
        &review.remove_review_ref,
    )?;
    require_non_empty(
        "remove_rollback_review.rollback_target_ref",
        &review.rollback_target_ref,
    )?;
    require_non_empty(
        "remove_rollback_review.rollback_checkpoint_ref",
        &review.rollback_checkpoint_ref,
    )?;
    if review.removable_assets.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "remove_rollback_review.removable_assets must not be empty",
        ));
    }
    for asset in &review.removable_assets {
        require_non_empty(
            "remove_rollback_review.removable_assets[].asset_ref",
            &asset.asset_ref,
        )?;
        require_non_empty(
            "remove_rollback_review.removable_assets[].asset_kind",
            &asset.asset_kind,
        )?;
        require_one_of(
            "remove_rollback_review.removable_assets[].ownership_class",
            WORKFLOW_BUNDLE_ASSET_OWNERSHIP_CLASSES,
            &asset.ownership_class,
        )?;
        require_one_of(
            "remove_rollback_review.removable_assets[].safe_to_remove_class",
            WORKFLOW_BUNDLE_SAFE_TO_REMOVE_CLASSES,
            &asset.safe_to_remove_class,
        )?;
        if !asset.explicit_reviewed {
            return Err(WorkflowBundleReviewValidationError::new(
                "remove_rollback_review.removable_assets must be explicitly reviewed",
            ));
        }
        match asset.ownership_class.as_str() {
            "user_owned" if asset.safe_to_remove_class != "not_safe_to_remove_user_owned" => {
                return Err(WorkflowBundleReviewValidationError::new(
                    "user_owned removable assets must be not_safe_to_remove_user_owned",
                ));
            }
            "bundle_owned"
                if matches!(
                    asset.safe_to_remove_class.as_str(),
                    "not_safe_to_remove_user_owned" | "review_required_user_data_co_resident"
                ) =>
            {
                return Err(WorkflowBundleReviewValidationError::new(
                    "bundle_owned removable assets cannot carry user-owned safe-to-remove classes",
                ));
            }
            "shared_user_overlay_on_bundle"
                if !matches!(
                    asset.safe_to_remove_class.as_str(),
                    "safe_to_remove_user_overlay_preserved"
                        | "review_required_user_data_co_resident"
                ) =>
            {
                return Err(WorkflowBundleReviewValidationError::new(
                    "shared user overlays must preserve or explicitly review local data",
                ));
            }
            _ => {}
        }
    }
    for retained in &review.retained_local_overrides {
        require_non_empty(
            "remove_rollback_review.retained_local_overrides[].override_ref",
            &retained.override_ref,
        )?;
        require_non_empty(
            "remove_rollback_review.retained_local_overrides[].retained_class",
            &retained.retained_class,
        )?;
        require_non_empty(
            "remove_rollback_review.retained_local_overrides[].target_scope_class",
            &retained.target_scope_class,
        )?;
    }
    validate_review_actions("remove_rollback_review.actions", &review.actions)?;
    Ok(())
}

fn validate_mirror_offline_review(
    identity: &WorkflowBundleIdentity,
    review: &WorkflowBundleMirrorOfflineReview,
) -> Result<(), WorkflowBundleReviewValidationError> {
    require_one_of(
        "mirror_offline.posture_class",
        WORKFLOW_BUNDLE_MIRROR_OFFLINE_POSTURE_CLASSES,
        &review.posture_class,
    )?;
    require_non_empty(
        "mirror_offline.source_registry_ref",
        &review.source_registry_ref,
    )?;
    require_non_empty("mirror_offline.signer_ref", &review.signer_ref)?;
    require_non_empty(
        "mirror_offline.compatible_aureline_range",
        &review.compatible_aureline_range,
    )?;
    require_non_empty(
        "mirror_offline.offline_restore_review_ref",
        &review.offline_restore_review_ref,
    )?;
    if review.compatible_aureline_range != identity.compatible_aureline_range {
        return Err(WorkflowBundleReviewValidationError::new(
            "mirror_offline.compatible_aureline_range must match bundle identity",
        ));
    }
    if review.signer_ref != identity.signer_ref {
        return Err(WorkflowBundleReviewValidationError::new(
            "mirror_offline.signer_ref must match bundle identity signer_ref",
        ));
    }
    if matches!(
        review.posture_class.as_str(),
        "mirror_only" | "signed_offline_bundle"
    ) && review.mirror_ref.is_none()
        && review.offline_pack_ref.is_none()
    {
        return Err(WorkflowBundleReviewValidationError::new(
            "mirror/offline posture must cite mirror_ref or offline_pack_ref",
        ));
    }
    Ok(())
}

fn validate_support_export(
    export: &WorkflowBundleSupportExport,
) -> Result<(), WorkflowBundleReviewValidationError> {
    if export.export_packet_refs.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "support_export.export_packet_refs must not be empty",
        ));
    }
    if export.diagnostics_refs.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "support_export.diagnostics_refs must not be empty",
        ));
    }
    if export.cli_headless_refs.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "support_export.cli_headless_refs must not be empty",
        ));
    }
    if export.raw_secret_export_allowed
        || export.raw_user_content_export_allowed
        || export.raw_paths_export_allowed
    {
        return Err(WorkflowBundleReviewValidationError::new(
            "support_export raw export booleans must remain false",
        ));
    }
    require_one_of(
        "support_export.redaction_class",
        WORKFLOW_BUNDLE_REDACTION_CLASSES,
        &export.redaction_class,
    )?;
    Ok(())
}

fn validate_consumer_surfaces(
    surfaces: &[String],
) -> Result<(), WorkflowBundleReviewValidationError> {
    if surfaces.is_empty() {
        return Err(WorkflowBundleReviewValidationError::new(
            "consumer_surfaces must not be empty",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for surface in surfaces {
        require_one_of(
            "consumer_surfaces[]",
            WORKFLOW_BUNDLE_CONSUMER_SURFACES,
            surface,
        )?;
    }
    for required in [
        "start_center",
        "cli_headless",
        "diagnostics",
        "support_export",
    ] {
        if !surfaces.iter().any(|surface| surface == required) {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "consumer_surfaces must include {required}"
            )));
        }
    }
    Ok(())
}

fn validate_guardrails(
    guardrails: &WorkflowBundleReviewGuardrails,
) -> Result<(), WorkflowBundleReviewValidationError> {
    if !guardrails.providers_recommended_only
        || !guardrails.remote_modes_recommended_only
        || !guardrails.templates_recommended_only
    {
        return Err(WorkflowBundleReviewValidationError::new(
            "provider, remote-mode, and template routes must remain recommendation-only",
        ));
    }
    if !guardrails_pass(guardrails) {
        return Err(WorkflowBundleReviewValidationError::new(
            "workflow bundle guardrails forbid silent workspace trust, network egress, policy scope, or approval-default widening",
        ));
    }
    Ok(())
}

fn guardrails_pass(guardrails: &WorkflowBundleReviewGuardrails) -> bool {
    !guardrails.workspace_trust_widened
        && !guardrails.network_egress_widened_without_review
        && !guardrails.policy_scope_widened_without_review
        && !guardrails.approval_defaults_widened_without_review
}

fn validate_invariants(
    invariants: &WorkflowBundleReviewInvariants,
) -> Result<(), WorkflowBundleReviewValidationError> {
    if !invariants.diff_before_apply
        || !invariants.rollback_reviewed
        || !invariants.removal_preserves_user_assets
        || !invariants.cli_headless_parity
        || !invariants.diagnostics_export_parity
        || !invariants.offline_mirror_truth_preserved
        || !invariants.no_hidden_imperative_hooks
        || !invariants.no_raw_secret_injection
    {
        return Err(WorkflowBundleReviewValidationError::new(
            "review_invariants must all be true",
        ));
    }
    Ok(())
}

fn require_non_empty_items(
    label: &str,
    items: &[WorkflowBundleContentItem],
) -> Result<(), WorkflowBundleReviewValidationError> {
    if items.is_empty() {
        Err(WorkflowBundleReviewValidationError::new(format!(
            "{label} must not be empty"
        )))
    } else {
        Ok(())
    }
}

fn validate_content_items(
    label: &str,
    items: &[WorkflowBundleContentItem],
) -> Result<(), WorkflowBundleReviewValidationError> {
    let mut seen = BTreeSet::new();
    for item in items {
        require_non_empty(&format!("{label}[].item_ref"), &item.item_ref)?;
        require_non_empty(&format!("{label}[].item_class"), &item.item_class)?;
        require_one_of(
            &format!("{label}[].ownership_class"),
            WORKFLOW_BUNDLE_ASSET_OWNERSHIP_CLASSES,
            &item.ownership_class,
        )?;
        require_one_of(
            &format!("{label}[].source_class"),
            WORKFLOW_BUNDLE_SOURCE_CLASSES,
            &item.source_class,
        )?;
        require_non_empty(&format!("{label}[].summary_ref"), &item.summary_ref)?;
        require_non_empty(&format!("{label}[].revision_ref"), &item.revision_ref)?;
        if !item.disclosure_required {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "{label}[].disclosure_required must be true"
            )));
        }
        if !seen.insert(item.item_ref.as_str()) {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "{label} contains duplicate item_ref {}",
                item.item_ref
            )));
        }
    }
    Ok(())
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), WorkflowBundleReviewValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(WorkflowBundleReviewValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), WorkflowBundleReviewValidationError> {
    if value.trim().is_empty() {
        Err(WorkflowBundleReviewValidationError::new(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn require_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), WorkflowBundleReviewValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(WorkflowBundleReviewValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), WorkflowBundleReviewValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(WorkflowBundleReviewValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAUNCH_FIXTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/workflow_bundle_review/launch_wedge_install_update_drift_rollback.json"
    ));

    #[test]
    fn launch_fixture_projects() {
        let projection = project_workflow_bundle_review(LAUNCH_FIXTURE).expect("valid review");
        assert_eq!(
            projection.bundle_id,
            "launch_bundle:typescript_web_app.seed"
        );
        assert_eq!(projection.effective_badge_class, "certified");
        assert!(projection.missing_required_diff_axes.is_empty());
        assert!(projection
            .resolve_actions
            .iter()
            .any(|action| action == "resolve.keep_local"));
        assert!(!projection.raw_export_allowed);
        assert!(projection.guardrails_pass);
    }
}
