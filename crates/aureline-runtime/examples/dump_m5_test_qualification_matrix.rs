//! Conformance dump for the M5 test-intelligence qualification matrix packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix::*;
use aureline_runtime::testing_identity::TestItemIdentityClass;
use aureline_runtime::tests::{FlakyVerdictState, ImportedCiProjectionClass};

const PACKET_ID: &str = "m5-test-qualification-matrix:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn proposal(
    proposal_id: &str,
    proposal_kind: ProposalKind,
    mutates: bool,
) -> TestGenerationProposalDescriptor {
    TestGenerationProposalDescriptor {
        proposal_id: proposal_id.to_owned(),
        proposal_kind,
        mutates_source_or_artifact: mutates,
        requires_preview_diff: true,
        requires_explicit_apply: true,
    }
}

fn selection_object(selection_object_id: &str) -> SelectionObjectDeclaration {
    SelectionObjectDeclaration {
        selection_object_id: selection_object_id.to_owned(),
        basis_token: "stable_node_id_set".to_owned(),
        portable_to_rerun: true,
        portable_to_cli: true,
        captures_display_name_only: false,
        survives_rediscovery: true,
    }
}

fn triage_packet(
    triage_packet_id: &str,
    quarantine_state: FlakyVerdictState,
) -> TriagePacketDeclaration {
    TriagePacketDeclaration {
        triage_packet_id: triage_packet_id.to_owned(),
        quarantine_state,
        quarantine_visible: true,
        quarantine_exportable: true,
        has_renewal_or_expiry: true,
        evidence_backed: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: TestIntelligenceSurface,
    label: &str,
    test_item_identity_class: Option<TestItemIdentityClass>,
    discovery_snapshot_class: Option<DiscoverySnapshotClass>,
    selection_object_class: Option<SelectionObjectClass>,
    session_attempt_class: Option<SessionAttemptClass>,
    verdict_projection_class: ImportedCiProjectionClass,
    quarantine_state: FlakyVerdictState,
    claimed: TestMatrixQualificationClass,
    proposal_descriptors: Vec<TestGenerationProposalDescriptor>,
) -> TestQualificationRow {
    TestQualificationRow {
        row_id: row_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        test_item_identity_class,
        discovery_snapshot_class,
        selection_object_class,
        session_attempt_class,
        verdict_projection_class,
        selection_object: selection_object(&format!("selection:{row_id}")),
        triage_packet: triage_packet(&format!("triage:{row_id}"), quarantine_state),
        proposal_descriptors,
        identity_independent_of_display_name: true,
        template_distinct_from_invocation: true,
        partial_discovery_stays_visible: true,
        imported_results_not_shown_as_local: true,
        quarantine_visible_and_exportable: true,
        proposals_use_preview_apply: true,
        claimed_qualification: claimed,
        effective_qualification: claimed,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[M5_TEST_QUALIFICATION_MATRIX_DOC_REF]),
    }
}

fn downgraded_support_export_row() -> TestQualificationRow {
    let mut export_row = row(
        "test-row:support-export:0001",
        TestIntelligenceSurface::SupportExportProjection,
        "Support/export projection of a test row whose session-attempt class is not yet identified",
        Some(TestItemIdentityClass::Stable),
        Some(DiscoverySnapshotClass::CompleteDiscovery),
        Some(SelectionObjectClass::DurableIdentitySelection),
        None,
        ImportedCiProjectionClass::NotImportedCi,
        FlakyVerdictState::StableAgain,
        TestMatrixQualificationClass::Beta,
        Vec::new(),
    );
    export_row.effective_qualification = TestMatrixQualificationClass::Held;
    export_row.downgrade_trigger = Some(TestMatrixDowngradeTrigger::UnidentifiedSessionAttempt);
    export_row.degraded_label = Some(
        "Session-attempt class not yet identified for this projected row; held below preview until a session plan and attempt lineage are published"
            .to_owned(),
    );
    export_row
}

fn rows() -> Vec<TestQualificationRow> {
    vec![
        row(
            "test-row:framework-test-explorer:0001",
            TestIntelligenceSurface::FrameworkTestExplorer,
            "Framework test explorer with durable item identity, complete discovery, and a local live session",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::CompleteDiscovery),
            Some(SelectionObjectClass::DurableIdentitySelection),
            Some(SessionAttemptClass::LocalLiveSession),
            ImportedCiProjectionClass::NotImportedCi,
            FlakyVerdictState::StableAgain,
            TestMatrixQualificationClass::Beta,
            Vec::new(),
        ),
        row(
            "test-row:notebook-test-cells:0001",
            TestIntelligenceSurface::NotebookTestCells,
            "Notebook test cells with partial-but-visible discovery and a durable cell-identity selection",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::PartialVisibleDiscovery),
            Some(SelectionObjectClass::DurableIdentitySelection),
            Some(SessionAttemptClass::LocalLiveSession),
            ImportedCiProjectionClass::NotImportedCi,
            FlakyVerdictState::StableAgain,
            TestMatrixQualificationClass::Preview,
            Vec::new(),
        ),
        row(
            "test-row:ai-test-generation:0001",
            TestIntelligenceSurface::AiTestGeneration,
            "AI test-generation surface whose generate/codemod proposals preview a diff and gate behind explicit apply",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::CompleteDiscovery),
            Some(SelectionObjectClass::DurableIdentitySelection),
            Some(SessionAttemptClass::LocalLiveSession),
            ImportedCiProjectionClass::NotImportedCi,
            FlakyVerdictState::StableAgain,
            TestMatrixQualificationClass::Beta,
            vec![
                proposal("ai.generate", ProposalKind::GenerateTest, true),
                proposal("ai.codemod", ProposalKind::ApplyCodemod, true),
            ],
        ),
        row(
            "test-row:review-test-panel:0001",
            TestIntelligenceSurface::ReviewTestPanel,
            "Review test panel reconciling local attempts with imported CI evidence over a query-matched selection",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::CompleteDiscovery),
            Some(SelectionObjectClass::QueryMatchedSelection),
            Some(SessionAttemptClass::MixedLocalImportedSession),
            ImportedCiProjectionClass::AuthoritativeImportedReadOnly,
            FlakyVerdictState::SuspectedFlaky,
            TestMatrixQualificationClass::Beta,
            Vec::new(),
        ),
        row(
            "test-row:ci-import-overlay:0001",
            TestIntelligenceSurface::CiImportOverlay,
            "Imported CI overlay with read-only item identity, provider-imported discovery, and a provider-scoped session",
            Some(TestItemIdentityClass::ImportedReadOnly),
            Some(DiscoverySnapshotClass::ProviderImportedDiscovery),
            Some(SelectionObjectClass::ProviderScopedSelection),
            Some(SessionAttemptClass::ImportedCiSession),
            ImportedCiProjectionClass::AuthoritativeImportedReadOnly,
            FlakyVerdictState::Unknown,
            TestMatrixQualificationClass::Beta,
            Vec::new(),
        ),
        row(
            "test-row:coverage-surface:0001",
            TestIntelligenceSurface::CoverageSurface,
            "Coverage surface with durable identity, complete discovery, and a rerun-last attempt lineage",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::CompleteDiscovery),
            Some(SelectionObjectClass::DurableIdentitySelection),
            Some(SessionAttemptClass::RerunAttemptLineage),
            ImportedCiProjectionClass::FreshLocalReconfirmation,
            FlakyVerdictState::StableAgain,
            TestMatrixQualificationClass::Stable,
            Vec::new(),
        ),
        row(
            "test-row:flaky-quarantine-board:0001",
            TestIntelligenceSurface::FlakyQuarantineBoard,
            "Flaky/quarantine board keeping a muted test visible, filterable, and exportable with renewal/expiry semantics",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::CompleteDiscovery),
            Some(SelectionObjectClass::DurableIdentitySelection),
            Some(SessionAttemptClass::RerunAttemptLineage),
            ImportedCiProjectionClass::NotImportedCi,
            FlakyVerdictState::Muted,
            TestMatrixQualificationClass::Beta,
            Vec::new(),
        ),
        row(
            "test-row:snapshot-golden-review:0001",
            TestIntelligenceSurface::SnapshotGoldenReview,
            "Snapshot/golden review whose accept-snapshot and update-golden proposals preview a diff before explicit apply",
            Some(TestItemIdentityClass::Stable),
            Some(DiscoverySnapshotClass::CompleteDiscovery),
            Some(SelectionObjectClass::DurableIdentitySelection),
            Some(SessionAttemptClass::LocalLiveSession),
            ImportedCiProjectionClass::NotImportedCi,
            FlakyVerdictState::StableAgain,
            TestMatrixQualificationClass::Beta,
            vec![
                proposal("snapshot.accept", ProposalKind::AcceptSnapshot, true),
                proposal("golden.update", ProposalKind::UpdateGolden, true),
            ],
        ),
        downgraded_support_export_row(),
    ]
}

fn guardrails() -> TestMatrixGuardrails {
    TestMatrixGuardrails {
        display_labels_never_substitute_test_identity: true,
        parameterized_templates_distinct_from_invocations: true,
        partial_discovery_stays_visible: true,
        imported_results_never_masquerade_as_local: true,
        quarantines_visible_filterable_exportable: true,
        proposals_use_preview_diff_apply: true,
        rows_auto_downgrade_on_unidentified_objects: true,
    }
}

fn consumer_projection() -> TestMatrixConsumerProjection {
    TestMatrixConsumerProjection {
        product_ingests_matrix: true,
        docs_help_ingests_matrix: true,
        diagnostics_ingests_matrix: true,
        ai_review_ingests_matrix: true,
        release_control_ingests_matrix: true,
        downgraded_rows_labeled_below_current: true,
    }
}

fn evidence_freshness() -> TestMatrixEvidenceFreshness {
    TestMatrixEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_downgrade_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_TEST_QUALIFICATION_MATRIX_SCHEMA_REF,
        M5_TEST_QUALIFICATION_MATRIX_DOC_REF,
        M5_TEST_QUALIFICATION_MATRIX_ARTIFACT_REF,
        "schemas/testing/test_item_identity.schema.json",
        "schemas/testing/test_session.schema.json",
        "schemas/testing/test_attempt.schema.json",
        "schemas/testing/test_quarantine_record.schema.json",
        "schemas/testing/ai_test_generation_gate.schema.json",
    ])
}

fn packet() -> TestQualificationMatrixPacket {
    TestQualificationMatrixPacket::new(TestQualificationMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Test-Intelligence Qualification Matrix".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
