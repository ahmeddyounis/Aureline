//! Headless emitter for the M5 run-comparison-table / compare-guard-banner controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-run-comparison-table-compare-guard-banner-controls/`. The comparison and
//! compare-guard surfaces read these components so one comparison table names its baseline and
//! candidate run identities, metric values, delta, threshold state, confidence, comparator type,
//! and explicit code / data / environment / hardware difference summaries — and offers
//! open-baseline / open-current / export-comparison — and one guard banner discloses what is
//! comparable, partially comparable, or not comparable — including missing lineage fields, changed
//! factors, and redaction — and offers open-full-lineage, so a metric delta never implies a fair
//! baseline when the parity evidence is incomplete.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- fixture-comparison-table-not-comparable
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- fixture-compare-guard-banner-blocked
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- validate
//! ```

use aureline_notebook::implement_run_comparison_tables_and_compare_guard_banners_with_baseline_candidate_identity_confounder_disclosure_and_no_fair_delta_claims_when_parity_evidence_is_incomplete_across_claimed_m5_compare_flows::{
    seeded_run_comparison_table_compare_guard_banner_controls,
    seeded_run_comparison_table_compare_guard_banner_controls_compare_guard_banner_blocked,
    seeded_run_comparison_table_compare_guard_banner_controls_comparison_table_not_comparable,
    RunComparisonTableCompareGuardBannerControlsPacket,
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
            let packet = seeded_run_comparison_table_compare_guard_banner_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_run_comparison_table_compare_guard_banner_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_run_comparison_table_compare_guard_banner_controls().render_matrix_csv()
            );
        }
        Some("fixture-comparison-table-not-comparable") => {
            let packet =
                seeded_run_comparison_table_compare_guard_banner_controls_comparison_table_not_comparable();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-compare-guard-banner-blocked") => {
            let packet =
                seeded_run_comparison_table_compare_guard_banner_controls_compare_guard_banner_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_run_comparison_table_compare_guard_banner_controls(),
                seeded_run_comparison_table_compare_guard_banner_controls_comparison_table_not_comparable(),
                seeded_run_comparison_table_compare_guard_banner_controls_compare_guard_banner_blocked(),
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
    packet: &RunComparisonTableCompareGuardBannerControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "run comparison compare guard primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
