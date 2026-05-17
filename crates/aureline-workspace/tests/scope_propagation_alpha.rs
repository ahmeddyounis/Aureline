//! Fixture-driven coverage for the scope-propagation alpha lane.
//!
//! Each fixture pairs an alpha `workset_artifact_record` with a beta
//! consumer surface and a crossing class, then asserts that the
//! propagation alpha record preserves every scope label, observes every
//! required guardrail, and resolves to the expected disposition.

use std::path::Path;

use serde::Deserialize;

use aureline_workspace::{
    BetaConsumerSurface, ScopeClass, ScopeMode, ScopeObservationInputs, ScopePropagationAlphaError,
    ScopePropagationAlphaRecord, ScopePropagationAlphaSupportExport, ScopePropagationCrossingClass,
    ScopePropagationDegradedReason, ScopePropagationDestination, ScopePropagationGuardrail,
    ScopePropagationProjectionInputs, WorksetArtifactRecord, WorksetScopeBetaTruth,
};

#[derive(Debug, Clone, Deserialize)]
struct Fixture {
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
    workspace: WorkspaceBlock,
    artifact: WorksetArtifactRecord,
    source_consumer_surface: String,
    crossing: String,
    destination: DestinationBlock,
    #[serde(default)]
    hidden_member_count: Option<u32>,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureMeta {
    name: String,
    #[serde(default)]
    outside_root_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceBlock {
    workspace_root_refs: Vec<String>,
    #[serde(default)]
    workspace_root_labels: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
struct DestinationBlock {
    kind: String,
    destination_label: String,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default)]
    explain_note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    disposition: String,
    completed: bool,
    remote_attach_disclosed: bool,
    hidden_member_count_preserved: bool,
    preserved_scope_class: String,
    preserved_scope_mode: String,
    lineage_length: u32,
    #[serde(default)]
    lineage_underlying_ref: Option<String>,
    guardrails_required: Vec<String>,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/remote_provider_scope")
}

fn load_fixtures() -> Vec<(std::path::PathBuf, Fixture)> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("propagation fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let parsed: Fixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
            (path, parsed)
        })
        .collect()
}

fn surface_from_token(token: &str) -> BetaConsumerSurface {
    match token {
        "search" => BetaConsumerSurface::Search,
        "graph" => BetaConsumerSurface::Graph,
        "refactor" => BetaConsumerSurface::Refactor,
        "ai" => BetaConsumerSurface::Ai,
        "export" => BetaConsumerSurface::Export,
        "support_packet" => BetaConsumerSurface::SupportPacket,
        other => panic!("unknown consumer surface token {other}"),
    }
}

fn crossing_from_token(token: &str) -> ScopePropagationCrossingClass {
    match token {
        "remote_helper_attach" => ScopePropagationCrossingClass::RemoteHelperAttach,
        "provider_overlay_link" => ScopePropagationCrossingClass::ProviderOverlayLink,
        "export_archive_write" => ScopePropagationCrossingClass::ExportArchiveWrite,
        "browser_handoff_mint" => ScopePropagationCrossingClass::BrowserHandoffMint,
        "support_packet_bundle" => ScopePropagationCrossingClass::SupportPacketBundle,
        other => panic!("unknown crossing token {other}"),
    }
}

fn degraded_reason_from_token(token: &str) -> ScopePropagationDegradedReason {
    match token {
        "remote_helper_unreachable" => ScopePropagationDegradedReason::RemoteHelperUnreachable,
        "remote_helper_skew" => ScopePropagationDegradedReason::RemoteHelperSkew,
        "provider_overlay_stale" => ScopePropagationDegradedReason::ProviderOverlayStale,
        "browser_handoff_expiring_session" => {
            ScopePropagationDegradedReason::BrowserHandoffExpiringSession
        }
        "export_target_unavailable" => ScopePropagationDegradedReason::ExportTargetUnavailable,
        "support_attribution_only" => ScopePropagationDegradedReason::SupportAttributionOnly,
        other => panic!("unknown degraded reason token {other}"),
    }
}

fn guardrail_from_token(token: &str) -> ScopePropagationGuardrail {
    match token {
        "no_silent_scope_widening" => ScopePropagationGuardrail::NoSilentScopeWidening,
        "hidden_members_not_leaked" => ScopePropagationGuardrail::HiddenMembersNotLeaked,
        "degraded_state_not_masked" => ScopePropagationGuardrail::DegradedStateNotMasked,
        "lineage_preserved" => ScopePropagationGuardrail::LineagePreserved,
        other => panic!("unknown guardrail token {other}"),
    }
}

fn destination_from_block(block: &DestinationBlock) -> ScopePropagationDestination {
    match block.kind.as_str() {
        "exact" => ScopePropagationDestination::Exact {
            destination_label: block.destination_label.clone(),
        },
        "degraded" => ScopePropagationDestination::Degraded {
            destination_label: block.destination_label.clone(),
            reason: degraded_reason_from_token(
                block
                    .reason
                    .as_deref()
                    .expect("degraded destination must declare reason"),
            ),
            explain_note: block
                .explain_note
                .clone()
                .expect("degraded destination must declare explain_note"),
        },
        "blocked_by_outside_scope" => ScopePropagationDestination::BlockedByOutsideScope {
            destination_label: block.destination_label.clone(),
            explain_note: block
                .explain_note
                .clone()
                .expect("blocked destination must declare explain_note"),
        },
        "blocked_by_policy" => ScopePropagationDestination::BlockedByPolicy {
            destination_label: block.destination_label.clone(),
            explain_note: block
                .explain_note
                .clone()
                .expect("blocked destination must declare explain_note"),
        },
        "blocked_by_portability" => ScopePropagationDestination::BlockedByPortability {
            destination_label: block.destination_label.clone(),
            explain_note: block
                .explain_note
                .clone()
                .expect("blocked destination must declare explain_note"),
        },
        other => panic!("unknown destination kind {other}"),
    }
}

fn project_truth(fixture: &Fixture) -> WorksetScopeBetaTruth {
    let inputs = ScopeObservationInputs {
        workspace_root_refs: &fixture.workspace.workspace_root_refs,
        workspace_root_labels: &fixture.workspace.workspace_root_labels,
        parent_artifact: None,
    };
    let surface = surface_from_token(&fixture.source_consumer_surface);
    if let Some(outside_root) = fixture.meta.outside_root_ref.as_deref() {
        fixture.artifact.project_beta_truth_outside_scope(
            surface,
            inputs,
            outside_root,
            "Row outside current scope without a widen review.",
            "mono:propagation:test",
        )
    } else {
        fixture
            .artifact
            .project_beta_truth(surface, inputs, "mono:propagation:test")
    }
}

fn scope_class_token(scope_class: ScopeClass) -> &'static str {
    match scope_class {
        ScopeClass::CurrentRepo => "current_repo",
        ScopeClass::SelectedWorkset => "selected_workset",
        ScopeClass::SparseSlice => "sparse_slice",
        ScopeClass::FullWorkspace => "full_workspace",
        ScopeClass::PolicyLimitedView => "policy_limited_view",
    }
}

fn scope_mode_token(scope_mode: ScopeMode) -> &'static str {
    match scope_mode {
        ScopeMode::Full => "full",
        ScopeMode::Sparse => "sparse",
    }
}

#[test]
fn every_fixture_projects_a_valid_propagation_record() {
    for (path, fixture) in load_fixtures() {
        fixture
            .artifact
            .validate()
            .unwrap_or_else(|err| panic!("alpha artifact in {path:?} must validate: {err}"));
        let beta = project_truth(&fixture);
        beta.validate()
            .unwrap_or_else(|err| panic!("beta truth in {path:?} must validate: {err}"));

        let inputs = ScopePropagationProjectionInputs {
            propagation_id: format!("prop:{}", fixture.meta.name),
            crossing: crossing_from_token(&fixture.crossing),
            destination: destination_from_block(&fixture.destination),
            hidden_member_count: fixture.hidden_member_count,
            emitted_at: "mono:propagation:emit".to_string(),
        };
        let record = beta
            .project_scope_propagation(inputs)
            .unwrap_or_else(|err| panic!("propagation in {path:?} must project: {err}"));

        assert_eq!(
            record.workset_ref, fixture.artifact.workset_id,
            "propagation workset_ref mismatch in {path:?}",
        );
        assert_eq!(
            record.stable_scope_id,
            fixture.artifact.stable_scope_id(),
            "propagation stable_scope_id mismatch in {path:?}",
        );
        assert_eq!(
            scope_class_token(record.scope_class),
            fixture.expect.preserved_scope_class,
            "propagation scope_class mismatch in {path:?}",
        );
        assert_eq!(
            scope_mode_token(record.scope_mode),
            fixture.expect.preserved_scope_mode,
            "propagation scope_mode mismatch in {path:?}",
        );
        assert_eq!(
            record.disposition.as_str(),
            fixture.expect.disposition,
            "propagation disposition mismatch in {path:?}",
        );
        assert_eq!(
            record.completed(),
            fixture.expect.completed,
            "propagation completed mismatch in {path:?}",
        );
        assert_eq!(
            record.remote_attach_disclosed, fixture.expect.remote_attach_disclosed,
            "propagation remote_attach_disclosed mismatch in {path:?}",
        );
        assert_eq!(
            record.hidden_member_count_preserved, fixture.expect.hidden_member_count_preserved,
            "propagation hidden_member_count_preserved mismatch in {path:?}",
        );
        assert_eq!(
            record.lineage.len() as u32,
            fixture.expect.lineage_length,
            "propagation lineage length mismatch in {path:?}",
        );
        if let Some(expected_underlying) = fixture.expect.lineage_underlying_ref.as_deref() {
            assert!(
                record
                    .lineage
                    .iter()
                    .any(|entry| entry.workset_ref == expected_underlying),
                "propagation lineage must include {expected_underlying} in {path:?}",
            );
        }
        for token in &fixture.expect.guardrails_required {
            let guardrail = guardrail_from_token(token);
            assert!(
                record.guardrails_observed.contains(&guardrail),
                "propagation in {path:?} must observe guardrail {token}",
            );
        }

        // Preserved scope must match the beta truth verbatim.
        assert_eq!(
            record.preserved_included_roots, beta.included_roots,
            "propagation preserved_included_roots mismatch in {path:?}",
        );
        assert_eq!(
            record.preserved_excluded_roots, beta.excluded_roots,
            "propagation preserved_excluded_roots mismatch in {path:?}",
        );
        assert_eq!(
            record.preserved_include_patterns, beta.include_patterns,
            "propagation preserved_include_patterns mismatch in {path:?}",
        );
        assert_eq!(
            record.preserved_exclude_patterns, beta.exclude_patterns,
            "propagation preserved_exclude_patterns mismatch in {path:?}",
        );
        assert_eq!(
            record.lineage, beta.lineage,
            "propagation lineage mismatch in {path:?}",
        );

        // Round-trip through serde.
        let payload = serde_json::to_string(&record).expect("record must serialize");
        let parsed: ScopePropagationAlphaRecord =
            serde_json::from_str(&payload).expect("record must round-trip");
        assert_eq!(parsed, record, "record round-trip mismatch in {path:?}");
    }
}

#[test]
fn fixtures_cover_every_crossing_class() {
    let fixtures = load_fixtures();
    let mut covered = std::collections::BTreeSet::new();
    for (_, fixture) in &fixtures {
        covered.insert(crossing_from_token(&fixture.crossing));
    }
    for required in ScopePropagationCrossingClass::all() {
        assert!(
            covered.contains(&required),
            "missing fixture coverage for crossing class {}; got {covered:?}",
            required.as_str()
        );
    }
}

#[test]
fn fixtures_cover_exact_degraded_and_blocked_dispositions() {
    let fixtures = load_fixtures();
    let mut covered = std::collections::BTreeSet::new();
    for (_, fixture) in &fixtures {
        covered.insert(fixture.expect.disposition.clone());
    }
    for required in [
        "scope_labels_preserved_exact",
        "scope_labels_preserved_degraded",
        "blocked_by_outside_scope",
    ] {
        assert!(
            covered.contains(required),
            "missing fixture coverage for disposition {required}; got {covered:?}",
        );
    }
}

#[test]
fn fixture_names_are_unique() {
    let fixtures = load_fixtures();
    let mut names: Vec<String> = fixtures.iter().map(|(_, f)| f.meta.name.clone()).collect();
    names.sort();
    let unique = names.iter().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        names.len(),
        unique.len(),
        "fixture names must be unique; got {names:?}"
    );
}

#[test]
fn support_export_packet_bundles_every_crossing_for_the_same_artifact() {
    // Build a packet from the sparse / sparse-slice fixtures that all share
    // one artifact identity by projecting the same beta truth through every
    // crossing.
    let (_, fixture) = load_fixtures()
        .into_iter()
        .find(|(_, f)| f.meta.name == "remote_helper_attach_exact")
        .expect("baseline fixture required");
    let beta = project_truth(&fixture);
    let mut records = Vec::new();
    for (i, crossing) in ScopePropagationCrossingClass::all().into_iter().enumerate() {
        let inputs = ScopePropagationProjectionInputs {
            propagation_id: format!("prop:packet:{i}"),
            crossing,
            destination: ScopePropagationDestination::Exact {
                destination_label: format!("dest:packet:{}", crossing.as_str()),
            },
            hidden_member_count: None,
            emitted_at: format!("mono:packet:{i}"),
        };
        records.push(
            beta.project_scope_propagation(inputs)
                .expect("propagation must project for support export"),
        );
    }
    let packet = ScopePropagationAlphaSupportExport::from_propagations(records, "mono:packet:emit")
        .expect("support packet must bundle");
    for crossing in ScopePropagationCrossingClass::all() {
        assert!(packet.propagation_for(crossing).is_some());
    }
    let payload = serde_json::to_string(&packet).expect("packet must serialize");
    let parsed: ScopePropagationAlphaSupportExport =
        serde_json::from_str(&payload).expect("packet must round-trip");
    assert_eq!(parsed, packet);
}

#[test]
fn support_export_rejects_propagations_for_different_artifacts() {
    let fixtures = load_fixtures();
    let mut a = None;
    let mut b = None;
    for (_, f) in &fixtures {
        let beta = project_truth(f);
        let propagation = beta
            .project_scope_propagation(ScopePropagationProjectionInputs {
                propagation_id: format!("prop:mismatch:{}", f.meta.name),
                crossing: crossing_from_token(&f.crossing),
                destination: destination_from_block(&f.destination),
                hidden_member_count: f.hidden_member_count,
                emitted_at: "mono:propagation:mix".to_string(),
            })
            .expect("propagation must project");
        if a.is_none() {
            a = Some(propagation);
        } else if b.is_none() && propagation.workset_ref != a.as_ref().unwrap().workset_ref {
            b = Some(propagation);
            break;
        }
    }
    let a = a.expect("first propagation required");
    let b = b.expect("second propagation with different artifact required");
    let err = ScopePropagationAlphaSupportExport::from_propagations(vec![a, b], "mono:mix")
        .expect_err("mixed-artifact packet must fail");
    assert!(matches!(
        err,
        ScopePropagationAlphaError::ScopeLabelsMustMatchBetaTruth
    ));
}
