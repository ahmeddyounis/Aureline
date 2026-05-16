//! Permission-manifest beta records, the version-to-version re-consent
//! delta, and the first consuming support / partner export.
//!
//! This module is the source of truth for the typed permission-manifest
//! lane on the first beta-bearing ecosystem path. Every record here
//! serializes against
//! [`/schemas/extensions/permission_manifest.schema.json`](../../../../schemas/extensions/permission_manifest.schema.json),
//! and the closed enums in this file are kept in lock-step with the
//! schema's `$defs` blocks.
//!
//! # Why a permission-manifest record on top of the manifest baseline
//!
//! The manifest baseline already pins publisher identity, lifecycle, and
//! the declared / effective permission diff. What it does not by itself
//! answer is:
//!
//! - what capability classes (network, filesystem, process, data, ui,
//!   credential) the extension actually declares;
//! - what changed between two versions of the same extension, in a form
//!   the install / update review surface and a mirror review can read
//!   verbatim; and
//! - whether the change requires re-consent, a soft user inform, or no
//!   action.
//!
//! One [`PermissionManifestRecord`] holds the capability-class projection
//! of an extension manifest at a single version; one
//! [`PermissionManifestDeltaRecord`] holds the deterministic diff between
//! two records and pairs it with a closed
//! [`ReConsentDecisionClass`] and [`ReConsentReasonClass`] so the install /
//! update review surface, the permission inspector, the support export,
//! and the partner packet template all read one record instead of
//! inventing per-surface "permissions changed" copy.
//!
//! The first consumer is the
//! [`PermissionManifestSupportExportRecord`] projection that pins
//! [`RedactionClass::MetadataSafeDefault`] and forbids raw permission
//! bytes, raw signing-key material, raw policy bundles, raw paths, raw
//! tokens, and raw publisher-private data from the serialized form.
//!
//! # Mirror-safe vocabulary
//!
//! The same record set is emitted for primary-registry rows, private-
//! registry rows, mirrored rows, offline-bundle rows, and vendored-local
//! rows. The closed [`ManifestOriginSourceClass`] in
//! [`crate::manifest_baseline`] is the only knob; the permission
//! vocabulary itself never forks on origin. The reviewer-facing landing
//! page is
//! [`/docs/extensions/m3/permission_manifest_beta.md`](../../../../docs/extensions/m3/permission_manifest_beta.md);
//! the checked-in fixtures live under
//! [`/fixtures/extensions/m3/permission_deltas/`](../../../../fixtures/extensions/m3/permission_deltas/).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::{
    ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord, ManifestOriginSourceClass,
    PermissionScopeClass, PublisherLifecycleStateClass, PublisherTrustTierClass, RedactionClass,
};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`PermissionManifestRecord`] payloads.
pub const PERMISSION_MANIFEST_RECORD_KIND: &str = "permission_manifest_record";

/// Record-kind tag carried on serialized [`PermissionManifestDeltaRecord`] payloads.
pub const PERMISSION_MANIFEST_DELTA_RECORD_KIND: &str = "permission_manifest_delta_record";

/// Record-kind tag carried on serialized
/// [`PermissionManifestSupportExportRecord`] payloads.
pub const PERMISSION_MANIFEST_SUPPORT_EXPORT_RECORD_KIND: &str =
    "permission_manifest_support_export_record";

/// Schema version for the permission-manifest payloads.
///
/// Bumped on breaking payload changes. Additive enum members or
/// optional fields are additive-minor and require consumers to keep
/// unknown-field preservation at their boundary.
pub const PERMISSION_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Closed capability-class vocabulary.
///
/// Captures the six high-level capability domains the spec calls out:
/// network, filesystem, process, data, ui, credential. Every
/// [`PermissionScopeClass`] resolves to exactly one capability class
/// through [`capability_class_for_scope`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClassClass {
    Network,
    Filesystem,
    Process,
    Data,
    Ui,
    Credential,
}

/// Per-permission delta class describing how one
/// `(scope_class, scope_target)` pair changed between a prior and next
/// manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionDeltaClass {
    /// Same scope_class, scope_target, scope_constraint, rationale_label.
    Unchanged,
    /// Pair appears in next but not in prior; always a widening.
    ScopeAdded,
    /// Pair appears in prior but not in next; a narrowing.
    ScopeRemoved,
    /// Same pair, but the constraint was loosened or dropped.
    ScopeConstraintWidened,
    /// Same pair, but the constraint was tightened or added.
    ScopeConstraintNarrowed,
    /// Same pair, same constraint, but the rationale text was rewritten.
    RationaleOnlyChanged,
}

/// Per-capability-class delta class describing how the row count under
/// one capability class changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClassDeltaClass {
    /// No entries added or removed under this class.
    Unchanged,
    /// The class was empty in prior and has entries in next.
    CapabilityClassAdded,
    /// The class had entries in prior and is empty in next.
    CapabilityClassRemoved,
    /// Same class, but at least one entry was added (widening).
    EntriesWidenedWithinClass,
    /// Same class, but at least one entry was removed (narrowing).
    EntriesNarrowedWithinClass,
    /// Same class, entries added and removed (mixed change).
    EntriesMixedWideningAndNarrowing,
}

/// Closed re-consent decision class.
///
/// Mirrors the install / update review surface's typed action vocabulary
/// so consumers never invent a local "permissions changed" copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReConsentDecisionClass {
    /// No declared permission change between prior and next.
    NotRequiredNoChange,
    /// Permissions only narrowed; no re-consent required.
    NotRequiredNarrowingOnly,
    /// No permission change, but at least one rationale label changed.
    InformOnlyRationaleChanged,
    /// At least one widening was detected; an explicit re-consent flow
    /// MUST surface before the update is enabled.
    ReConsentRequiredWidening,
    /// The delta cites a new capability class that was not present in
    /// the prior manifest; even if every change is otherwise narrowing,
    /// the new class itself requires re-consent.
    ReConsentRequiredNewCapabilityClass,
    /// The delta is refused for a structural / lifecycle reason. The
    /// install / update review surface holds without enabling and does
    /// not run a re-consent flow.
    RefusedInconsistentInput,
}

/// Closed re-consent reason class paired with
/// [`ReConsentDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReConsentReasonClass {
    NoChangeBetweenVersions,
    NarrowingDoesNotRequireReConsent,
    RationaleChangedInformOnly,
    WideningAddedNewScope,
    WideningConstraintLoosened,
    WideningAddedNewCapabilityClass,
    RefusedExtensionIdentityMismatch,
    RefusedPriorAndNextSameVersion,
    RefusedPriorManifestRefMissing,
    RefusedNextManifestRefMissing,
    RefusedPublisherQuarantined,
    RefusedPublisherLifecycleRetired,
    RefusedExtensionLifecycleRetired,
    RefusedOriginSourceUnknown,
    RefusedRationaleMissingOnWideningEntry,
}

/// One declared permission-scope entry projected onto a capability class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityScopeEntry {
    pub capability_class_class: CapabilityClassClass,
    pub scope_class: PermissionScopeClass,
    pub scope_target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_constraint: Option<String>,
    pub rationale_label: String,
}

/// Per-capability-class summary entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityClassSummaryEntry {
    pub capability_class_class: CapabilityClassClass,
    pub declared_entry_count: u32,
}

/// One inspectable permission-manifest record per extension version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionManifestRecord {
    pub record_kind: String,
    pub permission_manifest_schema_version: u32,
    pub permission_manifest_id: String,
    pub manifest_baseline_ref: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub extension_lifecycle_state_class: ExtensionLifecycleStateClass,
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    pub origin_source_label: String,
    pub publisher_identity_ref: String,
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    pub publisher_lifecycle_state_class: PublisherLifecycleStateClass,
    pub declared_permissions: Vec<CapabilityScopeEntry>,
    pub capability_class_summary: Vec<CapabilityClassSummaryEntry>,
    pub redaction_class: RedactionClass,
}

/// One typed permission-delta row between two
/// [`PermissionManifestRecord`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionDeltaEntry {
    pub capability_class_class: CapabilityClassClass,
    pub scope_class: PermissionScopeClass,
    pub scope_target: String,
    pub delta_class: PermissionDeltaClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_constraint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_constraint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_rationale_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_rationale_label: Option<String>,
    pub delta_reason_label: String,
}

/// One typed capability-class-delta row between two
/// [`PermissionManifestRecord`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityClassDeltaEntry {
    pub capability_class_class: CapabilityClassClass,
    pub prior_entry_count: u32,
    pub next_entry_count: u32,
    pub delta_class: CapabilityClassDeltaClass,
    pub entries_added_count: u32,
    pub entries_removed_count: u32,
}

/// Inputs to evaluate one re-consent delta.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionManifestDeltaInput {
    pub delta_id: String,
    pub prior_manifest: PermissionManifestRecord,
    pub next_manifest: PermissionManifestRecord,
    pub computed_at: String,
}

/// One typed re-consent delta record between two versions of the same
/// extension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionManifestDeltaRecord {
    pub record_kind: String,
    pub permission_manifest_schema_version: u32,
    pub delta_id: String,
    pub extension_identity_ref: String,
    pub prior_manifest_ref: String,
    pub prior_extension_version: String,
    pub next_manifest_ref: String,
    pub next_extension_version: String,
    pub delta_entries: Vec<PermissionDeltaEntry>,
    pub capability_class_deltas: Vec<CapabilityClassDeltaEntry>,
    pub widening_count: u32,
    pub narrowing_count: u32,
    pub rationale_only_changed_count: u32,
    pub re_consent_decision_class: ReConsentDecisionClass,
    pub re_consent_reason_class: ReConsentReasonClass,
    pub delta_summary: String,
    pub computed_at: String,
    pub redaction_class: RedactionClass,
}

/// First consumer projection: a metadata-safe support / partner export
/// that joins a permission manifest and its optional re-consent delta.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionManifestSupportExportRecord {
    pub record_kind: String,
    pub permission_manifest_schema_version: u32,
    pub export_id: String,
    pub manifest_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta_ref: Option<String>,
    pub extension_identity_ref: String,
    pub extension_version: String,
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    pub origin_source_label: String,
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    pub declared_capability_classes: Vec<CapabilityClassClass>,
    pub declared_entry_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub re_consent_decision_class: Option<ReConsentDecisionClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub re_consent_reason_class: Option<ReConsentReasonClass>,
    pub widening_count: u32,
    pub narrowing_count: u32,
    pub rationale_only_changed_count: u32,
    pub requires_re_consent: bool,
    pub blocks_activation: bool,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by the permission-manifest validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionManifestFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl PermissionManifestFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Map a permission-scope class to its capability class.
///
/// The mapping is closed and total over [`PermissionScopeClass`]:
///
/// - `NetworkEgress` → `Network`.
/// - `FilesystemRead`, `FilesystemWrite` → `Filesystem`.
/// - `ShellExecute`, `ExecutionContextBind` → `Process`.
/// - `AiProviderAccess`, `ConnectedProviderAccess`, `WorkspaceSettingsRead`,
///   `WorkspaceSettingsWrite`, `SubscriptionSubscribe`, `CapabilityInherit`
///   → `Data`.
/// - `UiCommandContribute` → `Ui`.
/// - `SecretHandleUse` → `Credential`.
pub fn capability_class_for_scope(scope: PermissionScopeClass) -> CapabilityClassClass {
    match scope {
        PermissionScopeClass::NetworkEgress => CapabilityClassClass::Network,
        PermissionScopeClass::FilesystemRead | PermissionScopeClass::FilesystemWrite => {
            CapabilityClassClass::Filesystem
        }
        PermissionScopeClass::ShellExecute | PermissionScopeClass::ExecutionContextBind => {
            CapabilityClassClass::Process
        }
        PermissionScopeClass::AiProviderAccess
        | PermissionScopeClass::ConnectedProviderAccess
        | PermissionScopeClass::WorkspaceSettingsRead
        | PermissionScopeClass::WorkspaceSettingsWrite
        | PermissionScopeClass::SubscriptionSubscribe
        | PermissionScopeClass::CapabilityInherit => CapabilityClassClass::Data,
        PermissionScopeClass::UiCommandContribute => CapabilityClassClass::Ui,
        PermissionScopeClass::SecretHandleUse => CapabilityClassClass::Credential,
    }
}

/// Project an [`ExtensionManifestBaselineRecord`] onto a
/// [`PermissionManifestRecord`] by attaching the capability-class for
/// every declared permission and computing the per-class summary.
///
/// The projection is deterministic: declared permissions are emitted in
/// the same order as the baseline; the capability-class summary is
/// emitted in a stable order (network, filesystem, process, data, ui,
/// credential), and classes with zero entries are omitted.
pub fn project_permission_manifest(
    baseline: &ExtensionManifestBaselineRecord,
    permission_manifest_id: &str,
) -> PermissionManifestRecord {
    let declared: Vec<CapabilityScopeEntry> = baseline
        .declared_permissions
        .iter()
        .map(|entry| CapabilityScopeEntry {
            capability_class_class: capability_class_for_scope(entry.scope_class),
            scope_class: entry.scope_class,
            scope_target: entry.scope_target.clone(),
            scope_constraint: entry.scope_constraint.clone(),
            rationale_label: entry.rationale_label.clone(),
        })
        .collect();

    let summary = capability_class_summary(&declared);

    PermissionManifestRecord {
        record_kind: PERMISSION_MANIFEST_RECORD_KIND.to_string(),
        permission_manifest_schema_version: PERMISSION_MANIFEST_SCHEMA_VERSION,
        permission_manifest_id: permission_manifest_id.to_string(),
        manifest_baseline_ref: baseline.manifest_baseline_id.clone(),
        extension_identity: baseline.extension_identity.clone(),
        extension_version: baseline.extension_version.clone(),
        extension_lifecycle_state_class: baseline.extension_lifecycle_state_class,
        manifest_origin_source_class: baseline.manifest_origin_source_class,
        origin_source_label: baseline.origin_source_label.clone(),
        publisher_identity_ref: baseline.publisher_identity_ref.clone(),
        publisher_trust_tier_class: baseline.publisher_trust_tier_class,
        publisher_lifecycle_state_class: baseline.publisher_lifecycle_state_class,
        declared_permissions: declared,
        capability_class_summary: summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Compute the deterministic re-consent delta between two
/// [`PermissionManifestRecord`]s.
///
/// Decision precedence (most-restrictive first):
///
/// 1. Identity / lifecycle / publisher guardrails refuse the delta with
///    [`ReConsentDecisionClass::RefusedInconsistentInput`].
/// 2. Any widening with a missing rationale_label refuses the delta with
///    [`ReConsentReasonClass::RefusedRationaleMissingOnWideningEntry`].
/// 3. A new capability class on the next manifest → re-consent required
///    with [`ReConsentReasonClass::WideningAddedNewCapabilityClass`].
/// 4. Any scope-added widening → re-consent required with
///    [`ReConsentReasonClass::WideningAddedNewScope`].
/// 5. Any constraint-loosening widening → re-consent required with
///    [`ReConsentReasonClass::WideningConstraintLoosened`].
/// 6. Narrowing-only (no widening, no rationale-only change) → not
///    required with [`ReConsentReasonClass::NarrowingDoesNotRequireReConsent`].
/// 7. Rationale-only changes (no permission change, at least one
///    rationale touched) → inform-only with
///    [`ReConsentReasonClass::RationaleChangedInformOnly`].
/// 8. Otherwise → not required with
///    [`ReConsentReasonClass::NoChangeBetweenVersions`].
pub fn evaluate_permission_manifest_delta(
    input: PermissionManifestDeltaInput,
) -> PermissionManifestDeltaRecord {
    let PermissionManifestDeltaInput {
        delta_id,
        prior_manifest,
        next_manifest,
        computed_at,
    } = input;

    let entries = compute_permission_delta_entries(
        &prior_manifest.declared_permissions,
        &next_manifest.declared_permissions,
    );
    let class_deltas = compute_capability_class_deltas(&prior_manifest, &next_manifest, &entries);

    let widening_count = entries
        .iter()
        .filter(|e| {
            matches!(
                e.delta_class,
                PermissionDeltaClass::ScopeAdded | PermissionDeltaClass::ScopeConstraintWidened
            )
        })
        .count() as u32;
    let narrowing_count = entries
        .iter()
        .filter(|e| {
            matches!(
                e.delta_class,
                PermissionDeltaClass::ScopeRemoved | PermissionDeltaClass::ScopeConstraintNarrowed
            )
        })
        .count() as u32;
    let rationale_only_changed_count = entries
        .iter()
        .filter(|e| matches!(e.delta_class, PermissionDeltaClass::RationaleOnlyChanged))
        .count() as u32;
    let new_capability_classes: Vec<CapabilityClassClass> = class_deltas
        .iter()
        .filter(|d| d.delta_class == CapabilityClassDeltaClass::CapabilityClassAdded)
        .map(|d| d.capability_class_class)
        .collect();

    let (decision, reason, summary) = decide_re_consent(
        &prior_manifest,
        &next_manifest,
        &entries,
        widening_count,
        narrowing_count,
        rationale_only_changed_count,
        &new_capability_classes,
    );

    PermissionManifestDeltaRecord {
        record_kind: PERMISSION_MANIFEST_DELTA_RECORD_KIND.to_string(),
        permission_manifest_schema_version: PERMISSION_MANIFEST_SCHEMA_VERSION,
        delta_id,
        extension_identity_ref: next_manifest.extension_identity.clone(),
        prior_manifest_ref: prior_manifest.permission_manifest_id.clone(),
        prior_extension_version: prior_manifest.extension_version.clone(),
        next_manifest_ref: next_manifest.permission_manifest_id.clone(),
        next_extension_version: next_manifest.extension_version.clone(),
        delta_entries: entries,
        capability_class_deltas: class_deltas,
        widening_count,
        narrowing_count,
        rationale_only_changed_count,
        re_consent_decision_class: decision,
        re_consent_reason_class: reason,
        delta_summary: summary,
        computed_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a permission manifest (with an optional re-consent delta)
/// into the first consumer support / partner export.
pub fn project_permission_manifest_support_export(
    manifest: &PermissionManifestRecord,
    delta: Option<&PermissionManifestDeltaRecord>,
    export_id: &str,
) -> PermissionManifestSupportExportRecord {
    let declared_capability_classes: Vec<CapabilityClassClass> = manifest
        .capability_class_summary
        .iter()
        .map(|s| s.capability_class_class)
        .collect();
    let declared_entry_count = manifest.declared_permissions.len() as u32;

    let (
        delta_ref,
        re_consent_decision_class,
        re_consent_reason_class,
        widening_count,
        narrowing_count,
        rationale_only_changed_count,
    ) = match delta {
        Some(d) => (
            Some(d.delta_id.clone()),
            Some(d.re_consent_decision_class),
            Some(d.re_consent_reason_class),
            d.widening_count,
            d.narrowing_count,
            d.rationale_only_changed_count,
        ),
        None => (None, None, None, 0, 0, 0),
    };
    let requires_re_consent = matches!(
        re_consent_decision_class,
        Some(ReConsentDecisionClass::ReConsentRequiredWidening)
            | Some(ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass)
    );
    let blocks_activation = matches!(
        re_consent_decision_class,
        Some(ReConsentDecisionClass::ReConsentRequiredWidening)
            | Some(ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass)
            | Some(ReConsentDecisionClass::RefusedInconsistentInput)
    );

    let export_safe_summary = match delta {
        Some(d) => format!(
            "{} declared entries across {} capability classes; widening={}, narrowing={}, rationale_only={}; re-consent decision={:?}",
            declared_entry_count,
            declared_capability_classes.len(),
            widening_count,
            narrowing_count,
            rationale_only_changed_count,
            d.re_consent_decision_class,
        ),
        None => format!(
            "{} declared entries across {} capability classes; no re-consent delta attached.",
            declared_entry_count,
            declared_capability_classes.len(),
        ),
    };

    PermissionManifestSupportExportRecord {
        record_kind: PERMISSION_MANIFEST_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        permission_manifest_schema_version: PERMISSION_MANIFEST_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        manifest_ref: manifest.permission_manifest_id.clone(),
        delta_ref,
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        manifest_origin_source_class: manifest.manifest_origin_source_class,
        origin_source_label: manifest.origin_source_label.clone(),
        publisher_trust_tier_class: manifest.publisher_trust_tier_class,
        declared_capability_classes,
        declared_entry_count,
        re_consent_decision_class,
        re_consent_reason_class,
        widening_count,
        narrowing_count,
        rationale_only_changed_count,
        requires_re_consent,
        blocks_activation,
        export_safe_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate the structural invariants of a permission-manifest record.
pub fn validate_permission_manifest_record(
    record: &PermissionManifestRecord,
) -> Vec<PermissionManifestFinding> {
    let mut findings = Vec::new();

    if record.record_kind != PERMISSION_MANIFEST_RECORD_KIND {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.record_kind_wrong",
            format!(
                "record_kind must be '{PERMISSION_MANIFEST_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.permission_manifest_schema_version != PERMISSION_MANIFEST_SCHEMA_VERSION {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.schema_version_wrong",
            format!(
                "permission_manifest_schema_version must be {PERMISSION_MANIFEST_SCHEMA_VERSION}; got {}",
                record.permission_manifest_schema_version
            ),
        ));
    }
    if !record
        .permission_manifest_id
        .starts_with("permission_manifest:")
    {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.id_unprefixed",
            "permission_manifest_id must start with 'permission_manifest:'",
        ));
    }
    if !record
        .manifest_baseline_ref
        .starts_with("manifest_baseline:")
    {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.manifest_baseline_ref_unprefixed",
            "manifest_baseline_ref must start with 'manifest_baseline:'",
        ));
    }
    if !record.extension_identity.contains('/') {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.extension_identity_unnamespaced",
            "extension_identity must be of the form 'publisher_id/extension_id'",
        ));
    }
    if record.publisher_identity_ref.trim().is_empty() {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.publisher_identity_required",
            "publisher_identity_ref must be a non-empty opaque ref",
        ));
    }
    if record.origin_source_label.trim().is_empty() {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.origin_source_label_required",
            "origin_source_label must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.redaction_class_must_be_metadata_safe",
            "permission_manifest records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    for (idx, entry) in record.declared_permissions.iter().enumerate() {
        if entry.rationale_label.trim().is_empty() {
            findings.push(PermissionManifestFinding::new(
                "permission_manifest.declared_permission_rationale_required",
                format!(
                    "declared_permissions[{idx}] (scope_class={:?}, scope_target='{}') is missing a non-empty rationale_label",
                    entry.scope_class, entry.scope_target
                ),
            ));
        }
        if entry.scope_target.trim().is_empty() {
            findings.push(PermissionManifestFinding::new(
                "permission_manifest.declared_permission_scope_target_required",
                format!("declared_permissions[{idx}].scope_target must be a non-empty string"),
            ));
        }
        let expected = capability_class_for_scope(entry.scope_class);
        if entry.capability_class_class != expected {
            findings.push(PermissionManifestFinding::new(
                "permission_manifest.capability_class_mismatch",
                format!(
                    "declared_permissions[{idx}] (scope_class={:?}) must carry capability_class_class={:?}; got {:?}",
                    entry.scope_class, expected, entry.capability_class_class
                ),
            ));
        }
    }

    // Cross-check capability_class_summary against declared_permissions.
    let recomputed = capability_class_summary(&record.declared_permissions);
    if record.capability_class_summary != recomputed {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest.capability_class_summary_inconsistent",
            "capability_class_summary must equal the deterministic summary computed from declared_permissions",
        ));
    }

    findings
}

/// Validate the structural invariants of a permission-manifest delta
/// record.
pub fn validate_permission_manifest_delta_record(
    record: &PermissionManifestDeltaRecord,
) -> Vec<PermissionManifestFinding> {
    let mut findings = Vec::new();

    if record.record_kind != PERMISSION_MANIFEST_DELTA_RECORD_KIND {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.record_kind_wrong",
            format!(
                "record_kind must be '{PERMISSION_MANIFEST_DELTA_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.permission_manifest_schema_version != PERMISSION_MANIFEST_SCHEMA_VERSION {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.schema_version_wrong",
            format!(
                "permission_manifest_schema_version must be {PERMISSION_MANIFEST_SCHEMA_VERSION}; got {}",
                record.permission_manifest_schema_version
            ),
        ));
    }
    if !record.delta_id.starts_with("permission_manifest_delta:") {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.id_unprefixed",
            "delta_id must start with 'permission_manifest_delta:'",
        ));
    }
    if !record
        .prior_manifest_ref
        .starts_with("permission_manifest:")
    {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.prior_manifest_ref_unprefixed",
            "prior_manifest_ref must start with 'permission_manifest:'",
        ));
    }
    if !record.next_manifest_ref.starts_with("permission_manifest:") {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.next_manifest_ref_unprefixed",
            "next_manifest_ref must start with 'permission_manifest:'",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.redaction_class_must_be_metadata_safe",
            "permission_manifest_delta records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    let widening_seen = record
        .delta_entries
        .iter()
        .filter(|e| {
            matches!(
                e.delta_class,
                PermissionDeltaClass::ScopeAdded | PermissionDeltaClass::ScopeConstraintWidened
            )
        })
        .count() as u32;
    if widening_seen != record.widening_count {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.widening_count_inconsistent",
            "widening_count must equal the number of scope_added or scope_constraint_widened delta entries",
        ));
    }
    let narrowing_seen = record
        .delta_entries
        .iter()
        .filter(|e| {
            matches!(
                e.delta_class,
                PermissionDeltaClass::ScopeRemoved | PermissionDeltaClass::ScopeConstraintNarrowed
            )
        })
        .count() as u32;
    if narrowing_seen != record.narrowing_count {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.narrowing_count_inconsistent",
            "narrowing_count must equal the number of scope_removed or scope_constraint_narrowed delta entries",
        ));
    }
    if matches!(
        record.re_consent_decision_class,
        ReConsentDecisionClass::ReConsentRequiredWidening
            | ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass
    ) && record.widening_count == 0
        && record.re_consent_reason_class != ReConsentReasonClass::WideningAddedNewCapabilityClass
    {
        findings.push(PermissionManifestFinding::new(
            "permission_manifest_delta.reconsent_without_widening",
            "a re_consent_required decision must cite at least one widening delta or a new capability class",
        ));
    }

    findings
}

fn capability_class_summary(declared: &[CapabilityScopeEntry]) -> Vec<CapabilityClassSummaryEntry> {
    const ORDER: &[CapabilityClassClass] = &[
        CapabilityClassClass::Network,
        CapabilityClassClass::Filesystem,
        CapabilityClassClass::Process,
        CapabilityClassClass::Data,
        CapabilityClassClass::Ui,
        CapabilityClassClass::Credential,
    ];
    let mut out = Vec::new();
    for class in ORDER {
        let count = declared
            .iter()
            .filter(|e| e.capability_class_class == *class)
            .count() as u32;
        if count > 0 {
            out.push(CapabilityClassSummaryEntry {
                capability_class_class: *class,
                declared_entry_count: count,
            });
        }
    }
    out
}

fn compute_permission_delta_entries(
    prior: &[CapabilityScopeEntry],
    next: &[CapabilityScopeEntry],
) -> Vec<PermissionDeltaEntry> {
    let mut entries: Vec<PermissionDeltaEntry> = Vec::new();

    for next_entry in next {
        match prior.iter().find(|p| {
            p.scope_class == next_entry.scope_class && p.scope_target == next_entry.scope_target
        }) {
            Some(prior_entry) => {
                let constraint_delta = compare_constraints(
                    prior_entry.scope_constraint.as_deref(),
                    next_entry.scope_constraint.as_deref(),
                );
                let rationale_changed = prior_entry.rationale_label != next_entry.rationale_label;
                let (delta_class, delta_reason_label) = match constraint_delta {
                    ConstraintDelta::Unchanged => {
                        if rationale_changed {
                            (
                                PermissionDeltaClass::RationaleOnlyChanged,
                                "rationale_label rewritten; permission unchanged".to_string(),
                            )
                        } else {
                            (
                                PermissionDeltaClass::Unchanged,
                                "scope unchanged across versions".to_string(),
                            )
                        }
                    }
                    ConstraintDelta::Widened => (
                        PermissionDeltaClass::ScopeConstraintWidened,
                        "scope_constraint relaxed or dropped".to_string(),
                    ),
                    ConstraintDelta::Narrowed => (
                        PermissionDeltaClass::ScopeConstraintNarrowed,
                        "scope_constraint tightened or added".to_string(),
                    ),
                };
                entries.push(PermissionDeltaEntry {
                    capability_class_class: next_entry.capability_class_class,
                    scope_class: next_entry.scope_class,
                    scope_target: next_entry.scope_target.clone(),
                    delta_class,
                    prior_constraint: prior_entry.scope_constraint.clone(),
                    next_constraint: next_entry.scope_constraint.clone(),
                    prior_rationale_label: Some(prior_entry.rationale_label.clone()),
                    next_rationale_label: Some(next_entry.rationale_label.clone()),
                    delta_reason_label,
                });
            }
            None => {
                entries.push(PermissionDeltaEntry {
                    capability_class_class: next_entry.capability_class_class,
                    scope_class: next_entry.scope_class,
                    scope_target: next_entry.scope_target.clone(),
                    delta_class: PermissionDeltaClass::ScopeAdded,
                    prior_constraint: None,
                    next_constraint: next_entry.scope_constraint.clone(),
                    prior_rationale_label: None,
                    next_rationale_label: Some(next_entry.rationale_label.clone()),
                    delta_reason_label: "scope pair not present in prior manifest; widening"
                        .to_string(),
                });
            }
        }
    }

    for prior_entry in prior {
        let still_present = next.iter().any(|n| {
            n.scope_class == prior_entry.scope_class && n.scope_target == prior_entry.scope_target
        });
        if !still_present {
            entries.push(PermissionDeltaEntry {
                capability_class_class: prior_entry.capability_class_class,
                scope_class: prior_entry.scope_class,
                scope_target: prior_entry.scope_target.clone(),
                delta_class: PermissionDeltaClass::ScopeRemoved,
                prior_constraint: prior_entry.scope_constraint.clone(),
                next_constraint: None,
                prior_rationale_label: Some(prior_entry.rationale_label.clone()),
                next_rationale_label: None,
                delta_reason_label:
                    "scope pair present in prior manifest is no longer declared; narrowing"
                        .to_string(),
            });
        }
    }

    entries
}

fn compute_capability_class_deltas(
    prior: &PermissionManifestRecord,
    next: &PermissionManifestRecord,
    entries: &[PermissionDeltaEntry],
) -> Vec<CapabilityClassDeltaEntry> {
    const ORDER: &[CapabilityClassClass] = &[
        CapabilityClassClass::Network,
        CapabilityClassClass::Filesystem,
        CapabilityClassClass::Process,
        CapabilityClassClass::Data,
        CapabilityClassClass::Ui,
        CapabilityClassClass::Credential,
    ];
    let mut out = Vec::new();
    for class in ORDER {
        let prior_count = prior
            .declared_permissions
            .iter()
            .filter(|e| e.capability_class_class == *class)
            .count() as u32;
        let next_count = next
            .declared_permissions
            .iter()
            .filter(|e| e.capability_class_class == *class)
            .count() as u32;
        let added = entries
            .iter()
            .filter(|e| {
                e.capability_class_class == *class
                    && e.delta_class == PermissionDeltaClass::ScopeAdded
            })
            .count() as u32;
        let removed = entries
            .iter()
            .filter(|e| {
                e.capability_class_class == *class
                    && e.delta_class == PermissionDeltaClass::ScopeRemoved
            })
            .count() as u32;
        if prior_count == 0 && next_count == 0 {
            continue;
        }
        let delta_class = if prior_count == 0 && next_count > 0 {
            CapabilityClassDeltaClass::CapabilityClassAdded
        } else if prior_count > 0 && next_count == 0 {
            CapabilityClassDeltaClass::CapabilityClassRemoved
        } else if added > 0 && removed > 0 {
            CapabilityClassDeltaClass::EntriesMixedWideningAndNarrowing
        } else if added > 0 {
            CapabilityClassDeltaClass::EntriesWidenedWithinClass
        } else if removed > 0 {
            CapabilityClassDeltaClass::EntriesNarrowedWithinClass
        } else {
            CapabilityClassDeltaClass::Unchanged
        };
        out.push(CapabilityClassDeltaEntry {
            capability_class_class: *class,
            prior_entry_count: prior_count,
            next_entry_count: next_count,
            delta_class,
            entries_added_count: added,
            entries_removed_count: removed,
        });
    }
    out
}

enum ConstraintDelta {
    Unchanged,
    Widened,
    Narrowed,
}

fn compare_constraints(prior: Option<&str>, next: Option<&str>) -> ConstraintDelta {
    match (prior, next) {
        (None, None) => ConstraintDelta::Unchanged,
        (Some(a), Some(b)) if a == b => ConstraintDelta::Unchanged,
        (Some(_), None) => ConstraintDelta::Widened,
        (None, Some(_)) => ConstraintDelta::Narrowed,
        (Some(_), Some(_)) => {
            // Same scope_target, different constraint text — without a
            // formal constraint algebra the safe default is "widened",
            // because the install / update review surface MUST default to
            // requiring re-consent on an opaque constraint change.
            ConstraintDelta::Widened
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn decide_re_consent(
    prior: &PermissionManifestRecord,
    next: &PermissionManifestRecord,
    entries: &[PermissionDeltaEntry],
    widening_count: u32,
    narrowing_count: u32,
    rationale_only_changed_count: u32,
    new_capability_classes: &[CapabilityClassClass],
) -> (ReConsentDecisionClass, ReConsentReasonClass, String) {
    if prior.extension_identity != next.extension_identity {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedExtensionIdentityMismatch,
            format!(
                "Refused: prior extension_identity '{}' does not match next '{}'.",
                prior.extension_identity, next.extension_identity
            ),
        );
    }
    if !prior
        .permission_manifest_id
        .starts_with("permission_manifest:")
    {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedPriorManifestRefMissing,
            "Refused: prior permission_manifest_id is missing the 'permission_manifest:' prefix."
                .to_string(),
        );
    }
    if !next
        .permission_manifest_id
        .starts_with("permission_manifest:")
    {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedNextManifestRefMissing,
            "Refused: next permission_manifest_id is missing the 'permission_manifest:' prefix."
                .to_string(),
        );
    }
    if prior.extension_version == next.extension_version {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedPriorAndNextSameVersion,
            format!(
                "Refused: prior and next manifests share extension_version '{}'.",
                prior.extension_version
            ),
        );
    }
    if matches!(
        next.publisher_trust_tier_class,
        PublisherTrustTierClass::QuarantinedPublisher
    ) || matches!(
        next.publisher_lifecycle_state_class,
        PublisherLifecycleStateClass::Quarantined
    ) {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedPublisherQuarantined,
            "Refused: next manifest's publisher is quarantined; re-consent cannot proceed."
                .to_string(),
        );
    }
    if matches!(
        next.publisher_lifecycle_state_class,
        PublisherLifecycleStateClass::Retired
    ) {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedPublisherLifecycleRetired,
            "Refused: next manifest's publisher lifecycle is retired.".to_string(),
        );
    }
    if matches!(
        next.extension_lifecycle_state_class,
        ExtensionLifecycleStateClass::Retired | ExtensionLifecycleStateClass::Quarantined
    ) {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedExtensionLifecycleRetired,
            "Refused: next manifest's extension lifecycle is retired or quarantined.".to_string(),
        );
    }
    if matches!(
        next.manifest_origin_source_class,
        ManifestOriginSourceClass::UnknownSourceClass
    ) {
        return (
            ReConsentDecisionClass::RefusedInconsistentInput,
            ReConsentReasonClass::RefusedOriginSourceUnknown,
            "Refused: next manifest origin source could not be attributed.".to_string(),
        );
    }
    for entry in entries {
        let widening = matches!(
            entry.delta_class,
            PermissionDeltaClass::ScopeAdded | PermissionDeltaClass::ScopeConstraintWidened
        );
        if widening
            && entry
                .next_rationale_label
                .as_deref()
                .map(|s| s.trim().is_empty())
                .unwrap_or(true)
        {
            return (
                ReConsentDecisionClass::RefusedInconsistentInput,
                ReConsentReasonClass::RefusedRationaleMissingOnWideningEntry,
                format!(
                    "Refused: widening delta on (scope_class={:?}, scope_target='{}') is missing a non-empty next rationale_label.",
                    entry.scope_class, entry.scope_target,
                ),
            );
        }
    }

    if !new_capability_classes.is_empty() {
        let label = format!(
            "Re-consent required: next manifest introduces a new capability class: {:?}.",
            new_capability_classes
        );
        return (
            ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass,
            ReConsentReasonClass::WideningAddedNewCapabilityClass,
            label,
        );
    }
    if entries
        .iter()
        .any(|e| e.delta_class == PermissionDeltaClass::ScopeAdded)
    {
        return (
            ReConsentDecisionClass::ReConsentRequiredWidening,
            ReConsentReasonClass::WideningAddedNewScope,
            "Re-consent required: a previously undeclared scope pair was added.".to_string(),
        );
    }
    if entries
        .iter()
        .any(|e| e.delta_class == PermissionDeltaClass::ScopeConstraintWidened)
    {
        return (
            ReConsentDecisionClass::ReConsentRequiredWidening,
            ReConsentReasonClass::WideningConstraintLoosened,
            "Re-consent required: a scope_constraint was relaxed or dropped.".to_string(),
        );
    }
    if widening_count == 0 && narrowing_count > 0 {
        return (
            ReConsentDecisionClass::NotRequiredNarrowingOnly,
            ReConsentReasonClass::NarrowingDoesNotRequireReConsent,
            "Not required: only narrowing changes between prior and next manifest.".to_string(),
        );
    }
    if widening_count == 0 && narrowing_count == 0 && rationale_only_changed_count > 0 {
        return (
            ReConsentDecisionClass::InformOnlyRationaleChanged,
            ReConsentReasonClass::RationaleChangedInformOnly,
            "Inform-only: rationale text changed but no permission scope changed.".to_string(),
        );
    }
    (
        ReConsentDecisionClass::NotRequiredNoChange,
        ReConsentReasonClass::NoChangeBetweenVersions,
        "Not required: declared permissions are identical between prior and next.".to_string(),
    )
}
