//! Start Center projection for admission checkpoints and first-useful-work routes.
//!
//! The workspace crate owns the checkpoint and route contract. This module is the
//! first shell consumer: it maps the record into reviewable rows that can be used
//! by Start Center, CLI/headless previews, support exports, and later native UI
//! rendering without inventing another readiness or archetype vocabulary.

use aureline_workspace::{
    AdmissionCheckpointRouteRecord, ContinueWithoutClass, FirstUsefulWorkRoute,
    MixedWorkspaceBoundaryChoice, ReadinessBucket, ReadinessTask,
};

/// Shell row for one readiness group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstUsefulReadinessGroup {
    /// Bucket rendered by the row.
    pub bucket: ReadinessBucket,
    /// Number of tasks in the bucket.
    pub task_count: usize,
    /// Redacted task summaries.
    pub task_summaries: Vec<String>,
}

/// Shell projection for one admission checkpoint route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterFirstUsefulWorkSurface {
    /// Route record id.
    pub route_record_id: String,
    /// Heading for the review surface.
    pub title: String,
    /// Detection family label.
    pub archetype_family_label: String,
    /// Exact confidence token.
    pub confidence_token: String,
    /// Source labels that explain the detection result.
    pub detection_sources: Vec<String>,
    /// Landing surface token.
    pub landing_surface: String,
    /// Route reason token.
    pub route_reason: String,
    /// Readiness groups, in blocking / recommended / optional order.
    pub readiness_groups: Vec<FirstUsefulReadinessGroup>,
    /// Same-weight bypass action tokens.
    pub same_weight_bypass_actions: Vec<String>,
    /// Explicit mixed-root or mixed-stack choices.
    pub boundary_choices: Vec<String>,
    /// Route switch options.
    pub switch_options: Vec<String>,
    /// Summary shown in compact review contexts.
    pub summary: String,
}

impl StartCenterFirstUsefulWorkSurface {
    /// Returns deterministic compact rows for text review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            self.title.clone(),
            format!(
                "Archetype: {} ({}) via {}",
                self.archetype_family_label,
                self.confidence_token,
                self.detection_sources.join(" + ")
            ),
            format!(
                "Landing: {} because {}",
                self.landing_surface, self.route_reason
            ),
        ];
        for group in &self.readiness_groups {
            lines.push(format!("{}: {}", group.bucket.as_str(), group.task_count));
            for summary in &group.task_summaries {
                lines.push(format!("  - {summary}"));
            }
        }
        if !self.boundary_choices.is_empty() {
            lines.push(format!(
                "Boundary choices: {}",
                self.boundary_choices.join(", ")
            ));
        }
        lines.push(format!(
            "Same-weight actions: {}",
            self.same_weight_bypass_actions.join(", ")
        ));
        lines.push(format!(
            "Switch options: {}",
            self.switch_options.join(", ")
        ));
        lines
    }
}

/// Builds a Start Center surface projection for an admission checkpoint route.
pub fn start_center_first_useful_work_surface(
    record: &AdmissionCheckpointRouteRecord,
) -> StartCenterFirstUsefulWorkSurface {
    let route = &record.first_useful_route;
    StartCenterFirstUsefulWorkSurface {
        route_record_id: record.route_record_id.clone(),
        title: format!("First useful work after {}", route.entry_source.as_str()),
        archetype_family_label: record.archetype.outcome.family_label().to_string(),
        confidence_token: record.archetype.confidence_class.as_str().to_string(),
        detection_sources: record
            .archetype
            .signals
            .iter()
            .map(|signal| signal.source_class.as_str().to_string())
            .collect(),
        landing_surface: route.landing_surface.as_str().to_string(),
        route_reason: route.route_reason_class.as_str().to_string(),
        readiness_groups: readiness_groups(record),
        same_weight_bypass_actions: bypass_tokens(&record.same_weight_bypass_actions),
        boundary_choices: boundary_tokens(&record.boundary_choices),
        switch_options: switch_tokens(route),
        summary: route.summary.clone(),
    }
}

/// Renders the Start Center first-useful-work projection as deterministic plaintext.
pub fn render_first_useful_work_plaintext(record: &AdmissionCheckpointRouteRecord) -> String {
    let surface = start_center_first_useful_work_surface(record);
    let mut lines = surface.compact_lines();
    lines.push(String::new());
    lines.join("\n")
}

fn readiness_groups(record: &AdmissionCheckpointRouteRecord) -> Vec<FirstUsefulReadinessGroup> {
    vec![
        group(ReadinessBucket::BlockingNow, &record.readiness.blocking_now),
        group(
            ReadinessBucket::RecommendedSoon,
            &record.readiness.recommended_soon,
        ),
        group(
            ReadinessBucket::OptionalLater,
            &record.readiness.optional_later,
        ),
    ]
}

fn group(bucket: ReadinessBucket, tasks: &[ReadinessTask]) -> FirstUsefulReadinessGroup {
    FirstUsefulReadinessGroup {
        bucket,
        task_count: tasks.len(),
        task_summaries: tasks.iter().map(|task| task.summary.clone()).collect(),
    }
}

fn bypass_tokens(actions: &[ContinueWithoutClass]) -> Vec<String> {
    actions
        .iter()
        .map(|action| action.as_str().to_string())
        .collect()
}

fn boundary_tokens(choices: &[MixedWorkspaceBoundaryChoice]) -> Vec<String> {
    choices
        .iter()
        .map(|choice| choice.as_str().to_string())
        .collect()
}

fn switch_tokens(route: &FirstUsefulWorkRoute) -> Vec<String> {
    route
        .switch_options
        .iter()
        .map(|option| option.as_str())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{
        build_admission_checkpoint_route, AdmissionCheckpointBuildRequest, AdmissionClass,
        AdmissionReviewRequest, AdmissionSourceSurface, ArchetypeTruth, BlockedReasonClass,
        DetectionConfidenceClass, DetectionOutcome, DetectionSignal, DetectionSignalSourceClass,
        DetectorState, EntryVerb, ExecutionBoundary, FirstUsefulEntrySource,
        MixedWorkspaceBoundaryChoice, ReadinessBucket, ReadinessBuckets, ReadinessTask,
        ReadinessTaskClass, ReadinessTaskState, ResultingMode, SideEffectClass,
        SignalMaterialEffect, SupportClaimClass, TargetKind, TrustReviewClass,
    };

    #[test]
    fn renders_certified_checkpoint_with_sources_and_buckets() {
        let record = certified_record();
        let text = render_first_useful_work_plaintext(&record);
        assert!(text.contains("First useful work after folder_or_repo_open"));
        assert!(
            text.contains("Archetype: Certified (certified_exact) via manifest + bundle_marker")
        );
        assert!(text.contains("blocking_now: 0"));
        assert!(text.contains("recommended_soon: 1"));
        assert!(text.contains("optional_later: 1"));
        assert!(text
            .contains("Same-weight actions: set_up_later, open_minimal, dismiss_recommendation"));
    }

    #[test]
    fn renders_mixed_workspace_boundary_choices() {
        let record = mixed_record();
        let surface = start_center_first_useful_work_surface(&record);
        assert_eq!(surface.landing_surface, "nested_root_choice_sheet");
        assert_eq!(
            surface.boundary_choices,
            vec![
                "open_whole_repo",
                "open_probable_project",
                "open_current_folder_only",
                "create_workset_or_slice"
            ]
        );
        assert!(surface
            .switch_options
            .contains(&"choose_root_or_workset".to_string()));
    }

    fn certified_record() -> AdmissionCheckpointRouteRecord {
        let admission = aureline_workspace::review_entry_admission(AdmissionReviewRequest::new(
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
                    "signal.web.manifest",
                    DetectionSignalSourceClass::Manifest,
                    vec![SignalMaterialEffect::RouteSelection],
                    "Manifest signal.",
                ),
                DetectionSignal::new(
                    "signal.web.bundle",
                    DetectionSignalSourceClass::BundleMarker,
                    vec![SignalMaterialEffect::Recommendation],
                    "Bundle signal.",
                ),
            ],
        )
        .with_recommendation_refs(vec!["rec.web.compare_bundle".to_string()]);
        let readiness = ReadinessBuckets::new()
            .with_task(ReadinessTask::new(
                "task.web.restore",
                ReadinessTaskClass::DependencyRestore,
                ReadinessBucket::RecommendedSoon,
                ReadinessTaskState::Pending,
                ExecutionBoundary::LocalMachine,
                vec![SideEffectClass::DownloadsDependencies],
                "Dependency restore is recommended.",
            ))
            .with_task(
                ReadinessTask::new(
                    "task.web.extension",
                    ReadinessTaskClass::ExtensionRecommendation,
                    ReadinessBucket::OptionalLater,
                    ReadinessTaskState::Optional,
                    ExecutionBoundary::NoExecution,
                    vec![SideEffectClass::NoSideEffect],
                    "Extension recommendation is optional.",
                )
                .with_optional_reason(
                    aureline_workspace::OptionalReasonClass::OptionalRecommendedOnly,
                ),
            );
        build_admission_checkpoint_route(
            AdmissionCheckpointBuildRequest::new(
                admission,
                "entry.action.open.web",
                FirstUsefulEntrySource::FolderOrRepoOpen,
                archetype,
            )
            .with_readiness(readiness),
        )
    }

    fn mixed_record() -> AdmissionCheckpointRouteRecord {
        let admission = aureline_workspace::review_entry_admission(AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Open,
            TargetKind::LocalFolder,
            ResultingMode::WorkspaceCandidate,
            "~/Code/mixed",
        ));
        let archetype = ArchetypeTruth::new(
            DetectionOutcome::MixedOrAmbiguousWorkspace,
            DetectionConfidenceClass::MixedConflicting,
            SupportClaimClass::GenericNoClaim,
            DetectorState::Partial,
            vec![DetectionSignal::new(
                "signal.mixed.layout",
                DetectionSignalSourceClass::FilesystemLayout,
                vec![SignalMaterialEffect::RouteSelection],
                "Nested roots compete.",
            )],
        )
        .with_recommendation_refs(vec!["rec.mixed.choose_boundary".to_string()]);
        let readiness = ReadinessBuckets::new().with_task(
            ReadinessTask::new(
                "task.mixed.boundary",
                ReadinessTaskClass::UserBoundaryChoice,
                ReadinessBucket::BlockingNow,
                ReadinessTaskState::BlockedByTrust,
                ExecutionBoundary::NoExecution,
                vec![SideEffectClass::NoSideEffect],
                "Choose a workspace boundary.",
            )
            .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
        );
        build_admission_checkpoint_route(
            AdmissionCheckpointBuildRequest::new(
                admission,
                "entry.action.open.mixed",
                FirstUsefulEntrySource::FolderOrRepoOpen,
                archetype,
            )
            .with_readiness(readiness)
            .with_boundary_choices(vec![
                MixedWorkspaceBoundaryChoice::OpenWholeRepo,
                MixedWorkspaceBoundaryChoice::OpenProbableProject,
                MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
                MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
            ])
            .with_admission_class(AdmissionClass::TrustReviewRequired)
            .with_trust(
                aureline_workspace::TrustState::PendingEvaluation,
                TrustReviewClass::TrustReviewPending,
            ),
        )
    }
}
