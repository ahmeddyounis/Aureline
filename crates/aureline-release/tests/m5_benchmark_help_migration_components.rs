//! Integration tests for M5 benchmark/help/migration component proof artifacts.

use std::collections::BTreeSet;

use aureline_release::m5_benchmark_help_migration_components::{
    current_about_service_health_card, current_benchmark_evidence_card,
    current_benchmark_evidence_cards, current_community_handoff_tiles, current_importer_diff_row,
    current_importer_review_table, current_support_package_card, validate_benchmark_evidence_cards,
    validate_community_handoff_tiles, AboutDowngradeState, BenchmarkEvidenceSourceClass,
    CommunityHandoffTileViolation, HandoffDestinationGroup, HandoffDestinationState,
    HandoffTrustClass, ImporterApplyState, ImporterDiffRowViolation, ImporterMigrationDomain,
    ImporterOutcomeState, ServiceFreshnessState, SupportPackageState,
    M5_ABOUT_SERVICE_HEALTH_CARD_FIXTURE_REF, M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_REF,
    M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF, M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF,
    M5_COMMUNITY_HANDOFF_TILE_FIXTURE_REF, M5_COMMUNITY_HANDOFF_TILE_SCHEMA_REF,
    M5_IMPORTER_DIFF_ROW_FIXTURE_REF, M5_IMPORTER_DIFF_ROW_SCHEMA_REF,
    M5_IMPORTER_REVIEW_TABLE_FIXTURE_REF, M5_SUPPORT_PACKAGE_CARD_FIXTURE_REF,
    M5_SUPPORT_PACKAGE_CARD_SCHEMA_REF,
};

const PROOF_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-benchmark-help-migration-proof/proof_packet.json"
));

const SUPPORT_EXPORT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-benchmark-help-migration-proof/support_export.json"
));

#[test]
fn checked_in_benchmark_cards_validate_cleanly() {
    let cards = current_benchmark_evidence_cards().expect("fixtures parse");
    let violations = validate_benchmark_evidence_cards(&cards);
    assert!(
        violations.is_empty(),
        "unexpected benchmark card violations: {violations:#?}"
    );
}

#[test]
fn canonical_fixture_ref_points_to_the_lab_reference_card() {
    let card = current_benchmark_evidence_card().expect("canonical card parses");
    assert_eq!(
        card.evidence_source_class,
        BenchmarkEvidenceSourceClass::LabReferenceRun
    );
    assert_eq!(
        M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF,
        "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json"
    );
    assert_eq!(
        M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF,
        "schemas/ui/m5-benchmark-evidence-card.schema.json"
    );
}

#[test]
fn proof_packet_names_required_source_classes_and_exports() {
    let proof: serde_json::Value =
        serde_json::from_str(PROOF_PACKET_JSON).expect("proof packet parses");
    let family = proof["component_families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|family| family["family"].as_str() == Some("benchmark_evidence_card"))
        .expect("benchmark family present");

    assert_eq!(
        family["schema_ref"].as_str(),
        Some(M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF)
    );
    assert_eq!(
        family["fixture_ref"].as_str(),
        Some(M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF)
    );

    let proved: BTreeSet<_> = family["evidence_source_classes_proved"]
        .as_array()
        .expect("source classes")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "lab_reference_run",
        "self_capture",
        "design_partner_result",
        "community_report",
        "imported_evidence",
    ] {
        assert!(proved.contains(required), "missing {required}");
    }

    let export_fields: BTreeSet<_> = family["export_parity_fields"]
        .as_array()
        .expect("export fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "benchmark_id",
        "caveat_summary_refs",
        "compare_view",
        "trace_report_export",
    ] {
        assert!(export_fields.contains(required), "missing {required}");
    }
}

#[test]
fn support_export_preserves_source_class_coverage_and_caveat_parity() {
    let export: serde_json::Value =
        serde_json::from_str(SUPPORT_EXPORT_JSON).expect("support export parses");
    let row = export["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .find(|row| row["family"].as_str() == Some("benchmark_evidence_card"))
        .expect("benchmark support row present");

    assert_eq!(
        row["workflow_budget_truth"].as_str(),
        Some("measured_value_vs_budget_cold_warm_sample_size_extension_set_power_mode_scope_as_of_visible")
    );
    assert_eq!(
        row["export_parity"].as_str(),
        Some("benchmark_id_caveat_summaries_compare_view_trace_report_export_preserved")
    );

    let coverage: BTreeSet<_> = row["source_class_coverage"]
        .as_array()
        .expect("coverage")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "lab_reference_run",
        "self_capture",
        "design_partner_result",
        "community_report",
        "imported_evidence",
    ] {
        assert!(coverage.contains(required), "missing {required}");
    }
}

#[test]
fn about_service_health_fixture_validates_and_preserves_local_first_truth() {
    let card = current_about_service_health_card().expect("about fixture parses");
    let violations = card.validate();
    assert!(
        violations.is_empty(),
        "unexpected about/service-health violations: {violations:#?}"
    );
    assert_eq!(
        card.freshness_state,
        ServiceFreshnessState::StaleCache,
        "fixture must prove cached/stale health honesty"
    );
    assert_eq!(
        card.downgrade_state,
        AboutDowngradeState::CachedServiceHealth
    );

    let copy = format!(
        "{}\n{}\n{}",
        card.copy_export.text, card.copy_export.json, card.copy_export.markdown
    );
    for required in [
        "1.0.0",
        "stable",
        "local_app",
        "mirrored_verified",
        "local_docs_pack_search",
        "Copy build info",
        "diagnostics",
        "require no sign-in",
    ] {
        assert!(copy.contains(required), "missing {required}");
    }
}

#[test]
fn support_package_fixture_validates_and_preserves_save_local_submit_later_truth() {
    let card = current_support_package_card().expect("support fixture parses");
    let violations = card.validate();
    assert!(
        violations.is_empty(),
        "unexpected support-package violations: {violations:#?}"
    );
    assert_eq!(card.package_state, SupportPackageState::SavedLocalOnly);

    let copy = format!(
        "{}\n{}\n{}",
        card.copy_export.text, card.copy_export.json, card.copy_export.markdown
    );
    for required in [
        "saved_local_only",
        "local-support-packet:m5:import-preview:0001",
        "not_submitted",
        "build_info",
        "service_health_snapshot",
        "explicit user action",
        "inspection",
    ] {
        assert!(copy.contains(required), "missing {required}");
    }
}

#[test]
fn proof_packet_names_about_service_health_and_support_required_fields() {
    let proof: serde_json::Value =
        serde_json::from_str(PROOF_PACKET_JSON).expect("proof packet parses");

    let about = proof["component_families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|family| family["family"].as_str() == Some("about_service_health_card"))
        .expect("about/service-health family present");
    assert_eq!(
        about["schema_ref"].as_str(),
        Some(M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_REF)
    );
    assert_eq!(
        about["fixture_ref"].as_str(),
        Some(M5_ABOUT_SERVICE_HEALTH_CARD_FIXTURE_REF)
    );
    let about_fields: BTreeSet<_> = about["export_parity_fields"]
        .as_array()
        .expect("about export fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "version",
        "channel",
        "install_mode",
        "provenance_state",
        "copy_build_info_action",
        "local_workflows_available",
        "diagnostics_action",
        "export_action",
    ] {
        assert!(about_fields.contains(required), "missing {required}");
    }

    let support = proof["component_families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|family| family["family"].as_str() == Some("support_package_card"))
        .expect("support-package family present");
    assert_eq!(
        support["schema_ref"].as_str(),
        Some(M5_SUPPORT_PACKAGE_CARD_SCHEMA_REF)
    );
    assert_eq!(
        support["fixture_ref"].as_str(),
        Some(M5_SUPPORT_PACKAGE_CARD_FIXTURE_REF)
    );
    let support_fields: BTreeSet<_> = support["export_parity_fields"]
        .as_array()
        .expect("support export fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "package_contents",
        "local_save_summary",
        "redaction_export_summary",
        "submit_later_summary",
    ] {
        assert!(support_fields.contains(required), "missing {required}");
    }
}

#[test]
fn importer_fixtures_validate_and_group_stable_outcomes() {
    let row = current_importer_diff_row().expect("importer row parses");
    let row_violations = row.validate();
    assert!(
        row_violations.is_empty(),
        "unexpected importer row violations: {row_violations:#?}"
    );

    let table = current_importer_review_table().expect("importer table parses");
    let table_violations = table.validate();
    assert!(
        table_violations.is_empty(),
        "unexpected importer table violations: {table_violations:#?}"
    );

    let outcomes: BTreeSet<_> = table.rows.iter().map(|row| row.outcome_state).collect();
    for required in ImporterOutcomeState::STABLE_GROUP_ORDER {
        assert!(outcomes.contains(&required), "missing {required:?}");
    }

    let domains: BTreeSet<_> = table.rows.iter().map(|row| row.migration_domain).collect();
    for required in [
        ImporterMigrationDomain::Settings,
        ImporterMigrationDomain::Shortcuts,
        ImporterMigrationDomain::ExtensionsAndProviders,
        ImporterMigrationDomain::TasksAndRunConfigs,
        ImporterMigrationDomain::WorkspaceMetadata,
    ] {
        assert!(domains.contains(&required), "missing {required:?}");
    }
}

#[test]
fn dropping_post_apply_importer_visibility_fails_validation() {
    let mut table = current_importer_review_table().expect("importer table parses");
    table.post_apply_summary.unsupported_row_refs.clear();
    table.post_apply_summary.bridge_required_row_refs.clear();
    let violations = table.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            ImporterDiffRowViolation::UnsupportedRowMissingFromSummary { .. }
        )),
        "expected unsupported summary violation, got {violations:#?}"
    );
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            ImporterDiffRowViolation::BridgeRowMissingFromSummary { .. }
        )),
        "expected bridge summary violation, got {violations:#?}"
    );
}

#[test]
fn importer_post_apply_summary_preserves_shortcuts_bridges_issue_export_partial_apply_and_restore()
{
    let table = current_importer_review_table().expect("importer table parses");

    assert_eq!(
        table.shortcut_change_digest.digest_id,
        "shortcut-change-digest:vscode-first-run-demo"
    );
    assert!(
        table.shortcut_change_digest.visible_after_apply
            && table.shortcut_change_digest.support_export_visible
    );
    assert!(table
        .shortcut_change_digest
        .high_frequency_command_refs
        .contains(&"vscode:keybinding:workbench.action.showCommands".to_owned()));
    assert!(table
        .shortcut_change_digest
        .lossy_or_changed_row_refs
        .contains(&"importer-diff:vscode-shortcut-command-palette".to_owned()));

    let bridge_rows: BTreeSet<_> = table
        .rows
        .iter()
        .filter(|row| row.outcome_state == ImporterOutcomeState::BridgeRequired)
        .map(|row| row.row_id.as_str())
        .collect();
    for row_ref in bridge_rows {
        assert!(
            table
                .bridge_detail_inspectors
                .iter()
                .any(|inspector| inspector.row_ref == row_ref
                    && inspector.visible_after_apply
                    && inspector.support_export_visible
                    && !inspector.compatibility_report_ref.is_empty()
                    && !inspector.issue_template_ref.is_empty()),
            "missing post-apply bridge inspector for {row_ref}"
        );
    }
    assert!(table.issue_template_export.includes_bridge_details);
    assert!(
        table
            .issue_template_export
            .includes_compatibility_report_links
    );
    assert!(table.issue_template_export.includes_partial_apply_summary);
    assert!(table.issue_template_export.includes_restore_summary);
    assert_eq!(
        table.partial_apply_summary.apply_state,
        ImporterApplyState::PartialApplied
    );
    assert!(table
        .partial_apply_summary
        .unresolved_row_refs
        .contains(&"importer-diff:vscode-eslint-extension".to_owned()));
    assert!(table.restore_summary.restore_available);
    assert!(table.restore_summary.visible_after_apply);
    assert!(table.restore_summary.support_export_visible);
}

#[test]
fn collapsing_post_apply_importer_supportability_fails_validation() {
    let mut table = current_importer_review_table().expect("importer table parses");
    table.shortcut_change_digest.row_refs.clear();
    table.bridge_detail_inspectors.clear();
    table.compatibility_report_links.clear();
    table.issue_template_export.includes_bridge_details = false;
    table.partial_apply_summary.apply_state = ImporterApplyState::Complete;
    table.restore_summary.restore_available = false;

    let violations = table.validate();
    for expected in [
        "shortcut digest",
        "bridge inspector",
        "compatibility report",
        "issue template",
        "partial apply",
        "restore summary",
    ] {
        let matched = match expected {
            "shortcut digest" => violations.iter().any(|violation| {
                matches!(
                    violation,
                    ImporterDiffRowViolation::ShortcutDigestMissingPostApplyTruth { .. }
                )
            }),
            "bridge inspector" => violations.iter().any(|violation| {
                matches!(
                    violation,
                    ImporterDiffRowViolation::BridgeInspectorMissing { .. }
                )
            }),
            "compatibility report" => violations.iter().any(|violation| {
                matches!(
                    violation,
                    ImporterDiffRowViolation::CompatibilityReportLinkMissing { .. }
                )
            }),
            "issue template" => violations.iter().any(|violation| {
                matches!(
                    violation,
                    ImporterDiffRowViolation::IssueTemplateExportMissing { .. }
                )
            }),
            "partial apply" => violations.iter().any(|violation| {
                matches!(
                    violation,
                    ImporterDiffRowViolation::PartialApplyTruthCollapsed { .. }
                )
            }),
            "restore summary" => violations.iter().any(|violation| {
                matches!(
                    violation,
                    ImporterDiffRowViolation::RestoreSummaryMissing { .. }
                )
            }),
            _ => false,
        };
        assert!(
            matched,
            "expected {expected} violation, got {violations:#?}"
        );
    }
}

#[test]
fn proof_packet_names_importer_diff_rows_review_groups_and_export_fields() {
    let proof: serde_json::Value =
        serde_json::from_str(PROOF_PACKET_JSON).expect("proof packet parses");
    let family = proof["component_families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|family| family["family"].as_str() == Some("importer_diff_row"))
        .expect("importer family present");

    assert_eq!(
        family["schema_ref"].as_str(),
        Some(M5_IMPORTER_DIFF_ROW_SCHEMA_REF)
    );
    assert_eq!(
        family["fixture_ref"].as_str(),
        Some(M5_IMPORTER_DIFF_ROW_FIXTURE_REF)
    );
    assert!(family["additional_fixture_refs"]
        .as_array()
        .expect("additional fixtures")
        .iter()
        .any(|value| value.as_str() == Some(M5_IMPORTER_REVIEW_TABLE_FIXTURE_REF)));

    let groups: BTreeSet<_> = family["stable_outcome_groups_proved"]
        .as_array()
        .expect("outcome groups")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "imported",
        "mapped",
        "skipped",
        "manual_review",
        "bridge_required",
        "unsupported",
    ] {
        assert!(groups.contains(required), "missing {required}");
    }

    let fields: BTreeSet<_> = family["importer_diff_truth_fields"]
        .as_array()
        .expect("importer fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "source_object_ref",
        "source_value",
        "target_object_ref",
        "target_value",
        "translated_result",
        "reason_detail_note",
        "manual_review_action",
        "docs_action",
        "export_safe_identifiers",
    ] {
        assert!(fields.contains(required), "missing {required}");
    }

    let grouped_fields: BTreeSet<_> = family["grouped_review_truth_fields"]
        .as_array()
        .expect("grouped review fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "post_apply_summary",
        "shortcut_change_digest",
        "bridge_detail_inspectors",
        "compatibility_report_links",
        "issue_template_export",
        "partial_apply_summary",
        "restore_summary",
    ] {
        assert!(grouped_fields.contains(required), "missing {required}");
    }

    let supportability: BTreeSet<_> = family["post_import_supportability_proved"]
        .as_array()
        .expect("post-import supportability")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "high_frequency_shortcut_changes_render_separately_from_settings_diffs",
        "bridge_required_rows_keep_detail_inspectors_after_apply",
        "bridge_and_unsupported_rows_keep_compatibility_report_links_after_apply",
        "issue_template_export_includes_bridge_unsupported_partial_apply_and_restore_truth",
        "partial_apply_summary_uses_partial_applied_not_generic_complete",
        "restore_summary_keeps_checkpoint_and_restore_refs",
    ] {
        assert!(supportability.contains(required), "missing {required}");
    }
}

#[test]
fn support_export_preserves_importer_grouping_and_post_apply_truth() {
    let export: serde_json::Value =
        serde_json::from_str(SUPPORT_EXPORT_JSON).expect("support export parses");
    let row = export["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .find(|row| row["family"].as_str() == Some("importer_diff_row"))
        .expect("importer support row present");

    assert_eq!(
        row["row_truth"].as_str(),
        Some("source_object_value_target_object_value_translated_result_outcome_reason_actions_export_safe_identifiers_visible")
    );
    assert_eq!(
        row["review_table_truth"].as_str(),
        Some("rows_grouped_by_imported_mapped_skipped_manual_review_bridge_required_unsupported")
    );
    assert_eq!(
        row["post_apply_export_truth"].as_str(),
        Some("lossy_skipped_bridge_required_unsupported_rows_remain_visible_after_apply_and_in_support_export")
    );
    assert_eq!(
        row["shortcut_change_digest_truth"].as_str(),
        Some("high_frequency_shortcut_changes_render_in_dedicated_digest_after_apply_and_support_export")
    );
    assert_eq!(
        row["bridge_followup_truth"].as_str(),
        Some(
            "bridge_detail_inspectors_and_compatibility_report_links_remain_available_after_apply"
        )
    );
    assert_eq!(
        row["issue_template_export_truth"].as_str(),
        Some("issue_template_export_includes_bridge_details_compatibility_reports_partial_apply_summary_and_restore_summary")
    );
    assert_eq!(
        row["partial_apply_truth"].as_str(),
        Some(
            "partial_applied_state_remains_visible_with_unresolved_downgraded_and_blocked_row_refs"
        )
    );
    assert_eq!(
        row["restore_summary_truth"].as_str(),
        Some("restore_available_with_checkpoint_refs_and_restore_refs_visible_after_apply_and_support_export")
    );

    let coverage: BTreeSet<_> = row["outcome_group_coverage"]
        .as_array()
        .expect("outcome coverage")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "imported",
        "mapped",
        "skipped",
        "manual_review",
        "bridge_required",
        "unsupported",
    ] {
        assert!(coverage.contains(required), "missing {required}");
    }
}

#[test]
fn community_handoff_fixtures_validate_and_preserve_class_boundaries() {
    let tiles = current_community_handoff_tiles().expect("community handoff fixtures parse");
    let violations = validate_community_handoff_tiles(&tiles);
    assert!(
        violations.is_empty(),
        "unexpected community handoff violations: {violations:#?}"
    );

    let trust_classes: BTreeSet<_> = tiles.iter().map(|tile| tile.trust_class).collect();
    for required in [
        HandoffTrustClass::OfficialPublic,
        HandoffTrustClass::OfficialAuthenticated,
        HandoffTrustClass::Community,
        HandoffTrustClass::VendorManaged,
        HandoffTrustClass::LocalOnly,
    ] {
        assert!(trust_classes.contains(&required), "missing {required:?}");
    }

    let groups: BTreeSet<_> = tiles.iter().map(|tile| tile.destination_group).collect();
    for required in [
        HandoffDestinationGroup::Help,
        HandoffDestinationGroup::Release,
        HandoffDestinationGroup::Migration,
        HandoffDestinationGroup::Support,
    ] {
        assert!(groups.contains(&required), "missing {required:?}");
    }

    for blocked in [
        HandoffDestinationState::BrowserBlocked,
        HandoffDestinationState::Offline,
        HandoffDestinationState::StaleCachedTarget,
    ] {
        let tile = tiles
            .iter()
            .find(|tile| tile.destination_state == blocked)
            .expect("blocked/offline fixture present");
        assert!(
            tile.actions.iter().any(|action| action.available_offline
                && action.preserves_destination_identity
                && action.preserves_trust_class_context),
            "{blocked:?} missing offline context action"
        );
    }
}

#[test]
fn missing_vendor_class_coverage_fails_community_handoff_validation() {
    let tiles: Vec<_> = current_community_handoff_tiles()
        .expect("community handoff fixtures parse")
        .into_iter()
        .filter(|tile| tile.trust_class != HandoffTrustClass::VendorManaged)
        .collect();
    let violations = validate_community_handoff_tiles(&tiles);
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            CommunityHandoffTileViolation::MissingTrustClass {
                trust_class: HandoffTrustClass::VendorManaged
            }
        )),
        "expected vendor trust-class coverage violation, got {violations:#?}"
    );
}

#[test]
fn proof_packet_names_community_handoff_destination_classes_and_actions() {
    let proof: serde_json::Value =
        serde_json::from_str(PROOF_PACKET_JSON).expect("proof packet parses");
    let family = proof["component_families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|family| family["family"].as_str() == Some("community_handoff_tile"))
        .expect("community handoff family present");

    assert_eq!(
        family["schema_ref"].as_str(),
        Some(M5_COMMUNITY_HANDOFF_TILE_SCHEMA_REF)
    );
    assert_eq!(
        family["fixture_ref"].as_str(),
        Some(M5_COMMUNITY_HANDOFF_TILE_FIXTURE_REF)
    );

    let classes: BTreeSet<_> = family["destination_classes_proved"]
        .as_array()
        .expect("destination classes")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "official_public",
        "official_authenticated",
        "community",
        "vendor_managed",
        "local_only",
    ] {
        assert!(classes.contains(required), "missing {required}");
    }

    let fields: BTreeSet<_> = family["handoff_truth_fields"]
        .as_array()
        .expect("handoff truth fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "destination_group",
        "destination_type",
        "trust_class",
        "version_awareness_note",
        "destination_state",
        "local_safe_fallback_ref",
        "actions",
    ] {
        assert!(fields.contains(required), "missing {required}");
    }
}

#[test]
fn support_export_names_handoff_continuity_and_class_coverage() {
    let export: serde_json::Value =
        serde_json::from_str(SUPPORT_EXPORT_JSON).expect("support export parses");
    let row = export["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .find(|row| row["family"].as_str() == Some("community_handoff_tile"))
        .expect("community handoff support row present");

    assert_eq!(
        row["handoff_truth"].as_str(),
        Some("destination_group_type_route_ownership_trust_version_visibility_auth_data_exit_commitment_destination_state_fallback_actions_visible")
    );
    assert_eq!(
        row["continuity_truth"].as_str(),
        Some("browser_blocked_offline_stale_cached_targets_keep_copy_or_export_actions_with_destination_identity_and_trust_class")
    );
    let classes: BTreeSet<_> = row["destination_class_coverage"]
        .as_array()
        .expect("destination classes")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "official_public",
        "official_authenticated",
        "community",
        "vendor_managed",
        "local_only",
    ] {
        assert!(classes.contains(required), "missing {required}");
    }
}
