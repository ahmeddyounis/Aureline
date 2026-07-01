//! Canonical controlled-lifecycle-vocabulary parity certification across every M5 consumer surface.
//!
//! The [frozen lifecycle matrix][matrix] freezes the controlled M5 lifecycle-state vocabulary — the
//! fifteen states `ready`, `warming`, `partial`, `stale`, `rebuilding`, `restricted`,
//! `policy_blocked`, `reconnecting`, `degraded`, `read_only_degraded`, `unavailable`,
//! `rollback_available`, `deprecated`, `experimental`, and `retest_pending` — as the single shared
//! state language every long-lived M5 object must speak, and it names the consumer surfaces that
//! must project that vocabulary: the product UI, the CLI/headless output, docs/help, diagnostics,
//! support exports, telemetry, claim tooling, and release notes. This lane is the **certification
//! capstone** that keeps that vocabulary *parity* honest: for every controlled state term it
//! certifies that the term **keeps one meaning across every consumer surface**, **stays
//! semantically distinct rather than collapsing into a generic failure**, **exports one stable
//! status code identically on every export path**, and **narrows its published release/docs/help
//! copy automatically when evidence or support state changes** — and that the same state-truth
//! vocabulary survives a headless or companion-adjacent execution rather than degrading into a
//! surface-local loading/error phrasing.
//!
//! Three records carry the truth:
//!
//! - the per-term **parity row** ([`VocabularyParityRow`]): one row per [`M5LifecycleState`] naming
//!   the object families that admit the term (pulled from the matrix), its cross-surface /
//!   semantic-distinction / export-code / published-copy posture, whether the same vocabulary
//!   survives headless/companion-adjacent execution, the consumer surfaces it evaluated, any active
//!   waiver, and a derived green/yellow/red [`VocabularyParityStatus`].
//! - the release **parity packet** ([`VocabularyParityPacket`]): the full set of rows with derived
//!   per-row status, aggregate green/yellow/red counts, the active waivers, the exact term causes
//!   ([`VocabularyParityCause`]), and the blocking findings the lane refuses to ship with.
//! - the **parity dashboard** ([`VocabularyParityDashboard`]): a light projection the product UI /
//!   CLI / diagnostics / support / telemetry automation reads to auto-narrow a controlled term's
//!   published wording when its parity falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a
//! surface paraphrases the controlled term behind a disclosed, waivered friendlier label, groups it
//! under a disclosed family header, exports a disclosed partial status code, or narrows its
//! published copy only through a disclosed manual step; it drops to `red` if the term's meaning
//! drifts across surfaces, it collapses into a generic failure, its status code stops exporting, its
//! published copy stays stale and overclaims after the state changed, the same state-truth
//! vocabulary is lost in a headless/companion-adjacent execution, or the row fails to certify every
//! consumer surface the matrix declares. That derivation is the auto-narrowing the acceptance
//! criteria require, and the consumer-surface completeness check is the lint that prevents a
//! certification from silently regressing into a partial, single-surface view — the exact regression
//! that lets one surface still use legacy or vague wording while the claim publishes as if every
//! surface agreed.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The lifecycle-state, object-family, consumer-surface,
//! and downgrade-trigger vocabulary is re-exported by reference from the already frozen [matrix],
//! and every term binding is pulled straight from that matrix's seeded packet, so this lane mints no
//! parallel lifecycle vocabulary and cannot certify a term the matrix does not freeze. Only the
//! parity-specific vocabulary ([`M5LifecycleParityDimension`], [`VocabularyParityStatus`],
//! [`CrossSurfaceTermState`], [`SemanticDistinctionState`], [`ExportCodeParityState`],
//! [`PublishedCopyNarrowingState`], [`VocabularyParityWaiver`], [`VocabularyParityCause`],
//! [`VocabularyParityFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix as matrix;

pub use matrix::{
    M5LifecycleConsumerSurface, M5LifecycleDowngradeTrigger, M5LifecycleObjectFamily,
    M5LifecycleState,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_lifecycle_vocabulary_parity_packet,
    seeded_m5_lifecycle_vocabulary_parity_packet_deprecated_stale_copy_overclaims_blocked,
    seeded_m5_lifecycle_vocabulary_parity_packet_experimental_headless_parity_lost_blocked,
    seeded_m5_lifecycle_vocabulary_parity_packet_policy_blocked_status_code_unexportable_blocked,
    seeded_m5_lifecycle_vocabulary_parity_packet_reconnecting_term_drift_blocked,
    seeded_m5_lifecycle_vocabulary_parity_packet_retest_pending_generic_collapse_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_SHARED_CONTRACT_REF: &str =
    "lifecycle:m5_lifecycle_vocabulary_parity:v1";

/// Stable record kind for [`VocabularyParityPacket`] payloads.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_vocabulary_parity_packet_record";

/// Stable record kind for [`VocabularyParityDashboard`] payloads.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_vocabulary_parity_dashboard_record";

/// Stable record kind for [`VocabularyParitySupportExport`] payloads.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_vocabulary_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PACKET_ID: &str =
    "m5-lifecycle-vocabulary-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_DASHBOARD_ID: &str =
    "m5-lifecycle-vocabulary-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-lifecycle-vocabulary-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-lifecycle-vocabulary-parity.schema.json";

/// Published markdown report ref reviewers reopen the parity proof from.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-lifecycle-vocabulary-parity.md";

/// Published parity-packet artifact ref.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-lifecycle-vocabulary-parity-proof/packet.json";

/// Published parity-dashboard artifact ref.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-lifecycle-vocabulary-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-lifecycle-vocabulary-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-lifecycle-vocabulary-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_lifecycle_vocabulary_parity_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_MATRIX_DOC_REF: &str = matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for the cross-surface term binding.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the published-copy binding.
pub const M5_LIFECYCLE_VOCABULARY_PARITY_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every controlled lifecycle-state term the certification must cover, in canonical order. These
/// are exactly the terms the frozen lifecycle matrix freezes; a certification that covers fewer
/// regresses into a partial view and blocks.
pub const REQUIRED_STATES: [M5LifecycleState; 15] = M5LifecycleState::ALL;

/// Every parity dimension each term row certifies, in canonical order.
pub const REQUIRED_PARITY_DIMENSIONS: [M5LifecycleParityDimension; 4] =
    M5LifecycleParityDimension::ALL;

/// Canonical consumer-surface ordering used to derive the required-surface set from the matrix.
/// This is only an ordering over the matrix's frozen [`M5LifecycleConsumerSurface`] vocabulary — it
/// mints no new surface term.
const CONSUMER_SURFACE_ORDER: [M5LifecycleConsumerSurface; 8] = [
    M5LifecycleConsumerSurface::ProductUi,
    M5LifecycleConsumerSurface::Cli,
    M5LifecycleConsumerSurface::DocsHelp,
    M5LifecycleConsumerSurface::Diagnostics,
    M5LifecycleConsumerSurface::SupportExport,
    M5LifecycleConsumerSurface::Telemetry,
    M5LifecycleConsumerSurface::ClaimTooling,
    M5LifecycleConsumerSurface::ReleaseNotes,
];

/// The set of consumer surfaces every controlled term must keep parity across — the union of every
/// consumer surface the frozen matrix declares on any governed object family, in canonical order.
///
/// Deriving this from the matrix (rather than restating it) keeps the required-surface set honest:
/// the parity lane cannot certify a term across a surface the matrix never asked an object to
/// project to, and cannot skip one the matrix declared.
pub fn required_consumer_surfaces() -> Vec<M5LifecycleConsumerSurface> {
    let declared: BTreeSet<&'static str> = matrix::seeded_m5_lifecycle_matrix()
        .object_state_rows
        .iter()
        .flat_map(|row| row.consumer_surfaces.iter())
        .map(|surface| surface.as_str())
        .collect();
    CONSUMER_SURFACE_ORDER
        .into_iter()
        .filter(|surface| declared.contains(surface.as_str()))
        .collect()
}

/// One of the four parity dimensions each controlled-term row certifies.
///
/// These are exactly the four ways the acceptance criteria require a controlled lifecycle term to
/// stay honest across surfaces: it keeps one meaning across every surface, it stays semantically
/// distinct from a generic failure, it exports one stable status code identically, and its published
/// release/docs/help copy narrows automatically when evidence or support state changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleParityDimension {
    /// The term keeps one meaning across every consumer surface.
    CrossSurfaceTerm,
    /// The term stays semantically distinct rather than collapsing into a generic failure.
    SemanticDistinction,
    /// The term's status code exports identically on every export path.
    ExportCodeParity,
    /// The term's published release/docs/help copy narrows automatically on a state change.
    PublishedCopyNarrowing,
}

impl M5LifecycleParityDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CrossSurfaceTerm,
        Self::SemanticDistinction,
        Self::ExportCodeParity,
        Self::PublishedCopyNarrowing,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrossSurfaceTerm => "cross_surface_term",
            Self::SemanticDistinction => "semantic_distinction",
            Self::ExportCodeParity => "export_code_parity",
            Self::PublishedCopyNarrowing => "published_copy_narrowing",
        }
    }
}

/// The derived vocabulary-parity light a controlled term carries.
///
/// `green` means the term keeps one meaning across every consumer surface, stays semantically
/// distinct, exports one stable status code identically, and narrows its published copy
/// automatically — and the same state-truth vocabulary survives a headless/companion-adjacent
/// execution across every declared consumer surface. `yellow` is a disclosed narrowing (a waivered
/// surface paraphrase, a disclosed grouped presentation, a disclosed partial status-code export, or
/// a disclosed manual copy narrowing). `red` is blocked: the term's meaning drifted across surfaces,
/// it collapsed into a generic failure, its status code stopped exporting, its published copy went
/// stale and overclaims, the state-truth vocabulary is lost in a headless/companion-adjacent
/// execution, or the row did not certify every declared consumer surface — and it may not keep a
/// vocabulary claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VocabularyParityStatus {
    /// Full standing: all four parity dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl VocabularyParityStatus {
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

/// How the controlled term keeps one meaning across every consumer surface.
///
/// `term_stable_across_all_surfaces` means the UI badge, CLI/headless output, docs/help,
/// diagnostics, support export, telemetry, claim tooling, and release notes all use the term with
/// the same meaning. `disclosed_surface_paraphrase` means one surface presents a disclosed,
/// waivered friendlier label that still maps to the same controlled token (for example release
/// notes phrasing `experimental` as an "early access" label bound to the same state) — a yellow
/// narrowing. `term_meaning_drifted_across_surfaces` means the same term means different things on
/// different surfaces — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossSurfaceTermState {
    /// Every consumer surface uses the term with the same meaning.
    TermStableAcrossAllSurfaces,
    /// One surface uses a disclosed, waivered paraphrase mapped to the same token.
    DisclosedSurfaceParaphrase,
    /// The term means different things on different surfaces — a blocker.
    TermMeaningDriftedAcrossSurfaces,
}

impl CrossSurfaceTermState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TermStableAcrossAllSurfaces => "term_stable_across_all_surfaces",
            Self::DisclosedSurfaceParaphrase => "disclosed_surface_paraphrase",
            Self::TermMeaningDriftedAcrossSurfaces => "term_meaning_drifted_across_surfaces",
        }
    }

    /// `true` when the term is stable across every surface at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::TermStableAcrossAllSurfaces)
    }

    /// `true` when the term took a disclosed surface-paraphrase narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedSurfaceParaphrase)
    }
}

/// How the controlled term stays semantically distinct rather than collapsing into a generic
/// failure.
///
/// `distinct_meaning_preserved` means the term stays individually named and distinguishable from
/// its siblings and from a generic "failed" — so `retest_pending`, `experimental`, `policy_blocked`,
/// and `read_only_degraded` never read as the same anonymous error. `disclosed_grouped_presentation`
/// means a compact surface groups the term under a disclosed family header (for example
/// `read_only_degraded` shown under a "Degraded" group) while still naming it individually — a
/// yellow narrowing. `collapsed_into_generic_failure` means the term lost its distinct meaning and
/// reads as a generic failure — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticDistinctionState {
    /// The term stays individually named and distinct from a generic failure.
    DistinctMeaningPreserved,
    /// A compact surface groups the term under a disclosed family header.
    DisclosedGroupedPresentation,
    /// The term collapsed into a generic failure — a blocker.
    CollapsedIntoGenericFailure,
}

impl SemanticDistinctionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DistinctMeaningPreserved => "distinct_meaning_preserved",
            Self::DisclosedGroupedPresentation => "disclosed_grouped_presentation",
            Self::CollapsedIntoGenericFailure => "collapsed_into_generic_failure",
        }
    }

    /// `true` when the term stays distinct at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::DistinctMeaningPreserved)
    }

    /// `true` when the term took a disclosed grouped-presentation narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedGroupedPresentation)
    }
}

/// How the controlled term exports one stable status code identically on every export path.
///
/// `code_exports_identically_all_paths` means the term's status code exports identically to the CLI,
/// diagnostics, support export, telemetry, and claim tooling. `disclosed_partial_export` means the
/// status code exports in a disclosed reduced form on a subset of surfaces (for example telemetry
/// exporting a coarse policy code until finalized) while still naming the same controlled state — a
/// yellow narrowing. `status_code_unexportable` means the status code stopped exporting on an export
/// path — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportCodeParityState {
    /// The status code exports identically across every export path.
    CodeExportsIdenticallyAllPaths,
    /// The status code exports in a disclosed reduced form on a subset of surfaces.
    DisclosedPartialExport,
    /// The status code stopped exporting on an export path — a blocker.
    StatusCodeUnexportable,
}

impl ExportCodeParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeExportsIdenticallyAllPaths => "code_exports_identically_all_paths",
            Self::DisclosedPartialExport => "disclosed_partial_export",
            Self::StatusCodeUnexportable => "status_code_unexportable",
        }
    }

    /// `true` when the status code exports identically at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CodeExportsIdenticallyAllPaths)
    }

    /// `true` when the term took a disclosed partial-export narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialExport)
    }
}

/// How the term's published release/docs/help copy narrows automatically when evidence or support
/// state changes.
///
/// `copy_auto_narrows_on_state_change` means published copy re-narrows automatically the moment the
/// evidence or support state changes, so release notes and help never overclaim. `disclosed_manual_
/// narrowing` means the copy still narrows but only through a disclosed manual publish step rather
/// than automatically — a yellow narrowing. `stale_copy_overclaims` means the published copy stayed
/// stale and overclaims after the state changed — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishedCopyNarrowingState {
    /// Published copy re-narrows automatically on a state change.
    CopyAutoNarrowsOnStateChange,
    /// Published copy narrows only through a disclosed manual publish step.
    DisclosedManualNarrowing,
    /// Published copy stayed stale and overclaims — a blocker.
    StaleCopyOverclaims,
}

impl PublishedCopyNarrowingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyAutoNarrowsOnStateChange => "copy_auto_narrows_on_state_change",
            Self::DisclosedManualNarrowing => "disclosed_manual_narrowing",
            Self::StaleCopyOverclaims => "stale_copy_overclaims",
        }
    }

    /// `true` when published copy auto-narrows at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CopyAutoNarrowsOnStateChange)
    }

    /// `true` when the term took a disclosed manual-narrowing narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedManualNarrowing)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow)
/// rather than blocked — never lets a term-meaning drift, a semantic collapse, an unexportable
/// status code, or a stale overclaiming copy hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The controlled term the waiver applies to.
    pub state: M5LifecycleState,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl VocabularyParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a controlled term's parity certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityCause {
    /// The controlled term the cause applies to.
    pub state: M5LifecycleState,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is
    /// a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl VocabularyParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One controlled lifecycle-state term, certified across its cross-surface, semantic-distinction,
/// export-code, and published-copy parity dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityRow {
    /// The controlled term being certified.
    pub state: M5LifecycleState,
    /// Short reviewer-facing term label.
    pub state_label: String,
    /// Object families whose explicit state machine admits this term. Pulled from the matrix.
    pub admitting_object_families: Vec<M5LifecycleObjectFamily>,
    /// Consumer surfaces the matrix declares this term must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Cross-surface term posture.
    pub cross_surface_term: CrossSurfaceTermState,
    /// Semantic-distinction posture.
    pub semantic_distinction: SemanticDistinctionState,
    /// Export-code parity posture.
    pub export_code_parity: ExportCodeParityState,
    /// Published-copy narrowing posture.
    pub published_copy_narrowing: PublishedCopyNarrowingState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to this term. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed surface paraphrase is in force.
    pub active_waiver: Option<VocabularyParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: VocabularyParityStatus,
    /// The exact term causes that narrowed or blocked this row.
    pub term_causes: Vec<VocabularyParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl VocabularyParityRow {
    /// `true` when the row certified every consumer surface the matrix declares — no declared
    /// surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.cross_surface_term,
            CrossSurfaceTermState::TermMeaningDriftedAcrossSurfaces
        ) {
            return true;
        }
        if matches!(
            self.semantic_distinction,
            SemanticDistinctionState::CollapsedIntoGenericFailure
        ) {
            return true;
        }
        if matches!(
            self.export_code_parity,
            ExportCodeParityState::StatusCodeUnexportable
        ) {
            return true;
        }
        if matches!(
            self.published_copy_narrowing,
            PublishedCopyNarrowingState::StaleCopyOverclaims
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.cross_surface_term.is_disclosed_narrowing()
            || self.semantic_distinction.is_disclosed_narrowing()
            || self.export_code_parity.is_disclosed_narrowing()
            || self.published_copy_narrowing.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the term posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> VocabularyParityStatus {
        if self.has_hard_blocker() {
            VocabularyParityStatus::Red
        } else if self.has_narrowing() {
            VocabularyParityStatus::Yellow
        } else {
            VocabularyParityStatus::Green
        }
    }

    /// Recomputes the exact term causes for the row, in deterministic order (cross-surface,
    /// semantic-distinction, export-code, published-copy, then headless parity).
    pub fn recompute_causes(&self) -> Vec<VocabularyParityCause> {
        let mut causes = Vec::new();
        match self.cross_surface_term {
            CrossSurfaceTermState::TermStableAcrossAllSurfaces => {}
            CrossSurfaceTermState::DisclosedSurfaceParaphrase => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "One consumer surface presents a disclosed, waivered friendlier label \
                             for this controlled term while still mapping it to the same status \
                             token, so the wording is narrowed and disclosed rather than drifting \
                             into a private synonym."
                        .to_owned(),
                });
            }
            CrossSurfaceTermState::TermMeaningDriftedAcrossSurfaces => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "The controlled term means different things on different consumer \
                             surfaces, so users, support, and automation read a different state \
                             depending on which surface they look at."
                        .to_owned(),
                });
            }
        }
        match self.semantic_distinction {
            SemanticDistinctionState::DistinctMeaningPreserved => {}
            SemanticDistinctionState::DisclosedGroupedPresentation => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A compact consumer surface groups this controlled term under a \
                             disclosed family header while still naming it individually, so the \
                             presentation is narrowed and disclosed rather than collapsing the term \
                             into a generic failure."
                        .to_owned(),
                });
            }
            SemanticDistinctionState::CollapsedIntoGenericFailure => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "The controlled term collapsed into a generic failure wording, so a \
                             blocked, partial, cached, retest-pending, or reconnecting state can no \
                             longer be told apart from an ordinary error."
                        .to_owned(),
                });
            }
        }
        match self.export_code_parity {
            ExportCodeParityState::CodeExportsIdenticallyAllPaths => {}
            ExportCodeParityState::DisclosedPartialExport => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The controlled term's status code exports in a disclosed reduced form \
                             on a subset of surfaces while still naming the same controlled state, so \
                             the export is narrowed and disclosed rather than lost."
                        .to_owned(),
                });
            }
            ExportCodeParityState::StatusCodeUnexportable => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::StatusCodeUnexportable,
                    disclosed: false,
                    detail: "The controlled term's stable status code stopped exporting on an export \
                             path, so support, CLI, or telemetry can no longer read the same code the \
                             UI shows."
                        .to_owned(),
                });
            }
        }
        match self.published_copy_narrowing {
            PublishedCopyNarrowingState::CopyAutoNarrowsOnStateChange => {}
            PublishedCopyNarrowingState::DisclosedManualNarrowing => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The controlled term's published release/docs/help copy narrows only \
                             through a disclosed manual publish step rather than automatically, so \
                             the copy is narrowed and disclosed rather than overclaiming."
                        .to_owned(),
                });
            }
            PublishedCopyNarrowingState::StaleCopyOverclaims => {
                causes.push(VocabularyParityCause {
                    state: self.state,
                    trigger: M5LifecycleDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail:
                        "The controlled term's published release/docs/help copy stayed stale and \
                             overclaims after the evidence or support state changed, so published \
                             wording promises more than the current state supports."
                            .to_owned(),
                });
            }
        }
        if !self.headless_parity_preserved {
            causes.push(VocabularyParityCause {
                state: self.state,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail: "A headless or companion-adjacent execution lost the shared state-truth \
                         vocabulary for this controlled term, so the same state reports a different \
                         state language depending on how it runs."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed surface paraphrase may only stay yellow (rather than red) when a waiver discloses
    /// it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.cross_surface_term,
            CrossSurfaceTermState::DisclosedSurfaceParaphrase
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<VocabularyParityFinding> {
        let mut findings = Vec::new();
        let term = self.state.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings
                .push(VocabularyParityFinding::ConsumerSurfacesIncomplete { term: term.clone() });
        }
        if !self.headless_parity_preserved {
            findings.push(VocabularyParityFinding::HeadlessParityLost { term: term.clone() });
        }
        if matches!(
            self.cross_surface_term,
            CrossSurfaceTermState::TermMeaningDriftedAcrossSurfaces
        ) {
            findings.push(VocabularyParityFinding::TermMeaningDrifted { term: term.clone() });
        }
        if matches!(
            self.semantic_distinction,
            SemanticDistinctionState::CollapsedIntoGenericFailure
        ) {
            findings
                .push(VocabularyParityFinding::CollapsedIntoGenericFailure { term: term.clone() });
        }
        if matches!(
            self.export_code_parity,
            ExportCodeParityState::StatusCodeUnexportable
        ) {
            findings.push(VocabularyParityFinding::StatusCodeUnexportable { term: term.clone() });
        }
        if matches!(
            self.published_copy_narrowing,
            PublishedCopyNarrowingState::StaleCopyOverclaims
        ) {
            findings.push(VocabularyParityFinding::StaleCopyOverclaims { term: term.clone() });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, VocabularyParityStatus::Green) && !self.has_reason() {
            findings.push(VocabularyParityFinding::NarrowedRowWithoutReason { term: term.clone() });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(VocabularyParityFinding::NarrowedRowWithoutWaiver { term: term.clone() });
        }
        // An attached waiver must still be active and must point at this term.
        if let Some(waiver) = &self.active_waiver {
            if waiver.state != self.state {
                findings.push(VocabularyParityFinding::WaiverStateMismatch {
                    term: term.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(VocabularyParityFinding::WaiverExpired {
                    term: term.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(VocabularyParityFinding::RowStatusStale { term: term.clone() });
        }
        if self.term_causes != self.recompute_causes() {
            findings.push(VocabularyParityFinding::RowCausesStale { term });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} cross={} distinct={} export={} copy={} headless={} surfaces={} waiver={}",
            self.state.as_str(),
            self.derived_status.as_str(),
            self.cross_surface_term.as_str(),
            self.semantic_distinction.as_str(),
            self.export_code_parity.as_str(),
            self.published_copy_narrowing.as_str(),
            self.headless_parity_preserved,
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the vocabulary-parity certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum VocabularyParityFinding {
    /// A controlled term has no certification row.
    TermMissing {
        /// The missing term token.
        term: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The term token.
        term: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The term token.
        term: String,
    },
    /// The term's meaning drifted across consumer surfaces.
    TermMeaningDrifted {
        /// The term token.
        term: String,
    },
    /// The term collapsed into a generic failure wording.
    CollapsedIntoGenericFailure {
        /// The term token.
        term: String,
    },
    /// The term's stable status code stopped exporting.
    StatusCodeUnexportable {
        /// The term token.
        term: String,
    },
    /// The term's published copy stayed stale and overclaims.
    StaleCopyOverclaims {
        /// The term token.
        term: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The term token.
        term: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The term token.
        term: String,
    },
    /// An attached waiver does not point at the row's term.
    WaiverStateMismatch {
        /// The term token.
        term: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The term token.
        term: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The term token.
        term: String,
    },
    /// The declared term causes do not match the recomputed causes.
    RowCausesStale {
        /// The term token.
        term: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered terms do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl VocabularyParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::TermMissing { .. } => "term_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::TermMeaningDrifted { .. } => "term_meaning_drifted",
            Self::CollapsedIntoGenericFailure { .. } => "collapsed_into_generic_failure",
            Self::StatusCodeUnexportable { .. } => "status_code_unexportable",
            Self::StaleCopyOverclaims { .. } => "stale_copy_overclaims",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverStateMismatch { .. } => "waiver_state_mismatch",
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
            Self::TermMissing { term }
            | Self::ConsumerSurfacesIncomplete { term }
            | Self::HeadlessParityLost { term }
            | Self::TermMeaningDrifted { term }
            | Self::CollapsedIntoGenericFailure { term }
            | Self::StatusCodeUnexportable { term }
            | Self::StaleCopyOverclaims { term }
            | Self::NarrowedRowWithoutReason { term }
            | Self::NarrowedRowWithoutWaiver { term }
            | Self::WaiverStateMismatch { term, .. }
            | Self::WaiverExpired { term, .. }
            | Self::RowStatusStale { term }
            | Self::RowCausesStale { term } => term,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release vocabulary-parity packet shared by the product UI / CLI / diagnostics / support /
/// telemetry automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityPacket {
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
    /// The frozen lifecycle matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen lifecycle object-state schema.
    pub object_state_schema_ref: String,
    /// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
    pub journey_checkpoint_schema_ref: String,
    /// Frozen lifecycle-matrix contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// State-object inventory this proof mirrors for the cross-surface term binding.
    pub state_object_inventory_ref: String,
    /// State-class recovery reference this proof mirrors for the published-copy binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four parity dimensions every term row certifies.
    pub required_parity_dimensions: Vec<String>,
    /// The fifteen controlled terms the certification must cover.
    pub required_states: Vec<String>,
    /// The consumer surfaces every term must keep parity across.
    pub required_consumer_surfaces: Vec<String>,
    /// Per-term parity rows, in canonical order.
    pub rows: Vec<VocabularyParityRow>,
    /// Controlled terms certified, in canonical (sorted) order.
    pub covered_states: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-parity) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<VocabularyParityWaiver>,
    /// Every exact term cause, in row then cause order.
    pub term_causes: Vec<VocabularyParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<VocabularyParityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Lifecycle / release automation refs that consume this packet to auto-narrow published copy.
    pub lifecycle_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published parity-packet ref.
    pub published_packet_ref: String,
    /// Published parity-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl VocabularyParityPacket {
    /// Returns the parity row for `state`, if present.
    pub fn row(&self, state: M5LifecycleState) -> Option<&VocabularyParityRow> {
        self.rows.iter().find(|row| row.state == state)
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
                waiver.state.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.term_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.state.as_str(),
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

    /// Projects the light parity dashboard the lifecycle automation consumes.
    pub fn dashboard(&self) -> VocabularyParityDashboard {
        VocabularyParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 lifecycle-vocabulary-parity packet serializes")
    }

    /// Deterministic, machine-readable parity CSV: one row per controlled term naming its status,
    /// the four parity postures, headless parity, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "state,status,cross_surface_term,semantic_distinction,export_code_parity,published_copy_narrowing,headless_parity,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.state.as_str(),
                row.derived_status.as_str(),
                row.cross_surface_term.as_str(),
                row.semantic_distinction.as_str(),
                row.export_code_parity.as_str(),
                row.published_copy_narrowing.as_str(),
                row.headless_parity_preserved,
                row.evaluated_consumer_surfaces.len(),
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
            "# M5 lifecycle-vocabulary parity: controlled state terms kept semantically stable across UI, CLI, docs/help, support exports, telemetry, and claim publication\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_lifecycle_vocabulary_parity`](../../crates/aureline-shell/src/m5_lifecycle_vocabulary_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity -- markdown > \\\n  artifacts/lifecycle/m5-lifecycle-vocabulary-parity.md\n",
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
        out.push_str(&format!(
            "- Required parity dimensions: {}\n",
            self.required_parity_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Consumer surfaces certified: {}\n",
            self.required_consumer_surfaces
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Controlled terms certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full parity): {}\n",
            self.green_row_count
        ));
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

        out.push_str("## Parity rows\n\n");
        out.push_str(
            "| Controlled term | Status | Cross-surface | Semantic distinction | Export code | Published copy | Headless | Waiver |\n\
             | --------------- | ------ | ------------- | -------------------- | ----------- | -------------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.state_label,
                row.derived_status.as_str(),
                row.cross_surface_term.as_str(),
                row.semantic_distinction.as_str(),
                row.export_code_parity.as_str(),
                row.published_copy_narrowing.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&VocabularyParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, VocabularyParityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every controlled lifecycle term keeps one meaning across UI, CLI, docs/help, diagnostics, support exports, telemetry, claim tooling, and release notes, stays semantically distinct, exports one stable status code, and narrows its published copy automatically.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.state.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact term causes\n\n");
        if self.term_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.term_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.state.as_str(),
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
                    waiver.state.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_lifecycle_vocabulary_parity_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light parity dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityDashboardRow {
    /// The controlled term.
    pub state: M5LifecycleState,
    /// Short term label.
    pub state_label: String,
    /// Derived green/yellow/red status.
    pub status: VocabularyParityStatus,
    /// Number of declared consumer surfaces certified for this term.
    pub evaluated_surface_count: usize,
    /// Cross-surface term posture.
    pub cross_surface_term: CrossSurfaceTermState,
    /// Semantic-distinction posture.
    pub semantic_distinction: SemanticDistinctionState,
    /// Export-code parity posture.
    pub export_code_parity: ExportCodeParityState,
    /// Published-copy narrowing posture.
    pub published_copy_narrowing: PublishedCopyNarrowingState,
    /// `true` when headless/companion-adjacent parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light parity dashboard the product UI / CLI / diagnostics / support / telemetry automation
/// reads to auto-narrow a controlled term's published wording.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityDashboard {
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
    pub rows: Vec<VocabularyParityDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Lifecycle / release automation refs that consume the dashboard.
    pub lifecycle_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl VocabularyParityDashboard {
    /// Projects the dashboard from a parity packet.
    pub fn from_packet(packet: &VocabularyParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| VocabularyParityDashboardRow {
                state: row.state,
                state_label: row.state_label.clone(),
                status: row.derived_status,
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                cross_surface_term: row.cross_surface_term,
                semantic_distinction: row.semantic_distinction,
                export_code_parity: row.export_code_parity,
                published_copy_narrowing: row.published_copy_narrowing,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .term_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_LIFECYCLE_VOCABULARY_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_VOCABULARY_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_LIFECYCLE_VOCABULARY_PARITY_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            lifecycle_automation_refs: packet.lifecycle_automation_refs.clone(),
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
            .expect("m5 lifecycle-vocabulary-parity dashboard serializes")
    }
}

/// Support-export wrapper for the vocabulary-parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: VocabularyParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: VocabularyParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl VocabularyParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each controlled term, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the lifecycle automation —
    /// can name the same term and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: VocabularyParityPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.state.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_LIFECYCLE_VOCABULARY_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_VOCABULARY_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_LIFECYCLE_VOCABULARY_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_lifecycle_vocabulary_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VocabularyParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-term parity rows.
    pub rows: Vec<VocabularyParityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The parity packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
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

/// Builds a [`VocabularyParityPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-term parity rows.
///
/// Each row's derived status and term causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_lifecycle_vocabulary_parity_packet(
    input: VocabularyParityInput,
) -> VocabularyParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<VocabularyParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.term_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<VocabularyParityFinding> = Vec::new();

    // Every controlled term must carry a parity row.
    let present: BTreeSet<M5LifecycleState> = rows.iter().map(|row| row.state).collect();
    for state in REQUIRED_STATES {
        if !present.contains(&state) {
            blocking_findings.push(VocabularyParityFinding::TermMissing {
                term: state.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_states: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, VocabularyParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, VocabularyParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, VocabularyParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(VocabularyParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<VocabularyParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let term_causes: Vec<VocabularyParityCause> = rows
        .iter()
        .flat_map(|row| row.term_causes.clone())
        .collect();

    let required_parity_dimensions: Vec<String> = REQUIRED_PARITY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_states: Vec<String> = REQUIRED_STATES
        .iter()
        .map(|state| state.as_str().to_owned())
        .collect();
    let required_consumer_surfaces: Vec<String> = required_consumer_surfaces()
        .iter()
        .map(|surface| surface.as_str().to_owned())
        .collect();

    let mut packet = VocabularyParityPacket {
        record_kind: M5_LIFECYCLE_VOCABULARY_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_LIFECYCLE_VOCABULARY_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_LIFECYCLE_VOCABULARY_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_LIFECYCLE_VOCABULARY_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_LIFECYCLE_VOCABULARY_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Controlled lifecycle-vocabulary parity on every M5 consumer surface: each of the \
                   fifteen controlled state terms — ready, warming, partial, stale, rebuilding, \
                   restricted, policy_blocked, reconnecting, degraded, read_only_degraded, \
                   unavailable, rollback_available, deprecated, experimental, and retest_pending — \
                   certified so the term keeps one meaning across the product UI, CLI/headless \
                   output, docs/help, diagnostics, support exports, telemetry, claim tooling, and \
                   release notes, stays semantically distinct rather than collapsing into a generic \
                   failure, exports one stable status code identically, and narrows its published \
                   copy automatically when evidence or support state changes — with the same \
                   state-truth vocabulary preserved in headless and companion-adjacent execution — \
                   and each term's green/yellow/red claim auto-narrowed from its four parity \
                   postures."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_LIFECYCLE_VOCABULARY_PARITY_OBJECT_STATE_SCHEMA_REF.to_owned(),
        journey_checkpoint_schema_ref: M5_LIFECYCLE_VOCABULARY_PARITY_JOURNEY_CHECKPOINT_SCHEMA_REF
            .to_owned(),
        matrix_doc_ref: M5_LIFECYCLE_VOCABULARY_PARITY_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_LIFECYCLE_VOCABULARY_PARITY_STATE_OBJECT_INVENTORY_REF
            .to_owned(),
        state_class_recovery_ref: M5_LIFECYCLE_VOCABULARY_PARITY_STATE_CLASS_RECOVERY_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_parity_dimensions,
        required_states,
        required_consumer_surfaces,
        rows,
        covered_states,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        term_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.vocabulary_parity_registry".to_owned(),
            "release_automation.auto_narrow.lifecycle_vocabulary_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.lifecycle_vocabulary_parity".to_owned(),
            M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-lifecycle-vocabulary-parity".to_owned()],
        published_report_ref: M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_LIFECYCLE_VOCABULARY_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("parity packet serializes"),
    ) {
        blocking_findings.push(VocabularyParityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_lifecycle_vocabulary_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum VocabularyParityValidationError {
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
    /// The declared required parity dimensions do not match the lane constants.
    RequiredParityDimensionsStale,
    /// The declared required states do not match the lane constants.
    RequiredStatesStale,
    /// The declared required consumer surfaces do not match the matrix-derived set.
    RequiredConsumerSurfacesStale,
    /// The rows do not cover all fifteen controlled terms.
    CoverageIncomplete,
    /// The declared covered terms do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared term causes do not match the recomputed causes.
    TermCausesStale,
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

/// Validates a packet against the vocabulary-parity invariants.
///
/// The checks encode the track invariant and acceptance criteria: every controlled term carries a
/// current parity row; each row's status is the derived auto-narrowed value, never asserted; a green
/// row cannot keep a claim while a term's meaning drifts across surfaces, the term collapses into a
/// generic failure, its status code stops exporting, its published copy stays stale and overclaims,
/// headless/companion-adjacent parity is lost, or the row fails to certify every declared consumer
/// surface; and a disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_lifecycle_vocabulary_parity_packet(
    packet: &VocabularyParityPacket,
) -> Result<(), Vec<VocabularyParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(VocabularyParityValidationError::NoRows);
    }
    if packet.record_kind != M5_LIFECYCLE_VOCABULARY_PARITY_PACKET_RECORD_KIND {
        errors.push(VocabularyParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_LIFECYCLE_VOCABULARY_PARITY_SCHEMA_VERSION {
        errors.push(VocabularyParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(VocabularyParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(VocabularyParityValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_PARITY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_parity_dimensions != expected_dimensions {
        errors.push(VocabularyParityValidationError::RequiredParityDimensionsStale);
    }
    let expected_states: Vec<String> = REQUIRED_STATES
        .iter()
        .map(|state| state.as_str().to_owned())
        .collect();
    if packet.required_states != expected_states {
        errors.push(VocabularyParityValidationError::RequiredStatesStale);
    }
    let expected_surfaces: Vec<String> = required_consumer_surfaces()
        .iter()
        .map(|surface| surface.as_str().to_owned())
        .collect();
    if packet.required_consumer_surfaces != expected_surfaces {
        errors.push(VocabularyParityValidationError::RequiredConsumerSurfacesStale);
    }

    let present: BTreeSet<M5LifecycleState> = packet.rows.iter().map(|row| row.state).collect();
    let coverage_complete = REQUIRED_STATES.iter().all(|state| present.contains(state));
    if !coverage_complete || packet.rows.len() != REQUIRED_STATES.len() {
        errors.push(VocabularyParityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_states {
        errors.push(VocabularyParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), VocabularyParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), VocabularyParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), VocabularyParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(VocabularyParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<VocabularyParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(VocabularyParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<VocabularyParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.term_causes {
        errors.push(VocabularyParityValidationError::TermCausesStale);
    }

    let mut recomputed: Vec<VocabularyParityFinding> = Vec::new();
    for state in REQUIRED_STATES {
        if !present.contains(&state) {
            recomputed.push(VocabularyParityFinding::TermMissing {
                term: state.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(VocabularyParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("parity packet serializes"),
    ) {
        recomputed.push(VocabularyParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(VocabularyParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(VocabularyParityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(VocabularyParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(VocabularyParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(VocabularyParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(VocabularyParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
