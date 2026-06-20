//! Conformance dump for the hidden-surface render-suppression policy.
//!
//! Emits, for every representative scenario, the surfaces that requested work
//! together with the suppression audit, energy/thermal trace, and diagnostics
//! projection the policy derives from them. The output backs the checked-in
//! fixtures under `fixtures/efficiency/hidden-pane-audits/` so the suppression
//! audit, energy trace, and diagnostics view provably share one object.

use aureline_shell::efficiency::hidden_surfaces::seeded_hidden_surface_cases;

fn main() {
    let cases = seeded_hidden_surface_cases();
    println!(
        "{}",
        serde_json::to_string_pretty(&cases).expect("hidden-surface cases serialize")
    );
}
