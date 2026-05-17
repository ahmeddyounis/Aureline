//! Beta policy-pack inspection projection.
//!
//! This module promotes the workspace-trust beta page into an admin- and
//! support-facing policy-pack projection. It gives one inspectable record for:
//!
//! - the effective policy pack with origin, signature, signer, provenance,
//!   apply state, and per-surface effective rules;
//! - the policy-pack diff between a baseline pack and a candidate or staged
//!   pack so admins can review additions, removals, scope changes, effect
//!   changes, and reason-token changes before applying;
//! - the policy-pack explain trace that ties a denial decision in a product
//!   surface back to the originating pack, rule, and reason token;
//! - the policy-pack export wrapper used by support bundles, mirror feeds, and
//!   manual signed-file import receipts so signatures, provenance, and
//!   explanation fields survive transport across mirror-only, offline, and
//!   air-gapped lanes without forking truth or silently downgrading to public
//!   endpoints, plaintext secrets, or implicit managed assumptions.
//!
//! Surfaces (admin console, support bundle, shell trust center, headless
//! inspector, settings/help docs) consume [`seeded_policy_pack_beta_page`]
//! rather than re-deriving local "is_policy_signed" checks. The seed covers
//! the connected, mirror-only, offline, and enterprise-managed beta profiles
//! and exposes one denial trace per claimed product row so denial reasons in
//! product surfaces can be traced back to the same pack and rule identifiers.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::trust::{CapabilityAuthorityClass, LaunchWedgeCapabilityFamily};

/// Beta schema version exported with every policy-pack record.
pub const POLICY_PACK_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every policy-pack beta record.
pub const POLICY_PACK_BETA_SHARED_CONTRACT_REF: &str = "security:policy_pack_beta:v1";

/// Stable record kind for [`PolicyPackBetaPage`] payloads.
pub const POLICY_PACK_BETA_PAGE_RECORD_KIND: &str = "security_policy_pack_beta_page_record";

/// Stable record kind for [`PolicyPackBetaPack`] payloads.
pub const POLICY_PACK_BETA_PACK_RECORD_KIND: &str = "security_policy_pack_beta_pack_record";

/// Stable record kind for [`PolicyPackBetaRule`] payloads.
pub const POLICY_PACK_BETA_RULE_RECORD_KIND: &str = "security_policy_pack_beta_rule_record";

/// Stable record kind for [`PolicyPackBetaDiff`] payloads.
pub const POLICY_PACK_BETA_DIFF_RECORD_KIND: &str = "security_policy_pack_beta_diff_record";

/// Stable record kind for [`PolicyPackBetaDiffEntry`] payloads.
pub const POLICY_PACK_BETA_DIFF_ENTRY_RECORD_KIND: &str =
    "security_policy_pack_beta_diff_entry_record";

/// Stable record kind for [`PolicyPackBetaImportReceipt`] payloads.
pub const POLICY_PACK_BETA_IMPORT_RECEIPT_RECORD_KIND: &str =
    "security_policy_pack_beta_import_receipt_record";

/// Stable record kind for [`PolicyPackBetaDenialTrace`] payloads.
pub const POLICY_PACK_BETA_DENIAL_TRACE_RECORD_KIND: &str =
    "security_policy_pack_beta_denial_trace_record";

/// Stable record kind for [`PolicyPackBetaDefect`] payloads.
pub const POLICY_PACK_BETA_DEFECT_RECORD_KIND: &str = "security_policy_pack_beta_defect_record";

/// Stable record kind for [`PolicyPackBetaSupportExport`] payloads.
pub const POLICY_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_policy_pack_beta_support_export_record";

/// Stable record kind for [`PolicyPackBetaSummary`] payloads.
pub const POLICY_PACK_BETA_SUMMARY_RECORD_KIND: &str = "security_policy_pack_beta_summary_record";

/// Profile under which a pack is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackBetaProfileClass {
    /// Connected beta profile with live policy source.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline profile served from a last-known-good or air-gapped snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed policy narrowing.
    EnterpriseManaged,
}

impl PolicyPackBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on beta records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Origin class for the policy pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackSourceClass {
    /// Local advisory pack on disk; never sufficient for managed authority.
    LocalAdvisoryFile,
    /// Customer-operated self-hosted policy origin.
    CustomerSelfHostedOrigin,
    /// Vendor-managed policy origin.
    VendorManagedOrigin,
    /// Signed mirror of an authoritative policy origin.
    SignedMirrorOrigin,
    /// Manual signed file import.
    ManualSignedFileImport,
    /// Air-gapped signed transfer.
    AirGappedSignedTransfer,
    /// Build preload or first-run seed.
    RuntimePreloadOrigin,
}

impl PolicyPackSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAdvisoryFile => "local_advisory_file",
            Self::CustomerSelfHostedOrigin => "customer_self_hosted_origin",
            Self::VendorManagedOrigin => "vendor_managed_origin",
            Self::SignedMirrorOrigin => "signed_mirror_origin",
            Self::ManualSignedFileImport => "manual_signed_file_import",
            Self::AirGappedSignedTransfer => "air_gapped_signed_transfer",
            Self::RuntimePreloadOrigin => "runtime_preload_origin",
        }
    }

    /// True when this origin requires a verified signature before authority widens.
    pub const fn requires_signature(self) -> bool {
        !matches!(self, Self::LocalAdvisoryFile | Self::RuntimePreloadOrigin)
    }

    /// True when this origin arrives through a mirror or manual import lane and
    /// must preserve its provenance and signature fields verbatim.
    pub const fn is_mirror_or_manual(self) -> bool {
        matches!(
            self,
            Self::SignedMirrorOrigin | Self::ManualSignedFileImport | Self::AirGappedSignedTransfer
        )
    }
}

/// Signature state attached to a policy pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackSignatureStateClass {
    /// Signature verified live against the authoritative origin.
    VerifiedLive,
    /// Signature verified against a signed mirror trust anchor.
    VerifiedMirror,
    /// Signature verified during manual signed-file import.
    VerifiedManualImport,
    /// Signature verified during air-gapped signed transfer.
    VerifiedAirGapped,
    /// Signature is not required for this origin (local advisory or preload).
    NotRequiredLocalOrigin,
    /// Signature is missing on an origin that requires one.
    MissingForRequiredOrigin,
    /// Signature was present but did not validate.
    Invalid,
}

impl PolicyPackSignatureStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedLive => "verified_live",
            Self::VerifiedMirror => "verified_mirror",
            Self::VerifiedManualImport => "verified_manual_import",
            Self::VerifiedAirGapped => "verified_air_gapped",
            Self::NotRequiredLocalOrigin => "not_required_local_origin",
            Self::MissingForRequiredOrigin => "missing_for_required_origin",
            Self::Invalid => "invalid",
        }
    }

    /// True when this signature state is sufficient for managed-authority
    /// effects to apply without silently falling back to a wider posture.
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::VerifiedLive
                | Self::VerifiedMirror
                | Self::VerifiedManualImport
                | Self::VerifiedAirGapped
                | Self::NotRequiredLocalOrigin
        )
    }
}

/// Apply state for a policy pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackApplyStateClass {
    /// Pack is currently effective on this profile.
    Effective,
    /// Pack is staged and awaiting admin review or rollout.
    StagedAwaitingReview,
    /// Pack was replaced by a newer version.
    ReplacedBySuccessor,
    /// Pack was rolled back or revoked.
    Revoked,
    /// Pack has not been applied on this profile.
    NeverApplied,
}

impl PolicyPackApplyStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Effective => "effective",
            Self::StagedAwaitingReview => "staged_awaiting_review",
            Self::ReplacedBySuccessor => "replaced_by_successor",
            Self::Revoked => "revoked",
            Self::NeverApplied => "never_applied",
        }
    }
}

/// Rule effect applied to a capability family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackRuleEffectClass {
    /// Rule narrows authority by denying the capability.
    Deny,
    /// Rule narrows authority by requiring a per-invocation approval.
    RequireApproval,
    /// Rule narrows authority to a read-only or preview posture.
    NarrowToReadOrPreview,
    /// Rule explicitly allows the capability with the matrix default.
    AllowMatrixDefault,
    /// Rule emits an audit trail without changing the matrix authority.
    AuditOnly,
}

impl PolicyPackRuleEffectClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deny => "deny",
            Self::RequireApproval => "require_approval",
            Self::NarrowToReadOrPreview => "narrow_to_read_or_preview",
            Self::AllowMatrixDefault => "allow_matrix_default",
            Self::AuditOnly => "audit_only",
        }
    }

    /// The narrowed authority class implied by the rule effect.
    pub const fn applied_authority(self) -> CapabilityAuthorityClass {
        match self {
            Self::Deny => CapabilityAuthorityClass::PolicyDenied,
            Self::RequireApproval => CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            Self::NarrowToReadOrPreview => CapabilityAuthorityClass::DegradedPreviewOnly,
            Self::AllowMatrixDefault => CapabilityAuthorityClass::Allowed,
            Self::AuditOnly => CapabilityAuthorityClass::Allowed,
        }
    }

    /// True when this effect must surface a denial reason on a product row.
    pub const fn surfaces_denial(self) -> bool {
        matches!(
            self,
            Self::Deny | Self::RequireApproval | Self::NarrowToReadOrPreview
        )
    }
}

/// Diff entry kind classifying a rule change between two packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackDiffEntryKind {
    /// Rule was added in the target pack.
    Added,
    /// Rule was removed in the target pack.
    Removed,
    /// Rule effect changed between packs.
    EffectChanged,
    /// Rule scope changed between packs.
    ScopeChanged,
    /// Rule reason token changed between packs.
    ReasonChanged,
}

impl PolicyPackDiffEntryKind {
    /// Stable token recorded on diff entries.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::EffectChanged => "effect_changed",
            Self::ScopeChanged => "scope_changed",
            Self::ReasonChanged => "reason_changed",
        }
    }
}

/// Typed validator defect for the policy-pack beta page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPackBetaDefectKind {
    /// A pack exposes managed authority without a verified signature.
    UnsignedManagedAuthority,
    /// A pack's source token does not match its source class.
    SourceTokenDrift,
    /// A pack's signature token does not match its signature state.
    SignatureTokenDrift,
    /// A mirror or manual-import pack drops the upstream provenance pointer.
    MirrorOrImportProvenanceDropped,
    /// A mirror or manual-import pack drops the preserved signature blob ref.
    MirrorOrImportSignatureBlobDropped,
    /// A mirror or manual-import pack drops the rule explanation field.
    MirrorOrImportExplanationDropped,
    /// A pack permits an undeclared public endpoint fallback.
    HiddenPublicEndpointFallback,
    /// A pack does not declare profile applicability for a required profile.
    ProfileCoverageMissing,
    /// A diff entry's effect does not match the resolved before/after rules.
    DiffEntryEffectMismatch,
    /// A denial trace cannot be resolved to a pack and rule pair.
    DenialTraceUnresolvable,
    /// A denial trace references a rule whose effect does not surface denial.
    DenialTraceEffectNotDenial,
    /// A surface family appears in a denial trace but is absent from the
    /// active pack rules.
    DenialTraceSurfaceMissing,
    /// A record would expose raw private or secret material.
    RawPrivateMaterialExposed,
}

impl PolicyPackBetaDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsignedManagedAuthority => "unsigned_managed_authority",
            Self::SourceTokenDrift => "source_token_drift",
            Self::SignatureTokenDrift => "signature_token_drift",
            Self::MirrorOrImportProvenanceDropped => "mirror_or_import_provenance_dropped",
            Self::MirrorOrImportSignatureBlobDropped => "mirror_or_import_signature_blob_dropped",
            Self::MirrorOrImportExplanationDropped => "mirror_or_import_explanation_dropped",
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::DiffEntryEffectMismatch => "diff_entry_effect_mismatch",
            Self::DenialTraceUnresolvable => "denial_trace_unresolvable",
            Self::DenialTraceEffectNotDenial => "denial_trace_effect_not_denial",
            Self::DenialTraceSurfaceMissing => "denial_trace_surface_missing",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
        }
    }
}

/// Reviewable provenance bundle attached to a pack or import receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackProvenance {
    /// Authoritative origin URL or filesystem reference.
    pub origin_ref: String,
    /// Signer identity recorded on the signed bundle.
    pub signer_id: String,
    /// Signed-at timestamp on the bundle.
    pub signed_at: String,
    /// Fetched-at timestamp on this profile.
    pub fetched_at: String,
    /// Mirror or import path label, if any.
    pub transport_label: String,
    /// Stable ref to the preserved signature blob in the artifact store.
    pub signature_blob_ref: String,
}

/// One policy-pack rule applied to a capability family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaRule {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable rule id within its pack.
    pub rule_id: String,
    /// Capability family this rule targets.
    pub surface_family: LaunchWedgeCapabilityFamily,
    /// Stable token for [`Self::surface_family`].
    pub surface_family_token: String,
    /// Effect class.
    pub effect: PolicyPackRuleEffectClass,
    /// Stable token for [`Self::effect`].
    pub effect_token: String,
    /// Narrowed authority implied by [`Self::effect`].
    pub applied_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::applied_authority`].
    pub applied_authority_token: String,
    /// Reviewable scope label naming the workspace, identity, or surface scope.
    pub scope_label: String,
    /// Stable reason token surfaced on product denials caused by this rule.
    pub reason_token: String,
    /// Plain-language explanation displayed by inspectors and support exports.
    pub explanation: String,
}

/// One policy pack inspected on this profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaPack {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable pack id (namespace and version captured by [`Self::pack_version`]).
    pub pack_id: String,
    /// Pack version string.
    pub pack_version: String,
    /// Source class.
    pub source_class: PolicyPackSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_token: String,
    /// Signature state.
    pub signature_state: PolicyPackSignatureStateClass,
    /// Stable token for [`Self::signature_state`].
    pub signature_state_token: String,
    /// Apply state.
    pub apply_state: PolicyPackApplyStateClass,
    /// Stable token for [`Self::apply_state`].
    pub apply_state_token: String,
    /// Reviewable provenance.
    pub provenance: PolicyPackProvenance,
    /// Effective rules.
    pub rules: Vec<PolicyPackBetaRule>,
    /// Profile tokens this pack applies to.
    pub applies_to_profiles: Vec<String>,
    /// True when no undeclared public endpoint fallback is permitted.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private/secret material is excluded from the record.
    pub raw_private_material_excluded: bool,
}

/// One entry in a policy-pack diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaDiffEntry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Diff entry kind.
    pub kind: PolicyPackDiffEntryKind,
    /// Stable token for [`Self::kind`].
    pub kind_token: String,
    /// Rule id affected by this entry.
    pub rule_id: String,
    /// Surface family token affected by this entry.
    pub surface_family_token: String,
    /// Effect token in the base pack, or `not_present` if added.
    pub before_effect_token: String,
    /// Effect token in the target pack, or `not_present` if removed.
    pub after_effect_token: String,
    /// Scope label in the base pack, or `not_present` if added.
    pub before_scope_label: String,
    /// Scope label in the target pack, or `not_present` if removed.
    pub after_scope_label: String,
    /// Reason token in the base pack, or `not_present` if added.
    pub before_reason_token: String,
    /// Reason token in the target pack, or `not_present` if removed.
    pub after_reason_token: String,
    /// Plain-language note rendered by inspectors.
    pub note: String,
}

/// Diff between a base pack and a target/staged pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaDiff {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Base pack id.
    pub base_pack_id: String,
    /// Target pack id.
    pub target_pack_id: String,
    /// Profile token under which this diff applies.
    pub profile_token: String,
    /// Diff entries.
    pub entries: Vec<PolicyPackBetaDiffEntry>,
}

/// Receipt produced when a mirror or manual-import pack lands on a profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaImportReceipt {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable receipt id.
    pub receipt_id: String,
    /// Pack id this receipt covers.
    pub pack_id: String,
    /// Pack version string.
    pub pack_version: String,
    /// Source class on the receipt.
    pub source_class: PolicyPackSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_token: String,
    /// Signature state on the receipt.
    pub signature_state: PolicyPackSignatureStateClass,
    /// Stable token for [`Self::signature_state`].
    pub signature_state_token: String,
    /// Provenance preserved verbatim from the upstream pack.
    pub provenance: PolicyPackProvenance,
    /// Profile tokens this receipt is valid for.
    pub applies_to_profiles: Vec<String>,
    /// True when the upstream signature blob ref is preserved.
    pub preserves_signature_blob: bool,
    /// True when upstream provenance is preserved verbatim.
    pub preserves_provenance: bool,
    /// True when upstream rule explanations are preserved verbatim.
    pub preserves_explanation: bool,
}

/// Explain trace tying a product-surface denial back to a pack and rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaDenialTrace {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable trace id.
    pub trace_id: String,
    /// Pack id referenced by this trace.
    pub pack_id: String,
    /// Rule id referenced by this trace.
    pub rule_id: String,
    /// Surface family the denial was raised against.
    pub surface_family: LaunchWedgeCapabilityFamily,
    /// Stable token for [`Self::surface_family`].
    pub surface_family_token: String,
    /// Reason token surfaced to the product row.
    pub reason_token: String,
    /// Plain-language explanation rendered by inspectors.
    pub explanation: String,
    /// Profile token this trace applies to.
    pub profile_token: String,
}

/// Typed validation defect for the policy-pack beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: PolicyPackBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (pack, rule, diff entry, or trace).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl PolicyPackBetaDefect {
    fn new(
        defect_kind: PolicyPackBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: POLICY_PACK_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the policy-pack beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of inspected packs.
    pub pack_count: usize,
    /// Number of effective packs (apply_state = effective).
    pub effective_pack_count: usize,
    /// Number of mirror-or-manual import receipts on the page.
    pub mirror_or_import_receipt_count: usize,
    /// Number of diff records on the page.
    pub diff_count: usize,
    /// Number of denial traces on the page.
    pub denial_trace_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Source tokens present across the page.
    pub source_tokens_present: Vec<String>,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl PolicyPackBetaSummary {
    /// Builds a summary over packs, diffs, traces, receipts, and defects.
    pub fn from_records(
        packs: &[PolicyPackBetaPack],
        diffs: &[PolicyPackBetaDiff],
        traces: &[PolicyPackBetaDenialTrace],
        receipts: &[PolicyPackBetaImportReceipt],
        defects: &[PolicyPackBetaDefect],
    ) -> Self {
        let profiles_present: BTreeSet<String> = packs
            .iter()
            .flat_map(|pack| pack.applies_to_profiles.iter().cloned())
            .collect();
        let source_tokens_present: BTreeSet<String> =
            packs.iter().map(|pack| pack.source_token.clone()).collect();
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: POLICY_PACK_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: POLICY_PACK_BETA_SUMMARY_RECORD_KIND.to_owned(),
            pack_count: packs.len(),
            effective_pack_count: packs
                .iter()
                .filter(|pack| pack.apply_state == PolicyPackApplyStateClass::Effective)
                .count(),
            mirror_or_import_receipt_count: receipts.len(),
            diff_count: diffs.len(),
            denial_trace_count: traces.len(),
            profiles_present: profiles_present.into_iter().collect(),
            source_tokens_present: source_tokens_present.into_iter().collect(),
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin, support, shell, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Inspected packs (ordered base, target, profile-specific imports).
    pub packs: Vec<PolicyPackBetaPack>,
    /// Diffs comparing base and target/staged packs per profile.
    pub diffs: Vec<PolicyPackBetaDiff>,
    /// Denial traces tying product-row denials to pack and rule ids.
    pub denial_traces: Vec<PolicyPackBetaDenialTrace>,
    /// Mirror or manual-import receipts.
    pub import_receipts: Vec<PolicyPackBetaImportReceipt>,
    /// Typed validation defects.
    pub defects: Vec<PolicyPackBetaDefect>,
    /// Aggregate summary.
    pub summary: PolicyPackBetaSummary,
}

/// Support-export wrapper for the policy-pack beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaSupportExport {
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
    pub page: PolicyPackBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private/secret material is excluded.
    pub raw_private_material_excluded: bool,
}

impl PolicyPackBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: PolicyPackBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: POLICY_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_private_material_excluded: true,
        }
    }
}

/// Builds the seeded policy-pack beta page covering connected, mirror,
/// offline, and enterprise-managed profiles together with the diff, explain,
/// and import-receipt baselines.
pub fn seeded_policy_pack_beta_page() -> PolicyPackBetaPage {
    let base_pack = seed_base_pack();
    let target_pack = seed_target_pack();
    let mirror_pack = seed_mirror_pack(&target_pack);
    let manual_import_pack = seed_manual_import_pack(&target_pack);
    let offline_pack = seed_offline_pack(&target_pack);

    let packs = vec![
        base_pack.clone(),
        target_pack.clone(),
        mirror_pack.clone(),
        manual_import_pack.clone(),
        offline_pack.clone(),
    ];

    let diffs = vec![
        seed_diff(
            &base_pack,
            &target_pack,
            PolicyPackBetaProfileClass::Connected,
        ),
        seed_diff(
            &base_pack,
            &mirror_pack,
            PolicyPackBetaProfileClass::MirrorOnly,
        ),
        seed_diff(
            &base_pack,
            &manual_import_pack,
            PolicyPackBetaProfileClass::EnterpriseManaged,
        ),
        seed_diff(
            &base_pack,
            &offline_pack,
            PolicyPackBetaProfileClass::Offline,
        ),
    ];

    let denial_traces = seed_denial_traces(&target_pack);
    let import_receipts = vec![
        seed_import_receipt(&mirror_pack, "policy-pack-beta:import:mirror-001"),
        seed_import_receipt(&manual_import_pack, "policy-pack-beta:import:manual-001"),
        seed_import_receipt(&offline_pack, "policy-pack-beta:import:airgap-001"),
    ];

    let defects = audit_policy_pack_beta_page(&packs, &diffs, &denial_traces, &import_receipts);
    let summary = PolicyPackBetaSummary::from_records(
        &packs,
        &diffs,
        &denial_traces,
        &import_receipts,
        &defects,
    );

    PolicyPackBetaPage {
        record_kind: POLICY_PACK_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: "artifacts/security/policy_pack_matrix.yaml".to_owned(),
        packs,
        diffs,
        denial_traces,
        import_receipts,
        defects,
        summary,
    }
}

/// Validates the page and returns typed defects on failure.
pub fn validate_policy_pack_beta_page(
    page: &PolicyPackBetaPage,
) -> Result<(), Vec<PolicyPackBetaDefect>> {
    let defects = audit_policy_pack_beta_page(
        &page.packs,
        &page.diffs,
        &page.denial_traces,
        &page.import_receipts,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for a policy-pack beta page.
pub fn audit_policy_pack_beta_page(
    packs: &[PolicyPackBetaPack],
    diffs: &[PolicyPackBetaDiff],
    denial_traces: &[PolicyPackBetaDenialTrace],
    import_receipts: &[PolicyPackBetaImportReceipt],
) -> Vec<PolicyPackBetaDefect> {
    let mut defects = Vec::new();
    let pack_by_id: BTreeMap<&str, &PolicyPackBetaPack> = packs
        .iter()
        .map(|pack| (pack.pack_id.as_str(), pack))
        .collect();

    for pack in packs {
        if pack.source_token != pack.source_class.as_str() {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::SourceTokenDrift,
                pack.pack_id.clone(),
                "source_token",
                "source_token must match source_class",
            ));
        }
        if pack.signature_state_token != pack.signature_state.as_str() {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::SignatureTokenDrift,
                pack.pack_id.clone(),
                "signature_state_token",
                "signature_state_token must match signature_state",
            ));
        }
        if narrows_managed_authority(pack) && !pack.signature_state.is_verified() {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::UnsignedManagedAuthority,
                pack.pack_id.clone(),
                "signature_state",
                "managed-authority pack must present a verified signature",
            ));
        }
        if !pack.no_public_endpoint_fallback {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::HiddenPublicEndpointFallback,
                pack.pack_id.clone(),
                "no_public_endpoint_fallback",
                "pack permits undeclared public endpoint fallback",
            ));
        }
        if !pack.raw_private_material_excluded {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::RawPrivateMaterialExposed,
                pack.pack_id.clone(),
                "raw_private_material_excluded",
                "policy pack records must be export-safe metadata",
            ));
        }
        if pack.applies_to_profiles.is_empty() {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::ProfileCoverageMissing,
                pack.pack_id.clone(),
                "applies_to_profiles",
                "pack must declare at least one applicable profile",
            ));
        }
    }

    let required_profiles: BTreeSet<&str> = PolicyPackBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    let observed_profiles: BTreeSet<&str> = packs
        .iter()
        .flat_map(|pack| pack.applies_to_profiles.iter().map(String::as_str))
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(PolicyPackBetaDefect::new(
            PolicyPackBetaDefectKind::ProfileCoverageMissing,
            "page",
            "packs.applies_to_profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    for receipt in import_receipts {
        let Some(pack) = pack_by_id.get(receipt.pack_id.as_str()) else {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::MirrorOrImportProvenanceDropped,
                receipt.receipt_id.clone(),
                "pack_id",
                "import receipt references unknown pack",
            ));
            continue;
        };
        if !receipt.preserves_provenance
            || receipt.provenance.origin_ref != pack.provenance.origin_ref
            || receipt.provenance.signer_id != pack.provenance.signer_id
            || receipt.provenance.signed_at != pack.provenance.signed_at
        {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::MirrorOrImportProvenanceDropped,
                receipt.receipt_id.clone(),
                "provenance",
                "import receipt must preserve upstream provenance verbatim",
            ));
        }
        if !receipt.preserves_signature_blob
            || receipt.provenance.signature_blob_ref != pack.provenance.signature_blob_ref
        {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::MirrorOrImportSignatureBlobDropped,
                receipt.receipt_id.clone(),
                "signature_blob_ref",
                "import receipt must preserve upstream signature blob ref",
            ));
        }
        if !receipt.preserves_explanation
            || pack.rules.iter().any(|rule| rule.explanation.is_empty())
        {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::MirrorOrImportExplanationDropped,
                receipt.receipt_id.clone(),
                "preserves_explanation",
                "import receipt must preserve upstream explanation fields",
            ));
        }
    }

    for diff in diffs {
        let base = pack_by_id.get(diff.base_pack_id.as_str());
        let target = pack_by_id.get(diff.target_pack_id.as_str());
        for entry in &diff.entries {
            if entry.kind_token != entry.kind.as_str() {
                defects.push(PolicyPackBetaDefect::new(
                    PolicyPackBetaDefectKind::DiffEntryEffectMismatch,
                    format!("{}::{}", diff.base_pack_id, entry.rule_id),
                    "kind_token",
                    "diff entry kind_token must match kind",
                ));
                continue;
            }
            let base_rule =
                base.and_then(|pack| pack.rules.iter().find(|rule| rule.rule_id == entry.rule_id));
            let target_rule = target
                .and_then(|pack| pack.rules.iter().find(|rule| rule.rule_id == entry.rule_id));
            let resolved_before = base_rule
                .map(|rule| rule.effect_token.clone())
                .unwrap_or_else(|| "not_present".to_owned());
            let resolved_after = target_rule
                .map(|rule| rule.effect_token.clone())
                .unwrap_or_else(|| "not_present".to_owned());
            if entry.before_effect_token != resolved_before
                || entry.after_effect_token != resolved_after
            {
                defects.push(PolicyPackBetaDefect::new(
                    PolicyPackBetaDefectKind::DiffEntryEffectMismatch,
                    format!("{}::{}", diff.target_pack_id, entry.rule_id),
                    "before_after_effect_tokens",
                    "diff entry before/after effect tokens must resolve from the referenced packs",
                ));
            }
        }
    }

    for trace in denial_traces {
        let Some(pack) = pack_by_id.get(trace.pack_id.as_str()) else {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::DenialTraceUnresolvable,
                trace.trace_id.clone(),
                "pack_id",
                "denial trace references unknown pack",
            ));
            continue;
        };
        let Some(rule) = pack.rules.iter().find(|rule| rule.rule_id == trace.rule_id) else {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::DenialTraceUnresolvable,
                trace.trace_id.clone(),
                "rule_id",
                "denial trace references unknown rule",
            ));
            continue;
        };
        if !rule.effect.surfaces_denial() {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::DenialTraceEffectNotDenial,
                trace.trace_id.clone(),
                "rule.effect",
                "denial trace points at a rule whose effect does not surface denial",
            ));
        }
        if rule.surface_family_token != trace.surface_family_token {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::DenialTraceSurfaceMissing,
                trace.trace_id.clone(),
                "surface_family_token",
                "denial trace surface does not match rule surface",
            ));
        }
        if rule.reason_token != trace.reason_token {
            defects.push(PolicyPackBetaDefect::new(
                PolicyPackBetaDefectKind::DenialTraceUnresolvable,
                trace.trace_id.clone(),
                "reason_token",
                "denial trace reason_token does not match rule reason_token",
            ));
        }
    }

    defects
}

fn narrows_managed_authority(pack: &PolicyPackBetaPack) -> bool {
    pack.source_class.requires_signature()
        && pack.rules.iter().any(|rule| rule.effect.surfaces_denial())
}

fn seed_base_pack() -> PolicyPackBetaPack {
    let provenance = PolicyPackProvenance {
        origin_ref: "https://policy.aureline.example/packs/baseline/2026.04.0".to_owned(),
        signer_id: "policy-signer:aureline-baseline".to_owned(),
        signed_at: "2026-04-01T00:00:00Z".to_owned(),
        fetched_at: "2026-04-01T00:05:00Z".to_owned(),
        transport_label: "https-managed-origin".to_owned(),
        signature_blob_ref: "artifacts/security/policy_packs/baseline-2026.04.0.sig".to_owned(),
    };
    let rules = vec![
        rule(
            "rule:provider-tool-call",
            LaunchWedgeCapabilityFamily::ConnectedProviderToolCall,
            PolicyPackRuleEffectClass::RequireApproval,
            "workspace:default",
            "provider_tool_call_requires_approval",
            "Provider tool calls require a per-invocation approval ticket.",
        ),
        rule(
            "rule:remote-attach",
            LaunchWedgeCapabilityFamily::RemoteAttach,
            PolicyPackRuleEffectClass::Deny,
            "workspace:default",
            "remote_attach_blocked_by_baseline",
            "Remote attach is denied until the baseline pack is replaced.",
        ),
        rule(
            "rule:admin-policy-read",
            LaunchWedgeCapabilityFamily::AdminPolicyRead,
            PolicyPackRuleEffectClass::AuditOnly,
            "workspace:default",
            "admin_policy_read_audited",
            "Admin policy reads are audited but admitted.",
        ),
    ];
    pack(
        "policy-pack-beta:baseline:2026.04.0",
        "2026.04.0",
        PolicyPackSourceClass::VendorManagedOrigin,
        PolicyPackSignatureStateClass::VerifiedLive,
        PolicyPackApplyStateClass::ReplacedBySuccessor,
        provenance,
        rules,
        &[PolicyPackBetaProfileClass::Connected],
    )
}

fn seed_target_pack() -> PolicyPackBetaPack {
    let provenance = PolicyPackProvenance {
        origin_ref: "https://policy.aureline.example/packs/baseline/2026.05.0".to_owned(),
        signer_id: "policy-signer:aureline-baseline".to_owned(),
        signed_at: "2026-05-01T00:00:00Z".to_owned(),
        fetched_at: "2026-05-15T00:05:00Z".to_owned(),
        transport_label: "https-managed-origin".to_owned(),
        signature_blob_ref: "artifacts/security/policy_packs/baseline-2026.05.0.sig".to_owned(),
    };
    let rules = vec![
        rule(
            "rule:provider-tool-call",
            LaunchWedgeCapabilityFamily::ConnectedProviderToolCall,
            PolicyPackRuleEffectClass::RequireApproval,
            "workspace:default",
            "provider_tool_call_requires_approval",
            "Provider tool calls require a per-invocation approval ticket.",
        ),
        rule(
            "rule:remote-attach",
            LaunchWedgeCapabilityFamily::RemoteAttach,
            PolicyPackRuleEffectClass::RequireApproval,
            "workspace:reviewed-targets",
            "remote_attach_requires_review",
            "Remote attach requires per-target review approval.",
        ),
        rule(
            "rule:ai-tool-call",
            LaunchWedgeCapabilityFamily::AiToolCallMutating,
            PolicyPackRuleEffectClass::Deny,
            "workspace:default",
            "ai_tool_call_blocked_by_policy",
            "AI mutating tool calls are denied for this workspace scope.",
        ),
        rule(
            "rule:admin-policy-read",
            LaunchWedgeCapabilityFamily::AdminPolicyRead,
            PolicyPackRuleEffectClass::AuditOnly,
            "workspace:default",
            "admin_policy_read_audited",
            "Admin policy reads are audited but admitted.",
        ),
    ];
    pack(
        "policy-pack-beta:baseline:2026.05.0",
        "2026.05.0",
        PolicyPackSourceClass::VendorManagedOrigin,
        PolicyPackSignatureStateClass::VerifiedLive,
        PolicyPackApplyStateClass::Effective,
        provenance,
        rules,
        &[PolicyPackBetaProfileClass::Connected],
    )
}

fn seed_mirror_pack(target: &PolicyPackBetaPack) -> PolicyPackBetaPack {
    let mut mirror = target.clone();
    mirror.pack_id = "policy-pack-beta:mirror:2026.05.0".to_owned();
    mirror.source_class = PolicyPackSourceClass::SignedMirrorOrigin;
    mirror.source_token = PolicyPackSourceClass::SignedMirrorOrigin
        .as_str()
        .to_owned();
    mirror.signature_state = PolicyPackSignatureStateClass::VerifiedMirror;
    mirror.signature_state_token = PolicyPackSignatureStateClass::VerifiedMirror
        .as_str()
        .to_owned();
    mirror.provenance.transport_label = "signed-mirror".to_owned();
    mirror.provenance.fetched_at = "2026-05-15T00:10:00Z".to_owned();
    mirror.applies_to_profiles = vec![PolicyPackBetaProfileClass::MirrorOnly.as_str().to_owned()];
    mirror
}

fn seed_manual_import_pack(target: &PolicyPackBetaPack) -> PolicyPackBetaPack {
    let mut manual = target.clone();
    manual.pack_id = "policy-pack-beta:manual:2026.05.0".to_owned();
    manual.source_class = PolicyPackSourceClass::ManualSignedFileImport;
    manual.source_token = PolicyPackSourceClass::ManualSignedFileImport
        .as_str()
        .to_owned();
    manual.signature_state = PolicyPackSignatureStateClass::VerifiedManualImport;
    manual.signature_state_token = PolicyPackSignatureStateClass::VerifiedManualImport
        .as_str()
        .to_owned();
    manual.provenance.transport_label = "manual-signed-file-import".to_owned();
    manual.provenance.fetched_at = "2026-05-15T00:20:00Z".to_owned();
    manual.applies_to_profiles = vec![PolicyPackBetaProfileClass::EnterpriseManaged
        .as_str()
        .to_owned()];
    manual
}

fn seed_offline_pack(target: &PolicyPackBetaPack) -> PolicyPackBetaPack {
    let mut offline = target.clone();
    offline.pack_id = "policy-pack-beta:airgapped:2026.05.0".to_owned();
    offline.source_class = PolicyPackSourceClass::AirGappedSignedTransfer;
    offline.source_token = PolicyPackSourceClass::AirGappedSignedTransfer
        .as_str()
        .to_owned();
    offline.signature_state = PolicyPackSignatureStateClass::VerifiedAirGapped;
    offline.signature_state_token = PolicyPackSignatureStateClass::VerifiedAirGapped
        .as_str()
        .to_owned();
    offline.provenance.transport_label = "air-gapped-signed-transfer".to_owned();
    offline.provenance.fetched_at = "2026-05-15T00:30:00Z".to_owned();
    offline.applies_to_profiles = vec![PolicyPackBetaProfileClass::Offline.as_str().to_owned()];
    offline
}

fn seed_diff(
    base: &PolicyPackBetaPack,
    target: &PolicyPackBetaPack,
    profile: PolicyPackBetaProfileClass,
) -> PolicyPackBetaDiff {
    let entries = diff_entries(base, target);
    PolicyPackBetaDiff {
        record_kind: POLICY_PACK_BETA_DIFF_RECORD_KIND.to_owned(),
        schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
        base_pack_id: base.pack_id.clone(),
        target_pack_id: target.pack_id.clone(),
        profile_token: profile.as_str().to_owned(),
        entries,
    }
}

fn diff_entries(
    base: &PolicyPackBetaPack,
    target: &PolicyPackBetaPack,
) -> Vec<PolicyPackBetaDiffEntry> {
    let mut entries = Vec::new();
    let base_by_id: BTreeMap<&str, &PolicyPackBetaRule> = base
        .rules
        .iter()
        .map(|rule| (rule.rule_id.as_str(), rule))
        .collect();
    let target_by_id: BTreeMap<&str, &PolicyPackBetaRule> = target
        .rules
        .iter()
        .map(|rule| (rule.rule_id.as_str(), rule))
        .collect();
    let mut rule_ids: BTreeSet<&str> = base_by_id.keys().copied().collect();
    rule_ids.extend(target_by_id.keys().copied());

    for rule_id in rule_ids {
        match (base_by_id.get(rule_id), target_by_id.get(rule_id)) {
            (None, Some(target_rule)) => {
                entries.push(diff_entry(
                    PolicyPackDiffEntryKind::Added,
                    None,
                    Some(target_rule),
                    "Rule was added in the target pack.",
                ));
            }
            (Some(base_rule), None) => {
                entries.push(diff_entry(
                    PolicyPackDiffEntryKind::Removed,
                    Some(base_rule),
                    None,
                    "Rule was removed in the target pack.",
                ));
            }
            (Some(base_rule), Some(target_rule)) => {
                if base_rule.effect_token != target_rule.effect_token {
                    entries.push(diff_entry(
                        PolicyPackDiffEntryKind::EffectChanged,
                        Some(base_rule),
                        Some(target_rule),
                        "Rule effect changed between packs.",
                    ));
                }
                if base_rule.scope_label != target_rule.scope_label {
                    entries.push(diff_entry(
                        PolicyPackDiffEntryKind::ScopeChanged,
                        Some(base_rule),
                        Some(target_rule),
                        "Rule scope changed between packs.",
                    ));
                }
                if base_rule.reason_token != target_rule.reason_token {
                    entries.push(diff_entry(
                        PolicyPackDiffEntryKind::ReasonChanged,
                        Some(base_rule),
                        Some(target_rule),
                        "Rule reason token changed between packs.",
                    ));
                }
            }
            (None, None) => {}
        }
    }

    entries
}

fn diff_entry(
    kind: PolicyPackDiffEntryKind,
    base_rule: Option<&PolicyPackBetaRule>,
    target_rule: Option<&PolicyPackBetaRule>,
    note: &str,
) -> PolicyPackBetaDiffEntry {
    let rule = target_rule
        .or(base_rule)
        .expect("diff entry needs at least one rule");
    PolicyPackBetaDiffEntry {
        record_kind: POLICY_PACK_BETA_DIFF_ENTRY_RECORD_KIND.to_owned(),
        schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
        kind,
        kind_token: kind.as_str().to_owned(),
        rule_id: rule.rule_id.clone(),
        surface_family_token: rule.surface_family_token.clone(),
        before_effect_token: base_rule
            .map(|rule| rule.effect_token.clone())
            .unwrap_or_else(|| "not_present".to_owned()),
        after_effect_token: target_rule
            .map(|rule| rule.effect_token.clone())
            .unwrap_or_else(|| "not_present".to_owned()),
        before_scope_label: base_rule
            .map(|rule| rule.scope_label.clone())
            .unwrap_or_else(|| "not_present".to_owned()),
        after_scope_label: target_rule
            .map(|rule| rule.scope_label.clone())
            .unwrap_or_else(|| "not_present".to_owned()),
        before_reason_token: base_rule
            .map(|rule| rule.reason_token.clone())
            .unwrap_or_else(|| "not_present".to_owned()),
        after_reason_token: target_rule
            .map(|rule| rule.reason_token.clone())
            .unwrap_or_else(|| "not_present".to_owned()),
        note: note.to_owned(),
    }
}

fn seed_denial_traces(target: &PolicyPackBetaPack) -> Vec<PolicyPackBetaDenialTrace> {
    target
        .rules
        .iter()
        .filter(|rule| rule.effect.surfaces_denial())
        .enumerate()
        .map(|(index, rule)| PolicyPackBetaDenialTrace {
            record_kind: POLICY_PACK_BETA_DENIAL_TRACE_RECORD_KIND.to_owned(),
            schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
            trace_id: format!("policy-pack-beta:trace:{:03}", index + 1),
            pack_id: target.pack_id.clone(),
            rule_id: rule.rule_id.clone(),
            surface_family: rule.surface_family,
            surface_family_token: rule.surface_family_token.clone(),
            reason_token: rule.reason_token.clone(),
            explanation: rule.explanation.clone(),
            profile_token: PolicyPackBetaProfileClass::Connected.as_str().to_owned(),
        })
        .collect()
}

fn seed_import_receipt(pack: &PolicyPackBetaPack, receipt_id: &str) -> PolicyPackBetaImportReceipt {
    PolicyPackBetaImportReceipt {
        record_kind: POLICY_PACK_BETA_IMPORT_RECEIPT_RECORD_KIND.to_owned(),
        schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
        receipt_id: receipt_id.to_owned(),
        pack_id: pack.pack_id.clone(),
        pack_version: pack.pack_version.clone(),
        source_class: pack.source_class,
        source_token: pack.source_token.clone(),
        signature_state: pack.signature_state,
        signature_state_token: pack.signature_state_token.clone(),
        provenance: pack.provenance.clone(),
        applies_to_profiles: pack.applies_to_profiles.clone(),
        preserves_signature_blob: true,
        preserves_provenance: true,
        preserves_explanation: true,
    }
}

fn rule(
    rule_id: &str,
    surface_family: LaunchWedgeCapabilityFamily,
    effect: PolicyPackRuleEffectClass,
    scope_label: &str,
    reason_token: &str,
    explanation: &str,
) -> PolicyPackBetaRule {
    PolicyPackBetaRule {
        record_kind: POLICY_PACK_BETA_RULE_RECORD_KIND.to_owned(),
        schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
        rule_id: rule_id.to_owned(),
        surface_family,
        surface_family_token: surface_family.as_str().to_owned(),
        effect,
        effect_token: effect.as_str().to_owned(),
        applied_authority: effect.applied_authority(),
        applied_authority_token: effect.applied_authority().as_str().to_owned(),
        scope_label: scope_label.to_owned(),
        reason_token: reason_token.to_owned(),
        explanation: explanation.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn pack(
    pack_id: &str,
    pack_version: &str,
    source_class: PolicyPackSourceClass,
    signature_state: PolicyPackSignatureStateClass,
    apply_state: PolicyPackApplyStateClass,
    provenance: PolicyPackProvenance,
    rules: Vec<PolicyPackBetaRule>,
    profiles: &[PolicyPackBetaProfileClass],
) -> PolicyPackBetaPack {
    PolicyPackBetaPack {
        record_kind: POLICY_PACK_BETA_PACK_RECORD_KIND.to_owned(),
        schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
        pack_id: pack_id.to_owned(),
        pack_version: pack_version.to_owned(),
        source_class,
        source_token: source_class.as_str().to_owned(),
        signature_state,
        signature_state_token: signature_state.as_str().to_owned(),
        apply_state,
        apply_state_token: apply_state.as_str().to_owned(),
        provenance,
        rules,
        applies_to_profiles: profiles
            .iter()
            .copied()
            .map(|profile| profile.as_str().to_owned())
            .collect(),
        no_public_endpoint_fallback: true,
        raw_private_material_excluded: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_policy_pack_beta_page();
        validate_policy_pack_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        assert!(page.packs.len() >= 4);
        assert!(page
            .summary
            .profiles_present
            .contains(&"mirror_only".to_owned()));
        assert!(page
            .summary
            .profiles_present
            .contains(&"enterprise_managed".to_owned()));
        assert!(page
            .summary
            .profiles_present
            .contains(&"offline".to_owned()));
    }

    #[test]
    fn diff_records_added_and_changed_rules() {
        let page = seeded_policy_pack_beta_page();
        let connected_diff = page
            .diffs
            .iter()
            .find(|diff| diff.profile_token == "connected")
            .expect("connected diff is seeded");
        assert!(connected_diff
            .entries
            .iter()
            .any(|entry| entry.kind == PolicyPackDiffEntryKind::Added
                && entry.rule_id == "rule:ai-tool-call"));
        assert!(connected_diff.entries.iter().any(|entry| {
            entry.kind == PolicyPackDiffEntryKind::EffectChanged
                && entry.rule_id == "rule:remote-attach"
                && entry.before_effect_token == "deny"
                && entry.after_effect_token == "require_approval"
        }));
        assert!(connected_diff.entries.iter().any(|entry| {
            entry.kind == PolicyPackDiffEntryKind::ScopeChanged
                && entry.rule_id == "rule:remote-attach"
        }));
        assert!(connected_diff.entries.iter().any(|entry| {
            entry.kind == PolicyPackDiffEntryKind::ReasonChanged
                && entry.rule_id == "rule:remote-attach"
        }));
    }

    #[test]
    fn denial_traces_resolve_to_target_rules() {
        let page = seeded_policy_pack_beta_page();
        assert!(!page.denial_traces.is_empty());
        let target = page
            .packs
            .iter()
            .find(|pack| pack.pack_id.ends_with(":baseline:2026.05.0"))
            .expect("target pack");
        for trace in &page.denial_traces {
            let resolved = target
                .rules
                .iter()
                .find(|rule| rule.rule_id == trace.rule_id)
                .expect("rule resolves");
            assert_eq!(resolved.reason_token, trace.reason_token);
        }
    }

    #[test]
    fn mirror_and_manual_import_preserve_signatures_and_provenance() {
        let page = seeded_policy_pack_beta_page();
        let mirror = page
            .import_receipts
            .iter()
            .find(|receipt| receipt.source_class == PolicyPackSourceClass::SignedMirrorOrigin)
            .expect("mirror receipt");
        assert!(mirror.preserves_signature_blob);
        assert!(mirror.preserves_provenance);
        assert!(mirror.preserves_explanation);

        let manual = page
            .import_receipts
            .iter()
            .find(|receipt| receipt.source_class == PolicyPackSourceClass::ManualSignedFileImport)
            .expect("manual import receipt");
        assert!(manual.preserves_signature_blob);
        assert!(manual.preserves_provenance);
        assert!(manual.preserves_explanation);

        let airgap = page
            .import_receipts
            .iter()
            .find(|receipt| receipt.source_class == PolicyPackSourceClass::AirGappedSignedTransfer)
            .expect("airgap receipt");
        assert!(airgap.preserves_signature_blob);
        assert!(airgap.preserves_provenance);
        assert!(airgap.preserves_explanation);
    }

    #[test]
    fn validator_rejects_signature_dropped_on_import() {
        let mut page = seeded_policy_pack_beta_page();
        let receipt = page
            .import_receipts
            .iter_mut()
            .find(|receipt| receipt.source_class == PolicyPackSourceClass::SignedMirrorOrigin)
            .expect("mirror receipt");
        receipt.preserves_signature_blob = false;
        receipt.provenance.signature_blob_ref = "dropped".to_owned();
        let defects = audit_policy_pack_beta_page(
            &page.packs,
            &page.diffs,
            &page.denial_traces,
            &page.import_receipts,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == PolicyPackBetaDefectKind::MirrorOrImportSignatureBlobDropped));
    }

    #[test]
    fn validator_rejects_denial_trace_unresolvable() {
        let mut page = seeded_policy_pack_beta_page();
        page.denial_traces[0].rule_id = "rule:does-not-exist".to_owned();
        let defects = audit_policy_pack_beta_page(
            &page.packs,
            &page.diffs,
            &page.denial_traces,
            &page.import_receipts,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == PolicyPackBetaDefectKind::DenialTraceUnresolvable));
    }

    #[test]
    fn validator_rejects_public_fallback_on_pack() {
        let mut page = seeded_policy_pack_beta_page();
        page.packs[1].no_public_endpoint_fallback = false;
        let defects = audit_policy_pack_beta_page(
            &page.packs,
            &page.diffs,
            &page.denial_traces,
            &page.import_receipts,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == PolicyPackBetaDefectKind::HiddenPublicEndpointFallback));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_policy_pack_beta_page();
        let export = PolicyPackBetaSupportExport::from_page(
            "policy-pack-beta:support-export:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
