//! Emits the seeded localized-profile matrix packet and its projections.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_localized_profile_matrix -- packet
//! cargo run -q -p aureline-i18n --example dump_localized_profile_matrix -- surfaces
//! cargo run -q -p aureline-i18n --example dump_localized_profile_matrix -- profiles
//! cargo run -q -p aureline-i18n --example dump_localized_profile_matrix -- coverage
//! cargo run -q -p aureline-i18n --example dump_localized_profile_matrix -- summary
//! ```

use aureline_i18n::seeded_localized_profile_matrix_packet;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_localized_profile_matrix_packet();
    packet
        .validate()
        .map_err(|findings| format!("seeded matrix failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("surfaces") => print_json(&packet.surface_inventory)?,
        Some("profiles") => print_json(&packet.localized_profiles)?,
        Some("coverage") => print_json(&packet.profile_surface_coverage)?,
        Some("summary") => print_json(&packet.summary)?,
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
