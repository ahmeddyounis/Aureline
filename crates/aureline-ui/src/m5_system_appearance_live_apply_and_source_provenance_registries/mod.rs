//! Implemented M5 system-appearance live-apply and appearance-source-provenance registries.
//!
//! The frozen [platform-fit matrix][matrix] names Aureline's six platform-fit families and locks their
//! controlled vocabulary. This module is the implement lane for the live system-appearance-response family:
//! it turns the concrete *live theme / contrast / accent / text-scale response* grammar of the
//! theme-contrast-live-change family into registry resolvers that produce export-safe, honest projections. A
//! user can then trust that when the host platform changes its system theme, contrast, accent, or text scale,
//! every claimed macOS, Windows, and Linux desktop profile either applies the change live or names an
//! explicit fallback / restart-required posture instead of drifting silently, that a live change preserves
//! active shell, editor, and dialog continuity rather than forcing a mystery repaint or resetting local
//! context, that the active platform-appearance source and any fallback posture are recorded in settings,
//! diagnostics, and support exports, and that a surface which mislabels its posture or hides its appearance
//! source degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Apply system theme, contrast, accent, and text-scale changes live wherever the host platform supports
//!   it, and expose an explicit fallback or restart-required posture where live reapplication is unavailable
//!   or unsafe.** [`resolve_appearance_live_apply_entry`] refuses to read as a clean, registry-bound response
//!   entry unless it names a canonical registry token, a classified [appearance posture][M5AppearancePosture],
//!   a theme-contrast-live-change role, covers every [response form][M5AppearanceResponseForm] (the applied
//!   visual reapply, the recorded canonical posture truth, and the accessible announcement), records a posture
//!   label and live-reapply state that match the claimed support posture, preserves active-context continuity,
//!   and explains any narrower-than-live behavior; otherwise it degrades.
//! * **Preserve active shell, editor, and dialog continuity during live changes instead of forcing mystery
//!   repaints or resetting local context.** The `preserves_active_context_continuity` invariant degrades an
//!   entry to [`M5AppearanceLiveApplyEntryDegradeReason::ActiveContextContinuityNotPreserved`] so a live theme,
//!   contrast, accent, or text-scale change can never corrupt focus, layout, or meaning on a protected path.
//! * **Record the active platform-appearance source and any fallback posture in settings, diagnostics, and
//!   support exports, and generate diagnostics from the same appearance registry.**
//!   [`resolve_appearance_source_provenance_entry`] names a classified
//!   [record surface][M5AppearanceRecordSurface], requires the active source and posture to be recorded by
//!   stable command ID, an in-product record surface, and a source signal, and degrades to
//!   [`M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded`] when a record drops any
//!   leg of the provenance triple, so diagnostics and support exports can always distinguish live-apply
//!   support from restart-required or unsupported behavior. [`appearance_response_matches_posture`] rejects a
//!   live-apply entry that did not reapply live and a restart-required or unsupported entry that claims to have
//!   reapplied live so a mislabeled posture degrades to
//!   [`M5AppearanceLiveApplyEntryDegradeReason::PostureMislabeledForSupport`], and
//!   [`M5SystemAppearanceRegistriesPacket::render_appearance_posture_table`] emits the same posture truth the
//!   resolvers produced.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5PlatformFitRole`] role vocabulary and
//! the [`M5ThemeContrastLiveChangeRole`] theme-contrast-live-change-role vocabulary — so shell, settings,
//! docs, onboarding, CLI, and support surfaces can never fork their own appearance-response meaning. Raw
//! secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_platform_fit_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_system_appearance_live_apply_and_source_provenance_registries,
    seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed,
    seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed,
    M5_SYSTEM_APPEARANCE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_platform_fit_matrix::{
    M5PlatformFitAccessibilityRoute, M5PlatformFitConsumerSurface, M5PlatformFitDeploymentLine,
    M5PlatformFitDowngradeTrigger, M5PlatformFitFamily, M5PlatformFitQualificationClass,
    M5PlatformFitRequiredLabel, M5PlatformFitRole, M5ThemeContrastLiveChangeRole,
    M5_FILE_PATH_AND_REVEAL_SCHEMA_REF, M5_PLATFORM_FIT_MATRIX_DOC_REF,
    M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SystemAppearanceRegistriesPacket`].
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_system_appearance_live_apply_and_source_provenance_registries";

/// Schema version for M5 system-appearance live-apply / source-provenance registry records.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/platform/m5-system-appearance-live-apply-and-source-provenance-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_DOC_REF: &str =
    "docs/platform/m5_system_appearance_live_apply_and_source_provenance_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-system-appearance-live-apply-and-source-provenance-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-system-appearance-live-apply-and-source-provenance-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-system-appearance-live-apply-and-source-provenance-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/platform/m5-system-appearance-live-apply-and-source-provenance-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SystemAppearanceRegistriesConsumerSurface = M5PlatformFitConsumerSurface;

/// One of the three response forms every appearance-response or provenance entry must hold across so a live
/// change keeps its truth whether it is applied to the surface, resolved to its canonical posture, or
/// announced to a screen reader. Minted by this lane because the frozen matrix names the
/// theme-contrast-live-change *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceResponseForm {
    /// The applied visual reapply form (the theme / contrast / accent / text-scale actually reapplied).
    AppliedVisualReapply,
    /// The literal / canonical posture truth kept explicit alongside the applied change.
    CanonicalPostureTruth,
    /// The spoken / searchable accessible announcement that keeps the change discoverable.
    AccessibleAnnouncement,
}

impl M5AppearanceResponseForm {
    /// Every response form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::AppliedVisualReapply,
        Self::CanonicalPostureTruth,
        Self::AccessibleAnnouncement,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppliedVisualReapply => "applied_visual_reapply",
            Self::CanonicalPostureTruth => "canonical_posture_truth",
            Self::AccessibleAnnouncement => "accessible_announcement",
        }
    }
}

/// Controlled support posture an appearance-response entry claims, so the canonical posture label and
/// live-reapply expectation share one registry rather than a hand-copied per-platform string. Minted by this
/// lane because the frozen matrix carries the live-appearance role but not the concrete live-apply /
/// restart-required / unsupported posture an entry must match. Every classified posture carries its canonical
/// posture label and whether it applies live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearancePosture {
    /// The change applies live on this host (`applies live`, live-reapplied).
    LiveApply,
    /// The change reapplies only after a restart (`restart required`, not live-reapplied, must explain).
    RestartRequired,
    /// The host does not expose this appearance signal (`not supported on this host`, must explain).
    Unsupported,
    /// The support posture is unclassified, which is disallowed.
    PostureUnclassified,
}

impl M5AppearancePosture {
    /// Every support posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveApply,
        Self::RestartRequired,
        Self::Unsupported,
        Self::PostureUnclassified,
    ];

    /// The three canonical postures every claimed M5 profile resolves appearance truth from.
    pub const CANONICAL_POSTURES: [Self; 3] =
        [Self::LiveApply, Self::RestartRequired, Self::Unsupported];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveApply => "live_apply",
            Self::RestartRequired => "restart_required",
            Self::Unsupported => "unsupported",
            Self::PostureUnclassified => "posture_unclassified",
        }
    }

    /// Whether the posture is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PostureUnclassified)
    }

    /// Whether this posture reapplies the change live rather than only after a restart or not at all.
    pub const fn applies_live(self) -> bool {
        matches!(self, Self::LiveApply)
    }

    /// The canonical posture label for this posture.
    pub const fn canonical_posture_label(self) -> &'static str {
        match self {
            Self::LiveApply => "applies live",
            Self::RestartRequired => "restart required",
            Self::Unsupported => "not supported on this host",
            Self::PostureUnclassified => "",
        }
    }
}

/// Controlled record surface an appearance source and posture must be recorded on, so the active
/// platform-appearance source is never hidden from diagnostics or support. Minted by this lane, tracking the
/// surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceRecordSurface {
    /// The settings surface.
    Settings,
    /// The diagnostics surface.
    Diagnostics,
    /// The support export.
    SupportExport,
    /// The record surface is unclassified, which is disallowed.
    RecordSurfaceUnclassified,
}

impl M5AppearanceRecordSurface {
    /// Every record surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Settings,
        Self::Diagnostics,
        Self::SupportExport,
        Self::RecordSurfaceUnclassified,
    ];

    /// The three canonical record surfaces the active source and posture must be recorded on.
    pub const CANONICAL_SURFACES: [Self; 3] =
        [Self::Settings, Self::Diagnostics, Self::SupportExport];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::RecordSurfaceUnclassified => "record_surface_unclassified",
        }
    }

    /// Whether the record surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::RecordSurfaceUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface a live appearance change must apply to and preserve
/// continuity on, so a change's meaning stays stable whether it repaints the shell chrome, the active editor,
/// an open dialog, a settings preview, or docs. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceSurfaceContext {
    /// The shell-chrome surface.
    ShellChrome,
    /// The active-editor surface.
    ActiveEditor,
    /// The open-dialog surface.
    OpenDialog,
    /// The settings-preview surface.
    SettingsPreview,
    /// The help / docs surface.
    DocsHelp,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5AppearanceSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellChrome,
        Self::ActiveEditor,
        Self::OpenDialog,
        Self::SettingsPreview,
        Self::DocsHelp,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ShellChrome,
        Self::ActiveEditor,
        Self::OpenDialog,
        Self::SettingsPreview,
        Self::DocsHelp,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellChrome => "shell_chrome",
            Self::ActiveEditor => "active_editor",
            Self::OpenDialog => "open_dialog",
            Self::SettingsPreview => "settings_preview",
            Self::DocsHelp => "docs_help",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part an appearance-response or provenance entry must be able to show, so no posture,
/// appearance source, or registry fact is left implicit behind a hand-copied per-platform string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The support posture the entry claims (response entry).
    SupportPosture,
    /// The rendered applied-appearance summary (response entry).
    AppliedAppearance,
    /// The response-form coverage (applied / canonical / accessible).
    ResponseFormCoverage,
    /// The rendered posture label (response entry).
    PostureLabel,
    /// The active appearance-source signal the entry records (provenance entry).
    AppearanceSourceSignal,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the appearance change or source (both entries).
    PlainLanguageMeaning,
}

impl M5AppearanceRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::SupportPosture,
        Self::AppliedAppearance,
        Self::ResponseFormCoverage,
        Self::PostureLabel,
        Self::AppearanceSourceSignal,
        Self::SurfaceContext,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::SupportPosture => "support_posture",
            Self::AppliedAppearance => "applied_appearance",
            Self::ResponseFormCoverage => "response_form_coverage",
            Self::PostureLabel => "posture_label",
            Self::AppearanceSourceSignal => "appearance_source_signal",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a posture,
/// appearance source, or a degraded appearance-response / provenance entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceRegistryNextAction {
    /// Expand the change's or source's plain-language meaning.
    ExpandResponseMeaning,
    /// Inspect the support posture or record surface the entry maps.
    InspectPostureOrSurface,
    /// Complete the applied / canonical / accessible response-form coverage.
    CompleteResponseFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5AppearanceRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandResponseMeaning,
        Self::InspectPostureOrSurface,
        Self::CompleteResponseFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandResponseMeaning => "expand_response_meaning",
            Self::InspectPostureOrSurface => "inspect_posture_or_surface",
            Self::CompleteResponseFormCoverage => "complete_response_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The platform-fit families covered.
    PlatformFitFamilies,
    /// The support postures carried.
    SupportPostures,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The response forms covered.
    ResponseForms,
    /// The record surfaces carried.
    RecordSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The rendered posture labels carried.
    PostureLabels,
    /// The accountable owner role.
    OwnerRole,
}

impl M5AppearanceRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::SupportPostures,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResponseForms,
        Self::RecordSurfaces,
        Self::SurfaceContext,
        Self::PostureLabels,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::SupportPostures,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::PlatformFitFamilies => "platform_fit_families",
            Self::SupportPostures => "support_postures",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResponseForms => "response_forms",
            Self::RecordSurfaces => "record_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::PostureLabels => "posture_labels",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an appearance-response entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, posture-mislabeled, continuity-losing, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceLiveApplyEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the response means.
    AppearanceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The support posture is unclassified (not in the preserved taxonomy).
    AppearancePostureUnclassified,
    /// The response is a hand-copied per-platform behavior instead of tracing to the canonical registry.
    ResponseNotBoundToRegistry,
    /// The recorded posture label or live-reapply state does not match the claimed support posture.
    PostureMislabeledForSupport,
    /// The entry does not preserve active shell / editor / dialog continuity through the change.
    ActiveContextContinuityNotPreserved,
    /// The applied / canonical / accessible response-form coverage is incomplete.
    ResponseFormCoverageIncomplete,
    /// The change is narrower than live (restart-required or unsupported) and no fallback is explained.
    NarrowerBehaviorNotExplained,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AppearanceLiveApplyEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::AppearanceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::AppearancePostureUnclassified,
        Self::ResponseNotBoundToRegistry,
        Self::PostureMislabeledForSupport,
        Self::ActiveContextContinuityNotPreserved,
        Self::ResponseFormCoverageIncomplete,
        Self::NarrowerBehaviorNotExplained,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppearanceTokenUnstated => "appearance_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::AppearancePostureUnclassified => "appearance_posture_unclassified",
            Self::ResponseNotBoundToRegistry => "response_not_bound_to_registry",
            Self::PostureMislabeledForSupport => "posture_mislabeled_for_support",
            Self::ActiveContextContinuityNotPreserved => "active_context_continuity_not_preserved",
            Self::ResponseFormCoverageIncomplete => "response_form_coverage_incomplete",
            Self::NarrowerBehaviorNotExplained => "narrower_behavior_not_explained",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AppearanceRegistryNextAction {
        match self {
            Self::AppearanceTokenUnstated | Self::ResponseNotBoundToRegistry => {
                M5AppearanceRegistryNextAction::TraceCanonicalRegistry
            }
            Self::AppearancePostureUnclassified
            | Self::PostureMislabeledForSupport
            | Self::ActiveContextContinuityNotPreserved => {
                M5AppearanceRegistryNextAction::InspectPostureOrSurface
            }
            Self::ResponseFormCoverageIncomplete => {
                M5AppearanceRegistryNextAction::CompleteResponseFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::NarrowerBehaviorNotExplained
            | Self::ProofStale => M5AppearanceRegistryNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::AppearanceTokenUnstated | Self::ResponseFormCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::PathVerbUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::AppearancePostureUnclassified => {
                M5PlatformFitDowngradeTrigger::HostPlatformUnstated
            }
            Self::ResponseNotBoundToRegistry => {
                M5PlatformFitDowngradeTrigger::ShortcutNotationDriftedByPlatform
            }
            Self::PostureMislabeledForSupport | Self::NarrowerBehaviorNotExplained => {
                M5PlatformFitDowngradeTrigger::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback
            }
            Self::ActiveContextContinuityNotPreserved => {
                M5PlatformFitDowngradeTrigger::PlatformWordingChangedCommandOrPermissionMeaning
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an appearance-source-provenance entry degraded below a clean, recorded state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceSourceProvenanceEntryDegradeReason {
    /// The canonical registry token name is unstated.
    ProvenanceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The record surface is unclassified (not in the preserved taxonomy).
    RecordSurfaceUnclassified,
    /// The active source or posture is not recorded — not by stable ID, an in-product record surface, and a
    /// source signal.
    SourceOrPostureNotRecorded,
    /// The applied / canonical / accessible response-form coverage of the provenance record is incomplete.
    ProvenancePhrasingCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AppearanceSourceProvenanceEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProvenanceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RecordSurfaceUnclassified,
        Self::SourceOrPostureNotRecorded,
        Self::ProvenancePhrasingCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceTokenUnstated => "provenance_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RecordSurfaceUnclassified => "record_surface_unclassified",
            Self::SourceOrPostureNotRecorded => "source_or_posture_not_recorded",
            Self::ProvenancePhrasingCoverageIncomplete => "provenance_phrasing_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AppearanceRegistryNextAction {
        match self {
            Self::ProvenanceTokenUnstated => M5AppearanceRegistryNextAction::TraceCanonicalRegistry,
            Self::RecordSurfaceUnclassified | Self::SourceOrPostureNotRecorded => {
                M5AppearanceRegistryNextAction::InspectPostureOrSurface
            }
            Self::ProvenancePhrasingCoverageIncomplete => {
                M5AppearanceRegistryNextAction::CompleteResponseFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5AppearanceRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::ProvenanceTokenUnstated => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::RecordSurfaceUnclassified => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SourceOrPostureNotRecorded => {
                M5PlatformFitDowngradeTrigger::PrimaryActionHiddenOnlyInOsChrome
            }
            Self::ProvenancePhrasingCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::PathVerbUnstated
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_appearance_live_apply_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AppearanceLiveApplyEntryResolutionInput {
    /// Stable identity of the appearance-response-registry entry.
    pub entry_id: String,
    /// The stable command ID this response binds to (e.g. `command.appearance.apply`); empty means unstated.
    pub command_id: String,
    /// The canonical registry token name (e.g. `appearance.theme.live`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The theme-contrast-live-change role (from the frozen matrix vocabulary).
    pub appearance_role: M5ThemeContrastLiveChangeRole,
    /// The support posture this entry claims.
    pub posture: M5AppearancePosture,
    /// The render / surface context.
    pub surface_context: M5AppearanceSurfaceContext,
    /// The response forms this entry holds across (must cover applied / canonical / accessible).
    pub response_form_coverage: Vec<M5AppearanceResponseForm>,
    /// The rendered applied-appearance summary (e.g. `dark high-contrast, accent blue, text 125%`).
    pub applied_appearance_summary: String,
    /// The rendered posture label (e.g. `applies live` or `restart required`).
    pub posture_label: String,
    /// True when the response traces to the shared appearance registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the entry preserves active shell / editor / dialog continuity (a hard invariant when `false`).
    pub preserves_active_context_continuity: bool,
    /// True when the change was reapplied live on this surface (must match the claimed posture).
    pub live_reapplied: bool,
    /// True when a narrower-than-live posture (restart-required / unsupported) explains its fallback.
    pub fallback_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe appearance-response-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAppearanceLiveApplyEntry {
    /// Stable identity of the appearance-response-registry entry.
    pub entry_id: String,
    /// The stable command ID this response binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve command identity as platform labels and notation adapt.
    pub semantic_role_preserves_command_identity_under_platform_adaptation: bool,
    /// The theme-contrast-live-change-role token named by the entry.
    pub appearance_role: String,
    /// Whether the appearance role names the disallowed silent-theme-drift token.
    pub appearance_role_silent_drift: bool,
    /// The support-posture token named by the entry.
    pub posture: String,
    /// Whether the support posture is classified into the preserved taxonomy.
    pub posture_is_classified: bool,
    /// Whether the support posture reapplies the change live.
    pub posture_applies_live: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The rendered applied-appearance summary.
    pub applied_appearance_summary: String,
    /// The rendered posture label.
    pub posture_label: String,
    /// The canonical posture label for the entry's support posture.
    pub canonical_posture_label: String,
    /// The response-form tokens covered by the entry.
    pub response_form_coverage: Vec<String>,
    /// Whether the entry covers all three response forms.
    pub covers_all_response_forms: bool,
    /// Whether the recorded posture label and live-reapply state match the claimed support posture.
    pub posture_matches_support: bool,
    /// Whether the entry traces to the shared appearance registry.
    pub bound_to_registry: bool,
    /// Whether the entry preserves active shell / editor / dialog continuity through the change.
    pub preserves_active_context_continuity: bool,
    /// Whether the change was reapplied live on this surface.
    pub live_reapplied: bool,
    /// Whether a narrower-than-live posture explains its fallback.
    pub fallback_explained: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5AppearanceLiveApplyEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AppearanceRegistryNextAction,
    /// Whether the appearance response holds honestly across every surface and channel (clean entry naming
    /// every fact).
    pub appearance_response_honest_across_surfaces_and_channels: bool,
}

impl M5ResolvedAppearanceLiveApplyEntry {
    /// Whether this appearance-response entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_appearance_source_provenance_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AppearanceSourceProvenanceEntryResolutionInput {
    /// Stable identity of the appearance-source-provenance entry.
    pub entry_id: String,
    /// The stable command ID this record binds to; empty means unstated.
    pub command_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The theme-contrast-live-change role this record carries (from the frozen matrix vocabulary).
    pub provenance_role: M5ThemeContrastLiveChangeRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The record surface this source and posture must be recorded on.
    pub record_surface: M5AppearanceRecordSurface,
    /// The render / surface context.
    pub surface_context: M5AppearanceSurfaceContext,
    /// The response forms this entry holds across (must cover applied / canonical / accessible).
    pub response_form_coverage: Vec<M5AppearanceResponseForm>,
    /// The active appearance-source signal label (e.g. `system appearance`); empty means missing.
    pub source_signal_label: String,
    /// The in-product record route the source and posture are recorded through (e.g.
    /// `settings.appearance.source`); empty means missing.
    pub record_route: String,
    /// True when the active source and posture are recorded by stable ID, an in-product record surface, and a
    /// source signal (never hidden from diagnostics).
    pub posture_recorded: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe appearance-source-provenance projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAppearanceSourceProvenanceEntry {
    /// Stable identity of the appearance-source-provenance entry.
    pub entry_id: String,
    /// The stable command ID this record binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The theme-contrast-live-change-role token named by the entry.
    pub provenance_role: String,
    /// Whether the provenance role names the disallowed silent-theme-drift token.
    pub provenance_role_silent_drift: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The record-surface token named by the entry.
    pub record_surface: String,
    /// Whether the record surface is classified into the preserved taxonomy.
    pub record_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The response-form tokens covered by the entry.
    pub response_form_coverage: Vec<String>,
    /// Whether the entry covers all three response forms.
    pub covers_all_response_forms: bool,
    /// The active appearance-source signal label named by the entry.
    pub source_signal_label: String,
    /// The in-product record route named by the entry.
    pub record_route: String,
    /// Whether the active source and posture are recorded by stable ID, an in-product record surface, and a
    /// source signal.
    pub posture_recorded: bool,
    /// Whether the entry provides the complete stable-ID / record-surface / source-signal provenance triple.
    pub provides_complete_provenance_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, recorded state.
    pub degrade_reason: Option<M5AppearanceSourceProvenanceEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AppearanceRegistryNextAction,
    /// Whether the appearance source is recorded on every claimed desktop profile (clean entry naming every
    /// fact).
    pub source_recorded_on_every_profile: bool,
}

impl M5ResolvedAppearanceSourceProvenanceEntry {
    /// Whether this appearance-source-provenance entry reads as a clean, recorded state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SystemAppearanceResolutionError {
    /// The appearance-response-entry id was empty.
    EmptyAppearanceLiveApplyEntryId,
    /// The appearance-source-provenance-entry id was empty.
    EmptyAppearanceSourceProvenanceEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SystemAppearanceResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyAppearanceLiveApplyEntryId => "empty_appearance_live_apply_entry_id",
            Self::EmptyAppearanceSourceProvenanceEntryId => {
                "empty_appearance_source_provenance_entry_id"
            }
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SystemAppearanceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 system-appearance live-apply / source-provenance registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SystemAppearanceResolutionError {}

fn form_tokens(forms: &[M5AppearanceResponseForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_response_forms(forms: &[M5AppearanceResponseForm]) -> bool {
    let present: BTreeSet<M5AppearanceResponseForm> = forms.iter().copied().collect();
    M5AppearanceResponseForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the recorded posture label and live-reapply state match the claimed support posture: a live-apply
/// entry must render the `applies live` label and have reapplied live, and a restart-required or unsupported
/// entry must render its own label and never claim to have reapplied live. An unclassified, empty-labelled, or
/// live-inconsistent entry never matches.
pub fn appearance_response_matches_posture(
    posture: M5AppearancePosture,
    live_reapplied: bool,
    posture_label: &str,
) -> bool {
    if !posture.is_classified() || posture_label.trim().is_empty() {
        return false;
    }
    let label_matches = posture_label
        .trim()
        .eq_ignore_ascii_case(posture.canonical_posture_label());
    let live_matches = posture.applies_live() == live_reapplied;
    label_matches && live_matches
}

/// Resolves an appearance-response-registry entry so it stays bound to the shared appearance registry: the
/// entry names its canonical token, semantic role, appearance role, and support posture, covers all three
/// response forms, records a posture label and live-reapply state that match the claimed posture, preserves
/// active-context continuity, and explains any narrower-than-live behavior.
pub fn resolve_appearance_live_apply_entry(
    input: M5AppearanceLiveApplyEntryResolutionInput,
) -> Result<M5ResolvedAppearanceLiveApplyEntry, M5SystemAppearanceResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SystemAppearanceResolutionError::EmptyAppearanceLiveApplyEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.applied_appearance_summary)
        || string_is_forbidden(&input.posture_label)
    {
        return Err(M5SystemAppearanceResolutionError::ForbiddenMaterial);
    }

    let appearance_role_silent_drift = matches!(
        input.appearance_role,
        M5ThemeContrastLiveChangeRole::SilentThemeDriftDisallowed
    );
    let all_forms = covers_all_response_forms(&input.response_form_coverage);
    let matches_support = appearance_response_matches_posture(
        input.posture,
        input.live_reapplied,
        &input.posture_label,
    );
    let narrower_unexplained = !input.posture.applies_live() && !input.fallback_explained;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5AppearanceLiveApplyEntryDegradeReason::AppearanceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5AppearanceLiveApplyEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.posture.is_classified() {
        Some(M5AppearanceLiveApplyEntryDegradeReason::AppearancePostureUnclassified)
    } else if appearance_role_silent_drift || !input.bound_to_registry {
        Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseNotBoundToRegistry)
    } else if !matches_support {
        Some(M5AppearanceLiveApplyEntryDegradeReason::PostureMislabeledForSupport)
    } else if !input.preserves_active_context_continuity {
        Some(M5AppearanceLiveApplyEntryDegradeReason::ActiveContextContinuityNotPreserved)
    } else if !all_forms {
        Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseFormCoverageIncomplete)
    } else if narrower_unexplained {
        Some(M5AppearanceLiveApplyEntryDegradeReason::NarrowerBehaviorNotExplained)
    } else if !input.proof_fresh {
        Some(M5AppearanceLiveApplyEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AppearanceRegistryNextAction::ExpandResponseMeaning,
    };

    Ok(M5ResolvedAppearanceLiveApplyEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_command_identity_under_platform_adaptation: input
            .semantic_role
            .must_preserve_command_identity_under_platform_adaptation(),
        appearance_role: input.appearance_role.as_str().to_owned(),
        appearance_role_silent_drift,
        posture: input.posture.as_str().to_owned(),
        posture_is_classified: input.posture.is_classified(),
        posture_applies_live: input.posture.applies_live(),
        surface_context: input.surface_context.as_str().to_owned(),
        applied_appearance_summary: input.applied_appearance_summary,
        posture_label: input.posture_label,
        canonical_posture_label: input.posture.canonical_posture_label().to_owned(),
        response_form_coverage: form_tokens(&input.response_form_coverage),
        covers_all_response_forms: all_forms,
        posture_matches_support: matches_support,
        bound_to_registry: input.bound_to_registry,
        preserves_active_context_continuity: input.preserves_active_context_continuity,
        live_reapplied: input.live_reapplied,
        fallback_explained: input.fallback_explained,
        degrade_reason,
        next_action,
        appearance_response_honest_across_surfaces_and_channels: degrade_reason.is_none(),
    })
}

/// Resolves an appearance-source-provenance entry so the active appearance source and posture stay recorded:
/// the entry names its canonical token, provenance role, semantic role, and record surface, covers all three
/// response forms, provides the stable-ID / record-surface / source-signal provenance triple, and degrades
/// honestly when the source or posture would not be recorded.
pub fn resolve_appearance_source_provenance_entry(
    input: M5AppearanceSourceProvenanceEntryResolutionInput,
) -> Result<M5ResolvedAppearanceSourceProvenanceEntry, M5SystemAppearanceResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SystemAppearanceResolutionError::EmptyAppearanceSourceProvenanceEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.source_signal_label)
        || string_is_forbidden(&input.record_route)
    {
        return Err(M5SystemAppearanceResolutionError::ForbiddenMaterial);
    }

    let provenance_role_silent_drift = matches!(
        input.provenance_role,
        M5ThemeContrastLiveChangeRole::SilentThemeDriftDisallowed
    );
    let all_forms = covers_all_response_forms(&input.response_form_coverage);
    let provides_triple = input.record_surface.is_classified()
        && !input.command_id.trim().is_empty()
        && !input.source_signal_label.trim().is_empty()
        && !input.record_route.trim().is_empty()
        && input.posture_recorded;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::ProvenanceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.record_surface.is_classified() {
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::RecordSurfaceUnclassified)
    } else if provenance_role_silent_drift || !provides_triple {
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded)
    } else if !all_forms {
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::ProvenancePhrasingCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5AppearanceSourceProvenanceEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AppearanceRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedAppearanceSourceProvenanceEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        provenance_role: input.provenance_role.as_str().to_owned(),
        provenance_role_silent_drift,
        semantic_role: input.semantic_role.as_str().to_owned(),
        record_surface: input.record_surface.as_str().to_owned(),
        record_surface_is_classified: input.record_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        response_form_coverage: form_tokens(&input.response_form_coverage),
        covers_all_response_forms: all_forms,
        source_signal_label: input.source_signal_label,
        record_route: input.record_route,
        posture_recorded: input.posture_recorded,
        provides_complete_provenance_triple: provides_triple,
        degrade_reason,
        next_action,
        source_recorded_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved appearance-response and appearance-source
/// provenance entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SystemAppearanceRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5PlatformFitQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5PlatformFitDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5PlatformFitRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5PlatformFitAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5AppearanceRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5AppearanceRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    /// Resolved appearance-response-registry examples.
    pub appearance_live_apply_entries: Vec<M5ResolvedAppearanceLiveApplyEntry>,
    /// Resolved appearance-source-provenance examples.
    pub appearance_source_provenance_entries: Vec<M5ResolvedAppearanceSourceProvenanceEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical file-path-and-reveal domain
    /// schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a live appearance change never corrupts focus, layout, or meaning on a protected path.
    /// MUST be `false`.
    pub appearance_change_corrupts_focus_layout_or_meaning_on_protected_path: bool,
    /// Hard invariant: a live change never forces a mystery repaint or resets local context. MUST be `false`.
    pub live_change_forces_mystery_repaint_or_resets_context: bool,
    /// Hard invariant: appearance response is never hand-copied per platform instead of tracing to the
    /// registry. MUST be `false`.
    pub appearance_response_hardcoded_instead_of_registry: bool,
    /// Hard invariant: diagnostics or exports can always distinguish live-apply from restart-required or
    /// unsupported. MUST be `false`.
    pub diagnostics_or_export_cannot_distinguish_live_from_restart: bool,
}

impl M5SystemAppearanceRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AppearanceRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AppearanceRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AppearanceRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5AppearanceRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.appearance_change_corrupts_focus_layout_or_meaning_on_protected_path
            && !self.live_change_forces_mystery_repaint_or_resets_context
            && !self.appearance_response_hardcoded_instead_of_registry
            && !self.diagnostics_or_export_cannot_distinguish_live_from_restart
    }

    /// True when a clean appearance-response entry preserves registry-bound behavior: it traces to the
    /// registry, never names the disallowed silent-drift role, keeps a classified posture, matches the claimed
    /// support posture, preserves active-context continuity, covers all three response forms, and explains any
    /// narrower-than-live behavior.
    fn response_is_honest(ex: &M5ResolvedAppearanceLiveApplyEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && !ex.appearance_role_silent_drift
                && ex.posture_is_classified
                && ex.posture_matches_support
                && ex.preserves_active_context_continuity
                && ex.covers_all_response_forms
                && (ex.posture_applies_live || ex.fallback_explained))
    }

    /// True when a clean appearance-source-provenance entry preserves recording: it keeps a classified record
    /// surface, never names the disallowed silent-drift role, provides the provenance triple, and covers all
    /// three response forms.
    fn provenance_is_honest(ex: &M5ResolvedAppearanceSourceProvenanceEntry) -> bool {
        !ex.is_clean()
            || (ex.record_surface_is_classified
                && !ex.provenance_role_silent_drift
                && ex.provides_complete_provenance_triple
                && ex.covers_all_response_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.appearance_live_apply_entries
            .iter()
            .all(Self::response_is_honest)
            && self
                .appearance_source_provenance_entries
                .iter()
                .all(Self::provenance_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Theme-contrast-live-change-role tokens (bound from the frozen matrix).
    pub appearance_roles: Vec<String>,
    /// Response-form tokens (minted by this lane).
    pub response_forms: Vec<String>,
    /// Support-posture tokens (minted by this lane).
    pub postures: Vec<String>,
    /// Record-surface tokens (minted by this lane).
    pub record_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Appearance-response-entry degrade-reason tokens.
    pub appearance_live_apply_degrade_reasons: Vec<String>,
    /// Appearance-source-provenance-entry degrade-reason tokens.
    pub appearance_source_provenance_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SystemAppearanceRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5PlatformFitRole::ALL, |v| v.as_str()),
            appearance_roles: tokens(&M5ThemeContrastLiveChangeRole::ALL, |v| v.as_str()),
            response_forms: tokens(&M5AppearanceResponseForm::ALL, |v| v.as_str()),
            postures: tokens(&M5AppearancePosture::ALL, |v| v.as_str()),
            record_surfaces: tokens(&M5AppearanceRecordSurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5AppearanceSurfaceContext::ALL, |v| v.as_str()),
            appearance_live_apply_degrade_reasons: tokens(
                &M5AppearanceLiveApplyEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            appearance_source_provenance_degrade_reasons: tokens(
                &M5AppearanceSourceProvenanceEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5AppearanceRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5AppearanceRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AppearanceRegistryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5PlatformFitConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesGovernanceReview {
    /// The appearance registry names a canonical token, appearance role, and support posture for every entry.
    pub appearance_registry_names_token_role_and_posture: bool,
    /// Live theme / contrast / accent / text-scale changes apply from the shared registry, not per-surface
    /// behavior.
    pub live_changes_applied_from_shared_registry: bool,
    /// The canonical live-versus-fallback posture truth is kept explicit on every response entry.
    pub live_versus_fallback_posture_truth_kept_explicit: bool,
    /// Restart-required and unsupported postures explain their narrower behavior on every claimed profile.
    pub narrower_behavior_explained_on_every_profile: bool,
    /// Active shell / editor / dialog continuity is preserved through every live change.
    pub active_context_continuity_preserved_through_live_change: bool,
    /// The active appearance source and posture are recorded in settings, diagnostics, and support exports.
    pub appearance_source_and_posture_recorded_in_settings_diagnostics_and_export: bool,
    /// Every response and provenance entry covers the applied / canonical / accessible response forms.
    pub every_entry_covers_all_response_forms: bool,
    /// Appearance response stays bound to one registry rather than hand-copied per platform.
    pub appearance_response_bound_to_single_registry_not_hand_copied: bool,
    /// Diagnostics and support exports are generated from the same appearance registry.
    pub diagnostics_and_export_generated_from_registry: bool,
    /// A mislabeled posture or an unrecorded appearance source is caught by fixtures before release evidence
    /// turns green.
    pub posture_or_provenance_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesConsumerProjection {
    /// The shell (theme / chrome repaint) consumes the shared appearance registries.
    pub shell_consumes_shared_registries: bool,
    /// The settings (appearance preview / source record) consumes the shared registries.
    pub settings_consumes_shared_registries: bool,
    /// Docs and help consume the shared registries.
    pub docs_help_consumes_shared_registries: bool,
    /// Onboarding and CLI export consume the shared registries.
    pub onboarding_and_cli_consume_shared_registries: bool,
    /// Appearance response traces back to one canonical appearance domain contract.
    pub appearance_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical appearance / provenance registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting platform-fit audit for the lane.
    pub platform_fit_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SystemAppearanceRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SystemAppearanceRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SystemAppearanceRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SystemAppearanceRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SystemAppearanceRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SystemAppearanceRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SystemAppearanceRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SystemAppearanceRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 system-appearance live-apply and appearance-source-provenance registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SystemAppearanceRegistriesPacket {
    /// Record kind; must equal [`M5_SYSTEM_APPEARANCE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SystemAppearanceRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SystemAppearanceRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SystemAppearanceRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SystemAppearanceRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SystemAppearanceRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SystemAppearanceRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SystemAppearanceRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SystemAppearanceRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SYSTEM_APPEARANCE_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5SystemAppearanceRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SYSTEM_APPEARANCE_REGISTRIES_RECORD_KIND {
            violations.push(M5SystemAppearanceRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5SystemAppearanceRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SystemAppearanceRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5SystemAppearanceRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 system-appearance live-apply / source-provenance registries packet serializes",
        )) {
            violations.push(M5SystemAppearanceRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect(
            "m5 system-appearance live-apply / source-provenance registries packet serializes",
        )
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,appearance_live_apply_entries,appearance_source_provenance_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .appearance_live_apply_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.appearance_source_provenance_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.appearance_live_apply_entries.len(),
                row.appearance_source_provenance_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 System-Appearance Live-Apply and Appearance-Source-Provenance Registries\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Support postures: {}\n",
            self.vocabulary_set.postures.join(", ")
        ));
        out.push_str(&format!(
            "- Response forms: {}\n",
            self.vocabulary_set.response_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Response entries: {} / provenance entries: {}\n",
                row.appearance_live_apply_entries.len(),
                row.appearance_source_provenance_entries.len()
            ));
        }
        out
    }

    /// Deterministic diagnostics / support posture table generated from the registry, so diagnostics render
    /// the same command / posture / applied-appearance / label truth the resolvers produced rather than a
    /// hand-copied panel. Only clean, registry-bound response entries are listed.
    pub fn render_appearance_posture_table(&self) -> String {
        let mut out = String::new();
        out.push_str("| command_id | posture | applied_appearance | posture_label | surface |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.appearance_live_apply_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | {} | {} |\n",
                    ex.command_id,
                    ex.posture,
                    ex.applied_appearance_summary,
                    ex.posture_label,
                    ex.surface_context
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SystemAppearanceRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SystemAppearanceRegistriesViolation>),
}

impl fmt::Display for M5SystemAppearanceRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 system-appearance live-apply / source-provenance registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 system-appearance live-apply / source-provenance registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SystemAppearanceRegistriesArtifactError {}

/// Validation failures emitted by [`M5SystemAppearanceRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SystemAppearanceRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at the canonical file-path-and-reveal domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, posture-mislabeled, continuity-losing,
    /// form-incomplete, or a provenance entry missing the recording triple).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Live-or-explained appearance response is not proven across surfaces: clean response entries do not cover
    /// the appearance / command-stability semantic-role families or the first shell / editor / dialog /
    /// settings / docs surfaces, no hand-copied example degrades, or a clean entry is not bound to the
    /// registry.
    LiveOrExplainedAcrossSurfacesNotProven,
    /// The active appearance source is not proven recorded across profiles: clean provenance entries do not
    /// cover the settings / diagnostics / support-export record surfaces with full response-form coverage while
    /// providing the recording triple, no not-recorded or phrasing-incomplete example degrades, or a clean
    /// entry is missing the triple.
    AppearanceSourceRecordedOnEveryProfileNotProven,
    /// A mislabeled posture or unrecorded appearance source is not detectable: no posture-mislabeled example
    /// and no source-not-recorded example degrade, clean entries do not trace to the registry, or a clean entry
    /// is mislabeled for its posture.
    MislabeledPostureOrUnrecordedSourceDetectableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SystemAppearanceRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::LiveOrExplainedAcrossSurfacesNotProven => {
                "live_or_explained_across_surfaces_not_proven"
            }
            Self::AppearanceSourceRecordedOnEveryProfileNotProven => {
                "appearance_source_recorded_on_every_profile_not_proven"
            }
            Self::MislabeledPostureOrUnrecordedSourceDetectableNotProven => {
                "mislabeled_posture_or_unrecorded_source_detectable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_system_appearance_live_apply_and_source_provenance_registries_export(
) -> Result<M5SystemAppearanceRegistriesPacket, M5SystemAppearanceRegistriesArtifactError> {
    let packet: M5SystemAppearanceRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-system-appearance-live-apply-and-source-provenance-registries-proof/support_export.json"
    )))
    .map_err(M5SystemAppearanceRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SystemAppearanceRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_REF,
        M5_SYSTEM_APPEARANCE_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SystemAppearanceRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SystemAppearanceRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5SystemAppearanceRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SystemAppearanceRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5SystemAppearanceRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_FILE_PATH_AND_REVEAL_SCHEMA_REF) {
            violations.push(M5SystemAppearanceRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.appearance_live_apply_entries.is_empty()
            || row.appearance_source_provenance_entries.is_empty()
        {
            violations.push(M5SystemAppearanceRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5SystemAppearanceRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5SystemAppearanceRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.appearance_registry_names_token_role_and_posture,
        review.live_changes_applied_from_shared_registry,
        review.live_versus_fallback_posture_truth_kept_explicit,
        review.narrower_behavior_explained_on_every_profile,
        review.active_context_continuity_preserved_through_live_change,
        review.appearance_source_and_posture_recorded_in_settings_diagnostics_and_export,
        review.every_entry_covers_all_response_forms,
        review.appearance_response_bound_to_single_registry_not_hand_copied,
        review.diagnostics_and_export_generated_from_registry,
        review.posture_or_provenance_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5SystemAppearanceRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.settings_consumes_shared_registries,
        projection.docs_help_consumes_shared_registries,
        projection.onboarding_and_cli_consume_shared_registries,
        projection.appearance_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5SystemAppearanceRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SystemAppearanceRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.platform_fit_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SystemAppearanceRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5SystemAppearanceRegistriesPacket,
    violations: &mut Vec<M5SystemAppearanceRegistriesViolation>,
) {
    let responses = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.appearance_live_apply_entries.iter())
    };
    let provenance = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.appearance_source_provenance_entries.iter())
    };

    // AC1: claimed desktop profiles either apply host appearance changes live or clearly explain the narrower
    // supported behavior across surfaces. Clean response entries cover the appearance / command-stability
    // semantic-role families and the first shell / editor / dialog / settings / docs surfaces, a hand-copied
    // example degrades, and no clean entry is unbound.
    let clean_semantic_roles: BTreeSet<String> = responses()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = responses()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = [
        M5PlatformFitRole::Appearance.as_str(),
        M5PlatformFitRole::CommandStability.as_str(),
    ]
    .iter()
    .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5AppearanceSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let hand_copied_degrades = responses().any(|ex| {
        ex.degrade_reason
            == Some(M5AppearanceLiveApplyEntryDegradeReason::ResponseNotBoundToRegistry)
    });
    let no_clean_unbound = !responses().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    if !(semantic_families_covered
        && first_surfaces_covered
        && hand_copied_degrades
        && no_clean_unbound)
    {
        violations
            .push(M5SystemAppearanceRegistriesViolation::LiveOrExplainedAcrossSurfacesNotProven);
    }

    // AC2: diagnostics and support exports can distinguish live-apply from restart-required or unsupported.
    // Clean provenance entries cover every canonical record surface with full response-form coverage while
    // providing the recording triple, a source-not-recorded example degrades, a phrasing-incomplete example
    // degrades, and no clean entry is missing the triple.
    let clean_record_surfaces: BTreeSet<String> = provenance()
        .filter(|ex| {
            ex.is_clean()
                && ex.record_surface_is_classified
                && ex.provides_complete_provenance_triple
                && ex.covers_all_response_forms
        })
        .map(|ex| ex.record_surface.clone())
        .collect();
    let record_surfaces_covered = M5AppearanceRecordSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_record_surfaces.contains(s.as_str()));
    let not_recorded_degrades = provenance().any(|ex| {
        ex.degrade_reason
            == Some(M5AppearanceSourceProvenanceEntryDegradeReason::SourceOrPostureNotRecorded)
    });
    let phrasing_incomplete_degrades = provenance().any(|ex| {
        ex.degrade_reason
            == Some(M5AppearanceSourceProvenanceEntryDegradeReason::ProvenancePhrasingCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !provenance().any(|ex| ex.is_clean() && !ex.provides_complete_provenance_triple);
    if !(record_surfaces_covered
        && not_recorded_degrades
        && phrasing_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5SystemAppearanceRegistriesViolation::AppearanceSourceRecordedOnEveryProfileNotProven,
        );
    }

    // AC3: live changes do not corrupt meaning, and review fixtures fail when a surface mislabels its posture
    // or hides its appearance source. A posture-mislabeled example and a source-not-recorded example both
    // degrade, at least one clean response and one clean provenance entry trace to the registry, no clean
    // response is unbound, and no clean response is mislabeled for its posture.
    let mislabeled_degrades = responses().any(|ex| {
        ex.degrade_reason
            == Some(M5AppearanceLiveApplyEntryDegradeReason::PostureMislabeledForSupport)
    });
    let bound_response = responses().any(|ex| ex.is_clean() && ex.bound_to_registry);
    let bound_provenance =
        provenance().any(|ex| ex.is_clean() && ex.provides_complete_provenance_triple);
    let no_clean_mislabeled = !responses().any(|ex| ex.is_clean() && !ex.posture_matches_support);
    if !(mislabeled_degrades
        && not_recorded_degrades
        && bound_response
        && bound_provenance
        && no_clean_unbound
        && no_clean_mislabeled)
    {
        violations.push(
            M5SystemAppearanceRegistriesViolation::MislabeledPostureOrUnrecordedSourceDetectableNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The platform-fit families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5PlatformFitFamily; 1] =
    [M5PlatformFitFamily::ThemeContrastLiveChange];
