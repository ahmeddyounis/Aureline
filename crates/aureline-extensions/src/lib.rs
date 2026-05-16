//! Extension-manifest baseline, effective-permission summary, and alpha
//! review decision validator for the first ecosystem-bearing lane.
//!
//! This crate owns:
//!
//! - one inspectable [`manifest_baseline::ExtensionManifestBaselineRecord`]
//!   that pins publisher identity, lifecycle state, scope, declared
//!   permission classes, and origin / source metadata,
//! - one [`manifest_baseline::EffectivePermissionBaselineRecord`] projection
//!   that records the declared-vs-effective diff and refuses to silently
//!   pass through a permission scope not in the declared manifest set, and
//! - one [`manifest_baseline::ManifestInstallDecisionRecord`] projection
//!   the install / review surface emits with a typed
//!   [`manifest_baseline::InstallDecisionClass`] and a typed
//!   [`manifest_baseline::InstallDecisionReasonClass`], and
//! - one [`review_alpha::ExtensionReviewAlphaPacketRecord`] projection
//!   that combines install, update, disable, revoke, publisher-continuity,
//!   revocation, and policy-pack truth for the first consuming review
//!   surface.
//! - one [`install_review::InstallReviewAlphaPacketRecord`] projection that
//!   consumes review, provider/runtime boundary, compatibility,
//!   activation-budget, and install-topology truth for the first
//!   marketplace/package review lane.
//! - one [`collections::ExtensionInstallCollectionAlphaPacket`] projection
//!   that renders package/inventory rows through the shared dense-collection
//!   filter, counter, selection, and batch-review contract.
//! - one [`runtime::RuntimeV1BetaContractRecord`] admission packet that
//!   binds capability-bounded Wasm extensions and separately supervised
//!   external host processes into the runtime v1 beta lifecycle and
//!   capability-negotiation model, plus a
//!   [`runtime::RuntimeV1BetaSupportExportRecord`] projection for the
//!   first consuming support / partner export surface.
//! - one [`supervision::ExtensionHostSupervisionRecord`] supervision
//!   packet that finalizes extension host isolation, restart budgets,
//!   resource limits, and quarantine behavior on top of the runtime
//!   contract, plus a
//!   [`supervision::ExtensionHostSupervisionSupportExportRecord`]
//!   projection for the first consuming support / partner export
//!   surface.
//! - one [`sdk_v1::SdkV1StarterPackRecord`] starter pack that joins the
//!   published SDK v1 typed API surfaces, the manifest authoring guides,
//!   and the canonical sample-extension pack (wasm and external-host)
//!   into one inspectable row, plus an
//!   [`sdk_v1::SdkV1StarterPackSupportExportRecord`] projection for the
//!   first consuming support / partner export surface.
//! - a headless extension conformance validator that consumes authored
//!   beta manifests before registry ingest and emits the report schema at
//!   [`/schemas/extensions/conformance_kit_report.schema.json`](../../../schemas/extensions/conformance_kit_report.schema.json).
//! - one [`publication::ExtensionPublicationPipelineRecord`] publication
//!   packet that binds artifact digest, signer metadata, provenance,
//!   compatibility, promotion steps, rollback plan, and catalog
//!   transaction guards into a single headless publication lane.
//! - one [`registry::CatalogDescriptorRecord`] descriptor that carries
//!   publisher continuity, lifecycle, moderation, revocation-ready, and
//!   mirror-compatible catalog metadata from the publication lane into
//!   discovery, support export, and mirror import consumers.
//!
//! Surfaces (install / review docs, support exports, runtime truth badges,
//! CI / schema validation) read these records by reference. They never
//! invent a local "Trusted" badge, never hide the declared-vs-effective
//! diff, never admit an extension whose manifest scope is incomplete or
//! whose publisher identity is missing, and never silently downgrade a
//! quarantined publisher into an unverified one.
//!
//! The reviewer-facing landing page is
//! [`/docs/extensions/m1_permission_and_publisher_baseline.md`](../../../docs/extensions/m1_permission_and_publisher_baseline.md);
//! the cross-tool boundary schema is
//! [`/schemas/extensions/m1_extension_manifest.schema.json`](../../../schemas/extensions/m1_extension_manifest.schema.json).

pub mod collections;
pub mod install_review;
pub mod manifest_baseline;
pub mod permission_manifest;
pub mod publication;
pub mod registry;
pub mod review_alpha;
pub mod runtime;
pub mod sdk_v1;
pub mod supervision;

pub use collections::{
    ExtensionInstallCollectionAlphaInput, ExtensionInstallCollectionAlphaPacket,
    EXTENSION_INSTALL_COLLECTION_ALPHA_PACKET_RECORD_KIND,
    EXTENSION_INSTALL_COLLECTION_ALPHA_SCHEMA_VERSION,
};
pub use install_review::{
    evaluate_install_review_alpha, project_install_review_alpha_surface,
    validate_install_review_alpha_packet, ActivationBudget, ActivationBudgetDisclosure,
    BridgeStateClass, CompatibilityClaimClass, CompatibilityLabel, CompatibilityLabelBlock,
    InstallReviewActionClass, InstallReviewActionOfferClass, InstallReviewAlphaEvaluation,
    InstallReviewAlphaFinding, InstallReviewAlphaInput, InstallReviewAlphaPacketRecord,
    InstallReviewAlphaProjectionRecord, InstallReviewBoundaryTruth,
    InstallReviewContentSourceClass, InstallReviewDecisionClass, InstallReviewDecisionReasonClass,
    InstallReviewDisclosureClass, InstallReviewSurfaceClass, NativeReviewAuthorityClass,
    RuntimeCostClass, RuntimeCostEvidenceClass, INSTALL_REVIEW_ALPHA_PACKET_RECORD_KIND,
    INSTALL_REVIEW_ALPHA_PROJECTION_RECORD_KIND, INSTALL_REVIEW_ALPHA_SCHEMA_VERSION,
};
pub use manifest_baseline::{
    compute_effective_permission_baseline, decide_manifest_install,
    validate_manifest_baseline_record, DeclaredVsEffectiveDiffEntry,
    EffectivePermissionBaselineRecord, EffectivePermissionDiffClass, ExtensionLifecycleStateClass,
    ExtensionManifestBaselineRecord, HostContractFamilyClass, InstallDecisionClass,
    InstallDecisionReasonClass, ManifestInstallDecisionRecord, ManifestOriginSourceClass,
    ManifestScopeCompletenessClass, ManifestValidationFinding, PermissionScopeClass,
    PermissionScopeEntry, PolicyPackNarrowing, PublisherLifecycleStateClass,
    PublisherTrustTierClass, RedactionClass, SummaryFreshnessClass,
    EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND, EXTENSION_MANIFEST_BASELINE_RECORD_KIND,
    EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION, MANIFEST_INSTALL_DECISION_RECORD_KIND,
};
pub use permission_manifest::{
    capability_class_for_scope, evaluate_permission_manifest_delta, project_permission_manifest,
    project_permission_manifest_support_export, validate_permission_manifest_delta_record,
    validate_permission_manifest_record, CapabilityClassClass, CapabilityClassDeltaClass,
    CapabilityClassDeltaEntry, CapabilityClassSummaryEntry, CapabilityScopeEntry,
    PermissionDeltaClass, PermissionDeltaEntry, PermissionManifestDeltaInput,
    PermissionManifestDeltaRecord, PermissionManifestFinding, PermissionManifestRecord,
    PermissionManifestSupportExportRecord, ReConsentDecisionClass, ReConsentReasonClass,
    PERMISSION_MANIFEST_DELTA_RECORD_KIND, PERMISSION_MANIFEST_RECORD_KIND,
    PERMISSION_MANIFEST_SCHEMA_VERSION, PERMISSION_MANIFEST_SUPPORT_EXPORT_RECORD_KIND,
};
pub use publication::{
    evaluate_extension_publication_pipeline, project_extension_publication_support_export,
    validate_extension_publication_pipeline_record,
    validate_extension_publication_support_export_record, ExtensionPublicationPipelineInput,
    ExtensionPublicationPipelineRecord, ExtensionPublicationSupportExportRecord,
    PublicationArtifactMetadata, PublicationChannelClass, PublicationCompatibilityMetadata,
    PublicationContentAddress, PublicationDecisionClass, PublicationFailureAtomicityGuard,
    PublicationPipelineFinding, PublicationProvenanceClass, PublicationProvenanceMetadata,
    PublicationReasonClass, PublicationRollbackPlan, PublicationSignatureClass,
    PublicationSignerMetadata, PublicationTransactionWriteClass, PublicationVersionMetadata,
    EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND, EXTENSION_PUBLICATION_SCHEMA_VERSION,
    EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use registry::{
    evaluate_catalog_descriptor, project_catalog_descriptor_support_export,
    validate_catalog_descriptor_record, validate_catalog_descriptor_support_export_record,
    CatalogActionOfferClass, CatalogCompatibilityMetadata, CatalogDescriptorDecisionClass,
    CatalogDescriptorFinding, CatalogDescriptorInput, CatalogDescriptorReasonClass,
    CatalogDescriptorRecord, CatalogDescriptorSupportExportRecord, CatalogDisclosureClass,
    CatalogLifecycleMetadata, CatalogLifecycleStateClass, CatalogMirrorMetadata,
    CatalogMirrorabilityClass, CatalogModerationMetadata, CatalogModerationStateClass,
    CatalogPublisherContinuityMetadata, CatalogRegistrySourceClass, CatalogRevocationMetadata,
    CatalogRevocationSnapshotAgeClass, CatalogSupportExplanationClass,
    CatalogTrustBadgeInheritanceRuleClass, CATALOG_DESCRIPTOR_RECORD_KIND,
    CATALOG_DESCRIPTOR_SCHEMA_VERSION, CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND,
};
pub use review_alpha::{
    evaluate_extension_review_alpha, project_review_alpha_surface,
    validate_extension_capability_lifecycle_claim, validate_extension_review_alpha_packet,
    validate_publisher_continuity_alpha_record, validate_revocation_alpha_record,
    ExtensionReviewAlphaInput, ExtensionReviewAlphaPacketRecord,
    ExtensionReviewAlphaProjectionRecord, PolicyPackAlphaApplication, PolicyPackEffectClass,
    PublisherContinuityAlphaRecord, PublisherContinuityStateClass, ReviewActionClass,
    ReviewActionOfferClass, ReviewAlphaFinding, ReviewDecisionClass, ReviewDecisionReasonClass,
    ReviewDisclosureClass, ReviewMutationClass, ReviewSurfaceClass, RevocationAlphaRecord,
    RevocationSourceClass, RevocationStateClass, RevocationSubjectClass,
    EXTENSION_REVIEW_ALPHA_PACKET_RECORD_KIND, EXTENSION_REVIEW_ALPHA_PROJECTION_RECORD_KIND,
    PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND, REVIEW_ALPHA_SCHEMA_VERSION,
    REVOCATION_ALPHA_RECORD_KIND,
};
pub use runtime::{
    evaluate_runtime_v1_beta_contract, project_runtime_v1_beta_support_export,
    validate_runtime_v1_beta_contract, DegradedStateClass, HostPlacementClass,
    HostSupervisionClass, RestartPostureClass, RuntimeAdmissionDecisionClass,
    RuntimeAdmissionReasonClass, RuntimeLifecycleStateClass, RuntimeV1BetaContractInput,
    RuntimeV1BetaContractRecord, RuntimeV1BetaFinding, RuntimeV1BetaSupportExportRecord,
    SdkAlignmentClass, RUNTIME_V1_BETA_CONTRACT_RECORD_KIND, RUNTIME_V1_BETA_SCHEMA_VERSION,
    RUNTIME_V1_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use sdk_v1::{
    evaluate_sdk_v1_starter_pack, host_contract_family_for_api_surface,
    project_sdk_v1_starter_pack_support_export, validate_sample_pack_extension_record,
    validate_sdk_v1_api_surface_record, validate_sdk_v1_manifest_authoring_guide_record,
    validate_sdk_v1_starter_pack_record, SamplePackEntryClass, SamplePackExtensionRecord,
    SamplePackValidationClass, SdkV1ApiAvailabilityClass, SdkV1ApiSurfaceClass,
    SdkV1ApiSurfaceRecord, SdkV1ManifestAuthoringGuideRecord, SdkV1ManifestGuideClass,
    SdkV1StarterPackDecisionClass, SdkV1StarterPackFinding, SdkV1StarterPackInput,
    SdkV1StarterPackReasonClass, SdkV1StarterPackRecord, SdkV1StarterPackSupportExportRecord,
    SAMPLE_PACK_EXTENSION_RECORD_KIND, SDK_V1_API_SURFACE_RECORD_KIND,
    SDK_V1_MANIFEST_AUTHORING_GUIDE_RECORD_KIND, SDK_V1_STARTER_PACK_RECORD_KIND,
    SDK_V1_STARTER_PACK_SCHEMA_VERSION, SDK_V1_STARTER_PACK_SUPPORT_EXPORT_RECORD_KIND,
};
pub use supervision::{
    evaluate_extension_host_supervision, project_extension_host_supervision_support_export,
    validate_extension_host_supervision, AxisBudgetEntry, BudgetPressureClass,
    DiscoveryRankingPostureClass, ExtensionHostSupervisionFinding, ExtensionHostSupervisionInput,
    ExtensionHostSupervisionRecord, ExtensionHostSupervisionSupportExportRecord,
    MaintainerCoverageClass, RecoveryPreconditionClass, RecoveryVisibleProjectionClass,
    RestartBudgetSnapshot, SupervisionAxisClass, SupervisionDecisionClass, SupervisionReasonClass,
    SupervisionResponseClass, VisibilityPostureClass, EXTENSION_HOST_SUPERVISION_RECORD_KIND,
    EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION,
    EXTENSION_HOST_SUPERVISION_SUPPORT_EXPORT_RECORD_KIND,
};
