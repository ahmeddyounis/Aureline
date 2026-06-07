use std::path::Path;

use aureline_runtime::{
    current_stable_build_test_event_interoperability_input, BuildTestEventInteroperabilityPacket,
    BuildTestEventInteroperabilityPacketInput, BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR,
};
use serde_json::json;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = BuildTestEventInteroperabilityPacket::materialize(
        current_stable_build_test_event_interoperability_input(),
    );
    write_json(
        &root.join("artifacts/runtime/m4/build_test_event_interoperability_packet.json"),
        &packet,
    );

    for (file_name, case_name, scenario, mutation) in [
        (
            "baseline_stable.json",
            "baseline_stable",
            "Canonical packet preserves source kind, confidence, raw payload refs, provenance, capability negotiation, and replay/export parity.",
            "none",
        ),
        (
            "missing_raw_payload_reference_blocks_stable.json",
            "missing_raw_payload_reference_blocks_stable",
            "A normalized heuristic diagnostic lost its retained raw payload reference before replay/export.",
            "drop_raw_payload_ref",
        ),
        (
            "heuristic_overclaim_blocks_stable.json",
            "heuristic_overclaim_blocks_stable",
            "A heuristic parser event attempted to present medium-high confidence without downgrade disclosure.",
            "heuristic_overclaims_confidence",
        ),
        (
            "missing_capability_negotiation_blocks_stable.json",
            "missing_capability_negotiation_blocks_stable",
            "Imported Bazel BEP evidence is missing an explicit adapter capability negotiation row.",
            "drop_capability_negotiation",
        ),
        (
            "consumer_confidence_drift_blocks_stable.json",
            "consumer_confidence_drift_blocks_stable",
            "The AI explanation projection drops confidence while keeping the event reference.",
            "consumer_drops_confidence",
        ),
        (
            "export_raw_private_material_blocks_stable.json",
            "export_raw_private_material_blocks_stable",
            "Replay/export parity admits raw private material past the support boundary.",
            "export_raw_private_material",
        ),
        (
            "missing_target_graph_ready_blocks_stable.json",
            "missing_target_graph_ready_blocks_stable",
            "The canonical TargetGraphReady lifecycle event is absent from the packet.",
            "drop_target_graph_ready",
        ),
    ] {
        let packet = BuildTestEventInteroperabilityPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "build_test_event_interoperability_stable_case",
            "schema_version": 1,
            "case_name": case_name,
            "scenario": scenario,
            "mutation": mutation,
            "expect": {
                "promotion_state": packet.promotion_state.as_str(),
                "validation_finding_count": packet.validation_findings.len(),
                "expected_finding_kinds": packet.validation_findings.iter().map(|finding| finding.finding_kind.as_str()).collect::<Vec<_>>(),
                "source_kind_tokens": packet.source_kind_tokens(),
                "event_kind_tokens": packet.event_kind_tokens(),
                "consumer_surface_tokens": packet.consumer_surface_tokens(),
                "support_export_safe": packet.support_export(format!("support-export:{case_name}"), "2026-06-07T00:01:00Z").is_export_safe()
            }
        });
        write_json(
            &root
                .join(BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR)
                .join(file_name),
            &fixture,
        );
    }
}

fn mutated_input(mutation: &str) -> BuildTestEventInteroperabilityPacketInput {
    let mut input = current_stable_build_test_event_interoperability_input();
    match mutation {
        "none" => {}
        "drop_raw_payload_ref" => {
            input
                .raw_payload_refs
                .retain(|raw| raw.raw_payload_ref != "raw:heuristic:diagnostic");
        }
        "heuristic_overclaims_confidence" => {
            let heuristic = input
                .events
                .iter_mut()
                .find(|event| event.raw_payload_ref == "raw:heuristic:diagnostic")
                .expect("stable input contains heuristic diagnostic event");
            heuristic.confidence = aureline_runtime::BuildTestInteropConfidence::MediumHigh;
            heuristic.downgraded = false;
        }
        "drop_capability_negotiation" => {
            input.capability_negotiations.retain(|row| {
                !(row.lane == aureline_runtime::BuildTestInteropLane::ImportedProvider
                    && row.source_kind == aureline_runtime::BuildTestInteropSourceKind::BazelBep)
            });
        }
        "consumer_drops_confidence" => {
            let projection = input
                .consumer_projections
                .iter_mut()
                .find(|projection| {
                    projection.consumer_surface
                        == aureline_runtime::BuildTestConsumerSurface::AiExplanation
                })
                .expect("stable input contains AI explanation projection");
            projection.preserves_confidence = false;
        }
        "export_raw_private_material" => {
            input.replay_export_parity.raw_private_material_excluded = false;
        }
        "drop_target_graph_ready" => {
            input.events.retain(|event| {
                event.event_kind != aureline_runtime::BuildTestInteropEventKind::TargetGraphReady
            });
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    let payload = serde_json::to_string_pretty(value).expect("serialize JSON");
    std::fs::write(path, format!("{payload}\n")).expect("write JSON");
}
