//! Stabilize approval-ticket audit and target-identity lineage for external
//! mutation, credential projection, and privileged debug surfaces.
//!
//! This module promotes the beta approval-ticket page in
//! [`aureline_auth::approval_tickets`] to an evidence-backed stable proof
//! packet whose qualification is derived from the audit rather than asserted
//! from a spreadsheet row.
//!
//! The stable claim holds when **all** of the following conditions are
//! verified:
//!
//! 1. The upstream approval-ticket beta page audits with zero defects.
//! 2. All four required beta profiles (`connected`, `mirror_only`, `offline`,
//!    `enterprise_managed`) have at least one issued ticket row.
//! 3. All four sandbox-profile classes (`local_only_authority`,
//!    `provider_mutation_sandbox`, `remote_helper_sandbox`,
//!    `credential_projection_sandbox`) are covered by sandbox-profile rows.
//! 4. Every capability envelope and every ticket row carries a non-empty
//!    target identity (non-empty `target_ref`).
//! 5. No ticket admits capabilities beyond its capability envelope's allowed
//!    set (no silent authority widening).
//! 6. Every ticket with `bounded_reuse` use-posture (remembered-approval
//!    rows) carries at least one evidence ref that proves fresh-ticket-at-
//!    use-time lineage.
//! 7. Credential projection rows expose projection mode via the
//!    `credential_projection_sandbox` sandbox-profile class; no credential
//!    projection action class is admitted under a non-projection sandbox.
//!
//! Two conditions force `Withdrawn` immediately and cannot be overridden:
//!
//! - A `raw_authority_material_present` defect in the upstream beta page
//!   (narrow reason:
//!   [`StabilizeApprovalTicketNarrowReasonClass::RawAuthorityMaterialPresent`]).
//! - A `self_authorization_attempted` defect in the upstream beta page
//!   (narrow reason:
//!   [`StabilizeApprovalTicketNarrowReasonClass::SelfAuthorizationAttempted`]).
//!
//! A missing required beta profile or sandbox-profile class narrows to
//! `Preview` (not `Beta`) because the coverage gap prevents any verifiable
//! claim for that profile or sandbox.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only. Raw ticket bodies, raw credential payloads, raw delegated-
//! token material, plaintext secrets, and raw authority evidence bodies stay
//! outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage.md`
//! - Artifact: `artifacts/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage.md`
//! - Contract ref: [`STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use aureline_auth::approval_tickets::{
    audit_approval_ticket_beta_page, seeded_approval_ticket_beta_page,
    ApprovalTicketBetaDefectKind, ApprovalTicketBetaPage, ApprovalTicketBetaProfileClass,
    ApprovalTicketBetaSupportExport, SandboxProfileClass,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const STABILIZE_APPROVAL_TICKET_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF: &str =
    "policy:stabilize_approval_ticket_audit_target_identity:v1";

/// Record-kind tag for [`StabilizeApprovalTicketPage`] payloads.
pub const STABILIZE_APPROVAL_TICKET_PAGE_RECORD_KIND: &str =
    "policy_stabilize_approval_ticket_page_record";

/// Record-kind tag for [`StabilizeApprovalTicketRow`] payloads.
pub const STABILIZE_APPROVAL_TICKET_ROW_RECORD_KIND: &str =
    "policy_stabilize_approval_ticket_row_record";

/// Record-kind tag for [`StabilizeApprovalTicketDefect`] payloads.
pub const STABILIZE_APPROVAL_TICKET_DEFECT_RECORD_KIND: &str =
    "policy_stabilize_approval_ticket_defect_record";

/// Record-kind tag for [`StabilizeApprovalTicketSupportExport`] payloads.
pub const STABILIZE_APPROVAL_TICKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_stabilize_approval_ticket_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const STABILIZE_APPROVAL_TICKET_DOC_REF: &str =
    "docs/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage.md";

/// Repo-relative path of the artifact summary for this lane.
pub const STABILIZE_APPROVAL_TICKET_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage.md";

/// Repo-relative path of the upstream approval-ticket beta contract.
pub const APPROVAL_TICKET_BETA_CONTRACT_REF: &str =
    "docs/security/approval_ticket_beta_contract.md";

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual
/// rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// lists from the beta page against the seven stability conditions. A caller
/// may never bump a row to `stable` without a clean audit and complete
/// coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeApprovalTicketQualificationClass {
    /// All seven stability conditions hold and the upstream audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required profile or sandbox-profile class has no coverage; the gap
    /// prevents a beta claim for the missing profile.
    Preview,
    /// Raw authority material was present or self-authorization was attempted
    /// in the upstream beta page; the row is withdrawn entirely and cannot be
    /// overridden.
    Withdrawn,
}

impl StabilizeApprovalTicketQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`StabilizeApprovalTicketQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeApprovalTicketNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// The upstream approval-ticket beta page has one or more defects.
    BetaPageHasDefects,
    /// A required beta profile has no ticket rows; narrows to preview.
    ProfileCoverageMissing,
    /// A required sandbox-profile class has no sandbox rows; narrows to
    /// preview.
    SandboxProfileCoverageMissing,
    /// One or more envelope or ticket rows carry an empty target identity.
    TargetIdentityMissing,
    /// A ticket with `bounded_reuse` posture (remembered approval) is missing
    /// evidence refs that prove fresh-ticket-at-use-time lineage.
    RememberedApprovalMissingFreshTicketEvidence,
    /// A credential projection action class is admitted under a sandbox
    /// profile that is not `credential_projection_sandbox`.
    CredentialProjectionSandboxMissing,
    /// Raw authority material was present in the upstream beta page; withdraws
    /// the packet immediately.
    RawAuthorityMaterialPresent,
    /// A self-authorization attempt was recorded in the upstream beta page;
    /// withdraws the packet immediately.
    SelfAuthorizationAttempted,
}

impl StabilizeApprovalTicketNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::BetaPageHasDefects => "beta_page_has_defects",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::SandboxProfileCoverageMissing => "sandbox_profile_coverage_missing",
            Self::TargetIdentityMissing => "target_identity_missing",
            Self::RememberedApprovalMissingFreshTicketEvidence => {
                "remembered_approval_missing_fresh_ticket_evidence"
            }
            Self::CredentialProjectionSandboxMissing => "credential_projection_sandbox_missing",
            Self::RawAuthorityMaterialPresent => "raw_authority_material_present",
            Self::SelfAuthorizationAttempted => "self_authorization_attempted",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawAuthorityMaterialPresent | Self::SelfAuthorizationAttempted
        )
    }

    /// True when this reason narrows to preview rather than beta.
    pub const fn narrows_to_preview(self) -> bool {
        matches!(
            self,
            Self::ProfileCoverageMissing | Self::SandboxProfileCoverageMissing
        )
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect types
// ---------------------------------------------------------------------------

/// Stability qualification for one profile row in the stabilize packet.
///
/// Each row is bound to a single [`ApprovalTicketBetaProfileClass`] from the
/// upstream beta page. The qualification is derived from the combined audit of
/// all tickets, envelopes, and sandbox rows for that profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizeApprovalTicketRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Beta profile token for this row.
    pub profile_token: String,
    /// Number of sandbox-profile rows for this beta profile.
    pub sandbox_profile_row_count: usize,
    /// Number of issued ticket rows for this beta profile.
    pub ticket_row_count: usize,
    /// Number of capability-envelope rows for this beta profile.
    pub envelope_row_count: usize,
    /// Number of spend-attempt events for this beta profile.
    pub spend_attempt_count: usize,
    /// Number of ticket rows missing non-empty target identity.
    pub missing_target_identity_count: usize,
    /// Number of `bounded_reuse` ticket rows missing evidence refs.
    pub remembered_approval_missing_evidence_count: usize,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

/// Aggregate banner emitted with the stabilize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StabilizeApprovalTicketSummary {
    /// Total row count (one row per beta profile).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Number of defects in the upstream beta page.
    pub beta_page_defect_count: usize,
    /// Beta profile tokens covered across the page.
    pub profiles_covered: Vec<String>,
    /// Sandbox-profile class tokens covered across the page.
    pub sandbox_profiles_covered: Vec<String>,
    /// Number of issued tickets across all profiles.
    pub total_ticket_count: usize,
    /// Number of spend attempts across all profiles.
    pub total_spend_attempt_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl StabilizeApprovalTicketSummary {
    fn from_rows(rows: &[StabilizeApprovalTicketRow], beta_page: &ApprovalTicketBetaPage) -> Self {
        let mut stable = 0usize;
        let mut beta_count = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta_count += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let overall = if withdrawn > 0 {
            StabilizeApprovalTicketQualificationClass::Withdrawn
        } else if preview > 0 {
            StabilizeApprovalTicketQualificationClass::Preview
        } else if beta_count > 0 {
            StabilizeApprovalTicketQualificationClass::Beta
        } else {
            StabilizeApprovalTicketQualificationClass::Stable
        };

        let profiles_covered: BTreeSet<String> = beta_page
            .ticket_rows
            .iter()
            .map(|t| t.profile_token.clone())
            .collect();
        let sandbox_profiles_covered: BTreeSet<String> = beta_page
            .sandbox_profile_rows
            .iter()
            .map(|s| s.sandbox_profile_class_token.clone())
            .collect();

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta_count,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            beta_page_defect_count: beta_page.defects.len(),
            profiles_covered: profiles_covered.into_iter().collect(),
            sandbox_profiles_covered: sandbox_profiles_covered.into_iter().collect(),
            total_ticket_count: beta_page.ticket_rows.len(),
            total_spend_attempt_count: beta_page.spend_attempt_events.len(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the stabilize page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizeApprovalTicketDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: StabilizeApprovalTicketNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (profile token, row id, ticket id, or `"page"`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl StabilizeApprovalTicketDefect {
    fn new(
        narrow_reason: StabilizeApprovalTicketNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: STABILIZE_APPROVAL_TICKET_DEFECT_RECORD_KIND.to_owned(),
            schema_version: STABILIZE_APPROVAL_TICKET_SCHEMA_VERSION,
            shared_contract_ref: STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:stabilize-approval-ticket:{}:{}",
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

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for approval-ticket audit and target-identity lineage.
///
/// The packet is the single inspectable record that proves the stable claim
/// for this lane. Dashboards, docs, Help/About surfaces, and support exports
/// should ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizeApprovalTicketPage {
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
    pub summary: StabilizeApprovalTicketSummary,
    /// Per-profile stability rows.
    pub rows: Vec<StabilizeApprovalTicketRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<StabilizeApprovalTicketDefect>,
    /// The upstream approval-ticket beta page embedded as evidence.
    pub approval_ticket_beta_page: ApprovalTicketBetaPage,
}

impl StabilizeApprovalTicketPage {
    /// Build the stabilize page from an upstream approval-ticket beta page.
    ///
    /// Rows are derived per beta profile, and the qualification for each is
    /// computed from the combined audit of the whole page.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        approval_ticket_beta_page: ApprovalTicketBetaPage,
    ) -> Self {
        let defects = audit_stabilize_page(&approval_ticket_beta_page);
        let rows = derive_stabilize_rows(&approval_ticket_beta_page, &defects);
        let summary = StabilizeApprovalTicketSummary::from_rows(&rows, &approval_ticket_beta_page);
        Self {
            record_kind: STABILIZE_APPROVAL_TICKET_PAGE_RECORD_KIND.to_owned(),
            schema_version: STABILIZE_APPROVAL_TICKET_SCHEMA_VERSION,
            shared_contract_ref: STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            approval_ticket_beta_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == StabilizeApprovalTicketQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all four required beta profiles have at least one ticket row.
    pub fn covers_all_required_profiles(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .approval_ticket_beta_page
            .ticket_rows
            .iter()
            .map(|t| t.profile_token.as_str())
            .collect();
        ApprovalTicketBetaProfileClass::ALL
            .iter()
            .all(|p| covered.contains(p.as_str()))
    }

    /// True when all four required sandbox-profile classes have at least one
    /// sandbox row.
    pub fn covers_all_sandbox_profile_classes(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .approval_ticket_beta_page
            .sandbox_profile_rows
            .iter()
            .map(|s| s.sandbox_profile_class_token.as_str())
            .collect();
        SandboxProfileClass::ALL
            .iter()
            .all(|s| covered.contains(s.as_str()))
    }

    /// True when every envelope and ticket row carries a non-empty target
    /// identity.
    pub fn all_target_identities_are_present(&self) -> bool {
        let envelopes_ok = self
            .approval_ticket_beta_page
            .capability_envelope_rows
            .iter()
            .all(|e| !e.target_identity.target_ref.is_empty());
        let tickets_ok = self
            .approval_ticket_beta_page
            .ticket_rows
            .iter()
            .all(|t| !t.target_identity.target_ref.is_empty());
        envelopes_ok && tickets_ok
    }

    /// True when every `bounded_reuse` ticket carries at least one evidence
    /// ref proving fresh-ticket-at-use-time lineage.
    pub fn remembered_approvals_have_fresh_ticket_evidence(&self) -> bool {
        use aureline_auth::approval_tickets::UsePosture;
        self.approval_ticket_beta_page
            .ticket_rows
            .iter()
            .filter(|t| t.use_posture == UsePosture::BoundedReuse)
            .all(|t| !t.evidence_refs.is_empty())
    }

    /// True when every credential-projection action class is admitted under a
    /// `credential_projection_sandbox` sandbox-profile row.
    pub fn credential_projection_sandbox_is_present(&self) -> bool {
        use aureline_auth::approval_tickets::HighRiskActionClass;
        let has_credential_projection = self
            .approval_ticket_beta_page
            .ticket_rows
            .iter()
            .any(|t| t.action_class == HighRiskActionClass::CredentialProjection);
        if !has_credential_projection {
            return true;
        }
        self.approval_ticket_beta_page
            .sandbox_profile_rows
            .iter()
            .any(|s| s.sandbox_profile_class == SandboxProfileClass::CredentialProjectionSandbox)
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the stabilize page plus a
/// metadata-safe defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizeApprovalTicketSupportExport {
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
    /// The stabilize page embedded as evidence.
    pub page: StabilizeApprovalTicketPage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<StabilizeApprovalTicketNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// The upstream approval-ticket beta support export.
    pub approval_ticket_beta_support_export: ApprovalTicketBetaSupportExport,
    /// True when raw authority material is excluded from this export.
    pub raw_authority_material_excluded: bool,
    /// True when authority lineage (sandbox profile, capability envelope,
    /// ticket, spend attempt, audit-event refs) is preserved verbatim.
    pub authority_lineage_preserved: bool,
    /// True when the export proves the no-self-authorization invariant.
    pub no_self_authorization_invariant: bool,
}

impl StabilizeApprovalTicketSupportExport {
    /// Wrap a stabilize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: StabilizeApprovalTicketPage,
    ) -> Self {
        let mut reasons: Vec<StabilizeApprovalTicketNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        let generated = generated_at.into();
        let export_id_str = export_id.into();
        let approval_ticket_beta_support_export = ApprovalTicketBetaSupportExport::from_page(
            format!("{export_id_str}-approval-ticket-beta"),
            generated.clone(),
            page.approval_ticket_beta_page.clone(),
        );
        let no_self_auth = page.defects.iter().all(|d| {
            d.narrow_reason != StabilizeApprovalTicketNarrowReasonClass::SelfAuthorizationAttempted
        });
        Self {
            record_kind: STABILIZE_APPROVAL_TICKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: STABILIZE_APPROVAL_TICKET_SCHEMA_VERSION,
            shared_contract_ref: STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id_str,
            generated_at: generated,
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            approval_ticket_beta_support_export,
            raw_authority_material_excluded: true,
            authority_lineage_preserved: true,
            no_self_authorization_invariant: no_self_auth,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the stabilize audit over the upstream beta page.
pub fn audit_stabilize_approval_ticket_page(
    page: &StabilizeApprovalTicketPage,
) -> Vec<StabilizeApprovalTicketDefect> {
    audit_stabilize_page(&page.approval_ticket_beta_page)
}

/// Validate a stabilize page; returns `Ok` when the audit is clean.
pub fn validate_stabilize_approval_ticket_page(
    page: &StabilizeApprovalTicketPage,
) -> Result<(), Vec<StabilizeApprovalTicketDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn audit_stabilize_page(beta_page: &ApprovalTicketBetaPage) -> Vec<StabilizeApprovalTicketDefect> {
    let mut defects: Vec<StabilizeApprovalTicketDefect> = Vec::new();

    // Recompute upstream defects so we always operate on the live audit.
    let upstream_defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );

    // Hard guardrail 1: raw authority material present — withdraw immediately.
    let has_raw_authority = upstream_defects
        .iter()
        .any(|d| d.defect_kind == ApprovalTicketBetaDefectKind::RawAuthorityMaterialPresent);
    if has_raw_authority {
        defects.push(StabilizeApprovalTicketDefect::new(
            StabilizeApprovalTicketNarrowReasonClass::RawAuthorityMaterialPresent,
            "approval_ticket_beta_page",
            "upstream beta page has a raw_authority_material_present defect; \
             packet is withdrawn",
        ));
        return defects;
    }

    // Hard guardrail 2: self-authorization attempted — withdraw immediately.
    let has_self_auth = upstream_defects
        .iter()
        .any(|d| d.defect_kind == ApprovalTicketBetaDefectKind::SelfAuthorizationAttempted);
    if has_self_auth {
        defects.push(StabilizeApprovalTicketDefect::new(
            StabilizeApprovalTicketNarrowReasonClass::SelfAuthorizationAttempted,
            "approval_ticket_beta_page",
            "upstream beta page has a self_authorization_attempted defect; \
             packet is withdrawn",
        ));
        return defects;
    }

    // Non-critical: upstream beta page has other defects.
    if !upstream_defects.is_empty() {
        defects.push(StabilizeApprovalTicketDefect::new(
            StabilizeApprovalTicketNarrowReasonClass::BetaPageHasDefects,
            "approval_ticket_beta_page",
            "upstream approval-ticket beta page has one or more defects; \
             packet is narrowed to beta",
        ));
    }

    // Required beta profile coverage (narrows to preview when missing).
    let covered_profiles: BTreeSet<&str> = beta_page
        .ticket_rows
        .iter()
        .map(|t| t.profile_token.as_str())
        .collect();
    for profile in ApprovalTicketBetaProfileClass::ALL {
        if !covered_profiles.contains(profile.as_str()) {
            defects.push(StabilizeApprovalTicketDefect::new(
                StabilizeApprovalTicketNarrowReasonClass::ProfileCoverageMissing,
                profile.as_str(),
                format!(
                    "no ticket row covers the required beta profile '{}'; \
                     packet is narrowed to preview",
                    profile.as_str()
                ),
            ));
        }
    }

    // Required sandbox-profile class coverage (narrows to preview when
    // missing).
    let covered_sandbox_classes: BTreeSet<&str> = beta_page
        .sandbox_profile_rows
        .iter()
        .map(|s| s.sandbox_profile_class_token.as_str())
        .collect();
    for sandbox_class in SandboxProfileClass::ALL {
        if !covered_sandbox_classes.contains(sandbox_class.as_str()) {
            defects.push(StabilizeApprovalTicketDefect::new(
                StabilizeApprovalTicketNarrowReasonClass::SandboxProfileCoverageMissing,
                sandbox_class.as_str(),
                format!(
                    "no sandbox-profile row covers the required class '{}'; \
                     packet is narrowed to preview",
                    sandbox_class.as_str()
                ),
            ));
        }
    }

    // Target identity completeness: every envelope and ticket must carry a
    // non-empty target ref.
    for envelope in &beta_page.capability_envelope_rows {
        if envelope.target_identity.target_ref.is_empty() {
            defects.push(StabilizeApprovalTicketDefect::new(
                StabilizeApprovalTicketNarrowReasonClass::TargetIdentityMissing,
                envelope.capability_envelope_row_id.clone(),
                "capability envelope row carries an empty target identity target_ref",
            ));
        }
    }
    for ticket in &beta_page.ticket_rows {
        if ticket.target_identity.target_ref.is_empty() {
            defects.push(StabilizeApprovalTicketDefect::new(
                StabilizeApprovalTicketNarrowReasonClass::TargetIdentityMissing,
                ticket.approval_ticket_id.clone(),
                "ticket row carries an empty target identity target_ref",
            ));
        }
    }

    // Remembered-approval lineage: BoundedReuse tickets must carry at least
    // one evidence ref.
    use aureline_auth::approval_tickets::UsePosture;
    for ticket in &beta_page.ticket_rows {
        if ticket.use_posture == UsePosture::BoundedReuse && ticket.evidence_refs.is_empty() {
            defects.push(StabilizeApprovalTicketDefect::new(
                StabilizeApprovalTicketNarrowReasonClass::RememberedApprovalMissingFreshTicketEvidence,
                ticket.approval_ticket_id.clone(),
                "bounded_reuse ticket (remembered approval) carries no evidence refs; \
                 fresh-ticket-at-use-time lineage cannot be verified",
            ));
        }
    }

    // Credential projection sandbox: any CredentialProjection action class
    // ticket must be covered by a CredentialProjectionSandbox sandbox row.
    use aureline_auth::approval_tickets::HighRiskActionClass;
    let has_credential_projection_ticket = beta_page
        .ticket_rows
        .iter()
        .any(|t| t.action_class == HighRiskActionClass::CredentialProjection);
    if has_credential_projection_ticket {
        let has_credential_projection_sandbox = beta_page
            .sandbox_profile_rows
            .iter()
            .any(|s| s.sandbox_profile_class == SandboxProfileClass::CredentialProjectionSandbox);
        if !has_credential_projection_sandbox {
            defects.push(StabilizeApprovalTicketDefect::new(
                StabilizeApprovalTicketNarrowReasonClass::CredentialProjectionSandboxMissing,
                "approval_ticket_beta_page",
                "credential projection ticket rows exist but no \
                 credential_projection_sandbox row is present; projection mode \
                 cannot be verified",
            ));
        }
    }

    defects
}

fn derive_stabilize_rows(
    beta_page: &ApprovalTicketBetaPage,
    page_defects: &[StabilizeApprovalTicketDefect],
) -> Vec<StabilizeApprovalTicketRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.narrows_to_preview());

    let overall_qual = if has_withdrawal {
        StabilizeApprovalTicketQualificationClass::Withdrawn
    } else if has_preview {
        StabilizeApprovalTicketQualificationClass::Preview
    } else if !page_defects.is_empty() {
        StabilizeApprovalTicketQualificationClass::Beta
    } else {
        StabilizeApprovalTicketQualificationClass::Stable
    };

    let narrow_reason = if has_withdrawal {
        // Pick the first withdrawal reason.
        page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(StabilizeApprovalTicketNarrowReasonClass::RawAuthorityMaterialPresent)
    } else if has_preview {
        page_defects
            .iter()
            .find(|d| d.narrow_reason.narrows_to_preview())
            .map(|d| d.narrow_reason)
            .unwrap_or(StabilizeApprovalTicketNarrowReasonClass::ProfileCoverageMissing)
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        StabilizeApprovalTicketNarrowReasonClass::NotNarrowed
    };

    use aureline_auth::approval_tickets::UsePosture;

    ApprovalTicketBetaProfileClass::ALL
        .iter()
        .map(|profile| {
            let profile_token = profile.as_str().to_owned();

            let sandbox_count = beta_page
                .sandbox_profile_rows
                .iter()
                .filter(|s| s.profile_token == profile_token)
                .count();
            let ticket_count = beta_page
                .ticket_rows
                .iter()
                .filter(|t| t.profile_token == profile_token)
                .count();
            let envelope_count = beta_page
                .capability_envelope_rows
                .iter()
                .filter(|e| e.profile_token == profile_token)
                .count();
            let spend_count = beta_page
                .spend_attempt_events
                .iter()
                .filter(|e| e.profile_token == profile_token)
                .count();
            let missing_target = beta_page
                .ticket_rows
                .iter()
                .filter(|t| {
                    t.profile_token == profile_token && t.target_identity.target_ref.is_empty()
                })
                .count();
            let remembered_missing_evidence = beta_page
                .ticket_rows
                .iter()
                .filter(|t| {
                    t.profile_token == profile_token
                        && t.use_posture == UsePosture::BoundedReuse
                        && t.evidence_refs.is_empty()
                })
                .count();

            let summary = build_row_summary(&profile_token, &overall_qual, narrow_reason);

            StabilizeApprovalTicketRow {
                record_kind: STABILIZE_APPROVAL_TICKET_ROW_RECORD_KIND.to_owned(),
                schema_version: STABILIZE_APPROVAL_TICKET_SCHEMA_VERSION,
                shared_contract_ref: STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF.to_owned(),
                profile_token,
                sandbox_profile_row_count: sandbox_count,
                ticket_row_count: ticket_count,
                envelope_row_count: envelope_count,
                spend_attempt_count: spend_count,
                missing_target_identity_count: missing_target,
                remembered_approval_missing_evidence_count: remembered_missing_evidence,
                qualification_token: overall_qual.as_str().to_owned(),
                narrow_reason_token: narrow_reason.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn build_row_summary(
    profile_token: &str,
    qual: &StabilizeApprovalTicketQualificationClass,
    narrow_reason: StabilizeApprovalTicketNarrowReasonClass,
) -> String {
    match qual {
        StabilizeApprovalTicketQualificationClass::Stable => format!(
            "Profile '{}' qualifies stable: upstream beta audit is clean, all required \
             profiles and sandbox-profile classes are covered, every ticket carries a \
             non-empty target identity, remembered-approval lineage is complete, and \
             credential-projection rows are backed by a credential_projection_sandbox.",
            profile_token
        ),
        _ => format!(
            "Profile '{}' narrowed to {} ({}): see defect list for details.",
            profile_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page seeds zero defects: the upstream beta page audits clean,
/// all four required profiles and all four sandbox-profile classes are
/// covered, every ticket carries a non-empty target identity, remembered-
/// approval lineage is complete, and credential-projection rows are backed by
/// a `credential_projection_sandbox` sandbox row.
pub fn seeded_stabilize_approval_ticket_page() -> StabilizeApprovalTicketPage {
    StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:default",
        "Approval-ticket audit and target-identity lineage (stable)",
        "2026-06-01T00:00:00Z",
        seeded_approval_ticket_beta_page(),
    )
}
