//! Implements the reusable start-center launch-wedge primitive: a start-center
//! bundle card and a certified-archetype badge group that both resolve from one
//! launch-wedge context and share one wedge identity, so start-center, workspace
//! switcher, docs/help, diagnostics, and support surfaces are truthful about what
//! is official, current, or stale *before* a user chooses or adopts a supported
//! stack.
//!
//! Where
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]
//! *freezes* the reusable workflow-bundle component families as a governed
//! contract, this module *narrows* two of those families —
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily::StartCenterBundleCard`]
//! and
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup`]
//! — into one working primitive with a real **resolver**. A single launch-wedge
//! context projects onto two surfaces that share one wedge identity, so the
//! bundle's signer / source, support class, certification state, compatible
//! Aureline range, and archetype evidence age never blur across the start-center
//! card and the certified-archetype badge group.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — certified / approximate / local-only assurance is legible before
//!   install.** The start-center bundle card names the bundle name, persona / stack
//!   tag, support class, certification state, compatible Aureline range, and signer
//!   / source, keeps a `Review bundle` action, and derives an entry-assurance tier
//!   so a user can always tell whether a stack entry is certified, approximate, or
//!   local-only before install or adoption.
//! - **AC2 — archetype badges degrade visibly when evidence ages or scope
//!   narrows.** The certified-archetype badge group carries the archetype id,
//!   evidence age, and supported platform / toolchain envelope, and downgrades to a
//!   `Retest pending` or `Limited` state whenever certification freshness slips or
//!   archetype confidence narrows — never silently reads as fully certified.
//! - **AC3 — stack entry never inherits a hidden marketplace / certification
//!   assumption.** The launch wedge names one shared source class
//!   ([`CertificationTarget`]) explicitly on the card and the badge group, so a
//!   start-center entry never inherits an official-looking claim from backend state
//!   alone.
//!
//! Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider
//! cursors never cross this boundary; the resolver carries only opaque refs, typed
//! class tokens, booleans, and redacted labels, so support and diagnostics exports
//! reconstruct exactly what a surface would have shown without leaking source or
//! live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-start-center-launch-wedge-primitive.schema.json`](../../../../schemas/ui/m5-start-center-launch-wedge-primitive.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_start_center_launch_wedge_primitive.md`](../../../../docs/bundles/m5_start_center_launch_wedge_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the primitive binds to the freeze matrix's
// truth-mode, downgrade-trigger, and degraded-state tokens rather than mint parallel
// ones.
use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    DegradedState, M5BundleComponentDowngradeTrigger, M5BundleTruthMode,
};
// Reused canonical bundle / archetype vocabulary already carried by the frozen
// bundle-manifest, scorecard, and entry-governance contracts.
use crate::m5_bundle_scorecards::{EvidenceFreshness, ImportedVsNativeConfidence};
use crate::m5_entry_and_bundle_governance::{ArchetypeConfidence, BundleClass, SourceTrust};
use crate::m5_workflow_bundle_manifests::{CertificationTarget, LifecycleStage};

/// Stable record-kind tag carried by [`M5StartCenterLaunchWedgePacket`].
pub const M5_START_CENTER_WEDGE_RECORD_KIND: &str = "m5_start_center_launch_wedge_primitive";

/// Schema version for the start-center launch-wedge primitive packet.
pub const M5_START_CENTER_WEDGE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_START_CENTER_WEDGE_SCHEMA_REF: &str =
    "schemas/ui/m5-start-center-launch-wedge-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_START_CENTER_WEDGE_DOC_REF: &str =
    "docs/bundles/m5_start_center_launch_wedge_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive
/// narrows.
pub const M5_START_CENTER_WEDGE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_START_CENTER_WEDGE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-start-center-launch-wedge-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_START_CENTER_WEDGE_ARTIFACT_REF: &str =
    "artifacts/release/m5-start-center-launch-wedge-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_START_CENTER_WEDGE_CSV_REF: &str =
    "artifacts/release/m5-start-center-launch-wedge-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_START_CENTER_WEDGE_REPORT_REF: &str =
    "artifacts/release/m5-start-center-launch-wedge-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed launch-wedge surface family. Each family is one parity surface that
/// ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchWedgeSurfaceFamily {
    /// The start-center bundle card offering a supported stack.
    StartCenterCard,
    /// The workspace switcher row naming the active stack's source class.
    WorkspaceSwitcher,
    /// The bundle-picker list showing candidate stacks side by side.
    BundlePickerList,
    /// The docs / help bundle entry describing a supported stack.
    DocsHelpBundleEntry,
    /// The diagnostics bundle view reconstructing wedge truth.
    DiagnosticsBundleView,
    /// The support / export replay surface reconstructing wedge truth offline.
    SupportExportReplay,
}

impl M5LaunchWedgeSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StartCenterCard,
        Self::WorkspaceSwitcher,
        Self::BundlePickerList,
        Self::DocsHelpBundleEntry,
        Self::DiagnosticsBundleView,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterCard => "start_center_card",
            Self::WorkspaceSwitcher => "workspace_switcher",
            Self::BundlePickerList => "bundle_picker_list",
            Self::DocsHelpBundleEntry => "docs_help_bundle_entry",
            Self::DiagnosticsBundleView => "diagnostics_bundle_view",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StartCenterCard => "Start-center bundle card",
            Self::WorkspaceSwitcher => "Workspace switcher",
            Self::BundlePickerList => "Bundle-picker list",
            Self::DocsHelpBundleEntry => "Docs / help bundle entry",
            Self::DiagnosticsBundleView => "Diagnostics bundle view",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed entry-assurance tier. Names whether a stack entry is certified,
/// approximate, or local-only so a user can tell before install or adoption — the
/// AC1 legibility the start-center card must derive from its source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EntryAssuranceTier {
    /// An official, certified or managed-approved stack entry.
    Certified,
    /// A community-reviewed or imported entry: usable, but approximate.
    Approximate,
    /// A local draft with no external certification claim.
    LocalOnly,
}

impl M5EntryAssuranceTier {
    /// Every assurance tier, in declaration order (strongest to weakest).
    pub const ALL: [Self; 3] = [Self::Certified, Self::Approximate, Self::LocalOnly];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Approximate => "approximate",
            Self::LocalOnly => "local_only",
        }
    }

    /// Derives the assurance tier a start-center card publishes from the shared
    /// source class, so certified / managed entries never blur with community /
    /// imported ones and a local draft never inherits an official tier.
    pub const fn for_source_class(source_class: CertificationTarget) -> Self {
        match source_class {
            CertificationTarget::Certified | CertificationTarget::ManagedApproved => {
                Self::Certified
            }
            CertificationTarget::CommunityReviewed | CertificationTarget::ImportedPendingReview => {
                Self::Approximate
            }
            CertificationTarget::LocalDraft => Self::LocalOnly,
        }
    }
}

/// Closed archetype-badge downgrade state. Names the visible `Retest pending` /
/// `Limited` degrade a certified-archetype badge group shows when evidence ages or
/// scope narrows (AC2), so a stale or narrowed badge never reads as fully current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArchetypeBadgeDowngradeState {
    /// The badge group is current and fully certified.
    None,
    /// The badge group is narrowed: evidence is aging or archetype scope is partial.
    Limited,
    /// The badge group needs a retest: evidence is stale or missing.
    RetestPending,
}

impl M5ArchetypeBadgeDowngradeState {
    /// Every downgrade state, in declaration order (strongest to weakest).
    pub const ALL: [Self; 3] = [Self::None, Self::Limited, Self::RetestPending];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
        }
    }

    /// True when the badge group is degraded below a fully-current certified state.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Derives the visible downgrade a badge group must show from certification
    /// freshness and archetype confidence: stale / missing evidence needs a retest,
    /// aging evidence or an unconfirmed archetype narrows to limited, and only fresh
    /// evidence on a confirmed archetype reads as fully current.
    pub const fn for_evidence(
        freshness: EvidenceFreshness,
        confidence: ArchetypeConfidence,
    ) -> Self {
        match freshness {
            EvidenceFreshness::Stale | EvidenceFreshness::Missing => Self::RetestPending,
            EvidenceFreshness::Aging => Self::Limited,
            EvidenceFreshness::Fresh => match confidence {
                ArchetypeConfidence::Confirmed => Self::None,
                _ => Self::Limited,
            },
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchWedgeExportField {
    /// The stable wedge identity shared across surfaces.
    WedgeId,
    /// The opaque bundle identity ref and human name.
    BundleIdentity,
    /// The shared source class (certified / managed / community / imported / draft).
    SourceClass,
    /// The support / lifecycle class of the bundle.
    SupportClass,
    /// The signer / source trust of the bundle.
    SignerSource,
    /// The certification freshness of the claim.
    CertificationFreshness,
    /// The compatible Aureline range the bundle declares.
    CompatibleRange,
    /// The archetype id and supported platform / toolchain envelope.
    ArchetypeEnvelope,
}

impl M5LaunchWedgeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::WedgeId,
        Self::BundleIdentity,
        Self::SourceClass,
        Self::SupportClass,
        Self::SignerSource,
        Self::CertificationFreshness,
        Self::CompatibleRange,
        Self::ArchetypeEnvelope,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::WedgeId,
        Self::BundleIdentity,
        Self::SourceClass,
        Self::CertificationFreshness,
        Self::CompatibleRange,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WedgeId => "wedge_id",
            Self::BundleIdentity => "bundle_identity",
            Self::SourceClass => "source_class",
            Self::SupportClass => "support_class",
            Self::SignerSource => "signer_source",
            Self::CertificationFreshness => "certification_freshness",
            Self::CompatibleRange => "compatible_range",
            Self::ArchetypeEnvelope => "archetype_envelope",
        }
    }
}

// --- resolver input ---

/// The full input to the launch-wedge resolver for one launch-wedge context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchWedgeInput {
    /// The stable wedge identity that must survive across the bundle card and the
    /// certified-archetype badge group.
    pub wedge_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// Opaque ref to the bundle id the card offers; never raw manifest bytes.
    pub bundle_id_ref: String,
    /// Human-readable bundle name shown on the card.
    pub bundle_name: String,
    /// Persona / stack tag the card advertises (e.g. "Rust service", "web app").
    pub persona_stack_tag: String,
    /// The bundle class the card offers.
    pub bundle_class: BundleClass,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Support-class / lifecycle stage of the bundle.
    pub support_class: LifecycleStage,
    /// The shared source class (certified / managed / community / imported / draft).
    pub source_class: CertificationTarget,
    /// Certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class the card binds to.
    pub truth_mode: M5BundleTruthMode,
    /// Opaque ref to the `Review bundle` action target; never empty.
    pub review_action_ref: String,
    /// The card inherits a hidden marketplace / certification assumption from
    /// backend state alone rather than naming its source class; must be `false`.
    pub inherits_hidden_marketplace_assumption: bool,
    /// The card claims a current certification despite stale / missing freshness;
    /// must be `false`.
    pub claims_current_despite_stale: bool,
    /// Opaque ref to the archetype family the badge group describes.
    pub archetype_family_ref: String,
    /// Human-readable archetype id the badges carry.
    pub archetype_id: String,
    /// Detected archetype confidence.
    pub archetype_confidence: ArchetypeConfidence,
    /// Opaque ref to the supported platform / toolchain envelope; never empty.
    pub supported_platform_envelope_ref: String,
    /// Number of badges rendered in the group.
    pub badge_count: u32,
    /// Imported-vs-native confidence contributing to the assurance story.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// An externally-observed narrowing (stale mirror, offline cache, unverified
    /// signer) carried through onto the wedge before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved start-center bundle card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedStartCenterBundleCard {
    /// The wedge identity — identical to the badge group.
    pub wedge_id: String,
    /// The opaque bundle id ref.
    pub bundle_id_ref: String,
    /// The human-readable bundle name.
    pub bundle_name: String,
    /// The persona / stack tag.
    pub persona_stack_tag: String,
    /// The bundle class offered.
    pub bundle_class: BundleClass,
    /// The signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// The support / lifecycle class of the bundle.
    pub support_class: LifecycleStage,
    /// The shared source class named explicitly on the card.
    pub source_class: CertificationTarget,
    /// The entry-assurance tier derived from the source class (AC1).
    pub entry_assurance_tier: M5EntryAssuranceTier,
    /// The certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// The compatible Aureline range the bundle declares.
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class.
    pub truth_mode: M5BundleTruthMode,
    /// The opaque `Review bundle` action ref.
    pub review_action_ref: String,
    /// The card discloses signer / source; always holds.
    pub discloses_signer_source: bool,
    /// The card discloses certification freshness; always holds.
    pub discloses_certification_freshness: bool,
    /// The card names its source class rather than inheriting a hidden assumption
    /// (AC3); always holds.
    pub source_class_named: bool,
    /// A `Review bundle` action is present (AC1); always holds.
    pub review_action_present: bool,
}

/// The resolved certified-archetype badge group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCertifiedArchetypeBadgeGroup {
    /// The wedge identity — identical to the bundle card.
    pub wedge_id: String,
    /// The opaque archetype family ref.
    pub archetype_family_ref: String,
    /// The human-readable archetype id.
    pub archetype_id: String,
    /// The detected archetype confidence.
    pub archetype_confidence: ArchetypeConfidence,
    /// The shared source class — identical to the card.
    pub source_class: CertificationTarget,
    /// The certification freshness driving the badge state.
    pub certification_freshness: EvidenceFreshness,
    /// The opaque supported platform / toolchain envelope ref.
    pub supported_platform_envelope_ref: String,
    /// The number of badges rendered.
    pub badge_count: u32,
    /// The visible downgrade the group shows when evidence ages or scope narrows
    /// (AC2).
    pub downgrade_state: M5ArchetypeBadgeDowngradeState,
    /// The badge group degrades visibly whenever a downgrade is warranted (AC2).
    pub badges_degrade_visibly: bool,
    /// The badge group discloses certification freshness; always holds.
    pub discloses_certification_freshness: bool,
}

/// The resolved launch-wedge truth shared across the start-center bundle card and
/// the certified-archetype badge group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLaunchWedge {
    /// The stable wedge identity.
    pub wedge_id: String,
    /// The resolved start-center bundle card.
    pub bundle_card: M5ResolvedStartCenterBundleCard,
    /// The resolved certified-archetype badge group.
    pub badge_group: M5ResolvedCertifiedArchetypeBadgeGroup,
    /// The entry-assurance tier is legible before install (AC1).
    pub entry_assurance_disclosed: bool,
    /// The badges degrade visibly when evidence ages or scope narrows (AC2).
    pub badges_degrade_visibly: bool,
    /// The source class is named explicitly, never inherited from backend state
    /// alone (AC3).
    pub source_class_not_inherited: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedLaunchWedge {
    /// True when the wedge identity is identical across the bundle card and badge
    /// group.
    pub fn identity_consistent(&self) -> bool {
        self.bundle_card.wedge_id == self.wedge_id && self.badge_group.wedge_id == self.wedge_id
    }

    /// True when the card and badge group name the same shared source class — the
    /// wedge never tells two source stories.
    pub fn source_class_consistent(&self) -> bool {
        self.bundle_card.source_class == self.badge_group.source_class
    }

    /// True when the entry-assurance tier is disclosed before install (AC1).
    pub fn entry_assurance_disclosed(&self) -> bool {
        self.entry_assurance_disclosed
    }

    /// True when the badge group degrades visibly whenever a downgrade is warranted
    /// (AC2).
    pub fn badges_degrade_visibly(&self) -> bool {
        self.badges_degrade_visibly
    }

    /// True when the source class is named explicitly, not inherited (AC3).
    pub fn source_class_not_inherited(&self) -> bool {
        self.source_class_not_inherited
    }
}

/// Errors returned by [`resolve_launch_wedge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5LaunchWedgeResolutionError {
    /// The wedge identity was empty.
    EmptyWedgeId,
    /// The bundle id ref was empty.
    EmptyBundleIdRef,
    /// The bundle name was empty.
    EmptyBundleName,
    /// The compatible Aureline range was empty.
    EmptyCompatibleRange,
    /// The `Review bundle` action ref was empty.
    EmptyReviewAction,
    /// The archetype id was empty.
    EmptyArchetypeId,
    /// The supported platform / toolchain envelope ref was empty.
    EmptyPlatformEnvelope,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The card inherited a hidden marketplace / certification assumption instead of
    /// naming its source class.
    HiddenMarketplaceInheritance,
    /// A stale / missing certification was claimed as current instead of narrowing.
    StaleClaimShownAsCurrent,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5LaunchWedgeResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyWedgeId => "empty_wedge_id",
            Self::EmptyBundleIdRef => "empty_bundle_id_ref",
            Self::EmptyBundleName => "empty_bundle_name",
            Self::EmptyCompatibleRange => "empty_compatible_range",
            Self::EmptyReviewAction => "empty_review_action",
            Self::EmptyArchetypeId => "empty_archetype_id",
            Self::EmptyPlatformEnvelope => "empty_platform_envelope",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::HiddenMarketplaceInheritance => "hidden_marketplace_inheritance",
            Self::StaleClaimShownAsCurrent => "stale_claim_shown_as_current",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5LaunchWedgeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "launch-wedge resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5LaunchWedgeResolutionError {}

/// Resolves one launch-wedge context into its shared start-center bundle card and
/// certified-archetype badge group.
///
/// The two surfaces share one wedge identity, so the bundle's signer / source,
/// support class, certification state, compatible Aureline range, and archetype
/// evidence age never blur across them. The card always names its source class
/// (never inheriting a hidden marketplace assumption) and keeps a `Review bundle`
/// action; the badge group downgrades visibly when certification freshness slips or
/// archetype scope narrows; a stale certification never reads as current.
pub fn resolve_launch_wedge(
    input: &M5LaunchWedgeInput,
) -> Result<M5ResolvedLaunchWedge, M5LaunchWedgeResolutionError> {
    if input.wedge_id.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyWedgeId);
    }
    if input.bundle_id_ref.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyBundleIdRef);
    }
    if input.bundle_name.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyBundleName);
    }
    if input.compatible_aureline_range.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyCompatibleRange);
    }
    if input.review_action_ref.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyReviewAction);
    }
    if input.archetype_id.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyArchetypeId);
    }
    if input.supported_platform_envelope_ref.trim().is_empty() {
        return Err(M5LaunchWedgeResolutionError::EmptyPlatformEnvelope);
    }

    for value in [
        input.wedge_id.as_str(),
        input.surface_label.as_str(),
        input.bundle_id_ref.as_str(),
        input.bundle_name.as_str(),
        input.persona_stack_tag.as_str(),
        input.compatible_aureline_range.as_str(),
        input.review_action_ref.as_str(),
        input.archetype_family_ref.as_str(),
        input.archetype_id.as_str(),
        input.supported_platform_envelope_ref.as_str(),
    ] {
        if value_is_forbidden(value) {
            return Err(M5LaunchWedgeResolutionError::ForbiddenMaterial);
        }
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5LaunchWedgeResolutionError::DegradedLabelGeneric);
        }
    }

    // AC3: the card never inherits a hidden marketplace / certification assumption;
    // it must name its source class explicitly.
    if input.inherits_hidden_marketplace_assumption {
        return Err(M5LaunchWedgeResolutionError::HiddenMarketplaceInheritance);
    }

    // AC2: a stale / missing certification narrows the claim rather than being shown
    // as current.
    let downgrade_state = M5ArchetypeBadgeDowngradeState::for_evidence(
        input.certification_freshness,
        input.archetype_confidence,
    );
    if input.claims_current_despite_stale && downgrade_state.is_degraded() {
        return Err(M5LaunchWedgeResolutionError::StaleClaimShownAsCurrent);
    }

    let entry_assurance_tier = M5EntryAssuranceTier::for_source_class(input.source_class);

    // The badge group always renders the derived downgrade state, so a slip in
    // certification freshness or archetype scope is always visible rather than
    // silently absorbed into a fully-certified badge.
    let badges_degrade_visibly = true;

    let bundle_card = M5ResolvedStartCenterBundleCard {
        wedge_id: input.wedge_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        bundle_name: input.bundle_name.clone(),
        persona_stack_tag: input.persona_stack_tag.clone(),
        bundle_class: input.bundle_class,
        signer_source: input.signer_source,
        support_class: input.support_class,
        source_class: input.source_class,
        entry_assurance_tier,
        certification_freshness: input.certification_freshness,
        compatible_aureline_range: input.compatible_aureline_range.clone(),
        truth_mode: input.truth_mode,
        review_action_ref: input.review_action_ref.clone(),
        discloses_signer_source: true,
        discloses_certification_freshness: true,
        source_class_named: true,
        review_action_present: true,
    };

    let badge_group = M5ResolvedCertifiedArchetypeBadgeGroup {
        wedge_id: input.wedge_id.clone(),
        archetype_family_ref: input.archetype_family_ref.clone(),
        archetype_id: input.archetype_id.clone(),
        archetype_confidence: input.archetype_confidence,
        source_class: input.source_class,
        certification_freshness: input.certification_freshness,
        supported_platform_envelope_ref: input.supported_platform_envelope_ref.clone(),
        badge_count: input.badge_count,
        downgrade_state,
        badges_degrade_visibly,
        discloses_certification_freshness: true,
    };

    Ok(M5ResolvedLaunchWedge {
        wedge_id: input.wedge_id.clone(),
        bundle_card,
        badge_group,
        entry_assurance_disclosed: true,
        badges_degrade_visibly,
        source_class_not_inherited: !input.inherits_hidden_marketplace_assumption,
        degraded: input.degraded.clone(),
    })
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

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs launch-wedge truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchWedgeCase {
    /// The resolver input.
    pub input: M5LaunchWedgeInput,
    /// The resolved launch-wedge truth. Must equal `resolve_launch_wedge(&input)`.
    pub resolved: M5ResolvedLaunchWedge,
}

impl M5LaunchWedgeCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5LaunchWedgeInput) -> Self {
        let resolved = resolve_launch_wedge(&input).expect("seed launch-wedge case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_launch_wedge(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one launch-wedge surface family bound to the
/// shared launch-wedge contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchWedgeSurfaceRow {
    /// The launch-wedge surface family.
    pub surface_family: M5LaunchWedgeSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Source classes this surface can disclose (must be non-empty).
    pub source_classes: Vec<CertificationTarget>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5BundleTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5LaunchWedgeExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5BundleComponentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_wedges: Vec<M5LaunchWedgeCase>,
    /// Hard invariant: this row never hides the entry-assurance tier. MUST be
    /// `false`.
    pub hides_entry_assurance: bool,
    /// Hard invariant: this row never hides an archetype-badge downgrade. MUST be
    /// `false`.
    pub hides_badge_downgrade: bool,
    /// Hard invariant: this row never inherits a hidden source class. MUST be
    /// `false`.
    pub inherits_hidden_source_class: bool,
}

impl M5LaunchWedgeSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5LaunchWedgeExportField> =
            self.export_fields.iter().copied().collect();
        M5LaunchWedgeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_entry_assurance
            && !self.hides_badge_downgrade
            && !self.inherits_hidden_source_class
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchWedgeVocabularySet {
    /// Launch-wedge surface-family tokens.
    pub surface_families: Vec<String>,
    /// Entry-assurance-tier tokens.
    pub entry_assurance_tiers: Vec<String>,
    /// Archetype-badge downgrade-state tokens.
    pub badge_downgrade_states: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Source-class tokens (reused from the bundle-manifest contract).
    pub source_classes: Vec<String>,
    /// Bundle-class tokens (reused from the entry-governance contract).
    pub bundle_classes: Vec<String>,
    /// Signer / source-trust tokens (reused from the entry-governance contract).
    pub signer_sources: Vec<String>,
    /// Support-class / lifecycle tokens (reused from the bundle-manifest contract).
    pub support_classes: Vec<String>,
    /// Certification-freshness tokens (reused from the scorecard contract).
    pub freshness_states: Vec<String>,
    /// Archetype-confidence tokens (reused from the entry-governance contract).
    pub archetype_confidences: Vec<String>,
    /// Imported-vs-native confidence tokens (reused from the scorecard contract).
    pub imported_confidences: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5LaunchWedgeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5LaunchWedgeSurfaceFamily::ALL,
                M5LaunchWedgeSurfaceFamily::as_str,
            ),
            entry_assurance_tiers: tokens(&M5EntryAssuranceTier::ALL, M5EntryAssuranceTier::as_str),
            badge_downgrade_states: tokens(
                &M5ArchetypeBadgeDowngradeState::ALL,
                M5ArchetypeBadgeDowngradeState::as_str,
            ),
            export_fields: tokens(
                &M5LaunchWedgeExportField::ALL,
                M5LaunchWedgeExportField::as_str,
            ),
            source_classes: tokens(&CertificationTarget::ALL, CertificationTarget::as_str),
            bundle_classes: tokens(&BundleClass::ALL, BundleClass::as_str),
            signer_sources: tokens(&SourceTrust::ALL, SourceTrust::as_str),
            support_classes: tokens(&LifecycleStage::ALL, LifecycleStage::as_str),
            freshness_states: tokens(&EvidenceFreshness::ALL, EvidenceFreshness::as_str),
            archetype_confidences: tokens(&ArchetypeConfidence::ALL, ArchetypeConfidence::as_str),
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
pub struct M5LaunchWedgeGovernanceReview {
    /// One primitive carries start-center-card and archetype-badge-group truth on
    /// every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Wedge identity is preserved across the card and badge group.
    pub wedge_identity_preserved_across_surfaces: bool,
    /// Certified / approximate / local-only assurance is legible before install.
    pub entry_assurance_legible_before_install: bool,
    /// Archetype badges degrade visibly when evidence ages or scope narrows.
    pub archetype_badges_degrade_visibly: bool,
    /// Stack entry never inherits a hidden marketplace / certification assumption.
    pub source_class_never_inherited: bool,
    /// The support / export packet reconstructs launch-wedge truth.
    pub support_export_reconstructs_wedge: bool,
    /// Later M5 rows cannot invent parallel launch-wedge vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchWedgeConsumerProjection {
    /// Start-center / switcher / picker / diagnostics surfaces all consume the
    /// shared primitive.
    pub launch_surfaces_consume_shared_primitive: bool,
    /// The launch-wedge resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The badge group reads a single canonical certification source.
    pub badge_group_reads_single_certification_source: bool,
    /// Support / export reads a single canonical launch-wedge source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the launch-wedge primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchWedgeReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting launch-wedge audit.
    pub launch_wedge_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5StartCenterLaunchWedgePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StartCenterLaunchWedgePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5LaunchWedgeSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LaunchWedgeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LaunchWedgeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LaunchWedgeConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5LaunchWedgeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 start-center launch-wedge primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StartCenterLaunchWedgePacket {
    /// Record kind; must equal [`M5_START_CENTER_WEDGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_START_CENTER_WEDGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5LaunchWedgeSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LaunchWedgeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LaunchWedgeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LaunchWedgeConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5LaunchWedgeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5StartCenterLaunchWedgePacket {
    /// Builds an M5 start-center launch-wedge primitive packet from stable-lane
    /// input.
    pub fn new(input: M5StartCenterLaunchWedgePacketInput) -> Self {
        Self {
            record_kind: M5_START_CENTER_WEDGE_RECORD_KIND.to_owned(),
            schema_version: M5_START_CENTER_WEDGE_SCHEMA_VERSION,
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

    /// Validates the M5 launch-wedge primitive invariants.
    pub fn validate(&self) -> Vec<M5LaunchWedgeViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_START_CENTER_WEDGE_RECORD_KIND {
            violations.push(M5LaunchWedgeViolation::WrongRecordKind);
        }
        if self.schema_version != M5_START_CENTER_WEDGE_SCHEMA_VERSION {
            violations.push(M5LaunchWedgeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LaunchWedgeViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 launch-wedge primitive packet serializes"),
        ) {
            violations.push(M5LaunchWedgeViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 launch-wedge primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,source_classes,truth_modes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.source_classes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_wedges.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Start-Center Launch-Wedge Primitive: Start-Center Bundle Card and Certified-Archetype Badge Group\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Launch-wedge surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5LaunchWedgeSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Source classes: {}\n",
            self.vocabulary_set.source_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Entry-assurance tiers: {}\n",
            self.vocabulary_set.entry_assurance_tiers.join(", ")
        ));
        out.push_str(&format!(
            "- Badge downgrade states: {}\n",
            self.vocabulary_set.badge_downgrade_states.join(", ")
        ));
        out.push_str("\n## Launch-wedge surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked cases: {}\n", row.example_wedges.len()));
            for case in &row.example_wedges {
                out.push_str(&format!(
                    "    - `{}` → source `{}` (tier `{}`), badge `{}`, range `{}`\n",
                    case.resolved.wedge_id,
                    case.resolved.bundle_card.source_class.as_str(),
                    case.resolved.bundle_card.entry_assurance_tier.as_str(),
                    case.resolved.badge_group.downgrade_state.as_str(),
                    case.resolved.bundle_card.compatible_aureline_range,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 launch-wedge export.
#[derive(Debug)]
pub enum M5LaunchWedgeArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LaunchWedgeViolation>),
}

impl fmt::Display for M5LaunchWedgeArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 launch-wedge primitive export parse failed: {error}"
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
                    "m5 launch-wedge primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LaunchWedgeArtifactError {}

/// Validation failures emitted by [`M5StartCenterLaunchWedgePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LaunchWedgeViolation {
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
    /// A required launch-wedge surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
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
    /// A surface row declares no worked launch-wedge cases.
    ExampleWedgesMissing,
    /// A worked launch-wedge case does not match a fresh resolve of its input.
    ExampleWedgeDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves wedge identity preserved and entry assurance disclosed
    /// (AC1).
    EntryAssuranceUnproven,
    /// No worked case proves archetype badges degrade visibly (AC2).
    BadgeDowngradeUnproven,
    /// No worked case proves the source class is named, never inherited (AC3).
    SourceClassInheritanceUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5LaunchWedgeViolation {
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
            Self::SourceClassMissing => "source_class_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleWedgesMissing => "example_wedges_missing",
            Self::ExampleWedgeDrift => "example_wedge_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::EntryAssuranceUnproven => "entry_assurance_unproven",
            Self::BadgeDowngradeUnproven => "badge_downgrade_unproven",
            Self::SourceClassInheritanceUnproven => "source_class_inheritance_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 launch-wedge export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_start_center_launch_wedge_export(
) -> Result<M5StartCenterLaunchWedgePacket, M5LaunchWedgeArtifactError> {
    let packet: M5StartCenterLaunchWedgePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-start-center-launch-wedge-primitive-proof/support_export.json"
    )))
    .map_err(M5LaunchWedgeArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LaunchWedgeArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_START_CENTER_WEDGE_SCHEMA_REF,
        M5_START_CENTER_WEDGE_DOC_REF,
        M5_START_CENTER_WEDGE_COMPONENT_MATRIX_REF,
        M5_START_CENTER_WEDGE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LaunchWedgeViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LaunchWedgeViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    let present: BTreeSet<M5LaunchWedgeSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5LaunchWedgeSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5LaunchWedgeViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5LaunchWedgeViolation::SurfaceRowIncomplete);
        }
        if row.source_classes.is_empty() {
            violations.push(M5LaunchWedgeViolation::SourceClassMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5LaunchWedgeViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5LaunchWedgeViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LaunchWedgeViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LaunchWedgeViolation::ConsumerSurfacesMissing);
        }
        if row.example_wedges.is_empty() {
            violations.push(M5LaunchWedgeViolation::ExampleWedgesMissing);
        }
        if row
            .example_wedges
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5LaunchWedgeViolation::ExampleWedgeDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5LaunchWedgeViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: wedge identity preserved and entry assurance disclosed (AC1),
/// archetype badges degrade visibly when evidence ages or scope narrows (AC2), and
/// the source class named rather than inherited from backend state alone (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    let cases: Vec<&M5ResolvedLaunchWedge> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_wedges.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case preserves identity, names one consistent source class,
    // and discloses the entry-assurance tier; and every case discloses it.
    let entry_assurance_proven = cases.iter().any(|resolved| {
        resolved.identity_consistent()
            && resolved.source_class_consistent()
            && resolved.entry_assurance_disclosed()
    }) && cases
        .iter()
        .all(|resolved| resolved.entry_assurance_disclosed());
    if !entry_assurance_proven {
        violations.push(M5LaunchWedgeViolation::EntryAssuranceUnproven);
    }

    // AC2: at least one case actually shows a badge downgrade, and every case
    // degrades visibly exactly when warranted (never silently current).
    let badge_downgrade_proven = cases
        .iter()
        .any(|resolved| resolved.badge_group.downgrade_state.is_degraded())
        && cases
            .iter()
            .all(|resolved| resolved.badges_degrade_visibly());
    if !badge_downgrade_proven {
        violations.push(M5LaunchWedgeViolation::BadgeDowngradeUnproven);
    }

    // AC3: every case names its source class rather than inheriting one, and at
    // least one non-certified case shows its true (approximate / local-only) class
    // instead of inheriting an official tier.
    let source_class_proven = cases
        .iter()
        .all(|resolved| resolved.source_class_not_inherited())
        && cases.iter().any(|resolved| {
            resolved.source_class_not_inherited()
                && resolved.bundle_card.entry_assurance_tier != M5EntryAssuranceTier::Certified
        });
    if !source_class_proven {
        violations.push(M5LaunchWedgeViolation::SourceClassInheritanceUnproven);
    }
}

fn validate_governance_review(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.wedge_identity_preserved_across_surfaces,
        review.entry_assurance_legible_before_install,
        review.archetype_badges_degrade_visibly,
        review.source_class_never_inherited,
        review.support_export_reconstructs_wedge,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5LaunchWedgeViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.launch_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.badge_group_reads_single_certification_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5LaunchWedgeViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5StartCenterLaunchWedgePacket,
    violations: &mut Vec<M5LaunchWedgeViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.launch_wedge_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LaunchWedgeViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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
