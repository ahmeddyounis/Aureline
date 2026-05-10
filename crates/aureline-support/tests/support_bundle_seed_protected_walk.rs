//! Integration tests for the support-bundle seed.
//!
//! These tests own the protected walk and the failure drill described
//! in `/docs/support/support_bundle_seed.md`. They drive the public
//! crate API end-to-end: build a preview from caller seeds, persist it
//! to disk, reopen it, and assert the manifest carries exact-build
//! identity plus the local-first redaction posture the chrome will
//! render.
//!
//! The tests also act as the regeneration source for the reviewer
//! fixtures under `/fixtures/support/support_seed_cases/*.json`. Run
//! `cargo test -p aureline-support --test support_bundle_seed_protected_walk -- --ignored emit_fixtures`
//! to refresh those files after a contract change.

use std::path::{Path, PathBuf};

use aureline_support::bundle::{
    ActionabilityImpactClass, DiagnosticDataClass, ExactBuildCapture, ExcludedReasonClass,
    HighRiskContentClass, LocalFirstDefaults, PreviewItemSeed, RedactionState, ReleaseChannelClass,
    SizeEstimate, SupportBundlePreview, SupportBundlePreviewBuilder,
};

const FIXTURE_BUILD_ID: &str =
    "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456";
const FIXTURE_TIMESTAMP_PROTECTED_WALK: &str = "2026-05-10T08:00:00Z";
const FIXTURE_TIMESTAMP_FAILURE_DRILL: &str = "2026-05-10T08:01:00Z";

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(FIXTURE_BUILD_ID, "0.0.0", ReleaseChannelClass::DevLocal)
}

fn build_identity_seed() -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.build_identity".into(),
        title: "Exact build and install identity".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "build_and_install_truth".into(),
        artifact_kind_class: "exact_build_identity_manifest".into(),
        manifest_path_ref: "preview_items[0]".into(),
        bundle_member_path_ref: Some("manifest/build_identity.json".into()),
        source_refs: vec!["docs/build/exact_build_identity_model.md".into()],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(4096),
            confidence_class: "estimated".into(),
            display_label: "4 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::BlocksFirstActionableDiagnosis,
        impact_summary:
            "Removing this row would prevent support from matching crash, symbol, docs, and \
             release evidence to one build."
                .into(),
        notes: "Metadata-only; embedded by default under the local-first defaults.".into(),
    }
}

fn policy_trust_seed() -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.policy_trust_state".into(),
        title: "Policy fingerprint and trust state".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".into(),
        artifact_kind_class: "policy_fingerprint_summary".into(),
        manifest_path_ref: "preview_items[1]".into(),
        bundle_member_path_ref: Some("manifest/policy_trust.json".into()),
        source_refs: vec!["docs/policy/admin_policy_and_bundle_cache_contract.md".into()],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(2048),
            confidence_class: "estimated".into(),
            display_label: "2 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot tell which policy allowed or excluded bundle classes."
                .into(),
        notes: "Metadata-only; the manifest names the policy and trust state that governed \
                collection."
            .into(),
    }
}

fn secret_seed() -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.raw_secrets".into(),
        title: "Captured secret material (failure drill)".into(),
        data_class: DiagnosticDataClass::HighRisk,
        high_risk_content_class: HighRiskContentClass::SecretBearing,
        bundle_section_class: "logs_traces_and_manifests".into(),
        artifact_kind_class: "raw_secret_capture".into(),
        manifest_path_ref: "preview_items[1]".into(),
        bundle_member_path_ref: None,
        source_refs: vec!["docs/security/safe_preview_trust_classes.md".into()],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(2048),
            confidence_class: "upper_bound".into(),
            display_label: "<= 2 KB".into(),
            size_source_class: "upper_bound_policy".into(),
        },
        impact_class: ActionabilityImpactClass::None,
        impact_summary:
            "Removing this row has no diagnostic cost because raw secret bytes never travel.".into(),
        notes: "Failure drill: the local-first defaults rewrite this row to 'prohibited'.".into(),
    }
}

fn build_protected_walk_preview() -> SupportBundlePreview {
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:local-default:0001",
        "Local-first support bundle preview",
        FIXTURE_TIMESTAMP_PROTECTED_WALK,
        fixture_capture(),
    );
    builder
        .add_item(build_identity_seed())
        .add_item(policy_trust_seed());
    builder.build().expect("build protected walk preview")
}

fn build_failure_drill_preview() -> SupportBundlePreview {
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:failure-drill:0001",
        "Failure drill: queued secret material is held back",
        FIXTURE_TIMESTAMP_FAILURE_DRILL,
        fixture_capture(),
    );
    builder
        .add_item(build_identity_seed())
        .add_item(secret_seed());
    builder.build().expect("build failure drill preview")
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| {
            p.join("fixtures")
                .join("support")
                .join("support_seed_cases")
        })
        .expect("derive fixtures dir")
}

#[test]
fn protected_walk_preview_carries_exact_build_identity_and_no_prohibited_rows() {
    let preview = build_protected_walk_preview();

    assert!(preview.manifest.has_exact_build_identity());
    assert!(!preview.honesty_marker_present());
    assert!(!preview.manifest.has_prohibited_row());
    assert_eq!(preview.manifest.preview_items.len(), 2);
    assert_eq!(preview.manifest.excluded_classes.len(), 0);
    assert_eq!(
        preview
            .manifest
            .collection_context
            .active_redaction_profile_ref,
        LocalFirstDefaults::PROFILE_REF
    );
    assert!(
        preview
            .manifest
            .reopen_after_export_path
            .can_reopen_without_network
    );
    assert_eq!(
        preview
            .manifest
            .build_identity
            .release_channel_class
            .as_str(),
        ReleaseChannelClass::DevLocal.as_str()
    );

    for item in &preview.manifest.preview_items {
        assert_eq!(
            item.redaction.redaction_state,
            RedactionState::NotRequiredMetadata
        );
        assert!(item.redaction.visible_high_risk_label.is_none());
    }
}

#[test]
fn failure_drill_preview_rewrites_secret_row_to_prohibited_and_records_excluded_entry() {
    let preview = build_failure_drill_preview();

    assert!(preview.manifest.has_exact_build_identity());
    assert!(preview.manifest.has_prohibited_row());
    assert!(preview.honesty_marker_present());

    let secret_row = preview
        .manifest
        .preview_items
        .iter()
        .find(|i| i.parity_binding.support_pack_item_id == "support.item.raw_secrets")
        .expect("secret row preserved in preview");
    assert_eq!(
        secret_row.redaction.redaction_state,
        RedactionState::Prohibited
    );
    assert!(secret_row.redaction.visible_high_risk_label.is_some());

    assert!(preview
        .manifest
        .redaction_report
        .prohibited_items_confirmed_absent
        .iter()
        .any(|id| id == "support.item.raw_secrets"));
    assert!(preview.manifest.excluded_classes.iter().any(|c| {
        c.support_pack_item_id.as_deref() == Some("support.item.raw_secrets")
            && matches!(
                c.exclusion_reason_class,
                ExcludedReasonClass::ProhibitedSecretOrToken
            )
    }));
    assert!(
        !preview
            .manifest
            .redaction_report
            .secret_scan_summary
            .raw_secret_values_exported
    );
}

#[test]
fn preview_round_trips_to_disk_and_reloads_verbatim() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("preview.json");

    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:roundtrip:0001",
        "Round-trip preview",
        FIXTURE_TIMESTAMP_PROTECTED_WALK,
        fixture_capture(),
    );
    builder.add_item(build_identity_seed());
    let preview = builder
        .write_preview_snapshot(&path)
        .expect("write preview");

    let bytes = std::fs::read(&path).expect("read preview");
    let parsed: SupportBundlePreview = serde_json::from_slice(&bytes).expect("parse preview");

    assert_eq!(parsed, preview);
    assert!(parsed.manifest.has_exact_build_identity());
    assert_eq!(
        parsed.manifest.build_identity.exact_build_refs,
        vec![FIXTURE_BUILD_ID.to_owned()]
    );
}

#[test]
fn protected_walk_fixture_matches_seed_output() {
    let preview = build_protected_walk_preview();
    let path = fixtures_dir().join("protected_walk_default_local_preview.json");
    let stored: SupportBundlePreview = serde_json::from_slice(
        &std::fs::read(&path).unwrap_or_else(|err| {
            panic!(
                "read protected-walk fixture {}: {err}\n  hint: regenerate via cargo test -- --ignored emit_fixtures",
                path.display()
            )
        }),
    )
    .expect("parse protected-walk fixture");
    assert_eq!(stored, preview);
}

#[test]
fn failure_drill_fixture_matches_seed_output() {
    let preview = build_failure_drill_preview();
    let path = fixtures_dir().join("failure_drill_secret_bearing_prohibited.json");
    let stored: SupportBundlePreview = serde_json::from_slice(
        &std::fs::read(&path).unwrap_or_else(|err| {
            panic!(
                "read failure-drill fixture {}: {err}\n  hint: regenerate via cargo test -- --ignored emit_fixtures",
                path.display()
            )
        }),
    )
    .expect("parse failure-drill fixture");
    assert_eq!(stored, preview);
}

/// Regeneration helper. Run with
/// `cargo test -p aureline-support --test support_bundle_seed_protected_walk -- --ignored emit_fixtures`
/// to refresh the JSON snapshots under
/// `/fixtures/support/support_seed_cases/`.
#[test]
#[ignore]
fn emit_fixtures() {
    let dir = fixtures_dir();
    std::fs::create_dir_all(&dir).expect("create fixtures dir");

    let walk = build_protected_walk_preview();
    let walk_bytes = serde_json::to_vec_pretty(&walk).expect("ser walk");
    std::fs::write(
        dir.join("protected_walk_default_local_preview.json"),
        walk_bytes,
    )
    .expect("write walk fixture");

    let drill = build_failure_drill_preview();
    let drill_bytes = serde_json::to_vec_pretty(&drill).expect("ser drill");
    std::fs::write(
        dir.join("failure_drill_secret_bearing_prohibited.json"),
        drill_bytes,
    )
    .expect("write drill fixture");
}
