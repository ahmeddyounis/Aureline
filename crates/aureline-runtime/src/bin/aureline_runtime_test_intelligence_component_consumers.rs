//! Headless emitter for the M05-1033 test-intelligence component consumer
//! adoption lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-test-intelligence-component-consumer-proof/`, its
//! matrix CSV, and the Markdown report. Editor gutters and inline coverage
//! summaries, the test tree, PR / review views, CLI summaries, imported-CI
//! detail views, and support / export packets read this packet so every surface
//! reuses one of the seven frozen test-intelligence components and keeps
//! provenance / freshness, included-run scope, artifact baseline identity,
//! raw-or-text fallback, and generated-test assumption boundaries aligned — and
//! a consumer whose evidence is imported, whose shard scope is omitted, whose
//! provenance is stale, whose flakiness is only suspected, or whose generated
//! test still carries unverified assumptions auto-narrows its claim rather than
//! letting a single percentage hide a shard omission, an intermittent failure
//! read as confirmed flakiness, or generated changes collapse into one opaque
//! apply.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- validate
//! ```

use aureline_runtime::add_shared_editor_gutter_test_tree_pr_review_cli_summary_support_export_and_imported_ci_consumers_so_test_intelligence_components_keep_scope_freshness_and_baseline_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_test_intelligence_component_consumers_packet, IntelConsumerPacket,
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
            let packet = seeded_m5_test_intelligence_component_consumers_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_test_intelligence_component_consumers_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_test_intelligence_component_consumers_packet().render_matrix_csv()
            );
        }
        Some("validate") => {
            assert_valid(&seeded_m5_test_intelligence_component_consumers_packet())?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &IntelConsumerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<String> = violations.iter().map(|v| v.to_string()).collect();
        Err(format!(
            "test intelligence component consumers failed validation: {}",
            tokens.join("; ")
        )
        .into())
    }
}
