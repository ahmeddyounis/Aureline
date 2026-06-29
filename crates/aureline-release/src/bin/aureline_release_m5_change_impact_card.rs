//! Headless emitter for the change-impact card set.
//!
//! The bin is the only mint-from-truth path for the published card-set inventory checked in at
//! `artifacts/release/m5-change-impact-cards.json`, the release-grade parity proof under
//! `artifacts/release/m5-change-impact-proof/` (and its Markdown report), the machine-readable
//! per-card CSV export at `artifacts/release/m5-change-impact-cards.csv`, and the per-state card-set
//! fixtures under `fixtures/release/change-impact/`. It forecasts, before restart, the impact of a
//! staged M5 update across every claimed dimension — workspace / profile / schema / cache migration,
//! extension compatibility, remote-helper skew, toolchain floor / ceiling, certified archetype, and
//! behavior change — and labels speculative inputs honestly instead of raising them as hard failures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- variant <canonical|review|hold|speculative>
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- validate
//! ```

use aureline_release::m5_change_impact_card::{
    seeded_m5_change_impact_card_set, seeded_m5_change_impact_card_set_hold,
    seeded_m5_change_impact_card_set_review, seeded_m5_change_impact_card_set_speculative,
    ChangeImpactCardSet,
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
        Some("registry") | Some("proof") | None => {
            let packet = seeded_m5_change_impact_card_set();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_change_impact_card_set();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_change_impact_card_set();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_card_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_change_impact_card_set();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let consumer = packet
                .consumers
                .iter()
                .find(|c| c.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(consumer)?);
        }
        Some("validate") => {
            let packet = seeded_m5_change_impact_card_set();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_change_impact_card_set_review())?;
            assert_packet_valid(&seeded_m5_change_impact_card_set_hold())?;
            assert_packet_valid(&seeded_m5_change_impact_card_set_speculative())?;
            println!(
                "ok: change-impact cards valid ({} dimensions, {} consumers)",
                packet.cards.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<ChangeImpactCardSet, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_change_impact_card_set()),
        "review" => Ok(seeded_m5_change_impact_card_set_review()),
        "hold" => Ok(seeded_m5_change_impact_card_set_hold()),
        "speculative" => Ok(seeded_m5_change_impact_card_set_speculative()),
        other => {
            Err(format!("unknown variant: {other} (canonical|review|hold|speculative)").into())
        }
    }
}

fn assert_packet_valid(packet: &ChangeImpactCardSet) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
