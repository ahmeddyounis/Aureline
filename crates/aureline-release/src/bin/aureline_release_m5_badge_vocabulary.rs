//! Headless emitter for the M5 badge vocabulary.
//!
//! The bin is the only mint-from-truth path for the published badge-vocabulary inventory at
//! `artifacts/public-truth/m5-badge-vocabulary.json`, the Markdown drawer catalog at
//! `artifacts/public-truth/m5-badge-vocabulary-governance.md`, the release parity proof under
//! `artifacts/release/m5-descriptor-parity-proof/`, and the consumer-render fixtures under
//! `fixtures/public-truth/m5-badge-consumers/`. The packet resolves every controlled-enum
//! value behind a provenance / freshness / qualification / client-scope badge to one
//! export-safe id, one user-facing label, and one explanation drawer that every public-truth
//! surface renders identically.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- family <token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- badge <badge_id>
//! cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- term <user_facing_term>
//! cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- validate
//! ```

use aureline_release::m5_badge_vocabulary::{seeded_m5_badge_vocabulary, M5BadgeVocabulary};

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
            let packet = seeded_m5_badge_vocabulary();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!("{}", seeded_m5_badge_vocabulary().render_markdown_summary());
        }
        Some("family") => {
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let packet = seeded_m5_badge_vocabulary();
            let group = packet
                .families
                .iter()
                .find(|g| g.badge_family.as_str() == token)
                .ok_or_else(|| format!("unknown badge family token: {token}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(group).expect("family serializes")
            );
        }
        Some("badge") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let packet = seeded_m5_badge_vocabulary();
            let entry = packet
                .badge(id)
                .ok_or_else(|| format!("unknown badge id: {id}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(entry).expect("badge serializes")
            );
        }
        Some("term") => {
            let term = args.get(1).map(String::as_str).unwrap_or("");
            let packet = seeded_m5_badge_vocabulary();
            let entry = packet
                .badge_for_term(term)
                .ok_or_else(|| format!("unknown user-facing term: {term}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(entry).expect("badge serializes")
            );
        }
        Some("validate") => {
            let packet = seeded_m5_badge_vocabulary();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5BadgeVocabulary) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("badge vocabulary failed validation: {}", tokens.join(",")).into())
    }
}
