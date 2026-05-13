//! Start Center quick-action surface projections.
//!
//! The Start Center is the local-first entry surface rendered when the shell
//! has no active workspace. It advertises the canonical entry verbs (`Open`,
//! `Clone`, `Import`, and `Restore`) without requiring account sign-in, and it
//! resolves preflight/enablement status through the seeded command registry so
//! every surface shares the same truth.

use std::fmt;

use aureline_commands::invocation::ArgumentProvenanceEntry;
use aureline_commands::{CommandEnablementContext, CommandRegistry, PreflightDecision};
use serde::Deserialize;

pub mod admission_review;

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
}
