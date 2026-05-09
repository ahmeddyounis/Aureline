//! Entry-flow state machine and flow-sheet resolution.
//!
//! Entry flows keep Aureline's project-entry verbs honest by resolving each
//! activation into one reviewed flow sheet before any durable write, trust
//! change, runtime attach, or restore rehydration occurs.
//!
//! The vocabulary is re-exported from the workspace entry/restore object model
//! (`docs/workspace/entry_restore_object_model.md`) and the project-entry
//! contract (`docs/ux/project_entry_contract.md`). Surfaces MUST NOT silently
//! promote one verb into another; ambiguous targets fail closed and surfaces
//! must route the user deliberately to a compatible verb instead.

use serde::{Deserialize, Serialize};

use crate::TargetKind;

/// Stable entry-verb vocabulary for project entry activations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryVerb {
    Open,
    Clone,
    Import,
    AddRoot,
    Restore,
    Resume,
    StartFromSnapshot,
}

impl EntryVerb {
    /// Returns the stable string vocabulary for this entry verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Clone => "clone",
            Self::Import => "import",
            Self::AddRoot => "add_root",
            Self::Restore => "restore",
            Self::Resume => "resume",
            Self::StartFromSnapshot => "start_from_snapshot",
        }
    }
}

/// Stable resulting-mode vocabulary for entry-flow commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultingMode {
    SingleFile,
    Folder,
    RepoRoot,
    WorkspaceCandidate,
    WorkspaceWithRoots,
    WorksetSlice,
    InspectOnly,
    CloneThenReview,
    CloneThenOpen,
    CloneThenAdd,
    CloneOnly,
    ExtractThenReview,
    CompareBeforeRestore,
    ApplyToActiveWorkspace,
    OpenPrebuildWithSetupActions,
    OpenPrebuildMinimal,
    ResumeLiveSession,
    RestoreLastSession,
    RestoreFromCheckpoint,
}

impl ResultingMode {
    /// Returns the stable string vocabulary for this resulting mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::Folder => "folder",
            Self::RepoRoot => "repo_root",
            Self::WorkspaceCandidate => "workspace_candidate",
            Self::WorkspaceWithRoots => "workspace_with_roots",
            Self::WorksetSlice => "workset_slice",
            Self::InspectOnly => "inspect_only",
            Self::CloneThenReview => "clone_then_review",
            Self::CloneThenOpen => "clone_then_open",
            Self::CloneThenAdd => "clone_then_add",
            Self::CloneOnly => "clone_only",
            Self::ExtractThenReview => "extract_then_review",
            Self::CompareBeforeRestore => "compare_before_restore",
            Self::ApplyToActiveWorkspace => "apply_to_active_workspace",
            Self::OpenPrebuildWithSetupActions => "open_prebuild_with_setup_actions",
            Self::OpenPrebuildMinimal => "open_prebuild_minimal",
            Self::ResumeLiveSession => "resume_live_session",
            Self::RestoreLastSession => "restore_last_session",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
        }
    }
}

/// Open-flow sheet class resolved for an entry activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenFlowSheetClass {
    OpenLocalTarget,
    CloneRemoteTarget,
    ImportArtifact,
    RestoreOrResume,
}

impl OpenFlowSheetClass {
    /// Returns the stable string vocabulary for this sheet class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocalTarget => "open_local_target",
            Self::CloneRemoteTarget => "clone_remote_target",
            Self::ImportArtifact => "import_artifact",
            Self::RestoreOrResume => "restore_or_resume",
        }
    }
}

/// Input target for an entry-flow resolution request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryFlowTarget {
    /// No target has been selected yet.
    Unspecified,
    /// The surface has already resolved the target kind (for example, the user
    /// picked "Clone repository", or the OS handler supplied a typed intent).
    ExplicitTargetKind(TargetKind),
    /// Unstructured user input that still needs target-kind disambiguation.
    RawSpecifier(String),
}

/// Request packet for resolving one entry flow into a reviewed sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryFlowRequest {
    /// Entry verb the user activated.
    pub entry_verb: EntryVerb,
    /// Target chosen or supplied by the surface.
    pub target: EntryFlowTarget,
    /// Optional preferred resulting mode when the surface already picked one.
    pub preferred_resulting_mode: Option<ResultingMode>,
}

/// Failure codes returned when an entry-flow request cannot be resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryFlowDenialCode {
    /// The target kind could not be resolved without additional user input.
    TargetKindUnresolved,
    /// The resolved target kind is not compatible with the activated entry verb.
    TargetKindMismatch,
    /// The requested resulting mode is not compatible with the activated entry verb.
    ResultingModeMismatch,
}

impl EntryFlowDenialCode {
    /// Returns the stable string vocabulary for this denial code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetKindUnresolved => "target_kind_unresolved",
            Self::TargetKindMismatch => "target_kind_mismatch",
            Self::ResultingModeMismatch => "resulting_mode_mismatch",
        }
    }
}

/// Successful entry-flow resolution into a reviewed flow sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryFlowResolved {
    pub sheet_class: OpenFlowSheetClass,
    pub entry_verb: EntryVerb,
    pub target_kind: TargetKind,
    pub resulting_mode: ResultingMode,
    pub candidate_resulting_modes: Vec<ResultingMode>,
}

impl EntryFlowResolved {
    /// Returns the verb-first title rendered by flow-sheet surfaces.
    pub const fn sheet_title(&self) -> &'static str {
        match self.entry_verb {
            EntryVerb::Open => "Open",
            EntryVerb::Clone => "Clone",
            EntryVerb::Import => "Import",
            EntryVerb::Restore => "Restore",
            EntryVerb::AddRoot => "Add root",
            EntryVerb::Resume => "Resume",
            EntryVerb::StartFromSnapshot => "Start from snapshot",
        }
    }
}

/// Denied entry-flow resolution that fails closed without mutating verbs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryFlowDenied {
    pub entry_verb: EntryVerb,
    pub denial_code: EntryFlowDenialCode,
    pub candidate_target_kinds: Vec<TargetKind>,
    pub suggested_reroute: Option<EntryVerb>,
}

/// Outcome of resolving an entry-flow request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryFlowOutcome {
    Resolved(EntryFlowResolved),
    Denied(EntryFlowDenied),
}

/// Resolves an entry-flow request into a reviewed flow sheet.
pub fn resolve_entry_flow(request: EntryFlowRequest) -> EntryFlowOutcome {
    let candidate_target_kinds = match &request.target {
        EntryFlowTarget::Unspecified => Vec::new(),
        EntryFlowTarget::ExplicitTargetKind(kind) => vec![*kind],
        EntryFlowTarget::RawSpecifier(spec) => classify_target_specifier(spec),
    };

    let Some(target_kind) = single_candidate(&candidate_target_kinds) else {
        return EntryFlowOutcome::Denied(EntryFlowDenied {
            entry_verb: request.entry_verb,
            denial_code: EntryFlowDenialCode::TargetKindUnresolved,
            candidate_target_kinds,
            suggested_reroute: None,
        });
    };

    if !entry_verb_allows_target_kind(request.entry_verb, target_kind) {
        return EntryFlowOutcome::Denied(EntryFlowDenied {
            entry_verb: request.entry_verb,
            denial_code: EntryFlowDenialCode::TargetKindMismatch,
            candidate_target_kinds,
            suggested_reroute: suggested_reroute_for(request.entry_verb, target_kind),
        });
    }

    let Some(resulting_mode) = resolve_resulting_mode(
        request.entry_verb,
        target_kind,
        request.preferred_resulting_mode,
    ) else {
        return EntryFlowOutcome::Denied(EntryFlowDenied {
            entry_verb: request.entry_verb,
            denial_code: EntryFlowDenialCode::ResultingModeMismatch,
            candidate_target_kinds,
            suggested_reroute: None,
        });
    };

    EntryFlowOutcome::Resolved(EntryFlowResolved {
        sheet_class: sheet_class_for(request.entry_verb),
        entry_verb: request.entry_verb,
        target_kind,
        resulting_mode,
        candidate_resulting_modes: candidate_resulting_modes_for(request.entry_verb).to_vec(),
    })
}

fn single_candidate(candidates: &[TargetKind]) -> Option<TargetKind> {
    match candidates {
        [only] => Some(*only),
        _ => None,
    }
}

fn sheet_class_for(entry_verb: EntryVerb) -> OpenFlowSheetClass {
    match entry_verb {
        EntryVerb::Open | EntryVerb::AddRoot => OpenFlowSheetClass::OpenLocalTarget,
        EntryVerb::Clone => OpenFlowSheetClass::CloneRemoteTarget,
        EntryVerb::Import => OpenFlowSheetClass::ImportArtifact,
        EntryVerb::Restore | EntryVerb::Resume => OpenFlowSheetClass::RestoreOrResume,
        EntryVerb::StartFromSnapshot => OpenFlowSheetClass::ImportArtifact,
    }
}

fn resolve_resulting_mode(
    entry_verb: EntryVerb,
    target_kind: TargetKind,
    preferred: Option<ResultingMode>,
) -> Option<ResultingMode> {
    if let Some(preferred) = preferred {
        if resulting_mode_allowed(entry_verb, preferred) {
            return Some(preferred);
        }
        return None;
    }

    Some(default_resulting_mode(entry_verb, target_kind))
}

fn default_resulting_mode(entry_verb: EntryVerb, target_kind: TargetKind) -> ResultingMode {
    match entry_verb {
        EntryVerb::Open => match target_kind {
            TargetKind::LocalFile => ResultingMode::SingleFile,
            TargetKind::WorkspaceManifest | TargetKind::WorksetManifest => {
                ResultingMode::WorkspaceWithRoots
            }
            _ => ResultingMode::Folder,
        },
        EntryVerb::Clone => ResultingMode::CloneThenReview,
        EntryVerb::Import => ResultingMode::ExtractThenReview,
        EntryVerb::Restore => ResultingMode::RestoreLastSession,
        EntryVerb::Resume => ResultingMode::ResumeLiveSession,
        EntryVerb::AddRoot => ResultingMode::WorkspaceWithRoots,
        EntryVerb::StartFromSnapshot => ResultingMode::OpenPrebuildWithSetupActions,
    }
}

fn candidate_resulting_modes_for(entry_verb: EntryVerb) -> &'static [ResultingMode] {
    match entry_verb {
        EntryVerb::Open => &[
            ResultingMode::SingleFile,
            ResultingMode::Folder,
            ResultingMode::RepoRoot,
            ResultingMode::WorkspaceCandidate,
            ResultingMode::WorkspaceWithRoots,
            ResultingMode::WorksetSlice,
        ],
        EntryVerb::Clone => &[
            ResultingMode::CloneThenReview,
            ResultingMode::CloneThenOpen,
            ResultingMode::CloneThenAdd,
            ResultingMode::CloneOnly,
        ],
        EntryVerb::Import => &[
            ResultingMode::ExtractThenReview,
            ResultingMode::CompareBeforeRestore,
            ResultingMode::ApplyToActiveWorkspace,
        ],
        EntryVerb::Restore => &[
            ResultingMode::RestoreLastSession,
            ResultingMode::RestoreFromCheckpoint,
        ],
        EntryVerb::AddRoot => &[
            ResultingMode::WorkspaceWithRoots,
            ResultingMode::WorksetSlice,
        ],
        EntryVerb::Resume => &[
            ResultingMode::ResumeLiveSession,
            ResultingMode::OpenPrebuildMinimal,
        ],
        EntryVerb::StartFromSnapshot => &[
            ResultingMode::OpenPrebuildWithSetupActions,
            ResultingMode::OpenPrebuildMinimal,
        ],
    }
}

fn resulting_mode_allowed(entry_verb: EntryVerb, resulting_mode: ResultingMode) -> bool {
    candidate_resulting_modes_for(entry_verb)
        .iter()
        .any(|mode| *mode == resulting_mode)
}

fn entry_verb_allows_target_kind(entry_verb: EntryVerb, target_kind: TargetKind) -> bool {
    match entry_verb {
        EntryVerb::Open => matches!(
            target_kind,
            TargetKind::LocalFile
                | TargetKind::LocalFolder
                | TargetKind::LocalRepoRoot
                | TargetKind::WorkspaceManifest
                | TargetKind::WorksetManifest
        ),
        EntryVerb::Clone => target_kind == TargetKind::RemoteRepository,
        EntryVerb::Import => matches!(
            target_kind,
            TargetKind::PortableStatePackage
                | TargetKind::HandoffPacket
                | TargetKind::CompetitorConfigRoot
                | TargetKind::TemplateOrPrebuildSnapshot
        ),
        EntryVerb::Restore => matches!(
            target_kind,
            TargetKind::RecoveryCheckpoint
                | TargetKind::LocalFolder
                | TargetKind::LocalRepoRoot
                | TargetKind::WorkspaceManifest
                | TargetKind::WorksetManifest
                | TargetKind::ManagedCloudWorkspace
        ),
        EntryVerb::AddRoot => matches!(
            target_kind,
            TargetKind::LocalFolder
                | TargetKind::LocalRepoRoot
                | TargetKind::RemoteRepository
                | TargetKind::SshWorkspace
                | TargetKind::ContainerWorkspace
                | TargetKind::DevcontainerWorkspace
                | TargetKind::ManagedCloudWorkspace
        ),
        EntryVerb::Resume => matches!(
            target_kind,
            TargetKind::ManagedCloudWorkspace
                | TargetKind::SshWorkspace
                | TargetKind::ContainerWorkspace
                | TargetKind::DevcontainerWorkspace
                | TargetKind::TemplateOrPrebuildSnapshot
        ),
        EntryVerb::StartFromSnapshot => target_kind == TargetKind::TemplateOrPrebuildSnapshot,
    }
}

fn suggested_reroute_for(entry_verb: EntryVerb, target_kind: TargetKind) -> Option<EntryVerb> {
    if entry_verb == EntryVerb::Open {
        return match target_kind {
            TargetKind::RemoteRepository => Some(EntryVerb::Clone),
            TargetKind::PortableStatePackage
            | TargetKind::HandoffPacket
            | TargetKind::CompetitorConfigRoot
            | TargetKind::TemplateOrPrebuildSnapshot => Some(EntryVerb::Import),
            TargetKind::RecoveryCheckpoint => Some(EntryVerb::Restore),
            _ => None,
        };
    }
    None
}

fn classify_target_specifier(spec: &str) -> Vec<TargetKind> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Vec::new();
    }

    if spec.starts_with("http://") || spec.starts_with("https://") {
        return vec![TargetKind::RemoteRepository];
    }

    if spec.starts_with("ssh://") {
        return vec![TargetKind::SshWorkspace];
    }

    if spec.ends_with(".zip") || spec.ends_with(".tar") || spec.ends_with(".tar.gz") {
        return vec![TargetKind::PortableStatePackage];
    }

    if spec.contains('/') || spec.contains('\\') {
        return vec![TargetKind::LocalFile, TargetKind::LocalFolder];
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct EntryFlowFixtureRecord {
        record_kind: String,
        entry_flow_case_schema_version: u32,
        case_id: String,
        request: EntryFlowFixtureRequest,
        expect: EntryFlowFixtureExpectation,
    }

    #[derive(Debug, Deserialize)]
    struct EntryFlowFixtureRequest {
        entry_verb: EntryVerb,
        #[serde(default)]
        target_kind_hint: Option<TargetKind>,
        #[serde(default)]
        raw_specifier: Option<String>,
        #[serde(default)]
        preferred_resulting_mode: Option<ResultingMode>,
    }

    #[derive(Debug, Deserialize)]
    struct EntryFlowFixtureExpectation {
        outcome_class: String,
        #[serde(default)]
        sheet_class: Option<OpenFlowSheetClass>,
        #[serde(default)]
        target_kind: Option<TargetKind>,
        #[serde(default)]
        resulting_mode: Option<ResultingMode>,
        #[serde(default)]
        denial_code: Option<EntryFlowDenialCode>,
        #[serde(default)]
        suggested_reroute: Option<EntryVerb>,
    }

    fn resolve_fixture_request(request: EntryFlowFixtureRequest) -> EntryFlowOutcome {
        let target = match (request.target_kind_hint, request.raw_specifier) {
            (Some(kind), _) => EntryFlowTarget::ExplicitTargetKind(kind),
            (None, Some(spec)) => EntryFlowTarget::RawSpecifier(spec),
            (None, None) => EntryFlowTarget::Unspecified,
        };

        resolve_entry_flow(EntryFlowRequest {
            entry_verb: request.entry_verb,
            target,
            preferred_resulting_mode: request.preferred_resulting_mode,
        })
    }

    #[test]
    fn entry_flow_fixtures_resolve_and_fail_closed() {
        let dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/entry_flow_cases");
        let entries = std::fs::read_dir(&dir).expect("entry_flow_cases dir must exist");
        let mut seen = 0usize;

        for entry in entries {
            let entry = entry.expect("dir entry must read");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            seen += 1;

            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let fixture: EntryFlowFixtureRecord =
                serde_json::from_str(&payload).expect("fixture must parse");
            assert_eq!(fixture.record_kind, "entry_flow_case_record");
            assert_eq!(fixture.entry_flow_case_schema_version, 1);
            assert!(!fixture.case_id.trim().is_empty());

            let outcome = resolve_fixture_request(fixture.request);
            match (fixture.expect.outcome_class.as_str(), outcome) {
                ("resolved", EntryFlowOutcome::Resolved(resolved)) => {
                    assert_eq!(Some(resolved.sheet_class), fixture.expect.sheet_class);
                    assert_eq!(Some(resolved.target_kind), fixture.expect.target_kind);
                    assert_eq!(Some(resolved.resulting_mode), fixture.expect.resulting_mode);
                }
                ("denied", EntryFlowOutcome::Denied(denied)) => {
                    assert_eq!(fixture.expect.denial_code, Some(denied.denial_code));
                    assert_eq!(fixture.expect.suggested_reroute, denied.suggested_reroute);
                }
                (expected, actual) => panic!(
                    "fixture outcome mismatch for {}: expected {expected}, got {actual:?}",
                    fixture.case_id
                ),
            }
        }

        assert!(seen > 0, "expected at least one entry-flow fixture case");
    }
}
