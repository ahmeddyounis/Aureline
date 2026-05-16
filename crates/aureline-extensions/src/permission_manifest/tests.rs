//! Unit and fixture coverage for the permission-manifest beta lane.

use serde::Deserialize;

use super::{
    capability_class_for_scope, evaluate_permission_manifest_delta, project_permission_manifest,
    project_permission_manifest_support_export, validate_permission_manifest_delta_record,
    validate_permission_manifest_record, CapabilityClassClass, CapabilityClassDeltaClass,
    PermissionDeltaClass, PermissionManifestDeltaInput, ReConsentDecisionClass,
    ReConsentReasonClass, PERMISSION_MANIFEST_DELTA_RECORD_KIND, PERMISSION_MANIFEST_RECORD_KIND,
    PERMISSION_MANIFEST_SCHEMA_VERSION, PERMISSION_MANIFEST_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::{
    EffectivePermissionDiffClass, ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord,
    HostContractFamilyClass, ManifestOriginSourceClass, ManifestScopeCompletenessClass,
    PermissionScopeClass, PermissionScopeEntry, PublisherLifecycleStateClass,
    PublisherTrustTierClass, RedactionClass, EXTENSION_MANIFEST_BASELINE_RECORD_KIND,
    EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
};

#[derive(Debug, Deserialize)]
struct DeltaFixture {
    input: PermissionManifestDeltaInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: ReConsentDecisionClass,
    expected_reason_class: ReConsentReasonClass,
    expected_widening_count: u32,
    expected_narrowing_count: u32,
    expected_rationale_only_changed_count: u32,
}

fn load_fixture(name: &str) -> DeltaFixture {
    let raw = match name {
        "unchanged_no_reconsent" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/unchanged_no_reconsent.json"
        ),
        "narrowing_only" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/narrowing_only.json"
        ),
        "widening_added_scope" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/widening_added_scope.json"
        ),
        "widening_added_capability_class" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/widening_added_capability_class.json"
        ),
        "rationale_only_change" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/rationale_only_change.json"
        ),
        "mirror_origin_preserved" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/mirror_origin_preserved.json"
        ),
        "quarantined_publisher_refused" => include_str!(
            "../../../../fixtures/extensions/m3/permission_deltas/quarantined_publisher_refused.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let prior = fixture.input.prior_manifest.clone();
    let next = fixture.input.next_manifest.clone();
    let prior_findings = validate_permission_manifest_record(&prior);
    assert!(
        prior_findings.is_empty(),
        "prior manifest in {name} produced findings: {prior_findings:?}"
    );
    let next_findings = validate_permission_manifest_record(&next);
    assert!(
        next_findings.is_empty(),
        "next manifest in {name} produced findings: {next_findings:?}"
    );

    let record = evaluate_permission_manifest_delta(fixture.input);
    assert_eq!(record.record_kind, PERMISSION_MANIFEST_DELTA_RECORD_KIND);
    assert_eq!(
        record.permission_manifest_schema_version,
        PERMISSION_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(record.redaction_class, RedactionClass::MetadataSafeDefault);
    assert_eq!(
        record.re_consent_decision_class, fixture.meta.expected_decision_class,
        "decision mismatch for {name}"
    );
    assert_eq!(
        record.re_consent_reason_class, fixture.meta.expected_reason_class,
        "reason mismatch for {name}"
    );
    assert_eq!(
        record.widening_count, fixture.meta.expected_widening_count,
        "widening_count mismatch for {name}"
    );
    assert_eq!(
        record.narrowing_count, fixture.meta.expected_narrowing_count,
        "narrowing_count mismatch for {name}"
    );
    assert_eq!(
        record.rationale_only_changed_count, fixture.meta.expected_rationale_only_changed_count,
        "rationale_only_changed_count mismatch for {name}"
    );

    let findings = validate_permission_manifest_delta_record(&record);
    assert!(
        findings.is_empty(),
        "fixture {name} produced delta validation findings: {findings:?}"
    );

    let export = project_permission_manifest_support_export(
        &next,
        Some(&record),
        &format!("permission_manifest_support_export:{}", record.delta_id),
    );
    assert_eq!(
        export.record_kind,
        PERMISSION_MANIFEST_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.manifest_ref, next.permission_manifest_id);
    assert_eq!(export.delta_ref.as_deref(), Some(record.delta_id.as_str()));
    assert_eq!(
        export.re_consent_decision_class,
        Some(record.re_consent_decision_class)
    );
    assert_eq!(
        export.requires_re_consent,
        matches!(
            record.re_consent_decision_class,
            ReConsentDecisionClass::ReConsentRequiredWidening
                | ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass
        )
    );
    assert_eq!(
        export.blocks_activation,
        matches!(
            record.re_consent_decision_class,
            ReConsentDecisionClass::ReConsentRequiredWidening
                | ReConsentDecisionClass::ReConsentRequiredNewCapabilityClass
                | ReConsentDecisionClass::RefusedInconsistentInput
        )
    );
}

#[test]
fn unchanged_no_reconsent_round_trips() {
    run_fixture("unchanged_no_reconsent");
}

#[test]
fn narrowing_only_round_trips() {
    run_fixture("narrowing_only");
}

#[test]
fn widening_added_scope_round_trips() {
    run_fixture("widening_added_scope");
}

#[test]
fn widening_added_capability_class_round_trips() {
    run_fixture("widening_added_capability_class");
}

#[test]
fn rationale_only_change_round_trips() {
    run_fixture("rationale_only_change");
}

#[test]
fn mirror_origin_preserved_round_trips() {
    run_fixture("mirror_origin_preserved");
}

#[test]
fn quarantined_publisher_refused_round_trips() {
    run_fixture("quarantined_publisher_refused");
}

#[test]
fn capability_class_mapping_is_total_and_stable() {
    use PermissionScopeClass as S;
    assert_eq!(
        capability_class_for_scope(S::NetworkEgress),
        CapabilityClassClass::Network
    );
    assert_eq!(
        capability_class_for_scope(S::FilesystemRead),
        CapabilityClassClass::Filesystem
    );
    assert_eq!(
        capability_class_for_scope(S::FilesystemWrite),
        CapabilityClassClass::Filesystem
    );
    assert_eq!(
        capability_class_for_scope(S::ShellExecute),
        CapabilityClassClass::Process
    );
    assert_eq!(
        capability_class_for_scope(S::ExecutionContextBind),
        CapabilityClassClass::Process
    );
    assert_eq!(
        capability_class_for_scope(S::SecretHandleUse),
        CapabilityClassClass::Credential
    );
    assert_eq!(
        capability_class_for_scope(S::UiCommandContribute),
        CapabilityClassClass::Ui
    );
    for scope in [
        S::AiProviderAccess,
        S::ConnectedProviderAccess,
        S::WorkspaceSettingsRead,
        S::WorkspaceSettingsWrite,
        S::SubscriptionSubscribe,
        S::CapabilityInherit,
    ] {
        assert_eq!(
            capability_class_for_scope(scope),
            CapabilityClassClass::Data,
            "scope {scope:?} must map to data"
        );
    }
}

#[test]
fn project_permission_manifest_keeps_baseline_truth() {
    let baseline = baseline_manifest();
    let manifest = project_permission_manifest(
        &baseline,
        "permission_manifest:acme-labs/prose-helper:1.4.2",
    );
    assert_eq!(manifest.record_kind, PERMISSION_MANIFEST_RECORD_KIND);
    assert_eq!(
        manifest.manifest_baseline_ref,
        baseline.manifest_baseline_id
    );
    assert_eq!(manifest.declared_permissions.len(), 2);
    // Filesystem and data entries should both surface in the summary.
    assert_eq!(manifest.capability_class_summary.len(), 2);
    let findings = validate_permission_manifest_record(&manifest);
    assert!(findings.is_empty(), "valid manifest produced: {findings:?}");
}

#[test]
fn delta_refused_on_extension_identity_mismatch() {
    let mut fixture = load_fixture("widening_added_scope");
    fixture.input.next_manifest.extension_identity = "other-pub/other-ext".to_string();
    let record = evaluate_permission_manifest_delta(fixture.input);
    assert_eq!(
        record.re_consent_decision_class,
        ReConsentDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.re_consent_reason_class,
        ReConsentReasonClass::RefusedExtensionIdentityMismatch
    );
}

#[test]
fn delta_refused_on_same_version() {
    let mut fixture = load_fixture("unchanged_no_reconsent");
    fixture.input.next_manifest.extension_version =
        fixture.input.prior_manifest.extension_version.clone();
    let record = evaluate_permission_manifest_delta(fixture.input);
    assert_eq!(
        record.re_consent_decision_class,
        ReConsentDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.re_consent_reason_class,
        ReConsentReasonClass::RefusedPriorAndNextSameVersion
    );
}

#[test]
fn delta_refused_on_unknown_origin_source() {
    let mut fixture = load_fixture("widening_added_scope");
    fixture.input.next_manifest.manifest_origin_source_class =
        ManifestOriginSourceClass::UnknownSourceClass;
    let record = evaluate_permission_manifest_delta(fixture.input);
    assert_eq!(
        record.re_consent_decision_class,
        ReConsentDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.re_consent_reason_class,
        ReConsentReasonClass::RefusedOriginSourceUnknown
    );
}

#[test]
fn delta_refused_when_widening_entry_missing_rationale() {
    let mut fixture = load_fixture("widening_added_scope");
    fixture
        .input
        .next_manifest
        .declared_permissions
        .last_mut()
        .expect("widening fixture has at least one entry")
        .rationale_label = String::new();
    // The manifest itself becomes invalid; we bypass manifest validation
    // (which would catch the missing rationale at the manifest layer too)
    // and verify the delta evaluator refuses on the widening axis.
    let record = evaluate_permission_manifest_delta(fixture.input);
    assert_eq!(
        record.re_consent_decision_class,
        ReConsentDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.re_consent_reason_class,
        ReConsentReasonClass::RefusedRationaleMissingOnWideningEntry
    );
}

#[test]
fn capability_class_added_surfaces_in_class_deltas() {
    let fixture = load_fixture("widening_added_capability_class");
    let record = evaluate_permission_manifest_delta(fixture.input);
    let network = record
        .capability_class_deltas
        .iter()
        .find(|d| d.capability_class_class == CapabilityClassClass::Network)
        .expect("network class delta must be present");
    assert_eq!(
        network.delta_class,
        CapabilityClassDeltaClass::CapabilityClassAdded
    );
    assert_eq!(network.entries_added_count, 1);
    assert_eq!(network.entries_removed_count, 0);
}

#[test]
fn narrowing_only_surfaces_entries_narrowed_within_class() {
    let fixture = load_fixture("narrowing_only");
    let record = evaluate_permission_manifest_delta(fixture.input);
    let data = record
        .capability_class_deltas
        .iter()
        .find(|d| d.capability_class_class == CapabilityClassClass::Data)
        .expect("data class delta must be present (removed entries)");
    assert_eq!(
        data.delta_class,
        CapabilityClassDeltaClass::CapabilityClassRemoved
    );
    assert_eq!(data.prior_entry_count, 1);
    assert_eq!(data.next_entry_count, 0);
    assert_eq!(data.entries_removed_count, 1);
}

#[test]
fn delta_validation_flags_id_unprefixed() {
    let fixture = load_fixture("unchanged_no_reconsent");
    let mut record = evaluate_permission_manifest_delta(fixture.input);
    record.delta_id = "ad-hoc-id".to_string();
    let findings = validate_permission_manifest_delta_record(&record);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"permission_manifest_delta.id_unprefixed"));
}

#[test]
fn manifest_validation_flags_capability_class_mismatch() {
    let baseline = baseline_manifest();
    let mut manifest = project_permission_manifest(
        &baseline,
        "permission_manifest:acme-labs/prose-helper:1.4.2",
    );
    manifest
        .declared_permissions
        .first_mut()
        .expect("baseline declares at least one permission")
        .capability_class_class = CapabilityClassClass::Network;
    let findings = validate_permission_manifest_record(&manifest);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"permission_manifest.capability_class_mismatch"));
}

#[test]
fn delta_constraint_widened_surfaces_widening_decision() {
    let mut fixture = load_fixture("unchanged_no_reconsent");
    // Drop the constraint on the next manifest's filesystem_read entry.
    fixture
        .input
        .next_manifest
        .declared_permissions
        .iter_mut()
        .find(|e| e.scope_class == PermissionScopeClass::FilesystemRead)
        .expect("filesystem_read present")
        .scope_constraint = None;
    // Bump version monotonically.
    fixture.input.next_manifest.extension_version = "1.4.10".to_string();
    fixture.input.next_manifest.permission_manifest_id =
        "permission_manifest:acme-labs/prose-helper:1.4.10".to_string();

    let record = evaluate_permission_manifest_delta(fixture.input);
    assert_eq!(
        record.re_consent_decision_class,
        ReConsentDecisionClass::ReConsentRequiredWidening
    );
    assert_eq!(
        record.re_consent_reason_class,
        ReConsentReasonClass::WideningConstraintLoosened
    );
    assert!(record.delta_entries.iter().any(|e| matches!(
        e.delta_class,
        PermissionDeltaClass::ScopeConstraintWidened
    )));
}

#[test]
fn support_export_without_delta_pins_no_re_consent_state() {
    let baseline = baseline_manifest();
    let manifest = project_permission_manifest(
        &baseline,
        "permission_manifest:acme-labs/prose-helper:1.4.2",
    );
    let export = project_permission_manifest_support_export(
        &manifest,
        None,
        "permission_manifest_support_export:acme-labs/prose-helper:1.4.2",
    );
    assert!(export.delta_ref.is_none());
    assert!(!export.requires_re_consent);
    assert!(!export.blocks_activation);
    assert_eq!(export.widening_count, 0);
}

fn baseline_manifest() -> ExtensionManifestBaselineRecord {
    ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_string(),
        extension_identity: "acme-labs/prose-helper".to_string(),
        extension_version: "1.4.2".to_string(),
        extension_lifecycle_state_class: ExtensionLifecycleStateClass::Published,
        host_contract_family_class: HostContractFamilyClass::WasmComponentModel,
        manifest_origin_source_class: ManifestOriginSourceClass::PublicRegistry,
        origin_source_label: "public registry: registry.aureline.dev".to_string(),
        publisher_identity_ref: "publisher:acme-labs".to_string(),
        publisher_display_label: "Acme Labs".to_string(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        publisher_lifecycle_state_class: PublisherLifecycleStateClass::Active,
        publisher_signing_key_ref: "key:acme-labs:ed25519:2026-q2".to_string(),
        declared_permissions: vec![
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::FilesystemRead,
                scope_target: "workspace:/docs/**".to_string(),
                scope_constraint: Some("read-only under declared workspace prefix".to_string()),
                rationale_label: "Read prose documents for grammar suggestions.".to_string(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::AiProviderAccess,
                scope_target: "connected-provider:ai:acme-default".to_string(),
                scope_constraint: Some("requires user-configured provider link".to_string()),
                rationale_label: "Use the user's AI provider to refine suggestions.".to_string(),
            },
        ],
        manifest_scope_completeness_class: ManifestScopeCompletenessClass::Complete,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

// Cross-reference: keep these imports alive so trait derivations stay used
// in test contexts that don't otherwise touch them.
#[allow(dead_code)]
const _USED: EffectivePermissionDiffClass = EffectivePermissionDiffClass::Unchanged;
