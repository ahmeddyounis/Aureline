//! Deterministic claimed-stable matrix for workspace archetype readiness preflight.
//!
//! Every scenario here is projected through the **live** admission checkpoint
//! route builder (`crate::admission::checkpoint::build_admission_checkpoint_route`)
//! so the preflight records are a genuine projection of the workspace admission
//! code rather than a parallel model. The corpus then mints one governed
//! [`WorkspaceArchetypeReadinessPreflightRecord`] per scenario and pins it on
//! disk under
//! `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/`.
//!
//! The matrix covers all six required detection outcomes:
//! certified archetype match, probable archetype, mixed or ambiguous workspace,
//! unknown or generic workspace, restricted or policy blocked, and missing
//! prerequisite.

use crate::admission::checkpoint::{
    build_admission_checkpoint_route, AdmissionCheckpointBuildRequest, AdmissionClass,
    ArchetypeTruth, BlockedReasonClass, ContinueWithoutClass, DetectionConfidenceClass,
    DetectionEvidenceFreshness, DetectionOutcome, DetectionSignal, DetectionSignalSourceClass,
    DetectorState, ExecutionBoundary, FirstUsefulEntrySource, MixedWorkspaceBoundaryChoice,
    OptionalReasonClass, ReadinessBucket, ReadinessBuckets, ReadinessTask, ReadinessTaskClass,
    ReadinessTaskState, SideEffectClass, SignalFreshnessClass, SignalMaterialEffect,
    SupportClaimClass, TrustReviewClass,
};
use crate::{
    review_entry_admission, AdmissionReviewRequest, AdmissionSourceSurface, EntryVerb,
    ResultingMode, TargetKind, TrustState,
};

use super::model::{
    PreflightInput, WorkspaceArchetypeReadinessPreflightRecord,

};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-06-02T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/workspace-archetype-readiness-preflight";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/workspace-archetype-readiness-preflight";
const EVIDENCE_REF: &str = "aureline://artifact/ux-m4-stabilize-workspace-archetype-detection";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-stabilize-workspace-archetype-detection";

/// One scenario in the claimed-stable preflight matrix.
#[derive(Debug, Clone)]
pub struct WorkspaceArchetypeReadinessPreflightScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected detection outcome.
    pub expected_detection_outcome: DetectionOutcome,
    /// Expected detection confidence.
    pub expected_detection_confidence: DetectionConfidenceClass,
    /// Expected landing surface.
    pub expected_landing_surface: crate::admission::checkpoint::LandingSurface,
    record: WorkspaceArchetypeReadinessPreflightRecord,
}

impl WorkspaceArchetypeReadinessPreflightScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> WorkspaceArchetypeReadinessPreflightRecord {
        self.record.clone()
    }
}

struct ScenarioSeed {
    scenario_id: &'static str,
    fixture_filename: &'static str,
    source_surface: AdmissionSourceSurface,
    entry_verb: EntryVerb,
    target_kind: TargetKind,
    resulting_mode: ResultingMode,
    target_specifier: &'static str,
    entry_source: FirstUsefulEntrySource,
    entry_action_ref: &'static str,
    archetype: ArchetypeTruth,
    readiness: ReadinessBuckets,
    continue_without: ContinueWithoutClass,
    trust_state: TrustState,
    trust_review_class: TrustReviewClass,
    admission_class: AdmissionClass,
    plain_open_available: bool,
    ordinary_editing_available: bool,
    boundary_choices: Vec<MixedWorkspaceBoundaryChoice>,
    rememberable: bool,
}

fn certified_ts_web_app_seed() -> ScenarioSeed {
    let _admission_review = review_entry_admission(AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        TargetKind::LocalRepoRoot,
        ResultingMode::RepoRoot,
        "~/Code/web-app",
    ));
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::CertifiedArchetypeMatch,
        DetectionConfidenceClass::CertifiedExact,
        SupportClaimClass::CertifiedCurrent,
        DetectorState::ReadyEnough,
        vec![
            DetectionSignal::new(
                "signal.ts.manifest",
                DetectionSignalSourceClass::Manifest,
                vec![
                    SignalMaterialEffect::SupportClaim,
                    SignalMaterialEffect::RouteSelection,
                ],
                "Project manifest (package.json, tsconfig.json) matches the certified TS web launch wedge.",
            )
            .with_freshness_class(SignalFreshnessClass::FreshCurrent),
            DetectionSignal::new(
                "signal.ts.bundle_marker",
                DetectionSignalSourceClass::BundleMarker,
                vec![SignalMaterialEffect::Recommendation],
                "Bundle marker points to a compatible reviewed TypeScript web bundle.",
            )
            .with_freshness_class(SignalFreshnessClass::FreshCurrent),
        ],
    )
    .with_archetype_ref("archetype.ts_web_app.certified")
    .with_compatible_bundle_refs(vec!["bundle.ts_web.current".to_string()])
    .with_evidence_freshness(vec![
        DetectionEvidenceFreshness::new(
            "evidence.ts_web.certified_scorecard",
            SignalFreshnessClass::FreshCurrent,
            "Certified TypeScript web archetype evidence is current.",
        )
        .with_review_window(Some("2026-05-20".to_string()), Some("P21D".to_string())),
    ])
    .with_detected_fact_refs(vec![
        "fact.ts_web.manifest_present".to_string(),
        "fact.ts_web.bundle_marker_present".to_string(),
    ])
    .with_recommendation_refs(vec!["rec.ts_web.compare_bundle".to_string()]);

    let readiness = ReadinessBuckets::new()
        .with_task(
            ReadinessTask::new(
                "task.ts_web.dependency_restore",
                ReadinessTaskClass::DependencyRestore,
                ReadinessBucket::RecommendedSoon,
                ReadinessTaskState::Pending,
                ExecutionBoundary::LocalMachine,
                vec![
                    SideEffectClass::ReadsWorkspace,
                    SideEffectClass::DownloadsDependencies,
                ],
                "Dependency restore is recommended, but plain editing is available now.",
            )
            .with_source_signal_refs(vec!["signal.ts.manifest".to_string()])
            .with_recommendation_refs(vec!["rec.ts_web.compare_bundle".to_string()]),
        )
        .with_task(
            ReadinessTask::new(
                "task.ts_web.extension_recommendation",
                ReadinessTaskClass::ExtensionRecommendation,
                ReadinessBucket::OptionalLater,
                ReadinessTaskState::Optional,
                ExecutionBoundary::NoExecution,
                vec![SideEffectClass::NoSideEffect],
                "Extension recommendations are optional and dismissable.",
            )
            .with_optional_reason(OptionalReasonClass::OptionalRecommendedOnly)
            .with_source_signal_refs(vec!["signal.ts.bundle_marker".to_string()]),
        );

    ScenarioSeed {
        scenario_id: "preflight:certified-ts-web-app",
        fixture_filename: "certified_ts_web_app.json",
        source_surface: AdmissionSourceSurface::StartCenter,
        entry_verb: EntryVerb::Open,
        target_kind: TargetKind::LocalRepoRoot,
        resulting_mode: ResultingMode::RepoRoot,
        target_specifier: "~/Code/web-app",
        entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
        entry_action_ref: "entry.action.open.certified_ts_web_app",
        archetype,
        readiness,
        continue_without: ContinueWithoutClass::SetUpLater,
        trust_state: TrustState::PendingEvaluation,
        trust_review_class: TrustReviewClass::TrustReviewPending,
        admission_class: AdmissionClass::Admitted,
        plain_open_available: true,
        ordinary_editing_available: true,
        boundary_choices: Vec::new(),
        rememberable: true,
    }
}

fn probable_python_service_seed() -> ScenarioSeed {
    let _admission_review = review_entry_admission(AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        TargetKind::LocalRepoRoot,
        ResultingMode::RepoRoot,
        "~/Code/python-api",
    ));
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::ProbableArchetype,
        DetectionConfidenceClass::HighProbable,
        SupportClaimClass::SupportedScoped,
        DetectorState::RetestNeeded,
        vec![
            DetectionSignal::new(
                "signal.py.manifest",
                DetectionSignalSourceClass::Manifest,
                vec![
                    SignalMaterialEffect::SupportClaim,
                    SignalMaterialEffect::Recommendation,
                ],
                "Project manifest (pyproject.toml) matches the Python service archetype.",
            )
            .with_freshness_class(SignalFreshnessClass::CachedCurrentEnough),
        ],
    )
    .with_archetype_ref("archetype.python_service.probable")
    .with_compatible_bundle_refs(vec!["bundle.python_api.current".to_string()])
    .with_evidence_freshness(vec![
        DetectionEvidenceFreshness::new(
            "evidence.python_service.scorecard",
            SignalFreshnessClass::CachedCurrentEnough,
            "Python service archetype evidence is cached but still current enough for routing.",
        )
        .with_review_window(Some("2026-04-15".to_string()), Some("P60D".to_string())),
    ])
    .with_detected_fact_refs(vec!["fact.py.pyproject_present".to_string()])
    .with_recommendation_refs(vec!["rec.py.compare_bundle".to_string()]);

    let readiness = ReadinessBuckets::new()
        .with_task(
            ReadinessTask::new(
                "task.py.toolchain_detect",
                ReadinessTaskClass::ToolchainDetect,
                ReadinessBucket::RecommendedSoon,
                ReadinessTaskState::Pending,
                ExecutionBoundary::LocalMachine,
                vec![SideEffectClass::ReadsWorkspace],
                "Toolchain detection is recommended to confirm Python runtime availability.",
            )
            .with_source_signal_refs(vec!["signal.py.manifest".to_string()]),
        )
        .with_task(
            ReadinessTask::new(
                "task.py.index_warmup",
                ReadinessTaskClass::IndexWarmup,
                ReadinessBucket::OptionalLater,
                ReadinessTaskState::Optional,
                ExecutionBoundary::LocalMachine,
                vec![SideEffectClass::ReadsWorkspace],
                "Index warmup is optional and improves symbol search fidelity.",
            )
            .with_optional_reason(OptionalReasonClass::OptionalAdditive)
            .with_source_signal_refs(vec!["signal.py.manifest".to_string()]),
        );

    ScenarioSeed {
        scenario_id: "preflight:probable-python-service",
        fixture_filename: "probable_python_service.json",
        source_surface: AdmissionSourceSurface::StartCenter,
        entry_verb: EntryVerb::Open,
        target_kind: TargetKind::LocalRepoRoot,
        resulting_mode: ResultingMode::RepoRoot,
        target_specifier: "~/Code/python-api",
        entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
        entry_action_ref: "entry.action.open.probable_python_service",
        archetype,
        readiness,
        continue_without: ContinueWithoutClass::SetUpLater,
        trust_state: TrustState::PendingEvaluation,
        trust_review_class: TrustReviewClass::TrustReviewPending,
        admission_class: AdmissionClass::Admitted,
        plain_open_available: true,
        ordinary_editing_available: true,
        boundary_choices: Vec::new(),
        rememberable: true,
    }
}

fn mixed_ts_python_repo_seed() -> ScenarioSeed {
    let _admission_review = review_entry_admission(AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        TargetKind::LocalFolder,
        ResultingMode::WorkspaceCandidate,
        "~/Code/monorepo",
    ));
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::MixedOrAmbiguousWorkspace,
        DetectionConfidenceClass::MixedConflicting,
        SupportClaimClass::GenericNoClaim,
        DetectorState::Partial,
        vec![
            DetectionSignal::new(
                "signal.mixed.ts_manifest",
                DetectionSignalSourceClass::Manifest,
                vec![SignalMaterialEffect::Readiness],
                "TypeScript manifest (package.json) detected in frontend/ subfolder.",
            ),
            DetectionSignal::new(
                "signal.mixed.py_manifest",
                DetectionSignalSourceClass::Manifest,
                vec![SignalMaterialEffect::Readiness],
                "Python manifest (pyproject.toml) detected in backend/ subfolder.",
            ),
            DetectionSignal::new(
                "signal.mixed.layout",
                DetectionSignalSourceClass::FilesystemLayout,
                vec![SignalMaterialEffect::RouteSelection],
                "Nested root signals conflict: multiple stack manifests at different depths.",
            ),
        ],
    )
    .with_detected_fact_refs(vec![
        "fact.mixed.multi_root_signals".to_string(),
        "fact.mixed.ts_and_py_manifests".to_string(),
    ])
    .with_recommendation_refs(vec!["rec.mixed.choose_boundary".to_string()]);

    let readiness = ReadinessBuckets::new().with_task(
        ReadinessTask::new(
            "task.mixed.user_boundary_choice",
            ReadinessTaskClass::UserBoundaryChoice,
            ReadinessBucket::BlockingNow,
            ReadinessTaskState::BlockedByTrust,
            ExecutionBoundary::NoExecution,
            vec![SideEffectClass::NoSideEffect],
            "Choose whole repo, probable project, current folder only, or a workset.",
        )
        .with_blocked_reason(BlockedReasonClass::BlockedByTrust)
        .with_source_signal_refs(vec![
            "signal.mixed.ts_manifest".to_string(),
            "signal.mixed.py_manifest".to_string(),
            "signal.mixed.layout".to_string(),
        ]),
    );

    ScenarioSeed {
        scenario_id: "preflight:mixed-ts-python-repo",
        fixture_filename: "mixed_ts_python_repo.json",
        source_surface: AdmissionSourceSurface::StartCenter,
        entry_verb: EntryVerb::Open,
        target_kind: TargetKind::LocalFolder,
        resulting_mode: ResultingMode::WorkspaceCandidate,
        target_specifier: "~/Code/monorepo",
        entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
        entry_action_ref: "entry.action.open.mixed_ts_python_repo",
        archetype,
        readiness,
        continue_without: ContinueWithoutClass::OpenMinimal,
        trust_state: TrustState::PendingEvaluation,
        trust_review_class: TrustReviewClass::TrustReviewPending,
        admission_class: AdmissionClass::Admitted,
        plain_open_available: true,
        ordinary_editing_available: true,
        boundary_choices: vec![
            MixedWorkspaceBoundaryChoice::OpenWholeRepo,
            MixedWorkspaceBoundaryChoice::OpenProbableProject,
            MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
            MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
        ],
        rememberable: false,
    }
}

fn unknown_plain_folder_seed() -> ScenarioSeed {
    let _admission_review = review_entry_admission(AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        TargetKind::LocalFolder,
        ResultingMode::WorkspaceCandidate,
        "~/Code/notes",
    ));
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::UnknownOrGenericWorkspace,
        DetectionConfidenceClass::GenericUnknown,
        SupportClaimClass::GenericNoClaim,
        DetectorState::Unknown,
        vec![
            DetectionSignal::new(
                "signal.unknown.no_markers",
                DetectionSignalSourceClass::FilesystemLayout,
                vec![SignalMaterialEffect::DiagnosticOnly],
                "Bounded scan found no recognized archetype markers at depth <= 2.",
            ),
        ],
    )
    .with_detected_fact_refs(vec!["fact.unknown.no_recognized_markers".to_string()]);

    let readiness = ReadinessBuckets::new()
        .with_task(
            ReadinessTask::new(
                "task.unknown.index_warmup",
                ReadinessTaskClass::IndexWarmup,
                ReadinessBucket::OptionalLater,
                ReadinessTaskState::Optional,
                ExecutionBoundary::LocalMachine,
                vec![SideEffectClass::ReadsWorkspace],
                "Index warmup is optional and improves search fidelity in generic workspaces.",
            )
            .with_optional_reason(OptionalReasonClass::OptionalAdditive)
            .with_source_signal_refs(vec!["fact.unknown.no_recognized_markers".to_string()]),
        )
        .with_task(
            ReadinessTask::new(
                "task.unknown.diagnostics",
                ReadinessTaskClass::ToolchainDetect,
                ReadinessBucket::OptionalLater,
                ReadinessTaskState::Optional,
                ExecutionBoundary::LocalMachine,
                vec![SideEffectClass::ReadsWorkspace],
                "Toolchain detection is optional for generic folders.",
            )
            .with_optional_reason(OptionalReasonClass::OptionalAdditive)
            .with_source_signal_refs(vec!["fact.unknown.no_recognized_markers".to_string()]),
        );

    ScenarioSeed {
        scenario_id: "preflight:unknown-plain-folder",
        fixture_filename: "unknown_plain_folder.json",
        source_surface: AdmissionSourceSurface::StartCenter,
        entry_verb: EntryVerb::Open,
        target_kind: TargetKind::LocalFolder,
        resulting_mode: ResultingMode::WorkspaceCandidate,
        target_specifier: "~/Code/notes",
        entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
        entry_action_ref: "entry.action.open.unknown_plain_folder",
        archetype,
        readiness,
        continue_without: ContinueWithoutClass::OpenPlainExplorer,
        trust_state: TrustState::PendingEvaluation,
        trust_review_class: TrustReviewClass::TrustReviewPending,
        admission_class: AdmissionClass::Admitted,
        plain_open_available: true,
        ordinary_editing_available: true,
        boundary_choices: Vec::new(),
        rememberable: false,
    }
}

fn restricted_policy_block_seed() -> ScenarioSeed {
    let _admission_review = review_entry_admission(AdmissionReviewRequest::new(
        AdmissionSourceSurface::DragAndDrop,
        EntryVerb::Open,
        TargetKind::LocalRepoRoot,
        ResultingMode::RepoRoot,
        "~/Code/enterprise-app",
    ));
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::RestrictedOrPolicyBlocked,
        DetectionConfidenceClass::RestrictedByPolicy,
        SupportClaimClass::ClaimBlockedByPolicy,
        DetectorState::Blocked,
        vec![
            DetectionSignal::new(
                "signal.restricted.admin_policy",
                DetectionSignalSourceClass::AdminPolicy,
                vec![
                    SignalMaterialEffect::Policy,
                    SignalMaterialEffect::RouteSelection,
                ],
                "Admin policy blocks dependency restore and external package installation.",
            )
            .with_freshness_class(SignalFreshnessClass::PolicyEpochCurrent),
            DetectionSignal::new(
                "signal.restricted.manifest",
                DetectionSignalSourceClass::Manifest,
                vec![SignalMaterialEffect::DiagnosticOnly],
                "Project manifest is present but policy narrows allowed setup.",
            ),
        ],
    )
    .with_detected_fact_refs(vec![
        "fact.restricted.policy_active".to_string(),
        "fact.restricted.manifest_present".to_string(),
    ])
    .with_policy_block_refs(vec!["policy.dependency_install.blocked".to_string()]);

    let readiness = ReadinessBuckets::new().with_task(
        ReadinessTask::new(
            "task.restricted.dependency_restore",
            ReadinessTaskClass::DependencyRestore,
            ReadinessBucket::BlockingNow,
            ReadinessTaskState::BlockedByPolicy,
            ExecutionBoundary::LocalMachine,
            vec![
                SideEffectClass::InstallsPackages,
                SideEffectClass::ContactsRemote,
            ],
            "Dependency restore is blocked by admin policy; plain editing remains available.",
        )
        .with_blocked_reason(BlockedReasonClass::BlockedByPolicy)
        .with_source_signal_refs(vec!["signal.restricted.admin_policy".to_string()])
        .with_policy_block_refs(vec!["policy.dependency_install.blocked".to_string()]),
    );

    ScenarioSeed {
        scenario_id: "preflight:restricted-policy-block",
        fixture_filename: "restricted_policy_block.json",
        source_surface: AdmissionSourceSurface::DragAndDrop,
        entry_verb: EntryVerb::Open,
        target_kind: TargetKind::LocalRepoRoot,
        resulting_mode: ResultingMode::RepoRoot,
        target_specifier: "~/Code/enterprise-app",
        entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
        entry_action_ref: "entry.action.open.restricted_policy_block",
        archetype,
        readiness,
        continue_without: ContinueWithoutClass::ContinueInRestrictedMode,
        trust_state: TrustState::Restricted,
        trust_review_class: TrustReviewClass::TrustReviewBlockedByPolicy,
        admission_class: AdmissionClass::PolicyBlocked,
        plain_open_available: true,
        ordinary_editing_available: true,
        boundary_choices: Vec::new(),
        rememberable: false,
    }
}

fn missing_devcontainer_engine_seed() -> ScenarioSeed {
    let _admission_review = review_entry_admission(AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        TargetKind::DevcontainerWorkspace,
        ResultingMode::WorkspaceCandidate,
        "~/Code/web-client",
    ));
    let archetype = ArchetypeTruth::new(
        DetectionOutcome::MissingPrerequisite,
        DetectionConfidenceClass::PrerequisiteMissing,
        SupportClaimClass::ClaimUnavailableMissingPrerequisite,
        DetectorState::Blocked,
        vec![
            DetectionSignal::new(
                "signal.missing.workspace_file",
                DetectionSignalSourceClass::WorkspaceFile,
                vec![SignalMaterialEffect::Readiness],
                ".devcontainer/devcontainer.json is present but the container engine is not reachable.",
            ),
            DetectionSignal::new(
                "signal.missing.runtime_probe",
                DetectionSignalSourceClass::RuntimeProbe,
                vec![
                    SignalMaterialEffect::Readiness,
                    SignalMaterialEffect::RouteSelection,
                ],
                "Runtime probe did not find a container engine; devcontainer build is unavailable.",
            ),
        ],
    )
    .with_detected_fact_refs(vec![
        "fact.missing.devcontainer_json_present".to_string(),
        "fact.missing.container_engine_unavailable".to_string(),
    ]);

    let readiness = ReadinessBuckets::new().with_task(
        ReadinessTask::new(
            "task.missing.devcontainer_build",
            ReadinessTaskClass::DevcontainerBuild,
            ReadinessBucket::BlockingNow,
            ReadinessTaskState::MissingPrerequisite,
            ExecutionBoundary::Container,
            vec![
                SideEffectClass::StartsProcess,
                SideEffectClass::AttachesRuntime,
            ],
            "Devcontainer build is blocked because the container engine is missing; plain editing remains available.",
        )
        .with_blocked_reason(BlockedReasonClass::BlockedByMissingPrerequisite)
        .with_source_signal_refs(vec![
            "signal.missing.workspace_file".to_string(),
            "signal.missing.runtime_probe".to_string(),
        ]),
    );

    ScenarioSeed {
        scenario_id: "preflight:missing-devcontainer-engine",
        fixture_filename: "missing_devcontainer_engine.json",
        source_surface: AdmissionSourceSurface::StartCenter,
        entry_verb: EntryVerb::Open,
        target_kind: TargetKind::DevcontainerWorkspace,
        resulting_mode: ResultingMode::WorkspaceCandidate,
        target_specifier: "~/Code/web-client",
        entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
        entry_action_ref: "entry.action.open.missing_devcontainer_engine",
        archetype,
        readiness,
        continue_without: ContinueWithoutClass::OpenMinimal,
        trust_state: TrustState::PendingEvaluation,
        trust_review_class: TrustReviewClass::TrustReviewPending,
        admission_class: AdmissionClass::NeedsRepair,
        plain_open_available: true,
        ordinary_editing_available: false,
        boundary_choices: Vec::new(),
        rememberable: false,
    }
}

fn all_seeds() -> Vec<ScenarioSeed> {
    vec![
        certified_ts_web_app_seed(),
        probable_python_service_seed(),
        mixed_ts_python_repo_seed(),
        unknown_plain_folder_seed(),
        restricted_policy_block_seed(),
        missing_devcontainer_engine_seed(),
    ]
}

fn build_seed(seed: &ScenarioSeed) -> WorkspaceArchetypeReadinessPreflightRecord {
    let admission_review = review_entry_admission(AdmissionReviewRequest::new(
        seed.source_surface,
        seed.entry_verb,
        seed.target_kind,
        seed.resulting_mode,
        seed.target_specifier,
    ));

    let underlying = build_admission_checkpoint_route(
        AdmissionCheckpointBuildRequest::new(
            admission_review,
            seed.entry_action_ref,
            seed.entry_source,
            seed.archetype.clone(),
        )
        .with_readiness(seed.readiness.clone())
        .with_trust(seed.trust_state, seed.trust_review_class)
        .with_admission_class(seed.admission_class)
        .with_continue_without(seed.continue_without)
        .with_availability(seed.plain_open_available, seed.ordinary_editing_available)
        .with_boundary_choices(seed.boundary_choices.clone())
        .with_remembered_routing(seed.rememberable, crate::admission::checkpoint::RememberedRoutingEffect::NotRemembered),
    );

    let input = PreflightInput {
        record_id: seed.scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        underlying,
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![EVIDENCE_REF.to_string()],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    WorkspaceArchetypeReadinessPreflightRecord::build(input)
        .unwrap_or_else(|err| panic!("{}: {err}", seed.scenario_id))
}

/// Returns the full claimed-stable preflight matrix.
pub fn workspace_archetype_readiness_preflight_corpus(
) -> Vec<WorkspaceArchetypeReadinessPreflightScenario> {
    all_seeds()
        .iter()
        .map(|seed| {
            let record = build_seed(seed);
            let expected_landing_surface = record.landing_surface;
            WorkspaceArchetypeReadinessPreflightScenario {
                scenario_id: seed.scenario_id,
                fixture_filename: seed.fixture_filename,
                expected_detection_outcome: record.detection_outcome,
                expected_detection_confidence: record.detection_confidence,
                expected_landing_surface,
                record,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admission::checkpoint::RouteSwitchOption;

    #[test]
    fn corpus_covers_all_required_detection_outcomes() {
        let corpus = workspace_archetype_readiness_preflight_corpus();
        assert_eq!(corpus.len(), all_seeds().len());

        let outcomes: Vec<DetectionOutcome> =
            corpus.iter().map(|s| s.expected_detection_outcome).collect();
        for required in [
            DetectionOutcome::CertifiedArchetypeMatch,
            DetectionOutcome::ProbableArchetype,
            DetectionOutcome::MixedOrAmbiguousWorkspace,
            DetectionOutcome::UnknownOrGenericWorkspace,
            DetectionOutcome::RestrictedOrPolicyBlocked,
            DetectionOutcome::MissingPrerequisite,
        ] {
            assert!(
                outcomes.contains(&required),
                "missing detection outcome {required:?}"
            );
        }
    }

    #[test]
    fn every_record_holds_contract_and_safety_invariants() {
        for scenario in workspace_archetype_readiness_preflight_corpus() {
            let record = scenario.record();
            assert!(
                record.is_contract_valid(),
                "{}: {:?}",
                scenario.scenario_id,
                record.contract_findings()
            );
            assert!(!record.auto_install_allowed, "{}", scenario.scenario_id);
            assert!(!record.auto_trust_allowed, "{}", scenario.scenario_id);
            assert!(!record.hidden_setup_executed, "{}", scenario.scenario_id);
            assert!(!record.trust_widened, "{}", scenario.scenario_id);
        }
    }

    #[test]
    fn certified_and_probable_records_carry_evidence_freshness() {
        for scenario in workspace_archetype_readiness_preflight_corpus() {
            let record = scenario.record();
            if matches!(
                record.detection_outcome,
                DetectionOutcome::CertifiedArchetypeMatch | DetectionOutcome::ProbableArchetype
            ) {
                assert!(
                    !record.evidence_freshness.is_empty(),
                    "{} must carry evidence freshness",
                    scenario.scenario_id
                );
            }
        }
    }

    #[test]
    fn mixed_workspace_has_all_boundary_choices() {
        for scenario in workspace_archetype_readiness_preflight_corpus() {
            let record = scenario.record();
            if record.detection_outcome == DetectionOutcome::MixedOrAmbiguousWorkspace {
                assert_eq!(
                    record.boundary_choices.len(),
                    4,
                    "{} must have 4 boundary choices",
                    scenario.scenario_id
                );
            }
        }
    }

    #[test]
    fn restricted_and_missing_prerequisite_offer_open_minimal() {
        for scenario in workspace_archetype_readiness_preflight_corpus() {
            let record = scenario.record();
            if matches!(
                record.detection_outcome,
                DetectionOutcome::RestrictedOrPolicyBlocked | DetectionOutcome::MissingPrerequisite
            ) {
                assert!(
                    record.switch_options.contains(&RouteSwitchOption::OpenMinimal),
                    "{} must offer OpenMinimal",
                    scenario.scenario_id
                );
            }
        }
    }

    #[test]
    fn readiness_tasks_carry_source_signal_refs() {
        for scenario in workspace_archetype_readiness_preflight_corpus() {
            let record = scenario.record();
            for task in record.readiness_buckets.all_tasks() {
                assert!(
                    !task.source_signal_refs.is_empty(),
                    "{}: task {} must have source_signal_refs",
                    scenario.scenario_id,
                    task.task_ref
                );
            }
        }
    }

    #[test]
    fn support_export_lines_render_all_sections() {
        for scenario in workspace_archetype_readiness_preflight_corpus() {
            let record = scenario.record();
            let lines = record.support_export_lines();
            assert!(
                lines
                    .iter()
                    .any(|line| line.starts_with("workspace_archetype_readiness_preflight:")),
                "{} must render header",
                scenario.scenario_id
            );
            assert!(
                lines.iter().any(|line| line.starts_with("detection:")),
                "{} must render detection",
                scenario.scenario_id
            );
            assert!(
                lines.iter().any(|line| line.starts_with("readiness:")),
                "{} must render readiness",
                scenario.scenario_id
            );
            assert!(
                lines.iter().any(|line| line.starts_with("route:")),
                "{} must render route",
                scenario.scenario_id
            );
            assert!(
                lines.iter().any(|line| line.starts_with("safety:")),
                "{} must render safety",
                scenario.scenario_id
            );
        }
    }
}
