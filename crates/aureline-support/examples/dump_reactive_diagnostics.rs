//! Dumps the seeded reactive-diagnostics packet and troubleshooting fixtures.
//!
//! Used to regenerate the checked-in artifact packet and fixture corpus under
//! `artifacts/support/reactive_diagnostics.json` and
//! `fixtures/support/reactive_diagnostics/`.

use aureline_support::{seeded_reactive_diagnostics_fixtures, seeded_reactive_diagnostics_packet};

fn main() {
    let packet = seeded_reactive_diagnostics_packet();
    let fixtures = seeded_reactive_diagnostics_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
