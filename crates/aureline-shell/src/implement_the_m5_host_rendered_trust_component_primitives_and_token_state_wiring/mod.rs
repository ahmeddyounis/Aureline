//! One host-rendered layer that binds the M5 trust / config / history component
//! families to canonical primitives with shared token / state wiring, so desktop,
//! companion, and extension-backed first consumers cannot each restyle or relabel
//! them into different meanings.
//!
//! Aureline's frozen component matrix
//! ([`crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix`])
//! names six governed high-trust component families, and the four primitive lanes
//! that followed it turned each family into a working resolver: the settings row
//! ([`crate::implement_the_m5_settings_row_effective_value_source_pill_and_lock_state_primitive`]),
//! the capability sheet
//! ([`crate::implement_the_m5_capability_sheet_consequence_grouping_transitive_scope_and_reconsent_primitive`]),
//! the evidence / activity row
//! ([`crate::implement_the_m5_evidence_and_activity_timeline_row_primitive`]), and the
//! chronology group / narrative card / export preview
//! ([`crate::implement_the_m5_chronology_group_narrative_card_and_export_preview_primitive`]).
//! Each of those lanes proved its family *resolves* the same way everywhere. This
//! module takes the remaining step the exit gate demands: it binds those families to
//! canonical *host-rendered* primitives and pins the design-token / state wiring so
//! the same component cannot be restyled or relabelled into a different meaning as it
//! moves between the desktop app, the companion surface, and an extension host.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_binding`] — that takes one first consumer's declared
//!    render of a primitive on a host surface (which host runtime, which render mode,
//!    which shared token / state slots it wired, which cosmetic aspects it restyled,
//!    and — critically — whether it overrode any fixed contract part) and produces
//!    one [`M5ResolvedBinding`] carrying whether the consumer renders through the
//!    canonical primitive (or an audited wrapper) instead of a bespoke local variant,
//!    whether it wired every fixed token slot, whether its restyle stayed within the
//!    cosmetic bounds, and a single [`M5BindingConformance`] verdict. The resolver
//!    never blesses a bespoke local variant, never lets a fixed token slot go
//!    unwired, and never lets a restyle reach into a contract part that carries
//!    meaning.
//! 2. A parity matrix — [`M5HostRenderedPrimitivePacket`] — that binds one row per
//!    canonical host-rendered primitive family (settings row, capability sheet,
//!    event / history row, timeline group, and chronology export preview) to the
//!    frozen component families it renders, the host surfaces it renders on, the
//!    fixed token slots and contract parts it pins, the cosmetic aspects it lets a
//!    contributor restyle, worked binding cases, and a naming-parity block, so the
//!    component demos, the screenshots, and the support / export packet all reference
//!    the same primitive family names from one shared model.
//!
//! The frozen component families ([`M5TrustComponentFamily`]), the source pills
//! ([`M5SettingSourcePill`]), the provenance badges ([`M5ProvenanceBadge`]), the
//! non-visual accessibility routes ([`M5TrustAccessibilityRoute`]), the qualification
//! classes ([`M5TrustQualificationClass`]), and the downgrade triggers
//! ([`M5TrustComponentDowngradeTrigger`]) are reused verbatim from the frozen
//! component matrix; the shell topology — zones, responsive classes, window classes,
//! and consumer surfaces — is reused from the frozen shell-zone matrix. This module
//! mints new vocabulary only for what the frozen matrix left implicit about *binding*
//! a family to a host-rendered primitive: the host-rendered primitive families, the
//! host surfaces, the render modes, the shared design-token slots, the restylable
//! cosmetic aspects, the fixed contract parts, and the binding conformance verdicts.
//! No M5 first consumer invents a second row grammar or restyles a contract part into
//! a different meaning.
//!
//! Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, credentials, and
//! user text bodies stay outside the support boundary; opaque, export-safe reprs are
//! the only material carried.
//!
//! The boundary schema is
//! [`schemas/ui/m5-host-rendered-primitives.schema.json`](../../../../schemas/ui/m5-host-rendered-primitives.schema.json)
//! and the contract doc is
//! [`docs/components/m5_host_rendered_primitives_contract.md`](../../../../docs/components/m5_host_rendered_primitives_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-host-rendered-primitives/`](../../../../fixtures/ui/m5-host-rendered-primitives/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed,
    seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed,
    seeded_m5_host_rendered_primitive_packet, M5_HOST_RENDERED_PRIMITIVE_PACKET_ID,
};

// The frozen component families, source pills, provenance badges, accessibility
// routes, qualification classes, and downgrade triggers are frozen once, in the
// trust-chronology component matrix. This binding layer reuses them verbatim so it
// never invents a parallel component grammar.
pub use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix::{
    M5ProvenanceBadge, M5SettingSourcePill, M5TrustAccessibilityRoute,
    M5TrustComponentDowngradeTrigger, M5TrustComponentFamily, M5TrustQualificationClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5HostRenderedPrimitivePacket`].
pub const M5_HOST_RENDERED_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_host_rendered_trust_component_primitives_and_token_state_wiring";

/// Schema version for M5 host-rendered-primitive records.
pub const M5_HOST_RENDERED_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the host-rendered-primitive boundary schema.
pub const M5_HOST_RENDERED_SCHEMA_REF: &str = "schemas/ui/m5-host-rendered-primitives.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_HOST_RENDERED_DOC_REF: &str =
    "docs/components/m5_host_rendered_primitives_contract.md";

/// Repo-relative path of the frozen shell-zone schema this layer binds against.
pub const M5_HOST_RENDERED_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen component matrix this layer binds.
pub const M5_HOST_RENDERED_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-trust-chronology-components.schema.json";

/// Repo-relative path of the settings-row primitive this layer host-renders.
pub const M5_HOST_RENDERED_SETTINGS_ROW_REF: &str = "schemas/ui/m5-settings-row.schema.json";

/// Repo-relative path of the capability-sheet primitive this layer host-renders.
pub const M5_HOST_RENDERED_CAPABILITY_SHEET_REF: &str =
    "schemas/ui/m5-capability-sheet.schema.json";

/// Repo-relative path of the evidence-row primitive this layer host-renders.
pub const M5_HOST_RENDERED_EVIDENCE_ROW_REF: &str = "schemas/ui/m5-evidence-row.schema.json";

/// Repo-relative path of the chronology-export-preview primitive this layer
/// host-renders.
pub const M5_HOST_RENDERED_CHRONOLOGY_PREVIEW_REF: &str =
    "schemas/ui/m5-chronology-export-preview.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_HOST_RENDERED_FIXTURE_DIR: &str = "fixtures/ui/m5-host-rendered-primitives";

/// Repo-relative path of the checked support-export artifact.
pub const M5_HOST_RENDERED_ARTIFACT_REF: &str =
    "artifacts/release/m5-host-rendered-primitives-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_HOST_RENDERED_CSV_REF: &str =
    "artifacts/release/m5-host-rendered-primitives-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_HOST_RENDERED_REPORT_REF: &str = "artifacts/components/m5-host-rendered-primitives.md";

/// One canonical host-rendered primitive family. These are the primitives the goal
/// names — a settings row, a capability sheet, an event / history row, a timeline
/// group, and a chronology export preview — each bound to one or more frozen
/// component families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostRenderedPrimitiveFamily {
    /// A settings row carrying effective-versus-configured truth.
    SettingsRow,
    /// A permission / capability sheet grouping requests by consequence.
    CapabilitySheet,
    /// A single event / history row in an activity or evidence timeline.
    EventHistoryRow,
    /// A timeline group (and its narrative summary card) over a chronology span.
    TimelineGroup,
    /// A chronology export preview showing what will be exported.
    ChronologyExportPreview,
}

impl M5HostRenderedPrimitiveFamily {
    /// Every host-rendered primitive family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SettingsRow,
        Self::CapabilitySheet,
        Self::EventHistoryRow,
        Self::TimelineGroup,
        Self::ChronologyExportPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsRow => "settings_row",
            Self::CapabilitySheet => "capability_sheet",
            Self::EventHistoryRow => "event_history_row",
            Self::TimelineGroup => "timeline_group",
            Self::ChronologyExportPreview => "chronology_export_preview",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SettingsRow => "Settings Row",
            Self::CapabilitySheet => "Capability Sheet",
            Self::EventHistoryRow => "Event / History Row",
            Self::TimelineGroup => "Timeline Group",
            Self::ChronologyExportPreview => "Chronology Export Preview",
        }
    }

    /// The frozen component families this host-rendered primitive renders. The union
    /// across all five primitives covers every [`M5TrustComponentFamily`]: the
    /// timeline-group primitive host-renders both the timeline group and its narrative
    /// summary card.
    pub fn bound_component_families(self) -> Vec<M5TrustComponentFamily> {
        match self {
            Self::SettingsRow => vec![M5TrustComponentFamily::SettingsRow],
            Self::CapabilitySheet => vec![M5TrustComponentFamily::CapabilitySheet],
            Self::EventHistoryRow => vec![M5TrustComponentFamily::EventHistoryRow],
            Self::TimelineGroup => vec![
                M5TrustComponentFamily::TimelineGroup,
                M5TrustComponentFamily::NarrativeSummaryCard,
            ],
            Self::ChronologyExportPreview => vec![M5TrustComponentFamily::ChronologyExportPreview],
        }
    }

    /// The canonical shell zone this primitive attaches to. Settings rows live in the
    /// main workspace, capability sheets in the transient overlay, and the chronology
    /// primitives in the bottom panel.
    pub const fn canonical_zone(self) -> M5ShellZoneSlot {
        match self {
            Self::SettingsRow => M5ShellZoneSlot::MainWorkspace,
            Self::CapabilitySheet => M5ShellZoneSlot::TransientOverlay,
            Self::EventHistoryRow | Self::TimelineGroup | Self::ChronologyExportPreview => {
                M5ShellZoneSlot::BottomPanel
            }
        }
    }

    /// The shared design-token / state slots this primitive pins as fixed. A first
    /// consumer must wire every one of these through the host-rendered layer; it may
    /// not swap them for a bespoke local encoding. Source pills are fixed only for the
    /// settings row; provenance badges only for the chronology families; severity /
    /// state colour, disclosure affordance, focus ring, state label, and density
    /// metric are fixed everywhere.
    pub fn fixed_token_slots(self) -> Vec<M5DesignTokenSlot> {
        let mut slots = vec![
            M5DesignTokenSlot::SeverityStateColor,
            M5DesignTokenSlot::DisclosureAffordance,
            M5DesignTokenSlot::FocusRing,
            M5DesignTokenSlot::StateLabel,
            M5DesignTokenSlot::DensityMetric,
        ];
        match self {
            Self::SettingsRow => slots.insert(0, M5DesignTokenSlot::SourcePill),
            Self::CapabilitySheet => {}
            Self::EventHistoryRow | Self::TimelineGroup | Self::ChronologyExportPreview => {
                slots.insert(0, M5DesignTokenSlot::ProvenanceBadge)
            }
        }
        slots
    }
}

/// A host runtime that renders the canonical primitive. Every primitive renders on
/// all three so its meaning cannot drift as it moves between runtimes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrimitiveHostSurface {
    /// The native desktop application.
    DesktopApp,
    /// The companion (adjacent / secondary) surface.
    CompanionSurface,
    /// An extension host rendering an extension-backed lane.
    ExtensionHost,
}

impl M5PrimitiveHostSurface {
    /// Every host surface, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DesktopApp,
        Self::CompanionSurface,
        Self::ExtensionHost,
    ];

    /// Every host surface is mandatory: a primitive must render on all three so its
    /// meaning cannot drift between runtimes.
    pub const MANDATORY: [Self; 3] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopApp => "desktop_app",
            Self::CompanionSurface => "companion_surface",
            Self::ExtensionHost => "extension_host",
        }
    }
}

/// How a first consumer renders a primitive. Only the canonical host-rendered path
/// and an audited wrapper are permitted; a bespoke local variant is drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrimitiveRenderMode {
    /// Rendered directly through the canonical host-rendered primitive.
    HostRenderedCanonical,
    /// Rendered through an audited wrapper over the canonical primitive.
    AuditedWrapper,
    /// Rendered by a bespoke local variant — the drift this lane prevents.
    BespokeLocalVariant,
}

impl M5PrimitiveRenderMode {
    /// Every render mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::HostRenderedCanonical,
        Self::AuditedWrapper,
        Self::BespokeLocalVariant,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostRenderedCanonical => "host_rendered_canonical",
            Self::AuditedWrapper => "audited_wrapper",
            Self::BespokeLocalVariant => "bespoke_local_variant",
        }
    }

    /// `true` when this mode renders through the canonical primitive (directly or via
    /// an audited wrapper) rather than a bespoke local variant.
    pub const fn renders_through_canonical(self) -> bool {
        matches!(self, Self::HostRenderedCanonical | Self::AuditedWrapper)
    }
}

/// A shared design-token / state slot the host-rendered layer wires. These carry
/// meaning — a source pill, a provenance badge, a severity / state colour, a
/// disclosure affordance, a focus ring, a typed-state label, and a density metric —
/// so they are pinned by the host layer, not restyled per consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesignTokenSlot {
    /// The settings source pill token (which source produced the effective value).
    SourcePill,
    /// The provenance badge token (who / what initiated a chronology event).
    ProvenanceBadge,
    /// The severity / state colour token.
    SeverityStateColor,
    /// The disclosure affordance token (expand / reveal control).
    DisclosureAffordance,
    /// The focus ring token.
    FocusRing,
    /// The typed-state label token.
    StateLabel,
    /// The density metric token (compact / comfortable spacing scale).
    DensityMetric,
}

impl M5DesignTokenSlot {
    /// Every token slot, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SourcePill,
        Self::ProvenanceBadge,
        Self::SeverityStateColor,
        Self::DisclosureAffordance,
        Self::FocusRing,
        Self::StateLabel,
        Self::DensityMetric,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourcePill => "source_pill",
            Self::ProvenanceBadge => "provenance_badge",
            Self::SeverityStateColor => "severity_state_color",
            Self::DisclosureAffordance => "disclosure_affordance",
            Self::FocusRing => "focus_ring",
            Self::StateLabel => "state_label",
            Self::DensityMetric => "density_metric",
        }
    }
}

/// A cosmetic aspect a contributor / extension MAY restyle. These carry no meaning,
/// so restyling them never drifts the component's semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestylableAspect {
    /// The spacing scale.
    SpacingScale,
    /// The corner radius.
    CornerRadius,
    /// The accent tint.
    AccentTint,
    /// The typography family.
    TypographyFamily,
    /// The icon set.
    IconSet,
    /// The elevation / shadow.
    ElevationShadow,
    /// The motion curve.
    MotionCurve,
}

impl M5RestylableAspect {
    /// Every restylable aspect, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SpacingScale,
        Self::CornerRadius,
        Self::AccentTint,
        Self::TypographyFamily,
        Self::IconSet,
        Self::ElevationShadow,
        Self::MotionCurve,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpacingScale => "spacing_scale",
            Self::CornerRadius => "corner_radius",
            Self::AccentTint => "accent_tint",
            Self::TypographyFamily => "typography_family",
            Self::IconSet => "icon_set",
            Self::ElevationShadow => "elevation_shadow",
            Self::MotionCurve => "motion_curve",
        }
    }
}

/// A fixed contract part a first consumer must NOT override or remove. These carry
/// the component's meaning; overriding one is drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrimitiveContractPart {
    /// The stable identity label.
    IdentityLabel,
    /// The typed current state.
    TypedState,
    /// The provenance / source attribution.
    ProvenanceOrSourceAttribution,
    /// The severity / state semantics (never colour-only).
    SeveritySemantics,
    /// The disclosure control that reveals scope / detail.
    DisclosureControl,
    /// The non-visual keyboard route.
    KeyboardRoute,
    /// The audit / export anchor.
    AuditExportAnchor,
}

impl M5PrimitiveContractPart {
    /// Every contract part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::IdentityLabel,
        Self::TypedState,
        Self::ProvenanceOrSourceAttribution,
        Self::SeveritySemantics,
        Self::DisclosureControl,
        Self::KeyboardRoute,
        Self::AuditExportAnchor,
    ];

    /// Every contract part is mandatory: every primitive pins all of them.
    pub const MANDATORY: [Self; 7] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityLabel => "identity_label",
            Self::TypedState => "typed_state",
            Self::ProvenanceOrSourceAttribution => "provenance_or_source_attribution",
            Self::SeveritySemantics => "severity_semantics",
            Self::DisclosureControl => "disclosure_control",
            Self::KeyboardRoute => "keyboard_route",
            Self::AuditExportAnchor => "audit_export_anchor",
        }
    }
}

/// The verdict the resolver reaches on one declared binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BindingConformance {
    /// The consumer renders through the canonical primitive, wired every fixed token
    /// slot, and overrode no contract part.
    Conformant,
    /// The consumer renders a bespoke local variant — not allowed.
    BespokeDrift,
    /// The consumer left a fixed token slot unwired.
    TokenWiringIncomplete,
    /// The consumer overrode a fixed contract part.
    ContractPartOverridden,
}

impl M5BindingConformance {
    /// Every conformance verdict, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Conformant,
        Self::BespokeDrift,
        Self::TokenWiringIncomplete,
        Self::ContractPartOverridden,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::BespokeDrift => "bespoke_drift",
            Self::TokenWiringIncomplete => "token_wiring_incomplete",
            Self::ContractPartOverridden => "contract_part_overridden",
        }
    }

    /// `true` when the binding is fully conformant.
    pub const fn is_conformant(self) -> bool {
        matches!(self, Self::Conformant)
    }
}

/// One first consumer's declared render of a primitive on a host surface, before
/// resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrimitiveBindingInput {
    /// The host-rendered primitive family being rendered.
    pub primitive_family: M5HostRenderedPrimitiveFamily,
    /// Opaque, export-safe id of the first consumer declaring the binding.
    pub consumer_id: String,
    /// The host runtime this consumer renders on.
    pub host_surface: M5PrimitiveHostSurface,
    /// How the consumer renders the primitive.
    pub render_mode: M5PrimitiveRenderMode,
    /// Opaque, export-safe audit ref for the wrapper, required when the render mode
    /// is an audited wrapper.
    pub audited_wrapper_ref: Option<String>,
    /// The shared token / state slots the consumer wired through the host layer.
    pub wired_token_slots: Vec<M5DesignTokenSlot>,
    /// The cosmetic aspects the consumer restyled.
    pub restyled_aspects: Vec<M5RestylableAspect>,
    /// The fixed contract parts the consumer overrode — must be empty for a
    /// conformant binding.
    pub overridden_contract_parts: Vec<M5PrimitiveContractPart>,
}

impl M5PrimitiveBindingInput {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.consumer_id)
            || self
                .audited_wrapper_ref
                .as_deref()
                .is_some_and(repr_is_forbidden)
    }
}

/// The resolved posture of one declared binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBinding {
    /// The host-rendered primitive family.
    pub primitive_family: M5HostRenderedPrimitiveFamily,
    /// The opaque consumer id.
    pub consumer_id: String,
    /// The host surface.
    pub host_surface: M5PrimitiveHostSurface,
    /// The render mode.
    pub render_mode: M5PrimitiveRenderMode,
    /// True when the consumer renders through the canonical primitive or an audited
    /// wrapper.
    pub renders_through_canonical: bool,
    /// True when every fixed token slot for the family was wired.
    pub fixed_token_slots_wired: bool,
    /// True when the restyle stayed within cosmetic bounds (no contract part
    /// overridden).
    pub restyle_within_bounds: bool,
    /// The fixed token slots the family requires.
    pub required_token_slots: Vec<M5DesignTokenSlot>,
    /// The token slots the consumer actually wired.
    pub wired_token_slots: Vec<M5DesignTokenSlot>,
    /// The audit ref for the wrapper, echoed when present.
    pub audited_wrapper_ref: Option<String>,
    /// The single conformance verdict.
    pub conformance: M5BindingConformance,
}

/// Errors returned by [`resolve_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BindingResolutionError {
    /// The consumer id was empty.
    EmptyConsumerId,
    /// The render mode was an audited wrapper but no wrapper ref was supplied.
    WrapperRefMissing,
    /// A wrapper ref was supplied but the render mode is not an audited wrapper.
    UnexpectedWrapperRef,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5BindingResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyConsumerId => "empty_consumer_id",
            Self::WrapperRefMissing => "wrapper_ref_missing",
            Self::UnexpectedWrapperRef => "unexpected_wrapper_ref",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5BindingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "binding resolution error: {}", self.as_str())
    }
}

impl Error for M5BindingResolutionError {}

/// Resolves one first consumer's declared render into a single conformance verdict.
///
/// A bespoke local variant is always [`M5BindingConformance::BespokeDrift`]; an
/// override of a fixed contract part is [`M5BindingConformance::ContractPartOverridden`];
/// a fixed token slot left unwired is [`M5BindingConformance::TokenWiringIncomplete`];
/// only a canonical (or audited-wrapper) render that wires every fixed slot and
/// overrides no contract part is [`M5BindingConformance::Conformant`]. The resolver
/// never blesses a bespoke variant, never lets a fixed token slot go unwired, and
/// never lets a restyle reach a contract part that carries meaning.
pub fn resolve_binding(
    input: &M5PrimitiveBindingInput,
) -> Result<M5ResolvedBinding, M5BindingResolutionError> {
    if input.consumer_id.trim().is_empty() {
        return Err(M5BindingResolutionError::EmptyConsumerId);
    }
    match (input.render_mode, input.audited_wrapper_ref.as_deref()) {
        (M5PrimitiveRenderMode::AuditedWrapper, None) => {
            return Err(M5BindingResolutionError::WrapperRefMissing)
        }
        (M5PrimitiveRenderMode::AuditedWrapper, Some(reference)) if reference.trim().is_empty() => {
            return Err(M5BindingResolutionError::WrapperRefMissing)
        }
        (mode, Some(_)) if mode != M5PrimitiveRenderMode::AuditedWrapper => {
            return Err(M5BindingResolutionError::UnexpectedWrapperRef)
        }
        _ => {}
    }
    if input.carries_forbidden_material() {
        return Err(M5BindingResolutionError::ForbiddenMaterial);
    }

    let required_token_slots = input.primitive_family.fixed_token_slots();
    let wired: BTreeSet<M5DesignTokenSlot> = input.wired_token_slots.iter().copied().collect();
    let fixed_token_slots_wired = required_token_slots.iter().all(|slot| wired.contains(slot));
    let restyle_within_bounds = input.overridden_contract_parts.is_empty();
    let renders_through_canonical = input.render_mode.renders_through_canonical();

    // Precedence: a bespoke variant is the worst drift, then a contract-part
    // override, then an unwired token slot; otherwise the binding is conformant.
    let conformance = if !renders_through_canonical {
        M5BindingConformance::BespokeDrift
    } else if !restyle_within_bounds {
        M5BindingConformance::ContractPartOverridden
    } else if !fixed_token_slots_wired {
        M5BindingConformance::TokenWiringIncomplete
    } else {
        M5BindingConformance::Conformant
    };

    Ok(M5ResolvedBinding {
        primitive_family: input.primitive_family,
        consumer_id: input.consumer_id.clone(),
        host_surface: input.host_surface,
        render_mode: input.render_mode,
        renders_through_canonical,
        fixed_token_slots_wired,
        restyle_within_bounds,
        required_token_slots,
        wired_token_slots: input.wired_token_slots.clone(),
        audited_wrapper_ref: input.audited_wrapper_ref.clone(),
        conformance,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs the binding verdict from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrimitiveBindingCase {
    /// The resolver input.
    pub input: M5PrimitiveBindingInput,
    /// The resolved binding. Must equal `resolve_binding(&input)`.
    pub resolved: M5ResolvedBinding,
}

impl M5PrimitiveBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5PrimitiveBindingInput) -> Self {
        let resolved = resolve_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// The naming a row projects into demos, screenshots, and support exports. Every
/// name must equal the primitive family token so no surface relabels the primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrimitiveNamingParity {
    /// The name used in the component demo / gallery.
    pub demo_name: String,
    /// The name used in the screenshot / golden evidence.
    pub screenshot_name: String,
    /// The name used in the support / export packet.
    pub support_export_name: String,
}

impl M5PrimitiveNamingParity {
    /// True when all three names equal the family token.
    fn matches_family(&self, family: M5HostRenderedPrimitiveFamily) -> bool {
        let token = family.as_str();
        self.demo_name == token
            && self.screenshot_name == token
            && self.support_export_name == token
    }
}

/// One row in the binding matrix: one host-rendered primitive family bound to the
/// frozen component families it renders, the host surfaces it renders on, the fixed
/// token slots and contract parts it pins, the cosmetic aspects a contributor may
/// restyle, worked binding cases, and its naming parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostRenderedPrimitiveRow {
    /// Host-rendered primitive family.
    pub primitive_family: M5HostRenderedPrimitiveFamily,
    /// Qualification class earned by this primitive.
    pub qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this primitive governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Frozen component families this primitive host-renders (must equal
    /// `primitive_family.bound_component_families()`).
    pub bound_component_families: Vec<M5TrustComponentFamily>,
    /// Canonical shell zone this primitive attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this primitive must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this primitive keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Host surfaces this primitive renders on (must include every mandatory host
    /// surface).
    pub host_surfaces: Vec<M5PrimitiveHostSurface>,
    /// Render modes this primitive permits (must include the canonical mode and never
    /// the bespoke variant).
    pub render_modes: Vec<M5PrimitiveRenderMode>,
    /// The shared token slots this primitive pins as fixed (must equal
    /// `primitive_family.fixed_token_slots()`).
    pub fixed_token_slots: Vec<M5DesignTokenSlot>,
    /// The fixed contract parts this primitive pins (must include the mandatory
    /// parts).
    pub fixed_contract_parts: Vec<M5PrimitiveContractPart>,
    /// The cosmetic aspects a contributor may restyle.
    pub restylable_aspects: Vec<M5RestylableAspect>,
    /// Source pills this primitive wires (settings row only; empty otherwise).
    pub source_pills: Vec<M5SettingSourcePill>,
    /// Provenance badges this primitive wires (chronology families only; empty
    /// otherwise).
    pub provenance_badges: Vec<M5ProvenanceBadge>,
    /// Non-visual accessibility routes this primitive offers.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Shell subsystems that consume this primitive's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this primitive.
    pub downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// The naming this row projects into demos, screenshots, and support exports.
    pub naming_parity: M5PrimitiveNamingParity,
    /// Proof packet refs that keep this primitive current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this primitive.
    pub source_contract_refs: Vec<String>,
    /// Worked binding cases proving the resolver on this primitive.
    pub example_bindings: Vec<M5PrimitiveBindingCase>,
    /// Hard invariant: this primitive never allows a bespoke local variant. MUST be
    /// `false`.
    pub allows_bespoke_local_variant: bool,
    /// Hard invariant: this primitive never drops a fixed token slot from the host
    /// wiring. MUST be `false`.
    pub drops_fixed_token_wiring: bool,
    /// Hard invariant: this primitive never lets a restyle reach a fixed contract
    /// part. MUST be `false`.
    pub restyles_fixed_contract_part: bool,
    /// Hard invariant: this primitive never drops export / audit truth. MUST be
    /// `false`.
    pub drops_export_or_audit_truth: bool,
}

impl M5HostRenderedPrimitiveRow {
    /// True when the row binds exactly the frozen families for its primitive.
    fn binds_expected_families(&self) -> bool {
        let present: BTreeSet<M5TrustComponentFamily> =
            self.bound_component_families.iter().copied().collect();
        let expected: BTreeSet<M5TrustComponentFamily> = self
            .primitive_family
            .bound_component_families()
            .into_iter()
            .collect();
        present == expected
    }

    /// True when the row pins exactly the fixed token slots for its primitive.
    fn pins_expected_token_slots(&self) -> bool {
        let present: BTreeSet<M5DesignTokenSlot> = self.fixed_token_slots.iter().copied().collect();
        let expected: BTreeSet<M5DesignTokenSlot> = self
            .primitive_family
            .fixed_token_slots()
            .into_iter()
            .collect();
        present == expected
    }

    /// True when the row declares every mandatory contract part.
    fn declares_mandatory_contract_parts(&self) -> bool {
        let present: BTreeSet<M5PrimitiveContractPart> =
            self.fixed_contract_parts.iter().copied().collect();
        M5PrimitiveContractPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory host surface.
    fn declares_mandatory_host_surfaces(&self) -> bool {
        let present: BTreeSet<M5PrimitiveHostSurface> =
            self.host_surfaces.iter().copied().collect();
        M5PrimitiveHostSurface::MANDATORY
            .iter()
            .all(|surface| present.contains(surface))
    }

    /// True when the row permits the canonical render mode and never the bespoke
    /// variant.
    fn render_modes_are_safe(&self) -> bool {
        self.render_modes
            .contains(&M5PrimitiveRenderMode::HostRenderedCanonical)
            && !self
                .render_modes
                .contains(&M5PrimitiveRenderMode::BespokeLocalVariant)
    }

    /// True when every worked case binds this row's primitive.
    fn examples_match_family(&self) -> bool {
        self.example_bindings
            .iter()
            .all(|case| case.input.primitive_family == self.primitive_family)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.allows_bespoke_local_variant
            && !self.drops_fixed_token_wiring
            && !self.restyles_fixed_contract_part
            && !self.drops_export_or_audit_truth
    }
}

/// Self-describing controlled-vocabulary set minted by this binding layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostRenderedVocabularySet {
    /// Host-rendered-primitive-family tokens.
    pub primitive_families: Vec<String>,
    /// Component-family tokens (reused from the frozen matrix).
    pub component_families: Vec<String>,
    /// Host-surface tokens.
    pub host_surfaces: Vec<String>,
    /// Render-mode tokens.
    pub render_modes: Vec<String>,
    /// Design-token-slot tokens.
    pub token_slots: Vec<String>,
    /// Restylable-aspect tokens.
    pub restylable_aspects: Vec<String>,
    /// Contract-part tokens.
    pub contract_parts: Vec<String>,
    /// Binding-conformance tokens.
    pub binding_conformances: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5HostRenderedVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            primitive_families: tokens(&M5HostRenderedPrimitiveFamily::ALL, |v| v.as_str()),
            component_families: tokens(&M5TrustComponentFamily::ALL, |v| v.as_str()),
            host_surfaces: tokens(&M5PrimitiveHostSurface::ALL, |v| v.as_str()),
            render_modes: tokens(&M5PrimitiveRenderMode::ALL, |v| v.as_str()),
            token_slots: tokens(&M5DesignTokenSlot::ALL, |v| v.as_str()),
            restylable_aspects: tokens(&M5RestylableAspect::ALL, |v| v.as_str()),
            contract_parts: tokens(&M5PrimitiveContractPart::ALL, |v| v.as_str()),
            binding_conformances: tokens(&M5BindingConformance::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TrustAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5HostRenderedGovernanceReview {
    /// Every family binds to one canonical host-rendered primitive.
    pub every_family_binds_one_canonical_primitive: bool,
    /// First consumers render through the primitive or an audited wrapper, never a
    /// bespoke local variant.
    pub consumers_render_through_canonical_or_wrapper: bool,
    /// Shared token / state wiring is pinned by the host layer.
    pub shared_token_state_wiring_pinned: bool,
    /// Fixed contract parts stay fixed; only cosmetic aspects are restylable.
    pub contract_parts_fixed_only_cosmetics_restylable: bool,
    /// Provenance badges, source pills, and severity / state colours are wired
    /// through the host layer.
    pub badges_pills_and_severity_wired_through_host: bool,
    /// The same primitive keeps its meaning across desktop, companion, and extension
    /// hosts.
    pub meaning_stable_across_host_surfaces: bool,
    /// Demos, screenshots, and support exports reference the same primitive family
    /// names.
    pub demos_screenshots_and_exports_share_names: bool,
    /// No first consumer invents a second row grammar.
    pub no_consumer_invents_second_row_grammar: bool,
    /// Every primitive is bound to a canonical shell zone.
    pub every_primitive_bound_to_shell_zone: bool,
    /// Later M5 lanes cannot invent parallel host-rendered vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostRenderedConsumerProjection {
    /// Desktop first consumers render through the canonical primitives.
    pub desktop_consumers_render_canonical: bool,
    /// Companion first consumers render through the canonical primitives.
    pub companion_consumers_render_canonical: bool,
    /// Extension-backed first consumers render through canonical primitives or
    /// audited wrappers.
    pub extension_consumers_render_canonical_or_wrapper: bool,
    /// The token / state wiring reads a single canonical source.
    pub token_state_wiring_reads_single_source: bool,
    /// Support / export reads a single canonical primitive source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostRenderedProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the host-rendered primitive layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostRenderedReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting host-rendered audit.
    pub host_rendered_audit_ref: String,
    /// True when support / export parity is required for every primitive.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every primitive.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5HostRenderedPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HostRenderedPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Primitive rows.
    pub primitive_rows: Vec<M5HostRenderedPrimitiveRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HostRenderedVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HostRenderedGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HostRenderedConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HostRenderedProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HostRenderedReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 host-rendered-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostRenderedPrimitivePacket {
    /// Record kind; must equal [`M5_HOST_RENDERED_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HOST_RENDERED_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Primitive rows.
    pub primitive_rows: Vec<M5HostRenderedPrimitiveRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HostRenderedVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HostRenderedGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HostRenderedConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HostRenderedProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HostRenderedReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5HostRenderedPrimitivePacket {
    /// Builds an M5 host-rendered-primitive packet from stable-lane input.
    pub fn new(input: M5HostRenderedPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_HOST_RENDERED_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_HOST_RENDERED_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            primitive_rows: input.primitive_rows,
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

    /// Validates the M5 host-rendered-primitive invariants.
    pub fn validate(&self) -> Vec<M5HostRenderedPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HOST_RENDERED_PRIMITIVE_RECORD_KIND {
            violations.push(M5HostRenderedPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HOST_RENDERED_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5HostRenderedPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HostRenderedPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_primitive_rows(self, &mut violations);
        validate_canonical_rendering_covered(self, &mut violations);
        validate_token_wiring_parity_covered(self, &mut violations);
        validate_naming_parity_covered(self, &mut violations);
        validate_matrix_family_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 host-rendered primitive packet serializes"),
        ) {
            violations.push(M5HostRenderedPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 host-rendered primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per primitive family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "primitive_family,qualification,owner,shell_zone_slot,bound_families,host_surfaces,render_modes,fixed_token_slots,restylable_aspects,example_count\n",
        );
        for row in &self.primitive_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.primitive_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.bound_component_families, |v| v.as_str()),
                join_tokens(&row.host_surfaces, |v| v.as_str()),
                join_tokens(&row.render_modes, |v| v.as_str()),
                join_tokens(&row.fixed_token_slots, |v| v.as_str()),
                join_tokens(&row.restylable_aspects, |v| v.as_str()),
                row.example_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .primitive_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Host-Rendered Trust-Component Primitives and Token / State Wiring\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Primitive families: {} ({} stable)\n",
            self.primitive_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Host surfaces: {}\n",
            self.vocabulary_set.host_surfaces.join(", ")
        ));
        out.push_str(&format!(
            "- Render modes: {}\n",
            self.vocabulary_set.render_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Token slots: {}\n",
            self.vocabulary_set.token_slots.join(", ")
        ));
        out.push_str(&format!(
            "- Contract parts (fixed): {}\n",
            self.vocabulary_set.contract_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Restylable aspects: {}\n",
            self.vocabulary_set.restylable_aspects.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Primitive families\n\n");
        for row in &self.primitive_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.primitive_family.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Bound families: {}\n",
                row.bound_component_families
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Worked bindings: {}\n",
                row.example_bindings.len()
            ));
            for case in &row.example_bindings {
                out.push_str(&format!(
                    "    - {} on {}: {}\n",
                    case.resolved.render_mode.as_str(),
                    case.resolved.host_surface.as_str(),
                    case.resolved.conformance.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 host-rendered-primitive export.
#[derive(Debug)]
pub enum M5HostRenderedPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HostRenderedPrimitiveViolation>),
}

impl fmt::Display for M5HostRenderedPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 host-rendered primitive export parse failed: {error}"
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
                    "m5 host-rendered primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5HostRenderedPrimitiveArtifactError {}

/// Validation failures emitted by [`M5HostRenderedPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HostRenderedPrimitiveViolation {
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
    /// A required host-rendered primitive family is missing from the matrix.
    RequiredPrimitiveMissing,
    /// A primitive row is incomplete.
    PrimitiveRowIncomplete,
    /// A primitive row binds the wrong frozen component families.
    BoundFamiliesMismatch,
    /// A primitive row pins the wrong fixed token slots.
    FixedTokenSlotsMismatch,
    /// A primitive row omits one of the mandatory contract parts.
    MandatoryContractPartMissing,
    /// A primitive row omits one of the mandatory host surfaces.
    MandatoryHostSurfaceMissing,
    /// A primitive row permits a bespoke variant or omits the canonical mode.
    RenderModeUnsafe,
    /// A primitive row declares no restylable aspects.
    RestylableAspectMissing,
    /// A primitive row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A primitive row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A primitive row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A primitive row declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A worked binding case's family disagrees with its row.
    ExampleFamilyMismatch,
    /// A primitive claiming Stable is missing required proof packet refs.
    StablePrimitiveMissingProof,
    /// Not every primitive is proven — with a conformant worked binding — to render
    /// through the canonical primitive, or some binding is a bespoke drift.
    CanonicalRenderingUnproven,
    /// Some primitive does not prove identical fixed token wiring across two or more
    /// host surfaces.
    TokenWiringParityUnproven,
    /// Some primitive's demo / screenshot / support-export name does not match its
    /// family token.
    NamingParityUnproven,
    /// The bound component families do not cover every frozen component family.
    MatrixFamilyCoverageUnproven,
    /// A primitive row violates a hard invariant.
    PrimitiveInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5HostRenderedPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredPrimitiveMissing => "required_primitive_missing",
            Self::PrimitiveRowIncomplete => "primitive_row_incomplete",
            Self::BoundFamiliesMismatch => "bound_families_mismatch",
            Self::FixedTokenSlotsMismatch => "fixed_token_slots_mismatch",
            Self::MandatoryContractPartMissing => "mandatory_contract_part_missing",
            Self::MandatoryHostSurfaceMissing => "mandatory_host_surface_missing",
            Self::RenderModeUnsafe => "render_mode_unsafe",
            Self::RestylableAspectMissing => "restylable_aspect_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::ExampleFamilyMismatch => "example_family_mismatch",
            Self::StablePrimitiveMissingProof => "stable_primitive_missing_proof",
            Self::CanonicalRenderingUnproven => "canonical_rendering_unproven",
            Self::TokenWiringParityUnproven => "token_wiring_parity_unproven",
            Self::NamingParityUnproven => "naming_parity_unproven",
            Self::MatrixFamilyCoverageUnproven => "matrix_family_coverage_unproven",
            Self::PrimitiveInvariantViolated => "primitive_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 host-rendered-primitive export.
pub fn current_stable_m5_host_rendered_primitive_export(
) -> Result<M5HostRenderedPrimitivePacket, M5HostRenderedPrimitiveArtifactError> {
    let packet: M5HostRenderedPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-host-rendered-primitives-proof/support_export.json"
    )))
    .map_err(M5HostRenderedPrimitiveArtifactError::SupportExport)?;

    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HostRenderedPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_HOST_RENDERED_SCHEMA_REF,
        M5_HOST_RENDERED_DOC_REF,
        M5_HOST_RENDERED_SHELL_ZONE_REF,
        M5_HOST_RENDERED_COMPONENT_MATRIX_REF,
        M5_HOST_RENDERED_SETTINGS_ROW_REF,
        M5_HOST_RENDERED_CAPABILITY_SHEET_REF,
        M5_HOST_RENDERED_EVIDENCE_ROW_REF,
        M5_HOST_RENDERED_CHRONOLOGY_PREVIEW_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5HostRenderedPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5HostRenderedPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_primitive_rows(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let present: BTreeSet<M5HostRenderedPrimitiveFamily> = packet
        .primitive_rows
        .iter()
        .map(|row| row.primitive_family)
        .collect();
    for required in M5HostRenderedPrimitiveFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5HostRenderedPrimitiveViolation::RequiredPrimitiveMissing);
            return;
        }
    }

    for row in &packet.primitive_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.bound_component_families.is_empty()
        {
            violations.push(M5HostRenderedPrimitiveViolation::PrimitiveRowIncomplete);
        }
        if !row.binds_expected_families() {
            violations.push(M5HostRenderedPrimitiveViolation::BoundFamiliesMismatch);
        }
        if !row.pins_expected_token_slots() {
            violations.push(M5HostRenderedPrimitiveViolation::FixedTokenSlotsMismatch);
        }
        if !row.declares_mandatory_contract_parts() {
            violations.push(M5HostRenderedPrimitiveViolation::MandatoryContractPartMissing);
        }
        if !row.declares_mandatory_host_surfaces() {
            violations.push(M5HostRenderedPrimitiveViolation::MandatoryHostSurfaceMissing);
        }
        if !row.render_modes_are_safe() {
            violations.push(M5HostRenderedPrimitiveViolation::RenderModeUnsafe);
        }
        if row.restylable_aspects.is_empty() {
            violations.push(M5HostRenderedPrimitiveViolation::RestylableAspectMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TrustAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5HostRenderedPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5HostRenderedPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5HostRenderedPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_bindings.is_empty() {
            violations.push(M5HostRenderedPrimitiveViolation::ExampleBindingMissing);
        }
        if row
            .example_bindings
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5HostRenderedPrimitiveViolation::ExampleBindingDrift);
        }
        if !row.examples_match_family() {
            violations.push(M5HostRenderedPrimitiveViolation::ExampleFamilyMismatch);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5HostRenderedPrimitiveViolation::StablePrimitiveMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5HostRenderedPrimitiveViolation::PrimitiveInvariantViolated);
        }
    }
}

/// AC1: every primitive must be proven — with a conformant worked binding — to render
/// through the canonical primitive (or an audited wrapper), and no worked binding
/// anywhere may be a bespoke drift.
fn validate_canonical_rendering_covered(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let proven: BTreeSet<M5HostRenderedPrimitiveFamily> = packet
        .primitive_rows
        .iter()
        .filter(|row| {
            row.example_bindings.iter().any(|case| {
                case.resolved.conformance.is_conformant() && case.resolved.renders_through_canonical
            })
        })
        .map(|row| row.primitive_family)
        .collect();
    let any_bespoke = packet.primitive_rows.iter().any(|row| {
        row.example_bindings
            .iter()
            .any(|case| case.resolved.conformance == M5BindingConformance::BespokeDrift)
    });
    if any_bespoke
        || !M5HostRenderedPrimitiveFamily::ALL
            .iter()
            .all(|family| proven.contains(family))
    {
        violations.push(M5HostRenderedPrimitiveViolation::CanonicalRenderingUnproven);
    }
}

/// AC2: every primitive must prove identical fixed token wiring across two or more
/// host surfaces — the proof that shared token / state wiring prevents meaning drift
/// between surfaces.
fn validate_token_wiring_parity_covered(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    for row in &packet.primitive_rows {
        let required: BTreeSet<M5DesignTokenSlot> = row
            .primitive_family
            .fixed_token_slots()
            .into_iter()
            .collect();
        let conformant_surfaces: BTreeSet<M5PrimitiveHostSurface> = row
            .example_bindings
            .iter()
            .filter(|case| {
                case.resolved.conformance.is_conformant()
                    && required
                        .iter()
                        .all(|slot| case.resolved.wired_token_slots.contains(slot))
            })
            .map(|case| case.resolved.host_surface)
            .collect();
        if conformant_surfaces.len() < 2 {
            violations.push(M5HostRenderedPrimitiveViolation::TokenWiringParityUnproven);
            return;
        }
    }
}

/// AC3: every primitive's demo, screenshot, and support-export name must equal its
/// family token — the proof that demos, screenshots, and support / export packets all
/// reference the same primitive family names.
fn validate_naming_parity_covered(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let all_match = packet
        .primitive_rows
        .iter()
        .all(|row| row.naming_parity.matches_family(row.primitive_family));
    if !all_match {
        violations.push(M5HostRenderedPrimitiveViolation::NamingParityUnproven);
    }
}

/// The bound component families across all rows must cover every frozen component
/// family — the proof that the binding layer host-renders the whole matrix, not a
/// subset.
fn validate_matrix_family_coverage(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let covered: BTreeSet<M5TrustComponentFamily> = packet
        .primitive_rows
        .iter()
        .flat_map(|row| row.bound_component_families.iter().copied())
        .collect();
    if !M5TrustComponentFamily::ALL
        .iter()
        .all(|family| covered.contains(family))
    {
        violations.push(M5HostRenderedPrimitiveViolation::MatrixFamilyCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.every_family_binds_one_canonical_primitive,
        review.consumers_render_through_canonical_or_wrapper,
        review.shared_token_state_wiring_pinned,
        review.contract_parts_fixed_only_cosmetics_restylable,
        review.badges_pills_and_severity_wired_through_host,
        review.meaning_stable_across_host_surfaces,
        review.demos_screenshots_and_exports_share_names,
        review.no_consumer_invents_second_row_grammar,
        review.every_primitive_bound_to_shell_zone,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5HostRenderedPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_consumers_render_canonical,
        projection.companion_consumers_render_canonical,
        projection.extension_consumers_render_canonical_or_wrapper,
        projection.token_state_wiring_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5HostRenderedPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5HostRenderedPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5HostRenderedPrimitivePacket,
    violations: &mut Vec<M5HostRenderedPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.host_rendered_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5HostRenderedPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
