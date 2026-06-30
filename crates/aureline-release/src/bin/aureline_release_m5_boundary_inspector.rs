//! Headless emitter for the M5 boundary inspector.
//!
//! The bin is the only mint-from-truth path for the published boundary-inspector inventory checked in
//! at `artifacts/public-truth/m5-boundary-inspector.json`, the rendered overview document at
//! `artifacts/public-truth/m5-boundary-inspector.md`, the machine-readable action / facet matrix at
//! `artifacts/public-truth/m5-boundary-inspector-actions.csv`, the release-grade parity proof under
//! `artifacts/public-truth/m5-boundary-inspector-proof/` (and its Markdown report), the exported
//! evaluation packet, and the per-state drill fixtures under
//! `fixtures/public-truth/m5-boundary-inspector/`. The inspector explains, for each consequential M5
//! action, where execution and data went, which host / service hops carried it, and which approval
//! authority was in effect, so a card can never read safer than its proof.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- overview
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- evaluation
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- variant <canonical|boundary|route-drift|route-unattributed|approval-expired>
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- action <action-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_boundary_inspector -- validate
//! ```

use aureline_release::m5_boundary_inspector::{
    seeded_m5_boundary_inspector, seeded_m5_boundary_inspector_approval_expired_blocked,
    seeded_m5_boundary_inspector_boundary_narrowed,
    seeded_m5_boundary_inspector_route_drift_narrowed,
    seeded_m5_boundary_inspector_route_unattributed_blocked, M5BoundaryInspector,
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
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("overview") => {
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_overview_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_actions_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("evaluation") => {
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            println!("{}", packet.render_evaluation_packet());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("action") => {
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let inspector = packet
                .action_inspectors
                .iter()
                .find(|i| i.action.as_str() == token)
                .ok_or_else(|| format!("unknown action token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(inspector)?);
        }
        Some("validate") => {
            let packet = seeded_m5_boundary_inspector();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_boundary_inspector_boundary_narrowed())?;
            assert_packet_valid(&seeded_m5_boundary_inspector_route_drift_narrowed())?;
            assert_packet_valid(&seeded_m5_boundary_inspector_route_unattributed_blocked())?;
            assert_packet_valid(&seeded_m5_boundary_inspector_approval_expired_blocked())?;
            println!(
                "ok: boundary inspector valid ({} actions)",
                packet.action_inspectors.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5BoundaryInspector, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_boundary_inspector()),
        "boundary" => Ok(seeded_m5_boundary_inspector_boundary_narrowed()),
        "route-drift" => Ok(seeded_m5_boundary_inspector_route_drift_narrowed()),
        "route-unattributed" => Ok(seeded_m5_boundary_inspector_route_unattributed_blocked()),
        "approval-expired" => Ok(seeded_m5_boundary_inspector_approval_expired_blocked()),
        other => Err(format!(
            "unknown variant: {other} (canonical|boundary|route-drift|route-unattributed|approval-expired)"
        )
        .into()),
    }
}

fn assert_packet_valid(packet: &M5BoundaryInspector) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
