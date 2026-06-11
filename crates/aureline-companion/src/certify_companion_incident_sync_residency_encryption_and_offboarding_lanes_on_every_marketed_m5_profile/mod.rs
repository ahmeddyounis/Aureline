//! Certification of the companion, incident, sync, residency, encryption, and
//! offboarding lanes on every marketed M5 profile.
//!
//! This module locks the canonical M5 companion-depth certification into one
//! export-safe packet. Each [`CompanionLaneCertification`] binds one frozen
//! matrix lane — companion notification, review follow-up, session follow,
//! bounded light-edit, incident workspaces, managed sync, residency/encryption,
//! and offboarding continuity — to the qualification it holds on every marketed
//! M5 profile, an explicit per-profile locality disclosure of what stays local
//! and what requires provider or admin continuity, and a closed set of downgrade
//! rules that narrow the claim instead of hiding the lane.
//!
//! The certification is the single source of truth for whether each companion
//! lane may keep its public claim on a given marketed profile — the local-solo,
//! team-managed, enterprise-managed, browser-companion, mobile-companion, and
//! air-gapped-offline profiles qualified in this batch. It reuses the
//! qualification, rollout, downgrade-trigger, and locality vocabularies frozen by
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! rather than inventing parallel terms, and it binds every certified lane to
//! that frozen matrix as its ceiling: a lane may never be certified greener than
//! the matrix admits, so no surface may stay greener than this packet. The packet
//! refuses to certify a managed lane (managed sync, residency/encryption) on a
//! profile with no managed plane, refuses a relay-bound companion lane on an
//! air-gapped profile, refuses to claim a profile while hiding what stays local
//! or what requires continuity, refuses to strand user-owned local work on any
//! profile, and narrows rather than hides on stale proof, an unverified residency
//! or encryption claim, an unavailable provider or admin plane, or a narrowed
//! upstream matrix lane.
//!
//! Browser and mobile companions stay narrow, incident packets stay attributable,
//! managed sync stays inspectable, customer-managed/E2EE residency claims stay
//! provable, and offboarding never strands user-owned local work — incident and
//! offboarding lanes stay certified even on the air-gapped profile because they
//! are local-first. Credential bodies, raw provider payloads, and raw sync record
//! contents stay outside this boundary; the packet carries only typed disclosure
//! booleans, class tokens, and review-safe summaries.
//!
//! The boundary schema is
//! [`schemas/companion/certify-companion-incident-sync-residency-encryption-and-offboarding-lanes-on-every-marketed-m5-profile.schema.json`](../../../../schemas/companion/certify-companion-incident-sync-residency-encryption-and-offboarding-lanes-on-every-marketed-m5-profile.schema.json).
//! The contract doc is
//! [`docs/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile.md`](../../../../docs/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/`](../../../../fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/).

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    current_stable_m5_companion_matrix_export, M5CompanionDowngradeTrigger,
    M5CompanionLocalityDisclosure, M5CompanionMatrixDomain, M5CompanionMatrixLane,
    M5CompanionQualificationClass, M5CompanionRolloutStage, M5_COMPANION_MATRIX_ARTIFACT_REF,
    M5_COMPANION_MATRIX_DOC_REF, M5_COMPANION_MATRIX_SCHEMA_REF, M5_COMPANION_SURFACE_CONTRACT_REF,
    M5_INCIDENT_WORKSPACE_CONTRACT_REF, M5_MANAGED_SYNC_POLICY_REF, M5_OFFBOARDING_CONTRACT_REF,
    M5_REGION_RESIDENCY_REF,
};

/// Stable record-kind tag carried by [`M5CompanionCertificationPacket`].
pub const M5_COMPANION_CERTIFICATION_RECORD_KIND: &str =
    "certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile";

/// Schema version for M5 companion certification records.
pub const M5_COMPANION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_COMPANION_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/companion/certify-companion-incident-sync-residency-encryption-and-offboarding-lanes-on-every-marketed-m5-profile.schema.json";

/// Repo-relative path of the certification contract doc.
pub const M5_COMPANION_CERTIFICATION_DOC_REF: &str =
    "docs/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMPANION_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMPANION_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COMPANION_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile.md";

/// One marketed M5 profile a companion lane is certified against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketedM5Profile {
    /// Individual local-first install with no managed control plane.
    LocalSolo,
    /// Team on a managed sync and cloud plane.
    TeamManaged,
    /// Enterprise managed tenant with admin continuity, customer-managed keys,
    /// and region residency.
    EnterpriseManaged,
    /// Browser companion channel paired to a desktop host.
    BrowserCompanion,
    /// Mobile companion channel paired to a desktop host.
    MobileCompanion,
    /// Air-gapped or offline-only install with no provider or admin continuity.
    AirGappedOffline,
}

impl MarketedM5Profile {
    /// Every marketed profile, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalSolo,
        Self::TeamManaged,
        Self::EnterpriseManaged,
        Self::BrowserCompanion,
        Self::MobileCompanion,
        Self::AirGappedOffline,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSolo => "local_solo",
            Self::TeamManaged => "team_managed",
            Self::EnterpriseManaged => "enterprise_managed",
            Self::BrowserCompanion => "browser_companion",
            Self::MobileCompanion => "mobile_companion",
            Self::AirGappedOffline => "air_gapped_offline",
        }
    }

    /// Whether this profile provides a managed sync and residency control plane.
    ///
    /// Managed lanes (managed sync, residency/encryption) are only marketable on
    /// a profile that provides a managed plane.
    pub const fn provides_managed_plane(self) -> bool {
        matches!(self, Self::TeamManaged | Self::EnterpriseManaged)
    }

    /// Whether this profile can reach a companion relay.
    ///
    /// Relay-bound companion lanes (notification, review, session follow, light
    /// edit) are only marketable on a profile that can reach the relay; the
    /// air-gapped profile cannot.
    pub const fn provides_companion_relay(self) -> bool {
        !matches!(self, Self::AirGappedOffline)
    }

    /// Whether this profile is itself a browser or mobile companion channel.
    pub const fn is_companion_channel(self) -> bool {
        matches!(self, Self::BrowserCompanion | Self::MobileCompanion)
    }

    /// Whether this profile is isolated from any provider or admin continuity.
    pub const fn is_air_gapped(self) -> bool {
        matches!(self, Self::AirGappedOffline)
    }
}

/// Freshness state recorded for a per-profile certification row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFreshness {
    /// Proof is within its freshness SLO.
    Fresh,
    /// Proof has gone stale and the row is labeled rather than shown as live.
    Stale,
    /// Freshness could not be determined and is labeled as unknown.
    Unknown,
}

impl CertificationFreshness {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this freshness state must carry an explicit label.
    pub const fn requires_label(self) -> bool {
        matches!(self, Self::Stale | Self::Unknown)
    }
}

/// How one lane is certified on one marketed profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCertificationRow {
    /// Marketed profile this row covers.
    pub profile: MarketedM5Profile,
    /// Whether the lane carries a public claim on this profile.
    pub certified_on_profile: bool,
    /// Qualification the lane holds on this profile.
    pub profile_qualification: M5CompanionQualificationClass,
    /// Staged rollout stage the lane holds on this profile.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Explicit locality disclosure for this lane on this profile.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Whether this lane requires provider or admin continuity on this profile.
    pub requires_provider_or_admin_continuity: bool,
    /// Whether the local core keeps working and never strands user-owned work on
    /// this profile.
    pub local_core_continuity_preserved: bool,
    /// Proof freshness state for this row.
    pub freshness: CertificationFreshness,
    /// Whether a stale or unknown freshness state is labeled rather than shown
    /// as live.
    pub freshness_label_shown: bool,
}

/// One downgrade rule that narrows a lane's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionLaneDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5CompanionDowngradeTrigger,
    /// Qualification the lane narrows to when the trigger fires.
    pub narrowed_to: M5CompanionQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// Full certification for one companion-matrix lane across every marketed
/// profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionLaneCertification {
    /// Frozen matrix lane being certified.
    pub lane: M5CompanionMatrixLane,
    /// Domain the lane belongs to.
    pub domain: M5CompanionMatrixDomain,
    /// Headline qualification class claimed for this lane.
    pub claimed_qualification: M5CompanionQualificationClass,
    /// Frozen matrix baseline qualification that ceilings this claim.
    pub matrix_baseline_qualification: M5CompanionQualificationClass,
    /// Review-safe scope summary.
    pub scope_summary: String,
    /// Per-profile certification rows.
    pub profile_coverage: Vec<ProfileCertificationRow>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<CompanionLaneDowngradeRule>,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl CompanionLaneCertification {
    /// Qualification this lane narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches the
    /// trigger; this is the deterministic downgrade automation consumers and
    /// release tooling project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5CompanionDowngradeTrigger,
    ) -> M5CompanionQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Whether this lane carries a publicly claimed qualification.
    pub fn is_claimed(&self) -> bool {
        is_claimed_qualification(self.claimed_qualification)
    }

    /// Whether this lane is marketable on `profile`.
    ///
    /// A managed lane needs a managed plane; a relay-bound companion lane needs a
    /// reachable relay; incident and offboarding lanes are local-first and stay
    /// marketable on every profile.
    pub fn marketable_on(&self, profile: MarketedM5Profile) -> bool {
        lane_marketable_on_profile(self.lane, profile)
    }
}

/// Security and privacy review block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionCertificationSecurityReview {
    /// No lane is certified greener than the frozen matrix admits.
    pub no_lane_greener_than_matrix: bool,
    /// Browser and mobile companions stay narrow.
    pub companions_stay_narrow: bool,
    /// Incident packets stay attributable on every certified profile.
    pub incident_packets_attributable: bool,
    /// Managed sync stays inspectable on every certified profile.
    pub managed_sync_inspectable: bool,
    /// Customer-managed-key and E2EE residency claims stay provable.
    pub residency_and_e2ee_claims_provable: bool,
    /// Offboarding never strands user-owned local work on any profile.
    pub offboarding_never_strands_local_work: bool,
    /// Every certified row discloses what stays local and what requires continuity.
    pub locality_disclosed_per_profile: bool,
    /// No managed lane is certified on a profile without a managed plane.
    pub no_managed_claim_without_managed_plane: bool,
    /// No companion or managed surface implies a hidden control plane.
    pub no_hidden_control_plane: bool,
    /// No credential bodies or raw provider payloads cross the export boundary.
    pub no_credential_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionCertificationConsumerProjection {
    /// Desktop companion panel shows the per-profile certification truth.
    pub desktop_companion_shows_certification: bool,
    /// Browser companion shows the per-profile certification truth.
    pub browser_companion_shows_certification: bool,
    /// Mobile companion shows the per-profile certification truth.
    pub mobile_companion_shows_certification: bool,
    /// Incident workspace shows the per-profile certification truth.
    pub incident_workspace_shows_certification: bool,
    /// CLI / headless shows the per-profile certification truth.
    pub cli_headless_shows_certification: bool,
    /// Support export shows the per-profile certification truth.
    pub support_export_shows_certification: bool,
    /// Diagnostics shows the per-profile certification truth.
    pub diagnostics_shows_certification: bool,
    /// Help / About shows the per-profile certification truth.
    pub help_about_shows_certification: bool,
    /// Unqualified or not-marketed rows are visibly labeled instead of optimistic.
    pub unqualified_rows_labeled: bool,
}

/// Proof freshness block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed lanes.
    pub auto_narrow_on_stale: bool,
}

/// Reason a certification row or lane was narrowed by downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionCertificationDegradedReason {
    /// Proof packet has gone stale.
    ProofStale,
    /// Evidence packet failed validation or is missing.
    EvidenceInvalid,
    /// Required companion relay or sync provider is unavailable.
    ProviderUnavailable,
    /// Managed or admin continuity is required and unavailable.
    AdminContinuityRequired,
    /// Residency or end-to-end-encryption claim could not be verified.
    ResidencyOrEncryptionUnverified,
    /// An upstream matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// Freshness was downgraded to stale and labeled.
    FreshnessDowngradedToStale,
}

impl CompanionCertificationDegradedReason {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidenceInvalid => "evidence_invalid",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::AdminContinuityRequired => "admin_continuity_required",
            Self::ResidencyOrEncryptionUnverified => "residency_or_encryption_unverified",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
        }
    }
}

/// Per-lane observation fed to
/// [`M5CompanionCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionCertificationObservation {
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
    /// True when an upstream matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Constructor input for [`M5CompanionCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompanionCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-lane certifications.
    pub lane_certifications: Vec<CompanionLaneCertification>,
    /// Security review block.
    pub security_review: M5CompanionCertificationSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompanionCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompanionCertificationProofFreshness,
    /// Degraded labels recorded by downgrade automation.
    pub degraded_labels: Vec<CompanionCertificationDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 companion certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionCertificationPacket {
    /// Record kind; must equal [`M5_COMPANION_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPANION_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-lane certifications.
    pub lane_certifications: Vec<CompanionLaneCertification>,
    /// Security review block.
    pub security_review: M5CompanionCertificationSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompanionCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompanionCertificationProofFreshness,
    /// Degraded labels recorded by downgrade automation.
    pub degraded_labels: Vec<CompanionCertificationDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CompanionCertificationPacket {
    /// Builds an M5 companion certification packet from stable-lane input.
    pub fn new(input: M5CompanionCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_COMPANION_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_COMPANION_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            lane_certifications: input.lane_certifications,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            degraded_labels: input.degraded_labels,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows lanes whose evidence failed validation, whose proof is stale,
    /// whose residency or encryption claim is unverified, whose provider or admin
    /// continuity is unavailable, or whose upstream matrix lane narrowed.
    ///
    /// Invalid evidence holds the lane and withholds every certified profile row.
    /// Any other adverse signal narrows the lane's headline claim and every
    /// certified profile row one step each, and a stale proof forces every row to
    /// a labeled stale state. Lanes without an observation are left unchanged;
    /// observations for lanes not present in the packet are ignored. The narrowing
    /// stays at or below the frozen matrix baseline, so the certification can only
    /// shrink a claim, never widen it.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[CompanionCertificationObservation],
    ) {
        let mut labels: BTreeSet<CompanionCertificationDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        for cert in &mut self.lane_certifications {
            let Some(observation) = observations.iter().find(|obs| obs.lane == cert.lane) else {
                continue;
            };

            if !observation.evidence_valid {
                labels.insert(CompanionCertificationDegradedReason::EvidenceInvalid);
                cert.claimed_qualification = M5CompanionQualificationClass::Held;
                for row in &mut cert.profile_coverage {
                    row.profile_qualification = M5CompanionQualificationClass::Held;
                    row.rollout_stage = M5CompanionRolloutStage::Withheld;
                    row.certified_on_profile = false;
                }
                continue;
            }

            let adverse = !observation.proof_fresh
                || !observation.provider_or_admin_available
                || !observation.residency_and_encryption_verified
                || observation.upstream_matrix_narrowed;
            if !adverse {
                continue;
            }

            if !observation.proof_fresh {
                labels.insert(CompanionCertificationDegradedReason::ProofStale);
                labels.insert(CompanionCertificationDegradedReason::FreshnessDowngradedToStale);
            }
            if !observation.provider_or_admin_available {
                labels.insert(CompanionCertificationDegradedReason::ProviderUnavailable);
            }
            if !observation.residency_and_encryption_verified {
                labels
                    .insert(CompanionCertificationDegradedReason::ResidencyOrEncryptionUnverified);
            }
            if observation.upstream_matrix_narrowed {
                labels.insert(CompanionCertificationDegradedReason::UpstreamMatrixNarrowed);
            }

            cert.claimed_qualification = cert.claimed_qualification.narrowed_one_step();
            for row in &mut cert.profile_coverage {
                if row.certified_on_profile {
                    row.profile_qualification = row.profile_qualification.narrowed_one_step();
                    row.rollout_stage = row.rollout_stage.narrowed_one_step();
                    row.certified_on_profile = is_claimed_qualification(row.profile_qualification);
                }
                if !observation.proof_fresh {
                    row.freshness = CertificationFreshness::Stale;
                    row.freshness_label_shown = true;
                }
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Validates the M5 companion certification invariants.
    pub fn validate(&self) -> Vec<M5CompanionCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPANION_CERTIFICATION_RECORD_KIND {
            violations.push(M5CompanionCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPANION_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5CompanionCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CompanionCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lanes_present(self, &mut violations);

        let matrix_baseline = matrix_baseline_qualifications();
        if matrix_baseline.is_none() {
            violations.push(M5CompanionCertificationViolation::MatrixBaselineUnavailable);
        }
        for cert in &self.lane_certifications {
            validate_lane_certification(cert, matrix_baseline.as_ref(), &mut violations);
        }

        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 companion certification packet serializes"),
        ) {
            violations.push(M5CompanionCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of lanes whose headline qualification is Stable.
    pub fn stable_lane_count(&self) -> usize {
        self.lane_certifications
            .iter()
            .filter(|cert| cert.claimed_qualification.is_stable())
            .count()
    }

    /// Count of (lane, profile) pairs that carry a public certification.
    pub fn certified_profile_count(&self) -> usize {
        self.lane_certifications
            .iter()
            .flat_map(|cert| cert.profile_coverage.iter())
            .filter(|row| row.certified_on_profile)
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 companion certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Companion Lane Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_certifications.len(),
            self.stable_lane_count()
        ));
        out.push_str(&format!(
            "- Certified (lane, profile) pairs: {}\n",
            self.certified_profile_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        if self.degraded_labels.is_empty() {
            out.push_str("- Degraded: none\n");
        } else {
            out.push_str(&format!(
                "- Degraded: {}\n",
                self.degraded_labels
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out.push_str("\n## Lanes\n\n");
        for cert in &self.lane_certifications {
            out.push_str(&format!(
                "- **{}** ({}): `{}` (matrix baseline `{}`)\n",
                cert.lane.as_str(),
                cert.domain.as_str(),
                cert.claimed_qualification.as_str(),
                cert.matrix_baseline_qualification.as_str(),
            ));
            out.push_str(&format!("  - Scope: {}\n", cert.scope_summary));
            out.push_str("  - Profiles:\n");
            for row in &cert.profile_coverage {
                out.push_str(&format!(
                    "    - `{}`: {} ({})\n",
                    row.profile.as_str(),
                    if row.certified_on_profile {
                        row.profile_qualification.as_str()
                    } else {
                        "not marketed"
                    },
                    row.rollout_stage.as_str(),
                ));
            }
            out.push_str(&format!(
                "  - Downgrade rules: {} | Evidence refs: {}\n",
                cert.downgrade_rules.len(),
                cert.evidence_packet_refs.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 companion certification export.
#[derive(Debug)]
pub enum M5CompanionCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CompanionCertificationViolation>),
}

impl fmt::Display for M5CompanionCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 companion certification export parse failed: {error}"
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
                    "m5 companion certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CompanionCertificationArtifactError {}

/// Validation failures emitted by [`M5CompanionCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CompanionCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required lane is missing from the certification.
    RequiredLaneMissing,
    /// A lane appears more than once.
    DuplicateLane,
    /// A lane certification row is incomplete.
    LaneRowIncomplete,
    /// A lane row's declared domain does not match its lane.
    LaneDomainMismatch,
    /// The frozen matrix baseline could not be read.
    MatrixBaselineUnavailable,
    /// A lane's recorded matrix baseline disagrees with the frozen matrix.
    MatrixBaselineMismatch,
    /// A lane's claimed qualification exceeds the frozen matrix baseline.
    ClaimExceedsMatrixBaseline,
    /// A lane does not cover every marketed profile.
    RequiredProfileMissing,
    /// A profile appears more than once in a lane's coverage.
    DuplicateProfile,
    /// A profile row's certification flag disagrees with its qualification.
    ProfileClaimFlagInconsistent,
    /// A profile row claims more than the lane's headline claim.
    ProfileQualificationExceedsClaim,
    /// A lane is certified on a profile where it is not marketable.
    LaneNotMarketableOnProfile,
    /// A certified row's locality disclosure is incomplete.
    LocalityDisclosureIncomplete,
    /// A certified row's continuity flag disagrees with the lane and profile.
    ContinuityFlagInconsistent,
    /// A row would strand user-owned local work.
    LocalCoreContinuityStranded,
    /// A stale or unknown freshness state is not labeled.
    FreshnessStateNotLabeled,
    /// A certified lane is missing required evidence packet refs.
    ClaimedLaneMissingEvidence,
    /// A lane has no downgrade rules.
    DowngradeRulesMissing,
    /// A lane's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A downgrade rule does not narrow below the matrix baseline.
    DowngradeRuleNotNarrowing,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CompanionCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::DuplicateLane => "duplicate_lane",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::LaneDomainMismatch => "lane_domain_mismatch",
            Self::MatrixBaselineUnavailable => "matrix_baseline_unavailable",
            Self::MatrixBaselineMismatch => "matrix_baseline_mismatch",
            Self::ClaimExceedsMatrixBaseline => "claim_exceeds_matrix_baseline",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::DuplicateProfile => "duplicate_profile",
            Self::ProfileClaimFlagInconsistent => "profile_claim_flag_inconsistent",
            Self::ProfileQualificationExceedsClaim => "profile_qualification_exceeds_claim",
            Self::LaneNotMarketableOnProfile => "lane_not_marketable_on_profile",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::ContinuityFlagInconsistent => "continuity_flag_inconsistent",
            Self::LocalCoreContinuityStranded => "local_core_continuity_stranded",
            Self::FreshnessStateNotLabeled => "freshness_state_not_labeled",
            Self::ClaimedLaneMissingEvidence => "claimed_lane_missing_evidence",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in M5 companion certification export.
///
/// This is the canonical reader: a companion panel, incident workspace,
/// diagnostics, support-export, or Help/About surface calls it to ingest the
/// certification packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`M5CompanionCertificationArtifactError`] when the checked-in support
/// export fails to parse or fails validation.
pub fn current_stable_m5_companion_certification_export(
) -> Result<M5CompanionCertificationPacket, M5CompanionCertificationArtifactError> {
    let packet: M5CompanionCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/support_export.json"
    )))
    .map_err(M5CompanionCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompanionCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Whether a lane is marketable on a profile.
///
/// Managed lanes need a managed plane, relay-bound companion lanes need a
/// reachable relay, and the local-first incident and offboarding lanes are
/// marketable on every profile.
pub fn lane_marketable_on_profile(lane: M5CompanionMatrixLane, profile: MarketedM5Profile) -> bool {
    use M5CompanionMatrixLane as Lane;
    match lane {
        Lane::CompanionNotification
        | Lane::CompanionReview
        | Lane::CompanionSessionFollow
        | Lane::CompanionLightEdit => profile.provides_companion_relay(),
        Lane::ManagedSync | Lane::ResidencyEncryption => profile.provides_managed_plane(),
        Lane::IncidentWorkspace | Lane::OffboardingContinuity => true,
    }
}

/// Whether a lane requires provider or admin continuity on a profile.
///
/// Managed lanes always require continuity; relay-bound companion lanes require
/// the relay whenever a row is actually certified; incident and offboarding lanes
/// are local-first and never require continuity for their certified row.
pub fn lane_requires_continuity_on_profile(
    lane: M5CompanionMatrixLane,
    profile: MarketedM5Profile,
) -> bool {
    use M5CompanionMatrixLane as Lane;
    match lane {
        Lane::ManagedSync | Lane::ResidencyEncryption => true,
        Lane::CompanionNotification
        | Lane::CompanionReview
        | Lane::CompanionSessionFollow
        | Lane::CompanionLightEdit => profile.provides_companion_relay(),
        Lane::IncidentWorkspace | Lane::OffboardingContinuity => false,
    }
}

/// Whether a qualification class is a publicly claimed lane.
fn is_claimed_qualification(class: M5CompanionQualificationClass) -> bool {
    matches!(
        class,
        M5CompanionQualificationClass::Stable
            | M5CompanionQualificationClass::Beta
            | M5CompanionQualificationClass::Preview
    )
}

/// Ordinal rank used to compare qualification severity.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5CompanionQualificationClass) -> u8 {
    match class {
        M5CompanionQualificationClass::Unavailable => 0,
        M5CompanionQualificationClass::Held => 1,
        M5CompanionQualificationClass::Experimental => 2,
        M5CompanionQualificationClass::Preview => 3,
        M5CompanionQualificationClass::Beta => 4,
        M5CompanionQualificationClass::Stable => 5,
    }
}

/// Reads the frozen matrix and returns each lane's baseline qualification.
///
/// Returns `None` only if the checked-in matrix export cannot be read or
/// validated, which surfaces as [`M5CompanionCertificationViolation::MatrixBaselineUnavailable`].
fn matrix_baseline_qualifications(
) -> Option<BTreeMap<M5CompanionMatrixLane, M5CompanionQualificationClass>> {
    let matrix = current_stable_m5_companion_matrix_export().ok()?;
    Some(
        matrix
            .lane_rows
            .iter()
            .map(|row| (row.lane, row.qualification))
            .collect(),
    )
}

fn validate_source_contracts(
    packet: &M5CompanionCertificationPacket,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMPANION_CERTIFICATION_SCHEMA_REF,
        M5_COMPANION_CERTIFICATION_DOC_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
        M5_COMPANION_MATRIX_ARTIFACT_REF,
        M5_COMPANION_MATRIX_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_INCIDENT_WORKSPACE_CONTRACT_REF,
        M5_MANAGED_SYNC_POLICY_REF,
        M5_REGION_RESIDENCY_REF,
        M5_OFFBOARDING_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CompanionCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_lanes_present(
    packet: &M5CompanionCertificationPacket,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    let mut seen: BTreeSet<M5CompanionMatrixLane> = BTreeSet::new();
    for cert in &packet.lane_certifications {
        if !seen.insert(cert.lane) {
            violations.push(M5CompanionCertificationViolation::DuplicateLane);
        }
    }
    for required in M5CompanionMatrixLane::ALL {
        if !seen.contains(&required) {
            violations.push(M5CompanionCertificationViolation::RequiredLaneMissing);
            return;
        }
    }
}

fn validate_lane_certification(
    cert: &CompanionLaneCertification,
    matrix_baseline: Option<&BTreeMap<M5CompanionMatrixLane, M5CompanionQualificationClass>>,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    if cert.scope_summary.trim().is_empty() {
        violations.push(M5CompanionCertificationViolation::LaneRowIncomplete);
    }
    if cert.domain != cert.lane.domain() {
        violations.push(M5CompanionCertificationViolation::LaneDomainMismatch);
    }

    if let Some(baseline) = matrix_baseline {
        if let Some(&matrix_qual) = baseline.get(&cert.lane) {
            if cert.matrix_baseline_qualification != matrix_qual {
                violations.push(M5CompanionCertificationViolation::MatrixBaselineMismatch);
            }
            if qualification_rank(cert.claimed_qualification) > qualification_rank(matrix_qual) {
                violations.push(M5CompanionCertificationViolation::ClaimExceedsMatrixBaseline);
            }
        }
    }

    validate_profile_coverage(cert, violations);
    validate_downgrade_rules(cert, violations);

    if cert.is_claimed() && cert.evidence_packet_refs.is_empty() {
        violations.push(M5CompanionCertificationViolation::ClaimedLaneMissingEvidence);
    }
}

fn validate_profile_coverage(
    cert: &CompanionLaneCertification,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    let mut seen: BTreeSet<MarketedM5Profile> = BTreeSet::new();
    let mut duplicate = false;
    for row in &cert.profile_coverage {
        if !seen.insert(row.profile) {
            duplicate = true;
        }
    }
    if duplicate {
        violations.push(M5CompanionCertificationViolation::DuplicateProfile);
    }
    for required in MarketedM5Profile::ALL {
        if !seen.contains(&required) {
            violations.push(M5CompanionCertificationViolation::RequiredProfileMissing);
            break;
        }
    }

    let claimed_rank = qualification_rank(cert.claimed_qualification);
    for row in &cert.profile_coverage {
        if row.certified_on_profile != is_claimed_qualification(row.profile_qualification) {
            violations.push(M5CompanionCertificationViolation::ProfileClaimFlagInconsistent);
        }
        if qualification_rank(row.profile_qualification) > claimed_rank {
            violations.push(M5CompanionCertificationViolation::ProfileQualificationExceedsClaim);
        }

        // The local core must never strand user-owned work on any profile, even
        // where the lane is not marketed.
        if !row.local_core_continuity_preserved {
            violations.push(M5CompanionCertificationViolation::LocalCoreContinuityStranded);
        }
        if row.freshness.requires_label() && !row.freshness_label_shown {
            violations.push(M5CompanionCertificationViolation::FreshnessStateNotLabeled);
        }

        if !row.certified_on_profile {
            continue;
        }

        if !lane_marketable_on_profile(cert.lane, row.profile) {
            violations.push(M5CompanionCertificationViolation::LaneNotMarketableOnProfile);
        }
        if row.locality_disclosure.stays_local.trim().is_empty()
            || row
                .locality_disclosure
                .requires_provider_or_admin_continuity
                .trim()
                .is_empty()
        {
            violations.push(M5CompanionCertificationViolation::LocalityDisclosureIncomplete);
        }
        if row.requires_provider_or_admin_continuity
            != lane_requires_continuity_on_profile(cert.lane, row.profile)
        {
            violations.push(M5CompanionCertificationViolation::ContinuityFlagInconsistent);
        }
    }
}

fn validate_downgrade_rules(
    cert: &CompanionLaneCertification,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    if cert.downgrade_rules.is_empty() {
        violations.push(M5CompanionCertificationViolation::DowngradeRulesMissing);
        return;
    }

    if !cert
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5CompanionDowngradeTrigger::ProofStale)
    {
        violations.push(M5CompanionCertificationViolation::DowngradeRuleMissingProofStale);
    }

    // Downgrade rules are defined relative to the frozen matrix baseline — the
    // strongest claim the lane can hold — so they stay valid even after live
    // narrowing has lowered the headline claim.
    let baseline_rank = qualification_rank(cert.matrix_baseline_qualification);
    for rule in &cert.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= baseline_rank {
            violations.push(M5CompanionCertificationViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_security_review(
    packet: &M5CompanionCertificationPacket,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.no_lane_greener_than_matrix,
        review.companions_stay_narrow,
        review.incident_packets_attributable,
        review.managed_sync_inspectable,
        review.residency_and_e2ee_claims_provable,
        review.offboarding_never_strands_local_work,
        review.locality_disclosed_per_profile,
        review.no_managed_claim_without_managed_plane,
        review.no_hidden_control_plane,
        review.no_credential_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5CompanionCertificationViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CompanionCertificationPacket,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_companion_shows_certification,
        projection.browser_companion_shows_certification,
        projection.mobile_companion_shows_certification,
        projection.incident_workspace_shows_certification,
        projection.cli_headless_shows_certification,
        projection.support_export_shows_certification,
        projection.diagnostics_shows_certification,
        projection.help_about_shows_certification,
        projection.unqualified_rows_labeled,
    ] {
        if !ok {
            violations.push(M5CompanionCertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CompanionCertificationPacket,
    violations: &mut Vec<M5CompanionCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CompanionCertificationViolation::ProofFreshnessIncomplete);
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

// ---------------------------------------------------------------------------
// First consumer: the canonical certification builder.
// ---------------------------------------------------------------------------

/// Canonical source contract refs every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_COMPANION_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_COMPANION_CERTIFICATION_DOC_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
        M5_COMPANION_MATRIX_ARTIFACT_REF.to_owned(),
        M5_COMPANION_MATRIX_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_MANAGED_SYNC_POLICY_REF.to_owned(),
        M5_REGION_RESIDENCY_REF.to_owned(),
        M5_OFFBOARDING_CONTRACT_REF.to_owned(),
    ]
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> M5CompanionCertificationSecurityReview {
    M5CompanionCertificationSecurityReview {
        no_lane_greener_than_matrix: true,
        companions_stay_narrow: true,
        incident_packets_attributable: true,
        managed_sync_inspectable: true,
        residency_and_e2ee_claims_provable: true,
        offboarding_never_strands_local_work: true,
        locality_disclosed_per_profile: true,
        no_managed_claim_without_managed_plane: true,
        no_hidden_control_plane: true,
        no_credential_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting truth.
pub fn canonical_consumer_projection() -> M5CompanionCertificationConsumerProjection {
    M5CompanionCertificationConsumerProjection {
        desktop_companion_shows_certification: true,
        browser_companion_shows_certification: true,
        mobile_companion_shows_certification: true,
        incident_workspace_shows_certification: true,
        cli_headless_shows_certification: true,
        support_export_shows_certification: true,
        diagnostics_shows_certification: true,
        help_about_shows_certification: true,
        unqualified_rows_labeled: true,
    }
}

/// Builds the canonical M5 companion certification from the frozen matrix.
///
/// This is the first consumer: it reads each lane's qualification from the frozen
/// matrix export, certifies it across every marketed profile, and mints the
/// packet the checked-in support export and Markdown summary are generated from,
/// so the artifact never drifts from the matrix or the typed lane definitions.
///
/// # Errors
///
/// Returns [`M5CompanionCertificationArtifactError`] when the frozen matrix
/// export cannot be read or validated.
pub fn canonical_m5_companion_certification(
    packet_id: String,
    certification_label: String,
    minted_at: String,
    proof_freshness: M5CompanionCertificationProofFreshness,
) -> Result<M5CompanionCertificationPacket, M5CompanionCertificationArtifactError> {
    let matrix = current_stable_m5_companion_matrix_export().map_err(|_| {
        M5CompanionCertificationArtifactError::Validation(vec![
            M5CompanionCertificationViolation::MatrixBaselineUnavailable,
        ])
    })?;
    let baseline: BTreeMap<M5CompanionMatrixLane, M5CompanionQualificationClass> = matrix
        .lane_rows
        .iter()
        .map(|row| (row.lane, row.qualification))
        .collect();
    let rollout: BTreeMap<M5CompanionMatrixLane, M5CompanionRolloutStage> = matrix
        .lane_rows
        .iter()
        .map(|row| (row.lane, row.rollout_stage))
        .collect();

    let lane_certifications = M5CompanionMatrixLane::ALL
        .into_iter()
        .map(|lane| {
            let claimed = baseline[&lane];
            let stage = rollout[&lane];
            lane_certification(lane, claimed, stage)
        })
        .collect::<Vec<_>>();

    let packet = M5CompanionCertificationPacket::new(M5CompanionCertificationPacketInput {
        packet_id,
        certification_label,
        lane_certifications,
        security_review: canonical_security_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        degraded_labels: Vec::new(),
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    });

    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompanionCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn lane_certification(
    lane: M5CompanionMatrixLane,
    claimed: M5CompanionQualificationClass,
    stage: M5CompanionRolloutStage,
) -> CompanionLaneCertification {
    let profile_coverage = MarketedM5Profile::ALL
        .into_iter()
        .map(|profile| profile_row(lane, profile, claimed, stage))
        .collect::<Vec<_>>();

    CompanionLaneCertification {
        lane,
        domain: lane.domain(),
        claimed_qualification: claimed,
        matrix_baseline_qualification: claimed,
        scope_summary: lane_scope_summary(lane).to_owned(),
        profile_coverage,
        downgrade_rules: lane_downgrade_rules(lane, claimed),
        evidence_packet_refs: lane_evidence_refs(lane),
    }
}

fn profile_row(
    lane: M5CompanionMatrixLane,
    profile: MarketedM5Profile,
    claimed: M5CompanionQualificationClass,
    stage: M5CompanionRolloutStage,
) -> ProfileCertificationRow {
    let marketable = lane_marketable_on_profile(lane, profile);
    if marketable {
        ProfileCertificationRow {
            profile,
            certified_on_profile: true,
            profile_qualification: claimed,
            rollout_stage: stage,
            locality_disclosure: certified_locality(lane, profile),
            requires_provider_or_admin_continuity: lane_requires_continuity_on_profile(
                lane, profile,
            ),
            local_core_continuity_preserved: true,
            freshness: CertificationFreshness::Fresh,
            freshness_label_shown: false,
        }
    } else {
        ProfileCertificationRow {
            profile,
            certified_on_profile: false,
            profile_qualification: M5CompanionQualificationClass::Unavailable,
            rollout_stage: M5CompanionRolloutStage::Withheld,
            locality_disclosure: not_marketed_locality(lane, profile),
            requires_provider_or_admin_continuity: false,
            local_core_continuity_preserved: true,
            freshness: CertificationFreshness::Fresh,
            freshness_label_shown: false,
        }
    }
}

fn certified_locality(
    lane: M5CompanionMatrixLane,
    profile: MarketedM5Profile,
) -> M5CompanionLocalityDisclosure {
    use M5CompanionMatrixLane as Lane;
    let continuity = match lane {
        Lane::ManagedSync => {
            "Server-side sync and conflict relay require the sync provider and, for managed tenants, admin continuity."
        }
        Lane::ResidencyEncryption => {
            "Customer-managed-key and region-residency guarantees require the managed key authority and admin continuity, and are claimed only when verifiable."
        }
        Lane::CompanionNotification
        | Lane::CompanionReview
        | Lane::CompanionSessionFollow
        | Lane::CompanionLightEdit => {
            "Paired companion delivery and any relayed action require the companion relay and an active host; the local core never depends on them."
        }
        Lane::IncidentWorkspace => {
            "Escalation to a managed support channel requires admin continuity; local triage never depends on it."
        }
        Lane::OffboardingContinuity => {
            "Revoking managed or cloud artifacts requires admin continuity; local-core continuity is never gated on it."
        }
    };
    let stays_local = match lane {
        Lane::ManagedSync => {
            "The local core is the source of truth; every synced record stays inspectable and reconcilable offline."
        }
        Lane::ResidencyEncryption => {
            "Local-only artifacts never leave the device and carry no residency dependency."
        }
        Lane::CompanionNotification
        | Lane::CompanionReview
        | Lane::CompanionSessionFollow
        | Lane::CompanionLightEdit => {
            "Source events, review records, session state, and edits are authored and stored by the local core and stay usable offline."
        }
        Lane::IncidentWorkspace => {
            "Incident evidence, missing-span facts, and runbook records are local-first and inspectable offline."
        }
        Lane::OffboardingContinuity => {
            "User-owned local work and history remain on the device and fully usable after offboarding."
        }
    };
    let staged = format!(
        "Certified on the {} profile; cross-profile rollout is staged per cohort and capability.",
        profile.as_str()
    );
    M5CompanionLocalityDisclosure {
        stays_local: stays_local.to_owned(),
        staged,
        requires_provider_or_admin_continuity: continuity.to_owned(),
    }
}

fn not_marketed_locality(
    lane: M5CompanionMatrixLane,
    profile: MarketedM5Profile,
) -> M5CompanionLocalityDisclosure {
    use M5CompanionMatrixLane as Lane;
    let reason = match lane {
        Lane::ManagedSync | Lane::ResidencyEncryption => {
            "This profile provides no managed control plane, so the managed lane is not marketed here."
        }
        _ => "This profile cannot reach a companion relay, so the relay-bound companion lane is not marketed here.",
    };
    let stays_local = match lane {
        Lane::OffboardingContinuity | Lane::IncidentWorkspace => {
            "The local-first lane stays fully usable on this profile."
        }
        _ => "User-owned local work stays on the device and fully usable on this profile.",
    };
    M5CompanionLocalityDisclosure {
        stays_local: stays_local.to_owned(),
        staged: format!("Not marketed on the {} profile.", profile.as_str()),
        requires_provider_or_admin_continuity: reason.to_owned(),
    }
}

fn lane_scope_summary(lane: M5CompanionMatrixLane) -> &'static str {
    use M5CompanionMatrixLane as Lane;
    match lane {
        Lane::CompanionNotification => {
            "Browser/mobile companion notifications, read-only with no editor authority, certified across the relay-reachable profiles"
        }
        Lane::CompanionReview => {
            "Companion review/approve follow-up without authoring edits, certified across the relay-reachable profiles"
        }
        Lane::CompanionSessionFollow => {
            "Companion session-follow and handoff eligibility, narrowed to read plus handoff, certified across the relay-reachable profiles"
        }
        Lane::CompanionLightEdit => {
            "Companion bounded light-edit relayed to the host for preview/approval, certified across the relay-reachable profiles"
        }
        Lane::IncidentWorkspace => {
            "Attributable, local-first incident workspaces certified on every marketed profile including air-gapped"
        }
        Lane::ManagedSync => {
            "Inspectable managed sync with conflict review, certified only on profiles that provide a managed plane"
        }
        Lane::ResidencyEncryption => {
            "Provable customer-managed-key and end-to-end-encryption residency posture, certified only on profiles that provide a managed plane"
        }
        Lane::OffboardingContinuity => {
            "Offboarding that never strands user-owned local work, certified on every marketed profile including air-gapped"
        }
    }
}

fn lane_evidence_refs(lane: M5CompanionMatrixLane) -> Vec<String> {
    use M5CompanionMatrixLane as Lane;
    let token = match lane {
        Lane::CompanionNotification => "evidence:companion-surface-conformance:m5",
        Lane::CompanionReview => "evidence:companion-review-followup:m5",
        Lane::CompanionSessionFollow => "evidence:companion-handoff-eligibility:m5",
        Lane::CompanionLightEdit => "evidence:companion-light-edit-bounds:m5",
        Lane::IncidentWorkspace => "evidence:incident-workspace-attribution:m5",
        Lane::ManagedSync => "evidence:managed-sync-inspectability:m5",
        Lane::ResidencyEncryption => "evidence:residency-and-e2ee-proof:m5",
        Lane::OffboardingContinuity => "evidence:offboarding-local-continuity:m5",
    };
    vec![token.to_owned()]
}

fn lane_downgrade_rules(
    lane: M5CompanionMatrixLane,
    claimed: M5CompanionQualificationClass,
) -> Vec<CompanionLaneDowngradeRule> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionMatrixLane as Lane;

    // The first narrowing step below the headline claim, and one further step,
    // both stay strictly below the matrix baseline.
    let one_step = claimed.narrowed_one_step();
    let two_step = one_step.narrowed_one_step();

    let mut rules = vec![CompanionLaneDowngradeRule {
        trigger: Trigger::ProofStale,
        narrowed_to: one_step,
        auto_enforced: true,
        rationale: "Stale proof narrows the certified claim one step rather than hiding the lane"
            .to_owned(),
    }];

    match lane {
        Lane::ManagedSync => {
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::SyncInspectionUnavailable,
                narrowed_to: two_step,
                auto_enforced: true,
                rationale: "Sync that cannot be inspected or reconciled narrows the claim"
                    .to_owned(),
            });
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::AdminContinuityRequired,
                narrowed_to: two_step,
                auto_enforced: true,
                rationale: "Unavailable admin continuity narrows the managed claim".to_owned(),
            });
        }
        Lane::ResidencyEncryption => {
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::ResidencyOrEncryptionUnverified,
                narrowed_to: two_step,
                auto_enforced: true,
                rationale: "An unverifiable residency or E2EE claim narrows the lane".to_owned(),
            });
        }
        Lane::IncidentWorkspace => {
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::IncidentAttributionMissing,
                narrowed_to: two_step,
                auto_enforced: true,
                rationale: "An incident packet that lost attribution narrows the lane".to_owned(),
            });
        }
        Lane::OffboardingContinuity => {
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::OffboardingStrandsLocalWork,
                narrowed_to: M5CompanionQualificationClass::Held,
                auto_enforced: true,
                rationale: "Offboarding that would strand local work holds the lane".to_owned(),
            });
        }
        Lane::CompanionNotification
        | Lane::CompanionReview
        | Lane::CompanionSessionFollow
        | Lane::CompanionLightEdit => {
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::ProviderUnavailable,
                narrowed_to: two_step,
                auto_enforced: true,
                rationale: "An unavailable companion relay narrows the lane".to_owned(),
            });
            rules.push(CompanionLaneDowngradeRule {
                trigger: Trigger::CompanionScopeExpansionUnqualified,
                narrowed_to: two_step,
                auto_enforced: true,
                rationale: "Companion scope expanding beyond its narrow boundary narrows the lane"
                    .to_owned(),
            });
        }
    }

    rules
}
