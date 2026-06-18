//! Inline unit tests for the cross-surface label-parity packet.

use super::*;
use crate::m5_automation_contract_baseline::AutomationSafetyLabelId;

#[test]
fn seeded_packet_is_stable_and_clean() {
    let packet = seeded_label_parity_packet();
    assert!(packet.is_stable());
    assert!(packet.validation_findings.is_empty());
    assert_eq!(packet.record_kind, LABEL_PARITY_RECORD_KIND);
    assert_eq!(packet.schema_version, LABEL_PARITY_SCHEMA_VERSION);
    assert!(packet.packet_digest.starts_with("fnv1a64:"));
}

#[test]
fn seed_covers_the_whole_vocabulary() {
    let packet = seeded_label_parity_packet();
    // The vocabulary block is the full, ordered, frozen set.
    assert_eq!(packet.vocabulary, canonical_safety_labels());
    // Every label appears as a source label on at least one command.
    for label in AutomationSafetyLabelId::ALL {
        assert!(
            packet
                .command_rows
                .iter()
                .any(|row| row.source_labels.contains(&label)),
            "no command claims {}",
            label.as_str()
        );
    }
}

#[test]
fn every_command_projects_to_every_surface_with_canonical_tokens() {
    let packet = seeded_label_parity_packet();
    for row in &packet.command_rows {
        for surface in LabelSurfaceClass::ALL {
            let projection = row.projection(surface).unwrap_or_else(|| {
                panic!("missing {} on {}", surface.as_str(), row.canonical_verb)
            });
            // Same stable-id set as the source.
            assert_eq!(
                sorted(projection.stable_id_tokens()),
                sorted(row.source_stable_id_tokens()),
            );
            // Canonical tokens, no synonyms, all states preserved.
            for label in &projection.projected_labels {
                assert!(label.stable_id_matches());
                assert!(label.display_token_matches());
            }
            assert!(projection.preserves_stable_ids());
        }
    }
}

#[test]
fn digest_is_order_invariant() {
    let mut input = current_label_parity_input();
    let forward = LabelParityPacket::materialize(input.clone());
    input.command_rows.reverse();
    let reversed = LabelParityPacket::materialize(input);
    assert_eq!(forward.packet_digest, reversed.packet_digest);
}

fn first_finding_kind(packet: &LabelParityPacket) -> LabelParityFindingKind {
    packet
        .validation_findings
        .first()
        .expect("at least one finding")
        .finding_kind
}

#[test]
fn missing_surface_blocks_stable() {
    let mut input = current_label_parity_input();
    input.command_rows[0]
        .surface_projections
        .retain(|projection| projection.surface != LabelSurfaceClass::DocsHelp);
    let packet = LabelParityPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert_eq!(
        first_finding_kind(&packet),
        LabelParityFindingKind::MissingSurfaceProjection
    );
    assert_eq!(packet.validation_findings.len(), 1);
}

#[test]
fn extra_label_is_set_drift() {
    let mut input = current_label_parity_input();
    let projection = input.command_rows[2]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == LabelSurfaceClass::CommandPaletteRow)
        .expect("palette projection");
    // request.send_saved has no macro_safe; adding it is a non-effect drift.
    projection.projected_labels.push(ProjectedLabel::canonical(
        AutomationSafetyLabelId::MacroSafe,
    ));
    let packet = LabelParityPacket::materialize(input);
    assert_eq!(
        first_finding_kind(&packet),
        LabelParityFindingKind::SurfaceLabelSetDrift
    );
    assert_eq!(packet.validation_findings.len(), 1);
}

#[test]
fn synonym_display_token_blocks_stable() {
    let mut input = current_label_parity_input();
    let projection = input.command_rows[0]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == LabelSurfaceClass::ReleasePublicTruth)
        .expect("release projection");
    let label = projection
        .projected_labels
        .iter_mut()
        .find(|label| label.label_id == AutomationSafetyLabelId::WritesFiles)
        .expect("writes_files label");
    label.display_token = "Writes to disk".to_owned();
    let packet = LabelParityPacket::materialize(input);
    assert_eq!(
        first_finding_kind(&packet),
        LabelParityFindingKind::SynonymDisplayToken
    );
    assert_eq!(packet.validation_findings.len(), 1);
}

#[test]
fn dropped_effect_disclosure_blocks_stable() {
    let mut input = current_label_parity_input();
    let projection = input.command_rows[0]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == LabelSurfaceClass::DocsHelp)
        .expect("docs projection");
    projection
        .projected_labels
        .retain(|label| label.label_id != AutomationSafetyLabelId::WritesFiles);
    let packet = LabelParityPacket::materialize(input);
    assert_eq!(
        first_finding_kind(&packet),
        LabelParityFindingKind::EffectDisclosureDropped
    );
    // Only the focused finding fires, not the generic set drift.
    assert_eq!(packet.validation_findings.len(), 1);
}

#[test]
fn lost_stable_id_state_blocks_stable() {
    let mut input = current_label_parity_input();
    let projection = input.command_rows[4]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == LabelSurfaceClass::SupportExport)
        .expect("support projection");
    projection.preserves_stable_ids_on_downgrade = false;
    let packet = LabelParityPacket::materialize(input);
    assert_eq!(
        first_finding_kind(&packet),
        LabelParityFindingKind::StableIdNotPreservedAcrossStates
    );
    assert_eq!(packet.validation_findings.len(), 1);
}

#[test]
fn stable_id_token_drift_blocks_stable() {
    let mut input = current_label_parity_input();
    let projection = input.command_rows[1]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == LabelSurfaceClass::MacroRecorder)
        .expect("macro projection");
    let label = projection
        .projected_labels
        .iter_mut()
        .find(|label| label.label_id == AutomationSafetyLabelId::MacroSafe)
        .expect("macro_safe label");
    label.stable_id_token = "macrosafe".to_owned();
    let packet = LabelParityPacket::materialize(input);
    // A drifted stable id changes the set too, so both the drift and the
    // outside-vocabulary findings fire; the stable-id drift is reported.
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == LabelParityFindingKind::LabelOutsideVocabulary));
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
}

#[test]
fn invariant_violation_blocks_stable() {
    let mut input = current_label_parity_input();
    input.invariants.no_surface_invents_synonyms = false;
    let packet = LabelParityPacket::materialize(input);
    assert_eq!(
        first_finding_kind(&packet),
        LabelParityFindingKind::InvariantViolated
    );
    assert_eq!(packet.validation_findings.len(), 1);
}

#[test]
fn support_export_and_cli_view_are_consistent() {
    let packet = seeded_label_parity_packet();
    let export = packet.support_export(LABEL_PARITY_SUPPORT_EXPORT_ID, "2026-06-18T00:01:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id, packet.packet_id);
    assert_eq!(export.packet_digest, packet.packet_digest);
    assert_eq!(export.command_rows.len(), packet.command_rows.len());
    assert_eq!(
        export.vocabulary_tokens.len(),
        AutomationSafetyLabelId::ALL.len()
    );

    let view = packet.cli_headless_view(LABEL_PARITY_CLI_HEADLESS_ID, "2026-06-18T00:01:00Z");
    assert!(view.explains_command_count(packet.command_rows.len()));
}

#[test]
fn validate_helper_agrees_with_promotion() {
    let packet = seeded_label_parity_packet();
    assert!(validate_label_parity_packet(&packet).is_ok());
}
