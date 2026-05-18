//! Release-center objects that keep publication, rollback, and revocation history consistent.
//!
//! The model in this module is deliberately metadata-only. It binds release
//! candidates, version proposals, publish targets, artifact bundles,
//! promotion steps, and rollback or revocation records to immutable artifact
//! graph refs and evidence refs without carrying raw artifacts, raw logs, or
//! credential material.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ReleaseCenterObjectModel`] records.
pub const RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND: &str = "release_center_object_model";

/// Schema version for the release-center object model.
pub const RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION: u32 = 1;

/// Release artifact family classes that can move inside an artifact graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyClass {
    /// Desktop application binary or package.
    IdeBinary,
    /// Command-line binary or package.
    CliBinary,
    /// Remote agent runtime artifact.
    RemoteAgent,
    /// Debug symbols, crash symbols, or symbolication sidecar.
    DebugSymbols,
    /// JavaScript or CSS source-map bundle.
    SourceMapBundle,
    /// SDK, ABI, or extension-host library artifact.
    SdkArtifact,
    /// User-facing or embedded documentation pack.
    DocsPack,
    /// Public schema or contract export.
    SchemaExport,
    /// Software bill of materials document.
    SbomDocument,
    /// Signed provenance or build attestation.
    SignedAttestation,
    /// Compatibility report or certified matrix row.
    CompatibilityReport,
    /// Security advisory, known-issue, or public notice artifact.
    Advisory,
    /// Mirror metadata, offline import metadata, or air-gap receipt.
    MirrorMetadata,
    /// Update metadata, channel manifest, or rollback manifest.
    UpdateMetadata,
    /// Release evidence, support runbook, or proof packet.
    ReleaseEvidencePacket,
    /// Policy, entitlement, or revocation bundle.
    PolicyBundle,
    /// Extension package or marketplace metadata artifact.
    ExtensionPackage,
}

/// Publication authority source disclosed before an action runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceClass {
    /// No credentialed authority is used.
    NoneLocal,
    /// Local developer keychain or local signing identity.
    LocalDevKeychain,
    /// CI-issued OIDC release identity.
    CiOidcReleaseIdentity,
    /// Governed release vault token or equivalent release authority.
    ReleaseVaultToken,
    /// Emergency quorum or admitted break-glass authority.
    SecurityEmergencyQuorum,
    /// Customer or mirror-operator publication receipt.
    MirrorOperatorReceipt,
    /// Registry or marketplace publisher identity.
    RegistryPublisherIdentity,
    /// Metadata-only support export reference.
    SupportExportRef,
}

/// Rollout ring associated with a candidate, target, or promotion step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRing {
    /// Local preview or dry-run only.
    LocalPreview,
    /// Internal release-team validation.
    Internal,
    /// Canary cohort.
    Canary,
    /// Managed pilot cohort.
    Pilot,
    /// Beta or design-partner cohort.
    Beta,
    /// Broad stable rollout.
    Stable,
    /// Long-term support rollout.
    Lts,
    /// Mirror-only or offline target.
    MirrorOnly,
    /// Emergency containment target.
    Emergency,
}

/// Evidence freshness class used by promotion and target checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessClass {
    /// Required evidence is current.
    Current,
    /// Required evidence is current with an explicit scoped waiver.
    CurrentWithWaiver,
    /// Evidence is stale and blocks promotion.
    StaleBlocking,
    /// Evidence is stale but does not block the scoped action.
    StaleNonBlocking,
    /// Evidence is absent and blocks promotion.
    MissingBlocking,
    /// Evidence does not apply to this scoped object.
    NotApplicable,
}

impl EvidenceFreshnessClass {
    /// Returns true when this freshness class blocks promotion.
    pub const fn blocks_promotion(self) -> bool {
        matches!(self, Self::StaleBlocking | Self::MissingBlocking)
    }
}

/// Signature, attestation, or verification state of an artifact bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureStateClass {
    /// Signature or attestation verified in the current release lane.
    Verified,
    /// Metadata is present but has not been verified by this lane.
    PresentUnverified,
    /// Release signature has not landed yet.
    PendingReleaseSignature,
    /// Signature or attestation is missing.
    Missing,
    /// Signature or attestation was revoked.
    Revoked,
}

/// Publish target class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishTargetClass {
    /// Local release-center preview.
    LocalPreview,
    /// Internal validation ring.
    InternalRing,
    /// Public preview or beta channel.
    PublicPreview,
    /// Stable public channel.
    Stable,
    /// Long-term support channel.
    Lts,
    /// Customer mirror or offline feed.
    MirrorFeed,
    /// Registry namespace or marketplace listing.
    RegistryMarketplace,
    /// Emergency channel or containment feed.
    EmergencyChannel,
}

/// Target visibility class after publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetVisibilityClass {
    /// Target is local to a developer workspace.
    LocalOnly,
    /// Target is internal to the release organization.
    Internal,
    /// Target is private to selected partners.
    PrivatePartner,
    /// Target is public preview.
    PublicPreview,
    /// Target is public stable.
    PublicStable,
    /// Target is public long-term support.
    PublicLts,
    /// Target is mirror-only.
    MirrorOnly,
    /// Target is public registry metadata.
    RegistryPublic,
    /// Target is private registry metadata.
    RegistryPrivate,
    /// Target is bounded emergency visibility.
    EmergencyLimited,
    /// Target is support-only metadata.
    SupportOnly,
}

/// Target mutability class after publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetMutabilityClass {
    /// Local preview pointer may be regenerated.
    MutableLocalPointer,
    /// Ring pointer may move over immutable artifacts.
    MutableRingPointer,
    /// Artifact version is immutable while the channel pointer can move.
    ImmutableVersionMutablePointer,
    /// Published version and metadata are immutable.
    ImmutableOncePublished,
    /// Mirror snapshot can be superseded without rewriting origin identity.
    MirrorSnapshotSupersedable,
    /// Registry version is immutable while metadata can change.
    RegistryVersionImmutableMetadataMutable,
    /// Emergency pointer is time-boxed and must reconcile.
    EmergencyTimeBoxed,
}

/// Dry-run availability for a publish target or action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DryRunAvailabilityClass {
    /// Dry run is required and current.
    RequiredCurrent,
    /// Dry run is supported and current.
    SupportedCurrent,
    /// Dry run exists but is stale.
    SupportedStale,
    /// Dry run is not applicable to the local scope.
    UnavailableNotApplicable,
    /// Target does not support dry runs.
    UnavailableTargetDoesNotSupport,
    /// Dry run failed.
    Failed,
}

/// Release stage class used by candidates and timeline steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionStage {
    /// Candidate is a draft.
    Draft,
    /// Candidate is a local preview.
    LocalPreview,
    /// Candidate is in an internal ring.
    InternalRing,
    /// Candidate is in public preview.
    PublicPreview,
    /// Candidate is a stable candidate.
    StableCandidate,
    /// Candidate is stable.
    Stable,
    /// Candidate is a long-term-support candidate.
    LtsCandidate,
    /// Candidate is long-term support.
    Lts,
    /// Candidate or target is mirror-staged.
    MirrorStaged,
    /// Candidate or target is mirror-published.
    MirrorPublished,
    /// Registry artifact is published.
    RegistryPublished,
    /// Emergency state is active.
    EmergencyActive,
    /// Publication is paused.
    Paused,
    /// State rolled back.
    RolledBack,
    /// Artifact or target revoked.
    Revoked,
    /// Artifact or target yanked.
    Yanked,
    /// Channel, ring, or metadata pointer repinned.
    Repinned,
    /// Emergency or break-glass action reconciled.
    Reconciled,
}

/// Semantic version-change class for version proposals and timeline rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticChangeClass {
    /// No public version-impacting change.
    NoPublicChange,
    /// Patch-level change.
    Patch,
    /// Minor-level change.
    Minor,
    /// Major-level change.
    Major,
    /// Pre-release change.
    PreRelease,
    /// Hotfix or emergency correction.
    Hotfix,
    /// Security-only change.
    SecurityOnly,
    /// Backport to a supported line.
    Backport,
    /// Schema or state epoch change.
    SchemaEpoch,
    /// Manual review is required before classifying the change.
    ManualReviewRequired,
}

/// Promotion timeline event class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionEventClass {
    /// Local build or candidate recorded.
    LocalBuildRecorded,
    /// Publish target staged.
    TargetStaged,
    /// Candidate promoted between stages.
    CandidatePromoted,
    /// Stable channel published.
    StablePublished,
    /// Long-term support channel published.
    LtsPublished,
    /// Mirror feed published.
    MirrorPublished,
    /// Registry or marketplace item published.
    RegistryPublished,
    /// Rollback applied.
    RollbackApplied,
    /// Artifact yanked.
    ArtifactYanked,
    /// Artifact revoked.
    ArtifactRevoked,
    /// Channel pointer repinned.
    ChannelRepinned,
    /// Deprecation or known-limit announced.
    DeprecationAnnounced,
    /// Emergency or break-glass action reconciled.
    Reconciled,
}

/// Public-surface impact class for compatibility notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityImpactClass {
    /// No public compatibility impact.
    None,
    /// Manifest changed.
    ManifestChange,
    /// Schema changed.
    SchemaChange,
    /// SDK range changed.
    SdkRangeChange,
    /// ABI range changed.
    AbiRangeChange,
    /// Extension compatibility changed.
    ExtensionCompatibility,
    /// Documentation pack changed.
    DocsPackChange,
    /// Mirror or import behavior changed.
    MirrorImportChange,
    /// State migration or downgrade behavior changed.
    StateMigrationChange,
}

/// Continuity class for release notes, support, rollback, and mirror state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityClass {
    /// Normal continuity.
    Normal,
    /// Known issue remains published and linked.
    KnownIssueLinked,
    /// Rollback or repin path is coordinated across the graph.
    RollbackCoordinated,
    /// Mirror or offline continuity is preserved.
    MirrorContinuity,
    /// Support export continuity is preserved.
    SupportContinuity,
    /// Continuity depends on emergency reconciliation.
    EmergencyReconciliation,
}

/// Break-glass state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakGlassStateClass {
    /// Break-glass was not used.
    NotUsed,
    /// Break-glass was eligible but not used.
    EligibleButNotUsed,
    /// Break-glass is active and awaiting reconciliation.
    ActivePendingReconciliation,
    /// Break-glass action has been reconciled.
    Reconciled,
    /// Break-glass action was superseded by a signed action.
    SupersededBySignedAction,
    /// Break-glass window expired without reconciliation.
    ExpiredWithoutReconciliation,
    /// Break-glass is forbidden for this action.
    ForbiddenForAction,
}

/// Rollback or revocation record class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackOrRevocationKind {
    /// Roll back to a last-known-good artifact graph.
    Rollback,
    /// Revoke trust or installability.
    Revoke,
    /// Yank future consumption while preserving history.
    Yank,
    /// Repin a mutable pointer to a known target.
    Repin,
    /// Disable a subject through an emergency action.
    EmergencyDisable,
}

/// Blast-radius class for rollback or revocation records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlastRadiusClass {
    /// Affects one artifact node.
    SingleArtifactNode,
    /// Affects a scoped artifact family.
    ArtifactFamilyScoped,
    /// Affects one channel or ring pointer.
    ChannelPointerScoped,
    /// Affects a registry namespace or marketplace listing.
    RegistryNamespaceScoped,
    /// Affects a mirror or offline feed.
    MirrorScoped,
    /// Affects the full coordinated release graph.
    FullArtifactGraph,
}

/// Artifact-graph consistency after a scoped rollback or revocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactGraphConsistency {
    /// Full graph remains consistent.
    ConsistentFullGraph,
    /// Scoped exception remains consistent with linked notes and advisories.
    ConsistentScopedException,
    /// Graph consistency is pending reconciliation.
    PendingReconciliation,
    /// Graph consistency is broken and blocks publication.
    Broken,
}

/// Promotion readiness projection for a release candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionReadiness {
    /// Candidate can be promoted for its declared scope.
    Ready,
    /// Candidate is blocked by stale or missing evidence.
    BlockedByEvidence,
    /// Candidate is blocked by recorded blockers.
    BlockedByBlockers,
    /// Candidate is blocked by missing artifact, target, or rollback refs.
    BlockedByMissingRefs,
}

/// Immutable digest reference bound to an artifact graph node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImmutableDigest {
    /// Stable digest row id.
    pub digest_id: String,
    /// Artifact graph node or artifact ref whose material is hashed.
    pub artifact_ref: String,
    /// Artifact family class of the hashed material.
    pub family_class: ArtifactFamilyClass,
    /// Digest algorithm such as `sha256`.
    pub algorithm: String,
    /// Stable digest material or value ref.
    pub digest_ref: String,
}

/// Evidence reference attached to a release object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    /// Stable evidence ref.
    pub evidence_ref: String,
    /// Evidence kind, such as compatibility report or clean-room rebuild.
    pub evidence_kind: String,
    /// Freshness class for the evidence.
    pub freshness_class: EvidenceFreshnessClass,
    /// Optional generation timestamp.
    pub generated_at: Option<String>,
    /// True when missing or stale evidence must block promotion.
    pub required_for_promotion: bool,
    /// Redaction-safe summary.
    pub summary: String,
}

/// Compatibility note for public-surface impacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityNote {
    /// Stable compatibility note id.
    pub note_id: String,
    /// Impact class for the note.
    pub impact_class: CompatibilityImpactClass,
    /// Affected public or operator surface.
    pub affected_surface: String,
    /// Whether this note affects a public surface.
    pub public_surface: bool,
    /// Redaction-safe summary.
    pub summary: String,
    /// Evidence, diff, schema, SDK, docs, or mirror refs supporting the note.
    pub source_refs: Vec<String>,
}

/// Continuity note that keeps known issues, support, and rollback linked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityNote {
    /// Stable continuity note id.
    pub note_id: String,
    /// Continuity class.
    pub continuity_class: ContinuityClass,
    /// Redaction-safe continuity summary.
    pub summary: String,
    /// Known issue refs preserved by the note.
    pub known_issue_refs: Vec<String>,
    /// Support or audit refs preserved by the note.
    pub support_refs: Vec<String>,
}

/// Dry-run and scope-preview disclosure for a target or action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunDisclosure {
    /// Dry-run availability.
    pub availability_class: DryRunAvailabilityClass,
    /// Optional dry-run result ref.
    pub dry_run_ref: Option<String>,
    /// Optional scope-preview ref.
    pub scope_preview_ref: Option<String>,
    /// Optional generation timestamp.
    pub generated_at: Option<String>,
    /// Optional expiration timestamp.
    pub expires_at: Option<String>,
    /// Blocking finding refs found during dry run.
    pub blocking_findings: Vec<String>,
}

impl DryRunDisclosure {
    /// Returns true when a current dry run or current scope preview exists.
    pub fn is_current_enough_for_publication(&self) -> bool {
        matches!(
            self.availability_class,
            DryRunAvailabilityClass::RequiredCurrent | DryRunAvailabilityClass::SupportedCurrent
        ) && self.blocking_findings.is_empty()
    }
}

/// Artifact sidecar refs carried through promotion timelines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactPayloadRefs {
    /// Symbol, source-map, or symbolication refs.
    pub symbol_refs: Vec<String>,
    /// Documentation pack refs.
    pub docs_pack_refs: Vec<String>,
    /// Schema export refs.
    pub schema_refs: Vec<String>,
    /// Compatibility-note refs.
    pub compatibility_note_refs: Vec<String>,
    /// Advisory, known-limit, or known-issue refs.
    pub advisory_refs: Vec<String>,
    /// Mirror metadata or offline import refs.
    pub mirror_metadata_refs: Vec<String>,
}

impl ArtifactPayloadRefs {
    /// Returns true when all release-bearing sidecar classes are linked.
    pub fn carries_release_bearing_sidecars(&self) -> bool {
        !self.symbol_refs.is_empty()
            && !self.docs_pack_refs.is_empty()
            && !self.schema_refs.is_empty()
            && !self.compatibility_note_refs.is_empty()
            && !self.advisory_refs.is_empty()
            && !self.mirror_metadata_refs.is_empty()
    }
}

/// Break-glass disclosure and reconciliation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakGlassDisclosure {
    /// Break-glass state.
    pub state_class: BreakGlassStateClass,
    /// Actor class that invoked or may invoke the exception.
    pub actor_class: Option<String>,
    /// Optional break-glass event ref.
    pub break_glass_event_ref: Option<String>,
    /// Reason class for the exception.
    pub reason_class: String,
    /// Optional reconciliation state.
    pub reconciliation_state: Option<String>,
    /// Optional timestamp by which reconciliation must close.
    pub reconcile_by: Option<String>,
    /// Follow-up refs for retrospective review or superseding action.
    pub follow_up_refs: Vec<String>,
}

impl BreakGlassDisclosure {
    /// Returns true when break-glass state requires explicit reconciliation refs.
    pub const fn state_requires_reconciliation(state_class: BreakGlassStateClass) -> bool {
        matches!(
            state_class,
            BreakGlassStateClass::ActivePendingReconciliation
                | BreakGlassStateClass::Reconciled
                | BreakGlassStateClass::SupersededBySignedAction
                | BreakGlassStateClass::ExpiredWithoutReconciliation
        )
    }
}

/// Artifact bundle card shown by release center, support, and audit exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactBundleCard {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Artifact graph ref this bundle belongs to.
    pub artifact_graph_ref: String,
    /// Exact-build identity refs covered by the bundle.
    pub exact_build_identity_refs: Vec<String>,
    /// Artifact refs included in the bundle.
    pub artifact_refs: Vec<String>,
    /// Immutable digest rows for bundle material.
    pub digest_set: Vec<ImmutableDigest>,
    /// Signature state.
    pub signature_state: SignatureStateClass,
    /// Attestation state.
    pub attestation_state: SignatureStateClass,
    /// Evidence refs attached to the bundle.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Sidecar refs that must travel with the bundle.
    pub payload_refs: ArtifactPayloadRefs,
    /// Export actions available without unpacking raw archives.
    pub export_actions: Vec<String>,
    /// Compatibility notes attached to the bundle.
    pub compatibility_notes: Vec<CompatibilityNote>,
    /// Continuity notes attached to the bundle.
    pub continuity_notes: Vec<ContinuityNote>,
}

/// Version-bump proposal linked before publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionBumpProposal {
    /// Stable proposal id.
    pub proposal_id: String,
    /// Previous version.
    pub prior_version: String,
    /// Target version.
    pub target_version: String,
    /// Semantic change class.
    pub semantic_change_class: SemanticChangeClass,
    /// Artifact refs affected by the version bump.
    pub affected_artifact_refs: Vec<String>,
    /// Manifest or schema change refs.
    pub manifest_schema_change_refs: Vec<String>,
    /// SDK or ABI range shift refs.
    pub sdk_abi_range_refs: Vec<String>,
    /// Extension compatibility refs.
    pub extension_compatibility_refs: Vec<String>,
    /// Documentation-pack change refs.
    pub docs_pack_change_refs: Vec<String>,
    /// Mirror or import implication refs.
    pub mirror_import_implication_refs: Vec<String>,
    /// Evidence refs supporting the proposal.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Compatibility notes surfaced by the proposal.
    pub compatibility_notes: Vec<CompatibilityNote>,
    /// Approval refs for the proposed public-surface impact.
    pub approval_refs: Vec<String>,
}

/// Release candidate object shared by release center and headless flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCandidate {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Candidate version label.
    pub version: String,
    /// Channel family.
    pub channel_family: String,
    /// Current promotion stage.
    pub current_stage: PromotionStage,
    /// Artifact graph ref scoped to the candidate.
    pub artifact_graph_ref: String,
    /// Artifact bundle refs in candidate scope.
    pub artifact_bundle_refs: Vec<String>,
    /// Publish target refs in candidate scope.
    pub publish_target_refs: Vec<String>,
    /// Version proposal refs in candidate scope.
    pub version_bump_proposal_refs: Vec<String>,
    /// Exact-build identity refs in candidate scope.
    pub exact_build_identity_refs: Vec<String>,
    /// Evidence refs required or attached to the candidate.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Candidate blocker refs.
    pub blocker_refs: Vec<String>,
    /// Known issue refs published with the candidate.
    pub known_issue_refs: Vec<String>,
    /// Rollback target ref.
    pub rollback_target_ref: String,
    /// Auth-source class disclosed for publication.
    pub auth_source_class: AuthSourceClass,
    /// Rollout ring.
    pub rollout_ring: RolloutRing,
    /// Compatibility notes for public-surface impact.
    pub compatibility_notes: Vec<CompatibilityNote>,
    /// Continuity notes for known issues, rollback, support, and mirrors.
    pub continuity_notes: Vec<ContinuityNote>,
}

impl ReleaseCandidate {
    /// Returns the candidate's promotion readiness from its local fields.
    pub fn promotion_readiness(&self) -> PromotionReadiness {
        if !self.blocker_refs.is_empty() {
            return PromotionReadiness::BlockedByBlockers;
        }
        if self.artifact_bundle_refs.is_empty()
            || self.publish_target_refs.is_empty()
            || self.rollback_target_ref.trim().is_empty()
            || self.exact_build_identity_refs.is_empty()
        {
            return PromotionReadiness::BlockedByMissingRefs;
        }
        if self.evidence_refs.iter().any(|evidence| {
            evidence.required_for_promotion && evidence.freshness_class.blocks_promotion()
        }) {
            return PromotionReadiness::BlockedByEvidence;
        }
        PromotionReadiness::Ready
    }
}

/// Publish target descriptor shared by release-center sheets and automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishTargetDescriptor {
    /// Stable publish target id.
    pub publish_target_id: String,
    /// Target class.
    pub target_class: PublishTargetClass,
    /// Destination class token.
    pub destination_class: String,
    /// Visibility after publication.
    pub visibility_class: TargetVisibilityClass,
    /// Mutability after publication.
    pub mutability_class: TargetMutabilityClass,
    /// Auth-source class disclosed before mutation.
    pub auth_source_class: AuthSourceClass,
    /// Actor class expected to publish.
    pub actor_class: String,
    /// Rollout ring.
    pub rollout_ring: RolloutRing,
    /// Dry-run disclosure.
    pub dry_run: DryRunDisclosure,
    /// Rollback target ref.
    pub rollback_target_ref: String,
    /// Evidence refs attached to the target.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Exact-build identity refs scoped to the target.
    pub exact_build_identity_refs: Vec<String>,
    /// Cross-surface parity refs.
    pub surface_parity_refs: Vec<String>,
    /// Compatibility notes.
    pub compatibility_notes: Vec<CompatibilityNote>,
    /// Continuity notes.
    pub continuity_notes: Vec<ContinuityNote>,
}

/// Promotion timeline step shared by UI, headless, support, and audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionTimelineStep {
    /// Stable timeline step id.
    pub timeline_step_id: String,
    /// Candidate ref moved by the step.
    pub candidate_ref: String,
    /// Artifact graph ref bound to the step.
    pub artifact_graph_ref: String,
    /// Source stage.
    pub source_stage: PromotionStage,
    /// Destination stage.
    pub destination_stage: PromotionStage,
    /// Timeline event class.
    pub event_class: PromotionEventClass,
    /// Semantic change class.
    pub semantic_change_class: SemanticChangeClass,
    /// Publish target refs affected by the step.
    pub publish_target_refs: Vec<String>,
    /// Artifact bundle refs affected by the step.
    pub artifact_bundle_refs: Vec<String>,
    /// Immutable digest refs bound to the step.
    pub digest_refs: Vec<String>,
    /// Evidence refs bound to the step.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Approving actor classes or refs.
    pub approving_actor_refs: Vec<String>,
    /// Auth-source class used by the step.
    pub auth_source_class: AuthSourceClass,
    /// Rollout ring affected by the step.
    pub rollout_ring: RolloutRing,
    /// Optional reversible window text or timestamp.
    pub reversible_window: Option<String>,
    /// Rollback target ref available for the step.
    pub rollback_target_ref: String,
    /// Sidecar refs preserved in the timeline.
    pub payload_refs: ArtifactPayloadRefs,
    /// Break-glass disclosure.
    pub break_glass: BreakGlassDisclosure,
    /// Compatibility notes.
    pub compatibility_notes: Vec<CompatibilityNote>,
    /// Continuity notes.
    pub continuity_notes: Vec<ContinuityNote>,
}

/// Scoped rollback, revocation, yank, repin, or emergency-disable record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackOrRevocationRecord {
    /// Stable rollback or revocation id.
    pub record_id: String,
    /// Record kind.
    pub kind: RollbackOrRevocationKind,
    /// Artifact graph ref affected by the record.
    pub artifact_graph_ref: String,
    /// Affected artifact refs.
    pub affected_artifact_refs: Vec<String>,
    /// Unaffected artifact refs explicitly preserved.
    pub unaffected_artifact_refs: Vec<String>,
    /// Blast-radius class.
    pub blast_radius_class: BlastRadiusClass,
    /// Last-known-good target ref.
    pub last_known_good_ref: String,
    /// Optional rollback manifest ref.
    pub rollback_manifest_ref: Option<String>,
    /// Revocation record refs.
    pub revocation_record_refs: Vec<String>,
    /// Advisory refs linked to the action.
    pub advisory_refs: Vec<String>,
    /// Known issue refs preserved after the action.
    pub known_issue_refs: Vec<String>,
    /// Support export refs.
    pub support_export_refs: Vec<String>,
    /// Auth-source class used by the action.
    pub auth_source_class: AuthSourceClass,
    /// Rollout ring or target scope.
    pub rollout_ring: RolloutRing,
    /// Artifact-graph consistency after the action.
    pub artifact_graph_consistency: ArtifactGraphConsistency,
    /// Evidence refs attached to the action.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Break-glass disclosure.
    pub break_glass: BreakGlassDisclosure,
    /// Compatibility notes.
    pub compatibility_notes: Vec<CompatibilityNote>,
    /// Continuity notes.
    pub continuity_notes: Vec<ContinuityNote>,
}

/// Identity index shared by UI, headless, support, and audit projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterObjectIdentityIndex {
    /// Release candidate ids.
    pub release_candidate_ids: Vec<String>,
    /// Version-bump proposal ids.
    pub version_bump_proposal_ids: Vec<String>,
    /// Publish target ids.
    pub publish_target_ids: Vec<String>,
    /// Promotion timeline step ids.
    pub promotion_timeline_step_ids: Vec<String>,
    /// Rollback or revocation record ids.
    pub rollback_or_revocation_record_ids: Vec<String>,
    /// Artifact bundle ids.
    pub artifact_bundle_ids: Vec<String>,
    /// Exact-build identity refs.
    pub exact_build_identity_refs: Vec<String>,
    /// Artifact graph refs.
    pub artifact_graph_refs: Vec<String>,
}

/// Release-center UI projection over the shared object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterUiState {
    /// Stable model id.
    pub model_id: String,
    /// Shared identity index.
    pub identity_index: ReleaseCenterObjectIdentityIndex,
    /// Candidate readiness projections.
    pub candidate_readiness: Vec<(String, PromotionReadiness)>,
}

/// Headless publication-plan projection over the shared object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterHeadlessPlan {
    /// Stable model id.
    pub model_id: String,
    /// Shared identity index.
    pub identity_index: ReleaseCenterObjectIdentityIndex,
    /// Target ids that have current-enough dry-run state.
    pub current_dry_run_target_ids: Vec<String>,
}

/// Support and audit export projection over the shared object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterSupportAuditExport {
    /// Stable model id.
    pub model_id: String,
    /// Shared identity index.
    pub identity_index: ReleaseCenterObjectIdentityIndex,
    /// Rollback, revocation, yank, repin, or emergency-disable ids.
    pub scoped_recovery_record_ids: Vec<String>,
    /// Break-glass timeline step ids.
    pub break_glass_step_ids: Vec<String>,
}

/// Complete release-center object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterObjectModel {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable model id.
    pub model_id: String,
    /// UTC timestamp or date when the model was generated.
    pub generated_at: String,
    /// Release candidate objects.
    pub release_candidates: Vec<ReleaseCandidate>,
    /// Version-bump proposals.
    pub version_bump_proposals: Vec<VersionBumpProposal>,
    /// Publish-target descriptors.
    pub publish_targets: Vec<PublishTargetDescriptor>,
    /// Promotion timeline steps.
    pub promotion_timeline_steps: Vec<PromotionTimelineStep>,
    /// Scoped rollback, revocation, yank, repin, and emergency records.
    pub rollback_or_revocation_records: Vec<RollbackOrRevocationRecord>,
    /// Artifact bundle cards.
    pub artifact_bundle_cards: Vec<ArtifactBundleCard>,
}

impl ReleaseCenterObjectModel {
    /// Validates release-center object identity, refs, and publication invariants.
    pub fn validate(&self) -> ReleaseCenterModelValidationReport {
        let mut violations = Vec::new();

        if self.record_kind != RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND {
            push_violation(
                &mut violations,
                "model.record_kind",
                &self.record_kind,
                "record_kind must be release_center_object_model",
            );
        }
        if self.schema_version != RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "model.schema_version",
                &self.model_id,
                "schema_version must be 1",
            );
        }
        if self.model_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            push_violation(
                &mut violations,
                "model.identity",
                &self.model_id,
                "model_id and generated_at must be non-empty",
            );
        }

        require_non_empty(
            &mut violations,
            "release_candidates.empty",
            "release_candidates",
            self.release_candidates.is_empty(),
            "at least one release candidate is required",
        );
        require_non_empty(
            &mut violations,
            "publish_targets.empty",
            "publish_targets",
            self.publish_targets.is_empty(),
            "at least one publish target is required",
        );
        require_non_empty(
            &mut violations,
            "promotion_timeline_steps.empty",
            "promotion_timeline_steps",
            self.promotion_timeline_steps.is_empty(),
            "at least one promotion timeline step is required",
        );
        require_non_empty(
            &mut violations,
            "artifact_bundle_cards.empty",
            "artifact_bundle_cards",
            self.artifact_bundle_cards.is_empty(),
            "at least one artifact bundle card is required",
        );

        let candidate_ids = collect_unique_ids(
            &mut violations,
            "candidate",
            self.release_candidates
                .iter()
                .map(|candidate| candidate.candidate_id.as_str()),
        );
        let proposal_ids = collect_unique_ids(
            &mut violations,
            "version_bump_proposal",
            self.version_bump_proposals
                .iter()
                .map(|proposal| proposal.proposal_id.as_str()),
        );
        let target_ids = collect_unique_ids(
            &mut violations,
            "publish_target",
            self.publish_targets
                .iter()
                .map(|target| target.publish_target_id.as_str()),
        );
        let _timeline_ids = collect_unique_ids(
            &mut violations,
            "promotion_timeline_step",
            self.promotion_timeline_steps
                .iter()
                .map(|step| step.timeline_step_id.as_str()),
        );
        let _recovery_ids = collect_unique_ids(
            &mut violations,
            "rollback_or_revocation",
            self.rollback_or_revocation_records
                .iter()
                .map(|record| record.record_id.as_str()),
        );
        let bundle_ids = collect_unique_ids(
            &mut violations,
            "artifact_bundle",
            self.artifact_bundle_cards
                .iter()
                .map(|bundle| bundle.bundle_id.as_str()),
        );

        for bundle in &self.artifact_bundle_cards {
            self.validate_bundle(bundle, &mut violations);
        }
        for proposal in &self.version_bump_proposals {
            self.validate_version_proposal(proposal, &mut violations);
        }
        for target in &self.publish_targets {
            self.validate_target(target, &mut violations);
        }
        for candidate in &self.release_candidates {
            self.validate_candidate(
                candidate,
                &proposal_ids,
                &target_ids,
                &bundle_ids,
                &mut violations,
            );
        }
        for step in &self.promotion_timeline_steps {
            self.validate_timeline_step(
                step,
                &candidate_ids,
                &target_ids,
                &bundle_ids,
                &mut violations,
            );
        }
        for record in &self.rollback_or_revocation_records {
            self.validate_recovery_record(record, &mut violations);
        }

        let ui = self.ui_state();
        let headless = self.headless_plan();
        let support = self.support_audit_export();
        if ui.identity_index != headless.identity_index
            || ui.identity_index != support.identity_index
        {
            push_violation(
                &mut violations,
                "projection.identity_mismatch",
                &self.model_id,
                "UI, headless, and support projections must expose the same object identity index",
            );
        }

        ReleaseCenterModelValidationReport { violations }
    }

    /// Projects release-center UI state from the shared model.
    pub fn ui_state(&self) -> ReleaseCenterUiState {
        ReleaseCenterUiState {
            model_id: self.model_id.clone(),
            identity_index: self.identity_index(),
            candidate_readiness: self
                .release_candidates
                .iter()
                .map(|candidate| {
                    (
                        candidate.candidate_id.clone(),
                        candidate.promotion_readiness(),
                    )
                })
                .collect(),
        }
    }

    /// Projects headless publication-plan state from the shared model.
    pub fn headless_plan(&self) -> ReleaseCenterHeadlessPlan {
        ReleaseCenterHeadlessPlan {
            model_id: self.model_id.clone(),
            identity_index: self.identity_index(),
            current_dry_run_target_ids: self
                .publish_targets
                .iter()
                .filter(|target| target.dry_run.is_current_enough_for_publication())
                .map(|target| target.publish_target_id.clone())
                .collect(),
        }
    }

    /// Projects support and audit export state from the shared model.
    pub fn support_audit_export(&self) -> ReleaseCenterSupportAuditExport {
        ReleaseCenterSupportAuditExport {
            model_id: self.model_id.clone(),
            identity_index: self.identity_index(),
            scoped_recovery_record_ids: self
                .rollback_or_revocation_records
                .iter()
                .map(|record| record.record_id.clone())
                .collect(),
            break_glass_step_ids: self
                .promotion_timeline_steps
                .iter()
                .filter(|step| {
                    !matches!(
                        step.break_glass.state_class,
                        BreakGlassStateClass::NotUsed | BreakGlassStateClass::ForbiddenForAction
                    )
                })
                .map(|step| step.timeline_step_id.clone())
                .collect(),
        }
    }

    fn identity_index(&self) -> ReleaseCenterObjectIdentityIndex {
        let mut exact_build_identity_refs = BTreeSet::new();
        let mut artifact_graph_refs = BTreeSet::new();

        for candidate in &self.release_candidates {
            artifact_graph_refs.insert(candidate.artifact_graph_ref.clone());
            exact_build_identity_refs.extend(candidate.exact_build_identity_refs.clone());
        }
        for target in &self.publish_targets {
            exact_build_identity_refs.extend(target.exact_build_identity_refs.clone());
        }
        for bundle in &self.artifact_bundle_cards {
            artifact_graph_refs.insert(bundle.artifact_graph_ref.clone());
            exact_build_identity_refs.extend(bundle.exact_build_identity_refs.clone());
        }
        for step in &self.promotion_timeline_steps {
            artifact_graph_refs.insert(step.artifact_graph_ref.clone());
        }
        for record in &self.rollback_or_revocation_records {
            artifact_graph_refs.insert(record.artifact_graph_ref.clone());
        }

        ReleaseCenterObjectIdentityIndex {
            release_candidate_ids: sorted_owned(
                self.release_candidates
                    .iter()
                    .map(|candidate| candidate.candidate_id.clone()),
            ),
            version_bump_proposal_ids: sorted_owned(
                self.version_bump_proposals
                    .iter()
                    .map(|proposal| proposal.proposal_id.clone()),
            ),
            publish_target_ids: sorted_owned(
                self.publish_targets
                    .iter()
                    .map(|target| target.publish_target_id.clone()),
            ),
            promotion_timeline_step_ids: sorted_owned(
                self.promotion_timeline_steps
                    .iter()
                    .map(|step| step.timeline_step_id.clone()),
            ),
            rollback_or_revocation_record_ids: sorted_owned(
                self.rollback_or_revocation_records
                    .iter()
                    .map(|record| record.record_id.clone()),
            ),
            artifact_bundle_ids: sorted_owned(
                self.artifact_bundle_cards
                    .iter()
                    .map(|bundle| bundle.bundle_id.clone()),
            ),
            exact_build_identity_refs: exact_build_identity_refs.into_iter().collect(),
            artifact_graph_refs: artifact_graph_refs.into_iter().collect(),
        }
    }

    fn validate_bundle(
        &self,
        bundle: &ArtifactBundleCard,
        violations: &mut Vec<ReleaseCenterModelViolation>,
    ) {
        require_fields(
            violations,
            "artifact_bundle",
            &bundle.bundle_id,
            [
                ("bundle_id", bundle.bundle_id.as_str()),
                ("artifact_graph_ref", bundle.artifact_graph_ref.as_str()),
            ],
        );
        if bundle.exact_build_identity_refs.is_empty() {
            push_violation(
                violations,
                "artifact_bundle.exact_build_identity_refs",
                &bundle.bundle_id,
                "artifact bundle must carry at least one exact-build identity ref",
            );
        }
        if bundle.digest_set.is_empty() {
            push_violation(
                violations,
                "artifact_bundle.digest_set",
                &bundle.bundle_id,
                "artifact bundle must carry immutable digest refs",
            );
        }
        for digest in &bundle.digest_set {
            require_fields(
                violations,
                "artifact_bundle.digest",
                &digest.digest_id,
                [
                    ("digest_id", digest.digest_id.as_str()),
                    ("artifact_ref", digest.artifact_ref.as_str()),
                    ("algorithm", digest.algorithm.as_str()),
                    ("digest_ref", digest.digest_ref.as_str()),
                ],
            );
        }
        if !bundle.payload_refs.carries_release_bearing_sidecars() {
            push_violation(
                violations,
                "artifact_bundle.payload_refs",
                &bundle.bundle_id,
                "artifact bundle must link symbols, docs, schemas, compatibility notes, advisories, and mirror metadata",
            );
        }
    }

    fn validate_version_proposal(
        &self,
        proposal: &VersionBumpProposal,
        violations: &mut Vec<ReleaseCenterModelViolation>,
    ) {
        require_fields(
            violations,
            "version_bump_proposal",
            &proposal.proposal_id,
            [
                ("proposal_id", proposal.proposal_id.as_str()),
                ("prior_version", proposal.prior_version.as_str()),
                ("target_version", proposal.target_version.as_str()),
            ],
        );
        if proposal.compatibility_notes.is_empty()
            && proposal.manifest_schema_change_refs.is_empty()
            && proposal.sdk_abi_range_refs.is_empty()
            && proposal.extension_compatibility_refs.is_empty()
            && proposal.docs_pack_change_refs.is_empty()
            && proposal.mirror_import_implication_refs.is_empty()
        {
            push_violation(
                violations,
                "version_bump_proposal.public_surface_impacts",
                &proposal.proposal_id,
                "version bump proposal must surface public-surface impacts or an explicit compatibility note",
            );
        }
    }

    fn validate_target(
        &self,
        target: &PublishTargetDescriptor,
        violations: &mut Vec<ReleaseCenterModelViolation>,
    ) {
        require_fields(
            violations,
            "publish_target",
            &target.publish_target_id,
            [
                ("publish_target_id", target.publish_target_id.as_str()),
                ("destination_class", target.destination_class.as_str()),
                ("actor_class", target.actor_class.as_str()),
                ("rollback_target_ref", target.rollback_target_ref.as_str()),
            ],
        );
        if target.exact_build_identity_refs.is_empty() {
            push_violation(
                violations,
                "publish_target.exact_build_identity_refs",
                &target.publish_target_id,
                "publish target must expose exact-build identity refs before mutation",
            );
        }
        if !matches!(
            target.target_class,
            PublishTargetClass::LocalPreview | PublishTargetClass::EmergencyChannel
        ) && !target.dry_run.is_current_enough_for_publication()
        {
            push_violation(
                violations,
                "publish_target.dry_run",
                &target.publish_target_id,
                "non-local publish target must have a current dry run or scope preview without blocking findings",
            );
        }
        if target.compatibility_notes.is_empty() {
            push_violation(
                violations,
                "publish_target.compatibility_notes",
                &target.publish_target_id,
                "publish target must carry compatibility notes for pre-action review",
            );
        }
        if target.continuity_notes.is_empty() {
            push_violation(
                violations,
                "publish_target.continuity_notes",
                &target.publish_target_id,
                "publish target must carry continuity notes for rollback, support, or mirror review",
            );
        }
    }

    fn validate_candidate(
        &self,
        candidate: &ReleaseCandidate,
        proposal_ids: &BTreeSet<&str>,
        target_ids: &BTreeSet<&str>,
        bundle_ids: &BTreeSet<&str>,
        violations: &mut Vec<ReleaseCenterModelViolation>,
    ) {
        require_fields(
            violations,
            "release_candidate",
            &candidate.candidate_id,
            [
                ("candidate_id", candidate.candidate_id.as_str()),
                ("version", candidate.version.as_str()),
                ("channel_family", candidate.channel_family.as_str()),
                ("artifact_graph_ref", candidate.artifact_graph_ref.as_str()),
                (
                    "rollback_target_ref",
                    candidate.rollback_target_ref.as_str(),
                ),
            ],
        );
        if !matches!(candidate.promotion_readiness(), PromotionReadiness::Ready) {
            push_violation(
                violations,
                "release_candidate.promotion_readiness",
                &candidate.candidate_id,
                "candidate is not eligible for promotion in its declared scope",
            );
        }
        validate_refs_exist(
            violations,
            "release_candidate.artifact_bundle_refs",
            &candidate.candidate_id,
            &candidate.artifact_bundle_refs,
            bundle_ids,
            "release candidate references an unknown artifact bundle",
        );
        validate_refs_exist(
            violations,
            "release_candidate.publish_target_refs",
            &candidate.candidate_id,
            &candidate.publish_target_refs,
            target_ids,
            "release candidate references an unknown publish target",
        );
        validate_refs_exist(
            violations,
            "release_candidate.version_bump_proposal_refs",
            &candidate.candidate_id,
            &candidate.version_bump_proposal_refs,
            proposal_ids,
            "release candidate references an unknown version-bump proposal",
        );
        if candidate.known_issue_refs.is_empty() {
            push_violation(
                violations,
                "release_candidate.known_issue_refs",
                &candidate.candidate_id,
                "candidate must preserve known-issue publication refs",
            );
        }
    }

    fn validate_timeline_step(
        &self,
        step: &PromotionTimelineStep,
        candidate_ids: &BTreeSet<&str>,
        target_ids: &BTreeSet<&str>,
        bundle_ids: &BTreeSet<&str>,
        violations: &mut Vec<ReleaseCenterModelViolation>,
    ) {
        require_fields(
            violations,
            "promotion_timeline_step",
            &step.timeline_step_id,
            [
                ("timeline_step_id", step.timeline_step_id.as_str()),
                ("candidate_ref", step.candidate_ref.as_str()),
                ("artifact_graph_ref", step.artifact_graph_ref.as_str()),
                ("rollback_target_ref", step.rollback_target_ref.as_str()),
            ],
        );
        validate_refs_exist(
            violations,
            "promotion_timeline_step.candidate_ref",
            &step.timeline_step_id,
            std::slice::from_ref(&step.candidate_ref),
            candidate_ids,
            "promotion timeline references an unknown release candidate",
        );
        validate_refs_exist(
            violations,
            "promotion_timeline_step.publish_target_refs",
            &step.timeline_step_id,
            &step.publish_target_refs,
            target_ids,
            "promotion timeline references an unknown publish target",
        );
        validate_refs_exist(
            violations,
            "promotion_timeline_step.artifact_bundle_refs",
            &step.timeline_step_id,
            &step.artifact_bundle_refs,
            bundle_ids,
            "promotion timeline references an unknown artifact bundle",
        );
        if step.digest_refs.is_empty() {
            push_violation(
                violations,
                "promotion_timeline_step.digest_refs",
                &step.timeline_step_id,
                "promotion timeline step must bind immutable digest refs",
            );
        }
        if step.evidence_refs.is_empty() {
            push_violation(
                violations,
                "promotion_timeline_step.evidence_refs",
                &step.timeline_step_id,
                "promotion timeline step must bind named evidence refs",
            );
        }
        if !step.payload_refs.carries_release_bearing_sidecars() {
            push_violation(
                violations,
                "promotion_timeline_step.payload_refs",
                &step.timeline_step_id,
                "promotion timeline must preserve symbols, docs, schemas, compatibility notes, advisories, and mirror metadata",
            );
        }
        if BreakGlassDisclosure::state_requires_reconciliation(step.break_glass.state_class)
            && (step.break_glass.break_glass_event_ref.is_none()
                || step.break_glass.follow_up_refs.is_empty())
        {
            push_violation(
                violations,
                "promotion_timeline_step.break_glass",
                &step.timeline_step_id,
                "break-glass timeline step must carry event and reconciliation refs",
            );
        }
    }

    fn validate_recovery_record(
        &self,
        record: &RollbackOrRevocationRecord,
        violations: &mut Vec<ReleaseCenterModelViolation>,
    ) {
        require_fields(
            violations,
            "rollback_or_revocation_record",
            &record.record_id,
            [
                ("record_id", record.record_id.as_str()),
                ("artifact_graph_ref", record.artifact_graph_ref.as_str()),
                ("last_known_good_ref", record.last_known_good_ref.as_str()),
            ],
        );
        if record.affected_artifact_refs.is_empty() {
            push_violation(
                violations,
                "rollback_or_revocation_record.affected_artifact_refs",
                &record.record_id,
                "rollback or revocation must name the affected artifact set",
            );
        }
        if matches!(
            record.kind,
            RollbackOrRevocationKind::Revoke
                | RollbackOrRevocationKind::Yank
                | RollbackOrRevocationKind::EmergencyDisable
        ) && record.revocation_record_refs.is_empty()
        {
            push_violation(
                violations,
                "rollback_or_revocation_record.revocation_record_refs",
                &record.record_id,
                "revocation, yank, and emergency-disable records must link revocation metadata",
            );
        }
        if matches!(record.kind, RollbackOrRevocationKind::Rollback)
            && record.rollback_manifest_ref.is_none()
        {
            push_violation(
                violations,
                "rollback_or_revocation_record.rollback_manifest_ref",
                &record.record_id,
                "rollback records must link a rollback manifest",
            );
        }
        if matches!(
            record.artifact_graph_consistency,
            ArtifactGraphConsistency::Broken
        ) {
            push_violation(
                violations,
                "rollback_or_revocation_record.artifact_graph_consistency",
                &record.record_id,
                "rollback or revocation must not leave the artifact graph broken",
            );
        }
        if record.known_issue_refs.is_empty() {
            push_violation(
                violations,
                "rollback_or_revocation_record.known_issue_refs",
                &record.record_id,
                "rollback or revocation must preserve known-issue publication refs",
            );
        }
        if record.support_export_refs.is_empty() {
            push_violation(
                violations,
                "rollback_or_revocation_record.support_export_refs",
                &record.record_id,
                "rollback or revocation must preserve support export refs",
            );
        }
    }
}

/// Validation report for a release-center object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterModelValidationReport {
    /// Validation violations found in the model.
    pub violations: Vec<ReleaseCenterModelViolation>,
}

impl ReleaseCenterModelValidationReport {
    /// Returns true when no validation violations were found.
    pub fn is_pass(&self) -> bool {
        self.violations.is_empty()
    }

    /// Returns true when a validation check id is present.
    pub fn has_violation(&self, check_id: &str) -> bool {
        self.violations
            .iter()
            .any(|violation| violation.check_id == check_id)
    }
}

/// One validation violation emitted by [`ReleaseCenterObjectModel::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterModelViolation {
    /// Stable validation check id.
    pub check_id: String,
    /// Object or ref associated with the violation.
    pub reference: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn collect_unique_ids<'a>(
    violations: &mut Vec<ReleaseCenterModelViolation>,
    label: &str,
    ids: impl Iterator<Item = &'a str>,
) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if id.trim().is_empty() {
            push_violation(
                violations,
                &format!("{label}.id_empty"),
                label,
                "object id must be non-empty",
            );
        } else if !seen.insert(id) {
            push_violation(
                violations,
                &format!("{label}.id_duplicate"),
                id,
                "object id must be unique",
            );
        }
    }
    seen
}

fn require_non_empty(
    violations: &mut Vec<ReleaseCenterModelViolation>,
    check_id: &str,
    reference: &str,
    failed: bool,
    message: &str,
) {
    if failed {
        push_violation(violations, check_id, reference, message);
    }
}

fn require_fields<'a>(
    violations: &mut Vec<ReleaseCenterModelViolation>,
    object_class: &str,
    reference: &str,
    fields: impl IntoIterator<Item = (&'a str, &'a str)>,
) {
    for (field, value) in fields {
        if value.trim().is_empty() {
            push_violation(
                violations,
                &format!("{object_class}.{field}"),
                reference,
                "required field must be non-empty",
            );
        }
    }
}

fn validate_refs_exist(
    violations: &mut Vec<ReleaseCenterModelViolation>,
    check_id: &str,
    owner_ref: &str,
    refs: &[String],
    known_refs: &BTreeSet<&str>,
    message: &str,
) {
    for reference in refs {
        if !known_refs.contains(reference.as_str()) {
            push_violation(
                violations,
                check_id,
                &format!("{owner_ref} -> {reference}"),
                message,
            );
        }
    }
}

fn sorted_owned(values: impl Iterator<Item = String>) -> Vec<String> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}

fn push_violation(
    violations: &mut Vec<ReleaseCenterModelViolation>,
    check_id: &str,
    reference: &str,
    message: &str,
) {
    violations.push(ReleaseCenterModelViolation {
        check_id: check_id.to_owned(),
        reference: reference.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(evidence_ref: &str, freshness_class: EvidenceFreshnessClass) -> EvidenceRef {
        EvidenceRef {
            evidence_ref: evidence_ref.to_owned(),
            evidence_kind: "release_evidence".to_owned(),
            freshness_class,
            generated_at: Some("2026-05-17T12:00:00Z".to_owned()),
            required_for_promotion: true,
            summary: "current release evidence ref".to_owned(),
        }
    }

    fn compatibility_note(
        note_id: &str,
        impact_class: CompatibilityImpactClass,
    ) -> CompatibilityNote {
        CompatibilityNote {
            note_id: note_id.to_owned(),
            impact_class,
            affected_surface: "public_release_surface".to_owned(),
            public_surface: true,
            summary: "public-surface impact is linked before promotion".to_owned(),
            source_refs: vec!["compatibility_report:beta.2_1_0".to_owned()],
        }
    }

    fn continuity_note(note_id: &str, continuity_class: ContinuityClass) -> ContinuityNote {
        ContinuityNote {
            note_id: note_id.to_owned(),
            continuity_class,
            summary: "known issues and rollback continuity remain linked".to_owned(),
            known_issue_refs: vec!["known_issue:beta.docs_pack_delta".to_owned()],
            support_refs: vec!["support_export:release_center.beta".to_owned()],
        }
    }

    fn payload_refs() -> ArtifactPayloadRefs {
        ArtifactPayloadRefs {
            symbol_refs: vec!["symbol_manifest:aureline.beta".to_owned()],
            docs_pack_refs: vec!["docs_pack:aureline.beta".to_owned()],
            schema_refs: vec!["schema_export:release.artifact_graph".to_owned()],
            compatibility_note_refs: vec!["compat_note:beta.public_surface".to_owned()],
            advisory_refs: vec!["known_issue:beta.docs_pack_delta".to_owned()],
            mirror_metadata_refs: vec!["mirror_metadata:offline.bundle.beta".to_owned()],
        }
    }

    fn break_glass_not_used() -> BreakGlassDisclosure {
        BreakGlassDisclosure {
            state_class: BreakGlassStateClass::NotUsed,
            actor_class: None,
            break_glass_event_ref: None,
            reason_class: "not_applicable".to_owned(),
            reconciliation_state: None,
            reconcile_by: None,
            follow_up_refs: Vec::new(),
        }
    }

    fn fixture_model() -> ReleaseCenterObjectModel {
        let exact = "build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb";
        let graph = "artifact_graph:aureline.beta.2_1_0_beta_1";
        let bundle_id = "artifact_bundle:aureline.beta.2_1_0_beta_1.release_family";
        let target_id = "publish_target:beta.design_partner_update_feed";
        let candidate_id = "release_candidate:aureline.2_1_0_beta_1";
        let proposal_id = "version_bump:aureline.2_0_4_to_2_1_0_beta_1";

        ReleaseCenterObjectModel {
            record_kind: RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND.to_owned(),
            schema_version: RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION,
            model_id: "release_center_model:beta.2_1_0_beta_1".to_owned(),
            generated_at: "2026-05-17T12:00:00Z".to_owned(),
            release_candidates: vec![ReleaseCandidate {
                candidate_id: candidate_id.to_owned(),
                version: "2.1.0-beta.1".to_owned(),
                channel_family: "beta".to_owned(),
                current_stage: PromotionStage::PublicPreview,
                artifact_graph_ref: graph.to_owned(),
                artifact_bundle_refs: vec![bundle_id.to_owned()],
                publish_target_refs: vec![target_id.to_owned()],
                version_bump_proposal_refs: vec![proposal_id.to_owned()],
                exact_build_identity_refs: vec![exact.to_owned()],
                evidence_refs: vec![evidence(
                    "release_evidence:beta.2_1_0_beta_1",
                    EvidenceFreshnessClass::Current,
                )],
                blocker_refs: Vec::new(),
                known_issue_refs: vec!["known_issue:beta.docs_pack_delta".to_owned()],
                rollback_target_ref: "release_candidate:aureline.2_0_4_stable".to_owned(),
                auth_source_class: AuthSourceClass::ReleaseVaultToken,
                rollout_ring: RolloutRing::Beta,
                compatibility_notes: vec![compatibility_note(
                    "compat_note:beta.public_surface",
                    CompatibilityImpactClass::SchemaChange,
                )],
                continuity_notes: vec![continuity_note(
                    "continuity:beta.rollback",
                    ContinuityClass::RollbackCoordinated,
                )],
            }],
            version_bump_proposals: vec![VersionBumpProposal {
                proposal_id: proposal_id.to_owned(),
                prior_version: "2.0.4".to_owned(),
                target_version: "2.1.0-beta.1".to_owned(),
                semantic_change_class: SemanticChangeClass::PreRelease,
                affected_artifact_refs: vec![
                    "artifact_node:aureline.beta.schema.artifact_graph".to_owned()
                ],
                manifest_schema_change_refs: vec!["schema:release.artifact_graph.v1".to_owned()],
                sdk_abi_range_refs: vec!["sdk_range:extension.runtime.beta".to_owned()],
                extension_compatibility_refs: vec!["compat_row:extension.sdk.window".to_owned()],
                docs_pack_change_refs: vec!["docs_pack:aureline.beta".to_owned()],
                mirror_import_implication_refs: vec![
                    "mirror_metadata:offline.bundle.beta".to_owned()
                ],
                evidence_refs: vec![evidence(
                    "compatibility_report:beta.2_1_0",
                    EvidenceFreshnessClass::Current,
                )],
                compatibility_notes: vec![compatibility_note(
                    "compat_note:beta.public_surface",
                    CompatibilityImpactClass::SchemaChange,
                )],
                approval_refs: vec!["approval:release.beta.public_surface".to_owned()],
            }],
            publish_targets: vec![PublishTargetDescriptor {
                publish_target_id: target_id.to_owned(),
                target_class: PublishTargetClass::PublicPreview,
                destination_class: "public_update_feed".to_owned(),
                visibility_class: TargetVisibilityClass::PrivatePartner,
                mutability_class: TargetMutabilityClass::ImmutableVersionMutablePointer,
                auth_source_class: AuthSourceClass::ReleaseVaultToken,
                actor_class: "release_operator".to_owned(),
                rollout_ring: RolloutRing::Beta,
                dry_run: DryRunDisclosure {
                    availability_class: DryRunAvailabilityClass::RequiredCurrent,
                    dry_run_ref: Some("dry_run:beta.design_partner.2_1_0_beta_1".to_owned()),
                    scope_preview_ref: Some(
                        "scope_preview:beta.design_partner.2_1_0_beta_1".to_owned(),
                    ),
                    generated_at: Some("2026-05-17T11:45:00Z".to_owned()),
                    expires_at: Some("2026-05-17T15:45:00Z".to_owned()),
                    blocking_findings: Vec::new(),
                },
                rollback_target_ref: "release_candidate:aureline.2_0_4_stable".to_owned(),
                evidence_refs: vec![evidence(
                    "release_evidence:beta.2_1_0_beta_1",
                    EvidenceFreshnessClass::Current,
                )],
                exact_build_identity_refs: vec![exact.to_owned()],
                surface_parity_refs: vec![
                    "release_center:beta.design_partner.publish".to_owned(),
                    "headless_publish:beta.design_partner.2_1_0_beta_1".to_owned(),
                    "support_export:release_center.beta".to_owned(),
                ],
                compatibility_notes: vec![compatibility_note(
                    "compat_note:beta.public_surface",
                    CompatibilityImpactClass::SchemaChange,
                )],
                continuity_notes: vec![continuity_note(
                    "continuity:beta.rollback",
                    ContinuityClass::RollbackCoordinated,
                )],
            }],
            promotion_timeline_steps: vec![PromotionTimelineStep {
                timeline_step_id: "promotion_timeline:beta.local_to_design_partner".to_owned(),
                candidate_ref: candidate_id.to_owned(),
                artifact_graph_ref: graph.to_owned(),
                source_stage: PromotionStage::LocalPreview,
                destination_stage: PromotionStage::PublicPreview,
                event_class: PromotionEventClass::CandidatePromoted,
                semantic_change_class: SemanticChangeClass::PreRelease,
                publish_target_refs: vec![target_id.to_owned()],
                artifact_bundle_refs: vec![bundle_id.to_owned()],
                digest_refs: vec!["digest_set:artifact_bundle.beta.release_family".to_owned()],
                evidence_refs: vec![evidence(
                    "release_evidence:beta.2_1_0_beta_1",
                    EvidenceFreshnessClass::Current,
                )],
                approving_actor_refs: vec!["actor:release_operator".to_owned()],
                auth_source_class: AuthSourceClass::ReleaseVaultToken,
                rollout_ring: RolloutRing::Beta,
                reversible_window: Some(
                    "until next beta candidate supersedes this graph".to_owned(),
                ),
                rollback_target_ref: "release_candidate:aureline.2_0_4_stable".to_owned(),
                payload_refs: payload_refs(),
                break_glass: break_glass_not_used(),
                compatibility_notes: vec![compatibility_note(
                    "compat_note:beta.public_surface",
                    CompatibilityImpactClass::SchemaChange,
                )],
                continuity_notes: vec![continuity_note(
                    "continuity:beta.rollback",
                    ContinuityClass::RollbackCoordinated,
                )],
            }],
            rollback_or_revocation_records: vec![RollbackOrRevocationRecord {
                record_id: "rollback_record:beta.coordinated_graph_to_previous_stable".to_owned(),
                kind: RollbackOrRevocationKind::Rollback,
                artifact_graph_ref: graph.to_owned(),
                affected_artifact_refs: vec![
                    "artifact_node:aureline.beta.desktop.shell".to_owned(),
                    "artifact_node:aureline.beta.cli.binary".to_owned(),
                ],
                unaffected_artifact_refs: vec![
                    "artifact_node:aureline.beta.support.projection".to_owned()
                ],
                blast_radius_class: BlastRadiusClass::FullArtifactGraph,
                last_known_good_ref: "release_candidate:aureline.2_0_4_stable".to_owned(),
                rollback_manifest_ref: Some("update_manifest:stable.rollback.2_0_4".to_owned()),
                revocation_record_refs: Vec::new(),
                advisory_refs: vec!["known_issue:beta.docs_pack_delta".to_owned()],
                known_issue_refs: vec!["known_issue:beta.docs_pack_delta".to_owned()],
                support_export_refs: vec!["support_export:release_center.beta".to_owned()],
                auth_source_class: AuthSourceClass::ReleaseVaultToken,
                rollout_ring: RolloutRing::Beta,
                artifact_graph_consistency: ArtifactGraphConsistency::ConsistentFullGraph,
                evidence_refs: vec![evidence(
                    "rollback_plan:beta.coordinated_graph",
                    EvidenceFreshnessClass::Current,
                )],
                break_glass: break_glass_not_used(),
                compatibility_notes: vec![compatibility_note(
                    "compat_note:beta.public_surface",
                    CompatibilityImpactClass::SchemaChange,
                )],
                continuity_notes: vec![continuity_note(
                    "continuity:beta.rollback",
                    ContinuityClass::RollbackCoordinated,
                )],
            }],
            artifact_bundle_cards: vec![ArtifactBundleCard {
                bundle_id: bundle_id.to_owned(),
                artifact_graph_ref: graph.to_owned(),
                exact_build_identity_refs: vec![exact.to_owned()],
                artifact_refs: vec![
                    "artifact_node:aureline.beta.desktop.shell".to_owned(),
                    "artifact_node:aureline.beta.cli.binary".to_owned(),
                    "artifact_node:aureline.beta.remote.agent".to_owned(),
                    "artifact_node:aureline.beta.docs.packaging".to_owned(),
                    "artifact_node:aureline.beta.schema.artifact_graph".to_owned(),
                ],
                digest_set: vec![ImmutableDigest {
                    digest_id: "digest:desktop.shell.source".to_owned(),
                    artifact_ref: "artifact_node:aureline.beta.desktop.shell".to_owned(),
                    family_class: ArtifactFamilyClass::IdeBinary,
                    algorithm: "sha256".to_owned(),
                    digest_ref: "digest_ref:desktop.shell.source".to_owned(),
                }],
                signature_state: SignatureStateClass::PendingReleaseSignature,
                attestation_state: SignatureStateClass::PresentUnverified,
                evidence_refs: vec![evidence(
                    "release_evidence:beta.2_1_0_beta_1",
                    EvidenceFreshnessClass::Current,
                )],
                payload_refs: payload_refs(),
                export_actions: vec![
                    "export_manifest".to_owned(),
                    "open_attestation".to_owned(),
                    "compare_previous".to_owned(),
                ],
                compatibility_notes: vec![compatibility_note(
                    "compat_note:beta.public_surface",
                    CompatibilityImpactClass::SchemaChange,
                )],
                continuity_notes: vec![continuity_note(
                    "continuity:beta.rollback",
                    ContinuityClass::RollbackCoordinated,
                )],
            }],
        }
    }

    #[test]
    fn valid_model_projects_same_identity_to_ui_headless_and_support() {
        let model = fixture_model();
        let report = model.validate();
        assert_eq!(report.violations, Vec::new());

        let ui = model.ui_state();
        let headless = model.headless_plan();
        let support = model.support_audit_export();

        assert_eq!(ui.identity_index, headless.identity_index);
        assert_eq!(ui.identity_index, support.identity_index);
        assert_eq!(
            ui.candidate_readiness,
            vec![(
                "release_candidate:aureline.2_1_0_beta_1".to_owned(),
                PromotionReadiness::Ready,
            )]
        );
        assert_eq!(
            headless.current_dry_run_target_ids,
            vec!["publish_target:beta.design_partner_update_feed".to_owned()]
        );
        assert_eq!(
            support.scoped_recovery_record_ids,
            vec!["rollback_record:beta.coordinated_graph_to_previous_stable".to_owned()]
        );
    }

    #[test]
    fn stale_candidate_evidence_blocks_promotion() {
        let mut model = fixture_model();
        model.release_candidates[0].evidence_refs[0].freshness_class =
            EvidenceFreshnessClass::StaleBlocking;

        let report = model.validate();

        assert!(report.has_violation("release_candidate.promotion_readiness"));
        assert_eq!(
            model.release_candidates[0].promotion_readiness(),
            PromotionReadiness::BlockedByEvidence,
        );
    }

    #[test]
    fn rollback_records_require_known_issue_and_support_continuity() {
        let mut model = fixture_model();
        model.rollback_or_revocation_records[0]
            .known_issue_refs
            .clear();
        model.rollback_or_revocation_records[0]
            .support_export_refs
            .clear();

        let report = model.validate();

        assert!(report.has_violation("rollback_or_revocation_record.known_issue_refs"));
        assert!(report.has_violation("rollback_or_revocation_record.support_export_refs"));
    }

    #[test]
    fn timeline_requires_release_bearing_sidecars() {
        let mut model = fixture_model();
        model.promotion_timeline_steps[0]
            .payload_refs
            .advisory_refs
            .clear();

        let report = model.validate();

        assert!(report.has_violation("promotion_timeline_step.payload_refs"));
    }

    #[test]
    fn model_round_trips_through_json() {
        let model = fixture_model();
        let json = serde_json::to_string(&model).expect("serialize model");
        let parsed: ReleaseCenterObjectModel =
            serde_json::from_str(&json).expect("deserialize model");

        assert_eq!(parsed, model);
    }
}
