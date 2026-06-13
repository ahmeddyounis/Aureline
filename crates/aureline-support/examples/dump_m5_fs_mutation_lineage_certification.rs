use std::env;

use aureline_support::{
    seeded_m5_fs_mutation_lineage_certification_packet,
    seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_m5_fs_mutation_lineage_certification_packet(),
        Some("missing_recovery_linkage") => {
            seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet()
        }
        Some(other) => {
            panic!("unsupported mode {other}; expected canonical or missing_recovery_linkage")
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize certification packet")
    );
}
