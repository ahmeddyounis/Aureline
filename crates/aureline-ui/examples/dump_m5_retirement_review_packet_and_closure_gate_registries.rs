//! Headless emitter for the M5 retirement-review-packet and support-note-closure-gate registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-retirement-review-packet-and-closure-gate-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-retirement-review-packet-and-closure-gate-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- report
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- retirement-review-packet-table
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- fixture-retirement-review-packet-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- fixture-closure-gate-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- validate
//! ```

use aureline_ui::m5_retirement_review_packet_and_closure_gate_registries::{
    seeded_m5_retirement_review_packet_and_closure_gate_registries,
    seeded_m5_retirement_review_packet_and_closure_gate_registries_closure_gate_preview_narrowed,
    seeded_m5_retirement_review_packet_and_closure_gate_registries_retirement_review_packet_beta_narrowed,
    M5RetirementReviewPacketClosureGateRegistriesPacket,
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
            let packet = seeded_m5_retirement_review_packet_and_closure_gate_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_retirement_review_packet_and_closure_gate_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_retirement_review_packet_and_closure_gate_registries()
                    .render_matrix_csv()
            );
        }
        Some("retirement-review-packet-table") => {
            print!(
                "{}",
                seeded_m5_retirement_review_packet_and_closure_gate_registries()
                    .render_retirement_review_packet_table()
            );
        }
        Some("fixture-retirement-review-packet-beta-narrowed") => {
            let packet =
                seeded_m5_retirement_review_packet_and_closure_gate_registries_retirement_review_packet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-closure-gate-preview-narrowed") => {
            let packet =
                seeded_m5_retirement_review_packet_and_closure_gate_registries_closure_gate_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_retirement_review_packet_and_closure_gate_registries(),
                seeded_m5_retirement_review_packet_and_closure_gate_registries_retirement_review_packet_beta_narrowed(),
                seeded_m5_retirement_review_packet_and_closure_gate_registries_closure_gate_preview_narrowed(),
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
    packet: &M5RetirementReviewPacketClosureGateRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
