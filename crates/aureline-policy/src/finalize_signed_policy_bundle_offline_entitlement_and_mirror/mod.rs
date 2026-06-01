//! Finalize signed policy-bundle, offline-entitlement grace, and
//! mirror/manual-import verification flows.
//!
//! This module produces a beta proof packet that demonstrates:
//!
//! 1. Signed policy bundles surface source, signer, last-known-good revision,
//!    grace window, expiry, and blocked-capability consequences.
//! 2. Offline entitlement staleness is labeled explicitly and never disguised
//!    as a generic auth failure.
//! 3. Mirror and manual-import verification flows preserve the same
//!    inspect/export vocabulary as online policy distribution: policy epoch,
//!    digest, trust root, and last successful validation time.
//! 4. Enterprise, self-hosted, and air-gapped rows keep local-core continuity
//!    explicit while surfacing which managed capabilities are outside grace or
//!    now blocked.
//! 5. Signed policy-bundle and manual-import flows expose a simulation packet
//!    before apply: changed feature areas, previous versus simulated values,
//!    affected surfaces, degraded-mode consequences, and offline or stale-policy
//!    notes.
//! 6. Bundle import or policy refresh may not widen enterprise-bearing or
//!    managed-bearing claims until the simulation packet, approval owner, and
//!    expiry posture are inspectable.
//!
//! Surfaces (admin console, support export, shell trust center, headless
//! inspector) read [`seeded_finalize_signed_policy_bundle_page`] rather than
//! minting parallel bundle-signed checks. The seed covers all five required
//! import flows (online, mirror, manual import, air-gapped, offline grace)
//! for both policy bundles and entitlement snapshots and proves that:
//!
//! - every failure mode downgrades managed authority without losing the
//!   local-editing floor;
//! - entitlement staleness is never surfaced as a generic authentication
//!   error;
//! - pre-apply simulation packets are present and inspectable for all import
//!   paths before any apply is permitted.
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

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF: &str =
    "policy:signed_policy_bundle_finalize:v1";

/// Record-kind tag for [`FinalizeSignedPolicyBundlePage`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_PAGE_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_page_record";

/// Record-kind tag for [`FinalizeSignedPolicyBundleRow`] payloads.
pub const SIGNED_POLICY_BUNDLE_FINALIZE_ROW_RECORD_KIND: &str =
    "policy_signed_policy_bundle_finalize_row_record";

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

// ---------------------------------------------------------------------------
// Import-flow vocabulary
// ---------------------------------------------------------------------------

/// Import flow that produced the bundle under inspection.
///
/// This is the delivery path for the signed artifact, not the runtime
/// verification outcome. The verifier outcome is captured in the embedded
/// [`OfflineEntitlementVerifierBetaPage`].
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
    /// Bundle used under a declared offline-grace window; the primary path
    /// is unavailable and the last-known-good bundle is extended within the
    /// grace window.
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

    /// True when this flow requires an explicit grace window declaration when
    /// the bundle is operating outside its primary validation window.
    pub const fn requires_grace_window_when_stale(self) -> bool {
        matches!(self, Self::AirGapped | Self::OfflineGrace)
    }

    /// True when this flow may result in widening managed claims (i.e., the
    /// import could introduce capabilities not already active).
    pub const fn may_widen_managed_claims(self) -> bool {
        matches!(
            self,
            Self::Online | Self::Mirror | Self::ManualImport | Self::AirGapped
        )
    }
}

// ---------------------------------------------------------------------------
// Bundle kind vocabulary
// ---------------------------------------------------------------------------

/// Kind of signed bundle the row inspects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleKindClass {
    /// Signed policy bundle authorising managed narrowing rules.
    PolicyBundle,
    /// Signed entitlement snapshot describing plan, seat, and quota state.
    EntitlementSnapshot,
}

impl BundleKindClass {
    /// All required bundle kinds in canonical order.
    pub const ALL: [Self; 2] = [Self::PolicyBundle, Self::EntitlementSnapshot];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBundle => "policy_bundle",
            Self::EntitlementSnapshot => "entitlement_snapshot",
        }
    }
}

// ---------------------------------------------------------------------------
// Grace posture vocabulary
// ---------------------------------------------------------------------------

/// Offline-grace posture for the bundle under inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GracePostureClass {
    /// Bundle is current and verified; no grace extension is active.
    NotInGrace,
    /// Bundle is within its declared grace window; last-known-good authority
    /// is extended while the primary path is unavailable.
    InGrace,
    /// Bundle's grace window has expired; managed authority is now narrowed.
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

    /// True when the bundle is operating outside its verified window (either
    /// within the grace extension or after the grace has expired).
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::InGrace | Self::GraceExpired)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Qualification tier for the finalize page and its rows.
///
/// The tier is derived, not asserted: it is set by the audit against the six
/// required conditions. A caller may never assert `stable` without a clean
/// audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSignedPolicyBundleQualificationClass {
    /// All required conditions hold and the upstream verifier audit is clean.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// A required import flow has no row; coverage gap prevents a beta claim.
    Preview,
    /// A hard guardrail was triggered; the page is withdrawn immediately.
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
/// [`FinalizeSignedPolicyBundleQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSignedPolicyBundleNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// The upstream offline-entitlement verifier page has defects.
    OfflineEntitlementVerifierHasDefects,
    /// An offline or air-gapped row has no declared grace window when the
    /// bundle is operating outside its primary validation window.
    GraceWindowNotDeclared,
    /// A stale bundle row has an empty `staleness_label`, making staleness
    /// indistinguishable from a generic auth failure. This is a hard
    /// guardrail and withdraws the packet.
    StalenessDisguisedAsAuthFailure,
    /// A row is missing one or more epoch inspection fields (epoch ref,
    /// digest, trust root, or last-successful-validation time).
    PolicyEpochNotInspectable,
    /// A row's simulation packet has no affected surfaces, indicating the
    /// pre-apply inspection was not run or was not exported.
    SimulationPacketMissingBeforeApply,
    /// A row's simulation packet reports `widens_managed_claims: true` but
    /// the `approval_owner_ref` is empty.
    ApprovalOwnerMissingOnWidening,
    /// A row's simulation packet has an empty `expiry_posture_token`.
    ExpiryPostureNotInspectable,
    /// A row does not carry `local_core_continuity_explicit: true`.
    LocalCoreContinuityNotExplicit,
    /// A required import flow has no rows; narrows to preview.
    ImportFlowCoverageGap,
    /// Raw private material was exposed in the upstream verifier page;
    /// withdraws the packet immediately.
    RawPrivateMaterialExposed,
}

impl FinalizeSignedPolicyBundleNarrowReasonClass {
    /// Stable token recorded on every serialized record.
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
            Self::ApprovalOwnerMissingOnWidening => "approval_owner_missing_on_widening",
            Self::ExpiryPostureNotInspectable => "expiry_posture_not_inspectable",
            Self::LocalCoreContinuityNotExplicit => "local_core_continuity_not_explicit",
            Self::ImportFlowCoverageGap => "import_flow_coverage_gap",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
        }
    }

    /// True when this reason triggers immediate withdrawal and cannot be
    /// overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed | Self::StalenessDisguisedAsAuthFailure
        )
    }
}

// ---------------------------------------------------------------------------
// Policy epoch state
// ---------------------------------------------------------------------------

/// Epoch inspection state for the bundle under inspection.
///
/// Preserves the same inspect/export vocabulary across all import flows so
/// mirror, manual-import, and air-gapped rows carry the same fields as online
/// rows.
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
    /// True when all four required epoch inspection fields are non-empty.
    pub fn is_fully_inspectable(&self) -> bool {
        !self.epoch_ref.is_empty()
            && !self.epoch_digest.is_empty()
            && !self.trust_root_ref.is_empty()
            && !self.last_successful_validation_time.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Offline grace state
// ---------------------------------------------------------------------------

/// Grace-window and staleness state for the bundle under inspection.
///
/// When a bundle is operating outside its primary validation window (either
/// within the declared grace period or after grace has expired), this block
/// carries the grace window bounds, the last-known-good revision ref, an
/// explicit staleness label, and the list of managed capabilities that will
/// be blocked when the grace window closes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineGraceState {
    /// Current grace posture.
    pub grace_posture: GracePostureClass,
    /// Stable token for [`Self::grace_posture`].
    pub grace_posture_token: String,
    /// Timestamp when the grace window started; empty when `not_in_grace`.
    pub grace_window_start: String,
    /// Timestamp when the grace window ends; must be non-empty when in grace
    /// or grace-expired.
    pub grace_window_end: String,
    /// Opaque ref to the last-known-good verified bundle revision.
    pub last_known_good_revision: String,
    /// Explicit staleness label surfaced in UI and support exports. Must be
    /// non-empty when [`GracePostureClass::is_stale`] is true; the label
    /// must not be a generic authentication-failure message.
    pub staleness_label: String,
    /// Export-safe labels for managed capabilities that are blocked when the
    /// grace window expires or the bundle cannot be re-verified.
    pub blocked_capability_consequences: Vec<String>,
}

impl OfflineGraceState {
    /// True when the staleness label is explicit (non-empty) for stale rows.
    ///
    /// A stale row with an empty `staleness_label` would disguise entitlement
    /// staleness as a generic failure; this function returns `false` in that
    /// case.
    pub fn staleness_is_explicitly_labeled(&self) -> bool {
        if self.grace_posture.is_stale() {
            !self.staleness_label.is_empty()
        } else {
            true
        }
    }

    /// True when the grace window bounds are declared for stale rows.
    ///
    /// A stale row without `grace_window_end` cannot prove the grace window
    /// is finite.
    pub fn grace_window_is_declared(&self) -> bool {
        if self.grace_posture.is_stale() {
            !self.grace_window_end.is_empty()
        } else {
            true
        }
    }
}

// ---------------------------------------------------------------------------
// Pre-apply simulation packet
// ---------------------------------------------------------------------------

/// Pre-apply simulation packet required before any bundle import or policy
/// refresh is applied.
///
/// Every row carries one simulation packet. For non-widening refreshes the
/// packet summarises "no material change" with `changed_feature_areas` and
/// `previous_values_summary` empty; `affected_surfaces` must still be
/// non-empty to prove the inspection was run.
///
/// When `widens_managed_claims` is `true`, the packet must additionally carry
/// a non-empty `approval_owner_ref` and a non-empty `expiry_posture_token`
/// before any apply is permitted.
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
    /// Plain-language labels for degraded-mode consequences if the apply
    /// is performed without a full verification path.
    pub degraded_mode_consequences: Vec<String>,
    /// Notes specific to offline or stale-policy apply paths.
    pub offline_or_stale_policy_notes: Vec<String>,
    /// Opaque ref to the approval owner who authorised the apply.
    /// Must be non-empty when `widens_managed_claims` is `true`.
    pub approval_owner_ref: String,
    /// Stable token describing the expiry posture after apply. Must be
    /// non-empty for all rows.
    pub expiry_posture_token: String,
    /// True when this import or refresh would widen enterprise-bearing or
    /// managed-bearing claims beyond the currently active bundle.
    pub widens_managed_claims: bool,
    /// True when this packet is present and inspectable before the apply
    /// is permitted.
    pub inspectable_before_apply: bool,
}

// ---------------------------------------------------------------------------
// Row, summary, defect
// ---------------------------------------------------------------------------

/// Finalize row for one `(import_flow × bundle_kind)` pair.
///
/// The row is the unit of qualification. Each row must carry a fully
/// inspectable epoch state, an explicit grace state, and a pre-apply
/// simulation packet. Failure on any required condition narrows the row
/// and the page.
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
    /// Pre-apply simulation packet for this row.
    pub simulation_packet: PolicyBundleSimulationPacket,
    /// True when local-core continuity is stated explicitly on this row.
    ///
    /// Enterprise, self-hosted, and air-gapped rows must name the
    /// local-editing floor explicitly so it cannot be silently removed by
    /// a managed capability change.
    pub local_core_continuity_explicit: bool,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
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
    /// Import flow tokens present on the page.
    pub import_flows_covered: Vec<String>,
    /// Bundle-kind tokens present on the page.
    pub bundle_kinds_covered: Vec<String>,
    /// Number of rows where the bundle is stale (in grace or grace-expired).
    pub stale_bundle_row_count: usize,
    /// Number of rows with fully inspectable epoch states.
    pub epoch_inspectable_row_count: usize,
    /// Number of rows with a simulation packet that covers affected surfaces.
    pub simulation_packet_present_row_count: usize,
    /// Number of rows with `local_core_continuity_explicit: true`.
    pub local_core_continuity_explicit_row_count: usize,
    /// Defect count from the upstream offline-entitlement verifier page.
    pub upstream_verifier_defect_count: usize,
    /// Overall qualification token derived from all rows and defects.
    pub overall_qualification_token: String,
}

impl FinalizeSignedPolicyBundleSummary {
    fn from_rows(
        rows: &[FinalizeSignedPolicyBundleRow],
        verifier_page: &OfflineEntitlementVerifierBetaPage,
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut import_flows: BTreeSet<String> = BTreeSet::new();
        let mut bundle_kinds: BTreeSet<String> = BTreeSet::new();
        let mut stale = 0usize;
        let mut epoch_ok = 0usize;
        let mut sim_ok = 0usize;
        let mut local_core_ok = 0usize;

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
            if row.grace_state.grace_posture.is_stale() {
                stale += 1;
            }
            if row.epoch_state.is_fully_inspectable() {
                epoch_ok += 1;
            }
            if !row.simulation_packet.affected_surfaces.is_empty() {
                sim_ok += 1;
            }
            if row.local_core_continuity_explicit {
                local_core_ok += 1;
            }
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
            stale_bundle_row_count: stale,
            epoch_inspectable_row_count: epoch_ok,
            simulation_packet_present_row_count: sim_ok,
            local_core_continuity_explicit_row_count: local_core_ok,
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
    /// Subject id (row id, import flow, or `page`).
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

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Beta proof packet for signed policy-bundle, offline-entitlement grace, and
/// mirror/manual-import verification flows.
///
/// This is the single inspectable record that proves the claims for this lane.
/// Dashboards, docs, Help/About surfaces, and support exports should ingest it
/// rather than cloning status text.
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
    /// Typed validation defects for this packet.
    pub defects: Vec<FinalizeSignedPolicyBundleDefect>,
    /// Upstream offline-entitlement verifier page embedded as evidence.
    pub offline_entitlement_verifier_page: OfflineEntitlementVerifierBetaPage,
}

impl FinalizeSignedPolicyBundlePage {
    /// Build the finalize page from a set of rows and an embedded verifier page.
    ///
    /// Defects are derived automatically from the audit. Rows are
    /// re-qualified based on the combined audit result.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<FinalizeSignedPolicyBundleRow>,
        offline_entitlement_verifier_page: OfflineEntitlementVerifierBetaPage,
    ) -> Self {
        let defects = audit_finalize_rows(&rows, &offline_entitlement_verifier_page);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary =
            FinalizeSignedPolicyBundleSummary::from_rows(&qualified_rows, &offline_entitlement_verifier_page);
        Self {
            record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_PAGE_RECORD_KIND.to_owned(),
            schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
            shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            defects,
            offline_entitlement_verifier_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == FinalizeSignedPolicyBundleQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all required import flows are covered.
    pub fn covers_all_required_import_flows(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|r| r.import_flow_token.as_str())
            .collect();
        BundleImportFlowClass::ALL
            .iter()
            .all(|f| covered.contains(f.as_str()))
    }

    /// True when all rows carry fully inspectable epoch states.
    pub fn all_epoch_states_inspectable(&self) -> bool {
        self.rows.iter().all(|r| r.epoch_state.is_fully_inspectable())
    }

    /// True when all simulation packets have at least one affected surface.
    pub fn all_simulation_packets_have_affected_surfaces(&self) -> bool {
        self.rows
            .iter()
            .all(|r| !r.simulation_packet.affected_surfaces.is_empty())
    }

    /// True when all rows carry `local_core_continuity_explicit: true`.
    pub fn all_rows_explicit_on_local_core_continuity(&self) -> bool {
        self.rows.iter().all(|r| r.local_core_continuity_explicit)
    }

    /// True when all stale rows carry explicit staleness labels.
    pub fn stale_rows_are_explicitly_labeled(&self) -> bool {
        self.rows
            .iter()
            .all(|r| r.grace_state.staleness_is_explicitly_labeled())
    }

    /// True when all stale rows declare a bounded grace window.
    pub fn stale_rows_have_declared_grace_windows(&self) -> bool {
        self.rows
            .iter()
            .all(|r| r.grace_state.grace_window_is_declared())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the finalize page plus a metadata-safe
/// defect roll-up.
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
        let mut reasons: Vec<FinalizeSignedPolicyBundleNarrowReasonClass> = Vec::new();
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
            record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
            shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
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
// Public audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the finalize audit over the rows and embedded verifier page.
pub fn audit_finalize_signed_policy_bundle_page(
    page: &FinalizeSignedPolicyBundlePage,
) -> Vec<FinalizeSignedPolicyBundleDefect> {
    audit_finalize_rows(&page.rows, &page.offline_entitlement_verifier_page)
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

/// Build the seeded finalize page covering all five required import flows for
/// both policy bundles and entitlement snapshots, including offline-grace and
/// pre-apply simulation packets for every row.
pub fn seeded_finalize_signed_policy_bundle_page() -> FinalizeSignedPolicyBundlePage {
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let rows = seeded_rows();
    FinalizeSignedPolicyBundlePage::new(
        "policy:signed-policy-bundle-finalize:seeded:0001",
        "Signed policy-bundle, offline-entitlement, and mirror/manual-import finalize packet",
        "2026-06-01T00:00:00Z",
        rows,
        verifier_page,
    )
}

// ---------------------------------------------------------------------------
// Internal audit helpers
// ---------------------------------------------------------------------------

fn audit_finalize_rows(
    rows: &[FinalizeSignedPolicyBundleRow],
    verifier_page: &OfflineEntitlementVerifierBetaPage,
) -> Vec<FinalizeSignedPolicyBundleDefect> {
    let mut defects: Vec<FinalizeSignedPolicyBundleDefect> = Vec::new();

    // Hard guardrail: raw private material exposed in upstream verifier page.
    let upstream_defects = audit_offline_entitlement_verifier_beta_rows(&verifier_page.rows);
    let has_raw_material = upstream_defects
        .iter()
        .any(|d| d.defect_kind == OfflineEntitlementVerifierBetaDefectKind::RawPrivateMaterialExposed);
    if has_raw_material {
        defects.push(FinalizeSignedPolicyBundleDefect::new(
            FinalizeSignedPolicyBundleNarrowReasonClass::RawPrivateMaterialExposed,
            "offline_entitlement_verifier_page",
            "upstream offline-entitlement verifier page has a raw_private_material_exposed defect; packet is withdrawn",
        ));
        return defects;
    }

    // Non-critical: upstream verifier page has other defects.
    if !verifier_page.defects.is_empty() {
        defects.push(FinalizeSignedPolicyBundleDefect::new(
            FinalizeSignedPolicyBundleNarrowReasonClass::OfflineEntitlementVerifierHasDefects,
            "offline_entitlement_verifier_page",
            "upstream offline-entitlement verifier page has defects; packet is narrowed to beta",
        ));
    }

    for row in rows {
        // Hard guardrail: stale bundle without explicit staleness label.
        if !row.grace_state.staleness_is_explicitly_labeled() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::StalenessDisguisedAsAuthFailure,
                row.row_id.clone(),
                "stale bundle row has an empty staleness_label; staleness must not be disguised as a generic auth failure",
            ));
        }

        // Grace window not declared for flows that require it.
        if row.import_flow.requires_grace_window_when_stale()
            && !row.grace_state.grace_window_is_declared()
        {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::GraceWindowNotDeclared,
                row.row_id.clone(),
                "offline or air-gapped row has a stale bundle with no declared grace window end",
            ));
        }

        // Policy epoch inspection fields must all be present.
        if !row.epoch_state.is_fully_inspectable() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::PolicyEpochNotInspectable,
                row.row_id.clone(),
                "row is missing one or more required epoch inspection fields (epoch_ref, epoch_digest, trust_root_ref, last_successful_validation_time)",
            ));
        }

        // Simulation packet must have at least one affected surface.
        if row.simulation_packet.affected_surfaces.is_empty() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::SimulationPacketMissingBeforeApply,
                row.row_id.clone(),
                "simulation packet has no affected surfaces; pre-apply inspection was not exported",
            ));
        }

        // Widening rows require an approval owner.
        if row.simulation_packet.widens_managed_claims
            && row.simulation_packet.approval_owner_ref.is_empty()
        {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::ApprovalOwnerMissingOnWidening,
                row.row_id.clone(),
                "simulation packet widens managed claims but approval_owner_ref is empty",
            ));
        }

        // Expiry posture must be declared.
        if row.simulation_packet.expiry_posture_token.is_empty() {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::ExpiryPostureNotInspectable,
                row.row_id.clone(),
                "simulation packet has an empty expiry_posture_token",
            ));
        }

        // Local-core continuity must be stated explicitly.
        if !row.local_core_continuity_explicit {
            defects.push(FinalizeSignedPolicyBundleDefect::new(
                FinalizeSignedPolicyBundleNarrowReasonClass::LocalCoreContinuityNotExplicit,
                row.row_id.clone(),
                "row does not carry local_core_continuity_explicit: true",
            ));
        }
    }

    // Coverage check: all required import flows must appear at least once.
    let required_flows: BTreeSet<&str> =
        BundleImportFlowClass::ALL.iter().map(|f| f.as_str()).collect();
    let observed_flows: BTreeSet<&str> =
        rows.iter().map(|r| r.import_flow_token.as_str()).collect();
    for missing in required_flows.difference(&observed_flows) {
        defects.push(FinalizeSignedPolicyBundleDefect::new(
            FinalizeSignedPolicyBundleNarrowReasonClass::ImportFlowCoverageGap,
            "page",
            format!("missing rows for required import flow '{missing}'; packet is narrowed to preview"),
        ));
    }

    defects
}

fn qualify_rows(
    mut rows: Vec<FinalizeSignedPolicyBundleRow>,
    page_defects: &[FinalizeSignedPolicyBundleDefect],
) -> Vec<FinalizeSignedPolicyBundleRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects.iter().any(|d| {
        d.narrow_reason == FinalizeSignedPolicyBundleNarrowReasonClass::ImportFlowCoverageGap
    });

    let (overall_qual, overall_reason) = if has_withdrawal {
        let r = page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(FinalizeSignedPolicyBundleNarrowReasonClass::RawPrivateMaterialExposed);
        (FinalizeSignedPolicyBundleQualificationClass::Withdrawn, r)
    } else if has_preview {
        (
            FinalizeSignedPolicyBundleQualificationClass::Preview,
            FinalizeSignedPolicyBundleNarrowReasonClass::ImportFlowCoverageGap,
        )
    } else if !page_defects.is_empty() {
        let r = page_defects[0].narrow_reason;
        (FinalizeSignedPolicyBundleQualificationClass::Beta, r)
    } else {
        (
            FinalizeSignedPolicyBundleQualificationClass::Stable,
            FinalizeSignedPolicyBundleNarrowReasonClass::NotNarrowed,
        )
    };

    for row in &mut rows {
        // Per-row defects may narrow below the page overall, but for
        // withdrawal and preview the page-level reason takes precedence.
        let row_qual = if has_withdrawal {
            FinalizeSignedPolicyBundleQualificationClass::Withdrawn
        } else if has_preview {
            FinalizeSignedPolicyBundleQualificationClass::Preview
        } else {
            let row_has_defect = page_defects.iter().any(|d| {
                d.source == row.row_id
                    && d.narrow_reason
                        != FinalizeSignedPolicyBundleNarrowReasonClass::OfflineEntitlementVerifierHasDefects
            });
            if row_has_defect || !page_defects.is_empty() {
                FinalizeSignedPolicyBundleQualificationClass::Beta
            } else {
                FinalizeSignedPolicyBundleQualificationClass::Stable
            }
        };

        let row_reason = if row_qual == overall_qual {
            overall_reason
        } else {
            page_defects
                .iter()
                .find(|d| d.source == row.row_id)
                .map(|d| d.narrow_reason)
                .unwrap_or(FinalizeSignedPolicyBundleNarrowReasonClass::NotNarrowed)
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary = build_row_summary(&row.row_id, &row.import_flow_token, &row.bundle_kind_token, row_qual, row_reason);
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    import_flow_token: &str,
    bundle_kind_token: &str,
    qual: FinalizeSignedPolicyBundleQualificationClass,
    narrow_reason: FinalizeSignedPolicyBundleNarrowReasonClass,
) -> String {
    match qual {
        FinalizeSignedPolicyBundleQualificationClass::Stable => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) qualifies stable: \
             epoch state inspectable, grace state declared, simulation packet present, \
             local-core continuity explicit, upstream verifier clean."
        ),
        FinalizeSignedPolicyBundleQualificationClass::Beta => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) narrowed to beta \
             (reason: {}): one or more required conditions are unmet.",
            narrow_reason.as_str()
        ),
        FinalizeSignedPolicyBundleQualificationClass::Preview => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) narrowed to preview: \
             a required import flow is missing from the page."
        ),
        FinalizeSignedPolicyBundleQualificationClass::Withdrawn => format!(
            "Row '{row_id}' ({import_flow_token}/{bundle_kind_token}) is withdrawn \
             (reason: {}): hard guardrail triggered.",
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded rows
// ---------------------------------------------------------------------------

fn seeded_rows() -> Vec<FinalizeSignedPolicyBundleRow> {
    vec![
        row_online_policy(),
        row_online_entitlement(),
        row_mirror_policy(),
        row_mirror_entitlement(),
        row_manual_import_policy(),
        row_manual_import_entitlement(),
        row_air_gapped_policy(),
        row_air_gapped_entitlement(),
        row_offline_grace_policy(),
        row_offline_grace_entitlement(),
    ]
}

fn make_row(
    row_id: &str,
    import_flow: BundleImportFlowClass,
    bundle_kind: BundleKindClass,
    signer_ref: &str,
    signed_at: &str,
    valid_until: &str,
    epoch_ref: &str,
    epoch_digest: &str,
    trust_root_ref: &str,
    last_validation: &str,
    grace_posture: GracePostureClass,
    grace_start: &str,
    grace_end: &str,
    last_known_good: &str,
    staleness_label: &str,
    blocked_consequences: Vec<&str>,
    packet_id: &str,
    changed_feature_areas: Vec<&str>,
    affected_surfaces: Vec<&str>,
    degraded_mode_consequences: Vec<&str>,
    offline_or_stale_notes: Vec<&str>,
    approval_owner_ref: &str,
    expiry_posture_token: &str,
    widens_managed_claims: bool,
) -> FinalizeSignedPolicyBundleRow {
    FinalizeSignedPolicyBundleRow {
        record_kind: SIGNED_POLICY_BUNDLE_FINALIZE_ROW_RECORD_KIND.to_owned(),
        schema_version: SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
        shared_contract_ref: SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        import_flow,
        import_flow_token: import_flow.as_str().to_owned(),
        bundle_kind,
        bundle_kind_token: bundle_kind.as_str().to_owned(),
        signer_ref: signer_ref.to_owned(),
        signed_at: signed_at.to_owned(),
        valid_until: valid_until.to_owned(),
        epoch_state: PolicyEpochState {
            epoch_ref: epoch_ref.to_owned(),
            epoch_digest: epoch_digest.to_owned(),
            trust_root_ref: trust_root_ref.to_owned(),
            last_successful_validation_time: last_validation.to_owned(),
            import_flow_token: import_flow.as_str().to_owned(),
        },
        grace_state: OfflineGraceState {
            grace_posture,
            grace_posture_token: grace_posture.as_str().to_owned(),
            grace_window_start: grace_start.to_owned(),
            grace_window_end: grace_end.to_owned(),
            last_known_good_revision: last_known_good.to_owned(),
            staleness_label: staleness_label.to_owned(),
            blocked_capability_consequences: blocked_consequences
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
        },
        simulation_packet: PolicyBundleSimulationPacket {
            packet_id: packet_id.to_owned(),
            changed_feature_areas: changed_feature_areas
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            previous_values_summary: BTreeMap::new(),
            simulated_values_summary: BTreeMap::new(),
            affected_surfaces: affected_surfaces
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            degraded_mode_consequences: degraded_mode_consequences
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            offline_or_stale_policy_notes: offline_or_stale_notes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            approval_owner_ref: approval_owner_ref.to_owned(),
            expiry_posture_token: expiry_posture_token.to_owned(),
            widens_managed_claims,
            inspectable_before_apply: true,
        },
        local_core_continuity_explicit: true,
        // Qualification and summary are filled in by qualify_rows.
        qualification_token: FinalizeSignedPolicyBundleQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: FinalizeSignedPolicyBundleNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn row_online_policy() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:online:policy_bundle",
        BundleImportFlowClass::Online,
        BundleKindClass::PolicyBundle,
        "signer:vendor-managed-baseline",
        "2026-05-01T00:00:00Z",
        "2026-08-01T00:00:00Z",
        "epoch:policy.online.2026.05.0001",
        "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        "trust_root:vendor-managed-root:2026",
        "2026-05-31T12:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "policy_bundle.online.2026.05.0001",
        "",
        vec![],
        "sim-packet:online:policy:2026.05.0001",
        vec![],
        vec!["policy_inspect_panel", "admin_trust_center"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_online_entitlement() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:online:entitlement_snapshot",
        BundleImportFlowClass::Online,
        BundleKindClass::EntitlementSnapshot,
        "signer:vendor-managed-entitlement",
        "2026-05-01T00:00:00Z",
        "2026-06-01T00:00:00Z",
        "epoch:entitlement.online.2026.05.0001",
        "sha256:1111111111111111111111111111111111111111111111111111111111111111",
        "trust_root:vendor-managed-root:2026",
        "2026-05-31T12:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "entitlement_snapshot.online.2026.05.0001",
        "",
        vec![],
        "sim-packet:online:entitlement:2026.05.0001",
        vec![],
        vec!["entitlement_inspect_panel", "seat_status_indicator"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_mirror_policy() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:mirror:policy_bundle",
        BundleImportFlowClass::Mirror,
        BundleKindClass::PolicyBundle,
        "signer:vendor-managed-baseline",
        "2026-05-01T00:00:00Z",
        "2026-08-01T00:00:00Z",
        "epoch:policy.mirror.2026.05.0001",
        "sha256:2222222222222222222222222222222222222222222222222222222222222222",
        "trust_root:signed-mirror-root:2026",
        "2026-05-30T08:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "policy_bundle.mirror.2026.05.0001",
        "",
        vec![],
        "sim-packet:mirror:policy:2026.05.0001",
        vec![],
        vec!["policy_inspect_panel", "mirror_trust_indicator"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_mirror_entitlement() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:mirror:entitlement_snapshot",
        BundleImportFlowClass::Mirror,
        BundleKindClass::EntitlementSnapshot,
        "signer:vendor-managed-entitlement",
        "2026-05-01T00:00:00Z",
        "2026-06-01T00:00:00Z",
        "epoch:entitlement.mirror.2026.05.0001",
        "sha256:3333333333333333333333333333333333333333333333333333333333333333",
        "trust_root:signed-mirror-root:2026",
        "2026-05-30T08:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "entitlement_snapshot.mirror.2026.05.0001",
        "",
        vec![],
        "sim-packet:mirror:entitlement:2026.05.0001",
        vec![],
        vec!["entitlement_inspect_panel", "mirror_trust_indicator"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_manual_import_policy() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:manual_import:policy_bundle",
        BundleImportFlowClass::ManualImport,
        BundleKindClass::PolicyBundle,
        "signer:vendor-managed-baseline",
        "2026-05-15T00:00:00Z",
        "2026-09-15T00:00:00Z",
        "epoch:policy.manual-import.2026.05.0002",
        "sha256:4444444444444444444444444444444444444444444444444444444444444444",
        "trust_root:vendor-managed-root:2026",
        "2026-05-15T14:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "policy_bundle.manual-import.2026.05.0002",
        "",
        vec![],
        "sim-packet:manual-import:policy:2026.05.0002",
        vec!["policy_inspect_panel"],
        vec!["policy_inspect_panel", "admin_trust_center", "import_review_panel"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_manual_import_entitlement() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:manual_import:entitlement_snapshot",
        BundleImportFlowClass::ManualImport,
        BundleKindClass::EntitlementSnapshot,
        "signer:vendor-managed-entitlement",
        "2026-05-15T00:00:00Z",
        "2026-07-15T00:00:00Z",
        "epoch:entitlement.manual-import.2026.05.0002",
        "sha256:5555555555555555555555555555555555555555555555555555555555555555",
        "trust_root:vendor-managed-root:2026",
        "2026-05-15T14:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "entitlement_snapshot.manual-import.2026.05.0002",
        "",
        vec![],
        "sim-packet:manual-import:entitlement:2026.05.0002",
        vec![],
        vec!["entitlement_inspect_panel", "import_review_panel"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_air_gapped_policy() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:air_gapped:policy_bundle",
        BundleImportFlowClass::AirGapped,
        BundleKindClass::PolicyBundle,
        "signer:vendor-managed-baseline",
        "2026-04-01T00:00:00Z",
        "2026-10-01T00:00:00Z",
        "epoch:policy.airgapped.2026.04.0001",
        "sha256:6666666666666666666666666666666666666666666666666666666666666666",
        "trust_root:air-gapped-root:2026",
        "2026-04-01T10:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "policy_bundle.airgapped.2026.04.0001",
        "",
        vec![],
        "sim-packet:air-gapped:policy:2026.04.0001",
        vec![],
        vec!["policy_inspect_panel", "offline_trust_indicator"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_air_gapped_entitlement() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:air_gapped:entitlement_snapshot",
        BundleImportFlowClass::AirGapped,
        BundleKindClass::EntitlementSnapshot,
        "signer:vendor-managed-entitlement",
        "2026-04-01T00:00:00Z",
        "2026-10-01T00:00:00Z",
        "epoch:entitlement.airgapped.2026.04.0001",
        "sha256:7777777777777777777777777777777777777777777777777777777777777777",
        "trust_root:air-gapped-root:2026",
        "2026-04-01T10:00:00Z",
        GracePostureClass::NotInGrace,
        "",
        "",
        "entitlement_snapshot.airgapped.2026.04.0001",
        "",
        vec![],
        "sim-packet:air-gapped:entitlement:2026.04.0001",
        vec![],
        vec!["entitlement_inspect_panel", "offline_trust_indicator"],
        vec![],
        vec![],
        "",
        "current_and_verified",
        false,
    )
}

fn row_offline_grace_policy() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:offline_grace:policy_bundle",
        BundleImportFlowClass::OfflineGrace,
        BundleKindClass::PolicyBundle,
        "signer:vendor-managed-baseline",
        "2026-03-01T00:00:00Z",
        "2026-05-01T00:00:00Z",
        "epoch:policy.offline-grace.2026.03.0001",
        "sha256:8888888888888888888888888888888888888888888888888888888888888888",
        "trust_root:vendor-managed-root:2026",
        "2026-03-01T00:00:00Z",
        GracePostureClass::InGrace,
        "2026-05-01T00:00:00Z",
        "2026-07-01T00:00:00Z",
        "policy_bundle.offline-grace.2026.03.0001",
        "Policy bundle is within the declared offline-grace window (expires 2026-07-01). \
         Managed narrowing rules remain active at last-known-good revision. \
         Bundle renewal is required before the grace window closes.",
        vec![
            "managed_ai_features (blocked after grace expiry)",
            "policy_enforcement_admin_overrides (blocked after grace expiry)",
        ],
        "sim-packet:offline-grace:policy:2026.03.0001",
        vec![],
        vec!["policy_inspect_panel", "offline_grace_indicator"],
        vec![
            "Managed policy features remain at last-known-good revision; \
             no new policy narrowing rules apply during the grace window.",
        ],
        vec![
            "Bundle is within offline grace window; renewal required before 2026-07-01.",
        ],
        "",
        "in_grace_until_2026_07_01",
        false,
    )
}

fn row_offline_grace_entitlement() -> FinalizeSignedPolicyBundleRow {
    make_row(
        "signed-policy-bundle-finalize:offline_grace:entitlement_snapshot",
        BundleImportFlowClass::OfflineGrace,
        BundleKindClass::EntitlementSnapshot,
        "signer:vendor-managed-entitlement",
        "2026-03-01T00:00:00Z",
        "2026-05-01T00:00:00Z",
        "epoch:entitlement.offline-grace.2026.03.0001",
        "sha256:9999999999999999999999999999999999999999999999999999999999999999",
        "trust_root:vendor-managed-root:2026",
        "2026-03-01T00:00:00Z",
        GracePostureClass::InGrace,
        "2026-05-01T00:00:00Z",
        "2026-07-01T00:00:00Z",
        "entitlement_snapshot.offline-grace.2026.03.0001",
        "Entitlement snapshot is within the declared offline-grace window (expires 2026-07-01). \
         Seat remains active at last-known-good entitlement state. \
         Snapshot renewal is required before the grace window closes.",
        vec![
            "seat_bound_extensions (suspended after grace expiry)",
            "managed_ai_seat_quota (suspended after grace expiry)",
        ],
        "sim-packet:offline-grace:entitlement:2026.03.0001",
        vec![],
        vec!["entitlement_inspect_panel", "offline_grace_indicator"],
        vec![
            "Entitlement features remain at last-known-good state; \
             no quota or seat changes apply during the grace window.",
        ],
        vec![
            "Entitlement snapshot is within offline grace window; renewal required before 2026-07-01.",
        ],
        "",
        "in_grace_until_2026_07_01",
        false,
    )
}
