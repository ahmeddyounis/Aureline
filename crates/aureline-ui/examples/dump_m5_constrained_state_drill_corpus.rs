//! Headless emitter for the M5 constrained-state drill corpus.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-constrained-state-drills/`, its matrix CSV, the Markdown summary, the health dashboard under
//! `dashboards/`, and the narrowed fixtures under `fixtures/editor/m5-constrained-state-drills/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- support-export
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- report
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- csv
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- dashboard
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- fixture-mixed-state-narrowed
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- fixture-read-only-generated-narrowed
//! cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- validate
//! ```

use aureline_ui::m5_constrained_state_drill_corpus::{
    seeded_m5_constrained_state_drill_corpus,
    seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed,
    seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed,
    M5ConstrainedStateDrillCorpusPacket,
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
            let packet = seeded_m5_constrained_state_drill_corpus();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_constrained_state_drill_corpus().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_constrained_state_drill_corpus().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_constrained_state_drill_corpus().render_health_dashboard()
            );
        }
        Some("fixture-mixed-state-narrowed") => {
            let packet = seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-read-only-generated-narrowed") => {
            let packet = seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_constrained_state_drill_corpus(),
                seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed(),
                seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed(),
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
    packet: &M5ConstrainedStateDrillCorpusPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "drill-corpus packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
