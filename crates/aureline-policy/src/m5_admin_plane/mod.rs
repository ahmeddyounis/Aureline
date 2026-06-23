//! M5 admin-plane matrix: the frozen, typed contract for Aureline's local admin
//! surfaces — effective policy, policy diff, decision history, locked-state
//! explanation, retention/deletion, offboarding, procurement/verification, and
//! endpoint posture.
//!
//! Aureline's managed, self-hosted, sovereign, mirrored, and offline-capable
//! profiles must stay *locally* explainable: a user can see why a control is
//! locked, what policy or mirror source is active, what data classes exist and
//! where they live, what can be exported or deleted now versus later, who owns
//! the next step, and what packet proves current posture — all without a
//! separate vendor console. Each of those admin-plane objects already has a
//! boundary schema under `schemas/admin/` (plus a few sibling `schemas/records/`,
//! `schemas/governance/`, `schemas/release/`, and `schemas/storage/` schemas) and
//! at least one producing crate. What was missing was a single place that names
//! the admin-plane object *families*, freezes their stable identifiers, pins one
//! controlled vocabulary across them, maps each one to the proof packet that
//! keeps it current, and states the invariants every admin surface must hold.
//! This lane is that place.
//!
//! The matrix does four things:
//!
//! 1. **Names the admin-plane object families** ([`AdminSurfaceClass`]) and, for
//!    each, cites the canonical boundary schema(s) it binds, the crate(s) that
//!    already produce that truth, and the [`proof packet`](AdminSurfaceEntry::proof_packet_ref)
//!    that keeps it current — so docs, help, support, and commercial surfaces
//!    point at the same object model rather than re-expressing policy, audit,
//!    retention, or offboarding truth ad hoc.
//! 2. **Freezes one state vocabulary** ([`AdminStateClass`]) spanning policy
//!    source state, locked/inherited/overridden values, stale-evidence
//!    downgrades, verification/signature posture, delete/export state, mirror
//!    offline continuity, and boundary recheck.
//! 3. **Defines the controlled vocabulary** ([`ControlledVocabulary`]) the spec
//!    requires: policy source state, verification/signature posture, delete/export
//!    state, managed-copy versus local-only data classes, and owner/escalation
//!    semantics. Each surface declares which of those vocabularies it binds.
//! 4. **Covers every admin path** ([`AdminPathClass`]): local-individual, managed
//!    cloud, self-hosted, sovereign/air-gapped, mirrored/offline, and imported
//!    snapshot, with the write posture and boundary-recheck rule each carries.
//!
//! [`admin_plane_matrix`] is the canonical binding: it builds the matrix
//! deterministically and computes each [`AdminMatrixInvariant`]'s `holds` flag
//! from the built data, so the checked-in fixture and the freeze gate freeze the
//! contract byte-for-byte and an inconsistent edit flips an invariant and fails
//! CI. In particular [`AdminMatrixInvariant`] `admin_plane.proof_packet_mapped`
//! flips false the moment a claimed admin surface lacks a mapped proof packet, so
//! stable promotion cannot harden an admin claim without current proof. The
//! record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, or absolute paths — only opaque object refs, stable tokens, and
//! short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the admin-plane matrix.
pub const M5_ADMIN_PLANE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the admin-plane matrix.
pub const M5_ADMIN_PLANE_SCHEMA_REF: &str = "schemas/admin/m5-admin-plane.schema.json";

/// Stable record-kind tag for the admin-plane matrix.
pub const M5_ADMIN_PLANE_RECORD_KIND: &str = "m5_admin_plane_matrix";

/// Stable id for the canonical admin-plane matrix.
pub const M5_ADMIN_PLANE_MATRIX_ID: &str = "m5-admin-plane:matrix:0001";

/// Evaluation stamp for the canonical matrix. Held as a constant so the
/// canonical binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ADMIN_PLANE_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the matrix binding current. Stable promotion runs
/// this gate; it fails when the in-code matrix drifts from the checked-in fixture
/// or any invariant flips.
pub const M5_ADMIN_PLANE_FREEZE_GATE_REF: &str = "crates/aureline-policy/tests/m5_admin_plane.rs";

// ---------------------------------------------------------------------------
// Admin-plane object families.
// ---------------------------------------------------------------------------

/// The closed set of admin-plane object families this matrix freezes.
///
/// Each family is one governed admin surface. Adding a family is a breaking
/// change to the matrix; renaming one breaks every consumer that resolves a
/// surface by token, so the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminSurfaceClass {
    /// The effective-policy view: the resolved value of each control, its active
    /// source, and whether it is locked, inherited, or locally overridden.
    EffectivePolicyView,
    /// A policy diff: what a pending or applied policy change moves, with its
    /// blast radius over the effective values.
    PolicyDiff,
    /// The decision-history timeline / audit-event explorer: an ordered,
    /// filterable record of policy, trust, entitlement, and delete/export events.
    DecisionHistoryTimeline,
    /// A locked-state explanation: why a specific control is locked, which policy
    /// source locks it, and who can change or escalate it.
    LockedStateExplanation,
    /// The retention/deletion matrix: each record class, its hold/delete/export
    /// outcome, where its copies live, and the receipt that proves destruction.
    RetentionDeletionMatrix,
    /// The offboarding wizard: the ordered local-safe export, deletion, and
    /// continuity steps for seat loss, deprovision, or org switch.
    OffboardingWizard,
    /// A procurement / verification packet: the metadata-safe posture proof a
    /// buyer or auditor needs, with signature and validity-window truth.
    ProcurementVerificationPacket,
    /// An endpoint-posture card: the enrolled device/install posture, its check
    /// freshness, and its managed-versus-local data footprint.
    EndpointPostureCard,
}

impl AdminSurfaceClass {
    /// All surface families, in matrix order.
    pub const ALL: [Self; 8] = [
        Self::EffectivePolicyView,
        Self::PolicyDiff,
        Self::DecisionHistoryTimeline,
        Self::LockedStateExplanation,
        Self::RetentionDeletionMatrix,
        Self::OffboardingWizard,
        Self::ProcurementVerificationPacket,
        Self::EndpointPostureCard,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectivePolicyView => "effective_policy_view",
            Self::PolicyDiff => "policy_diff",
            Self::DecisionHistoryTimeline => "decision_history_timeline",
            Self::LockedStateExplanation => "locked_state_explanation",
            Self::RetentionDeletionMatrix => "retention_deletion_matrix",
            Self::OffboardingWizard => "offboarding_wizard",
            Self::ProcurementVerificationPacket => "procurement_verification_packet",
            Self::EndpointPostureCard => "endpoint_posture_card",
        }
    }

    /// Stable surface id, namespaced so it is unique across the product.
    pub fn surface_id(self) -> String {
        format!("admin_surface.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EffectivePolicyView => "Effective policy view",
            Self::PolicyDiff => "Policy diff",
            Self::DecisionHistoryTimeline => "Decision-history timeline / audit explorer",
            Self::LockedStateExplanation => "Locked-state explanation",
            Self::RetentionDeletionMatrix => "Retention / deletion matrix",
            Self::OffboardingWizard => "Offboarding wizard",
            Self::ProcurementVerificationPacket => "Procurement / verification packet",
            Self::EndpointPostureCard => "Endpoint-posture card",
        }
    }
}

// ---------------------------------------------------------------------------
// Unified state vocabulary.
// ---------------------------------------------------------------------------

/// One shared state vocabulary spanning every admin-plane surface.
///
/// The tokens are the union of the per-surface state enums already frozen under
/// `schemas/admin/` and its sibling record/governance/release schemas; each
/// [`AdminStateTerm`] in the matrix cites the upstream enum tokens it derives
/// from, so this vocabulary never silently diverges from the surfaces it
/// summarizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminStateClass {
    /// The value/control is active and enforced from a confirmed-fresh source.
    ActiveEnforced,
    /// The control is locked; the surface names the policy source and reason and
    /// who can change or escalate it.
    LockedByPolicy,
    /// The value comes from an inherited / managed default, not a local override.
    InheritedDefault,
    /// The value is locally overridden where the active policy allows it.
    OverriddenLocal,
    /// A would-be-current value whose backing policy/audit evidence is stale,
    /// partial, or cached: the no-silent-green downgrade.
    UnconfirmedStale,
    /// Awaiting a managed/mirror sync; the last-known value is labeled, not shown
    /// as current.
    PendingManagedSync,
    /// Verification or signature could not be confirmed; managed claims are held
    /// rather than presented as verified.
    SignatureUnverified,
    /// A delete or export is requested and queued.
    DeletePending,
    /// A delete is blocked by a legal/retention hold and says so.
    DeleteBlockedByHold,
    /// A delete completed with a destruction receipt.
    DeleteReceipted,
    /// Data is exportable locally now.
    ExportAvailableNow,
    /// Export is deferred / queued to publish or reauthorize later, never lost.
    ExportDeferred,
    /// The managed source is offline; a last-known-good snapshot is shown
    /// read-only and labeled.
    MirrorOfflineLastKnown,
    /// A residency/tenant/key/endpoint boundary changed or is unknown and requires
    /// explicit recheck before managed writes resume.
    BoundaryChangedRecheckRequired,
    /// Imported/replay evidence with no live target: read-only.
    ImportedSnapshotNoLive,
    /// State could not be determined and requires user review.
    UnknownRequiresReview,
}

impl AdminStateClass {
    /// All states, in vocabulary order.
    pub const ALL: [Self; 16] = [
        Self::ActiveEnforced,
        Self::LockedByPolicy,
        Self::InheritedDefault,
        Self::OverriddenLocal,
        Self::UnconfirmedStale,
        Self::PendingManagedSync,
        Self::SignatureUnverified,
        Self::DeletePending,
        Self::DeleteBlockedByHold,
        Self::DeleteReceipted,
        Self::ExportAvailableNow,
        Self::ExportDeferred,
        Self::MirrorOfflineLastKnown,
        Self::BoundaryChangedRecheckRequired,
        Self::ImportedSnapshotNoLive,
        Self::UnknownRequiresReview,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveEnforced => "active_enforced",
            Self::LockedByPolicy => "locked_by_policy",
            Self::InheritedDefault => "inherited_default",
            Self::OverriddenLocal => "overridden_local",
            Self::UnconfirmedStale => "unconfirmed_stale",
            Self::PendingManagedSync => "pending_managed_sync",
            Self::SignatureUnverified => "signature_unverified",
            Self::DeletePending => "delete_pending",
            Self::DeleteBlockedByHold => "delete_blocked_by_hold",
            Self::DeleteReceipted => "delete_receipted",
            Self::ExportAvailableNow => "export_available_now",
            Self::ExportDeferred => "export_deferred",
            Self::MirrorOfflineLastKnown => "mirror_offline_last_known",
            Self::BoundaryChangedRecheckRequired => "boundary_changed_recheck_required",
            Self::ImportedSnapshotNoLive => "imported_snapshot_no_live",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveEnforced => "Active and enforced",
            Self::LockedByPolicy => "Locked by policy",
            Self::InheritedDefault => "Inherited default",
            Self::OverriddenLocal => "Overridden locally",
            Self::UnconfirmedStale => "Unconfirmed (green downgraded)",
            Self::PendingManagedSync => "Pending managed sync",
            Self::SignatureUnverified => "Signature unverified",
            Self::DeletePending => "Delete pending",
            Self::DeleteBlockedByHold => "Delete blocked by hold",
            Self::DeleteReceipted => "Delete receipted",
            Self::ExportAvailableNow => "Export available now",
            Self::ExportDeferred => "Export deferred",
            Self::MirrorOfflineLastKnown => "Mirror offline — last known good",
            Self::BoundaryChangedRecheckRequired => "Boundary changed — recheck required",
            Self::ImportedSnapshotNoLive => "Imported snapshot — no live target",
            Self::UnknownRequiresReview => "Unknown — requires review",
        }
    }

    /// Whether this state blocks new managed/side-effectful admin actions by
    /// default.
    pub const fn blocking_default(self) -> bool {
        matches!(
            self,
            Self::LockedByPolicy
                | Self::SignatureUnverified
                | Self::DeleteBlockedByHold
                | Self::BoundaryChangedRecheckRequired
        )
    }

    /// Whether this state is a stale/partial/cached downgrade of a would-be-green
    /// headline (the no-silent-green class).
    pub const fn is_stale_downgrade(self) -> bool {
        matches!(self, Self::UnconfirmedStale)
    }

    /// The upstream schema enum tokens this state derives from, for provenance.
    fn derived_from_refs(self) -> Vec<String> {
        let refs: &[&str] = match self {
            Self::ActiveEnforced => &[
                "schemas/admin/effective_policy_card.schema.json#effective_source",
                "schemas/admin/effective_policy_card.schema.json#local_policy_resolver",
            ],
            Self::LockedByPolicy => &[
                "schemas/admin/effective_policy_card.schema.json#lock_state.capability_locked",
                "schemas/admin/effective_policy_card.schema.json#lock_reason",
            ],
            Self::InheritedDefault => &[
                "schemas/admin/effective_policy_card.schema.json#baseline_source",
                "schemas/admin/effective_policy_card.schema.json#inheritance_summary",
            ],
            Self::OverriddenLocal => &[
                "schemas/admin/effective_policy_card.schema.json#local_derived_explanation",
                "schemas/admin/effective_policy_card.schema.json#local_only_continue",
            ],
            Self::UnconfirmedStale => &[
                "schemas/admin/effective_policy_card.schema.json#managed_workspace_stale",
                "schemas/admin/effective_policy_card.schema.json#customer_managed_mirror_stale",
            ],
            Self::PendingManagedSync => &[
                "schemas/admin/effective_policy_card.schema.json#managed_sign_in_required",
                "schemas/admin/effective_policy_card.schema.json#managed_workspace_offline",
            ],
            Self::SignatureUnverified => &[
                "schemas/release/offline_verification_packet.schema.json#validity_window_class.unverified",
                "schemas/release/offline_verification_packet.schema.json#stale_past_validity",
            ],
            Self::DeletePending => &[
                "schemas/records/record-class-registry.schema.json#hold_phase",
                "schemas/governance/records_export_delete_lifecycle.schema.json#export_id",
            ],
            Self::DeleteBlockedByHold => &[
                "schemas/records/record-class-registry.schema.json#hold_status.blocked_by_hold",
                "schemas/records/record-class-registry.schema.json#active_hold_refs",
            ],
            Self::DeleteReceipted => &[
                "schemas/records/record-class-registry.schema.json#destruction_receipt",
                "schemas/records/record-class-registry.schema.json#total_destroyed_count",
            ],
            Self::ExportAvailableNow => &[
                "schemas/admin/deprovision_handoff.schema.json#export_posture_class.local_export_available",
                "schemas/admin/deprovision_handoff.schema.json#export_user_owned_artifacts",
            ],
            Self::ExportDeferred => &[
                "schemas/admin/deprovision_handoff.schema.json#export_posture_class.export_available_after_reauth",
                "schemas/admin/deprovision_handoff.schema.json#export_blocked_by_policy_or_hold",
            ],
            Self::MirrorOfflineLastKnown => &[
                "schemas/admin/effective_policy_card.schema.json#managed_workspace_offline",
                "schemas/release/offline_verification_packet.schema.json#mirror_offline_review",
            ],
            Self::BoundaryChangedRecheckRequired => &[
                "schemas/admin/device_rebind_event.schema.json#device_rebind",
                "schemas/release/offline_verification_packet.schema.json#stale_past_validity_blocked",
            ],
            Self::ImportedSnapshotNoLive => &[
                "schemas/admin/effective_policy_card.schema.json#manual_snapshot_stale",
                "schemas/release/offline_verification_packet.schema.json#offline_bundle_reimport",
            ],
            Self::UnknownRequiresReview => &[
                "schemas/release/offline_verification_packet.schema.json#validity_window_class_unresolved",
            ],
        };
        refs.iter().map(|r| (*r).to_owned()).collect()
    }
}

// ---------------------------------------------------------------------------
// Admin paths.
// ---------------------------------------------------------------------------

/// The deployment/connectivity paths an admin surface must stay explainable on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminPathClass {
    /// Local-first individual install with no managed control plane.
    LocalIndividual,
    /// Managed cloud / control-plane-backed profile.
    ManagedCloud,
    /// Self-hosted control plane operated by the customer.
    SelfHosted,
    /// Sovereign / air-gapped install with no outbound control plane.
    SovereignAirGapped,
    /// Mirror-backed offline: last-synced read-only view of managed truth.
    MirroredOffline,
    /// Imported snapshot: replayed admin evidence with no live target.
    ImportedSnapshot,
}

impl AdminPathClass {
    /// All paths, in matrix order.
    pub const ALL: [Self; 6] = [
        Self::LocalIndividual,
        Self::ManagedCloud,
        Self::SelfHosted,
        Self::SovereignAirGapped,
        Self::MirroredOffline,
        Self::ImportedSnapshot,
    ];

    /// Stable snake_case token for this path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalIndividual => "local_individual",
            Self::ManagedCloud => "managed_cloud",
            Self::SelfHosted => "self_hosted",
            Self::SovereignAirGapped => "sovereign_air_gapped",
            Self::MirroredOffline => "mirrored_offline",
            Self::ImportedSnapshot => "imported_snapshot",
        }
    }

    /// Stable path id, namespaced for uniqueness.
    pub fn path_id(self) -> String {
        format!("admin_path.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalIndividual => "Local individual",
            Self::ManagedCloud => "Managed cloud",
            Self::SelfHosted => "Self-hosted",
            Self::SovereignAirGapped => "Sovereign / air-gapped",
            Self::MirroredOffline => "Mirrored / offline",
            Self::ImportedSnapshot => "Imported snapshot",
        }
    }
}

// ---------------------------------------------------------------------------
// Controlled vocabulary axes.
// ---------------------------------------------------------------------------

/// The named controlled-vocabulary axes this matrix defines and each surface
/// declares it binds.
///
/// These are exactly the vocabularies the contract requires: policy source
/// state, verification/signature posture, delete/export state, managed-copy
/// versus local-only data classes, and owner/escalation semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlledVocabulary {
    /// Where the active policy/value comes from and that source's state.
    PolicySourceState,
    /// Whether a managed claim is signed, verified, expired, revoked, or
    /// unverifiable offline.
    VerificationSignaturePosture,
    /// Whether a record can be exported or deleted now, later, or not at all.
    DeleteExportState,
    /// Where a data class lives: local-only, managed copy, mirrored, or exported.
    DataResidencyClass,
    /// Who owns a control or step and who it escalates to.
    OwnerEscalation,
}

impl ControlledVocabulary {
    /// All controlled-vocabulary axes, in order.
    pub const ALL: [Self; 5] = [
        Self::PolicySourceState,
        Self::VerificationSignaturePosture,
        Self::DeleteExportState,
        Self::DataResidencyClass,
        Self::OwnerEscalation,
    ];

    /// Stable snake_case token for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicySourceState => "policy_source_state",
            Self::VerificationSignaturePosture => "verification_signature_posture",
            Self::DeleteExportState => "delete_export_state",
            Self::DataResidencyClass => "data_residency_class",
            Self::OwnerEscalation => "owner_escalation",
        }
    }
}

// ---------------------------------------------------------------------------
// Shared, reused token vocabularies.
// ---------------------------------------------------------------------------

/// Deployment profile, mirroring the deployment classes used across the admin
/// schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminDeploymentProfileClass {
    /// Individual, local-first install.
    IndividualLocal,
    /// Self-hosted control plane.
    SelfHosted,
    /// Enterprise online / managed cloud.
    EnterpriseOnline,
    /// Sovereign / air-gapped, offline-only.
    SovereignAirGapped,
    /// Managed cloud.
    ManagedCloud,
}

/// Default redaction posture on export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminRedactionClass {
    /// Metadata-safe default — the export default for admin surfaces.
    MetadataSafeDefault,
    /// Restricted to admins.
    AdminOnlyRestricted,
    /// Restricted to internal support.
    InternalSupportRestricted,
    /// Signing/verification evidence only.
    SigningEvidenceOnly,
    /// Compliance-restricted (legal hold / retention review).
    ComplianceRestricted,
}

/// Local-versus-shared scope of a surface's underlying objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminScopeClass {
    /// Local and private to this user/host.
    LocalPrivate,
    /// Shared across a workspace/team.
    SharedWorkspace,
    /// Defined and governed at the managed-org / control-plane level.
    ManagedOrg,
}

/// Whether a surface is live, can be snapshotted, or is snapshot-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminLiveSnapshotClass {
    /// Always live; never persisted as a frozen snapshot.
    LiveOnly,
    /// Live when connected, captured as a labeled snapshot on export/handoff.
    SnapshotCapable,
    /// Snapshot-only: imported/replay evidence with no live refresh path.
    SnapshotOnly,
}

/// The consumers that render an admin surface instead of restating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminConsumerClass {
    /// Desktop shell UI / settings & admin center.
    ShellAdminCenter,
    /// CLI / headless inspect.
    CliHeadless,
    /// Help / About truth surface.
    HelpAbout,
    /// Support export / bundle.
    SupportExport,
    /// Commercial / procurement surface.
    CommercialProcurement,
    /// Release evidence / shiproom.
    ReleaseEvidence,
    /// Managed-service / control-plane consumer.
    ManagedService,
}

/// The write posture a path admits for side-effectful admin actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminPathWritePostureClass {
    /// Connected: managed writes run live (still subject to per-action approval).
    WritesLive,
    /// Writes are captured and queued to publish later.
    PublishLaterQueued,
    /// Writes are preserved as a local draft only.
    LocalDraftPreserved,
    /// Read-only replay of imported evidence; no writes admitted.
    ReadOnlyReplay,
    /// Writes are blocked pending a boundary recheck.
    BlockedPendingBoundaryRecheck,
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One `(token, label)` definition in the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminTokenDef {
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
}

/// The reused token vocabularies and the source schemas this matrix binds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminSharedVocabulary {
    /// Deployment profiles.
    pub deployment_profiles: Vec<AdminTokenDef>,
    /// Redaction classes.
    pub redaction_classes: Vec<AdminTokenDef>,
    /// Policy source states (`policy_source_state` controlled vocabulary).
    pub policy_source_states: Vec<AdminTokenDef>,
    /// Verification / signature postures (`verification_signature_posture`).
    pub verification_postures: Vec<AdminTokenDef>,
    /// Delete / export states (`delete_export_state`).
    pub delete_export_states: Vec<AdminTokenDef>,
    /// Managed-copy versus local-only data classes (`data_residency_class`).
    pub data_residency_classes: Vec<AdminTokenDef>,
    /// Owner / escalation roles (`owner_escalation`).
    pub owner_escalation_roles: Vec<AdminTokenDef>,
    /// Scope classes.
    pub scope_classes: Vec<AdminTokenDef>,
    /// Live-versus-snapshot classes.
    pub live_snapshot_classes: Vec<AdminTokenDef>,
    /// Consumer classes.
    pub consumer_classes: Vec<AdminTokenDef>,
    /// Boundary axes.
    pub boundary_axes: Vec<AdminTokenDef>,
    /// The boundary schemas this matrix binds as truth sources.
    pub source_schema_refs: Vec<String>,
}

/// One state in the unified vocabulary, with provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminStateTerm {
    /// The state.
    pub state: AdminStateClass,
    /// Stable token (equals `state.as_str()`), surfaced for reuse by consumers.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// Whether this state blocks new managed/side-effectful actions by default.
    pub blocking_default: bool,
    /// Whether this state is the stale/partial/cached green downgrade.
    pub stale_downgrade: bool,
    /// The upstream schema enum tokens this state derives from.
    pub derived_from_refs: Vec<String>,
}

/// One ownership/decision-right field a surface must carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminOwnershipField {
    /// Stable field id.
    pub field_id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the field is required on every row of the surface.
    pub required: bool,
}

/// The freshness rule a surface applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminFreshnessRule {
    /// The age tokens the surface uses, oldest path last.
    pub age_tokens: Vec<String>,
    /// Whether a stale/partial age downgrades a would-be-green headline
    /// (the no-silent-green rule).
    pub downgrades_green: bool,
    /// One reviewable sentence stating the rule.
    pub rule: String,
}

/// One admin-plane surface-family entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminSurfaceEntry {
    /// The surface family.
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id.
    pub surface_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the surface.
    pub summary: String,
    /// The canonical boundary schema(s) this surface binds.
    pub canonical_schema_refs: Vec<String>,
    /// The crate module(s) that already produce this truth.
    pub produced_by_refs: Vec<String>,
    /// The proof packet (contract, fixture, or evidence) that keeps this surface
    /// current. Stable promotion fails when this is empty.
    pub proof_packet_ref: String,
    /// The consumers that render this surface.
    pub consumed_by: Vec<AdminConsumerClass>,
    /// The states from the unified vocabulary this surface can show.
    pub applicable_states: Vec<AdminStateClass>,
    /// The controlled-vocabulary axes this surface binds.
    pub controlled_vocabularies: Vec<ControlledVocabulary>,
    /// The ownership/decision-right fields this surface carries.
    pub ownership_fields: Vec<AdminOwnershipField>,
    /// The freshness rule this surface applies.
    pub freshness_rule: AdminFreshnessRule,
    /// The default redaction posture on export.
    pub default_redaction: AdminRedactionClass,
    /// Local-versus-shared scope of the underlying objects.
    pub scope: AdminScopeClass,
    /// Live-versus-snapshot posture.
    pub live_vs_snapshot: AdminLiveSnapshotClass,
    /// Whether this surface captures user writes (requests, drafts, choices).
    pub captures_user_writes: bool,
    /// The local-safe actions that stay available offline / on a mirror.
    pub local_safe_actions: Vec<String>,
    /// Whether publish-later / draft capture is offered when writes are blocked.
    pub publish_later_capture: bool,
    /// Whether the surface is locally explainable (never portal-only / console-
    /// only) and states a short honesty rule.
    pub locally_explainable: bool,
    /// One reviewable sentence stating the surface's local-explainability /
    /// boundary-honesty rule.
    pub boundary_note: String,
    /// Whether the surface is typed (never screenshot-only / generic prose).
    pub typed_not_portal_only: bool,
}

impl AdminSurfaceEntry {
    /// Whether the surface binds the named controlled-vocabulary axis.
    pub fn binds(&self, vocab: ControlledVocabulary) -> bool {
        self.controlled_vocabularies.contains(&vocab)
    }

    /// Whether the surface can show a given state.
    pub fn can_show(&self, state: AdminStateClass) -> bool {
        self.applicable_states.contains(&state)
    }
}

/// One admin-path entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminPathEntry {
    /// The path.
    pub path: AdminPathClass,
    /// Stable, namespaced path id.
    pub path_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the path.
    pub summary: String,
    /// The deployment profiles that map to this path.
    pub deployment_profiles: Vec<AdminDeploymentProfileClass>,
    /// The default live-versus-snapshot posture on this path.
    pub default_live_vs_snapshot: AdminLiveSnapshotClass,
    /// The write posture this path admits.
    pub write_posture: AdminPathWritePostureClass,
    /// Whether managed writes require a boundary recheck on this path.
    pub boundary_recheck_required: bool,
    /// The local-safe baseline this path leans on for offline explainability.
    pub local_safe_baseline_ref: String,
    /// One reviewable sentence of path-specific notes.
    pub notes: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminMatrixInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built matrix satisfies the invariant.
    pub holds: bool,
}

/// The frozen admin-plane matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminPlaneMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_admin_plane_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the matrix binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the matrix.
    pub summary: String,
    /// The reused token vocabularies and bound source schemas.
    pub shared_vocabulary: AdminSharedVocabulary,
    /// The unified state vocabulary.
    pub state_vocabulary: Vec<AdminStateTerm>,
    /// The surface-family entries.
    pub surfaces: Vec<AdminSurfaceEntry>,
    /// The admin-path entries.
    pub admin_paths: Vec<AdminPathEntry>,
    /// The computed invariants.
    pub invariants: Vec<AdminMatrixInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the matrix fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminMatrixValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for AdminMatrixValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "admin-plane matrix invalid: {}", self.reason)
    }
}

impl std::error::Error for AdminMatrixValidationError {}

impl AdminPlaneMatrix {
    /// Returns the entry for a surface family, if present.
    pub fn surface(&self, surface: AdminSurfaceClass) -> Option<&AdminSurfaceEntry> {
        self.surfaces.iter().find(|s| s.surface == surface)
    }

    /// Returns the entry for an admin path, if present.
    pub fn path(&self, path: AdminPathClass) -> Option<&AdminPathEntry> {
        self.admin_paths.iter().find(|p| p.path == path)
    }

    /// Returns the state term for a state, if present.
    pub fn state_term(&self, state: AdminStateClass) -> Option<&AdminStateTerm> {
        self.state_vocabulary.iter().find(|t| t.state == state)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    /// Every ref string carried by the matrix, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_shared = self
            .shared_vocabulary
            .source_schema_refs
            .iter()
            .map(String::as_str);
        let from_states = self
            .state_vocabulary
            .iter()
            .flat_map(|t| t.derived_from_refs.iter().map(String::as_str));
        let from_surfaces = self.surfaces.iter().flat_map(|s| {
            s.canonical_schema_refs
                .iter()
                .map(String::as_str)
                .chain(s.produced_by_refs.iter().map(String::as_str))
                .chain(std::iter::once(s.proof_packet_ref.as_str()))
        });
        let from_paths = self
            .admin_paths
            .iter()
            .map(|p| p.local_safe_baseline_ref.as_str());
        let from_gate = std::iter::once(self.freeze_gate_ref.as_str());
        from_shared
            .chain(from_states)
            .chain(from_surfaces)
            .chain(from_paths)
            .chain(from_gate)
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`AdminMatrixInvariant`]s with the
    /// uniqueness and completeness checks a consumer relies on.
    pub fn validate(&self) -> Result<(), AdminMatrixValidationError> {
        let fail = |reason: String| Err(AdminMatrixValidationError { reason });

        if self.record_kind != M5_ADMIN_PLANE_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ADMIN_PLANE_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every family and every path is present exactly once.
        for surface in AdminSurfaceClass::ALL {
            if self
                .surfaces
                .iter()
                .filter(|s| s.surface == surface)
                .count()
                != 1
            {
                return fail(format!(
                    "surface {} not present exactly once",
                    surface.as_str()
                ));
            }
        }
        for path in AdminPathClass::ALL {
            if self.admin_paths.iter().filter(|p| p.path == path).count() != 1 {
                return fail(format!("path {} not present exactly once", path.as_str()));
            }
        }
        for state in AdminStateClass::ALL {
            if self
                .state_vocabulary
                .iter()
                .filter(|t| t.state == state)
                .count()
                != 1
            {
                return fail(format!("state {} not present exactly once", state.as_str()));
            }
        }

        // Stable ids are unique.
        if !all_unique(self.surfaces.iter().map(|s| s.surface_id.as_str())) {
            return fail("surface ids are not unique".to_owned());
        }
        if !all_unique(self.admin_paths.iter().map(|p| p.path_id.as_str())) {
            return fail("path ids are not unique".to_owned());
        }
        if !all_unique(self.state_vocabulary.iter().map(|t| t.token.as_str())) {
            return fail("state tokens are not unique".to_owned());
        }

        // Per-surface structural floor: typed, evidenced, owned, fresh, proven.
        for entry in &self.surfaces {
            if entry.surface_id != entry.surface.surface_id() {
                return fail(format!(
                    "surface id mismatch for {}",
                    entry.surface.as_str()
                ));
            }
            if entry.canonical_schema_refs.is_empty() {
                return fail(format!(
                    "surface {} cites no schema",
                    entry.surface.as_str()
                ));
            }
            if entry.produced_by_refs.is_empty() {
                return fail(format!(
                    "surface {} has no producer",
                    entry.surface.as_str()
                ));
            }
            if entry.proof_packet_ref.is_empty() {
                return fail(format!(
                    "surface {} has no mapped proof packet",
                    entry.surface.as_str()
                ));
            }
            if entry.applicable_states.is_empty() {
                return fail(format!(
                    "surface {} declares no states",
                    entry.surface.as_str()
                ));
            }
            if entry.controlled_vocabularies.is_empty() {
                return fail(format!(
                    "surface {} binds no controlled vocabulary",
                    entry.surface.as_str()
                ));
            }
            if entry.ownership_fields.is_empty() {
                return fail(format!(
                    "surface {} declares no ownership",
                    entry.surface.as_str()
                ));
            }
            if entry.freshness_rule.age_tokens.is_empty() {
                return fail(format!(
                    "surface {} has no freshness rule",
                    entry.surface.as_str()
                ));
            }
            // Every applicable state is a defined vocabulary term.
            for state in &entry.applicable_states {
                if self.state_term(*state).is_none() {
                    return fail(format!(
                        "surface {} references undefined state {}",
                        entry.surface.as_str(),
                        state.as_str()
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("matrix is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical admin-plane matrix.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the built surfaces, paths, and states, so an inconsistent edit
/// flips an invariant rather than silently passing.
pub fn admin_plane_matrix() -> AdminPlaneMatrix {
    let state_vocabulary = build_state_vocabulary();
    let surfaces = build_surfaces();
    let admin_paths = build_paths();
    let shared_vocabulary = build_shared_vocabulary(&surfaces);
    let invariants = compute_invariants(&surfaces, &admin_paths, &state_vocabulary);

    AdminPlaneMatrix {
        record_kind: M5_ADMIN_PLANE_RECORD_KIND.to_owned(),
        m5_admin_plane_schema_version: M5_ADMIN_PLANE_SCHEMA_VERSION,
        schema_ref: M5_ADMIN_PLANE_SCHEMA_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        as_of: M5_ADMIN_PLANE_AS_OF.to_owned(),
        freeze_gate_ref: M5_ADMIN_PLANE_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed matrix for Aureline's local admin plane — effective policy, \
                  policy diff, decision history, locked-state explanation, retention/deletion, \
                  offboarding, procurement/verification, and endpoint posture — across local, \
                  managed, self-hosted, sovereign/air-gapped, mirrored/offline, and \
                  imported-snapshot paths, with each object mapped to the proof packet that keeps \
                  it current."
            .to_owned(),
        shared_vocabulary,
        state_vocabulary,
        surfaces,
        admin_paths,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_state_vocabulary() -> Vec<AdminStateTerm> {
    AdminStateClass::ALL
        .iter()
        .map(|state| AdminStateTerm {
            state: *state,
            token: state.as_str().to_owned(),
            label: state.label().to_owned(),
            blocking_default: state.blocking_default(),
            stale_downgrade: state.is_stale_downgrade(),
            derived_from_refs: state.derived_from_refs(),
        })
        .collect()
}

fn own(field_id: &str, label: &str, required: bool) -> AdminOwnershipField {
    AdminOwnershipField {
        field_id: field_id.to_owned(),
        label: label.to_owned(),
        required,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

const FRESHNESS_AGE_TOKENS: [&str; 5] = ["fresh", "recent", "stale", "very_stale", "never"];

fn freshness(downgrades_green: bool, rule: &str) -> AdminFreshnessRule {
    AdminFreshnessRule {
        age_tokens: strvec(&FRESHNESS_AGE_TOKENS),
        downgrades_green,
        rule: rule.to_owned(),
    }
}

fn build_surfaces() -> Vec<AdminSurfaceEntry> {
    use AdminConsumerClass::*;
    use AdminStateClass::*;
    use ControlledVocabulary::*;

    vec![
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::EffectivePolicyView,
        surface_id: AdminSurfaceClass::EffectivePolicyView.surface_id(),
        label: AdminSurfaceClass::EffectivePolicyView.label().to_owned(),
        summary: "The resolved value of each control with its active source and whether it is \
                  locked, inherited, or locally overridden; a stale mirror downgrades a \
                  would-be-current value rather than presenting it as confirmed."
            .to_owned(),
        canonical_schema_refs: strvec(&["schemas/admin/effective_policy_card.schema.json"]),
        produced_by_refs: strvec(&[
            "crates/aureline-policy/src/stabilize_effective_policy_remembered_decision_waiver_expiry_and/mod.rs",
            "crates/aureline-shell/src/admin_alpha/mod.rs",
        ]),
        proof_packet_ref: "docs/admin/policy_explainability_contract.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, HelpAbout, SupportExport, ManagedService],
        applicable_states: vec![
            ActiveEnforced,
            LockedByPolicy,
            InheritedDefault,
            OverriddenLocal,
            UnconfirmedStale,
            PendingManagedSync,
            MirrorOfflineLastKnown,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![PolicySourceState, OwnerEscalation, DataResidencyClass],
        ownership_fields: vec![
            own("control_id", "Control", true),
            own("effective_source", "Effective source", true),
            own("policy_owner", "Policy owner", true),
            own("escalation_path", "Escalation path", false),
            own("evidence_age", "Evidence age", true),
        ],
        freshness_rule: freshness(
            true,
            "An effective value is shown as confirmed only when its active source is fresh; a \
             stale or offline managed/mirror source downgrades it to unconfirmed and names the \
             last-known source.",
        ),
        default_redaction: AdminRedactionClass::MetadataSafeDefault,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&[
            "open_source",
            "open_locked_explanation",
            "export_policy_snapshot",
            "continue_local_only",
        ]),
        publish_later_capture: false,
        locally_explainable: true,
        boundary_note: "Every control names its active source and a locked control routes to its \
                        explanation; the view never asserts a managed value it cannot evidence \
                        locally."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::PolicyDiff,
        surface_id: AdminSurfaceClass::PolicyDiff.surface_id(),
        label: AdminSurfaceClass::PolicyDiff.label().to_owned(),
        summary: "What a pending or applied policy change moves, with its blast radius over the \
                  effective values and the controls it would newly lock, unlock, or rescope."
            .to_owned(),
        canonical_schema_refs: strvec(&["schemas/admin/effective_policy_card.schema.json"]),
        produced_by_refs: strvec(&[
            "crates/aureline-policy/src/policy_simulation_and_expiry/mod.rs",
            "crates/aureline-shell/src/policy_simulation_beta/mod.rs",
        ]),
        proof_packet_ref: "docs/admin/policy_diff_alpha.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, HelpAbout, SupportExport, ManagedService],
        applicable_states: vec![
            ActiveEnforced,
            LockedByPolicy,
            InheritedDefault,
            OverriddenLocal,
            UnconfirmedStale,
            PendingManagedSync,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![PolicySourceState, OwnerEscalation],
        ownership_fields: vec![
            own("diff_subject", "Diff subject", true),
            own("from_source", "From source", true),
            own("to_source", "To source", true),
            own("change_owner", "Change owner", true),
            own("impact_summary", "Impact summary", false),
        ],
        freshness_rule: freshness(
            true,
            "A diff is computed against the current effective values; if those are stale the diff \
             is labeled provisional rather than presented as a confirmed before/after.",
        ),
        default_redaction: AdminRedactionClass::MetadataSafeDefault,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&[
            "open_changed_control",
            "open_impact",
            "export_diff_snapshot",
        ]),
        publish_later_capture: false,
        locally_explainable: true,
        boundary_note: "The diff names both the from-source and to-source and never implies a \
                        managed change applied locally without an approval path."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::DecisionHistoryTimeline,
        surface_id: AdminSurfaceClass::DecisionHistoryTimeline.surface_id(),
        label: AdminSurfaceClass::DecisionHistoryTimeline.label().to_owned(),
        summary: "An ordered, filterable record of policy, trust, entitlement, and delete/export \
                  events that names each event's origin lane and labels offline or partial \
                  coverage rather than implying a complete history."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/admin/audit_event_record.schema.json",
            "schemas/admin/audit_event_filter.schema.json",
            "schemas/admin/effective_policy_card.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/admin_audit_export_beta/mod.rs",
            "crates/aureline-shell/src/admin_alpha/mod.rs",
        ]),
        proof_packet_ref: "docs/admin/audit_event_explorer_contract.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, SupportExport, CommercialProcurement, ManagedService],
        applicable_states: vec![
            ActiveEnforced,
            UnconfirmedStale,
            MirrorOfflineLastKnown,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![OwnerEscalation, DataResidencyClass],
        ownership_fields: vec![
            own("event_id", "Event", true),
            own("event_class", "Event class", true),
            own("actor_owner", "Actor / owner", true),
            own("filter_ref", "Filter that produced the view", false),
            own("coverage_window", "Coverage window", true),
        ],
        freshness_rule: freshness(
            true,
            "The timeline names its coverage window and the filter that produced it; gaps, offline \
             tails, and partial coverage are labeled rather than implied complete.",
        ),
        default_redaction: AdminRedactionClass::InternalSupportRestricted,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&["open_event", "filter_scope", "export_audit_snapshot"]),
        publish_later_capture: false,
        locally_explainable: true,
        boundary_note: "Each row carries event provenance and origin lane; an imported export is \
                        labeled read-only with no live refresh path."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::LockedStateExplanation,
        surface_id: AdminSurfaceClass::LockedStateExplanation.surface_id(),
        label: AdminSurfaceClass::LockedStateExplanation.label().to_owned(),
        summary: "Why a specific control is locked, which policy source locks it, the verification \
                  posture of that source, and who can change or escalate it."
            .to_owned(),
        canonical_schema_refs: strvec(&["schemas/admin/effective_policy_card.schema.json"]),
        produced_by_refs: strvec(&[
            "crates/aureline-policy/src/stabilize_effective_policy_remembered_decision_waiver_expiry_and/mod.rs",
            "crates/aureline-shell/src/admin_alpha/mod.rs",
        ]),
        proof_packet_ref: "docs/admin/policy_explainability_contract.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, HelpAbout, SupportExport],
        applicable_states: vec![
            LockedByPolicy,
            InheritedDefault,
            SignatureUnverified,
            BoundaryChangedRecheckRequired,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![
            PolicySourceState,
            VerificationSignaturePosture,
            OwnerEscalation,
        ],
        ownership_fields: vec![
            own("locked_target_ref", "Locked target", true),
            own("lock_reason", "Lock reason", true),
            own("lock_source", "Lock source", true),
            own("change_owner", "Who can change it", true),
            own("escalation_path", "Escalation path", false),
        ],
        freshness_rule: freshness(
            false,
            "A locked control always names its lock reason and source; an unverifiable lock source \
             is shown as unverified, never silently treated as authoritative.",
        ),
        default_redaction: AdminRedactionClass::MetadataSafeDefault,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&["open_source", "open_escalation", "export_explanation"]),
        publish_later_capture: false,
        locally_explainable: true,
        boundary_note: "No control is locked without a stated reason, a named source, and a \
                        change/escalation owner the user can see locally."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::RetentionDeletionMatrix,
        surface_id: AdminSurfaceClass::RetentionDeletionMatrix.surface_id(),
        label: AdminSurfaceClass::RetentionDeletionMatrix.label().to_owned(),
        summary: "Each record class, its hold/delete/export outcome, where its copies live \
                  (local-only versus managed/mirrored), and the destruction receipt that proves a \
                  delete actually happened."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/records/record-class-registry.schema.json",
            "schemas/governance/records_export_delete_lifecycle.schema.json",
            "schemas/governance/export_delete_request_summary.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-records/src/stabilize_record_class_registry_legal_hold_delete_honesty/mod.rs",
            "crates/aureline-records/src/export_delete_lifecycle/mod.rs",
        ]),
        proof_packet_ref: "docs/governance/record_class_governance.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, HelpAbout, SupportExport, CommercialProcurement],
        applicable_states: vec![
            ActiveEnforced,
            DeletePending,
            DeleteBlockedByHold,
            DeleteReceipted,
            ExportAvailableNow,
            ExportDeferred,
            UnconfirmedStale,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![DeleteExportState, DataResidencyClass, OwnerEscalation],
        ownership_fields: vec![
            own("record_class_id", "Record class", true),
            own("delete_export_state", "Delete / export state", true),
            own("data_residency", "Where copies live", true),
            own("retention_owner", "Retention owner", true),
            own("destruction_receipt_ref", "Destruction receipt", false),
        ],
        freshness_rule: freshness(
            true,
            "A class is never shown deleted without a destruction receipt; a hold blocks deletion \
             and says so, and a stale registry view is labeled rather than implied current.",
        ),
        default_redaction: AdminRedactionClass::ComplianceRestricted,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&[
            "open_record_class",
            "export_before_delete",
            "request_delete",
            "export_retention_snapshot",
        ]),
        publish_later_capture: true,
        locally_explainable: true,
        boundary_note: "Delete and export honesty is local: a blocked-by-hold delete names the \
                        hold and a completed delete shows its receipt; local-only artifacts are \
                        labeled distinctly from managed copies."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::OffboardingWizard,
        surface_id: AdminSurfaceClass::OffboardingWizard.surface_id(),
        label: AdminSurfaceClass::OffboardingWizard.label().to_owned(),
        summary: "The ordered local-safe export, deletion, and continuity steps for seat loss, \
                  deprovision, or org switch, with what is preserved locally and who owns the \
                  next step."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/admin/deprovision_handoff.schema.json",
            "schemas/storage/m5_offboarding_continuity.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-policy/src/finalize_open_vs_paid_boundary_and_offboarding/mod.rs",
            "crates/aureline-auth/src/finalize_no_account_local_use_proof_deprovision_preserves/mod.rs",
        ]),
        proof_packet_ref: "docs/storage/m5_offboarding_continuity_contract.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, HelpAbout, SupportExport, CommercialProcurement],
        applicable_states: vec![
            ActiveEnforced,
            DeletePending,
            DeleteBlockedByHold,
            DeleteReceipted,
            ExportAvailableNow,
            ExportDeferred,
            MirrorOfflineLastKnown,
            BoundaryChangedRecheckRequired,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![DeleteExportState, DataResidencyClass, OwnerEscalation],
        ownership_fields: vec![
            own("offboarding_step", "Offboarding step", true),
            own("export_posture", "Export posture", true),
            own("local_continuity", "Local continuity", true),
            own("step_owner", "Step owner", true),
            own("grace_window", "Grace window", false),
        ],
        freshness_rule: freshness(
            false,
            "Each step names whether export is available now, after reauth, or blocked, and what \
             stays editable locally; deprovision never silently strips local-owned artifacts.",
        ),
        default_redaction: AdminRedactionClass::MetadataSafeDefault,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&[
            "export_user_owned_artifacts",
            "continue_local_only",
            "edit_local_artifacts",
            "publish_later",
        ]),
        publish_later_capture: true,
        locally_explainable: true,
        boundary_note: "Offboarding preserves local-owned work and names the owner of each \
                        remaining step; export that needs reauth is labeled deferred, not lost."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::ProcurementVerificationPacket,
        surface_id: AdminSurfaceClass::ProcurementVerificationPacket.surface_id(),
        label: AdminSurfaceClass::ProcurementVerificationPacket.label().to_owned(),
        summary: "The metadata-safe posture proof a buyer or auditor needs — signature and \
                  validity-window truth, residual-dependency disclosure, and an offline-reviewable \
                  evidence index — without a separate vendor console."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/release/offline_verification_packet.schema.json",
            "schemas/admin/admin_audit_export.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-release/src/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support/mod.rs",
            "crates/aureline-shell/src/admin_audit_export_beta/mod.rs",
        ]),
        proof_packet_ref: "docs/admin/admin_audit_export_contract.md".to_owned(),
        consumed_by: vec![CommercialProcurement, HelpAbout, SupportExport, ReleaseEvidence, ManagedService],
        applicable_states: vec![
            ActiveEnforced,
            SignatureUnverified,
            UnconfirmedStale,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![VerificationSignaturePosture, OwnerEscalation],
        ownership_fields: vec![
            own("packet_id", "Packet", true),
            own("verification_posture", "Verification posture", true),
            own("validity_window", "Validity window", true),
            own("packet_owner", "Packet owner", true),
            own("residual_dependencies", "Residual dependencies", false),
        ],
        freshness_rule: freshness(
            true,
            "The packet states its validity window and signature posture; a packet past validity \
             or with an unverifiable signature is labeled, never presented as currently verified.",
        ),
        default_redaction: AdminRedactionClass::MetadataSafeDefault,
        scope: AdminScopeClass::SharedWorkspace,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&[
            "open_verification_evidence",
            "export_procurement_packet",
            "verify_offline",
        ]),
        publish_later_capture: false,
        locally_explainable: true,
        boundary_note: "Verification is shown as verified only with a current, signed packet; an \
                        offline or past-validity packet is labeled and verifiable locally."
            .to_owned(),
        typed_not_portal_only: true,
    },
    AdminSurfaceEntry {
        surface: AdminSurfaceClass::EndpointPostureCard,
        surface_id: AdminSurfaceClass::EndpointPostureCard.surface_id(),
        label: AdminSurfaceClass::EndpointPostureCard.label().to_owned(),
        summary: "The enrolled device/install posture, its check freshness, its enrollment and \
                  rebind lineage, and its managed-versus-local data footprint; a stale check \
                  downgrades the posture rather than showing it green."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/admin/effective_policy_card.schema.json",
            "schemas/admin/fleet_status_row.schema.json",
            "schemas/admin/device_rebind_event.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/admin_alpha/mod.rs",
            "crates/aureline-install/src/ownership_audit/mod.rs",
        ]),
        proof_packet_ref: "docs/admin/org_admin_seat_and_fleet_contract.md".to_owned(),
        consumed_by: vec![ShellAdminCenter, CliHeadless, HelpAbout, SupportExport, ManagedService],
        applicable_states: vec![
            ActiveEnforced,
            UnconfirmedStale,
            PendingManagedSync,
            SignatureUnverified,
            MirrorOfflineLastKnown,
            BoundaryChangedRecheckRequired,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![
            VerificationSignaturePosture,
            DataResidencyClass,
            OwnerEscalation,
        ],
        ownership_fields: vec![
            own("device_or_install_id", "Device / install", true),
            own("posture_state", "Posture state", true),
            own("check_age", "Check age", true),
            own("enrollment_owner", "Enrollment owner", true),
            own("data_residency", "Data footprint", false),
        ],
        freshness_rule: freshness(
            true,
            "Posture is shown confirmed only when the last check is fresh; a stale or offline check \
             downgrades the card and names when it last verified.",
        ),
        default_redaction: AdminRedactionClass::MetadataSafeDefault,
        scope: AdminScopeClass::ManagedOrg,
        live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&[
            "open_device_details",
            "open_rebind_lineage",
            "export_posture_snapshot",
        ]),
        publish_later_capture: false,
        locally_explainable: true,
        boundary_note: "The card names its last-check age and managed-versus-local data footprint; \
                        a rebind is an attributable event, never a silent re-enrollment."
            .to_owned(),
        typed_not_portal_only: true,
    },
    ]
}

fn build_paths() -> Vec<AdminPathEntry> {
    use AdminDeploymentProfileClass::*;

    const LOCAL_SAFE_BASELINE: &str = "schemas/admin/effective_policy_card.schema.json";

    vec![
        AdminPathEntry {
            path: AdminPathClass::LocalIndividual,
            path_id: AdminPathClass::LocalIndividual.path_id(),
            label: AdminPathClass::LocalIndividual.label().to_owned(),
            summary: "Local-first individual install: admin surfaces render against local objects \
                      with no control-plane dependency and label any field that would need one."
                .to_owned(),
            deployment_profiles: vec![IndividualLocal],
            default_live_vs_snapshot: AdminLiveSnapshotClass::LiveOnly,
            write_posture: AdminPathWritePostureClass::WritesLive,
            boundary_recheck_required: false,
            local_safe_baseline_ref: LOCAL_SAFE_BASELINE.to_owned(),
            notes: "No managed policy applies; effective values are local defaults and overrides, \
                    and delete/export run locally with receipts."
                .to_owned(),
        },
        AdminPathEntry {
            path: AdminPathClass::ManagedCloud,
            path_id: AdminPathClass::ManagedCloud.path_id(),
            label: AdminPathClass::ManagedCloud.label().to_owned(),
            summary:
                "Managed cloud / control plane: policy bundles, audit history, retention, and \
                      offboarding are managed-org scoped and managed writes carry approval and \
                      boundary state."
                    .to_owned(),
            deployment_profiles: vec![EnterpriseOnline, ManagedCloud],
            default_live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
            write_posture: AdminPathWritePostureClass::WritesLive,
            boundary_recheck_required: true,
            local_safe_baseline_ref: LOCAL_SAFE_BASELINE.to_owned(),
            notes: "Managed writes require a fresh approval and pass a boundary recheck after any \
                    tenant/region/key change; the local admin plane stays explainable without the \
                    vendor portal."
                .to_owned(),
        },
        AdminPathEntry {
            path: AdminPathClass::SelfHosted,
            path_id: AdminPathClass::SelfHosted.path_id(),
            label: AdminPathClass::SelfHosted.label().to_owned(),
            summary: "Self-hosted control plane: the customer runs the managed source; admin \
                      surfaces name the self-hosted source and its verification posture."
                .to_owned(),
            deployment_profiles: vec![SelfHosted, EnterpriseOnline],
            default_live_vs_snapshot: AdminLiveSnapshotClass::SnapshotCapable,
            write_posture: AdminPathWritePostureClass::WritesLive,
            boundary_recheck_required: true,
            local_safe_baseline_ref: LOCAL_SAFE_BASELINE.to_owned(),
            notes: "Policy and audit sources are self-hosted; losing the source degrades to the \
                    mirrored/offline path rather than silently failing."
                .to_owned(),
        },
        AdminPathEntry {
            path: AdminPathClass::SovereignAirGapped,
            path_id: AdminPathClass::SovereignAirGapped.path_id(),
            label: AdminPathClass::SovereignAirGapped.label().to_owned(),
            summary: "Sovereign / air-gapped: no outbound control plane; policy and entitlement \
                      arrive as signed offline bundles and verification is local."
                .to_owned(),
            deployment_profiles: vec![SovereignAirGapped],
            default_live_vs_snapshot: AdminLiveSnapshotClass::SnapshotOnly,
            write_posture: AdminPathWritePostureClass::LocalDraftPreserved,
            boundary_recheck_required: true,
            local_safe_baseline_ref: LOCAL_SAFE_BASELINE.to_owned(),
            notes: "Effective policy and procurement proof are verified against signed offline \
                    bundles; an expired or unverifiable bundle is labeled, never assumed valid."
                .to_owned(),
        },
        AdminPathEntry {
            path: AdminPathClass::MirroredOffline,
            path_id: AdminPathClass::MirroredOffline.path_id(),
            label: AdminPathClass::MirroredOffline.label().to_owned(),
            summary: "Mirror-backed offline: the last-synced read-only view with freshness labels \
                      and publish-later capture for queued admin writes."
                .to_owned(),
            deployment_profiles: vec![ManagedCloud, EnterpriseOnline, SelfHosted],
            default_live_vs_snapshot: AdminLiveSnapshotClass::SnapshotOnly,
            write_posture: AdminPathWritePostureClass::PublishLaterQueued,
            boundary_recheck_required: true,
            local_safe_baseline_ref: LOCAL_SAFE_BASELINE.to_owned(),
            notes: "Reads are mirror-backed and labeled by freshness; delete/export requests are \
                    preserved as local drafts and queued to publish later, never lost."
                .to_owned(),
        },
        AdminPathEntry {
            path: AdminPathClass::ImportedSnapshot,
            path_id: AdminPathClass::ImportedSnapshot.path_id(),
            label: AdminPathClass::ImportedSnapshot.label().to_owned(),
            summary: "Imported snapshot: replayed admin evidence with no live target, rendered \
                      read-only and labeled imported."
                .to_owned(),
            deployment_profiles: vec![IndividualLocal, SelfHosted, EnterpriseOnline, ManagedCloud],
            default_live_vs_snapshot: AdminLiveSnapshotClass::SnapshotOnly,
            write_posture: AdminPathWritePostureClass::ReadOnlyReplay,
            boundary_recheck_required: false,
            local_safe_baseline_ref: LOCAL_SAFE_BASELINE.to_owned(),
            notes: "Every surface is labeled imported with no live destination; no admin action \
                    targets a live system from an imported snapshot."
                .to_owned(),
        },
    ]
}

fn build_shared_vocabulary(surfaces: &[AdminSurfaceEntry]) -> AdminSharedVocabulary {
    let def = |token: &str, label: &str| AdminTokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
    };

    // The bound source schemas are exactly the union of every surface's cited
    // schema, plus the local-safe baseline the paths lean on.
    let mut source_schema_refs: Vec<String> = surfaces
        .iter()
        .flat_map(|s| s.canonical_schema_refs.iter().cloned())
        .chain(std::iter::once(
            "schemas/admin/effective_policy_card.schema.json".to_owned(),
        ))
        .collect();
    source_schema_refs.sort();
    source_schema_refs.dedup();

    AdminSharedVocabulary {
        deployment_profiles: vec![
            def("individual_local", "Individual local"),
            def("self_hosted", "Self-hosted"),
            def("enterprise_online", "Enterprise online"),
            def("sovereign_air_gapped", "Sovereign / air-gapped"),
            def("managed_cloud", "Managed cloud"),
        ],
        redaction_classes: vec![
            def("metadata_safe_default", "Metadata-safe default"),
            def("admin_only_restricted", "Admin-only restricted"),
            def("internal_support_restricted", "Internal-support restricted"),
            def("signing_evidence_only", "Signing-evidence only"),
            def("compliance_restricted", "Compliance-restricted"),
        ],
        policy_source_states: vec![
            def("local_default", "Local default"),
            def("workspace_setting", "Workspace setting"),
            def("managed_policy_bundle", "Managed policy bundle"),
            def("mirrored_policy_bundle", "Mirrored policy bundle (offline)"),
            def("remembered_decision", "Remembered decision"),
            def("signed_offline_bundle", "Signed offline bundle"),
            def("unknown_source", "Unknown source — requires review"),
        ],
        verification_postures: vec![
            def("signed_verified", "Signed and verified"),
            def("signed_unverified", "Signed, not yet verified"),
            def("unsigned_local", "Unsigned local"),
            def("signature_expired", "Signature expired"),
            def("signature_revoked", "Signature revoked"),
            def("unverifiable_offline", "Unverifiable offline"),
        ],
        delete_export_states: vec![
            def("available_now", "Available now"),
            def("queued_publish_later", "Queued — publish later"),
            def("blocked_by_hold", "Blocked by hold"),
            def("in_progress", "In progress"),
            def("completed_with_receipt", "Completed with receipt"),
            def("expired_window", "Window expired"),
            def("not_applicable", "Not applicable"),
        ],
        data_residency_classes: vec![
            def("local_only", "Local-only"),
            def("managed_copy", "Managed copy"),
            def("mirrored_copy", "Mirrored copy"),
            def("shared_workspace_copy", "Shared workspace copy"),
            def("exported_snapshot", "Exported snapshot"),
        ],
        owner_escalation_roles: vec![
            def("local_user", "Local user"),
            def("workspace_owner", "Workspace owner"),
            def("org_admin", "Org admin"),
            def("security_owner", "Security owner"),
            def("compliance_owner", "Compliance owner"),
            def("vendor_support", "Vendor support"),
        ],
        scope_classes: vec![
            def("local_private", "Local / private"),
            def("shared_workspace", "Shared / workspace"),
            def("managed_org", "Managed / org"),
        ],
        live_snapshot_classes: vec![
            def("live_only", "Live only"),
            def("snapshot_capable", "Snapshot-capable"),
            def("snapshot_only", "Snapshot only"),
        ],
        consumer_classes: vec![
            def("shell_admin_center", "Shell admin center"),
            def("cli_headless", "CLI / headless"),
            def("help_about", "Help / About"),
            def("support_export", "Support export"),
            def("commercial_procurement", "Commercial / procurement"),
            def("release_evidence", "Release evidence"),
            def("managed_service", "Managed service"),
        ],
        boundary_axes: vec![
            def("tenant", "Tenant"),
            def("region", "Region"),
            def("residency", "Residency"),
            def("key_ownership", "Key ownership"),
            def("endpoint_identity", "Endpoint identity"),
        ],
        source_schema_refs,
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> AdminMatrixInvariant {
    AdminMatrixInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    surfaces: &[AdminSurfaceEntry],
    paths: &[AdminPathEntry],
    states: &[AdminStateTerm],
) -> Vec<AdminMatrixInvariant> {
    use AdminStateClass::*;
    use ControlledVocabulary::*;

    let mut out = Vec::new();

    // Every surface points at a canonical object and a producer.
    out.push(invariant(
        "admin_plane.canonical_object_identity",
        "Every admin surface cites at least one canonical boundary schema and at least one \
         producing crate, so docs/help/support/commercial point at the same objects.",
        surfaces
            .iter()
            .all(|s| !s.canonical_schema_refs.is_empty() && !s.produced_by_refs.is_empty()),
    ));

    // Release-automation binding: every surface maps to a proof packet. A claimed
    // admin surface with no mapped proof row flips this false and fails promotion.
    out.push(invariant(
        "admin_plane.proof_packet_mapped",
        "Every admin surface maps to a non-empty proof packet that keeps it current, so stable \
         promotion fails when a claimed surface lacks a mapped proof row.",
        surfaces.iter().all(|s| !s.proof_packet_ref.is_empty()),
    ));

    // No-silent-green: every freshness-headlined surface carries the stale
    // downgrade and downgrades green.
    let green_headlined = [
        AdminSurfaceClass::EffectivePolicyView,
        AdminSurfaceClass::PolicyDiff,
        AdminSurfaceClass::RetentionDeletionMatrix,
        AdminSurfaceClass::ProcurementVerificationPacket,
        AdminSurfaceClass::EndpointPostureCard,
        AdminSurfaceClass::DecisionHistoryTimeline,
    ];
    out.push(invariant(
        "admin_plane.no_silent_green",
        "Every freshness-headlined surface carries the unconfirmed_stale state and a freshness \
         rule that downgrades a would-be-current headline when its evidence is stale or cached.",
        green_headlined.iter().all(|class| {
            surfaces
                .iter()
                .find(|s| s.surface == *class)
                .is_some_and(|s| s.can_show(UnconfirmedStale) && s.freshness_rule.downgrades_green)
        }),
    ));

    // Locked state is always explained with a source and an owner.
    out.push(invariant(
        "admin_plane.locked_state_explained",
        "Every surface that can show locked_by_policy binds the policy-source-state and \
         owner/escalation vocabularies and declares a required ownership/decision-right field.",
        surfaces.iter().all(|s| {
            if !s.can_show(LockedByPolicy) {
                return true;
            }
            s.binds(PolicySourceState)
                && s.binds(OwnerEscalation)
                && s.ownership_fields.iter().any(|f| f.required)
        }),
    ));

    // Ownership and decision-rights stay visible.
    out.push(invariant(
        "admin_plane.ownership_visible",
        "Every surface binds the owner/escalation vocabulary and declares at least one required \
         ownership/decision-right field.",
        surfaces
            .iter()
            .all(|s| s.binds(OwnerEscalation) && s.ownership_fields.iter().any(|f| f.required)),
    ));

    // Delete/export honesty: surfaces that act on data carry the state and a
    // receipt or blocked-by-hold path, never a bare 'deleted' claim.
    out.push(invariant(
        "admin_plane.delete_export_honest",
        "Every surface that can show a delete or export state binds the delete/export vocabulary \
         and exposes a destruction-receipt or blocked-by-hold path rather than a bare deleted \
         claim.",
        surfaces.iter().all(|s| {
            let acts_on_data = s.can_show(DeletePending)
                || s.can_show(ExportAvailableNow)
                || s.can_show(ExportDeferred);
            if !acts_on_data {
                return true;
            }
            let receipt_or_hold = s.can_show(DeleteReceipted) || s.can_show(DeleteBlockedByHold);
            let delete_capable = s.can_show(DeletePending);
            s.binds(DeleteExportState) && (!delete_capable || receipt_or_hold)
        }),
    ));

    // Data classes are located: managed-copy versus local-only is declared.
    out.push(invariant(
        "admin_plane.data_class_located",
        "Every surface that captures or exports data binds the data-residency vocabulary so \
         managed-copy versus local-only is explicit.",
        surfaces.iter().all(|s| {
            let touches_data = s.captures_user_writes
                || s.can_show(ExportAvailableNow)
                || s.can_show(ExportDeferred);
            !touches_data || s.binds(DataResidencyClass)
        }),
    ));

    // Verification posture is explicit wherever a signature can be unverified.
    out.push(invariant(
        "admin_plane.verification_posture_explicit",
        "Every surface that can show signature_unverified binds the verification/signature \
         vocabulary so unverified posture is never presented as verified.",
        surfaces
            .iter()
            .all(|s| !s.can_show(SignatureUnverified) || s.binds(VerificationSignaturePosture)),
    ));

    // Local explainability offline: surfaces keep local-safe actions and label
    // mirror-offline.
    out.push(invariant(
        "admin_plane.locally_explainable_offline",
        "Every surface is locally explainable and keeps local-safe actions, and every \
         write-bearing surface offers publish-later capture for queued writes.",
        surfaces.iter().all(|s| {
            if !s.locally_explainable || s.local_safe_actions.is_empty() {
                return false;
            }
            !s.captures_user_writes || s.publish_later_capture
        }),
    ));

    // Every named controlled vocabulary is actually bound by some surface.
    out.push(invariant(
        "admin_plane.controlled_vocabulary_complete",
        "Each of the five named controlled vocabularies — policy source state, verification/\
         signature posture, delete/export state, data residency, owner/escalation — is bound by \
         at least one surface.",
        ControlledVocabulary::ALL
            .iter()
            .all(|v| surfaces.iter().any(|s| s.binds(*v))),
    ));

    // Stable ids and tokens defined once and unique.
    out.push(invariant(
        "admin_plane.stable_ids_unique",
        "Surface ids, path ids, and state tokens are each defined once and unique, so consumers \
         can resolve a surface, path, or state by a stable token.",
        all_unique(surfaces.iter().map(|s| s.surface_id.as_str()))
            && all_unique(paths.iter().map(|p| p.path_id.as_str()))
            && all_unique(states.iter().map(|t| t.token.as_str())),
    ));

    // Every admin path is covered.
    out.push(invariant(
        "admin_plane.all_paths_covered",
        "The matrix covers local-individual, managed-cloud, self-hosted, sovereign/air-gapped, \
         mirrored/offline, and imported-snapshot admin paths.",
        AdminPathClass::ALL
            .iter()
            .all(|class| paths.iter().any(|p| p.path == *class)),
    ));

    // Every surface family is present.
    out.push(invariant(
        "admin_plane.all_surfaces_present",
        "Every admin-plane object family in the matrix is present exactly once.",
        AdminSurfaceClass::ALL
            .iter()
            .all(|class| surfaces.iter().filter(|s| s.surface == *class).count() == 1),
    ));

    // Typed, never portal/console-only.
    out.push(invariant(
        "admin_plane.typed_not_portal_only",
        "Every surface is typed and locally explainable: it carries state terms and schema refs \
         and is never reduced to a portal-only or console-only view.",
        surfaces.iter().all(|s| {
            s.typed_not_portal_only
                && s.locally_explainable
                && !s.applicable_states.is_empty()
                && !s.canonical_schema_refs.is_empty()
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the matrix as human-readable lines for CLI/headless and support.
pub fn admin_plane_lines(matrix: &AdminPlaneMatrix) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Admin-plane matrix — {} ({})",
        matrix.matrix_id, matrix.as_of
    ));
    lines.push(matrix.summary.clone());
    lines.push(format!(
        "Surfaces: {}  Paths: {}  States: {}  Invariants: {}",
        matrix.surfaces.len(),
        matrix.admin_paths.len(),
        matrix.state_vocabulary.len(),
        matrix.invariants.len(),
    ));

    lines.push("Surfaces:".to_owned());
    for s in &matrix.surfaces {
        let states: Vec<&str> = s.applicable_states.iter().map(|st| st.as_str()).collect();
        let vocab: Vec<&str> = s
            .controlled_vocabularies
            .iter()
            .map(|v| v.as_str())
            .collect();
        lines.push(format!(
            "  - {} [{}] scope={} live={:?} redaction={:?}",
            s.surface.as_str(),
            s.surface_id,
            scope_token(s.scope),
            s.live_vs_snapshot,
            s.default_redaction,
        ));
        lines.push(format!("      {}", s.summary));
        lines.push(format!("      states: {}", states.join(", ")));
        lines.push(format!("      vocabularies: {}", vocab.join(", ")));
        lines.push(format!(
            "      schemas: {}",
            s.canonical_schema_refs.join(", ")
        ));
        lines.push(format!("      proof: {}", s.proof_packet_ref));
        if !s.local_safe_actions.is_empty() {
            lines.push(format!(
                "      local-safe: {} (publish-later: {})",
                s.local_safe_actions.join(", "),
                s.publish_later_capture
            ));
        }
    }

    lines.push("Paths:".to_owned());
    for p in &matrix.admin_paths {
        lines.push(format!(
            "  - {} [{}] write={:?} boundary_recheck={}",
            p.path.as_str(),
            p.path_id,
            p.write_posture,
            p.boundary_recheck_required
        ));
        lines.push(format!("      {}", p.summary));
    }

    lines.push("Invariants:".to_owned());
    for i in &matrix.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}

fn scope_token(scope: AdminScopeClass) -> &'static str {
    match scope {
        AdminScopeClass::LocalPrivate => "local_private",
        AdminScopeClass::SharedWorkspace => "shared_workspace",
        AdminScopeClass::ManagedOrg => "managed_org",
    }
}
