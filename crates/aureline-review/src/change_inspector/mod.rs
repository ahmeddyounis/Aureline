//! Change-lineage alpha — the review-time projection of one change-object
//! record. The record is the durable, exportable row the landing-state
//! inspector renders **before** a branch, worktree, or patch stack is
//! published, merged, or applied. It binds the underlying change-object id
//! and re-projects its target identity and ancestry, then adds the
//! conflict-state and publish-readiness truth the inspector needs so a
//! reviewer can answer four questions from one row:
//!
//! 1. **Which scope am I operating on?** `active_scope_class` distinguishes
//!    the main worktree from a side worktree from a stacked patch set so a
//!    user can never widen mutation by accident.
//! 2. **Where will the change land?** `target_summary` re-projects the
//!    change-object landing target — landing-state class, landing-action
//!    class, target ref, mutation authority, remote visibility, and
//!    required network egress.
//! 3. **What does the lineage look like?** `ancestry_view` re-projects the
//!    base ref, divergence class, commits-ahead / commits-behind counts,
//!    and the ancestor chain so support packets and review surfaces quote
//!    one lineage truth.
//! 4. **Is it ready, and what's in the way?** `conflict_state` and
//!    `publish_readiness` carry closed-vocabulary classes and a bounded
//!    list of blockers so the inspector can name the next step.
//!
//! The companion schema lives at
//! `schemas/review/change_lineage.schema.json`. The reviewer doc lives at
//! `docs/ux/m3/change_lineage_alpha.md`. Canonical fixtures live under
//! `fixtures/review/m3/change_lineage/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every alpha change-lineage record.
pub const CHANGE_LINEAGE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for [`ChangeLineageRecord`].
pub const CHANGE_LINEAGE_ALPHA_RECORD_KIND: &str = "change_lineage_alpha_record";

/// Closed set of change-object kinds the lineage record can wrap.
pub const CHANGE_LINEAGE_OBJECT_KINDS: &[&str] = &["branch", "worktree", "patch_stack"];

/// Closed set of active-scope classes carried on every record.
pub const CHANGE_LINEAGE_ACTIVE_SCOPE_CLASSES: &[&str] = &[
    "main_worktree",
    "side_worktree",
    "stacked_patch_set",
    "detached_inspection",
    "active_scope_unknown_requires_review",
];

/// Closed set of landing-state classes (mirrors the change-object family).
pub const CHANGE_LINEAGE_LANDING_STATE_CLASSES: &[&str] = &[
    "local_only_no_remote_yet",
    "pending_publish_to_remote",
    "pending_merge_into_base",
    "pending_patch_apply",
    "landed_locally_only",
    "landed_publicly",
    "degraded_unknown_target_requires_review",
];

/// Closed set of landing-action classes.
pub const CHANGE_LINEAGE_LANDING_ACTION_CLASSES: &[&str] = &[
    "publish",
    "merge",
    "apply",
    "inspect_only",
    "action_class_unknown_requires_review",
];

/// Closed set of mutation-authority classes.
pub const CHANGE_LINEAGE_MUTATION_AUTHORITY_CLASSES: &[&str] = &[
    "local_only",
    "provider_bound",
    "managed_workspace_bound",
    "mirror_cached",
    "mutation_authority_unknown_requires_review",
];

/// Closed set of remote-visibility classes.
pub const CHANGE_LINEAGE_REMOTE_VISIBILITY_CLASSES: &[&str] = &[
    "no_remote_attached",
    "remote_attached_private",
    "remote_attached_team",
    "remote_attached_public",
    "remote_visibility_unknown_requires_review",
];

/// Closed set of required network-egress classes.
pub const CHANGE_LINEAGE_NETWORK_EGRESS_CLASSES: &[&str] = &[
    "no_network_egress_required",
    "first_party_origin_only",
    "team_managed_mirror_only",
    "provider_bound_origin_required",
    "managed_workspace_envelope_only",
    "egress_envelope_unknown_requires_review",
];

/// Closed set of divergence classes.
pub const CHANGE_LINEAGE_DIVERGENCE_CLASSES: &[&str] = &[
    "even_with_base",
    "ahead_of_base",
    "behind_base",
    "ahead_and_behind",
    "no_base_bound",
    "divergence_unknown_requires_review",
];

/// Closed set of conflict-state classes.
pub const CHANGE_LINEAGE_CONFLICT_STATE_CLASSES: &[&str] = &[
    "no_conflicts_detected",
    "merge_conflicts_pending_review",
    "rebase_conflicts_pending_review",
    "apply_conflicts_pending_review",
    "upstream_diverged_requires_rebase",
    "conflict_state_unknown_requires_review",
];

/// Closed set of publish-readiness classes.
pub const CHANGE_LINEAGE_PUBLISH_READINESS_CLASSES: &[&str] = &[
    "ready_to_publish",
    "ready_to_merge",
    "ready_to_apply",
    "blocked_by_conflicts",
    "blocked_by_review_required",
    "blocked_by_authority",
    "not_applicable_inspect_only",
    "readiness_unknown_requires_review",
];

/// Closed set of readiness-blocker classes.
pub const CHANGE_LINEAGE_READINESS_BLOCKER_CLASSES: &[&str] = &[
    "conflict_resolution_required",
    "rebase_required",
    "review_approval_required",
    "authority_widening_required",
    "remote_visibility_widening_required",
    "policy_review_required",
    "no_blockers",
    "blocker_class_unknown_requires_review",
];

/// Closed set of consumer surfaces.
pub const CHANGE_LINEAGE_CONSUMER_SURFACES: &[&str] = &[
    "change_inspector",
    "change_object_inspector",
    "activity_center",
    "review_preview",
    "cli_headless_entry",
    "support_export",
    "docs_review",
];

/// Ancestor entry carried on `ancestry_view.ancestor_chain`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageAncestorEntry {
    pub ancestor_ref: String,
    pub ancestor_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ancestor_note: Option<String>,
}

/// Ancestry view re-projected from the change-object lineage block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageAncestryView {
    pub base_ref: String,
    pub base_kind: String,
    pub divergence_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_ahead: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_behind: Option<u32>,
    pub ancestor_chain: Vec<ChangeLineageAncestorEntry>,
}

/// Target summary re-projected from the change-object landing state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageTargetSummary {
    pub landing_state_class: String,
    pub landing_action_class: String,
    pub target_ref: String,
    pub target_kind: String,
    pub mutation_authority_class: String,
    pub remote_visibility_class: String,
    pub required_network_egress_class: String,
    pub pending_writes_summary: String,
}

/// Conflict-state block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageConflictState {
    pub conflict_state_class: String,
    pub conflict_path_count: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflict_notes: Vec<String>,
}

/// Publish-readiness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineagePublishReadiness {
    pub publish_readiness_class: String,
    pub blockers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_notes: Vec<String>,
}

/// Closed support-export disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageSupportExport {
    pub export_packet_refs: Vec<String>,
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_remote_url_export_allowed: bool,
    pub raw_diff_body_export_allowed: bool,
    pub redaction_class: String,
}

/// Pre-execution review invariants the record must claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageReviewInvariants {
    pub target_ref_pinned: bool,
    pub ancestry_pinned: bool,
    pub conflict_state_inspectable: bool,
    pub publish_readiness_inspectable: bool,
    pub no_hidden_target_mutation: bool,
}

/// One alpha change-lineage record bound to a single change-object id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeLineageRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub change_lineage_id: String,
    pub change_object_ref: String,
    pub change_object_kind: String,
    pub display_label: String,
    pub summary: String,
    pub active_scope_class: String,
    pub operator_caveat: String,
    pub target_summary: ChangeLineageTargetSummary,
    pub ancestry_view: ChangeLineageAncestryView,
    pub conflict_state: ChangeLineageConflictState,
    pub publish_readiness: ChangeLineagePublishReadiness,
    pub consumer_surfaces: Vec<String>,
    pub support_export: ChangeLineageSupportExport,
    pub review_invariants: ChangeLineageReviewInvariants,
    pub minted_at: String,
}

/// Compact projection consumed by shell, CLI / headless, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLineageProjection {
    pub change_lineage_id: String,
    pub change_object_ref: String,
    pub change_object_kind: String,
    pub display_label: String,
    pub summary: String,
    pub active_scope_class: String,
    pub operator_caveat: String,
    pub landing_state_class: String,
    pub landing_action_class: String,
    pub target_ref: String,
    pub target_kind: String,
    pub mutation_authority_class: String,
    pub remote_visibility_class: String,
    pub required_network_egress_class: String,
    pub pending_writes_summary: String,
    pub base_ref: String,
    pub base_kind: String,
    pub divergence_class: String,
    pub commits_ahead: Option<u32>,
    pub commits_behind: Option<u32>,
    pub ancestor_chain_len: usize,
    pub conflict_state_class: String,
    pub conflict_path_count: u32,
    pub conflict_notes: Vec<String>,
    pub publish_readiness_class: String,
    pub readiness_blockers: Vec<String>,
    pub readiness_notes: Vec<String>,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_remote_url_export_allowed: bool,
    pub raw_diff_body_export_allowed: bool,
}

impl ChangeLineageRecord {
    /// Validates the record against the alpha change-lineage contract.
    ///
    /// # Errors
    ///
    /// Returns [`ChangeLineageValidationError`] when any frozen guarantee is
    /// violated.
    pub fn validate(&self) -> Result<(), ChangeLineageValidationError> {
        validate_record(self)
    }

    /// Projects the record into the compact landing-state inspector row.
    pub fn project(&self) -> ChangeLineageProjection {
        ChangeLineageProjection {
            change_lineage_id: self.change_lineage_id.clone(),
            change_object_ref: self.change_object_ref.clone(),
            change_object_kind: self.change_object_kind.clone(),
            display_label: self.display_label.clone(),
            summary: self.summary.clone(),
            active_scope_class: self.active_scope_class.clone(),
            operator_caveat: self.operator_caveat.clone(),
            landing_state_class: self.target_summary.landing_state_class.clone(),
            landing_action_class: self.target_summary.landing_action_class.clone(),
            target_ref: self.target_summary.target_ref.clone(),
            target_kind: self.target_summary.target_kind.clone(),
            mutation_authority_class: self.target_summary.mutation_authority_class.clone(),
            remote_visibility_class: self.target_summary.remote_visibility_class.clone(),
            required_network_egress_class: self
                .target_summary
                .required_network_egress_class
                .clone(),
            pending_writes_summary: self.target_summary.pending_writes_summary.clone(),
            base_ref: self.ancestry_view.base_ref.clone(),
            base_kind: self.ancestry_view.base_kind.clone(),
            divergence_class: self.ancestry_view.divergence_class.clone(),
            commits_ahead: self.ancestry_view.commits_ahead,
            commits_behind: self.ancestry_view.commits_behind,
            ancestor_chain_len: self.ancestry_view.ancestor_chain.len(),
            conflict_state_class: self.conflict_state.conflict_state_class.clone(),
            conflict_path_count: self.conflict_state.conflict_path_count,
            conflict_notes: self.conflict_state.conflict_notes.clone(),
            publish_readiness_class: self.publish_readiness.publish_readiness_class.clone(),
            readiness_blockers: self.publish_readiness.blockers.clone(),
            readiness_notes: self.publish_readiness.readiness_notes.clone(),
            consumer_surfaces: self.consumer_surfaces.clone(),
            support_export_refs: self.support_export.export_packet_refs.clone(),
            redaction_class: self.support_export.redaction_class.clone(),
            raw_path_export_allowed: self.support_export.raw_path_export_allowed,
            raw_branch_name_export_allowed: self.support_export.raw_branch_name_export_allowed,
            raw_remote_url_export_allowed: self.support_export.raw_remote_url_export_allowed,
            raw_diff_body_export_allowed: self.support_export.raw_diff_body_export_allowed,
        }
    }
}

/// Validation failure for a change-lineage record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLineageValidationError {
    message: String,
}

impl ChangeLineageValidationError {
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

impl fmt::Display for ChangeLineageValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "change-lineage validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for ChangeLineageValidationError {}

/// Error returned when a change-lineage JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeLineageError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha change-lineage contract.
    Validation(ChangeLineageValidationError),
}

impl ChangeLineageError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for ChangeLineageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "change-lineage JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ChangeLineageError {}

/// Parses and validates an alpha change-lineage JSON payload, returning the
/// compact landing-state inspector projection on success.
pub fn project_change_lineage(
    payload: &str,
) -> Result<ChangeLineageProjection, ChangeLineageError> {
    let record: ChangeLineageRecord =
        serde_json::from_str(payload).map_err(|err| ChangeLineageError::Json(err.to_string()))?;
    record.validate().map_err(ChangeLineageError::Validation)?;
    Ok(record.project())
}

fn validate_record(
    record: &ChangeLineageRecord,
) -> Result<(), ChangeLineageValidationError> {
    require_equal(
        "record_kind",
        CHANGE_LINEAGE_ALPHA_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != CHANGE_LINEAGE_ALPHA_SCHEMA_VERSION {
        return Err(ChangeLineageValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, CHANGE_LINEAGE_ALPHA_SCHEMA_VERSION
        )));
    }
    require_non_empty("change_lineage_id", &record.change_lineage_id)?;
    require_non_empty("change_object_ref", &record.change_object_ref)?;
    require_one_of(
        "change_object_kind",
        CHANGE_LINEAGE_OBJECT_KINDS,
        &record.change_object_kind,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_one_of(
        "active_scope_class",
        CHANGE_LINEAGE_ACTIVE_SCOPE_CLASSES,
        &record.active_scope_class,
    )?;
    require_non_empty("operator_caveat", &record.operator_caveat)?;
    require_non_empty("minted_at", &record.minted_at)?;

    validate_target_summary(&record.target_summary)?;
    validate_ancestry_view(&record.ancestry_view)?;
    validate_conflict_state(&record.conflict_state)?;
    validate_publish_readiness(&record.publish_readiness)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    validate_review_invariants(&record.review_invariants)?;
    cross_check_scope(record)?;
    cross_check_readiness(record)?;
    Ok(())
}

fn validate_target_summary(
    target: &ChangeLineageTargetSummary,
) -> Result<(), ChangeLineageValidationError> {
    require_one_of(
        "target_summary.landing_state_class",
        CHANGE_LINEAGE_LANDING_STATE_CLASSES,
        &target.landing_state_class,
    )?;
    require_one_of(
        "target_summary.landing_action_class",
        CHANGE_LINEAGE_LANDING_ACTION_CLASSES,
        &target.landing_action_class,
    )?;
    require_non_empty("target_summary.target_ref", &target.target_ref)?;
    require_non_empty("target_summary.target_kind", &target.target_kind)?;
    require_one_of(
        "target_summary.mutation_authority_class",
        CHANGE_LINEAGE_MUTATION_AUTHORITY_CLASSES,
        &target.mutation_authority_class,
    )?;
    require_one_of(
        "target_summary.remote_visibility_class",
        CHANGE_LINEAGE_REMOTE_VISIBILITY_CLASSES,
        &target.remote_visibility_class,
    )?;
    require_one_of(
        "target_summary.required_network_egress_class",
        CHANGE_LINEAGE_NETWORK_EGRESS_CLASSES,
        &target.required_network_egress_class,
    )?;
    require_non_empty(
        "target_summary.pending_writes_summary",
        &target.pending_writes_summary,
    )?;
    Ok(())
}

fn validate_ancestry_view(
    view: &ChangeLineageAncestryView,
) -> Result<(), ChangeLineageValidationError> {
    require_non_empty("ancestry_view.base_ref", &view.base_ref)?;
    require_non_empty("ancestry_view.base_kind", &view.base_kind)?;
    require_one_of(
        "ancestry_view.divergence_class",
        CHANGE_LINEAGE_DIVERGENCE_CLASSES,
        &view.divergence_class,
    )?;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for entry in &view.ancestor_chain {
        require_non_empty(
            "ancestry_view.ancestor_chain[].ancestor_ref",
            &entry.ancestor_ref,
        )?;
        require_non_empty(
            "ancestry_view.ancestor_chain[].ancestor_kind",
            &entry.ancestor_kind,
        )?;
        if !seen.insert(entry.ancestor_ref.as_str()) {
            return Err(ChangeLineageValidationError::new(format!(
                "ancestry_view.ancestor_chain contains a duplicate ancestor_ref: {}",
                entry.ancestor_ref
            )));
        }
    }
    Ok(())
}

fn validate_conflict_state(
    conflict: &ChangeLineageConflictState,
) -> Result<(), ChangeLineageValidationError> {
    require_one_of(
        "conflict_state.conflict_state_class",
        CHANGE_LINEAGE_CONFLICT_STATE_CLASSES,
        &conflict.conflict_state_class,
    )?;
    if conflict.conflict_state_class == "no_conflicts_detected" && conflict.conflict_path_count != 0
    {
        return Err(ChangeLineageValidationError::new(
            "conflict_state.conflict_path_count must be zero when conflict_state_class is no_conflicts_detected",
        ));
    }
    if matches!(
        conflict.conflict_state_class.as_str(),
        "merge_conflicts_pending_review"
            | "rebase_conflicts_pending_review"
            | "apply_conflicts_pending_review"
    ) && conflict.conflict_path_count == 0
    {
        return Err(ChangeLineageValidationError::new(
            "conflict_state.conflict_path_count must be greater than zero for pending-conflict classes",
        ));
    }
    Ok(())
}

fn validate_publish_readiness(
    readiness: &ChangeLineagePublishReadiness,
) -> Result<(), ChangeLineageValidationError> {
    require_one_of(
        "publish_readiness.publish_readiness_class",
        CHANGE_LINEAGE_PUBLISH_READINESS_CLASSES,
        &readiness.publish_readiness_class,
    )?;
    require_unique("publish_readiness.blockers", &readiness.blockers)?;
    for blocker in &readiness.blockers {
        require_one_of(
            "publish_readiness.blockers[]",
            CHANGE_LINEAGE_READINESS_BLOCKER_CLASSES,
            blocker,
        )?;
    }
    let ready_class = matches!(
        readiness.publish_readiness_class.as_str(),
        "ready_to_publish" | "ready_to_merge" | "ready_to_apply" | "not_applicable_inspect_only"
    );
    let only_no_blockers = readiness.blockers.iter().all(|blocker| blocker == "no_blockers");
    if ready_class && !only_no_blockers {
        return Err(ChangeLineageValidationError::new(
            "ready_to_* and not_applicable_inspect_only publish-readiness classes must carry no_blockers only",
        ));
    }
    let blocked_class = matches!(
        readiness.publish_readiness_class.as_str(),
        "blocked_by_conflicts"
            | "blocked_by_review_required"
            | "blocked_by_authority"
            | "readiness_unknown_requires_review"
    );
    if blocked_class
        && (readiness.blockers.is_empty() || readiness.blockers.iter().all(|b| b == "no_blockers"))
    {
        return Err(ChangeLineageValidationError::new(
            "blocked_by_* and readiness_unknown_requires_review must declare at least one real blocker",
        ));
    }
    Ok(())
}

fn validate_consumer_surfaces(
    surfaces: &[String],
) -> Result<(), ChangeLineageValidationError> {
    if surfaces.is_empty() {
        return Err(ChangeLineageValidationError::new(
            "consumer_surfaces must list at least one consumer surface",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for surface in surfaces {
        require_one_of(
            "consumer_surfaces[]",
            CHANGE_LINEAGE_CONSUMER_SURFACES,
            surface,
        )?;
    }
    if !surfaces.iter().any(|s| s == "change_inspector") {
        return Err(ChangeLineageValidationError::new(
            "consumer_surfaces must include change_inspector so the first product surface stays wired",
        ));
    }
    Ok(())
}

fn validate_support_export(
    export: &ChangeLineageSupportExport,
) -> Result<(), ChangeLineageValidationError> {
    if export.raw_path_export_allowed
        || export.raw_branch_name_export_allowed
        || export.raw_remote_url_export_allowed
        || export.raw_diff_body_export_allowed
    {
        return Err(ChangeLineageValidationError::new(
            "support_export must keep raw_*_export_allowed false",
        ));
    }
    require_non_empty("support_export.redaction_class", &export.redaction_class)?;
    require_unique(
        "support_export.export_packet_refs",
        &export.export_packet_refs,
    )?;
    Ok(())
}

fn validate_review_invariants(
    invariants: &ChangeLineageReviewInvariants,
) -> Result<(), ChangeLineageValidationError> {
    if !invariants.target_ref_pinned
        || !invariants.ancestry_pinned
        || !invariants.conflict_state_inspectable
        || !invariants.publish_readiness_inspectable
        || !invariants.no_hidden_target_mutation
    {
        return Err(ChangeLineageValidationError::new(
            "review_invariants must all be true; the change-lineage record is a pre-execution review record",
        ));
    }
    Ok(())
}

fn cross_check_scope(
    record: &ChangeLineageRecord,
) -> Result<(), ChangeLineageValidationError> {
    match record.change_object_kind.as_str() {
        "branch" => {
            if matches!(record.active_scope_class.as_str(), "stacked_patch_set") {
                return Err(ChangeLineageValidationError::new(
                    "branch change-object records must not open a stacked_patch_set scope",
                ));
            }
        }
        "worktree" => {
            if !matches!(
                record.active_scope_class.as_str(),
                "main_worktree" | "side_worktree" | "active_scope_unknown_requires_review"
            ) {
                return Err(ChangeLineageValidationError::new(
                    "worktree change-object records must open a main_worktree or side_worktree scope",
                ));
            }
        }
        "patch_stack" => {
            if !matches!(
                record.active_scope_class.as_str(),
                "stacked_patch_set" | "active_scope_unknown_requires_review"
            ) {
                return Err(ChangeLineageValidationError::new(
                    "patch_stack change-object records must open the stacked_patch_set scope",
                ));
            }
        }
        other => {
            return Err(ChangeLineageValidationError::new(format!(
                "unsupported change_object_kind {other}"
            )));
        }
    }
    Ok(())
}

fn cross_check_readiness(
    record: &ChangeLineageRecord,
) -> Result<(), ChangeLineageValidationError> {
    let readiness = &record.publish_readiness;
    let conflict = &record.conflict_state;
    let target = &record.target_summary;

    match readiness.publish_readiness_class.as_str() {
        "ready_to_publish" => {
            if target.landing_action_class != "publish" {
                return Err(ChangeLineageValidationError::new(
                    "ready_to_publish requires target_summary.landing_action_class=publish",
                ));
            }
            if conflict.conflict_state_class != "no_conflicts_detected" {
                return Err(ChangeLineageValidationError::new(
                    "ready_to_publish requires conflict_state_class=no_conflicts_detected",
                ));
            }
        }
        "ready_to_merge" => {
            if target.landing_action_class != "merge" {
                return Err(ChangeLineageValidationError::new(
                    "ready_to_merge requires target_summary.landing_action_class=merge",
                ));
            }
            if conflict.conflict_state_class != "no_conflicts_detected" {
                return Err(ChangeLineageValidationError::new(
                    "ready_to_merge requires conflict_state_class=no_conflicts_detected",
                ));
            }
        }
        "ready_to_apply" => {
            if target.landing_action_class != "apply" {
                return Err(ChangeLineageValidationError::new(
                    "ready_to_apply requires target_summary.landing_action_class=apply",
                ));
            }
            if conflict.conflict_state_class != "no_conflicts_detected" {
                return Err(ChangeLineageValidationError::new(
                    "ready_to_apply requires conflict_state_class=no_conflicts_detected",
                ));
            }
        }
        "blocked_by_conflicts" => {
            if conflict.conflict_state_class == "no_conflicts_detected" {
                return Err(ChangeLineageValidationError::new(
                    "blocked_by_conflicts must report a non-no_conflicts_detected conflict state",
                ));
            }
        }
        "not_applicable_inspect_only" => {
            if target.landing_action_class != "inspect_only" {
                return Err(ChangeLineageValidationError::new(
                    "not_applicable_inspect_only requires target_summary.landing_action_class=inspect_only",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), ChangeLineageValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ChangeLineageValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(
    label: &str,
    value: &str,
) -> Result<(), ChangeLineageValidationError> {
    if value.trim().is_empty() {
        Err(ChangeLineageValidationError::new(format!(
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
) -> Result<(), ChangeLineageValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(ChangeLineageValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), ChangeLineageValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(ChangeLineageValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_BRANCH_PUBLISH_READY: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/change_lineage/branch_main_worktree_ready_to_publish.json"
    ));
    const FIXTURE_WORKTREE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/change_lineage/worktree_side_worktree_inspect_only.json"
    ));
    const FIXTURE_PATCH_STACK: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/change_lineage/patch_stack_blocked_by_conflicts.json"
    ));

    #[test]
    fn branch_ready_to_publish_projects() {
        let projection = project_change_lineage(FIXTURE_BRANCH_PUBLISH_READY)
            .expect("branch ready_to_publish fixture must project");
        assert_eq!(projection.change_object_kind, "branch");
        assert_eq!(projection.active_scope_class, "main_worktree");
        assert_eq!(projection.publish_readiness_class, "ready_to_publish");
        assert_eq!(projection.conflict_state_class, "no_conflicts_detected");
        assert_eq!(projection.landing_action_class, "publish");
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "change_inspector"));
    }

    #[test]
    fn worktree_inspect_only_projects() {
        let projection = project_change_lineage(FIXTURE_WORKTREE)
            .expect("worktree inspect-only fixture must project");
        assert_eq!(projection.change_object_kind, "worktree");
        assert!(matches!(
            projection.active_scope_class.as_str(),
            "side_worktree" | "main_worktree"
        ));
        assert_eq!(projection.publish_readiness_class, "not_applicable_inspect_only");
        assert_eq!(projection.landing_action_class, "inspect_only");
    }

    #[test]
    fn patch_stack_blocked_projects() {
        let projection = project_change_lineage(FIXTURE_PATCH_STACK)
            .expect("patch-stack blocked fixture must project");
        assert_eq!(projection.change_object_kind, "patch_stack");
        assert_eq!(projection.active_scope_class, "stacked_patch_set");
        assert_eq!(projection.publish_readiness_class, "blocked_by_conflicts");
        assert_ne!(projection.conflict_state_class, "no_conflicts_detected");
    }

    #[test]
    fn rejects_branch_with_stacked_patch_scope() {
        let mut record: ChangeLineageRecord =
            serde_json::from_str(FIXTURE_BRANCH_PUBLISH_READY).expect("fixture must parse");
        record.active_scope_class = "stacked_patch_set".to_string();
        let err = record
            .validate()
            .expect_err("branch must not open the stacked_patch_set scope");
        assert!(err.message().contains("stacked_patch_set"));
    }

    #[test]
    fn rejects_ready_to_publish_with_conflicts() {
        let mut record: ChangeLineageRecord =
            serde_json::from_str(FIXTURE_BRANCH_PUBLISH_READY).expect("fixture must parse");
        record.conflict_state.conflict_state_class =
            "merge_conflicts_pending_review".to_string();
        record.conflict_state.conflict_path_count = 1;
        let err = record
            .validate()
            .expect_err("ready_to_publish must not declare conflicts");
        assert!(err.message().contains("conflict"));
    }

    #[test]
    fn rejects_blocked_without_real_blocker() {
        let mut record: ChangeLineageRecord =
            serde_json::from_str(FIXTURE_PATCH_STACK).expect("fixture must parse");
        record.publish_readiness.blockers = vec!["no_blockers".to_string()];
        let err = record
            .validate()
            .expect_err("blocked_by_* must declare a real blocker");
        assert!(err.message().contains("real blocker"));
    }

    #[test]
    fn rejects_raw_path_export() {
        let mut record: ChangeLineageRecord =
            serde_json::from_str(FIXTURE_BRANCH_PUBLISH_READY).expect("fixture must parse");
        record.support_export.raw_path_export_allowed = true;
        let err = record
            .validate()
            .expect_err("must reject raw path export");
        assert!(err.message().contains("raw_"));
    }

    #[test]
    fn rejects_missing_change_inspector_consumer() {
        let mut record: ChangeLineageRecord =
            serde_json::from_str(FIXTURE_BRANCH_PUBLISH_READY).expect("fixture must parse");
        record
            .consumer_surfaces
            .retain(|surface| surface != "change_inspector");
        let err = record
            .validate()
            .expect_err("must reject consumer list without change_inspector");
        assert!(err.message().contains("change_inspector"));
    }
}
