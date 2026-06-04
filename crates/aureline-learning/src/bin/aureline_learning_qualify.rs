//! Headless emitter and validator for the guided-learning qualification corpus.
//!
//! Emits the seeded qualification manifest as JSON, validates it, and can
//! refresh the on-disk fixtures so the fixture corpus stays in sync with the
//! Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_qualify -- manifest
//!
//! # Print a plaintext qualification summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_qualify -- summary
//!
//! # Validate the seeded corpus (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_qualify -- validate
//!
//! # Emit the on-disk qualification fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_qualify \
//!   -- emit-fixture fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/qualification_manifest.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_guided_learning_qualification_corpus, validate_guided_learning_qualification,
    GuidedLearningQualificationManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let corpus = seeded_guided_learning_qualification_corpus();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&corpus)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&corpus);
            Ok(())
        }
        Some("validate") => match validate_guided_learning_qualification(&corpus) {
            Ok(()) => {
                println!("ok — all qualification invariants pass");
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
            let json = serde_json::to_string_pretty(&corpus)?;
            std::fs::write(&path, json)?;
            println!("wrote {}", path.display());
            Ok(())
        }
        Some(unknown) => Err(format!("unknown subcommand: {unknown}").into()),
    }
}

fn print_summary(manifest: &GuidedLearningQualificationManifest) {
    println!(
        "Guided-learning qualification manifest: {}",
        manifest.manifest_id
    );
    println!("Generated: {}", manifest.generated_at);
    println!("Overall verdict: {}", manifest.overall_verdict.as_str());
    if !manifest.overall_narrowing_reasons.is_empty() {
        println!("Narrowing reasons:");
        for r in &manifest.overall_narrowing_reasons {
            println!("  - {r}");
        }
    }

    println!("\nGlossary packs ({}):", manifest.glossary_packs.len());
    for p in &manifest.glossary_packs {
        println!(
            "  {} → {} [{}]",
            p.record_id,
            p.verdict.as_str(),
            p.lifecycle_label
        );
    }

    println!("\nTour packages ({}):", manifest.tour_packages.len());
    for p in &manifest.tour_packages {
        let reasons = if p.narrowing_reasons.is_empty() {
            String::new()
        } else {
            format!(" ({})", p.narrowing_reasons.join("; "))
        };
        println!("  {} → {}{}", p.record_id, p.verdict.as_str(), reasons);
    }

    println!("\nExercise rails ({}):", manifest.exercise_rails.len());
    for r in &manifest.exercise_rails {
        println!(
            "  {} → {} explain/apply={}",
            r.record_id,
            r.verdict.as_str(),
            r.explain_apply_class.as_str()
        );
    }

    println!(
        "\nLearning-mode profiles ({}):",
        manifest.learning_mode_profiles.len()
    );
    for p in &manifest.learning_mode_profiles {
        println!(
            "  {} → {} opt_in={} blocks_first_work={}",
            p.record_id,
            p.verdict.as_str(),
            p.opt_in_only,
            p.blocks_first_useful_work
        );
    }

    println!(
        "\nProgress snapshots ({}):",
        manifest.progress_snapshots.len()
    );
    for s in &manifest.progress_snapshots {
        println!(
            "  {} → {} local_by_default={}",
            s.record_id,
            s.verdict.as_str(),
            s.privacy.progress_local_by_default
        );
    }

    println!(
        "\nTeaching sessions ({}):",
        manifest.teaching_sessions.len()
    );
    for s in &manifest.teaching_sessions {
        println!(
            "  {} → {} speaker_notes={} restore={}",
            s.record_id,
            s.verdict.as_str(),
            s.speaker_note_locality.as_str(),
            if s.restore_proof.layout_restored_on_exit {
                "proved"
            } else {
                "missing"
            }
        );
    }

    // Print schema and doc refs.
    println!("\nContract refs:");
    for (k, v) in &manifest.contract_refs {
        println!("  {k}: {v}");
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ All surfaces qualify Stable.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more surfaces narrowed below Stable.",
            manifest.overall_verdict.as_str()
        );
    }
}
