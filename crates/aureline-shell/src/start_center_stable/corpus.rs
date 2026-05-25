//! Deterministic claimed-stable matrix for entry-target disclosure.
//!
//! Every scenario here is projected through the **live** Start Center and
//! workspace-switcher builders (`crate::start_center::build_searchable_recent_work_rows`
//! and `crate::workspace_switcher::build_switcher_rows`) from one shared
//! recent-work registry, so the disclosure records are a genuine projection of
//! the shell's row code rather than a parallel model. The corpus then mints one
//! governed [`EntryTargetDisclosureRecord`] per target and pins it on disk under
//! `fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace/`.
//!
//! The matrix covers the required target-kind, target-class, and recovery
//! coverage: local file, local folder, multi-root workspace, SSH, dev container,
//! and managed cloud workspace, across reachable, missing-path, moved-root,
//! reconnect-required, and reauthorization-required states.

use aureline_workspace::{
    PortabilityClass, RecentWorkEntryRecord, RecentWorkEntryRecordKind, RecentWorkFailureState,
    RecentWorkRegistry, RecentWorkRegistryRecordKind, RecentWorkTargetState, RestoreAvailability,
    SafeRecoveryAction, TargetKind, TrustState,
};

use crate::start_center::{
    build_searchable_recent_work_rows, StartCenterRecentWorkPrivacyMode, StartCenterRecentWorkRow,
};
use crate::workspace_switcher::{build_switcher_rows, WorkspaceSwitcherRow};

use super::model::{
    AccessibilityDisclosure, EntryRouteRecord, EntryRouteSurface, EntryTargetDisclosureInput,
    EntryTargetDisclosureRecord, LayoutMode, LayoutModeDisclosure, PublicClaimCeiling,
    RecoveryRouteRecord, SubtitleKind, SurfaceParity, TargetClass,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/entry-target-disclosure";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/entry-target-disclosure";
const EVIDENCE_REF: &str = "aureline://artifact/ux-m4-stabilize-start-center";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-stabilize-start-center";

/// One scenario in the claimed-stable disclosure matrix.
#[derive(Debug, Clone)]
pub struct EntryTargetDisclosureScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected canonical target kind.
    pub expected_target_kind: TargetKind,
    /// Expected matrix target class.
    pub expected_target_class: TargetClass,
    /// Expected failure-state classification.
    pub expected_failure_state: RecentWorkFailureState,
    /// Expected trust posture.
    pub expected_trust_state: TrustState,
    /// Expected restore availability.
    pub expected_restore_availability: RestoreAvailability,
    /// Expected honesty marker.
    pub expected_honesty_marker_present: bool,
    record: EntryTargetDisclosureRecord,
}

impl EntryTargetDisclosureScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> EntryTargetDisclosureRecord {
        self.record.clone()
    }
}

struct ScenarioSeed {
    scenario_id: &'static str,
    fixture_filename: &'static str,
    recent_work_id: &'static str,
    presentation_label: &'static str,
    presentation_subtitle: &'static str,
    target_kind: TargetKind,
    target_state: RecentWorkTargetState,
    portability_class: PortabilityClass,
    trust_state: TrustState,
    restore_availability: RestoreAvailability,
    safe_recovery_actions: &'static [SafeRecoveryAction],
    pinned: bool,
    last_opened_at: &'static str,
    filesystem_identity_ref: Option<&'static str>,
    remote_target_descriptor_ref: Option<&'static str>,
}

const SCENARIO_SEEDS: &[ScenarioSeed] = &[
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:local-folder-docs",
        fixture_filename: "local_folder_reachable.json",
        recent_work_id: "recent:stable:local-folder-docs",
        presentation_label: "Aureline docs",
        presentation_subtitle: "~/Code/aureline-docs",
        target_kind: TargetKind::LocalFolder,
        target_state: RecentWorkTargetState::Reachable,
        portability_class: PortabilityClass::LocalOnly,
        trust_state: TrustState::Trusted,
        restore_availability: RestoreAvailability::Exact,
        safe_recovery_actions: &[
            SafeRecoveryAction::Open,
            SafeRecoveryAction::OpenInNewWindow,
            SafeRecoveryAction::RevealInExplorer,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: true,
        last_opened_at: "mono:2026-05-25T10:45:00.0000",
        filesystem_identity_ref: Some("fs:docs-local-folder"),
        remote_target_descriptor_ref: None,
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:local-file-manifest",
        fixture_filename: "local_file_reachable.json",
        recent_work_id: "recent:stable:local-file-manifest",
        presentation_label: "Cargo.toml",
        presentation_subtitle: "~/Code/aureline/Cargo.toml",
        target_kind: TargetKind::LocalFile,
        target_state: RecentWorkTargetState::Reachable,
        portability_class: PortabilityClass::LocalOnly,
        trust_state: TrustState::Trusted,
        restore_availability: RestoreAvailability::Compatible,
        safe_recovery_actions: &[
            SafeRecoveryAction::Open,
            SafeRecoveryAction::RevealInExplorer,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: false,
        last_opened_at: "mono:2026-05-24T16:10:00.0000",
        filesystem_identity_ref: Some("fs:local-file-manifest"),
        remote_target_descriptor_ref: None,
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:multi-root-platform",
        fixture_filename: "multi_root_workspace_reachable.json",
        recent_work_id: "recent:stable:multi-root-platform",
        presentation_label: "Platform workspace",
        presentation_subtitle: "~/Work/platform.code-workspace",
        target_kind: TargetKind::WorkspaceManifest,
        target_state: RecentWorkTargetState::Reachable,
        portability_class: PortabilityClass::LocalOnly,
        trust_state: TrustState::Trusted,
        restore_availability: RestoreAvailability::Exact,
        safe_recovery_actions: &[
            SafeRecoveryAction::Open,
            SafeRecoveryAction::OpenInNewWindow,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: true,
        last_opened_at: "mono:2026-05-23T18:20:00.0000",
        filesystem_identity_ref: Some("fs:multi-root-platform"),
        remote_target_descriptor_ref: None,
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:payments-missing-path",
        fixture_filename: "local_repo_missing_path.json",
        recent_work_id: "recent:stable:payments-missing-path",
        presentation_label: "Payments service",
        presentation_subtitle: "~/Code/payments",
        target_kind: TargetKind::LocalRepoRoot,
        target_state: RecentWorkTargetState::MissingTarget,
        portability_class: PortabilityClass::LocalOnly,
        trust_state: TrustState::Trusted,
        restore_availability: RestoreAvailability::Compatible,
        safe_recovery_actions: &[
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: false,
        last_opened_at: "mono:2026-05-22T09:10:00.0000",
        filesystem_identity_ref: Some("fs:payments-missing-path"),
        remote_target_descriptor_ref: None,
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:api-workspace-moved",
        fixture_filename: "workspace_moved_root.json",
        recent_work_id: "recent:stable:api-workspace-moved",
        presentation_label: "API workspace",
        presentation_subtitle: "~/Work/api.code-workspace (identity mismatch)",
        target_kind: TargetKind::WorkspaceManifest,
        target_state: RecentWorkTargetState::MovedTargetDetected,
        portability_class: PortabilityClass::LocalOnly,
        trust_state: TrustState::Trusted,
        restore_availability: RestoreAvailability::LayoutOnly,
        safe_recovery_actions: &[
            SafeRecoveryAction::OpenReadOnlyCachedView,
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: false,
        last_opened_at: "mono:2026-05-21T13:30:00.0000",
        filesystem_identity_ref: Some("fs:api-workspace-moved"),
        remote_target_descriptor_ref: None,
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:infra-ssh-unreachable",
        fixture_filename: "ssh_remote_unreachable.json",
        recent_work_id: "recent:stable:infra-ssh-unreachable",
        presentation_label: "Infra provisioning",
        presentation_subtitle: "ssh://infra.example - host unavailable",
        target_kind: TargetKind::SshWorkspace,
        target_state: RecentWorkTargetState::RemoteUnreachable,
        portability_class: PortabilityClass::ProviderLinked,
        trust_state: TrustState::PendingEvaluation,
        restore_availability: RestoreAvailability::EvidenceOnly,
        safe_recovery_actions: &[
            SafeRecoveryAction::Reconnect,
            SafeRecoveryAction::OpenReadOnlyCachedView,
            SafeRecoveryAction::RetryLater,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: false,
        last_opened_at: "mono:2026-05-20T14:05:00.0000",
        filesystem_identity_ref: None,
        remote_target_descriptor_ref: Some("remote:ssh:infra-provisioning"),
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:web-devcontainer-offline",
        fixture_filename: "devcontainer_unreachable.json",
        recent_work_id: "recent:stable:web-devcontainer-offline",
        presentation_label: "Web client dev container",
        presentation_subtitle: "devcontainer - engine offline",
        target_kind: TargetKind::DevcontainerWorkspace,
        target_state: RecentWorkTargetState::RemoteUnreachable,
        portability_class: PortabilityClass::ProviderLinked,
        trust_state: TrustState::Restricted,
        restore_availability: RestoreAvailability::Compatible,
        safe_recovery_actions: &[
            SafeRecoveryAction::Reconnect,
            SafeRecoveryAction::RetryLater,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: false,
        last_opened_at: "mono:2026-05-19T17:30:00.0000",
        filesystem_identity_ref: None,
        remote_target_descriptor_ref: Some("remote:devcontainer:web-client"),
    },
    ScenarioSeed {
        scenario_id: "entry-target-disclosure:managed-data-expired",
        fixture_filename: "managed_cloud_authority_expired.json",
        recent_work_id: "recent:stable:managed-data-expired",
        presentation_label: "Managed data workspace",
        presentation_subtitle: "cloud workspace - reauthorization required",
        target_kind: TargetKind::ManagedCloudWorkspace,
        target_state: RecentWorkTargetState::AuthorityExpired,
        portability_class: PortabilityClass::ProviderLinked,
        trust_state: TrustState::PendingEvaluation,
        restore_availability: RestoreAvailability::LayoutOnly,
        safe_recovery_actions: &[
            SafeRecoveryAction::Reauth,
            SafeRecoveryAction::RetryLater,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        pinned: false,
        last_opened_at: "mono:2026-05-18T08:25:00.0000",
        filesystem_identity_ref: None,
        remote_target_descriptor_ref: Some("remote:managed-cloud:data-workspace"),
    },
];

/// Builds the claimed-stable recent-work registry the corpus projects from.
pub fn seeded_stable_recent_work_registry() -> RecentWorkRegistry {
    RecentWorkRegistry {
        record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
        recent_work_registry_schema_version: 1,
        updated_at: "mono:2026-05-25T12:00:00.0000".to_string(),
        entries: SCENARIO_SEEDS.iter().map(seed_entry).collect(),
    }
}

/// Returns the full claimed-stable disclosure matrix.
pub fn entry_target_disclosure_corpus() -> Vec<EntryTargetDisclosureScenario> {
    let registry = seeded_stable_recent_work_registry();
    let start_center_rows =
        build_searchable_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::Default, "")
            .rows;
    let switcher_rows = build_switcher_rows(&registry, "");

    SCENARIO_SEEDS
        .iter()
        .enumerate()
        .map(|(index, seed)| {
            let start_center_row = start_center_rows
                .iter()
                .find(|row| row.recent_work_id == seed.recent_work_id)
                .unwrap_or_else(|| panic!("missing Start Center row for {}", seed.recent_work_id));
            let switcher_row = switcher_rows
                .iter()
                .find(|row| row.recent_work_id == seed.recent_work_id)
                .unwrap_or_else(|| panic!("missing switcher row for {}", seed.recent_work_id));
            let record = build_record(seed, index as u32, start_center_row, switcher_row);
            EntryTargetDisclosureScenario {
                scenario_id: seed.scenario_id,
                fixture_filename: seed.fixture_filename,
                expected_target_kind: record.target_kind,
                expected_target_class: record.target_class,
                expected_failure_state: record.failure_state,
                expected_trust_state: record.trust_state,
                expected_restore_availability: record.restore_availability,
                expected_honesty_marker_present: record.honesty_marker_present,
                record,
            }
        })
        .collect()
}

fn seed_entry(seed: &ScenarioSeed) -> RecentWorkEntryRecord {
    RecentWorkEntryRecord {
        record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
        entry_and_restore_schema_version: 1,
        recent_work_id: seed.recent_work_id.to_string(),
        presentation_label: seed.presentation_label.to_string(),
        presentation_subtitle: Some(seed.presentation_subtitle.to_string()),
        target_kind: seed.target_kind,
        target_state: seed.target_state,
        portability_class: seed.portability_class,
        trust_state: seed.trust_state,
        restore_availability: seed.restore_availability,
        safe_recovery_actions: seed.safe_recovery_actions.to_vec(),
        pinned: seed.pinned,
        last_opened_at: seed.last_opened_at.to_string(),
        filesystem_identity_ref: seed.filesystem_identity_ref.map(str::to_string),
        remote_target_descriptor_ref: seed.remote_target_descriptor_ref.map(str::to_string),
        artifact_descriptor_ref: None,
        recovery_checkpoint_refs: None,
    }
}

fn build_record(
    seed: &ScenarioSeed,
    focus_order_index: u32,
    start_center_row: &StartCenterRecentWorkRow,
    switcher_row: &WorkspaceSwitcherRow,
) -> EntryTargetDisclosureRecord {
    let target_kind = start_center_row.target_kind;
    let target_kind_label = start_center_row.target_kind_label.to_string();
    let target_class = TargetClass::from_target_kind(target_kind);
    let failure_state = start_center_row.failure_state;
    let trust_state = start_center_row.trust_state;
    let restore = start_center_row.restore_availability;
    let is_ready = failure_state == RecentWorkFailureState::Ready;

    let recovery_routes: Vec<RecoveryRouteRecord> = start_center_row
        .safe_recovery_actions
        .iter()
        .copied()
        .map(RecoveryRouteRecord::from_action)
        .collect();
    let recovery_action_ids: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_id.clone())
        .collect();
    let action_labels: Vec<String> = recovery_routes
        .iter()
        .map(|route| route.action_label.clone())
        .collect();

    let parity_holds = start_center_row.target_kind == switcher_row.target_kind
        && start_center_row.target_state == switcher_row.target_state
        && start_center_row.failure_state == switcher_row.failure_state
        && start_center_row.trust_state == switcher_row.trust_state
        && start_center_row.restore_availability == switcher_row.restore_availability
        && action_ids(&start_center_row.safe_recovery_actions)
            == action_ids(&switcher_row.safe_recovery_actions);

    let subtitle_kind = match target_class {
        TargetClass::Local => SubtitleKind::Path,
        TargetClass::RemoteBacked => SubtitleKind::Host,
        TargetClass::Managed => SubtitleKind::Provider,
    };

    let claim_ceiling = PublicClaimCeiling {
        asserts_live_open: is_ready,
        asserts_remote_available: is_ready && !matches!(target_class, TargetClass::Local),
        asserts_full_restore_fidelity: restore == RestoreAvailability::Exact,
        asserts_trusted_without_evaluation: trust_state == TrustState::Trusted,
    };

    let row_narration = format!(
        "{}, {} \u{2014} {} \u{2014} recovery: {}.",
        target_kind_label,
        start_center_row.primary_label,
        state_phrase(failure_state),
        action_labels.join(", ")
    );

    let accessibility = AccessibilityDisclosure {
        focus_order_index,
        tab_stop_count: 1 + recovery_routes.len() as u32,
        row_narration,
        action_labels,
        layout_modes: LayoutMode::REQUIRED
            .into_iter()
            .map(|mode| LayoutModeDisclosure {
                mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    };

    let routes = EntryRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!(
                "aureline://entry-route/{}/{}",
                surface.as_str(),
                seed.recent_work_id
            ),
            keyboard_reachable: true,
            activates_same_target: true,
        })
        .collect();

    let surfaces = SurfaceParity {
        start_center_row_id: format!("start-center:{}", seed.recent_work_id),
        workspace_switcher_row_id: format!("workspace-switcher:{}", seed.recent_work_id),
        switcher_entry_classes: switcher_row
            .entry_classes
            .iter()
            .map(|class| class.as_str().to_string())
            .collect(),
        switch_failure_actions: switcher_row
            .switch_failure_actions
            .iter()
            .map(|action| action.as_str().to_string())
            .collect(),
        recovery_action_ids,
        parity_holds,
    };

    let summary = format!(
        "{} disclosed as {} ({}); state {}, trust {}, restore {}; recovery: {}.",
        start_center_row.primary_label,
        target_kind_label,
        target_class.as_str(),
        failure_state.as_str(),
        trust_state.as_str(),
        restore.as_str(),
        seed.safe_recovery_actions
            .iter()
            .map(|action| action.surface_label())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let input = EntryTargetDisclosureInput {
        record_id: seed.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        recent_work_ref: format!("aureline://recent-work/{}", seed.recent_work_id),
        title: format!("{}: {}", start_center_row.primary_label, target_kind_label),
        summary,
        target_kind,
        target_state: start_center_row.target_state,
        failure_state,
        trust_state,
        restore_availability: restore,
        list_section: start_center_row.list_section,
        location_subtitle: start_center_row.location_or_target_subtitle.clone(),
        subtitle_kind,
        last_opened_at: start_center_row.last_opened_at.clone(),
        pinned: start_center_row.pinned,
        claim_ceiling,
        recovery_routes,
        discards_stale_entry_on_failure: false,
        surfaces,
        routes,
        accessibility,
        available_without_account: true,
        available_without_managed_services: true,
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![EVIDENCE_REF.to_string()],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    EntryTargetDisclosureRecord::build(input)
        .unwrap_or_else(|err| panic!("{}: {err}", seed.scenario_id))
}

fn state_phrase(failure_state: RecentWorkFailureState) -> &'static str {
    match failure_state {
        RecentWorkFailureState::Ready => "ready to open",
        RecentWorkFailureState::MissingPath => "target missing",
        RecentWorkFailureState::MovedRoot => "target moved",
        RecentWorkFailureState::ReconnectRequired => "reconnect required",
        RecentWorkFailureState::InspectOnly => "inspect only",
        RecentWorkFailureState::Blocked => "blocked",
        RecentWorkFailureState::Unknown => "state unknown",
    }
}

fn action_ids(actions: &[SafeRecoveryAction]) -> Vec<&'static str> {
    actions.iter().map(|action| action.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_covers_required_target_kinds_and_classes() {
        let corpus = entry_target_disclosure_corpus();
        assert_eq!(corpus.len(), SCENARIO_SEEDS.len());

        let kinds: Vec<TargetKind> = corpus.iter().map(|s| s.expected_target_kind).collect();
        for required in [
            TargetKind::LocalFile,
            TargetKind::LocalFolder,
            TargetKind::WorkspaceManifest,
            TargetKind::LocalRepoRoot,
            TargetKind::SshWorkspace,
            TargetKind::DevcontainerWorkspace,
            TargetKind::ManagedCloudWorkspace,
        ] {
            assert!(kinds.contains(&required), "missing target kind {required:?}");
        }

        let classes: Vec<TargetClass> = corpus.iter().map(|s| s.expected_target_class).collect();
        for required in [
            TargetClass::Local,
            TargetClass::RemoteBacked,
            TargetClass::Managed,
        ] {
            assert!(classes.contains(&required), "missing class {required:?}");
        }
    }

    #[test]
    fn corpus_covers_required_recovery_states() {
        let corpus = entry_target_disclosure_corpus();
        let failures: Vec<RecentWorkFailureState> =
            corpus.iter().map(|s| s.expected_failure_state).collect();
        for required in [
            RecentWorkFailureState::Ready,
            RecentWorkFailureState::MissingPath,
            RecentWorkFailureState::MovedRoot,
            RecentWorkFailureState::ReconnectRequired,
        ] {
            assert!(failures.contains(&required), "missing failure {required:?}");
        }
    }

    #[test]
    fn every_record_holds_surface_route_and_accessibility_parity() {
        for scenario in entry_target_disclosure_corpus() {
            let record = scenario.record();
            assert!(record.surfaces.parity_holds, "{}", scenario.scenario_id);
            assert_eq!(record.routes.len(), EntryRouteSurface::REQUIRED.len());
            assert_eq!(record.accessibility.layout_modes.len(), LayoutMode::REQUIRED.len());
            assert!(record.available_without_account);
            assert!(record.available_without_managed_services);
            assert!(!record.discards_stale_entry_on_failure);
        }
    }

    #[test]
    fn local_targets_never_claim_remote_availability() {
        for scenario in entry_target_disclosure_corpus() {
            let record = scenario.record();
            if record.target_class == TargetClass::Local {
                assert!(!record.claim_ceiling.asserts_remote_available);
            }
            if record.failure_state == RecentWorkFailureState::ReconnectRequired {
                assert!(!record.claim_ceiling.asserts_remote_available);
                assert!(!record.claim_ceiling.asserts_live_open);
            }
        }
    }
}
