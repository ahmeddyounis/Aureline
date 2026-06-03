//! Typed stabilization register for stable CLI/headless schemas, machine-readable output,
//! and support/export compatibility promises for the M4 stable line.
//!
//! This module is the **CLI/headless schema and compatibility stabilization register**. For every
//! CLI command surface, headless output schema, machine-readable output format, and support/export
//! compatibility promise it records one binding to the stable claim manifest entry whose lifecycle
//! label it backs, the proof packet that grounds it, the waiver (if any) holding it provisionally,
//! and the owner sign-off.
//!
//! The register is checked in at
//! `artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It carries no
//! raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_release::{
    FreshnessSloState, LaunchCutline, OwnerSignoff, PromotionDecision, ProofPacket,
    QualificationWaiver, StableClaimLevel,
};

/// Supported stabilization-register schema version.
pub const STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_RECORD_KIND: &str =
    "cli_headless_schema_machine_readable_output_stabilize";

/// Repo-relative path to the checked-in register.
pub const STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_PATH: &str =
    "artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json";

/// Embedded checked-in register JSON.
pub const STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_JSON: &str =
    include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json"
));

// ---------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------

/// The class of CLI/headless subject a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliHeadlessKind {
    /// A CLI command surface (command grammar, flags, exit codes).
    CliCommandSurface,
    /// A headless output schema (JSON, XML, etc.).
    HeadlessOutputSchema,
    /// A machine-readable output format (tab-separated, JSON-lines, etc.).
    MachineReadableFormat,
    /// A support/export compatibility promise.
    SupportExportCompatibility,
}

impl CliHeadlessKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CliCommandSurface,
        Self::HeadlessOutputSchema,
        Self::MachineReadableFormat,
        Self::SupportExportCompatibility,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CliCommandSurface => "cli_command_surface",
            Self::HeadlessOutputSchema => "headless_output_schema",
            Self::MachineReadableFormat => "machine_readable_format",
            Self::SupportExportCompatibility => "support_export_compatibility",
        }
    }
}

// ---------------------------------------------------------------------------
// States
// ---------------------------------------------------------------------------

/// Stabilization state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliHeadlessState {
    /// The row is finalized stable: a captured, within-SLO proof packet, complete evidence,
    /// and an owner sign-off back the public claim at its full canonical lifecycle label.
    FinalizedStable,
    /// The row carries the claim's full label only because an active, unexpired waiver covers
    /// a recorded gap.
    FinalizedOnWaiver,
    /// The proof packet or row evidence is incomplete, a required schema is missing, or
    /// a surface capability is absent; the row is not backed and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this row backs is itself below the cutline, so the row inherits that
    /// ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the row is not backed and
    /// the label must narrow.
    NarrowedStale,
    /// The row relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// The row evidence is incomplete (missing schema, unstable output, etc.); the label must narrow.
    NarrowedEvidenceIncomplete,
    /// A breaking schema change was detected; the row cannot finalize stable until the schema
    /// is stabilized and the label must narrow.
    NarrowedSchemaBreaking,
}

impl CliHeadlessState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::FinalizedStable,
        Self::FinalizedOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedEvidenceIncomplete,
        Self::NarrowedSchemaBreaking,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalizedStable => "finalized_stable",
            Self::FinalizedOnWaiver => "finalized_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedEvidenceIncomplete => "narrowed_evidence_incomplete",
            Self::NarrowedSchemaBreaking => "narrowed_schema_breaking",
        }
    }

    /// Whether the state lets a row carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::FinalizedStable | Self::FinalizedOnWaiver)
    }

    /// Whether the state forces the row below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

// ---------------------------------------------------------------------------
// Gap reasons
// ---------------------------------------------------------------------------

/// Closed reason a row narrows or a stabilization rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliHeadlessGapReason {
    /// The public claim this row backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The surface capability is absent or not yet implemented.
    SurfaceCapabilityAbsent,
    /// The row evidence is incomplete.
    EvidenceIncomplete,
    /// A breaking schema change was detected.
    SchemaBreakingChange,
    /// The output format is unstable or not yet frozen.
    OutputFormatUnstable,
    /// The machine-readable schema is missing.
    MachineReadableSchemaMissing,
    /// The support/export compatibility promise was breached.
    SupportExportCompatBreached,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// The waiver covering this row has expired.
    WaiverExpired,
    /// The owner has not signed off on this row.
    OwnerSignoffMissing,
}

impl CliHeadlessGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ClaimLabelNarrowed,
        Self::SurfaceCapabilityAbsent,
        Self::EvidenceIncomplete,
        Self::SchemaBreakingChange,
        Self::OutputFormatUnstable,
        Self::MachineReadableSchemaMissing,
        Self::SupportExportCompatBreached,
        Self::ProofPacketFreshnessBreached,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::SchemaBreakingChange => "schema_breaking_change",
            Self::OutputFormatUnstable => "output_format_unstable",
            Self::MachineReadableSchemaMissing => "machine_readable_schema_missing",
            Self::SupportExportCompatBreached => "support_export_compat_breached",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

/// Action a stabilization rule recommends when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliHeadlessAction {
    /// Hold publication until the gap is resolved.
    HoldPublication,
    /// Narrow the public claim label below the cutline.
    NarrowLabel,
    /// Refresh the proof packet to bring it within its freshness SLO.
    RefreshProofPacket,
    /// Recapture evidence for the row.
    RecaptureEvidence,
    /// Request owner sign-off.
    RequestOwnerSignoff,
}

impl CliHeadlessAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

// ---------------------------------------------------------------------------
// Detail structs
// ---------------------------------------------------------------------------

/// Detail for a CLI command surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliSchemaDetail {
    /// Frozen command-surface version.
    pub command_surface_version: String,
    /// Stable commands in this surface.
    #[serde(default)]
    pub stable_commands: Vec<String>,
    /// Stable flags in this surface.
    #[serde(default)]
    pub stable_flags: Vec<String>,
    /// URL to the schema documentation.
    pub schema_doc_url: String,
}

/// Detail for a machine-readable output format row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MachineReadableDetail {
    /// Output format name (e.g. "json_lines").
    pub output_format: String,
    /// Schema version for the format.
    pub schema_version: String,
    /// URL to the schema documentation.
    pub schema_doc_url: String,
    /// Date the format was frozen stable.
    pub stable_since: String,
}

/// Detail for a support/export compatibility promise row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportExportCompatDetail {
    /// Human-readable compatibility promise.
    pub compat_promise: String,
    /// Supported export format versions.
    #[serde(default)]
    pub supported_export_versions: Vec<String>,
    /// URL to the rollback checkpoint documentation.
    pub rollback_checkpoint_url: String,
}

// ---------------------------------------------------------------------------
// Row
// ---------------------------------------------------------------------------

/// One row in the stabilization register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliHeadlessRow {
    /// Stable row id.
    pub entry_id: String,
    /// The kind of CLI/headless subject.
    pub kind: CliHeadlessKind,
    /// The surface id.
    pub surface_ref: String,
    /// Human-readable summary of the surface.
    pub surface_summary: String,
    /// Whether this row is part of the release-blocking set.
    pub release_blocking: bool,
    /// Ref to the stable claim manifest entry this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label from the claim manifest.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// The stabilization state earned.
    pub state: CliHeadlessState,
    /// Active gap reasons narrowing this row.
    #[serde(default)]
    pub active_gap_reasons: Vec<CliHeadlessGapReason>,
    /// The proof packet backing this row.
    pub proof_packet: ProofPacket,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Waiver, if any.
    pub waiver: Option<QualificationWaiver>,
    /// CLI schema detail, present when kind is [`CliHeadlessKind::CliCommandSurface`].
    pub schema_detail: Option<CliSchemaDetail>,
    /// Machine-readable output detail, present when kind is [`CliHeadlessKind::MachineReadableFormat`].
    pub output_detail: Option<MachineReadableDetail>,
    /// Support/export compatibility detail, present when kind is [`CliHeadlessKind::SupportExportCompatibility`].
    pub compat_detail: Option<SupportExportCompatDetail>,
}

impl CliHeadlessRow {
    /// Whether the effective label is at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        self.effective_label.rank() >= StableClaimLevel::Stable.rank()
    }

    /// Whether the row holds its public claim label.
    pub fn holds_label(&self) -> bool {
        self.state.holds_label()
    }

    /// Whether the backing public claim is at or above the stable cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.rank() >= StableClaimLevel::Stable.rank()
    }

    /// Whether the row has the given gap reason active.
    pub fn has_active_reason(&self, reason: CliHeadlessGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

// ---------------------------------------------------------------------------
// Rule
// ---------------------------------------------------------------------------

/// One stabilization stop rule: a closed condition that gates stable publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliHeadlessRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// Gap reason that triggers this rule.
    pub trigger_reason: CliHeadlessGapReason,
    /// Lifecycle labels this rule applies to.
    #[serde(default)]
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action when the rule fires.
    pub default_action: CliHeadlessAction,
    /// Whether a firing rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Computed summary of the stabilization register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliHeadlessSummary {
    /// Total rows.
    pub total_entries: usize,
    /// Unique claim refs.
    pub total_claims: usize,
    /// Rows at or above the cutline.
    pub entries_finalized_stable: usize,
    /// Rows below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Rows carried on an active waiver.
    pub entries_on_active_waiver: usize,
    /// Release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows at or above the cutline.
    pub release_blocking_finalized_stable: usize,
    /// Release-blocking rows below the cutline.
    pub release_blocking_narrowed: usize,
    /// CLI command surface rows.
    pub cli_command_surface_entries: usize,
    /// Headless output schema rows.
    pub headless_output_schema_entries: usize,
    /// Machine-readable format rows.
    pub machine_readable_format_entries: usize,
    /// Support/export compatibility rows.
    pub support_export_compatibility_entries: usize,
    /// Rows with current proof packets.
    pub packets_current: usize,
    /// Rows with proof packets due for refresh.
    pub packets_due_for_refresh: usize,
    /// Rows with breached proof packets.
    pub packets_breached: usize,
    /// Rows with missing proof packets.
    pub packets_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Rules currently firing.
    pub rules_firing: usize,
}

// ---------------------------------------------------------------------------
// Publication record
// ---------------------------------------------------------------------------

/// Publication decision derived from the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliHeadlessPublicationRecord {
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Rule ids that block publication.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Entry ids that block publication.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// UTC date the decision was recorded.
    pub recorded_at: String,
}

// ---------------------------------------------------------------------------
// Register
// ---------------------------------------------------------------------------

/// The stabilization register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilizeStableCliHeadlessSchemasMachineReadableOutput {
    pub schema_version: u32,
    pub record_kind: String,
    pub register_id: String,
    pub status: String,
    pub overview_page: String,
    pub as_of: String,
    pub claim_manifest_ref: String,
    #[serde(default)]
    pub lifecycle_labels: Vec<StableClaimLevel>,
    #[serde(default)]
    pub kinds: Vec<CliHeadlessKind>,
    #[serde(default)]
    pub states: Vec<CliHeadlessState>,
    #[serde(default)]
    pub gap_reasons: Vec<CliHeadlessGapReason>,
    #[serde(default)]
    pub actions: Vec<CliHeadlessAction>,
    pub launch_cutline: LaunchCutline,
    #[serde(default)]
    pub release_blocking_surface_refs: Vec<String>,
    #[serde(default)]
    pub rules: Vec<CliHeadlessRule>,
    #[serde(default)]
    pub rows: Vec<CliHeadlessRow>,
    pub summary: CliHeadlessSummary,
    pub publication: CliHeadlessPublicationRecord,
}

impl StabilizeStableCliHeadlessSchemasMachineReadableOutput {
    /// Rows of the given kind.
    pub fn rows_for_kind(&self, kind: CliHeadlessKind) -> Vec<&CliHeadlessRow> {
        self.rows.iter().filter(|row| row.kind == kind).collect()
    }

    /// Unique claim refs.
    pub fn claims(&self) -> BTreeSet<&str> {
        self.rows.iter().map(|row| row.claim_ref.as_str()).collect()
    }

    /// Rows at or above the stable cutline.
    pub fn rows_published_stable(&self) -> Vec<&CliHeadlessRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Rows below the stable cutline.
    pub fn rows_narrowed(&self) -> Vec<&CliHeadlessRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Whether a rule is currently firing.
    pub fn rule_fires(&self, rule: &CliHeadlessRule) -> bool {
        self.rows.iter().any(|row| {
            row.claim_holds_stable() && row.active_gap_reasons.contains(&rule.trigger_reason)
        })
    }

    /// Recomputes the publication decision from the rows and rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        let has_blocking = self.rows.iter().any(|row| {
            row.claim_holds_stable()
                && row.active_gap_reasons.iter().any(|reason| {
                    self.rules
                        .iter()
                        .any(|rule| rule.blocks_publication && rule.trigger_reason == *reason)
                })
        });
        if has_blocking {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Recomputes the blocking rule ids.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if rule.blocks_publication && self.rule_fires(rule) {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the blocking entry ids.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<CliHeadlessGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> CliHeadlessSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: CliHeadlessKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&CliHeadlessRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        CliHeadlessSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_finalized_stable: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.state == CliHeadlessState::FinalizedOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_finalized_stable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            cli_command_surface_entries: kind(CliHeadlessKind::CliCommandSurface),
            headless_output_schema_entries: kind(CliHeadlessKind::HeadlessOutputSchema),
            machine_readable_format_entries: kind(CliHeadlessKind::MachineReadableFormat),
            support_export_compatibility_entries: kind(CliHeadlessKind::SupportExportCompatibility),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .rules
                .iter()
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register.
    pub fn support_export_projection(&self) -> CliHeadlessExportProjection {
        CliHeadlessExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| CliHeadlessExportRow {
                    entry_id: row.entry_id.clone(),
                    kind: row.kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    publishes_stable: row.publishes_stable(),
                    state: row.state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<CliHeadlessViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(CliHeadlessViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(CliHeadlessViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(CliHeadlessViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<CliHeadlessViolation>) {
        if self.schema_version
            != STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_SCHEMA_VERSION
        {
            violations.push(CliHeadlessViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_RECORD_KIND
        {
            violations.push(CliHeadlessViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CliHeadlessViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.kinds != CliHeadlessKind::ALL.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch { field: "kinds" });
        }
        if self.states != CliHeadlessState::ALL.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch { field: "states" });
        }
        if self.gap_reasons != CliHeadlessGapReason::ALL.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.actions != CliHeadlessAction::ALL.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch { field: "actions" });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(CliHeadlessViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(CliHeadlessViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(CliHeadlessViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<CliHeadlessViolation>) {
        if self.rules.is_empty() {
            violations.push(CliHeadlessViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(CliHeadlessViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(CliHeadlessViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(CliHeadlessViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in CliHeadlessGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(CliHeadlessViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &CliHeadlessRow, violations: &mut Vec<CliHeadlessViolation>) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CliHeadlessViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(CliHeadlessViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.effective_label,
            });
        }

        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(CliHeadlessViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(CliHeadlessViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // Kind-specific completeness checks.
        if let Some(detail) = &row.schema_detail {
            if detail.schema_doc_url.trim().is_empty() {
                violations.push(CliHeadlessViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "schema_detail.schema_doc_url",
                });
            }
        }
        if let Some(detail) = &row.output_detail {
            if detail.schema_doc_url.trim().is_empty() {
                violations.push(CliHeadlessViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "output_detail.schema_doc_url",
                });
            }
        }
        if let Some(detail) = &row.compat_detail {
            if detail.rollback_checkpoint_url.trim().is_empty() {
                violations.push(CliHeadlessViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "compat_detail.rollback_checkpoint_url",
                });
            }
        }

        // Coherence: a narrowed state must have an active gap reason, and a held label must not.
        if row.holds_label() {
            if row.effective_label != row.claim_label {
                violations.push(CliHeadlessViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(CliHeadlessViolation::HeldWithActiveReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row.proof_packet.slo_state != FreshnessSloState::Current {
                violations.push(CliHeadlessViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state: row.proof_packet.slo_state,
                });
            }
            if !row.owner_signoff.signed_off {
                violations.push(CliHeadlessViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            if row.effective_label.rank() >= row.claim_label.rank() {
                violations.push(CliHeadlessViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.state,
                    published: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(CliHeadlessViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.state,
                });
            }
        }

        if row.state == CliHeadlessState::FinalizedOnWaiver && row.waiver.is_none() {
            violations.push(CliHeadlessViolation::OnWaiverWithoutWaiver {
                entry_id: row.entry_id.clone(),
            });
        }

        if row.state == CliHeadlessState::NarrowedStale
            && !row.has_active_reason(CliHeadlessGapReason::ProofPacketFreshnessBreached)
        {
            violations.push(CliHeadlessViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.state,
                expected_reason: CliHeadlessGapReason::ProofPacketFreshnessBreached,
            });
        }

        if row.state == CliHeadlessState::NarrowedEvidenceIncomplete
            && !row.has_active_reason(CliHeadlessGapReason::EvidenceIncomplete)
        {
            violations.push(CliHeadlessViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.state,
                expected_reason: CliHeadlessGapReason::EvidenceIncomplete,
            });
        }

        if row.state == CliHeadlessState::NarrowedSchemaBreaking
            && !row.has_active_reason(CliHeadlessGapReason::SchemaBreakingChange)
        {
            violations.push(CliHeadlessViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.state,
                expected_reason: CliHeadlessGapReason::SchemaBreakingChange,
            });
        }

        if row.release_blocking
            && !self
                .release_blocking_surface_refs
                .contains(&row.surface_ref)
        {
            violations.push(CliHeadlessViolation::ReleaseBlockingSurfaceNotDeclared {
                entry_id: row.entry_id.clone(),
                surface_ref: row.surface_ref.clone(),
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<CliHeadlessViolation>) {
        for kind in CliHeadlessKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(CliHeadlessViolation::KindUncovered { kind });
            }
        }
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &self.release_blocking_surface_refs {
            if !covered.contains(declared.as_str()) {
                violations.push(CliHeadlessViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: declared.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<CliHeadlessViolation>) {
        if self.publication.decision != self.computed_publication_decision() {
            violations.push(CliHeadlessViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed: self.computed_publication_decision(),
            });
        }
        let computed_blocking_rule_ids = self.computed_blocking_rule_ids();
        let computed_blocking_entry_ids = self.computed_blocking_entry_ids();
        if self.publication.blocking_rule_ids != computed_blocking_rule_ids {
            violations.push(CliHeadlessViolation::BlockingRuleIdsMismatch {
                recorded: self.publication.blocking_rule_ids.clone(),
                computed: computed_blocking_rule_ids,
            });
        }
        if self.publication.blocking_entry_ids != computed_blocking_entry_ids {
            violations.push(CliHeadlessViolation::BlockingEntryIdsMismatch {
                recorded: self.publication.blocking_entry_ids.clone(),
                computed: computed_blocking_entry_ids,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Export projection
// ---------------------------------------------------------------------------

/// Export/Help-About-safe projection of one stabilization row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliHeadlessExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The kind of CLI/headless subject.
    pub kind: CliHeadlessKind,
    /// The surface id.
    pub surface_ref: String,
    /// Whether the row is release-blocking.
    pub release_blocking: bool,
    /// The claim ref.
    pub claim_ref: String,
    /// The canonical claim label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the effective label is at or above the cutline.
    pub publishes_stable: bool,
    /// The stabilization state.
    pub state: CliHeadlessState,
    /// The freshness-SLO state of the proof packet.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    #[serde(default)]
    pub active_gap_reasons: Vec<CliHeadlessGapReason>,
}

/// Export/Help-About-safe projection of the stabilization register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliHeadlessExportProjection {
    /// Stable register id.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The publication decision.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<CliHeadlessExportRow>,
}

// ---------------------------------------------------------------------------
// Violations
// ---------------------------------------------------------------------------

/// Every structural or logical violation the validation can detect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliHeadlessViolation {
    /// The schema version does not match the supported version.
    UnsupportedSchemaVersion { actual: u32 },
    /// The record kind does not match the expected kind.
    UnsupportedRecordKind { actual: String },
    /// A required string field is empty or missing.
    EmptyField {
        entry_id: String,
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch { field: &'static str },
    /// No rules are defined.
    NoRules,
    /// A rule id appears more than once.
    DuplicateRuleId { rule_id: String },
    /// A rule watches no labels.
    RuleWithoutLabels { rule_id: String },
    /// A gap reason has no rule that covers it.
    GapReasonWithoutRule { reason: CliHeadlessGapReason },
    /// A row id appears more than once.
    DuplicateEntryId { entry_id: String },
    /// The register has no rows.
    EmptyRegister,
    /// A kind has no covering row.
    KindUncovered { kind: CliHeadlessKind },
    /// A declared release-blocking surface ref has no covering row.
    ReleaseBlockingSurfaceUncovered { surface_ref: String },
    /// A row's effective label is wider than its claim's canonical label.
    PublishedWiderThanClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    /// A row's freshness SLO window is inconsistent.
    FreshnessSloInconsistent { entry_id: String },
    /// A row holds its label but the label does not equal the claim.
    HeldLabelNotEqualClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    /// A row holds its label but has active gap reasons.
    HeldWithActiveReason { entry_id: String },
    /// A row holds its label but its packet is not fresh.
    HeldOnStalePacket {
        entry_id: String,
        slo_state: FreshnessSloState,
    },
    /// A row holds its label but lacks owner sign-off.
    HeldWithoutSignoff { entry_id: String },
    /// A row is narrowed but its published label is not narrowed.
    PublishedLabelNotNarrowed {
        entry_id: String,
        state: CliHeadlessState,
        published: StableClaimLevel,
    },
    /// A row is narrowed but has no active gap reason.
    NarrowingWithoutReason {
        entry_id: String,
        state: CliHeadlessState,
    },
    /// A row is on waiver but has no waiver record.
    OnWaiverWithoutWaiver { entry_id: String },
    /// A row's state does not match its expected active gap reason.
    StateReasonMismatch {
        entry_id: String,
        state: CliHeadlessState,
        expected_reason: CliHeadlessGapReason,
    },
    /// A release-blocking row references a surface not declared in the register.
    ReleaseBlockingSurfaceNotDeclared {
        entry_id: String,
        surface_ref: String,
    },
    /// The stored publication decision does not match the computed decision.
    PublicationDecisionInconsistent {
        declared: PromotionDecision,
        computed: PromotionDecision,
    },
    /// The stored blocking rule ids do not match the computed ids.
    BlockingRuleIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    /// The stored blocking entry ids do not match the computed ids.
    BlockingEntryIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    /// The summary counts do not match the computed counts.
    SummaryMismatch,
}

impl fmt::Display for CliHeadlessViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind {actual}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => {
                write!(f, "field {field_name} is empty on {entry_id}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch for {field}")
            }
            Self::NoRules => write!(f, "register has no rules"),
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason {} has no rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::KindUncovered { kind } => {
                write!(f, "kind {} has no covering row", kind.as_str())
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "published label {published:?} is wider than claim {claim:?} on {entry_id}"
                )
            }
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "freshness SLO inconsistent on {entry_id}")
            }
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "row {entry_id} holds label {published:?} but claim is {claim:?}"
                )
            }
            Self::HeldWithActiveReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label but has active gap reasons"
                )
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "row {entry_id} holds its label but its packet SLO state is {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds its label but lacks owner sign-off")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => {
                write!(
                    f,
                    "row {entry_id} is in state {state:?} but published label {published:?} is not narrowed"
                )
            }
            Self::NarrowingWithoutReason { entry_id, state } => {
                write!(
                    f,
                    "row {entry_id} is in state {state:?} but has no active gap reason"
                )
            }
            Self::OnWaiverWithoutWaiver { entry_id } => {
                write!(f, "row {entry_id} is on waiver but has no waiver record")
            }
            Self::StateReasonMismatch {
                entry_id,
                state,
                expected_reason,
            } => {
                write!(
                    f,
                    "row {entry_id} is in state {state:?} but lacks expected reason {expected_reason:?}"
                )
            }
            Self::ReleaseBlockingSurfaceNotDeclared {
                entry_id,
                surface_ref,
            } => {
                write!(
                    f,
                    "row {entry_id} is release-blocking but its surface {surface_ref} is not in release_blocking_surface_refs"
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication decision {declared:?} does not match computed {computed:?}"
                )
            }
            Self::BlockingRuleIdsMismatch { recorded, computed } => {
                write!(
                    f,
                    "blocking rule ids {recorded:?} do not match computed {computed:?}"
                )
            }
            Self::BlockingEntryIdsMismatch { recorded, computed } => {
                write!(
                    f,
                    "blocking entry ids {recorded:?} do not match computed {computed:?}"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match computed summary"),
        }
    }
}

impl Error for CliHeadlessViolation {}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Loads the embedded stabilization register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`StabilizeStableCliHeadlessSchemasMachineReadableOutput`].
pub fn current_stabilize_stable_cli_headless_schemas_machine_readable_output(
) -> Result<StabilizeStableCliHeadlessSchemasMachineReadableOutput, Box<dyn Error>> {
    Ok(serde_json::from_str(
        STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_JSON,
    )?)
}

#[cfg(test)]
fn register() -> StabilizeStableCliHeadlessSchemasMachineReadableOutput {
    current_stabilize_stable_cli_headless_schemas_machine_readable_output()
        .expect("register parses")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_register_parses_and_validates() {
        let reg = register();
        assert_eq!(
            reg.schema_version,
            STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_SCHEMA_VERSION
        );
        assert_eq!(
            reg.record_kind,
            STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_RECORD_KIND
        );
        assert_eq!(reg.validate(), Vec::new());
        assert!(!reg.rows.is_empty());
    }

    #[test]
    fn every_kind_is_covered() {
        let reg = register();
        for kind in CliHeadlessKind::ALL {
            assert!(
                !reg.rows_for_kind(kind).is_empty(),
                "kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_surface_is_covered() {
        let reg = register();
        let covered: BTreeSet<&str> = reg
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        assert!(!reg.release_blocking_surface_refs.is_empty());
        for declared in &reg.release_blocking_surface_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn register_exercises_published_and_narrowed_rows() {
        let reg = register();
        assert!(
            !reg.rows_published_stable().is_empty(),
            "register must show at least one published-stable row"
        );
        assert!(
            !reg.rows_narrowed().is_empty(),
            "register must show at least one narrowed row"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let reg = register();
        assert_eq!(reg.summary, reg.computed_summary());
        assert_eq!(
            reg.summary.entries_finalized_stable + reg.summary.entries_narrowed_below_cutline,
            reg.rows.len()
        );
        assert_eq!(
            reg.summary.packets_current
                + reg.summary.packets_due_for_refresh
                + reg.summary.packets_breached
                + reg.summary.packets_missing,
            reg.rows.len()
        );
        assert_eq!(
            reg.summary.cli_command_surface_entries
                + reg.summary.headless_output_schema_entries
                + reg.summary.machine_readable_format_entries
                + reg.summary.support_export_compatibility_entries,
            reg.rows.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let reg = register();
        assert_eq!(reg.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            reg.publication.decision,
            reg.computed_publication_decision()
        );
        assert!(!reg.publication.blocking_rule_ids.is_empty());
        assert!(!reg.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let reg = register();
        let covered: BTreeSet<CliHeadlessGapReason> =
            reg.rules.iter().map(|rule| rule.trigger_reason).collect();
        for reason in CliHeadlessGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_publishes_wider_than_its_claim_ceiling() {
        let reg = register();
        for row in &reg.rows {
            assert!(
                row.effective_label.rank() <= row.claim_label.rank(),
                "{} publishes wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_publication_wider_than_ceiling() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| !row.publishes_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.effective_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            CliHeadlessViolation::PublishedWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.state == CliHeadlessState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.effective_label = row.claim_label;
        reg.summary = reg.computed_summary();
        reg.publication.decision = reg.computed_publication_decision();
        reg.publication.blocking_rule_ids = reg.computed_blocking_rule_ids();
        reg.publication.blocking_entry_ids = reg.computed_blocking_entry_ids();
        assert!(reg
            .validate()
            .iter()
            .any(|v| matches!(v, CliHeadlessViolation::PublishedLabelNotNarrowed { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut reg = register();
        reg.publication.decision = PromotionDecision::Proceed;
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            CliHeadlessViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_row_without_signoff() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a backed row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let entry_id = row.entry_id.clone();
        reg.summary = reg.computed_summary();
        assert!(reg
            .validate()
            .contains(&CliHeadlessViolation::HeldWithoutSignoff { entry_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let reg = register();
        let projection = reg.support_export_projection();
        assert_eq!(projection.rows.len(), reg.rows.len());
        assert_eq!(projection.publication_decision, reg.publication.decision);
        for (row, projected) in reg.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.surface_ref, projected.surface_ref);
            assert_eq!(row.publishes_stable(), projected.publishes_stable);
            assert_eq!(row.effective_label, projected.effective_label);
            assert_eq!(row.proof_packet.slo_state, projected.slo_state);
        }
    }
}
