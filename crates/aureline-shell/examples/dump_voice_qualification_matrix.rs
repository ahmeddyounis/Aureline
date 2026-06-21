//! Conformance dump for the M5 voice-qualification matrix packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), a standalone provider descriptor (`provider`), or a
//! standalone session state (`session`), so the checked-in artifacts stay
//! byte-aligned with the in-crate builder.

use aureline_shell::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_voice_qualification_matrix_packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    match which.as_str() {
        "summary" => print!("{}", packet.render_markdown_summary()),
        "provider" => {
            let provider = &packet
                .row("voice-qual:command-overlay:local:0001")
                .expect("seed row")
                .provider;
            assert!(provider.is_well_formed(), "provider must be well formed");
            println!(
                "{}",
                serde_json::to_string_pretty(provider).expect("provider serializes")
            );
        }
        "session" => {
            let session = &packet
                .row("voice-qual:dictation-input:local:0001")
                .expect("seed row")
                .session;
            assert!(session.is_well_formed(), "session must be well formed");
            println!(
                "{}",
                serde_json::to_string_pretty(session).expect("session serializes")
            );
        }
        _ => println!("{}", packet.export_safe_json()),
    }
}
