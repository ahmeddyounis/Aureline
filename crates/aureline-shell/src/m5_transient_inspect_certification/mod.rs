//! Tooltip, hovercard, and peek-panel representation, promotion, keyboard-reach,
//! and stale-labeling truth certified across every claimed M5 transient-inspect
//! context.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the transient
//! inspect primitives — the tooltip, the hovercard, the peek panel, and the
//! pinned-preview promotion — into one export-safe packet: their representation
//! classes, promotion states, source/provider/freshness labels, accessibility
//! routes, and the mandatory labels every transient inspect surface must be able to
//! show. This lane is the **transient-inspect certification capstone** on top of
//! that matrix: for every claimed M5 inspect context — the search, docs/help,
//! review/change, editor, data-grid, profiler, and operator lanes — it certifies
//! that a tooltip, hovercard, or peek panel preserves canonical target identity,
//! source/provider class, freshness/mapping quality, and representation label; that
//! pinning or promoting a peek keeps that same identity and state without dropping
//! its representation or provenance truth; that no glanceable information is
//! hover-only or pointer-only but stays reachable through keyboard focus, an explicit
//! context action, or an info affordance on touch/pen and compact layouts; and that
//! stale, cached, or approximate preview content stays visibly labeled before and
//! after pinning and is reconstructable from a support export.
//!
//! Three records carry the truth:
//!
//! - the per-context **certification row** ([`TransientInspectCertificationRow`]):
//!   one row per [`M5InspectContext`] naming the transient inspect primitives it
//!   drives, the representation classes / promotion states / freshness labels /
//!   required labels / accessibility routes / consumer surfaces / downgrade triggers
//!   pulled from the frozen matrix, its representation / promotion / keyboard-reach /
//!   stale-labeling posture, any active waiver, and a derived green/yellow/red
//!   [`TransientInspectCertificationStatus`].
//! - the release **certification packet** ([`TransientInspectCertificationPacket`]):
//!   the full set of rows with derived per-row status, aggregate green/yellow/red
//!   counts, the active waivers, the exact certification causes
//!   ([`TransientInspectCertificationCause`]), and the blocking findings the lane
//!   refuses to ship with.
//! - the **certification dashboard** ([`TransientInspectCertificationDashboard`]): a
//!   light projection the shell / attention router / release automation reads to
//!   auto-narrow a claimed inspect context when its certification proof falls out of
//!   policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment it discloses a reduced representation detail, a reduced
//! promotion path (backed by a waiver), a reduced non-hover reach route, or a
//! partial support-export capture; it drops to `red` if it hides source/provider or
//! freshness truth, a promotion drops the target's identity or representation, its
//! information becomes hover- or pointer-only, a stale preview reads as live or is
//! absent from capture, a tooltip carries the sole critical instruction, or its
//! representation classes / promotion states / required labels / stale labels are
//! incomplete. That derivation is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials —
//! only stable ids, closed vocabulary, counts, refs, and short labels. The
//! representation-class, promotion-state, source-freshness, required-label,
//! accessibility-route, consumer-surface, downgrade-trigger, and qualification
//! vocabulary is re-exported by reference from the already frozen [matrix]; each row
//! pulls its transient-inspect bindings straight from that matrix's seeded tooltip,
//! hovercard, peek-panel, and pinned-preview rows, so this lane mints no parallel
//! shell vocabulary and cannot certify a transient-inspect posture the matrix does
//! not freeze. Only the certification-specific vocabulary ([`M5InspectContext`],
//! [`M5TransientInspectProofDimension`], [`TransientInspectCertificationStatus`],
//! [`RepresentationTruthState`], [`PromotionContinuityState`], [`NonHoverReachState`],
//! [`StalePreviewLabelingState`], [`TransientInspectCertificationWaiver`],
//! [`TransientInspectCertificationCause`], [`TransientInspectCertificationFinding`])
//! is new.
//!
//! [matrix]: crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix as matrix;

pub use matrix::{
    M5AccessibilityRoute, M5PrimitiveQualificationClass, M5PrimitiveRequiredLabel,
    M5PromotionState, M5RepresentationClass, M5ShellConsumerSurface,
    M5ShellPrimitiveDowngradeTrigger, M5ShellPrimitiveFamily, M5ShellZoneSlot,
    M5SourceFreshnessLabel,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_transient_inspect_certification_packet,
    seeded_m5_transient_inspect_certification_packet_data_stale_reads_live_blocked,
    seeded_m5_transient_inspect_certification_packet_docs_promotion_dropped_blocked,
    seeded_m5_transient_inspect_certification_packet_editor_hover_only_blocked,
    seeded_m5_transient_inspect_certification_packet_operator_tooltip_sole_instruction_blocked,
    seeded_m5_transient_inspect_certification_packet_search_representation_hidden_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "shell:m5_transient_inspect_certification:v1";

/// Stable record kind for [`TransientInspectCertificationPacket`] payloads.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "shell_m5_transient_inspect_certification_packet_record";

/// Stable record kind for [`TransientInspectCertificationDashboard`] payloads.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_transient_inspect_certification_dashboard_record";

/// Stable record kind for [`TransientInspectCertificationSupportExport`] payloads.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_transient_inspect_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PACKET_ID: &str =
    "m5-transient-inspect-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_DASHBOARD_ID: &str =
    "m5-transient-inspect-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-transient-inspect-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-transient-inspect-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-transient-inspect-certification.md";

/// Published certification-packet artifact ref.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-transient-inspect-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-transient-inspect-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-transient-inspect-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-transient-inspect-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_transient_inspect_certification_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_TRANSIENT_INSPECT_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// One of the claimed M5 inspect contexts the certification proof must cover, in
/// canonical order. Each context is a claimed M5 shell lane whose surfaces render
/// tooltips, hovercards, or peek panels; the lane certifies none beyond them and
/// refuses to ship if any is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InspectContext {
    /// Search results / command-palette lane.
    SearchResults,
    /// Docs / help lane.
    DocsHelp,
    /// Review / change-request lane.
    ReviewChange,
    /// Editor / code lane.
    Editor,
    /// Data grid / API-run lane.
    DataGrid,
    /// Profiler / performance-capture lane.
    Profiler,
    /// Operator / incident-console lane.
    Operator,
}

impl M5InspectContext {
    /// Every governed inspect context, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::SearchResults,
        Self::DocsHelp,
        Self::ReviewChange,
        Self::Editor,
        Self::DataGrid,
        Self::Profiler,
        Self::Operator,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResults => "search_results",
            Self::DocsHelp => "docs_help",
            Self::ReviewChange => "review_change",
            Self::Editor => "editor",
            Self::DataGrid => "data_grid",
            Self::Profiler => "profiler",
            Self::Operator => "operator",
        }
    }

    /// Short, reviewer-facing label for the context's transient inspect surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SearchResults => "Search results tooltips & peek",
            Self::DocsHelp => "Docs / help hovercards",
            Self::ReviewChange => "Review / change hovercards & peek",
            Self::Editor => "Editor symbol tooltips & peek",
            Self::DataGrid => "Data grid cell hovercards & peek",
            Self::Profiler => "Profiler flame-graph peek",
            Self::Operator => "Operator console tooltips & peek",
        }
    }
}

/// One of the four certification dimensions each inspect context is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TransientInspectProofDimension {
    /// Representation truth (identity, source/provider, freshness, representation).
    RepresentationTruth,
    /// Promotion continuity (pin/open/full-view paths preserve identity & state).
    PromotionContinuity,
    /// Non-hover reach (keyboard / focus / context / info affordance).
    NonHoverReach,
    /// Stale-preview labeling (stale/cached/approximate labeled + reconstructable).
    StalePreviewLabeling,
}

impl M5TransientInspectProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RepresentationTruth,
        Self::PromotionContinuity,
        Self::NonHoverReach,
        Self::StalePreviewLabeling,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepresentationTruth => "representation_truth",
            Self::PromotionContinuity => "promotion_continuity",
            Self::NonHoverReach => "non_hover_reach",
            Self::StalePreviewLabeling => "stale_preview_labeling",
        }
    }
}

/// The derived certification light a governed inspect context carries.
///
/// `green` means the context's transient inspect surfaces preserve identity /
/// source-provider / freshness / representation truth, promote without dropping
/// identity or state, stay reachable without hover, and keep stale content labeled
/// and reconstructable. `yellow` is a disclosed narrowing (a reduced representation
/// detail, a waivered reduced promotion path, a reduced non-hover reach route, or a
/// partial support-export capture). `red` is blocked: source/provider or freshness
/// truth is hidden, a promotion drops identity or representation, information is hover-
/// or pointer-only, a stale preview reads as live or is absent from capture, a tooltip
/// carries the sole critical instruction, or the representation classes / promotion
/// states / required labels / stale labels are incomplete — and the context may not
/// keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransientInspectCertificationStatus {
    /// Full standing: preserved representation, safe promotion, non-hover reach.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl TransientInspectCertificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the context's transient inspect surfaces preserve representation truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationTruthState {
    /// Every tooltip / hovercard / peek carries canonical target identity, its
    /// source/provider class, its freshness or mapping quality, and its
    /// representation label.
    IdentitySourceFreshnessRepresentationLabeled,
    /// Under compact width the representation detail is disclosedly reduced (a
    /// hovercard falls back to a shorter form) while identity, source/provider, and
    /// freshness stay labeled.
    DisclosedReducedRepresentationDetail,
    /// A surface hides its source/provider or freshness truth so a cached or stale
    /// value can read as live canonical content — always a blocker.
    SourceProviderOrFreshnessHidden,
}

impl RepresentationTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentitySourceFreshnessRepresentationLabeled => {
                "identity_source_freshness_representation_labeled"
            }
            Self::DisclosedReducedRepresentationDetail => "disclosed_reduced_representation_detail",
            Self::SourceProviderOrFreshnessHidden => "source_provider_or_freshness_hidden",
        }
    }

    /// `true` when representation truth is fully labeled.
    pub const fn is_labeled(self) -> bool {
        matches!(self, Self::IdentitySourceFreshnessRepresentationLabeled)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedRepresentationDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::SourceProviderOrFreshnessHidden)
    }
}

/// How the context's peeks pin, open, and promote without losing identity or state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionContinuityState {
    /// Hovercards and peeks expose clear pin / open / full-view paths, and every
    /// promotion or demotion keeps the same object identity, state, and
    /// representation truth; peeks disclose preview-only versus live-editable posture
    /// before promotion.
    PinOpenPathsPreserveIdentityAndState,
    /// One promotion path is disclosedly reduced (a detach-to-window step is deferred)
    /// while pin/open still preserve identity and state; the reduction is disclosed
    /// and waivered.
    DisclosedReducedPromotionPath,
    /// A promotion or pin drops the target's identity or representation truth, or a
    /// peek promotes without disclosing its preview-only versus live-editable posture
    /// — always a blocker.
    PromotionDropsIdentityOrRepresentation,
}

impl PromotionContinuityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinOpenPathsPreserveIdentityAndState => {
                "pin_open_paths_preserve_identity_and_state"
            }
            Self::DisclosedReducedPromotionPath => "disclosed_reduced_promotion_path",
            Self::PromotionDropsIdentityOrRepresentation => {
                "promotion_drops_identity_or_representation"
            }
        }
    }

    /// `true` when promotion preserves identity and state on every path.
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::PinOpenPathsPreserveIdentityAndState)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedPromotionPath)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::PromotionDropsIdentityOrRepresentation)
    }
}

/// How every glanceable piece of transient inspect information stays reachable
/// without hover on touch/pen and compact layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonHoverReachState {
    /// Every hover-only piece of information is also reachable through keyboard
    /// focus, an explicit context action, or an info affordance on touch/pen and
    /// compact layouts.
    KeyboardFocusContextReachable,
    /// One non-hover reach route is temporarily reduced but at least one route
    /// (keyboard focus, context action, or info affordance) remains and the
    /// reduction is disclosed.
    DisclosedReducedReachRoute,
    /// Information is reachable only through pointer hover or pointer interaction —
    /// always a blocker.
    InformationHoverOrPointerOnly,
}

impl NonHoverReachState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusContextReachable => "keyboard_focus_context_reachable",
            Self::DisclosedReducedReachRoute => "disclosed_reduced_reach_route",
            Self::InformationHoverOrPointerOnly => "information_hover_or_pointer_only",
        }
    }

    /// `true` when every route resolves without hover.
    pub const fn is_reachable(self) -> bool {
        matches!(self, Self::KeyboardFocusContextReachable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedReachRoute)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::InformationHoverOrPointerOnly)
    }
}

/// How stale / cached / approximate preview content stays labeled and reconstructable
/// before and after pinning or promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StalePreviewLabelingState {
    /// Stale, cached, and approximate preview content is visibly labeled before and
    /// after pinning, and the promotion packet is reconstructable from a support
    /// export with the same identity, representation, and freshness labels.
    StaleLabeledAndExportReconstructable,
    /// The support export reconstructs the visible preview and discloses a partial
    /// capture of the promoted / pinned set while a refresh is in flight.
    DisclosedPartialCapture,
    /// A stale, cached, or approximate preview reads as live canonical content, or a
    /// pinned preview is absent from the support-export capture — always a blocker.
    StaleReadsAsLiveOrAbsentFromCapture,
}

impl StalePreviewLabelingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleLabeledAndExportReconstructable => {
                "stale_labeled_and_export_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::StaleReadsAsLiveOrAbsentFromCapture => {
                "stale_reads_as_live_or_absent_from_capture"
            }
        }
    }

    /// `true` when stale content is labeled and the export reconstructs everything.
    pub const fn is_labeled(self) -> bool {
        matches!(self, Self::StaleLabeledAndExportReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::StaleReadsAsLiveOrAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red promotion narrowing
/// stay yellow rather than blocked — never lets a hidden representation, a dropped
/// promotion, a hover-only reach, or an unlabeled stale preview hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed context the waiver applies to.
    pub context: M5InspectContext,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl TransientInspectCertificationWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed context's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`]
/// vocabulary so a cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationCause {
    /// The governed context the cause applies to.
    pub context: M5InspectContext,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl TransientInspectCertificationCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed inspect context, certified across representation truth, promotion
/// continuity, non-hover reach, and stale-preview labeling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationRow {
    /// The governed context being certified.
    pub context: M5InspectContext,
    /// The transient inspect primitives this context drives. Pulled from the matrix.
    pub driven_primitive_families: Vec<M5ShellPrimitiveFamily>,
    /// The frozen qualification class of the driven transient inspect primitives
    /// (the most-narrowed of the four). Pulled from the matrix.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this context certified.
    pub owner_role: String,
    /// Short context-surface label.
    pub context_label: String,
    /// The canonical shell zone the transient inspect surfaces attach to. Pulled from
    /// the matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Representation classes these surfaces carry (union across the four transient
    /// families). Pulled from the matrix.
    pub certified_representation_classes: Vec<M5RepresentationClass>,
    /// Promotion states these surfaces honour across pin / promote / demote. Pulled
    /// from the matrix.
    pub certified_promotion_states: Vec<M5PromotionState>,
    /// Source / provider / freshness labels these surfaces can show. Pulled from the
    /// matrix.
    pub source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels every surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems this context stays aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this context. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Representation-truth posture.
    pub representation_truth: RepresentationTruthState,
    /// Promotion-continuity posture.
    pub promotion_continuity: PromotionContinuityState,
    /// Non-hover-reach posture.
    pub non_hover_reach: NonHoverReachState,
    /// Stale-preview-labeling posture.
    pub stale_preview_labeling: StalePreviewLabelingState,
    /// Hard invariant: a tooltip never carries the sole critical instruction. `false`
    /// is a blocker.
    pub tooltip_never_sole_critical_instruction: bool,
    /// Active waiver, when a disclosed promotion-path reduction is in force.
    pub active_waiver: Option<TransientInspectCertificationWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: TransientInspectCertificationStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<TransientInspectCertificationCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl TransientInspectCertificationRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every representation class the matrix freezes is certified — the
    /// lint that prevents a transient surface from shipping without its full
    /// tooltip/hovercard/peek/pinned/provenance/truncated representation vocabulary.
    pub fn representation_classes_complete(&self) -> bool {
        let present: BTreeSet<M5RepresentationClass> = self
            .certified_representation_classes
            .iter()
            .copied()
            .collect();
        M5RepresentationClass::ALL
            .iter()
            .all(|class| present.contains(class))
    }

    /// `true` when every promotion state the matrix freezes is certified — the lint
    /// that prevents a peek from shipping without a full transient → pinned →
    /// promoted → detached → demoted → dismissed transition set.
    pub fn promotion_states_complete(&self) -> bool {
        let present: BTreeSet<M5PromotionState> =
            self.certified_promotion_states.iter().copied().collect();
        M5PromotionState::ALL
            .iter()
            .all(|state| present.contains(state))
    }

    /// `true` when every required label the matrix freezes is certified — the lint
    /// that prevents a transient surface from shipping without identity, state,
    /// keyboard-route, source/provider, freshness, and reopen-path labels.
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5PrimitiveRequiredLabel::ALL
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the stale/cached freshness labels are certified — the lint that
    /// prevents a stale or cached preview from reading as live canonical content.
    pub fn stale_labels_present(&self) -> bool {
        let present: BTreeSet<M5SourceFreshnessLabel> =
            self.source_freshness_labels.iter().copied().collect();
        [
            M5SourceFreshnessLabel::CachedSnapshot,
            M5SourceFreshnessLabel::StaleInvalidated,
        ]
        .iter()
        .all(|label| present.contains(label))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.representation_truth.is_blocked()
            || self.promotion_continuity.is_blocked()
            || self.non_hover_reach.is_blocked()
            || self.stale_preview_labeling.is_blocked()
            || !self.tooltip_never_sole_critical_instruction
            || !self.representation_classes_complete()
            || !self.promotion_states_complete()
            || !self.required_labels_complete()
            || !self.stale_labels_present()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.representation_truth.is_disclosed()
            || self.promotion_continuity.is_disclosed()
            || self.non_hover_reach.is_disclosed()
            || self.stale_preview_labeling.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the tooltip invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> TransientInspectCertificationStatus {
        if self.has_hard_blocker() {
            TransientInspectCertificationStatus::Red
        } else if self.has_narrowing() {
            TransientInspectCertificationStatus::Yellow
        } else {
            TransientInspectCertificationStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order
    /// (representation, promotion, non-hover reach, stale labeling, tooltip).
    pub fn recompute_causes(&self) -> Vec<TransientInspectCertificationCause> {
        let mut causes = Vec::new();
        if !self.representation_truth.is_labeled() {
            causes.push(TransientInspectCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::SourceFreshnessHidden,
                disclosed: self.representation_truth.is_disclosed(),
                detail: if self.representation_truth.is_disclosed() {
                    "Under compact width the hovercard falls back to a disclosed, shorter \
                     representation while the target identity, source/provider class, and freshness \
                     stay labeled; the reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A transient inspect surface hides its source/provider or freshness truth, so a \
                     cached or stale value can read as live canonical content."
                        .to_owned()
                },
            });
        }
        if !self.promotion_continuity.is_preserved() {
            causes.push(TransientInspectCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::PromotionDroppedTruth,
                disclosed: self.promotion_continuity.is_disclosed(),
                detail: if self.promotion_continuity.is_disclosed() {
                    "One promotion path (detach-to-window) is disclosedly deferred while pin and \
                     open still preserve the target identity, state, and representation truth; the \
                     reduction is disclosed and waivered, and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A promotion or pin dropped the target's identity or representation truth, or a \
                     peek promoted without disclosing its preview-only versus live-editable posture."
                        .to_owned()
                },
            });
        }
        if !self.non_hover_reach.is_reachable() {
            causes.push(TransientInspectCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: self.non_hover_reach.is_disclosed(),
                detail: if self.non_hover_reach.is_disclosed() {
                    "One non-hover reach route is temporarily reduced; at least one route (keyboard \
                     focus, context action, or info affordance) still resolves and the reduction is \
                     disclosed."
                        .to_owned()
                } else {
                    "A transient inspect surface keeps its information reachable only through \
                     pointer hover or pointer interaction, with no keyboard-focus, context-action, \
                     or info-affordance route."
                        .to_owned()
                },
            });
        }
        if !self.stale_preview_labeling.is_labeled() {
            causes.push(TransientInspectCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::StalePreviewMistakenForLive,
                disclosed: self.stale_preview_labeling.is_disclosed(),
                detail: if self.stale_preview_labeling.is_disclosed() {
                    "The support export reconstructs the visible preview and discloses a partial \
                     capture of the promoted / pinned set while a refresh is in flight; the partial \
                     capture is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A stale, cached, or approximate preview reads as live canonical content, or a \
                     pinned preview is absent from the support-export capture."
                        .to_owned()
                },
            });
        }
        if !self.tooltip_never_sole_critical_instruction {
            causes.push(TransientInspectCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: false,
                detail: "A tooltip carries the sole critical instruction for an action, so the \
                         instruction is reachable only through pointer hover."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced promotion path may only stay yellow (rather than red) when
    /// a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.promotion_continuity.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<TransientInspectCertificationFinding> {
        let mut findings = Vec::new();
        let context = self.context.as_str().to_owned();

        if self.representation_truth.is_blocked() {
            findings.push(
                TransientInspectCertificationFinding::RepresentationTruthHidden {
                    context: context.clone(),
                },
            );
        }
        if self.promotion_continuity.is_blocked() {
            findings.push(
                TransientInspectCertificationFinding::PromotionDroppedTruth {
                    context: context.clone(),
                },
            );
        }
        if self.non_hover_reach.is_blocked() {
            findings.push(TransientInspectCertificationFinding::InformationHoverOnly {
                context: context.clone(),
            });
        }
        if self.stale_preview_labeling.is_blocked() {
            findings.push(
                TransientInspectCertificationFinding::StalePreviewMistakenForLive {
                    context: context.clone(),
                },
            );
        }
        if !self.tooltip_never_sole_critical_instruction {
            findings.push(
                TransientInspectCertificationFinding::TooltipSoleCriticalInstruction {
                    context: context.clone(),
                },
            );
        }
        if !self.representation_classes_complete() {
            findings.push(
                TransientInspectCertificationFinding::RepresentationClassesIncomplete {
                    context: context.clone(),
                },
            );
        }
        if !self.promotion_states_complete() {
            findings.push(
                TransientInspectCertificationFinding::PromotionStatesIncomplete {
                    context: context.clone(),
                },
            );
        }
        if !self.required_labels_complete() {
            findings.push(
                TransientInspectCertificationFinding::RequiredLabelsIncomplete {
                    context: context.clone(),
                },
            );
        }
        if !self.stale_labels_present() {
            findings.push(TransientInspectCertificationFinding::StaleLabelsMissing {
                context: context.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, TransientInspectCertificationStatus::Green) && !self.has_reason() {
            findings.push(
                TransientInspectCertificationFinding::NarrowedRowWithoutReason {
                    context: context.clone(),
                },
            );
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry
        // an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(
                TransientInspectCertificationFinding::NarrowedRowWithoutWaiver {
                    context: context.clone(),
                },
            );
        }
        // An attached waiver must still be active and must point at this context.
        if let Some(waiver) = &self.active_waiver {
            if waiver.context != self.context {
                findings.push(
                    TransientInspectCertificationFinding::WaiverContextMismatch {
                        context: context.clone(),
                        waiver_id: waiver.waiver_id.clone(),
                    },
                );
            }
            if !waiver.is_active_at(as_of) {
                findings.push(TransientInspectCertificationFinding::WaiverExpired {
                    context: context.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(TransientInspectCertificationFinding::RowStatusStale {
                context: context.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(TransientInspectCertificationFinding::RowCausesStale { context });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} representation={} promotion={} reach={} stale={} tooltip={} waiver={}",
            self.context.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.representation_truth.as_str(),
            self.promotion_continuity.as_str(),
            self.non_hover_reach.as_str(),
            self.stale_preview_labeling.as_str(),
            self.tooltip_never_sole_critical_instruction,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the transient-inspect certification proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum TransientInspectCertificationFinding {
    /// A governed inspect context has no certification row.
    ContextMissing {
        /// The missing context token.
        context: String,
    },
    /// A context hides source/provider or freshness truth.
    RepresentationTruthHidden {
        /// The context token.
        context: String,
    },
    /// A context's promotion drops the target identity or representation.
    PromotionDroppedTruth {
        /// The context token.
        context: String,
    },
    /// A context keeps information reachable only through hover or pointer.
    InformationHoverOnly {
        /// The context token.
        context: String,
    },
    /// A context's stale preview reads as live or is absent from capture.
    StalePreviewMistakenForLive {
        /// The context token.
        context: String,
    },
    /// A context's tooltip carries the sole critical instruction.
    TooltipSoleCriticalInstruction {
        /// The context token.
        context: String,
    },
    /// A context does not certify every frozen representation class.
    RepresentationClassesIncomplete {
        /// The context token.
        context: String,
    },
    /// A context does not certify every frozen promotion state.
    PromotionStatesIncomplete {
        /// The context token.
        context: String,
    },
    /// A context does not certify every frozen required label.
    RequiredLabelsIncomplete {
        /// The context token.
        context: String,
    },
    /// A context does not certify the stale/cached freshness labels.
    StaleLabelsMissing {
        /// The context token.
        context: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The context token.
        context: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The context token.
        context: String,
    },
    /// An attached waiver does not point at the row's context.
    WaiverContextMismatch {
        /// The context token.
        context: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The context token.
        context: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The context token.
        context: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The context token.
        context: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered contexts do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl TransientInspectCertificationFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ContextMissing { .. } => "context_missing",
            Self::RepresentationTruthHidden { .. } => "representation_truth_hidden",
            Self::PromotionDroppedTruth { .. } => "promotion_dropped_truth",
            Self::InformationHoverOnly { .. } => "information_hover_only",
            Self::StalePreviewMistakenForLive { .. } => "stale_preview_mistaken_for_live",
            Self::TooltipSoleCriticalInstruction { .. } => "tooltip_sole_critical_instruction",
            Self::RepresentationClassesIncomplete { .. } => "representation_classes_incomplete",
            Self::PromotionStatesIncomplete { .. } => "promotion_states_incomplete",
            Self::RequiredLabelsIncomplete { .. } => "required_labels_incomplete",
            Self::StaleLabelsMissing { .. } => "stale_labels_missing",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverContextMismatch { .. } => "waiver_context_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ContextMissing { context }
            | Self::RepresentationTruthHidden { context }
            | Self::PromotionDroppedTruth { context }
            | Self::InformationHoverOnly { context }
            | Self::StalePreviewMistakenForLive { context }
            | Self::TooltipSoleCriticalInstruction { context }
            | Self::RepresentationClassesIncomplete { context }
            | Self::PromotionStatesIncomplete { context }
            | Self::RequiredLabelsIncomplete { context }
            | Self::StaleLabelsMissing { context }
            | Self::NarrowedRowWithoutReason { context }
            | Self::NarrowedRowWithoutWaiver { context }
            | Self::WaiverContextMismatch { context, .. }
            | Self::WaiverExpired { context, .. }
            | Self::RowStatusStale { context }
            | Self::RowCausesStale { context } => context,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / attention router / release
/// automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen shell-primitives matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen shell-primitives matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The certification dimensions every context is certified across.
    pub required_proof_dimensions: Vec<M5TransientInspectProofDimension>,
    /// The representation classes every context must certify.
    pub required_representation_classes: Vec<M5RepresentationClass>,
    /// The promotion states every context must certify.
    pub required_promotion_states: Vec<M5PromotionState>,
    /// The required labels every context must certify.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Per-context certification rows, in canonical order.
    pub rows: Vec<TransientInspectCertificationRow>,
    /// Governed contexts certified, in canonical (sorted) order.
    pub covered_contexts: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<TransientInspectCertificationWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<TransientInspectCertificationCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<TransientInspectCertificationFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow
    /// claimed inspect contexts.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TransientInspectCertificationPacket {
    /// Returns the certification row for `context`, if present.
    pub fn row(&self, context: M5InspectContext) -> Option<&TransientInspectCertificationRow> {
        self.rows.iter().find(|row| row.context == context)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.context.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.context.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the shell automation consumes.
    pub fn dashboard(&self) -> TransientInspectCertificationDashboard {
        TransientInspectCertificationDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 transient-inspect certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per context.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "context,status,qualification,shell_zone_slot,representation_truth,promotion_continuity,non_hover_reach,stale_preview_labeling,tooltip_never_sole_instruction,representation_classes,promotion_states,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.context.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.shell_zone_slot.as_str(),
                row.representation_truth.as_str(),
                row.promotion_continuity.as_str(),
                row.non_hover_reach.as_str(),
                row.stale_preview_labeling.as_str(),
                row.tooltip_never_sole_critical_instruction,
                join_tokens(&row.certified_representation_classes, |c| c.as_str()),
                join_tokens(&row.certified_promotion_states, |s| s.as_str()),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 tooltip, hovercard & peek-panel representation, promotion, reach & stale-labeling\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_transient_inspect_certification`](../../crates/aureline-shell/src/m5_transient_inspect_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification -- markdown > \\\n  artifacts/shell/m5-transient-inspect-certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!("- Green: {}\n", self.green_row_count));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification dimensions\n\n");
        for dimension in &self.required_proof_dimensions {
            out.push_str(&format!("- `{}`\n", dimension.as_str()));
        }
        out.push('\n');

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Context | Status | Qualification | Representation | Promotion | Reach | Stale | Tooltip | Waiver |\n\
             | ------- | ------ | ------------- | -------------- | --------- | ----- | ----- | ------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.context_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.representation_truth.as_str(),
                row.promotion_continuity.as_str(),
                row.non_hover_reach.as_str(),
                row.stale_preview_labeling.as_str(),
                row.tooltip_never_sole_critical_instruction,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&TransientInspectCertificationRow> = self
            .rows
            .iter()
            .filter(|row| {
                !matches!(
                    row.derived_status,
                    TransientInspectCertificationStatus::Green
                )
            })
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed inspect context is certified at full standing.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.context.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact certification causes\n\n");
        if self.certification_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.certification_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.context.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.context.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_transient_inspect_certification_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationDashboardRow {
    /// The governed context.
    pub context: M5InspectContext,
    /// Short context-surface label.
    pub context_label: String,
    /// Derived green/yellow/red status.
    pub status: TransientInspectCertificationStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Representation-truth posture.
    pub representation_truth: RepresentationTruthState,
    /// Promotion-continuity posture.
    pub promotion_continuity: PromotionContinuityState,
    /// Non-hover-reach posture.
    pub non_hover_reach: NonHoverReachState,
    /// Stale-preview-labeling posture.
    pub stale_preview_labeling: StalePreviewLabelingState,
    /// `true` when a tooltip never carries the sole critical instruction.
    pub tooltip_never_sole_critical_instruction: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / attention router / release
/// automation reads to auto-narrow claimed inspect contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<TransientInspectCertificationDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Shell / release automation refs that consume the dashboard.
    pub shell_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TransientInspectCertificationDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &TransientInspectCertificationPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| TransientInspectCertificationDashboardRow {
                context: row.context,
                context_label: row.context_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                representation_truth: row.representation_truth,
                promotion_continuity: row.promotion_continuity,
                non_hover_reach: row.non_hover_reach,
                stale_preview_labeling: row.stale_preview_labeling,
                tooltip_never_sole_critical_instruction: row
                    .tooltip_never_sole_critical_instruction,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .certification_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_TRANSIENT_INSPECT_CERTIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSIENT_INSPECT_CERTIFICATION_SCHEMA_VERSION,
            dashboard_id: M5_TRANSIENT_INSPECT_CERTIFICATION_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            shell_automation_refs: packet.shell_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 transient-inspect certification dashboard serializes")
    }
}

/// Support-export wrapper for the transient-inspect certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransientInspectCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: TransientInspectCertificationPacket,
    /// Dashboard quoted in full.
    pub dashboard: TransientInspectCertificationDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl TransientInspectCertificationSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each context, and
    /// each active waiver id is quoted as a case id so a support reviewer — or the
    /// shell automation — can name the same context and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: TransientInspectCertificationPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.context.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_TRANSIENT_INSPECT_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSIENT_INSPECT_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_transient_inspect_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientInspectCertificationInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-context certification rows.
    pub rows: Vec<TransientInspectCertificationRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
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
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`TransientInspectCertificationPacket`] from the exact build identity, the
/// frozen matrix ref, and the per-context certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the
/// active waivers, and the blocking findings are recomputed here so the packet is the
/// single source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_transient_inspect_certification_packet(
    input: TransientInspectCertificationInput,
) -> TransientInspectCertificationPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<TransientInspectCertificationRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<TransientInspectCertificationFinding> = Vec::new();

    // Every governed context must carry a certification row.
    let present: BTreeSet<M5InspectContext> = rows.iter().map(|row| row.context).collect();
    for context in M5InspectContext::ALL {
        if !present.contains(&context) {
            blocking_findings.push(TransientInspectCertificationFinding::ContextMissing {
                context: context.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_contexts: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|context| context.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.derived_status,
                TransientInspectCertificationStatus::Green
            )
        })
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.derived_status,
                TransientInspectCertificationStatus::Yellow
            )
        })
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TransientInspectCertificationStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(TransientInspectCertificationFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<TransientInspectCertificationWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<TransientInspectCertificationCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = TransientInspectCertificationPacket {
        record_kind: M5_TRANSIENT_INSPECT_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_TRANSIENT_INSPECT_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_TRANSIENT_INSPECT_CERTIFICATION_PACKET_ID.to_owned(),
        source_schema_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Tooltip, hovercard, and peek-panel representation, promotion, keyboard-reach, \
                   and stale-labeling truth certified across every claimed M5 inspect context: \
                   search, docs/help, review/change, editor, data grid, profiler, and operator \
                   each preserve canonical target identity, source/provider class, freshness, and \
                   representation label; pin, open, and promote a peek without dropping identity \
                   or state; keep every glanceable piece of information reachable without hover; \
                   and keep stale, cached, or approximate previews labeled and reconstructable \
                   from a support export — with each row's green/yellow/red claim auto-narrowed \
                   from its representation, promotion, reach, and stale-labeling posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5TransientInspectProofDimension::ALL.to_vec(),
        required_representation_classes: M5RepresentationClass::ALL.to_vec(),
        required_promotion_states: M5PromotionState::ALL.to_vec(),
        required_labels: M5PrimitiveRequiredLabel::ALL.to_vec(),
        rows,
        covered_contexts,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        certification_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.transient_inspect.representation_registry".to_owned(),
            "release_automation.auto_narrow.transient_inspect_certification_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.transient_inspect_certification".to_owned(),
            "artifacts/release/m5-transient-inspect-certification-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-transient-inspect-certification".to_owned()],
        published_report_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_DASHBOARD_REF
            .to_owned(),
        published_doc_ref: M5_TRANSIENT_INSPECT_CERTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(TransientInspectCertificationFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_transient_inspect_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum TransientInspectCertificationValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The rows do not cover all seven governed contexts.
    CoverageIncomplete,
    /// The declared covered contexts do not match the rows.
    CoverageStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required representation classes are not the canonical set.
    RequiredRepresentationClassesStale,
    /// The declared required promotion states are not the canonical set.
    RequiredPromotionStatesStale,
    /// The declared required labels are not the canonical set.
    RequiredLabelsStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared certification causes do not match the recomputed causes.
    CertificationCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the transient-inspect certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// inspect context carries a current certification row; each row's status is the
/// derived auto-narrowed value, never asserted; a green row cannot keep a claim while
/// it hides source/freshness truth, drops a promotion, keeps information hover-only,
/// lets a stale preview read as live, carries a tooltip-only instruction, or leaves
/// its representation classes / promotion states / required labels / stale labels
/// incomplete; and a disclosed narrowing is backed by a reason and, where required, an
/// active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_transient_inspect_certification_packet(
    packet: &TransientInspectCertificationPacket,
) -> Result<(), Vec<TransientInspectCertificationValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(TransientInspectCertificationValidationError::NoRows);
    }
    if packet.record_kind != M5_TRANSIENT_INSPECT_CERTIFICATION_PACKET_RECORD_KIND {
        errors.push(TransientInspectCertificationValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_TRANSIENT_INSPECT_CERTIFICATION_SCHEMA_VERSION {
        errors.push(TransientInspectCertificationValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(TransientInspectCertificationValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(TransientInspectCertificationValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5TransientInspectProofDimension::ALL {
        errors.push(TransientInspectCertificationValidationError::RequiredDimensionsStale);
    }
    if packet.required_representation_classes != M5RepresentationClass::ALL {
        errors
            .push(TransientInspectCertificationValidationError::RequiredRepresentationClassesStale);
    }
    if packet.required_promotion_states != M5PromotionState::ALL {
        errors.push(TransientInspectCertificationValidationError::RequiredPromotionStatesStale);
    }
    if packet.required_labels != M5PrimitiveRequiredLabel::ALL {
        errors.push(TransientInspectCertificationValidationError::RequiredLabelsStale);
    }

    let present: BTreeSet<M5InspectContext> = packet.rows.iter().map(|row| row.context).collect();
    let coverage_complete = M5InspectContext::ALL
        .iter()
        .all(|context| present.contains(context));
    if !coverage_complete || packet.rows.len() != M5InspectContext::ALL.len() {
        errors.push(TransientInspectCertificationValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|context| context.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_contexts {
        errors.push(TransientInspectCertificationValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                TransientInspectCertificationStatus::Green
            )
        })
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                TransientInspectCertificationStatus::Yellow
            )
        })
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                TransientInspectCertificationStatus::Red
            )
        })
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(TransientInspectCertificationValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<TransientInspectCertificationWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(TransientInspectCertificationValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<TransientInspectCertificationCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(TransientInspectCertificationValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<TransientInspectCertificationFinding> = Vec::new();
    for context in M5InspectContext::ALL {
        if !present.contains(&context) {
            recomputed.push(TransientInspectCertificationFinding::ContextMissing {
                context: context.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(TransientInspectCertificationFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(TransientInspectCertificationFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(TransientInspectCertificationValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            TransientInspectCertificationValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(TransientInspectCertificationValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(TransientInspectCertificationValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(TransientInspectCertificationValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(TransientInspectCertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
