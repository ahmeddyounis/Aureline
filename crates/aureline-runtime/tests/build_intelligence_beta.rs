//! Integration tests for build-intelligence health, receipts, and diffs.
//!
//! The fixture-backed path proves the runtime model keeps every discovery
//! lane, adapter-health reason, imported-versus-live receipt, and refresh-diff
//! bucket visible through one support-export packet.

use std::path::PathBuf;

use aureline_runtime::{
    AdapterHealthReason, AdapterHealthState, AdapterHealthStrip, AdapterIdentity,
    ArtifactSourceClass, BuildIntelligenceAction, BuildIntelligenceActionClass,
    BuildIntelligenceCoverageManifest, BuildIntelligenceLaneType, BuildIntelligenceReceipt,
    BuildIntelligenceRunConfigCard, BuildIntelligenceSupportExport, BuildIntelligenceTargetRow,
    DiscoveryDiffReview, HighTrustActionPosture, ImportedLiveState, RefreshLineage,
    TargetExactnessStatus, BUILD_INTELLIGENCE_COVERAGE_MANIFEST_RECORD_KIND,
    BUILD_INTELLIGENCE_SCHEMA_VERSION, BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/m3/build_intelligence_confidence")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()))
}

#[derive(Debug, Deserialize)]
struct BuildIntelligenceCase {
    record_kind: String,
    schema_version: u32,
    generated_at: String,
    support_export_id: String,
    workspace_id: String,
    expect: ExpectBlock,
}

#[derive(Debug, Deserialize)]
struct ExpectBlock {
    lane_type_tokens: Vec<String>,
    diff_counts: DiffCounts,
    non_live_receipt_postures: Vec<String>,
    required_health_reasons: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DiffCounts {
    added: usize,
    removed: usize,
    renamed: usize,
    downgraded_confidence: usize,
    newly_heuristic: usize,
    newly_exact: usize,
    now_unresolved: usize,
}

fn action(class: BuildIntelligenceActionClass, suffix: &str) -> BuildIntelligenceAction {
    BuildIntelligenceAction::enabled(class, format!("action:build-intelligence:{suffix}"))
}

fn lineage(
    refresh: &str,
    previous: Option<&str>,
    live_ref: Option<&str>,
    import_ref: Option<&str>,
) -> RefreshLineage {
    RefreshLineage::new(
        refresh.to_owned(),
        previous.map(str::to_owned),
        "2026-05-18T15:00:00Z",
        Some("2026-05-18T15:00:05Z".to_owned()),
    )
    .with_refs(
        Some(format!("snapshot:{refresh}")),
        Some(format!("raw:{refresh}")),
        live_ref.map(str::to_owned),
        import_ref.map(str::to_owned),
    )
}

#[allow(clippy::too_many_arguments)]
fn strip(
    id: &str,
    lane: BuildIntelligenceLaneType,
    state: AdapterHealthState,
    reason: Option<AdapterHealthReason>,
    provenance: ImportedLiveState,
    live_ref: Option<&str>,
    import_ref: Option<&str>,
) -> AdapterHealthStrip {
    let protocol_id = match lane {
        BuildIntelligenceLaneType::StructuredProtocol => Some("bsp".to_owned()),
        BuildIntelligenceLaneType::BuildEventStream => Some("bep".to_owned()),
        _ => None,
    };
    AdapterHealthStrip::new(
        format!("strip:{id}"),
        "workspace:build-intelligence-fixture",
        lane,
        AdapterIdentity::new(
            format!("adapter:{id}"),
            format!("{id} adapter"),
            Some("1.0".to_owned()),
            protocol_id,
            Some("1".to_owned()),
        ),
        state,
        reason,
        Some("2026-05-18T15:00:05Z".to_owned()),
        provenance,
        lineage(
            "refresh:current",
            Some("refresh:previous"),
            live_ref,
            import_ref,
        ),
        action(
            BuildIntelligenceActionClass::RefreshDiscovery,
            &format!("{id}:refresh"),
        ),
        action(
            BuildIntelligenceActionClass::OpenDetails,
            &format!("{id}:details"),
        ),
    )
}

#[allow(clippy::too_many_arguments)]
fn target(
    refresh: &str,
    id: &str,
    name: &str,
    lane: BuildIntelligenceLaneType,
    exactness: TargetExactnessStatus,
    strip_ref: &str,
    provenance: ImportedLiveState,
    note: &str,
) -> BuildIntelligenceTargetRow {
    let live_ref = matches!(
        provenance,
        ImportedLiveState::LiveWorkspaceInspection | ImportedLiveState::MixedLiveAndImported
    )
    .then_some("inspection:current");
    let import_ref = matches!(
        provenance,
        ImportedLiveState::ImportedArtifact
            | ImportedLiveState::ReplayedReceipt
            | ImportedLiveState::MixedLiveAndImported
    )
    .then_some("artifact:imported");
    BuildIntelligenceTargetRow::new(
        format!("row:{id}:{refresh}"),
        id,
        name,
        lane,
        strip_ref,
        exactness,
        provenance,
        note,
        lineage(refresh, Some("refresh:previous"), live_ref, import_ref),
    )
    .with_bindings(
        Some("typescript_web_app".to_owned()),
        Some("vite".to_owned()),
    )
    .with_actions(
        Some(action(
            BuildIntelligenceActionClass::OpenSource,
            &format!("{id}:source"),
        )),
        Some(action(
            BuildIntelligenceActionClass::OpenConfig,
            &format!("{id}:config"),
        )),
    )
}

#[test]
fn fixture_all_lanes_refresh_diff_replays_end_to_end() {
    let payload = fixture("all_lanes_refresh_diff.json");
    let case: BuildIntelligenceCase = serde_json::from_str(&payload).expect("parse fixture");
    assert_eq!(case.record_kind, "build_intelligence_beta_case");
    assert_eq!(case.schema_version, BUILD_INTELLIGENCE_SCHEMA_VERSION);

    let native_strip = strip(
        "native",
        BuildIntelligenceLaneType::NativeAdapter,
        AdapterHealthState::Healthy,
        None,
        ImportedLiveState::LiveWorkspaceInspection,
        Some("inspection:native"),
        None,
    );
    let protocol_strip = strip(
        "protocol",
        BuildIntelligenceLaneType::StructuredProtocol,
        AdapterHealthState::Partial,
        Some(AdapterHealthReason::VersionSkew),
        ImportedLiveState::MixedLiveAndImported,
        Some("inspection:protocol"),
        Some("artifact:protocol:previous"),
    )
    .with_continuation_actions(
        Some(action(
            BuildIntelligenceActionClass::ContinueLocal,
            "protocol:continue-local",
        )),
        Some(action(
            BuildIntelligenceActionClass::InspectOnly,
            "protocol:inspect-only",
        )),
    );
    let event_strip = strip(
        "event",
        BuildIntelligenceLaneType::BuildEventStream,
        AdapterHealthState::ImportedOnly,
        Some(AdapterHealthReason::ControlPlaneOutage),
        ImportedLiveState::ReplayedReceipt,
        None,
        Some("receipt:bep:51"),
    );
    let import_strip = strip(
        "import",
        BuildIntelligenceLaneType::StructuredOutputImport,
        AdapterHealthState::ImportedOnly,
        Some(AdapterHealthReason::StaleArtifact),
        ImportedLiveState::ImportedArtifact,
        None,
        Some("artifact:junit:51"),
    );
    let heuristic_strip = strip(
        "heuristic",
        BuildIntelligenceLaneType::HeuristicFallback,
        AdapterHealthState::Degraded,
        Some(AdapterHealthReason::ParseAmbiguity),
        ImportedLiveState::HeuristicInference,
        Some("inspection:heuristic"),
        None,
    );

    let previous_rows = vec![
        target(
            "refresh:previous",
            "target:web",
            "web",
            BuildIntelligenceLaneType::NativeAdapter,
            TargetExactnessStatus::Exact,
            &native_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "live native adapter target",
        ),
        target(
            "refresh:previous",
            "target:api",
            "api",
            BuildIntelligenceLaneType::StructuredProtocol,
            TargetExactnessStatus::ProtocolBacked,
            &protocol_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "protocol-backed target",
        ),
        target(
            "refresh:previous",
            "target:legacy",
            "legacy",
            BuildIntelligenceLaneType::HeuristicFallback,
            TargetExactnessStatus::Heuristic,
            &heuristic_strip.strip_id,
            ImportedLiveState::HeuristicInference,
            "heuristic target before adapter support landed",
        ),
        target(
            "refresh:previous",
            "target:removed",
            "removed",
            BuildIntelligenceLaneType::StructuredProtocol,
            TargetExactnessStatus::ProtocolBacked,
            &protocol_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "removed after refresh",
        ),
    ];
    let current_rows = vec![
        target(
            "refresh:current",
            "target:web",
            "web-test",
            BuildIntelligenceLaneType::HeuristicFallback,
            TargetExactnessStatus::Heuristic,
            &heuristic_strip.strip_id,
            ImportedLiveState::HeuristicInference,
            "heuristic fallback; review before rerun",
        ),
        target(
            "refresh:current",
            "target:api",
            "api",
            BuildIntelligenceLaneType::StructuredProtocol,
            TargetExactnessStatus::Unresolved,
            &protocol_strip.strip_id,
            ImportedLiveState::MixedLiveAndImported,
            "protocol target unresolved after version skew",
        )
        .with_unresolved_reason(AdapterHealthReason::VersionSkew),
        target(
            "refresh:current",
            "target:legacy",
            "legacy",
            BuildIntelligenceLaneType::NativeAdapter,
            TargetExactnessStatus::Exact,
            &native_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "now exact through native adapter",
        ),
        target(
            "refresh:current",
            "target:bep",
            "bep imported test",
            BuildIntelligenceLaneType::BuildEventStream,
            TargetExactnessStatus::Imported,
            &event_strip.strip_id,
            ImportedLiveState::ReplayedReceipt,
            "replayed build-event stream; inspect only until live refresh",
        ),
        target(
            "refresh:current",
            "target:junit",
            "junit import",
            BuildIntelligenceLaneType::StructuredOutputImport,
            TargetExactnessStatus::Imported,
            &import_strip.strip_id,
            ImportedLiveState::ImportedArtifact,
            "structured output import; refresh before rerun",
        ),
    ];

    let cards = current_rows
        .iter()
        .map(|row| {
            let posture = match row.imported_live_state {
                ImportedLiveState::LiveWorkspaceInspection => {
                    HighTrustActionPosture::LiveActionsAllowed
                }
                ImportedLiveState::MixedLiveAndImported => {
                    HighTrustActionPosture::ContinueLocalAvailable
                }
                ImportedLiveState::ImportedArtifact => HighTrustActionPosture::RefreshRequired,
                ImportedLiveState::ReplayedReceipt => HighTrustActionPosture::InspectOnly,
                ImportedLiveState::HeuristicInference => {
                    HighTrustActionPosture::ReviewBeforeDispatch
                }
            };
            BuildIntelligenceRunConfigCard::from_target_row(
                format!("card:{}", row.stable_target_id),
                "task.run",
                row,
                posture,
            )
        })
        .collect::<Vec<_>>();
    let receipts = vec![
        BuildIntelligenceReceipt::from_target_row(
            "receipt:legacy",
            "task.run",
            "run:legacy:1",
            &current_rows[2],
            &native_strip,
            "local/macos",
            ArtifactSourceClass::LiveAdapter,
            Some("artifact:legacy:1".to_owned()),
            "live native adapter result",
            HighTrustActionPosture::LiveActionsAllowed,
        ),
        BuildIntelligenceReceipt::from_target_row(
            "receipt:bep",
            "test.run",
            "run:bep:51",
            &current_rows[3],
            &event_strip,
            "ci/linux",
            ArtifactSourceClass::ReplayedReceipt,
            Some("artifact:bep:51".to_owned()),
            "replayed build-event stream; not current live discovery",
            HighTrustActionPosture::InspectOnly,
        ),
        BuildIntelligenceReceipt::from_target_row(
            "receipt:junit",
            "test.run",
            "run:junit:51",
            &current_rows[4],
            &import_strip,
            "ci/linux",
            ArtifactSourceClass::StructuredImport,
            Some("artifact:junit:51".to_owned()),
            "imported structured output; refresh required before rerun",
            HighTrustActionPosture::RefreshRequired,
        ),
    ];
    let diff = DiscoveryDiffReview::between(
        "diff:all-lanes",
        case.workspace_id.clone(),
        "refresh:previous",
        "refresh:current",
        "2026-05-18T15:01:00Z",
        &previous_rows,
        &current_rows,
    );
    let export = BuildIntelligenceSupportExport::new(
        case.support_export_id.clone(),
        case.workspace_id.clone(),
        case.generated_at.clone(),
        vec![
            native_strip,
            protocol_strip,
            event_strip,
            import_strip,
            heuristic_strip,
        ],
        current_rows,
        cards,
        receipts,
        vec![diff],
    );

    assert_eq!(
        export.record_kind,
        BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(
        export.coverage_manifest.record_kind,
        BUILD_INTELLIGENCE_COVERAGE_MANIFEST_RECORD_KIND
    );
    let canonical_manifest = BuildIntelligenceCoverageManifest::canonical(
        export.coverage_manifest.manifest_id.clone(),
        export.coverage_manifest.generated_at.clone(),
    );
    assert_eq!(canonical_manifest, export.coverage_manifest);
    assert_eq!(
        export.coverage_manifest.lane_type_tokens,
        case.expect.lane_type_tokens
    );

    let diff = export.discovery_diffs.first().expect("diff");
    assert_eq!(diff.added.len(), case.expect.diff_counts.added);
    assert_eq!(diff.removed.len(), case.expect.diff_counts.removed);
    assert_eq!(diff.renamed.len(), case.expect.diff_counts.renamed);
    assert_eq!(
        diff.downgraded_confidence.len(),
        case.expect.diff_counts.downgraded_confidence
    );
    assert_eq!(
        diff.newly_heuristic.len(),
        case.expect.diff_counts.newly_heuristic
    );
    assert_eq!(diff.newly_exact.len(), case.expect.diff_counts.newly_exact);
    assert_eq!(
        diff.now_unresolved.len(),
        case.expect.diff_counts.now_unresolved
    );

    for expected_reason in &case.expect.required_health_reasons {
        assert!(
            export.adapter_health_strips.iter().any(|strip| {
                strip.health_reason_token.as_deref() == Some(expected_reason.as_str())
            }),
            "missing health reason {expected_reason}"
        );
    }
    for expected_posture in &case.expect.non_live_receipt_postures {
        assert!(
            export.receipts.iter().any(|receipt| {
                receipt.high_trust_action_posture_token == *expected_posture
                    && receipt.imported_live_state != ImportedLiveState::LiveWorkspaceInspection
            }),
            "missing non-live receipt posture {expected_posture}"
        );
    }

    let plaintext = export.render_plaintext();
    for token in &case.expect.lane_type_tokens {
        assert!(plaintext.contains(token), "missing lane token {token}");
    }
    assert!(plaintext.contains("provenance=replayed_receipt"));
    assert!(plaintext.contains("provenance=imported_artifact"));
    assert!(plaintext.contains("posture=inspect_only"));
    assert!(plaintext.contains("posture=refresh_required"));
    assert!(plaintext.contains("newly_heuristic=1"));
    assert!(plaintext.contains("newly_exact=1"));
    assert!(!plaintext.contains("/Users/"));
}
