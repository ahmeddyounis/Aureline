//! M05-883 surface certification over the frozen M5 AI-execution/replay component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! defines the eight reusable AI-action-state-banner, connector-detail-row,
//! local-model-pack-card, high-friction approval-sheet, tool-call-timeline-row,
//! run-history-row, replay/rerun-review, and agent-status components, the M05-877..881
//! primitive lanes narrow each one, and the M05-882 consumer lane
//! ([`crate::add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers`])
//! proves they are reusable across the claimed AI consumers, this closing capstone
//! *certifies* that the shared AI-execution/replay component truth holds on every
//! claimed M5 AI surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user reviews, reruns, pauses, resumes,
//! exports, or hands off AI work through (inline assistant, assistant panel,
//! patch-review, test-generation, branch/worktree queue, help console, CLI/headless,
//! and support/export), not on component family or primitive lane. Each
//! [`AiSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, CLI/export, degraded-state, and execution-boundary /
//! replay provenance — and either passes (green), auto-narrows its AI-execution
//! support claim to the weakest supported ceiling (yellow), or is blocked (red) when a
//! degraded axis is hidden behind a full-truth claim inherited from a healthier AI
//! surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `LiveGovernedExecution` / `CompleteReplay` claim while one of
//! its truth axes is not current — the provider/model route drifted, replay is
//! incomplete, an approval is stale, a connector is policy-blocked, or a background
//! agent was interrupted — is over-claiming and blocks; a surface that discloses the
//! reduction by narrowing its support claim (with a bound reason and a frozen downgrade
//! trigger) is honestly yellow. The always-on CLI/export axis must always stay
//! certified, so support and automation can reconstruct the same mode / route / tool
//! boundary / approval / checkpoint / replay / takeover truth from the same run
//! identity the user saw.
//!
//! Every row cites exactly one canonical AI-execution/replay proof bundle
//! ([`AI_CERT_CANONICAL_BUNDLE_REF`]) — the frozen AI-execution component matrix
//! release proof — rather than cloning per-surface evidence. The packet is
//! metadata-only: raw prompts, provider tokens, connector credentials, and replay
//! bodies never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-execution-replay-component-certification.schema.json`](../../../../schemas/ai/m5-ai-execution-replay-component-certification.schema.json).
//! The contract doc is
//! [`docs/ai/m5/m5_ai_execution_replay_component_certification_contract.md`](../../../../docs/ai/m5/m5_ai_execution_replay_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers as consumers;
use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix as matrix;
use matrix::{M5AiExecutionComponentFamily, M5AiExecutionDowngradeTrigger};

/// Schema version stamped on the M05-883 certification packet.
pub const AI_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`AiSurfaceCertificationPacket`].
pub const AI_CERT_RECORD_KIND: &str = "m5_ai_execution_replay_component_certification_packet";

/// Stable record-kind tag carried by each [`AiSurfaceCertificationRow`].
pub const AI_CERT_ROW_RECORD_KIND: &str = "m5_ai_execution_replay_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const AI_CERT_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-execution-replay-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const AI_CERT_DOC_REF: &str =
    "docs/ai/m5/m5_ai_execution_replay_component_certification_contract.md";

/// Repo-relative path of the frozen AI-execution component matrix schema the certified
/// surfaces render.
pub const AI_CERT_MATRIX_REF: &str = matrix::M5_AI_EXECUTION_COMPONENT_SCHEMA_REF;

/// The one canonical AI-execution/replay proof bundle every certified surface cites as
/// its first-resolved component truth. All eight surfaces point back to it rather than
/// cloning per-surface evidence.
pub const AI_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_AI_EXECUTION_COMPONENT_ARTIFACT_REF;

/// The M05-882 consumer-adoption support export the certification builds on. Recorded
/// as a supporting evidence ref on every row.
pub const AI_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_AI_EXECUTION_REPLAY_CONSUMER_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const AI_CERT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/m5-ai-execution-replay-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const AI_CERT_CSV_REF: &str =
    "artifacts/ai/m5/m5-ai-execution-replay-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const AI_CERT_REPORT_REF: &str =
    "artifacts/ai/m5/m5-ai-execution-replay-component-certification/report.md";

/// The controlled AI-execution support claim a certified surface asserts and is
/// certified down to. A six-tier ladder from live governed execution down to a
/// policy-blocked reference; a surface may only ever be certified *no stronger* than it
/// claims, and narrows down this ladder when a truth axis is not current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionSupportClaim {
    /// Live, fully governed execution with complete replay: mode, route/provider/model,
    /// tool boundary, auth posture, approval gate, checkpoint lineage, replay
    /// completeness, and manual-takeover path are all current and explicit.
    LiveGovernedExecution,
    /// The run is reconstructable end-to-end from a complete replay, even if it is no
    /// longer live.
    CompleteReplay,
    /// Replay is reconstructable but the provider / model route drifted from the
    /// original run; the adjacency is disclosed rather than presented as identical.
    RouteAdjacentReplay,
    /// Evidence is served from a cache / buffer while the provider or connector is
    /// unreachable; last-known state, not a live read.
    CachedEvidence,
    /// The agent / replay state is not confirmed — a background agent was interrupted,
    /// a checkpoint lineage is incomplete, or an approval is stale — so the state is
    /// unverified pending manual takeover or re-review.
    UnverifiedAgentState,
    /// A connector, tool, or model pack in this surface is blocked by policy, so the
    /// surface cannot claim governed execution at all.
    PolicyBlockedExecution,
}

impl M5AiExecutionSupportClaim {
    /// Every support claim, strongest first.
    pub const ALL: [M5AiExecutionSupportClaim; 6] = [
        M5AiExecutionSupportClaim::LiveGovernedExecution,
        M5AiExecutionSupportClaim::CompleteReplay,
        M5AiExecutionSupportClaim::RouteAdjacentReplay,
        M5AiExecutionSupportClaim::CachedEvidence,
        M5AiExecutionSupportClaim::UnverifiedAgentState,
        M5AiExecutionSupportClaim::PolicyBlockedExecution,
    ];

    /// Capability rank, strongest = 5 down to weakest = 0. Certification may only ever
    /// narrow the claim (lower this rank), never raise it.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::LiveGovernedExecution => 5,
            Self::CompleteReplay => 4,
            Self::RouteAdjacentReplay => 3,
            Self::CachedEvidence => 2,
            Self::UnverifiedAgentState => 1,
            Self::PolicyBlockedExecution => 0,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveGovernedExecution => "live_governed_execution",
            Self::CompleteReplay => "complete_replay",
            Self::RouteAdjacentReplay => "route_adjacent_replay",
            Self::CachedEvidence => "cached_evidence",
            Self::UnverifiedAgentState => "unverified_agent_state",
            Self::PolicyBlockedExecution => "policy_blocked_execution",
        }
    }
}

/// The eight claimed M5 AI surfaces this capstone certifies. Keyed on the surface a
/// user actually reviews, reruns, pauses, resumes, exports, or hands off AI work
/// through, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionCertifiedSurface {
    /// The inline AI assistant embedded in the editor / terminal.
    InlineAssistant,
    /// The dedicated assistant panel where runs and history are reviewed.
    AssistantPanel,
    /// The guided patch-review surface where AI edits are approved.
    PatchReview,
    /// The AI test-generation and review surface.
    TestGeneration,
    /// The branch / worktree background-agent queue.
    BranchWorktreeQueue,
    /// The help / support console that references AI runs and connectors.
    HelpConsole,
    /// The CLI / headless AI surface.
    CliHeadless,
    /// The support / export bundle surface.
    SupportExport,
}

impl M5AiExecutionCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5AiExecutionCertifiedSurface; 8] = [
        M5AiExecutionCertifiedSurface::InlineAssistant,
        M5AiExecutionCertifiedSurface::AssistantPanel,
        M5AiExecutionCertifiedSurface::PatchReview,
        M5AiExecutionCertifiedSurface::TestGeneration,
        M5AiExecutionCertifiedSurface::BranchWorktreeQueue,
        M5AiExecutionCertifiedSurface::HelpConsole,
        M5AiExecutionCertifiedSurface::CliHeadless,
        M5AiExecutionCertifiedSurface::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineAssistant => "inline_assistant",
            Self::AssistantPanel => "assistant_panel",
            Self::PatchReview => "patch_review",
            Self::TestGeneration => "test_generation",
            Self::BranchWorktreeQueue => "branch_worktree_queue",
            Self::HelpConsole => "help_console",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader,
/// CLI/export, degraded-state, and execution-boundary / replay provenance. The
/// CLI/export axis is always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiCertificationAxis {
    /// Visual parity: active execution mode, action state, route/provider/model, tool
    /// boundary, auth posture, approval gate, and replay completeness are shown on the
    /// primary surface.
    Visual,
    /// Keyboard-reach parity: the same mode / route / approval / replay / takeover truth
    /// and its controls are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable
    /// as text / JSON / Markdown for support and automation, from the same run identity.
    CliExport,
    /// Degraded-state parity: a cached / buffered read, an unreachable provider, or a
    /// stale proof honestly downgrades a `LiveGovernedExecution` / `CompleteReplay`
    /// claim to a weaker support tier.
    DegradedState,
    /// Execution-boundary / replay provenance parity: route/provider/model, tool
    /// boundary, auth posture, approval gate, checkpoint lineage, replay completeness,
    /// drift reason, and manual-takeover path stay explicit, never inheriting a
    /// healthier surface's provenance or masking an interrupted agent as live.
    ExecutionBoundaryAndReplayProvenance,
}

impl AiCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [AiCertificationAxis; 6] = [
        AiCertificationAxis::Visual,
        AiCertificationAxis::Keyboard,
        AiCertificationAxis::ScreenReader,
        AiCertificationAxis::CliExport,
        AiCertificationAxis::DegradedState,
        AiCertificationAxis::ExecutionBoundaryAndReplayProvenance,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::ExecutionBoundaryAndReplayProvenance => {
                "execution_boundary_and_replay_provenance"
            }
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim
    /// inherited from a healthier surface.
    UndisclosedDrift,
}

impl AiAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the
/// author — always recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, or
    /// the narrowing is inconsistent.
    Red,
}

impl AiSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow);
    /// red surfaces block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis
/// certifies only when this offers text / JSON / Markdown reconstruction and prohibits
/// a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The run / route / approval / replay / takeover fields the surface preserves in
    /// export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl AiCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: AiCertificationAxis,
    /// The certification state of the axis.
    pub state: AiAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5AiExecutionDowngradeTrigger>,
}

impl AiAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible
    ///   trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            AiAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            AiAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            AiAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current.
/// Present iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: AiCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5AiExecutionSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5AiExecutionSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 AI surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiSurfaceCertificationRow {
    /// Record kind; must equal [`AI_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`AI_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5AiExecutionCertifiedSurface,
    /// The AI-execution support-claim ceiling the surface asserts.
    pub claimed_claim: M5AiExecutionSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no
    /// stronger than `claimed_claim`.
    pub certified_claim: M5AiExecutionSupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5AiExecutionComponentFamily>,
    /// One outcome per [`AiCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<AiAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<AiClaimAutoNarrow>,
    /// The one canonical AI-execution proof bundle this surface cites. Must equal
    /// [`AI_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: AiSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: AiCertExportParity,
    /// The compatibility notes captured for this surface.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl AiSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: AiCertificationAxis) -> Option<&AiAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<AiCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && AiCertificationAxis::ALL.iter().all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes.iter().all(AiAxisOutcome::well_formed)
    }

    /// True when the surface narrows its support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<AiCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == AiAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> AiSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != AI_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return AiSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return AiSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(AiCertificationAxis::CliExport) {
            Some(o) if o.state == AiAxisCertificationState::Certified => {}
            _ => return AiSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == AiAxisCertificationState::UndisclosedDrift)
        {
            return AiSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return AiSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return AiSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return AiSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return AiSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return AiSurfaceClaimStatus::Red;
        }

        AiSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == AI_CERT_ROW_RECORD_KIND
            && self.schema_version == AI_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-883 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiSurfaceCertificationSummary {
    pub row_count: usize,
    pub surface_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_surfaces_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`AiSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<AiSurfaceCertificationRow>,
}

/// Checked-in M05-883 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<AiSurfaceCertificationRow>,
    pub summary: AiSurfaceCertificationSummary,
}

impl AiSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: AiSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: AI_CERT_SCHEMA_VERSION,
            record_kind: AI_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: AiSurfaceCertificationSummary {
                row_count: 0,
                surface_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_surfaces_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                every_axis_covered_on_every_row: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5AiExecutionCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5AiExecutionComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5AiExecutionCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface —
    /// proof the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5AiExecutionComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(AiCertificationAxis::CliExport)
                .is_some_and(|o| o.state == AiAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> AiSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == AiSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == AiSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == AiSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(AiSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        AiSurfaceCertificationSummary {
            row_count: self.rows.len(),
            surface_count: surfaces.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_surfaces_present: all_surfaces,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == AI_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(AiSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<AiCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != AI_CERT_SCHEMA_VERSION {
            violations.push(AiCertificationViolation::SchemaVersion {
                expected: AI_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != AI_CERT_RECORD_KIND {
            violations.push(AiCertificationViolation::RecordKind {
                expected: AI_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(AiCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != AI_CERT_CANONICAL_BUNDLE_REF {
            violations.push(AiCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(AiCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(AiCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(AiCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(AiCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != AI_CERT_CANONICAL_BUNDLE_REF {
                violations.push(AiCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(AiCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(AiCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(AiCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(AiCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == AiSurfaceClaimStatus::Red {
                violations.push(AiCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(AiCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(AiCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(AiCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(AiCertificationViolation::RawAiMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 AI-Execution/Replay Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5AiExecutionCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Auto-narrowed surfaces: {}\n",
            self.summary.narrowed_surface_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_ai_execution_replay_component_certification_export(
) -> Result<AiSurfaceCertificationPacket, AiCertificationArtifactError> {
    let packet: AiSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-ai-execution-replay-component-certification/support_export.json"
    )))
    .map_err(AiCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum AiCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiCertificationViolation>),
}

impl fmt::Display for AiCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for AiCertificationArtifactError {}

/// Validation failure for M05-883 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawAiMaterialInExport,
}

impl fmt::Display for AiCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical AI-execution proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical AI-execution proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::SurfaceBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a full claim, \
CLI/export parity dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 AI surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen AI-execution component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawAiMaterialInExport => {
                write!(f, "export contains raw AI material")
            }
        }
    }
}

impl Error for AiCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&AiAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != AiAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure.
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
            | "stale"
            | "cached"
            | "unverified"
            | "offline"
            | "blocked"
            | "paused"
            | "interrupted"
            | "incomplete"
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

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-883 certification packet. Certifies all eight
/// claimed M5 AI surfaces: four deliver their claim (green) and four auto-narrow a
/// not-current truth axis to a weaker support ceiling (yellow). No surface hides drift
/// (red).
pub fn seeded_m5_ai_execution_replay_component_certification_packet() -> AiSurfaceCertificationPacket
{
    AiSurfaceCertificationPacket::new(AiSurfaceCertificationPacketInput {
        packet_id: "m5-ai-execution-replay-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: AI_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: AI_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:ai-execution-certification:{id}"),
        AI_CERT_CONSUMER_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> AiCertExportParity {
    AiCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: AiCertificationAxis) -> &'static str {
    match axis {
        AiCertificationAxis::Visual => {
            "execution mode, action state, route/provider/model, tool boundary, auth posture, approval gate, and replay completeness shown on-surface"
        }
        AiCertificationAxis::Keyboard => {
            "the same mode/route/approval/replay/takeover truth and its controls are keyboard-reachable"
        }
        AiCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        AiCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support replay from the same run identity"
        }
        AiCertificationAxis::DegradedState => {
            "a cached/buffered read, an unreachable provider, or a stale proof honestly downgrades the LiveGovernedExecution/CompleteReplay claim"
        }
        AiCertificationAxis::ExecutionBoundaryAndReplayProvenance => {
            "route/provider/model, tool boundary, auth posture, approval gate, checkpoint lineage, replay completeness, drift reason, and manual-takeover path stay explicit"
        }
    }
}

fn seed_certified(axis: AiCertificationAxis) -> AiAxisOutcome {
    AiAxisOutcome {
        axis,
        state: AiAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: AiCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5AiExecutionDowngradeTrigger,
) -> AiAxisOutcome {
    AiAxisOutcome {
        axis,
        state: AiAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<AiAxisOutcome> {
    AiCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(axis: AiCertificationAxis, outcome: AiAxisOutcome) -> Vec<AiAxisOutcome> {
    AiCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    surface: M5AiExecutionCertifiedSurface,
    claimed_claim: M5AiExecutionSupportClaim,
    certified_claim: M5AiExecutionSupportClaim,
    consumed_families: &[M5AiExecutionComponentFamily],
    axis_outcomes: Vec<AiAxisOutcome>,
    claim_auto_narrow: Option<AiClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> AiSurfaceCertificationRow {
    let mut row = AiSurfaceCertificationRow {
        record_kind: AI_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: AI_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: AI_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: AiSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![AI_CERT_MATRIX_REF.to_owned(), AI_CERT_SCHEMA_REF.to_owned()],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: AiCertificationAxis,
    from_claim: M5AiExecutionSupportClaim,
    to_claim: M5AiExecutionSupportClaim,
    label: &str,
) -> AiClaimAutoNarrow {
    AiClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<AiSurfaceCertificationRow> {
    use AiCertificationAxis as Ax;
    use M5AiExecutionCertifiedSurface as S;
    use M5AiExecutionComponentFamily::*;
    use M5AiExecutionDowngradeTrigger as Trig;
    use M5AiExecutionSupportClaim::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:inline-assistant",
            S::InlineAssistant,
            LiveGovernedExecution,
            LiveGovernedExecution,
            &[AiActionStateBanner, ToolCallTimelineRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "execution_mode"],
            &[
                "the action-state banner names the active execution mode and live action state",
                "the tool-call timeline row names where each tool ran and its side-effect class",
                "keyboard/screen-reader reach preserved for the banner and timeline controls",
                "boundary/replay: an inline run never masks its route/provider or leaves an approval gate implicit",
            ],
        ),
        seed_row(
            "cert:assistant-panel",
            S::AssistantPanel,
            CompleteReplay,
            CompleteReplay,
            &[RunHistoryRow, ApprovalSheet],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "run_identity"],
            &[
                "the run-history row keeps its canonical run identity, route/provider/model, and outcome",
                "the high-friction approval sheet keeps its effective approval gate and friction reason visible",
                "export reconstructs the run identity, route, and approval truth from the same object",
                "boundary/replay: a stale approval is never presented as an in-force gate on the panel",
            ],
        ),
        seed_row(
            "cert:patch-review",
            S::PatchReview,
            LiveGovernedExecution,
            LiveGovernedExecution,
            &[ApprovalSheet, ToolCallTimelineRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "approval_gate"],
            &[
                "the approval sheet keeps the requested-action scope, side effect, and rollback/checkpoint explicit",
                "the tool-call timeline row keeps its tool boundary and always-visible provenance follow-ups",
                "keyboard/screen-reader reach preserved for approve-once / deny / open-plan controls",
                "boundary/replay: a mutating patch action is never masked as a read-only status row",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            LiveGovernedExecution,
            LiveGovernedExecution,
            &[RunHistoryRow, ReplayReview],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "replay_completeness"],
            &[
                "support export reconstructs mode/route/tool-boundary/approval/checkpoint/replay/takeover truth from the same run identity",
                "the replay-review sheet names replay completeness and any rerun-review reason",
                "text / JSON / Markdown reconstruction certified for support replay",
                "boundary/replay: a support packet never exports raw prompts, tokens, or connector credentials",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:test-generation",
            S::TestGeneration,
            LiveGovernedExecution,
            UnverifiedAgentState,
            &[ReplayReview, RunHistoryRow],
            seed_certified_except(
                Ax::ExecutionBoundaryAndReplayProvenance,
                seed_narrowed(
                    Ax::ExecutionBoundaryAndReplayProvenance,
                    "generated-test rerun replay is incomplete",
                    "The generated-test rerun could not reconstruct a complete replay — a checkpoint's evidence is missing — so the LiveGovernedExecution claim narrows to unverified instead of presenting an incomplete replay as fully reconstructable",
                    Trig::ReplayCompletenessOverstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ExecutionBoundaryAndReplayProvenance,
                LiveGovernedExecution,
                UnverifiedAgentState,
                "Unverified replay: the generated-test rerun is missing a checkpoint's evidence, so its replay is shown as incomplete rather than complete",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the replay-review sheet keeps the incomplete-replay banner and the missing checkpoint explicit",
                "the run-history row keeps its route/provider/model and outcome visible without asserting a complete replay",
                "boundary/replay: LiveGovernedExecution narrows to unverified (auto-narrowed)",
                "known compatibility note: replay incompleteness — an incomplete rerun never reads as fully reconstructable",
            ],
        ),
        seed_row(
            "cert:branch-worktree-queue",
            S::BranchWorktreeQueue,
            CompleteReplay,
            UnverifiedAgentState,
            &[AgentStatus, ReplayReview],
            seed_certified_except(
                Ax::ExecutionBoundaryAndReplayProvenance,
                seed_narrowed(
                    Ax::ExecutionBoundaryAndReplayProvenance,
                    "background agent interrupted; checkpoint lineage incomplete",
                    "The background branch agent was interrupted and its checkpoint lineage is incomplete, so the CompleteReplay claim narrows to unverified and the agent is shown as needing safe manual takeover rather than appearing alive or reusable",
                    Trig::CheckpointLineageBroken,
                ),
            ),
            Some(seed_narrow(
                Ax::ExecutionBoundaryAndReplayProvenance,
                CompleteReplay,
                UnverifiedAgentState,
                "Unverified agent: the branch agent was interrupted with an incomplete checkpoint lineage; the card offers safe manual takeover instead of implying the run is live",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the agent-status card keeps the interrupted lifecycle state and the manual-takeover path explicit",
                "the replay-review sheet keeps the checkpoint lineage and drift reason visible",
                "boundary/replay: CompleteReplay narrows to unverified (auto-narrowed)",
                "known compatibility note: manual-takeover path — an interrupted agent never appears alive or reusable by implication",
            ],
        ),
        seed_row(
            "cert:help-console",
            S::HelpConsole,
            CompleteReplay,
            PolicyBlockedExecution,
            &[ConnectorDetailRow, LocalModelPackCard],
            seed_certified_except(
                Ax::ExecutionBoundaryAndReplayProvenance,
                seed_narrowed(
                    Ax::ExecutionBoundaryAndReplayProvenance,
                    "a referenced connector is blocked by policy",
                    "A connector referenced from the help console is blocked by policy, so the CompleteReplay claim narrows to policy-blocked instead of presenting a blocked connector as available governed execution",
                    Trig::AuthPostureMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::ExecutionBoundaryAndReplayProvenance,
                CompleteReplay,
                PolicyBlockedExecution,
                "Policy-blocked connector: a referenced tool-server is blocked by policy; the help console shows the auth posture and block reason rather than an available connector",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the connector detail row keeps its execution locus, auth posture, and policy-blocked readiness distinct",
                "the local model pack card keeps its pack readiness and provenance visible",
                "boundary/replay: CompleteReplay narrows to policy-blocked (auto-narrowed)",
                "known compatibility note: connector policy block — a policy-blocked connector never reads as available",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            LiveGovernedExecution,
            CachedEvidence,
            &[ConnectorDetailRow, LocalModelPackCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "provider unreachable; headless output replays cached evidence",
                    "The provider/model is unreachable from the headless run, so the CLI output replays cached evidence rather than a live read, and the LiveGovernedExecution claim narrows to cached instead of presenting last-known state as current",
                    Trig::RouteOrProviderMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                LiveGovernedExecution,
                CachedEvidence,
                "Cached evidence: the provider is unreachable, so headless output is a last-known cached read; the connector row shows the provider as unavailable rather than masking the route",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the connector detail row keeps its unreachable readiness and route/provider explicit in the structured output",
                "the local model pack card keeps its offline locality and provenance explicit",
                "degraded-state: LiveGovernedExecution narrows to cached (auto-narrowed)",
                "known compatibility note: provider/model unavailability — cached headless output never reads as a live provider read",
            ],
        ),
    ]
}
