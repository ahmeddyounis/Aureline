//! Docs-pack manifest, mirror/offline, stale-example, and citation-set
//! truth packet.
//!
//! This module owns the stable contract that hardens four launch-critical
//! pillars on every claimed stable docs/help row:
//!
//! 1. **Docs-pack manifest contract** — Every pack carries pack id, signer
//!    identity, source channel, version range, refresh state, mirror source,
//!    pin state, and schema version. Offline import/export, quarantine,
//!    refresh, and stale-example flows all consume the same manifest;
//!    signer / channel / mirror-source identity stays attributable even when
//!    pack content is unavailable locally.
//! 2. **Mirror / offline truth** — Mirror chains, offline pins, refresh
//!    deadlines, and local-availability postures are pinned through closed
//!    enums and never collapse into a single generic "offline" badge.
//! 3. **Stale-example detection** — Findings keep the
//!    `nearby_version` / `stale_example` / `quarantined_pack` distinction
//!    visible and reviewable. Suppressions stay attributable: actor, reason,
//!    expiry, and evidence refs survive export, mirror, and release-packet
//!    reuse.
//! 4. **Citation-set export** — Derived explanations carry a citation-set
//!    object that preserves cited files, symbols, docs refs, graph epoch,
//!    locale, and derivation tool / version. Citation-set export works
//!    without bundling raw pack bodies by default and stays available to AI
//!    evidence, onboarding/help, and support-export lanes.
//!
//! The packet is metadata-only: no raw document bodies, no raw URLs, no
//! provider payloads, no ambient credentials. It is read verbatim by the
//! docs-browser shell, help pane, onboarding tour, AI context inspector,
//! CLI / headless emitter, support export, release proof index,
//! mirror / offline console, citation drawer, and stale-example review
//! surfaces.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`DocsPackTruthPacket`].
pub const DOCS_PACK_TRUTH_PACKET_RECORD_KIND: &str = "docs_pack_truth_packet";

/// Stable record-kind tag for [`DocsPackTruthSupportExport`].
pub const DOCS_PACK_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_pack_truth_support_export";

/// Integer schema version for the docs-pack truth packet.
pub const DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the contract doc.
pub const DOCS_PACK_TRUTH_PACKET_DOC_REF: &str = "docs/search/m4/docs_pack_truth_packet.md";

/// Repo-relative path of the milestone-level note.
pub const DOCS_PACK_TRUTH_PACKET_MILESTONE_DOC_REF: &str = "docs/m4/docs_pack_truth_packet.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const DOCS_PACK_TRUTH_PACKET_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/docs_pack_truth_packet.md";

/// Repo-relative path of the JSON schema.
pub const DOCS_PACK_TRUTH_PACKET_SCHEMA_REF: &str =
    "schemas/docs/docs_pack_truth_packet.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const DOCS_PACK_TRUTH_PACKET_FIXTURE_DIR: &str = "fixtures/search/m4/docs_pack_truth_packet";

/// Repo-relative path of the checked-in stable docs-pack truth packet.
pub const DOCS_PACK_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/docs_pack_truth_packet.json";

/// Closed source-class taxonomy for docs-pack manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackSourceClass {
    /// Workspace-owned project documentation.
    ProjectDocs,
    /// Generated reference built from source identity and the running build.
    GeneratedReference,
    /// Signed mirror of official vendor / framework / language docs.
    MirroredOfficialDocs,
    /// Curated knowledge pack (tutorials, glossaries, runbooks).
    CuratedKnowledgePack,
    /// Support-pipeline pack pinned for incident response.
    SupportRunbook,
    /// Extension-contributed docs pack.
    ExtensionDocsPack,
}

impl DocsPackSourceClass {
    /// Every required source class that must appear in a stable packet.
    pub const REQUIRED: [Self; 6] = [
        Self::ProjectDocs,
        Self::GeneratedReference,
        Self::MirroredOfficialDocs,
        Self::CuratedKnowledgePack,
        Self::SupportRunbook,
        Self::ExtensionDocsPack,
    ];

    /// Stable token used in fixtures, schema, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::SupportRunbook => "support_runbook",
            Self::ExtensionDocsPack => "extension_docs_pack",
        }
    }

    /// True when this source class is expected to mirror an upstream.
    pub const fn is_mirror_class(self) -> bool {
        matches!(self, Self::MirroredOfficialDocs)
    }
}

/// Closed signer-class taxonomy for docs-pack manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackSignerClass {
    /// Running project itself.
    FirstPartyProject,
    /// Third-party publisher the project has allow-listed.
    PermittedPublisher,
    /// Signed mirror of an upstream canonical source.
    OfficialUpstreamMirror,
    /// Pack an operator assembled for a deployment.
    OperatorCurated,
    /// Support-runbook pack pinned by the support pipeline.
    SupportPipeline,
}

impl DocsPackSignerClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyProject => "first_party_project",
            Self::PermittedPublisher => "permitted_publisher",
            Self::OfficialUpstreamMirror => "official_upstream_mirror",
            Self::OperatorCurated => "operator_curated",
            Self::SupportPipeline => "support_pipeline",
        }
    }
}

/// Closed signature-status taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackSignatureStatus {
    /// Pack signature verified end-to-end against the signing authority.
    SignedAndVerified,
    /// Pack carries a signature but verification has not completed.
    SignedButUnverified,
    /// Pack arrived without a signature.
    SignatureMissing,
    /// Pack signature was revoked.
    SignatureRevoked,
}

impl DocsPackSignatureStatus {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedAndVerified => "signed_and_verified",
            Self::SignedButUnverified => "signed_but_unverified",
            Self::SignatureMissing => "signature_missing",
            Self::SignatureRevoked => "signature_revoked",
        }
    }

    /// True when the signature cannot be used to back a stable claim.
    pub const fn is_publishable_blocker(self) -> bool {
        !matches!(self, Self::SignedAndVerified)
    }
}

/// Closed source-channel taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackChannel {
    /// Stable channel.
    Stable,
    /// Beta channel.
    Beta,
    /// Nightly channel.
    Nightly,
    /// Enterprise / managed channel.
    Enterprise,
}

impl DocsPackChannel {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Nightly => "nightly",
            Self::Enterprise => "enterprise",
        }
    }
}

/// Closed refresh-state taxonomy preserved across help surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRefreshState {
    /// Source was live and authoritative at mint time.
    AuthoritativeLive,
    /// Cached source remained within its freshness window.
    WarmCached,
    /// Cached source was usable only with degraded disclosure.
    DegradedCached,
    /// Source was stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is in flight and the current claim is provisional.
    RefreshPending,
}

impl DocsPackRefreshState {
    /// Every required refresh state a stable packet must exercise.
    pub const REQUIRED: [Self; 6] = [
        Self::AuthoritativeLive,
        Self::WarmCached,
        Self::DegradedCached,
        Self::Stale,
        Self::Unverified,
        Self::RefreshPending,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::RefreshPending => "refresh_pending",
        }
    }

    /// True when the manifest must carry a downgrade disclosure note.
    pub const fn lowers_certainty(self) -> bool {
        !matches!(self, Self::AuthoritativeLive)
    }
}

/// Closed mirror-state taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackMirrorState {
    /// Source class does not mirror upstream.
    NotApplicable,
    /// Mirror chain is continuous and every predecessor digest resolves.
    Continuous,
    /// Predecessor revision is missing from the mirror chain.
    PredecessorMissing,
    /// Signing chain is broken across mirror revisions.
    SigningChainBroken,
}

impl DocsPackMirrorState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Continuous => "continuous",
            Self::PredecessorMissing => "predecessor_missing",
            Self::SigningChainBroken => "signing_chain_broken",
        }
    }

    /// True when the state surfaces a mirror_continuity_broken disclosure.
    pub const fn is_continuity_broken(self) -> bool {
        matches!(self, Self::PredecessorMissing | Self::SigningChainBroken)
    }
}

/// Closed pin-state taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackPinState {
    /// Pack revision is pinned to an exact source revision.
    Pinned,
    /// Pack revision is not pinned and tracks the source channel head.
    Unpinned,
    /// Pack is pinned for offline use and never refreshes online.
    PinnedOffline,
    /// Pack is pinned within a declared compatibility window.
    PinnedCompatWindow,
}

impl DocsPackPinState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Unpinned => "unpinned",
            Self::PinnedOffline => "pinned_offline",
            Self::PinnedCompatWindow => "pinned_compat_window",
        }
    }
}

/// Closed local-availability taxonomy. Surfaces the difference between a
/// locally available pack, an offline-pinned pack, a missing pack, and a
/// quarantined pack instead of collapsing them into one "offline" badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackLocalAvailability {
    /// Pack content is available locally.
    AvailableLocal,
    /// Pack is pinned for offline use; content resolves from the offline pin.
    MirrorOfflinePinned,
    /// Pack is not installed in this workspace / instance.
    NotInstalled,
    /// Pack is unavailable; the surface must disclose the reason.
    UnavailableDisclosed,
    /// Pack is quarantined; rendering is denied but identity is preserved.
    Quarantined,
}

impl DocsPackLocalAvailability {
    /// Every required local-availability posture for a stable packet.
    pub const REQUIRED: [Self; 5] = [
        Self::AvailableLocal,
        Self::MirrorOfflinePinned,
        Self::NotInstalled,
        Self::UnavailableDisclosed,
        Self::Quarantined,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableLocal => "available_local",
            Self::MirrorOfflinePinned => "mirror_offline_pinned",
            Self::NotInstalled => "not_installed",
            Self::UnavailableDisclosed => "unavailable_disclosed",
            Self::Quarantined => "quarantined",
        }
    }

    /// True when content is unavailable locally but identity must still be
    /// preserved verbatim.
    pub const fn content_unavailable_locally(self) -> bool {
        matches!(
            self,
            Self::NotInstalled | Self::UnavailableDisclosed | Self::Quarantined
        )
    }
}

/// Closed publishable-state taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackPublishableState {
    /// Pack may render on stable surfaces.
    Publishable,
    /// Pack is still being authored.
    Draft,
    /// Pack is blocked by at least one publishable_blocking_reason.
    Blocked,
    /// Pack is superseded by a newer revision.
    Withdrawn,
    /// Pack is quarantined by policy or pipeline.
    Quarantined,
}

impl DocsPackPublishableState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publishable => "publishable",
            Self::Draft => "draft",
            Self::Blocked => "blocked",
            Self::Withdrawn => "withdrawn",
            Self::Quarantined => "quarantined",
        }
    }
}

/// Closed docs-render-mode taxonomy. Surfaces the difference between a
/// rendered preview, a syntax-checked block, an executed example, a
/// mirrored-only view, and a browser-handoff-only view instead of collapsing
/// them into one generic success badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsRenderMode {
    /// Content was rendered for the help surface (CommonMark / Markdown view).
    Rendered,
    /// Example was syntax-checked locally.
    SyntaxChecked,
    /// Example was executed locally.
    ExecutedLocally,
    /// Example was executed in a remote environment.
    ExecutedRemotely,
    /// Content rendered from a signed mirror only; no live source available.
    MirroredOnly,
    /// Surface can only hand off to an external browser; nothing renders inline.
    BrowserHandoffOnly,
    /// Content cannot be rendered (quarantined / not installed / withheld).
    NotRendered,
}

impl DocsRenderMode {
    /// Every required render mode a stable packet must exercise.
    pub const REQUIRED: [Self; 7] = [
        Self::Rendered,
        Self::SyntaxChecked,
        Self::ExecutedLocally,
        Self::ExecutedRemotely,
        Self::MirroredOnly,
        Self::BrowserHandoffOnly,
        Self::NotRendered,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rendered => "rendered",
            Self::SyntaxChecked => "syntax_checked",
            Self::ExecutedLocally => "executed_locally",
            Self::ExecutedRemotely => "executed_remotely",
            Self::MirroredOnly => "mirrored_only",
            Self::BrowserHandoffOnly => "browser_handoff_only",
            Self::NotRendered => "not_rendered",
        }
    }
}

/// Closed docs-validation result class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsValidationResultClass {
    /// Validation succeeded with no warnings.
    ValidatedClean,
    /// Validation succeeded with reviewable warnings.
    ValidatedWithWarnings,
    /// Validation observed a concrete failure.
    ValidatedFailed,
    /// Validation was not run.
    NotValidated,
    /// Validation is unsupported in the current environment.
    ValidationUnsupported,
}

impl DocsValidationResultClass {
    /// Every required result class a stable packet must exercise.
    pub const REQUIRED: [Self; 5] = [
        Self::ValidatedClean,
        Self::ValidatedWithWarnings,
        Self::ValidatedFailed,
        Self::NotValidated,
        Self::ValidationUnsupported,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidatedClean => "validated_clean",
            Self::ValidatedWithWarnings => "validated_with_warnings",
            Self::ValidatedFailed => "validated_failed",
            Self::NotValidated => "not_validated",
            Self::ValidationUnsupported => "validation_unsupported",
        }
    }
}

/// Closed stale-example finding-class vocabulary. Keeps the
/// `nearby_version` / `stale_example` / `quarantined_pack` distinction visible
/// rather than collapsing every drift into one warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleExampleFindingClass {
    /// Example targets a nearby compatible version; the row is usable with a
    /// disclosure note but is not a stale example.
    NearbyVersion,
    /// Example is stale at the active build / workspace revision.
    StaleExample,
    /// Containing pack is quarantined; the example must not render.
    QuarantinedPack,
    /// Linked target no longer resolves.
    BrokenLink,
    /// Drift has been detected but renderability has not been re-verified.
    NeedsReview,
    /// Required citation evidence is missing.
    MissingEvidence,
}

impl StaleExampleFindingClass {
    /// Every required finding class a stable packet must exercise.
    pub const REQUIRED: [Self; 3] = [
        Self::NearbyVersion,
        Self::StaleExample,
        Self::QuarantinedPack,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NearbyVersion => "nearby_version",
            Self::StaleExample => "stale_example",
            Self::QuarantinedPack => "quarantined_pack",
            Self::BrokenLink => "broken_link",
            Self::NeedsReview => "needs_review",
            Self::MissingEvidence => "missing_evidence",
        }
    }
}

/// Closed consumer surface that must inherit the docs-pack truth packet
/// verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackConsumerSurface {
    /// In-product docs-browser shell.
    DocsBrowserShell,
    /// In-product help pane.
    HelpPane,
    /// Onboarding / learning tour projection.
    OnboardingTour,
    /// AI context inspector projection.
    AiContextInspector,
    /// CLI / headless emitter.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Mirror / offline console projection.
    MirrorOfflineConsole,
    /// Citation drawer for derived explanations.
    CitationDrawer,
    /// Stale-example review surface.
    StaleExampleReview,
}

impl DocsPackConsumerSurface {
    /// Every required consumer surface in declaration order.
    pub const REQUIRED: [Self; 10] = [
        Self::DocsBrowserShell,
        Self::HelpPane,
        Self::OnboardingTour,
        Self::AiContextInspector,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::MirrorOfflineConsole,
        Self::CitationDrawer,
        Self::StaleExampleReview,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::HelpPane => "help_pane",
            Self::OnboardingTour => "onboarding_tour",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::MirrorOfflineConsole => "mirror_offline_console",
            Self::CitationDrawer => "citation_drawer",
            Self::StaleExampleReview => "stale_example_review",
        }
    }
}

/// Closed promotion state for the docs-pack truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackPromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet has reviewable findings and must remain narrowed below stable.
    NarrowedBelowStable,
    /// Packet has blocker findings and cannot publish on stable surfaces.
    BlocksStable,
}

impl DocsPackPromotionState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`DocsPackTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Packet identity refs are empty.
    MissingPacketIdentity,
    /// Packet declared no pack manifests.
    MissingPackManifests,
    /// Packet declared no citation sets.
    MissingCitationSets,
    /// Packet declared no stale-example findings.
    MissingStaleExampleFindings,
    /// A pack manifest field required for a stable claim is empty.
    PackManifestIncomplete,
    /// A pack manifest dropped its signer / channel / mirror-source identity
    /// even though local content is unavailable.
    PackIdentityLostWhenOffline,
    /// A pack manifest with a non-applicable mirror class still claimed a
    /// mirror state or vice versa.
    MirrorStateInconsistent,
    /// A pack manifest's publishable state disagreed with its blocking reasons
    /// or signature posture.
    PublishableStateInconsistent,
    /// A pack manifest's refresh state requires a disclosure note that is
    /// missing.
    RefreshDisclosureMissing,
    /// A stale-example finding collapsed the nearby-version / stale-example /
    /// quarantined-pack distinction into a single warning.
    StaleStateCollapsed,
    /// A stale-example finding cites a pack manifest that no manifest declared.
    StaleFindingPackRefUnpinned,
    /// A stale-example suppression dropped actor / reason / expiry / evidence
    /// attribution.
    SuppressionAttributionLost,
    /// A stale-example suppression dropped the pack source / version context
    /// that survives export, mirror, and release-packet reuse.
    SuppressionLostSourceVersion,
    /// A citation set bundled raw pack bodies by default.
    CitationSetBundlesRawPack,
    /// A citation set lost cited files, symbols, docs refs, graph epoch,
    /// locale, or derivation tool / version identity.
    CitationSetIdentityIncomplete,
    /// A citation set references a pack manifest that no manifest declared.
    CitationSetPackRefUnpinned,
    /// Required source-class coverage for a stable claim is missing.
    RequiredSourceClassCoverageMissing,
    /// Required render-mode coverage for a stable claim is missing.
    RequiredRenderModeCoverageMissing,
    /// Required local-availability coverage for a stable claim is missing.
    RequiredLocalAvailabilityCoverageMissing,
    /// A consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops or remints docs-pack truth.
    ConsumerProjectionDrift,
    /// A consumer projection drops the docs-render-mode vocabulary.
    RenderModeVocabularyDropped,
    /// A consumer projection drops the validation-result-class vocabulary.
    ValidationResultClassDropped,
    /// A consumer projection drops the stale-example finding-class vocabulary.
    StaleFindingClassDropped,
    /// A consumer projection drops the citation-set identity binding.
    CitationSetIdentityDropped,
    /// A row admits raw URLs, raw bodies, secrets, or provider payloads.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DocsPackFindingKind {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingPackManifests => "missing_pack_manifests",
            Self::MissingCitationSets => "missing_citation_sets",
            Self::MissingStaleExampleFindings => "missing_stale_example_findings",
            Self::PackManifestIncomplete => "pack_manifest_incomplete",
            Self::PackIdentityLostWhenOffline => "pack_identity_lost_when_offline",
            Self::MirrorStateInconsistent => "mirror_state_inconsistent",
            Self::PublishableStateInconsistent => "publishable_state_inconsistent",
            Self::RefreshDisclosureMissing => "refresh_disclosure_missing",
            Self::StaleStateCollapsed => "stale_state_collapsed",
            Self::StaleFindingPackRefUnpinned => "stale_finding_pack_ref_unpinned",
            Self::SuppressionAttributionLost => "suppression_attribution_lost",
            Self::SuppressionLostSourceVersion => "suppression_lost_source_version",
            Self::CitationSetBundlesRawPack => "citation_set_bundles_raw_pack",
            Self::CitationSetIdentityIncomplete => "citation_set_identity_incomplete",
            Self::CitationSetPackRefUnpinned => "citation_set_pack_ref_unpinned",
            Self::RequiredSourceClassCoverageMissing => "required_source_class_coverage_missing",
            Self::RequiredRenderModeCoverageMissing => "required_render_mode_coverage_missing",
            Self::RequiredLocalAvailabilityCoverageMissing => {
                "required_local_availability_coverage_missing"
            }
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RenderModeVocabularyDropped => "render_mode_vocabulary_dropped",
            Self::ValidationResultClassDropped => "validation_result_class_dropped",
            Self::StaleFindingClassDropped => "stale_finding_class_dropped",
            Self::CitationSetIdentityDropped => "citation_set_identity_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the docs-pack validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DocsPackFindingKind,
    /// Finding severity.
    pub severity: DocsPackFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DocsPackValidationFinding {
    fn new(
        finding_kind: DocsPackFindingKind,
        severity: DocsPackFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Compatibility version range bounding a pack manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackVersionRange {
    /// Minimum inclusive semver / revision the pack is authoritative for.
    pub min_inclusive_ref: String,
    /// Maximum inclusive semver / revision the pack is authoritative for.
    pub max_inclusive_ref: String,
}

impl DocsPackVersionRange {
    fn is_well_formed(&self) -> bool {
        !self.min_inclusive_ref.trim().is_empty() && !self.max_inclusive_ref.trim().is_empty()
    }
}

/// Mirror / offline lineage block. Preserves signer / channel / mirror-source
/// identity even when local pack content is unavailable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackMirrorLineage {
    /// Closed mirror state.
    pub mirror_state: DocsPackMirrorState,
    /// Opaque pin to the upstream pack id this manifest mirrors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_of_pack_id: Option<String>,
    /// Opaque pin to the upstream revision this mirror corresponds to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_revision_ref: Option<String>,
    /// Opaque pin to the immediate predecessor revision in the mirror chain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_revision_ref: Option<String>,
    /// Operator-supplied label for an air-gapped origin (no raw URLs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub air_gapped_origin_label: Option<String>,
    /// Offline expiration ISO timestamp; null for live-fetch packs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_expiration_at: Option<String>,
}

/// Signing block for a pack manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackSigningBlock {
    /// Closed signature status.
    pub signature_status: DocsPackSignatureStatus,
    /// Closed signer class.
    pub signer_class: DocsPackSignerClass,
    /// Opaque id of the signing authority record.
    pub signing_authority_ref: String,
    /// Opaque digest identifying the signing chain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_chain_digest: Option<String>,
}

impl DocsPackSigningBlock {
    fn is_well_formed(&self) -> bool {
        !self.signing_authority_ref.trim().is_empty()
    }
}

/// Docs-pack manifest. Carries every field offline import/export, quarantine,
/// refresh, and stale-example flows must consume from one record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackManifest {
    /// Stable pack id.
    pub pack_id: String,
    /// Pack revision ref (opaque).
    pub pack_revision_ref: String,
    /// Display label for help / about surfaces.
    pub display_label: String,
    /// Closed source class.
    pub source_class: DocsPackSourceClass,
    /// Closed source channel.
    pub source_channel: DocsPackChannel,
    /// Signing block (signer class + signature status + signing authority).
    pub signing: DocsPackSigningBlock,
    /// Compatibility version range the pack is authoritative for.
    pub version_range: DocsPackVersionRange,
    /// Closed refresh state.
    pub refresh_state: DocsPackRefreshState,
    /// Last refresh ISO timestamp; absent for packs that don't refresh
    /// independently of the running binary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_at: Option<String>,
    /// Mirror / offline lineage block.
    pub mirror_lineage: DocsPackMirrorLineage,
    /// Closed pin state.
    pub pin_state: DocsPackPinState,
    /// Closed local-availability posture.
    pub local_availability: DocsPackLocalAvailability,
    /// Closed publishable state.
    pub publishable_state: DocsPackPublishableState,
    /// Closed publishable blocking reasons; empty when publishable.
    #[serde(default)]
    pub publishable_blocking_reasons: Vec<String>,
    /// Integer manifest schema version.
    pub manifest_schema_version: u32,
    /// Disclosure note for refresh / mirror / availability postures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
    /// True when raw URLs, raw bodies, secrets, and provider payloads are
    /// excluded from this manifest projection.
    pub raw_boundary_material_excluded: bool,
}

impl DocsPackManifest {
    fn requires_disclosure(&self) -> bool {
        self.refresh_state.lowers_certainty()
            || self.local_availability.content_unavailable_locally()
            || self.mirror_lineage.mirror_state.is_continuity_broken()
    }

    fn has_disclosure(&self) -> bool {
        self.disclosure_note
            .as_deref()
            .map(|note| !note.trim().is_empty())
            .unwrap_or(false)
    }

    fn is_well_formed(&self) -> bool {
        !self.pack_id.trim().is_empty()
            && !self.pack_revision_ref.trim().is_empty()
            && !self.display_label.trim().is_empty()
            && self.signing.is_well_formed()
            && self.version_range.is_well_formed()
            && self.manifest_schema_version >= 1
    }

    fn preserves_offline_identity(&self) -> bool {
        // When content is unavailable locally, identity (signer / channel /
        // mirror-source) MUST stay attributable.
        let signer_present = !self.signing.signing_authority_ref.trim().is_empty();
        let channel_present = matches!(
            self.source_channel,
            DocsPackChannel::Stable
                | DocsPackChannel::Beta
                | DocsPackChannel::Nightly
                | DocsPackChannel::Enterprise
        );
        let mirror_present = if self.source_class.is_mirror_class() {
            self.mirror_lineage
                .mirror_of_pack_id
                .as_deref()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
        } else {
            true
        };
        signer_present && channel_present && mirror_present
    }
}

/// One stale-example suppression record. Preserves actor / reason / expiry /
/// evidence and the source / version context that must survive export,
/// mirror, and release-packet reuse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleExampleSuppression {
    /// Stable suppression id.
    pub suppression_id: String,
    /// Actor who recorded the suppression.
    pub actor_ref: String,
    /// Reviewable suppression reason.
    pub reason: String,
    /// Time after which the suppression lapses.
    pub expiry_at: String,
    /// Evidence refs backing the suppression decision.
    pub evidence_refs: Vec<String>,
    /// Pack source ref the suppression is bound to (must survive reuse).
    pub source_pack_id_ref: String,
    /// Pack revision ref the suppression is bound to (must survive reuse).
    pub source_pack_revision_ref: String,
}

impl StaleExampleSuppression {
    fn has_attribution(&self) -> bool {
        !self.suppression_id.trim().is_empty()
            && !self.actor_ref.trim().is_empty()
            && !self.reason.trim().is_empty()
            && !self.expiry_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self
                .evidence_refs
                .iter()
                .all(|value| !value.trim().is_empty())
    }

    fn preserves_source_version(&self) -> bool {
        !self.source_pack_id_ref.trim().is_empty()
            && !self.source_pack_revision_ref.trim().is_empty()
    }
}

/// Stale-example finding row. Carries the closed finding-class vocabulary,
/// the docs-render mode that exposed the drift, and the docs-validation
/// result class that observed it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleExampleFinding {
    /// Stable finding id.
    pub finding_id: String,
    /// Pack manifest id the finding belongs to.
    pub pack_id_ref: String,
    /// Pack revision ref the finding was observed against.
    pub pack_revision_ref: String,
    /// Closed finding class.
    pub finding_class: StaleExampleFindingClass,
    /// Closed render mode that surfaced the finding.
    pub render_mode: DocsRenderMode,
    /// Closed validation result class.
    pub validation_result_class: DocsValidationResultClass,
    /// Optional nearby-version ref (only meaningful for `nearby_version`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nearby_version_ref: Option<String>,
    /// Optional superseding example id when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseding_example_id: Option<String>,
    /// Suppression attached to this finding (when suppressed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression: Option<StaleExampleSuppression>,
    /// Evidence refs backing the finding.
    pub evidence_refs: Vec<String>,
}

impl StaleExampleFinding {
    fn is_well_formed(&self) -> bool {
        !self.finding_id.trim().is_empty()
            && !self.pack_id_ref.trim().is_empty()
            && !self.pack_revision_ref.trim().is_empty()
            && !self.evidence_refs.is_empty()
    }
}

/// Citation-set export object. Preserves cited files, symbols, docs refs,
/// graph epoch, locale, and derivation tool / version so explanations,
/// tours, and help cards cannot outlive or hide their citation basis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationSetExport {
    /// Stable citation-set id.
    pub citation_set_id: String,
    /// Derived-explanation id this citation set backs.
    pub derived_explanation_ref: String,
    /// Pack manifest ids the citations resolve through.
    pub source_pack_id_refs: Vec<String>,
    /// Cited file refs (opaque paths; no raw bodies).
    pub cited_file_refs: Vec<String>,
    /// Cited symbol refs.
    pub cited_symbol_refs: Vec<String>,
    /// Cited docs anchor / page refs.
    pub cited_docs_refs: Vec<String>,
    /// Graph epoch the citations were resolved against.
    pub graph_epoch_ref: String,
    /// Locale rendered by the citation set (BCP-47).
    pub locale: String,
    /// Derivation tool ref (which AI/derivation pipeline emitted the
    /// explanation).
    pub derivation_tool_ref: String,
    /// Derivation tool version.
    pub derivation_tool_version: String,
    /// True when the export excludes raw pack bodies (must be true on stable).
    pub raw_pack_bodies_excluded: bool,
    /// True when the export excludes raw URLs (must be true on stable).
    pub raw_urls_excluded: bool,
}

impl CitationSetExport {
    fn is_well_formed(&self) -> bool {
        !self.citation_set_id.trim().is_empty()
            && !self.derived_explanation_ref.trim().is_empty()
            && !self.source_pack_id_refs.is_empty()
            && !self.graph_epoch_ref.trim().is_empty()
            && !self.locale.trim().is_empty()
            && !self.derivation_tool_ref.trim().is_empty()
            && !self.derivation_tool_version.trim().is_empty()
            && (!self.cited_file_refs.is_empty()
                || !self.cited_symbol_refs.is_empty()
                || !self.cited_docs_refs.is_empty())
    }
}

/// Consumer projection proving a surface reads the same packet without
/// reinventing docs-pack truth locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: DocsPackConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the closed pack-manifest schema verbatim.
    pub preserves_pack_manifest: bool,
    /// True when the surface preserves the closed mirror / offline taxonomy verbatim.
    pub preserves_mirror_offline: bool,
    /// True when the surface preserves the closed pin / availability taxonomy verbatim.
    pub preserves_pin_availability: bool,
    /// True when the surface preserves the closed render-mode taxonomy verbatim.
    pub preserves_render_mode: bool,
    /// True when the surface preserves the closed validation-result-class taxonomy verbatim.
    pub preserves_validation_result_class: bool,
    /// True when the surface preserves the closed stale-example finding-class taxonomy verbatim.
    pub preserves_stale_finding_class: bool,
    /// True when the surface preserves the stale-example suppression attribution verbatim.
    pub preserves_suppression_attribution: bool,
    /// True when the surface preserves the citation-set identity verbatim.
    pub preserves_citation_set_identity: bool,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl DocsPackConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_pack_manifest
            && self.preserves_mirror_offline
            && self.preserves_pin_availability
            && self.preserves_render_mode
            && self.preserves_validation_result_class
            && self.preserves_stale_finding_class
            && self.preserves_suppression_attribution
            && self.preserves_citation_set_identity
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DocsPackTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow / surface id the packet describes.
    pub workflow_or_surface_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Docs-pack manifests.
    #[serde(default)]
    pub manifests: Vec<DocsPackManifest>,
    /// Stale-example findings.
    #[serde(default)]
    pub stale_example_findings: Vec<StaleExampleFinding>,
    /// Citation-set exports.
    #[serde(default)]
    pub citation_sets: Vec<CitationSetExport>,
    /// Consumer projections.
    #[serde(default)]
    pub consumer_projections: Vec<DocsPackConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Stable docs-pack truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow / surface id the packet describes.
    pub workflow_or_surface_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Docs-pack manifests.
    #[serde(default)]
    pub manifests: Vec<DocsPackManifest>,
    /// Stale-example findings.
    #[serde(default)]
    pub stale_example_findings: Vec<StaleExampleFinding>,
    /// Citation-set exports.
    #[serde(default)]
    pub citation_sets: Vec<CitationSetExport>,
    /// Consumer projections.
    #[serde(default)]
    pub consumer_projections: Vec<DocsPackConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: DocsPackPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<DocsPackValidationFinding>,
}

impl DocsPackTruthPacket {
    /// Materialize a packet and record derived validation findings.
    pub fn materialize(input: DocsPackTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_PACK_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            manifests: input.manifests,
            stale_example_findings: input.stale_example_findings,
            citation_sets: input.citation_sets,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: DocsPackPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validate the packet against stable invariants.
    pub fn validate(&self) -> Vec<DocsPackValidationFinding> {
        self.derived_findings(true)
    }

    /// True when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DocsPackFindingSeverity::Blocker)
    }

    /// True when at least one consumer projection preserves this packet for
    /// `surface`.
    pub fn has_projection_for(&self, surface: DocsPackConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique source-class tokens carried across pack manifests.
    pub fn source_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for manifest in &self.manifests {
            set.insert(manifest.source_class);
        }
        set.into_iter().map(DocsPackSourceClass::as_str).collect()
    }

    /// Returns the unique render-mode tokens carried across stale-example
    /// findings.
    pub fn render_mode_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for finding in &self.stale_example_findings {
            set.insert(finding.render_mode);
        }
        set.into_iter().map(DocsRenderMode::as_str).collect()
    }

    /// Returns the unique stale-finding-class tokens carried across
    /// stale-example findings.
    pub fn stale_finding_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for finding in &self.stale_example_findings {
            set.insert(finding.finding_class);
        }
        set.into_iter()
            .map(StaleExampleFindingClass::as_str)
            .collect()
    }

    /// Build a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DocsPackTruthSupportExport {
        DocsPackTruthSupportExport {
            record_kind: DOCS_PACK_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn pack_id_set(&self) -> BTreeSet<&str> {
        self.manifests.iter().map(|m| m.pack_id.as_str()).collect()
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<DocsPackValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != DOCS_PACK_TRUTH_PACKET_RECORD_KIND {
            findings.push(DocsPackValidationFinding::new(
                DocsPackFindingKind::WrongRecordKind,
                DocsPackFindingSeverity::Blocker,
                "docs-pack truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION {
            findings.push(DocsPackValidationFinding::new(
                DocsPackFindingKind::WrongSchemaVersion,
                DocsPackFindingSeverity::Blocker,
                "docs-pack truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(DocsPackValidationFinding::new(
                DocsPackFindingKind::MissingPacketIdentity,
                DocsPackFindingSeverity::Blocker,
                "packet id, workflow id, and capture timestamp are required",
            ));
        }
        if self.manifests.is_empty() {
            findings.push(DocsPackValidationFinding::new(
                DocsPackFindingKind::MissingPackManifests,
                DocsPackFindingSeverity::Blocker,
                "packet must declare at least one docs-pack manifest",
            ));
        }
        if self.citation_sets.is_empty() {
            findings.push(DocsPackValidationFinding::new(
                DocsPackFindingKind::MissingCitationSets,
                DocsPackFindingSeverity::Blocker,
                "packet must declare at least one citation set",
            ));
        }
        if self.stale_example_findings.is_empty() {
            findings.push(DocsPackValidationFinding::new(
                DocsPackFindingKind::MissingStaleExampleFindings,
                DocsPackFindingSeverity::Blocker,
                "packet must declare at least one stale-example finding",
            ));
        }

        let pack_id_set = self.pack_id_set();

        for manifest in &self.manifests {
            if !manifest.is_well_formed() {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::PackManifestIncomplete,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "pack manifest {} drops a required identity field",
                        manifest.pack_id
                    ),
                ));
            }
            if !manifest.raw_boundary_material_excluded {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RawBoundaryMaterialPresent,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "pack manifest {} admits raw boundary material",
                        manifest.pack_id
                    ),
                ));
            }
            // Mirror-state consistency.
            if manifest.source_class.is_mirror_class()
                && manifest.mirror_lineage.mirror_state == DocsPackMirrorState::NotApplicable
            {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::MirrorStateInconsistent,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "pack manifest {} mirrors upstream but declared mirror_state = not_applicable",
                        manifest.pack_id
                    ),
                ));
            }
            if !manifest.source_class.is_mirror_class()
                && manifest.mirror_lineage.mirror_state != DocsPackMirrorState::NotApplicable
            {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::MirrorStateInconsistent,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "pack manifest {} does not mirror upstream but declared a mirror state",
                        manifest.pack_id
                    ),
                ));
            }
            // Publishable-state consistency.
            match manifest.publishable_state {
                DocsPackPublishableState::Publishable => {
                    if !manifest.publishable_blocking_reasons.is_empty() {
                        findings.push(DocsPackValidationFinding::new(
                            DocsPackFindingKind::PublishableStateInconsistent,
                            DocsPackFindingSeverity::Blocker,
                            format!(
                                "pack manifest {} is publishable but lists blocking reasons",
                                manifest.pack_id
                            ),
                        ));
                    }
                    if manifest.signing.signature_status.is_publishable_blocker() {
                        findings.push(DocsPackValidationFinding::new(
                            DocsPackFindingKind::PublishableStateInconsistent,
                            DocsPackFindingSeverity::Blocker,
                            format!(
                                "pack manifest {} is publishable but signature status {} blocks publication",
                                manifest.pack_id,
                                manifest.signing.signature_status.as_str()
                            ),
                        ));
                    }
                }
                DocsPackPublishableState::Blocked => {
                    if manifest.publishable_blocking_reasons.is_empty() {
                        findings.push(DocsPackValidationFinding::new(
                            DocsPackFindingKind::PublishableStateInconsistent,
                            DocsPackFindingSeverity::Blocker,
                            format!(
                                "pack manifest {} is blocked but lists no blocking reasons",
                                manifest.pack_id
                            ),
                        ));
                    }
                }
                _ => {}
            }
            // Refresh disclosure required for non-authoritative-live states.
            if manifest.requires_disclosure() && !manifest.has_disclosure() {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RefreshDisclosureMissing,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "pack manifest {} requires a refresh / mirror / availability disclosure note",
                        manifest.pack_id
                    ),
                ));
            }
            // Identity preserved even when content is offline / unavailable /
            // quarantined.
            if manifest.local_availability.content_unavailable_locally()
                && !manifest.preserves_offline_identity()
            {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::PackIdentityLostWhenOffline,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "pack manifest {} drops signer / channel / mirror-source identity even though content is unavailable locally",
                        manifest.pack_id
                    ),
                ));
            }
        }

        for finding in &self.stale_example_findings {
            if !finding.is_well_formed() {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::StaleStateCollapsed,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "stale-example finding {} drops a required identity field",
                        finding.finding_id
                    ),
                ));
            }
            if !pack_id_set.contains(finding.pack_id_ref.as_str()) {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::StaleFindingPackRefUnpinned,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "stale-example finding {} references unpinned pack {}",
                        finding.finding_id, finding.pack_id_ref
                    ),
                ));
            }
            // Nearby-version findings must carry a nearby_version_ref.
            if finding.finding_class == StaleExampleFindingClass::NearbyVersion
                && finding
                    .nearby_version_ref
                    .as_deref()
                    .map(|value| value.trim().is_empty())
                    .unwrap_or(true)
            {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::StaleStateCollapsed,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "nearby-version finding {} drops its nearby_version_ref and collapses into a generic stale warning",
                        finding.finding_id
                    ),
                ));
            }
            // Quarantined-pack findings must reference a quarantined manifest.
            if finding.finding_class == StaleExampleFindingClass::QuarantinedPack {
                let matched = self
                    .manifests
                    .iter()
                    .find(|manifest| manifest.pack_id == finding.pack_id_ref)
                    .map(|manifest| {
                        manifest.publishable_state == DocsPackPublishableState::Quarantined
                            || manifest.local_availability == DocsPackLocalAvailability::Quarantined
                    })
                    .unwrap_or(false);
                if !matched {
                    findings.push(DocsPackValidationFinding::new(
                        DocsPackFindingKind::StaleStateCollapsed,
                        DocsPackFindingSeverity::Blocker,
                        format!(
                            "quarantined-pack finding {} references pack {} which is not quarantined",
                            finding.finding_id, finding.pack_id_ref
                        ),
                    ));
                }
            }
            // Suppression attribution + source/version context must survive.
            if let Some(suppression) = &finding.suppression {
                if !suppression.has_attribution() {
                    findings.push(DocsPackValidationFinding::new(
                        DocsPackFindingKind::SuppressionAttributionLost,
                        DocsPackFindingSeverity::Blocker,
                        format!(
                            "stale-example finding {} suppressed without actor / reason / expiry / evidence attribution",
                            finding.finding_id
                        ),
                    ));
                }
                if !suppression.preserves_source_version() {
                    findings.push(DocsPackValidationFinding::new(
                        DocsPackFindingKind::SuppressionLostSourceVersion,
                        DocsPackFindingSeverity::Blocker,
                        format!(
                            "stale-example finding {} suppression lost pack source / version context",
                            finding.finding_id
                        ),
                    ));
                }
            }
        }

        for citation_set in &self.citation_sets {
            if !citation_set.is_well_formed() {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::CitationSetIdentityIncomplete,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "citation set {} drops cited files / symbols / docs refs / graph epoch / locale / derivation tool / version identity",
                        citation_set.citation_set_id
                    ),
                ));
            }
            if !citation_set.raw_pack_bodies_excluded {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::CitationSetBundlesRawPack,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "citation set {} bundles raw pack bodies; export must stay reference-only by default",
                        citation_set.citation_set_id
                    ),
                ));
            }
            if !citation_set.raw_urls_excluded {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RawBoundaryMaterialPresent,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "citation set {} admits raw URLs",
                        citation_set.citation_set_id
                    ),
                ));
            }
            for source_ref in &citation_set.source_pack_id_refs {
                if !pack_id_set.contains(source_ref.as_str()) {
                    findings.push(DocsPackValidationFinding::new(
                        DocsPackFindingKind::CitationSetPackRefUnpinned,
                        DocsPackFindingSeverity::Blocker,
                        format!(
                            "citation set {} references unpinned pack {}",
                            citation_set.citation_set_id, source_ref
                        ),
                    ));
                }
            }
        }

        // Required source-class coverage on stable claims.
        let observed_source_classes: BTreeSet<DocsPackSourceClass> = self
            .manifests
            .iter()
            .map(|manifest| manifest.source_class)
            .collect();
        for required in DocsPackSourceClass::REQUIRED {
            if !observed_source_classes.contains(&required) {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RequiredSourceClassCoverageMissing,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "packet does not cover required source class {}",
                        required.as_str()
                    ),
                ));
            }
        }

        // Required render-mode coverage on stable claims.
        let observed_render_modes: BTreeSet<DocsRenderMode> = self
            .stale_example_findings
            .iter()
            .map(|finding| finding.render_mode)
            .collect();
        for required in DocsRenderMode::REQUIRED {
            if !observed_render_modes.contains(&required) {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RequiredRenderModeCoverageMissing,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "packet does not cover required render mode {}",
                        required.as_str()
                    ),
                ));
            }
        }

        // Required local-availability coverage on stable claims.
        let observed_availability: BTreeSet<DocsPackLocalAvailability> = self
            .manifests
            .iter()
            .map(|manifest| manifest.local_availability)
            .collect();
        for required in DocsPackLocalAvailability::REQUIRED {
            if !observed_availability.contains(&required) {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RequiredLocalAvailabilityCoverageMissing,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "packet does not cover required local-availability posture {}",
                        required.as_str()
                    ),
                ));
            }
        }

        for required_surface in DocsPackConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::MissingConsumerProjection,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::ConsumerProjectionDrift,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve docs-pack truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_render_mode {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::RenderModeVocabularyDropped,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the docs-render-mode vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_validation_result_class {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::ValidationResultClassDropped,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the validation-result-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_stale_finding_class {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::StaleFindingClassDropped,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the stale-example finding-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_citation_set_identity {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::CitationSetIdentityDropped,
                    DocsPackFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the citation-set identity binding",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != DocsPackFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(DocsPackValidationFinding::new(
                    DocsPackFindingKind::PromotionStateMismatch,
                    DocsPackFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[DocsPackValidationFinding]) -> DocsPackPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DocsPackFindingSeverity::Blocker)
    {
        DocsPackPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DocsPackFindingSeverity::Warning)
    {
        DocsPackPromotionState::NarrowedBelowStable
    } else {
        DocsPackPromotionState::Stable
    }
}

/// Support-export wrapper preserving the product docs-pack packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: DocsPackTruthPacket,
}

impl DocsPackTruthSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_PACK_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable docs-pack packet.
#[derive(Debug)]
pub enum DocsPackTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<DocsPackValidationFinding>),
}

impl fmt::Display for DocsPackTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "docs-pack truth packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs-pack truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsPackTruthArtifactError {}

/// Returns the constructor input for the seeded stable docs-pack truth packet.
pub fn seeded_stable_docs_pack_truth_packet_input() -> DocsPackTruthPacketInput {
    seed::seeded_stable_input()
}

/// Materialize the checked-in stable docs-pack truth packet from the seed.
///
/// # Errors
///
/// Returns an artifact error if the materialized packet fails validation.
pub fn current_stable_docs_pack_truth_packet(
) -> Result<DocsPackTruthPacket, DocsPackTruthArtifactError> {
    let packet = DocsPackTruthPacket::materialize(seeded_stable_docs_pack_truth_packet_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DocsPackTruthArtifactError::Validation(findings))
    }
}

mod seed {
    use super::*;

    pub(super) fn seeded_stable_input() -> DocsPackTruthPacketInput {
        let packet_id = "packet:m4:docs_pack_truth:stable".to_owned();

        let manifests = vec![
            project_docs_manifest(),
            generated_reference_manifest(),
            mirrored_official_docs_manifest(),
            curated_knowledge_pack_manifest(),
            support_runbook_manifest(),
            extension_docs_pack_manifest(),
            quarantined_extension_manifest(),
        ];

        let stale_example_findings = vec![
            nearby_version_finding(),
            stale_example_finding(),
            quarantined_pack_finding(),
            broken_link_finding(),
            needs_review_finding(),
            missing_evidence_finding(),
            executed_remotely_finding(),
            browser_handoff_only_finding(),
            not_rendered_finding(),
        ];

        let citation_sets = vec![
            project_docs_citation_set(),
            mirrored_docs_citation_set(),
            ai_evidence_citation_set(),
        ];

        let consumer_projections = DocsPackConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| DocsPackConsumerProjection {
                consumer_surface: surface,
                projection_ref: format!("projection:docs_pack_truth:{}", surface.as_str()),
                packet_id_ref: packet_id.clone(),
                rendered_at: "2026-05-26T12:00:03Z".to_owned(),
                preserves_pack_manifest: true,
                preserves_mirror_offline: true,
                preserves_pin_availability: true,
                preserves_render_mode: true,
                preserves_validation_result_class: true,
                preserves_stale_finding_class: true,
                preserves_suppression_attribution: true,
                preserves_citation_set_identity: true,
                preserves_same_packet: true,
                supports_json_export: true,
                raw_private_material_excluded: true,
                ambient_authority_excluded: true,
            })
            .collect();

        DocsPackTruthPacketInput {
            packet_id,
            workflow_or_surface_id: "workflow.docs.pack_truth.stable".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            manifests,
            stale_example_findings,
            citation_sets,
            consumer_projections,
            source_contract_refs: vec![
                DOCS_PACK_TRUTH_PACKET_DOC_REF.to_owned(),
                DOCS_PACK_TRUTH_PACKET_ARTIFACT_DOC_REF.to_owned(),
                DOCS_PACK_TRUTH_PACKET_SCHEMA_REF.to_owned(),
                DOCS_PACK_TRUTH_PACKET_FIXTURE_DIR.to_owned(),
            ],
        }
    }

    fn project_docs_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:project-docs:aureline-workspace".to_owned(),
            pack_revision_ref: "rev:project-docs:aureline-workspace@2026.05.26".to_owned(),
            display_label: "Aureline workspace project docs".to_owned(),
            source_class: DocsPackSourceClass::ProjectDocs,
            source_channel: DocsPackChannel::Stable,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: DocsPackSignerClass::FirstPartyProject,
                signing_authority_ref: "signing-authority:aureline:first-party".to_owned(),
                signing_chain_digest: Some(
                    "digest:project-docs:aureline-workspace:2026.05.26".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "rev:aureline:2026.05.01".to_owned(),
                max_inclusive_ref: "rev:aureline:2026.05.30".to_owned(),
            },
            refresh_state: DocsPackRefreshState::AuthoritativeLive,
            last_refresh_at: Some("2026-05-26T11:30:00Z".to_owned()),
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::NotApplicable,
                mirror_of_pack_id: None,
                upstream_revision_ref: None,
                predecessor_revision_ref: None,
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: DocsPackPinState::Pinned,
            local_availability: DocsPackLocalAvailability::AvailableLocal,
            publishable_state: DocsPackPublishableState::Publishable,
            publishable_blocking_reasons: Vec::new(),
            manifest_schema_version: 1,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn generated_reference_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:generated-reference:aureline-api".to_owned(),
            pack_revision_ref: "rev:generated-reference:aureline-api@2026.05.26".to_owned(),
            display_label: "Aureline generated API reference".to_owned(),
            source_class: DocsPackSourceClass::GeneratedReference,
            source_channel: DocsPackChannel::Stable,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: DocsPackSignerClass::FirstPartyProject,
                signing_authority_ref: "signing-authority:aureline:first-party".to_owned(),
                signing_chain_digest: Some(
                    "digest:generated-reference:aureline-api:2026.05.26".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "rev:aureline:2026.05.26".to_owned(),
                max_inclusive_ref: "rev:aureline:2026.05.26".to_owned(),
            },
            refresh_state: DocsPackRefreshState::AuthoritativeLive,
            last_refresh_at: Some("2026-05-26T11:45:00Z".to_owned()),
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::NotApplicable,
                mirror_of_pack_id: None,
                upstream_revision_ref: None,
                predecessor_revision_ref: None,
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: DocsPackPinState::Pinned,
            local_availability: DocsPackLocalAvailability::AvailableLocal,
            publishable_state: DocsPackPublishableState::Publishable,
            publishable_blocking_reasons: Vec::new(),
            manifest_schema_version: 1,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn mirrored_official_docs_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:mirrored-official:rust-std".to_owned(),
            pack_revision_ref: "rev:mirrored-official:rust-std@1.78.0".to_owned(),
            display_label: "Rust std (mirrored)".to_owned(),
            source_class: DocsPackSourceClass::MirroredOfficialDocs,
            source_channel: DocsPackChannel::Stable,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: DocsPackSignerClass::OfficialUpstreamMirror,
                signing_authority_ref: "signing-authority:rust-foundation:mirror"
                    .to_owned(),
                signing_chain_digest: Some(
                    "digest:mirrored-official:rust-std:1.78.0".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "1.77.0".to_owned(),
                max_inclusive_ref: "1.79.0".to_owned(),
            },
            refresh_state: DocsPackRefreshState::WarmCached,
            last_refresh_at: Some("2026-05-26T09:00:00Z".to_owned()),
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::Continuous,
                mirror_of_pack_id: Some("pack:upstream:rust-std".to_owned()),
                upstream_revision_ref: Some("upstream:rust-std@1.78.0".to_owned()),
                predecessor_revision_ref: Some(
                    "rev:mirrored-official:rust-std@1.77.0".to_owned(),
                ),
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: DocsPackPinState::PinnedCompatWindow,
            local_availability: DocsPackLocalAvailability::MirrorOfflinePinned,
            publishable_state: DocsPackPublishableState::Publishable,
            publishable_blocking_reasons: Vec::new(),
            manifest_schema_version: 1,
            disclosure_note: Some(
                "Cached mirror — last refreshed 2026-05-26 09:00 UTC; pinned within 1.77.0..1.79.0 compat window."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn curated_knowledge_pack_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:curated:onboarding-and-runbooks".to_owned(),
            pack_revision_ref: "rev:curated:onboarding-and-runbooks@2026.05.20".to_owned(),
            display_label: "Curated onboarding and runbooks".to_owned(),
            source_class: DocsPackSourceClass::CuratedKnowledgePack,
            source_channel: DocsPackChannel::Beta,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: DocsPackSignerClass::OperatorCurated,
                signing_authority_ref: "signing-authority:aureline:operator-pipeline"
                    .to_owned(),
                signing_chain_digest: Some(
                    "digest:curated:onboarding-and-runbooks:2026.05.20".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "rev:aureline:2026.05.01".to_owned(),
                max_inclusive_ref: "rev:aureline:2026.05.30".to_owned(),
            },
            refresh_state: DocsPackRefreshState::DegradedCached,
            last_refresh_at: Some("2026-05-25T08:00:00Z".to_owned()),
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::NotApplicable,
                mirror_of_pack_id: None,
                upstream_revision_ref: None,
                predecessor_revision_ref: None,
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: DocsPackPinState::Pinned,
            local_availability: DocsPackLocalAvailability::AvailableLocal,
            publishable_state: DocsPackPublishableState::Publishable,
            publishable_blocking_reasons: Vec::new(),
            manifest_schema_version: 1,
            disclosure_note: Some(
                "Curated pack cached more than 24h ago; refresh recommended before publish handoff."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn support_runbook_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:support-runbook:incident-response".to_owned(),
            pack_revision_ref: "rev:support-runbook:incident-response@2026.05.18".to_owned(),
            display_label: "Support runbooks — incident response".to_owned(),
            source_class: DocsPackSourceClass::SupportRunbook,
            source_channel: DocsPackChannel::Enterprise,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: DocsPackSignerClass::SupportPipeline,
                signing_authority_ref: "signing-authority:aureline:support-pipeline"
                    .to_owned(),
                signing_chain_digest: Some(
                    "digest:support-runbook:incident-response:2026.05.18".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "rev:aureline:2026.04.01".to_owned(),
                max_inclusive_ref: "rev:aureline:2026.06.30".to_owned(),
            },
            refresh_state: DocsPackRefreshState::Stale,
            last_refresh_at: Some("2026-05-10T00:00:00Z".to_owned()),
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::NotApplicable,
                mirror_of_pack_id: None,
                upstream_revision_ref: None,
                predecessor_revision_ref: None,
                air_gapped_origin_label: Some(
                    "support-distribution-2026-05".to_owned(),
                ),
                offline_expiration_at: Some("2026-06-30T00:00:00Z".to_owned()),
            },
            pin_state: DocsPackPinState::PinnedOffline,
            local_availability: DocsPackLocalAvailability::UnavailableDisclosed,
            publishable_state: DocsPackPublishableState::Publishable,
            publishable_blocking_reasons: Vec::new(),
            manifest_schema_version: 1,
            disclosure_note: Some(
                "Support runbook pinned offline; pack is stale (last refreshed 2026-05-10) and content body is unavailable until the next pipeline import."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn extension_docs_pack_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:extension-docs:python-stdlib".to_owned(),
            pack_revision_ref: "rev:extension-docs:python-stdlib@3.12.4".to_owned(),
            display_label: "Python stdlib (extension pack)".to_owned(),
            source_class: DocsPackSourceClass::ExtensionDocsPack,
            source_channel: DocsPackChannel::Stable,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: DocsPackSignerClass::PermittedPublisher,
                signing_authority_ref: "signing-authority:python-extension:permitted"
                    .to_owned(),
                signing_chain_digest: Some(
                    "digest:extension-docs:python-stdlib:3.12.4".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "3.12.0".to_owned(),
                max_inclusive_ref: "3.12.4".to_owned(),
            },
            refresh_state: DocsPackRefreshState::Unverified,
            last_refresh_at: None,
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::NotApplicable,
                mirror_of_pack_id: None,
                upstream_revision_ref: None,
                predecessor_revision_ref: None,
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: DocsPackPinState::Pinned,
            local_availability: DocsPackLocalAvailability::NotInstalled,
            publishable_state: DocsPackPublishableState::Publishable,
            publishable_blocking_reasons: Vec::new(),
            manifest_schema_version: 1,
            disclosure_note: Some(
                "Extension pack is allow-listed and signed but is not installed locally; signer / channel / mirror-source identity is preserved for handoff."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn quarantined_extension_manifest() -> DocsPackManifest {
        DocsPackManifest {
            pack_id: "pack:extension-docs:experimental-go".to_owned(),
            pack_revision_ref: "rev:extension-docs:experimental-go@0.1.0".to_owned(),
            display_label: "Experimental Go extension docs (quarantined)".to_owned(),
            source_class: DocsPackSourceClass::ExtensionDocsPack,
            source_channel: DocsPackChannel::Nightly,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedButUnverified,
                signer_class: DocsPackSignerClass::PermittedPublisher,
                signing_authority_ref: "signing-authority:go-extension:unverified"
                    .to_owned(),
                signing_chain_digest: Some(
                    "digest:extension-docs:experimental-go:0.1.0".to_owned(),
                ),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "0.1.0".to_owned(),
                max_inclusive_ref: "0.1.0".to_owned(),
            },
            refresh_state: DocsPackRefreshState::RefreshPending,
            last_refresh_at: None,
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: DocsPackMirrorState::NotApplicable,
                mirror_of_pack_id: None,
                upstream_revision_ref: None,
                predecessor_revision_ref: None,
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: DocsPackPinState::Pinned,
            local_availability: DocsPackLocalAvailability::Quarantined,
            publishable_state: DocsPackPublishableState::Quarantined,
            publishable_blocking_reasons: vec![
                "signature_unverified".to_owned(),
                "pack_quarantined".to_owned(),
            ],
            manifest_schema_version: 1,
            disclosure_note: Some(
                "Pack is quarantined by policy; identity preserved for review but content does not render."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn nearby_version_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:nearby_version:rust-std-vec-push".to_owned(),
            pack_id_ref: "pack:mirrored-official:rust-std".to_owned(),
            pack_revision_ref: "rev:mirrored-official:rust-std@1.78.0".to_owned(),
            finding_class: StaleExampleFindingClass::NearbyVersion,
            render_mode: DocsRenderMode::Rendered,
            validation_result_class: DocsValidationResultClass::ValidatedWithWarnings,
            nearby_version_ref: Some("rev:mirrored-official:rust-std@1.79.0".to_owned()),
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:nearby_version_scan:rust-std-vec-push".to_owned()],
        }
    }

    fn stale_example_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:example:help-snippet-01".to_owned(),
            pack_id_ref: "pack:curated:onboarding-and-runbooks".to_owned(),
            pack_revision_ref: "rev:curated:onboarding-and-runbooks@2026.05.20".to_owned(),
            finding_class: StaleExampleFindingClass::StaleExample,
            render_mode: DocsRenderMode::SyntaxChecked,
            validation_result_class: DocsValidationResultClass::ValidatedFailed,
            nearby_version_ref: None,
            superseding_example_id: Some(
                "example:curated:onboarding-snippet-02".to_owned(),
            ),
            suppression: Some(StaleExampleSuppression {
                suppression_id: "suppression:stale:help-snippet-01".to_owned(),
                actor_ref: "actor:maintainer:docs-owner-01".to_owned(),
                reason:
                    "Stale snippet acknowledged; replacement scheduled for the next curated pack revision."
                        .to_owned(),
                expiry_at: "2026-06-30T00:00:00Z".to_owned(),
                evidence_refs: vec![
                    "evidence:stale_example_scan:help-snippet-01".to_owned(),
                ],
                source_pack_id_ref: "pack:curated:onboarding-and-runbooks".to_owned(),
                source_pack_revision_ref:
                    "rev:curated:onboarding-and-runbooks@2026.05.20".to_owned(),
            }),
            evidence_refs: vec![
                "evidence:stale_example_scan:help-snippet-01".to_owned(),
            ],
        }
    }

    fn quarantined_pack_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:quarantined:experimental-go".to_owned(),
            pack_id_ref: "pack:extension-docs:experimental-go".to_owned(),
            pack_revision_ref: "rev:extension-docs:experimental-go@0.1.0".to_owned(),
            finding_class: StaleExampleFindingClass::QuarantinedPack,
            render_mode: DocsRenderMode::NotRendered,
            validation_result_class: DocsValidationResultClass::ValidationUnsupported,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:quarantine_policy:experimental-go".to_owned()],
        }
    }

    fn broken_link_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:broken_link:project-docs-readme".to_owned(),
            pack_id_ref: "pack:project-docs:aureline-workspace".to_owned(),
            pack_revision_ref: "rev:project-docs:aureline-workspace@2026.05.26".to_owned(),
            finding_class: StaleExampleFindingClass::BrokenLink,
            render_mode: DocsRenderMode::Rendered,
            validation_result_class: DocsValidationResultClass::ValidatedFailed,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:link_check:project-docs-readme".to_owned()],
        }
    }

    fn needs_review_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:needs_review:support-runbook-step-04".to_owned(),
            pack_id_ref: "pack:support-runbook:incident-response".to_owned(),
            pack_revision_ref: "rev:support-runbook:incident-response@2026.05.18".to_owned(),
            finding_class: StaleExampleFindingClass::NeedsReview,
            render_mode: DocsRenderMode::ExecutedLocally,
            validation_result_class: DocsValidationResultClass::NotValidated,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:needs_review:support-runbook-step-04".to_owned()],
        }
    }

    fn missing_evidence_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:missing_evidence:generated-reference-claim".to_owned(),
            pack_id_ref: "pack:generated-reference:aureline-api".to_owned(),
            pack_revision_ref: "rev:generated-reference:aureline-api@2026.05.26".to_owned(),
            finding_class: StaleExampleFindingClass::MissingEvidence,
            render_mode: DocsRenderMode::MirroredOnly,
            validation_result_class: DocsValidationResultClass::ValidatedWithWarnings,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:missing_citation:generated-reference-claim".to_owned()],
        }
    }

    fn executed_remotely_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:executed_remotely:python-stdlib-list-comprehension".to_owned(),
            pack_id_ref: "pack:extension-docs:python-stdlib".to_owned(),
            pack_revision_ref: "rev:extension-docs:python-stdlib@3.12.4".to_owned(),
            finding_class: StaleExampleFindingClass::NeedsReview,
            render_mode: DocsRenderMode::ExecutedRemotely,
            validation_result_class: DocsValidationResultClass::ValidatedClean,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec![
                "evidence:remote_runner:python-stdlib-list-comprehension".to_owned()
            ],
        }
    }

    fn browser_handoff_only_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:browser_handoff_only:rust-std-changelog".to_owned(),
            pack_id_ref: "pack:mirrored-official:rust-std".to_owned(),
            pack_revision_ref: "rev:mirrored-official:rust-std@1.78.0".to_owned(),
            finding_class: StaleExampleFindingClass::NeedsReview,
            render_mode: DocsRenderMode::BrowserHandoffOnly,
            validation_result_class: DocsValidationResultClass::ValidationUnsupported,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:browser_handoff:rust-std-changelog".to_owned()],
        }
    }

    fn not_rendered_finding() -> StaleExampleFinding {
        StaleExampleFinding {
            finding_id: "stale:not_rendered:support-runbook-step-08".to_owned(),
            pack_id_ref: "pack:support-runbook:incident-response".to_owned(),
            pack_revision_ref: "rev:support-runbook:incident-response@2026.05.18".to_owned(),
            finding_class: StaleExampleFindingClass::NeedsReview,
            render_mode: DocsRenderMode::NotRendered,
            validation_result_class: DocsValidationResultClass::NotValidated,
            nearby_version_ref: None,
            superseding_example_id: None,
            suppression: None,
            evidence_refs: vec!["evidence:offline_pin:support-runbook-step-08".to_owned()],
        }
    }

    fn project_docs_citation_set() -> CitationSetExport {
        CitationSetExport {
            citation_set_id: "citation_set:derived:project-docs-overview".to_owned(),
            derived_explanation_ref: "derived_explanation:project-docs-overview".to_owned(),
            source_pack_id_refs: vec!["pack:project-docs:aureline-workspace".to_owned()],
            cited_file_refs: vec![
                "file:docs/contributing.md".to_owned(),
                "file:docs/getting-started.md".to_owned(),
            ],
            cited_symbol_refs: Vec::new(),
            cited_docs_refs: vec!["docs-anchor:project_docs:contributing#summary".to_owned()],
            graph_epoch_ref: "graph:epoch:2026.05.26-01".to_owned(),
            locale: "en-US".to_owned(),
            derivation_tool_ref: "tool:aureline-derived-explainer".to_owned(),
            derivation_tool_version: "0.4.2".to_owned(),
            raw_pack_bodies_excluded: true,
            raw_urls_excluded: true,
        }
    }

    fn mirrored_docs_citation_set() -> CitationSetExport {
        CitationSetExport {
            citation_set_id: "citation_set:derived:vec-push-explainer".to_owned(),
            derived_explanation_ref: "derived_explanation:vec-push-overview".to_owned(),
            source_pack_id_refs: vec!["pack:mirrored-official:rust-std".to_owned()],
            cited_file_refs: Vec::new(),
            cited_symbol_refs: vec!["symbol:rust-std:Vec::push".to_owned()],
            cited_docs_refs: vec!["docs-anchor:rust-std:Vec::push".to_owned()],
            graph_epoch_ref: "graph:epoch:2026.05.26-01".to_owned(),
            locale: "en-US".to_owned(),
            derivation_tool_ref: "tool:aureline-derived-explainer".to_owned(),
            derivation_tool_version: "0.4.2".to_owned(),
            raw_pack_bodies_excluded: true,
            raw_urls_excluded: true,
        }
    }

    fn ai_evidence_citation_set() -> CitationSetExport {
        CitationSetExport {
            citation_set_id: "citation_set:ai_evidence:router-overview".to_owned(),
            derived_explanation_ref: "derived_explanation:router-overview".to_owned(),
            source_pack_id_refs: vec![
                "pack:project-docs:aureline-workspace".to_owned(),
                "pack:generated-reference:aureline-api".to_owned(),
            ],
            cited_file_refs: vec!["file:crates/aureline-router/src/lib.rs".to_owned()],
            cited_symbol_refs: vec!["symbol:aureline-router:Router::dispatch".to_owned()],
            cited_docs_refs: vec!["docs-anchor:generated-reference:aureline-router".to_owned()],
            graph_epoch_ref: "graph:epoch:2026.05.26-01".to_owned(),
            locale: "en-US".to_owned(),
            derivation_tool_ref: "tool:aureline-ai-evidence".to_owned(),
            derivation_tool_version: "0.7.1".to_owned(),
            raw_pack_bodies_excluded: true,
            raw_urls_excluded: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_input(packet_id: &str) -> DocsPackTruthPacketInput {
        let mut input = seed::seeded_stable_input();
        input.packet_id = packet_id.to_owned();
        for projection in input.consumer_projections.iter_mut() {
            projection.packet_id_ref = packet_id.to_owned();
        }
        input
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(DocsPackSourceClass::ProjectDocs.as_str(), "project_docs");
        assert_eq!(
            DocsPackSourceClass::MirroredOfficialDocs.as_str(),
            "mirrored_official_docs"
        );
        assert_eq!(
            DocsPackLocalAvailability::MirrorOfflinePinned.as_str(),
            "mirror_offline_pinned"
        );
        assert_eq!(
            DocsRenderMode::BrowserHandoffOnly.as_str(),
            "browser_handoff_only"
        );
        assert_eq!(
            DocsValidationResultClass::ValidatedWithWarnings.as_str(),
            "validated_with_warnings"
        );
        assert_eq!(
            StaleExampleFindingClass::QuarantinedPack.as_str(),
            "quarantined_pack"
        );
        assert_eq!(
            DocsPackConsumerSurface::MirrorOfflineConsole.as_str(),
            "mirror_offline_console"
        );
        assert_eq!(
            DocsPackFindingKind::PackIdentityLostWhenOffline.as_str(),
            "pack_identity_lost_when_offline"
        );
    }

    #[test]
    fn baseline_packet_certifies_stable() {
        let packet =
            DocsPackTruthPacket::materialize(baseline_input("packet:m4:docs_pack_truth:baseline"));
        assert_eq!(
            packet.promotion_state,
            DocsPackPromotionState::Stable,
            "unexpected findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
    }

    #[test]
    fn missing_required_source_class_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:no_extension_pack");
        input
            .manifests
            .retain(|m| m.source_class != DocsPackSourceClass::ExtensionDocsPack);
        // Drop dependent findings and citation sets to isolate the test.
        input
            .stale_example_findings
            .retain(|finding| !finding.pack_id_ref.starts_with("pack:extension-docs:"));
        input.citation_sets.iter_mut().for_each(|set| {
            set.source_pack_id_refs
                .retain(|r| !r.starts_with("pack:extension-docs:"));
        });
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsPackFindingKind::RequiredSourceClassCoverageMissing
        }));
    }

    #[test]
    fn missing_required_local_availability_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:no_offline_availability");
        for manifest in input.manifests.iter_mut() {
            if manifest.local_availability == DocsPackLocalAvailability::MirrorOfflinePinned {
                manifest.local_availability = DocsPackLocalAvailability::AvailableLocal;
            }
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsPackFindingKind::RequiredLocalAvailabilityCoverageMissing
        }));
    }

    #[test]
    fn offline_pack_dropping_signer_identity_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:offline_signer_lost");
        for manifest in input.manifests.iter_mut() {
            if matches!(
                manifest.local_availability,
                DocsPackLocalAvailability::NotInstalled
                    | DocsPackLocalAvailability::UnavailableDisclosed
            ) {
                manifest.signing.signing_authority_ref = String::new();
            }
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsPackFindingKind::PackManifestIncomplete
                || finding.finding_kind == DocsPackFindingKind::PackIdentityLostWhenOffline
        }));
    }

    #[test]
    fn nearby_version_dropping_ref_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:nearby_version_collapsed");
        for finding in input.stale_example_findings.iter_mut() {
            if finding.finding_class == StaleExampleFindingClass::NearbyVersion {
                finding.nearby_version_ref = None;
            }
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| { finding.finding_kind == DocsPackFindingKind::StaleStateCollapsed }));
    }

    #[test]
    fn citation_set_bundling_raw_pack_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:citation_bundles_raw");
        if let Some(set) = input.citation_sets.first_mut() {
            set.raw_pack_bodies_excluded = false;
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsPackFindingKind::CitationSetBundlesRawPack
        }));
    }

    #[test]
    fn suppression_losing_attribution_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:suppression_attribution_lost");
        for finding in input.stale_example_findings.iter_mut() {
            if let Some(suppression) = finding.suppression.as_mut() {
                suppression.actor_ref = String::new();
            }
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsPackFindingKind::SuppressionAttributionLost
        }));
    }

    #[test]
    fn consumer_projection_dropping_render_mode_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:projection_drops_render_mode");
        if let Some(projection) = input.consumer_projections.first_mut() {
            projection.preserves_render_mode = false;
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsPackFindingKind::RenderModeVocabularyDropped
        }));
    }

    #[test]
    fn quarantined_finding_referencing_publishable_pack_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_pack_truth:quarantined_collapsed");
        for finding in input.stale_example_findings.iter_mut() {
            if finding.finding_class == StaleExampleFindingClass::QuarantinedPack {
                finding.pack_id_ref = "pack:project-docs:aureline-workspace".to_owned();
            }
        }
        let packet = DocsPackTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, DocsPackPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| { finding.finding_kind == DocsPackFindingKind::StaleStateCollapsed }));
    }
}
