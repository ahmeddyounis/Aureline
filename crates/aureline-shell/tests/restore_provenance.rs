use aureline_recovery::session_restore::proposal::{
    RestoreProposal, RestoreProposalArtifactRefs, RestoreProposalCounts, RestoreProposalPanePlan,
    RestoreProposalPlanKind,
};
use aureline_recovery::session_restore::records::{RestoreClass, SurfaceClass, SurfaceRole};
use aureline_shell::restore::provenance::{
    IntentionalExclusionClass, IntentionalExclusionRow, MissingDependencyPlaceholderCard,
    PreservedPaneRole, PreservedSurfaceClass, RestoreArtifactFamily, RestoreFidelityClass,
    RestoreProducerBuildStamp, RestoreProvenanceInput, RestoreProvenanceRecord,
    RestoreProvenanceSource, RestoreRedactionClass, RestoreSourceClass, RestoreSourceEventClass,
    RestoreTruthSurface, RestoreWithoutRerunDowngrade, RestoreWithoutRerunLabel,
};
use aureline_shell::support_seed::SupportSeedSurface;
use aureline_support::bundle::{ExactBuildCapture, ReleaseChannelClass};

fn producer() -> RestoreProducerBuildStamp {
    RestoreProducerBuildStamp {
        producer_name: "aureline".to_string(),
        producer_version: "0.0.0-dev".to_string(),
        producer_channel: None,
        producer_platform_class: None,
        producer_instance_handle: Some("producer-instance:test".to_string()),
    }
}

fn source() -> RestoreProvenanceSource {
    RestoreProvenanceSource {
        artifact_family: RestoreArtifactFamily::SessionRestoreManifest,
        source_class: RestoreSourceClass::AurelineSessionRestoreManifest,
        source_artifact_ref: "session-restore-manifest:test".to_string(),
    }
}

fn proposal_with_terminal() -> RestoreProposal {
    RestoreProposal {
        record_kind: "restore_proposal_record".to_string(),
        restore_proposal_schema_version: 1,
        prior_run_abnormal: true,
        restore_class: RestoreClass::LayoutOnly,
        counts: RestoreProposalCounts {
            windows: 1,
            tab_groups: 1,
            tabs: 2,
            terminals: 1,
            evidence_packets: 1,
            ..RestoreProposalCounts::default()
        },
        artifact_refs: RestoreProposalArtifactRefs {
            checkpoint_id: Some("checkpoint:test".to_string()),
            snapshot_id: Some("snapshot:test".to_string()),
            workspace_authority_ref: Some("workspace-authority:test".to_string()),
            window_id: Some("window:test".to_string()),
        },
        pane_plans: vec![
            RestoreProposalPanePlan {
                pane_id: "pane-editor-0001".to_string(),
                surface_role: SurfaceRole::Editor,
                surface_class: SurfaceClass::TextEditor,
                plan_kind: RestoreProposalPlanKind::LiveSkeleton,
                title_hint: Some("lib.rs".to_string()),
                restore_metadata: None,
                note: "skeleton restored".to_string(),
            },
            RestoreProposalPanePlan {
                pane_id: "pane-terminal-0001".to_string(),
                surface_role: SurfaceRole::Terminal,
                surface_class: SurfaceClass::TerminalView,
                plan_kind: RestoreProposalPlanKind::BlockedSideEffectful,
                title_hint: Some("cargo test".to_string()),
                restore_metadata: None,
                note: "side-effectful surface; never auto-rerun".to_string(),
            },
        ],
        dirty_buffer_entries: Vec::new(),
        downgrade_triggers: Vec::new(),
        auto_rerun_forbidden: true,
        notes: Some("layout restore with terminal transcript".to_string()),
    }
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
}

#[test]
fn terminal_restore_without_rerun_projects_to_summary_diagnostics_and_support() {
    let proposal = proposal_with_terminal();
    let mut input = RestoreProvenanceInput::new(
        "restore-provenance:terminal-no-rerun",
        RestoreSourceEventClass::AutoCheckpoint,
        source(),
        "mono:test:created",
        producer(),
        "session-restore-manifest-schema-v1",
        "session-restore-manifest-schema-v1",
        RestoreRedactionClass::RedactValuePreserveShape,
        "mono:test:emitted",
    );
    input.compare_ref = Some("compare:restore-provenance:terminal-no-rerun".to_string());
    input.export_ref = Some("export:restore-provenance:terminal-no-rerun".to_string());
    input.missing_dependency_placeholder_cards.push(
        MissingDependencyPlaceholderCard::absent_remote(
            "placeholder-card:remote-terminal",
            "pane-remote-shell-0001",
            PreservedPaneRole::Terminal,
            PreservedSurfaceClass::TerminalView,
            "remote shell",
            "evidence:remote-shell-transcript",
        ),
    );
    input
        .restore_without_rerun_downgrades
        .push(RestoreWithoutRerunDowngrade {
            downgrade_id: "downgrade:wake:remote-shell".to_string(),
            pane_id: "pane-remote-shell-0001".to_string(),
            surface_role: PreservedPaneRole::Terminal,
            surface_class: PreservedSurfaceClass::TerminalView,
            label: RestoreWithoutRerunLabel::ReconnectRequired,
            runtime_survived: false,
            command_rerun_forbidden: true,
            authority_reacquire_forbidden: true,
            evidence_ref: Some("evidence:remote-shell-transcript".to_string()),
            note: "wake/reconnect requires explicit review; command not rerun".to_string(),
        });
    input.intentional_exclusions.push(IntentionalExclusionRow {
        exclusion_id: "intentional-exclusion:live-authority".to_string(),
        exclusion_class: IntentionalExclusionClass::NonPortableLiveAuthority,
        scope_note: "PTY handles and remote authority were not serialized.".to_string(),
    });

    let record =
        RestoreProvenanceRecord::from_proposal(&proposal, input).expect("valid provenance record");

    assert_eq!(record.resulting_fidelity, RestoreFidelityClass::LayoutOnly);
    assert!(record
        .restore_without_rerun_downgrades
        .iter()
        .any(|row| row.label == RestoreWithoutRerunLabel::TranscriptOnly));
    assert!(record
        .restore_without_rerun_downgrades
        .iter()
        .any(|row| row.label == RestoreWithoutRerunLabel::ReconnectRequired));

    let projections = record.surface_projections();
    for surface in [
        RestoreTruthSurface::StartupRecovery,
        RestoreTruthSurface::RestoreSummary,
        RestoreTruthSurface::Diagnostics,
        RestoreTruthSurface::SupportExport,
    ] {
        let projection = projections
            .iter()
            .find(|projection| projection.surface == surface)
            .expect("surface projection");
        assert_eq!(projection.fidelity, RestoreFidelityClass::LayoutOnly);
        assert!(projection
            .restore_without_rerun_labels
            .contains(&RestoreWithoutRerunLabel::TranscriptOnly));
    }

    let support = SupportSeedSurface::restore_provenance_preview(
        fixture_capture(),
        "2026-05-13T02:20:00Z",
        &record,
    )
    .expect("support preview");
    let row = support
        .preview
        .manifest
        .preview_items
        .iter()
        .find(|item| item.parity_binding.support_pack_item_id == record.support_pack_item_id())
        .expect("restore provenance support row");
    assert!(row.notes.contains("Layout only"));
    assert!(row.notes.contains("transcript restored; command not rerun"));
    assert!(row.notes.contains("reconnect required"));
}

#[test]
fn workspace_restore_fidelity_fixtures_cover_all_controlled_classes() {
    let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| {
            p.join("fixtures")
                .join("workspace")
                .join("restore_fidelity_cases")
        })
        .expect("derive fixtures dir");

    let mut covered = std::collections::BTreeSet::new();
    let mut found_support_export = false;
    let mut found_no_rerun = false;

    for entry in std::fs::read_dir(&fixtures_dir).expect("fixtures dir") {
        let entry = entry.expect("fixture entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
            continue;
        }
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        let record: RestoreProvenanceRecord = serde_yaml::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));
        record
            .validate()
            .unwrap_or_else(|err| panic!("validate {}: {err}", path.display()));

        let projections = record.surface_projections();
        assert_eq!(projections.len(), 4);
        assert!(projections
            .iter()
            .any(|projection| projection.surface == RestoreTruthSurface::StartupRecovery));
        assert!(projections
            .iter()
            .any(|projection| projection.surface == RestoreTruthSurface::Diagnostics));
        found_support_export |= projections
            .iter()
            .any(|projection| projection.surface == RestoreTruthSurface::SupportExport);
        found_no_rerun |= !record.restore_without_rerun_downgrades.is_empty();
        covered.insert(record.resulting_fidelity);
    }

    for required in [
        RestoreFidelityClass::Exact,
        RestoreFidelityClass::Compatible,
        RestoreFidelityClass::LayoutOnly,
        RestoreFidelityClass::RecoveredDrafts,
        RestoreFidelityClass::EvidenceOnly,
    ] {
        assert!(covered.contains(&required), "missing {:?}", required);
    }
    assert!(found_support_export);
    assert!(found_no_rerun);
}

#[test]
fn exact_restore_rejects_hidden_downgrade_labels() {
    let mut record = RestoreProvenanceRecord {
        schema: None,
        fixture: None,
        record_kind: "state_restore_provenance_and_placeholder_record".to_string(),
        restore_provenance_schema_version: 1,
        restore_provenance_id: "restore-provenance:bad-exact".to_string(),
        source_event_class: RestoreSourceEventClass::ManualExport,
        source: source(),
        created_at: "mono:test:created".to_string(),
        producer_build: producer(),
        source_schema_version: "session-restore-manifest-schema-v1".to_string(),
        redaction_class: RestoreRedactionClass::None,
        resulting_fidelity: RestoreFidelityClass::Exact,
        restore_level: RestoreFidelityClass::Exact.restore_level(),
        missing_dependency_classes: Vec::new(),
        missing_dependency_placeholder_cards: Vec::new(),
        schema_migration_note:
            aureline_shell::restore::provenance::SchemaMigrationNote::no_migration_required(
                "session-restore-manifest-schema-v1",
                "session-restore-manifest-schema-v1",
            ),
        preserved_prior_artifacts: Vec::new(),
        intentional_exclusions: Vec::new(),
        rollback_checkpoint_ref: None,
        equivalence_map_ref: None,
        compare_ref: None,
        export_ref: None,
        restore_without_rerun_downgrades: Vec::new(),
        emitted_at: "mono:test:emitted".to_string(),
        notes: None,
    };
    record
        .restore_without_rerun_downgrades
        .push(RestoreWithoutRerunDowngrade {
            downgrade_id: "downgrade:hidden-terminal".to_string(),
            pane_id: "pane-terminal-0001".to_string(),
            surface_role: PreservedPaneRole::Terminal,
            surface_class: PreservedSurfaceClass::TerminalView,
            label: RestoreWithoutRerunLabel::TranscriptOnly,
            runtime_survived: false,
            command_rerun_forbidden: true,
            authority_reacquire_forbidden: true,
            evidence_ref: Some("evidence:terminal".to_string()),
            note: "terminal transcript restored; command not rerun".to_string(),
        });

    assert!(record.validate().is_err());
}
