use std::env;

use aureline_support::{
    seeded_consent_drill_stale_m5_supportability_qualification_packet,
    seeded_environment_evidence_stale_m5_supportability_qualification_packet,
    seeded_m5_supportability_qualification_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_m5_supportability_qualification_packet(),
        Some("consent_drill_stale") => {
            seeded_consent_drill_stale_m5_supportability_qualification_packet()
        }
        Some("environment_evidence_stale") => {
            seeded_environment_evidence_stale_m5_supportability_qualification_packet()
        }
        Some(other) => panic!(
            "unsupported mode {other}; expected canonical, consent_drill_stale, or environment_evidence_stale"
        ),
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize qualification packet")
    );
}
