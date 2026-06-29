//! Headless emitter for the M5 client-scope-card registry.
//!
//! The bin is the only mint-from-truth path for the published client-scope-card registry checked in
//! at `artifacts/public-truth/m5-client-scope-card.json`, the release-grade parity proof under
//! `artifacts/release/m5-descriptor-parity-proof/client-scope-card.json` (and its Markdown summary),
//! and the per-surface consumer fixtures under `fixtures/public-truth/m5-badge-consumers/`. Each card
//! takes one client-scope descriptor, derives its granted capabilities, blocked actions, parity
//! caveats, claim state, and disclosures, and projects them onto the discovery, deep-link, handoff,
//! and companion surfaces — so a narrowed client states its scope and authority before a user
//! discovers a limit by failing, and can never imply desktop parity.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_client_scope_card -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_client_scope_card -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_client_scope_card -- card <card-id>
//! cargo run -q -p aureline-release --bin aureline_release_m5_client_scope_card -- validate
//! ```

use aureline_release::m5_client_scope_card::{
    seeded_m5_client_scope_card_registry, ClientScopeCard, M5ClientScopeCardRegistry,
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
            let registry = seeded_m5_client_scope_card_registry();
            assert_registry_valid(&registry)?;
            println!("{}", registry.export_safe_json());
        }
        Some("markdown") => {
            let registry = seeded_m5_client_scope_card_registry();
            assert_registry_valid(&registry)?;
            print!("{}", registry.render_markdown_summary());
        }
        Some("card") => {
            let card = parse_card(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_card_valid(&card)?;
            println!("{}", card.export_safe_json());
        }
        Some("validate") => {
            let registry = seeded_m5_client_scope_card_registry();
            assert_registry_valid(&registry)?;
            println!(
                "ok: client-scope-card registry valid ({} cards, {} disclosure projections)",
                registry.cards.len(),
                registry.summary.total_disclosure_projections
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_card(token: &str) -> Result<ClientScopeCard, Box<dyn std::error::Error>> {
    seeded_m5_client_scope_card_registry()
        .cards
        .into_iter()
        .find(|c| c.card_id == token)
        .ok_or_else(|| format!("unknown card id: {token}").into())
}

fn assert_registry_valid(
    registry: &M5ClientScopeCardRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = registry.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("registry failed validation: {}", tokens.join(",")).into())
}

fn assert_card_valid(card: &ClientScopeCard) -> Result<(), Box<dyn std::error::Error>> {
    let violations = card.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("card failed validation: {}", tokens.join(",")).into())
}
