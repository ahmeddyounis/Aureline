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

pub mod manifest_baseline;
pub mod review_alpha;

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
pub use review_alpha::{
    evaluate_extension_review_alpha, project_review_alpha_surface,
    validate_extension_review_alpha_packet, validate_publisher_continuity_alpha_record,
    validate_revocation_alpha_record, ExtensionReviewAlphaInput, ExtensionReviewAlphaPacketRecord,
    ExtensionReviewAlphaProjectionRecord, PolicyPackAlphaApplication, PolicyPackEffectClass,
    PublisherContinuityAlphaRecord, PublisherContinuityStateClass, ReviewActionClass,
    ReviewActionOfferClass, ReviewAlphaFinding, ReviewDecisionClass, ReviewDecisionReasonClass,
    ReviewDisclosureClass, ReviewMutationClass, ReviewSurfaceClass, RevocationAlphaRecord,
    RevocationSourceClass, RevocationStateClass, RevocationSubjectClass,
    EXTENSION_REVIEW_ALPHA_PACKET_RECORD_KIND, EXTENSION_REVIEW_ALPHA_PROJECTION_RECORD_KIND,
    PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND, REVIEW_ALPHA_SCHEMA_VERSION,
    REVOCATION_ALPHA_RECORD_KIND,
};
