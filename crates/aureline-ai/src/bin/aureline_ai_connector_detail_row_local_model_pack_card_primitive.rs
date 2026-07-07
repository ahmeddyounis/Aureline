//! Headless emitter for the M5 AI connector-detail-row / local-model-pack-card
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/`.
//! AI settings, the model picker, the route inspector, the evidence view, and the CLI /
//! support export all read this primitive so one connector row names its canonical id,
//! publisher, execution locus, capabilities, auth posture, and warm/cold/unavailable/
//! policy-blocked readiness, and one model pack card names its identity, digest, disk
//! cost, hardware fit, offline posture, and bounded select/verify/remove actions.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- fixture-route-inspector-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- fixture-evidence-view-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- validate
//! ```

use aureline_ai::implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces::{
    seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed,
    seeded_m5_ai_connector_model_primitive_packet,
    seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed,
    M5AiConnectorModelPrimitivePacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_ai_connector_model_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_connector_model_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_connector_model_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-route-inspector-preview-narrowed") => {
            let packet = seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-evidence-view-beta-narrowed") => {
            let packet = seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_connector_model_primitive_packet(),
                seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed(),
                seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &M5AiConnectorModelPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
