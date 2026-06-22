//! Unit tests for the canonical assist-support / provider-debug packet.

use super::*;

#[test]
fn packet_builds_and_every_invariant_holds() {
    let packet = assist_support_packet();
    assert_eq!(packet.record_kind, M5_ASSIST_SUPPORT_RECORD_KIND);
    assert_eq!(packet.schema_ref, M5_ASSIST_SUPPORT_SCHEMA_REF);
    assert_eq!(packet.packet_id, M5_ASSIST_SUPPORT_PACKET_ID);
    assert_eq!(
        packet.m5_assist_support_schema_version,
        M5_ASSIST_SUPPORT_SCHEMA_VERSION
    );
    assert!(
        packet.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        packet
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(packet.is_support_export_safe());
    assert!(packet.raw_payload_excluded);
}

#[test]
fn packet_serialization_round_trips() {
    let packet = assist_support_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let restored: AssistSupportPacket = serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(packet, restored);
}

#[test]
fn every_decision_id_uses_its_kind_prefix() {
    let packet = assist_support_packet();
    for decision in &packet.decisions {
        assert!(
            decision.decision_id.starts_with(decision.kind.id_prefix()),
            "decision {} must use prefix {}",
            decision.decision_id,
            decision.kind.id_prefix()
        );
        assert!(decision.field_id.starts_with("assist_support."));
        assert!(!decision.subject_ref.is_empty());
        assert!(!decision.provider_id.is_empty());
        assert!(decision.redaction_safe);
    }
}

#[test]
fn decision_ids_are_unique() {
    let packet = assist_support_packet();
    let mut ids: Vec<&str> = packet
        .decisions
        .iter()
        .map(|decision| decision.decision_id.as_str())
        .collect();
    ids.sort_unstable();
    let count = ids.len();
    ids.dedup();
    assert_eq!(count, ids.len(), "decision ids must be unique");
}

#[test]
fn clean_baseline_exists_for_every_kind() {
    let packet = assist_support_packet();
    for kind in AssistDecisionKind::ALL {
        assert!(
            packet
                .decisions_for_kind(kind)
                .any(|decision| decision.is_clean()),
            "kind {} must have a no-drift baseline decision",
            kind.as_str()
        );
    }
}

#[test]
fn drifted_decisions_name_a_route_and_explain_themselves() {
    let packet = assist_support_packet();
    for decision in packet
        .decisions
        .iter()
        .filter(|decision| !decision.is_clean())
    {
        assert!(
            decision.next_safe_action.is_some(),
            "drifted decision {} must offer a next-safe-action",
            decision.decision_id
        );
        assert!(
            decision.next_action_command.is_some(),
            "drifted decision {} must carry the route command id",
            decision.decision_id
        );
        assert!(
            !decision.explanation.is_empty(),
            "drifted decision {} must carry an explanation",
            decision.decision_id
        );
    }
}

#[test]
fn clean_decisions_carry_no_remediation() {
    let packet = assist_support_packet();
    for decision in packet
        .decisions
        .iter()
        .filter(|decision| decision.is_clean())
    {
        assert!(decision.next_safe_action.is_none());
        assert!(decision.next_action_command.is_none());
        assert!(decision.narrow_reason.is_none());
        assert_eq!(decision.degrade_state, AssistDegradeClass::FullFidelity);
        assert!(decision.content_state.is_live());
        assert_eq!(decision.mapping_quality, MappingQualityClass::Exact);
    }
}

#[test]
fn every_drift_class_is_observed_and_rolled_up() {
    let packet = assist_support_packet();
    for class in AssistDriftClass::ALL {
        let observed = packet
            .decisions
            .iter()
            .any(|decision| decision.drift_class == class);
        assert!(observed, "drift class {} must be exercised", class.as_str());
        let rollup = packet
            .drift_rollup(class)
            .unwrap_or_else(|| panic!("missing rollup for {}", class.as_str()));
        let count = packet
            .decisions
            .iter()
            .filter(|decision| decision.drift_class == class)
            .count();
        assert_eq!(rollup.count, count);
    }
}

#[test]
fn rollup_counts_sum_to_decision_total() {
    let packet = assist_support_packet();
    let drift_total: usize = packet.drift_rollups.iter().map(|rollup| rollup.count).sum();
    let surface_total: usize = packet
        .surface_rollups
        .iter()
        .map(|rollup| rollup.count)
        .sum();
    assert_eq!(drift_total, packet.decisions.len());
    assert_eq!(surface_total, packet.decisions.len());
}

#[test]
fn support_export_redacts_sensitive_classes() {
    let packet = assist_support_packet();
    for class in [
        "source_text",
        "prompt_context",
        "provider_payload",
        "credential_body",
    ] {
        assert!(
            packet
                .support_export
                .redacted_classes
                .iter()
                .any(|redacted| redacted == class),
            "support export must redact {class}"
        );
    }
    assert!(packet.support_export.raw_payload_excluded);
    // No decision field carries free-form payload text beyond the bounded
    // explanation; the explanation never echoes a provider id verbatim as content.
    for decision in &packet.decisions {
        assert!(decision.explanation.len() < 400);
    }
}

#[test]
fn constrained_surfaces_are_all_represented() {
    let packet = assist_support_packet();
    for surface in [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
    ] {
        assert!(
            packet.decisions_for_surface(surface).next().is_some(),
            "constrained surface {} must be represented",
            surface.as_str()
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let packet = assist_support_packet();
    let lines = assist_support_packet_lines(&packet);
    assert!(lines
        .iter()
        .any(|line| line.contains("Assist-support packet")));
    assert!(lines.iter().any(|line| line.contains("Drift rollups:")));
    assert!(lines.iter().any(|line| line.contains("Surface rollups:")));
    // Every drift-class token surfaces in the projection.
    for class in AssistDriftClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(class.as_str())),
            "projection must mention drift class {}",
            class.as_str()
        );
    }
}
