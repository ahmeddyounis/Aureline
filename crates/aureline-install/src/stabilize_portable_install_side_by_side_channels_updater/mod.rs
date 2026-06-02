//! Stable install-profile rows for portable, side-by-side, and managed install lanes.
//!
//! This module materializes one [`StabilizePortableInstallPage`] consumed by
//! About, update center, diagnostics, install-review, and support-export
//! surfaces. Every row carries the full install-profile truth — install mode,
//! channel, updater owner, binary-root class, primary durable-state roots,
//! side-by-side relation, rollback target and artifact-graph scope, and
//! shell-integration ownership — so no surface has to infer topology from
//! launcher behavior or external deployment notes.
//!
//! ## What the page asserts
//!
//! For each install-profile row:
//!
//! - **Install mode and channel** — [`InstallModeClass`] and [`ChannelClass`]
//!   are explicit on every row.
//! - **Updater owner** — the [`UpdaterOwnerClass`] is named, so About and
//!   update surfaces can show whether the user, admin, package manager, or
//!   fleet service owns update decisions.
//! - **Handler ownership** — the row declares who owns file and protocol
//!   handler registration and whether a last-writer-wins scenario is blocked.
//! - **Rollback scope** — [`ArtifactGraphRollbackScope`] names whether binary,
//!   sidecars, symbols, manifests, and update metadata move together.
//! - **Side-by-side isolation** — [`SideBySideIsolationVerdict`] states whether
//!   the channel pair keeps independent durable-state roots or requires an
//!   explicit import-review before any state is shared.
//! - **Portable write guard** — [`PortableWriteGuardClass`] states whether the
//!   running build has been verified not to write undisclosed machine-global
//!   state while presenting itself as portable.
//! - **Durable state roots** — all root refs are present so diagnostics and
//!   support exports never have to infer what state belongs to the install.
//!
//! ## Side-by-side import review
//!
//! Any handoff between Stable, Preview, Beta, portable, or admin-owned installs
//! uses a [`SideBySideImportReviewRow`]. These rows prove that the compare,
//! skip, and checkpoint-backed rollback paths exist before any durable state
//! root is shared or merged.
//!
//! ## Qualification narrowing
//!
//! | Condition | Narrowing |
//! |---|---|
//! | Any row has `stable: false` | `Beta` |
//! | Any portable row permits hidden global writes | `Withdrawn` |
//! | Side-by-side isolation is undisclosed | `Preview` |
//! | Import review missing compare-or-skip | `Beta` |
//! | All conditions met | `Stable` |
//!
//! ## Boundaries
//!
//! This module does not implement an installer, updater, or fleet-control
//! service. All fields are typed tokens, opaque refs, counts, or plain-language
//! labels; no raw host paths, credentials, or secret material cross this
//! boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::profile_cards::InstallSurfaceClass;
use crate::topology::{
    ArchitectureClass, BinaryRootClass, ChannelClass, InstallModeClass, PlatformClass,
    RollbackOwnerClass, SideBySideRelationClass, UpdaterOwnerClass,
};

/// Schema version for stabilize-portable-install records.
pub const STABILIZE_PORTABLE_INSTALL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`StabilizePortableInstallPage`].
pub const STABILIZE_PORTABLE_INSTALL_PAGE_RECORD_KIND: &str =
    "stabilize_portable_install_page_record";

/// Stable record-kind tag for [`StabilizePortableInstallSupportExport`].
pub const STABILIZE_PORTABLE_INSTALL_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stabilize_portable_install_support_export_record";

/// Shared contract ref consumed by every stabilize-portable-install record.
pub const STABILIZE_PORTABLE_INSTALL_SHARED_CONTRACT_REF: &str =
    "install:stabilize_portable_install:v1";

// ── Enumerations ────────────────────────────────────────────────────────────

/// Portable write-guard class for a row claiming portable mode.
///
/// Portable rows must name a guard class; a running build may not present
/// itself as portable and then silently write to machine-global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableWriteGuardClass {
    /// All machine-global writes are suppressed; only colocated-bundle state is written.
    FullySuppressed,
    /// Disclosed global writes exist and the user was shown an explicit opt-in review.
    DisclosedWithOptIn,
    /// Portable mode is not claimed; class is not applicable.
    NotApplicable,
    /// Hidden machine-global writes were detected; the claim is invalid.
    HiddenWritesDetected,
}

/// Side-by-side isolation verdict for a channel pair.
///
/// A verdict of [`Isolated`] means each channel keeps independent durable
/// state roots. A verdict of [`RequiresImportReview`] means shared state is
/// only allowed after a compare-or-skip checkpoint-backed review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideBySideIsolationVerdict {
    /// Channels keep fully independent durable state roots.
    Isolated,
    /// Shared state is only allowed after a compare-or-skip import review.
    RequiresImportReview,
    /// Isolation has not yet been evaluated.
    Undisclosed,
    /// Side-by-side is not applicable for this row.
    NotApplicable,
}

/// Artifact-graph rollback scope for an install row.
///
/// Describes whether rollback moves binary, sidecars, symbols, manifests,
/// and update metadata together as a unit or only reverts the primary
/// executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactGraphRollbackScope {
    /// Binary, sidecars, symbols, manifests, and update metadata all revert together.
    FullArtifactGraph,
    /// Only the primary binary reverts; sidecars and metadata are unchanged.
    BinaryOnlyPartialGraph,
    /// Rollback is owned by the external package manager; scope is not controlled by the product.
    PackageManagerOwned,
    /// Rollback is owned by the managed-fleet rollout service.
    ManagedFleetOwned,
    /// Rollback scope is not disclosed.
    Undisclosed,
    /// Rollback is unsupported for this install.
    Unsupported,
}

/// Handler registration class for an install row.
///
/// Captures whether the row registers file or protocol handlers and who
/// controls the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandlerRegistrationClass {
    /// User or admin selects the default handler; last-writer-wins is blocked.
    UserOrAdminSelectableNeverLastWriterWins,
    /// Administrator policy controls the default handler.
    AdminOnly,
    /// Portable install does not register handlers.
    PortableNoRegistration,
    /// Install does not register handlers.
    NotRegistered,
}

/// Import review class for a side-by-side import-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReviewClass {
    /// Compare-before-apply review with checkpoint-backed rollback.
    CompareOrSkipWithCheckpoint,
    /// Skip preserves source and writes no target state.
    SkipPreservingSource,
    /// Review is blocked until collision is resolved.
    BlockedPendingCollisionResolution,
    /// Import is not applicable.
    NotApplicable,
}

/// Narrow reason token for a defect or row qualification narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeNarrowReasonToken {
    /// No narrowing applied.
    NotNarrowed,
    /// Portable row has hidden machine-global writes.
    PortableHiddenWritesDetected,
    /// Side-by-side isolation verdict is undisclosed.
    SideBySideIsolationUndisclosed,
    /// Side-by-side import review is missing compare-or-skip semantics.
    ImportReviewMissingCompareOrSkip,
    /// Handler ownership is not named on the row.
    HandlerOwnershipNotNamed,
    /// Rollback scope is not disclosed.
    RollbackScopeUndisclosed,
    /// Updater owner is not named on the row.
    UpdaterOwnerNotNamed,
    /// Durable state roots are not declared.
    DurableStateRootsNotDeclared,
    /// Row display label is empty.
    DisplayLabelEmpty,
    /// Row profile-card ref is empty.
    ProfileCardRefEmpty,
    /// No install-profile rows are present in the page.
    NoInstallProfileRows,
    /// Install-profile identity mismatch between row and referenced card.
    ProfileIdentityMismatch,
}

/// Qualification token for a row or page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeQualificationToken {
    /// All conditions met; the row or page is stable.
    Stable,
    /// Non-critical conditions unmet; narrowed to beta.
    Beta,
    /// Structural coverage gap; narrowed to preview.
    Preview,
    /// Critical invariant violated; the page is withdrawn.
    Withdrawn,
}

// ── Row-level structures ─────────────────────────────────────────────────────

/// Handler ownership summary for an install-profile stable row.
///
/// Carries file-association and protocol-handler ownership truth so About,
/// update, and diagnostics surfaces can show who owns the default-open behavior
/// without reading OS registration tables.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlerOwnershipSummary {
    /// File-association registration class.
    pub file_association_class: HandlerRegistrationClass,
    /// Protocol-handler registration class.
    pub protocol_handler_class: HandlerRegistrationClass,
    /// Channel that currently owns the selected default handler, if applicable.
    pub owning_channel_class: Option<ChannelClass>,
    /// True when a last-writer-wins takeover of handler registration is blocked.
    pub last_writer_wins_blocked: bool,
    /// Human-readable collision disclosure.
    pub collision_disclosure: String,
}

/// Shell-integration ownership for a portable install row.
///
/// Portable rows must declare what shell integration is absent; the product
/// must not register shell hooks that imply machine-global state while
/// presenting itself as portable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableShellIntegrationOwnership {
    /// True when shell hook integration is suppressed.
    pub shell_hooks_suppressed: bool,
    /// True when PATH or environment mutation is suppressed.
    pub path_mutation_suppressed: bool,
    /// True when credential-store access is suppressed.
    pub credential_store_suppressed: bool,
    /// True when service registration is suppressed.
    pub service_registration_suppressed: bool,
    /// Human-readable disclosure of which integrations are absent.
    pub absent_integrations_disclosure: String,
}

/// A single install-profile stable row.
///
/// The canonical install-profile object consumed by About, update center,
/// diagnostics, install-review, and support-export surfaces. Each row carries
/// install mode, updater owner, handler ownership, rollback scope, isolation
/// verdict, portable write guard, and durable state-root refs so no surface
/// has to infer topology from external notes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileStableRow {
    /// Stable row id.
    pub row_id: String,
    /// Ref to the profile-card record this row extends.
    pub profile_card_ref: String,
    /// Human-readable display label.
    pub display_label: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Architecture class.
    pub architecture_class: ArchitectureClass,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Opaque binary root ref.
    pub binary_root_ref: String,
    /// Actor class that owns updates for this row.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Handler ownership truth.
    pub handler_ownership: HandlerOwnershipSummary,
    /// Actor class that owns rollback decisions.
    pub rollback_owner_class: RollbackOwnerClass,
    /// Rollback artifact-graph scope.
    pub rollback_scope: ArtifactGraphRollbackScope,
    /// Rollback target ref.
    pub rollback_target_ref: Option<String>,
    /// Side-by-side relation class claimed by this row.
    pub side_by_side_relation_class: SideBySideRelationClass,
    /// Side-by-side isolation verdict for this row's channel.
    pub isolation_verdict: SideBySideIsolationVerdict,
    /// Portable write guard class.
    ///
    /// Must be [`PortableWriteGuardClass::FullySuppressed`] or
    /// [`PortableWriteGuardClass::DisclosedWithOptIn`] for any row claiming
    /// portable mode.
    pub portable_write_guard: PortableWriteGuardClass,
    /// Shell-integration ownership for portable rows.
    pub portable_shell_integration: Option<PortableShellIntegrationOwnership>,
    /// Durable state-root refs carried by this row.
    pub durable_state_root_refs: Vec<String>,
    /// Surfaces where this row is exposed.
    pub exposed_in_surfaces: Vec<InstallSurfaceClass>,
    /// True when all stability conditions are met.
    pub stable: bool,
    /// Qualification token.
    pub qualification_token: StabilizeQualificationToken,
    /// Narrow reason when the row is not `Stable`.
    pub narrow_reason_token: StabilizeNarrowReasonToken,
    /// Export-safe plain-language summary.
    pub plain_language_summary: String,
}

impl InstallProfileStableRow {
    /// Returns true when this row describes a portable install.
    pub fn is_portable(&self) -> bool {
        matches!(self.install_mode_class, InstallModeClass::Portable)
    }

    /// Returns true when the row's portable write guard is valid.
    ///
    /// A portable row is only valid when the guard is either
    /// [`PortableWriteGuardClass::FullySuppressed`] or
    /// [`PortableWriteGuardClass::DisclosedWithOptIn`].
    pub fn portable_write_guard_valid(&self) -> bool {
        if !self.is_portable() {
            return true;
        }
        matches!(
            self.portable_write_guard,
            PortableWriteGuardClass::FullySuppressed | PortableWriteGuardClass::DisclosedWithOptIn
        )
    }
}

/// A side-by-side import-review row.
///
/// Proves that the compare-or-skip and checkpoint-backed rollback paths exist
/// before any durable state root can be shared between two channels or install
/// modes. No channel may silently migrate durable-state roots, collapse
/// namespaces, or inherit file-association ownership without passing this
/// review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideBySideImportReviewRow {
    /// Stable review row id.
    pub review_id: String,
    /// Source install-profile row id.
    pub source_row_id: String,
    /// Target install-profile row id.
    pub target_row_id: String,
    /// Source channel class.
    pub source_channel_class: ChannelClass,
    /// Target channel class.
    pub target_channel_class: ChannelClass,
    /// Import review class.
    pub review_class: ImportReviewClass,
    /// True when compare-before-apply is available.
    pub can_compare_before_apply: bool,
    /// Checkpoint ref created before apply, when required.
    pub checkpoint_ref: Option<String>,
    /// True when a checkpoint is created before any apply.
    pub checkpoint_created_before_apply: bool,
    /// True when skip preserves source state with no target write.
    pub skip_preserves_source: bool,
    /// Domains included in the comparison scope.
    pub comparison_scope_domains: Vec<String>,
    /// Collision disclosures for this import.
    pub collision_disclosures: Vec<String>,
    /// Human-readable import review summary.
    pub review_summary: String,
}

impl SideBySideImportReviewRow {
    /// Returns true when the review satisfies the compare-or-skip contract.
    pub fn compare_or_skip_satisfied(&self) -> bool {
        matches!(
            self.review_class,
            ImportReviewClass::CompareOrSkipWithCheckpoint
                | ImportReviewClass::SkipPreservingSource
                | ImportReviewClass::NotApplicable
        ) && (self.can_compare_before_apply || self.skip_preserves_source)
    }
}

/// A fleet rollout diagnostics row for managed or air-gapped install lanes.
///
/// Carries install-profile identity and channel separation proof so that
/// opening logs, exporting diagnostics, or reverting a managed rollout
/// cannot collapse channel or install-profile namespaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetRolloutInstallDiagnosticsRow {
    /// Stable row id.
    pub row_id: String,
    /// Install-profile row id this row extends.
    pub install_profile_row_id: String,
    /// Platform token.
    pub platform_token: String,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Rollback scope for this fleet lane.
    pub rollback_scope: ArtifactGraphRollbackScope,
    /// True when install-profile identity is preserved through log or diagnostic export.
    pub identity_preserved_in_export: bool,
    /// True when channel separation is maintained through rollout revert.
    pub channel_separation_maintained_on_revert: bool,
    /// Human-readable fleet diagnostics summary.
    pub fleet_diagnostics_summary: String,
}

// ── Summary and defect structures ───────────────────────────────────────────

/// Aggregate summary for a [`StabilizePortableInstallPage`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallSummary {
    /// Total number of install-profile rows.
    pub install_profile_row_count: u32,
    /// Number of rows qualified as `Stable`.
    pub stable_row_count: u32,
    /// Number of rows qualified as `Beta`.
    pub beta_row_count: u32,
    /// Number of rows qualified as `Preview`.
    pub preview_row_count: u32,
    /// Number of rows qualified as `Withdrawn`.
    pub withdrawn_row_count: u32,
    /// Install modes covered.
    pub install_modes_covered: Vec<InstallModeClass>,
    /// Channels covered.
    pub channels_covered: Vec<ChannelClass>,
    /// Number of portable rows with a valid write guard.
    pub portable_write_guard_valid_count: u32,
    /// Number of side-by-side import reviews with compare-or-skip satisfied.
    pub import_review_compare_or_skip_count: u32,
    /// Overall page qualification token.
    pub overall_qualification_token: StabilizeQualificationToken,
}

/// One defect emitted by the stabilize-portable-install auditor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallDefect {
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for the defect.
    pub narrow_reason: StabilizeNarrowReasonToken,
    /// Narrow reason token (kept for schema symmetry).
    pub narrow_reason_token: StabilizeNarrowReasonToken,
    /// Row or packet ref that produced the defect.
    pub source: String,
    /// Export-safe note explaining the defect.
    pub note: String,
}

// ── Page record ──────────────────────────────────────────────────────────────

/// Stable install-profile page for portable, side-by-side, and managed lanes.
///
/// The single inspectable record consumed by About, update center, diagnostics,
/// install-review, and support-export surfaces. Dashboards, help surfaces, and
/// release-evidence reviewers should ingest this page rather than maintaining
/// parallel prose about install topology, channel separation, handler
/// ownership, or rollback scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallPage {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// ISO 8601 generation timestamp.
    pub generated_at: String,
    /// Install-profile stable rows.
    pub install_profile_rows: Vec<InstallProfileStableRow>,
    /// Side-by-side import-review rows.
    pub import_review_rows: Vec<SideBySideImportReviewRow>,
    /// Fleet rollout diagnostics rows.
    pub fleet_rollout_diagnostics_rows: Vec<FleetRolloutInstallDiagnosticsRow>,
    /// Defects found by the auditor.
    pub defects: Vec<StabilizePortableInstallDefect>,
    /// Aggregate summary.
    pub summary: StabilizePortableInstallSummary,
}

impl StabilizePortableInstallPage {
    /// Audits the page and returns all defects.
    pub fn audit(&self) -> Vec<StabilizePortableInstallDefect> {
        audit_stabilize_portable_install_page(self)
    }

    /// Validates the page and returns a structured report.
    pub fn validate(&self) -> StabilizePortableInstallValidationReport {
        validate_stabilize_portable_install_page(self)
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> StabilizePortableInstallSupportExport {
        let narrow_reasons: BTreeSet<StabilizeNarrowReasonToken> =
            self.defects.iter().map(|d| d.narrow_reason_token).collect();
        let mut defect_counts: BTreeMap<String, u32> = BTreeMap::new();
        for defect in &self.defects {
            let key = format!("{:?}", defect.narrow_reason_token)
                .to_lowercase()
                .replace(' ', "_");
            *defect_counts.entry(key).or_insert(0) += 1;
        }
        StabilizePortableInstallSupportExport {
            record_kind: STABILIZE_PORTABLE_INSTALL_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: STABILIZE_PORTABLE_INSTALL_SCHEMA_VERSION,
            shared_contract_ref: STABILIZE_PORTABLE_INSTALL_SHARED_CONTRACT_REF.to_string(),
            export_id: format!("stabilize-portable-install:support-export:{}", self.page_id),
            generated_at: self.generated_at.clone(),
            page: self.clone(),
            narrow_reasons_present: narrow_reasons.into_iter().collect(),
            defect_counts_by_narrow_reason: defect_counts,
            raw_private_material_excluded: true,
        }
    }

    /// Returns the overall qualification token derived from the summary.
    pub fn overall_qualification(&self) -> StabilizeQualificationToken {
        self.summary.overall_qualification_token
    }
}

/// Metadata-safe support-export for a stabilize-portable-install page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// ISO 8601 generation timestamp.
    pub generated_at: String,
    /// Source page.
    pub page: StabilizePortableInstallPage,
    /// Narrow reasons present across all defects.
    pub narrow_reasons_present: Vec<StabilizeNarrowReasonToken>,
    /// Defect counts keyed by narrow-reason token string.
    pub defect_counts_by_narrow_reason: BTreeMap<String, u32>,
    /// Always `true`; no raw private material crosses this boundary.
    pub raw_private_material_excluded: bool,
}

// ── Validation coverage, findings, report ───────────────────────────────────

/// Validation coverage collected from a stabilize-portable-install page.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallCoverage {
    /// Install modes covered by rows.
    pub install_modes: BTreeSet<InstallModeClass>,
    /// Channels covered by rows.
    pub channels: BTreeSet<ChannelClass>,
    /// Updater owner classes present.
    pub updater_owner_classes: BTreeSet<UpdaterOwnerClass>,
    /// Binary root classes present.
    pub binary_root_classes: BTreeSet<BinaryRootClass>,
    /// Rollback scope classes present.
    pub rollback_scope_classes: BTreeSet<ArtifactGraphRollbackScope>,
    /// Portable write guard classes present.
    pub portable_write_guard_classes: BTreeSet<PortableWriteGuardClass>,
}

/// One validation finding from [`validate_stabilize_portable_install_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable finding message.
    pub message: String,
    /// Row or packet ref that caused the finding.
    pub ref_id: String,
}

/// Validation report for a stabilize-portable-install page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizePortableInstallValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Coverage collected during validation.
    pub coverage: StabilizePortableInstallCoverage,
    /// Validation findings.
    pub findings: Vec<StabilizePortableInstallValidationFinding>,
}

// ── Audit function ───────────────────────────────────────────────────────────

/// Audits a [`StabilizePortableInstallPage`] and returns all defects.
///
/// Returns an empty `Vec` when the page is fully `Stable`. Each defect carries
/// a closed `narrow_reason_token` and an export-safe `note`.
///
/// One condition forces `Withdrawn` immediately and skips all remaining
/// checks: any portable row where `portable_write_guard` is
/// [`PortableWriteGuardClass::HiddenWritesDetected`].
pub fn audit_stabilize_portable_install_page(
    page: &StabilizePortableInstallPage,
) -> Vec<StabilizePortableInstallDefect> {
    let mut defects = Vec::new();
    let mut defect_idx = 0u32;

    let mut next_id = || {
        defect_idx += 1;
        format!("stabilize-portable-defect:{:04}", defect_idx)
    };

    if page.install_profile_rows.is_empty() {
        defects.push(StabilizePortableInstallDefect {
            defect_id: next_id(),
            narrow_reason: StabilizeNarrowReasonToken::NoInstallProfileRows,
            narrow_reason_token: StabilizeNarrowReasonToken::NoInstallProfileRows,
            source: page.page_id.clone(),
            note: "no install-profile rows are present; page is narrowed to preview".into(),
        });
        return defects;
    }

    for row in &page.install_profile_rows {
        // Critical: hidden writes on a portable row force Withdrawn.
        if row.is_portable()
            && row.portable_write_guard == PortableWriteGuardClass::HiddenWritesDetected
        {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::PortableHiddenWritesDetected,
                narrow_reason_token: StabilizeNarrowReasonToken::PortableHiddenWritesDetected,
                source: row.row_id.clone(),
                note: format!(
                    "portable row '{}' has hidden machine-global writes; \
                     a build may not present itself as portable and write undisclosed global state",
                    row.row_id
                ),
            });
            // Withdrawal — skip remaining checks.
            return defects;
        }

        if row.display_label.trim().is_empty() {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::DisplayLabelEmpty,
                narrow_reason_token: StabilizeNarrowReasonToken::DisplayLabelEmpty,
                source: row.row_id.clone(),
                note: format!(
                    "install-profile row '{}' has an empty display label; \
                     every row must carry a human-readable label for About and diagnostics surfaces",
                    row.row_id
                ),
            });
        }

        if row.profile_card_ref.trim().is_empty() {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::ProfileCardRefEmpty,
                narrow_reason_token: StabilizeNarrowReasonToken::ProfileCardRefEmpty,
                source: row.row_id.clone(),
                note: format!(
                    "install-profile row '{}' has no profile_card_ref; \
                     every stable row must reference the profile-card record it extends",
                    row.row_id
                ),
            });
        }

        if row.durable_state_root_refs.is_empty() {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::DurableStateRootsNotDeclared,
                narrow_reason_token: StabilizeNarrowReasonToken::DurableStateRootsNotDeclared,
                source: row.row_id.clone(),
                note: format!(
                    "install-profile row '{}' declares no durable state-root refs; \
                     every row must disclose its durable state roots",
                    row.row_id
                ),
            });
        }

        if row.isolation_verdict == SideBySideIsolationVerdict::Undisclosed {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::SideBySideIsolationUndisclosed,
                narrow_reason_token: StabilizeNarrowReasonToken::SideBySideIsolationUndisclosed,
                source: row.row_id.clone(),
                note: format!(
                    "install-profile row '{}' has an undisclosed side-by-side isolation verdict; \
                     channels must declare whether their state roots are isolated or require import review",
                    row.row_id
                ),
            });
        }

        if row.rollback_scope == ArtifactGraphRollbackScope::Undisclosed {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::RollbackScopeUndisclosed,
                narrow_reason_token: StabilizeNarrowReasonToken::RollbackScopeUndisclosed,
                source: row.row_id.clone(),
                note: format!(
                    "install-profile row '{}' has an undisclosed rollback scope; \
                     the blast radius of rollback must be named so recovery copy is accurate",
                    row.row_id
                ),
            });
        }

        if row.handler_ownership.collision_disclosure.trim().is_empty() {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::HandlerOwnershipNotNamed,
                narrow_reason_token: StabilizeNarrowReasonToken::HandlerOwnershipNotNamed,
                source: row.row_id.clone(),
                note: format!(
                    "install-profile row '{}' has no handler-ownership collision disclosure; \
                     file-association and protocol-handler ownership must be explicitly named",
                    row.row_id
                ),
            });
        }
    }

    for review in &page.import_review_rows {
        if !review.compare_or_skip_satisfied() {
            defects.push(StabilizePortableInstallDefect {
                defect_id: next_id(),
                narrow_reason: StabilizeNarrowReasonToken::ImportReviewMissingCompareOrSkip,
                narrow_reason_token: StabilizeNarrowReasonToken::ImportReviewMissingCompareOrSkip,
                source: review.review_id.clone(),
                note: format!(
                    "side-by-side import review '{}' does not satisfy compare-or-skip; \
                     every channel handoff must provide compare-before-apply or skip-preserving-source \
                     before any durable state root can be shared",
                    review.review_id
                ),
            });
        }
    }

    defects
}

// ── Validation function ──────────────────────────────────────────────────────

/// Validates a [`StabilizePortableInstallPage`] and returns a structured report.
pub fn validate_stabilize_portable_install_page(
    page: &StabilizePortableInstallPage,
) -> StabilizePortableInstallValidationReport {
    let mut findings = Vec::new();
    let mut coverage = StabilizePortableInstallCoverage::default();

    let push = |findings: &mut Vec<StabilizePortableInstallValidationFinding>,
                check_id: &str,
                message: String,
                ref_id: String| {
        findings.push(StabilizePortableInstallValidationFinding {
            check_id: check_id.to_string(),
            message,
            ref_id,
        });
    };

    if page.record_kind != STABILIZE_PORTABLE_INSTALL_PAGE_RECORD_KIND {
        push(
            &mut findings,
            "stabilize_portable.page.record_kind",
            "page record_kind is not stabilize_portable_install_page_record".into(),
            page.page_id.clone(),
        );
    }
    if page.schema_version != STABILIZE_PORTABLE_INSTALL_SCHEMA_VERSION {
        push(
            &mut findings,
            "stabilize_portable.page.schema_version",
            "page schema_version is unsupported".into(),
            page.page_id.clone(),
        );
    }
    if page.shared_contract_ref != STABILIZE_PORTABLE_INSTALL_SHARED_CONTRACT_REF {
        push(
            &mut findings,
            "stabilize_portable.page.shared_contract_ref",
            "page shared_contract_ref does not match expected value".into(),
            page.page_id.clone(),
        );
    }
    if page.page_id.trim().is_empty() {
        push(
            &mut findings,
            "stabilize_portable.page.page_id_empty",
            "page_id must be non-empty".into(),
            page.page_id.clone(),
        );
    }

    for row in &page.install_profile_rows {
        coverage.install_modes.insert(row.install_mode_class);
        coverage.channels.insert(row.channel_class);
        coverage
            .updater_owner_classes
            .insert(row.updater_owner_class);
        coverage.binary_root_classes.insert(row.binary_root_class);
        coverage.rollback_scope_classes.insert(row.rollback_scope);
        coverage
            .portable_write_guard_classes
            .insert(row.portable_write_guard);

        if row.row_id.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.row.row_id_empty",
                "install-profile row has empty row_id".into(),
                row.profile_card_ref.clone(),
            );
        }
        if row.display_label.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.row.display_label_empty",
                "install-profile row has empty display_label".into(),
                row.row_id.clone(),
            );
        }
        if row.profile_card_ref.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.row.profile_card_ref_empty",
                "install-profile row has empty profile_card_ref".into(),
                row.row_id.clone(),
            );
        }
        if row.binary_root_ref.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.row.binary_root_ref_empty",
                "install-profile row has empty binary_root_ref".into(),
                row.row_id.clone(),
            );
        }
        if row.durable_state_root_refs.is_empty() {
            push(
                &mut findings,
                "stabilize_portable.row.state_roots_missing",
                "install-profile row declares no durable state-root refs".into(),
                row.row_id.clone(),
            );
        }
        if row.isolation_verdict == SideBySideIsolationVerdict::Undisclosed {
            push(
                &mut findings,
                "stabilize_portable.row.isolation_undisclosed",
                "install-profile row has undisclosed side-by-side isolation verdict".into(),
                row.row_id.clone(),
            );
        }
        if row.rollback_scope == ArtifactGraphRollbackScope::Undisclosed {
            push(
                &mut findings,
                "stabilize_portable.row.rollback_scope_undisclosed",
                "install-profile row has undisclosed rollback scope".into(),
                row.row_id.clone(),
            );
        }
        if row.is_portable() && !row.portable_write_guard_valid() {
            push(
                &mut findings,
                "stabilize_portable.row.portable_write_guard_invalid",
                "portable row has invalid portable_write_guard".into(),
                row.row_id.clone(),
            );
        }
        if row.plain_language_summary.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.row.summary_empty",
                "install-profile row has empty plain_language_summary".into(),
                row.row_id.clone(),
            );
        }
    }

    for review in &page.import_review_rows {
        if review.review_id.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.review.review_id_empty",
                "import-review row has empty review_id".into(),
                review.source_row_id.clone(),
            );
        }
        if !review.compare_or_skip_satisfied() {
            push(
                &mut findings,
                "stabilize_portable.review.compare_or_skip_not_satisfied",
                "import-review row does not satisfy compare-or-skip contract".into(),
                review.review_id.clone(),
            );
        }
        if review.review_summary.trim().is_empty() {
            push(
                &mut findings,
                "stabilize_portable.review.summary_empty",
                "import-review row has empty review_summary".into(),
                review.review_id.clone(),
            );
        }
    }

    StabilizePortableInstallValidationReport {
        passed: findings.is_empty(),
        coverage,
        findings,
    }
}

// ── Seeded instance ──────────────────────────────────────────────────────────

/// Returns a fully populated, valid [`StabilizePortableInstallPage`] seeded
/// from the stable deployment matrix.
///
/// This is the canonical seeded record for the lane. About, update, diagnostics,
/// install-review, and support-export surfaces should ingest it rather than
/// maintaining parallel prose about install topology or channel separation.
pub fn seeded_stabilize_portable_install_page() -> StabilizePortableInstallPage {
    let row_user_stable = InstallProfileStableRow {
        row_id: "stabilize-portable.row.windows.per_user.stable".into(),
        profile_card_ref: "card.windows.x86_64.per_user_installed.stable".into(),
        display_label: "Windows Stable per-user".into(),
        platform_class: PlatformClass::Windows,
        architecture_class: ArchitectureClass::X86_64,
        install_mode_class: InstallModeClass::PerUserInstalled,
        channel_class: ChannelClass::Stable,
        binary_root_class: BinaryRootClass::PerUserProfileProgramArea,
        binary_root_ref: "binary_root:windows:user:stable".into(),
        updater_owner_class: UpdaterOwnerClass::User,
        handler_ownership: HandlerOwnershipSummary {
            file_association_class:
                HandlerRegistrationClass::UserOrAdminSelectableNeverLastWriterWins,
            protocol_handler_class:
                HandlerRegistrationClass::UserOrAdminSelectableNeverLastWriterWins,
            owning_channel_class: Some(ChannelClass::Stable),
            last_writer_wins_blocked: true,
            collision_disclosure: "User selects default handler; last-writer-wins registration \
                is blocked; stable and preview use per-channel scheme suffixes."
                .into(),
        },
        rollback_owner_class: RollbackOwnerClass::User,
        rollback_scope: ArtifactGraphRollbackScope::FullArtifactGraph,
        rollback_target_ref: Some(
            "artifacts/release/m3/update_rollback/rollback_plan.json\
             #release_candidate:aureline.2_0_4_stable"
                .into(),
        ),
        side_by_side_relation_class: SideBySideRelationClass::StableAndPreview,
        isolation_verdict: SideBySideIsolationVerdict::Isolated,
        portable_write_guard: PortableWriteGuardClass::NotApplicable,
        portable_shell_integration: None,
        durable_state_root_refs: vec![
            "state.per_user_configuration_root.stable".into(),
            "state.per_user_recovery_root.stable".into(),
            "state.per_user_derived_cache_root.stable".into(),
        ],
        exposed_in_surfaces: vec![
            InstallSurfaceClass::About,
            InstallSurfaceClass::UpdateCenter,
            InstallSurfaceClass::DiagnosticsCenter,
            InstallSurfaceClass::SupportBundle,
        ],
        stable: true,
        qualification_token: StabilizeQualificationToken::Stable,
        narrow_reason_token: StabilizeNarrowReasonToken::NotNarrowed,
        plain_language_summary: "Windows Stable per-user: user-owned updates, full artifact-graph \
            rollback, state roots isolated from Preview, handler registration is \
            user-selectable with last-writer-wins blocked."
            .into(),
    };

    let row_user_preview = InstallProfileStableRow {
        row_id: "stabilize-portable.row.windows.per_user.preview".into(),
        profile_card_ref: "card.windows.x86_64.per_user_installed.preview".into(),
        display_label: "Windows Preview per-user".into(),
        platform_class: PlatformClass::Windows,
        architecture_class: ArchitectureClass::X86_64,
        install_mode_class: InstallModeClass::SideBySidePreview,
        channel_class: ChannelClass::Preview,
        binary_root_class: BinaryRootClass::PerUserProfileProgramArea,
        binary_root_ref: "binary_root:windows:user:preview".into(),
        updater_owner_class: UpdaterOwnerClass::User,
        handler_ownership: HandlerOwnershipSummary {
            file_association_class:
                HandlerRegistrationClass::UserOrAdminSelectableNeverLastWriterWins,
            protocol_handler_class:
                HandlerRegistrationClass::UserOrAdminSelectableNeverLastWriterWins,
            owning_channel_class: None,
            last_writer_wins_blocked: true,
            collision_disclosure: "Preview uses a channel-suffixed scheme; user selects \
                which channel handles the default scheme; last-writer-wins is blocked."
                .into(),
        },
        rollback_owner_class: RollbackOwnerClass::User,
        rollback_scope: ArtifactGraphRollbackScope::FullArtifactGraph,
        rollback_target_ref: Some(
            "artifacts/release/m3/update_rollback/rollback_plan.json\
             #release_candidate:aureline.2_0_4_preview"
                .into(),
        ),
        side_by_side_relation_class: SideBySideRelationClass::StableAndPreview,
        isolation_verdict: SideBySideIsolationVerdict::Isolated,
        portable_write_guard: PortableWriteGuardClass::NotApplicable,
        portable_shell_integration: None,
        durable_state_root_refs: vec![
            "state.per_user_configuration_root.preview".into(),
            "state.per_user_recovery_root.preview".into(),
            "state.per_user_derived_cache_root.preview".into(),
        ],
        exposed_in_surfaces: vec![
            InstallSurfaceClass::About,
            InstallSurfaceClass::UpdateCenter,
            InstallSurfaceClass::DiagnosticsCenter,
            InstallSurfaceClass::ImportSheet,
            InstallSurfaceClass::SupportBundle,
        ],
        stable: true,
        qualification_token: StabilizeQualificationToken::Stable,
        narrow_reason_token: StabilizeNarrowReasonToken::NotNarrowed,
        plain_language_summary: "Windows Preview per-user: user-owned updates, independent \
            state roots, full artifact-graph rollback, side-by-side with Stable. \
            Handler import review required before any state-root sharing."
            .into(),
    };

    let row_portable_stable = InstallProfileStableRow {
        row_id: "stabilize-portable.row.windows.portable.stable".into(),
        profile_card_ref: "card.windows.x86_64.portable.stable".into(),
        display_label: "Windows Portable Stable".into(),
        platform_class: PlatformClass::Windows,
        architecture_class: ArchitectureClass::X86_64,
        install_mode_class: InstallModeClass::Portable,
        channel_class: ChannelClass::PortableStable,
        binary_root_class: BinaryRootClass::PortableDirectory,
        binary_root_ref: "binary_root:windows:portable:stable".into(),
        updater_owner_class: UpdaterOwnerClass::User,
        handler_ownership: HandlerOwnershipSummary {
            file_association_class: HandlerRegistrationClass::PortableNoRegistration,
            protocol_handler_class: HandlerRegistrationClass::PortableNoRegistration,
            owning_channel_class: None,
            last_writer_wins_blocked: true,
            collision_disclosure: "Portable install does not register file associations or \
                protocol handlers; no handler collision is possible from this install."
                .into(),
        },
        rollback_owner_class: RollbackOwnerClass::User,
        rollback_scope: ArtifactGraphRollbackScope::FullArtifactGraph,
        rollback_target_ref: None,
        side_by_side_relation_class: SideBySideRelationClass::InstalledAndPortable,
        isolation_verdict: SideBySideIsolationVerdict::Isolated,
        portable_write_guard: PortableWriteGuardClass::FullySuppressed,
        portable_shell_integration: Some(PortableShellIntegrationOwnership {
            shell_hooks_suppressed: true,
            path_mutation_suppressed: true,
            credential_store_suppressed: true,
            service_registration_suppressed: true,
            absent_integrations_disclosure: "Shell hooks, PATH mutation, credential-store \
                access, and service registration are all suppressed; state is colocated \
                with the portable bundle directory."
                .into(),
        }),
        durable_state_root_refs: vec!["state.portable_colocated_root.stable".into()],
        exposed_in_surfaces: vec![
            InstallSurfaceClass::About,
            InstallSurfaceClass::DiagnosticsCenter,
            InstallSurfaceClass::SupportBundle,
        ],
        stable: true,
        qualification_token: StabilizeQualificationToken::Stable,
        narrow_reason_token: StabilizeNarrowReasonToken::NotNarrowed,
        plain_language_summary: "Windows Portable Stable: no machine-global writes, no handler \
            registration, state colocated with bundle; shell hooks, PATH mutation, and \
            credential-store access are all suppressed."
            .into(),
    };

    let row_managed_stable = InstallProfileStableRow {
        row_id: "stabilize-portable.row.windows.managed.stable".into(),
        profile_card_ref: "card.windows.x86_64.per_machine_managed.stable".into(),
        display_label: "Windows Managed per-machine Stable".into(),
        platform_class: PlatformClass::Windows,
        architecture_class: ArchitectureClass::X86_64,
        install_mode_class: InstallModeClass::ManagedDeployed,
        channel_class: ChannelClass::Stable,
        binary_root_class: BinaryRootClass::PerMachineProgramArea,
        binary_root_ref: "binary_root:windows:machine:managed:stable".into(),
        updater_owner_class: UpdaterOwnerClass::ManagedFleet,
        handler_ownership: HandlerOwnershipSummary {
            file_association_class: HandlerRegistrationClass::AdminOnly,
            protocol_handler_class: HandlerRegistrationClass::AdminOnly,
            owning_channel_class: Some(ChannelClass::Stable),
            last_writer_wins_blocked: true,
            collision_disclosure: "Administrator policy controls file-association and \
                protocol-handler registration; user cannot override; last-writer-wins blocked."
                .into(),
        },
        rollback_owner_class: RollbackOwnerClass::Admin,
        rollback_scope: ArtifactGraphRollbackScope::ManagedFleetOwned,
        rollback_target_ref: Some("fleet.rollback_target.windows.managed.stable".into()),
        side_by_side_relation_class: SideBySideRelationClass::ManagedAndPortable,
        isolation_verdict: SideBySideIsolationVerdict::Isolated,
        portable_write_guard: PortableWriteGuardClass::NotApplicable,
        portable_shell_integration: None,
        durable_state_root_refs: vec![
            "state.per_user_configuration_root.stable".into(),
            "state.per_machine_admin_policy_root.stable".into(),
            "state.per_machine_shared_data_root.stable".into(),
        ],
        exposed_in_surfaces: vec![
            InstallSurfaceClass::About,
            InstallSurfaceClass::UpdateCenter,
            InstallSurfaceClass::DiagnosticsCenter,
            InstallSurfaceClass::FleetConsole,
            InstallSurfaceClass::SupportBundle,
        ],
        stable: true,
        qualification_token: StabilizeQualificationToken::Stable,
        narrow_reason_token: StabilizeNarrowReasonToken::NotNarrowed,
        plain_language_summary: "Windows Managed per-machine Stable: fleet-owned updates, \
            admin-controlled handler registration, fleet-owned rollback, machine and user \
            state roots visible from admin support view."
            .into(),
    };

    let row_airgap = InstallProfileStableRow {
        row_id: "stabilize-portable.row.airgap.bundle.stable".into(),
        profile_card_ref: "card.airgap.bundle.stable".into(),
        display_label: "Air-gapped Bundle Stable".into(),
        platform_class: PlatformClass::AirGapBundleTarget,
        architecture_class: ArchitectureClass::X86_64,
        install_mode_class: InstallModeClass::OfflineBundle,
        channel_class: ChannelClass::Stable,
        binary_root_class: BinaryRootClass::OfflineBundleExtractedProgramArea,
        binary_root_ref: "binary_root:airgap:bundle:stable".into(),
        updater_owner_class: UpdaterOwnerClass::Admin,
        handler_ownership: HandlerOwnershipSummary {
            file_association_class: HandlerRegistrationClass::AdminOnly,
            protocol_handler_class: HandlerRegistrationClass::AdminOnly,
            owning_channel_class: Some(ChannelClass::Stable),
            last_writer_wins_blocked: true,
            collision_disclosure: "Air-gapped install handler registration is owned by the \
                offline-bundle administrator; no user-selectable default override."
                .into(),
        },
        rollback_owner_class: RollbackOwnerClass::Admin,
        rollback_scope: ArtifactGraphRollbackScope::FullArtifactGraph,
        rollback_target_ref: Some("fleet.rollback_target.airgap.bundle.stable".into()),
        side_by_side_relation_class: SideBySideRelationClass::None,
        isolation_verdict: SideBySideIsolationVerdict::NotApplicable,
        portable_write_guard: PortableWriteGuardClass::NotApplicable,
        portable_shell_integration: None,
        durable_state_root_refs: vec![
            "state.per_user_configuration_root.stable".into(),
            "state.offline_bundle_mirror_metadata_root.stable".into(),
        ],
        exposed_in_surfaces: vec![
            InstallSurfaceClass::About,
            InstallSurfaceClass::DiagnosticsCenter,
            InstallSurfaceClass::FleetConsole,
            InstallSurfaceClass::SupportBundle,
        ],
        stable: true,
        qualification_token: StabilizeQualificationToken::Stable,
        narrow_reason_token: StabilizeNarrowReasonToken::NotNarrowed,
        plain_language_summary: "Air-gapped Bundle Stable: admin-owned updates and rollback, \
            full artifact-graph rollback, offline-bundle mirror metadata root plus user config \
            root visible from admin support view."
            .into(),
    };

    let import_review_stable_to_preview = SideBySideImportReviewRow {
        review_id: "stabilize-portable.import-review.stable-to-preview.windows".into(),
        source_row_id: "stabilize-portable.row.windows.per_user.stable".into(),
        target_row_id: "stabilize-portable.row.windows.per_user.preview".into(),
        source_channel_class: ChannelClass::Stable,
        target_channel_class: ChannelClass::Preview,
        review_class: ImportReviewClass::CompareOrSkipWithCheckpoint,
        can_compare_before_apply: true,
        checkpoint_ref: Some(
            "checkpoint:import-review:stable-to-preview:windows:before-apply".into(),
        ),
        checkpoint_created_before_apply: true,
        skip_preserves_source: true,
        comparison_scope_domains: vec![
            "profile".into(),
            "settings".into(),
            "keybindings".into(),
            "snippets".into(),
            "extensions".into(),
        ],
        collision_disclosures: vec![
            "State roots are independent; no namespace collapse is permitted.".into(),
            "File-association ownership must be explicitly selected before apply.".into(),
        ],
        review_summary: "Stable-to-Preview import: compare-or-skip with checkpoint before apply; \
            state roots remain isolated; handler ownership requires explicit review."
            .into(),
    };

    let import_review_portable_to_installed = SideBySideImportReviewRow {
        review_id: "stabilize-portable.import-review.portable-to-installed.windows".into(),
        source_row_id: "stabilize-portable.row.windows.portable.stable".into(),
        target_row_id: "stabilize-portable.row.windows.per_user.stable".into(),
        source_channel_class: ChannelClass::PortableStable,
        target_channel_class: ChannelClass::Stable,
        review_class: ImportReviewClass::CompareOrSkipWithCheckpoint,
        can_compare_before_apply: true,
        checkpoint_ref: Some(
            "checkpoint:import-review:portable-to-installed:windows:before-apply".into(),
        ),
        checkpoint_created_before_apply: true,
        skip_preserves_source: true,
        comparison_scope_domains: vec!["profile".into(), "settings".into(), "keybindings".into()],
        collision_disclosures: vec![
            "Portable colocated root and installed per-user root are kept separate until \
                user explicitly applies the import."
                .into(),
            "Handler registration is absent on the portable source; installed target \
                default handler selection is not changed by this import."
                .into(),
        ],
        review_summary: "Portable-to-installed import: compare-or-skip with checkpoint; \
            handler registration unchanged; durable roots stay separate until explicit apply."
            .into(),
    };

    let fleet_diag_managed = FleetRolloutInstallDiagnosticsRow {
        row_id: "stabilize-portable.fleet-diag.windows.managed.stable".into(),
        install_profile_row_id: "stabilize-portable.row.windows.managed.stable".into(),
        platform_token: "windows".into(),
        updater_owner_class: UpdaterOwnerClass::ManagedFleet,
        channel_class: ChannelClass::Stable,
        rollback_scope: ArtifactGraphRollbackScope::ManagedFleetOwned,
        identity_preserved_in_export: true,
        channel_separation_maintained_on_revert: true,
        fleet_diagnostics_summary: "Windows managed-fleet diagnostics: install-profile identity \
            (channel, ring, updater owner, binary root, state roots) preserved through log \
            export and rollout revert; channel separation maintained."
            .into(),
    };

    let fleet_diag_airgap = FleetRolloutInstallDiagnosticsRow {
        row_id: "stabilize-portable.fleet-diag.airgap.bundle.stable".into(),
        install_profile_row_id: "stabilize-portable.row.airgap.bundle.stable".into(),
        platform_token: "air_gap_bundle_target".into(),
        updater_owner_class: UpdaterOwnerClass::Admin,
        channel_class: ChannelClass::Stable,
        rollback_scope: ArtifactGraphRollbackScope::FullArtifactGraph,
        identity_preserved_in_export: true,
        channel_separation_maintained_on_revert: true,
        fleet_diagnostics_summary: "Air-gapped bundle diagnostics: install-profile identity and \
            offline-bundle mirror metadata preserved through diagnostic export and rollout \
            revert; no channel namespace collapse."
            .into(),
    };

    let summary = StabilizePortableInstallSummary {
        install_profile_row_count: 5,
        stable_row_count: 5,
        beta_row_count: 0,
        preview_row_count: 0,
        withdrawn_row_count: 0,
        install_modes_covered: vec![
            InstallModeClass::PerUserInstalled,
            InstallModeClass::SideBySidePreview,
            InstallModeClass::Portable,
            InstallModeClass::ManagedDeployed,
            InstallModeClass::OfflineBundle,
        ],
        channels_covered: vec![
            ChannelClass::Stable,
            ChannelClass::Preview,
            ChannelClass::PortableStable,
        ],
        portable_write_guard_valid_count: 1,
        import_review_compare_or_skip_count: 2,
        overall_qualification_token: StabilizeQualificationToken::Stable,
    };

    StabilizePortableInstallPage {
        record_kind: STABILIZE_PORTABLE_INSTALL_PAGE_RECORD_KIND.to_string(),
        schema_version: STABILIZE_PORTABLE_INSTALL_SCHEMA_VERSION,
        shared_contract_ref: STABILIZE_PORTABLE_INSTALL_SHARED_CONTRACT_REF.to_string(),
        page_id: "stabilize-portable-install:seeded:0001".into(),
        page_label: "Stabilized Install Profiles — Portable, Side-by-Side, and Managed Lanes"
            .into(),
        generated_at: "2026-06-01T00:00:00Z".into(),
        install_profile_rows: vec![
            row_user_stable,
            row_user_preview,
            row_portable_stable,
            row_managed_stable,
            row_airgap,
        ],
        import_review_rows: vec![
            import_review_stable_to_preview,
            import_review_portable_to_installed,
        ],
        fleet_rollout_diagnostics_rows: vec![fleet_diag_managed, fleet_diag_airgap],
        defects: vec![],
        summary,
    }
}
