//! Dumps the canonical M5 policy-impact simulation packet as YAML.
//!
//! Run with
//! `cargo run -p aureline-records --example dump_m5_policy_impact_simulation`
//! and write the output to
//! `fixtures/governance/m5_policy_impact_simulation/canonical_packet.yaml`
//! whenever the seeded packet changes so the checked-in fixture stays in sync.

use aureline_records::m5_policy_simulation::seeded_m5_policy_simulation_packet;

fn main() {
    let packet = seeded_m5_policy_simulation_packet();
    let yaml = serde_yaml::to_string(&packet).expect("packet serializes to YAML");
    print!("{yaml}");
}
