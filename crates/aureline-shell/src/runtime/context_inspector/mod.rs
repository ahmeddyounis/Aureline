//! Execution-context inspector shared by terminal, task, and debug-prep
//! seed surfaces.
//!
//! The inspector is the protected-row surface a user opens when they need to
//! answer "why did this launch here?" without scanning logs. It is a thin
//! projection over [`aureline_runtime::ExecutionContext`]: every value comes
//! verbatim from the resolved record, every winning input source is quoted
//! from the [`aureline_runtime::Provenance`] decision log, and every missing
//! or prototype-limited field is labeled honestly instead of silently
//! omitted.
//!
//! ## Why one inspector, not three
//!
//! The terminal pane, the task seed, and the debug-prep seed all need the
//! same answer when a user asks "what target, cwd, env, and toolchain is
//! this lane using?" — only the *opening surface* differs. Forking three
//! inspector copies would let one lane drift its vocabulary while another
//! lags. This module renders one snapshot type that carries an
//! [`InspectorOpeningSurface`] tag so the chrome can pick the appropriate
//! framing copy without re-deriving the rows.
//!
//! ## Failure-drill posture
//!
//! When the resolved context carries a non-empty
//! [`aureline_runtime::ExecutionContext::degraded_fields`] list, the
//! inspector surfaces a typed honesty-marker row per degraded field rather
//! than rendering a stale "all green" snapshot. When a partially resolved
//! context omits a working directory the row carries the
//! [`InspectorMissingFieldReason::ResolverUnsettled`] marker and a
//! human-readable label. The fixtures under
//! [`/fixtures/runtime/context_inspector_cases/*.json`] exercise the drill
//! against the canonical snapshot shape every consumer renders.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    ActorClass, CacheDisposition, CapsuleDriftState, ConfidenceLevel, DegradedFieldReason,
    DegradedFieldRecord, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextEffectClass,
    ExecutionContextExplanation, ExecutionContextReasonCode, ExecutionContextReasonSource,
    IdentityMode, InvocationSubject, MixedVersionDrift, MixedVersionDriftState,
    NodeToolchainDetection, NodeToolchainProvenanceDisposition, NodeToolchainResolutionState,
    NodeToolchainSourceKind, NodeToolchainSubject, PolicyAndTrust, PrebuildInvalidationReason,
    PrebuildMetadata, PrebuildReuseState, Provenance, PythonEnvironmentDetection,
    PythonEnvironmentProvenanceDisposition, PythonEnvironmentResolutionState,
    PythonEnvironmentSourceKind, PythonEnvironmentSubject, ReachabilityState,
    ResolverInputDecision, ResolverInputField, ResolverInputSource, ScopeClass, SurfaceClass,
    TargetClass, TargetConfidenceReason, TargetIdentity, ToolchainClass, ToolchainIdentity,
    TrustState,
};

/// Stable record-kind tag carried in serialized inspector snapshots.
pub const EXECUTION_CONTEXT_INSPECTOR_RECORD_KIND: &str = "execution_context_inspector_snapshot";

/// Schema version for the [`ExecutionContextInspectorSnapshot`] payload shape.
pub const EXECUTION_CONTEXT_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Which seed surface opened the inspector. The chrome uses this tag to
/// frame the snapshot; the rows themselves do not change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorOpeningSurface {
    /// Opened from the bottom-panel terminal pane.
    Terminal,
    /// Opened from the task seed.
    Task,
    /// Opened from the debug-prep seed.
    DebugPrep,
    /// Opened from a provider/auth or support flow that already holds the
    /// resolved context.
    SupportFlow,
}

impl InspectorOpeningSurface {
    /// Stable string token recorded on the snapshot.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Task => "task",
            Self::DebugPrep => "debug_prep",
            Self::SupportFlow => "support_flow",
        }
    }

    /// Map a resolved [`SurfaceClass`] onto the matching opening surface.
    /// Surfaces outside the seed lanes settle on
    /// [`InspectorOpeningSurface::SupportFlow`] so the inspector still has a
    /// truthful default rather than panicking on an unmapped variant.
    pub const fn from_surface_class(surface: SurfaceClass) -> Self {
        match surface {
            SurfaceClass::Terminal => Self::Terminal,
            SurfaceClass::Task => Self::Task,
            SurfaceClass::Debug => Self::DebugPrep,
            _ => Self::SupportFlow,
        }
    }
}

/// Stable section ids the inspector renders. The order is the canonical
/// reading order: subject -> target -> toolchain -> capsule -> policy/trust
/// -> scope/cache -> provenance -> degraded markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorSectionId {
    InvocationSubject,
    TargetIdentity,
    ToolchainIdentity,
    PythonEnvironmentDetection,
    NodeToolchainDetection,
    EnvironmentCapsule,
    PolicyAndTrust,
    Scope,
    Cache,
    ResolverExplanations,
    Provenance,
    DegradedFields,
}

impl InspectorSectionId {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvocationSubject => "invocation_subject",
            Self::TargetIdentity => "target_identity",
            Self::ToolchainIdentity => "toolchain_identity",
            Self::PythonEnvironmentDetection => "python_environment_detection",
            Self::NodeToolchainDetection => "node_toolchain_detection",
            Self::EnvironmentCapsule => "environment_capsule",
            Self::PolicyAndTrust => "policy_and_trust",
            Self::Scope => "scope",
            Self::Cache => "cache",
            Self::ResolverExplanations => "resolver_explanations",
            Self::Provenance => "provenance",
            Self::DegradedFields => "degraded_fields",
        }
    }

    /// Human-readable section heading.
    pub const fn heading(self) -> &'static str {
        match self {
            Self::InvocationSubject => "Invocation",
            Self::TargetIdentity => "Target",
            Self::ToolchainIdentity => "Toolchain",
            Self::PythonEnvironmentDetection => "Python environment",
            Self::NodeToolchainDetection => "Node detector",
            Self::EnvironmentCapsule => "Environment capsule",
            Self::PolicyAndTrust => "Policy and trust",
            Self::Scope => "Scope",
            Self::Cache => "Cache reuse",
            Self::ResolverExplanations => "Resolver explanations",
            Self::Provenance => "Why this launch?",
            Self::DegradedFields => "Honesty markers",
        }
    }
}

/// Why a row's value is missing or labeled instead of quoted verbatim.
///
/// The inspector never silently omits a field. When the resolver did not
/// settle a value, or the seed object intentionally covers a subset of the
/// boundary schema, the row carries one of these reasons so the chrome can
/// render an honest label instead of an empty cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorMissingFieldReason {
    /// The resolver did not settle a value (e.g. no caller, surface, or
    /// workspace input contributed a working directory).
    ResolverUnsettled,
    /// The seed object intentionally covers a subset of the boundary
    /// schema; the field is reserved but not minted in the seed.
    PrototypeLimitedToSeed,
    /// The field exists upstream but the inspector seed does not yet
    /// project it; the chrome shows a placeholder rather than fabricating
    /// completeness.
    SeedPlaceholderAwaitingWiring,
}

impl InspectorMissingFieldReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolverUnsettled => "resolver_unsettled",
            Self::PrototypeLimitedToSeed => "prototype_limited_to_seed",
            Self::SeedPlaceholderAwaitingWiring => "seed_placeholder_awaiting_wiring",
        }
    }

    /// Human-readable label rendered in place of the missing value.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ResolverUnsettled => "Not settled by resolver",
            Self::PrototypeLimitedToSeed => "Reserved (seed limit)",
            Self::SeedPlaceholderAwaitingWiring => "Seed placeholder",
        }
    }
}

/// Stable inspector actions. These are the only addressable actions the
/// inspector exposes in the seed; consumers route them onto the chrome's
/// command surface rather than minting button-only labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorAction {
    /// Copy the structured execution context to the clipboard.
    CopyContext,
    /// Reveal the per-input precedence decisions in a focused row.
    ViewResolverDetails,
    /// Open the workspace target/profile settings related to this lane.
    OpenTargetSettings,
    /// Return to the surface that opened the inspector (terminal pane, task
    /// channel, debug-prep stub, or support flow).
    ReturnToInvokingSurface,
}

impl InspectorAction {
    /// Stable string token rendered on the action row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyContext => "copy_context",
            Self::ViewResolverDetails => "view_resolver_details",
            Self::OpenTargetSettings => "open_target_settings",
            Self::ReturnToInvokingSurface => "return_to_invoking_surface",
        }
    }

    /// Human-readable label for the action.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CopyContext => "Copy context",
            Self::ViewResolverDetails => "Why this launch?",
            Self::OpenTargetSettings => "Open target settings",
            Self::ReturnToInvokingSurface => "Return to surface",
        }
    }
}

/// One inspector row.
///
/// The row carries enough context for the chrome to render `label: value`
/// with an optional provenance source and an optional missing-field marker
/// without re-deriving truth from raw context fields. `value` is always
/// non-empty: when the underlying field is unsettled, the value is the
/// human-readable label of [`InspectorMissingFieldReason`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorRow {
    /// Stable row id within the section. Suffixed with the section id at the
    /// snapshot level so consumers can address rows across sections without
    /// collisions.
    pub row_id: String,
    /// Human-readable row label (e.g. `"Working directory"`).
    pub label: String,
    /// Resolved value the chrome quotes verbatim. When the field is
    /// unsettled, the resolver-fallback label, or seed-limited, this carries
    /// the [`InspectorMissingFieldReason::label`] string instead.
    pub value: String,
    /// Stable token form of the value when the row is value-bearing. Null
    /// when the row is purely descriptive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
    /// Resolver input source that won the precedence contest for this row,
    /// when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_source: Option<ResolverInputSource>,
    /// Other sources that contributed a different value but lost. Empty
    /// when there was no conflict or no precedence contest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicting_sources: Vec<ResolverInputSource>,
    /// Honesty marker when the value is missing or seed-limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_field_reason: Option<InspectorMissingFieldReason>,
    /// Degraded reason copied from the resolver record when the row mirrors
    /// a [`DegradedFieldRecord`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<DegradedFieldReason>,
}

/// One inspector section. A section is a fixed group of rows the chrome
/// renders together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSection {
    pub section_id: InspectorSectionId,
    pub heading: String,
    pub rows: Vec<InspectorRow>,
}

impl InspectorSection {
    fn new(section_id: InspectorSectionId, rows: Vec<InspectorRow>) -> Self {
        Self {
            section_id,
            heading: section_id.heading().to_owned(),
            rows,
        }
    }
}

/// One inspector action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorActionRow {
    pub action: InspectorAction,
    pub label: String,
}

impl InspectorActionRow {
    fn with_label(action: InspectorAction) -> Self {
        Self {
            action,
            label: action.label().to_owned(),
        }
    }
}

/// Inspector snapshot.
///
/// The snapshot is the canonical record the chrome renders, a support
/// export quotes, and a fixture replays. Every section is always present
/// — even when its rows reduce to honesty markers — so a degraded snapshot
/// is never silently smaller than a green snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextInspectorSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub opening_surface: InspectorOpeningSurface,
    pub execution_context_id: String,
    pub workspace_id: String,
    pub sections: Vec<InspectorSection>,
    pub actions: Vec<InspectorActionRow>,
    /// True when at least one section row carries a degraded reason or a
    /// missing-field marker. The chrome MUST render a visible honesty chip
    /// when this is true.
    pub honesty_marker_present: bool,
}

impl ExecutionContextInspectorSnapshot {
    /// Project a snapshot from a resolved [`ExecutionContext`].
    ///
    /// The opening surface defaults to the matching seed lane based on the
    /// context's [`SurfaceClass`]. Use [`Self::project_from_surface`] to
    /// override that mapping (e.g. when a support flow opens the inspector
    /// on a context resolved by a different lane).
    pub fn project(context: &ExecutionContext) -> Self {
        let opening_surface =
            InspectorOpeningSurface::from_surface_class(context.invocation_subject.surface);
        Self::project_from_surface(context, opening_surface)
    }

    /// Project a snapshot with an explicit opening-surface tag.
    pub fn project_from_surface(
        context: &ExecutionContext,
        opening_surface: InspectorOpeningSurface,
    ) -> Self {
        let mut sections = vec![
            project_invocation_section(&context.invocation_subject),
            project_target_section(&context.target_identity, &context.provenance),
            project_toolchain_section(&context.toolchain_identity, &context.provenance),
        ];
        if let Some(detection) = &context.python_environment_detection {
            sections.push(project_python_environment_section(detection));
        }
        if let Some(detection) = &context.node_toolchain_detection {
            sections.push(project_node_detection_section(detection));
        }
        sections.extend([
            project_capsule_section(&context.environment_capsule_ref),
            project_policy_section(&context.policy_and_trust),
            project_scope_section(context.workset_scope_class),
            project_cache_section(context.cache_disposition),
            project_resolver_explanations_section(context),
            project_provenance_section(&context.provenance),
            project_degraded_section(&context.degraded_fields),
        ]);
        let honesty_marker_present = sections
            .iter()
            .flat_map(|section| section.rows.iter())
            .any(|row| row.missing_field_reason.is_some() || row.degraded_reason.is_some());

        Self {
            record_kind: EXECUTION_CONTEXT_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_CONTEXT_INSPECTOR_SCHEMA_VERSION,
            opening_surface,
            execution_context_id: context.execution_context_id.clone(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            sections,
            actions: default_actions(),
            honesty_marker_present,
        }
    }

    /// Locate one section by id.
    pub fn section(&self, id: InspectorSectionId) -> Option<&InspectorSection> {
        self.sections
            .iter()
            .find(|section| section.section_id == id)
    }

    /// Iterator over the snapshot's stable actions.
    pub fn actions(&self) -> impl Iterator<Item = &InspectorActionRow> {
        self.actions.iter()
    }

    /// Render a deterministic plaintext block that downstream consumers
    /// (copy-context action, support exports, fixture replays) can quote
    /// without re-deriving the row vocabulary. The block is stable across
    /// runs for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Execution context inspector\n");
        out.push_str(&format!(
            "Opened from: {}\nExecution context: {}\nWorkspace: {}\n\n",
            self.opening_surface.as_str(),
            self.execution_context_id,
            self.workspace_id,
        ));
        for section in &self.sections {
            out.push_str(&format!("[{}]\n", section.heading));
            if section.rows.is_empty() {
                out.push_str("  (no rows)\n");
            }
            for row in &section.rows {
                out.push_str(&format!("  {}: {}", row.label, row.value));
                if let Some(source) = row.winning_source {
                    out.push_str(&format!("  (source: {})", source.as_str()));
                }
                if let Some(reason) = row.missing_field_reason {
                    out.push_str(&format!("  [{}]", reason.as_str()));
                }
                if let Some(reason) = row.degraded_reason {
                    out.push_str(&format!("  [degraded: {}]", reason.as_str()));
                }
                out.push('\n');
            }
            out.push('\n');
        }
        out
    }
}

fn default_actions() -> Vec<InspectorActionRow> {
    vec![
        InspectorActionRow::with_label(InspectorAction::CopyContext),
        InspectorActionRow::with_label(InspectorAction::ViewResolverDetails),
        InspectorActionRow::with_label(InspectorAction::OpenTargetSettings),
        InspectorActionRow::with_label(InspectorAction::ReturnToInvokingSurface),
    ]
}

fn project_invocation_section(subject: &InvocationSubject) -> InspectorSection {
    let mut rows = vec![
        value_row("command_id", "Command", subject.command_id.clone()),
        token_row(
            "surface",
            "Surface",
            surface_class_label(subject.surface).to_owned(),
            subject.surface.as_str(),
        ),
        token_row(
            "actor_class",
            "Actor",
            actor_label(subject.actor_class).to_owned(),
            subject.actor_class.as_str(),
        ),
        value_row("workspace_id", "Workspace id", subject.workspace_id.clone()),
    ];
    rows.push(match &subject.profile_id {
        Some(profile) => value_row("profile_id", "Profile", profile.clone()),
        None => missing_row(
            "profile_id",
            "Profile",
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
    });
    InspectorSection::new(InspectorSectionId::InvocationSubject, rows)
}

fn project_target_section(target: &TargetIdentity, provenance: &Provenance) -> InspectorSection {
    let target_decision = decision_for(provenance, ResolverInputField::TargetClass);
    let wd_decision = decision_for(provenance, ResolverInputField::WorkingDirectory);

    let mut target_class_row = token_row(
        "target_class",
        "Target class",
        target_class_label(target.target_class).to_owned(),
        target.target_class.as_str(),
    );
    if let Some(decision) = target_decision {
        target_class_row.winning_source = Some(decision.winning_source);
        target_class_row.conflicting_sources = decision.conflicting_sources.clone();
    }

    let mut rows = vec![
        target_class_row,
        value_row(
            "canonical_target_id",
            "Canonical target id",
            target.canonical_target_id.clone(),
        ),
    ];

    rows.push(match &target.working_directory {
        Some(cwd) => {
            let mut row = value_row("working_directory", "Working directory", cwd.clone());
            if let Some(decision) = wd_decision {
                row.winning_source = Some(decision.winning_source);
                row.conflicting_sources = decision.conflicting_sources.clone();
            }
            row
        }
        None => missing_row(
            "working_directory",
            "Working directory",
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
    });

    rows.push(token_row(
        "reachability_state",
        "Reachability",
        reachability_label(target.reachability_state).to_owned(),
        target.reachability_state.as_str(),
    ));
    rows.push(value_row(
        "boundary_cue_visible",
        "Local-vs-managed cue",
        if target.local_vs_managed_boundary_visible {
            "Visible (target is not the local desktop)".to_owned()
        } else {
            "Hidden (local desktop)".to_owned()
        },
    ));
    InspectorSection::new(InspectorSectionId::TargetIdentity, rows)
}

fn project_toolchain_section(
    toolchain: &ToolchainIdentity,
    provenance: &Provenance,
) -> InspectorSection {
    let toolchain_decision = decision_for(provenance, ResolverInputField::ToolchainClass);

    let mut class_row = token_row(
        "toolchain_class",
        "Toolchain class",
        toolchain_class_label(toolchain.toolchain_class).to_owned(),
        toolchain.toolchain_class.as_str(),
    );
    if let Some(decision) = toolchain_decision {
        class_row.winning_source = Some(decision.winning_source);
        class_row.conflicting_sources = decision.conflicting_sources.clone();
    }

    let mut rows = vec![
        class_row,
        value_row(
            "toolchain_id",
            "Toolchain id",
            toolchain.toolchain_id.clone(),
        ),
        value_row(
            "resolved_version",
            "Resolved version",
            toolchain.resolved_version.clone(),
        ),
        value_row(
            "activation_strategy",
            "Activation strategy",
            toolchain.activation_strategy.as_str().to_owned(),
        ),
    ];
    if toolchain.degraded_fallback_flag {
        rows.push(InspectorRow {
            row_id: "degraded_fallback".to_owned(),
            label: "Degraded fallback".to_owned(),
            value: "Resolver fell back to a less-preferred toolchain".to_owned(),
            value_token: Some("true".to_owned()),
            winning_source: None,
            conflicting_sources: Vec::new(),
            missing_field_reason: None,
            degraded_reason: Some(DegradedFieldReason::ToolchainFallback),
        });
    }
    InspectorSection::new(InspectorSectionId::ToolchainIdentity, rows)
}

fn project_python_environment_section(detection: &PythonEnvironmentDetection) -> InspectorSection {
    let mut rows = vec![
        value_row(
            "detector_version",
            "Detector version",
            detection.detector_version.clone(),
        ),
        value_row(
            "workspace_root_ref",
            "Workspace root",
            detection.workspace_root_ref.clone(),
        ),
        token_row(
            "interpreter_state",
            "Interpreter state",
            python_resolution_state_label(detection.interpreter.resolution_state).to_owned(),
            detection.interpreter.resolution_state.as_str(),
        ),
        value_or_missing_row(
            "interpreter_value",
            "Interpreter",
            python_interpreter_value(detection),
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
        value_or_missing_row(
            "interpreter_ref",
            "Interpreter ref",
            detection.interpreter.interpreter_ref.clone(),
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
        python_source_row(
            "interpreter_source",
            "Interpreter source",
            detection.interpreter.winning_source,
        ),
        fallback_row(
            "interpreter_fallback",
            "Interpreter fallback",
            detection
                .interpreter
                .fallback_path
                .as_ref()
                .map(|fallback| fallback.value_token.clone()),
        ),
        token_row(
            "environment_manager_state",
            "Environment manager state",
            python_resolution_state_label(detection.environment_manager.resolution_state)
                .to_owned(),
            detection.environment_manager.resolution_state.as_str(),
        ),
        value_or_missing_row(
            "environment_manager_value",
            "Environment manager",
            python_manager_value(detection),
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
        value_or_missing_row(
            "environment_ref",
            "Environment ref",
            detection.environment_manager.environment_ref.clone(),
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
        python_source_row(
            "environment_manager_source",
            "Environment manager source",
            detection.environment_manager.winning_source,
        ),
        fallback_row(
            "environment_manager_fallback",
            "Environment manager fallback",
            detection
                .environment_manager
                .fallback_path
                .as_ref()
                .map(|fallback| fallback.value_token.clone()),
        ),
    ];

    for (idx, ambiguity) in detection.unresolved_ambiguities.iter().enumerate() {
        rows.push(InspectorRow {
            row_id: format!("python_ambiguity_{idx}"),
            label: format!(
                "{} ambiguity",
                python_environment_subject_label(ambiguity.subject)
            ),
            value: format!(
                "{} ({})",
                ambiguity.candidate_values.join(" vs "),
                ambiguity.resolution_hint
            ),
            value_token: Some(ambiguity.candidate_values.join("|")),
            winning_source: None,
            conflicting_sources: Vec::new(),
            missing_field_reason: None,
            degraded_reason: Some(DegradedFieldReason::ConfidenceLow),
        });
    }

    for card in &detection.provenance_cards {
        rows.push(InspectorRow {
            row_id: format!("python_card_{}", stable_row_suffix(&card.card_id)),
            label: format!(
                "{} · {}",
                python_environment_subject_label(card.subject),
                python_source_kind_label(card.source_kind)
            ),
            value: format!("{} ({})", card.summary, card.disposition.as_str()),
            value_token: card.value_token.clone(),
            winning_source: None,
            conflicting_sources: Vec::new(),
            missing_field_reason: None,
            degraded_reason: python_detector_disposition_degraded_reason(card.disposition),
        });
    }

    InspectorSection::new(InspectorSectionId::PythonEnvironmentDetection, rows)
}

fn project_node_detection_section(detection: &NodeToolchainDetection) -> InspectorSection {
    let mut rows = vec![
        value_row(
            "detector_version",
            "Detector version",
            detection.detector_version.clone(),
        ),
        value_row(
            "workspace_root_ref",
            "Workspace root",
            detection.workspace_root_ref.clone(),
        ),
        token_row(
            "node_runtime_state",
            "Node runtime state",
            node_resolution_state_label(detection.node_runtime.resolution_state).to_owned(),
            detection.node_runtime.resolution_state.as_str(),
        ),
        value_or_missing_row(
            "node_runtime_value",
            "Node runtime",
            detection
                .node_runtime
                .resolved_requirement
                .as_ref()
                .map(|value| format!("node@{value}")),
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
        node_source_row(
            "node_runtime_source",
            "Node source",
            detection.node_runtime.winning_source,
        ),
        fallback_row(
            "node_runtime_fallback",
            "Node fallback",
            detection
                .node_runtime
                .fallback_path
                .as_ref()
                .map(|fallback| fallback.value_token.clone()),
        ),
        token_row(
            "package_manager_state",
            "Package manager state",
            node_resolution_state_label(detection.package_manager.resolution_state).to_owned(),
            detection.package_manager.resolution_state.as_str(),
        ),
        value_or_missing_row(
            "package_manager_value",
            "Package manager",
            package_manager_value(detection),
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
        node_source_row(
            "package_manager_source",
            "Package manager source",
            detection.package_manager.winning_source,
        ),
        fallback_row(
            "package_manager_fallback",
            "Package manager fallback",
            detection
                .package_manager
                .fallback_path
                .as_ref()
                .map(|fallback| fallback.value_token.clone()),
        ),
    ];

    for (idx, ambiguity) in detection.unresolved_ambiguities.iter().enumerate() {
        rows.push(InspectorRow {
            row_id: format!("node_ambiguity_{idx}"),
            label: format!(
                "{} ambiguity",
                node_detection_subject_label(ambiguity.subject)
            ),
            value: format!(
                "{} ({})",
                ambiguity.candidate_values.join(" vs "),
                ambiguity.resolution_hint
            ),
            value_token: Some(ambiguity.candidate_values.join("|")),
            winning_source: None,
            conflicting_sources: Vec::new(),
            missing_field_reason: None,
            degraded_reason: Some(DegradedFieldReason::ConfidenceLow),
        });
    }

    for card in &detection.provenance_cards {
        rows.push(InspectorRow {
            row_id: format!("node_card_{}", stable_row_suffix(&card.card_id)),
            label: format!(
                "{} · {}",
                node_detection_subject_label(card.subject),
                node_source_kind_label(card.source_kind)
            ),
            value: format!("{} ({})", card.summary, card.disposition.as_str()),
            value_token: card.value_token.clone(),
            winning_source: None,
            conflicting_sources: Vec::new(),
            missing_field_reason: None,
            degraded_reason: detector_disposition_degraded_reason(card.disposition),
        });
    }

    InspectorSection::new(InspectorSectionId::NodeToolchainDetection, rows)
}

fn project_capsule_section(capsule: &EnvironmentCapsuleRef) -> InspectorSection {
    let rows = vec![
        value_row("capsule_id", "Capsule id", capsule.capsule_id.clone()),
        value_row("capsule_hash", "Capsule hash", capsule.capsule_hash.clone()),
        value_row(
            "resolved_schema_version",
            "Capsule schema",
            capsule.resolved_schema_version.clone(),
        ),
        token_row(
            "drift_state",
            "Drift state",
            capsule_drift_label(capsule.drift_state).to_owned(),
            capsule.drift_state.as_str(),
        ),
    ];
    InspectorSection::new(InspectorSectionId::EnvironmentCapsule, rows)
}

fn project_policy_section(policy: &PolicyAndTrust) -> InspectorSection {
    let rows = vec![
        token_row(
            "trust_state",
            "Trust state",
            trust_label(policy.trust_state).to_owned(),
            trust_token(policy.trust_state),
        ),
        token_row(
            "identity_mode",
            "Identity mode",
            identity_mode_label(policy.identity_mode).to_owned(),
            policy.identity_mode.as_str(),
        ),
        value_row(
            "policy_epoch",
            "Policy epoch",
            policy.policy_epoch.to_string(),
        ),
    ];
    InspectorSection::new(InspectorSectionId::PolicyAndTrust, rows)
}

fn project_scope_section(scope: ScopeClass) -> InspectorSection {
    let rows = vec![token_row(
        "scope_class",
        "Workset scope",
        scope_label(scope).to_owned(),
        scope.as_str(),
    )];
    InspectorSection::new(InspectorSectionId::Scope, rows)
}

fn project_cache_section(cache: CacheDisposition) -> InspectorSection {
    let rows = vec![token_row(
        "cache_disposition",
        "Cache disposition",
        cache_label(cache).to_owned(),
        cache.as_str(),
    )];
    InspectorSection::new(InspectorSectionId::Cache, rows)
}

fn project_resolver_explanations_section(context: &ExecutionContext) -> InspectorSection {
    let mut rows = vec![
        token_row(
            "target_confidence_level",
            "Target confidence",
            confidence_label(context.target_confidence.level).to_owned(),
            context.target_confidence.level.as_str(),
        ),
        value_row(
            "target_confidence_reasons",
            "Target confidence reasons",
            context
                .target_confidence
                .reasons
                .iter()
                .map(|reason| target_confidence_reason_label(*reason))
                .collect::<Vec<_>>()
                .join(", "),
        ),
        value_row(
            "reusable_surfaces",
            "Reusable by",
            context
                .reusable_surfaces
                .iter()
                .map(|surface| surface.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        ),
        prebuild_state_row(&context.prebuild_metadata),
        mixed_version_row(&context.mixed_version_drift),
    ];

    if let Some(invalidation_reason) = context.prebuild_metadata.invalidation_reason {
        rows.push(token_row(
            "prebuild_invalidation_reason",
            "Prebuild invalidation",
            prebuild_invalidation_label(invalidation_reason).to_owned(),
            invalidation_reason.as_str(),
        ));
    }

    for (idx, explanation) in context.explanations.iter().enumerate() {
        rows.push(explanation_row(idx, explanation));
    }

    InspectorSection::new(InspectorSectionId::ResolverExplanations, rows)
}

fn project_provenance_section(provenance: &Provenance) -> InspectorSection {
    let mut rows = vec![
        value_row(
            "provenance_record_id",
            "Provenance record id",
            provenance.provenance_record_id.clone(),
        ),
        value_row(
            "resolver_version",
            "Resolver version",
            provenance.resolver_version.clone(),
        ),
        value_row("recorded_at", "Recorded at", provenance.recorded_at.clone()),
        token_row(
            "confidence_level",
            "Confidence",
            confidence_label(provenance.confidence_level).to_owned(),
            provenance.confidence_level.as_str(),
        ),
    ];
    for decision in &provenance.input_decisions {
        rows.push(decision_row(decision));
    }
    InspectorSection::new(InspectorSectionId::Provenance, rows)
}

fn project_degraded_section(degraded: &[DegradedFieldRecord]) -> InspectorSection {
    let rows = if degraded.is_empty() {
        vec![InspectorRow {
            row_id: "no_degraded_fields".to_owned(),
            label: "Honesty markers".to_owned(),
            value: "None — every field resolved cleanly".to_owned(),
            value_token: Some("none".to_owned()),
            winning_source: None,
            conflicting_sources: Vec::new(),
            missing_field_reason: None,
            degraded_reason: None,
        }]
    } else {
        degraded
            .iter()
            .enumerate()
            .map(|(idx, record)| InspectorRow {
                row_id: format!("degraded_{idx}"),
                label: record.field_path.clone(),
                value: degraded_reason_label(record.reason).to_owned(),
                value_token: Some(record.reason.as_str().to_owned()),
                winning_source: None,
                conflicting_sources: Vec::new(),
                missing_field_reason: None,
                degraded_reason: Some(record.reason),
            })
            .collect()
    };
    InspectorSection::new(InspectorSectionId::DegradedFields, rows)
}

fn decision_row(decision: &ResolverInputDecision) -> InspectorRow {
    let label = match decision.field {
        ResolverInputField::TargetClass => "Target precedence",
        ResolverInputField::WorkingDirectory => "Working directory precedence",
        ResolverInputField::ToolchainClass => "Toolchain precedence",
    };
    let value = if decision.resolved_value_token.is_empty() {
        "(unsettled)".to_owned()
    } else {
        decision.resolved_value_token.clone()
    };
    InspectorRow {
        row_id: format!("decision_{}", decision.field.as_str()),
        label: label.to_owned(),
        value,
        value_token: Some(decision.resolved_value_token.clone()),
        winning_source: Some(decision.winning_source),
        conflicting_sources: decision.conflicting_sources.clone(),
        missing_field_reason: None,
        degraded_reason: None,
    }
}

fn decision_for(
    provenance: &Provenance,
    field: ResolverInputField,
) -> Option<&ResolverInputDecision> {
    provenance
        .input_decisions
        .iter()
        .find(|decision| decision.field == field)
}

fn value_row(row_id: &str, label: &str, value: String) -> InspectorRow {
    let token = value.clone();
    InspectorRow {
        row_id: row_id.to_owned(),
        label: label.to_owned(),
        value,
        value_token: Some(token),
        winning_source: None,
        conflicting_sources: Vec::new(),
        missing_field_reason: None,
        degraded_reason: None,
    }
}

fn value_or_missing_row(
    row_id: &str,
    label: &str,
    value: Option<String>,
    reason: InspectorMissingFieldReason,
) -> InspectorRow {
    match value {
        Some(value) => value_row(row_id, label, value),
        None => missing_row(row_id, label, reason),
    }
}

fn node_source_row(
    row_id: &str,
    label: &str,
    source: Option<NodeToolchainSourceKind>,
) -> InspectorRow {
    match source {
        Some(source) => token_row(
            row_id,
            label,
            node_source_kind_label(source).to_owned(),
            source.as_str(),
        ),
        None => missing_row(
            row_id,
            label,
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
    }
}

fn python_source_row(
    row_id: &str,
    label: &str,
    source: Option<PythonEnvironmentSourceKind>,
) -> InspectorRow {
    match source {
        Some(source) => token_row(
            row_id,
            label,
            python_source_kind_label(source).to_owned(),
            source.as_str(),
        ),
        None => missing_row(
            row_id,
            label,
            InspectorMissingFieldReason::ResolverUnsettled,
        ),
    }
}

fn fallback_row(row_id: &str, label: &str, value: Option<String>) -> InspectorRow {
    value_or_missing_row(
        row_id,
        label,
        value,
        InspectorMissingFieldReason::ResolverUnsettled,
    )
}

fn token_row(row_id: &str, label: &str, value: String, token: &str) -> InspectorRow {
    InspectorRow {
        row_id: row_id.to_owned(),
        label: label.to_owned(),
        value,
        value_token: Some(token.to_owned()),
        winning_source: None,
        conflicting_sources: Vec::new(),
        missing_field_reason: None,
        degraded_reason: None,
    }
}

fn missing_row(row_id: &str, label: &str, reason: InspectorMissingFieldReason) -> InspectorRow {
    InspectorRow {
        row_id: row_id.to_owned(),
        label: label.to_owned(),
        value: reason.label().to_owned(),
        value_token: None,
        winning_source: None,
        conflicting_sources: Vec::new(),
        missing_field_reason: Some(reason),
        degraded_reason: None,
    }
}

fn prebuild_state_row(prebuild: &PrebuildMetadata) -> InspectorRow {
    token_row(
        "prebuild_reuse_state",
        "Prebuild reuse",
        prebuild_reuse_label(prebuild.reuse_state).to_owned(),
        prebuild.reuse_state.as_str(),
    )
}

fn mixed_version_row(mixed_version: &MixedVersionDrift) -> InspectorRow {
    token_row(
        "mixed_version_state",
        "Mixed-version state",
        mixed_version_label(mixed_version.state).to_owned(),
        mixed_version.state.as_str(),
    )
}

fn explanation_row(idx: usize, explanation: &ExecutionContextExplanation) -> InspectorRow {
    InspectorRow {
        row_id: format!("explanation_{idx}"),
        label: explanation.field_path.clone(),
        value: format!(
            "{} · {} · {}",
            effect_label(explanation.effect),
            reason_code_label(explanation.reason_code),
            reason_source_label(explanation.source)
        ),
        value_token: Some(explanation.reason_code.as_str().to_owned()),
        winning_source: None,
        conflicting_sources: explanation.related_input_sources.clone(),
        missing_field_reason: None,
        degraded_reason: None,
    }
}

const fn target_class_label(class: TargetClass) -> &'static str {
    match class {
        TargetClass::LocalHost => "Local desktop",
        TargetClass::SshRemote => "Remote (SSH)",
        TargetClass::ContainerLocal => "Local container",
        TargetClass::Devcontainer => "Devcontainer",
        TargetClass::RemoteWorkspaceVm => "Remote workspace VM",
        TargetClass::PrebuildRuntime => "Prebuild runtime",
        TargetClass::ManagedWorkspace => "Managed workspace",
        TargetClass::NotebookKernelLocal => "Notebook kernel (local)",
        TargetClass::NotebookKernelRemote => "Notebook kernel (remote)",
        TargetClass::AiSandbox => "AI sandbox",
    }
}

const fn reachability_label(state: ReachabilityState) -> &'static str {
    match state {
        ReachabilityState::Reachable => "Reachable",
        ReachabilityState::Warming => "Warming",
        ReachabilityState::Degraded => "Degraded",
        ReachabilityState::Unreachable => "Unreachable",
        ReachabilityState::PolicyBlocked => "Policy blocked",
    }
}

const fn toolchain_class_label(class: ToolchainClass) -> &'static str {
    match class {
        ToolchainClass::Interpreter => "Interpreter",
        ToolchainClass::CompilerToolchain => "Compiler toolchain",
        ToolchainClass::PackageManagerRunner => "Package-manager runner",
        ToolchainClass::ContainerisedRuntime => "Containerised runtime",
        ToolchainClass::NotebookKernelRuntime => "Notebook kernel runtime",
        ToolchainClass::LanguageServerProcess => "Language-server process",
        ToolchainClass::DebugAdapterRuntime => "Debug-adapter runtime",
        ToolchainClass::TestRunnerRuntime => "Test-runner runtime",
        ToolchainClass::BuildDriverRuntime => "Build-driver runtime",
        ToolchainClass::AiToolRuntime => "AI tool runtime",
        ToolchainClass::LoginShell => "Login shell",
    }
}

fn python_interpreter_value(detection: &PythonEnvironmentDetection) -> Option<String> {
    match (
        detection.interpreter.resolved_requirement.as_deref(),
        detection.interpreter.interpreter_ref.as_deref(),
    ) {
        (Some(requirement), Some(interpreter_ref))
            if !requirement.is_empty() && !interpreter_ref.is_empty() =>
        {
            Some(format!("python@{requirement} ({interpreter_ref})"))
        }
        (Some(requirement), _) if !requirement.is_empty() => Some(format!("python@{requirement}")),
        (_, Some(interpreter_ref)) if !interpreter_ref.is_empty() => {
            Some(interpreter_ref.to_owned())
        }
        _ => None,
    }
}

fn python_manager_value(detection: &PythonEnvironmentDetection) -> Option<String> {
    let kind = detection.environment_manager.kind?;
    let base = match &detection.environment_manager.version {
        Some(version) if !version.is_empty() => format!("{}@{version}", kind.as_str()),
        _ => kind.as_str().to_owned(),
    };
    match &detection.environment_manager.environment_ref {
        Some(environment_ref) if !environment_ref.is_empty() => {
            Some(format!("{base} ({environment_ref})"))
        }
        _ => Some(base),
    }
}

const fn python_environment_subject_label(subject: PythonEnvironmentSubject) -> &'static str {
    match subject {
        PythonEnvironmentSubject::Interpreter => "Interpreter",
        PythonEnvironmentSubject::EnvironmentManager => "Environment manager",
    }
}

const fn python_resolution_state_label(state: PythonEnvironmentResolutionState) -> &'static str {
    match state {
        PythonEnvironmentResolutionState::Resolved => "Resolved",
        PythonEnvironmentResolutionState::Fallback => "Fallback",
        PythonEnvironmentResolutionState::Missing => "Missing",
        PythonEnvironmentResolutionState::Ambiguous => "Ambiguous",
        PythonEnvironmentResolutionState::Unsupported => "Unsupported",
    }
}

const fn python_source_kind_label(source: PythonEnvironmentSourceKind) -> &'static str {
    match source {
        PythonEnvironmentSourceKind::ExplicitOverride => "Explicit override",
        PythonEnvironmentSourceKind::VenvPyvenvCfg => ".venv pyvenv.cfg",
        PythonEnvironmentSourceKind::VenvDirectory => ".venv directory",
        PythonEnvironmentSourceKind::PythonVersionFile => ".python-version",
        PythonEnvironmentSourceKind::ToolVersions => ".tool-versions",
        PythonEnvironmentSourceKind::MiseToml => "mise.toml",
        PythonEnvironmentSourceKind::PyprojectRequiresPython => "pyproject requires-python",
        PythonEnvironmentSourceKind::PyprojectPoetryDependency => {
            "pyproject Poetry python dependency"
        }
        PythonEnvironmentSourceKind::UvLockfile => "uv lockfile",
        PythonEnvironmentSourceKind::PyprojectUv => "pyproject uv section",
        PythonEnvironmentSourceKind::PoetryLockfile => "Poetry lockfile",
        PythonEnvironmentSourceKind::PyprojectPoetry => "pyproject Poetry section",
        PythonEnvironmentSourceKind::CondaEnvironmentFile => "Conda environment file",
        PythonEnvironmentSourceKind::UserProfileDefault => "User/profile default",
        PythonEnvironmentSourceKind::AmbientPath => "Ambient PATH",
        PythonEnvironmentSourceKind::DetectorFallback => "Detector fallback",
        PythonEnvironmentSourceKind::UnreadableSource => "Unreadable source",
    }
}

const fn python_detector_disposition_degraded_reason(
    disposition: PythonEnvironmentProvenanceDisposition,
) -> Option<DegradedFieldReason> {
    match disposition {
        PythonEnvironmentProvenanceDisposition::Fallback => {
            Some(DegradedFieldReason::ToolchainFallback)
        }
        PythonEnvironmentProvenanceDisposition::Ambiguous => {
            Some(DegradedFieldReason::ConfidenceLow)
        }
        PythonEnvironmentProvenanceDisposition::Unsupported => {
            Some(DegradedFieldReason::ActivatorUnsupportedOnTarget)
        }
        _ => None,
    }
}

fn package_manager_value(detection: &NodeToolchainDetection) -> Option<String> {
    let kind = detection.package_manager.kind?;
    Some(match &detection.package_manager.version {
        Some(version) if !version.is_empty() => format!("{}@{version}", kind.as_str()),
        _ => kind.as_str().to_owned(),
    })
}

const fn node_detection_subject_label(subject: NodeToolchainSubject) -> &'static str {
    match subject {
        NodeToolchainSubject::NodeRuntime => "Node runtime",
        NodeToolchainSubject::PackageManager => "Package manager",
    }
}

const fn node_resolution_state_label(state: NodeToolchainResolutionState) -> &'static str {
    match state {
        NodeToolchainResolutionState::Resolved => "Resolved",
        NodeToolchainResolutionState::Fallback => "Fallback",
        NodeToolchainResolutionState::Missing => "Missing",
        NodeToolchainResolutionState::Ambiguous => "Ambiguous",
        NodeToolchainResolutionState::Unsupported => "Unsupported",
    }
}

const fn node_source_kind_label(source: NodeToolchainSourceKind) -> &'static str {
    match source {
        NodeToolchainSourceKind::ExplicitOverride => "Explicit override",
        NodeToolchainSourceKind::PackageJsonPackageManager => "package.json packageManager",
        NodeToolchainSourceKind::PackageJsonEngines => "package.json engines",
        NodeToolchainSourceKind::PackageJsonVolta => "package.json Volta",
        NodeToolchainSourceKind::Nvmrc => ".nvmrc",
        NodeToolchainSourceKind::NodeVersionFile => ".node-version",
        NodeToolchainSourceKind::ToolVersions => ".tool-versions",
        NodeToolchainSourceKind::MiseToml => "mise.toml",
        NodeToolchainSourceKind::PnpmLockfile => "pnpm lockfile",
        NodeToolchainSourceKind::NpmLockfile => "npm lockfile",
        NodeToolchainSourceKind::UserProfileDefault => "User/profile default",
        NodeToolchainSourceKind::AmbientPath => "Ambient PATH",
        NodeToolchainSourceKind::DetectorFallback => "Detector fallback",
        NodeToolchainSourceKind::UnreadableSource => "Unreadable source",
    }
}

const fn detector_disposition_degraded_reason(
    disposition: NodeToolchainProvenanceDisposition,
) -> Option<DegradedFieldReason> {
    match disposition {
        NodeToolchainProvenanceDisposition::Fallback => {
            Some(DegradedFieldReason::ToolchainFallback)
        }
        NodeToolchainProvenanceDisposition::Ambiguous => Some(DegradedFieldReason::ConfidenceLow),
        NodeToolchainProvenanceDisposition::Unsupported => {
            Some(DegradedFieldReason::ActivatorUnsupportedOnTarget)
        }
        _ => None,
    }
}

fn stable_row_suffix(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | '0'..='9' | '_' => ch,
            'A'..='Z' => ch.to_ascii_lowercase(),
            _ => '_',
        })
        .collect()
}

const fn capsule_drift_label(state: CapsuleDriftState) -> &'static str {
    match state {
        CapsuleDriftState::InSync => "In sync",
        CapsuleDriftState::StaleInputs => "Stale inputs",
        CapsuleDriftState::GeneratorChanged => "Generator changed",
        CapsuleDriftState::ManuallyDiverged => "Manually diverged",
        CapsuleDriftState::UnknownLineage => "Unknown lineage",
    }
}

const fn identity_mode_label(mode: IdentityMode) -> &'static str {
    match mode {
        IdentityMode::AccountFreeLocal => "Account-free (local)",
        IdentityMode::SelfHostedOrg => "Self-hosted org",
        IdentityMode::ManagedConvenience => "Managed convenience",
    }
}

const fn scope_label(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::CurrentRoot => "Current root",
        ScopeClass::NamedWorkset => "Named workset",
        ScopeClass::SparseSlice => "Sparse slice",
        ScopeClass::FullWorkspace => "Full workspace",
        ScopeClass::PolicyLimitedView => "Policy-limited view",
        ScopeClass::ReviewWorkspace => "Review workspace",
        ScopeClass::CompanionSurface => "Companion surface",
    }
}

const fn cache_label(disposition: CacheDisposition) -> &'static str {
    match disposition {
        CacheDisposition::Cold => "Cold (no reuse)",
        CacheDisposition::Warm => "Warm",
        CacheDisposition::PrebuildReused => "Prebuild reused",
        CacheDisposition::CapsuleReused => "Capsule reused",
        CacheDisposition::RejectedDrift => "Rejected (drift)",
        CacheDisposition::RejectedPolicy => "Rejected (policy)",
        CacheDisposition::RejectedTrust => "Rejected (trust)",
    }
}

const fn target_confidence_reason_label(reason: TargetConfidenceReason) -> &'static str {
    match reason {
        TargetConfidenceReason::ExactLocalTarget => "exact local target",
        TargetConfidenceReason::SurfaceRequestedTarget => "surface requested target",
        TargetConfidenceReason::ExplicitTargetOverride => "explicit target override",
        TargetConfidenceReason::WorkspaceDefaultTarget => "workspace default target",
        TargetConfidenceReason::ResolverFallbackTarget => "resolver fallback target",
        TargetConfidenceReason::ConflictingTargetSources => "conflicting target sources",
        TargetConfidenceReason::RemoteOrManagedBoundary => "remote or managed boundary",
        TargetConfidenceReason::TrustPending => "trust pending",
        TargetConfidenceReason::TrustRestricted => "trust restricted",
        TargetConfidenceReason::PolicyBlockedReachability => "policy blocked reachability",
        TargetConfidenceReason::CapsuleDrift => "capsule drift",
        TargetConfidenceReason::PrebuildRuntime => "prebuild runtime",
        TargetConfidenceReason::MixedVersionUnchecked => "mixed version unchecked",
    }
}

const fn prebuild_reuse_label(state: PrebuildReuseState) -> &'static str {
    match state {
        PrebuildReuseState::NotApplicable => "Not applicable",
        PrebuildReuseState::Candidate => "Candidate",
        PrebuildReuseState::Reused => "Reused",
        PrebuildReuseState::RejectedDrift => "Rejected: drift",
        PrebuildReuseState::RejectedPolicy => "Rejected: policy",
        PrebuildReuseState::RejectedTrust => "Rejected: trust",
    }
}

const fn prebuild_invalidation_label(reason: PrebuildInvalidationReason) -> &'static str {
    match reason {
        PrebuildInvalidationReason::CapsuleDrift => "Capsule drift",
        PrebuildInvalidationReason::PolicyEpochAdvanced => "Policy epoch advanced",
        PrebuildInvalidationReason::TrustStateRestricted => "Trust state restricted",
        PrebuildInvalidationReason::TrustStatePending => "Trust state pending",
        PrebuildInvalidationReason::TargetClassChanged => "Target class changed",
    }
}

const fn mixed_version_label(state: MixedVersionDriftState) -> &'static str {
    match state {
        MixedVersionDriftState::NotApplicable => "Not applicable",
        MixedVersionDriftState::Aligned => "Aligned",
        MixedVersionDriftState::NotNegotiated => "Not negotiated",
        MixedVersionDriftState::DriftDetected => "Drift detected",
    }
}

const fn effect_label(effect: ExecutionContextEffectClass) -> &'static str {
    match effect {
        ExecutionContextEffectClass::SelectedByPrecedence => "selected by precedence",
        ExecutionContextEffectClass::ConflictResolved => "conflict resolved",
        ExecutionContextEffectClass::TargetBoundaryVisible => "target boundary visible",
        ExecutionContextEffectClass::TargetBoundaryLocal => "local target boundary",
        ExecutionContextEffectClass::PolicyAllowed => "policy allowed",
        ExecutionContextEffectClass::PolicyNarrowed => "policy narrowed",
        ExecutionContextEffectClass::PolicyBlocked => "policy blocked",
        ExecutionContextEffectClass::TrustAccepted => "trust accepted",
        ExecutionContextEffectClass::TrustPending => "trust pending",
        ExecutionContextEffectClass::TrustRestricted => "trust restricted",
        ExecutionContextEffectClass::ScopeSelected => "scope selected",
        ExecutionContextEffectClass::PrebuildNotApplicable => "prebuild not applicable",
        ExecutionContextEffectClass::PrebuildReused => "prebuild reused",
        ExecutionContextEffectClass::PrebuildRejected => "prebuild rejected",
        ExecutionContextEffectClass::MixedVersionNotApplicable => "mixed version not applicable",
        ExecutionContextEffectClass::MixedVersionUnchecked => "mixed version unchecked",
        ExecutionContextEffectClass::ReusableAcrossSurfaces => "reusable across surfaces",
    }
}

const fn reason_code_label(reason: ExecutionContextReasonCode) -> &'static str {
    match reason {
        ExecutionContextReasonCode::ExplicitOverrideWon => "explicit override won",
        ExecutionContextReasonCode::SurfaceRequestWon => "surface request won",
        ExecutionContextReasonCode::WorkspaceDefaultWon => "workspace default won",
        ExecutionContextReasonCode::ResolverFallbackUsed => "resolver fallback used",
        ExecutionContextReasonCode::LowerPrecedenceConflict => "lower precedence conflict",
        ExecutionContextReasonCode::LocalTargetNoBoundary => "local target no boundary",
        ExecutionContextReasonCode::RemoteOrManagedBoundary => "remote or managed boundary",
        ExecutionContextReasonCode::PolicyEpochCurrent => "policy epoch current",
        ExecutionContextReasonCode::PolicyNarrowedByTrust => "policy narrowed by trust",
        ExecutionContextReasonCode::PolicyBlockedTargetReachability => {
            "policy blocked target reachability"
        }
        ExecutionContextReasonCode::TrustStateTrusted => "trust state trusted",
        ExecutionContextReasonCode::TrustStateRestricted => "trust state restricted",
        ExecutionContextReasonCode::TrustStatePendingEvaluation => "trust state pending evaluation",
        ExecutionContextReasonCode::WorkspaceScopeDefault => "workspace scope default",
        ExecutionContextReasonCode::PrebuildTargetNotSelected => "prebuild target not selected",
        ExecutionContextReasonCode::PrebuildSnapshotCompatible => "prebuild snapshot compatible",
        ExecutionContextReasonCode::PrebuildRejectedByCapsuleDrift => {
            "prebuild rejected by capsule drift"
        }
        ExecutionContextReasonCode::PrebuildRejectedByTrust => "prebuild rejected by trust",
        ExecutionContextReasonCode::LocalOnlyNoHelperVersion => "local only no helper version",
        ExecutionContextReasonCode::HelperBoundaryNotNegotiated => "helper boundary not negotiated",
        ExecutionContextReasonCode::SharedContextContract => "shared context contract",
    }
}

const fn reason_source_label(source: ExecutionContextReasonSource) -> &'static str {
    match source {
        ExecutionContextReasonSource::Resolver => "resolver",
        ExecutionContextReasonSource::ExplicitOverride => "explicit override",
        ExecutionContextReasonSource::SurfaceRequest => "surface request",
        ExecutionContextReasonSource::WorkspaceAuthority => "workspace authority",
        ExecutionContextReasonSource::PolicyAuthority => "policy authority",
        ExecutionContextReasonSource::TrustAuthority => "trust authority",
        ExecutionContextReasonSource::EnvironmentCapsule => "environment capsule",
        ExecutionContextReasonSource::HelperBoundary => "helper boundary",
    }
}

const fn confidence_label(level: ConfidenceLevel) -> &'static str {
    match level {
        ConfidenceLevel::High => "High",
        ConfidenceLevel::Medium => "Medium",
        ConfidenceLevel::Low => "Low",
    }
}

const fn degraded_reason_label(reason: DegradedFieldReason) -> &'static str {
    match reason {
        DegradedFieldReason::ToolchainFallback => "Toolchain fell back to a less-preferred lane",
        DegradedFieldReason::ActivatorBlockedByTrust => "Activator blocked by trust policy",
        DegradedFieldReason::ActivatorBlockedByPolicy => "Activator blocked by org policy",
        DegradedFieldReason::ActivatorUnsupportedOnTarget => "Activator unsupported on this target",
        DegradedFieldReason::CapsuleUnresolved => "Environment capsule did not resolve",
        DegradedFieldReason::CapsuleDriftDetected => "Environment capsule drifted",
        DegradedFieldReason::TargetUnreachable => "Target is unreachable",
        DegradedFieldReason::PolicyEpochStale => "Policy epoch is stale",
        DegradedFieldReason::TrustStateUnresolved => "Workspace trust posture is unresolved",
        DegradedFieldReason::WorksetMemberUnavailable => "A workset member is unavailable",
        DegradedFieldReason::ProvenanceGap => "Provenance is incomplete for this lane",
        DegradedFieldReason::ConfidenceLow => "Resolver confidence is low",
        DegradedFieldReason::RemoteAgentScopeMismatch => "Remote-agent scope mismatch",
    }
}

const fn trust_label(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "Trusted",
        TrustState::Restricted => "Restricted",
        TrustState::PendingEvaluation => "Pending evaluation",
    }
}

const fn trust_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

const fn actor_label(actor: ActorClass) -> &'static str {
    match actor {
        ActorClass::UserKeystroke => "User keystroke",
        ActorClass::UserCommand => "User command",
        ActorClass::SessionOverride => "Session override",
        ActorClass::WorkspaceMigration => "Workspace migration",
        ActorClass::ExtensionApi => "Extension API",
        ActorClass::AiApply => "AI apply",
        ActorClass::ScheduledTask => "Scheduled task",
        ActorClass::ImportedProfile => "Imported profile",
        ActorClass::AdminPolicyInjector => "Admin policy injector",
    }
}

const fn surface_class_label(surface: SurfaceClass) -> &'static str {
    match surface {
        SurfaceClass::Terminal => "Terminal",
        SurfaceClass::Task => "Task",
        SurfaceClass::Debug => "Debug",
        SurfaceClass::Test => "Test",
        SurfaceClass::NotebookKernel => "Notebook kernel",
        SurfaceClass::Scaffolding => "Scaffolding",
        SurfaceClass::AiToolCall => "AI tool call",
        SurfaceClass::DoctorRepair => "Doctor repair",
        SurfaceClass::ImportProbe => "Import probe",
        SurfaceClass::ReplayProbe => "Replay probe",
    }
}

#[cfg(test)]
mod tests;
