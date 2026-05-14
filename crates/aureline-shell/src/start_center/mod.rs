//! Start Center quick-action surface projections.
//!
//! The Start Center is the local-first entry surface rendered when the shell
//! has no active workspace. It advertises the canonical entry verbs (`Open`,
//! `Clone`, `Import`, and `Restore`) without requiring account sign-in, and it
//! resolves preflight/enablement status through the seeded command registry so
//! every surface shares the same truth.

use std::collections::HashMap;
use std::fmt;

use aureline_commands::invocation::ArgumentProvenanceEntry;
use aureline_commands::{CommandEnablementContext, CommandRegistry, PreflightDecision};
use aureline_workspace::{
    classify_recent_work_failure, normalized_recent_work_recovery_actions, RecentWorkEntryRecord,
    RecentWorkFailureState, RecentWorkRegistry, RestoreAvailability, SafeRecoveryAction,
    TargetKind, TrustState,
};
use serde::Deserialize;

use crate::restore::placeholders::{
    recent_work_placeholder_card, PlaceholderSurfaceClass, RecentWorkPlaceholderCard,
};

pub mod admission_review;
pub mod bundles;
pub mod first_useful_work;
pub mod templates;

/// Presentation label rendered for the Start Center surface.
pub const START_CENTER_PRESENTATION_LABEL: &str = "Start Center";

/// Presentation subtitle rendered for the Start Center surface.
pub const START_CENTER_PRESENTATION_SUBTITLE: &str =
    "Open, clone, restore, or import. No account required.";

/// Start Center primary actions that must remain distinct on first-run entry
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartCenterPrimaryActionId {
    OpenFolder,
    OpenWorkspace,
    CloneRepository,
    RestoreLastSession,
    ImportFrom,
}

impl StartCenterPrimaryActionId {
    /// Returns the stable surface-local ordering for Start Center actions.
    pub const fn ordered() -> &'static [Self] {
        &[
            Self::OpenFolder,
            Self::OpenWorkspace,
            Self::CloneRepository,
            Self::RestoreLastSession,
            Self::ImportFrom,
        ]
    }

    /// Returns the verb-first label rendered on the Start Center surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenFolder => "Open folder",
            Self::OpenWorkspace => "Open workspace",
            Self::CloneRepository => "Clone repository",
            Self::RestoreLastSession => "Restore last session",
            Self::ImportFrom => "Import from…",
        }
    }

    /// Returns the short, surface-local summary for the action.
    pub const fn summary(self) -> &'static str {
        match self {
            Self::OpenFolder => "Open a local folder as the active workspace.",
            Self::OpenWorkspace => "Open a saved workspace file.",
            Self::CloneRepository => "Clone a remote repository into a new workspace.",
            Self::RestoreLastSession => "Restore state from the most recent checkpoint.",
            Self::ImportFrom => "Import settings and shortcuts from another tool.",
        }
    }

    /// Returns the canonical command id this action resolves through.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::OpenFolder | Self::OpenWorkspace => "cmd:workspace.open_folder",
            Self::CloneRepository => "cmd:workspace.clone_repository",
            Self::RestoreLastSession => "cmd:workspace.restore_from_checkpoint",
            Self::ImportFrom => "cmd:workspace.import_profile",
        }
    }

    /// Returns a stable token used for message ids and fixture refs.
    pub const fn token(self) -> &'static str {
        match self {
            Self::OpenFolder => "open_folder",
            Self::OpenWorkspace => "open_workspace",
            Self::CloneRepository => "clone_repository",
            Self::RestoreLastSession => "restore_last_session",
            Self::ImportFrom => "import_from",
        }
    }
}

/// Runtime posture required to evaluate Start Center action enablement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartCenterRuntimeInputs<'a> {
    /// Client scope the Start Center issues commands under (usually
    /// `desktop_product`).
    pub client_scope: &'a str,
    /// Current workspace trust state token (`trusted`, `restricted`, ...).
    pub workspace_trust_state: &'a str,
    /// Whether an execution context is available for command dispatch.
    pub execution_context_available: bool,
    /// Optional provider-linked state when a command depends on a provider.
    pub provider_linked: Option<bool>,
    /// Optional credential availability when a command depends on credentials.
    pub credential_available: Option<bool>,
    /// Whether commands are disabled globally by policy.
    pub policy_disabled: bool,
    /// Whether commands are blocked in the current context by policy.
    pub policy_blocked_in_context: bool,
    /// Whether Labs commands are explicitly enabled for this local session.
    pub labs_enabled: bool,
}

/// One Start Center action row projected from the canonical command registry.
#[derive(Debug, Clone)]
pub struct StartCenterActionRow {
    pub action_id: StartCenterPrimaryActionId,
    pub title: &'static str,
    pub summary: &'static str,
    pub command_id: &'static str,
    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    pub preflight: Option<PreflightDecision>,
}

/// Privacy posture applied to Start Center recent-work metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartCenterRecentWorkPrivacyMode {
    /// Render recent-work metadata normally.
    Default,
    /// Hide path and host details while keeping rows visible.
    HidePaths,
    /// Hide recent-work rows while preserving primary entry actions.
    HideRecentWork,
    /// Hide recent-work rows and account affordances; keep open and clone entry.
    HideAllExceptOpenAndClone,
}

impl StartCenterRecentWorkPrivacyMode {
    /// Returns the stable string vocabulary for privacy mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::HidePaths => "hide_paths",
            Self::HideRecentWork => "hide_recent_work",
            Self::HideAllExceptOpenAndClone => "hide_all_except_open_and_clone",
        }
    }
}

/// Start Center projection of a canonical recent-work entry.
#[derive(Debug, Clone)]
pub struct StartCenterRecentWorkRow {
    pub recent_work_id: String,
    pub primary_label: String,
    pub location_or_target_subtitle: Option<String>,
    pub target_kind: TargetKind,
    pub target_kind_label: &'static str,
    pub failure_state: RecentWorkFailureState,
    pub trust_state: TrustState,
    pub restore_availability: RestoreAvailability,
    pub pinned: bool,
    pub safe_recovery_actions: Vec<SafeRecoveryAction>,
    pub placeholder_card: Option<RecentWorkPlaceholderCard>,
    pub privacy_redaction_applied: bool,
}

/// Start Center recent-work projection plus privacy-preserved entry affordances.
#[derive(Debug, Clone)]
pub struct StartCenterRecentWorkProjection {
    pub privacy_mode: StartCenterRecentWorkPrivacyMode,
    pub rows: Vec<StartCenterRecentWorkRow>,
    pub metadata_hidden: bool,
    pub local_open_still_available: bool,
    pub workspace_open_still_available: bool,
    pub restore_local_state_still_available: bool,
    pub clear_recent_work_available: bool,
}

/// Mutable Start Center interaction state (selection, focus).
#[derive(Debug, Clone)]
pub struct StartCenterState {
    selection: usize,
}

impl StartCenterState {
    /// Creates a new Start Center state with the first action selected.
    pub const fn new() -> Self {
        Self { selection: 0 }
    }

    /// Returns the currently selected row index.
    pub const fn selection(&self) -> usize {
        self.selection
    }

    /// Selects a row by index, clamping to the available row count.
    pub fn select_index(&mut self, index: usize, row_count: usize) {
        if row_count == 0 {
            self.selection = 0;
        } else {
            self.selection = index.min(row_count - 1);
        }
    }

    /// Advances the selection by one, wrapping at `row_count`.
    pub fn select_next(&mut self, row_count: usize) {
        if row_count == 0 {
            self.selection = 0;
            return;
        }
        self.selection = (self.selection + 1) % row_count;
    }

    /// Moves the selection up by one, wrapping at `row_count`.
    pub fn select_prev(&mut self, row_count: usize) {
        if row_count == 0 {
            self.selection = 0;
            return;
        }
        self.selection = self.selection.wrapping_add(row_count - 1) % row_count;
    }
}

/// Builds the canonical Start Center action rows for the provided runtime.
pub fn build_action_rows(
    registry: &CommandRegistry,
    runtime: StartCenterRuntimeInputs<'_>,
) -> Vec<StartCenterActionRow> {
    let mut rows = Vec::new();
    for action_id in StartCenterPrimaryActionId::ordered() {
        let command_id = action_id.command_id();
        let Some(entry) = registry.get(command_id) else {
            rows.push(StartCenterActionRow {
                action_id: *action_id,
                title: action_id.label(),
                summary: action_id.summary(),
                command_id,
                argument_provenance_map: Vec::new(),
                preflight: None,
            });
            continue;
        };

        let argument_provenance_map = match action_id {
            StartCenterPrimaryActionId::CloneRepository => clone_repository_placeholder_arguments(),
            StartCenterPrimaryActionId::RestoreLastSession => {
                restore_from_checkpoint_placeholder_arguments()
            }
            StartCenterPrimaryActionId::OpenFolder | StartCenterPrimaryActionId::ImportFrom => {
                crate::commands::argument_provenance_map_for(entry)
            }
            StartCenterPrimaryActionId::OpenWorkspace => {
                let mut map = crate::commands::argument_provenance_map_for(entry);
                override_open_folder_scope_to_workspace_file(&mut map);
                map
            }
        };

        let context = CommandEnablementContext {
            client_scope: runtime.client_scope.to_string(),
            workspace_trust_state: runtime.workspace_trust_state.to_string(),
            execution_context_available: runtime.execution_context_available,
            provider_linked: runtime.provider_linked,
            credential_available: runtime.credential_available,
            policy_disabled: runtime.policy_disabled,
            policy_blocked_in_context: runtime.policy_blocked_in_context,
            labs_enabled: runtime.labs_enabled,
            argument_provenance_map: argument_provenance_map.clone(),
        };

        rows.push(StartCenterActionRow {
            action_id: *action_id,
            title: action_id.label(),
            summary: action_id.summary(),
            command_id,
            argument_provenance_map,
            preflight: Some(entry.preflight(&context)),
        });
    }
    rows
}

/// Builds Start Center recent-work rows from the canonical registry.
pub fn build_recent_work_rows(
    registry: &RecentWorkRegistry,
    privacy_mode: StartCenterRecentWorkPrivacyMode,
) -> StartCenterRecentWorkProjection {
    let metadata_hidden = matches!(
        privacy_mode,
        StartCenterRecentWorkPrivacyMode::HideRecentWork
            | StartCenterRecentWorkPrivacyMode::HideAllExceptOpenAndClone
    );
    let rows = if metadata_hidden {
        Vec::new()
    } else {
        registry
            .entries
            .iter()
            .map(|entry| start_center_recent_work_row(entry, privacy_mode))
            .collect()
    };

    StartCenterRecentWorkProjection {
        privacy_mode,
        rows,
        metadata_hidden,
        local_open_still_available: true,
        workspace_open_still_available: true,
        restore_local_state_still_available: true,
        clear_recent_work_available: !registry.entries.is_empty(),
    }
}

fn start_center_recent_work_row(
    entry: &RecentWorkEntryRecord,
    privacy_mode: StartCenterRecentWorkPrivacyMode,
) -> StartCenterRecentWorkRow {
    let privacy_redaction_applied = privacy_mode == StartCenterRecentWorkPrivacyMode::HidePaths;
    let location_or_target_subtitle = if privacy_redaction_applied {
        Some(entry.target_kind.surface_label().to_string())
    } else {
        entry.presentation_subtitle.clone()
    };
    StartCenterRecentWorkRow {
        recent_work_id: entry.recent_work_id.clone(),
        primary_label: entry.presentation_label.clone(),
        location_or_target_subtitle,
        target_kind: entry.target_kind,
        target_kind_label: entry.target_kind.surface_label(),
        failure_state: classify_recent_work_failure(entry),
        trust_state: entry.trust_state,
        restore_availability: entry.restore_availability,
        pinned: entry.pinned,
        safe_recovery_actions: normalized_recent_work_recovery_actions(entry),
        placeholder_card: recent_work_placeholder_card(
            entry,
            PlaceholderSurfaceClass::StartCenterRecentWork,
        ),
        privacy_redaction_applied,
    }
}

const ALPHA_BUNDLE_MANIFESTS: &[(&str, &str)] = &[
    (
        "artifacts/bundles/tsjs_launch_bundle_alpha.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/bundles/tsjs_launch_bundle_alpha.yaml"
        )),
    ),
    (
        "artifacts/bundles/python_launch_bundle_alpha.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/bundles/python_launch_bundle_alpha.yaml"
        )),
    ),
];

/// Start Center projection for one external-alpha launch bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterBundleGalleryRow {
    /// Stable launch bundle id from the canonical manifest.
    pub bundle_id: String,
    /// Persona and stack label rendered as one compact Start Center line.
    pub persona_or_stack_label: String,
    /// Signer label shown with the bundle source to keep provenance visible.
    pub signer_label: String,
    /// Source label shown next to the signer.
    pub source_label: String,
    /// Release channel the bundle belongs to.
    pub channel: String,
    /// Compatible Aureline version range copied from the manifest.
    pub compatible_aureline_range: String,
    /// Archetype seed row linked by this bundle.
    pub archetype_seed_row_ref: String,
    /// Current certification state; seed rows must not render as certified.
    pub certification_state: String,
    /// Combined online, mirror, and offline availability label.
    pub mirror_availability_label: String,
    /// Evidence packet opened by bundle or archetype badging.
    pub evidence_packet_ref: String,
    /// Explicit user choices supported by install/update and drift review.
    pub available_actions: Vec<String>,
}

/// Error returned when the Start Center cannot project a checked-in bundle manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaBundleGalleryError {
    manifest_ref: &'static str,
    message: String,
}

impl AlphaBundleGalleryError {
    /// Returns the manifest path that failed to project.
    pub const fn manifest_ref(&self) -> &'static str {
        self.manifest_ref
    }

    /// Returns the parse or projection failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for AlphaBundleGalleryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.manifest_ref, self.message)
    }
}

impl std::error::Error for AlphaBundleGalleryError {}

/// Builds the Start Center bundle-gallery rows from the canonical launch bundle manifests.
///
/// # Errors
///
/// Returns [`AlphaBundleGalleryError`] when one of the checked-in YAML manifests cannot
/// be parsed or lacks a field required by the Start Center projection.
pub fn build_alpha_bundle_gallery_rows(
) -> Result<Vec<StartCenterBundleGalleryRow>, AlphaBundleGalleryError> {
    ALPHA_BUNDLE_MANIFESTS
        .iter()
        .map(|(manifest_ref, contents)| project_alpha_bundle_manifest(manifest_ref, contents))
        .collect()
}

/// Renders the Start Center bundle-gallery projection as deterministic plaintext.
///
/// # Errors
///
/// Returns [`AlphaBundleGalleryError`] when [`build_alpha_bundle_gallery_rows`] cannot
/// project one of the canonical manifests.
pub fn render_alpha_bundle_gallery_plaintext() -> Result<String, AlphaBundleGalleryError> {
    let rows = build_alpha_bundle_gallery_rows()?;
    let mut lines = vec![
        "External alpha bundle gallery".to_string(),
        "bundle_id | persona_or_stack | signer/source | channel | compatible_range | archetype_state | mirror".to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {} ({}) | {} | {} | {} | {}",
            row.bundle_id,
            row.persona_or_stack_label,
            row.signer_label,
            row.source_label,
            row.channel,
            row.compatible_aureline_range,
            row.certification_state,
            row.mirror_availability_label
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

#[derive(Debug, Deserialize)]
struct AlphaBundleManifestDoc {
    bundle_id: String,
    stack_identity: AlphaBundleStackIdentityDoc,
    source: AlphaBundleSourceDoc,
    mirror_availability: AlphaBundleMirrorAvailabilityDoc,
    install_update_review: AlphaBundleInstallReviewDoc,
    evidence_binding: AlphaBundleEvidenceBindingDoc,
}

#[derive(Debug, Deserialize)]
struct AlphaBundleStackIdentityDoc {
    persona_label: String,
    stack_label: String,
    channel: String,
    compatible_aureline_range: String,
}

#[derive(Debug, Deserialize)]
struct AlphaBundleSourceDoc {
    source_label: String,
    signer_label: String,
}

#[derive(Debug, Deserialize)]
struct AlphaBundleMirrorAvailabilityDoc {
    online_source: String,
    approved_mirror: String,
    offline_bundle: String,
}

#[derive(Debug, Deserialize)]
struct AlphaBundleInstallReviewDoc {
    action_vocabulary: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AlphaBundleEvidenceBindingDoc {
    archetype_seed_row_ref: String,
    archetype_evidence_packet_ref: String,
    certification_state: String,
}

fn project_alpha_bundle_manifest(
    manifest_ref: &'static str,
    contents: &str,
) -> Result<StartCenterBundleGalleryRow, AlphaBundleGalleryError> {
    let doc: AlphaBundleManifestDoc =
        serde_yaml::from_str(contents).map_err(|err| AlphaBundleGalleryError {
            manifest_ref,
            message: err.to_string(),
        })?;
    if doc.install_update_review.action_vocabulary.is_empty() {
        return Err(AlphaBundleGalleryError {
            manifest_ref,
            message: "install_update_review.action_vocabulary must not be empty".to_string(),
        });
    }
    Ok(StartCenterBundleGalleryRow {
        bundle_id: doc.bundle_id,
        persona_or_stack_label: format!(
            "{} - {}",
            doc.stack_identity.persona_label, doc.stack_identity.stack_label
        ),
        signer_label: doc.source.signer_label,
        source_label: doc.source.source_label,
        channel: doc.stack_identity.channel,
        compatible_aureline_range: doc.stack_identity.compatible_aureline_range,
        archetype_seed_row_ref: doc.evidence_binding.archetype_seed_row_ref,
        certification_state: doc.evidence_binding.certification_state,
        mirror_availability_label: format!(
            "{}/{}/{}",
            doc.mirror_availability.online_source,
            doc.mirror_availability.approved_mirror,
            doc.mirror_availability.offline_bundle
        ),
        evidence_packet_ref: doc.evidence_binding.archetype_evidence_packet_ref,
        available_actions: doc.install_update_review.action_vocabulary,
    })
}

const WORKSPACE_TEMPLATE_SEED_MANIFEST: (&str, &str) = (
    "artifacts/templates/workspace_template_seed.yaml",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/workspace_template_seed.yaml"
    )),
);

/// Start Center projection for one workspace-template seed row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterWorkspaceTemplateSeedRow {
    /// Stable workspace-template id from the seed manifest.
    pub template_id: String,
    /// Human-readable template label.
    pub display_label: String,
    /// Support class copied from the template seed.
    pub support_class: String,
    /// Runtime and toolchain scope copied from the template seed.
    pub runtime_and_toolchain_scope: String,
    /// Environment capsule the template hydrates.
    pub environment_capsule_ref: String,
    /// Launch bundle ids that may reference this template.
    pub launch_bundle_refs: Vec<String>,
    /// Target class exposed by the capsule.
    pub target_class: String,
    /// Capsule location class exposed by the capsule.
    pub capsule_location_class: String,
    /// Boundary class exposed by the capsule.
    pub boundary_class: String,
    /// Network posture exposed by the capsule.
    pub network_posture: String,
    /// Toolchain ids and classes the capsule names.
    pub toolchain_summary: Vec<String>,
    /// Environment variable names projected without raw values.
    pub environment_variable_names: Vec<String>,
    /// Number of variables represented by credential alias and secret class.
    pub secret_alias_count: usize,
    /// Number of variables that attempted to include a raw value.
    pub raw_value_included_count: usize,
    /// Prebuild reuse state from the template row.
    pub prebuild_reuse_state: String,
    /// Stale-prebuild behavior from the template row.
    pub stale_behavior: String,
    /// Same-weight bypass paths shown by the template surface.
    pub bypass_path_ids: Vec<String>,
    /// Reviewable summary for compact template rows.
    pub review_summary: String,
}

/// Error returned when the Start Center cannot project the workspace-template seed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceTemplateSeedError {
    manifest_ref: &'static str,
    message: String,
}

impl WorkspaceTemplateSeedError {
    /// Returns the manifest path that failed to project.
    pub const fn manifest_ref(&self) -> &'static str {
        self.manifest_ref
    }

    /// Returns the parse or projection failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for WorkspaceTemplateSeedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.manifest_ref, self.message)
    }
}

impl std::error::Error for WorkspaceTemplateSeedError {}

/// Builds Start Center rows from the canonical workspace-template seed manifest.
///
/// # Errors
///
/// Returns [`WorkspaceTemplateSeedError`] when the checked-in YAML manifest cannot be
/// parsed, references a missing capsule, or attempts to project raw environment values.
pub fn build_workspace_template_seed_rows(
) -> Result<Vec<StartCenterWorkspaceTemplateSeedRow>, WorkspaceTemplateSeedError> {
    project_workspace_template_seed_manifest(
        WORKSPACE_TEMPLATE_SEED_MANIFEST.0,
        WORKSPACE_TEMPLATE_SEED_MANIFEST.1,
    )
}

/// Renders the workspace-template seed projection as deterministic plaintext.
///
/// # Errors
///
/// Returns [`WorkspaceTemplateSeedError`] when [`build_workspace_template_seed_rows`]
/// cannot project the canonical seed.
pub fn render_workspace_template_seed_plaintext() -> Result<String, WorkspaceTemplateSeedError> {
    let rows = build_workspace_template_seed_rows()?;
    let mut lines = vec![
        "Workspace template seed gallery".to_string(),
        "template_id | capsule | launch_bundles | target | toolchains | variables | prebuild"
            .to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {} | {}/{} | {} | vars={} secret_aliases={} raw_values={} | {}/{}",
            row.template_id,
            row.environment_capsule_ref,
            row.launch_bundle_refs.join(","),
            row.target_class,
            row.boundary_class,
            row.toolchain_summary.join(","),
            row.environment_variable_names.len(),
            row.secret_alias_count,
            row.raw_value_included_count,
            row.prebuild_reuse_state,
            row.stale_behavior
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

#[derive(Debug, Deserialize)]
struct WorkspaceTemplateSeedManifestDoc {
    environment_capsules: Vec<EnvironmentCapsuleAlphaDoc>,
    workspace_templates: Vec<WorkspaceTemplateSeedDoc>,
}

#[derive(Debug, Deserialize)]
struct EnvironmentCapsuleAlphaDoc {
    capsule_id: String,
    target_plan: EnvironmentCapsuleTargetPlanDoc,
    toolchains: Vec<EnvironmentCapsuleToolchainDoc>,
    environment_variables: Vec<EnvironmentCapsuleVariableDoc>,
}

#[derive(Debug, Deserialize)]
struct EnvironmentCapsuleTargetPlanDoc {
    target_class: String,
    capsule_location_class: String,
    boundary_class: String,
    network_posture: String,
}

#[derive(Debug, Deserialize)]
struct EnvironmentCapsuleToolchainDoc {
    toolchain_class: String,
    toolchain_id: String,
}

#[derive(Debug, Deserialize)]
struct EnvironmentCapsuleVariableDoc {
    variable_name: String,
    value_source: String,
    secret_class: Option<String>,
    raw_value_included: bool,
}

#[derive(Debug, Deserialize)]
struct WorkspaceTemplateSeedDoc {
    template_id: String,
    display_label: String,
    support_class: String,
    runtime_and_toolchain_scope: String,
    environment_capsule_ref: String,
    launch_bundle_refs: Vec<String>,
    bypass_path_ids: Vec<String>,
    review_summary: String,
    prebuild_reuse_policy: WorkspaceTemplatePrebuildPolicyDoc,
}

#[derive(Debug, Deserialize)]
struct WorkspaceTemplatePrebuildPolicyDoc {
    reuse_state: String,
    stale_behavior: String,
}

fn project_workspace_template_seed_manifest(
    manifest_ref: &'static str,
    contents: &str,
) -> Result<Vec<StartCenterWorkspaceTemplateSeedRow>, WorkspaceTemplateSeedError> {
    let doc: WorkspaceTemplateSeedManifestDoc =
        serde_yaml::from_str(contents).map_err(|err| WorkspaceTemplateSeedError {
            manifest_ref,
            message: err.to_string(),
        })?;
    let capsules: HashMap<&str, &EnvironmentCapsuleAlphaDoc> = doc
        .environment_capsules
        .iter()
        .map(|capsule| (capsule.capsule_id.as_str(), capsule))
        .collect();
    let mut rows = Vec::with_capacity(doc.workspace_templates.len());
    for template in doc.workspace_templates {
        if template.launch_bundle_refs.is_empty() {
            return Err(WorkspaceTemplateSeedError {
                manifest_ref,
                message: format!(
                    "{} launch_bundle_refs must not be empty",
                    template.template_id
                ),
            });
        }
        if template.bypass_path_ids.is_empty() {
            return Err(WorkspaceTemplateSeedError {
                manifest_ref,
                message: format!("{} bypass_path_ids must not be empty", template.template_id),
            });
        }
        let Some(capsule) = capsules.get(template.environment_capsule_ref.as_str()) else {
            return Err(WorkspaceTemplateSeedError {
                manifest_ref,
                message: format!(
                    "{} references missing capsule {}",
                    template.template_id, template.environment_capsule_ref
                ),
            });
        };
        let raw_value_included_count = capsule
            .environment_variables
            .iter()
            .filter(|variable| variable.raw_value_included)
            .count();
        if raw_value_included_count > 0 {
            return Err(WorkspaceTemplateSeedError {
                manifest_ref,
                message: format!(
                    "{} attempted to project raw environment values",
                    capsule.capsule_id
                ),
            });
        }
        let secret_alias_count = capsule
            .environment_variables
            .iter()
            .filter(|variable| {
                variable.value_source == "secret_alias" || variable.secret_class.is_some()
            })
            .count();
        rows.push(StartCenterWorkspaceTemplateSeedRow {
            template_id: template.template_id,
            display_label: template.display_label,
            support_class: template.support_class,
            runtime_and_toolchain_scope: template.runtime_and_toolchain_scope,
            environment_capsule_ref: template.environment_capsule_ref,
            launch_bundle_refs: template.launch_bundle_refs,
            target_class: capsule.target_plan.target_class.clone(),
            capsule_location_class: capsule.target_plan.capsule_location_class.clone(),
            boundary_class: capsule.target_plan.boundary_class.clone(),
            network_posture: capsule.target_plan.network_posture.clone(),
            toolchain_summary: capsule
                .toolchains
                .iter()
                .map(|toolchain| {
                    format!("{}:{}", toolchain.toolchain_id, toolchain.toolchain_class)
                })
                .collect(),
            environment_variable_names: capsule
                .environment_variables
                .iter()
                .map(|variable| variable.variable_name.clone())
                .collect(),
            secret_alias_count,
            raw_value_included_count,
            prebuild_reuse_state: template.prebuild_reuse_policy.reuse_state,
            stale_behavior: template.prebuild_reuse_policy.stale_behavior,
            bypass_path_ids: template.bypass_path_ids,
            review_summary: template.review_summary,
        });
    }
    Ok(rows)
}

const WARM_START_DESCRIPTOR_SEED_MANIFEST: (&str, &str) = (
    "artifacts/templates/warm_start_descriptor_seed.yaml",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/warm_start_descriptor_seed.yaml"
    )),
);

/// Start Center projection for one warm-start descriptor seed row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterWarmStartDescriptorSeedRow {
    /// Stable descriptor id from the warm-start seed manifest.
    pub descriptor_id: String,
    /// Human-readable descriptor label.
    pub display_label: String,
    /// Source class for the snapshot or cache metadata.
    pub source_class: String,
    /// Source artifact or object ref backing the descriptor.
    pub source_ref: String,
    /// Freshness state copied from the descriptor.
    pub freshness_state: String,
    /// Age class copied from the descriptor.
    pub age_class: String,
    /// Target class copied from the descriptor.
    pub target_class: String,
    /// Runtime boundary class copied from the descriptor.
    pub boundary_class: String,
    /// Environment capsule the descriptor was fingerprinted against.
    pub environment_capsule_ref: String,
    /// Warm-start state shared by launch, entry, and support review surfaces.
    pub warm_start_state: String,
    /// Prebuild reuse state from execution-context vocabulary.
    pub reuse_state: String,
    /// Cache disposition from execution-context vocabulary.
    pub cache_disposition: String,
    /// Optional invalidation reason for rejected descriptors.
    pub invalidation_reason: Option<String>,
    /// Resume posture for live or cached runtime state.
    pub resume_capability: String,
    /// Explicit materializer claim; seed rows must remain metadata-only.
    pub materializer_claim: String,
    /// Explicit live-runtime claim; seed rows must not pretend an attach exists.
    pub live_runtime_claim: String,
    /// Launch bundles that can surface this row.
    pub launch_bundle_refs: Vec<String>,
    /// Project-entry reviews that can surface this row.
    pub project_entry_review_refs: Vec<String>,
    /// Same-weight fallback action for cold, review, or reauth paths.
    pub fallback_action_id: String,
    /// Review surfaces that consume this descriptor.
    pub review_surfaces: Vec<String>,
    /// Number of drift markers attached to the descriptor.
    pub drift_marker_count: usize,
    /// Whether raw secret values were attempted in the seed.
    pub raw_secret_values_included: bool,
    /// Whether raw command lines were attempted in the seed.
    pub raw_command_lines_included: bool,
    /// Whether the descriptor requires secret or credential revalidation.
    pub secret_revalidation_required: bool,
    /// Reviewable summary for compact warm-start rows.
    pub review_summary: String,
}

/// Error returned when the Start Center cannot project the warm-start seed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarmStartDescriptorSeedError {
    manifest_ref: &'static str,
    message: String,
}

impl WarmStartDescriptorSeedError {
    /// Returns the manifest path that failed to project.
    pub const fn manifest_ref(&self) -> &'static str {
        self.manifest_ref
    }

    /// Returns the parse or projection failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for WarmStartDescriptorSeedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.manifest_ref, self.message)
    }
}

impl std::error::Error for WarmStartDescriptorSeedError {}

/// Builds Start Center rows from the canonical warm-start descriptor seed.
///
/// # Errors
///
/// Returns [`WarmStartDescriptorSeedError`] when the checked-in YAML manifest cannot
/// be parsed, lacks launch or project-entry review reachability, or attempts to
/// project raw secret or command material.
pub fn build_warm_start_descriptor_seed_rows(
) -> Result<Vec<StartCenterWarmStartDescriptorSeedRow>, WarmStartDescriptorSeedError> {
    project_warm_start_descriptor_seed_manifest(
        WARM_START_DESCRIPTOR_SEED_MANIFEST.0,
        WARM_START_DESCRIPTOR_SEED_MANIFEST.1,
    )
}

/// Renders the warm-start descriptor seed projection as deterministic plaintext.
///
/// # Errors
///
/// Returns [`WarmStartDescriptorSeedError`] when
/// [`build_warm_start_descriptor_seed_rows`] cannot project the canonical seed.
pub fn render_warm_start_descriptor_seed_plaintext() -> Result<String, WarmStartDescriptorSeedError>
{
    let rows = build_warm_start_descriptor_seed_rows()?;
    let mut lines = vec![
        "Warm-start descriptor seed gallery".to_string(),
        "descriptor_id | source | freshness | target | warm_state | resume | claim".to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {}:{} | {}/{} | {}/{} | {}/{} | {} | {}:{}",
            row.descriptor_id,
            row.source_class,
            row.source_ref,
            row.freshness_state,
            row.age_class,
            row.target_class,
            row.boundary_class,
            row.warm_start_state,
            row.reuse_state,
            row.resume_capability,
            row.materializer_claim,
            row.live_runtime_claim
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

#[derive(Debug, Deserialize)]
struct WarmStartDescriptorSeedManifestDoc {
    prebuild_descriptors: Vec<PrebuildDescriptorAlphaDoc>,
}

#[derive(Debug, Deserialize)]
struct PrebuildDescriptorAlphaDoc {
    descriptor_id: String,
    display_label: String,
    source_identity: WarmStartSourceIdentityDoc,
    freshness: WarmStartFreshnessDoc,
    target: WarmStartTargetDoc,
    compatibility_fingerprint: WarmStartCompatibilityFingerprintDoc,
    warm_start_descriptor: WarmStartDescriptorDoc,
    safety: WarmStartSafetyDoc,
    drift_markers: Vec<WarmStartDriftMarkerDoc>,
    review_surfaces: Vec<String>,
    launch_bundle_refs: Vec<String>,
    project_entry_review_refs: Vec<String>,
    review_summary: String,
}

#[derive(Debug, Deserialize)]
struct WarmStartSourceIdentityDoc {
    source_class: String,
    source_ref: String,
}

#[derive(Debug, Deserialize)]
struct WarmStartFreshnessDoc {
    freshness_state: String,
    age_class: String,
}

#[derive(Debug, Deserialize)]
struct WarmStartTargetDoc {
    target_class: String,
    boundary_class: String,
}

#[derive(Debug, Deserialize)]
struct WarmStartCompatibilityFingerprintDoc {
    capsule_ref: String,
}

#[derive(Debug, Deserialize)]
struct WarmStartDescriptorDoc {
    warm_start_state: String,
    reuse_state: String,
    invalidation_reason: Option<String>,
    cache_disposition: String,
    resume_capability: String,
    materializer_claim: String,
    live_runtime_claim: String,
    fallback_action_id: String,
}

#[derive(Debug, Deserialize)]
struct WarmStartSafetyDoc {
    raw_secret_values_included: bool,
    raw_command_lines_included: bool,
    secret_revalidation_required: bool,
}

#[derive(Debug, Deserialize)]
struct WarmStartDriftMarkerDoc {}

fn project_warm_start_descriptor_seed_manifest(
    manifest_ref: &'static str,
    contents: &str,
) -> Result<Vec<StartCenterWarmStartDescriptorSeedRow>, WarmStartDescriptorSeedError> {
    let doc: WarmStartDescriptorSeedManifestDoc =
        serde_yaml::from_str(contents).map_err(|err| WarmStartDescriptorSeedError {
            manifest_ref,
            message: err.to_string(),
        })?;
    let mut rows = Vec::with_capacity(doc.prebuild_descriptors.len());
    for descriptor in doc.prebuild_descriptors {
        if descriptor.launch_bundle_refs.is_empty()
            && descriptor.project_entry_review_refs.is_empty()
        {
            return Err(WarmStartDescriptorSeedError {
                manifest_ref,
                message: format!(
                    "{} must be reachable from a launch-bundle or project-entry review",
                    descriptor.descriptor_id
                ),
            });
        }
        if !descriptor
            .review_surfaces
            .iter()
            .any(|surface| surface == "start_center")
        {
            return Err(WarmStartDescriptorSeedError {
                manifest_ref,
                message: format!(
                    "{} is not projected to Start Center",
                    descriptor.descriptor_id
                ),
            });
        }
        if descriptor.safety.raw_secret_values_included {
            return Err(WarmStartDescriptorSeedError {
                manifest_ref,
                message: format!(
                    "{} attempted to project raw secrets",
                    descriptor.descriptor_id
                ),
            });
        }
        if descriptor.safety.raw_command_lines_included {
            return Err(WarmStartDescriptorSeedError {
                manifest_ref,
                message: format!(
                    "{} attempted to project raw command lines",
                    descriptor.descriptor_id
                ),
            });
        }
        rows.push(StartCenterWarmStartDescriptorSeedRow {
            descriptor_id: descriptor.descriptor_id,
            display_label: descriptor.display_label,
            source_class: descriptor.source_identity.source_class,
            source_ref: descriptor.source_identity.source_ref,
            freshness_state: descriptor.freshness.freshness_state,
            age_class: descriptor.freshness.age_class,
            target_class: descriptor.target.target_class,
            boundary_class: descriptor.target.boundary_class,
            environment_capsule_ref: descriptor.compatibility_fingerprint.capsule_ref,
            warm_start_state: descriptor.warm_start_descriptor.warm_start_state,
            reuse_state: descriptor.warm_start_descriptor.reuse_state,
            cache_disposition: descriptor.warm_start_descriptor.cache_disposition,
            invalidation_reason: descriptor.warm_start_descriptor.invalidation_reason,
            resume_capability: descriptor.warm_start_descriptor.resume_capability,
            materializer_claim: descriptor.warm_start_descriptor.materializer_claim,
            live_runtime_claim: descriptor.warm_start_descriptor.live_runtime_claim,
            launch_bundle_refs: descriptor.launch_bundle_refs,
            project_entry_review_refs: descriptor.project_entry_review_refs,
            fallback_action_id: descriptor.warm_start_descriptor.fallback_action_id,
            review_surfaces: descriptor.review_surfaces,
            drift_marker_count: descriptor.drift_markers.len(),
            raw_secret_values_included: descriptor.safety.raw_secret_values_included,
            raw_command_lines_included: descriptor.safety.raw_command_lines_included,
            secret_revalidation_required: descriptor.safety.secret_revalidation_required,
            review_summary: descriptor.review_summary,
        });
    }
    Ok(rows)
}

fn override_open_folder_scope_to_workspace_file(
    argument_provenance_map: &mut [ArgumentProvenanceEntry],
) {
    let Some(row) = argument_provenance_map
        .iter_mut()
        .find(|row| row.argument_name == "workspace_scope_ref")
    else {
        return;
    };
    row.resolved_value_ref = Some("workspace-scope:workspace_file:recent:01".to_string());
}

fn clone_repository_placeholder_arguments() -> Vec<ArgumentProvenanceEntry> {
    vec![
        ArgumentProvenanceEntry {
            argument_name: "remote_repository_ref".to_string(),
            provenance: "user_selected_from_palette_suggestion".to_string(),
            resolved_value_ref: Some("provider:git:example:01".to_string()),
        },
        ArgumentProvenanceEntry {
            argument_name: "destination_root_ref".to_string(),
            provenance: "default_from_descriptor".to_string(),
            resolved_value_ref: None,
        },
        ArgumentProvenanceEntry {
            argument_name: "open_after_clone".to_string(),
            provenance: "default_from_descriptor".to_string(),
            resolved_value_ref: Some("value:bool:true".to_string()),
        },
    ]
}

fn restore_from_checkpoint_placeholder_arguments() -> Vec<ArgumentProvenanceEntry> {
    vec![
        ArgumentProvenanceEntry {
            argument_name: "checkpoint_ref".to_string(),
            provenance: "user_selected_from_palette_suggestion".to_string(),
            resolved_value_ref: Some("checkpoint:seed:workspace_restore:01".to_string()),
        },
        ArgumentProvenanceEntry {
            argument_name: "restore_scope".to_string(),
            provenance: "default_from_descriptor".to_string(),
            resolved_value_ref: Some(
                "enum:workspace.restore_from_checkpoint:workspace".to_string(),
            ),
        },
        ArgumentProvenanceEntry {
            argument_name: "create_safety_checkpoint".to_string(),
            provenance: "default_from_descriptor".to_string(),
            resolved_value_ref: Some("value:bool:true".to_string()),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_commands::registry::seeded_registry;
    use aureline_workspace::{
        PortabilityClass, RecentWorkEntryRecordKind, RecentWorkRegistryRecordKind,
        RecentWorkTargetState,
    };
    use std::path::Path;

    #[test]
    fn builds_primary_actions_in_contract_order() {
        let registry = seeded_registry();
        let runtime = StartCenterRuntimeInputs {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
            labs_enabled: false,
        };

        let rows = build_action_rows(&registry, runtime);
        assert_eq!(rows.len(), StartCenterPrimaryActionId::ordered().len());
        for (idx, expected) in StartCenterPrimaryActionId::ordered().iter().enumerate() {
            assert_eq!(rows[idx].action_id, *expected);
            assert_eq!(rows[idx].title, expected.label());
            assert_eq!(rows[idx].command_id, expected.command_id());
            assert!(rows[idx].preflight.is_some());
        }
    }

    #[test]
    fn open_workspace_overrides_scope_argument() {
        let registry = seeded_registry();
        let runtime = StartCenterRuntimeInputs {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
            labs_enabled: false,
        };

        let rows = build_action_rows(&registry, runtime);
        let open_folder = rows
            .iter()
            .find(|row| row.action_id == StartCenterPrimaryActionId::OpenFolder)
            .expect("Open folder row must exist");
        let open_workspace = rows
            .iter()
            .find(|row| row.action_id == StartCenterPrimaryActionId::OpenWorkspace)
            .expect("Open workspace row must exist");

        let folder_scope = open_folder
            .argument_provenance_map
            .iter()
            .find(|row| row.argument_name == "workspace_scope_ref")
            .and_then(|row| row.resolved_value_ref.as_deref());
        assert_eq!(
            folder_scope,
            Some("workspace-scope:folder:recent:01"),
            "Open folder should resolve to the folder scope ref"
        );

        let workspace_scope = open_workspace
            .argument_provenance_map
            .iter()
            .find(|row| row.argument_name == "workspace_scope_ref")
            .and_then(|row| row.resolved_value_ref.as_deref());
        assert_eq!(
            workspace_scope,
            Some("workspace-scope:workspace_file:recent:01"),
            "Open workspace should resolve to the workspace-file scope ref"
        );
    }

    #[derive(Debug, Deserialize)]
    struct StartCenterSurfaceFixture {
        presentation_label: String,
        presentation_subtitle: String,
    }

    #[test]
    fn presentation_strings_match_first_run_fixture() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/ux/start_center_rows/start_center_first_run_no_account.json");
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let fixture: StartCenterSurfaceFixture =
            serde_json::from_str(&payload).expect("fixture must parse");

        assert_eq!(fixture.presentation_label, START_CENTER_PRESENTATION_LABEL);
        assert_eq!(
            fixture.presentation_subtitle,
            START_CENTER_PRESENTATION_SUBTITLE
        );
    }

    #[test]
    fn alpha_bundle_gallery_projects_manifest_rows_for_start_center() {
        let rows = build_alpha_bundle_gallery_rows().expect("gallery rows project");
        assert_eq!(rows.len(), 2);

        let ts_web = rows
            .iter()
            .find(|row| row.bundle_id == "launch_bundle:typescript_web_app.seed")
            .expect("typescript bundle row");
        assert_eq!(ts_web.channel, "external_alpha");
        assert_eq!(ts_web.certification_state, "seed_not_certified");
        assert_eq!(
            ts_web.archetype_seed_row_ref,
            "archetype_certification_seed:ts_web_app_or_service"
        );
        assert!(ts_web
            .mirror_availability_label
            .contains("offline_metadata_and_docs_pack"));
        assert!(ts_web
            .available_actions
            .iter()
            .any(|action| action == "apply"));
        assert!(ts_web
            .available_actions
            .iter()
            .any(|action| action == "compare_again_later"));
        assert!(ts_web
            .evidence_packet_ref
            .contains("launch_bundles_and_archetypes.md#typescript"));

        let python = rows
            .iter()
            .find(|row| row.bundle_id == "launch_bundle:python_service_or_data_app.seed")
            .expect("python bundle row");
        assert_eq!(python.channel, "external_alpha");
        assert_eq!(python.certification_state, "seed_not_certified");
        assert_eq!(
            python.archetype_seed_row_ref,
            "archetype_certification_seed:python_service_or_data_app"
        );
    }

    #[test]
    fn alpha_bundle_gallery_plaintext_exposes_required_summary_fields() {
        let text = render_alpha_bundle_gallery_plaintext().expect("gallery plaintext");
        assert!(text.contains("External alpha bundle gallery"));
        assert!(text.contains("launch_bundle:typescript_web_app.seed"));
        assert!(text.contains("launch_bundle:python_service_or_data_app.seed"));
        assert!(text.contains("Aureline project seed signer"));
        assert!(text.contains(">=0.0.0-alpha <0.1.0"));
        assert!(text.contains("seed_not_certified"));
        assert!(text.contains("offline_metadata_and_docs_pack"));
    }

    #[test]
    fn workspace_template_seed_projects_capsule_rows_for_start_center() {
        let rows = build_workspace_template_seed_rows().expect("workspace template rows project");
        assert_eq!(rows.len(), 2);

        let ts_web = rows
            .iter()
            .find(|row| row.template_id == "workspace_template.alpha.ts_web.local_seed")
            .expect("typescript workspace template row");
        assert_eq!(
            ts_web.environment_capsule_ref,
            "capsule.alpha.ts_web.local_node"
        );
        assert_eq!(ts_web.runtime_and_toolchain_scope, "local_only");
        assert_eq!(ts_web.target_class, "local_host");
        assert_eq!(ts_web.raw_value_included_count, 0);
        assert_eq!(ts_web.secret_alias_count, 1);
        assert!(ts_web
            .launch_bundle_refs
            .iter()
            .any(|bundle| bundle == "launch_bundle:typescript_web_app.seed"));
        assert!(ts_web
            .environment_variable_names
            .iter()
            .any(|name| name == "NPM_TOKEN"));

        let python = rows
            .iter()
            .find(|row| row.template_id == "workspace_template.alpha.python.devcontainer_seed")
            .expect("python workspace template row");
        assert_eq!(
            python.environment_capsule_ref,
            "capsule.alpha.python.local_devcontainer"
        );
        assert_eq!(python.target_class, "devcontainer");
        assert_eq!(python.boundary_class, "devcontainer_boundary");
        assert_eq!(python.prebuild_reuse_state, "rejected_drift");
    }

    #[test]
    fn workspace_template_seed_plaintext_exposes_capsule_and_secret_posture() {
        let text = render_workspace_template_seed_plaintext().expect("template plaintext");
        assert!(text.contains("Workspace template seed gallery"));
        assert!(text.contains("workspace_template.alpha.ts_web.local_seed"));
        assert!(text.contains("capsule.alpha.ts_web.local_node"));
        assert!(text.contains("launch_bundle:typescript_web_app.seed"));
        assert!(text.contains("secret_aliases=1"));
        assert!(text.contains("raw_values=0"));
    }

    #[test]
    fn warm_start_descriptor_seed_projects_source_freshness_target_and_resume() {
        let rows = build_warm_start_descriptor_seed_rows().expect("warm-start rows project");
        assert_eq!(rows.len(), 3);

        let ts_web = rows
            .iter()
            .find(|row| row.descriptor_id == "prebuild.alpha.ts_web.local_dependency_cache")
            .expect("typescript warm-start descriptor row");
        assert_eq!(ts_web.source_class, "workspace_template_seed");
        assert_eq!(ts_web.freshness_state, "cached");
        assert_eq!(ts_web.target_class, "local_host");
        assert_eq!(ts_web.warm_start_state, "warm_candidate");
        assert_eq!(ts_web.reuse_state, "candidate");
        assert_eq!(
            ts_web.materializer_claim,
            "metadata_only_no_materializer_claim"
        );
        assert!(!ts_web.raw_secret_values_included);
        assert!(!ts_web.raw_command_lines_included);

        let stale_python = rows
            .iter()
            .find(|row| row.descriptor_id == "prebuild.alpha.python.devcontainer.stale_snapshot")
            .expect("python stale warm-start descriptor row");
        assert_eq!(stale_python.freshness_state, "stale");
        assert_eq!(stale_python.reuse_state, "rejected_drift");
        assert_eq!(
            stale_python.invalidation_reason.as_deref(),
            Some("capsule_drift")
        );
        assert_eq!(stale_python.drift_marker_count, 1);

        let managed_resume = rows
            .iter()
            .find(|row| row.descriptor_id == "prebuild.alpha.managed_workspace.resume_metadata")
            .expect("managed resume descriptor row");
        assert_eq!(managed_resume.target_class, "managed_workspace");
        assert_eq!(managed_resume.warm_start_state, "live_resume_candidate");
        assert_eq!(managed_resume.resume_capability, "resume_requires_reauth");
        assert!(managed_resume.secret_revalidation_required);
        assert!(managed_resume
            .project_entry_review_refs
            .iter()
            .any(|entry| entry == "project_entry:resume_managed_workspace.seed"));
    }

    #[test]
    fn warm_start_descriptor_plaintext_exposes_metadata_only_claims() {
        let text = render_warm_start_descriptor_seed_plaintext().expect("warm-start plaintext");
        assert!(text.contains("Warm-start descriptor seed gallery"));
        assert!(text.contains("prebuild.alpha.ts_web.local_dependency_cache"));
        assert!(text.contains("prebuild.alpha.python.devcontainer.stale_snapshot"));
        assert!(text.contains("stale_warm_candidate/rejected_drift"));
        assert!(text.contains("live_resume_candidate/candidate"));
        assert!(text.contains("metadata_only_no_materializer_claim"));
        assert!(text.contains("resume_requires_reauth"));
    }

    #[test]
    fn recent_work_rows_reuse_failure_taxonomy_and_privacy_preserves_entry() {
        let registry = RecentWorkRegistry {
            record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
            recent_work_registry_schema_version: 1,
            updated_at: "mono:test".to_string(),
            entries: vec![RecentWorkEntryRecord {
                record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
                entry_and_restore_schema_version: 1,
                recent_work_id: "recent:missing".to_string(),
                presentation_label: "payments".to_string(),
                presentation_subtitle: Some("/private/path/payments".to_string()),
                target_kind: TargetKind::LocalRepoRoot,
                target_state: RecentWorkTargetState::MissingTarget,
                portability_class: PortabilityClass::LocalOnly,
                trust_state: TrustState::Trusted,
                restore_availability: RestoreAvailability::LayoutOnly,
                safe_recovery_actions: vec![SafeRecoveryAction::LocateMissingTarget],
                pinned: false,
                last_opened_at: "mono:test".to_string(),
                filesystem_identity_ref: Some("fs:payments".to_string()),
                remote_target_descriptor_ref: None,
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: None,
            }],
        };

        let rows = build_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::Default);
        assert_eq!(rows.rows.len(), 1);
        assert_eq!(
            rows.rows[0].failure_state,
            RecentWorkFailureState::MissingPath
        );
        assert!(rows.rows[0].placeholder_card.is_some());
        assert!(rows.rows[0]
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::OpenWithoutRestore));

        let reduced =
            build_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::HidePaths);
        assert_eq!(
            reduced.rows[0].location_or_target_subtitle.as_deref(),
            Some("Repository")
        );
        assert!(reduced.rows[0].privacy_redaction_applied);

        let hidden =
            build_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::HideRecentWork);
        assert!(hidden.metadata_hidden);
        assert!(hidden.rows.is_empty());
        assert!(hidden.local_open_still_available);
        assert!(hidden.workspace_open_still_available);
        assert!(hidden.restore_local_state_still_available);
        assert!(hidden.clear_recent_work_available);
    }
}
