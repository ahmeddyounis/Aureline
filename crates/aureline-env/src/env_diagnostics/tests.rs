use super::*;
use std::collections::BTreeSet;

fn bundle(bundle_id: &str) -> EnvArtifactBundle {
    seeded_env_artifact_bundles()
        .into_iter()
        .find(|bundle| bundle.bundle_id == bundle_id)
        .unwrap_or_else(|| panic!("bundle {bundle_id} exists"))
}

#[test]
fn every_seeded_bundle_validates_and_imports() {
    for bundle in seeded_env_artifact_bundles() {
        validate_env_artifact_bundle(&bundle)
            .unwrap_or_else(|err| panic!("bundle {} must validate: {err}", bundle.bundle_id));
        import_env_bundle(&bundle)
            .unwrap_or_else(|err| panic!("bundle {} must import: {err}", bundle.bundle_id));
    }
}

#[test]
fn every_seeded_fixture_validates() {
    for fixture in seeded_env_diagnostics_fixtures() {
        validate_env_diagnostics_fixture(&fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn corpus_covers_every_source_channel() {
    let channels: BTreeSet<SourceChannel> = seeded_env_artifact_bundles()
        .iter()
        .map(|bundle| bundle.provenance.source_channel)
        .collect();
    for required in SourceChannel::ALL {
        assert!(
            channels.contains(&required),
            "bundles must cover the {} channel",
            required.as_str()
        );
    }
}

#[test]
fn corpus_covers_every_hydration_outcome_and_key_finding_codes() {
    let mut outcomes = BTreeSet::new();
    let mut codes = BTreeSet::new();
    for bundle in seeded_env_artifact_bundles() {
        for diagnostic in diagnose_bundle(&bundle).diagnostics {
            outcomes.insert(diagnostic.outcome);
            codes.insert(diagnostic.finding_code);
        }
    }
    for required in [
        HydrationOutcome::Trusted,
        HydrationOutcome::Degraded,
        HydrationOutcome::Unreusable,
        HydrationOutcome::Untrusted,
    ] {
        assert!(outcomes.contains(&required), "missing outcome {required:?}");
    }
    // The marquee diagnostics the lane must surface across mirror/offline.
    for required in [
        FindingCode::Trusted,
        FindingCode::MirrorSourceUnverified,
        FindingCode::PrebuildInvalidated,
        FindingCode::ClaimWithheld,
        FindingCode::MaterializationMismatch,
    ] {
        assert!(
            codes.contains(&required),
            "missing finding code {required:?}"
        );
    }
}

#[test]
fn online_bundle_is_fully_trusted_and_pending_review() {
    let report = diagnose_bundle(&bundle("env.bundle.local_online"));
    assert_eq!(report.source_channel, SourceChannel::Online);
    assert!(!report.share_blocked);
    assert_eq!(report.review_state, ReviewState::PendingReview);
    assert_eq!(report.untrusted_count, 0);
    assert_eq!(report.degraded_count, 0);
    assert_eq!(report.unreusable_count, 0);
    assert_eq!(report.trusted_count, report.diagnostics.len());
    assert!(report
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.finding_code == FindingCode::Trusted));
}

#[test]
fn mirror_bundle_downgrades_visibly_but_stays_shareable() {
    let report = diagnose_bundle(&bundle("env.bundle.remote_mirror"));
    assert_eq!(report.source_channel, SourceChannel::Mirror);
    assert!(
        !report.share_blocked,
        "a mirror bundle with only degraded artifacts is not blocked"
    );
    assert_eq!(report.review_state, ReviewState::PendingReview);
    assert_eq!(report.untrusted_count, 0);
    assert!(report.degraded_count > 0, "mirror artifacts downgrade");
    // The community template's mirror provenance is surfaced explicitly,
    // reusing the same vocabulary the online path uses.
    let template = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.artifact_kind == ArtifactKind::Template)
        .expect("mirror bundle carries a template");
    assert_eq!(template.finding_code, FindingCode::MirrorSourceUnverified);
    assert_eq!(template.source_channel, SourceChannel::Mirror);
    // Every diagnostic carries the mirror channel, not an opaque import.
    assert!(report
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.source_channel == SourceChannel::Mirror));
}

#[test]
fn offline_bundle_is_blocked_by_untrusted_artifacts() {
    let report = diagnose_bundle(&bundle("env.bundle.offline_sealed"));
    assert_eq!(report.source_channel, SourceChannel::Offline);
    assert!(report.share_blocked);
    assert_eq!(report.review_state, ReviewState::Blocked);
    assert!(report.untrusted_count >= 2, "ungated hook and wrong target");
    let codes: BTreeSet<FindingCode> = report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.finding_code)
        .collect();
    assert!(codes.contains(&FindingCode::ClaimWithheld));
    assert!(codes.contains(&FindingCode::MaterializationMismatch));
    assert!(codes.contains(&FindingCode::PrebuildInvalidated));
    // Every untrusted artifact appears in the blocking-token roll-up.
    assert_eq!(
        report.blocking_artifact_tokens.len(),
        report.untrusted_count
    );
}

#[test]
fn materialization_diagnostics_explain_wrong_target() {
    // The wrong-target runtime is reported as a mismatch with the reasons
    // behind it, never collapsed into a generic "workspace started".
    let report = diagnose_bundle(&bundle("env.bundle.offline_sealed"));
    let runtime = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.artifact_kind == ArtifactKind::Runtime)
        .expect("offline bundle carries a runtime");
    assert_eq!(runtime.finding_code, FindingCode::MaterializationMismatch);
    assert_eq!(runtime.outcome, HydrationOutcome::Untrusted);
    assert!(
        !runtime.reason_tokens.is_empty(),
        "a mismatch must carry the facets behind it"
    );
}

#[test]
fn desktop_headless_and_support_share_one_report() {
    let bundle = bundle("env.bundle.remote_mirror");
    let desktop = desktop_env_diagnostics(&bundle);
    let headless = headless_env_diagnostics(&bundle);
    let support = support_env_diagnostics(&bundle);
    assert_eq!(desktop, headless, "desktop and headless must be identical");
    assert_eq!(support, desktop, "support must read the same report object");
}

#[test]
fn doctor_probes_mirror_the_report_one_for_one() {
    let bundle = bundle("env.bundle.offline_sealed");
    let report = diagnose_bundle(&bundle);
    let probes = doctor_env_probes(&bundle);
    assert_eq!(probes.len(), report.diagnostics.len());
    for (probe, diagnostic) in probes.iter().zip(report.diagnostics.iter()) {
        assert_eq!(probe.finding_code, diagnostic.finding_code);
        assert_eq!(probe.artifact_id, diagnostic.artifact_id);
        assert_eq!(probe.source_channel, diagnostic.source_channel);
        assert_eq!(probe.redaction_class, RedactionClass::MetadataOnly);
        if diagnostic.blocks_share {
            assert_eq!(probe.severity, ProbeSeverity::Blocking);
        }
    }
    // The offline bundle has at least one blocking probe Doctor can explain.
    assert!(probes
        .iter()
        .any(|probe| probe.severity == ProbeSeverity::Blocking));
}

#[test]
fn comparison_flags_channel_and_artifact_changes() {
    let online = bundle("env.bundle.local_online");
    let mirror = bundle("env.bundle.remote_mirror");
    let comparison = compare_env_bundles(&online, &mirror);
    assert!(comparison.source_channel_changed);
    assert_eq!(comparison.base_source_channel, SourceChannel::Online);
    assert_eq!(comparison.target_source_channel, SourceChannel::Mirror);
    assert!(comparison.schema_version_compatible);
    assert!(!comparison.identical);
    assert!(
        !comparison.deltas.is_empty(),
        "different bundles must produce deltas"
    );
}

#[test]
fn comparison_of_identical_bundles_is_empty() {
    let bundle = bundle("env.bundle.local_online");
    let comparison = compare_env_bundles(&bundle, &bundle);
    assert!(comparison.identical);
    assert!(comparison.deltas.is_empty());
    assert!(!comparison.source_channel_changed);
}

#[test]
fn comparison_detects_schema_version_drift() {
    let base = bundle("env.bundle.local_online");
    let mut target = base.clone();
    target.provenance.schema_version = ENV_DIAGNOSTICS_SCHEMA_VERSION + 1;
    let comparison = compare_env_bundles(&base, &target);
    assert!(!comparison.schema_version_compatible);
    assert!(!comparison.identical);
}

#[test]
fn unsupported_artifact_schema_version_is_untrusted() {
    let mut bundle = bundle("env.bundle.local_online");
    bundle.capsules[0].schema_version = 999;
    let report = diagnose_bundle(&bundle);
    let capsule = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.artifact_kind == ArtifactKind::Capsule)
        .expect("capsule diagnostic");
    assert_eq!(capsule.finding_code, FindingCode::SchemaVersionUnsupported);
    assert_eq!(capsule.outcome, HydrationOutcome::Untrusted);
    assert!(report.share_blocked);
}

#[test]
fn import_rejects_a_mirror_bundle_without_an_origin_ref() {
    let mut bundle = bundle("env.bundle.remote_mirror");
    bundle.provenance.mirror_origin_ref = String::new();
    let err = import_env_bundle(&bundle).expect_err("a mirror bundle must name its origin");
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "bundle.provenance.mirror_origin_ref"));
}

#[test]
fn import_rejects_an_empty_bundle() {
    let empty = assemble_env_bundle(
        "env.bundle.empty",
        ArtifactProvenance {
            schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
            producer_surface: ProducerSurface::Desktop,
            producer_build_ref: "artifacts/build/build_identity.json".to_owned(),
            source_channel: SourceChannel::Online,
            source_truth: "First-party origin".to_owned(),
            mirror_origin_ref: String::new(),
            redaction_class: RedactionClass::MetadataOnly,
            captured_ref: "artifacts/env/env-diagnostics-runbook.md".to_owned(),
        },
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let err = import_env_bundle(&empty).expect_err("an empty bundle must not import");
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "bundle.artifacts"));
}

#[test]
fn report_and_bundle_round_trip_through_json() {
    let bundle = bundle("env.bundle.remote_mirror");
    let bundle_json = serde_json::to_string(&bundle).expect("bundle serializes");
    let bundle_back: EnvArtifactBundle =
        serde_json::from_str(&bundle_json).expect("bundle deserializes");
    assert_eq!(bundle, bundle_back);

    let report = diagnose_bundle(&bundle);
    let report_json = serde_json::to_string(&report).expect("report serializes");
    let report_back: EnvDiagnosticsReport =
        serde_json::from_str(&report_json).expect("report deserializes");
    assert_eq!(report, report_back);
}

#[test]
fn finding_code_blocking_invariant_matches_outcome() {
    // Share is blocked exactly when an artifact is untrusted.
    for bundle in seeded_env_artifact_bundles() {
        let report = diagnose_bundle(&bundle);
        let any_untrusted = report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.outcome == HydrationOutcome::Untrusted);
        assert_eq!(report.share_blocked, any_untrusted);
        for diagnostic in &report.diagnostics {
            assert_eq!(
                diagnostic.blocks_share,
                diagnostic.outcome == HydrationOutcome::Untrusted
            );
        }
    }
}
