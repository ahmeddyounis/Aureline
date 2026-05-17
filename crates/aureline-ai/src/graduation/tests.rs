use std::collections::BTreeSet;

use super::*;
use crate::registry::ProviderModelRegistryPacket;

fn fixture_registry() -> ProviderModelRegistryPacket {
    serde_json::from_str(include_str!(
        "../../../../fixtures/ai/provider_model_registry_beta/registry_packet.json"
    ))
    .expect("provider/model registry fixture parses")
}

fn fixture_graduation_state() -> AiGraduationState {
    current_beta_graduation_state().expect("graduation state parses")
}

#[test]
fn checked_in_graduation_state_validates_claimed_ai_surfaces() {
    let registry = fixture_registry();
    let graduation_state = fixture_graduation_state();

    assert!(
        graduation_state.validate(&registry).is_empty(),
        "{:?}",
        graduation_state.validate(&registry)
    );

    let summaries = graduation_state.support_summaries_for_registry(&registry);
    assert_eq!(summaries.len(), registry.claimed_surfaces.len());
    assert!(summaries.iter().all(|summary| {
        summary.promotion_gate_token == "promotable"
            && summary.effective_support_class_token == "supported"
            && summary.packet_freshness_token == "current"
            && summary.owner_ref.is_some()
            && summary.eval_set_ref.is_some()
            && summary.eval_thresholds_ref.is_some()
            && summary.cost_profile_ref.is_some()
            && summary.kill_switch_ref.is_some()
    }));
}

#[test]
fn missing_or_stale_packets_downgrade_ai_rows() {
    let registry = fixture_registry();
    let mut graduation_state = fixture_graduation_state();
    graduation_state
        .packets
        .retain(|packet| packet.workflow_or_surface_id != "surface:inline-chat-local-first");

    let missing_status =
        graduation_state.surface_status(&registry, "surface:inline-chat-local-first");
    assert_eq!(missing_status.gate_state, AiGraduationGateState::Downgraded);
    assert_eq!(
        missing_status.effective_support_class,
        AiGraduationSupportClass::EvidenceStale
    );
    assert_eq!(
        missing_status.downgrade_reason_tokens,
        vec![AiGraduationViolation::SurfaceMissingPacket
            .as_str()
            .to_owned()]
    );
    assert!(graduation_state
        .validate(&registry)
        .contains(&AiGraduationViolation::SurfaceMissingPacket));

    let mut stale_state = fixture_graduation_state();
    stale_state
        .packets
        .iter_mut()
        .find(|packet| packet.workflow_or_surface_id == "surface:review-chat-cheapest")
        .expect("review packet exists")
        .expires_at = "2026-05-01T00:00:00Z".to_owned();
    let stale_status = stale_state.surface_status(&registry, "surface:review-chat-cheapest");
    assert_eq!(stale_status.gate_state, AiGraduationGateState::Downgraded);
    assert_eq!(
        stale_status.effective_support_class,
        AiGraduationSupportClass::EvidenceStale
    );
    assert!(stale_status.downgrade_reason_tokens.contains(
        &AiGraduationViolation::SurfacePacketStale
            .as_str()
            .to_owned()
    ));
}

#[test]
fn checked_in_support_export_reads_graduation_state() {
    let registry = fixture_registry();
    let graduation_state = fixture_graduation_state();

    let generated: serde_json::Value = serde_json::from_str(
        &registry
            .support_export_projection_with_graduation(&graduation_state)
            .export_safe_json(),
    )
    .expect("generated support export parses");
    let checked_in: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../artifacts/ai/m3/graduation_packets/support_export_projection.json"
    ))
    .expect("checked-in graduation support export parses");

    assert_eq!(generated, checked_in);
}

#[test]
fn standalone_packets_and_thresholds_match_graduation_state() {
    let graduation_state = fixture_graduation_state();
    let packet_artifacts =
        current_beta_graduation_packet_artifacts().expect("standalone packets parse");
    let packet_ids = packet_artifacts.keys().cloned().collect::<BTreeSet<_>>();

    for packet_ref in &graduation_state.packet_refs {
        let packet_id = packet_ref
            .split('#')
            .nth(1)
            .expect("packet ref carries fragment");
        assert!(
            packet_ids.contains(packet_id),
            "packet ref {packet_ref} must resolve to checked-in packet artifact"
        );
    }

    let thresholds: serde_yaml::Value = serde_yaml::from_str(include_str!(
        "../../../../artifacts/ai/m3/eval_thresholds.yaml"
    ))
    .expect("thresholds YAML parses");
    let threshold_set_id = thresholds
        .get("threshold_set_id")
        .and_then(serde_yaml::Value::as_str)
        .expect("threshold set id exists");
    assert_eq!(
        graduation_state.eval_thresholds_ref,
        format!("artifacts/ai/m3/eval_thresholds.yaml#{threshold_set_id}")
    );

    let threshold_surfaces = thresholds
        .get("thresholds")
        .and_then(serde_yaml::Value::as_sequence)
        .expect("threshold rows exist")
        .iter()
        .filter_map(|row| row.get("workflow_or_surface_id"))
        .filter_map(serde_yaml::Value::as_str)
        .collect::<BTreeSet<_>>();
    for packet in packet_artifacts.values() {
        assert!(threshold_surfaces.contains(packet.workflow_or_surface_id.as_str()));
        assert_eq!(
            packet.eval_thresholds_ref,
            graduation_state.eval_thresholds_ref
        );
    }
}
