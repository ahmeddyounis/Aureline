//! M05-899 surface certification over the frozen M5 local-history / write-scope component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`])
//! defines the seven reusable local-history-row, checkpoint-group-card,
//! restore-preview-card, retention/export-card, write-scope-preview-tree,
//! restore-granularity-selector, and history-export-manifest components, the M05-893..896
//! primitive lanes narrow each one, the M05-897 consumer lane
//! ([`crate::add_shared_rename_refactor_replace_import_repair_generated_artifact_and_ai_review_consumers_so_local_history_and_write_scope_components_keep_checkpoint_rollback_language_aligned_across_claimed_m5_mutation_surfaces`])
//! proves they are reusable across the claimed mutation / recovery consumers, and the
//! M05-898 accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_capture_is_metadata_only_restore_is_partial_or_manual_scope_is_stale_or_checkpoints_are_unavailable_across_claimed_m5_recovery_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing
//! capstone *certifies* that the shared local-history / write-scope component truth holds on
//! every claimed M5 mutation and recovery surface — and auto-narrows any surface that cannot
//! sustain it.
//!
//! It is keyed on the claimed **surface** a user restores from, applies through, or exports
//! a history from (editor rename / refactor, replace-in-files, import / migration, repair
//! transaction, generated-artifact regeneration, AI review / apply, the recovery / local
//! history console, and support / export), not on component family or primitive lane. Each
//! [`HistorySurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, CLI/export, degraded-state, and mutation / recovery provenance —
//! and either passes (green), auto-narrows its history support claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a
//! full-truth claim inherited from a healthier mutation / recovery lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `RestorableCheckpoint` / `ReviewableHistory` claim while one of
//! its truth axes is not current — capture was metadata-only, the restore is only a partial
//! or manually chosen scope, the write / restore scope drifted, a checkpoint is unavailable
//! or expired, or a generated / managed-file caveat is unstated — is over-claiming and
//! blocks; a surface that discloses the reduction by narrowing its support claim (with a
//! bound reason and a frozen downgrade trigger) is honestly yellow. Restoring never erases
//! history: a narrowed restore always adds a new checkpoint rather than rewriting the
//! timeline. The always-on CLI/export axis must always stay certified, so support and
//! automation can reconstruct the same snapshot origin / actor / file identity / branch /
//! drift / restore-granularity / write-scope / retention-and-redaction truth from the same
//! history identity the user saw.
//!
//! Every row cites exactly one canonical local-history / write-scope component proof bundle
//! ([`HISTORY_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof —
//! rather than cloning per-surface evidence. The packet is metadata-only: raw file bodies,
//! snapshot contents, diffs, and credentials never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-local-history-write-scope-component-certification.schema.json`](../../../../schemas/ui/m5-local-history-write-scope-component-certification.schema.json).
//! The contract doc is
//! [`docs/recovery/m5_local_history_write_scope_component_certification_contract.md`](../../../../docs/recovery/m5_local_history_write_scope_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_rename_refactor_replace_import_repair_generated_artifact_and_ai_review_consumers_so_local_history_and_write_scope_components_keep_checkpoint_rollback_language_aligned_across_claimed_m5_mutation_surfaces as consumers;
use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_capture_is_metadata_only_restore_is_partial_or_manual_scope_is_stale_or_checkpoints_are_unavailable_across_claimed_m5_recovery_components as a11y;
use a11y::M5HistorySupportClaim;
use matrix::{M5HistoryDowngradeTrigger, M5LocalHistoryWriteScopeComponentFamily};

/// Schema version stamped on the M05-899 certification packet.
pub const HISTORY_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`HistorySurfaceCertificationPacket`].
pub const HISTORY_CERT_RECORD_KIND: &str =
    "m5_local_history_write_scope_component_certification_packet";

/// Stable record-kind tag carried by each [`HistorySurfaceCertificationRow`].
pub const HISTORY_CERT_ROW_RECORD_KIND: &str =
    "m5_local_history_write_scope_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const HISTORY_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const HISTORY_CERT_DOC_REF: &str =
    "docs/recovery/m5_local_history_write_scope_component_certification_contract.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix schema the
/// certified surfaces render.
pub const HISTORY_CERT_MATRIX_REF: &str = matrix::M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF;

/// The one canonical local-history / write-scope component proof bundle every certified
/// surface cites as its first-resolved component truth. All eight surfaces point back to it
/// rather than cloning per-surface evidence.
pub const HISTORY_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_ARTIFACT_REF;

/// The M05-897 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const HISTORY_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-898 accessibility / auto-narrowing support export whose keyboard / screen-reader
/// / CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on
/// every row.
pub const HISTORY_CERT_A11Y_BUNDLE_REF: &str = a11y::HISTORY_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const HISTORY_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const HISTORY_CERT_CSV_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const HISTORY_CERT_REPORT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-certification/report.md";

/// The eight claimed M5 mutation and recovery surfaces this capstone certifies. Keyed on the
/// surface a user actually restores from, applies broad changes through, or exports a
/// history from, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryWriteScopeCertifiedSurface {
    /// The editor rename / refactor apply surface.
    EditorRenameRefactor,
    /// The replace-in-files multi-file apply surface.
    ReplaceInFiles,
    /// The import / migration session surface.
    ImportMigration,
    /// The repair-transaction recovery surface.
    RepairTransaction,
    /// The generated-artifact regeneration surface.
    GeneratedArtifact,
    /// The AI review / apply surface.
    AiReviewApply,
    /// The recovery / local-history console surface.
    RecoveryConsole,
    /// The support / export bundle surface.
    SupportExport,
}

impl M5LocalHistoryWriteScopeCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5LocalHistoryWriteScopeCertifiedSurface; 8] = [
        M5LocalHistoryWriteScopeCertifiedSurface::EditorRenameRefactor,
        M5LocalHistoryWriteScopeCertifiedSurface::ReplaceInFiles,
        M5LocalHistoryWriteScopeCertifiedSurface::ImportMigration,
        M5LocalHistoryWriteScopeCertifiedSurface::RepairTransaction,
        M5LocalHistoryWriteScopeCertifiedSurface::GeneratedArtifact,
        M5LocalHistoryWriteScopeCertifiedSurface::AiReviewApply,
        M5LocalHistoryWriteScopeCertifiedSurface::RecoveryConsole,
        M5LocalHistoryWriteScopeCertifiedSurface::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRenameRefactor => "editor_rename_refactor",
            Self::ReplaceInFiles => "replace_in_files",
            Self::ImportMigration => "import_migration",
            Self::RepairTransaction => "repair_transaction",
            Self::GeneratedArtifact => "generated_artifact",
            Self::AiReviewApply => "ai_review_apply",
            Self::RecoveryConsole => "recovery_console",
            Self::SupportExport => "support_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, CLI/export,
/// degraded-state, and mutation / recovery provenance. The CLI/export axis is always-on and
/// must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryCertificationAxis {
    /// Visual parity: snapshot origin, actor lineage, file / object identity, branch /
    /// worktree context, external drift, generated / managed boundary, restore granularity,
    /// selectable apply scope, and retention / redaction posture are shown on the primary
    /// surface.
    Visual,
    /// Keyboard-reach parity: the same origin / actor / identity / scope / drift / restore /
    /// export truth and its controls are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as
    /// text / JSON / Markdown for support and automation, from the same history identity.
    CliExport,
    /// Degraded-state parity: a metadata-only capture, an unavailable / expired checkpoint,
    /// or a stale scope honestly downgrades a `RestorableCheckpoint` / `ReviewableHistory`
    /// claim to a weaker support tier.
    DegradedState,
    /// Mutation / recovery provenance parity: snapshot origin, actor lineage, file / object
    /// identity, branch / worktree context, external drift, generated / managed boundary,
    /// restore granularity, selectable apply scope, and retention / redaction posture stay
    /// explicit before any restore or multi-file apply commits — never inheriting a
    /// healthier lane's history truth, never masking a metadata-only capture, partial /
    /// manual restore, drifted scope, unavailable checkpoint, or generated / managed caveat
    /// as a full restorable checkpoint, and never erasing history on restore.
    MutationAndRecoveryProvenance,
}

impl HistoryCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [HistoryCertificationAxis; 6] = [
        HistoryCertificationAxis::Visual,
        HistoryCertificationAxis::Keyboard,
        HistoryCertificationAxis::ScreenReader,
        HistoryCertificationAxis::CliExport,
        HistoryCertificationAxis::DegradedState,
        HistoryCertificationAxis::MutationAndRecoveryProvenance,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::MutationAndRecoveryProvenance => "mutation_and_recovery_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim
    /// inherited from a healthier surface.
    UndisclosedDrift,
}

impl HistoryAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the author —
/// always recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistorySurfaceClaimStatus {
    /// Full standing: every axis certified, claimed support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, or the
    /// narrowing is inconsistent.
    Red,
}

impl HistorySurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow); red
    /// surfaces block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis certifies
/// only when this offers text / JSON / Markdown reconstruction and prohibits a
/// screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The origin / actor / identity / scope / drift / restore / retention fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl HistoryCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a screenshot-only
    /// export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: HistoryCertificationAxis,
    /// The certification state of the axis.
    pub state: HistoryAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5HistoryDowngradeTrigger>,
}

impl HistoryAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible
    ///   trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            HistoryAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            HistoryAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            HistoryAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current. Present
/// iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: HistoryCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5HistorySupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5HistorySupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed restore / apply still preserves history rather than erasing
    /// the timeline (a narrowed restore adds a new checkpoint, never rewrites).
    pub preserves_history_integrity: bool,
}

/// One certified M5 mutation / recovery surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurfaceCertificationRow {
    /// Record kind; must equal [`HISTORY_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`HISTORY_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5LocalHistoryWriteScopeCertifiedSurface,
    /// The history support-claim ceiling the surface asserts.
    pub claimed_claim: M5HistorySupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5HistorySupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5LocalHistoryWriteScopeComponentFamily>,
    /// One outcome per [`HistoryCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<HistoryAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<HistoryClaimAutoNarrow>,
    /// True when this surface never erases history on restore (a narrowed restore adds a new
    /// checkpoint rather than rewriting the timeline).
    pub history_preserved: bool,
    /// The one canonical local-history / write-scope proof bundle this surface cites. Must
    /// equal [`HISTORY_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: HistorySurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: HistoryCertExportParity,
    /// The compatibility notes captured for this surface.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl HistorySurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: HistoryCertificationAxis) -> Option<&HistoryAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<HistoryCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && HistoryCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(HistoryAxisOutcome::well_formed)
    }

    /// True when the surface narrows its support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<HistoryCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == HistoryAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed restore preserves history rather than erasing the timeline. A
    /// non-narrowed surface trivially preserves history; a narrowed one must say so.
    pub fn preserves_history_integrity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.history_preserved && narrow.preserves_history_integrity,
            None => self.history_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of
    /// the capstone: a degraded axis must produce a visible claim narrowing, CLI/export
    /// parity must always certify, restore must never erase history, and the narrowing must
    /// be consistent.
    pub fn derive_status(&self) -> HistorySurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != HISTORY_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_history_integrity()
        {
            return HistorySurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return HistorySurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(HistoryCertificationAxis::CliExport) {
            Some(o) if o.state == HistoryAxisCertificationState::Certified => {}
            _ => return HistorySurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == HistoryAxisCertificationState::UndisclosedDrift)
        {
            return HistorySurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return HistorySurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return HistorySurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_history_integrity
                {
                    return HistorySurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return HistorySurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return HistorySurfaceClaimStatus::Red;
        }

        HistorySurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == HISTORY_CERT_ROW_RECORD_KIND
            && self.schema_version == HISTORY_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed} history_preserved={preserved}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.history_preserved,
        )
    }
}

/// Rolled-up summary of an M05-899 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurfaceCertificationSummary {
    pub row_count: usize,
    pub surface_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_surfaces_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub every_axis_covered_on_every_row: bool,
    pub all_history_preserved: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`HistorySurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistorySurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<HistorySurfaceCertificationRow>,
}

/// Checked-in M05-899 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<HistorySurfaceCertificationRow>,
    pub summary: HistorySurfaceCertificationSummary,
}

impl HistorySurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: HistorySurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: HISTORY_CERT_SCHEMA_VERSION,
            record_kind: HISTORY_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: HistorySurfaceCertificationSummary {
                row_count: 0,
                surface_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_surfaces_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                every_axis_covered_on_every_row: false,
                all_history_preserved: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5LocalHistoryWriteScopeCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5LocalHistoryWriteScopeComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5LocalHistoryWriteScopeCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof
    /// the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5LocalHistoryWriteScopeComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(HistoryCertificationAxis::CliExport)
                .is_some_and(|o| o.state == HistoryAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> HistorySurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == HistorySurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == HistorySurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == HistorySurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(HistorySurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(HistorySurfaceCertificationRow::preserves_history_integrity);

        HistorySurfaceCertificationSummary {
            row_count: self.rows.len(),
            surface_count: surfaces.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_surfaces_present: all_surfaces,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == HISTORY_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(HistorySurfaceCertificationRow::covers_all_axes),
            all_history_preserved: all_preserved,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<HistoryCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != HISTORY_CERT_SCHEMA_VERSION {
            violations.push(HistoryCertificationViolation::SchemaVersion {
                expected: HISTORY_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != HISTORY_CERT_RECORD_KIND {
            violations.push(HistoryCertificationViolation::RecordKind {
                expected: HISTORY_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(HistoryCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != HISTORY_CERT_CANONICAL_BUNDLE_REF {
            violations.push(HistoryCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(HistoryCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(HistoryCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(HistoryCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(HistoryCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != HISTORY_CERT_CANONICAL_BUNDLE_REF {
                violations.push(HistoryCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(HistoryCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(HistoryCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Restore must never erase history.
            if !row.preserves_history_integrity() {
                violations.push(HistoryCertificationViolation::HistoryErased {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(HistoryCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(HistoryCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == HistorySurfaceClaimStatus::Red {
                violations.push(HistoryCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(HistoryCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(HistoryCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(HistoryCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(HistoryCertificationViolation::RawHistoryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,history_preserved\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding},{preserved}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
                preserved = row.history_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Local-History / Write-Scope Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5LocalHistoryWriteScopeCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- History preserved on every surface: {}\n",
            self.summary.all_history_preserved
        ));
        out.push_str(&format!(
            "- Auto-narrowed surfaces: {}\n",
            self.summary.narrowed_surface_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_local_history_write_scope_component_certification_export(
) -> Result<HistorySurfaceCertificationPacket, HistoryCertificationArtifactError> {
    let packet: HistorySurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-local-history-write-scope-component-certification/support_export.json"
    )))
    .map_err(HistoryCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HistoryCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum HistoryCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<HistoryCertificationViolation>),
}

impl fmt::Display for HistoryCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for HistoryCertificationArtifactError {}

/// Validation failure for M05-899 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    ExportParityNotCertified { id: String },
    HistoryErased { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawHistoryMaterialInExport,
}

impl fmt::Display for HistoryCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical local-history / write-scope proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical local-history / write-scope proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::HistoryErased { id } => {
                write!(
                    f,
                    "row {id} does not preserve history on restore (a narrowed restore must add a new checkpoint, never rewrite the timeline)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::SurfaceBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a full claim, \
CLI/export parity dropped, history was erased, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 mutation / recovery surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen local-history / write-scope component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawHistoryMaterialInExport => {
                write!(f, "export contains raw history material")
            }
        }
    }
}

impl Error for HistoryCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&HistoryAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != HistoryAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "stale"
            | "cached"
            | "unverified"
            | "offline"
            | "blocked"
            | "paused"
            | "interrupted"
            | "incomplete"
            | "metadata only"
            | "metadata_only"
            | "partial"
            | "manual"
            | "expired"
            | "no history"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-899 certification packet. Certifies all eight
/// claimed M5 mutation / recovery surfaces: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker support ceiling (yellow). No surface
/// hides drift (red), and no surface erases history on restore.
pub fn seeded_m5_local_history_write_scope_component_certification_packet(
) -> HistorySurfaceCertificationPacket {
    HistorySurfaceCertificationPacket::new(HistorySurfaceCertificationPacketInput {
        packet_id: "m5-local-history-write-scope-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: HISTORY_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: HISTORY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:local-history-write-scope-certification:{id}"),
        HISTORY_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        HISTORY_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> HistoryCertExportParity {
    HistoryCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: HistoryCertificationAxis) -> &'static str {
    match axis {
        HistoryCertificationAxis::Visual => {
            "snapshot origin, actor lineage, file/object identity, branch/worktree context, external drift, generated/managed boundary, restore granularity, selectable apply scope, and retention/redaction posture shown on-surface"
        }
        HistoryCertificationAxis::Keyboard => {
            "the same origin/actor/identity/scope/drift/restore/export truth and its controls are keyboard-reachable"
        }
        HistoryCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        HistoryCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support from the same history identity"
        }
        HistoryCertificationAxis::DegradedState => {
            "a metadata-only capture, an unavailable/expired checkpoint, or a stale scope honestly downgrades the RestorableCheckpoint/ReviewableHistory claim"
        }
        HistoryCertificationAxis::MutationAndRecoveryProvenance => {
            "snapshot origin, actor lineage, file/object identity, branch/worktree context, external drift, generated/managed boundary, restore granularity, selectable apply scope, and retention/redaction posture stay explicit before any restore or multi-file apply; restore never erases history"
        }
    }
}

fn seed_certified(axis: HistoryCertificationAxis) -> HistoryAxisOutcome {
    HistoryAxisOutcome {
        axis,
        state: HistoryAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: HistoryCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5HistoryDowngradeTrigger,
) -> HistoryAxisOutcome {
    HistoryAxisOutcome {
        axis,
        state: HistoryAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<HistoryAxisOutcome> {
    HistoryCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: HistoryCertificationAxis,
    outcome: HistoryAxisOutcome,
) -> Vec<HistoryAxisOutcome> {
    HistoryCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    surface: M5LocalHistoryWriteScopeCertifiedSurface,
    claimed_claim: M5HistorySupportClaim,
    certified_claim: M5HistorySupportClaim,
    consumed_families: &[M5LocalHistoryWriteScopeComponentFamily],
    axis_outcomes: Vec<HistoryAxisOutcome>,
    claim_auto_narrow: Option<HistoryClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> HistorySurfaceCertificationRow {
    let mut row = HistorySurfaceCertificationRow {
        record_kind: HISTORY_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: HISTORY_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        history_preserved: true,
        canonical_bundle_ref: HISTORY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: HistorySurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            HISTORY_CERT_MATRIX_REF.to_owned(),
            HISTORY_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: HistoryCertificationAxis,
    from_claim: M5HistorySupportClaim,
    to_claim: M5HistorySupportClaim,
    label: &str,
) -> HistoryClaimAutoNarrow {
    HistoryClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_history_integrity: true,
    }
}

fn seeded_rows() -> Vec<HistorySurfaceCertificationRow> {
    use HistoryCertificationAxis as Ax;
    use M5HistoryDowngradeTrigger as Trig;
    use M5HistorySupportClaim::*;
    use M5LocalHistoryWriteScopeCertifiedSurface as S;
    use M5LocalHistoryWriteScopeComponentFamily::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:editor-rename-refactor",
            S::EditorRenameRefactor,
            RestorableCheckpoint,
            RestorableCheckpoint,
            &[WriteScopePreviewTree, LocalHistoryRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "write_scope"],
            &[
                "the write-scope preview tree names every file the rename/refactor will touch before the apply commits",
                "the local-history row keeps its snapshot origin, actor lineage, and capture fidelity explicit",
                "keyboard/screen-reader reach preserved for the scope tree and the local-history row controls",
                "provenance: a rename/refactor apply never leaves its write scope or the actor who made the checkpoint implicit",
            ],
        ),
        seed_row(
            "cert:recovery-console",
            S::RecoveryConsole,
            RestorableCheckpoint,
            RestorableCheckpoint,
            &[CheckpointGroupCard, RestorePreviewCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "restore_granularity"],
            &[
                "the checkpoint-group card keeps its checkpoint lineage and mutation class explicit before a restore",
                "the restore-preview card keeps external drift and the restore granularity visible before the restore commits",
                "restore adds a new checkpoint rather than erasing the timeline (no history erasure)",
                "provenance: a whole-snapshot restore never hides that a new checkpoint is written over the old timeline",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableHistory,
            ReviewableHistory,
            &[RetentionExportCard, HistoryExportManifest],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "retention_and_redaction"],
            &[
                "support export reconstructs origin/actor/identity/scope/drift/restore/retention truth from the same history identity",
                "the retention/export card keeps its retention posture and export redaction explicit with no hidden sharing",
                "the history-export manifest keeps its export class and lineage explicit; credentials are scrubbed before export",
                "provenance: a support packet never exports raw file bodies, snapshot contents, or diffs",
            ],
        ),
        seed_row(
            "cert:replace-in-files",
            S::ReplaceInFiles,
            RestorableCheckpoint,
            RestorableCheckpoint,
            &[WriteScopePreviewTree, RestoreGranularitySelector],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "selectable_scope"],
            &[
                "the write-scope preview tree keeps the honest file-count and workspace-root grouping before the multi-file apply",
                "the restore-granularity selector keeps the selectable apply scope explicit rather than implying a whole-tree apply",
                "keyboard/screen-reader reach preserved for the scope tree and the granularity selector",
                "provenance: a replace-in-files apply never understates the write scope or the selectable apply granularity",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:import-migration",
            S::ImportMigration,
            RestorableCheckpoint,
            StaleScopeHistory,
            &[WriteScopePreviewTree, RestoreGranularitySelector],
            seed_certified_except(
                Ax::MutationAndRecoveryProvenance,
                seed_narrowed(
                    Ax::MutationAndRecoveryProvenance,
                    "the import/migration write scope has drifted since capture and must be re-resolved",
                    "The import/migration session's write scope drifted against the working tree since the snapshot was captured, so the RestorableCheckpoint claim narrows to stale-scope instead of applying a scope that no longer matches disk",
                    Trig::WriteScopeUnderstated,
                ),
            ),
            Some(seed_narrow(
                Ax::MutationAndRecoveryProvenance,
                RestorableCheckpoint,
                StaleScopeHistory,
                "Drifted import scope: the migration write scope no longer matches the working tree; the scope tree shows it must be re-resolved rather than applied as captured",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the write-scope preview tree keeps the drifted-scope reason explicit and offers re-resolution",
                "the restore-granularity selector keeps the selectable scope explicit as stale rather than ready",
                "mutation/recovery: RestorableCheckpoint narrows to stale-scope (auto-narrowed)",
                "known compatibility note: scope-drift behavior — a drifted import scope never reads as apply-ready",
            ],
        ),
        seed_row(
            "cert:generated-artifact",
            S::GeneratedArtifact,
            RestorableCheckpoint,
            NarrowedRestore,
            &[RestorePreviewCard, WriteScopePreviewTree],
            seed_certified_except(
                Ax::MutationAndRecoveryProvenance,
                seed_narrowed(
                    Ax::MutationAndRecoveryProvenance,
                    "generated/managed files are excluded, so only a partial scope restores",
                    "The generated-artifact restore excludes generated and managed files that are owned by their producers, so the RestorableCheckpoint claim narrows to narrowed-restore instead of implying the whole snapshot restores",
                    Trig::RestoreGranularityCollapsed,
                ),
            ),
            Some(seed_narrow(
                Ax::MutationAndRecoveryProvenance,
                RestorableCheckpoint,
                NarrowedRestore,
                "Partial restore: generated/managed files stay owned by their producers; the restore-preview card shows the excluded scope rather than a whole-snapshot restore",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the restore-preview card keeps the generated/managed-file caveat and the partial restore scope explicit",
                "the write-scope preview tree keeps the excluded generated/managed nodes visible rather than hiding them",
                "mutation/recovery: RestorableCheckpoint narrows to narrowed-restore (auto-narrowed)",
                "known compatibility note: partial/manual restore — a partial generated-artifact restore never reads as whole-snapshot",
            ],
        ),
        seed_row(
            "cert:repair-transaction",
            S::RepairTransaction,
            RestorableCheckpoint,
            MetadataOnlyHistory,
            &[LocalHistoryRow, CheckpointGroupCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "only metadata was captured for the repair transaction; no full body is available",
                    "The repair transaction captured only metadata (no full file body), so the RestorableCheckpoint claim narrows to metadata-only instead of implying the file contents can be reconstructed",
                    Trig::CaptureFidelityMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                RestorableCheckpoint,
                MetadataOnlyHistory,
                "Capture was metadata-only: the repair transaction recorded attribution but not the file body; the local-history row shows metadata-only rather than a restorable body",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the local-history row keeps the metadata-only capture fidelity explicit rather than implying a full body",
                "the checkpoint-group card keeps its lineage explicit while marking the capture as metadata-only",
                "degraded-state: RestorableCheckpoint narrows to metadata-only (auto-narrowed)",
                "known compatibility note: metadata-only capture — a metadata-only repair record never reads as a restorable body",
            ],
        ),
        seed_row(
            "cert:ai-review-apply",
            S::AiReviewApply,
            RestorableCheckpoint,
            UnavailableCheckpoint,
            &[CheckpointGroupCard, HistoryExportManifest],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the AI-review checkpoint has expired and is no longer available to restore",
                    "The AI review/apply checkpoint has expired under retention and is no longer available, so the RestorableCheckpoint claim narrows to unavailable-checkpoint instead of offering a restore that cannot proceed",
                    Trig::CheckpointLineageUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                RestorableCheckpoint,
                UnavailableCheckpoint,
                "Checkpoint no longer available: the AI-review checkpoint expired under retention; the checkpoint-group card shows it as unavailable rather than offering a dead restore",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the checkpoint-group card keeps the expired/unavailable checkpoint explicit rather than offering a dead restore",
                "the history-export manifest keeps its lineage explicit while marking the checkpoint as unavailable",
                "degraded-state: RestorableCheckpoint narrows to unavailable-checkpoint (auto-narrowed)",
                "known compatibility note: unavailable checkpoint — an expired AI-review checkpoint never reads as restorable",
            ],
        ),
    ]
}
