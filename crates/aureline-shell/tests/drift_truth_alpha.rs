use std::collections::BTreeSet;
use std::path::Path;

use aureline_runtime::{
    CompatibilityWindow, CompatibilityWindowStatus, EffectiveCapabilityPosture,
    HelperCapabilityResponse, NegotiationOutcome, HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
};
use aureline_shell::drift_truth::{
    helper_negotiation_refusal_row, DriftStateClass, DriftTruthExportAudience, DriftTruthSnapshot,
    DriftTruthSurfaceClass, DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND, DRIFT_TRUTH_ROW_RECORD_KIND,
};
use aureline_shell::support_seed::SupportSeedSurface;
use aureline_support::bundle::{DiagnosticDataClass, ExactBuildCapture, ReleaseChannelClass};

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:version-skew-alpha",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
}

fn load_snapshot() -> DriftTruthSnapshot {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/compat/version_skew_alpha/manifest.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn version_skew_alpha_fixture_projects_required_display_states() {
    let snapshot = load_snapshot();
    snapshot.validate().expect("snapshot validates");

    assert!(snapshot.has_alpha_state_coverage());
    assert!(snapshot.has_alpha_surface_coverage());

    let states = snapshot
        .display_rows()
        .into_iter()
        .map(|row| (row.surface_class, row.state_class, row.state_label))
        .collect::<BTreeSet<_>>();
    assert!(states.contains(&(
        DriftTruthSurfaceClass::HelperAgent,
        DriftStateClass::UnsupportedSkew,
        "Unsupported skew".to_owned(),
    )));
    assert!(states.contains(&(
        DriftTruthSurfaceClass::HelperAgent,
        DriftStateClass::RetryRequired,
        "Retry required".to_owned(),
    )));
    assert!(states.contains(&(
        DriftTruthSurfaceClass::ProviderSnapshot,
        DriftStateClass::StaleSnapshot,
        "Stale snapshot".to_owned(),
    )));
    assert!(states.contains(&(
        DriftTruthSurfaceClass::SavedArtifact,
        DriftStateClass::MigrationReviewNeeded,
        "Migration review needed".to_owned(),
    )));
}

#[test]
fn drift_truth_exports_support_and_review_packets_without_payloads() {
    let snapshot = load_snapshot();
    snapshot.validate().expect("snapshot validates");

    let support = snapshot.export_packet(DriftTruthExportAudience::Support);
    let review = snapshot.export_packet(DriftTruthExportAudience::Review);

    assert!(support.is_export_safe());
    assert!(review.is_export_safe());
    assert_eq!(support.rows.len(), snapshot.rows.len());
    assert_eq!(review.rows.len(), snapshot.rows.len());

    let provider_row = support
        .rows
        .iter()
        .find(|row| row.row_id == "drift_truth.provider_snapshot.cached_read_only_stale")
        .expect("provider stale row");
    assert_eq!(provider_row.state_class, DriftStateClass::StaleSnapshot);
    assert_eq!(
        provider_row.skew_case_ref,
        "skew_case:provider.cached_metadata_only"
    );
    assert!(provider_row
        .blocked_action_refs
        .iter()
        .any(|item| item.contains("provider")));

    let saved_row = review
        .rows
        .iter()
        .find(|row| row.row_id == "drift_truth.saved_artifact.support_bundle_manual_review")
        .expect("saved-artifact review row");
    assert_eq!(
        saved_row.state_class,
        DriftStateClass::MigrationReviewNeeded
    );
    assert!(!saved_row.preserved_artifact_refs.is_empty());
}

#[test]
fn support_seed_consumes_drift_truth_export_packet() {
    let snapshot = load_snapshot();
    snapshot.validate().expect("snapshot validates");

    let surface = SupportSeedSurface::drift_truth_preview(
        fixture_capture(),
        "2026-05-14T00:00:00Z",
        &snapshot,
    )
    .expect("support preview builds");

    assert!(surface.has_exact_build_identity());
    assert_eq!(surface.preview_row_count(), 3);

    let drift_row = surface
        .preview
        .manifest
        .preview_items
        .iter()
        .find(|item| {
            item.parity_binding.support_pack_item_id == "support.item.version_skew_drift_truth"
        })
        .expect("drift truth support row");

    assert_eq!(
        drift_row.file_section_identity.artifact_kind_class,
        DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND
    );
    assert_eq!(
        drift_row.redaction.data_class,
        DiagnosticDataClass::MetadataOnly
    );
    assert!(drift_row
        .file_section_identity
        .source_refs
        .iter()
        .any(|item| item == "docs/compat/version_skew_alpha.md"));
}

#[test]
fn refused_helper_negotiation_projects_typed_drift_truth_row() {
    let snapshot = load_snapshot();
    let fixture_row = snapshot
        .row_by_id("drift_truth.helper_agent.unsupported_required_feature")
        .expect("unsupported helper row fixture");
    let response = HelperCapabilityResponse {
        schema_version: HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
        request_id: fixture_row.surface_ref.clone(),
        row_id: "drift_truth.helper_agent.runtime_negotiation_refused".to_owned(),
        surface_ref: fixture_row.surface_ref.clone(),
        title: fixture_row.title.clone(),
        outcome: NegotiationOutcome::Refuse,
        selected_protocol_ref: fixture_row.skew_truth.skew_case_ref.clone(),
        negotiated_capabilities: Vec::new(),
        dropped_capabilities: Vec::new(),
        mutation_allowed: false,
        effective_posture: EffectiveCapabilityPosture::Blocked,
        visible_summary: fixture_row.visible_summary.clone(),
        safe_continuation: fixture_row.safe_continuation.clone(),
        primary_recovery_ref: fixture_row.repair_action_refs.first().cloned(),
        recovery_refs: fixture_row.repair_action_refs.clone(),
        blocked_action_refs: fixture_row.blocked_action_refs.clone(),
        preserved_read_only_refs: Vec::new(),
        retry_ref: None,
        support_packet_refs: fixture_row.support_packet_refs.clone(),
        review_packet_refs: fixture_row.review_packet_refs.clone(),
        source_refs: fixture_row.source_refs.clone(),
        client_manifest_digest: "digest:client-manifest-fixture".to_owned(),
        helper_manifest_digest: "digest:helper-manifest-fixture".to_owned(),
        compatibility_window: CompatibilityWindow {
            boundary_family: fixture_row.skew_truth.boundary_family.clone(),
            compatibility_row_ref: fixture_row.skew_truth.compatibility_row_ref.clone(),
            version_skew_register_ref: fixture_row.skew_truth.version_skew_register_ref.clone(),
            skew_case_ref: fixture_row.skew_truth.skew_case_ref.clone(),
            skew_window_declaration_ref: fixture_row.skew_truth.skew_window_declaration_ref.clone(),
            status: CompatibilityWindowStatus::Unsupported,
            selected_protocol_ref: fixture_row.skew_truth.skew_case_ref.clone(),
            source_refs: fixture_row.skew_truth.source_refs.clone(),
        },
    };

    let row = helper_negotiation_refusal_row(&response).expect("refused response yields row");
    row.validate().expect("projected row validates");

    assert_eq!(row.record_kind, DRIFT_TRUTH_ROW_RECORD_KIND);
    assert_eq!(row.state_class, DriftStateClass::UnsupportedSkew);
    assert!(row.repair_action_refs.contains(
        response
            .primary_recovery_ref
            .as_ref()
            .expect("recovery ref")
    ));
    assert!(!row.support_packet_refs.is_empty());
}
