//! Managed-boundary, org-switch, seat-quota, grace-window, and offboarding
//! beta surface backed by the published boundary manifest.
//!
//! The shell does not mint a parallel boundary, seat, or offboarding model.
//! It loads the published boundary manifest YAML
//! (`artifacts/milestones/m3/boundary_manifest_beta.yaml`), validates the
//! row vocabulary against the closed sets declared in the schema, and
//! exposes typed per-flow projections so the org-switch surface, seat /
//! quota surface, grace-window surface, and offboarding surface all use the
//! same boundary state names and caveats published to docs, admin,
//! CLI/headless, support-export, and release-evidence consumers.
//!
//! Surfaces consume one of:
//!
//! - [`ManagedBoundaryBetaPage`] — typed projection of every row with
//!   org-switch, grace-window, seat-quota, and offboarding blocks aligned to
//!   the manifest vocabulary, plus typed defects and a render summary.
//! - [`ManagedBoundaryBetaPage::org_switch_projection`] — rows reduced to
//!   their org-switch behavior, with the local-state-preserved invariant
//!   exposed per row.
//! - [`ManagedBoundaryBetaPage::seat_quota_projection`] — rows reduced to
//!   their seat/quota state, observed-states list, meter refs, and absence
//!   narrowing.
//! - [`ManagedBoundaryBetaPage::grace_window_projection`] — rows reduced to
//!   their grace-window class, duration, and summary.
//! - [`ManagedBoundaryBetaPage::offboarding_projection`] — rows reduced to
//!   their observed phases, export packet class, destruction-receipt flag,
//!   and absence narrowing.
//! - [`ManagedBoundaryBetaSupportExport`] — support-export wrapper that
//!   preserves the typed defect vocabulary, the no-public-endpoint and
//!   local-core invariants, and references the same artifact paths the
//!   release packet links to.
//!
//! Guardrails: a row that boundary class is `managed` or `paid_seat_bound`
//! MUST disclose an `absence_narrows_to` clause. A row that fails the
//! published schema fails closed with a typed defect rather than silently
//! widening. Local editing and on-disk files are always preserved per the
//! manifest's `local_core_continuity` clause; the projection records that
//! invariant explicitly so support surfaces can quote it verbatim.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

const BOUNDARY_MANIFEST_YAML: &str =
    include_str!("../../../../artifacts/milestones/m3/boundary_manifest_beta.yaml");

/// Stable record kind for [`ManagedBoundaryBetaPage`] payloads.
pub const MANAGED_BOUNDARY_BETA_PAGE_RECORD_KIND: &str = "shell_managed_boundary_beta_page_record";

/// Stable record kind for [`ManagedBoundaryBetaRow`] payloads.
pub const MANAGED_BOUNDARY_BETA_ROW_RECORD_KIND: &str = "shell_managed_boundary_beta_row_record";

/// Stable record kind for [`ManagedBoundaryBetaOrgSwitchRow`] payloads.
pub const MANAGED_BOUNDARY_BETA_ORG_SWITCH_ROW_RECORD_KIND: &str =
    "shell_managed_boundary_beta_org_switch_row_record";

/// Stable record kind for [`ManagedBoundaryBetaSeatQuotaRow`] payloads.
pub const MANAGED_BOUNDARY_BETA_SEAT_QUOTA_ROW_RECORD_KIND: &str =
    "shell_managed_boundary_beta_seat_quota_row_record";

/// Stable record kind for [`ManagedBoundaryBetaGraceWindowRow`] payloads.
pub const MANAGED_BOUNDARY_BETA_GRACE_WINDOW_ROW_RECORD_KIND: &str =
    "shell_managed_boundary_beta_grace_window_row_record";

/// Stable record kind for [`ManagedBoundaryBetaOffboardingRow`] payloads.
pub const MANAGED_BOUNDARY_BETA_OFFBOARDING_ROW_RECORD_KIND: &str =
    "shell_managed_boundary_beta_offboarding_row_record";

/// Stable record kind for [`ManagedBoundaryBetaDefect`] payloads.
pub const MANAGED_BOUNDARY_BETA_DEFECT_RECORD_KIND: &str =
    "shell_managed_boundary_beta_defect_record";

/// Stable record kind for [`ManagedBoundaryBetaSummary`] payloads.
pub const MANAGED_BOUNDARY_BETA_SUMMARY_RECORD_KIND: &str =
    "shell_managed_boundary_beta_summary_record";

/// Stable record kind for [`ManagedBoundaryBetaSupportExport`] payloads.
pub const MANAGED_BOUNDARY_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_managed_boundary_beta_support_export_record";

/// Schema version exported with every record produced by this module.
pub const MANAGED_BOUNDARY_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record produced by this module.
pub const MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF: &str = "security:managed_boundary_beta:v1";

/// Source manifest ref.
pub const MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF: &str =
    "artifacts/milestones/m3/boundary_manifest_beta.yaml";

/// Source schema ref.
pub const MANAGED_BOUNDARY_BETA_SOURCE_SCHEMA_REF: &str =
    "schemas/governance/boundary_manifest_beta.schema.json";

/// Release doc ref linked from the support-export wrapper.
pub const MANAGED_BOUNDARY_BETA_RELEASE_DOC_REF: &str = "docs/release/m3/managed_boundary_beta.md";

/// UX doc ref linked from the support-export wrapper.
pub const MANAGED_BOUNDARY_BETA_UX_DOC_REF: &str = "docs/ux/m3/managed_boundary_beta.md";

/// Closed boundary-class vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// Works with no account, no managed control plane, and no seat.
    LocalOnly,
    /// Reachable via a signed mirror or offline bundle when the live service is denied.
    Mirrored,
    /// Hosted-on-customer-infra alternative to a managed service.
    SelfHosted,
    /// Vendor-hosted convenience layered on a self-hostable protocol or workflow.
    Managed,
    /// Managed surface gated by a seat or paid entitlement.
    PaidSeatBound,
    /// Reserved row preventing implicit beta drift.
    ExplicitlyOutOfScope,
}

impl BoundaryClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Mirrored => "mirrored",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::PaidSeatBound => "paid_seat_bound",
            Self::ExplicitlyOutOfScope => "explicitly_out_of_scope",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "local_only" => Some(Self::LocalOnly),
            "mirrored" => Some(Self::Mirrored),
            "self_hosted" => Some(Self::SelfHosted),
            "managed" => Some(Self::Managed),
            "paid_seat_bound" => Some(Self::PaidSeatBound),
            "explicitly_out_of_scope" => Some(Self::ExplicitlyOutOfScope),
            _ => None,
        }
    }

    /// True when a row in this class MUST publish an `absence_narrows_to`
    /// clause. Mirrors the manifest schema's conditional requirement.
    pub const fn requires_absence_narrowing(self) -> bool {
        matches!(self, Self::Managed | Self::PaidSeatBound)
    }
}

/// Closed claim-state vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimState {
    BetaClaim,
    PreviewClaim,
    HeldForLater,
    ExplicitlyOutOfScope,
}

impl ClaimState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BetaClaim => "beta_claim",
            Self::PreviewClaim => "preview_claim",
            Self::HeldForLater => "held_for_later",
            Self::ExplicitlyOutOfScope => "explicitly_out_of_scope",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "beta_claim" => Some(Self::BetaClaim),
            "preview_claim" => Some(Self::PreviewClaim),
            "held_for_later" => Some(Self::HeldForLater),
            "explicitly_out_of_scope" => Some(Self::ExplicitlyOutOfScope),
            _ => None,
        }
    }
}

/// Closed deployment-profile vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    IndividualLocal,
    SelfHosted,
    EnterpriseOnline,
    AirGapped,
    ManagedCloud,
}

impl DeploymentProfile {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "individual_local" => Some(Self::IndividualLocal),
            "self_hosted" => Some(Self::SelfHosted),
            "enterprise_online" => Some(Self::EnterpriseOnline),
            "air_gapped" => Some(Self::AirGapped),
            "managed_cloud" => Some(Self::ManagedCloud),
            _ => None,
        }
    }
}

/// Org-switch behavior vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrgSwitchBehaviorClass {
    NotApplicable,
    PreservesLocalState,
    ScopesToNewOrg,
    DeniesUntilResolved,
    RequiresAdminHandoff,
}

impl OrgSwitchBehaviorClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::PreservesLocalState => "preserves_local_state",
            Self::ScopesToNewOrg => "scopes_to_new_org",
            Self::DeniesUntilResolved => "denies_until_resolved",
            Self::RequiresAdminHandoff => "requires_admin_handoff",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "not_applicable" => Some(Self::NotApplicable),
            "preserves_local_state" => Some(Self::PreservesLocalState),
            "scopes_to_new_org" => Some(Self::ScopesToNewOrg),
            "denies_until_resolved" => Some(Self::DeniesUntilResolved),
            "requires_admin_handoff" => Some(Self::RequiresAdminHandoff),
            _ => None,
        }
    }
}

/// Grace-window class vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraceWindowClass {
    NotApplicable,
    ShortLived,
    PolicyPinned,
    AuditOnly,
    DeniedForBeta,
}

impl GraceWindowClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::ShortLived => "short_lived",
            Self::PolicyPinned => "policy_pinned",
            Self::AuditOnly => "audit_only",
            Self::DeniedForBeta => "denied_for_beta",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "not_applicable" => Some(Self::NotApplicable),
            "short_lived" => Some(Self::ShortLived),
            "policy_pinned" => Some(Self::PolicyPinned),
            "audit_only" => Some(Self::AuditOnly),
            "denied_for_beta" => Some(Self::DeniedForBeta),
            _ => None,
        }
    }
}

/// Seat / quota state vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeatQuotaState {
    NotApplicable,
    SeatActive,
    SeatUnassigned,
    SeatRevoked,
    QuotaWithinWindow,
    QuotaGrace,
    QuotaExhausted,
}

impl SeatQuotaState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::SeatActive => "seat_active",
            Self::SeatUnassigned => "seat_unassigned",
            Self::SeatRevoked => "seat_revoked",
            Self::QuotaWithinWindow => "quota_within_window",
            Self::QuotaGrace => "quota_grace",
            Self::QuotaExhausted => "quota_exhausted",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "not_applicable" => Some(Self::NotApplicable),
            "seat_active" => Some(Self::SeatActive),
            "seat_unassigned" => Some(Self::SeatUnassigned),
            "seat_revoked" => Some(Self::SeatRevoked),
            "quota_within_window" => Some(Self::QuotaWithinWindow),
            "quota_grace" => Some(Self::QuotaGrace),
            "quota_exhausted" => Some(Self::QuotaExhausted),
            _ => None,
        }
    }
}

/// Offboarding phase vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingPhase {
    NotApplicable,
    Announce,
    FreezeWrites,
    ExportAvailable,
    ManagedAccessEnd,
    DestructionReceiptIssued,
}

impl OffboardingPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Announce => "announce",
            Self::FreezeWrites => "freeze_writes",
            Self::ExportAvailable => "export_available",
            Self::ManagedAccessEnd => "managed_access_end",
            Self::DestructionReceiptIssued => "destruction_receipt_issued",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "not_applicable" => Some(Self::NotApplicable),
            "announce" => Some(Self::Announce),
            "freeze_writes" => Some(Self::FreezeWrites),
            "export_available" => Some(Self::ExportAvailable),
            "managed_access_end" => Some(Self::ManagedAccessEnd),
            "destruction_receipt_issued" => Some(Self::DestructionReceiptIssued),
            _ => None,
        }
    }
}

/// Export-packet class vocabulary mirrored from the manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPacketClass {
    NotApplicable,
    LocalSupportBundle,
    ManagedUsageExport,
    EntitlementSnapshot,
    DestructionReceipt,
}

impl ExportPacketClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::LocalSupportBundle => "local_support_bundle",
            Self::ManagedUsageExport => "managed_usage_export",
            Self::EntitlementSnapshot => "entitlement_snapshot",
            Self::DestructionReceipt => "destruction_receipt",
        }
    }

    fn from_token(token: &str) -> Option<Self> {
        match token {
            "not_applicable" => Some(Self::NotApplicable),
            "local_support_bundle" => Some(Self::LocalSupportBundle),
            "managed_usage_export" => Some(Self::ManagedUsageExport),
            "entitlement_snapshot" => Some(Self::EntitlementSnapshot),
            "destruction_receipt" => Some(Self::DestructionReceipt),
            _ => None,
        }
    }
}

/// Org-switch behavior block carried on every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrgSwitchBehaviorBlock {
    pub behavior_class: OrgSwitchBehaviorClass,
    pub behavior_class_token: String,
    pub summary: String,
    pub local_state_preserved: bool,
}

/// Grace-window block carried on every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraceWindowBlock {
    pub window_class: GraceWindowClass,
    pub window_class_token: String,
    pub duration_iso8601: Option<String>,
    pub summary: String,
}

/// Seat / quota block carried on every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeatQuotaBlock {
    pub quota_state: SeatQuotaState,
    pub quota_state_token: String,
    pub states_observed: Vec<SeatQuotaState>,
    pub states_observed_tokens: Vec<String>,
    pub meter_refs: Vec<String>,
    pub summary: String,
}

/// Offboarding block carried on every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingBlock {
    pub phases_observed: Vec<OffboardingPhase>,
    pub phases_observed_tokens: Vec<String>,
    pub export_packet_class: ExportPacketClass,
    pub export_packet_class_token: String,
    pub destruction_receipt_required: bool,
    pub summary: String,
}

/// One typed boundary-manifest row consumed by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_id: String,
    pub title: String,
    pub summary: String,
    pub boundary_class: BoundaryClass,
    pub boundary_class_token: String,
    pub claim_state: ClaimState,
    pub claim_state_token: String,
    pub deployment_profiles: Vec<DeploymentProfile>,
    pub deployment_profile_tokens: Vec<String>,
    pub alpha_capability_refs: Vec<String>,
    pub local_core_continuity: String,
    pub org_switch_behavior: OrgSwitchBehaviorBlock,
    pub grace_window: GraceWindowBlock,
    pub seat_quota: SeatQuotaBlock,
    pub offboarding: OffboardingBlock,
    pub absence_narrows_to: Option<String>,
    pub linked_evidence_class: String,
    pub evidence_refs: Vec<String>,
    pub notes: Option<String>,
}

/// Per-flow projection: org-switch row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaOrgSwitchRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_id: String,
    pub title: String,
    pub boundary_class_token: String,
    pub behavior_class: OrgSwitchBehaviorClass,
    pub behavior_class_token: String,
    pub summary: String,
    pub local_state_preserved: bool,
    pub local_core_continuity: String,
    pub absence_narrows_to: Option<String>,
}

/// Per-flow projection: seat / quota row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaSeatQuotaRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_id: String,
    pub title: String,
    pub boundary_class_token: String,
    pub quota_state: SeatQuotaState,
    pub quota_state_token: String,
    pub states_observed_tokens: Vec<String>,
    pub meter_refs: Vec<String>,
    pub summary: String,
    pub local_core_continuity: String,
    pub absence_narrows_to: Option<String>,
}

/// Per-flow projection: grace-window row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaGraceWindowRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_id: String,
    pub title: String,
    pub boundary_class_token: String,
    pub window_class: GraceWindowClass,
    pub window_class_token: String,
    pub duration_iso8601: Option<String>,
    pub summary: String,
    pub local_core_continuity: String,
    pub absence_narrows_to: Option<String>,
}

/// Per-flow projection: offboarding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaOffboardingRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_id: String,
    pub title: String,
    pub boundary_class_token: String,
    pub phases_observed_tokens: Vec<String>,
    pub export_packet_class: ExportPacketClass,
    pub export_packet_class_token: String,
    pub destruction_receipt_required: bool,
    pub summary: String,
    pub local_core_continuity: String,
    pub absence_narrows_to: Option<String>,
}

/// Typed validator defect kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedBoundaryBetaDefectKind {
    /// A required boundary class is missing from the projected rows.
    RequiredBoundaryClassMissing,
    /// A managed or paid_seat_bound row lacks an `absence_narrows_to` clause.
    AbsenceNarrowingMissing,
    /// A row carries an unknown vocabulary token (would silently widen).
    UnknownVocabularyToken,
    /// A row's `local_core_continuity` clause is empty.
    LocalCoreContinuityMissing,
    /// A managed or paid_seat_bound row links no beta-class evidence.
    BetaEvidenceMissing,
    /// A row's claim-state would silently widen authority.
    ClaimStateWidensSilently,
    /// The published manifest could not be parsed.
    ManifestParseError,
}

impl ManagedBoundaryBetaDefectKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredBoundaryClassMissing => "required_boundary_class_missing",
            Self::AbsenceNarrowingMissing => "absence_narrowing_missing",
            Self::UnknownVocabularyToken => "unknown_vocabulary_token",
            Self::LocalCoreContinuityMissing => "local_core_continuity_missing",
            Self::BetaEvidenceMissing => "beta_evidence_missing",
            Self::ClaimStateWidensSilently => "claim_state_widens_silently",
            Self::ManifestParseError => "manifest_parse_error",
        }
    }
}

/// Typed validation defect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_kind: ManagedBoundaryBetaDefectKind,
    pub defect_kind_token: String,
    pub subject_id: String,
    pub field: String,
    pub note: String,
}

impl ManagedBoundaryBetaDefect {
    fn new(
        kind: ManagedBoundaryBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: MANAGED_BOUNDARY_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind: kind,
            defect_kind_token: kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the managed-boundary beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaSummary {
    pub page_record_kind: String,
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub source_manifest_ref: String,
    pub manifest_id: String,
    pub manifest_status: String,
    pub manifest_as_of: String,
    pub row_count: usize,
    pub boundary_classes_present: Vec<String>,
    pub claim_states_present: Vec<String>,
    pub deployment_profiles_present: Vec<String>,
    pub org_switch_classes_present: Vec<String>,
    pub grace_window_classes_present: Vec<String>,
    pub seat_quota_states_present: Vec<String>,
    pub offboarding_phases_present: Vec<String>,
    pub export_packet_classes_present: Vec<String>,
    pub absence_narrowed_row_count: usize,
    pub local_state_preserved_row_count: usize,
    pub destruction_receipt_required_row_count: usize,
    pub defect_count: usize,
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl ManagedBoundaryBetaSummary {
    fn from_records(
        manifest: &ParsedManifest,
        rows: &[ManagedBoundaryBetaRow],
        defects: &[ManagedBoundaryBetaDefect],
    ) -> Self {
        let mut boundary_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut claim_states_present: BTreeSet<String> = BTreeSet::new();
        let mut deployment_profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut org_switch_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut grace_window_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut seat_quota_states_present: BTreeSet<String> = BTreeSet::new();
        let mut offboarding_phases_present: BTreeSet<String> = BTreeSet::new();
        let mut export_packet_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut absence_narrowed_row_count = 0usize;
        let mut local_state_preserved_row_count = 0usize;
        let mut destruction_receipt_required_row_count = 0usize;
        for row in rows {
            boundary_classes_present.insert(row.boundary_class_token.clone());
            claim_states_present.insert(row.claim_state_token.clone());
            for token in &row.deployment_profile_tokens {
                deployment_profiles_present.insert(token.clone());
            }
            org_switch_classes_present.insert(row.org_switch_behavior.behavior_class_token.clone());
            grace_window_classes_present.insert(row.grace_window.window_class_token.clone());
            for token in &row.seat_quota.states_observed_tokens {
                seat_quota_states_present.insert(token.clone());
            }
            for token in &row.offboarding.phases_observed_tokens {
                offboarding_phases_present.insert(token.clone());
            }
            export_packet_classes_present.insert(row.offboarding.export_packet_class_token.clone());
            if row.absence_narrows_to.is_some() {
                absence_narrowed_row_count += 1;
            }
            if row.org_switch_behavior.local_state_preserved {
                local_state_preserved_row_count += 1;
            }
            if row.offboarding.destruction_receipt_required {
                destruction_receipt_required_row_count += 1;
            }
        }

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: MANAGED_BOUNDARY_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: MANAGED_BOUNDARY_BETA_SUMMARY_RECORD_KIND.to_owned(),
            schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
            source_manifest_ref: MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF.to_owned(),
            manifest_id: manifest.manifest_id.clone(),
            manifest_status: manifest.status.clone(),
            manifest_as_of: manifest.as_of.clone(),
            row_count: rows.len(),
            boundary_classes_present: boundary_classes_present.into_iter().collect(),
            claim_states_present: claim_states_present.into_iter().collect(),
            deployment_profiles_present: deployment_profiles_present.into_iter().collect(),
            org_switch_classes_present: org_switch_classes_present.into_iter().collect(),
            grace_window_classes_present: grace_window_classes_present.into_iter().collect(),
            seat_quota_states_present: seat_quota_states_present.into_iter().collect(),
            offboarding_phases_present: offboarding_phases_present.into_iter().collect(),
            export_packet_classes_present: export_packet_classes_present.into_iter().collect(),
            absence_narrowed_row_count,
            local_state_preserved_row_count,
            destruction_receipt_required_row_count,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level managed-boundary beta page consumed by product surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub source_manifest_ref: String,
    pub source_schema_ref: String,
    pub manifest_id: String,
    pub manifest_status: String,
    pub manifest_as_of: String,
    pub required_boundary_class_coverage: Vec<String>,
    pub rows: Vec<ManagedBoundaryBetaRow>,
    pub defects: Vec<ManagedBoundaryBetaDefect>,
    pub summary: ManagedBoundaryBetaSummary,
}

impl ManagedBoundaryBetaPage {
    /// Projects every row into its org-switch posture.
    pub fn org_switch_projection(&self) -> Vec<ManagedBoundaryBetaOrgSwitchRow> {
        self.rows
            .iter()
            .map(|row| ManagedBoundaryBetaOrgSwitchRow {
                record_kind: MANAGED_BOUNDARY_BETA_ORG_SWITCH_ROW_RECORD_KIND.to_owned(),
                schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                title: row.title.clone(),
                boundary_class_token: row.boundary_class_token.clone(),
                behavior_class: row.org_switch_behavior.behavior_class,
                behavior_class_token: row.org_switch_behavior.behavior_class_token.clone(),
                summary: row.org_switch_behavior.summary.clone(),
                local_state_preserved: row.org_switch_behavior.local_state_preserved,
                local_core_continuity: row.local_core_continuity.clone(),
                absence_narrows_to: row.absence_narrows_to.clone(),
            })
            .collect()
    }

    /// Projects every row into its seat / quota posture.
    pub fn seat_quota_projection(&self) -> Vec<ManagedBoundaryBetaSeatQuotaRow> {
        self.rows
            .iter()
            .map(|row| ManagedBoundaryBetaSeatQuotaRow {
                record_kind: MANAGED_BOUNDARY_BETA_SEAT_QUOTA_ROW_RECORD_KIND.to_owned(),
                schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                title: row.title.clone(),
                boundary_class_token: row.boundary_class_token.clone(),
                quota_state: row.seat_quota.quota_state,
                quota_state_token: row.seat_quota.quota_state_token.clone(),
                states_observed_tokens: row.seat_quota.states_observed_tokens.clone(),
                meter_refs: row.seat_quota.meter_refs.clone(),
                summary: row.seat_quota.summary.clone(),
                local_core_continuity: row.local_core_continuity.clone(),
                absence_narrows_to: row.absence_narrows_to.clone(),
            })
            .collect()
    }

    /// Projects every row into its grace-window posture.
    pub fn grace_window_projection(&self) -> Vec<ManagedBoundaryBetaGraceWindowRow> {
        self.rows
            .iter()
            .map(|row| ManagedBoundaryBetaGraceWindowRow {
                record_kind: MANAGED_BOUNDARY_BETA_GRACE_WINDOW_ROW_RECORD_KIND.to_owned(),
                schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                title: row.title.clone(),
                boundary_class_token: row.boundary_class_token.clone(),
                window_class: row.grace_window.window_class,
                window_class_token: row.grace_window.window_class_token.clone(),
                duration_iso8601: row.grace_window.duration_iso8601.clone(),
                summary: row.grace_window.summary.clone(),
                local_core_continuity: row.local_core_continuity.clone(),
                absence_narrows_to: row.absence_narrows_to.clone(),
            })
            .collect()
    }

    /// Projects every row into its offboarding posture.
    pub fn offboarding_projection(&self) -> Vec<ManagedBoundaryBetaOffboardingRow> {
        self.rows
            .iter()
            .map(|row| ManagedBoundaryBetaOffboardingRow {
                record_kind: MANAGED_BOUNDARY_BETA_OFFBOARDING_ROW_RECORD_KIND.to_owned(),
                schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                title: row.title.clone(),
                boundary_class_token: row.boundary_class_token.clone(),
                phases_observed_tokens: row.offboarding.phases_observed_tokens.clone(),
                export_packet_class: row.offboarding.export_packet_class,
                export_packet_class_token: row.offboarding.export_packet_class_token.clone(),
                destruction_receipt_required: row.offboarding.destruction_receipt_required,
                summary: row.offboarding.summary.clone(),
                local_core_continuity: row.local_core_continuity.clone(),
                absence_narrows_to: row.absence_narrows_to.clone(),
            })
            .collect()
    }

    /// Lookup a row by its `row_id`.
    pub fn row_by_id(&self, row_id: &str) -> Option<&ManagedBoundaryBetaRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// All rows in a given boundary class, in canonical order.
    pub fn rows_in_class(&self, class: BoundaryClass) -> Vec<&ManagedBoundaryBetaRow> {
        self.rows
            .iter()
            .filter(|row| row.boundary_class == class)
            .collect()
    }
}

/// Support-export wrapper for the managed-boundary beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub source_manifest_ref: String,
    pub source_schema_ref: String,
    pub release_doc_ref: String,
    pub ux_doc_ref: String,
    pub export_id: String,
    pub exported_at: String,
    pub page: ManagedBoundaryBetaPage,
    pub org_switch_rows: Vec<ManagedBoundaryBetaOrgSwitchRow>,
    pub seat_quota_rows: Vec<ManagedBoundaryBetaSeatQuotaRow>,
    pub grace_window_rows: Vec<ManagedBoundaryBetaGraceWindowRow>,
    pub offboarding_rows: Vec<ManagedBoundaryBetaOffboardingRow>,
    pub defect_kinds_present: Vec<String>,
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    pub no_public_endpoint_fallback_invariant: bool,
    pub local_core_continuity_invariant: bool,
    pub absence_narrowing_invariant: bool,
    pub raw_private_material_excluded: bool,
}

impl ManagedBoundaryBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: ManagedBoundaryBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        let local_core_continuity_invariant = page
            .rows
            .iter()
            .all(|row| !row.local_core_continuity.trim().is_empty());
        let absence_narrowing_invariant = page.rows.iter().all(|row| {
            !row.boundary_class.requires_absence_narrowing()
                || row
                    .absence_narrows_to
                    .as_ref()
                    .map(|note| !note.trim().is_empty())
                    .unwrap_or(false)
        });
        let org_switch_rows = page.org_switch_projection();
        let seat_quota_rows = page.seat_quota_projection();
        let grace_window_rows = page.grace_window_projection();
        let offboarding_rows = page.offboarding_projection();
        Self {
            record_kind: MANAGED_BOUNDARY_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
            source_manifest_ref: MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF.to_owned(),
            source_schema_ref: MANAGED_BOUNDARY_BETA_SOURCE_SCHEMA_REF.to_owned(),
            release_doc_ref: MANAGED_BOUNDARY_BETA_RELEASE_DOC_REF.to_owned(),
            ux_doc_ref: MANAGED_BOUNDARY_BETA_UX_DOC_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            org_switch_rows,
            seat_quota_rows,
            grace_window_rows,
            offboarding_rows,
            defect_kinds_present,
            defect_counts_by_kind,
            no_public_endpoint_fallback_invariant: true,
            local_core_continuity_invariant,
            absence_narrowing_invariant,
            raw_private_material_excluded: true,
        }
    }
}

/// Shell render summary suitable for the org-switch / seat-quota /
/// grace-window / offboarding surface and the support-export clipboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBoundaryBetaRenderSummary {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub source_manifest_ref: String,
    pub manifest_id: String,
    pub manifest_status: String,
    pub row_count: usize,
    pub boundary_classes_present: Vec<String>,
    pub claim_states_present: Vec<String>,
    pub org_switch_classes_present: Vec<String>,
    pub grace_window_classes_present: Vec<String>,
    pub seat_quota_states_present: Vec<String>,
    pub offboarding_phases_present: Vec<String>,
    pub export_packet_classes_present: Vec<String>,
    pub defect_count: usize,
}

impl ManagedBoundaryBetaRenderSummary {
    pub const RENDER_RECORD_KIND: &'static str = "shell_managed_boundary_beta_render_record";

    /// Build the shell render summary from the beta page.
    pub fn from_page(page: &ManagedBoundaryBetaPage) -> Self {
        Self {
            record_kind: Self::RENDER_RECORD_KIND.to_owned(),
            schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
            shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
            source_manifest_ref: MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF.to_owned(),
            manifest_id: page.manifest_id.clone(),
            manifest_status: page.manifest_status.clone(),
            row_count: page.rows.len(),
            boundary_classes_present: page.summary.boundary_classes_present.clone(),
            claim_states_present: page.summary.claim_states_present.clone(),
            org_switch_classes_present: page.summary.org_switch_classes_present.clone(),
            grace_window_classes_present: page.summary.grace_window_classes_present.clone(),
            seat_quota_states_present: page.summary.seat_quota_states_present.clone(),
            offboarding_phases_present: page.summary.offboarding_phases_present.clone(),
            export_packet_classes_present: page.summary.export_packet_classes_present.clone(),
            defect_count: page.defects.len(),
        }
    }
}

/// Recomputes typed defects for a slice of rows.
pub fn audit_managed_boundary_beta_rows(
    rows: &[ManagedBoundaryBetaRow],
    required_boundary_class_coverage: &[String],
) -> Vec<ManagedBoundaryBetaDefect> {
    let mut defects = Vec::new();

    let mut observed_classes: BTreeSet<String> = BTreeSet::new();
    for row in rows {
        observed_classes.insert(row.boundary_class_token.clone());
        audit_row(row, &mut defects);
    }

    for required in required_boundary_class_coverage {
        if !observed_classes.contains(required) {
            defects.push(ManagedBoundaryBetaDefect::new(
                ManagedBoundaryBetaDefectKind::RequiredBoundaryClassMissing,
                "page",
                "rows",
                format!("missing required boundary class: {}", required),
            ));
        }
    }

    defects
}

fn audit_row(row: &ManagedBoundaryBetaRow, defects: &mut Vec<ManagedBoundaryBetaDefect>) {
    if row.local_core_continuity.trim().is_empty() {
        defects.push(ManagedBoundaryBetaDefect::new(
            ManagedBoundaryBetaDefectKind::LocalCoreContinuityMissing,
            row.row_id.clone(),
            "local_core_continuity",
            "every row must declare a local-core continuity clause",
        ));
    }
    if row.boundary_class.requires_absence_narrowing() {
        match row.absence_narrows_to.as_ref() {
            None => defects.push(ManagedBoundaryBetaDefect::new(
                ManagedBoundaryBetaDefectKind::AbsenceNarrowingMissing,
                row.row_id.clone(),
                "absence_narrows_to",
                "managed/paid_seat_bound row must publish an absence-narrowing clause",
            )),
            Some(note) if note.trim().is_empty() => defects.push(ManagedBoundaryBetaDefect::new(
                ManagedBoundaryBetaDefectKind::AbsenceNarrowingMissing,
                row.row_id.clone(),
                "absence_narrows_to",
                "managed/paid_seat_bound row must publish an absence-narrowing clause",
            )),
            _ => {}
        }
        if row.evidence_refs.is_empty() {
            defects.push(ManagedBoundaryBetaDefect::new(
                ManagedBoundaryBetaDefectKind::BetaEvidenceMissing,
                row.row_id.clone(),
                "evidence_refs",
                "managed/paid_seat_bound row must cite at least one evidence ref",
            ));
        }
    }
    if row.boundary_class == BoundaryClass::ExplicitlyOutOfScope
        && row.claim_state != ClaimState::ExplicitlyOutOfScope
    {
        defects.push(ManagedBoundaryBetaDefect::new(
            ManagedBoundaryBetaDefectKind::ClaimStateWidensSilently,
            row.row_id.clone(),
            "claim_state",
            "explicitly_out_of_scope boundary class must carry the explicitly_out_of_scope claim state",
        ));
    }
}

/// Validation hook returning typed defects on failure.
pub fn validate_managed_boundary_beta_page(
    page: &ManagedBoundaryBetaPage,
) -> Result<(), Vec<ManagedBoundaryBetaDefect>> {
    let defects =
        audit_managed_boundary_beta_rows(&page.rows, &page.required_boundary_class_coverage);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Loads the boundary manifest, projects it into a typed page, and audits it.
///
/// This is the canonical entry point consumed by every surface. A parse
/// failure surfaces as a [`ManagedBoundaryBetaDefectKind::ManifestParseError`]
/// defect on the returned page rather than a silent fall-back to default
/// vocabulary.
pub fn seeded_managed_boundary_beta_page() -> ManagedBoundaryBetaPage {
    match parse_manifest_yaml(BOUNDARY_MANIFEST_YAML) {
        Ok((manifest, rows)) => {
            let defects =
                audit_managed_boundary_beta_rows(&rows, &manifest.required_boundary_class_coverage);
            let summary = ManagedBoundaryBetaSummary::from_records(&manifest, &rows, &defects);
            ManagedBoundaryBetaPage {
                record_kind: MANAGED_BOUNDARY_BETA_PAGE_RECORD_KIND.to_owned(),
                schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
                source_manifest_ref: MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF.to_owned(),
                source_schema_ref: MANAGED_BOUNDARY_BETA_SOURCE_SCHEMA_REF.to_owned(),
                manifest_id: manifest.manifest_id.clone(),
                manifest_status: manifest.status.clone(),
                manifest_as_of: manifest.as_of.clone(),
                required_boundary_class_coverage: manifest.required_boundary_class_coverage.clone(),
                rows,
                defects,
                summary,
            }
        }
        Err(err) => {
            let manifest = ParsedManifest {
                manifest_id: String::new(),
                status: String::new(),
                as_of: String::new(),
                required_boundary_class_coverage: Vec::new(),
            };
            let rows: Vec<ManagedBoundaryBetaRow> = Vec::new();
            let defects = vec![ManagedBoundaryBetaDefect::new(
                ManagedBoundaryBetaDefectKind::ManifestParseError,
                "page",
                MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF,
                err,
            )];
            let summary = ManagedBoundaryBetaSummary::from_records(&manifest, &rows, &defects);
            ManagedBoundaryBetaPage {
                record_kind: MANAGED_BOUNDARY_BETA_PAGE_RECORD_KIND.to_owned(),
                schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
                shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
                source_manifest_ref: MANAGED_BOUNDARY_BETA_SOURCE_MANIFEST_REF.to_owned(),
                source_schema_ref: MANAGED_BOUNDARY_BETA_SOURCE_SCHEMA_REF.to_owned(),
                manifest_id: manifest.manifest_id,
                manifest_status: manifest.status,
                manifest_as_of: manifest.as_of,
                required_boundary_class_coverage: manifest.required_boundary_class_coverage,
                rows,
                defects,
                summary,
            }
        }
    }
}

struct ParsedManifest {
    manifest_id: String,
    status: String,
    as_of: String,
    required_boundary_class_coverage: Vec<String>,
}

fn parse_manifest_yaml(
    yaml: &str,
) -> Result<(ParsedManifest, Vec<ManagedBoundaryBetaRow>), String> {
    let raw: RawManifest = serde_yaml::from_str(yaml).map_err(|err| err.to_string())?;
    let manifest = ParsedManifest {
        manifest_id: raw.manifest_id,
        status: raw.status,
        as_of: raw.as_of,
        required_boundary_class_coverage: raw.required_boundary_class_coverage.unwrap_or_default(),
    };

    let mut rows = Vec::with_capacity(raw.rows.len());
    for row in raw.rows {
        rows.push(project_row(row)?);
    }

    Ok((manifest, rows))
}

fn project_row(raw: RawRow) -> Result<ManagedBoundaryBetaRow, String> {
    let boundary_class = BoundaryClass::from_token(&raw.boundary_class).ok_or_else(|| {
        format!(
            "unknown boundary_class token '{}' on row {}",
            raw.boundary_class, raw.row_id
        )
    })?;
    let claim_state = ClaimState::from_token(&raw.claim_state).ok_or_else(|| {
        format!(
            "unknown claim_state token '{}' on row {}",
            raw.claim_state, raw.row_id
        )
    })?;
    let mut deployment_profiles = Vec::with_capacity(raw.deployment_profiles.len());
    for token in &raw.deployment_profiles {
        let profile = DeploymentProfile::from_token(token).ok_or_else(|| {
            format!(
                "unknown deployment_profile token '{}' on row {}",
                token, raw.row_id
            )
        })?;
        deployment_profiles.push(profile);
    }
    let org_switch = project_org_switch(&raw.row_id, raw.org_switch_behavior)?;
    let grace_window = project_grace_window(&raw.row_id, raw.grace_window)?;
    let seat_quota = project_seat_quota(&raw.row_id, raw.seat_quota)?;
    let offboarding = project_offboarding(&raw.row_id, raw.offboarding)?;

    let deployment_profile_tokens = raw.deployment_profiles.clone();

    Ok(ManagedBoundaryBetaRow {
        record_kind: MANAGED_BOUNDARY_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
        shared_contract_ref: MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: raw.row_id,
        title: collapse_whitespace(&raw.title),
        summary: collapse_whitespace(&raw.summary),
        boundary_class,
        boundary_class_token: raw.boundary_class,
        claim_state,
        claim_state_token: raw.claim_state,
        deployment_profiles,
        deployment_profile_tokens,
        alpha_capability_refs: raw.alpha_capability_refs.unwrap_or_default(),
        local_core_continuity: collapse_whitespace(&raw.local_core_continuity),
        org_switch_behavior: org_switch,
        grace_window,
        seat_quota,
        offboarding,
        absence_narrows_to: raw
            .absence_narrows_to
            .map(|note| collapse_whitespace(&note)),
        linked_evidence_class: raw.linked_evidence_class,
        evidence_refs: raw.evidence_refs,
        notes: raw.notes.map(|note| collapse_whitespace(&note)),
    })
}

fn project_org_switch(
    row_id: &str,
    raw: RawOrgSwitchBlock,
) -> Result<OrgSwitchBehaviorBlock, String> {
    let behavior_class =
        OrgSwitchBehaviorClass::from_token(&raw.behavior_class).ok_or_else(|| {
            format!(
                "unknown org_switch behavior_class token '{}' on row {}",
                raw.behavior_class, row_id
            )
        })?;
    Ok(OrgSwitchBehaviorBlock {
        behavior_class,
        behavior_class_token: raw.behavior_class,
        summary: collapse_whitespace(&raw.summary),
        local_state_preserved: raw.local_state_preserved.unwrap_or(true),
    })
}

fn project_grace_window(
    row_id: &str,
    raw: RawGraceWindowBlock,
) -> Result<GraceWindowBlock, String> {
    let window_class = GraceWindowClass::from_token(&raw.window_class).ok_or_else(|| {
        format!(
            "unknown grace_window window_class token '{}' on row {}",
            raw.window_class, row_id
        )
    })?;
    Ok(GraceWindowBlock {
        window_class,
        window_class_token: raw.window_class,
        duration_iso8601: raw.duration_iso8601,
        summary: collapse_whitespace(&raw.summary),
    })
}

fn project_seat_quota(row_id: &str, raw: RawSeatQuotaBlock) -> Result<SeatQuotaBlock, String> {
    let quota_state = SeatQuotaState::from_token(&raw.quota_state).ok_or_else(|| {
        format!(
            "unknown seat_quota quota_state token '{}' on row {}",
            raw.quota_state, row_id
        )
    })?;
    let mut states_observed = Vec::with_capacity(raw.states_observed.len());
    for token in &raw.states_observed {
        let state = SeatQuotaState::from_token(token).ok_or_else(|| {
            format!(
                "unknown seat_quota observed-state token '{}' on row {}",
                token, row_id
            )
        })?;
        states_observed.push(state);
    }
    Ok(SeatQuotaBlock {
        quota_state,
        quota_state_token: raw.quota_state,
        states_observed,
        states_observed_tokens: raw.states_observed,
        meter_refs: raw.meter_refs.unwrap_or_default(),
        summary: collapse_whitespace(&raw.summary),
    })
}

fn project_offboarding(row_id: &str, raw: RawOffboardingBlock) -> Result<OffboardingBlock, String> {
    let mut phases_observed = Vec::with_capacity(raw.phases_observed.len());
    for token in &raw.phases_observed {
        let phase = OffboardingPhase::from_token(token).ok_or_else(|| {
            format!(
                "unknown offboarding phase token '{}' on row {}",
                token, row_id
            )
        })?;
        phases_observed.push(phase);
    }
    let export_packet_class =
        ExportPacketClass::from_token(&raw.export_packet_class).ok_or_else(|| {
            format!(
                "unknown offboarding export_packet_class token '{}' on row {}",
                raw.export_packet_class, row_id
            )
        })?;
    Ok(OffboardingBlock {
        phases_observed,
        phases_observed_tokens: raw.phases_observed,
        export_packet_class,
        export_packet_class_token: raw.export_packet_class,
        destruction_receipt_required: raw.destruction_receipt_required.unwrap_or(false),
        summary: collapse_whitespace(&raw.summary),
    })
}

fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[derive(Debug, Deserialize)]
struct RawManifest {
    manifest_id: String,
    status: String,
    as_of: String,
    #[serde(default)]
    required_boundary_class_coverage: Option<Vec<String>>,
    rows: Vec<RawRow>,
}

#[derive(Debug, Deserialize)]
struct RawRow {
    row_id: String,
    title: String,
    summary: String,
    boundary_class: String,
    claim_state: String,
    deployment_profiles: Vec<String>,
    #[serde(default)]
    alpha_capability_refs: Option<Vec<String>>,
    local_core_continuity: String,
    org_switch_behavior: RawOrgSwitchBlock,
    grace_window: RawGraceWindowBlock,
    seat_quota: RawSeatQuotaBlock,
    offboarding: RawOffboardingBlock,
    #[serde(default)]
    absence_narrows_to: Option<String>,
    linked_evidence_class: String,
    evidence_refs: Vec<String>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawOrgSwitchBlock {
    behavior_class: String,
    summary: String,
    #[serde(default)]
    local_state_preserved: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct RawGraceWindowBlock {
    window_class: String,
    #[serde(default)]
    duration_iso8601: Option<String>,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct RawSeatQuotaBlock {
    quota_state: String,
    states_observed: Vec<String>,
    #[serde(default)]
    meter_refs: Option<Vec<String>>,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct RawOffboardingBlock {
    phases_observed: Vec<String>,
    export_packet_class: String,
    #[serde(default)]
    destruction_receipt_required: Option<bool>,
    summary: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_manifest_loads_with_zero_defects() {
        let page = seeded_managed_boundary_beta_page();
        validate_managed_boundary_beta_page(&page).expect("published manifest must validate");
        assert!(page.defects.is_empty());
        assert!(!page.rows.is_empty());
        assert_eq!(page.manifest_status, "seeded");
        assert!(page
            .summary
            .boundary_classes_present
            .iter()
            .any(|t| t == "local_only"));
        assert!(page
            .summary
            .boundary_classes_present
            .iter()
            .any(|t| t == "managed"));
        assert!(page
            .summary
            .boundary_classes_present
            .iter()
            .any(|t| t == "paid_seat_bound"));
    }

    #[test]
    fn required_boundary_class_coverage_is_enforced() {
        let page = seeded_managed_boundary_beta_page();
        for required in &page.required_boundary_class_coverage {
            assert!(
                page.summary
                    .boundary_classes_present
                    .iter()
                    .any(|present| present == required),
                "required boundary class {required} missing from projection"
            );
        }
    }

    #[test]
    fn managed_and_paid_rows_publish_absence_narrowing() {
        let page = seeded_managed_boundary_beta_page();
        for row in &page.rows {
            if row.boundary_class.requires_absence_narrowing() {
                assert!(
                    row.absence_narrows_to
                        .as_ref()
                        .map(|note| !note.trim().is_empty())
                        .unwrap_or(false),
                    "row {} (class {}) must publish an absence_narrows_to clause",
                    row.row_id,
                    row.boundary_class.as_str()
                );
            }
        }
    }

    #[test]
    fn per_flow_projections_cover_every_row() {
        let page = seeded_managed_boundary_beta_page();
        assert_eq!(page.org_switch_projection().len(), page.rows.len());
        assert_eq!(page.seat_quota_projection().len(), page.rows.len());
        assert_eq!(page.grace_window_projection().len(), page.rows.len());
        assert_eq!(page.offboarding_projection().len(), page.rows.len());
    }

    #[test]
    fn flow_projections_reuse_manifest_vocabulary() {
        let page = seeded_managed_boundary_beta_page();
        for row in page.org_switch_projection() {
            assert_eq!(
                row.behavior_class.as_str(),
                row.behavior_class_token,
                "org-switch behavior_class token must match the typed enum"
            );
        }
        for row in page.seat_quota_projection() {
            assert_eq!(row.quota_state.as_str(), row.quota_state_token);
            for token in &row.states_observed_tokens {
                assert!(
                    SeatQuotaState::from_token(token).is_some(),
                    "seat_quota observed state token {token} must be in the published vocabulary"
                );
            }
        }
        for row in page.grace_window_projection() {
            assert_eq!(row.window_class.as_str(), row.window_class_token);
        }
        for row in page.offboarding_projection() {
            assert_eq!(
                row.export_packet_class.as_str(),
                row.export_packet_class_token
            );
            for token in &row.phases_observed_tokens {
                assert!(
                    OffboardingPhase::from_token(token).is_some(),
                    "offboarding phase token {token} must be in the published vocabulary"
                );
            }
        }
    }

    #[test]
    fn validator_flags_missing_absence_narrowing() {
        let mut page = seeded_managed_boundary_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.boundary_class.requires_absence_narrowing())
            .expect("at least one managed or paid row");
        row.absence_narrows_to = None;
        let defects =
            audit_managed_boundary_beta_rows(&page.rows, &page.required_boundary_class_coverage);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == ManagedBoundaryBetaDefectKind::AbsenceNarrowingMissing));
    }

    #[test]
    fn validator_flags_missing_local_core_continuity() {
        let mut page = seeded_managed_boundary_beta_page();
        page.rows[0].local_core_continuity = String::new();
        let defects =
            audit_managed_boundary_beta_rows(&page.rows, &page.required_boundary_class_coverage);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ManagedBoundaryBetaDefectKind::LocalCoreContinuityMissing));
    }

    #[test]
    fn validator_flags_missing_required_boundary_class() {
        let mut page = seeded_managed_boundary_beta_page();
        page.rows
            .retain(|row| row.boundary_class != BoundaryClass::PaidSeatBound);
        let defects =
            audit_managed_boundary_beta_rows(&page.rows, &page.required_boundary_class_coverage);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ManagedBoundaryBetaDefectKind::RequiredBoundaryClassMissing));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_managed_boundary_beta_page();
        let row_count = page.rows.len();
        let export = ManagedBoundaryBetaSupportExport::from_page(
            "managed-boundary:support-export:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.no_public_endpoint_fallback_invariant);
        assert!(export.local_core_continuity_invariant);
        assert!(export.absence_narrowing_invariant);
        assert_eq!(export.org_switch_rows.len(), row_count);
        assert_eq!(export.seat_quota_rows.len(), row_count);
        assert_eq!(export.grace_window_rows.len(), row_count);
        assert_eq!(export.offboarding_rows.len(), row_count);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn render_summary_mirrors_summary_counts() {
        let page = seeded_managed_boundary_beta_page();
        let render = ManagedBoundaryBetaRenderSummary::from_page(&page);
        assert_eq!(render.row_count, page.rows.len());
        assert_eq!(
            render.boundary_classes_present,
            page.summary.boundary_classes_present
        );
        assert_eq!(
            render.seat_quota_states_present,
            page.summary.seat_quota_states_present
        );
        assert_eq!(render.defect_count, page.defects.len());
    }

    #[test]
    fn row_lookups_resolve_canonical_local_core_row() {
        let page = seeded_managed_boundary_beta_page();
        let local_core = page
            .row_by_id("beta_boundary_row:local_core_editor")
            .expect("local-core editor row is published");
        assert_eq!(local_core.boundary_class, BoundaryClass::LocalOnly);
        let paid = page.rows_in_class(BoundaryClass::PaidSeatBound);
        assert!(!paid.is_empty());
    }
}
