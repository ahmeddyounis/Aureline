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
//! - one [`mirror_import::MirrorImportBaselineRecord`] baseline that keeps
//!   primary catalog, approved mirror, offline bundle, and manual artifact
//!   imports aligned on artifact identity, source visibility, publisher
//!   continuity, permission, compatibility, lifecycle, and per-claim trust
//!   downgrade metadata.
//! - one [`revocation::ExtensionIncidentCommunicationRecord`] incident
//!   packet that binds advisory, emergency disable, quarantine,
//!   revocation, primary-registry, mirror, and recovery guidance into one
//!   support-exportable ecosystem incident lane.
//! - one [`marketplace_truth::MarketplaceTruthRowRecord`] row projection
//!   that binds catalog descriptor truth to the current generated
//!   compatibility report before any marketplace row can open install review.
//! - one [`fact_grid::MarketplaceFactGridRecord`] shared fact-grid projection
//!   that carries client scope, registry source, compatibility, script/native
//!   risk, manifest changes, permission deltas, lockfile churn, revocation,
//!   rollback, and activation-budget truth across marketplace rows, detail
//!   pages, install review, diagnostics, and support exports.
//! - one [`compatibility_matrix::ExtensionBridgeMatrix`] compatibility
//!   matrix that names the runtime, SDK, manifest, and bridge windows for
//!   claimed beta extension lanes so marketplace, SDK docs, publication
//!   packets, and support exports do not invent local bridge claims.
//! - one [`lifecycle_metadata::LifecycleMetadataPacket`] packet that
//!   publishes support windows, versioning rules, deprecation guidance,
//!   replacement paths, and removal targets for declared beta SDK and
//!   public-interface rows, plus a
//!   [`lifecycle_metadata::LifecycleMetadataSupportExportRecord`]
//!   projection for support, publication, and partner review consumers.
//! - one [`webview_boundary::ExtensionWebviewBoundaryAuditPacket`] audit packet
//!   that keeps extension-owned webviews, hosted dashboards, provider-auth
//!   checkpoints, and browser-runtime bridges aligned on owner/origin chrome,
//!   system-browser handoff posture, trust-class parity, and support-export
//!   vocabulary.
//! - one [`manifest_editor::ManifestEditorSession`] authoring record that turns
//!   an in-progress extension manifest into inline validation with field-level
//!   anchors, permission explanation chips, lifecycle-driven migration /
//!   deprecation hints, version-range targeting, and open-schema/open-docs
//!   links. Its blocker findings reuse the same stable check ids and severities
//!   the validator CLI and conformance kit emit, separated from editor-only
//!   UX/performance advisories, and it validates fully offline without
//!   executing extension code or a network round-trip.
//! - the [`conformance_reports`] module that turns extension validation output
//!   into first-class author/reviewer report surfaces: an
//!   [`conformance_reports::ExtensionConformanceReport`] with passed/failed
//!   checks, shared severity, repro/screenshot guidance, required fixes, docs
//!   links, and an inline compatibility section (target version range,
//!   deprecated APIs, required shims, removal horizons, migration impact); a
//!   [`conformance_reports::MirrorBundleReview`] that renders the side-loaded,
//!   mirrored, or offline path as first-class with artifact hashes, signing /
//!   provenance state, source registry / mirror, dependency graph, and
//!   reproducibility notes (and never hides a signing/provenance gap behind a
//!   green compatibility check); and a
//!   [`conformance_reports::ReviewExportBundle`] that emits the same reports as
//!   attachable Markdown and JSON. All three reuse one
//!   [`conformance_reports::ReviewSeverityClass`] and
//!   [`conformance_reports::ReviewLifecycleClass`] across authoring surfaces,
//!   install review, marketplace facts, and support packets.
//! - one [`stabilize_external_host_contracts_for_language_tools_debuggers::StableExternalHostContractPacket`]
//!   that stabilizes the external-host contract for language tools, debuggers,
//!   CLIs, and database / infrastructure adapters: it carries a typed data-plane
//!   contract (connection / target class, auth-source mode, read-only-vs-write
//!   posture, local / tunneled / remote / managed origin, result / export safety,
//!   and control-plane-boundary truth) for database / infra adapters, plus a
//!   reconnect / replay safety record that keeps connection state honest and
//!   refuses to silently re-run a query, apply-capable action, or control-plane
//!   mutation after a host restart. It derives the stability qualification with
//!   automatic narrowing below Stable and projects an
//!   [`stabilize_external_host_contracts_for_language_tools_debuggers::ExternalHostContractSupportExport`].
//! - the [`appearance_conformance`] module that makes extension-UI appearance
//!   inheritance a first-class compatibility dimension: an
//!   [`appearance_conformance::AppearanceConformanceRow`] joins each declared
//!   theme, density, focus-ring, high-contrast, reduced-motion, and host-token
//!   inheritance posture with a host-side conformance probe so a row is only
//!   badged [`appearance_conformance::AppearanceSupportClass::FullInheritance`]
//!   when parity is proven, downgrades unproven claims to reduced support, and
//!   refuses claims a probe contradicts. It carries inheritance-gap caveats for
//!   marketplace rows, detail pages, install and side-load review, mirrored /
//!   offline bundle review, and post-install diagnostics, keeps host-stable
//!   trust / severity / permission / policy labels host-rendered, and projects a
//!   metadata-safe [`appearance_conformance::AppearanceConformanceSupportExport`].
//! - one [`harden_extension_manifest_permission_display_lifecycle_labels_and::StableManifestHardeningPacket`]
//!   that hardens the extension manifest, permission display, lifecycle labels,
//!   and compatibility-range truth for the stable line: it declares hard
//!   dependencies and optional integrations (each with a dependency class,
//!   resolution state, lifecycle/deprecation marker, and the permissions it
//!   contributes), pins the compatible API and runtime ranges so a range conflict
//!   is visible before install / upgrade / mirror promotion, and resolves the
//!   **effective** permission set after dependency resolution so authority is
//!   never widened implicitly. It derives the stability qualification with
//!   automatic narrowing below Stable and projects a
//!   [`harden_extension_manifest_permission_display_lifecycle_labels_and::StableManifestHardeningSupportExport`].
//! - one [`stabilize_sdk_schemas_samples_templates_and_conformance_kits::StableSdkAuthorLanePacket`]
//!   that stabilizes the SDK schemas, sample extensions, project templates, and
//!   conformance kits for the stable extension-author lane: it binds the kit
//!   artifacts (each with a kind, host class, published-version pin, and
//!   conformance state), the aggregate conformance summary, the worst-case
//!   activation-budget instrumentation, and the publisher-continuity binding, then
//!   derives the stability qualification with automatic narrowing below Stable so
//!   ambient template privilege, catalog-only trust, an unbounded activation cost,
//!   or a nonconformant artifact can never ride a stable author-lane claim. It
//!   projects a
//!   [`stabilize_sdk_schemas_samples_templates_and_conformance_kits::StableSdkAuthorLaneSupportExport`].
//! - one [`finalize_marketplace_catalog_truth_provenance_compatibility_labels_activation::StableMarketplaceCatalogTruthPacket`]
//!   that finalizes marketplace catalog truth for the stable line: it binds the row
//!   identity, the mechanically-sourced provenance posture, the surface boundary
//!   (runtime class, host boundary, hosted-surface / browser-handoff implications, and
//!   reduced accessibility / theming parity), the discoverability posture (ranked /
//!   penalized / quarantined) kept separate from provenance and support-class truth,
//!   the machine-readable compatibility scorecards (each with a parity band, freshness
//!   window, evidence source, and downgrade state), the activation budget, the support
//!   class (with profile-limited and verified-runtime-profile truth), the
//!   publisher-continuity binding, and the cross-view alignment, then derives the
//!   stability qualification with automatic narrowing below Stable so catalog-only
//!   trust, an unbounded activation cost, inherited parity, ranking-implied trust, or a
//!   quarantined / under-review row can never ride a stable catalog claim. It preserves
//!   runtime-class truth into a
//!   [`finalize_marketplace_catalog_truth_provenance_compatibility_labels_activation::StableMarketplaceCatalogTruthSupportExport`]
//!   for support, mirror, and partner consumers.
//! - one [`harden_extension_performance_inspection_budget_enforcement_and_user::StablePerformanceBudgetPacket`]
//!   that hardens extension performance inspection, budget enforcement, and the
//!   user-visible cost explanation for the stable line: it binds the inspected
//!   worst-case measurement (budget axis, measured p50 / p95, sample count,
//!   benchmark-lab trace, corpus metadata, freshness, and trace-attested flag), the
//!   budget enforcement against the published p50 / p95 ceilings (status,
//!   enforcement mode, threshold-adjustment posture), the waiver hook that backs any
//!   intentional threshold tightening / narrowing / relaxation, the user-visible
//!   cost explanation (cost class, dominant factor, explained flag), the permission
//!   posture, compatibility, and install posture, then derives the stability
//!   qualification with automatic narrowing below Stable so an unbounded activation
//!   cost, a catalog-only (unenforced) budget, or a permission widened beyond the
//!   declared manifest can never ride a stable performance claim. It cross-checks the
//!   numeric p50 / p95 measurement against the published budget so the inspected cost
//!   and the budget status can never contradict, and projects a
//!   [`harden_extension_performance_inspection_budget_enforcement_and_user::StablePerformanceBudgetSupportExport`].
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

pub mod appearance_conformance;
pub mod collections;
pub mod compatibility_matrix;
pub mod conformance_reports;
pub mod fact_grid;
pub mod finalize_extension_bridge_and_certification_scope_and_downgrade;
pub mod finalize_marketplace_catalog_truth_provenance_compatibility_labels_activation;
pub mod finalize_wasm_host_quotas_crash_loop_quarantine_and;
pub mod harden_extension_manifest_permission_display_lifecycle_labels_and;
pub mod harden_extension_performance_inspection_budget_enforcement_and_user;
pub mod harden_install_review_update_review_disable_rollback_and;
pub mod install_review;
pub mod lifecycle_metadata;
pub mod locale_support;
pub mod manifest_baseline;
pub mod manifest_editor;
pub mod marketplace_truth;
pub mod mirror_import;
pub mod permission_manifest;
pub mod publication;
pub mod registry;
pub mod review_alpha;
pub mod revocation;
pub mod runtime;
pub mod sdk_v1;
pub mod stabilize_extension_runtime_v1_abi_capability_envelopes_and;
pub mod stabilize_external_host_contracts_for_language_tools_debuggers;
pub mod stabilize_mirror_manual_import_offline_catalog_and_publisher;
pub mod stabilize_sdk_schemas_samples_templates_and_conformance_kits;
pub mod supervision;
pub mod webview_boundary;

pub use appearance_conformance::{
    audit_appearance_conformance_rows, evaluate_appearance_conformance_row,
    project_appearance_conformance_support_export, project_appearance_conformance_support_row,
    seeded_appearance_conformance_inputs, seeded_appearance_conformance_packet,
    validate_appearance_conformance_packet, validate_appearance_conformance_row,
    validate_appearance_conformance_support_export, AppearanceAxisClass, AppearanceAxisConformance,
    AppearanceAxisDeclaration, AppearanceAxisProbe, AppearanceConformanceDecisionClass,
    AppearanceConformanceDefect, AppearanceConformanceDefectKind, AppearanceConformanceInput,
    AppearanceConformancePacket, AppearanceConformanceReasonClass, AppearanceConformanceRow,
    AppearanceConformanceSummary, AppearanceConformanceSupportExport,
    AppearanceConformanceSupportRow, AppearanceProofClass, AppearanceSupportClass,
    AppearanceSupportDeclaration, AppearanceSurfaceCaveat, AppearanceSurfaceClass,
    AppearanceUnsupportedState, APPEARANCE_AXES, APPEARANCE_SURFACES,
    EXTENSION_APPEARANCE_CONFORMANCE_DEFECT_RECORD_KIND,
    EXTENSION_APPEARANCE_CONFORMANCE_PACKET_RECORD_KIND,
    EXTENSION_APPEARANCE_CONFORMANCE_ROW_RECORD_KIND,
    EXTENSION_APPEARANCE_CONFORMANCE_SCHEMA_VERSION,
    EXTENSION_APPEARANCE_CONFORMANCE_SHARED_CONTRACT_REF,
    EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND,
    EXTENSION_APPEARANCE_CONFORMANCE_SUPPORT_ROW_RECORD_KIND,
};
pub use collections::{
    ExtensionInstallCollectionAlphaInput, ExtensionInstallCollectionAlphaPacket,
    EXTENSION_INSTALL_COLLECTION_ALPHA_PACKET_RECORD_KIND,
    EXTENSION_INSTALL_COLLECTION_ALPHA_SCHEMA_VERSION,
};
pub use compatibility_matrix::{
    current_extension_bridge_matrix, validate_extension_bridge_matrix, ExtensionBridgeMatrix,
    ExtensionBridgeMatrixFinding, ExtensionBridgeMatrixRow, ExtensionBridgeStateClass,
    ExtensionBridgeWindow, ExtensionCompatibilityLabel, ExtensionCompatibilityWindow,
    ExtensionDowngradeBehavior, ExtensionParityClaimClass, CURRENT_EXTENSION_BRIDGE_MATRIX_PATH,
    EXTENSION_BRIDGE_MATRIX_RECORD_KIND, EXTENSION_BRIDGE_MATRIX_SCHEMA_VERSION,
};
pub use conformance_reports::{
    build_conformance_report, build_mirror_bundle_review, build_review_export_bundle,
    render_conformance_report_markdown, render_mirror_bundle_review_markdown,
    render_review_export_bundle_markdown, validate_conformance_report,
    validate_mirror_bundle_review, validate_review_export_bundle, BundleArtifactIdentity,
    BundleDependencyNode, BundleDependencyResolutionClass, BundleReproducibility,
    BundleReproducibilityClass, BundleReviewDecisionClass, BundleReviewReasonClass,
    BundleReviewSummary, BundleSigningProvenance, BundleSource, CompatibilitySection,
    ConformanceDecisionClass, ConformanceReasonClass, ConformanceReportFinding,
    ConformanceReportInput, ConformanceReportSummary, DeprecatedApi, ExtensionConformanceReport,
    MirrorBundleReview, MirrorBundleReviewInput, RequiredShim, ReviewCheck, ReviewCheckStatusClass,
    ReviewExportBundle, ReviewLifecycleClass, ReviewSeverityClass,
    CONFORMANCE_REPORTS_SCHEMA_VERSION, EXTENSION_CONFORMANCE_REPORT_RECORD_KIND,
    MIRROR_BUNDLE_REVIEW_RECORD_KIND, REVIEW_EXPORT_BUNDLE_RECORD_KIND,
};
pub use fact_grid::{
    project_marketplace_fact_grid, project_marketplace_fact_grid_support_export,
    validate_marketplace_fact_grid, validate_marketplace_fact_grid_support_export,
    ClientScopeClass, LockfileImpact, LockfileImpactClass, ManifestChangeClass, ManifestChangeRow,
    MarketplaceFactGridFinding, MarketplaceFactGridInput, MarketplaceFactGridRecord,
    MarketplaceFactGridSupportExportRecord, MarketplaceFactGridSurfaceClass,
    QuarantineRevocationState, ScriptRiskClass, ScriptRiskDisclosure,
    MARKETPLACE_FACT_GRID_RECORD_KIND, MARKETPLACE_FACT_GRID_SCHEMA_VERSION,
    MARKETPLACE_FACT_GRID_SUPPORT_EXPORT_RECORD_KIND,
};
pub use finalize_extension_bridge_and_certification_scope_and_downgrade::{
    project_stable_bridge_certification_scope,
    project_stable_bridge_certification_scope_support_export, BridgeCertificationActivationBudget,
    BridgeCertificationActivationBudgetInput, BridgeCertificationCompatibility,
    BridgeCertificationCompatibilityInput, BridgeCertificationDowngradedBanner,
    BridgeCertificationIdentity, BridgeCertificationIdentityInput,
    BridgeCertificationInstallPosture, BridgeCertificationInstallPostureInput,
    BridgeCertificationPermissionPosture, BridgeCertificationPermissionPostureInput,
    BridgeCertificationQualificationClaim, BridgeCertificationQualificationClaimInput,
    BridgeCertificationScope, BridgeCertificationScopeInput, BridgeSurfaceBinding,
    BridgeSurfaceBindingInput, StableBridgeCertificationScopeError,
    StableBridgeCertificationScopeInput, StableBridgeCertificationScopeInspection,
    StableBridgeCertificationScopePacket, StableBridgeCertificationScopeProjection,
    StableBridgeCertificationScopeSupportExport, StableBridgeCertificationScopeValidationError,
    BRIDGE_CERTIFICATION_ACTIVATION_BUDGET_RECORD_KIND, BRIDGE_CERTIFICATION_COMPATIBILITY_RECORD_KIND,
    BRIDGE_CERTIFICATION_DOWNGRADED_BANNER_RECORD_KIND, BRIDGE_CERTIFICATION_DOWNGRADE_REASONS,
    BRIDGE_CERTIFICATION_IDENTITY_RECORD_KIND, BRIDGE_CERTIFICATION_INSTALL_POSTURE_RECORD_KIND,
    BRIDGE_CERTIFICATION_PERMISSION_POSTURE_RECORD_KIND,
    BRIDGE_CERTIFICATION_QUALIFICATION_CLAIM_RECORD_KIND, BRIDGE_CERTIFICATION_SCOPE_RECORD_KIND,
    BRIDGE_KIND_CLASSES, BRIDGE_SURFACE_BINDING_RECORD_KIND, CERTIFICATION_CATEGORY_CLASSES,
    CERTIFICATION_EVIDENCE_SOURCE_CLASSES, CERTIFIED_SCOPE_STATUS,
    SCOPE_STATUS_CLASSES, STABLE_BRIDGE_CERTIFICATION_CONSUMER_SURFACES,
    STABLE_BRIDGE_CERTIFICATION_INSPECTION_RECORD_KIND,
    STABLE_BRIDGE_CERTIFICATION_PACKET_RECORD_KIND,
    STABLE_BRIDGE_CERTIFICATION_PUBLISHED_SCOPE_VERSION, STABLE_BRIDGE_CERTIFICATION_SCHEMA_REF,
    STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
    STABLE_BRIDGE_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND, STABLE_BRIDGE_PUBLISHED_ABI_VERSION,
    ACTIVATION_BUDGET_CLASSES as BRIDGE_CERTIFICATION_ACTIVATION_BUDGET_CLASSES,
    CONTROL_PLANE_BOUNDARY_CLASSES as BRIDGE_CERTIFICATION_CONTROL_PLANE_BOUNDARY_CLASSES,
    CLAIM_BASIS_CLASSES as BRIDGE_CERTIFICATION_CLAIM_BASIS_CLASSES,
    COMPATIBILITY_LABEL_CLASSES as BRIDGE_CERTIFICATION_COMPATIBILITY_LABEL_CLASSES,
    INSTALL_SCOPE_CLASSES as BRIDGE_CERTIFICATION_INSTALL_SCOPE_CLASSES,
    LIFECYCLE_STATE_CLASSES as BRIDGE_CERTIFICATION_LIFECYCLE_STATE_CLASSES,
    MIRRORABILITY_CLASSES as BRIDGE_CERTIFICATION_MIRRORABILITY_CLASSES,
    REVOCATION_POSTURE_CLASSES as BRIDGE_CERTIFICATION_REVOCATION_POSTURE_CLASSES,
    RUNNABLE_LIFECYCLE_STATES as BRIDGE_CERTIFICATION_RUNNABLE_LIFECYCLE_STATES,
    STABILITY_TIERS as BRIDGE_CERTIFICATION_STABILITY_TIERS,
    STABLE_TIERS as BRIDGE_CERTIFICATION_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as BRIDGE_CERTIFICATION_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as BRIDGE_CERTIFICATION_TRUST_TIER_CLASSES,
};
pub use finalize_marketplace_catalog_truth_provenance_compatibility_labels_activation::{
    project_stable_marketplace_catalog_truth,
    project_stable_marketplace_catalog_truth_support_export, DowngradedCatalogBanner,
    MarketplaceCatalogActivationBudget, MarketplaceCatalogActivationBudgetInput,
    MarketplaceCatalogDiscoverabilityPosture, MarketplaceCatalogDiscoverabilityPostureInput,
    MarketplaceCatalogProvenance, MarketplaceCatalogProvenanceInput,
    MarketplaceCatalogPublisherContinuity, MarketplaceCatalogPublisherContinuityInput,
    MarketplaceCatalogSupportClass, MarketplaceCatalogSupportClassInput,
    MarketplaceCatalogSurfaceBoundary, MarketplaceCatalogSurfaceBoundaryInput,
    MarketplaceCatalogTruthIdentity, MarketplaceCatalogTruthIdentityInput,
    MarketplaceCatalogTruthQualificationClaim, MarketplaceCatalogTruthQualificationClaimInput,
    MarketplaceCatalogViewAlignment, MarketplaceCatalogViewAlignmentInput,
    MarketplaceCompatibilityScorecard, MarketplaceCompatibilityScorecardInput,
    MarketplaceCompatibilitySummary, StableMarketplaceCatalogTruthError,
    StableMarketplaceCatalogTruthInput, StableMarketplaceCatalogTruthInspection,
    StableMarketplaceCatalogTruthPacket, StableMarketplaceCatalogTruthProjection,
    StableMarketplaceCatalogTruthSupportExport, StableMarketplaceCatalogTruthValidationError,
    DOWNGRADED_CATALOG_BANNER_RECORD_KIND, MARKETPLACE_CATALOG_ACTIVATION_BUDGET_RECORD_KIND,
    MARKETPLACE_CATALOG_DISCOVERABILITY_POSTURE_RECORD_KIND,
    MARKETPLACE_CATALOG_DOWNGRADE_REASONS, MARKETPLACE_CATALOG_PROVENANCE_RECORD_KIND,
    MARKETPLACE_CATALOG_PUBLISHER_CONTINUITY_RECORD_KIND,
    MARKETPLACE_CATALOG_SUPPORT_CLASS_RECORD_KIND,
    MARKETPLACE_CATALOG_SURFACE_BOUNDARY_RECORD_KIND,
    MARKETPLACE_CATALOG_TRUTH_IDENTITY_RECORD_KIND,
    MARKETPLACE_CATALOG_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND,
    MARKETPLACE_CATALOG_VIEW_ALIGNMENT_RECORD_KIND, MARKETPLACE_COMPATIBILITY_SCORECARD_RECORD_KIND,
    MARKETPLACE_COMPATIBILITY_SUMMARY_RECORD_KIND,
    STABLE_MARKETPLACE_CATALOG_CONSUMER_SURFACES, STABLE_MARKETPLACE_CATALOG_PUBLISHED_VERSION,
    STABLE_MARKETPLACE_CATALOG_TRUTH_INSPECTION_RECORD_KIND,
    STABLE_MARKETPLACE_CATALOG_TRUTH_PACKET_RECORD_KIND,
    STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_REF, STABLE_MARKETPLACE_CATALOG_TRUTH_SCHEMA_VERSION,
    STABLE_MARKETPLACE_CATALOG_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
    CATALOG_VIEW_CLASSES as MARKETPLACE_CATALOG_VIEW_CLASSES,
    EVIDENCE_SOURCE_CLASSES as MARKETPLACE_CATALOG_EVIDENCE_SOURCE_CLASSES,
    FRESHNESS_WINDOW_CLASSES as MARKETPLACE_CATALOG_FRESHNESS_WINDOW_CLASSES,
    HOST_BOUNDARY_CLASSES as MARKETPLACE_CATALOG_HOST_BOUNDARY_CLASSES,
    PARITY_BAND_CLASSES as MARKETPLACE_CATALOG_PARITY_BAND_CLASSES,
    PROVENANCE_CLASSES as MARKETPLACE_CATALOG_PROVENANCE_CLASSES,
    RANKING_STATE_CLASSES as MARKETPLACE_CATALOG_RANKING_STATE_CLASSES,
    RUNTIME_CLASSES as MARKETPLACE_CATALOG_RUNTIME_CLASSES,
    SCORECARD_SUBJECT_CLASSES as MARKETPLACE_CATALOG_SCORECARD_SUBJECT_CLASSES,
    STABILITY_TIERS as MARKETPLACE_CATALOG_STABILITY_TIERS,
    STABLE_TIERS as MARKETPLACE_CATALOG_STABLE_TIERS,
    SUPPORT_CLASS_CLASSES as MARKETPLACE_CATALOG_SUPPORT_CLASS_CLASSES,
    TRUST_TIER_CLASSES as MARKETPLACE_CATALOG_TRUST_TIER_CLASSES,
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
pub use lifecycle_metadata::{
    current_extension_lifecycle_metadata_packet, evaluate_lifecycle_metadata_packet,
    project_lifecycle_metadata_support_export, validate_lifecycle_metadata_packet,
    validate_lifecycle_metadata_support_export, LifecycleDeprecationMetadata,
    LifecycleDeprecationPostureClass, LifecycleMetadataDecisionClass, LifecycleMetadataFinding,
    LifecycleMetadataPacket, LifecycleMetadataPacketInput, LifecycleMetadataReasonClass,
    LifecycleMetadataRow, LifecycleMetadataSupportExportRecord, LifecycleStabilityLabel,
    LifecycleSupportWindow, LifecycleSurfaceKind, LifecycleVersioningScheme,
    CURRENT_EXTENSION_LIFECYCLE_METADATA_PACKET_PATH, LIFECYCLE_METADATA_PACKET_RECORD_KIND,
    LIFECYCLE_METADATA_SCHEMA_VERSION, LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use locale_support::seeded_extension_locale_declarations;
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
pub use manifest_editor::{
    evaluate_manifest_editor_session, validate_manifest_editor_session,
    ManifestEditorCompatibilityBadgeClass, ManifestEditorConformanceExport,
    ManifestEditorConnectivityClass, ManifestEditorFinding, ManifestEditorFindingSeverity,
    ManifestEditorFindingStatus, ManifestEditorFindingSuite, ManifestEditorLinks,
    ManifestEditorPublishReadinessClass, ManifestEditorPublishReadinessReasonClass,
    ManifestEditorResultClass, ManifestEditorSession, ManifestEditorSessionFinding,
    ManifestEditorSessionInput, ManifestMigrationHint, PermissionExplanationChip,
    VersionTargetingSummary, MANIFEST_EDITOR_SESSION_ID_PREFIX,
    MANIFEST_EDITOR_SESSION_RECORD_KIND, MANIFEST_EDITOR_SESSION_SCHEMA_VERSION,
    MANIFEST_EDITOR_VALIDATOR_ID, MANIFEST_EDITOR_VALIDATOR_VERSION,
};
pub use marketplace_truth::{
    project_marketplace_truth_row, project_marketplace_truth_support_export,
    validate_marketplace_truth_row, validate_marketplace_truth_support_export,
    CompatibilityReportRow, CompatibilityReportSnapshot, CompatibilityReportSupportClass,
    MarketplaceCompatibilityLabelClass, MarketplaceCompatibilityLabelSourceClass,
    MarketplaceSupportChipClass, MarketplaceTrustChipClass, MarketplaceTruthBadgeClass,
    MarketplaceTruthFinding, MarketplaceTruthRowInput, MarketplaceTruthRowRecord,
    MarketplaceTruthSupportExportRecord, MARKETPLACE_TRUTH_ROW_RECORD_KIND,
    MARKETPLACE_TRUTH_SCHEMA_VERSION, MARKETPLACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use mirror_import::{
    evaluate_mirror_import_baseline, project_mirror_import_support_export,
    validate_mirror_import_baseline_record, validate_mirror_import_support_export_record,
    MirrorImportActionOfferClass, MirrorImportBaselineInput, MirrorImportBaselineRecord,
    MirrorImportDecisionClass, MirrorImportDisclosureClass, MirrorImportDowngradeReasonClass,
    MirrorImportFinding, MirrorImportPermissionMetadata, MirrorImportReasonClass,
    MirrorImportRouteClass, MirrorImportSupportExplanationClass, MirrorImportSupportExportRecord,
    MirrorImportTrustClaimClass, MirrorImportTrustClaimEntry, MirrorImportTrustClaimStateClass,
    MIRROR_IMPORT_BASELINE_RECORD_KIND, MIRROR_IMPORT_BASELINE_SCHEMA_VERSION,
    MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND,
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
pub use revocation::{
    evaluate_extension_incident_communication, project_extension_incident_support_export,
    validate_extension_incident_communication_record,
    validate_extension_incident_support_export_record, AdvisorySeverityClass,
    ExtensionIncidentAction, ExtensionIncidentActionClass, ExtensionIncidentActorClass,
    ExtensionIncidentAdvisory, ExtensionIncidentBlockedOperationClass,
    ExtensionIncidentCommunicationInput, ExtensionIncidentCommunicationRecord,
    ExtensionIncidentDecisionClass, ExtensionIncidentDecisionReasonClass,
    ExtensionIncidentDisclosureClass, ExtensionIncidentFinding,
    ExtensionIncidentLifecycleStateClass, ExtensionIncidentReasonCode,
    ExtensionIncidentRecoveryActionClass, ExtensionIncidentRecoveryGuidance,
    ExtensionIncidentRegistryLane, ExtensionIncidentRegistryLaneClass,
    ExtensionIncidentSourceClass, ExtensionIncidentSubject, ExtensionIncidentSupportActionClass,
    ExtensionIncidentSupportExportRecord, ExtensionIncidentTrustStateClass,
    EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND, EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION,
    EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND,
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
pub use stabilize_external_host_contracts_for_language_tools_debuggers::{
    project_stable_external_host_contract, project_stable_external_host_contract_support_export,
    ExternalHostActivationBudget, ExternalHostActivationBudgetInput,
    ExternalHostCapabilityEnvelope, ExternalHostCapabilityEnvelopeInput,
    ExternalHostContractIdentity, ExternalHostContractIdentityInput,
    ExternalHostContractInspection, ExternalHostContractSupportExport,
    ExternalHostContributionEntry, ExternalHostContributionEntryInput,
    ExternalHostDataPlaneContract, ExternalHostDataPlaneContractInput,
    ExternalHostDowngradedBanner, ExternalHostKindDeclaration, ExternalHostKindDeclarationInput,
    ExternalHostQualificationClaim, ExternalHostQualificationClaimInput,
    ExternalHostReconnectReplaySafety, ExternalHostReconnectReplaySafetyInput,
    ExternalHostSandboxBinding, ExternalHostSandboxBindingInput, StableExternalHostContractInput,
    StableExternalHostContractPacket, StableExternalHostContractProjection, StableExternalHostError,
    StableExternalHostValidationError, ADAPTER_ORIGIN_CLASSES, AUTH_SOURCE_MODE_CLASSES,
    CONNECTION_STATE_CLASSES, CONNECTION_TARGET_CLASSES, CONTROL_PLANE_BOUNDARY_CLASSES,
    DATA_PLANE_HOST_KINDS, EXTERNAL_HOST_BACKEND_CLASSES, EXTERNAL_HOST_DOWNGRADE_REASONS,
    EXTERNAL_HOST_EXECUTION_LOCUS_CLASSES, EXTERNAL_HOST_KIND_CLASSES,
    HOST_PROTOCOL_CLASSES, PENDING_SIDE_EFFECT_CLASSES, REATTACH_POLICY_CLASSES,
    RESULT_EXPORT_SAFETY_CLASSES, SIDE_EFFECTING_PENDING_CLASSES,
    EXTERNAL_HOST_SUPPORT_EXPORT_RECORD_KIND, STABLE_EXTERNAL_HOST_CONSUMER_SURFACES,
    STABLE_EXTERNAL_HOST_PACKET_RECORD_KIND, STABLE_EXTERNAL_HOST_PUBLISHED_ABI_VERSION,
    STABLE_EXTERNAL_HOST_SCHEMA_REF, STABLE_EXTERNAL_HOST_SCHEMA_VERSION, WRITE_POSTURE_CLASSES,
    ACTIVATION_BUDGET_CLASSES as EXTERNAL_HOST_ACTIVATION_BUDGET_CLASSES,
    CLAIM_BASIS_CLASSES as EXTERNAL_HOST_CLAIM_BASIS_CLASSES,
    CONTRIBUTION_HOST_STATE_CLASSES as EXTERNAL_HOST_CONTRIBUTION_HOST_STATE_CLASSES,
    CONTRIBUTION_KIND_CLASSES as EXTERNAL_HOST_CONTRIBUTION_KIND_CLASSES,
    LIFECYCLE_STATE_CLASSES as EXTERNAL_HOST_LIFECYCLE_STATE_CLASSES,
    RESTART_POSTURE_CLASSES as EXTERNAL_HOST_RESTART_POSTURE_CLASSES,
    RUNNABLE_LIFECYCLE_STATES as EXTERNAL_HOST_RUNNABLE_LIFECYCLE_STATES,
    SANDBOX_ENFORCEMENT_STATES as EXTERNAL_HOST_SANDBOX_ENFORCEMENT_STATES,
    STABILITY_TIERS as EXTERNAL_HOST_STABILITY_TIERS,
    STABLE_TIERS as EXTERNAL_HOST_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as EXTERNAL_HOST_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as EXTERNAL_HOST_TRUST_TIER_CLASSES,
};
pub use stabilize_mirror_manual_import_offline_catalog_and_publisher::{
    project_stable_mirror_import_truth, project_stable_mirror_import_truth_support_export,
    DowngradedImportBanner, MirrorImportActivationBudget, MirrorImportActivationBudgetInput,
    MirrorImportCompatibility, MirrorImportCompatibilityInput, MirrorImportContinuity,
    MirrorImportContinuityInput, MirrorImportInstallPosture, MirrorImportInstallPostureInput,
    MirrorImportMappingOutcome, MirrorImportMappingOutcomeInput, MirrorImportPermissionPosture,
    MirrorImportPermissionPostureInput, MirrorImportSourceClass, MirrorImportSourceClassInput,
    MirrorImportTruthIdentity, MirrorImportTruthIdentityInput, MirrorImportTruthQualificationClaim,
    MirrorImportTruthQualificationClaimInput, StableMirrorImportTruthError,
    StableMirrorImportTruthInput, StableMirrorImportTruthInspection, StableMirrorImportTruthPacket,
    StableMirrorImportTruthProjection, StableMirrorImportTruthSupportExport,
    StableMirrorImportTruthValidationError, ACTIVATION_BUDGET_CLASSES as MIRROR_IMPORT_ACTIVATION_BUDGET_CLASSES,
    CLAIM_BASIS_CLASSES as MIRROR_IMPORT_CLAIM_BASIS_CLASSES,
    COMPATIBILITY_EVIDENCE_SOURCE_CLASSES as MIRROR_IMPORT_COMPATIBILITY_EVIDENCE_SOURCE_CLASSES,
    COMPATIBILITY_LABEL_CLASSES as MIRROR_IMPORT_COMPATIBILITY_LABEL_CLASSES,
    CONTINUITY_EVENT_CLASSES, CONTINUITY_STATE_CLASSES,
    DOWNGRADED_IMPORT_BANNER_RECORD_KIND, IMPORT_ROUTE_CLASSES,
    INSTALLABLE_LIFECYCLE_STATES as MIRROR_IMPORT_INSTALLABLE_LIFECYCLE_STATES,
    INSTALL_SCOPE_CLASSES as MIRROR_IMPORT_INSTALL_SCOPE_CLASSES,
    LIFECYCLE_STATE_CLASSES as MIRROR_IMPORT_LIFECYCLE_STATE_CLASSES, MAPPING_OUTCOME_CLASSES,
    MIRRORABILITY_CLASSES, MIRROR_IMPORT_ACTIVATION_BUDGET_RECORD_KIND,
    MIRROR_IMPORT_COMPATIBILITY_RECORD_KIND, MIRROR_IMPORT_CONTINUITY_RECORD_KIND,
    MIRROR_IMPORT_DOWNGRADE_REASONS, MIRROR_IMPORT_INSTALL_POSTURE_RECORD_KIND,
    MIRROR_IMPORT_MAPPING_OUTCOME_RECORD_KIND, MIRROR_IMPORT_PERMISSION_POSTURE_RECORD_KIND,
    MIRROR_IMPORT_SOURCE_CLASS_RECORD_KIND, MIRROR_IMPORT_TRUTH_IDENTITY_RECORD_KIND,
    MIRROR_IMPORT_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND, REVOCATION_POSTURE_CLASSES,
    SOURCE_VISIBILITY_CLASSES, STABILITY_TIERS as MIRROR_IMPORT_STABILITY_TIERS,
    STABLE_GRADE_MAPPING_OUTCOMES, STABLE_MIRROR_IMPORT_CONSUMER_SURFACES,
    STABLE_MIRROR_IMPORT_PUBLISHED_VERSION, STABLE_MIRROR_IMPORT_TRUTH_INSPECTION_RECORD_KIND,
    STABLE_MIRROR_IMPORT_TRUTH_PACKET_RECORD_KIND, STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_REF,
    STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION, STABLE_MIRROR_IMPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
    STABLE_TIERS as MIRROR_IMPORT_STABLE_TIERS, SUPPORT_CLAIM_CLASSES as MIRROR_IMPORT_SUPPORT_CLAIM_CLASSES,
    TRANSFER_CONTINUITY_EVENTS, TRUST_TIER_CLASSES as MIRROR_IMPORT_TRUST_TIER_CLASSES,
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
pub use webview_boundary::{
    audit_extension_webview_boundary_rows, evaluate_extension_webview_boundary_row,
    project_extension_webview_boundary_support_export,
    project_extension_webview_boundary_support_row, seeded_extension_webview_boundary_audit_packet,
    seeded_extension_webview_boundary_inputs, validate_extension_webview_boundary_packet,
    validate_extension_webview_boundary_row, validate_extension_webview_boundary_support_export,
    ExtensionAppearanceInheritance, ExtensionBoundaryStateClass,
    ExtensionBrowserHandoffPostureClass, ExtensionBrowserHandoffReasonClass,
    ExtensionEmbeddedSurfaceClass, ExtensionFallbackTargetClass, ExtensionHostAuthorityScopeClass,
    ExtensionHostChromeControlClass, ExtensionInheritanceClass,
    ExtensionNativeApprovalBoundaryClass, ExtensionOriginClass, ExtensionSurfacePermissionClass,
    ExtensionSurfaceTrustClass, ExtensionWebviewBoundaryAuditPacket,
    ExtensionWebviewBoundaryDecisionClass, ExtensionWebviewBoundaryDefect,
    ExtensionWebviewBoundaryDefectKind, ExtensionWebviewBoundaryInput, ExtensionWebviewBoundaryRow,
    ExtensionWebviewBoundarySummary, ExtensionWebviewBoundarySupportExport,
    ExtensionWebviewBoundarySupportRow, EXTENSION_WEBVIEW_BOUNDARY_AUDIT_PACKET_RECORD_KIND,
    EXTENSION_WEBVIEW_BOUNDARY_DEFECT_RECORD_KIND, EXTENSION_WEBVIEW_BOUNDARY_ROW_RECORD_KIND,
    EXTENSION_WEBVIEW_BOUNDARY_SCHEMA_VERSION, EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF,
    EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND,
    EXTENSION_WEBVIEW_BOUNDARY_SUPPORT_ROW_RECORD_KIND,
};
pub use finalize_wasm_host_quotas_crash_loop_quarantine_and::{
    project_stable_wasm_host_governance, project_stable_wasm_host_governance_support_export,
    CrashLoopGovernance, CrashLoopGovernanceInput, GovernanceDowngradedHostBanner,
    GovernanceIdentity, GovernanceIdentityInput, GovernanceQualificationClaim,
    GovernanceQualificationClaimInput, GovernanceRuntimeClassDeclaration,
    GovernanceRuntimeClassDeclarationInput, GovernedContributionEntry,
    GovernedContributionEntryInput, HostQuotaAxis, HostQuotaAxisInput, QuarantinePosture,
    QuarantinePostureInput, RestartBudgetGovernance, RestartBudgetGovernanceInput,
    StableWasmHostGovernanceError, StableWasmHostGovernanceInput, StableWasmHostGovernanceInspection,
    StableWasmHostGovernancePacket, StableWasmHostGovernanceProjection,
    StableWasmHostGovernanceSupportExport, StableWasmHostGovernanceValidationError,
    CLAIM_BASIS_CLASSES as WASM_HOST_GOVERNANCE_CLAIM_BASIS_CLASSES,
    CONTRIBUTION_HOST_STATE_CLASSES as WASM_HOST_GOVERNANCE_CONTRIBUTION_HOST_STATE_CLASSES,
    CONTRIBUTION_KIND_CLASSES as WASM_HOST_GOVERNANCE_CONTRIBUTION_KIND_CLASSES,
    CRASH_LOOP_GOVERNANCE_RECORD_KIND, CRASH_LOOP_STATE_CLASSES,
    EXECUTION_LOCUS_CLASSES as WASM_HOST_GOVERNANCE_EXECUTION_LOCUS_CLASSES,
    GOVERNANCE_DOWNGRADED_HOST_BANNER_RECORD_KIND, GOVERNANCE_DOWNGRADE_REASONS,
    GOVERNANCE_IDENTITY_RECORD_KIND, GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND,
    GOVERNANCE_RUNTIME_CLASS_DECLARATION_RECORD_KIND, GOVERNED_CONTRIBUTION_ENTRY_RECORD_KIND,
    HOST_QUOTA_AXIS_RECORD_KIND,
    LIFECYCLE_STATE_CLASSES as WASM_HOST_GOVERNANCE_LIFECYCLE_STATE_CLASSES,
    QUARANTINE_POSTURE_RECORD_KIND, QUARANTINE_STATE_CLASSES, QUOTA_AXIS_CLASSES,
    QUOTA_ENFORCEMENT_STATES, QUOTA_PRESSURE_CLASSES, RECOVERY_PRECONDITION_CLASSES,
    RESTART_BUDGET_GOVERNANCE_RECORD_KIND,
    RESTART_POSTURE_CLASSES as WASM_HOST_GOVERNANCE_RESTART_POSTURE_CLASSES,
    RUNNABLE_LIFECYCLE_STATES as WASM_HOST_GOVERNANCE_RUNNABLE_LIFECYCLE_STATES,
    RUNTIME_CLASSES as WASM_HOST_GOVERNANCE_RUNTIME_CLASSES,
    STABILITY_TIERS as WASM_HOST_GOVERNANCE_STABILITY_TIERS,
    STABLE_GOVERNANCE_PUBLISHED_VERSION, STABLE_TIERS as WASM_HOST_GOVERNANCE_STABLE_TIERS,
    STABLE_WASM_HOST_GOVERNANCE_CONSUMER_SURFACES,
    STABLE_WASM_HOST_GOVERNANCE_INSPECTION_RECORD_KIND,
    STABLE_WASM_HOST_GOVERNANCE_PACKET_RECORD_KIND, STABLE_WASM_HOST_GOVERNANCE_SCHEMA_REF,
    STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
    STABLE_WASM_HOST_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND,
    SUPPORT_CLAIM_CLASSES as WASM_HOST_GOVERNANCE_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as WASM_HOST_GOVERNANCE_TRUST_TIER_CLASSES,
};
pub use stabilize_extension_runtime_v1_abi_capability_envelopes_and::{
    project_stable_runtime_abi, project_stable_runtime_abi_support_export,
    ActivationBudget as StableRuntimeActivationBudget,
    ActivationBudgetInput as StableRuntimeActivationBudgetInput,
    ActiveContributionInspectorEntry, ActiveContributionInspectorEntryInput, CapabilityEnvelope,
    CapabilityEnvelopeInput,
    DowngradedHostBanner, HostIsolationPosture, HostIsolationPostureInput,
    RuntimeAbiIdentity, RuntimeAbiIdentityInput, RuntimeAbiQualificationClaim,
    RuntimeAbiQualificationClaimInput, RuntimeClassDeclaration, RuntimeClassDeclarationInput,
    SandboxProfileBinding, SandboxProfileBindingInput, StableRuntimeAbiError,
    StableRuntimeAbiInput, StableRuntimeAbiInspection, StableRuntimeAbiPacket,
    StableRuntimeAbiProjection, StableRuntimeAbiSupportExport, StableRuntimeAbiValidationError,
    ACTIVATION_BUDGET_CLASSES, ACTIVATION_BUDGET_RECORD_KIND,
    ACTIVE_CONTRIBUTION_INSPECTOR_ENTRY_RECORD_KIND, BACKEND_CLASSIFICATION_CLASSES,
    CAPABILITY_ENVELOPE_RECORD_KIND, CLAIM_BASIS_CLASSES as STABLE_RUNTIME_ABI_CLAIM_BASIS_CLASSES,
    CONTRIBUTION_HOST_STATE_CLASSES, CONTRIBUTION_KIND_CLASSES, DOWNGRADED_HOST_BANNER_RECORD_KIND,
    EXECUTION_LOCUS_CLASSES, HOST_ISOLATION_POSTURE_RECORD_KIND, ISOLATION_BOUNDARY_CLASSES,
    LIFECYCLE_STATE_CLASSES as STABLE_RUNTIME_ABI_LIFECYCLE_STATE_CLASSES,
    NON_EXECUTING_RUNTIME_CLASSES, RESTART_POSTURE_CLASSES as STABLE_RUNTIME_ABI_RESTART_POSTURE_CLASSES,
    RUNNABLE_LIFECYCLE_STATES, RUNTIME_ABI_DOWNGRADE_REASONS, RUNTIME_ABI_IDENTITY_RECORD_KIND,
    RUNTIME_ABI_QUALIFICATION_CLAIM_RECORD_KIND, RUNTIME_CLASSES,
    RUNTIME_CLASS_DECLARATION_RECORD_KIND, SANDBOXED_RUNTIME_CLASSES,
    SANDBOX_ENFORCEMENT_STATES, SANDBOX_PROFILE_BINDING_RECORD_KIND,
    STABILITY_TIERS as STABLE_RUNTIME_ABI_STABILITY_TIERS, STABLE_RUNTIME_ABI_CONSUMER_SURFACES,
    STABLE_RUNTIME_ABI_INSPECTION_RECORD_KIND, STABLE_RUNTIME_ABI_PACKET_RECORD_KIND,
    STABLE_RUNTIME_ABI_PUBLISHED_VERSION, STABLE_RUNTIME_ABI_SCHEMA_REF,
    STABLE_RUNTIME_ABI_SCHEMA_VERSION, STABLE_RUNTIME_ABI_SUPPORT_EXPORT_RECORD_KIND,
    STABLE_TIERS as STABLE_RUNTIME_ABI_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as STABLE_RUNTIME_ABI_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as STABLE_RUNTIME_ABI_TRUST_TIER_CLASSES,
};
pub use harden_extension_manifest_permission_display_lifecycle_labels_and::{
    project_stable_manifest_hardening, project_stable_manifest_hardening_support_export,
    DowngradedManifestBanner, EffectivePermissionDiffEntry, EffectivePermissionResolution,
    ManifestCompatibilityRange, ManifestCompatibilityRangeInput, ManifestDependencyEdge,
    ManifestDependencyEdgeInput, ManifestHardeningIdentity, ManifestHardeningIdentityInput,
    ManifestHardeningQualificationClaim, ManifestHardeningQualificationClaimInput,
    ManifestLifecycleLabel, ManifestLifecycleLabelInput, ManifestPermissionEntry,
    ManifestPermissionEntryInput, StableManifestHardeningError, StableManifestHardeningInput,
    StableManifestHardeningInspection, StableManifestHardeningPacket,
    StableManifestHardeningProjection, StableManifestHardeningSupportExport,
    StableManifestHardeningValidationError,
    CLAIM_BASIS_CLASSES as MANIFEST_HARDENING_CLAIM_BASIS_CLASSES,
    DEPENDENCY_CLASSES as MANIFEST_HARDENING_DEPENDENCY_CLASSES,
    DEPENDENCY_RESOLUTION_STATE_CLASSES as MANIFEST_HARDENING_DEPENDENCY_RESOLUTION_STATE_CLASSES,
    DEPRECATION_CLASSES as MANIFEST_HARDENING_DEPRECATION_CLASSES,
    DOWNGRADED_MANIFEST_BANNER_RECORD_KIND, EFFECTIVE_PERMISSION_RESOLUTION_RECORD_KIND,
    INSTALLABLE_LIFECYCLE_STATES as MANIFEST_HARDENING_INSTALLABLE_LIFECYCLE_STATES,
    LIFECYCLE_STATE_CLASSES as MANIFEST_HARDENING_LIFECYCLE_STATE_CLASSES,
    MANIFEST_COMPATIBILITY_RANGE_RECORD_KIND, MANIFEST_DEPENDENCY_EDGE_RECORD_KIND,
    MANIFEST_HARDENING_DOWNGRADE_REASONS, MANIFEST_HARDENING_IDENTITY_RECORD_KIND,
    MANIFEST_HARDENING_QUALIFICATION_CLAIM_RECORD_KIND, MANIFEST_LIFECYCLE_LABEL_RECORD_KIND,
    PERMISSION_CAPABILITY_CLASSES as MANIFEST_HARDENING_PERMISSION_CAPABILITY_CLASSES,
    PERMISSION_SOURCE_CLASSES as MANIFEST_HARDENING_PERMISSION_SOURCE_CLASSES,
    RANGE_RESOLUTION_CLASSES as MANIFEST_HARDENING_RANGE_RESOLUTION_CLASSES,
    STABILITY_TIERS as MANIFEST_HARDENING_STABILITY_TIERS,
    STABLE_MANIFEST_HARDENING_CONSUMER_SURFACES,
    STABLE_MANIFEST_HARDENING_INSPECTION_RECORD_KIND,
    STABLE_MANIFEST_HARDENING_PACKET_RECORD_KIND,
    STABLE_MANIFEST_HARDENING_PUBLISHED_MANIFEST_VERSION, STABLE_MANIFEST_HARDENING_SCHEMA_REF,
    STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
    STABLE_MANIFEST_HARDENING_SUPPORT_EXPORT_RECORD_KIND,
    STABLE_TIERS as MANIFEST_HARDENING_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as MANIFEST_HARDENING_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as MANIFEST_HARDENING_TRUST_TIER_CLASSES,
};
pub use stabilize_sdk_schemas_samples_templates_and_conformance_kits::{
    project_stable_sdk_author_lane, project_stable_sdk_author_lane_support_export,
    DowngradedLaneBanner, SdkActivationBudget, SdkActivationBudgetInput, SdkAuthorLaneIdentity,
    SdkAuthorLaneIdentityInput, SdkAuthorLaneQualificationClaim,
    SdkAuthorLaneQualificationClaimInput, SdkConformanceSummary, SdkKitArtifact, SdkKitArtifactInput,
    SdkPublisherContinuity, SdkPublisherContinuityInput, StableSdkAuthorLaneError,
    StableSdkAuthorLaneInput, StableSdkAuthorLaneInspection, StableSdkAuthorLanePacket,
    StableSdkAuthorLaneProjection, StableSdkAuthorLaneSupportExport,
    StableSdkAuthorLaneValidationError, ARTIFACT_HOST_CLASSES, ARTIFACT_KIND_CLASSES,
    CONFORMANCE_STATE_CLASSES, DOWNGRADED_LANE_BANNER_RECORD_KIND, PUBLISHER_CONTINUITY_CLASSES,
    REQUIRED_ARTIFACT_KINDS, SDK_ACTIVATION_BUDGET_RECORD_KIND,
    SDK_AUTHOR_LANE_DOWNGRADE_REASONS, SDK_AUTHOR_LANE_IDENTITY_RECORD_KIND,
    SDK_AUTHOR_LANE_QUALIFICATION_CLAIM_RECORD_KIND, SDK_CONFORMANCE_SUMMARY_RECORD_KIND,
    SDK_KIT_ARTIFACT_RECORD_KIND, SDK_PUBLISHER_CONTINUITY_RECORD_KIND,
    STABLE_SDK_AUTHOR_LANE_CONSUMER_SURFACES, STABLE_SDK_AUTHOR_LANE_INSPECTION_RECORD_KIND,
    STABLE_SDK_AUTHOR_LANE_PACKET_RECORD_KIND,
    STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION,
    STABLE_SDK_AUTHOR_LANE_PUBLISHED_SDK_VERSION, STABLE_SDK_AUTHOR_LANE_SCHEMA_REF,
    STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION, STABLE_SDK_AUTHOR_LANE_SUPPORT_EXPORT_RECORD_KIND,
    ACTIVATION_BUDGET_CLASSES as SDK_AUTHOR_LANE_ACTIVATION_BUDGET_CLASSES,
    CLAIM_BASIS_CLASSES as SDK_AUTHOR_LANE_CLAIM_BASIS_CLASSES,
    INSTALLABLE_LIFECYCLE_STATES as SDK_AUTHOR_LANE_INSTALLABLE_LIFECYCLE_STATES,
    LIFECYCLE_STATE_CLASSES as SDK_AUTHOR_LANE_LIFECYCLE_STATE_CLASSES,
    STABILITY_TIERS as SDK_AUTHOR_LANE_STABILITY_TIERS,
    STABLE_TIERS as SDK_AUTHOR_LANE_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as SDK_AUTHOR_LANE_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as SDK_AUTHOR_LANE_TRUST_TIER_CLASSES,
};
pub use harden_extension_performance_inspection_budget_enforcement_and_user::{
    project_stable_performance_budget, project_stable_performance_budget_support_export,
    PerformanceBudgetEnforcement, PerformanceBudgetEnforcementInput, PerformanceBudgetIdentity,
    PerformanceBudgetIdentityInput, PerformanceBudgetWaiver, PerformanceBudgetWaiverInput,
    PerformanceCompatibility, PerformanceCompatibilityInput, PerformanceCostExplanation,
    PerformanceCostExplanationInput, PerformanceDowngradedBanner, PerformanceInstallPosture,
    PerformanceInstallPostureInput, PerformanceMeasurement, PerformanceMeasurementInput,
    PerformancePermissionPosture, PerformancePermissionPostureInput, PerformanceQualificationClaim,
    PerformanceQualificationClaimInput, StablePerformanceBudgetError, StablePerformanceBudgetInput,
    StablePerformanceBudgetInspection, StablePerformanceBudgetPacket,
    StablePerformanceBudgetProjection, StablePerformanceBudgetSupportExport,
    StablePerformanceBudgetValidationError, BUDGET_AXIS_CLASSES, BUDGET_STATUS_CLASSES,
    COST_CLASSES, ENFORCEMENT_MODE_CLASSES, MEASUREMENT_FRESHNESS_CLASSES,
    PERFORMANCE_BUDGET_DOWNGRADE_REASONS, PERFORMANCE_BUDGET_ENFORCEMENT_RECORD_KIND,
    PERFORMANCE_BUDGET_IDENTITY_RECORD_KIND, PERFORMANCE_BUDGET_WAIVER_RECORD_KIND,
    PERFORMANCE_COMPATIBILITY_RECORD_KIND, PERFORMANCE_COST_EXPLANATION_RECORD_KIND,
    PERFORMANCE_DOWNGRADED_BANNER_RECORD_KIND, PERFORMANCE_INSTALL_POSTURE_RECORD_KIND,
    PERFORMANCE_MEASUREMENT_RECORD_KIND, PERFORMANCE_PERMISSION_POSTURE_RECORD_KIND,
    PERFORMANCE_QUALIFICATION_CLAIM_RECORD_KIND, STABLE_PERFORMANCE_BUDGET_CONSUMER_SURFACES,
    STABLE_PERFORMANCE_BUDGET_INSPECTION_RECORD_KIND, STABLE_PERFORMANCE_BUDGET_PACKET_RECORD_KIND,
    STABLE_PERFORMANCE_BUDGET_PUBLISHED_PROFILE_VERSION, STABLE_PERFORMANCE_BUDGET_SCHEMA_REF,
    STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION, STABLE_PERFORMANCE_BUDGET_SUPPORT_EXPORT_RECORD_KIND,
    THRESHOLD_ADJUSTMENT_CLASSES, WAIVER_AUTHORITY_CLASSES, WAIVER_STATE_CLASSES,
    CLAIM_BASIS_CLASSES as PERFORMANCE_BUDGET_CLAIM_BASIS_CLASSES,
    COMPATIBILITY_LABEL_CLASSES as PERFORMANCE_BUDGET_COMPATIBILITY_LABEL_CLASSES,
    INSTALL_SCOPE_CLASSES as PERFORMANCE_BUDGET_INSTALL_SCOPE_CLASSES,
    LIFECYCLE_STATE_CLASSES as PERFORMANCE_BUDGET_LIFECYCLE_STATE_CLASSES,
    MIRRORABILITY_CLASSES as PERFORMANCE_BUDGET_MIRRORABILITY_CLASSES,
    REVOCATION_POSTURE_CLASSES as PERFORMANCE_BUDGET_REVOCATION_POSTURE_CLASSES,
    RUNNABLE_LIFECYCLE_STATES as PERFORMANCE_BUDGET_RUNNABLE_LIFECYCLE_STATES,
    STABILITY_TIERS as PERFORMANCE_BUDGET_STABILITY_TIERS,
    STABLE_TIERS as PERFORMANCE_BUDGET_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as PERFORMANCE_BUDGET_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as PERFORMANCE_BUDGET_TRUST_TIER_CLASSES,
};
pub use harden_install_review_update_review_disable_rollback_and::{
    project_stable_lifecycle_flow, project_stable_lifecycle_flow_support_export,
    DependencyNode, DependencyNodeInput, DeterministicResolution, DeterministicResolutionInput,
    DisableRollbackPosture, DisableRollbackPostureInput, DowngradedFlowBanner,
    EffectivePermissionInheritance, EffectivePermissionInheritanceInput, LifecycleFlowIdentity,
    LifecycleFlowIdentityInput, LifecycleFlowInspection, LifecycleFlowQualificationClaim,
    LifecycleFlowQualificationClaimInput, LockExportPlan, LockExportPlanInput, ReConsentRequirement,
    ReConsentRequirementInput, RevocationPosture, RevocationPostureInput, StableLifecycleFlowError,
    StableLifecycleFlowInput, StableLifecycleFlowPacket, StableLifecycleFlowProjection,
    StableLifecycleFlowSupportExport, StableLifecycleFlowValidationError,
    DEPENDENCY_NODE_KIND_CLASSES, DEPENDENCY_NODE_RECORD_KIND,
    DEPENDENCY_RESOLUTION_STATE_CLASSES as LIFECYCLE_FLOW_DEPENDENCY_RESOLUTION_STATE_CLASSES,
    DETERMINISTIC_RESOLUTION_RECORD_KIND, DISABLE_ROLLBACK_POSTURE_RECORD_KIND,
    DOWNGRADED_FLOW_BANNER_RECORD_KIND, EFFECTIVE_PERMISSION_INHERITANCE_RECORD_KIND,
    FLOW_CLASSES, INSTALL_SCOPE_CLASSES, INSTALL_SHAPED_FLOWS,
    LIFECYCLE_FLOW_DOWNGRADE_REASONS, LIFECYCLE_FLOW_IDENTITY_RECORD_KIND,
    LIFECYCLE_FLOW_INSPECTION_RECORD_KIND, LIFECYCLE_FLOW_QUALIFICATION_CLAIM_RECORD_KIND,
    LOCK_EXPORT_PLAN_RECORD_KIND, LOCK_EXPORT_STATE_CLASSES, PERMISSION_EXPANSION_CLASSES,
    RECONSENT_REQUIREMENT_RECORD_KIND, RECONSENT_STATE_CLASSES, RESOLVER_DETERMINISM_CLASSES,
    REVOCATION_POSTURE_RECORD_KIND, REVOCATION_PROPAGATION_CLASSES, REVOCATION_STATE_CLASSES,
    ROLLBACK_STATE_CLASSES, STABLE_LIFECYCLE_FLOW_CONSUMER_SURFACES,
    STABLE_LIFECYCLE_FLOW_PACKET_RECORD_KIND, STABLE_LIFECYCLE_FLOW_PUBLISHED_VERSION,
    STABLE_LIFECYCLE_FLOW_SCHEMA_REF, STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
    STABLE_LIFECYCLE_FLOW_SUPPORT_EXPORT_RECORD_KIND, SUBJECT_CLASSES,
    CLAIM_BASIS_CLASSES as LIFECYCLE_FLOW_CLAIM_BASIS_CLASSES,
    INSTALLABLE_LIFECYCLE_STATES as LIFECYCLE_FLOW_INSTALLABLE_LIFECYCLE_STATES,
    LIFECYCLE_STATE_CLASSES as LIFECYCLE_FLOW_LIFECYCLE_STATE_CLASSES,
    STABILITY_TIERS as LIFECYCLE_FLOW_STABILITY_TIERS,
    STABLE_TIERS as LIFECYCLE_FLOW_STABLE_TIERS,
    SUPPORT_CLAIM_CLASSES as LIFECYCLE_FLOW_SUPPORT_CLAIM_CLASSES,
    TRUST_TIER_CLASSES as LIFECYCLE_FLOW_TRUST_TIER_CLASSES,
};
