use std::env;

use aureline_support::{
    seeded_m5_fault_crash_certification_packet,
    seeded_stale_schema_m5_fault_crash_certification_packet,
    seeded_stale_symbolication_m5_fault_crash_certification_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_m5_fault_crash_certification_packet(),
        Some("stale_symbolication") => {
            seeded_stale_symbolication_m5_fault_crash_certification_packet()
        }
        Some("stale_schema") => seeded_stale_schema_m5_fault_crash_certification_packet(),
        Some(other) => {
            panic!(
                "unsupported mode {other}; expected canonical, stale_symbolication, or stale_schema"
            )
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize certification packet")
    );
}
