use std::env;

use aureline_search::{
    seeded_navigation_continuity_packet, seeded_workset_drift_navigation_continuity_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_navigation_continuity_packet(),
        Some("workset_drift") => seeded_workset_drift_navigation_continuity_packet(),
        Some(other) => {
            panic!("unsupported mode {other}; expected canonical or workset_drift")
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize navigation continuity packet")
    );
}
