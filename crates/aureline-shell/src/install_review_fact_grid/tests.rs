//! Unit and fixture tests for the bounded install-review fact grid
//! wedge.
//!
//! These tests cover:
//!
//! - the protected walk on a verified-publisher complete-manifest row
//!   (clean admit, every fact present, no invariant violations);
//! - the protected walk on an unverified-publisher row (review-only
//!   decision, `Limited` degraded chip);
//! - the named failure drill (a buggy caller proposes admitting a row
//!   that has incomplete facts — stripped publisher identity, stripped
//!   permission rationale, unknown origin — the wedge refuses to render
//!   the row as clean and instead surfaces the typed missing-facts
//!   invariants);
//! - adjacent drills covering admitting a widening-attempted row,
//!   admitting a row with `not_yet_admitted_no_rollback_needed` rollback
//!   posture, and pairing an `EagerWithinWorkspaceOnly` activation
//!   budget with a denied install decision;
//! - serde round-trip and deterministic plaintext rendering.

use std::path::Path;

use serde::Deserialize;

use aureline_extensions::manifest_baseline::{
    DeclaredVsEffectiveDiffEntry, EffectivePermissionBaselineRecord, EffectivePermissionDiffClass,
    ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord, HostContractFamilyClass,
    InstallDecisionClass, InstallDecisionReasonClass, ManifestInstallDecisionRecord,
    ManifestOriginSourceClass, ManifestScopeCompletenessClass, PermissionScopeClass,
    PermissionScopeEntry, PublisherLifecycleStateClass, PublisherTrustTierClass, RedactionClass,
    SummaryFreshnessClass, EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND,
    EXTENSION_MANIFEST_BASELINE_RECORD_KIND, EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
    MANIFEST_INSTALL_DECISION_RECORD_KIND,
};

use super::*;

fn verified_admit_manifest() -> ExtensionManifestBaselineRecord {
    ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_owned(),
        extension_identity: "acme-labs/prose-helper".to_owned(),
        extension_version: "1.4.2".to_owned(),
        extension_lifecycle_state_class: ExtensionLifecycleStateClass::Published,
        host_contract_family_class: HostContractFamilyClass::WasmComponentModel,
        manifest_origin_source_class: ManifestOriginSourceClass::PublicRegistry,
        origin_source_label: "public registry: registry.aureline.dev".to_owned(),
        publisher_identity_ref: "publisher:acme-labs".to_owned(),
        publisher_display_label: "Acme Labs".to_owned(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        publisher_lifecycle_state_class: PublisherLifecycleStateClass::Active,
        publisher_signing_key_ref: "key:acme-labs:ed25519:2026-q2".to_owned(),
        declared_permissions: vec![
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::FilesystemRead,
                scope_target: "workspace:/docs/**".to_owned(),
                scope_constraint: Some("read-only under declared workspace prefix".to_owned()),
                rationale_label: "Read prose documents for grammar suggestions.".to_owned(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::AiProviderAccess,
                scope_target: "connected-provider:ai:acme-default".to_owned(),
                scope_constraint: Some("requires user-configured provider link".to_owned()),
                rationale_label: "Use the user's AI provider to refine suggestions.".to_owned(),
            },
        ],
        manifest_scope_completeness_class: ManifestScopeCompletenessClass::Complete,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_effective() -> EffectivePermissionBaselineRecord {
    let manifest = verified_admit_manifest();
    EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: manifest
            .declared_permissions
            .iter()
            .map(|p| DeclaredVsEffectiveDiffEntry {
                scope_class: p.scope_class,
                scope_target: p.scope_target.clone(),
                diff_class: EffectivePermissionDiffClass::Unchanged,
                narrowing_reason_label: "unchanged".to_owned(),
            })
            .collect(),
        widening_attempted_blocked_count: 0,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_decision() -> ManifestInstallDecisionRecord {
    ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_owned(),
        install_decision_class: InstallDecisionClass::Admit,
        install_decision_reason_class: InstallDecisionReasonClass::AdmittedNoViolation,
        decision_summary: "Admitted: complete manifest, attributed publisher, no widening attempt."
            .to_owned(),
        decided_at: "2026-05-11T08:00:01Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_wedge() -> InstallReviewFactGridWedge {
    InstallReviewFactGridWedge::new(
        verified_admit_manifest(),
        verified_admit_effective(),
        verified_admit_decision(),
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    )
}

fn unverified_review_only_wedge() -> InstallReviewFactGridWedge {
    let mut manifest = verified_admit_manifest();
    manifest.manifest_baseline_id = "manifest_baseline:indie-author/quick-notes:0.2.0".to_owned();
    manifest.extension_identity = "indie-author/quick-notes".to_owned();
    manifest.extension_version = "0.2.0".to_owned();
    manifest.publisher_identity_ref = "publisher:indie-author".to_owned();
    manifest.publisher_display_label = "Indie Author".to_owned();
    manifest.publisher_trust_tier_class = PublisherTrustTierClass::UnverifiedPublisher;
    manifest.publisher_signing_key_ref = "key:indie-author:ed25519:2026-q1".to_owned();
    manifest.host_contract_family_class = HostContractFamilyClass::WasmCoreModule;
    manifest.declared_permissions = vec![PermissionScopeEntry {
        scope_class: PermissionScopeClass::UiCommandContribute,
        scope_target: "command:quick_notes.open".to_owned(),
        scope_constraint: Some("contributes one command id".to_owned()),
        rationale_label: "Contribute a command for opening the user's quick notes.".to_owned(),
    }];

    let effective = EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: vec![DeclaredVsEffectiveDiffEntry {
            scope_class: PermissionScopeClass::UiCommandContribute,
            scope_target: "command:quick_notes.open".to_owned(),
            diff_class: EffectivePermissionDiffClass::Unchanged,
            narrowing_reason_label: "unchanged".to_owned(),
        }],
        widening_attempted_blocked_count: 0,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let decision = ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        install_decision_class: InstallDecisionClass::ReviewOnly,
        install_decision_reason_class: InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher,
        decision_summary:
            "Review-only: unverified publisher; manifest landed for review without enabling."
                .to_owned(),
        decided_at: "2026-05-11T08:00:01Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::UninstallWithUserStateRetained,
    )
    .with_degraded(DegradedStateToken::Limited)
}

#[test]
fn protected_walk_verified_publisher_renders_clean_admit_card() {
    let card = verified_admit_wedge().card();
    assert_eq!(card.record_kind, INSTALL_REVIEW_FACT_GRID_RECORD_KIND);
    assert_eq!(card.schema_version, INSTALL_REVIEW_FACT_GRID_SCHEMA_VERSION);
    assert_eq!(
        card.prototype_label_token,
        "m1_prototype_install_review_fact_grid"
    );
    assert_eq!(card.extension_identity, "acme-labs/prose-helper");
    assert_eq!(
        card.publisher.publisher_trust_tier_token,
        "verified_publisher"
    );
    assert_eq!(card.origin.manifest_origin_source_token, "public_registry");
    assert_eq!(
        card.lifecycle.host_contract_family_token,
        "wasm_component_model"
    );
    assert_eq!(card.declared_permissions.len(), 2);
    assert_eq!(card.effective_permission_diff.len(), 2);
    assert_eq!(card.widening_attempted_blocked_count, 0);
    assert_eq!(card.activation_budget_token, "lazy_on_demand_only");
    assert_eq!(
        card.rollback_posture_token,
        "clean_uninstall_and_state_purge"
    );
    assert_eq!(card.decision.install_decision_class_token, "admit");
    assert_eq!(
        card.decision.install_decision_reason_class_token,
        "admitted_no_violation"
    );
    assert!(
        card.invariants.is_empty(),
        "invariants: {:?}",
        card.invariants
    );
    assert!(!card.has_invariant_violations);
    assert!(card.is_clean_admit());

    let tokens: Vec<&str> = card
        .claim_limits
        .iter()
        .map(|row| row.token.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "single_bounded_wedge_only",
            "no_marketplace_breadth",
            "no_publisher_services",
            "no_compatibility_policy_automation",
        ]
    );
}

#[test]
fn protected_walk_unverified_publisher_renders_review_only_with_limited_chip() {
    let card = unverified_review_only_wedge().card();
    assert_eq!(
        card.publisher.publisher_trust_tier_token,
        "unverified_publisher"
    );
    assert_eq!(card.decision.install_decision_class_token, "review_only");
    assert_eq!(
        card.decision.install_decision_reason_class_token,
        "review_only_unverified_publisher"
    );
    assert_eq!(card.degraded_token.as_deref(), Some("Limited"));
    assert_eq!(
        card.rollback_posture_token,
        "uninstall_with_user_state_retained"
    );
    // No invariant violations; review-only is an honest decision.
    assert!(!card.has_invariant_violations);
    assert!(!card.is_clean_admit());
}

#[test]
fn failure_drill_incomplete_facts_refuses_to_render_clean_card() {
    // Named failure drill: a buggy install / review surface tries to
    // mark an extension as admit while stripping publisher identity,
    // origin source, and one declared-permission rationale_label. The
    // wedge MUST surface every typed missing-facts invariant rather
    // than hiding any axis under a generic warning.
    let mut manifest = verified_admit_manifest();
    manifest.publisher_identity_ref = String::new();
    manifest.manifest_origin_source_class = ManifestOriginSourceClass::UnknownSourceClass;
    manifest.origin_source_label = "(unknown)".to_owned();
    // Strip the rationale_label from the first declared permission.
    if let Some(row) = manifest.declared_permissions.get_mut(0) {
        row.rationale_label = String::new();
    }
    manifest.manifest_scope_completeness_class =
        ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing;

    let mut effective = verified_admit_effective();
    effective.manifest_baseline_ref = manifest.manifest_baseline_id.clone();

    // Even with the buggy attempt to admit, the install / review
    // surface SHOULD be flagging this as denied; the named drill
    // explicitly forces "admit" so we can verify the wedge refuses to
    // render it as clean.
    let mut decision = verified_admit_decision();
    decision.manifest_baseline_ref = manifest.manifest_baseline_id.clone();
    decision.install_decision_class = InstallDecisionClass::Admit;
    decision.install_decision_reason_class = InstallDecisionReasonClass::AdmittedNoViolation;
    decision.decision_summary = "Buggy admit attempt against incomplete facts.".to_owned();

    let wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    let violation_tokens: Vec<&str> = card
        .invariants
        .iter()
        .map(|row| row.violation_token.as_str())
        .collect();
    for expected in [
        "publisher_identity_missing",
        "origin_source_missing",
        "declared_permission_rationale_missing",
        "manifest_validation_findings_present",
    ] {
        assert!(
            violation_tokens.contains(&expected),
            "expected invariant {expected} to fire on failure drill card; got {violation_tokens:?}"
        );
    }
    // Summary line must call out the block so chrome can refuse to
    // commit before the user clicks Install.
    assert!(
        card.summary_line.contains("INVARIANTS BLOCKED"),
        "summary line should call out the block: {:?}",
        card.summary_line,
    );
}

#[test]
fn widening_attempt_with_admit_decision_surfaces_typed_invariant() {
    let manifest = verified_admit_manifest();
    let mut effective = verified_admit_effective();
    effective.widening_attempted_blocked_count = 1;
    effective
        .declared_vs_effective_diff
        .push(DeclaredVsEffectiveDiffEntry {
            scope_class: PermissionScopeClass::NetworkEgress,
            scope_target: "host:exfil.example".to_owned(),
            diff_class: EffectivePermissionDiffClass::WideningAttemptedBlocked,
            narrowing_reason_label: "declared scope did not include this scope; widening blocked"
                .to_owned(),
        });
    let wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        verified_admit_decision(),
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card
        .invariants
        .iter()
        .any(|row| row.violation_token == "widening_attempted_without_denied_decision"));
}

#[test]
fn widening_attempt_with_denied_decision_does_not_surface_widening_invariant() {
    let manifest = verified_admit_manifest();
    let mut effective = verified_admit_effective();
    effective.widening_attempted_blocked_count = 1;
    effective
        .declared_vs_effective_diff
        .push(DeclaredVsEffectiveDiffEntry {
            scope_class: PermissionScopeClass::NetworkEgress,
            scope_target: "host:exfil.example".to_owned(),
            diff_class: EffectivePermissionDiffClass::WideningAttemptedBlocked,
            narrowing_reason_label: "widening blocked".to_owned(),
        });
    let mut decision = verified_admit_decision();
    decision.install_decision_class = InstallDecisionClass::Denied;
    decision.install_decision_reason_class =
        InstallDecisionReasonClass::EffectivePermissionWideningAttempted;
    decision.decision_summary =
        "Denied: requested permissions were not declared in the manifest scope.".to_owned();
    let wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        ActivationBudgetClass::NotApplicableInstallDenied,
        RollbackPostureClass::NotYetAdmittedNoRollbackNeeded,
    );
    let card = wedge.card();
    assert!(!card
        .invariants
        .iter()
        .any(|row| row.violation_token == "widening_attempted_without_denied_decision"));
}

#[test]
fn admit_with_not_yet_admitted_rollback_posture_surfaces_typed_invariant() {
    let wedge = InstallReviewFactGridWedge::new(
        verified_admit_manifest(),
        verified_admit_effective(),
        verified_admit_decision(),
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::NotYetAdmittedNoRollbackNeeded,
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    let invariant = card
        .invariants
        .iter()
        .find(|row| row.violation_token == "admit_without_rollback_posture")
        .expect("admit_without_rollback_posture invariant must surface");
    assert!(invariant
        .violation_label
        .contains("not_yet_admitted_no_rollback_needed"));
}

#[test]
fn activation_budget_inconsistent_with_denied_decision_surfaces_invariant() {
    let manifest = verified_admit_manifest();
    let effective = verified_admit_effective();
    let mut decision = verified_admit_decision();
    decision.install_decision_class = InstallDecisionClass::Denied;
    decision.install_decision_reason_class = InstallDecisionReasonClass::PublisherQuarantined;
    decision.decision_summary = "Denied for test.".to_owned();
    let wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        // Buggy caller: eager activation alongside a denied decision.
        ActivationBudgetClass::EagerWithinWorkspaceOnly,
        RollbackPostureClass::NotYetAdmittedNoRollbackNeeded,
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card
        .invariants
        .iter()
        .any(|row| row.violation_token == "activation_budget_inconsistent_with_decision"));
}

#[test]
fn step_up_decision_requires_restricted_step_up_activation_budget() {
    let manifest = verified_admit_manifest();
    let effective = verified_admit_effective();
    let mut decision = verified_admit_decision();
    decision.install_decision_class = InstallDecisionClass::AdmitWithStepUp;
    decision.install_decision_reason_class = InstallDecisionReasonClass::StepUpRequiredByPolicyPack;
    decision.decision_summary =
        "Admit with step-up: policy pack requires a typed step-up.".to_owned();
    // Buggy caller: lazy activation paired with admit-with-step-up.
    let wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card
        .invariants
        .iter()
        .any(|row| row.violation_token == "activation_budget_inconsistent_with_decision"));
}

#[test]
fn missing_effective_permission_diff_surfaces_typed_invariant() {
    let manifest = verified_admit_manifest();
    let mut effective = verified_admit_effective();
    effective.declared_vs_effective_diff.clear();
    let wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        verified_admit_decision(),
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    );
    let card = wedge.card();
    assert!(card
        .invariants
        .iter()
        .any(|row| row.violation_token == "effective_permission_diff_missing"));
}

#[test]
fn render_plaintext_quotes_every_section_in_stable_order() {
    let card = verified_admit_wedge().card();
    let text = card.render_plaintext();
    assert!(text.starts_with("[m1_prototype_install_review_fact_grid]"));
    assert!(text.contains("extension=acme-labs/prose-helper"));
    assert!(text.contains("manifest_baseline_ref=manifest_baseline:acme-labs/prose-helper:1.4.2"));
    assert!(text.contains("publisher:"));
    assert!(text.contains("trust_tier=verified_publisher"));
    assert!(text.contains("origin:"));
    assert!(text.contains("source=public_registry"));
    assert!(text.contains("lifecycle:"));
    assert!(text.contains("manifest_scope_completeness=complete"));
    assert!(text.contains("declared_permissions:"));
    assert!(text.contains("filesystem_read=workspace:/docs/**"));
    assert!(text.contains("effective_permission_diff:"));
    assert!(text.contains("widening_attempted_blocked_count=0"));
    assert!(text.contains("activation_budget=lazy_on_demand_only"));
    assert!(text.contains("rollback_posture=clean_uninstall_and_state_purge"));
    assert!(text.contains("decision:"));
    assert!(text.contains("class=admit"));
    assert!(text.contains("single_bounded_wedge_only"));
    assert!(text.contains("no_marketplace_breadth"));
    assert!(text.contains("no_publisher_services"));
    assert!(text.contains("no_compatibility_policy_automation"));
    assert!(text.contains("invariants:\n  - clean"));
}

#[test]
fn record_round_trips_through_serde_json() {
    let card = verified_admit_wedge().card();
    let json = serde_json::to_string(&card).expect("serialise");
    let parsed: InstallReviewFactGridRecord =
        serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, card);
}

#[test]
fn fixture_protected_walk_verified_publisher_admit_replays_into_the_wedge() {
    let fixture: FactGridFixture = load_fixture("protected_walk_verified_publisher_admit.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.is_clean_admit());
}

#[test]
fn fixture_protected_walk_unverified_publisher_review_only_replays_into_the_wedge() {
    let fixture: FactGridFixture =
        load_fixture("protected_walk_unverified_publisher_review_only.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert_eq!(card.decision.install_decision_class_token, "review_only");
    assert_eq!(card.degraded_token.as_deref(), Some("Limited"));
}

#[test]
fn fixture_failure_drill_incomplete_facts_surfaces_typed_invariants() {
    let fixture: FactGridFixture = load_fixture("failure_drill_incomplete_facts.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.has_invariant_violations);
    let expected = fixture
        .expect
        .expected_violation_tokens
        .as_ref()
        .expect("failure drill must list expected violation tokens");
    for token in expected {
        assert!(
            card.invariants
                .iter()
                .any(|row| &row.violation_token == token),
            "expected invariant {token} to fire on failure drill card"
        );
    }
}

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct FactGridFixture {
    #[serde(default)]
    wedge_id: Option<String>,
    #[serde(default)]
    degraded_token: Option<String>,
    activation_budget_class: String,
    rollback_posture_class: String,
    manifest_baseline: FixtureManifest,
    effective_permission_baseline: FixtureEffective,
    install_decision: FixtureDecision,
    expect: FactGridFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    manifest_baseline_id: String,
    extension_identity: String,
    extension_version: String,
    extension_lifecycle_state_class: String,
    host_contract_family_class: String,
    manifest_origin_source_class: String,
    origin_source_label: String,
    publisher_identity_ref: String,
    publisher_display_label: String,
    publisher_trust_tier_class: String,
    publisher_lifecycle_state_class: String,
    publisher_signing_key_ref: String,
    declared_permissions: Vec<FixtureDeclaredPermission>,
    manifest_scope_completeness_class: String,
}

#[derive(Debug, Deserialize)]
struct FixtureDeclaredPermission {
    scope_class: String,
    scope_target: String,
    #[serde(default)]
    scope_constraint: Option<String>,
    rationale_label: String,
}

#[derive(Debug, Deserialize)]
struct FixtureEffective {
    declared_vs_effective_diff: Vec<FixtureDiffEntry>,
    widening_attempted_blocked_count: u32,
}

#[derive(Debug, Deserialize)]
struct FixtureDiffEntry {
    scope_class: String,
    scope_target: String,
    diff_class: String,
    narrowing_reason_label: String,
}

#[derive(Debug, Deserialize)]
struct FixtureDecision {
    install_decision_class: String,
    install_decision_reason_class: String,
    decision_summary: String,
}

#[derive(Debug, Deserialize)]
struct FactGridFixtureExpect {
    has_invariant_violations: bool,
    install_decision_class: String,
    #[serde(default)]
    summary_contains: Option<String>,
    #[serde(default)]
    expected_violation_tokens: Option<Vec<String>>,
}

fn load_fixture(name: &str) -> FactGridFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/install/m1_fact_grid_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn build_card_from_fixture(fixture: &FactGridFixture) -> InstallReviewFactGridRecord {
    let manifest = ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: fixture.manifest_baseline.manifest_baseline_id.clone(),
        extension_identity: fixture.manifest_baseline.extension_identity.clone(),
        extension_version: fixture.manifest_baseline.extension_version.clone(),
        extension_lifecycle_state_class: parse_extension_lifecycle(
            &fixture.manifest_baseline.extension_lifecycle_state_class,
        ),
        host_contract_family_class: parse_host_contract_family(
            &fixture.manifest_baseline.host_contract_family_class,
        ),
        manifest_origin_source_class: parse_origin_source(
            &fixture.manifest_baseline.manifest_origin_source_class,
        ),
        origin_source_label: fixture.manifest_baseline.origin_source_label.clone(),
        publisher_identity_ref: fixture.manifest_baseline.publisher_identity_ref.clone(),
        publisher_display_label: fixture.manifest_baseline.publisher_display_label.clone(),
        publisher_trust_tier_class: parse_publisher_trust_tier(
            &fixture.manifest_baseline.publisher_trust_tier_class,
        ),
        publisher_lifecycle_state_class: parse_publisher_lifecycle(
            &fixture.manifest_baseline.publisher_lifecycle_state_class,
        ),
        publisher_signing_key_ref: fixture.manifest_baseline.publisher_signing_key_ref.clone(),
        declared_permissions: fixture
            .manifest_baseline
            .declared_permissions
            .iter()
            .map(|p| PermissionScopeEntry {
                scope_class: parse_scope_class(&p.scope_class),
                scope_target: p.scope_target.clone(),
                scope_constraint: p.scope_constraint.clone(),
                rationale_label: p.rationale_label.clone(),
            })
            .collect(),
        manifest_scope_completeness_class: parse_manifest_scope_completeness(
            &fixture.manifest_baseline.manifest_scope_completeness_class,
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let effective = EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: fixture
            .effective_permission_baseline
            .declared_vs_effective_diff
            .iter()
            .map(|d| DeclaredVsEffectiveDiffEntry {
                scope_class: parse_scope_class(&d.scope_class),
                scope_target: d.scope_target.clone(),
                diff_class: parse_diff_class(&d.diff_class),
                narrowing_reason_label: d.narrowing_reason_label.clone(),
            })
            .collect(),
        widening_attempted_blocked_count: fixture
            .effective_permission_baseline
            .widening_attempted_blocked_count,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let decision = ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        install_decision_class: parse_install_decision_class(
            &fixture.install_decision.install_decision_class,
        ),
        install_decision_reason_class: parse_install_decision_reason_class(
            &fixture.install_decision.install_decision_reason_class,
        ),
        decision_summary: fixture.install_decision.decision_summary.clone(),
        decided_at: "2026-05-11T08:00:01Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let mut wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        parse_activation_budget(&fixture.activation_budget_class),
        parse_rollback_posture(&fixture.rollback_posture_class),
    );
    if let Some(id) = &fixture.wedge_id {
        wedge = wedge.with_wedge_id(id);
    }
    if let Some(token) = &fixture.degraded_token {
        wedge = wedge.with_degraded(parse_degraded(token));
    }
    wedge.card()
}

fn assert_fixture_matches(card: &InstallReviewFactGridRecord, fixture: &FactGridFixture) {
    assert_eq!(
        card.has_invariant_violations, fixture.expect.has_invariant_violations,
        "has_invariant_violations mismatch"
    );
    assert_eq!(
        card.decision.install_decision_class_token, fixture.expect.install_decision_class,
        "install_decision_class mismatch"
    );
    if let Some(needle) = &fixture.expect.summary_contains {
        assert!(
            card.summary_line.contains(needle),
            "summary_line {:?} should contain {:?}",
            card.summary_line,
            needle,
        );
    }
}

fn parse_publisher_trust_tier(token: &str) -> PublisherTrustTierClass {
    match token {
        "verified_publisher" => PublisherTrustTierClass::VerifiedPublisher,
        "community_publisher" => PublisherTrustTierClass::CommunityPublisher,
        "organisational_publisher" => PublisherTrustTierClass::OrganisationalPublisher,
        "unverified_publisher" => PublisherTrustTierClass::UnverifiedPublisher,
        "quarantined_publisher" => PublisherTrustTierClass::QuarantinedPublisher,
        "anonymous_publisher_class" => PublisherTrustTierClass::AnonymousPublisherClass,
        other => panic!("unknown publisher_trust_tier_class {other}"),
    }
}

fn parse_publisher_lifecycle(token: &str) -> PublisherLifecycleStateClass {
    match token {
        "active" => PublisherLifecycleStateClass::Active,
        "preview" => PublisherLifecycleStateClass::Preview,
        "deprecated" => PublisherLifecycleStateClass::Deprecated,
        "retired" => PublisherLifecycleStateClass::Retired,
        "quarantined" => PublisherLifecycleStateClass::Quarantined,
        other => panic!("unknown publisher_lifecycle_state_class {other}"),
    }
}

fn parse_extension_lifecycle(token: &str) -> ExtensionLifecycleStateClass {
    match token {
        "published" => ExtensionLifecycleStateClass::Published,
        "preview" => ExtensionLifecycleStateClass::Preview,
        "deprecated" => ExtensionLifecycleStateClass::Deprecated,
        "retired" => ExtensionLifecycleStateClass::Retired,
        "quarantined" => ExtensionLifecycleStateClass::Quarantined,
        other => panic!("unknown extension_lifecycle_state_class {other}"),
    }
}

fn parse_origin_source(token: &str) -> ManifestOriginSourceClass {
    match token {
        "public_registry" => ManifestOriginSourceClass::PublicRegistry,
        "private_registry" => ManifestOriginSourceClass::PrivateRegistry,
        "mirror" => ManifestOriginSourceClass::Mirror,
        "offline_bundle" => ManifestOriginSourceClass::OfflineBundle,
        "vendored_local" => ManifestOriginSourceClass::VendoredLocal,
        "unknown_source_class" => ManifestOriginSourceClass::UnknownSourceClass,
        other => panic!("unknown manifest_origin_source_class {other}"),
    }
}

fn parse_host_contract_family(token: &str) -> HostContractFamilyClass {
    match token {
        "wasm_component_model" => HostContractFamilyClass::WasmComponentModel,
        "wasm_core_module" => HostContractFamilyClass::WasmCoreModule,
        "external_host_process" => HostContractFamilyClass::ExternalHostProcess,
        "helper_binary" => HostContractFamilyClass::HelperBinary,
        "remote_side_component" => HostContractFamilyClass::RemoteSideComponent,
        "compatibility_bridge" => HostContractFamilyClass::CompatibilityBridge,
        other => panic!("unknown host_contract_family_class {other}"),
    }
}

fn parse_scope_class(token: &str) -> PermissionScopeClass {
    match token {
        "filesystem_read" => PermissionScopeClass::FilesystemRead,
        "filesystem_write" => PermissionScopeClass::FilesystemWrite,
        "shell_execute" => PermissionScopeClass::ShellExecute,
        "network_egress" => PermissionScopeClass::NetworkEgress,
        "ai_provider_access" => PermissionScopeClass::AiProviderAccess,
        "connected_provider_access" => PermissionScopeClass::ConnectedProviderAccess,
        "secret_handle_use" => PermissionScopeClass::SecretHandleUse,
        "workspace_settings_read" => PermissionScopeClass::WorkspaceSettingsRead,
        "workspace_settings_write" => PermissionScopeClass::WorkspaceSettingsWrite,
        "execution_context_bind" => PermissionScopeClass::ExecutionContextBind,
        "subscription_subscribe" => PermissionScopeClass::SubscriptionSubscribe,
        "ui_command_contribute" => PermissionScopeClass::UiCommandContribute,
        "capability_inherit" => PermissionScopeClass::CapabilityInherit,
        other => panic!("unknown permission_scope_class {other}"),
    }
}

fn parse_diff_class(token: &str) -> EffectivePermissionDiffClass {
    match token {
        "unchanged" => EffectivePermissionDiffClass::Unchanged,
        "narrowed" => EffectivePermissionDiffClass::Narrowed,
        "denied" => EffectivePermissionDiffClass::Denied,
        "step_up_required" => EffectivePermissionDiffClass::StepUpRequired,
        "widening_attempted_blocked" => EffectivePermissionDiffClass::WideningAttemptedBlocked,
        other => panic!("unknown effective_permission_diff_class {other}"),
    }
}

fn parse_manifest_scope_completeness(token: &str) -> ManifestScopeCompletenessClass {
    match token {
        "complete" => ManifestScopeCompletenessClass::Complete,
        "incomplete_publisher_missing" => {
            ManifestScopeCompletenessClass::IncompletePublisherMissing
        }
        "incomplete_origin_missing" => ManifestScopeCompletenessClass::IncompleteOriginMissing,
        "incomplete_permission_rationale_missing" => {
            ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing
        }
        "incomplete_lifecycle_unknown" => {
            ManifestScopeCompletenessClass::IncompleteLifecycleUnknown
        }
        other => panic!("unknown manifest_scope_completeness_class {other}"),
    }
}

fn parse_install_decision_class(token: &str) -> InstallDecisionClass {
    match token {
        "admit" => InstallDecisionClass::Admit,
        "admit_with_step_up" => InstallDecisionClass::AdmitWithStepUp,
        "review_only" => InstallDecisionClass::ReviewOnly,
        "denied" => InstallDecisionClass::Denied,
        other => panic!("unknown install_decision_class {other}"),
    }
}

fn parse_install_decision_reason_class(token: &str) -> InstallDecisionReasonClass {
    match token {
        "admitted_no_violation" => InstallDecisionReasonClass::AdmittedNoViolation,
        "step_up_required_by_policy_pack" => InstallDecisionReasonClass::StepUpRequiredByPolicyPack,
        "review_only_unverified_publisher" => {
            InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher
        }
        "publisher_identity_required" => InstallDecisionReasonClass::PublisherIdentityRequired,
        "publisher_anonymous" => InstallDecisionReasonClass::PublisherAnonymous,
        "publisher_quarantined" => InstallDecisionReasonClass::PublisherQuarantined,
        "publisher_lifecycle_retired" => InstallDecisionReasonClass::PublisherLifecycleRetired,
        "extension_lifecycle_retired" => InstallDecisionReasonClass::ExtensionLifecycleRetired,
        "manifest_scope_incomplete" => InstallDecisionReasonClass::ManifestScopeIncomplete,
        "manifest_origin_unknown" => InstallDecisionReasonClass::ManifestOriginUnknown,
        "declared_permission_rationale_required" => {
            InstallDecisionReasonClass::DeclaredPermissionRationaleRequired
        }
        "effective_permission_widening_attempted" => {
            InstallDecisionReasonClass::EffectivePermissionWideningAttempted
        }
        "lifecycle_state_unknown_class" => InstallDecisionReasonClass::LifecycleStateUnknownClass,
        other => panic!("unknown install_decision_reason_class {other}"),
    }
}

fn parse_activation_budget(token: &str) -> ActivationBudgetClass {
    match token {
        "eager_within_workspace_only" => ActivationBudgetClass::EagerWithinWorkspaceOnly,
        "lazy_on_demand_only" => ActivationBudgetClass::LazyOnDemandOnly,
        "lazy_on_event_subscription" => ActivationBudgetClass::LazyOnEventSubscription,
        "restricted_step_up_required" => ActivationBudgetClass::RestrictedStepUpRequired,
        "denied_by_policy_pack" => ActivationBudgetClass::DeniedByPolicyPack,
        "not_applicable_install_denied" => ActivationBudgetClass::NotApplicableInstallDenied,
        other => panic!("unknown activation_budget_class {other}"),
    }
}

fn parse_rollback_posture(token: &str) -> RollbackPostureClass {
    match token {
        "clean_uninstall_and_state_purge" => RollbackPostureClass::CleanUninstallAndStatePurge,
        "uninstall_with_user_state_retained" => {
            RollbackPostureClass::UninstallWithUserStateRetained
        }
        "quarantine_only_pending_publisher_review" => {
            RollbackPostureClass::QuarantineOnlyPendingPublisherReview
        }
        "uninstall_blocked_pending_admin_review" => {
            RollbackPostureClass::UninstallBlockedPendingAdminReview
        }
        "not_yet_admitted_no_rollback_needed" => {
            RollbackPostureClass::NotYetAdmittedNoRollbackNeeded
        }
        other => panic!("unknown rollback_posture_class {other}"),
    }
}

fn parse_degraded(token: &str) -> DegradedStateToken {
    match token {
        "Warming" => DegradedStateToken::Warming,
        "Cached" => DegradedStateToken::Cached,
        "Partial" => DegradedStateToken::Partial,
        "Stale" => DegradedStateToken::Stale,
        "Offline" => DegradedStateToken::Offline,
        "PolicyBlocked" => DegradedStateToken::PolicyBlocked,
        "Limited" => DegradedStateToken::Limited,
        "Unsupported" => DegradedStateToken::Unsupported,
        "Experimental" => DegradedStateToken::Experimental,
        "RetestPending" => DegradedStateToken::RetestPending,
        other => panic!("unknown degraded_token {other}"),
    }
}
