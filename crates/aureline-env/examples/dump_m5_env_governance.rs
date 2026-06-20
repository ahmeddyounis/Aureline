//! Dumps the environment-governance packet and fixture corpus.
//!
//! With no argument it prints `{ "packet": ..., "fixtures": [...] }` for
//! human inspection. Pass `packet` to print only the proof packet (the
//! form written to `artifacts/env/m5-env-proof-packet.json`) or
//! `fixtures` to print only the fixture corpus.

use aureline_env::{seeded_m5_env_governance_fixtures, seeded_m5_env_governance_packet};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let value = match mode.as_str() {
        "packet" => serde_json::to_value(seeded_m5_env_governance_packet()),
        "fixtures" => serde_json::to_value(seeded_m5_env_governance_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "packet": seeded_m5_env_governance_packet(),
            "fixtures": seeded_m5_env_governance_fixtures(),
        })),
    }
    .expect("environment-governance packet and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
