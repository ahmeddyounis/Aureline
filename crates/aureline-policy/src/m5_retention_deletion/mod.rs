//! M5 admin-plane *retention / deletion matrices*: the concrete, typed instances
//! of the retention/deletion surface Aureline shows on its claimed managed-cloud,
//! self-hosted, sovereign/air-gapped, and mirrored/offline profiles.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — including the
//! [`RetentionDeletionMatrix`](crate::m5_admin_plane::AdminSurfaceClass::RetentionDeletionMatrix)
//! surface family, its applicable states, the controlled vocabularies it binds,
//! and the proof packet that keeps it current — this lane *renders that surface*.
//! It turns retention and deletion truth into a first-class local product
//! surface: for every claimed managed artifact family a user or admin can read, on
//! the machine in front of them, what data class it is, where its copies live,
//! what its default retention is, what its export and delete routes are, who owns
//! it, what schema governs it, and — most importantly — whether a delete completes
//! immediately, is deferred, or is blocked, and exactly what remains, where it
//! remains, and who controls the next step, without opening a separate vendor
//! console.
//!
//! Each matrix binds back to the frozen
//! [admin-plane matrix](crate::m5_admin_plane). Every machine-readable state a row
//! or the coverage posture shows must be one the matrix declares applicable for
//! the retention/deletion surface
//! ([`RetentionDeletionInvariant`] `retention_deletion.surface_states_within_matrix`),
//! and every owner and data-residency token it uses is a term the matrix's shared
//! vocabulary defines. So the render layer cannot drift from the frozen contract:
//! an edit that shows a state the matrix does not admit flips an invariant and
//! fails the freeze gate.
//!
//! The bundle holds one [`RetentionDeletionPacket`] per claimed managed-bearing
//! profile and computes each invariant's `holds` flag from the rendered data, so
//! the checked-in fixture freezes the rendered matrices byte-for-byte. Honesty
//! rules are enforced, not just described:
//!
//! - Every row names a *specific* [`ArtifactOwnerClass`] — user-owned,
//!   workspace-owned, tenant-owned, imported, or derived-cache — rather than
//!   collapsing every artifact into one bucket
//!   (`retention_deletion.data_classes_distinguished`).
//! - Every row names its retention class, export route, delete route, current
//!   block/defer/immediate [`DeleteOutcomeClass`], machine-readable state, owner,
//!   and governing schema (`retention_deletion.retention_route_outcome_complete`).
//! - A delete that cannot complete immediately explains what remains, where it
//!   remains, when it is expected to complete, and who controls the next step
//!   (`retention_deletion.non_immediate_explains_remainder`).
//! - Deletion states link to destruction receipts, privacy-request cases, holds,
//!   and partial-delete reasons as *distinct* [`DeletionLinkageClass`] linkages
//!   instead of one generic pending status
//!   (`retention_deletion.deletion_linkage_distinct`); a receipted delete carries
//!   its receipt and a hold-blocked delete names its hold, never a bare deleted
//!   claim (`retention_deletion.delete_export_honest`).
//! - A row whose backing evidence is stale is never shown as a confirmed-green
//!   delete/export claim (`retention_deletion.no_silent_green`).
//! - The retention/delete states propagate unchanged into support exports,
//!   offboarding flows, compliance packets, and Help/About public-truth surfaces
//!   (`retention_deletion.propagation_complete`), and every profile stays locally
//!   inspectable without a vendor console
//!   (`retention_deletion.locally_inspectable_offline`).
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, raw record bodies, or absolute paths — only opaque object refs,
//! stable tokens, rendered metadata-safe summaries, and short reviewable sentences
//! — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminStateClass,
    AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};
use crate::m5_admin_render::{DataResidencyClass, EvidenceAgeClass, OwnerEscalationRoleClass};
// Reuse — and re-export — the generic completeness and export-form vocabularies
// the sibling decision-history render layer already freezes, so the
// retention/deletion matrix labels coverage and export forms with the same tokens
// every admin surface uses and consumers can resolve them from this module.
pub use crate::m5_decision_history::{CompletenessClass, ExportForm, ExportFormatClass};

#[cfg(test)]
mod tests;

/// Schema version for the retention/deletion bundle.
pub const M5_RETENTION_DELETION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the retention/deletion bundle.
pub const M5_RETENTION_DELETION_SCHEMA_REF: &str =
    "schemas/admin/m5-retention-deletion.schema.json";

/// Stable record-kind tag for the retention/deletion bundle.
pub const M5_RETENTION_DELETION_RECORD_KIND: &str = "m5_retention_deletion_bundle";

/// Stable id for the canonical retention/deletion bundle.
pub const M5_RETENTION_DELETION_BUNDLE_ID: &str = "m5-retention-deletion:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_RETENTION_DELETION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this render layer binds back to.
pub const M5_RETENTION_DELETION_MATRIX_REF: &str =
    "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the retention/deletion bundle current.
pub const M5_RETENTION_DELETION_FREEZE_GATE_REF: &str =
    "crates/aureline-policy/tests/m5_retention_deletion.rs";

// ---------------------------------------------------------------------------
// Retention/deletion token enums.
// ---------------------------------------------------------------------------

/// The data-class / ownership of an artifact family — the spec's honesty
/// requirement to distinguish user-owned, workspace-owned, tenant-owned,
/// imported, and derived-cache artifacts rather than flattening them into one
/// generic "your data" bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactOwnerClass {
    /// Owned by the local user; the user controls export and deletion directly.
    UserOwned,
    /// Owned by a workspace/team; shared, governed at the workspace level.
    WorkspaceOwned,
    /// Owned by the managed tenant/organization; governed by managed policy.
    TenantOwned,
    /// Imported from an external snapshot/bundle with no live source.
    Imported,
    /// A derived cache/index regenerated from other artifacts; not a system of
    /// record.
    DerivedCache,
}

impl ArtifactOwnerClass {
    /// All artifact-owner classes, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::UserOwned,
        Self::WorkspaceOwned,
        Self::TenantOwned,
        Self::Imported,
        Self::DerivedCache,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserOwned => "user_owned",
            Self::WorkspaceOwned => "workspace_owned",
            Self::TenantOwned => "tenant_owned",
            Self::Imported => "imported",
            Self::DerivedCache => "derived_cache",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserOwned => "User-owned",
            Self::WorkspaceOwned => "Workspace-owned",
            Self::TenantOwned => "Tenant-owned",
            Self::Imported => "Imported",
            Self::DerivedCache => "Derived cache",
        }
    }
}

/// The current delete outcome for a row — the spec's required block/immediate/
/// deferred distinction. A non-immediate outcome must explain its remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteOutcomeClass {
    /// Delete completes now, locally, with nothing left behind.
    Immediate,
    /// Delete is accepted but completes later; copies remain until it does.
    Deferred,
    /// Delete cannot proceed — a hold or policy blocks it — and says why.
    Blocked,
}

impl DeleteOutcomeClass {
    /// All delete outcomes, in vocabulary order.
    pub const ALL: [Self; 3] = [Self::Immediate, Self::Deferred, Self::Blocked];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Deferred => "deferred",
            Self::Blocked => "blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Immediate => "Immediate",
            Self::Deferred => "Deferred",
            Self::Blocked => "Blocked",
        }
    }

    /// Whether this outcome must explain what remains, where, and who controls the
    /// next step (anything that does not complete immediately).
    pub const fn requires_remainder(self) -> bool {
        matches!(self, Self::Deferred | Self::Blocked)
    }
}

/// The default-retention rule a record class carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionRuleClass {
    /// Kept until the user deletes it; no automatic expiry.
    UserControlled,
    /// Kept for a fixed window, then auto-deleted.
    FixedWindow,
    /// Retained to satisfy a regulatory / legal-hold minimum.
    RegulatoryHold,
    /// Retained for the lifetime of the entitlement / seat.
    EntitlementLifetime,
    /// Derived and regenerable; not separately retained.
    EphemeralRegenerable,
    /// A last-synced mirror copy retained until the next sync.
    MirrorLastSynced,
}

impl RetentionRuleClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserControlled => "user_controlled",
            Self::FixedWindow => "fixed_window",
            Self::RegulatoryHold => "regulatory_hold",
            Self::EntitlementLifetime => "entitlement_lifetime",
            Self::EphemeralRegenerable => "ephemeral_regenerable",
            Self::MirrorLastSynced => "mirror_last_synced",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserControlled => "User-controlled",
            Self::FixedWindow => "Fixed window",
            Self::RegulatoryHold => "Regulatory hold",
            Self::EntitlementLifetime => "Entitlement lifetime",
            Self::EphemeralRegenerable => "Ephemeral / regenerable",
            Self::MirrorLastSynced => "Mirror last-synced",
        }
    }
}

/// The export route a record class offers — the spec's export-path column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRouteClass {
    /// Exportable locally right now.
    LocalExportNow,
    /// Exportable after a managed reauthentication.
    ExportAfterReauth,
    /// Exportable through the offboarding flow.
    ExportViaOffboarding,
    /// Exportable from the last-synced mirror only.
    ExportFromMirror,
    /// No export route — the artifact is derived/regenerable, not a record.
    ExportNotApplicable,
}

impl ExportRouteClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalExportNow => "local_export_now",
            Self::ExportAfterReauth => "export_after_reauth",
            Self::ExportViaOffboarding => "export_via_offboarding",
            Self::ExportFromMirror => "export_from_mirror",
            Self::ExportNotApplicable => "export_not_applicable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalExportNow => "Local export now",
            Self::ExportAfterReauth => "Export after reauth",
            Self::ExportViaOffboarding => "Export via offboarding",
            Self::ExportFromMirror => "Export from mirror",
            Self::ExportNotApplicable => "Export not applicable",
        }
    }
}

/// The delete route a record class offers — the spec's delete-path column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteRouteClass {
    /// Deletable locally right now.
    LocalDeleteNow,
    /// Deletable by filing a deletion / privacy request.
    DeleteViaRequest,
    /// Deletable through the offboarding flow.
    DeleteViaOffboarding,
    /// Deletion is queued and completes when the mirror reconnects.
    DeleteOnReconnect,
    /// Deletion is blocked by an active hold.
    DeleteBlockedHold,
    /// No delete route — the artifact is derived/regenerable.
    DeleteNotApplicable,
}

impl DeleteRouteClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeleteNow => "local_delete_now",
            Self::DeleteViaRequest => "delete_via_request",
            Self::DeleteViaOffboarding => "delete_via_offboarding",
            Self::DeleteOnReconnect => "delete_on_reconnect",
            Self::DeleteBlockedHold => "delete_blocked_hold",
            Self::DeleteNotApplicable => "delete_not_applicable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDeleteNow => "Local delete now",
            Self::DeleteViaRequest => "Delete via request",
            Self::DeleteViaOffboarding => "Delete via offboarding",
            Self::DeleteOnReconnect => "Delete on reconnect",
            Self::DeleteBlockedHold => "Delete blocked by hold",
            Self::DeleteNotApplicable => "Delete not applicable",
        }
    }
}

/// What a deletion state links to — the spec's requirement to link deletion
/// states to destruction receipts, privacy-request cases, holds, and
/// partial-delete reasons as distinct linkages rather than one generic pending
/// status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletionLinkageClass {
    /// A destruction receipt proving a delete actually happened.
    DestructionReceipt,
    /// A privacy / data-subject request case driving the deletion.
    PrivacyRequestCase,
    /// A legal / retention hold blocking the deletion.
    LegalHold,
    /// A reason a deletion can only complete partially right now.
    PartialDeleteReason,
}

impl DeletionLinkageClass {
    /// All linkage classes, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::DestructionReceipt,
        Self::PrivacyRequestCase,
        Self::LegalHold,
        Self::PartialDeleteReason,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DestructionReceipt => "destruction_receipt",
            Self::PrivacyRequestCase => "privacy_request_case",
            Self::LegalHold => "legal_hold",
            Self::PartialDeleteReason => "partial_delete_reason",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DestructionReceipt => "Destruction receipt",
            Self::PrivacyRequestCase => "Privacy-request case",
            Self::LegalHold => "Legal hold",
            Self::PartialDeleteReason => "Partial-delete reason",
        }
    }
}

/// The surfaces a row's retention/delete state must propagate into unchanged — the
/// spec's requirement that retention/delete states reach support exports,
/// offboarding flows, compliance packets, and Help/About public-truth surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropagationTargetClass {
    /// The support export / bundle.
    SupportExport,
    /// The offboarding wizard / flow.
    OffboardingFlow,
    /// A compliance / procurement packet.
    CompliancePacket,
    /// The Help / About public-truth surface.
    HelpAboutPublicTruth,
}

impl PropagationTargetClass {
    /// All propagation targets, in order.
    pub const ALL: [Self; 4] = [
        Self::SupportExport,
        Self::OffboardingFlow,
        Self::CompliancePacket,
        Self::HelpAboutPublicTruth,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportExport => "support_export",
            Self::OffboardingFlow => "offboarding_flow",
            Self::CompliancePacket => "compliance_packet",
            Self::HelpAboutPublicTruth => "help_about_public_truth",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SupportExport => "Support export",
            Self::OffboardingFlow => "Offboarding flow",
            Self::CompliancePacket => "Compliance packet",
            Self::HelpAboutPublicTruth => "Help / About public truth",
        }
    }
}

// ---------------------------------------------------------------------------
// Deletion linkage, remainder, and the retention row.
// ---------------------------------------------------------------------------

/// One link from a deletion state to its receipt, privacy case, hold, or
/// partial-delete reason — kept distinct rather than flattened into a generic
/// pending status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionLinkage {
    /// The linkage class.
    pub linkage: DeletionLinkageClass,
    /// One reviewable label.
    pub label: String,
    /// The opaque object ref this links to (receipt id, case id, hold id, or
    /// reason token).
    pub linked_ref: String,
    /// One reviewable sentence describing the link.
    pub note: String,
}

/// What remains after a delete that cannot complete immediately — the spec's
/// requirement to explain what remains, where it remains, when it is expected to
/// complete, and who controls the next step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionRemainder {
    /// What still exists after the immediate part of the delete.
    pub what_remains: String,
    /// Where the remainder lives.
    pub where_it_remains: DataResidencyClass,
    /// When the delete is expected to complete (a reviewable phrase, not a raw
    /// timestamp).
    pub expected_completion: String,
    /// Who controls the next step.
    pub next_step_owner: OwnerEscalationRoleClass,
    /// One reviewable sentence describing the remainder.
    pub note: String,
}

/// One row in a profile's retention/deletion matrix: a claimed managed artifact
/// family, its data class and location, its retention and routes, and its current
/// delete outcome with the receipts/holds/cases that explain it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionRow {
    /// Stable, opaque row id (deep-linkable, export-safe).
    pub row_id: String,
    /// The artifact family token (a stable record-class token).
    pub record_family: String,
    /// One reviewable label for the artifact family.
    pub record_family_label: String,
    /// The data class / ownership of the artifact.
    pub data_class: ArtifactOwnerClass,
    /// Where the artifact's copies live (local-only versus hosted/managed/
    /// mirrored).
    pub location: DataResidencyClass,
    /// The default-retention rule.
    pub retention: RetentionRuleClass,
    /// One reviewable label stating the default retention.
    pub retention_label: String,
    /// The export route.
    pub export_route: ExportRouteClass,
    /// One reviewable label for the export route.
    pub export_label: String,
    /// The delete route.
    pub delete_route: DeleteRouteClass,
    /// One reviewable label for the delete route.
    pub delete_label: String,
    /// The current delete outcome (immediate / deferred / blocked).
    pub delete_outcome: DeleteOutcomeClass,
    /// The machine-readable state (must be one the matrix admits for this
    /// surface).
    pub machine_state: AdminStateClass,
    /// The freshness of the evidence backing the row.
    pub evidence_age: EvidenceAgeClass,
    /// Who owns retention/deletion for this artifact family.
    pub owner: OwnerEscalationRoleClass,
    /// The schema that governs this record class (the spec's "schema note").
    pub governing_schema_ref: String,
    /// One reviewable sentence noting how the class is schema-governed.
    pub schema_note: String,
    /// The remainder for a non-immediate delete, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<DeletionRemainder>,
    /// The receipts/cases/holds/partial-reasons this row links to.
    pub linkages: Vec<DeletionLinkage>,
    /// The export-safe machine-readable summary (stable tokens, never a secret).
    pub machine_summary: String,
    /// The plain-language support/admin handoff sentence.
    pub plain_language: String,
}

impl RetentionRow {
    /// Whether the row carries both export representations.
    pub fn has_export_parity(&self) -> bool {
        !self.machine_summary.is_empty() && !self.plain_language.is_empty()
    }

    /// Whether the row carries a linkage of the given class.
    pub fn has_linkage(&self, class: DeletionLinkageClass) -> bool {
        self.linkages.iter().any(|l| l.linkage == class)
    }

    /// The distinct linkage classes present on this row.
    pub fn linkage_classes(&self) -> std::collections::BTreeSet<DeletionLinkageClass> {
        self.linkages.iter().map(|l| l.linkage).collect()
    }
}

// ---------------------------------------------------------------------------
// Propagation, coverage, the matrix, the per-profile packet, and the bundle.
// ---------------------------------------------------------------------------

/// One surface a row's retention/delete state propagates into unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropagationTarget {
    /// The target surface.
    pub target: PropagationTargetClass,
    /// One reviewable label.
    pub label: String,
    /// One reviewable sentence describing the propagation.
    pub note: String,
}

/// The coverage posture of a rendered matrix: how complete the registry view is,
/// and whether it stays locally inspectable without a vendor console.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionCoverage {
    /// The coverage state (must be one the matrix admits for this surface).
    pub coverage_state: AdminStateClass,
    /// How complete the registry view is.
    pub completeness: CompletenessClass,
    /// One reviewable label for the coverage window.
    pub window_label: String,
    /// One reviewable sentence stating the coverage rule and any labeled gap.
    pub coverage_note: String,
    /// Whether the matrix is locally inspectable on this profile.
    pub locally_inspectable: bool,
    /// Whether the matrix is available without a vendor console / control plane.
    pub vendor_console_independent: bool,
}

/// The rendered retention/deletion matrix for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionDeletionMatrix {
    /// The surface family (always
    /// [`AdminSurfaceClass::RetentionDeletionMatrix`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The retention/deletion rows, one per claimed artifact family.
    pub rows: Vec<RetentionRow>,
    /// The export forms offered.
    pub export_forms: Vec<ExportForm>,
    /// The surfaces the retention/delete states propagate into unchanged.
    pub propagation: Vec<PropagationTarget>,
    /// The coverage posture of the matrix.
    pub coverage: RetentionCoverage,
}

impl RetentionDeletionMatrix {
    /// Resolves a row by id, if present.
    pub fn row(&self, row_id: &str) -> Option<&RetentionRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// The distinct artifact-owner classes present in the matrix.
    pub fn owner_classes(&self) -> std::collections::BTreeSet<ArtifactOwnerClass> {
        self.rows.iter().map(|r| r.data_class).collect()
    }

    /// Whether the matrix offers a given export format.
    pub fn offers(&self, format: ExportFormatClass) -> bool {
        self.export_forms.iter().any(|f| f.format == format)
    }

    /// Whether the matrix names a given propagation target.
    pub fn propagates_to(&self, target: PropagationTargetClass) -> bool {
        self.propagation.iter().any(|p| p.target == target)
    }
}

/// The rendered retention/deletion surface for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionDeletionPacket {
    /// The admin path / profile this packet renders.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The retention/deletion matrix.
    pub matrix: RetentionDeletionMatrix,
}

impl RetentionDeletionPacket {
    /// Resolves a row by id within this packet.
    pub fn row(&self, row_id: &str) -> Option<&RetentionRow> {
        self.matrix.row(row_id)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionDeletionInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the rendered bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen retention/deletion bundle: one packet per claimed managed-bearing
/// profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionDeletionBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_retention_deletion_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The matrix this render layer binds back to.
    pub matrix_ref: String,
    /// The matrix id this render layer binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps this bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The per-profile retention/deletion packets.
    pub profiles: Vec<RetentionDeletionPacket>,
    /// The computed invariants.
    pub invariants: Vec<RetentionDeletionInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionDeletionValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RetentionDeletionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "retention/deletion bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for RetentionDeletionValidationError {}

/// The profiles the retention/deletion bundle covers, in bundle order.
pub const RETENTION_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

impl RetentionDeletionBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&RetentionDeletionPacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque token, never
    /// a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The repo-relative file refs carried by the bundle, for export-safety
    /// auditing. Stable token ids are audited separately by [`is_safe_token`].
    fn file_refs(&self) -> Vec<&str> {
        let mut refs = vec![
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ];
        for p in &self.profiles {
            for r in &p.matrix.rows {
                refs.push(r.governing_schema_ref.as_str());
            }
        }
        refs
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            ids.push(p.matrix.surface_id.as_str());
            for r in &p.matrix.rows {
                ids.push(r.row_id.as_str());
                ids.push(r.record_family.as_str());
                for l in &r.linkages {
                    ids.push(l.linked_ref.as_str());
                }
            }
            for x in &p.matrix.export_forms {
                ids.push(x.artifact_ref.as_str());
            }
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`RetentionDeletionInvariant`]s with the
    /// coverage and resolution checks a consumer relies on.
    pub fn validate(&self) -> Result<(), RetentionDeletionValidationError> {
        let fail = |reason: String| Err(RetentionDeletionValidationError { reason });

        if self.record_kind != M5_RETENTION_DELETION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_RETENTION_DELETION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in RETENTION_PROFILES {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        for packet in &self.profiles {
            validate_packet(packet)
                .map_err(|reason| RetentionDeletionValidationError { reason })?;
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
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

/// Whether a stable token id is safe to export: non-empty and carries no URL
/// scheme or absolute path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Whether a state asserts a currently-confirmed positive delete/export claim, so
/// stale evidence under it would be a silent-green lie: the artifact is actively
/// retained, exportable now, or already destroyed with a receipt. The other
/// admitted states are explicit non-confirmations (pending, deferred, blocked,
/// unconfirmed).
fn requires_fresh_evidence(state: AdminStateClass) -> bool {
    matches!(
        state,
        AdminStateClass::ActiveEnforced
            | AdminStateClass::ExportAvailableNow
            | AdminStateClass::DeleteReceipted
    )
}

/// Per-packet structural floor checks, shared by
/// [`RetentionDeletionBundle::validate`].
fn validate_packet(packet: &RetentionDeletionPacket) -> Result<(), String> {
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!(
            "profile id mismatch for {}",
            packet.profile.as_str()
        ));
    }
    let matrix = &packet.matrix;
    if matrix.surface != AdminSurfaceClass::RetentionDeletionMatrix {
        return Err(format!(
            "{}: matrix is not the retention/deletion surface",
            packet.profile.as_str()
        ));
    }
    if matrix.rows.is_empty() {
        return Err(format!("{}: no retention rows", packet.profile.as_str()));
    }
    if !all_unique(matrix.rows.iter().map(|r| r.row_id.as_str())) {
        return Err(format!(
            "{}: row ids are not unique",
            packet.profile.as_str()
        ));
    }
    for row in &matrix.rows {
        // A non-immediate delete must explain its remainder.
        if row.delete_outcome.requires_remainder() {
            let Some(remainder) = &row.remainder else {
                return Err(format!(
                    "{}: row {} is {} but explains no remainder",
                    packet.profile.as_str(),
                    row.row_id,
                    row.delete_outcome.as_str()
                ));
            };
            if remainder.what_remains.is_empty() || remainder.expected_completion.is_empty() {
                return Err(format!(
                    "{}: row {} remainder is incomplete",
                    packet.profile.as_str(),
                    row.row_id
                ));
            }
        }
        // Delete/export honesty: a receipted delete carries its receipt; a
        // hold-blocked delete names its hold.
        if row.machine_state == AdminStateClass::DeleteReceipted
            && !row.has_linkage(DeletionLinkageClass::DestructionReceipt)
        {
            return Err(format!(
                "{}: row {} is receipted with no destruction receipt",
                packet.profile.as_str(),
                row.row_id
            ));
        }
        if row.machine_state == AdminStateClass::DeleteBlockedByHold
            && !row.has_linkage(DeletionLinkageClass::LegalHold)
        {
            return Err(format!(
                "{}: row {} is hold-blocked with no named hold",
                packet.profile.as_str(),
                row.row_id
            ));
        }
        if !row.has_export_parity() {
            return Err(format!(
                "{}: row {} lacks both export representations",
                packet.profile.as_str(),
                row.row_id
            ));
        }
    }
    // Both export forms are offered.
    if !matrix.offers(ExportFormatClass::MachineReadableJson)
        || !matrix.offers(ExportFormatClass::PlainLanguageHandoff)
    {
        return Err(format!(
            "{}: matrix does not offer both export forms",
            packet.profile.as_str()
        ));
    }
    // The states propagate into all four required surfaces.
    for target in PropagationTargetClass::ALL {
        if !matrix.propagates_to(target) {
            return Err(format!(
                "{}: state does not propagate into {}",
                packet.profile.as_str(),
                target.as_str()
            ));
        }
    }
    // The matrix is locally inspectable without a vendor console.
    if !matrix.coverage.locally_inspectable || !matrix.coverage.vendor_console_independent {
        return Err(format!(
            "{}: matrix is not locally inspectable without a vendor console",
            packet.profile.as_str()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical retention/deletion bundle.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the rendered packets, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn retention_deletion_bundle() -> RetentionDeletionBundle {
    let profiles: Vec<RetentionDeletionPacket> = RETENTION_PROFILES
        .iter()
        .map(|p| retention_packet(*p))
        .collect();
    let invariants = compute_invariants(&profiles);

    RetentionDeletionBundle {
        record_kind: M5_RETENTION_DELETION_RECORD_KIND.to_owned(),
        m5_retention_deletion_schema_version: M5_RETENTION_DELETION_SCHEMA_VERSION,
        schema_ref: M5_RETENTION_DELETION_SCHEMA_REF.to_owned(),
        bundle_id: M5_RETENTION_DELETION_BUNDLE_ID.to_owned(),
        as_of: M5_RETENTION_DELETION_AS_OF.to_owned(),
        matrix_ref: M5_RETENTION_DELETION_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_RETENTION_DELETION_FREEZE_GATE_REF.to_owned(),
        summary:
            "Rendered retention/deletion matrices — each claimed managed artifact family with \
                  its data class, local-only versus hosted location, default retention, export and \
                  delete routes, owner, and governing schema, plus a current immediate/deferred/ \
                  blocked delete outcome linked to destruction receipts, privacy-request cases, \
                  holds, and partial-delete reasons — bound back to the frozen admin-plane matrix \
                  and rendered identically for shell, CLI/headless, Help/About, support export, \
                  and procurement consumers across the managed-cloud, self-hosted, \
                  sovereign/air-gapped, and mirrored/offline profiles, each kept locally \
                  inspectable without a vendor console."
                .to_owned(),
        profiles,
        invariants,
        raw_payload_excluded: true,
    }
}

/// The consumers every packet must serve identically; mirrors the matrix's
/// declared consumers for the retention/deletion surface.
fn parity_consumers() -> Vec<AdminConsumerClass> {
    admin_plane_matrix()
        .surface(AdminSurfaceClass::RetentionDeletionMatrix)
        .map(|entry| entry.consumed_by.clone())
        .unwrap_or_default()
}

fn retention_packet(profile: AdminPathClass) -> RetentionDeletionPacket {
    let (deployment_profile, summary) = match profile {
        AdminPathClass::ManagedCloud => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Managed-cloud profile: a live retention/deletion matrix confirmed against the managed \
             control plane; immediate, deferred, and hold-blocked deletes each name their receipt, \
             case, or hold.",
        ),
        AdminPathClass::SelfHosted => (
            AdminDeploymentProfileClass::SelfHosted,
            "Self-hosted profile: the customer's own control plane governs retention; a \
             regulatory-hold-blocked delete and a partial delete are both shown as themselves \
             rather than a generic pending status.",
        ),
        AdminPathClass::SovereignAirGapped => (
            AdminDeploymentProfileClass::SovereignAirGapped,
            "Sovereign / air-gapped profile: retention/deletion truth resolves from the signed \
             offline bundle and imported snapshots; an imported class and a derived cache are \
             labeled distinctly and the registry view is shown unconfirmed rather than green.",
        ),
        AdminPathClass::MirroredOffline => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Mirrored / offline profile: the managed source is offline, so a deferred delete is \
             queued to complete on reconnect and the upstream remainder is named; recorded rows \
             stay locally inspectable.",
        ),
        _ => (
            AdminDeploymentProfileClass::IndividualLocal,
            "Local profile.",
        ),
    };

    RetentionDeletionPacket {
        profile,
        profile_id: profile.path_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers: parity_consumers(),
        matrix: render_matrix(profile),
    }
}

fn render_matrix(profile: AdminPathClass) -> RetentionDeletionMatrix {
    let surface = AdminSurfaceClass::RetentionDeletionMatrix;
    let rows = build_rows(profile);
    let export_forms = build_export_forms(profile);
    let propagation = build_propagation();
    let coverage = build_coverage(profile);

    let summary = match profile {
        AdminPathClass::ManagedCloud => {
            "Each artifact family names its data class, where copies live, default retention, \
             export and delete routes, owner, and governing schema, with a current \
             immediate/deferred/blocked outcome and the receipt, case, or hold that explains it."
        }
        AdminPathClass::SelfHosted => {
            "The self-hosted control plane governs retention; a regulatory-hold-blocked delete and \
             a partial delete are surfaced as distinct outcomes, not folded into one pending row."
        }
        AdminPathClass::SovereignAirGapped => {
            "Retention/deletion truth comes from the signed offline bundle and imported snapshots; \
             imported and derived-cache classes are labeled distinctly and unconfirmed rows are \
             never shown as confirmed-green."
        }
        AdminPathClass::MirroredOffline => {
            "The mirror is offline, so deferred deletes are queued to complete on reconnect with \
             the upstream remainder named; nothing is shown as deleted without confirmation."
        }
        _ => "Retention/deletion matrix.",
    };

    RetentionDeletionMatrix {
        surface,
        surface_id: surface.surface_id(),
        summary: summary.to_owned(),
        rows,
        export_forms,
        propagation,
        coverage,
    }
}

/// One concise builder for a deletion linkage.
fn linkage(
    linkage: DeletionLinkageClass,
    label: &str,
    linked_ref: &str,
    note: &str,
) -> DeletionLinkage {
    DeletionLinkage {
        linkage,
        label: label.to_owned(),
        linked_ref: linked_ref.to_owned(),
        note: note.to_owned(),
    }
}

/// One concise builder for a deletion remainder.
fn remainder(
    what_remains: &str,
    where_it_remains: DataResidencyClass,
    expected_completion: &str,
    next_step_owner: OwnerEscalationRoleClass,
    note: &str,
) -> DeletionRemainder {
    DeletionRemainder {
        what_remains: what_remains.to_owned(),
        where_it_remains,
        expected_completion: expected_completion.to_owned(),
        next_step_owner,
        note: note.to_owned(),
    }
}

/// One concise builder for a retention row, to keep the per-profile data dense
/// and reviewable.
#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    record_family: &str,
    record_family_label: &str,
    data_class: ArtifactOwnerClass,
    location: DataResidencyClass,
    retention: RetentionRuleClass,
    retention_label: &str,
    export_route: ExportRouteClass,
    export_label: &str,
    delete_route: DeleteRouteClass,
    delete_label: &str,
    delete_outcome: DeleteOutcomeClass,
    machine_state: AdminStateClass,
    evidence_age: EvidenceAgeClass,
    owner: OwnerEscalationRoleClass,
    governing_schema_ref: &str,
    schema_note: &str,
    remainder: Option<DeletionRemainder>,
    linkages: Vec<DeletionLinkage>,
    machine_summary: &str,
    plain_language: &str,
) -> RetentionRow {
    RetentionRow {
        row_id: row_id.to_owned(),
        record_family: record_family.to_owned(),
        record_family_label: record_family_label.to_owned(),
        data_class,
        location,
        retention,
        retention_label: retention_label.to_owned(),
        export_route,
        export_label: export_label.to_owned(),
        delete_route,
        delete_label: delete_label.to_owned(),
        delete_outcome,
        machine_state,
        evidence_age,
        owner,
        governing_schema_ref: governing_schema_ref.to_owned(),
        schema_note: schema_note.to_owned(),
        remainder,
        linkages,
        machine_summary: machine_summary.to_owned(),
        plain_language: plain_language.to_owned(),
    }
}

const REGISTRY_SCHEMA: &str = "schemas/records/record-class-registry.schema.json";
const LIFECYCLE_SCHEMA: &str = "schemas/governance/records_export_delete_lifecycle.schema.json";

fn build_rows(profile: AdminPathClass) -> Vec<RetentionRow> {
    use AdminStateClass::*;
    use ArtifactOwnerClass::*;
    use DataResidencyClass::*;
    use DeleteOutcomeClass::*;
    use DeleteRouteClass::*;
    use DeletionLinkageClass::*;
    use EvidenceAgeClass::*;
    use ExportRouteClass::*;
    use OwnerEscalationRoleClass::*;
    use RetentionRuleClass::*;

    match profile {
        AdminPathClass::ManagedCloud => vec![
            row(
                "retention.row.managed_cloud.0001",
                "durable_workspace_state",
                "Durable workspace state",
                UserOwned,
                LocalOnly,
                UserControlled,
                "Kept on this machine until you delete it",
                LocalExportNow,
                "Export the local workspace now",
                LocalDeleteNow,
                "Delete locally now",
                Immediate,
                ExportAvailableNow,
                Fresh,
                LocalUser,
                REGISTRY_SCHEMA,
                "Governed as a user-owned durable record class with no managed copy.",
                None,
                vec![],
                "class=durable_workspace_state data_class=user_owned location=local_only \
                 retention=user_controlled outcome=immediate state=export_available_now",
                "Your durable workspace state lives only on this machine, is exportable now, and \
                 deletes immediately with nothing left behind.",
            ),
            row(
                "retention.row.managed_cloud.0002",
                "collaboration_session_record",
                "Collaboration session record",
                WorkspaceOwned,
                ManagedCopy,
                FixedWindow,
                "Retained 90 days in the managed copy, then auto-deleted",
                LocalExportNow,
                "Export your local copy now",
                DeleteViaRequest,
                "File a deletion request for the managed copy",
                Deferred,
                DeletePending,
                Fresh,
                WorkspaceOwner,
                REGISTRY_SCHEMA,
                "Governed as a workspace-owned class with a managed copy distinct from the local \
                 copy.",
                Some(remainder(
                    "the managed copy of the session record",
                    ManagedCopy,
                    "within 30 days of the privacy request completing",
                    ComplianceOwner,
                    "Your local copy is removed immediately; the managed copy is deleted when the \
                     privacy request completes.",
                )),
                vec![linkage(
                    PrivacyRequestCase,
                    "Privacy request case",
                    "privacy_case.managed.dsr_0042",
                    "Deletion is driven by an open data-subject request case.",
                )],
                "class=collaboration_session_record data_class=workspace_owned location=managed_copy \
                 retention=fixed_window outcome=deferred state=delete_pending case=privacy_request",
                "Your collaboration session record is deletable now locally; the managed copy \
                 follows the open privacy request and completes within 30 days.",
            ),
            row(
                "retention.row.managed_cloud.0003",
                "ai_retained_evidence_packet",
                "AI retained evidence packet",
                TenantOwned,
                ManagedCopy,
                RegulatoryHold,
                "Retained under a regulatory minimum set by the organization",
                ExportAfterReauth,
                "Export after a managed reauthentication",
                DeleteBlockedHold,
                "Delete is blocked by an active hold",
                Blocked,
                DeleteBlockedByHold,
                Fresh,
                ComplianceOwner,
                REGISTRY_SCHEMA,
                "Governed as a tenant-owned evidence class subject to legal-hold honesty rules.",
                Some(remainder(
                    "the full evidence packet",
                    ManagedCopy,
                    "when the legal hold is released by the compliance owner",
                    ComplianceOwner,
                    "Nothing is deleted while the hold is active; the hold and its owner are named.",
                )),
                vec![linkage(
                    LegalHold,
                    "Legal hold",
                    "hold.managed.lit_2026_07",
                    "An active legal hold blocks deletion of this evidence packet.",
                )],
                "class=ai_retained_evidence_packet data_class=tenant_owned location=managed_copy \
                 retention=regulatory_hold outcome=blocked state=delete_blocked_by_hold hold=active",
                "This AI evidence packet cannot be deleted while a legal hold is active; the hold \
                 and the compliance owner who controls it are named.",
            ),
            row(
                "retention.row.managed_cloud.0004",
                "support_export_packet",
                "Support export packet",
                UserOwned,
                ExportedSnapshot,
                FixedWindow,
                "Retained 7 days after export, then auto-deleted",
                LocalExportNow,
                "Re-export the support packet now",
                LocalDeleteNow,
                "Delete locally now",
                Immediate,
                DeleteReceipted,
                Fresh,
                LocalUser,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle with a durable destruction receipt.",
                None,
                vec![linkage(
                    DestructionReceipt,
                    "Destruction receipt",
                    "receipt.managed.destroy_0091",
                    "A destruction receipt proves this support packet was deleted.",
                )],
                "class=support_export_packet data_class=user_owned location=exported_snapshot \
                 retention=fixed_window outcome=immediate state=delete_receipted receipt=present",
                "Your support export packet was deleted immediately and carries a destruction \
                 receipt proving it is gone.",
            ),
        ],
        AdminPathClass::SelfHosted => vec![
            row(
                "retention.row.self_hosted.0001",
                "operational_audit_record",
                "Operational audit record",
                TenantOwned,
                ManagedCopy,
                RegulatoryHold,
                "Retained under the self-hosted regulatory minimum",
                LocalExportNow,
                "Export from the self-hosted control plane now",
                DeleteBlockedHold,
                "Delete is blocked by an active hold",
                Blocked,
                DeleteBlockedByHold,
                Fresh,
                SecurityOwner,
                REGISTRY_SCHEMA,
                "Governed as a tenant-owned audit class on the customer's own control plane.",
                Some(remainder(
                    "the operational audit history",
                    ManagedCopy,
                    "when the security owner releases the hold",
                    SecurityOwner,
                    "Audit history is retained under the hold; the security owner controls release.",
                )),
                vec![linkage(
                    LegalHold,
                    "Legal hold",
                    "hold.self_hosted.audit_min",
                    "A regulatory hold blocks deletion of the audit history.",
                )],
                "class=operational_audit_record data_class=tenant_owned location=managed_copy \
                 retention=regulatory_hold outcome=blocked state=delete_blocked_by_hold hold=active",
                "Operational audit history is retained under a regulatory hold on the self-hosted \
                 plane; the security owner controls when it can be released.",
            ),
            row(
                "retention.row.self_hosted.0002",
                "portable_state_package",
                "Portable state package",
                UserOwned,
                LocalOnly,
                UserControlled,
                "Kept locally until you delete it",
                LocalExportNow,
                "Export the portable package now",
                LocalDeleteNow,
                "Delete locally now",
                Immediate,
                DeleteReceipted,
                Fresh,
                LocalUser,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle with a local destruction receipt.",
                None,
                vec![linkage(
                    DestructionReceipt,
                    "Destruction receipt",
                    "receipt.self_hosted.destroy_0007",
                    "A local destruction receipt proves this package was deleted.",
                )],
                "class=portable_state_package data_class=user_owned location=local_only \
                 retention=user_controlled outcome=immediate state=delete_receipted receipt=present",
                "Your portable state package was deleted locally and immediately, with a \
                 destruction receipt proving it is gone.",
            ),
            row(
                "retention.row.self_hosted.0003",
                "collaboration_review_evidence",
                "Collaboration review evidence",
                WorkspaceOwned,
                ManagedCopy,
                FixedWindow,
                "Retained 30 days, then auto-deleted",
                LocalExportNow,
                "Export the review evidence now",
                DeleteViaRequest,
                "File a deletion request",
                Deferred,
                DeletePending,
                Fresh,
                SecurityOwner,
                REGISTRY_SCHEMA,
                "Governed as a workspace-owned review class whose derived index is rebuilt on a \
                 schedule.",
                Some(remainder(
                    "derived references in the review search index",
                    ManagedCopy,
                    "on the next nightly reindex",
                    SecurityOwner,
                    "The primary record is removed now; derived index references clear on the next \
                     reindex, so the delete is partial until then.",
                )),
                vec![linkage(
                    PartialDeleteReason,
                    "Partial-delete reason",
                    "partial.self_hosted.index_rebuild",
                    "Derived index references persist until the next scheduled reindex.",
                )],
                "class=collaboration_review_evidence data_class=workspace_owned location=managed_copy \
                 retention=fixed_window outcome=deferred state=delete_pending partial=index_rebuild",
                "The review evidence record is deleted now, but derived references in the search \
                 index clear on the next nightly reindex, so the delete is partial until then.",
            ),
        ],
        AdminPathClass::SovereignAirGapped => vec![
            row(
                "retention.row.sovereign.0001",
                "ai_retained_evidence_packet",
                "AI retained evidence packet",
                TenantOwned,
                LocalOnly,
                RegulatoryHold,
                "Retained under the sovereign regulatory minimum",
                ExportViaOffboarding,
                "Export through the offboarding flow",
                DeleteBlockedHold,
                "Delete is blocked by an active hold",
                Blocked,
                DeleteBlockedByHold,
                Recent,
                ComplianceOwner,
                REGISTRY_SCHEMA,
                "Governed as a tenant-owned evidence class under a sealed offline bundle.",
                Some(remainder(
                    "the sealed evidence packet",
                    LocalOnly,
                    "when the offline hold seal is lifted by the compliance owner",
                    ComplianceOwner,
                    "The packet stays sealed locally under the hold; the compliance owner controls \
                     the seal.",
                )),
                vec![linkage(
                    LegalHold,
                    "Legal hold",
                    "hold.offline.seal_a1",
                    "A sealed offline hold blocks deletion of the evidence packet.",
                )],
                "class=ai_retained_evidence_packet data_class=tenant_owned location=local_only \
                 retention=regulatory_hold outcome=blocked state=delete_blocked_by_hold hold=sealed",
                "On this air-gapped install the AI evidence packet is sealed under a hold and \
                 cannot be deleted; the compliance owner controls the seal.",
            ),
            row(
                "retention.row.sovereign.0002",
                "imported_audit_snapshot",
                "Imported audit snapshot",
                Imported,
                LocalOnly,
                MirrorLastSynced,
                "Read-only imported snapshot with no live source",
                LocalExportNow,
                "Re-export the imported snapshot now",
                LocalDeleteNow,
                "Delete the local imported copy now",
                Immediate,
                UnconfirmedStale,
                Stale,
                ComplianceOwner,
                LIFECYCLE_SCHEMA,
                "Governed as an imported class; its upstream delete state cannot be confirmed \
                 offline.",
                None,
                vec![],
                "class=imported_audit_snapshot data_class=imported location=local_only \
                 retention=mirror_last_synced outcome=immediate state=unconfirmed_stale source=none",
                "This imported audit snapshot can be deleted locally now, but its upstream status \
                 is unconfirmed offline and is labeled as such rather than shown confirmed.",
            ),
            row(
                "retention.row.sovereign.0003",
                "derived_offline_index",
                "Derived offline index",
                DerivedCache,
                LocalOnly,
                EphemeralRegenerable,
                "Regenerated on demand; not separately retained",
                ExportNotApplicable,
                "No export — regenerated from local records",
                LocalDeleteNow,
                "Delete the cache locally now",
                Immediate,
                ActiveEnforced,
                Fresh,
                LocalUser,
                REGISTRY_SCHEMA,
                "Governed as a derived-cache class that is not a system of record.",
                None,
                vec![],
                "class=derived_offline_index data_class=derived_cache location=local_only \
                 retention=ephemeral_regenerable outcome=immediate state=active_enforced source=local",
                "The derived offline index is a regenerable cache, not a record; it deletes locally \
                 and immediately and is rebuilt on demand.",
            ),
        ],
        AdminPathClass::MirroredOffline => vec![
            row(
                "retention.row.mirrored.0001",
                "managed_copy_index_entry",
                "Managed copy index entry",
                TenantOwned,
                MirroredCopy,
                MirrorLastSynced,
                "Last-synced mirror copy retained until the next sync",
                ExportFromMirror,
                "Export from the last-synced mirror",
                DeleteOnReconnect,
                "Deletion is queued to complete on reconnect",
                Deferred,
                ExportDeferred,
                Stale,
                OrgAdmin,
                REGISTRY_SCHEMA,
                "Governed as a tenant-owned managed-copy index whose authoritative copy is \
                 upstream.",
                Some(remainder(
                    "the upstream managed copy",
                    ManagedCopy,
                    "when the mirror reconnects to the control plane",
                    OrgAdmin,
                    "The mirror entry is queued for deletion; the authoritative upstream copy is \
                     removed when the mirror reconnects.",
                )),
                vec![linkage(
                    PartialDeleteReason,
                    "Partial-delete reason",
                    "partial.mirror.upstream_offline",
                    "The upstream managed copy persists until the mirror reconnects.",
                )],
                "class=managed_copy_index_entry data_class=tenant_owned location=mirrored_copy \
                 retention=mirror_last_synced outcome=deferred state=export_deferred partial=offline",
                "While the mirror is offline this entry is exportable from the last sync and its \
                 deletion is queued to finish upstream when the mirror reconnects.",
            ),
            row(
                "retention.row.mirrored.0002",
                "sync_mirror_ledger",
                "Sync mirror ledger",
                DerivedCache,
                MirroredCopy,
                MirrorLastSynced,
                "Derived mirror ledger; rebuilt on the next sync",
                ExportNotApplicable,
                "No export — derived from the sync",
                LocalDeleteNow,
                "Delete the local ledger cache now",
                Immediate,
                UnconfirmedStale,
                Stale,
                OrgAdmin,
                REGISTRY_SCHEMA,
                "Governed as a derived-cache ledger; its freshness depends on the last sync.",
                None,
                vec![],
                "class=sync_mirror_ledger data_class=derived_cache location=mirrored_copy \
                 retention=mirror_last_synced outcome=immediate state=unconfirmed_stale source=mirror",
                "The sync mirror ledger is a derived cache shown as last-known while the mirror is \
                 offline; it deletes locally now and rebuilds on the next sync.",
            ),
            row(
                "retention.row.mirrored.0003",
                "offboarding_exit_packet",
                "Offboarding exit packet",
                UserOwned,
                LocalOnly,
                UserControlled,
                "Kept locally until you delete it",
                LocalExportNow,
                "Export the exit packet now",
                LocalDeleteNow,
                "Delete locally now",
                Immediate,
                ExportAvailableNow,
                Fresh,
                LocalUser,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle as a user-owned exit packet.",
                None,
                vec![],
                "class=offboarding_exit_packet data_class=user_owned location=local_only \
                 retention=user_controlled outcome=immediate state=export_available_now source=local",
                "Your offboarding exit packet stays on this machine, is exportable now even with \
                 the mirror offline, and deletes immediately.",
            ),
        ],
        _ => Vec::new(),
    }
}

fn build_export_forms(profile: AdminPathClass) -> Vec<ExportForm> {
    let profile_token = profile.as_str();
    vec![
        ExportForm {
            format: ExportFormatClass::MachineReadableJson,
            label: "Machine-readable summary".to_owned(),
            artifact_ref: format!("retention.export.{profile_token}.machine"),
            redaction: AdminRedactionClass::MetadataSafeDefault,
            description: "Each row's stable record-class token, data class, location, retention, \
                          routes, outcome, and state as JSON summary objects, copyable or \
                          exportable for tooling."
                .to_owned(),
        },
        ExportForm {
            format: ExportFormatClass::PlainLanguageHandoff,
            label: "Plain-language handoff packet".to_owned(),
            artifact_ref: format!("retention.export.{profile_token}.handoff"),
            redaction: AdminRedactionClass::ComplianceRestricted,
            description: "The same rows as reviewable plain-language sentences for a support, \
                          offboarding, or compliance handoff, with no raw payloads."
                .to_owned(),
        },
    ]
}

fn build_propagation() -> Vec<PropagationTarget> {
    let target = |target: PropagationTargetClass, note: &str| PropagationTarget {
        target,
        label: target.label().to_owned(),
        note: note.to_owned(),
    };
    vec![
        target(
            PropagationTargetClass::SupportExport,
            "The same retention/delete states are embedded verbatim in the support export.",
        ),
        target(
            PropagationTargetClass::OffboardingFlow,
            "The offboarding flow reads these rows for its export-before-delete and continuity \
             steps.",
        ),
        target(
            PropagationTargetClass::CompliancePacket,
            "Compliance and procurement packets cite these rows as the retention/delete posture.",
        ),
        target(
            PropagationTargetClass::HelpAboutPublicTruth,
            "Help/About public-truth surfaces restate these states without re-deriving them.",
        ),
    ]
}

fn build_coverage(profile: AdminPathClass) -> RetentionCoverage {
    use AdminStateClass::*;
    use CompletenessClass::*;

    match profile {
        AdminPathClass::ManagedCloud => RetentionCoverage {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "All managed artifact families — live".to_owned(),
            coverage_note: "The managed control plane is live; every claimed artifact family is \
                            listed and its delete/export state is confirmed."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        AdminPathClass::SelfHosted => RetentionCoverage {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "All managed artifact families — self-hosted".to_owned(),
            coverage_note:
                "The customer's own control plane governs retention; the full matrix is \
                            inspectable on this machine without any vendor console."
                    .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        AdminPathClass::SovereignAirGapped => RetentionCoverage {
            coverage_state: UnconfirmedStale,
            completeness: PartialImported,
            window_label: "Signed offline bundle — imported, no live tail".to_owned(),
            coverage_note: "Retention/deletion truth comes from the sealed offline bundle and \
                            imported snapshots; the upstream tail is absent and the view is \
                            labeled unconfirmed rather than implied complete."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        AdminPathClass::MirroredOffline => RetentionCoverage {
            coverage_state: UnconfirmedStale,
            completeness: PartialOffline,
            window_label: "Last synced — mirror offline".to_owned(),
            coverage_note: "The mirror is offline, so states past the last sync are labeled \
                            unconfirmed and queued deletes complete on reconnect; the recorded \
                            rows remain locally inspectable."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
        _ => RetentionCoverage {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Local".to_owned(),
            coverage_note: "Local retention matrix.".to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
        },
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RetentionDeletionInvariant {
    RetentionDeletionInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(profiles: &[RetentionDeletionPacket]) -> Vec<RetentionDeletionInvariant> {
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| -> bool {
        matrix
            .surface(AdminSurfaceClass::RetentionDeletionMatrix)
            .is_some_and(|entry| entry.applicable_states.contains(&state))
    };
    let declared_consumers = parity_consumers();
    let all_rows = || profiles.iter().flat_map(|p| p.matrix.rows.iter());

    let mut out = Vec::new();

    // Every rendered state is one the matrix admits for this surface.
    out.push(invariant(
        "retention_deletion.surface_states_within_matrix",
        "Every state a row or the coverage posture shows is one the frozen admin-plane matrix \
         declares applicable for the retention/deletion surface, so the render layer cannot drift \
         from the contract.",
        profiles.iter().all(|p| {
            p.matrix.rows.iter().all(|r| admitted(r.machine_state))
                && admitted(p.matrix.coverage.coverage_state)
        }),
    ));

    // Every row names its retention class, routes, outcome, state, owner, schema
    // (acceptance criterion: retention class, delete/export route, current outcome
    // locally in-product).
    out.push(invariant(
        "retention_deletion.retention_route_outcome_complete",
        "Every row names a stable id and record family, a data class, a location, a default \
         retention, an export route and a delete route, a current immediate/deferred/blocked \
         outcome, a machine-readable state, an owner, and a governing schema, so each claimed \
         managed artifact family shows its retention class, delete/export route, and outcome \
         locally.",
        all_unique(all_rows().map(|r| r.row_id.as_str()))
            && all_rows().all(|r| {
                !r.row_id.is_empty()
                    && !r.record_family.is_empty()
                    && !r.retention_label.is_empty()
                    && !r.export_label.is_empty()
                    && !r.delete_label.is_empty()
                    && !r.governing_schema_ref.is_empty()
                    && !r.schema_note.is_empty()
            }),
    ));

    // Data classes are distinguished, never collapsed into one bucket.
    out.push(invariant(
        "retention_deletion.data_classes_distinguished",
        "Every row names a specific data class — user-owned, workspace-owned, tenant-owned, \
         imported, or derived-cache — and across the bundle every class appears at least once, so \
         artifacts are not flattened into one generic bucket.",
        ArtifactOwnerClass::ALL.iter().all(|class| {
            profiles
                .iter()
                .any(|p| p.matrix.rows.iter().any(|r| r.data_class == *class))
        }),
    ));

    // A non-immediate delete explains its remainder (acceptance criterion: a
    // deletion that cannot complete immediately explains what/where/who).
    out.push(invariant(
        "retention_deletion.non_immediate_explains_remainder",
        "Every row whose delete outcome is deferred or blocked carries a remainder that names what \
         remains, where it remains, when it is expected to complete, and who controls the next \
         step; an immediate delete carries no remainder.",
        all_rows().all(|r| {
            if r.delete_outcome.requires_remainder() {
                r.remainder.as_ref().is_some_and(|rem| {
                    !rem.what_remains.is_empty() && !rem.expected_completion.is_empty()
                })
            } else {
                r.remainder.is_none()
            }
        }),
    ));

    // Deletion linkages stay distinct and every class appears across the bundle.
    out.push(invariant(
        "retention_deletion.deletion_linkage_distinct",
        "Every deferred or blocked delete links to at least one specific linkage — destruction \
         receipt, privacy-request case, legal hold, or partial-delete reason — rather than a \
         generic pending status, and across the bundle every linkage class appears at least once.",
        all_rows().all(|r| !r.delete_outcome.requires_remainder() || !r.linkages.is_empty())
            && DeletionLinkageClass::ALL.iter().all(|class| {
                profiles
                    .iter()
                    .any(|p| p.matrix.rows.iter().any(|r| r.has_linkage(*class)))
            }),
    ));

    // Delete/export honesty: receipted deletes carry their receipt; hold-blocked
    // deletes name their hold.
    out.push(invariant(
        "retention_deletion.delete_export_honest",
        "A row shown as receipted carries a destruction receipt and a row shown as blocked by a \
         hold names that hold, so a class is never shown deleted without a receipt and a hold \
         blocks deletion and says so.",
        all_rows().all(|r| {
            let receipt_ok = r.machine_state != AdminStateClass::DeleteReceipted
                || r.has_linkage(DeletionLinkageClass::DestructionReceipt);
            let hold_ok = r.machine_state != AdminStateClass::DeleteBlockedByHold
                || r.has_linkage(DeletionLinkageClass::LegalHold);
            receipt_ok && hold_ok
        }),
    ));

    // Location explicit: the local-only versus hosted distinction is exercised, so
    // a local-only artifact is labeled distinctly from a hosted copy.
    out.push(invariant(
        "retention_deletion.location_explicit",
        "Across the bundle rows name both local-only locations and hosted (managed, mirrored, \
         shared, or exported) locations, so a local-only artifact is labeled distinctly from a \
         hosted copy rather than blurred into one location.",
        all_rows().any(|r| r.location == DataResidencyClass::LocalOnly)
            && all_rows().any(|r| r.location != DataResidencyClass::LocalOnly),
    ));

    // Export parity: machine summary and plain-language handoff on every row, both
    // export forms offered (so states propagate unchanged on export).
    out.push(invariant(
        "retention_deletion.export_parity",
        "Every row carries both an export-safe machine-readable summary and a plain-language \
         support/admin handoff sentence, and every matrix offers both export forms.",
        profiles.iter().all(|p| {
            p.matrix.rows.iter().all(RetentionRow::has_export_parity)
                && p.matrix.offers(ExportFormatClass::MachineReadableJson)
                && p.matrix.offers(ExportFormatClass::PlainLanguageHandoff)
        }),
    ));

    // States propagate unchanged into support, offboarding, compliance, Help/About.
    out.push(invariant(
        "retention_deletion.propagation_complete",
        "Every matrix names propagation into support export, offboarding, compliance packet, and \
         Help/About public truth, so retention/delete states reach those surfaces unchanged rather \
         than being re-derived.",
        profiles.iter().all(|p| {
            PropagationTargetClass::ALL
                .iter()
                .all(|target| p.matrix.propagates_to(*target))
        }),
    ));

    // No-silent-green: stale evidence never sits under a confirmed positive claim.
    out.push(invariant(
        "retention_deletion.no_silent_green",
        "A row whose backing evidence is stale is never shown under a confirmed active/enforced, \
         export-available-now, or receipted state; stale rows use an explicit non-confirmed state \
         instead.",
        all_rows()
            .all(|r| !(r.evidence_age.is_stale() && requires_fresh_evidence(r.machine_state))),
    ));

    // Ownership stays visible and a blocked delete escalates beyond the local user
    // to a governance owner who controls the next step.
    out.push(invariant(
        "retention_deletion.ownership_visible",
        "Every blocked delete names a next-step owner other than the local user, so a hold-blocked \
         delete escalates to the workspace, org, security, or compliance owner who actually \
         controls release rather than implying the user can act alone.",
        all_rows().all(|r| {
            if r.delete_outcome != DeleteOutcomeClass::Blocked {
                return true;
            }
            r.remainder
                .as_ref()
                .is_some_and(|rem| rem.next_step_owner != OwnerEscalationRoleClass::LocalUser)
        }),
    ));

    // Locally inspectable without a vendor console on every profile.
    out.push(invariant(
        "retention_deletion.locally_inspectable_offline",
        "Every profile — including self-hosted, sovereign/air-gapped, and mirrored/offline — keeps \
         a locally inspectable retention/deletion matrix that does not require a vendor console or \
         control plane.",
        profiles.iter().all(|p| {
            p.matrix.coverage.locally_inspectable && p.matrix.coverage.vendor_console_independent
        }),
    ));

    // Partial registry views are labeled, never implied complete.
    out.push(invariant(
        "retention_deletion.coverage_labeled",
        "A registry view that is offline, imported, or redaction-limited is labeled with a \
         non-complete completeness class and a coverage note, so a partial matrix is never \
         presented as complete.",
        profiles.iter().all(|p| {
            let coverage = &p.matrix.coverage;
            !coverage.coverage_note.is_empty()
                && (!coverage.completeness.is_partial()
                    || coverage.coverage_state != AdminStateClass::ActiveEnforced)
        }),
    ));

    // Cross-surface parity: one typed packet serves every declared consumer.
    out.push(invariant(
        "retention_deletion.consumer_parity",
        "Each profile is one typed packet consumed identically by every consumer the matrix \
         declares for the retention/deletion surface, so the matrix is identical across UI, CLI, \
         Help/About, support export, and procurement surfaces by construction.",
        !declared_consumers.is_empty()
            && profiles
                .iter()
                .all(|p| declared_consumers.iter().all(|c| p.consumers.contains(c))),
    ));

    // Every claimed managed-bearing profile is rendered.
    out.push(invariant(
        "retention_deletion.profiles_covered",
        "The bundle renders the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles.",
        RETENTION_PROFILES
            .iter()
            .all(|profile| profiles.iter().any(|p| p.profile == *profile)),
    ));

    // Delete outcomes are all exercised — proof the block/defer/immediate
    // distinction is real, not a single status.
    out.push(invariant(
        "retention_deletion.outcomes_all_present",
        "Across the bundle every delete outcome — immediate, deferred, and blocked — appears at \
         least once, so the block/defer/immediate distinction is real rather than collapsed into \
         one pending status.",
        DeleteOutcomeClass::ALL.iter().all(|outcome| {
            profiles
                .iter()
                .any(|p| p.matrix.rows.iter().any(|r| r.delete_outcome == *outcome))
        }),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "retention_deletion.export_safe",
        "Every stable surface, profile, row, record-family, linkage, and export id is an opaque \
         token with no URL scheme or absolute path, and every governing schema is a repo-relative \
         ref, so the bundle is safe to embed in a support export verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && is_safe_token(p.matrix.surface_id.as_str())
                && p.matrix.rows.iter().all(|r| {
                    is_safe_token(r.row_id.as_str())
                        && is_safe_token(r.record_family.as_str())
                        && is_export_safe_ref(r.governing_schema_ref.as_str())
                        && r.linkages
                            .iter()
                            .all(|l| is_safe_token(l.linked_ref.as_str()))
                })
                && p.matrix
                    .export_forms
                    .iter()
                    .all(|x| is_safe_token(x.artifact_ref.as_str()))
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn retention_deletion_lines(bundle: &RetentionDeletionBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Retention/deletion bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Profiles: {}  Invariants: {}  (binds matrix {})",
        bundle.profiles.len(),
        bundle.invariants.len(),
        bundle.matrix_id,
    ));

    for p in &bundle.profiles {
        lines.push(format!("Profile {} [{}]", p.profile.as_str(), p.profile_id));
        lines.push(format!("  {}", p.summary));
        let coverage = &p.matrix.coverage;
        lines.push(format!(
            "  Coverage: state={} completeness={} window={} local={} console_independent={}",
            coverage.coverage_state.as_str(),
            coverage.completeness.as_str(),
            coverage.window_label,
            coverage.locally_inspectable,
            coverage.vendor_console_independent,
        ));
        lines.push("  Rows:".to_owned());
        for r in &p.matrix.rows {
            lines.push(format!(
                "    - {} [{}] data_class={} location={} retention={} export={} delete={} \
                 outcome={} state={} age={} owner={}",
                r.row_id,
                r.record_family,
                r.data_class.as_str(),
                r.location.as_str(),
                r.retention.as_str(),
                r.export_route.as_str(),
                r.delete_route.as_str(),
                r.delete_outcome.as_str(),
                r.machine_state.as_str(),
                r.evidence_age.as_str(),
                r.owner.as_str(),
            ));
            lines.push(format!("        {}", r.plain_language));
            if let Some(rem) = &r.remainder {
                lines.push(format!(
                    "        remains: {} @ {} → {} (owner {})",
                    rem.what_remains,
                    rem.where_it_remains.as_str(),
                    rem.expected_completion,
                    rem.next_step_owner.as_str(),
                ));
            }
            for l in &r.linkages {
                lines.push(format!(
                    "        link[{}] {} → {}",
                    l.linkage.as_str(),
                    l.label,
                    l.linked_ref,
                ));
            }
        }
        lines.push("  Propagates into:".to_owned());
        for t in &p.matrix.propagation {
            lines.push(format!("    - {} ({})", t.label, t.target.as_str()));
        }
        lines.push("  Export forms:".to_owned());
        for x in &p.matrix.export_forms {
            lines.push(format!("    - {} [{}]", x.label, x.format.as_str()));
        }
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
