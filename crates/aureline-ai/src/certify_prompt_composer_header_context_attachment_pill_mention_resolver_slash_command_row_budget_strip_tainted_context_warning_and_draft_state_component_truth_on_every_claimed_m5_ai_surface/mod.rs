//! M05-891 surface certification over the frozen M5 prompt-composer component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`])
//! defines the nine reusable prompt-composer-header, context-attachment-pill,
//! mention-resolver, slash-command-row, budget / size-strip, tainted-context-warning,
//! draft-state-row, attachment-stale-banner, and split-send / review-control components,
//! the M05-885..888 primitive lanes narrow each one, the M05-889 consumer lane
//! ([`crate::add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers`])
//! proves they are reusable across the claimed AI composition consumers, and the M05-890
//! accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_mentions_are_unresolved_attachments_are_stale_budget_overflow_changes_composition_or_policy_blocks_routes_across_claimed_m5_composer_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing
//! capstone *certifies* that the shared prompt-composer component truth holds on every
//! claimed M5 AI composition surface — and auto-narrows any surface that cannot sustain
//! it.
//!
//! It is keyed on the claimed **surface** a user composes, attaches, reviews, or sends an
//! AI request through (inline composer, assistant panel, patch-review, branch / worktree
//! agent queue, docs / help console, companion app, CLI / headless, and support / export),
//! not on component family or primitive lane. Each [`ComposerSurfaceCertificationRow`]
//! certifies one surface across six truth axes — visual, keyboard, screen-reader,
//! CLI/export, degraded-state, and composition / send provenance — and either passes
//! (green), auto-narrows its composer support claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier AI composition surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `ReadyToSend` / `ReviewableComposition` claim while one of its
//! truth axes is not current — a mention is unresolved, an attachment is stale, the
//! composition overflows a hard budget, a pasted external block is tainted, the route
//! drifted or is policy-blocked, or the draft is offline / local-only — is over-claiming
//! and blocks; a surface that discloses the reduction by narrowing its support claim
//! (with a bound reason and a frozen downgrade trigger) is honestly yellow. The always-on
//! CLI/export axis must always stay certified, so support and automation can reconstruct
//! the same mode / scope / route / attachment identity / freshness / taint / omitted-context
//! / draft-locality / send-gate truth from the same draft identity the user saw.
//!
//! Every row cites exactly one canonical prompt-composer component proof bundle
//! ([`COMPOSER_CERT_CANONICAL_BUNDLE_REF`]) — the frozen prompt-composer component matrix
//! release proof — rather than cloning per-surface evidence. The packet is metadata-only:
//! raw draft bodies, pasted external text, provider tokens, and attachment contents never
//! cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ai/m5-prompt-composer-component-certification.schema.json`](../../../../schemas/ai/m5-prompt-composer-component-certification.schema.json).
//! The contract doc is
//! [`docs/ai/m5/m5_prompt_composer_component_certification_contract.md`](../../../../docs/ai/m5/m5_prompt_composer_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers as consumers;
use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_mentions_are_unresolved_attachments_are_stale_budget_overflow_changes_composition_or_policy_blocks_routes_across_claimed_m5_composer_components as a11y;
use a11y::M5ComposerSupportClaim;
use matrix::{M5ComposerDowngradeTrigger, M5PromptComposerComponentFamily};

/// Schema version stamped on the M05-891 certification packet.
pub const COMPOSER_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ComposerSurfaceCertificationPacket`].
pub const COMPOSER_CERT_RECORD_KIND: &str = "m5_prompt_composer_component_certification_packet";

/// Stable record-kind tag carried by each [`ComposerSurfaceCertificationRow`].
pub const COMPOSER_CERT_ROW_RECORD_KIND: &str = "m5_prompt_composer_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const COMPOSER_CERT_SCHEMA_REF: &str =
    "schemas/ai/m5-prompt-composer-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const COMPOSER_CERT_DOC_REF: &str =
    "docs/ai/m5/m5_prompt_composer_component_certification_contract.md";

/// Repo-relative path of the frozen prompt-composer component matrix schema the certified
/// surfaces render.
pub const COMPOSER_CERT_MATRIX_REF: &str = matrix::M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF;

/// The one canonical prompt-composer component proof bundle every certified surface cites
/// as its first-resolved component truth. All eight surfaces point back to it rather than
/// cloning per-surface evidence.
pub const COMPOSER_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_PROMPT_COMPOSER_COMPONENT_ARTIFACT_REF;

/// The M05-889 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const COMPOSER_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-890 accessibility / auto-narrowing support export whose keyboard / screen-reader
/// / CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on
/// every row.
pub const COMPOSER_CERT_A11Y_BUNDLE_REF: &str = a11y::COMPOSER_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const COMPOSER_CERT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COMPOSER_CERT_CSV_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COMPOSER_CERT_REPORT_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-certification/report.md";

/// The eight claimed M5 AI composition surfaces this capstone certifies. Keyed on the
/// surface a user actually composes, attaches, reviews, or sends AI work through, not on
/// the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromptComposerCertifiedSurface {
    /// The inline AI composer embedded in the editor / terminal.
    InlineComposer,
    /// The dedicated assistant panel composer.
    AssistantPanel,
    /// The guided patch-review composer where AI edits are approved before send.
    PatchReview,
    /// The branch / worktree background-agent composer / queue.
    BranchAgentQueue,
    /// The docs / help console composer that references AI runs and attachments.
    DocsHelpConsole,
    /// The companion-app composer.
    CompanionApp,
    /// The CLI / headless composition surface.
    CliHeadless,
    /// The support / export bundle surface.
    SupportExport,
}

impl M5PromptComposerCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5PromptComposerCertifiedSurface; 8] = [
        M5PromptComposerCertifiedSurface::InlineComposer,
        M5PromptComposerCertifiedSurface::AssistantPanel,
        M5PromptComposerCertifiedSurface::PatchReview,
        M5PromptComposerCertifiedSurface::BranchAgentQueue,
        M5PromptComposerCertifiedSurface::DocsHelpConsole,
        M5PromptComposerCertifiedSurface::CompanionApp,
        M5PromptComposerCertifiedSurface::CliHeadless,
        M5PromptComposerCertifiedSurface::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineComposer => "inline_composer",
            Self::AssistantPanel => "assistant_panel",
            Self::PatchReview => "patch_review",
            Self::BranchAgentQueue => "branch_agent_queue",
            Self::DocsHelpConsole => "docs_help_console",
            Self::CompanionApp => "companion_app",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, CLI/export,
/// degraded-state, and composition / send provenance. The CLI/export axis is always-on and
/// must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerCertificationAxis {
    /// Visual parity: composer mode, scope, route/provider/model, attachment identity,
    /// freshness / trust / taint, omitted / truncated context, draft locality, and send
    /// gate are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same mode / route / attachment / taint / budget / draft /
    /// send truth and its controls are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as
    /// text / JSON / Markdown for support and automation, from the same draft identity.
    CliExport,
    /// Degraded-state parity: an offline / local-only draft, an unreachable route, or a
    /// stale attachment honestly downgrades a `ReadyToSend` / `ReviewableComposition` claim
    /// to a weaker support tier.
    DegradedState,
    /// Composition / send provenance parity: composer mode / scope, route/provider/model,
    /// attachment identity, freshness / trust / taint, omitted / truncated context, draft
    /// locality, and the send / review gate stay explicit before send — never inheriting a
    /// healthier surface's composition truth or masking an unresolved mention, stale
    /// attachment, over-budget composition, tainted paste, or policy-blocked route as
    /// ready-to-send.
    CompositionAndSendProvenance,
}

impl ComposerCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ComposerCertificationAxis; 6] = [
        ComposerCertificationAxis::Visual,
        ComposerCertificationAxis::Keyboard,
        ComposerCertificationAxis::ScreenReader,
        ComposerCertificationAxis::CliExport,
        ComposerCertificationAxis::DegradedState,
        ComposerCertificationAxis::CompositionAndSendProvenance,
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
            Self::CompositionAndSendProvenance => "composition_and_send_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim
    /// inherited from a healthier surface.
    UndisclosedDrift,
}

impl ComposerAxisCertificationState {
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
pub enum ComposerSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, or the
    /// narrowing is inconsistent.
    Red,
}

impl ComposerSurfaceClaimStatus {
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
pub struct ComposerCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The mode / scope / route / attachment / taint / budget / draft / send fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl ComposerCertExportParity {
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
pub struct ComposerAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ComposerCertificationAxis,
    /// The certification state of the axis.
    pub state: ComposerAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ComposerDowngradeTrigger>,
}

impl ComposerAxisOutcome {
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
            ComposerAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ComposerAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ComposerAxisCertificationState::UndisclosedDrift => {
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
pub struct ComposerClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ComposerCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5ComposerSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5ComposerSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 AI composition surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerSurfaceCertificationRow {
    /// Record kind; must equal [`COMPOSER_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPOSER_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5PromptComposerCertifiedSurface,
    /// The composer support-claim ceiling the surface asserts.
    pub claimed_claim: M5ComposerSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5ComposerSupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5PromptComposerComponentFamily>,
    /// One outcome per [`ComposerCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ComposerAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ComposerClaimAutoNarrow>,
    /// The one canonical prompt-composer proof bundle this surface cites. Must equal
    /// [`COMPOSER_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ComposerSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: ComposerCertExportParity,
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

impl ComposerSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ComposerCertificationAxis) -> Option<&ComposerAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ComposerCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ComposerCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ComposerAxisOutcome::well_formed)
    }

    /// True when the surface narrows its support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ComposerCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ComposerAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of
    /// the capstone: a degraded axis must produce a visible claim narrowing, CLI/export
    /// parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ComposerSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != COMPOSER_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return ComposerSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ComposerSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ComposerCertificationAxis::CliExport) {
            Some(o) if o.state == ComposerAxisCertificationState::Certified => {}
            _ => return ComposerSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ComposerAxisCertificationState::UndisclosedDrift)
        {
            return ComposerSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ComposerSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ComposerSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return ComposerSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ComposerSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return ComposerSurfaceClaimStatus::Red;
        }

        ComposerSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COMPOSER_CERT_ROW_RECORD_KIND
            && self.schema_version == COMPOSER_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-891 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerSurfaceCertificationSummary {
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
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`ComposerSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ComposerSurfaceCertificationRow>,
}

/// Checked-in M05-891 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ComposerSurfaceCertificationRow>,
    pub summary: ComposerSurfaceCertificationSummary,
}

impl ComposerSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ComposerSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COMPOSER_CERT_SCHEMA_VERSION,
            record_kind: COMPOSER_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ComposerSurfaceCertificationSummary {
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
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5PromptComposerCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5PromptComposerComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5PromptComposerCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof
    /// the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5PromptComposerComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ComposerCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ComposerAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ComposerSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ComposerSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ComposerSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ComposerSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ComposerSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        ComposerSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == COMPOSER_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ComposerSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ComposerCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COMPOSER_CERT_SCHEMA_VERSION {
            violations.push(ComposerCertificationViolation::SchemaVersion {
                expected: COMPOSER_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPOSER_CERT_RECORD_KIND {
            violations.push(ComposerCertificationViolation::RecordKind {
                expected: COMPOSER_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ComposerCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != COMPOSER_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ComposerCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ComposerCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ComposerCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(ComposerCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ComposerCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != COMPOSER_CERT_CANONICAL_BUNDLE_REF {
                violations.push(ComposerCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ComposerCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(ComposerCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(ComposerCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ComposerCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == ComposerSurfaceClaimStatus::Red {
                violations.push(ComposerCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(ComposerCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(ComposerCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ComposerCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ComposerCertificationViolation::RawComposerMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Prompt-Composer Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5PromptComposerCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
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
pub fn current_m5_prompt_composer_component_certification_export(
) -> Result<ComposerSurfaceCertificationPacket, ComposerCertificationArtifactError> {
    let packet: ComposerSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-prompt-composer-component-certification/support_export.json"
    )))
    .map_err(ComposerCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ComposerCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ComposerCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ComposerCertificationViolation>),
}

impl fmt::Display for ComposerCertificationArtifactError {
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

impl Error for ComposerCertificationArtifactError {}

/// Validation failure for M05-891 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerCertificationViolation {
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
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawComposerMaterialInExport,
}

impl fmt::Display for ComposerCertificationViolation {
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
                    "packet does not cite the canonical prompt-composer proof bundle"
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
                    "row {id} does not cite the one canonical prompt-composer proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
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
CLI/export parity dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 AI composition surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen prompt-composer component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawComposerMaterialInExport => {
                write!(f, "export contains raw composer material")
            }
        }
    }
}

impl Error for ComposerCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ComposerAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ComposerAxisCertificationState::Certified,
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
            | "unresolved"
            | "tainted"
            | "over budget"
            | "over_budget"
            | "local only"
            | "local_only"
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

/// Builds the canonical, checked-in M05-891 certification packet. Certifies all eight
/// claimed M5 AI composition surfaces: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker support ceiling (yellow). No surface
/// hides drift (red).
pub fn seeded_m5_prompt_composer_component_certification_packet(
) -> ComposerSurfaceCertificationPacket {
    ComposerSurfaceCertificationPacket::new(ComposerSurfaceCertificationPacketInput {
        packet_id: "m5-prompt-composer-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: COMPOSER_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: COMPOSER_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:prompt-composer-certification:{id}"),
        COMPOSER_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        COMPOSER_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ComposerCertExportParity {
    ComposerCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ComposerCertificationAxis) -> &'static str {
    match axis {
        ComposerCertificationAxis::Visual => {
            "composer mode, scope, route/provider/model, attachment identity, freshness/trust/taint, omitted context, draft locality, and send gate shown on-surface"
        }
        ComposerCertificationAxis::Keyboard => {
            "the same mode/route/attachment/taint/budget/draft/send truth and its controls are keyboard-reachable"
        }
        ComposerCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        ComposerCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support from the same draft identity"
        }
        ComposerCertificationAxis::DegradedState => {
            "an offline/local-only draft, an unreachable route, or a stale attachment honestly downgrades the ReadyToSend/ReviewableComposition claim"
        }
        ComposerCertificationAxis::CompositionAndSendProvenance => {
            "composer mode/scope, route/provider/model, attachment identity, freshness/trust/taint, omitted context, draft locality, and the send/review gate stay explicit before send"
        }
    }
}

fn seed_certified(axis: ComposerCertificationAxis) -> ComposerAxisOutcome {
    ComposerAxisOutcome {
        axis,
        state: ComposerAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ComposerCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ComposerDowngradeTrigger,
) -> ComposerAxisOutcome {
    ComposerAxisOutcome {
        axis,
        state: ComposerAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ComposerAxisOutcome> {
    ComposerCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ComposerCertificationAxis,
    outcome: ComposerAxisOutcome,
) -> Vec<ComposerAxisOutcome> {
    ComposerCertificationAxis::ALL
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
    surface: M5PromptComposerCertifiedSurface,
    claimed_claim: M5ComposerSupportClaim,
    certified_claim: M5ComposerSupportClaim,
    consumed_families: &[M5PromptComposerComponentFamily],
    axis_outcomes: Vec<ComposerAxisOutcome>,
    claim_auto_narrow: Option<ComposerClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ComposerSurfaceCertificationRow {
    let mut row = ComposerSurfaceCertificationRow {
        record_kind: COMPOSER_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: COMPOSER_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: COMPOSER_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ComposerSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            COMPOSER_CERT_MATRIX_REF.to_owned(),
            COMPOSER_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ComposerCertificationAxis,
    from_claim: M5ComposerSupportClaim,
    to_claim: M5ComposerSupportClaim,
    label: &str,
) -> ComposerClaimAutoNarrow {
    ComposerClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<ComposerSurfaceCertificationRow> {
    use ComposerCertificationAxis as Ax;
    use M5ComposerDowngradeTrigger as Trig;
    use M5ComposerSupportClaim::*;
    use M5PromptComposerCertifiedSurface as S;
    use M5PromptComposerComponentFamily::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:inline-composer",
            S::InlineComposer,
            ReadyToSend,
            ReadyToSend,
            &[PromptComposerHeader, ContextAttachmentPill],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "composer_mode"],
            &[
                "the composer header names the active mode, scope, and route/provider/model before send",
                "the context-attachment pill names each attached object identity and its trust/freshness state",
                "keyboard/screen-reader reach preserved for the header and attachment-pill controls",
                "boundary/send: an inline composition never leaves its route/provider or attachment identity implicit",
            ],
        ),
        seed_row(
            "cert:assistant-panel",
            S::AssistantPanel,
            ReviewableComposition,
            ReviewableComposition,
            &[SlashCommandRow, SendReviewControl],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "send_gate"],
            &[
                "the slash-command row keeps its stable command id, capability class, and disabled-state explanation",
                "the split-send / review control keeps the explain-only / review / mutating send paths distinct",
                "export reconstructs the command availability and send-gate truth from the same draft object",
                "boundary/send: a reviewable composition never collapses a widened-authority send into a one-tap action",
            ],
        ),
        seed_row(
            "cert:patch-review",
            S::PatchReview,
            ReadyToSend,
            ReadyToSend,
            &[SendReviewControl, BudgetSizeStrip],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "budget_headroom"],
            &[
                "the send-review control keeps the review-before-send gate explicit for the patch composition",
                "the budget / size strip keeps included / omitted context classes and truncation reasons visible",
                "keyboard/screen-reader reach preserved for review / send and the omitted-context drawer",
                "boundary/send: a truncated patch composition never reads as fully within budget",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReadyToSend,
            ReadyToSend,
            &[DraftStateRow, AttachmentStaleBanner],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "draft_locality"],
            &[
                "support export reconstructs mode/scope/route/attachment/taint/budget/draft/send truth from the same draft identity",
                "the draft-state row keeps its locality / retention posture explicit with no hidden sharing",
                "text / JSON / Markdown reconstruction certified for support handoff",
                "boundary/send: a support packet never exports raw draft bodies, pasted text, or attachment contents",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:branch-agent-queue",
            S::BranchAgentQueue,
            ReadyToSend,
            LocalOnlyComposition,
            &[DraftStateRow, PromptComposerHeader],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "route unreachable; the queued draft is held offline / local-only",
                    "The branch-agent route is unreachable, so the queued draft is held offline and cannot leave the shell live; the ReadyToSend claim narrows to local-only instead of presenting an offline draft as send-ready",
                    Trig::DraftLocalityMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReadyToSend,
                LocalOnlyComposition,
                "Offline draft: the branch-agent route is unreachable, so the composition is held local-only; the draft-state row shows the locality rather than implying a live send",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the draft-state row keeps the offline / local-only posture and the local-safe alternative explicit",
                "the composer header keeps its route/provider explicit as unreachable rather than masking it",
                "degraded-state: ReadyToSend narrows to local-only (auto-narrowed)",
                "known compatibility note: offline / local-only fallback — an offline draft never reads as send-ready",
            ],
        ),
        seed_row(
            "cert:docs-help-console",
            S::DocsHelpConsole,
            ReviewableComposition,
            PolicyBlockedComposition,
            &[SlashCommandRow, ContextAttachmentPill],
            seed_certified_except(
                Ax::CompositionAndSendProvenance,
                seed_narrowed(
                    Ax::CompositionAndSendProvenance,
                    "the referenced composition route is policy-blocked",
                    "A route referenced from the docs / help console is policy-blocked for this workspace, so the ReviewableComposition claim narrows to policy-blocked instead of presenting a blocked route as an available reviewable send path",
                    Trig::RouteOrProviderMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::CompositionAndSendProvenance,
                ReviewableComposition,
                PolicyBlockedComposition,
                "Policy-blocked route: the referenced composition route is blocked by policy; the console shows the route posture and block reason rather than an available send path",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the slash-command row keeps its command id and disabled-state explanation for the policy-owned route",
                "the context-attachment pill keeps its object identity and trust state visible",
                "composition/send: ReviewableComposition narrows to policy-blocked (auto-narrowed)",
                "known compatibility note: policy-owned route narrowing — a policy-blocked route never reads as available",
            ],
        ),
        seed_row(
            "cert:companion-app",
            S::CompanionApp,
            ReadyToSend,
            NarrowedComposition,
            &[AttachmentStaleBanner, ContextAttachmentPill],
            seed_certified_except(
                Ax::CompositionAndSendProvenance,
                seed_narrowed(
                    Ax::CompositionAndSendProvenance,
                    "the companion mirror can only attach a narrowed attachment scope",
                    "The companion app resolves attachments from a mirrored, narrowed scope rather than the exact one the desktop composer would, so the ReadyToSend claim narrows to narrowed-composition instead of implying the exact attachment scope was sent",
                    Trig::AttachmentFreshnessMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::CompositionAndSendProvenance,
                ReadyToSend,
                NarrowedComposition,
                "Narrowed attachment scope: the companion attaches from a mirrored scope; the attachment-stale banner and pill show the narrowed scope rather than the exact requested one",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the attachment-stale banner keeps the staleness reason and draft-preserving refresh path explicit",
                "the context-attachment pill keeps its object identity and narrowed scope explicit",
                "composition/send: ReadyToSend narrows to narrowed-composition (auto-narrowed)",
                "known compatibility note: unsupported consumer scope — a narrowed companion scope never reads as the exact one",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ReadyToSend,
            UnresolvedComposition,
            &[MentionResolver, TaintedContextWarning],
            seed_certified_except(
                Ax::CompositionAndSendProvenance,
                seed_narrowed(
                    Ax::CompositionAndSendProvenance,
                    "a mention could not be resolved to an exact target in the headless run",
                    "The headless composition references a mention that cannot be resolved to an exact target, so the ReadyToSend claim narrows to unresolved instead of presenting an ambiguous mention as a confirmed send target",
                    Trig::MentionLeftUnresolved,
                ),
            ),
            Some(seed_narrow(
                Ax::CompositionAndSendProvenance,
                ReadyToSend,
                UnresolvedComposition,
                "Unresolved mention: the headless composition has a mention with no exact target; the resolver row shows it as unresolved rather than a confirmed target",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the mention resolver keeps the unresolved mention explicit in the structured output, blocking an ambiguous send",
                "the tainted-context warning keeps any pasted-external taint source and severity explicit",
                "composition/send: ReadyToSend narrows to unresolved (auto-narrowed)",
                "known compatibility note: unresolved mention — an ambiguous mention never reads as a confirmed send target",
            ],
        ),
    ]
}
