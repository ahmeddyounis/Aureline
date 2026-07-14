//! Implemented M5 side-by-side channel-isolation, association-precedence-review, and artifact-graph
//! rollback-target registries.
//!
//! The frozen [install-topology matrix][matrix] names Aureline's side-by-side stable-plus-preview
//! delivery-topology family and the first implement lane [over the whole matrix][registries] resolves the
//! side-by-side install-topology object. This module is the side-by-side coexistence-execution lane: it makes
//! *side-by-side channel isolation* a contract instead of a set of installer accidents. It turns the *channel
//! root / mutable-state-namespace isolation* grammar and the *file-association / protocol-handler / deep-link /
//! default-open precedence review plus full artifact-graph rollback target* grammar into registry resolvers
//! that produce export-safe, honest projections. A claimed side-by-side profile then resolves to one stable
//! channel-isolation object — the channel, the channel / state-namespace / secrets-namespace roots, the full
//! isolation inventory (channel root, state namespace, secrets namespace, services namespace), and the explicit
//! isolated-versus-governed-handoff containment — that proves a preview or beta channel never reuses the stable
//! durable-state namespace without an explicit governed handoff, and to one association-precedence /
//! rollback-target record — the precedence domain, the disclosed owner channel, precedence rank, conflict
//! resolution, rollback artifact graph, and inspectable-before-and-after posture — that support and admin
//! surfaces can inspect and that drills can fail against when handler ownership drifts to last-writer-wins or a
//! rollback target narrows below the full artifact graph.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Isolate channel roots and mutable-state namespaces unless the user explicitly chooses a supported import
//!   or copy path.** [`resolve_channel_isolation_entry`] refuses to read as a clean, registry-bound channel
//!   unless it names a canonical token, a classified [channel][M5SideBySideChannel], a side-by-side role, covers
//!   every [presentation form][M5ChannelPresentationForm], inventories every mandatory
//!   [isolation field][M5ChannelIsolationField], keeps a disclosed [containment][M5ChannelStateContainment], and
//!   proves no preview / beta channel reused the stable state namespace without a governed handoff; a reuse
//!   degrades to [`M5ChannelIsolationEntryDegradeReason::PreviewCorruptedStableDurableState`].
//! * **Bind rollback targets to the full artifact graph — sidecars, symbols, manifests, and update metadata —
//!   not just the primary executable.** A precedence entry whose rollback posture narrows to the primary
//!   executable while its artifact-graph continuity is undocumented degrades to
//!   [`M5PrecedenceRollbackEntryDegradeReason::RollbackArtifactGraphIncomplete`] so a rollback can never claim to
//!   restore an install truthfully while sidecars or metadata drift.
//! * **Publish precedence rules for file associations, protocol handlers, deep links, and default-open so
//!   ownership never becomes a last-writer-wins accident.** [`resolve_precedence_and_rollback_entry`] names a
//!   classified [precedence domain][M5PrecedenceReviewDomain], must disclose every mandatory
//!   [precedence field][M5PrecedenceReviewField] (owner channel, precedence rank, conflict resolution, rollback
//!   artifact graph, inspectable-before-and-after) and the precedence ownership, and degrades to
//!   [`M5PrecedenceRollbackEntryDegradeReason::HandlerPrecedenceNotInspectable`] when a field or the ownership is
//!   left implicit.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5InstallTopologyRole`] role vocabulary,
//! the [`M5InstallTopologyConsumerSurface`] consumer-surface taxonomy, and the matrix downgrade triggers — so
//! installer, updater, diagnostics, admin, docs, CLI, and support surfaces can never fork their own coexistence
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_install_topology_matrix
//! [registries]: crate::m5_install_topology_and_state_root_registries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_channel_isolation_precedence_review_and_rollback_targets,
    seeded_m5_channel_isolation_precedence_review_and_rollback_targets_offline_airgap_bundle_preview_narrowed,
    seeded_m5_channel_isolation_precedence_review_and_rollback_targets_side_by_side_channel_beta_narrowed,
    M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_topology_and_state_root_registries::M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF;
use crate::m5_install_topology_matrix::{
    M5InstallTopologyAccessibilityRoute, M5InstallTopologyConsumerSurface,
    M5InstallTopologyDeploymentLine, M5InstallTopologyDowngradeTrigger, M5InstallTopologyFamily,
    M5InstallTopologyQualificationClass, M5InstallTopologyRequiredLabel, M5InstallTopologyRole,
    M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF, M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
    M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket`].
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_RECORD_KIND: &str =
    "implement_m5_channel_isolation_precedence_review_and_rollback_targets";

/// Schema version for M5 channel-isolation / precedence-review / rollback-target registry records.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_REF: &str =
    "schemas/install/m5-channel-isolation-precedence-review-and-rollback-targets.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_DOC_REF: &str =
    "docs/install/m5_channel_isolation_precedence_review_and_rollback_targets.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_ARTIFACT_REF: &str =
    "artifacts/release/m5-channel-isolation-precedence-review-and-rollback-targets-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_CSV_REF: &str =
    "artifacts/release/m5-channel-isolation-precedence-review-and-rollback-targets-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_REPORT_REF: &str =
    "artifacts/release/m5-channel-isolation-precedence-review-and-rollback-targets-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_FIXTURE_DIR: &str =
    "fixtures/install/m5-channel-isolation-precedence-review-and-rollback-targets";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5ChannelConsumerSurface = M5InstallTopologyConsumerSurface;

/// One of the three presentation forms every channel-isolation or precedence-rollback entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary,
/// or written to the audit / support record. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelPresentationForm {
    /// The canonical resolved channel-isolation / precedence-rollback object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved coexistence truth discoverable without
    /// visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved coexistence truth inspectable off-renderer.
    AuditRecord,
}

impl M5ChannelPresentationForm {
    /// Every presentation form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled side-by-side channel a channel-isolation entry resolves, so a claimed side-by-side profile
/// exposes stable, preview, beta, and LTS channels living on one machine through one inspectable contract rather
/// than a per-installer accident. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SideBySideChannel {
    /// The stable channel.
    Stable,
    /// The preview channel.
    Preview,
    /// The beta channel.
    Beta,
    /// The long-term-support (LTS) channel.
    Lts,
    /// The channel is unclassified, which is disallowed.
    ChannelUnclassified,
}

impl M5SideBySideChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Preview,
        Self::Beta,
        Self::Lts,
        Self::ChannelUnclassified,
    ];

    /// The four canonical channels every claimed side-by-side profile isolates against.
    pub const CANONICAL_CHANNELS: [Self; 4] = [Self::Stable, Self::Preview, Self::Beta, Self::Lts];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Lts => "lts",
            Self::ChannelUnclassified => "channel_unclassified",
        }
    }

    /// Whether the channel is one of the supported side-by-side channels (never the unclassified sentinel).
    pub const fn is_isolatable(self) -> bool {
        !matches!(self, Self::ChannelUnclassified)
    }
}

/// Controlled isolation field a channel must publish so its durable roots and mutable-state namespaces stay
/// separated instead of sharing one namespace by accident. Minted by this lane, tracking the roots the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelIsolationField {
    /// The isolated channel binary / install root.
    ChannelRoot,
    /// The isolated durable settings state namespace.
    StateNamespace,
    /// The isolated secrets namespace.
    SecretsNamespace,
    /// The isolated services / background-agent namespace.
    ServicesNamespace,
    /// The explicit governed import / handoff path (present only when the user chooses a supported handoff).
    HandoffPath,
}

impl M5ChannelIsolationField {
    /// Every isolation field, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ChannelRoot,
        Self::StateNamespace,
        Self::SecretsNamespace,
        Self::ServicesNamespace,
        Self::HandoffPath,
    ];

    /// The four isolation fields a channel must publish before it can read as complete — the exact durable
    /// roots and mutable-state namespaces the implementation requirement names.
    pub const MANDATORY: [Self; 4] = [
        Self::ChannelRoot,
        Self::StateNamespace,
        Self::SecretsNamespace,
        Self::ServicesNamespace,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelRoot => "channel_root",
            Self::StateNamespace => "state_namespace",
            Self::SecretsNamespace => "secrets_namespace",
            Self::ServicesNamespace => "services_namespace",
            Self::HandoffPath => "handoff_path",
        }
    }
}

/// Controlled containment a channel-isolation entry resolves, so a channel's durable state stays explicitly
/// isolated, governed by a disclosed handoff, or disclosed-shared rather than silently corrupting a coexisting
/// channel. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelStateContainment {
    /// The channel's durable state is fully isolated from every coexisting channel.
    Isolated,
    /// The channel imports another channel's state only through an explicit, user-chosen governed handoff.
    GovernedHandoff,
    /// The channel shares a namespace with another channel and that sharing is explicitly disclosed.
    SharedDisclosed,
    /// The containment cannot be distinguished, which is disallowed.
    ContainmentAmbiguous,
}

impl M5ChannelStateContainment {
    /// Every containment, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Isolated,
        Self::GovernedHandoff,
        Self::SharedDisclosed,
        Self::ContainmentAmbiguous,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Isolated => "isolated",
            Self::GovernedHandoff => "governed_handoff",
            Self::SharedDisclosed => "shared_disclosed",
            Self::ContainmentAmbiguous => "containment_ambiguous",
        }
    }

    /// Whether the containment is disclosed (never the ambiguous sentinel).
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::ContainmentAmbiguous)
    }
}

/// Controlled precedence-review domain a precedence-rollback entry resolves, so file-association,
/// protocol-handler, deep-link, and default-open ownership shares one registry rather than a per-surface,
/// last-writer-wins accident. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrecedenceReviewDomain {
    /// The file-association ownership domain.
    FileAssociation,
    /// The protocol-handler ownership domain.
    ProtocolHandler,
    /// The deep-link ownership domain.
    DeepLink,
    /// The default-open behaviour ownership domain.
    DefaultOpen,
    /// The precedence domain is unclassified, which is disallowed.
    DomainUnclassified,
}

impl M5PrecedenceReviewDomain {
    /// Every precedence domain, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FileAssociation,
        Self::ProtocolHandler,
        Self::DeepLink,
        Self::DefaultOpen,
        Self::DomainUnclassified,
    ];

    /// The four canonical precedence domains the published precedence truth must stay complete across.
    pub const CANONICAL_DOMAINS: [Self; 4] = [
        Self::FileAssociation,
        Self::ProtocolHandler,
        Self::DeepLink,
        Self::DefaultOpen,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileAssociation => "file_association",
            Self::ProtocolHandler => "protocol_handler",
            Self::DeepLink => "deep_link",
            Self::DefaultOpen => "default_open",
            Self::DomainUnclassified => "domain_unclassified",
        }
    }

    /// Whether the precedence domain is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::DomainUnclassified)
    }
}

/// One precedence-review field the published precedence rule must disclose so nothing about the owner channel,
/// precedence rank, conflict resolution, rollback artifact graph, or before/after inspectability is left
/// implicit. Minted by this lane, tracking the fields the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrecedenceReviewField {
    /// The channel that owns the association / handler.
    OwnerChannel,
    /// The precedence rank that resolves ownership deterministically instead of last-writer-wins.
    PrecedenceRank,
    /// The conflict-resolution rule applied when two channels claim the same association.
    ConflictResolution,
    /// The full rollback artifact graph (primary executable, sidecars, symbols, manifests, update metadata).
    RollbackArtifactGraph,
    /// The proof that ownership is inspectable before and after update / import flows.
    InspectableBeforeAndAfter,
}

impl M5PrecedenceReviewField {
    /// Every precedence field, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OwnerChannel,
        Self::PrecedenceRank,
        Self::ConflictResolution,
        Self::RollbackArtifactGraph,
        Self::InspectableBeforeAndAfter,
    ];

    /// Every field is mandatory: a published precedence rule must disclose all five.
    pub const MANDATORY: [Self; 5] = Self::ALL;

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerChannel => "owner_channel",
            Self::PrecedenceRank => "precedence_rank",
            Self::ConflictResolution => "conflict_resolution",
            Self::RollbackArtifactGraph => "rollback_artifact_graph",
            Self::InspectableBeforeAndAfter => "inspectable_before_and_after",
        }
    }
}

/// Controlled rollback-completeness posture a precedence-rollback entry resolves, so a rollback target stays
/// bound to the full artifact graph rather than narrowing to the primary executable while sidecars or metadata
/// drift. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackCompletenessPosture {
    /// The rollback target is bound to the full artifact graph (executable, sidecars, symbols, manifests,
    /// update metadata).
    FullArtifactGraphBound,
    /// The rollback target narrows to the primary executable only, which cannot restore the install truthfully.
    PrimaryExecutableOnly,
    /// The rollback target is a disclosed governed partial (some artifacts restored, the narrowing disclosed).
    GovernedPartialDisclosed,
    /// The posture is unclassified, which is disallowed.
    PostureUnclassified,
}

impl M5RollbackCompletenessPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullArtifactGraphBound,
        Self::PrimaryExecutableOnly,
        Self::GovernedPartialDisclosed,
        Self::PostureUnclassified,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullArtifactGraphBound => "full_artifact_graph_bound",
            Self::PrimaryExecutableOnly => "primary_executable_only",
            Self::GovernedPartialDisclosed => "governed_partial_disclosed",
            Self::PostureUnclassified => "posture_unclassified",
        }
    }

    /// Whether the posture is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PostureUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a coexistence token's
/// meaning stays stable whether it appears in the installer flow, the update flow, diagnostics, admin, or a
/// support / export form. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelSurfaceContext {
    /// The installer flow surface.
    InstallerFlow,
    /// The update flow surface.
    UpdateFlow,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ChannelSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InstallerFlow,
        Self::UpdateFlow,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::InstallerFlow,
        Self::UpdateFlow,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallerFlow => "installer_flow",
            Self::UpdateFlow => "update_flow",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a channel-isolation or precedence-rollback entry must be able to show, so no
/// channel, isolation field, containment, precedence field, rollback posture, or registry fact is left
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The channel the entry resolves (channel entry).
    Channel,
    /// The channel / state-namespace / secrets-namespace roots (channel entry).
    ChannelAndNamespaceRoots,
    /// The presentation-form coverage (canonical / accessible / audit).
    PresentationFormCoverage,
    /// The isolation inventory (channel entry).
    IsolationInventory,
    /// The isolated-versus-governed-handoff containment (channel entry).
    Containment,
    /// The precedence rule and rollback artifact-graph posture (precedence entry).
    PrecedenceAndRollbackPosture,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved channel or precedence record (both entries).
    PlainLanguageMeaning,
}

impl M5ChannelAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::Channel,
        Self::ChannelAndNamespaceRoots,
        Self::PresentationFormCoverage,
        Self::IsolationInventory,
        Self::Containment,
        Self::PrecedenceAndRollbackPosture,
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
            Self::Channel => "channel",
            Self::ChannelAndNamespaceRoots => "channel_and_namespace_roots",
            Self::PresentationFormCoverage => "presentation_form_coverage",
            Self::IsolationInventory => "isolation_inventory",
            Self::Containment => "containment",
            Self::PrecedenceAndRollbackPosture => "precedence_and_rollback_posture",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// channel, a precedence record, or a degraded entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelNextAction {
    /// Expand the resolved channel's or precedence record's plain-language meaning.
    ExpandChannelMeaning,
    /// Inspect the channel or precedence domain the entry resolves.
    InspectChannelOrDomain,
    /// Complete the canonical / accessible / audit presentation-form coverage.
    CompletePresentationFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ChannelNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandChannelMeaning,
        Self::InspectChannelOrDomain,
        Self::CompletePresentationFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandChannelMeaning => "expand_channel_meaning",
            Self::InspectChannelOrDomain => "inspect_channel_or_domain",
            Self::CompletePresentationFormCoverage => "complete_presentation_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The channels covered.
    Channels,
    /// The containments carried.
    Containments,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The presentation forms covered.
    PresentationForms,
    /// The precedence domains carried.
    PrecedenceDomains,
    /// The render / surface context.
    SurfaceContext,
    /// The isolation fields carried.
    IsolationFields,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ChannelExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::Channels,
        Self::Containments,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::PresentationForms,
        Self::PrecedenceDomains,
        Self::SurfaceContext,
        Self::IsolationFields,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::Channels,
        Self::Containments,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::Channels => "channels",
            Self::Containments => "containments",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::PresentationForms => "presentation_forms",
            Self::PrecedenceDomains => "precedence_domains",
            Self::SurfaceContext => "surface_context",
            Self::IsolationFields => "isolation_fields",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a channel-isolation entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, namespace-reusing, inventory-incomplete, or
/// form-incomplete channel read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelIsolationEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the channel means.
    ChannelTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The channel is unclassified (not in the resolved taxonomy).
    ChannelUnclassified,
    /// The behavior is a hand-copied per-profile assumption instead of tracing to the canonical registry.
    ChannelNotBoundToRegistry,
    /// The isolation inventory is incomplete: a mandatory field (channel root, state namespace, secrets
    /// namespace, or services namespace) is not published, or the channel / state-namespace / secrets-namespace
    /// roots are unstated.
    ChannelNamespaceInventoryIncomplete,
    /// A preview or beta channel reused the stable durable-state namespace without an explicit governed handoff,
    /// so coexisting installs corrupt one another's durable state.
    PreviewCorruptedStableDurableState,
    /// The containment is ambiguous, so a coexisting channel could corrupt this channel's durable state.
    ContainmentAmbiguous,
    /// The canonical / accessible / audit presentation-form coverage is incomplete.
    PresentationFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ChannelIsolationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ChannelTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ChannelUnclassified,
        Self::ChannelNotBoundToRegistry,
        Self::ChannelNamespaceInventoryIncomplete,
        Self::PreviewCorruptedStableDurableState,
        Self::ContainmentAmbiguous,
        Self::PresentationFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelTokenUnstated => "channel_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ChannelUnclassified => "channel_unclassified",
            Self::ChannelNotBoundToRegistry => "channel_not_bound_to_registry",
            Self::ChannelNamespaceInventoryIncomplete => "channel_namespace_inventory_incomplete",
            Self::PreviewCorruptedStableDurableState => "preview_corrupted_stable_durable_state",
            Self::ContainmentAmbiguous => "containment_ambiguous",
            Self::PresentationFormCoverageIncomplete => "presentation_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ChannelNextAction {
        match self {
            Self::ChannelTokenUnstated | Self::ChannelNotBoundToRegistry => {
                M5ChannelNextAction::TraceCanonicalRegistry
            }
            Self::ChannelUnclassified
            | Self::ChannelNamespaceInventoryIncomplete
            | Self::PreviewCorruptedStableDurableState
            | Self::ContainmentAmbiguous => M5ChannelNextAction::InspectChannelOrDomain,
            Self::PresentationFormCoverageIncomplete => {
                M5ChannelNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ChannelNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::ChannelTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::PresentationFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ChannelUnclassified => M5InstallTopologyDowngradeTrigger::InstallModeUnstated,
            Self::ChannelNamespaceInventoryIncomplete | Self::ContainmentAmbiguous => {
                M5InstallTopologyDowngradeTrigger::StateRootUnstated
            }
            Self::PreviewCorruptedStableDurableState => {
                M5InstallTopologyDowngradeTrigger::PreviewChannelReusedStableStateNamespaceWithoutHandoff
            }
            Self::ChannelNotBoundToRegistry => {
                M5InstallTopologyDowngradeTrigger::StateRootBoundaryDriftedByTopology
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a precedence-rollback entry degraded below a clean, published state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrecedenceRollbackEntryDegradeReason {
    /// The canonical registry token name is unstated.
    PrecedenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The precedence domain is unclassified (not in the resolved taxonomy).
    PrecedenceDomainUnclassified,
    /// The precedence rule is not inspectable — a mandatory field (owner channel, precedence rank, conflict
    /// resolution, rollback artifact graph, or inspectable-before-and-after) or the precedence ownership is
    /// missing, so handler ownership could become a last-writer-wins accident.
    HandlerPrecedenceNotInspectable,
    /// The rollback target narrowed below the full artifact graph (primary executable only) while sidecars,
    /// symbols, manifests, or update metadata drift, so a rollback cannot restore the prior install truthfully.
    RollbackArtifactGraphIncomplete,
    /// The canonical / accessible / audit presentation-form coverage of the precedence record is incomplete.
    PrecedenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PrecedenceRollbackEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PrecedenceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::PrecedenceDomainUnclassified,
        Self::HandlerPrecedenceNotInspectable,
        Self::RollbackArtifactGraphIncomplete,
        Self::PrecedenceFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrecedenceTokenUnstated => "precedence_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PrecedenceDomainUnclassified => "precedence_domain_unclassified",
            Self::HandlerPrecedenceNotInspectable => "handler_precedence_not_inspectable",
            Self::RollbackArtifactGraphIncomplete => "rollback_artifact_graph_incomplete",
            Self::PrecedenceFormCoverageIncomplete => "precedence_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ChannelNextAction {
        match self {
            Self::PrecedenceTokenUnstated => M5ChannelNextAction::TraceCanonicalRegistry,
            Self::PrecedenceDomainUnclassified
            | Self::HandlerPrecedenceNotInspectable
            | Self::RollbackArtifactGraphIncomplete => M5ChannelNextAction::InspectChannelOrDomain,
            Self::PrecedenceFormCoverageIncomplete => {
                M5ChannelNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ChannelNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::PrecedenceTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::PrecedenceDomainUnclassified
            | Self::PrecedenceFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::HandlerPrecedenceNotInspectable => {
                M5InstallTopologyDowngradeTrigger::UpdaterOwnershipOrAdminControlHiddenInManagedFlow
            }
            Self::RollbackArtifactGraphIncomplete => {
                M5InstallTopologyDowngradeTrigger::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_channel_isolation_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChannelIsolationEntryResolutionInput {
    /// Stable identity of the channel entry.
    pub entry_id: String,
    /// The stable side-by-side-profile ID this channel binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name (e.g. `channel.side_by_side.stable`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The side-by-side channel this entry resolves.
    pub channel: M5SideBySideChannel,
    /// The render / surface context.
    pub surface_context: M5ChannelSurfaceContext,
    /// The presentation forms this entry holds across (must cover canonical / accessible / audit).
    pub presentation_form_coverage: Vec<M5ChannelPresentationForm>,
    /// The published isolated channel binary / install root (a filesystem path, never a URL); empty means
    /// unstated.
    pub channel_root: String,
    /// The published isolated durable-state namespace root (a filesystem path); empty means unstated.
    pub state_namespace_root: String,
    /// The published isolated secrets namespace root (a filesystem path); empty means unstated.
    pub secrets_namespace_root: String,
    /// The isolation fields published by this channel (must cover every mandatory field).
    pub isolation_fields_covered: Vec<M5ChannelIsolationField>,
    /// The isolated-versus-governed-handoff containment distinguishing this channel's durable state.
    pub containment: M5ChannelStateContainment,
    /// True when the behavior traces to the shared coexistence registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when a preview / beta channel actually reused the stable state namespace without a governed handoff.
    pub namespace_reuse_used: bool,
    /// True when honest per-channel state-namespace isolation is enforced, proving absence of reuse.
    pub namespace_isolation_enforced: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe channel-isolation projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChannelIsolationEntry {
    /// Stable identity of the channel entry.
    pub entry_id: String,
    /// The stable side-by-side-profile ID this channel binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The channel token named by the entry.
    pub channel: String,
    /// Whether the channel is one of the supported side-by-side channels.
    pub channel_is_isolatable: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published isolated channel root.
    pub channel_root: String,
    /// The published isolated state-namespace root.
    pub state_namespace_root: String,
    /// The published isolated secrets-namespace root.
    pub secrets_namespace_root: String,
    /// The isolation-field tokens published by the entry.
    pub isolation_fields_covered: Vec<String>,
    /// The containment token named by the entry.
    pub containment: String,
    /// Whether the containment is disclosed (isolated / governed / disclosed-shared, never ambiguous).
    pub containment_is_disclosed: bool,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// Whether the isolation inventory publishes every required root and mandatory isolation field.
    pub channel_isolation_complete: bool,
    /// Whether the channel state is isolated: inventory complete and no stable-namespace reuse.
    pub channel_state_isolated: bool,
    /// Whether the behavior traces to the shared coexistence registry.
    pub bound_to_registry: bool,
    /// Whether a preview / beta channel actually reused the stable state namespace without a governed handoff.
    pub namespace_reuse_used: bool,
    /// Whether honest per-channel state-namespace isolation is enforced.
    pub namespace_isolation_enforced: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5ChannelIsolationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ChannelNextAction,
    /// Whether the channel resolves cleanly across every claimed profile (clean entry naming every fact).
    pub channel_resolves_across_profiles: bool,
}

impl M5ResolvedChannelIsolationEntry {
    /// Whether this channel entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_precedence_and_rollback_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PrecedenceRollbackEntryResolutionInput {
    /// Stable identity of the precedence entry.
    pub entry_id: String,
    /// The stable side-by-side-profile ID this precedence record binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The precedence domain this entry resolves.
    pub precedence_domain: M5PrecedenceReviewDomain,
    /// The render / surface context.
    pub surface_context: M5ChannelSurfaceContext,
    /// The presentation forms this entry holds across (must cover canonical / accessible / audit).
    pub presentation_form_coverage: Vec<M5ChannelPresentationForm>,
    /// The disclosed owner channel that holds precedence for this association; empty means unstated.
    pub association_owner: String,
    /// The published rollback artifact-graph root (a filesystem path); empty means unstated.
    pub rollback_artifact_graph_root: String,
    /// The precedence fields disclosed by this record (must cover every mandatory field).
    pub disclosed_fields: Vec<M5PrecedenceReviewField>,
    /// The rollback-completeness posture this record resolves.
    pub rollback_posture: M5RollbackCompletenessPosture,
    /// True when the rollback artifact-graph continuity (sidecars, symbols, manifests, update metadata) is
    /// documented.
    pub rollback_artifact_graph_continuity_documented: bool,
    /// True when the handler / association precedence ownership is explicitly disclosed (never last-writer-wins).
    pub precedence_ownership_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe precedence-rollback projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPrecedenceRollbackEntry {
    /// Stable identity of the precedence entry.
    pub entry_id: String,
    /// The stable side-by-side-profile ID this precedence record binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The precedence-domain token named by the entry.
    pub precedence_domain: String,
    /// Whether the precedence domain is classified into the resolved taxonomy.
    pub precedence_domain_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// The disclosed owner channel that holds precedence for this association.
    pub association_owner: String,
    /// The published rollback artifact-graph root.
    pub rollback_artifact_graph_root: String,
    /// The precedence-field tokens disclosed by the entry.
    pub disclosed_fields: Vec<String>,
    /// The rollback-completeness posture token named by the entry.
    pub rollback_posture: String,
    /// Whether the rollback-completeness posture is classified.
    pub rollback_posture_is_classified: bool,
    /// Whether the rollback artifact-graph continuity note is documented.
    pub rollback_artifact_graph_continuity_documented: bool,
    /// Whether the handler / association precedence ownership is explicitly disclosed.
    pub precedence_ownership_disclosed: bool,
    /// Whether the precedence rule discloses every mandatory field and is inspectable before and after.
    pub handler_precedence_inspectable: bool,
    /// Whether the rollback target is bound to the full artifact graph with documented continuity.
    pub rollback_full_artifact_graph: bool,
    /// Degrade reason, if the entry could not read as a clean, published state.
    pub degrade_reason: Option<M5PrecedenceRollbackEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ChannelNextAction,
    /// Whether the precedence record is inspectable on every claimed profile (clean entry naming every fact).
    pub precedence_and_rollback_inspectable_on_every_profile: bool,
}

impl M5ResolvedPrecedenceRollbackEntry {
    /// Whether this precedence entry reads as a clean, published state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ChannelResolutionError {
    /// The channel-entry id was empty.
    EmptyChannelEntryId,
    /// The precedence-entry id was empty.
    EmptyPrecedenceEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ChannelResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyChannelEntryId => "empty_channel_entry_id",
            Self::EmptyPrecedenceEntryId => "empty_precedence_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ChannelResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 channel-isolation / precedence-review / rollback-target registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ChannelResolutionError {}

fn presentation_form_tokens(forms: &[M5ChannelPresentationForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_presentation_forms(forms: &[M5ChannelPresentationForm]) -> bool {
    let present: BTreeSet<M5ChannelPresentationForm> = forms.iter().copied().collect();
    M5ChannelPresentationForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

fn isolation_fields_cover_mandatory(fields: &[M5ChannelIsolationField]) -> bool {
    let present: BTreeSet<M5ChannelIsolationField> = fields.iter().copied().collect();
    M5ChannelIsolationField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

fn precedence_fields_cover_mandatory(fields: &[M5PrecedenceReviewField]) -> bool {
    let present: BTreeSet<M5PrecedenceReviewField> = fields.iter().copied().collect();
    M5PrecedenceReviewField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// Whether the channel-isolation inventory publishes every required root and mandatory isolation field: the
/// channel must be isolatable, the channel / state-namespace / secrets-namespace roots must all be stated, and
/// the channel root, state namespace, secrets namespace, and services namespace must all be published.
pub fn channel_isolation_is_complete(
    channel: M5SideBySideChannel,
    channel_root: &str,
    state_namespace_root: &str,
    secrets_namespace_root: &str,
    isolation_fields_covered: &[M5ChannelIsolationField],
) -> bool {
    channel.is_isolatable()
        && !channel_root.trim().is_empty()
        && !state_namespace_root.trim().is_empty()
        && !secrets_namespace_root.trim().is_empty()
        && isolation_fields_cover_mandatory(isolation_fields_covered)
}

/// Whether the channel's durable state is isolated: the isolation inventory must be complete, a preview / beta
/// channel must not have reused the stable namespace, and honest per-channel state-namespace isolation must be
/// enforced (proving absence of reuse).
#[allow(clippy::too_many_arguments)]
pub fn channel_state_is_isolated(
    channel: M5SideBySideChannel,
    channel_root: &str,
    state_namespace_root: &str,
    secrets_namespace_root: &str,
    isolation_fields_covered: &[M5ChannelIsolationField],
    namespace_reuse_used: bool,
    namespace_isolation_enforced: bool,
) -> bool {
    channel_isolation_is_complete(
        channel,
        channel_root,
        state_namespace_root,
        secrets_namespace_root,
        isolation_fields_covered,
    ) && !namespace_reuse_used
        && namespace_isolation_enforced
}

/// Whether the published precedence rule is inspectable: the domain must be classified, every mandatory
/// precedence field must be present, and the handler / association precedence ownership must be disclosed.
pub fn handler_precedence_is_inspectable(
    domain: M5PrecedenceReviewDomain,
    disclosed_fields: &[M5PrecedenceReviewField],
    precedence_ownership_disclosed: bool,
) -> bool {
    domain.is_classified()
        && precedence_fields_cover_mandatory(disclosed_fields)
        && precedence_ownership_disclosed
}

/// Whether the rollback target is bound to the full artifact graph: the posture must be classified and the
/// artifact-graph continuity (sidecars, symbols, manifests, update metadata) must be documented.
pub fn rollback_targets_full_artifact_graph(
    posture: M5RollbackCompletenessPosture,
    rollback_artifact_graph_continuity_documented: bool,
) -> bool {
    posture.is_classified() && rollback_artifact_graph_continuity_documented
}

/// Resolves a channel-isolation entry so it stays bound to the shared coexistence registry: the entry names its
/// canonical token, semantic role, and channel, covers all three presentation forms, inventories every
/// isolation field, keeps a disclosed containment, and proves no preview / beta channel reused the stable state
/// namespace without a governed handoff.
pub fn resolve_channel_isolation_entry(
    input: M5ChannelIsolationEntryResolutionInput,
) -> Result<M5ResolvedChannelIsolationEntry, M5ChannelResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ChannelResolutionError::EmptyChannelEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.channel_root)
        || string_is_forbidden(&input.state_namespace_root)
        || string_is_forbidden(&input.secrets_namespace_root)
    {
        return Err(M5ChannelResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let isolation_complete = channel_isolation_is_complete(
        input.channel,
        &input.channel_root,
        &input.state_namespace_root,
        &input.secrets_namespace_root,
        &input.isolation_fields_covered,
    );
    let state_isolated = channel_state_is_isolated(
        input.channel,
        &input.channel_root,
        &input.state_namespace_root,
        &input.secrets_namespace_root,
        &input.isolation_fields_covered,
        input.namespace_reuse_used,
        input.namespace_isolation_enforced,
    );
    let reuse_detected = input.namespace_reuse_used || !input.namespace_isolation_enforced;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ChannelIsolationEntryDegradeReason::ChannelTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ChannelIsolationEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.channel.is_isolatable() {
        Some(M5ChannelIsolationEntryDegradeReason::ChannelUnclassified)
    } else if !input.bound_to_registry {
        Some(M5ChannelIsolationEntryDegradeReason::ChannelNotBoundToRegistry)
    } else if !isolation_complete {
        Some(M5ChannelIsolationEntryDegradeReason::ChannelNamespaceInventoryIncomplete)
    } else if reuse_detected {
        Some(M5ChannelIsolationEntryDegradeReason::PreviewCorruptedStableDurableState)
    } else if !input.containment.is_disclosed() {
        Some(M5ChannelIsolationEntryDegradeReason::ContainmentAmbiguous)
    } else if !all_forms {
        Some(M5ChannelIsolationEntryDegradeReason::PresentationFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5ChannelIsolationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ChannelNextAction::ExpandChannelMeaning,
    };

    Ok(M5ResolvedChannelIsolationEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        channel: input.channel.as_str().to_owned(),
        channel_is_isolatable: input.channel.is_isolatable(),
        surface_context: input.surface_context.as_str().to_owned(),
        channel_root: input.channel_root,
        state_namespace_root: input.state_namespace_root,
        secrets_namespace_root: input.secrets_namespace_root,
        isolation_fields_covered: input
            .isolation_fields_covered
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        containment: input.containment.as_str().to_owned(),
        containment_is_disclosed: input.containment.is_disclosed(),
        presentation_form_coverage: presentation_form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        channel_isolation_complete: isolation_complete,
        channel_state_isolated: state_isolated,
        bound_to_registry: input.bound_to_registry,
        namespace_reuse_used: input.namespace_reuse_used,
        namespace_isolation_enforced: input.namespace_isolation_enforced,
        degrade_reason,
        next_action,
        channel_resolves_across_profiles: degrade_reason.is_none(),
    })
}

/// Resolves a precedence-rollback entry so its precedence stays inspectable and its rollback target stays bound
/// to the full artifact graph: the entry names its canonical token, semantic role, and precedence domain, covers
/// all three presentation forms, discloses every mandatory precedence field and the precedence ownership, and
/// keeps its rollback artifact-graph continuity documented.
pub fn resolve_precedence_and_rollback_entry(
    input: M5PrecedenceRollbackEntryResolutionInput,
) -> Result<M5ResolvedPrecedenceRollbackEntry, M5ChannelResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ChannelResolutionError::EmptyPrecedenceEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.association_owner)
        || string_is_forbidden(&input.rollback_artifact_graph_root)
    {
        return Err(M5ChannelResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let is_inspectable = handler_precedence_is_inspectable(
        input.precedence_domain,
        &input.disclosed_fields,
        input.precedence_ownership_disclosed,
    ) && !input.association_owner.trim().is_empty()
        && !input.rollback_artifact_graph_root.trim().is_empty();
    let full_graph = rollback_targets_full_artifact_graph(
        input.rollback_posture,
        input.rollback_artifact_graph_continuity_documented,
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PrecedenceRollbackEntryDegradeReason::PrecedenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PrecedenceRollbackEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.precedence_domain.is_classified() {
        Some(M5PrecedenceRollbackEntryDegradeReason::PrecedenceDomainUnclassified)
    } else if !is_inspectable {
        Some(M5PrecedenceRollbackEntryDegradeReason::HandlerPrecedenceNotInspectable)
    } else if !full_graph {
        Some(M5PrecedenceRollbackEntryDegradeReason::RollbackArtifactGraphIncomplete)
    } else if !all_forms {
        Some(M5PrecedenceRollbackEntryDegradeReason::PrecedenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PrecedenceRollbackEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ChannelNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedPrecedenceRollbackEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        precedence_domain: input.precedence_domain.as_str().to_owned(),
        precedence_domain_is_classified: input.precedence_domain.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        presentation_form_coverage: presentation_form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        association_owner: input.association_owner,
        rollback_artifact_graph_root: input.rollback_artifact_graph_root,
        disclosed_fields: input
            .disclosed_fields
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        rollback_posture: input.rollback_posture.as_str().to_owned(),
        rollback_posture_is_classified: input.rollback_posture.is_classified(),
        rollback_artifact_graph_continuity_documented: input
            .rollback_artifact_graph_continuity_documented,
        precedence_ownership_disclosed: input.precedence_ownership_disclosed,
        handler_precedence_inspectable: is_inspectable,
        rollback_full_artifact_graph: full_graph,
        degrade_reason,
        next_action,
        precedence_and_rollback_inspectable_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved channel-isolation and precedence-rollback
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ChannelConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5InstallTopologyQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5InstallTopologyDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5InstallTopologyRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5InstallTopologyAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ChannelAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ChannelExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    /// Resolved channel-isolation examples.
    pub channel_isolation_entries: Vec<M5ResolvedChannelIsolationEntry>,
    /// Resolved precedence-rollback examples.
    pub precedence_rollback_entries: Vec<M5ResolvedPrecedenceRollbackEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the install-topology domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a preview / beta channel never reuses the stable state namespace. MUST be `false`.
    pub preview_or_beta_reused_stable_state_namespace: bool,
    /// Hard invariant: handler ownership never becomes a last-writer-wins accident. MUST be `false`.
    pub handler_ownership_became_last_writer_wins: bool,
    /// Hard invariant: a rollback never targets only the primary executable while sidecars drift. MUST be
    /// `false`.
    pub rollback_targeted_primary_executable_only: bool,
    /// Hard invariant: channel precedence / rollback never drifts from the published matrix. MUST be `false`.
    pub channel_precedence_or_rollback_drifted_from_matrix: bool,
}

impl M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ChannelAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5ChannelAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ChannelExportField> = self.export_fields.iter().copied().collect();
        M5ChannelExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.preview_or_beta_reused_stable_state_namespace
            && !self.handler_ownership_became_last_writer_wins
            && !self.rollback_targeted_primary_executable_only
            && !self.channel_precedence_or_rollback_drifted_from_matrix
    }

    /// True when a clean channel entry preserves registry-bound truth: it traces to the registry, keeps a
    /// supported channel, inventories every isolation field, stays isolated (no stable-namespace reuse), keeps a
    /// disclosed containment, and covers all three presentation forms.
    fn channel_is_honest(ex: &M5ResolvedChannelIsolationEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.channel_is_isolatable
                && ex.channel_isolation_complete
                && ex.channel_state_isolated
                && ex.containment_is_disclosed
                && ex.covers_all_presentation_forms)
    }

    /// True when a clean precedence entry preserves published truth: it keeps a classified domain, stays
    /// inspectable, binds the full rollback artifact graph, and covers all three presentation forms.
    fn precedence_is_honest(ex: &M5ResolvedPrecedenceRollbackEntry) -> bool {
        !ex.is_clean()
            || (ex.precedence_domain_is_classified
                && ex.handler_precedence_inspectable
                && ex.rollback_full_artifact_graph
                && ex.covers_all_presentation_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.channel_isolation_entries
            .iter()
            .all(Self::channel_is_honest)
            && self
                .precedence_rollback_entries
                .iter()
                .all(Self::precedence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Presentation-form tokens (minted by this lane).
    pub presentation_forms: Vec<String>,
    /// Channel tokens (minted by this lane).
    pub channels: Vec<String>,
    /// Isolation-field tokens (minted by this lane).
    pub isolation_fields: Vec<String>,
    /// Containment tokens (minted by this lane).
    pub containments: Vec<String>,
    /// Precedence-domain tokens (minted by this lane).
    pub precedence_domains: Vec<String>,
    /// Precedence-field tokens (minted by this lane).
    pub precedence_fields: Vec<String>,
    /// Rollback-completeness posture tokens (minted by this lane).
    pub rollback_postures: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Channel-entry degrade-reason tokens.
    pub channel_degrade_reasons: Vec<String>,
    /// Precedence-entry degrade-reason tokens.
    pub precedence_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ChannelIsolationPrecedenceReviewAndRollbackTargetsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5InstallTopologyRole::ALL, |v| v.as_str()),
            presentation_forms: tokens(&M5ChannelPresentationForm::ALL, |v| v.as_str()),
            channels: tokens(&M5SideBySideChannel::ALL, |v| v.as_str()),
            isolation_fields: tokens(&M5ChannelIsolationField::ALL, |v| v.as_str()),
            containments: tokens(&M5ChannelStateContainment::ALL, |v| v.as_str()),
            precedence_domains: tokens(&M5PrecedenceReviewDomain::ALL, |v| v.as_str()),
            precedence_fields: tokens(&M5PrecedenceReviewField::ALL, |v| v.as_str()),
            rollback_postures: tokens(&M5RollbackCompletenessPosture::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ChannelSurfaceContext::ALL, |v| v.as_str()),
            channel_degrade_reasons: tokens(&M5ChannelIsolationEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            precedence_degrade_reasons: tokens(&M5PrecedenceRollbackEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ChannelAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ChannelNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ChannelExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5InstallTopologyConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsGovernanceReview {
    /// The coexistence registry names a canonical token, semantic role, and channel for every entry.
    pub registry_names_token_role_and_channel: bool,
    /// Every claimed side-by-side profile isolates the stable, preview, beta, and LTS channels.
    pub profile_isolates_all_canonical_channels: bool,
    /// Every isolation field (channel root, state namespace, secrets namespace, services namespace) is
    /// published.
    pub all_isolation_fields_published: bool,
    /// A preview / beta channel is never allowed to reuse the stable state namespace without a governed handoff.
    pub preview_never_reuses_stable_state_namespace: bool,
    /// Isolated-versus-governed-handoff containment stays explicit and distinguishable.
    pub containment_explicit_and_distinguishable: bool,
    /// Precedence rules are published across the file-association, protocol-handler, deep-link, and default-open
    /// domains.
    pub precedence_published_across_domains: bool,
    /// Every channel and precedence entry covers the canonical / accessible / audit presentation forms.
    pub every_entry_covers_all_presentation_forms: bool,
    /// Rollback targets bind the full artifact graph rather than the primary executable only.
    pub rollback_binds_full_artifact_graph: bool,
    /// Coexistence behavior stays bound to the shared registries rather than hand-copied per profile.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Installer, update, diagnostics, admin, docs, and support read a single coexistence source.
    pub installer_update_diagnostics_admin_read_single_source: bool,
    /// A namespace reuse, a last-writer-wins handler, or a narrowed rollback is caught by fixtures before
    /// release evidence turns green.
    pub coexistence_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsConsumerProjection {
    /// The installer and update flows consume the shared coexistence registry.
    pub installer_and_update_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared coexistence registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// The updater service and precedence-review surface consume the shared registries.
    pub updater_and_precedence_review_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical install-topology and state-root-boundary contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical coexistence registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting channel-isolation audit for the lane.
    pub channel_isolation_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 channel-isolation / precedence-review / rollback-target registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket {
    /// Record kind; must equal [`M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacketInput) -> Self {
        Self {
            record_kind: M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_RECORD_KIND
        {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_VERSION
        {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect(
                "m5 channel-isolation / precedence-review / rollback-target packet serializes",
            ),
        ) {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RawMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 channel-isolation / precedence-review / rollback-target packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,channel_isolation_entries,precedence_rollback_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .channel_isolation_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.precedence_rollback_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.channel_isolation_entries.len(),
                row.precedence_rollback_entries.len(),
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
            "# M5 Channel-Isolation, Precedence-Review, and Rollback-Target Registries\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Channels: {}\n",
            self.vocabulary_set.channels.join(", ")
        ));
        out.push_str(&format!(
            "- Presentation forms: {}\n",
            self.vocabulary_set.presentation_forms.join(", ")
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
                "  - Channel entries: {} / precedence entries: {}\n",
                row.channel_isolation_entries.len(),
                row.precedence_rollback_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-profile channel-isolation reference table generated from the registry, so docs and
    /// support runbooks render the same channel / channel-root / state-namespace-root / secrets-namespace-root /
    /// containment truth the resolvers produced rather than a hand-copied path table. Only clean,
    /// registry-bound channel entries are listed.
    pub fn render_channel_isolation_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| profile_id | channel | channel_root | state_namespace_root | secrets_namespace_root | containment |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.channel_isolation_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | {} |\n",
                    ex.profile_id,
                    ex.channel,
                    ex.channel_root,
                    ex.state_namespace_root,
                    ex.secrets_namespace_root,
                    ex.containment
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5ChannelIsolationPrecedenceReviewAndRollbackTargetsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>),
}

impl fmt::Display for M5ChannelIsolationPrecedenceReviewAndRollbackTargetsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 channel-isolation / precedence-review / rollback-target export parse failed: {error}"
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
                    "m5 channel-isolation / precedence-review / rollback-target export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChannelIsolationPrecedenceReviewAndRollbackTargetsArtifactError {}

/// Validation failures emitted by
/// [`M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation {
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
    /// A registry row does not point at the install-topology domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, namespace-reusing, inventory-incomplete,
    /// containment-ambiguous, or a precedence entry missing a disclosure).
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
    /// Channel-isolation-contract is not proven: clean channel entries do not cover the canonical channels or
    /// the first installer / update / diagnostics / admin / support surfaces, no inventory-incomplete example
    /// degrades, or a clean channel entry published an incomplete isolation inventory.
    ChannelIsolationContractNotProven,
    /// Handler-precedence-inspectability is not proven: no containment-ambiguous example degrades, no clean
    /// contained channel entry is present, a clean channel entry is ambiguous, or clean precedence entries do
    /// not cover the canonical precedence domains with full presentation-form coverage while inspectable.
    HandlerPrecedenceInspectabilityNotProven,
    /// Rollback-artifact-graph-completeness is not proven: no namespace-reuse example degrades, a clean channel
    /// entry reused the stable namespace, no precedence-not-inspectable example degrades, or no
    /// rollback-artifact-graph-incomplete example degrades.
    RollbackArtifactGraphCompletenessNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation {
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
            Self::ChannelIsolationContractNotProven => "channel_isolation_contract_not_proven",
            Self::HandlerPrecedenceInspectabilityNotProven => {
                "handler_precedence_inspectability_not_proven"
            }
            Self::RollbackArtifactGraphCompletenessNotProven => {
                "rollback_artifact_graph_completeness_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_channel_isolation_precedence_review_and_rollback_targets_export() -> Result<
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsArtifactError,
> {
    let packet: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-channel-isolation-precedence-review-and-rollback-targets-proof/support_export.json"
        )))
        .map_err(M5ChannelIsolationPrecedenceReviewAndRollbackTargetsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5ChannelIsolationPrecedenceReviewAndRollbackTargetsArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_REF,
        M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::NoRegistryRows);
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
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF) {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::DomainSchemaRefMissing,
            );
        }
        if row.channel_isolation_entries.is_empty() || row.precedence_rollback_entries.is_empty() {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.registry_names_token_role_and_channel,
        review.profile_isolates_all_canonical_channels,
        review.all_isolation_fields_published,
        review.preview_never_reuses_stable_state_namespace,
        review.containment_explicit_and_distinguishable,
        review.precedence_published_across_domains,
        review.every_entry_covers_all_presentation_forms,
        review.rollback_binds_full_artifact_graph,
        review.behavior_bound_to_registry_not_hand_copied,
        review.installer_update_diagnostics_admin_read_single_source,
        review.coexistence_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.installer_and_update_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.updater_and_precedence_review_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.channel_isolation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket,
    violations: &mut Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation>,
) {
    let channels = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.channel_isolation_entries.iter())
    };
    let precedences = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.precedence_rollback_entries.iter())
    };

    // AC1: preview and stable installs can coexist without corrupting one another's durable state. Clean channel
    // entries cover the canonical channels and the first installer / update / diagnostics / admin / support
    // surfaces, an inventory-incomplete example degrades, and no clean channel entry published an incomplete
    // isolation inventory.
    let clean_channels: BTreeSet<String> = channels()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.channel.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = channels()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let channels_covered = M5SideBySideChannel::CANONICAL_CHANNELS
        .iter()
        .all(|c| clean_channels.contains(c.as_str()));
    let first_surfaces_covered = M5ChannelSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let inventory_incomplete_degrades = channels().any(|ex| {
        ex.degrade_reason
            == Some(M5ChannelIsolationEntryDegradeReason::ChannelNamespaceInventoryIncomplete)
    });
    let no_clean_incomplete = !channels().any(|ex| ex.is_clean() && !ex.channel_isolation_complete);
    if !(channels_covered
        && first_surfaces_covered
        && inventory_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ChannelIsolationContractNotProven,
        );
    }

    // AC2: handler ownership and channel precedence are inspectable before and after update / import flows. A
    // containment-ambiguous example degrades, at least one clean contained channel entry is present, no clean
    // channel entry is ambiguous, and clean precedence entries cover the canonical precedence domains with full
    // presentation-form coverage while inspectable.
    let containment_ambiguous_degrades = channels().any(|ex| {
        ex.degrade_reason == Some(M5ChannelIsolationEntryDegradeReason::ContainmentAmbiguous)
    });
    let disclosed_clean_channel = channels().any(|ex| ex.is_clean() && ex.containment_is_disclosed);
    let no_clean_ambiguous = !channels().any(|ex| ex.is_clean() && !ex.containment_is_disclosed);
    let clean_precedence_domains: BTreeSet<String> = precedences()
        .filter(|ex| {
            ex.is_clean()
                && ex.precedence_domain_is_classified
                && ex.handler_precedence_inspectable
                && ex.covers_all_presentation_forms
        })
        .map(|ex| ex.precedence_domain.clone())
        .collect();
    let precedence_domains_covered = M5PrecedenceReviewDomain::CANONICAL_DOMAINS
        .iter()
        .all(|d| clean_precedence_domains.contains(d.as_str()));
    if !(containment_ambiguous_degrades
        && disclosed_clean_channel
        && no_clean_ambiguous
        && precedence_domains_covered)
    {
        violations.push(
            M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::HandlerPrecedenceInspectabilityNotProven,
        );
    }

    // AC3: rollback validation fails when a target lacks compatible sidecars, metadata, or association state,
    // and a preview channel that reuses stable durable state is caught. A namespace-reuse example degrades, no
    // clean channel entry reused the stable namespace, a precedence-not-inspectable example degrades, and a
    // rollback-artifact-graph-incomplete example degrades.
    let reuse_degrades = channels().any(|ex| {
        ex.degrade_reason
            == Some(M5ChannelIsolationEntryDegradeReason::PreviewCorruptedStableDurableState)
    });
    let no_clean_reuse = !channels()
        .any(|ex| ex.is_clean() && (ex.namespace_reuse_used || !ex.namespace_isolation_enforced));
    let precedence_not_inspectable_degrades = precedences().any(|ex| {
        ex.degrade_reason
            == Some(M5PrecedenceRollbackEntryDegradeReason::HandlerPrecedenceNotInspectable)
    });
    let rollback_incomplete_degrades = precedences().any(|ex| {
        ex.degrade_reason
            == Some(M5PrecedenceRollbackEntryDegradeReason::RollbackArtifactGraphIncomplete)
    });
    if !(reuse_degrades
        && no_clean_reuse
        && precedence_not_inspectable_degrades
        && rollback_incomplete_degrades)
    {
        violations.push(
            M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RollbackArtifactGraphCompletenessNotProven,
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

/// The install-topology families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5InstallTopologyFamily; 1] =
    [M5InstallTopologyFamily::SideBySideStablePreview];
