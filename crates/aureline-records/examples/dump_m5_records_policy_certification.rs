//! Dumps the canonical M5 record-governance certification packet as YAML.
//!
//! Run with
//! `cargo run -p aureline-records --example dump_m5_records_policy_certification`
//! and write the output to
//! `fixtures/governance/m5_records_policy_certification/canonical_packet.yaml`
//! whenever the seeded packet changes so the checked-in fixture stays in sync.

use aureline_records::m5_records_policy_certification::seeded_m5_records_policy_certification_packet;

fn main() {
    let packet = seeded_m5_records_policy_certification_packet();
    let yaml = serde_yaml::to_string(&packet).expect("packet serializes to YAML");
    print!("{yaml}");
}
