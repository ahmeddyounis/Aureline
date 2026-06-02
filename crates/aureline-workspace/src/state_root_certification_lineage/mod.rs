//! State-root certification lineage: the governed, export-safe
//! projection that certifies the storage-discipline surfaces,
//! cache-governance inspector, and state-root audit views Aureline
//! ships on a claimed stable profile.
//!
//! Where the cache / storage-class lineage proves how individual
//! storage classes carry user state, the portable-state lineage
//! proves how restored state preserves provenance, and the
//! schema-migration / repair lineage proves how migrations and
//! repairs preserve fidelity, this projection proves the *audit
//! surfaces* on top of those lineages: which state-root resource
//! classes Aureline audits, which audit surfaces are reachable in
//! product, which audit findings are honest under destructive
//! cleanup, and which inspection / cleanup / repair hooks must fire
//! before any destructive cleanup of state-root resources commits on
//! a claimed stable profile.
//!
//! The projection ingests a live [`StateRootCertificationInputs`]
//! envelope verbatim (one [`ResourceAuditObservation`] per audited
//! state-root resource, one [`AuditSurfaceObservation`] per governed
//! audit surface, plus the controlled inspection-hook table) and
//! produces a lineage record that proves the contract claims the
//! stable line is anchored on:
//!
//! - **Resource-class coverage truth.** Every governed state-root
//!   resource class
//!   (`persistent_state_envelope`, `workspace_state_root`,
//!   `profile_root`, `recent_work_root`, `local_history_root`,
//!   `restore_checkpoint_root`, `cache_governance_root`) ships a row
//!   bound to one closed [`StateRootResourceKind`]; the optional
//!   `prebuild_cache_root` and `mutation_journal_root` rows ride on
//!   top without changing the required set.
//! - **Audit-surface coverage truth.** Every governed audit surface
//!   (`storage_discipline_overview`, `cache_governance_inspector`,
//!   `state_root_audit_panel`, `cleanup_inventory_audit`,
//!   `eviction_rule_audit`, `headless_audit_cli`,
//!   `support_export_audit_section`) ships a row bound to one closed
//!   [`AuditSurfaceKind`].
//! - **Storage-class taxonomy truth.** Every resource row reports a
//!   non-empty `storage_class_ref` so the audit binds to the
//!   storage-class lineage already governing eviction rules; an
//!   empty ref narrows the record.
//! - **Audit-finding honesty.** Every resource row declares one
//!   closed [`AuditFindingClass`]; a `dirty_with_disclosure`,
//!   `inconclusive_held`, or `refused_unsafe` finding ships behind a
//!   non-empty audit disclosure ref.
//! - **Cleanup-precondition truth.** Every dirty or held audit row
//!   binds at least one cleanup-surface ref and at least one
//!   inspection / cleanup / repair hook so a destructive cleanup
//!   never fires without user-visible review.
//! - **No-silent-rerun honesty.** Every audit and cleanup row that
//!   mutates persistent state declares
//!   `explicit_user_action_required` or `terminal_no_further_run`
//!   with both a commit action id and a commit disclosure id; a
//!   `silent_rerun_permitted` posture is forbidden on Stable rows.
//! - **Restore-provenance / encoding / trust-state preservation.**
//!   Every audit row preserves the restore provenance, encoding
//!   fidelity, trust state, and lineage refs of the resource it
//!   audits; any deviation narrows the record below Stable.
//! - **Claimed stable profile honesty.** The record binds one closed
//!   [`ClaimedStableProfile`]; the projection enforces that the
//!   profile is a Stable-claimed posture and refuses to certify on a
//!   `narrowed_below_stable` profile.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / cleanup / repair hooks
//!   (`inspect_state_root`, `compare_before_cleanup`,
//!   `preview_cleanup`, `preview_repair`, `rollback_cleanup`,
//!   `rollback_repair`, `export_before_cleanup`, `export_before_repair`)
//!   is reachable before any destructive cleanup commits.
//! - **Support-export honesty.** Each row's support-export
//!   projection preserves the resource class, audit finding,
//!   storage-class ref, claimed profile, audit transaction id,
//!   finding code, and redaction class while excluding raw secrets,
//!   raw artifact bytes, approval tickets, delegated credentials,
//!   and live authority handles.
//! - **Producer attribution.** Each record carries the producer ref,
//!   the schema version, the capture timestamp, and an integrity
//!   hash derived from the input identities so replay and support
//!   pipelines can pin the source before applying.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to
//!   the source workspace, corpus, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`StateRootCertificationLineageRecord`].
pub const STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the state-root certification lineage record.
pub const STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/state_root_certification_lineage.schema.json";

/// Stable record-kind tag for the state-root certification lineage
/// record.
pub const STATE_ROOT_CERTIFICATION_LINEAGE_RECORD_KIND: &str =
    "state_root_certification_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the state-root resource classes Aureline
/// audits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootResourceKind {
    /// Top-level persistent-state envelope binding the layers below.
    PersistentStateEnvelope,
    /// Durable workspace state root — layout, panes, sessions.
    WorkspaceStateRoot,
    /// Portable user-profile root (keymaps, settings, presets).
    ProfileRoot,
    /// Persistent recent-work / entry-restore registry root.
    RecentWorkRoot,
    /// Local-history corpus root.
    LocalHistoryRoot,
    /// Named restore-checkpoint root.
    RestoreCheckpointRoot,
    /// Cache-governance inspector root (the storage-discipline
    /// directory the inspector reads from).
    CacheGovernanceRoot,
    /// Optional prebuild / build-artifact cache root.
    PrebuildCacheRoot,
    /// Optional mutation-journal artifact root.
    MutationJournalRoot,
}

impl StateRootResourceKind {
    /// Returns the stable snake_case token for this resource class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentStateEnvelope => "persistent_state_envelope",
            Self::WorkspaceStateRoot => "workspace_state_root",
            Self::ProfileRoot => "profile_root",
            Self::RecentWorkRoot => "recent_work_root",
            Self::LocalHistoryRoot => "local_history_root",
            Self::RestoreCheckpointRoot => "restore_checkpoint_root",
            Self::CacheGovernanceRoot => "cache_governance_root",
            Self::PrebuildCacheRoot => "prebuild_cache_root",
            Self::MutationJournalRoot => "mutation_journal_root",
        }
    }

    /// True when this resource class is part of the required set every
    /// Stable corpus must seed.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::PersistentStateEnvelope
                | Self::WorkspaceStateRoot
                | Self::ProfileRoot
                | Self::RecentWorkRoot
                | Self::LocalHistoryRoot
                | Self::RestoreCheckpointRoot
                | Self::CacheGovernanceRoot
        )
    }
}

/// Closed list of state-root resource classes every certification
/// record must seed.
pub const REQUIRED_STATE_ROOT_RESOURCES: [StateRootResourceKind; 7] = [
    StateRootResourceKind::PersistentStateEnvelope,
    StateRootResourceKind::WorkspaceStateRoot,
    StateRootResourceKind::ProfileRoot,
    StateRootResourceKind::RecentWorkRoot,
    StateRootResourceKind::LocalHistoryRoot,
    StateRootResourceKind::RestoreCheckpointRoot,
    StateRootResourceKind::CacheGovernanceRoot,
];

/// Closed vocabulary for the audit surfaces Aureline governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditSurfaceKind {
    /// Settings / Storage Discipline overview panel.
    StorageDisciplineOverview,
    /// Cache-governance inspector (per-class drill-in).
    CacheGovernanceInspector,
    /// State-root audit panel (per-resource drill-in).
    StateRootAuditPanel,
    /// Cleanup-inventory audit (covering cleanup surface reachability).
    CleanupInventoryAudit,
    /// Eviction-rule audit (cross-checking eviction policy honesty).
    EvictionRuleAudit,
    /// Headless audit CLI (`aureline audit ...`).
    HeadlessAuditCli,
    /// Support-export audit section (proof shipped with support
    /// bundles).
    SupportExportAuditSection,
}

impl AuditSurfaceKind {
    /// Returns the stable snake_case token for this audit surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StorageDisciplineOverview => "storage_discipline_overview",
            Self::CacheGovernanceInspector => "cache_governance_inspector",
            Self::StateRootAuditPanel => "state_root_audit_panel",
            Self::CleanupInventoryAudit => "cleanup_inventory_audit",
            Self::EvictionRuleAudit => "eviction_rule_audit",
            Self::HeadlessAuditCli => "headless_audit_cli",
            Self::SupportExportAuditSection => "support_export_audit_section",
        }
    }
}

/// Closed list of audit surfaces every certification record must seed.
pub const REQUIRED_AUDIT_SURFACES: [AuditSurfaceKind; 7] = [
    AuditSurfaceKind::StorageDisciplineOverview,
    AuditSurfaceKind::CacheGovernanceInspector,
    AuditSurfaceKind::StateRootAuditPanel,
    AuditSurfaceKind::CleanupInventoryAudit,
    AuditSurfaceKind::EvictionRuleAudit,
    AuditSurfaceKind::HeadlessAuditCli,
    AuditSurfaceKind::SupportExportAuditSection,
];

/// Closed vocabulary for the outcome of a state-root resource audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditFindingClass {
    /// Resource clean; no cleanup or repair needed.
    AuditClean,
    /// Resource clean but a disclosure rides on top (e.g. a recent
    /// repair carried a redaction).
    AuditCleanWithDisclosure,
    /// Resource dirty (drift, residue, schema mismatch) and shipped
    /// behind a non-empty disclosure ref.
    AuditDirtyWithDisclosure,
    /// Audit could not classify the resource safely; held for manual
    /// review.
    AuditInconclusiveHeld,
    /// Audit refused to evaluate the resource (e.g. trust-policy or
    /// license narrowing); held.
    AuditRefusedUnsafe,
}

impl AuditFindingClass {
    /// Returns the stable snake_case token for this audit finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuditClean => "audit_clean",
            Self::AuditCleanWithDisclosure => "audit_clean_with_disclosure",
            Self::AuditDirtyWithDisclosure => "audit_dirty_with_disclosure",
            Self::AuditInconclusiveHeld => "audit_inconclusive_held",
            Self::AuditRefusedUnsafe => "audit_refused_unsafe",
        }
    }

    /// True when this finding requires a non-empty audit disclosure
    /// ref.
    pub const fn requires_disclosure(self) -> bool {
        matches!(
            self,
            Self::AuditCleanWithDisclosure
                | Self::AuditDirtyWithDisclosure
                | Self::AuditInconclusiveHeld
                | Self::AuditRefusedUnsafe
        )
    }

    /// True when this finding requires the row to bind a cleanup
    /// precondition (cleanup-surface ref plus inspection hook).
    pub const fn requires_cleanup_precondition(self) -> bool {
        matches!(
            self,
            Self::AuditDirtyWithDisclosure | Self::AuditInconclusiveHeld | Self::AuditRefusedUnsafe
        )
    }
}

/// Closed vocabulary for the no-rerun posture a state-root audit
/// declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRerunPosture {
    /// Requires an explicit user commit action before re-running.
    ExplicitUserActionRequired,
    /// Terminal: does not re-fire after the captured run.
    TerminalNoFurtherRun,
    /// May re-fire silently — forbidden on Stable rows.
    SilentRerunPermitted,
}

impl AuditRerunPosture {
    /// Returns the stable snake_case token for this rerun posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserActionRequired => "explicit_user_action_required",
            Self::TerminalNoFurtherRun => "terminal_no_further_run",
            Self::SilentRerunPermitted => "silent_rerun_permitted",
        }
    }

    /// True when this posture is safe to ship on a Stable row.
    pub const fn safe_for_stable(self) -> bool {
        matches!(
            self,
            Self::ExplicitUserActionRequired | Self::TerminalNoFurtherRun
        )
    }
}

/// Closed redaction class for audit rows shipped into support
/// evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRedactionClass {
    /// Metadata-safe row; no body bytes, no payload refs.
    MetadataOnly,
    /// Body shipped behind an explicit override-disclosure ref.
    RedactedWithDisclosure,
    /// Body excluded by policy (trust / license / export control).
    ExcludedByPolicy,
}

impl AuditRedactionClass {
    /// Returns the stable snake_case token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::RedactedWithDisclosure => "redacted_with_disclosure",
            Self::ExcludedByPolicy => "excluded_by_policy",
        }
    }

    /// True when this class requires an explicit override-disclosure
    /// ref before being shipped.
    pub const fn requires_override_disclosure(self) -> bool {
        matches!(self, Self::RedactedWithDisclosure)
    }
}

/// Closed vocabulary for the claimed stable profile a record
/// certifies against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedStableProfile {
    /// Default Stable profile: every surface and resource bound.
    StableDefault,
    /// Stable profile under the support / shiproom export lane.
    StableSupportExport,
    /// Stable profile under restricted mode (read-only narrowing).
    StableRestrictedMode,
    /// Explicitly narrowed below Stable.
    NarrowedBelowStable,
}

impl ClaimedStableProfile {
    /// Returns the stable snake_case token for this profile.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableDefault => "stable_default",
            Self::StableSupportExport => "stable_support_export",
            Self::StableRestrictedMode => "stable_restricted_mode",
            Self::NarrowedBelowStable => "narrowed_below_stable",
        }
    }

    /// True when the profile is a Stable-claimed posture.
    pub const fn is_stable_claim(self) -> bool {
        matches!(
            self,
            Self::StableDefault | Self::StableSupportExport | Self::StableRestrictedMode
        )
    }
}

/// Closed vocabulary for pre-action inspection / cleanup / repair
/// hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootInspectionHookClass {
    /// Open the state-root inspector with the resource class,
    /// storage-class ref, audit_transaction_id, and finding_code.
    InspectStateRoot,
    /// Compare the in-place state-root resource to the would-be
    /// cleaned-up resource before any destructive cleanup commits.
    CompareBeforeCleanup,
    /// Preview the cleanup's effects (bytes evicted, layers narrowed)
    /// before any apply commits.
    PreviewCleanup,
    /// Preview the repair flow's effects before any apply commits.
    PreviewRepair,
    /// Roll a destructive cleanup back to the pre-cleanup state-root
    /// identity.
    RollbackCleanup,
    /// Roll a destructive repair back to the pre-repair state-root
    /// identity.
    RollbackRepair,
    /// Export the state-root resource (support-safe) before any
    /// destructive cleanup commits.
    ExportBeforeCleanup,
    /// Export the state-root resource (support-safe) before any
    /// destructive repair commits.
    ExportBeforeRepair,
}

impl StateRootInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectStateRoot => "inspect_state_root",
            Self::CompareBeforeCleanup => "compare_before_cleanup",
            Self::PreviewCleanup => "preview_cleanup",
            Self::PreviewRepair => "preview_repair",
            Self::RollbackCleanup => "rollback_cleanup",
            Self::RollbackRepair => "rollback_repair",
            Self::ExportBeforeCleanup => "export_before_cleanup",
            Self::ExportBeforeRepair => "export_before_repair",
        }
    }
}

/// One pre-action inspection / cleanup / repair hook offered before
/// a destructive cleanup / repair commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootInspectionHook {
    /// Hook class.
    pub hook_class: StateRootInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-action inspection / cleanup / repair hook
/// table.
pub fn default_state_root_inspection_hooks() -> Vec<StateRootInspectionHook> {
    vec![
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::InspectStateRoot,
            action_id: "state_root_certification.inspect_state_root".to_owned(),
            label: "Inspect state-root resource".to_owned(),
            available: true,
            disclosure:
                "Opens the state-root inspector with the resource class, storage-class ref, audit_transaction_id, finding_code, and restore provenance before any cleanup or repair commits."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::CompareBeforeCleanup,
            action_id: "state_root_certification.compare_before_cleanup".to_owned(),
            label: "Compare before cleanup".to_owned(),
            available: true,
            disclosure:
                "Renders the typed compare view between the in-place state-root resource and the would-be cleaned-up resource so the user can review residue before any cleanup applies."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::PreviewCleanup,
            action_id: "state_root_certification.preview_cleanup".to_owned(),
            label: "Preview cleanup".to_owned(),
            available: true,
            disclosure:
                "Previews the bytes evicted, layers narrowed, and restore-provenance changes the cleanup will land before any apply commits."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::PreviewRepair,
            action_id: "state_root_certification.preview_repair".to_owned(),
            label: "Preview repair".to_owned(),
            available: true,
            disclosure:
                "Previews the repair flow's effects (state regenerated, residue quarantined, restore-provenance changes) before any apply commits."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::RollbackCleanup,
            action_id: "state_root_certification.rollback_cleanup".to_owned(),
            label: "Roll back cleanup".to_owned(),
            available: true,
            disclosure:
                "Reverts the most recent destructive cleanup back to the pre-cleanup state-root identity, preserving restore provenance, encoding, and trust state."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::RollbackRepair,
            action_id: "state_root_certification.rollback_repair".to_owned(),
            label: "Roll back repair".to_owned(),
            available: true,
            disclosure:
                "Reverts the most recent destructive repair back to the pre-repair state-root identity, preserving restore provenance, encoding, and trust state."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::ExportBeforeCleanup,
            action_id: "state_root_certification.export_before_cleanup".to_owned(),
            label: "Export before cleanup".to_owned(),
            available: true,
            disclosure:
                "Exports the in-place state-root resource (support-safe) before any destructive cleanup commits so the user can replay or audit elsewhere."
                    .to_owned(),
        },
        StateRootInspectionHook {
            hook_class: StateRootInspectionHookClass::ExportBeforeRepair,
            action_id: "state_root_certification.export_before_repair".to_owned(),
            label: "Export before repair".to_owned(),
            available: true,
            disclosure:
                "Exports the in-place state-root resource (support-safe) before any destructive repair commits so the user can replay or audit elsewhere."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for an audit row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootSupportExportInputs {
    /// Whether the row ships a metadata-safe export or holds for
    /// manual review.
    pub posture: StateRootSupportExportPosture,
    pub includes_resource_class: bool,
    pub includes_audit_finding: bool,
    pub includes_storage_class_ref: bool,
    pub includes_claimed_profile: bool,
    pub includes_audit_transaction_id: bool,
    pub includes_finding_code: bool,
    pub includes_redaction_class: bool,
    pub raw_secrets_excluded: bool,
    pub raw_artifact_bytes_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl StateRootSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: StateRootSupportExportPosture) -> Self {
        Self {
            posture,
            includes_resource_class: true,
            includes_audit_finding: true,
            includes_storage_class_ref: true,
            includes_claimed_profile: true,
            includes_audit_transaction_id: true,
            includes_finding_code: true,
            includes_redaction_class: true,
            raw_secrets_excluded: true,
            raw_artifact_bytes_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// Closed support-export posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootSupportExportPosture {
    /// Row ships a metadata-safe projection in the support packet.
    MetadataSafeExport,
    /// Row withholds its state until manual review.
    HeldRecord,
}

impl StateRootSupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// One observation of a state-root resource audit captured at a
/// moment in time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAuditObservation {
    /// Stable resource identity every consumer preserves.
    pub resource_id: String,
    /// Closed state-root resource class.
    pub resource_class: StateRootResourceKind,
    /// Opaque resource ref (e.g. `state-root:workspace:abc`).
    pub resource_ref: String,
    /// Opaque ref into the storage-class lineage (e.g.
    /// `cache:durable_workspace_state`).
    pub storage_class_ref: String,
    /// Closed audit finding.
    pub audit_finding: AuditFindingClass,
    /// Optional audit-disclosure ref (required for findings that
    /// declare dirty / inconclusive / refused state).
    pub audit_disclosure_ref: Option<String>,
    /// Stable audit-transaction id pinned for this audit.
    pub audit_transaction_id: String,
    /// Stable finding code (e.g. `WS-AUD-0001`).
    pub finding_code: String,
    /// True when the audit preserves the restore provenance of the
    /// resource.
    pub preserves_restore_provenance: bool,
    /// True when the audit preserves the source encoding/newline
    /// class.
    pub preserves_encoding_fidelity: bool,
    /// True when the audit preserves the workspace trust state.
    pub preserves_trust_state: bool,
    /// True when the audit preserves the resource's lineage refs
    /// (storage-class ref + restore-of ref + mutation-journal ref).
    pub preserves_lineage_refs: bool,
    /// Closed no-rerun posture for this audit.
    pub rerun_posture: AuditRerunPosture,
    /// True when the audit may mutate persistent state (e.g. cleanup,
    /// quarantine).
    pub mutates_state: bool,
    /// Stable id of the commit action that gates this audit
    /// (required when the audit mutates persistent state).
    pub commit_action_id: String,
    /// Stable id of the disclosure paired with the commit action
    /// (required when the audit mutates persistent state).
    pub commit_disclosure_id: String,
    /// Refs into cleanup surfaces the user can reach (e.g.
    /// `cleanup:settings_panel`). Required to be non-empty when the
    /// audit finding declares dirty / inconclusive / refused state.
    pub cleanup_surface_refs: Vec<String>,
    /// Refs into inspection hooks the user must reach before any
    /// destructive cleanup (e.g.
    /// `state_root_certification.compare_before_cleanup`). Required
    /// to be non-empty when the audit finding declares dirty /
    /// inconclusive / refused state.
    pub inspection_hook_refs: Vec<String>,
    /// Closed redaction class for support evidence.
    pub redaction_class: AuditRedactionClass,
    /// Optional redaction-disclosure ref (required when the redaction
    /// class requires one).
    pub redaction_disclosure_ref: Option<String>,
    /// Support-export projection for the audit row.
    pub support_export: StateRootSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One observation of an audit surface captured at a moment in time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditSurfaceObservation {
    /// Stable audit-surface id.
    pub audit_surface_id: String,
    /// Human-readable label.
    pub label: String,
    /// Closed audit-surface kind.
    pub audit_surface_kind: AuditSurfaceKind,
    /// True when the surface is reachable on this posture.
    pub reachable: bool,
    /// True when the surface preserves the resource lineage refs it
    /// projects.
    pub preserves_lineage_refs: bool,
    /// True when the surface preserves the resource trust state it
    /// projects.
    pub preserves_trust_state: bool,
    /// True when the surface declares its own audit-disclosure ref
    /// whenever a non-clean finding is in view.
    pub discloses_non_clean_findings: bool,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootCertificationInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Claimed stable profile this corpus certifies against.
    pub claimed_profile: ClaimedStableProfile,
    /// Captured state-root resource audits.
    pub resource_audits: Vec<ResourceAuditObservation>,
    /// Captured audit-surface observations.
    pub audit_surfaces: Vec<AuditSurfaceObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a state-root certification lineage record narrows
/// below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootCertificationLineageNarrowReason {
    /// The captured input had no resource audits or no audit surfaces.
    CorpusEmpty,
    /// A required state-root resource class is missing from the
    /// corpus.
    RequiredResourceClassMissing,
    /// A required audit surface is missing from the corpus.
    RequiredAuditSurfaceMissing,
    /// A resource audit references an empty storage-class ref.
    StorageClassRefMissing,
    /// An audit declares a non-clean finding without an audit
    /// disclosure ref.
    AuditDisclosureMissing,
    /// An audit row ships a redaction class that requires an
    /// override-disclosure ref but no disclosure ref is present.
    RedactionDisclosureMissing,
    /// A dirty / held / refused audit row does not bind at least one
    /// cleanup-surface ref or at least one inspection hook ref.
    CleanupPreconditionMissing,
    /// An audit row declares `silent_rerun_permitted` (forbidden on
    /// Stable rows).
    RerunSilentForbidden,
    /// A state-mutating audit row is missing its commit action id or
    /// commit disclosure id.
    CommitActionMetadataMissing,
    /// An audit row does not preserve restore provenance.
    RestoreProvenanceNotPreserved,
    /// An audit row does not preserve encoding fidelity.
    EncodingFidelityNotPreserved,
    /// An audit row does not preserve trust state.
    TrustStateNotPreserved,
    /// An audit row does not preserve resource lineage refs.
    LineageRefsNotPreserved,
    /// An audit row ships without an audit-transaction id.
    AuditTransactionIdNotPinned,
    /// An audit row ships without a finding code.
    FindingCodeMissing,
    /// An audit surface is unreachable on the captured profile.
    AuditSurfaceUnreachable,
    /// An audit surface fails to disclose non-clean findings.
    AuditSurfaceDisclosureGap,
    /// A required pre-action inspection / cleanup / repair hook is
    /// unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, raw artifact bytes, approval tickets, delegated
    /// credentials, or live authority handles slipped into a
    /// support-export projection.
    SupportExportRedactionUnsafe,
    /// The claimed profile is not a Stable-claimed posture.
    ClaimedProfileNotStable,
    /// Producer attribution is incomplete (producer ref or
    /// captured-at empty).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl StateRootCertificationLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredResourceClassMissing => "required_resource_class_missing",
            Self::RequiredAuditSurfaceMissing => "required_audit_surface_missing",
            Self::StorageClassRefMissing => "storage_class_ref_missing",
            Self::AuditDisclosureMissing => "audit_disclosure_missing",
            Self::RedactionDisclosureMissing => "redaction_disclosure_missing",
            Self::CleanupPreconditionMissing => "cleanup_precondition_missing",
            Self::RerunSilentForbidden => "rerun_silent_forbidden",
            Self::CommitActionMetadataMissing => "commit_action_metadata_missing",
            Self::RestoreProvenanceNotPreserved => "restore_provenance_not_preserved",
            Self::EncodingFidelityNotPreserved => "encoding_fidelity_not_preserved",
            Self::TrustStateNotPreserved => "trust_state_not_preserved",
            Self::LineageRefsNotPreserved => "lineage_refs_not_preserved",
            Self::AuditTransactionIdNotPinned => "audit_transaction_id_not_pinned",
            Self::FindingCodeMissing => "finding_code_missing",
            Self::AuditSurfaceUnreachable => "audit_surface_unreachable",
            Self::AuditSurfaceDisclosureGap => "audit_surface_disclosure_gap",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::ClaimedProfileNotStable => "claimed_profile_not_stable",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a state-root certification
/// lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootCertificationLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<StateRootCertificationLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One resource audit row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAuditRow {
    pub resource_id: String,
    pub resource_class: StateRootResourceKind,
    pub resource_ref: String,
    pub storage_class_ref: String,
    pub audit_finding: AuditFindingClass,
    pub audit_disclosure_ref: Option<String>,
    pub audit_transaction_id: String,
    pub finding_code: String,
    pub preserves_restore_provenance: bool,
    pub preserves_encoding_fidelity: bool,
    pub preserves_trust_state: bool,
    pub preserves_lineage_refs: bool,
    pub rerun_posture: AuditRerunPosture,
    pub mutates_state: bool,
    pub commit_action_id: String,
    pub commit_disclosure_id: String,
    pub cleanup_surface_refs: Vec<String>,
    pub inspection_hook_refs: Vec<String>,
    pub redaction_class: AuditRedactionClass,
    pub redaction_disclosure_ref: Option<String>,
    pub support_export_posture: StateRootSupportExportPosture,
    pub is_required: bool,
}

/// One audit-surface row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditSurfaceRow {
    pub audit_surface_id: String,
    pub label: String,
    pub audit_surface_kind: AuditSurfaceKind,
    pub reachable: bool,
    pub preserves_lineage_refs: bool,
    pub preserves_trust_state: bool,
    pub discloses_non_clean_findings: bool,
    pub is_required: bool,
}

/// Resource-class coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceClassCoverageSummary {
    pub resource_audit_rows: Vec<ResourceAuditRow>,
    pub all_required_resource_classes_present: bool,
}

/// Audit-surface coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditSurfaceCoverageSummary {
    pub audit_surface_rows: Vec<AuditSurfaceRow>,
    pub all_required_audit_surfaces_present: bool,
}

/// Storage-class taxonomy / audit-finding honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditHonestySummary {
    pub all_rows_pin_storage_class_ref: bool,
    pub all_audit_disclosures_present: bool,
    pub all_redaction_disclosures_present: bool,
    pub all_dirty_rows_have_cleanup_precondition: bool,
}

/// Preservation posture for restore provenance, encoding, trust
/// state, and lineage refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservationSummary {
    pub all_rows_preserve_restore_provenance: bool,
    pub all_rows_preserve_encoding_fidelity: bool,
    pub all_rows_preserve_trust_state: bool,
    pub all_rows_preserve_lineage_refs: bool,
}

/// No-silent-rerun posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSilentRerunSummary {
    pub all_rows_safe_rerun_posture: bool,
    pub all_mutating_rows_have_commit_metadata: bool,
}

/// Audit-transaction-id / finding-code pinning posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditTransactionPinningSummary {
    pub all_rows_pin_audit_transaction_id: bool,
    pub all_rows_pin_finding_code: bool,
}

/// Surface-reachability / disclosure posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditSurfaceReachabilitySummary {
    pub all_required_surfaces_reachable: bool,
    pub all_required_surfaces_disclose_non_clean: bool,
    pub all_required_surfaces_preserve_lineage_refs: bool,
    pub all_required_surfaces_preserve_trust_state: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootSupportExportHonestySummary {
    pub all_rows_preserve_fields: bool,
    pub all_rows_exclude_raw_secrets: bool,
    pub all_rows_exclude_raw_artifact_bytes: bool,
    pub all_rows_exclude_approval_tickets: bool,
    pub all_rows_exclude_delegated_credentials: bool,
    pub all_rows_exclude_live_authority_handles: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootProducerAttributionSummary {
    pub producer_ref: String,
    pub schema_version: u32,
    pub integrity_hash: String,
    pub captured_at: String,
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe state-root certification lineage record per
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRootCertificationLineageRecord {
    pub record_kind: String,
    pub state_root_certification_lineage_schema_version: u32,
    pub schema_ref: String,
    pub posture_id: String,
    pub workspace_ref: String,
    pub corpus_ref: String,
    pub claimed_profile: ClaimedStableProfile,
    pub producer_attribution: StateRootProducerAttributionSummary,
    pub resource_class_coverage: ResourceClassCoverageSummary,
    pub audit_surface_coverage: AuditSurfaceCoverageSummary,
    pub audit_honesty: AuditHonestySummary,
    pub preservation: PreservationSummary,
    pub no_silent_rerun: NoSilentRerunSummary,
    pub audit_transaction_pinning: AuditTransactionPinningSummary,
    pub audit_surface_reachability: AuditSurfaceReachabilitySummary,
    pub support_export_honesty: StateRootSupportExportHonestySummary,
    pub inspection_hooks: Vec<StateRootInspectionHook>,
    pub stable_qualification: StateRootCertificationLineageQualification,
    pub raw_payload_excluded: bool,
    pub summary: String,
}

impl StateRootCertificationLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_REF
            && self.record_kind == STATE_ROOT_CERTIFICATION_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: StateRootInspectionHookClass,
    ) -> Option<&StateRootInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed state-root certification lineage record from a
/// live [`StateRootCertificationInputs`] envelope using the default
/// inspection-hook set.
pub fn project_state_root_certification_lineage(
    posture_id: impl Into<String>,
    inputs: &StateRootCertificationInputs,
) -> StateRootCertificationLineageRecord {
    project_state_root_certification_lineage_with_hooks(
        posture_id,
        inputs,
        default_state_root_inspection_hooks(),
    )
}

/// Like [`project_state_root_certification_lineage`] but with an
/// explicit inspection-hook set (for testing degraded-hook postures).
pub fn project_state_root_certification_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &StateRootCertificationInputs,
    inspection_hooks: Vec<StateRootInspectionHook>,
) -> StateRootCertificationLineageRecord {
    let posture_id: String = posture_id.into();

    let resource_class_coverage = project_resource_class_coverage(inputs);
    let audit_surface_coverage = project_audit_surface_coverage(inputs);
    let audit_honesty = project_audit_honesty(&resource_class_coverage);
    let preservation = project_preservation(&resource_class_coverage);
    let no_silent_rerun = project_no_silent_rerun(&resource_class_coverage);
    let audit_transaction_pinning = project_audit_transaction_pinning(&resource_class_coverage);
    let audit_surface_reachability = project_audit_surface_reachability(&audit_surface_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.resource_audits.is_empty() || inputs.audit_surfaces.is_empty() {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::CorpusEmpty);
    }
    if !resource_class_coverage.all_required_resource_classes_present {
        narrow_reasons
            .push(StateRootCertificationLineageNarrowReason::RequiredResourceClassMissing);
    }
    if !audit_surface_coverage.all_required_audit_surfaces_present {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::RequiredAuditSurfaceMissing);
    }
    if !audit_honesty.all_rows_pin_storage_class_ref {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::StorageClassRefMissing);
    }
    if !audit_honesty.all_audit_disclosures_present {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::AuditDisclosureMissing);
    }
    if !audit_honesty.all_redaction_disclosures_present {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::RedactionDisclosureMissing);
    }
    if !audit_honesty.all_dirty_rows_have_cleanup_precondition {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::CleanupPreconditionMissing);
    }
    if !no_silent_rerun.all_rows_safe_rerun_posture {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::RerunSilentForbidden);
    }
    if !no_silent_rerun.all_mutating_rows_have_commit_metadata {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::CommitActionMetadataMissing);
    }
    if !preservation.all_rows_preserve_restore_provenance {
        narrow_reasons
            .push(StateRootCertificationLineageNarrowReason::RestoreProvenanceNotPreserved);
    }
    if !preservation.all_rows_preserve_encoding_fidelity {
        narrow_reasons
            .push(StateRootCertificationLineageNarrowReason::EncodingFidelityNotPreserved);
    }
    if !preservation.all_rows_preserve_trust_state {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::TrustStateNotPreserved);
    }
    if !preservation.all_rows_preserve_lineage_refs {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::LineageRefsNotPreserved);
    }
    if !audit_transaction_pinning.all_rows_pin_audit_transaction_id {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::AuditTransactionIdNotPinned);
    }
    if !audit_transaction_pinning.all_rows_pin_finding_code {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::FindingCodeMissing);
    }
    if !audit_surface_reachability.all_required_surfaces_reachable {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::AuditSurfaceUnreachable);
    }
    if !(audit_surface_reachability.all_required_surfaces_disclose_non_clean
        && audit_surface_reachability.all_required_surfaces_preserve_lineage_refs
        && audit_surface_reachability.all_required_surfaces_preserve_trust_state)
    {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::AuditSurfaceDisclosureGap);
    }

    let required_hooks = [
        StateRootInspectionHookClass::InspectStateRoot,
        StateRootInspectionHookClass::CompareBeforeCleanup,
        StateRootInspectionHookClass::PreviewCleanup,
        StateRootInspectionHookClass::PreviewRepair,
        StateRootInspectionHookClass::RollbackCleanup,
        StateRootInspectionHookClass::RollbackRepair,
        StateRootInspectionHookClass::ExportBeforeCleanup,
        StateRootInspectionHookClass::ExportBeforeRepair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::InspectionHookUnavailable);
    }

    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    if !inputs.claimed_profile.is_stable_claim() {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::ClaimedProfileNotStable);
    }

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons
            .push(StateRootCertificationLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = StateRootCertificationLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &resource_class_coverage,
        &audit_surface_coverage,
        &stable_qualification,
    );

    StateRootCertificationLineageRecord {
        record_kind: STATE_ROOT_CERTIFICATION_LINEAGE_RECORD_KIND.to_owned(),
        state_root_certification_lineage_schema_version:
            STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_VERSION,
        schema_ref: STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        claimed_profile: inputs.claimed_profile,
        producer_attribution,
        resource_class_coverage,
        audit_surface_coverage,
        audit_honesty,
        preservation,
        no_silent_rerun,
        audit_transaction_pinning,
        audit_surface_reachability,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_resource_class_coverage(
    inputs: &StateRootCertificationInputs,
) -> ResourceClassCoverageSummary {
    let resource_audit_rows: Vec<ResourceAuditRow> = inputs
        .resource_audits
        .iter()
        .map(project_resource_audit_row)
        .collect();
    let observed: BTreeSet<_> = resource_audit_rows
        .iter()
        .map(|row| row.resource_class)
        .collect();
    let all_required_resource_classes_present = REQUIRED_STATE_ROOT_RESOURCES
        .iter()
        .all(|required| observed.contains(required));
    ResourceClassCoverageSummary {
        resource_audit_rows,
        all_required_resource_classes_present,
    }
}

fn project_resource_audit_row(observation: &ResourceAuditObservation) -> ResourceAuditRow {
    ResourceAuditRow {
        resource_id: observation.resource_id.clone(),
        resource_class: observation.resource_class,
        resource_ref: observation.resource_ref.clone(),
        storage_class_ref: observation.storage_class_ref.clone(),
        audit_finding: observation.audit_finding,
        audit_disclosure_ref: observation.audit_disclosure_ref.clone(),
        audit_transaction_id: observation.audit_transaction_id.clone(),
        finding_code: observation.finding_code.clone(),
        preserves_restore_provenance: observation.preserves_restore_provenance,
        preserves_encoding_fidelity: observation.preserves_encoding_fidelity,
        preserves_trust_state: observation.preserves_trust_state,
        preserves_lineage_refs: observation.preserves_lineage_refs,
        rerun_posture: observation.rerun_posture,
        mutates_state: observation.mutates_state,
        commit_action_id: observation.commit_action_id.clone(),
        commit_disclosure_id: observation.commit_disclosure_id.clone(),
        cleanup_surface_refs: observation.cleanup_surface_refs.clone(),
        inspection_hook_refs: observation.inspection_hook_refs.clone(),
        redaction_class: observation.redaction_class,
        redaction_disclosure_ref: observation.redaction_disclosure_ref.clone(),
        support_export_posture: observation.support_export.posture,
        is_required: observation.resource_class.is_required(),
    }
}

fn project_audit_surface_coverage(
    inputs: &StateRootCertificationInputs,
) -> AuditSurfaceCoverageSummary {
    let audit_surface_rows: Vec<AuditSurfaceRow> = inputs
        .audit_surfaces
        .iter()
        .map(project_audit_surface_row)
        .collect();
    let observed: BTreeSet<_> = audit_surface_rows
        .iter()
        .map(|row| row.audit_surface_kind)
        .collect();
    let all_required_audit_surfaces_present = REQUIRED_AUDIT_SURFACES
        .iter()
        .all(|required| observed.contains(required));
    AuditSurfaceCoverageSummary {
        audit_surface_rows,
        all_required_audit_surfaces_present,
    }
}

fn project_audit_surface_row(observation: &AuditSurfaceObservation) -> AuditSurfaceRow {
    AuditSurfaceRow {
        audit_surface_id: observation.audit_surface_id.clone(),
        label: observation.label.clone(),
        audit_surface_kind: observation.audit_surface_kind,
        reachable: observation.reachable,
        preserves_lineage_refs: observation.preserves_lineage_refs,
        preserves_trust_state: observation.preserves_trust_state,
        discloses_non_clean_findings: observation.discloses_non_clean_findings,
        is_required: true,
    }
}

fn project_audit_honesty(coverage: &ResourceClassCoverageSummary) -> AuditHonestySummary {
    let mut storage_ok = true;
    let mut audit_ok = true;
    let mut red_ok = true;
    let mut cleanup_ok = true;
    for row in &coverage.resource_audit_rows {
        if row.storage_class_ref.trim().is_empty() {
            storage_ok = false;
        }
        if row.audit_finding.requires_disclosure()
            && row
                .audit_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            audit_ok = false;
        }
        if row.redaction_class.requires_override_disclosure()
            && row
                .redaction_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            red_ok = false;
        }
        if row.audit_finding.requires_cleanup_precondition()
            && (row.cleanup_surface_refs.is_empty() || row.inspection_hook_refs.is_empty())
        {
            cleanup_ok = false;
        }
    }
    AuditHonestySummary {
        all_rows_pin_storage_class_ref: storage_ok,
        all_audit_disclosures_present: audit_ok,
        all_redaction_disclosures_present: red_ok,
        all_dirty_rows_have_cleanup_precondition: cleanup_ok,
    }
}

fn project_preservation(coverage: &ResourceClassCoverageSummary) -> PreservationSummary {
    let mut prov_ok = true;
    let mut enc_ok = true;
    let mut trust_ok = true;
    let mut lineage_ok = true;
    for row in &coverage.resource_audit_rows {
        if !row.preserves_restore_provenance {
            prov_ok = false;
        }
        if !row.preserves_encoding_fidelity {
            enc_ok = false;
        }
        if !row.preserves_trust_state {
            trust_ok = false;
        }
        if !row.preserves_lineage_refs {
            lineage_ok = false;
        }
    }
    PreservationSummary {
        all_rows_preserve_restore_provenance: prov_ok,
        all_rows_preserve_encoding_fidelity: enc_ok,
        all_rows_preserve_trust_state: trust_ok,
        all_rows_preserve_lineage_refs: lineage_ok,
    }
}

fn project_no_silent_rerun(coverage: &ResourceClassCoverageSummary) -> NoSilentRerunSummary {
    let mut posture_ok = true;
    let mut commit_ok = true;
    for row in &coverage.resource_audit_rows {
        if !row.rerun_posture.safe_for_stable() {
            posture_ok = false;
        }
        if row.mutates_state
            && (row.commit_action_id.trim().is_empty()
                || row.commit_disclosure_id.trim().is_empty())
        {
            commit_ok = false;
        }
    }
    NoSilentRerunSummary {
        all_rows_safe_rerun_posture: posture_ok,
        all_mutating_rows_have_commit_metadata: commit_ok,
    }
}

fn project_audit_transaction_pinning(
    coverage: &ResourceClassCoverageSummary,
) -> AuditTransactionPinningSummary {
    let mut tx_ok = true;
    let mut finding_ok = true;
    for row in &coverage.resource_audit_rows {
        if row.audit_transaction_id.trim().is_empty() {
            tx_ok = false;
        }
        if row.finding_code.trim().is_empty() {
            finding_ok = false;
        }
    }
    AuditTransactionPinningSummary {
        all_rows_pin_audit_transaction_id: tx_ok,
        all_rows_pin_finding_code: finding_ok,
    }
}

fn project_audit_surface_reachability(
    coverage: &AuditSurfaceCoverageSummary,
) -> AuditSurfaceReachabilitySummary {
    let mut reach_ok = true;
    let mut disclose_ok = true;
    let mut lineage_ok = true;
    let mut trust_ok = true;
    for row in &coverage.audit_surface_rows {
        if !row.reachable {
            reach_ok = false;
        }
        if !row.discloses_non_clean_findings {
            disclose_ok = false;
        }
        if !row.preserves_lineage_refs {
            lineage_ok = false;
        }
        if !row.preserves_trust_state {
            trust_ok = false;
        }
    }
    AuditSurfaceReachabilitySummary {
        all_required_surfaces_reachable: reach_ok,
        all_required_surfaces_disclose_non_clean: disclose_ok,
        all_required_surfaces_preserve_lineage_refs: lineage_ok,
        all_required_surfaces_preserve_trust_state: trust_ok,
    }
}

fn project_support_export_honesty(
    inputs: &StateRootCertificationInputs,
) -> StateRootSupportExportHonestySummary {
    let mut preserve_fields = true;
    let mut redact_secrets = true;
    let mut exclude_bytes = true;
    let mut exclude_approvals = true;
    let mut exclude_credentials = true;
    let mut exclude_authority = true;

    for support in inputs.resource_audits.iter().map(|a| a.support_export) {
        if !(support.includes_resource_class
            && support.includes_audit_finding
            && support.includes_storage_class_ref
            && support.includes_claimed_profile
            && support.includes_audit_transaction_id
            && support.includes_finding_code
            && support.includes_redaction_class)
        {
            preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            redact_secrets = false;
        }
        if !support.raw_artifact_bytes_excluded {
            exclude_bytes = false;
        }
        if !support.approval_tickets_excluded {
            exclude_approvals = false;
        }
        if !support.delegated_credentials_excluded {
            exclude_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            exclude_authority = false;
        }
    }

    StateRootSupportExportHonestySummary {
        all_rows_preserve_fields: preserve_fields,
        all_rows_exclude_raw_secrets: redact_secrets,
        all_rows_exclude_raw_artifact_bytes: exclude_bytes,
        all_rows_exclude_approval_tickets: exclude_approvals,
        all_rows_exclude_delegated_credentials: exclude_credentials,
        all_rows_exclude_live_authority_handles: exclude_authority,
    }
}

fn project_producer_attribution(
    inputs: &StateRootCertificationInputs,
) -> StateRootProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    StateRootProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_support_export_narrows(
    summary: &StateRootSupportExportHonestySummary,
    narrow_reasons: &mut Vec<StateRootCertificationLineageNarrowReason>,
) {
    if !summary.all_rows_preserve_fields {
        narrow_reasons.push(StateRootCertificationLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !(summary.all_rows_exclude_raw_secrets
        && summary.all_rows_exclude_raw_artifact_bytes
        && summary.all_rows_exclude_approval_tickets
        && summary.all_rows_exclude_delegated_credentials
        && summary.all_rows_exclude_live_authority_handles)
    {
        narrow_reasons
            .push(StateRootCertificationLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &StateRootCertificationInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
        inputs.claimed_profile.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for audit in &inputs.resource_audits {
        for byte in audit.resource_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(audit.resource_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(audit.audit_finding.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for surface in &inputs.audit_surfaces {
        for byte in surface.audit_surface_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(surface.audit_surface_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("src:{hash:016x}")
}

fn hook_available(hooks: &[StateRootInspectionHook], class: StateRootInspectionHookClass) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    resource_coverage: &ResourceClassCoverageSummary,
    surface_coverage: &AuditSurfaceCoverageSummary,
    qualification: &StateRootCertificationLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "State-root certification lineage proven Stable: resources={resources} audit_surfaces={surfaces}.",
            resources = resource_coverage.resource_audit_rows.len(),
            surfaces = surface_coverage.audit_surface_rows.len(),
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "State-root certification lineage narrowed below Stable (resources={resources} audit_surfaces={surfaces}): {reasons}.",
            resources = resource_coverage.resource_audit_rows.len(),
            surfaces = surface_coverage.audit_surface_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a state-root
/// certification lineage record. The same projection is consumed by
/// the workspace state-root audit status surface, the headless CLI
/// emitter, Help/About, and support export.
pub fn state_root_certification_lineage_lines(
    record: &StateRootCertificationLineageRecord,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "State-root certification lineage — {} ({}) profile={}",
        record.posture_id,
        record.stable_qualification.level,
        record.claimed_profile.as_str(),
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
    ));
    lines.push(format!(
        "resource_class_coverage: resources={} required_present={}",
        record.resource_class_coverage.resource_audit_rows.len(),
        record
            .resource_class_coverage
            .all_required_resource_classes_present,
    ));
    lines.push("Resource audits:".to_owned());
    for row in &record.resource_class_coverage.resource_audit_rows {
        let aud_disclosure = row.audit_disclosure_ref.as_deref().unwrap_or("none");
        let red_disclosure = row.redaction_disclosure_ref.as_deref().unwrap_or("none");
        lines.push(format!(
            "  - {class} {id} ref={resource_ref} storage={storage} finding={finding} aud_disclosure={aud_disclosure} audit_tx={tx} finding_code={finding_code} preserves_provenance={prov} preserves_encoding={enc} preserves_trust={trust} preserves_lineage={lineage} rerun_posture={rerun_posture} mutates={mutates} commit_action={commit_action} commit_disclosure={commit_disclosure} cleanup_surfaces={cleanup_surfaces} inspection_hooks={inspection_hooks} redaction={red} red_disclosure={red_disclosure} required={required} support_export={support}",
            class = row.resource_class.as_str(),
            id = row.resource_id,
            resource_ref = row.resource_ref,
            storage = row.storage_class_ref,
            finding = row.audit_finding.as_str(),
            aud_disclosure = aud_disclosure,
            tx = row.audit_transaction_id,
            finding_code = row.finding_code,
            prov = row.preserves_restore_provenance,
            enc = row.preserves_encoding_fidelity,
            trust = row.preserves_trust_state,
            lineage = row.preserves_lineage_refs,
            rerun_posture = row.rerun_posture.as_str(),
            mutates = row.mutates_state,
            commit_action = row.commit_action_id,
            commit_disclosure = row.commit_disclosure_id,
            cleanup_surfaces = row.cleanup_surface_refs.join("|"),
            inspection_hooks = row.inspection_hook_refs.join("|"),
            red = row.redaction_class.as_str(),
            red_disclosure = red_disclosure,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "audit_surface_coverage: surfaces={} required_present={}",
        record.audit_surface_coverage.audit_surface_rows.len(),
        record
            .audit_surface_coverage
            .all_required_audit_surfaces_present,
    ));
    lines.push("Audit surfaces:".to_owned());
    for row in &record.audit_surface_coverage.audit_surface_rows {
        lines.push(format!(
            "  - {kind} {id} reachable={reachable} discloses_non_clean={disclose} preserves_lineage={lineage} preserves_trust={trust} required={required} — {label}",
            kind = row.audit_surface_kind.as_str(),
            id = row.audit_surface_id,
            reachable = row.reachable,
            disclose = row.discloses_non_clean_findings,
            lineage = row.preserves_lineage_refs,
            trust = row.preserves_trust_state,
            required = row.is_required,
            label = row.label,
        ));
    }
    lines.push(format!(
        "Audit honesty: storage_ref={s} audit_disclosures={a} redaction_disclosures={r} cleanup_precondition={c}",
        s = record.audit_honesty.all_rows_pin_storage_class_ref,
        a = record.audit_honesty.all_audit_disclosures_present,
        r = record.audit_honesty.all_redaction_disclosures_present,
        c = record.audit_honesty.all_dirty_rows_have_cleanup_precondition,
    ));
    lines.push(format!(
        "Preservation: provenance={p} encoding={e} trust={t} lineage={l}",
        p = record.preservation.all_rows_preserve_restore_provenance,
        e = record.preservation.all_rows_preserve_encoding_fidelity,
        t = record.preservation.all_rows_preserve_trust_state,
        l = record.preservation.all_rows_preserve_lineage_refs,
    ));
    lines.push(format!(
        "No-silent-rerun: posture={p} commit_metadata={c}",
        p = record.no_silent_rerun.all_rows_safe_rerun_posture,
        c = record
            .no_silent_rerun
            .all_mutating_rows_have_commit_metadata,
    ));
    lines.push(format!(
        "Audit-transaction pinning: tx={t} finding={f}",
        t = record
            .audit_transaction_pinning
            .all_rows_pin_audit_transaction_id,
        f = record.audit_transaction_pinning.all_rows_pin_finding_code,
    ));
    lines.push(format!(
        "Audit-surface reachability: reachable={r} disclose_non_clean={d} preserves_lineage={l} preserves_trust={t}",
        r = record
            .audit_surface_reachability
            .all_required_surfaces_reachable,
        d = record
            .audit_surface_reachability
            .all_required_surfaces_disclose_non_clean,
        l = record
            .audit_surface_reachability
            .all_required_surfaces_preserve_lineage_refs,
        t = record
            .audit_surface_reachability
            .all_required_surfaces_preserve_trust_state,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} exclude_secrets={secrets} exclude_bytes={bytes} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority}",
        fields = record.support_export_honesty.all_rows_preserve_fields,
        secrets = record.support_export_honesty.all_rows_exclude_raw_secrets,
        bytes = record
            .support_export_honesty
            .all_rows_exclude_raw_artifact_bytes,
        approvals = record
            .support_export_honesty
            .all_rows_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
