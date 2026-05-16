//! Native extension mutation review surface for install, update, disable,
//! and rollback flows.
//!
//! The lower-level extension contracts own manifest validation, permission
//! deltas, publisher continuity, runtime budget, install topology, registry
//! truth, and rollback metadata. This module owns the first shell projection
//! that renders those facts together before a mutation can commit. Native
//! review, CLI/headless inspection, mirror review, and support export all read
//! the same record shape so registry, mirror, offline bundle, and manual
//! import lanes cannot fork their review vocabulary.

use serde::{Deserialize, Serialize};

use aureline_extensions::{
    ActivationBudget, CompatibilityLabel, DeclaredVsEffectiveDiffEntry,
    EffectivePermissionDiffClass, ExtensionReviewAlphaPacketRecord, InstallReviewAlphaPacketRecord,
    InstallReviewContentSourceClass, InstallReviewDecisionClass, PermissionDeltaEntry,
    PermissionManifestDeltaRecord, PublisherContinuityAlphaRecord, PublisherTrustTierClass,
    ReConsentDecisionClass, ReConsentReasonClass, RedactionClass, ReviewDecisionClass,
};
use aureline_extensions::{ManifestOriginSourceClass, RuntimeCostClass, RuntimeCostEvidenceClass};

#[cfg(test)]
mod tests;

/// Record-kind tag carried by [`ExtensionMutationReviewSurface`] payloads.
pub const EXTENSION_MUTATION_REVIEW_SURFACE_RECORD_KIND: &str =
    "extension_mutation_review_surface_record";

/// Record-kind tag carried by [`ExtensionMutationReviewSupportExport`] payloads.
pub const EXTENSION_MUTATION_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_mutation_review_support_export_record";

/// Schema version for shell extension mutation review payloads.
pub const EXTENSION_MUTATION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Mutation reviewed by the shell-owned extension review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionMutationReviewActionClass {
    /// Install a previously absent extension.
    Install,
    /// Update an installed extension to a new reviewed version.
    Update,
    /// Disable an installed extension while preserving user-owned state.
    Disable,
    /// Restore a last-known-good extension version from a reviewed checkpoint.
    Rollback,
}

/// Consumer surface rendering an extension mutation review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionMutationReviewSurfaceClass {
    /// Product-owned native review sheet that may commit after admission.
    NativeReviewSheet,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support-export projection.
    SupportExport,
    /// Admin, mirror, or offline-bundle review projection.
    RegistryMirrorReview,
}

/// Disclosure class the review surface must render before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionMutationReviewDisclosureClass {
    /// Publisher identity, trust tier, and continuity state are visible.
    PublisherIdentity,
    /// Primary, private, mirror, offline, or local source lane is visible.
    SourceLane,
    /// Product-owned native authority boundary is visible.
    NativeAuthority,
    /// Permission delta or explicit no-widening statement is visible.
    PermissionDelta,
    /// Compatibility range, label, and evidence are visible.
    CompatibilityRange,
    /// Activation budget, triggers, runtime-cost class, and evidence are visible.
    ActivationBudget,
    /// Rollback checkpoint, last-known-good, or no-rollback posture is visible.
    RollbackImplications,
    /// User-owned state preservation and remaining installed/cache/revoked state are visible.
    StatePreservation,
}

/// Decision emitted by the shell extension review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionMutationReviewDecisionClass {
    /// The native review sheet may commit the mutation.
    ReadyForNativeMutation,
    /// A user acknowledgement or re-consent decision is still required.
    AwaitingUserReview,
    /// An administrator or mirror operator must act first.
    AwaitingAdminReview,
    /// The mutation is refused.
    Denied,
}

/// Typed reason paired with [`ExtensionMutationReviewDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionMutationReviewReasonClass {
    /// All required truth was rendered and no blocker was found.
    AllTruthRendered,
    /// A required disclosure was not rendered.
    MissingRequiredDisclosure,
    /// Publisher identity is absent, anonymous, or quarantined.
    PublisherIdentityOpaque,
    /// Source lane or source-lane summary is missing.
    MissingSourceLaneTruth,
    /// Upstream extension or install review refused the mutation.
    UpstreamReviewDenied,
    /// Upstream extension or install review still awaits user/admin review.
    UpstreamReviewPending,
    /// Install/update review does not contain a permission diff.
    MissingPermissionDelta,
    /// Version-to-version permission delta requires explicit re-consent.
    PermissionWideningRequiresReConsent,
    /// Version-to-version permission delta was structurally refused.
    PermissionDeltaRefused,
    /// Effective-permission truth blocked a widening attempt.
    EffectivePermissionWideningBlocked,
    /// Compatibility range, activation budget, triggers, or runtime evidence is missing.
    MissingCompatibilityOrBudgetTruth,
    /// Rollback checkpoint or last-known-good truth is missing.
    MissingRollbackTruth,
    /// Disable or rollback state preservation truth is missing.
    MissingStatePreservationTruth,
    /// Disable can proceed and preserves user-owned state.
    DisablePreservesUserState,
    /// Rollback target and state preservation are ready.
    RollbackCandidateReady,
}

/// Action offered by one review projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionMutationReviewActionOfferClass {
    /// Commit an install from the native review sheet.
    ApproveInstall,
    /// Commit an update from the native review sheet.
    ApproveUpdate,
    /// Disable the installed extension from the native review sheet.
    DisableExtension,
    /// Apply a reviewed rollback from the native review sheet.
    ApplyRollback,
    /// Open the product-owned native review sheet.
    OpenNativeReviewSheet,
    /// Keep the current installed version pinned.
    StayPinned,
    /// Keep the extension disabled.
    KeepDisabled,
    /// Open the permission delta.
    OpenPermissionDelta,
    /// Open compatibility evidence.
    OpenCompatibilityEvidence,
    /// Open activation-budget evidence.
    OpenActivationBudgetEvidence,
    /// Open rollback and preserved-state details.
    OpenRollbackDetails,
    /// Export a metadata-safe support packet.
    ExportSupportPacket,
    /// Ask an administrator, mirror operator, or policy owner to review.
    ConsultAdmin,
}

/// Rollback implication rendered before a mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackImplicationClass {
    /// Fresh install has no prior installed version to restore.
    NotRequiredFreshInstall,
    /// A checkpoint is created before mutation and can restore the previous state.
    CheckpointCreatedBeforeMutation,
    /// The prior version remains pinned until explicit approval.
    PriorVersionPinned,
    /// A last-known-good version can be restored.
    RollbackAvailableFromLastKnownGood,
    /// Rollback is blocked because the checkpoint or target is missing.
    RollbackBlockedMissingCheckpoint,
    /// Rollback disables the current version and restores a prior candidate.
    RollbackWillDisableCurrentVersion,
}

/// User-owned state preservation posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStateRetentionClass {
    /// User-owned state is preserved through the mutation.
    UserOwnedStatePreserved,
    /// User-owned state is exported or checkpointed before mutation.
    UserOwnedStateExportedBeforeMutation,
    /// User-owned state is not touched by the mutation.
    UserOwnedStateNotTouched,
    /// State preservation truth is missing and the mutation must not proceed.
    StatePreservationMissing,
}

/// Installed artifact disposition after the reviewed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstalledArtifactDispositionClass {
    /// Requested version becomes installed and enabled.
    InstalledEnabled,
    /// Requested version remains installed but disabled.
    InstalledDisabled,
    /// Requested version replaces the current installed version.
    ReplacedByRequestedVersion,
    /// Last-known-good version is restored.
    RestoredToLastKnownGood,
    /// Extension is removed from the active set.
    RemovedFromActiveSet,
}

/// Local cache disposition after the reviewed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheDispositionClass {
    /// Cache is preserved.
    CachePreserved,
    /// Cache is retained only to support rollback.
    CacheRetainedForRollback,
    /// Cache is retained with revoked entries marked and inactive.
    CacheRevoked,
    /// Cache is purged only after a checkpoint exists.
    CachePurgedAfterCheckpoint,
}

/// Revocation disposition after the reviewed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationDispositionClass {
    /// No revocation state changes.
    NoRevocationChange,
    /// Existing revocation state remains visible.
    RevocationRetained,
    /// Activation is revoked while installed metadata remains.
    ActivationRevoked,
    /// Catalog or mirror revocation is applied and cited.
    CatalogRevocationApplied,
}

/// Source lane truth shared by primary registry, mirror, and manual import paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionMutationReviewSourcePosture {
    /// Manifest origin/source class from the extension contracts.
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    /// Marketplace/package content source class when a catalog lane participates.
    pub content_source_class: InstallReviewContentSourceClass,
    /// Export-safe source summary rendered by UI, CLI, mirror, and support surfaces.
    pub source_summary: String,
}

/// State and rollback plan rendered before disable or rollback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionMutationStatePlan {
    /// Stable state-plan ref.
    pub state_plan_ref: String,
    /// Currently installed version, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// Requested version, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_version: Option<String>,
    /// Last-known-good version available for rollback, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_version: Option<String>,
    /// Rollback or pre-mutation checkpoint ref, when required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Rollback implication shown before commit.
    pub rollback_implication_class: RollbackImplicationClass,
    /// User-owned state retention posture.
    pub user_state_retention_class: UserStateRetentionClass,
    /// Installed artifact disposition after mutation.
    pub installed_artifact_disposition_class: InstalledArtifactDispositionClass,
    /// Cache disposition after mutation.
    pub cache_disposition_class: CacheDispositionClass,
    /// Revocation disposition after mutation.
    pub revocation_disposition_class: RevocationDispositionClass,
    /// User-owned state refs preserved or exported by the plan.
    pub preserved_user_state_refs: Vec<String>,
    /// Installed artifact refs that remain after mutation.
    pub installed_artifact_refs: Vec<String>,
    /// Cached artifact refs that remain after mutation.
    pub cached_artifact_refs: Vec<String>,
    /// Revoked artifact refs that remain visible after mutation.
    pub revoked_artifact_refs: Vec<String>,
    /// Export-safe state summary for UI, CLI, and support consumers.
    pub state_summary: String,
}

/// Borrowed inputs for projecting an extension mutation review surface.
pub struct ExtensionMutationReviewInput<'a> {
    /// Stable review id.
    pub review_id: &'a str,
    /// Requested mutation.
    pub action_class: ExtensionMutationReviewActionClass,
    /// Consumer surface building the review.
    pub surface_class: ExtensionMutationReviewSurfaceClass,
    /// Extension subject ref.
    pub subject_ref: &'a str,
    /// Source lane truth.
    pub source_posture: ExtensionMutationReviewSourcePosture,
    /// Upstream install/update review packet, required for install and update.
    pub install_review_packet: Option<&'a InstallReviewAlphaPacketRecord>,
    /// Upstream extension review packet.
    pub extension_review_packet: &'a ExtensionReviewAlphaPacketRecord,
    /// Publisher continuity record rendered by the review.
    pub publisher_continuity: &'a PublisherContinuityAlphaRecord,
    /// Optional version-to-version permission delta.
    pub permission_manifest_delta: Option<&'a PermissionManifestDeltaRecord>,
    /// State preservation and rollback plan.
    pub state_plan: ExtensionMutationStatePlan,
    /// Disclosure classes rendered before the decision.
    pub rendered_disclosures: Vec<ExtensionMutationReviewDisclosureClass>,
    /// Review event refs emitted while building the surface.
    pub review_event_refs: Vec<String>,
    /// Decision timestamp.
    pub decided_at: &'a str,
}

/// Shell projection rendered by native review, CLI/headless, mirror, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionMutationReviewSurface {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this record.
    pub extension_mutation_review_schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Requested mutation.
    pub action_class: ExtensionMutationReviewActionClass,
    /// Consumer surface rendered by this record.
    pub surface_class: ExtensionMutationReviewSurfaceClass,
    /// Extension subject ref.
    pub subject_ref: String,
    /// Publisher identity ref rendered to the reviewer.
    pub publisher_identity_ref: String,
    /// Publisher display label rendered to the reviewer.
    pub publisher_display_label: String,
    /// Publisher trust tier rendered to the reviewer.
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    /// Source lane truth rendered to the reviewer.
    pub source_posture: ExtensionMutationReviewSourcePosture,
    /// Compatibility range rendered for install/update flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_range: Option<String>,
    /// Compatibility label rendered for install/update flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_label: Option<CompatibilityLabel>,
    /// Compatibility evidence refs rendered for install/update flows.
    pub compatibility_evidence_refs: Vec<String>,
    /// Structured activation budget rendered for install/update flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_budget: Option<ActivationBudget>,
    /// Runtime-cost class rendered for install/update flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_cost_class: Option<RuntimeCostClass>,
    /// Runtime-cost evidence class rendered for install/update flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_cost_evidence_class: Option<RuntimeCostEvidenceClass>,
    /// Runtime budget class rendered for install/update flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_budget_class: Option<String>,
    /// Activation trigger refs rendered for install/update flows.
    pub activation_trigger_refs: Vec<String>,
    /// Activation evidence refs rendered for install/update flows.
    pub activation_evidence_refs: Vec<String>,
    /// Effective-permission summary ref rendered by the surface.
    pub effective_permission_summary_ref: String,
    /// Optional version-to-version permission delta ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_delta_ref: Option<String>,
    /// Declared-vs-effective permission delta entries.
    pub effective_permission_delta_entries: Vec<DeclaredVsEffectiveDiffEntry>,
    /// Version-to-version permission delta entries.
    pub version_permission_delta_entries: Vec<PermissionDeltaEntry>,
    /// Re-consent decision from version-to-version permission delta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub re_consent_decision_class: Option<ReConsentDecisionClass>,
    /// Re-consent reason from version-to-version permission delta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub re_consent_reason_class: Option<ReConsentReasonClass>,
    /// Count of permission-widening deltas.
    pub permission_widening_count: u32,
    /// Count of permission-narrowing deltas.
    pub permission_narrowing_count: u32,
    /// True when version-to-version permission delta requires explicit re-consent.
    pub requires_re_consent: bool,
    /// State preservation and rollback plan rendered by the surface.
    pub state_plan: ExtensionMutationStatePlan,
    /// Required disclosures for the action.
    pub required_disclosures: Vec<ExtensionMutationReviewDisclosureClass>,
    /// Disclosures rendered before decision.
    pub rendered_disclosures: Vec<ExtensionMutationReviewDisclosureClass>,
    /// True when this surface may commit the mutation.
    pub mutation_allowed: bool,
    /// Decision emitted by the shell review surface.
    pub decision_class: ExtensionMutationReviewDecisionClass,
    /// Typed reason paired with the decision.
    pub decision_reason_class: ExtensionMutationReviewReasonClass,
    /// Actions offered by the surface.
    pub offered_actions: Vec<ExtensionMutationReviewActionOfferClass>,
    /// Export-safe summary rendered by UI, CLI, and support export.
    pub export_safe_summary: String,
    /// Review event refs emitted while building the surface.
    pub review_event_refs: Vec<String>,
    /// Decision timestamp.
    pub decided_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support export derived from an extension mutation review surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionMutationReviewSupportExport {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this record.
    pub extension_mutation_review_schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Source review ref.
    pub review_ref: String,
    /// Requested mutation.
    pub action_class: ExtensionMutationReviewActionClass,
    /// Extension subject ref.
    pub subject_ref: String,
    /// Decision emitted by the source review.
    pub decision_class: ExtensionMutationReviewDecisionClass,
    /// Typed reason paired with the source decision.
    pub decision_reason_class: ExtensionMutationReviewReasonClass,
    /// Publisher identity ref.
    pub publisher_identity_ref: String,
    /// Manifest origin/source class.
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    /// Permission delta ref, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_delta_ref: Option<String>,
    /// Compatibility range, when rendered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_range: Option<String>,
    /// Runtime budget class, when rendered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_budget_class: Option<String>,
    /// Rollback checkpoint ref, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Last-known-good version, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_version: Option<String>,
    /// User state retention posture.
    pub user_state_retention_class: UserStateRetentionClass,
    /// Installed artifact disposition.
    pub installed_artifact_disposition_class: InstalledArtifactDispositionClass,
    /// Cache disposition.
    pub cache_disposition_class: CacheDispositionClass,
    /// Revocation disposition.
    pub revocation_disposition_class: RevocationDispositionClass,
    /// Export-safe summary.
    pub export_safe_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by extension mutation review validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionMutationReviewFinding {
    /// Stable validation check id.
    pub check_id: String,
    /// Human-readable validation message.
    pub message: String,
}

impl ExtensionMutationReviewFinding {
    fn new(check_id: &str, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.to_string(),
            message: message.into(),
        }
    }
}

/// Project the shell extension mutation review surface from lower-level
/// extension contracts.
pub fn project_extension_mutation_review_surface(
    input: ExtensionMutationReviewInput<'_>,
) -> ExtensionMutationReviewSurface {
    let required_disclosures = required_disclosures_for(input.action_class);
    let missing_disclosure =
        first_missing_disclosure(&required_disclosures, &input.rendered_disclosures);
    let publisher_identity_ref = input.publisher_continuity.publisher_identity_ref.clone();
    let publisher_display_label = input.publisher_continuity.publisher_display_label.clone();
    let publisher_trust_tier_class = input.publisher_continuity.publisher_trust_tier_class;

    let compatibility_range = input
        .install_review_packet
        .map(|packet| packet.compatibility.aureline_version_range.clone());
    let compatibility_label = input
        .install_review_packet
        .map(|packet| packet.compatibility.compatibility_label);
    let compatibility_evidence_refs = input
        .install_review_packet
        .map(|packet| packet.compatibility.evidence_refs.clone())
        .unwrap_or_default();
    let activation_budget = input
        .install_review_packet
        .map(|packet| packet.activation_budget.activation_budget.clone());
    let runtime_cost_class = input
        .install_review_packet
        .map(|packet| packet.activation_budget.runtime_cost_class);
    let runtime_cost_evidence_class = input
        .install_review_packet
        .map(|packet| packet.activation_budget.runtime_cost_evidence_class);
    let runtime_budget_class = input
        .install_review_packet
        .map(|packet| packet.activation_budget.runtime_budget_class.clone());
    let activation_trigger_refs = input
        .install_review_packet
        .map(|packet| packet.activation_budget.activation_trigger_refs.clone())
        .unwrap_or_default();
    let activation_evidence_refs = input
        .install_review_packet
        .map(|packet| packet.activation_budget.evidence_refs.clone())
        .unwrap_or_default();
    let effective_permission_summary_ref = input
        .install_review_packet
        .map(|packet| packet.effective_permission_summary_ref.clone())
        .unwrap_or_else(|| {
            input
                .extension_review_packet
                .effective_permission_summary_ref
                .clone()
        });
    let effective_permission_delta_entries = input
        .install_review_packet
        .map(|packet| packet.permission_delta_entries.clone())
        .unwrap_or_default();
    let upstream_widening_blocked = input
        .install_review_packet
        .map(|packet| {
            packet.widening_attempted_blocked_count > 0
                || packet.permission_delta_entries.iter().any(|entry| {
                    matches!(
                        entry.diff_class,
                        EffectivePermissionDiffClass::WideningAttemptedBlocked
                    )
                })
        })
        .unwrap_or(false);

    let permission_delta_ref = input
        .permission_manifest_delta
        .map(|delta| delta.delta_id.clone());
    let version_permission_delta_entries = input
        .permission_manifest_delta
        .map(|delta| delta.delta_entries.clone())
        .unwrap_or_default();
    let re_consent_decision_class = input
        .permission_manifest_delta
        .map(|delta| delta.re_consent_decision_class);
    let re_consent_reason_class = input
        .permission_manifest_delta
        .map(|delta| delta.re_consent_reason_class);
    let permission_widening_count = input
        .permission_manifest_delta
        .map(|delta| delta.widening_count)
        .unwrap_or(0);
    let permission_narrowing_count = input
        .permission_manifest_delta
        .map(|delta| delta.narrowing_count)
        .unwrap_or(0);
    let requires_re_consent = matches!(
        re_consent_decision_class,
        Some(ReConsentDecisionClass::ReConsentRequiredWidening)
            | Some(ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass)
    );

    let (decision_class, decision_reason_class, decision_summary) = if let Some(missing) =
        missing_disclosure
    {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::MissingRequiredDisclosure,
            format!("Denied: review surface did not render required disclosure '{missing:?}'."),
        )
    } else if publisher_identity_opaque(
        &publisher_identity_ref,
        &publisher_display_label,
        publisher_trust_tier_class,
    ) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::PublisherIdentityOpaque,
            "Denied: publisher identity is missing, anonymous, or quarantined.".to_string(),
        )
    } else if source_lane_missing(&input.source_posture) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::MissingSourceLaneTruth,
            "Denied: source lane truth is missing or unattributed.".to_string(),
        )
    } else if matches!(
        input.extension_review_packet.decision_class,
        ReviewDecisionClass::Denied
    ) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::UpstreamReviewDenied,
            "Denied: upstream extension review denied the mutation.".to_string(),
        )
    } else if matches!(
        input.extension_review_packet.decision_class,
        ReviewDecisionClass::AwaitingAdminReview
    ) {
        (
            ExtensionMutationReviewDecisionClass::AwaitingAdminReview,
            ExtensionMutationReviewReasonClass::UpstreamReviewPending,
            "Awaiting admin review: upstream extension review has not admitted the mutation."
                .to_string(),
        )
    } else if matches!(
        input.extension_review_packet.decision_class,
        ReviewDecisionClass::AwaitingUserReview
    ) {
        (
            ExtensionMutationReviewDecisionClass::AwaitingUserReview,
            ExtensionMutationReviewReasonClass::UpstreamReviewPending,
            "Awaiting user review: upstream extension review has not admitted the mutation."
                .to_string(),
        )
    } else if install_or_update_runtime_truth_missing(
        input.action_class,
        input.install_review_packet,
    ) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::MissingCompatibilityOrBudgetTruth,
            "Denied: install/update review is missing compatibility range, activation budget, trigger, or evidence truth.".to_string(),
        )
    } else if install_or_update_upstream_pending_or_denied(input.install_review_packet)
        == Some(ExtensionMutationReviewDecisionClass::Denied)
    {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::UpstreamReviewDenied,
            "Denied: upstream install/update review denied the mutation.".to_string(),
        )
    } else if install_or_update_upstream_pending_or_denied(input.install_review_packet)
        == Some(ExtensionMutationReviewDecisionClass::AwaitingAdminReview)
    {
        (
            ExtensionMutationReviewDecisionClass::AwaitingAdminReview,
            ExtensionMutationReviewReasonClass::UpstreamReviewPending,
            "Awaiting admin review: upstream install/update review has not admitted the mutation."
                .to_string(),
        )
    } else if install_or_update_upstream_pending_or_denied(input.install_review_packet)
        == Some(ExtensionMutationReviewDecisionClass::AwaitingUserReview)
    {
        (
            ExtensionMutationReviewDecisionClass::AwaitingUserReview,
            ExtensionMutationReviewReasonClass::UpstreamReviewPending,
            "Awaiting user review: upstream install/update review has not admitted the mutation."
                .to_string(),
        )
    } else if install_or_update_permission_delta_missing(
        input.action_class,
        input.install_review_packet,
        input.permission_manifest_delta,
    ) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::MissingPermissionDelta,
            "Denied: install/update review is missing the required permission delta.".to_string(),
        )
    } else if upstream_widening_blocked {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::EffectivePermissionWideningBlocked,
            "Denied: effective-permission truth blocked a widening attempt.".to_string(),
        )
    } else if matches!(
        re_consent_decision_class,
        Some(ReConsentDecisionClass::RefusedInconsistentInput)
    ) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::PermissionDeltaRefused,
            "Denied: version-to-version permission delta was refused.".to_string(),
        )
    } else if requires_re_consent {
        (
            ExtensionMutationReviewDecisionClass::AwaitingUserReview,
            ExtensionMutationReviewReasonClass::PermissionWideningRequiresReConsent,
            "Awaiting user review: permission delta widens authority and requires re-consent."
                .to_string(),
        )
    } else if state_preservation_missing(input.action_class, &input.state_plan) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::MissingStatePreservationTruth,
            "Denied: disable or rollback review is missing user-owned state preservation truth."
                .to_string(),
        )
    } else if rollback_truth_missing(input.action_class, &input.state_plan) {
        (
            ExtensionMutationReviewDecisionClass::Denied,
            ExtensionMutationReviewReasonClass::MissingRollbackTruth,
            "Denied: rollback checkpoint or last-known-good truth is missing.".to_string(),
        )
    } else {
        match input.action_class {
            ExtensionMutationReviewActionClass::Disable => (
                ExtensionMutationReviewDecisionClass::ReadyForNativeMutation,
                ExtensionMutationReviewReasonClass::DisablePreservesUserState,
                "Ready: disable preserves user-owned state and explains installed, cached, and revoked state.".to_string(),
            ),
            ExtensionMutationReviewActionClass::Rollback => (
                ExtensionMutationReviewDecisionClass::ReadyForNativeMutation,
                ExtensionMutationReviewReasonClass::RollbackCandidateReady,
                "Ready: rollback target, checkpoint, preserved state, and remaining cached/revoked state were rendered.".to_string(),
            ),
            ExtensionMutationReviewActionClass::Install | ExtensionMutationReviewActionClass::Update => (
                ExtensionMutationReviewDecisionClass::ReadyForNativeMutation,
                ExtensionMutationReviewReasonClass::AllTruthRendered,
                "Ready: publisher identity, permission delta, compatibility range, activation budget, source lane, and rollback implications were rendered.".to_string(),
            ),
        }
    };

    let mutation_allowed = decision_class
        == ExtensionMutationReviewDecisionClass::ReadyForNativeMutation
        && input.surface_class == ExtensionMutationReviewSurfaceClass::NativeReviewSheet;
    let offered_actions = offered_actions_for(
        input.action_class,
        input.surface_class,
        decision_class,
        mutation_allowed,
    );
    let export_safe_summary = export_safe_summary(
        &decision_summary,
        input.action_class,
        &input.source_posture,
        compatibility_range.as_deref(),
        runtime_budget_class.as_deref(),
        &input.state_plan,
        permission_widening_count,
        permission_narrowing_count,
    );

    ExtensionMutationReviewSurface {
        record_kind: EXTENSION_MUTATION_REVIEW_SURFACE_RECORD_KIND.to_string(),
        extension_mutation_review_schema_version: EXTENSION_MUTATION_REVIEW_SCHEMA_VERSION,
        review_id: input.review_id.to_string(),
        action_class: input.action_class,
        surface_class: input.surface_class,
        subject_ref: input.subject_ref.to_string(),
        publisher_identity_ref,
        publisher_display_label,
        publisher_trust_tier_class,
        source_posture: input.source_posture,
        compatibility_range,
        compatibility_label,
        compatibility_evidence_refs,
        activation_budget,
        runtime_cost_class,
        runtime_cost_evidence_class,
        runtime_budget_class,
        activation_trigger_refs,
        activation_evidence_refs,
        effective_permission_summary_ref,
        permission_delta_ref,
        effective_permission_delta_entries,
        version_permission_delta_entries,
        re_consent_decision_class,
        re_consent_reason_class,
        permission_widening_count,
        permission_narrowing_count,
        requires_re_consent,
        state_plan: input.state_plan,
        required_disclosures,
        rendered_disclosures: input.rendered_disclosures,
        mutation_allowed,
        decision_class,
        decision_reason_class,
        offered_actions,
        export_safe_summary,
        review_event_refs: input.review_event_refs,
        decided_at: input.decided_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a metadata-safe support export from a review surface.
pub fn project_extension_mutation_review_support_export(
    surface: &ExtensionMutationReviewSurface,
    export_id: &str,
) -> ExtensionMutationReviewSupportExport {
    ExtensionMutationReviewSupportExport {
        record_kind: EXTENSION_MUTATION_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        extension_mutation_review_schema_version: EXTENSION_MUTATION_REVIEW_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        review_ref: surface.review_id.clone(),
        action_class: surface.action_class,
        subject_ref: surface.subject_ref.clone(),
        decision_class: surface.decision_class,
        decision_reason_class: surface.decision_reason_class,
        publisher_identity_ref: surface.publisher_identity_ref.clone(),
        manifest_origin_source_class: surface.source_posture.manifest_origin_source_class,
        permission_delta_ref: surface.permission_delta_ref.clone(),
        compatibility_range: surface.compatibility_range.clone(),
        runtime_budget_class: surface.runtime_budget_class.clone(),
        rollback_checkpoint_ref: surface.state_plan.rollback_checkpoint_ref.clone(),
        last_known_good_version: surface.state_plan.last_known_good_version.clone(),
        user_state_retention_class: surface.state_plan.user_state_retention_class,
        installed_artifact_disposition_class: surface
            .state_plan
            .installed_artifact_disposition_class,
        cache_disposition_class: surface.state_plan.cache_disposition_class,
        revocation_disposition_class: surface.state_plan.revocation_disposition_class,
        export_safe_summary: surface.export_safe_summary.clone(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a projected extension mutation review.
pub fn validate_extension_mutation_review_surface(
    surface: &ExtensionMutationReviewSurface,
) -> Vec<ExtensionMutationReviewFinding> {
    let mut findings = Vec::new();

    if surface.record_kind != EXTENSION_MUTATION_REVIEW_SURFACE_RECORD_KIND {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_MUTATION_REVIEW_SURFACE_RECORD_KIND}'; got {:?}",
                surface.record_kind
            ),
        ));
    }
    if surface.extension_mutation_review_schema_version != EXTENSION_MUTATION_REVIEW_SCHEMA_VERSION
    {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.schema_version_wrong",
            format!(
                "extension_mutation_review_schema_version must be {EXTENSION_MUTATION_REVIEW_SCHEMA_VERSION}; got {}",
                surface.extension_mutation_review_schema_version
            ),
        ));
    }
    if !surface.review_id.starts_with("extension_mutation_review:") {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.review_id_unprefixed",
            "review_id must start with 'extension_mutation_review:'",
        ));
    }
    if let Some(missing) =
        first_missing_disclosure(&surface.required_disclosures, &surface.rendered_disclosures)
    {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.required_disclosure_missing",
            format!("required disclosure '{missing:?}' was not rendered"),
        ));
    }
    if publisher_identity_opaque(
        &surface.publisher_identity_ref,
        &surface.publisher_display_label,
        surface.publisher_trust_tier_class,
    ) {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.publisher_identity_opaque",
            "publisher identity must be present and must not be anonymous or quarantined",
        ));
    }
    if source_lane_missing(&surface.source_posture) {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.source_lane_missing",
            "source lane truth must be present and attributed",
        ));
    }
    if matches!(
        surface.action_class,
        ExtensionMutationReviewActionClass::Install | ExtensionMutationReviewActionClass::Update
    ) && (surface
        .compatibility_range
        .as_deref()
        .is_none_or(str::is_empty)
        || surface
            .activation_budget
            .as_ref()
            .is_none_or(activation_budget_unknown)
        || surface.activation_trigger_refs.is_empty()
        || surface.activation_evidence_refs.is_empty())
    {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.compatibility_or_budget_missing",
            "install/update reviews must render compatibility range, activation budget, triggers, and evidence",
        ));
    }
    if install_or_update_permission_delta_entries_missing(surface) {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.permission_delta_missing",
            "install/update reviews must render declared-vs-effective and version-to-version permission deltas where applicable",
        ));
    }
    if surface.requires_re_consent && surface.mutation_allowed {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.reconsent_mutation_allowed",
            "permission-widening re-consent reviews must not allow mutation before acknowledgement",
        ));
    }
    if matches!(
        surface.action_class,
        ExtensionMutationReviewActionClass::Disable | ExtensionMutationReviewActionClass::Rollback
    ) && state_preservation_missing(surface.action_class, &surface.state_plan)
    {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.state_preservation_missing",
            "disable/rollback reviews must preserve user-owned state and explain remaining installed, cached, and revoked state",
        ));
    }
    if rollback_truth_missing(surface.action_class, &surface.state_plan) {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.rollback_truth_missing",
            "update/rollback reviews must cite a rollback checkpoint and rollback reviews must cite a last-known-good version",
        ));
    }
    if surface.mutation_allowed
        && surface.surface_class != ExtensionMutationReviewSurfaceClass::NativeReviewSheet
    {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review.non_native_mutation_allowed",
            "only the product-owned native review sheet may commit extension mutations",
        ));
    }

    findings
}

/// Validate a support-export projection for structural parity with its source surface.
pub fn validate_extension_mutation_review_support_export(
    export: &ExtensionMutationReviewSupportExport,
    surface: &ExtensionMutationReviewSurface,
) -> Vec<ExtensionMutationReviewFinding> {
    let mut findings = Vec::new();

    if export.record_kind != EXTENSION_MUTATION_REVIEW_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review_support_export.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_MUTATION_REVIEW_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                export.record_kind
            ),
        ));
    }
    if export.extension_mutation_review_schema_version != EXTENSION_MUTATION_REVIEW_SCHEMA_VERSION {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review_support_export.schema_version_wrong",
            "support export schema version must match the review surface schema version",
        ));
    }
    if export.review_ref != surface.review_id {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review_support_export.review_ref_drift",
            "support export review_ref must match the source review id",
        ));
    }
    if export.action_class != surface.action_class
        || export.decision_class != surface.decision_class
        || export.decision_reason_class != surface.decision_reason_class
        || export.manifest_origin_source_class
            != surface.source_posture.manifest_origin_source_class
        || export.user_state_retention_class != surface.state_plan.user_state_retention_class
    {
        findings.push(ExtensionMutationReviewFinding::new(
            "extension_mutation_review_support_export.parity_drift",
            "support export must quote the same action, decision, source lane, and state-retention vocabulary as the source review",
        ));
    }

    findings
}

fn required_disclosures_for(
    action_class: ExtensionMutationReviewActionClass,
) -> Vec<ExtensionMutationReviewDisclosureClass> {
    let mut required = vec![
        ExtensionMutationReviewDisclosureClass::PublisherIdentity,
        ExtensionMutationReviewDisclosureClass::SourceLane,
        ExtensionMutationReviewDisclosureClass::NativeAuthority,
        ExtensionMutationReviewDisclosureClass::PermissionDelta,
        ExtensionMutationReviewDisclosureClass::RollbackImplications,
        ExtensionMutationReviewDisclosureClass::StatePreservation,
    ];
    if matches!(
        action_class,
        ExtensionMutationReviewActionClass::Install | ExtensionMutationReviewActionClass::Update
    ) {
        required.push(ExtensionMutationReviewDisclosureClass::CompatibilityRange);
        required.push(ExtensionMutationReviewDisclosureClass::ActivationBudget);
    }
    required
}

fn first_missing_disclosure(
    required: &[ExtensionMutationReviewDisclosureClass],
    rendered: &[ExtensionMutationReviewDisclosureClass],
) -> Option<ExtensionMutationReviewDisclosureClass> {
    required
        .iter()
        .find(|required| !rendered.contains(required))
        .copied()
}

fn publisher_identity_opaque(
    publisher_identity_ref: &str,
    publisher_display_label: &str,
    publisher_trust_tier_class: PublisherTrustTierClass,
) -> bool {
    publisher_identity_ref.trim().is_empty()
        || publisher_display_label.trim().is_empty()
        || matches!(
            publisher_trust_tier_class,
            PublisherTrustTierClass::AnonymousPublisherClass
                | PublisherTrustTierClass::QuarantinedPublisher
        )
}

fn source_lane_missing(source_posture: &ExtensionMutationReviewSourcePosture) -> bool {
    source_posture.source_summary.trim().is_empty()
        || matches!(
            source_posture.manifest_origin_source_class,
            ManifestOriginSourceClass::UnknownSourceClass
        )
}

fn install_or_update_runtime_truth_missing(
    action_class: ExtensionMutationReviewActionClass,
    install_review_packet: Option<&InstallReviewAlphaPacketRecord>,
) -> bool {
    if !matches!(
        action_class,
        ExtensionMutationReviewActionClass::Install | ExtensionMutationReviewActionClass::Update
    ) {
        return false;
    }

    let Some(packet) = install_review_packet else {
        return true;
    };

    packet
        .compatibility
        .aureline_version_range
        .trim()
        .is_empty()
        || packet.compatibility.evidence_refs.is_empty()
        || packet.activation_budget.evidence_refs.is_empty()
        || packet.activation_budget.activation_trigger_refs.is_empty()
        || activation_budget_unknown(&packet.activation_budget.activation_budget)
        || matches!(
            packet.activation_budget.runtime_cost_class,
            RuntimeCostClass::RuntimeCostUnknownPendingEvidence
        )
        || matches!(
            packet.activation_budget.runtime_cost_evidence_class,
            RuntimeCostEvidenceClass::ActivationEvidencePacketAbsentPendingFirstSession
                | RuntimeCostEvidenceClass::BenchmarkArchiveAbsent
                | RuntimeCostEvidenceClass::SelfReportedOnlyUnverified
        )
}

fn activation_budget_unknown(budget: &ActivationBudget) -> bool {
    value_unknown(&budget.cpu)
        || value_unknown(&budget.memory)
        || value_unknown(&budget.startup_cost_ceiling)
        || budget.opt_in_feature_gates.is_empty()
        || budget
            .opt_in_feature_gates
            .iter()
            .any(|gate| value_unknown(gate))
}

fn value_unknown(value: &str) -> bool {
    let value = value.trim();
    value.is_empty() || value.eq_ignore_ascii_case("unknown")
}

fn install_or_update_upstream_pending_or_denied(
    install_review_packet: Option<&InstallReviewAlphaPacketRecord>,
) -> Option<ExtensionMutationReviewDecisionClass> {
    let packet = install_review_packet?;
    match packet.decision_class {
        InstallReviewDecisionClass::Denied => Some(ExtensionMutationReviewDecisionClass::Denied),
        InstallReviewDecisionClass::AwaitingAdminReview => {
            Some(ExtensionMutationReviewDecisionClass::AwaitingAdminReview)
        }
        InstallReviewDecisionClass::AwaitingUserReview => {
            Some(ExtensionMutationReviewDecisionClass::AwaitingUserReview)
        }
        InstallReviewDecisionClass::AdmitAfterNativeReview => None,
    }
}

fn install_or_update_permission_delta_missing(
    action_class: ExtensionMutationReviewActionClass,
    install_review_packet: Option<&InstallReviewAlphaPacketRecord>,
    permission_manifest_delta: Option<&PermissionManifestDeltaRecord>,
) -> bool {
    match action_class {
        ExtensionMutationReviewActionClass::Install => install_review_packet
            .map(|packet| packet.permission_delta_entries.is_empty())
            .unwrap_or(true),
        ExtensionMutationReviewActionClass::Update => {
            install_review_packet
                .map(|packet| packet.permission_delta_entries.is_empty())
                .unwrap_or(true)
                || permission_manifest_delta.is_none()
        }
        ExtensionMutationReviewActionClass::Disable
        | ExtensionMutationReviewActionClass::Rollback => false,
    }
}

fn install_or_update_permission_delta_entries_missing(
    surface: &ExtensionMutationReviewSurface,
) -> bool {
    match surface.action_class {
        ExtensionMutationReviewActionClass::Install => {
            surface.effective_permission_delta_entries.is_empty()
        }
        ExtensionMutationReviewActionClass::Update => {
            surface.effective_permission_delta_entries.is_empty()
                || surface.permission_delta_ref.is_none()
        }
        ExtensionMutationReviewActionClass::Disable
        | ExtensionMutationReviewActionClass::Rollback => false,
    }
}

fn state_preservation_missing(
    action_class: ExtensionMutationReviewActionClass,
    state_plan: &ExtensionMutationStatePlan,
) -> bool {
    if state_plan.state_plan_ref.trim().is_empty() || state_plan.state_summary.trim().is_empty() {
        return true;
    }
    if matches!(
        state_plan.user_state_retention_class,
        UserStateRetentionClass::StatePreservationMissing
    ) {
        return true;
    }
    if matches!(
        action_class,
        ExtensionMutationReviewActionClass::Disable | ExtensionMutationReviewActionClass::Rollback
    ) && state_plan.preserved_user_state_refs.is_empty()
    {
        return true;
    }
    false
}

fn rollback_truth_missing(
    action_class: ExtensionMutationReviewActionClass,
    state_plan: &ExtensionMutationStatePlan,
) -> bool {
    if matches!(
        state_plan.rollback_implication_class,
        RollbackImplicationClass::RollbackBlockedMissingCheckpoint
    ) {
        return true;
    }

    match action_class {
        ExtensionMutationReviewActionClass::Install => false,
        ExtensionMutationReviewActionClass::Update => state_plan.rollback_checkpoint_ref.is_none(),
        ExtensionMutationReviewActionClass::Disable => false,
        ExtensionMutationReviewActionClass::Rollback => {
            state_plan.rollback_checkpoint_ref.is_none()
                || state_plan
                    .last_known_good_version
                    .as_deref()
                    .is_none_or(str::is_empty)
        }
    }
}

fn offered_actions_for(
    action_class: ExtensionMutationReviewActionClass,
    surface_class: ExtensionMutationReviewSurfaceClass,
    decision_class: ExtensionMutationReviewDecisionClass,
    mutation_allowed: bool,
) -> Vec<ExtensionMutationReviewActionOfferClass> {
    let mut actions = vec![
        ExtensionMutationReviewActionOfferClass::OpenNativeReviewSheet,
        ExtensionMutationReviewActionOfferClass::OpenPermissionDelta,
        ExtensionMutationReviewActionOfferClass::OpenCompatibilityEvidence,
        ExtensionMutationReviewActionOfferClass::OpenActivationBudgetEvidence,
        ExtensionMutationReviewActionOfferClass::OpenRollbackDetails,
        ExtensionMutationReviewActionOfferClass::ExportSupportPacket,
    ];

    if mutation_allowed && surface_class == ExtensionMutationReviewSurfaceClass::NativeReviewSheet {
        let primary = match action_class {
            ExtensionMutationReviewActionClass::Install => {
                ExtensionMutationReviewActionOfferClass::ApproveInstall
            }
            ExtensionMutationReviewActionClass::Update => {
                ExtensionMutationReviewActionOfferClass::ApproveUpdate
            }
            ExtensionMutationReviewActionClass::Disable => {
                ExtensionMutationReviewActionOfferClass::DisableExtension
            }
            ExtensionMutationReviewActionClass::Rollback => {
                ExtensionMutationReviewActionOfferClass::ApplyRollback
            }
        };
        actions.insert(0, primary);
    }

    match decision_class {
        ExtensionMutationReviewDecisionClass::AwaitingAdminReview
        | ExtensionMutationReviewDecisionClass::Denied => {
            actions.push(ExtensionMutationReviewActionOfferClass::ConsultAdmin);
            actions.push(ExtensionMutationReviewActionOfferClass::StayPinned);
            if matches!(action_class, ExtensionMutationReviewActionClass::Disable) {
                actions.push(ExtensionMutationReviewActionOfferClass::KeepDisabled);
            }
        }
        ExtensionMutationReviewDecisionClass::AwaitingUserReview => {
            actions.push(ExtensionMutationReviewActionOfferClass::StayPinned);
        }
        ExtensionMutationReviewDecisionClass::ReadyForNativeMutation => {}
    }

    actions
}

fn export_safe_summary(
    decision_summary: &str,
    action_class: ExtensionMutationReviewActionClass,
    source_posture: &ExtensionMutationReviewSourcePosture,
    compatibility_range: Option<&str>,
    runtime_budget_class: Option<&str>,
    state_plan: &ExtensionMutationStatePlan,
    permission_widening_count: u32,
    permission_narrowing_count: u32,
) -> String {
    format!(
        "{} Action: {:?}. Source: {:?}. Source summary: {}. Compatibility range: {}. Runtime budget: {}. Permission widening={}, narrowing={}. Rollback: {:?}. Installed: {:?}. Cache: {:?}. Revocation: {:?}. State: {}.",
        decision_summary,
        action_class,
        source_posture.manifest_origin_source_class,
        source_posture.source_summary,
        compatibility_range.unwrap_or("not_applicable"),
        runtime_budget_class.unwrap_or("not_applicable"),
        permission_widening_count,
        permission_narrowing_count,
        state_plan.rollback_implication_class,
        state_plan.installed_artifact_disposition_class,
        state_plan.cache_disposition_class,
        state_plan.revocation_disposition_class,
        state_plan.state_summary
    )
}
