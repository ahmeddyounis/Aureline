//! Package explorer rows with direct/transitive state, manifest scope,
//! registry source, license/advisory/changelog signals, and action truth.
//!
//! This module narrows the `package_explorer_row` component frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`] into an
//! implemented, export-safe package browse row. Every [`PackageExplorerRow`]
//! answers, from the component alone, what package the user is acting on, which
//! ecosystem it belongs to, its current and candidate version, whether it is a
//! direct or transitive dependency, which manifest scope owns it, which registry
//! answered for it, and what the license, advisory, and changelog signals say.
//! The row's action is never a generic button: the primary action truth is
//! derived from the lifecycle state and the direct/transitive relation, so an
//! installed, available, outdated, imported, policy-pinned, or remove-blocked
//! package can never present the same undifferentiated "manage" affordance, and
//! a transitive or blocked package always names why it cannot be mutated
//! directly.
//!
//! The registry/resolution degradation vocabulary
//! ([`M5PackageComponentDegradationState`]) and rollback posture
//! ([`M5PackageComponentRollbackPosture`]) are reused directly from the frozen
//! matrix so mirror-backed, offline-snapshot, auth-required, or stale resolution
//! reads the same everywhere and never flattens into a clean "installed" or "up
//! to date" message. Downgrade triggers
//! ([`M5PackageComponentDowngradeTrigger`]) and consumer surfaces
//! ([`M5PackageComponentConsumerSurface`]) are reused from the matrix as well.
//!
//! The packet references the upstream package-management component matrix and the
//! canonical dependency-row contract by id rather than embedding their content.
//! Raw manifest bodies, raw lockfile bodies, registry credentials, private
//! registry URLs, and live registry responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-package-explorer-row.schema.json`](../../../../schemas/ui/m5-package-explorer-row.schema.json).
//! The contract doc is
//! [`docs/deps/m5/implement_package_explorer_rows_with_scope_relation_registry_and_signal_truth.md`](../../../../docs/deps/m5/implement_package_explorer_rows_with_scope_relation_registry_and_signal_truth.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-package-explorer-row/`](../../../../fixtures/ui/m5-package-explorer-row/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::{
    M5PackageComponent, M5PackageComponentConsumerSurface, M5PackageComponentDegradationState,
    M5PackageComponentDowngradeTrigger, M5PackageComponentRollbackPosture,
    M5_PACKAGE_COMPONENT_MATRIX_DOC_REF, M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`PackageExplorerRowPacket`].
pub const PACKAGE_EXPLORER_ROW_RECORD_KIND: &str = "package_explorer_row_controls";

/// Schema version for package-explorer-row control records.
pub const PACKAGE_EXPLORER_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PACKAGE_EXPLORER_ROW_SCHEMA_REF: &str = "schemas/ui/m5-package-explorer-row.schema.json";

/// Repo-relative path of the contract doc.
pub const PACKAGE_EXPLORER_ROW_DOC_REF: &str =
    "docs/deps/m5/implement_package_explorer_rows_with_scope_relation_registry_and_signal_truth.md";

/// Repo-relative path of the protected fixture directory.
pub const PACKAGE_EXPLORER_ROW_FIXTURE_DIR: &str = "fixtures/ui/m5-package-explorer-row";

/// Repo-relative path of the checked support-export artifact.
pub const PACKAGE_EXPLORER_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-package-explorer-row-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PACKAGE_EXPLORER_ROW_SUMMARY_REF: &str =
    "artifacts/release/m5-package-explorer-row-proof/summary.md";

/// Lifecycle state a package explorer row reports.
///
/// These states must stay visually distinct and copy/export safe: an installed
/// package is not the same as an available one, an outdated package names a
/// candidate upgrade, an imported package is owned elsewhere, a policy-pinned
/// package is held by governance, and a remove-blocked package cannot be removed
/// because something depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageLifecycleState {
    /// Installed and resolved in this project.
    Installed,
    /// Available to install but not currently present.
    Available,
    /// Installed but a newer candidate version exists.
    Outdated,
    /// Present via an imported/vendored snapshot owned elsewhere.
    Imported,
    /// Pinned by policy or governance; the version is held.
    PolicyPinned,
    /// Installed but cannot be removed because dependents rely on it.
    RemoveBlocked,
}

impl PackageLifecycleState {
    /// Every lifecycle state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Installed,
        Self::Available,
        Self::Outdated,
        Self::Imported,
        Self::PolicyPinned,
        Self::RemoveBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Available => "available",
            Self::Outdated => "outdated",
            Self::Imported => "imported",
            Self::PolicyPinned => "policy_pinned",
            Self::RemoveBlocked => "remove_blocked",
        }
    }
}

/// Direct / transitive relation of a package to the project's manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageDependencyRelation {
    /// Declared directly in a manifest the project owns.
    Direct,
    /// Pulled in transitively by another dependency; not directly declared.
    Transitive,
    /// Declared directly and also pulled in transitively.
    DirectAndTransitive,
}

impl PackageDependencyRelation {
    /// Every relation, in declaration order.
    pub const ALL: [Self; 3] = [Self::Direct, Self::Transitive, Self::DirectAndTransitive];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::DirectAndTransitive => "direct_and_transitive",
        }
    }

    /// Whether the row must carry an explicit relation note (transitive parent).
    pub const fn needs_relation_note(self) -> bool {
        !matches!(self, Self::Direct)
    }

    /// Whether the relation is purely transitive (no direct declaration).
    pub const fn is_pure_transitive(self) -> bool {
        matches!(self, Self::Transitive)
    }
}

/// Manifest scope that owns the package declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageManifestScopeClass {
    /// Runtime dependency section.
    RuntimeDependency,
    /// Development-only dependency section.
    DevDependency,
    /// Optional dependency section.
    OptionalDependency,
    /// Peer dependency section.
    PeerDependency,
    /// Build / toolchain dependency section.
    BuildDependency,
    /// Workspace catalog or root-level shared scope.
    WorkspaceCatalog,
}

impl PackageManifestScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeDependency => "runtime_dependency",
            Self::DevDependency => "dev_dependency",
            Self::OptionalDependency => "optional_dependency",
            Self::PeerDependency => "peer_dependency",
            Self::BuildDependency => "build_dependency",
            Self::WorkspaceCatalog => "workspace_catalog",
        }
    }
}

/// Registry source that answered for the package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageRegistrySourceClass {
    /// A public upstream registry.
    PublicRegistry,
    /// A private / authenticated registry.
    PrivateRegistry,
    /// An enterprise mirror standing in for the upstream registry.
    EnterpriseMirror,
    /// An offline snapshot or local cache.
    OfflineSnapshot,
    /// A git source dependency.
    GitSource,
    /// A path or vendored source dependency.
    PathOrVendored,
}

impl PackageRegistrySourceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::PrivateRegistry => "private_registry",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::GitSource => "git_source",
            Self::PathOrVendored => "path_or_vendored",
        }
    }
}

/// License signal shown on a package explorer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageLicenseSignal {
    /// License is allowed by policy.
    Allowed,
    /// License requires review before use.
    ReviewRequired,
    /// License is disallowed by policy.
    Disallowed,
    /// License could not be determined.
    Unknown,
}

impl PackageLicenseSignal {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::ReviewRequired => "review_required",
            Self::Disallowed => "disallowed",
            Self::Unknown => "unknown",
        }
    }
}

/// Security-advisory signal shown on a package explorer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageAdvisorySignal {
    /// No known advisory for this version.
    NoKnownAdvisory,
    /// A low-severity advisory applies.
    AdvisoryLow,
    /// A high-severity advisory applies.
    AdvisoryHigh,
    /// A critical advisory applies.
    AdvisoryCritical,
    /// Advisory state could not be determined.
    AdvisoryUnknown,
}

impl PackageAdvisorySignal {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoKnownAdvisory => "no_known_advisory",
            Self::AdvisoryLow => "advisory_low",
            Self::AdvisoryHigh => "advisory_high",
            Self::AdvisoryCritical => "advisory_critical",
            Self::AdvisoryUnknown => "advisory_unknown",
        }
    }
}

/// Changelog signal shown on a package explorer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageChangelogSignal {
    /// A changelog is available for the candidate change.
    Available,
    /// Only a partial changelog is available.
    Partial,
    /// No changelog is available.
    Unavailable,
    /// The change notes a breaking change.
    BreakingChangeNoted,
}

impl PackageChangelogSignal {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
            Self::BreakingChangeNoted => "breaking_change_noted",
        }
    }
}

/// Derived action-truth class for a package explorer row.
///
/// This is the core honesty axis: the action a row may present is derived from
/// the lifecycle state and the direct/transitive relation, never asserted
/// directly, so a transitive, imported, policy-pinned, or remove-blocked package
/// can never present a plain install/update/remove button as though it were a
/// direct, mutable dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageRowActionClass {
    /// Available and directly installable.
    InstallAvailable,
    /// Outdated and directly updatable.
    UpdateAvailable,
    /// Installed and directly manageable (update or remove).
    ManageInstalled,
    /// Transitive-only; not directly mutable, name the parent instead.
    TransitiveReadOnly,
    /// Imported/vendored; canonical truth lives elsewhere.
    ImportedReadOnly,
    /// Pinned by policy; the version is held.
    PolicyPinnedBlocked,
    /// Installed but removal is blocked by dependents.
    RemoveBlocked,
}

impl PackageRowActionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallAvailable => "install_available",
            Self::UpdateAvailable => "update_available",
            Self::ManageInstalled => "manage_installed",
            Self::TransitiveReadOnly => "transitive_read_only",
            Self::ImportedReadOnly => "imported_read_only",
            Self::PolicyPinnedBlocked => "policy_pinned_blocked",
            Self::RemoveBlocked => "remove_blocked",
        }
    }

    /// Whether this action class offers a direct install/update/remove mutation.
    pub const fn is_directly_actionable(self) -> bool {
        matches!(
            self,
            Self::InstallAvailable | Self::UpdateAvailable | Self::ManageInstalled
        )
    }
}

/// Disclosures a package explorer row must carry, derived from lifecycle and relation.
///
/// The resolver output anchors the honesty invariants: a directly-actionable row
/// derives its action from an installed/available/outdated + direct/both state; a
/// transitive or blocked row is never directly actionable and always names why;
/// and a candidate version is required wherever an install or update is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageRowActionDisclosure {
    /// The derived action class this row may present.
    pub action_class: PackageRowActionClass,
    /// Whether the row may present a direct install/update/remove action.
    pub is_directly_actionable: bool,
    /// Whether the row must name a candidate version (install/update offered).
    pub needs_candidate_version: bool,
    /// Whether the row must carry an explicit blocked/read-only reason.
    pub needs_blocked_reason: bool,
    /// Whether the row must carry an explicit direct/transitive relation note.
    pub needs_relation_note: bool,
}

/// Resolves the action truth a package explorer row may present.
///
/// Read-only lifecycle states (imported, policy-pinned, remove-blocked) dominate,
/// and a purely transitive relation is never directly actionable even when the
/// package is installed, available, or outdated: it must name the parent that
/// pulls it rather than offering a direct mutation. Only a direct (or
/// direct-and-transitive) installed/available/outdated package resolves to a
/// directly-actionable class.
pub fn resolve_package_explorer_row_action(
    lifecycle: PackageLifecycleState,
    relation: PackageDependencyRelation,
) -> PackageRowActionDisclosure {
    let action_class = match lifecycle {
        PackageLifecycleState::Imported => PackageRowActionClass::ImportedReadOnly,
        PackageLifecycleState::PolicyPinned => PackageRowActionClass::PolicyPinnedBlocked,
        PackageLifecycleState::RemoveBlocked => PackageRowActionClass::RemoveBlocked,
        PackageLifecycleState::Installed
        | PackageLifecycleState::Available
        | PackageLifecycleState::Outdated => {
            if relation.is_pure_transitive() {
                PackageRowActionClass::TransitiveReadOnly
            } else {
                match lifecycle {
                    PackageLifecycleState::Available => PackageRowActionClass::InstallAvailable,
                    PackageLifecycleState::Outdated => PackageRowActionClass::UpdateAvailable,
                    _ => PackageRowActionClass::ManageInstalled,
                }
            }
        }
    };

    let is_directly_actionable = action_class.is_directly_actionable();
    PackageRowActionDisclosure {
        action_class,
        is_directly_actionable,
        needs_candidate_version: matches!(
            action_class,
            PackageRowActionClass::InstallAvailable | PackageRowActionClass::UpdateAvailable
        ),
        needs_blocked_reason: !is_directly_actionable,
        needs_relation_note: relation.needs_relation_note(),
    }
}

/// A package explorer row naming scope, provenance, signals, and action truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExplorerRow {
    /// Frozen component this row implements; must be `package_explorer_row`.
    pub component: M5PackageComponent,
    /// Stable row id.
    pub row_id: String,
    /// Human-readable package label.
    pub package_label: String,
    /// Ecosystem token (npm, pypi, cargo, ...).
    pub ecosystem: String,
    /// Current resolved version; empty is allowed only when not installed.
    pub current_version: String,
    /// Candidate version; required when an install or update is offered.
    pub candidate_version: String,
    /// Direct / transitive relation.
    pub relation: PackageDependencyRelation,
    /// Lifecycle state.
    pub lifecycle: PackageLifecycleState,
    /// Manifest scope class.
    pub manifest_scope: PackageManifestScopeClass,
    /// Which manifest / scope owns the declaration; required and non-empty.
    pub manifest_scope_disclosure: String,
    /// Direct/transitive relation note; required when the relation is not direct.
    pub relation_note: String,
    /// Registry source class.
    pub registry_source: PackageRegistrySourceClass,
    /// Registry-source provenance; required and non-empty.
    pub registry_source_disclosure: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// License signal.
    pub license_signal: PackageLicenseSignal,
    /// Advisory signal.
    pub advisory_signal: PackageAdvisorySignal,
    /// Changelog signal.
    pub changelog_signal: PackageChangelogSignal,
    /// License/advisory/changelog signal summary; required and non-empty.
    pub signal_disclosure: String,
    /// Whether this row presents a direct install/update/remove action.
    pub offers_direct_action: bool,
    /// Primary action label; required and non-empty.
    pub primary_action_label: String,
    /// Action provenance note; required when a direct action is offered.
    pub action_provenance_note: String,
    /// Blocked / read-only reason; required when no direct action is offered.
    pub blocked_reason: String,
    /// Rollback / write-back posture, reused from the frozen matrix.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Row fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl PackageExplorerRow {
    /// Action-truth disclosures this row must carry, derived from lifecycle and relation.
    pub fn action_disclosure(&self) -> PackageRowActionDisclosure {
        resolve_package_explorer_row_action(self.lifecycle, self.relation)
    }

    /// Whether the rollback posture is consistent with the action truth.
    ///
    /// A directly-actionable row must not be read-only; a read-only or blocked
    /// row must carry a read-only posture that never mutates the manifest.
    pub fn rollback_posture_consistent(&self) -> bool {
        let actionable = self.action_disclosure().is_directly_actionable;
        let read_only = matches!(
            self.rollback_posture,
            M5PackageComponentRollbackPosture::ReadOnlyNoMutation
        );
        actionable != read_only
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExplorerRowTrustReview {
    /// The acted-on package identity is always explicit.
    pub package_identity_always_explicit: bool,
    /// The owning manifest scope is always explicit.
    pub manifest_scope_always_explicit: bool,
    /// Direct/transitive relation is always explicit.
    pub direct_transitive_relation_explicit: bool,
    /// Registry source is always explicit.
    pub registry_source_always_explicit: bool,
    /// License, advisory, and changelog signals are always explicit.
    pub license_advisory_changelog_signals_explicit: bool,
    /// Lifecycle states stay visually distinct.
    pub lifecycle_state_visually_distinct: bool,
    /// The action truth matches the lifecycle and relation.
    pub action_truth_matches_state: bool,
    /// No generic action is offered without provenance.
    pub no_generic_action_without_provenance: bool,
    /// Transitive and blocked states always name their reason.
    pub transitive_and_blocked_states_name_reason: bool,
    /// Mirror / offline degradation stays explicit rather than reading as clean.
    pub mirror_offline_degradation_explicit: bool,
    /// The row stays copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl PackageExplorerRowTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.package_identity_always_explicit
            && self.manifest_scope_always_explicit
            && self.direct_transitive_relation_explicit
            && self.registry_source_always_explicit
            && self.license_advisory_changelog_signals_explicit
            && self.lifecycle_state_visually_distinct
            && self.action_truth_matches_state
            && self.no_generic_action_without_provenance
            && self.transitive_and_blocked_states_name_reason
            && self.mirror_offline_degradation_explicit
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExplorerRowConsumerProjection {
    /// The row shows the package label, manifest scope, and relation.
    pub row_shows_label_scope_and_relation: bool,
    /// The lifecycle state is shown distinctly.
    pub lifecycle_state_shown_distinctly: bool,
    /// The registry source and signals are shown.
    pub registry_source_and_signals_shown: bool,
    /// The action reflects the state and carries provenance.
    pub action_reflects_state_and_provenance: bool,
    /// The blocked / read-only reason is shown inline.
    pub blocked_reason_shown_inline: bool,
    /// CLI / headless shows row truth.
    pub cli_headless_shows_row_truth: bool,
    /// Support export shows row truth.
    pub support_export_shows_row_truth: bool,
    /// Help / About shows row truth.
    pub help_about_shows_row_truth: bool,
}

impl PackageExplorerRowConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.row_shows_label_scope_and_relation
            && self.lifecycle_state_shown_distinctly
            && self.registry_source_and_signals_shown
            && self.action_reflects_state_and_provenance
            && self.blocked_reason_shown_inline
            && self.cli_headless_shows_row_truth
            && self.support_export_shows_row_truth
            && self.help_about_shows_row_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExplorerRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PackageExplorerRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageExplorerRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Package explorer rows.
    pub rows: Vec<PackageExplorerRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse this row.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: PackageExplorerRowTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PackageExplorerRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageExplorerRowProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe package explorer row packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExplorerRowPacket {
    /// Record kind; must equal [`PACKAGE_EXPLORER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PACKAGE_EXPLORER_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Package explorer rows.
    pub rows: Vec<PackageExplorerRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse this row.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: PackageExplorerRowTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PackageExplorerRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageExplorerRowProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PackageExplorerRowPacket {
    /// Builds a package explorer row packet from stable-lane input.
    pub fn new(input: PackageExplorerRowPacketInput) -> Self {
        Self {
            record_kind: PACKAGE_EXPLORER_ROW_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_EXPLORER_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            rows: input.rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the package explorer row invariants.
    pub fn validate(&self) -> Vec<PackageExplorerRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PACKAGE_EXPLORER_ROW_RECORD_KIND {
            violations.push(PackageExplorerRowViolation::WrongRecordKind);
        }
        if self.schema_version != PACKAGE_EXPLORER_ROW_SCHEMA_VERSION {
            violations.push(PackageExplorerRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PackageExplorerRowViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PackageExplorerRowViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(PackageExplorerRowViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PackageExplorerRowViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PackageExplorerRowViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PackageExplorerRowViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("package explorer row packet serializes"),
        ) {
            violations.push(PackageExplorerRowViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("package explorer row packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let actionable = self
            .rows
            .iter()
            .filter(|row| row.offers_direct_action)
            .count();
        let transitive = self
            .rows
            .iter()
            .filter(|row| row.relation.is_pure_transitive())
            .count();

        let mut out = String::new();
        out.push_str("# Package Explorer Rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Rows: {} ({} directly actionable, {} transitive)\n",
            self.rows.len(),
            actionable,
            transitive
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) `{}`: {} in {} — action `{}` [{}]\n",
                row.package_label,
                row.ecosystem,
                row.lifecycle.as_str(),
                row.relation.as_str(),
                row.manifest_scope.as_str(),
                row.action_disclosure().action_class.as_str(),
                row.registry_source.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in package-explorer-row export.
#[derive(Debug)]
pub enum PackageExplorerRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PackageExplorerRowViolation>),
}

impl fmt::Display for PackageExplorerRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "package explorer row export parse failed: {error}"
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
                    "package explorer row export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PackageExplorerRowArtifactError {}

/// Validation failures emitted by [`PackageExplorerRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageExplorerRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No package explorer rows are present.
    RowsMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row carries the wrong frozen component class.
    RowWrongComponentClass,
    /// A row does not name its owning manifest scope.
    ManifestScopeDisclosureMissing,
    /// A row does not name its registry-source provenance.
    RegistrySourceDisclosureMissing,
    /// A row does not name its license/advisory/changelog signal disclosure.
    SignalDisclosureMissing,
    /// A degraded resolution does not carry a degradation note.
    DegradationNoteMissing,
    /// A row does not name a primary action label.
    PrimaryActionLabelMissing,
    /// A row misrepresents action truth relative to lifecycle/relation.
    ActionTruthMisrepresented,
    /// A directly-actionable row offers an action without provenance.
    ActionProvenanceMissing,
    /// A read-only or blocked row does not name its blocked reason.
    BlockedReasonMissing,
    /// A transitive (or direct-and-transitive) row does not explain its relation.
    TransitiveRelationNotExplained,
    /// A row offering install or update does not name a candidate version.
    CandidateVersionMissing,
    /// The rollback posture is inconsistent with the action truth.
    RollbackPostureInconsistent,
    /// The rows do not cover installed, available, and outdated states.
    LifecycleCoverageMissing,
    /// The rows do not cover a non-actionable (read-only/blocked) state.
    NonActionableStateCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PackageExplorerRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsMissing => "rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowWrongComponentClass => "row_wrong_component_class",
            Self::ManifestScopeDisclosureMissing => "manifest_scope_disclosure_missing",
            Self::RegistrySourceDisclosureMissing => "registry_source_disclosure_missing",
            Self::SignalDisclosureMissing => "signal_disclosure_missing",
            Self::DegradationNoteMissing => "degradation_note_missing",
            Self::PrimaryActionLabelMissing => "primary_action_label_missing",
            Self::ActionTruthMisrepresented => "action_truth_misrepresented",
            Self::ActionProvenanceMissing => "action_provenance_missing",
            Self::BlockedReasonMissing => "blocked_reason_missing",
            Self::TransitiveRelationNotExplained => "transitive_relation_not_explained",
            Self::CandidateVersionMissing => "candidate_version_missing",
            Self::RollbackPostureInconsistent => "rollback_posture_inconsistent",
            Self::LifecycleCoverageMissing => "lifecycle_coverage_missing",
            Self::NonActionableStateCoverageMissing => "non_actionable_state_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable package-explorer-row export.
pub fn current_package_explorer_row_export(
) -> Result<PackageExplorerRowPacket, PackageExplorerRowArtifactError> {
    let packet: PackageExplorerRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-package-explorer-row-proof/support_export.json"
    )))
    .map_err(PackageExplorerRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PackageExplorerRowArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &PackageExplorerRowPacket,
    violations: &mut Vec<PackageExplorerRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PACKAGE_EXPLORER_ROW_SCHEMA_REF,
        PACKAGE_EXPLORER_ROW_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PackageExplorerRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &PackageExplorerRowPacket,
    violations: &mut Vec<PackageExplorerRowViolation>,
) {
    if packet.rows.is_empty() {
        violations.push(PackageExplorerRowViolation::RowsMissing);
        return;
    }

    let mut lifecycles: BTreeSet<PackageLifecycleState> = BTreeSet::new();
    let mut has_non_actionable = false;

    for row in &packet.rows {
        lifecycles.insert(row.lifecycle);

        if row.row_id.trim().is_empty()
            || row.package_label.trim().is_empty()
            || row.ecosystem.trim().is_empty()
            || row.primary_action_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(PackageExplorerRowViolation::RowIncomplete);
        }
        if row.component != M5PackageComponent::PackageExplorerRow {
            violations.push(PackageExplorerRowViolation::RowWrongComponentClass);
        }
        if row.manifest_scope_disclosure.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::ManifestScopeDisclosureMissing);
        }
        if row.registry_source_disclosure.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::RegistrySourceDisclosureMissing);
        }
        if row.signal_disclosure.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::SignalDisclosureMissing);
        }
        if row.primary_action_label.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::PrimaryActionLabelMissing);
        }
        if !matches!(
            row.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && row.degradation_note.trim().is_empty()
        {
            violations.push(PackageExplorerRowViolation::DegradationNoteMissing);
        }

        let disclosure = row.action_disclosure();

        if !disclosure.is_directly_actionable {
            has_non_actionable = true;
        }
        if row.offers_direct_action != disclosure.is_directly_actionable {
            violations.push(PackageExplorerRowViolation::ActionTruthMisrepresented);
        }
        if row.offers_direct_action && row.action_provenance_note.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::ActionProvenanceMissing);
        }
        if disclosure.needs_blocked_reason && row.blocked_reason.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::BlockedReasonMissing);
        }
        if disclosure.needs_relation_note && row.relation_note.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::TransitiveRelationNotExplained);
        }
        if disclosure.needs_candidate_version && row.candidate_version.trim().is_empty() {
            violations.push(PackageExplorerRowViolation::CandidateVersionMissing);
        }
        if !row.rollback_posture_consistent() {
            violations.push(PackageExplorerRowViolation::RollbackPostureInconsistent);
        }
    }

    for required in [
        PackageLifecycleState::Installed,
        PackageLifecycleState::Available,
        PackageLifecycleState::Outdated,
    ] {
        if !lifecycles.contains(&required) {
            violations.push(PackageExplorerRowViolation::LifecycleCoverageMissing);
            break;
        }
    }

    if !has_non_actionable {
        violations.push(PackageExplorerRowViolation::NonActionableStateCoverageMissing);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
