use std::env;

use aureline_search::{
    seeded_limited_m5_search_navigation_certification_packet,
    seeded_m5_search_navigation_certification_packet,
    seeded_retest_pending_m5_search_navigation_certification_packet,
    seeded_unsupported_m5_search_navigation_certification_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_m5_search_navigation_certification_packet(),
        Some("retest_pending") => seeded_retest_pending_m5_search_navigation_certification_packet(),
        Some("limited") => seeded_limited_m5_search_navigation_certification_packet(),
        Some("unsupported") => seeded_unsupported_m5_search_navigation_certification_packet(),
        Some(other) => panic!(
            "unsupported mode {other}; expected canonical, retest_pending, limited, or unsupported"
        ),
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize certification packet")
    );
}
