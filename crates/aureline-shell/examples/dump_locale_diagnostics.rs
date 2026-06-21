//! Emits the seeded locale diagnostics packet and its projections.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_locale_diagnostics -- packet
//! cargo run -q -p aureline-shell --example dump_locale_diagnostics -- support-export
//! cargo run -q -p aureline-shell --example dump_locale_diagnostics -- help-about
//! cargo run -q -p aureline-shell --example dump_locale_diagnostics -- release-gate
//! ```

use aureline_shell::i18n::seeded_locale_diagnostics_packet;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_locale_diagnostics_packet();
    packet
        .validate()
        .map_err(|findings| format!("seeded packet failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("support-export") => print_json(&packet.support_export)?,
        Some("help-about") => print_json(&packet.help_about_card)?,
        Some("release-gate") => print_json(&packet.release_gate)?,
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
