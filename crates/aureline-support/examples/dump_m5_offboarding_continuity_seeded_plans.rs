//! Composes the seeded offboarding continuity requests through the frozen
//! artifact-family matrix and prints each plan as a YAML document. Used to
//! regenerate the scenario corpus under
//! `fixtures/storage/m5_offboarding_continuity_cases/`. Each document is preceded
//! by a `# FIXTURE: <path>` marker line and followed by `# END` so the corpus can
//! never drift from the composer.

use aureline_support::m5_offboarding_continuity::{
    compose_offboarding_plan, seeded_offboarding_requests,
};
use aureline_support::m5_storage_governance::current_m5_artifact_family_storage_matrix;

const FIXTURE_PATHS: &[&str] = &[
    "fixtures/storage/m5_offboarding_continuity_cases/account_offboarding_durable_retained.yaml",
    "fixtures/storage/m5_offboarding_continuity_cases/device_reset_caches_only.yaml",
    "fixtures/storage/m5_offboarding_continuity_cases/offline_certified_policy_pins_retained.yaml",
    "fixtures/storage/m5_offboarding_continuity_cases/offline_bundle_reviewed_away_continuity_warned.yaml",
    "fixtures/storage/m5_offboarding_continuity_cases/workspace_wipe_reviewed_away_export_first.yaml",
];

fn main() {
    let matrix = current_m5_artifact_family_storage_matrix().expect("matrix parses");
    for (request, path) in seeded_offboarding_requests().iter().zip(FIXTURE_PATHS) {
        let plan = compose_offboarding_plan(&matrix, request);
        let yaml = serde_yaml::to_string(&plan).expect("serialize plan");
        println!("# FIXTURE: {path}");
        print!("{yaml}");
        println!("# END");
    }
}
