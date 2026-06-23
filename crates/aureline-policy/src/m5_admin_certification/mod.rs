//! M5 admin-plane *certification*: the qualification capstone that binds the
//! local admin plane's effective-policy, decision-history, endpoint-posture,
//! retention/deletion, offboarding, and procurement/admin-packet truth into M5
//! promotion, and auto-narrows a managed claim the moment any of that proof goes
//! stale or starts failing.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — the surface families, the shared state vocabulary, and the proof
//! packets that keep them current — [`m5_admin_render`](crate::m5_admin_render)
//! *renders the current admin state*, and
//! [`m5_rollout_simulation`](crate::m5_rollout_simulation) *simulates the next
//! state*, this lane *certifies the present state*. It does not re-derive any
//! admin truth: each [`CertifiedFamilyClass`] cites the upstream proof lane that
//! already produces it (its boundary schema, worked fixture, and freeze gate) and
//! reads that lane's freshness and pass/fail result into a single per-profile
//! qualification row. The capstone's only job is the honest verdict: is this
//! family, on this profile, *proven current* right now, or not.
//!
//! Each packet *binds back to the matrix*. Every family row names the
//! [`AdminSurfaceClass`] surfaces it certifies, and each must be a surface the
//! frozen matrix declares — present, locally explainable, and typed rather than
//! portal-only ([`CertificationInvariant`] `admin_cert.bound_surfaces_in_matrix`).
//! Every qualification headline state it emits is one the matrix's unified state
//! vocabulary defines (`admin_cert.claim_states_in_vocabulary`). An edit that
//! certifies a surface the matrix does not define, or emits a state outside the
//! frozen vocabulary, flips an invariant and fails the freeze gate.
//!
//! The honesty rules the spec requires are enforced, not just described:
//!
//! - **No green on stale or failing proof.** A family row reads *qualified* only
//!   when its upstream proof is bound, fresh, and passing; the moment the proof is
//!   stale, failing, or unproven the row narrows off qualified and names exactly
//!   why (`admin_cert.no_green_on_stale_or_failing`). A qualified row must cite a
//!   real, export-safe proof lane, so a claim can never go green because the
//!   mechanics exist *somewhere in the stack* while the user-facing proof is
//!   absent (`admin_cert.qualified_requires_proven_lane`).
//! - **Profile claims auto-narrow.** A profile's managed claim reads confirmed
//!   only when every claimed family qualifies; otherwise it downgrades off
//!   confirmed and names which families and reasons narrowed it
//!   (`admin_cert.profile_claim_auto_narrows`). The reported proof freshness is
//!   the stalest claimed family, so one stale family cannot hide behind fresher
//!   siblings (`admin_cert.proof_freshness_is_worst_case`).
//! - **Release evidence carries explicit rows.** The bundle publishes a
//!   release-evidence row for each named dimension — policy source/verification,
//!   audit history, delete/export honesty, offboarding continuity, and
//!   procurement/support/admin-packet fidelity — and each reflects the *worst*
//!   qualification across all profiles, never a rosier summary
//!   (`admin_cert.release_evidence_rows_present`,
//!   `admin_cert.release_evidence_reflects_worst`).
//!
//! There is exactly one typed packet per claimed managed-bearing profile, consumed
//! identically by the shell admin center, CLI/headless inspect, Help/About,
//! support export, commercial/procurement, and release evidence, so the
//! qualification state is the same bytes on every surface by construction
//! (`admin_cert.consumer_parity`) — About/help/support/commercial read this packet
//! instead of restating admin-plane quality claims by hand.
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, or absolute paths — only opaque object refs, stable tokens, rendered
//! metadata-safe value summaries, and short reviewable sentences — so it is safe
//! to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminStateClass,
    AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};
use crate::m5_admin_render::{EvidenceAgeClass, OwnerEscalationRoleClass};

#[cfg(test)]
mod tests;

/// Schema version for the admin-certification bundle.
pub const M5_ADMIN_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the admin-certification bundle.
pub const M5_ADMIN_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/admin/m5-admin-certification.schema.json";

/// Stable record-kind tag for the admin-certification bundle.
pub const M5_ADMIN_CERTIFICATION_RECORD_KIND: &str = "m5_admin_certification_bundle";

/// Stable id for the canonical admin-certification bundle.
pub const M5_ADMIN_CERTIFICATION_BUNDLE_ID: &str = "m5-admin-certification:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ADMIN_CERTIFICATION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this certification layer binds back to.
pub const M5_ADMIN_CERTIFICATION_MATRIX_REF: &str =
    "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the certification bundle current.
pub const M5_ADMIN_CERTIFICATION_FREEZE_GATE_REF: &str =
    "crates/aureline-policy/tests/m5_admin_certification.rs";

// ---------------------------------------------------------------------------
// Token enums.
// ---------------------------------------------------------------------------

/// The admin-plane families this capstone certifies — the governed local admin
/// surfaces whose proof M5 promotion depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedFamilyClass {
    /// Effective policy, policy diff, and locked-state explanation: why a control
    /// is locked, what source is active, and what a change moves.
    PolicyExplainability,
    /// The decision-history timeline / audit-event explorer.
    DecisionHistory,
    /// The endpoint-posture card: enrolled device/install posture and freshness.
    EndpointPosture,
    /// The retention/deletion matrix: data classes, hold/delete/export outcomes,
    /// and destruction receipts.
    RetentionDelete,
    /// The offboarding wizard: ordered local-safe export, delete, and continuity
    /// steps for seat loss, deprovision, or org switch.
    Offboarding,
    /// The procurement / verification packet and admin-handoff bundle.
    ProcurementAdminPacket,
}

impl CertifiedFamilyClass {
    /// All certified families, in bundle order.
    pub const ALL: [Self; 6] = [
        Self::PolicyExplainability,
        Self::DecisionHistory,
        Self::EndpointPosture,
        Self::RetentionDelete,
        Self::Offboarding,
        Self::ProcurementAdminPacket,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyExplainability => "policy_explainability",
            Self::DecisionHistory => "decision_history",
            Self::EndpointPosture => "endpoint_posture",
            Self::RetentionDelete => "retention_delete",
            Self::Offboarding => "offboarding",
            Self::ProcurementAdminPacket => "procurement_admin_packet",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PolicyExplainability => "Policy explainability",
            Self::DecisionHistory => "Decision history / audit explorer",
            Self::EndpointPosture => "Endpoint posture",
            Self::RetentionDelete => "Retention / deletion truth",
            Self::Offboarding => "Offboarding continuity",
            Self::ProcurementAdminPacket => "Procurement / admin packet",
        }
    }

    /// The matrix surfaces this family certifies.
    pub const fn bound_surfaces(self) -> &'static [AdminSurfaceClass] {
        match self {
            Self::PolicyExplainability => &[
                AdminSurfaceClass::EffectivePolicyView,
                AdminSurfaceClass::PolicyDiff,
                AdminSurfaceClass::LockedStateExplanation,
            ],
            Self::DecisionHistory => &[AdminSurfaceClass::DecisionHistoryTimeline],
            Self::EndpointPosture => &[AdminSurfaceClass::EndpointPostureCard],
            Self::RetentionDelete => &[AdminSurfaceClass::RetentionDeletionMatrix],
            Self::Offboarding => &[AdminSurfaceClass::OffboardingWizard],
            Self::ProcurementAdminPacket => &[AdminSurfaceClass::ProcurementVerificationPacket],
        }
    }

    /// The upstream proof lane this family reads its freshness and pass/fail result
    /// from. Stable promotion fails when a claimed family cites no lane.
    pub fn proof_lane(self) -> ProofLaneRef {
        match self {
            Self::PolicyExplainability => ProofLaneRef::new(
                "proof_lane.m5_admin_render.effective_policy",
                "m5_admin_render_bundle",
                "schemas/admin/m5-admin-render.schema.json",
                "fixtures/admin/m5-admin-render/canonical_render.json",
                "crates/aureline-policy/tests/m5_admin_render.rs",
                "docs/admin/m5-admin-render.md",
                "crates/aureline-policy/src/m5_admin_render/",
            ),
            Self::DecisionHistory => ProofLaneRef::new(
                "proof_lane.m5_decision_history",
                "m5_decision_history_bundle",
                "schemas/admin/m5-decision-history.schema.json",
                "fixtures/admin/m5-decision-history/canonical_history.json",
                "crates/aureline-policy/tests/m5_decision_history.rs",
                "docs/admin/m5-decision-history.md",
                "crates/aureline-policy/src/m5_decision_history/",
            ),
            Self::EndpointPosture => ProofLaneRef::new(
                "proof_lane.m5_admin_render.endpoint_posture",
                "m5_admin_render_bundle",
                "schemas/admin/m5-admin-render.schema.json",
                "fixtures/admin/m5-admin-render/canonical_render.json",
                "crates/aureline-policy/tests/m5_admin_render.rs",
                "docs/admin/m5-admin-render.md",
                "crates/aureline-policy/src/m5_admin_render/",
            ),
            Self::RetentionDelete => ProofLaneRef::new(
                "proof_lane.m5_retention_deletion",
                "m5_retention_deletion_bundle",
                "schemas/admin/m5-retention-deletion.schema.json",
                "fixtures/admin/m5-retention-deletion/canonical_retention.json",
                "crates/aureline-policy/tests/m5_retention_deletion.rs",
                "docs/admin/m5-retention-deletion.md",
                "crates/aureline-policy/src/m5_retention_deletion/",
            ),
            Self::Offboarding => ProofLaneRef::new(
                "proof_lane.m5_offboarding",
                "m5_offboarding_bundle",
                "schemas/admin/m5-offboarding.schema.json",
                "fixtures/admin/m5-offboarding/canonical_offboarding.json",
                "crates/aureline-policy/tests/m5_offboarding.rs",
                "docs/admin/m5-offboarding.md",
                "crates/aureline-policy/src/m5_offboarding/",
            ),
            Self::ProcurementAdminPacket => ProofLaneRef::new(
                "proof_lane.m5_procurement",
                "m5_procurement_bundle",
                "schemas/admin/m5-procurement.schema.json",
                "fixtures/admin/m5-procurement/canonical_procurement.json",
                "crates/aureline-policy/tests/m5_procurement.rs",
                "docs/admin/m5-procurement.md",
                "crates/aureline-policy/src/m5_procurement/",
            ),
        }
    }

    /// Who owns this family's qualification.
    const fn owner(self) -> OwnerEscalationRoleClass {
        match self {
            Self::PolicyExplainability => OwnerEscalationRoleClass::OrgAdmin,
            Self::DecisionHistory => OwnerEscalationRoleClass::OrgAdmin,
            Self::EndpointPosture => OwnerEscalationRoleClass::SecurityOwner,
            Self::RetentionDelete => OwnerEscalationRoleClass::ComplianceOwner,
            Self::Offboarding => OwnerEscalationRoleClass::OrgAdmin,
            Self::ProcurementAdminPacket => OwnerEscalationRoleClass::ComplianceOwner,
        }
    }

    /// Who this family's qualification escalates to.
    const fn escalation_owner(self) -> OwnerEscalationRoleClass {
        match self {
            Self::PolicyExplainability => OwnerEscalationRoleClass::SecurityOwner,
            Self::DecisionHistory => OwnerEscalationRoleClass::ComplianceOwner,
            Self::EndpointPosture => OwnerEscalationRoleClass::OrgAdmin,
            Self::RetentionDelete => OwnerEscalationRoleClass::SecurityOwner,
            Self::Offboarding => OwnerEscalationRoleClass::WorkspaceOwner,
            Self::ProcurementAdminPacket => OwnerEscalationRoleClass::OrgAdmin,
        }
    }
}

/// The per-row qualification verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationClass {
    /// Proof is bound, fresh, and passing: the family qualifies.
    Qualified,
    /// Proof is bound and passing but past its soft-refresh window: narrowed off
    /// qualified until it refreshes.
    NarrowedStaleEvidence,
    /// Proof is bound but its freeze gate / invariants are failing: narrowed off
    /// qualified until the proof passes again.
    NarrowedFailingProof,
    /// No upstream proof lane is bound, so the family cannot be certified at all.
    NarrowedUnproven,
}

impl QualificationClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::NarrowedStaleEvidence => "narrowed_stale_evidence",
            Self::NarrowedFailingProof => "narrowed_failing_proof",
            Self::NarrowedUnproven => "narrowed_unproven",
        }
    }

    /// Whether the family qualifies (proof current and passing).
    pub const fn is_qualified(self) -> bool {
        matches!(self, Self::Qualified)
    }

    /// Severity rank: higher is more narrowed. Used to compute the worst
    /// qualification across a set of rows.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Qualified => 0,
            Self::NarrowedStaleEvidence => 1,
            Self::NarrowedFailingProof => 2,
            Self::NarrowedUnproven => 3,
        }
    }

    /// The admin-plane headline state this verdict maps to. Qualified reads
    /// active/enforced; a stale or failing verdict reads the no-silent-green
    /// downgrade; an unproven verdict requires review.
    pub const fn claim_state(self) -> AdminStateClass {
        match self {
            Self::Qualified => AdminStateClass::ActiveEnforced,
            Self::NarrowedStaleEvidence | Self::NarrowedFailingProof => {
                AdminStateClass::UnconfirmedStale
            }
            Self::NarrowedUnproven => AdminStateClass::UnknownRequiresReview,
        }
    }
}

/// Why a family row — and through it a profile's managed claim — narrowed off
/// qualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimNarrowReasonClass {
    /// The family's local proof evidence is past its soft-refresh window.
    FamilyEvidenceStale,
    /// The family's proof is mirror-backed and the offline mirror's last sync is
    /// past its soft-refresh window.
    MirrorEvidenceStale,
    /// The family's proof packet / freeze gate is failing.
    FamilyProofFailing,
    /// The family cites no upstream proof lane, so it cannot be certified.
    FamilyProofMissing,
}

impl ClaimNarrowReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FamilyEvidenceStale => "family_evidence_stale",
            Self::MirrorEvidenceStale => "mirror_evidence_stale",
            Self::FamilyProofFailing => "family_proof_failing",
            Self::FamilyProofMissing => "family_proof_missing",
        }
    }
}

/// The named release-evidence dimensions the bundle must publish explicit rows
/// for, so release automation reads one source of admin-plane qualification truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseEvidenceDimensionClass {
    /// Policy source and verification posture.
    PolicySourceVerification,
    /// Decision-history / audit-event truth.
    AuditHistory,
    /// Delete and export honesty.
    DeleteExportHonesty,
    /// Offboarding continuity.
    OffboardingContinuity,
    /// Procurement, support, and admin-packet fidelity.
    ProcurementSupportAdminPacket,
}

impl ReleaseEvidenceDimensionClass {
    /// All release-evidence dimensions, in bundle order.
    pub const ALL: [Self; 5] = [
        Self::PolicySourceVerification,
        Self::AuditHistory,
        Self::DeleteExportHonesty,
        Self::OffboardingContinuity,
        Self::ProcurementSupportAdminPacket,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicySourceVerification => "policy_source_verification",
            Self::AuditHistory => "audit_history",
            Self::DeleteExportHonesty => "delete_export_honesty",
            Self::OffboardingContinuity => "offboarding_continuity",
            Self::ProcurementSupportAdminPacket => "procurement_support_admin_packet",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PolicySourceVerification => "Policy source & verification",
            Self::AuditHistory => "Audit history",
            Self::DeleteExportHonesty => "Delete / export honesty",
            Self::OffboardingContinuity => "Offboarding continuity",
            Self::ProcurementSupportAdminPacket => "Procurement / support / admin packet",
        }
    }

    /// The certified families this dimension summarizes.
    pub const fn families(self) -> &'static [CertifiedFamilyClass] {
        match self {
            Self::PolicySourceVerification => &[
                CertifiedFamilyClass::PolicyExplainability,
                CertifiedFamilyClass::EndpointPosture,
            ],
            Self::AuditHistory => &[CertifiedFamilyClass::DecisionHistory],
            Self::DeleteExportHonesty => &[CertifiedFamilyClass::RetentionDelete],
            Self::OffboardingContinuity => &[CertifiedFamilyClass::Offboarding],
            Self::ProcurementSupportAdminPacket => &[CertifiedFamilyClass::ProcurementAdminPacket],
        }
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// A reference to the upstream proof lane a family is certified against. Carries
/// only repo-relative object refs, never a URL, host, or absolute path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofLaneRef {
    /// Stable, namespaced lane id.
    pub lane_id: String,
    /// The upstream bundle's record kind.
    pub record_kind: String,
    /// The upstream boundary schema.
    pub schema_ref: String,
    /// The upstream worked fixture.
    pub fixture_ref: String,
    /// The upstream freeze gate that keeps the lane current.
    pub freeze_gate_ref: String,
    /// The upstream contract document.
    pub doc_ref: String,
    /// The crate module that produces the upstream truth.
    pub produced_by_ref: String,
}

impl ProofLaneRef {
    fn new(
        lane_id: &str,
        record_kind: &str,
        schema_ref: &str,
        fixture_ref: &str,
        freeze_gate_ref: &str,
        doc_ref: &str,
        produced_by_ref: &str,
    ) -> Self {
        Self {
            lane_id: lane_id.to_owned(),
            record_kind: record_kind.to_owned(),
            schema_ref: schema_ref.to_owned(),
            fixture_ref: fixture_ref.to_owned(),
            freeze_gate_ref: freeze_gate_ref.to_owned(),
            doc_ref: doc_ref.to_owned(),
            produced_by_ref: produced_by_ref.to_owned(),
        }
    }

    /// Whether every ref this lane carries is a repo-relative, export-safe object
    /// ref. A row may only qualify against a lane that passes.
    pub fn is_proven(&self) -> bool {
        !self.lane_id.is_empty()
            && [
                self.schema_ref.as_str(),
                self.fixture_ref.as_str(),
                self.freeze_gate_ref.as_str(),
                self.doc_ref.as_str(),
                self.produced_by_ref.as_str(),
            ]
            .into_iter()
            .all(is_export_safe_ref)
    }
}

/// One certified family's qualification row on one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyQualificationRow {
    /// The certified family.
    pub family: CertifiedFamilyClass,
    /// Stable, namespaced row id (profile + family).
    pub row_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The matrix surfaces this family certifies.
    pub bound_surfaces: Vec<AdminSurfaceClass>,
    /// The upstream proof lane this row reads.
    pub proof_lane: ProofLaneRef,
    /// Whether this profile claims this family at all.
    pub claimed: bool,
    /// Whether the family's proof is mirror-backed (offline-mirror freshness
    /// counts).
    pub mirror_backed: bool,
    /// The freshness of the upstream proof evidence.
    pub proof_freshness: EvidenceAgeClass,
    /// Whether the upstream proof packet / freeze gate is failing.
    pub proof_failing: bool,
    /// The qualification verdict computed from the proof state.
    pub qualification: QualificationClass,
    /// The admin-plane headline state this row reads.
    pub claim_state: AdminStateClass,
    /// Why the row narrowed, absent when qualified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrow_reason: Option<ClaimNarrowReasonClass>,
    /// Who owns this family's qualification.
    pub owner: OwnerEscalationRoleClass,
    /// Who the qualification escalates to.
    pub escalation_owner: OwnerEscalationRoleClass,
    /// Whether the family is locally explainable (never portal-only).
    pub locally_explainable: bool,
    /// The redaction rule applied to this row on export.
    pub redaction: AdminRedactionClass,
    /// One reviewable sentence stating the qualification evidence.
    pub evidence_note: String,
}

impl FamilyQualificationRow {
    /// Whether this row qualifies (proof current and passing).
    pub fn is_qualified(&self) -> bool {
        self.qualification.is_qualified()
    }
}

/// The certification packet for one profile: a qualification row per certified
/// family and the auto-narrowed managed claim aggregated from them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPacket {
    /// The admin path / profile this packet certifies.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The per-family qualification rows.
    pub families: Vec<FamilyQualificationRow>,
    /// The freshness of the stalest claimed family.
    pub proof_freshness: EvidenceAgeClass,
    /// The managed-claim state after auto-narrowing: confirmed when every claimed
    /// family qualifies, downgraded otherwise.
    pub claim_state: AdminStateClass,
    /// Why the claim auto-narrowed, empty when confirmed.
    pub narrow_reasons: Vec<ClaimNarrowReasonClass>,
    /// One reviewable sentence stating the claim posture.
    pub claim_note: String,
}

impl CertificationPacket {
    /// Whether the managed claim is confirmed (no auto-narrowing).
    pub fn claim_confirmed(&self) -> bool {
        self.claim_state == AdminStateClass::ActiveEnforced && self.narrow_reasons.is_empty()
    }

    /// Resolves a family row within this packet.
    pub fn row(&self, family: CertifiedFamilyClass) -> Option<&FamilyQualificationRow> {
        self.families.iter().find(|r| r.family == family)
    }

    /// The claimed family rows.
    pub fn claimed_rows(&self) -> impl Iterator<Item = &FamilyQualificationRow> {
        self.families.iter().filter(|r| r.claimed)
    }
}

/// One release-evidence row: the worst qualification across all profiles for a
/// named dimension, so release automation reads the admin-plane truth directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseEvidenceRow {
    /// The release-evidence dimension.
    pub dimension: ReleaseEvidenceDimensionClass,
    /// Stable, namespaced row id.
    pub row_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable statement of what this dimension proves.
    pub statement: String,
    /// The certified families this dimension summarizes.
    pub families: Vec<CertifiedFamilyClass>,
    /// The worst qualification across all profiles for the bound families.
    pub worst_qualification: QualificationClass,
    /// The admin-plane headline state this row reads.
    pub claim_state: AdminStateClass,
    /// One reviewable sentence stating the release-evidence posture.
    pub note: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen admin-certification bundle: one packet per claimed managed-bearing
/// profile, plus the release-evidence rows release automation consumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminCertificationBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_admin_certification_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The matrix this certification layer binds back to.
    pub matrix_ref: String,
    /// The matrix id this certification layer binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps this bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The per-profile certification packets.
    pub profiles: Vec<CertificationPacket>,
    /// The release-evidence rows.
    pub release_evidence: Vec<ReleaseEvidenceRow>,
    /// The computed invariants.
    pub invariants: Vec<CertificationInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminCertificationValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for AdminCertificationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "admin-certification bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for AdminCertificationValidationError {}

/// The profiles the certification bundle covers, in bundle order.
pub const CERTIFIED_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

/// The consumers every packet must serve identically for cross-surface parity.
/// The commercial/procurement and Help/About consumers are required so those
/// surfaces read the qualification state instead of restating it.
const PARITY_CONSUMERS: [AdminConsumerClass; 6] = [
    AdminConsumerClass::ShellAdminCenter,
    AdminConsumerClass::CliHeadless,
    AdminConsumerClass::HelpAbout,
    AdminConsumerClass::SupportExport,
    AdminConsumerClass::CommercialProcurement,
    AdminConsumerClass::ReleaseEvidence,
];

impl AdminCertificationBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&CertificationPacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Returns the release-evidence row for a dimension, if present.
    pub fn release_evidence_row(
        &self,
        dimension: ReleaseEvidenceDimensionClass,
    ) -> Option<&ReleaseEvidenceRow> {
        self.release_evidence
            .iter()
            .find(|r| r.dimension == dimension)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Every family row across every profile, in bundle order.
    pub fn rows(&self) -> impl Iterator<Item = &FamilyQualificationRow> {
        self.profiles.iter().flat_map(|p| p.families.iter())
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque token.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The top-level repo-relative file refs carried by the bundle.
    fn file_refs(&self) -> [&str; 3] {
        [
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ]
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            for r in &p.families {
                ids.push(r.row_id.as_str());
                ids.push(r.proof_lane.lane_id.as_str());
            }
        }
        for r in &self.release_evidence {
            ids.push(r.row_id.as_str());
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), AdminCertificationValidationError> {
        let fail = |reason: String| Err(AdminCertificationValidationError { reason });

        if self.record_kind != M5_ADMIN_CERTIFICATION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ADMIN_CERTIFICATION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in CERTIFIED_PROFILES {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        for packet in &self.profiles {
            validate_packet(packet)
                .map_err(|reason| AdminCertificationValidationError { reason })?;
        }

        for dimension in ReleaseEvidenceDimensionClass::ALL {
            if self
                .release_evidence
                .iter()
                .filter(|r| r.dimension == dimension)
                .count()
                != 1
            {
                return fail(format!(
                    "release-evidence dimension {} not present exactly once",
                    dimension.as_str()
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

/// Whether a stable token id is safe to export: non-empty and carries no URL
/// scheme or absolute path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Per-packet structural floor checks, shared by [`AdminCertificationBundle::validate`].
fn validate_packet(packet: &CertificationPacket) -> Result<(), String> {
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!(
            "profile id mismatch for {}",
            packet.profile.as_str()
        ));
    }
    for family in CertifiedFamilyClass::ALL {
        if packet
            .families
            .iter()
            .filter(|r| r.family == family)
            .count()
            != 1
        {
            return Err(format!(
                "{}: family {} not present exactly once",
                packet.profile.as_str(),
                family.as_str()
            ));
        }
    }
    if !all_unique(packet.families.iter().map(|r| r.row_id.as_str())) {
        return Err(format!("{}: row ids not unique", packet.profile.as_str()));
    }
    if packet.claimed_rows().next().is_none() {
        return Err(format!("{}: no claimed families", packet.profile.as_str()));
    }
    for row in &packet.families {
        if row.bound_surfaces.is_empty() {
            return Err(format!(
                "{}: family {} binds no surface",
                packet.profile.as_str(),
                row.family.as_str()
            ));
        }
        if row.row_id != row_id(packet.profile, row.family) {
            return Err(format!(
                "{}: family {} has an unexpected row id",
                packet.profile.as_str(),
                row.family.as_str()
            ));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// The stable, namespaced row id for a profile + family.
fn row_id(profile: AdminPathClass, family: CertifiedFamilyClass) -> String {
    format!("admin_cert.{}.{}", profile.as_str(), family.as_str())
}

/// Builds the canonical admin-certification bundle.
///
/// Deterministic: the same bytes every call. The per-row qualification, the
/// per-profile claim state and narrow reasons, the release-evidence worst-case
/// qualifications, and the invariant `holds` flags are all computed from the proof
/// states, so an inconsistent edit flips an invariant rather than silently
/// passing.
pub fn admin_certification_bundle() -> AdminCertificationBundle {
    let profiles: Vec<CertificationPacket> = CERTIFIED_PROFILES
        .iter()
        .map(|p| certify_profile(*p))
        .collect();
    let release_evidence = compute_release_evidence(&profiles);
    let invariants = compute_invariants(&profiles, &release_evidence);

    AdminCertificationBundle {
        record_kind: M5_ADMIN_CERTIFICATION_RECORD_KIND.to_owned(),
        m5_admin_certification_schema_version: M5_ADMIN_CERTIFICATION_SCHEMA_VERSION,
        schema_ref: M5_ADMIN_CERTIFICATION_SCHEMA_REF.to_owned(),
        bundle_id: M5_ADMIN_CERTIFICATION_BUNDLE_ID.to_owned(),
        as_of: M5_ADMIN_CERTIFICATION_AS_OF.to_owned(),
        matrix_ref: M5_ADMIN_CERTIFICATION_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ADMIN_CERTIFICATION_FREEZE_GATE_REF.to_owned(),
        summary: "Admin-plane certification — effective-policy, decision-history, endpoint-posture, \
                  retention/deletion, offboarding, and procurement/admin-packet truth qualified per \
                  profile against the upstream proof lanes that already produce it, bound back to \
                  the frozen admin-plane matrix and read identically by shell, CLI/headless, \
                  Help/About, support export, commercial/procurement, and release evidence across \
                  the managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline \
                  profiles. A family qualifies only when its proof is fresh and passing; a profile's \
                  managed claim auto-narrows off confirmed the moment any claimed family's proof \
                  goes stale or fails, and the release-evidence rows carry the worst case."
            .to_owned(),
        profiles,
        release_evidence,
        invariants,
        raw_payload_excluded: true,
    }
}

/// One family's proof state on one profile: its freshness and whether the proof
/// is failing.
struct FamilyProof {
    family: CertifiedFamilyClass,
    claimed: bool,
    mirror_backed: bool,
    freshness: EvidenceAgeClass,
    failing: bool,
    evidence_note: &'static str,
}

/// The per-family proof inputs for a profile. Held as data so the qualification
/// and claim states are computed, never hand-asserted.
fn profile_proofs(profile: AdminPathClass) -> Vec<FamilyProof> {
    use CertifiedFamilyClass::*;
    use EvidenceAgeClass::{Fresh, Recent, Stale};

    let proof = |family: CertifiedFamilyClass,
                 freshness: EvidenceAgeClass,
                 failing: bool,
                 mirror_backed: bool,
                 evidence_note: &'static str| FamilyProof {
        family,
        claimed: true,
        mirror_backed,
        freshness,
        failing,
        evidence_note,
    };

    match profile {
        // Managed cloud: control plane online, every proof lane fresh and passing.
        AdminPathClass::ManagedCloud => vec![
            proof(
                PolicyExplainability,
                Fresh,
                false,
                false,
                "Effective-policy, policy-diff, and locked-state proof is fresh and its freeze gate \
                 passes.",
            ),
            proof(
                DecisionHistory,
                Fresh,
                false,
                false,
                "Decision-history / audit-explorer proof is fresh and its freeze gate passes.",
            ),
            proof(
                EndpointPosture,
                Fresh,
                false,
                false,
                "Endpoint-posture proof is fresh and its freeze gate passes.",
            ),
            proof(
                RetentionDelete,
                Fresh,
                false,
                false,
                "Retention / deletion proof is fresh and its freeze gate passes.",
            ),
            proof(
                Offboarding,
                Fresh,
                false,
                false,
                "Offboarding proof is fresh and its freeze gate passes.",
            ),
            proof(
                ProcurementAdminPacket,
                Fresh,
                false,
                false,
                "Procurement / admin-packet proof is fresh and its freeze gate passes.",
            ),
        ],
        // Self-hosted: customer-operated, healthy except the audit-history proof is
        // failing, which narrows that one family without touching the rest.
        AdminPathClass::SelfHosted => vec![
            proof(
                PolicyExplainability,
                Recent,
                false,
                false,
                "Effective-policy proof is recent and passing.",
            ),
            proof(
                DecisionHistory,
                Fresh,
                true,
                false,
                "Decision-history proof is fresh in age but its freeze gate is failing, so audit \
                 history is narrowed off qualified until it passes again.",
            ),
            proof(
                EndpointPosture,
                Recent,
                false,
                false,
                "Endpoint-posture proof is recent and passing.",
            ),
            proof(
                RetentionDelete,
                Recent,
                false,
                false,
                "Retention / deletion proof is recent and passing.",
            ),
            proof(
                Offboarding,
                Recent,
                false,
                false,
                "Offboarding proof is recent and passing.",
            ),
            proof(
                ProcurementAdminPacket,
                Recent,
                false,
                false,
                "Procurement / admin-packet proof is recent and passing.",
            ),
        ],
        // Sovereign / air-gapped: no outbound control plane, so the posture and
        // procurement evidence age past their windows and narrow on stale evidence.
        AdminPathClass::SovereignAirGapped => vec![
            proof(
                PolicyExplainability,
                Recent,
                false,
                false,
                "Effective-policy proof from the last signed offline bundle is recent and passing.",
            ),
            proof(
                DecisionHistory,
                Recent,
                false,
                false,
                "Decision-history proof from the last signed offline bundle is recent and passing.",
            ),
            proof(
                EndpointPosture,
                Stale,
                false,
                false,
                "Endpoint-posture evidence is past its soft-refresh window with no outbound control \
                 plane, so it narrows off qualified.",
            ),
            proof(
                RetentionDelete,
                Recent,
                false,
                false,
                "Retention / deletion proof from the last signed offline bundle is recent and \
                 passing.",
            ),
            proof(
                Offboarding,
                Recent,
                false,
                false,
                "Offboarding proof from the last signed offline bundle is recent and passing.",
            ),
            proof(
                ProcurementAdminPacket,
                Stale,
                false,
                false,
                "Procurement / verification proof is past its validity window offline, so it \
                 narrows off qualified until a fresh signed packet is imported.",
            ),
        ],
        // Mirrored / offline: managed source offline, so the mirror-backed
        // retention and offboarding proof is stale and narrows on mirror evidence.
        AdminPathClass::MirroredOffline => vec![
            proof(
                PolicyExplainability,
                Recent,
                false,
                false,
                "Effective-policy proof from the last mirror sync is recent and passing.",
            ),
            proof(
                DecisionHistory,
                Recent,
                false,
                false,
                "Decision-history proof from the last mirror sync is recent and passing.",
            ),
            proof(
                EndpointPosture,
                Recent,
                false,
                false,
                "Endpoint-posture proof from the last mirror sync is recent and passing.",
            ),
            proof(
                RetentionDelete,
                Stale,
                false,
                true,
                "Retention / deletion proof is mirror-backed and the offline mirror's last sync is \
                 past its window, so it narrows off qualified.",
            ),
            proof(
                Offboarding,
                Stale,
                false,
                true,
                "Offboarding proof is mirror-backed and the offline mirror's last sync is past its \
                 window, so it narrows off qualified.",
            ),
            proof(
                ProcurementAdminPacket,
                Recent,
                false,
                false,
                "Procurement / admin-packet proof from the last mirror sync is recent and passing.",
            ),
        ],
        // Non-managed paths are not certified by this capstone.
        _ => Vec::new(),
    }
}

/// Computes the qualification verdict and narrow reason for a family from its
/// proof state. The order matters: a missing lane outranks a failing proof, which
/// outranks stale evidence.
fn qualify(proof: &FamilyProof) -> (QualificationClass, Option<ClaimNarrowReasonClass>) {
    if !proof.family.proof_lane().is_proven() {
        return (
            QualificationClass::NarrowedUnproven,
            Some(ClaimNarrowReasonClass::FamilyProofMissing),
        );
    }
    if proof.failing {
        return (
            QualificationClass::NarrowedFailingProof,
            Some(ClaimNarrowReasonClass::FamilyProofFailing),
        );
    }
    if proof.freshness.is_stale() {
        let reason = if proof.mirror_backed {
            ClaimNarrowReasonClass::MirrorEvidenceStale
        } else {
            ClaimNarrowReasonClass::FamilyEvidenceStale
        };
        return (QualificationClass::NarrowedStaleEvidence, Some(reason));
    }
    (QualificationClass::Qualified, None)
}

fn certify_profile(profile: AdminPathClass) -> CertificationPacket {
    use AdminConsumerClass::*;

    let consumers = vec![
        ShellAdminCenter,
        CliHeadless,
        HelpAbout,
        SupportExport,
        CommercialProcurement,
        ReleaseEvidence,
        ManagedService,
    ];

    let (deployment_profile, summary) = match profile {
        AdminPathClass::ManagedCloud => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Managed-cloud profile: the control plane is online and every admin-plane proof lane is \
             fresh and passing, so the managed claim is confirmed.",
        ),
        AdminPathClass::SelfHosted => (
            AdminDeploymentProfileClass::SelfHosted,
            "Self-hosted profile: the customer operates the control plane; the audit-history proof \
             is failing, so the managed claim auto-narrows off confirmed until it passes again.",
        ),
        AdminPathClass::SovereignAirGapped => (
            AdminDeploymentProfileClass::SovereignAirGapped,
            "Sovereign / air-gapped profile: no outbound control plane, so the endpoint-posture and \
             procurement proof ages past its window and the managed claim auto-narrows off \
             confirmed.",
        ),
        AdminPathClass::MirroredOffline => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Mirrored / offline profile: the managed source is offline, so the mirror-backed \
             retention and offboarding proof is stale and the managed claim auto-narrows off \
             confirmed.",
        ),
        _ => (AdminDeploymentProfileClass::IndividualLocal, "Local profile."),
    };

    let families: Vec<FamilyQualificationRow> = profile_proofs(profile)
        .into_iter()
        .map(|proof| build_row(profile, proof))
        .collect();

    let proof_freshness = worst_age(
        families
            .iter()
            .filter(|r| r.claimed)
            .map(|r| r.proof_freshness),
    );

    let (claim_state, narrow_reasons) = compute_claim(&families);
    let claim_note = claim_note(&families, &narrow_reasons);

    CertificationPacket {
        profile,
        profile_id: profile.path_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers,
        families,
        proof_freshness,
        claim_state,
        narrow_reasons,
        claim_note,
    }
}

fn build_row(profile: AdminPathClass, proof: FamilyProof) -> FamilyQualificationRow {
    let (qualification, narrow_reason) = qualify(&proof);
    let family = proof.family;
    FamilyQualificationRow {
        family,
        row_id: row_id(profile, family),
        label: family.label().to_owned(),
        summary: format!(
            "{} certified against {} on the {} profile.",
            family.label(),
            family.proof_lane().lane_id,
            profile.as_str()
        ),
        bound_surfaces: family.bound_surfaces().to_vec(),
        proof_lane: family.proof_lane(),
        claimed: proof.claimed,
        mirror_backed: proof.mirror_backed,
        proof_freshness: proof.freshness,
        proof_failing: proof.failing,
        qualification,
        claim_state: qualification.claim_state(),
        narrow_reason,
        owner: family.owner(),
        escalation_owner: family.escalation_owner(),
        locally_explainable: true,
        redaction: AdminRedactionClass::MetadataSafeDefault,
        evidence_note: proof.evidence_note.to_owned(),
    }
}

/// The stalest of a set of evidence ages — the conservative aggregate that drives
/// the reported proof freshness.
fn worst_age(ages: impl IntoIterator<Item = EvidenceAgeClass>) -> EvidenceAgeClass {
    ages.into_iter().max().unwrap_or(EvidenceAgeClass::Fresh)
}

/// Computes the auto-narrowed claim state and its deduplicated reasons from the
/// claimed family rows. Confirmed only when every claimed family qualifies.
fn compute_claim(
    families: &[FamilyQualificationRow],
) -> (AdminStateClass, Vec<ClaimNarrowReasonClass>) {
    let mut reasons: Vec<ClaimNarrowReasonClass> = Vec::new();
    let mut worst = QualificationClass::Qualified;
    for row in families.iter().filter(|r| r.claimed) {
        if let Some(reason) = row.narrow_reason {
            if !reasons.contains(&reason) {
                reasons.push(reason);
            }
        }
        if row.qualification.rank() > worst.rank() {
            worst = row.qualification;
        }
    }
    (worst.claim_state(), reasons)
}

fn claim_note(
    families: &[FamilyQualificationRow],
    narrow_reasons: &[ClaimNarrowReasonClass],
) -> String {
    if narrow_reasons.is_empty() {
        return "Every claimed admin-plane family is proven fresh and passing; the managed claim is \
                confirmed."
            .to_owned();
    }
    let narrowed: Vec<&str> = families
        .iter()
        .filter(|r| r.claimed && !r.is_qualified())
        .map(|r| r.family.as_str())
        .collect();
    let reasons: Vec<&str> = narrow_reasons.iter().map(|r| r.as_str()).collect();
    format!(
        "Managed claim auto-narrowed off confirmed because these families are not proven current: \
         {} ({}).",
        narrowed.join(", "),
        reasons.join(", ")
    )
}

/// Computes the release-evidence rows: one per named dimension, carrying the worst
/// qualification across every profile for the dimension's families.
fn compute_release_evidence(profiles: &[CertificationPacket]) -> Vec<ReleaseEvidenceRow> {
    ReleaseEvidenceDimensionClass::ALL
        .iter()
        .map(|dimension| {
            let families = dimension.families();
            let worst = worst_qualification(profiles, families);
            let note = if worst.is_qualified() {
                format!(
                    "{} is proven current on every profile.",
                    dimension.label()
                )
            } else {
                format!(
                    "{} is narrowed on at least one profile ({}); release automation downgrades the \
                     affected managed claim.",
                    dimension.label(),
                    worst.as_str()
                )
            };
            ReleaseEvidenceRow {
                dimension: *dimension,
                row_id: format!("admin_cert.release_evidence.{}", dimension.as_str()),
                label: dimension.label().to_owned(),
                statement: release_evidence_statement(*dimension),
                families: families.to_vec(),
                worst_qualification: worst,
                claim_state: worst.claim_state(),
                note,
            }
        })
        .collect()
}

/// The worst (most narrowed) qualification across every profile for a set of
/// families.
fn worst_qualification(
    profiles: &[CertificationPacket],
    families: &[CertifiedFamilyClass],
) -> QualificationClass {
    let mut worst = QualificationClass::Qualified;
    for packet in profiles {
        for row in &packet.families {
            if row.claimed
                && families.contains(&row.family)
                && row.qualification.rank() > worst.rank()
            {
                worst = row.qualification;
            }
        }
    }
    worst
}

fn release_evidence_statement(dimension: ReleaseEvidenceDimensionClass) -> String {
    match dimension {
        ReleaseEvidenceDimensionClass::PolicySourceVerification => {
            "Effective policy source and verification posture, plus endpoint posture, are proven \
             current per profile."
        }
        ReleaseEvidenceDimensionClass::AuditHistory => {
            "The decision-history timeline / audit-event explorer is proven current per profile."
        }
        ReleaseEvidenceDimensionClass::DeleteExportHonesty => {
            "Delete and export outcomes — what can be deleted or exported now versus later — are \
             proven current per profile."
        }
        ReleaseEvidenceDimensionClass::OffboardingContinuity => {
            "Offboarding export, deletion, and local-safe continuity steps are proven current per \
             profile."
        }
        ReleaseEvidenceDimensionClass::ProcurementSupportAdminPacket => {
            "Procurement / verification packets and admin-handoff bundles are proven current per \
             profile."
        }
    }
    .to_owned()
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> CertificationInvariant {
    CertificationInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    profiles: &[CertificationPacket],
    release_evidence: &[ReleaseEvidenceRow],
) -> Vec<CertificationInvariant> {
    let matrix = admin_plane_matrix();

    let surface_ok = |surface: AdminSurfaceClass| -> bool {
        matrix
            .surface(surface)
            .is_some_and(|e| e.locally_explainable && e.typed_not_portal_only)
    };
    let state_in_vocabulary =
        |state: AdminStateClass| -> bool { matrix.state_term(state).is_some() };

    let mut out = Vec::new();

    // Every certified family has a row on every profile, and every family is
    // certified somewhere.
    out.push(invariant(
        "admin_cert.families_covered",
        "Every certified admin-plane family — policy explainability, decision history, endpoint \
         posture, retention/delete, offboarding, and procurement/admin packet — has exactly one \
         qualification row on every certified profile.",
        profiles.iter().all(|p| {
            CertifiedFamilyClass::ALL
                .iter()
                .all(|f| p.families.iter().filter(|r| r.family == *f).count() == 1)
        }),
    ));

    // Every claimed managed-bearing profile is certified.
    out.push(invariant(
        "admin_cert.profiles_covered",
        "The bundle certifies the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles.",
        CERTIFIED_PROFILES
            .iter()
            .all(|profile| profiles.iter().any(|p| p.profile == *profile)),
    ));

    // Every surface a family binds is present in the matrix, locally explainable,
    // and typed rather than portal-only.
    out.push(invariant(
        "admin_cert.bound_surfaces_in_matrix",
        "Every surface a family certifies is present in the frozen admin-plane matrix, locally \
         explainable, and typed rather than portal-only, so certification cannot drift from the \
         contract.",
        profiles.iter().all(|p| {
            p.families.iter().all(|r| {
                !r.bound_surfaces.is_empty() && r.bound_surfaces.iter().copied().all(surface_ok)
            })
        }) && CertifiedFamilyClass::ALL
            .iter()
            .all(|f| f.bound_surfaces().iter().copied().all(surface_ok)),
    ));

    // Every qualification headline state is one the matrix's unified state
    // vocabulary defines.
    out.push(invariant(
        "admin_cert.claim_states_in_vocabulary",
        "Every per-row, per-profile, and release-evidence claim state is one the frozen matrix's \
         unified state vocabulary defines.",
        profiles.iter().all(|p| {
            state_in_vocabulary(p.claim_state)
                && p.families
                    .iter()
                    .all(|r| state_in_vocabulary(r.claim_state))
        }) && release_evidence
            .iter()
            .all(|r| state_in_vocabulary(r.claim_state)),
    ));

    // No green on stale or failing proof: a row qualifies exactly when its proof is
    // proven, fresh, and passing; otherwise it narrows and names a reason.
    out.push(invariant(
        "admin_cert.no_green_on_stale_or_failing",
        "A family row reads qualified only when its proof is bound, fresh, and passing; a row whose \
         proof is stale, failing, or unproven is narrowed off qualified, reads a non-active \
         downgrade state, and names a narrow reason.",
        profiles.iter().all(|p| {
            p.families.iter().all(|r| {
                let (expected_qual, expected_reason) = qualify(&proof_view(r));
                r.qualification == expected_qual
                    && r.narrow_reason == expected_reason
                    && r.claim_state == expected_qual.claim_state()
                    && (r.is_qualified()
                        == (r.claim_state == AdminStateClass::ActiveEnforced
                            && r.narrow_reason.is_none()))
            })
        }),
    ));

    // A qualified row must cite a real, export-safe proof lane, so a claim can never
    // go green because the mechanics exist somewhere while the proof is absent.
    out.push(invariant(
        "admin_cert.qualified_requires_proven_lane",
        "A qualified row cites an upstream proof lane whose schema, fixture, freeze gate, doc, and \
         producer refs are all present and export-safe, so a claim cannot go green because the \
         mechanics exist somewhere in the stack while the user-facing proof is absent.",
        profiles
            .iter()
            .all(|p| p.families.iter().all(|r| !r.is_qualified() || r.proof_lane.is_proven())),
    ));

    // Every family row cites a non-empty upstream proof lane.
    out.push(invariant(
        "admin_cert.proof_lane_bound",
        "Every family row — qualified or narrowed — cites a non-empty upstream proof lane, so the \
         certification is always traceable to a producing crate, schema, fixture, and freeze gate.",
        profiles.iter().all(|p| {
            p.families
                .iter()
                .all(|r| !r.proof_lane.lane_id.is_empty() && r.proof_lane.is_proven())
        }),
    ));

    // A profile's managed claim auto-narrows off confirmed exactly when a claimed
    // family does not qualify.
    out.push(invariant(
        "admin_cert.profile_claim_auto_narrows",
        "A profile's managed claim reads confirmed only when every claimed family qualifies; when \
         any claimed family is stale, failing, or unproven the claim downgrades off confirmed and \
         names which reasons narrowed it.",
        profiles.iter().all(|p| {
            let (expected_state, expected_reasons) = compute_claim(&p.families);
            let all_qualified = p.claimed_rows().all(|r| r.is_qualified());
            p.claim_state == expected_state
                && p.narrow_reasons == expected_reasons
                && (p.claim_confirmed() == all_qualified)
        }),
    ));

    // The reported proof freshness is the stalest claimed family.
    out.push(invariant(
        "admin_cert.proof_freshness_is_worst_case",
        "Each profile's reported proof freshness is the stalest of its claimed family rows, so a \
         single stale family cannot hide behind fresher siblings.",
        profiles
            .iter()
            .all(|p| p.proof_freshness == worst_age(p.claimed_rows().map(|r| r.proof_freshness))),
    ));

    // The bundle publishes an explicit release-evidence row for every named
    // dimension, bound to at least one family.
    out.push(invariant(
        "admin_cert.release_evidence_rows_present",
        "The bundle publishes exactly one release-evidence row for each named dimension — policy \
         source/verification, audit history, delete/export honesty, offboarding continuity, and \
         procurement/support/admin-packet fidelity — each bound to at least one certified family.",
        ReleaseEvidenceDimensionClass::ALL.iter().all(|d| {
            release_evidence
                .iter()
                .filter(|r| r.dimension == *d)
                .count()
                == 1
                && !d.families().is_empty()
        }) && release_evidence.len() == ReleaseEvidenceDimensionClass::ALL.len(),
    ));

    // Each release-evidence row reflects the worst qualification across all
    // profiles, never a rosier summary.
    out.push(invariant(
        "admin_cert.release_evidence_reflects_worst",
        "Each release-evidence row's worst qualification equals the most-narrowed qualification \
         across every profile for its bound families, so release evidence is never rosier than the \
         underlying admin plane.",
        release_evidence.iter().all(|r| {
            r.worst_qualification == worst_qualification(profiles, &r.families)
                && r.claim_state == r.worst_qualification.claim_state()
        }),
    ));

    // Every family is locally explainable, never portal-only.
    out.push(invariant(
        "admin_cert.local_explainability",
        "Every certified family is locally explainable: a user can see the qualification state \
         without a separate vendor console.",
        profiles
            .iter()
            .all(|p| p.families.iter().all(|r| r.locally_explainable)),
    ));

    // Cross-surface parity: one typed packet serves every required consumer.
    out.push(invariant(
        "admin_cert.consumer_parity",
        "Each profile is one typed packet consumed identically by shell, CLI/headless, Help/About, \
         support export, commercial/procurement, and release evidence, so About/help/support/\
         commercial read the qualification state instead of restating it.",
        profiles
            .iter()
            .all(|p| PARITY_CONSUMERS.iter().all(|c| p.consumers.contains(c))),
    ));

    // Stable ids are unique within their scope.
    out.push(invariant(
        "admin_cert.stable_ids_unique",
        "Profile ids, family row ids, and release-evidence row ids are unique within their scope, \
         so a consumer can resolve any object by a stable id.",
        all_unique(profiles.iter().map(|p| p.profile_id.as_str()))
            && profiles
                .iter()
                .all(|p| all_unique(p.families.iter().map(|r| r.row_id.as_str())))
            && all_unique(release_evidence.iter().map(|r| r.row_id.as_str())),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "admin_cert.export_safe",
        "Every stable id is an opaque token and every proof-lane ref is a repo-relative object ref, \
         with no URL scheme or absolute path, so the bundle is safe to embed in a support export \
         verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && p.families.iter().all(|r| {
                    is_safe_token(r.row_id.as_str())
                        && is_safe_token(r.proof_lane.lane_id.as_str())
                        && r.proof_lane.is_proven()
                })
        }) && release_evidence
            .iter()
            .all(|r| is_safe_token(r.row_id.as_str())),
    ));

    // The bundle binds the frozen admin-plane matrix.
    out.push(invariant(
        "admin_cert.binds_admin_plane_matrix",
        "The bundle binds the frozen admin-plane matrix by id and cites its canonical fixture, so \
         the certification and the contract it qualifies cannot drift apart.",
        matrix.matrix_id == M5_ADMIN_PLANE_MATRIX_ID
            && M5_ADMIN_CERTIFICATION_MATRIX_REF.starts_with("fixtures/admin/m5-admin-plane/"),
    ));

    out
}

/// Reconstructs a [`FamilyProof`] view from a built row, so the invariants can
/// re-derive the expected qualification without trusting the stored verdict.
fn proof_view(row: &FamilyQualificationRow) -> FamilyProof {
    FamilyProof {
        family: row.family,
        claimed: row.claimed,
        mirror_backed: row.mirror_backed,
        freshness: row.proof_freshness,
        failing: row.proof_failing,
        evidence_note: "",
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn admin_certification_lines(bundle: &AdminCertificationBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Admin-certification bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Profiles: {}  Release-evidence rows: {}  Invariants: {}  (binds matrix {})",
        bundle.profiles.len(),
        bundle.release_evidence.len(),
        bundle.invariants.len(),
        bundle.matrix_id,
    ));

    for p in &bundle.profiles {
        lines.push(format!("Profile {} [{}]", p.profile.as_str(), p.profile_id));
        lines.push(format!("  {}", p.summary));
        lines.push(format!(
            "  Claim: {} (proof freshness {}){}",
            p.claim_state.as_str(),
            p.proof_freshness.as_str(),
            if p.narrow_reasons.is_empty() {
                String::new()
            } else {
                let r: Vec<&str> = p.narrow_reasons.iter().map(|x| x.as_str()).collect();
                format!(" narrowed: {}", r.join(", "))
            }
        ));
        lines.push("  Families:".to_owned());
        for r in &p.families {
            lines.push(format!(
                "    - {} [{}] qualification={} state={} freshness={}{}",
                r.family.as_str(),
                r.proof_lane.lane_id,
                r.qualification.as_str(),
                r.claim_state.as_str(),
                r.proof_freshness.as_str(),
                match r.narrow_reason {
                    Some(reason) => format!(" reason={}", reason.as_str()),
                    None => String::new(),
                }
            ));
        }
    }

    lines.push("Release evidence:".to_owned());
    for r in &bundle.release_evidence {
        lines.push(format!(
            "  - {} [{}] worst={} state={}",
            r.dimension.as_str(),
            r.row_id,
            r.worst_qualification.as_str(),
            r.claim_state.as_str(),
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
