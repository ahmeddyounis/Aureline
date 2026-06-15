//! Composes the seeded cache-repair signals through the canonical runtime
//! storage-class profiles and prints each plan as a YAML document. Used to
//! regenerate the scenario corpus under
//! `fixtures/storage/m5_cache_repair_cases/`. Each document is preceded by a
//! `# FIXTURE: <path>` marker line so the corpus can never drift from the
//! composer.

use aureline_support::m5_cache_repair::{
    compose_plan, current_runtime_profiles, seeded_repair_signals,
};

const FIXTURE_PATHS: &[&str] = &[
    "fixtures/storage/m5_cache_repair_cases/knowledge_cache_corrupt_index_reindex.yaml",
    "fixtures/storage/m5_cache_repair_cases/artifact_pack_checksum_mismatch_refetch.yaml",
    "fixtures/storage/m5_cache_repair_cases/generated_preview_torn_rederive.yaml",
    "fixtures/storage/m5_cache_repair_cases/evidence_trace_corrupt_quarantined_for_review.yaml",
    "fixtures/storage/m5_cache_repair_cases/recovery_state_torn_repair_in_place.yaml",
    "fixtures/storage/m5_cache_repair_cases/prebuild_missing_backing_repair_failed_fallback.yaml",
];

fn main() {
    let profiles = current_runtime_profiles().expect("runtime profiles parse");
    for (signal, path) in seeded_repair_signals().iter().zip(FIXTURE_PATHS) {
        let plan = compose_plan(&profiles, signal);
        let yaml = serde_yaml::to_string(&plan).expect("serialize plan");
        println!("# FIXTURE: {path}");
        print!("{yaml}");
        println!("# END");
    }
}
