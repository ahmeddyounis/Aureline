//! Install/update/remove review sheets and lockfile-impact cards carrying the
//! real dependency-mutation blast radius before Aureline writes any manifest or
//! lockfile.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`] — the
//! `install_review_sheet` and the `lockfile_impact_card` — into one implemented,
//! export-safe packet with two co-equal control vectors. Together they preview
//! the whole blast radius of an install, update, or remove *before* any write:
//! which manifests and lockfiles change, the version delta, peer/runtime shifts,
//! validation expectations, registry/auth state, and the checkpoint/rollback
//! actions on offer.
//!
//! An [`InstallReviewSheet`] always names the affected manifests, the version
//! delta, the validation tasks, and the registry/auth state, and its change
//! breadth (small-single, grouped, or broad) is *derived* from the affected
//! manifest/lockfile counts, the transitive churn, and any peer conflicts rather
//! than asserted — so a broad change that regenerates several lockfiles or must
//! resolve a peer conflict can never present as a small isolated one, and no
//! generic confirm hides a manifest write, lockfile churn, a peer conflict, or a
//! validation expectation.
//!
//! A [`LockfileImpactCard`] always names the resolver identity and version, the
//! affected lockfiles, the direct/transitive churn, and whether the write is a
//! regenerate-from-source or an in-place edit. Its churn magnitude is *derived*
//! from the change counts, and its rollback posture is *derived* from the write
//! mode — a regenerate-only lockfile can never claim a manual-edit write-back —
//! so lockfile churn is never understated and platform/tool-version sensitivity
//! stays explicit.
//!
//! The registry/resolution degradation vocabulary
//! ([`M5PackageComponentDegradationState`]) and rollback posture
//! ([`M5PackageComponentRollbackPosture`]) are reused directly from the frozen
//! matrix, as are the downgrade triggers
//! ([`M5PackageComponentDowngradeTrigger`]) and consumer surfaces
//! ([`M5PackageComponentConsumerSurface`]).
//!
//! Raw manifest bodies, raw lockfile bodies, registry credentials, private
//! registry URLs, and live registry responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-install-review-lockfile-controls.schema.json`](../../../../schemas/ui/m5-install-review-lockfile-controls.schema.json).
//! The contract doc is
//! [`docs/deps/m5/implement_install_review_sheets_and_lockfile_impact_cards.md`](../../../../docs/deps/m5/implement_install_review_sheets_and_lockfile_impact_cards.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-install-review-lockfile-controls/`](../../../../fixtures/ui/m5-install-review-lockfile-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::{
    M5PackageComponent, M5PackageComponentConsumerSurface, M5PackageComponentDegradationState,
    M5PackageComponentDowngradeTrigger, M5PackageComponentRollbackPosture,
    M5_PACKAGE_COMPONENT_MATRIX_DOC_REF, M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`InstallReviewLockfileControlsPacket`].
pub const INSTALL_REVIEW_LOCKFILE_RECORD_KIND: &str = "install_review_lockfile_controls";

/// Schema version for install-review / lockfile-impact control records.
pub const INSTALL_REVIEW_LOCKFILE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INSTALL_REVIEW_LOCKFILE_SCHEMA_REF: &str =
    "schemas/ui/m5-install-review-lockfile-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const INSTALL_REVIEW_LOCKFILE_DOC_REF: &str =
    "docs/deps/m5/implement_install_review_sheets_and_lockfile_impact_cards.md";

/// Repo-relative path of the protected fixture directory.
pub const INSTALL_REVIEW_LOCKFILE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-install-review-lockfile-controls";

/// Repo-relative path of the checked support-export artifact.
pub const INSTALL_REVIEW_LOCKFILE_ARTIFACT_REF: &str =
    "artifacts/release/m5-install-review-lockfile-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const INSTALL_REVIEW_LOCKFILE_SUMMARY_REF: &str =
    "artifacts/release/m5-install-review-lockfile-proof/summary.md";

/// Transitive-churn count at or above which a change is at least grouped.
pub const GROUPED_TRANSITIVE_CHURN_THRESHOLD: u32 = 6;

/// Transitive-churn count at or above which a change is broad.
pub const BROAD_TRANSITIVE_CHURN_THRESHOLD: u32 = 25;

/// Total lockfile-change count at or below which churn is narrow.
pub const NARROW_LOCKFILE_CHURN_THRESHOLD: u32 = 5;

/// Total lockfile-change count at or below which churn is moderate.
pub const MODERATE_LOCKFILE_CHURN_THRESHOLD: u32 = 25;

/// The package mutation an install-review sheet previews.
///
/// The three reviewed flows — install, update, remove — stay distinct so a
/// remove is never flattened into a generic "apply change" and each carries the
/// blast radius appropriate to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationOperationClass {
    /// Install one or more new dependencies.
    Install,
    /// Update one or more existing dependencies.
    Update,
    /// Remove one or more dependencies.
    Remove,
}

impl MutationOperationClass {
    /// Every operation, in declaration order.
    pub const ALL: [Self; 3] = [Self::Install, Self::Update, Self::Remove];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Remove => "remove",
        }
    }
}

/// Derived change breadth an install-review sheet may present.
///
/// This is the sheet honesty axis and the AC's "quantify whether the change is
/// small, grouped, or broad enough to warrant deeper inspection": the breadth is
/// derived from the affected manifest/lockfile counts, the transitive churn, and
/// any peer conflicts, never asserted, so a broad change can never read as a
/// small isolated one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewChangeBreadth {
    /// A single-manifest, single-lockfile change with little transitive churn.
    SmallSingle,
    /// A grouped change touching several manifests or a bounded churn set.
    GroupedChange,
    /// A broad change: many transitive edits, several lockfiles, or a peer conflict.
    BroadChange,
}

impl ReviewChangeBreadth {
    /// Every breadth, in declaration order.
    pub const ALL: [Self; 3] = [Self::SmallSingle, Self::GroupedChange, Self::BroadChange];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SmallSingle => "small_single",
            Self::GroupedChange => "grouped_change",
            Self::BroadChange => "broad_change",
        }
    }

    /// Whether this breadth warrants deeper inspection before applying.
    pub const fn warrants_deeper_inspection(self) -> bool {
        !matches!(self, Self::SmallSingle)
    }
}

/// Disclosures an install-review sheet must carry, derived from its blast radius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewChangeDisclosure {
    /// The derived change breadth this sheet may present.
    pub change_breadth: ReviewChangeBreadth,
    /// Whether the change warrants deeper inspection before applying.
    pub warrants_deeper_inspection: bool,
    /// Whether the sheet must carry a peer/runtime shift note.
    pub needs_peer_runtime_note: bool,
    /// Whether the sheet must carry a broad-change note.
    pub needs_broad_change_note: bool,
}

/// Resolves the change breadth an install-review sheet may present.
///
/// A change is broad when it must resolve a peer conflict, regenerates more than
/// one lockfile, or crosses the broad transitive-churn threshold. Otherwise it is
/// grouped when it touches several manifests, is explicitly grouped, or crosses
/// the grouped transitive-churn threshold. Otherwise it is a small single change.
pub fn resolve_review_change_breadth(
    peer_conflict_count: u32,
    affected_manifest_count: u32,
    affected_lockfile_count: u32,
    transitive_churn_count: u32,
    is_grouped: bool,
) -> ReviewChangeDisclosure {
    let is_broad = peer_conflict_count > 0
        || affected_lockfile_count > 1
        || transitive_churn_count >= BROAD_TRANSITIVE_CHURN_THRESHOLD;
    let is_grouped_change = !is_broad
        && (is_grouped
            || affected_manifest_count > 1
            || transitive_churn_count >= GROUPED_TRANSITIVE_CHURN_THRESHOLD);

    let change_breadth = if is_broad {
        ReviewChangeBreadth::BroadChange
    } else if is_grouped_change {
        ReviewChangeBreadth::GroupedChange
    } else {
        ReviewChangeBreadth::SmallSingle
    };

    ReviewChangeDisclosure {
        change_breadth,
        warrants_deeper_inspection: change_breadth.warrants_deeper_inspection(),
        needs_peer_runtime_note: peer_conflict_count > 0,
        needs_broad_change_note: is_broad,
    }
}

/// An install/update/remove review sheet previewing the mutation blast radius.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewSheet {
    /// Frozen component this control implements; must be `install_review_sheet`.
    pub component: M5PackageComponent,
    /// Stable sheet id.
    pub sheet_id: String,
    /// The package mutation this sheet previews.
    pub operation: MutationOperationClass,
    /// Human-readable package (or package-set) label; required and non-empty.
    pub package_label: String,
    /// Manifests this mutation writes; always required and non-empty (no hiding).
    pub affected_manifests: Vec<String>,
    /// Lockfiles this mutation regenerates or edits; may be empty for a
    /// manifest-only change.
    pub affected_lockfiles: Vec<String>,
    /// Version delta this mutation applies; required and non-empty.
    pub version_delta: String,
    /// Peer / runtime shift note; required when a peer conflict is present.
    pub peer_runtime_shift_note: String,
    /// Number of peer conflicts this mutation must resolve.
    pub peer_conflict_count: u32,
    /// Transitive-dependency churn count this mutation introduces.
    pub transitive_churn_count: u32,
    /// Whether this mutation is part of a grouped update.
    pub is_grouped: bool,
    /// Change breadth; derived and validated against the blast radius.
    pub change_breadth: ReviewChangeBreadth,
    /// Whether the change warrants deeper inspection; derived and validated.
    pub warrants_deeper_inspection: bool,
    /// Deeper-inspection note; required when deeper inspection is warranted.
    pub deeper_inspection_note: String,
    /// Broad-change note; required when the change is broad.
    pub broad_change_note: String,
    /// Validation tasks this mutation expects; required and non-empty.
    pub validation_tasks: Vec<String>,
    /// Registry / auth state note; required and non-empty.
    pub registry_auth_state_note: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// Checkpoint action label offered before write; required and non-empty.
    pub checkpoint_action_label: String,
    /// Rollback action label offered before write; required and non-empty.
    pub rollback_action_label: String,
    /// Rollback / write-back posture, reused from the frozen matrix.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this sheet.
    pub source_contract_refs: Vec<String>,
}

impl InstallReviewSheet {
    /// Change-breadth disclosures this sheet must carry, derived from blast radius.
    pub fn change_disclosure(&self) -> ReviewChangeDisclosure {
        resolve_review_change_breadth(
            self.peer_conflict_count,
            self.affected_manifests.len() as u32,
            self.affected_lockfiles.len() as u32,
            self.transitive_churn_count,
            self.is_grouped,
        )
    }

    /// Whether the rollback posture is consistent with a preview-first review sheet.
    ///
    /// A review sheet previews a mutation and never writes until an explicit
    /// apply, so it must be staged-review or write-back-behind-a-checkpoint.
    pub fn rollback_posture_consistent(&self) -> bool {
        matches!(
            self.rollback_posture,
            M5PackageComponentRollbackPosture::StagedReviewNoWrite
                | M5PackageComponentRollbackPosture::WriteBackCheckpointed
        )
    }
}

/// How a lockfile-impact card's write reaches the lockfile.
///
/// This is the AC's regenerate-versus-edit mode, kept explicit so a
/// regenerate-from-source write is never confused with a surgical in-place edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileWriteMode {
    /// The whole lockfile regenerates from the manifest; no manual edit is kept.
    RegenerateWholeLockfile,
    /// Only the affected lockfile entries are edited in place.
    EditInPlaceEntries,
    /// The mutation writes no lockfile (e.g. a manifest-only or tool change).
    NoLockfileWrite,
}

impl LockfileWriteMode {
    /// Every write mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RegenerateWholeLockfile,
        Self::EditInPlaceEntries,
        Self::NoLockfileWrite,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegenerateWholeLockfile => "regenerate_whole_lockfile",
            Self::EditInPlaceEntries => "edit_in_place_entries",
            Self::NoLockfileWrite => "no_lockfile_write",
        }
    }

    /// The rollback posture this write mode implies.
    ///
    /// A regenerate-from-source lockfile regenerates rather than accepting manual
    /// edits, an in-place edit writes back behind a durable checkpoint, and a
    /// no-lockfile-write card mutates nothing.
    pub const fn expected_rollback_posture(self) -> M5PackageComponentRollbackPosture {
        match self {
            Self::RegenerateWholeLockfile => {
                M5PackageComponentRollbackPosture::RegenerateOnlyNoManualEdit
            }
            Self::EditInPlaceEntries => M5PackageComponentRollbackPosture::WriteBackCheckpointed,
            Self::NoLockfileWrite => M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
        }
    }
}

/// Derived lockfile-churn magnitude a lockfile-impact card may present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileChurnMagnitude {
    /// No lockfile entries change.
    NoChurn,
    /// A narrow set of lockfile entries change.
    NarrowChurn,
    /// A moderate set of lockfile entries change.
    ModerateChurn,
    /// A broad set of lockfile entries change.
    BroadChurn,
}

impl LockfileChurnMagnitude {
    /// Every magnitude, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoChurn,
        Self::NarrowChurn,
        Self::ModerateChurn,
        Self::BroadChurn,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChurn => "no_churn",
            Self::NarrowChurn => "narrow_churn",
            Self::ModerateChurn => "moderate_churn",
            Self::BroadChurn => "broad_churn",
        }
    }
}

/// Disclosures a lockfile-impact card must carry, derived from its churn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockfileChurnDisclosure {
    /// The derived churn magnitude this card may present.
    pub churn_magnitude: LockfileChurnMagnitude,
    /// Whether the card must carry a churn note.
    pub needs_churn_note: bool,
    /// Whether the card must carry a platform / tool-version sensitivity note.
    pub needs_platform_tool_note: bool,
    /// Whether the churn is broad regeneration.
    pub is_broad_regeneration: bool,
}

/// Resolves the churn magnitude a lockfile-impact card may present.
///
/// Magnitude is derived from the total direct-plus-transitive change count so
/// lockfile churn is quantified, never understated; a platform- or
/// tool-version-sensitive resolution always carries an explicit note.
pub fn resolve_lockfile_churn(
    direct_change_count: u32,
    transitive_churn_count: u32,
    platform_sensitive: bool,
    tool_version_sensitive: bool,
) -> LockfileChurnDisclosure {
    let total = direct_change_count.saturating_add(transitive_churn_count);
    let churn_magnitude = if total == 0 {
        LockfileChurnMagnitude::NoChurn
    } else if total <= NARROW_LOCKFILE_CHURN_THRESHOLD {
        LockfileChurnMagnitude::NarrowChurn
    } else if total <= MODERATE_LOCKFILE_CHURN_THRESHOLD {
        LockfileChurnMagnitude::ModerateChurn
    } else {
        LockfileChurnMagnitude::BroadChurn
    };

    LockfileChurnDisclosure {
        churn_magnitude,
        needs_churn_note: !matches!(churn_magnitude, LockfileChurnMagnitude::NoChurn),
        needs_platform_tool_note: platform_sensitive || tool_version_sensitive,
        is_broad_regeneration: matches!(churn_magnitude, LockfileChurnMagnitude::BroadChurn),
    }
}

/// A lockfile-impact card quantifying resolver identity, churn, and write mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileImpactCard {
    /// Frozen component this control implements; must be `lockfile_impact_card`.
    pub component: M5PackageComponent,
    /// Stable card id.
    pub card_id: String,
    /// Resolver identity label; required and non-empty.
    pub resolver_label: String,
    /// Resolver version; required and non-empty.
    pub resolver_version: String,
    /// Lockfiles this write affects; always required and non-empty.
    pub affected_lockfiles: Vec<String>,
    /// Number of directly changed lockfile entries.
    pub direct_change_count: u32,
    /// Number of transitively churned lockfile entries.
    pub transitive_churn_count: u32,
    /// Churn magnitude; derived and validated against the change counts.
    pub churn_magnitude: LockfileChurnMagnitude,
    /// Churn note; required when there is any churn.
    pub churn_note: String,
    /// Whether the resolution is platform-sensitive.
    pub platform_sensitive: bool,
    /// Whether the resolution is tool-version-sensitive.
    pub tool_version_sensitive: bool,
    /// Platform / tool-version note; required when either sensitivity holds.
    pub platform_tool_note: String,
    /// How this write reaches the lockfile (regenerate versus edit).
    pub write_mode: LockfileWriteMode,
    /// Write-mode note; always required and non-empty.
    pub write_mode_note: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// Rollback / write-back posture; derived from and validated against write mode.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
}

impl LockfileImpactCard {
    /// Churn disclosures this card must carry, derived from its change counts.
    pub fn churn_disclosure(&self) -> LockfileChurnDisclosure {
        resolve_lockfile_churn(
            self.direct_change_count,
            self.transitive_churn_count,
            self.platform_sensitive,
            self.tool_version_sensitive,
        )
    }

    /// Whether the rollback posture is consistent with the card's write mode.
    ///
    /// A regenerate-only lockfile can never claim a manual-edit write-back.
    pub fn rollback_posture_consistent(&self) -> bool {
        self.rollback_posture == self.write_mode.expected_rollback_posture()
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewLockfileTrustReview {
    /// Manifest writes are always explicit.
    pub manifest_writes_always_explicit: bool,
    /// Lockfile churn is never understated.
    pub lockfile_churn_never_understated: bool,
    /// The version delta is always explicit.
    pub version_delta_always_explicit: bool,
    /// Peer / runtime shifts stay explicit.
    pub peer_runtime_shifts_explicit: bool,
    /// Validation expectations stay explicit.
    pub validation_expectations_explicit: bool,
    /// Registry / auth state stays explicit.
    pub registry_auth_state_explicit: bool,
    /// The change breadth is quantified (small / grouped / broad).
    pub change_breadth_quantified: bool,
    /// The resolver identity is always named.
    pub resolver_identity_always_named: bool,
    /// Platform / tool-version sensitivity stays explicit.
    pub platform_tool_sensitivity_explicit: bool,
    /// The regenerate-versus-edit mode stays explicit.
    pub regenerate_versus_edit_explicit: bool,
    /// A rollback / checkpoint action is always offered before write.
    pub rollback_checkpoint_always_offered: bool,
    /// No generic confirm language conceals scope, churn, peers, or validation.
    pub no_generic_confirm_language: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl InstallReviewLockfileTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.manifest_writes_always_explicit
            && self.lockfile_churn_never_understated
            && self.version_delta_always_explicit
            && self.peer_runtime_shifts_explicit
            && self.validation_expectations_explicit
            && self.registry_auth_state_explicit
            && self.change_breadth_quantified
            && self.resolver_identity_always_named
            && self.platform_tool_sensitivity_explicit
            && self.regenerate_versus_edit_explicit
            && self.rollback_checkpoint_always_offered
            && self.no_generic_confirm_language
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewLockfileConsumerProjection {
    /// The install-review sheet shows the affected scope and lockfile churn.
    pub install_review_sheet_shows_scope_and_churn: bool,
    /// The version delta and peer/runtime shifts are shown inline.
    pub version_delta_and_peer_shifts_shown_inline: bool,
    /// Validation tasks and registry/auth state are shown inline.
    pub validation_and_registry_state_shown_inline: bool,
    /// The lockfile-impact card shows the resolver identity and churn.
    pub lockfile_card_shows_resolver_and_churn: bool,
    /// The regenerate-versus-edit mode is shown inline.
    pub regenerate_versus_edit_shown_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl InstallReviewLockfileConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.install_review_sheet_shows_scope_and_churn
            && self.version_delta_and_peer_shifts_shown_inline
            && self.validation_and_registry_state_shown_inline
            && self.lockfile_card_shows_resolver_and_churn
            && self.regenerate_versus_edit_shown_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewLockfileProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`InstallReviewLockfileControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallReviewLockfileControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Install-review sheets.
    pub install_review_sheets: Vec<InstallReviewSheet>,
    /// Lockfile-impact cards.
    pub lockfile_impact_cards: Vec<LockfileImpactCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: InstallReviewLockfileTrustReview,
    /// Consumer projection block.
    pub consumer_projection: InstallReviewLockfileConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: InstallReviewLockfileProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe install-review / lockfile-impact controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewLockfileControlsPacket {
    /// Record kind; must equal [`INSTALL_REVIEW_LOCKFILE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INSTALL_REVIEW_LOCKFILE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Install-review sheets.
    pub install_review_sheets: Vec<InstallReviewSheet>,
    /// Lockfile-impact cards.
    pub lockfile_impact_cards: Vec<LockfileImpactCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: InstallReviewLockfileTrustReview,
    /// Consumer projection block.
    pub consumer_projection: InstallReviewLockfileConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: InstallReviewLockfileProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl InstallReviewLockfileControlsPacket {
    /// Builds an install-review / lockfile-impact controls packet from stable-lane input.
    pub fn new(input: InstallReviewLockfileControlsPacketInput) -> Self {
        Self {
            record_kind: INSTALL_REVIEW_LOCKFILE_RECORD_KIND.to_owned(),
            schema_version: INSTALL_REVIEW_LOCKFILE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            install_review_sheets: input.install_review_sheets,
            lockfile_impact_cards: input.lockfile_impact_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the install-review / lockfile-impact control invariants.
    pub fn validate(&self) -> Vec<InstallReviewLockfileViolation> {
        let mut violations = Vec::new();

        if self.record_kind != INSTALL_REVIEW_LOCKFILE_RECORD_KIND {
            violations.push(InstallReviewLockfileViolation::WrongRecordKind);
        }
        if self.schema_version != INSTALL_REVIEW_LOCKFILE_SCHEMA_VERSION {
            violations.push(InstallReviewLockfileViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(InstallReviewLockfileViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(InstallReviewLockfileViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(InstallReviewLockfileViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_review_sheets(self, &mut violations);
        validate_lockfile_cards(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(InstallReviewLockfileViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(InstallReviewLockfileViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(InstallReviewLockfileViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("install review lockfile packet serializes"),
        ) {
            violations.push(InstallReviewLockfileViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("install review lockfile packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let broad_sheets = self
            .install_review_sheets
            .iter()
            .filter(|sheet| sheet.change_breadth == ReviewChangeBreadth::BroadChange)
            .count();
        let broad_cards = self
            .lockfile_impact_cards
            .iter()
            .filter(|card| card.churn_magnitude == LockfileChurnMagnitude::BroadChurn)
            .count();

        let mut out = String::new();
        out.push_str("# Install-review sheets and lockfile-impact cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Review sheets: {} ({} broad)\n",
            self.install_review_sheets.len(),
            broad_sheets
        ));
        out.push_str(&format!(
            "- Lockfile-impact cards: {} ({} broad churn)\n",
            self.lockfile_impact_cards.len(),
            broad_cards
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Install-review sheets\n\n");
        for sheet in &self.install_review_sheets {
            out.push_str(&format!(
                "- **{}** ({}) — breadth `{}`, {} manifest(s), {} transitive churn\n",
                sheet.package_label,
                sheet.operation.as_str(),
                sheet.change_breadth.as_str(),
                sheet.affected_manifests.len(),
                sheet.transitive_churn_count
            ));
        }

        out.push_str("\n## Lockfile-impact cards\n\n");
        for card in &self.lockfile_impact_cards {
            out.push_str(&format!(
                "- **{} {}** — churn `{}` [{}], {} lockfile(s)\n",
                card.resolver_label,
                card.resolver_version,
                card.churn_magnitude.as_str(),
                card.write_mode.as_str(),
                card.affected_lockfiles.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in install-review / lockfile export.
#[derive(Debug)]
pub enum InstallReviewLockfileArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<InstallReviewLockfileViolation>),
}

impl fmt::Display for InstallReviewLockfileArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "install review lockfile export parse failed: {error}"
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
                    "install review lockfile export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for InstallReviewLockfileArtifactError {}

/// Validation failures emitted by [`InstallReviewLockfileControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstallReviewLockfileViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No install-review sheets are present.
    ReviewSheetsMissing,
    /// A review sheet is incomplete.
    ReviewSheetIncomplete,
    /// A review sheet carries the wrong frozen component class.
    ReviewSheetWrongComponentClass,
    /// A review sheet does not name its package label.
    PackageLabelMissing,
    /// A review sheet does not name the manifests it writes.
    AffectedManifestsMissing,
    /// A review sheet does not name its version delta.
    VersionDeltaMissing,
    /// A review sheet misrepresents its change breadth relative to blast radius.
    ChangeBreadthMisrepresented,
    /// A sheet warranting deeper inspection does not carry a deeper-inspection note.
    DeeperInspectionNoteMissing,
    /// A sheet with peer conflicts does not carry a peer/runtime shift note.
    PeerRuntimeNoteMissing,
    /// A broad-change sheet does not carry a broad-change note.
    BroadChangeNoteMissing,
    /// A review sheet does not name its validation expectations.
    ValidationTasksMissing,
    /// A review sheet does not name its registry/auth state.
    RegistryAuthStateMissing,
    /// A degraded review sheet does not carry a degradation note.
    ReviewSheetDegradationNoteMissing,
    /// A review sheet does not offer a checkpoint / rollback action.
    CheckpointActionMissing,
    /// A review-sheet rollback posture is inconsistent with a preview-first sheet.
    ReviewSheetRollbackPostureInconsistent,
    /// The review sheets do not cover install, update, and remove.
    OperationCoverageMissing,
    /// The review sheets do not cover small, grouped, and broad change breadths.
    BreadthCoverageMissing,
    /// No lockfile-impact cards are present.
    LockfileCardsMissing,
    /// A lockfile-impact card is incomplete.
    LockfileCardIncomplete,
    /// A lockfile-impact card carries the wrong frozen component class.
    LockfileCardWrongComponentClass,
    /// A lockfile-impact card does not name its resolver identity/version.
    ResolverIdentityMissing,
    /// A lockfile-impact card does not name the lockfiles it affects.
    AffectedLockfilesMissing,
    /// A lockfile-impact card misrepresents its churn magnitude.
    ChurnMagnitudeMisrepresented,
    /// A card with churn does not carry a churn note.
    ChurnNoteMissing,
    /// A platform/tool-sensitive card does not carry a sensitivity note.
    PlatformToolNoteMissing,
    /// A lockfile-impact card does not name its regenerate-versus-edit mode note.
    WriteModeNoteMissing,
    /// A degraded lockfile-impact card does not carry a degradation note.
    LockfileCardDegradationNoteMissing,
    /// A lockfile-impact card rollback posture is inconsistent with its write mode.
    CardRollbackPostureInconsistent,
    /// The cards do not cover both regenerate and edit write modes.
    WriteModeCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl InstallReviewLockfileViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ReviewSheetsMissing => "review_sheets_missing",
            Self::ReviewSheetIncomplete => "review_sheet_incomplete",
            Self::ReviewSheetWrongComponentClass => "review_sheet_wrong_component_class",
            Self::PackageLabelMissing => "package_label_missing",
            Self::AffectedManifestsMissing => "affected_manifests_missing",
            Self::VersionDeltaMissing => "version_delta_missing",
            Self::ChangeBreadthMisrepresented => "change_breadth_misrepresented",
            Self::DeeperInspectionNoteMissing => "deeper_inspection_note_missing",
            Self::PeerRuntimeNoteMissing => "peer_runtime_note_missing",
            Self::BroadChangeNoteMissing => "broad_change_note_missing",
            Self::ValidationTasksMissing => "validation_tasks_missing",
            Self::RegistryAuthStateMissing => "registry_auth_state_missing",
            Self::ReviewSheetDegradationNoteMissing => "review_sheet_degradation_note_missing",
            Self::CheckpointActionMissing => "checkpoint_action_missing",
            Self::ReviewSheetRollbackPostureInconsistent => {
                "review_sheet_rollback_posture_inconsistent"
            }
            Self::OperationCoverageMissing => "operation_coverage_missing",
            Self::BreadthCoverageMissing => "breadth_coverage_missing",
            Self::LockfileCardsMissing => "lockfile_cards_missing",
            Self::LockfileCardIncomplete => "lockfile_card_incomplete",
            Self::LockfileCardWrongComponentClass => "lockfile_card_wrong_component_class",
            Self::ResolverIdentityMissing => "resolver_identity_missing",
            Self::AffectedLockfilesMissing => "affected_lockfiles_missing",
            Self::ChurnMagnitudeMisrepresented => "churn_magnitude_misrepresented",
            Self::ChurnNoteMissing => "churn_note_missing",
            Self::PlatformToolNoteMissing => "platform_tool_note_missing",
            Self::WriteModeNoteMissing => "write_mode_note_missing",
            Self::LockfileCardDegradationNoteMissing => "lockfile_card_degradation_note_missing",
            Self::CardRollbackPostureInconsistent => "card_rollback_posture_inconsistent",
            Self::WriteModeCoverageMissing => "write_mode_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable install-review / lockfile export.
pub fn current_install_review_lockfile_export(
) -> Result<InstallReviewLockfileControlsPacket, InstallReviewLockfileArtifactError> {
    let packet: InstallReviewLockfileControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-install-review-lockfile-proof/support_export.json"
    )))
    .map_err(InstallReviewLockfileArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(InstallReviewLockfileArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &InstallReviewLockfileControlsPacket,
    violations: &mut Vec<InstallReviewLockfileViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        INSTALL_REVIEW_LOCKFILE_SCHEMA_REF,
        INSTALL_REVIEW_LOCKFILE_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(InstallReviewLockfileViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_review_sheets(
    packet: &InstallReviewLockfileControlsPacket,
    violations: &mut Vec<InstallReviewLockfileViolation>,
) {
    if packet.install_review_sheets.is_empty() {
        violations.push(InstallReviewLockfileViolation::ReviewSheetsMissing);
        return;
    }

    let mut operations: BTreeSet<MutationOperationClass> = BTreeSet::new();
    let mut breadths: BTreeSet<ReviewChangeBreadth> = BTreeSet::new();

    for sheet in &packet.install_review_sheets {
        operations.insert(sheet.operation);
        breadths.insert(sheet.change_breadth);

        if sheet.sheet_id.trim().is_empty()
            || sheet.fields_shown.is_empty()
            || sheet.source_contract_refs.is_empty()
        {
            violations.push(InstallReviewLockfileViolation::ReviewSheetIncomplete);
        }
        if sheet.component != M5PackageComponent::InstallReviewSheet {
            violations.push(InstallReviewLockfileViolation::ReviewSheetWrongComponentClass);
        }
        if sheet.package_label.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::PackageLabelMissing);
        }
        if sheet.affected_manifests.is_empty()
            || sheet.affected_manifests.iter().any(|m| m.trim().is_empty())
        {
            violations.push(InstallReviewLockfileViolation::AffectedManifestsMissing);
        }
        if sheet.version_delta.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::VersionDeltaMissing);
        }
        if sheet.validation_tasks.is_empty()
            || sheet.validation_tasks.iter().any(|t| t.trim().is_empty())
        {
            violations.push(InstallReviewLockfileViolation::ValidationTasksMissing);
        }
        if sheet.registry_auth_state_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::RegistryAuthStateMissing);
        }
        if sheet.checkpoint_action_label.trim().is_empty()
            || sheet.rollback_action_label.trim().is_empty()
        {
            violations.push(InstallReviewLockfileViolation::CheckpointActionMissing);
        }
        if !matches!(
            sheet.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && sheet.degradation_note.trim().is_empty()
        {
            violations.push(InstallReviewLockfileViolation::ReviewSheetDegradationNoteMissing);
        }

        let disclosure = sheet.change_disclosure();

        if sheet.change_breadth != disclosure.change_breadth
            || sheet.warrants_deeper_inspection != disclosure.warrants_deeper_inspection
        {
            violations.push(InstallReviewLockfileViolation::ChangeBreadthMisrepresented);
        }
        if disclosure.warrants_deeper_inspection && sheet.deeper_inspection_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::DeeperInspectionNoteMissing);
        }
        if disclosure.needs_peer_runtime_note && sheet.peer_runtime_shift_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::PeerRuntimeNoteMissing);
        }
        if disclosure.needs_broad_change_note && sheet.broad_change_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::BroadChangeNoteMissing);
        }
        if !sheet.rollback_posture_consistent() {
            violations.push(InstallReviewLockfileViolation::ReviewSheetRollbackPostureInconsistent);
        }
    }

    for required in MutationOperationClass::ALL {
        if !operations.contains(&required) {
            violations.push(InstallReviewLockfileViolation::OperationCoverageMissing);
            break;
        }
    }
    for required in ReviewChangeBreadth::ALL {
        if !breadths.contains(&required) {
            violations.push(InstallReviewLockfileViolation::BreadthCoverageMissing);
            break;
        }
    }
}

fn validate_lockfile_cards(
    packet: &InstallReviewLockfileControlsPacket,
    violations: &mut Vec<InstallReviewLockfileViolation>,
) {
    if packet.lockfile_impact_cards.is_empty() {
        violations.push(InstallReviewLockfileViolation::LockfileCardsMissing);
        return;
    }

    let mut write_modes: BTreeSet<LockfileWriteMode> = BTreeSet::new();

    for card in &packet.lockfile_impact_cards {
        write_modes.insert(card.write_mode);

        if card.card_id.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(InstallReviewLockfileViolation::LockfileCardIncomplete);
        }
        if card.component != M5PackageComponent::LockfileImpactCard {
            violations.push(InstallReviewLockfileViolation::LockfileCardWrongComponentClass);
        }
        if card.resolver_label.trim().is_empty() || card.resolver_version.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::ResolverIdentityMissing);
        }
        if card.affected_lockfiles.is_empty()
            || card.affected_lockfiles.iter().any(|l| l.trim().is_empty())
        {
            violations.push(InstallReviewLockfileViolation::AffectedLockfilesMissing);
        }
        if card.write_mode_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::WriteModeNoteMissing);
        }
        if !matches!(
            card.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && card.degradation_note.trim().is_empty()
        {
            violations.push(InstallReviewLockfileViolation::LockfileCardDegradationNoteMissing);
        }

        let disclosure = card.churn_disclosure();

        if card.churn_magnitude != disclosure.churn_magnitude {
            violations.push(InstallReviewLockfileViolation::ChurnMagnitudeMisrepresented);
        }
        if disclosure.needs_churn_note && card.churn_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::ChurnNoteMissing);
        }
        if disclosure.needs_platform_tool_note && card.platform_tool_note.trim().is_empty() {
            violations.push(InstallReviewLockfileViolation::PlatformToolNoteMissing);
        }
        if !card.rollback_posture_consistent() {
            violations.push(InstallReviewLockfileViolation::CardRollbackPostureInconsistent);
        }
    }

    if !write_modes.contains(&LockfileWriteMode::RegenerateWholeLockfile)
        || !write_modes.contains(&LockfileWriteMode::EditInPlaceEntries)
    {
        violations.push(InstallReviewLockfileViolation::WriteModeCoverageMissing);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
