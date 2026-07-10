//! Two reusable M5 scaffold / project-entry components — the scaffold template card and the
//! starter parameter row — so a user can tell who authored a starter, how it is supported, and
//! which values come from where before creation begins: the template card names its template
//! name and version, its starter source class, its template support class, its target runtime
//! and toolchain, its host boundary, and its setup-task or extension impact, and offers a
//! first-class open-manifest action; the parameter row names its parameter name and type, its
//! required / optional state, its source precedence (`Template default`, `User input`,
//! `Workspace value`, `Policy value`, or `Secret reference`), its validation state, whether its
//! action is applied immediately or deferred, and an explicit persistence / portability cue.
//!
//! Aureline's frozen scaffold-component matrix
//! ([`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`])
//! names the scaffold template card and the starter parameter row as two governed component
//! families and freezes their controlled vocabulary — the starter source classes
//! (`first_party_starter`, `team_managed_starter`, `community_starter`, `local_only_starter`,
//! `mirrored_starter`, `unknown_source_starter`) and template support classes
//! (`officially_supported`, `community_supported`, `experimental`, `bridge_behavior`,
//! `deprecated`, `unsupported`) a template card binds; the parameter source layers
//! (`default_value`, `user_provided`, `profile_inherited`, `environment_derived`,
//! `computed_derived`, `unset_required`) and action timings (`applied_immediately`,
//! `deferred_after_create`, `requires_confirmation`, `blocked_invalid`, `optional_skippable`,
//! `not_applicable`) a parameter row binds; the one controlled disposition vocabulary; the
//! surface families; the deployment lines; the consumer surfaces; the accessibility routes; the
//! required labels; and the downgrade triggers. This module *implements* that contract as two
//! co-equal component vectors so a claimed M5 start-center, template-gallery, parameter-form, or
//! CLI surface can project a template card and a parameter row that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_template_posture`] — takes a template card's frozen starter source class and
//!    template support class and derives its source class (first-party, team-managed, community,
//!    local, or unknown), its support posture (fully supported, community supported,
//!    experimental / bridge, or unsupported / deprecated), whether the source is a governed
//!    first-party source, whether the support is exact first-party support, and which notes the
//!    card must carry — so a community, local, mirrored, or unknown starter can never read as a
//!    governed first-party starter and bridge or heuristic behavior can never read as exact
//!    first-party support before a user commits.
//! 2. [`resolve_parameter_disclosure`] — takes a parameter row's source-precedence origin class
//!    and its frozen action timing and derives its portability class (portable template value,
//!    portable user value, workspace-scoped value, policy-managed value, or secret reference not
//!    persisted), whether the value is portable, and which notes the row must carry — so a
//!    workspace-scoped value can never read as portable, a policy-managed value can never read as
//!    user input, and a secret reference never reveals a raw secret value.
//!
//! A single controls packet — [`ScaffoldTemplateCardStarterParameterRowControlsPacket`] — binds
//! one vector of template cards and one vector of parameter rows to the same source / support,
//! host-boundary, precedence, deep-link, and non-visual accessibility vocabulary, so starter
//! identity and parameter provenance stay explicit across desktop, headless / export, and
//! support consumers.
//!
//! The starter source class ([`M5StarterSourceClass`]), template support class
//! ([`M5TemplateSupportClass`]), parameter source layer ([`M5ParameterSourceLayer`]), parameter
//! action timing ([`M5ParameterActionTiming`]), disposition ([`M5ScaffoldDisposition`]), surface
//! family ([`M5ScaffoldSurfaceFamily`]), deployment line ([`M5ScaffoldDeploymentLine`]), consumer
//! surface ([`M5ScaffoldConsumerSurface`]), accessibility route
//! ([`M5ScaffoldAccessibilityRoute`]), required label ([`M5ScaffoldRequiredLabel`]), and
//! downgrade trigger ([`M5ScaffoldDowngradeTrigger`]) are reused verbatim from the frozen matrix.
//! This module mints new vocabulary only for what that matrix left implicit about the two
//! components themselves: the derived source and support classes, the source-precedence origin
//! class the acceptance criteria pin, the derived portability class, the bounded template-card
//! and parameter-row actions, and the deep-link kinds. No M5 project-entry surface invents a
//! second template-card or parameter-row grammar.
//!
//! Raw file bodies, raw secret values, pasted local paths, repository URLs, credentials, and
//! secrets stay outside the export boundary; every note, deep-link reference, and component
//! identity is carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The starter source classes and template support classes, the parameter source layers and
// action timings, the disposition vocabulary, and the surface / deployment / consumer /
// accessibility / label / downgrade vocabularies are frozen once, in the scaffold-component
// matrix. This lane reuses them verbatim so it never invents a parallel template-card or
// parameter-row vocabulary.
pub use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5ParameterActionTiming, M5ParameterSourceLayer, M5ScaffoldAccessibilityRoute,
    M5ScaffoldComponentFamily, M5ScaffoldConsumerSurface, M5ScaffoldDeploymentLine,
    M5ScaffoldDisposition, M5ScaffoldDowngradeTrigger, M5ScaffoldRequiredLabel,
    M5ScaffoldSurfaceFamily, M5StarterSourceClass, M5TemplateSupportClass,
    M5_SCAFFOLD_COMPONENT_DOC_REF, M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF, M5_STARTER_PARAMETER_ROW_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`ScaffoldTemplateCardStarterParameterRowControlsPacket`].
pub const SCAFFOLD_ENTRY_CONTROLS_RECORD_KIND: &str =
    "implement_scaffold_template_cards_and_starter_parameter_rows_with_source_support_host_boundary_and_portability_truth_across_claimed_m5_project_entry_surfaces";

/// Schema version for M5 scaffold-template-card / starter-parameter-row control records.
pub const SCAFFOLD_ENTRY_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-template-card-starter-parameter-row-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const SCAFFOLD_ENTRY_CONTROLS_DOC_REF: &str =
    "docs/templates/m5_scaffold_template_card_starter_parameter_row_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const SCAFFOLD_ENTRY_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-scaffold-template-card-starter-parameter-row-controls";

/// Repo-relative path of the checked support-export artifact.
pub const SCAFFOLD_ENTRY_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-scaffold-template-card-starter-parameter-row-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SCAFFOLD_ENTRY_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-scaffold-template-card-starter-parameter-row-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const SCAFFOLD_ENTRY_CONTROLS_REPORT_REF: &str =
    "artifacts/design/m5-scaffold-template-card-starter-parameter-row.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a scaffold-entry component binds its next step against, so a
/// template card or parameter row never routes through an ephemeral overlay — every next step is
/// a stable template manifest, starter-registry entry, docs, or policy reference the user can
/// reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable template-manifest reference.
    TemplateManifest,
    /// A stable starter-registry entry reference.
    StarterRegistryEntry,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable policy reference.
    PolicyReference,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TemplateManifest,
        Self::StarterRegistryEntry,
        Self::DocsAnchor,
        Self::PolicyReference,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateManifest => "template_manifest",
            Self::StarterRegistryEntry => "starter_registry_entry",
            Self::DocsAnchor => "docs_anchor",
            Self::PolicyReference => "policy_reference",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- scaffold-template-card vocabulary ----------------------------------

/// Derived source class a scaffold template card may present.
///
/// This is the template-card source honesty axis: the class is derived from the frozen starter
/// source class, never asserted, so a community, local, mirrored, or unknown starter can never
/// present as a governed first-party starter and a user can always tell who authored a starter
/// before committing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSourceClass {
    /// A first-party starter shipped by Aureline.
    FirstPartyTemplate,
    /// A team-managed starter from a governed registry.
    TeamManagedTemplate,
    /// A community starter.
    CommunityTemplate,
    /// A local-only or mirrored starter on this machine or mirror.
    LocalTemplate,
    /// A starter whose source could not be resolved.
    SourceUnknown,
}

impl TemplateSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstPartyTemplate,
        Self::TeamManagedTemplate,
        Self::CommunityTemplate,
        Self::LocalTemplate,
        Self::SourceUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyTemplate => "first_party_template",
            Self::TeamManagedTemplate => "team_managed_template",
            Self::CommunityTemplate => "community_template",
            Self::LocalTemplate => "local_template",
            Self::SourceUnknown => "source_unknown",
        }
    }

    /// True when the starter comes from a governed first-party source (first-party or
    /// team-managed).
    pub const fn is_governed_first_party(self) -> bool {
        matches!(self, Self::FirstPartyTemplate | Self::TeamManagedTemplate)
    }
}

/// Derived support posture a scaffold template card may present.
///
/// This is the template-card support honesty axis: the posture is derived from the frozen
/// template support class, never asserted, so bridge or heuristic behavior can never present as
/// exact first-party support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSupportPosture {
    /// Officially / fully supported.
    FullySupported,
    /// Community-supported, best effort.
    CommunitySupported,
    /// Experimental or bridge behavior, not exact first-party support.
    ExperimentalOrBridge,
    /// Unsupported or deprecated.
    UnsupportedOrDeprecated,
}

impl TemplateSupportPosture {
    /// Every support posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullySupported,
        Self::CommunitySupported,
        Self::ExperimentalOrBridge,
        Self::UnsupportedOrDeprecated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::CommunitySupported => "community_supported",
            Self::ExperimentalOrBridge => "experimental_or_bridge",
            Self::UnsupportedOrDeprecated => "unsupported_or_deprecated",
        }
    }

    /// True only when the starter carries exact first-party support.
    pub const fn is_exact_first_party_support(self) -> bool {
        matches!(self, Self::FullySupported)
    }
}

/// One keyboard-complete default action a scaffold template card offers, so a card never hides
/// its open-manifest, inspect, or host-boundary affordance behind a pointer-only gesture and
/// never routes creation through a generic Create. `OpenManifest`, `InspectSourceAndSupport`,
/// and `ReviewHostBoundary` are always offered so source, support, and host posture are
/// inspectable before any commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCardAction {
    /// Open the template manifest (always available).
    OpenManifest,
    /// Inspect the starter source and support class (always available).
    InspectSourceAndSupport,
    /// Review the host boundary the starter runs against (always available).
    ReviewHostBoundary,
    /// Start from this template (routes through preflight, never a generic Create).
    StartFromTemplate,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
    /// Copy the stable template id.
    CopyTemplateId,
}

impl TemplateCardAction {
    /// Every template-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenManifest,
        Self::InspectSourceAndSupport,
        Self::ReviewHostBoundary,
        Self::StartFromTemplate,
        Self::OpenDeepLink,
        Self::CopyTemplateId,
    ];

    /// The default actions every keyboard-complete template card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenManifest,
        Self::InspectSourceAndSupport,
        Self::ReviewHostBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenManifest => "open_manifest",
            Self::InspectSourceAndSupport => "inspect_source_and_support",
            Self::ReviewHostBoundary => "review_host_boundary",
            Self::StartFromTemplate => "start_from_template",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopyTemplateId => "copy_template_id",
        }
    }
}

/// Disclosures a scaffold template card must carry, derived from the starter source class and
/// the template support class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateCardDisclosure {
    /// The derived source class this card may present.
    pub source_class: TemplateSourceClass,
    /// The derived support posture this card may present.
    pub support_posture: TemplateSupportPosture,
    /// Whether the starter comes from a governed first-party source.
    pub is_governed_first_party: bool,
    /// Whether the starter carries exact first-party support.
    pub is_exact_first_party_support: bool,
    /// Whether the card must carry an explicit community-source note.
    pub needs_community_source_note: bool,
    /// Whether the card must carry an explicit local / mirrored-source note.
    pub needs_local_source_note: bool,
    /// Whether the card must carry an explicit unknown-source note.
    pub needs_unknown_source_note: bool,
    /// Whether the card must carry an explicit not-exactly-first-party support note.
    pub needs_nonexact_support_note: bool,
}

/// Resolves the source and support truth a scaffold template card may present.
///
/// A `first_party_starter` is a first-party template, a `team_managed_starter` is team-managed,
/// a `community_starter` is community, a `local_only_starter` or `mirrored_starter` is a local
/// template, and an `unknown_source_starter` is source-unknown, so a starter Aureline did not
/// author can never read as governed first-party. An `officially_supported` starter is fully
/// supported; a `community_supported` one is community-supported; an `experimental` or
/// `bridge_behavior` one is experimental / bridge; a `deprecated` or `unsupported` one is
/// unsupported / deprecated — so bridge or heuristic behavior can never read as exact
/// first-party support.
pub fn resolve_template_posture(
    source: M5StarterSourceClass,
    support: M5TemplateSupportClass,
) -> TemplateCardDisclosure {
    use M5StarterSourceClass as Source;
    use M5TemplateSupportClass as Support;
    use TemplateSourceClass as SrcClass;
    use TemplateSupportPosture as Posture;

    let source_class = match source {
        Source::FirstPartyStarter => SrcClass::FirstPartyTemplate,
        Source::TeamManagedStarter => SrcClass::TeamManagedTemplate,
        Source::CommunityStarter => SrcClass::CommunityTemplate,
        Source::LocalOnlyStarter | Source::MirroredStarter => SrcClass::LocalTemplate,
        Source::UnknownSourceStarter => SrcClass::SourceUnknown,
    };
    let support_posture = match support {
        Support::OfficiallySupported => Posture::FullySupported,
        Support::CommunitySupported => Posture::CommunitySupported,
        Support::Experimental | Support::BridgeBehavior => Posture::ExperimentalOrBridge,
        Support::Deprecated | Support::Unsupported => Posture::UnsupportedOrDeprecated,
    };

    TemplateCardDisclosure {
        source_class,
        support_posture,
        is_governed_first_party: source_class.is_governed_first_party(),
        is_exact_first_party_support: support_posture.is_exact_first_party_support(),
        needs_community_source_note: matches!(source_class, SrcClass::CommunityTemplate),
        needs_local_source_note: matches!(source_class, SrcClass::LocalTemplate),
        needs_unknown_source_note: matches!(source_class, SrcClass::SourceUnknown),
        needs_nonexact_support_note: !support_posture.is_exact_first_party_support(),
    }
}

/// A scaffold template card naming its template name and version, starter source class, template
/// support class, target runtime and toolchain, host boundary, setup-task or extension impact,
/// derived source class and support posture, bounded open-manifest / inspect / host-boundary
/// actions, and a stable manifest / registry / docs / policy deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldTemplateCard {
    /// Frozen component this control implements; must be `scaffold_template_card`.
    pub component: M5ScaffoldComponentFamily,
    /// Stable template id.
    pub template_id: String,
    /// Human-readable template name; required and non-empty.
    pub template_name: String,
    /// Template version; required and non-empty.
    pub template_version: String,
    /// Starter source class, reused from the frozen matrix.
    pub source_class: M5StarterSourceClass,
    /// Template support class, reused from the frozen matrix.
    pub support_class: M5TemplateSupportClass,
    /// Derived source class (must equal the resolved class).
    pub derived_source_class: TemplateSourceClass,
    /// Derived support posture (must equal the resolved posture).
    pub derived_support_posture: TemplateSupportPosture,
    /// Whether the card claims a governed first-party source (must equal derived truth).
    pub claims_governed_first_party: bool,
    /// Whether the card claims exact first-party support (must equal derived truth).
    pub claims_exact_first_party_support: bool,
    /// Community-source note; required when the source is community.
    pub community_source_note: String,
    /// Local / mirrored-source note; required when the source is local or mirrored.
    pub local_source_note: String,
    /// Unknown-source note; required when the source could not be resolved.
    pub unknown_source_note: String,
    /// Not-exactly-first-party support note; required when support is not fully supported.
    pub nonexact_support_note: String,
    /// Source / support note; always required so who authored a starter and how it is supported
    /// stays explicit.
    pub source_and_support_note: String,
    /// Target runtime label; always required.
    pub target_runtime_label: String,
    /// Target toolchain label; always required.
    pub toolchain_label: String,
    /// Host boundary label; always required so where the starter runs stays explicit.
    pub host_boundary_label: String,
    /// Setup-task or extension impact label; always required so file / dependency / task /
    /// extension impact stays explicit.
    pub setup_task_or_extension_impact_label: String,
    /// Opaque stable manifest reference; always required.
    pub manifest_ref: String,
    /// Context note; always required so the card names what to check before committing.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open-manifest / inspect / host-boundary).
    pub card_actions: Vec<TemplateCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its starter source or support class. MUST be `false`.
    pub hides_starter_source_or_support_class: bool,
    /// Hard invariant: never hides a side effect behind a generic Create or leaves the host
    /// boundary unstated. MUST be `false`.
    pub hides_side_effect_or_host_boundary: bool,
    /// Hard invariant: never exposes a raw secret or file body by default. MUST be `false`.
    pub exposes_secret_or_raw_value_by_default: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ScaffoldTemplateCard {
    /// Source / support disclosures this card must carry, derived from the frozen classes.
    pub fn posture_disclosure(&self) -> TemplateCardDisclosure {
        resolve_template_posture(self.source_class, self.support_class)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<TemplateCardAction> = self.card_actions.iter().copied().collect();
        TemplateCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions
            .contains(&TemplateCardAction::OpenDeepLink)
    }
}

// ---- starter-parameter-row vocabulary -----------------------------------

/// Source-precedence origin class a starter parameter row must name. These are the exact
/// acceptance-criteria labels so no surface hides source precedence inside implementation
/// detail: a value is a `Template default`, a `User input`, a `Workspace value`, a `Policy
/// value`, or a `Secret reference`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterOriginClass {
    /// A template default value.
    TemplateDefault,
    /// A user-entered value.
    UserInput,
    /// A value inherited from the workspace.
    WorkspaceValue,
    /// A value set by policy.
    PolicyValue,
    /// A reference to a secret (never the raw secret).
    SecretReference,
}

impl ParameterOriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TemplateDefault,
        Self::UserInput,
        Self::WorkspaceValue,
        Self::PolicyValue,
        Self::SecretReference,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateDefault => "template_default",
            Self::UserInput => "user_input",
            Self::WorkspaceValue => "workspace_value",
            Self::PolicyValue => "policy_value",
            Self::SecretReference => "secret_reference",
        }
    }
}

/// Derived portability class a starter parameter row may present.
///
/// This is the parameter-row persistence / portability honesty axis: the class is derived from
/// the source-precedence origin class, never asserted, so a workspace-scoped value can never
/// read as portable, a policy-managed value can never read as user input, and a secret reference
/// is never persisted with the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterPortabilityClass {
    /// A portable template default value.
    PortableTemplateValue,
    /// A portable user-entered value.
    PortableUserValue,
    /// A workspace-scoped value that does not travel between workspaces.
    WorkspaceScopedValue,
    /// A policy-managed value that the workspace does not own.
    PolicyManagedValue,
    /// A secret reference that is not persisted with the project.
    SecretReferenceNotPersisted,
}

impl ParameterPortabilityClass {
    /// Every portability class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PortableTemplateValue,
        Self::PortableUserValue,
        Self::WorkspaceScopedValue,
        Self::PolicyManagedValue,
        Self::SecretReferenceNotPersisted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableTemplateValue => "portable_template_value",
            Self::PortableUserValue => "portable_user_value",
            Self::WorkspaceScopedValue => "workspace_scoped_value",
            Self::PolicyManagedValue => "policy_managed_value",
            Self::SecretReferenceNotPersisted => "secret_reference_not_persisted",
        }
    }

    /// True when the value travels with the project (portable template or user value).
    pub const fn is_portable(self) -> bool {
        matches!(self, Self::PortableTemplateValue | Self::PortableUserValue)
    }
}

/// Whether a starter parameter is required, optional, or conditionally required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRequirement {
    /// Required.
    Required,
    /// Optional.
    Optional,
    /// Conditionally required.
    ConditionallyRequired,
}

impl ParameterRequirement {
    /// Every requirement state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Required, Self::Optional, Self::ConditionallyRequired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::ConditionallyRequired => "conditionally_required",
        }
    }
}

/// Validation state a starter parameter row must name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterValidationState {
    /// The value validates.
    Valid,
    /// The value fails validation.
    Invalid,
    /// The value has not been validated yet.
    Unvalidated,
    /// Validation is pending an external check.
    Pending,
    /// Validation does not apply to this value.
    NotApplicable,
}

impl ParameterValidationState {
    /// Every validation state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Valid,
        Self::Invalid,
        Self::Unvalidated,
        Self::Pending,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::Unvalidated => "unvalidated",
            Self::Pending => "pending",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// One keyboard-complete default action a starter parameter row offers, so a row never hides its
/// inspect or review affordance behind a pointer-only gesture. `InspectSource` and
/// `ReviewValidation` are always offered so source precedence and validation stay inspectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRowAction {
    /// Inspect the source precedence for this value (always available).
    InspectSource,
    /// Review the validation state for this value (always available).
    ReviewValidation,
    /// Edit the value.
    EditValue,
    /// Reset the value to its template default.
    ResetToTemplateDefault,
    /// Open the secret reference (routes to the secret manager, never reveals the raw value).
    OpenSecretReference,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
}

impl ParameterRowAction {
    /// Every parameter-row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectSource,
        Self::ReviewValidation,
        Self::EditValue,
        Self::ResetToTemplateDefault,
        Self::OpenSecretReference,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete parameter row must offer.
    pub const MANDATORY: [Self; 2] = [Self::InspectSource, Self::ReviewValidation];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectSource => "inspect_source",
            Self::ReviewValidation => "review_validation",
            Self::EditValue => "edit_value",
            Self::ResetToTemplateDefault => "reset_to_template_default",
            Self::OpenSecretReference => "open_secret_reference",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a starter parameter row must carry, derived from the origin class and the action
/// timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterRowDisclosure {
    /// The derived portability class this row may present.
    pub portability_class: ParameterPortabilityClass,
    /// Whether the value is portable.
    pub is_portable: bool,
    /// Whether the value is a secret reference.
    pub is_secret_reference: bool,
    /// Whether the value is workspace-scoped.
    pub is_workspace_scoped: bool,
    /// Whether the value is policy-managed.
    pub is_policy_managed: bool,
    /// Whether the action is applied immediately.
    pub is_applied_immediately: bool,
    /// Whether the row must carry an explicit secret-reference note.
    pub needs_secret_note: bool,
    /// Whether the row must carry an explicit local-only / workspace-scoped note.
    pub needs_local_only_note: bool,
    /// Whether the row must carry an explicit confirmation note.
    pub needs_confirmation_note: bool,
    /// Whether the row must carry an explicit blocked-invalid note.
    pub needs_blocked_note: bool,
}

/// Resolves the portability and persistence truth a starter parameter row may present.
///
/// A `Template default` is a portable template value, a `User input` is a portable user value, a
/// `Workspace value` is workspace-scoped, a `Policy value` is policy-managed, and a `Secret
/// reference` is not persisted with the project — so a workspace-scoped or policy-managed value
/// can never read as a portable user value and a secret reference never reveals a raw secret.
pub fn resolve_parameter_disclosure(
    origin: ParameterOriginClass,
    timing: M5ParameterActionTiming,
) -> ParameterRowDisclosure {
    use ParameterOriginClass as Origin;
    use ParameterPortabilityClass as Portability;

    let portability_class = match origin {
        Origin::TemplateDefault => Portability::PortableTemplateValue,
        Origin::UserInput => Portability::PortableUserValue,
        Origin::WorkspaceValue => Portability::WorkspaceScopedValue,
        Origin::PolicyValue => Portability::PolicyManagedValue,
        Origin::SecretReference => Portability::SecretReferenceNotPersisted,
    };

    ParameterRowDisclosure {
        portability_class,
        is_portable: portability_class.is_portable(),
        is_secret_reference: matches!(origin, Origin::SecretReference),
        is_workspace_scoped: matches!(origin, Origin::WorkspaceValue),
        is_policy_managed: matches!(origin, Origin::PolicyValue),
        is_applied_immediately: matches!(timing, M5ParameterActionTiming::AppliedImmediately),
        needs_secret_note: matches!(origin, Origin::SecretReference),
        needs_local_only_note: matches!(origin, Origin::WorkspaceValue),
        needs_confirmation_note: matches!(timing, M5ParameterActionTiming::RequiresConfirmation),
        needs_blocked_note: matches!(timing, M5ParameterActionTiming::BlockedInvalid),
    }
}

/// A starter parameter row naming its parameter name and type, required / optional state, source
/// precedence, frozen source layer, action timing, validation state, derived portability class,
/// bounded inspect / review actions, an explicit persistence / portability cue, and a stable
/// deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterParameterRow {
    /// Frozen component this control implements; must be `starter_parameter_row`.
    pub component: M5ScaffoldComponentFamily,
    /// Stable parameter id.
    pub parameter_id: String,
    /// Human-readable parameter name; required and non-empty.
    pub parameter_name: String,
    /// Parameter type label; required and non-empty.
    pub parameter_type_label: String,
    /// Whether the parameter is required, optional, or conditionally required.
    pub requirement: ParameterRequirement,
    /// Parameter source layer, reused from the frozen matrix.
    pub source_layer: M5ParameterSourceLayer,
    /// Source-precedence origin class (the acceptance-criteria label).
    pub origin_class: ParameterOriginClass,
    /// Parameter action timing, reused from the frozen matrix.
    pub action_timing: M5ParameterActionTiming,
    /// Validation state.
    pub validation_state: ParameterValidationState,
    /// Derived portability class (must equal the resolved class).
    pub derived_portability_class: ParameterPortabilityClass,
    /// Whether the row claims the value is portable (must equal derived truth).
    pub claims_portable: bool,
    /// Secret-reference note; required when the value is a secret reference.
    pub secret_reference_note: String,
    /// Local-only / workspace-scoped note; required when the value is workspace-scoped.
    pub local_only_note: String,
    /// Confirmation note; required when the action requires confirmation.
    pub confirmation_note: String,
    /// Blocked-invalid note; required when the action is blocked because the value is invalid.
    pub blocked_note: String,
    /// Source / precedence note; always required so where a value comes from stays explicit.
    pub source_and_precedence_note: String,
    /// Persistence / portability note; always required so persistence / portability stays
    /// explicit.
    pub persistence_or_portability_note: String,
    /// Opaque value display label; always required and never a raw secret value.
    pub value_display_label: String,
    /// Context note; always required so the row names what to check before committing.
    pub context_note: String,
    /// Kind of stable deep link this row binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include inspect-source / review-validation).
    pub row_actions: Vec<ParameterRowAction>,
    /// Dispositions this row binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its parameter source precedence. MUST be `false`.
    pub hides_starter_source_or_support_class: bool,
    /// Hard invariant: never hides a side effect behind a generic Create or leaves the
    /// immediate-versus-deferred boundary unstated. MUST be `false`.
    pub hides_side_effect_or_host_boundary: bool,
    /// Hard invariant: never exposes a raw secret value by default. MUST be `false`.
    pub exposes_secret_or_raw_value_by_default: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl StarterParameterRow {
    /// Portability disclosures this row must carry, derived from the origin class and timing.
    pub fn portability_disclosure(&self) -> ParameterRowDisclosure {
        resolve_parameter_disclosure(self.origin_class, self.action_timing)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ParameterRowAction> = self.row_actions.iter().copied().collect();
        ParameterRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the row offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.row_actions.contains(&ParameterRowAction::OpenDeepLink)
    }
}

/// Whether a required-label list declares all three mandatory labels.
fn declares_mandatory_labels(labels: &[M5ScaffoldRequiredLabel]) -> bool {
    let present: BTreeSet<M5ScaffoldRequiredLabel> = labels.iter().copied().collect();
    M5ScaffoldRequiredLabel::MANDATORY
        .iter()
        .all(|label| present.contains(label))
}

// ---- review blocks ------------------------------------------------------

/// First-glance scaffold-entry review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldEntryReview {
    /// The template card names its starter source and support class.
    pub template_card_shows_source_and_support: bool,
    /// The template card names its host boundary.
    pub template_card_shows_host_boundary: bool,
    /// The template card offers an open-manifest action.
    pub template_card_offers_open_manifest: bool,
    /// The parameter row names its source precedence.
    pub parameter_row_shows_source_precedence: bool,
    /// The parameter row names its validation state and action timing.
    pub parameter_row_shows_validation_and_timing: bool,
    /// The parameter row offers inspect and review.
    pub parameter_row_offers_inspect_and_review: bool,
    /// Source, support, and portability are derived from state, never asserted.
    pub source_support_and_portability_derived_never_asserted: bool,
    /// A community, local, or unknown starter is never shown as governed first-party.
    pub nonfirst_party_never_shown_as_governed_first_party: bool,
    /// Bridge or heuristic behavior is never shown as exact first-party support.
    pub bridge_never_shown_as_exact_first_party_support: bool,
    /// A secret reference never reveals a raw secret value.
    pub secret_reference_never_reveals_raw_value: bool,
    /// Creation never routes through a generic Create that hides side effects.
    pub create_never_generic_hides_side_effects: bool,
    /// Every next step names one stable manifest / registry / docs / policy deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Persistence and portability stay explicit.
    pub persistence_and_portability_always_explicit: bool,
    /// The host boundary stays visible.
    pub host_boundary_always_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ScaffoldEntryReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.template_card_shows_source_and_support
            && self.template_card_shows_host_boundary
            && self.template_card_offers_open_manifest
            && self.parameter_row_shows_source_precedence
            && self.parameter_row_shows_validation_and_timing
            && self.parameter_row_offers_inspect_and_review
            && self.source_support_and_portability_derived_never_asserted
            && self.nonfirst_party_never_shown_as_governed_first_party
            && self.bridge_never_shown_as_exact_first_party_support
            && self.secret_reference_never_reveals_raw_value
            && self.create_never_generic_hides_side_effects
            && self.every_next_step_names_stable_deep_link
            && self.persistence_and_portability_always_explicit
            && self.host_boundary_always_visible
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldEntryConsumerProjection {
    /// The start-center reads a single canonical source.
    pub start_center_reads_single_source: bool,
    /// The template gallery reads a single canonical source.
    pub template_gallery_reads_single_source: bool,
    /// The parameter form reads a single canonical source.
    pub parameter_form_reads_single_source: bool,
    /// Source and support are visible before commit.
    pub source_and_support_visible_before_commit: bool,
    /// Parameter precedence is visible before commit.
    pub parameter_precedence_visible_before_commit: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl ScaffoldEntryConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.start_center_reads_single_source
            && self.template_gallery_reads_single_source
            && self.parameter_form_reads_single_source
            && self.source_and_support_visible_before_commit
            && self.parameter_precedence_visible_before_commit
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldEntryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`ScaffoldTemplateCardStarterParameterRowControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldTemplateCardStarterParameterRowControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Scaffold template cards.
    pub template_cards: Vec<ScaffoldTemplateCard>,
    /// Starter parameter rows.
    pub parameter_rows: Vec<StarterParameterRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Scaffold-entry review block.
    pub scaffold_review: ScaffoldEntryReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldEntryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldEntryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe scaffold-template-card / starter-parameter-row controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldTemplateCardStarterParameterRowControlsPacket {
    /// Record kind; must equal [`SCAFFOLD_ENTRY_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCAFFOLD_ENTRY_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Scaffold template cards.
    pub template_cards: Vec<ScaffoldTemplateCard>,
    /// Starter parameter rows.
    pub parameter_rows: Vec<StarterParameterRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Scaffold-entry review block.
    pub scaffold_review: ScaffoldEntryReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldEntryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldEntryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ScaffoldTemplateCardStarterParameterRowControlsPacket {
    /// Builds a scaffold-template-card / starter-parameter-row controls packet from stable-lane
    /// input.
    pub fn new(input: ScaffoldTemplateCardStarterParameterRowControlsPacketInput) -> Self {
        Self {
            record_kind: SCAFFOLD_ENTRY_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_ENTRY_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            template_cards: input.template_cards,
            parameter_rows: input.parameter_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            scaffold_review: input.scaffold_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the scaffold-template-card / starter-parameter-row control invariants.
    pub fn validate(&self) -> Vec<ScaffoldEntryControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SCAFFOLD_ENTRY_CONTROLS_RECORD_KIND {
            violations.push(ScaffoldEntryControlsViolation::WrongRecordKind);
        }
        if self.schema_version != SCAFFOLD_ENTRY_CONTROLS_SCHEMA_VERSION {
            violations.push(ScaffoldEntryControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ScaffoldEntryControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ScaffoldEntryControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ScaffoldEntryControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_template_cards(self, &mut violations);
        validate_parameter_rows(self, &mut violations);

        if !self.scaffold_review.all_hold() {
            violations.push(ScaffoldEntryControlsViolation::ScaffoldReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ScaffoldEntryControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ScaffoldEntryControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("scaffold entry controls packet serializes"),
        ) {
            violations.push(ScaffoldEntryControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scaffold entry controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,frozen_state,secondary_state,derived,portable_or_first_party,deep_link_kind\n",
        );
        for card in &self.template_cards {
            let disclosure = card.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "scaffold_template_card",
                csv_field(&card.template_id),
                card.source_class.as_str(),
                card.support_class.as_str(),
                disclosure.source_class.as_str(),
                disclosure.is_governed_first_party,
                card.deep_link_kind.as_str(),
            ));
        }
        for row in &self.parameter_rows {
            let disclosure = row.portability_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "starter_parameter_row",
                csv_field(&row.parameter_id),
                row.origin_class.as_str(),
                row.action_timing.as_str(),
                disclosure.portability_class.as_str(),
                disclosure.is_portable,
                row.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let non_first_party = self
            .template_cards
            .iter()
            .filter(|card| !card.posture_disclosure().is_governed_first_party)
            .count();
        let non_portable = self
            .parameter_rows
            .iter()
            .filter(|row| !row.portability_disclosure().is_portable)
            .count();

        let mut out = String::new();
        out.push_str("# Scaffold template cards and starter parameter rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Scaffold template cards: {} ({} not governed first-party)\n",
            self.template_cards.len(),
            non_first_party
        ));
        out.push_str(&format!(
            "- Starter parameter rows: {} ({} not portable)\n",
            self.parameter_rows.len(),
            non_portable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Scaffold template cards\n\n");
        for card in &self.template_cards {
            out.push_str(&format!(
                "- **{}** — source `{}`, support `{}` → `{}` / `{}`, host `{}`, deep link `{}`\n",
                card.template_name,
                card.source_class.as_str(),
                card.support_class.as_str(),
                card.posture_disclosure().source_class.as_str(),
                card.posture_disclosure().support_posture.as_str(),
                card.host_boundary_label,
                card.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Starter parameter rows\n\n");
        for row in &self.parameter_rows {
            out.push_str(&format!(
                "- **{}** — origin `{}`, layer `{}`, timing `{}` → `{}`, deep link `{}`\n",
                row.parameter_name,
                row.origin_class.as_str(),
                row.source_layer.as_str(),
                row.action_timing.as_str(),
                row.portability_disclosure().portability_class.as_str(),
                row.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in scaffold-entry controls export.
#[derive(Debug)]
pub enum ScaffoldEntryControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScaffoldEntryControlsViolation>),
}

impl fmt::Display for ScaffoldEntryControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "scaffold entry controls export parse failed: {error}"
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
                    "scaffold entry controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScaffoldEntryControlsArtifactError {}

/// Validation failures emitted by
/// [`ScaffoldTemplateCardStarterParameterRowControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaffoldEntryControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No scaffold template cards are present.
    TemplateCardsMissing,
    /// A scaffold template card is incomplete.
    TemplateCardIncomplete,
    /// A scaffold template card carries the wrong frozen component class.
    TemplateCardWrongComponentClass,
    /// A template card misrepresents its derived source class or support posture.
    TemplatePostureMisrepresented,
    /// A community-source card does not name its community source.
    CommunitySourceNoteMissing,
    /// A local / mirrored-source card does not name its local source.
    LocalSourceNoteMissing,
    /// An unknown-source card does not name its unknown source.
    UnknownSourceNoteMissing,
    /// A not-exactly-first-party-support card does not name its non-exact support.
    NonexactSupportNoteMissing,
    /// A template card does not name its source / support.
    SourceAndSupportNoteMissing,
    /// A template card does not name its target runtime.
    TargetRuntimeMissing,
    /// A template card does not name its toolchain.
    ToolchainMissing,
    /// A template card does not name its host boundary.
    HostBoundaryMissing,
    /// A template card does not name its setup-task or extension impact.
    ImpactLabelMissing,
    /// A template card does not name its manifest reference.
    ManifestRefMissing,
    /// A template card omits a mandatory open-manifest / inspect / host-boundary action.
    TemplateCardActionsIncomplete,
    /// The template cards do not cover every derived source class.
    SourceClassCoverageMissing,
    /// The template cards do not cover every starter source class.
    StarterSourceClassCoverageMissing,
    /// The template cards do not cover every support posture.
    SupportPostureCoverageMissing,
    /// The template cards do not cover every template support class.
    TemplateSupportClassCoverageMissing,
    /// No starter parameter rows are present.
    ParameterRowsMissing,
    /// A starter parameter row is incomplete.
    ParameterRowIncomplete,
    /// A starter parameter row carries the wrong frozen component class.
    ParameterRowWrongComponentClass,
    /// A parameter row misrepresents its derived portability class.
    PortabilityMisrepresented,
    /// A secret-reference row does not name its secret reference.
    SecretReferenceNoteMissing,
    /// A workspace-scoped row does not name its local-only / workspace scope.
    LocalOnlyNoteMissing,
    /// A requires-confirmation row does not name its confirmation.
    ConfirmationNoteMissing,
    /// A blocked-invalid row does not name its blocked state.
    BlockedNoteMissing,
    /// A parameter row does not name its source / precedence.
    SourceAndPrecedenceNoteMissing,
    /// A parameter row does not name its persistence / portability.
    PersistenceNoteMissing,
    /// A parameter row does not name its opaque value display.
    ValueDisplayMissing,
    /// A parameter row omits a mandatory inspect-source / review-validation action.
    ParameterRowActionsIncomplete,
    /// The parameter rows do not cover every source-precedence origin class.
    OriginClassCoverageMissing,
    /// The parameter rows do not cover every parameter source layer.
    ParameterSourceLayerCoverageMissing,
    /// The parameter rows do not cover every parameter action timing.
    ParameterActionTimingCoverageMissing,
    /// The parameter rows do not cover every derived portability class.
    PortabilityClassCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component hides its starter source or support class.
    StarterSourceOrSupportHidden,
    /// A component hides a side effect behind a generic Create or leaves the host boundary
    /// unstated.
    SideEffectOrHostBoundaryHidden,
    /// A component exposes a raw secret or file body by default.
    SecretOrRawValueExposed,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Scaffold review does not satisfy required invariants.
    ScaffoldReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ScaffoldEntryControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::TemplateCardsMissing => "template_cards_missing",
            Self::TemplateCardIncomplete => "template_card_incomplete",
            Self::TemplateCardWrongComponentClass => "template_card_wrong_component_class",
            Self::TemplatePostureMisrepresented => "template_posture_misrepresented",
            Self::CommunitySourceNoteMissing => "community_source_note_missing",
            Self::LocalSourceNoteMissing => "local_source_note_missing",
            Self::UnknownSourceNoteMissing => "unknown_source_note_missing",
            Self::NonexactSupportNoteMissing => "nonexact_support_note_missing",
            Self::SourceAndSupportNoteMissing => "source_and_support_note_missing",
            Self::TargetRuntimeMissing => "target_runtime_missing",
            Self::ToolchainMissing => "toolchain_missing",
            Self::HostBoundaryMissing => "host_boundary_missing",
            Self::ImpactLabelMissing => "impact_label_missing",
            Self::ManifestRefMissing => "manifest_ref_missing",
            Self::TemplateCardActionsIncomplete => "template_card_actions_incomplete",
            Self::SourceClassCoverageMissing => "source_class_coverage_missing",
            Self::StarterSourceClassCoverageMissing => "starter_source_class_coverage_missing",
            Self::SupportPostureCoverageMissing => "support_posture_coverage_missing",
            Self::TemplateSupportClassCoverageMissing => "template_support_class_coverage_missing",
            Self::ParameterRowsMissing => "parameter_rows_missing",
            Self::ParameterRowIncomplete => "parameter_row_incomplete",
            Self::ParameterRowWrongComponentClass => "parameter_row_wrong_component_class",
            Self::PortabilityMisrepresented => "portability_misrepresented",
            Self::SecretReferenceNoteMissing => "secret_reference_note_missing",
            Self::LocalOnlyNoteMissing => "local_only_note_missing",
            Self::ConfirmationNoteMissing => "confirmation_note_missing",
            Self::BlockedNoteMissing => "blocked_note_missing",
            Self::SourceAndPrecedenceNoteMissing => "source_and_precedence_note_missing",
            Self::PersistenceNoteMissing => "persistence_note_missing",
            Self::ValueDisplayMissing => "value_display_missing",
            Self::ParameterRowActionsIncomplete => "parameter_row_actions_incomplete",
            Self::OriginClassCoverageMissing => "origin_class_coverage_missing",
            Self::ParameterSourceLayerCoverageMissing => "parameter_source_layer_coverage_missing",
            Self::ParameterActionTimingCoverageMissing => {
                "parameter_action_timing_coverage_missing"
            }
            Self::PortabilityClassCoverageMissing => "portability_class_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::StarterSourceOrSupportHidden => "starter_source_or_support_hidden",
            Self::SideEffectOrHostBoundaryHidden => "side_effect_or_host_boundary_hidden",
            Self::SecretOrRawValueExposed => "secret_or_raw_value_exposed",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ScaffoldReviewIncomplete => "scaffold_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable scaffold-entry controls export.
///
/// This is the first real consumer of the scaffold-entry component lane: a start-center,
/// template-gallery, parameter-form, or support-export surface calls it to ingest the canonical
/// components rather than cloning status text.
///
/// # Errors
///
/// Returns [`ScaffoldEntryControlsArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_scaffold_entry_controls_export(
) -> Result<ScaffoldTemplateCardStarterParameterRowControlsPacket, ScaffoldEntryControlsArtifactError>
{
    let packet: ScaffoldTemplateCardStarterParameterRowControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-scaffold-template-card-starter-parameter-row-proof/support_export.json"
        )))
        .map_err(ScaffoldEntryControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScaffoldEntryControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ScaffoldTemplateCardStarterParameterRowControlsPacket,
    violations: &mut Vec<ScaffoldEntryControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF,
        SCAFFOLD_ENTRY_CONTROLS_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF,
        M5_STARTER_PARAMETER_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ScaffoldEntryControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_template_cards(
    packet: &ScaffoldTemplateCardStarterParameterRowControlsPacket,
    violations: &mut Vec<ScaffoldEntryControlsViolation>,
) {
    if packet.template_cards.is_empty() {
        violations.push(ScaffoldEntryControlsViolation::TemplateCardsMissing);
        return;
    }

    let mut source_classes: BTreeSet<TemplateSourceClass> = BTreeSet::new();
    let mut sources: BTreeSet<M5StarterSourceClass> = BTreeSet::new();
    let mut postures: BTreeSet<TemplateSupportPosture> = BTreeSet::new();
    let mut supports: BTreeSet<M5TemplateSupportClass> = BTreeSet::new();

    for card in &packet.template_cards {
        let disclosure = card.posture_disclosure();
        source_classes.insert(disclosure.source_class);
        sources.insert(card.source_class);
        postures.insert(disclosure.support_posture);
        supports.insert(card.support_class);

        if card.template_id.trim().is_empty()
            || card.template_name.trim().is_empty()
            || card.template_version.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(ScaffoldEntryControlsViolation::TemplateCardIncomplete);
        }
        if card.component != M5ScaffoldComponentFamily::ScaffoldTemplateCard {
            violations.push(ScaffoldEntryControlsViolation::TemplateCardWrongComponentClass);
        }
        if card.derived_source_class != disclosure.source_class
            || card.derived_support_posture != disclosure.support_posture
            || card.claims_governed_first_party != disclosure.is_governed_first_party
            || card.claims_exact_first_party_support != disclosure.is_exact_first_party_support
        {
            violations.push(ScaffoldEntryControlsViolation::TemplatePostureMisrepresented);
        }
        if disclosure.needs_community_source_note && card.community_source_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::CommunitySourceNoteMissing);
        }
        if disclosure.needs_local_source_note && card.local_source_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::LocalSourceNoteMissing);
        }
        if disclosure.needs_unknown_source_note && card.unknown_source_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::UnknownSourceNoteMissing);
        }
        if disclosure.needs_nonexact_support_note && card.nonexact_support_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::NonexactSupportNoteMissing);
        }
        if card.source_and_support_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::SourceAndSupportNoteMissing);
        }
        if card.target_runtime_label.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::TargetRuntimeMissing);
        }
        if card.toolchain_label.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::ToolchainMissing);
        }
        if card.host_boundary_label.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::HostBoundaryMissing);
        }
        if card.setup_task_or_extension_impact_label.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::ImpactLabelMissing);
        }
        if card.manifest_ref.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::ManifestRefMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(ScaffoldEntryControlsViolation::TemplateCardActionsIncomplete);
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                hides_starter_source_or_support_class: card.hides_starter_source_or_support_class,
                hides_side_effect_or_host_boundary: card.hides_side_effect_or_host_boundary,
                exposes_secret_or_raw_value_by_default: card.exposes_secret_or_raw_value_by_default,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in TemplateSourceClass::ALL {
        if !source_classes.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::SourceClassCoverageMissing);
            break;
        }
    }
    for required in M5StarterSourceClass::ALL {
        if !sources.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::StarterSourceClassCoverageMissing);
            break;
        }
    }
    for required in TemplateSupportPosture::ALL {
        if !postures.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::SupportPostureCoverageMissing);
            break;
        }
    }
    for required in M5TemplateSupportClass::ALL {
        if !supports.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::TemplateSupportClassCoverageMissing);
            break;
        }
    }
}

fn validate_parameter_rows(
    packet: &ScaffoldTemplateCardStarterParameterRowControlsPacket,
    violations: &mut Vec<ScaffoldEntryControlsViolation>,
) {
    if packet.parameter_rows.is_empty() {
        violations.push(ScaffoldEntryControlsViolation::ParameterRowsMissing);
        return;
    }

    let mut origins: BTreeSet<ParameterOriginClass> = BTreeSet::new();
    let mut layers: BTreeSet<M5ParameterSourceLayer> = BTreeSet::new();
    let mut timings: BTreeSet<M5ParameterActionTiming> = BTreeSet::new();
    let mut portabilities: BTreeSet<ParameterPortabilityClass> = BTreeSet::new();

    for row in &packet.parameter_rows {
        let disclosure = row.portability_disclosure();
        origins.insert(row.origin_class);
        layers.insert(row.source_layer);
        timings.insert(row.action_timing);
        portabilities.insert(disclosure.portability_class);

        if row.parameter_id.trim().is_empty()
            || row.parameter_name.trim().is_empty()
            || row.parameter_type_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ScaffoldEntryControlsViolation::ParameterRowIncomplete);
        }
        if row.component != M5ScaffoldComponentFamily::StarterParameterRow {
            violations.push(ScaffoldEntryControlsViolation::ParameterRowWrongComponentClass);
        }
        if row.derived_portability_class != disclosure.portability_class
            || row.claims_portable != disclosure.is_portable
        {
            violations.push(ScaffoldEntryControlsViolation::PortabilityMisrepresented);
        }
        if disclosure.needs_secret_note && row.secret_reference_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::SecretReferenceNoteMissing);
        }
        if disclosure.needs_local_only_note && row.local_only_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::LocalOnlyNoteMissing);
        }
        if disclosure.needs_confirmation_note && row.confirmation_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::ConfirmationNoteMissing);
        }
        if disclosure.needs_blocked_note && row.blocked_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::BlockedNoteMissing);
        }
        if row.source_and_precedence_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::SourceAndPrecedenceNoteMissing);
        }
        if row.persistence_or_portability_note.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::PersistenceNoteMissing);
        }
        if row.value_display_label.trim().is_empty() {
            violations.push(ScaffoldEntryControlsViolation::ValueDisplayMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(ScaffoldEntryControlsViolation::ParameterRowActionsIncomplete);
        }
        validate_deep_link(
            row.offers_deep_link_action(),
            row.deep_link_kind,
            &row.deep_link_ref,
            &row.context_note,
            violations,
        );
        validate_common_control(
            &row.dispositions,
            &row.downgrade_triggers,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            ControlInvariants {
                hides_starter_source_or_support_class: row.hides_starter_source_or_support_class,
                hides_side_effect_or_host_boundary: row.hides_side_effect_or_host_boundary,
                exposes_secret_or_raw_value_by_default: row.exposes_secret_or_raw_value_by_default,
                invents_alternate_state_label: row.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in ParameterOriginClass::ALL {
        if !origins.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::OriginClassCoverageMissing);
            break;
        }
    }
    for required in M5ParameterSourceLayer::ALL {
        if !layers.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::ParameterSourceLayerCoverageMissing);
            break;
        }
    }
    for required in M5ParameterActionTiming::ALL {
        if !timings.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::ParameterActionTimingCoverageMissing);
            break;
        }
    }
    for required in ParameterPortabilityClass::ALL {
        if !portabilities.contains(&required) {
            violations.push(ScaffoldEntryControlsViolation::PortabilityClassCoverageMissing);
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name
/// its context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<ScaffoldEntryControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ScaffoldEntryControlsViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(ScaffoldEntryControlsViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(ScaffoldEntryControlsViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    hides_starter_source_or_support_class: bool,
    hides_side_effect_or_host_boundary: bool,
    exposes_secret_or_raw_value_by_default: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5ScaffoldDisposition],
    downgrade_triggers: &[M5ScaffoldDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ScaffoldAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<ScaffoldEntryControlsViolation>,
) {
    if dispositions.is_empty() {
        violations.push(ScaffoldEntryControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ScaffoldEntryControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(ScaffoldEntryControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(ScaffoldEntryControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.hides_starter_source_or_support_class {
        violations.push(ScaffoldEntryControlsViolation::StarterSourceOrSupportHidden);
    }
    if invariants.hides_side_effect_or_host_boundary {
        violations.push(ScaffoldEntryControlsViolation::SideEffectOrHostBoundaryHidden);
    }
    if invariants.exposes_secret_or_raw_value_by_default {
        violations.push(ScaffoldEntryControlsViolation::SecretOrRawValueExposed);
    }
    if invariants.invents_alternate_state_label {
        violations.push(ScaffoldEntryControlsViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical scaffold-entry controls packet.
pub const SCAFFOLD_ENTRY_CONTROLS_PACKET_ID: &str =
    "m5-scaffold-template-card-starter-parameter-row-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn template_card_source_refs() -> Vec<String> {
    strings(&[
        M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn parameter_row_source_refs() -> Vec<String> {
    strings(&[
        M5_STARTER_PARAMETER_ROW_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn template_card_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::StarterSourceUnstated,
        M5ScaffoldDowngradeTrigger::SupportClassUnstated,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn parameter_row_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::ParameterSourceUnstated,
        M5ScaffoldDowngradeTrigger::ActionTimingUnstated,
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

/// Builds a scaffold template card, deriving the source class, support posture, first-party and
/// support claims, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn template_card(
    template_id: &str,
    template_name: &str,
    template_version: &str,
    source_class: M5StarterSourceClass,
    support_class: M5TemplateSupportClass,
    target_runtime_label: &str,
    toolchain_label: &str,
    host_boundary_label: &str,
    setup_task_or_extension_impact_label: &str,
    manifest_ref: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    card_actions: Vec<TemplateCardAction>,
    dispositions: Vec<M5ScaffoldDisposition>,
) -> ScaffoldTemplateCard {
    let disclosure = resolve_template_posture(source_class, support_class);
    ScaffoldTemplateCard {
        component: M5ScaffoldComponentFamily::ScaffoldTemplateCard,
        template_id: template_id.to_owned(),
        template_name: template_name.to_owned(),
        template_version: template_version.to_owned(),
        source_class,
        support_class,
        derived_source_class: disclosure.source_class,
        derived_support_posture: disclosure.support_posture,
        claims_governed_first_party: disclosure.is_governed_first_party,
        claims_exact_first_party_support: disclosure.is_exact_first_party_support,
        community_source_note: if disclosure.needs_community_source_note {
            "Community starter; review the source before trusting it as first-party".to_owned()
        } else {
            String::new()
        },
        local_source_note: if disclosure.needs_local_source_note {
            "Local or mirrored starter on this machine; provenance is only as complete as the mirror"
                .to_owned()
        } else {
            String::new()
        },
        unknown_source_note: if disclosure.needs_unknown_source_note {
            "Starter source could not be resolved; do not treat it as a governed first-party starter"
                .to_owned()
        } else {
            String::new()
        },
        nonexact_support_note: if disclosure.needs_nonexact_support_note {
            format!(
                "Support posture is {}; this is not exact first-party support",
                disclosure.support_posture.as_str()
            )
        } else {
            String::new()
        },
        source_and_support_note: format!(
            "Source {}; support {}",
            disclosure.source_class.as_str(),
            disclosure.support_posture.as_str()
        ),
        target_runtime_label: target_runtime_label.to_owned(),
        toolchain_label: toolchain_label.to_owned(),
        host_boundary_label: host_boundary_label.to_owned(),
        setup_task_or_extension_impact_label: setup_task_or_extension_impact_label.to_owned(),
        manifest_ref: manifest_ref.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        card_actions,
        dispositions,
        downgrade_triggers: template_card_downgrade_triggers(),
        required_labels: label_set(M5ScaffoldRequiredLabel::StarterSourceAndSupport),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "template_name",
            "template_version",
            "source_class",
            "support_class",
            "target_runtime_label",
            "host_boundary_label",
            "setup_task_or_extension_impact_label",
            "deep_link_kind",
        ]),
        source_contract_refs: template_card_source_refs(),
        hides_starter_source_or_support_class: false,
        hides_side_effect_or_host_boundary: false,
        exposes_secret_or_raw_value_by_default: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a starter parameter row, deriving the portability class, portable claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn parameter_row(
    parameter_id: &str,
    parameter_name: &str,
    parameter_type_label: &str,
    requirement: ParameterRequirement,
    source_layer: M5ParameterSourceLayer,
    origin_class: ParameterOriginClass,
    action_timing: M5ParameterActionTiming,
    validation_state: ParameterValidationState,
    value_display_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    row_actions: Vec<ParameterRowAction>,
    dispositions: Vec<M5ScaffoldDisposition>,
) -> StarterParameterRow {
    let disclosure = resolve_parameter_disclosure(origin_class, action_timing);
    StarterParameterRow {
        component: M5ScaffoldComponentFamily::StarterParameterRow,
        parameter_id: parameter_id.to_owned(),
        parameter_name: parameter_name.to_owned(),
        parameter_type_label: parameter_type_label.to_owned(),
        requirement,
        source_layer,
        origin_class,
        action_timing,
        validation_state,
        derived_portability_class: disclosure.portability_class,
        claims_portable: disclosure.is_portable,
        secret_reference_note: if disclosure.needs_secret_note {
            "Secret reference only; the raw secret is never stored with the project".to_owned()
        } else {
            String::new()
        },
        local_only_note: if disclosure.needs_local_only_note {
            "Workspace-scoped value; it does not travel with the project between workspaces"
                .to_owned()
        } else {
            String::new()
        },
        confirmation_note: if disclosure.needs_confirmation_note {
            "Applying this value requires explicit confirmation before create".to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "Value is invalid; this parameter is blocked until it is corrected".to_owned()
        } else {
            String::new()
        },
        source_and_precedence_note: format!(
            "Value comes from {}; precedence {}",
            origin_class.as_str(),
            source_layer.as_str()
        ),
        persistence_or_portability_note: format!(
            "Portability {}",
            disclosure.portability_class.as_str()
        ),
        value_display_label: value_display_label.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        row_actions,
        dispositions,
        downgrade_triggers: parameter_row_downgrade_triggers(),
        required_labels: label_set(M5ScaffoldRequiredLabel::SideEffectDisclosure),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "parameter_name",
            "parameter_type_label",
            "requirement",
            "origin_class",
            "source_layer",
            "action_timing",
            "validation_state",
            "deep_link_kind",
        ]),
        source_contract_refs: parameter_row_source_refs(),
        hides_starter_source_or_support_class: false,
        hides_side_effect_or_host_boundary: false,
        exposes_secret_or_raw_value_by_default: false,
        invents_alternate_state_label: false,
    }
}

/// The three mandatory labels plus one extra truth label.
fn label_set(extra: M5ScaffoldRequiredLabel) -> Vec<M5ScaffoldRequiredLabel> {
    let mut labels = M5ScaffoldRequiredLabel::MANDATORY.to_vec();
    labels.push(extra);
    labels
}

fn template_cards() -> Vec<ScaffoldTemplateCard> {
    use DeepLinkKind as Link;
    use M5ScaffoldDisposition as Disp;
    use M5StarterSourceClass as Source;
    use M5TemplateSupportClass as Support;
    use TemplateCardAction as Action;

    vec![
        // 1. First-party / officially supported → governed first-party, exact support.
        template_card(
            "tmpl-react-spa",
            "React SPA starter",
            "4.2.0",
            Source::FirstPartyStarter,
            Support::OfficiallySupported,
            "Node.js 20 LTS",
            "Vite, TypeScript, ESLint",
            "Runs locally in this workspace; no remote provisioning",
            "Writes 24 files, installs 18 dependencies, adds 1 setup task",
            "manifest:starters/react-spa",
            "First-party starter with exact support; open the manifest to see what it writes",
            Link::TemplateManifest,
            "manifest:starters/react-spa",
            vec![
                Action::OpenManifest,
                Action::InspectSourceAndSupport,
                Action::ReviewHostBoundary,
                Action::StartFromTemplate,
                Action::OpenDeepLink,
            ],
            vec![Disp::FirstParty],
        ),
        // 2. Team-managed / community supported → governed first-party, non-exact support.
        template_card(
            "tmpl-internal-service",
            "Internal service starter",
            "2.7.1",
            Source::TeamManagedStarter,
            Support::CommunitySupported,
            "Go 1.22",
            "Go modules, golangci-lint",
            "Runs against the team-managed workspace; provisions a managed namespace on create",
            "Writes 15 files, installs 9 dependencies, adds 2 setup tasks",
            "registry:team/internal-service",
            "Team-managed starter; community-supported, so review before relying on it",
            Link::StarterRegistryEntry,
            "registry:team/internal-service",
            vec![
                Action::OpenManifest,
                Action::InspectSourceAndSupport,
                Action::ReviewHostBoundary,
                Action::StartFromTemplate,
                Action::CopyTemplateId,
            ],
            vec![Disp::TeamManaged, Disp::Warning],
        ),
        // 3. Community / experimental → community source, non-exact support.
        template_card(
            "tmpl-community-cli",
            "Community CLI starter",
            "0.9.0-beta",
            Source::CommunityStarter,
            Support::Experimental,
            "Rust 1.78",
            "Cargo, clippy",
            "Runs locally; downloads the community pack over the network on first use",
            "Writes 11 files, installs 6 dependencies, adds 1 setup task",
            "registry:community/cli-starter",
            "Community starter; experimental support, so treat generated output as a starting point",
            Link::StarterRegistryEntry,
            "registry:community/cli-starter",
            vec![
                Action::OpenManifest,
                Action::InspectSourceAndSupport,
                Action::ReviewHostBoundary,
                Action::OpenDeepLink,
            ],
            vec![Disp::Community, Disp::Warning],
        ),
        // 4. Local-only / bridge behavior → local source, non-exact support.
        template_card(
            "tmpl-local-notebook",
            "Local notebook starter",
            "1.0.0-local",
            Source::LocalOnlyStarter,
            Support::BridgeBehavior,
            "Python 3.11",
            "venv, ruff",
            "Runs entirely on this machine; bridge generation, not exact first-party output",
            "Writes 8 files, installs 4 dependencies, adds 0 setup tasks",
            "manifest:local/notebook-starter",
            "Local-only starter using bridge behavior; not exact first-party generation",
            Link::DocsAnchor,
            "docs:templates/local-starters",
            vec![
                Action::OpenManifest,
                Action::InspectSourceAndSupport,
                Action::ReviewHostBoundary,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalOnly, Disp::Optional],
        ),
        // 5. Mirrored / deprecated → local source, unsupported/deprecated posture.
        template_card(
            "tmpl-mirrored-legacy",
            "Mirrored legacy starter",
            "3.1.4",
            Source::MirroredStarter,
            Support::Deprecated,
            "Node.js 18",
            "Webpack, Babel",
            "Runs from an offline mirror; deprecated, so a newer starter is preferred",
            "Writes 30 files, installs 22 dependencies, adds 3 setup tasks",
            "manifest:mirror/legacy-starter",
            "Mirrored, deprecated starter; provenance is only as complete as the mirror",
            Link::DocsAnchor,
            "docs:templates/deprecated-starters",
            vec![
                Action::OpenManifest,
                Action::InspectSourceAndSupport,
                Action::ReviewHostBoundary,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalOnly, Disp::Warning],
        ),
        // 6. Unknown source / unsupported → source unknown, unsupported posture.
        template_card(
            "tmpl-unknown-source",
            "Unlabeled starter",
            "0.0.0",
            Source::UnknownSourceStarter,
            Support::Unsupported,
            "Runtime unresolved",
            "Toolchain unresolved",
            "Host boundary unresolved; do not run until the source is clarified",
            "File impact unknown until the source is resolved",
            "manifest:unresolved/unknown-starter",
            "Starter source could not be resolved; blocked from creation until clarified",
            Link::NoDeepLink,
            "",
            vec![
                Action::OpenManifest,
                Action::InspectSourceAndSupport,
                Action::ReviewHostBoundary,
            ],
            vec![Disp::Blocked],
        ),
    ]
}

fn parameter_rows() -> Vec<StarterParameterRow> {
    use DeepLinkKind as Link;
    use M5ParameterActionTiming as Timing;
    use M5ParameterSourceLayer as Layer;
    use M5ScaffoldDisposition as Disp;
    use ParameterOriginClass as Origin;
    use ParameterRequirement as Req;
    use ParameterRowAction as Action;
    use ParameterValidationState as Valid;

    vec![
        // 1. Template default → portable template value.
        parameter_row(
            "param-app-name",
            "Application name",
            "string",
            Req::Required,
            Layer::DefaultValue,
            Origin::TemplateDefault,
            Timing::DeferredAfterCreate,
            Valid::Valid,
            "my-app (template default)",
            "Template default; edit it or accept it before create",
            Link::TemplateManifest,
            "manifest:starters/react-spa#app-name",
            vec![
                Action::InspectSource,
                Action::ReviewValidation,
                Action::EditValue,
                Action::ResetToTemplateDefault,
                Action::OpenDeepLink,
            ],
            vec![Disp::Optional],
        ),
        // 2. User input → portable user value, applied immediately.
        parameter_row(
            "param-port",
            "Dev server port",
            "integer",
            Req::Optional,
            Layer::UserProvided,
            Origin::UserInput,
            Timing::AppliedImmediately,
            Valid::Valid,
            "5173 (user input)",
            "User-entered value applied immediately to the dev server config",
            Link::TemplateManifest,
            "manifest:starters/react-spa#port",
            vec![
                Action::InspectSource,
                Action::ReviewValidation,
                Action::EditValue,
                Action::OpenDeepLink,
            ],
            vec![Disp::Optional],
        ),
        // 3. Workspace value → workspace-scoped (needs local-only note), requires confirmation.
        parameter_row(
            "param-registry-url",
            "Package registry",
            "string",
            Req::Required,
            Layer::ProfileInherited,
            Origin::WorkspaceValue,
            Timing::RequiresConfirmation,
            Valid::Unvalidated,
            "internal-registry (workspace value)",
            "Inherited from the workspace; confirm before it is applied to the new project",
            Link::PolicyReference,
            "policy:workspace/registry",
            vec![
                Action::InspectSource,
                Action::ReviewValidation,
                Action::EditValue,
                Action::OpenDeepLink,
            ],
            vec![Disp::Warning],
        ),
        // 4. Policy value → policy-managed, optional skippable.
        parameter_row(
            "param-license",
            "License",
            "enum",
            Req::Required,
            Layer::EnvironmentDerived,
            Origin::PolicyValue,
            Timing::OptionalSkippable,
            Valid::Valid,
            "Apache-2.0 (policy value)",
            "Set by org policy; the workspace does not own this value",
            Link::PolicyReference,
            "policy:org/license",
            vec![
                Action::InspectSource,
                Action::ReviewValidation,
                Action::OpenDeepLink,
            ],
            vec![Disp::Warning],
        ),
        // 5. Secret reference → not persisted (needs secret note), not applicable timing.
        parameter_row(
            "param-api-token",
            "Service token",
            "secret-ref",
            Req::ConditionallyRequired,
            Layer::ComputedDerived,
            Origin::SecretReference,
            Timing::NotApplicable,
            Valid::NotApplicable,
            "secret-ref: service-token (reference only)",
            "Secret reference only; open the secret manager, the raw value is never shown here",
            Link::PolicyReference,
            "policy:secrets/service-token",
            vec![
                Action::InspectSource,
                Action::ReviewValidation,
                Action::OpenSecretReference,
                Action::OpenDeepLink,
            ],
            vec![Disp::Optional],
        ),
        // 6. User input, unset required, blocked-invalid → portable user value, blocked.
        parameter_row(
            "param-owner-email",
            "Owner email",
            "string",
            Req::Required,
            Layer::UnsetRequired,
            Origin::UserInput,
            Timing::BlockedInvalid,
            Valid::Invalid,
            "(unset — required)",
            "Required value is unset and invalid; enter a valid email before create",
            Link::DocsAnchor,
            "docs:templates/required-parameters",
            vec![
                Action::InspectSource,
                Action::ReviewValidation,
                Action::EditValue,
            ],
            vec![Disp::Blocked],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::StarterSourceUnstated,
        M5ScaffoldDowngradeTrigger::SupportClassUnstated,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::ParameterSourceUnstated,
        M5ScaffoldDowngradeTrigger::ActionTimingUnstated,
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn scaffold_review() -> ScaffoldEntryReview {
    ScaffoldEntryReview {
        template_card_shows_source_and_support: true,
        template_card_shows_host_boundary: true,
        template_card_offers_open_manifest: true,
        parameter_row_shows_source_precedence: true,
        parameter_row_shows_validation_and_timing: true,
        parameter_row_offers_inspect_and_review: true,
        source_support_and_portability_derived_never_asserted: true,
        nonfirst_party_never_shown_as_governed_first_party: true,
        bridge_never_shown_as_exact_first_party_support: true,
        secret_reference_never_reveals_raw_value: true,
        create_never_generic_hides_side_effects: true,
        every_next_step_names_stable_deep_link: true,
        persistence_and_portability_always_explicit: true,
        host_boundary_always_visible: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ScaffoldEntryConsumerProjection {
    ScaffoldEntryConsumerProjection {
        start_center_reads_single_source: true,
        template_gallery_reads_single_source: true,
        parameter_form_reads_single_source: true,
        source_and_support_visible_before_commit: true,
        parameter_precedence_visible_before_commit: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> ScaffoldEntryProofFreshness {
    ScaffoldEntryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF,
        SCAFFOLD_ENTRY_CONTROLS_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF,
        M5_STARTER_PARAMETER_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical scaffold-template-card / starter-parameter-row controls packet.
pub fn seeded_scaffold_entry_controls() -> ScaffoldTemplateCardStarterParameterRowControlsPacket {
    ScaffoldTemplateCardStarterParameterRowControlsPacket::new(
        ScaffoldTemplateCardStarterParameterRowControlsPacketInput {
            packet_id: SCAFFOLD_ENTRY_CONTROLS_PACKET_ID.to_owned(),
            surface_label:
                "M5 scaffold template cards and starter parameter rows: starter source, support, host boundary, parameter source precedence, and portability truth across claimed project-entry surfaces"
                    .to_owned(),
            template_cards: template_cards(),
            parameter_rows: parameter_rows(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
            scaffold_review: scaffold_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a community template card that must never read as a governed
/// first-party starter or as exact first-party support. Every source class, starter source
/// class, support posture, and support class stays covered so the fixture validates on its own.
pub fn seeded_scaffold_entry_controls_template_card_community(
) -> ScaffoldTemplateCardStarterParameterRowControlsPacket {
    let mut packet = seeded_scaffold_entry_controls();
    packet.packet_id =
        "m5-scaffold-template-card-starter-parameter-row-controls:fixture:template-card-community"
            .to_owned();
    packet.surface_label =
        "M5 scaffold template cards: a community starter never reads as governed first-party"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a secret-reference parameter row that must never reveal a raw
/// secret value. Every origin class, parameter source layer, action timing, and portability
/// class stays covered so the fixture validates on its own.
pub fn seeded_scaffold_entry_controls_parameter_row_secret_reference(
) -> ScaffoldTemplateCardStarterParameterRowControlsPacket {
    let mut packet = seeded_scaffold_entry_controls();
    packet.packet_id =
        "m5-scaffold-template-card-starter-parameter-row-controls:fixture:parameter-row-secret-reference"
            .to_owned();
    packet.surface_label =
        "M5 starter parameter rows: a secret reference never reveals a raw secret value".to_owned();
    packet
}
