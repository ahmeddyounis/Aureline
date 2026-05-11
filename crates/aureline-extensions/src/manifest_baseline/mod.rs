//! Extension manifest-baseline records, the effective-permission baseline
//! summary, and the install / review decision projection.
//!
//! This module is the M1 source of truth for the first ecosystem-bearing
//! extension lane. Every record here serializes against
//! [`/schemas/extensions/m1_extension_manifest.schema.json`](../../../../schemas/extensions/m1_extension_manifest.schema.json),
//! and the closed enums in this file are kept in lock-step with the
//! schema's `$defs` blocks. Adding a new enum member is additive-minor
//! with a [`EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION`] bump; repurposing
//! an existing member is breaking and requires a new decision row.
//!
//! ## Why one inspectable seed
//!
//! Without a single seed object every later extension surface (install /
//! review, support exports, runtime truth badges, CI / schema validation)
//! is free to invent a local "Trusted" badge, hide the declared-vs-
//! effective diff, admit an extension whose manifest scope is incomplete,
//! or silently downgrade a quarantined publisher into an unverified one.
//! The seed closes those gaps before the first marketplace-bearing lane
//! lands.
//!
//! ## Failure-drill posture
//!
//! [`compute_effective_permission_baseline`] never silently passes through
//! a permission scope not in the declared manifest set: a requested scope
//! whose `(scope_class, scope_target)` does not appear in the declared set
//! is recorded as a
//! [`EffectivePermissionDiffClass::WideningAttemptedBlocked`] entry and
//! dropped from the effective set; the resulting summary's
//! `widening_attempted_blocked_count` is non-zero, and
//! [`decide_manifest_install`] returns
//! [`InstallDecisionClass::Denied`] with reason
//! [`InstallDecisionReasonClass::EffectivePermissionWideningAttempted`].
//!
//! The fixtures
//! `/fixtures/extensions/m1_extension_manifest_baseline_rows/`
//! exercise the named failure drills end to end.

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized
/// [`ExtensionManifestBaselineRecord`] payloads.
pub const EXTENSION_MANIFEST_BASELINE_RECORD_KIND: &str = "extension_manifest_baseline_record";

/// Record-kind tag carried on serialized
/// [`EffectivePermissionBaselineRecord`] payloads.
pub const EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND: &str = "effective_permission_baseline_record";

/// Record-kind tag carried on serialized
/// [`ManifestInstallDecisionRecord`] payloads.
pub const MANIFEST_INSTALL_DECISION_RECORD_KIND: &str = "manifest_install_decision_record";

/// Schema version of the manifest-baseline payloads.
///
/// Bumped on breaking payload changes; additive-optional fields do not bump
/// this version. The frozen cross-tool boundary contract in
/// `/schemas/extensions/m1_extension_manifest.schema.json` follows the same
/// versioning rule.
pub const EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION: u32 = 1;

/// Closed publisher-trust-tier vocabulary mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublisherTrustTierClass {
    VerifiedPublisher,
    CommunityPublisher,
    OrganisationalPublisher,
    UnverifiedPublisher,
    QuarantinedPublisher,
    /// Typed terminal class for a row whose publisher cannot be attributed.
    /// Permitted only on a denial-drill row paired with
    /// [`InstallDecisionClass::Denied`] and
    /// [`InstallDecisionReasonClass::PublisherAnonymous`].
    AnonymousPublisherClass,
}

/// Closed publisher-lifecycle-state vocabulary mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublisherLifecycleStateClass {
    Active,
    Preview,
    Deprecated,
    Retired,
    Quarantined,
}

/// Closed extension-lifecycle-state vocabulary mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionLifecycleStateClass {
    Published,
    Preview,
    Deprecated,
    Retired,
    Quarantined,
}

/// Closed manifest-origin-source vocabulary mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestOriginSourceClass {
    PublicRegistry,
    PrivateRegistry,
    Mirror,
    OfflineBundle,
    VendoredLocal,
    /// Typed terminal class for a row whose origin cannot be attributed.
    /// Permitted only on a denial-drill row paired with
    /// [`InstallDecisionClass::Denied`].
    UnknownSourceClass,
}

/// Closed host-contract-family vocabulary mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostContractFamilyClass {
    WasmComponentModel,
    WasmCoreModule,
    ExternalHostProcess,
    HelperBinary,
    RemoteSideComponent,
    CompatibilityBridge,
}

/// Closed permission-scope vocabulary mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScopeClass {
    FilesystemRead,
    FilesystemWrite,
    ShellExecute,
    NetworkEgress,
    AiProviderAccess,
    ConnectedProviderAccess,
    SecretHandleUse,
    WorkspaceSettingsRead,
    WorkspaceSettingsWrite,
    ExecutionContextBind,
    SubscriptionSubscribe,
    UiCommandContribute,
    CapabilityInherit,
}

/// Permission-scope entry. Pairs a typed scope with a target and a
/// non-empty rationale label that the install / review surface MUST
/// render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionScopeEntry {
    pub scope_class: PermissionScopeClass,
    pub scope_target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_constraint: Option<String>,
    pub rationale_label: String,
}

/// Effective-permission diff class mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectivePermissionDiffClass {
    Unchanged,
    Narrowed,
    Denied,
    StepUpRequired,
    /// The row attempted to claim a permission not in the declared
    /// manifest set; effective-permission truth blocked it. Install /
    /// review MUST surface this as a denial.
    WideningAttemptedBlocked,
}

/// Declared-vs-effective diff entry. One per scope present in either the
/// declared or effective set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredVsEffectiveDiffEntry {
    pub scope_class: PermissionScopeClass,
    pub scope_target: String,
    pub diff_class: EffectivePermissionDiffClass,
    pub narrowing_reason_label: String,
}

/// Self-declared completeness class for the manifest-baseline row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestScopeCompletenessClass {
    Complete,
    IncompletePublisherMissing,
    IncompleteOriginMissing,
    IncompletePermissionRationaleMissing,
    IncompleteLifecycleUnknown,
}

/// Typed install / review decision class mirrored from the M1 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallDecisionClass {
    Admit,
    AdmitWithStepUp,
    ReviewOnly,
    Denied,
}

/// Typed install / review decision reason class mirrored from the M1
/// schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallDecisionReasonClass {
    AdmittedNoViolation,
    StepUpRequiredByPolicyPack,
    ReviewOnlyUnverifiedPublisher,
    PublisherIdentityRequired,
    PublisherAnonymous,
    PublisherQuarantined,
    PublisherLifecycleRetired,
    ExtensionLifecycleRetired,
    ManifestScopeIncomplete,
    ManifestOriginUnknown,
    DeclaredPermissionRationaleRequired,
    EffectivePermissionWideningAttempted,
    LifecycleStateUnknownClass,
}

/// Mirrors the ADR-0011 freshness-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryFreshnessClass {
    AuthoritativeLive,
    WarmCached,
    DegradedCached,
    Stale,
    Unverified,
}

/// Mirrors the ADR-0007 redaction defaults plus the metadata-safe default
/// the M1 baseline emits to support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    LogsLocal,
    TracesLocal,
    SupportBundle,
    EvidencePacket,
    AiContextCapture,
    RecipeManifest,
    ProfileExport,
    Sync,
    CrashDump,
    MutationJournalEntry,
    SaveManifest,
    ClaimManifest,
    TerminalTranscript,
    MetadataSafeDefault,
}

/// One inspectable manifest-baseline row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionManifestBaselineRecord {
    pub record_kind: String,
    pub extension_manifest_baseline_schema_version: u32,
    pub manifest_baseline_id: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub extension_lifecycle_state_class: ExtensionLifecycleStateClass,
    pub host_contract_family_class: HostContractFamilyClass,
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    pub origin_source_label: String,
    pub publisher_identity_ref: String,
    pub publisher_display_label: String,
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    pub publisher_lifecycle_state_class: PublisherLifecycleStateClass,
    pub publisher_signing_key_ref: String,
    pub declared_permissions: Vec<PermissionScopeEntry>,
    pub manifest_scope_completeness_class: ManifestScopeCompletenessClass,
    pub redaction_class: RedactionClass,
}

/// Effective-permission baseline summary computed from a manifest baseline
/// plus a list of typed policy-pack narrowings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionBaselineRecord {
    pub record_kind: String,
    pub extension_manifest_baseline_schema_version: u32,
    pub manifest_baseline_ref: String,
    pub extension_identity_ref: String,
    pub extension_version: String,
    pub effective_permissions: Vec<PermissionScopeEntry>,
    pub declared_vs_effective_diff: Vec<DeclaredVsEffectiveDiffEntry>,
    pub widening_attempted_blocked_count: u32,
    pub applied_policy_pack_refs: Vec<String>,
    pub summary_freshness_class: SummaryFreshnessClass,
    pub computed_at: String,
    pub redaction_class: RedactionClass,
}

/// Install / review decision the install surface emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestInstallDecisionRecord {
    pub record_kind: String,
    pub extension_manifest_baseline_schema_version: u32,
    pub manifest_baseline_ref: String,
    pub install_decision_class: InstallDecisionClass,
    pub install_decision_reason_class: InstallDecisionReasonClass,
    pub decision_summary: String,
    pub decided_at: String,
    pub redaction_class: RedactionClass,
}

/// One typed policy-pack narrowing the effective-permission summary
/// applies. The full ADR-0012 policy-pack contract is broader; this seed
/// captures the minimum the M1 lane needs to compute a typed diff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyPackNarrowing {
    pub policy_pack_ref: String,
    pub scope_class: PermissionScopeClass,
    pub scope_target: String,
    pub diff_class: EffectivePermissionDiffClass,
    pub narrowing_reason_label: String,
}

/// Typed validation finding emitted by [`validate_manifest_baseline_record`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidationFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl ManifestValidationFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Validate the structural invariants of a manifest-baseline record.
///
/// Returns the empty list when every invariant holds. Each finding's
/// [`ManifestValidationFinding::check_id`] is one of the typed
/// `manifest_baseline.*` ids the install / review surface, the support
/// export, and the unattended validation lane share.
pub fn validate_manifest_baseline_record(
    record: &ExtensionManifestBaselineRecord,
) -> Vec<ManifestValidationFinding> {
    let mut findings: Vec<ManifestValidationFinding> = Vec::new();

    if record.record_kind != EXTENSION_MANIFEST_BASELINE_RECORD_KIND {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_MANIFEST_BASELINE_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }

    if record.extension_manifest_baseline_schema_version
        != EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION
    {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.schema_version_wrong",
            format!(
                "extension_manifest_baseline_schema_version must be {}; got {}",
                EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
                record.extension_manifest_baseline_schema_version
            ),
        ));
    }

    if !record
        .manifest_baseline_id
        .starts_with("manifest_baseline:")
    {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.id_unprefixed",
            format!(
                "manifest_baseline_id '{}' must start with 'manifest_baseline:'",
                record.manifest_baseline_id
            ),
        ));
    }

    if !record.extension_identity.contains('/') {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.extension_identity_unnamespaced",
            format!(
                "extension_identity '{}' must be of the form 'publisher_id/extension_id'",
                record.extension_identity
            ),
        ));
    }

    if record.publisher_identity_ref.trim().is_empty() {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.publisher_identity_required",
            "publisher_identity_ref MUST be a non-empty opaque ref; anonymous or ambient publisher privilege is not acceptable".to_string(),
        ));
    }

    if record.publisher_display_label.trim().is_empty() {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.publisher_display_label_required",
            "publisher_display_label MUST be a non-empty string".to_string(),
        ));
    }

    if record.publisher_signing_key_ref.trim().is_empty() {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.publisher_signing_key_required",
            "publisher_signing_key_ref MUST be a non-empty opaque ref".to_string(),
        ));
    }

    if record.origin_source_label.trim().is_empty() {
        findings.push(ManifestValidationFinding::new(
            "manifest_baseline.origin_source_label_required",
            "origin_source_label MUST be a non-empty string".to_string(),
        ));
    }

    for (idx, perm) in record.declared_permissions.iter().enumerate() {
        if perm.rationale_label.trim().is_empty() {
            findings.push(ManifestValidationFinding::new(
                "manifest_baseline.declared_permission_rationale_required",
                format!(
                    "declared_permissions[{idx}] (scope_class={:?}, scope_target='{}') is missing a non-empty rationale_label",
                    perm.scope_class, perm.scope_target
                ),
            ));
        }
        if perm.scope_target.trim().is_empty() {
            findings.push(ManifestValidationFinding::new(
                "manifest_baseline.declared_permission_scope_target_required",
                format!("declared_permissions[{idx}].scope_target MUST be a non-empty string"),
            ));
        }
    }

    findings
}

/// Compute the effective-permission baseline summary for a manifest record.
///
/// `requested_effective` is the set of `(scope_class, scope_target)` the
/// extension is asking to actually use. Any requested scope whose pair is
/// not in the manifest's declared set is recorded as a
/// [`EffectivePermissionDiffClass::WideningAttemptedBlocked`] entry and
/// dropped from the effective-permission set; the resulting summary's
/// `widening_attempted_blocked_count` is non-zero.
///
/// `narrowings` is the typed list of policy-pack narrowings to apply. A
/// declared entry that matches a narrowing carries that narrowing's
/// `diff_class` (`narrowed`, `denied`, or `step_up_required`); a denied
/// declared entry is dropped from the effective set.
pub fn compute_effective_permission_baseline(
    manifest: &ExtensionManifestBaselineRecord,
    requested_effective: &[(PermissionScopeClass, String)],
    narrowings: &[PolicyPackNarrowing],
    computed_at: &str,
    summary_freshness_class: SummaryFreshnessClass,
) -> EffectivePermissionBaselineRecord {
    let declared_pairs: Vec<(PermissionScopeClass, String)> = manifest
        .declared_permissions
        .iter()
        .map(|p| (p.scope_class, p.scope_target.clone()))
        .collect();

    let mut diff: Vec<DeclaredVsEffectiveDiffEntry> = Vec::new();
    let mut effective: Vec<PermissionScopeEntry> = Vec::new();
    let mut widening_blocked: u32 = 0;
    let mut applied_packs: Vec<String> = Vec::new();

    // Pass 1: every declared entry contributes a diff entry. The diff
    // class comes from the matching narrowing (when one exists), else
    // `unchanged`.
    for declared in &manifest.declared_permissions {
        let matching = narrowings.iter().find(|n| {
            n.scope_class == declared.scope_class && n.scope_target == declared.scope_target
        });
        let (diff_class, narrowing_reason_label) = match matching {
            Some(n) => {
                if !applied_packs.contains(&n.policy_pack_ref) {
                    applied_packs.push(n.policy_pack_ref.clone());
                }
                (n.diff_class, n.narrowing_reason_label.clone())
            }
            None => (
                EffectivePermissionDiffClass::Unchanged,
                "unchanged".to_string(),
            ),
        };

        diff.push(DeclaredVsEffectiveDiffEntry {
            scope_class: declared.scope_class,
            scope_target: declared.scope_target.clone(),
            diff_class,
            narrowing_reason_label,
        });

        // Effective permissions only include entries that were not denied
        // (and only when they were also actually requested).
        let requested = requested_effective
            .iter()
            .any(|(c, t)| *c == declared.scope_class && t == &declared.scope_target);
        if requested && diff_class != EffectivePermissionDiffClass::Denied {
            let scope_constraint = match diff_class {
                EffectivePermissionDiffClass::Unchanged => declared.scope_constraint.clone(),
                EffectivePermissionDiffClass::Narrowed
                | EffectivePermissionDiffClass::StepUpRequired => Some(format!(
                    "policy-pack narrowing applied: {}",
                    diff.last().expect("just pushed").narrowing_reason_label
                )),
                EffectivePermissionDiffClass::Denied => None,
                EffectivePermissionDiffClass::WideningAttemptedBlocked => None,
            };
            effective.push(PermissionScopeEntry {
                scope_class: declared.scope_class,
                scope_target: declared.scope_target.clone(),
                scope_constraint,
                rationale_label: declared.rationale_label.clone(),
            });
        }
    }

    // Pass 2: every requested entry whose declared pair is missing is
    // recorded as a widening-attempted-blocked diff and never appears in
    // effective.
    for (scope_class, scope_target) in requested_effective {
        let in_declared = declared_pairs
            .iter()
            .any(|(c, t)| c == scope_class && t == scope_target);
        if !in_declared {
            widening_blocked += 1;
            diff.push(DeclaredVsEffectiveDiffEntry {
                scope_class: *scope_class,
                scope_target: scope_target.clone(),
                diff_class: EffectivePermissionDiffClass::WideningAttemptedBlocked,
                narrowing_reason_label:
                    "declared scope did not include this scope; widening blocked".to_string(),
            });
        }
    }

    EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: effective,
        declared_vs_effective_diff: diff,
        widening_attempted_blocked_count: widening_blocked,
        applied_policy_pack_refs: applied_packs,
        summary_freshness_class,
        computed_at: computed_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Decide whether to admit, admit-with-step-up, review-only, or deny the
/// install based on the manifest-baseline record and the effective
/// permission summary. The decision is deterministic and is paired with a
/// typed [`InstallDecisionReasonClass`].
///
/// Precedence (most-restrictive first):
/// 1. anonymous publisher → denied (publisher_anonymous)
/// 2. quarantined publisher → denied (publisher_quarantined)
/// 3. retired publisher → denied (publisher_lifecycle_retired)
/// 4. retired extension → denied (extension_lifecycle_retired)
/// 5. unknown manifest origin → denied (manifest_origin_unknown)
/// 6. effective permission widening attempted → denied (effective_permission_widening_attempted)
/// 7. manifest scope incomplete (any class except `complete`) → denied
///    (manifest_scope_incomplete or the more specific reason class)
/// 8. policy-pack step-up required (any diff entry with class step_up_required) →
///    admit_with_step_up (step_up_required_by_policy_pack)
/// 9. unverified publisher → review_only (review_only_unverified_publisher)
/// 10. otherwise → admit (admitted_no_violation).
pub fn decide_manifest_install(
    manifest: &ExtensionManifestBaselineRecord,
    summary: &EffectivePermissionBaselineRecord,
    decided_at: &str,
) -> ManifestInstallDecisionRecord {
    let (decision, reason, summary_text) = if matches!(
        manifest.publisher_trust_tier_class,
        PublisherTrustTierClass::AnonymousPublisherClass
    ) {
        (
            InstallDecisionClass::Denied,
            InstallDecisionReasonClass::PublisherAnonymous,
            "Denied: publisher identity is anonymous; manifest must attribute a publisher.",
        )
    } else if matches!(
        manifest.publisher_trust_tier_class,
        PublisherTrustTierClass::QuarantinedPublisher
    ) || matches!(
        manifest.publisher_lifecycle_state_class,
        PublisherLifecycleStateClass::Quarantined
    ) {
        (
            InstallDecisionClass::Denied,
            InstallDecisionReasonClass::PublisherQuarantined,
            "Denied: publisher quarantined; install / review must refuse.",
        )
    } else if matches!(
        manifest.publisher_lifecycle_state_class,
        PublisherLifecycleStateClass::Retired
    ) {
        (
            InstallDecisionClass::Denied,
            InstallDecisionReasonClass::PublisherLifecycleRetired,
            "Denied: publisher lifecycle is retired; install / review must refuse.",
        )
    } else if matches!(
        manifest.extension_lifecycle_state_class,
        ExtensionLifecycleStateClass::Retired | ExtensionLifecycleStateClass::Quarantined
    ) {
        (
            InstallDecisionClass::Denied,
            InstallDecisionReasonClass::ExtensionLifecycleRetired,
            "Denied: extension lifecycle is retired or quarantined.",
        )
    } else if matches!(
        manifest.manifest_origin_source_class,
        ManifestOriginSourceClass::UnknownSourceClass
    ) {
        (
            InstallDecisionClass::Denied,
            InstallDecisionReasonClass::ManifestOriginUnknown,
            "Denied: manifest origin source could not be attributed.",
        )
    } else if summary.widening_attempted_blocked_count > 0 {
        (
            InstallDecisionClass::Denied,
            InstallDecisionReasonClass::EffectivePermissionWideningAttempted,
            "Denied: requested permissions were not declared in the manifest scope; widening blocked.",
        )
    } else if !matches!(
        manifest.manifest_scope_completeness_class,
        ManifestScopeCompletenessClass::Complete
    ) {
        let (more_specific_reason, summary_text) = match manifest.manifest_scope_completeness_class
        {
            ManifestScopeCompletenessClass::IncompletePublisherMissing => (
                InstallDecisionReasonClass::PublisherIdentityRequired,
                "Denied: manifest is missing a publisher identity.",
            ),
            ManifestScopeCompletenessClass::IncompleteOriginMissing => (
                InstallDecisionReasonClass::ManifestOriginUnknown,
                "Denied: manifest is missing an origin / source.",
            ),
            ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing => (
                InstallDecisionReasonClass::DeclaredPermissionRationaleRequired,
                "Denied: a declared permission is missing its rationale_label.",
            ),
            ManifestScopeCompletenessClass::IncompleteLifecycleUnknown => (
                InstallDecisionReasonClass::LifecycleStateUnknownClass,
                "Denied: manifest carries an unknown lifecycle-state class.",
            ),
            ManifestScopeCompletenessClass::Complete => (
                InstallDecisionReasonClass::ManifestScopeIncomplete,
                "Denied: manifest scope incomplete.",
            ),
        };
        (
            InstallDecisionClass::Denied,
            more_specific_reason,
            summary_text,
        )
    } else if summary
        .declared_vs_effective_diff
        .iter()
        .any(|d| matches!(d.diff_class, EffectivePermissionDiffClass::StepUpRequired))
    {
        (
            InstallDecisionClass::AdmitWithStepUp,
            InstallDecisionReasonClass::StepUpRequiredByPolicyPack,
            "Admit with step-up: policy pack requires a typed step-up before some scopes are usable.",
        )
    } else if matches!(
        manifest.publisher_trust_tier_class,
        PublisherTrustTierClass::UnverifiedPublisher
    ) {
        (
            InstallDecisionClass::ReviewOnly,
            InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher,
            "Review-only: unverified publisher; manifest landed for review without enabling.",
        )
    } else {
        (
            InstallDecisionClass::Admit,
            InstallDecisionReasonClass::AdmittedNoViolation,
            "Admitted: complete manifest, attributed publisher, no widening attempt.",
        )
    };

    ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        install_decision_class: decision,
        install_decision_reason_class: reason,
        decision_summary: summary_text.to_string(),
        decided_at: decided_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}
