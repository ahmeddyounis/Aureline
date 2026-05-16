//! Signed policy-bundle and offline-entitlement verifier baseline.
//!
//! This module owns the beta verifier for signed policy bundles and
//! entitlement snapshots that must remain provable without live vendor
//! calls. It gives admin, support, shell, headless, and reviewer surfaces
//! one inspectable record per verifier row that names:
//!
//! - the bundle under inspection (kind, version, epoch, signer, signed-at,
//!   valid-until);
//! - the trust anchor that authorises it (vendor-managed root, customer
//!   self-hosted root, signed mirror root, manual signed-file import root,
//!   air-gapped root, runtime preload root, or local-advisory no-root);
//! - the typed verifier outcome (verified live / mirror / manual import /
//!   air-gapped / runtime preload, expired, signature missing, signature
//!   invalid, untrusted signer, bundle not present, revoked, or unsigned
//!   local advisory);
//! - the managed-capability downgrade that follows from the outcome and
//!   the explicit local-editing preservation posture so a failed verify
//!   never blocks local work; and
//! - the recovery action a product surface or admin runbook should offer.
//!
//! Surfaces (admin console, support export, shell trust center, headless
//! inspector) read [`seeded_offline_entitlement_verifier_beta_page`]
//! rather than minting parallel "is_bundle_signed" checks. The seed
//! covers connected, mirror-only, offline, and enterprise-managed
//! profiles for both policy bundles and entitlement snapshots and proves
//! the failure-mode rows downgrade managed authority without losing the
//! local-editing floor.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Beta schema version exported with every verifier record.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every verifier record.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF: &str =
    "security:offline_entitlement_verifier_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/offline_entitlement_verifier_matrix.yaml";

/// Stable record kind for [`OfflineEntitlementVerifierBetaPage`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_PAGE_RECORD_KIND: &str =
    "security_offline_entitlement_verifier_beta_page_record";

/// Stable record kind for [`OfflineEntitlementVerifierBetaRow`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_ROW_RECORD_KIND: &str =
    "security_offline_entitlement_verifier_beta_row_record";

/// Stable record kind for [`OfflineEntitlementVerifierBetaDefect`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_DEFECT_RECORD_KIND: &str =
    "security_offline_entitlement_verifier_beta_defect_record";

/// Stable record kind for [`OfflineEntitlementVerifierBetaSummary`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUMMARY_RECORD_KIND: &str =
    "security_offline_entitlement_verifier_beta_summary_record";

/// Stable record kind for [`OfflineEntitlementVerifierBetaSupportExport`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_offline_entitlement_verifier_beta_support_export_record";

/// Stable record kind for [`OfflineEntitlementVerifierBetaSupportRow`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "security_offline_entitlement_verifier_beta_support_row_record";

/// Connectedness or enterprise profile under which a verifier row is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineEntitlementVerifierBetaProfileClass {
    /// Connected beta profile with live verification path available.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline or air-gapped profile served from an imported snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed narrowing.
    EnterpriseManaged,
}

impl OfflineEntitlementVerifierBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Kind of signed bundle the verifier inspects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifiedBundleKindClass {
    /// Signed policy bundle authorising managed narrowing rules.
    PolicyBundle,
    /// Signed entitlement snapshot describing plan, seat, and quota state.
    EntitlementSnapshot,
}

impl VerifiedBundleKindClass {
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

/// Trust anchor source the verifier resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustAnchorSourceClass {
    /// Vendor-managed signing root pinned in the runtime.
    VendorManagedRoot,
    /// Customer self-hosted signing root authorised by the deployment.
    CustomerSelfHostedRoot,
    /// Signed mirror root used by mirror-only profiles.
    SignedMirrorRoot,
    /// Manual signed-file import root accepted at admin time.
    ManualImportRoot,
    /// Air-gapped signed-transfer root used by offline profiles.
    AirGappedRoot,
    /// Build preload or first-run seed root.
    RuntimePreloadRoot,
    /// No trust anchor; only unsigned local-advisory bundles may resolve here.
    LocalAdvisoryNoRoot,
}

impl TrustAnchorSourceClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VendorManagedRoot => "vendor_managed_root",
            Self::CustomerSelfHostedRoot => "customer_self_hosted_root",
            Self::SignedMirrorRoot => "signed_mirror_root",
            Self::ManualImportRoot => "manual_import_root",
            Self::AirGappedRoot => "air_gapped_root",
            Self::RuntimePreloadRoot => "runtime_preload_root",
            Self::LocalAdvisoryNoRoot => "local_advisory_no_root",
        }
    }

    /// True when this anchor is capable of authorising managed narrowing.
    pub const fn authorises_managed(self) -> bool {
        !matches!(self, Self::LocalAdvisoryNoRoot)
    }
}

/// Typed verifier outcome for one bundle on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifierOutcomeClass {
    /// Signature verified live against the authoritative trust anchor.
    VerifiedLive,
    /// Signature verified against a signed mirror trust anchor.
    VerifiedMirror,
    /// Signature verified during manual signed-file import.
    VerifiedManualImport,
    /// Signature verified during air-gapped signed transfer.
    VerifiedAirGapped,
    /// Bundle is an unsigned local advisory; no managed authority follows.
    UnsignedLocalAdvisory,
    /// Bundle is past `valid_until` and must downgrade managed authority.
    Expired,
    /// Bundle is missing a signature on an origin that requires one.
    SignatureMissing,
    /// Bundle's signature did not validate against the trust anchor.
    SignatureInvalid,
    /// Signature validated but the signer is not on the trust anchor list.
    UntrustedSigner,
    /// No bundle is present for the inspected profile and kind.
    BundleNotPresent,
    /// Bundle was explicitly revoked by an emergency action record.
    Revoked,
}

impl VerifierOutcomeClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedLive => "verified_live",
            Self::VerifiedMirror => "verified_mirror",
            Self::VerifiedManualImport => "verified_manual_import",
            Self::VerifiedAirGapped => "verified_air_gapped",
            Self::UnsignedLocalAdvisory => "unsigned_local_advisory",
            Self::Expired => "expired",
            Self::SignatureMissing => "signature_missing",
            Self::SignatureInvalid => "signature_invalid",
            Self::UntrustedSigner => "untrusted_signer",
            Self::BundleNotPresent => "bundle_not_present",
            Self::Revoked => "revoked",
        }
    }

    /// True when this outcome is sufficient for managed authority to apply.
    pub const fn authorises_full_managed(self) -> bool {
        matches!(
            self,
            Self::VerifiedLive
                | Self::VerifiedMirror
                | Self::VerifiedManualImport
                | Self::VerifiedAirGapped
        )
    }

    /// True when this outcome requires the managed capability to downgrade.
    pub const fn requires_downgrade(self) -> bool {
        matches!(
            self,
            Self::Expired
                | Self::SignatureMissing
                | Self::SignatureInvalid
                | Self::UntrustedSigner
                | Self::BundleNotPresent
                | Self::Revoked
        )
    }
}

/// Managed-capability impact decided from the verifier outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedCapabilityImpactClass {
    /// Managed authority is active for this capability.
    FullAuthorityActive,
    /// Managed authority narrowed to inspect-only.
    NarrowedToInspectOnly,
    /// Managed authority narrowed to read or preview.
    NarrowedToReadOrPreview,
    /// Managed capability paused with a visible recovery cue.
    PausedWithVisibleRecovery,
    /// Managed capability blocked pending admin repair.
    BlockedPendingRepair,
    /// Capability has no managed component on this profile/kind.
    NotApplicableLocalOnly,
}

impl ManagedCapabilityImpactClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullAuthorityActive => "full_authority_active",
            Self::NarrowedToInspectOnly => "narrowed_to_inspect_only",
            Self::NarrowedToReadOrPreview => "narrowed_to_read_or_preview",
            Self::PausedWithVisibleRecovery => "paused_with_visible_recovery",
            Self::BlockedPendingRepair => "blocked_pending_repair",
            Self::NotApplicableLocalOnly => "not_applicable_local_only",
        }
    }

    /// True when this impact widens authority instead of narrowing it.
    pub const fn is_full_authority(self) -> bool {
        matches!(self, Self::FullAuthorityActive)
    }

    /// True when managed authority has narrowed for this row.
    pub const fn narrows_managed_authority(self) -> bool {
        matches!(
            self,
            Self::NarrowedToInspectOnly
                | Self::NarrowedToReadOrPreview
                | Self::PausedWithVisibleRecovery
                | Self::BlockedPendingRepair
        )
    }
}

/// Local-editing preservation posture under this verifier outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEditingPreservationClass {
    /// Local editing remains fully available.
    Preserved,
    /// Local editing remains available with an advisory banner.
    PreservedWithAdvisory,
    /// Capability is local-only by design; no managed pathway exists.
    NotApplicableLocalOnly,
}

impl LocalEditingPreservationClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::PreservedWithAdvisory => "preserved_with_advisory",
            Self::NotApplicableLocalOnly => "not_applicable_local_only",
        }
    }

    /// True when local editing must remain available on this row.
    pub const fn preserves_local_editing(self) -> bool {
        matches!(self, Self::Preserved | Self::PreservedWithAdvisory)
    }
}

/// Recovery action a surface should offer for this verifier row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifierRecoveryActionClass {
    /// No recovery action is required; the bundle verifies cleanly.
    NoActionVerified,
    /// Refresh the signed bundle from the live origin.
    RefreshSignedBundleLive,
    /// Refresh the signed bundle from the signed mirror.
    RefreshSignedBundleMirror,
    /// Import a fresh signed file from admin or vendor delivery.
    ImportSignedBundleFile,
    /// Apply a fresh air-gapped signed transfer.
    ImportAirGappedTransfer,
    /// Escalate to admin or quorum-approved emergency action.
    EscalateAdminQuorum,
    /// Continue with local-only behaviour while managed authority is narrowed.
    ContinueLocalOnly,
}

impl VerifierRecoveryActionClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoActionVerified => "no_action_verified",
            Self::RefreshSignedBundleLive => "refresh_signed_bundle_live",
            Self::RefreshSignedBundleMirror => "refresh_signed_bundle_mirror",
            Self::ImportSignedBundleFile => "import_signed_bundle_file",
            Self::ImportAirGappedTransfer => "import_air_gapped_transfer",
            Self::EscalateAdminQuorum => "escalate_admin_quorum",
            Self::ContinueLocalOnly => "continue_local_only",
        }
    }
}

/// Typed validator defect for the verifier beta page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineEntitlementVerifierBetaDefectKind {
    /// A row's verifier outcome accepts an untrusted signer.
    UntrustedSignerAccepted,
    /// A row's verifier outcome accepts an expired bundle without downgrade.
    ExpiredBundleAcceptedWithoutDowngrade,
    /// A row reports a verified outcome but no trust anchor authorises it.
    VerifiedOutcomeWithoutTrustAnchor,
    /// A row's outcome does not match its managed-capability impact.
    OutcomeImpactMismatch,
    /// A row's outcome token does not match its outcome class.
    OutcomeTokenDrift,
    /// A row's profile token does not match its profile class.
    ProfileTokenDrift,
    /// A row's bundle-kind token does not match its bundle kind.
    BundleKindTokenDrift,
    /// A row's trust-anchor token does not match its source class.
    TrustAnchorTokenDrift,
    /// A row's impact token does not match its impact class.
    ImpactTokenDrift,
    /// A row's local-editing-preservation token does not match its class.
    LocalEditingTokenDrift,
    /// A row's recovery-action token does not match its action class.
    RecoveryActionTokenDrift,
    /// A failed-verify row reports local editing is blocked.
    LocalEditingBlockedOnFailedVerification,
    /// A row permits an undeclared public endpoint fallback.
    HiddenPublicEndpointFallback,
    /// A row would expose raw private or secret material.
    RawPrivateMaterialExposed,
    /// A required profile is missing from the page rows.
    ProfileCoverageMissing,
    /// A required bundle kind is missing from the page rows.
    BundleKindCoverageMissing,
    /// An unsigned-local-advisory outcome appears on a managed trust anchor.
    UnsignedLocalAdvisoryOnManagedAnchor,
    /// A downgrade outcome offered no recovery action.
    DowngradeMissingRecoveryAction,
}

impl OfflineEntitlementVerifierBetaDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UntrustedSignerAccepted => "untrusted_signer_accepted",
            Self::ExpiredBundleAcceptedWithoutDowngrade => {
                "expired_bundle_accepted_without_downgrade"
            }
            Self::VerifiedOutcomeWithoutTrustAnchor => "verified_outcome_without_trust_anchor",
            Self::OutcomeImpactMismatch => "outcome_impact_mismatch",
            Self::OutcomeTokenDrift => "outcome_token_drift",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::BundleKindTokenDrift => "bundle_kind_token_drift",
            Self::TrustAnchorTokenDrift => "trust_anchor_token_drift",
            Self::ImpactTokenDrift => "impact_token_drift",
            Self::LocalEditingTokenDrift => "local_editing_token_drift",
            Self::RecoveryActionTokenDrift => "recovery_action_token_drift",
            Self::LocalEditingBlockedOnFailedVerification => {
                "local_editing_blocked_on_failed_verification"
            }
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::BundleKindCoverageMissing => "bundle_kind_coverage_missing",
            Self::UnsignedLocalAdvisoryOnManagedAnchor => {
                "unsigned_local_advisory_on_managed_anchor"
            }
            Self::DowngradeMissingRecoveryAction => "downgrade_missing_recovery_action",
        }
    }
}

/// Resolved trust anchor authorising a verifier row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifierTrustAnchor {
    /// Source class for the trust anchor.
    pub source_class: TrustAnchorSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_token: String,
    /// Opaque ref into the trust anchor catalogue.
    pub anchor_ref: String,
    /// Export-safe anchor label.
    pub anchor_label: String,
    /// Last-rotated timestamp for the anchor.
    pub last_rotated_at: String,
}

/// Bundle under verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifierBundleSubject {
    /// Opaque ref to the bundle artifact.
    pub bundle_ref: String,
    /// Kind of bundle this row inspects.
    pub bundle_kind: VerifiedBundleKindClass,
    /// Stable token for [`Self::bundle_kind`].
    pub bundle_kind_token: String,
    /// Reviewable bundle version string.
    pub bundle_version: String,
    /// Opaque epoch ref for the bundle.
    pub bundle_epoch_ref: String,
    /// Signer id recorded on the bundle.
    pub signer_id: String,
    /// Signed-at timestamp on the bundle.
    pub signed_at: String,
    /// `valid_until` timestamp on the bundle.
    pub valid_until: String,
}

/// One verifier row covering one bundle on one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Export-safe display label.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile_class: OfflineEntitlementVerifierBetaProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_token: String,
    /// Subject bundle.
    pub subject: VerifierBundleSubject,
    /// Resolved trust anchor.
    pub trust_anchor: VerifierTrustAnchor,
    /// Verifier outcome.
    pub outcome_class: VerifierOutcomeClass,
    /// Stable token for [`Self::outcome_class`].
    pub outcome_token: String,
    /// Export-safe outcome label.
    pub outcome_label: String,
    /// Managed-capability impact decided from the outcome.
    pub managed_capability_impact: ManagedCapabilityImpactClass,
    /// Stable token for [`Self::managed_capability_impact`].
    pub managed_capability_impact_token: String,
    /// Local-editing preservation posture.
    pub local_editing_preservation: LocalEditingPreservationClass,
    /// Stable token for [`Self::local_editing_preservation`].
    pub local_editing_preservation_token: String,
    /// Recovery action a surface should offer.
    pub recovery_action: VerifierRecoveryActionClass,
    /// Stable token for [`Self::recovery_action`].
    pub recovery_action_token: String,
    /// Export-safe verifier explanation rendered by inspectors.
    pub explanation_label: String,
    /// True when no undeclared public endpoint fallback is permitted.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private or secret material is excluded from the record.
    pub raw_private_material_excluded: bool,
}

/// Typed validator defect for the verifier beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: OfflineEntitlementVerifierBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row, page, or coverage axis).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl OfflineEntitlementVerifierBetaDefect {
    fn new(
        defect_kind: OfflineEntitlementVerifierBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the verifier beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of inspected rows.
    pub row_count: usize,
    /// Number of rows whose outcome verifies cleanly.
    pub verified_row_count: usize,
    /// Number of rows whose outcome requires a downgrade.
    pub failed_row_count: usize,
    /// Number of rows that preserve local editing under failure.
    pub local_editing_preserved_row_count: usize,
    /// Number of rows that narrow managed authority.
    pub managed_authority_narrowed_row_count: usize,
    /// Profile tokens present on the page.
    pub profiles_present: Vec<String>,
    /// Bundle-kind tokens present on the page.
    pub bundle_kinds_present: Vec<String>,
    /// Outcome tokens present on the page.
    pub outcomes_present: Vec<String>,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl OfflineEntitlementVerifierBetaSummary {
    /// Build a summary from the underlying rows and defects.
    pub fn from_records(
        rows: &[OfflineEntitlementVerifierBetaRow],
        defects: &[OfflineEntitlementVerifierBetaDefect],
    ) -> Self {
        let profiles_present: BTreeSet<String> =
            rows.iter().map(|row| row.profile_token.clone()).collect();
        let bundle_kinds_present: BTreeSet<String> = rows
            .iter()
            .map(|row| row.subject.bundle_kind_token.clone())
            .collect();
        let outcomes_present: BTreeSet<String> =
            rows.iter().map(|row| row.outcome_token.clone()).collect();
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            page_record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUMMARY_RECORD_KIND.to_owned(),
            row_count: rows.len(),
            verified_row_count: rows
                .iter()
                .filter(|row| row.outcome_class.authorises_full_managed())
                .count(),
            failed_row_count: rows
                .iter()
                .filter(|row| row.outcome_class.requires_downgrade())
                .count(),
            local_editing_preserved_row_count: rows
                .iter()
                .filter(|row| row.local_editing_preservation.preserves_local_editing())
                .count(),
            managed_authority_narrowed_row_count: rows
                .iter()
                .filter(|row| row.managed_capability_impact.narrows_managed_authority())
                .count(),
            profiles_present: profiles_present.into_iter().collect(),
            bundle_kinds_present: bundle_kinds_present.into_iter().collect(),
            outcomes_present: outcomes_present.into_iter().collect(),
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin, support, shell, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Inspected verifier rows.
    pub rows: Vec<OfflineEntitlementVerifierBetaRow>,
    /// Typed validator defects.
    pub defects: Vec<OfflineEntitlementVerifierBetaDefect>,
    /// Aggregate summary.
    pub summary: OfflineEntitlementVerifierBetaSummary,
}

/// Support-facing projection of a verifier row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaSupportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source row id.
    pub row_id: String,
    /// Profile token.
    pub profile_token: String,
    /// Bundle-kind token.
    pub bundle_kind_token: String,
    /// Trust-anchor source token.
    pub trust_anchor_source_token: String,
    /// Bundle version label.
    pub bundle_version: String,
    /// Signer id.
    pub signer_id: String,
    /// Outcome token.
    pub outcome_token: String,
    /// Managed-capability impact token.
    pub managed_capability_impact_token: String,
    /// Local-editing preservation token.
    pub local_editing_preservation_token: String,
    /// Recovery action token.
    pub recovery_action_token: String,
    /// Export-safe explanation label.
    pub explanation_label: String,
}

impl OfflineEntitlementVerifierBetaSupportRow {
    /// Project a support row from a verifier row.
    pub fn from_row(row: &OfflineEntitlementVerifierBetaRow) -> Self {
        Self {
            record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: row.row_id.clone(),
            profile_token: row.profile_token.clone(),
            bundle_kind_token: row.subject.bundle_kind_token.clone(),
            trust_anchor_source_token: row.trust_anchor.source_token.clone(),
            bundle_version: row.subject.bundle_version.clone(),
            signer_id: row.subject.signer_id.clone(),
            outcome_token: row.outcome_token.clone(),
            managed_capability_impact_token: row.managed_capability_impact_token.clone(),
            local_editing_preservation_token: row.local_editing_preservation_token.clone(),
            recovery_action_token: row.recovery_action_token.clone(),
            explanation_label: row.explanation_label.clone(),
        }
    }
}

/// Support-export wrapper for the verifier beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: OfflineEntitlementVerifierBetaPage,
    /// Per-row support projections.
    pub support_rows: Vec<OfflineEntitlementVerifierBetaSupportRow>,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private material is excluded from the export.
    pub raw_private_material_excluded: bool,
}

impl OfflineEntitlementVerifierBetaSupportExport {
    /// Build a support export wrapper from a verifier beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: OfflineEntitlementVerifierBetaPage,
    ) -> Self {
        let support_rows: Vec<_> = page
            .rows
            .iter()
            .map(OfflineEntitlementVerifierBetaSupportRow::from_row)
            .collect();
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            support_rows,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_private_material_excluded: true,
        }
    }
}

/// Request to stage a verifier row with stable tokens filled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageOfflineEntitlementVerifierBetaRowRequest<'a> {
    pub row_id: &'a str,
    pub display_label: &'a str,
    pub profile_class: OfflineEntitlementVerifierBetaProfileClass,
    pub subject: VerifierBundleSubject,
    pub trust_anchor: VerifierTrustAnchor,
    pub outcome_class: VerifierOutcomeClass,
    pub outcome_label: &'a str,
    pub managed_capability_impact: ManagedCapabilityImpactClass,
    pub local_editing_preservation: LocalEditingPreservationClass,
    pub recovery_action: VerifierRecoveryActionClass,
    pub explanation_label: &'a str,
}

impl OfflineEntitlementVerifierBetaRow {
    /// Stage a verifier row and fill stable tokens.
    pub fn stage(request: StageOfflineEntitlementVerifierBetaRowRequest<'_>) -> Self {
        let mut subject = request.subject;
        subject.bundle_kind_token = subject.bundle_kind.as_str().to_owned();
        let mut trust_anchor = request.trust_anchor;
        trust_anchor.source_token = trust_anchor.source_class.as_str().to_owned();
        Self {
            record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: request.row_id.to_owned(),
            display_label: request.display_label.to_owned(),
            profile_class: request.profile_class,
            profile_token: request.profile_class.as_str().to_owned(),
            subject,
            trust_anchor,
            outcome_class: request.outcome_class,
            outcome_token: request.outcome_class.as_str().to_owned(),
            outcome_label: request.outcome_label.to_owned(),
            managed_capability_impact: request.managed_capability_impact,
            managed_capability_impact_token: request.managed_capability_impact.as_str().to_owned(),
            local_editing_preservation: request.local_editing_preservation,
            local_editing_preservation_token: request.local_editing_preservation.as_str().to_owned(),
            recovery_action: request.recovery_action,
            recovery_action_token: request.recovery_action.as_str().to_owned(),
            explanation_label: request.explanation_label.to_owned(),
            no_public_endpoint_fallback: true,
            raw_private_material_excluded: true,
        }
    }
}

/// Build the seeded verifier beta page covering connected, mirror, offline,
/// and enterprise-managed profiles for both policy bundles and entitlement
/// snapshots, including failure rows that exercise the downgrade path.
pub fn seeded_offline_entitlement_verifier_beta_page() -> OfflineEntitlementVerifierBetaPage {
    let rows = seeded_rows();
    let defects = audit_offline_entitlement_verifier_beta_rows(&rows);
    let summary = OfflineEntitlementVerifierBetaSummary::from_records(&rows, &defects);
    OfflineEntitlementVerifierBetaPage {
        record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
        shared_contract_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SOURCE_MATRIX_REF.to_owned(),
        rows,
        defects,
        summary,
    }
}

/// Validate the page and return typed defects on failure.
pub fn validate_offline_entitlement_verifier_beta_page(
    page: &OfflineEntitlementVerifierBetaPage,
) -> Result<(), Vec<OfflineEntitlementVerifierBetaDefect>> {
    let defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recompute defects for the verifier rows. The audit enforces:
///
/// - stable token consistency on each row;
/// - verified outcomes resolve to `FullAuthorityActive` or
///   `NotApplicableLocalOnly`;
/// - failed outcomes never resolve to `FullAuthorityActive`;
/// - failed outcomes never block local editing;
/// - downgrades carry a recovery action other than `NoActionVerified`;
/// - `UnsignedLocalAdvisory` outcomes only appear with `LocalAdvisoryNoRoot`;
/// - every required profile and bundle kind appears at least once;
/// - records refuse undeclared public endpoint fallback and exclude raw
///   private/secret material from the row.
pub fn audit_offline_entitlement_verifier_beta_rows(
    rows: &[OfflineEntitlementVerifierBetaRow],
) -> Vec<OfflineEntitlementVerifierBetaDefect> {
    let mut defects = Vec::new();

    for row in rows {
        if row.profile_token != row.profile_class.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::ProfileTokenDrift,
                row.row_id.clone(),
                "profile_token",
                "profile_token must match profile_class",
            ));
        }
        if row.subject.bundle_kind_token != row.subject.bundle_kind.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::BundleKindTokenDrift,
                row.row_id.clone(),
                "subject.bundle_kind_token",
                "bundle_kind_token must match bundle_kind",
            ));
        }
        if row.trust_anchor.source_token != row.trust_anchor.source_class.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::TrustAnchorTokenDrift,
                row.row_id.clone(),
                "trust_anchor.source_token",
                "trust_anchor source_token must match source_class",
            ));
        }
        if row.outcome_token != row.outcome_class.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::OutcomeTokenDrift,
                row.row_id.clone(),
                "outcome_token",
                "outcome_token must match outcome_class",
            ));
        }
        if row.managed_capability_impact_token != row.managed_capability_impact.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::ImpactTokenDrift,
                row.row_id.clone(),
                "managed_capability_impact_token",
                "managed_capability_impact_token must match managed_capability_impact",
            ));
        }
        if row.local_editing_preservation_token != row.local_editing_preservation.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::LocalEditingTokenDrift,
                row.row_id.clone(),
                "local_editing_preservation_token",
                "local_editing_preservation_token must match local_editing_preservation",
            ));
        }
        if row.recovery_action_token != row.recovery_action.as_str() {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::RecoveryActionTokenDrift,
                row.row_id.clone(),
                "recovery_action_token",
                "recovery_action_token must match recovery_action",
            ));
        }

        let outcome = row.outcome_class;
        let impact = row.managed_capability_impact;
        if outcome.authorises_full_managed() {
            if !row.trust_anchor.source_class.authorises_managed() {
                defects.push(OfflineEntitlementVerifierBetaDefect::new(
                    OfflineEntitlementVerifierBetaDefectKind::VerifiedOutcomeWithoutTrustAnchor,
                    row.row_id.clone(),
                    "trust_anchor.source_class",
                    "verified outcome requires a managed trust anchor",
                ));
            }
            if !matches!(
                impact,
                ManagedCapabilityImpactClass::FullAuthorityActive
                    | ManagedCapabilityImpactClass::NotApplicableLocalOnly
            ) {
                defects.push(OfflineEntitlementVerifierBetaDefect::new(
                    OfflineEntitlementVerifierBetaDefectKind::OutcomeImpactMismatch,
                    row.row_id.clone(),
                    "managed_capability_impact",
                    "verified outcome must apply full authority or be local-only",
                ));
            }
        }
        if outcome.requires_downgrade() {
            if impact.is_full_authority() {
                let defect_kind = match outcome {
                    VerifierOutcomeClass::Expired => {
                        OfflineEntitlementVerifierBetaDefectKind::ExpiredBundleAcceptedWithoutDowngrade
                    }
                    VerifierOutcomeClass::UntrustedSigner => {
                        OfflineEntitlementVerifierBetaDefectKind::UntrustedSignerAccepted
                    }
                    _ => OfflineEntitlementVerifierBetaDefectKind::OutcomeImpactMismatch,
                };
                defects.push(OfflineEntitlementVerifierBetaDefect::new(
                    defect_kind,
                    row.row_id.clone(),
                    "managed_capability_impact",
                    "failed verifier outcome must narrow managed authority",
                ));
            }
            if !row.local_editing_preservation.preserves_local_editing() {
                defects.push(OfflineEntitlementVerifierBetaDefect::new(
                    OfflineEntitlementVerifierBetaDefectKind::LocalEditingBlockedOnFailedVerification,
                    row.row_id.clone(),
                    "local_editing_preservation",
                    "failed verifier outcome must preserve local editing",
                ));
            }
            if matches!(row.recovery_action, VerifierRecoveryActionClass::NoActionVerified) {
                defects.push(OfflineEntitlementVerifierBetaDefect::new(
                    OfflineEntitlementVerifierBetaDefectKind::DowngradeMissingRecoveryAction,
                    row.row_id.clone(),
                    "recovery_action",
                    "downgrade row must declare a recovery action",
                ));
            }
        }
        if matches!(outcome, VerifierOutcomeClass::UnsignedLocalAdvisory)
            && row.trust_anchor.source_class.authorises_managed()
        {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::UnsignedLocalAdvisoryOnManagedAnchor,
                row.row_id.clone(),
                "trust_anchor.source_class",
                "unsigned local advisory cannot reuse a managed trust anchor",
            ));
        }
        if !row.no_public_endpoint_fallback {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::HiddenPublicEndpointFallback,
                row.row_id.clone(),
                "no_public_endpoint_fallback",
                "row permits undeclared public endpoint fallback",
            ));
        }
        if !row.raw_private_material_excluded {
            defects.push(OfflineEntitlementVerifierBetaDefect::new(
                OfflineEntitlementVerifierBetaDefectKind::RawPrivateMaterialExposed,
                row.row_id.clone(),
                "raw_private_material_excluded",
                "verifier records must be export-safe metadata",
            ));
        }
    }

    let required_profiles: BTreeSet<&str> = OfflineEntitlementVerifierBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    let observed_profiles: BTreeSet<&str> =
        rows.iter().map(|row| row.profile_token.as_str()).collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(OfflineEntitlementVerifierBetaDefect::new(
            OfflineEntitlementVerifierBetaDefectKind::ProfileCoverageMissing,
            "page",
            "rows.profile_token",
            format!("missing {} profile coverage", missing),
        ));
    }
    let required_kinds: BTreeSet<&str> = VerifiedBundleKindClass::ALL
        .iter()
        .map(|kind| kind.as_str())
        .collect();
    let observed_kinds: BTreeSet<&str> = rows
        .iter()
        .map(|row| row.subject.bundle_kind_token.as_str())
        .collect();
    for missing in required_kinds.difference(&observed_kinds) {
        defects.push(OfflineEntitlementVerifierBetaDefect::new(
            OfflineEntitlementVerifierBetaDefectKind::BundleKindCoverageMissing,
            "page",
            "rows.subject.bundle_kind_token",
            format!("missing {} bundle kind coverage", missing),
        ));
    }

    defects
}

fn seeded_rows() -> Vec<OfflineEntitlementVerifierBetaRow> {
    vec![
        connected_policy_row(),
        connected_entitlement_row(),
        mirror_policy_row(),
        mirror_entitlement_row(),
        offline_policy_row(),
        offline_entitlement_expired_row(),
        enterprise_policy_row(),
        enterprise_entitlement_signature_missing_row(),
        untrusted_signer_drill_row(),
        unsigned_local_advisory_row(),
    ]
}

fn connected_policy_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:connected:policy",
        display_label: "Connected policy bundle, verified live",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::Connected,
        subject: subject(
            "policy_bundle.connected.2026.05",
            VerifiedBundleKindClass::PolicyBundle,
            "baseline@2026.05.0",
            "policy_epoch.connected.2026.05.0001",
            "signer:vendor-managed-baseline",
            "2026-05-01T00:00:00Z",
            "2026-08-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::VendorManagedRoot,
            "trust_anchor:vendor-managed-root:2026",
            "Vendor-managed signing root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::VerifiedLive,
        outcome_label: "Signed policy bundle verified against the live trust anchor.",
        managed_capability_impact: ManagedCapabilityImpactClass::FullAuthorityActive,
        local_editing_preservation: LocalEditingPreservationClass::Preserved,
        recovery_action: VerifierRecoveryActionClass::NoActionVerified,
        explanation_label:
            "Live verify against the vendor-managed root admits managed authority cleanly.",
    })
}

fn connected_entitlement_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:connected:entitlement",
        display_label: "Connected entitlement snapshot, verified live",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::Connected,
        subject: subject(
            "entitlement_snapshot.connected.2026.05",
            VerifiedBundleKindClass::EntitlementSnapshot,
            "managed_team@2026.05.0",
            "entitlement_epoch.connected.2026.05.0001",
            "signer:vendor-managed-entitlement",
            "2026-05-01T00:00:00Z",
            "2026-06-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::VendorManagedRoot,
            "trust_anchor:vendor-managed-root:2026",
            "Vendor-managed signing root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::VerifiedLive,
        outcome_label: "Entitlement snapshot verified live; seat active.",
        managed_capability_impact: ManagedCapabilityImpactClass::FullAuthorityActive,
        local_editing_preservation: LocalEditingPreservationClass::Preserved,
        recovery_action: VerifierRecoveryActionClass::NoActionVerified,
        explanation_label:
            "Live verify against the vendor-managed root admits the entitlement snapshot.",
    })
}

fn mirror_policy_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:mirror:policy",
        display_label: "Mirror policy bundle, verified mirror",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::MirrorOnly,
        subject: subject(
            "policy_bundle.mirror.2026.05",
            VerifiedBundleKindClass::PolicyBundle,
            "baseline@2026.05.0",
            "policy_epoch.mirror.2026.05.0001",
            "signer:vendor-managed-baseline",
            "2026-05-01T00:00:00Z",
            "2026-08-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::SignedMirrorRoot,
            "trust_anchor:signed-mirror-root:2026",
            "Signed mirror trust root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::VerifiedMirror,
        outcome_label: "Signed mirror policy bundle verified against the mirror root.",
        managed_capability_impact: ManagedCapabilityImpactClass::FullAuthorityActive,
        local_editing_preservation: LocalEditingPreservationClass::Preserved,
        recovery_action: VerifierRecoveryActionClass::NoActionVerified,
        explanation_label:
            "Mirror verify against the signed-mirror root preserves the upstream chain.",
    })
}

fn mirror_entitlement_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:mirror:entitlement",
        display_label: "Mirror entitlement snapshot, verified mirror",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::MirrorOnly,
        subject: subject(
            "entitlement_snapshot.mirror.2026.05",
            VerifiedBundleKindClass::EntitlementSnapshot,
            "managed_team@2026.05.0",
            "entitlement_epoch.mirror.2026.05.0001",
            "signer:vendor-managed-entitlement",
            "2026-05-01T00:00:00Z",
            "2026-06-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::SignedMirrorRoot,
            "trust_anchor:signed-mirror-root:2026",
            "Signed mirror trust root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::VerifiedMirror,
        outcome_label: "Entitlement snapshot verified against the signed mirror.",
        managed_capability_impact: ManagedCapabilityImpactClass::FullAuthorityActive,
        local_editing_preservation: LocalEditingPreservationClass::Preserved,
        recovery_action: VerifierRecoveryActionClass::NoActionVerified,
        explanation_label:
            "Mirror entitlement verify keeps the seat active without live vendor calls.",
    })
}

fn offline_policy_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:offline:policy",
        display_label: "Offline policy bundle, verified air-gapped",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::Offline,
        subject: subject(
            "policy_bundle.airgapped.2026.05",
            VerifiedBundleKindClass::PolicyBundle,
            "baseline@2026.05.0",
            "policy_epoch.airgapped.2026.05.0001",
            "signer:vendor-managed-baseline",
            "2026-05-01T00:00:00Z",
            "2026-09-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::AirGappedRoot,
            "trust_anchor:air-gapped-root:2026",
            "Air-gapped signing root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::VerifiedAirGapped,
        outcome_label: "Air-gapped policy bundle verified against the offline root.",
        managed_capability_impact: ManagedCapabilityImpactClass::FullAuthorityActive,
        local_editing_preservation: LocalEditingPreservationClass::Preserved,
        recovery_action: VerifierRecoveryActionClass::NoActionVerified,
        explanation_label:
            "Air-gapped verify proves policy authority without leaving the offline boundary.",
    })
}

fn offline_entitlement_expired_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:offline:entitlement-expired",
        display_label: "Offline entitlement snapshot, expired",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::Offline,
        subject: subject(
            "entitlement_snapshot.airgapped.2026.04",
            VerifiedBundleKindClass::EntitlementSnapshot,
            "managed_team@2026.04.0",
            "entitlement_epoch.airgapped.2026.04.0001",
            "signer:vendor-managed-entitlement",
            "2026-04-01T00:00:00Z",
            "2026-05-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::AirGappedRoot,
            "trust_anchor:air-gapped-root:2026",
            "Air-gapped signing root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::Expired,
        outcome_label: "Entitlement snapshot is past valid_until on the offline boundary.",
        managed_capability_impact: ManagedCapabilityImpactClass::NarrowedToInspectOnly,
        local_editing_preservation: LocalEditingPreservationClass::PreservedWithAdvisory,
        recovery_action: VerifierRecoveryActionClass::ImportAirGappedTransfer,
        explanation_label:
            "Expired offline entitlement narrows managed actions to inspect-only; local editing continues.",
    })
}

fn enterprise_policy_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:enterprise:policy",
        display_label: "Enterprise policy bundle, manual signed import",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::EnterpriseManaged,
        subject: subject(
            "policy_bundle.enterprise.2026.05",
            VerifiedBundleKindClass::PolicyBundle,
            "baseline@2026.05.0",
            "policy_epoch.enterprise.2026.05.0001",
            "signer:enterprise-admin-quorum",
            "2026-05-01T00:00:00Z",
            "2026-08-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::ManualImportRoot,
            "trust_anchor:manual-import-root:2026",
            "Manual signed-import root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::VerifiedManualImport,
        outcome_label: "Enterprise policy bundle verified against the manual-import root.",
        managed_capability_impact: ManagedCapabilityImpactClass::FullAuthorityActive,
        local_editing_preservation: LocalEditingPreservationClass::Preserved,
        recovery_action: VerifierRecoveryActionClass::NoActionVerified,
        explanation_label:
            "Manual signed import keeps the enterprise managed policy authority intact.",
    })
}

fn enterprise_entitlement_signature_missing_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:enterprise:entitlement-missing",
        display_label: "Enterprise entitlement snapshot, signature missing",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::EnterpriseManaged,
        subject: subject(
            "entitlement_snapshot.enterprise.2026.05",
            VerifiedBundleKindClass::EntitlementSnapshot,
            "managed_enterprise@2026.05.0",
            "entitlement_epoch.enterprise.2026.05.0001",
            "signer:enterprise-admin-quorum",
            "2026-05-01T00:00:00Z",
            "2026-06-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::ManualImportRoot,
            "trust_anchor:manual-import-root:2026",
            "Manual signed-import root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::SignatureMissing,
        outcome_label: "Enterprise entitlement snapshot arrived without a signature blob.",
        managed_capability_impact: ManagedCapabilityImpactClass::PausedWithVisibleRecovery,
        local_editing_preservation: LocalEditingPreservationClass::PreservedWithAdvisory,
        recovery_action: VerifierRecoveryActionClass::ImportSignedBundleFile,
        explanation_label:
            "Missing signature pauses managed entitlement actions until admin imports a signed file; local editing continues.",
    })
}

fn untrusted_signer_drill_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:connected:untrusted-signer",
        display_label: "Connected policy bundle from an untrusted signer",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::Connected,
        subject: subject(
            "policy_bundle.untrusted.2026.05",
            VerifiedBundleKindClass::PolicyBundle,
            "rogue@2026.05.0",
            "policy_epoch.untrusted.2026.05.0001",
            "signer:untrusted-experimental",
            "2026-05-15T00:00:00Z",
            "2026-08-15T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::VendorManagedRoot,
            "trust_anchor:vendor-managed-root:2026",
            "Vendor-managed signing root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::UntrustedSigner,
        outcome_label: "Bundle signed by a signer not on the vendor-managed trust anchor list.",
        managed_capability_impact: ManagedCapabilityImpactClass::BlockedPendingRepair,
        local_editing_preservation: LocalEditingPreservationClass::PreservedWithAdvisory,
        recovery_action: VerifierRecoveryActionClass::EscalateAdminQuorum,
        explanation_label:
            "Untrusted signer blocks managed authority pending admin quorum review; local editing continues.",
    })
}

fn unsigned_local_advisory_row() -> OfflineEntitlementVerifierBetaRow {
    OfflineEntitlementVerifierBetaRow::stage(StageOfflineEntitlementVerifierBetaRowRequest {
        row_id: "offline-entitlement-verifier:local:advisory",
        display_label: "Local advisory policy bundle, unsigned",
        profile_class: OfflineEntitlementVerifierBetaProfileClass::Connected,
        subject: subject(
            "policy_bundle.local_advisory.2026.05",
            VerifiedBundleKindClass::PolicyBundle,
            "local_advisory@2026.05.0",
            "policy_epoch.local_advisory.2026.05.0001",
            "signer:local-advisory-self",
            "2026-05-01T00:00:00Z",
            "2026-08-01T00:00:00Z",
        ),
        trust_anchor: anchor(
            TrustAnchorSourceClass::LocalAdvisoryNoRoot,
            "trust_anchor:local-advisory:no-root",
            "Local advisory, no trust root",
            "2026-01-01T00:00:00Z",
        ),
        outcome_class: VerifierOutcomeClass::UnsignedLocalAdvisory,
        outcome_label: "Unsigned local advisory bundle; no managed authority follows.",
        managed_capability_impact: ManagedCapabilityImpactClass::NotApplicableLocalOnly,
        local_editing_preservation: LocalEditingPreservationClass::NotApplicableLocalOnly,
        recovery_action: VerifierRecoveryActionClass::ContinueLocalOnly,
        explanation_label:
            "Local advisory bundles are inspectable but never carry managed authority.",
    })
}

fn subject(
    bundle_ref: &str,
    bundle_kind: VerifiedBundleKindClass,
    bundle_version: &str,
    bundle_epoch_ref: &str,
    signer_id: &str,
    signed_at: &str,
    valid_until: &str,
) -> VerifierBundleSubject {
    VerifierBundleSubject {
        bundle_ref: bundle_ref.to_owned(),
        bundle_kind,
        bundle_kind_token: bundle_kind.as_str().to_owned(),
        bundle_version: bundle_version.to_owned(),
        bundle_epoch_ref: bundle_epoch_ref.to_owned(),
        signer_id: signer_id.to_owned(),
        signed_at: signed_at.to_owned(),
        valid_until: valid_until.to_owned(),
    }
}

fn anchor(
    source_class: TrustAnchorSourceClass,
    anchor_ref: &str,
    anchor_label: &str,
    last_rotated_at: &str,
) -> VerifierTrustAnchor {
    VerifierTrustAnchor {
        source_class,
        source_token: source_class.as_str().to_owned(),
        anchor_ref: anchor_ref.to_owned(),
        anchor_label: anchor_label.to_owned(),
        last_rotated_at: last_rotated_at.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        validate_offline_entitlement_verifier_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        assert!(page.rows.len() >= 8);

        for required in OfflineEntitlementVerifierBetaProfileClass::ALL {
            assert!(
                page.summary
                    .profiles_present
                    .iter()
                    .any(|token| token == required.as_str()),
                "summary must list profile {}",
                required.as_str()
            );
        }
        for required in VerifiedBundleKindClass::ALL {
            assert!(
                page.summary
                    .bundle_kinds_present
                    .iter()
                    .any(|token| token == required.as_str()),
                "summary must list bundle kind {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn expired_row_narrows_managed_authority_but_preserves_local_editing() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        let expired = page
            .rows
            .iter()
            .find(|row| row.outcome_class == VerifierOutcomeClass::Expired)
            .expect("expired offline entitlement row");
        assert!(expired
            .managed_capability_impact
            .narrows_managed_authority());
        assert!(expired
            .local_editing_preservation
            .preserves_local_editing());
        assert_ne!(expired.recovery_action, VerifierRecoveryActionClass::NoActionVerified);
    }

    #[test]
    fn signature_missing_row_pauses_managed_actions_but_keeps_local_editing() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        let missing = page
            .rows
            .iter()
            .find(|row| row.outcome_class == VerifierOutcomeClass::SignatureMissing)
            .expect("signature-missing row");
        assert!(matches!(
            missing.managed_capability_impact,
            ManagedCapabilityImpactClass::PausedWithVisibleRecovery
        ));
        assert!(missing
            .local_editing_preservation
            .preserves_local_editing());
    }

    #[test]
    fn untrusted_signer_row_blocks_managed_authority() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        let untrusted = page
            .rows
            .iter()
            .find(|row| row.outcome_class == VerifierOutcomeClass::UntrustedSigner)
            .expect("untrusted signer row");
        assert!(matches!(
            untrusted.managed_capability_impact,
            ManagedCapabilityImpactClass::BlockedPendingRepair
        ));
        assert!(untrusted
            .local_editing_preservation
            .preserves_local_editing());
    }

    #[test]
    fn unsigned_local_advisory_row_uses_local_no_root_anchor() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        let advisory = page
            .rows
            .iter()
            .find(|row| row.outcome_class == VerifierOutcomeClass::UnsignedLocalAdvisory)
            .expect("local advisory row");
        assert_eq!(
            advisory.trust_anchor.source_class,
            TrustAnchorSourceClass::LocalAdvisoryNoRoot
        );
        assert_eq!(
            advisory.managed_capability_impact,
            ManagedCapabilityImpactClass::NotApplicableLocalOnly
        );
    }

    #[test]
    fn validator_rejects_expired_bundle_accepting_full_authority() {
        let mut page = seeded_offline_entitlement_verifier_beta_page();
        let expired = page
            .rows
            .iter_mut()
            .find(|row| row.outcome_class == VerifierOutcomeClass::Expired)
            .expect("expired row");
        expired.managed_capability_impact = ManagedCapabilityImpactClass::FullAuthorityActive;
        expired.managed_capability_impact_token =
            ManagedCapabilityImpactClass::FullAuthorityActive.as_str().to_owned();
        let defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == OfflineEntitlementVerifierBetaDefectKind::ExpiredBundleAcceptedWithoutDowngrade));
    }

    #[test]
    fn validator_rejects_failed_outcome_blocking_local_editing() {
        let mut page = seeded_offline_entitlement_verifier_beta_page();
        let missing = page
            .rows
            .iter_mut()
            .find(|row| row.outcome_class == VerifierOutcomeClass::SignatureMissing)
            .expect("missing row");
        missing.local_editing_preservation = LocalEditingPreservationClass::NotApplicableLocalOnly;
        missing.local_editing_preservation_token =
            LocalEditingPreservationClass::NotApplicableLocalOnly.as_str().to_owned();
        let defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == OfflineEntitlementVerifierBetaDefectKind::LocalEditingBlockedOnFailedVerification));
    }

    #[test]
    fn validator_rejects_untrusted_signer_accepted_as_full_authority() {
        let mut page = seeded_offline_entitlement_verifier_beta_page();
        let untrusted = page
            .rows
            .iter_mut()
            .find(|row| row.outcome_class == VerifierOutcomeClass::UntrustedSigner)
            .expect("untrusted row");
        untrusted.managed_capability_impact = ManagedCapabilityImpactClass::FullAuthorityActive;
        untrusted.managed_capability_impact_token =
            ManagedCapabilityImpactClass::FullAuthorityActive.as_str().to_owned();
        let defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == OfflineEntitlementVerifierBetaDefectKind::UntrustedSignerAccepted));
    }

    #[test]
    fn validator_rejects_unsigned_advisory_on_managed_anchor() {
        let mut page = seeded_offline_entitlement_verifier_beta_page();
        let advisory = page
            .rows
            .iter_mut()
            .find(|row| row.outcome_class == VerifierOutcomeClass::UnsignedLocalAdvisory)
            .expect("advisory row");
        advisory.trust_anchor.source_class = TrustAnchorSourceClass::VendorManagedRoot;
        advisory.trust_anchor.source_token =
            TrustAnchorSourceClass::VendorManagedRoot.as_str().to_owned();
        let defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == OfflineEntitlementVerifierBetaDefectKind::UnsignedLocalAdvisoryOnManagedAnchor));
    }

    #[test]
    fn validator_rejects_public_fallback() {
        let mut page = seeded_offline_entitlement_verifier_beta_page();
        page.rows[0].no_public_endpoint_fallback = false;
        let defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == OfflineEntitlementVerifierBetaDefectKind::HiddenPublicEndpointFallback));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        let export = OfflineEntitlementVerifierBetaSupportExport::from_page(
            "offline-entitlement-verifier:support-export:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert_eq!(export.support_rows.len(), export.page.rows.len());
    }
}
