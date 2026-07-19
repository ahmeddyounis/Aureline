//! Implements the reusable bundle class-disclosure primitive: a class-disclosure card that names a
//! bundle's disclosure class (native first-party, imported-user handoff, org-approved / managed,
//! design-partner / certified, community, or local draft) alongside its policy owner, mirror
//! source, entitlement dependency, and confidence / posture labels, and a claim-narrowing row that
//! narrows the compatibility / support claim so an imported or org-approved bundle never inherits
//! full native-parity language when it is capability-mapped or policy-bound — both resolving from
//! one disclosure context and sharing one disclosure identity, so start-center, bundle-detail,
//! migration, docs / help, diagnostics, and support surfaces explain the *same class truth*: what
//! kind of bundle this is, why it is recommended, and how strong its compatibility / support claim
//! actually is, before a user trusts its promises.
//!
//! Where
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]
//! *freezes* the reusable workflow-bundle component families as a governed contract, this module
//! *narrows* the two class-truth families —
//! [`M5WorkflowBundleComponentFamily::BundleClassDisclosureCard`] and
//! [`M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow`]
//! ([`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily`])
//! — into a dedicated **class-disclosure** primitive with a real **resolver**. A single disclosure
//! context projects onto a class-disclosure card and a claim-narrowing row that share one disclosure
//! identity, so a bundle's class, dependency posture, and narrowed claim are stated with one shared
//! vocabulary rather than re-coined per surface.
//!
//! The resolver reuses the canonical governance / scorecard / manifest vocabulary already carried by
//! [`crate::m5_entry_and_bundle_governance`] ([`BundleClass`], [`SourceTrust`]),
//! [`crate::m5_workflow_bundle_manifests`] ([`CertificationTarget`], [`LifecycleStage`]), and
//! [`crate::m5_bundle_scorecards`] ([`BundleScorecardClass`], [`EvidenceFreshness`],
//! [`ImportedVsNativeConfidence`]) — never a bespoke per-flow class model. It adds only the
//! disclosure-specific vocabulary the resolver needs: the disclosure class
//! ([`M5BundleDisclosureClass`]) and the one shared capability-confidence vocabulary
//! ([`M5CapabilityConfidence`], carrying `native`, `exact`, `capability_mapped`, `approximate`, and
//! `unsupported_gap`).
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — users can tell why a bundle is recommended and how strong its claim actually is.**
//!   Every disclosure carries a concrete recommendation reason and a support-claim strength honestly
//!   capped by imported-versus-native confidence and certification freshness; a card with no
//!   recommendation reason is rejected.
//! - **AC2 — imported-user and org-approved bundles no longer inherit full native-parity language
//!   when they are capability-mapped or policy-bound.** A bundle only inherits native-parity
//!   language when its disclosure class is native first-party, its capability confidence is native,
//!   and it is not policy-bound; any other bundle that claims full native parity is rejected.
//! - **AC3 — class disclosure remains stable across UI, docs / help, diagnostics, and support
//!   packets.** One primitive projects the class card and claim-narrowing row across every surface
//!   with one shared vocabulary; the support / export packet reconstructs the same class truth
//!   offline, and a bundle that depends on managed registries, org identity, mirror freshness, or
//!   policy-controlled availability never implies standalone local completeness.
//!
//! Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors never
//! cross this boundary; the resolver carries only opaque refs, typed class tokens, booleans, and
//! redacted labels, so support and diagnostics exports reconstruct exactly what a surface would have
//! shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-bundle-class-disclosure-primitive.schema.json`](../../../../schemas/ui/m5-bundle-class-disclosure-primitive.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_bundle_class_disclosure_primitive.md`](../../../../docs/bundles/m5_bundle_class_disclosure_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the primitive binds to the freeze matrix's truth-mode,
// downgrade-trigger, and degraded-state tokens rather than mint parallel ones.
use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    DegradedState, M5BundleComponentDowngradeTrigger, M5BundleTruthMode,
};
// Reused canonical bundle / scorecard / governance vocabulary already carried by the frozen
// bundle-manifest, scorecard, and entry-governance contracts.
use crate::m5_bundle_scorecards::{
    BundleScorecardClass, EvidenceFreshness, ImportedVsNativeConfidence,
};
use crate::m5_entry_and_bundle_governance::{BundleClass, SourceTrust};
use crate::m5_workflow_bundle_manifests::{CertificationTarget, LifecycleStage};

/// Stable record-kind tag carried by [`M5BundleClassDisclosurePacket`].
pub const M5_BUNDLE_CLASS_DISCLOSURE_RECORD_KIND: &str = "m5_bundle_class_disclosure_primitive";

/// Schema version for the bundle class-disclosure primitive packet.
pub const M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_REF: &str =
    "schemas/ui/m5-bundle-class-disclosure-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUNDLE_CLASS_DISCLOSURE_DOC_REF: &str =
    "docs/bundles/m5_bundle_class_disclosure_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_BUNDLE_CLASS_DISCLOSURE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUNDLE_CLASS_DISCLOSURE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-bundle-class-disclosure-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const M5_BUNDLE_CLASS_DISCLOSURE_ARTIFACT_REF: &str =
    "artifacts/release/m5-bundle-class-disclosure-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_BUNDLE_CLASS_DISCLOSURE_CSV_REF: &str =
    "artifacts/release/m5-bundle-class-disclosure-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BUNDLE_CLASS_DISCLOSURE_REPORT_REF: &str =
    "artifacts/release/m5-bundle-class-disclosure-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed class-disclosure surface family. Each family is one parity surface that ingests the shared
/// primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDisclosureSurfaceFamily {
    /// The start-center class card shown when a bundle is offered for a guided stack entry.
    StartCenterClassCard,
    /// The bundle detail class panel disclosing one bundle's class in full.
    BundleDetailClassPanel,
    /// The migration class-disclosure row shown when an imported bundle is reviewed.
    MigrationClassDisclosureRow,
    /// The docs / help class block that explains a bundle class in the guide.
    DocsHelpClassBlock,
    /// The diagnostics class report used for triage / support handoff.
    DiagnosticsClassReport,
    /// The support / export replay surface reconstructing class truth offline.
    SupportExportReplay,
}

impl M5BundleDisclosureSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StartCenterClassCard,
        Self::BundleDetailClassPanel,
        Self::MigrationClassDisclosureRow,
        Self::DocsHelpClassBlock,
        Self::DiagnosticsClassReport,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterClassCard => "start_center_class_card",
            Self::BundleDetailClassPanel => "bundle_detail_class_panel",
            Self::MigrationClassDisclosureRow => "migration_class_disclosure_row",
            Self::DocsHelpClassBlock => "docs_help_class_block",
            Self::DiagnosticsClassReport => "diagnostics_class_report",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StartCenterClassCard => "Start-center class card",
            Self::BundleDetailClassPanel => "Bundle detail class panel",
            Self::MigrationClassDisclosureRow => "Migration class-disclosure row",
            Self::DocsHelpClassBlock => "Docs / help class block",
            Self::DiagnosticsClassReport => "Diagnostics class report",
            Self::SupportExportReplay => "Support / export replay",
        }
    }

    /// Whether this surface is a docs / help surface (used to prove AC3 doc parity).
    pub const fn is_docs_help(self) -> bool {
        matches!(self, Self::DocsHelpClassBlock)
    }

    /// Whether this surface is a support / export surface (used to prove AC3 support parity).
    pub const fn is_support_export(self) -> bool {
        matches!(self, Self::SupportExportReplay)
    }
}

/// The disclosure class of a bundle: what *kind* of bundle it is and how it is governed. Every
/// disclosure names exactly one class so a user can tell whether a bundle is native, imported,
/// org-approved, certified, community, or a local draft before trusting its promises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDisclosureClass {
    /// A native, first-party bundle built into Aureline.
    NativeFirstParty,
    /// A bundle imported from another setup as a user handoff.
    ImportedUserHandoff,
    /// An organization-approved, managed bundle.
    ManagedApproved,
    /// A design-partner / certified bundle.
    DesignPartnerCertified,
    /// A community bundle.
    Community,
    /// A local draft with no external claim.
    LocalDraft,
}

impl M5BundleDisclosureClass {
    /// Every disclosure class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NativeFirstParty,
        Self::ImportedUserHandoff,
        Self::ManagedApproved,
        Self::DesignPartnerCertified,
        Self::Community,
        Self::LocalDraft,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeFirstParty => "native_first_party",
            Self::ImportedUserHandoff => "imported_user_handoff",
            Self::ManagedApproved => "managed_approved",
            Self::DesignPartnerCertified => "design_partner_certified",
            Self::Community => "community",
            Self::LocalDraft => "local_draft",
        }
    }

    /// Human-readable label for the card and Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NativeFirstParty => "Native (first-party)",
            Self::ImportedUserHandoff => "Imported (user handoff)",
            Self::ManagedApproved => "Managed (org-approved)",
            Self::DesignPartnerCertified => "Design-partner (certified)",
            Self::Community => "Community",
            Self::LocalDraft => "Local draft",
        }
    }

    /// Whether this disclosure class is honest for the bundle's certification target: the class the
    /// card names must be consistent with the certification target the manifest claims, so a
    /// community or imported bundle can never present with a certified or managed class.
    pub const fn permits_source_class(self, source: CertificationTarget) -> bool {
        match self {
            // A native first-party bundle and a design-partner certified bundle both back a
            // certified target; the class distinguishes first-party from design-partner provenance.
            Self::NativeFirstParty | Self::DesignPartnerCertified => {
                matches!(source, CertificationTarget::Certified)
            }
            Self::ManagedApproved => matches!(source, CertificationTarget::ManagedApproved),
            Self::Community => matches!(source, CertificationTarget::CommunityReviewed),
            Self::ImportedUserHandoff => {
                matches!(source, CertificationTarget::ImportedPendingReview)
            }
            Self::LocalDraft => matches!(source, CertificationTarget::LocalDraft),
        }
    }

    /// Whether this disclosure class may back full native-parity language. Only a native first-party
    /// bundle can; every other class (imported, managed, certified design-partner, community, draft)
    /// must narrow rather than inherit native parity.
    pub const fn is_native_parity_class(self) -> bool {
        matches!(self, Self::NativeFirstParty)
    }
}

/// The one shared capability-confidence vocabulary spanning migration, start center, docs / help,
/// and exports: how faithfully a bundle's behavior maps to native Aureline behavior. Every surface
/// names the compatibility claim with this closed set — `native`, `exact`, `capability_mapped`,
/// `approximate`, or `unsupported_gap` — rather than coining a private strength word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityConfidence {
    /// Native behavior, mapped exactly, full parity.
    Native,
    /// Imported behavior mapped exactly onto a native capability, one-to-one.
    Exact,
    /// Behavior mapped through a capability bridge; close but not native.
    CapabilityMapped,
    /// Approximate behavior through a shim.
    Approximate,
    /// No verified mapping — an unsupported gap.
    UnsupportedGap,
}

impl M5CapabilityConfidence {
    /// Every capability-confidence level, in declaration order (strongest to weakest).
    pub const ALL: [Self; 5] = [
        Self::Native,
        Self::Exact,
        Self::CapabilityMapped,
        Self::Approximate,
        Self::UnsupportedGap,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Exact => "exact",
            Self::CapabilityMapped => "capability_mapped",
            Self::Approximate => "approximate",
            Self::UnsupportedGap => "unsupported_gap",
        }
    }

    /// Human-readable label for the card and Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Native => "native parity",
            Self::Exact => "exact mapping",
            Self::CapabilityMapped => "capability-mapped",
            Self::Approximate => "approximate",
            Self::UnsupportedGap => "unsupported gap",
        }
    }

    /// Strength rank: higher is a stronger compatibility claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Native => 4,
            Self::Exact => 3,
            Self::CapabilityMapped => 2,
            Self::Approximate => 1,
            Self::UnsupportedGap => 0,
        }
    }

    /// The strongest capability confidence an imported-versus-native confidence can honestly back.
    /// Anything stronger over-claims the mapping.
    pub const fn strongest_for_imported(imported: ImportedVsNativeConfidence) -> Self {
        match imported {
            ImportedVsNativeConfidence::Native => Self::Native,
            ImportedVsNativeConfidence::Bridged => Self::CapabilityMapped,
            ImportedVsNativeConfidence::Approximated => Self::Approximate,
            ImportedVsNativeConfidence::Unverified => Self::UnsupportedGap,
        }
    }

    /// Whether this capability confidence is honest for the imported-versus-native confidence: it
    /// must be no stronger than what the mapping can back.
    pub const fn is_honest_for_imported(self, imported: ImportedVsNativeConfidence) -> bool {
        self.rank() <= Self::strongest_for_imported(imported).rank()
    }

    /// Whether this capability confidence backs full native parity. Only `native` does.
    pub const fn inherits_native_parity(self) -> bool {
        matches!(self, Self::Native)
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must carry per
/// surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDisclosureExportField {
    /// The stable disclosure identity shared across surfaces.
    DisclosureId,
    /// The opaque bundle identity ref and human name.
    BundleIdentity,
    /// The disclosure class of the bundle.
    DisclosureClass,
    /// The capability confidence the compatibility claim carries.
    CapabilityConfidence,
    /// The dependency disclosure (policy owner, mirror source, entitlement dependency).
    DependencyDisclosure,
    /// The concrete reason the bundle is recommended.
    RecommendationReason,
    /// The honestly-capped support-claim strength.
    SupportClaimStrength,
    /// The mirror / offline posture of the source.
    MirrorOfflinePosture,
}

impl M5BundleDisclosureExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DisclosureId,
        Self::BundleIdentity,
        Self::DisclosureClass,
        Self::CapabilityConfidence,
        Self::DependencyDisclosure,
        Self::RecommendationReason,
        Self::SupportClaimStrength,
        Self::MirrorOfflinePosture,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::DisclosureId,
        Self::BundleIdentity,
        Self::DisclosureClass,
        Self::CapabilityConfidence,
        Self::DependencyDisclosure,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisclosureId => "disclosure_id",
            Self::BundleIdentity => "bundle_identity",
            Self::DisclosureClass => "disclosure_class",
            Self::CapabilityConfidence => "capability_confidence",
            Self::DependencyDisclosure => "dependency_disclosure",
            Self::RecommendationReason => "recommendation_reason",
            Self::SupportClaimStrength => "support_claim_strength",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
        }
    }
}

/// The dependency disclosure carried by a class card: whether a bundle depends on managed
/// registries, org identity, mirror freshness, or policy-controlled availability, and the
/// redacted labels naming the policy owner, mirror source, and entitlement dependency. A declared
/// dependency must carry its label so a card never implies standalone local completeness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDependencyDisclosure {
    /// The bundle depends on a managed registry to resolve or update.
    pub depends_on_managed_registry: bool,
    /// The bundle depends on org identity / entitlement to be available.
    pub depends_on_org_identity: bool,
    /// The bundle's freshness is bounded by a mirror.
    pub depends_on_mirror_freshness: bool,
    /// The bundle's availability is controlled by policy.
    pub depends_on_policy_availability: bool,
    /// The redacted policy owner label; required when availability is policy-controlled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_owner: Option<String>,
    /// The redacted mirror source label; required when freshness is mirror-bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_source: Option<String>,
    /// The redacted entitlement-dependency label; required when a managed registry or org identity
    /// is depended on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_dependency: Option<String>,
}

impl M5BundleDependencyDisclosure {
    /// A dependency disclosure with no dependencies (a fully standalone local bundle).
    pub fn none() -> Self {
        Self {
            depends_on_managed_registry: false,
            depends_on_org_identity: false,
            depends_on_mirror_freshness: false,
            depends_on_policy_availability: false,
            policy_owner: None,
            mirror_source: None,
            entitlement_dependency: None,
        }
    }

    /// Whether the bundle depends on anything beyond standalone local completeness.
    pub const fn has_any_dependency(&self) -> bool {
        self.depends_on_managed_registry
            || self.depends_on_org_identity
            || self.depends_on_mirror_freshness
            || self.depends_on_policy_availability
    }

    /// Whether the bundle's availability is bound to org identity or policy control.
    pub const fn is_policy_or_org_bound(&self) -> bool {
        self.depends_on_org_identity || self.depends_on_policy_availability
    }

    /// Whether every declared dependency carries its required, non-empty label.
    pub fn is_consistent(&self) -> bool {
        if self.depends_on_policy_availability && !label_present(&self.policy_owner) {
            return false;
        }
        if self.depends_on_mirror_freshness && !label_present(&self.mirror_source) {
            return false;
        }
        if (self.depends_on_managed_registry || self.depends_on_org_identity)
            && !label_present(&self.entitlement_dependency)
        {
            return false;
        }
        true
    }
}

/// Whether an optional label is present and non-empty.
fn label_present(label: &Option<String>) -> bool {
    label.as_ref().is_some_and(|value| !value.trim().is_empty())
}

// --- resolver input ---

/// The full input to the bundle class-disclosure resolver for one disclosure context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleClassDisclosureInput {
    /// The stable disclosure identity that must survive across the card and claim-narrowing row.
    pub disclosure_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// Opaque ref to the bundle id being disclosed; never raw manifest bytes.
    pub bundle_id_ref: String,
    /// Human-readable bundle name shown on the card.
    pub bundle_name: String,
    /// The disclosure class the card names; must be honest for the certification target.
    pub disclosure_class: M5BundleDisclosureClass,
    /// The bundle class under disclosure.
    pub bundle_class: BundleClass,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Support-class / lifecycle stage of the bundle.
    pub support_class: LifecycleStage,
    /// The certification target the manifest claims.
    pub source_class: CertificationTarget,
    /// The scorecard class the bundle carries (its headline assurance claim).
    pub scorecard_class: BundleScorecardClass,
    /// Certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// Imported-vs-native confidence contributing to the portability story.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// The disclosed capability confidence; must be honest for `imported_confidence`.
    pub capability_confidence: M5CapabilityConfidence,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class the disclosure binds to.
    pub truth_mode: M5BundleTruthMode,
    /// The dependency disclosure (managed registry / org identity / mirror / policy).
    pub dependencies: M5BundleDependencyDisclosure,
    /// The concrete reason the bundle is recommended (must be non-empty, AC1).
    pub reason_for_recommendation: String,
    /// The card claims full native parity; only legal for a native first-party, native-confidence,
    /// non-policy-bound bundle (AC2). Must otherwise be `false`.
    pub claims_full_native_parity: bool,
    /// The card implies standalone local completeness; must be `false` when the bundle depends on a
    /// managed registry, org identity, mirror freshness, or policy availability.
    pub implies_standalone_local_completeness: bool,
    /// A stale / missing certification is claimed as current; must be `false`.
    pub claims_current_despite_stale: bool,
    /// An externally-observed narrowing carried through onto the disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

impl M5BundleClassDisclosureInput {
    /// Whether the bundle's availability is bound to org identity, policy control, or a
    /// policy-gated / mirror-only lifecycle stage.
    pub fn is_policy_bound(&self) -> bool {
        self.dependencies.is_policy_or_org_bound()
            || matches!(
                self.support_class,
                LifecycleStage::PolicyGated | LifecycleStage::MirrorOnly
            )
    }

    /// The honestly-capped support-claim strength: the scorecard class capped by what
    /// imported-versus-native confidence and certification freshness can back, so a stale or
    /// approximate bundle never presents a stronger claim than its evidence supports (AC1).
    pub fn honest_support_claim_strength(&self) -> BundleScorecardClass {
        let capped = self
            .scorecard_class
            .rank()
            .min(self.imported_confidence.cap_rank())
            .min(self.certification_freshness.cap_rank());
        BundleScorecardClass::from_rank(capped)
    }

    /// Whether the bundle may inherit full native-parity language: it must be a native first-party
    /// class, carry native capability confidence, and not be policy-bound (AC2).
    pub fn may_inherit_native_parity(&self) -> bool {
        self.disclosure_class.is_native_parity_class()
            && self.capability_confidence.inherits_native_parity()
            && !self.is_policy_bound()
    }

    /// Whether the claim is narrowed relative to its headline scorecard class: the honest strength
    /// dropped, the capability is not native, the bundle is policy-bound, the certification is
    /// stale, or an external narrowing was observed.
    pub fn is_claim_narrowed(&self) -> bool {
        self.honest_support_claim_strength().rank() < self.scorecard_class.rank()
            || !self.capability_confidence.inherits_native_parity()
            || self.is_policy_bound()
            || self.certification_freshness.is_stale()
            || self.degraded.is_some()
    }
}

// --- resolved projections ---

/// The resolved class-disclosure card: the bundle class, its policy owner / mirror source /
/// entitlement dependency, and its confidence / posture labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedClassDisclosureCard {
    /// The disclosure identity — identical to the claim-narrowing row.
    pub disclosure_id: String,
    /// The opaque bundle id ref.
    pub bundle_id_ref: String,
    /// The human-readable bundle name.
    pub bundle_name: String,
    /// The disclosure class the card names.
    pub disclosure_class: M5BundleDisclosureClass,
    /// The certification target backing the class.
    pub source_class: CertificationTarget,
    /// The capability confidence the compatibility claim carries.
    pub capability_confidence: M5CapabilityConfidence,
    /// A concrete, human-readable posture label built from the class and confidence.
    pub posture_label: String,
    /// The redacted policy owner label, when the bundle is policy-controlled.
    pub policy_owner: Option<String>,
    /// The redacted mirror source label, when the bundle is mirror-bounded.
    pub mirror_source: Option<String>,
    /// The redacted entitlement-dependency label, when the bundle is entitlement-bound.
    pub entitlement_dependency: Option<String>,
    /// The bundle depends on a managed registry.
    pub depends_on_managed_registry: bool,
    /// The bundle depends on org identity / entitlement.
    pub depends_on_org_identity: bool,
    /// The bundle's freshness is mirror-bounded.
    pub depends_on_mirror_freshness: bool,
    /// The bundle's availability is policy-controlled.
    pub depends_on_policy_availability: bool,
    /// The concrete reason the bundle is recommended.
    pub reason_for_recommendation: String,
    /// The card discloses the class meaning; always `true`.
    pub discloses_class_meaning: bool,
    /// The card discloses every declared dependency; always `true`.
    pub discloses_dependencies: bool,
    /// The card implies standalone local completeness (AC / requirement); always `false`.
    pub implies_standalone_local_completeness: bool,
    /// The card invents a private class meaning; always `false`.
    pub invents_private_class_meaning: bool,
}

/// The resolved claim-narrowing row: the capability confidence, the honest support-claim strength,
/// and whether — and why — native parity is narrowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedClaimNarrowingRow {
    /// The disclosure identity — identical to the card.
    pub disclosure_id: String,
    /// The opaque bundle id ref the row narrows.
    pub bundle_id_ref: String,
    /// The capability confidence the compatibility claim carries.
    pub capability_confidence: M5CapabilityConfidence,
    /// The honestly-capped support-claim strength.
    pub support_claim_strength: BundleScorecardClass,
    /// Certification freshness driving any narrowing.
    pub certification_freshness: EvidenceFreshness,
    /// Imported-vs-native confidence contributing to any narrowing.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// Whether the bundle's availability is policy-bound.
    pub policy_bound: bool,
    /// Whether the claim is narrowed relative to the headline scorecard class.
    pub is_narrowed: bool,
    /// The bundle inherits full native-parity language; `true` only for a native, non-policy-bound
    /// first-party bundle (AC2).
    pub inherits_native_parity: bool,
    /// A concrete narrowing reason built from the drivers, present when the claim is narrowed.
    pub narrowing_reason: Option<String>,
    /// The row discloses the narrowing reason whenever it narrows.
    pub discloses_narrowing_reason: bool,
    /// The row invents private stale-claim wording; always `false`.
    pub invents_stale_wording: bool,
}

/// The resolved bundle class-disclosure truth shared across the card and claim-narrowing row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBundleClassDisclosure {
    /// The stable disclosure identity.
    pub disclosure_id: String,
    /// The resolved class-disclosure card.
    pub card: M5ResolvedClassDisclosureCard,
    /// The resolved claim-narrowing row.
    pub row: M5ResolvedClaimNarrowingRow,
    /// The class is disclosed with the one shared vocabulary (AC3); always `true`.
    pub class_disclosed_with_shared_vocabulary: bool,
    /// Native-parity language is not over-claimed (AC2); always `true`.
    pub native_parity_not_overclaimed: bool,
    /// Standalone local completeness is not over-claimed when the bundle has dependencies; always
    /// `true`.
    pub standalone_completeness_not_overclaimed: bool,
    /// The recommendation reason and support-claim strength are disclosed (AC1); always `true`.
    pub recommendation_and_strength_disclosed: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedBundleClassDisclosure {
    /// True when the disclosure identity is identical across the card and claim-narrowing row.
    pub fn identity_consistent(&self) -> bool {
        self.card.disclosure_id == self.disclosure_id
            && self.row.disclosure_id == self.disclosure_id
    }

    /// True when native-parity language is not over-claimed (AC2).
    pub fn native_parity_not_overclaimed(&self) -> bool {
        self.native_parity_not_overclaimed
    }

    /// True when the recommendation reason and support-claim strength are disclosed (AC1).
    pub fn recommendation_and_strength_disclosed(&self) -> bool {
        self.recommendation_and_strength_disclosed
    }

    /// True when the class is disclosed with the one shared vocabulary (AC3).
    pub fn class_disclosed_with_shared_vocabulary(&self) -> bool {
        self.class_disclosed_with_shared_vocabulary
    }
}

/// Errors returned by [`resolve_bundle_class_disclosure`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BundleClassDisclosureResolutionError {
    /// The disclosure identity was empty.
    EmptyDisclosureId,
    /// The bundle id ref was empty.
    EmptyBundleIdRef,
    /// The bundle name was empty.
    EmptyBundleName,
    /// The compatible Aureline range was empty.
    EmptyCompatibleRange,
    /// The recommendation reason was empty (AC1).
    EmptyRecommendationReason,
    /// The disclosure class does not match the certification target it claims.
    ClassSourceMismatch,
    /// The disclosed capability confidence over-claims the imported-versus-native mapping.
    CapabilityConfidenceDishonest,
    /// A declared dependency is missing its required label.
    DependencyDisclosureInconsistent,
    /// An imported / org-approved / non-native bundle claimed full native parity (AC2).
    NativeParityOverclaimed,
    /// A dependent bundle implied standalone local completeness.
    StandaloneCompletenessOverclaimed,
    /// A stale / missing certification was claimed as current instead of narrowing.
    StaleClaimShownAsCurrent,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5BundleClassDisclosureResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDisclosureId => "empty_disclosure_id",
            Self::EmptyBundleIdRef => "empty_bundle_id_ref",
            Self::EmptyBundleName => "empty_bundle_name",
            Self::EmptyCompatibleRange => "empty_compatible_range",
            Self::EmptyRecommendationReason => "empty_recommendation_reason",
            Self::ClassSourceMismatch => "class_source_mismatch",
            Self::CapabilityConfidenceDishonest => "capability_confidence_dishonest",
            Self::DependencyDisclosureInconsistent => "dependency_disclosure_inconsistent",
            Self::NativeParityOverclaimed => "native_parity_overclaimed",
            Self::StandaloneCompletenessOverclaimed => "standalone_completeness_overclaimed",
            Self::StaleClaimShownAsCurrent => "stale_claim_shown_as_current",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5BundleClassDisclosureResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "bundle-class-disclosure resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BundleClassDisclosureResolutionError {}

/// Resolves one bundle class-disclosure context into its shared class-disclosure card and
/// claim-narrowing row.
///
/// The two surfaces share one disclosure identity, so a bundle's class, dependency posture, and
/// narrowed claim never drift between surfaces. The disclosure class must be honest for the
/// certification target; the capability confidence must be honest for the imported-versus-native
/// mapping; a bundle only inherits native-parity language when it is native first-party, native
/// confidence, and not policy-bound; a dependent bundle never implies standalone local
/// completeness; a stale certification never reads as current; and the claim-narrowing row names a
/// concrete reason whenever it narrows.
pub fn resolve_bundle_class_disclosure(
    input: &M5BundleClassDisclosureInput,
) -> Result<M5ResolvedBundleClassDisclosure, M5BundleClassDisclosureResolutionError> {
    if input.disclosure_id.trim().is_empty() {
        return Err(M5BundleClassDisclosureResolutionError::EmptyDisclosureId);
    }
    if input.bundle_id_ref.trim().is_empty() {
        return Err(M5BundleClassDisclosureResolutionError::EmptyBundleIdRef);
    }
    if input.bundle_name.trim().is_empty() {
        return Err(M5BundleClassDisclosureResolutionError::EmptyBundleName);
    }
    if input.compatible_aureline_range.trim().is_empty() {
        return Err(M5BundleClassDisclosureResolutionError::EmptyCompatibleRange);
    }
    // AC1: the disclosure must state why the bundle is recommended.
    if input.reason_for_recommendation.trim().is_empty() {
        return Err(M5BundleClassDisclosureResolutionError::EmptyRecommendationReason);
    }

    if input_carries_forbidden_material(input) {
        return Err(M5BundleClassDisclosureResolutionError::ForbiddenMaterial);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5BundleClassDisclosureResolutionError::DegradedLabelGeneric);
        }
    }

    // The disclosure class must be honest for the certification target the manifest claims.
    if !input
        .disclosure_class
        .permits_source_class(input.source_class)
    {
        return Err(M5BundleClassDisclosureResolutionError::ClassSourceMismatch);
    }

    // The capability confidence must not over-claim the imported-versus-native mapping.
    if !input
        .capability_confidence
        .is_honest_for_imported(input.imported_confidence)
    {
        return Err(M5BundleClassDisclosureResolutionError::CapabilityConfidenceDishonest);
    }

    // Every declared dependency must carry its required label.
    if !input.dependencies.is_consistent() {
        return Err(M5BundleClassDisclosureResolutionError::DependencyDisclosureInconsistent);
    }

    // AC2: only a native first-party, native-confidence, non-policy-bound bundle may claim full
    // native parity.
    if input.claims_full_native_parity && !input.may_inherit_native_parity() {
        return Err(M5BundleClassDisclosureResolutionError::NativeParityOverclaimed);
    }

    // A dependent bundle never implies standalone local completeness.
    if input.implies_standalone_local_completeness && input.dependencies.has_any_dependency() {
        return Err(M5BundleClassDisclosureResolutionError::StandaloneCompletenessOverclaimed);
    }

    // A stale / missing certification narrows the claim rather than being shown as current.
    if input.claims_current_despite_stale && input.certification_freshness.is_stale() {
        return Err(M5BundleClassDisclosureResolutionError::StaleClaimShownAsCurrent);
    }

    let inherits_native_parity = input.may_inherit_native_parity();
    let is_narrowed = input.is_claim_narrowed();
    let support_claim_strength = input.honest_support_claim_strength();

    let posture_label = build_posture_label(input);
    let narrowing_reason = if is_narrowed {
        Some(build_narrowing_reason(input, support_claim_strength))
    } else {
        None
    };

    let card = M5ResolvedClassDisclosureCard {
        disclosure_id: input.disclosure_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        bundle_name: input.bundle_name.clone(),
        disclosure_class: input.disclosure_class,
        source_class: input.source_class,
        capability_confidence: input.capability_confidence,
        posture_label,
        policy_owner: input.dependencies.policy_owner.clone(),
        mirror_source: input.dependencies.mirror_source.clone(),
        entitlement_dependency: input.dependencies.entitlement_dependency.clone(),
        depends_on_managed_registry: input.dependencies.depends_on_managed_registry,
        depends_on_org_identity: input.dependencies.depends_on_org_identity,
        depends_on_mirror_freshness: input.dependencies.depends_on_mirror_freshness,
        depends_on_policy_availability: input.dependencies.depends_on_policy_availability,
        reason_for_recommendation: input.reason_for_recommendation.clone(),
        discloses_class_meaning: true,
        discloses_dependencies: true,
        implies_standalone_local_completeness: false,
        invents_private_class_meaning: false,
    };

    let row = M5ResolvedClaimNarrowingRow {
        disclosure_id: input.disclosure_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        capability_confidence: input.capability_confidence,
        support_claim_strength,
        certification_freshness: input.certification_freshness,
        imported_confidence: input.imported_confidence,
        policy_bound: input.is_policy_bound(),
        is_narrowed,
        inherits_native_parity,
        narrowing_reason,
        discloses_narrowing_reason: is_narrowed,
        invents_stale_wording: false,
    };

    Ok(M5ResolvedBundleClassDisclosure {
        disclosure_id: input.disclosure_id.clone(),
        card,
        row,
        class_disclosed_with_shared_vocabulary: true,
        native_parity_not_overclaimed: true,
        standalone_completeness_not_overclaimed: true,
        recommendation_and_strength_disclosed: true,
        degraded: input.degraded.clone(),
    })
}

/// Builds a concrete, deterministic posture label from the disclosure class and capability
/// confidence — never a generic non-answer.
fn build_posture_label(input: &M5BundleClassDisclosureInput) -> String {
    format!(
        "{} bundle, {} compatibility, {} support",
        input.disclosure_class.label(),
        input.capability_confidence.label(),
        input.support_class.as_str(),
    )
}

/// Builds a concrete narrowing reason from the typed drivers, so the reason is always specific and
/// export-safe rather than free-form.
fn build_narrowing_reason(
    input: &M5BundleClassDisclosureInput,
    strength: BundleScorecardClass,
) -> String {
    let mut drivers: Vec<String> = Vec::new();
    if !input.capability_confidence.inherits_native_parity() {
        drivers.push(format!(
            "compatibility is {} rather than native",
            input.capability_confidence.label()
        ));
    }
    if input.is_policy_bound() {
        drivers.push("availability is policy-bound".to_owned());
    }
    if input.certification_freshness.is_stale() {
        drivers.push(format!(
            "certification is {}",
            input.certification_freshness.as_str()
        ));
    }
    if strength.rank() < input.scorecard_class.rank() {
        drivers.push(format!("support claim capped to {}", strength.as_str()));
    }
    if drivers.is_empty() {
        drivers.push("an external narrowing was observed".to_owned());
    }
    format!("Claim narrowed: {}", drivers.join("; "))
}

/// True when any label, ref, or note on the input carries obviously forbidden material.
fn input_carries_forbidden_material(input: &M5BundleClassDisclosureInput) -> bool {
    let mut values: Vec<&str> = vec![
        input.disclosure_id.as_str(),
        input.surface_label.as_str(),
        input.bundle_id_ref.as_str(),
        input.bundle_name.as_str(),
        input.compatible_aureline_range.as_str(),
        input.reason_for_recommendation.as_str(),
    ];
    if let Some(owner) = &input.dependencies.policy_owner {
        values.push(owner.as_str());
    }
    if let Some(source) = &input.dependencies.mirror_source {
        values.push(source.as_str());
    }
    if let Some(entitlement) = &input.dependencies.entitlement_dependency {
        values.push(entitlement.as_str());
    }
    values.into_iter().any(value_is_forbidden)
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet reconstructs
/// class-disclosure truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleClassDisclosureCase {
    /// The resolver input.
    pub input: M5BundleClassDisclosureInput,
    /// The resolved class-disclosure truth. Must equal `resolve_bundle_class_disclosure(&input)`.
    pub resolved: M5ResolvedBundleClassDisclosure,
}

impl M5BundleClassDisclosureCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BundleClassDisclosureInput) -> Self {
        let resolved =
            resolve_bundle_class_disclosure(&input).expect("seed class-disclosure case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_bundle_class_disclosure(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one class-disclosure surface family bound to the shared
/// disclosure contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDisclosureSurfaceRow {
    /// The class-disclosure surface family.
    pub surface_family: M5BundleDisclosureSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Disclosure classes this surface can render (must be non-empty).
    pub disclosure_classes: Vec<M5BundleDisclosureClass>,
    /// Capability confidences this surface can render (must be non-empty).
    pub capability_confidences: Vec<M5CapabilityConfidence>,
    /// Source classes this surface can disclose (must be non-empty).
    pub source_classes: Vec<CertificationTarget>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5BundleTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5BundleDisclosureExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5BundleComponentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_disclosures: Vec<M5BundleClassDisclosureCase>,
    /// Hard invariant: this row never over-claims native parity. MUST be `false`.
    pub overclaims_native_parity: bool,
    /// Hard invariant: this row never implies standalone completeness. MUST be `false`.
    pub implies_standalone_completeness: bool,
    /// Hard invariant: this row never collapses the class to a generic label. MUST be `false`.
    pub collapses_class_to_generic: bool,
}

impl M5BundleDisclosureSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BundleDisclosureExportField> =
            self.export_fields.iter().copied().collect();
        M5BundleDisclosureExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.overclaims_native_parity
            && !self.implies_standalone_completeness
            && !self.collapses_class_to_generic
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDisclosureVocabularySet {
    /// Class-disclosure surface-family tokens.
    pub surface_families: Vec<String>,
    /// Disclosure-class tokens.
    pub disclosure_classes: Vec<String>,
    /// Capability-confidence tokens (the one shared compatibility vocabulary).
    pub capability_confidences: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Source-class tokens (reused from the bundle-manifest contract).
    pub source_classes: Vec<String>,
    /// Bundle-class tokens (reused from the entry-governance contract).
    pub bundle_classes: Vec<String>,
    /// Signer / source-trust tokens (reused from the entry-governance contract).
    pub signer_sources: Vec<String>,
    /// Support-class / lifecycle tokens (reused from the bundle-manifest contract).
    pub support_classes: Vec<String>,
    /// Scorecard-class tokens (reused from the scorecard contract).
    pub scorecard_classes: Vec<String>,
    /// Certification-freshness tokens (reused from the scorecard contract).
    pub freshness_states: Vec<String>,
    /// Imported-vs-native confidence tokens (reused from the scorecard contract).
    pub imported_confidences: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5BundleDisclosureVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5BundleDisclosureSurfaceFamily::ALL,
                M5BundleDisclosureSurfaceFamily::as_str,
            ),
            disclosure_classes: tokens(
                &M5BundleDisclosureClass::ALL,
                M5BundleDisclosureClass::as_str,
            ),
            capability_confidences: tokens(
                &M5CapabilityConfidence::ALL,
                M5CapabilityConfidence::as_str,
            ),
            export_fields: tokens(
                &M5BundleDisclosureExportField::ALL,
                M5BundleDisclosureExportField::as_str,
            ),
            source_classes: tokens(&CertificationTarget::ALL, CertificationTarget::as_str),
            bundle_classes: tokens(&BundleClass::ALL, BundleClass::as_str),
            signer_sources: tokens(&SourceTrust::ALL, SourceTrust::as_str),
            support_classes: tokens(&LifecycleStage::ALL, LifecycleStage::as_str),
            scorecard_classes: tokens(&BundleScorecardClass::ALL, BundleScorecardClass::as_str),
            freshness_states: tokens(&EvidenceFreshness::ALL, EvidenceFreshness::as_str),
            imported_confidences: tokens(
                &ImportedVsNativeConfidence::ALL,
                ImportedVsNativeConfidence::as_str,
            ),
            truth_modes: tokens(&M5BundleTruthMode::ALL, M5BundleTruthMode::as_str),
            downgrade_triggers: tokens(
                &DOWNGRADE_TRIGGER_ALL,
                M5BundleComponentDowngradeTrigger::as_str,
            ),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5BundleComponentDowngradeTrigger; 9] = [
    M5BundleComponentDowngradeTrigger::StaleCertification,
    M5BundleComponentDowngradeTrigger::MirrorStale,
    M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
    M5BundleComponentDowngradeTrigger::UnverifiedSigner,
    M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
    M5BundleComponentDowngradeTrigger::IncompatibleAureline,
    M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
    M5BundleComponentDowngradeTrigger::ImportedNotNative,
    M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDisclosureGovernanceReview {
    /// One primitive carries card and claim-narrowing-row truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Disclosure identity is preserved across the card and row.
    pub disclosure_identity_preserved_across_surfaces: bool,
    /// The class is disclosed with one shared vocabulary across migration, start center, docs, and
    /// exports.
    pub class_disclosed_with_shared_vocabulary: bool,
    /// Imported / org-approved bundles never inherit native parity when capability-mapped or
    /// policy-bound.
    pub native_parity_never_inherited_when_mapped: bool,
    /// A dependent bundle never implies standalone local completeness.
    pub dependency_posture_disclosed: bool,
    /// The recommendation reason and support-claim strength are always disclosed.
    pub recommendation_and_strength_disclosed: bool,
    /// The support / export packet reconstructs class-disclosure truth.
    pub support_export_reconstructs_disclosure: bool,
    /// Later M5 rows cannot invent parallel class / confidence vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDisclosureConsumerProjection {
    /// Start-center / bundle-detail / migration / docs / diagnostics / support surfaces all consume
    /// the shared primitive.
    pub disclosure_surfaces_consume_shared_primitive: bool,
    /// The disclosure resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The claim-narrowing row reads a single canonical disclosure source.
    pub narrowing_reads_single_source: bool,
    /// Support / export reads a single canonical disclosure source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the class-disclosure primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDisclosureReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting disclosure audit.
    pub disclosure_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BundleClassDisclosurePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BundleClassDisclosurePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleDisclosureSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleDisclosureVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleDisclosureGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleDisclosureConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleDisclosureReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 bundle class-disclosure primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleClassDisclosurePacket {
    /// Record kind; must equal [`M5_BUNDLE_CLASS_DISCLOSURE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleDisclosureSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleDisclosureVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleDisclosureGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleDisclosureConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleDisclosureReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BundleClassDisclosurePacket {
    /// Builds an M5 bundle class-disclosure primitive packet from stable-lane input.
    pub fn new(input: M5BundleClassDisclosurePacketInput) -> Self {
        Self {
            record_kind: M5_BUNDLE_CLASS_DISCLOSURE_RECORD_KIND.to_owned(),
            schema_version: M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 class-disclosure primitive invariants.
    pub fn validate(&self) -> Vec<M5BundleDisclosureViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUNDLE_CLASS_DISCLOSURE_RECORD_KIND {
            violations.push(M5BundleDisclosureViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_VERSION {
            violations.push(M5BundleDisclosureViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BundleDisclosureViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 bundle class-disclosure primitive packet serializes"),
        ) {
            violations.push(M5BundleDisclosureViolation::RawMaterialInExport);
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
            .expect("m5 bundle class-disclosure primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,disclosure_classes,capability_confidences,source_classes,truth_modes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.disclosure_classes, |v| v.as_str()),
                join_tokens(&row.capability_confidences, |v| v.as_str()),
                join_tokens(&row.source_classes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_disclosures.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Bundle Class-Disclosure Primitive: Class-Disclosure Card and Claim-Narrowing Row\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Class-disclosure surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5BundleDisclosureSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Disclosure classes: {}\n",
            self.vocabulary_set.disclosure_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Capability confidences: {}\n",
            self.vocabulary_set.capability_confidences.join(", ")
        ));
        out.push_str("\n## Class-disclosure surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_disclosures.len()
            ));
            for case in &row.example_disclosures {
                out.push_str(&format!(
                    "    - `{}` → {} class, {} compatibility, {} claim{}\n",
                    case.resolved.disclosure_id,
                    case.resolved.card.disclosure_class.as_str(),
                    case.resolved.card.capability_confidence.as_str(),
                    case.resolved.row.support_claim_strength.as_str(),
                    if case.resolved.row.is_narrowed {
                        " (narrowed)"
                    } else {
                        ""
                    },
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 class-disclosure export.
#[derive(Debug)]
pub enum M5BundleDisclosureArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BundleDisclosureViolation>),
}

impl fmt::Display for M5BundleDisclosureArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 bundle class-disclosure primitive export parse failed: {error}"
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
                    "m5 bundle class-disclosure primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BundleDisclosureArtifactError {}

/// Validation failures emitted by [`M5BundleClassDisclosurePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BundleDisclosureViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required class-disclosure surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no disclosure classes.
    DisclosureClassMissing,
    /// A surface row declares no capability confidences.
    CapabilityConfidenceMissing,
    /// A surface row declares no source classes.
    SourceClassMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked disclosure cases.
    ExampleDisclosuresMissing,
    /// A worked disclosure case does not match a fresh resolve of its input.
    ExampleDisclosureDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves why a bundle is recommended and how strong its claim is (AC1).
    RecommendationStrengthUnproven,
    /// No worked case proves an imported / org-approved bundle does not inherit native parity (AC2).
    NativeParityNarrowingUnproven,
    /// The matrix does not prove class-disclosure stability across docs / help and support (AC3).
    CrossSurfaceStabilityUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BundleDisclosureViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::DisclosureClassMissing => "disclosure_class_missing",
            Self::CapabilityConfidenceMissing => "capability_confidence_missing",
            Self::SourceClassMissing => "source_class_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleDisclosuresMissing => "example_disclosures_missing",
            Self::ExampleDisclosureDrift => "example_disclosure_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::RecommendationStrengthUnproven => "recommendation_strength_unproven",
            Self::NativeParityNarrowingUnproven => "native_parity_narrowing_unproven",
            Self::CrossSurfaceStabilityUnproven => "cross_surface_stability_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 class-disclosure export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_bundle_class_disclosure_export(
) -> Result<M5BundleClassDisclosurePacket, M5BundleDisclosureArtifactError> {
    let packet: M5BundleClassDisclosurePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-class-disclosure-primitive-proof/support_export.json"
    )))
    .map_err(M5BundleDisclosureArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BundleDisclosureArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_REF,
        M5_BUNDLE_CLASS_DISCLOSURE_DOC_REF,
        M5_BUNDLE_CLASS_DISCLOSURE_COMPONENT_MATRIX_REF,
        M5_BUNDLE_CLASS_DISCLOSURE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BundleDisclosureViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BundleDisclosureViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    let present: BTreeSet<M5BundleDisclosureSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5BundleDisclosureSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BundleDisclosureViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5BundleDisclosureViolation::SurfaceRowIncomplete);
        }
        if row.disclosure_classes.is_empty() {
            violations.push(M5BundleDisclosureViolation::DisclosureClassMissing);
        }
        if row.capability_confidences.is_empty() {
            violations.push(M5BundleDisclosureViolation::CapabilityConfidenceMissing);
        }
        if row.source_classes.is_empty() {
            violations.push(M5BundleDisclosureViolation::SourceClassMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5BundleDisclosureViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BundleDisclosureViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BundleDisclosureViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BundleDisclosureViolation::ConsumerSurfacesMissing);
        }
        if row.example_disclosures.is_empty() {
            violations.push(M5BundleDisclosureViolation::ExampleDisclosuresMissing);
        }
        if row
            .example_disclosures
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BundleDisclosureViolation::ExampleDisclosureDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5BundleDisclosureViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across the matrix:
/// every case names why the bundle is recommended and how strong its claim is, and at least one case
/// narrows its claim (AC1); at least one imported or org-approved bundle does not inherit native
/// parity, and every non-native case never inherits native parity (AC2); the matrix spans a docs /
/// help surface and a support / export surface with one shared vocabulary and one identity (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    let cases: Vec<&M5ResolvedBundleClassDisclosure> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter().map(|case| &case.resolved))
        .collect();

    // AC1: every case discloses the recommendation reason and support strength, and at least one
    // case shows a narrowed claim so strength is honestly represented rather than always headline.
    let recommendation_proven = cases.iter().all(|resolved| {
        resolved.recommendation_and_strength_disclosed()
            && !resolved.card.reason_for_recommendation.trim().is_empty()
    }) && cases.iter().any(|resolved| resolved.row.is_narrowed);
    if !recommendation_proven {
        violations.push(M5BundleDisclosureViolation::RecommendationStrengthUnproven);
    }

    // AC2: every case honours native-parity honesty; every non-native-parity class never inherits
    // native parity; at least one imported or org-approved bundle is shown not inheriting parity.
    let non_native_never_inherits = cases.iter().all(|resolved| {
        resolved.native_parity_not_overclaimed()
            && (resolved.card.disclosure_class.is_native_parity_class()
                || !resolved.row.inherits_native_parity)
    });
    let imported_or_managed_narrowed = cases.iter().any(|resolved| {
        matches!(
            resolved.card.disclosure_class,
            M5BundleDisclosureClass::ImportedUserHandoff | M5BundleDisclosureClass::ManagedApproved
        ) && !resolved.row.inherits_native_parity
    });
    if !(non_native_never_inherits && imported_or_managed_narrowed) {
        violations.push(M5BundleDisclosureViolation::NativeParityNarrowingUnproven);
    }

    // AC3: every case shares one identity and the shared vocabulary; the matrix spans a docs / help
    // surface and a support / export surface, and demonstrates both a native and a non-native class.
    let has_docs = packet
        .surface_rows
        .iter()
        .any(|row| row.surface_family.is_docs_help());
    let has_support = packet
        .surface_rows
        .iter()
        .any(|row| row.surface_family.is_support_export());
    let has_native = cases
        .iter()
        .any(|resolved| resolved.card.disclosure_class.is_native_parity_class());
    let has_non_native = cases
        .iter()
        .any(|resolved| !resolved.card.disclosure_class.is_native_parity_class());
    let stability_proven = cases.iter().all(|resolved| {
        resolved.identity_consistent() && resolved.class_disclosed_with_shared_vocabulary()
    }) && has_docs
        && has_support
        && has_native
        && has_non_native;
    if !stability_proven {
        violations.push(M5BundleDisclosureViolation::CrossSurfaceStabilityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.disclosure_identity_preserved_across_surfaces,
        review.class_disclosed_with_shared_vocabulary,
        review.native_parity_never_inherited_when_mapped,
        review.dependency_posture_disclosed,
        review.recommendation_and_strength_disclosed,
        review.support_export_reconstructs_disclosure,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BundleDisclosureViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.disclosure_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.narrowing_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BundleDisclosureViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5BundleClassDisclosurePacket,
    violations: &mut Vec<M5BundleDisclosureViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.disclosure_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BundleDisclosureViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
