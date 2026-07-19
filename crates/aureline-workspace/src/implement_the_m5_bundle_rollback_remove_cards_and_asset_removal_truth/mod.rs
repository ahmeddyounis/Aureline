//! Implements the reusable bundle rollback / remove primitive: a rollback / remove card, an
//! asset inventory that distinguishes bundle-created assets from user-created files, profiles,
//! local history, imported settings, and adopted packages, and a restore path carrying a
//! checkpoint restore and an export-before-remove action — all resolving from one removal context
//! and sharing one removal identity, so workspace, bundle, extension, migration, diagnostics, and
//! support surfaces explain the *same removal truth*: what backing out of a guided stack actually
//! reverts, what remains, and what a user must handle manually, before a bundle is rolled back or
//! removed.
//!
//! Where
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]
//! *freezes* the reusable workflow-bundle component families as a governed contract, and
//! [`crate::implement_the_m5_bundle_drift_banners_and_local_override_rows`] narrows the drift /
//! override / rollback-card families into a drift resolver, this module *narrows* the
//! [`M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard`]
//! ([`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily`])
//! family into a dedicated **removal** primitive with a real **resolver**. A single removal
//! context projects onto a rollback / remove card, a created-versus-adopted asset inventory, and a
//! restore path that share one removal identity, so a bundle's removal never blurs bundle-owned
//! cleanup with the user-created work, profiles, local history, imported settings, and adopted
//! packages that must survive removal unless explicitly selected.
//!
//! The resolver reuses the canonical review / rollback vocabulary already carried by
//! [`crate::m5_bundle_review_and_rollback`] ([`AssetOwnership`], [`BundleReviewOperation`],
//! [`RollbackCheckpoint`]), the side-effect vocabulary already minted by the bundle-review
//! primitive ([`M5BundleSideEffectClass`]), and the manifest / scorecard / governance vocabulary —
//! never a bespoke per-flow removal model. It adds only the removal-specific vocabulary the
//! resolver needs: the created-versus-adopted asset origin ([`M5RemovalAssetOrigin`]), the
//! safe-to-remove classes ([`M5SafeToRemoveClass`]), and the shared rollback / remove disposition
//! vocabulary ([`M5RemovalDisposition`]).
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — bundle removal no longer implies destructive cleanup of user work or hidden state
//!   roots.** Every asset is attributed to a created-versus-adopted origin, and a user-owned asset
//!   (a user-created file, profile, local history, imported setting, or adopted package) is never
//!   reverted unless the user explicitly selects it; a card that reads as destructive cleanup is
//!   rejected.
//! - **AC2 — rollback and remove actions state what remains, what is reverted, and what must be
//!   handled manually.** The card partitions the inventory into three explicit lists — kept-local
//!   (what remains), reverted (what is rolled back), and manual-follow-up (what must be handled
//!   manually) — each derived from an asset's safe-to-remove class.
//! - **AC3 — export-before-remove and checkpoint restore are available wherever removal could
//!   narrow support or portability truth.** A mutating remove creates a one-step rollback
//!   checkpoint before it commits, and whenever removal touches user-owned, imported, or stale
//!   state, an export-before-remove action captures it so nothing portable is lost; the card never
//!   forces a bundle reset to make removal exportable.
//!
//! Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors never
//! cross this boundary; the resolver carries only opaque refs, typed class tokens, booleans, and
//! redacted labels, so support and diagnostics exports reconstruct exactly what a surface would
//! have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-bundle-rollback-remove-primitive.schema.json`](../../../../schemas/ui/m5-bundle-rollback-remove-primitive.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_bundle_rollback_remove_primitive.md`](../../../../docs/bundles/m5_bundle_rollback_remove_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the primitive binds to the freeze matrix's truth-mode,
// downgrade-trigger, and degraded-state tokens rather than mint parallel ones.
use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    DegradedState, M5BundleComponentDowngradeTrigger, M5BundleTruthMode,
};
// Reused side-effect vocabulary already minted by the bundle-review primitive: the rollback /
// remove card names removal side effects with the same closed class set the review sheet uses.
use crate::implement_the_m5_bundle_detail_pages_and_install_update_review_sheets::M5BundleSideEffectClass;
// Reused canonical review / rollback vocabulary — the removal primitive binds to the same
// ownership, operation, and checkpoint model the install / update / remove / drift review flows
// already carry.
use crate::m5_bundle_review_and_rollback::{
    AssetOwnership, BundleReviewOperation, RollbackCheckpoint,
};
// Reused canonical bundle / scorecard / governance vocabulary already carried by the frozen
// bundle-manifest, scorecard, and entry-governance contracts.
use crate::m5_bundle_scorecards::{
    BundleScorecardClass, EvidenceFreshness, ImportedVsNativeConfidence,
};
use crate::m5_entry_and_bundle_governance::{BundleClass, SourceTrust};
use crate::m5_workflow_bundle_manifests::{
    BundleComponentKind, CertificationTarget, LifecycleStage,
};

/// Stable record-kind tag carried by [`M5BundleRollbackRemovePacket`].
pub const M5_BUNDLE_REMOVAL_RECORD_KIND: &str = "m5_bundle_rollback_remove_primitive";

/// Schema version for the bundle rollback / remove primitive packet.
pub const M5_BUNDLE_REMOVAL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_BUNDLE_REMOVAL_SCHEMA_REF: &str =
    "schemas/ui/m5-bundle-rollback-remove-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUNDLE_REMOVAL_DOC_REF: &str = "docs/bundles/m5_bundle_rollback_remove_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_BUNDLE_REMOVAL_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUNDLE_REMOVAL_FIXTURE_DIR: &str = "fixtures/ui/m5-bundle-rollback-remove-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const M5_BUNDLE_REMOVAL_ARTIFACT_REF: &str =
    "artifacts/release/m5-bundle-rollback-remove-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_BUNDLE_REMOVAL_CSV_REF: &str =
    "artifacts/release/m5-bundle-rollback-remove-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BUNDLE_REMOVAL_REPORT_REF: &str =
    "artifacts/release/m5-bundle-rollback-remove-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed bundle-removal surface family. Each family is one parity surface that ingests the shared
/// primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleRemovalSurfaceFamily {
    /// The workspace rollback card shown when a guided stack is backed out.
    WorkspaceRollbackCard,
    /// The bundle detail remove panel reviewing a removal for one bundle in full.
    BundleDetailRemovePanel,
    /// The extension remove row shown in an extension / capability list.
    ExtensionRemoveRow,
    /// The migration rollback view previewing what removing an imported bundle would do.
    MigrationRollbackView,
    /// The diagnostics removal report used for triage / support handoff.
    DiagnosticsRemovalReport,
    /// The support / export replay surface reconstructing removal truth offline.
    SupportExportReplay,
}

impl M5BundleRemovalSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceRollbackCard,
        Self::BundleDetailRemovePanel,
        Self::ExtensionRemoveRow,
        Self::MigrationRollbackView,
        Self::DiagnosticsRemovalReport,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRollbackCard => "workspace_rollback_card",
            Self::BundleDetailRemovePanel => "bundle_detail_remove_panel",
            Self::ExtensionRemoveRow => "extension_remove_row",
            Self::MigrationRollbackView => "migration_rollback_view",
            Self::DiagnosticsRemovalReport => "diagnostics_removal_report",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRollbackCard => "Workspace rollback card",
            Self::BundleDetailRemovePanel => "Bundle detail remove panel",
            Self::ExtensionRemoveRow => "Extension remove row",
            Self::MigrationRollbackView => "Migration rollback view",
            Self::DiagnosticsRemovalReport => "Diagnostics removal report",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// The created-versus-adopted asset origin. Every asset in a removal inventory is attributed to
/// exactly one origin so bundle-created cleanup is never confused with the user-created files,
/// profiles, local history, imported settings, or adopted packages that must survive removal
/// unless explicitly selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemovalAssetOrigin {
    /// Created and owned by the bundle; removed with the bundle.
    BundleCreated,
    /// A file the user authored; survives removal unless explicitly selected.
    UserCreatedFile,
    /// A user profile / settings / layout preset; survives removal.
    UserProfile,
    /// Local history / timeline the user accrued; survives removal.
    LocalHistory,
    /// A setting imported from another tool / handoff; survives removal.
    ImportedSetting,
    /// A package the user adopted as their own; survives removal unless explicitly selected.
    AdoptedPackage,
}

impl M5RemovalAssetOrigin {
    /// Every origin, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundleCreated,
        Self::UserCreatedFile,
        Self::UserProfile,
        Self::LocalHistory,
        Self::ImportedSetting,
        Self::AdoptedPackage,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleCreated => "bundle_created",
            Self::UserCreatedFile => "user_created_file",
            Self::UserProfile => "user_profile",
            Self::LocalHistory => "local_history",
            Self::ImportedSetting => "imported_setting",
            Self::AdoptedPackage => "adopted_package",
        }
    }

    /// Whether this origin carries durable user state that removal must preserve unless the user
    /// explicitly selects it. Everything except a bundle-created asset is user-owned.
    pub const fn is_user_owned(self) -> bool {
        !matches!(self, Self::BundleCreated)
    }
}

/// The safe-to-remove class of an asset. A bundle-created asset is safe to remove with the bundle;
/// a user-owned asset is kept local or requires manual handling, never silently removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SafeToRemoveClass {
    /// A bundle-owned asset safe to remove with the bundle.
    SafeToRemove,
    /// A user-owned asset preserved on removal unless explicitly selected.
    KeepLocal,
    /// A user-owned asset whose removal has dependents and must be handled manually.
    RequiresManualHandling,
}

impl M5SafeToRemoveClass {
    /// Every safe-to-remove class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SafeToRemove,
        Self::KeepLocal,
        Self::RequiresManualHandling,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeToRemove => "safe_to_remove",
            Self::KeepLocal => "keep_local",
            Self::RequiresManualHandling => "requires_manual_handling",
        }
    }

    /// Whether this class is honest for the given origin: a bundle-created asset is always
    /// safe-to-remove; a user-owned asset is never safe-to-remove (it is kept-local or requires
    /// manual handling).
    pub const fn is_honest_for_origin(self, origin: M5RemovalAssetOrigin) -> bool {
        match self {
            Self::SafeToRemove => !origin.is_user_owned(),
            Self::KeepLocal | Self::RequiresManualHandling => origin.is_user_owned(),
        }
    }
}

/// The shared rollback / remove disposition vocabulary. Every surface — card, docs, help, and
/// export — names what happens to an asset on removal with the same closed set: reverted (rolled
/// back with the bundle), kept-local (survives), or manual-follow-up (must be handled manually).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemovalDisposition {
    /// The asset is reverted / rolled back with the bundle.
    Reverted,
    /// The asset remains; it survives removal.
    KeptLocal,
    /// The asset must be handled manually by the user.
    ManualFollowUp,
}

impl M5RemovalDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [Self::Reverted, Self::KeptLocal, Self::ManualFollowUp];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reverted => "reverted",
            Self::KeptLocal => "kept_local",
            Self::ManualFollowUp => "manual_follow_up",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must carry per
/// surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleRemovalExportField {
    /// The stable removal identity shared across surfaces.
    RemovalId,
    /// The opaque bundle identity ref and human name.
    BundleIdentity,
    /// The operation the card reviews (remove / rollback / removal preview).
    Operation,
    /// The created-versus-adopted asset inventory.
    AssetInventory,
    /// The safe-to-remove classes the inventory carries.
    SafeToRemoveClasses,
    /// The one-step checkpoint restore path.
    RollbackCheckpoint,
    /// The export-before-remove action.
    ExportBeforeRemove,
    /// The mirror / offline posture of the source.
    MirrorOfflinePosture,
}

impl M5BundleRemovalExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RemovalId,
        Self::BundleIdentity,
        Self::Operation,
        Self::AssetInventory,
        Self::SafeToRemoveClasses,
        Self::RollbackCheckpoint,
        Self::ExportBeforeRemove,
        Self::MirrorOfflinePosture,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::RemovalId,
        Self::BundleIdentity,
        Self::Operation,
        Self::AssetInventory,
        Self::ExportBeforeRemove,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemovalId => "removal_id",
            Self::BundleIdentity => "bundle_identity",
            Self::Operation => "operation",
            Self::AssetInventory => "asset_inventory",
            Self::SafeToRemoveClasses => "safe_to_remove_classes",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::ExportBeforeRemove => "export_before_remove",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
        }
    }
}

/// One removal-inventory asset: one asset attributed to a created-versus-adopted origin, a
/// safe-to-remove class, and a disposition, so removal is never one opaque "cleanup" bucket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemovalAsset {
    /// The created-versus-adopted origin of the asset.
    pub origin: M5RemovalAssetOrigin,
    /// Which content category the asset belongs to.
    pub component_kind: BundleComponentKind,
    /// Opaque ref to the asset; never raw content.
    pub target_ref: String,
    /// A human-readable, one-line asset label.
    pub label: String,
    /// Who owns the asset's current local state (reused review / rollback vocabulary).
    pub ownership: AssetOwnership,
    /// The safe-to-remove class of the asset; must be honest for its origin.
    pub safe_to_remove_class: M5SafeToRemoveClass,
    /// What happens to the asset on removal; must be honest for its class.
    pub disposition: M5RemovalDisposition,
    /// Whether the user explicitly selected this asset for removal. A user-owned asset is only
    /// reverted when this is `true`.
    pub explicitly_selected_for_removal: bool,
}

impl M5RemovalAsset {
    /// Whether the safe-to-remove class is honest for the asset's origin.
    pub fn class_honest(&self) -> bool {
        self.safe_to_remove_class.is_honest_for_origin(self.origin)
    }

    /// Whether the asset's ownership is consistent with its created-versus-adopted origin: a
    /// bundle-created asset is bundle-owned or removable; a user-owned asset is user-protected or
    /// blocked, never silently bundle-owned.
    pub fn ownership_matches_origin(&self) -> bool {
        if self.origin.is_user_owned() {
            self.ownership.is_user_protected() || self.ownership.is_blocked()
        } else {
            matches!(
                self.ownership,
                AssetOwnership::BundleOwned | AssetOwnership::Removable
            )
        }
    }

    /// Whether the disposition is honest for the safe-to-remove class:
    ///
    /// - A safe-to-remove (bundle-created) asset is reverted with the bundle.
    /// - A keep-local asset remains unless the user explicitly selects it for removal.
    /// - A requires-manual-handling asset must be handled manually.
    pub fn disposition_honest(&self) -> bool {
        match self.safe_to_remove_class {
            M5SafeToRemoveClass::SafeToRemove => self.disposition == M5RemovalDisposition::Reverted,
            M5SafeToRemoveClass::KeepLocal => match self.disposition {
                M5RemovalDisposition::KeptLocal => true,
                M5RemovalDisposition::Reverted => self.explicitly_selected_for_removal,
                M5RemovalDisposition::ManualFollowUp => false,
            },
            M5SafeToRemoveClass::RequiresManualHandling => {
                self.disposition == M5RemovalDisposition::ManualFollowUp
            }
        }
    }

    /// Whether the asset preserves user work: a user-owned asset is never reverted unless the user
    /// explicitly selected it (AC1).
    pub fn preserves_user_asset(&self) -> bool {
        !self.origin.is_user_owned()
            || self.disposition != M5RemovalDisposition::Reverted
            || self.explicitly_selected_for_removal
    }

    /// Whether this asset row is internally consistent and attributable.
    pub fn is_consistent(&self) -> bool {
        !self.target_ref.trim().is_empty()
            && !self.label.trim().is_empty()
            && self.class_honest()
            && self.ownership_matches_origin()
            && self.disposition_honest()
            && self.preserves_user_asset()
    }
}

/// The export-before-remove action: an offline-safe export that captures the removal state — and
/// in particular the user-owned assets — so nothing portable is lost before a bundle is removed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExportBeforeRemove {
    /// Opaque ref to the export target; never raw content.
    pub export_ref: String,
    /// A stable format token for the export (e.g. `support_bundle_json`).
    pub format_label: String,
    /// Whether the export captures user-owned assets so they remain portable.
    pub captures_user_assets: bool,
    /// Whether the export action is available on this surface.
    pub available: bool,
}

impl M5ExportBeforeRemove {
    /// Whether the export-before-remove action is available and captures user-owned state.
    pub fn is_available(&self) -> bool {
        !self.export_ref.trim().is_empty()
            && !self.format_label.trim().is_empty()
            && self.available
            && self.captures_user_assets
    }
}

// --- resolver input ---

/// The full input to the bundle removal resolver for one removal context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalInput {
    /// The stable removal identity that must survive across the card, inventory, and restore path.
    pub removal_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// Opaque ref to the bundle id under removal; never raw manifest bytes.
    pub bundle_id_ref: String,
    /// Human-readable bundle name shown on the card.
    pub bundle_name: String,
    /// The bundle class under removal.
    pub bundle_class: BundleClass,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Support-class / lifecycle stage of the bundle.
    pub support_class: LifecycleStage,
    /// The shared source class (certified / managed / community / imported / draft).
    pub source_class: CertificationTarget,
    /// The scorecard class the bundle carries.
    pub scorecard_class: BundleScorecardClass,
    /// Certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// Imported-vs-native confidence contributing to the portability story.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class the removal binds to.
    pub truth_mode: M5BundleTruthMode,
    /// The operation the rollback / remove card reviews (remove / update rollback / removal
    /// preview via drift review).
    pub operation: BundleReviewOperation,
    /// The created-versus-adopted asset inventory (must be non-empty).
    pub assets: Vec<M5RemovalAsset>,
    /// The toolchain / scaffold / settings / docs side effects a rollback / remove carries.
    pub side_effects: Vec<M5BundleSideEffectClass>,
    /// The one-step checkpoint restore path created before a mutating removal. Required for a
    /// mutating remove / update; may be absent for a read-only removal preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint: Option<RollbackCheckpoint>,
    /// The export-before-remove action. Required whenever removal narrows support or portability
    /// truth (user-owned, imported, or stale state); may be absent otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_before_remove: Option<M5ExportBeforeRemove>,
    /// The card reads like destructive cleanup of user work; must be `false` (AC1).
    pub reads_like_destructive_cleanup: bool,
    /// A stale / missing certification is claimed as current; must be `false`.
    pub claims_current_despite_stale: bool,
    /// The card forces a bundle reset to make removal exportable; must be `false` (AC3).
    pub forces_reset_to_export: bool,
    /// An externally-observed narrowing carried through onto the removal before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

impl M5BundleRemovalInput {
    /// Whether this removal narrows support or portability truth, so an export-before-remove
    /// action must be available (AC3): any user-owned asset is touched, the bundle is not fully
    /// native, or the certification is stale / missing.
    pub fn narrows_support_or_portability(&self) -> bool {
        self.assets.iter().any(|a| a.origin.is_user_owned())
            || !matches!(self.imported_confidence, ImportedVsNativeConfidence::Native)
            || matches!(
                self.certification_freshness,
                EvidenceFreshness::Stale | EvidenceFreshness::Missing
            )
    }
}

// --- resolved projections ---

/// The resolved rollback / remove card: three explicit partitions of the inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRemovalCard {
    /// The removal identity — identical to the inventory and restore path.
    pub removal_id: String,
    /// The opaque bundle id ref.
    pub bundle_id_ref: String,
    /// The human-readable bundle name.
    pub bundle_name: String,
    /// The operation the card reviews.
    pub operation: BundleReviewOperation,
    /// The side effects a rollback / remove carries.
    pub side_effects: Vec<M5BundleSideEffectClass>,
    /// What is reverted — the target refs rolled back with the bundle.
    pub reverted: Vec<String>,
    /// What remains — the target refs kept local across the removal.
    pub kept_local: Vec<String>,
    /// What must be handled manually — the target refs requiring manual follow-up.
    pub manual_follow_up: Vec<String>,
    /// The card discloses what remains; always `true`.
    pub discloses_what_remains: bool,
    /// The card discloses what is reverted; always `true`.
    pub discloses_what_is_reverted: bool,
    /// The card discloses what must be handled manually; always `true`.
    pub discloses_manual_follow_up: bool,
    /// The card implies destructive cleanup of user work (AC1); always `false`.
    pub implies_destructive_cleanup: bool,
}

/// The resolved created-versus-adopted asset inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAssetInventory {
    /// The removal identity — identical to the card and restore path.
    pub removal_id: String,
    /// The per-asset rows.
    pub assets: Vec<M5RemovalAsset>,
    /// The origins present across the rows, sorted — proves created-versus-adopted attribution.
    pub origins_present: Vec<M5RemovalAssetOrigin>,
    /// The safe-to-remove classes present across the rows, sorted.
    pub safe_to_remove_classes_present: Vec<M5SafeToRemoveClass>,
    /// The count of bundle-created assets.
    pub bundle_created_count: usize,
    /// The count of user-owned assets.
    pub user_owned_count: usize,
    /// The inventory distinguishes bundle-created assets from user-owned assets (AC1); always
    /// `true`.
    pub distinguishes_created_from_adopted: bool,
    /// The inventory preserves user assets unless explicitly selected (AC1); always `true`.
    pub preserves_user_assets: bool,
    /// The inventory collapses to one opaque removal bucket; always `false`.
    pub collapses_to_opaque_removal: bool,
}

impl M5ResolvedAssetInventory {
    /// True when the inventory carries both a bundle-created asset and a user-owned asset, proving
    /// removal separates what Aureline created from what the user owns.
    pub fn separates_created_from_user_owned(&self) -> bool {
        self.bundle_created_count >= 1 && self.user_owned_count >= 1
    }
}

/// The resolved restore path: checkpoint restore and export-before-remove.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRestorePath {
    /// The removal identity — identical to the card and inventory.
    pub removal_id: String,
    /// The one-step checkpoint restore path created before a mutating removal, when any.
    pub rollback_checkpoint: Option<RollbackCheckpoint>,
    /// The export-before-remove action, when available.
    pub export_before_remove: Option<M5ExportBeforeRemove>,
    /// The path provides a checkpoint restore (true for a mutating removal).
    pub provides_checkpoint_restore: bool,
    /// The path provides an export-before-remove action.
    pub provides_export_before_remove: bool,
    /// The removal narrows support or portability truth, so export-before-remove is required here.
    pub narrows_support_or_portability: bool,
    /// The path forces a bundle reset to export removal (AC3); always `false`.
    pub forces_reset: bool,
}

/// The resolved bundle removal truth shared across the card, inventory, and restore path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBundleRemoval {
    /// The stable removal identity.
    pub removal_id: String,
    /// The resolved rollback / remove card.
    pub card: M5ResolvedRemovalCard,
    /// The resolved created-versus-adopted asset inventory.
    pub inventory: M5ResolvedAssetInventory,
    /// The resolved restore path.
    pub restore_path: M5ResolvedRestorePath,
    /// Removal is non-destructive of user work (AC1).
    pub non_destructive_of_user_work: bool,
    /// The card states what remains, what is reverted, and what is manual (AC2).
    pub states_remains_reverted_manual: bool,
    /// Export-before-remove and checkpoint restore are available where removal narrows truth (AC3).
    pub export_and_restore_available: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedBundleRemoval {
    /// True when the removal identity is identical across the card, inventory, and restore path.
    pub fn identity_consistent(&self) -> bool {
        self.card.removal_id == self.removal_id
            && self.inventory.removal_id == self.removal_id
            && self.restore_path.removal_id == self.removal_id
    }

    /// True when removal is non-destructive of user work (AC1).
    pub fn non_destructive_of_user_work(&self) -> bool {
        self.non_destructive_of_user_work
    }

    /// True when the card states what remains, what is reverted, and what is manual (AC2).
    pub fn states_remains_reverted_manual(&self) -> bool {
        self.states_remains_reverted_manual
    }

    /// True when export-before-remove and checkpoint restore are available where needed (AC3).
    pub fn export_and_restore_available(&self) -> bool {
        self.export_and_restore_available
    }
}

/// Errors returned by [`resolve_bundle_removal`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BundleRemovalResolutionError {
    /// The removal identity was empty.
    EmptyRemovalId,
    /// The bundle id ref was empty.
    EmptyBundleIdRef,
    /// The bundle name was empty.
    EmptyBundleName,
    /// The compatible Aureline range was empty.
    EmptyCompatibleRange,
    /// The removal carried no assets.
    EmptyAssetInventory,
    /// An asset row was incomplete or inconsistent.
    AssetRowIncomplete,
    /// A user-owned asset was reverted without an explicit selection (AC1).
    UserAssetNotPreserved,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The card read like destructive cleanup of user work (AC1).
    ReadsLikeDestructiveCleanup,
    /// A mutating remove / update offered no one-step checkpoint restore path (AC3).
    MutatingOpWithoutCheckpoint,
    /// Removal narrowed support / portability truth but offered no export-before-remove (AC3).
    ExportBeforeRemoveMissing,
    /// A stale / missing certification was claimed as current instead of narrowing.
    StaleClaimShownAsCurrent,
    /// The card forced a bundle reset to make removal exportable (AC3).
    ForcesResetToExport,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5BundleRemovalResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRemovalId => "empty_removal_id",
            Self::EmptyBundleIdRef => "empty_bundle_id_ref",
            Self::EmptyBundleName => "empty_bundle_name",
            Self::EmptyCompatibleRange => "empty_compatible_range",
            Self::EmptyAssetInventory => "empty_asset_inventory",
            Self::AssetRowIncomplete => "asset_row_incomplete",
            Self::UserAssetNotPreserved => "user_asset_not_preserved",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::ReadsLikeDestructiveCleanup => "reads_like_destructive_cleanup",
            Self::MutatingOpWithoutCheckpoint => "mutating_op_without_checkpoint",
            Self::ExportBeforeRemoveMissing => "export_before_remove_missing",
            Self::StaleClaimShownAsCurrent => "stale_claim_shown_as_current",
            Self::ForcesResetToExport => "forces_reset_to_export",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5BundleRemovalResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "bundle-removal resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BundleRemovalResolutionError {}

/// Resolves one bundle removal context into its shared rollback / remove card, created-versus-
/// adopted asset inventory, and restore path.
///
/// The three surfaces share one removal identity, so a bundle's removal never blurs bundle-owned
/// cleanup with the user-created work that must survive. Every asset is attributed to a created-
/// versus-adopted origin and a safe-to-remove class; a user-owned asset is never reverted unless
/// explicitly selected; the card partitions the inventory into what remains, what is reverted, and
/// what must be handled manually; a mutating remove creates a one-step checkpoint before it
/// commits; an export-before-remove action is available wherever removal narrows support or
/// portability truth; a stale certification never reads as current; and the card never forces a
/// reset to make removal exportable.
pub fn resolve_bundle_removal(
    input: &M5BundleRemovalInput,
) -> Result<M5ResolvedBundleRemoval, M5BundleRemovalResolutionError> {
    if input.removal_id.trim().is_empty() {
        return Err(M5BundleRemovalResolutionError::EmptyRemovalId);
    }
    if input.bundle_id_ref.trim().is_empty() {
        return Err(M5BundleRemovalResolutionError::EmptyBundleIdRef);
    }
    if input.bundle_name.trim().is_empty() {
        return Err(M5BundleRemovalResolutionError::EmptyBundleName);
    }
    if input.compatible_aureline_range.trim().is_empty() {
        return Err(M5BundleRemovalResolutionError::EmptyCompatibleRange);
    }
    if input.assets.is_empty() {
        return Err(M5BundleRemovalResolutionError::EmptyAssetInventory);
    }

    if input_carries_forbidden_material(input) {
        return Err(M5BundleRemovalResolutionError::ForbiddenMaterial);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5BundleRemovalResolutionError::DegradedLabelGeneric);
        }
    }

    // AC1: the card never reads like destructive cleanup of user work.
    if input.reads_like_destructive_cleanup {
        return Err(M5BundleRemovalResolutionError::ReadsLikeDestructiveCleanup);
    }

    // Every asset row is complete, honestly attributed, and preserves user work; a user-owned
    // asset is never reverted without an explicit selection.
    for asset in &input.assets {
        if !asset.preserves_user_asset() {
            return Err(M5BundleRemovalResolutionError::UserAssetNotPreserved);
        }
        if !asset.is_consistent() {
            return Err(M5BundleRemovalResolutionError::AssetRowIncomplete);
        }
    }

    // AC3: a mutating remove / update must create a one-step checkpoint captured before the
    // mutation commits.
    if input.operation.is_mutating() {
        let has_checkpoint = input
            .rollback_checkpoint
            .as_ref()
            .is_some_and(RollbackCheckpoint::supports_one_step_rollback);
        if !has_checkpoint {
            return Err(M5BundleRemovalResolutionError::MutatingOpWithoutCheckpoint);
        }
    }

    // AC3: whenever removal narrows support or portability truth, an export-before-remove action
    // must be available so nothing portable is lost.
    let narrows = input.narrows_support_or_portability();
    let has_export = input
        .export_before_remove
        .as_ref()
        .is_some_and(M5ExportBeforeRemove::is_available);
    if narrows && !has_export {
        return Err(M5BundleRemovalResolutionError::ExportBeforeRemoveMissing);
    }

    // A stale / missing certification narrows the claim rather than being shown as current.
    let freshness_is_stale = matches!(
        input.certification_freshness,
        EvidenceFreshness::Stale | EvidenceFreshness::Missing
    );
    if input.claims_current_despite_stale && freshness_is_stale {
        return Err(M5BundleRemovalResolutionError::StaleClaimShownAsCurrent);
    }

    // AC3: the card never forces a reset to make removal exportable.
    if input.forces_reset_to_export {
        return Err(M5BundleRemovalResolutionError::ForcesResetToExport);
    }

    // Partition the inventory into what is reverted, what remains, and what is manual (AC2).
    let reverted: Vec<String> = input
        .assets
        .iter()
        .filter(|a| a.disposition == M5RemovalDisposition::Reverted)
        .map(|a| a.target_ref.clone())
        .collect();
    let kept_local: Vec<String> = input
        .assets
        .iter()
        .filter(|a| a.disposition == M5RemovalDisposition::KeptLocal)
        .map(|a| a.target_ref.clone())
        .collect();
    let manual_follow_up: Vec<String> = input
        .assets
        .iter()
        .filter(|a| a.disposition == M5RemovalDisposition::ManualFollowUp)
        .map(|a| a.target_ref.clone())
        .collect();

    // Enumerate the origins and safe-to-remove classes present across the inventory, sorted.
    let origins: BTreeSet<M5RemovalAssetOrigin> = input.assets.iter().map(|a| a.origin).collect();
    let origins_present: Vec<M5RemovalAssetOrigin> = origins.into_iter().collect();
    let classes: BTreeSet<M5SafeToRemoveClass> = input
        .assets
        .iter()
        .map(|a| a.safe_to_remove_class)
        .collect();
    let safe_to_remove_classes_present: Vec<M5SafeToRemoveClass> = classes.into_iter().collect();

    let bundle_created_count = input
        .assets
        .iter()
        .filter(|a| !a.origin.is_user_owned())
        .count();
    let user_owned_count = input
        .assets
        .iter()
        .filter(|a| a.origin.is_user_owned())
        .count();

    let provides_checkpoint_restore = input.operation.is_mutating();

    let card = M5ResolvedRemovalCard {
        removal_id: input.removal_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        bundle_name: input.bundle_name.clone(),
        operation: input.operation,
        side_effects: input.side_effects.clone(),
        reverted,
        kept_local,
        manual_follow_up,
        discloses_what_remains: true,
        discloses_what_is_reverted: true,
        discloses_manual_follow_up: true,
        implies_destructive_cleanup: false,
    };

    let inventory = M5ResolvedAssetInventory {
        removal_id: input.removal_id.clone(),
        assets: input.assets.clone(),
        origins_present,
        safe_to_remove_classes_present,
        bundle_created_count,
        user_owned_count,
        distinguishes_created_from_adopted: true,
        preserves_user_assets: true,
        collapses_to_opaque_removal: false,
    };

    let restore_path = M5ResolvedRestorePath {
        removal_id: input.removal_id.clone(),
        rollback_checkpoint: input.rollback_checkpoint.clone(),
        export_before_remove: input.export_before_remove.clone(),
        provides_checkpoint_restore,
        provides_export_before_remove: has_export,
        narrows_support_or_portability: narrows,
        forces_reset: false,
    };

    Ok(M5ResolvedBundleRemoval {
        removal_id: input.removal_id.clone(),
        card,
        inventory,
        restore_path,
        non_destructive_of_user_work: true,
        states_remains_reverted_manual: true,
        export_and_restore_available: true,
        degraded: input.degraded.clone(),
    })
}

/// True when any label, ref, or note on the input carries obviously forbidden material.
fn input_carries_forbidden_material(input: &M5BundleRemovalInput) -> bool {
    let mut values: Vec<&str> = vec![
        input.removal_id.as_str(),
        input.surface_label.as_str(),
        input.bundle_id_ref.as_str(),
        input.bundle_name.as_str(),
        input.compatible_aureline_range.as_str(),
    ];
    for asset in &input.assets {
        values.push(asset.target_ref.as_str());
        values.push(asset.label.as_str());
    }
    if let Some(checkpoint) = &input.rollback_checkpoint {
        values.push(checkpoint.checkpoint_ref.as_str());
    }
    if let Some(export) = &input.export_before_remove {
        values.push(export.export_ref.as_str());
        values.push(export.format_label.as_str());
    }
    values.into_iter().any(value_is_forbidden)
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet reconstructs
/// bundle-removal truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalCase {
    /// The resolver input.
    pub input: M5BundleRemovalInput,
    /// The resolved bundle-removal truth. Must equal `resolve_bundle_removal(&input)`.
    pub resolved: M5ResolvedBundleRemoval,
}

impl M5BundleRemovalCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BundleRemovalInput) -> Self {
        let resolved = resolve_bundle_removal(&input).expect("seed bundle-removal case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_bundle_removal(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one bundle-removal surface family bound to the shared removal
/// contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalSurfaceRow {
    /// The bundle-removal surface family.
    pub surface_family: M5BundleRemovalSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Removal operations this surface can review (must be non-empty).
    pub operations: Vec<BundleReviewOperation>,
    /// Source classes this surface can disclose (must be non-empty).
    pub source_classes: Vec<CertificationTarget>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5BundleTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5BundleRemovalExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5BundleComponentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_removals: Vec<M5BundleRemovalCase>,
    /// Hard invariant: this row never implies destructive cleanup. MUST be `false`.
    pub implies_destructive_cleanup: bool,
    /// Hard invariant: this row never collapses assets to one opaque removal. MUST be `false`.
    pub collapses_to_opaque_removal: bool,
    /// Hard invariant: this row never forces a reset to export removal. MUST be `false`.
    pub forces_reset_to_export: bool,
}

impl M5BundleRemovalSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BundleRemovalExportField> =
            self.export_fields.iter().copied().collect();
        M5BundleRemovalExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.implies_destructive_cleanup
            && !self.collapses_to_opaque_removal
            && !self.forces_reset_to_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalVocabularySet {
    /// Bundle-removal surface-family tokens.
    pub surface_families: Vec<String>,
    /// Asset-origin tokens.
    pub asset_origins: Vec<String>,
    /// Safe-to-remove-class tokens.
    pub safe_to_remove_classes: Vec<String>,
    /// Removal-disposition tokens.
    pub dispositions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Side-effect-class tokens (reused from the bundle-review primitive).
    pub side_effect_classes: Vec<String>,
    /// Review-operation tokens (reused from the review / rollback contract).
    pub review_operations: Vec<String>,
    /// Asset-ownership tokens (reused from the review / rollback contract).
    pub asset_ownerships: Vec<String>,
    /// Component-kind tokens (reused from the bundle-manifest contract).
    pub component_kinds: Vec<String>,
    /// Source-class tokens (reused from the bundle-manifest contract).
    pub source_classes: Vec<String>,
    /// Bundle-class tokens (reused from the entry-governance contract).
    pub bundle_classes: Vec<String>,
    /// Signer / source-trust tokens (reused from the entry-governance contract).
    pub signer_sources: Vec<String>,
    /// Support-class / lifecycle tokens (reused from the bundle-manifest contract).
    pub support_classes: Vec<String>,
    /// Scorecard-class tokens (reused from the scorecard contract).
    pub scorecard_classes: Vec<String>,
    /// Certification-freshness tokens (reused from the scorecard contract).
    pub freshness_states: Vec<String>,
    /// Imported-vs-native confidence tokens (reused from the scorecard contract).
    pub imported_confidences: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5BundleRemovalVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5BundleRemovalSurfaceFamily::ALL,
                M5BundleRemovalSurfaceFamily::as_str,
            ),
            asset_origins: tokens(&M5RemovalAssetOrigin::ALL, M5RemovalAssetOrigin::as_str),
            safe_to_remove_classes: tokens(&M5SafeToRemoveClass::ALL, M5SafeToRemoveClass::as_str),
            dispositions: tokens(&M5RemovalDisposition::ALL, M5RemovalDisposition::as_str),
            export_fields: tokens(
                &M5BundleRemovalExportField::ALL,
                M5BundleRemovalExportField::as_str,
            ),
            side_effect_classes: tokens(
                &M5BundleSideEffectClass::ALL,
                M5BundleSideEffectClass::as_str,
            ),
            review_operations: tokens(&BundleReviewOperation::ALL, BundleReviewOperation::as_str),
            asset_ownerships: tokens(&AssetOwnership::ALL, AssetOwnership::as_str),
            component_kinds: tokens(&BundleComponentKind::ALL, BundleComponentKind::as_str),
            source_classes: tokens(&CertificationTarget::ALL, CertificationTarget::as_str),
            bundle_classes: tokens(&BundleClass::ALL, BundleClass::as_str),
            signer_sources: tokens(&SourceTrust::ALL, SourceTrust::as_str),
            support_classes: tokens(&LifecycleStage::ALL, LifecycleStage::as_str),
            scorecard_classes: tokens(&BundleScorecardClass::ALL, BundleScorecardClass::as_str),
            freshness_states: tokens(&EvidenceFreshness::ALL, EvidenceFreshness::as_str),
            imported_confidences: tokens(
                &ImportedVsNativeConfidence::ALL,
                ImportedVsNativeConfidence::as_str,
            ),
            truth_modes: tokens(&M5BundleTruthMode::ALL, M5BundleTruthMode::as_str),
            downgrade_triggers: tokens(
                &DOWNGRADE_TRIGGER_ALL,
                M5BundleComponentDowngradeTrigger::as_str,
            ),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5BundleComponentDowngradeTrigger; 9] = [
    M5BundleComponentDowngradeTrigger::StaleCertification,
    M5BundleComponentDowngradeTrigger::MirrorStale,
    M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
    M5BundleComponentDowngradeTrigger::UnverifiedSigner,
    M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
    M5BundleComponentDowngradeTrigger::IncompatibleAureline,
    M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
    M5BundleComponentDowngradeTrigger::ImportedNotNative,
    M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalGovernanceReview {
    /// One primitive carries card, inventory, and restore-path truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Removal identity is preserved across the card, inventory, and restore path.
    pub removal_identity_preserved_across_surfaces: bool,
    /// Removal is non-destructive of user-created work and hidden state roots.
    pub removal_non_destructive_of_user_work: bool,
    /// The card states what remains, what is reverted, and what is manual.
    pub states_remains_reverted_manual: bool,
    /// Created-versus-adopted assets stay distinct and attributable.
    pub created_versus_adopted_distinguished: bool,
    /// A mutating remove always creates a one-step checkpoint restore before mutation.
    pub checkpoint_restore_before_mutation: bool,
    /// Export-before-remove is available wherever removal narrows support / portability truth.
    pub export_before_remove_available_when_narrowing: bool,
    /// The support / export packet reconstructs bundle-removal truth.
    pub support_export_reconstructs_removal: bool,
    /// Later M5 rows cannot invent parallel removal / disposition vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalConsumerProjection {
    /// Workspace / bundle / extension / migration / diagnostics / support surfaces all consume the
    /// shared primitive.
    pub removal_surfaces_consume_shared_primitive: bool,
    /// The removal resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The asset inventory reads a single canonical removal source.
    pub inventory_reads_single_source: bool,
    /// Support / export reads a single canonical removal source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the bundle-removal primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRemovalReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting removal audit.
    pub removal_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BundleRollbackRemovePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BundleRollbackRemovePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleRemovalSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleRemovalVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleRemovalGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleRemovalConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleRemovalReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 bundle rollback / remove primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleRollbackRemovePacket {
    /// Record kind; must equal [`M5_BUNDLE_REMOVAL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUNDLE_REMOVAL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleRemovalSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleRemovalVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleRemovalGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleRemovalConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleRemovalReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BundleRollbackRemovePacket {
    /// Builds an M5 bundle rollback / remove primitive packet from stable-lane input.
    pub fn new(input: M5BundleRollbackRemovePacketInput) -> Self {
        Self {
            record_kind: M5_BUNDLE_REMOVAL_RECORD_KIND.to_owned(),
            schema_version: M5_BUNDLE_REMOVAL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 bundle-removal primitive invariants.
    pub fn validate(&self) -> Vec<M5BundleRemovalViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUNDLE_REMOVAL_RECORD_KIND {
            violations.push(M5BundleRemovalViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUNDLE_REMOVAL_SCHEMA_VERSION {
            violations.push(M5BundleRemovalViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BundleRemovalViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 bundle-removal primitive packet serializes"),
        ) {
            violations.push(M5BundleRemovalViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 bundle-removal primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,operations,source_classes,truth_modes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.operations, |v| v.as_str()),
                join_tokens(&row.source_classes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_removals.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Bundle Rollback / Remove Primitive: Rollback / Remove Card, Created-versus-Adopted Asset Inventory, and Restore Path\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Bundle-removal surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5BundleRemovalSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Asset origins: {}\n",
            self.vocabulary_set.asset_origins.join(", ")
        ));
        out.push_str(&format!(
            "- Safe-to-remove classes: {}\n",
            self.vocabulary_set.safe_to_remove_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str("\n## Bundle-removal surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_removals.len()
            ));
            for case in &row.example_removals {
                out.push_str(&format!(
                    "    - `{}` → op `{}`, {} asset(s): {} reverted / {} kept-local / {} manual\n",
                    case.resolved.removal_id,
                    case.resolved.card.operation.as_str(),
                    case.resolved.inventory.assets.len(),
                    case.resolved.card.reverted.len(),
                    case.resolved.card.kept_local.len(),
                    case.resolved.card.manual_follow_up.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 bundle-removal export.
#[derive(Debug)]
pub enum M5BundleRemovalArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BundleRemovalViolation>),
}

impl fmt::Display for M5BundleRemovalArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 bundle-removal primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 bundle-removal primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BundleRemovalArtifactError {}

/// Validation failures emitted by [`M5BundleRollbackRemovePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BundleRemovalViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required bundle-removal surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no removal operations.
    OperationMissing,
    /// A surface row declares no source classes.
    SourceClassMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked removal cases.
    ExampleRemovalsMissing,
    /// A worked removal case does not match a fresh resolve of its input.
    ExampleRemovalDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves removal non-destructive of user work with a preserved asset (AC1).
    NonDestructivenessUnproven,
    /// No worked case proves the remains / reverted / manual partition across the matrix (AC2).
    PartitionSeparationUnproven,
    /// No worked case proves export-before-remove and checkpoint restore across ops (AC3).
    RestoreAvailabilityUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BundleRemovalViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::OperationMissing => "operation_missing",
            Self::SourceClassMissing => "source_class_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleRemovalsMissing => "example_removals_missing",
            Self::ExampleRemovalDrift => "example_removal_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::NonDestructivenessUnproven => "non_destructiveness_unproven",
            Self::PartitionSeparationUnproven => "partition_separation_unproven",
            Self::RestoreAvailabilityUnproven => "restore_availability_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 bundle-removal export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_bundle_rollback_remove_export(
) -> Result<M5BundleRollbackRemovePacket, M5BundleRemovalArtifactError> {
    let packet: M5BundleRollbackRemovePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-rollback-remove-primitive-proof/support_export.json"
    )))
    .map_err(M5BundleRemovalArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BundleRemovalArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUNDLE_REMOVAL_SCHEMA_REF,
        M5_BUNDLE_REMOVAL_DOC_REF,
        M5_BUNDLE_REMOVAL_COMPONENT_MATRIX_REF,
        M5_BUNDLE_REMOVAL_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BundleRemovalViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BundleRemovalViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    let present: BTreeSet<M5BundleRemovalSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5BundleRemovalSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BundleRemovalViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5BundleRemovalViolation::SurfaceRowIncomplete);
        }
        if row.operations.is_empty() {
            violations.push(M5BundleRemovalViolation::OperationMissing);
        }
        if row.source_classes.is_empty() {
            violations.push(M5BundleRemovalViolation::SourceClassMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5BundleRemovalViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BundleRemovalViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BundleRemovalViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BundleRemovalViolation::ConsumerSurfacesMissing);
        }
        if row.example_removals.is_empty() {
            violations.push(M5BundleRemovalViolation::ExampleRemovalsMissing);
        }
        if row
            .example_removals
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BundleRemovalViolation::ExampleRemovalDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5BundleRemovalViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across the
/// matrix: removal is non-destructive of user work and at least one case preserves a user-owned
/// asset alongside bundle-created cleanup (AC1); the card partitions the inventory into remains /
/// reverted / manual with all three lists demonstrated across the matrix (AC2); and export-before-
/// remove and checkpoint restore are available across a read-only preview and a mutating remove
/// (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    let cases: Vec<&M5ResolvedBundleRemoval> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_removals.iter().map(|case| &case.resolved))
        .collect();

    // AC1: every case is non-destructive and never implies destructive cleanup; at least one case
    // separates a bundle-created asset from a preserved user-owned asset.
    let non_destructive_proven = cases.iter().all(|resolved| {
        resolved.non_destructive_of_user_work() && !resolved.card.implies_destructive_cleanup
    }) && cases
        .iter()
        .any(|resolved| resolved.inventory.separates_created_from_user_owned());
    if !non_destructive_proven {
        violations.push(M5BundleRemovalViolation::NonDestructivenessUnproven);
    }

    // AC2: every case states the partition, and the matrix demonstrates a reverted asset, a
    // kept-local asset, and a manual-follow-up asset.
    let has_reverted = cases
        .iter()
        .any(|resolved| !resolved.card.reverted.is_empty());
    let has_kept_local = cases
        .iter()
        .any(|resolved| !resolved.card.kept_local.is_empty());
    let has_manual = cases
        .iter()
        .any(|resolved| !resolved.card.manual_follow_up.is_empty());
    let partition_proven = cases
        .iter()
        .all(|resolved| resolved.states_remains_reverted_manual())
        && has_reverted
        && has_kept_local
        && has_manual;
    if !partition_proven {
        violations.push(M5BundleRemovalViolation::PartitionSeparationUnproven);
    }

    // AC3: every case keeps export-and-restore available and shares one removal identity, never
    // forcing a reset; the matrix spans a read-only preview and a mutating remove, and proves both
    // an export-before-remove action and a checkpoint restore.
    let restore_proven = cases.iter().all(|resolved| {
        resolved.identity_consistent()
            && resolved.export_and_restore_available()
            && !resolved.restore_path.forces_reset
    }) && cases
        .iter()
        .any(|resolved| !resolved.card.operation.is_mutating())
        && cases
            .iter()
            .any(|resolved| resolved.card.operation.is_mutating())
        && cases
            .iter()
            .any(|resolved| resolved.restore_path.provides_export_before_remove)
        && cases
            .iter()
            .any(|resolved| resolved.restore_path.provides_checkpoint_restore);
    if !restore_proven {
        violations.push(M5BundleRemovalViolation::RestoreAvailabilityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.removal_identity_preserved_across_surfaces,
        review.removal_non_destructive_of_user_work,
        review.states_remains_reverted_manual,
        review.created_versus_adopted_distinguished,
        review.checkpoint_restore_before_mutation,
        review.export_before_remove_available_when_narrowing,
        review.support_export_reconstructs_removal,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BundleRemovalViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.removal_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.inventory_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BundleRemovalViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5BundleRollbackRemovePacket,
    violations: &mut Vec<M5BundleRemovalViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.removal_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BundleRemovalViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
