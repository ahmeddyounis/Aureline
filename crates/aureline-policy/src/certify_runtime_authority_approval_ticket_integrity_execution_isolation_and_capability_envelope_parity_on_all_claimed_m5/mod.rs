//! Certify runtime-authority, approval-ticket integrity, execution-isolation,
//! and capability-envelope parity on every claimed M5 executing profile.
//!
//! The frozen runtime-authority matrix states *what authority each M5 executing
//! surface claims*; the surface-resolution, approval-ticket, capability-envelope,
//! child-derivation, authority-lifecycle, and launch-inspector contracts each
//! prove *one dimension* of that claim. This module is the **certification side**
//! of that contract: it joins those per-dimension proofs into a single
//! inspectable verdict per claimed surface and **auto-narrows any surface that
//! lacks current proof** instead of letting a claim outrun its evidence.
//!
//! Each [`M5SurfaceCertificationEntry`] certifies one claimed surface against
//! four parity dimensions ([`M5CertifiedAuthorityDimension`]):
//!
//! - **execution isolation** — the surface's sandbox-profile descriptor is
//!   resolved and supported on the claimed platform;
//! - **approval-ticket integrity** — the surface's mutating actions are gated by
//!   a replay-protected, expiry-bounded, locally verifiable ticket;
//! - **capability-envelope parity** — the surface runs inside an actor- and
//!   target-bound envelope with handle-only secret refs;
//! - **runtime-authority lineage** — the surface's authority is issued, used,
//!   and revoked under the frozen matrix with no ambient privilege.
//!
//! Each dimension carries an [`M5CertificationProofStatus`] (`Current`, `Stale`,
//! `Missing`, or `UnsupportedBackend`). The entry's [`M5CertificationVerdict`] is
//! the **worst** status across its dimensions:
//!
//! - all `Current` → `Certified`, effective qualification equals the claim;
//! - any `Stale` → `NarrowedStaleProof`, narrowed to a sanitized preview;
//! - any `Missing` → `NarrowedMissingProof`, held pending fresh proof;
//! - any `UnsupportedBackend` → `FailedClosedUnsupportedBackend`, fails closed.
//!
//! The effective qualification is **never wider than the claimed qualification**:
//! a narrowed verdict always lands on a strictly more-restricted tier, so a
//! missing, stale, or unsupported proof can never silently widen a claim. A
//! narrowed or failed-closed entry always names its downgrade trigger, narrowed
//! fallback, and a concrete recovery action.
//!
//! The track invariant holds end to end: no ambient privilege; no helper
//! self-issues authority; sandbox profile, approval-ticket posture, secret scope,
//! policy claim, and degraded fallback stay inspectable and export-safe; and if a
//! dimension cannot be proven the surface **narrows or fails closed with a named
//! trigger** instead of silently widening. No raw secret material, credential
//! body, or live ticket signature is ever exported.
//!
//! The boundary schema is
//! [`schemas/execution-auth/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.schema.json`](../../../../schemas/execution-auth/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.md`](../../../../docs/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/`](../../../../fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5ApprovalTicketPosture, M5DegradedFallback,
    M5ExecutingSurface, M5RuntimeAuthorityDowngradeTrigger, M5RuntimeAuthorityMatrixSurfaceRow,
    M5RuntimeAuthorityQualificationClass, M5SandboxProfile, M5SecretScope,
    M5_RUNTIME_AUTHORITY_MATRIX_ARTIFACT_REF, M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID, M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
};
use super::implement_execution_surface_classes_sandbox_profile_descriptors_and_unsupported_or_stricter_profile_truth::{
    M5_EXECUTION_SURFACE_RESOLUTION_ARTIFACT_REF, M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF,
    M5_EXECUTION_SURFACE_RESOLUTION_PACKET_ID, M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
};
use super::implement_approval_ticket_issuance_deny_reason_packets_replay_nonce_or_expiry_enforcement_and_local_first_verification_f::{
    M5_APPROVAL_TICKET_LEDGER_ARTIFACT_REF, M5_APPROVAL_TICKET_LEDGER_DOC_REF,
    M5_APPROVAL_TICKET_LEDGER_PACKET_ID, M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF,
};
use super::ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::{
    M5_CAPABILITY_ENVELOPE_ARTIFACT_REF, M5_CAPABILITY_ENVELOPE_DOC_REF,
    M5_CAPABILITY_ENVELOPE_PACKET_ID, M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
};
use super::ship_child_envelope_derivation_nested_launch_narrowing_handle_only_secret_projection_and_no_ambient_privilege_enforcemen::{
    M5_CHILD_ENVELOPE_DERIVATION_DOC_REF, M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF,
};
use super::add_issue_use_revoke_audit_ledgers_invalidation_on_target_or_trust_or_policy_or_sandbox_drift_and_support_export_safe_au::{
    M5_AUTHORITY_LIFECYCLE_LEDGER_DOC_REF, M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_REF,
};
use super::add_launch_inspector_and_command_runtime_explain_sheets_that_answer_where_this_runs_why_this_toolchain_what_it_can_acces::{
    M5_LAUNCH_INSPECTOR_DOC_REF, M5_LAUNCH_INSPECTOR_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5RuntimeAuthorityCertificationPacket`].
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_RECORD_KIND: &str =
    "certify_m5_runtime_authority_approval_ticket_execution_isolation_capability_envelope_parity";

/// Schema version for the M5 runtime-authority certification packet records.
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/execution-auth/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_DOC_REF: &str =
    "docs/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5";

/// Stable packet id minted by [`frozen_stable_m5_runtime_authority_certification_packet`].
pub const M5_RUNTIME_AUTHORITY_CERTIFICATION_PACKET_ID: &str =
    "m5-runtime-authority-certification:stable:0001";

/// One parity dimension a claimed surface is certified against.
///
/// A surface keeps its claim only when **every** dimension carries current proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertifiedAuthorityDimension {
    /// The execution-isolation (sandbox) profile is resolved and supported.
    ExecutionIsolation,
    /// Mutating actions are gated by a replay- and expiry-protected ticket.
    ApprovalTicketIntegrity,
    /// The surface runs inside an actor- and target-bound capability envelope.
    CapabilityEnvelopeParity,
    /// Authority is issued, used, and revoked under the frozen matrix.
    RuntimeAuthorityLineage,
}

impl M5CertifiedAuthorityDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExecutionIsolation,
        Self::ApprovalTicketIntegrity,
        Self::CapabilityEnvelopeParity,
        Self::RuntimeAuthorityLineage,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionIsolation => "execution_isolation",
            Self::ApprovalTicketIntegrity => "approval_ticket_integrity",
            Self::CapabilityEnvelopeParity => "capability_envelope_parity",
            Self::RuntimeAuthorityLineage => "runtime_authority_lineage",
        }
    }

    /// Repo-relative artifact ref of the source packet that proves this dimension.
    pub const fn source_artifact_ref(self) -> &'static str {
        match self {
            Self::ExecutionIsolation => M5_EXECUTION_SURFACE_RESOLUTION_ARTIFACT_REF,
            Self::ApprovalTicketIntegrity => M5_APPROVAL_TICKET_LEDGER_ARTIFACT_REF,
            Self::CapabilityEnvelopeParity => M5_CAPABILITY_ENVELOPE_ARTIFACT_REF,
            Self::RuntimeAuthorityLineage => M5_RUNTIME_AUTHORITY_MATRIX_ARTIFACT_REF,
        }
    }

    /// Repo-relative schema ref of the source packet that proves this dimension.
    pub const fn source_schema_ref(self) -> &'static str {
        match self {
            Self::ExecutionIsolation => M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
            Self::ApprovalTicketIntegrity => M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF,
            Self::CapabilityEnvelopeParity => M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
            Self::RuntimeAuthorityLineage => M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        }
    }

    /// Stable packet id of the source packet that proves this dimension.
    pub const fn source_packet_id(self) -> &'static str {
        match self {
            Self::ExecutionIsolation => M5_EXECUTION_SURFACE_RESOLUTION_PACKET_ID,
            Self::ApprovalTicketIntegrity => M5_APPROVAL_TICKET_LEDGER_PACKET_ID,
            Self::CapabilityEnvelopeParity => M5_CAPABILITY_ENVELOPE_PACKET_ID,
            Self::RuntimeAuthorityLineage => M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID,
        }
    }
}

/// Freshness/availability status of one dimension's proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationProofStatus {
    /// The proof packet is present and within the freshness window.
    Current,
    /// The proof packet is present but past the freshness window.
    Stale,
    /// No proof packet is published for this dimension on the claimed surface.
    Missing,
    /// The enforcement backend the proof depends on is unsupported on this build.
    UnsupportedBackend,
}

impl M5CertificationProofStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::UnsupportedBackend => "unsupported_backend",
        }
    }

    /// Whether this proof keeps the surface's full claim.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Severity rank used to fold the worst status across a surface's dimensions.
    ///
    /// A higher rank is a worse (more narrowing) status.
    const fn severity_rank(self) -> u8 {
        match self {
            Self::Current => 0,
            Self::Stale => 1,
            Self::Missing => 2,
            Self::UnsupportedBackend => 3,
        }
    }
}

/// Verdict reached for one claimed surface after folding its dimension proofs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationVerdict {
    /// Every dimension carries current proof; the claim stands.
    Certified,
    /// At least one dimension's proof is stale; the claim is narrowed.
    NarrowedStaleProof,
    /// At least one dimension's proof is missing; the claim is held.
    NarrowedMissingProof,
    /// At least one dimension's enforcement backend is unsupported; fail closed.
    FailedClosedUnsupportedBackend,
}

impl M5CertificationVerdict {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedStaleProof => "narrowed_stale_proof",
            Self::NarrowedMissingProof => "narrowed_missing_proof",
            Self::FailedClosedUnsupportedBackend => "failed_closed_unsupported_backend",
        }
    }

    /// Whether the surface keeps its claimed qualification.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// The downgrade trigger a narrowed or failed verdict must record.
    pub const fn downgrade_trigger(self) -> Option<M5RuntimeAuthorityDowngradeTrigger> {
        match self {
            Self::Certified => None,
            Self::NarrowedStaleProof | Self::NarrowedMissingProof => {
                Some(M5RuntimeAuthorityDowngradeTrigger::UpstreamDependencyNarrowed)
            }
            Self::FailedClosedUnsupportedBackend => {
                Some(M5RuntimeAuthorityDowngradeTrigger::EnforcementBackendMissing)
            }
        }
    }

    /// The narrowed fallback a narrowed or failed verdict must record.
    pub const fn narrowed_to(self) -> Option<M5DegradedFallback> {
        match self {
            Self::Certified => None,
            Self::NarrowedStaleProof => Some(M5DegradedFallback::NarrowToSanitizedPreview),
            Self::NarrowedMissingProof => Some(M5DegradedFallback::RequireFreshTicket),
            Self::FailedClosedUnsupportedBackend => Some(M5DegradedFallback::FailClosedBlock),
        }
    }

    /// The qualification floor a narrowed verdict narrows the claim onto.
    ///
    /// `Certified` keeps the claim; every other verdict lands on a strictly
    /// more-restricted tier so a claim can never silently widen past its proof.
    pub const fn narrowed_floor(self) -> Option<M5RuntimeAuthorityQualificationClass> {
        match self {
            Self::Certified => None,
            Self::NarrowedStaleProof => Some(M5RuntimeAuthorityQualificationClass::Preview),
            Self::NarrowedMissingProof => Some(M5RuntimeAuthorityQualificationClass::Held),
            Self::FailedClosedUnsupportedBackend => {
                Some(M5RuntimeAuthorityQualificationClass::Unavailable)
            }
        }
    }
}

/// Folds the worst proof status across a surface's dimensions into a verdict.
///
/// This is the auto-narrowing gate: a single non-current dimension is enough to
/// strip a surface of its full claim.
pub fn certification_verdict_for<I>(statuses: I) -> M5CertificationVerdict
where
    I: IntoIterator<Item = M5CertificationProofStatus>,
{
    let worst = statuses
        .into_iter()
        .max_by_key(|status| status.severity_rank())
        .unwrap_or(M5CertificationProofStatus::Missing);
    match worst {
        M5CertificationProofStatus::Current => M5CertificationVerdict::Certified,
        M5CertificationProofStatus::Stale => M5CertificationVerdict::NarrowedStaleProof,
        M5CertificationProofStatus::Missing => M5CertificationVerdict::NarrowedMissingProof,
        M5CertificationProofStatus::UnsupportedBackend => {
            M5CertificationVerdict::FailedClosedUnsupportedBackend
        }
    }
}

/// Restriction rank for a qualification tier; a higher rank is more restricted.
///
/// Used to assert a narrowed verdict never widens the claim.
const fn qualification_restriction_rank(qualification: M5RuntimeAuthorityQualificationClass) -> u8 {
    match qualification {
        M5RuntimeAuthorityQualificationClass::Stable => 0,
        M5RuntimeAuthorityQualificationClass::Beta => 1,
        M5RuntimeAuthorityQualificationClass::Preview => 2,
        M5RuntimeAuthorityQualificationClass::Experimental => 3,
        M5RuntimeAuthorityQualificationClass::Held => 4,
        M5RuntimeAuthorityQualificationClass::Unavailable => 5,
    }
}

/// One dimension's proof binding for a claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertificationProof {
    /// The parity dimension this proof covers.
    pub dimension: M5CertifiedAuthorityDimension,
    /// Freshness/availability status of the proof.
    pub status: M5CertificationProofStatus,
    /// Repo-relative artifact ref of the source packet; never empty.
    pub proof_source_ref: String,
    /// Repo-relative schema ref of the source packet; never empty.
    pub proof_schema_ref: String,
    /// Stable packet id of the source packet; never empty.
    pub proof_packet_id: String,
    /// RFC 3339 timestamp the proof was last observed; never empty.
    pub proof_observed_at: String,
    /// Export-safe note describing the proof or why it does not hold; never empty.
    pub note: String,
}

/// One claimed surface certified against all four parity dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceCertificationEntry {
    /// Matrix surface being certified.
    pub surface: M5ExecutingSurface,
    /// Qualification the matrix claims for this surface.
    pub claimed_qualification: M5RuntimeAuthorityQualificationClass,
    /// Default execution-isolation profile claimed for this surface.
    pub default_sandbox_profile: M5SandboxProfile,
    /// Required approval-ticket posture claimed for this surface.
    pub approval_ticket_posture: M5ApprovalTicketPosture,
    /// Secret-scope posture claimed for this surface.
    pub secret_scope: M5SecretScope,
    /// Whether this surface is an untrusted helper that must never self-issue.
    pub is_untrusted_helper: bool,
    /// Per-dimension proof bindings; one per [`M5CertifiedAuthorityDimension`].
    pub proofs: Vec<M5CertificationProof>,
    /// Verdict folded from the dimension proofs.
    pub verdict: M5CertificationVerdict,
    /// Effective qualification after auto-narrowing; never wider than claimed.
    pub effective_qualification: M5RuntimeAuthorityQualificationClass,
    /// Narrowed fallback; present exactly when the verdict is not `Certified`.
    pub narrowed_to: Option<M5DegradedFallback>,
    /// Downgrade trigger; present exactly when the verdict is not `Certified`.
    pub downgrade_trigger: Option<M5RuntimeAuthorityDowngradeTrigger>,
    /// Export-safe recovery or status note; never empty.
    pub recovery_action: String,
    /// Per-entry redaction class token.
    pub redaction_class_token: String,
}

/// Trust and isolation review block for the certification packet.
///
/// Every field encodes a hard invariant; all must hold for the packet to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityCertificationTrustReview {
    /// No certified surface confers ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No helper surface self-issues authority.
    pub no_self_issued_authority_by_helpers: bool,
    /// Every claimed surface is certified across all four parity dimensions.
    pub every_surface_certified_across_all_dimensions: bool,
    /// A missing or stale proof auto-narrows the affected surface.
    pub auto_narrows_on_missing_or_stale_proof: bool,
    /// An unsupported enforcement backend fails the surface closed.
    pub fail_closes_on_unsupported_backend: bool,
    /// A narrowed verdict never widens a claim past its proof.
    pub never_silently_widens_a_claim: bool,
    /// Secret references are handle-only; no raw secret material is recorded.
    pub secret_refs_handle_only_no_raw_material: bool,
    /// The certification verifies offline without widening authority.
    pub certification_offline_verifiable: bool,
    /// No raw secret material is exported inside the certification packet.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityCertificationConsumerProjection {
    /// Desktop shows the certification verdict and any narrowing.
    pub desktop_shows_certification_and_narrowing: bool,
    /// Command palette and policy inspector reference the same certification.
    pub command_and_policy_reference_same_certification: bool,
    /// CLI / headless reads the certification offline.
    pub cli_headless_reads_certification_offline: bool,
    /// Help / About ingests the certification status rather than cloning prose.
    pub help_about_ingests_certification_status: bool,
    /// Diagnostics consumes the certification.
    pub diagnostics_consumes_certification: bool,
    /// Release evidence gates promotion on the certification.
    pub release_evidence_gates_on_certification: bool,
    /// Support export shows the full certification.
    pub support_export_shows_full_certification: bool,
    /// Remote and browser-routed surfaces preserve certification semantics.
    pub remote_and_browser_preserve_certification_semantics: bool,
}

/// Proof-freshness block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last certification refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected surfaces.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5RuntimeAuthorityCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RuntimeAuthorityCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Certification entries.
    pub entries: Vec<M5SurfaceCertificationEntry>,
    /// Trust review block.
    pub trust_review: M5RuntimeAuthorityCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5RuntimeAuthorityCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RuntimeAuthorityCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 runtime-authority certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeAuthorityCertificationPacket {
    /// Record kind; must equal [`M5_RUNTIME_AUTHORITY_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Certification entries.
    pub entries: Vec<M5SurfaceCertificationEntry>,
    /// Trust review block.
    pub trust_review: M5RuntimeAuthorityCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5RuntimeAuthorityCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RuntimeAuthorityCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RuntimeAuthorityCertificationPacket {
    /// Builds an M5 runtime-authority certification packet from frozen input.
    pub fn new(input: M5RuntimeAuthorityCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_RUNTIME_AUTHORITY_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            entries: input.entries,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Count of surfaces that keep their full claim.
    pub fn certified_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.verdict.is_certified())
            .count()
    }

    /// Count of surfaces that were auto-narrowed or failed closed.
    pub fn narrowed_count(&self) -> usize {
        self.entries.len() - self.certified_count()
    }

    /// Validates the M5 runtime-authority certification packet invariants.
    pub fn validate(&self) -> Vec<M5RuntimeAuthorityCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RUNTIME_AUTHORITY_CERTIFICATION_RECORD_KIND {
            violations.push(M5RuntimeAuthorityCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5RuntimeAuthorityCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RuntimeAuthorityCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        for entry in &self.entries {
            validate_entry(entry, &mut violations);
        }
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runtime-authority certification serializes"),
        ) {
            violations.push(M5RuntimeAuthorityCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 runtime-authority certification packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runtime-Authority Parity Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} certified, {} auto-narrowed)\n",
            self.entries.len(),
            self.certified_count(),
            self.narrowed_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Certified surfaces\n\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "- **{}** — verdict: {} — claim: {} → effective: {}\n",
                entry.surface.as_str(),
                entry.verdict.as_str(),
                entry.claimed_qualification.as_str(),
                entry.effective_qualification.as_str()
            ));
            out.push_str(&format!(
                "  - Isolation: {} · Ticket: {} · Secret scope: {}\n",
                entry.default_sandbox_profile.as_str(),
                entry.approval_ticket_posture.as_str(),
                entry.secret_scope.as_str()
            ));
            for proof in &entry.proofs {
                out.push_str(&format!(
                    "    - {}: {} ({})\n",
                    proof.dimension.as_str(),
                    proof.status.as_str(),
                    proof.proof_packet_id
                ));
            }
            if let Some(narrowed_to) = entry.narrowed_to {
                out.push_str(&format!(
                    "  - Narrowed to {} — {}\n",
                    narrowed_to.as_str(),
                    entry.recovery_action
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 runtime-authority certification export.
#[derive(Debug)]
pub enum M5RuntimeAuthorityCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RuntimeAuthorityCertificationViolation>),
}

impl fmt::Display for M5RuntimeAuthorityCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 runtime-authority certification export parse failed: {error}"
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
                    "m5 runtime-authority certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RuntimeAuthorityCertificationArtifactError {}

/// Validation failures emitted by [`M5RuntimeAuthorityCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RuntimeAuthorityCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A claimed M5 executing surface is never certified.
    SurfaceCoverageIncomplete,
    /// A surface is certified more than once.
    DuplicateSurface,
    /// An entry is missing required identity fields.
    EntryIncomplete,
    /// An entry does not cover all four parity dimensions exactly once.
    DimensionCoverageIncomplete,
    /// A proof binding omits required refs, packet id, timestamp, or note.
    ProofIncomplete,
    /// A proof binding references the wrong source packet for its dimension.
    ProofSourceMismatch,
    /// The verdict does not match the worst proof status across dimensions.
    VerdictInconsistentWithProofs,
    /// A certified entry carries a non-current dimension proof.
    CertifiedSurfaceCarriesUnprovenProof,
    /// A certified entry's effective qualification differs from its claim.
    CertifiedEffectiveQualificationDrift,
    /// A narrowed entry's effective qualification is wider than or equal to its claim.
    NarrowedQualificationWidened,
    /// A narrowed entry's effective qualification is off the verdict's floor.
    NarrowedQualificationOffFloor,
    /// A narrowed or failed entry omits its downgrade trigger.
    NarrowedEntryMissingTrigger,
    /// A narrowed or failed entry omits its narrowed fallback.
    NarrowedEntryMissingFallback,
    /// A narrowed or failed entry carries the wrong trigger or fallback.
    NarrowedEntryWrongDegradation,
    /// A certified entry carries a downgrade trigger or narrowed fallback.
    CertifiedEntryCarriesNarrowing,
    /// An unsupported-backend verdict does not fail closed.
    UnsupportedBackendNotFailClosed,
    /// An entry omits its recovery or status note.
    RecoveryActionMissing,
    /// A helper surface is certified without being flagged as a helper.
    HelperSurfaceFlagMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RuntimeAuthorityCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfaceCoverageIncomplete => "surface_coverage_incomplete",
            Self::DuplicateSurface => "duplicate_surface",
            Self::EntryIncomplete => "entry_incomplete",
            Self::DimensionCoverageIncomplete => "dimension_coverage_incomplete",
            Self::ProofIncomplete => "proof_incomplete",
            Self::ProofSourceMismatch => "proof_source_mismatch",
            Self::VerdictInconsistentWithProofs => "verdict_inconsistent_with_proofs",
            Self::CertifiedSurfaceCarriesUnprovenProof => {
                "certified_surface_carries_unproven_proof"
            }
            Self::CertifiedEffectiveQualificationDrift => "certified_effective_qualification_drift",
            Self::NarrowedQualificationWidened => "narrowed_qualification_widened",
            Self::NarrowedQualificationOffFloor => "narrowed_qualification_off_floor",
            Self::NarrowedEntryMissingTrigger => "narrowed_entry_missing_trigger",
            Self::NarrowedEntryMissingFallback => "narrowed_entry_missing_fallback",
            Self::NarrowedEntryWrongDegradation => "narrowed_entry_wrong_degradation",
            Self::CertifiedEntryCarriesNarrowing => "certified_entry_carries_narrowing",
            Self::UnsupportedBackendNotFailClosed => "unsupported_backend_not_fail_closed",
            Self::RecoveryActionMissing => "recovery_action_missing",
            Self::HelperSurfaceFlagMismatch => "helper_surface_flag_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen stable M5 runtime-authority certification packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_RUNTIME_AUTHORITY_CERTIFICATION_ARTIFACT_REF`]; the dumper emits this
/// packet and a test asserts the checked-in artifact deserializes back to it
/// unchanged. Every claimed surface in the frozen matrix is certified across all
/// four parity dimensions, so the canonical packet is fully `Certified`.
pub fn frozen_stable_m5_runtime_authority_certification_packet(
) -> M5RuntimeAuthorityCertificationPacket {
    build_certification_packet(
        M5_RUNTIME_AUTHORITY_CERTIFICATION_PACKET_ID,
        "M5 Runtime-Authority Parity Certification",
        certified_entries(),
    )
}

/// Builds a certification packet from a set of entries, applying the frozen
/// review, projection, freshness, and source-contract blocks.
///
/// Shared by the canonical packet and the checked fixtures so they cannot drift
/// in their trust posture.
pub fn build_certification_packet(
    packet_id: &str,
    packet_label: &str,
    entries: Vec<M5SurfaceCertificationEntry>,
) -> M5RuntimeAuthorityCertificationPacket {
    M5RuntimeAuthorityCertificationPacket::new(M5RuntimeAuthorityCertificationPacketInput {
        packet_id: packet_id.to_owned(),
        packet_label: packet_label.to_owned(),
        entries,
        trust_review: M5RuntimeAuthorityCertificationTrustReview {
            no_ambient_machine_privilege: true,
            no_self_issued_authority_by_helpers: true,
            every_surface_certified_across_all_dimensions: true,
            auto_narrows_on_missing_or_stale_proof: true,
            fail_closes_on_unsupported_backend: true,
            never_silently_widens_a_claim: true,
            secret_refs_handle_only_no_raw_material: true,
            certification_offline_verifiable: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5RuntimeAuthorityCertificationConsumerProjection {
            desktop_shows_certification_and_narrowing: true,
            command_and_policy_reference_same_certification: true,
            cli_headless_reads_certification_offline: true,
            help_about_ingests_certification_status: true,
            diagnostics_consumes_certification: true,
            release_evidence_gates_on_certification: true,
            support_export_shows_full_certification: true,
            remote_and_browser_preserve_certification_semantics: true,
        },
        proof_freshness: M5RuntimeAuthorityCertificationProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_CERTIFICATION_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF.to_owned(),
            M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF.to_owned(),
            M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF.to_owned(),
            M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF.to_owned(),
            M5_APPROVAL_TICKET_LEDGER_DOC_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_SCHEMA_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_DOC_REF.to_owned(),
            M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF.to_owned(),
            M5_CHILD_ENVELOPE_DERIVATION_DOC_REF.to_owned(),
            M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_REF.to_owned(),
            M5_AUTHORITY_LIFECYCLE_LEDGER_DOC_REF.to_owned(),
            M5_LAUNCH_INSPECTOR_SCHEMA_REF.to_owned(),
            M5_LAUNCH_INSPECTOR_DOC_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

/// The required source-contract refs every certification packet must carry.
fn required_source_contract_refs() -> [&'static str; 16] {
    [
        M5_RUNTIME_AUTHORITY_CERTIFICATION_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_CERTIFICATION_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
        M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
        M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF,
        M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF,
        M5_APPROVAL_TICKET_LEDGER_DOC_REF,
        M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
        M5_CAPABILITY_ENVELOPE_DOC_REF,
        M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF,
        M5_CHILD_ENVELOPE_DERIVATION_DOC_REF,
        M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_REF,
        M5_AUTHORITY_LIFECYCLE_LEDGER_DOC_REF,
        M5_LAUNCH_INSPECTOR_SCHEMA_REF,
        M5_LAUNCH_INSPECTOR_DOC_REF,
    ]
}

/// A fully certified entry for every claimed surface in the frozen matrix.
///
/// Each surface is certified across all four parity dimensions with current
/// proof, so every entry keeps its matrix-claimed qualification.
pub fn certified_entries() -> Vec<M5SurfaceCertificationEntry> {
    frozen_stable_m5_runtime_authority_matrix_packet()
        .surface_rows
        .iter()
        .map(certified_entry_from_row)
        .collect()
}

/// Builds a fully certified entry from one frozen matrix surface row.
fn certified_entry_from_row(
    row: &M5RuntimeAuthorityMatrixSurfaceRow,
) -> M5SurfaceCertificationEntry {
    let proofs = M5CertifiedAuthorityDimension::ALL
        .into_iter()
        .map(|dimension| proof_for(dimension, M5CertificationProofStatus::Current, row.surface))
        .collect();
    M5SurfaceCertificationEntry {
        surface: row.surface,
        claimed_qualification: row.qualification,
        default_sandbox_profile: row.default_sandbox_profile,
        approval_ticket_posture: row.approval_ticket_posture,
        secret_scope: row.secret_scope,
        is_untrusted_helper: row.surface.is_untrusted_helper(),
        proofs,
        verdict: M5CertificationVerdict::Certified,
        effective_qualification: row.qualification,
        narrowed_to: None,
        downgrade_trigger: None,
        recovery_action: format!(
            "No action needed; {} keeps its {} claim with current proof on every dimension.",
            row.surface.as_str(),
            row.qualification.as_str()
        ),
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

/// Builds one dimension proof binding at the given status for a surface.
fn proof_for(
    dimension: M5CertifiedAuthorityDimension,
    status: M5CertificationProofStatus,
    surface: M5ExecutingSurface,
) -> M5CertificationProof {
    let note = match status {
        M5CertificationProofStatus::Current => format!(
            "Current {} proof published for {}.",
            dimension.as_str(),
            surface.as_str()
        ),
        M5CertificationProofStatus::Stale => format!(
            "{} proof for {} is past the freshness window and must be refreshed.",
            dimension.as_str(),
            surface.as_str()
        ),
        M5CertificationProofStatus::Missing => format!(
            "No {} proof is published for {}; the claim is held pending fresh proof.",
            dimension.as_str(),
            surface.as_str()
        ),
        M5CertificationProofStatus::UnsupportedBackend => format!(
            "The {} enforcement backend is unsupported on this build for {}; the surface fails closed.",
            dimension.as_str(),
            surface.as_str()
        ),
    };
    let observed_at = match status {
        M5CertificationProofStatus::Current => "2026-06-10T00:00:00Z",
        M5CertificationProofStatus::Stale => "2026-05-01T00:00:00Z",
        // Missing / unsupported proofs are still timestamped at the certification
        // run so the absence itself is inspectable and dated.
        _ => "2026-06-10T00:00:00Z",
    };
    M5CertificationProof {
        dimension,
        status,
        proof_source_ref: dimension.source_artifact_ref().to_owned(),
        proof_schema_ref: dimension.source_schema_ref().to_owned(),
        proof_packet_id: dimension.source_packet_id().to_owned(),
        proof_observed_at: observed_at.to_owned(),
        note,
    }
}

/// Re-derives an entry's verdict, qualification, and degradation fields from a
/// set of per-dimension statuses.
///
/// This is the auto-narrowing gate applied to one surface: callers supply the
/// observed proof statuses and the certification recomputes everything that the
/// matrix claim must narrow to. Used by the fixtures to build narrowed entries
/// that stay internally coherent.
pub fn recertify_surface(
    base: &M5SurfaceCertificationEntry,
    statuses: [(M5CertifiedAuthorityDimension, M5CertificationProofStatus); 4],
) -> M5SurfaceCertificationEntry {
    let proofs = M5CertifiedAuthorityDimension::ALL
        .into_iter()
        .map(|dimension| {
            let status = statuses
                .iter()
                .find(|(dim, _)| *dim == dimension)
                .map(|(_, status)| *status)
                .unwrap_or(M5CertificationProofStatus::Current);
            proof_for(dimension, status, base.surface)
        })
        .collect::<Vec<_>>();
    let verdict = certification_verdict_for(proofs.iter().map(|proof| proof.status));
    let effective_qualification = verdict
        .narrowed_floor()
        .unwrap_or(base.claimed_qualification);
    let recovery_action = if verdict.is_certified() {
        format!(
            "No action needed; {} keeps its {} claim with current proof on every dimension.",
            base.surface.as_str(),
            base.claimed_qualification.as_str()
        )
    } else {
        format!(
            "Refresh or restore the failing proof to recertify {}; until then it is narrowed to {}.",
            base.surface.as_str(),
            effective_qualification.as_str()
        )
    };
    M5SurfaceCertificationEntry {
        surface: base.surface,
        claimed_qualification: base.claimed_qualification,
        default_sandbox_profile: base.default_sandbox_profile,
        approval_ticket_posture: base.approval_ticket_posture,
        secret_scope: base.secret_scope,
        is_untrusted_helper: base.is_untrusted_helper,
        proofs,
        verdict,
        effective_qualification,
        narrowed_to: verdict.narrowed_to(),
        downgrade_trigger: verdict.downgrade_trigger(),
        recovery_action,
        redaction_class_token: base.redaction_class_token.clone(),
    }
}

/// Certified entries with one surface auto-narrowed by a missing envelope proof.
///
/// Not part of the canonical packet; used by checked fixtures to exercise the
/// missing-proof auto-narrowing gate.
pub fn entries_with_missing_proof_surface() -> Vec<M5SurfaceCertificationEntry> {
    narrow_one_surface(
        M5ExecutingSurface::RemoteMutation,
        [
            (
                M5CertifiedAuthorityDimension::ExecutionIsolation,
                M5CertificationProofStatus::Current,
            ),
            (
                M5CertifiedAuthorityDimension::ApprovalTicketIntegrity,
                M5CertificationProofStatus::Current,
            ),
            (
                M5CertifiedAuthorityDimension::CapabilityEnvelopeParity,
                M5CertificationProofStatus::Missing,
            ),
            (
                M5CertifiedAuthorityDimension::RuntimeAuthorityLineage,
                M5CertificationProofStatus::Current,
            ),
        ],
    )
}

/// Certified entries with one surface auto-narrowed by a stale ticket proof.
///
/// Not part of the canonical packet; used by checked fixtures.
pub fn entries_with_stale_proof_surface() -> Vec<M5SurfaceCertificationEntry> {
    narrow_one_surface(
        M5ExecutingSurface::AiTool,
        [
            (
                M5CertifiedAuthorityDimension::ExecutionIsolation,
                M5CertificationProofStatus::Current,
            ),
            (
                M5CertifiedAuthorityDimension::ApprovalTicketIntegrity,
                M5CertificationProofStatus::Stale,
            ),
            (
                M5CertifiedAuthorityDimension::CapabilityEnvelopeParity,
                M5CertificationProofStatus::Current,
            ),
            (
                M5CertifiedAuthorityDimension::RuntimeAuthorityLineage,
                M5CertificationProofStatus::Current,
            ),
        ],
    )
}

/// Certified entries with one surface failed closed by an unsupported backend.
///
/// Not part of the canonical packet; used by checked fixtures to exercise the
/// fail-closed gate when an enforcement backend is missing.
pub fn entries_with_unsupported_backend_surface() -> Vec<M5SurfaceCertificationEntry> {
    narrow_one_surface(
        M5ExecutingSurface::NotebookKernel,
        [
            (
                M5CertifiedAuthorityDimension::ExecutionIsolation,
                M5CertificationProofStatus::UnsupportedBackend,
            ),
            (
                M5CertifiedAuthorityDimension::ApprovalTicketIntegrity,
                M5CertificationProofStatus::Current,
            ),
            (
                M5CertifiedAuthorityDimension::CapabilityEnvelopeParity,
                M5CertificationProofStatus::Current,
            ),
            (
                M5CertifiedAuthorityDimension::RuntimeAuthorityLineage,
                M5CertificationProofStatus::Current,
            ),
        ],
    )
}

/// Replaces one surface's certified entry with a recertified (narrowed) one.
fn narrow_one_surface(
    surface: M5ExecutingSurface,
    statuses: [(M5CertifiedAuthorityDimension, M5CertificationProofStatus); 4],
) -> Vec<M5SurfaceCertificationEntry> {
    certified_entries()
        .into_iter()
        .map(|entry| {
            if entry.surface == surface {
                recertify_surface(&entry, statuses)
            } else {
                entry
            }
        })
        .collect()
}

fn validate_source_contracts(
    packet: &M5RuntimeAuthorityCertificationPacket,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in required_source_contract_refs() {
        if !refs.contains(required) {
            violations.push(M5RuntimeAuthorityCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_coverage(
    packet: &M5RuntimeAuthorityCertificationPacket,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    let mut seen: BTreeSet<M5ExecutingSurface> = BTreeSet::new();
    for entry in &packet.entries {
        if !seen.insert(entry.surface) {
            violations.push(M5RuntimeAuthorityCertificationViolation::DuplicateSurface);
        }
    }
    for required in M5ExecutingSurface::ALL {
        if !seen.contains(&required) {
            violations.push(M5RuntimeAuthorityCertificationViolation::SurfaceCoverageIncomplete);
            break;
        }
    }
}

fn validate_entry(
    entry: &M5SurfaceCertificationEntry,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    if entry.redaction_class_token.trim().is_empty() {
        violations.push(M5RuntimeAuthorityCertificationViolation::EntryIncomplete);
    }
    if entry.recovery_action.trim().is_empty() {
        violations.push(M5RuntimeAuthorityCertificationViolation::RecoveryActionMissing);
    }
    if entry.is_untrusted_helper != entry.surface.is_untrusted_helper() {
        violations.push(M5RuntimeAuthorityCertificationViolation::HelperSurfaceFlagMismatch);
    }

    validate_entry_dimensions(entry, violations);
    validate_entry_verdict(entry, violations);
}

fn validate_entry_dimensions(
    entry: &M5SurfaceCertificationEntry,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    let mut dimensions_seen: BTreeSet<M5CertifiedAuthorityDimension> = BTreeSet::new();
    for proof in &entry.proofs {
        dimensions_seen.insert(proof.dimension);
        if proof.proof_source_ref.trim().is_empty()
            || proof.proof_schema_ref.trim().is_empty()
            || proof.proof_packet_id.trim().is_empty()
            || proof.proof_observed_at.trim().is_empty()
            || proof.note.trim().is_empty()
        {
            violations.push(M5RuntimeAuthorityCertificationViolation::ProofIncomplete);
        }
        if proof.proof_source_ref != proof.dimension.source_artifact_ref()
            || proof.proof_schema_ref != proof.dimension.source_schema_ref()
            || proof.proof_packet_id != proof.dimension.source_packet_id()
        {
            violations.push(M5RuntimeAuthorityCertificationViolation::ProofSourceMismatch);
        }
    }
    if dimensions_seen.len() != M5CertifiedAuthorityDimension::ALL.len() {
        violations.push(M5RuntimeAuthorityCertificationViolation::DimensionCoverageIncomplete);
    }
}

fn validate_entry_verdict(
    entry: &M5SurfaceCertificationEntry,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    // The verdict must equal the worst proof status across the dimensions: this
    // is the auto-narrowing gate, and a mismatch is the silent-widening defect.
    let expected_verdict = certification_verdict_for(entry.proofs.iter().map(|proof| proof.status));
    if entry.verdict != expected_verdict {
        violations.push(M5RuntimeAuthorityCertificationViolation::VerdictInconsistentWithProofs);
    }

    let all_current = entry.proofs.iter().all(|proof| proof.status.is_current());
    if entry.verdict.is_certified() {
        if !all_current {
            violations.push(
                M5RuntimeAuthorityCertificationViolation::CertifiedSurfaceCarriesUnprovenProof,
            );
        }
        if entry.effective_qualification != entry.claimed_qualification {
            violations.push(
                M5RuntimeAuthorityCertificationViolation::CertifiedEffectiveQualificationDrift,
            );
        }
        if entry.narrowed_to.is_some() || entry.downgrade_trigger.is_some() {
            violations
                .push(M5RuntimeAuthorityCertificationViolation::CertifiedEntryCarriesNarrowing);
        }
    } else {
        // A narrowed verdict must land strictly more restricted than the claim
        // (never wider) and on the verdict's qualification floor.
        if qualification_restriction_rank(entry.effective_qualification)
            <= qualification_restriction_rank(entry.claimed_qualification)
        {
            violations.push(M5RuntimeAuthorityCertificationViolation::NarrowedQualificationWidened);
        }
        if Some(entry.effective_qualification) != entry.verdict.narrowed_floor() {
            violations
                .push(M5RuntimeAuthorityCertificationViolation::NarrowedQualificationOffFloor);
        }
        if entry.downgrade_trigger.is_none() {
            violations.push(M5RuntimeAuthorityCertificationViolation::NarrowedEntryMissingTrigger);
        } else if entry.downgrade_trigger != entry.verdict.downgrade_trigger() {
            violations
                .push(M5RuntimeAuthorityCertificationViolation::NarrowedEntryWrongDegradation);
        }
        if entry.narrowed_to.is_none() {
            violations.push(M5RuntimeAuthorityCertificationViolation::NarrowedEntryMissingFallback);
        } else if entry.narrowed_to != entry.verdict.narrowed_to() {
            violations
                .push(M5RuntimeAuthorityCertificationViolation::NarrowedEntryWrongDegradation);
        }
        if entry.verdict == M5CertificationVerdict::FailedClosedUnsupportedBackend
            && (entry.narrowed_to != Some(M5DegradedFallback::FailClosedBlock)
                || entry.effective_qualification
                    != M5RuntimeAuthorityQualificationClass::Unavailable)
        {
            violations
                .push(M5RuntimeAuthorityCertificationViolation::UnsupportedBackendNotFailClosed);
        }
    }
}

fn validate_trust_review(
    packet: &M5RuntimeAuthorityCertificationPacket,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_self_issued_authority_by_helpers,
        review.every_surface_certified_across_all_dimensions,
        review.auto_narrows_on_missing_or_stale_proof,
        review.fail_closes_on_unsupported_backend,
        review.never_silently_widens_a_claim,
        review.secret_refs_handle_only_no_raw_material,
        review.certification_offline_verifiable,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5RuntimeAuthorityCertificationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RuntimeAuthorityCertificationPacket,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_certification_and_narrowing,
        projection.command_and_policy_reference_same_certification,
        projection.cli_headless_reads_certification_offline,
        projection.help_about_ingests_certification_status,
        projection.diagnostics_consumes_certification,
        projection.release_evidence_gates_on_certification,
        projection.support_export_shows_full_certification,
        projection.remote_and_browser_preserve_certification_semantics,
    ] {
        if !ok {
            violations.push(M5RuntimeAuthorityCertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RuntimeAuthorityCertificationPacket,
    violations: &mut Vec<M5RuntimeAuthorityCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RuntimeAuthorityCertificationViolation::ProofFreshnessIncomplete);
    }
}

/// Reads and validates the checked-in stable M5 runtime-authority certification export.
pub fn current_stable_m5_runtime_authority_certification_export(
) -> Result<M5RuntimeAuthorityCertificationPacket, M5RuntimeAuthorityCertificationArtifactError> {
    let packet: M5RuntimeAuthorityCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/support_export.json"
    )))
    .map_err(M5RuntimeAuthorityCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RuntimeAuthorityCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
