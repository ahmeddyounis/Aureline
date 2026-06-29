//! Headless emitter for the M5 claim-narrowing registry.
//!
//! The bin is the only mint-from-truth path for the published claim-narrowing registry checked
//! in at `artifacts/public-truth/m5-claim-narrowing.json`, the release-grade parity proof under
//! `artifacts/release/m5-descriptor-parity-proof/claim-narrowing.json` (and its Markdown
//! summary), and the per-state consumer fixtures under `fixtures/public-truth/m5-badge-consumers/`.
//! Each case takes an underlying evidence condition (a descriptor object) and projects the one
//! controlled degraded-claim state it implies onto every public-truth consumer — release/help,
//! marketplace, docs/help, certification, evaluation packs, support exports, and companion
//! handoffs — so a stale or narrowed supporting descriptor cannot leave any surface green by
//! accident, every consumer converges on the same state, and the downgrade reason and what would
//! restore it stay inspectable.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_claim_narrowing -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_claim_narrowing -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_claim_narrowing -- case <case-id>
//! cargo run -q -p aureline-release --bin aureline_release_m5_claim_narrowing -- validate
//! ```

use aureline_release::m5_claim_narrowing::{
    seeded_m5_claim_narrowing_registry, ClaimNarrowingCase, M5ClaimNarrowingRegistry,
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
            let registry = seeded_m5_claim_narrowing_registry();
            assert_registry_valid(&registry)?;
            println!("{}", registry.export_safe_json());
        }
        Some("markdown") => {
            let registry = seeded_m5_claim_narrowing_registry();
            assert_registry_valid(&registry)?;
            print!("{}", registry.render_markdown_summary());
        }
        Some("case") => {
            let case = parse_case(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_case_valid(&case)?;
            println!("{}", case.export_safe_json());
        }
        Some("validate") => {
            let registry = seeded_m5_claim_narrowing_registry();
            assert_registry_valid(&registry)?;
            println!(
                "ok: claim-narrowing registry valid ({} cases, {} converged projections)",
                registry.cases.len(),
                registry.summary.converged_projections
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_case(token: &str) -> Result<ClaimNarrowingCase, Box<dyn std::error::Error>> {
    seeded_m5_claim_narrowing_registry()
        .cases
        .into_iter()
        .find(|c| c.case_id == token)
        .ok_or_else(|| format!("unknown case id: {token}").into())
}

fn assert_registry_valid(
    registry: &M5ClaimNarrowingRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = registry.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("registry failed validation: {}", tokens.join(",")).into())
}

fn assert_case_valid(case: &ClaimNarrowingCase) -> Result<(), Box<dyn std::error::Error>> {
    let violations = case.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("case failed validation: {}", tokens.join(",")).into())
}
