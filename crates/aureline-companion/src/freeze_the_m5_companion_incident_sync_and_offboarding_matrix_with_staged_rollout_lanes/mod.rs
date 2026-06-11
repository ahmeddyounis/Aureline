//! Frozen M5 companion, incident, sync, residency, and offboarding matrix with
//! staged rollout lanes.
//!
//! This module locks the canonical M5 depth qualification for eight lanes across
//! five domains — companion (notification, review follow-up, session follow, and
//! bounded light-edit), incident workspaces, managed sync, customer-managed/E2EE
//! residency, and offboarding continuity — into one export-safe packet. Each
//! [`M5CompanionMatrixLaneRow`] binds a lane to its [`M5CompanionMatrixDomain`],
//! its [`M5CompanionQualificationClass`], its [`M5CompanionRolloutStage`], an
//! explicit [`M5CompanionLocalityDisclosure`] of what stays local, what is staged,
//! and what requires provider or admin continuity, its required evidence packet
//! refs, downgrade triggers, rollback posture, source contracts, and consumer
//! surface parity.
//!
//! The matrix is the single source of truth for whether these lanes may ship as
//! Stable, Beta, Preview, or must narrow further, and at which staged rollout
//! stage. [`M5CompanionMatrixPacket::apply_downgrade_automation`] narrows lanes
//! whose evidence failed validation, whose proof is stale, whose residency or
//! encryption claim is unverified, whose provider or admin continuity is
//! unavailable, or whose upstream dependency narrowed — so CI or release tooling
//! can narrow or withhold a lane automatically instead of shipping greener than
//! the evidence. Browser and mobile companions stay narrow, incident packets stay
//! attributable, managed sync stays inspectable, residency claims stay provable,
//! and offboarding never strands user-owned local work. Credential bodies, raw
//! provider payloads, and raw sync record contents stay outside this boundary.
//!
//! [`canonical_m5_companion_matrix`] builds the frozen matrix and
//! [`current_stable_m5_companion_matrix_export`] reads and validates the
//! checked-in support export, so companion, incident, support, diagnostics, and
//! Help/About surfaces ingest the packet rather than cloning status text.
//!
//! The boundary schema is
//! [`schemas/companion/freeze-the-m5-companion-incident-sync-and-offboarding-matrix-with-staged-rollout-lanes.schema.json`](../../../../schemas/companion/freeze-the-m5-companion-incident-sync-and-offboarding-matrix-with-staged-rollout-lanes.schema.json).
//! The contract doc is
//! [`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`](../../../../docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes/`](../../../../fixtures/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CompanionMatrixPacket`].
pub const M5_COMPANION_MATRIX_RECORD_KIND: &str =
    "freeze_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes";

/// Schema version for M5 companion matrix records.
pub const M5_COMPANION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_COMPANION_MATRIX_SCHEMA_REF: &str =
    "schemas/companion/freeze-the-m5-companion-incident-sync-and-offboarding-matrix-with-staged-rollout-lanes.schema.json";

/// Repo-relative path of the M5 companion matrix contract doc.
pub const M5_COMPANION_MATRIX_DOC_REF: &str =
    "docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md";

/// Repo-relative path of the frozen companion-surface contract (companion lanes).
pub const M5_COMPANION_SURFACE_CONTRACT_REF: &str = "docs/companion/companion_surface_contract.md";

/// Repo-relative path of the browser/mobile companion qualification contract.
pub const M5_COMPANION_QUALIFICATION_REF: &str =
    "docs/help/browser-mobile-companion-surface-qualification.md";

/// Repo-relative path of the browser-companion and embedded boundary manifest with handoff rows.
pub const M5_COMPANION_BOUNDARY_MANIFEST_REF: &str =
    "docs/m5/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows.md";

/// Repo-relative path of the incident-workspace contract.
pub const M5_INCIDENT_WORKSPACE_CONTRACT_REF: &str = "docs/ops/incident_workspace_contract.md";

/// Repo-relative path of the admin policy story register for the companion and sync lanes.
pub const M5_MANAGED_SYNC_POLICY_REF: &str =
    "docs/m5/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes.md";

/// Repo-relative path of the profile-sync and conflict contract.
pub const M5_PROFILE_SYNC_CONTRACT_REF: &str = "docs/profile/profile_sync_and_conflict_contract.md";

/// Repo-relative path of the region-residency contract.
pub const M5_REGION_RESIDENCY_REF: &str = "docs/managed/region_residency_alpha.md";

/// Repo-relative path of the usage-export and offboarding contract.
pub const M5_OFFBOARDING_CONTRACT_REF: &str =
    "docs/governance/usage_export_and_offboarding_contract.md";

/// Repo-relative path of the storage, retention, export, and offboarding matrix for durable artifacts.
pub const M5_OFFBOARDING_RETENTION_MATRIX_REF: &str =
    "docs/m5/publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMPANION_MATRIX_FIXTURE_DIR: &str =
    "fixtures/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMPANION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COMPANION_MATRIX_SUMMARY_REF: &str =
    "artifacts/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md";

/// Domain a companion-matrix lane belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionMatrixDomain {
    /// Browser and mobile companion surfaces.
    Companion,
    /// Incident workspaces.
    Incident,
    /// Managed sync.
    Sync,
    /// Customer-managed-key and end-to-end-encryption residency.
    Residency,
    /// Offboarding continuity.
    Offboarding,
}

impl M5CompanionMatrixDomain {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Companion => "companion",
            Self::Incident => "incident",
            Self::Sync => "sync",
            Self::Residency => "residency",
            Self::Offboarding => "offboarding",
        }
    }
}

/// One of the eight M5 companion/incident/sync/residency/offboarding lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionMatrixLane {
    /// Browser/mobile companion notifications, read-only with no editor authority.
    CompanionNotification,
    /// Companion review/approve follow-up without authoring edits.
    CompanionReview,
    /// Companion session-follow and handoff eligibility.
    CompanionSessionFollow,
    /// Bounded companion light-edit relayed to the host for preview/approval.
    CompanionLightEdit,
    /// Attributable incident workspaces.
    IncidentWorkspace,
    /// Inspectable managed sync of settings, profile, and device registry.
    ManagedSync,
    /// Provable customer-managed-key and end-to-end-encryption residency posture.
    ResidencyEncryption,
    /// Offboarding continuity that never strands user-owned local work.
    OffboardingContinuity,
}

impl M5CompanionMatrixLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CompanionNotification,
        Self::CompanionReview,
        Self::CompanionSessionFollow,
        Self::CompanionLightEdit,
        Self::IncidentWorkspace,
        Self::ManagedSync,
        Self::ResidencyEncryption,
        Self::OffboardingContinuity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompanionNotification => "companion_notification",
            Self::CompanionReview => "companion_review",
            Self::CompanionSessionFollow => "companion_session_follow",
            Self::CompanionLightEdit => "companion_light_edit",
            Self::IncidentWorkspace => "incident_workspace",
            Self::ManagedSync => "managed_sync",
            Self::ResidencyEncryption => "residency_encryption",
            Self::OffboardingContinuity => "offboarding_continuity",
        }
    }

    /// Domain this lane belongs to.
    pub const fn domain(self) -> M5CompanionMatrixDomain {
        match self {
            Self::CompanionNotification
            | Self::CompanionReview
            | Self::CompanionSessionFollow
            | Self::CompanionLightEdit => M5CompanionMatrixDomain::Companion,
            Self::IncidentWorkspace => M5CompanionMatrixDomain::Incident,
            Self::ManagedSync => M5CompanionMatrixDomain::Sync,
            Self::ResidencyEncryption => M5CompanionMatrixDomain::Residency,
            Self::OffboardingContinuity => M5CompanionMatrixDomain::Offboarding,
        }
    }
}

/// Qualification class for a companion-matrix lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionQualificationClass {
    /// Lane qualifies for the Stable claim.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental and not claimed.
    Experimental,
    /// Lane is unavailable on this build.
    Unavailable,
    /// Lane is held pending upstream resolution.
    Held,
}

impl M5CompanionQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the lane may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Narrows the qualification one step toward [`Self::Held`].
    pub const fn narrowed_one_step(self) -> Self {
        match self {
            Self::Stable => Self::Beta,
            Self::Beta => Self::Preview,
            Self::Preview => Self::Experimental,
            Self::Experimental | Self::Unavailable | Self::Held => Self::Held,
        }
    }
}

/// Staged rollout stage for a companion-matrix lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionRolloutStage {
    /// Available to internal builds only.
    InternalOnly,
    /// Early-access behind an explicit capability gate.
    EarlyAccess,
    /// Staged rollout to a subset of cohorts.
    StagedRollout,
    /// Generally available.
    GeneralAvailability,
    /// Withheld from rollout pending recovery.
    Withheld,
}

impl M5CompanionRolloutStage {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InternalOnly => "internal_only",
            Self::EarlyAccess => "early_access",
            Self::StagedRollout => "staged_rollout",
            Self::GeneralAvailability => "general_availability",
            Self::Withheld => "withheld",
        }
    }

    /// Narrows the rollout stage one step toward [`Self::Withheld`].
    pub const fn narrowed_one_step(self) -> Self {
        match self {
            Self::GeneralAvailability => Self::StagedRollout,
            Self::StagedRollout => Self::EarlyAccess,
            Self::EarlyAccess => Self::InternalOnly,
            Self::InternalOnly | Self::Withheld => Self::Withheld,
        }
    }
}

/// Evidence requirement level for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this lane's current qualification.
    NotApplicable,
}

impl M5CompanionEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Required companion relay or sync provider is unavailable.
    ProviderUnavailable,
    /// Managed or admin continuity is required and unavailable.
    AdminContinuityRequired,
    /// Residency or end-to-end-encryption claim could not be verified.
    ResidencyOrEncryptionUnverified,
    /// An incident packet lost its attribution to evidence or build identity.
    IncidentAttributionMissing,
    /// Managed sync could not be inspected or reconciled.
    SyncInspectionUnavailable,
    /// Companion scope expanded beyond its narrow qualified boundary.
    CompanionScopeExpansionUnqualified,
    /// Offboarding would strand user-owned local work.
    OffboardingStrandsLocalWork,
    /// Workspace or device trust narrowed.
    TrustNarrowing,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5CompanionDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::AdminContinuityRequired,
        Self::ResidencyOrEncryptionUnverified,
        Self::IncidentAttributionMissing,
        Self::SyncInspectionUnavailable,
        Self::CompanionScopeExpansionUnqualified,
        Self::OffboardingStrandsLocalWork,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::AdminContinuityRequired => "admin_continuity_required",
            Self::ResidencyOrEncryptionUnverified => "residency_or_encryption_unverified",
            Self::IncidentAttributionMissing => "incident_attribution_missing",
            Self::SyncInspectionUnavailable => "sync_inspection_unavailable",
            Self::CompanionScopeExpansionUnqualified => "companion_scope_expansion_unqualified",
            Self::OffboardingStrandsLocalWork => "offboarding_strands_local_work",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionRollbackPosture {
    /// Local core keeps working with no remote state to roll back.
    LocalCoreContinuesNoRemoteState,
    /// Companion is read-only within a narrow scope and mutates nothing.
    CompanionReadOnlyNarrowScope,
    /// Staged behavior is reversible by narrowing or withdrawing the rollout.
    StagedReversibleViaRollout,
    /// Managed sync reconciles back to the authoritative local core.
    SyncReconcilesToLocalCore,
    /// Offboarding export preserves user-owned local work intact.
    OffboardingExportPreservesLocalWork,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for read-only or non-mutating lanes.
    NotApplicable,
}

impl M5CompanionRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreContinuesNoRemoteState => "local_core_continues_no_remote_state",
            Self::CompanionReadOnlyNarrowScope => "companion_read_only_narrow_scope",
            Self::StagedReversibleViaRollout => "staged_reversible_via_rollout",
            Self::SyncReconcilesToLocalCore => "sync_reconciles_to_local_core",
            Self::OffboardingExportPreservesLocalWork => "offboarding_export_preserves_local_work",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionConsumerSurface {
    /// Desktop companion panel.
    DesktopCompanionPanel,
    /// Browser companion.
    BrowserCompanion,
    /// Mobile companion.
    MobileCompanion,
    /// Incident workspace.
    IncidentWorkspace,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About truth surface.
    HelpAbout,
}

impl M5CompanionConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DesktopCompanionPanel,
        Self::BrowserCompanion,
        Self::MobileCompanion,
        Self::IncidentWorkspace,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopCompanionPanel => "desktop_companion_panel",
            Self::BrowserCompanion => "browser_companion",
            Self::MobileCompanion => "mobile_companion",
            Self::IncidentWorkspace => "incident_workspace",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Explicit locality disclosure for a lane.
///
/// Every incident, sync, residency, encryption, and offboarding row must say what
/// stays local, what is staged, and what requires provider or admin continuity, so
/// no companion or managed surface can imply a hidden control plane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionLocalityDisclosure {
    /// What stays on the local core and works offline.
    pub stays_local: String,
    /// What is staged behind a rollout cohort or capability gate.
    pub staged: String,
    /// What requires provider or admin continuity to function.
    pub requires_provider_or_admin_continuity: String,
}

/// One row in the M5 companion matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionMatrixLaneRow {
    /// Lane.
    pub lane: M5CompanionMatrixLane,
    /// Domain the lane belongs to.
    pub domain: M5CompanionMatrixDomain,
    /// Qualification class earned by this lane.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Explicit locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Evidence requirement level.
    pub evidence_requirement: M5CompanionEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionMatrixSecurityReview {
    /// Browser and mobile companions stay narrow.
    pub companions_stay_narrow: bool,
    /// No companion or managed surface implies a hidden control plane.
    pub no_hidden_control_plane: bool,
    /// No companion or managed surface implies a second flagship.
    pub no_second_flagship_implied: bool,
    /// Incident packets stay attributable to evidence and build identity.
    pub incident_packets_attributable: bool,
    /// Managed sync stays inspectable and reconcilable.
    pub managed_sync_inspectable: bool,
    /// Customer-managed-key and E2EE residency claims stay provable.
    pub residency_and_e2ee_claims_provable: bool,
    /// Offboarding never strands user-owned local work.
    pub offboarding_never_strands_local_work: bool,
    /// Every row discloses local, staged, and provider/admin continuity.
    pub locality_disclosed_per_row: bool,
    /// No credential bodies or raw provider payloads cross the export boundary.
    pub no_credential_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified lanes automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionMatrixConsumerProjection {
    /// Desktop companion panel shows qualification truth.
    pub desktop_companion_shows_qualification: bool,
    /// Browser companion shows qualification truth.
    pub browser_companion_shows_qualification: bool,
    /// Mobile companion shows qualification truth.
    pub mobile_companion_shows_qualification: bool,
    /// Incident workspace shows qualification truth.
    pub incident_workspace_shows_qualification: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_qualification: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_qualification: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Preview / Labs lanes are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_lanes: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Per-lane observation fed to [`M5CompanionMatrixPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompanionMatrixLaneObservation {
    /// Lane the observation applies to.
    pub lane: M5CompanionMatrixLane,
    /// True when the lane's evidence currently validates.
    pub evidence_valid: bool,
    /// True when the lane's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when the required provider or admin continuity is available.
    pub provider_or_admin_available: bool,
    /// True when the lane's residency and encryption claims are verified.
    pub residency_and_encryption_verified: bool,
    /// True when an upstream dependency of the lane narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`M5CompanionMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompanionMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5CompanionMatrixLaneRow>,
    /// Security review block.
    pub security_review: M5CompanionMatrixSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompanionMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompanionMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 companion matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionMatrixPacket {
    /// Record kind; must equal [`M5_COMPANION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPANION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5CompanionMatrixLaneRow>,
    /// Security review block.
    pub security_review: M5CompanionMatrixSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompanionMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompanionMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CompanionMatrixPacket {
    /// Builds an M5 companion matrix packet from stable-lane input.
    pub fn new(input: M5CompanionMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_COMPANION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_COMPANION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            lane_rows: input.lane_rows,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows lanes whose evidence failed validation, whose proof is stale, whose
    /// residency or encryption claim is unverified, whose provider or admin
    /// continuity is unavailable, or whose upstream dependency narrowed.
    ///
    /// Invalid evidence holds the lane and withholds its rollout. Any other adverse
    /// signal narrows the qualification and rollout stage one step each. Lanes
    /// without an observation are left unchanged; observations for lanes not present
    /// in the packet are ignored.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[M5CompanionMatrixLaneObservation],
    ) {
        for row in &mut self.lane_rows {
            let Some(observation) = observations.iter().find(|obs| obs.lane == row.lane) else {
                continue;
            };
            if !observation.evidence_valid {
                row.qualification = M5CompanionQualificationClass::Held;
                row.rollout_stage = M5CompanionRolloutStage::Withheld;
                continue;
            }
            let adverse = !observation.proof_fresh
                || !observation.provider_or_admin_available
                || !observation.residency_and_encryption_verified
                || observation.upstream_narrowed;
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }
    }

    /// Validates the M5 companion matrix invariants.
    pub fn validate(&self) -> Vec<M5CompanionMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPANION_MATRIX_RECORD_KIND {
            violations.push(M5CompanionMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPANION_MATRIX_SCHEMA_VERSION {
            violations.push(M5CompanionMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CompanionMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 companion matrix packet serializes"),
        ) {
            violations.push(M5CompanionMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 companion matrix packet serializes")
    }

    /// Lanes currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_lanes(&self) -> impl Iterator<Item = &M5CompanionMatrixLaneRow> {
        self.lane_rows.iter().filter(|row| {
            matches!(
                row.qualification,
                M5CompanionQualificationClass::Stable
                    | M5CompanionQualificationClass::Beta
                    | M5CompanionQualificationClass::Preview
            ) && row.rollout_stage != M5CompanionRolloutStage::Withheld
        })
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lanes = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Companion, Incident, Sync, Residency, and Offboarding Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_rows.len(),
            stable_lanes
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Lanes\n\n");
        for row in &self.lane_rows {
            out.push_str(&format!(
                "- **{}** ({}): `{}` / `{}`\n",
                row.lane.as_str(),
                row.domain.as_str(),
                row.qualification.as_str(),
                row.rollout_stage.as_str(),
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Local: {}\n",
                row.locality_disclosure.stays_local
            ));
            out.push_str(&format!("  - Staged: {}\n", row.locality_disclosure.staged));
            out.push_str(&format!(
                "  - Requires continuity: {}\n",
                row.locality_disclosure
                    .requires_provider_or_admin_continuity
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 companion matrix export.
#[derive(Debug)]
pub enum M5CompanionMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CompanionMatrixViolation>),
}

impl fmt::Display for M5CompanionMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 companion matrix export parse failed: {error}"
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
                    "m5 companion matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CompanionMatrixArtifactError {}

/// Validation failures emitted by [`M5CompanionMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CompanionMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required lane is missing from the matrix.
    RequiredLaneMissing,
    /// A lane row's declared domain does not match its lane.
    LaneDomainMismatch,
    /// A lane row is incomplete.
    LaneRowIncomplete,
    /// A lane row's locality disclosure is incomplete.
    LocalityDisclosureIncomplete,
    /// A lane claiming Stable is missing required evidence packet refs.
    StableLaneMissingEvidence,
    /// A lane has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A lane has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CompanionMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::LaneDomainMismatch => "lane_domain_mismatch",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::StableLaneMissingEvidence => "stable_lane_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 companion matrix export.
///
/// This is the canonical reader: a companion panel, incident workspace,
/// diagnostics, support-export, or Help/About surface calls it to ingest the
/// matrix packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`M5CompanionMatrixArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_stable_m5_companion_matrix_export(
) -> Result<M5CompanionMatrixPacket, M5CompanionMatrixArtifactError> {
    let packet: M5CompanionMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes/support_export.json"
    )))
    .map_err(M5CompanionMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompanionMatrixArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every matrix export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
        M5_COMPANION_MATRIX_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_QUALIFICATION_REF.to_owned(),
        M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
        M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_MANAGED_SYNC_POLICY_REF.to_owned(),
        M5_PROFILE_SYNC_CONTRACT_REF.to_owned(),
        M5_REGION_RESIDENCY_REF.to_owned(),
        M5_OFFBOARDING_CONTRACT_REF.to_owned(),
        M5_OFFBOARDING_RETENTION_MATRIX_REF.to_owned(),
    ]
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> M5CompanionMatrixSecurityReview {
    M5CompanionMatrixSecurityReview {
        companions_stay_narrow: true,
        no_hidden_control_plane: true,
        no_second_flagship_implied: true,
        incident_packets_attributable: true,
        managed_sync_inspectable: true,
        residency_and_e2ee_claims_provable: true,
        offboarding_never_strands_local_work: true,
        locality_disclosed_per_row: true,
        no_credential_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting qualification truth.
pub fn canonical_consumer_projection() -> M5CompanionMatrixConsumerProjection {
    M5CompanionMatrixConsumerProjection {
        desktop_companion_shows_qualification: true,
        browser_companion_shows_qualification: true,
        mobile_companion_shows_qualification: true,
        incident_workspace_shows_qualification: true,
        cli_headless_shows_qualification: true,
        support_export_shows_qualification: true,
        diagnostics_shows_qualification: true,
        help_about_shows_qualification: true,
        preview_labs_label_for_unqualified_lanes: true,
    }
}

/// Builds the canonical frozen M5 companion matrix from the eight lane descriptors.
///
/// This is the first consumer: it mints the matrix the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed lane definitions.
pub fn canonical_m5_companion_matrix(
    packet_id: String,
    matrix_label: String,
    minted_at: String,
    proof_freshness: M5CompanionMatrixProofFreshness,
) -> M5CompanionMatrixPacket {
    let lane_rows = M5CompanionMatrixLane::ALL
        .into_iter()
        .map(lane_descriptor)
        .collect::<Vec<_>>();

    M5CompanionMatrixPacket::new(M5CompanionMatrixPacketInput {
        packet_id,
        matrix_label,
        lane_rows,
        security_review: canonical_security_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

fn locality(stays_local: &str, staged: &str, continuity: &str) -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local: stays_local.to_owned(),
        staged: staged.to_owned(),
        requires_provider_or_admin_continuity: continuity.to_owned(),
    }
}

fn lane_descriptor(lane: M5CompanionMatrixLane) -> M5CompanionMatrixLaneRow {
    use M5CompanionConsumerSurface as Surface;
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionEvidenceRequirement as Evidence;
    use M5CompanionMatrixLane as Lane;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    match lane {
        Lane::CompanionNotification => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            scope_summary: "Browser/mobile companion notifications for build, review, and agent events — read-only with no editor authority".to_owned(),
            locality_disclosure: locality(
                "Notification source events are computed by the local core and stay inspectable offline.",
                "Companion push fan-out to paired browser/mobile sessions rolls out per cohort.",
                "Paired companion delivery requires the companion relay; the local core never depends on it to function.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:companion-surface-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
            source_contract_refs: vec![
                M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
                M5_COMPANION_QUALIFICATION_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::DesktopCompanionPanel,
                Surface::BrowserCompanion,
                Surface::MobileCompanion,
                Surface::SupportExport,
            ],
        },
        Lane::CompanionReview => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            scope_summary: "Companion review/approve follow-up: inspect findings and approve or defer pre-staged actions without authoring edits".to_owned(),
            locality_disclosure: locality(
                "Review content and decision records are authored and stored by the local core.",
                "Companion-initiated approvals roll out behind staged cohorts.",
                "Relaying an approval to a running desktop session requires the companion relay and an active host.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:companion-review-followup:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
            source_contract_refs: vec![
                M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
                M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::BrowserCompanion,
                Surface::MobileCompanion,
                Surface::SupportExport,
            ],
        },
        Lane::CompanionSessionFollow => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            scope_summary: "Companion session-follow and handoff eligibility: observe an active desktop session and resume context, narrow to read plus handoff".to_owned(),
            locality_disclosure: locality(
                "Session state and handoff records originate on the local core and survive offline.",
                "Session-follow streaming is staged per cohort and capability.",
                "Live follow requires an active host session and the companion relay; eligibility narrows when either is unavailable.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:companion-handoff-eligibility:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
            source_contract_refs: vec![
                M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
                M5_COMPANION_QUALIFICATION_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::DesktopCompanionPanel,
                Surface::BrowserCompanion,
                Surface::MobileCompanion,
                Surface::SupportExport,
            ],
        },
        Lane::CompanionLightEdit => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            scope_summary: "Companion light-edit: bounded text touch-ups relayed to the host for preview/approval, never a full mobile editor".to_owned(),
            locality_disclosure: locality(
                "Edits apply only through the host's local change pipeline with preview and revert.",
                "Light-edit is early-access behind an explicit capability gate.",
                "Applying a light-edit requires an active host session and the companion relay; without them it is read-only.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:companion-light-edit-bounds:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::StagedReversibleViaRollout,
            source_contract_refs: vec![
                M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
                M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::BrowserCompanion,
                Surface::MobileCompanion,
                Surface::SupportExport,
            ],
        },
        Lane::IncidentWorkspace => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            scope_summary: "Attributable incident workspaces binding crash trails, evidence spans, and runbook steps to a redacted support bundle preview".to_owned(),
            locality_disclosure: locality(
                "Incident evidence, missing-span facts, and runbook records are local-first and inspectable offline.",
                "Cross-device incident sharing rolls out per cohort.",
                "Escalation to a managed support channel requires admin continuity; local triage never depends on it.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:incident-workspace-attribution:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::IncidentAttributionMissing,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
            source_contract_refs: vec![M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                Surface::IncidentWorkspace,
                Surface::CliHeadless,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        Lane::ManagedSync => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            scope_summary: "Inspectable managed sync of settings, profile, and device registry with conflict review and no silent server authority".to_owned(),
            locality_disclosure: locality(
                "The local core is the source of truth; every synced record stays inspectable and reconcilable offline.",
                "Managed sync expands per cohort and record class.",
                "Server-side sync and conflict relay require the sync provider and, for managed tenants, admin continuity.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:managed-sync-inspectability:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::AdminContinuityRequired,
                Trigger::SyncInspectionUnavailable,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::SyncReconcilesToLocalCore,
            source_contract_refs: vec![
                M5_MANAGED_SYNC_POLICY_REF.to_owned(),
                M5_PROFILE_SYNC_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::DesktopCompanionPanel,
                Surface::CliHeadless,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        Lane::ResidencyEncryption => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            scope_summary: "Provable customer-managed-key and end-to-end-encryption residency posture for managed/synced artifacts, with region pinning".to_owned(),
            locality_disclosure: locality(
                "Local-only artifacts never leave the device and carry no residency dependency.",
                "Customer-managed-key and region pinning roll out per managed tenant.",
                "End-to-end-encryption and region-residency guarantees require the managed key authority and admin continuity, and are claimed only when verifiable.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:residency-and-e2ee-proof:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ResidencyOrEncryptionUnverified,
                Trigger::AdminContinuityRequired,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::EvidencePreservedNoRevert,
            source_contract_refs: vec![
                M5_REGION_RESIDENCY_REF.to_owned(),
                M5_MANAGED_SYNC_POLICY_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::CliHeadless,
                Surface::SupportExport,
                Surface::Diagnostics,
                Surface::HelpAbout,
            ],
        },
        Lane::OffboardingContinuity => M5CompanionMatrixLaneRow {
            lane,
            domain: lane.domain(),
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            scope_summary: "Offboarding that exports user-owned durable artifacts and guarantees the local core keeps working after any managed teardown".to_owned(),
            locality_disclosure: locality(
                "User-owned local work and history remain on the device and fully usable after offboarding.",
                "Bulk managed-export tooling rolls out per cohort.",
                "Revoking managed/cloud artifacts requires admin continuity; local-core continuity is never gated on it.",
            ),
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec![
                "evidence:offboarding-local-continuity:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::OffboardingStrandsLocalWork,
                Trigger::AdminContinuityRequired,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::OffboardingExportPreservesLocalWork,
            source_contract_refs: vec![
                M5_OFFBOARDING_CONTRACT_REF.to_owned(),
                M5_OFFBOARDING_RETENTION_MATRIX_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                Surface::CliHeadless,
                Surface::SupportExport,
                Surface::Diagnostics,
                Surface::HelpAbout,
            ],
        },
    }
}

fn validate_source_contracts(
    packet: &M5CompanionMatrixPacket,
    violations: &mut Vec<M5CompanionMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMPANION_MATRIX_SCHEMA_REF,
        M5_COMPANION_MATRIX_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_INCIDENT_WORKSPACE_CONTRACT_REF,
        M5_MANAGED_SYNC_POLICY_REF,
        M5_REGION_RESIDENCY_REF,
        M5_OFFBOARDING_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CompanionMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_rows(
    packet: &M5CompanionMatrixPacket,
    violations: &mut Vec<M5CompanionMatrixViolation>,
) {
    let present: BTreeSet<M5CompanionMatrixLane> =
        packet.lane_rows.iter().map(|row| row.lane).collect();
    for required in M5CompanionMatrixLane::ALL {
        if !present.contains(&required) {
            violations.push(M5CompanionMatrixViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.domain != row.lane.domain() {
            violations.push(M5CompanionMatrixViolation::LaneDomainMismatch);
        }
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5CompanionMatrixViolation::LaneRowIncomplete);
        }
        if row.locality_disclosure.stays_local.trim().is_empty()
            || row.locality_disclosure.staged.trim().is_empty()
            || row
                .locality_disclosure
                .requires_provider_or_admin_continuity
                .trim()
                .is_empty()
        {
            violations.push(M5CompanionMatrixViolation::LocalityDisclosureIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5CompanionMatrixViolation::StableLaneMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CompanionMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CompanionMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_security_review(
    packet: &M5CompanionMatrixPacket,
    violations: &mut Vec<M5CompanionMatrixViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.companions_stay_narrow,
        review.no_hidden_control_plane,
        review.no_second_flagship_implied,
        review.incident_packets_attributable,
        review.managed_sync_inspectable,
        review.residency_and_e2ee_claims_provable,
        review.offboarding_never_strands_local_work,
        review.locality_disclosed_per_row,
        review.no_credential_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5CompanionMatrixViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CompanionMatrixPacket,
    violations: &mut Vec<M5CompanionMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_companion_shows_qualification,
        projection.browser_companion_shows_qualification,
        projection.mobile_companion_shows_qualification,
        projection.incident_workspace_shows_qualification,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_qualification,
        projection.diagnostics_shows_qualification,
        projection.help_about_shows_qualification,
        projection.preview_labs_label_for_unqualified_lanes,
    ] {
        if !ok {
            violations.push(M5CompanionMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CompanionMatrixPacket,
    violations: &mut Vec<M5CompanionMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CompanionMatrixViolation::ProofFreshnessIncomplete);
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
