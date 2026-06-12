//! Canonical runtime-continuity surface qualification and evidence index.
//!
//! This module certifies the claimed queue-fairness, restore-fidelity, and
//! terminal-boundary posture that Aureline currently evidences for runtime-heavy
//! M5 surfaces. It does not mint a second scheduler or terminal model. Instead,
//! it binds the checked `queue_session_terminal_governance` packet and the
//! related protocol / restore proof corpus into one auto-narrowing
//! qualification record and one canonical evidence index that docs/help,
//! Help/About, support playbooks, and public-truth consumers can cite
//! directly.
//!
//! The packet keeps three promises aligned with the runtime architecture
//! contract:
//!
//! - claimed profiles may remain `stable` only when queue-fairness,
//!   restore-no-hidden-rerun, terminal protocol/clipboard, and
//!   transcript/shared-control proof stays current;
//! - profiles that still lack a current runtime-continuity proof narrow
//!   automatically instead of inheriting generic desktop continuity claims; and
//! - docs/help, Help/About, support playbooks, and public-truth consumers all
//!   bind to the same evidence index entry set instead of restating runtime
//!   continuity posture in parallel text.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::queue_session_terminal_governance::{
    current_queue_session_terminal_governance_packet, BACKGROUND_QUEUE_CONTRACT_DOC_REF,
    CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF, QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF,
};

/// Stable record kind carried by [`RuntimeContinuitySurfaceQualificationPacket`].
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND: &str =
    "runtime_continuity_surface_qualification";

/// Schema version for runtime-continuity surface qualification packets.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/runtime/runtime-continuity-surface-qualification.schema.json";

/// Repo-relative path of the certification contract doc.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_DOC_REF: &str =
    "docs/runtime/runtime-continuity-surface-qualification.md";

/// Repo-relative path of the help-facing index summary.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_HELP_REF: &str =
    "docs/help/runtime-continuity-surface-qualification.md";

/// Repo-relative path of the protected fixture directory.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/runtime/runtime-continuity-surface-qualification";

/// Repo-relative path of the checked support-export artifact.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/runtime/runtime-continuity-surface-qualification/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SUMMARY_REF: &str =
    "artifacts/runtime/runtime-continuity-surface-qualification.md";

/// Claimed runtime-continuity profile surfaced to downstream consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeContinuityProfile {
    /// Desktop-local runtime with no managed or browser authority requirement.
    LocalOnly,
    /// Desktop runtime whose docs/help and support surfaces may be mirrored.
    Mirrored,
    /// Managed or remote-backed runtime whose active boundary stays visible.
    Managed,
    /// Browser or companion handoff posture that must not imply live terminal
    /// authority without a narrower proof packet.
    BrowserHandoff,
}

impl RuntimeContinuityProfile {
    /// Every claimed profile in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::Mirrored,
        Self::Managed,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Mirrored => "mirrored",
            Self::Managed => "managed",
            Self::BrowserHandoff => "browser_handoff",
        }
    }
}

/// Proof class a claimed profile requires before it can stay fully qualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeContinuityProofClass {
    /// Queue identities, collapse behavior, protected-path fitness, and fairness.
    QueueFairness,
    /// Restore fidelity and no-hidden-rerun proof.
    RestoreNoHiddenRerun,
    /// Terminal protocol, clipboard, and boundary labeling proof.
    TerminalProtocolClipboard,
    /// Transcript-export, session-continuity, and shared-control proof.
    TranscriptAndSharedControl,
    /// Browser or companion handoff continuity proof.
    BrowserHandoffContinuity,
}

impl RuntimeContinuityProofClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueueFairness => "queue_fairness",
            Self::RestoreNoHiddenRerun => "restore_no_hidden_rerun",
            Self::TerminalProtocolClipboard => "terminal_protocol_clipboard",
            Self::TranscriptAndSharedControl => "transcript_and_shared_control",
            Self::BrowserHandoffContinuity => "browser_handoff_continuity",
        }
    }
}

/// Currency of the checked proof packet for one claimed profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCurrency {
    /// Proof is current on the claimed profile.
    Current,
    /// Proof packet is present but stale or structurally regressed.
    Stale,
}

/// Lifecycle label surfaced to Help/About, support, and public-truth consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeContinuityLabel {
    /// Fully qualified on the claimed profile.
    Stable,
    /// Narrowed below the cutline but still partially claimable.
    Beta,
    /// Preview-only because a required proof family is still absent.
    Preview,
}

impl RuntimeContinuityLabel {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
        }
    }

    const fn rank(self) -> u8 {
        match self {
            Self::Stable => 3,
            Self::Beta => 2,
            Self::Preview => 1,
        }
    }
}

/// Reason a profile narrowed below its claim ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeContinuityNarrowReason {
    /// Upstream queue/restore/terminal proof is stale or structurally regressed.
    ProofPacketStale,
    /// Browser or companion handoff continuity still lacks current proof.
    BrowserHandoffContinuityUnqualified,
}

impl RuntimeContinuityNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofPacketStale => "proof_packet_stale",
            Self::BrowserHandoffContinuityUnqualified => "browser_handoff_continuity_unqualified",
        }
    }
}

/// Claimed runtime-continuity row for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeContinuityProfileRow {
    /// Claimed profile this row covers.
    pub profile: RuntimeContinuityProfile,
    /// Hard claim ceiling the row may never exceed.
    pub claim_label: RuntimeContinuityLabel,
    /// Effective label later consumers must render.
    pub displayed_label: RuntimeContinuityLabel,
    /// Currency of the proof behind this row.
    pub evidence_currency: EvidenceCurrency,
    /// Checked proof refs the row cites directly.
    pub packet_refs: Vec<String>,
    /// Proof classes required for the claim on this profile.
    pub required_proofs: Vec<RuntimeContinuityProofClass>,
    /// Proof classes currently satisfied on this profile.
    pub satisfied_proofs: Vec<RuntimeContinuityProofClass>,
    /// Typed reasons the row narrowed below its claim ceiling.
    pub narrow_reasons: Vec<RuntimeContinuityNarrowReason>,
    /// Review-safe scope summary for this profile.
    pub scope_summary: String,
}

/// Canonical evidence-index entry for one surfaced profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeContinuityEvidenceIndexEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Profile this entry backs.
    pub profile: RuntimeContinuityProfile,
    /// Boundary schema ref for the shared index.
    pub schema_ref: String,
    /// Contract doc ref for the shared index.
    pub doc_ref: String,
    /// Artifact ref consumers cite directly.
    pub artifact_ref: String,
    /// Negative-drill refs that prove automatic narrowing.
    pub drill_refs: Vec<String>,
    /// Review-safe support summary for downstream consumers.
    pub support_summary: String,
}

/// Downstream consumer that must reuse the shared runtime-continuity index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeContinuityEvidenceConsumer {
    /// Docs/help page.
    DocsHelp,
    /// Help/About and provenance surfaces.
    HelpAbout,
    /// Support-playbook and support-export consumers.
    SupportPlaybook,
    /// Public-truth publication.
    PublicTruth,
}

impl RuntimeContinuityEvidenceConsumer {
    /// Every downstream consumer in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocsHelp,
        Self::HelpAbout,
        Self::SupportPlaybook,
        Self::PublicTruth,
    ];
}

/// One consumer binding over the shared runtime-continuity evidence index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeContinuityEvidenceConsumerBinding {
    /// Downstream consumer this binding covers.
    pub consumer: RuntimeContinuityEvidenceConsumer,
    /// Repo-relative consumer ref.
    pub consumer_ref: String,
    /// Profiles this consumer surfaces directly.
    pub profile_refs: Vec<RuntimeContinuityProfile>,
    /// Consumer uses the shared evidence index instead of restating posture.
    pub uses_shared_index: bool,
    /// Consumer renders `displayed_label` rather than the claim ceiling.
    pub shows_displayed_label: bool,
}

/// Constructor input for [`RuntimeContinuitySurfaceQualificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContinuitySurfaceQualificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-profile rows.
    pub profile_rows: Vec<RuntimeContinuityProfileRow>,
    /// Canonical evidence-index entries.
    pub evidence_index: Vec<RuntimeContinuityEvidenceIndexEntry>,
    /// Downstream consumer bindings.
    pub consumer_bindings: Vec<RuntimeContinuityEvidenceConsumerBinding>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet mint timestamp.
    pub minted_at: String,
    /// Review-safe support summary for the packet.
    pub support_summary: String,
}

/// Export-safe runtime-continuity qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeContinuitySurfaceQualificationPacket {
    /// Record kind; must equal [`RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-profile rows.
    pub profile_rows: Vec<RuntimeContinuityProfileRow>,
    /// Canonical evidence-index entries.
    pub evidence_index: Vec<RuntimeContinuityEvidenceIndexEntry>,
    /// Downstream consumer bindings.
    pub consumer_bindings: Vec<RuntimeContinuityEvidenceConsumerBinding>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet mint timestamp.
    pub minted_at: String,
    /// Review-safe support summary for the packet.
    pub support_summary: String,
}

impl RuntimeContinuitySurfaceQualificationPacket {
    /// Builds a packet from stable profile rows and evidence bindings.
    pub fn new(input: RuntimeContinuitySurfaceQualificationPacketInput) -> Self {
        Self {
            record_kind: RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            profile_rows: input.profile_rows,
            evidence_index: input.evidence_index,
            consumer_bindings: input.consumer_bindings,
            source_contract_refs: input.source_contract_refs,
            minted_at: input.minted_at,
            support_summary: input.support_summary,
        }
    }

    /// Validates the runtime-continuity qualification invariants.
    pub fn validate(&self) -> Vec<RuntimeContinuitySurfaceQualificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(RuntimeContinuitySurfaceQualificationViolation::WrongRecordKind);
        }
        if self.schema_version != RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(RuntimeContinuitySurfaceQualificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.support_summary.trim().is_empty()
        {
            violations.push(RuntimeContinuitySurfaceQualificationViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(RuntimeContinuitySurfaceQualificationViolation::MissingSourceContracts);
        }

        for required in RuntimeContinuityProfile::ALL {
            if !self.profile_rows.iter().any(|row| row.profile == required) {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::ProfileCoverageIncomplete,
                );
            }
            if !self
                .evidence_index
                .iter()
                .any(|entry| entry.profile == required)
            {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::EvidenceIndexCoverageIncomplete,
                );
            }
        }

        for row in &self.profile_rows {
            if row.packet_refs.is_empty()
                || row.required_proofs.is_empty()
                || row.scope_summary.trim().is_empty()
            {
                violations
                    .push(RuntimeContinuitySurfaceQualificationViolation::ProfileRowIncomplete);
            }

            let required: BTreeSet<_> = row.required_proofs.iter().copied().collect();
            let satisfied: BTreeSet<_> = row.satisfied_proofs.iter().copied().collect();
            if !satisfied.is_subset(&required) {
                violations
                    .push(RuntimeContinuitySurfaceQualificationViolation::ProofCoverageMismatch);
            }

            let missing_required = required.difference(&satisfied).next().is_some();
            let expected_label = derive_displayed_label(row.evidence_currency, missing_required);
            if row.displayed_label != expected_label
                || row.displayed_label.rank() > row.claim_label.rank()
            {
                violations
                    .push(RuntimeContinuitySurfaceQualificationViolation::DisplayedLabelMismatch);
            }

            if row.displayed_label == row.claim_label && !row.narrow_reasons.is_empty() {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::NarrowReasonsInconsistent,
                );
            }
            if row.displayed_label.rank() < row.claim_label.rank() && row.narrow_reasons.is_empty()
            {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::NarrowReasonsInconsistent,
                );
            }
        }

        for entry in &self.evidence_index {
            if entry.entry_id.trim().is_empty()
                || entry.schema_ref.trim().is_empty()
                || entry.doc_ref.trim().is_empty()
                || entry.artifact_ref.trim().is_empty()
                || entry.support_summary.trim().is_empty()
                || entry.drill_refs.is_empty()
            {
                violations
                    .push(RuntimeContinuitySurfaceQualificationViolation::EvidenceIndexRefMismatch);
            }
        }

        for required in RuntimeContinuityEvidenceConsumer::ALL {
            let Some(binding) = self
                .consumer_bindings
                .iter()
                .find(|binding| binding.consumer == required)
            else {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::ConsumerBindingIncomplete,
                );
                continue;
            };
            if binding.consumer_ref.trim().is_empty()
                || !binding.uses_shared_index
                || !binding.shows_displayed_label
            {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::ConsumerBindingIncomplete,
                );
            }
            let covered: BTreeSet<_> = binding.profile_refs.iter().copied().collect();
            let required: BTreeSet<_> = RuntimeContinuityProfile::ALL.into_iter().collect();
            if covered != required {
                violations.push(
                    RuntimeContinuitySurfaceQualificationViolation::ConsumerBindingCoverageMismatch,
                );
            }
        }

        violations
    }
}

/// Validation failures for runtime-continuity qualification packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeContinuitySurfaceQualificationViolation {
    /// Packet record kind drifted.
    WrongRecordKind,
    /// Packet schema version drifted.
    WrongSchemaVersion,
    /// Packet identity or summary fields are incomplete.
    MissingIdentity,
    /// Canonical source contracts are missing.
    MissingSourceContracts,
    /// One or more required profiles are missing.
    ProfileCoverageIncomplete,
    /// One profile row omitted required refs or summaries.
    ProfileRowIncomplete,
    /// Satisfied proofs do not line up with required proofs.
    ProofCoverageMismatch,
    /// Displayed label widened beyond the derivable state.
    DisplayedLabelMismatch,
    /// Narrow reasons do not match whether the row is narrowed.
    NarrowReasonsInconsistent,
    /// Evidence-index entries are missing or incomplete.
    EvidenceIndexCoverageIncomplete,
    /// One evidence-index entry omitted required refs or drills.
    EvidenceIndexRefMismatch,
    /// A required consumer binding is missing or incomplete.
    ConsumerBindingIncomplete,
    /// A consumer binding does not cover every profile.
    ConsumerBindingCoverageMismatch,
}

impl RuntimeContinuitySurfaceQualificationViolation {
    /// Stable token exported in validation failures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ProfileCoverageIncomplete => "profile_coverage_incomplete",
            Self::ProfileRowIncomplete => "profile_row_incomplete",
            Self::ProofCoverageMismatch => "proof_coverage_mismatch",
            Self::DisplayedLabelMismatch => "displayed_label_mismatch",
            Self::NarrowReasonsInconsistent => "narrow_reasons_inconsistent",
            Self::EvidenceIndexCoverageIncomplete => "evidence_index_coverage_incomplete",
            Self::EvidenceIndexRefMismatch => "evidence_index_ref_mismatch",
            Self::ConsumerBindingIncomplete => "consumer_binding_incomplete",
            Self::ConsumerBindingCoverageMismatch => "consumer_binding_coverage_mismatch",
        }
    }
}

/// Errors raised while loading the checked support-export artifact.
#[derive(Debug)]
pub enum RuntimeContinuitySurfaceQualificationArtifactError {
    /// Checked artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// Parsed artifact failed validation.
    Validation(Vec<RuntimeContinuitySurfaceQualificationViolation>),
}

impl fmt::Display for RuntimeContinuitySurfaceQualificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "runtime continuity support export parse failed: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "runtime continuity support export failed validation: {:?}",
                violations
            ),
        }
    }
}

impl Error for RuntimeContinuitySurfaceQualificationArtifactError {}

/// Reads and validates the checked support-export artifact.
pub fn current_runtime_continuity_surface_qualification_export() -> Result<
    RuntimeContinuitySurfaceQualificationPacket,
    RuntimeContinuitySurfaceQualificationArtifactError,
> {
    let payload = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/runtime-continuity-surface-qualification/support_export.json"
    ));
    let packet: RuntimeContinuitySurfaceQualificationPacket = serde_json::from_str(payload)
        .map_err(RuntimeContinuitySurfaceQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RuntimeContinuitySurfaceQualificationArtifactError::Validation(violations))
    }
}

/// Returns the canonical seeded runtime-continuity qualification packet.
pub fn seeded_runtime_continuity_surface_qualification_packet(
) -> RuntimeContinuitySurfaceQualificationPacket {
    let governance_packet = current_queue_session_terminal_governance_packet();
    let governance_current = governance_packet.validate().is_empty();
    let common_refs = vec![
        QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF.to_owned(),
        QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF.to_owned(),
        "fixtures/terminal/protocol_corpus_alpha/manifest.json".to_owned(),
        "fixtures/terminal/restore_cases/failure_drill_lost_transport_becomes_transcript.json"
            .to_owned(),
        "fixtures/terminal/paste_boundary_alpha/high_risk_remote_multiline_review.json".to_owned(),
    ];
    let common_proofs = vec![
        RuntimeContinuityProofClass::QueueFairness,
        RuntimeContinuityProofClass::RestoreNoHiddenRerun,
        RuntimeContinuityProofClass::TerminalProtocolClipboard,
        RuntimeContinuityProofClass::TranscriptAndSharedControl,
    ];
    let common_satisfied = if governance_current {
        common_proofs.clone()
    } else {
        Vec::new()
    };

    let mut profile_rows = vec![
        profile_row(
            RuntimeContinuityProfile::LocalOnly,
            EvidenceCurrency::from_current(governance_current),
            common_refs.clone(),
            common_proofs.clone(),
            common_satisfied.clone(),
            "Local desktop runtime continuity keeps queue identity, fairness, restore fidelity, and terminal boundary truth current across the claimed notebook, data, pipeline, preview, profiler, docs recall, sync, incident, and infrastructure surfaces.",
        ),
        profile_row(
            RuntimeContinuityProfile::Mirrored,
            EvidenceCurrency::from_current(governance_current),
            common_refs.clone(),
            common_proofs.clone(),
            common_satisfied.clone(),
            "Mirrored docs/help and support publication reuse the same runtime queue, restore, transcript-export, and terminal-boundary proof instead of advertising a wider continuity claim than the checked desktop runtime packet supports.",
        ),
        profile_row(
            RuntimeContinuityProfile::Managed,
            EvidenceCurrency::from_current(governance_current),
            common_refs.clone(),
            common_proofs.clone(),
            common_satisfied.clone(),
            "Managed and remote-backed runtime continuity keeps foreground budget protection, checkpointed retry, honest restore, boundary labeling, and shared-control audit truth visible instead of letting managed lanes bypass the protected-path contract.",
        ),
    ];

    let mut browser_required = common_proofs.clone();
    browser_required.push(RuntimeContinuityProofClass::BrowserHandoffContinuity);
    profile_rows.push(profile_row(
        RuntimeContinuityProfile::BrowserHandoff,
        EvidenceCurrency::from_current(governance_current),
        vec![
            QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF.to_owned(),
            QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF.to_owned(),
            RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_HELP_REF.to_owned(),
        ],
        browser_required,
        common_satisfied,
        "Browser and companion handoff surfaces reuse the desktop runtime evidence index only as a narrowed continuity disclosure: layout and handoff context may be restored, but live terminal authority, clipboard-write posture, and rerun semantics remain underqualified until a browser-handoff runtime packet is checked in.",
    ));

    RuntimeContinuitySurfaceQualificationPacket::new(
        RuntimeContinuitySurfaceQualificationPacketInput {
            packet_id: "runtime-continuity-surface-qualification:stable:0001".to_owned(),
            certification_label: "Runtime Continuity Surface Qualification".to_owned(),
            profile_rows,
            evidence_index: vec![
                evidence_entry(
                    "evidence:runtime-continuity:local-only",
                    RuntimeContinuityProfile::LocalOnly,
                    "Current queue identity, fairness, restore, transcript-export, and terminal protocol proof for desktop-local runtime continuity.",
                ),
                evidence_entry(
                    "evidence:runtime-continuity:mirrored",
                    RuntimeContinuityProfile::Mirrored,
                    "Current mirrored docs/help and support publication row that quotes the same runtime continuity packet without widening the desktop claim.",
                ),
                evidence_entry(
                    "evidence:runtime-continuity:managed",
                    RuntimeContinuityProfile::Managed,
                    "Current managed-boundary runtime continuity row covering protected-path fairness, honest restore, transcript export, and shared-control truth.",
                ),
                evidence_entry(
                    "evidence:runtime-continuity:browser-handoff",
                    RuntimeContinuityProfile::BrowserHandoff,
                    "Narrowed browser-handoff continuity row proving public-truth consumers quote the preview posture instead of implying live terminal authority from desktop evidence.",
                ),
            ],
            consumer_bindings: vec![
                consumer_binding(
                    RuntimeContinuityEvidenceConsumer::DocsHelp,
                    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_HELP_REF,
                ),
                consumer_binding(
                    RuntimeContinuityEvidenceConsumer::HelpAbout,
                    "docs/help/help_about_truth_source.md",
                ),
                consumer_binding(
                    RuntimeContinuityEvidenceConsumer::SupportPlaybook,
                    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_ARTIFACT_REF,
                ),
                consumer_binding(
                    RuntimeContinuityEvidenceConsumer::PublicTruth,
                    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_ARTIFACT_REF,
                ),
            ],
            source_contract_refs: vec![
                QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF.to_owned(),
                QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF.to_owned(),
                QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF.to_owned(),
                BACKGROUND_QUEUE_CONTRACT_DOC_REF.to_owned(),
                CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF.to_owned(),
                RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_REF.to_owned(),
            ],
            minted_at: "2026-06-12T18:15:00Z".to_owned(),
            support_summary: "Canonical runtime-continuity qualification packet and evidence index for queue fairness, restore fidelity, terminal protocol, transcript export, and shared-control truth across claimed M5 profiles.".to_owned(),
        },
    )
}

fn profile_row(
    profile: RuntimeContinuityProfile,
    evidence_currency: EvidenceCurrency,
    packet_refs: Vec<String>,
    required_proofs: Vec<RuntimeContinuityProofClass>,
    satisfied_proofs: Vec<RuntimeContinuityProofClass>,
    scope_summary: impl Into<String>,
) -> RuntimeContinuityProfileRow {
    let required: BTreeSet<_> = required_proofs.iter().copied().collect();
    let satisfied: BTreeSet<_> = satisfied_proofs.iter().copied().collect();
    let missing_required = required.difference(&satisfied).next().is_some();
    let displayed_label = derive_displayed_label(evidence_currency, missing_required);
    let narrow_reasons = derive_narrow_reasons(profile, evidence_currency, &required, &satisfied);

    RuntimeContinuityProfileRow {
        profile,
        claim_label: RuntimeContinuityLabel::Stable,
        displayed_label,
        evidence_currency,
        packet_refs,
        required_proofs,
        satisfied_proofs,
        narrow_reasons,
        scope_summary: scope_summary.into(),
    }
}

fn evidence_entry(
    entry_id: impl Into<String>,
    profile: RuntimeContinuityProfile,
    support_summary: impl Into<String>,
) -> RuntimeContinuityEvidenceIndexEntry {
    RuntimeContinuityEvidenceIndexEntry {
        entry_id: entry_id.into(),
        profile,
        schema_ref: RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_REF.to_owned(),
        doc_ref: RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_DOC_REF.to_owned(),
        artifact_ref: RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_ARTIFACT_REF.to_owned(),
        drill_refs: vec![
            format!(
                "{}/browser_handoff_widened_packet.json",
                RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_FIXTURE_DIR
            ),
            format!(
                "{}/stale_managed_profile_packet.json",
                RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_FIXTURE_DIR
            ),
        ],
        support_summary: support_summary.into(),
    }
}

fn consumer_binding(
    consumer: RuntimeContinuityEvidenceConsumer,
    consumer_ref: impl Into<String>,
) -> RuntimeContinuityEvidenceConsumerBinding {
    RuntimeContinuityEvidenceConsumerBinding {
        consumer,
        consumer_ref: consumer_ref.into(),
        profile_refs: RuntimeContinuityProfile::ALL.to_vec(),
        uses_shared_index: true,
        shows_displayed_label: true,
    }
}

fn derive_displayed_label(
    evidence_currency: EvidenceCurrency,
    missing_required: bool,
) -> RuntimeContinuityLabel {
    match (evidence_currency, missing_required) {
        (EvidenceCurrency::Current, false) => RuntimeContinuityLabel::Stable,
        (EvidenceCurrency::Current, true) => RuntimeContinuityLabel::Preview,
        (EvidenceCurrency::Stale, _) => RuntimeContinuityLabel::Beta,
    }
}

fn derive_narrow_reasons(
    profile: RuntimeContinuityProfile,
    evidence_currency: EvidenceCurrency,
    required: &BTreeSet<RuntimeContinuityProofClass>,
    satisfied: &BTreeSet<RuntimeContinuityProofClass>,
) -> Vec<RuntimeContinuityNarrowReason> {
    let mut reasons = Vec::new();
    if matches!(evidence_currency, EvidenceCurrency::Stale) {
        reasons.push(RuntimeContinuityNarrowReason::ProofPacketStale);
    }
    if profile == RuntimeContinuityProfile::BrowserHandoff
        && required.contains(&RuntimeContinuityProofClass::BrowserHandoffContinuity)
        && !satisfied.contains(&RuntimeContinuityProofClass::BrowserHandoffContinuity)
    {
        reasons.push(RuntimeContinuityNarrowReason::BrowserHandoffContinuityUnqualified);
    }
    reasons
}

impl EvidenceCurrency {
    const fn from_current(current: bool) -> Self {
        if current {
            Self::Current
        } else {
            Self::Stale
        }
    }
}
