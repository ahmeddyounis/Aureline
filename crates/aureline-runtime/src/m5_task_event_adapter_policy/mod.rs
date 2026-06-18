//! Frozen task-event adapter-policy baseline shared by build, test, debug,
//! pipeline, coverage, notebook, CLI/headless, and support/export surfaces.
//!
//! This module freezes the policy layer the canonical build/test event envelope
//! depends on: an explicit adapter-priority ladder (native, BSP, Bazel BEP/BES,
//! structured output, heuristic parser), a raw-payload-retention matrix, one
//! closed downgrade vocabulary, and the consumer bindings that prove every later
//! execution surface reads the same envelope instead of inferring truth from
//! rendered output. It deliberately reuses the
//! [`crate::build_test_event_interoperability`] source-kind, confidence,
//! retention-class, lifecycle, provenance, severity, and promotion vocabulary
//! rather than minting parallel tokens.
//!
//! The invariant this layer protects is attribution: a lower-priority adapter
//! never masquerades as native/BSP/BEP truth. When more than one adapter
//! observes the same target and lifecycle kind, arbitration keeps the
//! highest-priority emission authoritative and forces every shadowing emission
//! to a visible, reason-bearing downgrade under its own confidence ceiling.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/task-event-and-adapter-policy.md`](../../../docs/m5/task-event-and-adapter-policy.md);
//! the machine-readable boundaries live at
//! [`/schemas/tooling/task-event-envelope.schema.json`](../../../schemas/tooling/task-event-envelope.schema.json)
//! and
//! [`/schemas/tooling/adapter-capability.schema.json`](../../../schemas/tooling/adapter-capability.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    BuildTestEventConfidence, BuildTestEventKind, BuildTestEventProvenance,
    BuildTestEventSourceKind, BuildTestInteropFindingSeverity, BuildTestInteropPromotionState,
    RawPayloadRetentionClass,
};

/// Stable record-kind tag for [`TaskEventAdapterPolicyBaseline`].
pub const TASK_EVENT_ADAPTER_POLICY_RECORD_KIND: &str = "m5_task_event_adapter_policy_baseline";

/// Stable record-kind tag for [`TaskEventAdapterPolicySupportExport`].
pub const TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_task_event_adapter_policy_support_export";

/// Integer schema version for the policy baseline.
pub const TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the adapter-capability boundary schema.
pub const TASK_EVENT_ADAPTER_POLICY_CAPABILITY_SCHEMA_REF: &str =
    "schemas/tooling/adapter-capability.schema.json";

/// Repo-relative path of the task-event envelope boundary schema.
pub const TASK_EVENT_ADAPTER_POLICY_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const TASK_EVENT_ADAPTER_POLICY_DOC_REF: &str = "docs/m5/task-event-and-adapter-policy.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR: &str = "fixtures/tooling/m5/bsp-bep-native";

/// Repo-relative path of the checked-in baseline artifact.
pub const TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Stable baseline id minted by the seed.
pub const TASK_EVENT_ADAPTER_POLICY_BASELINE_ID: &str = "tooling:m5:task-event-adapter-policy:v1";

/// Stable support-export id minted by the seed inspector.
pub const TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:task-event-adapter-policy";

/// The M4 build/test interoperability contract this baseline extends.
pub const TASK_EVENT_ADAPTER_POLICY_SEED_CONTRACT_REF: &str =
    "schemas/runtime/build-test-event-envelope.schema.json";

/// Closed downgrade-reason vocabulary for the whole task-event lane.
///
/// Every reduced-certainty emission names exactly one of these and remains
/// visibly downgraded on every consumer projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The adapter supports the source only partially.
    PartialSupport,
    /// A heuristic parser stood in for a structured or native adapter.
    HeuristicFallback,
    /// A replay or re-ingest could not reconstruct the full emission.
    ReplayGap,
    /// A negotiated capability the consumer expected was unsupported.
    UnsupportedAdapterCapability,
}

impl DowngradeReason {
    /// Every downgrade reason in stable declaration order.
    pub const ALL: [Self; 4] = [
        Self::PartialSupport,
        Self::HeuristicFallback,
        Self::ReplayGap,
        Self::UnsupportedAdapterCapability,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialSupport => "partial_support",
            Self::HeuristicFallback => "heuristic_fallback",
            Self::ReplayGap => "replay_gap",
            Self::UnsupportedAdapterCapability => "unsupported_adapter_capability",
        }
    }
}

/// Later M5 execution surface that consumes the canonical task-event envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventConsumer {
    /// Pipeline / run-control overlays.
    Pipeline,
    /// Coverage overlays and merge sheets.
    Coverage,
    /// Snapshot / flaky / golden intelligence.
    SnapshotFlaky,
    /// Notebook run cells and kernel-backed tests.
    NotebookRun,
    /// CLI / headless stable JSON surface.
    CliHeadless,
    /// Support and release export packets.
    SupportExport,
}

impl TaskEventConsumer {
    /// Every M5 consumer that must inherit the canonical envelope.
    pub const REQUIRED: [Self; 6] = [
        Self::Pipeline,
        Self::Coverage,
        Self::SnapshotFlaky,
        Self::NotebookRun,
        Self::CliHeadless,
        Self::SupportExport,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pipeline => "pipeline",
            Self::Coverage => "coverage",
            Self::SnapshotFlaky => "snapshot_flaky",
            Self::NotebookRun => "notebook_run",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// Canonical priority rank for a source kind on the adapter ladder.
///
/// Lower numbers are higher authority. The ladder is frozen: native truth wins
/// over BSP, which wins over Bazel BEP/BES, which wins over structured output,
/// which wins over a heuristic parser fallback.
pub const fn canonical_priority_rank(source_kind: BuildTestEventSourceKind) -> u8 {
    match source_kind {
        BuildTestEventSourceKind::Native => 1,
        BuildTestEventSourceKind::Bsp => 2,
        BuildTestEventSourceKind::BazelBep => 3,
        BuildTestEventSourceKind::StructuredOutput => 4,
        BuildTestEventSourceKind::HeuristicParser => 5,
    }
}

/// Maximum confidence a source kind is allowed to assert on any emission.
pub const fn canonical_confidence_ceiling(
    source_kind: BuildTestEventSourceKind,
) -> BuildTestEventConfidence {
    match source_kind {
        BuildTestEventSourceKind::Native
        | BuildTestEventSourceKind::Bsp
        | BuildTestEventSourceKind::BazelBep => BuildTestEventConfidence::High,
        BuildTestEventSourceKind::StructuredOutput => BuildTestEventConfidence::MediumHigh,
        BuildTestEventSourceKind::HeuristicParser => BuildTestEventConfidence::Low,
    }
}

/// True when the source kind is a first-party / negotiated-protocol authority.
pub const fn source_is_authoritative(source_kind: BuildTestEventSourceKind) -> bool {
    matches!(
        source_kind,
        BuildTestEventSourceKind::Native
            | BuildTestEventSourceKind::Bsp
            | BuildTestEventSourceKind::BazelBep
    )
}

/// Numeric weight used to compare confidence levels (higher is stronger).
const fn confidence_weight(confidence: BuildTestEventConfidence) -> u8 {
    match confidence {
        BuildTestEventConfidence::High => 4,
        BuildTestEventConfidence::MediumHigh => 3,
        BuildTestEventConfidence::Medium => 2,
        BuildTestEventConfidence::Low => 1,
    }
}

/// True when `confidence` is above the ceiling allowed for `source_kind`.
fn confidence_overclaims(
    confidence: BuildTestEventConfidence,
    source_kind: BuildTestEventSourceKind,
) -> bool {
    confidence_weight(confidence) > confidence_weight(canonical_confidence_ceiling(source_kind))
}

/// One rung of the frozen adapter-priority ladder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterPriorityRung {
    /// Source kind this rung covers.
    pub source_kind: BuildTestEventSourceKind,
    /// Priority rank (1 is highest authority, 5 lowest).
    pub priority_rank: u8,
    /// Maximum confidence this source may assert.
    pub confidence_ceiling: BuildTestEventConfidence,
    /// True when the source is first-party or negotiated-protocol truth.
    pub authoritative: bool,
    /// True when the source must never present as a higher-priority truth.
    pub masquerade_blocked: bool,
    /// Short reviewer-facing note.
    pub note: String,
}

/// One cell of the raw-payload-retention matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawPayloadRetentionCell {
    /// Source kind this cell covers.
    pub source_kind: BuildTestEventSourceKind,
    /// Retention class for the cell.
    pub retention_class: RawPayloadRetentionClass,
    /// True when this source may use this retention class.
    pub allowed: bool,
    /// True when this is the default retention posture for the source.
    pub is_default: bool,
    /// True when the retention class requires an approval gate.
    pub approval_required: bool,
    /// Short reviewer-facing note.
    pub note: String,
}

/// One entry in the closed downgrade vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeVocabularyEntry {
    /// Downgrade reason.
    pub reason: DowngradeReason,
    /// Short reviewer-facing summary.
    pub summary: String,
    /// True when the reason forces a visible downgrade on every projection.
    pub forces_visible_downgrade: bool,
    /// Confidence ceiling applied while this downgrade is in force.
    pub max_confidence: BuildTestEventConfidence,
}

/// Binding proving a later M5 surface reads the canonical envelope verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventConsumerBinding {
    /// Consumer surface.
    pub consumer: TaskEventConsumer,
    /// Stable binding ref.
    pub binding_ref: String,
    /// True when the surface reads the canonical envelope, not rendered output.
    pub reads_canonical_envelope: bool,
    /// True when the surface preserves source kind.
    pub preserves_source_kind: bool,
    /// True when the surface preserves the adapter priority rank.
    pub preserves_priority_rank: bool,
    /// True when the surface preserves confidence.
    pub preserves_confidence: bool,
    /// True when the surface preserves the downgrade reason and disclosure.
    pub preserves_downgrade_reason: bool,
    /// True when the surface preserves the retained raw-payload reference.
    pub preserves_raw_payload_ref: bool,
}

impl TaskEventConsumerBinding {
    fn preserves_truth(&self) -> bool {
        !self.binding_ref.trim().is_empty()
            && self.reads_canonical_envelope
            && self.preserves_source_kind
            && self.preserves_priority_rank
            && self.preserves_confidence
            && self.preserves_downgrade_reason
            && self.preserves_raw_payload_ref
    }
}

/// Canonical M5 task-event envelope record used in arbitration and replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventEnvelope {
    /// Unique stable identifier within the task session.
    pub event_id: String,
    /// Correlation id shared across emissions for the same run.
    pub trace_id: String,
    /// Workspace or workset identity.
    pub workspace_id: String,
    /// Build target, task, test suite, or debug-configuration identity.
    pub target_id: String,
    /// Canonical lifecycle kind.
    pub event_kind: BuildTestEventKind,
    /// Source kind.
    pub source_kind: BuildTestEventSourceKind,
    /// Adapter priority rank (must match the source's canonical rank).
    pub priority_rank: u8,
    /// Confidence.
    pub confidence: BuildTestEventConfidence,
    /// Resolved environment/toolchain/runtime context.
    pub execution_context_id: String,
    /// Pointer to the retained raw adapter payload.
    pub raw_payload_ref: String,
    /// Retention class for the raw payload.
    pub raw_payload_retention_class: RawPayloadRetentionClass,
    /// Producer provenance.
    pub provenance: BuildTestEventProvenance,
    /// True when the emission is visibly downgraded on every consumer surface.
    pub downgraded: bool,
    /// Downgrade reason, present iff the emission is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_reason: Option<DowngradeReason>,
}

impl TaskEventEnvelope {
    fn is_bound(&self) -> bool {
        !self.event_id.trim().is_empty()
            && !self.trace_id.trim().is_empty()
            && !self.workspace_id.trim().is_empty()
            && !self.target_id.trim().is_empty()
            && !self.execution_context_id.trim().is_empty()
            && !self.raw_payload_ref.trim().is_empty()
            && !self.provenance.adapter_id.trim().is_empty()
    }
}

/// One arbitration example proving priority order survives co-observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterArbitrationRow {
    /// Stable arbitration id.
    pub arbitration_id: String,
    /// Target the adapters observed.
    pub target_id: String,
    /// Lifecycle kind observed.
    pub event_kind: BuildTestEventKind,
    /// Shared correlation id across the co-observing adapters.
    pub trace_id: String,
    /// The authoritative winning emission.
    pub winning_event: TaskEventEnvelope,
    /// Lower-priority emissions kept visible as downgraded shadows.
    #[serde(default)]
    pub shadow_events: Vec<TaskEventEnvelope>,
}

/// Closed validation finding vocabulary for the policy baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// The priority ladder does not cover every source kind exactly once.
    PriorityLadderIncomplete,
    /// A rung's rank does not match the canonical adapter order.
    PriorityRankMismatch,
    /// A rung's confidence ceiling does not match the canonical ceiling.
    ConfidenceCeilingMismatch,
    /// A rung's authority flag does not match the canonical authority.
    AuthorityMismatch,
    /// A non-authoritative rung does not block masquerade.
    MasqueradeNotBlocked,
    /// The retention matrix does not cover every source/class cell.
    RetentionMatrixIncomplete,
    /// A source kind has no single allowed default retention class.
    RetentionDefaultInvalid,
    /// A retention cell's approval flag is inconsistent with its class.
    RetentionApprovalMismatch,
    /// The downgrade vocabulary drifts from the closed set.
    DowngradeVocabularyDrift,
    /// A required consumer binding is absent.
    ConsumerBindingMissing,
    /// A consumer binding drops envelope truth.
    ConsumerBindingDropsTruth,
    /// An envelope's priority rank does not match its source kind.
    EnvelopePriorityMismatch,
    /// An envelope claims confidence above its source ceiling.
    EnvelopeConfidenceOverclaim,
    /// An envelope's downgrade flag and reason are inconsistent.
    EnvelopeDowngradeInconsistent,
    /// The arbitration winner is not the highest-priority observed source.
    ArbitrationWinnerNotHighestPriority,
    /// An arbitration shadow is not strictly lower priority and downgraded.
    ArbitrationShadowNotDowngraded,
    /// Co-observing arbitration emissions do not share trace/target/kind.
    ArbitrationCorrelationMismatch,
    /// Stored promotion state disagrees with the derived state.
    PromotionStateMismatch,
}

impl PolicyFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::PriorityLadderIncomplete => "priority_ladder_incomplete",
            Self::PriorityRankMismatch => "priority_rank_mismatch",
            Self::ConfidenceCeilingMismatch => "confidence_ceiling_mismatch",
            Self::AuthorityMismatch => "authority_mismatch",
            Self::MasqueradeNotBlocked => "masquerade_not_blocked",
            Self::RetentionMatrixIncomplete => "retention_matrix_incomplete",
            Self::RetentionDefaultInvalid => "retention_default_invalid",
            Self::RetentionApprovalMismatch => "retention_approval_mismatch",
            Self::DowngradeVocabularyDrift => "downgrade_vocabulary_drift",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
            Self::ConsumerBindingDropsTruth => "consumer_binding_drops_truth",
            Self::EnvelopePriorityMismatch => "envelope_priority_mismatch",
            Self::EnvelopeConfidenceOverclaim => "envelope_confidence_overclaim",
            Self::EnvelopeDowngradeInconsistent => "envelope_downgrade_inconsistent",
            Self::ArbitrationWinnerNotHighestPriority => "arbitration_winner_not_highest_priority",
            Self::ArbitrationShadowNotDowngraded => "arbitration_shadow_not_downgraded",
            Self::ArbitrationCorrelationMismatch => "arbitration_correlation_mismatch",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the policy validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyValidationFinding {
    /// Closed finding kind.
    pub finding_kind: PolicyFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl PolicyValidationFinding {
    fn blocker(finding_kind: PolicyFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`TaskEventAdapterPolicyBaseline::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventAdapterPolicyBaselineInput {
    /// Stable baseline id.
    pub baseline_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Frozen adapter-priority ladder.
    #[serde(default)]
    pub priority_ladder: Vec<AdapterPriorityRung>,
    /// Frozen raw-payload-retention matrix.
    #[serde(default)]
    pub retention_matrix: Vec<RawPayloadRetentionCell>,
    /// Closed downgrade vocabulary.
    #[serde(default)]
    pub downgrade_vocabulary: Vec<DowngradeVocabularyEntry>,
    /// Consumer bindings.
    #[serde(default)]
    pub consumer_bindings: Vec<TaskEventConsumerBinding>,
    /// Arbitration examples.
    #[serde(default)]
    pub arbitration_rows: Vec<AdapterArbitrationRow>,
}

/// Frozen task-event adapter-policy baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventAdapterPolicyBaseline {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable baseline id.
    pub baseline_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Adapter-capability boundary schema ref.
    pub adapter_capability_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// The M4 interoperability contract this baseline extends.
    pub seed_contract_ref: String,
    /// Frozen adapter-priority ladder.
    #[serde(default)]
    pub priority_ladder: Vec<AdapterPriorityRung>,
    /// Frozen raw-payload-retention matrix.
    #[serde(default)]
    pub retention_matrix: Vec<RawPayloadRetentionCell>,
    /// Closed downgrade vocabulary.
    #[serde(default)]
    pub downgrade_vocabulary: Vec<DowngradeVocabularyEntry>,
    /// Consumer bindings.
    #[serde(default)]
    pub consumer_bindings: Vec<TaskEventConsumerBinding>,
    /// Arbitration examples.
    #[serde(default)]
    pub arbitration_rows: Vec<AdapterArbitrationRow>,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<PolicyValidationFinding>,
}

impl TaskEventAdapterPolicyBaseline {
    /// Materializes a baseline and records derived validation findings.
    pub fn materialize(input: TaskEventAdapterPolicyBaselineInput) -> Self {
        let mut baseline = Self {
            record_kind: TASK_EVENT_ADAPTER_POLICY_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION,
            baseline_id: input.baseline_id,
            generated_at: input.generated_at,
            envelope_schema_ref: TASK_EVENT_ADAPTER_POLICY_ENVELOPE_SCHEMA_REF.to_owned(),
            adapter_capability_schema_ref: TASK_EVENT_ADAPTER_POLICY_CAPABILITY_SCHEMA_REF
                .to_owned(),
            doc_ref: TASK_EVENT_ADAPTER_POLICY_DOC_REF.to_owned(),
            seed_contract_ref: TASK_EVENT_ADAPTER_POLICY_SEED_CONTRACT_REF.to_owned(),
            priority_ladder: input.priority_ladder,
            retention_matrix: input.retention_matrix,
            downgrade_vocabulary: input.downgrade_vocabulary,
            consumer_bindings: input.consumer_bindings,
            arbitration_rows: input.arbitration_rows,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = baseline.derived_findings(false);
        baseline.promotion_state = promotion_state_for_findings(&findings);
        baseline.validation_findings = findings;
        baseline
    }

    /// Re-validates the baseline against the frozen policy invariants.
    pub fn validate(&self) -> Vec<PolicyValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Builds an export-safe support packet carrying the exact baseline.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> TaskEventAdapterPolicySupportExport {
        TaskEventAdapterPolicySupportExport {
            record_kind: TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            baseline_id_ref: self.baseline_id.clone(),
            baseline: self.clone(),
        }
    }

    /// Returns the source-kind tokens present on the priority ladder.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for rung in &self.priority_ladder {
            set.insert(rung.source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the consumer tokens present in the consumer bindings.
    pub fn consumer_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for binding in &self.consumer_bindings {
            set.insert(binding.consumer);
        }
        set.into_iter().map(TaskEventConsumer::as_str).collect()
    }

    /// Returns the downgrade-reason tokens present in the vocabulary.
    pub fn downgrade_reason_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for entry in &self.downgrade_vocabulary {
            set.insert(entry.reason);
        }
        set.into_iter().map(DowngradeReason::as_str).collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "baseline {} schema_version={} promotion={}",
            self.baseline_id,
            self.schema_version,
            self.promotion_state.as_str()
        ));
        for rung in &self.priority_ladder {
            lines.push(format!(
                "ladder rank={} source={} ceiling={} authoritative={} masquerade_blocked={}",
                rung.priority_rank,
                rung.source_kind.as_str(),
                rung.confidence_ceiling.as_str(),
                rung.authoritative,
                rung.masquerade_blocked
            ));
        }
        for cell in &self.retention_matrix {
            lines.push(format!(
                "retention source={} class={} allowed={} default={} approval_required={}",
                cell.source_kind.as_str(),
                cell.retention_class.as_str(),
                cell.allowed,
                cell.is_default,
                cell.approval_required
            ));
        }
        for entry in &self.downgrade_vocabulary {
            lines.push(format!(
                "downgrade reason={} max_confidence={} forces_visible={}",
                entry.reason.as_str(),
                entry.max_confidence.as_str(),
                entry.forces_visible_downgrade
            ));
        }
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} reads_canonical={} preserves_priority={} preserves_confidence={}",
                binding.consumer.as_str(),
                binding.reads_canonical_envelope,
                binding.preserves_priority_rank,
                binding.preserves_confidence
            ));
        }
        for row in &self.arbitration_rows {
            lines.push(format!(
                "arbitration {} target={} kind={} winner={} shadows={}",
                row.arbitration_id,
                row.target_id,
                row.event_kind.as_str(),
                row.winning_event.source_kind.as_str(),
                row.shadow_events.len()
            ));
        }
        lines
    }

    fn covered_source_kinds(&self) -> BTreeSet<BuildTestEventSourceKind> {
        self.priority_ladder.iter().map(|r| r.source_kind).collect()
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<PolicyValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != TASK_EVENT_ADAPTER_POLICY_RECORD_KIND {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::WrongRecordKind,
                "baseline has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION
        {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::WrongSchemaVersion,
                "baseline has the wrong schema version",
            ));
        }
        if self.baseline_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::MissingIdentity,
                "baseline id and timestamp are required",
            ));
        }
        for (label, value) in [
            ("envelope schema", self.envelope_schema_ref.as_str()),
            (
                "adapter-capability schema",
                self.adapter_capability_schema_ref.as_str(),
            ),
            ("doc", self.doc_ref.as_str()),
            ("seed contract", self.seed_contract_ref.as_str()),
        ] {
            if value.trim().is_empty() {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_priority_ladder(&mut findings);
        self.check_retention_matrix(&mut findings);
        self.check_downgrade_vocabulary(&mut findings);
        self.check_consumer_bindings(&mut findings);
        self.check_arbitration_rows(&mut findings);

        if include_record_fields {
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::PromotionStateMismatch,
                    format!(
                        "stored promotion state {} does not match derived {}",
                        self.promotion_state.as_str(),
                        expected.as_str()
                    ),
                ));
            }
        }

        findings
    }

    fn check_priority_ladder(&self, findings: &mut Vec<PolicyValidationFinding>) {
        let covered = self.covered_source_kinds();
        if covered.len() != self.priority_ladder.len() {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::PriorityLadderIncomplete,
                "priority ladder repeats a source kind",
            ));
        }
        for source_kind in BuildTestEventSourceKind::ALL {
            if !covered.contains(&source_kind) {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::PriorityLadderIncomplete,
                    format!("priority ladder is missing {}", source_kind.as_str()),
                ));
            }
        }
        for rung in &self.priority_ladder {
            if rung.priority_rank != canonical_priority_rank(rung.source_kind) {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::PriorityRankMismatch,
                    format!(
                        "{} carries rank {} instead of the canonical rank",
                        rung.source_kind.as_str(),
                        rung.priority_rank
                    ),
                ));
            }
            if rung.confidence_ceiling != canonical_confidence_ceiling(rung.source_kind) {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::ConfidenceCeilingMismatch,
                    format!(
                        "{} carries the wrong confidence ceiling",
                        rung.source_kind.as_str()
                    ),
                ));
            }
            if rung.authoritative != source_is_authoritative(rung.source_kind) {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::AuthorityMismatch,
                    format!(
                        "{} carries the wrong authority flag",
                        rung.source_kind.as_str()
                    ),
                ));
            }
            if !source_is_authoritative(rung.source_kind) && !rung.masquerade_blocked {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::MasqueradeNotBlocked,
                    format!(
                        "{} is non-authoritative but does not block masquerade",
                        rung.source_kind.as_str()
                    ),
                ));
            }
        }
    }

    fn check_retention_matrix(&self, findings: &mut Vec<PolicyValidationFinding>) {
        for source_kind in BuildTestEventSourceKind::ALL {
            let cells: Vec<&RawPayloadRetentionCell> = self
                .retention_matrix
                .iter()
                .filter(|cell| cell.source_kind == source_kind)
                .collect();
            for retention_class in RETENTION_CLASS_ALL {
                if !cells
                    .iter()
                    .any(|cell| cell.retention_class == retention_class)
                {
                    findings.push(PolicyValidationFinding::blocker(
                        PolicyFindingKind::RetentionMatrixIncomplete,
                        format!(
                            "retention matrix is missing {} for {}",
                            retention_class.as_str(),
                            source_kind.as_str()
                        ),
                    ));
                }
            }
            let default_cells: Vec<&RawPayloadRetentionCell> = cells
                .iter()
                .copied()
                .filter(|cell| cell.is_default)
                .collect();
            if default_cells.len() != 1 {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::RetentionDefaultInvalid,
                    format!(
                        "{} must declare exactly one default retention class",
                        source_kind.as_str()
                    ),
                ));
            }
            if let Some(default_cell) = default_cells.first() {
                if !default_cell.allowed || default_cell.approval_required {
                    findings.push(PolicyValidationFinding::blocker(
                        PolicyFindingKind::RetentionDefaultInvalid,
                        format!(
                            "{} default retention must be allowed without approval",
                            source_kind.as_str()
                        ),
                    ));
                }
            }
        }
        for cell in &self.retention_matrix {
            let approval_expected =
                cell.retention_class == RawPayloadRetentionClass::SupportApprovalRequired;
            if cell.allowed && cell.approval_required != approval_expected {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::RetentionApprovalMismatch,
                    format!(
                        "{}/{} approval flag is inconsistent with its class",
                        cell.source_kind.as_str(),
                        cell.retention_class.as_str()
                    ),
                ));
            }
        }
    }

    fn check_downgrade_vocabulary(&self, findings: &mut Vec<PolicyValidationFinding>) {
        let present: BTreeSet<DowngradeReason> =
            self.downgrade_vocabulary.iter().map(|e| e.reason).collect();
        if present.len() != self.downgrade_vocabulary.len() {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::DowngradeVocabularyDrift,
                "downgrade vocabulary repeats a reason",
            ));
        }
        for reason in DowngradeReason::ALL {
            if !present.contains(&reason) {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::DowngradeVocabularyDrift,
                    format!("downgrade vocabulary is missing {}", reason.as_str()),
                ));
            }
        }
        for entry in &self.downgrade_vocabulary {
            if !entry.forces_visible_downgrade || entry.summary.trim().is_empty() {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::DowngradeVocabularyDrift,
                    format!(
                        "{} must force a visible, summarized downgrade",
                        entry.reason.as_str()
                    ),
                ));
            }
        }
    }

    fn check_consumer_bindings(&self, findings: &mut Vec<PolicyValidationFinding>) {
        let present: BTreeSet<TaskEventConsumer> =
            self.consumer_bindings.iter().map(|b| b.consumer).collect();
        for consumer in TaskEventConsumer::REQUIRED {
            if !present.contains(&consumer) {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::ConsumerBindingMissing,
                    format!("consumer binding is missing for {}", consumer.as_str()),
                ));
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth() {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::ConsumerBindingDropsTruth,
                    format!(
                        "{} binding drops canonical envelope truth",
                        binding.consumer.as_str()
                    ),
                ));
            }
        }
    }

    fn check_arbitration_rows(&self, findings: &mut Vec<PolicyValidationFinding>) {
        for row in &self.arbitration_rows {
            self.check_envelope(&row.winning_event, findings);
            for shadow in &row.shadow_events {
                self.check_envelope(shadow, findings);
            }

            if row.winning_event.downgraded || row.winning_event.downgrade_reason.is_some() {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::ArbitrationWinnerNotHighestPriority,
                    format!("arbitration {} winner is downgraded", row.arbitration_id),
                ));
            }

            let winner_rank = row.winning_event.priority_rank;
            for shadow in &row.shadow_events {
                if shadow.priority_rank <= winner_rank {
                    findings.push(PolicyValidationFinding::blocker(
                        PolicyFindingKind::ArbitrationWinnerNotHighestPriority,
                        format!(
                            "arbitration {} shadow {} is not lower priority than the winner",
                            row.arbitration_id, shadow.event_id
                        ),
                    ));
                }
                if !shadow.downgraded || shadow.downgrade_reason.is_none() {
                    findings.push(PolicyValidationFinding::blocker(
                        PolicyFindingKind::ArbitrationShadowNotDowngraded,
                        format!(
                            "arbitration {} shadow {} is not visibly downgraded",
                            row.arbitration_id, shadow.event_id
                        ),
                    ));
                }
                if shadow.trace_id != row.trace_id
                    || shadow.target_id != row.target_id
                    || shadow.event_kind != row.event_kind
                {
                    findings.push(PolicyValidationFinding::blocker(
                        PolicyFindingKind::ArbitrationCorrelationMismatch,
                        format!(
                            "arbitration {} shadow {} does not share trace/target/kind",
                            row.arbitration_id, shadow.event_id
                        ),
                    ));
                }
            }

            if row.winning_event.trace_id != row.trace_id
                || row.winning_event.target_id != row.target_id
                || row.winning_event.event_kind != row.event_kind
            {
                findings.push(PolicyValidationFinding::blocker(
                    PolicyFindingKind::ArbitrationCorrelationMismatch,
                    format!(
                        "arbitration {} winner does not share trace/target/kind",
                        row.arbitration_id
                    ),
                ));
            }
        }
    }

    fn check_envelope(
        &self,
        envelope: &TaskEventEnvelope,
        findings: &mut Vec<PolicyValidationFinding>,
    ) {
        if !envelope.is_bound() {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::MissingIdentity,
                format!("envelope {} has incomplete identity", envelope.event_id),
            ));
        }
        if envelope.priority_rank != canonical_priority_rank(envelope.source_kind) {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::EnvelopePriorityMismatch,
                format!(
                    "envelope {} carries a priority rank that disagrees with {}",
                    envelope.event_id,
                    envelope.source_kind.as_str()
                ),
            ));
        }
        if confidence_overclaims(envelope.confidence, envelope.source_kind) {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::EnvelopeConfidenceOverclaim,
                format!(
                    "envelope {} overclaims confidence for {}",
                    envelope.event_id,
                    envelope.source_kind.as_str()
                ),
            ));
        }
        if envelope.downgraded != envelope.downgrade_reason.is_some() {
            findings.push(PolicyValidationFinding::blocker(
                PolicyFindingKind::EnvelopeDowngradeInconsistent,
                format!(
                    "envelope {} downgrade flag and reason disagree",
                    envelope.event_id
                ),
            ));
        }
    }
}

/// Support-export wrapper carrying the exact policy baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventAdapterPolicySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Baseline id ref.
    pub baseline_id_ref: String,
    /// Exact baseline exported.
    pub baseline: TaskEventAdapterPolicyBaseline,
}

impl TaskEventAdapterPolicySupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.baseline_id_ref == self.baseline.baseline_id
            && self.baseline.is_stable()
    }
}

/// All retention classes in stable declaration order.
const RETENTION_CLASS_ALL: [RawPayloadRetentionClass; 3] = [
    RawPayloadRetentionClass::MetadataDigestOnly,
    RawPayloadRetentionClass::RedactedReference,
    RawPayloadRetentionClass::SupportApprovalRequired,
];

fn promotion_state_for_findings(
    findings: &[PolicyValidationFinding],
) -> BuildTestInteropPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    {
        BuildTestInteropPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Warning)
    {
        BuildTestInteropPromotionState::NarrowedBelowStable
    } else {
        BuildTestInteropPromotionState::Stable
    }
}

/// Builds the canonical stable policy-baseline input.
pub fn current_stable_task_event_adapter_policy_input() -> TaskEventAdapterPolicyBaselineInput {
    TaskEventAdapterPolicyBaselineInput {
        baseline_id: TASK_EVENT_ADAPTER_POLICY_BASELINE_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        priority_ladder: canonical_priority_ladder(),
        retention_matrix: canonical_retention_matrix(),
        downgrade_vocabulary: canonical_downgrade_vocabulary(),
        consumer_bindings: canonical_consumer_bindings(),
        arbitration_rows: canonical_arbitration_rows(),
    }
}

/// Materializes the canonical stable policy baseline.
pub fn seeded_task_event_adapter_policy_baseline() -> TaskEventAdapterPolicyBaseline {
    TaskEventAdapterPolicyBaseline::materialize(current_stable_task_event_adapter_policy_input())
}

/// Validates a baseline and returns an `Ok(())` / findings result.
pub fn validate_task_event_adapter_policy_baseline(
    baseline: &TaskEventAdapterPolicyBaseline,
) -> Result<(), Vec<PolicyValidationFinding>> {
    let findings = baseline.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn canonical_priority_ladder() -> Vec<AdapterPriorityRung> {
    BuildTestEventSourceKind::ALL
        .into_iter()
        .map(|source_kind| AdapterPriorityRung {
            source_kind,
            priority_rank: canonical_priority_rank(source_kind),
            confidence_ceiling: canonical_confidence_ceiling(source_kind),
            authoritative: source_is_authoritative(source_kind),
            masquerade_blocked: !source_is_authoritative(source_kind),
            note: match source_kind {
                BuildTestEventSourceKind::Native => {
                    "first-party runtime/adapter truth; highest authority".to_owned()
                }
                BuildTestEventSourceKind::Bsp => {
                    "negotiated Build Server Protocol truth".to_owned()
                }
                BuildTestEventSourceKind::BazelBep => {
                    "Bazel Build Event Protocol/Service truth".to_owned()
                }
                BuildTestEventSourceKind::StructuredOutput => {
                    "imported structured output; bounded translation, never authoritative"
                        .to_owned()
                }
                BuildTestEventSourceKind::HeuristicParser => {
                    "heuristic fallback over unstructured output; always downgraded".to_owned()
                }
            },
        })
        .collect()
}

fn canonical_retention_matrix() -> Vec<RawPayloadRetentionCell> {
    let mut cells = Vec::new();
    for source_kind in BuildTestEventSourceKind::ALL {
        let default_class = match source_kind {
            // Bazel BEP carries artifact references, so a redacted reference is
            // its natural default; every other source defaults to metadata only.
            BuildTestEventSourceKind::BazelBep => RawPayloadRetentionClass::RedactedReference,
            _ => RawPayloadRetentionClass::MetadataDigestOnly,
        };
        for retention_class in RETENTION_CLASS_ALL {
            let approval_required =
                retention_class == RawPayloadRetentionClass::SupportApprovalRequired;
            cells.push(RawPayloadRetentionCell {
                source_kind,
                retention_class,
                allowed: true,
                is_default: retention_class == default_class,
                approval_required,
                note: format!(
                    "{} may retain raw payloads under {}",
                    source_kind.as_str(),
                    retention_class.as_str()
                ),
            });
        }
    }
    cells
}

fn canonical_downgrade_vocabulary() -> Vec<DowngradeVocabularyEntry> {
    DowngradeReason::ALL
        .into_iter()
        .map(|reason| DowngradeVocabularyEntry {
            reason,
            forces_visible_downgrade: true,
            max_confidence: match reason {
                DowngradeReason::PartialSupport => BuildTestEventConfidence::Medium,
                DowngradeReason::HeuristicFallback => BuildTestEventConfidence::Low,
                DowngradeReason::ReplayGap => BuildTestEventConfidence::Medium,
                DowngradeReason::UnsupportedAdapterCapability => BuildTestEventConfidence::Low,
            },
            summary: match reason {
                DowngradeReason::PartialSupport => {
                    "adapter understood the source only partially".to_owned()
                }
                DowngradeReason::HeuristicFallback => {
                    "heuristic parser stood in for a structured or native adapter".to_owned()
                }
                DowngradeReason::ReplayGap => {
                    "replay could not reconstruct the full emission".to_owned()
                }
                DowngradeReason::UnsupportedAdapterCapability => {
                    "an expected negotiated capability was unsupported".to_owned()
                }
            },
        })
        .collect()
}

fn canonical_consumer_bindings() -> Vec<TaskEventConsumerBinding> {
    TaskEventConsumer::REQUIRED
        .into_iter()
        .map(|consumer| TaskEventConsumerBinding {
            consumer,
            binding_ref: format!(
                "binding:tooling:m5:task-event-adapter-policy:{}",
                consumer.as_str()
            ),
            reads_canonical_envelope: true,
            preserves_source_kind: true,
            preserves_priority_rank: true,
            preserves_confidence: true,
            preserves_downgrade_reason: true,
            preserves_raw_payload_ref: true,
        })
        .collect()
}

fn canonical_arbitration_rows() -> Vec<AdapterArbitrationRow> {
    vec![
        // Native build truth wins over a co-observing heuristic parser.
        AdapterArbitrationRow {
            arbitration_id: "arbitration:native-over-heuristic".to_owned(),
            target_id: "target:checkout:build".to_owned(),
            event_kind: BuildTestEventKind::DiagnosticEmitted,
            trace_id: "trace:arbitration:native-over-heuristic".to_owned(),
            winning_event: arbitration_event(
                "event:native:diagnostic",
                "trace:arbitration:native-over-heuristic",
                "target:checkout:build",
                BuildTestEventKind::DiagnosticEmitted,
                BuildTestEventSourceKind::Native,
                BuildTestEventConfidence::High,
                "aureline-build",
                "adapter:aureline-build",
                None,
            ),
            shadow_events: vec![arbitration_event(
                "event:heuristic:diagnostic",
                "trace:arbitration:native-over-heuristic",
                "target:checkout:build",
                BuildTestEventKind::DiagnosticEmitted,
                BuildTestEventSourceKind::HeuristicParser,
                BuildTestEventConfidence::Low,
                "stderr",
                "adapter:problem-matcher",
                Some(DowngradeReason::HeuristicFallback),
            )],
        },
        // BSP truth wins over imported structured output for the same target.
        AdapterArbitrationRow {
            arbitration_id: "arbitration:bsp-over-structured".to_owned(),
            target_id: "target:checkout:test".to_owned(),
            event_kind: BuildTestEventKind::TestCaseFinished,
            trace_id: "trace:arbitration:bsp-over-structured".to_owned(),
            winning_event: arbitration_event(
                "event:bsp:test-finished",
                "trace:arbitration:bsp-over-structured",
                "target:checkout:test",
                BuildTestEventKind::TestCaseFinished,
                BuildTestEventSourceKind::Bsp,
                BuildTestEventConfidence::High,
                "bsp",
                "adapter:bsp",
                None,
            ),
            shadow_events: vec![arbitration_event(
                "event:structured:test-finished",
                "trace:arbitration:bsp-over-structured",
                "target:checkout:test",
                BuildTestEventKind::TestCaseFinished,
                BuildTestEventSourceKind::StructuredOutput,
                BuildTestEventConfidence::MediumHigh,
                "junit",
                "adapter:junit-import",
                Some(DowngradeReason::PartialSupport),
            )],
        },
        // Bazel BEP artifact truth wins over a heuristic reader on replay.
        AdapterArbitrationRow {
            arbitration_id: "arbitration:bep-over-heuristic-replay".to_owned(),
            target_id: "target:checkout:artifact".to_owned(),
            event_kind: BuildTestEventKind::ArtifactPublished,
            trace_id: "trace:arbitration:bep-over-heuristic-replay".to_owned(),
            winning_event: arbitration_event(
                "event:bep:artifact",
                "trace:arbitration:bep-over-heuristic-replay",
                "target:checkout:artifact",
                BuildTestEventKind::ArtifactPublished,
                BuildTestEventSourceKind::BazelBep,
                BuildTestEventConfidence::High,
                "bazel",
                "adapter:bazel-bep",
                None,
            ),
            shadow_events: vec![arbitration_event(
                "event:heuristic:artifact-replay",
                "trace:arbitration:bep-over-heuristic-replay",
                "target:checkout:artifact",
                BuildTestEventKind::ArtifactPublished,
                BuildTestEventSourceKind::HeuristicParser,
                BuildTestEventConfidence::Low,
                "replay-scraper",
                "adapter:replay-heuristic",
                Some(DowngradeReason::ReplayGap),
            )],
        },
    ]
}

#[allow(clippy::too_many_arguments)]
fn arbitration_event(
    event_id: &str,
    trace_id: &str,
    target_id: &str,
    event_kind: BuildTestEventKind,
    source_kind: BuildTestEventSourceKind,
    confidence: BuildTestEventConfidence,
    build_tool_name: &str,
    adapter_id: &str,
    downgrade_reason: Option<DowngradeReason>,
) -> TaskEventEnvelope {
    let retention_class = match source_kind {
        BuildTestEventSourceKind::BazelBep => RawPayloadRetentionClass::RedactedReference,
        _ => RawPayloadRetentionClass::MetadataDigestOnly,
    };
    TaskEventEnvelope {
        event_id: event_id.to_owned(),
        trace_id: trace_id.to_owned(),
        workspace_id: "workspace:checkout".to_owned(),
        target_id: target_id.to_owned(),
        event_kind,
        source_kind,
        priority_rank: canonical_priority_rank(source_kind),
        confidence,
        execution_context_id: "exec-context:local:checkout".to_owned(),
        raw_payload_ref: format!("raw:{event_id}"),
        raw_payload_retention_class: retention_class,
        provenance: BuildTestEventProvenance {
            build_tool_name: build_tool_name.to_owned(),
            build_tool_version: Some("1.0.0".to_owned()),
            adapter_id: adapter_id.to_owned(),
            adapter_version: "1.0.0".to_owned(),
            workspace_revision: Some("rev:checkout:abc123".to_owned()),
        },
        downgraded: downgrade_reason.is_some(),
        downgrade_reason,
    }
}
