use std::env;

use aureline_support::{
    seeded_blurred_cache_authority_m5_storage_certification_packet,
    seeded_m5_storage_certification_packet,
    seeded_stale_pin_retention_m5_storage_certification_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_m5_storage_certification_packet(),
        Some("stale_pin_retention") => seeded_stale_pin_retention_m5_storage_certification_packet(),
        Some("blurred_cache_authority") => {
            seeded_blurred_cache_authority_m5_storage_certification_packet()
        }
        Some(other) => {
            panic!(
                "unsupported mode {other}; expected canonical, stale_pin_retention, or blurred_cache_authority"
            )
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize certification packet")
    );
}
