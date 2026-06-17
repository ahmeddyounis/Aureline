use std::env;

use aureline_search::{
    seeded_partial_index_stale_ranking_explainability_packet, seeded_ranking_explainability_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_ranking_explainability_packet(),
        Some("partial_index_stale") => seeded_partial_index_stale_ranking_explainability_packet(),
        Some(other) => {
            panic!("unsupported mode {other}; expected canonical or partial_index_stale")
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize ranking-explainability packet")
    );
}
