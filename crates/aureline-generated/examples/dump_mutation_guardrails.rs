//! Dumps the mutation-guardrails packet and fixture corpus.
//!
//! With no argument it prints `{ "packet": ..., "fixtures": [...] }` for
//! human inspection. Pass `packet` to print only the proof packet (the form
//! written to `artifacts/generated/mutation-guardrails-packet.json`) or
//! `fixtures` to print only the fixture corpus.

use aureline_generated::{seeded_mutation_guardrails_fixtures, seeded_mutation_guardrails_packet};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let value = match mode.as_str() {
        "packet" => serde_json::to_value(seeded_mutation_guardrails_packet()),
        "fixtures" => serde_json::to_value(seeded_mutation_guardrails_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "packet": seeded_mutation_guardrails_packet(),
            "fixtures": seeded_mutation_guardrails_fixtures(),
        })),
    }
    .expect("mutation-guardrails packet and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
