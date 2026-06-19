//! One typed proposal/session ledger for every mutating quality action on M5
//! code-quality and runtime-finding surfaces.
//!
//! The governed record family in [`crate::quality`] already owns one
//! [`QualityActionProposal`] and one [`QualitySession`] type whose
//! preview/apply/validate/revert contract is derived from a proposal's safety,
//! scope, and policy posture. That base module answers "what would this single
//! mutation do, and may it auto-apply?"; it does not, on its own, prove that
//! *every* mutating quality route on the claimed M5 lanes — format-on-type,
//! format-on-save, manual quick-fix and fix-all, headless lint autofix,
//! review-apply baseline/suppression governance, and imported-scan comparison —
//! actually routes through that one contract rather than through divergent,
//! provider-specific status text.
//!
//! This module owns that proof. A [`QualitySessionLedgerPacket`] bundles a set of
//! [`QualitySession`] records (each carrying its own [`QualityActionProposal`]
//! set) that together span every required trigger path, every action class, every
//! safety class, and a representative spread of outcomes, then projects them onto
//! the UI, Problems, review, CLI, and support surfaces and rolls them into a
//! metadata-only support export.
//!
//! The four guarantees this delivery owns:
//!
//! 1. **Every mutating action is a typed proposal inside a typed session.** A
//!    mutating quality route is never a bare side effect: it is a
//!    [`QualityActionProposal`] with an explicit scope, safety class, preview
//!    requirement, checkpoint ref, and rollback boundary, serialized inside a
//!    [`QualitySession`] that names its trigger, effective profile, execution
//!    context, validation refs, and rollback note.
//! 2. **One result vocabulary across every path.** On-type, on-save, manual,
//!    headless, review-apply, and import-comparison sessions all report through the
//!    same typed [`QualitySessionOutcomeClass`] and the same per-proposal class
//!    tokens, never a divergent per-provider status string. The validator refuses a
//!    record whose serialized token disagrees with its typed class.
//! 3. **Generated, protected, and policy paths reuse the same lifecycle.** A
//!    proposal that touches a generated family, a lockfile/manifest, a protected
//!    path, or a policy-scoped mutation cannot claim a weaker mutation bar because
//!    it "looks like formatting": it must require preview-first or block apply, and
//!    it carries a real rollback boundary rather than `no_mutation`.
//! 4. **Rollback notes, validation refs, and safety classes stay inspectable.**
//!    Every required surface receives a projection that exposes the session's
//!    outcome, the safety classes in play, the rollback boundary note, and the
//!    validation refs — so a user can audit what ran, why, and how to undo it from
//!    the editor, Problems, review, CLI, or a support export alike.
//!
//! [`QualitySessionLedgerPacket::validate`] refuses a packet that omits a required
//! trigger path or action class, lets a result token diverge from its typed class,
//! grants a generated/protected mutation a weaker bar, drops a mutating proposal's
//! rollback boundary, hides the proposal/session truth from a required surface, or
//! serializes a lossy or raw-content-bearing support export.
//!
//! Raw patches, raw source bytes, raw tool arguments, raw provider payloads,
//! credentials, and raw logs never cross this boundary; the packet carries only
//! typed class tokens, booleans, opaque ids, counts, and redaction-aware
//! reviewable summaries.
//!
//! The boundary schemas are
//! [`schemas/quality/quality_action_proposal.schema.json`](../../../../schemas/quality/quality_action_proposal.schema.json),
//! [`schemas/quality/quality_session.schema.json`](../../../../schemas/quality/quality_session.schema.json),
//! and the ledger schema
//! [`schemas/quality/quality-session-ledger.schema.json`](../../../../schemas/quality/quality-session-ledger.schema.json).
//! The reviewer-facing doc is
//! [`docs/help/quality-actions-and-sessions.md`](../../../../docs/help/quality-actions-and-sessions.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/quality-actions-and-sessions/`](../../../../fixtures/quality/m5/quality-actions-and-sessions/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::DiagnosticRedactionClass;
use crate::quality::{
    QualityActionClass, QualityApplyPostureClass, QualityMutationScopeClass,
    QualityRollbackBoundaryClass, QualitySafetyClass, QualitySession, QualitySessionOutcomeClass,
    QualitySessionTriggerClass, QualitySurfaceClass,
};

/// Stable record-kind tag carried by [`QualitySessionLedgerPacket`].
pub const M5_QUALITY_SESSION_LEDGER_RECORD_KIND: &str = "m5_quality_session_ledger";

/// Stable record-kind tag for a [`QualitySessionSurfaceProjection`].
pub const M5_QUALITY_SESSION_SURFACE_PROJECTION_RECORD_KIND: &str =
    "m5_quality_session_surface_projection";

/// Stable record-kind tag for a [`QualitySessionLedgerSupportExport`].
pub const M5_QUALITY_SESSION_LEDGER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_quality_session_ledger_support_export";

/// Schema version for the M5 quality-session ledger.
pub const M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the proposal boundary schema.
pub const QUALITY_ACTION_PROPOSAL_SCHEMA_REF: &str =
    "schemas/quality/quality_action_proposal.schema.json";

/// Repo-relative path of the session boundary schema.
pub const QUALITY_SESSION_SCHEMA_REF: &str = "schemas/quality/quality_session.schema.json";

/// Repo-relative path of the ledger boundary schema.
pub const M5_QUALITY_SESSION_LEDGER_SCHEMA_REF: &str =
    "schemas/quality/quality-session-ledger.schema.json";

/// Repo-relative path of the reviewer-facing doc.
pub const M5_QUALITY_SESSION_LEDGER_DOC_REF: &str = "docs/help/quality-actions-and-sessions.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_QUALITY_SESSION_LEDGER_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/quality-session-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_QUALITY_SESSION_LEDGER_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/quality-session-proof/support_export.md";

/// Trigger paths a claimed M5 quality-action ledger must demonstrate so on-type,
/// on-save, manual, headless, and review-apply routes prove they share one
/// vocabulary, and the imported-scan comparison route proves it stays read-only.
pub const REQUIRED_TRIGGER_PATHS: [QualitySessionTriggerClass; 6] = [
    QualitySessionTriggerClass::OnType,
    QualitySessionTriggerClass::OnSave,
    QualitySessionTriggerClass::ManualCommand,
    QualitySessionTriggerClass::CliHeadless,
    QualitySessionTriggerClass::Review,
    QualitySessionTriggerClass::ImportComparison,
];

/// Action classes a claimed M5 quality-action ledger must represent as typed
/// proposals, covering the format, organize-imports, quick-fix, fix-all, lint
/// autofix, and suppression/baseline-update flows named by the contract.
pub const REQUIRED_ACTION_CLASSES: [QualityActionClass; 8] = [
    QualityActionClass::FormatRange,
    QualityActionClass::FormatDocument,
    QualityActionClass::OrganizeImports,
    QualityActionClass::QuickFixSingle,
    QualityActionClass::FixAllRule,
    QualityActionClass::LintAutofixBatch,
    QualityActionClass::SuppressionProposal,
    QualityActionClass::BaselineUpdate,
];

/// Consumer surfaces that must expose proposal/session truth so a user can audit
/// what ran, the safety class, the rollback note, and the validation refs.
pub const QUALITY_ACTION_EXPOSURE_SURFACES: [QualitySurfaceClass; 5] =
    QualitySurfaceClass::required_profile_inspection_surfaces();

/// A cross-surface projection of one session that exposes its outcome, safety
/// classes, rollback note, and validation refs without raw content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualitySessionSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Session id projected.
    pub session_id: String,
    /// Surface consuming the projection.
    pub surface_class: QualitySurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Session trigger copied from the session.
    pub trigger_class: QualitySessionTriggerClass,
    /// Stable trigger token.
    pub trigger_token: String,
    /// Session outcome copied from the session.
    pub outcome_class: QualitySessionOutcomeClass,
    /// Stable outcome token.
    pub outcome_token: String,
    /// Number of proposals serialized in the session.
    pub proposal_count: usize,
    /// Number of mutating proposals in the session.
    pub mutating_proposal_count: usize,
    /// Distinct safety classes carried by the session's proposals.
    pub safety_classes: Vec<QualitySafetyClass>,
    /// Distinct rollback boundaries carried by the session's proposals.
    pub rollback_boundaries: Vec<QualityRollbackBoundaryClass>,
    /// True when any proposal requires preview before apply.
    pub any_preview_first_required: bool,
    /// True when any proposal blocks apply.
    pub any_apply_blocked: bool,
    /// Number of validation refs the session produced.
    pub validation_ref_count: usize,
    /// Number of rollback/revert refs the session produced.
    pub rollback_ref_count: usize,
    /// Whether this projection exposes the session outcome.
    pub exposes_outcome: bool,
    /// Whether this projection exposes the per-proposal safety classes.
    pub exposes_safety_class: bool,
    /// Whether this projection exposes the rollback-boundary note.
    pub exposes_rollback_note: bool,
    /// Whether this projection exposes the validation refs.
    pub exposes_validation_refs: bool,
    /// Whether raw source content is included in this projection.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included in this projection.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl QualitySessionSurfaceProjection {
    /// Whether this projection exposes the proposal/session truth without raw
    /// content and agrees with its source session.
    pub fn is_honest(&self, session: &QualitySession) -> bool {
        self.exposes_outcome
            && self.exposes_safety_class
            && self.exposes_rollback_note
            && self.exposes_validation_refs
            && !self.raw_source_content_included
            && !self.raw_payload_included
            && self.outcome_class == session.outcome_class
            && self.trigger_class == session.trigger_class
            && self.proposal_count == session.proposals.len()
            && self.validation_ref_count == session.validation_refs.len()
            && self.rollback_ref_count == session.rollback_refs.len()
            && self.any_preview_first_required == session.any_preview_first_required
            && self.any_apply_blocked == session.any_apply_blocked
            && self.safety_classes == distinct_safety_classes(session)
            && self.rollback_boundaries == distinct_rollback_boundaries(session)
    }
}

/// One row of a session's preserved proposal trail in a support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualitySessionExportRow {
    /// Session id this row preserves.
    pub session_id: String,
    /// Session trigger.
    pub trigger_class: QualitySessionTriggerClass,
    /// Session outcome.
    pub outcome_class: QualitySessionOutcomeClass,
    /// Proposal ids serialized in the session, in order.
    pub proposal_ids: Vec<String>,
    /// Action classes serialized in the session, in proposal order.
    pub action_classes: Vec<QualityActionClass>,
    /// Validation refs the session produced.
    pub validation_refs: Vec<String>,
    /// Rollback/revert refs the session produced.
    pub rollback_refs: Vec<String>,
}

/// Support export that preserves every session's proposal trail so support and
/// review flows can audit what ran without forking provider-local state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualitySessionLedgerSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub export_id: String,
    /// Workspace id covered by the export.
    pub workspace_id: String,
    /// Session ids cited by the export.
    pub session_refs: Vec<String>,
    /// Per-session preserved proposal trail.
    pub session_trails: Vec<QualitySessionExportRow>,
    /// True when the export preserves each session's proposal trail.
    pub preserves_session_proposals: bool,
    /// Redaction posture for the export.
    pub redaction_class: DiagnosticRedactionClass,
    /// Whether raw source content is included by default.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included by default.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl QualitySessionLedgerSupportExport {
    /// Builds a metadata-only support export from a set of sessions.
    pub fn from_sessions(
        export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        sessions: &[QualitySession],
    ) -> Self {
        let session_refs = sessions
            .iter()
            .map(|session| session.session_id.clone())
            .collect::<Vec<_>>();
        let session_trails = sessions
            .iter()
            .map(|session| QualitySessionExportRow {
                session_id: session.session_id.clone(),
                trigger_class: session.trigger_class,
                outcome_class: session.outcome_class,
                proposal_ids: session
                    .proposals
                    .iter()
                    .map(|proposal| proposal.proposal_id.clone())
                    .collect(),
                action_classes: session
                    .proposals
                    .iter()
                    .map(|proposal| proposal.action_class)
                    .collect(),
                validation_refs: session.validation_refs.clone(),
                rollback_refs: session.rollback_refs.clone(),
            })
            .collect::<Vec<_>>();
        let proposal_total: usize = sessions.iter().map(|session| session.proposals.len()).sum();

        Self {
            record_kind: M5_QUALITY_SESSION_LEDGER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION,
            export_id: export_id.into(),
            workspace_id: workspace_id.into(),
            session_refs,
            session_trails,
            preserves_session_proposals: true,
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary: format!(
                "Support export preserves {} quality sessions and {} typed proposals with raw content omitted by default.",
                sessions.len(),
                proposal_total
            ),
        }
    }

    /// Whether the export preserves every session's id and ordered proposal trail.
    pub fn preserves(&self, sessions: &[QualitySession]) -> bool {
        if !self.preserves_session_proposals {
            return false;
        }
        sessions.iter().all(|session| {
            self.session_refs.contains(&session.session_id)
                && self.session_trails.iter().any(|row| {
                    row.session_id == session.session_id
                        && row.outcome_class == session.outcome_class
                        && row.proposal_ids
                            == session
                                .proposals
                                .iter()
                                .map(|proposal| proposal.proposal_id.clone())
                                .collect::<Vec<_>>()
                })
        })
    }
}

/// Coverage rollup proving the ledger spans the required trigger paths, action
/// classes, safety classes, and outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityActionCoverage {
    /// Distinct trigger paths represented across the sessions.
    pub trigger_paths: Vec<QualitySessionTriggerClass>,
    /// Distinct action classes represented across the proposals.
    pub action_classes: Vec<QualityActionClass>,
    /// Distinct safety classes represented across the proposals.
    pub safety_classes: Vec<QualitySafetyClass>,
    /// Distinct outcomes represented across the sessions.
    pub outcome_classes: Vec<QualitySessionOutcomeClass>,
}

impl QualityActionCoverage {
    /// Builds the coverage rollup from a set of sessions.
    pub fn from_sessions(sessions: &[QualitySession]) -> Self {
        Self {
            trigger_paths: distinct_sorted(sessions.iter().map(|session| session.trigger_class)),
            action_classes: distinct_sorted(
                sessions
                    .iter()
                    .flat_map(|session| session.proposals.iter().map(|p| p.action_class)),
            ),
            safety_classes: distinct_sorted(
                sessions
                    .iter()
                    .flat_map(|session| session.proposals.iter().map(|p| p.safety_class)),
            ),
            outcome_classes: distinct_sorted(sessions.iter().map(|session| session.outcome_class)),
        }
    }

    /// Whether every required trigger path is represented.
    pub fn covers_required_trigger_paths(&self) -> bool {
        REQUIRED_TRIGGER_PATHS
            .iter()
            .all(|required| self.trigger_paths.contains(required))
    }

    /// Whether every required action class is represented.
    pub fn covers_required_action_classes(&self) -> bool {
        REQUIRED_ACTION_CLASSES
            .iter()
            .all(|required| self.action_classes.contains(required))
    }
}

/// Set-level guardrail invariants that must all hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityActionGuardrails {
    /// Every mutating quality action is a typed proposal.
    pub every_mutating_action_is_typed_proposal: bool,
    /// Every proposal is serialized inside a typed session.
    pub every_proposal_serialized_in_session: bool,
    /// On-type, on-save, manual, headless, review-apply, and import-comparison
    /// paths report through one result vocabulary.
    pub one_result_vocabulary_across_paths: bool,
    /// Generated, lockfile/manifest, and protected paths reuse the same
    /// preview/apply/validate/revert lifecycle.
    pub generated_and_protected_reuse_lifecycle: bool,
    /// Rollback notes stay inspectable through every required surface.
    pub rollback_notes_inspectable: bool,
    /// Validation refs stay inspectable through every required surface.
    pub validation_refs_inspectable: bool,
    /// Safety classes stay inspectable through every required surface.
    pub safety_classes_inspectable: bool,
    /// Imported-scan comparison stays read-only and never reads as a local apply.
    pub import_comparison_stays_read_only: bool,
}

impl QualityActionGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_mutating_action_is_typed_proposal
            && self.every_proposal_serialized_in_session
            && self.one_result_vocabulary_across_paths
            && self.generated_and_protected_reuse_lifecycle
            && self.rollback_notes_inspectable
            && self.validation_refs_inspectable
            && self.safety_classes_inspectable
            && self.import_comparison_stays_read_only
    }
}

/// Declares which consumer surfaces expose proposal/session truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityActionConsumerProjection {
    /// Save-participant / editor UI exposes the proposal and session.
    pub ui_shows_proposal_and_session: bool,
    /// Problems exposes the proposal and session.
    pub problems_shows_proposal_and_session: bool,
    /// Review exposes the proposal and session.
    pub review_shows_proposal_and_session: bool,
    /// CLI / headless exposes the proposal and session.
    pub cli_shows_proposal_and_session: bool,
    /// Support export preserves the sessions and proposals.
    pub support_export_preserves_sessions: bool,
}

impl QualityActionConsumerProjection {
    /// Whether every consumer projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.ui_shows_proposal_and_session
            && self.problems_shows_proposal_and_session
            && self.review_shows_proposal_and_session
            && self.cli_shows_proposal_and_session
            && self.support_export_preserves_sessions
    }
}

/// Constructor input for a [`QualitySessionLedgerPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualitySessionLedgerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable ledger label.
    pub ledger_label: String,
    /// Workspace id covered by the ledger.
    pub workspace_id: String,
    /// Quality sessions in the ledger.
    pub sessions: Vec<QualitySession>,
    /// Guardrail invariants block.
    pub guardrails: QualityActionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: QualityActionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 quality-session ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualitySessionLedgerPacket {
    /// Record kind; must equal [`M5_QUALITY_SESSION_LEDGER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable ledger label.
    pub ledger_label: String,
    /// Workspace id covered by the ledger.
    pub workspace_id: String,
    /// Quality sessions in the ledger.
    pub sessions: Vec<QualitySession>,
    /// Cross-surface projections, one per session per exposure surface.
    pub surface_projections: Vec<QualitySessionSurfaceProjection>,
    /// Coverage rollup across trigger paths, action classes, safety, and outcome.
    pub coverage: QualityActionCoverage,
    /// Default support export preserving the session proposal trails.
    pub support_export: QualitySessionLedgerSupportExport,
    /// Guardrail invariants block.
    pub guardrails: QualityActionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: QualityActionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl QualitySessionLedgerPacket {
    /// Builds an M5 quality-session ledger packet, deriving cross-surface
    /// projections, coverage, and the default support export from the sessions.
    pub fn new(input: QualitySessionLedgerPacketInput) -> Self {
        let surface_projections = input
            .sessions
            .iter()
            .flat_map(|session| {
                QUALITY_ACTION_EXPOSURE_SURFACES
                    .into_iter()
                    .map(|surface| session_surface_projection(session, surface))
            })
            .collect::<Vec<_>>();
        let coverage = QualityActionCoverage::from_sessions(&input.sessions);
        let support_export = QualitySessionLedgerSupportExport::from_sessions(
            format!(
                "quality_session_support_export:{}",
                sanitize_id(&input.packet_id)
            ),
            input.workspace_id.clone(),
            &input.sessions,
        );

        Self {
            record_kind: M5_QUALITY_SESSION_LEDGER_RECORD_KIND.to_owned(),
            schema_version: M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            ledger_label: input.ledger_label,
            workspace_id: input.workspace_id,
            sessions: input.sessions,
            surface_projections,
            coverage,
            support_export,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// The projection matching one session and surface, when present.
    pub fn projection_for(
        &self,
        session_id: &str,
        surface_class: QualitySurfaceClass,
    ) -> Option<&QualitySessionSurfaceProjection> {
        self.surface_projections.iter().find(|projection| {
            projection.session_id == session_id && projection.surface_class == surface_class
        })
    }

    /// Distinct outcomes represented across the ledger.
    pub fn represented_outcomes(&self) -> BTreeSet<QualitySessionOutcomeClass> {
        self.sessions
            .iter()
            .map(|session| session.outcome_class)
            .collect()
    }

    /// Validates the M5 quality-session ledger invariants.
    pub fn validate(&self) -> Vec<QualityActionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_QUALITY_SESSION_LEDGER_RECORD_KIND {
            violations.push(QualityActionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION {
            violations.push(QualityActionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.ledger_label.trim().is_empty()
            || self.workspace_id.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(QualityActionViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_sessions(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_support_export(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(QualityActionViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(QualityActionViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("quality-session ledger serializes"),
        ) {
            violations.push(QualityActionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("quality-session ledger serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Quality-Session Ledger\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.ledger_label));
        out.push_str(&format!("- Workspace: `{}`\n", self.workspace_id));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!("- Sessions: {}\n", self.sessions.len()));
        out.push_str(&format!(
            "- Trigger paths covered: {}\n",
            self.coverage.trigger_paths.len()
        ));
        out.push_str(&format!(
            "- Action classes covered: {}\n\n",
            self.coverage.action_classes.len()
        ));

        out.push_str("| Session | Trigger | Outcome | Proposals | Mutating | Preview-first | Apply-blocked |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for session in &self.sessions {
            let mutating = session
                .proposals
                .iter()
                .filter(|proposal| proposal.is_mutating())
                .count();
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | {} | {} |\n",
                session.session_id,
                session.trigger_class.as_str(),
                session.outcome_class.as_str(),
                session.proposals.len(),
                mutating,
                session.any_preview_first_required,
                session.any_apply_blocked,
            ));
        }

        out.push('\n');
        for session in &self.sessions {
            out.push_str(&format!(
                "- `{}` — {} ({})\n",
                session.session_id,
                session.summary,
                session.trigger_class.as_str()
            ));
            for proposal in &session.proposals {
                out.push_str(&format!(
                    "  - {} / {} / {} → preview: {}, rollback: {}\n",
                    proposal.action_class.as_str(),
                    proposal.safety_class.as_str(),
                    proposal.apply_posture_class.as_str(),
                    proposal.preview_requirement_class.as_str(),
                    proposal.rollback_boundary_class.as_str(),
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum QualitySessionLedgerArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<QualityActionViolation>),
}

impl fmt::Display for QualitySessionLedgerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "quality-session ledger support export parse error: {err}"
                )
            }
            Self::Validation(violations) => write!(
                f,
                "quality-session ledger support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for QualitySessionLedgerArtifactError {}

/// Invariant violations reported by [`QualitySessionLedgerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityActionViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required canonical source contracts are missing.
    MissingSourceContracts,
    /// The ledger has no sessions.
    NoSessions,
    /// A session failed its structural completeness invariants.
    SessionStructurallyIncomplete,
    /// A session's proposal refs disagree with its serialized proposals.
    ProposalRefsInconsistent,
    /// A mutating-capable session carries no typed proposal.
    MutatingSessionWithoutProposal,
    /// A serialized result token disagrees with its typed class.
    ResultVocabularyDivergent,
    /// A generated, lockfile/manifest, or protected mutation claimed a weaker
    /// bar instead of the shared preview/apply/validate/revert lifecycle.
    GeneratedOrProtectedWeakerBar,
    /// A mutating proposal dropped its rollback boundary.
    RollbackNoteMissing,
    /// An import-comparison session attempted a local mutation.
    ImportComparisonMutated,
    /// A required exposure-surface projection is missing for a session.
    SurfaceProjectionMissing,
    /// A surface projection drops the outcome, safety class, rollback note, or
    /// validation refs.
    SurfaceProjectionDropsTruth,
    /// The coverage rollup disagrees with the sessions.
    CoverageInconsistent,
    /// A required trigger path is missing.
    RequiredTriggerPathMissing,
    /// A required action class is missing.
    RequiredActionClassMissing,
    /// The support export lost a session's proposal trail.
    SupportExportLossy,
    /// The support export includes raw source or payload content by default.
    SupportExportIncludesRawContent,
    /// Guardrail block is incomplete.
    GuardrailsIncomplete,
    /// Consumer projection block is incomplete.
    ConsumerProjectionIncomplete,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl QualityActionViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoSessions => "no_sessions",
            Self::SessionStructurallyIncomplete => "session_structurally_incomplete",
            Self::ProposalRefsInconsistent => "proposal_refs_inconsistent",
            Self::MutatingSessionWithoutProposal => "mutating_session_without_proposal",
            Self::ResultVocabularyDivergent => "result_vocabulary_divergent",
            Self::GeneratedOrProtectedWeakerBar => "generated_or_protected_weaker_bar",
            Self::RollbackNoteMissing => "rollback_note_missing",
            Self::ImportComparisonMutated => "import_comparison_mutated",
            Self::SurfaceProjectionMissing => "surface_projection_missing",
            Self::SurfaceProjectionDropsTruth => "surface_projection_drops_truth",
            Self::CoverageInconsistent => "coverage_inconsistent",
            Self::RequiredTriggerPathMissing => "required_trigger_path_missing",
            Self::RequiredActionClassMissing => "required_action_class_missing",
            Self::SupportExportLossy => "support_export_lossy",
            Self::SupportExportIncludesRawContent => "support_export_includes_raw_content",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream editor, Problems, review, CLI, and
/// support surfaces use to ingest the quality-session ledger instead of forking
/// per-surface mutation status.
///
/// # Errors
///
/// Returns [`QualitySessionLedgerArtifactError`] when the artifact cannot be
/// parsed or fails validation.
pub fn current_m5_quality_session_ledger_export(
) -> Result<QualitySessionLedgerPacket, QualitySessionLedgerArtifactError> {
    let packet: QualitySessionLedgerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/quality-session-proof/support_export.json"
    )))
    .map_err(QualitySessionLedgerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(QualitySessionLedgerArtifactError::Validation(violations))
    }
}

/// Whether a proposal touches a generated, lockfile/manifest, protected, or
/// policy-scoped path, which must reuse the shared mutation lifecycle.
fn touches_generated_or_protected(proposal: &crate::quality::QualityActionProposal) -> bool {
    proposal.generated_path_count > 0
        || proposal.protected_path_count > 0
        || proposal.safety_class == QualitySafetyClass::GeneratedOrProtected
        || proposal.mutation_scope_class == QualityMutationScopeClass::GeneratedFamily
        || proposal.mutation_scope_class == QualityMutationScopeClass::ProtectedOrPolicyScoped
}

fn distinct_safety_classes(session: &QualitySession) -> Vec<QualitySafetyClass> {
    distinct_sorted(
        session
            .proposals
            .iter()
            .map(|proposal| proposal.safety_class),
    )
}

fn distinct_rollback_boundaries(session: &QualitySession) -> Vec<QualityRollbackBoundaryClass> {
    distinct_sorted(
        session
            .proposals
            .iter()
            .map(|proposal| proposal.rollback_boundary_class),
    )
}

fn session_surface_projection(
    session: &QualitySession,
    surface_class: QualitySurfaceClass,
) -> QualitySessionSurfaceProjection {
    let mutating_proposal_count = session
        .proposals
        .iter()
        .filter(|proposal| proposal.is_mutating())
        .count();
    QualitySessionSurfaceProjection {
        record_kind: M5_QUALITY_SESSION_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION,
        projection_id: format!(
            "quality_session_projection:{}:{}",
            surface_class.as_str(),
            sanitize_id(&session.session_id)
        ),
        session_id: session.session_id.clone(),
        surface_class,
        surface_token: surface_class.as_str().to_owned(),
        trigger_class: session.trigger_class,
        trigger_token: session.trigger_class.as_str().to_owned(),
        outcome_class: session.outcome_class,
        outcome_token: session.outcome_class.as_str().to_owned(),
        proposal_count: session.proposals.len(),
        mutating_proposal_count,
        safety_classes: distinct_safety_classes(session),
        rollback_boundaries: distinct_rollback_boundaries(session),
        any_preview_first_required: session.any_preview_first_required,
        any_apply_blocked: session.any_apply_blocked,
        validation_ref_count: session.validation_refs.len(),
        rollback_ref_count: session.rollback_refs.len(),
        exposes_outcome: true,
        exposes_safety_class: true,
        exposes_rollback_note: true,
        exposes_validation_refs: true,
        raw_source_content_included: false,
        raw_payload_included: false,
        export_safe_summary: format!(
            "{} projection exposes the {} outcome, {} proposals, and rollback note for {} session {}.",
            surface_class.as_str(),
            session.outcome_class.as_str(),
            session.proposals.len(),
            session.trigger_class.as_str(),
            session.session_id
        ),
    }
}

fn validate_source_contracts(
    packet: &QualitySessionLedgerPacket,
    violations: &mut Vec<QualityActionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        QUALITY_ACTION_PROPOSAL_SCHEMA_REF,
        QUALITY_SESSION_SCHEMA_REF,
        M5_QUALITY_SESSION_LEDGER_SCHEMA_REF,
        M5_QUALITY_SESSION_LEDGER_DOC_REF,
        M5_QUALITY_SESSION_LEDGER_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(QualityActionViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_sessions(
    packet: &QualitySessionLedgerPacket,
    violations: &mut Vec<QualityActionViolation>,
) {
    if packet.sessions.is_empty() {
        violations.push(QualityActionViolation::NoSessions);
    }

    for session in &packet.sessions {
        if !session_is_structurally_complete(session) {
            violations.push(QualityActionViolation::SessionStructurallyIncomplete);
        }
        if !session_proposal_refs_consistent(session) {
            violations.push(QualityActionViolation::ProposalRefsInconsistent);
        }
        if session.proposals.is_empty() {
            violations.push(QualityActionViolation::MutatingSessionWithoutProposal);
        }
        if !session_result_vocabulary_consistent(session) {
            violations.push(QualityActionViolation::ResultVocabularyDivergent);
        }

        let is_import_comparison =
            session.trigger_class == QualitySessionTriggerClass::ImportComparison;
        for proposal in &session.proposals {
            if !proposal_result_vocabulary_consistent(proposal) {
                violations.push(QualityActionViolation::ResultVocabularyDivergent);
            }
            if touches_generated_or_protected(proposal)
                && proposal.apply_posture_class == QualityApplyPostureClass::AutoApplyAllowed
            {
                violations.push(QualityActionViolation::GeneratedOrProtectedWeakerBar);
            }
            if proposal.is_mutating()
                && proposal.rollback_boundary_class == QualityRollbackBoundaryClass::NoMutation
            {
                violations.push(QualityActionViolation::RollbackNoteMissing);
            }
            if is_import_comparison && proposal.is_mutating() {
                violations.push(QualityActionViolation::ImportComparisonMutated);
            }
        }

        for surface_class in QUALITY_ACTION_EXPOSURE_SURFACES {
            match packet.projection_for(&session.session_id, surface_class) {
                Some(projection) => {
                    if !projection.is_honest(session) {
                        violations.push(QualityActionViolation::SurfaceProjectionDropsTruth);
                    }
                }
                None => violations.push(QualityActionViolation::SurfaceProjectionMissing),
            }
        }
    }
}

fn validate_coverage(
    packet: &QualitySessionLedgerPacket,
    violations: &mut Vec<QualityActionViolation>,
) {
    let expected = QualityActionCoverage::from_sessions(&packet.sessions);
    if packet.coverage != expected {
        violations.push(QualityActionViolation::CoverageInconsistent);
    }
    if !packet.coverage.covers_required_trigger_paths() {
        violations.push(QualityActionViolation::RequiredTriggerPathMissing);
    }
    if !packet.coverage.covers_required_action_classes() {
        violations.push(QualityActionViolation::RequiredActionClassMissing);
    }
}

fn validate_support_export(
    packet: &QualitySessionLedgerPacket,
    violations: &mut Vec<QualityActionViolation>,
) {
    if packet.support_export.raw_source_content_included
        || packet.support_export.raw_payload_included
    {
        violations.push(QualityActionViolation::SupportExportIncludesRawContent);
    }
    if !packet.support_export.preserves(&packet.sessions) {
        violations.push(QualityActionViolation::SupportExportLossy);
    }
}

fn session_is_structurally_complete(session: &QualitySession) -> bool {
    !session.session_id.trim().is_empty()
        && !session.effective_profile_ref.trim().is_empty()
        && !session.started_at.trim().is_empty()
        && !session.summary.trim().is_empty()
        && session
            .proposals
            .iter()
            .all(proposal_is_structurally_complete)
}

fn proposal_is_structurally_complete(proposal: &crate::quality::QualityActionProposal) -> bool {
    !proposal.proposal_id.trim().is_empty()
        && !proposal.effective_profile_ref.trim().is_empty()
        && !proposal.summary.trim().is_empty()
}

fn session_proposal_refs_consistent(session: &QualitySession) -> bool {
    session.proposal_refs
        == session
            .proposals
            .iter()
            .map(|proposal| proposal.proposal_id.clone())
            .collect::<Vec<_>>()
}

fn session_result_vocabulary_consistent(session: &QualitySession) -> bool {
    session.trigger_token == session.trigger_class.as_str()
        && session.target_scope_token == session.target_scope_class.as_str()
        && session.outcome_token == session.outcome_class.as_str()
}

fn proposal_result_vocabulary_consistent(proposal: &crate::quality::QualityActionProposal) -> bool {
    proposal.action_token == proposal.action_class.as_str()
        && proposal.target_scope_token == proposal.target_scope_class.as_str()
        && proposal.mutation_scope_token == proposal.mutation_scope_class.as_str()
        && proposal.safety_token == proposal.safety_class.as_str()
        && proposal.disclosure_token == proposal.disclosure_class.as_str()
        && proposal.preview_requirement_token == proposal.preview_requirement_class.as_str()
        && proposal.apply_posture_token == proposal.apply_posture_class.as_str()
        && proposal.rollback_boundary_token == proposal.rollback_boundary_class.as_str()
}

fn distinct_sorted<T>(values: impl Iterator<Item = T>) -> Vec<T>
where
    T: Ord,
{
    values.collect::<BTreeSet<_>>().into_iter().collect()
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
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
