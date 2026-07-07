//! Shared consumers for the reusable M5 release/publication components, so
//! release-candidate cards, version-bump / publish-target rows, artifact
//! provenance bundles, and promotion / rollback timeline steps keep provenance,
//! freshness, qualification, and client-scope truth aligned across every claimed
//! M5 surface that makes a trust or compatibility claim about an Aureline build.
//!
//! Aureline's frozen release-center component matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`])
//! names the six governed release-center component families, and four sibling
//! `implement_*` / `ship_*` lanes narrow four of those families into working
//! primitives with their own canonical schema, contract doc, and support-export
//! artifact:
//!
//! * the release-candidate card / promotion-blocked banner
//!   ([`crate::implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces`]),
//! * the version-bump row / publish-target review sheet
//!   ([`crate::ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes`]),
//! * the artifact provenance bundle card / attestation-SBOM status row
//!   ([`crate::implement_artifact_provenance_bundle_cards_and_attestation_or_sbom_status_rows_across_claimed_m5_release_evaluation_support_surfaces`]),
//!   and
//! * the promotion-timeline step / rollback-revocation row
//!   ([`crate::implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories`]).
//!
//! This module is the *adoption* lane over those primitives. It proves the four
//! families are reusable components — not one release pipeline plus a few
//! admin-only pages — by binding every claimed M5 publication-component consumer
//! (the release center, the update center, the About/help surface, the docs
//! portal, the enterprise-evaluation packet, and the support export) to the same
//! canonical component schemas and the same descriptor vocabulary. Each consumer
//! points at the primitive's canonical schema and support-export artifact rather
//! than re-wording provenance, freshness, qualification, or client-scope facts in
//! local prose, and each keeps that descriptor vocabulary truthful even when it
//! renders in a narrowed-client, mirror/offline, or browser/companion-handoff
//! context outside the main release center.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_publication_binding`] — that takes one consumer's
//!    adoption of one component family, the descriptor set it surfaces, its
//!    client-scope mode, and any mirror/offline or browser/companion handoff
//!    caveats, and produces one [`M5ResolvedPublicationBinding`] carrying the
//!    derived descriptor-parity state and — whenever the binding renders under a
//!    reduced client scope — a self-contained [`M5ReducedScopeBanner`] that names
//!    the exact reason (client narrowed, mirror/offline, or browser/companion
//!    handoff), the descriptors that stay preserved, and the next action, rather
//!    than a generic "reduced" note. The resolver never lets a narrowed context
//!    drop a required descriptor and never invents a second descriptor grammar.
//! 2. A parity matrix — [`M5PublicationComponentConsumerPacket`] — that binds one
//!    row per claimed M5 publication-component consumer to the four canonical
//!    component families, the one shared descriptor vocabulary, the same
//!    client-scope modes, handoff caveats, reduced-scope reasons, next actions,
//!    export fields, and non-visual accessibility routes, so release/publication
//!    facts stop diverging between the product UI, the docs, the evaluation
//!    packet, and the support artifact.
//!
//! The publication surface families, deployment lines, release-center consumer
//! surfaces, accessibility routes, qualification classes, and downgrade triggers
//! are reused verbatim from the frozen release-center component matrix. This
//! module mints new vocabulary only for what the adoption lane itself needs: its
//! publication-component consumers, the four canonical component families and
//! their canonical refs, the shared descriptor vocabulary, the client-scope
//! modes, the handoff caveats, the descriptor-parity states, the reduced-scope
//! reasons and next actions, the consumer anatomy parts, and the export fields.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and
//! user text bodies stay outside the support boundary; every label is carried
//! only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-publication-component-consumer.schema.json`](../../../../schemas/ui/m5-publication-component-consumer.schema.json)
//! and the contract doc is
//! [`docs/release/m5_publication_component_consumer_contract.md`](../../../../docs/release/m5_publication_component_consumer_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-publication-component-consumers/`](../../../../fixtures/ui/m5-publication-component-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_publication_component_consumer_about_help_handoff_narrowed,
    seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed,
    seeded_m5_publication_component_consumer_packet, M5_PUBLICATION_COMPONENT_CONSUMER_PACKET_ID,
};

// The publication surface families, deployment lines, release-center consumer
// surfaces, accessibility routes, qualification classes, and downgrade triggers
// are frozen once, in the release-center component matrix. This adoption lane
// reuses them verbatim so it never invents a parallel release vocabulary.
pub use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    M5DeploymentLine, M5PublicationSurfaceFamily, M5ReleaseCenterAccessibilityRoute,
    M5ReleaseCenterConsumerSurface, M5ReleaseCenterDowngradeTrigger,
    M5ReleaseCenterQualificationClass,
};

// The four canonical primitive schema / doc / artifact refs this adoption lane
// points every consumer at, rather than re-wording their facts in local prose.
use crate::implement_artifact_provenance_bundle_cards_and_attestation_or_sbom_status_rows_across_claimed_m5_release_evaluation_support_surfaces::{
    M5_PROVENANCE_BUNDLE_ARTIFACT_REF, M5_PROVENANCE_BUNDLE_DOC_REF, M5_PROVENANCE_BUNDLE_SCHEMA_REF,
};
use crate::implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories::{
    M5_RELEASE_HISTORY_ARTIFACT_REF, M5_RELEASE_HISTORY_DOC_REF, M5_RELEASE_HISTORY_STEP_SCHEMA_REF,
};
use crate::implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces::{
    M5_RELEASE_CANDIDATE_ARTIFACT_REF, M5_RELEASE_CANDIDATE_DOC_REF, M5_RELEASE_CANDIDATE_SCHEMA_REF,
};
use crate::ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes::{
    M5_PUBLICATION_REVIEW_ARTIFACT_REF, M5_PUBLICATION_REVIEW_DOC_REF, M5_PUBLICATION_REVIEW_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PublicationComponentConsumerPacket`].
pub const M5_PUBLICATION_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_release_center_update_center_about_help_docs_evaluation_and_support_publication_component_consumers";

/// Schema version for M5 publication-component-consumer records.
pub const M5_PUBLICATION_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the publication-component-consumer boundary schema.
pub const M5_PUBLICATION_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-publication-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PUBLICATION_CONSUMER_DOC_REF: &str =
    "docs/release/m5_publication_component_consumer_contract.md";

/// Repo-relative path of the frozen release-center component matrix this lane
/// adopts from.
pub const M5_PUBLICATION_CONSUMER_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the release-center object-model contract this lane binds
/// against.
pub const M5_PUBLICATION_CONSUMER_OBJECT_MODEL_REF: &str =
    "docs/release/release_center_object_model_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PUBLICATION_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-publication-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PUBLICATION_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-publication-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PUBLICATION_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-publication-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PUBLICATION_CONSUMER_REPORT_REF: &str =
    "artifacts/components/m5-publication-component-consumer.md";

/// One claimed M5 publication-component consumer that adopts the shared
/// release/publication components. These are the consumers the acceptance
/// criteria name — the release center, the update center, About/help, the docs
/// portal, the enterprise-evaluation packet, and the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationComponentConsumer {
    /// The release-center / shiproom surface.
    ReleaseCenter,
    /// The update-center surface.
    UpdateCenter,
    /// The About/help surface.
    AboutHelp,
    /// The docs portal.
    DocsPortal,
    /// The enterprise-evaluation packet.
    EnterpriseEvaluation,
    /// The support export.
    SupportExport,
}

impl M5PublicationComponentConsumer {
    /// Every claimed publication-component consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenter,
        Self::UpdateCenter,
        Self::AboutHelp,
        Self::DocsPortal,
        Self::EnterpriseEvaluation,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::UpdateCenter => "update_center",
            Self::AboutHelp => "about_help",
            Self::DocsPortal => "docs_portal",
            Self::EnterpriseEvaluation => "enterprise_evaluation",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "Release Center",
            Self::UpdateCenter => "Update Center",
            Self::AboutHelp => "About / Help",
            Self::DocsPortal => "Docs Portal",
            Self::EnterpriseEvaluation => "Enterprise Evaluation",
            Self::SupportExport => "Support Export",
        }
    }

    /// True when this consumer is a docs/help surface — the surfaces the
    /// acceptance criteria single out for a canonical-schema reference so their
    /// prose can never drift from the product truth.
    pub const fn is_docs_or_help(self) -> bool {
        matches!(self, Self::AboutHelp | Self::DocsPortal)
    }
}

/// One canonical M5 release/publication component family this lane adopts. Each
/// maps to exactly one narrowed primitive's canonical schema, doc, and
/// support-export artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationComponentFamily {
    /// The release-candidate card / promotion-blocked banner primitive.
    ReleaseCandidateCard,
    /// The version-bump row / publish-target review-sheet primitive.
    VersionBumpPublishTarget,
    /// The artifact provenance bundle card / attestation-SBOM status-row primitive.
    ArtifactProvenanceBundle,
    /// The promotion-timeline step / rollback-revocation row primitive.
    PromotionRollbackHistory,
}

impl M5PublicationComponentFamily {
    /// Every canonical component family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReleaseCandidateCard,
        Self::VersionBumpPublishTarget,
        Self::ArtifactProvenanceBundle,
        Self::PromotionRollbackHistory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCandidateCard => "release_candidate_card",
            Self::VersionBumpPublishTarget => "version_bump_publish_target",
            Self::ArtifactProvenanceBundle => "artifact_provenance_bundle",
            Self::PromotionRollbackHistory => "promotion_rollback_history",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCandidateCard => "Release-Candidate Card",
            Self::VersionBumpPublishTarget => "Version-Bump / Publish-Target",
            Self::ArtifactProvenanceBundle => "Artifact Provenance Bundle",
            Self::PromotionRollbackHistory => "Promotion / Rollback History",
        }
    }

    /// The canonical boundary schema ref of the narrowed primitive that owns this
    /// family. A consumer that adopts this family must point at this schema, not
    /// at a local re-description.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::ReleaseCandidateCard => M5_RELEASE_CANDIDATE_SCHEMA_REF,
            Self::VersionBumpPublishTarget => M5_PUBLICATION_REVIEW_SCHEMA_REF,
            Self::ArtifactProvenanceBundle => M5_PROVENANCE_BUNDLE_SCHEMA_REF,
            Self::PromotionRollbackHistory => M5_RELEASE_HISTORY_STEP_SCHEMA_REF,
        }
    }

    /// The canonical contract-doc ref of the narrowed primitive that owns this
    /// family.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::ReleaseCandidateCard => M5_RELEASE_CANDIDATE_DOC_REF,
            Self::VersionBumpPublishTarget => M5_PUBLICATION_REVIEW_DOC_REF,
            Self::ArtifactProvenanceBundle => M5_PROVENANCE_BUNDLE_DOC_REF,
            Self::PromotionRollbackHistory => M5_RELEASE_HISTORY_DOC_REF,
        }
    }

    /// The canonical support-export artifact ref of the narrowed primitive that
    /// owns this family.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::ReleaseCandidateCard => M5_RELEASE_CANDIDATE_ARTIFACT_REF,
            Self::VersionBumpPublishTarget => M5_PUBLICATION_REVIEW_ARTIFACT_REF,
            Self::ArtifactProvenanceBundle => M5_PROVENANCE_BUNDLE_ARTIFACT_REF,
            Self::PromotionRollbackHistory => M5_RELEASE_HISTORY_ARTIFACT_REF,
        }
    }
}

/// The one shared descriptor vocabulary every publication component keeps aligned
/// across surfaces, so no consumer invents a new badge or stale wording. The
/// descriptors in [`M5PublicationDescriptor::REQUIRED`] must be present on every
/// binding — the track invariant that provenance, freshness, qualification, and
/// client-scope stay explicit everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationDescriptor {
    /// The provenance descriptor (signature, attestation, digest lineage).
    Provenance,
    /// The freshness descriptor (evidence / proof freshness).
    Freshness,
    /// The qualification descriptor (stable / beta / preview class).
    Qualification,
    /// The client-scope descriptor (full / narrowed / mirror / handoff scope).
    ClientScope,
}

impl M5PublicationDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Provenance,
        Self::Freshness,
        Self::Qualification,
        Self::ClientScope,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provenance => "provenance",
            Self::Freshness => "freshness",
            Self::Qualification => "qualification",
            Self::ClientScope => "client_scope",
        }
    }
}

/// The client-scope mode a consumer renders a component under. A reduced mode
/// still keeps the descriptor vocabulary — it only discloses that the rendered
/// scope is narrowed relative to the authoritative release center.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClientScopeMode {
    /// Full client scope: the authoritative release-center rendering.
    FullClientScope,
    /// A narrowed client scope (e.g. a client-limited or role-limited view).
    NarrowedClientScope,
    /// A mirror / offline snapshot scope.
    MirrorOfflineScope,
    /// A browser / companion handoff scope.
    BrowserCompanionHandoff,
}

impl M5ClientScopeMode {
    /// Every client-scope mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullClientScope,
        Self::NarrowedClientScope,
        Self::MirrorOfflineScope,
        Self::BrowserCompanionHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullClientScope => "full_client_scope",
            Self::NarrowedClientScope => "narrowed_client_scope",
            Self::MirrorOfflineScope => "mirror_offline_scope",
            Self::BrowserCompanionHandoff => "browser_companion_handoff",
        }
    }

    /// True when the mode renders below the authoritative full client scope and so
    /// must disclose a self-contained reduced-scope banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullClientScope)
    }

    /// The reduced-scope reason a narrowed mode discloses, if any.
    pub const fn reduced_scope_reason(self) -> Option<M5ReducedScopeReason> {
        Some(match self {
            Self::NarrowedClientScope => M5ReducedScopeReason::ClientNarrowed,
            Self::MirrorOfflineScope => M5ReducedScopeReason::MirrorOffline,
            Self::BrowserCompanionHandoff => M5ReducedScopeReason::BrowserCompanionHandoff,
            Self::FullClientScope => return None,
        })
    }
}

/// A mirror/offline or browser/companion handoff caveat a consumer preserves when
/// a component appears outside the main release center.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffCaveat {
    /// The mirror replica may lag the authoritative registry.
    MirrorReplicationLag,
    /// The offline snapshot is fixed at its capture time.
    OfflineSnapshot,
    /// The browser rendering is read-only and cannot mutate the release.
    BrowserReadOnly,
    /// The companion surface deep-links back to the authoritative host.
    CompanionDeepLink,
}

impl M5HandoffCaveat {
    /// Every handoff caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MirrorReplicationLag,
        Self::OfflineSnapshot,
        Self::BrowserReadOnly,
        Self::CompanionDeepLink,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorReplicationLag => "mirror_replication_lag",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::BrowserReadOnly => "browser_read_only",
            Self::CompanionDeepLink => "companion_deep_link",
        }
    }
}

/// The exact reason a binding renders under a reduced client scope, so a
/// reduced-scope banner never reads like a generic "reduced" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReducedScopeReason {
    /// The client scope is narrowed relative to the authoritative view.
    ClientNarrowed,
    /// The component renders from a mirror / offline snapshot.
    MirrorOffline,
    /// The component renders under a browser / companion handoff.
    BrowserCompanionHandoff,
}

impl M5ReducedScopeReason {
    /// Every reduced-scope reason, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ClientNarrowed,
        Self::MirrorOffline,
        Self::BrowserCompanionHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClientNarrowed => "client_narrowed",
            Self::MirrorOffline => "mirror_offline",
            Self::BrowserCompanionHandoff => "browser_companion_handoff",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ClientNarrowed => "the client scope is narrowed relative to the release center",
            Self::MirrorOffline => "this view renders from a mirror or offline snapshot",
            Self::BrowserCompanionHandoff => {
                "this view renders under a browser or companion handoff"
            }
        }
    }

    /// The next action a reader should take to reach the authoritative view.
    pub const fn next_action(self) -> M5ScopeNextAction {
        match self {
            Self::ClientNarrowed => M5ScopeNextAction::WidenClientScope,
            Self::MirrorOffline => M5ScopeNextAction::RefreshFromCanonicalSource,
            Self::BrowserCompanionHandoff => M5ScopeNextAction::OpenAuthoritativeReleaseCenter,
        }
    }
}

/// The next action named on a reduced-scope banner, so a narrowed rendering is
/// actionable from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScopeNextAction {
    /// Widen the client scope to the authoritative view.
    WidenClientScope,
    /// Refresh from the canonical (non-mirror) source.
    RefreshFromCanonicalSource,
    /// Open the authoritative release center.
    OpenAuthoritativeReleaseCenter,
}

impl M5ScopeNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::WidenClientScope,
        Self::RefreshFromCanonicalSource,
        Self::OpenAuthoritativeReleaseCenter,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenClientScope => "widen_client_scope",
            Self::RefreshFromCanonicalSource => "refresh_from_canonical_source",
            Self::OpenAuthoritativeReleaseCenter => "open_authoritative_release_center",
        }
    }
}

/// The derived descriptor-parity state of a binding — whether the shared
/// descriptor vocabulary is preserved as-is or preserved with a disclosed
/// narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DescriptorParityState {
    /// The descriptor vocabulary is preserved at full client scope.
    DescriptorsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed scope narrowing.
    DescriptorsDisclosedNarrowed,
}

impl M5DescriptorParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [
        Self::DescriptorsPreserved,
        Self::DescriptorsDisclosedNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorsPreserved => "descriptors_preserved",
            Self::DescriptorsDisclosedNarrowed => "descriptors_disclosed_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5PublicationConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The client-scope cue.
    ClientScopeCue,
    /// The handoff-caveat list.
    HandoffCaveats,
    /// The derived descriptor-parity verdict.
    DescriptorParityVerdict,
    /// The reduced-scope banner (shown when narrowed).
    ReducedScopeBanner,
}

impl M5PublicationConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClientScopeCue,
        Self::HandoffCaveats,
        Self::DescriptorParityVerdict,
        Self::ReducedScopeBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::DescriptorParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ClientScopeCue => "client_scope_cue",
            Self::HandoffCaveats => "handoff_caveats",
            Self::DescriptorParityVerdict => "descriptor_parity_verdict",
            Self::ReducedScopeBanner => "reduced_scope_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is
/// reconstructable from the shared model. The fields in
/// [`M5PublicationConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The client-scope mode.
    ClientScopeMode,
    /// The handoff caveats.
    HandoffCaveats,
    /// The descriptor-parity state.
    DescriptorParityState,
    /// The reduced-scope reason (when narrowed).
    ReducedScopeReason,
}

impl M5PublicationConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClientScopeMode,
        Self::HandoffCaveats,
        Self::DescriptorParityState,
        Self::ReducedScopeReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::DescriptorParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ClientScopeMode => "client_scope_mode",
            Self::HandoffCaveats => "handoff_caveats",
            Self::DescriptorParityState => "descriptor_parity_state",
            Self::ReducedScopeReason => "reduced_scope_reason",
        }
    }
}

/// A self-contained reduced-scope banner: the exact reason, the descriptors that
/// stay preserved, the handoff caveats, and the next action, so a narrowed
/// rendering is understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReducedScopeBanner {
    /// The exact reduced-scope reason.
    pub reason: M5ReducedScopeReason,
    /// The next action a reader should take.
    pub next_action: M5ScopeNextAction,
    /// The consumer the banner applies to.
    pub consumer: M5PublicationComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5PublicationComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5PublicationDescriptor>,
    /// The handoff caveats disclosed alongside the narrowing.
    pub handoff_caveats: Vec<M5HandoffCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the next action — never a generic "reduced" note.
    pub headline: String,
}

/// The full input to the publication-binding resolver for one consumer/family
/// adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5PublicationComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5PublicationComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required
    /// descriptor so provenance, freshness, qualification, and client-scope stay
    /// explicit.
    pub descriptor_families: Vec<M5PublicationDescriptor>,
    /// The client-scope mode the binding renders under.
    pub client_scope_mode: M5ClientScopeMode,
    /// The mirror/offline or browser/companion handoff caveats disclosed.
    pub handoff_caveats: Vec<M5HandoffCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved descriptor-parity / reduced-scope truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPublicationBinding {
    /// The consumer.
    pub consumer: M5PublicationComponentConsumer,
    /// The component family.
    pub component_family: M5PublicationComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5PublicationDescriptor>,
    /// The client-scope mode.
    pub client_scope_mode: M5ClientScopeMode,
    /// The handoff caveats.
    pub handoff_caveats: Vec<M5HandoffCaveat>,
    /// The derived descriptor-parity state.
    pub descriptor_parity_state: M5DescriptorParityState,
    /// True when the binding renders under a reduced client scope.
    pub is_narrowed: bool,
    /// The reduced-scope banner, present when narrowed.
    pub reduced_scope_banner: Option<M5ReducedScopeBanner>,
}

/// Errors returned by [`resolve_publication_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PublicationBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5PublicationBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5PublicationBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "publication binding error: {}", self.as_str())
    }
}

impl Error for M5PublicationBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the track invariant that
/// provenance, freshness, qualification, and client-scope stay explicit on every
/// surface. The descriptor-parity state is preserved at full client scope and
/// disclosed-narrowed under any reduced scope, and a reduced scope always
/// produces a self-contained banner naming the exact reason and next action while
/// keeping the descriptor vocabulary intact.
pub fn resolve_publication_binding(
    input: &M5PublicationBindingInput,
) -> Result<M5ResolvedPublicationBinding, M5PublicationBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5PublicationBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5PublicationDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5PublicationDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5PublicationBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5PublicationBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.handoff_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future
        // free-text extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5PublicationBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.client_scope_mode.is_narrowed();
    let descriptor_parity_state = if is_narrowed {
        M5DescriptorParityState::DescriptorsDisclosedNarrowed
    } else {
        M5DescriptorParityState::DescriptorsPreserved
    };

    let reduced_scope_banner = input
        .client_scope_mode
        .reduced_scope_reason()
        .map(|reason| {
            let next_action = reason.next_action();
            let headline = format!(
                "Scope reduced: {} — {} renders {} with {} descriptor(s) preserved; next: {}",
                reason.phrase(),
                input.consumer.as_str(),
                input.component_family.as_str(),
                input.descriptor_families.len(),
                next_action.as_str()
            );
            M5ReducedScopeBanner {
                reason,
                next_action,
                consumer: input.consumer,
                component_family: input.component_family,
                preserved_descriptors: input.descriptor_families.clone(),
                handoff_caveats: input.handoff_caveats.clone(),
                headline,
            }
        });

    Ok(M5ResolvedPublicationBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: input.component_family.canonical_schema_ref().to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        client_scope_mode: input.client_scope_mode,
        handoff_caveats: input.handoff_caveats.clone(),
        descriptor_parity_state,
        is_narrowed,
        reduced_scope_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationBindingCase {
    /// The resolver input.
    pub input: M5PublicationBindingInput,
    /// The resolved truth. Must equal `resolve_publication_binding(&input)`.
    pub resolved: M5ResolvedPublicationBinding,
}

impl M5PublicationBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5PublicationBindingInput) -> Self {
        let resolved = resolve_publication_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_publication_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs
/// the consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5PublicationComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's
    /// canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must
    /// equal the family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5PublicationBindingCase>,
}

impl M5PublicationComponentBinding {
    /// True when the binding points at the family's canonical refs and references
    /// the canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == self.component_family.canonical_schema_ref()
            && self.canonical_artifact_ref == self.component_family.canonical_artifact_ref()
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one publication-component consumer bound to the
/// four canonical component families, the shared descriptor vocabulary, the
/// client-scope modes, handoff caveats, parity states, reduced-scope reasons, next
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationConsumerRow {
    /// Publication-component consumer.
    pub consumer: M5PublicationComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5ReleaseCenterQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 publication surface families that render / consume this projection.
    pub surface_families: Vec<M5PublicationSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5PublicationConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5PublicationDescriptor>,
    /// Client-scope modes this consumer distinguishes.
    pub client_scope_modes: Vec<M5ClientScopeMode>,
    /// Handoff caveats this consumer preserves.
    pub handoff_caveats: Vec<M5HandoffCaveat>,
    /// Descriptor-parity states this consumer distinguishes.
    pub descriptor_parity_states: Vec<M5DescriptorParityState>,
    /// Reduced-scope reasons this consumer names.
    pub reduced_scope_reasons: Vec<M5ReducedScopeReason>,
    /// Scope next actions this consumer names.
    pub scope_next_actions: Vec<M5ScopeNextAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5PublicationConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ReleaseCenterAccessibilityRoute>,
    /// Release-center subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ReleaseCenterDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5PublicationComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the descriptors per surface.
    /// MUST be `false`.
    pub rewords_descriptors_per_surface: bool,
    /// Hard invariant: this consumer never invents a new badge vocabulary. MUST be
    /// `false`.
    pub invents_new_badge_vocabulary: bool,
    /// Hard invariant: this consumer never drops provenance or freshness when
    /// narrowed. MUST be `false`.
    pub drops_provenance_or_freshness_when_narrowed: bool,
    /// Hard invariant: this consumer never hides a mirror/offline or handoff
    /// caveat. MUST be `false`.
    pub hides_mirror_or_offline_handoff_caveat: bool,
}

impl M5PublicationConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5PublicationConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5PublicationConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PublicationConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5PublicationConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5PublicationDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5PublicationDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5PublicationComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5PublicationComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_descriptors_per_surface
            && !self.invents_new_badge_vocabulary
            && !self.drops_provenance_or_freshness_when_narrowed
            && !self.hides_mirror_or_offline_handoff_caveat
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationConsumerVocabularySet {
    /// Publication-component-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Client-scope-mode tokens.
    pub client_scope_modes: Vec<String>,
    /// Handoff-caveat tokens.
    pub handoff_caveats: Vec<String>,
    /// Reduced-scope-reason tokens.
    pub reduced_scope_reasons: Vec<String>,
    /// Scope-next-action tokens.
    pub scope_next_actions: Vec<String>,
    /// Descriptor-parity-state tokens.
    pub descriptor_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5PublicationConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5PublicationComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5PublicationComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5PublicationDescriptor::ALL, |v| v.as_str()),
            client_scope_modes: tokens(&M5ClientScopeMode::ALL, |v| v.as_str()),
            handoff_caveats: tokens(&M5HandoffCaveat::ALL, |v| v.as_str()),
            reduced_scope_reasons: tokens(&M5ReducedScopeReason::ALL, |v| v.as_str()),
            scope_next_actions: tokens(&M5ScopeNextAction::ALL, |v| v.as_str()),
            descriptor_parity_states: tokens(&M5DescriptorParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5PublicationConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PublicationConsumerExportField::ALL, |v| v.as_str()),
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
pub struct M5PublicationConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new badge vocabulary.
    pub no_consumer_invents_new_badge: bool,
    /// Provenance, freshness, qualification, and client-scope stay explicit
    /// everywhere.
    pub descriptors_explicit_on_every_surface: bool,
    /// Mirror/offline and handoff caveats are preserved outside the release center.
    pub mirror_offline_and_handoff_caveats_preserved: bool,
    /// A narrowed rendering always shows a self-contained reduced-scope banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never a generic note.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs consumer parity.
    pub support_export_reconstructs_consumer_parity: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationConsumerProjection {
    /// Release-center, update-center, About/help, docs, evaluation, and support
    /// consumers all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The provenance descriptor reads a single canonical source.
    pub provenance_reads_single_source: bool,
    /// The freshness descriptor reads a single canonical source.
    pub freshness_reads_single_source: bool,
    /// The qualification descriptor reads a single canonical source.
    pub qualification_reads_single_source: bool,
    /// The client-scope descriptor reads a single canonical source.
    pub client_scope_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting consumer audit.
    pub consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PublicationComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PublicationComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5PublicationConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PublicationConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PublicationConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PublicationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PublicationConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PublicationConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 publication-component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationComponentConsumerPacket {
    /// Record kind; must equal [`M5_PUBLICATION_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PUBLICATION_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5PublicationConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PublicationConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PublicationConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PublicationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PublicationConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PublicationConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PublicationComponentConsumerPacket {
    /// Builds an M5 publication-component-consumer packet from stable-lane input.
    pub fn new(input: M5PublicationComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_PUBLICATION_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_PUBLICATION_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 publication-component-consumer invariants.
    pub fn validate(&self) -> Vec<M5PublicationConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PUBLICATION_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5PublicationConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PUBLICATION_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5PublicationConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PublicationConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_docs_help_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 publication-component consumer packet serializes"),
        ) {
            violations.push(M5PublicationConsumerViolation::RawMaterialInExport);
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
            .expect("m5 publication-component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,client_scope_modes,descriptor_parity_states,reduced_scope_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.client_scope_modes, |v| v.as_str()),
                join_tokens(&row.descriptor_parity_states, |v| v.as_str()),
                join_tokens(&row.reduced_scope_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Publication-Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Publication-component consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Client-scope modes: {}\n",
            self.vocabulary_set.client_scope_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Publication-component consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.reduced_scope_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.client_scope_mode.as_str(),
                        case.resolved.descriptor_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 publication-component-consumer export.
#[derive(Debug)]
pub enum M5PublicationConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PublicationConsumerViolation>),
}

impl fmt::Display for M5PublicationConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 publication-component consumer export parse failed: {error}"
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
                    "m5 publication-component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PublicationConsumerArtifactError {}

/// Validation failures emitted by [`M5PublicationComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PublicationConsumerViolation {
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
    /// A required publication-component consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one
    /// consumer (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-scope rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// A docs/help consumer does not reference the canonical component schema.
    DocsHelpReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5PublicationConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::DocsHelpReferenceMissing => "docs_help_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 publication-component-consumer export.
pub fn current_stable_m5_publication_component_consumer_export(
) -> Result<M5PublicationComponentConsumerPacket, M5PublicationConsumerArtifactError> {
    let packet: M5PublicationComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-publication-component-consumer-proof/support_export.json"
    )))
    .map_err(M5PublicationConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PublicationConsumerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PUBLICATION_CONSUMER_SCHEMA_REF,
        M5_PUBLICATION_CONSUMER_DOC_REF,
        M5_PUBLICATION_CONSUMER_COMPONENT_MATRIX_REF,
        M5_RELEASE_CANDIDATE_SCHEMA_REF,
        M5_PUBLICATION_REVIEW_SCHEMA_REF,
        M5_PROVENANCE_BUNDLE_SCHEMA_REF,
        M5_RELEASE_HISTORY_STEP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PublicationConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PublicationConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let present: BTreeSet<M5PublicationComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5PublicationComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5PublicationConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.client_scope_modes.is_empty()
            || row.handoff_caveats.is_empty()
            || row.descriptor_parity_states.is_empty()
            || row.reduced_scope_reasons.is_empty()
            || row.scope_next_actions.is_empty()
        {
            violations.push(M5PublicationConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5PublicationConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5PublicationConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5PublicationConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5PublicationConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PublicationConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PublicationConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5PublicationConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5PublicationConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5PublicationConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5PublicationConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PublicationConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5PublicationConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct
/// consumers — the acceptance-criterion proof that the families are reusable
/// components rather than one release pipeline plus a few admin-only pages.
fn validate_family_reuse(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    for family in M5PublicationComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5PublicationConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering
/// whose banner carries a specific reason, a next action, and a non-empty set of
/// preserved descriptors — the acceptance-criterion example that publication
/// components stay truthful in mirrored, offline, or narrowed-client contexts.
fn validate_narrowing_disclosure(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .reduced_scope_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5PublicationConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-scope rendering
/// with preserved parity and no banner — the acceptance-criterion example that
/// full-scope consumers keep the descriptor vocabulary without a spurious
/// narrowing note.
fn validate_scope_preserved(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.reduced_scope_banner.is_none()
            && case.resolved.descriptor_parity_state
                == M5DescriptorParityState::DescriptorsPreserved
    });
    if !proven {
        violations.push(M5PublicationConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every docs/help consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that docs/help prose can never
/// drift from the product truth.
fn validate_docs_help_reference(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_docs_or_help() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5PublicationComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5PublicationConsumerViolation::DocsHelpReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_badge,
        review.descriptors_explicit_on_every_surface,
        review.mirror_offline_and_handoff_caveats_preserved,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_consumer_parity,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5PublicationConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.provenance_reads_single_source,
        projection.freshness_reads_single_source,
        projection.qualification_reads_single_source,
        projection.client_scope_reads_single_source,
    ] {
        if !ok {
            violations.push(M5PublicationConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PublicationConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PublicationComponentConsumerPacket,
    violations: &mut Vec<M5PublicationConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.consumer_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PublicationConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5PublicationComponentConsumerPacket,
) -> impl Iterator<Item = &M5PublicationBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
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
