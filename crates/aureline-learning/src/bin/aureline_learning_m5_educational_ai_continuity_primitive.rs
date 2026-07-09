//! Headless emitter for the M5 learning educational-AI continuity controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-learning-educational-ai-continuity-proof/`, its matrix CSV, the Markdown
//! design report, and the scenario fixtures under
//! `fixtures/ui/m5-learning-educational-ai-continuity-controls/`. The learning / onboarding and
//! help / docs surfaces read these controls so one degraded learning component names whether its
//! content is live, cached, local-only, offline, stale, uncited, or not installed, what to do
//! next, and whether educational AI may apply anything — never mutating live state without the
//! ordinary preview / approval crossing, and staying useful offline.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- fixture-citation-unavailable-glossary
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- fixture-not-installed-progress-marker
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- validate
//! ```

use aureline_learning::ship_educational_ai_boundaries_no_hidden_apply_safeguards_and_offline_local_only_or_cached_pack_continuity_across_claimed_m5_guided_teaching_flows::{
    seeded_learning_educational_ai_continuity_controls,
    seeded_learning_educational_ai_continuity_controls_citation_unavailable_glossary,
    seeded_learning_educational_ai_continuity_controls_not_installed_progress_marker,
    LearningEducationalAiContinuityPacket,
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
            let packet = seeded_learning_educational_ai_continuity_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_learning_educational_ai_continuity_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_learning_educational_ai_continuity_controls().render_matrix_csv()
            );
        }
        Some("fixture-citation-unavailable-glossary") => {
            let packet =
                seeded_learning_educational_ai_continuity_controls_citation_unavailable_glossary();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-not-installed-progress-marker") => {
            let packet =
                seeded_learning_educational_ai_continuity_controls_not_installed_progress_marker();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_learning_educational_ai_continuity_controls(),
                seeded_learning_educational_ai_continuity_controls_citation_unavailable_glossary(),
                seeded_learning_educational_ai_continuity_controls_not_installed_progress_marker(),
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
    packet: &LearningEducationalAiContinuityPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "learning educational-AI continuity primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
