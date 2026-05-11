//! Unit coverage for the manifest-baseline validator, the
//! effective-permission baseline computation, and the install / review
//! decision projection.
//!
//! The fixture-driven coverage on the same row matrix lives in the
//! unattended Python validation lane under
//! `tests/extensions/m1_extension_manifest_baseline_lane/`.

use super::*;

fn verified_publisher_manifest() -> ExtensionManifestBaselineRecord {
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

#[test]
fn complete_verified_publisher_manifest_passes_all_invariants() {
    let manifest = verified_publisher_manifest();
    let findings = validate_manifest_baseline_record(&manifest);
    assert!(
        findings.is_empty(),
        "complete manifest must pass all invariants; got {findings:?}"
    );
}

#[test]
fn missing_publisher_identity_fires_publisher_identity_required() {
    let mut manifest = verified_publisher_manifest();
    manifest.publisher_identity_ref = String::new();
    manifest.manifest_scope_completeness_class =
        ManifestScopeCompletenessClass::IncompletePublisherMissing;
    let findings = validate_manifest_baseline_record(&manifest);
    assert!(
        findings
            .iter()
            .any(|f| f.check_id == "manifest_baseline.publisher_identity_required"),
        "expected manifest_baseline.publisher_identity_required; got {findings:?}"
    );
}

#[test]
fn missing_rationale_label_fires_declared_permission_rationale_required() {
    let mut manifest = verified_publisher_manifest();
    manifest.declared_permissions[0].rationale_label = String::new();
    manifest.manifest_scope_completeness_class =
        ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing;
    let findings = validate_manifest_baseline_record(&manifest);
    assert!(
        findings
            .iter()
            .any(|f| f.check_id == "manifest_baseline.declared_permission_rationale_required"),
        "expected manifest_baseline.declared_permission_rationale_required; got {findings:?}"
    );
}

#[test]
fn extension_identity_without_namespace_fires_unnamespaced() {
    let mut manifest = verified_publisher_manifest();
    manifest.extension_identity = "prose-helper".to_string();
    let findings = validate_manifest_baseline_record(&manifest);
    assert!(
        findings
            .iter()
            .any(|f| f.check_id == "manifest_baseline.extension_identity_unnamespaced"),
        "expected manifest_baseline.extension_identity_unnamespaced; got {findings:?}"
    );
}

#[test]
fn requested_scope_outside_declared_set_is_widening_attempted_blocked() {
    let manifest = verified_publisher_manifest();
    let requested = vec![
        (
            PermissionScopeClass::FilesystemRead,
            "workspace:/docs/**".to_string(),
        ),
        (
            PermissionScopeClass::FilesystemWrite,
            "workspace:/secrets/**".to_string(),
        ),
    ];
    let summary = compute_effective_permission_baseline(
        &manifest,
        &requested,
        &[],
        "2026-05-11T08:00:00Z",
        SummaryFreshnessClass::AuthoritativeLive,
    );
    assert_eq!(summary.widening_attempted_blocked_count, 1);
    let widening = summary
        .declared_vs_effective_diff
        .iter()
        .find(|d| {
            matches!(
                d.diff_class,
                EffectivePermissionDiffClass::WideningAttemptedBlocked
            )
        })
        .expect("a widening_attempted_blocked diff entry must be present");
    assert_eq!(widening.scope_class, PermissionScopeClass::FilesystemWrite);
    assert_eq!(widening.scope_target, "workspace:/secrets/**");
    assert!(
        !summary.effective_permissions.iter().any(|p| matches!(
            p.scope_class,
            PermissionScopeClass::FilesystemWrite
        )),
        "filesystem_write must not appear in effective_permissions when its declared pair is missing"
    );
}

#[test]
fn step_up_narrowing_routes_install_to_admit_with_step_up() {
    let manifest = verified_publisher_manifest();
    let requested = vec![
        (
            PermissionScopeClass::FilesystemRead,
            "workspace:/docs/**".to_string(),
        ),
        (
            PermissionScopeClass::AiProviderAccess,
            "connected-provider:ai:acme-default".to_string(),
        ),
    ];
    let narrowings = vec![PolicyPackNarrowing {
        policy_pack_ref: "policy:acme-org-default-v3".to_string(),
        scope_class: PermissionScopeClass::AiProviderAccess,
        scope_target: "connected-provider:ai:acme-default".to_string(),
        diff_class: EffectivePermissionDiffClass::StepUpRequired,
        narrowing_reason_label: "policy floor: AI provider access requires step-up".to_string(),
    }];
    let summary = compute_effective_permission_baseline(
        &manifest,
        &requested,
        &narrowings,
        "2026-05-11T08:00:00Z",
        SummaryFreshnessClass::AuthoritativeLive,
    );
    assert_eq!(summary.widening_attempted_blocked_count, 0);
    assert!(summary
        .applied_policy_pack_refs
        .contains(&"policy:acme-org-default-v3".to_string()));

    let decision = decide_manifest_install(&manifest, &summary, "2026-05-11T08:00:01Z");
    assert_eq!(
        decision.install_decision_class,
        InstallDecisionClass::AdmitWithStepUp
    );
    assert_eq!(
        decision.install_decision_reason_class,
        InstallDecisionReasonClass::StepUpRequiredByPolicyPack
    );
}

#[test]
fn anonymous_publisher_install_decision_is_denied_with_typed_reason() {
    let mut manifest = verified_publisher_manifest();
    manifest.publisher_trust_tier_class = PublisherTrustTierClass::AnonymousPublisherClass;
    manifest.publisher_identity_ref = "publisher:anonymous_repair_only".to_string();
    let summary = compute_effective_permission_baseline(
        &manifest,
        &[],
        &[],
        "2026-05-11T08:00:00Z",
        SummaryFreshnessClass::AuthoritativeLive,
    );
    let decision = decide_manifest_install(&manifest, &summary, "2026-05-11T08:00:01Z");
    assert_eq!(
        decision.install_decision_class,
        InstallDecisionClass::Denied
    );
    assert_eq!(
        decision.install_decision_reason_class,
        InstallDecisionReasonClass::PublisherAnonymous
    );
}

#[test]
fn quarantined_publisher_install_decision_is_denied_with_typed_reason() {
    let mut manifest = verified_publisher_manifest();
    manifest.publisher_trust_tier_class = PublisherTrustTierClass::QuarantinedPublisher;
    let summary = compute_effective_permission_baseline(
        &manifest,
        &[],
        &[],
        "2026-05-11T08:00:00Z",
        SummaryFreshnessClass::AuthoritativeLive,
    );
    let decision = decide_manifest_install(&manifest, &summary, "2026-05-11T08:00:01Z");
    assert_eq!(
        decision.install_decision_class,
        InstallDecisionClass::Denied
    );
    assert_eq!(
        decision.install_decision_reason_class,
        InstallDecisionReasonClass::PublisherQuarantined
    );
}

#[test]
fn unverified_publisher_routes_install_to_review_only() {
    let mut manifest = verified_publisher_manifest();
    manifest.publisher_trust_tier_class = PublisherTrustTierClass::UnverifiedPublisher;
    let requested = vec![(
        PermissionScopeClass::FilesystemRead,
        "workspace:/docs/**".to_string(),
    )];
    let summary = compute_effective_permission_baseline(
        &manifest,
        &requested,
        &[],
        "2026-05-11T08:00:00Z",
        SummaryFreshnessClass::AuthoritativeLive,
    );
    let decision = decide_manifest_install(&manifest, &summary, "2026-05-11T08:00:01Z");
    assert_eq!(
        decision.install_decision_class,
        InstallDecisionClass::ReviewOnly
    );
    assert_eq!(
        decision.install_decision_reason_class,
        InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher
    );
}

#[test]
fn widening_in_effective_set_routes_install_to_denied() {
    let manifest = verified_publisher_manifest();
    let requested = vec![(
        PermissionScopeClass::FilesystemWrite,
        "workspace:/secrets/**".to_string(),
    )];
    let summary = compute_effective_permission_baseline(
        &manifest,
        &requested,
        &[],
        "2026-05-11T08:00:00Z",
        SummaryFreshnessClass::AuthoritativeLive,
    );
    let decision = decide_manifest_install(&manifest, &summary, "2026-05-11T08:00:01Z");
    assert_eq!(
        decision.install_decision_class,
        InstallDecisionClass::Denied
    );
    assert_eq!(
        decision.install_decision_reason_class,
        InstallDecisionReasonClass::EffectivePermissionWideningAttempted
    );
}

#[test]
fn manifest_record_round_trips_through_serde_json() {
    let manifest = verified_publisher_manifest();
    let json = serde_json::to_string(&manifest).expect("serialize manifest record");
    let round_tripped: ExtensionManifestBaselineRecord =
        serde_json::from_str(&json).expect("deserialize manifest record");
    assert_eq!(manifest, round_tripped);
}
