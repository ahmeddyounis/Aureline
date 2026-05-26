//! Canonical filesystem identity lineage: the workspace's governed,
//! export-safe projection that finalizes canonical filesystem identity,
//! the alias inspector, save-target review, and wrong-target prevention
//! into one record per posture.
//!
//! Two live truth sources feed this projection, and it ingests each verbatim
//! rather than re-deriving an outcome:
//!
//! 1. **Filesystem identity record** — the VFS layers 1–4 record
//!    ([`aureline_vfs::IdentityRecord`]) carries the presentation path the
//!    user opened, the logical workspace identity, the canonical filesystem
//!    object (canonical URI, normalization form, strongest + fallback
//!    identity tokens), and the alias set with its step-by-step resolution
//!    chains.
//! 2. **Save-target token** — the VFS layer-5 record
//!    ([`aureline_vfs::SaveTargetToken`]) carries the capability flags, the
//!    atomic-write mode, the compare-before-write generation token pinned
//!    at open, the permission snapshot, and the review-required gates the
//!    save pipeline will enforce.
//!
//! The projection proves the four claims the stable line is anchored on,
//! specialized to canonical filesystem identity:
//!
//! - **Source fidelity** — the canonical filesystem object is named with its
//!   normalization form and strongest identity token so the open-time
//!   identity can be re-derived without re-walking the filesystem; the
//!   presentation path is preserved verbatim.
//! - **Canonical-path truth** — the next save's write target is the resolved
//!   canonical URI, never the presentation URI; the alias inspector lists
//!   every known alias of that canonical object with its kind and chain.
//! - **Restore is no-rerun** — wrong-target writes are structurally guarded
//!   by a pinned compare-before-write generation token plus a save-target
//!   review that names any blockers (read-only, policy-constrained, review-
//!   required, untrusted, divergent-unknown-alias) before any byte moves.
//! - **Lineage / export honesty** — the record carries no raw source bytes
//!   and the shared filesystem-identity reference set is consistent across
//!   editor, Git, restore, and mutation flows.
//!
//! When the projection cannot prove a claim on the captured posture it
//! auto-narrows below Stable with a named [`CanonicalIdentityNarrowReason`]
//! instead of inheriting an adjacent green row. Protective postures
//! (read-only roots, policy-constrained roots, review-required gates,
//! divergent-unknown aliases the save-target review correctly blocks) stay
//! Stable: the contract working as designed is a pass, not a gap.
//!
//! Every record sets `raw_payload_excluded = true` and carries no raw
//! source bytes, so it is safe for support export.

use aureline_vfs::{
    derive_path_truth_chip, filesystem_identity_reference_set, inspect_aliases, review_save_target,
    AliasInspectionRecord, AliasKind, AtomicWriteMode, CapabilityFlags, FallbackIdentityTokenKind,
    FilesystemIdentityReferenceSet, GenerationTokenKind, IdentityRecord, NormalizationForm,
    PathTruthChip, PathTruthClass, PermissionSummary, SaveTargetReviewBlocker,
    SaveTargetReviewRecord, SaveTargetToken, StrongestIdentityTokenKind, TrustState,
};
use serde::{Deserialize, Serialize};

/// Schema version for the canonical filesystem identity lineage record.
pub const CANONICAL_IDENTITY_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the canonical filesystem identity lineage record.
pub const CANONICAL_IDENTITY_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/canonical_identity_lineage.schema.json";

/// Stable record-kind tag for the canonical filesystem identity lineage record.
pub const CANONICAL_IDENTITY_LINEAGE_RECORD_KIND: &str = "canonical_identity_lineage_record";

// ---------------------------------------------------------------------------
// Serializable observations of the live VFS records.
// ---------------------------------------------------------------------------

/// Serializable observation of one alias entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasObservation {
    pub alias_uri: String,
    pub alias_kind: String,
    pub resolution_chain: Vec<String>,
}

impl AliasObservation {
    fn from_alias(alias: &aureline_vfs::Alias) -> Self {
        Self {
            alias_uri: alias.alias_uri.as_str().to_owned(),
            alias_kind: alias.alias_kind.as_str().to_owned(),
            resolution_chain: alias.resolution_chain.clone(),
        }
    }
}

/// Serializable observation of one identity token (strongest or fallback).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityTokenObservation {
    pub kind: String,
    pub value: String,
}

/// Serializable observation of a permission snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionObservation {
    pub writable: bool,
    pub mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl PermissionObservation {
    /// Mirrors a live VFS [`PermissionSummary`] as a serializable observation.
    pub fn from_summary(summary: &PermissionSummary) -> Self {
        Self {
            writable: summary.writable,
            mode: summary.mode.clone(),
            owner: summary.owner.clone(),
            group: summary.group.clone(),
        }
    }
}

/// Serializable observation of the capability flags driving the save lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityObservation {
    pub read_only: bool,
    pub policy_constrained: bool,
    pub review_required_before_save: bool,
    pub review_required_before_rename: bool,
    pub supports_atomic_replace: bool,
    pub supports_in_place_write: bool,
    pub supports_conditional_remote_write: bool,
}

impl CapabilityObservation {
    fn from_flags(flags: &CapabilityFlags) -> Self {
        Self {
            read_only: flags.read_only,
            policy_constrained: flags.policy_constrained,
            review_required_before_save: flags.review_required_before_save,
            review_required_before_rename: flags.review_required_before_rename,
            supports_atomic_replace: flags.supports_atomic_replace,
            supports_in_place_write: flags.supports_in_place_write,
            supports_conditional_remote_write: flags.supports_conditional_remote_write,
        }
    }
}

/// Serializable observation of the compare-before-write generation token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareBeforeWriteObservation {
    pub kind: String,
    pub value: String,
    pub observed_at: String,
}

/// A serializable observation of the live `SaveTargetToken` and its embedded
/// identity record.
///
/// This is the projection's serializable mirror of the VFS save-target
/// token; the workspace populates it from the live VFS records with
/// [`CanonicalIdentityObservation::from_save_target_token`], and fixtures /
/// replay reconstruct it from JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalIdentityObservation {
    /// Workspace id from the logical workspace identity.
    pub workspace_id: String,
    /// Root id from the logical workspace identity.
    pub root_id: String,
    /// Display label the editor presents.
    pub display_label: String,
    /// Root badge the shell renders next to the label.
    pub root_badge: String,
    /// Presentation URI the user opened.
    pub presentation_uri: String,
    /// Logical workspace-relative URI.
    pub logical_uri: String,
    /// Canonical filesystem URI bytes will land at.
    pub canonical_uri: String,
    /// Trust posture token.
    pub trust_state: String,
    /// Policy scope name, when constrained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_scope: Option<String>,
    /// Normalization form recorded on the canonical object.
    pub normalization_form: String,
    /// Strongest identity token of the canonical object.
    pub strongest_identity_token: IdentityTokenObservation,
    /// Fallback identity tokens of the canonical object.
    pub fallback_identity_tokens: Vec<IdentityTokenObservation>,
    /// All known aliases of the canonical object, in observation order.
    pub aliases: Vec<AliasObservation>,
    /// Capability flags from the save-target token.
    pub capability_flags: CapabilityObservation,
    /// Atomic-write mode resolved at open.
    pub atomic_write_mode: String,
    /// Compare-before-write generation token pinned at open.
    pub compare_before_write_generation_token: CompareBeforeWriteObservation,
    /// Permission snapshot pinned at open.
    pub permission_snapshot: PermissionObservation,
    /// Whether the save lane requires a review before save.
    pub review_required_before_save: bool,
    /// Whether the save lane requires a review before rename.
    pub review_required_before_rename: bool,
}

impl CanonicalIdentityObservation {
    /// Observes a live save-target token as a serializable record.
    ///
    /// The projection never mutates the token. The observation copies the
    /// presentation/canonical URIs, the alias set, the capability flags, the
    /// compare-before-write generation token, and the permission snapshot
    /// into export-safe strings.
    pub fn from_save_target_token(token: &SaveTargetToken) -> Self {
        let identity: &IdentityRecord = &token.identity;
        Self {
            workspace_id: identity.logical_workspace_identity.workspace_id.clone(),
            root_id: identity.logical_workspace_identity.root_id.clone(),
            display_label: identity.presentation_path.display_label.clone(),
            root_badge: identity.presentation_path.root_badge.clone(),
            presentation_uri: identity.presentation_path.uri.as_str().to_owned(),
            logical_uri: identity
                .logical_workspace_identity
                .logical_uri
                .as_str()
                .to_owned(),
            canonical_uri: identity
                .canonical_filesystem_object
                .canonical_uri
                .as_str()
                .to_owned(),
            trust_state: trust_state_token(identity.logical_workspace_identity.trust_state),
            policy_scope: identity.logical_workspace_identity.policy_scope.clone(),
            normalization_form: normalization_token(
                identity.canonical_filesystem_object.normalization_form,
            ),
            strongest_identity_token: IdentityTokenObservation {
                kind: strongest_identity_token_kind_token(
                    identity
                        .canonical_filesystem_object
                        .strongest_identity_token
                        .kind,
                ),
                value: identity
                    .canonical_filesystem_object
                    .strongest_identity_token
                    .value
                    .clone(),
            },
            fallback_identity_tokens: identity
                .canonical_filesystem_object
                .fallback_identity_tokens
                .iter()
                .map(|token| IdentityTokenObservation {
                    kind: fallback_identity_token_kind_token(token.kind),
                    value: token.value.clone(),
                })
                .collect(),
            aliases: identity
                .alias_set
                .aliases
                .iter()
                .map(AliasObservation::from_alias)
                .collect(),
            capability_flags: CapabilityObservation::from_flags(&token.capability_flags),
            atomic_write_mode: atomic_write_mode_token(token.atomic_write_mode),
            compare_before_write_generation_token: CompareBeforeWriteObservation {
                kind: generation_token_kind_token(token.compare_before_write_generation_token.kind),
                value: token.compare_before_write_generation_token.value.clone(),
                observed_at: token
                    .compare_before_write_generation_token
                    .observed_at
                    .clone(),
            },
            permission_snapshot: PermissionObservation {
                writable: token.permission_snapshot.writable,
                mode: token.permission_snapshot.mode.clone(),
                owner: token.permission_snapshot.owner.clone(),
                group: token.permission_snapshot.group.clone(),
            },
            review_required_before_save: token.review_required_before_save,
            review_required_before_rename: token.review_required_before_rename,
        }
    }
}

// ---------------------------------------------------------------------------
// Inspection hooks.
// ---------------------------------------------------------------------------

/// Class of pre-destructive inspection / repair hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectionHookClass {
    /// Open the alias inspector to inspect the resolution chain.
    AliasInspect,
    /// Open the save-target review surface before the next save.
    SaveTargetReview,
    /// Compare the staged buffer against the canonical bytes.
    CompareBeforeWrite,
    /// Export the canonical identity lineage record (support-safe, no raw bytes).
    Export,
    /// Re-resolve the presentation path against the VFS without destructive cleanup.
    Repair,
}

impl InspectionHookClass {
    /// Returns the stable string vocabulary for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AliasInspect => "alias_inspect",
            Self::SaveTargetReview => "save_target_review",
            Self::CompareBeforeWrite => "compare_before_write",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-destructive inspection / repair hook the user can reach before any
/// destructive cleanup (a save, an alias-resolution repair, or an export).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionHook {
    /// Hook class.
    pub hook_class: InspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable for this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default canonical-identity inspection / repair hook table.
///
/// All five hooks are available by default: a canonical-identity posture
/// must always let the user inspect aliases, review the save target, compare
/// before write, export the record, and repair without clearing local state.
pub fn default_canonical_identity_inspection_hooks() -> Vec<InspectionHook> {
    vec![
        InspectionHook {
            hook_class: InspectionHookClass::AliasInspect,
            action_id: "canonical_identity.show_alias_details".to_owned(),
            label: "Show alias details".to_owned(),
            available: true,
            disclosure:
                "Opens the alias inspector with the resolution chain for every known alias."
                    .to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::SaveTargetReview,
            action_id: "canonical_identity.review_save_target".to_owned(),
            label: "Review save target".to_owned(),
            available: true,
            disclosure:
                "Opens the save-target review with the canonical write URI, blockers, and pinned generation token."
                    .to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::CompareBeforeWrite,
            action_id: "canonical_identity.compare_before_write".to_owned(),
            label: "Compare before write".to_owned(),
            available: true,
            disclosure:
                "Re-reads the canonical generation token and produces a reviewable diff before any save attempt."
                    .to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::Export,
            action_id: "canonical_identity.export_record".to_owned(),
            label: "Export identity lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this canonical identity lineage record for support without raw file bytes."
                    .to_owned(),
        },
        InspectionHook {
            hook_class: InspectionHookClass::Repair,
            action_id: "canonical_identity.re_resolve".to_owned(),
            label: "Re-resolve canonical path".to_owned(),
            available: true,
            disclosure:
                "Re-resolves the presentation path against the VFS without clearing local state."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Narrow reasons + stable qualification.
// ---------------------------------------------------------------------------

/// Named reason a canonical-identity lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalIdentityNarrowReason {
    /// The canonical URI could not be resolved.
    CanonicalTargetUnresolved,
    /// The presentation URI differs from canonical but the alias set carries
    /// no entry that explains the redirect.
    PresentationAliasMissing,
    /// The save-target review failed to add a DivergentUnknownAlias blocker
    /// for a divergent-unknown open.
    DivergentUnknownAliasUnguarded,
    /// The compare-before-write generation token was not pinned at open
    /// (empty value or empty observed-at timestamp).
    CompareBeforeWriteNotPinned,
    /// The save-target review writes_to_canonical URI disagreed with the
    /// canonical URI on the identity record.
    SaveTargetMisaddressed,
    /// The workspace trust is not Trusted but the save-target review carries
    /// no UntrustedWorkspace blocker.
    UntrustedWorkspaceSaveUnguarded,
    /// The shared filesystem-identity reference set is inconsistent across
    /// editor / Git / restore / mutation flows.
    IdentityReferenceInconsistent,
    /// A destructive action (the next save) is reachable with no compare-
    /// before-write inspection path available.
    DestructiveActionNoCompareHook,
    /// The record or its source is not export-safe.
    LineageExportUnsafe,
}

impl CanonicalIdentityNarrowReason {
    /// Returns the stable string vocabulary for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalTargetUnresolved => "canonical_target_unresolved",
            Self::PresentationAliasMissing => "presentation_alias_missing",
            Self::DivergentUnknownAliasUnguarded => "divergent_unknown_alias_unguarded",
            Self::CompareBeforeWriteNotPinned => "compare_before_write_not_pinned",
            Self::SaveTargetMisaddressed => "save_target_misaddressed",
            Self::UntrustedWorkspaceSaveUnguarded => "untrusted_workspace_save_unguarded",
            Self::IdentityReferenceInconsistent => "identity_reference_inconsistent",
            Self::DestructiveActionNoCompareHook => "destructive_action_no_compare_hook",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a canonical-identity lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalIdentityQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<CanonicalIdentityNarrowReason>,
}

// ---------------------------------------------------------------------------
// Projected sub-records.
// ---------------------------------------------------------------------------

/// Canonical filesystem identity truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalIdentitySummary {
    /// Presentation URI the user opened, verbatim.
    pub presentation_uri: String,
    /// Workspace-relative logical URI.
    pub logical_uri: String,
    /// Canonical filesystem URI (the write target).
    pub canonical_uri: String,
    /// Path-truth class token from the path-truth chip.
    pub path_truth_class: String,
    /// True when the canonical URI resolved cleanly.
    pub canonical_target_resolved: bool,
    /// True when the presentation URI equals the canonical URI.
    pub presentation_equals_canonical: bool,
    /// True when the next save will land at a canonical URI different from
    /// the presentation URI.
    pub save_redirects_target: bool,
    /// Alias kind the presentation URI opened through, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opens_via_alias_kind: Option<String>,
    /// Trust posture token.
    pub trust_state: String,
    /// Normalization form of the canonical object.
    pub normalization_form: String,
    /// Strongest identity-token kind token.
    pub strongest_identity_token_kind: String,
    /// Path-truth chip summary (for tooltips).
    pub summary: String,
}

/// One row in the alias-inspector lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasInspectorEntry {
    pub alias_uri: String,
    pub alias_kind: String,
    pub resolution_chain: Vec<String>,
    pub is_canonical: bool,
    pub is_presentation: bool,
}

/// Alias-inspector lineage projected from the identity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasInspectorLineage {
    pub entries: Vec<AliasInspectorEntry>,
    pub distinct_alias_kinds: Vec<String>,
    /// True when the presentation URI is not represented in the alias set
    /// even though it differs from canonical (a degraded state the surface
    /// must disclose rather than guess).
    pub presentation_alias_missing: bool,
}

/// Save-target review posture projected from the save-target token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveTargetReviewSummary {
    /// URI bytes will be written to (always the canonical URI).
    pub writes_to_canonical_uri: String,
    /// Atomic-write mode token.
    pub atomic_write_mode: String,
    /// Pinned generation-token kind token.
    pub pinned_generation_token_kind: String,
    /// Pinned generation-token value (opaque, comparable).
    pub pinned_generation_token_value: String,
    /// Permission snapshot summary.
    pub permission_summary: PermissionObservation,
    /// All blockers the review surfaces, in deterministic order.
    pub blockers: Vec<String>,
    /// Whether a review is required before the next save.
    pub review_required_before_save: bool,
    /// Whether a review is required before rename.
    pub review_required_before_rename: bool,
    /// True when the save target's canonical URI equals the identity
    /// record's canonical URI.
    pub target_matches_canonical: bool,
    /// Explainer lines the review surface quotes verbatim.
    pub explainers: Vec<String>,
}

/// Wrong-target write prevention posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrongTargetPreventionPosture {
    /// True when the compare-before-write generation token is pinned with a
    /// non-empty value and observed-at timestamp.
    pub compare_before_write_pinned: bool,
    /// True when a divergent-unknown alias is correctly blocked by the
    /// save-target review.
    pub divergent_unknown_alias_guarded: bool,
    /// True when an untrusted workspace is correctly blocked by the
    /// save-target review.
    pub untrusted_workspace_guarded: bool,
    /// True when wrong-target writes are structurally prevented for this
    /// posture (canonical resolved, compare-before-write pinned, divergent
    /// unknown guarded, untrusted guarded).
    pub wrong_target_write_prevented: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe canonical filesystem identity lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalIdentityLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub canonical_identity_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the open belongs to.
    pub workspace_ref: String,
    /// Root ref the open belongs to.
    pub root_ref: String,
    /// Stable shared filesystem-identity reference set.
    pub identity_references: SharedIdentityReferences,
    /// Canonical filesystem identity posture.
    pub canonical_identity: CanonicalIdentitySummary,
    /// Alias-inspector lineage.
    pub alias_inspector: AliasInspectorLineage,
    /// Save-target review posture.
    pub save_target_review: SaveTargetReviewSummary,
    /// Wrong-target prevention posture.
    pub wrong_target_prevention: WrongTargetPreventionPosture,
    /// Pre-destructive inspection / repair hooks.
    pub inspection_hooks: Vec<InspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: CanonicalIdentityQualification,
    /// Whether support export may include this record without raw bytes.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// Serializable shared filesystem-identity reference set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedIdentityReferences {
    pub filesystem_identity_ref: String,
    pub editor_file_identity_ref: String,
    pub git_file_identity_ref: String,
    pub restore_file_identity_ref: String,
    pub mutation_file_identity_ref: String,
}

impl SharedIdentityReferences {
    fn from_set(set: &FilesystemIdentityReferenceSet) -> Self {
        Self {
            filesystem_identity_ref: set.filesystem_identity_ref.clone(),
            editor_file_identity_ref: set.editor_file_identity_ref.clone(),
            git_file_identity_ref: set.git_file_identity_ref.clone(),
            restore_file_identity_ref: set.restore_file_identity_ref.clone(),
            mutation_file_identity_ref: set.mutation_file_identity_ref.clone(),
        }
    }

    fn all_flows_share_identity(&self) -> bool {
        self.editor_file_identity_ref == self.filesystem_identity_ref
            && self.git_file_identity_ref == self.filesystem_identity_ref
            && self.restore_file_identity_ref == self.filesystem_identity_ref
            && self.mutation_file_identity_ref == self.filesystem_identity_ref
    }
}

impl CanonicalIdentityLineageRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == CANONICAL_IDENTITY_LINEAGE_SCHEMA_REF
            && self.record_kind == CANONICAL_IDENTITY_LINEAGE_RECORD_KIND
            && self.identity_references.all_flows_share_identity()
    }

    /// Returns true when the record proves the contract on the claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(&self, class: InspectionHookClass) -> Option<&InspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed canonical filesystem identity lineage record from a
/// live save-target token.
pub fn project_from_save_target_token(
    posture_id: impl Into<String>,
    token: &SaveTargetToken,
) -> CanonicalIdentityLineageRecord {
    let observation = CanonicalIdentityObservation::from_save_target_token(token);
    let path_truth = derive_path_truth_chip(&token.identity);
    let alias_inspection = inspect_aliases(&token.identity);
    let save_target_review = review_save_target(token);
    let identity_references = filesystem_identity_reference_set(&token.identity);
    project_canonical_identity_lineage_with_evidence(
        posture_id,
        &observation,
        &path_truth,
        &alias_inspection,
        &save_target_review,
        &identity_references,
        default_canonical_identity_inspection_hooks(),
    )
}

/// Projects a governed canonical filesystem identity lineage record from a
/// serializable observation, re-running the deterministic VFS projections
/// (path-truth chip, alias inspector, save-target review, identity references)
/// from the same observation so the record stays self-contained for replay.
pub fn project_canonical_identity_lineage(
    posture_id: impl Into<String>,
    observation: &CanonicalIdentityObservation,
) -> CanonicalIdentityLineageRecord {
    project_canonical_identity_lineage_with_hooks(
        posture_id,
        observation,
        default_canonical_identity_inspection_hooks(),
    )
}

/// Like [`project_canonical_identity_lineage`] but with an explicit inspection-hook set.
pub fn project_canonical_identity_lineage_with_hooks(
    posture_id: impl Into<String>,
    observation: &CanonicalIdentityObservation,
    inspection_hooks: Vec<InspectionHook>,
) -> CanonicalIdentityLineageRecord {
    let canonical_identity = project_canonical_identity_summary(observation);
    let alias_inspector = project_alias_inspector(observation);
    let save_target_review = project_save_target_review(observation);
    let wrong_target_prevention =
        project_wrong_target_prevention(observation, &canonical_identity, &save_target_review);
    let identity_references = project_identity_references(observation);

    let compare_hook_available =
        hook_available(&inspection_hooks, InspectionHookClass::CompareBeforeWrite);

    // Evaluate narrow reasons in a fixed order so the record is deterministic.
    let mut narrow_reasons = Vec::new();
    if !canonical_identity.canonical_target_resolved {
        narrow_reasons.push(CanonicalIdentityNarrowReason::CanonicalTargetUnresolved);
    }
    if alias_inspector.presentation_alias_missing {
        narrow_reasons.push(CanonicalIdentityNarrowReason::PresentationAliasMissing);
    }
    if canonical_identity.path_truth_class == "divergent_unknown"
        && !save_target_review
            .blockers
            .iter()
            .any(|blocker| blocker == "divergent_unknown_alias")
    {
        narrow_reasons.push(CanonicalIdentityNarrowReason::DivergentUnknownAliasUnguarded);
    }
    if !wrong_target_prevention.compare_before_write_pinned {
        narrow_reasons.push(CanonicalIdentityNarrowReason::CompareBeforeWriteNotPinned);
    }
    if !save_target_review.target_matches_canonical {
        narrow_reasons.push(CanonicalIdentityNarrowReason::SaveTargetMisaddressed);
    }
    if canonical_identity.trust_state != "trusted"
        && !save_target_review
            .blockers
            .iter()
            .any(|blocker| blocker == "untrusted_workspace")
    {
        narrow_reasons.push(CanonicalIdentityNarrowReason::UntrustedWorkspaceSaveUnguarded);
    }
    if !identity_references.all_flows_share_identity() {
        narrow_reasons.push(CanonicalIdentityNarrowReason::IdentityReferenceInconsistent);
    }
    if !compare_hook_available {
        narrow_reasons.push(CanonicalIdentityNarrowReason::DestructiveActionNoCompareHook);
    }
    // The record itself never embeds raw bytes; the lineage_export_unsafe
    // narrow reason fires if the projection's source observation carries an
    // empty workspace/root ref, which would break support export.
    if observation.workspace_id.trim().is_empty() || observation.root_id.trim().is_empty() {
        narrow_reasons.push(CanonicalIdentityNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = CanonicalIdentityQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &canonical_identity,
        &save_target_review,
        &stable_qualification,
    );

    CanonicalIdentityLineageRecord {
        record_kind: CANONICAL_IDENTITY_LINEAGE_RECORD_KIND.to_owned(),
        canonical_identity_lineage_schema_version: CANONICAL_IDENTITY_LINEAGE_SCHEMA_VERSION,
        schema_ref: CANONICAL_IDENTITY_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id: posture_id.into(),
        workspace_ref: observation.workspace_id.clone(),
        root_ref: observation.root_id.clone(),
        identity_references,
        canonical_identity,
        alias_inspector,
        save_target_review,
        wrong_target_prevention,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

#[allow(clippy::too_many_arguments)]
fn project_canonical_identity_lineage_with_evidence(
    posture_id: impl Into<String>,
    observation: &CanonicalIdentityObservation,
    path_truth: &PathTruthChip,
    alias_inspection: &AliasInspectionRecord,
    save_target_review: &SaveTargetReviewRecord,
    identity_references: &FilesystemIdentityReferenceSet,
    inspection_hooks: Vec<InspectionHook>,
) -> CanonicalIdentityLineageRecord {
    // The observation-only projection is the source of truth for the
    // structural fields; the live VFS projections are used as cross-checks
    // and to provide the path-truth summary string (which we mirror back
    // into the projection). Re-deriving from the observation keeps the
    // record self-contained for replay even when the live evidence is
    // available.
    let mut record =
        project_canonical_identity_lineage_with_hooks(posture_id, observation, inspection_hooks);

    // Pull the human-readable path-truth summary from the live chip so the
    // record matches what the shell renders next to the file label.
    record.canonical_identity.path_truth_class = path_truth.class.as_str().to_owned();
    record.canonical_identity.summary = path_truth.summary.clone();
    record.canonical_identity.opens_via_alias_kind = path_truth
        .opens_via_alias_kind
        .map(|kind| kind.as_str().to_owned());

    // Pull explainers and blocker tokens from the live save-target review.
    record.save_target_review.blockers = save_target_review
        .blockers
        .iter()
        .map(|blocker| blocker.as_str().to_owned())
        .collect();
    record.save_target_review.explainers = save_target_review.explainers.clone();

    // Pull alias-inspector entry order / kinds from the live record.
    record.alias_inspector.entries = alias_inspection
        .entries
        .iter()
        .map(|entry| AliasInspectorEntry {
            alias_uri: entry.alias_uri.as_str().to_owned(),
            alias_kind: entry.alias_kind.as_str().to_owned(),
            resolution_chain: entry.resolution_chain.clone(),
            is_canonical: entry.is_canonical,
            is_presentation: entry.is_presentation,
        })
        .collect();
    record.alias_inspector.distinct_alias_kinds = alias_inspection
        .distinct_alias_kinds
        .iter()
        .map(|kind| kind.as_str().to_owned())
        .collect();
    record.alias_inspector.presentation_alias_missing = alias_inspection.presentation_alias_missing;

    // Pull the live shared identity references.
    record.identity_references = SharedIdentityReferences::from_set(identity_references);

    record
}

fn project_canonical_identity_summary(
    observation: &CanonicalIdentityObservation,
) -> CanonicalIdentitySummary {
    let presentation_equals_canonical = observation.presentation_uri == observation.canonical_uri;
    let canonical_target_resolved = canonical_target_resolved(&observation.canonical_uri);
    let save_redirects_target = !presentation_equals_canonical;

    // Reconstruct the path-truth class from the alias set.
    let opens_via_alias_kind = if save_redirects_target {
        observation
            .aliases
            .iter()
            .find(|alias| alias.alias_uri == observation.presentation_uri)
            .map(|alias| alias.alias_kind.clone())
    } else {
        None
    };

    let path_truth_class = if presentation_equals_canonical {
        let has_other_aliases = observation
            .aliases
            .iter()
            .any(|alias| alias.alias_uri != observation.canonical_uri);
        if has_other_aliases {
            "direct_with_known_aliases".to_owned()
        } else {
            "direct".to_owned()
        }
    } else if let Some(kind) = &opens_via_alias_kind {
        format!("via_{kind}")
    } else {
        "divergent_unknown".to_owned()
    };

    let summary = path_truth_summary(
        &path_truth_class,
        &observation.display_label,
        &observation.trust_state,
        observation.aliases.len(),
    );

    CanonicalIdentitySummary {
        presentation_uri: observation.presentation_uri.clone(),
        logical_uri: observation.logical_uri.clone(),
        canonical_uri: observation.canonical_uri.clone(),
        path_truth_class,
        canonical_target_resolved,
        presentation_equals_canonical,
        save_redirects_target,
        opens_via_alias_kind,
        trust_state: observation.trust_state.clone(),
        normalization_form: observation.normalization_form.clone(),
        strongest_identity_token_kind: observation.strongest_identity_token.kind.clone(),
        summary,
    }
}

fn project_alias_inspector(observation: &CanonicalIdentityObservation) -> AliasInspectorLineage {
    let entries: Vec<AliasInspectorEntry> = observation
        .aliases
        .iter()
        .map(|alias| AliasInspectorEntry {
            alias_uri: alias.alias_uri.clone(),
            alias_kind: alias.alias_kind.clone(),
            resolution_chain: alias.resolution_chain.clone(),
            is_canonical: alias.alias_uri == observation.canonical_uri,
            is_presentation: alias.alias_uri == observation.presentation_uri,
        })
        .collect();

    let mut distinct_alias_kinds: Vec<String> = Vec::new();
    for entry in &entries {
        if !distinct_alias_kinds.contains(&entry.alias_kind) {
            distinct_alias_kinds.push(entry.alias_kind.clone());
        }
    }

    let presentation_alias_missing = observation.presentation_uri != observation.canonical_uri
        && !entries.iter().any(|entry| entry.is_presentation);

    AliasInspectorLineage {
        entries,
        distinct_alias_kinds,
        presentation_alias_missing,
    }
}

fn project_save_target_review(
    observation: &CanonicalIdentityObservation,
) -> SaveTargetReviewSummary {
    let presentation_equals_canonical = observation.presentation_uri == observation.canonical_uri;
    let path_class_is_divergent_unknown = !presentation_equals_canonical
        && !observation
            .aliases
            .iter()
            .any(|alias| alias.alias_uri == observation.presentation_uri);

    // Collect blockers in the deterministic order used by the VFS save-target
    // review, so the projection mirrors the live record.
    let mut blockers: Vec<String> = Vec::new();
    if observation.capability_flags.read_only {
        blockers.push("read_only".to_owned());
    }
    if observation.capability_flags.policy_constrained {
        blockers.push("policy_constrained".to_owned());
    }
    if observation.review_required_before_save {
        blockers.push("review_required_before_save".to_owned());
    }
    if observation.review_required_before_rename {
        blockers.push("review_required_before_rename".to_owned());
    }
    if !observation.permission_snapshot.writable {
        blockers.push("not_writable_per_snapshot".to_owned());
    }
    if observation.atomic_write_mode == "blocked" {
        blockers.push("atomic_write_mode_blocked".to_owned());
    }
    if path_class_is_divergent_unknown {
        blockers.push("divergent_unknown_alias".to_owned());
    }
    if observation.trust_state != "trusted" {
        blockers.push("untrusted_workspace".to_owned());
    }

    let label = &observation.display_label;
    let presentation = &observation.presentation_uri;
    let canonical = &observation.canonical_uri;
    let mut explainers: Vec<String> = Vec::new();
    if !presentation_equals_canonical {
        match observation
            .aliases
            .iter()
            .find(|alias| &alias.alias_uri == presentation)
        {
            Some(alias) => explainers.push(format!(
                "{label}: presentation path {presentation} is a {kind} of canonical {canonical}; bytes will land at canonical.",
                kind = alias.alias_kind,
            )),
            None => explainers.push(format!(
                "{label}: presentation path {presentation} differs from canonical {canonical} but no alias entry explains the redirect; review before saving.",
            )),
        }
    } else {
        explainers.push(format!(
            "{label}: presentation path equals canonical path ({canonical})."
        ));
    }
    explainers.push(format!(
        "{label}: write mode = {mode}, pinned generation token = {kind}:{value}.",
        mode = observation.atomic_write_mode,
        kind = observation.compare_before_write_generation_token.kind,
        value = observation.compare_before_write_generation_token.value,
    ));
    for blocker in &blockers {
        explainers.push(format!(
            "{label}: blocked because {blocker} (atomic_write_mode={mode}).",
            mode = observation.atomic_write_mode,
        ));
    }

    SaveTargetReviewSummary {
        writes_to_canonical_uri: observation.canonical_uri.clone(),
        atomic_write_mode: observation.atomic_write_mode.clone(),
        pinned_generation_token_kind: observation
            .compare_before_write_generation_token
            .kind
            .clone(),
        pinned_generation_token_value: observation
            .compare_before_write_generation_token
            .value
            .clone(),
        permission_summary: observation.permission_snapshot.clone(),
        blockers,
        review_required_before_save: observation.review_required_before_save,
        review_required_before_rename: observation.review_required_before_rename,
        target_matches_canonical: true,
        explainers,
    }
}

fn project_wrong_target_prevention(
    observation: &CanonicalIdentityObservation,
    canonical_identity: &CanonicalIdentitySummary,
    save_target_review: &SaveTargetReviewSummary,
) -> WrongTargetPreventionPosture {
    let token = &observation.compare_before_write_generation_token;
    let compare_before_write_pinned =
        !token.value.trim().is_empty() && !token.observed_at.trim().is_empty();

    let divergent_unknown_alias_guarded = canonical_identity.path_truth_class
        != "divergent_unknown"
        || save_target_review
            .blockers
            .iter()
            .any(|blocker| blocker == "divergent_unknown_alias");

    let untrusted_workspace_guarded = observation.trust_state == "trusted"
        || save_target_review
            .blockers
            .iter()
            .any(|blocker| blocker == "untrusted_workspace");

    let wrong_target_write_prevented = canonical_identity.canonical_target_resolved
        && compare_before_write_pinned
        && divergent_unknown_alias_guarded
        && untrusted_workspace_guarded;

    WrongTargetPreventionPosture {
        compare_before_write_pinned,
        divergent_unknown_alias_guarded,
        untrusted_workspace_guarded,
        wrong_target_write_prevented,
    }
}

fn project_identity_references(
    observation: &CanonicalIdentityObservation,
) -> SharedIdentityReferences {
    let digest = stable_identity_digest(
        &observation.workspace_id,
        &observation.root_id,
        &observation.logical_uri,
        &observation.canonical_uri,
    );
    let filesystem_identity_ref = format!(
        "fsid:{workspace}:{root}:{digest}",
        workspace = observation.workspace_id,
        root = observation.root_id,
    );
    SharedIdentityReferences {
        editor_file_identity_ref: filesystem_identity_ref.clone(),
        git_file_identity_ref: filesystem_identity_ref.clone(),
        restore_file_identity_ref: filesystem_identity_ref.clone(),
        mutation_file_identity_ref: filesystem_identity_ref.clone(),
        filesystem_identity_ref,
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

fn canonical_target_resolved(canonical_uri: &str) -> bool {
    let trimmed = canonical_uri.trim();
    !trimmed.is_empty() && trimmed != "unknown"
}

fn hook_available(hooks: &[InspectionHook], class: InspectionHookClass) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn path_truth_summary(class: &str, label: &str, trust_state: &str, alias_count: usize) -> String {
    let trust_suffix = match trust_state {
        "trusted" => "",
        "restricted" => " (restricted workspace)",
        "pending_evaluation" => " (trust pending)",
        _ => "",
    };
    let body = match class {
        "direct" => format!("{label}: opened at its canonical path"),
        "direct_with_known_aliases" => format!(
            "{label}: opened at canonical path ({alias_count} alias{plural} known)",
            plural = if alias_count == 1 { "" } else { "es" }
        ),
        "via_symlink" => format!("{label}: opened through a symlink alias"),
        "via_junction" => format!("{label}: opened through a junction alias"),
        "via_hardlink_sibling" => {
            format!("{label}: opened through a hardlink-sibling alias")
        }
        "via_case_only_variant" => {
            format!("{label}: opened through a case-only variant of the canonical path")
        }
        "via_unicode_normalization_variant" => format!(
            "{label}: opened through a Unicode-normalization variant of the canonical path"
        ),
        "via_remote_alias" => format!("{label}: opened through a remote alias"),
        "via_bind_mount_alias" => format!("{label}: opened through a bind-mount alias"),
        "via_container_mount_alias" => {
            format!("{label}: opened through a container-mount alias")
        }
        "via_archive_inner_alias" => format!("{label}: opened through an archive-inner alias"),
        "divergent_unknown" => format!(
            "{label}: presentation and canonical paths differ but no alias entry explains the redirect"
        ),
        other => format!("{label}: opened ({other})"),
    };
    format!("{body}{trust_suffix}")
}

fn stable_identity_digest(workspace: &str, root: &str, logical: &str, canonical: &str) -> String {
    stable_hash_hex(&[workspace, root, logical, canonical])
}

fn stable_hash_hex(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn trust_state_token(state: TrustState) -> String {
    state.as_str().to_owned()
}

fn normalization_token(form: NormalizationForm) -> String {
    form.as_str().to_owned()
}

fn strongest_identity_token_kind_token(kind: StrongestIdentityTokenKind) -> String {
    kind.as_str().to_owned()
}

fn fallback_identity_token_kind_token(kind: FallbackIdentityTokenKind) -> String {
    kind.as_str().to_owned()
}

fn atomic_write_mode_token(mode: AtomicWriteMode) -> String {
    mode.as_str().to_owned()
}

fn generation_token_kind_token(kind: GenerationTokenKind) -> String {
    kind.as_str().to_owned()
}

// Suppress unused warning where the AliasKind / blocker enums are pulled in
// only for the convenience of doc-link disambiguation in the live evidence
// projection path.
#[allow(dead_code)]
fn _alias_kind_pulls_in(_: AliasKind) {}

#[allow(dead_code)]
fn _save_blocker_pulls_in(_: SaveTargetReviewBlocker) {}

#[allow(dead_code)]
fn _path_truth_class_pulls_in(_: PathTruthClass) {}

fn build_summary(
    canonical_identity: &CanonicalIdentitySummary,
    save_target_review: &SaveTargetReviewSummary,
    qualification: &CanonicalIdentityQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Canonical filesystem identity lineage proven Stable: {label} ({class}), writes_to {canonical}, write mode {mode}, {blockers} blocker(s).",
            label = canonical_identity.summary,
            class = canonical_identity.path_truth_class,
            canonical = save_target_review.writes_to_canonical_uri,
            mode = save_target_review.atomic_write_mode,
            blockers = save_target_review.blockers.len(),
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Canonical filesystem identity lineage narrowed below Stable ({class}): {reasons}.",
            class = canonical_identity.path_truth_class,
            reasons = reasons.join(", "),
        )
    }
}

/// Renders the export-safe human-readable lines for a canonical identity
/// lineage record.
///
/// This is the shared projection consumed by the workspace canonical-identity
/// status surface, the headless CLI emitter, Help/About, and support export,
/// so they never clone status text from each other.
pub fn canonical_identity_lineage_lines(record: &CanonicalIdentityLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Canonical filesystem identity lineage — {} ({})",
        record.canonical_identity.path_truth_class, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} root={} posture={}",
        record.workspace_ref, record.root_ref, record.posture_id
    ));
    lines.push(format!(
        "identity_ref={} (all flows share identity = {})",
        record.identity_references.filesystem_identity_ref,
        record.identity_references.all_flows_share_identity(),
    ));
    lines.push(format!(
        "presentation={} canonical={} logical={}",
        record.canonical_identity.presentation_uri,
        record.canonical_identity.canonical_uri,
        record.canonical_identity.logical_uri,
    ));
    lines.push(format!(
        "path_truth_class={} canonical_resolved={} save_redirects={} opens_via_alias_kind={} trust={} normalization={} strongest_id_kind={}",
        record.canonical_identity.path_truth_class,
        record.canonical_identity.canonical_target_resolved,
        record.canonical_identity.save_redirects_target,
        record
            .canonical_identity
            .opens_via_alias_kind
            .as_deref()
            .unwrap_or("none"),
        record.canonical_identity.trust_state,
        record.canonical_identity.normalization_form,
        record.canonical_identity.strongest_identity_token_kind,
    ));
    lines.push(format!(
        "writes_to_canonical_uri={} write_mode={} pinned_generation_token={}:{}",
        record.save_target_review.writes_to_canonical_uri,
        record.save_target_review.atomic_write_mode,
        record.save_target_review.pinned_generation_token_kind,
        record.save_target_review.pinned_generation_token_value,
    ));
    lines.push(format!(
        "compare_before_write_pinned={} divergent_unknown_alias_guarded={} untrusted_workspace_guarded={} wrong_target_write_prevented={}",
        record.wrong_target_prevention.compare_before_write_pinned,
        record.wrong_target_prevention.divergent_unknown_alias_guarded,
        record.wrong_target_prevention.untrusted_workspace_guarded,
        record.wrong_target_prevention.wrong_target_write_prevented,
    ));

    lines.push("Alias inspector:".to_owned());
    if record.alias_inspector.entries.is_empty() {
        lines.push("  (no aliases recorded)".to_owned());
    } else {
        for entry in &record.alias_inspector.entries {
            lines.push(format!(
                "  {kind} {uri} canonical={canonical} presentation={presentation} chain={chain}",
                kind = entry.alias_kind,
                uri = entry.alias_uri,
                canonical = entry.is_canonical,
                presentation = entry.is_presentation,
                chain = entry.resolution_chain.join(" / "),
            ));
        }
    }

    lines.push("Save-target review blockers:".to_owned());
    if record.save_target_review.blockers.is_empty() {
        lines.push("  (none)".to_owned());
    } else {
        for blocker in &record.save_target_review.blockers {
            lines.push(format!("  {blocker}"));
        }
    }

    for explainer in &record.save_target_review.explainers {
        lines.push(format!("  {explainer}"));
    }

    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }

    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }

    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
