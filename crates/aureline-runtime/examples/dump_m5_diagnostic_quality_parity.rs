//! Conformance dump for the M5 diagnostic-quality snapshot and
//! imported-versus-live delta packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::diagnostics::{
    DiagnosticFreshnessClass, DiagnosticOriginClass, DiagnosticSourceKind,
};
use aureline_runtime::m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::*;
use aureline_runtime::m5_diagnostic_source_descriptors_and_collection_snapshots::DiagnosticCollectionScope;
use aureline_runtime::quality::QualityTargetScopeClass;

const PACKET_ID: &str = "m5-diagnostic-quality-parity:stable:0001";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";
const WORKSPACE_REF: &str = "workspace:primary";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn scope(
    scope_class: QualityTargetScopeClass,
    workset: Option<&str>,
    target: Option<&str>,
    profile: &str,
) -> DiagnosticCollectionScope {
    DiagnosticCollectionScope {
        scope_class,
        workspace_ref: WORKSPACE_REF.to_owned(),
        workset_ref: workset.map(str::to_owned),
        target_or_environment_ref: target.map(str::to_owned),
        active_profile_ref: Some(profile.to_owned()),
    }
}

fn tool_version(source_kind: DiagnosticSourceKind, family: &str) -> QualityToolVersionRow {
    QualityToolVersionRow {
        source_kind,
        tool_ref: format!("tool:{family}"),
        tool_version: format!("{family}:1.4.2"),
        rule_pack_ref: format!("rule-pack:{family}"),
        rule_pack_version: format!("{family}-rules:2026.06.0"),
        adapter_ref: Some(format!("adapter:{family}")),
        summary: format!("{family} analyzer and rule pack in force for this snapshot."),
    }
}

fn save_outcome(
    family: &str,
    action: &str,
    outcome: SaveParticipantOutcomeClass,
    preview_first: bool,
    apply_blocked: bool,
) -> SaveParticipantOutcomeRow {
    SaveParticipantOutcomeRow {
        participant_ref: format!("participant:{family}:{action}"),
        proposal_ref: format!("proposal:{family}:{action}:0001"),
        action_token: action.to_owned(),
        outcome_class: outcome,
        preview_first_required: preview_first,
        apply_blocked,
        observed_at: MINTED_AT.to_owned(),
        summary: format!("Last {action} save-participant outcome for the {family} lane."),
    }
}

fn snapshot_entries() -> Vec<DiagnosticQualitySnapshotEntry> {
    // 1. Language-service snapshot: live local, current, durable, stable.
    let language_provider = DiagnosticQualitySnapshot::new(DiagnosticQualitySnapshotInput {
        snapshot_id: "snapshot:m5:language-provider:0001".to_owned(),
        snapshot_label: "Language-service quality snapshot across the workspace".to_owned(),
        scope: scope(
            QualityTargetScopeClass::Workspace,
            None,
            None,
            "profile:default",
        ),
        origin_class: DiagnosticOriginClass::LiveLocalSession,
        freshness_class: DiagnosticFreshnessClass::Current,
        captured_at: MINTED_AT.to_owned(),
        active_profile_ref: "profile:default".to_owned(),
        profile_fingerprint: "fingerprint:default:9f1c".to_owned(),
        tool_versions: vec![tool_version(
            DiagnosticSourceKind::LanguageService,
            "language_service",
        )],
        recent_collection_refs: refs(&["snapshot:m5:language-provider:collection:0001"]),
        suppression_refs: Vec::new(),
        baseline_refs: Vec::new(),
        release_visible_debt_count: 0,
        imported_scanner_session_refs: Vec::new(),
        save_participant_outcomes: vec![save_outcome(
            "language_service",
            "organize_imports",
            SaveParticipantOutcomeClass::PreviewedNotApplied,
            true,
            false,
        )],
        source_descriptor_refs: refs(&["source:language_service"]),
        imported_not_shown_as_live: true,
        export_safe_summary: "Whole-workspace language-service quality state, current and live."
            .to_owned(),
    });

    // 2. Runtime/test snapshot: live local, recent, durable, beta, with debt.
    let runtime_test = DiagnosticQualitySnapshot::new(DiagnosticQualitySnapshotInput {
        snapshot_id: "snapshot:m5:runtime-test:0001".to_owned(),
        snapshot_label: "Runtime/test quality snapshot for the selected workset".to_owned(),
        scope: scope(
            QualityTargetScopeClass::SelectedWorkset,
            Some("workset:test-suite"),
            None,
            "profile:default",
        ),
        origin_class: DiagnosticOriginClass::LiveLocalSession,
        freshness_class: DiagnosticFreshnessClass::Recent,
        captured_at: MINTED_AT.to_owned(),
        active_profile_ref: "profile:default".to_owned(),
        profile_fingerprint: "fingerprint:default:9f1c".to_owned(),
        tool_versions: vec![tool_version(
            DiagnosticSourceKind::RuntimeOrTest,
            "runtime_or_test",
        )],
        recent_collection_refs: refs(&["snapshot:m5:runtime-test:collection:0001"]),
        suppression_refs: refs(&["suppression:flaky-rule:0001"]),
        baseline_refs: Vec::new(),
        release_visible_debt_count: 2,
        imported_scanner_session_refs: Vec::new(),
        save_participant_outcomes: vec![save_outcome(
            "runtime_or_test",
            "fix_all",
            SaveParticipantOutcomeClass::AppliedWithFollowups,
            true,
            false,
        )],
        source_descriptor_refs: refs(&["source:runtime_or_test"]),
        imported_not_shown_as_live: true,
        export_safe_summary: "Runtime/test quality state for the test workset, recent and live."
            .to_owned(),
    });

    // 3. Imported-scanner snapshot: imported snapshot held read-only, durable, beta.
    let imported_scanner = DiagnosticQualitySnapshot::new(DiagnosticQualitySnapshotInput {
        snapshot_id: "snapshot:m5:imported-scanner:0001".to_owned(),
        snapshot_label: "Imported SARIF/scanner quality snapshot held read-only".to_owned(),
        scope: scope(
            QualityTargetScopeClass::Workspace,
            None,
            Some("target:ci-import"),
            "profile:default",
        ),
        origin_class: DiagnosticOriginClass::ImportedSnapshot,
        freshness_class: DiagnosticFreshnessClass::ImportedSnapshot,
        captured_at: MINTED_AT.to_owned(),
        active_profile_ref: "profile:default".to_owned(),
        profile_fingerprint: "fingerprint:default:9f1c".to_owned(),
        tool_versions: vec![tool_version(
            DiagnosticSourceKind::ScannerImport,
            "scanner_import",
        )],
        recent_collection_refs: refs(&["snapshot:m5:imported-scanner:collection:0001"]),
        suppression_refs: refs(&["suppression:imported-waiver:0001"]),
        baseline_refs: refs(&["baseline:ci-family:0001"]),
        release_visible_debt_count: 3,
        imported_scanner_session_refs: refs(&["import-session:ci-sarif:0001"]),
        save_participant_outcomes: vec![save_outcome(
            "scanner_import",
            "import_compare",
            SaveParticipantOutcomeClass::Skipped,
            false,
            true,
        )],
        source_descriptor_refs: refs(&["source:scanner_import"]),
        imported_not_shown_as_live: true,
        export_safe_summary:
            "Imported CI scanner quality state, held read-only and never shown as live local truth."
                .to_owned(),
    });

    // 4. Stale CI-import snapshot: imported, stale freshness, auto-downgraded.
    let ci_import_stale = DiagnosticQualitySnapshot::new(DiagnosticQualitySnapshotInput {
        snapshot_id: "snapshot:m5:ci-import:0007".to_owned(),
        snapshot_label: "Nightly CI imported scan predating the current rule-pack epoch".to_owned(),
        scope: scope(
            QualityTargetScopeClass::BaselineFamily,
            None,
            Some("target:nightly-ci"),
            "profile:ci-legacy",
        ),
        origin_class: DiagnosticOriginClass::ImportedSnapshot,
        freshness_class: DiagnosticFreshnessClass::Stale,
        captured_at: MINTED_AT.to_owned(),
        active_profile_ref: "profile:ci-legacy".to_owned(),
        profile_fingerprint: "fingerprint:ci-legacy:41ab".to_owned(),
        tool_versions: vec![tool_version(
            DiagnosticSourceKind::ScannerImport,
            "scanner_import",
        )],
        recent_collection_refs: refs(&["snapshot:m5:ci-import:collection:0007"]),
        suppression_refs: refs(&["suppression:ci-legacy:0007"]),
        baseline_refs: refs(&["baseline:ci-family:0006"]),
        release_visible_debt_count: 5,
        imported_scanner_session_refs: refs(&["import-session:nightly-ci:0007"]),
        save_participant_outcomes: vec![save_outcome(
            "scanner_import",
            "import_compare",
            SaveParticipantOutcomeClass::Skipped,
            false,
            true,
        )],
        source_descriptor_refs: refs(&["source:scanner_import"]),
        imported_not_shown_as_live: true,
        export_safe_summary:
            "Nightly CI scanner quality state, imported and stale against the current epoch."
                .to_owned(),
    });

    let downgraded = DiagnosticQualitySnapshotEntry {
        entry_id: "entry:snapshot:m5:ci-import:0007".to_owned(),
        snapshot: ci_import_stale,
        claimed_qualification: DiagnosticQualitySnapshotQualificationClass::Beta,
        effective_qualification: DiagnosticQualitySnapshotQualificationClass::Held,
        downgrade_trigger: Some(DiagnosticQualitySnapshotDowngradeTrigger::StaleGovernanceState),
        degraded_label: Some(
            "The imported CI scan predates the current rule-pack epoch and is held below preview until a fresh import or local rerun re-establishes current governance state"
                .to_owned(),
        ),
        evidence_refs: refs(&["evidence:snapshot:m5:ci-import:0007"]),
    };
    debug_assert!(downgraded.needs_downgrade());
    debug_assert!(downgraded.downgrade_consistent());

    vec![
        DiagnosticQualitySnapshotEntry {
            entry_id: "entry:snapshot:m5:language-provider:0001".to_owned(),
            snapshot: language_provider,
            claimed_qualification: DiagnosticQualitySnapshotQualificationClass::Stable,
            effective_qualification: DiagnosticQualitySnapshotQualificationClass::Stable,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: refs(&["evidence:snapshot:m5:language-provider:0001"]),
        },
        DiagnosticQualitySnapshotEntry {
            entry_id: "entry:snapshot:m5:runtime-test:0001".to_owned(),
            snapshot: runtime_test,
            claimed_qualification: DiagnosticQualitySnapshotQualificationClass::Beta,
            effective_qualification: DiagnosticQualitySnapshotQualificationClass::Beta,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: refs(&["evidence:snapshot:m5:runtime-test:0001"]),
        },
        DiagnosticQualitySnapshotEntry {
            entry_id: "entry:snapshot:m5:imported-scanner:0001".to_owned(),
            snapshot: imported_scanner,
            claimed_qualification: DiagnosticQualitySnapshotQualificationClass::Beta,
            effective_qualification: DiagnosticQualitySnapshotQualificationClass::Beta,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: refs(&["evidence:snapshot:m5:imported-scanner:0001"]),
        },
        downgraded,
    ]
}

#[allow(clippy::too_many_arguments)]
fn side(
    side_label: &str,
    origin_class: DiagnosticOriginClass,
    freshness_class: DiagnosticFreshnessClass,
    source_kind: DiagnosticSourceKind,
    snapshot_ref: &str,
    collection_ref: &str,
    profile: &str,
    tool_version_refs: &[&str],
) -> DiagnosticDeltaSide {
    DiagnosticDeltaSide {
        side_label: side_label.to_owned(),
        origin_class,
        freshness_class,
        source_kind,
        snapshot_ref: snapshot_ref.to_owned(),
        collection_ref: collection_ref.to_owned(),
        active_profile_ref: profile.to_owned(),
        tool_version_refs: refs(tool_version_refs),
        summary: format!("{side_label}."),
    }
}

fn note(
    note_class: DiagnosticDeltaCompatibilityNoteClass,
    summary: &str,
) -> DiagnosticDeltaCompatibilityNote {
    DiagnosticDeltaCompatibilityNote {
        note_class,
        summary: summary.to_owned(),
    }
}

fn finding_delta(
    finding_ref: &str,
    delta_state: DiagnosticFindingDeltaState,
    base_present: bool,
    compare_present: bool,
    comparable: bool,
    summary: &str,
) -> DiagnosticFindingDelta {
    DiagnosticFindingDelta {
        finding_ref: finding_ref.to_owned(),
        delta_state,
        base_present,
        compare_present,
        comparable,
        summary: summary.to_owned(),
    }
}

fn delta_packets() -> Vec<DiagnosticDeltaPacket> {
    // A. Imported vs live rerun: comparable once the imported side is confirmed.
    let imported_vs_live = DiagnosticDeltaPacket::new(DiagnosticDeltaPacketInput {
        delta_id: "delta:imported-vs-live:0001".to_owned(),
        delta_label: "Imported SARIF scan versus a live local rerun".to_owned(),
        comparison_basis_class: DiagnosticDeltaComparisonBasisClass::ImportedVsLiveRerun,
        base_side: side(
            "Imported SARIF scanner snapshot",
            DiagnosticOriginClass::ImportedSnapshot,
            DiagnosticFreshnessClass::ImportedSnapshot,
            DiagnosticSourceKind::ScannerImport,
            "snapshot:m5:imported-scanner:0001",
            "snapshot:m5:imported-scanner:collection:0001",
            "profile:default",
            &["tool:scanner_import"],
        ),
        compare_side: side(
            "Live local language-service rerun",
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticFreshnessClass::Current,
            DiagnosticSourceKind::LanguageService,
            "snapshot:m5:language-provider:0001",
            "snapshot:m5:language-provider:collection:0001",
            "profile:default",
            &["tool:language_service"],
        ),
        compatibility_class: DiagnosticDeltaCompatibilityClass::CompatibleWithLocalConfirmation,
        compatibility_notes: vec![note(
            DiagnosticDeltaCompatibilityNoteClass::FreshnessSkew,
            "The imported side is a static snapshot; matches must be locally confirmed before the delta is treated as exact.",
        )],
        delta_counts: DiagnosticDeltaCounts {
            added: 1,
            resolved: 1,
            persisting: 2,
            suppressed_or_waived: 1,
            unmapped: 0,
        },
        finding_deltas: vec![
            finding_delta(
                "diagnostic:rule-a:0001",
                DiagnosticFindingDeltaState::Persisting,
                true,
                true,
                true,
                "Present on both the imported snapshot and the live rerun.",
            ),
            finding_delta(
                "diagnostic:rule-a:0002",
                DiagnosticFindingDeltaState::Persisting,
                true,
                true,
                true,
                "Present on both sides at a remapped anchor.",
            ),
            finding_delta(
                "diagnostic:rule-b:0003",
                DiagnosticFindingDeltaState::Added,
                false,
                true,
                true,
                "Surfaced only by the live local rerun.",
            ),
            finding_delta(
                "diagnostic:rule-c:0004",
                DiagnosticFindingDeltaState::Resolved,
                true,
                false,
                true,
                "Present only in the imported snapshot and resolved locally.",
            ),
            finding_delta(
                "diagnostic:rule-d:0005",
                DiagnosticFindingDeltaState::Suppressed,
                true,
                true,
                true,
                "Present on both sides but suppressed by the active profile.",
            ),
        ],
        impersonation_guarded: true,
        export_safe_summary:
            "Imported scanner findings compared to a live rerun, comparable once confirmed."
                .to_owned(),
    });

    // B. CI vs local rerun: blocked by a rule-pack and profile mismatch.
    let ci_vs_local = DiagnosticDeltaPacket::new(DiagnosticDeltaPacketInput {
        delta_id: "delta:ci-vs-local:0007".to_owned(),
        delta_label: "Stale nightly CI scan versus a current local rerun".to_owned(),
        comparison_basis_class: DiagnosticDeltaComparisonBasisClass::CiVsLocalRerun,
        base_side: side(
            "Nightly CI imported scan",
            DiagnosticOriginClass::ImportedSnapshot,
            DiagnosticFreshnessClass::Stale,
            DiagnosticSourceKind::ScannerImport,
            "snapshot:m5:ci-import:0007",
            "snapshot:m5:ci-import:collection:0007",
            "profile:ci-legacy",
            &["tool:scanner_import:legacy"],
        ),
        compare_side: side(
            "Current local rerun",
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticFreshnessClass::Current,
            DiagnosticSourceKind::LanguageService,
            "snapshot:m5:language-provider:0001",
            "snapshot:m5:language-provider:collection:0001",
            "profile:default",
            &["tool:language_service"],
        ),
        compatibility_class: DiagnosticDeltaCompatibilityClass::BlockedRulePackMismatch,
        compatibility_notes: vec![
            note(
                DiagnosticDeltaCompatibilityNoteClass::RulePackVersionSkew,
                "The CI scan ran an older rule-pack version, so an exact delta is blocked.",
            ),
            note(
                DiagnosticDeltaCompatibilityNoteClass::ProfileMismatch,
                "The CI scan ran under a different quality profile than the local rerun.",
            ),
        ],
        delta_counts: DiagnosticDeltaCounts {
            added: 0,
            resolved: 0,
            persisting: 0,
            suppressed_or_waived: 0,
            unmapped: 3,
        },
        finding_deltas: vec![
            finding_delta(
                "diagnostic:ci-rule-x:0001",
                DiagnosticFindingDeltaState::Unmapped,
                true,
                false,
                false,
                "Rule pack and profile differ, so this finding cannot be mapped to the rerun.",
            ),
            finding_delta(
                "diagnostic:ci-rule-y:0002",
                DiagnosticFindingDeltaState::Unmapped,
                true,
                false,
                false,
                "No comparable rule exists in the current rule pack.",
            ),
            finding_delta(
                "diagnostic:ci-rule-z:0003",
                DiagnosticFindingDeltaState::Unmapped,
                true,
                false,
                false,
                "Anchor and rule identity cannot be resolved across the mismatch.",
            ),
        ],
        impersonation_guarded: true,
        export_safe_summary:
            "Stale CI scan blocked from impersonating a current local result by a rule-pack mismatch."
                .to_owned(),
    });

    // C. Runtime vs static analysis: both live, exactly comparable.
    let runtime_vs_static = DiagnosticDeltaPacket::new(DiagnosticDeltaPacketInput {
        delta_id: "delta:runtime-vs-static:0001".to_owned(),
        delta_label: "Runtime/test findings versus static-analysis findings".to_owned(),
        comparison_basis_class: DiagnosticDeltaComparisonBasisClass::RuntimeVsStaticAnalysis,
        base_side: side(
            "Static-analysis findings",
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticFreshnessClass::Current,
            DiagnosticSourceKind::LanguageService,
            "snapshot:m5:language-provider:0001",
            "snapshot:m5:language-provider:collection:0001",
            "profile:default",
            &["tool:language_service"],
        ),
        compare_side: side(
            "Runtime/test findings",
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticFreshnessClass::Recent,
            DiagnosticSourceKind::RuntimeOrTest,
            "snapshot:m5:runtime-test:0001",
            "snapshot:m5:runtime-test:collection:0001",
            "profile:default",
            &["tool:runtime_or_test"],
        ),
        compatibility_class: DiagnosticDeltaCompatibilityClass::CompatibleExact,
        compatibility_notes: Vec::new(),
        delta_counts: DiagnosticDeltaCounts {
            added: 0,
            resolved: 0,
            persisting: 1,
            suppressed_or_waived: 0,
            unmapped: 0,
        },
        finding_deltas: vec![finding_delta(
            "diagnostic:rule-shared:0001",
            DiagnosticFindingDeltaState::Persisting,
            true,
            true,
            true,
            "The same rule is reported by both the static and runtime lanes.",
        )],
        impersonation_guarded: true,
        export_safe_summary: "Two live lanes compared exactly, no compatibility caveat required."
            .to_owned(),
    });

    vec![imported_vs_live, ci_vs_local, runtime_vs_static]
}

fn release_debt_projection() -> DiagnosticQualityReleaseDebtProjection {
    DiagnosticQualityReleaseDebtProjection {
        assembled_from_snapshots: true,
        owner_truth_preserved: true,
        expiry_truth_preserved: true,
        baseline_join_preserved: true,
        suppression_join_preserved: true,
        release_visible_debt_count: 10,
        debt_source_refs: refs(&[
            "suppression:flaky-rule:0001",
            "suppression:imported-waiver:0001",
            "baseline:ci-family:0001",
            "suppression:ci-legacy:0007",
            "baseline:ci-family:0006",
        ]),
        summary: "Release-visible debt assembled from the snapshots, retaining owner, expiry, baseline, and suppression truth."
            .to_owned(),
    }
}

fn guardrails() -> DiagnosticQualityParityGuardrails {
    DiagnosticQualityParityGuardrails {
        unlike_sources_never_flattened: true,
        anchors_never_silently_repaired: true,
        imported_live_class_explicit: true,
        freshness_and_remap_states_explicit: true,
        policy_state_preserved: true,
        every_fix_route_is_typed_proposal: true,
        ids_and_completeness_exportable: true,
    }
}

fn consumer_projection() -> DiagnosticQualityParityConsumerProjection {
    DiagnosticQualityParityConsumerProjection {
        problems_references_shared_model: true,
        review_references_shared_model: true,
        cli_headless_references_shared_model: true,
        support_export_references_shared_model: true,
        ai_evidence_references_shared_model: true,
        release_debt_references_shared_model: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_REF,
        DIAGNOSTIC_QUALITY_SNAPSHOT_SCHEMA_REF,
        DIAGNOSTIC_DELTA_PACKET_SCHEMA_REF,
        M5_DIAGNOSTIC_QUALITY_PARITY_DOC_REF,
        M5_DIAGNOSTIC_QUALITY_PARITY_ARTIFACT_REF,
        "schemas/quality/diagnostic-source-and-collection.schema.json",
        "schemas/quality/quality_session.schema.json",
    ])
}

fn packet() -> DiagnosticQualityParityPacket {
    DiagnosticQualityParityPacket::new(DiagnosticQualityParityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Diagnostic Quality Snapshots and Imported-versus-Live Deltas".to_owned(),
        snapshot_entries: snapshot_entries(),
        delta_packets: delta_packets(),
        release_debt_projection: release_debt_projection(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
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
