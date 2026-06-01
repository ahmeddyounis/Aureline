//! No-account local-use proof, managed-exit truth, deprovision-preserves-local-work,
//! and org-switch semantics.
//!
//! This module owns the final proof that local editing, user-owned exports,
//! and no-account local use remain available exactly as claimed after sign-out,
//! org-switch, seat loss, or deprovision — without requiring the user to
//! reverse-engineer which affordances survive.
//!
//! Each [`DeprovisionPreservesRow`] covers one managed-exit event
//! ([`ManagedExitEventClass`]) under one deployment profile and proves:
//!
//! 1. **Local-core continuity is explicit.** The row names which local-core
//!    capabilities (editing, search, local history, export, user preferences,
//!    no-account BYOK) remain available, which become read-only, and which
//!    require a user-initiated export before the affordance closes.
//! 2. **Org-scoped affordances are listed before they disappear.** The row
//!    discloses which org-bound capabilities (collab, managed AI, policy
//!    enforcement, seat-bound extensions, managed secret broker) are removed or
//!    narrowed, and surfaces that disclosure before data loss could occur.
//! 3. **Export and profile paths survive.** User-owned exports, local history,
//!    local settings, and local user preferences are never silently purged on
//!    managed exit. A row that claims otherwise is a defect.
//! 4. **Managed convenience does not block local-core work.** No managed exit
//!    event may block the local editor, local file system, or no-account BYOK
//!    lane by default.
//!
//! Two hard guardrails:
//!
//! - **No silent local-work loss.** A row whose
//!   [`LocalWorkPreservationClass`] is `silently_purged` or a row that claims
//!   local editing is unavailable post-exit without a prior export opportunity
//!   withdraws to
//!   [`DeprovisionProofQualificationClass::Withdrawn`].
//! - **No blocking exit.** A managed exit event that blocks the local editor
//!   or the account-free local lane by default withdraws the row immediately.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! tokens, closed-vocabulary tokens, plain-language labels, and opaque refs
//! only. Raw credentials, session tokens, plaintext user identity, raw tenant
//! configuration, and raw provisioning payloads stay outside the support
//! boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/finalize-no-account-local-use-proof-deprovision-preserves.md`
//! - Artifact summary: `artifacts/enterprise/m4/finalize-no-account-local-use-proof-deprovision-preserves.md`
//! - Contract refs consumed: [`DEPROVISION_PRESERVES_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const DEPROVISION_PRESERVES_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const DEPROVISION_PRESERVES_SHARED_CONTRACT_REF: &str =
    "auth:deprovision_preserves_local_work:v1";

/// Record-kind tag for [`DeprovisionPreservesBetaPage`] payloads.
pub const DEPROVISION_PRESERVES_BETA_PAGE_RECORD_KIND: &str =
    "auth_deprovision_preserves_beta_page_record";

/// Record-kind tag for [`DeprovisionPreservesRow`] payloads.
pub const DEPROVISION_PRESERVES_BETA_ROW_RECORD_KIND: &str =
    "auth_deprovision_preserves_beta_row_record";

/// Record-kind tag for [`DeprovisionPreservesBetaDefect`] payloads.
pub const DEPROVISION_PRESERVES_BETA_DEFECT_RECORD_KIND: &str =
    "auth_deprovision_preserves_beta_defect_record";

/// Record-kind tag for [`DeprovisionPreservesBetaSummary`] payloads.
pub const DEPROVISION_PRESERVES_BETA_SUMMARY_RECORD_KIND: &str =
    "auth_deprovision_preserves_beta_summary_record";

/// Record-kind tag for [`DeprovisionPreservesBetaSupportExport`] payloads.
pub const DEPROVISION_PRESERVES_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_deprovision_preserves_beta_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const DEPROVISION_PRESERVES_DOC_REF: &str =
    "docs/enterprise/m4/finalize-no-account-local-use-proof-deprovision-preserves.md";

/// Repo-relative path of the artifact summary for this lane.
pub const DEPROVISION_PRESERVES_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/finalize-no-account-local-use-proof-deprovision-preserves.md";

// ---------------------------------------------------------------------------
// Profile vocabulary
// ---------------------------------------------------------------------------

/// Deployment profile under which a deprovision-preserves row is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprovisionPreservesBetaProfileClass {
    /// Connected profile with live managed authority and provisioning paths.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline or air-gapped profile using imported bundles and snapshots.
    Offline,
    /// Enterprise-managed profile applying signed managed policy narrowing.
    EnterpriseManaged,
}

impl DeprovisionPreservesBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

// ---------------------------------------------------------------------------
// Managed-exit event vocabulary
// ---------------------------------------------------------------------------

/// Typed class of managed-exit event that triggers the local-work preservation
/// check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedExitEventClass {
    /// The user explicitly signed out from the current managed identity.
    SignOut,
    /// The user switched to a different org, tenant, or workspace identity.
    OrgSwitch,
    /// The user's managed seat was revoked by an admin or org policy.
    SeatLoss,
    /// The user's account was deprovisioned (SCIM delete, admin-initiated, or
    /// signed-file revocation).
    Deprovision,
    /// The row is an account-free local lane with no managed identity; exit
    /// events do not apply.
    AccountFreeLocalNoManagedExit,
}

impl ManagedExitEventClass {
    /// All managed exit event classes that apply to managed rows.
    pub const MANAGED_EXIT_EVENTS: [Self; 4] = [
        Self::SignOut,
        Self::OrgSwitch,
        Self::SeatLoss,
        Self::Deprovision,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignOut => "sign_out",
            Self::OrgSwitch => "org_switch",
            Self::SeatLoss => "seat_loss",
            Self::Deprovision => "deprovision",
            Self::AccountFreeLocalNoManagedExit => "account_free_local_no_managed_exit",
        }
    }

    /// True when the event class represents a managed identity being removed
    /// or replaced, so local-work preservation must be proven.
    pub const fn requires_local_work_preservation_proof(self) -> bool {
        matches!(
            self,
            Self::SignOut | Self::OrgSwitch | Self::SeatLoss | Self::Deprovision
        )
    }
}

// ---------------------------------------------------------------------------
// Local-work preservation vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary describing how a local-core capability is preserved after
/// a managed-exit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalWorkPreservationClass {
    /// The capability remains fully available after the exit event; no user
    /// action is required.
    PreservedUnchanged,
    /// The capability is available in read-only mode after the exit event.
    PreservedReadOnly,
    /// The capability is available until the user initiates an export; after
    /// export the affordance closes gracefully. The user MUST be given an
    /// export opportunity before closure.
    ExportAvailableThenClosed,
    /// The capability is not applicable for this row (e.g. account-free local
    /// lane does not have a managed-seat-bound capability).
    NotApplicable,
    /// The capability is silently purged without an export opportunity.
    ///
    /// **Hard guardrail**: This value immediately withdraws the row to
    /// [`DeprovisionProofQualificationClass::Withdrawn`].
    SilentlyPurged,
}

impl LocalWorkPreservationClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedUnchanged => "preserved_unchanged",
            Self::PreservedReadOnly => "preserved_read_only",
            Self::ExportAvailableThenClosed => "export_available_then_closed",
            Self::NotApplicable => "not_applicable",
            Self::SilentlyPurged => "silently_purged",
        }
    }

    /// True when this preservation class satisfies the no-silent-purge
    /// invariant (i.e. the row is not withdrawn).
    pub const fn satisfies_no_silent_purge(self) -> bool {
        !matches!(self, Self::SilentlyPurged)
    }

    /// True when the user has an explicit export opportunity before the
    /// affordance closes.
    pub const fn has_export_opportunity(self) -> bool {
        matches!(
            self,
            Self::PreservedUnchanged
                | Self::PreservedReadOnly
                | Self::ExportAvailableThenClosed
                | Self::NotApplicable
        )
    }
}

// ---------------------------------------------------------------------------
// Org-scoped affordance vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary describing what happens to an org-scoped affordance
/// after a managed-exit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrgAffordanceClass {
    /// The affordance is removed immediately on managed exit; the user is
    /// informed before the event completes.
    RemovedWithNotice,
    /// The affordance is narrowed (e.g. collab becomes read-only, AI becomes
    /// BYOK-only) and the user is informed.
    NarrowedWithNotice,
    /// The affordance was never present in this profile or row; removal is
    /// not applicable.
    NotApplicable,
    /// The affordance is removed without explicit notice.
    ///
    /// **Hard guardrail**: This value withdraws the row if the affordance is
    /// user-visible and data-bearing (e.g. collab session with unsaved work).
    RemovedWithoutNotice,
}

impl OrgAffordanceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemovedWithNotice => "removed_with_notice",
            Self::NarrowedWithNotice => "narrowed_with_notice",
            Self::NotApplicable => "not_applicable",
            Self::RemovedWithoutNotice => "removed_without_notice",
        }
    }

    /// True when the affordance disappears from this row without warning the
    /// user — triggers a defect in data-bearing lanes.
    pub const fn is_silent_removal(self) -> bool {
        matches!(self, Self::RemovedWithoutNotice)
    }
}

// ---------------------------------------------------------------------------
// Qualification vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier derived by the audit for the overall packet
/// and for individual rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprovisionProofQualificationClass {
    /// Every local-work preservation and org-affordance invariant holds for
    /// every managed-exit event and every required profile.
    Stable,
    /// One or more non-critical gaps prevent the stable claim.
    Beta,
    /// Structural gaps prevent a beta claim.
    Preview,
    /// A hard guardrail (silent local-work purge, blocking exit) withdrew
    /// the row entirely.
    Withdrawn,
}

impl DeprovisionProofQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this qualification tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when the row is in a claimable posture (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed narrow reason for a row qualified below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprovisionProofNarrowReasonClass {
    /// No narrowing required — the row qualifies as stable.
    NotNarrowed,
    /// A local-core capability is silently purged on managed exit.
    LocalWorkSilentlyPurged,
    /// A managed exit event blocks the local editor or account-free lane.
    ManagedExitBlocksLocalCore,
    /// A user-visible, data-bearing org affordance is removed without notice.
    DataBearingAffordanceRemovedWithoutNotice,
    /// The export path is unavailable before the managed-exit affordance
    /// closes.
    ExportPathUnavailableBeforeClose,
    /// The row is missing a required local-work survival block.
    LocalWorkSurvivalBlockMissing,
    /// The row is missing a required org-affordance block.
    OrgAffordanceBlockMissing,
    /// The local-continuity posture is inconsistent with the exit event class.
    LocalContinuityPostureInconsistent,
    /// Profile coverage is incomplete (required profile not present).
    RequiredProfileCoverageIncomplete,
}

impl DeprovisionProofNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::LocalWorkSilentlyPurged => "local_work_silently_purged",
            Self::ManagedExitBlocksLocalCore => "managed_exit_blocks_local_core",
            Self::DataBearingAffordanceRemovedWithoutNotice => {
                "data_bearing_affordance_removed_without_notice"
            }
            Self::ExportPathUnavailableBeforeClose => "export_path_unavailable_before_close",
            Self::LocalWorkSurvivalBlockMissing => "local_work_survival_block_missing",
            Self::OrgAffordanceBlockMissing => "org_affordance_block_missing",
            Self::LocalContinuityPostureInconsistent => "local_continuity_posture_inconsistent",
            Self::RequiredProfileCoverageIncomplete => "required_profile_coverage_incomplete",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the row.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::LocalWorkSilentlyPurged | Self::ManagedExitBlocksLocalCore
        )
    }
}

// ---------------------------------------------------------------------------
// Local-work survival block
// ---------------------------------------------------------------------------

/// Describes how local-core capabilities survive a managed-exit event.
///
/// Every row whose [`ManagedExitEventClass`] requires a preservation proof MUST
/// carry one of these blocks. The block names the preservation class for each
/// required local-core capability and discloses whether a prior-export
/// opportunity exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalWorkSurvivalBlock {
    /// Whether local editing (buffer, file system, undo history) is preserved.
    pub local_editing_token: String,
    /// Whether user-owned local export paths (file exports, git commits)
    /// survive or require a pre-exit export.
    pub local_export_paths_token: String,
    /// Whether local history and undo-tree snapshots survive.
    pub local_history_token: String,
    /// Whether local user settings and preferences survive.
    pub local_settings_token: String,
    /// Whether account-free BYOK (bring-your-own-key) lane remains available.
    pub account_free_byok_token: String,
    /// True when the user is offered at least one explicit export opportunity
    /// before any capability closes.
    pub prior_export_opportunity: bool,
    /// Plain-language note on what survives, what requires export, and what
    /// (if anything) closes.
    pub survival_summary: String,
}

// ---------------------------------------------------------------------------
// Org-scoped affordance block
// ---------------------------------------------------------------------------

/// Describes what happens to org-scoped affordances when a managed-exit
/// event occurs.
///
/// Every row whose exit event has org-scoped affordances MUST carry one of
/// these blocks. The block names the outcome class for each major org-bound
/// capability family and discloses whether notice is given before removal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrgScopedAffordanceBlock {
    /// What happens to managed collab sessions and shared workspace links.
    pub collab_session_token: String,
    /// What happens to managed AI features (non-BYOK, policy-governed).
    pub managed_ai_token: String,
    /// What happens to seat-bound extension licenses and capabilities.
    pub seat_bound_extensions_token: String,
    /// What happens to managed secret broker handles (vault, keychain scopes).
    pub managed_secret_broker_token: String,
    /// What happens to active policy enforcement (admin policy packs).
    pub policy_enforcement_token: String,
    /// True when the user is notified of affordance removal before the
    /// managed-exit event completes.
    pub removal_notice_given: bool,
    /// Plain-language note on which org affordances disappear and when.
    pub affordance_summary: String,
}

// ---------------------------------------------------------------------------
// Row
// ---------------------------------------------------------------------------

/// Proof row for one managed-exit event type under one deployment profile.
///
/// Rows are paired with a profile so the proof covers connected, mirror-only,
/// offline, and enterprise-managed postures independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprovisionPreservesRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    /// Unique row id for this (event × profile) pair.
    pub row_id: String,
    /// Managed-exit event class for this row.
    pub exit_event_token: String,
    /// Deployment profile class for this row.
    pub profile_token: String,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// How local-core capabilities survive. Required for every row where
    /// [`ManagedExitEventClass::requires_local_work_preservation_proof`] is
    /// true.
    pub local_work_survival: LocalWorkSurvivalBlock,
    /// How org-scoped affordances are handled. Required for every row where
    /// the profile carries org-scoped capabilities.
    pub org_affordance: OrgScopedAffordanceBlock,
    /// Plain-language summary of what the exit event changes.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the deprovision-preserves page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DeprovisionPreservesBetaSummary {
    pub row_count: usize,
    pub stable_row_count: usize,
    pub beta_row_count: usize,
    pub preview_row_count: usize,
    pub withdrawn_row_count: usize,
    pub profiles_covered: Vec<String>,
    pub exit_events_covered: Vec<String>,
    pub local_work_preserved_count: usize,
    pub org_affordance_removes_with_notice_count: usize,
    pub overall_qualification_token: String,
}

impl DeprovisionPreservesBetaSummary {
    fn from_rows(rows: &[DeprovisionPreservesRow]) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut profiles: Vec<String> = Vec::new();
        let mut events: Vec<String> = Vec::new();
        let mut local_preserved = 0usize;
        let mut org_removes_with_notice = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            if !profiles.contains(&row.profile_token) {
                profiles.push(row.profile_token.clone());
            }
            if !events.contains(&row.exit_event_token) {
                events.push(row.exit_event_token.clone());
            }
            if row.local_work_survival.local_editing_token
                == LocalWorkPreservationClass::PreservedUnchanged.as_str()
            {
                local_preserved += 1;
            }
            if row.org_affordance.removal_notice_given {
                org_removes_with_notice += 1;
            }
        }
        profiles.sort();
        events.sort();
        let overall = if withdrawn > 0 {
            DeprovisionProofQualificationClass::Withdrawn
        } else if preview > 0 {
            DeprovisionProofQualificationClass::Preview
        } else if beta > 0 {
            DeprovisionProofQualificationClass::Beta
        } else {
            DeprovisionProofQualificationClass::Stable
        };
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            profiles_covered: profiles,
            exit_events_covered: events,
            local_work_preserved_count: local_preserved,
            org_affordance_removes_with_notice_count: org_removes_with_notice,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defects
// ---------------------------------------------------------------------------

/// Typed defect emitted by the deprovision-preserves audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprovisionPreservesBetaDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub narrow_reason: DeprovisionProofNarrowReasonClass,
    pub narrow_reason_token: String,
    pub source_row_id: String,
    pub note: String,
}

impl DeprovisionPreservesBetaDefect {
    fn new(
        narrow_reason: DeprovisionProofNarrowReasonClass,
        source_row_id: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let row_id_str = source_row_id.into();
        Self {
            record_kind: DEPROVISION_PRESERVES_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: DEPROVISION_PRESERVES_SCHEMA_VERSION,
            shared_contract_ref: DEPROVISION_PRESERVES_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "auth:defect:deprovision-preserves:{}:{}",
                narrow_reason.as_str(),
                &row_id_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source_row_id: row_id_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Proof packet that no-account local-use, export paths, and local-core work
/// survive every managed-exit event across all required deployment profiles.
///
/// This is the single inspectable record that proves the stable claim for this
/// lane. Dashboards, docs, Help/About surfaces, and support exports should
/// ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprovisionPreservesBetaPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub page_label: String,
    pub generated_at: String,
    pub summary: DeprovisionPreservesBetaSummary,
    pub rows: Vec<DeprovisionPreservesRow>,
    pub defects: Vec<DeprovisionPreservesBetaDefect>,
}

impl DeprovisionPreservesBetaPage {
    /// Build the page from a set of rows. Defects are derived automatically.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<DeprovisionPreservesRow>,
    ) -> Self {
        let defects = audit_deprovision_preserves_rows(&rows);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary = DeprovisionPreservesBetaSummary::from_rows(&qualified_rows);
        Self {
            record_kind: DEPROVISION_PRESERVES_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: DEPROVISION_PRESERVES_SCHEMA_VERSION,
            shared_contract_ref: DEPROVISION_PRESERVES_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            defects,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == DeprovisionProofQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when local editing is marked preserved-unchanged for every managed
    /// exit event row across all required profiles.
    pub fn local_editing_preserved_across_all_exit_events(&self) -> bool {
        self.rows
            .iter()
            .filter(|r| r.exit_event_token != ManagedExitEventClass::AccountFreeLocalNoManagedExit.as_str())
            .all(|r| {
                r.local_work_survival.local_editing_token
                    == LocalWorkPreservationClass::PreservedUnchanged.as_str()
            })
    }

    /// True when every managed exit event row discloses a prior export
    /// opportunity.
    pub fn prior_export_opportunity_present_for_all_exits(&self) -> bool {
        self.rows
            .iter()
            .filter(|r| r.exit_event_token != ManagedExitEventClass::AccountFreeLocalNoManagedExit.as_str())
            .all(|r| r.local_work_survival.prior_export_opportunity)
    }

    /// True when every org-affordance removal gives the user explicit notice.
    pub fn org_affordance_removal_gives_notice(&self) -> bool {
        self.rows
            .iter()
            .filter(|r| r.exit_event_token != ManagedExitEventClass::AccountFreeLocalNoManagedExit.as_str())
            .all(|r| r.org_affordance.removal_notice_given)
    }

    /// True when the four required deployment profiles are covered for every
    /// managed exit event.
    pub fn all_required_profiles_covered(&self) -> bool {
        let required: Vec<&str> = DeprovisionPreservesBetaProfileClass::ALL
            .iter()
            .map(|p| p.as_str())
            .collect();
        for event in &ManagedExitEventClass::MANAGED_EXIT_EVENTS {
            for profile in &required {
                let found = self.rows.iter().any(|r| {
                    r.exit_event_token == event.as_str() && &r.profile_token.as_str() == profile
                });
                if !found {
                    return false;
                }
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the page plus a metadata-safe defect
/// roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprovisionPreservesBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub page: DeprovisionPreservesBetaPage,
    pub narrow_reasons_present: Vec<DeprovisionProofNarrowReasonClass>,
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    pub raw_private_material_excluded: bool,
}

impl DeprovisionPreservesBetaSupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: DeprovisionPreservesBetaPage,
    ) -> Self {
        let mut reasons: Vec<DeprovisionProofNarrowReasonClass> = Vec::new();
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
        Self {
            record_kind: DEPROVISION_PRESERVES_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEPROVISION_PRESERVES_SCHEMA_VERSION,
            shared_contract_ref: DEPROVISION_PRESERVES_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate
// ---------------------------------------------------------------------------

/// Run the deprovision-preserves audit over a set of rows and return any
/// defects found.
pub fn audit_deprovision_preserves_rows(
    rows: &[DeprovisionPreservesRow],
) -> Vec<DeprovisionPreservesBetaDefect> {
    let mut defects: Vec<DeprovisionPreservesBetaDefect> = Vec::new();

    for row in rows {
        // Skip account-free local sentinel rows from event-based checks.
        if row.exit_event_token == ManagedExitEventClass::AccountFreeLocalNoManagedExit.as_str() {
            continue;
        }

        // Check 1: local editing must be preserved-unchanged on every managed exit.
        if row.local_work_survival.local_editing_token
            == LocalWorkPreservationClass::SilentlyPurged.as_str()
        {
            defects.push(DeprovisionPreservesBetaDefect::new(
                DeprovisionProofNarrowReasonClass::LocalWorkSilentlyPurged,
                &row.row_id,
                format!(
                    "row '{}': local editing is silently purged on exit event '{}'; row is withdrawn",
                    row.row_id, row.exit_event_token
                ),
            ));
        }

        // Check 2: local export paths must not be silently purged.
        if row.local_work_survival.local_export_paths_token
            == LocalWorkPreservationClass::SilentlyPurged.as_str()
        {
            defects.push(DeprovisionPreservesBetaDefect::new(
                DeprovisionProofNarrowReasonClass::LocalWorkSilentlyPurged,
                &row.row_id,
                format!(
                    "row '{}': local export paths are silently purged on exit event '{}'; row is withdrawn",
                    row.row_id, row.exit_event_token
                ),
            ));
        }

        // Check 3: managed exit must not block local editing (preserved-unchanged required).
        if row.local_work_survival.local_editing_token
            != LocalWorkPreservationClass::PreservedUnchanged.as_str()
            && row.local_work_survival.local_editing_token
                != LocalWorkPreservationClass::NotApplicable.as_str()
        {
            defects.push(DeprovisionPreservesBetaDefect::new(
                DeprovisionProofNarrowReasonClass::ManagedExitBlocksLocalCore,
                &row.row_id,
                format!(
                    "row '{}': managed exit event '{}' does not preserve local editing as unchanged; row is withdrawn",
                    row.row_id, row.exit_event_token
                ),
            ));
        }

        // Check 4: prior export opportunity must be present.
        if !row.local_work_survival.prior_export_opportunity {
            defects.push(DeprovisionPreservesBetaDefect::new(
                DeprovisionProofNarrowReasonClass::ExportPathUnavailableBeforeClose,
                &row.row_id,
                format!(
                    "row '{}': exit event '{}' does not provide a prior export opportunity before affordance close",
                    row.row_id, row.exit_event_token
                ),
            ));
        }

        // Check 5: org affordance removal must be accompanied by notice.
        if row.org_affordance.collab_session_token
            == OrgAffordanceClass::RemovedWithoutNotice.as_str()
            || row.org_affordance.managed_ai_token
                == OrgAffordanceClass::RemovedWithoutNotice.as_str()
        {
            defects.push(DeprovisionPreservesBetaDefect::new(
                DeprovisionProofNarrowReasonClass::DataBearingAffordanceRemovedWithoutNotice,
                &row.row_id,
                format!(
                    "row '{}': a data-bearing org affordance (collab or managed AI) is removed without notice on exit event '{}'",
                    row.row_id, row.exit_event_token
                ),
            ));
        }
    }

    defects
}

/// Validate a page; returns `Ok` on a clean audit.
pub fn validate_deprovision_preserves_beta_page(
    page: &DeprovisionPreservesBetaPage,
) -> Result<(), Vec<DeprovisionPreservesBetaDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Requalify rows after the page-level audit has run. Rows that participate in
/// a hard-guardrail defect are withdrawn; rows that participate in a soft
/// defect are narrowed to beta; clean rows stay stable.
fn qualify_rows(
    mut rows: Vec<DeprovisionPreservesRow>,
    defects: &[DeprovisionPreservesBetaDefect],
) -> Vec<DeprovisionPreservesRow> {
    let withdrawn_ids: Vec<&str> = defects
        .iter()
        .filter(|d| d.narrow_reason.is_withdrawal_reason())
        .map(|d| d.source_row_id.as_str())
        .collect();
    let beta_ids: Vec<&str> = defects
        .iter()
        .filter(|d| !d.narrow_reason.is_withdrawal_reason())
        .map(|d| d.source_row_id.as_str())
        .collect();

    for row in &mut rows {
        if withdrawn_ids.contains(&row.row_id.as_str()) {
            row.qualification_token =
                DeprovisionProofQualificationClass::Withdrawn.as_str().to_owned();
            let reason = defects
                .iter()
                .find(|d| d.source_row_id == row.row_id && d.narrow_reason.is_withdrawal_reason())
                .map(|d| d.narrow_reason)
                .unwrap_or(DeprovisionProofNarrowReasonClass::LocalWorkSilentlyPurged);
            row.narrow_reason_token = reason.as_str().to_owned();
        } else if beta_ids.contains(&row.row_id.as_str()) {
            if row.qualification_token == DeprovisionProofQualificationClass::Stable.as_str() {
                row.qualification_token =
                    DeprovisionProofQualificationClass::Beta.as_str().to_owned();
                let reason = defects
                    .iter()
                    .find(|d| d.source_row_id == row.row_id)
                    .map(|d| d.narrow_reason)
                    .unwrap_or(DeprovisionProofNarrowReasonClass::NotNarrowed);
                row.narrow_reason_token = reason.as_str().to_owned();
            }
        }
    }
    rows
}

/// Build a clean local-work survival block for a managed exit event row.
fn clean_local_work_survival(exit_event: ManagedExitEventClass) -> LocalWorkSurvivalBlock {
    let event_label = exit_event.as_str();
    LocalWorkSurvivalBlock {
        local_editing_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        local_export_paths_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        local_history_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        local_settings_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        account_free_byok_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        prior_export_opportunity: true,
        survival_summary: format!(
            "On '{event_label}': local editing, export paths, local history, \
             settings, and account-free BYOK lane remain fully available. \
             User retains all local work without data loss."
        ),
    }
}

/// Build a clean org-affordance block for a managed exit event row.
fn clean_org_affordance(exit_event: ManagedExitEventClass) -> OrgScopedAffordanceBlock {
    let event_label = exit_event.as_str();
    OrgScopedAffordanceBlock {
        collab_session_token: OrgAffordanceClass::RemovedWithNotice.as_str().to_owned(),
        managed_ai_token: OrgAffordanceClass::NarrowedWithNotice.as_str().to_owned(),
        seat_bound_extensions_token: OrgAffordanceClass::RemovedWithNotice.as_str().to_owned(),
        managed_secret_broker_token: OrgAffordanceClass::RemovedWithNotice.as_str().to_owned(),
        policy_enforcement_token: OrgAffordanceClass::RemovedWithNotice.as_str().to_owned(),
        removal_notice_given: true,
        affordance_summary: format!(
            "On '{event_label}': managed collab sessions, seat-bound extensions, \
             managed secret-broker handles, and policy enforcement are removed with \
             explicit notice before the event completes. Managed AI narrows to \
             account-free BYOK with notice."
        ),
    }
}

/// Build a no-managed-exit local-only block for account-free rows.
fn account_free_local_survival() -> LocalWorkSurvivalBlock {
    LocalWorkSurvivalBlock {
        local_editing_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        local_export_paths_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        local_history_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        local_settings_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        account_free_byok_token: LocalWorkPreservationClass::PreservedUnchanged.as_str().to_owned(),
        prior_export_opportunity: true,
        survival_summary: "Account-free local lane: no managed identity; all local-core \
                           capabilities are permanently available without any exit event."
            .to_owned(),
    }
}

/// Build an account-free affordance block (all N/A).
fn account_free_org_affordance() -> OrgScopedAffordanceBlock {
    OrgScopedAffordanceBlock {
        collab_session_token: OrgAffordanceClass::NotApplicable.as_str().to_owned(),
        managed_ai_token: OrgAffordanceClass::NotApplicable.as_str().to_owned(),
        seat_bound_extensions_token: OrgAffordanceClass::NotApplicable.as_str().to_owned(),
        managed_secret_broker_token: OrgAffordanceClass::NotApplicable.as_str().to_owned(),
        policy_enforcement_token: OrgAffordanceClass::NotApplicable.as_str().to_owned(),
        removal_notice_given: true,
        affordance_summary: "Account-free local lane: no org-scoped affordances are present; \
                             removal notice is not applicable."
            .to_owned(),
    }
}

/// Build a seeded row for a given (event × profile) pair.
fn seeded_row(
    exit_event: ManagedExitEventClass,
    profile: DeprovisionPreservesBetaProfileClass,
) -> DeprovisionPreservesRow {
    let row_id = format!(
        "auth:deprovision_preserves:{}:{}",
        exit_event.as_str(),
        profile.as_str()
    );
    let (local_survival, org_affordance) =
        if exit_event == ManagedExitEventClass::AccountFreeLocalNoManagedExit {
            (account_free_local_survival(), account_free_org_affordance())
        } else {
            (
                clean_local_work_survival(exit_event),
                clean_org_affordance(exit_event),
            )
        };
    let summary = format!(
        "Row '{}': exit event '{}' under profile '{}'. \
         Local editing, history, export paths, settings, and BYOK lane: preserved-unchanged. \
         Org affordances removed/narrowed with explicit notice before event completes.",
        row_id,
        exit_event.as_str(),
        profile.as_str()
    );
    DeprovisionPreservesRow {
        record_kind: DEPROVISION_PRESERVES_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: DEPROVISION_PRESERVES_SCHEMA_VERSION,
        shared_contract_ref: DEPROVISION_PRESERVES_SHARED_CONTRACT_REF.to_owned(),
        row_id,
        exit_event_token: exit_event.as_str().to_owned(),
        profile_token: profile.as_str().to_owned(),
        qualification_token: DeprovisionProofQualificationClass::Stable.as_str().to_owned(),
        narrow_reason_token: DeprovisionProofNarrowReasonClass::NotNarrowed.as_str().to_owned(),
        local_work_survival: local_survival,
        org_affordance,
        plain_language_summary: summary,
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded proof packet that the live shell, the headless inspector,
/// and the integration test all consume.
///
/// The seeded page covers every managed-exit event across all four required
/// deployment profiles plus the account-free local lane, seeds zero defects,
/// and derives a stable qualification.
pub fn seeded_deprovision_preserves_beta_page() -> DeprovisionPreservesBetaPage {
    let mut rows: Vec<DeprovisionPreservesRow> = Vec::new();

    // Account-free local sentinel — one row, one profile (local only).
    rows.push(seeded_row(
        ManagedExitEventClass::AccountFreeLocalNoManagedExit,
        DeprovisionPreservesBetaProfileClass::Connected,
    ));

    // Managed exit events × all four required profiles.
    for event in &ManagedExitEventClass::MANAGED_EXIT_EVENTS {
        for profile in &DeprovisionPreservesBetaProfileClass::ALL {
            rows.push(seeded_row(*event, *profile));
        }
    }

    DeprovisionPreservesBetaPage::new(
        "auth:deprovision_preserves:default",
        "No-account local-use proof, managed-exit truth, deprovision-preserves-local-work, \
         and org-switch semantics (beta)",
        "2026-06-01T00:00:00Z",
        rows,
    )
}
