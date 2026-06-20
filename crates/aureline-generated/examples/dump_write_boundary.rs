//! Dumps the write-boundary packet and fixture corpus.
//!
//! With no argument it prints `{ "packet": ..., "fixtures": [...] }` for
//! human inspection. Pass `packet` to print only the proof packet (the form
//! written to `artifacts/generated/write-boundary-packet.json`) or `fixtures`
//! to print only the fixture corpus.

use aureline_generated::{seeded_write_boundary_fixtures, seeded_write_boundary_packet};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let value = match mode.as_str() {
        "packet" => serde_json::to_value(seeded_write_boundary_packet()),
        "fixtures" => serde_json::to_value(seeded_write_boundary_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "packet": seeded_write_boundary_packet(),
            "fixtures": seeded_write_boundary_fixtures(),
        })),
    }
    .expect("write-boundary packet and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
