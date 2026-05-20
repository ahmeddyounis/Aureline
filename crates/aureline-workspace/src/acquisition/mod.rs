//! Repository-acquisition beta truth.
//!
//! The three boundary records in this module ([`SourceLocatorRecord`],
//! [`CheckoutPlanRecord`], [`BootstrapQueueItemRecord`]) name **what
//! source is being acquired**, **what the checkout plan is allowed to do
//! before trust admission**, and **which typed bootstrap work the plan
//! enqueues**. The [`beta`] submodule binds those records into one
//! cross-surface [`RepositoryAcquisitionBetaProjection`] that Start Center,
//! the command palette, deep-link resolvers, and CLI/headless acquisition
//! paths all read so they agree, before any hydrate/init/fetch path runs,
//! on the acquisition verb, checkout shape, expected cost band, credential
//! posture, remaining manual setup, interrupted-acquisition recovery, and
//! the guardrails that keep acquisition from drifting into hidden setup or
//! hidden trust elevation.
//!
//! Boundary schemas:
//!
//! - `schemas/workspace/source_locator.schema.json`
//! - `schemas/workspace/checkout_plan.schema.json`
//! - `schemas/workspace/bootstrap_queue_item.schema.json`
//!
//! Projection schema:
//!
//! - `schemas/workspace/repository_acquisition.schema.json`
//!
//! The frozen vocabulary lives in
//! `docs/workspace/source_acquisition_and_bootstrap_seed.md`; the beta
//! contract lives in `docs/workspace/m3/repository_acquisition_beta.md`.
//! Worked fixtures live under
//! `fixtures/workspace/m3/repository_acquisition_and_bootstrap/`.

pub mod beta;
pub mod descriptors;
pub mod shared;

pub use shared::{AcquisitionSurface, FixtureMetadata as AcquisitionFixtureMetadata};

pub use descriptors::{
    AbsenceClass, AcquisitionFailureReasonClass, AcquisitionPosture, AcquisitionResumeState,
    ArtifactDescriptor, ArtifactSignatureState, AttributableEvidence, AttributableEvidenceClass,
    AuthModeClass, BlockedExecutionPathClass, BlockerClass, BootstrapExecutionClass,
    BootstrapItemClass, BootstrapItemState, BootstrapQueueItemRecord, BootstrapQueueItemRecordKind,
    BrowseSafeActionClass, CheckoutPlanRecord, CheckoutPlanRecordKind, CheckoutTrustStage,
    CheckoutTrustState, DeclaredFreshnessClass, DeepLinkClass, DeepLinkDescriptor, DiscardPosture,
    HostEndpointDescriptor, LiveSessionClass, LiveSessionDescriptor, LocatorArtifactClass,
    LocatorClass, LocatorEntryVerbHint, LocatorTargetKindHint, MirrorFreshnessClass,
    MirrorFreshnessEvidence, NextStepDecisionHook, PolicyNarrowingRef, PolicySourceClass,
    ReadOnlyPartialRoot, ReadOnlyPartialRootClass, RepairHookClass, ResumableAcquisitionState,
    SetupActionsClass, SideEffectBypassPath, SideEffectCleanupClass, SideEffectConnectivityClass,
    SideEffectEnvelope, SideEffectTimeClass, SignerContinuityClass, SignerContinuityEvidence,
    SkipReasonClass, SourceLocatorRecord, SourceLocatorRecordKind, TopologyMarker,
    TopologyMarkerClass, TransportClass, UpstreamDeltaClass, AttachAuthorityClass,
    BOOTSTRAP_QUEUE_ITEM_RECORD_KIND, BOOTSTRAP_QUEUE_ITEM_SCHEMA_VERSION,
    CHECKOUT_PLAN_RECORD_KIND, CHECKOUT_PLAN_SCHEMA_VERSION, SOURCE_LOCATOR_RECORD_KIND,
    SOURCE_LOCATOR_SCHEMA_VERSION,
};

pub use beta::{
    AcquisitionGuardrails, AcquisitionHonestyLabel, AcquisitionVerb, BootstrapCredentialPosture,
    BootstrapEvidencePacket, CheckoutModeClass, CheckoutShape, CredentialPostureClass,
    ExpectedCostBand, InterruptedRecovery, InterruptedRecoveryBranch, LfsPolicyClass,
    ManualFollowup, RepositoryAcquisitionBetaError, RepositoryAcquisitionBetaInputs,
    RepositoryAcquisitionBetaProjection, RepositoryAcquisitionRecordKind, SubmodulePolicyClass,
    REPOSITORY_ACQUISITION_RECORD_KIND, REPOSITORY_ACQUISITION_SCHEMA_VERSION,
};
