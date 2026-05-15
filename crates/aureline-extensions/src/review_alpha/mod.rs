//! Alpha extension review, publisher-continuity, and revocation records.
//!
//! This module is the bounded alpha owner for the extension install,
//! update, disable, and revoke review lane. It consumes the existing
//! manifest-baseline and effective-permission records from
//! [`crate::manifest_baseline`] instead of restating manifest truth, then
//! adds the review packet and projection needed by install sheets,
//! CLI/headless review, mirror review, and support exports.
//!
//! The policy-pack draft consumed by this module lives at
//! [`/schemas/policy/policy_pack_alpha.schema.json`](../../../../schemas/policy/policy_pack_alpha.schema.json).

use serde::{Deserialize, Serialize};

use aureline_support::capabilities::{
    current_capability_lifecycle_registry, CapabilityClaimValidation, CapabilityLifecycleRegistry,
    DenialReason, LifecycleState,
};

use crate::manifest_baseline::{
    EffectivePermissionBaselineRecord, EffectivePermissionDiffClass, InstallDecisionClass,
    InstallDecisionReasonClass, ManifestInstallDecisionRecord, PublisherTrustTierClass,
    RedactionClass, SummaryFreshnessClass,
};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`PublisherContinuityAlphaRecord`] payloads.
pub const PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND: &str = "publisher_continuity_alpha_record";

/// Record-kind tag carried on serialized [`RevocationAlphaRecord`] payloads.
pub const REVOCATION_ALPHA_RECORD_KIND: &str = "revocation_alpha_record";

/// Record-kind tag carried on serialized [`ExtensionReviewAlphaPacketRecord`] payloads.
pub const EXTENSION_REVIEW_ALPHA_PACKET_RECORD_KIND: &str = "extension_review_alpha_packet_record";

/// Record-kind tag carried on serialized [`ExtensionReviewAlphaProjectionRecord`] payloads.
pub const EXTENSION_REVIEW_ALPHA_PROJECTION_RECORD_KIND: &str =
    "extension_review_alpha_projection_record";

/// Schema version of the alpha review payloads.
///
/// Bumped on breaking payload changes. Additive enum values or optional
/// fields are additive-minor and require consumers to keep unknown-field
/// preservation at their boundary.
pub const REVIEW_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Review action requested against an extension install-state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewActionClass {
    /// Add a previously absent extension to a workspace, profile, or managed lane.
    Install,
    /// Move an installed extension from one version or publisher-continuity state to another.
    Update,
    /// Disable an installed extension without deleting its review or continuity evidence.
    Disable,
    /// Apply an explicit revocation or emergency-disable decision to installed state.
    Revoke,
}

/// Review decision emitted by the alpha review flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecisionClass {
    /// The mutation may proceed only through the surfaced review packet.
    AdmitAfterReview,
    /// The packet is complete, but a user acknowledgement or step-up is still required.
    AwaitingUserReview,
    /// The packet is complete, but admin or mirror authority must act first.
    AwaitingAdminReview,
    /// The mutation is refused and no install/update state may be changed.
    Denied,
}

/// Typed reason paired with [`ReviewDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecisionReasonClass {
    /// The packet is complete and no blocker was found.
    AdmittedAfterReview,
    /// The packet is complete but still requires a generic acknowledgement.
    AwaitingRequiredReview,
    /// The publisher lineage changed and must be acknowledged.
    PublisherTransferRequiresAcknowledgement,
    /// Publisher continuity is not fresh enough for the requested mutation.
    PublisherContinuityStale,
    /// The publisher is quarantined.
    PublisherQuarantined,
    /// The publisher is revoked or retired for this review path.
    PublisherRevoked,
    /// The extension artifact is revoked or emergency-disabled.
    ArtifactRevoked,
    /// Mirror or private-registry continuity is broken.
    MirrorContinuityBroken,
    /// A policy pack denies the extension for this target.
    PolicyPackDeniedExtension,
    /// A policy pack or emergency-disable bundle forces disablement.
    PolicyPackEmergencyDisable,
    /// A policy pack requires a user or admin step-up before the mutation.
    PolicyPackStepUpRequired,
    /// The publisher identity is missing or anonymous.
    PublisherIdentityRequired,
    /// The manifest origin could not be attributed.
    ManifestOriginUnknown,
    /// The manifest row is incomplete.
    ManifestScopeIncomplete,
    /// Required reviewer-visible fields were not rendered by the consumer.
    ReviewDisclosureIncomplete,
    /// The effective-permission computation observed a widening attempt.
    EffectivePermissionWideningAttempted,
    /// The disable action is admitted through review.
    DisableReviewRequired,
    /// The revoke action is admitted through review.
    RevokeReviewRequired,
    /// The review depends on stale or unavailable verification evidence.
    FreshnessFloorUnmet,
    /// The claimed lifecycle state exceeds the registry effective state.
    CapabilityLifecycleClaimRefused,
    /// The lifecycle row or registry could not be resolved.
    CapabilityLifecycleUnresolved,
}

/// Publisher continuity state visible on review and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublisherContinuityStateClass {
    /// Publisher and active signer lineage are current.
    Active,
    /// A signing-key rotation is underway and still visible to reviewers.
    KeyRotationInProgress,
    /// Ownership transfer has started but has not completed review.
    OwnershipTransferInProgress,
    /// Transfer completed, but the successor still needs first verification review.
    TransferCompletedPendingVerification,
    /// The publisher appears orphaned and requires admin or community review.
    Orphaned,
    /// The publisher succeeded a predecessor and cites that predecessor.
    Succeeded,
    /// The publisher is a reviewed fork adoption.
    ForkAdopted,
    /// The publisher identity is retired and must not be reused.
    Retired,
}

/// Subject class for a revocation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationSubjectClass {
    /// Revocation applies to a publisher continuity row.
    Publisher,
    /// Revocation applies to one extension artifact or package version.
    ExtensionArtifact,
    /// Revocation applies to a mirror promotion or mirrored catalog row.
    MirrorPromotion,
    /// Revocation applies to a policy-pack row.
    PolicyPack,
    /// Revocation applies to a signing-key ref.
    SigningKey,
}

/// Revocation state projected onto review and installed-state surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationStateClass {
    /// No revocation is known for the current continuity and freshness window.
    NoKnownRevocation,
    /// The subject is revoked and install or update must be denied.
    Revoked,
    /// The subject is quarantined pending review or remediation.
    Quarantined,
    /// A signed emergency-disable bundle or policy ratchet is active.
    EmergencyDisabled,
    /// A mirror promotion was revoked or mirror continuity is broken.
    MirrorPromotionRevoked,
    /// Verification is pending and the review cannot claim live authority.
    PendingReverify,
}

/// Source family for revocation or quarantine evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationSourceClass {
    /// Evidence came from the public registry.
    PublicRegistry,
    /// Evidence came from a private registry.
    PrivateRegistry,
    /// Evidence came from an approved mirror.
    Mirror,
    /// Evidence came from an offline bundle.
    OfflineBundle,
    /// Evidence came from an admin policy pack.
    AdminPolicyPack,
    /// Evidence came from an emergency-disable bundle.
    EmergencyDisableBundle,
}

/// Policy-pack effect projected into extension review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackEffectClass {
    /// The policy pack did not change the reviewed action.
    NoEffect,
    /// The policy pack narrowed a scope but did not block the action.
    Narrowed,
    /// The policy pack requires step-up before the action can proceed.
    StepUpRequired,
    /// The policy pack denies the reviewed extension or action.
    Denied,
    /// The policy pack applies an emergency disable.
    EmergencyDisabled,
    /// Signed publisher continuity is required before the action can proceed.
    SignedContinuityRequired,
    /// Mirror reverify is required before the action can proceed.
    MirrorReverifyRequired,
}

/// Review disclosure class that must remain visible before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisclosureClass {
    /// Publisher identity and display label are rendered.
    PublisherIdentity,
    /// Publisher continuity state and lineage refs are rendered.
    PublisherContinuity,
    /// Revocation or quarantine state is rendered.
    RevocationState,
    /// Policy-pack narrowing refs are rendered even when none applied.
    PolicyPackNarrowing,
    /// Mirror or private-registry continuity is rendered.
    MirrorContinuity,
    /// Declared-vs-effective permission diff is rendered.
    DeclaredEffectivePermissionDiff,
    /// Rollback, pin, or no-rollback posture is rendered.
    RollbackCheckpoint,
    /// Capability-lifecycle row refs are rendered.
    CapabilityLifecycle,
    /// Boundary-manifest row refs are rendered for managed or mirror claims.
    BoundaryManifest,
}

/// Durable mutation class represented by a review packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewMutationClass {
    /// No durable mutation may occur.
    NoMutation,
    /// The packet gates an install-state mutation.
    InstallStateMutation,
    /// The packet gates an update-state mutation.
    UpdateStateMutation,
    /// The packet gates a disable-state mutation.
    DisableStateMutation,
    /// The packet gates a revocation-state mutation.
    RevokeStateMutation,
}

/// Consumer surface that renders the alpha review packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSurfaceClass {
    /// Product-owned install or update review sheet.
    InstallReviewSheet,
    /// CLI or headless review projection.
    CliHeadless,
    /// Support export projection.
    SupportExport,
    /// Admin or mirror review projection.
    AdminMirrorReview,
}

/// Action offered by the first consumer projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewActionOfferClass {
    /// Approve the mutation after reading the packet.
    ApproveMutation,
    /// Open the publisher-continuity record.
    OpenPublisherContinuity,
    /// Open the declared-vs-effective permission diff.
    OpenPermissionDiff,
    /// Open applied policy-pack refs.
    OpenPolicyPack,
    /// Export an evidence-safe support packet.
    ExportSupportPacket,
    /// Ask an admin or mirror operator to review.
    ConsultAdmin,
    /// Roll back, pin, or keep the current version.
    RollBackOrPin,
    /// Remove or disable the extension.
    RemoveOrDisable,
}

/// One policy-pack application projected into an extension review decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackAlphaApplication {
    /// Opaque policy-pack ref from `policy_pack_alpha.schema.json`.
    pub policy_pack_ref: String,
    /// Policy epoch that scoped this application.
    pub policy_epoch_ref: String,
    /// Constraint rows that affected the review.
    pub constraint_refs: Vec<String>,
    /// Net effect on the reviewed action.
    pub effect_class: PolicyPackEffectClass,
    /// Whether an admin or mirror operator must act before the mutation.
    pub requires_admin_review: bool,
    /// Whether this application can be evaluated by a mirror/offline workflow.
    pub mirror_workflow_eligible: bool,
    /// Export-safe summary for UI, CLI, and support consumers.
    pub disclosure_summary: String,
}

/// Publisher continuity packet consumed by extension review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublisherContinuityAlphaRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this alpha record.
    pub review_alpha_schema_version: u32,
    /// Stable continuity packet id.
    pub continuity_id: String,
    /// Opaque publisher identity ref.
    pub publisher_identity_ref: String,
    /// Redaction-safe publisher label.
    pub publisher_display_label: String,
    /// Publisher trust tier from the manifest baseline.
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    /// Current publisher continuity state.
    pub continuity_state_class: PublisherContinuityStateClass,
    /// Active signing-key refs; raw key material is forbidden.
    pub active_signing_key_refs: Vec<String>,
    /// Optional predecessor publisher ref for succession or transfer rows.
    pub predecessor_publisher_ref: Option<String>,
    /// Optional successor publisher ref for transfer or retirement rows.
    pub successor_publisher_ref: Option<String>,
    /// Lifecycle event refs that explain continuity.
    pub lineage_event_refs: Vec<String>,
    /// Revocation or quarantine event refs tied to this publisher.
    pub revocation_event_refs: Vec<String>,
    /// Mirror-promotion refs associated with this publisher.
    pub mirror_promotion_refs: Vec<String>,
    /// Private-registry parity assertion refs.
    pub private_registry_parity_assertion_refs: Vec<String>,
    /// Freshness class for the continuity projection.
    pub freshness_class: SummaryFreshnessClass,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Revocation packet consumed by extension review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationAlphaRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this alpha record.
    pub review_alpha_schema_version: u32,
    /// Stable revocation packet id.
    pub revocation_id: String,
    /// Class of subject being revoked or checked.
    pub subject_class: RevocationSubjectClass,
    /// Opaque subject ref.
    pub subject_ref: String,
    /// Current revocation state.
    pub revocation_state_class: RevocationStateClass,
    /// Source family for the revocation evidence.
    pub revocation_source_class: RevocationSourceClass,
    /// Opaque source ref; raw policy or registry bytes are forbidden.
    pub source_ref: String,
    /// Effective timestamp for this revocation state.
    pub effective_at: String,
    /// Installed rows, manifests, or mirror rows affected by this state.
    pub blast_radius_refs: Vec<String>,
    /// Whether new installs are blocked.
    pub blocks_new_installs: bool,
    /// Whether updates are blocked.
    pub blocks_updates: bool,
    /// Whether activation or execution is blocked.
    pub blocks_activation: bool,
    /// Rollback, remove, pin, or admin repair refs.
    pub rollback_or_repair_refs: Vec<String>,
    /// Audit event refs that reconstruct the revocation.
    pub audit_event_refs: Vec<String>,
    /// Freshness class for the revocation projection.
    pub freshness_class: SummaryFreshnessClass,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Inputs supplied by an install, update, disable, or revoke caller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionReviewAlphaInput {
    /// Stable review packet id.
    pub review_id: String,
    /// Requested review action.
    pub action_class: ReviewActionClass,
    /// Opaque extension subject ref.
    pub subject_ref: String,
    /// Version requested by install or update, when applicable.
    pub requested_version: Option<String>,
    /// Current installed version, when applicable.
    pub current_version: Option<String>,
    /// Stable digest of the declared permission set.
    pub declared_permissions_digest: String,
    /// Disclosure classes the consumer rendered before asking for a decision.
    pub rendered_disclosures: Vec<ReviewDisclosureClass>,
    /// Capability-lifecycle rows consumed from the governance registry.
    pub capability_lifecycle_row_refs: Vec<String>,
    /// Lifecycle state claimed by the extension-facing surface, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_capability_lifecycle_state: Option<LifecycleState>,
    /// Boundary-manifest rows consumed for managed or mirror truth.
    pub boundary_manifest_row_refs: Vec<String>,
    /// Review event refs emitted while building the packet.
    pub review_event_refs: Vec<String>,
}

/// Review packet emitted by the alpha extension review flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionReviewAlphaPacketRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this alpha record.
    pub review_alpha_schema_version: u32,
    /// Stable review packet id.
    pub review_id: String,
    /// Requested review action.
    pub action_class: ReviewActionClass,
    /// Opaque extension subject ref.
    pub subject_ref: String,
    /// Version requested by install or update, when applicable.
    pub requested_version: Option<String>,
    /// Current installed version, when applicable.
    pub current_version: Option<String>,
    /// Manifest-baseline row ref consumed from the existing baseline lane.
    pub manifest_baseline_ref: String,
    /// Manifest install-decision row ref consumed from the existing baseline lane.
    pub manifest_install_decision_ref: String,
    /// Effective-permission summary ref consumed from the existing baseline lane.
    pub effective_permission_summary_ref: String,
    /// Stable digest of the declared permission set.
    pub declared_permissions_digest: String,
    /// Publisher-continuity packet ref rendered on the review surface.
    pub publisher_continuity_ref: String,
    /// Revocation packet ref rendered on the review surface.
    pub revocation_ref: String,
    /// Policy-pack refs that affected this review.
    pub policy_pack_application_refs: Vec<String>,
    /// Capability-lifecycle rows consumed from the governance registry.
    pub capability_lifecycle_row_refs: Vec<String>,
    /// Lifecycle state claimed by the extension-facing surface, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_capability_lifecycle_state: Option<LifecycleState>,
    /// Boundary-manifest rows consumed for managed or mirror truth.
    pub boundary_manifest_row_refs: Vec<String>,
    /// Disclosure classes required before mutation.
    pub required_disclosures: Vec<ReviewDisclosureClass>,
    /// Disclosure classes the consumer rendered.
    pub rendered_disclosures: Vec<ReviewDisclosureClass>,
    /// Durable mutation class gated by this packet.
    pub mutation_class: ReviewMutationClass,
    /// Decision emitted by the alpha review flow.
    pub decision_class: ReviewDecisionClass,
    /// Typed reason paired with the decision.
    pub decision_reason_class: ReviewDecisionReasonClass,
    /// Export-safe decision summary.
    pub decision_summary: String,
    /// Review event refs emitted while building the packet.
    pub review_event_refs: Vec<String>,
    /// Decision timestamp.
    pub decided_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// First consumer projection for review sheets, CLI, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionReviewAlphaProjectionRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this alpha record.
    pub review_alpha_schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Review packet ref.
    pub review_ref: String,
    /// Consumer surface that rendered the projection.
    pub surface_class: ReviewSurfaceClass,
    /// Publisher identity rendered to the user, admin, or support reader.
    pub visible_publisher_identity_ref: String,
    /// Publisher continuity state rendered by the consumer.
    pub visible_publisher_continuity_state: PublisherContinuityStateClass,
    /// Revocation state rendered by the consumer.
    pub visible_revocation_state: RevocationStateClass,
    /// Policy-pack refs rendered by the consumer.
    pub visible_policy_pack_refs: Vec<String>,
    /// Whether the projection blocks the requested mutation.
    pub blocked_mutation: bool,
    /// Actions made available by the consumer.
    pub offered_actions: Vec<ReviewActionOfferClass>,
    /// Export-safe summary for the consumer.
    pub export_safe_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by review-alpha validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewAlphaFinding {
    /// Stable validation check id.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
}

impl ReviewAlphaFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate an extension install, update, disable, or revoke review packet.
///
/// The function is intentionally deterministic. It never admits install or
/// update when disclosure is incomplete, publisher/revocation state blocks
/// the row, a policy pack denies the row, or effective-permission widening
/// was observed.
pub fn evaluate_extension_review_alpha(
    input: ExtensionReviewAlphaInput,
    manifest_decision: &ManifestInstallDecisionRecord,
    effective_permission: &EffectivePermissionBaselineRecord,
    continuity: &PublisherContinuityAlphaRecord,
    revocation: &RevocationAlphaRecord,
    policy_applications: &[PolicyPackAlphaApplication],
    decided_at: &str,
) -> ExtensionReviewAlphaPacketRecord {
    let required_disclosures = required_disclosures_for(input.action_class);
    let mutation_class = mutation_class_for(input.action_class);
    let missing_disclosure =
        first_missing_disclosure(&required_disclosures, &input.rendered_disclosures);

    let policy_pack_refs = policy_applications
        .iter()
        .map(|application| application.policy_pack_ref.clone())
        .collect::<Vec<_>>();

    let (decision_class, decision_reason_class, decision_summary) = if let Some(missing) =
        missing_disclosure
    {
        (
            ReviewDecisionClass::Denied,
            ReviewDecisionReasonClass::ReviewDisclosureIncomplete,
            format!("Denied: review surface did not render required disclosure '{missing:?}'."),
        )
    } else if let Some((reason, summary)) = capability_lifecycle_claim_denial(&input) {
        (ReviewDecisionClass::Denied, reason, summary)
    } else if matches!(
        input.action_class,
        ReviewActionClass::Disable | ReviewActionClass::Revoke
    ) {
        decide_disable_or_revoke(input.action_class, policy_applications)
    } else if let Some((reason, summary)) = revocation_blocks_install_or_update(revocation) {
        (ReviewDecisionClass::Denied, reason, summary.to_string())
    } else if let Some((reason, summary)) = policy_blocks_install_or_update(policy_applications) {
        (ReviewDecisionClass::Denied, reason, summary.to_string())
    } else if let Some((reason, summary)) =
        manifest_denial_blocks_install_or_update(manifest_decision)
    {
        (ReviewDecisionClass::Denied, reason, summary.to_string())
    } else if effective_permission.widening_attempted_blocked_count > 0
        || effective_permission
            .declared_vs_effective_diff
            .iter()
            .any(|entry| {
                matches!(
                    entry.diff_class,
                    EffectivePermissionDiffClass::WideningAttemptedBlocked
                )
            })
    {
        (
            ReviewDecisionClass::Denied,
            ReviewDecisionReasonClass::EffectivePermissionWideningAttempted,
            "Denied: effective-permission summary blocked a widening attempt.".to_string(),
        )
    } else if let Some((decision, reason, summary)) =
        continuity_requires_review_or_denial(continuity)
    {
        (decision, reason, summary.to_string())
    } else if policy_applications.iter().any(|application| {
        matches!(
            application.effect_class,
            PolicyPackEffectClass::StepUpRequired
                | PolicyPackEffectClass::SignedContinuityRequired
                | PolicyPackEffectClass::MirrorReverifyRequired
        )
    }) {
        let decision = if policy_applications
            .iter()
            .any(|application| application.requires_admin_review)
        {
            ReviewDecisionClass::AwaitingAdminReview
        } else {
            ReviewDecisionClass::AwaitingUserReview
        };
        (
            decision,
            ReviewDecisionReasonClass::PolicyPackStepUpRequired,
            "Awaiting review: policy pack requires step-up, signed continuity, or mirror reverify."
                .to_string(),
        )
    } else if matches!(
        manifest_decision.install_decision_class,
        InstallDecisionClass::AdmitWithStepUp
    ) {
        (
            ReviewDecisionClass::AwaitingUserReview,
            ReviewDecisionReasonClass::PolicyPackStepUpRequired,
            "Awaiting review: manifest decision requires step-up.".to_string(),
        )
    } else if matches!(
        manifest_decision.install_decision_class,
        InstallDecisionClass::ReviewOnly
    ) {
        (
            ReviewDecisionClass::AwaitingUserReview,
            ReviewDecisionReasonClass::AwaitingRequiredReview,
            "Awaiting review: manifest decision is review-only.".to_string(),
        )
    } else {
        (
                ReviewDecisionClass::AdmitAfterReview,
                ReviewDecisionReasonClass::AdmittedAfterReview,
                "Admitted after review: all required trust, continuity, revocation, policy, and permission disclosures were rendered.".to_string(),
            )
    };

    ExtensionReviewAlphaPacketRecord {
        record_kind: EXTENSION_REVIEW_ALPHA_PACKET_RECORD_KIND.to_string(),
        review_alpha_schema_version: REVIEW_ALPHA_SCHEMA_VERSION,
        review_id: input.review_id,
        action_class: input.action_class,
        subject_ref: input.subject_ref,
        requested_version: input.requested_version,
        current_version: input.current_version,
        manifest_baseline_ref: manifest_decision.manifest_baseline_ref.clone(),
        manifest_install_decision_ref: format!(
            "manifest_install_decision:{}",
            manifest_decision.manifest_baseline_ref
        ),
        effective_permission_summary_ref: effective_permission.manifest_baseline_ref.clone(),
        declared_permissions_digest: input.declared_permissions_digest,
        publisher_continuity_ref: continuity.continuity_id.clone(),
        revocation_ref: revocation.revocation_id.clone(),
        policy_pack_application_refs: policy_pack_refs,
        capability_lifecycle_row_refs: input.capability_lifecycle_row_refs,
        claimed_capability_lifecycle_state: input.claimed_capability_lifecycle_state,
        boundary_manifest_row_refs: input.boundary_manifest_row_refs,
        required_disclosures,
        rendered_disclosures: input.rendered_disclosures,
        mutation_class: if matches!(decision_class, ReviewDecisionClass::Denied) {
            ReviewMutationClass::NoMutation
        } else {
            mutation_class
        },
        decision_class,
        decision_reason_class,
        decision_summary,
        review_event_refs: input.review_event_refs,
        decided_at: decided_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a review packet into the first consumer surface.
///
/// The projection is intentionally export-safe: it repeats only the visible
/// trust, continuity, revocation, and policy refs needed by UI, CLI, and
/// support surfaces. It does not embed raw manifests, raw signatures, raw
/// policy bodies, paths, tokens, or artifact bytes.
pub fn project_review_alpha_surface(
    packet: &ExtensionReviewAlphaPacketRecord,
    continuity: &PublisherContinuityAlphaRecord,
    revocation: &RevocationAlphaRecord,
    policy_applications: &[PolicyPackAlphaApplication],
    surface_class: ReviewSurfaceClass,
) -> ExtensionReviewAlphaProjectionRecord {
    let blocked_mutation = matches!(packet.decision_class, ReviewDecisionClass::Denied);
    let mut offered_actions = vec![
        ReviewActionOfferClass::OpenPublisherContinuity,
        ReviewActionOfferClass::OpenPermissionDiff,
        ReviewActionOfferClass::OpenPolicyPack,
        ReviewActionOfferClass::ExportSupportPacket,
    ];

    match packet.decision_class {
        ReviewDecisionClass::AdmitAfterReview => {
            offered_actions.insert(0, ReviewActionOfferClass::ApproveMutation);
        }
        ReviewDecisionClass::AwaitingUserReview => {
            offered_actions.push(ReviewActionOfferClass::RollBackOrPin);
        }
        ReviewDecisionClass::AwaitingAdminReview => {
            offered_actions.push(ReviewActionOfferClass::ConsultAdmin);
        }
        ReviewDecisionClass::Denied => {
            offered_actions.push(ReviewActionOfferClass::RemoveOrDisable);
            offered_actions.push(ReviewActionOfferClass::ConsultAdmin);
        }
    }

    ExtensionReviewAlphaProjectionRecord {
        record_kind: EXTENSION_REVIEW_ALPHA_PROJECTION_RECORD_KIND.to_string(),
        review_alpha_schema_version: REVIEW_ALPHA_SCHEMA_VERSION,
        projection_id: format!("review_projection:{}:{surface_class:?}", packet.review_id),
        review_ref: packet.review_id.clone(),
        surface_class,
        visible_publisher_identity_ref: continuity.publisher_identity_ref.clone(),
        visible_publisher_continuity_state: continuity.continuity_state_class,
        visible_revocation_state: revocation.revocation_state_class,
        visible_policy_pack_refs: policy_applications
            .iter()
            .map(|application| application.policy_pack_ref.clone())
            .collect(),
        blocked_mutation,
        offered_actions,
        export_safe_summary: format!(
            "{} Publisher continuity: {:?}. Revocation: {:?}.",
            packet.decision_summary,
            continuity.continuity_state_class,
            revocation.revocation_state_class
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validates extension capability lifecycle refs against a claimed state.
pub fn validate_extension_capability_lifecycle_claim(
    row_refs: &[String],
    claimed_lifecycle_state: LifecycleState,
    registry: &CapabilityLifecycleRegistry,
) -> CapabilityClaimValidation {
    registry.validate_claim(row_refs, claimed_lifecycle_state)
}

/// Validate structural invariants for a publisher-continuity alpha record.
pub fn validate_publisher_continuity_alpha_record(
    record: &PublisherContinuityAlphaRecord,
) -> Vec<ReviewAlphaFinding> {
    let mut findings = Vec::new();

    if record.record_kind != PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.record_kind_wrong",
            format!(
                "record_kind must be '{PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.review_alpha_schema_version != REVIEW_ALPHA_SCHEMA_VERSION {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.schema_version_wrong",
            format!(
                "review_alpha_schema_version must be {REVIEW_ALPHA_SCHEMA_VERSION}; got {}",
                record.review_alpha_schema_version
            ),
        ));
    }
    if !record.continuity_id.starts_with("publisher_continuity:") {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.id_unprefixed",
            "continuity_id must start with 'publisher_continuity:'",
        ));
    }
    if record.publisher_identity_ref.trim().is_empty() {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.publisher_identity_required",
            "publisher_identity_ref must be present",
        ));
    }
    if record.publisher_display_label.trim().is_empty() {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.publisher_label_required",
            "publisher_display_label must be present",
        ));
    }
    if matches!(
        record.continuity_state_class,
        PublisherContinuityStateClass::Active
            | PublisherContinuityStateClass::Succeeded
            | PublisherContinuityStateClass::ForkAdopted
    ) && record.active_signing_key_refs.is_empty()
    {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.signing_key_required",
            "active continuity states must carry at least one signing-key ref",
        ));
    }
    if matches!(
        record.continuity_state_class,
        PublisherContinuityStateClass::OwnershipTransferInProgress
            | PublisherContinuityStateClass::TransferCompletedPendingVerification
            | PublisherContinuityStateClass::Retired
    ) && record.successor_publisher_ref.is_none()
    {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.publisher_continuity.successor_required",
            "transfer or retired continuity states must cite a successor publisher ref",
        ));
    }

    findings
}

/// Validate structural invariants for a revocation alpha record.
pub fn validate_revocation_alpha_record(record: &RevocationAlphaRecord) -> Vec<ReviewAlphaFinding> {
    let mut findings = Vec::new();

    if record.record_kind != REVOCATION_ALPHA_RECORD_KIND {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.revocation.record_kind_wrong",
            format!(
                "record_kind must be '{REVOCATION_ALPHA_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.review_alpha_schema_version != REVIEW_ALPHA_SCHEMA_VERSION {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.revocation.schema_version_wrong",
            format!(
                "review_alpha_schema_version must be {REVIEW_ALPHA_SCHEMA_VERSION}; got {}",
                record.review_alpha_schema_version
            ),
        ));
    }
    if !record.revocation_id.starts_with("revocation:") {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.revocation.id_unprefixed",
            "revocation_id must start with 'revocation:'",
        ));
    }
    if record.subject_ref.trim().is_empty() {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.revocation.subject_required",
            "subject_ref must be present",
        ));
    }
    if record.source_ref.trim().is_empty() {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.revocation.source_required",
            "source_ref must be present",
        ));
    }
    if !matches!(
        record.revocation_state_class,
        RevocationStateClass::NoKnownRevocation
    ) && record.audit_event_refs.is_empty()
    {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.revocation.audit_event_required",
            "active revocation states must cite at least one audit event ref",
        ));
    }

    findings
}

/// Validate structural invariants for a review alpha packet.
pub fn validate_extension_review_alpha_packet(
    packet: &ExtensionReviewAlphaPacketRecord,
) -> Vec<ReviewAlphaFinding> {
    let mut findings = Vec::new();

    if packet.record_kind != EXTENSION_REVIEW_ALPHA_PACKET_RECORD_KIND {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_REVIEW_ALPHA_PACKET_RECORD_KIND}'; got {:?}",
                packet.record_kind
            ),
        ));
    }
    if packet.review_alpha_schema_version != REVIEW_ALPHA_SCHEMA_VERSION {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.schema_version_wrong",
            format!(
                "review_alpha_schema_version must be {REVIEW_ALPHA_SCHEMA_VERSION}; got {}",
                packet.review_alpha_schema_version
            ),
        ));
    }
    if !packet.review_id.starts_with("review_alpha:") {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.id_unprefixed",
            "review_id must start with 'review_alpha:'",
        ));
    }
    if let Some(missing) =
        first_missing_disclosure(&packet.required_disclosures, &packet.rendered_disclosures)
    {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.required_disclosure_missing",
            format!("required disclosure '{missing:?}' was not rendered"),
        ));
    }
    if packet.publisher_continuity_ref.trim().is_empty() {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.publisher_continuity_ref_required",
            "publisher_continuity_ref must be present",
        ));
    }
    if packet.revocation_ref.trim().is_empty() {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.revocation_ref_required",
            "revocation_ref must be present",
        ));
    }
    if matches!(packet.decision_class, ReviewDecisionClass::Denied)
        && !matches!(packet.mutation_class, ReviewMutationClass::NoMutation)
        && matches!(
            packet.action_class,
            ReviewActionClass::Install | ReviewActionClass::Update
        )
    {
        findings.push(ReviewAlphaFinding::new(
            "review_alpha.packet.denied_install_or_update_cannot_mutate",
            "denied install/update packets must carry no mutation",
        ));
    }

    findings
}

fn required_disclosures_for(action_class: ReviewActionClass) -> Vec<ReviewDisclosureClass> {
    let mut disclosures = vec![
        ReviewDisclosureClass::PublisherIdentity,
        ReviewDisclosureClass::PublisherContinuity,
        ReviewDisclosureClass::RevocationState,
        ReviewDisclosureClass::PolicyPackNarrowing,
        ReviewDisclosureClass::DeclaredEffectivePermissionDiff,
        ReviewDisclosureClass::CapabilityLifecycle,
        ReviewDisclosureClass::BoundaryManifest,
    ];
    if matches!(
        action_class,
        ReviewActionClass::Install | ReviewActionClass::Update
    ) {
        disclosures.push(ReviewDisclosureClass::MirrorContinuity);
    }
    disclosures.push(ReviewDisclosureClass::RollbackCheckpoint);
    disclosures
}

fn first_missing_disclosure(
    required: &[ReviewDisclosureClass],
    rendered: &[ReviewDisclosureClass],
) -> Option<ReviewDisclosureClass> {
    required
        .iter()
        .find(|required| !rendered.contains(required))
        .copied()
}

fn capability_lifecycle_claim_denial(
    input: &ExtensionReviewAlphaInput,
) -> Option<(ReviewDecisionReasonClass, String)> {
    let claimed_lifecycle_state = input.claimed_capability_lifecycle_state?;
    let registry = match current_capability_lifecycle_registry() {
        Ok(registry) => registry,
        Err(_) => {
            return Some((
                ReviewDecisionReasonClass::CapabilityLifecycleUnresolved,
                "Denied: capability lifecycle registry could not be resolved.".to_string(),
            ));
        }
    };
    let validation = validate_extension_capability_lifecycle_claim(
        &input.capability_lifecycle_row_refs,
        claimed_lifecycle_state,
        &registry,
    );
    if validation.is_valid() {
        return None;
    }

    let reason = match validation
        .failures()
        .first()
        .map(|failure| failure.denial_reason())
    {
        Some(DenialReason::LifecycleStateUnresolved) | None => {
            ReviewDecisionReasonClass::CapabilityLifecycleUnresolved
        }
        Some(_) => ReviewDecisionReasonClass::CapabilityLifecycleClaimRefused,
    };
    Some((
        reason,
        format!(
            "Denied: {}",
            validation
                .first_failure_summary()
                .unwrap_or("capability lifecycle claim is not admissible")
        ),
    ))
}

fn mutation_class_for(action_class: ReviewActionClass) -> ReviewMutationClass {
    match action_class {
        ReviewActionClass::Install => ReviewMutationClass::InstallStateMutation,
        ReviewActionClass::Update => ReviewMutationClass::UpdateStateMutation,
        ReviewActionClass::Disable => ReviewMutationClass::DisableStateMutation,
        ReviewActionClass::Revoke => ReviewMutationClass::RevokeStateMutation,
    }
}

fn decide_disable_or_revoke(
    action_class: ReviewActionClass,
    policy_applications: &[PolicyPackAlphaApplication],
) -> (ReviewDecisionClass, ReviewDecisionReasonClass, String) {
    if policy_applications
        .iter()
        .any(|application| application.requires_admin_review)
    {
        return (
            ReviewDecisionClass::AwaitingAdminReview,
            ReviewDecisionReasonClass::PolicyPackStepUpRequired,
            "Awaiting admin review: policy pack owns this disable or revoke action.".to_string(),
        );
    }

    match action_class {
        ReviewActionClass::Disable => (
            ReviewDecisionClass::AdmitAfterReview,
            ReviewDecisionReasonClass::DisableReviewRequired,
            "Admitted after review: disable mutation is explicit and evidence-backed.".to_string(),
        ),
        ReviewActionClass::Revoke => (
            ReviewDecisionClass::AdmitAfterReview,
            ReviewDecisionReasonClass::RevokeReviewRequired,
            "Admitted after review: revocation mutation is explicit and evidence-backed."
                .to_string(),
        ),
        ReviewActionClass::Install | ReviewActionClass::Update => {
            unreachable!("disable/revoke decision helper must not be called for install/update")
        }
    }
}

fn revocation_blocks_install_or_update(
    revocation: &RevocationAlphaRecord,
) -> Option<(ReviewDecisionReasonClass, &'static str)> {
    match revocation.revocation_state_class {
        RevocationStateClass::NoKnownRevocation => None,
        RevocationStateClass::Quarantined => Some((
            ReviewDecisionReasonClass::PublisherQuarantined,
            "Denied: publisher or artifact is quarantined.",
        )),
        RevocationStateClass::Revoked => {
            if matches!(revocation.subject_class, RevocationSubjectClass::Publisher) {
                Some((
                    ReviewDecisionReasonClass::PublisherRevoked,
                    "Denied: publisher is revoked or retired.",
                ))
            } else {
                Some((
                    ReviewDecisionReasonClass::ArtifactRevoked,
                    "Denied: extension artifact is revoked.",
                ))
            }
        }
        RevocationStateClass::EmergencyDisabled => Some((
            ReviewDecisionReasonClass::PolicyPackEmergencyDisable,
            "Denied: emergency-disable policy is active.",
        )),
        RevocationStateClass::MirrorPromotionRevoked => Some((
            ReviewDecisionReasonClass::MirrorContinuityBroken,
            "Denied: mirror promotion was revoked or mirror continuity is broken.",
        )),
        RevocationStateClass::PendingReverify => Some((
            ReviewDecisionReasonClass::FreshnessFloorUnmet,
            "Denied: revocation or signature reverify is pending.",
        )),
    }
}

fn policy_blocks_install_or_update(
    policy_applications: &[PolicyPackAlphaApplication],
) -> Option<(ReviewDecisionReasonClass, &'static str)> {
    for application in policy_applications {
        match application.effect_class {
            PolicyPackEffectClass::Denied => {
                return Some((
                    ReviewDecisionReasonClass::PolicyPackDeniedExtension,
                    "Denied: policy pack denies this extension or action.",
                ));
            }
            PolicyPackEffectClass::EmergencyDisabled => {
                return Some((
                    ReviewDecisionReasonClass::PolicyPackEmergencyDisable,
                    "Denied: policy pack applies emergency disablement.",
                ));
            }
            PolicyPackEffectClass::NoEffect
            | PolicyPackEffectClass::Narrowed
            | PolicyPackEffectClass::StepUpRequired
            | PolicyPackEffectClass::SignedContinuityRequired
            | PolicyPackEffectClass::MirrorReverifyRequired => {}
        }
    }
    None
}

fn manifest_denial_blocks_install_or_update(
    manifest_decision: &ManifestInstallDecisionRecord,
) -> Option<(ReviewDecisionReasonClass, &'static str)> {
    if !matches!(
        manifest_decision.install_decision_class,
        InstallDecisionClass::Denied
    ) {
        return None;
    }

    let reason = match manifest_decision.install_decision_reason_class {
        InstallDecisionReasonClass::PublisherAnonymous
        | InstallDecisionReasonClass::PublisherIdentityRequired => (
            ReviewDecisionReasonClass::PublisherIdentityRequired,
            "Denied: publisher identity is missing or anonymous.",
        ),
        InstallDecisionReasonClass::PublisherQuarantined => (
            ReviewDecisionReasonClass::PublisherQuarantined,
            "Denied: publisher is quarantined.",
        ),
        InstallDecisionReasonClass::PublisherLifecycleRetired => (
            ReviewDecisionReasonClass::PublisherRevoked,
            "Denied: publisher lifecycle is retired.",
        ),
        InstallDecisionReasonClass::ExtensionLifecycleRetired => (
            ReviewDecisionReasonClass::ArtifactRevoked,
            "Denied: extension lifecycle is retired or quarantined.",
        ),
        InstallDecisionReasonClass::ManifestOriginUnknown => (
            ReviewDecisionReasonClass::ManifestOriginUnknown,
            "Denied: manifest origin source could not be attributed.",
        ),
        InstallDecisionReasonClass::DeclaredPermissionRationaleRequired
        | InstallDecisionReasonClass::ManifestScopeIncomplete
        | InstallDecisionReasonClass::LifecycleStateUnknownClass => (
            ReviewDecisionReasonClass::ManifestScopeIncomplete,
            "Denied: manifest scope is incomplete.",
        ),
        InstallDecisionReasonClass::EffectivePermissionWideningAttempted => (
            ReviewDecisionReasonClass::EffectivePermissionWideningAttempted,
            "Denied: requested permissions were not declared in manifest scope.",
        ),
        InstallDecisionReasonClass::AdmittedNoViolation
        | InstallDecisionReasonClass::StepUpRequiredByPolicyPack
        | InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher => (
            ReviewDecisionReasonClass::AwaitingRequiredReview,
            "Denied: manifest decision is inconsistent with its reason.",
        ),
    };

    Some(reason)
}

fn continuity_requires_review_or_denial(
    continuity: &PublisherContinuityAlphaRecord,
) -> Option<(ReviewDecisionClass, ReviewDecisionReasonClass, &'static str)> {
    match continuity.continuity_state_class {
        PublisherContinuityStateClass::Active
        | PublisherContinuityStateClass::Succeeded
        | PublisherContinuityStateClass::ForkAdopted => None,
        PublisherContinuityStateClass::Retired => Some((
            ReviewDecisionClass::Denied,
            ReviewDecisionReasonClass::PublisherRevoked,
            "Denied: publisher continuity is retired.",
        )),
        PublisherContinuityStateClass::Orphaned => Some((
            ReviewDecisionClass::AwaitingAdminReview,
            ReviewDecisionReasonClass::PublisherContinuityStale,
            "Awaiting admin review: publisher continuity is orphaned.",
        )),
        PublisherContinuityStateClass::KeyRotationInProgress
        | PublisherContinuityStateClass::OwnershipTransferInProgress
        | PublisherContinuityStateClass::TransferCompletedPendingVerification => Some((
            ReviewDecisionClass::AwaitingUserReview,
            ReviewDecisionReasonClass::PublisherTransferRequiresAcknowledgement,
            "Awaiting review: publisher continuity changed and must be acknowledged.",
        )),
    }
}
