//! Finalize signed administrative bundle review, offline-entitlement grace,
//! emergency-disable ratchets, and trust-root rotation across managed,
//! mirrored, fleet, and air-gapped paths.
//!
//! This module produces one inspectable proof packet for the signed artifact
//! lane that downstream shell, admin, support-export, and headless review
//! surfaces can reuse instead of minting delivery-path-specific truth.
//!
//! The packet proves that:
//!
//! 1. Admin policy bundles, offline entitlement snapshots, emergency-disable
//!    bundles, and trust-root or signer updates share one signed review model.
//! 2. Managed pull, mirror publication, file import, MDM or fleet drop, air-gap
//!    transfer, and last-known-good cache paths preserve the same signature,
//!    epoch, scope, supersedes or revokes, and delivery-source vocabulary.
//! 3. Stale last-known-good state preserves local-safe continuity while new
//!    privileged operations narrow until fresh verification lands.
//! 4. Emergency disable and trust-root rotation remain explicit bundle classes
//!    rather than hidden service-side behavior.
//! 5. Bundle lifecycle events for apply, supersede, revoke, signer rotation
//!    review, and emergency-disable activation remain export-safe and portable.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md`
//! - Artifact: `artifacts/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md`
//! - Contract ref: [`SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use aureline_auth::offline_entitlements::{
    audit_offline_entitlement_verifier_beta_rows, seeded_offline_entitlement_verifier_beta_page,
    OfflineEntitlementVerifierBetaDefectKind, OfflineEntitlementVerifierBetaPage,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION: u32 = 2;

/// Shared contract ref consumed by every record in this module.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF: &str =
    "policy:signed_policy_bundle_finalize:v2";

/// Record-kind tag for [`FinalizeSignedPolicyBundlePage`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_PAGE_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_page_record";

/// Record-kind tag for [`FinalizeSignedPolicyBundleRow`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_ROW_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_row_record";

/// Record-kind tag for [`BundleLifecycleAuditEvent`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_LIFECYCLE_EVENT_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_lifecycle_event_record";

/// Record-kind tag for [`FinalizeSignedPolicyBundleDefect`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_DEFECT_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_defect_record";

/// Record-kind tag for [`FinalizeSignedPolicyBundleSummary`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_SUMMARY_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_summary_record";

/// Record-kind tag for [`FinalizeSignedPolicyBundleSupportExport`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_DOC_REF: &str =
    "docs/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md";

/// Repo-relative path of the artifact summary for this lane.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md";

/// Upstream offline-entitlement verifier contract ref.
pub const OFFLINE_ENTITLEMENT_VERIFIER_CONTRACT_REF: &str =
    "security:offline_entitlement_verifier_beta:v1";

/// Import flow that produced the bundle under inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleImportFlowClass {
    /// Bundle fetched from the live signed origin.
    Online,
    /// Bundle served from a declared signed mirror.
    Mirror,
    /// Bundle imported as a signed file by an admin action.
    ManualImport,
    /// Bundle transferred via an air-gapped signed media exchange.
    AirGapped,
    /// Bundle used from a last-known-good cache within its declared stale
    /// posture.
    OfflineGrace,
}

impl BundleImportFlowClass {
    /// All required import flows in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Online,
        Self::Mirror,
        Self::ManualImport,
        Self::AirGapped,
        Self::OfflineGrace,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Mirror => "mirror",
            Self::ManualImport => "manual_import",
            Self::AirGapped => "air_gapped",
            Self::OfflineGrace => "offline_grace",
        }
    }

    /// True when stale rows on this flow must declare grace-window bounds.
    pub const fn requires_grace_window_when_stale(self) -> bool {
        matches!(self, Self::AirGapped | Self::OfflineGrace)
    }
}

/// Bundle kind frozen by the finalize packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleKindClass {
    /// Signed administrative policy bundle.
    AdminPolicyBundle,
    /// Signed offline entitlement or org snapshot.
    EntitlementSnapshot,
    /// Signed emergency-disable ratchet.
    EmergencyDisableBundle,
    /// Trust-root or signer-update review bundle.
    TrustRootSignerUpdate,
}

impl BundleKindClass {
    /// All required bundle kinds in canonical order.
    pub const ALL: [Self; 4] = [
        Self::AdminPolicyBundle,
        Self::EntitlementSnapshot,
        Self::EmergencyDisableBundle,
        Self::TrustRootSignerUpdate,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminPolicyBundle => "admin_policy_bundle",
            Self::EntitlementSnapshot => "entitlement_snapshot",
            Self::EmergencyDisableBundle => "emergency_disable_bundle",
            Self::TrustRootSignerUpdate => "trust_root_signer_update",
        }
    }

    fn signer_ref(self) -> &'static str {
        match self {
            Self::AdminPolicyBundle => "signer:policy-service:primary",
            Self::EntitlementSnapshot => "signer:identity-service:primary",
            Self::EmergencyDisableBundle => "signer:security-response:primary",
            Self::TrustRootSignerUpdate => "signer:trust-root-council:primary",
        }
    }

    fn issuer_ref(self) -> &'static str {
        match self {
            Self::AdminPolicyBundle => "issuer:policy-service:admin-bundle",
            Self::EntitlementSnapshot => "issuer:identity-service:offline-entitlement",
            Self::EmergencyDisableBundle => "issuer:security-response:disable-bundle",
            Self::TrustRootSignerUpdate => "issuer:trust-root-council:rotation-review",
        }
    }

    fn scope_ref(self) -> &'static str {
        match self {
            Self::AdminPolicyBundle => "scope:tenant:managed-alpha",
            Self::EntitlementSnapshot => "scope:tenant:managed-alpha",
            Self::EmergencyDisableBundle => "scope:artifact-supply-chain:global",
            Self::TrustRootSignerUpdate => "scope:signing-root:global",
        }
    }

    fn affected_surfaces(self) -> Vec<&'static str> {
        match self {
            Self::AdminPolicyBundle => {
                vec![
                    "policy_inspect_panel",
                    "admin_trust_center",
                    "settings_lock_strip",
                ]
            }
            Self::EntitlementSnapshot => {
                vec![
                    "entitlement_inspect_panel",
                    "seat_status_indicator",
                    "usage_export_card",
                ]
            }
            Self::EmergencyDisableBundle => vec![
                "emergency_action_banner",
                "extension_health_panel",
                "provider_route_guardrail",
            ],
            Self::TrustRootSignerUpdate => vec![
                "trust_root_review_panel",
                "mirror_admission_strip",
                "bundle_verify_history",
            ],
        }
    }

    fn blocked_capability_consequences(self) -> Vec<&'static str> {
        match self {
            Self::AdminPolicyBundle => vec![
                "managed_admin_overrides (fresh review required)",
                "fresh policy widening (paused until verification)",
            ],
            Self::EntitlementSnapshot => vec![
                "seat-bound managed features (paused until refresh)",
                "quota-bearing managed operations (paused until refresh)",
            ],
            Self::EmergencyDisableBundle => vec![
                "disabled extensions remain blocked",
                "blocked provider routes remain blocked",
            ],
            Self::TrustRootSignerUpdate => vec![
                "new privileged imports denied until trust-root review completes",
                "signer-rotated bundle apply denied until continuity is reviewed",
            ],
        }
    }
}

/// Delivery source preserved across managed, mirrored, fleet, and offline paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleDeliverySourceClass {
    /// Live managed pull from the signed origin.
    ManagedPull,
    /// Mirror publication or mirror-sync distribution.
    MirrorPublication,
    /// Local operator file import.
    FileImport,
    /// Device-management or fleet drop.
    MdmFleetDrop,
    /// Air-gapped transfer path.
    AirGapTransfer,
    /// Last-known-good cache selection.
    LastKnownGoodCache,
}

impl BundleDeliverySourceClass {
    /// All required delivery sources in canonical order.
    pub const ALL: [Self; 6] = [
        Self::ManagedPull,
        Self::MirrorPublication,
        Self::FileImport,
        Self::MdmFleetDrop,
        Self::AirGapTransfer,
        Self::LastKnownGoodCache,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPull => "managed_pull",
            Self::MirrorPublication => "mirror_publication",
            Self::FileImport => "file_import",
            Self::MdmFleetDrop => "mdm_fleet_drop",
            Self::AirGapTransfer => "air_gap_transfer",
            Self::LastKnownGoodCache => "last_known_good_cache",
        }
    }
}

/// Offline-grace posture for the bundle under inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GracePostureClass {
    /// Bundle is current and verified; no grace extension is active.
    NotInGrace,
    /// Bundle is within its declared grace window.
    InGrace,
    /// Bundle's grace window has expired and only local-safe continuity remains.
    GraceExpired,
}

impl GracePostureClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotInGrace => "not_in_grace",
            Self::InGrace => "in_grace",
            Self::GraceExpired => "grace_expired",
        }
    }

    /// True when the row is operating outside its fresh verification window.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::InGrace | Self::GraceExpired)
    }
}

/// Expiry guidance surfaced on the bundle envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryGuidanceClass {
    /// Bundle is current and verified.
    CurrentAndVerified,
    /// Bundle must refresh before `valid_until`.
    RefreshBeforeExpiry,
    /// Bundle remains in a bounded grace window.
    InGraceUntilRefresh,
    /// Last-known-good may continue only for local-safe continuity.
    LastKnownGoodOnly,
    /// Signed successor or expiry must resolve the bundle.
    SupersedeOrExpire,
    /// Rotation overlap or explicit trust review is required.
    RotationWindowRequired,
}

impl ExpiryGuidanceClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentAndVerified => "current_and_verified",
            Self::RefreshBeforeExpiry => "refresh_before_expiry",
            Self::InGraceUntilRefresh => "in_grace_until_refresh",
            Self::LastKnownGoodOnly => "last_known_good_only",
            Self::SupersedeOrExpire => "supersede_or_expire",
            Self::RotationWindowRequired => "rotation_window_required",
        }
    }
}

/// Privileged-operation posture while this bundle state is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivilegedOperationPostureClass {
    /// Fresh verification is present and new privileged operations are admitted.
    AdmittedWithCurrentVerification,
    /// New privileged operations pause until fresh verification lands.
    DeniedPendingFreshVerification,
    /// Trust-bearing privileged operations pause until trust-root review lands.
    DeniedPendingTrustRootRepair,
    /// Mutating managed actions pause under an emergency-disable ratchet.
    PausedByEmergencyDisable,
}

impl PrivilegedOperationPostureClass {
    /// Stable token recorded on rows and lifecycle events.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedWithCurrentVerification => "admitted_with_current_verification",
            Self::DeniedPendingFreshVerification => "denied_pending_fresh_verification",
            Self::DeniedPendingTrustRootRepair => "denied_pending_trust_root_repair",
            Self::PausedByEmergencyDisable => "paused_by_emergency_disable",
        }
    }

    fn denies_new_privileged_operations(self) -> bool {
        !matches!(self, Self::AdmittedWithCurrentVerification)
    }
}

/// Qualification tier for the finalize page and its rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSignedPolicyBundleQualificationClass {
    /// All required conditions hold and the upstream verifier audit is clean.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// Required coverage is missing.
    Preview,
    /// A hard guardrail was triggered.
    Withdrawn,
}

impl FinalizeSignedPolicyBundleQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Narrow reasons emitted by the finalize audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSignedPolicyBundleNarrowReasonClass {
    /// No narrowing applied.
    NotNarrowed,
    /// The upstream offline-entitlement verifier has non-withdrawal defects.
    OfflineEntitlementVerifierHasDefects,
    /// Grace-window bounds were not declared on a stale row.
    GraceWindowNotDeclared,
    /// Staleness was hidden behind a generic auth failure.
    StalenessDisguisedAsAuthFailure,
    /// Epoch metadata was not inspectable.
    PolicyEpochNotInspectable,
    /// The simulation packet lacked affected surfaces.
    SimulationPacketMissingBeforeApply,
    /// The simulation packet was not marked inspectable before apply.
    SimulationPacketNotInspectableBeforeApply,
    /// Widening managed claims lacked an approval owner.
    ApprovalOwnerMissingOnWidening,
    /// Expiry posture was not inspectable.
    ExpiryPostureNotInspectable,
    /// Local-core continuity was not explicit.
    LocalCoreContinuityNotExplicit,
    /// Import-flow coverage was incomplete.
    ImportFlowCoverageGap,
    /// Bundle-kind coverage was incomplete.
    BundleKindCoverageGap,
    /// Delivery-source coverage was incomplete.
    DeliverySourceCoverageGap,
    /// Required envelope metadata was missing.
    BundleEnvelopeNotInspectable,
    /// Required supersedes or revokes relations were not visible.
    BundleRelationsNotInspectable,
    /// Emergency-disable rows did not declare the minimum required version.
    RequiredMinimumVersionMissing,
    /// Stale rows failed to deny new privileged operations.
    PrivilegedOperationsNotNarrowedOnStaleBundle,
    /// Required lifecycle audit coverage was incomplete.
    LifecycleCoverageGap,
    /// Raw private material escaped from the upstream verifier page.
    RawPrivateMaterialExposed,
}

impl FinalizeSignedPolicyBundleNarrowReasonClass {
    /// Stable token recorded on defects and rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::OfflineEntitlementVerifierHasDefects => {
                "offline_entitlement_verifier_has_defects"
            }
            Self::GraceWindowNotDeclared => "grace_window_not_declared",
            Self::StalenessDisguisedAsAuthFailure => "staleness_disguised_as_auth_failure",
            Self::PolicyEpochNotInspectable => "policy_epoch_not_inspectable",
            Self::SimulationPacketMissingBeforeApply => "simulation_packet_missing_before_apply",
            Self::SimulationPacketNotInspectableBeforeApply => {
                "simulation_packet_not_inspectable_before_apply"
            }
            Self::ApprovalOwnerMissingOnWidening => "approval_owner_missing_on_widening",
            Self::ExpiryPostureNotInspectable => "expiry_posture_not_inspectable",
            Self::LocalCoreContinuityNotExplicit => "local_core_continuity_not_explicit",
            Self::ImportFlowCoverageGap => "import_flow_coverage_gap",
            Self::BundleKindCoverageGap => "bundle_kind_coverage_gap",
            Self::DeliverySourceCoverageGap => "delivery_source_coverage_gap",
            Self::BundleEnvelopeNotInspectable => "bundle_envelope_not_inspectable",
            Self::BundleRelationsNotInspectable => "bundle_relations_not_inspectable",
            Self::RequiredMinimumVersionMissing => "required_minimum_version_missing",
            Self::PrivilegedOperationsNotNarrowedOnStaleBundle => {
                "privileged_operations_not_narrowed_on_stale_bundle"
            }
            Self::LifecycleCoverageGap => "lifecycle_coverage_gap",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
        }
    }

    /// True when this reason triggers immediate withdrawal.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed | Self::StalenessDisguisedAsAuthFailure
        )
    }

    fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::ImportFlowCoverageGap
                | Self::BundleKindCoverageGap
                | Self::DeliverySourceCoverageGap
                | Self::LifecycleCoverageGap
        )
    }
}

/// Lifecycle audit event class that must remain reviewable and export-safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleLifecycleEventClass {
    /// A signed bundle was applied.
    Apply,
    /// A newer bundle superseded an older bundle.
    Supersede,
    /// A bundle or signer was revoked.
    Revoke,
    /// Signer rotation or trust-root review occurred.
    SignerRotationReview,
    /// Emergency-disable ratchet became active.
    EmergencyDisableActivation,
}

impl BundleLifecycleEventClass {
    /// All required lifecycle classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Apply,
        Self::Supersede,
        Self::Revoke,
        Self::SignerRotationReview,
        Self::EmergencyDisableActivation,
    ];

    /// Stable token recorded on events.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::Supersede => "supersede",
            Self::Revoke => "revoke",
            Self::SignerRotationReview => "signer_rotation_review",
            Self::EmergencyDisableActivation => "emergency_disable_activation",
        }
    }
}

/// Epoch inspection state for the bundle under inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEpochState {
    /// Opaque epoch ref for this bundle.
    pub epoch_ref: String,
    /// Opaque content digest for the epoch.
    pub epoch_digest: String,
    /// Opaque trust root ref that authorised this epoch.
    pub trust_root_ref: String,
    /// Timestamp of the last successful validation against the trust root.
    pub last_successful_validation_time: String,
    /// Import flow that sourced this epoch state.
    pub import_flow_token: String,
}

impl PolicyEpochState {
    /// True when all required epoch inspection fields are non-empty.
    pub fn is_fully_inspectable(&self) -> bool {
        !self.epoch_ref.is_empty()
            && !self.epoch_digest.is_empty()
            && !self.trust_root_ref.is_empty()
            && !self.last_successful_validation_time.is_empty()
    }
}

/// Grace-window and staleness state for the bundle under inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineGraceState {
    /// Current grace posture.
    pub grace_posture: GracePostureClass,
    /// Stable token for [`Self::grace_posture`].
    pub grace_posture_token: String,
    /// Timestamp when the grace window started; empty when `not_in_grace`.
    pub grace_window_start: String,
    /// Timestamp when the grace window ends; must be non-empty when stale.
    pub grace_window_end: String,
    /// Opaque ref to the last-known-good verified bundle revision.
    pub last_known_good_revision: String,
    /// Explicit staleness label surfaced in UI and support exports.
    pub staleness_label: String,
    /// Export-safe labels for managed capabilities blocked when grace expires.
    pub blocked_capability_consequences: Vec<String>,
}

impl OfflineGraceState {
    /// True when the staleness label is explicit (non-empty) for stale rows.
    pub fn staleness_is_explicitly_labeled(&self) -> bool {
        if self.grace_posture.is_stale() {
            !self.staleness_label.is_empty()
        } else {
            true
        }
    }

    /// True when the grace window bounds are declared for stale rows.
    pub fn grace_window_is_declared(&self) -> bool {
        if self.grace_posture.is_stale() {
            !self.grace_window_end.is_empty()
        } else {
            true
        }
    }
}

/// Reviewable envelope metadata for the signed bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleEnvelopeReview {
    /// Stable opaque bundle ref.
    pub bundle_ref: String,
    /// Opaque issuer ref.
    pub issuer_ref: String,
    /// Opaque scope ref.
    pub scope_ref: String,
    /// Delivery source that carried the bundle.
    pub delivery_source: BundleDeliverySourceClass,
    /// Stable token for [`Self::delivery_source`].
    pub delivery_source_token: String,
    /// Expiry guidance carried by the bundle.
    pub expiry_guidance: ExpiryGuidanceClass,
    /// Stable token for [`Self::expiry_guidance`].
    pub expiry_guidance_token: String,
    /// Older bundle refs explicitly superseded by this bundle.
    pub supersedes_refs: Vec<String>,
    /// Bundle or signer refs revoked by this bundle.
    pub revokes_refs: Vec<String>,
    /// Minimum supported version forced by an emergency-disable ratchet.
    pub required_minimum_version: String,
}

impl BundleEnvelopeReview {
    /// True when the envelope fields required for row review are present.
    pub fn is_fully_inspectable(&self) -> bool {
        !self.bundle_ref.is_empty()
            && !self.issuer_ref.is_empty()
            && !self.scope_ref.is_empty()
            && !self.delivery_source_token.is_empty()
            && !self.expiry_guidance_token.is_empty()
    }
}

/// Pre-apply simulation packet required before any bundle apply is admitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyBundleSimulationPacket {
    /// Stable opaque id for this packet.
    pub packet_id: String,
    /// Feature areas whose effective policy would change after apply.
    pub changed_feature_areas: Vec<String>,
    /// Export-safe summary of previous effective values for changed keys.
    pub previous_values_summary: BTreeMap<String, String>,
    /// Export-safe summary of simulated effective values after apply.
    pub simulated_values_summary: BTreeMap<String, String>,
    /// Surfaces whose behaviour would be affected by the apply.
    pub affected_surfaces: Vec<String>,
    /// Plain-language labels for degraded-mode consequences.
    pub degraded_mode_consequences: Vec<String>,
    /// Notes specific to offline or stale-policy apply paths.
    pub offline_or_stale_policy_notes: Vec<String>,
    /// Opaque ref to the approval owner who authorised the apply.
    pub approval_owner_ref: String,
    /// Stable token describing the expiry posture after apply.
    pub expiry_posture_token: String,
    /// True when this import or refresh would widen managed claims.
    pub widens_managed_claims: bool,
    /// True when this packet is inspectable before apply is permitted.
    pub inspectable_before_apply: bool,
}

/// Finalize row for one `(import_flow × bundle_kind)` pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSignedPolicyBundleRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Import flow for this row.
    pub import_flow: BundleImportFlowClass,
    /// Stable token for [`Self::import_flow`].
    pub import_flow_token: String,
    /// Bundle kind for this row.
    pub bundle_kind: BundleKindClass,
    /// Stable token for [`Self::bundle_kind`].
    pub bundle_kind_token: String,
    /// Opaque ref to the signer recorded on the bundle.
    pub signer_ref: String,
    /// Signed-at timestamp on the bundle.
    pub signed_at: String,
    /// `valid_until` timestamp on the bundle.
    pub valid_until: String,
    /// Epoch inspection state for this row.
    pub epoch_state: PolicyEpochState,
    /// Grace posture and staleness state for this row.
    pub grace_state: OfflineGraceState,
    /// Reviewable envelope metadata.
    pub envelope_review: BundleEnvelopeReview,
    /// Privileged-operation posture when this row is active.
    pub privileged_operation_posture: PrivilegedOperationPostureClass,
    /// Stable token for [`Self::privileged_operation_posture`].
    pub privileged_operation_posture_token: String,
    /// Pre-apply simulation packet for this row.
    pub simulation_packet: PolicyBundleSimulationPacket,
    /// True when local-core continuity is stated explicitly on this row.
    pub local_core_continuity_explicit: bool,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

impl FinalizeSignedPolicyBundleRow {
    fn relations_are_inspectable(&self) -> bool {
        !self.envelope_review.supersedes_refs.is_empty()
            || !self.envelope_review.revokes_refs.is_empty()
    }

    fn required_minimum_version_is_declared(&self) -> bool {
        if self.bundle_kind == BundleKindClass::EmergencyDisableBundle {
            !self.envelope_review.required_minimum_version.is_empty()
        } else {
            true
        }
    }

    fn stale_row_denies_privileged_operations(&self) -> bool {
        if self.grace_state.grace_posture.is_stale() {
            self.privileged_operation_posture
                .denies_new_privileged_operations()
        } else {
            true
        }
    }
}

/// Lifecycle audit event that support-export and offline review can quote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLifecycleAuditEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Lifecycle class.
    pub event_class: BundleLifecycleEventClass,
    /// Stable token for [`Self::event_class`].
    pub event_class_token: String,
    /// Bundle kind affected by the event.
    pub bundle_kind: BundleKindClass,
    /// Stable token for [`Self::bundle_kind`].
    pub bundle_kind_token: String,
    /// Opaque bundle ref affected by the event.
    pub bundle_ref: String,
    /// Opaque scope ref affected by the event.
    pub scope_ref: String,
    /// Delivery source involved in the event.
    pub delivery_source: BundleDeliverySourceClass,
    /// Stable token for [`Self::delivery_source`].
    pub delivery_source_token: String,
    /// Opaque actor or operator ref.
    pub actor_ref: String,
    /// Event timestamp.
    pub event_time: String,
    /// Superseded refs cited by the event.
    pub supersedes_refs: Vec<String>,
    /// Revoked refs cited by the event.
    pub revokes_refs: Vec<String>,
    /// Privileged-operation posture after the event.
    pub privileged_operation_posture: PrivilegedOperationPostureClass,
    /// Stable token for [`Self::privileged_operation_posture`].
    pub privileged_operation_posture_token: String,
    /// True when local-safe continuity remains preserved after the event.
    pub local_safe_continuity_preserved: bool,
    /// Export-safe event summary.
    pub note: String,
}

/// Aggregate summary for the finalize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FinalizeSignedPolicyBundleSummary {
    /// Total row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Import-flow tokens present on the page.
    pub import_flows_covered: Vec<String>,
    /// Bundle-kind tokens present on the page.
    pub bundle_kinds_covered: Vec<String>,
    /// Delivery-source tokens present on the page.
    pub delivery_sources_covered: Vec<String>,
    /// Lifecycle-event class tokens present on the page.
    pub lifecycle_classes_covered: Vec<String>,
    /// Number of lifecycle events on the page.
    pub lifecycle_event_count: usize,
    /// Number of rows where the bundle is stale.
    pub stale_bundle_row_count: usize,
    /// Number of rows with fully inspectable epoch states.
    pub epoch_inspectable_row_count: usize,
    /// Number of rows with inspectable envelope metadata.
    pub envelope_inspectable_row_count: usize,
    /// Number of rows with a simulation packet that covers affected surfaces.
    pub simulation_packet_present_row_count: usize,
    /// Number of rows with `local_core_continuity_explicit: true`.
    pub local_core_continuity_explicit_row_count: usize,
    /// Number of stale rows that deny new privileged operations.
    pub stale_privileged_pause_row_count: usize,
    /// Defect count from the upstream offline-entitlement verifier page.
    pub upstream_verifier_defect_count: usize,
    /// Overall qualification token derived from all rows and defects.
    pub overall_qualification_token: String,
}

impl FinalizeSignedPolicyBundleSummary {
    fn from_rows_and_events(
        rows: &[FinalizeSignedPolicyBundleRow],
        lifecycle_events: &[BundleLifecycleAuditEvent],
        verifier_page: &OfflineEntitlementVerifierBetaPage,
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut import_flows = BTreeSet::new();
        let mut bundle_kinds = BTreeSet::new();
        let mut delivery_sources = BTreeSet::new();
        let mut lifecycle_classes = BTreeSet::new();
        let mut stale = 0usize;
        let mut epoch_ok = 0usize;
        let mut envelope_ok = 0usize;
        let mut sim_ok = 0usize;
        let mut local_core_ok = 0usize;
        let mut stale_privileged_pause_ok = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            import_flows.insert(row.import_flow_token.clone());
            bundle_kinds.insert(row.bundle_kind_token.clone());
            delivery_sources.insert(row.envelope_review.delivery_source_token.clone());
            if row.grace_state.grace_posture.is_stale() {
                stale += 1;
                if row.stale_row_denies_privileged_operations() {
                    stale_privileged_pause_ok += 1;
                }
            }
            if row.epoch_state.is_fully_inspectable() {
                epoch_ok += 1;
            }
            if row.envelope_review.is_fully_inspectable() {
                envelope_ok += 1;
            }
            if !row.simulation_packet.affected_surfaces.is_empty()
                && row.simulation_packet.inspectable_before_apply
            {
                sim_ok += 1;
            }
            if row.local_core_continuity_explicit {
                local_core_ok += 1;
            }
        }

        for event in lifecycle_events {
            lifecycle_classes.insert(event.event_class_token.clone());
        }

        let overall = if withdrawn > 0 {
            FinalizeSignedPolicyBundleQualificationClass::Withdrawn
        } else if preview > 0 {
            FinalizeSignedPolicyBundleQualificationClass::Preview
        } else if beta > 0 {
            FinalizeSignedPolicyBundleQualificationClass::Beta
        } else {
            FinalizeSignedPolicyBundleQualificationClass::Stable
        };

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            import_flows_covered: import_flows.into_iter().collect(),
            bundle_kinds_covered: bundle_kinds.into_iter().collect(),
            delivery_sources_covered: delivery_sources.into_iter().collect(),
            lifecycle_classes_covered: lifecycle_classes.into_iter().collect(),
            lifecycle_event_count: lifecycle_events.len(),
            stale_bundle_row_count: stale,
            epoch_inspectable_row_count: epoch_ok,
            envelope_inspectable_row_count: envelope_ok,
            simulation_packet_present_row_count: sim_ok,
            local_core_continuity_explicit_row_count: local_core_ok,
            stale_privileged_pause_row_count: stale_privileged_pause_ok,
            upstream_verifier_defect_count: verifier_page.defects.len(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the finalize-page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSignedPolicyBundleDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: FinalizeSignedPolicyBundleNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (row id, import flow, lifecycle class, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl FinalizeSignedPolicyBundleDefect {
    fn new(
        narrow_reason: FinalizeSignedPolicyBundleNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
            shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:signed-policy-bundle-finalize:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

/// Proof packet for signed bundle review and offline continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSignedPolicyBundlePage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from all rows.
    pub summary: FinalizeSignedPolicyBundleSummary,
    /// Per-row qualification rows (one per import_flow × bundle_kind pair).
    pub rows: Vec<FinalizeSignedPolicyBundleRow>,
    /// Export-safe lifecycle audit events.
    pub lifecycle_events: Vec<BundleLifecycleAuditEvent>,
    /// Typed validation defects for this packet.
    pub defects: Vec<FinalizeSignedPolicyBundleDefect>,
    /// Upstream offline-entitlement verifier page embedded as evidence.
    pub offline_entitlement_verifier_page: OfflineEntitlementVerifierBetaPage,
}

impl FinalizeSignedPolicyBundlePage {
    /// Build the finalize page from rows, lifecycle events, and embedded verifier evidence.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<FinalizeSignedPolicyBundleRow>,
        lifecycle_events: Vec<BundleLifecycleAuditEvent>,
        offline_entitlement_verifier_page: OfflineEntitlementVerifierBetaPage,
    ) -> Self {
        let defects =
            audit_finalize_rows(&rows, &lifecycle_events, &offline_entitlement_verifier_page);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary = FinalizeSignedPolicyBundleSummary::from_rows_and_events(
            &qualified_rows,
            &lifecycle_events,
            &offline_entitlement_verifier_page,
        );
        Self {
            record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_PAGE_RECORD_KIND.to_owned(),
            schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
            shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            lifecycle_events,
            defects,
            offline_entitlement_verifier_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == FinalizeSignedPolicyBundleQualificationClass::Stable.as_str()
    }

    /// True when all required import flows are covered.
    pub fn covers_all_required_import_flows(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|row| row.import_flow_token.as_str())
            .collect();
        BundleImportFlowClass::ALL
            .iter()
            .all(|flow| covered.contains(flow.as_str()))
    }

    /// True when all required bundle kinds are covered.
    pub fn covers_all_required_bundle_kinds(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|row| row.bundle_kind_token.as_str())
            .collect();
        BundleKindClass::ALL
            .iter()
            .all(|kind| covered.contains(kind.as_str()))
    }

    /// True when all required delivery sources are covered.
    pub fn covers_all_required_delivery_sources(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|row| row.envelope_review.delivery_source_token.as_str())
            .collect();
        BundleDeliverySourceClass::ALL
            .iter()
            .all(|source| covered.contains(source.as_str()))
    }

    /// True when all rows carry fully inspectable epoch states.
    pub fn all_epoch_states_inspectable(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.epoch_state.is_fully_inspectable())
    }

    /// True when all rows carry fully inspectable envelope metadata.
    pub fn all_rows_have_inspectable_envelopes(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.envelope_review.is_fully_inspectable())
    }

    /// True when all simulation packets have at least one affected surface.
    pub fn all_simulation_packets_have_affected_surfaces(&self) -> bool {
        self.rows
            .iter()
            .all(|row| !row.simulation_packet.affected_surfaces.is_empty())
    }

    /// True when all rows carry `local_core_continuity_explicit: true`.
    pub fn all_rows_explicit_on_local_core_continuity(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.local_core_continuity_explicit)
    }

    /// True when all stale rows carry explicit staleness labels.
    pub fn stale_rows_are_explicitly_labeled(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.grace_state.staleness_is_explicitly_labeled())
    }

    /// True when all stale rows declare a bounded grace window.
    pub fn stale_rows_have_declared_grace_windows(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.grace_state.grace_window_is_declared())
    }

    /// True when all stale rows deny new privileged operations.
    pub fn stale_rows_deny_new_privileged_operations(&self) -> bool {
        self.rows
            .iter()
            .all(FinalizeSignedPolicyBundleRow::stale_row_denies_privileged_operations)
    }

    /// True when emergency-disable rows declare a minimum required version.
    pub fn emergency_disable_rows_declare_required_minimum_version(&self) -> bool {
        self.rows
            .iter()
            .filter(|row| row.bundle_kind == BundleKindClass::EmergencyDisableBundle)
            .all(FinalizeSignedPolicyBundleRow::required_minimum_version_is_declared)
    }

    /// True when lifecycle events cover all required classes.
    pub fn lifecycle_events_cover_required_classes(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .lifecycle_events
            .iter()
            .map(|event| event.event_class_token.as_str())
            .collect();
        BundleLifecycleEventClass::ALL
            .iter()
            .all(|class| covered.contains(class.as_str()))
    }
}

/// Support-export wrapper that quotes the finalize page plus metadata-safe rollups.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSignedPolicyBundleSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The finalize page embedded as evidence.
    pub page: FinalizeSignedPolicyBundlePage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<FinalizeSignedPolicyBundleNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// Lifecycle-event counts by lifecycle class.
    pub lifecycle_event_counts_by_class: BTreeMap<String, usize>,
    /// True when raw private material is excluded from the export.
    pub raw_private_material_excluded: bool,
}

impl FinalizeSignedPolicyBundleSupportExport {
    /// Wrap a finalize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: FinalizeSignedPolicyBundlePage,
    ) -> Self {
        let mut reasons = Vec::new();
        let mut defect_counts = BTreeMap::new();
        let mut lifecycle_counts = BTreeMap::new();

        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *defect_counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }

        for event in &page.lifecycle_events {
            *lifecycle_counts
                .entry(event.event_class_token.clone())
                .or_insert(0) += 1;
        }

        reasons.sort();

        Self {
            record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
            shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: defect_counts,
            lifecycle_event_counts_by_class: lifecycle_counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-run the finalize audit over the rows, lifecycle events, and verifier page.
pub fn audit_finalize_signed_policy_bundle_page(
    page: &FinalizeSignedPolicyBundlePage,
) -> Vec<FinalizeSignedPolicyBundleDefect> {
    audit_finalize_rows(
        &page.rows,
        &page.lifecycle_events,
        &page.offline_entitlement_verifier_page,
    )
}

/// Validate the finalize page; returns `Ok` when the audit is clean.
pub fn validate_finalize_signed_policy_bundle_page(
    page: &FinalizeSignedPolicyBundlePage,
) -> Result<(), Vec<FinalizeSignedPolicyBundleDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Build the seeded finalize page covering all required flows and bundle kinds.
pub fn seeded_finalize_signed_policy_bundle_page() -> FinalizeSignedPolicyBundlePage {
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let rows = seeded_rows();
    let lifecycle_events = seeded_lifecycle_events(&rows);
    FinalizeSignedPolicyBundlePage::new(
        "policy:signed-policy-bundle-finalize:seeded:0001",
        "Signed administrative bundle, offline entitlement, emergency-disable, and trust-root finalize packet",
        "2026-06-01T00:00:00Z",
        rows,
        lifecycle_events,
        verifier_page,
    )
}

fn audit_finalize_rows(
    rows: &[FinalizeSignedPolicyBundleRow],
    lifecycle_events: &[BundleLifecycleAuditEvent],
    verifier_page: &OfflineEntitlementVerifierBetaPage,
) -> Vec<FinalizeSignedPolicyBundleDefect> {
    let mut defects = Vec::new();

    let upstream_defects = audit_offline_entitlement_verifier_beta_rows(&verifier_page.rows);
    let has_raw_material = upstream_defects.iter().any(|defect| {
        defect.defect_kind == OfflineEntitlementVerifierBetaDefectKind::RawPrivateMaterialExposed
    });
    if has_raw_material {
        defects.push(FinalizeSignedPolicyBundleDefect::new(
            FinalizeSignedPolicyBundleNarrowReasonClass::RawPrivateMaterialExposed,
            "offline_entitlement_verifier_page",
            "upstream offline-entitlement verifier page has a raw_private_material_exposed defect; packet is withdrawn",
        ));
        return defects;
    }

    if !verifier_page.defects.is_empty() {
        defects.push(FinalizeSignedPolicyBundleDefect::new(
            FinalizeSignedPolicyBundleNarrowReasonClass::OfflineEntitlementVerifierHasDefects,
            "offline_entitlement_verifier_page",
            "upstream offline-entitlement verifier page has non-withdrawal defects; packet is narrowed to beta",
        ));
    }

    for row in rows {
        if !row.grace_state.staleness_is_explicitly_labeled() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::StalenessDisguisedAsAuthFailure,
                row.row_id.clone(),
                "stale bundle row has an empty staleness_label; staleness must not be disguised as a generic auth failure",
            ));
        }

        if row.import_flow.requires_grace_window_when_stale()
            && !row.grace_state.grace_window_is_declared()
        {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::GraceWindowNotDeclared,
                row.row_id.clone(),
                "offline or air-gapped row has stale bundle state with no declared grace-window end",
            ));
        }

        if !row.epoch_state.is_fully_inspectable() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::PolicyEpochNotInspectable,
                row.row_id.clone(),
                "row is missing one or more required epoch inspection fields",
            ));
        }

        if !row.envelope_review.is_fully_inspectable() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::BundleEnvelopeNotInspectable,
                row.row_id.clone(),
                "row is missing bundle_ref, issuer_ref, scope_ref, delivery_source, or expiry guidance metadata",
            ));
        }

        if !row.relations_are_inspectable() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::BundleRelationsNotInspectable,
                row.row_id.clone(),
                "row has no visible supersedes or revokes relation",
            ));
        }

        if !row.required_minimum_version_is_declared() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::RequiredMinimumVersionMissing,
                row.row_id.clone(),
                "emergency-disable row is missing required_minimum_version",
            ));
        }

        if !row.stale_row_denies_privileged_operations() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::PrivilegedOperationsNotNarrowedOnStaleBundle,
                row.row_id.clone(),
                "stale row still admits new privileged operations instead of narrowing to local-safe continuity",
            ));
        }

        if row.simulation_packet.affected_surfaces.is_empty() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::SimulationPacketMissingBeforeApply,
                row.row_id.clone(),
                "simulation packet has no affected surfaces; pre-apply inspection was not exported",
            ));
        }

        if !row.simulation_packet.inspectable_before_apply {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::SimulationPacketNotInspectableBeforeApply,
                row.row_id.clone(),
                "simulation packet is not marked inspectable_before_apply",
            ));
        }

        if row.simulation_packet.widens_managed_claims
            && row.simulation_packet.approval_owner_ref.is_empty()
        {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::ApprovalOwnerMissingOnWidening,
                row.row_id.clone(),
                "simulation packet widens managed claims but approval_owner_ref is empty",
            ));
        }

        if row.simulation_packet.expiry_posture_token.is_empty() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::ExpiryPostureNotInspectable,
                row.row_id.clone(),
                "simulation packet has an empty expiry_posture_token",
            ));
        }

        if !row.local_core_continuity_explicit {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::LocalCoreContinuityNotExplicit,
                row.row_id.clone(),
                "row does not carry local_core_continuity_explicit: true",
            ));
        }
    }

    push_coverage_gap_defects(
        &mut defects,
        rows.iter().map(|row| row.import_flow_token.as_str()),
        BundleImportFlowClass::ALL.iter().map(|flow| flow.as_str()),
        FinalizeSignedPolicyBundleNarrowReasonClass::ImportFlowCoverageGap,
        "page",
        "missing rows for required import flow",
    );
    push_coverage_gap_defects(
        &mut defects,
        rows.iter().map(|row| row.bundle_kind_token.as_str()),
        BundleKindClass::ALL.iter().map(|kind| kind.as_str()),
        FinalizeSignedPolicyBundleNarrowReasonClass::BundleKindCoverageGap,
        "page",
        "missing rows for required bundle kind",
    );
    push_coverage_gap_defects(
        &mut defects,
        rows.iter()
            .map(|row| row.envelope_review.delivery_source_token.as_str()),
        BundleDeliverySourceClass::ALL
            .iter()
            .map(|source| source.as_str()),
        FinalizeSignedPolicyBundleNarrowReasonClass::DeliverySourceCoverageGap,
        "page",
        "missing rows for required delivery source",
    );
    push_coverage_gap_defects(
        &mut defects,
        lifecycle_events
            .iter()
            .map(|event| event.event_class_token.as_str()),
        BundleLifecycleEventClass::ALL
            .iter()
            .map(|class| class.as_str()),
        FinalizeSignedPolicyBundleNarrowReasonClass::LifecycleCoverageGap,
        "page",
        "missing lifecycle audit coverage for required event class",
    );

    defects
}

fn push_coverage_gap_defects<'a>(
    defects: &mut Vec<FinalizeSignedPolicyBundleDefect>,
    observed: impl IntoIterator<Item = &'a str>,
    required: impl IntoIterator<Item = &'a str>,
    reason: FinalizeSignedPolicyBundleNarrowReasonClass,
    source: &str,
    prefix: &str,
) {
    let observed_set: BTreeSet<&str> = observed.into_iter().collect();
    for missing in required
        .into_iter()
        .filter(|item| !observed_set.contains(item))
    {
        defects.push(FinalizeSignedPolicyBundleDefect::new(
            reason,
            source,
            format!("{prefix} '{missing}'"),
        ));
    }
}

fn qualify_rows(
    mut rows: Vec<FinalizeSignedPolicyBundleRow>,
    page_defects: &[FinalizeSignedPolicyBundleDefect],
) -> Vec<FinalizeSignedPolicyBundleRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason());

    let overall_qual = if has_withdrawal {
        FinalizeSignedPolicyBundleQualificationClass::Withdrawn
    } else if has_preview {
        FinalizeSignedPolicyBundleQualificationClass::Preview
    } else if page_defects.is_empty() {
        FinalizeSignedPolicyBundleQualificationClass::Stable
    } else {
        FinalizeSignedPolicyBundleQualificationClass::Beta
    };

    for row in &mut rows {
        let row_reason = if has_withdrawal {
            page_defects
                .iter()
                .find(|defect| defect.narrow_reason.is_withdrawal_reason())
                .map(|defect| defect.narrow_reason)
                .unwrap_or(FinalizeSignedPolicyBundleNarrowReasonClass::RawPrivateMaterialExposed)
        } else if has_preview {
            page_defects
                .iter()
                .find(|defect| defect.narrow_reason.is_preview_reason())
                .map(|defect| defect.narrow_reason)
                .unwrap_or(FinalizeSignedPolicyBundleNarrowReasonClass::ImportFlowCoverageGap)
        } else {
            page_defects
                .iter()
                .find(|defect| defect.source == row.row_id)
                .map(|defect| defect.narrow_reason)
                .or_else(|| {
                    page_defects
                        .iter()
                        .find(|defect| {
                            defect.source == "offline_entitlement_verifier_page"
                                && defect.narrow_reason
                                    == FinalizeSignedPolicyBundleNarrowReasonClass::OfflineEntitlementVerifierHasDefects
                        })
                        .map(|defect| defect.narrow_reason)
                })
                .unwrap_or(FinalizeSignedPolicyBundleNarrowReasonClass::NotNarrowed)
        };

        let row_qual = if row_reason == FinalizeSignedPolicyBundleNarrowReasonClass::NotNarrowed {
            FinalizeSignedPolicyBundleQualificationClass::Stable
        } else if row_reason.is_withdrawal_reason() {
            FinalizeSignedPolicyBundleQualificationClass::Withdrawn
        } else if row_reason.is_preview_reason() {
            FinalizeSignedPolicyBundleQualificationClass::Preview
        } else {
            FinalizeSignedPolicyBundleQualificationClass::Beta
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary = build_row_summary(
            &row.row_id,
            &row.import_flow_token,
            &row.bundle_kind_token,
            row_qual,
            row_reason,
            row.privileged_operation_posture_token.as_str(),
        );

        if overall_qual > row_qual {
            row.qualification_token = overall_qual.as_str().to_owned();
        }
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    import_flow_token: &str,
    bundle_kind_token: &str,
    qual: FinalizeSignedPolicyBundleQualificationClass,
    narrow_reason: FinalizeSignedPolicyBundleNarrowReasonClass,
    privileged_posture_token: &str,
) -> String {
    match qual {
        FinalizeSignedPolicyBundleQualificationClass::Stable => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) qualifies stable: signed envelope is inspectable, stale posture is explicit, privileged posture is '{privileged_posture_token}', and lifecycle review remains portable."
        ),
        FinalizeSignedPolicyBundleQualificationClass::Beta => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) narrowed to beta (reason: {}); inspectable local-safe continuity remains available.",
            narrow_reason.as_str()
        ),
        FinalizeSignedPolicyBundleQualificationClass::Preview => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) narrowed to preview because required signed-artifact coverage is incomplete."
        ),
        FinalizeSignedPolicyBundleQualificationClass::Withdrawn => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) is withdrawn (reason: {}); hard guardrail triggered.",
            narrow_reason.as_str()
        ),
    }
}

fn seeded_rows() -> Vec<FinalizeSignedPolicyBundleRow> {
    let mut rows = Vec::new();
    for import_flow in BundleImportFlowClass::ALL {
        for bundle_kind in BundleKindClass::ALL {
            rows.push(make_seeded_row(import_flow, bundle_kind));
        }
    }
    rows
}

fn make_seeded_row(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
) -> FinalizeSignedPolicyBundleRow {
    let delivery_source = delivery_source_for(import_flow, bundle_kind);
    let grace_posture = grace_posture_for(import_flow, bundle_kind);
    let expiry_guidance = expiry_guidance_for(import_flow, bundle_kind, grace_posture);
    let privileged_posture =
        privileged_posture_for(import_flow, bundle_kind, grace_posture, delivery_source);
    let staleness_label = staleness_label_for(import_flow, bundle_kind, grace_posture);
    let bundle_ref = format!(
        "bundle:{}:{}:2026.06.0001",
        bundle_kind.as_str(),
        import_flow.as_str()
    );
    let row_id = format!(
        "signed-policy-bundle-finalize:{}:{}",
        import_flow.as_str(),
        bundle_kind.as_str()
    );
    let supersedes_refs = vec![format!(
        "bundle:{}:{}:2026.05.0004",
        bundle_kind.as_str(),
        import_flow.as_str()
    )];
    let revokes_refs = match bundle_kind {
        BundleKindClass::AdminPolicyBundle => {
            vec![format!(
                "revoke:policy:{}:2026.05.0001",
                import_flow.as_str()
            )]
        }
        BundleKindClass::EntitlementSnapshot => vec![format!(
            "revoke:entitlement:{}:2026.05.0001",
            import_flow.as_str()
        )],
        BundleKindClass::EmergencyDisableBundle => vec![
            format!("revoke:extension-set:{}:payments-sdk", import_flow.as_str()),
            format!("revoke:provider-route:{}:hosted-eval", import_flow.as_str()),
        ],
        BundleKindClass::TrustRootSignerUpdate => vec![
            "revoke:signer:trust-root:2025-primary".to_owned(),
            format!("revoke:root-pointer:{}:legacy", import_flow.as_str()),
        ],
    };
    let changed_feature_areas = changed_feature_areas_for(import_flow, bundle_kind);
    let affected_surfaces: Vec<String> = bundle_kind
        .affected_surfaces()
        .into_iter()
        .map(str::to_owned)
        .collect();
    let degraded_mode_consequences = degraded_mode_consequences_for(bundle_kind, grace_posture);
    let offline_notes = offline_notes_for(import_flow, bundle_kind, grace_posture);
    let widens_managed_claims = matches!(
        (import_flow, bundle_kind),
        (
            BundleImportFlowClass::ManualImport,
            BundleKindClass::AdminPolicyBundle
        )
    );
    let approval_owner_ref = if widens_managed_claims {
        "owner:policy-review-board:managed-alpha"
    } else {
        ""
    };
    let signed_at = signed_at_for(import_flow);
    let valid_until = valid_until_for(import_flow, bundle_kind);
    let (grace_window_start, grace_window_end, last_known_good_revision) =
        grace_window_for(import_flow, bundle_kind, grace_posture);

    FinalizeSignedPolicyBundleRow {
        record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_ROW_RECORD_KIND.to_owned(),
        schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
        shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.clone(),
        import_flow,
        import_flow_token: import_flow.as_str().to_owned(),
        bundle_kind,
        bundle_kind_token: bundle_kind.as_str().to_owned(),
        signer_ref: bundle_kind.signer_ref().to_owned(),
        signed_at: signed_at.to_owned(),
        valid_until: valid_until.to_owned(),
        epoch_state: PolicyEpochState {
            epoch_ref: format!(
                "epoch:{}:{}:2026.06",
                bundle_kind.as_str(),
                import_flow.as_str()
            ),
            epoch_digest: format!(
                "sha256:{}{}{}{}{}{}{}{}",
                bundle_kind.as_str().len(),
                import_flow.as_str().len(),
                "a1b2c3d4",
                "e5f6a7b8",
                "c9d0e1f2",
                "34567890",
                "abcdef12",
                "34567890abcdef12"
            ),
            trust_root_ref: match bundle_kind {
                BundleKindClass::TrustRootSignerUpdate => {
                    "trust_root:rotation-window:2026-primary".to_owned()
                }
                BundleKindClass::EmergencyDisableBundle => {
                    "trust_root:security-response:2026-primary".to_owned()
                }
                _ => "trust_root:managed-baseline:2026-primary".to_owned(),
            },
            last_successful_validation_time: match import_flow {
                BundleImportFlowClass::Online => "2026-05-31T12:00:00Z".to_owned(),
                BundleImportFlowClass::Mirror => "2026-05-30T08:00:00Z".to_owned(),
                BundleImportFlowClass::ManualImport => "2026-05-29T15:30:00Z".to_owned(),
                BundleImportFlowClass::AirGapped => "2026-05-28T11:45:00Z".to_owned(),
                BundleImportFlowClass::OfflineGrace => "2026-05-10T07:15:00Z".to_owned(),
            },
            import_flow_token: import_flow.as_str().to_owned(),
        },
        grace_state: OfflineGraceState {
            grace_posture,
            grace_posture_token: grace_posture.as_str().to_owned(),
            grace_window_start: grace_window_start.to_owned(),
            grace_window_end: grace_window_end.to_owned(),
            last_known_good_revision: last_known_good_revision.to_owned(),
            staleness_label: staleness_label.to_owned(),
            blocked_capability_consequences: if grace_posture.is_stale() {
                bundle_kind
                    .blocked_capability_consequences()
                    .into_iter()
                    .map(str::to_owned)
                    .collect()
            } else {
                Vec::new()
            },
        },
        envelope_review: BundleEnvelopeReview {
            bundle_ref: bundle_ref.clone(),
            issuer_ref: bundle_kind.issuer_ref().to_owned(),
            scope_ref: bundle_kind.scope_ref().to_owned(),
            delivery_source,
            delivery_source_token: delivery_source.as_str().to_owned(),
            expiry_guidance,
            expiry_guidance_token: expiry_guidance.as_str().to_owned(),
            supersedes_refs,
            revokes_refs,
            required_minimum_version: if bundle_kind == BundleKindClass::EmergencyDisableBundle {
                "2.8.4".to_owned()
            } else {
                String::new()
            },
        },
        privileged_operation_posture: privileged_posture,
        privileged_operation_posture_token: privileged_posture.as_str().to_owned(),
        simulation_packet: PolicyBundleSimulationPacket {
            packet_id: format!(
                "sim-packet:{}:{}",
                import_flow.as_str(),
                bundle_kind.as_str()
            ),
            changed_feature_areas: changed_feature_areas
                .into_iter()
                .map(str::to_owned)
                .collect(),
            previous_values_summary: BTreeMap::from([(
                "effective_posture".to_owned(),
                "last-known-good".to_owned(),
            )]),
            simulated_values_summary: BTreeMap::from([(
                "effective_posture".to_owned(),
                expiry_guidance.as_str().to_owned(),
            )]),
            affected_surfaces,
            degraded_mode_consequences: degraded_mode_consequences
                .into_iter()
                .map(str::to_owned)
                .collect(),
            offline_or_stale_policy_notes: offline_notes.into_iter().map(str::to_owned).collect(),
            approval_owner_ref: approval_owner_ref.to_owned(),
            expiry_posture_token: expiry_guidance.as_str().to_owned(),
            widens_managed_claims,
            inspectable_before_apply: true,
        },
        local_core_continuity_explicit: true,
        qualification_token: FinalizeSignedPolicyBundleQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: FinalizeSignedPolicyBundleNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn signed_at_for(import_flow: BundleImportFlowClass) -> &'static str {
    match import_flow {
        BundleImportFlowClass::Online => "2026-05-31T12:00:00Z",
        BundleImportFlowClass::Mirror => "2026-05-30T08:00:00Z",
        BundleImportFlowClass::ManualImport => "2026-05-29T15:30:00Z",
        BundleImportFlowClass::AirGapped => "2026-05-28T11:45:00Z",
        BundleImportFlowClass::OfflineGrace => "2026-05-10T07:15:00Z",
    }
}

fn valid_until_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
) -> &'static str {
    match (import_flow, bundle_kind) {
        (BundleImportFlowClass::OfflineGrace, BundleKindClass::TrustRootSignerUpdate) => {
            "2026-05-15T00:00:00Z"
        }
        (BundleImportFlowClass::OfflineGrace, _) => "2026-06-15T00:00:00Z",
        (BundleImportFlowClass::AirGapped, _) => "2026-10-01T00:00:00Z",
        (BundleImportFlowClass::ManualImport, BundleKindClass::EmergencyDisableBundle) => {
            "2026-07-15T00:00:00Z"
        }
        (BundleImportFlowClass::ManualImport, _) => "2026-08-15T00:00:00Z",
        (_, BundleKindClass::TrustRootSignerUpdate) => "2026-08-01T00:00:00Z",
        _ => "2026-09-01T00:00:00Z",
    }
}

fn delivery_source_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
) -> BundleDeliverySourceClass {
    match import_flow {
        BundleImportFlowClass::Online => BundleDeliverySourceClass::ManagedPull,
        BundleImportFlowClass::Mirror => BundleDeliverySourceClass::MirrorPublication,
        BundleImportFlowClass::ManualImport => match bundle_kind {
            BundleKindClass::EntitlementSnapshot | BundleKindClass::TrustRootSignerUpdate => {
                BundleDeliverySourceClass::MdmFleetDrop
            }
            _ => BundleDeliverySourceClass::FileImport,
        },
        BundleImportFlowClass::AirGapped => BundleDeliverySourceClass::AirGapTransfer,
        BundleImportFlowClass::OfflineGrace => BundleDeliverySourceClass::LastKnownGoodCache,
    }
}

fn grace_posture_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
) -> GracePostureClass {
    match (import_flow, bundle_kind) {
        (BundleImportFlowClass::OfflineGrace, BundleKindClass::TrustRootSignerUpdate) => {
            GracePostureClass::GraceExpired
        }
        (BundleImportFlowClass::OfflineGrace, _) => GracePostureClass::InGrace,
        _ => GracePostureClass::NotInGrace,
    }
}

fn expiry_guidance_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
    grace_posture: GracePostureClass,
) -> ExpiryGuidanceClass {
    if grace_posture == GracePostureClass::GraceExpired {
        return ExpiryGuidanceClass::LastKnownGoodOnly;
    }
    if grace_posture == GracePostureClass::InGrace {
        return ExpiryGuidanceClass::InGraceUntilRefresh;
    }
    match bundle_kind {
        BundleKindClass::AdminPolicyBundle => ExpiryGuidanceClass::RefreshBeforeExpiry,
        BundleKindClass::EntitlementSnapshot => ExpiryGuidanceClass::RefreshBeforeExpiry,
        BundleKindClass::EmergencyDisableBundle => ExpiryGuidanceClass::SupersedeOrExpire,
        BundleKindClass::TrustRootSignerUpdate => {
            if import_flow == BundleImportFlowClass::Online {
                ExpiryGuidanceClass::RotationWindowRequired
            } else {
                ExpiryGuidanceClass::RotationWindowRequired
            }
        }
    }
}

fn privileged_posture_for(
    _import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
    grace_posture: GracePostureClass,
    _delivery_source: BundleDeliverySourceClass,
) -> PrivilegedOperationPostureClass {
    if bundle_kind == BundleKindClass::EmergencyDisableBundle {
        return PrivilegedOperationPostureClass::PausedByEmergencyDisable;
    }
    if bundle_kind == BundleKindClass::TrustRootSignerUpdate
        && grace_posture == GracePostureClass::GraceExpired
    {
        return PrivilegedOperationPostureClass::DeniedPendingTrustRootRepair;
    }
    if grace_posture.is_stale() {
        return PrivilegedOperationPostureClass::DeniedPendingFreshVerification;
    }
    PrivilegedOperationPostureClass::AdmittedWithCurrentVerification
}

fn staleness_label_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
    grace_posture: GracePostureClass,
) -> String {
    if !grace_posture.is_stale() {
        return String::new();
    }

    match grace_posture {
        GracePostureClass::InGrace => format!(
            "{} on '{}' remains within its declared grace window; local-safe continuity remains available while new privileged operations wait for fresh verification.",
            bundle_kind.as_str(),
            import_flow.as_str()
        ),
        GracePostureClass::GraceExpired => format!(
            "{} on '{}' is past grace; last-known-good local-safe continuity remains visible but trust-bearing actions are paused until signer review lands.",
            bundle_kind.as_str(),
            import_flow.as_str()
        ),
        GracePostureClass::NotInGrace => String::new(),
    }
}

fn grace_window_for(
    import_flow: BundleImportFlowClass,
    _bundle_kind: BundleKindClass,
    grace_posture: GracePostureClass,
) -> (&'static str, &'static str, String) {
    let last_known_good_revision = format!(
        "bundle:last-known-good:{}:2026.05.0004",
        import_flow.as_str()
    );
    match grace_posture {
        GracePostureClass::NotInGrace => ("", "", last_known_good_revision),
        GracePostureClass::InGrace => (
            "2026-06-15T00:00:00Z",
            "2026-07-15T00:00:00Z",
            last_known_good_revision,
        ),
        GracePostureClass::GraceExpired => (
            "2026-05-01T00:00:00Z",
            "2026-05-15T00:00:00Z",
            last_known_good_revision,
        ),
    }
}

fn changed_feature_areas_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
) -> Vec<&'static str> {
    match (import_flow, bundle_kind) {
        (BundleImportFlowClass::ManualImport, BundleKindClass::AdminPolicyBundle) => {
            vec!["provider_routing", "network_egress"]
        }
        (_, BundleKindClass::EmergencyDisableBundle) => {
            vec!["extension_activation", "provider_routing"]
        }
        (_, BundleKindClass::TrustRootSignerUpdate) => {
            vec!["bundle_verification", "mirror_admission"]
        }
        (
            BundleImportFlowClass::ManualImport | BundleImportFlowClass::OfflineGrace,
            BundleKindClass::EntitlementSnapshot,
        ) => vec!["seat_entitlement", "managed_feature_quota"],
        _ => Vec::new(),
    }
}

fn degraded_mode_consequences_for(
    bundle_kind: BundleKindClass,
    grace_posture: GracePostureClass,
) -> Vec<&'static str> {
    let mut notes = Vec::new();
    if grace_posture.is_stale() {
        notes.push(
            "Fresh managed privilege is denied while last-known-good continuity remains visible.",
        );
    }
    match bundle_kind {
        BundleKindClass::EmergencyDisableBundle => notes.push(
            "Emergency ratchet remains in force until a signed successor or expiry resolves the disable.",
        ),
        BundleKindClass::TrustRootSignerUpdate => notes.push(
            "Trust-bearing bundle apply pauses until signer continuity and root rotation review complete.",
        ),
        _ => {}
    }
    notes
}

fn offline_notes_for(
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
    grace_posture: GracePostureClass,
) -> Vec<&'static str> {
    match (import_flow, grace_posture, bundle_kind) {
        (BundleImportFlowClass::OfflineGrace, GracePostureClass::InGrace, _) => vec![
            "Using last-known-good signed snapshot within the declared grace window.",
            "Local-safe workflows remain available while managed privilege waits for refresh.",
        ],
        (
            BundleImportFlowClass::OfflineGrace,
            GracePostureClass::GraceExpired,
            BundleKindClass::TrustRootSignerUpdate,
        ) => vec![
            "Trust-root review freshness expired; last-known-good continuity remains inspectable.",
            "New privileged bundle apply is denied until trust-root rotation review lands.",
        ],
        (BundleImportFlowClass::AirGapped, _, _) => vec![
            "Air-gapped transfer preserved the same signed envelope and review vocabulary as managed delivery.",
        ],
        (BundleImportFlowClass::Mirror, _, _) => vec![
            "Mirror publication preserved signature verification and precedence without claiming authoritative live origin.",
        ],
        _ => Vec::new(),
    }
}

fn seeded_lifecycle_events(
    rows: &[FinalizeSignedPolicyBundleRow],
) -> Vec<BundleLifecycleAuditEvent> {
    let apply_row = find_row(
        rows,
        BundleImportFlowClass::Online,
        BundleKindClass::AdminPolicyBundle,
    );
    let supersede_row = find_row(
        rows,
        BundleImportFlowClass::ManualImport,
        BundleKindClass::EntitlementSnapshot,
    );
    let revoke_row = find_row(
        rows,
        BundleImportFlowClass::OfflineGrace,
        BundleKindClass::TrustRootSignerUpdate,
    );
    let rotation_row = find_row(
        rows,
        BundleImportFlowClass::Mirror,
        BundleKindClass::TrustRootSignerUpdate,
    );
    let emergency_row = find_row(
        rows,
        BundleImportFlowClass::ManualImport,
        BundleKindClass::EmergencyDisableBundle,
    );

    vec![
        make_lifecycle_event(
            "bundle-event:apply:0001",
            BundleLifecycleEventClass::Apply,
            apply_row,
            "actor:policy-service:managed-sync",
            "2026-05-31T12:10:00Z",
            "Managed pull applied the current signed admin policy bundle after successful verification.",
        ),
        make_lifecycle_event(
            "bundle-event:supersede:0002",
            BundleLifecycleEventClass::Supersede,
            supersede_row,
            "actor:device-management:ops-alpha",
            "2026-05-29T15:35:00Z",
            "Fleet drop superseded the prior entitlement snapshot while preserving last-known-good lineage.",
        ),
        make_lifecycle_event(
            "bundle-event:revoke:0003",
            BundleLifecycleEventClass::Revoke,
            revoke_row,
            "actor:trust-root-council:on-call",
            "2026-05-16T09:00:00Z",
            "Expired trust-root pointer revoked the prior signer path and paused new privileged imports until review.",
        ),
        make_lifecycle_event(
            "bundle-event:signer-rotation-review:0004",
            BundleLifecycleEventClass::SignerRotationReview,
            rotation_row,
            "actor:security-trust-review:rotation-board",
            "2026-05-30T08:15:00Z",
            "Mirror-distributed signer rotation remained reviewable with old/new lineage and continuity evidence.",
        ),
        make_lifecycle_event(
            "bundle-event:emergency-disable-activation:0005",
            BundleLifecycleEventClass::EmergencyDisableActivation,
            emergency_row,
            "actor:security-response:break-glass",
            "2026-05-29T16:00:00Z",
            "Emergency-disable bundle activated the minimum-version ratchet and paused the affected managed actions without stranding local work.",
        ),
    ]
}

fn find_row(
    rows: &[FinalizeSignedPolicyBundleRow],
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
) -> &FinalizeSignedPolicyBundleRow {
    rows.iter()
        .find(|row| row.import_flow == import_flow && row.bundle_kind == bundle_kind)
        .expect("seeded row present")
}

fn make_lifecycle_event(
    event_id: &str,
    event_class: BundleLifecycleEventClass,
    row: &FinalizeSignedPolicyBundleRow,
    actor_ref: &str,
    event_time: &str,
    note: &str,
) -> BundleLifecycleAuditEvent {
    BundleLifecycleAuditEvent {
        record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_LIFECYCLE_EVENT_RECORD_KIND.to_owned(),
        schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
        shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
        event_id: event_id.to_owned(),
        event_class,
        event_class_token: event_class.as_str().to_owned(),
        bundle_kind: row.bundle_kind,
        bundle_kind_token: row.bundle_kind_token.clone(),
        bundle_ref: row.envelope_review.bundle_ref.clone(),
        scope_ref: row.envelope_review.scope_ref.clone(),
        delivery_source: row.envelope_review.delivery_source,
        delivery_source_token: row.envelope_review.delivery_source_token.clone(),
        actor_ref: actor_ref.to_owned(),
        event_time: event_time.to_owned(),
        supersedes_refs: row.envelope_review.supersedes_refs.clone(),
        revokes_refs: row.envelope_review.revokes_refs.clone(),
        privileged_operation_posture: row.privileged_operation_posture,
        privileged_operation_posture_token: row.privileged_operation_posture_token.clone(),
        local_safe_continuity_preserved: row.local_core_continuity_explicit,
        note: note.to_owned(),
    }
}
