//! Implemented M5 acquisition-evidence and partial-recovery registries.
//!
//! The frozen [repository-bootstrap matrix][matrix] names Aureline's five project-entry acquisition families
//! and locks their controlled vocabulary. This is the evidence-packet + resumable-acquisition implement lane: it
//! turns the *acquisition-evidence* grammar (how Aureline records the clone / fetch transcript, the warnings and
//! retries, the resulting root identity, the omitted-or-unfetched state, and the current bootstrap checkpoint of
//! an acquisition path) and the *partial-recovery* grammar (typed recovery actions that resume an interrupted
//! acquisition, discard partial state, open the partial root read-only, or merely report status) into registry
//! resolvers that produce export-safe, honest projections. Every claimed M5 acquisition path then resolves to one
//! stable acquisition-evidence packet — the evidence kind and canonical evidence mode, the clone / fetch
//! transcript reference, the warnings-and-retries reference, the resulting-root-identity reference, the
//! omitted-or-unfetched reference, the bootstrap-checkpoint reference, and the evidence provenance — and to one
//! partial-recovery object — the recovery-action kind, the recovery site, the state consequence, the lineage
//! consequence, the explicit-action requirement, and the attribution reference — that the acquisition, git,
//! trust, diagnostics, CLI, and support / export surfaces can inspect without manual reconstruction, so a partial
//! or interrupted acquisition stays visible and recoverable instead of reading as missing or unsupported data, a
//! recovery action never discards partial state or transcript lineage without an explicit discard or cleanup
//! action, partial content is never presented as a healthy full checkout, every recovery row identifies exactly
//! what the action would do, where it would run, and what state or lineage effect it carries, and an acquisition
//! path that cannot explain its evidence or its recovery choices degrades honestly instead of reading as a clean
//! pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable acquisition-evidence packet per acquisition path.**
//!   [`resolve_acquisition_evidence_entry`] refuses to read as a clean, registry-bound evidence entry unless it
//!   names a canonical registry token, a classified [evidence kind][M5AcquisitionEvidenceKind], a
//!   repository-bootstrap role, covers every [resolution form][M5RecoveryResolutionForm] (the canonical object,
//!   the accessible summary, and the audit record), publishes every evidence field (clone / fetch transcript
//!   reference, warnings-and-retries reference, resulting-root-identity reference, omitted-or-unfetched reference,
//!   bootstrap-checkpoint reference, and evidence provenance), keeps a partial or interrupted acquisition visible,
//!   and discloses partial-not-full status before any evidence kind that describes partial state; otherwise it
//!   degrades.
//! * **Keep the evidence from presenting partial content as a healthy full checkout.**
//!   [`acquisition_evidence_discloses_partial_state`] rejects an evidence packet that would hide a partial or
//!   interrupted acquisition or present it as a healthy full checkout before the partial-not-full status is
//!   disclosed, so it degrades to
//!   [`M5AcquisitionEvidenceEntryDegradeReason::EvidenceOverclaimsFullCheckoutOrHidesPartialState`].
//! * **Keep the partial recovery from discarding state or lineage without an explicit action.**
//!   [`resolve_partial_recovery_entry`] names a classified [recovery class][M5PartialRecoveryClass], requires the
//!   full recovery-action-kind / recovery-site / state-consequence / lineage-consequence / explicit-action-
//!   requirement / attribution partial-recovery object, covers every resolution form, and degrades to
//!   [`M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction`] when a
//!   state-mutating action would discard partial state during acquisition, run ungated without an explicit discard
//!   or cleanup action, or hide what it would do and where, so a recovery action can never read as safe when it
//!   has quietly discarded partial state or transcript lineage merely because an acquisition was interrupted.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5RepositoryBootstrapRole`] role vocabulary
//! and the [`M5RepositoryBootstrapConsumerSurface`] consumer-surface taxonomy — so the acquisition, shell, git,
//! trust, diagnostics, docs, CLI, and support surfaces can never fork their own evidence or recovery meaning. Raw
//! secret values, tokens, and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_repository_bootstrap_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_acquisition_evidence_and_partial_recovery_registries,
    seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed,
    seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed,
    M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_repository_bootstrap_matrix::{
    M5RepositoryBootstrapAccessibilityRoute, M5RepositoryBootstrapConsumerSurface,
    M5RepositoryBootstrapDeploymentLine, M5RepositoryBootstrapDowngradeTrigger,
    M5RepositoryBootstrapFamily, M5RepositoryBootstrapQualificationClass,
    M5RepositoryBootstrapRequiredLabel, M5RepositoryBootstrapRole,
    M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF, M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF, M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5AcquisitionEvidencePartialRecoveryRegistriesPacket`].
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_acquisition_evidence_and_partial_recovery_registries";

/// Schema version for M5 acquisition-evidence / partial-recovery registry records.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/workspaces/m5-acquisition-evidence-and-partial-recovery-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_DOC_REF: &str =
    "docs/workspaces/m5_acquisition_evidence_and_partial_recovery_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-acquisition-evidence-and-partial-recovery-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-acquisition-evidence-and-partial-recovery-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-acquisition-evidence-and-partial-recovery-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-acquisition-evidence-and-partial-recovery-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5AcquisitionEvidencePartialRecoveryRegistriesConsumerSurface =
    M5RepositoryBootstrapConsumerSurface;

/// One of the three resolution forms every acquisition-evidence or partial-recovery entry must hold across so its
/// truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the evidence-packet
/// and resumable-acquisition *roles* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoveryResolutionForm {
    /// The canonical resolved acquisition-evidence / partial-recovery object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved evidence / recovery discoverable without
    /// visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved evidence / recovery inspectable off-renderer.
    AuditRecord,
}

impl M5RecoveryResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
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

/// Controlled acquisition-evidence kind an evidence entry resolves, so the canonical evidence model shares one
/// registry rather than a hand-copied per-entry assumption. Minted by this lane because the frozen matrix carries
/// the acquisition families but not the concrete clone-fetch-transcript / warnings-and-retries /
/// resulting-root-identity / omitted-or-unfetched-state / bootstrap-checkpoint facet a path records. Every
/// classified kind carries its canonical evidence mode; the two partial-describing kinds carry
/// omitted-or-interrupted consequence and so must disclose partial-not-full status before they may present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AcquisitionEvidenceKind {
    /// The clone / fetch transcript reference for the acquisition (read-only record).
    CloneFetchTranscript,
    /// The warnings and retries observed during acquisition (read-only record).
    WarningsAndRetries,
    /// The resulting root identity computed after acquisition (read-only record).
    ResultingRootIdentity,
    /// The omitted-or-unfetched state left partial by an interrupted acquisition (describes partial state).
    OmittedOrUnfetchedState,
    /// The current bootstrap checkpoint an interrupted acquisition can resume from (describes partial state).
    BootstrapCheckpoint,
    /// The evidence kind is unclassified, which is disallowed.
    EvidenceUnclassified,
}

impl M5AcquisitionEvidenceKind {
    /// Every evidence kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CloneFetchTranscript,
        Self::WarningsAndRetries,
        Self::ResultingRootIdentity,
        Self::OmittedOrUnfetchedState,
        Self::BootstrapCheckpoint,
        Self::EvidenceUnclassified,
    ];

    /// The five canonical evidence kinds every claimed M5 acquisition path records.
    pub const CANONICAL_KINDS: [Self; 5] = [
        Self::CloneFetchTranscript,
        Self::WarningsAndRetries,
        Self::ResultingRootIdentity,
        Self::OmittedOrUnfetchedState,
        Self::BootstrapCheckpoint,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CloneFetchTranscript => "clone_fetch_transcript",
            Self::WarningsAndRetries => "warnings_and_retries",
            Self::ResultingRootIdentity => "resulting_root_identity",
            Self::OmittedOrUnfetchedState => "omitted_or_unfetched_state",
            Self::BootstrapCheckpoint => "bootstrap_checkpoint",
            Self::EvidenceUnclassified => "evidence_unclassified",
        }
    }

    /// Whether the kind is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::EvidenceUnclassified)
    }

    /// The canonical evidence mode for this kind.
    pub const fn canonical_evidence_mode(self) -> &'static str {
        match self {
            Self::CloneFetchTranscript => "clone_fetch_transcript_evidence",
            Self::WarningsAndRetries => "warnings_and_retries_evidence",
            Self::ResultingRootIdentity => "resulting_root_identity_evidence",
            Self::OmittedOrUnfetchedState => "omitted_or_unfetched_state_evidence",
            Self::BootstrapCheckpoint => "bootstrap_checkpoint_evidence",
            Self::EvidenceUnclassified => "",
        }
    }

    /// Whether this evidence kind describes a partial or interrupted acquisition and so must disclose
    /// partial-not-full status before it may present, never reading as a healthy full checkout.
    pub const fn describes_partial_or_interrupted_state(self) -> bool {
        matches!(
            self,
            Self::OmittedOrUnfetchedState | Self::BootstrapCheckpoint
        )
    }
}

/// Controlled partial-recovery class a recovery entry must resolve its action from, so a recovery action shares
/// one registry rather than a hand-copied per-entry action. Minted by this lane, tracking the resume-acquisition /
/// discard-partial-state / open-read-only-partial-root / inert-status-report action classes the implementation
/// requirement differentiates by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PartialRecoveryClass {
    /// The action resumes an interrupted clone / fetch / bootstrap from the recorded checkpoint (mutates state).
    ResumeAcquisition,
    /// The action discards partial state and transcript lineage under an explicit cleanup (mutates state).
    DiscardPartialState,
    /// The action opens the partial root read-only without pretending it is a healthy full checkout (read-only).
    OpenReadOnlyPartialRoot,
    /// The action is an inert status report (presents the partial state only; changes nothing).
    InertStatusReport,
    /// The partial-recovery class is unclassified, which is disallowed.
    RecoveryUnclassified,
}

impl M5PartialRecoveryClass {
    /// Every partial-recovery class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ResumeAcquisition,
        Self::DiscardPartialState,
        Self::OpenReadOnlyPartialRoot,
        Self::InertStatusReport,
        Self::RecoveryUnclassified,
    ];

    /// The four canonical classes every recovery action must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 4] = [
        Self::ResumeAcquisition,
        Self::DiscardPartialState,
        Self::OpenReadOnlyPartialRoot,
        Self::InertStatusReport,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumeAcquisition => "resume_acquisition",
            Self::DiscardPartialState => "discard_partial_state",
            Self::OpenReadOnlyPartialRoot => "open_read_only_partial_root",
            Self::InertStatusReport => "inert_status_report",
            Self::RecoveryUnclassified => "recovery_unclassified",
        }
    }

    /// Whether the partial-recovery class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::RecoveryUnclassified)
    }

    /// Whether this class is a state-mutating action that resumes acquisition or discards partial state, and so
    /// must require an explicit discard or cleanup action before it may run. Opening the partial root read-only or
    /// reporting inert status is not state-mutating.
    pub const fn mutates_or_discards_state(self) -> bool {
        matches!(self, Self::ResumeAcquisition | Self::DiscardPartialState)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so an acquisition-evidence or
/// partial-recovery token's meaning stays stable whether it appears in the shell, entry, diagnostics, admin, or a
/// support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoverySurfaceContext {
    /// The shell surface.
    ShellSurface,
    /// The project-entry surface.
    EntrySurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5RecoverySurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellSurface,
        Self::EntrySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ShellSurface,
        Self::EntrySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellSurface => "shell_surface",
            Self::EntrySurface => "entry_surface",
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

/// One mandatory rendered part an acquisition-evidence or partial-recovery entry must be able to show, so no
/// evidence kind, recovery action, partial-visibility fact, explicit-action gate, consequence, or registry fact
/// is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoveryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The evidence kind the entry resolves (acquisition-evidence entry).
    EvidenceKind,
    /// The transcript, warnings-and-retries, root-identity, omitted-or-unfetched, and checkpoint fields the entry
    /// publishes (acquisition-evidence entry).
    EvidencePacketFields,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The partial-state-visibility and partial-not-full-disclosure facts the entry publishes (acquisition-
    /// evidence entry).
    PartialStateVisibilityAndDisclosure,
    /// The partial-recovery fields (recovery class, recovery site, state / lineage consequence, explicit-action
    /// requirement) the entry publishes (partial-recovery entry).
    PartialRecoveryFields,
    /// The attribution reference the entry publishes (partial-recovery entry).
    RecoveryAttributionHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved acquisition evidence or partial recovery (both entries).
    PlainLanguageMeaning,
}

impl M5RecoveryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::EvidenceKind,
        Self::EvidencePacketFields,
        Self::ResolutionFormCoverage,
        Self::PartialStateVisibilityAndDisclosure,
        Self::PartialRecoveryFields,
        Self::RecoveryAttributionHint,
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
            Self::EvidenceKind => "evidence_kind",
            Self::EvidencePacketFields => "evidence_packet_fields",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::PartialStateVisibilityAndDisclosure => "partial_state_visibility_and_disclosure",
            Self::PartialRecoveryFields => "partial_recovery_fields",
            Self::RecoveryAttributionHint => "recovery_attribution_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// acquisition evidence, a partial recovery, or a degraded acquisition-evidence / partial-recovery entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoveryNextAction {
    /// Expand the resolved evidence's or recovery's plain-language meaning.
    ExpandRecoveryMeaning,
    /// Inspect the evidence kind or recovery class the entry resolves.
    InspectEvidenceOrRecovery,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5RecoveryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRecoveryMeaning,
        Self::InspectEvidenceOrRecovery,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRecoveryMeaning => "expand_recovery_meaning",
            Self::InspectEvidenceOrRecovery => "inspect_evidence_or_recovery",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoveryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The repository-bootstrap families covered.
    RepositoryBootstrapFamilies,
    /// The evidence kinds carried.
    EvidenceKinds,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The partial-recovery classes carried.
    PartialRecoveryClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The evidence modes carried.
    EvidenceModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RecoveryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::EvidenceKinds,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::PartialRecoveryClasses,
        Self::SurfaceContext,
        Self::EvidenceModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::EvidenceKinds,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::RepositoryBootstrapFamilies => "repository_bootstrap_families",
            Self::EvidenceKinds => "evidence_kinds",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::PartialRecoveryClasses => "partial_recovery_classes",
            Self::SurfaceContext => "surface_context",
            Self::EvidenceModes => "evidence_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an acquisition-evidence entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, overclaiming, field-incomplete, or form-incomplete
/// entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AcquisitionEvidenceEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the evidence means.
    EvidenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence kind is unclassified (not in the resolved taxonomy).
    EvidenceKindUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    EvidenceNotBoundToRegistry,
    /// The resolved acquisition-evidence packet is incomplete: the clone / fetch transcript reference,
    /// warnings-and-retries reference, resulting-root-identity reference, omitted-or-unfetched reference,
    /// bootstrap-checkpoint reference, or evidence provenance is unstated.
    EvidencePacketIncomplete,
    /// The evidence packet would hide a partial or interrupted acquisition or present it as a healthy full
    /// checkout before the partial-not-full status is disclosed.
    EvidenceOverclaimsFullCheckoutOrHidesPartialState,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A partial-describing evidence packet presented partial content as a healthy full checkout without
    /// disclosing partial-not-full status.
    PartialStatePresentedAsHealthyFullCheckout,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AcquisitionEvidenceEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::EvidenceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::EvidenceKindUnclassified,
        Self::EvidenceNotBoundToRegistry,
        Self::EvidencePacketIncomplete,
        Self::EvidenceOverclaimsFullCheckoutOrHidesPartialState,
        Self::ResolutionFormCoverageIncomplete,
        Self::PartialStatePresentedAsHealthyFullCheckout,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceTokenUnstated => "evidence_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::EvidenceKindUnclassified => "evidence_kind_unclassified",
            Self::EvidenceNotBoundToRegistry => "evidence_not_bound_to_registry",
            Self::EvidencePacketIncomplete => "evidence_packet_incomplete",
            Self::EvidenceOverclaimsFullCheckoutOrHidesPartialState => {
                "evidence_overclaims_full_checkout_or_hides_partial_state"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::PartialStatePresentedAsHealthyFullCheckout => {
                "partial_state_presented_as_healthy_full_checkout"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RecoveryNextAction {
        match self {
            Self::EvidenceTokenUnstated | Self::EvidenceNotBoundToRegistry => {
                M5RecoveryNextAction::TraceCanonicalRegistry
            }
            Self::EvidenceKindUnclassified
            | Self::EvidencePacketIncomplete
            | Self::EvidenceOverclaimsFullCheckoutOrHidesPartialState => {
                M5RecoveryNextAction::InspectEvidenceOrRecovery
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5RecoveryNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::PartialStatePresentedAsHealthyFullCheckout
            | Self::ProofStale => M5RecoveryNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::EvidenceKindUnclassified | Self::EvidencePacketIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::StagedTrustRuleUnstated
            }
            Self::EvidenceNotBoundToRegistry => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::EvidenceOverclaimsFullCheckoutOrHidesPartialState
            | Self::PartialStatePresentedAsHealthyFullCheckout => {
                M5RepositoryBootstrapDowngradeTrigger::RanRepoOwnedActionsImplicitlyDuringAcquisition
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a partial-recovery entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PartialRecoveryEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RecoveryTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The partial-recovery class is unclassified (not in the resolved taxonomy).
    PartialRecoveryClassUnclassified,
    /// A state-mutating recovery action would discard partial state during acquisition, run ungated without an
    /// explicit discard or cleanup action, or hide what it would do, where it would run, and its state / lineage
    /// consequence, or it dropped one of the required recovery-action fields (recovery action kind, recovery site,
    /// state consequence, lineage consequence, explicit-action requirement, attribution).
    PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction,
    /// The canonical / accessible / audit resolution-form coverage of the recovery action is incomplete.
    RecoveryFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PartialRecoveryEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RecoveryTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::PartialRecoveryClassUnclassified,
        Self::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction,
        Self::RecoveryFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoveryTokenUnstated => "recovery_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PartialRecoveryClassUnclassified => "partial_recovery_class_unclassified",
            Self::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction => {
                "partial_recovery_discards_state_or_lineage_without_explicit_action"
            }
            Self::RecoveryFormCoverageIncomplete => "recovery_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RecoveryNextAction {
        match self {
            Self::RecoveryTokenUnstated => M5RecoveryNextAction::TraceCanonicalRegistry,
            Self::PartialRecoveryClassUnclassified
            | Self::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction => {
                M5RecoveryNextAction::InspectEvidenceOrRecovery
            }
            Self::RecoveryFormCoverageIncomplete => {
                M5RecoveryNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RecoveryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::RecoveryTokenUnstated => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::PartialRecoveryClassUnclassified => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction => {
                M5RepositoryBootstrapDowngradeTrigger::RanRepoOwnedActionsImplicitlyDuringAcquisition
            }
            Self::RecoveryFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_acquisition_evidence_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AcquisitionEvidenceEntryResolutionInput {
    /// Stable identity of the acquisition-evidence-registry entry.
    pub entry_id: String,
    /// The stable acquisition-path ID this evidence binds to (e.g. `entry.acme.clone-remote`); empty means
    /// unstated.
    pub acquisition_path_id: String,
    /// The canonical registry token name (e.g. `acquisition.evidence.clone_fetch_transcript`); empty means
    /// unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The evidence kind this entry resolves.
    pub evidence_kind: M5AcquisitionEvidenceKind,
    /// The render / surface context.
    pub surface_context: M5RecoverySurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RecoveryResolutionForm>,
    /// The published clone / fetch transcript reference; empty means unstated.
    pub transcript_ref: String,
    /// The published warnings-and-retries reference; empty means unstated.
    pub warnings_and_retries_ref: String,
    /// The published resulting-root-identity reference; empty means unstated.
    pub resulting_root_identity_ref: String,
    /// The published omitted-or-unfetched reference; empty means unstated.
    pub omitted_or_unfetched_ref: String,
    /// The published bootstrap-checkpoint reference; empty means unstated.
    pub bootstrap_checkpoint_ref: String,
    /// The published evidence provenance; empty means unstated.
    pub evidence_provenance: String,
    /// True when the behavior traces to the acquisition-evidence registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when a partial or interrupted acquisition stays visible rather than reading as missing or unsupported
    /// data (a hard invariant when `false`).
    pub partial_state_visible: bool,
    /// True when this evidence kind describes a partial or interrupted acquisition.
    pub describes_partial_state: bool,
    /// True when partial-not-full status is disclosed before a partial-describing packet presents, so partial
    /// content is never presented as a healthy full checkout.
    pub partial_not_full_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe acquisition-evidence-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAcquisitionEvidenceEntry {
    /// Stable identity of the acquisition-evidence-registry entry.
    pub entry_id: String,
    /// The stable acquisition-path ID this evidence binds to.
    pub acquisition_path_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The evidence-kind token named by the entry.
    pub evidence_kind: String,
    /// Whether the evidence kind is classified into the resolved taxonomy.
    pub evidence_kind_is_classified: bool,
    /// The canonical evidence mode for the entry's kind.
    pub canonical_evidence_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published clone / fetch transcript reference.
    pub transcript_ref: String,
    /// The published warnings-and-retries reference.
    pub warnings_and_retries_ref: String,
    /// The published resulting-root-identity reference.
    pub resulting_root_identity_ref: String,
    /// The published omitted-or-unfetched reference.
    pub omitted_or_unfetched_ref: String,
    /// The published bootstrap-checkpoint reference.
    pub bootstrap_checkpoint_ref: String,
    /// The published evidence provenance.
    pub evidence_provenance: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved acquisition-evidence packet publishes every required field.
    pub evidence_packet_complete: bool,
    /// Whether the entry traces to the acquisition-evidence registry.
    pub bound_to_registry: bool,
    /// Whether a partial or interrupted acquisition stays visible.
    pub partial_state_visible: bool,
    /// Whether this evidence kind describes a partial or interrupted acquisition.
    pub describes_partial_state: bool,
    /// Whether partial-not-full status is disclosed before a partial-describing packet presents.
    pub partial_not_full_disclosed: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5AcquisitionEvidenceEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RecoveryNextAction,
    /// Whether the evidence resolves to one stable packet across every claimed acquisition path (clean entry
    /// naming every fact).
    pub evidence_resolves_across_entry_flows: bool,
}

impl M5ResolvedAcquisitionEvidenceEntry {
    /// Whether this acquisition-evidence entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_partial_recovery_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PartialRecoveryEntryResolutionInput {
    /// Stable identity of the partial-recovery entry.
    pub entry_id: String,
    /// The stable source-ref this recovery action binds to; empty means unstated.
    pub source_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The partial-recovery class this entry must resolve its action from.
    pub recovery_class: M5PartialRecoveryClass,
    /// The render / surface context.
    pub surface_context: M5RecoverySurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RecoveryResolutionForm>,
    /// The published recovery-action kind (what the action would do); empty means missing.
    pub recovery_action_kind: String,
    /// The published recovery site (where it would run); empty means missing.
    pub recovery_site: String,
    /// The published state consequence (what happens to the partial state); empty means missing.
    pub state_consequence: String,
    /// The published lineage consequence (what happens to the transcript lineage); empty means missing.
    pub lineage_consequence: String,
    /// The published explicit-action requirement; empty means missing.
    pub explicit_action_requirement: String,
    /// The published attribution reference (who / what attributes this recovery action); empty means missing.
    pub attribution_ref: String,
    /// True when the recovery action identifies exactly what it would do, where, and its state / lineage
    /// consequence.
    pub identifies_recovery_site_and_state_effect: bool,
    /// True when the recovery action is truthfully typed (never claims a safe class over a state-mutating action).
    pub action_is_truthfully_typed: bool,
    /// True when the recovery action is a state-mutating action (resumes acquisition or discards partial state).
    pub is_state_mutating_action: bool,
    /// True when a state-mutating action requires an explicit discard or cleanup action.
    pub explicit_discard_or_cleanup_action_present: bool,
    /// True when the action schedules deferred cleanup follow-up.
    pub schedules_deferred_cleanup: bool,
    /// True when scheduled deferred cleanup is disclosed rather than left implicit.
    pub cleanup_is_disclosed: bool,
    /// True when the action would discard partial state or transcript lineage without an explicit action (a hard
    /// invariant when `true`).
    pub discards_state_without_explicit_action: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe partial-recovery projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPartialRecoveryEntry {
    /// Stable identity of the partial-recovery entry.
    pub entry_id: String,
    /// The stable source-ref this recovery action binds to.
    pub source_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The partial-recovery-class token named by the entry.
    pub recovery_class: String,
    /// Whether the partial-recovery class is classified into the resolved taxonomy.
    pub recovery_class_is_classified: bool,
    /// Whether the partial-recovery class is a state-mutating action.
    pub recovery_class_is_state_mutating: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published recovery-action kind.
    pub recovery_action_kind: String,
    /// The published recovery site.
    pub recovery_site: String,
    /// The published state consequence.
    pub state_consequence: String,
    /// The published lineage consequence.
    pub lineage_consequence: String,
    /// The published explicit-action requirement.
    pub explicit_action_requirement: String,
    /// The published attribution reference.
    pub attribution_ref: String,
    /// Whether the recovery action identifies what it would do, where, and its consequence.
    pub identifies_recovery_site_and_state_effect: bool,
    /// Whether the recovery action is truthfully typed.
    pub action_is_truthfully_typed: bool,
    /// Whether the recovery action is a state-mutating action.
    pub is_state_mutating_action: bool,
    /// Whether a state-mutating action requires an explicit discard or cleanup action.
    pub explicit_discard_or_cleanup_action_present: bool,
    /// Whether the action schedules deferred cleanup follow-up.
    pub schedules_deferred_cleanup: bool,
    /// Whether scheduled deferred cleanup is disclosed.
    pub cleanup_is_disclosed: bool,
    /// Whether the action would discard partial state or transcript lineage without an explicit action.
    pub discards_state_without_explicit_action: bool,
    /// Whether the recovery action preserves lineage (state-mutating actions gated by an explicit discard or
    /// cleanup action, disclosed cleanup, never discarding partial state or lineage implicitly).
    pub partial_recovery_action_preserves_lineage: bool,
    /// Whether the entry provides the complete partial-recovery object (recovery action kind, recovery site,
    /// state / lineage consequence, explicit-action requirement, attribution).
    pub provides_complete_partial_recovery: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5PartialRecoveryEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RecoveryNextAction,
    /// Whether the recovery action is safe on every claimed source (clean entry naming every fact).
    pub recovery_safe_on_every_source: bool,
}

impl M5ResolvedPartialRecoveryEntry {
    /// Whether this partial-recovery entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RecoveryResolutionError {
    /// The acquisition-evidence-entry id was empty.
    EmptyAcquisitionEvidenceEntryId,
    /// The partial-recovery-entry id was empty.
    EmptyPartialRecoveryEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RecoveryResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyAcquisitionEvidenceEntryId => "empty_acquisition_evidence_entry_id",
            Self::EmptyPartialRecoveryEntryId => "empty_partial_recovery_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RecoveryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 acquisition-evidence / partial-recovery registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RecoveryResolutionError {}

fn form_tokens(forms: &[M5RecoveryResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5RecoveryResolutionForm]) -> bool {
    let present: BTreeSet<M5RecoveryResolutionForm> = forms.iter().copied().collect();
    M5RecoveryResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved acquisition-evidence packet publishes every required field: evidence mode (via a
/// classified kind), clone / fetch transcript reference, warnings-and-retries reference, resulting-root-identity
/// reference, omitted-or-unfetched reference, bootstrap-checkpoint reference, and evidence provenance. An
/// unclassified kind or any empty field never resolves to a complete packet.
#[allow(clippy::too_many_arguments)]
pub fn acquisition_evidence_object_is_complete(
    kind: M5AcquisitionEvidenceKind,
    transcript_ref: &str,
    warnings_and_retries_ref: &str,
    resulting_root_identity_ref: &str,
    omitted_or_unfetched_ref: &str,
    bootstrap_checkpoint_ref: &str,
    evidence_provenance: &str,
) -> bool {
    kind.is_classified()
        && !transcript_ref.trim().is_empty()
        && !warnings_and_retries_ref.trim().is_empty()
        && !resulting_root_identity_ref.trim().is_empty()
        && !omitted_or_unfetched_ref.trim().is_empty()
        && !bootstrap_checkpoint_ref.trim().is_empty()
        && !evidence_provenance.trim().is_empty()
}

/// Whether the acquisition evidence discloses partial state: the kind must be classified, a partial or interrupted
/// acquisition must stay visible, and a partial-describing packet must disclose partial-not-full status before it
/// may present (never presenting partial content as a healthy full checkout). An unclassified kind, an invisible
/// partial state, or a partial-describing packet with no disclosure never matches.
pub fn acquisition_evidence_discloses_partial_state(
    kind: M5AcquisitionEvidenceKind,
    partial_state_visible: bool,
    describes_partial_state: bool,
    partial_not_full_disclosed: bool,
) -> bool {
    kind.is_classified()
        && partial_state_visible
        && (!describes_partial_state || partial_not_full_disclosed)
}

/// Whether a partial-recovery action preserves lineage: the class must be classified, the action must be
/// truthfully typed, it must identify what it would do, where, and its state / lineage consequence, it must never
/// discard partial state or transcript lineage without an explicit action, any state-mutating action must require
/// an explicit discard or cleanup action, and any scheduled deferred cleanup must be disclosed.
#[allow(clippy::too_many_arguments)]
pub fn partial_recovery_action_preserves_lineage(
    class: M5PartialRecoveryClass,
    action_is_truthfully_typed: bool,
    identifies_recovery_site_and_state_effect: bool,
    is_state_mutating_action: bool,
    explicit_discard_or_cleanup_action_present: bool,
    schedules_deferred_cleanup: bool,
    cleanup_is_disclosed: bool,
    discards_state_without_explicit_action: bool,
) -> bool {
    class.is_classified()
        && action_is_truthfully_typed
        && identifies_recovery_site_and_state_effect
        && !discards_state_without_explicit_action
        && (!is_state_mutating_action || explicit_discard_or_cleanup_action_present)
        && (!schedules_deferred_cleanup || cleanup_is_disclosed)
}

/// Resolves an acquisition-evidence-registry entry so it stays bound to the acquisition-evidence registry: the
/// entry names its canonical token, semantic role, and evidence kind, covers all three resolution forms, publishes
/// a complete evidence packet (clone / fetch transcript reference, warnings-and-retries reference,
/// resulting-root-identity reference, omitted-or-unfetched reference, bootstrap-checkpoint reference, evidence
/// provenance), keeps a partial or interrupted acquisition visible, and discloses partial-not-full status before
/// any partial-describing packet.
pub fn resolve_acquisition_evidence_entry(
    input: M5AcquisitionEvidenceEntryResolutionInput,
) -> Result<M5ResolvedAcquisitionEvidenceEntry, M5RecoveryResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RecoveryResolutionError::EmptyAcquisitionEvidenceEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.acquisition_path_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.transcript_ref)
        || string_is_forbidden(&input.warnings_and_retries_ref)
        || string_is_forbidden(&input.resulting_root_identity_ref)
        || string_is_forbidden(&input.omitted_or_unfetched_ref)
        || string_is_forbidden(&input.bootstrap_checkpoint_ref)
        || string_is_forbidden(&input.evidence_provenance)
    {
        return Err(M5RecoveryResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = acquisition_evidence_object_is_complete(
        input.evidence_kind,
        &input.transcript_ref,
        &input.warnings_and_retries_ref,
        &input.resulting_root_identity_ref,
        &input.omitted_or_unfetched_ref,
        &input.bootstrap_checkpoint_ref,
        &input.evidence_provenance,
    );
    let discloses_ok = acquisition_evidence_discloses_partial_state(
        input.evidence_kind,
        input.partial_state_visible,
        input.describes_partial_state,
        input.partial_not_full_disclosed,
    );
    let presented_as_full_early =
        input.describes_partial_state && !input.partial_not_full_disclosed;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5AcquisitionEvidenceEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.evidence_kind.is_classified() {
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceKindUnclassified)
    } else if !input.bound_to_registry {
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceNotBoundToRegistry)
    } else if !object_complete {
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidencePacketIncomplete)
    } else if !discloses_ok {
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceOverclaimsFullCheckoutOrHidesPartialState)
    } else if !all_forms {
        Some(M5AcquisitionEvidenceEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if presented_as_full_early {
        Some(M5AcquisitionEvidenceEntryDegradeReason::PartialStatePresentedAsHealthyFullCheckout)
    } else if !input.proof_fresh {
        Some(M5AcquisitionEvidenceEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RecoveryNextAction::ExpandRecoveryMeaning,
    };

    Ok(M5ResolvedAcquisitionEvidenceEntry {
        entry_id: input.entry_id,
        acquisition_path_id: input.acquisition_path_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        evidence_kind: input.evidence_kind.as_str().to_owned(),
        evidence_kind_is_classified: input.evidence_kind.is_classified(),
        canonical_evidence_mode: input.evidence_kind.canonical_evidence_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        transcript_ref: input.transcript_ref,
        warnings_and_retries_ref: input.warnings_and_retries_ref,
        resulting_root_identity_ref: input.resulting_root_identity_ref,
        omitted_or_unfetched_ref: input.omitted_or_unfetched_ref,
        bootstrap_checkpoint_ref: input.bootstrap_checkpoint_ref,
        evidence_provenance: input.evidence_provenance,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        evidence_packet_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        partial_state_visible: input.partial_state_visible,
        describes_partial_state: input.describes_partial_state,
        partial_not_full_disclosed: input.partial_not_full_disclosed,
        degrade_reason,
        next_action,
        evidence_resolves_across_entry_flows: degrade_reason.is_none(),
    })
}

/// Resolves a partial-recovery entry so its action stays safe: the entry names its canonical token, semantic role,
/// and partial-recovery class, covers all three resolution forms, provides the complete recovery-action-kind /
/// recovery-site / state-consequence / lineage-consequence / explicit-action-requirement / attribution recovery
/// object, and degrades honestly when a state-mutating action would discard partial state during acquisition, run
/// ungated without an explicit discard or cleanup action, or hide what it would do and where.
pub fn resolve_partial_recovery_entry(
    input: M5PartialRecoveryEntryResolutionInput,
) -> Result<M5ResolvedPartialRecoveryEntry, M5RecoveryResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RecoveryResolutionError::EmptyPartialRecoveryEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.source_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.recovery_action_kind)
        || string_is_forbidden(&input.recovery_site)
        || string_is_forbidden(&input.state_consequence)
        || string_is_forbidden(&input.lineage_consequence)
        || string_is_forbidden(&input.explicit_action_requirement)
        || string_is_forbidden(&input.attribution_ref)
    {
        return Err(M5RecoveryResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let preserves_lineage = partial_recovery_action_preserves_lineage(
        input.recovery_class,
        input.action_is_truthfully_typed,
        input.identifies_recovery_site_and_state_effect,
        input.is_state_mutating_action,
        input.explicit_discard_or_cleanup_action_present,
        input.schedules_deferred_cleanup,
        input.cleanup_is_disclosed,
        input.discards_state_without_explicit_action,
    );
    let provides_recovery = input.recovery_class.is_classified()
        && !input.recovery_action_kind.trim().is_empty()
        && !input.recovery_site.trim().is_empty()
        && !input.state_consequence.trim().is_empty()
        && !input.lineage_consequence.trim().is_empty()
        && !input.explicit_action_requirement.trim().is_empty()
        && !input.attribution_ref.trim().is_empty()
        && preserves_lineage;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PartialRecoveryEntryDegradeReason::RecoveryTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PartialRecoveryEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.recovery_class.is_classified() {
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryClassUnclassified)
    } else if !provides_recovery {
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction)
    } else if !all_forms {
        Some(M5PartialRecoveryEntryDegradeReason::RecoveryFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PartialRecoveryEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RecoveryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedPartialRecoveryEntry {
        entry_id: input.entry_id,
        source_ref: input.source_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        recovery_class: input.recovery_class.as_str().to_owned(),
        recovery_class_is_classified: input.recovery_class.is_classified(),
        recovery_class_is_state_mutating: input.recovery_class.mutates_or_discards_state(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        recovery_action_kind: input.recovery_action_kind,
        recovery_site: input.recovery_site,
        state_consequence: input.state_consequence,
        lineage_consequence: input.lineage_consequence,
        explicit_action_requirement: input.explicit_action_requirement,
        attribution_ref: input.attribution_ref,
        identifies_recovery_site_and_state_effect: input.identifies_recovery_site_and_state_effect,
        action_is_truthfully_typed: input.action_is_truthfully_typed,
        is_state_mutating_action: input.is_state_mutating_action,
        explicit_discard_or_cleanup_action_present: input
            .explicit_discard_or_cleanup_action_present,
        schedules_deferred_cleanup: input.schedules_deferred_cleanup,
        cleanup_is_disclosed: input.cleanup_is_disclosed,
        discards_state_without_explicit_action: input.discards_state_without_explicit_action,
        partial_recovery_action_preserves_lineage: preserves_lineage,
        provides_complete_partial_recovery: provides_recovery,
        degrade_reason,
        next_action,
        recovery_safe_on_every_source: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved acquisition-evidence and partial-recovery entries
/// it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5AcquisitionEvidencePartialRecoveryRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5RepositoryBootstrapQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Acquisition contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5RepositoryBootstrapDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5RepositoryBootstrapRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5RepositoryBootstrapAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5RecoveryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RecoveryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    /// Resolved acquisition-evidence-registry examples.
    pub acquisition_evidence_entries: Vec<M5ResolvedAcquisitionEvidenceEntry>,
    /// Resolved partial-recovery examples.
    pub partial_recovery_entries: Vec<M5ResolvedPartialRecoveryEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the checkout-plan and bootstrap-evidence
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never presents a partial acquisition as a healthy full checkout. MUST be `false`.
    pub presents_partial_acquisition_as_healthy_full_checkout: bool,
    /// Hard invariant: this row never discards partial state or transcript lineage without an explicit action.
    /// MUST be `false`.
    pub discards_partial_state_or_lineage_without_explicit_action: bool,
    /// Hard invariant: this row never hides what a recovery action would do or its state / lineage effect. MUST be
    /// `false`.
    pub hides_what_a_recovery_action_would_do_or_its_state_or_lineage_effect: bool,
    /// Hard invariant: this row never leaves partial or interrupted state invisible or unrecoverable. MUST be
    /// `false`.
    pub leaves_partial_or_interrupted_state_invisible_or_unrecoverable: bool,
}

impl M5AcquisitionEvidencePartialRecoveryRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RecoveryAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5RecoveryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RecoveryExportField> = self.export_fields.iter().copied().collect();
        M5RecoveryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.presents_partial_acquisition_as_healthy_full_checkout
            && !self.discards_partial_state_or_lineage_without_explicit_action
            && !self.hides_what_a_recovery_action_would_do_or_its_state_or_lineage_effect
            && !self.leaves_partial_or_interrupted_state_invisible_or_unrecoverable
    }

    /// True when a clean acquisition-evidence entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified evidence kind, publishes a complete evidence packet, keeps the partial state visible,
    /// covers all three resolution forms, and discloses partial-not-full status before any partial-describing
    /// packet.
    fn evidence_is_honest(ex: &M5ResolvedAcquisitionEvidenceEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.evidence_kind_is_classified
                && ex.evidence_packet_complete
                && ex.partial_state_visible
                && ex.covers_all_resolution_forms
                && (!ex.describes_partial_state || ex.partial_not_full_disclosed))
    }

    /// True when a clean partial-recovery entry preserves a safe action: it keeps a classified class, provides the
    /// complete recovery object, preserves lineage, and covers all three resolution forms.
    fn recovery_is_honest(ex: &M5ResolvedPartialRecoveryEntry) -> bool {
        !ex.is_clean()
            || (ex.recovery_class_is_classified
                && ex.provides_complete_partial_recovery
                && ex.partial_recovery_action_preserves_lineage
                && !ex.discards_state_without_explicit_action
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.acquisition_evidence_entries
            .iter()
            .all(Self::evidence_is_honest)
            && self
                .partial_recovery_entries
                .iter()
                .all(Self::recovery_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Evidence-kind tokens (minted by this lane).
    pub evidence_kinds: Vec<String>,
    /// Partial-recovery-class tokens (minted by this lane).
    pub partial_recovery_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Acquisition-evidence-entry degrade-reason tokens.
    pub acquisition_evidence_degrade_reasons: Vec<String>,
    /// Partial-recovery-entry degrade-reason tokens.
    pub partial_recovery_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5AcquisitionEvidencePartialRecoveryRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5RepositoryBootstrapRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5RecoveryResolutionForm::ALL, |v| v.as_str()),
            evidence_kinds: tokens(&M5AcquisitionEvidenceKind::ALL, |v| v.as_str()),
            partial_recovery_classes: tokens(&M5PartialRecoveryClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5RecoverySurfaceContext::ALL, |v| v.as_str()),
            acquisition_evidence_degrade_reasons: tokens(
                &M5AcquisitionEvidenceEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            partial_recovery_degrade_reasons: tokens(
                &M5PartialRecoveryEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5RecoveryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5RecoveryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RecoveryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5RepositoryBootstrapConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesGovernanceReview {
    /// The acquisition-evidence registry names a canonical token, semantic role, and evidence kind for every
    /// entry.
    pub evidence_registry_names_token_role_and_kind: bool,
    /// Every claimed acquisition path resolves to one stable acquisition-evidence packet from the shared registry,
    /// not per-entry reconstruction.
    pub entry_flow_resolves_to_stable_evidence_from_shared_registry: bool,
    /// The transcript, warnings and retries, resulting root identity, omitted-or-unfetched state, bootstrap
    /// checkpoint, and provenance are published for every resolved evidence packet.
    pub transcript_warnings_root_omitted_checkpoint_and_provenance_published: bool,
    /// The acquisition evidence stays visible and recoverable; no partial content is presented as a healthy full
    /// checkout.
    pub acquisition_evidence_stays_visible_no_full_checkout_overclaim: bool,
    /// The partial recovery identifies exactly what the action would do, where, and what state / lineage
    /// consequence it carries.
    pub partial_recovery_identifies_action_and_consequence: bool,
    /// A state-mutating recovery action requires an explicit discard or cleanup action and never discards state or
    /// lineage implicitly.
    pub state_mutating_recovery_requires_explicit_discard_or_cleanup: bool,
    /// Every acquisition-evidence and partial-recovery entry covers the canonical / accessible / audit resolution
    /// forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Acquisition-evidence and partial-recovery behavior stay bound to the shared registries rather than
    /// hand-copied per acquisition path.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Acquisition, git, trust, and diagnostics read a single evidence / recovery source.
    pub acquisition_git_trust_diagnostics_read_single_source: bool,
    /// A partial content presented as full, a discarded state without an explicit action, or a hidden consequence
    /// is caught by fixtures before release evidence turns green.
    pub evidence_or_recovery_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesConsumerProjection {
    /// Acquisition engine and git service consume the shared acquisition-evidence registry.
    pub acquisition_and_git_consume_shared_registries: bool,
    /// Trust service and diagnostics consume the shared partial-recovery registry.
    pub trust_and_diagnostics_consume_shared_registries: bool,
    /// CLI export and support export consume the shared registries.
    pub cli_and_support_export_consume_shared_registries: bool,
    /// Docs, help, and workspace services consume the shared registries.
    pub docs_help_and_workspace_consume_shared_registries: bool,
    /// Behavior traces back to the canonical checkout-plan and bootstrap-evidence domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical acquisition-evidence / partial-recovery registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting repository-bootstrap audit for the lane.
    pub repository_bootstrap_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AcquisitionEvidencePartialRecoveryRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5AcquisitionEvidencePartialRecoveryRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AcquisitionEvidencePartialRecoveryRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AcquisitionEvidencePartialRecoveryRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AcquisitionEvidencePartialRecoveryRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AcquisitionEvidencePartialRecoveryRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AcquisitionEvidencePartialRecoveryRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 acquisition-evidence and partial-recovery registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AcquisitionEvidencePartialRecoveryRegistriesPacket {
    /// Record kind; must equal [`M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5AcquisitionEvidencePartialRecoveryRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AcquisitionEvidencePartialRecoveryRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AcquisitionEvidencePartialRecoveryRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AcquisitionEvidencePartialRecoveryRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AcquisitionEvidencePartialRecoveryRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AcquisitionEvidencePartialRecoveryRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AcquisitionEvidencePartialRecoveryRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5AcquisitionEvidencePartialRecoveryRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_RECORD_KIND {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_VERSION
        {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 acquisition-evidence / partial-recovery registries packet serializes"),
        ) {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 acquisition-evidence / partial-recovery registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,acquisition_evidence_entries,partial_recovery_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .acquisition_evidence_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.partial_recovery_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.acquisition_evidence_entries.len(),
                row.partial_recovery_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Acquisition-Evidence and Partial-Recovery Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Evidence kinds: {}\n",
            self.vocabulary_set.evidence_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
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
                "  - Acquisition-evidence entries: {} / partial-recovery entries: {}\n",
                row.acquisition_evidence_entries.len(),
                row.partial_recovery_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-item partial-recovery reference table generated from the registry, so docs and admin
    /// runbooks render the same recovery-action / recovery-site / state-consequence / lineage-consequence /
    /// explicit-action-requirement truth the resolvers produced rather than a hand-copied recovery table. Only
    /// clean, registry-bound partial-recovery entries are listed.
    pub fn render_partial_recovery_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| source_ref | recovery_class | recovery_action_kind | recovery_site | state_consequence | lineage_consequence | explicit_action_requirement |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.partial_recovery_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.source_ref,
                    ex.recovery_class,
                    ex.recovery_action_kind,
                    ex.recovery_site,
                    ex.state_consequence,
                    ex.lineage_consequence,
                    ex.explicit_action_requirement
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5AcquisitionEvidencePartialRecoveryRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>),
}

impl fmt::Display for M5AcquisitionEvidencePartialRecoveryRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 acquisition-evidence / partial-recovery registries export parse failed: {error}"
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
                    "m5 acquisition-evidence / partial-recovery registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AcquisitionEvidencePartialRecoveryRegistriesArtifactError {}

/// Validation failures emitted by [`M5AcquisitionEvidencePartialRecoveryRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AcquisitionEvidencePartialRecoveryRegistriesViolation {
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
    /// A registry row does not point at both the checkout-plan and bootstrap-evidence domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, overclaiming, field-incomplete,
    /// form-incomplete, or a partial-recovery entry missing the complete recovery object).
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
    /// Acquisition-evidence-resolution is not proven: clean evidence entries do not cover the canonical evidence
    /// kinds or the first shell / entry / diagnostics / admin / support surfaces, no packet-incomplete example
    /// degrades, or a clean evidence entry published an incomplete packet.
    AcquisitionEvidenceResolutionNotProven,
    /// Partial-state-visibility is not proven: no overclaim example and no unbound example degrade, no clean
    /// visible-partial evidence entry is present, or a clean evidence entry presented partial content as full or
    /// is unbound.
    PartialStateVisibilityNotProven,
    /// Partial-recovery-gating is not proven: clean recovery entries do not cover the canonical resume / discard /
    /// open-read-only / inert-status classes with full resolution-form coverage while providing the complete
    /// recovery object, no discard-without-action or form-incomplete example degrades, or a clean recovery entry
    /// discards state implicitly or is missing the complete recovery object.
    PartialRecoveryGatingNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AcquisitionEvidencePartialRecoveryRegistriesViolation {
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
            Self::AcquisitionEvidenceResolutionNotProven => {
                "acquisition_evidence_resolution_not_proven"
            }
            Self::PartialStateVisibilityNotProven => "partial_state_visibility_not_proven",
            Self::PartialRecoveryGatingNotProven => "partial_recovery_gating_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_acquisition_evidence_and_partial_recovery_registries_export() -> Result<
    M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    M5AcquisitionEvidencePartialRecoveryRegistriesArtifactError,
> {
    let packet: M5AcquisitionEvidencePartialRecoveryRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-acquisition-evidence-and-partial-recovery-registries-proof/support_export.json"
        )
    ))
    .map_err(M5AcquisitionEvidencePartialRecoveryRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AcquisitionEvidencePartialRecoveryRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_REF,
        M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::NoRegistryRows);
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
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.acquisition_evidence_entries.is_empty() || row.partial_recovery_entries.is_empty() {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.evidence_registry_names_token_role_and_kind,
        review.entry_flow_resolves_to_stable_evidence_from_shared_registry,
        review.transcript_warnings_root_omitted_checkpoint_and_provenance_published,
        review.acquisition_evidence_stays_visible_no_full_checkout_overclaim,
        review.partial_recovery_identifies_action_and_consequence,
        review.state_mutating_recovery_requires_explicit_discard_or_cleanup,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.acquisition_git_trust_diagnostics_read_single_source,
        review.evidence_or_recovery_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.acquisition_and_git_consume_shared_registries,
        projection.trust_and_diagnostics_consume_shared_registries,
        projection.cli_and_support_export_consume_shared_registries,
        projection.docs_help_and_workspace_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.repository_bootstrap_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
    violations: &mut Vec<M5AcquisitionEvidencePartialRecoveryRegistriesViolation>,
) {
    let evidences = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.acquisition_evidence_entries.iter())
    };
    let recoveries = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.partial_recovery_entries.iter())
    };

    // AC1: partial or interrupted acquisition remains visible and recoverable — every claimed acquisition path
    // resolves to one stable acquisition-evidence packet with transcript / warnings / root-identity /
    // omitted-or-unfetched / checkpoint / provenance fields. Clean evidence entries cover the canonical evidence
    // kinds and the first shell / entry / diagnostics / admin / support surfaces, a packet-incomplete example
    // degrades, and no clean evidence entry published an incomplete packet.
    let clean_kinds: BTreeSet<String> = evidences()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.evidence_kind.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = evidences()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let kinds_covered = M5AcquisitionEvidenceKind::CANONICAL_KINDS
        .iter()
        .all(|k| clean_kinds.contains(k.as_str()));
    let first_surfaces_covered = M5RecoverySurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let packet_incomplete_degrades = evidences().any(|ex| {
        ex.degrade_reason == Some(M5AcquisitionEvidenceEntryDegradeReason::EvidencePacketIncomplete)
    });
    let no_clean_incomplete = !evidences().any(|ex| ex.is_clean() && !ex.evidence_packet_complete);
    if !(kinds_covered
        && first_surfaces_covered
        && packet_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5AcquisitionEvidencePartialRecoveryRegistriesViolation::AcquisitionEvidenceResolutionNotProven,
        );
    }

    // AC1/AC2: the acquisition evidence stays visible and honest — partial content is never presented as a healthy
    // full checkout. An overclaim example degrades, an unbound example degrades, at least one clean visible-partial
    // evidence entry is present, and no clean evidence entry presented partial content as full or is unbound.
    let overclaim_degrades = evidences().any(|ex| {
        ex.degrade_reason
            == Some(
                M5AcquisitionEvidenceEntryDegradeReason::EvidenceOverclaimsFullCheckoutOrHidesPartialState,
            )
    });
    let unbound_degrades = evidences().any(|ex| {
        ex.degrade_reason
            == Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceNotBoundToRegistry)
    });
    let visible_clean_evidence = evidences().any(|ex| {
        ex.is_clean()
            && ex.partial_state_visible
            && (!ex.describes_partial_state || ex.partial_not_full_disclosed)
    });
    let no_clean_unbound = !evidences().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_overclaim = !evidences()
        .any(|ex| ex.is_clean() && ex.describes_partial_state && !ex.partial_not_full_disclosed);
    if !(overclaim_degrades
        && unbound_degrades
        && visible_clean_evidence
        && no_clean_unbound
        && no_clean_overclaim)
    {
        violations
            .push(M5AcquisitionEvidencePartialRecoveryRegistriesViolation::PartialStateVisibilityNotProven);
    }

    // AC3: the suite fails when a recovery action discards partial state or transcript lineage without an explicit
    // action. Clean recovery entries cover every canonical resume / discard / open-read-only / inert-status class
    // with full resolution-form coverage while providing the complete recovery object, a discard-without-action
    // example degrades, a form-incomplete example degrades, and no clean recovery entry discards state implicitly
    // or is missing the complete recovery object.
    let clean_recovery_classes: BTreeSet<String> = recoveries()
        .filter(|ex| {
            ex.is_clean()
                && ex.recovery_class_is_classified
                && ex.provides_complete_partial_recovery
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.recovery_class.clone())
        .collect();
    let recovery_classes_covered = M5PartialRecoveryClass::CANONICAL_CLASSES
        .iter()
        .all(|c| clean_recovery_classes.contains(c.as_str()));
    let discard_without_action_degrades = recoveries().any(|ex| {
        ex.degrade_reason
            == Some(
                M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction,
            )
    });
    let form_incomplete_degrades = recoveries().any(|ex| {
        ex.degrade_reason
            == Some(M5PartialRecoveryEntryDegradeReason::RecoveryFormCoverageIncomplete)
    });
    let no_clean_discarding =
        !recoveries().any(|ex| ex.is_clean() && ex.discards_state_without_explicit_action);
    let no_clean_missing_recovery =
        !recoveries().any(|ex| ex.is_clean() && !ex.provides_complete_partial_recovery);
    if !(recovery_classes_covered
        && discard_without_action_degrades
        && form_incomplete_degrades
        && no_clean_discarding
        && no_clean_missing_recovery)
    {
        violations.push(
            M5AcquisitionEvidencePartialRecoveryRegistriesViolation::PartialRecoveryGatingNotProven,
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

/// The repository-bootstrap families this lane implements, for downstream reference. Acquisition evidence and
/// partial recovery apply to every acquisition verb, so this lane covers all five families.
pub const IMPLEMENTED_FAMILIES: [M5RepositoryBootstrapFamily; 5] = [
    M5RepositoryBootstrapFamily::OpenLocal,
    M5RepositoryBootstrapFamily::CloneRemote,
    M5RepositoryBootstrapFamily::OpenArchive,
    M5RepositoryBootstrapFamily::ImportBundle,
    M5RepositoryBootstrapFamily::ResumeSnapshot,
];
