//! Learning-tour alpha manifest validation and shell projection.
//!
//! This module is the first shell-owned consumer for the checked-in
//! learning-tour alpha manifest. It keeps guided tour packages, exercise rails,
//! contextual teaching rows, support export rows, and user-owned progress
//! snapshots tied to the command registry and docs/help anchors.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use aureline_commands::{registry::seeded_registry, CommandRegistry};
use serde::{Deserialize, Serialize};

/// Stable path to the checked-in learning-tour alpha manifest.
pub const CURRENT_LEARNING_TOUR_ALPHA_PATH: &str = "artifacts/docs/learning_tour_alpha.yaml";

const CURRENT_LEARNING_TOUR_ALPHA_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/docs/learning_tour_alpha.yaml"
));

/// Current schema version for [`LearningTourAlphaManifest`].
pub const LEARNING_TOUR_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Current schema version for [`LearningProgressAlphaSnapshot`].
pub const LEARNING_PROGRESS_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Current record kind for [`LearningTourAlphaManifest`].
pub const LEARNING_TOUR_ALPHA_RECORD_KIND: &str = "learning_tour_alpha_manifest_record";

/// Current record kind for [`LearningProgressAlphaSnapshot`].
pub const LEARNING_PROGRESS_ALPHA_RECORD_KIND: &str = "learning_progress_alpha_snapshot_record";

/// Record kind for the shell-side contextual teaching projection.
pub const LEARNING_TOUR_ALPHA_SURFACE_RECORD_KIND: &str =
    "learning_tour_alpha_surface_projection_record";

/// Versioned learning-tour package manifest consumed by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTourAlphaManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version for the manifest shape.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Timestamp or deterministic fixture token for the manifest.
    pub generated_at: String,
    /// Schema, docs, command, and UX contracts this manifest consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// Runtime or support consumers that read this manifest directly.
    pub runtime_consumer_refs: Vec<String>,
    /// Versioned tour package descriptors.
    pub packages: Vec<LearningPackage>,
    /// Multi-step tours in the manifest.
    pub tours: Vec<LearningTour>,
    /// Step rows that tours and exercise rails reference.
    pub steps: Vec<LearningStep>,
    /// Guided exercise rails exposed by the first runtime surface.
    pub exercise_rails: Vec<ExerciseRail>,
    /// Contextual teaching rows projected into the shell.
    pub contextual_teaching_surfaces: Vec<ContextualTeachingSurface>,
    /// User-owned progress snapshots packaged with the alpha proof.
    #[serde(default)]
    pub progress_snapshots: Vec<LearningProgressAlphaSnapshot>,
    /// Support/export reconstruction projection.
    pub support_export: LearningSupportExport,
    /// Protected proof fixtures backing the manifest.
    pub protected_proofs: Vec<LearningTourProof>,
}

impl LearningTourAlphaManifest {
    /// Returns the package descriptor for `package_id`.
    pub fn package(&self, package_id: &str) -> Option<&LearningPackage> {
        self.packages
            .iter()
            .find(|package| package.package_id == package_id)
    }

    /// Returns the tour descriptor for `tour_id`.
    pub fn tour(&self, tour_id: &str) -> Option<&LearningTour> {
        self.tours.iter().find(|tour| tour.tour_id == tour_id)
    }

    /// Returns the step descriptor for `step_id`.
    pub fn step(&self, step_id: &str) -> Option<&LearningStep> {
        self.steps.iter().find(|step| step.step_id == step_id)
    }

    /// Returns the progress snapshot for `snapshot_id`.
    pub fn progress_snapshot(&self, snapshot_id: &str) -> Option<&LearningProgressAlphaSnapshot> {
        self.progress_snapshots
            .iter()
            .find(|snapshot| snapshot.snapshot_id == snapshot_id)
    }

    /// Returns the contextual surface rows that can render now.
    pub fn contextual_teaching_rows(&self) -> &[ContextualTeachingSurface] {
        &self.contextual_teaching_surfaces
    }

    /// Returns support/export-safe rows with raw bodies excluded.
    pub fn support_export_rows(&self) -> &[LearningSupportExportRow] {
        &self.support_export.rows
    }

    /// Renders the contextual teaching projection as support-safe plaintext.
    pub fn render_contextual_teaching_plaintext(&self) -> String {
        let mut lines = vec![
            "Learning tour alpha".to_owned(),
            format!("schema_version: {}", self.schema_version),
            format!("manifest_id: {}", self.manifest_id),
            "contextual_teaching_surfaces:".to_owned(),
        ];
        for surface in &self.contextual_teaching_surfaces {
            lines.push(format!(
                "- {} step={} command={} posture={} reopen={}",
                surface.surface_id,
                surface.step_ref,
                surface.command_id,
                surface.package_degradation_class.as_str(),
                surface.exact_reopen_ref
            ));
        }
        lines.push("exercise_rails:".to_owned());
        for rail in &self.exercise_rails {
            lines.push(format!(
                "- {} current_step={} guardrail={} reset={}",
                rail.rail_id,
                rail.current_step_ref,
                rail.mutation_guardrail_class.as_str(),
                rail.reset_action
                    .command_id
                    .as_deref()
                    .unwrap_or("local-state-reset")
            ));
        }
        lines.push("support_export_rows:".to_owned());
        for row in &self.support_export.rows {
            lines.push(format!(
                "- {} raw_body_exported={} progress={}",
                row.row_id, row.raw_body_exported, row.progress_snapshot_ref
            ));
        }
        lines.push(String::new());
        lines.join("\n")
    }

    /// Validates package, step, command, rail, progress, and support/export linkage.
    pub fn validate_against_registry(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), Vec<LearningTourValidationFinding>> {
        let mut validator = LearningTourValidator::new(self, registry);
        validator.validate();
        validator.finish()
    }

    /// Validates that proof fixture refs resolve under `repo_root`.
    pub fn validate_fixture_refs(
        &self,
        repo_root: impl AsRef<Path>,
    ) -> Result<(), Vec<LearningTourValidationFinding>> {
        let repo_root = repo_root.as_ref();
        let mut findings = Vec::new();
        for proof in &self.protected_proofs {
            if !repo_root.join(&proof.fixture_ref).exists() {
                findings.push(LearningTourValidationFinding::new(
                    proof.proof_id.clone(),
                    "learning_tour.proof_fixture_missing",
                    "protected proof fixture does not exist",
                ));
            }
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Versioned package that owns tour and exercise steps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningPackage {
    /// Stable package id.
    pub package_id: String,
    /// Human/release version ref for the package.
    pub package_version: String,
    /// Immutable package revision ref.
    pub package_revision_ref: String,
    /// Source class for the package content.
    pub source_class: String,
    /// Install state visible to learning surfaces.
    pub install_state: PackageInstallState,
    /// Availability state after cache, mirror, or install checks.
    pub availability_state: PackageAvailabilityState,
    /// Docs pack that backs package citations.
    pub docs_pack_ref: String,
    /// Docs pack revision that backs package citations.
    pub docs_pack_revision_ref: String,
    /// Source-language locale for package content.
    pub source_locale: String,
    /// Locales available in this package.
    pub available_locales: Vec<String>,
    /// Citation refs preserved even when the package degrades.
    pub citation_refs: Vec<String>,
    /// Exact reopen ref for the package.
    pub exact_reopen_ref: String,
    /// Degradation class rendered by consuming surfaces.
    pub degradation_copy_class: PackageDegradationClass,
}

/// Multi-step tour descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTour {
    /// Stable tour id.
    pub tour_id: String,
    /// Package that owns this tour.
    pub package_ref: String,
    /// Reviewable tour title.
    pub title: String,
    /// Audience class for the tour.
    pub audience_class: String,
    /// Learning profile ref that owns progress.
    pub profile_ref: String,
    /// Ordered step refs.
    pub ordered_step_refs: Vec<String>,
    /// Docs pack refs cited by the tour.
    pub docs_pack_refs: Vec<String>,
    /// Citation refs backing the tour.
    pub citation_refs: Vec<String>,
    /// Package availability observed by the tour.
    pub package_availability_state: PackageAvailabilityState,
    /// Exact resume ref for reopen behavior.
    pub exact_resume_ref: String,
}

/// One guided learning step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningStep {
    /// Stable step id.
    pub step_id: String,
    /// Tour that owns this step.
    pub tour_ref: String,
    /// Package that owns this step.
    pub package_ref: String,
    /// Zero-based position in the tour.
    pub position_index: u32,
    /// Surface kind for this step.
    pub step_kind: String,
    /// Reviewable step title.
    pub title: String,
    /// Stable target ref for the step.
    pub target_ref: LearningTargetRef,
    /// Scope ref before the step runs.
    pub current_scope_ref: String,
    /// Whether the step widens scope.
    pub scope_widening_class: ScopeWideningClass,
    /// Citation refs supporting the step.
    pub citation_refs: Vec<String>,
    /// Success criteria refs shown by the rail.
    pub success_criteria: Vec<String>,
    /// Hint action/content ref.
    pub hint_ref: String,
    /// Reveal action/content ref.
    pub reveal_ref: String,
    /// Skip action exposed by the step.
    pub skip_action: LearningAction,
    /// Reset action exposed by the step.
    pub reset_action: LearningAction,
    /// Sandbox posture for this step.
    pub sandbox_posture: SandboxPosture,
    /// Reversibility posture for this step.
    pub reversibility_posture: String,
    /// Whether explanation is shown before action.
    pub explain_before_action: bool,
    /// Primary action for the step.
    pub primary_action: LearningAction,
    /// Degradation class rendered by the step.
    pub degradation_copy_class: PackageDegradationClass,
    /// Exact reopen ref for this step.
    pub exact_reopen_ref: String,
    /// Whether this step may render as an active step.
    pub active: bool,
}

/// Stable target for a learning step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTargetRef {
    /// Target kind such as command, docs node, symbol, graph node, or help anchor.
    pub target_kind: String,
    /// Command id when `target_kind` is `command_id`.
    #[serde(default)]
    pub command_id: Option<String>,
    /// Docs node id when `target_kind` is `docs_node`.
    #[serde(default)]
    pub docs_node_id: Option<String>,
    /// Source file ref when `target_kind` is `source_file`.
    #[serde(default)]
    pub source_file_ref: Option<String>,
    /// Symbol ref when `target_kind` is `symbol`.
    #[serde(default)]
    pub symbol_ref: Option<String>,
    /// Graph node ref when `target_kind` is `graph_node`.
    #[serde(default)]
    pub graph_node_ref: Option<String>,
    /// Help anchor id when `target_kind` is `help_anchor`.
    #[serde(default)]
    pub help_anchor_id: Option<String>,
}

impl LearningTargetRef {
    /// Returns true when the target is a stable non-coordinate anchor.
    pub fn is_stable_anchor(&self) -> bool {
        match self.target_kind.as_str() {
            "command_id" => self.command_id.as_deref().is_some_and(is_command_id),
            "docs_node" => self.docs_node_id.as_deref().is_some_and(non_empty),
            "source_file" => self.source_file_ref.as_deref().is_some_and(non_empty),
            "symbol" => self.symbol_ref.as_deref().is_some_and(non_empty),
            "graph_node" => self.graph_node_ref.as_deref().is_some_and(non_empty),
            "help_anchor" => self.help_anchor_id.as_deref().is_some_and(non_empty),
            _ => false,
        }
    }
}

/// Action exposed by a step or rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningAction {
    /// Stable action id.
    pub action_id: String,
    /// Reviewable action label ref.
    pub action_label: String,
    /// Safety class for the action.
    pub action_safety_class: ActionSafetyClass,
    /// Command id when the action invokes the command graph.
    #[serde(default)]
    pub command_id: Option<String>,
    /// Preview sheet ref for preview-before-write actions.
    #[serde(default)]
    pub preview_sheet_ref: Option<String>,
    /// Approval path ref for mutation-capable actions.
    #[serde(default)]
    pub approval_path_ref: Option<String>,
    /// Evidence packet rule ref for mutation-capable actions.
    #[serde(default)]
    pub evidence_packet_rule_ref: Option<String>,
    /// Whether the action is separate from explanatory copy.
    pub separate_from_explanation: bool,
}

/// Guided exercise rail consumed by the contextual teaching surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseRail {
    /// Stable rail id.
    pub rail_id: String,
    /// Tour that owns this rail.
    pub tour_ref: String,
    /// Current step ref.
    pub current_step_ref: String,
    /// Ordered step refs displayed by the rail.
    pub ordered_step_refs: Vec<String>,
    /// Success criteria refs displayed by the rail.
    pub success_criteria_refs: Vec<String>,
    /// Hint action exposed by the rail.
    pub hint_action: LearningAction,
    /// Reveal action exposed by the rail.
    pub reveal_action: LearningAction,
    /// Skip action exposed by the rail.
    pub skip_action: LearningAction,
    /// Reset action exposed by the rail.
    pub reset_action: LearningAction,
    /// Sandbox posture exposed by the rail.
    pub sandbox_posture: SandboxPosture,
    /// Reversibility posture exposed by the rail.
    pub reversibility_posture: String,
    /// Whether explanation must precede action.
    pub explanation_before_action_required: bool,
    /// Guardrail used for mutation-capable work.
    pub mutation_guardrail_class: MutationGuardrailClass,
    /// Whether the exercise is reversible or sandboxed.
    pub reversible_or_sandboxed: bool,
}

/// Contextual teaching row projected into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualTeachingSurface {
    /// Stable surface id.
    pub surface_id: String,
    /// Step ref this surface renders.
    pub step_ref: String,
    /// Rendered learning surface kind.
    pub render_surface_kind: String,
    /// Command id that backs the action.
    pub command_id: String,
    /// Docs/help anchor for deeper context.
    pub docs_anchor_ref: String,
    /// Citation refs backing the surface.
    pub citation_refs: Vec<String>,
    /// Current step ref shown by the rail.
    pub current_step_ref: String,
    /// Package install state visible to the surface.
    pub package_install_state: PackageInstallState,
    /// Package degradation class visible to the surface.
    pub package_degradation_class: PackageDegradationClass,
    /// Exact reopen ref for the contextual surface.
    pub exact_reopen_ref: String,
    /// Message ref for explanatory copy.
    pub explanation_copy_ref: String,
    /// Message ref for action copy.
    pub action_copy_ref: String,
    /// Whether explanation and action are separate.
    pub explain_and_do_separate: bool,
    /// Whether mutations use the ordinary command registry.
    pub mutation_uses_command_registry: bool,
    /// Support export row for this surface.
    pub support_export_row_ref: String,
}

/// User-owned progress snapshot for learning mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningProgressAlphaSnapshot {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version for this snapshot.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Timestamp or fixture token.
    pub generated_at: String,
    /// Tunable learning-mode profile state.
    pub profile_state: LearningProfileState,
    /// Per-step progress entries.
    pub progress_entries: Vec<LearningProgressEntry>,
    /// Dismissal state entries.
    #[serde(default)]
    pub dismissal_entries: Vec<LearningDismissalEntry>,
    /// Bookmark state entries.
    #[serde(default)]
    pub bookmark_entries: Vec<LearningBookmarkEntry>,
    /// Export posture for this snapshot.
    pub export_posture: LearningProgressExportPosture,
    /// Support export projection for this snapshot.
    pub support_export_projection: LearningProgressSupportProjection,
}

/// Tunable learning-mode profile state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningProfileState {
    /// Learning profile ref.
    pub profile_ref: String,
    /// Profile scope ref.
    pub profile_scope: String,
    /// Tip intensity class.
    pub tip_intensity_class: String,
    /// Jargon level class.
    pub jargon_level_class: String,
    /// Educational AI posture.
    pub ai_explanation_posture: String,
    /// Mutation guardrail class.
    pub mutation_guardrail_class: MutationGuardrailClass,
    /// Data ownership class.
    pub data_ownership_class: String,
    /// Whether the profile can change trust boundaries.
    pub trust_boundary_change_allowed: bool,
    /// Whether bookmarks are enabled.
    pub bookmarks_enabled: bool,
    /// Whether dismissals are enabled.
    pub dismissals_enabled: bool,
}

/// Per-step user progress entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningProgressEntry {
    /// Stable progress entry id.
    pub entry_id: String,
    /// Package ref for the entry.
    pub package_ref: String,
    /// Tour ref for the entry.
    pub tour_ref: String,
    /// Step ref for the entry.
    pub step_ref: String,
    /// User-visible progress state.
    pub progress_state_class: ProgressStateClass,
    /// Resume ref for reopen behavior.
    pub resume_ref: String,
    /// Local/sync posture for the entry.
    pub sync_posture_class: SyncPostureClass,
    /// Export class for the entry.
    pub export_class: String,
    /// Whether local/sync posture is explicit.
    pub local_only_or_sync_posture_explicit: bool,
    /// Whether repo packs can read this progress by default.
    pub repo_pack_read_default: bool,
    /// Whether classroom artifacts can read this progress by default.
    pub classroom_read_default: bool,
    /// Whether telemetry-style readers can read this progress by default.
    pub telemetry_read_default: bool,
    /// Exact reopen ref for this entry.
    pub exact_reopen_ref: String,
    /// Whether command and help anchors are preserved.
    pub preserves_command_help_anchor: bool,
}

/// Dismissal state retained by the progress snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningDismissalEntry {
    /// Stable dismissal id.
    pub dismissal_id: String,
    /// Step ref this dismissal applies to.
    pub step_ref: String,
    /// Local/sync posture for the dismissal.
    pub sync_posture_class: SyncPostureClass,
    /// Whether the dismissal is reversible from the learning digest.
    pub reversible_from_learning_digest: bool,
}

/// Bookmark state retained by the progress snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningBookmarkEntry {
    /// Stable bookmark id.
    pub bookmark_id: String,
    /// Step ref this bookmark opens.
    pub step_ref: String,
    /// Local/sync posture for the bookmark.
    pub sync_posture_class: SyncPostureClass,
    /// Exact reopen ref for this bookmark.
    pub exact_reopen_ref: String,
}

/// Export posture for a learning progress snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningProgressExportPosture {
    /// Whether metadata can export with a portable profile.
    pub portable_profile_exportable: bool,
    /// Whether metadata can export in a redacted support bundle.
    pub support_bundle_exportable_redacted: bool,
    /// Whether raw step body text is exported.
    pub raw_step_body_exported: bool,
    /// Whether raw package body text is exported.
    pub raw_pack_body_exported: bool,
    /// Whether the user can delete or reset this state.
    pub user_can_delete_or_reset: bool,
}

/// Support export projection for learning progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningProgressSupportProjection {
    /// Stable support projection id.
    pub projection_id: String,
    /// Support export row refs.
    pub row_refs: Vec<String>,
    /// Whether raw profile body text is exported.
    pub raw_profile_body_exported: bool,
}

/// Support/export projection for the learning-tour manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSupportExport {
    /// Stable export id.
    pub export_id: String,
    /// Whether raw package body text is exported.
    pub raw_pack_body_exported: bool,
    /// Support/export-safe rows.
    pub rows: Vec<LearningSupportExportRow>,
}

/// One support/export-safe learning-tour row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSupportExportRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface ref being exported.
    pub surface_ref: String,
    /// Package ref being exported.
    pub package_ref: String,
    /// Step ref being exported.
    pub step_ref: String,
    /// Package revision ref being exported.
    pub package_revision_ref: String,
    /// Citation refs preserved in export.
    pub citation_refs: Vec<String>,
    /// Exact reopen ref preserved in export.
    pub exact_reopen_ref: String,
    /// Progress snapshot ref preserved in export.
    pub progress_snapshot_ref: String,
    /// Whether raw body text is exported.
    pub raw_body_exported: bool,
}

/// Protected proof fixture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTourProof {
    /// Stable proof id.
    pub proof_id: String,
    /// Fixture path relative to the repository root.
    pub fixture_ref: String,
    /// Package refs exercised by the proof.
    pub exercised_package_refs: Vec<String>,
    /// Step refs exercised by the proof.
    pub exercised_step_refs: Vec<String>,
    /// Acceptance states exercised by the proof.
    pub exercised_states: Vec<String>,
}

/// Shell-side contextual teaching projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTourAlphaSurfaceProjection {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version from the manifest.
    pub schema_version: u32,
    /// Manifest ref that produced this projection.
    pub manifest_ref: String,
    /// Projection timestamp copied from the manifest.
    pub generated_at: String,
    /// Contextual teaching rows that render in the shell.
    pub contextual_teaching_surfaces: Vec<ContextualTeachingSurface>,
    /// Exercise rails available to the contextual teaching rows.
    pub exercise_rails: Vec<ExerciseRail>,
    /// Progress snapshots retained as user-owned state.
    pub progress_snapshots: Vec<LearningProgressAlphaSnapshot>,
    /// Support/export-safe rows.
    pub support_export_rows: Vec<LearningSupportExportRow>,
}

impl LearningTourAlphaSurfaceProjection {
    /// Renders the projection as support-safe plaintext.
    pub fn render_plaintext(&self) -> String {
        let mut lines = vec![
            "Learning tour alpha surface".to_owned(),
            format!("manifest_ref: {}", self.manifest_ref),
            "contextual_teaching_surfaces:".to_owned(),
        ];
        for surface in &self.contextual_teaching_surfaces {
            lines.push(format!(
                "- {} command={} step={} separate={}",
                surface.surface_id,
                surface.command_id,
                surface.step_ref,
                surface.explain_and_do_separate
            ));
        }
        lines.push("progress_snapshots:".to_owned());
        for snapshot in &self.progress_snapshots {
            lines.push(format!(
                "- {} entries={}",
                snapshot.snapshot_id,
                snapshot.progress_entries.len()
            ));
        }
        lines.push(String::new());
        lines.join("\n")
    }
}

/// Package install state visible to learning surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageInstallState {
    /// Package ships with the local product build.
    LocalOnly,
    /// Package is available from a current cache.
    CachedCurrent,
    /// Package is available only from a verified mirror.
    MirrorOnly,
    /// Package is referenced but not installed.
    NotInstalled,
}

impl PackageInstallState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::CachedCurrent => "cached_current",
            Self::MirrorOnly => "mirror_only",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Package availability state after install/cache/mirror checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageAvailabilityState {
    /// Package is installed and available.
    Installed,
    /// Package is available from a current cache.
    CachedAvailable,
    /// Package is available from a mirror.
    MirrorAvailable,
    /// Package is unavailable because it is not installed.
    UnavailableNotInstalled,
}

impl PackageAvailabilityState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::CachedAvailable => "cached_available",
            Self::MirrorAvailable => "mirror_available",
            Self::UnavailableNotInstalled => "unavailable_not_installed",
        }
    }
}

/// Package degradation class rendered by learning surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageDegradationClass {
    /// No degradation is present.
    NoDegradation,
    /// Local-only posture is disclosed.
    LocalOnlyDisclosed,
    /// Cached posture is disclosed.
    CachedDisclosed,
    /// Mirror-only posture is disclosed.
    MirrorOnlyDisclosed,
    /// Not-installed package renders a placeholder.
    NotInstalledPlaceholder,
}

impl PackageDegradationClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDegradation => "no_degradation",
            Self::LocalOnlyDisclosed => "local_only_disclosed",
            Self::CachedDisclosed => "cached_disclosed",
            Self::MirrorOnlyDisclosed => "mirror_only_disclosed",
            Self::NotInstalledPlaceholder => "not_installed_placeholder",
        }
    }
}

/// Whether a step widens user or workspace scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeWideningClass {
    /// Step does not widen scope.
    NoScopeWidening,
    /// Step widens review scope and discloses that before action.
    DisclosedReviewScopeWidening,
    /// Step is blocked until review happens.
    BlockedUntilReview,
}

impl ScopeWideningClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoScopeWidening => "no_scope_widening",
            Self::DisclosedReviewScopeWidening => "disclosed_review_scope_widening",
            Self::BlockedUntilReview => "blocked_until_review",
        }
    }
}

/// Safety class for a learning action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionSafetyClass {
    /// Action is read-only.
    ReadOnly,
    /// Action prepares a preview before any write.
    PreviewBeforeWrite,
    /// Action can mutate only after approval.
    MutationRequiresApproval,
    /// Action is blocked because a package is unavailable.
    BlockedNotInstalled,
}

impl ActionSafetyClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PreviewBeforeWrite => "preview_before_write",
            Self::MutationRequiresApproval => "mutation_requires_approval",
            Self::BlockedNotInstalled => "blocked_not_installed",
        }
    }

    /// Returns true when preview, approval, or evidence metadata is required.
    pub const fn requires_review_path(self) -> bool {
        matches!(
            self,
            Self::PreviewBeforeWrite | Self::MutationRequiresApproval
        )
    }
}

/// Sandbox posture for an exercise step or rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxPosture {
    /// Read-only walkthrough.
    ReadOnlyWalkthrough,
    /// Workspace preview is reversible.
    ReversibleWorkspacePreview,
    /// Changes stay in an isolated sandbox.
    IsolatedSandbox,
    /// Exercise is blocked until trust is granted.
    BlockedUntilTrust,
}

impl SandboxPosture {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyWalkthrough => "read_only_walkthrough",
            Self::ReversibleWorkspacePreview => "reversible_workspace_preview",
            Self::IsolatedSandbox => "isolated_sandbox",
            Self::BlockedUntilTrust => "blocked_until_trust",
        }
    }
}

/// Mutation guardrail used by a profile, step, or rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationGuardrailClass {
    /// Only explanation is allowed.
    ExplainOnly,
    /// Preview is required before action.
    PreviewRequired,
    /// Approval is required before action.
    ApprovalRequired,
    /// Action is blocked until trust is granted.
    BlockedUntilTrust,
}

impl MutationGuardrailClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnly => "explain_only",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
            Self::BlockedUntilTrust => "blocked_until_trust",
        }
    }
}

/// User-visible progress state for a learning step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressStateClass {
    /// Step has not started.
    NotStarted,
    /// Step is in progress.
    InProgress,
    /// Step completed.
    Completed,
    /// Step was dismissed.
    Dismissed,
    /// Step is the current resume target.
    Resumed,
    /// Step was deferred for later.
    Deferred,
    /// Step is blocked by package availability.
    BlockedByPackageState,
}

impl ProgressStateClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Dismissed => "dismissed",
            Self::Resumed => "resumed",
            Self::Deferred => "deferred",
            Self::BlockedByPackageState => "blocked_by_package_state",
        }
    }
}

/// Local/sync posture for a user-owned progress row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncPostureClass {
    /// State remains local only.
    LocalOnly,
    /// State can travel with a portable profile.
    PortableProfile,
    /// Sync is opt-in.
    SyncOptIn,
    /// Sync is blocked by policy.
    SyncBlockedByPolicy,
}

impl SyncPostureClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::PortableProfile => "portable_profile",
            Self::SyncOptIn => "sync_opt_in",
            Self::SyncBlockedByPolicy => "sync_blocked_by_policy",
        }
    }
}

/// Validation finding for learning-tour alpha manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTourValidationFinding {
    /// Row or object that failed validation.
    pub row_ref: String,
    /// Stable validation check id.
    pub check_id: String,
    /// Reviewable validation message.
    pub message: String,
}

impl LearningTourValidationFinding {
    fn new(
        row_ref: impl Into<String>,
        check_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            row_ref: row_ref.into(),
            check_id: check_id.into(),
            message: message.into(),
        }
    }
}

/// Error returned when the shell cannot build the alpha projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearningTourAlphaProjectionError {
    /// The checked-in manifest failed to parse.
    ManifestParse(String),
    /// The checked-in manifest failed validation.
    Validation(Vec<LearningTourValidationFinding>),
}

impl fmt::Display for LearningTourAlphaProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManifestParse(err) => write!(f, "failed to parse learning tour manifest: {err}"),
            Self::Validation(findings) => {
                write!(f, "learning tour manifest validation failed: ")?;
                for (idx, finding) in findings.iter().enumerate() {
                    if idx > 0 {
                        write!(f, "; ")?;
                    }
                    write!(
                        f,
                        "{} {} {}",
                        finding.row_ref, finding.check_id, finding.message
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for LearningTourAlphaProjectionError {}

/// Parses the checked-in learning-tour alpha manifest.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in manifest no longer matches
/// the shell-side record model.
pub fn current_learning_tour_alpha_manifest() -> Result<LearningTourAlphaManifest, serde_yaml::Error>
{
    serde_yaml::from_str(CURRENT_LEARNING_TOUR_ALPHA_YAML)
}

/// Builds the first contextual teaching projection from the checked-in manifest.
///
/// # Errors
///
/// Returns an error when the checked-in manifest cannot parse or when
/// validation against the command registry fails.
pub fn build_learning_tour_alpha_surface_projection(
) -> Result<LearningTourAlphaSurfaceProjection, LearningTourAlphaProjectionError> {
    let manifest = current_learning_tour_alpha_manifest()
        .map_err(|err| LearningTourAlphaProjectionError::ManifestParse(err.to_string()))?;
    manifest
        .validate_against_registry(seeded_registry())
        .map_err(LearningTourAlphaProjectionError::Validation)?;
    Ok(LearningTourAlphaSurfaceProjection {
        record_kind: LEARNING_TOUR_ALPHA_SURFACE_RECORD_KIND.to_owned(),
        schema_version: manifest.schema_version,
        manifest_ref: manifest.manifest_id,
        generated_at: manifest.generated_at,
        contextual_teaching_surfaces: manifest.contextual_teaching_surfaces,
        exercise_rails: manifest.exercise_rails,
        progress_snapshots: manifest.progress_snapshots,
        support_export_rows: manifest.support_export.rows,
    })
}

/// Serializes the learning-tour alpha projection to a JSON export file.
///
/// # Errors
///
/// Returns an error when the projection fails to build, parent directory
/// creation fails, JSON serialization fails, or file writing fails.
pub fn write_learning_tour_alpha_export(
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let record = build_learning_tour_alpha_surface_projection()?;
    let payload = serde_json::to_string_pretty(&record)?;
    std::fs::write(path, format!("{payload}\n"))?;
    Ok(())
}

struct LearningTourValidator<'a> {
    manifest: &'a LearningTourAlphaManifest,
    registry: &'a CommandRegistry,
    findings: Vec<LearningTourValidationFinding>,
}

impl<'a> LearningTourValidator<'a> {
    fn new(manifest: &'a LearningTourAlphaManifest, registry: &'a CommandRegistry) -> Self {
        Self {
            manifest,
            registry,
            findings: Vec::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        self.validate_packages();
        self.validate_tours();
        self.validate_steps();
        self.validate_rails();
        self.validate_contextual_surfaces();
        self.validate_progress_snapshots();
        self.validate_support_export();
        self.validate_proofs();
    }

    fn finish(self) -> Result<(), Vec<LearningTourValidationFinding>> {
        if self.findings.is_empty() {
            Ok(())
        } else {
            Err(self.findings)
        }
    }

    fn validate_header(&mut self) {
        if self.manifest.record_kind != LEARNING_TOUR_ALPHA_RECORD_KIND {
            self.push(
                &self.manifest.manifest_id,
                "learning_tour.record_kind",
                "manifest record_kind is unsupported",
            );
        }
        if self.manifest.schema_version != LEARNING_TOUR_ALPHA_SCHEMA_VERSION {
            self.push(
                &self.manifest.manifest_id,
                "learning_tour.schema_version",
                "manifest schema version is unsupported",
            );
        }
        if self.manifest.runtime_consumer_refs.is_empty() {
            self.push(
                &self.manifest.manifest_id,
                "learning_tour.consumers_missing",
                "manifest must declare at least one runtime consumer",
            );
        }
    }

    fn validate_packages(&mut self) {
        let mut ids = BTreeSet::new();
        let mut states = BTreeSet::new();
        for package in self.manifest.packages.clone() {
            if !ids.insert(package.package_id.clone()) {
                self.push(
                    &package.package_id,
                    "learning_tour.package_duplicate",
                    "duplicate package id",
                );
            }
            states.insert(package.install_state);
            if package.citation_refs.is_empty() {
                self.push(
                    &package.package_id,
                    "learning_tour.package_citation_missing",
                    "package must preserve citation refs",
                );
            }
            if package.exact_reopen_ref.trim().is_empty() {
                self.push(
                    &package.package_id,
                    "learning_tour.package_reopen_missing",
                    "package must preserve exact reopen behavior",
                );
            }
            match package.install_state {
                PackageInstallState::LocalOnly => {
                    self.expect_package_pair(
                        &package,
                        PackageAvailabilityState::Installed,
                        PackageDegradationClass::LocalOnlyDisclosed,
                    );
                }
                PackageInstallState::CachedCurrent => {
                    self.expect_package_pair(
                        &package,
                        PackageAvailabilityState::CachedAvailable,
                        PackageDegradationClass::CachedDisclosed,
                    );
                }
                PackageInstallState::MirrorOnly => {
                    self.expect_package_pair(
                        &package,
                        PackageAvailabilityState::MirrorAvailable,
                        PackageDegradationClass::MirrorOnlyDisclosed,
                    );
                }
                PackageInstallState::NotInstalled => {
                    self.expect_package_pair(
                        &package,
                        PackageAvailabilityState::UnavailableNotInstalled,
                        PackageDegradationClass::NotInstalledPlaceholder,
                    );
                }
            }
        }

        for required in [
            PackageInstallState::LocalOnly,
            PackageInstallState::CachedCurrent,
            PackageInstallState::MirrorOnly,
            PackageInstallState::NotInstalled,
        ] {
            if !states.contains(&required) {
                self.push(
                    &self.manifest.manifest_id,
                    "learning_tour.package_state_coverage_missing",
                    format!("manifest must exercise package state {}", required.as_str()),
                );
            }
        }
    }

    fn expect_package_pair(
        &mut self,
        package: &LearningPackage,
        availability: PackageAvailabilityState,
        degradation: PackageDegradationClass,
    ) {
        if package.availability_state != availability {
            self.push(
                &package.package_id,
                "learning_tour.package_availability_mismatch",
                format!(
                    "install state {} must use availability {}",
                    package.install_state.as_str(),
                    availability.as_str()
                ),
            );
        }
        if package.degradation_copy_class != degradation {
            self.push(
                &package.package_id,
                "learning_tour.package_degradation_mismatch",
                format!(
                    "install state {} must use degradation {}",
                    package.install_state.as_str(),
                    degradation.as_str()
                ),
            );
        }
    }

    fn validate_tours(&mut self) {
        let packages = self.package_ids();
        let steps = self.step_ids();
        let mut ids = BTreeSet::new();
        for tour in self.manifest.tours.clone() {
            if !ids.insert(tour.tour_id.clone()) {
                self.push(
                    &tour.tour_id,
                    "learning_tour.tour_duplicate",
                    "duplicate tour id",
                );
            }
            if !packages.contains(tour.package_ref.as_str()) {
                self.push(
                    &tour.tour_id,
                    "learning_tour.tour_unknown_package",
                    "tour references unknown package",
                );
            }
            if tour.ordered_step_refs.is_empty() {
                self.push(
                    &tour.tour_id,
                    "learning_tour.tour_steps_missing",
                    "tour must include ordered step refs",
                );
            }
            for step_ref in &tour.ordered_step_refs {
                if !steps.contains(step_ref.as_str()) {
                    self.push(
                        &tour.tour_id,
                        "learning_tour.tour_unknown_step",
                        format!("tour references unknown step {step_ref}"),
                    );
                }
            }
            if tour.citation_refs.is_empty() || tour.docs_pack_refs.is_empty() {
                self.push(
                    &tour.tour_id,
                    "learning_tour.tour_citation_missing",
                    "tour must preserve docs pack and citation refs",
                );
            }
        }
    }

    fn validate_steps(&mut self) {
        let tours = self.tour_ids();
        let packages = self.package_ids();
        let mut ids = BTreeSet::new();
        for step in self.manifest.steps.clone() {
            if !ids.insert(step.step_id.clone()) {
                self.push(
                    &step.step_id,
                    "learning_tour.step_duplicate",
                    "duplicate step id",
                );
            }
            if !tours.contains(step.tour_ref.as_str()) {
                self.push(
                    &step.step_id,
                    "learning_tour.step_unknown_tour",
                    "step references unknown tour",
                );
            }
            if !packages.contains(step.package_ref.as_str()) {
                self.push(
                    &step.step_id,
                    "learning_tour.step_unknown_package",
                    "step references unknown package",
                );
            }
            if !step.target_ref.is_stable_anchor() {
                self.push(
                    &step.step_id,
                    "learning_tour.step_target_not_stable",
                    "step must target a stable command, docs, file, symbol, graph, or help anchor",
                );
            }
            if let Some(command_id) = &step.target_ref.command_id {
                self.validate_command(
                    command_id,
                    &step.step_id,
                    "learning_tour.step_unknown_command",
                );
            }
            if step.citation_refs.is_empty() {
                self.push(
                    &step.step_id,
                    "learning_tour.step_citation_missing",
                    "step must preserve citation refs",
                );
            }
            if step.success_criteria.is_empty() {
                self.push(
                    &step.step_id,
                    "learning_tour.step_success_missing",
                    "step must expose success criteria",
                );
            }
            if !step.explain_before_action {
                self.push(
                    &step.step_id,
                    "learning_tour.explain_before_action_missing",
                    "step must explain before action",
                );
            }
            self.validate_action(&step.skip_action, &step.step_id);
            self.validate_action(&step.reset_action, &step.step_id);
            self.validate_action(&step.primary_action, &step.step_id);
            if let Some(package) = self.manifest.package(&step.package_ref).cloned() {
                if package.install_state == PackageInstallState::NotInstalled && step.active {
                    self.push(
                        &step.step_id,
                        "learning_tour.not_installed_step_active",
                        "not-installed package steps must render inactive placeholders",
                    );
                }
                if step.degradation_copy_class != package.degradation_copy_class {
                    self.push(
                        &step.step_id,
                        "learning_tour.step_degradation_mismatch",
                        "step degradation must match its package posture",
                    );
                }
            }
            if step.scope_widening_class != ScopeWideningClass::NoScopeWidening
                && step.primary_action.action_safety_class == ActionSafetyClass::ReadOnly
            {
                self.push(
                    &step.step_id,
                    "learning_tour.scope_widening_without_review",
                    "scope-widening steps must use preview or approval action safety",
                );
            }
        }
    }

    fn validate_action(&mut self, action: &LearningAction, owner_ref: &str) {
        if !action.separate_from_explanation {
            self.push(
                owner_ref,
                "learning_tour.action_explain_do_not_separate",
                format!(
                    "action {} must stay separate from explanation",
                    action.action_id
                ),
            );
        }
        if let Some(command_id) = &action.command_id {
            self.validate_command(
                command_id,
                owner_ref,
                "learning_tour.action_unknown_command",
            );
        }
        if action.action_safety_class.requires_review_path() {
            if action.command_id.is_none() {
                self.push(
                    owner_ref,
                    "learning_tour.review_action_command_missing",
                    format!(
                        "action {} must name the canonical command",
                        action.action_id
                    ),
                );
            }
            if option_is_empty(&action.preview_sheet_ref) {
                self.push(
                    owner_ref,
                    "learning_tour.preview_sheet_missing",
                    format!("action {} must name a preview sheet", action.action_id),
                );
            }
            if action
                .evidence_packet_rule_ref
                .as_deref()
                .map(str::is_empty)
                .unwrap_or(true)
            {
                self.push(
                    owner_ref,
                    "learning_tour.evidence_rule_missing",
                    format!(
                        "action {} must name an evidence packet rule",
                        action.action_id
                    ),
                );
            }
        }
        if action.action_safety_class == ActionSafetyClass::MutationRequiresApproval
            && option_is_empty(&action.approval_path_ref)
        {
            self.push(
                owner_ref,
                "learning_tour.approval_path_missing",
                format!("action {} must name an approval path", action.action_id),
            );
        }
    }

    fn validate_rails(&mut self) {
        let tours = self.tour_ids();
        let steps = self.step_ids();
        let mut ids = BTreeSet::new();
        for rail in self.manifest.exercise_rails.clone() {
            if !ids.insert(rail.rail_id.clone()) {
                self.push(
                    &rail.rail_id,
                    "learning_tour.rail_duplicate",
                    "duplicate exercise rail id",
                );
            }
            if !tours.contains(rail.tour_ref.as_str()) {
                self.push(
                    &rail.rail_id,
                    "learning_tour.rail_unknown_tour",
                    "rail references unknown tour",
                );
            }
            if !steps.contains(rail.current_step_ref.as_str()) {
                self.push(
                    &rail.rail_id,
                    "learning_tour.rail_unknown_current_step",
                    "rail current step does not resolve",
                );
            }
            for step_ref in &rail.ordered_step_refs {
                if !steps.contains(step_ref.as_str()) {
                    self.push(
                        &rail.rail_id,
                        "learning_tour.rail_unknown_step",
                        format!("rail references unknown step {step_ref}"),
                    );
                }
            }
            if rail.success_criteria_refs.is_empty() {
                self.push(
                    &rail.rail_id,
                    "learning_tour.rail_success_missing",
                    "rail must expose success criteria",
                );
            }
            if !rail.explanation_before_action_required {
                self.push(
                    &rail.rail_id,
                    "learning_tour.rail_explain_before_action_missing",
                    "rail must require explanation before action",
                );
            }
            if !rail.reversible_or_sandboxed {
                self.push(
                    &rail.rail_id,
                    "learning_tour.rail_not_reversible_or_sandboxed",
                    "rail must be reversible or sandboxed",
                );
            }
            for action in [
                &rail.hint_action,
                &rail.reveal_action,
                &rail.skip_action,
                &rail.reset_action,
            ] {
                self.validate_action(action, &rail.rail_id);
            }
        }
    }

    fn validate_contextual_surfaces(&mut self) {
        let steps = self.step_ids();
        let support_rows = self
            .manifest
            .support_export
            .rows
            .iter()
            .map(|row| row.row_id.clone())
            .collect::<BTreeSet<_>>();
        let mut ids = BTreeSet::new();
        for surface in self.manifest.contextual_teaching_surfaces.clone() {
            if !ids.insert(surface.surface_id.clone()) {
                self.push(
                    &surface.surface_id,
                    "learning_tour.surface_duplicate",
                    "duplicate contextual surface id",
                );
            }
            if !steps.contains(surface.step_ref.as_str())
                || !steps.contains(surface.current_step_ref.as_str())
            {
                self.push(
                    &surface.surface_id,
                    "learning_tour.surface_unknown_step",
                    "contextual surface references an unknown step",
                );
            }
            self.validate_command(
                &surface.command_id,
                &surface.surface_id,
                "learning_tour.surface_unknown_command",
            );
            if surface.citation_refs.is_empty() || surface.docs_anchor_ref.trim().is_empty() {
                self.push(
                    &surface.surface_id,
                    "learning_tour.surface_anchor_missing",
                    "contextual surface must preserve docs and citation anchors",
                );
            }
            if !surface.explain_and_do_separate || !surface.mutation_uses_command_registry {
                self.push(
                    &surface.surface_id,
                    "learning_tour.surface_authority_split_missing",
                    "contextual surface must separate explanation and use command registry for mutations",
                );
            }
            if !support_rows.contains(surface.support_export_row_ref.as_str()) {
                self.push(
                    &surface.surface_id,
                    "learning_tour.surface_support_row_missing",
                    "contextual surface support export row does not resolve",
                );
            }
        }
    }

    fn validate_progress_snapshots(&mut self) {
        let packages = self.package_ids();
        let tours = self.tour_ids();
        let steps = self.step_ids();
        let mut observed_states = BTreeSet::new();
        for snapshot in self.manifest.progress_snapshots.clone() {
            if snapshot.record_kind != LEARNING_PROGRESS_ALPHA_RECORD_KIND {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_progress.record_kind",
                    "progress snapshot record_kind is unsupported",
                );
            }
            if snapshot.schema_version != LEARNING_PROGRESS_ALPHA_SCHEMA_VERSION {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_progress.schema_version",
                    "progress snapshot schema version is unsupported",
                );
            }
            if snapshot.profile_state.data_ownership_class != "user_owned_portable_profile"
                || snapshot.profile_state.trust_boundary_change_allowed
            {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_progress.ownership_boundary_invalid",
                    "learning profile state must be user-owned and must not change trust boundaries",
                );
            }
            if snapshot.export_posture.raw_step_body_exported
                || snapshot.export_posture.raw_pack_body_exported
                || !snapshot.export_posture.user_can_delete_or_reset
                || snapshot.support_export_projection.raw_profile_body_exported
            {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_progress.export_posture_invalid",
                    "progress export must be redacted and user-resettable",
                );
            }
            for entry in &snapshot.progress_entries {
                observed_states.insert(entry.progress_state_class);
                if !packages.contains(entry.package_ref.as_str()) {
                    self.push(
                        &entry.entry_id,
                        "learning_progress.unknown_package",
                        "progress entry references unknown package",
                    );
                }
                if !tours.contains(entry.tour_ref.as_str()) {
                    self.push(
                        &entry.entry_id,
                        "learning_progress.unknown_tour",
                        "progress entry references unknown tour",
                    );
                }
                if !steps.contains(entry.step_ref.as_str()) {
                    self.push(
                        &entry.entry_id,
                        "learning_progress.unknown_step",
                        "progress entry references unknown step",
                    );
                }
                if !entry.local_only_or_sync_posture_explicit
                    || entry.repo_pack_read_default
                    || entry.classroom_read_default
                    || entry.telemetry_read_default
                    || !entry.preserves_command_help_anchor
                {
                    self.push(
                        &entry.entry_id,
                        "learning_progress.hidden_read_or_reopen_invalid",
                        "progress entry must be explicit, user-owned, and preserve command/help anchors",
                    );
                }
            }
            for dismissal in &snapshot.dismissal_entries {
                if !steps.contains(dismissal.step_ref.as_str()) {
                    self.push(
                        &dismissal.dismissal_id,
                        "learning_progress.dismissal_unknown_step",
                        "dismissal references unknown step",
                    );
                }
                if !dismissal.reversible_from_learning_digest {
                    self.push(
                        &dismissal.dismissal_id,
                        "learning_progress.dismissal_not_reversible",
                        "dismissal must be reversible from the learning digest",
                    );
                }
            }
            for bookmark in &snapshot.bookmark_entries {
                if !steps.contains(bookmark.step_ref.as_str()) {
                    self.push(
                        &bookmark.bookmark_id,
                        "learning_progress.bookmark_unknown_step",
                        "bookmark references unknown step",
                    );
                }
            }
        }
        for required in [
            ProgressStateClass::Completed,
            ProgressStateClass::Dismissed,
            ProgressStateClass::Resumed,
            ProgressStateClass::Deferred,
        ] {
            if !observed_states.contains(&required) {
                self.push(
                    &self.manifest.manifest_id,
                    "learning_progress.state_coverage_missing",
                    format!("progress snapshot must include {}", required.as_str()),
                );
            }
        }
    }

    fn validate_support_export(&mut self) {
        let packages = self.package_ids();
        let steps = self.step_ids();
        let surfaces = self
            .manifest
            .contextual_teaching_surfaces
            .iter()
            .map(|surface| surface.surface_id.clone())
            .collect::<BTreeSet<_>>();
        let snapshots = self
            .manifest
            .progress_snapshots
            .iter()
            .map(|snapshot| snapshot.snapshot_id.clone())
            .collect::<BTreeSet<_>>();
        let support_export_id = self.manifest.support_export.export_id.clone();
        let raw_pack_body_exported = self.manifest.support_export.raw_pack_body_exported;
        if raw_pack_body_exported {
            self.push(
                support_export_id,
                "learning_tour.support_raw_pack_exported",
                "support export must not include raw pack bodies",
            );
        }
        let mut ids = BTreeSet::new();
        for row in self.manifest.support_export.rows.clone() {
            if !ids.insert(row.row_id.clone()) {
                self.push(
                    &row.row_id,
                    "learning_tour.support_row_duplicate",
                    "duplicate support export row id",
                );
            }
            if row.raw_body_exported {
                self.push(
                    &row.row_id,
                    "learning_tour.support_raw_body_exported",
                    "support export row must not include raw bodies",
                );
            }
            if !surfaces.contains(row.surface_ref.as_str()) {
                self.push(
                    &row.row_id,
                    "learning_tour.support_unknown_surface",
                    "support export references unknown contextual surface",
                );
            }
            if !packages.contains(row.package_ref.as_str())
                || !steps.contains(row.step_ref.as_str())
            {
                self.push(
                    &row.row_id,
                    "learning_tour.support_unknown_package_or_step",
                    "support export references unknown package or step",
                );
            }
            if !snapshots.contains(row.progress_snapshot_ref.as_str()) {
                self.push(
                    &row.row_id,
                    "learning_tour.support_unknown_progress_snapshot",
                    "support export references unknown progress snapshot",
                );
            }
            if row.citation_refs.is_empty() || row.exact_reopen_ref.trim().is_empty() {
                self.push(
                    &row.row_id,
                    "learning_tour.support_citation_or_reopen_missing",
                    "support export must preserve citations and exact reopen",
                );
            }
        }
    }

    fn validate_proofs(&mut self) {
        let packages = self.package_ids();
        let steps = self.step_ids();
        for proof in self.manifest.protected_proofs.clone() {
            if proof.exercised_states.is_empty() {
                self.push(
                    &proof.proof_id,
                    "learning_tour.proof_states_missing",
                    "proof must name exercised states",
                );
            }
            for package_ref in &proof.exercised_package_refs {
                if !packages.contains(package_ref.as_str()) {
                    self.push(
                        &proof.proof_id,
                        "learning_tour.proof_unknown_package",
                        format!("proof references unknown package {package_ref}"),
                    );
                }
            }
            for step_ref in &proof.exercised_step_refs {
                if !steps.contains(step_ref.as_str()) {
                    self.push(
                        &proof.proof_id,
                        "learning_tour.proof_unknown_step",
                        format!("proof references unknown step {step_ref}"),
                    );
                }
            }
        }
    }

    fn validate_command(&mut self, command_id: &str, owner_ref: &str, check_id: &str) {
        if !is_command_id(command_id) || self.registry.get(command_id).is_none() {
            self.push(
                owner_ref,
                check_id,
                format!("command id {command_id} does not resolve in the command registry"),
            );
        }
    }

    fn package_ids(&self) -> BTreeSet<String> {
        self.manifest
            .packages
            .iter()
            .map(|package| package.package_id.clone())
            .collect()
    }

    fn tour_ids(&self) -> BTreeSet<String> {
        self.manifest
            .tours
            .iter()
            .map(|tour| tour.tour_id.clone())
            .collect()
    }

    fn step_ids(&self) -> BTreeSet<String> {
        self.manifest
            .steps
            .iter()
            .map(|step| step.step_id.clone())
            .collect()
    }

    fn push(
        &mut self,
        row_ref: impl Into<String>,
        check_id: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.findings.push(LearningTourValidationFinding::new(
            row_ref, check_id, message,
        ));
    }
}

fn is_command_id(value: &str) -> bool {
    value.starts_with("cmd:") && value.len() > "cmd:".len()
}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

fn option_is_empty(value: &Option<String>) -> bool {
    value.as_deref().map(str::is_empty).unwrap_or(true)
}
