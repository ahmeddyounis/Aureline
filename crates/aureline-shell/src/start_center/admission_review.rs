//! Start Center and drag/drop projections for reviewed admission packets.
//!
//! This module is the shell consumer of the workspace admission contract. It
//! maps Start Center actions, clone/import sheet form values, and drag/drop
//! payloads onto the same [`aureline_workspace::AdmissionReviewPacket`] shape.

use std::path::Path;

use aureline_workspace::{
    build_admission_checkpoint_route, detect_workspace_archetype, review_drag_drop_admission,
    review_entry_admission, AdmissionCheckpointBuildRequest, AdmissionCheckpointRouteRecord,
    AdmissionReviewPacket, AdmissionReviewRequest, AdmissionSourceSurface, ArchetypeDetectionError,
    ArchetypeDetectionOutcome, ArchetypeDetectionReport, ArchetypeProposal, BlockedReasonClass,
    DragDropAdmissionRequest, DragDropPayloadKind, EntryVerb, ExecutionBoundary,
    FirstUsefulEntrySource, MixedWorkspaceBoundaryChoice, ReadinessBucket, ReadinessBuckets,
    ReadinessTask, ReadinessTaskClass, ReadinessTaskState, ResultingMode, SideEffectClass,
    TargetKind,
};

use super::StartCenterPrimaryActionId;

/// User choices shown alongside a Start Center archetype suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartCenterArchetypeOverrideAction {
    /// Keep the plain open route without adopting the suggestion.
    KeepPlainOpen,
    /// Choose another archetype explicitly.
    ChooseAnotherArchetype,
    /// Dismiss the suggestion for this admission.
    DismissRecommendation,
}

impl StartCenterArchetypeOverrideAction {
    /// Returns the stable snake_case token for this override action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepPlainOpen => "keep_plain_open",
            Self::ChooseAnotherArchetype => "choose_another_archetype",
            Self::DismissRecommendation => "dismiss_recommendation",
        }
    }
}

/// Start Center suggestion rendered during first workspace admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterArchetypeSuggestion {
    /// Seed-row id backing the suggestion.
    pub archetype_seed_row_id: String,
    /// Archetype row reference backing the suggestion.
    pub archetype_row_ref: String,
    /// Human-readable archetype label.
    pub public_label: String,
    /// Launch bundle reference compatible with the suggestion.
    pub bundle_ref: String,
    /// Bundle manifest reference compatible with the suggestion.
    pub bundle_manifest_ref: String,
    /// Confidence score in the inclusive range `0..=100`.
    pub confidence_score: u8,
    /// Compact confidence label for Start Center.
    pub confidence_label: &'static str,
    /// Marker files that explain the suggestion.
    pub source_markers: Vec<String>,
    /// Whether Start Center may apply the suggestion without confirmation.
    pub auto_apply: bool,
    /// Whether the user can override the suggestion.
    pub user_override_available: bool,
    /// Override actions rendered with the suggestion.
    pub override_actions: Vec<StartCenterArchetypeOverrideAction>,
}

/// First-admission review shown after opening a workspace from Start Center.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterFirstAdmissionReview {
    /// Existing entry admission packet.
    pub admission_packet: AdmissionReviewPacket,
    /// Read-only archetype detection report.
    pub detection_report: ArchetypeDetectionReport,
    /// Optional user-confirmed archetype suggestion.
    pub archetype_suggestion: Option<StartCenterArchetypeSuggestion>,
    /// Admission checkpoint route fed by the detection report.
    pub checkpoint_route: AdmissionCheckpointRouteRecord,
    /// Whether detection may auto-apply setup or bundle state.
    pub auto_apply_allowed: bool,
    /// Same-weight override actions available even when a suggestion exists.
    pub override_actions: Vec<StartCenterArchetypeOverrideAction>,
}

/// Builds an admission packet for a Start Center primary action.
pub fn admission_packet_for_start_center_action(
    action_id: StartCenterPrimaryActionId,
) -> AdmissionReviewPacket {
    let (entry_verb, target_kind, resulting_mode, target_specifier, destination) =
        start_center_admission_tuple(action_id);
    let mut request = AdmissionReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        entry_verb,
        target_kind,
        resulting_mode,
        target_specifier,
    );
    if let Some(destination) = destination {
        request = request.with_destination(destination);
    }
    review_entry_admission(request)
}

/// Builds an admission packet for a resolved entry-flow sheet.
pub fn admission_packet_for_resolved_entry(
    source_surface: AdmissionSourceSurface,
    entry_verb: EntryVerb,
    target_kind: TargetKind,
    resulting_mode: ResultingMode,
    target_specifier: impl Into<String>,
    destination: Option<String>,
) -> AdmissionReviewPacket {
    let mut request = AdmissionReviewRequest::new(
        source_surface,
        entry_verb,
        target_kind,
        resulting_mode,
        target_specifier,
    );
    if let Some(destination) = destination {
        request = request.with_destination(destination);
    }
    review_entry_admission(request)
}

/// Builds the first-admission review for a local workspace path selected from Start Center.
///
/// # Errors
///
/// Returns [`ArchetypeDetectionError`] when the selected path cannot be read as
/// a workspace root or the seed-row artifact cannot be parsed.
pub fn first_admission_review_for_workspace_path(
    path: impl AsRef<Path>,
) -> Result<StartCenterFirstAdmissionReview, ArchetypeDetectionError> {
    let path = path.as_ref();
    let target_kind = target_kind_for_workspace_path(path);
    let resulting_mode = resulting_mode_for_workspace_path(target_kind);
    let admission_packet = admission_packet_for_resolved_entry(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Open,
        target_kind,
        resulting_mode,
        path.display().to_string(),
        None,
    );
    let detection_report = detect_workspace_archetype(path)?;
    let archetype_suggestion = suggestion_from_detection(&detection_report);
    let readiness = readiness_for_detection(&detection_report);
    let mut checkpoint_request = AdmissionCheckpointBuildRequest::new(
        admission_packet.clone(),
        "entry.action.start_center.first_admission",
        FirstUsefulEntrySource::FolderOrRepoOpen,
        detection_report.to_archetype_truth(),
    )
    .with_readiness(readiness);
    if detection_report.outcome == ArchetypeDetectionOutcome::ConflictingMarkers {
        checkpoint_request = checkpoint_request.with_boundary_choices(vec![
            MixedWorkspaceBoundaryChoice::OpenWholeRepo,
            MixedWorkspaceBoundaryChoice::OpenProbableProject,
            MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
            MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
        ]);
    }
    let checkpoint_route = build_admission_checkpoint_route(checkpoint_request);

    Ok(StartCenterFirstAdmissionReview {
        admission_packet,
        detection_report,
        archetype_suggestion,
        checkpoint_route,
        auto_apply_allowed: false,
        override_actions: default_archetype_override_actions(),
    })
}

/// Builds a clone admission packet from clone sheet form values.
pub fn clone_form_admission_packet(
    remote_url: impl Into<String>,
    destination_path: impl Into<String>,
) -> AdmissionReviewPacket {
    review_entry_admission(
        AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            remote_url,
        )
        .with_destination(destination_path),
    )
}

/// Builds an import admission packet from import sheet form values.
pub fn import_form_admission_packet(
    source_path: impl Into<String>,
    destination_workspace_target: impl Into<String>,
) -> AdmissionReviewPacket {
    let source_path = source_path.into();
    let destination_workspace_target = destination_workspace_target.into();
    review_entry_admission(
        AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Import,
            classify_import_target(&source_path),
            ResultingMode::ExtractThenReview,
            source_path,
        )
        .with_destination(destination_workspace_target),
    )
}

/// Builds an admission packet for a shell drag/drop payload.
pub fn drag_drop_admission_packet_for_path(
    path: impl AsRef<Path>,
    aggregate_bytes: Option<u64>,
    active_workspace_label: Option<String>,
) -> AdmissionReviewPacket {
    let path = path.as_ref();
    let payload_kind = drag_drop_payload_kind_for_path(path);
    let mut request = DragDropAdmissionRequest::new(payload_kind, path.display().to_string());
    if let Some(bytes) = aggregate_bytes {
        request = request.with_aggregate_bytes(bytes);
    }
    if let Some(active_workspace_label) = active_workspace_label {
        request = request.with_active_workspace(active_workspace_label);
    }
    review_drag_drop_admission(request)
}

/// Renders a packet into compact text rows for the entry sheet.
pub fn compact_admission_review_lines(packet: &AdmissionReviewPacket) -> Vec<String> {
    packet.compact_lines()
}

fn start_center_admission_tuple(
    action_id: StartCenterPrimaryActionId,
) -> (
    EntryVerb,
    TargetKind,
    ResultingMode,
    &'static str,
    Option<&'static str>,
) {
    match action_id {
        StartCenterPrimaryActionId::OpenFolder => (
            EntryVerb::Open,
            TargetKind::LocalFolder,
            ResultingMode::Folder,
            "folder selected from Start Center",
            None,
        ),
        StartCenterPrimaryActionId::OpenWorkspace => (
            EntryVerb::Open,
            TargetKind::WorkspaceManifest,
            ResultingMode::WorkspaceWithRoots,
            "workspace file selected from Start Center",
            None,
        ),
        StartCenterPrimaryActionId::CloneRepository => (
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            "https://example.invalid/repository.git",
            Some("~/Code/repository"),
        ),
        StartCenterPrimaryActionId::RestoreLastSession => (
            EntryVerb::Restore,
            TargetKind::RecoveryCheckpoint,
            ResultingMode::RestoreLastSession,
            "last session checkpoint",
            None,
        ),
        StartCenterPrimaryActionId::ImportFrom => (
            EntryVerb::Import,
            TargetKind::CompetitorConfigRoot,
            ResultingMode::ExtractThenReview,
            "import source selected from Start Center",
            Some("labelled import staging"),
        ),
    }
}

fn classify_import_target(source_path: &str) -> TargetKind {
    let lower = source_path.to_ascii_lowercase();
    if lower.ends_with(".zip")
        || lower.ends_with(".tar")
        || lower.ends_with(".tar.gz")
        || lower.ends_with(".tgz")
    {
        TargetKind::PortableStatePackage
    } else if lower.ends_with(".patch") || lower.ends_with(".diff") {
        TargetKind::HandoffPacket
    } else {
        TargetKind::CompetitorConfigRoot
    }
}

fn target_kind_for_workspace_path(path: &Path) -> TargetKind {
    if path.join(".git").is_dir() {
        TargetKind::LocalRepoRoot
    } else {
        TargetKind::LocalFolder
    }
}

fn resulting_mode_for_workspace_path(target_kind: TargetKind) -> ResultingMode {
    match target_kind {
        TargetKind::LocalRepoRoot => ResultingMode::RepoRoot,
        _ => ResultingMode::WorkspaceCandidate,
    }
}

fn suggestion_from_detection(
    report: &ArchetypeDetectionReport,
) -> Option<StartCenterArchetypeSuggestion> {
    let proposal = report.proposal.as_ref()?;
    Some(StartCenterArchetypeSuggestion {
        archetype_seed_row_id: proposal.archetype_seed_row_id.clone(),
        archetype_row_ref: proposal.archetype_row_ref.clone(),
        public_label: proposal.public_label.clone(),
        bundle_ref: proposal.bundle_ref.clone(),
        bundle_manifest_ref: proposal.bundle_manifest_ref.clone(),
        confidence_score: proposal.confidence_score,
        confidence_label: confidence_label(proposal),
        source_markers: report
            .signals
            .iter()
            .filter(|signal| signal.family == proposal.family)
            .map(|signal| signal.marker.clone())
            .collect(),
        auto_apply: false,
        user_override_available: true,
        override_actions: default_archetype_override_actions(),
    })
}

fn confidence_label(proposal: &ArchetypeProposal) -> &'static str {
    if proposal.confidence_score >= 85 {
        "high"
    } else {
        "medium"
    }
}

fn readiness_for_detection(report: &ArchetypeDetectionReport) -> ReadinessBuckets {
    if report.outcome != ArchetypeDetectionOutcome::ConflictingMarkers {
        return ReadinessBuckets::new();
    }
    ReadinessBuckets::new().with_task(
        ReadinessTask::new(
            "task.archetype.choose_workspace_boundary",
            ReadinessTaskClass::UserBoundaryChoice,
            ReadinessBucket::BlockingNow,
            ReadinessTaskState::BlockedByTrust,
            ExecutionBoundary::NoExecution,
            vec![SideEffectClass::NoSideEffect],
            "Choose which root or stack should receive the first workspace view.",
        )
        .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
    )
}

fn default_archetype_override_actions() -> Vec<StartCenterArchetypeOverrideAction> {
    vec![
        StartCenterArchetypeOverrideAction::KeepPlainOpen,
        StartCenterArchetypeOverrideAction::ChooseAnotherArchetype,
        StartCenterArchetypeOverrideAction::DismissRecommendation,
    ]
}

fn drag_drop_payload_kind_for_path(path: &Path) -> DragDropPayloadKind {
    if path.is_dir() {
        if path.join(".git").exists() {
            return DragDropPayloadKind::Repository;
        }
        return DragDropPayloadKind::Folder;
    }
    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
        return DragDropPayloadKind::File;
    };
    match extension.to_ascii_lowercase().as_str() {
        "code-workspace" | "aureline-workspace" => DragDropPayloadKind::WorkspaceFile,
        "patch" | "diff" => DragDropPayloadKind::Patch,
        "zip" | "tar" | "tgz" | "gz" => DragDropPayloadKind::Archive,
        _ => DragDropPayloadKind::File,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{AdmissionAction, DestinationDisposition, TransferProgressClass};

    #[test]
    fn start_center_clone_packet_discloses_destination_write_scope() {
        let packet =
            admission_packet_for_start_center_action(StartCenterPrimaryActionId::CloneRepository);
        assert_eq!(packet.entry_verb, EntryVerb::Clone);
        assert_eq!(
            packet.destination_review.disposition,
            DestinationDisposition::WriteToUserDestination
        );
        assert!(packet.clone_review.is_some());
        assert!(packet.trust_and_setup_review.no_silent_trust_grant);
    }

    #[test]
    fn drag_drop_archive_packet_uses_import_verb_and_durable_progress() {
        let packet = drag_drop_admission_packet_for_path(
            "/tmp/archive.zip",
            Some(32 * 1024 * 1024),
            Some("workspace:active".to_string()),
        );
        assert_eq!(packet.entry_verb, EntryVerb::Import);
        let drop = packet.drag_drop_review.as_ref().expect("drop review");
        assert_eq!(drop.advertised_verb, AdmissionAction::Import);
        assert_eq!(
            drop.progress_class,
            TransferProgressClass::DurableProgressWithCancel
        );
        assert!(drop.uses_same_admission_model);
    }

    #[test]
    fn clone_form_packet_preserves_collision_choices() {
        let temp = std::env::temp_dir().join("aureline-shell-admission-collision");
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(&temp).expect("temp dir");
        std::fs::write(temp.join("README.md"), "occupied\n").expect("seed file");

        let packet = clone_form_admission_packet(
            "https://user:secret@example.com/acme/app.git",
            temp.display().to_string(),
        );
        let collision = packet.collision_review.as_ref().expect("collision");
        assert!(collision
            .safe_actions
            .contains(&AdmissionAction::CloneElsewhere));
        assert!(packet
            .clone_review
            .as_ref()
            .expect("clone")
            .normalized_remote_label
            .contains("example.com/acme/app"));
    }

    #[test]
    fn first_admission_suggests_archetype_without_auto_apply() {
        let temp = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir_all(temp.path().join("src")).expect("src dir");
        std::fs::write(
            temp.path().join("package.json"),
            r#"{
              "scripts": { "dev": "vite", "test": "vitest", "build": "vite build" },
              "dependencies": { "react": "latest", "vite": "latest" },
              "devDependencies": { "typescript": "latest", "vitest": "latest" }
            }"#,
        )
        .expect("package manifest");
        std::fs::write(temp.path().join("tsconfig.json"), "{}\n").expect("tsconfig");
        std::fs::write(temp.path().join("vite.config.ts"), "export default {}\n")
            .expect("vite config");
        std::fs::write(
            temp.path().join("src/App.tsx"),
            "export function App() { return null }\n",
        )
        .expect("tsx source");

        let review =
            first_admission_review_for_workspace_path(temp.path()).expect("first admission review");
        let suggestion = review
            .archetype_suggestion
            .as_ref()
            .expect("archetype suggestion");

        assert_eq!(suggestion.confidence_label, "high");
        assert!(suggestion.public_label.contains("TypeScript"));
        assert!(!suggestion.auto_apply);
        assert!(!review.auto_apply_allowed);
        assert!(suggestion.user_override_available);
        assert!(suggestion
            .override_actions
            .contains(&StartCenterArchetypeOverrideAction::KeepPlainOpen));
        assert!(suggestion
            .override_actions
            .contains(&StartCenterArchetypeOverrideAction::ChooseAnotherArchetype));
        assert!(
            review
                .admission_packet
                .trust_and_setup_review
                .no_setup_execution
        );
        assert!(
            review.checkpoint_route.is_contract_valid(),
            "{:?}",
            review.checkpoint_route.contract_findings()
        );
    }

    #[test]
    fn first_admission_conflicting_markers_requires_user_choice() {
        let temp = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir_all(temp.path().join("src")).expect("src dir");
        std::fs::write(
            temp.path().join("package.json"),
            r#"{
              "scripts": { "dev": "vite", "test": "vitest" },
              "dependencies": { "react": "latest", "vite": "latest" },
              "devDependencies": { "typescript": "latest" }
            }"#,
        )
        .expect("package manifest");
        std::fs::write(
            temp.path().join("pyproject.toml"),
            r#"[project]
dependencies = ["pandas", "pytest"]
"#,
        )
        .expect("pyproject");
        std::fs::write(temp.path().join("tsconfig.json"), "{}\n").expect("tsconfig");
        std::fs::write(temp.path().join("src/main.py"), "print('ready')\n").expect("python source");

        let review =
            first_admission_review_for_workspace_path(temp.path()).expect("first admission review");

        assert!(review.archetype_suggestion.is_none());
        assert_eq!(
            review.detection_report.outcome,
            ArchetypeDetectionOutcome::ConflictingMarkers
        );
        assert_eq!(
            review
                .checkpoint_route
                .first_useful_route
                .landing_surface
                .as_str(),
            "nested_root_choice_sheet"
        );
        assert!(review
            .checkpoint_route
            .boundary_choices
            .contains(&MixedWorkspaceBoundaryChoice::OpenWholeRepo));
        assert!(
            review.checkpoint_route.is_contract_valid(),
            "{:?}",
            review.checkpoint_route.contract_findings()
        );
    }
}
