//! Headless inspector for the durable-notification routing corpus.
//!
//! Emits the same packet consumed by the corpus fixture replay test, the
//! privacy/route audit, the support route/outcome export report, and the
//! conformance doc. It is the only mint-from-truth path for
//! `fixtures/ux/m3/notification_envelope_corpus/`,
//! `artifacts/ux/m3/notification_privacy_and_route_audit.md`,
//! `artifacts/support/m3/notification_route_outcome_export_report.md`, and
//! `docs/ux/m3/notification_route_conformance.md`, so the JSON and the
//! markdown cannot drift from the Rust types.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- drift-drills
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- badge-probes
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- overlay-parity
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- audit-md
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- export-report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- conformance-md
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- validate
//! ```

use aureline_shell::notification_envelope_corpus::{
    render_notification_privacy_and_route_audit_markdown,
    render_notification_route_conformance_markdown,
    render_notification_route_outcome_export_report_markdown,
    seeded_notification_envelope_corpus_packet,
    validate_notification_envelope_corpus_packet_summary,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_notification_envelope_corpus_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("cases") => print_json(&packet.cases)?,
        Some("drift-drills") => print_json(&packet.drift_drills)?,
        Some("badge-probes") => print_json(&packet.badge_probes)?,
        Some("overlay-parity") => print_json(&packet.overlay_parity)?,
        Some("support-export") => print_json(&packet.support_export)?,
        Some("audit-md") => print!(
            "{}",
            render_notification_privacy_and_route_audit_markdown(&packet)
        ),
        Some("export-report-md") => print!(
            "{}",
            render_notification_route_outcome_export_report_markdown(&packet)
        ),
        Some("conformance-md") => {
            print!("{}", render_notification_route_conformance_markdown(&packet))
        }
        Some("validate") => match validate_notification_envelope_corpus_packet_summary(&packet) {
            Ok(summary) => println!("{summary}"),
            Err(errors) => {
                for err in errors {
                    eprintln!("corpus error: {err}");
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
