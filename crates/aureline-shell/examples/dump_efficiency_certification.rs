//! Conformance dump for the M5 efficiency certification proof packet.
//!
//! Emits the canonical proof packet the certification lane produces from the
//! seeded energy/thermal traces and session-pressure postures: one certification
//! row per claimed laptop/desktop profile and long-running M5 surface family,
//! each with its drill results, evidence freshness, narrowed effective posture,
//! and the recomputed promotion gate. The output backs the checked-in artifact at
//! `artifacts/efficiency/m5-efficiency-proof-packet.json`, so the certification
//! evidence provably derives from the same efficiency-state objects the rest of
//! the low-power contract uses.

use aureline_shell::efficiency::certification::seeded_proof_packet;

fn main() {
    let packet = seeded_proof_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("efficiency proof packet serializes")
    );
}
