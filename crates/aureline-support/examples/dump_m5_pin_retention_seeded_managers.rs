//! Composes the seeded pin / retention manager signals through the frozen
//! artifact-family matrix and prints each manager as a YAML document. Used to
//! regenerate the scenario corpus under
//! `fixtures/storage/m5_pin_retention_cases/`. Each document is preceded by a
//! `# FIXTURE: <path>` marker line so the corpus can never drift from the
//! composer.

use aureline_support::m5_pin_retention::{compose_manager, seeded_manager_signals};
use aureline_support::m5_storage_governance::current_m5_artifact_family_storage_matrix;

const FIXTURE_PATHS: &[&str] = &[
    "fixtures/storage/m5_pin_retention_cases/evidence_and_checkpoint_pins.yaml",
    "fixtures/storage/m5_pin_retention_cases/offline_packs_and_certified_templates.yaml",
    "fixtures/storage/m5_pin_retention_cases/cleanup_history_blocked_by_pins.yaml",
    "fixtures/storage/m5_pin_retention_cases/managed_quota_preserves_user_owned_state.yaml",
];

fn main() {
    let matrix = current_m5_artifact_family_storage_matrix().expect("matrix parses");
    for (signal, path) in seeded_manager_signals().iter().zip(FIXTURE_PATHS) {
        let manager = compose_manager(&matrix, signal);
        let yaml = serde_yaml::to_string(&manager).expect("serialize manager");
        println!("# FIXTURE: {path}");
        print!("{yaml}");
        println!("# END");
    }
}
