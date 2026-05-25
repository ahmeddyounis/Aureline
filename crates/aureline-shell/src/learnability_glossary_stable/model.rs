//! Canonical stable truth model for learnability, glossary, and contextual
//! docs/help guidance on a claimed-stable switching row.
//!
//! ## Why one disclosure record per switching row
//!
//! A user switching from an incumbent editor asks, at the moment a flow is in
//! front of them: *why does this matter now, what is this thing called here, and
//! where do I read more?* When the switching row, the docs/help browser, the
//! command palette, and the menu each answer with their own bespoke copy they
//! drift — a glossary chip points at a moved coordinate instead of a stable
//! command, a why-now card claims to be grounded in product truth when it is
//! not, a tutorial funnel blocks first useful work, or a Beta guided tour sits
//! next to Stable cards and inherits "Stable" by adjacency.
//!
//! This module mints one governed [`LearnabilityDisclosureRecord`] per imported
//! source ecosystem (the switching cohort). The record binds, for a single
//! canonical switching identity:
//!
//! - **The why-now card** — an inline, dismissible card explaining why the flow
//!   matters now for a switcher, grounded in command/file/symbol truth.
//! - **The glossary chips** — incumbent term → Aureline term mappings, every
//!   chip citing a stable command/file/symbol/docs anchor (never a coordinate).
//! - **The contextual docs/help** — docs/help node refs reachable in place
//!   without losing the switcher's focus.
//! - **The learnability posture** — opt-in and non-blocking: the layer never
//!   forces a tutorial funnel before first useful work and preserves exact focus
//!   return.
//! - **The guided-affordance lifecycle markers** — any guided tour, learning
//!   mode, teaching, or presentation affordance present on the row carries its
//!   own `Preview`/`Beta`/`Stable` marker and support boundary, so it never
//!   implies full stable coverage by adjacency.
//! - **The learning-state privacy posture** — dismissals, resume entries, and
//!   the learning digest stay user-owned and local-first, never repo-visible or
//!   telemetry-grade by accident.
//! - **A public claim ceiling** — no row may assert its glossary anchors are
//!   stable, its why-now card is grounded in truth, its contextual docs are
//!   cited, its focus return is preserved, or its layer is non-blocking unless
//!   the product can prove it.
//! - **Automatic narrowing** — a row missing any learnability pillar is narrowed
//!   below Stable with a named reason instead of inheriting an adjacent green
//!   row.
//! - **Recovery, route, and accessibility parity** — Reopen-why-now / Open
//!   glossary / Open contextual docs / Resume tour / Dismiss-and-return-focus /
//!   Export-support routes, the same row reachable from the switching row, the
//!   docs/help browser, the command palette, and a menu command, with narration,
//!   action labels, and affordances reachable in normal, high-contrast, and
//!   zoomed layouts.
//! - **No-account / no-managed-services availability** — every row stays listed
//!   even when identity or managed services are absent.
//!
//! The switching cohort, glossary anchors, and privacy posture are **not**
//! reinvented here: the ecosystem vocabulary is [`crate::migration_corpus`] and
//! the stable target/anchor type is [`crate::learning_mode::LearningTargetRef`],
//! so there is no parallel model.

use serde::{Deserialize, Serialize};

use crate::learning_mode::LearningTargetRef;
use crate::migration_corpus::IncumbentEcosystem;

/// Stable record-kind tag carried in serialized disclosure records.
pub const LEARNABILITY_DISCLOSURE_RECORD_KIND: &str = "learnability_glossary_disclosure_record";

/// Schema version for the [`LearnabilityDisclosureRecord`] payload shape.
pub const LEARNABILITY_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const LEARNABILITY_DISCLOSURE_SHARED_CONTRACT_REF: &str =
    "shell:learnability_glossary_disclosure_stable:v1";

/// Reviewer-facing notice rendered on every disclosure surface.
pub const LEARNABILITY_DISCLOSURE_NOTICE: &str =
    "Learnability disclosure truth: the switching row, docs/help browser, command palette, and \
     menus show the same opt-in, non-blocking learnability layer — a why-now card grounded in \
     command/file/symbol truth, glossary chips that each cite a stable anchor, and contextual \
     docs/help reachable in place — and the same guided-tour/learning/teaching lifecycle markers; \
     no row claims its anchors are stable, its why-now card is grounded, its docs are cited, its \
     focus return is preserved, or its layer is non-blocking unless the product can prove it; a \
     row missing any pillar is narrowed below Stable with a named reason rather than inheriting an \
     adjacent green row; any guided affordance carries its own Preview/Beta/Stable marker and \
     support boundary rather than implying stable coverage by adjacency; dismissals, resume \
     entries, and the learning digest stay user-owned and local-first; the same row opens from \
     every surface, keyboard-first; and every row stays available without an account or managed \
     services.";

/// Canonical durable-object URI scheme. Every minted ref must be one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one is rejected so chrome cannot
/// wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home", "dashboard", "landing", "index", "overview", "start", "root",
];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

/// Returns `true` when a learning target is a stable command, file, or symbol
/// anchor — the "command/file/symbol truth" the v8 stable bar requires. A
/// docs-node anchor is a valid *contextual docs* citation but is not, on its
/// own, command/file/symbol grounding.
pub fn is_command_file_symbol_anchor(target: &LearningTargetRef) -> bool {
    matches!(
        target.target_kind.as_str(),
        "command_id" | "file_object_id" | "symbol_object_id"
    ) && target.is_stable_anchor()
}

/// Public claim class for the lane, reusing the stable lifecycle cutline.
///
/// `Stable` sits at or above the launch cutline; everything else is narrowed
/// below it. The builder *derives* this from the evidence, so a row can never
/// publish a claim wider than its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The learnability layer is replacement-grade.
    Stable,
    /// Narrowed to the beta promise.
    Beta,
    /// Narrowed to the preview/limited-availability promise.
    Preview,
    /// No public promise yet.
    NotClaimed,
}

impl StableClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }

    /// Returns `true` when the claim sits at or above the launch cutline.
    pub const fn at_or_above_cutline(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Closed reason a row is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableNarrowingReason {
    /// At least one glossary chip does not cite a stable anchor.
    GlossaryAnchorsNotStable,
    /// The why-now card is not grounded in command/file/symbol truth.
    WhyNowCardNotGroundedInTruth,
    /// The contextual docs/help layer does not cite any docs node.
    ContextualDocsUncited,
    /// The layer does not preserve exact focus return.
    FocusReturnNotPreserved,
    /// The layer is not opt-in / blocks first useful work with a tutorial funnel.
    BlocksFirstUsefulWork,
}

impl StableNarrowingReason {
    /// Returns the stable string vocabulary for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GlossaryAnchorsNotStable => "glossary_anchors_not_stable",
            Self::WhyNowCardNotGroundedInTruth => "why_now_card_not_grounded_in_truth",
            Self::ContextualDocsUncited => "contextual_docs_uncited",
            Self::FocusReturnNotPreserved => "focus_return_not_preserved",
            Self::BlocksFirstUsefulWork => "blocks_first_useful_work",
        }
    }
}

/// Lifecycle marker carried by a guided affordance present on the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMarker {
    /// Preview / limited-availability.
    Preview,
    /// Beta promise.
    Beta,
    /// Replacement-grade stable.
    Stable,
}

impl LifecycleMarker {
    /// Returns the stable string vocabulary for this marker.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Returns `true` when the marker sits below the stable cutline.
    pub const fn is_below_stable(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Closed vocabulary for the guided affordances that may be present on a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuidedAffordanceKind {
    /// A step-by-step guided tour.
    GuidedTour,
    /// Learning mode (guided exercises and progress).
    LearningMode,
    /// A teaching/classroom session.
    TeachingSession,
    /// Presentation / teaching mode for demos.
    PresentationMode,
}

impl GuidedAffordanceKind {
    /// Returns the stable string vocabulary for this affordance kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuidedTour => "guided_tour",
            Self::LearningMode => "learning_mode",
            Self::TeachingSession => "teaching_session",
            Self::PresentationMode => "presentation_mode",
        }
    }

    /// Returns the reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::GuidedTour => "Guided tour",
            Self::LearningMode => "Learning mode",
            Self::TeachingSession => "Teaching session",
            Self::PresentationMode => "Presentation mode",
        }
    }
}

/// Surface a row can be reached from. The same row must be reachable from all
/// four so the switching row and in-product learnability surfaces stay
/// consistent for keyboard-only and assistive-technology users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityRouteSurface {
    /// The claimed-stable switching row (migration / start center).
    SwitchingRow,
    /// The docs/help browser.
    DocsHelpBrowser,
    /// The command palette.
    CommandPalette,
    /// An application menu command.
    MenuCommand,
}

impl LearnabilityRouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SwitchingRow => "switching_row",
            Self::DocsHelpBrowser => "docs_help_browser",
            Self::CommandPalette => "command_palette",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a row.
    pub const REQUIRED: [Self; 4] = [
        Self::SwitchingRow,
        Self::DocsHelpBrowser,
        Self::CommandPalette,
        Self::MenuCommand,
    ];
}

/// Layout mode an accessibility disclosure is checked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    /// Default desktop layout.
    Normal,
    /// High-contrast theme.
    HighContrast,
    /// Zoomed / enlarged layout.
    Zoomed,
}

impl LayoutMode {
    /// Returns the stable string vocabulary for this layout mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::HighContrast => "high_contrast",
            Self::Zoomed => "zoomed",
        }
    }

    /// The three layout modes every disclosure must hold in.
    pub const REQUIRED: [Self; 3] = [Self::Normal, Self::HighContrast, Self::Zoomed];
}

/// Role a recovery action plays, used for placement and confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionRole {
    /// Opens or reopens the canonical learnability artifact.
    Primary,
    /// Restores focus or resumes guided progress.
    Recovery,
    /// Non-destructive open or export.
    Secondary,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
        }
    }
}

/// Closed recovery-action vocabulary exposed on a learnability row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityRecoveryAction {
    /// Reopen the inline why-now card.
    ReopenWhyNowCard,
    /// Open the glossary for this switching cohort.
    OpenGlossary,
    /// Open the contextual docs/help node in place.
    OpenContextualDocs,
    /// Resume the guided tour from the last step (only when a tour is present).
    ResumeGuidedTour,
    /// Dismiss the learnability layer and return focus exactly.
    DismissAndReturnFocus,
    /// Export a redacted learning-support packet.
    ExportLearningSupport,
}

impl LearnabilityRecoveryAction {
    /// Returns the stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenWhyNowCard => "reopen_why_now_card",
            Self::OpenGlossary => "open_glossary",
            Self::OpenContextualDocs => "open_contextual_docs",
            Self::ResumeGuidedTour => "resume_guided_tour",
            Self::DismissAndReturnFocus => "dismiss_and_return_focus",
            Self::ExportLearningSupport => "export_learning_support",
        }
    }

    /// Returns the reviewer-facing action label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::ReopenWhyNowCard => "Reopen why-now card",
            Self::OpenGlossary => "Open glossary",
            Self::OpenContextualDocs => "Open contextual docs",
            Self::ResumeGuidedTour => "Resume guided tour",
            Self::DismissAndReturnFocus => "Dismiss and return focus",
            Self::ExportLearningSupport => "Export learning support",
        }
    }

    /// Returns the placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::ReopenWhyNowCard | Self::OpenGlossary => RecoveryActionRole::Primary,
            Self::ResumeGuidedTour | Self::DismissAndReturnFocus => RecoveryActionRole::Recovery,
            Self::OpenContextualDocs | Self::ExportLearningSupport => RecoveryActionRole::Secondary,
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery actions a row must expose, in rendered order, given
/// whether a guided affordance is present on the row.
pub fn required_recovery_actions(has_guided_affordance: bool) -> Vec<LearnabilityRecoveryAction> {
    let mut actions = vec![
        LearnabilityRecoveryAction::ReopenWhyNowCard,
        LearnabilityRecoveryAction::OpenGlossary,
        LearnabilityRecoveryAction::OpenContextualDocs,
    ];
    if has_guided_affordance {
        actions.push(LearnabilityRecoveryAction::ResumeGuidedTour);
    }
    actions.push(LearnabilityRecoveryAction::DismissAndReturnFocus);
    actions.push(LearnabilityRecoveryAction::ExportLearningSupport);
    actions
}

/// The inline why-now card for a switching row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhyNowCard {
    /// Stable card id.
    pub card_id: String,
    /// Reviewable headline.
    pub headline: String,
    /// Reviewable body explaining why this matters now for a switcher.
    pub body: String,
    /// The command/file/symbol/docs anchor the card cites.
    pub cited_target: LearningTargetRef,
    /// Whether the card can be dismissed (it must, to be opt-in).
    pub dismissible: bool,
    /// Whether the card blocks first useful work (it must not).
    pub blocks_first_useful_work: bool,
}

impl WhyNowCard {
    /// Returns `true` when the card is grounded in command/file/symbol truth.
    pub fn is_grounded_in_truth(&self) -> bool {
        is_command_file_symbol_anchor(&self.cited_target)
    }
}

/// One glossary chip mapping an incumbent term to its Aureline equivalent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryChip {
    /// Stable chip id.
    pub chip_id: String,
    /// The incumbent term a switcher already knows.
    pub incumbent_term: String,
    /// The Aureline term it maps to.
    pub aureline_term: String,
    /// Reviewable one-line explanation of the mapping.
    pub explanation: String,
    /// The stable command/file/symbol/docs anchor the chip resolves to.
    pub anchor: LearningTargetRef,
    /// Docs/help node ref backing the chip (repo-relative source path).
    pub docs_help_ref: String,
}

impl GlossaryChip {
    /// Returns `true` when the chip cites a stable anchor.
    pub fn has_stable_anchor(&self) -> bool {
        self.anchor.is_stable_anchor()
    }
}

/// The contextual docs/help disclosure for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualDocsDisclosure {
    /// Canonical docs-browser surface ref.
    pub docs_browser_ref: String,
    /// Docs/help node refs cited in place (repo-relative source paths).
    pub help_node_refs: Vec<String>,
    /// Whether the docs open in place without losing the switcher's focus.
    pub opens_in_place: bool,
}

impl ContextualDocsDisclosure {
    /// Returns `true` when the layer cites at least one docs node.
    pub fn cites_docs_nodes(&self) -> bool {
        !self.help_node_refs.is_empty()
    }
}

/// The opt-in / non-blocking / focus-return posture of the learnability layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityPosture {
    /// Whether the layer is opt-in (offered, never forced).
    pub opt_in: bool,
    /// Whether the layer blocks first useful work with a tutorial funnel.
    pub blocks_first_useful_work: bool,
    /// Whether dismiss/close returns focus to exactly where it was.
    pub preserves_exact_focus_return: bool,
    /// Canonical anchor focus returns to on dismiss/close.
    pub focus_return_anchor_ref: String,
}

impl LearnabilityPosture {
    /// Returns `true` when the layer is opt-in and never forces a funnel.
    pub fn is_non_blocking(&self) -> bool {
        self.opt_in && !self.blocks_first_useful_work
    }
}

/// One guided affordance present on the row, carrying its own lifecycle marker
/// and support boundary so it never implies stable coverage by adjacency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedAffordanceDisclosure {
    /// Stable affordance id.
    pub affordance_id: String,
    /// Affordance kind.
    pub affordance_kind: GuidedAffordanceKind,
    /// The lifecycle marker (`Preview`/`Beta`/`Stable`) it publishes.
    pub lifecycle_marker: LifecycleMarker,
    /// Reviewable support-boundary sentence stating what is and isn't covered.
    pub support_boundary: String,
    /// Whether the marker is visible in-product.
    pub marker_visible_in_product: bool,
    /// Whether the marker is visible in docs/help.
    pub marker_visible_in_docs_help: bool,
    /// Whether the marker is visible in the support export.
    pub marker_visible_in_support_export: bool,
}

impl GuidedAffordanceDisclosure {
    /// Returns `true` when the marker is disclosed on every surface.
    pub fn marker_fully_disclosed(&self) -> bool {
        self.marker_visible_in_product
            && self.marker_visible_in_docs_help
            && self.marker_visible_in_support_export
    }

    /// Returns `true` when the affordance sits below the stable cutline.
    pub fn is_below_stable(&self) -> bool {
        self.lifecycle_marker.is_below_stable()
    }
}

/// The learning-state privacy posture: dismissals, resume entries, and the
/// learning digest stay user-owned and local-first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningStatePrivacyPosture {
    /// Canonical local state-store ref the row's learning state lives in.
    pub state_store_ref: String,
    /// Whether why-now / card dismissals are user-owned.
    pub dismissals_user_owned: bool,
    /// Whether guided resume entries are user-owned.
    pub resume_entries_user_owned: bool,
    /// Whether the learning digest is user-owned.
    pub learning_digest_user_owned: bool,
    /// Whether the learning state is repo-visible (it must not be).
    pub repo_visible: bool,
    /// Whether the learning state is telemetry-grade (it must not be).
    pub telemetry_grade: bool,
}

impl LearningStatePrivacyPosture {
    /// Returns `true` when every piece of state is user-owned.
    pub fn all_user_owned(&self) -> bool {
        self.dismissals_user_owned && self.resume_entries_user_owned && self.learning_digest_user_owned
    }

    /// Returns `true` when the state stays local-first and unleaked.
    pub fn is_local_first(&self) -> bool {
        self.all_user_owned() && !self.repo_visible && !self.telemetry_grade
    }
}

/// The public claim ceiling: what a row is allowed to assert. Each field must be
/// provable from the row's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LearnabilityClaimCeiling {
    /// Whether the row may claim every glossary chip cites a stable anchor.
    pub asserts_glossary_anchors_stable: bool,
    /// Whether the row may claim the why-now card is grounded in truth.
    pub asserts_why_now_grounded: bool,
    /// Whether the row may claim contextual docs/help are cited.
    pub asserts_contextual_docs_cited: bool,
    /// Whether the row may claim exact focus return is preserved.
    pub asserts_focus_return_preserved: bool,
    /// Whether the row may claim the layer is opt-in and non-blocking.
    pub asserts_non_blocking: bool,
}

/// The derived stable-claim verdict for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the row qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the row is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<StableNarrowingReason>,
}

/// One recovery route exposed on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
}

/// One route to the same row from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: LearnabilityRouteSurface,
    /// Canonical route ref pointing at the row on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same canonical learnability row.
    pub activates_same_row: bool,
}

/// Accessibility disclosure for one layout mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutModeDisclosure {
    /// Layout mode this disclosure was checked under.
    pub mode: LayoutMode,
    /// Whether the row narration is available in this mode.
    pub row_narration_available: bool,
    /// Whether the recovery affordances stay reachable in this mode.
    pub recovery_affordances_reachable: bool,
}

/// Accessibility disclosure for one row across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the row in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the row and its actions expose.
    pub tab_stop_count: u32,
    /// Row narration read by assistive tech; discloses the source ecosystem.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

/// Cross-surface parity between the switching row and docs/help projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParity {
    /// Switching-row id for this row.
    pub switching_row_id: String,
    /// Docs/help browser row id for this row.
    pub docs_help_row_id: String,
    /// Command-palette command id that opens this row.
    pub command_palette_command_id: String,
    /// Recovery action ids shared by both surfaces.
    pub recovery_action_ids: Vec<String>,
    /// Reopen surfaces (docs_help / command_palette / support_export) retained.
    pub reopen_surfaces: Vec<String>,
    /// Whether the projections agree on identity and recovery behaviour.
    pub parity_holds: bool,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRefs {
    /// Migration corpus scoreboard id the switching cohort came from.
    pub migration_scoreboard_ref: String,
    /// Source-ecosystem corpus section ref.
    pub corpus_section_ref: String,
    /// Learning-mode manifest id the guided affordance came from.
    pub learning_manifest_ref: String,
    /// Learning-mode surface projection id the anchors came from.
    pub learning_surface_projection_ref: String,
}

/// Validated input used to mint a [`LearnabilityDisclosureRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnabilityDisclosureInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Source ecosystem (switching cohort) this row serves.
    pub source_ecosystem: IncumbentEcosystem,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The inline why-now card.
    pub why_now_card: WhyNowCard,
    /// The glossary chips for this cohort.
    pub glossary_chips: Vec<GlossaryChip>,
    /// The contextual docs/help disclosure.
    pub contextual_docs: ContextualDocsDisclosure,
    /// The opt-in / non-blocking / focus-return posture.
    pub posture: LearnabilityPosture,
    /// Guided affordances present on the row, lifecycle-marked.
    pub guided_affordances: Vec<GuidedAffordanceDisclosure>,
    /// The learning-state privacy posture.
    pub privacy: LearningStatePrivacyPosture,
    /// Public claim ceiling for this row.
    pub claim_ceiling: LearnabilityClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same row.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the row stays available without an account.
    pub available_without_account: bool,
    /// Whether the row stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed learnability-disclosure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityDisclosureRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Source ecosystem (switching cohort) this row serves.
    pub source_ecosystem: IncumbentEcosystem,
    /// Compact source-ecosystem label (the vocabulary docs / Help/About ingest).
    pub source_ecosystem_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The inline why-now card.
    pub why_now_card: WhyNowCard,
    /// The glossary chips for this cohort.
    pub glossary_chips: Vec<GlossaryChip>,
    /// The contextual docs/help disclosure.
    pub contextual_docs: ContextualDocsDisclosure,
    /// The opt-in / non-blocking / focus-return posture.
    pub posture: LearnabilityPosture,
    /// Guided affordances present on the row, lifecycle-marked.
    pub guided_affordances: Vec<GuidedAffordanceDisclosure>,
    /// The learning-state privacy posture.
    pub privacy: LearningStatePrivacyPosture,
    /// Public claim ceiling.
    pub claim_ceiling: LearnabilityClaimCeiling,
    /// The derived stable-claim verdict (Stable, or narrowed with reasons).
    pub stable_qualification: StableQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same row.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the row stays available without an account.
    pub available_without_account: bool,
    /// Whether the row stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`LearnabilityDisclosureRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A learnability row was minted with no glossary chips.
    EmptyGlossary,
    /// The claim ceiling asserted stable glossary anchors it cannot prove.
    OverclaimsGlossaryAnchors,
    /// The claim ceiling asserted a grounded why-now card it cannot prove.
    OverclaimsWhyNowGrounded,
    /// The claim ceiling asserted cited contextual docs it cannot prove.
    OverclaimsContextualDocs,
    /// The claim ceiling asserted preserved focus return it cannot prove.
    OverclaimsFocusReturn,
    /// The claim ceiling asserted a non-blocking layer it cannot prove.
    OverclaimsNonBlocking,
    /// A guided affordance did not disclose its marker on every surface.
    GuidedAffordanceMarkerHidden { affordance_id: String },
    /// A guided affordance lacked a support-boundary sentence.
    GuidedAffordanceSupportBoundaryMissing { affordance_id: String },
    /// Learning state was repo-visible.
    LearningStateRepoVisible,
    /// Learning state was telemetry-grade.
    LearningStateTelemetryGrade,
    /// Learning state was not user-owned by default.
    LearningStateNotUserOwned,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: LearnabilityRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// The two surface projections disagreed on identity or recovery behaviour.
    SurfaceParityBroken,
    /// A required reopen surface was missing.
    ReopenSurfaceMissing { surface: &'static str },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: LearnabilityRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: LearnabilityRouteSurface },
    /// An entry route did not activate the same canonical row.
    RouteTargetsDifferentRow { surface: LearnabilityRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: LearnabilityRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The row narration did not disclose the source ecosystem.
    NarrationOmitsEcosystem,
    /// A row was hidden when no account was present.
    HiddenWithoutAccount,
    /// A row was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::EmptyGlossary => {
                write!(f, "a learnability row must carry at least one glossary chip")
            }
            Self::OverclaimsGlossaryAnchors => write!(
                f,
                "claim ceiling may not assert stable glossary anchors when a chip lacks one"
            ),
            Self::OverclaimsWhyNowGrounded => write!(
                f,
                "claim ceiling may not assert a grounded why-now card without command/file/symbol truth"
            ),
            Self::OverclaimsContextualDocs => write!(
                f,
                "claim ceiling may not assert cited contextual docs when none are cited"
            ),
            Self::OverclaimsFocusReturn => write!(
                f,
                "claim ceiling may not assert preserved focus return when the layer does not preserve it"
            ),
            Self::OverclaimsNonBlocking => write!(
                f,
                "claim ceiling may not assert a non-blocking layer that blocks first useful work"
            ),
            Self::GuidedAffordanceMarkerHidden { affordance_id } => write!(
                f,
                "guided affordance `{affordance_id}` must disclose its lifecycle marker in product, docs/help, and support export"
            ),
            Self::GuidedAffordanceSupportBoundaryMissing { affordance_id } => write!(
                f,
                "guided affordance `{affordance_id}` must carry a support-boundary sentence"
            ),
            Self::LearningStateRepoVisible => {
                write!(f, "learning state must not be repo-visible")
            }
            Self::LearningStateTelemetryGrade => {
                write!(f, "learning state must not be telemetry-grade")
            }
            Self::LearningStateNotUserOwned => {
                write!(f, "learning state must remain user-owned by default")
            }
            Self::MissingRecoveryRoute { action } => {
                write!(f, "row must expose recovery route `{}`", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => write!(
                f,
                "recovery route `{action_id}` must be keyboard reachable"
            ),
            Self::SurfaceParityBroken => write!(
                f,
                "switching row and docs/help projections must share identity and recovery behaviour"
            ),
            Self::ReopenSurfaceMissing { surface } => {
                write!(f, "reopen surface `{surface}` is missing")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentRow { surface } => write!(
                f,
                "entry route surface `{}` must activate the same row",
                surface.as_str()
            ),
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "entry route surface `{}` is duplicated", surface.as_str())
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "accessibility layout mode `{}` is missing", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::NarrationOmitsEcosystem => {
                write!(f, "row narration must disclose the source ecosystem")
            }
            Self::HiddenWithoutAccount => {
                write!(f, "a learnability row must stay available without an account")
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a learnability row must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl LearnabilityDisclosureRecord {
    /// Builds a governed disclosure record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about the glossary anchors, the why-now card, contextual docs, focus
    /// return, the non-blocking posture, guided-affordance lifecycle markers,
    /// the privacy posture, recovery, cross-surface parity, route reachability,
    /// or accessibility. The stable claim class is *derived* from the evidence,
    /// so a row can never publish a claim wider than its proof.
    pub fn build(input: LearnabilityDisclosureInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_reviewable_sentence(&input.why_now_card.headline) {
            return Err(BuildError::InvalidSentence {
                field: "why_now_card.headline",
            });
        }
        if !is_reviewable_sentence(&input.why_now_card.body) {
            return Err(BuildError::InvalidSentence {
                field: "why_now_card.body",
            });
        }
        require_ref(
            "contextual_docs.docs_browser_ref",
            &input.contextual_docs.docs_browser_ref,
        )?;
        require_ref(
            "posture.focus_return_anchor_ref",
            &input.posture.focus_return_anchor_ref,
        )?;
        require_ref("privacy.state_store_ref", &input.privacy.state_store_ref)?;
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_ref("narrative_refs", narrative)?;
        }

        // --- glossary integrity ----------------------------------------------
        if input.glossary_chips.is_empty() {
            return Err(BuildError::EmptyGlossary);
        }
        for chip in &input.glossary_chips {
            if !is_reviewable_sentence(&chip.explanation) {
                return Err(BuildError::InvalidSentence {
                    field: "glossary_chips.explanation",
                });
            }
        }

        // --- guided-affordance honesty: no stable-by-adjacency ----------------
        for affordance in &input.guided_affordances {
            if !affordance.marker_fully_disclosed() {
                return Err(BuildError::GuidedAffordanceMarkerHidden {
                    affordance_id: affordance.affordance_id.clone(),
                });
            }
            if !is_reviewable_sentence(&affordance.support_boundary) {
                return Err(BuildError::GuidedAffordanceSupportBoundaryMissing {
                    affordance_id: affordance.affordance_id.clone(),
                });
            }
        }

        // --- privacy: local-first, user-owned, never leaked -------------------
        if input.privacy.repo_visible {
            return Err(BuildError::LearningStateRepoVisible);
        }
        if input.privacy.telemetry_grade {
            return Err(BuildError::LearningStateTelemetryGrade);
        }
        if !input.privacy.all_user_owned() {
            return Err(BuildError::LearningStateNotUserOwned);
        }

        // --- derive the pillars from the evidence -----------------------------
        let glossary_anchors_stable = input.glossary_chips.iter().all(GlossaryChip::has_stable_anchor);
        let why_now_grounded = input.why_now_card.is_grounded_in_truth();
        let contextual_docs_cited = input.contextual_docs.cites_docs_nodes();
        let focus_return_preserved =
            input.posture.preserves_exact_focus_return && !input.posture.focus_return_anchor_ref.is_empty();
        let non_blocking = input.posture.is_non_blocking() && !input.why_now_card.blocks_first_useful_work;

        // --- claim ceiling: never claim what the product cannot prove ---------
        if input.claim_ceiling.asserts_glossary_anchors_stable && !glossary_anchors_stable {
            return Err(BuildError::OverclaimsGlossaryAnchors);
        }
        if input.claim_ceiling.asserts_why_now_grounded && !why_now_grounded {
            return Err(BuildError::OverclaimsWhyNowGrounded);
        }
        if input.claim_ceiling.asserts_contextual_docs_cited && !contextual_docs_cited {
            return Err(BuildError::OverclaimsContextualDocs);
        }
        if input.claim_ceiling.asserts_focus_return_preserved && !focus_return_preserved {
            return Err(BuildError::OverclaimsFocusReturn);
        }
        if input.claim_ceiling.asserts_non_blocking && !non_blocking {
            return Err(BuildError::OverclaimsNonBlocking);
        }

        // --- recovery routes -------------------------------------------------
        let has_guided_affordance = !input.guided_affordances.is_empty();
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(has_guided_affordance) {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- cross-surface parity --------------------------------------------
        if !input.surfaces.parity_holds {
            return Err(BuildError::SurfaceParityBroken);
        }
        let parity_ids: Vec<&str> = input
            .surfaces
            .recovery_action_ids
            .iter()
            .map(String::as_str)
            .collect();
        if parity_ids != route_ids {
            return Err(BuildError::SurfaceParityBroken);
        }
        for required in ["docs_help", "command_palette", "support_export"] {
            if !input
                .surfaces
                .reopen_surfaces
                .iter()
                .any(|surface| surface == required)
            {
                return Err(BuildError::ReopenSurfaceMissing { surface: required });
            }
        }

        // --- route parity across surfaces ------------------------------------
        let mut seen_surfaces = Vec::new();
        for route in &input.routes {
            if seen_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_surfaces.push(route.surface);
            require_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_row {
                return Err(BuildError::RouteTargetsDifferentRow {
                    surface: route.surface,
                });
            }
        }
        for required in LearnabilityRouteSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        let ecosystem_label = input.source_ecosystem.display_label().to_string();
        if !input.accessibility.row_narration.contains(&ecosystem_label) {
            return Err(BuildError::NarrationOmitsEcosystem);
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability: never bury a row behind account or services -------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- derive the stable-claim verdict from the evidence ---------------
        let mut narrowing_reasons = Vec::new();
        if !glossary_anchors_stable {
            narrowing_reasons.push(StableNarrowingReason::GlossaryAnchorsNotStable);
        }
        if !why_now_grounded {
            narrowing_reasons.push(StableNarrowingReason::WhyNowCardNotGroundedInTruth);
        }
        if !contextual_docs_cited {
            narrowing_reasons.push(StableNarrowingReason::ContextualDocsUncited);
        }
        if !focus_return_preserved {
            narrowing_reasons.push(StableNarrowingReason::FocusReturnNotPreserved);
        }
        if !non_blocking {
            narrowing_reasons.push(StableNarrowingReason::BlocksFirstUsefulWork);
        }
        let claim_class = if narrowing_reasons.is_empty() {
            StableClaimClass::Stable
        } else {
            StableClaimClass::Beta
        };
        let qualifies_stable = narrowing_reasons.is_empty();
        let stable_qualification = StableQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };

        let honesty_marker_present = !qualifies_stable
            || input
                .guided_affordances
                .iter()
                .any(GuidedAffordanceDisclosure::is_below_stable);

        Ok(Self {
            record_kind: LEARNABILITY_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: LEARNABILITY_DISCLOSURE_SCHEMA_VERSION,
            notice: LEARNABILITY_DISCLOSURE_NOTICE.to_string(),
            shared_contract_ref: LEARNABILITY_DISCLOSURE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            source_ecosystem: input.source_ecosystem,
            source_ecosystem_label: ecosystem_label,
            title: input.title,
            summary: input.summary,
            why_now_card: input.why_now_card,
            glossary_chips: input.glossary_chips,
            contextual_docs: input.contextual_docs,
            posture: input.posture,
            guided_affordances: input.guided_affordances,
            privacy: input.privacy,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            surfaces: input.surfaces,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: input.upstream,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("learnability_disclosure: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!(
                "source_ecosystem: {} ({})",
                self.source_ecosystem.as_str(),
                self.source_ecosystem_label
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "why_now_card: {} grounded={} dismissible={} blocks_first_useful_work={}",
                self.why_now_card.card_id,
                self.why_now_card.is_grounded_in_truth(),
                self.why_now_card.dismissible,
                self.why_now_card.blocks_first_useful_work
            ),
            format!(
                "contextual_docs: cites_docs_nodes={} opens_in_place={} nodes={}",
                self.contextual_docs.cites_docs_nodes(),
                self.contextual_docs.opens_in_place,
                self.contextual_docs.help_node_refs.len()
            ),
            format!(
                "posture: opt_in={} blocks_first_useful_work={} preserves_focus_return={}",
                self.posture.opt_in,
                self.posture.blocks_first_useful_work,
                self.posture.preserves_exact_focus_return
            ),
            format!(
                "claim_ceiling: glossary_anchors_stable={} why_now_grounded={} contextual_docs_cited={} focus_return_preserved={} non_blocking={}",
                self.claim_ceiling.asserts_glossary_anchors_stable,
                self.claim_ceiling.asserts_why_now_grounded,
                self.claim_ceiling.asserts_contextual_docs_cited,
                self.claim_ceiling.asserts_focus_return_preserved,
                self.claim_ceiling.asserts_non_blocking
            ),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "privacy: local_first={} user_owned={} repo_visible={} telemetry_grade={}",
                self.privacy.is_local_first(),
                self.privacy.all_user_owned(),
                self.privacy.repo_visible,
                self.privacy.telemetry_grade
            ),
        ];
        lines.push("glossary_chips:".to_string());
        for chip in &self.glossary_chips {
            lines.push(format!(
                "  - {} : {} -> {} [stable_anchor={}] -- {}",
                chip.chip_id,
                chip.incumbent_term,
                chip.aureline_term,
                chip.has_stable_anchor(),
                chip.explanation
            ));
        }
        lines.push("guided_affordances:".to_string());
        for affordance in &self.guided_affordances {
            lines.push(format!(
                "  - {} ({}) marker={} fully_disclosed={} -- {}",
                affordance.affordance_id,
                affordance.affordance_kind.as_str(),
                affordance.lifecycle_marker.as_str(),
                affordance.marker_fully_disclosed(),
                affordance.support_boundary
            ));
        }
        lines.push("recovery_routes:".to_string());
        for route in &self.recovery_routes {
            lines.push(format!(
                "  - {} ({}) role={} keyboard={}",
                route.action_id,
                route.action_label,
                route.action_role.as_str(),
                route.keyboard_reachable
            ));
        }
        lines.push(format!(
            "surfaces: switching_row={} docs_help={} command={} parity_holds={} reopen=[{}]",
            self.surfaces.switching_row_id,
            self.surfaces.docs_help_row_id,
            self.surfaces.command_palette_command_id,
            self.surfaces.parity_holds,
            self.surfaces.reopen_surfaces.join(", ")
        ));
        lines.push("routes:".to_string());
        for route in &self.routes {
            lines.push(format!(
                "  - {} -> {} keyboard={} same_row={}",
                route.surface.as_str(),
                route.route_ref,
                route.keyboard_reachable,
                route.activates_same_row
            ));
        }
        lines.push(format!(
            "accessibility: tab_order={} tab_stops={} narration={:?}",
            self.accessibility.focus_order_index,
            self.accessibility.tab_stop_count,
            self.accessibility.row_narration
        ));
        for mode in &self.accessibility.layout_modes {
            lines.push(format!(
                "  layout {} narration={} affordances_reachable={}",
                mode.mode.as_str(),
                mode.row_narration_available,
                mode.recovery_affordances_reachable
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "upstream: scoreboard={} section={} learning_manifest={} learning_surface={}",
            self.upstream.migration_scoreboard_ref,
            self.upstream.corpus_section_ref,
            self.upstream.learning_manifest_ref,
            self.upstream.learning_surface_projection_ref
        ));
        lines.push(format!("diagnostics_export_ref: {}", self.diagnostics_export_ref));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}
