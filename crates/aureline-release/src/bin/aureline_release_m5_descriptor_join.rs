//! Headless emitter for the M5 descriptor-join registry.
//!
//! The bin is the only mint-from-truth path for the published descriptor-join registry checked in
//! at `artifacts/public-truth/m5-descriptor-join.json`, the release-grade parity proof under
//! `artifacts/release/m5-descriptor-parity-proof/descriptor-join.json` (and its Markdown summary),
//! and the per-state carrier fixtures under `fixtures/public-truth/m5-badge-consumers/`. Each join
//! takes a descriptor object and projects it into the copy-safe carrier shapes the support, admin,
//! and reporting paths emit — an export packet, a support bundle, an admin report, and a copy-safe
//! summary — so the descriptor identity, the typed artifact binding, and the inspectable downgrade
//! reasons survive copy/export instead of collapsing to flat text.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_join -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_join -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_join -- join <join-id>
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_join -- validate
//! ```

use aureline_release::m5_descriptor_join::{
    seeded_m5_descriptor_join_registry, DescriptorJoin, M5DescriptorJoinRegistry,
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
            let registry = seeded_m5_descriptor_join_registry();
            assert_registry_valid(&registry)?;
            println!("{}", registry.export_safe_json());
        }
        Some("markdown") => {
            let registry = seeded_m5_descriptor_join_registry();
            assert_registry_valid(&registry)?;
            print!("{}", registry.render_markdown_summary());
        }
        Some("join") => {
            let join = parse_join(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_join_valid(&join)?;
            println!("{}", join.export_safe_json());
        }
        Some("validate") => {
            let registry = seeded_m5_descriptor_join_registry();
            assert_registry_valid(&registry)?;
            println!(
                "ok: descriptor-join registry valid ({} joins, {} carrier renderings)",
                registry.joins.len(),
                registry.summary.total_carrier_renderings
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_join(token: &str) -> Result<DescriptorJoin, Box<dyn std::error::Error>> {
    seeded_m5_descriptor_join_registry()
        .joins
        .into_iter()
        .find(|j| j.join_id == token)
        .ok_or_else(|| format!("unknown join id: {token}").into())
}

fn assert_registry_valid(
    registry: &M5DescriptorJoinRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = registry.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("registry failed validation: {}", tokens.join(",")).into())
}

fn assert_join_valid(join: &DescriptorJoin) -> Result<(), Box<dyn std::error::Error>> {
    let violations = join.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("join failed validation: {}", tokens.join(",")).into())
}
