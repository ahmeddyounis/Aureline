//! M05-1003 surface certification over the frozen M5 companion component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_companion_component_matrix`]) defines the six reusable
//! notification-row, mobile-review-card, CI-status-card, session-follow-tile,
//! incident-snapshot-card, and desktop-handoff-sheet components, the M05-997..999 primitive lanes
//! narrow each one, the M05-1000 degraded-state lane governs their cached / offline / auth-blocked
//! / policy-blocked states, the M05-1001 consumer lane
//! ([`crate::add_shared_inbox_review_ci_session_follow_incident_advisory_and_browser_or_desktop_handoff_consumers_so_companion_components_keep_scope_freshness_and_desktop_required_language_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed inbox / review / CI / session-follow / incident /
//! advisory / help / support / handoff / export consumers, and the M05-1002 accessibility /
//! auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_share_export_parity_and_automatic_narrowing_when_object_freshness_companion_authority_tenant_scope_or_handoff_validity_is_stale_limited_or_revoked_across_claimed_m5_companion_components`])
//! certifies keyboard / screen-reader / share / export parity per family, this closing capstone
//! *certifies* that the shared companion-component truth holds on every claimed M5 companion and
//! handoff surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user actually triages a notification on, reviews a
//! change on, reads CI on, follows a session on, stays aware of an incident on, hands work back to
//! desktop from, or exports / gets help on (the notification inbox, the mobile review queue, the
//! CI-status dashboard, session follow, incident awareness, the desktop handoff, support / export,
//! and Help / docs), not on component family or primitive lane. Each
//! [`CompanionSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, share/export, degraded-state, and companion-boundary provenance — and
//! either passes (green), auto-narrows its companion claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier companion lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `LiveCompanionSafe` / `CachedContinuitySafe` claim while one of its
//! truth axes is not current — the object's freshness is stale, the companion authority is
//! limited, the tenant scope has narrowed, or the handoff validity is revoked — is over-claiming
//! and blocks; a surface that discloses the reduction by narrowing its companion claim (with a
//! bound reason and a frozen downgrade trigger) is honestly yellow. Companion truth never loses
//! continuity: a narrowed surface always preserves its object identity, workspace/repo client
//! scope, freshness, companion-versus-desktop capability boundary, severity, and exact handoff
//! target rather than dropping it between a triage, a review, a follow, an escalation, and a
//! desktop handoff. The always-on share/export axis must always stay certified, so support and
//! automation can reconstruct the same object-identity / client-scope / freshness / capability /
//! severity / handoff truth from the same object identity the user saw — never carrying a raw
//! code-bearing payload across the boundary.
//!
//! Every row cites exactly one canonical companion component proof bundle
//! ([`COMPANION_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw object bodies, code-bearing
//! payloads, and companion record contents never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-companion-component-certification.schema.json`](../../../../schemas/ui/m5-companion-component-certification.schema.json).
//! The contract doc is
//! [`docs/companion/m5_companion_component_certification_contract.md`](../../../../docs/companion/m5_companion_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_inbox_review_ci_session_follow_incident_advisory_and_browser_or_desktop_handoff_consumers_so_companion_components_keep_scope_freshness_and_desktop_required_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_companion_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_share_export_parity_and_automatic_narrowing_when_object_freshness_companion_authority_tenant_scope_or_handoff_validity_is_stale_limited_or_revoked_across_claimed_m5_companion_components as a11y;
use a11y::M5CompanionComponentClaim;
use matrix::{M5CompanionComponentFamily, M5CompanionDowngradeTrigger};

/// Schema version stamped on the M05-1003 certification packet.
pub const COMPANION_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CompanionSurfaceCertificationPacket`].
pub const COMPANION_CERT_RECORD_KIND: &str = "m5_companion_component_certification_packet";

/// Stable record-kind tag carried by each [`CompanionSurfaceCertificationRow`].
pub const COMPANION_CERT_ROW_RECORD_KIND: &str = "m5_companion_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const COMPANION_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-companion-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const COMPANION_CERT_DOC_REF: &str =
    "docs/companion/m5_companion_component_certification_contract.md";

/// Repo-relative path of the frozen companion component matrix schema the certified surfaces
/// render.
pub const COMPANION_CERT_MATRIX_REF: &str = matrix::M5_COMPANION_COMPONENT_SCHEMA_REF;

/// The one canonical companion component proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const COMPANION_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_COMPANION_COMPONENT_ARTIFACT_REF;

/// The M05-1001 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const COMPANION_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_COMPANION_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-1002 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// share / export parity this capstone builds on. Recorded as a supporting evidence ref on every
/// row.
pub const COMPANION_CERT_A11Y_BUNDLE_REF: &str = a11y::COMPANION_COMPONENT_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const COMPANION_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-companion-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COMPANION_CERT_CSV_REF: &str =
    "artifacts/release/m5-companion-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COMPANION_CERT_REPORT_REF: &str =
    "artifacts/release/m5-companion-component-certification/report.md";

/// The eight claimed M5 companion and handoff surfaces this capstone certifies. Keyed on the
/// surface a user actually triages a notification on, reviews a change on, reads CI on, follows a
/// session on, stays aware of an incident on, hands work back to desktop from, or exports / gets
/// help on, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionCertifiedSurface {
    /// The companion notification inbox / triage surface.
    NotificationInbox,
    /// The mobile review queue surface.
    MobileReviewQueue,
    /// The CI-status dashboard surface.
    CiStatusDashboard,
    /// The session-follow surface.
    SessionFollow,
    /// The incident-awareness surface.
    IncidentAwareness,
    /// The desktop-handoff surface.
    DesktopHandoff,
    /// The support / export bundle surface.
    SupportExport,
    /// The Help / docs surface.
    HelpDocs,
}

impl M5CompanionCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5CompanionCertifiedSurface; 8] = [
        M5CompanionCertifiedSurface::NotificationInbox,
        M5CompanionCertifiedSurface::MobileReviewQueue,
        M5CompanionCertifiedSurface::CiStatusDashboard,
        M5CompanionCertifiedSurface::SessionFollow,
        M5CompanionCertifiedSurface::IncidentAwareness,
        M5CompanionCertifiedSurface::DesktopHandoff,
        M5CompanionCertifiedSurface::SupportExport,
        M5CompanionCertifiedSurface::HelpDocs,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationInbox => "notification_inbox",
            Self::MobileReviewQueue => "mobile_review_queue",
            Self::CiStatusDashboard => "ci_status_dashboard",
            Self::SessionFollow => "session_follow",
            Self::IncidentAwareness => "incident_awareness",
            Self::DesktopHandoff => "desktop_handoff",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions
/// the spec requires verifying — visual, keyboard, screen-reader, share/export, degraded-state, and
/// companion-boundary provenance. The share/export axis is always-on and must stay certified for
/// every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionCertificationAxis {
    /// Visual parity: object identity, workspace/repo client scope, freshness,
    /// companion-versus-desktop capability boundary, severity, and exact handoff target are shown
    /// on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same identity / scope / freshness / capability / severity /
    /// handoff truth and its controls are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or
    /// a status glyph alone.
    ScreenReader,
    /// Share / export parity (always-on): the certified surface state is reconstructable as text /
    /// JSON / Markdown for support and automation, from the same object identity, never carrying a
    /// raw code-bearing payload across the boundary.
    ShareExport,
    /// Degraded-state parity: a stale object, a limited companion authority, a narrowed tenant
    /// scope, or a revoked handoff honestly downgrades a `LiveCompanionSafe` /
    /// `CachedContinuitySafe` claim to a weaker companion tier.
    DegradedState,
    /// Companion-boundary provenance parity: object identity, workspace/repo client scope,
    /// freshness, companion-versus-desktop capability boundary, severity, and exact handoff target
    /// stay explicit before any triage, review, follow, escalation, or handoff — never inheriting a
    /// healthier lane's companion truth, never masking a stale object, limited authority, narrowed
    /// tenant scope, or revoked handoff as a live companion-safe surface, never letting friendly
    /// companion wording conceal object scope, freshness, or the desktop-required capability
    /// boundary, and never dropping identity / scope / freshness / capability / severity / handoff
    /// continuity between a triage, a review, a follow, an escalation, and a desktop handoff.
    CompanionBoundaryProvenance,
}

impl CompanionCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [CompanionCertificationAxis; 6] = [
        CompanionCertificationAxis::Visual,
        CompanionCertificationAxis::Keyboard,
        CompanionCertificationAxis::ScreenReader,
        CompanionCertificationAxis::ShareExport,
        CompanionCertificationAxis::DegradedState,
        CompanionCertificationAxis::CompanionBoundaryProvenance,
    ];

    /// The always-on share/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::ShareExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::ShareExport => "share_export",
            Self::DegradedState => "degraded_state",
            Self::CompanionBoundaryProvenance => "companion_boundary_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited
    /// from a healthier surface.
    UndisclosedDrift,
}

impl CompanionAxisCertificationState {
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
pub enum CompanionSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed companion tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, share/export parity drops, companion
    /// truth is dropped, or the narrowing is inconsistent.
    Red,
}

impl CompanionSurfaceClaimStatus {
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

/// The copy / export parity a certified surface preserves. The share/export axis certifies only
/// when this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The object-identity / client-scope / freshness / capability / severity / handoff fields the
    /// surface preserves in export (never a raw code-bearing payload).
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CompanionCertExportParity {
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
pub struct CompanionAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: CompanionCertificationAxis,
    /// The certification state of the axis.
    pub state: CompanionAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5CompanionDowngradeTrigger>,
}

impl CompanionAxisOutcome {
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
            CompanionAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            CompanionAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            CompanionAxisCertificationState::UndisclosedDrift => {
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
pub struct CompanionClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: CompanionCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5CompanionComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5CompanionComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its object-identity / client-scope /
    /// freshness / capability / severity / handoff continuity rather than dropping it between a
    /// triage, a review, a follow, an escalation, and a desktop handoff.
    pub preserves_companion_truth_continuity: bool,
}

/// One certified M5 companion / handoff surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionSurfaceCertificationRow {
    /// Record kind; must equal [`COMPANION_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPANION_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5CompanionCertifiedSurface,
    /// The companion-claim ceiling the surface asserts.
    pub claimed_claim: M5CompanionComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5CompanionComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5CompanionComponentFamily>,
    /// One outcome per [`CompanionCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<CompanionAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<CompanionClaimAutoNarrow>,
    /// True when this surface never drops its object-identity / client-scope / freshness /
    /// capability / severity / handoff continuity between a triage, a review, a follow, an
    /// escalation, and a desktop handoff.
    pub companion_truth_preserved: bool,
    /// The one canonical companion component proof bundle this surface cites. Must equal
    /// [`COMPANION_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: CompanionSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: CompanionCertExportParity,
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

impl CompanionSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: CompanionCertificationAxis) -> Option<&CompanionAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<CompanionCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && CompanionCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(CompanionAxisOutcome::well_formed)
    }

    /// True when the surface narrows its companion claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<CompanionCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == CompanionAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its object-identity / client-scope / freshness /
    /// capability / severity / handoff continuity rather than dropping it. A non-narrowed surface
    /// trivially preserves companion truth; a narrowed one must say so.
    pub fn preserves_companion_truth_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => {
                self.companion_truth_preserved && narrow.preserves_companion_truth_continuity
            }
            None => self.companion_truth_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, share/export parity must
    /// always certify, companion truth must never drop continuity, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> CompanionSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != COMPANION_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_companion_truth_continuity()
        {
            return CompanionSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return CompanionSurfaceClaimStatus::Red;
        }

        // The always-on share/export axis must stay certified.
        match self.axis(CompanionCertificationAxis::ShareExport) {
            Some(o) if o.state == CompanionAxisCertificationState::Certified => {}
            _ => return CompanionSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == CompanionAxisCertificationState::UndisclosedDrift)
        {
            return CompanionSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return CompanionSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return CompanionSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_companion_truth_continuity
                {
                    return CompanionSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return CompanionSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return CompanionSurfaceClaimStatus::Red;
        }

        CompanionSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COMPANION_CERT_ROW_RECORD_KIND
            && self.schema_version == COMPANION_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed} companion_truth_preserved={preserved}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.companion_truth_preserved,
        )
    }
}

/// Rolled-up summary of an M05-1003 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionSurfaceCertificationSummary {
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
    pub all_companion_truth_preserved: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`CompanionSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<CompanionSurfaceCertificationRow>,
}

/// Checked-in M05-1003 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<CompanionSurfaceCertificationRow>,
    pub summary: CompanionSurfaceCertificationSummary,
}

impl CompanionSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CompanionSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COMPANION_CERT_SCHEMA_VERSION,
            record_kind: COMPANION_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: CompanionSurfaceCertificationSummary {
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
                all_companion_truth_preserved: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5CompanionCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5CompanionComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5CompanionCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5CompanionComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a share/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(CompanionCertificationAxis::ShareExport)
                .is_some_and(|o| o.state == CompanionAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CompanionSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CompanionSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CompanionSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CompanionSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(CompanionSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(CompanionSurfaceCertificationRow::preserves_companion_truth_continuity);

        CompanionSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == COMPANION_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(CompanionSurfaceCertificationRow::covers_all_axes),
            all_companion_truth_preserved: all_preserved,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CompanionCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COMPANION_CERT_SCHEMA_VERSION {
            violations.push(CompanionCertificationViolation::SchemaVersion {
                expected: COMPANION_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPANION_CERT_RECORD_KIND {
            violations.push(CompanionCertificationViolation::RecordKind {
                expected: COMPANION_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CompanionCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != COMPANION_CERT_CANONICAL_BUNDLE_REF {
            violations.push(CompanionCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CompanionCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(CompanionCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(CompanionCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(CompanionCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != COMPANION_CERT_CANONICAL_BUNDLE_REF {
                violations.push(CompanionCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // Share/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(CompanionCertificationAxis::ShareExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(CompanionCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Companion truth must never drop continuity.
            if !row.preserves_companion_truth_continuity() {
                violations.push(CompanionCertificationViolation::CompanionTruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    CompanionCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(CompanionCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == CompanionSurfaceClaimStatus::Red {
                violations.push(CompanionCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(CompanionCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(CompanionCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(CompanionCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(CompanionCertificationViolation::RawCompanionMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,companion_truth_preserved\n",
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
                preserved = row.companion_truth_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Companion Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5CompanionCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Companion truth preserved on every surface: {}\n",
            self.summary.all_companion_truth_preserved
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
pub fn current_m5_companion_component_certification_export(
) -> Result<CompanionSurfaceCertificationPacket, CompanionCertificationArtifactError> {
    let packet: CompanionSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-companion-component-certification/support_export.json"
    )))
    .map_err(CompanionCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CompanionCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum CompanionCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CompanionCertificationViolation>),
}

impl fmt::Display for CompanionCertificationArtifactError {
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

impl Error for CompanionCertificationArtifactError {}

/// Validation failure for M05-1003 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompanionCertificationViolation {
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
    CompanionTruthDropped { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawCompanionMaterialInExport,
}

impl fmt::Display for CompanionCertificationViolation {
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
                    "packet does not cite the canonical companion component proof bundle"
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
                    "row {id} does not cite the one canonical companion component proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on share/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CompanionTruthDropped { id } => {
                write!(
                    f,
                    "row {id} drops object-identity / client-scope / freshness / capability / severity / handoff continuity (a narrowed surface must preserve its companion truth between a triage, a review, a follow, an escalation, and a desktop handoff)"
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
share/export parity dropped, companion truth was dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 companion / handoff surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen companion component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawCompanionMaterialInExport => {
                write!(f, "export contains raw companion material")
            }
        }
    }
}

impl Error for CompanionCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&CompanionAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != CompanionAxisCertificationState::Certified,
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
            | "offline"
            | "blocked"
            | "paused"
            | "interrupted"
            | "incomplete"
            | "uncertain"
            | "expired"
            | "revoked"
            | "limited"
            | "diverged"
            | "not joinable"
            | "not_joinable"
            | "desktop required"
            | "desktop_required"
            | "policy blocked"
            | "policy_blocked"
            | "connected"
            | "live"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The companion vocabulary is metadata-only, so anything that could carry an actual credential
/// body or code-bearing payload — a password, a passphrase, a bearer token, a PEM block, or a URL
/// — is rejected outright.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
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

/// Builds the canonical, checked-in M05-1003 certification packet. Certifies all eight claimed M5
/// companion / handoff surfaces: four deliver their claim (green) and four auto-narrow a
/// not-current truth axis to a weaker companion ceiling (yellow). No surface hides drift (red), and
/// no surface drops object-identity / client-scope / freshness / capability / severity / handoff
/// continuity.
pub fn seeded_m5_companion_component_certification_packet() -> CompanionSurfaceCertificationPacket {
    CompanionSurfaceCertificationPacket::new(CompanionSurfaceCertificationPacketInput {
        packet_id: "m5-companion-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: COMPANION_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: COMPANION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:companion-component-certification:{id}"),
        COMPANION_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        COMPANION_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> CompanionCertExportParity {
    CompanionCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: CompanionCertificationAxis) -> &'static str {
    match axis {
        CompanionCertificationAxis::Visual => {
            "object identity, workspace/repo client scope, freshness, companion-versus-desktop capability boundary, severity, and exact handoff target shown on-surface"
        }
        CompanionCertificationAxis::Keyboard => {
            "the same identity/scope/freshness/capability/severity/handoff truth and its controls are keyboard-reachable"
        }
        CompanionCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        CompanionCertificationAxis::ShareExport => {
            "surface state exports as text / JSON / Markdown for support and automation from the same object identity, with the raw code-bearing payload excluded"
        }
        CompanionCertificationAxis::DegradedState => {
            "a stale object, a limited companion authority, a narrowed tenant scope, or a revoked handoff honestly downgrades the LiveCompanionSafe/CachedContinuitySafe claim"
        }
        CompanionCertificationAxis::CompanionBoundaryProvenance => {
            "object identity, client scope, freshness, companion-versus-desktop capability boundary, severity, and exact handoff target stay explicit before any triage, review, follow, escalation, or handoff; friendly companion wording never conceals scope, freshness, or the desktop-required boundary, and the boundary never drops companion continuity"
        }
    }
}

fn seed_certified(axis: CompanionCertificationAxis) -> CompanionAxisOutcome {
    CompanionAxisOutcome {
        axis,
        state: CompanionAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: CompanionCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5CompanionDowngradeTrigger,
) -> CompanionAxisOutcome {
    CompanionAxisOutcome {
        axis,
        state: CompanionAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<CompanionAxisOutcome> {
    CompanionCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: CompanionCertificationAxis,
    outcome: CompanionAxisOutcome,
) -> Vec<CompanionAxisOutcome> {
    CompanionCertificationAxis::ALL
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
    surface: M5CompanionCertifiedSurface,
    claimed_claim: M5CompanionComponentClaim,
    certified_claim: M5CompanionComponentClaim,
    consumed_families: &[M5CompanionComponentFamily],
    axis_outcomes: Vec<CompanionAxisOutcome>,
    claim_auto_narrow: Option<CompanionClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> CompanionSurfaceCertificationRow {
    let mut row = CompanionSurfaceCertificationRow {
        record_kind: COMPANION_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: COMPANION_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        companion_truth_preserved: true,
        canonical_bundle_ref: COMPANION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: CompanionSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            COMPANION_CERT_MATRIX_REF.to_owned(),
            COMPANION_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-09T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: CompanionCertificationAxis,
    from_claim: M5CompanionComponentClaim,
    to_claim: M5CompanionComponentClaim,
    label: &str,
) -> CompanionClaimAutoNarrow {
    CompanionClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_companion_truth_continuity: true,
    }
}

fn seeded_rows() -> Vec<CompanionSurfaceCertificationRow> {
    use CompanionCertificationAxis as Ax;
    use M5CompanionCertifiedSurface as S;
    use M5CompanionComponentClaim::*;
    use M5CompanionComponentFamily::*;
    use M5CompanionDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:notification-inbox",
            S::NotificationInbox,
            LiveCompanionSafe,
            LiveCompanionSafe,
            &[NotificationRow, MobileReviewCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "object_identity"],
            &[
                "the notification row keeps its object identity and client scope explicit so a quick triage verb lands on one stable object rather than a generic activity page",
                "the mobile review card keeps its companion-versus-desktop capability boundary explicit so a desktop-required review never reads as companion-completable",
                "keyboard/screen-reader reach preserved for the notification row and the review card",
                "provenance: a notification-inbox surface never leaves which object a tap opens or whether the card is fresh enough to trust implicit",
            ],
        ),
        seed_row(
            "cert:ci-status-dashboard",
            S::CiStatusDashboard,
            LiveCompanionSafe,
            LiveCompanionSafe,
            &[CiStatusCard, SessionFollowTile],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "freshness"],
            &[
                "the CI-status card names its provider/source class and stable run and commit identity so a stale status never reads as a live pass or fail",
                "the session-follow tile keeps its presenter/session identity and joinability explicit so an ended or diverged session never reads as joinable",
                "keyboard/screen-reader reach preserved for the CI-status card and the session-follow tile",
                "provenance: a CI-status-dashboard surface never implies a desktop-only rerun is companion-safe",
            ],
        ),
        seed_row(
            "cert:incident-awareness",
            S::IncidentAwareness,
            CachedContinuitySafe,
            CachedContinuitySafe,
            &[IncidentSnapshotCard, DesktopHandoffSheet],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "severity"],
            &[
                "the incident-snapshot card names its service/source class, stable service and run identity, severity, and latest status so a stale incident never reads as a live one",
                "the desktop-handoff sheet names its target object, stable target identity, and exactly what opens on desktop rather than implying an open without user archaeology",
                "keyboard/screen-reader reach preserved for the incident-snapshot card and the desktop-handoff sheet",
                "provenance: an incident-awareness surface stays awareness-only rather than overpromising remediation depth",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            CachedContinuitySafe,
            CachedContinuitySafe,
            &[NotificationRow, IncidentSnapshotCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "handoff_target"],
            &[
                "support export reconstructs object-identity/client-scope/freshness/capability/severity/handoff truth from the same object identity with the raw code-bearing payload excluded",
                "the exported notification row and incident-snapshot card keep their freshness and capability boundary explicit rather than reading as live",
                "the export prohibits a screenshot-only bundle so support and automation reconstruct the same truth as text/JSON/Markdown",
                "provenance: a companion export never carries a raw object body or code-bearing payload across the boundary",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:mobile-review-queue",
            S::MobileReviewQueue,
            LiveCompanionSafe,
            LimitedAuthorityProjection,
            &[MobileReviewCard],
            seed_certified_except(
                Ax::CompanionBoundaryProvenance,
                seed_narrowed(
                    Ax::CompanionBoundaryProvenance,
                    "the companion authority is limited on this surface and the full review action set requires desktop",
                    "The mobile-review-queue surface resolves a limited companion authority, so the LiveCompanionSafe claim narrows to limited-authority-projection instead of implying the review is companion-completable",
                    Trig::CapabilityBoundaryUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::CompanionBoundaryProvenance,
                LiveCompanionSafe,
                LimitedAuthorityProjection,
                "Authority limited: the mobile review card shows the full review action set requires desktop rather than presenting the review as companion-completable",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the mobile review card keeps the limited-authority reason explicit and names its exact desktop-handoff target",
                "the mobile review card keeps its object identity and client scope explicit while the authority stays limited",
                "companion-boundary: LiveCompanionSafe narrows to limited-authority-projection (auto-narrowed)",
                "known compatibility note: limited-authority behavior — a desktop-required review never reads as companion-safe",
            ],
        ),
        seed_row(
            "cert:session-follow",
            S::SessionFollow,
            LiveCompanionSafe,
            NarrowedTenantProjection,
            &[SessionFollowTile],
            seed_certified_except(
                Ax::CompanionBoundaryProvenance,
                seed_narrowed(
                    Ax::CompanionBoundaryProvenance,
                    "the tenant scope has narrowed from what the followed session now grants, so the effective scope is narrower than claimed",
                    "The session-follow surface resolves a narrowed tenant scope, so the LiveCompanionSafe claim narrows to narrowed-tenant-projection instead of implying the follow still spans the full tenant scope",
                    Trig::ClientScopeUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::CompanionBoundaryProvenance,
                LiveCompanionSafe,
                NarrowedTenantProjection,
                "Tenant scope narrowed: the session-follow tile shows the follow now spans a narrower tenant scope rather than implying the full scope holds",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the session-follow tile keeps the narrowed-scope reason explicit and names its exact desktop-handoff target",
                "the session-follow tile keeps its presenter/session identity explicit while the tenant scope stays narrowed",
                "companion-boundary: LiveCompanionSafe narrows to narrowed-tenant-projection (auto-narrowed)",
                "known compatibility note: narrowed-tenant behavior — a follow never reads as spanning more tenant scope than it now holds",
            ],
        ),
        seed_row(
            "cert:desktop-handoff",
            S::DesktopHandoff,
            LiveCompanionSafe,
            RevokedHandoffProjection,
            &[DesktopHandoffSheet],
            seed_certified_except(
                Ax::CompanionBoundaryProvenance,
                seed_narrowed(
                    Ax::CompanionBoundaryProvenance,
                    "the handoff validity is revoked and the desktop target can no longer resolve exactly, so the sheet is not-openable",
                    "The desktop-handoff surface resolves a revoked handoff, so the LiveCompanionSafe claim narrows to revoked-handoff-projection instead of implying a desktop client will open the intended object",
                    Trig::HandoffTargetUnresolved,
                ),
            ),
            Some(seed_narrow(
                Ax::CompanionBoundaryProvenance,
                LiveCompanionSafe,
                RevokedHandoffProjection,
                "Handoff revoked: the desktop-handoff sheet shows the desktop target can no longer resolve exactly rather than implying it will open the intended object",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the desktop-handoff sheet keeps the revoked-handoff reason explicit and degrades to an explicit not-openable state",
                "the desktop-handoff sheet keeps its target object and stable target identity explicit while the handoff stays revoked",
                "companion-boundary: LiveCompanionSafe narrows to revoked-handoff-projection (auto-narrowed)",
                "known compatibility note: revoked-handoff behavior — a sheet with no resolvable target never reads as openable",
            ],
        ),
        seed_row(
            "cert:help-docs",
            S::HelpDocs,
            LiveCompanionSafe,
            StaleFreshnessProjection,
            &[NotificationRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the object freshness is stale and the surface cannot confirm the object is current",
                    "The help/docs surface resolves a stale object freshness, so the LiveCompanionSafe claim narrows to stale-freshness-projection instead of implying the object is fresh enough to trust as live",
                    Trig::FreshnessHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                LiveCompanionSafe,
                StaleFreshnessProjection,
                "Freshness stale: the notification row shows the object is stale and labeled rather than presenting it as live",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the notification row keeps the stale-freshness reason explicit and offers a refresh entrypoint",
                "the notification row keeps its object identity and client scope explicit while the object stays stale",
                "degraded-state: LiveCompanionSafe narrows to stale-freshness-projection (auto-narrowed)",
                "known compatibility note: stale-freshness behavior — a stale object never reads as a live one",
            ],
        ),
    ]
}
