//! Portable change-object contracts shared by review, shell, and support paths.
//!
//! This crate owns change-object families that are not live Git operations.
//! The first family is [`portable_bundle`], a metadata-safe bundle and shelf
//! record that can be exported, imported, inspected, or reopened without
//! implying code-host residency or live provider authority.

#![doc(html_root_url = "https://docs.rs/aureline-change-objects/0.0.0")]

pub mod portable_bundle;

pub use portable_bundle::{
    current_portable_bundle_fixture_projections, project_portable_bundle,
    PortableBundleAuthorityState, PortableBundleDiffRef, PortableBundleError,
    PortableBundleEvidenceRef, PortableBundleProjection, PortableBundleRecord,
    PortableBundleRedactionProfile, PortableBundleReviewInvariants,
    PortableBundleReviewPackBinding, PortableBundleSupportExportLineage,
    PortableBundleTargetIdentity, PortableBundleValidationError, PortableBundleValidationState,
    PORTABLE_BUNDLE_AUTHORITY_CLASSES, PORTABLE_BUNDLE_CONSUMER_SURFACES,
    PORTABLE_BUNDLE_DIFF_REF_CLASSES, PORTABLE_BUNDLE_DOC_REF,
    PORTABLE_BUNDLE_EVIDENCE_REF_CLASSES, PORTABLE_BUNDLE_FIXTURE_DIR,
    PORTABLE_BUNDLE_HANDOFF_PURPOSE_CLASSES, PORTABLE_BUNDLE_OBJECT_CLASSES,
    PORTABLE_BUNDLE_OPEN_MODE_CLASSES, PORTABLE_BUNDLE_RECORD_KIND,
    PORTABLE_BUNDLE_REDACTION_CLASSES, PORTABLE_BUNDLE_REVIEW_PACK_PARITY_CLASSES,
    PORTABLE_BUNDLE_SCHEMA_REF, PORTABLE_BUNDLE_SCHEMA_VERSION,
    PORTABLE_BUNDLE_STALE_VALIDATION_LABELS, PORTABLE_BUNDLE_STATE_CLASSES,
    PORTABLE_BUNDLE_SUPPORT_ARTIFACT_REF, PORTABLE_BUNDLE_SUPPORT_LINEAGE_CLASSES,
    PORTABLE_BUNDLE_TARGET_KIND_CLASSES, PORTABLE_BUNDLE_VALIDATION_FRESHNESS_CLASSES,
};
