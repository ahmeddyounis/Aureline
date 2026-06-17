use std::env;

use aureline_search::{seeded_redacted_export_packet, seeded_saved_query_governance_packet};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_saved_query_governance_packet(),
        Some("redacted") => seeded_redacted_export_packet(),
        Some(other) => {
            panic!("unsupported mode {other}; expected canonical or redacted")
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize saved-query governance packet")
    );
}
