//! Dumps the canonical M5 hold/retention truth packet as YAML.
//!
//! Run with `cargo run -p aureline-records --example dump_m5_records_policy_sim`
//! and write the output to
//! `fixtures/governance/m5_records_policy_sim/canonical_packet.yaml` whenever the
//! seeded packet changes so the checked-in fixture stays in sync.

use aureline_records::m5_records_policy::seeded_m5_records_policy_packet;

fn main() {
    let packet = seeded_m5_records_policy_packet();
    let yaml = serde_yaml::to_string(&packet).expect("packet serializes to YAML");
    print!("{yaml}");
}
