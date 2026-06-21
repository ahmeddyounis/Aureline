//! Emits the seeded localized claim-status qualification packet and projections.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_localized_profile_qualification -- packet
//! cargo run -q -p aureline-i18n --example dump_localized_profile_qualification -- profiles
//! cargo run -q -p aureline-i18n --example dump_localized_profile_qualification -- known-limits
//! cargo run -q -p aureline-i18n --example dump_localized_profile_qualification -- summary
//! ```

use aureline_i18n::seeded_localized_claim_status_packet;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_localized_claim_status_packet();
    packet
        .validate()
        .map_err(|findings| format!("seeded claim status failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("profiles") => print_json(&packet.claimed_profiles)?,
        Some("known-limits") => print_json(&packet.known_limits)?,
        Some("summary") => print_json(&packet.summary)?,
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
