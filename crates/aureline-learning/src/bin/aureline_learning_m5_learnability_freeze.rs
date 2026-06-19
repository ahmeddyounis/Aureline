//! Headless emitter and validator for the frozen M5 learnability lane.
//!
//! Emits the seeded freeze packet as JSON, validates it against the frozen
//! learnability invariants, and can refresh the on-disk fixture so it stays in
//! sync with the Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full freeze packet as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_freeze -- freeze
//!
//! # Print a plaintext summary of the vocabulary and lane matrix.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_freeze -- summary
//!
//! # Validate the seeded freeze packet (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_freeze -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_freeze \
//!   -- emit-fixture fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_learnability_lane_freeze, validate_m5_learnability_lane, M5LearnabilityLaneFreeze,
    QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let freeze = seeded_m5_learnability_lane_freeze();

    match args.first().map(String::as_str) {
        None | Some("freeze") => {
            println!("{}", serde_json::to_string_pretty(&freeze)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&freeze);
            Ok(())
        }
        Some("validate") => match validate_m5_learnability_lane(&freeze) {
            Ok(()) => {
                println!("ok — all M5 learnability-lane invariants pass");
                Ok(())
            }
            Err(errors) => {
                for e in &errors {
                    eprintln!("FAIL {e}");
                }
                Err(format!("{} validation error(s)", errors.len()).into())
            }
        },
        Some("emit-fixture") => {
            let path: PathBuf = args
                .get(1)
                .ok_or("emit-fixture requires a target path argument")?
                .into();
            let json = serde_json::to_string_pretty(&freeze)?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, format!("{json}\n"))?;
            println!("wrote {}", path.display());
            Ok(())
        }
        Some(unknown) => Err(format!("unknown subcommand: {unknown}").into()),
    }
}

fn print_summary(freeze: &M5LearnabilityLaneFreeze) {
    println!("M5 learnability-lane freeze: {}", freeze.freeze_id);
    println!("Generated: {}", freeze.generated_at);
    println!("Overall verdict: {}", freeze.overall_verdict.as_str());
    if !freeze.overall_narrowing_reasons.is_empty() {
        println!("Narrowing reasons:");
        for r in &freeze.overall_narrowing_reasons {
            println!("  - {r}");
        }
    }

    println!("\nControlled vocabulary ({}):", freeze.vocabulary.len());
    for entry in &freeze.vocabulary {
        println!(
            "  {} [explain_apply={}, mutation={}, ownership={}]",
            entry.token,
            entry.explain_apply_class.as_str(),
            entry.mutation_path_class.as_str(),
            entry.data_ownership_class.as_str(),
        );
    }

    println!("\nEducational-AI boundary:");
    let b = &freeze.educational_ai_boundary;
    println!("  explain_and_do_separate: {}", b.explain_and_do_separate);
    println!(
        "  do_requires_same_preview_approval: {}",
        b.do_requires_same_preview_approval
    );
    println!(
        "  can_mutate_live_state_directly: {}",
        b.can_mutate_live_state_directly
    );

    println!("\nLane matrix ({} rows):", freeze.lane_rows.len());
    for row in &freeze.lane_rows {
        if row.verdict != QualificationVerdict::QualifiedStable {
            println!("  {} → {}", row.surface_token, row.verdict.as_str());
            for r in &row.narrowing_reasons {
                println!("      ⚠ {r}");
            }
        }
    }

    if freeze.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ Every claimed M5 family routes through the frozen lane at Stable.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more lane rows narrowed below Stable.",
            freeze.overall_verdict.as_str()
        );
    }
}
