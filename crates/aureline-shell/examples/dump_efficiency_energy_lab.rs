//! Conformance dump for the energy/thermal efficiency lab.
//!
//! Emits, for every claimed M5 desktop profile, the injected pressure schedule
//! together with the lab trace, Project Doctor report, and support export the
//! canonical efficiency-state runtime produces from it. The output backs the
//! checked-in fixtures under `fixtures/efficiency/lab/` and the exported trace
//! artifacts under `artifacts/efficiency/m5-efficiency-traces/` so the lab
//! evidence, Doctor parity, and support packets provably derive from the same
//! efficiency-state objects.

use aureline_shell::efficiency::energy_lab::seeded_lab_cases;

fn main() {
    let cases = seeded_lab_cases();
    println!(
        "{}",
        serde_json::to_string_pretty(&cases).expect("efficiency lab cases serialize")
    );
}
