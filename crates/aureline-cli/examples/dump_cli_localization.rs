//! Emits the seeded CLI/help localization posture packet and its projections.
//!
//! ```sh
//! cargo run -q -p aureline-cli --example dump_cli_localization -- packet
//! cargo run -q -p aureline-cli --example dump_cli_localization -- support-export
//! cargo run -q -p aureline-cli --example dump_cli_localization -- parity
//! cargo run -q -p aureline-cli --example dump_cli_localization -- render es-MX
//! ```

use aureline_cli::seeded_cli_localization_packet;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_cli_localization_packet();
    packet
        .validate()
        .map_err(|findings| format!("seeded packet failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("support-export") => print_json(&packet.support_export)?,
        Some("parity") => print_json(&packet.parity_report())?,
        Some("render") => {
            let locale = args.get(1).map(String::as_str).unwrap_or("en-US");
            print_json(&packet.render(locale))?;
        }
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
