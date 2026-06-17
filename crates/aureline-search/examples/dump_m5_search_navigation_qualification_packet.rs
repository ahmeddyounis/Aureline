use std::env;

use aureline_search::{
    seeded_m5_search_navigation_qualification_packet,
    seeded_partial_index_stale_m5_search_navigation_qualification_packet,
    seeded_unconsented_query_text_m5_search_navigation_qualification_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_m5_search_navigation_qualification_packet(),
        Some("partial_index_stale") => {
            seeded_partial_index_stale_m5_search_navigation_qualification_packet()
        }
        Some("unconsented_query_text") => {
            seeded_unconsented_query_text_m5_search_navigation_qualification_packet()
        }
        Some(other) => panic!(
            "unsupported mode {other}; expected canonical, partial_index_stale, or unconsented_query_text"
        ),
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize qualification packet")
    );
}
