//! Beta guided-tour, guided-exercise, and learning-mode records.
//!
//! The module owns the shell-readable beta contract for learnability surfaces:
//! versioned tour packages, exercise rails, learning-mode profiles,
//! user-owned progress snapshots, surface projections, and support exports.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use aureline_commands::CommandRegistry;
use serde::{Deserialize, Serialize};

/// Schema version for beta learning-mode records.
pub const LEARNING_MODE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`LearningModeBetaManifest`].
pub const LEARNING_MODE_BETA_MANIFEST_RECORD_KIND: &str = "learning_mode_beta_manifest_record";

/// Stable record kind for [`LearningModeBetaSurfaceProjection`].
pub const LEARNING_MODE_BETA_SURFACE_RECORD_KIND: &str =
    "learning_mode_beta_surface_projection_record";

/// Stable record kind for [`LearningModeBetaSupportExport`].
pub const LEARNING_MODE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "learning_mode_beta_support_export_record";

/// Stable record kind for [`LearningModeProgressSnapshot`].
pub const LEARNING_MODE_PROGRESS_SNAPSHOT_RECORD_KIND: &str =
    "learning_mode_progress_snapshot_record";

/// Stable id for the seeded beta learning-mode manifest.
pub const LEARNING_MODE_BETA_MANIFEST_ID: &str = "learning-mode:guided-tours:beta:v1";

/// Stable version ref for the seeded beta learning-mode manifest.
pub const LEARNING_MODE_BETA_VERSION_REF: &str = "learning-mode-rev:guided-tours:2026.05.17-01";

/// Repository-relative schema ref for tour package objects.
pub const TOUR_PACKAGE_SCHEMA_REF: &str = "schemas/help/tour_package.schema.json";

/// Repository-relative schema ref for learning-mode profile objects.
pub const LEARNING_MODE_PROFILE_SCHEMA_REF: &str = "schemas/help/learning_mode_profile.schema.json";

/// Repository-relative manifest fixture ref.
pub const LEARNING_MODE_BETA_MANIFEST_FIXTURE_REF: &str =
    "fixtures/help/m3/guided_tours/manifest.json";

/// Repository-relative surface projection fixture ref.
pub const LEARNING_MODE_BETA_SURFACE_FIXTURE_REF: &str =
    "fixtures/help/m3/guided_tours/surface_projection.json";

/// Repository-relative support export fixture ref.
pub const LEARNING_MODE_BETA_SUPPORT_EXPORT_FIXTURE_REF: &str =
    "fixtures/help/m3/guided_tours/support_export.json";

/// Repository-relative docs page ref for the beta learning-mode contract.
pub const LEARNING_MODE_BETA_DOC_REF: &str = "docs/help/m3/guided_tours_and_learning_mode_beta.md";

/// Repository-relative release packet ref for the beta learning-mode contract.
pub const LEARNING_MODE_BETA_PACKET_REF: &str = "artifacts/help/m3/learning_mode_packet.md";

const GENERATED_AT: &str = "2026-05-17T20:30:00Z";

const AVAILABILITY_INSTALLED: &str = "installed";
const AVAILABILITY_CACHED: &str = "cached";
const AVAILABILITY_GRAPH_UNAVAILABLE: &str = "graph_unavailable";

const DEGRADATION_NONE: &str = "no_degradation";
const DEGRADATION_CACHED: &str = "cached_disclosed";
const DEGRADATION_GRAPH_UNAVAILABLE: &str = "graph_unavailable_placeholder";

const FRESHNESS_CURRENT: &str = "current";
const FRESHNESS_CACHED: &str = "cached";
const FRESHNESS_STALE: &str = "stale_disclosed";

const ACTION_READ_ONLY: &str = "read_only";
const ACTION_MUTATION_REQUIRES_APPROVAL: &str = "mutation_requires_approval";
const ACTION_BLOCKED_UNAVAILABLE: &str = "blocked_unavailable";

const COMMAND_REGISTRY: &str = "command_registry";
const LOCAL_PROFILE_STATE: &str = "local_profile_state";

const ROLE_EXPLAIN: &str = "explain";
const ROLE_PREPARE_PREVIEW: &str = "prepare_preview";
const ROLE_OPEN_DOCS: &str = "open_docs";
const ROLE_SKIP: &str = "skip";
const ROLE_RESET: &str = "reset";
const ROLE_UNAVAILABLE: &str = "unavailable";

const CONTROL_ENABLE: &str = "enable";
const CONTROL_PAUSE: &str = "pause";
const CONTROL_SNOOZE: &str = "snooze";
const CONTROL_RESET: &str = "reset";
const CONTROL_RESUME: &str = "resume";

const PROGRESS_COMPLETED: &str = "completed";
const PROGRESS_RESUMED: &str = "resumed";
const PROGRESS_DEFERRED: &str = "deferred";
const PROGRESS_DISMISSED: &str = "dismissed";
const PROGRESS_SKIPPED: &str = "skipped";

/// Versioned manifest consumed by guided tours, exercise rails, profiles, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeBetaManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version for the manifest shape.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Stable manifest version ref.
    pub manifest_version_ref: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Lifecycle label rendered by beta or preview learning surfaces.
    pub release_channel: String,
    /// Schema, docs, command, graph, and support contracts this manifest consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that read this manifest directly.
    pub runtime_consumer_refs: Vec<String>,
    /// Policy that allows learning surfaces to downgrade apart from onboarding core.
    pub downgrade_policy: LearningModeDowngradePolicy,
    /// Versioned tour package records.
    pub tour_packages: Vec<TourPackage>,
    /// Guided exercise rails exposed by the shell.
    pub exercise_rails: Vec<GuidedExerciseRail>,
    /// Learning-mode profile records.
    pub learning_profiles: Vec<LearningModeProfile>,
    /// User-owned progress snapshots.
    pub progress_snapshots: Vec<LearningModeProgressSnapshot>,
    /// Bounded support export policy.
    pub support_export_policy: LearningModeSupportExportPolicy,
    /// Protected proof fixture refs.
    pub protected_proofs: Vec<LearningModeProof>,
}

impl LearningModeBetaManifest {
    /// Returns the tour package with `package_id`.
    pub fn package(&self, package_id: &str) -> Option<&TourPackage> {
        self.tour_packages
            .iter()
            .find(|package| package.package_id == package_id)
    }

    /// Returns the tour step with `step_id`.
    pub fn step(&self, step_id: &str) -> Option<&LearningTourStep> {
        self.tour_packages
            .iter()
            .flat_map(|package| package.steps.iter())
            .find(|step| step.step_id == step_id)
    }

    /// Returns the learning profile with `profile_id`.
    pub fn profile(&self, profile_id: &str) -> Option<&LearningModeProfile> {
        self.learning_profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
    }

    /// Returns the progress snapshot with `snapshot_id`.
    pub fn progress_snapshot(&self, snapshot_id: &str) -> Option<&LearningModeProgressSnapshot> {
        self.progress_snapshots
            .iter()
            .find(|snapshot| snapshot.snapshot_id == snapshot_id)
    }

    /// Projects a deterministic surface record for shell rendering.
    pub fn surface_projection(&self) -> LearningModeBetaSurfaceProjection {
        let mut rows = Vec::new();
        for package in &self.tour_packages {
            for step in &package.steps {
                let rail_ref = self
                    .exercise_rails
                    .iter()
                    .find(|rail| rail.current_step_ref == step.step_id)
                    .map(|rail| rail.rail_id.clone());
                let primary_command_id = step
                    .actions
                    .iter()
                    .find_map(|action| action.command_id.clone());
                rows.push(LearningModeSurfaceRow {
                    row_id: format!("learning-surface-row:{}", step.step_id),
                    package_ref: package.package_id.clone(),
                    package_version_ref: package.package_version_ref.clone(),
                    step_ref: step.step_id.clone(),
                    rail_ref,
                    profile_ref: step.profile_ref.clone(),
                    release_label: package.release_label.clone(),
                    availability_state: package.availability_state.clone(),
                    degradation_state: step.degradation_state.clone(),
                    stable_target_refs: step.stable_targets.clone(),
                    command_id: primary_command_id,
                    citation_refs: step.citation_refs.clone(),
                    explain_and_apply_separate: step.actions.iter().all(|action| {
                        action.explain_and_apply_separate
                            && !(action.verb_class == ROLE_EXPLAIN && action.mutates_workspace)
                    }),
                    preview_required: step.actions.iter().any(|action| {
                        action.action_safety_class == ACTION_MUTATION_REQUIRES_APPROVAL
                            && action.preview_sheet_ref.is_some()
                    }),
                    restart_safe: self
                        .exercise_rails
                        .iter()
                        .find(|rail| rail.current_step_ref == step.step_id)
                        .map(|rail| rail.hint_reveal_state.restart_safe)
                        .unwrap_or(true),
                    rate_limit_ref: self
                        .exercise_rails
                        .iter()
                        .find(|rail| rail.current_step_ref == step.step_id)
                        .map(|rail| rail.hint_reveal_state.rate_limit_ref.clone()),
                    exact_reopen_ref: step.exact_reopen_ref.clone(),
                    support_row_ref: format!("support-row:learning-mode:{}", step.step_id),
                });
            }
        }

        let profile_controls = self
            .learning_profiles
            .iter()
            .flat_map(|profile| {
                profile
                    .controls
                    .iter()
                    .map(|control| LearningModeProfileControlRow {
                        row_id: format!(
                            "learning-profile-control-row:{}:{}",
                            profile.profile_id, control.control_class
                        ),
                        profile_ref: profile.profile_id.clone(),
                        control_class: control.control_class.clone(),
                        state_transition: control.state_transition.clone(),
                        user_visible: control.user_visible,
                        reversible_from_learning_digest: control.reversible_from_learning_digest,
                        silent_write_allowed: control.silent_write_allowed,
                    })
            })
            .collect::<Vec<_>>();

        LearningModeBetaSurfaceProjection {
            record_kind: LEARNING_MODE_BETA_SURFACE_RECORD_KIND.to_owned(),
            schema_version: LEARNING_MODE_BETA_SCHEMA_VERSION,
            projection_id: "learning-mode:guided-tours:surface-projection:v1".to_owned(),
            generated_at: self.generated_at.clone(),
            manifest_id: self.manifest_id.clone(),
            manifest_version_ref: self.manifest_version_ref.clone(),
            rows,
            profile_controls,
            coverage: LearningModeSurfaceCoverage {
                package_count: self.tour_packages.len() as u32,
                step_count: self
                    .tour_packages
                    .iter()
                    .map(|package| package.steps.len() as u32)
                    .sum(),
                rail_count: self.exercise_rails.len() as u32,
                profile_count: self.learning_profiles.len() as u32,
            },
        }
    }

    /// Projects a metadata-only support export.
    pub fn support_export(
        &self,
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> LearningModeBetaSupportExport {
        let active_package_versions = self
            .tour_packages
            .iter()
            .map(|package| LearningModeActivePackageVersion {
                package_id: package.package_id.clone(),
                package_version_ref: package.package_version_ref.clone(),
                package_revision_ref: package.package_revision_ref.clone(),
                release_label: package.release_label.clone(),
                availability_state: package.availability_state.clone(),
                freshness_class: package.freshness_class.clone(),
                downgrade_group_ref: package.independent_downgrade_group.clone(),
            })
            .collect::<Vec<_>>();

        let profile_rows = self
            .learning_profiles
            .iter()
            .map(|profile| LearningModeProfileSupportRow {
                row_id: format!("support-row:learning-mode-profile:{}", profile.profile_id),
                profile_ref: profile.profile_id.clone(),
                profile_scope: profile.profile_scope.clone(),
                profile_state: profile.profile_state.clone(),
                tip_intensity_class: profile.tip_intensity_class.clone(),
                jargon_level_class: profile.jargon_level_class.clone(),
                mutation_guardrail_class: profile.mutation_guardrail_class.clone(),
                optional_sync_posture: profile.optional_sync_posture.clone(),
                authority_boundary_change_allowed: profile.authority_boundary_change_allowed,
                blocking_onboarding_allowed: profile.blocking_onboarding_allowed,
                raw_profile_body_exported: false,
            })
            .collect::<Vec<_>>();

        let mut progress_rows = Vec::new();
        for snapshot in &self.progress_snapshots {
            for entry in &snapshot.progress_entries {
                progress_rows.push(LearningModeProgressSupportRow {
                    row_id: format!("support-row:learning-progress:{}", entry.entry_id),
                    snapshot_ref: snapshot.snapshot_id.clone(),
                    profile_ref: snapshot.profile_ref.clone(),
                    package_ref: entry.package_ref.clone(),
                    step_ref: entry.step_ref.clone(),
                    progress_state_class: entry.progress_state_class.clone(),
                    local_or_sync_posture: entry.local_or_sync_posture.clone(),
                    export_scope: entry.export_scope.clone(),
                    exact_reopen_ref: entry.exact_reopen_ref.clone(),
                    raw_step_body_exported: false,
                    raw_pack_body_exported: false,
                });
            }
        }

        LearningModeBetaSupportExport {
            record_kind: LEARNING_MODE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LEARNING_MODE_BETA_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            source_manifest_id: self.manifest_id.clone(),
            manifest_version_ref: self.manifest_version_ref.clone(),
            active_package_versions,
            profile_rows,
            progress_rows,
            omitted_material_classes: self.support_export_policy.omitted_material_classes.clone(),
            raw_bodies_exported: false,
        }
    }

    /// Validates the manifest against the command registry and beta invariants.
    pub fn validate_against_registry(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), Vec<LearningModeBetaFinding>> {
        let mut validator = LearningModeBetaValidator::new(self, registry);
        validator.validate();
        validator.finish()
    }
}

/// Downgrade policy for learning surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeDowngradePolicy {
    /// Stable policy id.
    pub policy_id: String,
    /// Whether learning surfaces downgrade independently from onboarding core.
    pub independent_from_onboarding_core: bool,
    /// Whether stale evidence can suppress beta surfaces without disabling core entry.
    pub stale_evidence_can_suppress_learning: bool,
    /// User-visible downgrade states.
    pub downgrade_states: Vec<String>,
}

/// Versioned package that owns tour steps and source truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TourPackage {
    /// Stable package id.
    pub package_id: String,
    /// Stable package version ref.
    pub package_version_ref: String,
    /// Immutable package revision ref.
    pub package_revision_ref: String,
    /// User-visible beta or preview label.
    pub release_label: String,
    /// Source class for the package.
    pub source_class: String,
    /// Source ref for the package.
    pub source_ref: String,
    /// Source version ref for citations.
    pub source_version_ref: String,
    /// Availability state after docs, graph, and cache checks.
    pub availability_state: String,
    /// Degradation state rendered when availability is incomplete.
    pub degradation_state: String,
    /// Freshness class for the package.
    pub freshness_class: String,
    /// Docs pack that backs citations.
    pub docs_pack_ref: String,
    /// Docs pack revision that backs citations.
    pub docs_pack_revision_ref: String,
    /// Graph epoch used by graph-backed targets.
    pub graph_epoch_ref: Option<String>,
    /// Command graph ref used by action targets.
    pub command_graph_ref: String,
    /// Semantic graph ref used by file, symbol, and graph-node targets.
    pub semantic_graph_ref: String,
    /// Citation refs preserved by all package consumers.
    pub citation_refs: Vec<String>,
    /// Independent downgrade group for release labels.
    pub independent_downgrade_group: String,
    /// Whether this package can downgrade without disabling onboarding core.
    pub downgrade_allowed: bool,
    /// Tours owned by this package.
    pub tours: Vec<LearningTour>,
    /// Ordered step records owned by this package.
    pub steps: Vec<LearningTourStep>,
}

/// Multi-step tour descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTour {
    /// Stable tour id.
    pub tour_id: String,
    /// Package that owns the tour.
    pub package_ref: String,
    /// Reviewable tour title.
    pub title: String,
    /// Audience class for the tour.
    pub audience_class: String,
    /// Profile ref that owns progress.
    pub profile_ref: String,
    /// Scope ref the tour operates in.
    pub scope_ref: String,
    /// Ordered step refs.
    pub ordered_step_refs: Vec<String>,
    /// Docs node refs cited by the tour.
    pub docs_node_refs: Vec<String>,
    /// Citation refs backing the tour.
    pub citation_refs: Vec<String>,
    /// Restart-safe resume ref.
    pub restart_resume_ref: String,
    /// Whether this tour is required before first edit.
    pub required_before_first_edit: bool,
}

/// One guided-tour or guided-exercise step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTourStep {
    /// Stable step id.
    pub step_id: String,
    /// Tour that owns this step.
    pub tour_ref: String,
    /// Package that owns this step.
    pub package_ref: String,
    /// Profile ref used for progress.
    pub profile_ref: String,
    /// Zero-based position in the tour.
    pub position_index: u32,
    /// Step kind token.
    pub step_kind: String,
    /// Reviewable step title.
    pub title: String,
    /// Stable command, file, symbol, docs, graph, or surface targets.
    pub stable_targets: Vec<LearningTargetRef>,
    /// Scope ref before the step runs.
    pub current_scope_ref: String,
    /// Source claims made by the step.
    pub source_claims: Vec<LearningSourceClaim>,
    /// Citation refs backing the step.
    pub citation_refs: Vec<String>,
    /// Success criteria shown by the rail.
    pub success_criteria: Vec<String>,
    /// Hint/reveal state row ref.
    pub hint_reveal_state_ref: String,
    /// Actions exposed by the step.
    pub actions: Vec<LearningAction>,
    /// Degradation state rendered by the step.
    pub degradation_state: String,
    /// Exact reopen ref for this step.
    pub exact_reopen_ref: String,
    /// Whether this step may render as active.
    pub active: bool,
}

/// Stable object target for learning content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningTargetRef {
    /// Target kind such as command, file, symbol, docs node, graph node, or surface.
    pub target_kind: String,
    /// Command id when targeting the command graph.
    pub command_id: Option<String>,
    /// File object id when targeting a file.
    pub file_object_id: Option<String>,
    /// Symbol object id when targeting a symbol.
    pub symbol_object_id: Option<String>,
    /// Docs node id when targeting docs.
    pub docs_node_id: Option<String>,
    /// Graph node id when targeting the semantic graph.
    pub graph_node_id: Option<String>,
    /// Surface object id when targeting shell chrome.
    pub surface_object_id: Option<String>,
}

impl LearningTargetRef {
    /// Returns true when the target uses a stable non-coordinate anchor.
    pub fn is_stable_anchor(&self) -> bool {
        match self.target_kind.as_str() {
            "command_id" => self.command_id.as_deref().is_some_and(is_command_id),
            "file_object_id" => self.file_object_id.as_deref().is_some_and(non_empty),
            "symbol_object_id" => self.symbol_object_id.as_deref().is_some_and(non_empty),
            "docs_node_id" => self.docs_node_id.as_deref().is_some_and(non_empty),
            "graph_node_id" => self.graph_node_id.as_deref().is_some_and(non_empty),
            "surface_object_id" => self.surface_object_id.as_deref().is_some_and(non_empty),
            _ => false,
        }
    }
}

/// Source claim shown by a guided step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSourceClaim {
    /// Stable claim id.
    pub claim_id: String,
    /// Claim class such as product truth, repo truth, or inference.
    pub claim_class: String,
    /// Stable source refs backing the claim.
    pub source_refs: Vec<String>,
    /// Confidence class for generated or graph-backed claims.
    pub confidence_class: String,
}

/// Action exposed by a learning step or rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningAction {
    /// Stable action id.
    pub action_id: String,
    /// Label ref for localized UI copy.
    pub label_ref: String,
    /// Verb class used to keep explanation and application separate.
    pub verb_class: String,
    /// Command id when the action invokes the command graph.
    pub command_id: Option<String>,
    /// Source of command metadata.
    pub command_metadata_source: String,
    /// Safety class for the action.
    pub action_safety_class: String,
    /// Preview sheet ref for preview-before-write actions.
    pub preview_sheet_ref: Option<String>,
    /// Approval path ref for mutation-capable actions.
    pub approval_path_ref: Option<String>,
    /// Rollback or reset semantics ref for mutation-capable actions.
    pub rollback_semantics_ref: Option<String>,
    /// Evidence packet rule ref for mutation-capable actions.
    pub evidence_packet_rule_ref: Option<String>,
    /// Whether action and explanatory copy are separate controls.
    pub explain_and_apply_separate: bool,
    /// Whether this action can mutate workspace or profile data.
    pub mutates_workspace: bool,
}

/// Guided exercise rail consumed by shell learning surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseRail {
    /// Stable rail id.
    pub rail_id: String,
    /// Package that owns the rail.
    pub package_ref: String,
    /// Tour that owns the rail.
    pub tour_ref: String,
    /// Current step ref.
    pub current_step_ref: String,
    /// Ordered step refs displayed in the rail.
    pub ordered_step_refs: Vec<String>,
    /// Success criteria refs displayed in the rail.
    pub success_criteria_refs: Vec<String>,
    /// Hint/reveal persistence and rate-limit state.
    pub hint_reveal_state: HintRevealState,
    /// Skip action exposed by the rail.
    pub skip_action: LearningAction,
    /// Reset action exposed by the rail.
    pub reset_action: LearningAction,
    /// Sandbox preference for exercise work.
    pub sandbox_preference: String,
    /// Reversibility preference for exercise work.
    pub reversible_preference: String,
    /// Mutation guardrail class for exercise actions.
    pub mutation_guardrail_class: String,
    /// Whether preview is required before mutation.
    pub preview_required: bool,
    /// Whether approval is required before mutation.
    pub approval_required: bool,
    /// Whether rail state survives restart.
    pub restart_safe: bool,
}

/// Hint and reveal state for a guided exercise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HintRevealState {
    /// Stable state id.
    pub state_ref: String,
    /// User-owned persistence ref.
    pub persistence_ref: String,
    /// Current hint state.
    pub hint_state: String,
    /// Current reveal state.
    pub reveal_state: String,
    /// Stable rate-limit ref.
    pub rate_limit_ref: String,
    /// Rate-limit window in seconds.
    pub rate_limit_window_seconds: u32,
    /// Maximum reveal count per window.
    pub max_reveals_per_window: u32,
    /// Whether state survives restart.
    pub restart_safe: bool,
    /// Whether hint and reveal state can be dismissed.
    pub dismissible: bool,
}

/// Learning-mode profile record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProfile {
    /// Stable profile id.
    pub profile_id: String,
    /// User or workspace profile scope.
    pub profile_scope: String,
    /// Current profile state.
    pub profile_state: String,
    /// Tip intensity class.
    pub tip_intensity_class: String,
    /// Jargon level class.
    pub jargon_level_class: String,
    /// Educational AI explanation posture.
    pub ai_explanation_posture: String,
    /// Mutation guardrail class.
    pub mutation_guardrail_class: String,
    /// Whether explanation comes before apply-capable action by default.
    pub explain_before_act_default: bool,
    /// Whether the profile can alter authority or trust boundaries.
    pub authority_boundary_change_allowed: bool,
    /// Data ownership class for profile state.
    pub data_ownership_class: String,
    /// Optional sync posture.
    pub optional_sync_posture: String,
    /// Dismissal state ref.
    pub dismissals_state_ref: String,
    /// Bookmark state ref.
    pub bookmarks_state_ref: String,
    /// Whether onboarding can block first edit.
    pub blocking_onboarding_allowed: bool,
    /// Profile controls exposed to the user.
    pub controls: Vec<LearningModeControl>,
}

/// User-visible learning-mode control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeControl {
    /// Stable control id.
    pub control_id: String,
    /// Control class such as enable, pause, snooze, reset, or resume.
    pub control_class: String,
    /// State transition performed by the control.
    pub state_transition: String,
    /// Whether the control writes user-owned local state.
    pub local_state_write: bool,
    /// Whether the control is reversible from the learning digest.
    pub reversible_from_learning_digest: bool,
    /// Whether the control may silently write state.
    pub silent_write_allowed: bool,
    /// Whether the control is visible in the shell.
    pub user_visible: bool,
}

/// User-owned progress snapshot for learning mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProgressSnapshot {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version for the snapshot shape.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Profile ref that owns the snapshot.
    pub profile_ref: String,
    /// Package refs represented by the snapshot.
    pub package_refs: Vec<String>,
    /// Progress entries in the snapshot.
    pub progress_entries: Vec<LearningModeProgressEntry>,
    /// Hint/reveal entries in the snapshot.
    pub hint_reveal_entries: Vec<LearningModeHintRevealEntry>,
    /// Export posture for the snapshot.
    pub export_posture: LearningModeProgressExportPosture,
    /// Support projection for the snapshot.
    pub support_projection: LearningModeProgressSupportProjection,
}

/// Per-step user progress entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProgressEntry {
    /// Stable progress entry id.
    pub entry_id: String,
    /// Package ref for the entry.
    pub package_ref: String,
    /// Tour ref for the entry.
    pub tour_ref: String,
    /// Step ref for the entry.
    pub step_ref: String,
    /// User-visible progress state.
    pub progress_state_class: String,
    /// Resume ref for restart behavior.
    pub resume_ref: String,
    /// Local-only or optional-sync posture.
    pub local_or_sync_posture: String,
    /// Export scope for this entry.
    pub export_scope: String,
    /// Whether repo packs can read this progress by default.
    pub repo_pack_read_default: bool,
    /// Whether collaborators can read this progress by default.
    pub collaborator_read_default: bool,
    /// Whether telemetry readers can read this progress by default.
    pub telemetry_read_default: bool,
    /// Whether the user can inspect this entry.
    pub inspectable_by_user: bool,
    /// Exact reopen ref for this entry.
    pub exact_reopen_ref: String,
    /// Deterministic update timestamp.
    pub updated_at: String,
}

/// Per-step hint/reveal persistence entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeHintRevealEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Hint/reveal state ref.
    pub state_ref: String,
    /// Hint state persisted for restart.
    pub hint_state: String,
    /// Reveal state persisted for restart.
    pub reveal_state: String,
    /// Rate-limit reset ref.
    pub rate_limit_reset_ref: String,
    /// Whether this entry persists across restart.
    pub persisted_across_restart: bool,
    /// Deterministic visibility timestamp.
    pub last_visible_at: String,
}

/// Export posture for a progress snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProgressExportPosture {
    /// Whether local-only state is the default.
    pub local_only_default: bool,
    /// Whether optional sync is supported.
    pub optional_sync_supported: bool,
    /// Whether sync requires explicit user action.
    pub sync_requires_user_action: bool,
    /// Bounded export field names.
    pub bounded_export_fields: Vec<String>,
    /// Whether raw step body text is exported.
    pub raw_step_body_exported: bool,
    /// Whether raw package body text is exported.
    pub raw_pack_body_exported: bool,
    /// Whether the user can reset progress.
    pub user_can_reset: bool,
    /// Whether the user can export bounded metadata.
    pub user_can_export_metadata: bool,
}

/// Support projection for a progress snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProgressSupportProjection {
    /// Stable support projection id.
    pub projection_id: String,
    /// Support export row refs.
    pub row_refs: Vec<String>,
    /// Whether raw profile body text is exported.
    pub raw_profile_body_exported: bool,
}

/// Support export policy for learning-mode records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeSupportExportPolicy {
    /// Stable policy id.
    pub policy_id: String,
    /// Whether active package versions are recorded.
    pub record_active_package_versions: bool,
    /// Whether profile state classes are recorded.
    pub record_profile_state: bool,
    /// Whether progress metadata is recorded.
    pub record_progress_metadata: bool,
    /// Bounded material classes included in export.
    pub bounded_material_classes: Vec<String>,
    /// Material classes omitted from export.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw bodies are exported.
    pub raw_bodies_exported: bool,
}

/// Protected proof fixture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProof {
    /// Stable proof id.
    pub proof_id: String,
    /// Fixture path relative to the repository root.
    pub fixture_ref: String,
    /// Package refs exercised by the proof.
    pub exercised_package_refs: Vec<String>,
    /// Step refs exercised by the proof.
    pub exercised_step_refs: Vec<String>,
    /// States exercised by the proof.
    pub exercised_states: Vec<String>,
}

/// Shell surface projection for learning-mode rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeBetaSurfaceProjection {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source manifest id.
    pub manifest_id: String,
    /// Source manifest version ref.
    pub manifest_version_ref: String,
    /// Projected learning surface rows.
    pub rows: Vec<LearningModeSurfaceRow>,
    /// Projected profile control rows.
    pub profile_controls: Vec<LearningModeProfileControlRow>,
    /// Surface coverage summary.
    pub coverage: LearningModeSurfaceCoverage,
}

/// One shell learning surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Package ref for the row.
    pub package_ref: String,
    /// Package version ref for the row.
    pub package_version_ref: String,
    /// Step ref for the row.
    pub step_ref: String,
    /// Exercise rail ref when present.
    pub rail_ref: Option<String>,
    /// Profile ref for progress.
    pub profile_ref: String,
    /// Release label rendered by the row.
    pub release_label: String,
    /// Package availability state.
    pub availability_state: String,
    /// Step degradation state.
    pub degradation_state: String,
    /// Stable target refs displayed by the row.
    pub stable_target_refs: Vec<LearningTargetRef>,
    /// Primary command id when the row exposes a command.
    pub command_id: Option<String>,
    /// Citation refs displayed by the row.
    pub citation_refs: Vec<String>,
    /// Whether explanation and apply controls are separate.
    pub explain_and_apply_separate: bool,
    /// Whether preview is required before mutation.
    pub preview_required: bool,
    /// Whether row state survives restart.
    pub restart_safe: bool,
    /// Rate-limit ref for hints or reveal state.
    pub rate_limit_ref: Option<String>,
    /// Exact reopen ref for the row.
    pub exact_reopen_ref: String,
    /// Support export row ref.
    pub support_row_ref: String,
}

/// One profile control row in the surface projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProfileControlRow {
    /// Stable row id.
    pub row_id: String,
    /// Profile ref for the row.
    pub profile_ref: String,
    /// Control class for the row.
    pub control_class: String,
    /// State transition performed by the control.
    pub state_transition: String,
    /// Whether the control is visible.
    pub user_visible: bool,
    /// Whether the control is reversible from the learning digest.
    pub reversible_from_learning_digest: bool,
    /// Whether the control may silently write state.
    pub silent_write_allowed: bool,
}

/// Coverage summary for the learning-mode surface projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeSurfaceCoverage {
    /// Number of projected packages.
    pub package_count: u32,
    /// Number of projected steps.
    pub step_count: u32,
    /// Number of projected exercise rails.
    pub rail_count: u32,
    /// Number of projected profiles.
    pub profile_count: u32,
}

/// Metadata-only support export for beta learning-mode records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeBetaSupportExport {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub support_export_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source manifest id.
    pub source_manifest_id: String,
    /// Source manifest version ref.
    pub manifest_version_ref: String,
    /// Active package versions represented by the export.
    pub active_package_versions: Vec<LearningModeActivePackageVersion>,
    /// Profile rows included in the export.
    pub profile_rows: Vec<LearningModeProfileSupportRow>,
    /// Progress rows included in the export.
    pub progress_rows: Vec<LearningModeProgressSupportRow>,
    /// Omitted material classes.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw body content is exported.
    pub raw_bodies_exported: bool,
}

impl LearningModeBetaSupportExport {
    /// Validates the support export against `manifest`.
    pub fn validate_against_manifest(
        &self,
        manifest: &LearningModeBetaManifest,
    ) -> Result<(), Vec<LearningModeBetaFinding>> {
        let package_ids = manifest.package_ids();
        let profile_ids = manifest.profile_ids();
        let step_ids = manifest.step_ids();
        let snapshot_ids = manifest
            .progress_snapshots
            .iter()
            .map(|snapshot| snapshot.snapshot_id.clone())
            .collect::<BTreeSet<_>>();
        let mut findings = Vec::new();

        if self.record_kind != LEARNING_MODE_BETA_SUPPORT_EXPORT_RECORD_KIND {
            findings.push(LearningModeBetaFinding::new(
                &self.support_export_id,
                "learning_mode.support.record_kind",
                "support export record_kind is unsupported",
            ));
        }
        if self.schema_version != LEARNING_MODE_BETA_SCHEMA_VERSION {
            findings.push(LearningModeBetaFinding::new(
                &self.support_export_id,
                "learning_mode.support.schema_version",
                "support export schema version is unsupported",
            ));
        }
        if self.raw_bodies_exported || self.omitted_material_classes.is_empty() {
            findings.push(LearningModeBetaFinding::new(
                &self.support_export_id,
                "learning_mode.support.raw_body_policy",
                "support export must omit raw bodies and disclose omitted material classes",
            ));
        }

        for version in &self.active_package_versions {
            if !package_ids.contains(version.package_id.as_str()) {
                findings.push(LearningModeBetaFinding::new(
                    &version.package_id,
                    "learning_mode.support.unknown_package",
                    "support export references unknown package",
                ));
            }
        }
        for row in &self.profile_rows {
            if !profile_ids.contains(row.profile_ref.as_str()) {
                findings.push(LearningModeBetaFinding::new(
                    &row.row_id,
                    "learning_mode.support.unknown_profile",
                    "support export references unknown profile",
                ));
            }
            if row.raw_profile_body_exported
                || row.authority_boundary_change_allowed
                || row.blocking_onboarding_allowed
            {
                findings.push(LearningModeBetaFinding::new(
                    &row.row_id,
                    "learning_mode.support.profile_boundary",
                    "profile export must stay redacted, optional, and authority-neutral",
                ));
            }
        }
        for row in &self.progress_rows {
            if !snapshot_ids.contains(row.snapshot_ref.as_str())
                || !profile_ids.contains(row.profile_ref.as_str())
                || !package_ids.contains(row.package_ref.as_str())
                || !step_ids.contains(row.step_ref.as_str())
            {
                findings.push(LearningModeBetaFinding::new(
                    &row.row_id,
                    "learning_mode.support.progress_ref",
                    "progress support row references an unknown snapshot, profile, package, or step",
                ));
            }
            if row.raw_step_body_exported || row.raw_pack_body_exported {
                findings.push(LearningModeBetaFinding::new(
                    &row.row_id,
                    "learning_mode.support.progress_raw_body",
                    "progress support rows must not export raw bodies",
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

/// Active package version row for support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeActivePackageVersion {
    /// Package id.
    pub package_id: String,
    /// Package version ref.
    pub package_version_ref: String,
    /// Package revision ref.
    pub package_revision_ref: String,
    /// Release label.
    pub release_label: String,
    /// Availability state.
    pub availability_state: String,
    /// Freshness class.
    pub freshness_class: String,
    /// Downgrade group ref.
    pub downgrade_group_ref: String,
}

/// Profile row for support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProfileSupportRow {
    /// Stable row id.
    pub row_id: String,
    /// Profile ref.
    pub profile_ref: String,
    /// Profile scope.
    pub profile_scope: String,
    /// Profile state.
    pub profile_state: String,
    /// Tip intensity class.
    pub tip_intensity_class: String,
    /// Jargon level class.
    pub jargon_level_class: String,
    /// Mutation guardrail class.
    pub mutation_guardrail_class: String,
    /// Optional sync posture.
    pub optional_sync_posture: String,
    /// Whether profile changes authority boundaries.
    pub authority_boundary_change_allowed: bool,
    /// Whether profile can block onboarding.
    pub blocking_onboarding_allowed: bool,
    /// Whether raw profile body content is exported.
    pub raw_profile_body_exported: bool,
}

/// Progress row for support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProgressSupportRow {
    /// Stable row id.
    pub row_id: String,
    /// Snapshot ref.
    pub snapshot_ref: String,
    /// Profile ref.
    pub profile_ref: String,
    /// Package ref.
    pub package_ref: String,
    /// Step ref.
    pub step_ref: String,
    /// Progress state class.
    pub progress_state_class: String,
    /// Local-only or optional-sync posture.
    pub local_or_sync_posture: String,
    /// Export scope.
    pub export_scope: String,
    /// Exact reopen ref.
    pub exact_reopen_ref: String,
    /// Whether raw step body content is exported.
    pub raw_step_body_exported: bool,
    /// Whether raw package body content is exported.
    pub raw_pack_body_exported: bool,
}

/// Validation finding for learning-mode beta records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeBetaFinding {
    /// Row or object that failed validation.
    pub row_ref: String,
    /// Stable validation check id.
    pub check_id: String,
    /// Reviewable validation message.
    pub message: String,
}

impl LearningModeBetaFinding {
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

/// Error returned when beta learning-mode records fail to build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearningModeBetaError {
    /// Seeded records failed validation.
    Validation(Vec<LearningModeBetaFinding>),
}

impl fmt::Display for LearningModeBetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(findings) => {
                write!(f, "learning-mode beta validation failed: ")?;
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

impl std::error::Error for LearningModeBetaError {}

/// Returns the seeded beta learning-mode manifest.
pub fn seeded_learning_mode_beta_manifest() -> LearningModeBetaManifest {
    let profile_ref = "profile:learning.default_individual";
    let safe_start_package = TourPackage {
        package_id: "tour-pack:aureline.safe-start.beta".to_owned(),
        package_version_ref: "tour-pack-rev:aureline.safe-start:2026.05.17-01".to_owned(),
        package_revision_ref: "tour-pack-revision:aureline.safe-start:sha256:001".to_owned(),
        release_label: "beta".to_owned(),
        source_class: "project_docs".to_owned(),
        source_ref: LEARNING_MODE_BETA_DOC_REF.to_owned(),
        source_version_ref: "docs-help:guided-tours:2026.05.17".to_owned(),
        availability_state: AVAILABILITY_INSTALLED.to_owned(),
        degradation_state: DEGRADATION_NONE.to_owned(),
        freshness_class: FRESHNESS_CURRENT.to_owned(),
        docs_pack_ref: "docs-pack:aureline-help:guided-tours".to_owned(),
        docs_pack_revision_ref: "docs-pack-rev:aureline-help:2026.05.17-01".to_owned(),
        graph_epoch_ref: Some("graph-epoch:aureline-reference:2026.05.17".to_owned()),
        command_graph_ref: "command-graph:aureline:2026.05.17".to_owned(),
        semantic_graph_ref: "semantic-graph:aureline-reference:2026.05.17".to_owned(),
        citation_refs: vec![
            "citation:tdd:7.3.7:learning-rail-architecture".to_owned(),
            "citation:uiux:17.24:learning-mode-guided-exercises".to_owned(),
        ],
        independent_downgrade_group: "downgrade-group:learnability-beta".to_owned(),
        downgrade_allowed: true,
        tours: vec![LearningTour {
            tour_id: "tour:aureline.safe-start.command-backed".to_owned(),
            package_ref: "tour-pack:aureline.safe-start.beta".to_owned(),
            title: "Command-backed safe start".to_owned(),
            audience_class: "new_contributor_or_learner".to_owned(),
            profile_ref: profile_ref.to_owned(),
            scope_ref: "scope:local-workspace-entry".to_owned(),
            ordered_step_refs: vec![
                "step:safe-start.open-folder".to_owned(),
                "step:safe-start.read-docs-source".to_owned(),
                "step:safe-start.import-profile-preview".to_owned(),
            ],
            docs_node_refs: vec![
                "docs-node:help.guided-tours.safe-start".to_owned(),
                "docs-node:onboarding.import-profile.preview".to_owned(),
            ],
            citation_refs: vec![
                "citation:tdd:7.3.7:tour-package".to_owned(),
                "citation:uiux:17.24:explain-do-separation".to_owned(),
            ],
            restart_resume_ref: "resume:tour:aureline.safe-start.command-backed".to_owned(),
            required_before_first_edit: false,
        }],
        steps: vec![
            LearningTourStep {
                step_id: "step:safe-start.open-folder".to_owned(),
                tour_ref: "tour:aureline.safe-start.command-backed".to_owned(),
                package_ref: "tour-pack:aureline.safe-start.beta".to_owned(),
                profile_ref: profile_ref.to_owned(),
                position_index: 0,
                step_kind: "guided_tour_step".to_owned(),
                title: "Open a local folder through the command graph".to_owned(),
                stable_targets: vec![target_command("cmd:workspace.open_folder")],
                current_scope_ref: "scope:local-workspace-entry".to_owned(),
                source_claims: vec![source_claim(
                    "claim:safe-start.open-folder.command-backed",
                    "product_truth",
                    vec![
                        "cmd:workspace.open_folder",
                        "docs-node:workspace.open-folder",
                    ],
                    "cited",
                )],
                citation_refs: vec![
                    "citation:command-registry:workspace.open-folder".to_owned(),
                    "citation:docs:workspace.open-folder".to_owned(),
                ],
                success_criteria: vec![
                    "success:open-folder.command-visible".to_owned(),
                    "success:open-folder.no-account-required".to_owned(),
                ],
                hint_reveal_state_ref: "hint-state:safe-start.open-folder".to_owned(),
                actions: vec![
                    read_only_action(
                        "action:safe-start.open-folder.explain",
                        "msg:learning.safe-start.open-folder.explain",
                        ROLE_EXPLAIN,
                        None,
                    ),
                    read_only_action(
                        "action:safe-start.open-folder.invoke",
                        "msg:learning.safe-start.open-folder.invoke",
                        "invoke_command",
                        Some("cmd:workspace.open_folder"),
                    ),
                ],
                degradation_state: DEGRADATION_NONE.to_owned(),
                exact_reopen_ref: "reopen:step:safe-start.open-folder@tour-pack-rev:aureline.safe-start:2026.05.17-01#en-US".to_owned(),
                active: true,
            },
            LearningTourStep {
                step_id: "step:safe-start.read-docs-source".to_owned(),
                tour_ref: "tour:aureline.safe-start.command-backed".to_owned(),
                package_ref: "tour-pack:aureline.safe-start.beta".to_owned(),
                profile_ref: profile_ref.to_owned(),
                position_index: 1,
                step_kind: "guided_tour_step".to_owned(),
                title: "Open the cited docs source before trusting a claim".to_owned(),
                stable_targets: vec![
                    target_docs("docs-node:help.guided-tours.safe-start"),
                    target_file("file:docs/help/m3/guided_tours_and_learning_mode_beta.md"),
                ],
                current_scope_ref: "scope:docs-help-guided-tour".to_owned(),
                source_claims: vec![source_claim(
                    "claim:safe-start.docs-source.cited",
                    "product_truth",
                    vec![
                        "docs-node:help.guided-tours.safe-start",
                        "file:docs/help/m3/guided_tours_and_learning_mode_beta.md",
                    ],
                    "cited",
                )],
                citation_refs: vec![
                    "citation:docs-help:guided-tours-beta".to_owned(),
                    "citation:architecture:BR.2:derived-tour-artifacts".to_owned(),
                ],
                success_criteria: vec![
                    "success:docs-source.citation-visible".to_owned(),
                    "success:docs-source.open-source-action-visible".to_owned(),
                ],
                hint_reveal_state_ref: "hint-state:safe-start.read-docs-source".to_owned(),
                actions: vec![
                    read_only_action(
                        "action:safe-start.read-docs.explain",
                        "msg:learning.safe-start.read-docs.explain",
                        ROLE_EXPLAIN,
                        None,
                    ),
                    read_only_action(
                        "action:safe-start.read-docs.open",
                        "msg:learning.safe-start.read-docs.open",
                        ROLE_OPEN_DOCS,
                        Some("cmd:docs.open_in_browser"),
                    ),
                ],
                degradation_state: DEGRADATION_NONE.to_owned(),
                exact_reopen_ref: "reopen:step:safe-start.read-docs-source@tour-pack-rev:aureline.safe-start:2026.05.17-01#en-US".to_owned(),
                active: true,
            },
            LearningTourStep {
                step_id: "step:safe-start.import-profile-preview".to_owned(),
                tour_ref: "tour:aureline.safe-start.command-backed".to_owned(),
                package_ref: "tour-pack:aureline.safe-start.beta".to_owned(),
                profile_ref: profile_ref.to_owned(),
                position_index: 2,
                step_kind: "guided_exercise_step".to_owned(),
                title: "Prepare an import preview before any profile mutation".to_owned(),
                stable_targets: vec![
                    target_command("cmd:workspace.import_profile"),
                    target_graph("graph-node:command.workspace.import_profile"),
                ],
                current_scope_ref: "scope:profile-import-review".to_owned(),
                source_claims: vec![source_claim(
                    "claim:safe-start.import-profile.preview-required",
                    "product_truth",
                    vec![
                        "cmd:workspace.import_profile",
                        "preview:workspace.import_profile",
                        "approval:path:workspace.import_profile",
                    ],
                    "cited",
                )],
                citation_refs: vec![
                    "citation:command-registry:workspace.import-profile".to_owned(),
                    "citation:docs:onboarding.import-profile.preview".to_owned(),
                ],
                success_criteria: vec![
                    "success:import-profile.preview-sheet-opened".to_owned(),
                    "success:import-profile.approval-fence-visible".to_owned(),
                    "success:import-profile.rollback-semantics-visible".to_owned(),
                ],
                hint_reveal_state_ref: "hint-state:safe-start.import-profile-preview".to_owned(),
                actions: vec![
                    read_only_action(
                        "action:safe-start.import-profile.explain",
                        "msg:learning.import-profile.explain",
                        ROLE_EXPLAIN,
                        None,
                    ),
                    mutation_action(
                        "action:safe-start.import-profile.prepare-preview",
                        "msg:learning.import-profile.prepare-preview",
                        "cmd:workspace.import_profile",
                        "preview:workspace.import_profile",
                        "approval:path:workspace.import_profile",
                        "rollback:workspace.import_profile.checkpoint-or-undo",
                        "evidence-rule:command-preview-approval",
                    ),
                ],
                degradation_state: DEGRADATION_NONE.to_owned(),
                exact_reopen_ref: "reopen:step:safe-start.import-profile-preview@tour-pack-rev:aureline.safe-start:2026.05.17-01#en-US".to_owned(),
                active: true,
            },
        ],
    };

    let cached_package = TourPackage {
        package_id: "tour-pack:aureline.cached-docs.preview".to_owned(),
        package_version_ref: "tour-pack-rev:aureline.cached-docs:2026.05.17-01".to_owned(),
        package_revision_ref: "tour-pack-revision:aureline.cached-docs:sha256:001".to_owned(),
        release_label: "preview".to_owned(),
        source_class: "mirrored_official_docs".to_owned(),
        source_ref: "mirror:docs:aureline-guided-learning".to_owned(),
        source_version_ref: "mirror-snapshot:aureline-guided-learning:2026.05.10".to_owned(),
        availability_state: AVAILABILITY_CACHED.to_owned(),
        degradation_state: DEGRADATION_CACHED.to_owned(),
        freshness_class: FRESHNESS_CACHED.to_owned(),
        docs_pack_ref: "docs-pack:mirror:aureline-guided-learning".to_owned(),
        docs_pack_revision_ref: "docs-pack-rev:mirror:aureline-guided-learning:2026.05.10-01"
            .to_owned(),
        graph_epoch_ref: Some("graph-epoch:aureline-reference:2026.05.10".to_owned()),
        command_graph_ref: "command-graph:aureline:2026.05.17".to_owned(),
        semantic_graph_ref: "semantic-graph:aureline-reference:2026.05.10".to_owned(),
        citation_refs: vec!["citation:mirror:guided-learning.cached-docs".to_owned()],
        independent_downgrade_group: "downgrade-group:learnability-preview".to_owned(),
        downgrade_allowed: true,
        tours: vec![LearningTour {
            tour_id: "tour:aureline.cached-docs.fallback".to_owned(),
            package_ref: "tour-pack:aureline.cached-docs.preview".to_owned(),
            title: "Cached docs fallback".to_owned(),
            audience_class: "offline_or_mirror_profile".to_owned(),
            profile_ref: profile_ref.to_owned(),
            scope_ref: "scope:cached-docs-help".to_owned(),
            ordered_step_refs: vec!["step:cached-docs.open-with-label".to_owned()],
            docs_node_refs: vec!["docs-node:help.cached-docs.learning".to_owned()],
            citation_refs: vec!["citation:mirror:guided-learning.cached-docs".to_owned()],
            restart_resume_ref: "resume:tour:aureline.cached-docs.fallback".to_owned(),
            required_before_first_edit: false,
        }],
        steps: vec![LearningTourStep {
            step_id: "step:cached-docs.open-with-label".to_owned(),
            tour_ref: "tour:aureline.cached-docs.fallback".to_owned(),
            package_ref: "tour-pack:aureline.cached-docs.preview".to_owned(),
            profile_ref: profile_ref.to_owned(),
            position_index: 0,
            step_kind: "guided_tour_step".to_owned(),
            title: "Open cached docs with the cached label visible".to_owned(),
            stable_targets: vec![target_docs("docs-node:help.cached-docs.learning")],
            current_scope_ref: "scope:cached-docs-help".to_owned(),
            source_claims: vec![source_claim(
                "claim:cached-docs.fallback.disclosed",
                "product_truth",
                vec!["docs-node:help.cached-docs.learning"],
                "cached_cited",
            )],
            citation_refs: vec!["citation:mirror:guided-learning.cached-docs".to_owned()],
            success_criteria: vec![
                "success:cached-docs.cached-label-visible".to_owned(),
                "success:cached-docs.exact-reopen-preserved".to_owned(),
            ],
            hint_reveal_state_ref: "hint-state:cached-docs.open-with-label".to_owned(),
            actions: vec![read_only_action(
                "action:cached-docs.open",
                "msg:learning.cached-docs.open",
                ROLE_OPEN_DOCS,
                Some("cmd:docs.open_in_browser"),
            )],
            degradation_state: DEGRADATION_CACHED.to_owned(),
            exact_reopen_ref: "reopen:step:cached-docs.open-with-label@tour-pack-rev:aureline.cached-docs:2026.05.17-01#en-US".to_owned(),
            active: true,
        }],
    };

    let graph_unavailable_package = TourPackage {
        package_id: "tour-pack:aureline.graph-map.placeholder".to_owned(),
        package_version_ref: "tour-pack-rev:aureline.graph-map:2026.05.17-01".to_owned(),
        package_revision_ref: "tour-pack-revision:aureline.graph-map:sha256:001".to_owned(),
        release_label: "preview".to_owned(),
        source_class: "semantic_graph".to_owned(),
        source_ref: "graph:aureline-reference:unavailable".to_owned(),
        source_version_ref: "graph-epoch:unavailable".to_owned(),
        availability_state: AVAILABILITY_GRAPH_UNAVAILABLE.to_owned(),
        degradation_state: DEGRADATION_GRAPH_UNAVAILABLE.to_owned(),
        freshness_class: FRESHNESS_STALE.to_owned(),
        docs_pack_ref: "docs-pack:aureline-help:graph-map-placeholder".to_owned(),
        docs_pack_revision_ref: "docs-pack-rev:aureline-help:2026.05.17-01".to_owned(),
        graph_epoch_ref: None,
        command_graph_ref: "command-graph:aureline:2026.05.17".to_owned(),
        semantic_graph_ref: "semantic-graph:unavailable".to_owned(),
        citation_refs: vec![
            "citation:architecture:BR.2:derived-tour-artifacts".to_owned(),
            "citation:docs-help:guided-tours.graph-unavailable".to_owned(),
        ],
        independent_downgrade_group: "downgrade-group:learnability-preview".to_owned(),
        downgrade_allowed: true,
        tours: vec![LearningTour {
            tour_id: "tour:aureline.graph-map.placeholder".to_owned(),
            package_ref: "tour-pack:aureline.graph-map.placeholder".to_owned(),
            title: "Graph-backed tour placeholder".to_owned(),
            audience_class: "offline_or_index_warming_profile".to_owned(),
            profile_ref: profile_ref.to_owned(),
            scope_ref: "scope:semantic-graph-unavailable".to_owned(),
            ordered_step_refs: vec!["step:graph-map.placeholder".to_owned()],
            docs_node_refs: vec!["docs-node:help.graph-map.unavailable".to_owned()],
            citation_refs: vec![
                "citation:architecture:BR.2:derived-tour-artifacts".to_owned(),
                "citation:docs-help:guided-tours.graph-unavailable".to_owned(),
            ],
            restart_resume_ref: "resume:tour:aureline.graph-map.placeholder".to_owned(),
            required_before_first_edit: false,
        }],
        steps: vec![LearningTourStep {
            step_id: "step:graph-map.placeholder".to_owned(),
            tour_ref: "tour:aureline.graph-map.placeholder".to_owned(),
            package_ref: "tour-pack:aureline.graph-map.placeholder".to_owned(),
            profile_ref: profile_ref.to_owned(),
            position_index: 0,
            step_kind: "fallback_placeholder".to_owned(),
            title: "Show a placeholder while graph facts are unavailable".to_owned(),
            stable_targets: vec![
                target_docs("docs-node:help.graph-map.unavailable"),
                target_graph("graph-node:unavailable"),
            ],
            current_scope_ref: "scope:semantic-graph-unavailable".to_owned(),
            source_claims: vec![source_claim(
                "claim:graph-map.placeholder.degraded",
                "inference_limit",
                vec!["docs-node:help.graph-map.unavailable"],
                "not_available_disclosed",
            )],
            citation_refs: vec![
                "citation:architecture:BR.2:derived-tour-artifacts".to_owned(),
                "citation:docs-help:guided-tours.graph-unavailable".to_owned(),
            ],
            success_criteria: vec![
                "success:graph-placeholder.visible".to_owned(),
                "success:graph-placeholder.no-uncited-repo-truth".to_owned(),
            ],
            hint_reveal_state_ref: "hint-state:graph-map.placeholder".to_owned(),
            actions: vec![blocked_action(
                "action:graph-map.placeholder.unavailable",
                "msg:learning.graph-map.unavailable",
            )],
            degradation_state: DEGRADATION_GRAPH_UNAVAILABLE.to_owned(),
            exact_reopen_ref: "reopen:step:graph-map.placeholder@tour-pack-rev:aureline.graph-map:2026.05.17-01#en-US".to_owned(),
            active: false,
        }],
    };

    let packages = vec![
        safe_start_package,
        cached_package,
        graph_unavailable_package,
    ];

    LearningModeBetaManifest {
        record_kind: LEARNING_MODE_BETA_MANIFEST_RECORD_KIND.to_owned(),
        schema_version: LEARNING_MODE_BETA_SCHEMA_VERSION,
        manifest_id: LEARNING_MODE_BETA_MANIFEST_ID.to_owned(),
        manifest_version_ref: LEARNING_MODE_BETA_VERSION_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        release_channel: "beta".to_owned(),
        contract_refs: BTreeMap::from([
            (
                "tour_package_schema".to_owned(),
                TOUR_PACKAGE_SCHEMA_REF.to_owned(),
            ),
            (
                "learning_mode_profile_schema".to_owned(),
                LEARNING_MODE_PROFILE_SCHEMA_REF.to_owned(),
            ),
            (
                "command_registry".to_owned(),
                "artifacts/commands/command_registry_seed.yaml".to_owned(),
            ),
            (
                "docs_graph".to_owned(),
                "artifacts/docs/docs_pack_alpha_manifest.yaml".to_owned(),
            ),
            (
                "semantic_graph".to_owned(),
                "schemas/graph/workspace_graph_seed.schema.json".to_owned(),
            ),
            (
                "release_packet".to_owned(),
                LEARNING_MODE_BETA_PACKET_REF.to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            "surface:learning_mode.left_rail".to_owned(),
            "surface:learning_mode.header".to_owned(),
            "surface:learning_mode.right_inspector".to_owned(),
            "surface:learning_mode.digest".to_owned(),
            "support_export:learning_mode_beta".to_owned(),
        ],
        downgrade_policy: LearningModeDowngradePolicy {
            policy_id: "downgrade-policy:learning-mode-beta:v1".to_owned(),
            independent_from_onboarding_core: true,
            stale_evidence_can_suppress_learning: true,
            downgrade_states: vec![
                "current".to_owned(),
                "cached_disclosed".to_owned(),
                "stale_disclosed".to_owned(),
                "graph_unavailable_placeholder".to_owned(),
                "not_installed_placeholder".to_owned(),
            ],
        },
        tour_packages: packages,
        exercise_rails: vec![GuidedExerciseRail {
            rail_id: "rail:safe-start.import-profile-preview".to_owned(),
            package_ref: "tour-pack:aureline.safe-start.beta".to_owned(),
            tour_ref: "tour:aureline.safe-start.command-backed".to_owned(),
            current_step_ref: "step:safe-start.import-profile-preview".to_owned(),
            ordered_step_refs: vec![
                "step:safe-start.open-folder".to_owned(),
                "step:safe-start.read-docs-source".to_owned(),
                "step:safe-start.import-profile-preview".to_owned(),
            ],
            success_criteria_refs: vec![
                "success:import-profile.preview-sheet-opened".to_owned(),
                "success:import-profile.approval-fence-visible".to_owned(),
                "success:import-profile.rollback-semantics-visible".to_owned(),
            ],
            hint_reveal_state: HintRevealState {
                state_ref: "hint-state:safe-start.import-profile-preview".to_owned(),
                persistence_ref: "user-state:learning-mode:hints:safe-start.import-profile-preview"
                    .to_owned(),
                hint_state: "hinted".to_owned(),
                reveal_state: "not_revealed".to_owned(),
                rate_limit_ref: "rate-limit:learning-hint:safe-start.import-profile-preview"
                    .to_owned(),
                rate_limit_window_seconds: 900,
                max_reveals_per_window: 2,
                restart_safe: true,
                dismissible: true,
            },
            skip_action: read_only_action(
                "action:rail.safe-start.import.skip",
                "msg:learning.rail.import.skip",
                ROLE_SKIP,
                None,
            ),
            reset_action: read_only_action(
                "action:rail.safe-start.import.reset",
                "msg:learning.rail.import.reset",
                ROLE_RESET,
                Some("cmd:editor.undo"),
            ),
            sandbox_preference: "reversible_workspace_preview".to_owned(),
            reversible_preference: "rollback_or_editor_undo".to_owned(),
            mutation_guardrail_class: "approval_required".to_owned(),
            preview_required: true,
            approval_required: true,
            restart_safe: true,
        }],
        learning_profiles: vec![LearningModeProfile {
            profile_id: profile_ref.to_owned(),
            profile_scope: "user_profile:local_default".to_owned(),
            profile_state: "enabled".to_owned(),
            tip_intensity_class: "gentle_hint".to_owned(),
            jargon_level_class: "beginner".to_owned(),
            ai_explanation_posture: "explain_then_prepare_preview".to_owned(),
            mutation_guardrail_class: "approval_required".to_owned(),
            explain_before_act_default: true,
            authority_boundary_change_allowed: false,
            data_ownership_class: "user_owned_portable_profile".to_owned(),
            optional_sync_posture: "local_only_default_optional_sync".to_owned(),
            dismissals_state_ref: "user-state:learning-mode:dismissals:default".to_owned(),
            bookmarks_state_ref: "user-state:learning-mode:bookmarks:default".to_owned(),
            blocking_onboarding_allowed: false,
            controls: profile_controls(profile_ref),
        }],
        progress_snapshots: vec![seeded_progress_snapshot(profile_ref)],
        support_export_policy: LearningModeSupportExportPolicy {
            policy_id: "support-policy:learning-mode-beta:v1".to_owned(),
            record_active_package_versions: true,
            record_profile_state: true,
            record_progress_metadata: true,
            bounded_material_classes: vec![
                "package_id".to_owned(),
                "package_version_ref".to_owned(),
                "profile_state_class".to_owned(),
                "progress_state_class".to_owned(),
                "local_or_sync_posture".to_owned(),
                "exact_reopen_ref".to_owned(),
            ],
            omitted_material_classes: vec![
                "raw_step_body".to_owned(),
                "raw_package_body".to_owned(),
                "raw_profile_notes".to_owned(),
                "private_workspace_path".to_owned(),
                "account_identifier".to_owned(),
            ],
            raw_bodies_exported: false,
        },
        protected_proofs: vec![
            LearningModeProof {
                proof_id: "proof:learning-mode-beta:mutation-preview-path".to_owned(),
                fixture_ref: "fixtures/help/m3/guided_tours/mutation_preview_path.yaml".to_owned(),
                exercised_package_refs: vec!["tour-pack:aureline.safe-start.beta".to_owned()],
                exercised_step_refs: vec!["step:safe-start.import-profile-preview".to_owned()],
                exercised_states: vec![
                    "command_registry_route".to_owned(),
                    "preview_sheet_required".to_owned(),
                    "approval_path_required".to_owned(),
                    "rollback_semantics_required".to_owned(),
                ],
            },
            LearningModeProof {
                proof_id: "proof:learning-mode-beta:degraded-source-states".to_owned(),
                fixture_ref: "fixtures/help/m3/guided_tours/degraded_sources.yaml".to_owned(),
                exercised_package_refs: vec![
                    "tour-pack:aureline.cached-docs.preview".to_owned(),
                    "tour-pack:aureline.graph-map.placeholder".to_owned(),
                ],
                exercised_step_refs: vec![
                    "step:cached-docs.open-with-label".to_owned(),
                    "step:graph-map.placeholder".to_owned(),
                ],
                exercised_states: vec![
                    DEGRADATION_CACHED.to_owned(),
                    DEGRADATION_GRAPH_UNAVAILABLE.to_owned(),
                    "inactive_placeholder".to_owned(),
                ],
            },
            LearningModeProof {
                proof_id: "proof:learning-mode-beta:progress-user-owned".to_owned(),
                fixture_ref: "fixtures/help/m3/guided_tours/progress_user_owned.yaml".to_owned(),
                exercised_package_refs: vec!["tour-pack:aureline.safe-start.beta".to_owned()],
                exercised_step_refs: vec![
                    "step:safe-start.open-folder".to_owned(),
                    "step:safe-start.import-profile-preview".to_owned(),
                ],
                exercised_states: vec![
                    "local_only_default_optional_sync".to_owned(),
                    "no_repo_read_default".to_owned(),
                    "no_telemetry_read_default".to_owned(),
                    "raw_bodies_not_exported".to_owned(),
                ],
            },
        ],
    }
}

/// Returns the seeded beta learning-mode surface projection.
pub fn seeded_learning_mode_beta_surface_projection() -> LearningModeBetaSurfaceProjection {
    seeded_learning_mode_beta_manifest().surface_projection()
}

/// Returns the seeded beta learning-mode support export.
pub fn seeded_learning_mode_beta_support_export() -> LearningModeBetaSupportExport {
    seeded_learning_mode_beta_manifest()
        .support_export("support-export:learning-mode-beta:001", GENERATED_AT)
}

/// Validates all seeded beta learning-mode records.
pub fn validate_seeded_learning_mode_beta(
    registry: &CommandRegistry,
) -> Result<(), Vec<LearningModeBetaFinding>> {
    let manifest = seeded_learning_mode_beta_manifest();
    let export = manifest.support_export("support-export:learning-mode-beta:001", GENERATED_AT);
    let mut findings = Vec::new();
    if let Err(mut manifest_findings) = manifest.validate_against_registry(registry) {
        findings.append(&mut manifest_findings);
    }
    if let Err(mut export_findings) = export.validate_against_manifest(&manifest) {
        findings.append(&mut export_findings);
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn seeded_progress_snapshot(profile_ref: &str) -> LearningModeProgressSnapshot {
    LearningModeProgressSnapshot {
        record_kind: LEARNING_MODE_PROGRESS_SNAPSHOT_RECORD_KIND.to_owned(),
        schema_version: LEARNING_MODE_BETA_SCHEMA_VERSION,
        snapshot_id: "snapshot:learning-mode:default-individual:2026.05.17".to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        profile_ref: profile_ref.to_owned(),
        package_refs: vec![
            "tour-pack:aureline.safe-start.beta".to_owned(),
            "tour-pack:aureline.cached-docs.preview".to_owned(),
            "tour-pack:aureline.graph-map.placeholder".to_owned(),
        ],
        progress_entries: vec![
            progress_entry(
                "progress:safe-start.open-folder.completed",
                "tour-pack:aureline.safe-start.beta",
                "tour:aureline.safe-start.command-backed",
                "step:safe-start.open-folder",
                PROGRESS_COMPLETED,
                "resume:step:safe-start.read-docs-source",
                "portable_profile_metadata",
            ),
            progress_entry(
                "progress:safe-start.import-profile.resumed",
                "tour-pack:aureline.safe-start.beta",
                "tour:aureline.safe-start.command-backed",
                "step:safe-start.import-profile-preview",
                PROGRESS_RESUMED,
                "resume:step:safe-start.import-profile-preview",
                "support_bundle_redacted",
            ),
            progress_entry(
                "progress:cached-docs.deferred",
                "tour-pack:aureline.cached-docs.preview",
                "tour:aureline.cached-docs.fallback",
                "step:cached-docs.open-with-label",
                PROGRESS_DEFERRED,
                "resume:step:cached-docs.open-with-label",
                "support_bundle_redacted",
            ),
            progress_entry(
                "progress:graph-map.dismissed",
                "tour-pack:aureline.graph-map.placeholder",
                "tour:aureline.graph-map.placeholder",
                "step:graph-map.placeholder",
                PROGRESS_DISMISSED,
                "resume:step:graph-map.placeholder",
                "portable_profile_metadata",
            ),
            progress_entry(
                "progress:cached-docs.skipped",
                "tour-pack:aureline.cached-docs.preview",
                "tour:aureline.cached-docs.fallback",
                "step:cached-docs.open-with-label",
                PROGRESS_SKIPPED,
                "resume:learning-digest",
                "local_only",
            ),
        ],
        hint_reveal_entries: vec![LearningModeHintRevealEntry {
            entry_id: "hint-progress:safe-start.import-profile-preview".to_owned(),
            state_ref: "hint-state:safe-start.import-profile-preview".to_owned(),
            hint_state: "hinted".to_owned(),
            reveal_state: "not_revealed".to_owned(),
            rate_limit_reset_ref: "rate-limit-reset:learning-hint:safe-start.import-profile-preview:2026.05.17T20:45:00Z".to_owned(),
            persisted_across_restart: true,
            last_visible_at: GENERATED_AT.to_owned(),
        }],
        export_posture: LearningModeProgressExportPosture {
            local_only_default: true,
            optional_sync_supported: true,
            sync_requires_user_action: true,
            bounded_export_fields: vec![
                "snapshot_id".to_owned(),
                "profile_ref".to_owned(),
                "package_ref".to_owned(),
                "step_ref".to_owned(),
                "progress_state_class".to_owned(),
                "local_or_sync_posture".to_owned(),
                "exact_reopen_ref".to_owned(),
            ],
            raw_step_body_exported: false,
            raw_pack_body_exported: false,
            user_can_reset: true,
            user_can_export_metadata: true,
        },
        support_projection: LearningModeProgressSupportProjection {
            projection_id: "support-projection:learning-mode-progress:default".to_owned(),
            row_refs: vec![
                "support-row:learning-progress:progress:safe-start.open-folder.completed"
                    .to_owned(),
                "support-row:learning-progress:progress:safe-start.import-profile.resumed"
                    .to_owned(),
            ],
            raw_profile_body_exported: false,
        },
    }
}

fn progress_entry(
    entry_id: &str,
    package_ref: &str,
    tour_ref: &str,
    step_ref: &str,
    progress_state_class: &str,
    resume_ref: &str,
    export_scope: &str,
) -> LearningModeProgressEntry {
    LearningModeProgressEntry {
        entry_id: entry_id.to_owned(),
        package_ref: package_ref.to_owned(),
        tour_ref: tour_ref.to_owned(),
        step_ref: step_ref.to_owned(),
        progress_state_class: progress_state_class.to_owned(),
        resume_ref: resume_ref.to_owned(),
        local_or_sync_posture: "local_only_default_optional_sync".to_owned(),
        export_scope: export_scope.to_owned(),
        repo_pack_read_default: false,
        collaborator_read_default: false,
        telemetry_read_default: false,
        inspectable_by_user: true,
        exact_reopen_ref: format!("reopen:{step_ref}@{resume_ref}"),
        updated_at: GENERATED_AT.to_owned(),
    }
}

fn profile_controls(profile_ref: &str) -> Vec<LearningModeControl> {
    [
        (CONTROL_ENABLE, "disabled->enabled"),
        (CONTROL_PAUSE, "enabled->paused"),
        (CONTROL_SNOOZE, "enabled->snoozed"),
        (CONTROL_RESET, "any->reset_requested"),
        (CONTROL_RESUME, "paused_or_snoozed->enabled"),
    ]
    .into_iter()
    .map(|(control_class, state_transition)| LearningModeControl {
        control_id: format!("control:{profile_ref}:{control_class}"),
        control_class: control_class.to_owned(),
        state_transition: state_transition.to_owned(),
        local_state_write: true,
        reversible_from_learning_digest: true,
        silent_write_allowed: false,
        user_visible: true,
    })
    .collect()
}

fn read_only_action(
    action_id: &str,
    label_ref: &str,
    verb_class: &str,
    command_id: Option<&str>,
) -> LearningAction {
    LearningAction {
        action_id: action_id.to_owned(),
        label_ref: label_ref.to_owned(),
        verb_class: verb_class.to_owned(),
        command_id: command_id.map(str::to_owned),
        command_metadata_source: command_id
            .map(|_| COMMAND_REGISTRY)
            .unwrap_or(LOCAL_PROFILE_STATE)
            .to_owned(),
        action_safety_class: ACTION_READ_ONLY.to_owned(),
        preview_sheet_ref: None,
        approval_path_ref: None,
        rollback_semantics_ref: None,
        evidence_packet_rule_ref: None,
        explain_and_apply_separate: true,
        mutates_workspace: false,
    }
}

fn mutation_action(
    action_id: &str,
    label_ref: &str,
    command_id: &str,
    preview_sheet_ref: &str,
    approval_path_ref: &str,
    rollback_semantics_ref: &str,
    evidence_packet_rule_ref: &str,
) -> LearningAction {
    LearningAction {
        action_id: action_id.to_owned(),
        label_ref: label_ref.to_owned(),
        verb_class: ROLE_PREPARE_PREVIEW.to_owned(),
        command_id: Some(command_id.to_owned()),
        command_metadata_source: COMMAND_REGISTRY.to_owned(),
        action_safety_class: ACTION_MUTATION_REQUIRES_APPROVAL.to_owned(),
        preview_sheet_ref: Some(preview_sheet_ref.to_owned()),
        approval_path_ref: Some(approval_path_ref.to_owned()),
        rollback_semantics_ref: Some(rollback_semantics_ref.to_owned()),
        evidence_packet_rule_ref: Some(evidence_packet_rule_ref.to_owned()),
        explain_and_apply_separate: true,
        mutates_workspace: true,
    }
}

fn blocked_action(action_id: &str, label_ref: &str) -> LearningAction {
    LearningAction {
        action_id: action_id.to_owned(),
        label_ref: label_ref.to_owned(),
        verb_class: ROLE_UNAVAILABLE.to_owned(),
        command_id: None,
        command_metadata_source: LOCAL_PROFILE_STATE.to_owned(),
        action_safety_class: ACTION_BLOCKED_UNAVAILABLE.to_owned(),
        preview_sheet_ref: None,
        approval_path_ref: None,
        rollback_semantics_ref: None,
        evidence_packet_rule_ref: None,
        explain_and_apply_separate: true,
        mutates_workspace: false,
    }
}

fn target_command(command_id: &str) -> LearningTargetRef {
    LearningTargetRef {
        target_kind: "command_id".to_owned(),
        command_id: Some(command_id.to_owned()),
        file_object_id: None,
        symbol_object_id: None,
        docs_node_id: None,
        graph_node_id: None,
        surface_object_id: None,
    }
}

fn target_docs(docs_node_id: &str) -> LearningTargetRef {
    LearningTargetRef {
        target_kind: "docs_node_id".to_owned(),
        command_id: None,
        file_object_id: None,
        symbol_object_id: None,
        docs_node_id: Some(docs_node_id.to_owned()),
        graph_node_id: None,
        surface_object_id: None,
    }
}

fn target_file(file_object_id: &str) -> LearningTargetRef {
    LearningTargetRef {
        target_kind: "file_object_id".to_owned(),
        command_id: None,
        file_object_id: Some(file_object_id.to_owned()),
        symbol_object_id: None,
        docs_node_id: None,
        graph_node_id: None,
        surface_object_id: None,
    }
}

fn target_graph(graph_node_id: &str) -> LearningTargetRef {
    LearningTargetRef {
        target_kind: "graph_node_id".to_owned(),
        command_id: None,
        file_object_id: None,
        symbol_object_id: None,
        docs_node_id: None,
        graph_node_id: Some(graph_node_id.to_owned()),
        surface_object_id: None,
    }
}

fn source_claim(
    claim_id: &str,
    claim_class: &str,
    source_refs: Vec<&str>,
    confidence_class: &str,
) -> LearningSourceClaim {
    LearningSourceClaim {
        claim_id: claim_id.to_owned(),
        claim_class: claim_class.to_owned(),
        source_refs: source_refs.into_iter().map(str::to_owned).collect(),
        confidence_class: confidence_class.to_owned(),
    }
}

struct LearningModeBetaValidator<'a> {
    manifest: &'a LearningModeBetaManifest,
    registry: &'a CommandRegistry,
    findings: Vec<LearningModeBetaFinding>,
}

impl<'a> LearningModeBetaValidator<'a> {
    fn new(manifest: &'a LearningModeBetaManifest, registry: &'a CommandRegistry) -> Self {
        Self {
            manifest,
            registry,
            findings: Vec::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        self.validate_packages();
        self.validate_steps();
        self.validate_rails();
        self.validate_profiles();
        self.validate_progress_snapshots();
        self.validate_support_policy();
        self.validate_proofs();
    }

    fn finish(self) -> Result<(), Vec<LearningModeBetaFinding>> {
        if self.findings.is_empty() {
            Ok(())
        } else {
            Err(self.findings)
        }
    }

    fn validate_header(&mut self) {
        if self.manifest.record_kind != LEARNING_MODE_BETA_MANIFEST_RECORD_KIND {
            self.push(
                &self.manifest.manifest_id,
                "learning_mode.manifest.record_kind",
                "manifest record_kind is unsupported",
            );
        }
        if self.manifest.schema_version != LEARNING_MODE_BETA_SCHEMA_VERSION {
            self.push(
                &self.manifest.manifest_id,
                "learning_mode.manifest.schema_version",
                "manifest schema version is unsupported",
            );
        }
        if self.manifest.release_channel != "beta" || self.manifest.runtime_consumer_refs.is_empty()
        {
            self.push(
                &self.manifest.manifest_id,
                "learning_mode.manifest.channel_or_consumer",
                "manifest must be a beta record with runtime consumers",
            );
        }
        if !self
            .manifest
            .downgrade_policy
            .independent_from_onboarding_core
            || !self
                .manifest
                .downgrade_policy
                .stale_evidence_can_suppress_learning
        {
            self.push(
                &self.manifest.downgrade_policy.policy_id,
                "learning_mode.downgrade.independent",
                "learning surfaces must downgrade independently from onboarding core",
            );
        }
    }

    fn validate_packages(&mut self) {
        let mut ids = BTreeSet::new();
        let mut availability = BTreeSet::new();
        for package in self.manifest.tour_packages.clone() {
            if !ids.insert(package.package_id.clone()) {
                self.push(
                    &package.package_id,
                    "learning_mode.package.duplicate",
                    "duplicate package id",
                );
            }
            availability.insert(package.availability_state.clone());
            if !matches!(package.release_label.as_str(), "beta" | "preview") {
                self.push(
                    &package.package_id,
                    "learning_mode.package.release_label",
                    "package must be visibly labeled beta or preview",
                );
            }
            if !package.downgrade_allowed || package.independent_downgrade_group.is_empty() {
                self.push(
                    &package.package_id,
                    "learning_mode.package.downgrade",
                    "package must support independent downgrade",
                );
            }
            if package.citation_refs.is_empty()
                || package.docs_pack_ref.is_empty()
                || package.command_graph_ref.is_empty()
            {
                self.push(
                    &package.package_id,
                    "learning_mode.package.citation_or_graph",
                    "package must preserve citations, docs pack, and command graph refs",
                );
            }
            if package.availability_state == AVAILABILITY_GRAPH_UNAVAILABLE
                && package.graph_epoch_ref.is_some()
            {
                self.push(
                    &package.package_id,
                    "learning_mode.package.graph_unavailable_epoch",
                    "graph-unavailable packages must not claim a graph epoch",
                );
            }
            if package.tours.is_empty() || package.steps.is_empty() {
                self.push(
                    &package.package_id,
                    "learning_mode.package.empty",
                    "package must include at least one tour and step",
                );
            }
            for tour in &package.tours {
                if tour.package_ref != package.package_id {
                    self.push(
                        &tour.tour_id,
                        "learning_mode.tour.package_ref",
                        "tour package ref must match owning package",
                    );
                }
                if tour.required_before_first_edit {
                    self.push(
                        &tour.tour_id,
                        "learning_mode.tour.mandatory_first_edit",
                        "learning tours must not block first edit",
                    );
                }
                if tour.ordered_step_refs.is_empty()
                    || tour.citation_refs.is_empty()
                    || tour.docs_node_refs.is_empty()
                    || tour.restart_resume_ref.is_empty()
                {
                    self.push(
                        &tour.tour_id,
                        "learning_mode.tour.refs_missing",
                        "tour must preserve ordered steps, docs nodes, citations, and resume refs",
                    );
                }
            }
        }

        for required in [
            AVAILABILITY_INSTALLED,
            AVAILABILITY_CACHED,
            AVAILABILITY_GRAPH_UNAVAILABLE,
        ] {
            if !availability.contains(required) {
                self.push(
                    &self.manifest.manifest_id,
                    "learning_mode.package.availability_coverage",
                    format!("manifest must exercise availability state {required}"),
                );
            }
        }
    }

    fn validate_steps(&mut self) {
        let package_ids = self.manifest.package_ids();
        let tour_ids = self.manifest.tour_ids();
        let profile_ids = self.manifest.profile_ids();
        let mut ids = BTreeSet::new();
        for package in self.manifest.tour_packages.clone() {
            let tour_step_refs = package
                .tours
                .iter()
                .flat_map(|tour| tour.ordered_step_refs.iter().cloned())
                .collect::<BTreeSet<_>>();
            for step in package.steps {
                if !ids.insert(step.step_id.clone()) {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.duplicate",
                        "duplicate step id",
                    );
                }
                if !package_ids.contains(step.package_ref.as_str())
                    || !tour_ids.contains(step.tour_ref.as_str())
                    || !profile_ids.contains(step.profile_ref.as_str())
                {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.unknown_ref",
                        "step references unknown package, tour, or profile",
                    );
                }
                if !tour_step_refs.contains(&step.step_id) {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.not_ordered",
                        "step must appear in its tour order",
                    );
                }
                if step.stable_targets.is_empty()
                    || step
                        .stable_targets
                        .iter()
                        .any(|target| !target.is_stable_anchor())
                {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.stable_target",
                        "step must target stable object ids rather than coordinates",
                    );
                }
                for target in &step.stable_targets {
                    if let Some(command_id) = &target.command_id {
                        self.validate_command(
                            command_id,
                            &step.step_id,
                            "learning_mode.step.command_target",
                        );
                    }
                }
                if step.source_claims.is_empty()
                    || step
                        .source_claims
                        .iter()
                        .any(|claim| claim.source_refs.is_empty())
                    || step.citation_refs.is_empty()
                {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.claims_citations",
                        "steps that claim product or repo truth must cite stable sources",
                    );
                }
                if step.success_criteria.is_empty() || step.hint_reveal_state_ref.is_empty() {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.success_or_hint",
                        "step must expose success criteria and hint/reveal state",
                    );
                }
                let has_explain = step
                    .actions
                    .iter()
                    .any(|action| action.verb_class == ROLE_EXPLAIN);
                for action in &step.actions {
                    self.validate_action(action, &step.step_id);
                }
                if step.actions.iter().any(|action| action.mutates_workspace) && !has_explain {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.mutation_without_explain",
                        "mutation-capable steps must expose a separate explain action",
                    );
                }
                if package.availability_state == AVAILABILITY_GRAPH_UNAVAILABLE && step.active {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.degraded_active",
                        "graph-unavailable placeholder steps must not render as active",
                    );
                }
                if package.degradation_state != step.degradation_state {
                    self.push(
                        &step.step_id,
                        "learning_mode.step.degradation_mismatch",
                        "step degradation must match owning package degradation",
                    );
                }
            }
        }
    }

    fn validate_action(&mut self, action: &LearningAction, owner_ref: &str) {
        if !action.explain_and_apply_separate {
            self.push(
                owner_ref,
                "learning_mode.action.explain_apply_split",
                format!(
                    "action {} must keep explanation and apply separate",
                    action.action_id
                ),
            );
        }
        if action.command_metadata_source == COMMAND_REGISTRY {
            if let Some(command_id) = action.command_id.as_deref() {
                self.validate_command(command_id, owner_ref, "learning_mode.action.command_ref");
            } else {
                self.push(
                    owner_ref,
                    "learning_mode.action.command_missing",
                    format!(
                        "action {} declares command registry metadata without a command id",
                        action.action_id
                    ),
                );
            }
        }
        if action.mutates_workspace {
            if action.action_safety_class != ACTION_MUTATION_REQUIRES_APPROVAL
                || action.command_metadata_source != COMMAND_REGISTRY
                || option_is_empty(&action.command_id)
                || option_is_empty(&action.preview_sheet_ref)
                || option_is_empty(&action.approval_path_ref)
                || option_is_empty(&action.rollback_semantics_ref)
                || option_is_empty(&action.evidence_packet_rule_ref)
            {
                self.push(
                    owner_ref,
                    "learning_mode.action.mutation_guardrails",
                    format!(
                        "mutating action {} must use registry command, preview, approval, rollback, and evidence refs",
                        action.action_id
                    ),
                );
            }
            if action.verb_class == ROLE_EXPLAIN {
                self.push(
                    owner_ref,
                    "learning_mode.action.explain_mutates",
                    "explain actions must never mutate workspace data",
                );
            }
        }
    }

    fn validate_rails(&mut self) {
        let package_ids = self.manifest.package_ids();
        let tour_ids = self.manifest.tour_ids();
        let step_ids = self.manifest.step_ids();
        let mut ids = BTreeSet::new();
        for rail in self.manifest.exercise_rails.clone() {
            if !ids.insert(rail.rail_id.clone()) {
                self.push(
                    &rail.rail_id,
                    "learning_mode.rail.duplicate",
                    "duplicate rail id",
                );
            }
            if !package_ids.contains(rail.package_ref.as_str())
                || !tour_ids.contains(rail.tour_ref.as_str())
                || !step_ids.contains(rail.current_step_ref.as_str())
            {
                self.push(
                    &rail.rail_id,
                    "learning_mode.rail.unknown_ref",
                    "rail references unknown package, tour, or step",
                );
            }
            for step_ref in &rail.ordered_step_refs {
                if !step_ids.contains(step_ref.as_str()) {
                    self.push(
                        &rail.rail_id,
                        "learning_mode.rail.ordered_step_ref",
                        format!("rail references unknown step {step_ref}"),
                    );
                }
            }
            if rail.success_criteria_refs.is_empty()
                || !rail.preview_required
                || !rail.approval_required
                || !rail.restart_safe
            {
                self.push(
                    &rail.rail_id,
                    "learning_mode.rail.guardrail_missing",
                    "rail must expose success criteria, preview, approval, and restart-safe state",
                );
            }
            if !rail.hint_reveal_state.restart_safe
                || !rail.hint_reveal_state.dismissible
                || rail.hint_reveal_state.rate_limit_window_seconds == 0
                || rail.hint_reveal_state.max_reveals_per_window == 0
                || rail.hint_reveal_state.persistence_ref.is_empty()
            {
                self.push(
                    &rail.rail_id,
                    "learning_mode.rail.hint_reveal_state",
                    "hint/reveal state must be restart-safe, dismissible, persisted, and rate-limited",
                );
            }
            self.validate_action(&rail.skip_action, &rail.rail_id);
            self.validate_action(&rail.reset_action, &rail.rail_id);
        }
    }

    fn validate_profiles(&mut self) {
        let mut ids = BTreeSet::new();
        for profile in self.manifest.learning_profiles.clone() {
            if !ids.insert(profile.profile_id.clone()) {
                self.push(
                    &profile.profile_id,
                    "learning_mode.profile.duplicate",
                    "duplicate profile id",
                );
            }
            if !profile.explain_before_act_default
                || profile.authority_boundary_change_allowed
                || profile.blocking_onboarding_allowed
                || profile.data_ownership_class != "user_owned_portable_profile"
            {
                self.push(
                    &profile.profile_id,
                    "learning_mode.profile.authority_or_blocking",
                    "profile must explain before act, stay user-owned, and not change authority or block onboarding",
                );
            }
            let controls = profile
                .controls
                .iter()
                .map(|control| control.control_class.as_str())
                .collect::<BTreeSet<_>>();
            for required in [
                CONTROL_ENABLE,
                CONTROL_PAUSE,
                CONTROL_SNOOZE,
                CONTROL_RESET,
                CONTROL_RESUME,
            ] {
                if !controls.contains(required) {
                    self.push(
                        &profile.profile_id,
                        "learning_mode.profile.control_missing",
                        format!("profile must expose {required} control"),
                    );
                }
            }
            for control in &profile.controls {
                if !control.user_visible
                    || !control.reversible_from_learning_digest
                    || control.silent_write_allowed
                    || !control.local_state_write
                {
                    self.push(
                        &control.control_id,
                        "learning_mode.profile.control_visibility",
                        "profile controls must be visible, reversible, and never silently write state",
                    );
                }
            }
        }
    }

    fn validate_progress_snapshots(&mut self) {
        let package_ids = self.manifest.package_ids();
        let tour_ids = self.manifest.tour_ids();
        let step_ids = self.manifest.step_ids();
        let profile_ids = self.manifest.profile_ids();
        let mut observed_states = BTreeSet::new();
        for snapshot in self.manifest.progress_snapshots.clone() {
            if snapshot.record_kind != LEARNING_MODE_PROGRESS_SNAPSHOT_RECORD_KIND {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_mode.progress.record_kind",
                    "progress snapshot record_kind is unsupported",
                );
            }
            if snapshot.schema_version != LEARNING_MODE_BETA_SCHEMA_VERSION
                || !profile_ids.contains(snapshot.profile_ref.as_str())
            {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_mode.progress.schema_or_profile",
                    "progress snapshot must use the current schema and known profile",
                );
            }
            if !snapshot.export_posture.local_only_default
                || !snapshot.export_posture.sync_requires_user_action
                || snapshot.export_posture.raw_step_body_exported
                || snapshot.export_posture.raw_pack_body_exported
                || !snapshot.export_posture.user_can_reset
                || !snapshot.export_posture.user_can_export_metadata
                || snapshot.support_projection.raw_profile_body_exported
            {
                self.push(
                    &snapshot.snapshot_id,
                    "learning_mode.progress.export_posture",
                    "progress export must be user-owned, explicit, redacted, resettable, and inspectable",
                );
            }
            for entry in &snapshot.progress_entries {
                observed_states.insert(entry.progress_state_class.clone());
                if !package_ids.contains(entry.package_ref.as_str())
                    || !tour_ids.contains(entry.tour_ref.as_str())
                    || !step_ids.contains(entry.step_ref.as_str())
                {
                    self.push(
                        &entry.entry_id,
                        "learning_mode.progress.unknown_ref",
                        "progress entry references unknown package, tour, or step",
                    );
                }
                if entry.repo_pack_read_default
                    || entry.collaborator_read_default
                    || entry.telemetry_read_default
                    || !entry.inspectable_by_user
                    || entry.local_or_sync_posture.is_empty()
                    || entry.exact_reopen_ref.is_empty()
                {
                    self.push(
                        &entry.entry_id,
                        "learning_mode.progress.hidden_read",
                        "progress entries must be inspectable and must not expose hidden repo, collaborator, or telemetry reads",
                    );
                }
            }
            for entry in &snapshot.hint_reveal_entries {
                if !entry.persisted_across_restart
                    || entry.state_ref.is_empty()
                    || entry.rate_limit_reset_ref.is_empty()
                {
                    self.push(
                        &entry.entry_id,
                        "learning_mode.progress.hint_reveal_restart",
                        "hint/reveal progress must persist across restart and preserve rate-limit state",
                    );
                }
            }
        }
        for required in [
            PROGRESS_COMPLETED,
            PROGRESS_RESUMED,
            PROGRESS_DEFERRED,
            PROGRESS_DISMISSED,
            PROGRESS_SKIPPED,
        ] {
            if !observed_states.contains(required) {
                self.push(
                    &self.manifest.manifest_id,
                    "learning_mode.progress.state_coverage",
                    format!("progress snapshots must include {required} state"),
                );
            }
        }
    }

    fn validate_support_policy(&mut self) {
        let policy = self.manifest.support_export_policy.clone();
        if !policy.record_active_package_versions
            || !policy.record_profile_state
            || !policy.record_progress_metadata
            || policy.raw_bodies_exported
            || policy.bounded_material_classes.is_empty()
            || policy.omitted_material_classes.is_empty()
        {
            self.push(
                &policy.policy_id,
                "learning_mode.support.policy",
                "support policy must record bounded metadata and omit raw bodies",
            );
        }
    }

    fn validate_proofs(&mut self) {
        let package_ids = self.manifest.package_ids();
        let step_ids = self.manifest.step_ids();
        for proof in self.manifest.protected_proofs.clone() {
            if proof.exercised_states.is_empty() {
                self.push(
                    &proof.proof_id,
                    "learning_mode.proof.states",
                    "proof must name exercised states",
                );
            }
            for package_ref in &proof.exercised_package_refs {
                if !package_ids.contains(package_ref.as_str()) {
                    self.push(
                        &proof.proof_id,
                        "learning_mode.proof.unknown_package",
                        format!("proof references unknown package {package_ref}"),
                    );
                }
            }
            for step_ref in &proof.exercised_step_refs {
                if !step_ids.contains(step_ref.as_str()) {
                    self.push(
                        &proof.proof_id,
                        "learning_mode.proof.unknown_step",
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

    fn push(
        &mut self,
        row_ref: impl Into<String>,
        check_id: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.findings
            .push(LearningModeBetaFinding::new(row_ref, check_id, message));
    }
}

impl LearningModeBetaManifest {
    fn package_ids(&self) -> BTreeSet<String> {
        self.tour_packages
            .iter()
            .map(|package| package.package_id.clone())
            .collect()
    }

    fn tour_ids(&self) -> BTreeSet<String> {
        self.tour_packages
            .iter()
            .flat_map(|package| package.tours.iter().map(|tour| tour.tour_id.clone()))
            .collect()
    }

    fn step_ids(&self) -> BTreeSet<String> {
        self.tour_packages
            .iter()
            .flat_map(|package| package.steps.iter().map(|step| step.step_id.clone()))
            .collect()
    }

    fn profile_ids(&self) -> BTreeSet<String> {
        self.learning_profiles
            .iter()
            .map(|profile| profile.profile_id.clone())
            .collect()
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
