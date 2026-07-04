//! Surface certification for the M5 run-attempt, input-request, artifact-publish,
//! rerun-review, and debug-hierarchy execution-lifecycle components.
//!
//! This module is the M05-827 certification capstone that CLOSES the frozen M5
//! execution-lifecycle component lane
//! ([`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`]).
//! Where the freeze matrix defines the reusable run/attempt-header,
//! input-request-prompt, artifact-publish-row, rerun-comparison-sheet, and
//! debug-hierarchy primitives, the 821-824 implementation lanes resolve their
//! per-surface truth, and the M05-826 accessibility capstone certifies keyboard /
//! screen-reader / CLI / export parity per family, this lane keys on the **claimed
//! M5 execution surface** and certifies that the shared component family behaves
//! consistently on every consumer:
//!
//! - **Certify or auto-narrow (AC1).** Each surface either passes the shared
//!   execution-lifecycle component packet (green) or auto-narrows its interactive
//!   claim to review-required / read-only / inspect-only (yellow), disclosing the
//!   binding component group and the frozen downgrade trigger. A surface that hides
//!   drift, over-asserts control, or drops export truth is blocked (red) and may not
//!   ship.
//! - **Degraded paths narrow visibly (AC2).** Compatibility across the
//!   local / remote / container / managed / provider-backed execution paths is
//!   captured per surface; a path whose parity is not current forces the claim to
//!   narrow rather than inheriting a full-truth label from a healthier lane. The
//!   support / release export always reconstructs each surface's meaning from typed
//!   tokens without a screenshot.
//! - **Anchored to a reusable component family (AC3).** Every surface cites the ONE
//!   canonical execution-lifecycle component bundle and references the canonical
//!   component families it consumes, so M5 execution claims are anchored to a shared
//!   component family rather than feature-local status chrome.
//!
//! Each [`ExecutionSurfaceCertRow`] keys on one [`M5ExecutionClaimedSurface`] and
//! reuses the frozen [`M5ExecutionComponentFamily`], [`M5ExecutionRequiredLabel`],
//! and [`M5ExecutionDowngradeTrigger`] vocabulary plus the shared
//! [`M5ExecutionInteractiveClaim`] claim tier rather than minting parallel synonyms,
//! so the certified labels stay byte-identical to the matrix and the sibling
//! primitive and accessibility packets.
//!
//! The packet is metadata-only: raw run logs, process memory, dump payloads, symbol
//! blobs, credentials, and provider cursors never cross this boundary; the packet
//! carries only typed class tokens, opaque summary / evidence refs, booleans, and
//! redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-execution-lifecycle-surface-certification.schema.json`](../../../../schemas/ui/m5-execution-lifecycle-surface-certification.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_execution_lifecycle_surface_certification.md`](../../../../docs/run-test-debug/m5_execution_lifecycle_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    M5ExecutionComponentFamily, M5ExecutionDowngradeTrigger, M5ExecutionRequiredLabel,
};
use crate::implement_keyboard_screen_reader_cli_export_parity_and_execution_lifecycle_auto_narrowing::M5ExecutionInteractiveClaim;

/// Schema version stamped on the M05-827 execution surface certification packet.
pub const EXECUTION_SURFACE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ExecutionSurfaceCertPacket`].
pub const EXECUTION_SURFACE_CERT_RECORD_KIND: &str =
    "m5_execution_lifecycle_surface_certification_packet";

/// Stable record-kind tag carried by each [`ExecutionSurfaceCertRow`].
pub const EXECUTION_SURFACE_CERT_ROW_RECORD_KIND: &str =
    "m5_execution_lifecycle_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const EXECUTION_SURFACE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-execution-lifecycle-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const EXECUTION_SURFACE_CERT_DOC_REF: &str =
    "docs/run-test-debug/m5_execution_lifecycle_surface_certification.md";

/// Repo-relative path of the frozen execution-lifecycle component matrix this lane
/// certifies against.
pub const EXECUTION_SURFACE_CERT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the ONE canonical execution-lifecycle component bundle every
/// certified surface cites. This is the frozen M05-820 release proof — the single
/// source of truth for the reusable component family.
pub const EXECUTION_SURFACE_CERT_BUNDLE_REF: &str =
    "artifacts/release/m5-execution-lifecycle-component-proof/support_export.json";

/// Repo-relative path of the protected fixture directory.
pub const EXECUTION_SURFACE_CERT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-execution-lifecycle-surface-certification";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EXECUTION_SURFACE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-surface-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EXECUTION_SURFACE_CERT_CSV_REF: &str =
    "artifacts/release/m5-execution-lifecycle-surface-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EXECUTION_SURFACE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-surface-certification-proof/report.md";

/// The claimed M5 execution surface a certification row keys on. The first nine are
/// interactive execution consumers; the last three are release-evidence surfaces that
/// publish and replay the certified truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionClaimedSurface {
    /// The task-run execution surface.
    TaskExecution,
    /// The test-run execution surface.
    TestExecution,
    /// The API / request-run execution surface.
    RequestExecution,
    /// The database / data query execution surface.
    DatabaseExecution,
    /// The notebook-cell execution surface.
    NotebookExecution,
    /// The preview / render execution surface.
    PreviewExecution,
    /// The AI-mediated execution surface.
    AiExecution,
    /// The publish / deploy execution surface.
    PublishExecution,
    /// The debug-session execution surface.
    DebugExecution,
    /// The support / export replay surface (release evidence).
    SupportExportReplay,
    /// The docs / help embeds surface (release evidence).
    DocsHelpEmbeds,
    /// The release-proof surface (release evidence).
    ReleaseProof,
}

impl M5ExecutionClaimedSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::TaskExecution,
        Self::TestExecution,
        Self::RequestExecution,
        Self::DatabaseExecution,
        Self::NotebookExecution,
        Self::PreviewExecution,
        Self::AiExecution,
        Self::PublishExecution,
        Self::DebugExecution,
        Self::SupportExportReplay,
        Self::DocsHelpEmbeds,
        Self::ReleaseProof,
    ];

    /// The release-evidence surfaces that must each be certified so claim publication
    /// and field triage stay anchored to the same component truth.
    pub const EVIDENCE_SURFACES: [Self; 3] =
        [Self::SupportExportReplay, Self::DocsHelpEmbeds, Self::ReleaseProof];

    /// Returns true when the surface is a release-evidence surface rather than an
    /// interactive execution consumer.
    pub const fn is_evidence(self) -> bool {
        matches!(
            self,
            Self::SupportExportReplay | Self::DocsHelpEmbeds | Self::ReleaseProof
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskExecution => "task_execution",
            Self::TestExecution => "test_execution",
            Self::RequestExecution => "request_execution",
            Self::DatabaseExecution => "database_execution",
            Self::NotebookExecution => "notebook_execution",
            Self::PreviewExecution => "preview_execution",
            Self::AiExecution => "ai_execution",
            Self::PublishExecution => "publish_execution",
            Self::DebugExecution => "debug_execution",
            Self::SupportExportReplay => "support_export_replay",
            Self::DocsHelpEmbeds => "docs_help_embeds",
            Self::ReleaseProof => "release_proof",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TaskExecution => "Task execution",
            Self::TestExecution => "Test execution",
            Self::RequestExecution => "Request execution",
            Self::DatabaseExecution => "Database execution",
            Self::NotebookExecution => "Notebook execution",
            Self::PreviewExecution => "Preview execution",
            Self::AiExecution => "AI-mediated execution",
            Self::PublishExecution => "Publish execution",
            Self::DebugExecution => "Debug execution",
            Self::SupportExportReplay => "Support / export replay",
            Self::DocsHelpEmbeds => "Docs / help embeds",
            Self::ReleaseProof => "Release proof",
        }
    }
}

/// A reusable execution-lifecycle component group a surface consumes. Each group maps
/// to one or more frozen [`M5ExecutionComponentFamily`] and drives exactly one
/// certification truth axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionComponentGroup {
    /// The run/attempt-header component group.
    RunAttempt,
    /// The input-request-prompt component group.
    InputRequest,
    /// The artifact-publish-row component group.
    ArtifactPublish,
    /// The rerun-comparison-sheet component group.
    RerunReview,
    /// The debug-hierarchy component group (session header, thread/process tree,
    /// dump/crash artifact card).
    DebugHierarchy,
}

impl M5ExecutionComponentGroup {
    /// Every component group, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunAttempt,
        Self::InputRequest,
        Self::ArtifactPublish,
        Self::RerunReview,
        Self::DebugHierarchy,
    ];

    /// The frozen component families this group maps to.
    pub fn families(self) -> Vec<M5ExecutionComponentFamily> {
        match self {
            Self::RunAttempt => vec![M5ExecutionComponentFamily::RunAttemptHeader],
            Self::InputRequest => vec![M5ExecutionComponentFamily::InputRequestPrompt],
            Self::ArtifactPublish => vec![M5ExecutionComponentFamily::ArtifactPublishRow],
            Self::RerunReview => vec![M5ExecutionComponentFamily::RerunComparisonSheet],
            Self::DebugHierarchy => vec![
                M5ExecutionComponentFamily::DebugSessionHeader,
                M5ExecutionComponentFamily::ThreadProcessTree,
                M5ExecutionComponentFamily::DumpCrashArtifactCard,
            ],
        }
    }

    /// The frozen downgrade trigger a narrowing of this group binds to.
    pub const fn default_trigger(self) -> M5ExecutionDowngradeTrigger {
        match self {
            Self::RunAttempt => M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
            Self::InputRequest => M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
            Self::ArtifactPublish => M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
            Self::RerunReview => M5ExecutionDowngradeTrigger::RerunContextDrift,
            Self::DebugHierarchy => M5ExecutionDowngradeTrigger::ConnectorLost,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunAttempt => "run_attempt",
            Self::InputRequest => "input_request",
            Self::ArtifactPublish => "artifact_publish",
            Self::RerunReview => "rerun_review",
            Self::DebugHierarchy => "debug_hierarchy",
        }
    }
}

/// An execution path whose local/remote/container/managed/provider-backed parity the
/// certification captures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionPathClass {
    /// Executes on the local machine.
    Local,
    /// Executes on a remote host.
    Remote,
    /// Executes inside a container / devcontainer.
    Container,
    /// Executes in a managed / hosted environment.
    Managed,
    /// Executes on a provider-backed control plane (agent / third-party runner).
    ProviderBacked,
}

impl M5ExecutionPathClass {
    /// Every execution path class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Local,
        Self::Remote,
        Self::Container,
        Self::Managed,
        Self::ProviderBacked,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Managed => "managed",
            Self::ProviderBacked => "provider_backed",
        }
    }
}

/// Whether an execution path's parity is current, disclosed-degraded, or unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionPathParityState {
    /// The path's component parity is current on this surface.
    Current,
    /// The path's parity is degraded but disclosed (forces a narrowed claim).
    DisclosedNarrowed,
    /// The path is unsupported on this surface (forces a blocked claim).
    Unsupported,
}

impl M5ExecutionPathParityState {
    /// Returns true when the path is current.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Returns true when the path carries a disclosed degradation.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Returns true when the path is not unsupported.
    pub const fn never_unsupported(self) -> bool {
        !matches!(self, Self::Unsupported)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Generates a gated per-group truth axis: certified / disclosed-narrowed / blocked,
/// plus `not_applicable` when the surface does not consume the group.
macro_rules! gated_truth_axis {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            /// The component group is certified: full component truth on this surface.
            Certified,
            /// The component group is reduced but disclosed (yellow).
            DisclosedNarrowed,
            /// The component group hides drift or over-claims (red).
            Blocked,
            /// The surface does not consume this component group.
            NotApplicable,
        }

        impl $name {
            /// Returns true when the axis never hides drift (is not blocked).
            pub const fn never_violates(self) -> bool {
                !matches!(self, Self::Blocked)
            }

            /// Returns true when the axis carries a disclosed reduction.
            pub const fn is_disclosed_reduction(self) -> bool {
                matches!(self, Self::DisclosedNarrowed)
            }

            /// Returns true when the surface does not consume this component group.
            pub const fn is_not_applicable(self) -> bool {
                matches!(self, Self::NotApplicable)
            }

            /// Stable token recorded in the row.
            pub const fn as_str(self) -> &'static str {
                match self {
                    Self::Certified => "certified",
                    Self::DisclosedNarrowed => "disclosed_narrowed",
                    Self::Blocked => "blocked",
                    Self::NotApplicable => "not_applicable",
                }
            }
        }
    };
}

gated_truth_axis!(
    RunAttemptTruthState,
    "Certification of run-versus-attempt identity and outcome-state truth on a surface."
);
gated_truth_axis!(
    InputRequestTruthState,
    "Certification of input-request timeout / approval consequence truth on a surface."
);
gated_truth_axis!(
    ArtifactPublishTruthState,
    "Certification of produced-artifact lineage and retention truth on a surface."
);
gated_truth_axis!(
    RerunReviewTruthState,
    "Certification of rerun exact-versus-current-context diff truth on a surface."
);
gated_truth_axis!(
    DebugHierarchyTruthState,
    "Certification of debug live-versus-captured hierarchy and boundary truth on a surface."
);

/// The always-applicable export-parity axis: the support / release export must
/// reconstruct the surface's certified truth without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimExportParityState {
    /// The export reconstructs the full certified truth.
    Certified,
    /// The export reconstructs a disclosed-partial projection (yellow).
    DisclosedPartial,
    /// The export drops truth or relies on a screenshot (red).
    Dropped,
}

impl ClaimExportParityState {
    /// Returns true when the export never drops truth.
    pub const fn never_violates(self) -> bool {
        !matches!(self, Self::Dropped)
    }

    /// Returns true when the export carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartial)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedPartial => "disclosed_partial",
            Self::Dropped => "dropped",
        }
    }
}

/// The reduction level of one component group's axis on a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisLevel {
    NotApplicable,
    Certified,
    Disclosed,
    Blocked,
}

/// One execution-path compatibility note for a certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPathCompatibility {
    /// The execution path class this note describes.
    pub path_class: M5ExecutionPathClass,
    /// The path's parity on this surface.
    pub parity: M5ExecutionPathParityState,
    /// A precise, non-generic note; required when the path is not current.
    #[serde(default)]
    pub note: String,
}

impl ExecutionPathCompatibility {
    /// Whether the note is well-formed: a current path may carry no note, but a
    /// degraded / unsupported path must carry a precise, non-generic explanation.
    pub fn is_well_formed(&self) -> bool {
        self.parity.is_current() || (!self.note.trim().is_empty() && !label_is_generic(&self.note))
    }
}

/// An honest interactive-claim auto-narrow block for a surface. When a consumed
/// component group's truth axis reduces, the surface's interactive claim lowers to the
/// permitted ceiling, names the binding group and frozen trigger, and preserves the
/// canonical component identity rather than inheriting a full-truth label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceClaimAutoNarrow {
    /// The interactive claim the surface is narrowed to.
    pub narrowed_to: M5ExecutionInteractiveClaim,
    /// The component group whose reduced axis bound the narrowing.
    pub binding_group: M5ExecutionComponentGroup,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ExecutionDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical component identity and boundary are preserved rather than
    /// dropped; must hold.
    pub preserves_component_identity: bool,
}

impl SurfaceClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves component identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_component_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// A named export field the certified support / release export preserves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionCertExportField {
    /// The claimed surface identity.
    SurfaceIdentity,
    /// The component groups the surface consumes.
    ConsumedGroups,
    /// The declared interactive claim.
    DeclaredClaim,
    /// The effective (post-narrowing) interactive claim.
    EffectiveClaim,
    /// The per-axis certification truth.
    PerAxisTruth,
    /// The execution-path compatibility notes.
    CompatibilityNotes,
    /// The narrowed-claim reason, when narrowed.
    NarrowedReason,
    /// The canonical certification bundle ref.
    CertificationBundleRef,
}

impl M5ExecutionCertExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SurfaceIdentity,
        Self::ConsumedGroups,
        Self::DeclaredClaim,
        Self::EffectiveClaim,
        Self::PerAxisTruth,
        Self::CompatibilityNotes,
        Self::NarrowedReason,
        Self::CertificationBundleRef,
    ];

    /// The mandatory subset every certified surface's export must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::SurfaceIdentity,
        Self::ConsumedGroups,
        Self::DeclaredClaim,
        Self::EffectiveClaim,
        Self::PerAxisTruth,
        Self::CertificationBundleRef,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceIdentity => "surface_identity",
            Self::ConsumedGroups => "consumed_groups",
            Self::DeclaredClaim => "declared_claim",
            Self::EffectiveClaim => "effective_claim",
            Self::PerAxisTruth => "per_axis_truth",
            Self::CompatibilityNotes => "compatibility_notes",
            Self::NarrowedReason => "narrowed_reason",
            Self::CertificationBundleRef => "certification_bundle_ref",
        }
    }
}

/// Copy / export parity for a certified surface: the same truth must be copyable as
/// text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl CertCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
    }
}

/// Derived certification status for an execution surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionSurfaceCertStatus {
    /// The surface passes the shared component packet with no narrowing (green).
    Certified,
    /// The surface auto-narrows its claim, honestly disclosed (yellow).
    NarrowedDisclosed,
    /// The surface hides drift, over-claims, or drops truth (red).
    Blocked,
}

impl M5ExecutionSurfaceCertStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Blocked => "blocked",
        }
    }
}

/// A certification row for one claimed M5 execution surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionSurfaceCertRow {
    /// Record kind; must equal [`EXECUTION_SURFACE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EXECUTION_SURFACE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed execution surface this row certifies.
    pub claimed_surface: M5ExecutionClaimedSurface,
    /// Ref to the frozen matrix this row certifies against.
    pub source_matrix_ref: String,
    /// Ref to the ONE canonical component bundle this surface cites.
    pub certification_bundle_ref: String,
    /// Opaque ref to the run / attempt / execution context this surface acts on.
    pub execution_context_ref: String,
    /// The component groups this surface consumes.
    #[serde(default)]
    pub consumed_groups: Vec<M5ExecutionComponentGroup>,
    /// The interactive claim the surface declares.
    pub declared_claim: M5ExecutionInteractiveClaim,
    /// The interactive claim the surface effectively asserts after narrowing.
    pub effective_claim: M5ExecutionInteractiveClaim,
    /// Run/attempt truth axis (`not_applicable` unless the run-attempt group is
    /// consumed).
    pub run_attempt_truth: RunAttemptTruthState,
    /// Input-request truth axis.
    pub input_request_truth: InputRequestTruthState,
    /// Artifact-publish truth axis.
    pub artifact_publish_truth: ArtifactPublishTruthState,
    /// Rerun-review truth axis.
    pub rerun_review_truth: RerunReviewTruthState,
    /// Debug-hierarchy truth axis.
    pub debug_hierarchy_truth: DebugHierarchyTruthState,
    /// The always-applicable export-parity axis.
    pub export_parity: ClaimExportParityState,
    /// Execution-path compatibility notes.
    #[serde(default)]
    pub compatibility_notes: Vec<ExecutionPathCompatibility>,
    /// The honest auto-narrow block, present only when the surface narrows its claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<SurfaceClaimAutoNarrow>,
    /// The copy / export parity of the certified surface.
    pub copy_export: CertCopyExportParity,
    /// The named export fields the certified export carries.
    #[serde(default)]
    pub export_fields: Vec<M5ExecutionCertExportField>,
    /// The required labels the certified surface preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ExecutionRequiredLabel>,
    /// The canonical component families the surface references (reused vocabulary).
    #[serde(default)]
    pub consumer_families: Vec<M5ExecutionComponentFamily>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ExecutionSurfaceCertRow {
    /// Returns true when this surface is a release-evidence surface.
    pub const fn is_evidence_surface(&self) -> bool {
        self.claimed_surface.is_evidence()
    }

    /// Whether the surface declares that it consumes the component group.
    pub fn consumes_group(&self, group: M5ExecutionComponentGroup) -> bool {
        self.consumed_groups.contains(&group)
    }

    /// The reduction level of one component group's truth axis.
    fn group_axis_level(&self, group: M5ExecutionComponentGroup) -> AxisLevel {
        let (na, disclosed, blocked) = match group {
            M5ExecutionComponentGroup::RunAttempt => (
                self.run_attempt_truth.is_not_applicable(),
                self.run_attempt_truth.is_disclosed_reduction(),
                !self.run_attempt_truth.never_violates(),
            ),
            M5ExecutionComponentGroup::InputRequest => (
                self.input_request_truth.is_not_applicable(),
                self.input_request_truth.is_disclosed_reduction(),
                !self.input_request_truth.never_violates(),
            ),
            M5ExecutionComponentGroup::ArtifactPublish => (
                self.artifact_publish_truth.is_not_applicable(),
                self.artifact_publish_truth.is_disclosed_reduction(),
                !self.artifact_publish_truth.never_violates(),
            ),
            M5ExecutionComponentGroup::RerunReview => (
                self.rerun_review_truth.is_not_applicable(),
                self.rerun_review_truth.is_disclosed_reduction(),
                !self.rerun_review_truth.never_violates(),
            ),
            M5ExecutionComponentGroup::DebugHierarchy => (
                self.debug_hierarchy_truth.is_not_applicable(),
                self.debug_hierarchy_truth.is_disclosed_reduction(),
                !self.debug_hierarchy_truth.never_violates(),
            ),
        };
        if na {
            AxisLevel::NotApplicable
        } else if blocked {
            AxisLevel::Blocked
        } else if disclosed {
            AxisLevel::Disclosed
        } else {
            AxisLevel::Certified
        }
    }

    /// AC3 invariant: each gated axis is `not_applicable` exactly when the surface
    /// does not consume its component group.
    pub fn axes_match_consumed_groups(&self) -> bool {
        M5ExecutionComponentGroup::ALL.iter().all(|&group| {
            let na = self.group_axis_level(group) == AxisLevel::NotApplicable;
            na != self.consumes_group(group)
        })
    }

    /// Whether any consumed-group axis is blocked, or the export axis dropped truth.
    pub fn any_axis_blocked(&self) -> bool {
        !self.export_parity.never_violates()
            || M5ExecutionComponentGroup::ALL
                .iter()
                .any(|&group| self.group_axis_level(group) == AxisLevel::Blocked)
    }

    /// Whether any consumed-group axis is disclosed, or the export axis is a disclosed
    /// partial.
    pub fn any_axis_disclosed(&self) -> bool {
        self.export_parity.is_disclosed_reduction()
            || M5ExecutionComponentGroup::ALL
                .iter()
                .any(|&group| self.group_axis_level(group) == AxisLevel::Disclosed)
    }

    /// The status implied purely by the truth axes.
    fn axis_status(&self) -> M5ExecutionSurfaceCertStatus {
        if self.any_axis_blocked() {
            M5ExecutionSurfaceCertStatus::Blocked
        } else if self.any_axis_disclosed() {
            M5ExecutionSurfaceCertStatus::NarrowedDisclosed
        } else {
            M5ExecutionSurfaceCertStatus::Certified
        }
    }

    /// The consumed component group (in canonical order) whose axis is reduced or
    /// blocked, i.e. the group that binds the narrowing.
    pub fn binding_group(&self) -> Option<M5ExecutionComponentGroup> {
        M5ExecutionComponentGroup::ALL
            .iter()
            .copied()
            .find(|&group| {
                matches!(
                    self.group_axis_level(group),
                    AxisLevel::Disclosed | AxisLevel::Blocked
                )
            })
    }

    /// Whether the effective claim is narrowed below the declared claim.
    pub fn claim_narrowed(&self) -> bool {
        self.effective_claim.capability_rank() < self.declared_claim.capability_rank()
    }

    /// AC1: a stale or degraded surface can no longer inherit a full-truth label. The
    /// effective claim never exceeds the declared claim; a certified surface asserts
    /// its declared claim with no narrow block; a narrowed surface carries an honest
    /// narrow block bound to a reduced consumed group with its frozen trigger.
    pub fn claim_is_honest(&self) -> bool {
        if self.effective_claim.capability_rank() > self.declared_claim.capability_rank() {
            return false;
        }
        match self.axis_status() {
            // Blocked rows are rejected by `status`; do not additionally constrain.
            M5ExecutionSurfaceCertStatus::Blocked => true,
            M5ExecutionSurfaceCertStatus::NarrowedDisclosed => {
                self.claim_narrowed()
                    && match (&self.claim_auto_narrow, self.binding_group()) {
                        (Some(narrow), Some(group)) => {
                            narrow.is_honest()
                                && narrow.narrowed_to == self.effective_claim
                                && narrow.binding_group == group
                                && narrow.trigger == group.default_trigger()
                                && self.consumes_group(group)
                        }
                        _ => false,
                    }
            }
            M5ExecutionSurfaceCertStatus::Certified => {
                !self.claim_narrowed() && self.claim_auto_narrow.is_none()
            }
        }
    }

    /// AC2: a path whose parity is not current forces the claim to narrow rather than
    /// inheriting a full-truth label.
    pub fn unsupported_paths_narrowed(&self) -> bool {
        let any_not_current = self
            .compatibility_notes
            .iter()
            .any(|c| !c.parity.is_current());
        !any_not_current || self.claim_narrowed()
    }

    /// Whether every compatibility note is well-formed and at least one path is
    /// covered.
    pub fn compatibility_notes_valid(&self) -> bool {
        !self.compatibility_notes.is_empty()
            && self.compatibility_notes.iter().all(|c| c.is_well_formed())
    }

    /// The export preserves the surface's certified truth without a screenshot and
    /// carries every mandatory export field.
    pub fn export_preserves_truth(&self) -> bool {
        self.export_parity.never_violates()
            && self.copy_export.is_complete()
            && M5ExecutionCertExportField::MANDATORY
                .iter()
                .all(|f| self.export_fields.contains(f))
    }

    /// AC3: the surface references every canonical component family of the groups it
    /// consumes, so its claim is anchored to the shared component family.
    pub fn references_canonical_families(&self) -> bool {
        !self.consumed_groups.is_empty()
            && self.consumed_groups.iter().all(|group| {
                group
                    .families()
                    .iter()
                    .all(|family| self.consumer_families.contains(family))
            })
    }

    /// Derived certification status.
    pub fn status(&self) -> M5ExecutionSurfaceCertStatus {
        if !self.claim_is_honest()
            || !self.export_preserves_truth()
            || !self.unsupported_paths_narrowed()
            || !self.compatibility_notes_valid()
            || !self.references_canonical_families()
            || !self.axes_match_consumed_groups()
        {
            return M5ExecutionSurfaceCertStatus::Blocked;
        }
        self.axis_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EXECUTION_SURFACE_CERT_ROW_RECORD_KIND
            && self.schema_version == EXECUTION_SURFACE_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && self.source_matrix_ref == EXECUTION_SURFACE_CERT_COMPONENT_MATRIX_REF
            && !self.certification_bundle_ref.trim().is_empty()
            && !self.execution_context_ref.trim().is_empty()
            && !self.consumed_groups.is_empty()
            && !self.export_fields.is_empty()
            && self.required_labels.len() >= M5ExecutionRequiredLabel::MANDATORY.len()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} run_attempt={run} input={input} artifact={artifact} \
rerun={rerun} debug={debug} export={export} declared={declared} effective={effective} \
status={status}",
            surface = self.claimed_surface.as_str(),
            run = self.run_attempt_truth.as_str(),
            input = self.input_request_truth.as_str(),
            artifact = self.artifact_publish_truth.as_str(),
            rerun = self.rerun_review_truth.as_str(),
            debug = self.debug_hierarchy_truth.as_str(),
            export = self.export_parity.as_str(),
            declared = self.declared_claim.as_str(),
            effective = self.effective_claim.as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-827 execution surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionSurfaceCertSummary {
    pub surface_count: usize,
    pub evidence_surface_count: usize,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub consumed_group_count: usize,
    pub path_class_count: usize,
    pub all_claims_honest: bool,
    pub all_export_preserve_truth: bool,
    pub all_unsupported_paths_narrowed: bool,
    pub group_coverage_complete: bool,
    pub path_class_coverage_complete: bool,
}

/// Constructor input for [`ExecutionSurfaceCertPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSurfaceCertPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    pub rows: Vec<ExecutionSurfaceCertRow>,
}

/// Checked-in M05-827 execution surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionSurfaceCertPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ExecutionSurfaceCertRow>,
    pub summary: ExecutionSurfaceCertSummary,
}

impl ExecutionSurfaceCertPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: ExecutionSurfaceCertPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EXECUTION_SURFACE_CERT_SCHEMA_VERSION,
            record_kind: EXECUTION_SURFACE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            certification_bundle_ref: input.certification_bundle_ref,
            rows: input.rows,
            summary: ExecutionSurfaceCertSummary {
                surface_count: 0,
                evidence_surface_count: 0,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                consumed_group_count: 0,
                path_class_count: 0,
                all_claims_honest: false,
                all_export_preserve_truth: false,
                all_unsupported_paths_narrowed: false,
                group_coverage_complete: false,
                path_class_coverage_complete: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5ExecutionClaimedSurface> {
        self.rows.iter().map(|r| r.claimed_surface).collect()
    }

    /// Component groups consumed by some row.
    pub fn consumed_groups(&self) -> BTreeSet<M5ExecutionComponentGroup> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_groups.iter().copied())
            .collect()
    }

    /// Execution path classes covered by some row's compatibility notes.
    pub fn covered_path_classes(&self) -> BTreeSet<M5ExecutionPathClass> {
        self.rows
            .iter()
            .flat_map(|r| r.compatibility_notes.iter().map(|c| c.path_class))
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ExecutionSurfaceCertSummary {
        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                M5ExecutionSurfaceCertStatus::Certified => green += 1,
                M5ExecutionSurfaceCertStatus::NarrowedDisclosed => yellow += 1,
                M5ExecutionSurfaceCertStatus::Blocked => red += 1,
            }
        }
        let consumed = self.consumed_groups();
        let paths = self.covered_path_classes();

        ExecutionSurfaceCertSummary {
            surface_count: self.rows.len(),
            evidence_surface_count: self
                .rows
                .iter()
                .filter(|r| r.is_evidence_surface())
                .count(),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            consumed_group_count: consumed.len(),
            path_class_count: paths.len(),
            all_claims_honest: self.rows.iter().all(ExecutionSurfaceCertRow::claim_is_honest),
            all_export_preserve_truth: self
                .rows
                .iter()
                .all(ExecutionSurfaceCertRow::export_preserves_truth),
            all_unsupported_paths_narrowed: self
                .rows
                .iter()
                .all(ExecutionSurfaceCertRow::unsupported_paths_narrowed),
            group_coverage_complete: M5ExecutionComponentGroup::ALL
                .iter()
                .all(|g| consumed.contains(g)),
            path_class_coverage_complete: M5ExecutionPathClass::ALL
                .iter()
                .all(|p| paths.contains(p)),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ExecutionSurfaceCertViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EXECUTION_SURFACE_CERT_SCHEMA_VERSION {
            violations.push(ExecutionSurfaceCertViolation::SchemaVersion {
                expected: EXECUTION_SURFACE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXECUTION_SURFACE_CERT_RECORD_KIND {
            violations.push(ExecutionSurfaceCertViolation::RecordKind {
                expected: EXECUTION_SURFACE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
            || self.certification_bundle_ref.trim().is_empty()
        {
            violations.push(ExecutionSurfaceCertViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ExecutionSurfaceCertViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_surfaces.insert(row.claimed_surface);

            if !row.is_complete() {
                violations.push(ExecutionSurfaceCertViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Every surface cites the ONE canonical component bundle.
            if row.certification_bundle_ref != self.certification_bundle_ref {
                violations.push(ExecutionSurfaceCertViolation::BundleRefMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC3: each gated axis is applicable exactly when its group is consumed.
            if !row.axes_match_consumed_groups() {
                violations.push(ExecutionSurfaceCertViolation::AxisApplicabilityMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: the claim never over-asserts control for a reduced surface.
            if !row.claim_is_honest() {
                violations.push(ExecutionSurfaceCertViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: unsupported / degraded paths force a narrowed claim.
            if !row.unsupported_paths_narrowed() {
                violations.push(ExecutionSurfaceCertViolation::UnsupportedPathNotNarrowed {
                    id: row.row_id.clone(),
                });
            }
            if !row.compatibility_notes_valid() {
                violations.push(ExecutionSurfaceCertViolation::CompatibilityNoteMalformed {
                    id: row.row_id.clone(),
                });
            }

            // AC2: export preserves truth without a screenshot.
            if !row.export_preserves_truth() {
                violations.push(ExecutionSurfaceCertViolation::ExportDropsTruth {
                    id: row.row_id.clone(),
                });
            }

            // AC3: anchored to the canonical component family.
            if !row.references_canonical_families() {
                violations.push(ExecutionSurfaceCertViolation::NotAnchoredToCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // No blocked (red) surface may ship.
            if row.status() == M5ExecutionSurfaceCertStatus::Blocked {
                violations.push(ExecutionSurfaceCertViolation::BlockedSurface {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every claimed surface is certified at least once.
        for surface in M5ExecutionClaimedSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations
                    .push(ExecutionSurfaceCertViolation::MissingSurfaceCoverage { surface });
            }
        }

        // Coverage: every release-evidence surface is present.
        for surface in M5ExecutionClaimedSurface::EVIDENCE_SURFACES {
            if !seen_surfaces.contains(&surface) {
                violations
                    .push(ExecutionSurfaceCertViolation::MissingEvidenceSurface { surface });
            }
        }

        // Coverage: every component group is consumed somewhere.
        let consumed = self.consumed_groups();
        for group in M5ExecutionComponentGroup::ALL {
            if !consumed.contains(&group) {
                violations.push(ExecutionSurfaceCertViolation::MissingGroupCoverage { group });
            }
        }

        // Coverage: every execution path class is exercised somewhere.
        let paths = self.covered_path_classes();
        for path in M5ExecutionPathClass::ALL {
            if !paths.contains(&path) {
                violations.push(ExecutionSurfaceCertViolation::MissingPathClassCoverage { path });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ExecutionSurfaceCertViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("execution surface certification packet serializes"),
        ) {
            violations.push(ExecutionSurfaceCertViolation::RawBoundaryMaterialInExport);
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
            .expect("execution surface certification packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,claimed_surface,run_attempt,input_request,artifact_publish,rerun_review,debug_hierarchy,export_parity,declared_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{surface},{run},{input},{artifact},{rerun},{debug},{export},{declared},{effective},{status}\n",
                id = row.row_id,
                surface = row.claimed_surface.as_str(),
                run = row.run_attempt_truth.as_str(),
                input = row.input_request_truth.as_str(),
                artifact = row.artifact_publish_truth.as_str(),
                rerun = row.rerun_review_truth.as_str(),
                debug = row.debug_hierarchy_truth.as_str(),
                export = row.export_parity.as_str(),
                declared = row.declared_claim.as_str(),
                effective = row.effective_claim.as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Execution-Lifecycle Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Bundle: `{}`\n", self.certification_bundle_ref));
        out.push_str(&format!(
            "- Surfaces: {} certified across {} / {} claimed surfaces\n",
            self.summary.surface_count,
            self.represented_surfaces().len(),
            M5ExecutionClaimedSurface::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.claimed_surface.label(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_auto_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (group={}, trigger={}) — {}\n",
                    row.declared_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_group.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in execution surface certification export.
pub fn current_m5_execution_surface_cert_export(
) -> Result<ExecutionSurfaceCertPacket, ExecutionSurfaceCertArtifactError> {
    let packet: ExecutionSurfaceCertPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-execution-lifecycle-surface-certification-proof/support_export.json"
    )))
    .map_err(ExecutionSurfaceCertArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExecutionSurfaceCertArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in execution surface certification export.
#[derive(Debug)]
pub enum ExecutionSurfaceCertArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExecutionSurfaceCertViolation>),
}

impl fmt::Display for ExecutionSurfaceCertArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "execution surface certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "execution surface certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ExecutionSurfaceCertArtifactError {}

/// Validation failure for M05-827 execution surface certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionSurfaceCertViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    BundleRefMismatch { id: String },
    AxisApplicabilityMismatch { id: String },
    ClaimOverAsserted { id: String },
    UnsupportedPathNotNarrowed { id: String },
    CompatibilityNoteMalformed { id: String },
    ExportDropsTruth { id: String },
    NotAnchoredToCanonicalFamily { id: String },
    BlockedSurface { id: String },
    MissingSurfaceCoverage { surface: M5ExecutionClaimedSurface },
    MissingEvidenceSurface { surface: M5ExecutionClaimedSurface },
    MissingGroupCoverage { group: M5ExecutionComponentGroup },
    MissingPathClassCoverage { path: M5ExecutionPathClass },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for ExecutionSurfaceCertViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::BundleRefMismatch { id } => {
                write!(f, "row {id} does not cite the packet's canonical bundle ref")
            }
            Self::AxisApplicabilityMismatch { id } => {
                write!(
                    f,
                    "row {id} has a truth axis that is applicable without its component group (or vice versa)"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts interactive control for a reduced surface, or narrows spuriously"
                )
            }
            Self::UnsupportedPathNotNarrowed { id } => {
                write!(
                    f,
                    "row {id} has a non-current execution path but does not narrow its claim"
                )
            }
            Self::CompatibilityNoteMalformed { id } => {
                write!(f, "row {id} has a missing or generic execution-path compatibility note")
            }
            Self::ExportDropsTruth { id } => {
                write!(f, "row {id} export cannot preserve certified truth without a screenshot")
            }
            Self::NotAnchoredToCanonicalFamily { id } => {
                write!(f, "row {id} does not reference the canonical families of its consumed groups")
            }
            Self::BlockedSurface { id } => {
                write!(f, "row {id} is blocked (red) and may not ship")
            }
            Self::MissingSurfaceCoverage { surface } => {
                write!(f, "claimed surface {surface:?} is not certified in the packet")
            }
            Self::MissingEvidenceSurface { surface } => {
                write!(f, "release-evidence surface {surface:?} is missing")
            }
            Self::MissingGroupCoverage { group } => {
                write!(f, "component group {} is not consumed in the packet", group.as_str())
            }
            Self::MissingPathClassCoverage { path } => {
                write!(f, "execution path class {} is not exercised in the packet", path.as_str())
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for ExecutionSurfaceCertViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in execution surface certification packet. This is
/// the one source of truth shared by the tests, the example dump, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_execution_surface_cert_packet() -> ExecutionSurfaceCertPacket {
    ExecutionSurfaceCertPacket::new(ExecutionSurfaceCertPacketInput {
        packet_id: "m5-execution-lifecycle-surface-certification:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: EXECUTION_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: EXECUTION_SURFACE_CERT_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

include!("seed.rs");
