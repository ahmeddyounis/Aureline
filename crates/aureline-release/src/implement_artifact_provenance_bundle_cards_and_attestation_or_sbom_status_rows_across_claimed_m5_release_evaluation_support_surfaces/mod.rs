//! One reusable M5 artifact-provenance-bundle-card / attestation-or-SBOM
//! status-row primitive: artifact identity, digest set, signature state,
//! attestation state, SBOM/notice bundle state, immutable-digest lineage,
//! inventory format, generator version, scope, freshness, export availability,
//! mirror refs, and compare/export actions, projected the same way across every
//! claimed M5 release, enterprise-evaluation, and support surface.
//!
//! Aureline's frozen release-center component matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`])
//! names the artifact-provenance-bundle card as one governed component family and
//! freezes the controlled vocabulary it depends on — the signature statuses, the
//! attestation statuses, the SBOM statuses, the digest-lineage states, the
//! publication surface families, the deployment lines, the accessibility routes,
//! the qualification classes, and the downgrade triggers. This module *implements*
//! that provenance-bundle contract as one reusable card-plus-status-rows primitive
//! so a user can tell — from the card and its status rows alone — exactly what an
//! artifact's provenance actually proves, *without* unpacking raw archives or
//! reaching for external tooling first, and so the mere presence of an SBOM or an
//! attestation never reads like a stronger security or licensing guarantee than the
//! component actually proves.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_provenance_bundle`] — that takes one artifact's
//!    identity, digest set, signature status, attestation status, SBOM status,
//!    notice-bundle status, digest-lineage state, inventory format / scope /
//!    freshness / generator version / export availability, mirror refs, and
//!    compare/export availability, and produces one [`M5ResolvedProvenanceBundle`]
//!    carrying the derived trust posture (proven versus narrowed versus blocked),
//!    the separated attestation/SBOM/notice status rows, the compare/export
//!    binding, and — whenever the provenance is narrowed or blocked — a
//!    self-contained [`M5ProvenanceBlockedBanner`] that names the exact reason, the
//!    bound artifact, its digest, its mirror refs, and the next action rather than a
//!    generic `provenance unavailable`. The resolver derives trust from the
//!    signature and the digest lineage — never from inventory presence — keeps the
//!    inventory format, generator version, scope, freshness, and export availability
//!    on their own status rows separate from signature verification, and preserves
//!    explicit `Not provided` and `Partial` states wherever evidence is missing or
//!    scoped.
//! 2. A parity matrix — [`M5ProvenanceBundlePrimitivePacket`] — that binds one row
//!    per claimed M5 provenance consumer (the release-center provenance card, the
//!    enterprise-evaluation provenance sheet, the CLI provenance inspect, the admin
//!    provenance report, and the support provenance export) to the shared card
//!    anatomy, the same signature / attestation / SBOM / digest-lineage vocabulary,
//!    the same inventory formats, scopes, freshnesses, and export availabilities,
//!    the same trust postures, block reasons, next actions, export fields, and
//!    non-visual accessibility routes, so provenance and inventory truth stays
//!    identical across the release center, enterprise evaluation, the CLI,
//!    admin/reporting, and support.
//!
//! The signature status ([`M5SignatureStatus`]), attestation status
//! ([`M5AttestationStatus`]), SBOM status ([`M5SbomStatus`]), digest-lineage state
//! ([`M5DigestLineageState`]), publication surface family
//! ([`M5PublicationSurfaceFamily`]), deployment line ([`M5DeploymentLine`]),
//! release-center consumer surface ([`M5ReleaseCenterConsumerSurface`]),
//! accessibility route ([`M5ReleaseCenterAccessibilityRoute`]), qualification class
//! ([`M5ReleaseCenterQualificationClass`]), and downgrade trigger
//! ([`M5ReleaseCenterDowngradeTrigger`]) are reused verbatim from the frozen
//! release-center component matrix. This module mints new vocabulary only for what
//! that matrix left implicit about the provenance card and its status rows
//! themselves: its provenance consumer families, its card anatomy parts, its
//! inventory kinds, formats, scopes, freshnesses, and export availabilities, its
//! trust postures, its block reasons, its next actions, and its export fields. No
//! M5 provenance surface invents a second provenance or inventory grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user
//! text bodies stay outside the support boundary; every artifact id, digest,
//! generator version, and mirror ref is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-artifact-provenance-bundle-card.schema.json`](../../../../schemas/ui/m5-artifact-provenance-bundle-card.schema.json)
//! and the contract doc is
//! [`docs/release/m5_artifact_provenance_bundle_card_primitive_contract.md`](../../../../docs/release/m5_artifact_provenance_bundle_card_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-artifact-provenance-bundle-card-primitive/`](../../../../fixtures/ui/m5-artifact-provenance-bundle-card-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provenance_bundle_primitive_cli_provenance_inspect_preview_narrowed,
    seeded_m5_provenance_bundle_primitive_evaluation_provenance_sheet_beta_narrowed,
    seeded_m5_provenance_bundle_primitive_packet, M5_PROVENANCE_BUNDLE_PRIMITIVE_PACKET_ID,
};

// The signature status, attestation status, SBOM status, digest-lineage state,
// publication surface family, deployment line, release-center consumer surface,
// accessibility routes, qualification classes, and downgrade triggers are frozen
// once, in the release-center component matrix. This primitive reuses them verbatim
// so it never invents a parallel provenance or inventory vocabulary.
pub use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    M5AttestationStatus, M5DeploymentLine, M5DigestLineageState, M5PublicationSurfaceFamily,
    M5ReleaseCenterAccessibilityRoute, M5ReleaseCenterConsumerSurface,
    M5ReleaseCenterDowngradeTrigger, M5ReleaseCenterQualificationClass, M5SbomStatus,
    M5SignatureStatus,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProvenanceBundlePrimitivePacket`].
pub const M5_PROVENANCE_BUNDLE_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_artifact_provenance_bundle_cards_and_attestation_or_sbom_status_rows_across_claimed_m5_release_evaluation_support_surfaces";

/// Schema version for M5 provenance-bundle-primitive records.
pub const M5_PROVENANCE_BUNDLE_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the artifact-provenance-bundle-card boundary schema.
pub const M5_PROVENANCE_BUNDLE_SCHEMA_REF: &str =
    "schemas/ui/m5-artifact-provenance-bundle-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVENANCE_BUNDLE_DOC_REF: &str =
    "docs/release/m5_artifact_provenance_bundle_card_primitive_contract.md";

/// Repo-relative path of the frozen release-center component matrix this primitive
/// narrows from.
pub const M5_PROVENANCE_BUNDLE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the release-center object-model contract this primitive
/// binds against.
pub const M5_PROVENANCE_BUNDLE_OBJECT_MODEL_REF: &str =
    "docs/release/release_center_object_model_contract.md";

/// Repo-relative path of the artifact-verification contract this primitive projects
/// provenance and inventory truth from.
pub const M5_PROVENANCE_BUNDLE_VERIFICATION_CONTRACT_REF: &str =
    "docs/release/artifact_verification_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVENANCE_BUNDLE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-artifact-provenance-bundle-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVENANCE_BUNDLE_ARTIFACT_REF: &str =
    "artifacts/release/m5-artifact-provenance-bundle-card-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVENANCE_BUNDLE_CSV_REF: &str =
    "artifacts/release/m5-artifact-provenance-bundle-card-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PROVENANCE_BUNDLE_REPORT_REF: &str =
    "artifacts/components/m5-artifact-provenance-bundle-and-attestation-sbom-status-primitive.md";

/// One claimed M5 provenance consumer that renders the shared provenance-bundle card
/// and its attestation/SBOM status rows. These are the consumers the acceptance
/// criteria name — the release center, enterprise evaluation, the CLI,
/// admin/reporting, and support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceBundleConsumerSurface {
    /// The release-center / shiproom provenance card.
    ReleaseCenterProvenanceCard,
    /// The enterprise-evaluation provenance sheet.
    EvaluationProvenanceSheet,
    /// The CLI provenance-inspect / headless surface.
    CliProvenanceInspect,
    /// The admin provenance report.
    AdminProvenanceReport,
    /// The support provenance export.
    SupportProvenanceExport,
}

impl M5ProvenanceBundleConsumerSurface {
    /// Every claimed provenance consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCenterProvenanceCard,
        Self::EvaluationProvenanceSheet,
        Self::CliProvenanceInspect,
        Self::AdminProvenanceReport,
        Self::SupportProvenanceExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterProvenanceCard => "release_center_provenance_card",
            Self::EvaluationProvenanceSheet => "evaluation_provenance_sheet",
            Self::CliProvenanceInspect => "cli_provenance_inspect",
            Self::AdminProvenanceReport => "admin_provenance_report",
            Self::SupportProvenanceExport => "support_provenance_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenterProvenanceCard => "Release-Center Provenance Card",
            Self::EvaluationProvenanceSheet => "Enterprise-Evaluation Provenance Sheet",
            Self::CliProvenanceInspect => "CLI Provenance Inspect",
            Self::AdminProvenanceReport => "Admin Provenance Report",
            Self::SupportProvenanceExport => "Support Provenance Export",
        }
    }
}

/// One anatomy part the shared provenance-bundle card / status rows surface. The
/// parts in [`M5ProvenanceBundleAnatomyPart::MANDATORY`] are required on every card
/// so a user can read what the provenance actually proves without unpacking an
/// archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceBundleAnatomyPart {
    /// The artifact identity.
    ArtifactIdentity,
    /// The immutable digest set.
    DigestSet,
    /// The signature state.
    SignatureState,
    /// The attestation state.
    AttestationState,
    /// The SBOM bundle state.
    SbomBundleState,
    /// The notice-bundle state.
    NoticeBundleState,
    /// The immutable-digest lineage state.
    DigestLineageState,
    /// The attestation/SBOM status rows (format, generator, scope, freshness, export).
    InventoryStatusRows,
    /// The mirror-reference list.
    MirrorRefList,
    /// The compare action.
    CompareAction,
    /// The export action.
    ExportAction,
    /// The derived trust verdict.
    TrustVerdict,
    /// The provenance-blocked banner (shown when narrowed or blocked).
    ProvenanceBlockedBanner,
}

impl M5ProvenanceBundleAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ArtifactIdentity,
        Self::DigestSet,
        Self::SignatureState,
        Self::AttestationState,
        Self::SbomBundleState,
        Self::NoticeBundleState,
        Self::DigestLineageState,
        Self::InventoryStatusRows,
        Self::MirrorRefList,
        Self::CompareAction,
        Self::ExportAction,
        Self::TrustVerdict,
        Self::ProvenanceBlockedBanner,
    ];

    /// The anatomy parts every provenance card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ArtifactIdentity,
        Self::DigestSet,
        Self::SignatureState,
        Self::InventoryStatusRows,
        Self::TrustVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentity => "artifact_identity",
            Self::DigestSet => "digest_set",
            Self::SignatureState => "signature_state",
            Self::AttestationState => "attestation_state",
            Self::SbomBundleState => "sbom_bundle_state",
            Self::NoticeBundleState => "notice_bundle_state",
            Self::DigestLineageState => "digest_lineage_state",
            Self::InventoryStatusRows => "inventory_status_rows",
            Self::MirrorRefList => "mirror_ref_list",
            Self::CompareAction => "compare_action",
            Self::ExportAction => "export_action",
            Self::TrustVerdict => "trust_verdict",
            Self::ProvenanceBlockedBanner => "provenance_blocked_banner",
        }
    }
}

/// Which inventory a status row describes, so an attestation, an SBOM, and a notice
/// bundle each keep their own status row rather than collapsing into one claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InventoryKind {
    /// A build attestation.
    Attestation,
    /// A software bill of materials.
    Sbom,
    /// A third-party notice / license bundle.
    Notice,
}

impl M5InventoryKind {
    /// Every inventory kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Attestation, Self::Sbom, Self::Notice];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attestation => "attestation",
            Self::Sbom => "sbom",
            Self::Notice => "notice",
        }
    }
}

/// Controlled inventory format — the format an attestation / SBOM / notice bundle is
/// carried in, so a status row never leaves its format implicit and can say
/// `Not provided` explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InventoryFormat {
    /// SPDX SBOM.
    SpdxSbom,
    /// CycloneDX SBOM.
    CycloneDxSbom,
    /// in-toto attestation.
    InTotoAttestation,
    /// SLSA provenance attestation.
    SlsaProvenance,
    /// A third-party notice manifest.
    NoticeManifest,
    /// No format is provided.
    NotProvidedFormat,
}

impl M5InventoryFormat {
    /// Every inventory format, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SpdxSbom,
        Self::CycloneDxSbom,
        Self::InTotoAttestation,
        Self::SlsaProvenance,
        Self::NoticeManifest,
        Self::NotProvidedFormat,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpdxSbom => "spdx_sbom",
            Self::CycloneDxSbom => "cyclone_dx_sbom",
            Self::InTotoAttestation => "in_toto_attestation",
            Self::SlsaProvenance => "slsa_provenance",
            Self::NoticeManifest => "notice_manifest",
            Self::NotProvidedFormat => "not_provided_format",
        }
    }
}

/// Controlled inventory scope — what an attestation / SBOM actually covers, so a
/// status row never shows a partial or absent inventory as a full closure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InventoryScope {
    /// The full dependency closure.
    FullClosure,
    /// Direct dependencies only.
    DirectDependenciesOnly,
    /// The runtime closure only.
    RuntimeClosureOnly,
    /// A partial scope, explicitly disclosed.
    PartialScope,
    /// No scope is provided.
    NotProvidedScope,
}

impl M5InventoryScope {
    /// Every inventory scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullClosure,
        Self::DirectDependenciesOnly,
        Self::RuntimeClosureOnly,
        Self::PartialScope,
        Self::NotProvidedScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullClosure => "full_closure",
            Self::DirectDependenciesOnly => "direct_dependencies_only",
            Self::RuntimeClosureOnly => "runtime_closure_only",
            Self::PartialScope => "partial_scope",
            Self::NotProvidedScope => "not_provided_scope",
        }
    }

    /// True when this scope is a full-closure claim.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FullClosure)
    }

    /// True when this scope is explicitly partial.
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::PartialScope)
    }

    /// True when this scope is explicitly not provided.
    pub const fn is_not_provided(self) -> bool {
        matches!(self, Self::NotProvidedScope)
    }
}

/// Controlled inventory freshness — how current an attestation / SBOM is relative to
/// the built artifact, so a stale or absent inventory never reads as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InventoryFreshness {
    /// Fresh within its window.
    InventoryFresh,
    /// Aging but still within tolerance.
    InventoryAging,
    /// Stale relative to the built artifact.
    InventoryStale,
    /// Being regenerated.
    InventoryRegenerating,
    /// No inventory is provided, so freshness does not apply.
    InventoryNotProvided,
}

impl M5InventoryFreshness {
    /// Every inventory freshness, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InventoryFresh,
        Self::InventoryAging,
        Self::InventoryStale,
        Self::InventoryRegenerating,
        Self::InventoryNotProvided,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InventoryFresh => "inventory_fresh",
            Self::InventoryAging => "inventory_aging",
            Self::InventoryStale => "inventory_stale",
            Self::InventoryRegenerating => "inventory_regenerating",
            Self::InventoryNotProvided => "inventory_not_provided",
        }
    }
}

/// Controlled inventory export availability — whether an attestation / SBOM can be
/// exported for offline inspection, so a user knows they can inspect it without
/// external tooling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InventoryExportAvailability {
    /// Exportable and usable offline / air-gapped.
    ExportAvailableOffline,
    /// Exportable only while online.
    ExportAvailableOnlineOnly,
    /// Exportable on request.
    ExportOnRequest,
    /// Not exportable.
    ExportUnavailable,
}

impl M5InventoryExportAvailability {
    /// Every export availability, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExportAvailableOffline,
        Self::ExportAvailableOnlineOnly,
        Self::ExportOnRequest,
        Self::ExportUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportAvailableOffline => "export_available_offline",
            Self::ExportAvailableOnlineOnly => "export_available_online_only",
            Self::ExportOnRequest => "export_on_request",
            Self::ExportUnavailable => "export_unavailable",
        }
    }
}

/// The derived headline trust posture of a provenance bundle — the resolver's
/// verdict about what the provenance actually proves. Trust is derived from the
/// signature and the digest lineage; the presence of an attestation or an SBOM never
/// elevates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceTrustPosture {
    /// Proven exactly: signed and verified, digest lineage intact, attestation
    /// verified.
    TrustProvenExact,
    /// Signed and verified with an intact digest lineage, but no attestation is
    /// present — trustworthy, and honest that it is not attested.
    TrustSignedNotAttested,
    /// Narrowed: a signature is present but its key is not verified, or the artifact
    /// is unsigned — inventory presence does not rescue it.
    NarrowedSignatureUnverified,
    /// Narrowed: an attestation is present but unverified or expired.
    NarrowedAttestationUnverified,
    /// Narrowed: the SBOM inventory is partial or stale.
    NarrowedInventoryIncomplete,
    /// Blocked: a signature is present but broken / does not verify.
    BlockedSignatureBroken,
    /// Blocked: the immutable-digest lineage is broken.
    BlockedDigestLineageBroken,
    /// Blocked: the provenance state is unknown / not yet evaluated.
    BlockedProvenanceUnknown,
}

impl M5ProvenanceTrustPosture {
    /// Every trust posture, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TrustProvenExact,
        Self::TrustSignedNotAttested,
        Self::NarrowedSignatureUnverified,
        Self::NarrowedAttestationUnverified,
        Self::NarrowedInventoryIncomplete,
        Self::BlockedSignatureBroken,
        Self::BlockedDigestLineageBroken,
        Self::BlockedProvenanceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustProvenExact => "trust_proven_exact",
            Self::TrustSignedNotAttested => "trust_signed_not_attested",
            Self::NarrowedSignatureUnverified => "narrowed_signature_unverified",
            Self::NarrowedAttestationUnverified => "narrowed_attestation_unverified",
            Self::NarrowedInventoryIncomplete => "narrowed_inventory_incomplete",
            Self::BlockedSignatureBroken => "blocked_signature_broken",
            Self::BlockedDigestLineageBroken => "blocked_digest_lineage_broken",
            Self::BlockedProvenanceUnknown => "blocked_provenance_unknown",
        }
    }

    /// True when the provenance is proven (possibly without an attestation).
    pub const fn is_proven(self) -> bool {
        matches!(self, Self::TrustProvenExact | Self::TrustSignedNotAttested)
    }

    /// True when the provenance is hard-blocked.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedSignatureBroken
                | Self::BlockedDigestLineageBroken
                | Self::BlockedProvenanceUnknown
        )
    }

    /// True when the provenance is narrowed below a proven claim.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::NarrowedSignatureUnverified
                | Self::NarrowedAttestationUnverified
                | Self::NarrowedInventoryIncomplete
        )
    }

    /// The specific block reason for a blocked or narrowed posture, if any. Returns
    /// `None` for a proven posture.
    pub const fn block_reason(self) -> Option<M5ProvenanceBlockReason> {
        Some(match self {
            Self::BlockedSignatureBroken => M5ProvenanceBlockReason::SignatureBroken,
            Self::BlockedDigestLineageBroken => M5ProvenanceBlockReason::DigestLineageBroken,
            Self::BlockedProvenanceUnknown => M5ProvenanceBlockReason::ProvenanceStateUnknown,
            Self::NarrowedSignatureUnverified => M5ProvenanceBlockReason::SignatureUnverified,
            Self::NarrowedAttestationUnverified => M5ProvenanceBlockReason::AttestationUnverified,
            Self::NarrowedInventoryIncomplete => M5ProvenanceBlockReason::InventoryIncomplete,
            Self::TrustProvenExact | Self::TrustSignedNotAttested => return None,
        })
    }
}

/// The exact reason a provenance bundle is narrowed or blocked, so a
/// provenance-blocked banner never reads like a generic `provenance unavailable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceBlockReason {
    /// A signature is present but broken / does not verify.
    SignatureBroken,
    /// The immutable-digest lineage is broken.
    DigestLineageBroken,
    /// The provenance state is unknown / not yet evaluated.
    ProvenanceStateUnknown,
    /// A signature key is unverified, or the artifact is unsigned.
    SignatureUnverified,
    /// An attestation is present but unverified or expired.
    AttestationUnverified,
    /// The SBOM inventory is partial or stale.
    InventoryIncomplete,
}

impl M5ProvenanceBlockReason {
    /// Every block reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SignatureBroken,
        Self::DigestLineageBroken,
        Self::ProvenanceStateUnknown,
        Self::SignatureUnverified,
        Self::AttestationUnverified,
        Self::InventoryIncomplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignatureBroken => "signature_broken",
            Self::DigestLineageBroken => "digest_lineage_broken",
            Self::ProvenanceStateUnknown => "provenance_state_unknown",
            Self::SignatureUnverified => "signature_unverified",
            Self::AttestationUnverified => "attestation_unverified",
            Self::InventoryIncomplete => "inventory_incomplete",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::SignatureBroken => "the signature is present but does not verify",
            Self::DigestLineageBroken => "the immutable-digest lineage is broken",
            Self::ProvenanceStateUnknown => "the provenance state is not yet evaluated",
            Self::SignatureUnverified => {
                "the signing key is not verified (inventory presence does not prove trust)"
            }
            Self::AttestationUnverified => "the attestation is present but not verified",
            Self::InventoryIncomplete => "the inventory is partial or stale",
        }
    }

    /// The next action a reviewer should take to clear this reason.
    pub const fn next_action(self) -> M5ProvenanceNextAction {
        match self {
            Self::SignatureBroken => M5ProvenanceNextAction::ReSignAndVerify,
            Self::DigestLineageBroken => M5ProvenanceNextAction::RebuildAndReconcileDigest,
            Self::ProvenanceStateUnknown => M5ProvenanceNextAction::RunProvenanceVerification,
            Self::SignatureUnverified => M5ProvenanceNextAction::VerifySigningKey,
            Self::AttestationUnverified => M5ProvenanceNextAction::VerifyAttestation,
            Self::InventoryIncomplete => M5ProvenanceNextAction::CompleteInventory,
        }
    }
}

/// The next action named on a provenance-blocked banner, so a narrowed or blocked
/// state is actionable from the banner itself rather than from a raw archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceNextAction {
    /// Re-sign the artifact and verify the signature.
    ReSignAndVerify,
    /// Rebuild the artifact and reconcile the digest lineage.
    RebuildAndReconcileDigest,
    /// Run provenance verification.
    RunProvenanceVerification,
    /// Verify the signing key.
    VerifySigningKey,
    /// Verify the attestation.
    VerifyAttestation,
    /// Complete the partial or stale inventory.
    CompleteInventory,
}

impl M5ProvenanceNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReSignAndVerify,
        Self::RebuildAndReconcileDigest,
        Self::RunProvenanceVerification,
        Self::VerifySigningKey,
        Self::VerifyAttestation,
        Self::CompleteInventory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReSignAndVerify => "re_sign_and_verify",
            Self::RebuildAndReconcileDigest => "rebuild_and_reconcile_digest",
            Self::RunProvenanceVerification => "run_provenance_verification",
            Self::VerifySigningKey => "verify_signing_key",
            Self::VerifyAttestation => "verify_attestation",
            Self::CompleteInventory => "complete_inventory",
        }
    }
}

/// A field the support / export packet carries so provenance and inventory truth is
/// reconstructable from the shared model. The fields in
/// [`M5ProvenanceExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceExportField {
    /// The opaque artifact identity.
    ArtifactIdentity,
    /// The immutable digest set.
    DigestSet,
    /// The signature status.
    SignatureStatus,
    /// The attestation status.
    AttestationStatus,
    /// The SBOM status.
    SbomStatus,
    /// The notice-bundle status.
    NoticeBundleStatus,
    /// The digest-lineage state.
    DigestLineageState,
    /// The inventory format.
    InventoryFormat,
    /// The inventory scope.
    InventoryScope,
    /// The inventory freshness.
    InventoryFreshness,
    /// The opaque generator version.
    GeneratorVersion,
    /// The inventory export availability.
    InventoryExport,
    /// The mirror references.
    MirrorRefs,
    /// The derived trust posture.
    TrustPosture,
    /// The block reason (when narrowed or blocked).
    BlockReason,
}

impl M5ProvenanceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::ArtifactIdentity,
        Self::DigestSet,
        Self::SignatureStatus,
        Self::AttestationStatus,
        Self::SbomStatus,
        Self::NoticeBundleStatus,
        Self::DigestLineageState,
        Self::InventoryFormat,
        Self::InventoryScope,
        Self::InventoryFreshness,
        Self::GeneratorVersion,
        Self::InventoryExport,
        Self::MirrorRefs,
        Self::TrustPosture,
        Self::BlockReason,
    ];

    /// The export fields every provenance-card export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ArtifactIdentity,
        Self::DigestSet,
        Self::SignatureStatus,
        Self::MirrorRefs,
        Self::TrustPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentity => "artifact_identity",
            Self::DigestSet => "digest_set",
            Self::SignatureStatus => "signature_status",
            Self::AttestationStatus => "attestation_status",
            Self::SbomStatus => "sbom_status",
            Self::NoticeBundleStatus => "notice_bundle_status",
            Self::DigestLineageState => "digest_lineage_state",
            Self::InventoryFormat => "inventory_format",
            Self::InventoryScope => "inventory_scope",
            Self::InventoryFreshness => "inventory_freshness",
            Self::GeneratorVersion => "generator_version",
            Self::InventoryExport => "inventory_export",
            Self::MirrorRefs => "mirror_refs",
            Self::TrustPosture => "trust_posture",
            Self::BlockReason => "block_reason",
        }
    }
}

/// One attestation / SBOM / notice status row. Its format, generator version, scope,
/// freshness, and export availability are kept explicitly separate from signature
/// verification so that the mere presence of the inventory never reads as a stronger
/// security or licensing guarantee than the component proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AttestationSbomStatusRow {
    /// Which inventory this row describes.
    pub kind: M5InventoryKind,
    /// The opaque status token for this inventory (its underlying attestation / SBOM
    /// status token).
    pub status_token: String,
    /// The inventory format (may be `not_provided_format`).
    pub format: M5InventoryFormat,
    /// The opaque generator version (empty when not provided).
    pub generator_version_repr: String,
    /// The inventory scope (may be `not_provided_scope` or `partial_scope`).
    pub scope: M5InventoryScope,
    /// The inventory freshness (may be `inventory_not_provided`).
    pub freshness: M5InventoryFreshness,
    /// The inventory export availability.
    pub export_availability: M5InventoryExportAvailability,
    /// Hard invariant: this row does not treat inventory presence as a stronger
    /// security or licensing guarantee than the signature actually proves. MUST be
    /// `true`.
    pub presence_does_not_imply_security: bool,
}

/// The derived compare/export binding for a provenance bundle, so a compare or an
/// export keeps the artifact identity, its digest, and its mirror provenance intact
/// across release, evaluation, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportBinding {
    /// True when a compare action is available on this surface.
    pub compare_available: bool,
    /// True when an export action is available on this surface.
    pub export_available: bool,
    /// True when the binding keeps the artifact digest attached (digest set is
    /// non-empty).
    pub digest_bound: bool,
    /// True when the binding preserves the mirror references (mirror-ref list is
    /// non-empty).
    pub mirror_provenance_preserved: bool,
    /// True when a compare or export keeps the artifact identity, digest, and mirror
    /// provenance intact — the acceptance-criterion binding guarantee.
    pub binding_intact: bool,
}

/// A self-contained provenance-blocked banner: the exact reason, the bound artifact,
/// its digest, its mirror refs, and the next action, so a narrowed or blocked
/// provenance state is understood from the banner alone rather than from a raw
/// archive or external tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBlockedBanner {
    /// The exact block reason.
    pub reason: M5ProvenanceBlockReason,
    /// The next action a reviewer should take.
    pub next_action: M5ProvenanceNextAction,
    /// The bound artifact identity.
    pub artifact_identity_repr: String,
    /// The primary immutable digest the banner binds to.
    pub bound_digest_repr: String,
    /// The mirror references the banner preserves.
    pub mirror_refs: Vec<String>,
    /// The trust posture the banner reports.
    pub trust_posture: M5ProvenanceTrustPosture,
    /// A deterministic, self-contained headline naming the reason, the artifact, the
    /// digest, and the next action — never a generic `provenance unavailable`.
    pub headline: String,
}

/// The full input to the provenance-bundle resolver for one artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleInput {
    /// The opaque, export-safe artifact identity.
    pub artifact_identity_repr: String,
    /// The immutable digest set. Must be non-empty so the artifact binding is
    /// explicit.
    pub digest_set: Vec<String>,
    /// The signature status.
    pub signature_status: M5SignatureStatus,
    /// The attestation status.
    pub attestation_status: M5AttestationStatus,
    /// The SBOM status.
    pub sbom_status: M5SbomStatus,
    /// The notice-bundle status (reuses the SBOM status vocabulary).
    pub notice_bundle_status: M5SbomStatus,
    /// The immutable-digest lineage state.
    pub digest_lineage_state: M5DigestLineageState,
    /// The SBOM inventory format.
    pub inventory_format: M5InventoryFormat,
    /// The inventory scope.
    pub inventory_scope: M5InventoryScope,
    /// The inventory freshness.
    pub inventory_freshness: M5InventoryFreshness,
    /// The opaque generator version (may be empty when not provided).
    pub generator_version_repr: String,
    /// The inventory export availability.
    pub inventory_export: M5InventoryExportAvailability,
    /// The mirror references for this artifact.
    pub mirror_refs: Vec<String>,
    /// Whether a compare action is available on this surface.
    pub compare_available: bool,
    /// Whether an export action is available on this surface.
    pub export_available: bool,
}

/// The resolved provenance / inventory truth for one artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedProvenanceBundle {
    /// The opaque artifact identity.
    pub artifact_identity_repr: String,
    /// The immutable digest set.
    pub digest_set: Vec<String>,
    /// The count of digests bound.
    pub digest_count: usize,
    /// The signature status.
    pub signature_status: M5SignatureStatus,
    /// The attestation status.
    pub attestation_status: M5AttestationStatus,
    /// The SBOM status.
    pub sbom_status: M5SbomStatus,
    /// The notice-bundle status.
    pub notice_bundle_status: M5SbomStatus,
    /// The immutable-digest lineage state.
    pub digest_lineage_state: M5DigestLineageState,
    /// The attestation/SBOM/notice status rows, kept separate from signature
    /// verification.
    pub status_rows: Vec<M5AttestationSbomStatusRow>,
    /// The mirror references.
    pub mirror_refs: Vec<String>,
    /// The derived compare/export binding.
    pub compare_export_binding: M5CompareExportBinding,
    /// The derived trust posture.
    pub trust_posture: M5ProvenanceTrustPosture,
    /// True when the provenance is proven.
    pub is_proven: bool,
    /// True when the provenance is hard-blocked.
    pub is_blocked: bool,
    /// True when the provenance is narrowed.
    pub is_narrowed: bool,
    /// The provenance-blocked banner, present when narrowed or blocked.
    pub provenance_banner: Option<M5ProvenanceBlockedBanner>,
}

/// Errors returned by [`resolve_provenance_bundle`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ProvenanceBundleError {
    /// The artifact identity was empty.
    EmptyArtifactIdentity,
    /// The digest set was empty (the artifact binding must be explicit).
    EmptyDigestSet,
    /// A digest was empty.
    EmptyDigest,
    /// An artifact id, digest, generator version, or mirror ref carried forbidden
    /// material.
    ForbiddenProvenanceMaterial,
}

impl M5ProvenanceBundleError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyArtifactIdentity => "empty_artifact_identity",
            Self::EmptyDigestSet => "empty_digest_set",
            Self::EmptyDigest => "empty_digest",
            Self::ForbiddenProvenanceMaterial => "forbidden_provenance_material",
        }
    }
}

impl fmt::Display for M5ProvenanceBundleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "provenance-bundle resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ProvenanceBundleError {}

/// Resolves one provenance bundle from its declared signature, attestation, SBOM,
/// notice, and digest-lineage state.
///
/// The derived trust posture is the headline verdict, computed in a fixed
/// blocking-first order: an unknown / in-progress signature, attestation, SBOM, or
/// digest reading blocks first, then a broken signature, then a broken digest
/// lineage, then an unverified or expired attestation narrows, then a partial or
/// stale SBOM narrows, then an unverified or absent signature narrows, and only a
/// signed-and-verified artifact with an intact digest lineage is proven — attested
/// exactly when its attestation verifies. Trust is never derived from the presence
/// of an attestation or an SBOM; those stay on their own status rows separate from
/// signature verification. A narrowed or blocked bundle always produces a
/// self-contained banner.
pub fn resolve_provenance_bundle(
    input: &M5ProvenanceBundleInput,
) -> Result<M5ResolvedProvenanceBundle, M5ProvenanceBundleError> {
    if input.artifact_identity_repr.trim().is_empty() {
        return Err(M5ProvenanceBundleError::EmptyArtifactIdentity);
    }
    if input.digest_set.is_empty() {
        return Err(M5ProvenanceBundleError::EmptyDigestSet);
    }
    if input.digest_set.iter().any(|d| d.trim().is_empty()) {
        return Err(M5ProvenanceBundleError::EmptyDigest);
    }
    if value_repr_is_forbidden(&input.artifact_identity_repr)
        || value_repr_is_forbidden(&input.generator_version_repr)
    {
        return Err(M5ProvenanceBundleError::ForbiddenProvenanceMaterial);
    }
    for digest in &input.digest_set {
        if value_repr_is_forbidden(digest) {
            return Err(M5ProvenanceBundleError::ForbiddenProvenanceMaterial);
        }
    }
    for mirror_ref in &input.mirror_refs {
        if value_repr_is_forbidden(mirror_ref) {
            return Err(M5ProvenanceBundleError::ForbiddenProvenanceMaterial);
        }
    }

    let trust_posture = derive_trust_posture(
        input.signature_status,
        input.attestation_status,
        input.sbom_status,
        input.digest_lineage_state,
    );

    let status_rows = build_status_rows(input);
    let compare_export_binding = derive_compare_export_binding(input);

    let is_proven = trust_posture.is_proven();
    let is_blocked = trust_posture.is_blocked();
    let is_narrowed = trust_posture.is_narrowed();

    let bound_digest_repr = input.digest_set.first().cloned().unwrap_or_default();

    let provenance_banner = trust_posture.block_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Provenance held: {} — artifact {} (digest {}, {} mirror ref(s)); posture {}, next: {}",
            reason.phrase(),
            input.artifact_identity_repr,
            bound_digest_repr,
            input.mirror_refs.len(),
            trust_posture.as_str(),
            next_action.as_str()
        );
        M5ProvenanceBlockedBanner {
            reason,
            next_action,
            artifact_identity_repr: input.artifact_identity_repr.clone(),
            bound_digest_repr: bound_digest_repr.clone(),
            mirror_refs: input.mirror_refs.clone(),
            trust_posture,
            headline,
        }
    });

    Ok(M5ResolvedProvenanceBundle {
        artifact_identity_repr: input.artifact_identity_repr.clone(),
        digest_set: input.digest_set.clone(),
        digest_count: input.digest_set.len(),
        signature_status: input.signature_status,
        attestation_status: input.attestation_status,
        sbom_status: input.sbom_status,
        notice_bundle_status: input.notice_bundle_status,
        digest_lineage_state: input.digest_lineage_state,
        status_rows,
        mirror_refs: input.mirror_refs.clone(),
        compare_export_binding,
        trust_posture,
        is_proven,
        is_blocked,
        is_narrowed,
        provenance_banner,
    })
}

/// The fixed blocking-first trust ladder. Trust is derived from the signature and
/// the digest lineage; inventory presence never elevates it.
fn derive_trust_posture(
    signature: M5SignatureStatus,
    attestation: M5AttestationStatus,
    sbom: M5SbomStatus,
    digest: M5DigestLineageState,
) -> M5ProvenanceTrustPosture {
    let unknown = matches!(signature, M5SignatureStatus::SignaturePending)
        || matches!(attestation, M5AttestationStatus::AttestationPending)
        || matches!(sbom, M5SbomStatus::SbomGenerating)
        || matches!(digest, M5DigestLineageState::DigestUnverified);
    if unknown {
        M5ProvenanceTrustPosture::BlockedProvenanceUnknown
    } else if matches!(signature, M5SignatureStatus::SignatureBroken) {
        M5ProvenanceTrustPosture::BlockedSignatureBroken
    } else if matches!(digest, M5DigestLineageState::DigestLineageBroken) {
        M5ProvenanceTrustPosture::BlockedDigestLineageBroken
    } else if matches!(
        attestation,
        M5AttestationStatus::AttestedUnverified | M5AttestationStatus::AttestationExpired
    ) {
        M5ProvenanceTrustPosture::NarrowedAttestationUnverified
    } else if matches!(sbom, M5SbomStatus::SbomPartial | M5SbomStatus::SbomStale) {
        M5ProvenanceTrustPosture::NarrowedInventoryIncomplete
    } else if !matches!(signature, M5SignatureStatus::SignedVerified) {
        // Signature is unsigned or signed-with-an-unverified-key. Inventory presence
        // never rescues this.
        M5ProvenanceTrustPosture::NarrowedSignatureUnverified
    } else if matches!(attestation, M5AttestationStatus::AttestedVerified) {
        M5ProvenanceTrustPosture::TrustProvenExact
    } else {
        // Signed and verified, digest lineage intact, but no attestation present.
        M5ProvenanceTrustPosture::TrustSignedNotAttested
    }
}

/// Builds the attestation, SBOM, and notice status rows, keeping their format,
/// generator version, scope, freshness, and export availability separate from
/// signature verification.
fn build_status_rows(input: &M5ProvenanceBundleInput) -> Vec<M5AttestationSbomStatusRow> {
    vec![
        M5AttestationSbomStatusRow {
            kind: M5InventoryKind::Attestation,
            status_token: input.attestation_status.as_str().to_owned(),
            format: attestation_format(input.attestation_status),
            generator_version_repr: input.generator_version_repr.clone(),
            scope: input.inventory_scope,
            freshness: input.inventory_freshness,
            export_availability: input.inventory_export,
            presence_does_not_imply_security: true,
        },
        M5AttestationSbomStatusRow {
            kind: M5InventoryKind::Sbom,
            status_token: input.sbom_status.as_str().to_owned(),
            format: input.inventory_format,
            generator_version_repr: input.generator_version_repr.clone(),
            scope: input.inventory_scope,
            freshness: input.inventory_freshness,
            export_availability: input.inventory_export,
            presence_does_not_imply_security: true,
        },
        M5AttestationSbomStatusRow {
            kind: M5InventoryKind::Notice,
            status_token: input.notice_bundle_status.as_str().to_owned(),
            format: notice_format(input.notice_bundle_status),
            generator_version_repr: input.generator_version_repr.clone(),
            scope: input.inventory_scope,
            freshness: input.inventory_freshness,
            export_availability: input.inventory_export,
            presence_does_not_imply_security: true,
        },
    ]
}

/// Derives the attestation row's format: in-toto when an attestation is present,
/// otherwise `not_provided_format`.
fn attestation_format(status: M5AttestationStatus) -> M5InventoryFormat {
    match status {
        M5AttestationStatus::NoAttestation => M5InventoryFormat::NotProvidedFormat,
        _ => M5InventoryFormat::InTotoAttestation,
    }
}

/// Derives the notice row's format: a notice manifest when a notice bundle is
/// present, otherwise `not_provided_format`.
fn notice_format(status: M5SbomStatus) -> M5InventoryFormat {
    match status {
        M5SbomStatus::SbomMissing => M5InventoryFormat::NotProvidedFormat,
        _ => M5InventoryFormat::NoticeManifest,
    }
}

/// Derives the compare/export binding, so a compare or an export keeps the artifact
/// identity, its digest, and its mirror provenance intact.
fn derive_compare_export_binding(input: &M5ProvenanceBundleInput) -> M5CompareExportBinding {
    let digest_bound = !input.digest_set.is_empty();
    let mirror_provenance_preserved = !input.mirror_refs.is_empty();
    let binding_intact = input.compare_available
        && input.export_available
        && digest_bound
        && mirror_provenance_preserved;
    M5CompareExportBinding {
        compare_available: input.compare_available,
        export_available: input.export_available,
        digest_bound,
        mirror_provenance_preserved,
        binding_intact,
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs provenance and inventory truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleResolutionCase {
    /// The resolver input.
    pub input: M5ProvenanceBundleInput,
    /// The resolved truth. Must equal `resolve_provenance_bundle(&input)`.
    pub resolved: M5ResolvedProvenanceBundle,
}

impl M5ProvenanceBundleResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ProvenanceBundleInput) -> Self {
        let resolved = resolve_provenance_bundle(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_provenance_bundle(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one provenance consumer bound to the shared card
/// anatomy, trust postures, signature / attestation / SBOM / digest-lineage
/// vocabulary, inventory formats, scopes, freshnesses, export availabilities, block
/// reasons, next actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleRow {
    /// Provenance consumer family.
    pub consumer_surface: M5ProvenanceBundleConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ReleaseCenterQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 publication surface families that render / consume this card.
    pub surface_families: Vec<M5PublicationSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this card renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ProvenanceBundleAnatomyPart>,
    /// Signature statuses this card distinguishes.
    pub signature_statuses: Vec<M5SignatureStatus>,
    /// Attestation statuses this card distinguishes.
    pub attestation_statuses: Vec<M5AttestationStatus>,
    /// SBOM statuses this card distinguishes.
    pub sbom_statuses: Vec<M5SbomStatus>,
    /// Digest-lineage states this card distinguishes.
    pub digest_lineage_states: Vec<M5DigestLineageState>,
    /// Inventory kinds this card's status rows cover.
    pub inventory_kinds: Vec<M5InventoryKind>,
    /// Inventory formats this card distinguishes.
    pub inventory_formats: Vec<M5InventoryFormat>,
    /// Inventory scopes this card distinguishes.
    pub inventory_scopes: Vec<M5InventoryScope>,
    /// Inventory freshnesses this card distinguishes.
    pub inventory_freshnesses: Vec<M5InventoryFreshness>,
    /// Inventory export availabilities this card distinguishes.
    pub inventory_export_availabilities: Vec<M5InventoryExportAvailability>,
    /// Trust postures this card distinguishes.
    pub trust_postures: Vec<M5ProvenanceTrustPosture>,
    /// Block reasons this card names.
    pub block_reasons: Vec<M5ProvenanceBlockReason>,
    /// Next actions this card names.
    pub next_actions: Vec<M5ProvenanceNextAction>,
    /// Export fields this card carries (must include the mandatory fields).
    pub export_fields: Vec<M5ProvenanceExportField>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ReleaseCenterAccessibilityRoute>,
    /// Release-center subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Downgrade triggers that apply to this card.
    pub downgrade_triggers: Vec<M5ReleaseCenterDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5ProvenanceBundleResolutionCase>,
    /// Hard invariant: this card never derives trust from inventory presence alone.
    /// MUST be `false`.
    pub infers_trust_from_inventory_presence: bool,
    /// Hard invariant: this card never conflates signed and unsigned provenance. MUST
    /// be `false`.
    pub conflates_signed_and_unsigned_provenance: bool,
    /// Hard invariant: this card never overstates SBOM completeness. MUST be `false`.
    pub overstates_sbom_completeness: bool,
    /// Hard invariant: this card never drops the artifact or mirror binding on a
    /// compare or export. MUST be `false`.
    pub drops_binding_on_compare_or_export: bool,
}

impl M5ProvenanceBundleRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ProvenanceBundleAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ProvenanceBundleAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ProvenanceExportField> =
            self.export_fields.iter().copied().collect();
        M5ProvenanceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.infers_trust_from_inventory_presence
            && !self.conflates_signed_and_unsigned_provenance
            && !self.overstates_sbom_completeness
            && !self.drops_binding_on_compare_or_export
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleVocabularySet {
    /// Provenance consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Inventory-kind tokens.
    pub inventory_kinds: Vec<String>,
    /// Inventory-format tokens.
    pub inventory_formats: Vec<String>,
    /// Inventory-scope tokens.
    pub inventory_scopes: Vec<String>,
    /// Inventory-freshness tokens.
    pub inventory_freshnesses: Vec<String>,
    /// Inventory-export-availability tokens.
    pub inventory_export_availabilities: Vec<String>,
    /// Trust-posture tokens.
    pub trust_postures: Vec<String>,
    /// Block-reason tokens.
    pub block_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Signature-status tokens (reused from the frozen matrix).
    pub signature_statuses: Vec<String>,
    /// Attestation-status tokens (reused from the frozen matrix).
    pub attestation_statuses: Vec<String>,
    /// SBOM-status tokens (reused from the frozen matrix).
    pub sbom_statuses: Vec<String>,
    /// Digest-lineage-state tokens (reused from the frozen matrix).
    pub digest_lineage_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ProvenanceBundleVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ProvenanceBundleConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ProvenanceBundleAnatomyPart::ALL, |v| v.as_str()),
            inventory_kinds: tokens(&M5InventoryKind::ALL, |v| v.as_str()),
            inventory_formats: tokens(&M5InventoryFormat::ALL, |v| v.as_str()),
            inventory_scopes: tokens(&M5InventoryScope::ALL, |v| v.as_str()),
            inventory_freshnesses: tokens(&M5InventoryFreshness::ALL, |v| v.as_str()),
            inventory_export_availabilities: tokens(&M5InventoryExportAvailability::ALL, |v| {
                v.as_str()
            }),
            trust_postures: tokens(&M5ProvenanceTrustPosture::ALL, |v| v.as_str()),
            block_reasons: tokens(&M5ProvenanceBlockReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ProvenanceNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ProvenanceExportField::ALL, |v| v.as_str()),
            signature_statuses: tokens(&M5SignatureStatus::ALL, |v| v.as_str()),
            attestation_statuses: tokens(&M5AttestationStatus::ALL, |v| v.as_str()),
            sbom_statuses: tokens(&M5SbomStatus::ALL, |v| v.as_str()),
            digest_lineage_states: tokens(&M5DigestLineageState::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ReleaseCenterAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleGovernanceReview {
    /// One provenance primitive carries provenance and inventory truth on every
    /// consumer.
    pub one_primitive_carries_provenance_truth: bool,
    /// Provenance and inventory state is inspectable without unpacking raw archives.
    pub inspectable_without_unpacking_archives: bool,
    /// Trust is derived from the signature and digest lineage, never from inventory
    /// presence.
    pub trust_never_derived_from_inventory_presence: bool,
    /// The attestation/SBOM status rows keep format, generator, scope, freshness, and
    /// export separate from signature verification.
    pub inventory_rows_separate_from_signature: bool,
    /// Explicit `Not provided` and `Partial` states are preserved.
    pub not_provided_and_partial_preserved: bool,
    /// Compare and export keep the artifact and mirror binding intact.
    pub compare_export_keeps_binding_intact: bool,
    /// A narrowed or blocked provenance always shows a self-contained banner.
    pub blocked_state_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never a generic message.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs provenance and inventory truth.
    pub support_export_reconstructs_provenance_truth: bool,
    /// No consumer invents a second provenance or inventory grammar.
    pub no_surface_invents_second_provenance_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel provenance / inventory vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleConsumerProjection {
    /// Release-center, enterprise-evaluation, CLI, admin, and support consumers all
    /// consume the shared primitive.
    pub provenance_surfaces_consume_shared_primitive: bool,
    /// The trust-posture resolver reads a single canonical source.
    pub trust_resolver_reads_single_source: bool,
    /// The inventory status rows read a single canonical source.
    pub inventory_rows_read_single_source: bool,
    /// The compare/export binding reads a single canonical source.
    pub compare_export_binding_reads_single_source: bool,
    /// Support / export reads a single canonical provenance-card source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the provenance primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundleReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting provenance audit.
    pub provenance_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProvenanceBundlePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProvenanceBundlePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provenance rows.
    pub provenance_rows: Vec<M5ProvenanceBundleRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProvenanceBundleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProvenanceBundleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProvenanceBundleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProvenanceBundleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProvenanceBundleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 provenance-bundle-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceBundlePrimitivePacket {
    /// Record kind; must equal [`M5_PROVENANCE_BUNDLE_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVENANCE_BUNDLE_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provenance rows.
    pub provenance_rows: Vec<M5ProvenanceBundleRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProvenanceBundleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProvenanceBundleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProvenanceBundleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProvenanceBundleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProvenanceBundleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProvenanceBundlePrimitivePacket {
    /// Builds an M5 provenance-bundle-primitive packet from stable-lane input.
    pub fn new(input: M5ProvenanceBundlePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_PROVENANCE_BUNDLE_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_PROVENANCE_BUNDLE_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            provenance_rows: input.provenance_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 provenance-bundle-primitive invariants.
    pub fn validate(&self) -> Vec<M5ProvenanceBundlePrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVENANCE_BUNDLE_PRIMITIVE_RECORD_KIND {
            violations.push(M5ProvenanceBundlePrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVENANCE_BUNDLE_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5ProvenanceBundlePrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProvenanceBundlePrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_provenance_rows(self, &mut violations);
        validate_provenance_coverage(self, &mut violations);
        validate_inventory_does_not_imply_security(self, &mut violations);
        validate_not_provided_and_partial_preserved(self, &mut violations);
        validate_compare_export_binding_intact(self, &mut violations);
        validate_blocked_banner_self_contained(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 provenance-bundle primitive packet serializes"),
        ) {
            violations.push(M5ProvenanceBundlePrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 provenance-bundle primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per provenance consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,trust_postures,signature_statuses,attestation_statuses,sbom_statuses,inventory_scopes,block_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.provenance_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.trust_postures, |v| v.as_str()),
                join_tokens(&row.signature_statuses, |v| v.as_str()),
                join_tokens(&row.attestation_statuses, |v| v.as_str()),
                join_tokens(&row.sbom_statuses, |v| v.as_str()),
                join_tokens(&row.inventory_scopes, |v| v.as_str()),
                join_tokens(&row.block_reasons, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .provenance_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Artifact-Provenance-Bundle Card and Attestation/SBOM Status-Row Primitive\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Provenance consumers: {} ({} stable)\n",
            self.provenance_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Trust postures: {}\n",
            self.vocabulary_set.trust_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Inventory scopes: {}\n",
            self.vocabulary_set.inventory_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Block reasons: {}\n",
            self.vocabulary_set.block_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Inventory formats: {}\n",
            self.vocabulary_set.inventory_formats.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Provenance consumers\n\n");
        for row in &self.provenance_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let banner = match &case.resolved.provenance_banner {
                    Some(banner) => banner.reason.as_str(),
                    None => "proven",
                };
                out.push_str(&format!(
                    "    - `{}` (digest {}) → `{}` (signature `{}`, sbom `{}`, banner `{}`)\n",
                    case.resolved.artifact_identity_repr,
                    case.resolved.digest_count,
                    case.resolved.trust_posture.as_str(),
                    case.resolved.signature_status.as_str(),
                    case.resolved.sbom_status.as_str(),
                    banner
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 provenance-bundle-primitive export.
#[derive(Debug)]
pub enum M5ProvenanceBundlePrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProvenanceBundlePrimitiveViolation>),
}

impl fmt::Display for M5ProvenanceBundlePrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 provenance-bundle primitive export parse failed: {error}"
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
                    "m5 provenance-bundle primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProvenanceBundlePrimitiveArtifactError {}

/// Validation failures emitted by [`M5ProvenanceBundlePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProvenanceBundlePrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required provenance consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A provenance row is incomplete.
    ProvenanceRowIncomplete,
    /// A provenance row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A provenance row declares no signature statuses.
    SignatureStatusMissing,
    /// A provenance row declares no trust postures.
    TrustPostureMissing,
    /// A provenance row declares no inventory formats.
    InventoryFormatMissing,
    /// A provenance row declares no inventory scopes.
    InventoryScopeMissing,
    /// A provenance row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A provenance row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A provenance row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A provenance row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A provenance row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A status row treats inventory presence as a stronger guarantee than proven.
    StatusRowImpliesSecurity,
    /// A provenance claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both a proven and a blocked provenance.
    ProvenanceCoverageUnproven,
    /// No worked resolution proves that inventory presence does not imply security.
    InventoryDoesNotImplySecurityUnproven,
    /// No worked resolution preserves an explicit `Not provided` and a `Partial`
    /// state.
    NotProvidedAndPartialPreservedUnproven,
    /// No worked resolution proves a compare/export binding kept intact.
    CompareExportBindingIntactUnproven,
    /// No worked resolution proves a blocked provenance with a self-contained banner.
    BlockedBannerSelfContainedUnproven,
    /// A provenance row violates a hard invariant.
    ProvenanceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ProvenanceBundlePrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ProvenanceRowIncomplete => "provenance_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SignatureStatusMissing => "signature_status_missing",
            Self::TrustPostureMissing => "trust_posture_missing",
            Self::InventoryFormatMissing => "inventory_format_missing",
            Self::InventoryScopeMissing => "inventory_scope_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StatusRowImpliesSecurity => "status_row_implies_security",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ProvenanceCoverageUnproven => "provenance_coverage_unproven",
            Self::InventoryDoesNotImplySecurityUnproven => {
                "inventory_does_not_imply_security_unproven"
            }
            Self::NotProvidedAndPartialPreservedUnproven => {
                "not_provided_and_partial_preserved_unproven"
            }
            Self::CompareExportBindingIntactUnproven => "compare_export_binding_intact_unproven",
            Self::BlockedBannerSelfContainedUnproven => "blocked_banner_self_contained_unproven",
            Self::ProvenanceInvariantViolated => "provenance_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 provenance-bundle-primitive export.
pub fn current_stable_m5_provenance_bundle_primitive_export(
) -> Result<M5ProvenanceBundlePrimitivePacket, M5ProvenanceBundlePrimitiveArtifactError> {
    let packet: M5ProvenanceBundlePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-artifact-provenance-bundle-card-proof/support_export.json"
    )))
    .map_err(M5ProvenanceBundlePrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProvenanceBundlePrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVENANCE_BUNDLE_SCHEMA_REF,
        M5_PROVENANCE_BUNDLE_DOC_REF,
        M5_PROVENANCE_BUNDLE_COMPONENT_MATRIX_REF,
        M5_PROVENANCE_BUNDLE_OBJECT_MODEL_REF,
        M5_PROVENANCE_BUNDLE_VERIFICATION_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProvenanceBundlePrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProvenanceBundlePrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_provenance_rows(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let present: BTreeSet<M5ProvenanceBundleConsumerSurface> = packet
        .provenance_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5ProvenanceBundleConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ProvenanceBundlePrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.provenance_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.attestation_statuses.is_empty()
            || row.sbom_statuses.is_empty()
            || row.digest_lineage_states.is_empty()
            || row.inventory_kinds.is_empty()
            || row.inventory_freshnesses.is_empty()
            || row.inventory_export_availabilities.is_empty()
            || row.block_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5ProvenanceBundlePrimitiveViolation::ProvenanceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.signature_statuses.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::SignatureStatusMissing);
        }
        if row.trust_postures.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::TrustPostureMissing);
        }
        if row.inventory_formats.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::InventoryFormatMissing);
        }
        if row.inventory_scopes.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::InventoryScopeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ProvenanceBundlePrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ProvenanceBundlePrimitiveViolation::ExampleResolutionDrift);
        }
        if row.example_resolutions.iter().any(|case| {
            case.resolved
                .status_rows
                .iter()
                .any(|status_row| !status_row.presence_does_not_imply_security)
        }) {
            violations.push(M5ProvenanceBundlePrimitiveViolation::StatusRowImpliesSecurity);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ProvenanceBundlePrimitiveViolation::ProvenanceInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a proven provenance
/// and at least one must prove a blocked provenance — the acceptance-criterion
/// example that a user can tell proven from blocked without unpacking an archive.
fn validate_provenance_coverage(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let has_proven = packet.provenance_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_proven)
    });
    let has_blocked = packet.provenance_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_blocked)
    });
    if !(has_proven && has_blocked) {
        violations.push(M5ProvenanceBundlePrimitiveViolation::ProvenanceCoverageUnproven);
    }
}

/// At least one worked resolution must prove a bundle that carries a verified
/// attestation or a complete SBOM while its signature is not verified, resolving to
/// a non-proven posture — the acceptance-criterion example that SBOM or attestation
/// presence never implies a stronger security guarantee than the component proves.
fn validate_inventory_does_not_imply_security(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let proven = packet.provenance_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            let resolved = &case.resolved;
            let inventory_present = matches!(
                resolved.attestation_status,
                M5AttestationStatus::AttestedVerified
            ) || matches!(resolved.sbom_status, M5SbomStatus::SbomComplete);
            let signature_unverified =
                !matches!(resolved.signature_status, M5SignatureStatus::SignedVerified);
            inventory_present && signature_unverified && !resolved.is_proven
        })
    });
    if !proven {
        violations
            .push(M5ProvenanceBundlePrimitiveViolation::InventoryDoesNotImplySecurityUnproven);
    }
}

/// At least one worked resolution must preserve an explicit `Not provided` state and
/// at least one must preserve a `Partial` state — the implementation requirement that
/// explicit `Not provided` and `Partial` states survive wherever evidence is missing
/// or scoped.
fn validate_not_provided_and_partial_preserved(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let cases: Vec<&M5ProvenanceBundleResolutionCase> = packet
        .provenance_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();
    let has_not_provided = cases.iter().any(|case| {
        case.resolved.status_rows.iter().any(|row| {
            row.scope.is_not_provided() || row.format == M5InventoryFormat::NotProvidedFormat
        })
    });
    let has_partial = cases.iter().any(|case| {
        case.resolved
            .status_rows
            .iter()
            .any(|row| row.scope.is_partial())
            || matches!(case.resolved.sbom_status, M5SbomStatus::SbomPartial)
    });
    if !(has_not_provided && has_partial) {
        violations
            .push(M5ProvenanceBundlePrimitiveViolation::NotProvidedAndPartialPreservedUnproven);
    }
}

/// At least one worked resolution must keep the compare/export binding intact —
/// the acceptance-criterion example that a compare or export keeps the artifact
/// identity, its digest, and its mirror provenance intact.
fn validate_compare_export_binding_intact(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let proven = packet.provenance_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            let binding = &case.resolved.compare_export_binding;
            binding.binding_intact
                && binding.digest_bound
                && binding.mirror_provenance_preserved
                && !case.resolved.mirror_refs.is_empty()
        })
    });
    if !proven {
        violations.push(M5ProvenanceBundlePrimitiveViolation::CompareExportBindingIntactUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a blocked provenance
/// whose banner carries a specific reason, a next action, the bound artifact, its
/// digest, and its mirror refs — the acceptance-criterion example that a blocked
/// state is understood from the banner rather than a raw archive.
fn validate_blocked_banner_self_contained(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let proven = packet.provenance_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_blocked
                && case
                    .resolved
                    .provenance_banner
                    .as_ref()
                    .is_some_and(|banner| {
                        !banner.headline.trim().is_empty()
                            && !banner.bound_digest_repr.trim().is_empty()
                    })
        })
    });
    if !proven {
        violations.push(M5ProvenanceBundlePrimitiveViolation::BlockedBannerSelfContainedUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_provenance_truth,
        review.inspectable_without_unpacking_archives,
        review.trust_never_derived_from_inventory_presence,
        review.inventory_rows_separate_from_signature,
        review.not_provided_and_partial_preserved,
        review.compare_export_keeps_binding_intact,
        review.blocked_state_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_provenance_truth,
        review.no_surface_invents_second_provenance_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ProvenanceBundlePrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.provenance_surfaces_consume_shared_primitive,
        projection.trust_resolver_reads_single_source,
        projection.inventory_rows_read_single_source,
        projection.compare_export_binding_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ProvenanceBundlePrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProvenanceBundlePrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProvenanceBundlePrimitivePacket,
    violations: &mut Vec<M5ProvenanceBundlePrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.provenance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProvenanceBundlePrimitiveViolation::ReleasePostureIncomplete);
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
