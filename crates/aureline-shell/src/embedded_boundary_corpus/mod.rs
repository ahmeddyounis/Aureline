//! Embedded-surface boundary audit corpus.
//!
//! Builds on the beta embedded-surface boundary audit in
//! [`crate::embedded_boundary_audit`] and promotes it from a single
//! conformant page into a regression-gated proof corpus. Every claimed
//! embedded beta surface (docs/help panes, marketplace/account pages,
//! provider-owned webviews, service dashboards, and auth-handoff
//! sheets) gets a worked drill that proves the host shell keeps the
//! owner/origin chrome, system-browser-first identity preference,
//! certificate/policy failure honesty, cross-origin limitation
//! messaging, stale-snapshot truth, open-in-browser fallback truth, and
//! native approval fences that an embedded body must never be allowed to
//! spoof, flatten, bypass, or widen.
//!
//! ## Two kinds of drill
//!
//! 1. **Boundary drills** are conformant rows. They prove the audited
//!    surface presents its boundary honestly: the
//!    [`audit_rows`](crate::embedded_boundary_audit::audit_rows)
//!    validator emits zero defects, the open-in-browser fallback keeps
//!    its target and reason, the fallback path never widens authority
//!    past the product-owned command, and a high-risk approval fence
//!    survives an app restart and a surface re-entry.
//! 2. **Denial drills** are adversarial rows that an embedded body could
//!    try to ship. Each one MUST be rejected: dropping owner/origin
//!    disclosure, masking a stale or certificate-failed origin as live,
//!    impersonating native trust/update chrome, hosting a high-risk
//!    approval, widening authority through the browser fallback,
//!    collecting a password without a registered exception, flattening
//!    the support export, or opening into the browser without keeping the
//!    target and reason. The corpus records the exact denial-reason
//!    tokens the gate produces so a regression that lets one through
//!    fails the fixture replay.
//!
//! The boundary-vocabulary schema of record stays
//! `schemas/ux/embedded_surface_boundary.schema.json`; this corpus reuses
//! the beta audit row/validator types so a drill cannot drift from the
//! shipped audit lane.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::embedded::boundary_card::SurfaceFamily;
use crate::embedded_boundary_audit::{
    audit_rows, seeded_embedded_boundary_audit_page, EmbeddedBoundaryAuditPage,
    EmbeddedBoundaryAuditRow, EmbeddedBoundaryAuditSupportRow,
    EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION, EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF,
    EMBEDDED_BOUNDARY_AUDIT_BETA_SUPPORT_ROW_RECORD_KIND,
};

/// Schema version exported with every corpus record.
pub const EMBEDDED_BOUNDARY_CORPUS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by fixtures, docs, and support exports.
pub const EMBEDDED_BOUNDARY_CORPUS_SHARED_CONTRACT_REF: &str = "shell:embedded_boundary_corpus:v1";

/// Stable record kind for [`EmbeddedBoundaryCorpusPacket`].
pub const EMBEDDED_BOUNDARY_CORPUS_PACKET_RECORD_KIND: &str =
    "shell_embedded_boundary_corpus_packet_record";

/// Stable record kind for [`EmbeddedBoundaryCorpusCase`].
pub const EMBEDDED_BOUNDARY_CORPUS_CASE_RECORD_KIND: &str =
    "shell_embedded_boundary_corpus_case_record";

/// Stable record kind for [`EmbeddedBoundaryCorpusSupportExport`].
pub const EMBEDDED_BOUNDARY_CORPUS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_embedded_boundary_corpus_support_export_record";

/// Stable record kind for [`EmbeddedBoundaryCorpusSupportRow`].
pub const EMBEDDED_BOUNDARY_CORPUS_SUPPORT_ROW_RECORD_KIND: &str =
    "shell_embedded_boundary_corpus_support_row_record";

/// Stable record kind for [`EmbeddedBoundaryCorpusMatrix`].
pub const EMBEDDED_BOUNDARY_CORPUS_MATRIX_RECORD_KIND: &str =
    "shell_embedded_boundary_corpus_matrix_record";

/// Stable packet id used by every consumer.
pub const EMBEDDED_BOUNDARY_CORPUS_PACKET_ID: &str =
    "shell:embedded_boundary_corpus:packet:default";

/// Deterministic packet timestamp.
pub const EMBEDDED_BOUNDARY_CORPUS_GENERATED_AT: &str = "2026-05-20T00:00:00Z";

// ---------------------------------------------------------------------------
// Coverage vocabularies
// ---------------------------------------------------------------------------

/// Closed boundary-drill vocabulary. The validator refuses a corpus that
/// does not exercise every variant in at least one conformant case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryCorpusBoundaryDrill {
    /// Owner, publisher/service, and origin are disclosed and verified.
    OwnerOriginVerified,
    /// Auth handoff prefers the system browser; nothing is typed inside
    /// the embedded card.
    SystemBrowserFirstAuth,
    /// TLS/certificate verification failed; the surface says so in
    /// product instead of rendering a broken page.
    CertificateFailure,
    /// Managed-workspace policy disables the embedded render; the surface
    /// names the policy instead of blanking.
    ManagedPolicyDeny,
    /// A cross-origin subframe is inspect-only; the limitation is named
    /// instead of presented as a silent failure.
    CrossOriginLimitation,
    /// The surface is showing a stale in-product snapshot and says so.
    StaleSnapshot,
    /// The surface is offline and external open is unavailable; the
    /// fallback posture is honest.
    OfflineSnapshot,
    /// Open-in-browser keeps the return target and the reason intact.
    OpenInBrowserFallback,
    /// Device-code is offered as the auditable auth fallback.
    DeviceCodeFallback,
    /// A high-risk approval fence stays host-native across an app
    /// restart.
    NativeApprovalFencePersistsRestart,
    /// A high-risk approval fence stays host-native across a surface
    /// re-entry.
    NativeApprovalFencePersistsReentry,
}

impl EmbeddedBoundaryCorpusBoundaryDrill {
    /// Stable token used in fixtures, packets, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginVerified => "owner_origin_verified",
            Self::SystemBrowserFirstAuth => "system_browser_first_auth",
            Self::CertificateFailure => "certificate_failure",
            Self::ManagedPolicyDeny => "managed_policy_deny",
            Self::CrossOriginLimitation => "cross_origin_limitation",
            Self::StaleSnapshot => "stale_snapshot",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::OpenInBrowserFallback => "open_in_browser_fallback",
            Self::DeviceCodeFallback => "device_code_fallback",
            Self::NativeApprovalFencePersistsRestart => "native_approval_fence_persists_restart",
            Self::NativeApprovalFencePersistsReentry => "native_approval_fence_persists_reentry",
        }
    }

    /// Every boundary drill the validator enforces presence of.
    pub const fn all() -> [Self; 11] {
        [
            Self::OwnerOriginVerified,
            Self::SystemBrowserFirstAuth,
            Self::CertificateFailure,
            Self::ManagedPolicyDeny,
            Self::CrossOriginLimitation,
            Self::StaleSnapshot,
            Self::OfflineSnapshot,
            Self::OpenInBrowserFallback,
            Self::DeviceCodeFallback,
            Self::NativeApprovalFencePersistsRestart,
            Self::NativeApprovalFencePersistsReentry,
        ]
    }
}

/// Closed denial-drill vocabulary. The validator refuses a corpus that
/// does not exercise every adversarial behavior in at least one case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryCorpusDenial {
    /// The embedded body dropped owner/origin disclosure.
    OwnerOriginSpoof,
    /// A stale or certificate-failed origin was painted as live.
    StaleMaskedAsLive,
    /// The embedded body tried to impersonate native trust/update chrome.
    NativeTrustChromeSpoof,
    /// The embedded body tried to host a high-risk approval surface.
    ApprovalBypass,
    /// The browser/reopen fallback granted broader authority than the
    /// product-owned native command.
    AuthorityWidening,
    /// The embedded body collected a password without a registered
    /// lower-trust exception.
    EmbeddedPasswordCollection,
    /// The support export flattened the boundary labels.
    SupportExportFlattening,
    /// Open-in-browser dropped the return target or the reason.
    BrowserFallbackDropsTargetOrReason,
}

impl EmbeddedBoundaryCorpusDenial {
    /// Stable token used in fixtures, packets, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOriginSpoof => "owner_origin_spoof",
            Self::StaleMaskedAsLive => "stale_masked_as_live",
            Self::NativeTrustChromeSpoof => "native_trust_chrome_spoof",
            Self::ApprovalBypass => "approval_bypass",
            Self::AuthorityWidening => "authority_widening",
            Self::EmbeddedPasswordCollection => "embedded_password_collection",
            Self::SupportExportFlattening => "support_export_flattening",
            Self::BrowserFallbackDropsTargetOrReason => "browser_fallback_drops_target_or_reason",
        }
    }

    /// Every denial drill the validator enforces presence of.
    pub const fn all() -> [Self; 8] {
        [
            Self::OwnerOriginSpoof,
            Self::StaleMaskedAsLive,
            Self::NativeTrustChromeSpoof,
            Self::ApprovalBypass,
            Self::AuthorityWidening,
            Self::EmbeddedPasswordCollection,
            Self::SupportExportFlattening,
            Self::BrowserFallbackDropsTargetOrReason,
        ]
    }
}

/// Corpus-level denial reasons that the per-row audit validator does not
/// own. The audit validator owns field/vocabulary defects; these three
/// own the fallback-truth, authority, and lifecycle-persistence proofs
/// the corpus adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryCorpusDenialReason {
    /// The fallback/reopen path grants more in-product authority than the
    /// product-owned native command.
    FallbackWidensAuthorityBeyondNative,
    /// Open-in-browser dropped the return-target or the reason truth.
    OpenInBrowserDropsTargetOrReason,
    /// A high-risk approval fence was not preserved across a lifecycle
    /// transition (restart or re-entry).
    NativeApprovalFenceNotPersisted,
}

impl EmbeddedBoundaryCorpusDenialReason {
    /// Stable token recorded in `actual_denial_reason_tokens`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FallbackWidensAuthorityBeyondNative => "fallback_widens_authority_beyond_native",
            Self::OpenInBrowserDropsTargetOrReason => "open_in_browser_drops_target_or_reason",
            Self::NativeApprovalFenceNotPersisted => "native_approval_fence_not_persisted",
        }
    }
}

// ---------------------------------------------------------------------------
// Authority ranking
// ---------------------------------------------------------------------------

/// Closed in-product authority class for the authority non-widening
/// proof. Higher rank means broader in-product authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedAuthorityClass {
    /// No in-product authority (e.g. a system-browser handoff that leaves
    /// the product boundary entirely).
    NoAuthorityWithinProduct,
    /// Inspect-only authority within the product.
    HostOwnedInspectOnly,
    /// Copy/export-only authority within the product.
    HostOwnedCopyExportOnly,
    /// Browser-only authority — the product can only hand off.
    HostOwnedBrowserOnly,
    /// Host-owned, with a native step-up gating any high-risk approval.
    HostOwnedWithNativeStepUp,
    /// Host-owned full authority within the surface's scope.
    HostOwnedFullAuthority,
}

impl EmbeddedAuthorityClass {
    /// Stable token used in fixtures, packets, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuthorityWithinProduct => "no_authority_within_product",
            Self::HostOwnedInspectOnly => "host_owned_inspect_only",
            Self::HostOwnedCopyExportOnly => "host_owned_copy_export_only",
            Self::HostOwnedBrowserOnly => "host_owned_browser_only",
            Self::HostOwnedWithNativeStepUp => "host_owned_with_native_step_up",
            Self::HostOwnedFullAuthority => "host_owned_full_authority",
        }
    }

    /// Monotonic authority rank. A fallback path may not exceed the
    /// native path's rank.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NoAuthorityWithinProduct => 0,
            Self::HostOwnedInspectOnly => 1,
            Self::HostOwnedCopyExportOnly => 1,
            Self::HostOwnedBrowserOnly => 2,
            Self::HostOwnedWithNativeStepUp => 3,
            Self::HostOwnedFullAuthority => 4,
        }
    }
}

// ---------------------------------------------------------------------------
// Proof sub-records
// ---------------------------------------------------------------------------

/// One lifecycle snapshot captured during an approval-fence persistence
/// drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryLifecyclePhase {
    /// Stable phase token (`initial_render`, `after_app_restart`,
    /// `after_surface_reentry`).
    pub phase_token: String,
    /// Native-reserved surfaces still host-owned at this phase.
    pub native_reserved_surface_tokens: Vec<String>,
    /// Permission class at this phase.
    pub permission_class_token: String,
    /// High-risk action posture at this phase (`host_native_only`).
    pub high_risk_action_posture_token: String,
}

/// Lifecycle persistence proof for a high-risk approval fence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryLifecyclePersistence {
    /// Ordered phases. The fence must be identical across every phase.
    pub phases: Vec<EmbeddedBoundaryLifecyclePhase>,
}

impl EmbeddedBoundaryLifecyclePersistence {
    /// True when the native-reserved fence, permission class, and
    /// host-native posture are identical and intact across every phase.
    fn fence_persists(&self) -> bool {
        let mut iter = self.phases.iter();
        let Some(first) = iter.next() else {
            return false;
        };
        if first.high_risk_action_posture_token != "host_native_only" {
            return false;
        }
        let baseline: BTreeSet<&str> = first
            .native_reserved_surface_tokens
            .iter()
            .map(String::as_str)
            .collect();
        if baseline.len() != REQUIRED_NATIVE_RESERVED_SURFACE_TOKENS.len() {
            return false;
        }
        for phase in &self.phases {
            if phase.high_risk_action_posture_token != "host_native_only" {
                return false;
            }
            if phase.permission_class_token != first.permission_class_token {
                return false;
            }
            let phase_set: BTreeSet<&str> = phase
                .native_reserved_surface_tokens
                .iter()
                .map(String::as_str)
                .collect();
            if phase_set != baseline {
                return false;
            }
        }
        true
    }
}

/// Open-in-browser fallback truth proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryFallbackTruth {
    /// Host-owned action that performs the fallback.
    pub action_id_token: String,
    /// Return target the user lands on. Must be non-empty.
    pub return_target_label: String,
    /// Reason the fallback is being offered. Must be non-empty.
    pub reason_label: String,
    /// The fallback opens the same object, not a generic preview.
    pub preserves_object_identity: bool,
    /// Browser handoff packet ref, if a system-browser handoff is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
}

impl EmbeddedBoundaryFallbackTruth {
    /// True when the fallback keeps the target and the reason intact.
    fn keeps_target_and_reason(&self) -> bool {
        !self.return_target_label.trim().is_empty()
            && !self.reason_label.trim().is_empty()
            && self.preserves_object_identity
    }
}

// ---------------------------------------------------------------------------
// Corpus case
// ---------------------------------------------------------------------------

/// Expected verdict for a corpus case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedBoundaryCorpusExpectation {
    /// The audited surface is honest; the gate must emit zero denials.
    Conformant,
    /// The audited surface is adversarial; the gate must deny it.
    Denied,
}

impl EmbeddedBoundaryCorpusExpectation {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::Denied => "denied",
        }
    }
}

/// One worked drill: an audited row plus the proof fields the corpus
/// validates and the verdict the gate produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusCase {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable case id.
    pub case_id: String,
    /// Human-readable case label.
    pub case_label: String,
    /// `boundary_drill` or `denial_drill`.
    pub case_kind_token: String,
    /// Surface family under audit.
    pub surface_family: SurfaceFamily,
    /// Boundary drills this case evidences (empty for denial drills).
    pub boundary_drills: Vec<EmbeddedBoundaryCorpusBoundaryDrill>,
    /// Denial drill this case evidences (None for boundary drills).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial: Option<EmbeddedBoundaryCorpusDenial>,
    /// Expected verdict.
    pub expectation: EmbeddedBoundaryCorpusExpectation,
    /// Denial-reason tokens this case must produce when `Denied`.
    pub expected_denial_reason_tokens: Vec<String>,
    /// The audited row under test.
    pub row: EmbeddedBoundaryAuditRow,
    /// Support row paired 1:1 with the audited row.
    pub support_row: EmbeddedBoundaryAuditSupportRow,
    /// Authority of the product-owned native command.
    pub native_path_authority_token: String,
    /// Authority the fallback/reopen path confers within the product.
    pub fallback_path_authority_token: String,
    /// Open-in-browser fallback truth proof, when the surface offers it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_fallback_truth: Option<EmbeddedBoundaryFallbackTruth>,
    /// Lifecycle persistence proof, for approval-fence drills.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_persistence: Option<EmbeddedBoundaryLifecyclePersistence>,
    /// Denial-reason tokens the gate actually produced.
    pub actual_denial_reason_tokens: Vec<String>,
    /// True when the actual outcome matched the expectation.
    pub verdict_holds: bool,
    /// Plain-language summary for review.
    pub plain_language_summary: String,
}

/// Runs the per-row audit validator plus the three corpus-level proofs
/// and returns the deduped, sorted denial-reason tokens.
fn evaluate_denial_reasons(case: &PartialCase) -> Vec<String> {
    let mut tokens: BTreeSet<String> = BTreeSet::new();

    for defect in audit_rows(
        std::slice::from_ref(&case.row),
        std::slice::from_ref(&case.support_row),
    ) {
        tokens.insert(defect.defect_kind_token.clone());
    }

    if case.fallback_path_authority.rank() > case.native_path_authority.rank() {
        tokens.insert(
            EmbeddedBoundaryCorpusDenialReason::FallbackWidensAuthorityBeyondNative
                .as_str()
                .to_string(),
        );
    }

    if let Some(truth) = &case.browser_fallback_truth {
        if !truth.keeps_target_and_reason() {
            tokens.insert(
                EmbeddedBoundaryCorpusDenialReason::OpenInBrowserDropsTargetOrReason
                    .as_str()
                    .to_string(),
            );
        }
    }

    if let Some(lifecycle) = &case.lifecycle_persistence {
        if !lifecycle.fence_persists() {
            tokens.insert(
                EmbeddedBoundaryCorpusDenialReason::NativeApprovalFenceNotPersisted
                    .as_str()
                    .to_string(),
            );
        }
    }

    tokens.into_iter().collect()
}

/// Intermediate builder shape before the verdict is computed.
struct PartialCase {
    case_id: String,
    case_label: String,
    surface_family: SurfaceFamily,
    boundary_drills: Vec<EmbeddedBoundaryCorpusBoundaryDrill>,
    denial: Option<EmbeddedBoundaryCorpusDenial>,
    expectation: EmbeddedBoundaryCorpusExpectation,
    expected_denial_reason_tokens: Vec<String>,
    row: EmbeddedBoundaryAuditRow,
    support_row: EmbeddedBoundaryAuditSupportRow,
    native_path_authority: EmbeddedAuthorityClass,
    fallback_path_authority: EmbeddedAuthorityClass,
    browser_fallback_truth: Option<EmbeddedBoundaryFallbackTruth>,
    lifecycle_persistence: Option<EmbeddedBoundaryLifecyclePersistence>,
    plain_language_summary: String,
}

impl PartialCase {
    fn finish(self) -> EmbeddedBoundaryCorpusCase {
        let actual = evaluate_denial_reasons(&self);
        let mut expected = self.expected_denial_reason_tokens.clone();
        expected.sort();
        expected.dedup();
        let verdict_holds = match self.expectation {
            EmbeddedBoundaryCorpusExpectation::Conformant => actual.is_empty(),
            EmbeddedBoundaryCorpusExpectation::Denied => {
                !actual.is_empty() && expected.iter().all(|token| actual.contains(token))
            }
        };
        let case_kind_token = if self.denial.is_some() {
            "denial_drill"
        } else {
            "boundary_drill"
        };
        EmbeddedBoundaryCorpusCase {
            record_kind: EMBEDDED_BOUNDARY_CORPUS_CASE_RECORD_KIND.to_string(),
            schema_version: EMBEDDED_BOUNDARY_CORPUS_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_CORPUS_SHARED_CONTRACT_REF.to_string(),
            case_id: self.case_id,
            case_label: self.case_label,
            case_kind_token: case_kind_token.to_string(),
            surface_family: self.surface_family,
            boundary_drills: self.boundary_drills,
            denial: self.denial,
            expectation: self.expectation,
            expected_denial_reason_tokens: expected,
            row: self.row,
            support_row: self.support_row,
            native_path_authority_token: self.native_path_authority.as_str().to_string(),
            fallback_path_authority_token: self.fallback_path_authority.as_str().to_string(),
            browser_fallback_truth: self.browser_fallback_truth,
            lifecycle_persistence: self.lifecycle_persistence,
            actual_denial_reason_tokens: actual,
            verdict_holds,
            plain_language_summary: self.plain_language_summary,
        }
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Export-safe support row: enough to reconstruct the surface owner,
/// boundary state, and approval origin without scraping transient text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusSupportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Case id this support row mirrors.
    pub case_id: String,
    /// Surface family token.
    pub surface_family_token: String,
    /// Owner label.
    pub owner_label: String,
    /// Owner class token.
    pub owner_class_token: String,
    /// Origin host or domain label.
    pub host_or_domain_label: String,
    /// Origin verification token.
    pub origin_verification_token: String,
    /// Boundary state token.
    pub boundary_state_token: String,
    /// Permission class token (the approval origin).
    pub permission_class_token: String,
    /// Browser fallback posture token.
    pub browser_fallback_posture_token: String,
    /// Fallback target class token.
    pub fallback_target_class_token: String,
    /// Native-path authority token.
    pub native_path_authority_token: String,
    /// Fallback-path authority token.
    pub fallback_path_authority_token: String,
    /// Expectation token.
    pub expectation_token: String,
    /// Denial-reason tokens the gate produced.
    pub denial_reason_tokens: Vec<String>,
    /// Whether the recorded verdict holds.
    pub verdict_holds: bool,
    /// Redaction class token.
    pub redaction_class_token: String,
}

/// Support export projection for the whole corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support export id.
    pub support_export_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// One row per corpus case.
    pub rows: Vec<EmbeddedBoundaryCorpusSupportRow>,
    /// Denial-reason token counts across the whole corpus.
    pub denial_reason_counts: BTreeMap<String, usize>,
    /// True — this export carries no raw row payload or secret literal.
    pub no_sensitive_payload: bool,
}

impl EmbeddedBoundaryCorpusSupportExport {
    fn from_cases(cases: &[EmbeddedBoundaryCorpusCase]) -> Self {
        let mut rows = Vec::with_capacity(cases.len());
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for case in cases {
            for token in &case.actual_denial_reason_tokens {
                *counts.entry(token.clone()).or_insert(0) += 1;
            }
            rows.push(EmbeddedBoundaryCorpusSupportRow {
                record_kind: EMBEDDED_BOUNDARY_CORPUS_SUPPORT_ROW_RECORD_KIND.to_string(),
                case_id: case.case_id.clone(),
                surface_family_token: case.row.surface_family_token.clone(),
                owner_label: case.row.owner_label.clone(),
                owner_class_token: case.row.owner_class_token.clone(),
                host_or_domain_label: case.row.host_or_domain_label.clone(),
                origin_verification_token: case.row.origin_verification_token.clone(),
                boundary_state_token: case.row.boundary_state_token.clone(),
                permission_class_token: case.row.permission_class_token.clone(),
                browser_fallback_posture_token: case.row.browser_fallback_posture_token.clone(),
                fallback_target_class_token: case.row.fallback_target_class_token.clone(),
                native_path_authority_token: case.native_path_authority_token.clone(),
                fallback_path_authority_token: case.fallback_path_authority_token.clone(),
                expectation_token: case.expectation.as_str().to_string(),
                denial_reason_tokens: case.actual_denial_reason_tokens.clone(),
                verdict_holds: case.verdict_holds,
                redaction_class_token: case.row.redaction_class_token.clone(),
            });
        }
        Self {
            record_kind: EMBEDDED_BOUNDARY_CORPUS_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: EMBEDDED_BOUNDARY_CORPUS_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_CORPUS_SHARED_CONTRACT_REF.to_string(),
            support_export_id: "shell:embedded_boundary_corpus:support_export:default".to_string(),
            generated_at: EMBEDDED_BOUNDARY_CORPUS_GENERATED_AT.to_string(),
            rows,
            denial_reason_counts: counts,
            no_sensitive_payload: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Matrix
// ---------------------------------------------------------------------------

/// One matrix row, per surface family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusMatrixRow {
    /// Surface family.
    pub surface_family: SurfaceFamily,
    /// Surface family token.
    pub surface_family_token: String,
    /// Boundary drills exercised by this family.
    pub boundary_drills_exercised: Vec<EmbeddedBoundaryCorpusBoundaryDrill>,
    /// Denial drills exercised by this family.
    pub denials_exercised: Vec<EmbeddedBoundaryCorpusDenial>,
    /// Boundary-state tokens present in this family.
    pub boundary_state_tokens: Vec<String>,
    /// Conformant case count.
    pub conformant_case_count: usize,
    /// Denial case count.
    pub denial_case_count: usize,
}

/// Coverage matrix projection consumed by the matrix.json artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusMatrix {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Rows in canonical surface-family order.
    pub rows: Vec<EmbeddedBoundaryCorpusMatrixRow>,
    /// Boundary-drill counts across the corpus.
    pub boundary_drill_counts: BTreeMap<String, usize>,
    /// Denial-drill counts across the corpus.
    pub denial_counts: BTreeMap<String, usize>,
}

impl EmbeddedBoundaryCorpusMatrix {
    fn from_cases(cases: &[EmbeddedBoundaryCorpusCase]) -> Self {
        let mut rows: Vec<EmbeddedBoundaryCorpusMatrixRow> = Vec::new();
        let mut boundary_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut denial_counts: BTreeMap<String, usize> = BTreeMap::new();

        for family in SURFACE_FAMILY_ORDER {
            let family_cases: Vec<&EmbeddedBoundaryCorpusCase> = cases
                .iter()
                .filter(|c| c.surface_family == family)
                .collect();
            if family_cases.is_empty() {
                continue;
            }
            let mut drills: BTreeSet<EmbeddedBoundaryCorpusBoundaryDrill> = BTreeSet::new();
            let mut denials: BTreeSet<EmbeddedBoundaryCorpusDenial> = BTreeSet::new();
            let mut states: BTreeSet<String> = BTreeSet::new();
            let mut conformant = 0usize;
            let mut denied = 0usize;
            for case in &family_cases {
                for drill in &case.boundary_drills {
                    drills.insert(*drill);
                }
                if let Some(denial) = case.denial {
                    denials.insert(denial);
                }
                states.insert(case.row.boundary_state_token.clone());
                match case.expectation {
                    EmbeddedBoundaryCorpusExpectation::Conformant => conformant += 1,
                    EmbeddedBoundaryCorpusExpectation::Denied => denied += 1,
                }
            }
            rows.push(EmbeddedBoundaryCorpusMatrixRow {
                surface_family: family,
                surface_family_token: surface_family_token(family).to_string(),
                boundary_drills_exercised: drills.into_iter().collect(),
                denials_exercised: denials.into_iter().collect(),
                boundary_state_tokens: states.into_iter().collect(),
                conformant_case_count: conformant,
                denial_case_count: denied,
            });
        }

        for case in cases {
            for drill in &case.boundary_drills {
                *boundary_counts
                    .entry(drill.as_str().to_string())
                    .or_insert(0) += 1;
            }
            if let Some(denial) = case.denial {
                *denial_counts
                    .entry(denial.as_str().to_string())
                    .or_insert(0) += 1;
            }
        }

        Self {
            record_kind: EMBEDDED_BOUNDARY_CORPUS_MATRIX_RECORD_KIND.to_string(),
            matrix_id: "shell:embedded_boundary_corpus:matrix:default".to_string(),
            schema_version: EMBEDDED_BOUNDARY_CORPUS_SCHEMA_VERSION,
            shared_contract_ref: EMBEDDED_BOUNDARY_CORPUS_SHARED_CONTRACT_REF.to_string(),
            rows,
            boundary_drill_counts: boundary_counts,
            denial_counts,
        }
    }
}

/// Corpus coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusCoverageSummary {
    /// Surface families present.
    pub surface_families_present: Vec<SurfaceFamily>,
    /// Boundary drills present.
    pub boundary_drills_present: Vec<EmbeddedBoundaryCorpusBoundaryDrill>,
    /// Denials present.
    pub denials_present: Vec<EmbeddedBoundaryCorpusDenial>,
    /// Boundary-state tokens present.
    pub boundary_state_tokens_present: Vec<String>,
    /// Conformant case count.
    pub conformant_case_count: usize,
    /// Denial case count.
    pub denial_case_count: usize,
    /// True when every recorded verdict holds.
    pub all_verdicts_hold: bool,
}

impl EmbeddedBoundaryCorpusCoverageSummary {
    fn from_cases(cases: &[EmbeddedBoundaryCorpusCase]) -> Self {
        let mut drills: BTreeSet<EmbeddedBoundaryCorpusBoundaryDrill> = BTreeSet::new();
        let mut denials: BTreeSet<EmbeddedBoundaryCorpusDenial> = BTreeSet::new();
        let mut states: BTreeSet<String> = BTreeSet::new();
        let mut conformant = 0usize;
        let mut denied = 0usize;
        let mut all_hold = true;
        for case in cases {
            for drill in &case.boundary_drills {
                drills.insert(*drill);
            }
            if let Some(denial) = case.denial {
                denials.insert(denial);
            }
            states.insert(case.row.boundary_state_token.clone());
            match case.expectation {
                EmbeddedBoundaryCorpusExpectation::Conformant => conformant += 1,
                EmbeddedBoundaryCorpusExpectation::Denied => denied += 1,
            }
            all_hold &= case.verdict_holds;
        }
        let families: Vec<SurfaceFamily> = SURFACE_FAMILY_ORDER
            .iter()
            .copied()
            .filter(|family| cases.iter().any(|c| c.surface_family == *family))
            .collect();
        Self {
            surface_families_present: families,
            boundary_drills_present: drills.into_iter().collect(),
            denials_present: denials.into_iter().collect(),
            boundary_state_tokens_present: states.into_iter().collect(),
            conformant_case_count: conformant,
            denial_case_count: denied,
            all_verdicts_hold: all_hold,
        }
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Top-level corpus packet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCorpusPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Beta audit page re-quoted in full so the corpus is self-contained
    /// for support reviews.
    pub base_audit_page: EmbeddedBoundaryAuditPage,
    /// Base audit page shared contract ref — re-asserted for review.
    pub base_audit_page_shared_contract_ref: String,
    /// Worked drill cases (boundary + denial).
    pub corpus_cases: Vec<EmbeddedBoundaryCorpusCase>,
    /// Support export projection.
    pub support_export: EmbeddedBoundaryCorpusSupportExport,
    /// Coverage matrix projection.
    pub matrix: EmbeddedBoundaryCorpusMatrix,
    /// Coverage summary.
    pub coverage_summary: EmbeddedBoundaryCorpusCoverageSummary,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validation errors raised against the corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum EmbeddedBoundaryCorpusValidationError {
    /// Packet metadata is wrong.
    PacketMetadataWrong { reason: String },
    /// A required surface family is missing.
    SurfaceFamilyMissing { missing: String },
    /// A required boundary drill is missing.
    BoundaryDrillMissing { missing: String },
    /// A required denial drill is missing.
    DenialMissing { missing: String },
    /// A case's recorded verdict does not hold (the gate let an
    /// adversarial case through or rejected a conformant one).
    VerdictDoesNotHold {
        case_id: String,
        expected: String,
        actual_tokens: Vec<String>,
    },
    /// A denial case did not name the denial-reason tokens it expects.
    DenialCaseMissingExpectedReasons { case_id: String },
    /// A conformant case named expected denial reasons.
    ConformantCaseNamedDenialReasons { case_id: String },
    /// The support export claimed to carry sensitive payload.
    SupportExportClaimsSensitivePayload,
    /// The support export row count drifted from the case count.
    SupportExportRowCountDrift { cases: usize, support_rows: usize },
}

impl std::fmt::Display for EmbeddedBoundaryCorpusValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketMetadataWrong { reason } => write!(f, "packet metadata invalid: {reason}"),
            Self::SurfaceFamilyMissing { missing } => {
                write!(f, "surface family missing from corpus: {missing}")
            }
            Self::BoundaryDrillMissing { missing } => {
                write!(f, "boundary drill missing from corpus: {missing}")
            }
            Self::DenialMissing { missing } => {
                write!(f, "denial drill missing from corpus: {missing}")
            }
            Self::VerdictDoesNotHold {
                case_id,
                expected,
                actual_tokens,
            } => write!(
                f,
                "case {case_id} verdict does not hold (expected {expected}, gate produced {actual_tokens:?})"
            ),
            Self::DenialCaseMissingExpectedReasons { case_id } => {
                write!(f, "denial case {case_id} names no expected denial reasons")
            }
            Self::ConformantCaseNamedDenialReasons { case_id } => {
                write!(f, "conformant case {case_id} names expected denial reasons")
            }
            Self::SupportExportClaimsSensitivePayload => {
                write!(f, "support export must declare no_sensitive_payload = true")
            }
            Self::SupportExportRowCountDrift {
                cases,
                support_rows,
            } => write!(
                f,
                "support export has {support_rows} rows but corpus has {cases} cases"
            ),
        }
    }
}

impl std::error::Error for EmbeddedBoundaryCorpusValidationError {}

/// Validates a corpus packet against the acceptance invariants.
pub fn validate_embedded_boundary_corpus_packet(
    packet: &EmbeddedBoundaryCorpusPacket,
) -> Result<(), Vec<EmbeddedBoundaryCorpusValidationError>> {
    let mut errors = Vec::new();

    if packet.record_kind != EMBEDDED_BOUNDARY_CORPUS_PACKET_RECORD_KIND {
        errors.push(EmbeddedBoundaryCorpusValidationError::PacketMetadataWrong {
            reason: "record kind mismatch".to_string(),
        });
    }
    if packet.schema_version != EMBEDDED_BOUNDARY_CORPUS_SCHEMA_VERSION {
        errors.push(EmbeddedBoundaryCorpusValidationError::PacketMetadataWrong {
            reason: "schema version mismatch".to_string(),
        });
    }
    if packet.shared_contract_ref != EMBEDDED_BOUNDARY_CORPUS_SHARED_CONTRACT_REF {
        errors.push(EmbeddedBoundaryCorpusValidationError::PacketMetadataWrong {
            reason: "shared contract ref mismatch".to_string(),
        });
    }
    if packet.base_audit_page_shared_contract_ref
        != EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF
    {
        errors.push(EmbeddedBoundaryCorpusValidationError::PacketMetadataWrong {
            reason: "base audit page shared contract ref mismatch".to_string(),
        });
    }

    // The base audit page must itself be defect-free.
    if !packet.base_audit_page.defects.is_empty() {
        errors.push(EmbeddedBoundaryCorpusValidationError::PacketMetadataWrong {
            reason: "base audit page seeds defects".to_string(),
        });
    }

    let mut families: Vec<SurfaceFamily> = Vec::new();
    let mut drills: BTreeSet<EmbeddedBoundaryCorpusBoundaryDrill> = BTreeSet::new();
    let mut denials: BTreeSet<EmbeddedBoundaryCorpusDenial> = BTreeSet::new();

    for case in &packet.corpus_cases {
        if !families.contains(&case.surface_family) {
            families.push(case.surface_family);
        }
        for drill in &case.boundary_drills {
            drills.insert(*drill);
        }
        if let Some(denial) = case.denial {
            denials.insert(denial);
        }

        if !case.verdict_holds {
            errors.push(EmbeddedBoundaryCorpusValidationError::VerdictDoesNotHold {
                case_id: case.case_id.clone(),
                expected: case.expectation.as_str().to_string(),
                actual_tokens: case.actual_denial_reason_tokens.clone(),
            });
        }

        match case.expectation {
            EmbeddedBoundaryCorpusExpectation::Denied => {
                if case.expected_denial_reason_tokens.is_empty() {
                    errors.push(
                        EmbeddedBoundaryCorpusValidationError::DenialCaseMissingExpectedReasons {
                            case_id: case.case_id.clone(),
                        },
                    );
                }
            }
            EmbeddedBoundaryCorpusExpectation::Conformant => {
                if !case.expected_denial_reason_tokens.is_empty() {
                    errors.push(
                        EmbeddedBoundaryCorpusValidationError::ConformantCaseNamedDenialReasons {
                            case_id: case.case_id.clone(),
                        },
                    );
                }
            }
        }
    }

    for family in SURFACE_FAMILY_ORDER {
        if !families.contains(&family) {
            errors.push(
                EmbeddedBoundaryCorpusValidationError::SurfaceFamilyMissing {
                    missing: surface_family_token(family).to_string(),
                },
            );
        }
    }
    for drill in EmbeddedBoundaryCorpusBoundaryDrill::all() {
        if !drills.contains(&drill) {
            errors.push(
                EmbeddedBoundaryCorpusValidationError::BoundaryDrillMissing {
                    missing: drill.as_str().to_string(),
                },
            );
        }
    }
    for denial in EmbeddedBoundaryCorpusDenial::all() {
        if !denials.contains(&denial) {
            errors.push(EmbeddedBoundaryCorpusValidationError::DenialMissing {
                missing: denial.as_str().to_string(),
            });
        }
    }

    if !packet.support_export.no_sensitive_payload {
        errors.push(EmbeddedBoundaryCorpusValidationError::SupportExportClaimsSensitivePayload);
    }
    if packet.support_export.rows.len() != packet.corpus_cases.len() {
        errors.push(
            EmbeddedBoundaryCorpusValidationError::SupportExportRowCountDrift {
                cases: packet.corpus_cases.len(),
                support_rows: packet.support_export.rows.len(),
            },
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// ---------------------------------------------------------------------------
// Seeded corpus
// ---------------------------------------------------------------------------

const SURFACE_FAMILY_ORDER: [SurfaceFamily; 5] = [
    SurfaceFamily::EmbeddedDocsHelp,
    SurfaceFamily::ExtensionHostedSurface,
    SurfaceFamily::EmbeddedMarketplaceOrAccount,
    SurfaceFamily::EmbeddedServiceDashboard,
    SurfaceFamily::EmbeddedAuthConfirmation,
];

const REQUIRED_NATIVE_RESERVED_SURFACE_TOKENS: [&str; 6] = [
    "product_security_messaging",
    "update_verification",
    "workspace_trust_elevation",
    "rollback_or_restore_confirmation",
    "ai_apply_review",
    "high_risk_approval_sheet",
];

/// Builds the deterministic seeded corpus packet.
pub fn seeded_embedded_boundary_corpus_packet() -> EmbeddedBoundaryCorpusPacket {
    let base_page = seeded_embedded_boundary_audit_page();
    let cases = build_cases(&base_page);
    let support_export = EmbeddedBoundaryCorpusSupportExport::from_cases(&cases);
    let matrix = EmbeddedBoundaryCorpusMatrix::from_cases(&cases);
    let coverage_summary = EmbeddedBoundaryCorpusCoverageSummary::from_cases(&cases);

    EmbeddedBoundaryCorpusPacket {
        record_kind: EMBEDDED_BOUNDARY_CORPUS_PACKET_RECORD_KIND.to_string(),
        schema_version: EMBEDDED_BOUNDARY_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: EMBEDDED_BOUNDARY_CORPUS_SHARED_CONTRACT_REF.to_string(),
        packet_id: EMBEDDED_BOUNDARY_CORPUS_PACKET_ID.to_string(),
        generated_at: EMBEDDED_BOUNDARY_CORPUS_GENERATED_AT.to_string(),
        base_audit_page_shared_contract_ref: base_page.shared_contract_ref.clone(),
        base_audit_page: base_page,
        corpus_cases: cases,
        support_export,
        matrix,
        coverage_summary,
    }
}

/// Renders the human-readable validation summary used by the bin.
pub fn validate_embedded_boundary_corpus_packet_summary(
    packet: &EmbeddedBoundaryCorpusPacket,
) -> Result<String, Vec<EmbeddedBoundaryCorpusValidationError>> {
    validate_embedded_boundary_corpus_packet(packet)?;
    Ok(format!(
        "ok ({} cases: {} conformant, {} denial; {} boundary drills, {} denials covered)",
        packet.corpus_cases.len(),
        packet.coverage_summary.conformant_case_count,
        packet.coverage_summary.denial_case_count,
        packet.coverage_summary.boundary_drills_present.len(),
        packet.coverage_summary.denials_present.len(),
    ))
}

// ---------------------------------------------------------------------------
// Case construction helpers
// ---------------------------------------------------------------------------

fn baseline(
    page: &EmbeddedBoundaryAuditPage,
    family: SurfaceFamily,
) -> (EmbeddedBoundaryAuditRow, EmbeddedBoundaryAuditSupportRow) {
    let row = page
        .rows
        .iter()
        .find(|r| r.surface_family == family)
        .cloned()
        .expect("base audit page covers every surface family");
    let support = page
        .support_rows
        .iter()
        .find(|s| s.row_id == row.row_id)
        .cloned()
        .expect("base audit page pairs every row with a support row");
    (row, support)
}

/// Re-keys a row and rebuilds a parity-correct support row for it.
fn rekey(
    case_id: &str,
    mut row: EmbeddedBoundaryAuditRow,
) -> (EmbeddedBoundaryAuditRow, EmbeddedBoundaryAuditSupportRow) {
    let row_id = format!("ux:embedded-boundary-corpus:{case_id}");
    row.case_id = case_id.to_string();
    row.row_id = row_id.clone();
    let support = support_from_row(case_id, &row_id, &row);
    (row, support)
}

/// Builds a support row that mirrors every parity-checked field of `row`,
/// so a conformant clone never trips the support-parity defect.
fn support_from_row(
    case_id: &str,
    row_id: &str,
    row: &EmbeddedBoundaryAuditRow,
) -> EmbeddedBoundaryAuditSupportRow {
    EmbeddedBoundaryAuditSupportRow {
        record_kind: EMBEDDED_BOUNDARY_AUDIT_BETA_SUPPORT_ROW_RECORD_KIND.to_string(),
        schema_version: EMBEDDED_BOUNDARY_AUDIT_BETA_SCHEMA_VERSION,
        shared_contract_ref: EMBEDDED_BOUNDARY_AUDIT_BETA_SHARED_CONTRACT_REF.to_string(),
        case_id: case_id.to_string(),
        row_id: row_id.to_string(),
        surface_family_token: row.surface_family_token.clone(),
        owner_label: row.owner_label.clone(),
        host_or_domain_label: row.host_or_domain_label.clone(),
        data_boundary_class_token: row.data_boundary_class_token.clone(),
        boundary_state_token: row.boundary_state_token.clone(),
        permission_class_token: row.permission_class_token.clone(),
        browser_fallback_posture_token: row.browser_fallback_posture_token.clone(),
        fallback_target_class_token: row.fallback_target_class_token.clone(),
        browser_handoff_packet_ref: row.browser_handoff_packet_ref.clone(),
        identity_mode_token: row.identity_mode_token.clone(),
        trust_state_token: row.trust_state_token.clone(),
        auth_flow_class_token: row.auth_flow_class_token.clone(),
        provider_health_token: row.provider_health_token.clone(),
        native_reserved_surface_tokens: row.native_reserved_surface_tokens.clone(),
        redaction_class_token: row.redaction_class_token.clone(),
    }
}

fn open_in_browser_truth(
    return_target: &str,
    reason: &str,
    packet_ref: Option<&str>,
) -> EmbeddedBoundaryFallbackTruth {
    EmbeddedBoundaryFallbackTruth {
        action_id_token: "open_in_system_browser".to_string(),
        return_target_label: return_target.to_string(),
        reason_label: reason.to_string(),
        preserves_object_identity: true,
        browser_handoff_packet_ref: packet_ref.map(str::to_string),
    }
}

fn lifecycle_fence_intact(permission_class_token: &str) -> EmbeddedBoundaryLifecyclePersistence {
    let phase = |token: &str| EmbeddedBoundaryLifecyclePhase {
        phase_token: token.to_string(),
        native_reserved_surface_tokens: REQUIRED_NATIVE_RESERVED_SURFACE_TOKENS
            .iter()
            .map(|t| t.to_string())
            .collect(),
        permission_class_token: permission_class_token.to_string(),
        high_risk_action_posture_token: "host_native_only".to_string(),
    };
    EmbeddedBoundaryLifecyclePersistence {
        phases: vec![
            phase("initial_render"),
            phase("after_app_restart"),
            phase("after_surface_reentry"),
        ],
    }
}

#[allow(clippy::too_many_lines)]
fn build_cases(page: &EmbeddedBoundaryAuditPage) -> Vec<EmbeddedBoundaryCorpusCase> {
    let mut cases: Vec<EmbeddedBoundaryCorpusCase> = Vec::new();

    // ----- Boundary drills (conformant) -----------------------------------

    // Docs/help, live verified, open-in-browser through a host handoff.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedDocsHelp);
        let (row, support) = rekey("embedded_boundary_corpus:docs_help_live_verified", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Docs/help pane — live verified, host-owned open-in-browser".to_string(),
                surface_family: SurfaceFamily::EmbeddedDocsHelp,
                boundary_drills: vec![
                    EmbeddedBoundaryCorpusBoundaryDrill::OwnerOriginVerified,
                    EmbeddedBoundaryCorpusBoundaryDrill::OpenInBrowserFallback,
                ],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: Some(open_in_browser_truth(
                    "Aureline desktop / Docs pane",
                    "Open the same docs topic in your system browser.",
                    row.browser_handoff_packet_ref.as_deref(),
                )),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedFullAuthority,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Local signed docs pack. Owner, origin, and freshness are disclosed; open-in-browser keeps the topic and grants no embedded authority."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Auth confirmation, system-browser first.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedAuthConfirmation);
        let (row, support) = rekey("embedded_boundary_corpus:auth_system_browser_first", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Auth handoff — system browser preferred".to_string(),
                surface_family: SurfaceFamily::EmbeddedAuthConfirmation,
                boundary_drills: vec![
                    EmbeddedBoundaryCorpusBoundaryDrill::SystemBrowserFirstAuth,
                    EmbeddedBoundaryCorpusBoundaryDrill::OwnerOriginVerified,
                ],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: Some(open_in_browser_truth(
                    "Aureline desktop / Auth handoff sheet",
                    "Sign in opens in your system browser; nothing is typed in the card.",
                    row.browser_handoff_packet_ref.as_deref(),
                )),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedWithNativeStepUp,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Sign-in opens in the system browser and returns a packet; high-risk approvals still require a host-native step-up after sign-in."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Marketplace/account, stale snapshot.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedMarketplaceOrAccount);
        let (row, support) = rekey("embedded_boundary_corpus:marketplace_stale_snapshot", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Marketplace/account — stale provider snapshot".to_string(),
                surface_family: SurfaceFamily::EmbeddedMarketplaceOrAccount,
                boundary_drills: vec![
                    EmbeddedBoundaryCorpusBoundaryDrill::StaleSnapshot,
                    EmbeddedBoundaryCorpusBoundaryDrill::OpenInBrowserFallback,
                ],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: Some(open_in_browser_truth(
                    "Aureline desktop / Marketplace account pane",
                    "Renew the provider session in your system browser before protected actions.",
                    row.browser_handoff_packet_ref.as_deref(),
                )),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedInspectOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Account page shows a stale in-product snapshot; it is inspect-only until the provider session is renewed in the browser."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Extension webview, cross-origin limited.
    {
        let (row, _) = baseline(page, SurfaceFamily::ExtensionHostedSurface);
        let (row, support) = rekey(
            "embedded_boundary_corpus:extension_cross_origin_limited",
            row,
        );
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Extension webview — cross-origin limited".to_string(),
                surface_family: SurfaceFamily::ExtensionHostedSurface,
                boundary_drills: vec![
                    EmbeddedBoundaryCorpusBoundaryDrill::CrossOriginLimitation,
                    EmbeddedBoundaryCorpusBoundaryDrill::OpenInBrowserFallback,
                ],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: Some(open_in_browser_truth(
                    "Aureline desktop / Extension panel",
                    "Open the cross-origin page in your system browser to interact with it.",
                    row.browser_handoff_packet_ref.as_deref(),
                )),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedInspectOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "The extension panel renders a cross-origin page Aureline cannot read; the limitation is named and open-in-browser preserves the page identity."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Service dashboard, managed policy deny.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedServiceDashboard);
        let (row, support) = rekey(
            "embedded_boundary_corpus:service_dashboard_policy_blocked",
            row,
        );
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Service dashboard — managed policy deny".to_string(),
                surface_family: SurfaceFamily::EmbeddedServiceDashboard,
                boundary_drills: vec![EmbeddedBoundaryCorpusBoundaryDrill::ManagedPolicyDeny],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedBrowserOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Managed-workspace policy disables the embedded render; the card names the policy and routes recovery into the host-native review surface."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Service dashboard, certificate failure (new drill).
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedServiceDashboard);
        row.boundary_state_token = "certificate_failed".to_string();
        row.boundary_state_label = "Certificate failed".to_string();
        row.origin_verification_token = "certificate_failed".to_string();
        row.data_boundary_label =
            "Hosted dashboard. TLS certificate verification failed.".to_string();
        row.permission_class_token = "host_owned_browser_only".to_string();
        row.permission_label =
            "Host-owned browser only. Certificate could not be verified.".to_string();
        row.browser_fallback_posture_token = "system_browser_first".to_string();
        row.fallback_target_class_token = "system_browser_handoff_packet".to_string();
        row.browser_handoff_packet_ref =
            Some("id:browser-handoff:service-dashboard:cert-review".to_string());
        row.provider_health_token = Some("unavailable".to_string());
        row.plain_language_summary =
            "Aureline could not verify this dashboard's certificate, so it refuses to render the body. Inspect the certificate or open the page in your system browser."
                .to_string();
        let (row, support) = rekey(
            "embedded_boundary_corpus:service_dashboard_certificate_failed",
            row,
        );
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Service dashboard — certificate verification failed".to_string(),
                surface_family: SurfaceFamily::EmbeddedServiceDashboard,
                boundary_drills: vec![
                    EmbeddedBoundaryCorpusBoundaryDrill::CertificateFailure,
                    EmbeddedBoundaryCorpusBoundaryDrill::OpenInBrowserFallback,
                ],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: Some(open_in_browser_truth(
                    "Aureline desktop / Service dashboard pane",
                    "Inspect the certificate or open the dashboard in your system browser.",
                    row.browser_handoff_packet_ref.as_deref(),
                )),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedBrowserOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Certificate verification failed; the host refuses to render the body, names the failure, and offers certificate inspection plus a system-browser handoff."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Marketplace/account, offline snapshot, external open unavailable.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedMarketplaceOrAccount);
        row.boundary_state_token = "offline_snapshot".to_string();
        row.boundary_state_label = "Offline snapshot".to_string();
        row.origin_verification_token = "offline_cached".to_string();
        row.data_boundary_label = "Provider account surface (offline cached snapshot).".to_string();
        row.permission_class_token = "host_owned_inspect_only".to_string();
        row.permission_label = "Host-owned inspect-only while offline.".to_string();
        row.browser_fallback_posture_token = "external_open_unavailable_offline".to_string();
        row.fallback_target_class_token = "local_inspect_or_export".to_string();
        row.browser_handoff_packet_ref = None;
        row.plain_language_summary =
            "You are offline. This is the last cached account snapshot; external open is unavailable until you reconnect."
                .to_string();
        let (row, support) = rekey("embedded_boundary_corpus:marketplace_offline_snapshot", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Marketplace/account — offline snapshot, external open unavailable"
                    .to_string(),
                surface_family: SurfaceFamily::EmbeddedMarketplaceOrAccount,
                boundary_drills: vec![EmbeddedBoundaryCorpusBoundaryDrill::OfflineSnapshot],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedInspectOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Offline cached account snapshot. The fallback posture is honest: external open is unavailable until reconnect, and the surface stays inspect-only."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Auth confirmation, device-code fallback.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedAuthConfirmation);
        row.boundary_state_label = "Live verified".to_string();
        row.permission_label =
            "Host-owned with native step-up; device-code is the auditable fallback.".to_string();
        row.browser_fallback_posture_token = "device_code_fallback_offered".to_string();
        row.fallback_target_class_token = "device_code_companion_card".to_string();
        row.browser_handoff_packet_ref = None;
        row.auth_flow_class_token = Some("device_code".to_string());
        row.plain_language_summary =
            "The system browser could not return automatically, so device-code is offered. Copy the code and approve it in any browser; nothing is typed in the card."
                .to_string();
        let (row, support) = rekey("embedded_boundary_corpus:auth_device_code_fallback", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Auth handoff — device-code fallback".to_string(),
                surface_family: SurfaceFamily::EmbeddedAuthConfirmation,
                boundary_drills: vec![EmbeddedBoundaryCorpusBoundaryDrill::DeviceCodeFallback],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedWithNativeStepUp,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "When the browser cannot return, device-code is the auditable fallback. The card copies a code only and never collects a password."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Auth confirmation, approval fence persists across restart and reentry.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedAuthConfirmation);
        let (row, support) = rekey("embedded_boundary_corpus:approval_fence_persists", row);
        let permission = row.permission_class_token.clone();
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Auth handoff — approval fence persists across restart and re-entry"
                    .to_string(),
                surface_family: SurfaceFamily::EmbeddedAuthConfirmation,
                boundary_drills: vec![
                    EmbeddedBoundaryCorpusBoundaryDrill::NativeApprovalFencePersistsRestart,
                    EmbeddedBoundaryCorpusBoundaryDrill::NativeApprovalFencePersistsReentry,
                ],
                denial: None,
                expectation: EmbeddedBoundaryCorpusExpectation::Conformant,
                expected_denial_reason_tokens: vec![],
                browser_fallback_truth: None,
                lifecycle_persistence: Some(lifecycle_fence_intact(&permission)),
                native_path_authority: EmbeddedAuthorityClass::HostOwnedWithNativeStepUp,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "The six native-reserved approval surfaces and the host-native step-up survive an app restart and a surface re-entry; the embedded body never inherits them."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // ----- Denial drills (adversarial) ------------------------------------

    // Owner/origin spoof: owner disclosure dropped.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedDocsHelp);
        row.owner_label = String::new();
        row.plain_language_summary =
            "Adversarial: the embedded body suppressed the owner label so the surface looks like first-party chrome."
                .to_string();
        let (row, support) = rekey("embedded_boundary_corpus:denial_owner_origin_spoof", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — owner/origin disclosure dropped".to_string(),
                surface_family: SurfaceFamily::EmbeddedDocsHelp,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::OwnerOriginSpoof),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec!["missing_owner_label".to_string()],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedFullAuthority,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Dropping the owner label must be denied — the host shell owns owner/origin disclosure and an embedded body cannot suppress it."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Stale masked as live: certificate-failed origin painted live_verified.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedServiceDashboard);
        row.boundary_state_token = "live_verified".to_string();
        row.boundary_state_label = "Live verified".to_string();
        row.origin_verification_token = "certificate_failed".to_string();
        row.plain_language_summary =
            "Adversarial: a certificate-failed origin is painted as live verified to hide the failure."
                .to_string();
        let (row, support) = rekey("embedded_boundary_corpus:denial_stale_masked_as_live", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — failed origin painted as live verified".to_string(),
                surface_family: SurfaceFamily::EmbeddedServiceDashboard,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::StaleMaskedAsLive),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec![
                    "boundary_state_inconsistent_with_origin_verification".to_string(),
                ],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedBrowserOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "A boundary state of live_verified over a certificate_failed origin must be denied — the boundary state may not contradict the origin verification."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Native trust/update chrome spoof: drop update_verification.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedDocsHelp);
        row.native_reserved_surface_tokens
            .retain(|t| t != "update_verification" && t != "product_security_messaging");
        row.plain_language_summary =
            "Adversarial: the embedded body tried to host update and security messaging itself."
                .to_string();
        let (row, support) = rekey(
            "embedded_boundary_corpus:denial_native_trust_chrome_spoof",
            row,
        );
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — embedded body impersonates native trust/update chrome"
                    .to_string(),
                surface_family: SurfaceFamily::EmbeddedDocsHelp,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::NativeTrustChromeSpoof),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec![
                    "embedded_minted_native_reserved_surface".to_string(),
                ],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedFullAuthority,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "Update verification and product security messaging are native-reserved; an embedded body that drops them from the host-owned set must be denied."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Approval bypass: drop high-risk approval / ai apply review.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedServiceDashboard);
        row.native_reserved_surface_tokens
            .retain(|t| t != "high_risk_approval_sheet" && t != "ai_apply_review");
        row.plain_language_summary =
            "Adversarial: the embedded body tried to host the high-risk approval and AI apply review.".to_string();
        let (row, support) = rekey("embedded_boundary_corpus:denial_approval_bypass", row);
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — embedded body hosts a high-risk approval".to_string(),
                surface_family: SurfaceFamily::EmbeddedServiceDashboard,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::ApprovalBypass),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec![
                    "embedded_minted_native_reserved_surface".to_string(),
                ],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedBrowserOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "The high-risk approval sheet and AI apply review are native-reserved; an embedded body that tries to host them bypasses preview/approval and must be denied."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Authority widening: browser fallback grants more than the native command.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedMarketplaceOrAccount);
        let (mut row, support) = rekey("embedded_boundary_corpus:denial_authority_widening", row);
        row.plain_language_summary =
            "Adversarial: the reopen path grants full provider authority in-product though the native command is inspect-only."
                .to_string();
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — fallback widens authority past the native command".to_string(),
                surface_family: SurfaceFamily::EmbeddedMarketplaceOrAccount,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::AuthorityWidening),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec![
                    "fallback_widens_authority_beyond_native".to_string(),
                ],
                browser_fallback_truth: Some(open_in_browser_truth(
                    "Aureline desktop / Marketplace account pane",
                    "Reopen the account surface.",
                    row.browser_handoff_packet_ref.as_deref(),
                )),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedInspectOnly,
                fallback_path_authority: EmbeddedAuthorityClass::HostOwnedFullAuthority,
                plain_language_summary:
                    "A reopen path that confers full in-product authority where the product-owned command is inspect-only widens authority and must be denied."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Embedded password collection without a registered exception.
    {
        let (mut row, _) = baseline(page, SurfaceFamily::EmbeddedAuthConfirmation);
        row.auth_flow_class_token = Some("embedded_password_exception".to_string());
        row.auth_exception_id_ref = None;
        row.permission_label =
            "Adversarial: password collected inside the embedded card.".to_string();
        row.plain_language_summary =
            "Adversarial: the card collects a password without a registered lower-trust exception."
                .to_string();
        let (row, support) = rekey(
            "embedded_boundary_corpus:denial_embedded_password_collection",
            row,
        );
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — embedded password without a registered exception".to_string(),
                surface_family: SurfaceFamily::EmbeddedAuthConfirmation,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::EmbeddedPasswordCollection),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec![
                    "embedded_auth_exception_missing_exception_ref".to_string(),
                ],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedWithNativeStepUp,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "An embedded password flow with no exception_id_ref must be denied — embedded credential collection is only ever a registered, lower-trust exception."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Support export flattening: support row drifts from the live row.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedMarketplaceOrAccount);
        let (row, mut support) = rekey(
            "embedded_boundary_corpus:denial_support_export_flattening",
            row,
        );
        support.boundary_state_token = "live_verified".to_string();
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — support export flattens the boundary state".to_string(),
                surface_family: SurfaceFamily::EmbeddedMarketplaceOrAccount,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::SupportExportFlattening),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec!["support_row_vocabulary_drift".to_string()],
                browser_fallback_truth: None,
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedInspectOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "A support export that paints a stale row as live verified flattens the boundary truth and must be denied — support rows mirror the live row exactly."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    // Browser fallback drops target or reason.
    {
        let (row, _) = baseline(page, SurfaceFamily::EmbeddedMarketplaceOrAccount);
        let (row, support) = rekey(
            "embedded_boundary_corpus:denial_browser_fallback_drops_target",
            row,
        );
        cases.push(
            PartialCase {
                case_id: row.case_id.clone(),
                case_label: "Denial — open-in-browser drops the target and reason".to_string(),
                surface_family: SurfaceFamily::EmbeddedMarketplaceOrAccount,
                boundary_drills: vec![],
                denial: Some(EmbeddedBoundaryCorpusDenial::BrowserFallbackDropsTargetOrReason),
                expectation: EmbeddedBoundaryCorpusExpectation::Denied,
                expected_denial_reason_tokens: vec![
                    "open_in_browser_drops_target_or_reason".to_string(),
                ],
                browser_fallback_truth: Some(EmbeddedBoundaryFallbackTruth {
                    action_id_token: "open_in_system_browser".to_string(),
                    return_target_label: String::new(),
                    reason_label: String::new(),
                    preserves_object_identity: false,
                    browser_handoff_packet_ref: row.browser_handoff_packet_ref.clone(),
                }),
                lifecycle_persistence: None,
                native_path_authority: EmbeddedAuthorityClass::HostOwnedInspectOnly,
                fallback_path_authority: EmbeddedAuthorityClass::NoAuthorityWithinProduct,
                plain_language_summary:
                    "An open-in-browser fallback that drops the return target and the reason flattens the handoff into a generic jump and must be denied."
                        .to_string(),
                row,
                support_row: support,
            }
            .finish(),
        );
    }

    cases
}

// ---------------------------------------------------------------------------
// Token helper (mirrors the audit lane's surface-family token)
// ---------------------------------------------------------------------------

fn surface_family_token(value: SurfaceFamily) -> &'static str {
    match value {
        SurfaceFamily::EmbeddedDocsHelp => "embedded_docs_help",
        SurfaceFamily::EmbeddedMarketplaceOrAccount => "embedded_marketplace_or_account",
        SurfaceFamily::EmbeddedServiceDashboard => "embedded_service_dashboard",
        SurfaceFamily::EmbeddedAuthConfirmation => "embedded_auth_confirmation",
        SurfaceFamily::ExtensionHostedSurface => "extension_hosted_surface",
    }
}

// ---------------------------------------------------------------------------
// Markdown renderers
// ---------------------------------------------------------------------------

/// Renders the audit report artifact
/// (`artifacts/ux/m3/embedded_boundary_audit_report.md`).
pub fn render_embedded_boundary_corpus_report_markdown(
    packet: &EmbeddedBoundaryCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Embedded boundary audit corpus report\n\n");
    out.push_str(
        "Generated from the seeded corpus in `crates/aureline-shell/src/embedded_boundary_corpus/mod.rs`.\n",
    );
    out.push_str("Regenerate with:\n\n");
    out.push_str("```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- report-md > \\\n  artifacts/ux/m3/embedded_boundary_audit_report.md\n",
    );
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- doc-md > \\\n  docs/ux/m3/embedded_boundary_audit_beta.md\n",
    );
    out.push_str("```\n\n");
    out.push_str(&format!("- Packet id: `{}`\n", packet.packet_id));
    out.push_str(&format!(
        "- Shared contract ref: `{}`\n",
        packet.shared_contract_ref
    ));
    out.push_str(&format!(
        "- Boundary vocabulary schema: `schemas/ux/embedded_surface_boundary.schema.json`\n",
    ));
    out.push_str(&format!("- Generated at: `{}`\n", packet.generated_at));
    out.push_str(&format!(
        "- Cases: {} ({} conformant, {} denial)\n",
        packet.corpus_cases.len(),
        packet.coverage_summary.conformant_case_count,
        packet.coverage_summary.denial_case_count,
    ));
    out.push_str(&format!(
        "- All recorded verdicts hold: {}\n\n",
        if packet.coverage_summary.all_verdicts_hold {
            "yes"
        } else {
            "no"
        }
    ));

    out.push_str("## Surface family matrix\n\n");
    out.push_str(
        "| Surface | Boundary drills | Denials | Boundary states | Conformant | Denial |\n",
    );
    out.push_str(
        "| ------- | --------------- | ------- | --------------- | ---------- | ------ |\n",
    );
    for row in &packet.matrix.rows {
        out.push_str(&format!(
            "| `{family}` | {drills} | {denials} | {states} | {conf} | {den} |\n",
            family = row.surface_family_token,
            drills = format_tokens(row.boundary_drills_exercised.iter().map(|d| d.as_str())),
            denials = format_tokens(row.denials_exercised.iter().map(|d| d.as_str())),
            states = format_tokens(row.boundary_state_tokens.iter().map(String::as_str)),
            conf = row.conformant_case_count,
            den = row.denial_case_count,
        ));
    }
    out.push('\n');

    out.push_str("## Boundary drill coverage\n\n");
    for drill in EmbeddedBoundaryCorpusBoundaryDrill::all() {
        let count = packet
            .matrix
            .boundary_drill_counts
            .get(drill.as_str())
            .copied()
            .unwrap_or(0);
        out.push_str(&format!("- `{}` -- {}\n", drill.as_str(), count));
    }
    out.push('\n');

    out.push_str("## Denial drill coverage\n\n");
    for denial in EmbeddedBoundaryCorpusDenial::all() {
        let count = packet
            .matrix
            .denial_counts
            .get(denial.as_str())
            .copied()
            .unwrap_or(0);
        out.push_str(&format!("- `{}` -- {}\n", denial.as_str(), count));
    }
    out.push('\n');

    out.push_str("## Cases\n\n");
    for case in &packet.corpus_cases {
        out.push_str(&format!(
            "### `{}` -- {}\n\n",
            case.case_id, case.case_label
        ));
        out.push_str(&format!("- Surface: `{}`\n", case.row.surface_family_token));
        out.push_str(&format!("- Kind: `{}`\n", case.case_kind_token));
        out.push_str(&format!(
            "- Owner: {} (`{}`)\n",
            owner_or_dash(&case.row.owner_label),
            case.row.owner_class_token
        ));
        out.push_str(&format!(
            "- Origin: `{}` ({})\n",
            case.row.host_or_domain_label, case.row.origin_verification_token
        ));
        out.push_str(&format!(
            "- Boundary state: `{}`\n",
            case.row.boundary_state_token
        ));
        out.push_str(&format!(
            "- Permission: `{}`\n",
            case.row.permission_class_token
        ));
        out.push_str(&format!(
            "- Fallback posture: `{}` -> `{}`\n",
            case.row.browser_fallback_posture_token, case.row.fallback_target_class_token
        ));
        out.push_str(&format!(
            "- Authority: native `{}` / fallback `{}`\n",
            case.native_path_authority_token, case.fallback_path_authority_token
        ));
        out.push_str(&format!("- Expectation: `{}`\n", case.expectation.as_str()));
        if !case.expected_denial_reason_tokens.is_empty() {
            out.push_str(&format!(
                "- Expected denial reasons: {}\n",
                format_tokens(
                    case.expected_denial_reason_tokens
                        .iter()
                        .map(String::as_str)
                )
            ));
        }
        out.push_str(&format!(
            "- Gate produced: {}\n",
            if case.actual_denial_reason_tokens.is_empty() {
                "(none)".to_string()
            } else {
                format_tokens(case.actual_denial_reason_tokens.iter().map(String::as_str))
            }
        ));
        out.push_str(&format!(
            "- Verdict holds: {}\n\n",
            if case.verdict_holds { "yes" } else { "no" }
        ));
        out.push_str(&format!("{}\n\n", case.plain_language_summary));
    }

    out
}

/// Renders the beta audit doc
/// (`docs/ux/m3/embedded_boundary_audit_beta.md`).
pub fn render_embedded_boundary_corpus_doc_markdown(
    packet: &EmbeddedBoundaryCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Embedded boundary audit (beta)\n\n");
    out.push_str(
        "This corpus turns the embedded-surface boundary toolkit into a regression-gated proof system. Every claimed embedded beta surface — docs/help panes, marketplace/account pages, provider-owned webviews, service dashboards, and auth-handoff sheets — has a worked drill that proves the host shell keeps the boundary honest, plus an adversarial drill that proves the gate rejects spoofing, flattening, approval bypass, and authority widening.\n\n",
    );
    out.push_str(
        "It is minted from `crates/aureline-shell/src/embedded_boundary_corpus/mod.rs` and replayed by `crates/aureline-shell/tests/embedded_boundary_corpus_fixtures.rs`. The boundary vocabulary schema of record is `schemas/ux/embedded_surface_boundary.schema.json`; the per-row audit validator is reused from the beta audit lane so a drill cannot drift from what ships.\n\n",
    );

    out.push_str("## What every conformant surface must prove\n\n");
    out.push_str("1. **Owner / origin / publisher disclosure.** The host shell, not the embedded body, paints the owner, publisher/service, and origin — never hidden behind hover, scroll, or the embedded body.\n");
    out.push_str("2. **System-browser-first identity.** Auth and risky web surfaces prefer the system browser, offer device-code as the auditable fallback, and never collect a password inside the card.\n");
    out.push_str("3. **Honest failure naming.** Certificate failure, managed-policy deny, cross-origin limitation, stale snapshot, and offline snapshot are named in product, not rendered as a broken or blank page.\n");
    out.push_str("4. **Open-in-browser truth.** The fallback keeps its return target and reason, preserves object identity, and never widens in-product authority past the product-owned command.\n");
    out.push_str("5. **Host-owned approvals.** The six native-reserved surfaces stay host-owned and survive an app restart and a surface re-entry; the embedded body never inherits them.\n");
    out.push_str("6. **Support-export parity.** The support export reconstructs owner, boundary state, and approval origin from stable tokens with no raw payload.\n\n");

    out.push_str("## Drill coverage\n\n");
    out.push_str("| Drill | Cases |\n| ----- | ----- |\n");
    for drill in EmbeddedBoundaryCorpusBoundaryDrill::all() {
        let count = packet
            .matrix
            .boundary_drill_counts
            .get(drill.as_str())
            .copied()
            .unwrap_or(0);
        out.push_str(&format!("| `{}` | {} |\n", drill.as_str(), count));
    }
    out.push('\n');

    out.push_str("## Denial coverage\n\n");
    out.push_str("Each denial drill ships an adversarial row that the gate must reject. The case records the exact reason token the gate produces, so a regression that lets the behavior through fails the fixture replay.\n\n");
    out.push_str("| Denial | Cases | Proven by |\n| ------ | ----- | --------- |\n");
    for denial in EmbeddedBoundaryCorpusDenial::all() {
        let count = packet
            .matrix
            .denial_counts
            .get(denial.as_str())
            .copied()
            .unwrap_or(0);
        let reasons = packet
            .corpus_cases
            .iter()
            .find(|c| c.denial == Some(denial))
            .map(|c| format_tokens(c.expected_denial_reason_tokens.iter().map(String::as_str)))
            .unwrap_or_else(|| "(none)".to_string());
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            denial.as_str(),
            count,
            reasons
        ));
    }
    out.push('\n');

    out.push_str("## How to read a case\n\n");
    out.push_str("Each case in `fixtures/ux/m3/embedded_boundary_corpus/corpus_cases.json` carries the audited row, the support row, the authority pair (native vs fallback), the open-in-browser fallback truth, the lifecycle persistence snapshots (for approval-fence drills), the expected verdict, and the denial-reason tokens the gate actually produced. A conformant case must produce zero denial reasons; a denial case must produce at least the reasons it names.\n\n");
    out.push_str("## Regenerate and verify\n\n");
    out.push_str("```sh\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- packet         > fixtures/ux/m3/embedded_boundary_corpus/packet.json\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- cases          > fixtures/ux/m3/embedded_boundary_corpus/corpus_cases.json\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- matrix-json     > fixtures/ux/m3/embedded_boundary_corpus/matrix.json\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- support-export  > fixtures/ux/m3/embedded_boundary_corpus/support_export.json\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- report-md       > artifacts/ux/m3/embedded_boundary_audit_report.md\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- doc-md          > docs/ux/m3/embedded_boundary_audit_beta.md\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- validate\n");
    out.push_str("cargo test -p aureline-shell --test embedded_boundary_corpus_fixtures\n");
    out.push_str("```\n");

    out
}

fn format_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> String {
    let collected: Vec<String> = tokens.map(|t| format!("`{t}`")).collect();
    if collected.is_empty() {
        "(none)".to_string()
    } else {
        collected.join(", ")
    }
}

fn owner_or_dash(label: &str) -> String {
    if label.trim().is_empty() {
        "(dropped)".to_string()
    } else {
        label.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_embedded_boundary_corpus_packet();
        validate_embedded_boundary_corpus_packet(&packet).expect("seeded packet must validate");
    }

    #[test]
    fn every_verdict_holds() {
        let packet = seeded_embedded_boundary_corpus_packet();
        for case in &packet.corpus_cases {
            assert!(
                case.verdict_holds,
                "case {} verdict must hold (expected {}, gate produced {:?})",
                case.case_id,
                case.expectation.as_str(),
                case.actual_denial_reason_tokens,
            );
        }
        assert!(packet.coverage_summary.all_verdicts_hold);
    }

    #[test]
    fn coverage_is_complete() {
        let packet = seeded_embedded_boundary_corpus_packet();
        for family in SURFACE_FAMILY_ORDER {
            assert!(packet
                .coverage_summary
                .surface_families_present
                .contains(&family));
        }
        for drill in EmbeddedBoundaryCorpusBoundaryDrill::all() {
            assert!(
                packet
                    .coverage_summary
                    .boundary_drills_present
                    .contains(&drill),
                "missing boundary drill {}",
                drill.as_str()
            );
        }
        for denial in EmbeddedBoundaryCorpusDenial::all() {
            assert!(
                packet.coverage_summary.denials_present.contains(&denial),
                "missing denial {}",
                denial.as_str()
            );
        }
    }

    #[test]
    fn conformant_cases_produce_no_denials() {
        let packet = seeded_embedded_boundary_corpus_packet();
        for case in packet
            .corpus_cases
            .iter()
            .filter(|c| c.expectation == EmbeddedBoundaryCorpusExpectation::Conformant)
        {
            assert!(
                case.actual_denial_reason_tokens.is_empty(),
                "conformant case {} unexpectedly produced {:?}",
                case.case_id,
                case.actual_denial_reason_tokens
            );
        }
    }

    #[test]
    fn denial_cases_fire_their_expected_gate_tokens() {
        let packet = seeded_embedded_boundary_corpus_packet();
        for case in packet
            .corpus_cases
            .iter()
            .filter(|c| c.expectation == EmbeddedBoundaryCorpusExpectation::Denied)
        {
            assert!(
                !case.actual_denial_reason_tokens.is_empty(),
                "denial case {} produced no gate denial",
                case.case_id
            );
            for token in &case.expected_denial_reason_tokens {
                assert!(
                    case.actual_denial_reason_tokens.contains(token),
                    "denial case {} missing expected token {}",
                    case.case_id,
                    token
                );
            }
        }
    }

    #[test]
    fn validator_flags_a_verdict_that_does_not_hold() {
        let mut packet = seeded_embedded_boundary_corpus_packet();
        // Flip a denial case into a falsely-passing verdict.
        if let Some(case) = packet
            .corpus_cases
            .iter_mut()
            .find(|c| c.expectation == EmbeddedBoundaryCorpusExpectation::Denied)
        {
            case.verdict_holds = false;
        }
        let errors = validate_embedded_boundary_corpus_packet(&packet)
            .expect_err("a non-holding verdict must fail validation");
        assert!(errors.iter().any(|e| matches!(
            e,
            EmbeddedBoundaryCorpusValidationError::VerdictDoesNotHold { .. }
        )));
    }

    #[test]
    fn support_export_carries_no_sensitive_payload() {
        let packet = seeded_embedded_boundary_corpus_packet();
        assert!(packet.support_export.no_sensitive_payload);
        assert_eq!(packet.support_export.rows.len(), packet.corpus_cases.len());
    }
}
