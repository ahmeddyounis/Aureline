//! Unit and fixture coverage for the extension manifest editor session.

use serde::Deserialize;

use super::*;

/// Every blocker (conformance-lane) check id the editor may emit. These mirror
/// the beta extension validator CLI
/// (`tools/extensions/m3/validator_cli/aureline_extension_validator.py`) and the
/// conformance kit report schema. Drift here means the editor and the CLI would
/// export findings under different stable ids.
const KNOWN_VALIDATOR_CHECK_IDS: &[&str] = &[
    "manifest_shape.object_required",
    "manifest_shape.required_fields",
    "manifest_shape.schema_version",
    "manifest_shape.package_id",
    "manifest_shape.publisher_identity",
    "manifest_shape.version_semver",
    "manifest_shape.runtime_origin_known",
    "manifest_shape.host_contract_family_known",
    "compatibility_targets.sdk_line_id",
    "compatibility_targets.sdk_semver",
    "compatibility_targets.lifecycle_metadata_refs",
    "compatibility_targets.host_abi_window",
    "compatibility_targets.host_family_matches_runtime_origin",
    "compatibility_targets.wit_worlds_declared",
    "compatibility_targets.external_host_contract_declared",
    "compatibility_targets.aureline_version_range",
    "compatibility_targets.platforms_declared",
    "compatibility_targets.support_class",
    "compatibility_targets.bridge_state",
    "permission_declarations.present",
    "permission_declarations.scope_known",
    "permission_declarations.targets_declared",
    "permission_declarations.purpose_text",
    "permission_declarations.trust_mode",
    "permission_declarations.prompt_copy",
    "permission_declarations.review_required_for_privileged_scope",
    "permission_declarations.network_endpoint_class",
    "permission_declarations.secret_handle_only",
    "permission_declarations.no_duplicate_scope_target",
    "lifecycle_metadata.state_known",
    "lifecycle_metadata.activation_triggers",
    "lifecycle_metadata.activation_budget",
    "lifecycle_metadata.degraded_path",
    "lifecycle_metadata.disable_support",
    "lifecycle_metadata.rollback_support",
    "conformance_fixtures.required_scenario_coverage",
    "conformance_fixtures.scenario_class_known",
    "conformance_fixtures.fixture_ref",
];

#[derive(Debug, Deserialize)]
struct Fixture {
    input: ManifestEditorSessionInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_publish_readiness: ManifestEditorPublishReadinessClass,
    expected_publish_readiness_reason: ManifestEditorPublishReadinessReasonClass,
    expected_result_class: ManifestEditorResultClass,
    expected_blocker_count: u32,
    expected_advisory_count: u32,
    expected_migration_hint_count: u32,
    #[serde(default)]
    expected_permission_chip_count: Option<u32>,
    #[serde(default)]
    expected_required_shim: Option<bool>,
    #[serde(default)]
    expected_red_flag_classes: Option<Vec<String>>,
    #[serde(default)]
    expected_blocker_check_ids: Option<Vec<String>>,
}

fn fixture_raw(name: &str) -> &'static str {
    match name {
        "ready_to_publish_wasm" => include_str!(
            "../../../../fixtures/extensions/m3/manifest_editor/ready_to_publish_wasm.json"
        ),
        "advisories_deprecated_state_eager_startup" => include_str!(
            "../../../../fixtures/extensions/m3/manifest_editor/advisories_deprecated_state_eager_startup.json"
        ),
        "blocked_invalid_identity_and_vocabulary" => include_str!(
            "../../../../fixtures/extensions/m3/manifest_editor/blocked_invalid_identity_and_vocabulary.json"
        ),
        other => panic!("unknown fixture {other}"),
    }
}

fn load_fixture(name: &str) -> Fixture {
    serde_json::from_str(fixture_raw(name))
        .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) -> ManifestEditorSession {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let session = evaluate_manifest_editor_session(fixture.input);

    assert_eq!(session.record_kind, MANIFEST_EDITOR_SESSION_RECORD_KIND);
    assert_eq!(
        session.manifest_editor_session_schema_version,
        MANIFEST_EDITOR_SESSION_SCHEMA_VERSION
    );
    assert!(session.local_validation_only);
    assert!(!session.network_round_trip_required);
    assert_eq!(session.redaction_class, RedactionClass::MetadataSafeDefault);

    assert_eq!(
        session.publish_readiness, fixture.meta.expected_publish_readiness,
        "publish readiness mismatch for {name}"
    );
    assert_eq!(
        session.publish_readiness_reason, fixture.meta.expected_publish_readiness_reason,
        "publish readiness reason mismatch for {name}"
    );
    assert_eq!(
        session.conformance_export.result_class, fixture.meta.expected_result_class,
        "result class mismatch for {name}"
    );
    assert_eq!(
        session.blocker_count, fixture.meta.expected_blocker_count,
        "blocker count mismatch for {name}"
    );
    assert_eq!(
        session.advisory_count, fixture.meta.expected_advisory_count,
        "advisory count mismatch for {name}"
    );
    assert_eq!(
        session.migration_hints.len() as u32,
        fixture.meta.expected_migration_hint_count,
        "migration hint count mismatch for {name}"
    );
    if let Some(expected) = fixture.meta.expected_permission_chip_count {
        assert_eq!(
            session.permission_chips.len() as u32,
            expected,
            "permission chip count mismatch for {name}"
        );
    }
    if let Some(expected) = fixture.meta.expected_required_shim {
        assert_eq!(
            session.version_targeting.required_shim, expected,
            "required shim mismatch for {name}"
        );
    }
    if let Some(expected) = &fixture.meta.expected_red_flag_classes {
        assert_eq!(
            &session.conformance_export.red_flag_classes, expected,
            "red flag classes mismatch for {name}"
        );
    }
    if let Some(expected_ids) = &fixture.meta.expected_blocker_check_ids {
        let failing: Vec<&str> = session
            .findings
            .iter()
            .filter(|f| {
                f.status == ManifestEditorFindingStatus::Fail
                    && f.severity == ManifestEditorFindingSeverity::Blocker
            })
            .map(|f| f.check_id.as_str())
            .collect();
        for id in expected_ids {
            assert!(
                failing.contains(&id.as_str()),
                "expected blocker {id} not present in {name}"
            );
        }
    }

    // The structural validator must accept every evaluated session.
    let structural = validate_manifest_editor_session(&session);
    assert!(
        structural.is_empty(),
        "structural findings for {name}: {structural:?}"
    );

    // Every non-advisory (blocker-lane) finding must reuse a validator-known
    // stable check id and carry blocker severity; advisories must not.
    for finding in &session.findings {
        match finding.suite {
            ManifestEditorFindingSuite::EditorAdvisory => {
                assert!(
                    finding.check_id.starts_with("advisory."),
                    "advisory finding {} must use the advisory namespace",
                    finding.check_id
                );
                assert_ne!(finding.severity, ManifestEditorFindingSeverity::Blocker);
            }
            _ => {
                assert!(
                    KNOWN_VALIDATOR_CHECK_IDS.contains(&finding.check_id.as_str()),
                    "blocker-lane check id {} drifted from the validator vocabulary",
                    finding.check_id
                );
                assert_eq!(finding.severity, ManifestEditorFindingSeverity::Blocker);
                // Field-anchored findings must carry a JSON pointer.
                if finding.field.is_some() {
                    assert!(finding.anchor.is_some());
                }
            }
        }
    }

    session
}

#[test]
fn ready_to_publish_wasm_has_no_blockers_or_advisories() {
    let session = run_fixture("ready_to_publish_wasm");
    assert_eq!(
        session.publish_readiness,
        ManifestEditorPublishReadinessClass::ReadyToPublish
    );
    assert_eq!(
        session.conformance_export.compatibility_badge_class,
        ManifestEditorCompatibilityBadgeClass::CompatibleOnDeclaredTargets
    );
    assert!(session.conformance_export.red_flag_classes.is_empty());
    assert!(session.passed_count > 0);
}

#[test]
fn advisories_fixture_surfaces_migration_hint_not_a_blocker() {
    let session = run_fixture("advisories_deprecated_state_eager_startup");

    // The deprecated lifecycle.state value is still a known state, so it must
    // NOT raise a generic invalid/unknown blocker.
    let state_blocker = session.findings.iter().any(|f| {
        f.check_id == "lifecycle_metadata.state_known"
            && f.status == ManifestEditorFindingStatus::Fail
    });
    assert!(
        !state_blocker,
        "deprecated-but-valid lifecycle.state must not raise a blocker"
    );

    // Instead it produces an actionable migration hint.
    assert_eq!(session.migration_hints.len(), 1);
    let hint = &session.migration_hints[0];
    assert_eq!(hint.field_path, "lifecycle.state");
    assert_eq!(hint.field_anchor, "/lifecycle/state");
    assert_eq!(hint.deprecated_value, "resolved");
    assert_eq!(hint.replacement_value.as_deref(), Some("verified"));
    assert_eq!(
        hint.replacement_example.as_deref(),
        Some("\"state\": \"verified\"")
    );
    assert!(hint.removal_target_version.is_some());
    assert!(hint.removal_target_date.is_some());
    assert!(hint.migration_guide_ref.is_some());
    assert_eq!(
        hint.posture,
        LifecycleDeprecationPostureClass::DeprecatedWithReplacement
    );

    // And a matching inline advisory anchored at the deprecated field.
    let advisory = session
        .findings
        .iter()
        .find(|f| f.check_id == "advisory.deprecated_manifest_field")
        .expect("deprecated-field advisory must be present");
    assert_eq!(advisory.anchor.as_deref(), Some("/lifecycle/state"));
    assert_eq!(advisory.severity, ManifestEditorFindingSeverity::Warning);

    // The bridge target drives the required-shim posture and a shim note.
    assert!(session.version_targeting.required_shim);
    assert!(session.version_targeting.shim_note.is_some());
}

#[test]
fn blocked_fixture_anchors_must_fix_blockers() {
    let session = run_fixture("blocked_invalid_identity_and_vocabulary");
    assert_eq!(
        session.publish_readiness,
        ManifestEditorPublishReadinessClass::BlockedOnMustFix
    );
    assert_eq!(
        session.conformance_export.result_class,
        ManifestEditorResultClass::Fail
    );
    assert_eq!(
        session.conformance_export.compatibility_badge_class,
        ManifestEditorCompatibilityBadgeClass::UnsupportedPendingQualification
    );

    let publisher = session
        .findings
        .iter()
        .find(|f| f.check_id == "manifest_shape.publisher_identity")
        .expect("publisher identity check present");
    assert_eq!(publisher.status, ManifestEditorFindingStatus::Fail);
    assert_eq!(publisher.anchor.as_deref(), Some("/publisher_id"));

    let support = session
        .findings
        .iter()
        .find(|f| f.check_id == "compatibility_targets.support_class")
        .expect("support class check present");
    assert_eq!(
        support.anchor.as_deref(),
        Some("/compatibility/support_class")
    );
}

#[test]
fn permission_chips_explain_capability_and_privilege() {
    let session = run_fixture("ready_to_publish_wasm");
    let chips = &session.permission_chips;
    assert_eq!(chips.len(), 3);

    let fs = &chips[0];
    assert_eq!(fs.scope, "filesystem_read");
    assert!(fs.scope_known);
    assert_eq!(fs.capability_class, Some(CapabilityClassClass::Filesystem));
    assert!(!fs.privileged);
    assert_eq!(fs.anchor, "/permissions/0");

    let net = &chips[1];
    assert_eq!(net.scope, "network_egress");
    assert_eq!(net.capability_class, Some(CapabilityClassClass::Network));
    assert!(net.privileged);
    assert!(net.review_required);
    assert!(net.prompt_summary_present);
    assert_eq!(
        net.network_endpoint_class.as_deref(),
        Some("metadata_fetch")
    );

    let ui = &chips[2];
    assert_eq!(ui.capability_class, Some(CapabilityClassClass::Ui));
    assert!(!ui.privileged);
}

#[test]
fn session_round_trips_through_json() {
    let session = run_fixture("advisories_deprecated_state_eager_startup");
    let json = serde_json::to_string(&session).expect("serialize");
    let back: ManifestEditorSession = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(session, back);
}

#[test]
fn links_point_at_canonical_schema_and_docs() {
    let session = run_fixture("ready_to_publish_wasm");
    let links = &session.links;
    assert_eq!(
        links.manifest_schema_ref,
        "schemas/extensions/beta_extension_manifest.schema.json"
    );
    assert_eq!(
        links.conformance_report_schema_ref,
        "schemas/extensions/conformance_kit_report.schema.json"
    );
    assert_eq!(
        links.validator_cli_ref,
        "tools/extensions/m3/validator_cli/aureline_extension_validator.py"
    );
    assert_eq!(
        links.authoring_docs_ref,
        "docs/ecosystem/m3/manifest_editor_beta.md"
    );
}

#[test]
fn empty_object_manifest_blocks_with_required_field_findings() {
    let input = ManifestEditorSessionInput {
        session_id: "extension_manifest_editor_session:empty".to_string(),
        generated_at: "2026-05-19T12:00:00Z".to_string(),
        subject_manifest_ref: "workspace://draft.json".to_string(),
        connectivity_class: ManifestEditorConnectivityClass::LocalOffline,
        manifest: serde_json::json!({}),
    };
    let session = evaluate_manifest_editor_session(input);
    assert_eq!(
        session.publish_readiness,
        ManifestEditorPublishReadinessClass::BlockedOnMustFix
    );
    assert!(session
        .findings
        .iter()
        .any(|f| f.check_id == "manifest_shape.required_fields"
            && f.status == ManifestEditorFindingStatus::Fail));
    assert!(validate_manifest_editor_session(&session).is_empty());
}

#[test]
fn non_object_manifest_raises_object_required() {
    let input = ManifestEditorSessionInput {
        session_id: "extension_manifest_editor_session:scalar".to_string(),
        generated_at: "2026-05-19T12:00:00Z".to_string(),
        subject_manifest_ref: "workspace://draft.json".to_string(),
        connectivity_class: ManifestEditorConnectivityClass::LocalOffline,
        manifest: serde_json::json!("not-an-object"),
    };
    let session = evaluate_manifest_editor_session(input);
    let root = session
        .findings
        .iter()
        .find(|f| f.check_id == "manifest_shape.object_required")
        .expect("object_required present");
    assert_eq!(root.field.as_deref(), Some("(root)"));
    assert_eq!(root.anchor.as_deref(), Some(""));
}

#[test]
fn field_path_to_anchor_handles_indexes_and_root() {
    assert_eq!(field_path_to_anchor("(root)"), "");
    assert_eq!(
        field_path_to_anchor("manifest_version"),
        "/manifest_version"
    );
    assert_eq!(
        field_path_to_anchor("compatibility.aureline_versions"),
        "/compatibility/aureline_versions"
    );
    assert_eq!(
        field_path_to_anchor("permissions[0].scope"),
        "/permissions/0/scope"
    );
    assert_eq!(
        field_path_to_anchor("conformance.fixtures[2].fixture_ref"),
        "/conformance/fixtures/2/fixture_ref"
    );
}

#[test]
fn validation_is_offline_for_every_connectivity_class() {
    for connectivity in [
        ManifestEditorConnectivityClass::LocalOffline,
        ManifestEditorConnectivityClass::MirrorReachable,
        ManifestEditorConnectivityClass::PrimaryRegistryReachable,
    ] {
        let mut fixture = load_fixture("ready_to_publish_wasm");
        fixture.input.connectivity_class = connectivity;
        let session = evaluate_manifest_editor_session(fixture.input);
        assert!(session.local_validation_only);
        assert!(!session.network_round_trip_required);
        assert_eq!(
            session.publish_readiness,
            ManifestEditorPublishReadinessClass::ReadyToPublish
        );
    }
}
