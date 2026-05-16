//! Change-object alpha — branches, worktrees, and patch stacks as explicit
//! reviewable records with stable ids, lineage, and landing-state summaries.
//!
//! A [`ChangeObjectRecord`] is the durable, exportable record that Activity
//! Center, shell change-list surfaces, the change-object inspector, review
//! previews, CLI / headless entry, and support packets read **before** a
//! branch, worktree, or patch stack is published, merged, or applied. Every
//! record carries:
//!
//! 1. A stable [`change_object_id`] safe to quote in logs, support bundles,
//!    and RPC.
//! 2. A `change_object_kind` (`branch`, `worktree`, or `patch_stack`) plus the
//!    matching variant block — and only that variant block.
//! 3. A `lineage` block with the base ref, divergence class, optional
//!    commits-ahead / commits-behind counts, and an ancestor chain.
//! 4. A `landing_state` block naming the landing-state class, the action class
//!    (`publish`, `merge`, `apply`, or `inspect_only`), the target ref, the
//!    mutation-authority class, the remote-visibility class, the required
//!    network-egress class, and a reviewable pending-writes summary.
//! 5. A non-empty `consumer_surfaces` list that always includes
//!    `change_object_inspector` so the first product surface stays wired.
//! 6. A `support_export` block that closes raw path, raw branch-name, raw
//!    remote-URL, and raw diff-body export.
//! 7. A `review_invariants` block claiming the four pre-execution review
//!    truths (`inspectable_before_publish`, `inspectable_before_merge`,
//!    `inspectable_before_apply`, `no_hidden_target_mutation`) all true.
//!
//! The companion schema lives at
//! `schemas/workspace/change_object.schema.json`. The reviewer doc lives at
//! `docs/review/m3/change_objects_alpha.md`. Canonical fixtures live under
//! `fixtures/workspace/m3/change_objects/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every alpha change-object record.
pub const CHANGE_OBJECT_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for [`ChangeObjectRecord`].
pub const CHANGE_OBJECT_ALPHA_RECORD_KIND: &str = "change_object_alpha_record";

/// Closed set of change-object kinds.
pub const CHANGE_OBJECT_KINDS: &[&str] = &["branch", "worktree", "patch_stack"];

/// Closed set of landing-state classes.
pub const CHANGE_OBJECT_LANDING_STATE_CLASSES: &[&str] = &[
    "local_only_no_remote_yet",
    "pending_publish_to_remote",
    "pending_merge_into_base",
    "pending_patch_apply",
    "landed_locally_only",
    "landed_publicly",
    "degraded_unknown_target_requires_review",
];

/// Closed set of landing-action classes.
pub const CHANGE_OBJECT_LANDING_ACTION_CLASSES: &[&str] = &[
    "publish",
    "merge",
    "apply",
    "inspect_only",
    "action_class_unknown_requires_review",
];

/// Closed set of mutation-authority classes.
pub const CHANGE_OBJECT_MUTATION_AUTHORITY_CLASSES: &[&str] = &[
    "local_only",
    "provider_bound",
    "managed_workspace_bound",
    "mirror_cached",
    "mutation_authority_unknown_requires_review",
];

/// Closed set of remote-visibility classes.
pub const CHANGE_OBJECT_REMOTE_VISIBILITY_CLASSES: &[&str] = &[
    "no_remote_attached",
    "remote_attached_private",
    "remote_attached_team",
    "remote_attached_public",
    "remote_visibility_unknown_requires_review",
];

/// Closed set of network-egress classes carried on `landing_state`.
pub const CHANGE_OBJECT_NETWORK_EGRESS_CLASSES: &[&str] = &[
    "no_network_egress_required",
    "first_party_origin_only",
    "team_managed_mirror_only",
    "provider_bound_origin_required",
    "managed_workspace_envelope_only",
    "egress_envelope_unknown_requires_review",
];

/// Closed set of divergence classes carried on `lineage`.
pub const CHANGE_OBJECT_DIVERGENCE_CLASSES: &[&str] = &[
    "even_with_base",
    "ahead_of_base",
    "behind_base",
    "ahead_and_behind",
    "no_base_bound",
    "divergence_unknown_requires_review",
];

/// Closed set of branch-kind classes.
pub const CHANGE_OBJECT_BRANCH_KIND_CLASSES: &[&str] = &[
    "local_branch",
    "remote_tracking_branch",
    "detached_head",
    "orphan_branch",
    "branch_kind_unknown_requires_review",
];

/// Closed set of worktree-kind classes.
pub const CHANGE_OBJECT_WORKTREE_KIND_CLASSES: &[&str] = &[
    "primary_worktree",
    "linked_worktree",
    "sparse_worktree",
    "snapshot_worktree",
    "worktree_kind_unknown_requires_review",
];

/// Closed set of worktree-attachment classes.
pub const CHANGE_OBJECT_WORKTREE_ATTACHMENT_CLASSES: &[&str] = &[
    "attached_local",
    "attached_devcontainer",
    "attached_container",
    "attached_remote_workspace",
    "attached_managed_workspace",
    "detached_requires_reattach",
    "attachment_class_unknown_requires_review",
];

/// Closed set of patch-stack target classes.
pub const CHANGE_OBJECT_PATCH_STACK_TARGET_CLASSES: &[&str] = &[
    "current_branch",
    "named_local_branch",
    "remote_provider_pull_request",
    "remote_provider_change_request",
    "mailing_list_thread",
    "patch_stack_target_unknown_requires_review",
];

/// Closed set of patch-state classes.
pub const CHANGE_OBJECT_PATCH_STATE_CLASSES: &[&str] = &[
    "drafted",
    "applied_local_only",
    "applied_remote",
    "needs_rebase",
    "conflict_requires_review",
    "patch_state_unknown_requires_review",
];

/// Closed set of patch review classes.
pub const CHANGE_OBJECT_REVIEW_CLASSES: &[&str] = &[
    "not_requested",
    "draft_review",
    "open_review",
    "merged_review",
    "abandoned_review",
    "review_class_unknown_requires_review",
];

/// Closed set of change-object consumer surfaces.
pub const CHANGE_OBJECT_CONSUMER_SURFACES: &[&str] = &[
    "change_object_inspector",
    "activity_center",
    "review_preview",
    "cli_headless_entry",
    "support_export",
    "docs_review",
];

/// Ancestor entry carried on `lineage.ancestor_chain`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectLineageEntry {
    pub ancestor_ref: String,
    pub ancestor_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ancestor_note: Option<String>,
}

/// Lineage block re-exported on every change-object record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectLineage {
    pub base_ref: String,
    pub base_kind: String,
    pub divergence_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_ahead: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_behind: Option<u32>,
    pub ancestor_chain: Vec<ChangeObjectLineageEntry>,
}

/// Landing-state block re-exported on every change-object record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectLandingState {
    pub landing_state_class: String,
    pub landing_action_class: String,
    pub target_ref: String,
    pub target_kind: String,
    pub mutation_authority_class: String,
    pub remote_visibility_class: String,
    pub required_network_egress_class: String,
    pub pending_writes_summary: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub landing_notes: Vec<String>,
}

/// Branch-variant block carried when `change_object_kind == "branch"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectBranchVariant {
    pub branch_kind_class: String,
    pub head_ref_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncommitted_paths_count: Option<u32>,
}

/// Worktree-variant block carried when `change_object_kind == "worktree"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectWorktreeVariant {
    pub worktree_kind_class: String,
    pub worktree_attachment_class: String,
    pub checked_out_ref_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_branch_ref_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse_scope_token: Option<String>,
}

/// Patch-stack variant block carried when `change_object_kind == "patch_stack"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectPatchStackVariant {
    pub patch_stack_target_class: String,
    pub patch_state_class: String,
    pub patch_count: u32,
    pub top_patch_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_handle_label: Option<String>,
}

/// Closed support-export disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectSupportExport {
    pub export_packet_refs: Vec<String>,
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_remote_url_export_allowed: bool,
    pub raw_diff_body_export_allowed: bool,
    pub redaction_class: String,
}

/// Pre-execution review invariants the record must claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectReviewInvariants {
    pub inspectable_before_publish: bool,
    pub inspectable_before_merge: bool,
    pub inspectable_before_apply: bool,
    pub no_hidden_target_mutation: bool,
}

/// One alpha change-object record covering branch / worktree / patch-stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub change_object_id: String,
    pub change_object_kind: String,
    pub display_label: String,
    pub summary: String,
    pub lineage: ChangeObjectLineage,
    pub landing_state: ChangeObjectLandingState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<ChangeObjectBranchVariant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree: Option<ChangeObjectWorktreeVariant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_stack: Option<ChangeObjectPatchStackVariant>,
    pub consumer_surfaces: Vec<String>,
    pub support_export: ChangeObjectSupportExport,
    pub review_invariants: ChangeObjectReviewInvariants,
    pub minted_at: String,
}

/// Compact projection consumed by shell, CLI / headless, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeObjectProjection {
    pub change_object_id: String,
    pub change_object_kind: String,
    pub display_label: String,
    pub summary: String,
    pub base_ref: String,
    pub base_kind: String,
    pub divergence_class: String,
    pub commits_ahead: Option<u32>,
    pub commits_behind: Option<u32>,
    pub ancestor_chain_len: usize,
    pub landing_state_class: String,
    pub landing_action_class: String,
    pub target_ref: String,
    pub target_kind: String,
    pub mutation_authority_class: String,
    pub remote_visibility_class: String,
    pub required_network_egress_class: String,
    pub pending_writes_summary: String,
    pub landing_notes: Vec<String>,
    pub variant_class_summary: String,
    pub variant_ref_label: String,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_remote_url_export_allowed: bool,
    pub raw_diff_body_export_allowed: bool,
}

impl ChangeObjectRecord {
    /// Validates the record against the alpha change-object contract.
    ///
    /// # Errors
    ///
    /// Returns [`ChangeObjectValidationError`] when any frozen guarantee is
    /// violated.
    pub fn validate(&self) -> Result<(), ChangeObjectValidationError> {
        validate_record(self)
    }

    /// Projects the record into the compact change-object surface row.
    pub fn project(&self) -> ChangeObjectProjection {
        let (variant_class_summary, variant_ref_label) = match self.change_object_kind.as_str() {
            "branch" => self
                .branch
                .as_ref()
                .map(|variant| {
                    (
                        format!(
                            "kind={} upstream={}",
                            variant.branch_kind_class,
                            variant.upstream_ref_label.as_deref().unwrap_or("none"),
                        ),
                        variant.head_ref_label.clone(),
                    )
                })
                .unwrap_or_else(|| ("variant=missing".to_string(), String::new())),
            "worktree" => self
                .worktree
                .as_ref()
                .map(|variant| {
                    (
                        format!(
                            "kind={} attachment={}",
                            variant.worktree_kind_class, variant.worktree_attachment_class,
                        ),
                        variant.checked_out_ref_label.clone(),
                    )
                })
                .unwrap_or_else(|| ("variant=missing".to_string(), String::new())),
            "patch_stack" => self
                .patch_stack
                .as_ref()
                .map(|variant| {
                    (
                        format!(
                            "target={} state={} patches={}",
                            variant.patch_stack_target_class,
                            variant.patch_state_class,
                            variant.patch_count,
                        ),
                        variant.top_patch_label.clone(),
                    )
                })
                .unwrap_or_else(|| ("variant=missing".to_string(), String::new())),
            _ => ("variant=unknown".to_string(), String::new()),
        };
        ChangeObjectProjection {
            change_object_id: self.change_object_id.clone(),
            change_object_kind: self.change_object_kind.clone(),
            display_label: self.display_label.clone(),
            summary: self.summary.clone(),
            base_ref: self.lineage.base_ref.clone(),
            base_kind: self.lineage.base_kind.clone(),
            divergence_class: self.lineage.divergence_class.clone(),
            commits_ahead: self.lineage.commits_ahead,
            commits_behind: self.lineage.commits_behind,
            ancestor_chain_len: self.lineage.ancestor_chain.len(),
            landing_state_class: self.landing_state.landing_state_class.clone(),
            landing_action_class: self.landing_state.landing_action_class.clone(),
            target_ref: self.landing_state.target_ref.clone(),
            target_kind: self.landing_state.target_kind.clone(),
            mutation_authority_class: self.landing_state.mutation_authority_class.clone(),
            remote_visibility_class: self.landing_state.remote_visibility_class.clone(),
            required_network_egress_class: self
                .landing_state
                .required_network_egress_class
                .clone(),
            pending_writes_summary: self.landing_state.pending_writes_summary.clone(),
            landing_notes: self.landing_state.landing_notes.clone(),
            variant_class_summary,
            variant_ref_label,
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

/// Validation failure for a change-object record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeObjectValidationError {
    message: String,
}

impl ChangeObjectValidationError {
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

impl fmt::Display for ChangeObjectValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "change-object validation error: {}", self.message)
    }
}

impl std::error::Error for ChangeObjectValidationError {}

/// Error returned when a change-object JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeObjectError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha change-object contract.
    Validation(ChangeObjectValidationError),
}

impl ChangeObjectError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for ChangeObjectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "change-object JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ChangeObjectError {}

/// Parses and validates an alpha change-object JSON payload.
pub fn project_change_object(
    payload: &str,
) -> Result<ChangeObjectProjection, ChangeObjectError> {
    let record: ChangeObjectRecord = serde_json::from_str(payload)
        .map_err(|err| ChangeObjectError::Json(err.to_string()))?;
    record
        .validate()
        .map_err(ChangeObjectError::Validation)?;
    Ok(record.project())
}

fn validate_record(
    record: &ChangeObjectRecord,
) -> Result<(), ChangeObjectValidationError> {
    require_equal("record_kind", CHANGE_OBJECT_ALPHA_RECORD_KIND, &record.record_kind)?;
    if record.schema_version != CHANGE_OBJECT_ALPHA_SCHEMA_VERSION {
        return Err(ChangeObjectValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, CHANGE_OBJECT_ALPHA_SCHEMA_VERSION
        )));
    }
    require_non_empty("change_object_id", &record.change_object_id)?;
    require_one_of(
        "change_object_kind",
        CHANGE_OBJECT_KINDS,
        &record.change_object_kind,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;

    validate_lineage(&record.lineage)?;
    validate_landing_state(&record.landing_state)?;
    validate_variant(record)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    validate_review_invariants(&record.review_invariants)?;
    cross_check_landing(record)?;
    Ok(())
}

fn validate_lineage(lineage: &ChangeObjectLineage) -> Result<(), ChangeObjectValidationError> {
    require_non_empty("lineage.base_ref", &lineage.base_ref)?;
    require_non_empty("lineage.base_kind", &lineage.base_kind)?;
    require_one_of(
        "lineage.divergence_class",
        CHANGE_OBJECT_DIVERGENCE_CLASSES,
        &lineage.divergence_class,
    )?;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for entry in &lineage.ancestor_chain {
        require_non_empty(
            "lineage.ancestor_chain[].ancestor_ref",
            &entry.ancestor_ref,
        )?;
        require_non_empty(
            "lineage.ancestor_chain[].ancestor_kind",
            &entry.ancestor_kind,
        )?;
        if !seen.insert(entry.ancestor_ref.as_str()) {
            return Err(ChangeObjectValidationError::new(format!(
                "lineage.ancestor_chain contains a duplicate ancestor_ref: {}",
                entry.ancestor_ref
            )));
        }
    }
    Ok(())
}

fn validate_landing_state(
    landing: &ChangeObjectLandingState,
) -> Result<(), ChangeObjectValidationError> {
    require_one_of(
        "landing_state.landing_state_class",
        CHANGE_OBJECT_LANDING_STATE_CLASSES,
        &landing.landing_state_class,
    )?;
    require_one_of(
        "landing_state.landing_action_class",
        CHANGE_OBJECT_LANDING_ACTION_CLASSES,
        &landing.landing_action_class,
    )?;
    require_non_empty("landing_state.target_ref", &landing.target_ref)?;
    require_non_empty("landing_state.target_kind", &landing.target_kind)?;
    require_one_of(
        "landing_state.mutation_authority_class",
        CHANGE_OBJECT_MUTATION_AUTHORITY_CLASSES,
        &landing.mutation_authority_class,
    )?;
    require_one_of(
        "landing_state.remote_visibility_class",
        CHANGE_OBJECT_REMOTE_VISIBILITY_CLASSES,
        &landing.remote_visibility_class,
    )?;
    require_one_of(
        "landing_state.required_network_egress_class",
        CHANGE_OBJECT_NETWORK_EGRESS_CLASSES,
        &landing.required_network_egress_class,
    )?;
    require_non_empty(
        "landing_state.pending_writes_summary",
        &landing.pending_writes_summary,
    )?;
    Ok(())
}

fn validate_variant(record: &ChangeObjectRecord) -> Result<(), ChangeObjectValidationError> {
    match record.change_object_kind.as_str() {
        "branch" => {
            if record.worktree.is_some() || record.patch_stack.is_some() {
                return Err(ChangeObjectValidationError::new(
                    "branch change-object must not carry worktree or patch_stack variant blocks",
                ));
            }
            let branch = record.branch.as_ref().ok_or_else(|| {
                ChangeObjectValidationError::new(
                    "branch change-object must carry a branch variant block",
                )
            })?;
            require_one_of(
                "branch.branch_kind_class",
                CHANGE_OBJECT_BRANCH_KIND_CLASSES,
                &branch.branch_kind_class,
            )?;
            require_non_empty("branch.head_ref_label", &branch.head_ref_label)?;
            if let Some(upstream) = &branch.upstream_ref_label {
                require_non_empty("branch.upstream_ref_label", upstream)?;
            }
        }
        "worktree" => {
            if record.branch.is_some() || record.patch_stack.is_some() {
                return Err(ChangeObjectValidationError::new(
                    "worktree change-object must not carry branch or patch_stack variant blocks",
                ));
            }
            let worktree = record.worktree.as_ref().ok_or_else(|| {
                ChangeObjectValidationError::new(
                    "worktree change-object must carry a worktree variant block",
                )
            })?;
            require_one_of(
                "worktree.worktree_kind_class",
                CHANGE_OBJECT_WORKTREE_KIND_CLASSES,
                &worktree.worktree_kind_class,
            )?;
            require_one_of(
                "worktree.worktree_attachment_class",
                CHANGE_OBJECT_WORKTREE_ATTACHMENT_CLASSES,
                &worktree.worktree_attachment_class,
            )?;
            require_non_empty(
                "worktree.checked_out_ref_label",
                &worktree.checked_out_ref_label,
            )?;
            if let Some(linked) = &worktree.linked_branch_ref_label {
                require_non_empty("worktree.linked_branch_ref_label", linked)?;
            }
            if let Some(scope) = &worktree.sparse_scope_token {
                require_non_empty("worktree.sparse_scope_token", scope)?;
            }
        }
        "patch_stack" => {
            if record.branch.is_some() || record.worktree.is_some() {
                return Err(ChangeObjectValidationError::new(
                    "patch_stack change-object must not carry branch or worktree variant blocks",
                ));
            }
            let stack = record.patch_stack.as_ref().ok_or_else(|| {
                ChangeObjectValidationError::new(
                    "patch_stack change-object must carry a patch_stack variant block",
                )
            })?;
            require_one_of(
                "patch_stack.patch_stack_target_class",
                CHANGE_OBJECT_PATCH_STACK_TARGET_CLASSES,
                &stack.patch_stack_target_class,
            )?;
            require_one_of(
                "patch_stack.patch_state_class",
                CHANGE_OBJECT_PATCH_STATE_CLASSES,
                &stack.patch_state_class,
            )?;
            require_non_empty("patch_stack.top_patch_label", &stack.top_patch_label)?;
            if let Some(review_class) = &stack.review_class {
                require_one_of(
                    "patch_stack.review_class",
                    CHANGE_OBJECT_REVIEW_CLASSES,
                    review_class,
                )?;
            }
            if let Some(handle) = &stack.review_handle_label {
                require_non_empty("patch_stack.review_handle_label", handle)?;
            }
        }
        other => {
            return Err(ChangeObjectValidationError::new(format!(
                "unsupported change_object_kind {other}"
            )));
        }
    }
    Ok(())
}

fn validate_consumer_surfaces(
    surfaces: &[String],
) -> Result<(), ChangeObjectValidationError> {
    if surfaces.is_empty() {
        return Err(ChangeObjectValidationError::new(
            "consumer_surfaces must list at least one consumer surface",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for surface in surfaces {
        require_one_of(
            "consumer_surfaces[]",
            CHANGE_OBJECT_CONSUMER_SURFACES,
            surface,
        )?;
    }
    if !surfaces.iter().any(|s| s == "change_object_inspector") {
        return Err(ChangeObjectValidationError::new(
            "consumer_surfaces must include change_object_inspector so the first product surface stays wired",
        ));
    }
    Ok(())
}

fn validate_support_export(
    export: &ChangeObjectSupportExport,
) -> Result<(), ChangeObjectValidationError> {
    if export.raw_path_export_allowed
        || export.raw_branch_name_export_allowed
        || export.raw_remote_url_export_allowed
        || export.raw_diff_body_export_allowed
    {
        return Err(ChangeObjectValidationError::new(
            "support_export must keep raw_*_export_allowed false",
        ));
    }
    require_non_empty("support_export.redaction_class", &export.redaction_class)?;
    require_unique("support_export.export_packet_refs", &export.export_packet_refs)?;
    Ok(())
}

fn validate_review_invariants(
    invariants: &ChangeObjectReviewInvariants,
) -> Result<(), ChangeObjectValidationError> {
    if !invariants.inspectable_before_publish
        || !invariants.inspectable_before_merge
        || !invariants.inspectable_before_apply
        || !invariants.no_hidden_target_mutation
    {
        return Err(ChangeObjectValidationError::new(
            "review_invariants must all be true; the change-object record is a pre-execution review record",
        ));
    }
    Ok(())
}

fn cross_check_landing(record: &ChangeObjectRecord) -> Result<(), ChangeObjectValidationError> {
    let landing = &record.landing_state;
    match landing.landing_state_class.as_str() {
        "pending_publish_to_remote" => {
            if landing.remote_visibility_class == "no_remote_attached" {
                return Err(ChangeObjectValidationError::new(
                    "pending_publish_to_remote requires a remote-attached visibility class",
                ));
            }
            if landing.required_network_egress_class == "no_network_egress_required" {
                return Err(ChangeObjectValidationError::new(
                    "pending_publish_to_remote requires a non-zero network egress envelope",
                ));
            }
            if !matches!(
                landing.landing_action_class.as_str(),
                "publish" | "action_class_unknown_requires_review"
            ) {
                return Err(ChangeObjectValidationError::new(
                    "pending_publish_to_remote must declare landing_action_class=publish",
                ));
            }
        }
        "local_only_no_remote_yet" => {
            if !matches!(
                landing.remote_visibility_class.as_str(),
                "no_remote_attached" | "remote_visibility_unknown_requires_review"
            ) {
                return Err(ChangeObjectValidationError::new(
                    "local_only_no_remote_yet must keep remote_visibility_class detached",
                ));
            }
            if !matches!(
                landing.required_network_egress_class.as_str(),
                "no_network_egress_required" | "egress_envelope_unknown_requires_review"
            ) {
                return Err(ChangeObjectValidationError::new(
                    "local_only_no_remote_yet must keep required_network_egress_class closed",
                ));
            }
            if !matches!(
                landing.mutation_authority_class.as_str(),
                "local_only" | "mutation_authority_unknown_requires_review"
            ) {
                return Err(ChangeObjectValidationError::new(
                    "local_only_no_remote_yet must keep mutation_authority_class local-only",
                ));
            }
        }
        "pending_merge_into_base" => {
            if !matches!(
                landing.landing_action_class.as_str(),
                "merge" | "action_class_unknown_requires_review"
            ) {
                return Err(ChangeObjectValidationError::new(
                    "pending_merge_into_base must declare landing_action_class=merge",
                ));
            }
        }
        "pending_patch_apply" => {
            if !matches!(
                landing.landing_action_class.as_str(),
                "apply" | "action_class_unknown_requires_review"
            ) {
                return Err(ChangeObjectValidationError::new(
                    "pending_patch_apply must declare landing_action_class=apply",
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
) -> Result<(), ChangeObjectValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ChangeObjectValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), ChangeObjectValidationError> {
    if value.trim().is_empty() {
        Err(ChangeObjectValidationError::new(format!(
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
) -> Result<(), ChangeObjectValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(ChangeObjectValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), ChangeObjectValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(ChangeObjectValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_BRANCH: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/change_objects/branch_local_pending_publish.json"
    ));
    const FIXTURE_WORKTREE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/change_objects/worktree_linked_local_only.json"
    ));
    const FIXTURE_PATCH_STACK: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/change_objects/patch_stack_provider_pull_request.json"
    ));

    #[test]
    fn branch_fixture_projects() {
        let projection = project_change_object(FIXTURE_BRANCH).expect("branch fixture must project");
        assert_eq!(projection.change_object_kind, "branch");
        assert_eq!(projection.landing_state_class, "pending_publish_to_remote");
        assert_eq!(projection.landing_action_class, "publish");
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "change_object_inspector"));
    }

    #[test]
    fn worktree_fixture_projects() {
        let projection =
            project_change_object(FIXTURE_WORKTREE).expect("worktree fixture must project");
        assert_eq!(projection.change_object_kind, "worktree");
        assert_eq!(projection.landing_state_class, "local_only_no_remote_yet");
        assert_eq!(projection.remote_visibility_class, "no_remote_attached");
        assert_eq!(
            projection.required_network_egress_class,
            "no_network_egress_required"
        );
    }

    #[test]
    fn patch_stack_fixture_projects() {
        let projection =
            project_change_object(FIXTURE_PATCH_STACK).expect("patch-stack fixture must project");
        assert_eq!(projection.change_object_kind, "patch_stack");
        assert_eq!(projection.landing_state_class, "pending_patch_apply");
        assert_eq!(projection.landing_action_class, "apply");
    }

    #[test]
    fn rejects_publish_without_remote() {
        let mut record: ChangeObjectRecord =
            serde_json::from_str(FIXTURE_BRANCH).expect("branch fixture must parse");
        record.landing_state.remote_visibility_class = "no_remote_attached".to_string();
        let err = record
            .validate()
            .expect_err("publish without remote must fail");
        assert!(err.message().contains("remote-attached"));
    }

    #[test]
    fn rejects_local_only_with_remote_egress() {
        let mut record: ChangeObjectRecord =
            serde_json::from_str(FIXTURE_WORKTREE).expect("worktree fixture must parse");
        record.landing_state.required_network_egress_class =
            "first_party_origin_only".to_string();
        let err = record
            .validate()
            .expect_err("local_only_no_remote_yet must not declare egress");
        assert!(err
            .message()
            .contains("required_network_egress_class"));
    }

    #[test]
    fn rejects_branch_with_worktree_variant() {
        let mut record: ChangeObjectRecord =
            serde_json::from_str(FIXTURE_BRANCH).expect("branch fixture must parse");
        record.worktree = Some(ChangeObjectWorktreeVariant {
            worktree_kind_class: "primary_worktree".to_string(),
            worktree_attachment_class: "attached_local".to_string(),
            checked_out_ref_label: "ref:branch:alpha".to_string(),
            linked_branch_ref_label: None,
            sparse_scope_token: None,
        });
        let err = record
            .validate()
            .expect_err("branch must not carry worktree variant");
        assert!(err.message().contains("worktree"));
    }

    #[test]
    fn rejects_raw_diff_export() {
        let mut record: ChangeObjectRecord =
            serde_json::from_str(FIXTURE_BRANCH).expect("branch fixture must parse");
        record.support_export.raw_diff_body_export_allowed = true;
        let err = record
            .validate()
            .expect_err("must reject raw diff export");
        assert!(err.message().contains("raw_"));
    }

    #[test]
    fn rejects_missing_change_object_inspector_consumer() {
        let mut record: ChangeObjectRecord =
            serde_json::from_str(FIXTURE_BRANCH).expect("branch fixture must parse");
        record
            .consumer_surfaces
            .retain(|surface| surface != "change_object_inspector");
        let err = record
            .validate()
            .expect_err("must reject consumer list without change_object_inspector");
        assert!(err.message().contains("change_object_inspector"));
    }
}
