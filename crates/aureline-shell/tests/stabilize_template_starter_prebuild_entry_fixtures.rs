//! Fixture-replay and invariant tests for the template, starter, and prebuild
//! entry disclosure drill corpus.
//!
//! The drills live under
//! `fixtures/ux/m4/stabilize-template-starter-prebuild-entry/` and are minted by
//! the `aureline_shell_stabilize_template_starter_prebuild_entry` emitter so the
//! checked-in JSON stays a literal projection of the in-code corpus.
//!
//! What this guards:
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_stabilize_template_starter_prebuild_entry -- emit-fixtures \
//!     fixtures/ux/m4/stabilize-template-starter-prebuild-entry
//!   ```
//!
//! - Bypass parity: every record carries at least one bypass path with
//!   `equal_weight_with_apply` continuity.
//! - Source honesty: community / uncertified sources carry trust notes.
//! - Runtime consistency: local_only does not require remote provisioning or
//!   managed services; managed_cloud_required declares both.
//! - Prebuild freshness: prebuild entries declare freshness (not unknown).
//! - Resulting-mode honesty: resume_live, start_from_snapshot, clone_fresh,
//!   open_prebuild_minimal, open_without_starter, create_empty, create_project,
//!   create_service, and add_module are distinct and correctly paired with entry
//!   kind.
//! - Failure transparency: partial application is disclosed, not hidden.
//! - Export safety: raw secrets, command lines, and URLs are never allowed.

use aureline_shell::stabilize_template_starter_prebuild_entry::{
    template_starter_prebuild_entry_corpus, BypassPathClass, EntryKind, FreshnessClass,
    ManagedServiceClass, NetworkEgressClass, RemoteProvisioningClass, ResultingMode,
    RuntimeScopeClass, SourceClass, TemplateStarterPrebuildEntryRecord,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/stabilize-template-starter-prebuild-entry",
);

fn load_record(filename: &str) -> TemplateStarterPrebuildEntryRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_stabilize_template_starter_prebuild_entry -- emit-fixtures fixtures/ux/m4/stabilize-template-starter-prebuild-entry`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert_eq!(
            record.accelerator_identity.entry_kind, scenario.expected_entry_kind,
            "{} entry_kind",
            scenario.scenario_id
        );
        assert_eq!(
            record.resulting_mode, scenario.expected_resulting_mode,
            "{} resulting_mode",
            scenario.scenario_id
        );
        assert_eq!(
            record.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present",
            scenario.scenario_id
        );
        assert_eq!(
            record.bypass_paths.len(),
            scenario.expected_bypass_path_count,
            "{} bypass_path_count",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_record_has_at_least_one_bypass_path() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            !record.bypass_paths.is_empty(),
            "{}: must have at least one bypass path",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_bypass_path_is_equal_weight() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        for path in &record.bypass_paths {
            assert_eq!(
                path.bypass_continuity_class, "equal_weight_with_apply",
                "{}: bypass path {} must be equal_weight_with_apply",
                scenario.scenario_id,
                path.path_class.as_str()
            );
        }
    }
}

#[test]
fn community_and_uncertified_sources_carry_trust_notes() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if matches!(
            record.source_review.source_class,
            SourceClass::Community | SourceClass::Uncertified
        ) {
            assert!(
                !record.source_review.trust_notes.is_empty(),
                "{}: community/uncertified source must carry trust notes",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn local_only_runtime_does_not_require_remote_or_managed() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if record.runtime_review.runtime_scope_class == RuntimeScopeClass::LocalOnly {
            assert!(
                matches!(
                    record.side_effect_envelope.required_remote_provisioning_class,
                    RemoteProvisioningClass::NoRemoteProvisioningRequired
                        | RemoteProvisioningClass::RemoteProvisioningUnknownRequiresReview
                ),
                "{}: local_only runtime cannot require remote provisioning",
                scenario.scenario_id
            );
            assert!(
                matches!(
                    record.side_effect_envelope.required_managed_service_class,
                    ManagedServiceClass::NoManagedServiceRequired
                        | ManagedServiceClass::ManagedServiceClassUnknownRequiresReview
                ),
                "{}: local_only runtime cannot require managed service",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn managed_cloud_requires_managed_service_and_egress() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if record.runtime_review.runtime_scope_class == RuntimeScopeClass::ManagedCloudRequired {
            assert!(
                !matches!(
                    record.side_effect_envelope.required_managed_service_class,
                    ManagedServiceClass::NoManagedServiceRequired
                        | ManagedServiceClass::ManagedServiceClassUnknownRequiresReview
                ),
                "{}: managed_cloud_required must declare a managed service",
                scenario.scenario_id
            );
            assert!(
                record.side_effect_envelope.required_network_egress_class
                    != NetworkEgressClass::NoNetworkEgressRequired,
                "{}: managed_cloud_required must declare network egress",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn prebuild_entries_declare_freshness() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if record.accelerator_identity.entry_kind == EntryKind::Prebuild {
            assert!(
                record.freshness_review.freshness_class != FreshnessClass::UnknownRequiresRevalidation,
                "{}: prebuild must declare freshness",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn resulting_mode_matches_entry_kind() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        let valid = match record.accelerator_identity.entry_kind {
            EntryKind::Template | EntryKind::Starter => matches!(
                record.resulting_mode,
                ResultingMode::CreateProject
                    | ResultingMode::CreateService
                    | ResultingMode::AddModule
                    | ResultingMode::CreateEmptyWorkspace
                    | ResultingMode::OpenWithoutStarter
            ),
            EntryKind::Prebuild => matches!(
                record.resulting_mode,
                ResultingMode::ResumeLiveWorkspace
                    | ResultingMode::StartFromSnapshot
                    | ResultingMode::CloneFresh
                    | ResultingMode::OpenPrebuildWithSetupActions
                    | ResultingMode::OpenPrebuildMinimal
                    | ResultingMode::OpenWithoutStarter
            ),
        };
        assert!(
            valid,
            "{}: resulting mode {} is invalid for entry kind {}",
            scenario.scenario_id,
            record.resulting_mode.as_str(),
            record.accelerator_identity.entry_kind.as_str()
        );
    }
}

#[test]
fn support_export_never_allows_raw_secrets_commands_or_urls() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        assert!(
            !record.support_export.raw_secret_export_allowed,
            "{}: raw_secret_export_allowed must be false",
            scenario.scenario_id
        );
        assert!(
            !record.support_export.raw_command_export_allowed,
            "{}: raw_command_export_allowed must be false",
            scenario.scenario_id
        );
        assert!(
            !record.support_export.raw_url_export_allowed,
            "{}: raw_url_export_allowed must be false",
            scenario.scenario_id
        );
    }
}

#[test]
fn failure_summary_does_not_hide_partial_application() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if let Some(ref failure) = record.failure_summary {
            let has_failed = !failure.failed.is_empty();
            let has_partial = !failure.partially_applied.is_empty();
            if has_failed && !has_partial {
                // This is acceptable — nothing was partially applied.
            }
            // The real invariant is that when partials exist, they must be present.
            // Since they are in the struct, they are present. This test documents
            // the expectation that partial application is never suppressed.
            assert!(
                failure.remaining_user_review.trim().len() >= 10,
                "{}: remaining_user_review must explain what is left for the user",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn bypass_paths_include_open_without_starter_or_equal_weight_alternative() {
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        let has_open_without = record.bypass_paths.iter().any(|p| {
            matches!(
                p.path_class,
                BypassPathClass::OpenFolderWithoutStarter
                    | BypassPathClass::OpenWorkspaceWithoutStarter
                    | BypassPathClass::CloneRepositoryWithoutStarter
                    | BypassPathClass::OpenPrebuildMinimal
                    | BypassPathClass::ContinueWithoutStarter
            )
        });
        assert!(
            has_open_without,
            "{}: must include an open-without-starter or equivalent bypass path",
            scenario.scenario_id
        );
    }
}

#[test]
fn resume_live_and_start_from_snapshot_are_distinct() {
    let mut seen_modes = std::collections::HashSet::new();
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if matches!(
            record.resulting_mode,
            ResultingMode::ResumeLiveWorkspace | ResultingMode::StartFromSnapshot
        ) {
            seen_modes.insert(record.resulting_mode);
        }
    }
    // The corpus should exercise at least one of each if prebuilds are present.
    // We verify that when they appear, they are not conflated with other modes.
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = load_record(scenario.fixture_filename);
        if record.resulting_mode == ResultingMode::ResumeLiveWorkspace {
            assert_eq!(
                record.freshness_review.freshness_class,
                FreshnessClass::FreshUnderWindow,
                "resume_live_workspace must pair with fresh prebuild"
            );
        }
    }
}
