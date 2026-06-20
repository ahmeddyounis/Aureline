//! Conformance dump for the per-surface low-power disclosures.
//!
//! Emits, for every representative posture, the typed inputs together with the
//! per-surface disclosure set the efficiency state produces. The output backs the
//! checked-in fixtures under `fixtures/efficiency/disclosures/` so the disclosures
//! provably derive from the same canonical efficiency-state objects as the status,
//! diagnostics, and support surfaces.

use aureline_shell::efficiency::disclosures::seeded_efficiency_disclosure_cases;

fn main() {
    let cases = seeded_efficiency_disclosure_cases();
    println!(
        "{}",
        serde_json::to_string_pretty(&cases).expect("efficiency disclosure cases serialize")
    );
}
