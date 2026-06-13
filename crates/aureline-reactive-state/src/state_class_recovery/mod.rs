//! State-class recovery routing and placeholder-continuity packet for the
//! file-bearing and managed-write families that expanded in the current
//! product generation.
//!
//! The packet freezes one shared recovery vocabulary for durable user state,
//! workspace state, derived caches, generated artifacts, recovery journals,
//! and trust or security state so repair, restore, and support surfaces do
//! not invent different answers for the same damaged family.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/state_class_recovery.schema.json`](../../../../schemas/state/state_class_recovery.schema.json)
//! - [`/docs/state/state_class_recovery.md`](../../../../docs/state/state_class_recovery.md)
//! - [`/artifacts/state/state_class_recovery.json`](../../../../artifacts/state/state_class_recovery.json)
//! - [`/artifacts/state/state_class_recovery.md`](../../../../artifacts/state/state_class_recovery.md)
//! - [`/fixtures/state/state_class_recovery/`](../../../../fixtures/state/state_class_recovery/)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const STATE_CLASS_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const STATE_CLASS_RECOVERY_PACKET_RECORD_KIND: &str = "state_class_recovery_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND: &str = "state_class_recovery_fixture_record";

/// Repo-relative schema ref.
pub const STATE_CLASS_RECOVERY_SCHEMA_REF: &str = "schemas/state/state_class_recovery.schema.json";

/// Repo-relative reviewer doc ref.
pub const STATE_CLASS_RECOVERY_DOC_REF: &str = "docs/state/state_class_recovery.md";

/// Repo-relative machine-readable artifact packet.
pub const STATE_CLASS_RECOVERY_PACKET_REF: &str = "artifacts/state/state_class_recovery.json";

/// Repo-relative reviewer artifact report.
pub const STATE_CLASS_RECOVERY_REPORT_REF: &str = "artifacts/state/state_class_recovery.md";

/// Repo-relative fixture directory.
pub const STATE_CLASS_RECOVERY_FIXTURE_DIR: &str = "fixtures/state/state_class_recovery";

/// Repo-relative fixture manifest.
pub const STATE_CLASS_RECOVERY_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/state_class_recovery/manifest.yaml";

/// Closed state-class vocabulary aligned with the persistence contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
    /// Durable user-owned state that must never be silently rebuilt.
    DurableUserState,
    /// Durable workspace-authored state that may block only its own feature lane.
    WorkspaceState,
    /// Disposable derived caches that rebuild from authoritative truth.
    DerivedCacheState,
    /// Generated artifacts that reopen from source lineage, not ad hoc edits.
    GeneratedArtifactState,
    /// Recovery journals that may preserve unsaved or unpublished work.
    RecoveryJournalState,
    /// Trust, policy, and security state that must fail closed for privileged work.
    SecurityTrustState,
}

impl StateClass {
    /// Returns the stable string vocabulary for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableUserState => "durable_user_state",
            Self::WorkspaceState => "workspace_state",
            Self::DerivedCacheState => "derived_cache_state",
            Self::GeneratedArtifactState => "generated_artifact_state",
            Self::RecoveryJournalState => "recovery_journal_state",
            Self::SecurityTrustState => "security_trust_state",
        }
    }
}

/// State-family surface covered by the recovery packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    /// Notebook documents and notebook-owned structured state.
    NotebookWorkspace,
    /// Request-workspace documents and linked request history.
    RequestWorkspace,
    /// Local-first provider drafts and publish-later state.
    ProviderDraft,
    /// Preview-runtime cache shards and rendered outputs.
    PreviewCache,
    /// Generated notebook outputs, exports, traces, and snapshot artifacts.
    GeneratedArtifacts,
    /// Sync packets, deferred intent journals, and preserved unsaved work.
    SyncJournal,
    /// Policy, entitlement, and trust-gated managed state.
    TrustPolicy,
}

impl SurfaceFamily {
    /// Returns the stable string vocabulary for this surface family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookWorkspace => "notebook_workspace",
            Self::RequestWorkspace => "request_workspace",
            Self::ProviderDraft => "provider_draft",
            Self::PreviewCache => "preview_cache",
            Self::GeneratedArtifacts => "generated_artifacts",
            Self::SyncJournal => "sync_journal",
            Self::TrustPolicy => "trust_policy",
        }
    }
}

/// Authority owner class mirrored from the state-object inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    /// User-authored durable truth.
    UserAuthoredDurableTruth,
    /// User-owned recovery state.
    UserOwnedRecoveryState,
    /// Admin or control-plane artifact.
    AdminOrControlArtifact,
    /// Fully regenerable derived cache.
    DisposableDerivedCache,
}

impl AuthorityClass {
    /// Returns the stable string vocabulary for this authority class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredDurableTruth => "user_authored_durable_truth",
            Self::UserOwnedRecoveryState => "user_owned_recovery_state",
            Self::AdminOrControlArtifact => "admin_or_control_artifact",
            Self::DisposableDerivedCache => "disposable_derived_cache",
        }
    }
}

/// Primary or fallback recovery route selected for a damaged family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryRoute {
    /// Rebuild the family from authoritative truth without touching user state.
    RebuildAutomatically,
    /// Route through a compare-first repair or manual review flow.
    GuidedRepair,
    /// Restore the last preserved durable artifact before mutating again.
    RollbackToPreservedArtifact,
    /// Block privileged or managed actions until integrity is re-established.
    FailClosedPrivilegedOperations,
}

impl RecoveryRoute {
    /// Returns the stable string vocabulary for this recovery route.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RebuildAutomatically => "rebuild_automatically",
            Self::GuidedRepair => "guided_repair",
            Self::RollbackToPreservedArtifact => "rollback_to_preserved_artifact",
            Self::FailClosedPrivilegedOperations => "fail_closed_privileged_operations",
        }
    }
}

/// Failure mode covered by one checked-in recovery family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureMode {
    /// A parse, integrity, or schema problem damaged part of the family.
    PartialCorruption,
    /// A required target, provider, or supporting dependency is missing.
    MissingDependency,
    /// A generated overlay or derivative is stale relative to its source.
    StaleDerivedOverlay,
    /// A cache shard or index segment is unreadable.
    BrokenCacheShard,
    /// Unsaved or unpublished work survives in a journal that needs review.
    JournalPreservedUnsavedWork,
    /// Trust or policy state was quarantined or revoked.
    QuarantinedTrustState,
}

impl FailureMode {
    /// Returns the stable string vocabulary for this failure mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialCorruption => "partial_corruption",
            Self::MissingDependency => "missing_dependency",
            Self::StaleDerivedOverlay => "stale_derived_overlay",
            Self::BrokenCacheShard => "broken_cache_shard",
            Self::JournalPreservedUnsavedWork => "journal_preserved_unsaved_work",
            Self::QuarantinedTrustState => "quarantined_trust_state",
        }
    }
}

/// Placeholder-continuity posture used when a family cannot reopen live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderKind {
    /// Context reopens, but the live family remains absent.
    ContextOnly,
    /// A stale or retained derived artifact stands in until rebuild completes.
    StaleDerivedArtifact,
    /// Preserved drafts reopen for compare or manual recovery.
    RecoveredDraft,
    /// The last preserved durable artifact is ready for compare or rollback.
    RollbackReady,
    /// The pane stays visible but privileged actions remain blocked.
    PrivilegedBlocked,
}

impl PlaceholderKind {
    /// Returns the stable string vocabulary for this placeholder kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextOnly => "context_only",
            Self::StaleDerivedArtifact => "stale_derived_artifact",
            Self::RecoveredDraft => "recovered_draft",
            Self::RollbackReady => "rollback_ready",
            Self::PrivilegedBlocked => "privileged_blocked",
        }
    }
}

/// Context retained around a placeholder-backed reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedContextClass {
    /// The layout slot and pane ordering remain intact.
    LayoutSlot,
    /// Logical object identity remains visible.
    LogicalIdentity,
    /// The surrounding workspace chrome and neighboring panes reopen.
    WorkspaceChrome,
    /// Request, draft, or activity history remains attributable.
    MutationLineage,
    /// Local draft edits remain available for compare.
    DraftEdits,
    /// Last-known redaction-safe metadata stays visible.
    LastKnownMetadata,
    /// Cached evidence or retained artifact refs remain reachable.
    RetainedEvidence,
}

impl PreservedContextClass {
    /// Returns the stable string vocabulary for this context class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutSlot => "layout_slot",
            Self::LogicalIdentity => "logical_identity",
            Self::WorkspaceChrome => "workspace_chrome",
            Self::MutationLineage => "mutation_lineage",
            Self::DraftEdits => "draft_edits",
            Self::LastKnownMetadata => "last_known_metadata",
            Self::RetainedEvidence => "retained_evidence",
        }
    }
}

/// Capability that remains blocked while a placeholder is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedCapabilityClass {
    /// Live execution or runtime-backed refresh stays unavailable.
    LiveExecution,
    /// Direct writes to the damaged family stay blocked.
    DirectWrite,
    /// Managed publish or sync apply stays blocked.
    ManagedPublish,
    /// Regeneration cannot be claimed as current truth yet.
    RegenerationClaims,
    /// Background refresh or rebuild is paused or unhealthy.
    BackgroundRefresh,
    /// Privileged operations stay blocked until trust is repaired.
    PrivilegedApply,
}

impl BlockedCapabilityClass {
    /// Returns the stable string vocabulary for this blocked capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveExecution => "live_execution",
            Self::DirectWrite => "direct_write",
            Self::ManagedPublish => "managed_publish",
            Self::RegenerationClaims => "regeneration_claims",
            Self::BackgroundRefresh => "background_refresh",
            Self::PrivilegedApply => "privileged_apply",
        }
    }
}

/// Recovery action offered by a placeholder-backed family reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderActionClass {
    /// Start or reopen the guided repair flow.
    StartRepairFlow,
    /// Compare the retained artifact or draft against current state.
    ComparePreservedArtifact,
    /// Restore the last preserved durable artifact.
    RestoreFromRollback,
    /// Retry an automatic rebuild or regeneration pass.
    RetryRebuild,
    /// Reauthenticate or rebind a managed dependency.
    ReauthenticateManagedSurface,
    /// Continue without the missing artifact in place.
    OpenWithoutArtifact,
    /// Export the support packet or retained evidence.
    ExportSupportPacket,
    /// Deliberately close the placeholder while preserving surrounding layout.
    RemovePlaceholder,
}

impl PlaceholderActionClass {
    /// Returns the stable string vocabulary for this placeholder action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartRepairFlow => "start_repair_flow",
            Self::ComparePreservedArtifact => "compare_preserved_artifact",
            Self::RestoreFromRollback => "restore_from_rollback",
            Self::RetryRebuild => "retry_rebuild",
            Self::ReauthenticateManagedSurface => "reauthenticate_managed_surface",
            Self::OpenWithoutArtifact => "open_without_artifact",
            Self::ExportSupportPacket => "export_support_packet",
            Self::RemovePlaceholder => "remove_placeholder",
        }
    }
}

/// Placeholder continuity plan bound to one state family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderContinuityPlan {
    /// Placeholder posture for this family.
    pub placeholder_kind: PlaceholderKind,
    /// True when the original layout slot is preserved.
    pub preserves_layout_slot: bool,
    /// True when logical identity remains visible.
    pub preserves_logical_identity: bool,
    /// Context preserved while the family is missing or quarantined.
    pub preserved_context: Vec<PreservedContextClass>,
    /// Capabilities that stay blocked until the family recovers.
    pub blocked_capabilities: Vec<BlockedCapabilityClass>,
    /// Safe next actions exposed on the placeholder.
    pub actions: Vec<PlaceholderActionClass>,
    /// Redaction-safe reviewer summary.
    pub summary: String,
}

/// One checked-in recovery-routing row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryFamilyRow {
    /// Stable family id.
    pub family_id: String,
    /// Surface family under recovery.
    pub surface_family: SurfaceFamily,
    /// State class under recovery.
    pub state_class: StateClass,
    /// Authority owner class.
    pub authority_class: AuthorityClass,
    /// Preferred recovery route.
    pub primary_recovery_route: RecoveryRoute,
    /// Narrower or later recovery routes that remain available.
    pub fallback_recovery_routes: Vec<RecoveryRoute>,
    /// Failure modes this family explicitly covers.
    pub supported_failure_modes: Vec<FailureMode>,
    /// Placeholder continuity plan used when the live family cannot reopen.
    pub placeholder_plan: PlaceholderContinuityPlan,
    /// Support-safe summary of what remains intact.
    pub intact_state_summary: String,
    /// Support-safe summary of why the chosen route is safest.
    pub safest_route_rationale: String,
    /// Contract or module refs that anchor the family.
    pub source_contract_refs: Vec<String>,
    /// Product consumers that quote the family directly.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the recovery-routing contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateClassRecoveryPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Recovery-routing rows.
    pub families: Vec<RecoveryFamilyRow>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// Fixture proving one failure mode against one family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateClassRecoveryFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Recovery family under test.
    pub expected_family_id: String,
    /// Failure mode under test.
    pub failure_mode: FailureMode,
    /// Expected primary route.
    pub expected_primary_recovery_route: RecoveryRoute,
    /// Expected placeholder kind.
    pub expected_placeholder_kind: PlaceholderKind,
    /// Expected placeholder actions.
    pub expected_actions: Vec<PlaceholderActionClass>,
    /// Expected preserved context.
    pub expected_preserved_context: Vec<PreservedContextClass>,
    /// One consumer that would quote this scenario.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "state-class recovery validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// Returns the checked-in packet this lane freezes.
pub fn seeded_state_class_recovery_packet() -> StateClassRecoveryPacket {
    StateClassRecoveryPacket {
        record_kind: STATE_CLASS_RECOVERY_PACKET_RECORD_KIND.to_owned(),
        schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
        packet_id: "state.state_class_recovery.v1".to_owned(),
        title: "State-class recovery routing and placeholder continuity".to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: STATE_CLASS_RECOVERY_DOC_REF.to_owned(),
            schema_ref: STATE_CLASS_RECOVERY_SCHEMA_REF.to_owned(),
            packet_ref: STATE_CLASS_RECOVERY_PACKET_REF.to_owned(),
            report_ref: STATE_CLASS_RECOVERY_REPORT_REF.to_owned(),
            fixture_manifest_ref: STATE_CLASS_RECOVERY_FIXTURE_MANIFEST_REF.to_owned(),
        },
        families: vec![
            RecoveryFamilyRow {
                family_id: "provider_local_draft".to_owned(),
                surface_family: SurfaceFamily::ProviderDraft,
                state_class: StateClass::DurableUserState,
                authority_class: AuthorityClass::UserAuthoredDurableTruth,
                primary_recovery_route: RecoveryRoute::RollbackToPreservedArtifact,
                fallback_recovery_routes: vec![RecoveryRoute::GuidedRepair],
                supported_failure_modes: vec![
                    FailureMode::PartialCorruption,
                    FailureMode::MissingDependency,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::RollbackReady,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::DraftEdits,
                        PreservedContextClass::LastKnownMetadata,
                        PreservedContextClass::MutationLineage,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::ManagedPublish,
                        BlockedCapabilityClass::DirectWrite,
                    ],
                    actions: vec![
                        PlaceholderActionClass::RestoreFromRollback,
                        PlaceholderActionClass::ComparePreservedArtifact,
                        PlaceholderActionClass::StartRepairFlow,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "The draft stays visible in place with its lineage and retained compare target while publish and direct writes remain blocked.".to_owned(),
                },
                intact_state_summary: "The surrounding layout, local draft text, linked review context, and publish-later lineage remain intact.".to_owned(),
                safest_route_rationale: "Rolling back to the preserved draft is safer than reset or rebuild because this family carries user-authored content that the product does not own.".to_owned(),
                source_contract_refs: vec![
                    "crates/aureline-provider/src/lib.rs".to_owned(),
                    "artifacts/state/migration_playbook_index.yaml#provider_family:linked_drafts"
                        .to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-shell/src/activity_center/deferred_publish.rs".to_owned(),
                    "crates/aureline-shell/src/m5_activity_objects/mod.rs".to_owned(),
                ],
                notes: "Provider-linked drafts keep a truthful placeholder and a preserved rollback target instead of silently reissuing managed writes.".to_owned(),
            },
            RecoveryFamilyRow {
                family_id: "request_workspace".to_owned(),
                surface_family: SurfaceFamily::RequestWorkspace,
                state_class: StateClass::WorkspaceState,
                authority_class: AuthorityClass::UserAuthoredDurableTruth,
                primary_recovery_route: RecoveryRoute::GuidedRepair,
                fallback_recovery_routes: vec![RecoveryRoute::RollbackToPreservedArtifact],
                supported_failure_modes: vec![
                    FailureMode::PartialCorruption,
                    FailureMode::MissingDependency,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::ContextOnly,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::WorkspaceChrome,
                        PreservedContextClass::MutationLineage,
                        PreservedContextClass::LastKnownMetadata,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::DirectWrite,
                        BlockedCapabilityClass::ManagedPublish,
                    ],
                    actions: vec![
                        PlaceholderActionClass::StartRepairFlow,
                        PlaceholderActionClass::ComparePreservedArtifact,
                        PlaceholderActionClass::RestoreFromRollback,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "Request panes reopen with their slot, identity, and request history intact while the broken document family waits for review.".to_owned(),
                },
                intact_state_summary: "Workspace chrome, surrounding tabs, pinned inputs, and redaction-safe request history remain open.".to_owned(),
                safest_route_rationale: "Request-workspace corruption can often be repaired or compared in place; a broader workspace reset would discard intact neighboring state for no gain.".to_owned(),
                source_contract_refs: vec![
                    "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                    "crates/aureline-shell/src/review_preview/mod.rs".to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                    "crates/aureline-shell/src/m5_activity_objects/mod.rs".to_owned(),
                ],
                notes: "Request workspace state keeps surrounding context and routes into compare-first repair rather than collapsing the whole workspace.".to_owned(),
            },
            RecoveryFamilyRow {
                family_id: "notebook_workspace".to_owned(),
                surface_family: SurfaceFamily::NotebookWorkspace,
                state_class: StateClass::WorkspaceState,
                authority_class: AuthorityClass::UserAuthoredDurableTruth,
                primary_recovery_route: RecoveryRoute::GuidedRepair,
                fallback_recovery_routes: vec![RecoveryRoute::RollbackToPreservedArtifact],
                supported_failure_modes: vec![
                    FailureMode::PartialCorruption,
                    FailureMode::MissingDependency,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::ContextOnly,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::WorkspaceChrome,
                        PreservedContextClass::MutationLineage,
                        PreservedContextClass::LastKnownMetadata,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::LiveExecution,
                        BlockedCapabilityClass::DirectWrite,
                    ],
                    actions: vec![
                        PlaceholderActionClass::StartRepairFlow,
                        PlaceholderActionClass::ComparePreservedArtifact,
                        PlaceholderActionClass::RestoreFromRollback,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "Notebook panes keep their slot, identity, and notebook context while execution and writes stay blocked behind repair or rollback.".to_owned(),
                },
                intact_state_summary: "The notebook pane, neighboring layout, and retained notebook lineage stay visible even when the structured notebook body is quarantined.".to_owned(),
                safest_route_rationale: "Notebook state is durable workspace truth; guided repair preserves metadata and attachments more safely than any destructive rebuild.".to_owned(),
                source_contract_refs: vec![
                    "crates/aureline-notebook/src/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/mod.rs".to_owned(),
                    "artifacts/state/migration_playbook_index.yaml#notebook_family:notebook_state"
                        .to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-shell/src/notebook_alpha/mod.rs".to_owned(),
                    "crates/aureline-shell/src/window_topology_restore_stable/mod.rs".to_owned(),
                ],
                notes: "Notebook placeholder continuity keeps the pane honest about blocked runtime state instead of silently dropping it from restore.".to_owned(),
            },
            RecoveryFamilyRow {
                family_id: "preview_cache".to_owned(),
                surface_family: SurfaceFamily::PreviewCache,
                state_class: StateClass::DerivedCacheState,
                authority_class: AuthorityClass::DisposableDerivedCache,
                primary_recovery_route: RecoveryRoute::RebuildAutomatically,
                fallback_recovery_routes: vec![RecoveryRoute::GuidedRepair],
                supported_failure_modes: vec![
                    FailureMode::BrokenCacheShard,
                    FailureMode::MissingDependency,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::StaleDerivedArtifact,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::LastKnownMetadata,
                        PreservedContextClass::RetainedEvidence,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::BackgroundRefresh,
                        BlockedCapabilityClass::RegenerationClaims,
                    ],
                    actions: vec![
                        PlaceholderActionClass::RetryRebuild,
                        PlaceholderActionClass::OpenWithoutArtifact,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "Preview surfaces reopen with a stale-artifact placeholder and retained evidence while rebuild reruns from authoritative source state.".to_owned(),
                },
                intact_state_summary: "The preview slot, route identity, and last-known metadata remain visible while only the broken cache shard is replaced.".to_owned(),
                safest_route_rationale: "Preview cache shards are derived state. Automatic rebuild is safer than rollback because the authoritative source remains elsewhere and user state is untouched.".to_owned(),
                source_contract_refs: vec![
                    "crates/aureline-preview/src/preview_origin/mod.rs".to_owned(),
                    "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
                    "crates/aureline-shell/src/window_topology_restore_stable/mod.rs".to_owned(),
                ],
                notes: "Broken preview cache shards narrow only the preview lane and reopen the slot with a truthful stale placeholder.".to_owned(),
            },
            RecoveryFamilyRow {
                family_id: "generated_artifacts".to_owned(),
                surface_family: SurfaceFamily::GeneratedArtifacts,
                state_class: StateClass::GeneratedArtifactState,
                authority_class: AuthorityClass::DisposableDerivedCache,
                primary_recovery_route: RecoveryRoute::RebuildAutomatically,
                fallback_recovery_routes: vec![RecoveryRoute::GuidedRepair],
                supported_failure_modes: vec![
                    FailureMode::StaleDerivedOverlay,
                    FailureMode::MissingDependency,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::StaleDerivedArtifact,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::MutationLineage,
                        PreservedContextClass::LastKnownMetadata,
                        PreservedContextClass::RetainedEvidence,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::DirectWrite,
                        BlockedCapabilityClass::RegenerationClaims,
                    ],
                    actions: vec![
                        PlaceholderActionClass::RetryRebuild,
                        PlaceholderActionClass::OpenWithoutArtifact,
                        PlaceholderActionClass::ComparePreservedArtifact,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "Generated outputs reopen as retained or stale artifacts with their lineage visible until regeneration succeeds or a repair review takes over.".to_owned(),
                },
                intact_state_summary: "Notebook outputs, exports, traces, and other generated artifacts keep their lineage, metadata, and surrounding layout even when current bytes are stale.".to_owned(),
                safest_route_rationale: "Generated artifacts derive from notebook, query, preview, or trace sources. Regeneration is the narrowest safe action and avoids rewriting durable source state.".to_owned(),
                source_contract_refs: vec![
                    "crates/aureline-notebook/src/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization/mod.rs".to_owned(),
                    "crates/aureline-data/src/database_qualification.rs".to_owned(),
                    "crates/aureline-profiler/src/integrate_profile_and_trace_artifacts_into_incident_workspaces_ai_explanations_and_support_bundles/mod.rs".to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
                    "crates/aureline-shell/src/support_center/mod.rs".to_owned(),
                ],
                notes: "Generated artifact drift is surfaced as stale or retained output, not as a silent claim that current output is still live truth.".to_owned(),
            },
            RecoveryFamilyRow {
                family_id: "sync_journal".to_owned(),
                surface_family: SurfaceFamily::SyncJournal,
                state_class: StateClass::RecoveryJournalState,
                authority_class: AuthorityClass::UserOwnedRecoveryState,
                primary_recovery_route: RecoveryRoute::GuidedRepair,
                fallback_recovery_routes: vec![RecoveryRoute::RollbackToPreservedArtifact],
                supported_failure_modes: vec![
                    FailureMode::JournalPreservedUnsavedWork,
                    FailureMode::MissingDependency,
                    FailureMode::PartialCorruption,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::RecoveredDraft,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::DraftEdits,
                        PreservedContextClass::MutationLineage,
                        PreservedContextClass::LastKnownMetadata,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::ManagedPublish,
                        BlockedCapabilityClass::DirectWrite,
                    ],
                    actions: vec![
                        PlaceholderActionClass::ComparePreservedArtifact,
                        PlaceholderActionClass::StartRepairFlow,
                        PlaceholderActionClass::RestoreFromRollback,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "Journal-backed work reopens as recovered drafts with compare-first repair rather than replaying or discarding queued mutations.".to_owned(),
                },
                intact_state_summary: "Queued sync intent, preserved unsaved work, and surrounding activity context stay reachable even when replay must stop for review.".to_owned(),
                safest_route_rationale: "Recovery journals may hold unsent or unpublished user intent. Guided repair and recovered-draft placeholders are safer than reset because they preserve user-owned work for review.".to_owned(),
                source_contract_refs: vec![
                    "artifacts/state/state_objects.yaml#state_objects[id=sync_metadata]".to_owned(),
                    "crates/aureline-shell/src/activity_center/deferred_publish.rs".to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-shell/src/activity_center/deferred_publish.rs".to_owned(),
                    "crates/aureline-shell/src/restore/mod.rs".to_owned(),
                ],
                notes: "Journal-preserved work must reopen as honest drafts or compare views; no managed action replays invisibly.".to_owned(),
            },
            RecoveryFamilyRow {
                family_id: "trust_policy".to_owned(),
                surface_family: SurfaceFamily::TrustPolicy,
                state_class: StateClass::SecurityTrustState,
                authority_class: AuthorityClass::AdminOrControlArtifact,
                primary_recovery_route: RecoveryRoute::FailClosedPrivilegedOperations,
                fallback_recovery_routes: vec![RecoveryRoute::GuidedRepair],
                supported_failure_modes: vec![
                    FailureMode::QuarantinedTrustState,
                    FailureMode::MissingDependency,
                ],
                placeholder_plan: PlaceholderContinuityPlan {
                    placeholder_kind: PlaceholderKind::PrivilegedBlocked,
                    preserves_layout_slot: true,
                    preserves_logical_identity: true,
                    preserved_context: vec![
                        PreservedContextClass::LayoutSlot,
                        PreservedContextClass::LogicalIdentity,
                        PreservedContextClass::WorkspaceChrome,
                        PreservedContextClass::LastKnownMetadata,
                    ],
                    blocked_capabilities: vec![
                        BlockedCapabilityClass::PrivilegedApply,
                        BlockedCapabilityClass::ManagedPublish,
                    ],
                    actions: vec![
                        PlaceholderActionClass::ReauthenticateManagedSurface,
                        PlaceholderActionClass::StartRepairFlow,
                        PlaceholderActionClass::ExportSupportPacket,
                        PlaceholderActionClass::RemovePlaceholder,
                    ],
                    summary: "Managed surfaces stay visible with a blocked-privileged placeholder so local editing continues while trust or policy state is repaired.".to_owned(),
                },
                intact_state_summary: "Ordinary local editing, layout context, and last-known managed metadata remain intact even while privileged routes stay blocked.".to_owned(),
                safest_route_rationale: "Corrupted trust or policy state must fail closed for privileged operations only. Blocking the entire app would overreach when local editing remains safe.".to_owned(),
                source_contract_refs: vec![
                    "artifacts/state/state_objects.yaml#state_objects[id=admin_policy_bundle]"
                        .to_owned(),
                    "artifacts/state/corruption_routing_matrix.yaml#posture_rows[id=fail_closed_for_privileged_operations]".to_owned(),
                ],
                consumer_refs: vec![
                    "crates/aureline-policy/src/lib.rs".to_owned(),
                    "crates/aureline-shell/src/policy_pack_beta/mod.rs".to_owned(),
                ],
                notes: "Trust and policy failures narrow managed capabilities in place instead of erasing surrounding context or silently widening authority.".to_owned(),
            },
        ],
        invariants: vec![
            "Each covered state class degrades independently; damage in one family does not justify a wider workspace or whole-app reset.".to_owned(),
            "Placeholder continuity preserves layout slot and logical identity when a family is missing, stale, quarantined, or under review.".to_owned(),
            "Durable user and workspace state never rebuild automatically; derived caches and generated artifacts prefer rebuild because authoritative truth lives elsewhere.".to_owned(),
            "Recovery journals preserve drafts and compare targets rather than silently replaying or discarding user-owned intent.".to_owned(),
            "Trust and policy corruption fails closed for privileged operations while keeping ordinary local editing and support export available.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixtures this lane freezes.
pub fn seeded_state_class_recovery_fixtures() -> Vec<StateClassRecoveryFixture> {
    vec![
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.provider_local_draft_partial_corruption"
                .to_owned(),
            expected_family_id: "provider_local_draft".to_owned(),
            failure_mode: FailureMode::PartialCorruption,
            expected_primary_recovery_route: RecoveryRoute::RollbackToPreservedArtifact,
            expected_placeholder_kind: PlaceholderKind::RollbackReady,
            expected_actions: vec![
                PlaceholderActionClass::RestoreFromRollback,
                PlaceholderActionClass::ComparePreservedArtifact,
                PlaceholderActionClass::StartRepairFlow,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::DraftEdits,
                PreservedContextClass::LastKnownMetadata,
                PreservedContextClass::MutationLineage,
            ],
            consumer_ref: "crates/aureline-shell/src/activity_center/deferred_publish.rs"
                .to_owned(),
            notes: "User-authored draft corruption reopens the draft in place and prefers rollback to the preserved artifact.".to_owned(),
        },
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.request_workspace_missing_dependency"
                .to_owned(),
            expected_family_id: "request_workspace".to_owned(),
            failure_mode: FailureMode::MissingDependency,
            expected_primary_recovery_route: RecoveryRoute::GuidedRepair,
            expected_placeholder_kind: PlaceholderKind::ContextOnly,
            expected_actions: vec![
                PlaceholderActionClass::StartRepairFlow,
                PlaceholderActionClass::ComparePreservedArtifact,
                PlaceholderActionClass::RestoreFromRollback,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::WorkspaceChrome,
                PreservedContextClass::MutationLineage,
                PreservedContextClass::LastKnownMetadata,
            ],
            consumer_ref: "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
            notes: "Missing managed dependencies keep request context honest and route into compare-first repair.".to_owned(),
        },
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.notebook_workspace_partial_corruption"
                .to_owned(),
            expected_family_id: "notebook_workspace".to_owned(),
            failure_mode: FailureMode::PartialCorruption,
            expected_primary_recovery_route: RecoveryRoute::GuidedRepair,
            expected_placeholder_kind: PlaceholderKind::ContextOnly,
            expected_actions: vec![
                PlaceholderActionClass::StartRepairFlow,
                PlaceholderActionClass::ComparePreservedArtifact,
                PlaceholderActionClass::RestoreFromRollback,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::WorkspaceChrome,
                PreservedContextClass::MutationLineage,
                PreservedContextClass::LastKnownMetadata,
            ],
            consumer_ref: "crates/aureline-shell/src/notebook_alpha/mod.rs".to_owned(),
            notes: "Notebook corruption preserves pane continuity and blocks only the notebook lane while repair or rollback is reviewed.".to_owned(),
        },
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.preview_cache_broken_cache_shard"
                .to_owned(),
            expected_family_id: "preview_cache".to_owned(),
            failure_mode: FailureMode::BrokenCacheShard,
            expected_primary_recovery_route: RecoveryRoute::RebuildAutomatically,
            expected_placeholder_kind: PlaceholderKind::StaleDerivedArtifact,
            expected_actions: vec![
                PlaceholderActionClass::RetryRebuild,
                PlaceholderActionClass::OpenWithoutArtifact,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::LastKnownMetadata,
                PreservedContextClass::RetainedEvidence,
            ],
            consumer_ref: "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            notes: "Broken preview cache shards rebuild in place without deleting surrounding workspace state.".to_owned(),
        },
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.generated_artifact_stale_overlay"
                .to_owned(),
            expected_family_id: "generated_artifacts".to_owned(),
            failure_mode: FailureMode::StaleDerivedOverlay,
            expected_primary_recovery_route: RecoveryRoute::RebuildAutomatically,
            expected_placeholder_kind: PlaceholderKind::StaleDerivedArtifact,
            expected_actions: vec![
                PlaceholderActionClass::RetryRebuild,
                PlaceholderActionClass::OpenWithoutArtifact,
                PlaceholderActionClass::ComparePreservedArtifact,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::MutationLineage,
                PreservedContextClass::LastKnownMetadata,
                PreservedContextClass::RetainedEvidence,
            ],
            consumer_ref: "crates/aureline-shell/src/support_center/mod.rs".to_owned(),
            notes: "Generated artifacts stay attributable to their source lineage and reopen as stale evidence until regeneration succeeds.".to_owned(),
        },
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.sync_journal_preserved_unsaved_work"
                .to_owned(),
            expected_family_id: "sync_journal".to_owned(),
            failure_mode: FailureMode::JournalPreservedUnsavedWork,
            expected_primary_recovery_route: RecoveryRoute::GuidedRepair,
            expected_placeholder_kind: PlaceholderKind::RecoveredDraft,
            expected_actions: vec![
                PlaceholderActionClass::ComparePreservedArtifact,
                PlaceholderActionClass::StartRepairFlow,
                PlaceholderActionClass::RestoreFromRollback,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::DraftEdits,
                PreservedContextClass::MutationLineage,
                PreservedContextClass::LastKnownMetadata,
            ],
            consumer_ref: "crates/aureline-shell/src/activity_center/deferred_publish.rs"
                .to_owned(),
            notes: "Journal-preserved unsaved work reopens as a recovered draft instead of replaying invisibly.".to_owned(),
        },
        StateClassRecoveryFixture {
            record_kind: STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: STATE_CLASS_RECOVERY_SCHEMA_VERSION,
            fixture_id: "fixture.state_class_recovery.trust_policy_quarantined".to_owned(),
            expected_family_id: "trust_policy".to_owned(),
            failure_mode: FailureMode::QuarantinedTrustState,
            expected_primary_recovery_route: RecoveryRoute::FailClosedPrivilegedOperations,
            expected_placeholder_kind: PlaceholderKind::PrivilegedBlocked,
            expected_actions: vec![
                PlaceholderActionClass::ReauthenticateManagedSurface,
                PlaceholderActionClass::StartRepairFlow,
                PlaceholderActionClass::ExportSupportPacket,
                PlaceholderActionClass::RemovePlaceholder,
            ],
            expected_preserved_context: vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LogicalIdentity,
                PreservedContextClass::WorkspaceChrome,
                PreservedContextClass::LastKnownMetadata,
            ],
            consumer_ref: "crates/aureline-shell/src/policy_pack_beta/mod.rs".to_owned(),
            notes: "Quarantined trust state blocks privileged paths only and keeps the managed surface visible with repair guidance.".to_owned(),
        },
    ]
}

/// Validates the checked-in packet contract.
pub fn validate_state_class_recovery_packet(
    packet: &StateClassRecoveryPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != STATE_CLASS_RECOVERY_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != STATE_CLASS_RECOVERY_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.source_contract_refs.doc_ref != STATE_CLASS_RECOVERY_DOC_REF {
        report.push(
            "packet.doc_ref",
            "doc_ref drifted from the frozen reviewer doc",
        );
    }
    if packet.source_contract_refs.schema_ref != STATE_CLASS_RECOVERY_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen JSON schema",
        );
    }
    if packet.source_contract_refs.packet_ref != STATE_CLASS_RECOVERY_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != STATE_CLASS_RECOVERY_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != STATE_CLASS_RECOVERY_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }

    let mut family_ids = BTreeSet::new();
    let mut covered_state_classes = BTreeSet::new();
    let mut covered_routes = BTreeSet::new();
    let mut covered_failure_modes = BTreeSet::new();

    for row in &packet.families {
        if !family_ids.insert(row.family_id.as_str()) {
            report.push(
                "family.id_unique",
                format!("duplicate family_id {}", row.family_id),
            );
        }
        if row.fallback_recovery_routes.is_empty() {
            report.push(
                "family.fallback_routes",
                format!(
                    "family {} must declare at least one fallback route",
                    row.family_id
                ),
            );
        }
        if row
            .fallback_recovery_routes
            .iter()
            .any(|route| *route == row.primary_recovery_route)
        {
            report.push(
                "family.fallback_routes",
                format!(
                    "family {} fallback routes must not repeat the primary route",
                    row.family_id
                ),
            );
        }
        if row.supported_failure_modes.is_empty() {
            report.push(
                "family.failure_modes",
                format!("family {} must declare failure modes", row.family_id),
            );
        }
        if row.placeholder_plan.actions.is_empty() {
            report.push(
                "family.placeholder_actions",
                format!("family {} must declare placeholder actions", row.family_id),
            );
        }
        if row.placeholder_plan.preserved_context.is_empty() {
            report.push(
                "family.placeholder_preserved_context",
                format!("family {} must preserve context", row.family_id),
            );
        }
        if !row.placeholder_plan.preserves_layout_slot
            || !row.placeholder_plan.preserves_logical_identity
        {
            report.push(
                "family.placeholder_continuity",
                format!(
                    "family {} must preserve layout slot and logical identity",
                    row.family_id
                ),
            );
        }
        if row.source_contract_refs.is_empty() {
            report.push(
                "family.source_contract_refs",
                format!("family {} must cite source contract refs", row.family_id),
            );
        }
        if row.consumer_refs.is_empty() {
            report.push(
                "family.consumer_refs",
                format!(
                    "family {} must cite at least one consumer ref",
                    row.family_id
                ),
            );
        }
        if row.intact_state_summary.trim().is_empty() {
            report.push(
                "family.intact_state_summary",
                format!("family {} must explain what remained intact", row.family_id),
            );
        }
        if row.safest_route_rationale.trim().is_empty() {
            report.push(
                "family.safest_route_rationale",
                format!(
                    "family {} must explain why the chosen route is safest",
                    row.family_id
                ),
            );
        }

        match row.state_class {
            StateClass::DurableUserState | StateClass::WorkspaceState => {
                if row.primary_recovery_route == RecoveryRoute::RebuildAutomatically {
                    report.push(
                        "family.primary_route_durable",
                        format!(
                            "family {} must not rebuild durable or workspace state automatically",
                            row.family_id
                        ),
                    );
                }
            }
            StateClass::DerivedCacheState | StateClass::GeneratedArtifactState => {
                if row.primary_recovery_route != RecoveryRoute::RebuildAutomatically {
                    report.push(
                        "family.primary_route_derived",
                        format!(
                            "family {} must rebuild derived or generated state automatically first",
                            row.family_id
                        ),
                    );
                }
            }
            StateClass::RecoveryJournalState => {
                if row.primary_recovery_route != RecoveryRoute::GuidedRepair {
                    report.push(
                        "family.primary_route_journal",
                        format!(
                            "family {} must route recovery journals through guided repair",
                            row.family_id
                        ),
                    );
                }
                if row.placeholder_plan.placeholder_kind != PlaceholderKind::RecoveredDraft {
                    report.push(
                        "family.placeholder_journal",
                        format!(
                            "family {} must reopen journal state as a recovered draft placeholder",
                            row.family_id
                        ),
                    );
                }
                if !row
                    .placeholder_plan
                    .preserved_context
                    .contains(&PreservedContextClass::DraftEdits)
                {
                    report.push(
                        "family.placeholder_journal_context",
                        format!(
                            "family {} must preserve draft edits for journal state",
                            row.family_id
                        ),
                    );
                }
            }
            StateClass::SecurityTrustState => {
                if row.primary_recovery_route != RecoveryRoute::FailClosedPrivilegedOperations {
                    report.push(
                        "family.primary_route_trust",
                        format!(
                            "family {} must fail closed for privileged operations first",
                            row.family_id
                        ),
                    );
                }
                if row.placeholder_plan.placeholder_kind != PlaceholderKind::PrivilegedBlocked {
                    report.push(
                        "family.placeholder_trust",
                        format!(
                            "family {} must use a privileged-blocked placeholder",
                            row.family_id
                        ),
                    );
                }
                if !row
                    .placeholder_plan
                    .blocked_capabilities
                    .contains(&BlockedCapabilityClass::PrivilegedApply)
                {
                    report.push(
                        "family.placeholder_trust_blocked",
                        format!(
                            "family {} must block privileged apply while trust state is degraded",
                            row.family_id
                        ),
                    );
                }
            }
        }

        if row.primary_recovery_route == RecoveryRoute::RebuildAutomatically
            && !row
                .placeholder_plan
                .actions
                .contains(&PlaceholderActionClass::RetryRebuild)
        {
            report.push(
                "family.rebuild_action",
                format!(
                    "family {} must offer retry_rebuild when rebuild is the primary route",
                    row.family_id
                ),
            );
        }
        if row.primary_recovery_route == RecoveryRoute::RollbackToPreservedArtifact
            && !row
                .placeholder_plan
                .actions
                .contains(&PlaceholderActionClass::RestoreFromRollback)
        {
            report.push(
                "family.rollback_action",
                format!(
                    "family {} must offer restore_from_rollback when rollback is the primary route",
                    row.family_id
                ),
            );
        }
        if row.primary_recovery_route == RecoveryRoute::GuidedRepair
            && !row
                .placeholder_plan
                .actions
                .contains(&PlaceholderActionClass::StartRepairFlow)
        {
            report.push(
                "family.repair_action",
                format!(
                    "family {} must offer start_repair_flow when guided repair is the primary route",
                    row.family_id
                ),
            );
        }
        if row.primary_recovery_route == RecoveryRoute::FailClosedPrivilegedOperations
            && !row
                .placeholder_plan
                .actions
                .contains(&PlaceholderActionClass::ReauthenticateManagedSurface)
        {
            report.push(
                "family.fail_closed_action",
                format!(
                    "family {} must offer reauthenticate_managed_surface when privileged work is failed closed",
                    row.family_id
                ),
            );
        }

        covered_state_classes.insert(row.state_class);
        covered_routes.insert(row.primary_recovery_route);
        for failure_mode in &row.supported_failure_modes {
            covered_failure_modes.insert(*failure_mode);
        }
    }

    for required in [
        StateClass::DurableUserState,
        StateClass::WorkspaceState,
        StateClass::DerivedCacheState,
        StateClass::GeneratedArtifactState,
        StateClass::RecoveryJournalState,
        StateClass::SecurityTrustState,
    ] {
        if !covered_state_classes.contains(&required) {
            report.push(
                "packet.covered_state_class",
                format!("packet must cover state class {}", required.as_str()),
            );
        }
    }

    for required in [
        RecoveryRoute::RebuildAutomatically,
        RecoveryRoute::GuidedRepair,
        RecoveryRoute::RollbackToPreservedArtifact,
        RecoveryRoute::FailClosedPrivilegedOperations,
    ] {
        if !covered_routes.contains(&required) {
            report.push(
                "packet.covered_route",
                format!("packet must cover recovery route {}", required.as_str()),
            );
        }
    }

    for required in [
        FailureMode::PartialCorruption,
        FailureMode::MissingDependency,
        FailureMode::StaleDerivedOverlay,
        FailureMode::BrokenCacheShard,
        FailureMode::JournalPreservedUnsavedWork,
        FailureMode::QuarantinedTrustState,
    ] {
        if !covered_failure_modes.contains(&required) {
            report.push(
                "packet.covered_failure_mode",
                format!("packet must cover failure mode {}", required.as_str()),
            );
        }
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one checked-in fixture against the frozen packet.
pub fn validate_state_class_recovery_fixture(
    packet: &StateClassRecoveryPacket,
    fixture: &StateClassRecoveryFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != STATE_CLASS_RECOVERY_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != STATE_CLASS_RECOVERY_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }

    let rows: BTreeMap<_, _> = packet
        .families
        .iter()
        .map(|row| (row.family_id.as_str(), row))
        .collect();
    let row = match rows.get(fixture.expected_family_id.as_str()) {
        Some(row) => *row,
        None => {
            report.push(
                "fixture.expected_family_id",
                format!(
                    "fixture {} references an unknown family",
                    fixture.fixture_id
                ),
            );
            if report.is_empty() {
                Ok(())
            } else {
                Err(report)
            }?;
            unreachable!()
        }
    };

    if !row.supported_failure_modes.contains(&fixture.failure_mode) {
        report.push(
            "fixture.failure_mode_supported",
            format!(
                "fixture {} uses unsupported failure mode {} for family {}",
                fixture.fixture_id,
                fixture.failure_mode.as_str(),
                row.family_id
            ),
        );
    }
    if row.primary_recovery_route != fixture.expected_primary_recovery_route {
        report.push(
            "fixture.primary_recovery_route",
            format!(
                "fixture {} drifted from family {}",
                fixture.fixture_id, row.family_id
            ),
        );
    }
    if row.placeholder_plan.placeholder_kind != fixture.expected_placeholder_kind {
        report.push(
            "fixture.placeholder_kind",
            format!(
                "fixture {} drifted from family {}",
                fixture.fixture_id, row.family_id
            ),
        );
    }
    if row.placeholder_plan.actions != fixture.expected_actions {
        report.push(
            "fixture.placeholder_actions",
            format!(
                "fixture {} drifted from family {}",
                fixture.fixture_id, row.family_id
            ),
        );
    }
    if row.placeholder_plan.preserved_context != fixture.expected_preserved_context {
        report.push(
            "fixture.placeholder_preserved_context",
            format!(
                "fixture {} drifted from family {}",
                fixture.fixture_id, row.family_id
            ),
        );
    }
    if !row
        .consumer_refs
        .iter()
        .any(|reference| reference == &fixture.consumer_ref)
    {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} cites a consumer_ref not declared by family {}",
                fixture.fixture_id, row.family_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_state_class_recovery_packet();
        validate_state_class_recovery_packet(&packet)
            .expect("seeded state-class recovery packet must validate");
    }

    #[test]
    fn seeded_fixtures_validate() {
        let packet = seeded_state_class_recovery_packet();
        for fixture in seeded_state_class_recovery_fixtures() {
            validate_state_class_recovery_fixture(&packet, &fixture).unwrap_or_else(|err| {
                panic!("fixture {} must validate: {err}", fixture.fixture_id)
            });
        }
    }
}
