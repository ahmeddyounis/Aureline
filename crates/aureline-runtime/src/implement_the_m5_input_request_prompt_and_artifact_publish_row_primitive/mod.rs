//! Implements the reusable input-request-prompt and artifact-publish-row primitive: a
//! typed input-request prompt, a set of produced artifact-publish rows, a CLI /
//! headless line, and a support-export projection that all resolve from one bounded
//! execution interaction and share one interaction identity, one run identity, and one
//! attempt identity, so a run that is still live stays explicit about the data it is
//! requesting, the consequence of a timeout or dismissal, and the lineage, freshness,
//! and retention of every object it has produced.
//!
//! Where
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`]
//! *freezes* the reusable execution-lifecycle component families as a governed
//! contract, this module *narrows* two of those families —
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::InputRequestPrompt`]
//! and
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily::ArtifactPublishRow`]
//! — into one working primitive with a real **resolver**. A single execution
//! interaction projects onto surfaces that share one interaction identity, one run
//! identity, and one attempt identity, so input consequence, produced-object lineage,
//! artifact freshness, and retention truth never blur across the prompt, the artifact
//! rows, the CLI / headless line, and the support-export projection.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — dismissed or timed-out requests no longer behave like silent failures.**
//!   Every input-request prompt resolves its disposition (awaiting, continued,
//!   timed-out, dismissed, cancelled) into an explicit, visible result posture, so a
//!   run that was not answered reads as blocked, cancelled, or default-applied rather
//!   than vanishing.
//! - **AC2 — produced artifacts remain attributable after the live pane clears.**
//!   Every artifact row keeps its producing run and attempt refs, so lineage survives
//!   even when retention has evicted the bytes or the activity-center history has
//!   compressed.
//! - **AC3 — users can tell whether an artifact is live, buffered, imported, sampled,
//!   or provider-supplied before opening or exporting it.** Every artifact row
//!   discloses its freshness class and carries an open / export action, so a stale or
//!   provider-supplied object never reads as a live local result.
//!
//! Raw prompt bytes, secret values, artifact bytes, provider cursors, credentials, and
//! raw event payloads never cross this boundary; the resolver carries only opaque refs,
//! typed class tokens, booleans, and redacted labels, so support and diagnostics
//! exports reconstruct exactly what a surface would have shown without leaking source
//! or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-input-request-artifact-publish.schema.json`](../../../../schemas/ui/m5-input-request-artifact-publish.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_input_request_artifact_publish_primitive.md`](../../../../docs/run-test-debug/m5_input_request_artifact_publish_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    DegradedState, M5ExecutionDowngradeTrigger, M5ExecutionLocality, M5ExecutionTruthMode,
    M5InputConsequence, M5RetentionClass, M5RunOutcome,
};
use crate::implement_the_m5_run_attempt_header_and_attempt_selector_primitive::M5RunAttemptSurfaceFamily;

/// Stable record-kind tag carried by [`M5ExecutionInteractionPrimitivePacket`].
pub const M5_EXECUTION_INTERACTION_RECORD_KIND: &str =
    "m5_input_request_artifact_publish_primitive";

/// Schema version for the input-request / artifact-publish primitive packet.
pub const M5_EXECUTION_INTERACTION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_EXECUTION_INTERACTION_SCHEMA_REF: &str =
    "schemas/ui/m5-input-request-artifact-publish.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXECUTION_INTERACTION_DOC_REF: &str =
    "docs/run-test-debug/m5_input_request_artifact_publish_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_EXECUTION_INTERACTION_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EXECUTION_INTERACTION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-input-request-artifact-publish-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_EXECUTION_INTERACTION_ARTIFACT_REF: &str =
    "artifacts/release/m5-input-request-artifact-publish-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_EXECUTION_INTERACTION_CSV_REF: &str =
    "artifacts/release/m5-input-request-artifact-publish-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_EXECUTION_INTERACTION_REPORT_REF: &str =
    "artifacts/release/m5-input-request-artifact-publish-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed input-request-kind vocabulary. Names the typed data a prompt requests so a
/// secret, a file, or a device handoff never reads as a plain-text field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputRequestKind {
    /// A plain-text value.
    PlainText,
    /// A secret value that is never echoed or exported.
    SecretInput,
    /// A file / path selection.
    FilePathSelection,
    /// An explicit approval gate.
    Approval,
    /// A choice among enumerated options.
    Choice,
    /// A device / browser handoff continuation.
    DeviceBrowserHandoff,
}

impl M5InputRequestKind {
    /// Every input-request kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlainText,
        Self::SecretInput,
        Self::FilePathSelection,
        Self::Approval,
        Self::Choice,
        Self::DeviceBrowserHandoff,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::SecretInput => "secret_input",
            Self::FilePathSelection => "file_path_selection",
            Self::Approval => "approval",
            Self::Choice => "choice",
            Self::DeviceBrowserHandoff => "device_browser_handoff",
        }
    }

    /// True when this kind carries a secret that must never be echoed or exported.
    pub const fn is_secret(self) -> bool {
        matches!(self, Self::SecretInput)
    }
}

/// Closed input-request-disposition vocabulary. Names what actually happened to a
/// prompt so a dismissal or timeout is recorded rather than silently dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputRequestDisposition {
    /// The prompt is still awaiting a response.
    AwaitingResponse,
    /// The user answered and the run continued.
    Continued,
    /// The prompt reached its deadline without a response.
    TimedOut,
    /// The user dismissed the prompt without answering.
    Dismissed,
    /// The user explicitly cancelled the request.
    Cancelled,
}

impl M5InputRequestDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AwaitingResponse,
        Self::Continued,
        Self::TimedOut,
        Self::Dismissed,
        Self::Cancelled,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingResponse => "awaiting_response",
            Self::Continued => "continued",
            Self::TimedOut => "timed_out",
            Self::Dismissed => "dismissed",
            Self::Cancelled => "cancelled",
        }
    }

    /// True when the disposition has been resolved (a response, timeout, dismissal, or
    /// cancellation has occurred).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::AwaitingResponse)
    }

    /// True when the disposition is one that must never behave like a silent failure:
    /// a timeout, a dismissal, or a cancellation (AC1).
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::TimedOut | Self::Dismissed | Self::Cancelled)
    }
}

/// Closed input-result-posture vocabulary. The user-visible consequence a prompt's
/// disposition resolves to, derived from the disposition and the timeout / approval
/// consequence so a run never silently stalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputResultPosture {
    /// The prompt is still awaiting a response; the run is paused and visible.
    AwaitingResponse,
    /// The run proceeds with the answered input.
    RunProceeds,
    /// The declared default was applied on timeout and the run proceeds.
    RunProceedsWithDefault,
    /// The run is blocked and waiting, attributable to the unanswered request.
    RunBlockedWaiting,
    /// The run was cancelled as the disclosed consequence of the unanswered request.
    RunCancelled,
}

impl M5InputResultPosture {
    /// Every result posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AwaitingResponse,
        Self::RunProceeds,
        Self::RunProceedsWithDefault,
        Self::RunBlockedWaiting,
        Self::RunCancelled,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingResponse => "awaiting_response",
            Self::RunProceeds => "run_proceeds",
            Self::RunProceedsWithDefault => "run_proceeds_with_default",
            Self::RunBlockedWaiting => "run_blocked_waiting",
            Self::RunCancelled => "run_cancelled",
        }
    }

    /// True when the posture is an explicit, attributable consequence of a dismissal or
    /// timeout rather than a plain continue or an active wait (AC1).
    pub const fn is_dismissal_or_timeout_consequence(self) -> bool {
        matches!(
            self,
            Self::RunProceedsWithDefault | Self::RunBlockedWaiting | Self::RunCancelled
        )
    }
}

/// Closed artifact-kind vocabulary. Names what a produced object is so a report, a
/// trace, a preview endpoint, a bundle, or an imported provider artifact never blur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactKind {
    /// A rendered report (HTML / JSON / Markdown summary).
    Report,
    /// A profile / execution trace.
    Trace,
    /// A live or buffered preview endpoint.
    PreviewEndpoint,
    /// A packaged bundle (build / release artifact).
    Bundle,
    /// An artifact imported from an external provider.
    ImportedProviderArtifact,
    /// A diagnostic / log artifact.
    DiagnosticLog,
}

impl M5ArtifactKind {
    /// Every artifact kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Report,
        Self::Trace,
        Self::PreviewEndpoint,
        Self::Bundle,
        Self::ImportedProviderArtifact,
        Self::DiagnosticLog,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Report => "report",
            Self::Trace => "trace",
            Self::PreviewEndpoint => "preview_endpoint",
            Self::Bundle => "bundle",
            Self::ImportedProviderArtifact => "imported_provider_artifact",
            Self::DiagnosticLog => "diagnostic_log",
        }
    }
}

/// Closed artifact-freshness vocabulary. Names whether a visible artifact is live,
/// buffered, imported, sampled, or provider-supplied so a stale object never reads as
/// a live local result (AC3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactFreshness {
    /// Live: currently produced and streaming from the running attempt.
    Live,
    /// Buffered: fully produced and retained for this run.
    Buffered,
    /// Imported: reconstructed from an imported external run.
    Imported,
    /// Sampled: a partial / sampled view of a larger object.
    Sampled,
    /// Provider-supplied: owned and completed by an external provider.
    ProviderSupplied,
}

impl M5ArtifactFreshness {
    /// Every artifact freshness class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::Buffered,
        Self::Imported,
        Self::Sampled,
        Self::ProviderSupplied,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Buffered => "buffered",
            Self::Imported => "imported",
            Self::Sampled => "sampled",
            Self::ProviderSupplied => "provider_supplied",
        }
    }

    /// True when the artifact is streaming live from an actively executing attempt and
    /// so requires an active run.
    pub const fn requires_active_run(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Closed artifact-trust vocabulary. Names how trustworthy a produced object is so a
/// provider-attested or untrusted artifact never reads as first-party verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactTrustClass {
    /// First-party and verified (checksummed / signed locally).
    FirstPartyVerified,
    /// First-party but not yet verified.
    FirstPartyUnverified,
    /// Attested by an external provider.
    ProviderAttested,
    /// Untrusted; provenance could not be established.
    Untrusted,
}

impl M5ArtifactTrustClass {
    /// Every artifact trust class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstPartyVerified,
        Self::FirstPartyUnverified,
        Self::ProviderAttested,
        Self::Untrusted,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyVerified => "first_party_verified",
            Self::FirstPartyUnverified => "first_party_unverified",
            Self::ProviderAttested => "provider_attested",
            Self::Untrusted => "untrusted",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractionExportField {
    /// The stable run identity shared across surfaces.
    RunId,
    /// The stable attempt identity, distinct from the run identity.
    AttemptId,
    /// The 1-based attempt ordinal within the run.
    AttemptOrdinal,
    /// The run's user-visible outcome.
    RunOutcome,
    /// The opaque input-request-prompt ref.
    InputPromptRef,
    /// The input-request disposition (awaiting / continued / timed-out / …).
    InputDisposition,
    /// The user-visible input-result posture.
    InputResultPosture,
    /// The opaque produced-artifact ref.
    ArtifactRef,
    /// The producing-run ref that carries artifact lineage.
    ProducingRunRef,
    /// The artifact freshness class.
    ArtifactFreshness,
    /// The artifact retention class.
    ArtifactRetention,
    /// The artifact trust class.
    ArtifactTrust,
}

impl M5InteractionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RunId,
        Self::AttemptId,
        Self::AttemptOrdinal,
        Self::RunOutcome,
        Self::InputPromptRef,
        Self::InputDisposition,
        Self::InputResultPosture,
        Self::ArtifactRef,
        Self::ProducingRunRef,
        Self::ArtifactFreshness,
        Self::ArtifactRetention,
        Self::ArtifactTrust,
    ];

    /// The mandatory subset every row must carry: the run/attempt IDs, the input
    /// disposition, and the producing-run ref, freshness, and retention that must
    /// survive into any support export (AC1/AC2/AC3).
    pub const MANDATORY: [Self; 6] = [
        Self::RunId,
        Self::AttemptId,
        Self::InputDisposition,
        Self::ProducingRunRef,
        Self::ArtifactFreshness,
        Self::ArtifactRetention,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunId => "run_id",
            Self::AttemptId => "attempt_id",
            Self::AttemptOrdinal => "attempt_ordinal",
            Self::RunOutcome => "run_outcome",
            Self::InputPromptRef => "input_prompt_ref",
            Self::InputDisposition => "input_disposition",
            Self::InputResultPosture => "input_result_posture",
            Self::ArtifactRef => "artifact_ref",
            Self::ProducingRunRef => "producing_run_ref",
            Self::ArtifactFreshness => "artifact_freshness",
            Self::ArtifactRetention => "artifact_retention",
            Self::ArtifactTrust => "artifact_trust",
        }
    }
}

// --- resolver input ---

/// One typed input-request prompt within an execution interaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InputRequestInput {
    /// Opaque ref to the input request; never raw prompt bytes.
    pub prompt_ref: String,
    /// The typed data the prompt requests.
    pub kind: M5InputRequestKind,
    /// Human-readable prompt label (never a secret value).
    pub prompt_label: String,
    /// What happens on timeout / dismissal.
    pub consequence: M5InputConsequence,
    /// What actually happened to this prompt.
    pub disposition: M5InputRequestDisposition,
    /// Whether the prompt carries a resolvable deadline.
    pub has_deadline: bool,
    /// Human-readable deadline label; required when the consequence is timeout-governed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_label: Option<String>,
    /// Human-readable declared-default label; required when a timeout applies a default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_label: Option<String>,
}

/// One produced artifact within an execution interaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactPublishInput {
    /// Opaque ref to the produced artifact; never raw artifact bytes.
    pub artifact_ref: String,
    /// Opaque ref to the run that produced the artifact; lineage is never lost.
    pub producing_run_ref: String,
    /// Opaque ref to the attempt that produced the artifact; lineage is never lost.
    pub producing_attempt_ref: String,
    /// Human-readable producing-step label.
    pub producing_step_label: String,
    /// Human-readable artifact label.
    pub artifact_label: String,
    /// What kind of object the artifact is.
    pub kind: M5ArtifactKind,
    /// Whether the artifact is live, buffered, imported, sampled, or provider-supplied.
    pub freshness: M5ArtifactFreshness,
    /// How long the artifact is retained.
    pub retention: M5RetentionClass,
    /// How trustworthy the artifact is.
    pub trust: M5ArtifactTrustClass,
    /// Opaque ref to an open action, when the artifact is openable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_action_ref: Option<String>,
    /// Opaque ref to an export action, when the artifact is exportable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_action_ref: Option<String>,
}

/// The full input to the execution-interaction resolver for one run-and-attempt
/// context that may be requesting input and is producing artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionInteractionInput {
    /// The stable interaction identity that must survive across every projection.
    pub interaction_id: String,
    /// Opaque ref to the run identity; never raw run bytes.
    pub run_ref: String,
    /// Opaque ref to the attempt identity; distinct from the run identity.
    pub attempt_ref: String,
    /// 1-based ordinal of the attempt within the run.
    pub attempt_ordinal: u32,
    /// Human-readable run label.
    pub run_label: String,
    /// The run's user-visible outcome.
    pub run_outcome: M5RunOutcome,
    /// The captured-versus-live truth class of the run pane.
    pub truth_mode: M5ExecutionTruthMode,
    /// The local / remote / container / managed target boundary.
    pub target_boundary: M5ExecutionLocality,
    /// Human-readable context summary.
    pub context_summary: String,
    /// Relative age label ("just now", "2m ago").
    pub age_label: String,
    /// The live input-request prompt, when the interaction is requesting input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_request: Option<M5InputRequestInput>,
    /// The artifacts produced so far in this interaction.
    #[serde(default)]
    pub artifacts: Vec<M5ArtifactPublishInput>,
    /// An externally-observed narrowing that degrades the surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved input-request prompt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInputRequestPrompt {
    /// The interaction identity — identical to every other projection.
    pub interaction_id: String,
    /// The opaque run ref.
    pub run_ref: String,
    /// The opaque attempt ref.
    pub attempt_ref: String,
    /// The opaque prompt ref.
    pub prompt_ref: String,
    /// The typed data the prompt requests.
    pub kind: M5InputRequestKind,
    /// The prompt label.
    pub prompt_label: String,
    /// The timeout / approval consequence.
    pub consequence: M5InputConsequence,
    /// What actually happened to the prompt.
    pub disposition: M5InputRequestDisposition,
    /// The user-visible consequence the disposition resolves to.
    pub result_posture: M5InputResultPosture,
    /// The prompt carries a resolvable deadline.
    pub has_deadline: bool,
    /// The deadline label, when present.
    pub deadline_label: Option<String>,
    /// The declared-default label, when present.
    pub default_label: Option<String>,
    /// The prompt discloses its timeout behaviour; always holds by construction.
    pub discloses_timeout: bool,
    /// The prompt discloses its approval requirement; always holds by construction.
    pub discloses_approval: bool,
    /// A dismissed or timed-out prompt resolves to a visible, attributable posture
    /// rather than a silent failure; always holds by construction.
    pub is_attributable: bool,
    /// A deterministic, human-readable resolution note.
    pub resolution_note: String,
}

/// The resolved artifact-publish row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedArtifactPublishRow {
    /// The interaction identity — identical to every other projection.
    pub interaction_id: String,
    /// The opaque run ref.
    pub run_ref: String,
    /// The opaque attempt ref.
    pub attempt_ref: String,
    /// The opaque artifact ref.
    pub artifact_ref: String,
    /// The producing-run ref — identical to the interaction's run ref.
    pub producing_run_ref: String,
    /// The producing-attempt ref — identical to the interaction's attempt ref.
    pub producing_attempt_ref: String,
    /// The producing-step label.
    pub producing_step_label: String,
    /// The artifact label.
    pub artifact_label: String,
    /// What kind of object the artifact is.
    pub kind: M5ArtifactKind,
    /// The artifact freshness class.
    pub freshness: M5ArtifactFreshness,
    /// The artifact retention class.
    pub retention: M5RetentionClass,
    /// The artifact trust class.
    pub trust: M5ArtifactTrustClass,
    /// The open-action ref, when the artifact is openable.
    pub open_action_ref: Option<String>,
    /// The export-action ref, when the artifact is exportable.
    pub export_action_ref: Option<String>,
    /// The producing-run lineage stays attached; always holds by construction.
    pub lineage_preserved: bool,
    /// The freshness class is disclosed on the row; always holds by construction.
    pub freshness_disclosed: bool,
    /// The retention class is visible on the row; always holds by construction.
    pub retention_visible: bool,
    /// The artifact remains attributable to its producing run and attempt even after
    /// the live pane clears or retention evicts the bytes; always holds by construction.
    pub is_attributable: bool,
    /// The artifact can be opened in place (an open action exists and the bytes are not
    /// gone).
    pub is_openable: bool,
}

/// The resolved CLI / headless line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInteractionCliLine {
    /// The interaction identity — identical to every other projection.
    pub interaction_id: String,
    /// The opaque run ref.
    pub run_ref: String,
    /// The opaque attempt ref.
    pub attempt_ref: String,
    /// The deterministic single-line summary in the shared interaction vocabulary.
    pub line: String,
    /// The run's user-visible outcome.
    pub run_outcome: M5RunOutcome,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
}

/// The resolved support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInteractionExport {
    /// The interaction identity — identical to every other projection.
    pub interaction_id: String,
    /// The opaque run ref — identical to every other projection.
    pub run_ref: String,
    /// The opaque attempt ref — identical to every other projection.
    pub attempt_ref: String,
    /// The 1-based attempt ordinal.
    pub attempt_ordinal: u32,
    /// The run's user-visible outcome.
    pub run_outcome: M5RunOutcome,
    /// The captured-versus-live truth class.
    pub truth_mode: M5ExecutionTruthMode,
    /// The target boundary.
    pub target_boundary: M5ExecutionLocality,
    /// The input disposition, when the interaction carried a prompt.
    pub input_disposition: Option<M5InputRequestDisposition>,
    /// The input result posture, when the interaction carried a prompt.
    pub input_result_posture: Option<M5InputResultPosture>,
    /// The opaque refs of every produced artifact, preserving lineage.
    pub artifact_refs: Vec<String>,
    /// The export fields this projection carries; includes the mandatory subset.
    pub export_fields: Vec<M5InteractionExportField>,
}

/// The resolved execution-interaction truth shared across the input-request prompt,
/// the artifact-publish rows, the CLI line, and the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedExecutionInteraction {
    /// The stable interaction identity.
    pub interaction_id: String,
    /// The opaque run ref.
    pub run_ref: String,
    /// The opaque attempt ref.
    pub attempt_ref: String,
    /// The 1-based attempt ordinal.
    pub attempt_ordinal: u32,
    /// The resolved input-request prompt, when the interaction is requesting input.
    pub input_prompt: Option<M5ResolvedInputRequestPrompt>,
    /// The resolved artifact-publish rows.
    pub artifact_rows: Vec<M5ResolvedArtifactPublishRow>,
    /// The resolved CLI / headless line.
    pub cli_line: M5ResolvedInteractionCliLine,
    /// The resolved support-export projection.
    pub export: M5ResolvedInteractionExport,
    /// A dismissed or timed-out request resolves to a visible, attributable posture
    /// (AC1).
    pub input_disposition_attributable: bool,
    /// Every produced artifact preserves its producing run/attempt lineage (AC2).
    pub artifact_lineage_preserved: bool,
    /// Every produced artifact discloses its freshness class before it is opened or
    /// exported (AC3).
    pub artifact_freshness_disclosed: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedExecutionInteraction {
    /// True when the interaction identity, run ref, and attempt ref are identical
    /// across the prompt, the artifact rows, the CLI line, and the export.
    pub fn identity_consistent(&self) -> bool {
        let prompt_ok = self.input_prompt.as_ref().map_or(true, |prompt| {
            prompt.interaction_id == self.interaction_id
                && prompt.run_ref == self.run_ref
                && prompt.attempt_ref == self.attempt_ref
        });
        let rows_ok = self.artifact_rows.iter().all(|row| {
            row.interaction_id == self.interaction_id
                && row.run_ref == self.run_ref
                && row.attempt_ref == self.attempt_ref
        });
        prompt_ok
            && rows_ok
            && self.cli_line.interaction_id == self.interaction_id
            && self.cli_line.run_ref == self.run_ref
            && self.cli_line.attempt_ref == self.attempt_ref
            && self.export.interaction_id == self.interaction_id
            && self.export.run_ref == self.run_ref
            && self.export.attempt_ref == self.attempt_ref
    }

    /// True when a dismissed or timed-out request resolves to a visible, attributable
    /// posture — never a silent failure (AC1). Holds trivially when the interaction
    /// carries no prompt.
    pub fn discloses_input_consequence(&self) -> bool {
        self.input_prompt.as_ref().map_or(true, |prompt| {
            prompt.is_attributable
                && prompt.result_posture == input_result_posture(prompt.disposition, prompt.consequence)
                && (!prompt.disposition.is_negative()
                    || prompt.result_posture.is_dismissal_or_timeout_consequence())
        })
    }

    /// True when every produced artifact preserves its producing run/attempt lineage,
    /// so it stays attributable after the live pane clears or history compresses (AC2).
    pub fn preserves_artifact_lineage(&self) -> bool {
        self.artifact_rows.iter().all(|row| {
            row.lineage_preserved
                && row.is_attributable
                && row.producing_run_ref == self.run_ref
                && row.producing_attempt_ref == self.attempt_ref
        })
    }

    /// True when every produced artifact discloses its freshness class and carries an
    /// open or export action before it is opened or exported (AC3).
    pub fn discloses_artifact_freshness(&self) -> bool {
        self.artifact_rows.iter().all(|row| {
            row.freshness_disclosed
                && row.retention_visible
                && (row.open_action_ref.is_some() || row.export_action_ref.is_some())
        })
    }
}

/// Errors returned by [`resolve_execution_interaction`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ExecutionInteractionError {
    /// The interaction identity was empty.
    EmptyInteractionId,
    /// The run ref was empty.
    EmptyRunRef,
    /// The attempt ref was empty.
    EmptyAttemptRef,
    /// The run label was empty.
    EmptyRunLabel,
    /// The context summary was empty.
    EmptyContextSummary,
    /// The age label was empty.
    EmptyAgeLabel,
    /// The attempt ordinal was zero.
    InvalidAttemptOrdinal,
    /// The run ref and attempt ref were equal — run and attempt identity collapsed.
    RunAttemptIdentityCollapsed,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The interaction carried neither an input request nor any artifacts.
    EmptyInteraction,
    /// An input-request prompt ref or label was empty.
    InputPromptIncomplete,
    /// A timeout-governed input request carried no resolvable deadline.
    InputDeadlineMissing,
    /// A timeout-applies-default input request named no default value.
    TimeoutDefaultMissing,
    /// A prompt was still awaiting a response but the run was not waiting on input.
    AwaitingResponseButRunNotWaiting,
    /// The run was waiting on input but carried no input-request prompt.
    WaitingWithoutInputPrompt,
    /// An artifact ref, producing ref, step, or label was empty.
    ArtifactRowIncomplete,
    /// An artifact's producing run/attempt ref did not match the interaction — lineage
    /// was broken.
    ArtifactLineageBroken,
    /// Two artifacts shared an artifact ref.
    DuplicateArtifact,
    /// A live-freshness artifact was produced by an inactive run.
    LiveArtifactFromInactiveRun,
    /// An evicted-gone artifact still offered an open action.
    EvictedGoneArtifactOffersOpen,
    /// An artifact offered neither an open nor an export action.
    ArtifactMissingAction,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5ExecutionInteractionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyInteractionId => "empty_interaction_id",
            Self::EmptyRunRef => "empty_run_ref",
            Self::EmptyAttemptRef => "empty_attempt_ref",
            Self::EmptyRunLabel => "empty_run_label",
            Self::EmptyContextSummary => "empty_context_summary",
            Self::EmptyAgeLabel => "empty_age_label",
            Self::InvalidAttemptOrdinal => "invalid_attempt_ordinal",
            Self::RunAttemptIdentityCollapsed => "run_attempt_identity_collapsed",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::EmptyInteraction => "empty_interaction",
            Self::InputPromptIncomplete => "input_prompt_incomplete",
            Self::InputDeadlineMissing => "input_deadline_missing",
            Self::TimeoutDefaultMissing => "timeout_default_missing",
            Self::AwaitingResponseButRunNotWaiting => "awaiting_response_but_run_not_waiting",
            Self::WaitingWithoutInputPrompt => "waiting_without_input_prompt",
            Self::ArtifactRowIncomplete => "artifact_row_incomplete",
            Self::ArtifactLineageBroken => "artifact_lineage_broken",
            Self::DuplicateArtifact => "duplicate_artifact",
            Self::LiveArtifactFromInactiveRun => "live_artifact_from_inactive_run",
            Self::EvictedGoneArtifactOffersOpen => "evicted_gone_artifact_offers_open",
            Self::ArtifactMissingAction => "artifact_missing_action",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5ExecutionInteractionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "execution-interaction resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ExecutionInteractionError {}

/// Resolves one execution interaction into its shared input-request prompt, artifact-
/// publish rows, CLI / headless line, and support-export projection.
///
/// The projections share one interaction identity, one run ref, and one attempt ref,
/// so input consequence, produced-object lineage, artifact freshness, and retention
/// truth never blur. A dismissed or timed-out request resolves to a visible,
/// attributable posture; every artifact preserves its producing run/attempt lineage and
/// discloses its freshness before it is opened or exported.
///
/// # Errors
///
/// Returns an [`M5ExecutionInteractionError`] when identity is missing or collapsed, an
/// input prompt hides its timeout / default consequence or its waiting state, an
/// artifact loses its producing lineage or hides its freshness / actions, a live
/// artifact claims an inactive run, or any ref / label carries forbidden material.
pub fn resolve_execution_interaction(
    input: &M5ExecutionInteractionInput,
) -> Result<M5ResolvedExecutionInteraction, M5ExecutionInteractionError> {
    if input.interaction_id.trim().is_empty() {
        return Err(M5ExecutionInteractionError::EmptyInteractionId);
    }
    if input.run_ref.trim().is_empty() {
        return Err(M5ExecutionInteractionError::EmptyRunRef);
    }
    if input.attempt_ref.trim().is_empty() {
        return Err(M5ExecutionInteractionError::EmptyAttemptRef);
    }
    if input.run_label.trim().is_empty() {
        return Err(M5ExecutionInteractionError::EmptyRunLabel);
    }
    if input.context_summary.trim().is_empty() {
        return Err(M5ExecutionInteractionError::EmptyContextSummary);
    }
    if input.age_label.trim().is_empty() {
        return Err(M5ExecutionInteractionError::EmptyAgeLabel);
    }
    if input.attempt_ordinal == 0 {
        return Err(M5ExecutionInteractionError::InvalidAttemptOrdinal);
    }
    if input.run_ref.trim() == input.attempt_ref.trim() {
        return Err(M5ExecutionInteractionError::RunAttemptIdentityCollapsed);
    }
    if input.input_request.is_none() && input.artifacts.is_empty() {
        return Err(M5ExecutionInteractionError::EmptyInteraction);
    }

    for value in [
        input.run_ref.as_str(),
        input.attempt_ref.as_str(),
        input.run_label.as_str(),
        input.context_summary.as_str(),
        input.age_label.as_str(),
    ] {
        if value_is_forbidden(value) {
            return Err(M5ExecutionInteractionError::ForbiddenMaterial);
        }
    }

    // The run's waiting state and the prompt's disposition must agree.
    if input.run_outcome == M5RunOutcome::WaitingInput && input.input_request.is_none() {
        return Err(M5ExecutionInteractionError::WaitingWithoutInputPrompt);
    }

    let input_prompt = match &input.input_request {
        Some(request) => Some(resolve_input_prompt(input, request)?),
        None => None,
    };

    let artifact_rows = resolve_artifact_rows(input)?;

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5ExecutionInteractionError::DegradedLabelGeneric);
        }
    }

    let cli_line = M5ResolvedInteractionCliLine {
        interaction_id: input.interaction_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        line: render_cli_line(input, input_prompt.as_ref(), &artifact_rows),
        run_outcome: input.run_outcome,
        truth_mode: input.truth_mode,
    };

    let export = M5ResolvedInteractionExport {
        interaction_id: input.interaction_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        attempt_ordinal: input.attempt_ordinal,
        run_outcome: input.run_outcome,
        truth_mode: input.truth_mode,
        target_boundary: input.target_boundary,
        input_disposition: input_prompt.as_ref().map(|prompt| prompt.disposition),
        input_result_posture: input_prompt.as_ref().map(|prompt| prompt.result_posture),
        artifact_refs: artifact_rows
            .iter()
            .map(|row| row.artifact_ref.clone())
            .collect(),
        export_fields: M5InteractionExportField::ALL.to_vec(),
    };

    Ok(M5ResolvedExecutionInteraction {
        interaction_id: input.interaction_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        attempt_ordinal: input.attempt_ordinal,
        input_prompt,
        artifact_rows,
        cli_line,
        export,
        input_disposition_attributable: true,
        artifact_lineage_preserved: true,
        artifact_freshness_disclosed: true,
        degraded: input.degraded.clone(),
    })
}

fn resolve_input_prompt(
    input: &M5ExecutionInteractionInput,
    request: &M5InputRequestInput,
) -> Result<M5ResolvedInputRequestPrompt, M5ExecutionInteractionError> {
    if request.prompt_ref.trim().is_empty() || request.prompt_label.trim().is_empty() {
        return Err(M5ExecutionInteractionError::InputPromptIncomplete);
    }
    for value in [request.prompt_ref.as_str(), request.prompt_label.as_str()]
        .into_iter()
        .chain(request.deadline_label.as_deref())
        .chain(request.default_label.as_deref())
    {
        if value_is_forbidden(value) {
            return Err(M5ExecutionInteractionError::ForbiddenMaterial);
        }
    }
    // A timeout-governed request must carry a resolvable deadline.
    if request.consequence.needs_deadline()
        && (!request.has_deadline
            || !request
                .deadline_label
                .as_deref()
                .is_some_and(|label| !label.trim().is_empty()))
    {
        return Err(M5ExecutionInteractionError::InputDeadlineMissing);
    }
    // A timeout that applies a default must name that default.
    if request.consequence == M5InputConsequence::TimeoutAppliesDefault
        && !request
            .default_label
            .as_deref()
            .is_some_and(|label| !label.trim().is_empty())
    {
        return Err(M5ExecutionInteractionError::TimeoutDefaultMissing);
    }
    // An awaiting prompt must correspond to a run that is waiting on input.
    if request.disposition == M5InputRequestDisposition::AwaitingResponse
        && input.run_outcome != M5RunOutcome::WaitingInput
    {
        return Err(M5ExecutionInteractionError::AwaitingResponseButRunNotWaiting);
    }

    let result_posture = input_result_posture(request.disposition, request.consequence);
    Ok(M5ResolvedInputRequestPrompt {
        interaction_id: input.interaction_id.clone(),
        run_ref: input.run_ref.clone(),
        attempt_ref: input.attempt_ref.clone(),
        prompt_ref: request.prompt_ref.clone(),
        kind: request.kind,
        prompt_label: request.prompt_label.clone(),
        consequence: request.consequence,
        disposition: request.disposition,
        result_posture,
        has_deadline: request.has_deadline,
        deadline_label: request.deadline_label.clone(),
        default_label: request.default_label.clone(),
        discloses_timeout: true,
        discloses_approval: true,
        is_attributable: true,
        resolution_note: resolution_note(result_posture).to_owned(),
    })
}

fn resolve_artifact_rows(
    input: &M5ExecutionInteractionInput,
) -> Result<Vec<M5ResolvedArtifactPublishRow>, M5ExecutionInteractionError> {
    let mut seen_refs: BTreeSet<&str> = BTreeSet::new();
    let mut rows = Vec::with_capacity(input.artifacts.len());
    for artifact in &input.artifacts {
        if artifact.artifact_ref.trim().is_empty()
            || artifact.producing_run_ref.trim().is_empty()
            || artifact.producing_attempt_ref.trim().is_empty()
            || artifact.producing_step_label.trim().is_empty()
            || artifact.artifact_label.trim().is_empty()
        {
            return Err(M5ExecutionInteractionError::ArtifactRowIncomplete);
        }
        for value in [
            artifact.artifact_ref.as_str(),
            artifact.producing_run_ref.as_str(),
            artifact.producing_attempt_ref.as_str(),
            artifact.producing_step_label.as_str(),
            artifact.artifact_label.as_str(),
        ]
        .into_iter()
        .chain(artifact.open_action_ref.as_deref())
        .chain(artifact.export_action_ref.as_deref())
        {
            if value_is_forbidden(value) {
                return Err(M5ExecutionInteractionError::ForbiddenMaterial);
            }
        }
        // Lineage: every artifact points back at this interaction's run and attempt.
        if artifact.producing_run_ref.trim() != input.run_ref.trim()
            || artifact.producing_attempt_ref.trim() != input.attempt_ref.trim()
        {
            return Err(M5ExecutionInteractionError::ArtifactLineageBroken);
        }
        if !seen_refs.insert(artifact.artifact_ref.trim()) {
            return Err(M5ExecutionInteractionError::DuplicateArtifact);
        }
        // A live artifact must be streaming from an actively executing run.
        if artifact.freshness.requires_active_run() && !input.run_outcome.is_active() {
            return Err(M5ExecutionInteractionError::LiveArtifactFromInactiveRun);
        }
        // A gone artifact cannot be opened, though its lineage stays exportable.
        if artifact.retention == M5RetentionClass::EvictedGone && artifact.open_action_ref.is_some()
        {
            return Err(M5ExecutionInteractionError::EvictedGoneArtifactOffersOpen);
        }
        // Every artifact must be openable or exportable so its freshness matters before
        // the user acts.
        if artifact.open_action_ref.is_none() && artifact.export_action_ref.is_none() {
            return Err(M5ExecutionInteractionError::ArtifactMissingAction);
        }

        let is_openable =
            artifact.open_action_ref.is_some() && artifact.retention != M5RetentionClass::EvictedGone;
        rows.push(M5ResolvedArtifactPublishRow {
            interaction_id: input.interaction_id.clone(),
            run_ref: input.run_ref.clone(),
            attempt_ref: input.attempt_ref.clone(),
            artifact_ref: artifact.artifact_ref.clone(),
            producing_run_ref: artifact.producing_run_ref.clone(),
            producing_attempt_ref: artifact.producing_attempt_ref.clone(),
            producing_step_label: artifact.producing_step_label.clone(),
            artifact_label: artifact.artifact_label.clone(),
            kind: artifact.kind,
            freshness: artifact.freshness,
            retention: artifact.retention,
            trust: artifact.trust,
            open_action_ref: artifact.open_action_ref.clone(),
            export_action_ref: artifact.export_action_ref.clone(),
            lineage_preserved: true,
            freshness_disclosed: true,
            retention_visible: true,
            is_attributable: true,
            is_openable,
        });
    }
    Ok(rows)
}

/// Derives the user-visible result posture from a prompt's disposition and its timeout
/// / approval consequence. A dismissal or timeout always resolves to an explicit
/// posture — cancelled, default-applied, or blocked-and-waiting — never a silent stall.
fn input_result_posture(
    disposition: M5InputRequestDisposition,
    consequence: M5InputConsequence,
) -> M5InputResultPosture {
    match disposition {
        M5InputRequestDisposition::AwaitingResponse => M5InputResultPosture::AwaitingResponse,
        M5InputRequestDisposition::Continued => M5InputResultPosture::RunProceeds,
        M5InputRequestDisposition::Cancelled => M5InputResultPosture::RunCancelled,
        M5InputRequestDisposition::TimedOut | M5InputRequestDisposition::Dismissed => {
            match consequence {
                M5InputConsequence::TimeoutCancelsRun => M5InputResultPosture::RunCancelled,
                M5InputConsequence::TimeoutAppliesDefault => {
                    M5InputResultPosture::RunProceedsWithDefault
                }
                M5InputConsequence::RequiresApproval
                | M5InputConsequence::BlocksUntilAnswered
                | M5InputConsequence::DismissLeavesWaiting => M5InputResultPosture::RunBlockedWaiting,
            }
        }
    }
}

/// A deterministic, human-readable note for a resolved input-result posture.
fn resolution_note(posture: M5InputResultPosture) -> &'static str {
    match posture {
        M5InputResultPosture::AwaitingResponse => {
            "awaiting a response before the run can continue"
        }
        M5InputResultPosture::RunProceeds => "input received; the run continues",
        M5InputResultPosture::RunProceedsWithDefault => {
            "no response before the deadline; the declared default was applied and the run continues"
        }
        M5InputResultPosture::RunBlockedWaiting => {
            "the request was not answered; the run is blocked and waiting, not silently failed"
        }
        M5InputResultPosture::RunCancelled => {
            "the request was not answered in time; the run was cancelled"
        }
    }
}

/// Renders the deterministic CLI / headless line in the shared interaction vocabulary.
fn render_cli_line(
    input: &M5ExecutionInteractionInput,
    prompt: Option<&M5ResolvedInputRequestPrompt>,
    rows: &[M5ResolvedArtifactPublishRow],
) -> String {
    let input_token = match prompt {
        Some(prompt) => format!(
            "{}:{}",
            prompt.disposition.as_str(),
            prompt.result_posture.as_str()
        ),
        None => "none".to_owned(),
    };
    format!(
        "interaction={interaction} run={run} attempt=#{ordinal} outcome={outcome} truth={truth} \
boundary={boundary} input={input_token} artifacts={artifacts}",
        interaction = input.interaction_id,
        run = input.run_ref,
        ordinal = input.attempt_ordinal,
        outcome = input.run_outcome.as_str(),
        truth = input.truth_mode.as_str(),
        boundary = input.target_boundary.as_str(),
        input_token = input_token,
        artifacts = rows.len(),
    )
}

/// True when a slice of export fields declares every mandatory field.
fn declares_mandatory_export_fields(fields: &[M5InteractionExportField]) -> bool {
    let present: BTreeSet<M5InteractionExportField> = fields.iter().copied().collect();
    M5InteractionExportField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret=")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs interaction truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionInteractionCase {
    /// The resolver input.
    pub input: M5ExecutionInteractionInput,
    /// The resolved interaction. Must equal
    /// `resolve_execution_interaction(&input)`.
    pub resolved: M5ResolvedExecutionInteraction,
}

impl M5ExecutionInteractionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ExecutionInteractionInput) -> Self {
        let resolved =
            resolve_execution_interaction(&input).expect("seed execution-interaction case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_execution_interaction(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one execution surface family bound to the shared
/// input-request / artifact-publish contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractionSurfaceRow {
    /// The execution surface family.
    pub surface_family: M5RunAttemptSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Input-request kinds this surface can render (may be empty for artifact-only
    /// surfaces).
    pub input_kinds: Vec<M5InputRequestKind>,
    /// Artifact kinds this surface can render (may be empty for input-only surfaces).
    pub artifact_kinds: Vec<M5ArtifactKind>,
    /// Freshness classes this surface renders (may be empty for input-only surfaces).
    pub freshness_classes: Vec<M5ArtifactFreshness>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5InteractionExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ExecutionDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_interactions: Vec<M5ExecutionInteractionCase>,
    /// Hard invariant: this row never hides a dismissed / timed-out consequence. MUST
    /// be `false`.
    pub hides_input_consequence: bool,
    /// Hard invariant: this row never drops produced-artifact lineage. MUST be `false`.
    pub drops_artifact_lineage: bool,
    /// Hard invariant: this row never hides artifact freshness. MUST be `false`.
    pub hides_artifact_freshness: bool,
    /// Hard invariant: this row never drops the exported IDs or states. MUST be `false`.
    pub drops_export_ids_or_states: bool,
}

impl M5InteractionSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        declares_mandatory_export_fields(&self.export_fields)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_input_consequence
            && !self.drops_artifact_lineage
            && !self.hides_artifact_freshness
            && !self.drops_export_ids_or_states
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractionVocabularySet {
    /// Surface-family tokens (reused from the run/attempt-header primitive).
    pub surface_families: Vec<String>,
    /// Input-request-kind tokens.
    pub input_kinds: Vec<String>,
    /// Input-disposition tokens.
    pub input_dispositions: Vec<String>,
    /// Input-result-posture tokens.
    pub input_result_postures: Vec<String>,
    /// Input-consequence tokens (reused from the frozen matrix).
    pub input_consequences: Vec<String>,
    /// Artifact-kind tokens.
    pub artifact_kinds: Vec<String>,
    /// Artifact-freshness tokens.
    pub artifact_freshnesses: Vec<String>,
    /// Artifact-trust tokens.
    pub artifact_trust_classes: Vec<String>,
    /// Retention-class tokens (reused from the frozen matrix).
    pub retention_classes: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Run-outcome tokens (reused from the frozen matrix).
    pub outcomes: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Execution-boundary tokens (reused from the frozen matrix).
    pub localities: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5InteractionVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5RunAttemptSurfaceFamily::ALL, |v| v.as_str()),
            input_kinds: tokens(&M5InputRequestKind::ALL, |v| v.as_str()),
            input_dispositions: tokens(&M5InputRequestDisposition::ALL, |v| v.as_str()),
            input_result_postures: tokens(&M5InputResultPosture::ALL, |v| v.as_str()),
            input_consequences: tokens(&INPUT_CONSEQUENCE_ALL, |v| v.as_str()),
            artifact_kinds: tokens(&M5ArtifactKind::ALL, |v| v.as_str()),
            artifact_freshnesses: tokens(&M5ArtifactFreshness::ALL, |v| v.as_str()),
            artifact_trust_classes: tokens(&M5ArtifactTrustClass::ALL, |v| v.as_str()),
            retention_classes: tokens(&RETENTION_ALL, |v| v.as_str()),
            export_fields: tokens(&M5InteractionExportField::ALL, |v| v.as_str()),
            outcomes: tokens(&M5RunOutcome::ALL, |v| v.as_str()),
            truth_modes: tokens(&TRUTH_MODE_ALL, |v| v.as_str()),
            localities: tokens(&LOCALITY_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The input consequences reused from the frozen matrix, in a stable order.
const INPUT_CONSEQUENCE_ALL: [M5InputConsequence; 5] = [
    M5InputConsequence::TimeoutCancelsRun,
    M5InputConsequence::TimeoutAppliesDefault,
    M5InputConsequence::RequiresApproval,
    M5InputConsequence::BlocksUntilAnswered,
    M5InputConsequence::DismissLeavesWaiting,
];

/// The retention classes reused from the frozen matrix, in a stable order.
const RETENTION_ALL: [M5RetentionClass; 5] = [
    M5RetentionClass::RetainedDurable,
    M5RetentionClass::ExpiresScheduled,
    M5RetentionClass::EphemeralSessionOnly,
    M5RetentionClass::EvictedRecoverable,
    M5RetentionClass::EvictedGone,
];

/// The truth classes reused from the frozen matrix, in a stable order.
const TRUTH_MODE_ALL: [M5ExecutionTruthMode; 5] = [
    M5ExecutionTruthMode::Live,
    M5ExecutionTruthMode::Captured,
    M5ExecutionTruthMode::Imported,
    M5ExecutionTruthMode::Planned,
    M5ExecutionTruthMode::ProviderReported,
];

/// The execution boundaries reused from the frozen matrix, in a stable order.
const LOCALITY_ALL: [M5ExecutionLocality; 4] = [
    M5ExecutionLocality::Local,
    M5ExecutionLocality::Remote,
    M5ExecutionLocality::Container,
    M5ExecutionLocality::Managed,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5ExecutionDowngradeTrigger; 9] = [
    M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
    M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
    M5ExecutionDowngradeTrigger::ArtifactLineageLost,
    M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
    M5ExecutionDowngradeTrigger::RerunContextDrift,
    M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
    M5ExecutionDowngradeTrigger::ConnectorLost,
    M5ExecutionDowngradeTrigger::DebugAdapterUnavailable,
    M5ExecutionDowngradeTrigger::SymbolsUnavailable,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractionGovernanceReview {
    /// One primitive carries prompt / artifact-row / CLI-line / export truth on every
    /// surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Dismissed and timed-out requests always resolve to a visible, attributable
    /// posture.
    pub input_consequences_never_silent: bool,
    /// Produced-artifact lineage is preserved after the live pane clears.
    pub artifact_lineage_preserved_after_pane_clears: bool,
    /// Artifact freshness and retention are disclosed before opening or exporting.
    pub artifact_freshness_disclosed_before_action: bool,
    /// Exported evidence preserves the run/attempt IDs and visible states.
    pub exported_evidence_preserves_ids_and_states: bool,
    /// The support / export packet reconstructs interaction truth.
    pub support_export_reconstructs_interaction: bool,
    /// Later M5 rows cannot invent parallel input / artifact vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractionConsumerProjection {
    /// Task / test / request / notebook / AI / publish / preview surfaces all consume
    /// the shared primitive.
    pub execution_surfaces_consume_shared_primitive: bool,
    /// The interaction resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The artifact rows read a single canonical producing-run source.
    pub artifact_rows_read_single_run_source: bool,
    /// Support / export reads a single canonical interaction source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the interaction primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractionReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting interaction audit.
    pub interaction_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ExecutionInteractionPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExecutionInteractionPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5InteractionSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InteractionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InteractionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InteractionConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5InteractionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 input-request / artifact-publish primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionInteractionPrimitivePacket {
    /// Record kind; must equal [`M5_EXECUTION_INTERACTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXECUTION_INTERACTION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5InteractionSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InteractionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InteractionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InteractionConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5InteractionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ExecutionInteractionPrimitivePacket {
    /// Builds an M5 interaction primitive packet from stable-lane input.
    pub fn new(input: M5ExecutionInteractionPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_EXECUTION_INTERACTION_RECORD_KIND.to_owned(),
            schema_version: M5_EXECUTION_INTERACTION_SCHEMA_VERSION,
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

    /// Validates the M5 interaction primitive invariants.
    pub fn validate(&self) -> Vec<M5InteractionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EXECUTION_INTERACTION_RECORD_KIND {
            violations.push(M5InteractionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EXECUTION_INTERACTION_SCHEMA_VERSION {
            violations.push(M5InteractionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InteractionViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 interaction primitive packet serializes"),
        ) {
            violations.push(M5InteractionViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 interaction primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,input_kinds,artifact_kinds,freshness_classes,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.input_kinds, |v| v.as_str()),
                join_tokens(&row.artifact_kinds, |v| v.as_str()),
                join_tokens(&row.freshness_classes, |v| v.as_str()),
                row.example_interactions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Input-Request / Artifact-Publish Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Execution surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5RunAttemptSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Input-request kinds: {}\n",
            self.vocabulary_set.input_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Artifact freshness classes: {}\n",
            self.vocabulary_set.artifact_freshnesses.join(", ")
        ));
        out.push_str("\n## Execution surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_interactions.len()
            ));
            for case in &row.example_interactions {
                let prompt = match &case.resolved.input_prompt {
                    Some(prompt) => format!(
                        "input {} → {}",
                        prompt.disposition.as_str(),
                        prompt.result_posture.as_str()
                    ),
                    None => "no input request".to_owned(),
                };
                out.push_str(&format!(
                    "    - `{}` → run `{}` [{}], {} ({} artifact(s))\n",
                    case.resolved.interaction_id,
                    case.resolved.run_ref,
                    case.resolved.export.run_outcome.as_str(),
                    prompt,
                    case.resolved.artifact_rows.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 interaction export.
#[derive(Debug)]
pub enum M5InteractionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InteractionViolation>),
}

impl fmt::Display for M5InteractionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 interaction primitive export parse failed: {error}"
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
                    "m5 interaction primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InteractionArtifactError {}

/// Validation failures emitted by [`M5ExecutionInteractionPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InteractionViolation {
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
    /// A required execution surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked interaction cases.
    ExampleInteractionsMissing,
    /// A worked interaction case does not match a fresh resolve of its input.
    ExampleInteractionDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves a dismissed / timed-out request resolves to a visible,
    /// attributable posture (AC1).
    InputConsequenceUnproven,
    /// No worked case proves produced-artifact lineage preserved after the pane clears
    /// (AC2).
    ArtifactLineageUnproven,
    /// No worked case proves artifact freshness disclosed before action, or the
    /// freshness classes are not fully covered (AC3).
    ArtifactFreshnessUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5InteractionViolation {
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
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleInteractionsMissing => "example_interactions_missing",
            Self::ExampleInteractionDrift => "example_interaction_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::InputConsequenceUnproven => "input_consequence_unproven",
            Self::ArtifactLineageUnproven => "artifact_lineage_unproven",
            Self::ArtifactFreshnessUnproven => "artifact_freshness_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 interaction export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_execution_interaction_export(
) -> Result<M5ExecutionInteractionPrimitivePacket, M5InteractionArtifactError> {
    let packet: M5ExecutionInteractionPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-input-request-artifact-publish-primitive-proof/support_export.json"
    )))
    .map_err(M5InteractionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InteractionArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EXECUTION_INTERACTION_SCHEMA_REF,
        M5_EXECUTION_INTERACTION_DOC_REF,
        M5_EXECUTION_INTERACTION_COMPONENT_MATRIX_REF,
        M5_EXECUTION_INTERACTION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InteractionViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5InteractionViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    let present: BTreeSet<M5RunAttemptSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5RunAttemptSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5InteractionViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5InteractionViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5InteractionViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5InteractionViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5InteractionViolation::ConsumerSurfacesMissing);
        }
        if row.example_interactions.is_empty() {
            violations.push(M5InteractionViolation::ExampleInteractionsMissing);
        }
        if row
            .example_interactions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5InteractionViolation::ExampleInteractionDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5InteractionViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated across the matrix: a dismissed /
/// timed-out request resolves to a visible posture (AC1), produced-artifact lineage is
/// preserved even when retention is evicted (AC2), and every artifact discloses its
/// freshness before action with every freshness class covered (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    let cases: Vec<&M5ResolvedExecutionInteraction> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_interactions.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case has a prompt with a negative disposition resolved to a
    // visible dismissal / timeout consequence, and every case discloses its input
    // consequence and keeps identity consistent.
    let input_proven = cases.iter().any(|resolved| {
        resolved.input_prompt.as_ref().is_some_and(|prompt| {
            prompt.disposition.is_negative()
                && prompt.result_posture.is_dismissal_or_timeout_consequence()
        })
    }) && cases
        .iter()
        .all(|resolved| resolved.discloses_input_consequence() && resolved.identity_consistent());
    if !input_proven {
        violations.push(M5InteractionViolation::InputConsequenceUnproven);
    }

    // AC2: at least one case has an evicted-retention artifact that stays attributable,
    // and every artifact preserves lineage.
    let lineage_proven = cases.iter().any(|resolved| {
        resolved
            .artifact_rows
            .iter()
            .any(|row| row.retention.is_evicted() && row.is_attributable)
    }) && cases
        .iter()
        .all(|resolved| resolved.preserves_artifact_lineage());
    if !lineage_proven {
        violations.push(M5InteractionViolation::ArtifactLineageUnproven);
    }

    // AC3: every case discloses artifact freshness before action, and every freshness
    // class appears at least once across the matrix.
    let mut freshness_seen: BTreeSet<M5ArtifactFreshness> = BTreeSet::new();
    for resolved in &cases {
        for row in &resolved.artifact_rows {
            freshness_seen.insert(row.freshness);
        }
    }
    let freshness_proven = cases
        .iter()
        .all(|resolved| resolved.discloses_artifact_freshness())
        && M5ArtifactFreshness::ALL
            .iter()
            .all(|freshness| freshness_seen.contains(freshness));
    if !freshness_proven {
        violations.push(M5InteractionViolation::ArtifactFreshnessUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.input_consequences_never_silent,
        review.artifact_lineage_preserved_after_pane_clears,
        review.artifact_freshness_disclosed_before_action,
        review.exported_evidence_preserves_ids_and_states,
        review.support_export_reconstructs_interaction,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5InteractionViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.execution_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.artifact_rows_read_single_run_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5InteractionViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5ExecutionInteractionPrimitivePacket,
    violations: &mut Vec<M5InteractionViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.interaction_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InteractionViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
