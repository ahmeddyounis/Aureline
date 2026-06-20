//! Dumps the reactive-state certification packet and fixture corpus.
//!
//! With no argument it prints `{ "packet": ..., "fixtures": [...] }` for
//! human inspection. Pass `packet` to print only the proof packet (the
//! form written to `artifacts/state/m5-reactive-proof-packet.json`) or
//! `fixtures` to print only the fixture corpus.

use aureline_reactive_state::{
    seeded_m5_reactive_certification_fixtures, seeded_m5_reactive_certification_packet,
};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let value = match mode.as_str() {
        "packet" => serde_json::to_value(seeded_m5_reactive_certification_packet()),
        "fixtures" => serde_json::to_value(seeded_m5_reactive_certification_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "packet": seeded_m5_reactive_certification_packet(),
            "fixtures": seeded_m5_reactive_certification_fixtures(),
        })),
    }
    .expect("certification packet and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
