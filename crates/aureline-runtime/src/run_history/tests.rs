//! Unit coverage for the run-history / evidence-panel object and its first
//! consumers: the seed is stable, every entrypoint binds a non-empty panel whose
//! evidence rows quote the recomputed rerun resolution, rerun always resolves
//! fresh authority, imported rows never offer rerun, recorded macros never offer
//! external rerun, open-as-recipe never launders a capability, and each freeze
//! guardrail blocks stable.

use super::*;

#[test]
fn seed_is_stable_with_no_findings() {
    let packet = seeded_run_history_first_consumers_packet();
    assert!(packet.is_stable());
    assert!(packet.validate().is_empty());
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::Stable
    );
}

#[test]
fn every_entrypoint_binds_a_non_empty_panel() {
    let packet = seeded_run_history_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let binding = packet
            .binding(entrypoint)
            .unwrap_or_else(|| panic!("missing binding for {}", entrypoint.as_str()));
        assert!(!binding.entries.is_empty());
        assert_eq!(binding.evidence_rows.len(), binding.entries.len());
        for (entry, row) in binding.entries.iter().zip(&binding.evidence_rows) {
            // Every entry resolves a run identity and a layer, and the projected
            // evidence row quotes the recomputed rerun resolution.
            assert!(!entry.run_identity.run_id.is_empty());
            assert_eq!(row.rerun_action_class, entry.resolved_rerun_class());
            assert_eq!(row.run_identity, entry.run_identity);
            assert!(entry.rerun_consistent());
            assert!(entry.open_as_recipe_consistent());
            assert!(entry.secret_references_opaque());
            assert!(entry.retention_consistent());
        }
    }
}

#[test]
fn rerun_resolution_is_always_fresh() {
    let packet = seeded_run_history_first_consumers_packet();
    for row in packet.all_evidence_rows() {
        // The evidence rows expose the resolved rerun for every consumer.
        let _ = row.rerun_action_class;
    }
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        for entry in seeded_consumer_panel(entrypoint) {
            let resolution = entry.resolve_rerun("2026-06-18T01:00:00Z");
            assert!(resolution.is_fresh());
            assert!(resolution.resolves_current_policy);
            assert!(!resolution.reuses_cached_approval);
            assert!(!resolution.reuses_stale_environment);
            assert!(resolution.secret_references_reresolved);
            assert_eq!(resolution.rerun_action_class, entry.resolved_rerun_class());
        }
    }
}

#[test]
fn imported_rows_never_offer_rerun() {
    let imported = seeded_imported_entry();
    assert!(imported.imported);
    assert_eq!(
        imported.resolved_rerun_class(),
        RerunActionClass::BlockedImportedRecord
    );
    assert!(!imported.rerun_admissible());
}

#[test]
fn macro_layer_never_resolves_to_external_rerun() {
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        for entry in seeded_consumer_panel(entrypoint) {
            if entry.automation_layer == AutomationLayerClass::RecordedMacro {
                assert!(!entry.resolved_rerun_class().is_extension_or_imported_only());
            }
        }
    }
}

#[test]
fn rerun_derivation_precedence_denies_over_revalidation() {
    // A denial blocker dominates a revalidation blocker.
    assert_eq!(
        derive_rerun_class(
            false,
            &[
                CurrentPolicyBlocker::FreshApprovalRequired,
                CurrentPolicyBlocker::PublisherRevoked,
            ],
        ),
        RerunActionClass::BlockedPublisherRevoked
    );
    // A revalidation blocker dominates no blocker.
    assert_eq!(
        derive_rerun_class(false, &[CurrentPolicyBlocker::KillSwitchEngaged]),
        RerunActionClass::AdmissibleAfterKillSwitchClear
    );
    // Imported always wins.
    assert_eq!(
        derive_rerun_class(true, &[CurrentPolicyBlocker::NoBlockerPresent]),
        RerunActionClass::BlockedImportedRecord
    );
    // No blocker resolves to admissible-no-revalidation.
    assert_eq!(
        derive_rerun_class(false, &[CurrentPolicyBlocker::NoBlockerPresent]),
        RerunActionClass::AdmissibleNoRevalidation
    );
}

#[test]
fn export_round_trips_and_preserves_identity_and_rerun() {
    let export = seeded_run_history_export_roundtrip();
    let imported = export.import();
    let reexported = imported.export(export.export_id.clone(), export.exported_at.clone());
    assert_eq!(reexported, export);
    assert!(export.identity_and_rerun_preserved());
}

#[test]
fn cli_headless_view_explains_every_entrypoint() {
    let packet = seeded_run_history_first_consumers_packet();
    let view = packet.cli_headless_view("view:test", "2026-06-18T00:01:00Z");
    assert!(view.every_entrypoint_explained());
}

#[test]
fn support_export_carries_one_row_and_evidence_per_entry() {
    let packet = seeded_run_history_first_consumers_packet();
    let export = packet.support_export("support:test", "2026-06-18T00:01:00Z");
    assert!(export.is_export_safe());
    assert_eq!(
        export.consumer_rows.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
    let total_entries: usize = packet
        .consumer_bindings
        .iter()
        .map(|binding| binding.entries.len())
        .sum();
    assert_eq!(export.evidence_rows.len(), total_entries);
}

#[test]
fn missing_entrypoint_blocks_stable() {
    let mut input = current_run_history_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
    let packet = RunHistoryFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == RunHistoryFindingKind::MissingEntrypoint));
}

#[test]
fn rerun_implying_cached_approval_blocks_stable() {
    let mut input = current_run_history_first_consumers_input();
    let mut entry = seeded_consumer_entry(RecipeBuilderEntrypoint::RequestApi);
    entry
        .current_policy_blockers
        .push(CurrentPolicyBlocker::NoBlockerPresent);
    rebuild_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, vec![entry]);
    let packet = RunHistoryFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == RunHistoryFindingKind::RerunImpliesCachedApproval));
}

#[test]
fn capability_laundering_blocks_stable() {
    let mut input = current_run_history_first_consumers_input();
    let mut entry = seeded_consumer_entry(RecipeBuilderEntrypoint::TaskTestDebug);
    entry.open_as_recipe_action_class = OpenAsRecipeActionClass::AdmissibleMacroPromotable;
    rebuild_binding(
        &mut input,
        RecipeBuilderEntrypoint::TaskTestDebug,
        vec![entry],
    );
    let packet = RunHistoryFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet.validation_findings.iter().any(
        |finding| finding.finding_kind == RunHistoryFindingKind::CapabilityLaunderedIntoRecipe
    ));
}

#[test]
fn raw_secret_material_blocks_stable() {
    let mut input = current_run_history_first_consumers_input();
    let mut entry = seeded_consumer_entry(RecipeBuilderEntrypoint::RequestApi);
    entry.secret_reference_refs = vec!["raw:plaintext-token".to_owned()];
    rebuild_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, vec![entry]);
    let packet = RunHistoryFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == RunHistoryFindingKind::RawSecretMaterialInHistory));
}

#[test]
fn invariant_violation_blocks_stable() {
    let mut input = current_run_history_first_consumers_input();
    input.invariants.raw_secrets_never_appear_in_history = false;
    let packet = RunHistoryFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == RunHistoryFindingKind::InvariantViolated));
}

fn rebuild_binding(
    input: &mut RunHistoryFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    entries: Vec<RunHistoryEntry>,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(RunHistoryConsumerBinding::from_entries(
            entrypoint,
            entries,
            "mutated panel",
        ));
}
