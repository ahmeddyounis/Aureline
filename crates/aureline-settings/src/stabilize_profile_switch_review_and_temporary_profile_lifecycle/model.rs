//! Profile-switch review, temporary-profile lifecycle, and rollback-safety
//! certification model.
//!
//! This module is the settings-lane contract for profile switching. It defines
//! the inspectable record that UI, shell, sync, help, support export, and file
//! portability surfaces must consume before a claimed stable profile lane
//! switches profiles, imports profile state, applies synced state, or promotes a
//! temporary profile.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for profile-switch lifecycle records.
pub const PROFILE_SWITCH_REVIEW_RECORD_KIND: &str =
    "profile_switch_review_lifecycle_certification_record";

/// Schema version for [`ProfileSwitchLifecycleCertification`] records.
pub const PROFILE_SWITCH_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, shell, sync, help, support, and export surfaces.
pub const PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF: &str =
    "settings:profile_switch_review_lifecycle:v1";

const CANONICAL_OBJECT_SCHEME: &str = "aureline://";
const MAX_REF_CHARS: usize = 240;

/// Returns true when `reference` is a non-empty canonical object ref.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    !class.is_empty() && !ident.is_empty()
}

/// Public claim class derived from certification evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The lane satisfies every stable profile-switch gate.
    Stable,
    /// The lane is usable but narrowed below the stable promise.
    Beta,
    /// The lane is preview-only.
    Preview,
    /// No public claim is made.
    NotClaimed,
}

/// High-level source class of a profile card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileSourceClass {
    /// User-authored durable profile.
    DurableUserProfile,
    /// Imported profile artifact.
    ImportedProfile,
    /// Optional synced profile state.
    SyncedProfile,
    /// Short-lived temporary profile.
    TemporaryProfile,
    /// Troubleshooting profile.
    TroubleshootingProfile,
    /// Admin-shaped or policy-owned profile defaults.
    PolicyShapedProfile,
}

impl ProfileSourceClass {
    /// Returns the canonical token for this profile source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableUserProfile => "durable_user_profile",
            Self::ImportedProfile => "imported_profile",
            Self::SyncedProfile => "synced_profile",
            Self::TemporaryProfile => "temporary_profile",
            Self::TroubleshootingProfile => "troubleshooting_profile",
            Self::PolicyShapedProfile => "policy_shaped_profile",
        }
    }
}

/// Included scope shown on a profile card or review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileScopeClass {
    /// Theme and design-token preferences.
    VisualAppearance,
    /// Keybindings and command-surface defaults.
    Keybindings,
    /// Snippets and reusable text templates.
    Snippets,
    /// Extension selection and extension lock references.
    ExtensionSelection,
    /// Layout and panel preferences.
    Layout,
    /// Task and launch presets.
    TasksAndLaunch,
    /// AI preset references and egress posture.
    AiAndNetworkDefaults,
}

impl ProfileScopeClass {
    /// Returns the canonical token for this profile scope class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisualAppearance => "visual_appearance",
            Self::Keybindings => "keybindings",
            Self::Snippets => "snippets",
            Self::ExtensionSelection => "extension_selection",
            Self::Layout => "layout",
            Self::TasksAndLaunch => "tasks_and_launch",
            Self::AiAndNetworkDefaults => "ai_and_network_defaults",
        }
    }
}

/// Durability state shown on profile cards and temporary-profile badges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileDurabilityClass {
    /// Writes go to a durable profile artifact after review.
    Durable,
    /// State is held for this session only.
    SessionOnly,
    /// State is discarded on exit unless promoted.
    DiscardedOnExit,
    /// State is inspect-only and cannot be promoted.
    InspectOnly,
}

impl ProfileDurabilityClass {
    /// Returns true when the class is temporary and must be visibly labeled.
    pub const fn requires_temporary_badge(self) -> bool {
        matches!(self, Self::SessionOnly | Self::DiscardedOnExit)
    }
}

/// One profile card consumed by profile switch surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCardRow {
    /// Canonical profile ref.
    pub profile_ref: String,
    /// Human-readable profile name.
    pub name: String,
    /// Purpose shown in review surfaces.
    pub purpose: String,
    /// Source class for the profile.
    pub source_class: ProfileSourceClass,
    /// Durability state shown on the profile card.
    pub durability_class: ProfileDurabilityClass,
    /// Included governed scopes.
    pub included_scopes: Vec<ProfileScopeClass>,
    /// Whether the profile is exported or syncable without secrets.
    pub exportable_without_secrets: bool,
    /// Whether the profile card visibly distinguishes temporary state.
    pub visible_state_badge: bool,
}

/// Application timing for one switch delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyTimingClass {
    /// The value applies without restart.
    Immediate,
    /// A view, workspace, extension host, or process restart is required.
    RestartRequired,
}

/// One field-level or setting-level profile switch delta.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchDeltaRow {
    /// Stable delta id.
    pub delta_id: String,
    /// Setting id or artifact field path.
    pub field_path: String,
    /// Governed scope affected by the delta.
    pub scope_class: ProfileScopeClass,
    /// Preview of the effective value before the switch.
    pub before_value_preview: String,
    /// Preview of the effective value after the switch.
    pub after_value_preview: String,
    /// Whether the delta applies immediately or waits for restart.
    pub apply_timing: ApplyTimingClass,
    /// Human-readable restart posture.
    pub restart_posture: String,
    /// Whether the delta narrows trust, egress, extension behavior, or authority.
    pub narrows_behavior: bool,
    /// Whether the delta would widen authority if allowed.
    pub would_widen_authority: bool,
    /// Whether widening was refused or routed to explicit review.
    pub widening_refused_or_reviewed: bool,
}

/// Machine-local state that is explicitly excluded from profile switching.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludedMachineStateRow {
    /// Stable exclusion id.
    pub exclusion_id: String,
    /// State class or field path excluded from the switch.
    pub state_class: String,
    /// Reason the state is machine-local.
    pub reason: String,
    /// Whether it can be exported only through a separately reviewed addendum.
    pub separate_addendum_required: bool,
}

/// One narrowing effect shown in the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowingEffectRow {
    /// Stable effect id.
    pub effect_id: String,
    /// Area narrowed by the switch or import.
    pub affected_area: String,
    /// Explanation shown before apply.
    pub explanation: String,
    /// Whether the effect is a narrowing rather than a widening.
    pub narrows_only: bool,
}

/// The pre-apply profile-switch review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchReviewSheet {
    /// Canonical ref for the source profile.
    pub from_profile_ref: String,
    /// Canonical ref for the destination profile.
    pub to_profile_ref: String,
    /// Immediate deltas shown before apply.
    pub immediate_changes: Vec<ProfileSwitchDeltaRow>,
    /// Restart-required deltas shown before apply.
    pub restart_required_changes: Vec<ProfileSwitchDeltaRow>,
    /// Machine-local state excluded from the switch.
    pub excluded_machine_state: Vec<ExcludedMachineStateRow>,
    /// Trust, egress, extension, or managed-authority narrowing effects.
    pub narrowing_effects: Vec<NarrowingEffectRow>,
    /// Whether durable state changes materially.
    pub durable_state_changes_materially: bool,
    /// Whether a rollback checkpoint is created before apply.
    pub creates_rollback_checkpoint: bool,
    /// Rollback checkpoint ref created before apply.
    pub rollback_checkpoint_ref: Option<String>,
    /// Inspectable change summary ref.
    pub change_summary_ref: String,
    /// Rollback banner or checkpoint reopen ref.
    pub rollback_banner_ref: String,
}

/// Temporary-profile lifecycle action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporaryProfileActionClass {
    /// Discard temporary state.
    Discard,
    /// Promote selected temporary state into a durable profile.
    Promote,
    /// Compare the temporary profile to the durable profile.
    CompareToDurableProfile,
}

impl TemporaryProfileActionClass {
    /// All actions expected for a temporary or troubleshooting profile.
    pub const REQUIRED: [Self; 3] = [Self::Discard, Self::Promote, Self::CompareToDurableProfile];
}

/// One action available on a temporary-profile badge or card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporaryProfileActionRow {
    /// Action class.
    pub action_class: TemporaryProfileActionClass,
    /// Canonical action target ref.
    pub target_ref: String,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the action shows its persistence effect before commit.
    pub persistence_effect_visible: bool,
}

/// Temporary or troubleshooting profile lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporaryProfileLifecycle {
    /// Canonical temporary profile ref.
    pub profile_ref: String,
    /// Temporary profile source class.
    pub source_class: ProfileSourceClass,
    /// Durability state.
    pub durability_class: ProfileDurabilityClass,
    /// Badge text shown on profile surfaces.
    pub badge_label: String,
    /// Session lifetime or expiry text.
    pub lifetime_or_expiry: String,
    /// Restricted persistence rules shown to the user.
    pub restricted_persistence_rules: Vec<String>,
    /// Available actions.
    pub actions: Vec<TemporaryProfileActionRow>,
    /// Whether durable-vs-session-only state is visibly distinguished.
    pub state_boundary_visible: bool,
}

/// Secret or authority class excluded from profile artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactExclusionClass {
    /// Raw secret material.
    SecretMaterial,
    /// Keychain or secure-store material.
    KeychainMaterial,
    /// Machine-unique trust anchors.
    MachineUniqueTrustAnchors,
    /// Delegated credentials.
    DelegatedCredentials,
    /// Admin-policy bundles.
    AdminPolicyBundles,
    /// Workspace trust approvals.
    WorkspaceTrustApprovals,
    /// Machine-local paths and bindings.
    MachineLocalBindings,
}

impl ArtifactExclusionClass {
    /// Exclusion classes required by the stable artifact boundary.
    pub const REQUIRED: [Self; 7] = [
        Self::SecretMaterial,
        Self::KeychainMaterial,
        Self::MachineUniqueTrustAnchors,
        Self::DelegatedCredentials,
        Self::AdminPolicyBundles,
        Self::WorkspaceTrustApprovals,
        Self::MachineLocalBindings,
    ];
}

/// Portable profile artifact boundary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileArtifactBoundaryRow {
    /// Canonical artifact ref.
    pub artifact_ref: String,
    /// Artifact file extension or shape.
    pub artifact_shape: String,
    /// Artifact schema version.
    pub schema_version: String,
    /// Whether the artifact is text-based.
    pub text_based: bool,
    /// Whether the artifact is diffable.
    pub diffable: bool,
    /// Whether export excludes all forbidden secret and authority classes.
    pub exportable_without_forbidden_material: bool,
    /// Explicit forbidden classes excluded from this artifact.
    pub excluded_classes: Vec<ArtifactExclusionClass>,
}

/// Source class for an import, merge, or sync conflict row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictSourceClass {
    /// Imported profile artifact.
    ImportedProfile,
    /// Synced profile payload.
    SyncedProfile,
    /// Local durable profile.
    LocalDurableProfile,
}

/// Reason a remote or incoming packet degrades to local-authoritative state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalAuthoritativeReason {
    /// Incoming data is stale.
    StaleRemote,
    /// Sync service or device registry is unavailable.
    SyncUnavailable,
    /// Payload cannot be decrypted.
    UndecryptablePayload,
    /// Policy denies the incoming packet.
    PolicyDenied,
}

impl LocalAuthoritativeReason {
    /// Required degradation reasons for the stable corpus.
    pub const REQUIRED: [Self; 4] = [
        Self::StaleRemote,
        Self::SyncUnavailable,
        Self::UndecryptablePayload,
        Self::PolicyDenied,
    ];
}

/// Field-aware import, merge, or sync conflict row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportConflictReviewRow {
    /// Stable conflict id.
    pub conflict_id: String,
    /// Incoming source class.
    pub source_class: ConflictSourceClass,
    /// Field or scope being reviewed.
    pub field_path: String,
    /// Local value preview.
    pub local_value_preview: String,
    /// Incoming value preview.
    pub incoming_value_preview: String,
    /// Effective value preview after policy and local precedence.
    pub effective_value_preview: String,
    /// Whether the row is field-aware rather than whole-file last-writer-wins.
    pub field_aware: bool,
    /// Whether the row is scope-aware.
    pub scope_aware: bool,
    /// Whether applying the incoming value would widen authority.
    pub would_widen_authority: bool,
    /// Whether widening was refused.
    pub widening_refused: bool,
    /// Local-authoritative reason, when local durable state wins.
    pub local_authoritative_reason: Option<LocalAuthoritativeReason>,
    /// Offered review choices.
    pub offered_choices: Vec<String>,
}

/// Apply record for switch, import, merge, or sync mutations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyAuditRow {
    /// Stable apply id.
    pub apply_id: String,
    /// Apply source such as switch, import, merge, or sync.
    pub apply_source: String,
    /// Whether durable state is materially changed.
    pub durable_state_changed: bool,
    /// Inspectable change summary ref.
    pub change_summary_ref: String,
    /// Rollback checkpoint ref when durable state changes.
    pub rollback_checkpoint_ref: Option<String>,
    /// Whether rollback remains inspectable after apply.
    pub rollback_inspectable: bool,
}

/// File-portability fallback row for degraded sync/device-registry paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncFallbackRow {
    /// Degradation reason.
    pub reason: LocalAuthoritativeReason,
    /// Whether local durable state remains authoritative.
    pub local_durable_state_authoritative: bool,
    /// Whether export/import from file remains visible.
    pub file_based_portability_visible: bool,
    /// Whether the surface avoids implying hidden cloud authority.
    pub no_hidden_cloud_authority_claim: bool,
}

/// Surface class that consumes the shared profile-switch lifecycle record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Desktop profile card and profile switch UI.
    DesktopProfileSurface,
    /// Shell restart/reload and rollback banner surface.
    ShellRollbackSurface,
    /// Sync conflict review surface.
    SyncConflictReview,
    /// Help and docs surface.
    HelpDocs,
    /// Support export surface.
    SupportExport,
}

impl SurfaceClass {
    /// Required surfaces for stable profile-switch lifecycle truth.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopProfileSurface,
        Self::ShellRollbackSurface,
        Self::SyncConflictReview,
        Self::HelpDocs,
        Self::SupportExport,
    ];
}

/// Surface parity row proving one shared record feeds all review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Canonical record ref consumed by this surface.
    pub record_ref: String,
    /// Whether the surface consumes the shared contract.
    pub consumes_shared_contract: bool,
    /// Whether profile source and durability state are visible.
    pub shows_profile_state: bool,
    /// Whether immediate and restart-required deltas are visible.
    pub shows_restart_delta_truth: bool,
    /// Whether rollback checkpoint posture is visible.
    pub shows_rollback_checkpoint: bool,
    /// Whether local-authoritative fallback is visible when sync fails.
    pub shows_local_authoritative_fallback: bool,
}

/// Derived pillar verdicts for the profile-switch lifecycle lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchLifecyclePillars {
    /// The switch review sheet shows immediate, restart, exclusion, narrowing,
    /// and rollback truth.
    pub switch_review_complete: bool,
    /// Temporary profiles expose badge, lifetime, actions, and persistence rules.
    pub temporary_lifecycle_complete: bool,
    /// Profile artifacts remain text-based, schema-versioned, diffable, and
    /// forbidden-material-free.
    pub artifact_boundary_held: bool,
    /// Imports, conflicts, and sync applies are field- and scope-aware and do
    /// not silently widen authority.
    pub import_conflicts_non_widening: bool,
    /// Durable mutations create change summaries and rollback checkpoints.
    pub rollback_checkpoints_created: bool,
    /// Sync/device failures degrade to local-authoritative file portability.
    pub local_authoritative_fallback_visible: bool,
    /// All required surfaces consume the shared record.
    pub surfaces_share_truth: bool,
}

/// Reason a profile-switch lifecycle posture is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileSwitchNarrowingReason {
    /// Switch review is missing required review facets.
    SwitchReviewIncomplete,
    /// Temporary profile lifecycle is ambiguous.
    TemporaryLifecycleAmbiguous,
    /// Profile artifact leaks forbidden material or is not diffable.
    ArtifactBoundaryViolated,
    /// Import/conflict review allows silent widening.
    HiddenAuthorityWidening,
    /// Durable mutation lacks change summary or checkpoint.
    RollbackCheckpointMissing,
    /// Sync failure hides local-authoritative file portability.
    LocalAuthoritativeFallbackHidden,
    /// A required surface cloned or omitted shared truth.
    SurfaceTruthDrift,
}

/// Derived stable qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchLifecycleQualification {
    /// Derived claim class.
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies for stable.
    pub qualifies_stable: bool,
    /// Reasons the posture is narrowed below Stable.
    pub narrowing_reasons: Vec<ProfileSwitchNarrowingReason>,
}

/// Canonical profile-switch lifecycle certification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchLifecycleCertification {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Profile cards participating in the scenario.
    pub profile_cards: Vec<ProfileCardRow>,
    /// Pre-apply switch review sheet.
    pub switch_review: ProfileSwitchReviewSheet,
    /// Temporary-profile lifecycle state.
    pub temporary_profile: TemporaryProfileLifecycle,
    /// Portable artifact boundary rows.
    pub artifact_boundaries: Vec<ProfileArtifactBoundaryRow>,
    /// Import, merge, and sync conflict rows.
    pub import_conflicts: Vec<ImportConflictReviewRow>,
    /// Apply audit rows.
    pub apply_audit: Vec<ApplyAuditRow>,
    /// Sync fallback rows.
    pub sync_fallbacks: Vec<SyncFallbackRow>,
    /// Cross-surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// Derived pillar verdicts.
    pub pillars: ProfileSwitchLifecyclePillars,
    /// Stable qualification.
    pub stable_qualification: ProfileSwitchLifecycleQualification,
}

/// Validation error for profile-switch lifecycle certification records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileSwitchLifecycleValidationError {
    /// Record identity or shared contract is not the expected stable value.
    IdentityMismatch,
    /// A canonical object ref was required and missing.
    NonCanonicalRef { field: &'static str, value: String },
    /// A switch-review facet is missing.
    SwitchReviewIncomplete,
    /// Temporary profile lifecycle is missing required state or actions.
    TemporaryLifecycleIncomplete,
    /// Profile artifact boundary is not secret-safe or diffable.
    ArtifactBoundaryViolation,
    /// Import, merge, or sync row can silently widen authority.
    HiddenAuthorityWidening { conflict_id: String },
    /// Durable state changed without change summary and rollback checkpoint.
    RollbackCheckpointMissing { apply_id: String },
    /// Required local-authoritative fallback reason is missing or hidden.
    LocalAuthoritativeFallbackMissing,
    /// Required surface truth row is missing or incomplete.
    SurfaceTruthMissing,
    /// Stable qualification overclaims failed pillars.
    StableOverclaim,
}

impl core::fmt::Display for ProfileSwitchLifecycleValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IdentityMismatch => write!(f, "profile-switch record identity mismatch"),
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::SwitchReviewIncomplete => write!(f, "profile-switch review is incomplete"),
            Self::TemporaryLifecycleIncomplete => {
                write!(f, "temporary-profile lifecycle is incomplete")
            }
            Self::ArtifactBoundaryViolation => write!(f, "profile artifact boundary is violated"),
            Self::HiddenAuthorityWidening { conflict_id } => {
                write!(f, "conflict `{conflict_id}` silently widens authority")
            }
            Self::RollbackCheckpointMissing { apply_id } => {
                write!(f, "apply `{apply_id}` lacks required rollback checkpoint")
            }
            Self::LocalAuthoritativeFallbackMissing => {
                write!(f, "local-authoritative fallback is missing")
            }
            Self::SurfaceTruthMissing => write!(f, "required surface truth is missing"),
            Self::StableOverclaim => write!(f, "stable qualification overclaims failed pillars"),
        }
    }
}

impl std::error::Error for ProfileSwitchLifecycleValidationError {}

/// Validates a profile-switch lifecycle certification record.
pub fn validate_profile_switch_lifecycle_record(
    record: &ProfileSwitchLifecycleCertification,
) -> Result<(), Vec<ProfileSwitchLifecycleValidationError>> {
    let mut errors = Vec::new();

    if record.record_kind != PROFILE_SWITCH_REVIEW_RECORD_KIND
        || record.schema_version != PROFILE_SWITCH_REVIEW_SCHEMA_VERSION
        || record.shared_contract_ref != PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF
    {
        errors.push(ProfileSwitchLifecycleValidationError::IdentityMismatch);
    }

    check_ref(
        &mut errors,
        "switch_review.from_profile_ref",
        &record.switch_review.from_profile_ref,
    );
    check_ref(
        &mut errors,
        "switch_review.to_profile_ref",
        &record.switch_review.to_profile_ref,
    );
    check_ref(
        &mut errors,
        "switch_review.change_summary_ref",
        &record.switch_review.change_summary_ref,
    );
    check_ref(
        &mut errors,
        "switch_review.rollback_banner_ref",
        &record.switch_review.rollback_banner_ref,
    );
    if let Some(checkpoint) = &record.switch_review.rollback_checkpoint_ref {
        check_ref(
            &mut errors,
            "switch_review.rollback_checkpoint_ref",
            checkpoint,
        );
    }

    let review_complete = !record.switch_review.immediate_changes.is_empty()
        && !record.switch_review.restart_required_changes.is_empty()
        && !record.switch_review.excluded_machine_state.is_empty()
        && !record.switch_review.narrowing_effects.is_empty()
        && (!record.switch_review.creates_rollback_checkpoint
            || record.switch_review.rollback_checkpoint_ref.is_some())
        && record.switch_review.immediate_changes.iter().all(|row| {
            row.apply_timing == ApplyTimingClass::Immediate
                && (!row.would_widen_authority || row.widening_refused_or_reviewed)
        })
        && record
            .switch_review
            .restart_required_changes
            .iter()
            .all(|row| row.apply_timing == ApplyTimingClass::RestartRequired);
    if !review_complete || record.pillars.switch_review_complete != review_complete {
        errors.push(ProfileSwitchLifecycleValidationError::SwitchReviewIncomplete);
    }

    let actions: BTreeSet<_> = record
        .temporary_profile
        .actions
        .iter()
        .map(|row| row.action_class)
        .collect();
    let temporary_lifecycle_complete = record
        .temporary_profile
        .durability_class
        .requires_temporary_badge()
        && matches!(
            record.temporary_profile.source_class,
            ProfileSourceClass::TemporaryProfile | ProfileSourceClass::TroubleshootingProfile
        )
        && !record.temporary_profile.badge_label.trim().is_empty()
        && !record
            .temporary_profile
            .lifetime_or_expiry
            .trim()
            .is_empty()
        && !record
            .temporary_profile
            .restricted_persistence_rules
            .is_empty()
        && record.temporary_profile.state_boundary_visible
        && TemporaryProfileActionClass::REQUIRED
            .iter()
            .all(|required| actions.contains(required))
        && record.temporary_profile.actions.iter().all(|row| {
            is_canonical_object_ref(&row.target_ref)
                && row.keyboard_reachable
                && row.persistence_effect_visible
        });
    if !temporary_lifecycle_complete
        || record.pillars.temporary_lifecycle_complete != temporary_lifecycle_complete
    {
        errors.push(ProfileSwitchLifecycleValidationError::TemporaryLifecycleIncomplete);
    }

    let excluded: BTreeSet<_> = record
        .artifact_boundaries
        .iter()
        .flat_map(|row| row.excluded_classes.iter().copied())
        .collect();
    let artifact_boundary_held = !record.artifact_boundaries.is_empty()
        && ArtifactExclusionClass::REQUIRED
            .iter()
            .all(|required| excluded.contains(required))
        && record.artifact_boundaries.iter().all(|row| {
            is_canonical_object_ref(&row.artifact_ref)
                && row.text_based
                && row.diffable
                && row.exportable_without_forbidden_material
                && !row.schema_version.trim().is_empty()
        });
    if !artifact_boundary_held || record.pillars.artifact_boundary_held != artifact_boundary_held {
        errors.push(ProfileSwitchLifecycleValidationError::ArtifactBoundaryViolation);
    }

    let mut conflicts_non_widening = !record.import_conflicts.is_empty();
    let mut hidden_widening_ids = Vec::new();
    for conflict in &record.import_conflicts {
        if !conflict.field_aware
            || !conflict.scope_aware
            || (conflict.would_widen_authority && !conflict.widening_refused)
        {
            conflicts_non_widening = false;
            hidden_widening_ids.push(conflict.conflict_id.clone());
        }
    }
    if record.pillars.import_conflicts_non_widening != conflicts_non_widening {
        errors.push(ProfileSwitchLifecycleValidationError::StableOverclaim);
    }

    let mut checkpoints_created = !record.apply_audit.is_empty();
    let mut missing_checkpoint_ids = Vec::new();
    for apply in &record.apply_audit {
        check_ref(
            &mut errors,
            "apply_audit.change_summary_ref",
            &apply.change_summary_ref,
        );
        if apply.durable_state_changed {
            match &apply.rollback_checkpoint_ref {
                Some(checkpoint)
                    if is_canonical_object_ref(checkpoint) && apply.rollback_inspectable => {}
                _ => {
                    checkpoints_created = false;
                    missing_checkpoint_ids.push(apply.apply_id.clone());
                }
            }
        }
    }
    if record.pillars.rollback_checkpoints_created != checkpoints_created {
        errors.push(ProfileSwitchLifecycleValidationError::StableOverclaim);
    }

    let fallback_reasons: BTreeSet<_> =
        record.sync_fallbacks.iter().map(|row| row.reason).collect();
    let fallback_visible = LocalAuthoritativeReason::REQUIRED
        .iter()
        .all(|required| fallback_reasons.contains(required))
        && record.sync_fallbacks.iter().all(|row| {
            row.local_durable_state_authoritative
                && row.file_based_portability_visible
                && row.no_hidden_cloud_authority_claim
        });
    if !fallback_visible || record.pillars.local_authoritative_fallback_visible != fallback_visible
    {
        errors.push(ProfileSwitchLifecycleValidationError::LocalAuthoritativeFallbackMissing);
    }

    let surface_classes: BTreeSet<_> = record
        .surface_truth
        .iter()
        .map(|row| row.surface_class)
        .collect();
    let surfaces_share_truth = SurfaceClass::REQUIRED
        .iter()
        .all(|required| surface_classes.contains(required))
        && record.surface_truth.iter().all(|row| {
            is_canonical_object_ref(&row.record_ref)
                && row.consumes_shared_contract
                && row.shows_profile_state
                && row.shows_restart_delta_truth
                && row.shows_rollback_checkpoint
                && row.shows_local_authoritative_fallback
        });
    if !surfaces_share_truth || record.pillars.surfaces_share_truth != surfaces_share_truth {
        errors.push(ProfileSwitchLifecycleValidationError::SurfaceTruthMissing);
    }

    let all_pillars = record.pillars.switch_review_complete
        && record.pillars.temporary_lifecycle_complete
        && record.pillars.artifact_boundary_held
        && record.pillars.import_conflicts_non_widening
        && record.pillars.rollback_checkpoints_created
        && record.pillars.local_authoritative_fallback_visible
        && record.pillars.surfaces_share_truth;
    if record.stable_qualification.qualifies_stable {
        if record.stable_qualification.claim_class != StableClaimClass::Stable
            || !record.stable_qualification.narrowing_reasons.is_empty()
            || !all_pillars
        {
            errors.push(ProfileSwitchLifecycleValidationError::StableOverclaim);
        }
        for conflict_id in hidden_widening_ids {
            errors.push(
                ProfileSwitchLifecycleValidationError::HiddenAuthorityWidening { conflict_id },
            );
        }
        for apply_id in missing_checkpoint_ids {
            errors.push(
                ProfileSwitchLifecycleValidationError::RollbackCheckpointMissing { apply_id },
            );
        }
    } else if record.stable_qualification.claim_class == StableClaimClass::Stable
        || record.stable_qualification.narrowing_reasons.is_empty()
    {
        errors.push(ProfileSwitchLifecycleValidationError::StableOverclaim);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_ref(
    errors: &mut Vec<ProfileSwitchLifecycleValidationError>,
    field: &'static str,
    value: &str,
) {
    if !is_canonical_object_ref(value) {
        errors.push(ProfileSwitchLifecycleValidationError::NonCanonicalRef {
            field,
            value: value.to_owned(),
        });
    }
}
