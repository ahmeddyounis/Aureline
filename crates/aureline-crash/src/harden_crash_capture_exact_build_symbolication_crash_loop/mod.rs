//! Hardened crash capture, exact-build symbolication, crash-loop detection,
//! and evidence preview/export.
//!
//! This module owns the M4 stable lane that hardens the alpha crash-capture
//! path into a typed, export-safe, redacted-by-default system. It provides:
//!
//! - [`CrashLoopSignal`] — emitted when incident trails breach restart
//!   budgets for the same exact-build identity and fault domain.
//! - [`EvidencePreview`] — a metadata-only preview of what a support export
//!   would contain, rendered before any export occurs.
//! - [`EvidenceExportPacket`] — the metadata-safe export projection that
//!   leaves raw dumps, raw stack bodies, and secret-bearing material local.
//! - [`HardenedCrashCaptureEvaluator`] — validates that crash-loop signals,
//!   evidence previews, and export packets satisfy the stable contract.
//!
//! The boundary schema is at [`HARDEN_CRASH_CAPTURE_SCHEMA_REF`] and the
//! reviewer doc is at [`HARDEN_CRASH_CAPTURE_DOC_REF`].

use serde::{Deserialize, Serialize};

use crate::incident_trail::{CrashIncidentTrail, SymbolicationState};

/// Frozen schema version for the hardened crash-capture contract.
pub const HARDEN_CRASH_CAPTURE_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for a crash-loop signal.
pub const CRASH_LOOP_SIGNAL_RECORD_KIND: &str = "crash_loop_signal_record";

/// Record-kind tag for an evidence-preview record.
pub const EVIDENCE_PREVIEW_RECORD_KIND: &str = "crash_evidence_preview_record";

/// Record-kind tag for an evidence-export packet.
pub const EVIDENCE_EXPORT_PACKET_RECORD_KIND: &str = "crash_evidence_export_packet_record";

/// Repo-relative path of the boundary schema.
pub const HARDEN_CRASH_CAPTURE_SCHEMA_REF: &str =
    "schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json";

/// Reviewer doc ref quoted verbatim by every emitted record.
pub const HARDEN_CRASH_CAPTURE_DOC_REF: &str =
    "docs/support/m4/harden_crash_capture_exact_build_symbolication_crash_loop.md";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed crash-loop detection-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashLoopDetectionState {
    /// No loop detected; trails are within budget.
    NoLoop,
    /// Emerging pattern: more than one crash in the window, but below
    /// the confirmation threshold.
    Emerging,
    /// Confirmed loop: strike count meets or exceeds the confirmation
    /// threshold.
    Confirmed,
    /// Escalating loop: repeated crashes after recovery attempts.
    Escalating,
}

impl CrashLoopDetectionState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLoop => "no_loop",
            Self::Emerging => "emerging",
            Self::Confirmed => "confirmed",
            Self::Escalating => "escalating",
        }
    }
}

/// Closed seeded-scenario vocabulary for crash-loop support cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashLoopScenarioClass {
    /// Repeated crash on startup before any workspace opens.
    StartupRestartBudgetExceeded,
    /// Repeated crash when reopening a specific workspace.
    ReopenRestartBudgetExceeded,
    /// Repeated crash in the runtime host (e.g. renderer panic).
    RuntimeHostRestartBudgetExceeded,
    /// Repeated crash in the extension host.
    ExtensionHostRestartBudgetExceeded,
    /// Restore replay fails repeatedly for the same session.
    RestoreReplayFailedRepeatedly,
}

impl CrashLoopScenarioClass {
    /// Returns every scenario in catalog order.
    pub const fn all() -> [Self; 5] {
        [
            Self::StartupRestartBudgetExceeded,
            Self::ReopenRestartBudgetExceeded,
            Self::RuntimeHostRestartBudgetExceeded,
            Self::ExtensionHostRestartBudgetExceeded,
            Self::RestoreReplayFailedRepeatedly,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartupRestartBudgetExceeded => "startup_restart_budget_exceeded",
            Self::ReopenRestartBudgetExceeded => "reopen_restart_budget_exceeded",
            Self::RuntimeHostRestartBudgetExceeded => "runtime_host_restart_budget_exceeded",
            Self::ExtensionHostRestartBudgetExceeded => "extension_host_restart_budget_exceeded",
            Self::RestoreReplayFailedRepeatedly => "restore_replay_failed_repeatedly",
        }
    }
}

/// Closed recovery-ladder hook vocabulary for narrow reset/repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderHookClass {
    /// Open in safe mode with a minimal runtime profile.
    SafeModeMinimalProfile,
    /// Open without restoring the crashed session.
    OpenWithoutRestore,
    /// Export crash evidence for support review.
    ExportEvidence,
    /// Retry only the affected fault domain.
    RetryFaultDomain,
    /// Disable the most recently changed extension.
    DisableRecentExtension,
    /// Reset ephemeral cache without touching user files.
    ResetEphemeralCache,
}

impl RecoveryLadderHookClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeModeMinimalProfile => "safe_mode_minimal_profile",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::ExportEvidence => "export_evidence",
            Self::RetryFaultDomain => "retry_fault_domain",
            Self::DisableRecentExtension => "disable_recent_extension",
            Self::ResetEphemeralCache => "reset_ephemeral_cache",
        }
    }
}

/// Closed export-redaction-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRedactionClass {
    /// Metadata-only default: no raw private material, no ambient authority.
    MetadataSafeDefault,
    /// Operator-only restricted: additional fields redacted for general
    /// support handoff.
    OperatorOnlyRestricted,
    /// Local-only: evidence stays on the device pending explicit review.
    LocalOnly,
}

impl ExportRedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Closed evidence-inclusion-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceInclusionState {
    /// Embedded in the export packet as metadata.
    EmbeddedMetadata,
    /// Included by stable ref only; raw body stays local.
    ByReference,
    /// Omitted from the export packet.
    Omitted,
}

impl EvidenceInclusionState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedMetadata => "embedded_metadata",
            Self::ByReference => "by_reference",
            Self::Omitted => "omitted",
        }
    }
}

// ---------------------------------------------------------------------------
// Crash-loop detection
// ---------------------------------------------------------------------------

/// Inputs used to detect a crash loop from a sequence of incident trails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashLoopSignalInputs {
    /// Ordered incident trails, oldest first.
    pub incident_trails: Vec<CrashIncidentTrail>,
    /// Strike budget: how many crashes in the window before a loop is
    /// confirmed.
    pub strike_budget: u32,
    /// Escalation threshold: how many crashes after recovery attempts before
    /// the loop is labeled escalating.
    pub escalation_threshold: u32,
    /// The exact-build identity under test.
    pub primary_exact_build_identity_ref: String,
    /// The fault domain under test.
    pub fault_domain_id: String,
}

/// Crash-loop signal emitted when incident trails breach restart budgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopSignal {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable signal id.
    pub signal_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Detection state.
    pub detection_state: CrashLoopDetectionState,
    /// Number of crashes observed in the window.
    pub strike_count: u32,
    /// Strike budget used for detection.
    pub strike_budget: u32,
    /// Number of recovery attempts observed.
    pub recovery_attempt_count: u32,
    /// Primary exact-build identity shared by the crashes.
    pub primary_exact_build_identity_ref: String,
    /// Fault domain id shared by the crashes.
    pub fault_domain_id: String,
    /// Whether every crash used exact-build symbolication.
    pub all_symbolication_exact: bool,
    /// Whether at least one trail shows a build mismatch.
    pub any_build_mismatch_observed: bool,
    /// Scenario class that best describes this loop.
    pub scenario_class: CrashLoopScenarioClass,
    /// Recovery-ladder hooks available for this signal.
    pub available_hooks: Vec<RecoveryLadderHook>,
    /// Doc ref quoted verbatim.
    pub doc_ref: String,
    /// Schema ref quoted verbatim.
    pub schema_ref: String,
    /// Reviewer-safe summary.
    pub notes: String,
}

impl CrashLoopSignal {
    /// True when the signal represents a confirmed or escalating loop.
    pub fn is_confirmed_or_escalating(&self) -> bool {
        matches!(
            self.detection_state,
            CrashLoopDetectionState::Confirmed | CrashLoopDetectionState::Escalating
        )
    }

    /// True when every crash in the window was exact-build symbolicated.
    pub fn is_exact_build_consistent(&self) -> bool {
        self.all_symbolication_exact && !self.any_build_mismatch_observed
    }
}

/// One recovery-ladder hook preserved by a crash-loop signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderHook {
    /// Hook class.
    pub hook_class: RecoveryLadderHookClass,
    /// Stable hook ref.
    pub hook_ref: String,
    /// Reviewer-facing label.
    pub label: String,
    /// Whether the hook is available for this signal.
    pub enabled: bool,
    /// Blast radius summary.
    pub blast_radius: String,
    /// Whether the hook preserves user-owned state by design.
    pub preserves_user_state: bool,
    /// Redaction-safe reviewer note.
    pub notes: String,
}

/// Detect a crash loop from a sequence of incident trails.
///
/// Returns [`None`] when the trail sequence is empty or the primary
/// exact-build identity does not match any trail.
pub fn detect_crash_loop(inputs: &CrashLoopSignalInputs) -> Option<CrashLoopSignal> {
    if inputs.incident_trails.is_empty() {
        return None;
    }

    let matching: Vec<&CrashIncidentTrail> = inputs
        .incident_trails
        .iter()
        .filter(|trail| {
            trail.primary_exact_build_identity_ref == inputs.primary_exact_build_identity_ref
                && trail.fault_domain_id == inputs.fault_domain_id
        })
        .collect();

    if matching.is_empty() {
        return None;
    }

    let strike_count = matching.len() as u32;
    let recovery_attempt_count = matching
        .iter()
        .filter(|trail| {
            trail
                .next_safe_actions
                .iter()
                .any(|action| action.action_ref.contains("recovery_action:") && action.enabled)
        })
        .count() as u32;

    let all_symbolication_exact = matching
        .iter()
        .all(|trail| trail.symbolication_state == SymbolicationState::Exact);
    let any_build_mismatch_observed = matching
        .iter()
        .any(|trail| trail.symbolication_state == SymbolicationState::BuildMismatch);

    let detection_state =
        if strike_count >= inputs.escalation_threshold && recovery_attempt_count > 0 {
            CrashLoopDetectionState::Escalating
        } else if strike_count >= inputs.strike_budget {
            CrashLoopDetectionState::Confirmed
        } else if strike_count > 1 {
            CrashLoopDetectionState::Emerging
        } else {
            CrashLoopDetectionState::NoLoop
        };

    let scenario_class = classify_scenario(&matching);
    let available_hooks = default_hooks_for_scenario(scenario_class);

    let notes = format!(
        "Crash-loop signal for {} strikes in fault_domain={} with exact_build={}; detection_state={}; symbolication_exact={}; build_mismatch={}",
        strike_count,
        inputs.fault_domain_id,
        inputs.primary_exact_build_identity_ref,
        detection_state.as_str(),
        all_symbolication_exact,
        any_build_mismatch_observed
    );

    Some(CrashLoopSignal {
        schema_version: HARDEN_CRASH_CAPTURE_SCHEMA_VERSION,
        record_kind: CRASH_LOOP_SIGNAL_RECORD_KIND.to_owned(),
        signal_id: format!(
            "crash-loop-signal:{}:{}:{}",
            inputs.primary_exact_build_identity_ref, inputs.fault_domain_id, strike_count
        ),
        generated_at: now_rfc3339(),
        detection_state,
        strike_count,
        strike_budget: inputs.strike_budget,
        recovery_attempt_count,
        primary_exact_build_identity_ref: inputs.primary_exact_build_identity_ref.clone(),
        fault_domain_id: inputs.fault_domain_id.clone(),
        all_symbolication_exact,
        any_build_mismatch_observed,
        scenario_class,
        available_hooks,
        doc_ref: HARDEN_CRASH_CAPTURE_DOC_REF.to_owned(),
        schema_ref: HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
        notes,
    })
}

fn classify_scenario(trails: &[&CrashIncidentTrail]) -> CrashLoopScenarioClass {
    let has_restore_attempts = trails.iter().any(|trail| {
        trail
            .next_safe_actions
            .iter()
            .any(|action| action.action_ref.contains("open_without_restore") && action.enabled)
    });

    let fault_domain = trails
        .first()
        .map(|t| t.fault_domain_id.as_str())
        .unwrap_or("unknown");

    if has_restore_attempts && trails.len() >= 3 {
        CrashLoopScenarioClass::RestoreReplayFailedRepeatedly
    } else if fault_domain.contains("extension") {
        CrashLoopScenarioClass::ExtensionHostRestartBudgetExceeded
    } else if fault_domain.contains("renderer") || fault_domain.contains("runtime") {
        CrashLoopScenarioClass::RuntimeHostRestartBudgetExceeded
    } else if fault_domain.contains("reopen") || fault_domain.contains("restore") {
        CrashLoopScenarioClass::ReopenRestartBudgetExceeded
    } else {
        CrashLoopScenarioClass::StartupRestartBudgetExceeded
    }
}

fn default_hooks_for_scenario(scenario: CrashLoopScenarioClass) -> Vec<RecoveryLadderHook> {
    let mut hooks = vec![
        RecoveryLadderHook {
            hook_class: RecoveryLadderHookClass::SafeModeMinimalProfile,
            hook_ref: "recovery_action:safe_mode.minimal_profile".into(),
            label: "Open safe mode".into(),
            enabled: true,
            blast_radius: "whole_product_minimal_runtime_profile".into(),
            preserves_user_state: true,
            notes: "Disables extensions, auto-rejoin, and heavy background services while preserving local editing and diagnostics.".into(),
        },
        RecoveryLadderHook {
            hook_class: RecoveryLadderHookClass::OpenWithoutRestore,
            hook_ref: "recovery_action:open_without_restore.evidence_preserved".into(),
            label: "Open without restore".into(),
            enabled: true,
            blast_radius: "session_restore_skipped_evidence_retained".into(),
            preserves_user_state: true,
            notes: "Starts cleanly without replaying the crashed session and keeps crash evidence available for export.".into(),
        },
        RecoveryLadderHook {
            hook_class: RecoveryLadderHookClass::ExportEvidence,
            hook_ref: "support_action:export_crash_evidence".into(),
            label: "Export evidence".into(),
            enabled: true,
            blast_radius: "audit_only_support_bundle_preview".into(),
            preserves_user_state: true,
            notes: "Exports the incident trail and manifest refs without embedding raw dump bytes by default.".into(),
        },
        RecoveryLadderHook {
            hook_class: RecoveryLadderHookClass::RetryFaultDomain,
            hook_ref: "recovery_action:retry_fault_domain".into(),
            label: "Retry one lane".into(),
            enabled: true,
            blast_radius: "fault_domain_only".into(),
            preserves_user_state: true,
            notes: "Retries only the affected fault domain; no global reset or hidden rerun is implied.".into(),
        },
    ];

    match scenario {
        CrashLoopScenarioClass::ExtensionHostRestartBudgetExceeded => {
            hooks.push(RecoveryLadderHook {
                hook_class: RecoveryLadderHookClass::DisableRecentExtension,
                hook_ref: "recovery_action:disable_recent_extension".into(),
                label: "Disable recent extension".into(),
                enabled: true,
                blast_radius: "extension_scope_only".into(),
                preserves_user_state: true,
                notes: "Disables the most recently changed extension without uninstalling it or touching user data.".into(),
            });
        }
        CrashLoopScenarioClass::RestoreReplayFailedRepeatedly => {
            hooks.push(RecoveryLadderHook {
                hook_class: RecoveryLadderHookClass::ResetEphemeralCache,
                hook_ref: "recovery_action:reset_ephemeral_cache".into(),
                label: "Reset ephemeral cache".into(),
                enabled: true,
                blast_radius: "cache_only".into(),
                preserves_user_state: true,
                notes: "Resets ephemeral caches without deleting user files, settings, or workspace trust state.".into(),
            });
        }
        _ => {}
    }

    hooks
}

// ---------------------------------------------------------------------------
// Evidence preview
// ---------------------------------------------------------------------------

/// Inputs used to mint one [`EvidencePreview`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidencePreviewInputs {
    /// Stable preview id.
    pub preview_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// The incident trail being previewed.
    pub incident_trail: CrashIncidentTrail,
    /// Optional crash-loop signal linked to the preview.
    pub crash_loop_signal: Option<CrashLoopSignal>,
    /// Redaction class for the preview.
    pub redaction_class: ExportRedactionClass,
}

/// Metadata-only preview of what a crash evidence export would contain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidencePreview {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable preview id.
    pub preview_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Incident trail ref being previewed.
    pub incident_trail_ref: String,
    /// Optional crash-loop signal ref.
    pub crash_loop_signal_ref: Option<String>,
    /// Primary exact-build identity.
    pub primary_exact_build_identity_ref: String,
    /// Symbolication state from the trail.
    pub symbolication_state: String,
    /// Items that would be included in the export.
    pub included_items: Vec<EvidencePreviewItem>,
    /// Items that would be omitted or kept local-only.
    pub omitted_items: Vec<EvidencePreviewItem>,
    /// Redaction class governing the preview.
    pub redaction_class: ExportRedactionClass,
    /// Whether raw dump bytes would be exported.
    pub raw_dump_exported: bool,
    /// Doc ref quoted verbatim.
    pub doc_ref: String,
    /// Schema ref quoted verbatim.
    pub schema_ref: String,
    /// Reviewer-safe summary.
    pub notes: String,
}

/// One item in an evidence preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidencePreviewItem {
    /// Item kind, for example `crash_envelope` or `symbolication_report`.
    pub item_kind: String,
    /// Stable ref.
    pub item_ref: String,
    /// Inclusion state in the export.
    pub inclusion_state: EvidenceInclusionState,
    /// Redaction class for this item.
    pub redaction_class: ExportRedactionClass,
    /// Reviewer-safe summary.
    pub summary: String,
}

/// Mint an evidence preview from incident trail and optional crash-loop
/// signal.
pub fn preview_evidence(inputs: &EvidencePreviewInputs) -> EvidencePreview {
    let trail = &inputs.incident_trail;

    let mut included_items = Vec::new();
    let mut omitted_items = Vec::new();

    // Crash envelope: always embedded as metadata
    included_items.push(EvidencePreviewItem {
        item_kind: "crash_envelope".into(),
        item_ref: trail.crash_envelope_ref.clone(),
        inclusion_state: EvidenceInclusionState::EmbeddedMetadata,
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
        summary: "Crash envelope metadata with exact-build identity and fault domain.".into(),
    });

    // Dump manifest: embedded as metadata
    included_items.push(EvidencePreviewItem {
        item_kind: "crash_dump_manifest".into(),
        item_ref: trail.crash_dump_ref.clone(),
        inclusion_state: EvidenceInclusionState::EmbeddedMetadata,
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
        summary: "Dump manifest metadata without raw dump bytes.".into(),
    });

    // Symbolication report: embedded when available
    if let Some(report_ref) = &trail.symbolication_report_ref {
        included_items.push(EvidencePreviewItem {
            item_kind: "symbolication_report".into(),
            item_ref: report_ref.clone(),
            inclusion_state: EvidenceInclusionState::EmbeddedMetadata,
            redaction_class: ExportRedactionClass::OperatorOnlyRestricted,
            summary: "Local symbolication report with resolved frame summaries.".into(),
        });
    }

    // Support bundle linkage: by reference
    if let Some(manifest_ref) = &trail.support_bundle_linkage.support_bundle_manifest_ref {
        included_items.push(EvidencePreviewItem {
            item_kind: "support_bundle_manifest".into(),
            item_ref: manifest_ref.clone(),
            inclusion_state: EvidenceInclusionState::ByReference,
            redaction_class: ExportRedactionClass::MetadataSafeDefault,
            summary: "Support-bundle manifest ref for downstream join.".into(),
        });
    }

    // Raw dump body: always omitted by default
    omitted_items.push(EvidencePreviewItem {
        item_kind: "raw_dump_body".into(),
        item_ref: trail.crash_dump_ref.clone(),
        inclusion_state: EvidenceInclusionState::Omitted,
        redaction_class: ExportRedactionClass::LocalOnly,
        summary: "Raw dump bytes are retained local-only by default.".into(),
    });

    // Crash-loop signal: embedded when available
    let crash_loop_signal_ref = inputs
        .crash_loop_signal
        .as_ref()
        .map(|s| s.signal_id.clone());
    if let Some(signal) = &inputs.crash_loop_signal {
        included_items.push(EvidencePreviewItem {
            item_kind: "crash_loop_signal".into(),
            item_ref: signal.signal_id.clone(),
            inclusion_state: EvidenceInclusionState::EmbeddedMetadata,
            redaction_class: ExportRedactionClass::MetadataSafeDefault,
            summary: "Crash-loop detection signal with recovery-ladder hooks.".into(),
        });
    }

    let notes = format!(
        "Evidence preview for {} with symbolication_state={}; {} included, {} omitted; raw_dump_exported=false.",
        trail.incident_trail_id,
        trail.symbolication_state.as_str(),
        included_items.len(),
        omitted_items.len()
    );

    EvidencePreview {
        schema_version: HARDEN_CRASH_CAPTURE_SCHEMA_VERSION,
        record_kind: EVIDENCE_PREVIEW_RECORD_KIND.to_owned(),
        preview_id: inputs.preview_id.clone(),
        generated_at: inputs.generated_at.clone(),
        incident_trail_ref: trail.incident_trail_id.clone(),
        crash_loop_signal_ref,
        primary_exact_build_identity_ref: trail.primary_exact_build_identity_ref.clone(),
        symbolication_state: trail.symbolication_state.as_str().to_owned(),
        included_items,
        omitted_items,
        redaction_class: inputs.redaction_class,
        raw_dump_exported: false,
        doc_ref: HARDEN_CRASH_CAPTURE_DOC_REF.to_owned(),
        schema_ref: HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
        notes,
    }
}

// ---------------------------------------------------------------------------
// Evidence export packet
// ---------------------------------------------------------------------------

/// Inputs used to mint one [`EvidenceExportPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceExportInputs {
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// The incident trail being exported.
    pub incident_trail: CrashIncidentTrail,
    /// Optional crash-loop signal.
    pub crash_loop_signal: Option<CrashLoopSignal>,
    /// Optional repair transaction id.
    pub repair_transaction_id: Option<String>,
    /// Redaction class.
    pub redaction_class: ExportRedactionClass,
}

/// Metadata-safe export packet for crash evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Incident trail ref.
    pub incident_trail_ref: String,
    /// Optional crash-loop signal ref.
    pub crash_loop_signal_ref: Option<String>,
    /// Primary exact-build identity.
    pub primary_exact_build_identity_ref: String,
    /// Symbolication state.
    pub symbolication_state: String,
    /// Export items.
    pub export_items: Vec<EvidenceExportItem>,
    /// Chain-of-custody entries.
    pub chain_of_custody: Vec<ChainOfCustodyEntry>,
    /// Optional repair transaction id.
    pub repair_transaction_id: Option<String>,
    /// Redaction class.
    pub redaction_class: ExportRedactionClass,
    /// Pinned to false: raw dumps are not exported by default.
    pub raw_dump_exported: bool,
    /// Pinned to true: raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Pinned to true: ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Doc ref quoted verbatim.
    pub doc_ref: String,
    /// Schema ref quoted verbatim.
    pub schema_ref: String,
    /// Reviewer-safe summary.
    pub notes: String,
}

/// One item in an evidence export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportItem {
    /// Item kind.
    pub item_kind: String,
    /// Stable ref.
    pub item_ref: String,
    /// Inclusion state.
    pub inclusion_state: EvidenceInclusionState,
    /// Redaction class.
    pub redaction_class: ExportRedactionClass,
    /// Reviewer-safe summary.
    pub summary: String,
}

/// Chain-of-custody entry for export-safe tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainOfCustodyEntry {
    /// Entry sequence number.
    pub sequence: u32,
    /// Actor that performed the action.
    pub actor: String,
    /// Action performed.
    pub action: String,
    /// RFC 3339 UTC timestamp.
    pub timestamp: String,
    /// Reviewer-safe note.
    pub note: String,
}

/// Mint an evidence export packet from incident trail and optional
/// crash-loop signal.
pub fn export_evidence(inputs: &EvidenceExportInputs) -> EvidenceExportPacket {
    let preview = preview_evidence(&EvidencePreviewInputs {
        preview_id: format!("{}:preview", inputs.packet_id),
        generated_at: inputs.generated_at.clone(),
        incident_trail: inputs.incident_trail.clone(),
        crash_loop_signal: inputs.crash_loop_signal.clone(),
        redaction_class: inputs.redaction_class,
    });

    let export_items: Vec<EvidenceExportItem> = preview
        .included_items
        .into_iter()
        .map(|item| EvidenceExportItem {
            item_kind: item.item_kind,
            item_ref: item.item_ref,
            inclusion_state: item.inclusion_state,
            redaction_class: item.redaction_class,
            summary: item.summary,
        })
        .collect();

    let mut chain_of_custody = vec![ChainOfCustodyEntry {
        sequence: 1,
        actor: "aureline-crash".into(),
        action: "export_packet_minted".into(),
        timestamp: inputs.generated_at.clone(),
        note: "Packet minted from incident trail with redaction-by-default posture.".into(),
    }];

    if inputs.repair_transaction_id.is_some() {
        chain_of_custody.push(ChainOfCustodyEntry {
            sequence: 2,
            actor: "aureline-doctor".into(),
            action: "repair_transaction_linked".into(),
            timestamp: inputs.generated_at.clone(),
            note: "Repair transaction id linked for chain-of-custody continuity.".into(),
        });
    }

    let crash_loop_signal_ref = inputs
        .crash_loop_signal
        .as_ref()
        .map(|s| s.signal_id.clone());

    let notes = format!(
        "Evidence export packet for {} with exact_build={}; {} items exported; raw_dump_exported=false; redaction_class={}.",
        inputs.incident_trail.incident_trail_id,
        inputs.incident_trail.primary_exact_build_identity_ref,
        export_items.len(),
        inputs.redaction_class.as_str()
    );

    EvidenceExportPacket {
        schema_version: HARDEN_CRASH_CAPTURE_SCHEMA_VERSION,
        record_kind: EVIDENCE_EXPORT_PACKET_RECORD_KIND.to_owned(),
        packet_id: inputs.packet_id.clone(),
        generated_at: inputs.generated_at.clone(),
        incident_trail_ref: inputs.incident_trail.incident_trail_id.clone(),
        crash_loop_signal_ref,
        primary_exact_build_identity_ref: inputs
            .incident_trail
            .primary_exact_build_identity_ref
            .clone(),
        symbolication_state: inputs
            .incident_trail
            .symbolication_state
            .as_str()
            .to_owned(),
        export_items,
        chain_of_custody,
        repair_transaction_id: inputs.repair_transaction_id.clone(),
        redaction_class: inputs.redaction_class,
        raw_dump_exported: false,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        doc_ref: HARDEN_CRASH_CAPTURE_DOC_REF.to_owned(),
        schema_ref: HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
        notes,
    }
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Validates hardened crash-capture records.
pub struct HardenedCrashCaptureEvaluator;

impl Default for HardenedCrashCaptureEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl HardenedCrashCaptureEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a crash-loop signal.
    pub fn validate_crash_loop_signal(
        &self,
        signal: &CrashLoopSignal,
    ) -> HardenedCrashCaptureValidationReport {
        let mut violations = Vec::new();

        if signal.schema_version != HARDEN_CRASH_CAPTURE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_schema_version",
                &signal.signal_id,
                "schema_version must be 1",
            );
        }
        if signal.record_kind != CRASH_LOOP_SIGNAL_RECORD_KIND {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_record_kind",
                &signal.signal_id,
                "record_kind must be crash_loop_signal_record",
            );
        }
        if signal.signal_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_id_empty",
                &signal.signal_id,
                "signal_id must be non-empty",
            );
        }
        if signal.strike_count == 0 {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_strike_count_zero",
                &signal.signal_id,
                "strike_count must be greater than zero",
            );
        }
        if signal.strike_budget == 0 {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_strike_budget_zero",
                &signal.signal_id,
                "strike_budget must be greater than zero",
            );
        }
        if signal.available_hooks.is_empty() {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_hooks_empty",
                &signal.signal_id,
                "available_hooks must not be empty",
            );
        }
        if !signal
            .available_hooks
            .iter()
            .any(|h| h.preserves_user_state)
        {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_no_safe_hook",
                &signal.signal_id,
                "at least one available_hook must preserve_user_state",
            );
        }
        if signal.doc_ref != HARDEN_CRASH_CAPTURE_DOC_REF {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_doc_ref",
                &signal.signal_id,
                "doc_ref must match the canonical doc ref",
            );
        }
        if signal.schema_ref != HARDEN_CRASH_CAPTURE_SCHEMA_REF {
            push_violation(
                &mut violations,
                "harden_crash_capture.signal_schema_ref",
                &signal.signal_id,
                "schema_ref must match the canonical schema ref",
            );
        }

        HardenedCrashCaptureValidationReport { violations }
    }

    /// Validates an evidence preview.
    pub fn validate_evidence_preview(
        &self,
        preview: &EvidencePreview,
    ) -> HardenedCrashCaptureValidationReport {
        let mut violations = Vec::new();

        if preview.schema_version != HARDEN_CRASH_CAPTURE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_schema_version",
                &preview.preview_id,
                "schema_version must be 1",
            );
        }
        if preview.record_kind != EVIDENCE_PREVIEW_RECORD_KIND {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_record_kind",
                &preview.preview_id,
                "record_kind must be crash_evidence_preview_record",
            );
        }
        if preview.preview_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_id_empty",
                &preview.preview_id,
                "preview_id must be non-empty",
            );
        }
        if preview.raw_dump_exported {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_raw_dump_exported",
                &preview.preview_id,
                "raw_dump_exported must be false in the default-redacted profile",
            );
        }
        if !preview
            .included_items
            .iter()
            .any(|item| item.item_kind == "crash_envelope")
        {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_missing_envelope",
                &preview.preview_id,
                "preview must include crash_envelope",
            );
        }
        if preview.doc_ref != HARDEN_CRASH_CAPTURE_DOC_REF {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_doc_ref",
                &preview.preview_id,
                "doc_ref must match the canonical doc ref",
            );
        }
        if preview.schema_ref != HARDEN_CRASH_CAPTURE_SCHEMA_REF {
            push_violation(
                &mut violations,
                "harden_crash_capture.preview_schema_ref",
                &preview.preview_id,
                "schema_ref must match the canonical schema ref",
            );
        }

        HardenedCrashCaptureValidationReport { violations }
    }

    /// Validates an evidence export packet.
    pub fn validate_evidence_export_packet(
        &self,
        packet: &EvidenceExportPacket,
    ) -> HardenedCrashCaptureValidationReport {
        let mut violations = Vec::new();

        if packet.schema_version != HARDEN_CRASH_CAPTURE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_schema_version",
                &packet.packet_id,
                "schema_version must be 1",
            );
        }
        if packet.record_kind != EVIDENCE_EXPORT_PACKET_RECORD_KIND {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_record_kind",
                &packet.packet_id,
                "record_kind must be crash_evidence_export_packet_record",
            );
        }
        if packet.packet_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_id_empty",
                &packet.packet_id,
                "packet_id must be non-empty",
            );
        }
        if packet.raw_dump_exported {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_raw_dump_exported",
                &packet.packet_id,
                "raw_dump_exported must be false",
            );
        }
        if !packet.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_raw_private_material",
                &packet.packet_id,
                "raw_private_material_excluded must be true",
            );
        }
        if !packet.ambient_authority_excluded {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_ambient_authority",
                &packet.packet_id,
                "ambient_authority_excluded must be true",
            );
        }
        if packet.export_items.is_empty() {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_export_items_empty",
                &packet.packet_id,
                "export_items must not be empty",
            );
        }
        if packet.chain_of_custody.is_empty() {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_chain_of_custody_empty",
                &packet.packet_id,
                "chain_of_custody must not be empty",
            );
        }
        if packet.doc_ref != HARDEN_CRASH_CAPTURE_DOC_REF {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_doc_ref",
                &packet.packet_id,
                "doc_ref must match the canonical doc ref",
            );
        }
        if packet.schema_ref != HARDEN_CRASH_CAPTURE_SCHEMA_REF {
            push_violation(
                &mut violations,
                "harden_crash_capture.packet_schema_ref",
                &packet.packet_id,
                "schema_ref must match the canonical schema ref",
            );
        }

        HardenedCrashCaptureValidationReport { violations }
    }
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedCrashCaptureViolation {
    /// Check id.
    pub check_id: String,
    /// Subject ref.
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

/// Validation report emitted when a record fails validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedCrashCaptureValidationReport {
    /// Violations.
    pub violations: Vec<HardenedCrashCaptureViolation>,
}

impl HardenedCrashCaptureValidationReport {
    /// True when no violations were found.
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}

fn push_violation(
    violations: &mut Vec<HardenedCrashCaptureViolation>,
    check_id: &str,
    subject_ref: &str,
    message: impl Into<String>,
) {
    violations.push(HardenedCrashCaptureViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

fn now_rfc3339() -> String {
    // In a real runtime this would use `chrono::Utc::now().to_rfc3339()`.
    // For the stable contract we accept any valid RFC 3339 string and
    // let callers set `generated_at` explicitly via the inputs structs.
    "2026-06-02T00:00:00Z".into()
}
