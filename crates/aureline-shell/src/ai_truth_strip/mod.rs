//! Bounded AI evidence-packet seed and route/spend truth strip for the
//! launch AI wedge.
//!
//! ## What the wedge owns
//!
//! Every AI interaction the launch wedge surfaces must answer the same
//! three questions before the user trusts the output: *which run was
//! this, what route / provider / spend posture was it admitted under,
//! and what is in the context that backed it?* The shell's
//! [`crate::ai_context_inspector`] already projects the composer draft
//! into a per-axis inspector card. This wedge extends that truth with
//! one inspectable [`AiEvidencePacketSeedRecord`] plus a typed
//! [`RouteSpendTruthStripRow`] list so the chrome can render a visible
//! provider / route / path / spend strip alongside the inspector — even
//! before M1 ships any model dispatch.
//!
//! ## Why a packet plus a strip, not just an inline chip
//!
//! Forking "what the AI knows" between the inspector (mentions /
//! attachments / commands), the strip (provider / route / spend), and a
//! future export packet would let one surface promote a tainted run to a
//! "completed" state while another lags. This module mints one record
//! the chrome quotes verbatim: the [`AiTruthStripSnapshot`] carries the
//! packet, the typed strip rows, and a typed
//! [`AiTruthStripInvariantViolation`] vocabulary the chrome surfaces
//! before letting the wedge render.
//!
//! ## Bounded scope (deliberately)
//!
//! - The wedge never dispatches a model and never mints a billable spend
//!   amount. The default route placeholder pins
//!   `provider_class = disabled_no_provider_in_m1_seed` /
//!   `route_path_class = denied_by_policy_in_m1_seed` /
//!   `dispatch_target_class = disabled_no_dispatch_in_m1_seed` so the
//!   strip is honest about what the seed actually does.
//! - Raw URLs, raw provider payloads, raw cost amounts in any currency,
//!   raw token counts, and raw credential bodies never appear on the
//!   packet or the strip. The export-safe surface
//!   [`AiTruthStripSnapshot::export_safe_run_metadata`] carries only the
//!   typed tokens.
//! - The wedge does not duplicate the upstream
//!   [`aureline_ai`] composer vocabulary; every mention / attachment /
//!   slash-command / route-placeholder token is read from the upstream
//!   draft. Forking would defeat the M01-116 truth-source guarantee.

use serde::{Deserialize, Serialize};

use aureline_ai::{
    BlockReason, ComposerDraft, ComposerDraftState, DispatchTargetClass, MentionResolutionState,
    RoutePlaceholder, SlashCommandResolutionState, TrustPosture,
};

/// Stable record-kind tag carried in serialized
/// [`AiEvidencePacketSeedRecord`] payloads.
pub const AI_EVIDENCE_SEED_RECORD_KIND: &str = "ai_evidence_seed_record";

/// Stable record-kind tag carried in serialized
/// [`AiTruthStripSnapshot`] payloads.
pub const AI_TRUTH_STRIP_SNAPSHOT_RECORD_KIND: &str = "ai_truth_strip_snapshot_record";

/// Schema version for the seed-record and snapshot payload shapes.
pub const AI_EVIDENCE_SEED_SCHEMA_VERSION: u32 = 1;

/// Frozen prototype-label vocabulary the chrome quotes verbatim on every
/// packet and strip readout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype: AI evidence packet and route/spend truth
    /// strip on one launch wedge.
    M1PrototypeEvidenceAndSpendSeed,
}

impl PrototypeLabel {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeEvidenceAndSpendSeed => "m1_prototype_evidence_and_spend_seed",
        }
    }

    /// Human-readable chip label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeEvidenceAndSpendSeed => {
                "M1 prototype — AI evidence packet and route/spend truth strip, no model dispatch"
            }
        }
    }
}

/// Run-state vocabulary the seed surfaces on the strip and on the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunStateClass {
    /// The live-wedge default: the M1 seed never dispatches a model.
    DispatchDisabledInM1Seed,
    /// At least one block reason other than the always-on
    /// policy-blocked-route marker prevents the draft from advancing.
    BlockedPendingResolution,
    /// Fixture-only: a preview run was minted but no bytes left the device.
    PreviewPreDispatchMocked,
    /// Fixture-only: a mocked completed run for replay.
    PostRunCompletedMocked,
    /// Fixture-only: a mocked failed run for replay.
    PostRunFailedMocked,
}

impl AiRunStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DispatchDisabledInM1Seed => "dispatch_disabled_in_m1_seed",
            Self::BlockedPendingResolution => "blocked_pending_resolution",
            Self::PreviewPreDispatchMocked => "preview_pre_dispatch_mocked",
            Self::PostRunCompletedMocked => "post_run_completed_mocked",
            Self::PostRunFailedMocked => "post_run_failed_mocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DispatchDisabledInM1Seed => "Dispatch disabled in M1 seed",
            Self::BlockedPendingResolution => "Blocked — pending resolution",
            Self::PreviewPreDispatchMocked => "Mocked preview (fixtures only)",
            Self::PostRunCompletedMocked => "Mocked completed run (fixtures only)",
            Self::PostRunFailedMocked => "Mocked failed run (fixtures only)",
        }
    }
}

/// Coarse local-vs-remote vocabulary the strip renders so the user can
/// see at a glance whether the wedge would run on the device, against a
/// cached store, or off-device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalOrRemotePathClass {
    /// Live default: nothing leaves the device because the seed does not
    /// dispatch.
    LocalNoDispatch,
    /// Reserved for future cache-only flows; rendered honestly when used.
    LocalOnlyCached,
    /// Fixture-only: the strip pretends the run targets a remote provider
    /// to prove the chrome surfaces the new path posture verbatim.
    RemoteMockedForFixtures,
    /// The packet was minted under a disabled route; no path applies.
    DisabledNoPath,
}

impl LocalOrRemotePathClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalNoDispatch => "local_no_dispatch",
            Self::LocalOnlyCached => "local_only_cached",
            Self::RemoteMockedForFixtures => "remote_mocked_for_fixtures",
            Self::DisabledNoPath => "disabled_no_path",
        }
    }

    /// Human-readable label rendered on the strip row.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalNoDispatch => "Local — no model dispatch in the M1 seed",
            Self::LocalOnlyCached => "Local-only cached path",
            Self::RemoteMockedForFixtures => "Remote (mocked for fixtures)",
            Self::DisabledNoPath => "Disabled — no route admitted",
        }
    }
}

/// Coarse spend-posture vocabulary the strip renders. Raw cost amounts
/// in any currency, raw token counts, and raw provider unit prices never
/// appear on the seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpendPostureClass {
    /// Live default: the seed never mints a billable spend amount.
    NoSpendInM1Seed,
    /// Fixture-only mocked posture used by the failure drill.
    MockedSpendForFixtures,
    /// Reserved for a future bundled-cost provider; rendered honestly
    /// when used.
    BundledNoIncrementalCost,
    /// Reserved for a future estimated-only spend band.
    EstimatedUnverifiedBand,
}

impl SpendPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSpendInM1Seed => "no_spend_in_m1_seed",
            Self::MockedSpendForFixtures => "mocked_spend_for_fixtures",
            Self::BundledNoIncrementalCost => "bundled_no_incremental_cost",
            Self::EstimatedUnverifiedBand => "estimated_unverified_band",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoSpendInM1Seed => "No spend — seed does not dispatch",
            Self::MockedSpendForFixtures => "Mocked spend (fixtures only)",
            Self::BundledNoIncrementalCost => "Bundled — no incremental cost",
            Self::EstimatedUnverifiedBand => "Estimated-unverified band",
        }
    }
}

/// Frozen claim-limit vocabulary. The snapshot MUST render this exact list
/// in canonical order; reordering or dropping a row surfaces the typed
/// `ClaimLimitsMissingOrOutOfOrder` invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiTruthStripClaimLimit {
    /// Bounded prototype: one wedge on one launch lane.
    SingleBoundedWedgeOnly,
    /// The seed never dispatches a model.
    NoLiveModelDispatch,
    /// The seed does not track billing, quotas, or chargeback.
    NoBillingOrQuotaTracking,
    /// Raw URLs, raw provider payloads, raw cost amounts, raw token counts,
    /// and raw credential bodies never cross the boundary.
    NoRawSecretsOrProviderUrls,
}

impl AiTruthStripClaimLimit {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => "single_bounded_wedge_only",
            Self::NoLiveModelDispatch => "no_live_model_dispatch",
            Self::NoBillingOrQuotaTracking => "no_billing_or_quota_tracking",
            Self::NoRawSecretsOrProviderUrls => "no_raw_secrets_or_provider_urls",
        }
    }

    /// Canonical claim-limit list. The chrome MUST quote the list in this
    /// exact order.
    pub fn canonical_list() -> Vec<Self> {
        vec![
            Self::SingleBoundedWedgeOnly,
            Self::NoLiveModelDispatch,
            Self::NoBillingOrQuotaTracking,
            Self::NoRawSecretsOrProviderUrls,
        ]
    }
}

/// Frozen invariant-violation vocabulary. A buggy caller cannot mint a
/// clean packet while contradicting the seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiTruthStripInvariantViolation {
    MissingPrototypeLabel,
    ClaimLimitsMissingOrOutOfOrder,
    RouteAndPathClassDisagree,
    SpendPostureContradictsRoute,
    DispatchDisabledButCompletedOutcomeClaimed,
    DraftBlockedButPacketClaimsReady,
    EvidencePacketIdMissing,
    ExactBuildIdentityRefMissing,
}

impl AiTruthStripInvariantViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "missing_prototype_label",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::RouteAndPathClassDisagree => "route_and_path_class_disagree",
            Self::SpendPostureContradictsRoute => "spend_posture_contradicts_route",
            Self::DispatchDisabledButCompletedOutcomeClaimed => {
                "dispatch_disabled_but_completed_outcome_claimed"
            }
            Self::DraftBlockedButPacketClaimsReady => "draft_blocked_but_packet_claims_ready",
            Self::EvidencePacketIdMissing => "evidence_packet_id_missing",
            Self::ExactBuildIdentityRefMissing => "exact_build_identity_ref_missing",
        }
    }
}

/// Caller-supplied route / path / spend posture. The wedge consumes one
/// posture per packet so a fixture can override the M1 defaults to
/// exercise the failure drill (e.g. a mocked alternative route + spend
/// class) without forking the live seed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRouteSpendPosture {
    pub local_or_remote_path_class: LocalOrRemotePathClass,
    pub spend_posture_class: SpendPostureClass,
    pub run_state_class: AiRunStateClass,
}

impl AiRouteSpendPosture {
    /// Live default for the M1 launch wedge: no dispatch, no spend.
    pub fn m1_seed_default() -> Self {
        Self {
            local_or_remote_path_class: LocalOrRemotePathClass::LocalNoDispatch,
            spend_posture_class: SpendPostureClass::NoSpendInM1Seed,
            run_state_class: AiRunStateClass::DispatchDisabledInM1Seed,
        }
    }

    /// Fixture-only mocked alternative used by the failure drill. Routes
    /// the run through a different (mocked) provider with a different
    /// spend class so the strip can prove the new posture surfaces
    /// verbatim.
    pub fn mocked_alternative_for_failure_drill() -> Self {
        Self {
            local_or_remote_path_class: LocalOrRemotePathClass::RemoteMockedForFixtures,
            spend_posture_class: SpendPostureClass::MockedSpendForFixtures,
            run_state_class: AiRunStateClass::PreviewPreDispatchMocked,
        }
    }
}

/// Coarse context summary the strip surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextSummary {
    pub mention_count: u32,
    pub resolved_mention_count: u32,
    pub attachment_count: u32,
    pub trusted_attachment_count: u32,
    pub tainted_attachment_count: u32,
    pub fenced_attachment_count: u32,
    pub slash_command_count: u32,
    pub resolved_slash_command_count: u32,
    pub aggregate_byte_estimate: u64,
    pub budget_byte_ceiling: u64,
}

impl ContextSummary {
    fn project(draft: &ComposerDraft) -> Self {
        let mention_count = draft.mentions.len() as u32;
        let resolved_mention_count = draft
            .mentions
            .iter()
            .filter(|mention| mention.resolution_state == MentionResolutionState::Resolved)
            .count() as u32;
        let attachment_count = draft.attachments.len() as u32;
        let trusted_attachment_count = draft
            .attachments
            .iter()
            .filter(|attachment| matches!(attachment.trust_posture, TrustPosture::TrustedFirstParty))
            .count() as u32;
        let tainted_attachment_count = draft
            .attachments
            .iter()
            .filter(|attachment| attachment.trust_posture.requires_fence())
            .count() as u32;
        let fenced_attachment_count = draft
            .attachments
            .iter()
            .filter(|attachment| attachment.placed_under_fenced_role)
            .count() as u32;
        let slash_command_count = draft.slash_command_invocations.len() as u32;
        let resolved_slash_command_count = draft
            .slash_command_invocations
            .iter()
            .filter(|invocation| {
                invocation.resolution_state == SlashCommandResolutionState::Resolved
            })
            .count() as u32;
        let aggregate_byte_estimate = draft
            .attachments
            .iter()
            .map(|attachment| attachment.estimated_byte_size)
            .fold(0u64, |acc, size| acc.saturating_add(size));
        Self {
            mention_count,
            resolved_mention_count,
            attachment_count,
            trusted_attachment_count,
            tainted_attachment_count,
            fenced_attachment_count,
            slash_command_count,
            resolved_slash_command_count,
            aggregate_byte_estimate,
            budget_byte_ceiling: draft.budget_byte_ceiling,
        }
    }

    fn render_summary_value(&self) -> String {
        format!(
            "mentions {resolved}/{total}, attachments {atotal} ({trusted} trusted, {tainted} tainted, {fenced} fenced), slash-commands {sresolved}/{stotal}, ~{bytes}/{ceiling} bytes",
            resolved = self.resolved_mention_count,
            total = self.mention_count,
            atotal = self.attachment_count,
            trusted = self.trusted_attachment_count,
            tainted = self.tainted_attachment_count,
            fenced = self.fenced_attachment_count,
            sresolved = self.resolved_slash_command_count,
            stotal = self.slash_command_count,
            bytes = self.aggregate_byte_estimate,
            ceiling = self.budget_byte_ceiling,
        )
    }
}

/// One typed result-lineage row. Every block reason on the upstream
/// validation outcome emits exactly one row so the packet's lineage
/// equals the draft's typed truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultLineageRow {
    pub block_reason_token: String,
    pub addressable_target_token: String,
}

impl ResultLineageRow {
    fn project(reason: &BlockReason) -> Self {
        let token = reason.as_str().to_owned();
        let target = match reason {
            BlockReason::UnresolvedMention { mention_id, .. } => format!("mention:{mention_id}"),
            BlockReason::StaleAttachment { attachment_id }
            | BlockReason::TaintedAttachmentOutsideFencedSection { attachment_id, .. }
            | BlockReason::OverBudgetContext { attachment_id }
            | BlockReason::PolicyBlockedAttachment { attachment_id }
            | BlockReason::OutOfScopeAttachment { attachment_id } => {
                format!("attachment:{attachment_id}")
            }
            BlockReason::UnresolvedSlashCommand { invocation_id, .. } => {
                format!("invocation:{invocation_id}")
            }
            BlockReason::PolicyBlockedRoute => "route:placeholder".to_owned(),
        };
        Self {
            block_reason_token: token,
            addressable_target_token: target,
        }
    }
}

/// Bounded M1 AI evidence-packet seed record. One record per inspected
/// composer draft on the launch AI wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidencePacketSeedRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub evidence_packet_id: String,
    pub prototype_label_token: String,
    pub prototype_label_text: String,
    pub composer_draft_ref: String,
    pub composer_session_ref: String,
    pub request_workspace_ref: String,
    pub exact_build_identity_ref: String,
    pub run_state_class: AiRunStateClass,
    pub provider_class: String,
    pub route_path_class: String,
    pub dispatch_target_class: String,
    pub local_or_remote_path_class: LocalOrRemotePathClass,
    pub spend_posture_class: SpendPostureClass,
    pub draft_state_token: String,
    pub context_summary: ContextSummary,
    pub result_lineage: Vec<ResultLineageRow>,
    pub claim_limits: Vec<AiTruthStripClaimLimit>,
    pub minted_at: String,
}

/// One typed truth-strip row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteSpendTruthStripRow {
    pub row_id: String,
    pub label: String,
    pub value_token: String,
    pub value_label: String,
}

/// Snapshot the chrome renders on the launch AI wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTruthStripSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub evidence_packet: AiEvidencePacketSeedRecord,
    pub truth_strip_rows: Vec<RouteSpendTruthStripRow>,
    pub invariant_violations: Vec<AiTruthStripInvariantViolation>,
    pub has_invariant_violations: bool,
}

/// Caller-supplied inputs that pin run identity, build identity, and the
/// timestamp the chrome quotes verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiTruthStripInputs<'a> {
    pub evidence_packet_id: &'a str,
    pub exact_build_identity_ref: &'a str,
    pub minted_at: &'a str,
}

impl AiTruthStripSnapshot {
    /// Project a snapshot from a composer draft, an explicit route/spend
    /// posture, and run-identity inputs.
    ///
    /// The wedge uses [`AiRouteSpendPosture::m1_seed_default`] on the
    /// live launch lane. Fixture replays may pass
    /// [`AiRouteSpendPosture::mocked_alternative_for_failure_drill`] to
    /// exercise the failure drill.
    pub fn project(
        draft: &ComposerDraft,
        posture: &AiRouteSpendPosture,
        inputs: AiTruthStripInputs<'_>,
    ) -> Self {
        let outcome = draft.validate();
        let context_summary = ContextSummary::project(draft);
        let result_lineage: Vec<ResultLineageRow> = outcome
            .block_reasons
            .iter()
            .map(ResultLineageRow::project)
            .collect();

        let prototype_label = PrototypeLabel::M1PrototypeEvidenceAndSpendSeed;
        let run_state_class = derive_run_state_class(posture, &outcome.state);

        let evidence_packet = AiEvidencePacketSeedRecord {
            record_kind: AI_EVIDENCE_SEED_RECORD_KIND.to_owned(),
            schema_version: AI_EVIDENCE_SEED_SCHEMA_VERSION,
            evidence_packet_id: inputs.evidence_packet_id.to_owned(),
            prototype_label_token: prototype_label.as_str().to_owned(),
            prototype_label_text: prototype_label.label().to_owned(),
            composer_draft_ref: draft.composer_draft_id.clone(),
            composer_session_ref: draft.composer_session_id.clone(),
            request_workspace_ref: draft.request_workspace_id.clone(),
            exact_build_identity_ref: inputs.exact_build_identity_ref.to_owned(),
            run_state_class,
            provider_class: draft.route_placeholder.provider_class.as_str().to_owned(),
            route_path_class: draft.route_placeholder.route_path_class.as_str().to_owned(),
            dispatch_target_class: draft
                .route_placeholder
                .dispatch_target_class
                .as_str()
                .to_owned(),
            local_or_remote_path_class: posture.local_or_remote_path_class,
            spend_posture_class: posture.spend_posture_class,
            draft_state_token: outcome.state.as_str().to_owned(),
            context_summary,
            result_lineage,
            claim_limits: AiTruthStripClaimLimit::canonical_list(),
            minted_at: inputs.minted_at.to_owned(),
        };

        let truth_strip_rows = build_truth_strip_rows(&evidence_packet, &draft.route_placeholder);
        let invariant_violations = validate_packet(&evidence_packet, draft);
        let has_invariant_violations = !invariant_violations.is_empty();

        Self {
            record_kind: AI_TRUTH_STRIP_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: AI_EVIDENCE_SEED_SCHEMA_VERSION,
            prototype_label_token: prototype_label.as_str().to_owned(),
            evidence_packet,
            truth_strip_rows,
            invariant_violations,
            has_invariant_violations,
        }
    }

    /// Export-safe subset of the snapshot. Returns a deterministic JSON
    /// string that carries only the typed tokens and counts; raw URLs,
    /// raw provider payloads, raw cost amounts, raw token counts, and
    /// raw credential bodies never appear. The chrome's "export run
    /// metadata" action emits this string verbatim into a support
    /// bundle.
    pub fn export_safe_run_metadata(&self) -> String {
        serde_json::to_string_pretty(&self.evidence_packet)
            .expect("evidence packet seed serializes through serde")
    }

    /// Deterministic plaintext render the support-bundle quote-export
    /// action and fixture replays both quote verbatim.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("AI evidence packet and route/spend truth strip\n");
        out.push_str(&format!(
            "Prototype: {label}\nEvidence packet: {pid}\nComposer draft: {draft}\nSession: {session}\nWorkspace: {ws}\nBuild identity: {build}\nMinted at: {minted}\n\n",
            label = self.evidence_packet.prototype_label_text,
            pid = self.evidence_packet.evidence_packet_id,
            draft = self.evidence_packet.composer_draft_ref,
            session = self.evidence_packet.composer_session_ref,
            ws = self.evidence_packet.request_workspace_ref,
            build = self.evidence_packet.exact_build_identity_ref,
            minted = self.evidence_packet.minted_at,
        ));
        out.push_str("[Route / spend truth strip]\n");
        for row in &self.truth_strip_rows {
            out.push_str(&format!(
                "  {label}: {value_label} [{token}]\n",
                label = row.label,
                value_label = row.value_label,
                token = row.value_token,
            ));
        }
        out.push('\n');
        out.push_str("[Result lineage]\n");
        if self.evidence_packet.result_lineage.is_empty() {
            out.push_str("  (no block reasons recorded)\n");
        } else {
            for row in &self.evidence_packet.result_lineage {
                out.push_str(&format!(
                    "  {reason} -> {target}\n",
                    reason = row.block_reason_token,
                    target = row.addressable_target_token,
                ));
            }
        }
        out.push('\n');
        out.push_str("[Claim limits]\n");
        for limit in &self.evidence_packet.claim_limits {
            out.push_str(&format!("  - {token}\n", token = limit.as_str()));
        }
        out.push('\n');
        out.push_str("[Invariants]\n");
        if self.invariant_violations.is_empty() {
            out.push_str("  (all clear)\n");
        } else {
            for violation in &self.invariant_violations {
                out.push_str(&format!("  - {token}\n", token = violation.as_str()));
            }
        }
        out
    }

    /// Locate one strip row by id.
    pub fn strip_row(&self, row_id: &str) -> Option<&RouteSpendTruthStripRow> {
        self.truth_strip_rows
            .iter()
            .find(|row| row.row_id == row_id)
    }
}

fn derive_run_state_class(
    posture: &AiRouteSpendPosture,
    draft_state: &ComposerDraftState,
) -> AiRunStateClass {
    // Fixture-only postures override the upstream draft state because the
    // failure drill must surface the alternative run-state class verbatim.
    if matches!(
        posture.run_state_class,
        AiRunStateClass::PreviewPreDispatchMocked
            | AiRunStateClass::PostRunCompletedMocked
            | AiRunStateClass::PostRunFailedMocked
    ) {
        return posture.run_state_class;
    }
    match draft_state {
        ComposerDraftState::BlockedPendingResolution => AiRunStateClass::BlockedPendingResolution,
        ComposerDraftState::Drafting
        | ComposerDraftState::ReadyForReviewOnly
        | ComposerDraftState::DispatchDisabledInM1Seed => AiRunStateClass::DispatchDisabledInM1Seed,
    }
}

fn build_truth_strip_rows(
    packet: &AiEvidencePacketSeedRecord,
    route: &RoutePlaceholder,
) -> Vec<RouteSpendTruthStripRow> {
    vec![
        RouteSpendTruthStripRow {
            row_id: "provider".to_owned(),
            label: "Provider".to_owned(),
            value_token: packet.provider_class.clone(),
            value_label: route.provider_class.label().to_owned(),
        },
        RouteSpendTruthStripRow {
            row_id: "route".to_owned(),
            label: "Route".to_owned(),
            value_token: packet.route_path_class.clone(),
            value_label: route.route_path_class.label().to_owned(),
        },
        RouteSpendTruthStripRow {
            row_id: "dispatch_target".to_owned(),
            label: "Dispatch target".to_owned(),
            value_token: packet.dispatch_target_class.clone(),
            value_label: route.dispatch_target_class.label().to_owned(),
        },
        RouteSpendTruthStripRow {
            row_id: "local_or_remote_path".to_owned(),
            label: "Local vs remote".to_owned(),
            value_token: packet.local_or_remote_path_class.as_str().to_owned(),
            value_label: packet.local_or_remote_path_class.label().to_owned(),
        },
        RouteSpendTruthStripRow {
            row_id: "spend_posture".to_owned(),
            label: "Spend".to_owned(),
            value_token: packet.spend_posture_class.as_str().to_owned(),
            value_label: packet.spend_posture_class.label().to_owned(),
        },
        RouteSpendTruthStripRow {
            row_id: "run_state".to_owned(),
            label: "Run state".to_owned(),
            value_token: packet.run_state_class.as_str().to_owned(),
            value_label: packet.run_state_class.label().to_owned(),
        },
        RouteSpendTruthStripRow {
            row_id: "context_summary".to_owned(),
            label: "Context".to_owned(),
            value_token: "context_summary".to_owned(),
            value_label: packet.context_summary.render_summary_value(),
        },
        RouteSpendTruthStripRow {
            row_id: "build_identity".to_owned(),
            label: "Build identity".to_owned(),
            value_token: packet.exact_build_identity_ref.clone(),
            value_label: packet.exact_build_identity_ref.clone(),
        },
    ]
}

fn validate_packet(
    packet: &AiEvidencePacketSeedRecord,
    draft: &ComposerDraft,
) -> Vec<AiTruthStripInvariantViolation> {
    let mut violations = Vec::new();

    if packet.prototype_label_token != PrototypeLabel::M1PrototypeEvidenceAndSpendSeed.as_str() {
        violations.push(AiTruthStripInvariantViolation::MissingPrototypeLabel);
    }

    if packet.claim_limits != AiTruthStripClaimLimit::canonical_list() {
        violations.push(AiTruthStripInvariantViolation::ClaimLimitsMissingOrOutOfOrder);
    }

    if packet.evidence_packet_id.is_empty() {
        violations.push(AiTruthStripInvariantViolation::EvidencePacketIdMissing);
    }

    if packet.exact_build_identity_ref.is_empty() {
        violations.push(AiTruthStripInvariantViolation::ExactBuildIdentityRefMissing);
    }

    // Route ↔ path consistency: a disabled dispatch target cannot pair
    // with a remote-mocked path class on the live wedge. The mocked
    // alternative is only legal when the run state itself is one of the
    // *_mocked variants — that's the fixture-only failure drill.
    let dispatch_disabled = draft.route_placeholder.dispatch_target_class
        == DispatchTargetClass::DisabledNoDispatchInM1Seed;
    let live_run_state = matches!(
        packet.run_state_class,
        AiRunStateClass::DispatchDisabledInM1Seed | AiRunStateClass::BlockedPendingResolution
    );
    if dispatch_disabled && live_run_state {
        match packet.local_or_remote_path_class {
            LocalOrRemotePathClass::LocalNoDispatch | LocalOrRemotePathClass::DisabledNoPath => {}
            LocalOrRemotePathClass::LocalOnlyCached
            | LocalOrRemotePathClass::RemoteMockedForFixtures => {
                violations.push(AiTruthStripInvariantViolation::RouteAndPathClassDisagree);
            }
        }
    }

    // A live dispatch-disabled wedge cannot claim a non-zero spend posture.
    if dispatch_disabled && live_run_state {
        match packet.spend_posture_class {
            SpendPostureClass::NoSpendInM1Seed => {}
            SpendPostureClass::MockedSpendForFixtures
            | SpendPostureClass::BundledNoIncrementalCost
            | SpendPostureClass::EstimatedUnverifiedBand => {
                violations.push(AiTruthStripInvariantViolation::SpendPostureContradictsRoute);
            }
        }
    }

    // A dispatch-disabled wedge cannot claim a post-run completed outcome.
    if dispatch_disabled
        && matches!(packet.run_state_class, AiRunStateClass::PostRunCompletedMocked)
    {
        violations.push(AiTruthStripInvariantViolation::DispatchDisabledButCompletedOutcomeClaimed);
    }

    // Independent draft-vs-packet honesty: any actionable upstream block
    // (anything other than the always-on policy-blocked-route marker)
    // must surface as BlockedPendingResolution unless the caller is in a
    // fixture-only *_mocked variant.
    let draft_blocked = draft.has_actionable_block_reasons();
    let fixture_only_state = matches!(
        packet.run_state_class,
        AiRunStateClass::PreviewPreDispatchMocked
            | AiRunStateClass::PostRunCompletedMocked
            | AiRunStateClass::PostRunFailedMocked
    );
    if draft_blocked
        && !fixture_only_state
        && !matches!(packet.run_state_class, AiRunStateClass::BlockedPendingResolution)
    {
        violations.push(AiTruthStripInvariantViolation::DraftBlockedButPacketClaimsReady);
    }

    violations
}

/// Convenience: the M1 launch-wedge canonical projection. Equivalent to
/// `AiTruthStripSnapshot::project` with
/// [`AiRouteSpendPosture::m1_seed_default`].
pub fn project_launch_wedge(
    draft: &ComposerDraft,
    inputs: AiTruthStripInputs<'_>,
) -> AiTruthStripSnapshot {
    AiTruthStripSnapshot::project(draft, &AiRouteSpendPosture::m1_seed_default(), inputs)
}

#[cfg(test)]
mod tests;
