//! Headless emitter and validator for the versioned tour/glossary package
//! manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the tour/glossary
//! package invariants, and can refresh the on-disk fixture so it stays in sync
//! with the Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_tour_and_glossary_packages -- manifest
//!
//! # Print a plaintext package summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_tour_and_glossary_packages -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_tour_and_glossary_packages -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_tour_and_glossary_packages \
//!   -- emit-fixture fixtures/help/m5/tour-and-glossary-packages/m5_tour_and_glossary_packages.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_tour_and_glossary_packages, validate_m5_tour_and_glossary_packages,
    M5TourAndGlossaryPackageManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_tour_and_glossary_packages();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_tour_and_glossary_packages(&manifest) {
            Ok(()) => {
                println!("ok — all tour/glossary package invariants pass");
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
            let json = serde_json::to_string_pretty(&manifest)?;
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

fn print_summary(manifest: &M5TourAndGlossaryPackageManifest) {
    println!(
        "M5 tour/glossary package manifest: {}",
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
            "  {} → {} [freshness={}, entries={}, locales={}]",
            p.pack_id,
            p.verdict.as_str(),
            p.freshness_state.as_str(),
            p.entries.len(),
            p.locale_overlays.len(),
        );
    }

    println!("\nTour packages ({}):", manifest.tour_packages.len());
    for p in &manifest.tour_packages {
        let widening = p.steps.iter().filter(|s| s.scope_widening.widens).count();
        println!(
            "  {} → {} [freshness={}, steps={}, scope_widening_steps={}, locales={}]",
            p.package_id,
            p.verdict.as_str(),
            p.freshness_state.as_str(),
            p.steps.len(),
            widening,
            p.locale_overlays.len(),
        );
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ Every M5 family ships a Stable, versioned glossary pack and tour package.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more packages narrowed below Stable (cached/local-only, disclosed).",
            manifest.overall_verdict.as_str()
        );
    }
}
