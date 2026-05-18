//! Symbol-linked docs search rows with citation and stale-example truth.
//!
//! This module is the docs/help lane that sits beside lexical, structural,
//! cached, and graph-backed search. It keeps symbol-to-doc links, exact
//! citation anchors, docs source/version/locality metadata, citation-drawer
//! hooks, docs suggestions, and stale-example signals attached to the same
//! planned result IDs and ranking reasons used by the rest of search.

use std::collections::BTreeMap;

use aureline_docs::{
    CitationAnchorAlpha, CitationAnchorAlphaInput, CitationAnchorAvailability,
    CitationConfidenceClass, CitationDrawerEvidenceView, CitationInferenceMarker,
    CitationLocalityClass, CitationSourceClass, DocsDerivedExplanationKind,
    DocsExampleValidationClass, DocsExternalOpenFallback,
    DocsFreshnessClass as CanonicalDocsFreshnessClass, DocsKnowledgeObjectKind,
    DocsKnowledgeSurfaceKind, DocsKnowledgeSurfaceProjection, DocsKnowledgeSurfaceProjectionInput,
    DocsMirrorOfflinePosture, DocsNodeIdentity, DocsNodeIdentityInput, DocsNodeKind,
    DocsNodeProvenance, DocsNodeProvenanceInput, DocsScopeClass, LocaleOverlayState,
    SourcePrecedenceClass, VersionMatchState,
};
use serde::{Deserialize, Serialize};

use crate::{
    PlannedResultSet, PlannedSearchResult, PlannerCandidate, PlannerDataPath,
    PlannerFreshnessClass, PlannerPathReadiness, PlannerPathSnapshot, PlannerRankingReason,
    PlannerResultTruthClass, PlannerTargetKind, PlannerUnavailableReason, SearchPlannerAlpha,
    SearchPlannerInputs, SearchQuerySession, SearchSurface,
};

/// Schema version for docs-linking alpha records.
pub const DOCS_LINKING_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Product subject class a docs reference binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSubjectKind {
    /// Workspace or package symbol.
    Symbol,
    /// Workspace file or document.
    File,
    /// Product setting or configuration key.
    Setting,
    /// Command-registry command.
    Command,
    /// Capability lifecycle row.
    CapabilityLifecycle,
    /// Keybinding or shortcut row.
    Keybinding,
    /// Support runbook step.
    RunbookStep,
    /// Release note entry.
    ReleaseNote,
    /// Glossary term.
    GlossaryTerm,
    /// Onboarding or guided-tour step.
    OnboardingStep,
    /// Service-health event.
    ServiceHealthEvent,
}

impl DocsSubjectKind {
    /// Returns the stable token for this subject kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::File => "file",
            Self::Setting => "setting",
            Self::Command => "command",
            Self::CapabilityLifecycle => "capability_lifecycle",
            Self::Keybinding => "keybinding",
            Self::RunbookStep => "runbook_step",
            Self::ReleaseNote => "release_note",
            Self::GlossaryTerm => "glossary_term",
            Self::OnboardingStep => "onboarding_step",
            Self::ServiceHealthEvent => "service_health_event",
        }
    }

    const fn is_symbol_linked(self) -> bool {
        matches!(
            self,
            Self::Symbol
                | Self::Setting
                | Self::Command
                | Self::CapabilityLifecycle
                | Self::Keybinding
        )
    }
}

/// Documentation material class represented by an exact anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsDocKind {
    /// API reference row.
    ApiReference,
    /// How-to or conceptual guide.
    Guide,
    /// Executable or illustrative example.
    Example,
    /// Migration guidance.
    MigrationNote,
    /// Release-note row.
    ReleaseNote,
    /// Product help article.
    HelpArticle,
    /// Generated reference from commands, settings, or schemas.
    GeneratedReference,
    /// Support runbook entry.
    Runbook,
    /// Glossary row.
    Glossary,
}

impl DocsDocKind {
    /// Returns the stable token for this doc kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApiReference => "api_reference",
            Self::Guide => "guide",
            Self::Example => "example",
            Self::MigrationNote => "migration_note",
            Self::ReleaseNote => "release_note",
            Self::HelpArticle => "help_article",
            Self::GeneratedReference => "generated_reference",
            Self::Runbook => "runbook",
            Self::Glossary => "glossary",
        }
    }
}

/// Source class for a docs/help row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSourceClass {
    /// Project-owned docs pack for the running product or workspace.
    ProjectDocs,
    /// Reference generated from the current build or command/schema registry.
    GeneratedReference,
    /// Signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// Curated knowledge pack.
    CuratedKnowledgePack,
    /// Derived explanation that cites upstream anchors.
    DerivedExplanation,
    /// Vendor or provider docs overlay.
    VendorProviderDocs,
    /// Support runbook source.
    SupportRunbook,
    /// External service-status feed.
    ExternalStatusFeed,
}

impl DocsSourceClass {
    /// Returns the stable token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::DerivedExplanation => "derived_explanation",
            Self::VendorProviderDocs => "vendor_provider_docs",
            Self::SupportRunbook => "support_runbook",
            Self::ExternalStatusFeed => "external_status_feed",
        }
    }
}

/// Freshness class captured for a docs anchor at projection time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFreshnessClass {
    /// Source was refreshed from the canonical owner.
    AuthoritativeLive,
    /// Cached source is still inside the accepted freshness window.
    WarmCached,
    /// Cached source is degraded but still usable with disclosure.
    DegradedCached,
    /// Source is stale and must not be shown as current authority.
    Stale,
    /// Source freshness cannot be verified.
    Unverified,
}

impl DocsFreshnessClass {
    /// Returns the stable token for this freshness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    const fn planner_readiness(self) -> PlannerPathReadiness {
        match self {
            Self::AuthoritativeLive | Self::WarmCached => PlannerPathReadiness::Ready,
            Self::DegradedCached | Self::Unverified => PlannerPathReadiness::Partial,
            Self::Stale => PlannerPathReadiness::Stale,
        }
    }
}

/// Version posture between a docs row and the running build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionMatchState {
    /// Docs source matches the running build exactly.
    ExactBuildMatch,
    /// Docs source is within the compatible minor drift window.
    CompatibleMinorDrift,
    /// Docs source is incompatible with the running build.
    IncompatibleDriftDetected,
    /// Pre-release docs have not completed validation.
    PreReleaseUnverified,
    /// Target build could not be resolved.
    UnknownTargetBuild,
}

impl DocsVersionMatchState {
    /// Returns the stable token for this version state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }
}

/// Locality or transport class for a docs row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsLocalityClass {
    /// Project docs available locally.
    LocalProjectPack,
    /// Generated reference available locally.
    GeneratedLocal,
    /// Mirrored docs pack available offline.
    MirroredOffline,
    /// Cached docs row available locally.
    CachedLocal,
    /// Live vendor or provider docs require a handoff.
    VendorLive,
    /// Support pack or runbook source.
    SupportPack,
}

impl DocsLocalityClass {
    /// Returns the stable token for this locality class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProjectPack => "local_project_pack",
            Self::GeneratedLocal => "generated_local",
            Self::MirroredOffline => "mirrored_offline",
            Self::CachedLocal => "cached_local",
            Self::VendorLive => "vendor_live",
            Self::SupportPack => "support_pack",
        }
    }
}

/// Citation availability for a result row or drawer hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsCitationAvailability {
    /// One or more citation anchors can be inspected.
    Available,
    /// The row requires a citation anchor but the exact anchor is missing.
    Missing,
    /// The row does not require a citation anchor.
    NotRequired,
}

impl DocsCitationAvailability {
    /// Returns the stable token for this citation state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Missing => "missing",
            Self::NotRequired => "not_required",
        }
    }
}

/// Resolution class used by symbol-linked docs references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsLinkResolutionClass {
    /// Exact symbol or subject matched an anchor.
    ExactSymbolMatch,
    /// Nearby version fallback was used.
    NearbyVersionFallback,
    /// Package or guide fallback was used.
    PackageLevelGuideFallback,
    /// Project docs outranked a vendor match.
    ProjectDocsOutranksVendorMatch,
    /// Vendor docs override was disclosed by policy.
    VendorOverridesProjectDisclosed,
    /// Reference requires a docs-pack refresh.
    UnresolvedRequiresRefresh,
    /// No claim exists yet and support routing is required.
    NoClaimYetSupportRouted,
}

impl DocsLinkResolutionClass {
    /// Returns the stable token for this resolution class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSymbolMatch => "exact_symbol_match",
            Self::NearbyVersionFallback => "nearby_version_fallback",
            Self::PackageLevelGuideFallback => "package_level_guide_fallback",
            Self::ProjectDocsOutranksVendorMatch => "project_docs_outranks_vendor_match",
            Self::VendorOverridesProjectDisclosed => "vendor_overrides_project_disclosed",
            Self::UnresolvedRequiresRefresh => "unresolved_requires_refresh",
            Self::NoClaimYetSupportRouted => "no_claim_yet_support_routed",
        }
    }
}

/// Project-docs versus vendor-docs precedence cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsProjectVendorTruthCue {
    /// Project docs are the only authority in scope.
    ProjectAuthoritativeOnly,
    /// Project docs outrank vendor docs by default.
    ProjectOutranksVendorDefault,
    /// Vendor docs override project docs under policy.
    VendorOverridesProjectByPolicy,
    /// Vendor docs are inspect-only overlay material.
    VendorProviderOverlayInspectOnly,
    /// No project claim exists but vendor docs are available.
    NoProjectClaimVendorAvailable,
}

impl DocsProjectVendorTruthCue {
    /// Returns the stable token for this precedence cue.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectAuthoritativeOnly => "project_authoritative_only",
            Self::ProjectOutranksVendorDefault => "project_outranks_vendor_default",
            Self::VendorOverridesProjectByPolicy => "vendor_overrides_project_by_policy",
            Self::VendorProviderOverlayInspectOnly => "vendor_provider_overlay_inspect_only",
            Self::NoProjectClaimVendorAvailable => "no_project_claim_vendor_available",
        }
    }
}

/// Derived-explanation reuse state for a linked docs reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsDerivedReuseState {
    /// Derived surfaces may reuse the row with citation anchors.
    ReusableWithCitationAnchor,
    /// Reuse is refused because no citation is available.
    RefusedUncited,
    /// Reuse is refused because the signature is not verified.
    RefusedSignatureUnverified,
    /// Reuse is refused because mirror continuity is broken.
    RefusedMirrorContinuityBroken,
    /// Reuse is refused because vendor overlay needs higher trust.
    RefusedVendorOverlayRequiresHigherTrust,
    /// Reuse is refused for an external status feed.
    RefusedExternalStatusFeed,
}

impl DocsDerivedReuseState {
    /// Returns the stable token for this reuse state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReusableWithCitationAnchor => "reusable_with_citation_anchor",
            Self::RefusedUncited => "refused_uncited",
            Self::RefusedSignatureUnverified => "refused_signature_unverified",
            Self::RefusedMirrorContinuityBroken => "refused_mirror_continuity_broken",
            Self::RefusedVendorOverlayRequiresHigherTrust => {
                "refused_vendor_overlay_requires_higher_trust"
            }
            Self::RefusedExternalStatusFeed => "refused_external_status_feed",
        }
    }
}

/// Trigger class for docs suggestions and stale-example findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsTriggerClass {
    /// Code diff changed the documented behavior.
    CodeDiff,
    /// API contract drift changed the documented behavior.
    ApiDrift,
    /// Existing example is stale.
    StaleExample,
    /// Snippet or transcript failed validation.
    FailingSnippet,
    /// Release note no longer matches the product state.
    ReleaseNoteDrift,
    /// Command output changed.
    CommandOutputDrift,
    /// Docs-pack freshness window expired.
    DocsPackFreshnessExpired,
}

impl DocsTriggerClass {
    /// Returns the stable token for this trigger class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeDiff => "code_diff",
            Self::ApiDrift => "api_drift",
            Self::StaleExample => "stale_example",
            Self::FailingSnippet => "failing_snippet",
            Self::ReleaseNoteDrift => "release_note_drift",
            Self::CommandOutputDrift => "command_output_drift",
            Self::DocsPackFreshnessExpired => "docs_pack_freshness_expired",
        }
    }
}

/// Validation freshness for docs evidence and suggestions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsValidationFreshness {
    /// Validation is current.
    Current,
    /// Validation remains inside the budget window.
    WithinValidationBudget,
    /// Revalidation is required before apply or publish.
    RevalidationRequired,
    /// Validation is stale.
    Stale,
    /// Validation has not been completed.
    Unverified,
    /// Validation is unsupported in the current environment.
    Unsupported,
}

impl DocsValidationFreshness {
    /// Returns the stable token for this validation freshness.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::WithinValidationBudget => "within_validation_budget",
            Self::RevalidationRequired => "revalidation_required",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Detection state for stale docs or examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsStaleDetectionState {
    /// A validator or resolver observed a concrete failure.
    ProvenBroken,
    /// Drift signals exist but a failure was not reproduced.
    SuspectedStale,
    /// Required validation is missing or expired.
    UnchangedUnverified,
    /// Stale detection does not apply.
    NotApplicable,
}

impl DocsStaleDetectionState {
    /// Returns the stable token for this detection state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenBroken => "proven_broken",
            Self::SuspectedStale => "suspected_stale",
            Self::UnchangedUnverified => "unchanged_unverified",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Publish-boundary state for docs maintenance cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPublishBoundaryState {
    /// Content remains local authoring material.
    LocalOnlyAuthoring,
    /// A local review diff is ready.
    LocalReviewReady,
    /// Publishing requires an external or release handoff.
    PublishHandoffOnly,
    /// Validation or owner review blocks publish.
    BlockedPendingValidation,
    /// Row is not publish-bearing.
    NotPublishBearing,
}

impl DocsPublishBoundaryState {
    /// Returns the stable token for this publish-boundary state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyAuthoring => "local_only_authoring",
            Self::LocalReviewReady => "local_review_ready",
            Self::PublishHandoffOnly => "publish_handoff_only",
            Self::BlockedPendingValidation => "blocked_pending_validation",
            Self::NotPublishBearing => "not_publish_bearing",
        }
    }
}

/// Evidence state for docs suggestion cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsEvidenceState {
    /// Suggestion is backed by current evidence.
    EvidenceBacked,
    /// Suggestion is waiting on validation budget.
    AwaitingValidationBudget,
}

impl DocsEvidenceState {
    /// Returns the stable token for this evidence state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceBacked => "evidence_backed",
            Self::AwaitingValidationBudget => "awaiting_validation_budget",
        }
    }
}

/// Docs maintenance suggestion class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSuggestionClass {
    /// README update suggestion.
    ReadmeUpdate,
    /// Changelog update suggestion.
    ChangelogUpdate,
    /// Help article update suggestion.
    HelpUpdate,
    /// Stale example finding.
    StaleExampleFinding,
    /// Symbol-linked docs suggestion.
    SymbolLinkedDocSuggestion,
}

impl DocsSuggestionClass {
    /// Returns the stable token for this suggestion class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadmeUpdate => "readme_update",
            Self::ChangelogUpdate => "changelog_update",
            Self::HelpUpdate => "help_update",
            Self::StaleExampleFinding => "stale_example_finding",
            Self::SymbolLinkedDocSuggestion => "symbol_linked_doc_suggestion",
        }
    }
}

/// Evidence action exposed from a docs row or suggestion card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsEvidenceActionKind {
    /// Open the citation drawer or evidence view.
    OpenCitationDrawer,
    /// Open the failing source, snippet, or validator output.
    OpenFailingSource,
    /// Open a review diff.
    ReviewDiff,
    /// Open docs maintenance details.
    OpenDocsMaintenance,
    /// Open docs-pack metadata.
    OpenPackMetadata,
}

impl DocsEvidenceActionKind {
    /// Returns the stable token for this evidence action kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCitationDrawer => "open_citation_drawer",
            Self::OpenFailingSource => "open_failing_source",
            Self::ReviewDiff => "review_diff",
            Self::OpenDocsMaintenance => "open_docs_maintenance",
            Self::OpenPackMetadata => "open_pack_metadata",
        }
    }
}

/// Exact docs anchor and source strip for a result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsExactAnchor {
    /// Stable citation or docs anchor id.
    pub anchor_id: String,
    /// Exact docs anchor ref shown to the user.
    pub exact_anchor_ref: String,
    /// Docs material kind for this anchor.
    pub doc_kind: DocsDocKind,
    /// Source class for this anchor.
    pub source_class: DocsSourceClass,
    /// Docs pack id.
    pub pack_id: String,
    /// Docs pack revision ref.
    pub pack_revision_ref: String,
    /// Display version or revision label.
    pub source_version: String,
    /// Source build date or deterministic build stamp.
    #[serde(default = "default_source_build_at")]
    pub source_build_at: String,
    /// Locality class for this anchor.
    pub locality: DocsLocalityClass,
    /// Freshness class captured at projection time.
    pub freshness: DocsFreshnessClass,
    /// Version match state captured at projection time.
    pub version_match_state: DocsVersionMatchState,
    /// Citation availability for this anchor.
    pub citation_availability: DocsCitationAvailability,
}

/// Citation-drawer or evidence-view hook attached to a summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsCitationDrawerHook {
    /// Stable hook id.
    pub hook_id: String,
    /// Citation availability exposed by this hook.
    pub availability: DocsCitationAvailability,
    /// Citation anchor refs opened by this hook.
    pub citation_anchor_refs: Vec<String>,
    /// Target ref for the drawer or evidence view.
    pub target_ref: String,
    /// Exact reopen ref for the drawer or evidence view.
    pub exact_reopen_ref: String,
    /// True when preview or summary cards must open this evidence view.
    pub opens_evidence_view: bool,
}

/// Downgrade record when an exact subject anchor is missing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMissingAnchorDowngrade {
    /// Anchor or subject that was requested first.
    pub requested_anchor_ref: String,
    /// Resolution class after downgrade.
    pub downgrade_state: DocsLinkResolutionClass,
    /// Opaque reason ref or short stable reason token.
    pub downgrade_reason_ref: String,
    /// Optional repair action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_action_ref: Option<String>,
}

/// Action link for evidence, failing sources, or review diffs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsEvidenceAction {
    /// Action kind.
    pub action_kind: DocsEvidenceActionKind,
    /// Stable action ref.
    pub action_ref: String,
    /// Target ref the action opens.
    pub target_ref: String,
}

/// Stale-example or failing-snippet signal attached to a docs result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsStaleExampleSignal {
    /// Stable finding id.
    pub finding_id: String,
    /// Trigger that produced the finding.
    pub trigger_class: DocsTriggerClass,
    /// Validation freshness for the finding.
    pub validation_freshness: DocsValidationFreshness,
    /// Optional timestamp for the last validation attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_validated_at: Option<String>,
    /// Detection posture for this stale signal.
    pub stale_detection_state: DocsStaleDetectionState,
    /// Action that opens the failing source.
    pub open_failing_source_action: DocsEvidenceAction,
    /// Additional evidence actions exposed by the card.
    pub evidence_actions: Vec<DocsEvidenceAction>,
    /// Publish-boundary posture for the finding.
    pub publish_boundary_state: DocsPublishBoundaryState,
}

/// Docs maintenance suggestion rendered as evidence, not authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionCard {
    /// Stable suggestion id.
    pub suggestion_id: String,
    /// Suggestion class.
    pub suggestion_class: DocsSuggestionClass,
    /// Trigger that produced the suggestion.
    pub trigger_class: DocsTriggerClass,
    /// Target branch for the suggestion.
    pub target_branch: String,
    /// Target release channel for the suggestion.
    pub target_channel: String,
    /// Local authoring posture.
    pub local_authoring_posture: String,
    /// Publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Evidence posture.
    pub evidence_state: DocsEvidenceState,
    /// Validation freshness for the suggestion.
    pub validation_freshness: DocsValidationFreshness,
    /// Review-diff action, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_diff_action: Option<DocsEvidenceAction>,
    /// Evidence actions exposed by the suggestion.
    pub evidence_actions: Vec<DocsEvidenceAction>,
}

/// Symbol-linked docs reference used as a docs-search candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkedReference {
    /// Stable reference id.
    pub reference_id: String,
    /// Product subject kind.
    pub subject_kind: DocsSubjectKind,
    /// Product subject ref.
    pub subject_ref: String,
    /// Display label for the row.
    pub display_label: String,
    /// Short support-safe summary.
    pub summary: String,
    /// Resolution class for the subject-to-doc link.
    pub resolution_class: DocsLinkResolutionClass,
    /// Ordered fallback chain traversed before this resolution.
    pub resolution_fallback_chain: Vec<DocsLinkResolutionClass>,
    /// Exact anchor opened by the row.
    pub exact_anchor: DocsExactAnchor,
    /// Project-docs versus vendor-docs precedence cue.
    pub project_vs_vendor_truth_cue: DocsProjectVendorTruthCue,
    /// Derived explanation reuse state.
    pub derived_explanation_reuse_state: DocsDerivedReuseState,
    /// Citation drawer or evidence-view hook.
    pub citation_drawer_hook: DocsCitationDrawerHook,
    /// Missing-anchor downgrade, when the exact subject anchor is absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_anchor_downgrade: Option<DocsMissingAnchorDowngrade>,
    /// Stale-example signal attached to this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_example_signal: Option<DocsStaleExampleSignal>,
    /// Docs suggestion card attached to this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_card: Option<DocsSuggestionCard>,
    /// Optional repair hook ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
    /// Optional browser handoff reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason: Option<String>,
}

impl DocsLinkedReference {
    /// Returns the canonical docs anchor id used by planner result fusion.
    pub fn canonical_id(&self) -> String {
        self.exact_anchor.anchor_id.clone()
    }

    /// Returns planner ranking reasons for this docs row.
    pub fn planner_ranking_reasons(&self) -> Vec<PlannerRankingReason> {
        let mut reasons = vec![PlannerRankingReason::DocsAnchorMatch];
        if self.subject_kind.is_symbol_linked() {
            reasons.push(PlannerRankingReason::DocsSymbolLinkedReference);
        }
        if matches!(
            self.project_vs_vendor_truth_cue,
            DocsProjectVendorTruthCue::ProjectAuthoritativeOnly
                | DocsProjectVendorTruthCue::ProjectOutranksVendorDefault
        ) {
            reasons.push(PlannerRankingReason::DocsSourcePrecedence);
        }
        match self.exact_anchor.citation_availability {
            DocsCitationAvailability::Available => {
                reasons.push(PlannerRankingReason::CitationAvailable)
            }
            DocsCitationAvailability::Missing => {
                reasons.push(PlannerRankingReason::CitationMissing)
            }
            DocsCitationAvailability::NotRequired => {}
        }
        if self.stale_example_signal.is_some() {
            reasons.push(PlannerRankingReason::StaleExampleSignal);
        }
        if self.exact_anchor.freshness.planner_readiness() != PlannerPathReadiness::Ready {
            reasons.push(PlannerRankingReason::PartialIndex);
        }
        reasons
    }

    /// Returns partial-truth cause tokens for this row.
    pub fn partial_truth_causes(&self) -> Vec<String> {
        let mut causes = Vec::new();
        if self.exact_anchor.citation_availability == DocsCitationAvailability::Missing {
            push_unique(&mut causes, "citation_missing");
        }
        if self.missing_anchor_downgrade.is_some() {
            push_unique(&mut causes, "missing_anchor_downgrade");
        }
        if let Some(signal) = &self.stale_example_signal {
            push_unique(&mut causes, signal.trigger_class.as_str());
            push_unique(&mut causes, signal.validation_freshness.as_str());
        }
        if matches!(
            self.exact_anchor.freshness,
            DocsFreshnessClass::DegradedCached
                | DocsFreshnessClass::Stale
                | DocsFreshnessClass::Unverified
        ) {
            push_unique(&mut causes, self.exact_anchor.freshness.as_str());
        }
        causes
    }

    fn to_planner_candidate(&self) -> PlannerCandidate {
        PlannerCandidate {
            candidate_id: self.reference_id.clone(),
            canonical_id: self.canonical_id(),
            target_kind: PlannerTargetKind::DocsAnchor,
            title: self.display_label.clone(),
            relative_path: None,
            symbol_ref: Some(self.subject_ref.clone()),
            ranking_reasons: self.planner_ranking_reasons(),
            partial_truth_causes: self.partial_truth_causes(),
            scope_truth: None,
        }
    }

    fn docs_node_identity(&self) -> DocsNodeIdentity {
        let hidden_or_omitted_note =
            if self.exact_anchor.citation_availability == DocsCitationAvailability::Missing {
                Some(
                    self.missing_anchor_downgrade
                        .as_ref()
                        .map(|downgrade| {
                            format!(
                                "{} via {}",
                                downgrade.downgrade_reason_ref,
                                downgrade.downgrade_state.as_str()
                            )
                        })
                        .unwrap_or_else(|| "exact citation anchor is missing".to_owned()),
                )
            } else {
                None
            };

        DocsNodeIdentity::new(DocsNodeIdentityInput {
            docs_node_id: format!("docs-node:{}", self.exact_anchor.anchor_id),
            doc_kind: docs_node_kind_from_doc_kind(self.exact_anchor.doc_kind),
            source_class: citation_source_class_from_docs(self.exact_anchor.source_class),
            scope_class: docs_scope_from_subject(self.subject_kind),
            source_pack_ref: self.exact_anchor.pack_id.clone(),
            source_pack_revision_ref: self.exact_anchor.pack_revision_ref.clone(),
            version_or_revision_ref: self.exact_anchor.source_version.clone(),
            version_match_state: version_match_from_docs(self.exact_anchor.version_match_state),
            freshness_class: freshness_from_docs(self.exact_anchor.freshness),
            locality_class: locality_from_docs(self.exact_anchor.locality),
            source_locale: "en".to_owned(),
            requested_locale: "en".to_owned(),
            effective_locale: "en".to_owned(),
            locale_overlay_state: LocaleOverlayState::SourceLanguageOriginal,
            source_language_fallback_ref: None,
            citation_availability: citation_availability_from_docs(
                self.exact_anchor.citation_availability,
            ),
            citation_anchor_refs: self.citation_drawer_hook.citation_anchor_refs.clone(),
            exact_reopen_ref: self.citation_drawer_hook.exact_reopen_ref.clone(),
            hidden_or_omitted_note,
        })
    }

    fn citation_anchor_alpha(&self) -> CitationAnchorAlpha {
        let availability = citation_availability_from_docs(self.exact_anchor.citation_availability);
        let hidden_or_omitted_note = if availability.requires_note() {
            Some(
                self.missing_anchor_downgrade
                    .as_ref()
                    .map(|downgrade| {
                        format!(
                            "{} via {}",
                            downgrade.downgrade_reason_ref,
                            downgrade.downgrade_state.as_str()
                        )
                    })
                    .unwrap_or_else(|| "exact citation anchor is missing".to_owned()),
            )
        } else {
            None
        };

        CitationAnchorAlpha::new(CitationAnchorAlphaInput {
            anchor_id: self.exact_anchor.anchor_id.clone(),
            docs_node_ref: format!("docs-node:{}", self.exact_anchor.anchor_id),
            source_class: citation_source_class_from_docs(self.exact_anchor.source_class),
            source_pack_ref: self.exact_anchor.pack_id.clone(),
            source_pack_revision_ref: self.exact_anchor.pack_revision_ref.clone(),
            target_ref: self.subject_ref.clone(),
            exact_anchor_ref: availability
                .is_exact()
                .then(|| self.exact_anchor.exact_anchor_ref.clone()),
            locale: "en".to_owned(),
            version_match_state: version_match_from_docs(self.exact_anchor.version_match_state),
            freshness_class: freshness_from_docs(self.exact_anchor.freshness),
            locality_class: locality_from_docs(self.exact_anchor.locality),
            citation_availability: availability,
            inference_marker: CitationInferenceMarker::RawSource,
            confidence_class: if availability.is_exact() {
                CitationConfidenceClass::EvidenceBacked
            } else {
                CitationConfidenceClass::LowConfidence
            },
            hidden_or_omitted_note,
        })
    }

    fn citation_evidence_view(&self) -> CitationDrawerEvidenceView {
        CitationDrawerEvidenceView::from_node_and_anchors(
            self.citation_drawer_hook.hook_id.clone(),
            self.docs_node_identity(),
            [self.citation_anchor_alpha()],
            source_precedence_from_docs(self.project_vs_vendor_truth_cue),
            self.citation_drawer_hook.exact_reopen_ref.clone(),
        )
    }

    fn docs_node_provenance(&self, docs_node: &DocsNodeIdentity) -> DocsNodeProvenance {
        let external_open = self
            .browser_handoff_reason
            .as_ref()
            .filter(|reason| !reason.trim().is_empty())
            .map(|reason| {
                DocsExternalOpenFallback::available(
                    format!(
                        "action:docs-search-open-source:{}",
                        sanitize_planner_id(&self.reference_id)
                    ),
                    "Open supporting source",
                    format!(
                        "browser-handoff:{}:{}",
                        reason,
                        sanitize_planner_id(&self.reference_id)
                    ),
                )
            })
            .unwrap_or_else(|| {
                if self.exact_anchor.locality == DocsLocalityClass::VendorLive {
                    DocsExternalOpenFallback::blocked_by_policy(
                        "External docs handoff was not available for this search result.",
                    )
                } else {
                    DocsExternalOpenFallback::not_required()
                }
            });
        DocsNodeProvenance::new(DocsNodeProvenanceInput {
            provenance_id: format!(
                "docs-search-provenance:{}",
                sanitize_planner_id(&self.reference_id)
            ),
            docs_node: docs_node.clone(),
            knowledge_object_kind: DocsKnowledgeObjectKind::from(docs_node.doc_kind),
            derived_explanation_kind: (docs_node.source_class
                == CitationSourceClass::DerivedExplanation)
                .then_some(DocsDerivedExplanationKind::Generated),
            source_build_at: self.exact_anchor.source_build_at.clone(),
            running_build_identity_ref: self.exact_anchor.source_version.clone(),
            mirror_offline_posture: DocsMirrorOfflinePosture::from_locality(
                docs_node.locality_class,
            ),
            external_open,
            example_validation: self
                .stale_example_signal
                .as_ref()
                .map(|signal| match signal.stale_detection_state {
                    DocsStaleDetectionState::NotApplicable => DocsExampleValidationClass::Verified,
                    DocsStaleDetectionState::ProvenBroken
                    | DocsStaleDetectionState::SuspectedStale => {
                        DocsExampleValidationClass::FailedStale
                    }
                    DocsStaleDetectionState::UnchangedUnverified => {
                        DocsExampleValidationClass::RetestPending
                    }
                })
                .unwrap_or(DocsExampleValidationClass::NotApplicable),
            citation_drawer_ref: Some(self.citation_drawer_hook.hook_id.clone()),
            surface_refs: vec![format!(
                "surface:docs-linked-search:{}",
                sanitize_planner_id(&self.reference_id)
            )],
        })
    }

    fn knowledge_surface_projection(
        &self,
        provenance: &DocsNodeProvenance,
    ) -> DocsKnowledgeSurfaceProjection {
        DocsKnowledgeSurfaceProjection::new(DocsKnowledgeSurfaceProjectionInput {
            surface_kind: DocsKnowledgeSurfaceKind::DocsBackedSearch,
            surface_id_ref: self.reference_id.clone(),
            provenance: provenance.clone(),
            citation_inspection_action_ref: self.citation_drawer_hook.hook_id.clone(),
            open_supporting_source_action_ref: Some(format!(
                "action:docs-search-open-source:{}",
                sanitize_planner_id(&self.reference_id)
            )),
            keyboard_accessible_actions: true,
            export_packet_refs: vec![format!(
                "docs-evidence-packet:docs-linked-search:{}",
                sanitize_planner_id(&self.reference_id)
            )],
        })
    }
}

/// Inputs for one docs-linked search projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkedSearchInputs {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Query session that owns this docs search.
    pub query_session: SearchQuerySession,
    /// Planner pass id to emit.
    pub planner_pass_id: String,
    /// Result-set id to emit.
    pub result_set_id: String,
    /// Docs path snapshot id to emit.
    pub snapshot_id: String,
    /// Timestamp for fixture and support parity.
    pub observed_at: String,
    /// Path readiness for the docs lane.
    pub readiness: PlannerPathReadiness,
    /// Path freshness for the docs lane.
    pub freshness: PlannerFreshnessClass,
    /// Index epoch for docs search, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_epoch: Option<String>,
    /// Graph epoch used to bind symbols to docs, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Symbol-linked docs references considered by this pass.
    pub references: Vec<DocsLinkedReference>,
}

impl DocsLinkedSearchInputs {
    /// Stable record-kind tag carried by docs-linked search inputs.
    pub const RECORD_KIND: &'static str = "docs_linked_search_input";
}

/// Docs-linked search projection with planner rows and evidence sidecars.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkedSearchProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Query session after planner projection.
    pub query_session: SearchQuerySession,
    /// Planned result set shared with search surfaces.
    pub result_set: PlannedResultSet,
    /// Docs-linked sidecars keyed by planned result id.
    pub rows: Vec<DocsLinkedSearchResult>,
    /// Support/export projection over the same rows.
    pub support_export: DocsLinkingSupportExport,
}

impl DocsLinkedSearchProjection {
    /// Stable record-kind tag carried by projection records.
    pub const RECORD_KIND: &'static str = "docs_linked_search_projection";

    /// Builds a docs-linked projection and runs the shared search planner.
    pub fn from_inputs(inputs: DocsLinkedSearchInputs) -> Self {
        let mut query_session = inputs.query_session.clone();
        query_session.surface = SearchSurface::DocsSearch;

        let snapshot = PlannerPathSnapshot {
            path_kind: PlannerDataPath::Docs,
            snapshot_id: inputs.snapshot_id.clone(),
            readiness: inputs.readiness,
            freshness: inputs.freshness,
            index_epoch: inputs.index_epoch.clone(),
            graph_epoch: inputs.graph_epoch.clone(),
            unavailable_reason: inputs
                .references
                .is_empty()
                .then_some(PlannerUnavailableReason::DocsUnavailable),
            partial_truth_causes: Vec::new(),
            rows: inputs
                .references
                .iter()
                .map(DocsLinkedReference::to_planner_candidate)
                .collect(),
        };
        let reference_by_canonical = inputs
            .references
            .iter()
            .map(|reference| (reference.canonical_id(), reference))
            .collect::<BTreeMap<_, _>>();

        let output = SearchPlannerAlpha::plan(SearchPlannerInputs {
            query_session,
            planner_pass_id: inputs.planner_pass_id,
            result_set_id: inputs.result_set_id,
            planner_version: crate::SEARCH_PLANNER_ALPHA_VERSION.to_string(),
            observed_at: inputs.observed_at.clone(),
            path_snapshots: vec![snapshot],
        });

        let rows = output
            .result_set
            .rows
            .iter()
            .filter_map(|planned| {
                reference_by_canonical
                    .get(&planned.canonical_id)
                    .map(|reference| {
                        DocsLinkedSearchResult::from_planned_result(planned, reference)
                    })
            })
            .collect::<Vec<_>>();
        let support_export = DocsLinkingSupportExport::from_rows(
            format!("support-export:{}", output.result_set.result_set_id),
            output.query_session.query_session_id.clone(),
            output.result_set.result_set_id.clone(),
            inputs.observed_at,
            &rows,
        );

        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: DOCS_LINKING_ALPHA_SCHEMA_VERSION,
            query_session: output.query_session,
            result_set: output.result_set,
            rows,
            support_export,
        }
    }

    /// Validates the acceptance-critical docs-linking states.
    pub fn validate_acceptance(&self) -> Result<(), Vec<DocsLinkingValidationFinding>> {
        let mut findings = Vec::new();
        for row in &self.rows {
            if row.exact_anchor.anchor_id.trim().is_empty()
                || row.exact_anchor.exact_anchor_ref.trim().is_empty()
            {
                findings.push(DocsLinkingValidationFinding::new(
                    row.result_id.clone(),
                    "docs row is missing its exact anchor",
                ));
            }
            if !row.citation_drawer_hook.opens_evidence_view {
                findings.push(DocsLinkingValidationFinding::new(
                    row.result_id.clone(),
                    "docs row does not open a citation drawer or evidence view",
                ));
            }
            if !row.citation_evidence_view.validate().is_empty() {
                findings.push(DocsLinkingValidationFinding::new(
                    row.result_id.clone(),
                    "docs row citation evidence view failed canonical validation",
                ));
            }
            if !row.knowledge_surface_projection.validate().is_empty() {
                findings.push(DocsLinkingValidationFinding::new(
                    row.result_id.clone(),
                    "docs row knowledge-surface provenance failed canonical validation",
                ));
            }
            match row.citation_availability {
                DocsCitationAvailability::Available
                    if row.citation_drawer_hook.citation_anchor_refs.is_empty() =>
                {
                    findings.push(DocsLinkingValidationFinding::new(
                        row.result_id.clone(),
                        "available citation state has no citation anchors",
                    ));
                }
                DocsCitationAvailability::Missing
                    if row.missing_anchor_downgrade_state.is_none() =>
                {
                    findings.push(DocsLinkingValidationFinding::new(
                        row.result_id.clone(),
                        "missing citation state lacks a downgrade disclosure",
                    ));
                }
                _ => {}
            }
            if let Some(signal) = &row.stale_example_signal {
                if signal.open_failing_source_action.action_kind
                    != DocsEvidenceActionKind::OpenFailingSource
                {
                    findings.push(DocsLinkingValidationFinding::new(
                        row.result_id.clone(),
                        "stale-example signal lacks an open-failing-source action",
                    ));
                }
            }
            if let Some(suggestion) = &row.suggestion_card {
                if suggestion.target_branch.trim().is_empty()
                    || suggestion.target_channel.trim().is_empty()
                    || suggestion.evidence_actions.is_empty()
                {
                    findings.push(DocsLinkingValidationFinding::new(
                        row.result_id.clone(),
                        "docs suggestion lacks branch/channel/evidence actions",
                    ));
                }
            }
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Docs-linked sidecar for one planned search row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkedSearchResult {
    /// Planned result id reused by search surfaces.
    pub result_id: String,
    /// Canonical docs anchor id.
    pub canonical_id: String,
    /// Result title.
    pub title: String,
    /// Support-safe summary.
    pub summary: String,
    /// Product subject kind.
    pub subject_kind: DocsSubjectKind,
    /// Product subject ref.
    pub subject_ref: String,
    /// Symbol-linked reference id.
    pub reference_id: String,
    /// Exact anchor and source strip.
    pub exact_anchor: DocsExactAnchor,
    /// Source class token repeated for compact exports.
    pub source_class: DocsSourceClass,
    /// Doc kind token repeated for compact exports.
    pub doc_kind: DocsDocKind,
    /// Version match state.
    pub version_match_state: DocsVersionMatchState,
    /// Locality class.
    pub locality: DocsLocalityClass,
    /// Freshness class.
    pub freshness: DocsFreshnessClass,
    /// Citation availability state.
    pub citation_availability: DocsCitationAvailability,
    /// Citation drawer or evidence-view hook.
    pub citation_drawer_hook: DocsCitationDrawerHook,
    /// Canonical docs-node identity shared by docs/help, onboarding, graph, and AI.
    pub docs_node_identity: DocsNodeIdentity,
    /// Canonical citation drawer or equivalent evidence view.
    pub citation_evidence_view: CitationDrawerEvidenceView,
    /// Shared knowledge-surface provenance consumed by docs/search/help/AI/support.
    pub docs_node_provenance: DocsNodeProvenance,
    /// Surface projection preserving source-strip, citation, and open-source truth.
    pub knowledge_surface_projection: DocsKnowledgeSurfaceProjection,
    /// Project-docs versus vendor-docs precedence cue.
    pub project_vs_vendor_truth_cue: DocsProjectVendorTruthCue,
    /// Symbol-link resolution class.
    pub resolution_class: DocsLinkResolutionClass,
    /// Missing-anchor downgrade state, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_anchor_downgrade_state: Option<DocsLinkResolutionClass>,
    /// Stale-example signal, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_example_signal: Option<DocsStaleExampleSignal>,
    /// Docs suggestion card, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_card: Option<DocsSuggestionCard>,
    /// Ordered ranking reasons from the planner row.
    pub ranking_reason_classes: Vec<String>,
    /// Partial-truth causes from the planner row.
    pub partial_truth_causes: Vec<String>,
    /// Readiness state from the planner row.
    pub readiness_state: PlannerPathReadiness,
    /// Result truth class from the planner row.
    pub result_truth_class: PlannerResultTruthClass,
    /// Data path that answered this row.
    pub answered_by: PlannerDataPath,
}

impl DocsLinkedSearchResult {
    fn from_planned_result(planned: &PlannedSearchResult, reference: &DocsLinkedReference) -> Self {
        let docs_node_identity = reference.docs_node_identity();
        let docs_node_provenance = reference.docs_node_provenance(&docs_node_identity);
        let knowledge_surface_projection =
            reference.knowledge_surface_projection(&docs_node_provenance);
        Self {
            result_id: planned.result_id.clone(),
            canonical_id: planned.canonical_id.clone(),
            title: planned.title.clone(),
            summary: reference.summary.clone(),
            subject_kind: reference.subject_kind,
            subject_ref: reference.subject_ref.clone(),
            reference_id: reference.reference_id.clone(),
            exact_anchor: reference.exact_anchor.clone(),
            source_class: reference.exact_anchor.source_class,
            doc_kind: reference.exact_anchor.doc_kind,
            version_match_state: reference.exact_anchor.version_match_state,
            locality: reference.exact_anchor.locality,
            freshness: reference.exact_anchor.freshness,
            citation_availability: reference.exact_anchor.citation_availability,
            citation_drawer_hook: reference.citation_drawer_hook.clone(),
            docs_node_identity,
            citation_evidence_view: reference.citation_evidence_view(),
            docs_node_provenance,
            knowledge_surface_projection,
            project_vs_vendor_truth_cue: reference.project_vs_vendor_truth_cue,
            resolution_class: reference.resolution_class,
            missing_anchor_downgrade_state: reference
                .missing_anchor_downgrade
                .as_ref()
                .map(|downgrade| downgrade.downgrade_state),
            stale_example_signal: reference.stale_example_signal.clone(),
            suggestion_card: reference.suggestion_card.clone(),
            ranking_reason_classes: planned
                .ranking_reason_tokens()
                .into_iter()
                .map(str::to_string)
                .collect(),
            partial_truth_causes: planned.partial_truth_causes.clone(),
            readiness_state: planned.readiness_state,
            result_truth_class: planned.truth_class,
            answered_by: planned.answered_by,
        }
    }

    /// Returns true when this row must disclose a missing-anchor downgrade.
    pub const fn has_missing_anchor_downgrade(&self) -> bool {
        self.missing_anchor_downgrade_state.is_some()
    }
}

/// Support/export payload for docs-linked search rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkingSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support artifact id.
    pub support_artifact_id: String,
    /// Query session referenced by the rows.
    pub query_session_id_ref: String,
    /// Result set referenced by the rows.
    pub result_set_id_ref: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Planned result ids included in the export.
    pub result_ids: Vec<String>,
    /// Export-safe docs rows.
    pub rows: Vec<DocsLinkingSupportRow>,
}

impl DocsLinkingSupportExport {
    /// Stable record-kind tag carried by support exports.
    pub const RECORD_KIND: &'static str = "docs_linking_support_export";

    fn from_rows(
        support_artifact_id: String,
        query_session_id_ref: String,
        result_set_id_ref: String,
        generated_at: String,
        rows: &[DocsLinkedSearchResult],
    ) -> Self {
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: DOCS_LINKING_ALPHA_SCHEMA_VERSION,
            support_artifact_id,
            query_session_id_ref,
            result_set_id_ref,
            generated_at,
            result_ids: rows.iter().map(|row| row.result_id.clone()).collect(),
            rows: rows
                .iter()
                .map(DocsLinkingSupportRow::from_result)
                .collect(),
        }
    }
}

/// Export-safe docs row preserving source, citation, and freshness truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkingSupportRow {
    /// Planned result id.
    pub result_id: String,
    /// Product subject ref.
    pub subject_ref: String,
    /// Exact anchor ref.
    pub exact_anchor_ref: String,
    /// Canonical docs-node identity for support reconstruction.
    pub docs_node_identity: DocsNodeIdentity,
    /// Shared docs-node provenance for support reconstruction.
    pub docs_node_provenance: DocsNodeProvenance,
    /// Surface projection preserving source-strip and citation action truth.
    pub knowledge_surface_projection: DocsKnowledgeSurfaceProjection,
    /// Doc kind token.
    pub doc_kind_token: String,
    /// Source class token.
    pub source_class_token: String,
    /// Source version label.
    pub source_version: String,
    /// Locality token.
    pub locality_token: String,
    /// Freshness token.
    pub freshness_token: String,
    /// Version match token.
    pub version_match_token: String,
    /// Citation availability token.
    pub citation_availability_token: String,
    /// Citation drawer ref.
    pub citation_drawer_ref: String,
    /// Project/vendor precedence token.
    pub project_vs_vendor_truth_token: String,
    /// Missing-anchor downgrade token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_anchor_downgrade_token: Option<String>,
    /// Stale-example trigger token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_example_trigger_token: Option<String>,
    /// Stale-example validation freshness token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_example_validation_freshness_token: Option<String>,
    /// Suggestion trigger token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_trigger_token: Option<String>,
    /// Suggestion publish-boundary token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_publish_boundary_token: Option<String>,
    /// Ranking reasons preserved from the planner row.
    pub ranking_reason_classes: Vec<String>,
    /// Partial-truth cause tokens.
    pub partial_truth_causes: Vec<String>,
}

impl DocsLinkingSupportRow {
    fn from_result(row: &DocsLinkedSearchResult) -> Self {
        Self {
            result_id: row.result_id.clone(),
            subject_ref: row.subject_ref.clone(),
            exact_anchor_ref: row.exact_anchor.exact_anchor_ref.clone(),
            docs_node_identity: row.docs_node_identity.clone(),
            docs_node_provenance: row.docs_node_provenance.clone(),
            knowledge_surface_projection: row.knowledge_surface_projection.clone(),
            doc_kind_token: row.doc_kind.as_str().to_string(),
            source_class_token: row.source_class.as_str().to_string(),
            source_version: row.exact_anchor.source_version.clone(),
            locality_token: row.locality.as_str().to_string(),
            freshness_token: row.freshness.as_str().to_string(),
            version_match_token: row.version_match_state.as_str().to_string(),
            citation_availability_token: row.citation_availability.as_str().to_string(),
            citation_drawer_ref: row.citation_drawer_hook.exact_reopen_ref.clone(),
            project_vs_vendor_truth_token: row.project_vs_vendor_truth_cue.as_str().to_string(),
            missing_anchor_downgrade_token: row
                .missing_anchor_downgrade_state
                .map(|state| state.as_str().to_string()),
            stale_example_trigger_token: row
                .stale_example_signal
                .as_ref()
                .map(|signal| signal.trigger_class.as_str().to_string()),
            stale_example_validation_freshness_token: row
                .stale_example_signal
                .as_ref()
                .map(|signal| signal.validation_freshness.as_str().to_string()),
            suggestion_trigger_token: row
                .suggestion_card
                .as_ref()
                .map(|card| card.trigger_class.as_str().to_string()),
            suggestion_publish_boundary_token: row
                .suggestion_card
                .as_ref()
                .map(|card| card.publish_boundary_state.as_str().to_string()),
            ranking_reason_classes: row.ranking_reason_classes.clone(),
            partial_truth_causes: row.partial_truth_causes.clone(),
        }
    }
}

/// Validation finding for docs-linking acceptance checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsLinkingValidationFinding {
    /// Row or result id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl DocsLinkingValidationFinding {
    fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

fn push_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|existing| existing == value) {
        target.push(value.to_string());
    }
}

fn sanitize_planner_id(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}

fn default_source_build_at() -> String {
    "source-build:unknown".to_owned()
}

fn docs_node_kind_from_doc_kind(kind: DocsDocKind) -> DocsNodeKind {
    match kind {
        DocsDocKind::ApiReference | DocsDocKind::GeneratedReference | DocsDocKind::ReleaseNote => {
            DocsNodeKind::ReferencePage
        }
        DocsDocKind::Guide
        | DocsDocKind::Example
        | DocsDocKind::MigrationNote
        | DocsDocKind::HelpArticle => DocsNodeKind::ProductHelp,
        DocsDocKind::Runbook => DocsNodeKind::SupportRunbook,
        DocsDocKind::Glossary => DocsNodeKind::GlossaryItem,
    }
}

fn citation_source_class_from_docs(source_class: DocsSourceClass) -> CitationSourceClass {
    match source_class {
        DocsSourceClass::ProjectDocs => CitationSourceClass::ProjectDocs,
        DocsSourceClass::GeneratedReference => CitationSourceClass::GeneratedReference,
        DocsSourceClass::MirroredOfficialDocs => CitationSourceClass::MirroredOfficialDocs,
        DocsSourceClass::CuratedKnowledgePack => CitationSourceClass::CuratedKnowledgePack,
        DocsSourceClass::DerivedExplanation => CitationSourceClass::DerivedExplanation,
        DocsSourceClass::VendorProviderDocs | DocsSourceClass::ExternalStatusFeed => {
            CitationSourceClass::VendorProviderDocs
        }
        DocsSourceClass::SupportRunbook => CitationSourceClass::SupportRunbook,
    }
}

fn freshness_from_docs(freshness: DocsFreshnessClass) -> CanonicalDocsFreshnessClass {
    match freshness {
        DocsFreshnessClass::AuthoritativeLive => CanonicalDocsFreshnessClass::AuthoritativeLive,
        DocsFreshnessClass::WarmCached => CanonicalDocsFreshnessClass::WarmCached,
        DocsFreshnessClass::DegradedCached => CanonicalDocsFreshnessClass::DegradedCached,
        DocsFreshnessClass::Stale => CanonicalDocsFreshnessClass::Stale,
        DocsFreshnessClass::Unverified => CanonicalDocsFreshnessClass::Unverified,
    }
}

fn version_match_from_docs(version_match: DocsVersionMatchState) -> VersionMatchState {
    match version_match {
        DocsVersionMatchState::ExactBuildMatch => VersionMatchState::ExactBuildMatch,
        DocsVersionMatchState::CompatibleMinorDrift => VersionMatchState::CompatibleMinorDrift,
        DocsVersionMatchState::IncompatibleDriftDetected => {
            VersionMatchState::IncompatibleDriftDetected
        }
        DocsVersionMatchState::PreReleaseUnverified => VersionMatchState::PreReleaseUnverified,
        DocsVersionMatchState::UnknownTargetBuild => VersionMatchState::UnknownTargetBuild,
    }
}

fn locality_from_docs(locality: DocsLocalityClass) -> CitationLocalityClass {
    match locality {
        DocsLocalityClass::LocalProjectPack => CitationLocalityClass::LocalProjectPack,
        DocsLocalityClass::GeneratedLocal => CitationLocalityClass::GeneratedLocal,
        DocsLocalityClass::MirroredOffline => CitationLocalityClass::MirroredOffline,
        DocsLocalityClass::CachedLocal => CitationLocalityClass::CachedLocal,
        DocsLocalityClass::VendorLive => CitationLocalityClass::VendorLive,
        DocsLocalityClass::SupportPack => CitationLocalityClass::SupportPack,
    }
}

fn citation_availability_from_docs(
    availability: DocsCitationAvailability,
) -> CitationAnchorAvailability {
    match availability {
        DocsCitationAvailability::Available => CitationAnchorAvailability::ExactAnchorAvailable,
        DocsCitationAvailability::Missing => CitationAnchorAvailability::AnchorUnavailableDisclosed,
        DocsCitationAvailability::NotRequired => CitationAnchorAvailability::NotCitationBearing,
    }
}

fn docs_scope_from_subject(subject_kind: DocsSubjectKind) -> DocsScopeClass {
    match subject_kind {
        DocsSubjectKind::GlossaryTerm => DocsScopeClass::HelpPack,
        DocsSubjectKind::OnboardingStep => DocsScopeClass::Onboarding,
        DocsSubjectKind::RunbookStep | DocsSubjectKind::ServiceHealthEvent => {
            DocsScopeClass::SupportExport
        }
        _ => DocsScopeClass::DocsHelp,
    }
}

fn source_precedence_from_docs(cue: DocsProjectVendorTruthCue) -> SourcePrecedenceClass {
    match cue {
        DocsProjectVendorTruthCue::ProjectAuthoritativeOnly => {
            SourcePrecedenceClass::ProjectAuthoritativeOnly
        }
        DocsProjectVendorTruthCue::ProjectOutranksVendorDefault => {
            SourcePrecedenceClass::ProjectOutranksVendorDefault
        }
        DocsProjectVendorTruthCue::VendorOverridesProjectByPolicy
        | DocsProjectVendorTruthCue::VendorProviderOverlayInspectOnly => {
            SourcePrecedenceClass::VendorOverrideDisclosed
        }
        DocsProjectVendorTruthCue::NoProjectClaimVendorAvailable => {
            SourcePrecedenceClass::ProjectVendorDisagreementInspectable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ScopeClass, SEARCH_PLANNER_ALPHA_VERSION};

    fn exact_anchor() -> DocsExactAnchor {
        DocsExactAnchor {
            anchor_id: "docs:anchor:project:http-client-get".to_string(),
            exact_anchor_ref: "docs/api/http-client.md#get".to_string(),
            doc_kind: DocsDocKind::ApiReference,
            source_class: DocsSourceClass::ProjectDocs,
            pack_id: "pack:project:aureline:alpha".to_string(),
            pack_revision_ref: "pack-rev:project:aureline:alpha:2026.05.13".to_string(),
            source_version: "alpha-help-2026.05".to_string(),
            source_build_at: "2026-05-13T00:00:00Z".to_string(),
            locality: DocsLocalityClass::LocalProjectPack,
            freshness: DocsFreshnessClass::AuthoritativeLive,
            version_match_state: DocsVersionMatchState::ExactBuildMatch,
            citation_availability: DocsCitationAvailability::Available,
        }
    }

    fn drawer_hook() -> DocsCitationDrawerHook {
        DocsCitationDrawerHook {
            hook_id: "hook:citation:project:http-client-get".to_string(),
            availability: DocsCitationAvailability::Available,
            citation_anchor_refs: vec!["anchor:project:symbol:http::Client::get".to_string()],
            target_ref: "citation:project:http-client-get".to_string(),
            exact_reopen_ref: "reopen:citation:project:http-client-get".to_string(),
            opens_evidence_view: true,
        }
    }

    #[test]
    fn docs_projection_uses_planner_result_id_and_imported_truth() {
        let session = SearchQuerySession::for_local_text(
            "search:session:docs:unit",
            SearchSurface::DocsSearch,
            "http client get",
            ScopeClass::CurrentRepo,
            "Current repo",
            SEARCH_PLANNER_ALPHA_VERSION,
            "ready",
            "mono:docs:unit:01",
        );
        let projection = DocsLinkedSearchProjection::from_inputs(DocsLinkedSearchInputs {
            record_kind: DocsLinkedSearchInputs::RECORD_KIND.to_string(),
            schema_version: DOCS_LINKING_ALPHA_SCHEMA_VERSION,
            query_session: session,
            planner_pass_id: "search:planner:docs:unit".to_string(),
            result_set_id: "search:result_set:docs:unit".to_string(),
            snapshot_id: "search:snapshot:docs:unit".to_string(),
            observed_at: "mono:docs:unit:02".to_string(),
            readiness: PlannerPathReadiness::Ready,
            freshness: PlannerFreshnessClass::AuthoritativeLive,
            index_epoch: Some("docs-index:unit".to_string()),
            graph_epoch: Some("graph:unit".to_string()),
            references: vec![DocsLinkedReference {
                reference_id: "symref:project:http-client-get".to_string(),
                subject_kind: DocsSubjectKind::Symbol,
                subject_ref: "symbol:project:http::Client::get".to_string(),
                display_label: "http::Client::get".to_string(),
                summary: "Project docs for the HTTP client get method.".to_string(),
                resolution_class: DocsLinkResolutionClass::ProjectDocsOutranksVendorMatch,
                resolution_fallback_chain: vec![
                    DocsLinkResolutionClass::ExactSymbolMatch,
                    DocsLinkResolutionClass::ProjectDocsOutranksVendorMatch,
                ],
                exact_anchor: exact_anchor(),
                project_vs_vendor_truth_cue:
                    DocsProjectVendorTruthCue::ProjectOutranksVendorDefault,
                derived_explanation_reuse_state: DocsDerivedReuseState::ReusableWithCitationAnchor,
                citation_drawer_hook: drawer_hook(),
                missing_anchor_downgrade: None,
                stale_example_signal: None,
                suggestion_card: None,
                repair_hook_ref: None,
                browser_handoff_reason: None,
            }],
        });

        projection
            .validate_acceptance()
            .expect("projection validates");
        let row = projection.rows.first().expect("docs row projected");
        assert_eq!(row.answered_by, PlannerDataPath::Docs);
        assert_eq!(row.result_truth_class, PlannerResultTruthClass::Imported);
        assert_eq!(
            row.result_id,
            "search:planned:docs_search:docs:anchor:project:http-client-get"
        );
        assert_eq!(
            row.ranking_reason_classes,
            vec![
                "docs_anchor_match",
                "docs_symbol_linked_reference",
                "docs_source_precedence",
                "citation_available",
            ]
        );
    }
}
