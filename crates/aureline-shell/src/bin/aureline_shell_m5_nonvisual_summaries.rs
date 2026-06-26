//! Headless emitter for the M5 non-visual custom-surface summary catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-nonvisual-summary-proof/` and the narrowed fixtures under
//! `fixtures/a11y/m5-nonvisual-summaries/`. Editor, terminal, data, observability,
//! review, and docs surfaces project these summaries so each custom-rendered surface
//! explains its own structure and current fidelity non-visually — a quantified
//! structure plus object-linked, keyboard-reachable drill-down routes, an export-safe
//! text alternative and metadata view for charts/traces/diffs/artifacts, and the
//! current preview/cached/generated/approximate/sampled/buffered presentation state —
//! rather than exposing pixels and hover states alone.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- fixture-proof-stale-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- fixture-bridge-unavailable-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- validate
//! ```

use aureline_shell::accessibility::summaries::{
    seeded_m5_nonvisual_summary_catalog,
    seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed,
    seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed, M5NonVisualSummaryCatalogPacket,
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
            let packet = seeded_m5_nonvisual_summary_catalog();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_nonvisual_summary_catalog().render_markdown_summary()
            );
        }
        Some("fixture-proof-stale-narrowed") => {
            let packet = seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-bridge-unavailable-narrowed") => {
            let packet = seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_nonvisual_summary_catalog(),
                seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed(),
                seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed(),
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
    packet: &M5NonVisualSummaryCatalogPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "non-visual summary catalog failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
