//! Headless emitter for the M5 no-silent-omission guard registry.
//!
//! The bin is the only mint-from-truth path for the published omission-guard registry checked in
//! at `artifacts/public-truth/m5-omission-guard.json`, the release-grade parity proof under
//! `artifacts/release/m5-descriptor-parity-proof/omission-guard.json` (and its Markdown summary),
//! and the per-condition consumer fixtures under `fixtures/public-truth/m5-badge-consumers/`. Each
//! case takes a descriptor object, derives the set of present weaker-evidence states from its
//! facets, and projects that exact set onto every public-truth consumer — so a Mirrored, Offline,
//! Side-loaded, `not_provided`, partial, or stale condition reads identically wherever it is
//! inspected and can never be silently omitted.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_omission_guard -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_omission_guard -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_omission_guard -- case <case-id>
//! cargo run -q -p aureline-release --bin aureline_release_m5_omission_guard -- validate
//! ```

use aureline_release::m5_omission_guard::{
    seeded_m5_omission_guard_registry, M5OmissionGuardRegistry, OmissionGuardCase,
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
        Some("registry") | None => {
            let registry = seeded_m5_omission_guard_registry();
            assert_registry_valid(&registry)?;
            println!("{}", registry.export_safe_json());
        }
        Some("markdown") => {
            let registry = seeded_m5_omission_guard_registry();
            assert_registry_valid(&registry)?;
            print!("{}", registry.render_markdown_summary());
        }
        Some("case") => {
            let case = parse_case(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_case_valid(&case)?;
            println!("{}", case.export_safe_json());
        }
        Some("validate") => {
            let registry = seeded_m5_omission_guard_registry();
            assert_registry_valid(&registry)?;
            println!(
                "ok: omission-guard registry valid ({} cases, {} state renderings)",
                registry.cases.len(),
                registry.summary.total_state_renderings
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_case(token: &str) -> Result<OmissionGuardCase, Box<dyn std::error::Error>> {
    seeded_m5_omission_guard_registry()
        .cases
        .into_iter()
        .find(|c| c.case_id == token)
        .ok_or_else(|| format!("unknown case id: {token}").into())
}

fn assert_registry_valid(
    registry: &M5OmissionGuardRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = registry.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("registry failed validation: {}", tokens.join(",")).into())
}

fn assert_case_valid(case: &OmissionGuardCase) -> Result<(), Box<dyn std::error::Error>> {
    let violations = case.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("case failed validation: {}", tokens.join(",")).into())
}
