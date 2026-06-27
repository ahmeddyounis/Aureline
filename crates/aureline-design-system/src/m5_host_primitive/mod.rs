//! Host-rendered canonical primitives for the launch-critical M5 component families.
//!
//! Where [`crate::m5_component_manifest`] freezes the *component contracts* — the anatomy, states,
//! keyboard, accessibility, and token dependencies each launch-critical family declares — this
//! module ships the host-rendered *implementations* M5 surfaces route through instead of forking a
//! per-feature variant of the same trust / state / boundary / review component. A versioned
//! [`M5HostPrimitiveLibrary`] carries one [`M5HostPrimitive`] per
//! [component kind](crate::m5_component_manifest::M5ComponentKind) — placeholder cards, state
//! blocks, review sheets, durable job rows, boundary bars, form controls, and dense collections.
//!
//! Each primitive is the single shared surface every M5 family renders through, so the same state,
//! boundary, and review patterns render equivalently rather than as parallel implementations. A
//! primitive records:
//!
//! - **manifest binding** — the [component id](M5HostPrimitive::component_id), accessibility role,
//!   keyboard chords, and foundation token references it inherits from its component manifest, so it
//!   wires to the shared contract rather than feature-local styling. The seed builder derives these
//!   directly from [the manifest package](crate::m5_component_manifest::seeded_m5_component_manifest_package),
//!   and [`audit_primitive_manifest_alignment`] proves the binding holds.
//! - **state render plans** — one [`M5StateRenderPlan`] per
//!   [controlled state](crate::CanonicalStateClass): the parts it renders, the
//!   [non-color cues](crate::NonColorCueClass) it carries (always including label text, so meaning
//!   is never color-only), the status message id it announces, and whether the state is interactive.
//! - **appearance binding** — the [density classes, motion postures, and contrast classes](M5AppearanceBinding)
//!   it preserves from the shared foundations, plus explicit focus / keyboard / high-contrast /
//!   reduced-motion guarantees.
//! - **consumer routing** — the [M5 family surfaces](M5PrimitiveConsumer) that route through the
//!   primitive, each with a [conformance posture](M5ConformancePosture). An embedded or
//!   extension-backed consumer either inherits the host-rendered primitive or declares a reduced
//!   posture with an explicit partial badge; it can never masquerade as first-party parity.
//!
//! The records are metadata-only truth packets: they carry semantic token *references* and message
//! *ids*, never raw color values, credential bodies, or provider payloads.
//!
//! - Schema:
//!   [`schemas/design-system/m5-host-primitive.schema.json`](../../../../../schemas/design-system/m5-host-primitive.schema.json)
//! - Doc:
//!   [`docs/design-system/m5-host-primitive.md`](../../../../../docs/design-system/m5-host-primitive.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_host_primitive_library, M5_HOST_PRIMITIVE_LIBRARY_ID,
    M5_HOST_PRIMITIVE_LIBRARY_VERSION,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_ui::density::DensityClass;
use aureline_ui::themes::AccessibilityPostureClass;
use aureline_ui::tokens::ThemeClass;

use crate::m5_component_manifest::{M5ComponentKind, M5ComponentManifestPackage};
use crate::{CanonicalStateClass, NonColorCueClass};

/// Record-kind tag carried by [`M5HostPrimitiveLibrary`].
pub const M5_HOST_PRIMITIVE_LIBRARY_RECORD_KIND: &str = "m5_design_system_host_primitive_library";

/// Record-kind tag carried by [`M5HostPrimitiveReleasePacket`].
pub const M5_HOST_PRIMITIVE_RELEASE_RECORD_KIND: &str = "m5_design_system_host_primitive_release";

/// Schema version shared by the host-primitive records.
pub const M5_HOST_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the host-primitive boundary schema.
pub const M5_HOST_PRIMITIVE_SCHEMA_REF: &str =
    "schemas/design-system/m5-host-primitive.schema.json";

/// Repo-relative path of the host-primitive contract doc.
pub const M5_HOST_PRIMITIVE_DOC_REF: &str = "docs/design-system/m5-host-primitive.md";

/// Repo-relative path of the release-grade host-primitive proof packet — the proof lane that blocks
/// drift for the library.
pub const M5_HOST_PRIMITIVE_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/host-primitive-release.json";

/// Release packet that keeps the host-primitive library current (shared with the foundation
/// package, component manifests, and contract matrix).
pub const M5_HOST_PRIMITIVE_RELEASE_PACKET_REF: &str = "evidence:m5-design-system-release-packet";

/// Repo-relative directory of the checked-in primitive fixtures.
pub const M5_HOST_PRIMITIVE_DIR: &str = "fixtures/ui/m5-component-gallery/";

/// Repo-relative extension-SDK guidance an extension author reads to consume a host primitive.
pub const M5_HOST_PRIMITIVE_EXTENSION_GUIDANCE_REF: &str =
    "docs/sdk/extension-ui-host-primitives.md";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX: &str = "design_system_primitive.";

/// The consumer surfaces that MUST each route through a host primitive. A claimed M5 family surface
/// absent from the library is a parallel implementation; the coverage gate rejects it.
pub const REQUIRED_CONSUMER_SURFACES: [&str; 21] = [
    "start_center.empty_workspace",
    "search_surface.no_results",
    "extension_view.empty_state",
    "activity_center.state_summary",
    "settings_root.managed_state",
    "provider_surface.degraded_state",
    "trust_prompt.batch_review",
    "dialog_sheet.staged_decision",
    "extension_review.batch_apply",
    "activity_center.job_row",
    "notification_envelope.durable_job",
    "provider_surface.remote_job_row",
    "embedded_boundary.origin_bar",
    "extension_host.boundary_bar",
    "provider_surface.remote_origin_bar",
    "settings_root.field_control",
    "dialog_sheet.form_field",
    "extension_view.form_field",
    "search_surface.result_list",
    "activity_center.dense_list",
    "provider_surface.remote_collection",
];

/// The class of surface that routes through a host primitive, which fixes the conformance postures
/// it may legitimately declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConsumerClass {
    /// Persistent host chrome the shell renders directly.
    HostChrome,
    /// First-party product surface rendered by the host.
    FirstParty,
    /// Provider-backed surface whose content crosses a remote or capability boundary.
    ProviderBacked,
    /// Extension-contributed surface rendered across the extension boundary.
    ExtensionContributed,
}

impl M5ConsumerClass {
    /// Every consumer class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HostChrome,
        Self::FirstParty,
        Self::ProviderBacked,
        Self::ExtensionContributed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostChrome => "host_chrome",
            Self::FirstParty => "first_party",
            Self::ProviderBacked => "provider_backed",
            Self::ExtensionContributed => "extension_contributed",
        }
    }

    /// True for embedded / extension-backed consumers — the classes that may legitimately declare a
    /// reduced conformance posture rather than full host-rendered parity.
    pub const fn is_embedded_or_extension(self) -> bool {
        matches!(self, Self::ProviderBacked | Self::ExtensionContributed)
    }
}

/// The conformance posture a consumer declares against a host primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConformancePosture {
    /// The consumer renders the host primitive verbatim — full first-party parity.
    InheritedHostRendered,
    /// The consumer cannot inherit fully and renders a reduced posture; it MUST carry a partial
    /// badge so it never reads as first-party parity.
    ReducedWithPartialBadge,
}

impl M5ConformancePosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InheritedHostRendered => "inherited_host_rendered",
            Self::ReducedWithPartialBadge => "reduced_with_partial_badge",
        }
    }
}

/// One controlled-state render plan: the host primitive's deterministic description of what it
/// renders, announces, and offers in a single [`CanonicalStateClass`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateRenderPlan {
    /// The controlled state this plan renders.
    pub state: CanonicalStateClass,
    /// True when the manifest marks this state mandatory for the family.
    pub mandatory: bool,
    /// Anatomy part ids rendered in this state (a subset of the manifest anatomy).
    pub rendered_parts: Vec<String>,
    /// Non-color cues carried in this state; always includes label text so meaning is never carried
    /// by color alone.
    pub non_color_cues: Vec<NonColorCueClass>,
    /// Governed status message id announced in this state; prefixed
    /// [`M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// True when the state offers a focusable action.
    pub interactive: bool,
}

/// The appearance behavior a host primitive preserves from the shared foundations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceBinding {
    /// Density classes the primitive honors; the full density vocabulary.
    pub density_classes: Vec<DensityClass>,
    /// Motion postures the primitive honors; includes the standard, reduced, and power-saver
    /// postures.
    pub motion_postures: Vec<AccessibilityPostureClass>,
    /// Contrast / theme classes the primitive honors; includes both high-contrast variants.
    pub contrast_classes: Vec<ThemeClass>,
    /// The primitive preserves the manifest focus-order rule.
    pub honors_focus_order: bool,
    /// The primitive preserves the manifest keyboard model.
    pub honors_keyboard_model: bool,
    /// The primitive renders correctly under high contrast.
    pub honors_high_contrast: bool,
    /// The primitive honors the reduced-motion posture.
    pub honors_reduced_motion: bool,
}

/// One M5 family surface that routes through a host primitive, with its conformance posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrimitiveConsumer {
    /// Stable surface class id, unique across the whole library.
    pub surface_class: String,
    /// Human-readable surface name.
    pub display_name: String,
    /// The class of consumer, which fixes the postures it may declare.
    pub consumer_class: M5ConsumerClass,
    /// The conformance posture the consumer declares.
    pub posture: M5ConformancePosture,
    /// Governed partial-badge message id; present iff the posture is
    /// [`M5ConformancePosture::ReducedWithPartialBadge`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_badge_message_id: Option<String>,
}

/// One launch-critical family's host-rendered primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostPrimitive {
    /// The governed component kind, shared with the component manifest.
    pub component_kind: M5ComponentKind,
    /// Stable primitive id, unique within the library.
    pub primitive_id: String,
    /// The component id of the manifest this primitive implements.
    pub component_id: String,
    /// Human-readable primitive name.
    pub display_name: String,
    /// Accessibility role inherited from the component manifest.
    pub accessibility_role: String,
    /// Keyboard chords inherited from the component manifest's keyboard model.
    pub keyboard_chords: Vec<String>,
    /// Foundation token references inherited from the component manifest's token dependencies.
    pub token_references: Vec<String>,
    /// One render plan per controlled state, covering the full canonical state set.
    pub state_render_plans: Vec<M5StateRenderPlan>,
    /// The appearance behavior the primitive preserves from the shared foundations.
    pub appearance: M5AppearanceBinding,
    /// The M5 family surfaces that route through this primitive.
    pub consumers: Vec<M5PrimitiveConsumer>,
    /// Stable summary message id; prefixed [`M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5HostPrimitive {
    /// The mandatory render plans, in declared order.
    pub fn mandatory_state_plans(&self) -> Vec<&M5StateRenderPlan> {
        self.state_render_plans
            .iter()
            .filter(|p| p.mandatory)
            .collect()
    }

    /// The render plan for a controlled state, if present.
    pub fn state_plan(&self, state: CanonicalStateClass) -> Option<&M5StateRenderPlan> {
        self.state_render_plans.iter().find(|p| p.state == state)
    }

    /// The consumers that inherit the host-rendered primitive verbatim.
    pub fn inherited_consumers(&self) -> Vec<&M5PrimitiveConsumer> {
        self.consumers
            .iter()
            .filter(|c| c.posture == M5ConformancePosture::InheritedHostRendered)
            .collect()
    }

    /// The consumers that render a reduced posture behind a partial badge.
    pub fn reduced_consumers(&self) -> Vec<&M5PrimitiveConsumer> {
        self.consumers
            .iter()
            .filter(|c| c.posture == M5ConformancePosture::ReducedWithPartialBadge)
            .collect()
    }
}

/// A versioned, machine-readable library of host-rendered M5 primitives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostPrimitiveLibrary {
    /// Record kind; must equal [`M5_HOST_PRIMITIVE_LIBRARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HOST_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable library id.
    pub library_id: String,
    /// Library version (semver `MAJOR.MINOR.PATCH`).
    pub library_version: String,
    /// Owner role accountable for the library.
    pub owner_role: String,
    /// The host primitives (one per [`M5ComponentKind`]).
    pub primitives: Vec<M5HostPrimitive>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Repo-relative release packet that keeps the library current.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable summary message id; prefixed [`M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5HostPrimitiveLibrary {
    /// Finds the primitive for a component kind.
    pub fn primitive(&self, kind: M5ComponentKind) -> Option<&M5HostPrimitive> {
        self.primitives.iter().find(|p| p.component_kind == kind)
    }

    /// Total primitive count.
    pub fn total_primitives(&self) -> usize {
        self.primitives.len()
    }

    /// Every consumer across the library, paired with its primitive's component kind.
    pub fn all_consumers(&self) -> Vec<(M5ComponentKind, &M5PrimitiveConsumer)> {
        self.primitives
            .iter()
            .flat_map(|p| p.consumers.iter().map(move |c| (p.component_kind, c)))
            .collect()
    }

    /// Validates the library invariants, returning the violations (empty when valid).
    pub fn validate(&self) -> Vec<M5HostPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HOST_PRIMITIVE_LIBRARY_RECORD_KIND {
            violations.push(M5HostPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HOST_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5HostPrimitiveViolation::WrongSchemaVersion);
        }
        if self.library_id.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.proof_lane_ref.trim().is_empty()
            || self.release_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HostPrimitiveViolation::MissingIdentity);
        }
        if !is_semver(&self.library_version) {
            violations.push(M5HostPrimitiveViolation::BadLibraryVersion);
        }
        if !self
            .summary_message_id
            .starts_with(M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX)
        {
            violations.push(M5HostPrimitiveViolation::MessageIdPrefixMissing);
        }

        for required in [
            M5_HOST_PRIMITIVE_SCHEMA_REF,
            M5_HOST_PRIMITIVE_DOC_REF,
            M5_HOST_PRIMITIVE_PROOF_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.push(M5HostPrimitiveViolation::MissingSourceContracts);
                break;
            }
        }

        validate_primitive_set(self, &mut violations);
        validate_consumer_coverage(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 host primitive library serializes"),
        ) {
            violations.push(M5HostPrimitiveViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// True when the library validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON for the library.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 host primitive library serializes")
    }

    /// Imports a library from JSON. The caller validates the returned library with
    /// [`Self::validate`].
    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    /// Projects the release-packet inclusion: per-primitive shape and conformance summaries, so a
    /// release record names the primitives QA and support exports cite.
    pub fn release_packet(&self) -> M5HostPrimitiveReleasePacket {
        let primitive_summaries: Vec<M5HostPrimitiveSummary> = self
            .primitives
            .iter()
            .map(|p| M5HostPrimitiveSummary {
                component_kind: p.component_kind,
                primitive_id: p.primitive_id.clone(),
                component_id: p.component_id.clone(),
                state_plan_count: p.state_render_plans.len() as u32,
                mandatory_state_count: p.mandatory_state_plans().len() as u32,
                token_reference_count: p.token_references.len() as u32,
                consumer_count: p.consumers.len() as u32,
                inherited_consumer_count: p.inherited_consumers().len() as u32,
                reduced_consumer_count: p.reduced_consumers().len() as u32,
            })
            .collect();

        M5HostPrimitiveReleasePacket {
            record_kind: M5_HOST_PRIMITIVE_RELEASE_RECORD_KIND.to_owned(),
            schema_version: M5_HOST_PRIMITIVE_SCHEMA_VERSION,
            library_id: self.library_id.clone(),
            library_version: self.library_version.clone(),
            total_primitives: self.total_primitives() as u32,
            total_consumers: self.all_consumers().len() as u32,
            primitive_summaries,
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.release",
                M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX, self.library_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Reads and validates the checked-in canonical host-primitive library fixture.
pub fn current_stable_m5_host_primitive_library(
) -> Result<M5HostPrimitiveLibrary, M5HostPrimitiveArtifactError> {
    let library: M5HostPrimitiveLibrary = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-component-gallery/host-primitive-library.json"
    )))
    .map_err(M5HostPrimitiveArtifactError::Parse)?;
    let violations = library.validate();
    if violations.is_empty() {
        Ok(library)
    } else {
        Err(M5HostPrimitiveArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading a checked-in host-primitive library export.
#[derive(Debug)]
pub enum M5HostPrimitiveArtifactError {
    /// The export failed to parse.
    Parse(serde_json::Error),
    /// The export failed validation.
    Validation(Vec<M5HostPrimitiveViolation>),
}

impl fmt::Display for M5HostPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(formatter, "m5 host primitive library parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 host primitive library failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5HostPrimitiveArtifactError {}

/// Validation failures emitted by [`M5HostPrimitiveLibrary::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HostPrimitiveViolation {
    /// Library record kind is wrong.
    WrongRecordKind,
    /// Library schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The library version is not `MAJOR.MINOR.PATCH`.
    BadLibraryVersion,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A governed component kind has no published primitive.
    RequiredPrimitiveKindMissing,
    /// Two primitives share a kind.
    DuplicatePrimitiveKind,
    /// Two primitives share a primitive id.
    DuplicatePrimitiveId,
    /// A primitive is missing an identity field (primitive id, component id, role, or summary id).
    PrimitiveIncomplete,
    /// A primitive declares no keyboard chords, or one is empty.
    KeyboardIncomplete,
    /// A primitive declares no token references, or one is empty.
    TokenReferencesIncomplete,
    /// A primitive's state render plans do not cover exactly the canonical state set, repeat a
    /// state, or declare no mandatory state.
    StatePlansIncomplete,
    /// A render plan is incomplete (no rendered parts, missing label-text cue, or unprefixed status
    /// id).
    RenderPlanIncomplete,
    /// A primitive's appearance binding does not preserve the full density / motion / contrast
    /// vocabulary or a focus / keyboard / contrast / motion guarantee.
    AppearanceIncomplete,
    /// A primitive declares no consumers.
    ConsumersIncomplete,
    /// A consumer is incomplete (empty surface class or display name).
    ConsumerIncomplete,
    /// A first-party or host-chrome consumer declares a reduced posture instead of full parity.
    FirstPartyCannotReduce,
    /// A reduced consumer is missing its partial badge (the masquerade guard).
    PartialBadgeMissing,
    /// An inherited consumer carries a partial badge it must not.
    InheritedMustNotBadge,
    /// Two consumers across the library claim the same surface (a parallel implementation).
    DuplicateConsumerSurface,
    /// A required consumer surface is not routed through any primitive.
    RequiredConsumerSurfaceMissing,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5HostPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BadLibraryVersion => "bad_library_version",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredPrimitiveKindMissing => "required_primitive_kind_missing",
            Self::DuplicatePrimitiveKind => "duplicate_primitive_kind",
            Self::DuplicatePrimitiveId => "duplicate_primitive_id",
            Self::PrimitiveIncomplete => "primitive_incomplete",
            Self::KeyboardIncomplete => "keyboard_incomplete",
            Self::TokenReferencesIncomplete => "token_references_incomplete",
            Self::StatePlansIncomplete => "state_plans_incomplete",
            Self::RenderPlanIncomplete => "render_plan_incomplete",
            Self::AppearanceIncomplete => "appearance_incomplete",
            Self::ConsumersIncomplete => "consumers_incomplete",
            Self::ConsumerIncomplete => "consumer_incomplete",
            Self::FirstPartyCannotReduce => "first_party_cannot_reduce",
            Self::PartialBadgeMissing => "partial_badge_missing",
            Self::InheritedMustNotBadge => "inherited_must_not_badge",
            Self::DuplicateConsumerSurface => "duplicate_consumer_surface",
            Self::RequiredConsumerSurfaceMissing => "required_consumer_surface_missing",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Release-packet records.
// ---------------------------------------------------------------------------

/// Release-packet projection of a host-primitive library: one shape and conformance summary per
/// primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostPrimitiveReleasePacket {
    /// Record kind; must equal [`M5_HOST_PRIMITIVE_RELEASE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The library id this release record projects.
    pub library_id: String,
    /// The library version.
    pub library_version: String,
    /// Total primitives across the library.
    pub total_primitives: u32,
    /// Total consumers across the library.
    pub total_consumers: u32,
    /// Per-primitive shape and conformance summaries, in library order.
    pub primitive_summaries: Vec<M5HostPrimitiveSummary>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5HostPrimitiveReleasePacket {
    /// Deterministic export-safe JSON for the release packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 host primitive release packet serializes")
    }
}

/// One primitive's shape and conformance summary inside a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostPrimitiveSummary {
    /// The governed component kind.
    pub component_kind: M5ComponentKind,
    /// The primitive id.
    pub primitive_id: String,
    /// The component id the primitive implements.
    pub component_id: String,
    /// State render plan count (the full canonical set).
    pub state_plan_count: u32,
    /// Mandatory state count.
    pub mandatory_state_count: u32,
    /// Token reference count.
    pub token_reference_count: u32,
    /// Consumer count.
    pub consumer_count: u32,
    /// Inherited (full-parity) consumer count.
    pub inherited_consumer_count: u32,
    /// Reduced (partial-badge) consumer count.
    pub reduced_consumer_count: u32,
}

// ---------------------------------------------------------------------------
// Manifest-alignment audit.
// ---------------------------------------------------------------------------

/// One finding from [`audit_primitive_manifest_alignment`]: a place where a primitive's binding has
/// drifted from the component manifest it implements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrimitiveAlignmentFinding {
    /// The primitive id the finding is about.
    pub primitive_id: String,
    /// The component id the primitive claims to implement.
    pub component_id: String,
    /// Stable finding code.
    pub code: String,
    /// Human-readable detail.
    pub detail: String,
}

/// Audits that every host primitive is wired to its component manifest rather than feature-local
/// styling or semantics: the component id, accessibility role, keyboard chords, token references,
/// and mandatory states must all match the manifest. Returns an empty vector when every primitive
/// is aligned.
pub fn audit_primitive_manifest_alignment(
    library: &M5HostPrimitiveLibrary,
    manifest_package: &M5ComponentManifestPackage,
) -> Vec<M5PrimitiveAlignmentFinding> {
    let mut findings = Vec::new();
    for primitive in &library.primitives {
        let Some(manifest) = manifest_package.manifest(primitive.component_kind) else {
            findings.push(finding(
                primitive,
                "missing_manifest",
                "no component manifest publishes this primitive's kind",
            ));
            continue;
        };
        if primitive.component_id != manifest.component_id {
            findings.push(finding(
                primitive,
                "component_id_mismatch",
                &format!(
                    "primitive binds {} but the manifest is {}",
                    primitive.component_id, manifest.component_id
                ),
            ));
        }
        if primitive.accessibility_role != manifest.accessibility.role {
            findings.push(finding(
                primitive,
                "accessibility_role_mismatch",
                "primitive role does not match the manifest accessibility role",
            ));
        }

        let manifest_chords: BTreeSet<&str> =
            manifest.keyboard.iter().map(|k| k.keys.as_str()).collect();
        if primitive
            .keyboard_chords
            .iter()
            .any(|c| !manifest_chords.contains(c.as_str()))
        {
            findings.push(finding(
                primitive,
                "keyboard_not_in_manifest",
                "primitive declares a keyboard chord the manifest does not publish",
            ));
        }

        let manifest_tokens: BTreeSet<&str> = manifest
            .token_dependencies
            .iter()
            .map(String::as_str)
            .collect();
        if primitive
            .token_references
            .iter()
            .any(|t| !manifest_tokens.contains(t.as_str()))
        {
            findings.push(finding(
                primitive,
                "token_not_in_manifest",
                "primitive references a token the manifest does not depend on",
            ));
        }

        let manifest_mandatory: BTreeSet<CanonicalStateClass> =
            manifest.states.mandatory.iter().copied().collect();
        let primitive_mandatory: BTreeSet<CanonicalStateClass> = primitive
            .state_render_plans
            .iter()
            .filter(|p| p.mandatory)
            .map(|p| p.state)
            .collect();
        if manifest_mandatory != primitive_mandatory {
            findings.push(finding(
                primitive,
                "mandatory_states_mismatch",
                "primitive mandatory render plans do not match the manifest mandatory states",
            ));
        }
    }
    findings
}

fn finding(primitive: &M5HostPrimitive, code: &str, detail: &str) -> M5PrimitiveAlignmentFinding {
    M5PrimitiveAlignmentFinding {
        primitive_id: primitive.primitive_id.clone(),
        component_id: primitive.component_id.clone(),
        code: code.to_owned(),
        detail: detail.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers.
// ---------------------------------------------------------------------------

fn validate_primitive_set(
    library: &M5HostPrimitiveLibrary,
    violations: &mut Vec<M5HostPrimitiveViolation>,
) {
    let present: BTreeSet<M5ComponentKind> = library
        .primitives
        .iter()
        .map(|p| p.component_kind)
        .collect();
    for required in M5ComponentKind::ALL {
        if !present.contains(&required) {
            violations.push(M5HostPrimitiveViolation::RequiredPrimitiveKindMissing);
            break;
        }
    }
    if present.len() != library.primitives.len() {
        violations.push(M5HostPrimitiveViolation::DuplicatePrimitiveKind);
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for primitive in &library.primitives {
        if !seen_ids.insert(primitive.primitive_id.as_str()) {
            violations.push(M5HostPrimitiveViolation::DuplicatePrimitiveId);
        }
        validate_primitive(primitive, violations);
    }
}

fn validate_primitive(primitive: &M5HostPrimitive, violations: &mut Vec<M5HostPrimitiveViolation>) {
    if primitive.primitive_id.trim().is_empty()
        || primitive.component_id.trim().is_empty()
        || primitive.display_name.trim().is_empty()
        || primitive.accessibility_role.trim().is_empty()
        || primitive.summary_message_id.trim().is_empty()
    {
        violations.push(M5HostPrimitiveViolation::PrimitiveIncomplete);
    }
    if !primitive
        .summary_message_id
        .starts_with(M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX)
    {
        violations.push(M5HostPrimitiveViolation::MessageIdPrefixMissing);
    }
    if primitive.keyboard_chords.is_empty()
        || primitive
            .keyboard_chords
            .iter()
            .any(|c| c.trim().is_empty())
    {
        violations.push(M5HostPrimitiveViolation::KeyboardIncomplete);
    }
    if primitive.token_references.is_empty()
        || primitive
            .token_references
            .iter()
            .any(|t| t.trim().is_empty())
    {
        violations.push(M5HostPrimitiveViolation::TokenReferencesIncomplete);
    }

    validate_state_plans(primitive, violations);
    validate_appearance(&primitive.appearance, violations);
    validate_consumers(primitive, violations);
}

fn validate_state_plans(
    primitive: &M5HostPrimitive,
    violations: &mut Vec<M5HostPrimitiveViolation>,
) {
    let states: BTreeSet<CanonicalStateClass> = primitive
        .state_render_plans
        .iter()
        .map(|p| p.state)
        .collect();
    let canonical: BTreeSet<CanonicalStateClass> =
        CanonicalStateClass::required().iter().copied().collect();
    let no_duplicates = states.len() == primitive.state_render_plans.len();
    let any_mandatory = primitive.state_render_plans.iter().any(|p| p.mandatory);
    if states != canonical || !no_duplicates || !any_mandatory {
        violations.push(M5HostPrimitiveViolation::StatePlansIncomplete);
    }

    for plan in &primitive.state_render_plans {
        let labelled = plan.non_color_cues.contains(&NonColorCueClass::LabelText);
        if plan.rendered_parts.is_empty()
            || plan.rendered_parts.iter().any(|p| p.trim().is_empty())
            || !labelled
            || !plan
                .status_message_id
                .starts_with(M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX)
        {
            violations.push(M5HostPrimitiveViolation::RenderPlanIncomplete);
        }
    }
}

fn validate_appearance(
    appearance: &M5AppearanceBinding,
    violations: &mut Vec<M5HostPrimitiveViolation>,
) {
    let density: BTreeSet<&str> = appearance
        .density_classes
        .iter()
        .map(|d| d.token())
        .collect();
    let full_density: BTreeSet<&str> = [
        DensityClass::Compact,
        DensityClass::Standard,
        DensityClass::Comfortable,
    ]
    .iter()
    .map(|d| d.token())
    .collect();

    let motion: BTreeSet<&str> = appearance
        .motion_postures
        .iter()
        .map(|m| m.token())
        .collect();
    let required_motion = [
        AccessibilityPostureClass::MotionStandard.token(),
        AccessibilityPostureClass::MotionReduced.token(),
        AccessibilityPostureClass::MotionPowerSaver.token(),
    ];

    let contrast: BTreeSet<&str> = appearance
        .contrast_classes
        .iter()
        .map(|c| c.token())
        .collect();
    let required_contrast = [
        ThemeClass::HighContrastDark.token(),
        ThemeClass::HighContrastLight.token(),
    ];

    if density != full_density
        || required_motion.iter().any(|m| !motion.contains(m))
        || required_contrast.iter().any(|c| !contrast.contains(c))
        || !appearance.honors_focus_order
        || !appearance.honors_keyboard_model
        || !appearance.honors_high_contrast
        || !appearance.honors_reduced_motion
    {
        violations.push(M5HostPrimitiveViolation::AppearanceIncomplete);
    }
}

fn validate_consumers(primitive: &M5HostPrimitive, violations: &mut Vec<M5HostPrimitiveViolation>) {
    if primitive.consumers.is_empty() {
        violations.push(M5HostPrimitiveViolation::ConsumersIncomplete);
    }
    for consumer in &primitive.consumers {
        if consumer.surface_class.trim().is_empty() || consumer.display_name.trim().is_empty() {
            violations.push(M5HostPrimitiveViolation::ConsumerIncomplete);
        }
        match consumer.posture {
            M5ConformancePosture::ReducedWithPartialBadge => {
                if !consumer.consumer_class.is_embedded_or_extension() {
                    violations.push(M5HostPrimitiveViolation::FirstPartyCannotReduce);
                }
                let has_prefixed_badge = consumer
                    .partial_badge_message_id
                    .as_deref()
                    .map(|b| {
                        !b.trim().is_empty() && b.starts_with(M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX)
                    })
                    .unwrap_or(false);
                if !has_prefixed_badge {
                    violations.push(M5HostPrimitiveViolation::PartialBadgeMissing);
                }
            }
            M5ConformancePosture::InheritedHostRendered => {
                if consumer.partial_badge_message_id.is_some() {
                    violations.push(M5HostPrimitiveViolation::InheritedMustNotBadge);
                }
            }
        }
    }
}

fn validate_consumer_coverage(
    library: &M5HostPrimitiveLibrary,
    violations: &mut Vec<M5HostPrimitiveViolation>,
) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut duplicate = false;
    for (_, consumer) in library.all_consumers() {
        if !seen.insert(consumer.surface_class.as_str()) {
            duplicate = true;
        }
    }
    if duplicate {
        violations.push(M5HostPrimitiveViolation::DuplicateConsumerSurface);
    }
    if REQUIRED_CONSUMER_SURFACES.iter().any(|s| !seen.contains(s)) {
        violations.push(M5HostPrimitiveViolation::RequiredConsumerSurfaceMissing);
    }
}

/// True when `value` is a `MAJOR.MINOR.PATCH` numeric semver.
fn is_semver(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material. Host primitives are
/// metadata-only by construction; this is a defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
