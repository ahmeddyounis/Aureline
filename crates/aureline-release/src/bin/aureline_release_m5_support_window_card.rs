//! Headless emitter for the support-window card set.
//!
//! The bin is the only mint-from-truth path for the published card-set inventory checked in at
//! `artifacts/release/m5-support-window-cards.json`, the release-grade channel-lifecycle parity proof
//! under `artifacts/release/m5-channel-lifecycle-proof/` (and its Markdown report), the
//! machine-readable per-card CSV export at `artifacts/release/m5-support-window-cards.csv`, and the
//! per-state card-set fixtures under `fixtures/release/support-window-and-eos/`. It surfaces, per
//! channel, the channel identity, support window, overlap window, deprecation horizon, removal target,
//! pin-or-postpone path, and compatibility caveats, and, per claimed subject — workspace/profile
//! files, extension SDKs/manifests, remote helpers, and public schemas — the end-of-support and
//! compatibility-window posture, without broadening any support commitment beyond what is claimed.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- variant <canonical|deprecation|end-of-support|subject-compat>
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- validate
//! ```

use aureline_release::m5_support_window_card::{
    seeded_m5_support_window_card_set, seeded_m5_support_window_card_set_deprecation,
    seeded_m5_support_window_card_set_end_of_support,
    seeded_m5_support_window_card_set_subject_compat, SupportWindowCardSet,
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
            let packet = seeded_m5_support_window_card_set();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_support_window_card_set();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_support_window_card_set();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_card_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_support_window_card_set();
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
            let packet = seeded_m5_support_window_card_set();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_support_window_card_set_deprecation())?;
            assert_packet_valid(&seeded_m5_support_window_card_set_end_of_support())?;
            assert_packet_valid(&seeded_m5_support_window_card_set_subject_compat())?;
            println!(
                "ok: support-window cards valid ({} channels, {} subjects, {} consumers)",
                packet.channels.len(),
                packet.subjects.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<SupportWindowCardSet, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_support_window_card_set()),
        "deprecation" => Ok(seeded_m5_support_window_card_set_deprecation()),
        "end-of-support" => Ok(seeded_m5_support_window_card_set_end_of_support()),
        "subject-compat" => Ok(seeded_m5_support_window_card_set_subject_compat()),
        other => Err(format!(
            "unknown variant: {other} (canonical|deprecation|end-of-support|subject-compat)"
        )
        .into()),
    }
}

fn assert_packet_valid(packet: &SupportWindowCardSet) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
