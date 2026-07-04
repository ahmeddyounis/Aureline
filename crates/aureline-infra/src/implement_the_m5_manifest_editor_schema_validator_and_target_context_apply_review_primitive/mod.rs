//! Implements the reusable manifest-authoring primitive: a manifest-editor header,
//! a schema / validator row, a target-context chip group, and an apply-review
//! banner that all resolve from one authoring context and share one target
//! identity, so config-authoring surfaces are truthful *before* users validate,
//! preview, or mutate live infrastructure.
//!
//! Where
//! [`crate::freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix`]
//! *freezes* the reusable manifest / build-confidence component families as a
//! governed contract, this module *narrows* three of those families —
//! [`crate::M5ManifestBuildComponentFamily::ManifestEditorHeader`],
//! [`crate::M5ManifestBuildComponentFamily::SchemaValidatorRow`], and
//! [`crate::M5ManifestBuildComponentFamily::TargetContextChipGroup`] — plus the
//! apply-review banner they imply into one working primitive with a real
//! **resolver**. A single manifest-authoring context projects onto four surfaces
//! that share one authoring identity and one truth class, so environment, schema
//! source, schema freshness, and desired / rendered / live / preview / apply state
//! never blur across the header, the validator row, the chip group, and the
//! apply-review banner.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — environment and schema source are never hidden.** Every projection
//!   carries the target-identity ref and the schema source kind; a header, a
//!   validator row, a chip group, and an apply banner never hide which environment
//!   or schema source the user is acting against.
//! - **AC2 — desired / rendered / live / preview / apply state stays explicit
//!   before mutation.** The apply-review banner discloses the truth class, the
//!   create / update / delete counts where known, dry-run availability, and
//!   rollback / checkpoint posture, and never offers an apply until the target is
//!   resolved and the schema / validator permits it.
//! - **AC3 — schema / validator freshness is visible wherever a manifest is
//!   trustworthy.** Both the header and the validator row carry the same schema
//!   freshness, so a stale or unversioned schema can never masquerade as fresh on
//!   any surface a manifest is considered on.
//!
//! Raw manifest bodies, credentials, connector tokens, and endpoint data never
//! cross this boundary; the resolver carries only opaque refs, typed class tokens,
//! booleans, and redacted labels, so support and diagnostics exports reconstruct
//! exactly what a surface would have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-manifest-authoring-primitive.schema.json`](../../../../schemas/ui/m5-manifest-authoring-primitive.schema.json).
//! The contract doc is
//! [`docs/infra/m5_manifest_authoring_primitive.md`](../../../../docs/infra/m5_manifest_authoring_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    truth_mode_token, DegradedState, M5ManifestBuildDowngradeTrigger, M5ManifestEditPosture,
    M5SchemaFreshness, M5SchemaValidationState, TruthMode,
};

/// Stable record-kind tag carried by [`M5ManifestAuthoringPrimitivePacket`].
pub const M5_MANIFEST_AUTHORING_RECORD_KIND: &str = "m5_manifest_authoring_primitive";

/// Schema version for the manifest-authoring primitive packet.
pub const M5_MANIFEST_AUTHORING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_MANIFEST_AUTHORING_SCHEMA_REF: &str =
    "schemas/ui/m5-manifest-authoring-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MANIFEST_AUTHORING_DOC_REF: &str = "docs/infra/m5_manifest_authoring_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive
/// narrows.
pub const M5_MANIFEST_AUTHORING_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MANIFEST_AUTHORING_FIXTURE_DIR: &str = "fixtures/ui/m5-manifest-authoring-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_MANIFEST_AUTHORING_ARTIFACT_REF: &str =
    "artifacts/release/m5-manifest-authoring-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_MANIFEST_AUTHORING_CSV_REF: &str =
    "artifacts/release/m5-manifest-authoring-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_MANIFEST_AUTHORING_REPORT_REF: &str =
    "artifacts/release/m5-manifest-authoring-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed manifest-authoring surface family. Each family is one parity surface that
/// ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestAuthoringSurfaceFamily {
    /// The desktop manifest editor where files are authored.
    DesktopManifestEditor,
    /// The plan / diff / dry-run preview pane.
    PlanPreviewPane,
    /// The cluster / resource explorer that reads live truth.
    ClusterResourceExplorer,
    /// The apply-review dialog gating a mutation.
    ApplyReviewDialog,
    /// The provider-console handoff surface.
    ProviderConsoleHandoff,
    /// The support / export replay surface that reconstructs authoring truth.
    SupportExportReplay,
}

impl M5ManifestAuthoringSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopManifestEditor,
        Self::PlanPreviewPane,
        Self::ClusterResourceExplorer,
        Self::ApplyReviewDialog,
        Self::ProviderConsoleHandoff,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopManifestEditor => "desktop_manifest_editor",
            Self::PlanPreviewPane => "plan_preview_pane",
            Self::ClusterResourceExplorer => "cluster_resource_explorer",
            Self::ApplyReviewDialog => "apply_review_dialog",
            Self::ProviderConsoleHandoff => "provider_console_handoff",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DesktopManifestEditor => "Desktop manifest editor",
            Self::PlanPreviewPane => "Plan / preview pane",
            Self::ClusterResourceExplorer => "Cluster / resource explorer",
            Self::ApplyReviewDialog => "Apply-review dialog",
            Self::ProviderConsoleHandoff => "Provider-console handoff",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed manifest source-type vocabulary. Names where the manifest content came
/// from so an authored file, a rendered artifact, and a provider overlay never
/// read as one another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestSourceType {
    /// A repo-authored source file.
    AuthoredFile,
    /// A rendered / generated artifact derived from authored inputs.
    RenderedArtifact,
    /// An imported snapshot from a prior run.
    ImportedSnapshot,
    /// A provider-owned overlay / console-only manifest.
    ProviderOverlay,
    /// A generated template not yet owned by source.
    GeneratedTemplate,
}

impl M5ManifestSourceType {
    /// Every source type, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AuthoredFile,
        Self::RenderedArtifact,
        Self::ImportedSnapshot,
        Self::ProviderOverlay,
        Self::GeneratedTemplate,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredFile => "authored_file",
            Self::RenderedArtifact => "rendered_artifact",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::ProviderOverlay => "provider_overlay",
            Self::GeneratedTemplate => "generated_template",
        }
    }

    /// True when source is repo-owned and safe to offer a source-first edit path.
    pub const fn is_source_owned(self) -> bool {
        matches!(self, Self::AuthoredFile)
    }
}

/// Closed execution-origin vocabulary. Names where an apply would execute so the
/// header never conceals target identity behind a generic "apply".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionOrigin {
    /// A local workspace with no live connector.
    LocalWorkspace,
    /// A connected live cluster / account.
    ConnectedCluster,
    /// A dry-run sandbox that never mutates live truth.
    DryRunSandbox,
    /// An imported replay of a prior run.
    ImportedReplay,
    /// A provider console the action hands off to.
    ProviderConsole,
}

impl M5ExecutionOrigin {
    /// Every execution origin, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalWorkspace,
        Self::ConnectedCluster,
        Self::DryRunSandbox,
        Self::ImportedReplay,
        Self::ProviderConsole,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::ConnectedCluster => "connected_cluster",
            Self::DryRunSandbox => "dry_run_sandbox",
            Self::ImportedReplay => "imported_replay",
            Self::ProviderConsole => "provider_console",
        }
    }

    /// True when an apply against this origin mutates live truth.
    pub const fn mutates_live(self) -> bool {
        matches!(self, Self::ConnectedCluster)
    }
}

/// Closed schema-source vocabulary. Names where the schema resolved from so an
/// imported or provider-overlay schema never claims bundled / registry authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SchemaSourceKind {
    /// Bundled with the application.
    BundledWithApp,
    /// A remote schema registry.
    RemoteRegistry,
    /// Discovered from a live cluster / account.
    ClusterDiscovered,
    /// An imported snapshot from a prior run.
    ImportedSnapshot,
    /// A provider-owned overlay schema.
    ProviderOverlay,
    /// Schema source not yet established.
    Unknown,
}

impl M5SchemaSourceKind {
    /// Every schema source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundledWithApp,
        Self::RemoteRegistry,
        Self::ClusterDiscovered,
        Self::ImportedSnapshot,
        Self::ProviderOverlay,
        Self::Unknown,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledWithApp => "bundled_with_app",
            Self::RemoteRegistry => "remote_registry",
            Self::ClusterDiscovered => "cluster_discovered",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::ProviderOverlay => "provider_overlay",
            Self::Unknown => "unknown",
        }
    }

    /// True when the schema source is explicitly established (not `Unknown`).
    pub const fn is_explicit(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Closed dry-run-availability vocabulary. Names whether a plan / dry-run can be
/// run before an apply, and why not when it cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DryRunAvailability {
    /// A dry-run / plan is available before apply.
    Available,
    /// A dry-run is unavailable because the live connector was lost.
    UnavailableConnectorLost,
    /// A dry-run is unavailable because a policy / capability block prevents it.
    UnavailablePolicyBlocked,
    /// A dry-run does not apply to this surface (read-only or offline).
    NotApplicable,
}

impl M5DryRunAvailability {
    /// Every dry-run availability, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Available,
        Self::UnavailableConnectorLost,
        Self::UnavailablePolicyBlocked,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::UnavailableConnectorLost => "unavailable_connector_lost",
            Self::UnavailablePolicyBlocked => "unavailable_policy_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed rollback / checkpoint posture vocabulary. Names what recovery is
/// available if an apply goes wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackPosture {
    /// A checkpoint / snapshot exists to restore from.
    CheckpointAvailable,
    /// Rollback is supported without an explicit checkpoint.
    RollbackSupported,
    /// No rollback is available; the apply is irreversible.
    NoRollback,
    /// Rollback posture not yet established.
    Unknown,
}

impl M5RollbackPosture {
    /// Every rollback posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CheckpointAvailable,
        Self::RollbackSupported,
        Self::NoRollback,
        Self::Unknown,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointAvailable => "checkpoint_available",
            Self::RollbackSupported => "rollback_supported",
            Self::NoRollback => "no_rollback",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet
/// must carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestAuthoringExportField {
    /// The stable authoring identity shared across surfaces.
    AuthoringId,
    /// The opaque manifest ref.
    ManifestRef,
    /// The target-identity ref the surface acts against.
    TargetIdentity,
    /// The authored / rendered / planned / live / provider-overlay truth class.
    TruthClass,
    /// The schema freshness disclosed on the header and validator row.
    SchemaFreshness,
    /// The execution origin an apply would run against.
    ExecutionOrigin,
    /// The create / update / delete mutation counts where known.
    MutationCounts,
}

impl M5ManifestAuthoringExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AuthoringId,
        Self::ManifestRef,
        Self::TargetIdentity,
        Self::TruthClass,
        Self::SchemaFreshness,
        Self::ExecutionOrigin,
        Self::MutationCounts,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::AuthoringId,
        Self::ManifestRef,
        Self::TargetIdentity,
        Self::TruthClass,
        Self::SchemaFreshness,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoringId => "authoring_id",
            Self::ManifestRef => "manifest_ref",
            Self::TargetIdentity => "target_identity",
            Self::TruthClass => "truth_class",
            Self::SchemaFreshness => "schema_freshness",
            Self::ExecutionOrigin => "execution_origin",
            Self::MutationCounts => "mutation_counts",
        }
    }
}

// --- shared value structs ---

/// The cluster / project / namespace / account context the chip group pins. Every
/// slot is opaque; raw endpoint data never crosses this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TargetContextChips {
    /// The account / subscription reference.
    pub account: Option<String>,
    /// The project reference.
    pub project: Option<String>,
    /// The cluster reference.
    pub cluster: Option<String>,
    /// The namespace reference.
    pub namespace: Option<String>,
}

impl M5TargetContextChips {
    /// True when the context is complete enough to act against: an account and at
    /// least one scope (project or cluster) are present, and every present slot is
    /// non-empty.
    pub fn is_complete(&self) -> bool {
        let all_present_non_empty = [&self.account, &self.project, &self.cluster, &self.namespace]
            .into_iter()
            .flatten()
            .all(|value| !value.trim().is_empty());
        let has_account = self
            .account
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty());
        let has_scope = [&self.project, &self.cluster]
            .into_iter()
            .flatten()
            .any(|value| !value.trim().is_empty());
        all_present_non_empty && has_account && has_scope
    }

    /// The count of resolved (present, non-empty) context chips.
    pub fn resolved_chip_count(&self) -> usize {
        [&self.account, &self.project, &self.cluster, &self.namespace]
            .into_iter()
            .flatten()
            .filter(|value| !value.trim().is_empty())
            .count()
    }
}

/// The create / update / delete counts an apply would perform, where known.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationCounts {
    /// Resources created.
    pub creates: u32,
    /// Resources updated.
    pub updates: u32,
    /// Resources deleted.
    pub deletes: u32,
}

impl M5MutationCounts {
    /// The total number of mutations.
    pub const fn total(self) -> u32 {
        self.creates + self.updates + self.deletes
    }
}

// --- resolver input ---

/// The full input to the manifest-authoring resolver for one manifest context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringInput {
    /// The stable authoring identity that must survive across header, validator
    /// row, chip group, and apply banner.
    pub authoring_id: String,
    /// Opaque ref to the manifest object; never raw manifest bytes.
    pub manifest_ref: String,
    /// Human-readable manifest / file label.
    pub manifest_label: String,
    /// Where the manifest content came from.
    pub source_type: M5ManifestSourceType,
    /// The truth class the manifest is shown in.
    pub truth_mode: TruthMode,
    /// Where the backing schema resolved from.
    pub schema_source: M5SchemaSourceKind,
    /// Opaque schema version / snapshot-date label, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version_label: Option<String>,
    /// The freshness of the schema backing the manifest.
    pub schema_freshness: M5SchemaFreshness,
    /// The validator verdict.
    pub validation_state: M5SchemaValidationState,
    /// A precise policy / offline note, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_offline_note: Option<String>,
    /// Opaque ref to the schema docs the open-docs action targets, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_ref: Option<String>,
    /// The edit posture the header offers.
    pub edit_posture: M5ManifestEditPosture,
    /// Where an apply would execute.
    pub execution_origin: M5ExecutionOrigin,
    /// Opaque ref to the target identity the chips name; never raw endpoint data.
    pub target_identity_ref: String,
    /// The cluster / project / namespace / account context.
    pub target_context: M5TargetContextChips,
    /// The create / update / delete counts, where known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_counts: Option<M5MutationCounts>,
    /// Whether a dry-run / plan is available before apply.
    pub dry_run: M5DryRunAvailability,
    /// The rollback / checkpoint posture.
    pub rollback: M5RollbackPosture,
    /// An externally-observed narrowing (drift, connector loss, policy block) that
    /// degrades the surface before execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved manifest-editor header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedManifestHeader {
    /// The authoring identity — identical to the validator row, chips, and banner.
    pub authoring_id: String,
    /// The opaque manifest ref.
    pub manifest_ref: String,
    /// The manifest / file label.
    pub manifest_label: String,
    /// Where the manifest content came from.
    pub source_type: M5ManifestSourceType,
    /// The truth class the manifest is shown in.
    pub truth_mode: TruthMode,
    /// The schema freshness disclosed on the header.
    pub schema_freshness: M5SchemaFreshness,
    /// The edit posture offered.
    pub edit_posture: M5ManifestEditPosture,
    /// Where an apply would execute.
    pub execution_origin: M5ExecutionOrigin,
    /// Target context is always visible on the header; always holds.
    pub target_context_visible: bool,
    /// The preview / plan entry point is offered.
    pub preview_available: bool,
    /// The apply entry point is offered (only when fully gated).
    pub apply_available: bool,
}

/// The resolved schema / validator row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSchemaValidatorRow {
    /// The authoring identity — identical to every other surface.
    pub authoring_id: String,
    /// Where the schema resolved from.
    pub schema_source: M5SchemaSourceKind,
    /// The opaque schema version / snapshot-date label, when known.
    pub schema_version_label: Option<String>,
    /// The schema freshness — identical to the header's freshness.
    pub schema_freshness: M5SchemaFreshness,
    /// The validator verdict.
    pub validation_state: M5SchemaValidationState,
    /// The row blocks apply when the validation state requires it.
    pub blocks_apply: bool,
    /// The precise policy / offline note, when applicable.
    pub policy_offline_note: Option<String>,
    /// The open-docs action is offered when a docs ref is available.
    pub open_docs_available: bool,
}

/// The resolved target-context chip group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTargetContextChips {
    /// The authoring identity — identical to every other surface.
    pub authoring_id: String,
    /// The truth class the chip group discloses.
    pub truth_mode: TruthMode,
    /// The opaque target-identity ref.
    pub target_identity_ref: String,
    /// The cluster / project / namespace / account chips.
    pub chips: M5TargetContextChips,
    /// Target identity, environment, and scope are all shown.
    pub context_complete: bool,
    /// The chip group stays visible as the surface scrolls; always holds.
    pub stays_visible: bool,
}

/// The resolved apply-review banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedApplyReviewBanner {
    /// The authoring identity — identical to every other surface.
    pub authoring_id: String,
    /// The truth class the banner discloses.
    pub truth_mode: TruthMode,
    /// The create / update / delete counts, where known.
    pub mutation_counts: Option<M5MutationCounts>,
    /// Whether the mutation counts are known.
    pub counts_known: bool,
    /// Whether a dry-run / plan is available before apply.
    pub dry_run: M5DryRunAvailability,
    /// The rollback / checkpoint posture.
    pub rollback: M5RollbackPosture,
    /// The preview / plan entry point is offered.
    pub preview_available: bool,
    /// The apply entry point is offered (only when fully gated).
    pub apply_available: bool,
    /// Why apply is gated, when it is; names a real, reconstructable trigger.
    pub apply_blocked_reason: Option<M5ManifestBuildDowngradeTrigger>,
}

/// The resolved manifest-authoring truth shared across header, validator row, chip
/// group, and apply banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedManifestAuthoring {
    /// The stable authoring identity.
    pub authoring_id: String,
    /// The resolved manifest-editor header.
    pub header: M5ResolvedManifestHeader,
    /// The resolved schema / validator row.
    pub schema_row: M5ResolvedSchemaValidatorRow,
    /// The resolved target-context chip group.
    pub context_chips: M5ResolvedTargetContextChips,
    /// The resolved apply-review banner.
    pub apply_banner: M5ResolvedApplyReviewBanner,
    /// The environment (target identity) and schema source are disclosed on every
    /// surface (AC1).
    pub environment_and_schema_source_disclosed: bool,
    /// Schema freshness is visible on both the header and the validator row (AC3).
    pub schema_freshness_visible: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedManifestAuthoring {
    /// True when the authoring identity is identical across the header, validator
    /// row, chip group, and apply banner.
    pub fn identity_consistent(&self) -> bool {
        self.header.authoring_id == self.authoring_id
            && self.schema_row.authoring_id == self.authoring_id
            && self.context_chips.authoring_id == self.authoring_id
            && self.apply_banner.authoring_id == self.authoring_id
    }

    /// True when the header, chip group, and apply banner all disclose the same
    /// truth class — desired / rendered / live / preview / apply never blurs.
    pub fn truth_class_consistent(&self) -> bool {
        self.header.truth_mode == self.context_chips.truth_mode
            && self.header.truth_mode == self.apply_banner.truth_mode
    }

    /// True when the environment (target identity) and schema source are disclosed
    /// (AC1).
    pub fn environment_disclosed(&self) -> bool {
        self.environment_and_schema_source_disclosed
    }

    /// True when desired / rendered / live / preview / apply state stays explicit
    /// before mutation: an apply is never offered unless a preview path and a
    /// resolved target back it (AC2).
    pub fn states_explicit_before_mutation(&self) -> bool {
        !self.apply_banner.apply_available
            || (self.header.preview_available
                && self.context_chips.context_complete
                && !self.schema_row.blocks_apply)
    }

    /// True when schema freshness is visible on the header and validator row (AC3).
    pub fn schema_freshness_disclosed(&self) -> bool {
        self.schema_freshness_visible
            && self.header.schema_freshness == self.schema_row.schema_freshness
    }
}

/// Errors returned by [`resolve_manifest_authoring`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ManifestAuthoringResolutionError {
    /// The authoring identity was empty.
    EmptyAuthoringId,
    /// The manifest ref was empty.
    EmptyManifestRef,
    /// The manifest label was empty.
    EmptyManifestLabel,
    /// The target-identity ref was empty.
    EmptyTargetIdentityRef,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// A writable manifest claimed an editable posture with no resolvable schema.
    WritableManifestWithoutSchema,
    /// An apply / write posture was offered against an unresolved target context.
    ApplyPostureOnUnresolvedTarget,
    /// Mutation counts were supplied on a surface with no write path.
    MutationCountsWithoutWritePath,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5ManifestAuthoringResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAuthoringId => "empty_authoring_id",
            Self::EmptyManifestRef => "empty_manifest_ref",
            Self::EmptyManifestLabel => "empty_manifest_label",
            Self::EmptyTargetIdentityRef => "empty_target_identity_ref",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::WritableManifestWithoutSchema => "writable_manifest_without_schema",
            Self::ApplyPostureOnUnresolvedTarget => "apply_posture_on_unresolved_target",
            Self::MutationCountsWithoutWritePath => "mutation_counts_without_write_path",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5ManifestAuthoringResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "manifest-authoring resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ManifestAuthoringResolutionError {}

/// Resolves one manifest-authoring context into its shared header, schema /
/// validator row, target-context chip group, and apply-review banner.
///
/// The four surfaces share one authoring identity and one truth class, so
/// environment, schema source, schema freshness, and desired / rendered / live /
/// preview / apply state never blur across them. An apply is never offered until
/// the target context is resolved and the schema / validator permits it; a
/// degraded input narrows the apply before execution rather than after a run
/// starts.
pub fn resolve_manifest_authoring(
    input: &M5ManifestAuthoringInput,
) -> Result<M5ResolvedManifestAuthoring, M5ManifestAuthoringResolutionError> {
    if input.authoring_id.trim().is_empty() {
        return Err(M5ManifestAuthoringResolutionError::EmptyAuthoringId);
    }
    if input.manifest_ref.trim().is_empty() {
        return Err(M5ManifestAuthoringResolutionError::EmptyManifestRef);
    }
    if input.manifest_label.trim().is_empty() {
        return Err(M5ManifestAuthoringResolutionError::EmptyManifestLabel);
    }
    if input.target_identity_ref.trim().is_empty() {
        return Err(M5ManifestAuthoringResolutionError::EmptyTargetIdentityRef);
    }

    for value in [
        input.manifest_ref.as_str(),
        input.manifest_label.as_str(),
        input.target_identity_ref.as_str(),
    ]
    .into_iter()
    .chain(input.schema_version_label.as_deref())
    .chain(input.policy_offline_note.as_deref())
    .chain(input.docs_ref.as_deref())
    {
        if value_is_forbidden(value) {
            return Err(M5ManifestAuthoringResolutionError::ForbiddenMaterial);
        }
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5ManifestAuthoringResolutionError::DegradedLabelGeneric);
        }
    }

    let writes = input.edit_posture.writes_manifest();
    let context_complete = input.target_context.is_complete();

    // A writable manifest cannot claim an editable posture with no resolvable
    // schema — a mutation would then run against nothing trustworthy.
    if writes && input.schema_freshness == M5SchemaFreshness::Unavailable {
        return Err(M5ManifestAuthoringResolutionError::WritableManifestWithoutSchema);
    }
    // An apply / write posture is never offered until the target context resolves.
    if writes && !context_complete {
        return Err(M5ManifestAuthoringResolutionError::ApplyPostureOnUnresolvedTarget);
    }
    // Mutation counts only make sense on a surface that can write.
    if input.mutation_counts.is_some() && !writes {
        return Err(M5ManifestAuthoringResolutionError::MutationCountsWithoutWritePath);
    }

    let blocks_apply = input.validation_state.must_block_apply();
    let preview_available = input.edit_posture != M5ManifestEditPosture::BlockedProtected
        && input.schema_freshness != M5SchemaFreshness::Unavailable;

    // The apply gate: a write path, a resolved target, a validator that permits it,
    // and no active narrowing. Everything else narrows the apply before execution.
    let apply_available = writes
        && context_complete
        && !blocks_apply
        && input.degraded.is_none()
        && !matches!(
            input.dry_run,
            M5DryRunAvailability::UnavailablePolicyBlocked
        );

    let apply_blocked_reason = if !writes || apply_available {
        None
    } else if let Some(degraded) = &input.degraded {
        Some(degraded.trigger)
    } else if !context_complete {
        Some(M5ManifestBuildDowngradeTrigger::TargetContextUnresolved)
    } else if input.dry_run == M5DryRunAvailability::UnavailablePolicyBlocked {
        Some(M5ManifestBuildDowngradeTrigger::PolicyBlock)
    } else if blocks_apply {
        Some(M5ManifestBuildDowngradeTrigger::SchemaStale)
    } else {
        None
    };

    let header = M5ResolvedManifestHeader {
        authoring_id: input.authoring_id.clone(),
        manifest_ref: input.manifest_ref.clone(),
        manifest_label: input.manifest_label.clone(),
        source_type: input.source_type,
        truth_mode: input.truth_mode,
        schema_freshness: input.schema_freshness,
        edit_posture: input.edit_posture,
        execution_origin: input.execution_origin,
        target_context_visible: true,
        preview_available,
        apply_available,
    };

    let schema_row = M5ResolvedSchemaValidatorRow {
        authoring_id: input.authoring_id.clone(),
        schema_source: input.schema_source,
        schema_version_label: input.schema_version_label.clone(),
        schema_freshness: input.schema_freshness,
        validation_state: input.validation_state,
        blocks_apply,
        policy_offline_note: input.policy_offline_note.clone(),
        open_docs_available: input.docs_ref.is_some(),
    };

    let context_chips = M5ResolvedTargetContextChips {
        authoring_id: input.authoring_id.clone(),
        truth_mode: input.truth_mode,
        target_identity_ref: input.target_identity_ref.clone(),
        chips: input.target_context.clone(),
        context_complete,
        stays_visible: true,
    };

    let apply_banner = M5ResolvedApplyReviewBanner {
        authoring_id: input.authoring_id.clone(),
        truth_mode: input.truth_mode,
        mutation_counts: input.mutation_counts,
        counts_known: input.mutation_counts.is_some(),
        dry_run: input.dry_run,
        rollback: input.rollback,
        preview_available,
        apply_available,
        apply_blocked_reason,
    };

    // The environment (target identity) and schema source are always disclosed
    // structurally — the chip group names the target, and the validator row names
    // the schema source kind explicitly.
    let environment_and_schema_source_disclosed =
        !input.target_identity_ref.trim().is_empty() && input.schema_source.is_explicit();
    // Schema freshness is always rendered on both the header and the validator row.
    let schema_freshness_visible = true;

    Ok(M5ResolvedManifestAuthoring {
        authoring_id: input.authoring_id.clone(),
        header,
        schema_row,
        context_chips,
        apply_banner,
        environment_and_schema_source_disclosed,
        schema_freshness_visible,
        degraded: input.degraded.clone(),
    })
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs authoring truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringCase {
    /// The resolver input.
    pub input: M5ManifestAuthoringInput,
    /// The resolved authoring truth. Must equal
    /// `resolve_manifest_authoring(&input)`.
    pub resolved: M5ResolvedManifestAuthoring,
}

impl M5ManifestAuthoringCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ManifestAuthoringInput) -> Self {
        let resolved = resolve_manifest_authoring(&input).expect("seed authoring case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_manifest_authoring(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one authoring surface family bound to the
/// shared manifest-authoring contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringSurfaceRow {
    /// The authoring surface family.
    pub surface_family: M5ManifestAuthoringSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Source types this surface can disclose (must be non-empty).
    pub source_types: Vec<M5ManifestSourceType>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<TruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5ManifestAuthoringExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ManifestBuildDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_authoring: Vec<M5ManifestAuthoringCase>,
    /// Hard invariant: this row never hides the environment or schema source. MUST
    /// be `false`.
    pub hides_environment_or_schema_source: bool,
    /// Hard invariant: this row never blurs desired / rendered / live / preview /
    /// apply truth. MUST be `false`.
    pub blurs_truth_states: bool,
    /// Hard invariant: this row never hides schema freshness where a manifest is
    /// trusted. MUST be `false`.
    pub hides_schema_freshness: bool,
    /// Hard invariant: this row never offers apply before review. MUST be `false`.
    pub offers_apply_before_review: bool,
}

impl M5ManifestAuthoringSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ManifestAuthoringExportField> =
            self.export_fields.iter().copied().collect();
        M5ManifestAuthoringExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_environment_or_schema_source
            && !self.blurs_truth_states
            && !self.hides_schema_freshness
            && !self.offers_apply_before_review
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringVocabularySet {
    /// Authoring surface-family tokens.
    pub surface_families: Vec<String>,
    /// Manifest source-type tokens.
    pub source_types: Vec<String>,
    /// Execution-origin tokens.
    pub execution_origins: Vec<String>,
    /// Schema-source tokens.
    pub schema_sources: Vec<String>,
    /// Dry-run-availability tokens.
    pub dry_run_states: Vec<String>,
    /// Rollback-posture tokens.
    pub rollback_postures: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Schema-freshness tokens (reused from the frozen matrix).
    pub schema_freshness: Vec<String>,
    /// Validation-state tokens (reused from the frozen matrix).
    pub validation_states: Vec<String>,
    /// Edit-posture tokens (reused from the frozen matrix).
    pub edit_postures: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5ManifestAuthoringVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5ManifestAuthoringSurfaceFamily::ALL, |v| v.as_str()),
            source_types: tokens(&M5ManifestSourceType::ALL, |v| v.as_str()),
            execution_origins: tokens(&M5ExecutionOrigin::ALL, |v| v.as_str()),
            schema_sources: tokens(&M5SchemaSourceKind::ALL, |v| v.as_str()),
            dry_run_states: tokens(&M5DryRunAvailability::ALL, |v| v.as_str()),
            rollback_postures: tokens(&M5RollbackPosture::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ManifestAuthoringExportField::ALL, |v| v.as_str()),
            truth_modes: tokens(&TRUTH_MODE_ALL, truth_mode_token),
            schema_freshness: tokens(&SCHEMA_FRESHNESS_ALL, |v| v.as_str()),
            validation_states: tokens(&VALIDATION_STATE_ALL, |v| v.as_str()),
            edit_postures: tokens(&EDIT_POSTURE_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The truth classes reused from the frozen matrix, in a stable order. [`TruthMode`]
/// is a pure token set, so the order is pinned here.
const TRUTH_MODE_ALL: [TruthMode; 5] = [
    TruthMode::Desired,
    TruthMode::Rendered,
    TruthMode::Plan,
    TruthMode::Live,
    TruthMode::ProviderOverlay,
];

/// The schema-freshness states reused from the frozen matrix, in a stable order.
const SCHEMA_FRESHNESS_ALL: [M5SchemaFreshness; 4] = [
    M5SchemaFreshness::Fresh,
    M5SchemaFreshness::Stale,
    M5SchemaFreshness::Unversioned,
    M5SchemaFreshness::Unavailable,
];

/// The validation states reused from the frozen matrix, in a stable order.
const VALIDATION_STATE_ALL: [M5SchemaValidationState; 5] = [
    M5SchemaValidationState::Valid,
    M5SchemaValidationState::Warnings,
    M5SchemaValidationState::Errors,
    M5SchemaValidationState::SchemaUnavailable,
    M5SchemaValidationState::Unversioned,
];

/// The edit postures reused from the frozen matrix, in a stable order.
const EDIT_POSTURE_ALL: [M5ManifestEditPosture; 3] = [
    M5ManifestEditPosture::ReadOnly,
    M5ManifestEditPosture::PreviewApplyReview,
    M5ManifestEditPosture::BlockedProtected,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5ManifestBuildDowngradeTrigger; 8] = [
    M5ManifestBuildDowngradeTrigger::SchemaStale,
    M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
    M5ManifestBuildDowngradeTrigger::ConnectorLoss,
    M5ManifestBuildDowngradeTrigger::PolicyBlock,
    M5ManifestBuildDowngradeTrigger::DriftFromSource,
    M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
    M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
    M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringGovernanceReview {
    /// One primitive carries header / validator-row / chip-group / apply-banner
    /// truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Authoring identity is preserved across header, validator row, chips, and
    /// banner.
    pub authoring_identity_preserved_across_surfaces: bool,
    /// Environment and schema source are never hidden.
    pub environment_and_schema_source_never_hidden: bool,
    /// Desired / rendered / live / preview / apply state is explicit before
    /// mutation.
    pub states_explicit_before_mutation: bool,
    /// Schema freshness is visible wherever a manifest is trusted.
    pub schema_freshness_visible_where_trusted: bool,
    /// The support / export packet reconstructs authoring truth.
    pub support_export_reconstructs_authoring: bool,
    /// Later M5 rows cannot invent parallel manifest-authoring vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringConsumerProjection {
    /// Editor / preview / explorer / apply-review surfaces all consume the shared
    /// primitive.
    pub authoring_surfaces_consume_shared_primitive: bool,
    /// The authoring resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The apply-review banner reads a single canonical mutation-gate source.
    pub apply_banner_reads_single_gate_source: bool,
    /// Support / export reads a single canonical authoring source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the manifest-authoring primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting authoring audit.
    pub authoring_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ManifestAuthoringPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ManifestAuthoringPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ManifestAuthoringSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ManifestAuthoringVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ManifestAuthoringGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ManifestAuthoringConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5ManifestAuthoringReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 manifest-authoring primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManifestAuthoringPrimitivePacket {
    /// Record kind; must equal [`M5_MANIFEST_AUTHORING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MANIFEST_AUTHORING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ManifestAuthoringSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ManifestAuthoringVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ManifestAuthoringGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ManifestAuthoringConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5ManifestAuthoringReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ManifestAuthoringPrimitivePacket {
    /// Builds an M5 manifest-authoring primitive packet from stable-lane input.
    pub fn new(input: M5ManifestAuthoringPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_MANIFEST_AUTHORING_RECORD_KIND.to_owned(),
            schema_version: M5_MANIFEST_AUTHORING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 manifest-authoring primitive invariants.
    pub fn validate(&self) -> Vec<M5ManifestAuthoringViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MANIFEST_AUTHORING_RECORD_KIND {
            violations.push(M5ManifestAuthoringViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MANIFEST_AUTHORING_SCHEMA_VERSION {
            violations.push(M5ManifestAuthoringViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ManifestAuthoringViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 manifest-authoring primitive packet serializes"),
        ) {
            violations.push(M5ManifestAuthoringViolation::RawMaterialInExport);
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
            .expect("m5 manifest-authoring primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("surface_family,owner,source_types,truth_modes,export_fields,example_count\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.source_types, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| truth_mode_token(*v)),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_authoring.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Manifest-Authoring Primitive: Header, Schema/Validator Row, Chips, and Apply-Review Banner\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Authoring surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5ManifestAuthoringSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Source types: {}\n",
            self.vocabulary_set.source_types.join(", ")
        ));
        out.push_str(&format!(
            "- Execution origins: {}\n",
            self.vocabulary_set.execution_origins.join(", ")
        ));
        out.push_str(&format!(
            "- Schema sources: {}\n",
            self.vocabulary_set.schema_sources.join(", ")
        ));
        out.push_str("\n## Authoring surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_authoring.len()
            ));
            for case in &row.example_authoring {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({}), schema `{}`, apply {}\n",
                    case.resolved.authoring_id,
                    case.resolved.header.manifest_label,
                    truth_mode_token(case.resolved.header.truth_mode),
                    case.resolved.schema_row.schema_freshness.as_str(),
                    if case.resolved.apply_banner.apply_available {
                        "available"
                    } else {
                        "gated"
                    },
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 manifest-authoring export.
#[derive(Debug)]
pub enum M5ManifestAuthoringArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ManifestAuthoringViolation>),
}

impl fmt::Display for M5ManifestAuthoringArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 manifest-authoring primitive export parse failed: {error}"
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
                    "m5 manifest-authoring primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ManifestAuthoringArtifactError {}

/// Validation failures emitted by [`M5ManifestAuthoringPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ManifestAuthoringViolation {
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
    /// A required authoring surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no source types.
    SourceTypeMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked authoring cases.
    ExampleAuthoringMissing,
    /// A worked authoring case does not match a fresh resolve of its input.
    ExampleAuthoringDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves authoring identity preserved across surfaces (AC1).
    IdentityPreservationUnproven,
    /// No worked case proves the environment and schema source disclosed (AC1).
    EnvironmentDisclosureUnproven,
    /// No worked case proves desired / rendered / live / preview / apply state
    /// explicit before mutation (AC2).
    StatesExplicitUnproven,
    /// No worked case proves schema freshness visible on header and validator row
    /// (AC3).
    SchemaFreshnessVisibilityUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ManifestAuthoringViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::SourceTypeMissing => "source_type_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleAuthoringMissing => "example_authoring_missing",
            Self::ExampleAuthoringDrift => "example_authoring_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::EnvironmentDisclosureUnproven => "environment_disclosure_unproven",
            Self::StatesExplicitUnproven => "states_explicit_unproven",
            Self::SchemaFreshnessVisibilityUnproven => "schema_freshness_visibility_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 manifest-authoring export.
pub fn current_stable_m5_manifest_authoring_export(
) -> Result<M5ManifestAuthoringPrimitivePacket, M5ManifestAuthoringArtifactError> {
    let packet: M5ManifestAuthoringPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-manifest-authoring-primitive-proof/support_export.json"
    )))
    .map_err(M5ManifestAuthoringArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ManifestAuthoringArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MANIFEST_AUTHORING_SCHEMA_REF,
        M5_MANIFEST_AUTHORING_DOC_REF,
        M5_MANIFEST_AUTHORING_COMPONENT_MATRIX_REF,
        M5_MANIFEST_AUTHORING_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ManifestAuthoringViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ManifestAuthoringViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    let present: BTreeSet<M5ManifestAuthoringSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5ManifestAuthoringSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ManifestAuthoringViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5ManifestAuthoringViolation::SurfaceRowIncomplete);
        }
        if row.source_types.is_empty() {
            violations.push(M5ManifestAuthoringViolation::SourceTypeMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5ManifestAuthoringViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ManifestAuthoringViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ManifestAuthoringViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ManifestAuthoringViolation::ConsumerSurfacesMissing);
        }
        if row.example_authoring.is_empty() {
            violations.push(M5ManifestAuthoringViolation::ExampleAuthoringMissing);
        }
        if row
            .example_authoring
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ManifestAuthoringViolation::ExampleAuthoringDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5ManifestAuthoringViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: authoring identity preserved across surfaces and environment
/// / schema source disclosed (AC1), desired / rendered / live / preview / apply
/// state explicit before mutation (AC2), and schema freshness visible on both the
/// header and the validator row (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    let cases: Vec<&M5ResolvedManifestAuthoring> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_authoring.iter().map(|case| &case.resolved))
        .collect();

    let identity_proven = cases
        .iter()
        .any(|resolved| resolved.identity_consistent() && resolved.truth_class_consistent());
    if !identity_proven {
        violations.push(M5ManifestAuthoringViolation::IdentityPreservationUnproven);
    }

    let environment_proven = cases.iter().any(|resolved| resolved.environment_disclosed());
    if !environment_proven {
        violations.push(M5ManifestAuthoringViolation::EnvironmentDisclosureUnproven);
    }

    // AC2 is proven when at least one case gates an apply before mutation (a write
    // surface whose apply is withheld) and every case keeps its states explicit.
    let states_proven = cases.iter().any(|resolved| {
        resolved.header.edit_posture.writes_manifest() && !resolved.apply_banner.apply_available
    }) && cases
        .iter()
        .all(|resolved| resolved.states_explicit_before_mutation());
    if !states_proven {
        violations.push(M5ManifestAuthoringViolation::StatesExplicitUnproven);
    }

    // AC3 is proven when at least one case discloses a non-fresh schema on both the
    // header and the validator row, and every case keeps freshness visible.
    let freshness_proven = cases.iter().any(|resolved| {
        !resolved.header.schema_freshness.is_current() && resolved.schema_freshness_disclosed()
    }) && cases
        .iter()
        .all(|resolved| resolved.schema_freshness_disclosed());
    if !freshness_proven {
        violations.push(M5ManifestAuthoringViolation::SchemaFreshnessVisibilityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.authoring_identity_preserved_across_surfaces,
        review.environment_and_schema_source_never_hidden,
        review.states_explicit_before_mutation,
        review.schema_freshness_visible_where_trusted,
        review.support_export_reconstructs_authoring,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ManifestAuthoringViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.authoring_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.apply_banner_reads_single_gate_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ManifestAuthoringViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5ManifestAuthoringPrimitivePacket,
    violations: &mut Vec<M5ManifestAuthoringViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.authoring_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ManifestAuthoringViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
