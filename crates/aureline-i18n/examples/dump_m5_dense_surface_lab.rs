//! Emits the seeded M5 dense-surface i18n qualification packet and projections.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- qualification
//! cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- review
//! cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- narrowing
//! cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- surfaces
//! cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- profiles
//! cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- summary
//! ```

use aureline_i18n::{
    seeded_m5_dense_surface_i18n_qualification, seeded_m5_dense_surface_i18n_review_packet,
    seeded_m5_dense_surface_narrowing_scenarios,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_m5_dense_surface_i18n_qualification();
    packet
        .validate()
        .map_err(|findings| format!("seeded qualification failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("qualification") | None => print_json(&packet)?,
        Some("review") => print_json(&seeded_m5_dense_surface_i18n_review_packet())?,
        Some("narrowing") => print_json(&seeded_m5_dense_surface_narrowing_scenarios())?,
        Some("surfaces") => print_json(&packet.surfaces)?,
        Some("profiles") => print_json(&packet.profile_qualifications)?,
        Some("summary") => print_json(&packet.summary)?,
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
