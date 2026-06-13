//! Dumps the canonical M5 evidence-chronology packet as YAML.
//!
//! Run with
//! `cargo run -p aureline-chronology --example dump_m5_evidence_chronology_lineage`
//! and write the output to
//! `fixtures/governance/m5_evidence_chronology_lineage/canonical_packet.yaml`
//! whenever the seeded packet changes so the checked-in fixture stays in sync.

use aureline_chronology::m5_evidence_chronology_lineage::seeded_m5_evidence_chronology_packet;

fn main() {
    let packet = seeded_m5_evidence_chronology_packet();
    let yaml = serde_yaml::to_string(&packet).expect("packet serializes to YAML");
    print!("{yaml}");
}
