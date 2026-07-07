//! M05-923 surface certification over the frozen M5 provider-account / project-or-board-mapping
//! / sync-behavior / offline-capture / privacy-redaction component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`])
//! defines the five reusable provider-account-row, project-or-board-mapping-row,
//! sync-behavior-row, offline-capture-row, and privacy-redaction-row components, the
//! M05-917..920 primitive lanes narrow each one, the M05-921 consumer lane
//! ([`crate::add_shared_work_item_status_transition_help_support_and_export_consumers_so_provider_account_and_offline_capture_components_keep_account_mapping_sync_and_redaction_language_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed work-item / status-transition / issue-intake /
//! help / support-export / browser-handoff consumers, and the M05-922 accessibility /
//! auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_scope_is_limited_mappings_are_blocked_or_offline_capture_packets_remain_local_only_across_claimed_m5_provider_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared provider-boundary component truth holds on every claimed M5
//! provider-backed team-workflow surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user reads, drafts, publishes, replays, or exports
//! provider-backed work-item data on (the work-item detail, the status-transition review, issue
//! intake, Help / docs, support / export, the browser handoff, the provider settings drill, and
//! the CLI), not on component family or primitive lane. Each
//! [`ProviderSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, CLI/export, degraded-state, and provider-boundary provenance — and
//! either passes (green), auto-narrows its provider claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier provider lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `ProviderCommitted` / `ReviewableProjection` claim while one of its
//! truth axes is not current — the effective write scope is limited, the session is stale, the
//! project/board mapping is policy-blocked, the offline-capture packet is local-only, or the
//! connection / mapping / sync / redaction boundary is unstated — is over-claiming and blocks; a
//! surface that discloses the reduction by narrowing its provider claim (with a bound reason and
//! a frozen downgrade trigger) is honestly yellow. Provider truth never loses lineage: a narrowed
//! surface always preserves its account / mapping / queued-draft / redaction lineage continuity
//! rather than dropping it between a cached read, a local draft, and a committed publish. The
//! always-on CLI/export axis must always stay certified, so support and automation can
//! reconstruct the same connection-state / tenant-scope / write-scope / default-destination /
//! sync-mode / queued-draft / redaction-boundary truth from the same provider identity the user
//! saw.
//!
//! Every row cites exactly one canonical provider-account / offline-capture component proof
//! bundle ([`PROVIDER_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof —
//! rather than cloning per-surface evidence. The packet is metadata-only: raw provider payloads,
//! captured draft bodies, redacted field contents, and credentials never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-provider-account-offline-capture-component-certification.schema.json`](../../../../schemas/ui/m5-provider-account-offline-capture-component-certification.schema.json).
//! The contract doc is
//! [`docs/providers/m5_provider_account_offline_capture_component_certification_contract.md`](../../../../docs/providers/m5_provider_account_offline_capture_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_work_item_status_transition_help_support_and_export_consumers_so_provider_account_and_offline_capture_components_keep_account_mapping_sync_and_redaction_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_scope_is_limited_mappings_are_blocked_or_offline_capture_packets_remain_local_only_across_claimed_m5_provider_components as a11y;
use a11y::M5ProviderComponentClaim;
use matrix::{M5ProviderAccountOfflineComponentFamily, M5ProviderDowngradeTrigger};

/// Schema version stamped on the M05-923 certification packet.
pub const PROVIDER_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ProviderSurfaceCertificationPacket`].
pub const PROVIDER_CERT_RECORD_KIND: &str =
    "m5_provider_account_offline_capture_component_certification_packet";

/// Stable record-kind tag carried by each [`ProviderSurfaceCertificationRow`].
pub const PROVIDER_CERT_ROW_RECORD_KIND: &str =
    "m5_provider_account_offline_capture_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const PROVIDER_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const PROVIDER_CERT_DOC_REF: &str =
    "docs/providers/m5_provider_account_offline_capture_component_certification_contract.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix schema
/// the certified surfaces render.
pub const PROVIDER_CERT_MATRIX_REF: &str =
    matrix::M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF;

/// The one canonical provider-account / offline-capture component proof bundle every certified
/// surface cites as its first-resolved component truth. All eight surfaces point back to it
/// rather than cloning per-surface evidence.
pub const PROVIDER_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_ARTIFACT_REF;

/// The M05-921 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const PROVIDER_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_PROVIDER_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-922 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on every
/// row.
pub const PROVIDER_CERT_A11Y_BUNDLE_REF: &str = a11y::PROVIDER_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const PROVIDER_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const PROVIDER_CERT_CSV_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const PROVIDER_CERT_REPORT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-certification/report.md";

/// The eight claimed M5 provider-backed team-workflow surfaces this capstone certifies. Keyed on
/// the surface a user actually reads, drafts, publishes, replays, or exports provider-backed
/// work-item data on, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderBackedTeamWorkflowCertifiedSurface {
    /// The work-item detail surface.
    WorkItemDetail,
    /// The status-transition review sheet surface.
    StatusTransitionReview,
    /// The issue-intake surface.
    IssueIntake,
    /// The Help / docs surface.
    DocsHelp,
    /// The support / export bundle surface.
    SupportExport,
    /// The browser / device-code handoff surface.
    BrowserHandoff,
    /// The provider-settings / account drill surface.
    ProviderSettings,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5ProviderBackedTeamWorkflowCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5ProviderBackedTeamWorkflowCertifiedSurface; 8] = [
        M5ProviderBackedTeamWorkflowCertifiedSurface::WorkItemDetail,
        M5ProviderBackedTeamWorkflowCertifiedSurface::StatusTransitionReview,
        M5ProviderBackedTeamWorkflowCertifiedSurface::IssueIntake,
        M5ProviderBackedTeamWorkflowCertifiedSurface::DocsHelp,
        M5ProviderBackedTeamWorkflowCertifiedSurface::SupportExport,
        M5ProviderBackedTeamWorkflowCertifiedSurface::BrowserHandoff,
        M5ProviderBackedTeamWorkflowCertifiedSurface::ProviderSettings,
        M5ProviderBackedTeamWorkflowCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItemDetail => "work_item_detail",
            Self::StatusTransitionReview => "status_transition_review",
            Self::IssueIntake => "issue_intake",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::BrowserHandoff => "browser_handoff",
            Self::ProviderSettings => "provider_settings",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions
/// the spec requires verifying — visual, keyboard, screen-reader, CLI/export, degraded-state, and
/// provider-boundary provenance. The CLI/export axis is always-on and must stay certified for
/// every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCertificationAxis {
    /// Visual parity: connection state, tenant scope, effective write scope, default-destination
    /// mapping, sync mode, queued-draft state, and redaction / export boundary are shown on the
    /// primary surface.
    Visual,
    /// Keyboard-reach parity: the same account / mapping / sync / offline-capture / redaction
    /// truth and its controls are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or
    /// a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as text /
    /// JSON / Markdown for support and automation, from the same provider identity.
    CliExport,
    /// Degraded-state parity: a limited effective write scope, a stale session, a policy-blocked
    /// mapping, or a local-only offline-capture packet honestly downgrades a `ProviderCommitted`
    /// / `ReviewableProjection` claim to a weaker provider tier.
    DegradedState,
    /// Provider-boundary provenance parity: connection state, tenant scope, effective write
    /// scope, default-destination mapping, sync mode, queued-draft state, and redaction / export
    /// boundary stay explicit before any read, draft, publish, replay, or export — never
    /// inheriting a healthier lane's provider truth, never masking a limited scope, stale
    /// session, policy-blocked mapping, local-only capture packet, or unstated boundary as a
    /// committed provider surface, and never dropping account / mapping / queued-draft /
    /// redaction lineage between a cached read, a local draft, and a committed publish.
    ProviderBoundaryProvenance,
}

impl ProviderCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ProviderCertificationAxis; 6] = [
        ProviderCertificationAxis::Visual,
        ProviderCertificationAxis::Keyboard,
        ProviderCertificationAxis::ScreenReader,
        ProviderCertificationAxis::CliExport,
        ProviderCertificationAxis::DegradedState,
        ProviderCertificationAxis::ProviderBoundaryProvenance,
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
            Self::ProviderBoundaryProvenance => "provider_boundary_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited
    /// from a healthier surface.
    UndisclosedDrift,
}

impl ProviderAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the author — always
/// recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed provider tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, lineage is
    /// dropped, or the narrowing is inconsistent.
    Red,
}

impl ProviderSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow); red surfaces
    /// block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis certifies only
/// when this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The connection / scope / mapping / sync / offline-capture / redaction fields the surface
    /// preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl ProviderCertExportParity {
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
pub struct ProviderAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ProviderCertificationAxis,
    /// The certification state of the axis.
    pub state: ProviderAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ProviderDowngradeTrigger>,
}

impl ProviderAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger
    ///   (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            ProviderAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ProviderAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ProviderAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current. Present iff
/// the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ProviderCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5ProviderComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5ProviderComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its account / mapping / queued-draft /
    /// redaction lineage continuity rather than dropping it between a cached read, a local draft,
    /// and a committed publish.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 provider-backed team-workflow surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSurfaceCertificationRow {
    /// Record kind; must equal [`PROVIDER_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PROVIDER_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5ProviderBackedTeamWorkflowCertifiedSurface,
    /// The provider-claim ceiling the surface asserts.
    pub claimed_claim: M5ProviderComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5ProviderComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ProviderAccountOfflineComponentFamily>,
    /// One outcome per [`ProviderCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ProviderAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ProviderClaimAutoNarrow>,
    /// True when this surface never drops its account / mapping / queued-draft / redaction lineage
    /// continuity between a cached read, a local draft, and a committed publish.
    pub lineage_preserved: bool,
    /// The one canonical provider-account / offline-capture proof bundle this surface cites. Must
    /// equal [`PROVIDER_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ProviderSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: ProviderCertExportParity,
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

impl ProviderSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ProviderCertificationAxis) -> Option<&ProviderAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ProviderCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ProviderCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ProviderAxisOutcome::well_formed)
    }

    /// True when the surface narrows its provider claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ProviderCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ProviderAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its account / mapping / queued-draft / redaction
    /// lineage continuity rather than dropping it. A non-narrowed surface trivially preserves
    /// lineage; a narrowed one must say so.
    pub fn preserves_lineage_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.lineage_preserved && narrow.preserves_lineage_continuity,
            None => self.lineage_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, CLI/export parity must
    /// always certify, provider truth must never drop lineage, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> ProviderSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != PROVIDER_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
        {
            return ProviderSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ProviderSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ProviderCertificationAxis::CliExport) {
            Some(o) if o.state == ProviderAxisCertificationState::Certified => {}
            _ => return ProviderSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ProviderAxisCertificationState::UndisclosedDrift)
        {
            return ProviderSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ProviderSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ProviderSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return ProviderSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ProviderSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return ProviderSurfaceClaimStatus::Red;
        }

        ProviderSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == PROVIDER_CERT_ROW_RECORD_KIND
            && self.schema_version == PROVIDER_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed} lineage_preserved={preserved}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.lineage_preserved,
        )
    }
}

/// Rolled-up summary of an M05-923 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSurfaceCertificationSummary {
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
    pub all_lineage_preserved: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`ProviderSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ProviderSurfaceCertificationRow>,
}

/// Checked-in M05-923 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ProviderSurfaceCertificationRow>,
    pub summary: ProviderSurfaceCertificationSummary,
}

impl ProviderSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ProviderSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: PROVIDER_CERT_SCHEMA_VERSION,
            record_kind: PROVIDER_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ProviderSurfaceCertificationSummary {
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
                all_lineage_preserved: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5ProviderBackedTeamWorkflowCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ProviderAccountOfflineComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5ProviderBackedTeamWorkflowCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ProviderAccountOfflineComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ProviderCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ProviderAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ProviderSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ProviderSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ProviderSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ProviderSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ProviderSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(ProviderSurfaceCertificationRow::preserves_lineage_continuity);

        ProviderSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == PROVIDER_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ProviderSurfaceCertificationRow::covers_all_axes),
            all_lineage_preserved: all_preserved,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ProviderCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROVIDER_CERT_SCHEMA_VERSION {
            violations.push(ProviderCertificationViolation::SchemaVersion {
                expected: PROVIDER_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PROVIDER_CERT_RECORD_KIND {
            violations.push(ProviderCertificationViolation::RecordKind {
                expected: PROVIDER_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ProviderCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != PROVIDER_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ProviderCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ProviderCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ProviderCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(ProviderCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ProviderCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != PROVIDER_CERT_CANONICAL_BUNDLE_REF {
                violations.push(ProviderCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ProviderCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(ProviderCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Provider truth must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(ProviderCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(ProviderCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ProviderCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == ProviderSurfaceClaimStatus::Red {
                violations.push(ProviderCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(ProviderCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(ProviderCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ProviderCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ProviderCertificationViolation::RawProviderMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,lineage_preserved\n",
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
                preserved = row.lineage_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Provider-Account / Offline-Capture Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5ProviderBackedTeamWorkflowCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Lineage preserved on every surface: {}\n",
            self.summary.all_lineage_preserved
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
pub fn current_m5_provider_account_offline_capture_component_certification_export(
) -> Result<ProviderSurfaceCertificationPacket, ProviderCertificationArtifactError> {
    let packet: ProviderSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-account-offline-capture-component-certification/support_export.json"
    )))
    .map_err(ProviderCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ProviderCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ProviderCertificationViolation>),
}

impl fmt::Display for ProviderCertificationArtifactError {
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

impl Error for ProviderCertificationArtifactError {}

/// Validation failure for M05-923 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderCertificationViolation {
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
    LineageDropped { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawProviderMaterialInExport,
}

impl fmt::Display for ProviderCertificationViolation {
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
                    "packet does not cite the canonical provider-account / offline-capture proof bundle"
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
                    "row {id} does not cite the one canonical provider-account / offline-capture proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} drops account / mapping / queued-draft / redaction lineage continuity (a narrowed surface must preserve its lineage between a cached read, a local draft, and a committed publish)"
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
CLI/export parity dropped, lineage was dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 provider-backed team-workflow surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen provider-account / offline-capture component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawProviderMaterialInExport => {
                write!(f, "export contains raw provider material")
            }
        }
    }
}

impl Error for ProviderCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ProviderAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ProviderAxisCertificationState::Certified,
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
            | "uncertain"
            | "limited scope"
            | "limited_scope"
            | "stale session"
            | "stale_session"
            | "policy blocked"
            | "policy_blocked"
            | "local only"
            | "local_only"
            | "unmapped"
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

/// Builds the canonical, checked-in M05-923 certification packet. Certifies all eight claimed M5
/// provider-backed team-workflow surfaces: four deliver their claim (green) and four auto-narrow
/// a not-current truth axis to a weaker provider ceiling (yellow). No surface hides drift (red),
/// and no surface drops account / mapping / queued-draft / redaction lineage.
pub fn seeded_m5_provider_account_offline_capture_component_certification_packet(
) -> ProviderSurfaceCertificationPacket {
    ProviderSurfaceCertificationPacket::new(ProviderSurfaceCertificationPacketInput {
        packet_id: "m5-provider-account-offline-capture-component-certification:stable:0001"
            .to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: PROVIDER_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: PROVIDER_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:provider-account-offline-capture-certification:{id}"),
        PROVIDER_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        PROVIDER_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ProviderCertExportParity {
    ProviderCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ProviderCertificationAxis) -> &'static str {
    match axis {
        ProviderCertificationAxis::Visual => {
            "connection state, tenant scope, effective write scope, default-destination mapping, sync mode, queued-draft state, and redaction/export boundary shown on-surface"
        }
        ProviderCertificationAxis::Keyboard => {
            "the same account/mapping/sync/offline-capture/redaction truth and its controls are keyboard-reachable"
        }
        ProviderCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        ProviderCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support and automation from the same provider identity"
        }
        ProviderCertificationAxis::DegradedState => {
            "a limited write scope, a stale session, a policy-blocked mapping, or a local-only capture packet honestly downgrades the ProviderCommitted/ReviewableProjection claim"
        }
        ProviderCertificationAxis::ProviderBoundaryProvenance => {
            "connection state, tenant scope, effective write scope, default-destination mapping, sync mode, queued-draft state, and redaction/export boundary stay explicit before any read, draft, publish, replay, or export; the boundary never drops account/mapping/queued-draft/redaction lineage"
        }
    }
}

fn seed_certified(axis: ProviderCertificationAxis) -> ProviderAxisOutcome {
    ProviderAxisOutcome {
        axis,
        state: ProviderAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ProviderCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ProviderDowngradeTrigger,
) -> ProviderAxisOutcome {
    ProviderAxisOutcome {
        axis,
        state: ProviderAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ProviderAxisOutcome> {
    ProviderCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ProviderCertificationAxis,
    outcome: ProviderAxisOutcome,
) -> Vec<ProviderAxisOutcome> {
    ProviderCertificationAxis::ALL
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
    surface: M5ProviderBackedTeamWorkflowCertifiedSurface,
    claimed_claim: M5ProviderComponentClaim,
    certified_claim: M5ProviderComponentClaim,
    consumed_families: &[M5ProviderAccountOfflineComponentFamily],
    axis_outcomes: Vec<ProviderAxisOutcome>,
    claim_auto_narrow: Option<ProviderClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ProviderSurfaceCertificationRow {
    let mut row = ProviderSurfaceCertificationRow {
        record_kind: PROVIDER_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        canonical_bundle_ref: PROVIDER_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ProviderSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            PROVIDER_CERT_MATRIX_REF.to_owned(),
            PROVIDER_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ProviderCertificationAxis,
    from_claim: M5ProviderComponentClaim,
    to_claim: M5ProviderComponentClaim,
    label: &str,
) -> ProviderClaimAutoNarrow {
    ProviderClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<ProviderSurfaceCertificationRow> {
    use M5ProviderAccountOfflineComponentFamily::*;
    use M5ProviderBackedTeamWorkflowCertifiedSurface as S;
    use M5ProviderComponentClaim::*;
    use M5ProviderDowngradeTrigger as Trig;
    use ProviderCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:work-item-detail",
            S::WorkItemDetail,
            ProviderCommitted,
            ProviderCommitted,
            &[ProviderAccountRow, SyncBehaviorRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "connection_state"],
            &[
                "the provider-account row keeps its connection state, tenant scope, and effective write scope explicit before the work item is drafted or published",
                "the sync-behavior row keeps its sync mode and queued-draft state explicit so a committed publish never masquerades as a local draft",
                "keyboard/screen-reader reach preserved for the provider-account row and the sync-behavior row",
                "provenance: a work-item detail never leaves whether Aureline can write right now or where a publish lands implicit",
            ],
        ),
        seed_row(
            "cert:status-transition-review",
            S::StatusTransitionReview,
            ProviderCommitted,
            ProviderCommitted,
            &[ProjectOrBoardMappingRow, SyncBehaviorRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "default_destination"],
            &[
                "the project/board mapping row keeps its default-destination target and mapping origin explicit before a status transition is committed",
                "the sync-behavior row keeps its sync mode and queued-draft state explicit while the transition is reviewed",
                "keyboard/screen-reader reach preserved for the mapping row and the sync-behavior row",
                "provenance: a status-transition review never leaves the committed destination or the queued-draft state implicit",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableProjection,
            ReviewableProjection,
            &[PrivacyRedactionRow, ProviderAccountRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "redaction_boundary"],
            &[
                "support export reconstructs connection-state/tenant-scope/write-scope/default-destination/sync-mode/queued-draft/redaction-boundary truth from the same provider identity",
                "the privacy/redaction row keeps its redaction class and metadata-safe export boundary explicit with no credential or endpoint leakage",
                "the provider-account row keeps its connection state explicit in the exported packet",
                "provenance: a provider export never carries raw provider payloads, captured draft bodies, or redacted field contents",
            ],
        ),
        seed_row(
            "cert:issue-intake",
            S::IssueIntake,
            ReviewableProjection,
            ReviewableProjection,
            &[ProjectOrBoardMappingRow, OfflineCaptureRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "queued_draft_state"],
            &[
                "the project/board mapping row keeps its default-destination target explicit before an intake draft is captured",
                "the offline-capture row keeps its offline-capture state and queued-draft count explicit so a publish-later draft never reads as published",
                "keyboard/screen-reader reach preserved for the mapping row and the offline-capture row",
                "provenance: an issue-intake surface never leaves what remains queued locally implicit",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:provider-settings",
            S::ProviderSettings,
            ProviderCommitted,
            LimitedScopeProjection,
            &[ProviderAccountRow],
            seed_certified_except(
                Ax::ProviderBoundaryProvenance,
                seed_narrowed(
                    Ax::ProviderBoundaryProvenance,
                    "the connected account's effective write scope is limited and cannot perform the full committed write",
                    "The provider-settings drill resolves an in-scope, session-fresh account whose effective write scope is limited, so the ProviderCommitted claim narrows to limited-scope-projection instead of implying Aureline can perform the full committed write",
                    Trig::WriteScopeUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ProviderBoundaryProvenance,
                ProviderCommitted,
                LimitedScopeProjection,
                "Write scope reduced: the connected account grants read / limited-write only; the provider-account row shows the full committed write is unavailable rather than implying it",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the provider-account row keeps the limited effective write scope explicit and offers a rescope entrypoint",
                "the provider-account row keeps its connection state and tenant scope explicit while the scope stays limited",
                "provider-boundary: ProviderCommitted narrows to limited-scope-projection (auto-narrowed)",
                "known compatibility note: limited-scope behavior — a limited-write account never reads as a committed write surface",
            ],
        ),
        seed_row(
            "cert:browser-handoff",
            S::BrowserHandoff,
            ProviderCommitted,
            StaleSessionProjection,
            &[ProviderAccountRow],
            seed_certified_except(
                Ax::ProviderBoundaryProvenance,
                seed_narrowed(
                    Ax::ProviderBoundaryProvenance,
                    "the account session is stale and only a cached read is available until re-authentication completes",
                    "The browser-handoff surface resolves a stale account session with only a cached read available, so the ProviderCommitted claim narrows to stale-session-projection instead of implying the session is live and ready to commit",
                    Trig::ConnectionStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ProviderBoundaryProvenance,
                ProviderCommitted,
                StaleSessionProjection,
                "Session stale: the connected account must re-authenticate; the provider-account row shows only a cached read is trustworthy rather than presenting a live commit path",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the provider-account row keeps the stale-session reason explicit and offers a re-authentication handoff without blind credential re-entry",
                "the provider-account row keeps its cached-read state explicit while the session stays stale",
                "provider-boundary: ProviderCommitted narrows to stale-session-projection (auto-narrowed)",
                "known compatibility note: stale-session behavior — a stale session never reads as a live committed surface",
            ],
        ),
        seed_row(
            "cert:docs-help",
            S::DocsHelp,
            ProviderCommitted,
            PolicyBlockedMapping,
            &[ProjectOrBoardMappingRow],
            seed_certified_except(
                Ax::ProviderBoundaryProvenance,
                seed_narrowed(
                    Ax::ProviderBoundaryProvenance,
                    "the project/board mapping is policy-blocked and no committed destination can resolve",
                    "The docs/help provider mapping is policy-blocked with no resolvable committed destination, so the ProviderCommitted claim narrows to policy-blocked-mapping instead of implying a publish can land on a mapped target",
                    Trig::MappingOriginUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ProviderBoundaryProvenance,
                ProviderCommitted,
                PolicyBlockedMapping,
                "Mapping policy-blocked: no committed destination resolves; the project/board mapping row explains why the mapping is blocked rather than assuming a default destination",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the project/board mapping row keeps the policy-blocked reason explicit rather than assuming a default destination",
                "the project/board mapping row keeps its unresolved-destination state explicit while the mapping stays blocked",
                "provider-boundary: ProviderCommitted narrows to policy-blocked-mapping (auto-narrowed)",
                "known compatibility note: policy-blocked mapping — a blocked mapping never reads as a committed destination",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ProviderCommitted,
            LocalOnlyPacket,
            &[OfflineCaptureRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the offline-capture packet remains local-only and nothing has been published to the provider",
                    "The CLI-headless capture resolves a local-only offline packet with nothing published to the provider, so the ProviderCommitted claim narrows to local-only-packet instead of implying the queued draft has been committed",
                    Trig::OfflineCaptureStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ProviderCommitted,
                LocalOnlyPacket,
                "Local-only packet: the captured draft stays queued on this device; the offline-capture row shows it is publish-later rather than presenting it as committed",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the offline-capture row keeps its local-only packet destination and queued-draft count explicit rather than implying a publish",
                "the offline-capture row keeps its publish-later state explicit while the packet stays local-only",
                "degraded-state: ProviderCommitted narrows to local-only-packet (auto-narrowed)",
                "known compatibility note: local-only packet — a queued offline capture never reads as committed to the provider",
            ],
        ),
    ]
}
