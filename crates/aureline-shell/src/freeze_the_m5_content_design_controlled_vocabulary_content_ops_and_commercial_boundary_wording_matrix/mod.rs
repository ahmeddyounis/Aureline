//! Frozen M5 content-design, controlled-vocabulary, content-ops metadata, and
//! commercial-boundary wording matrix.
//!
//! This module locks the canonical M5 product-wording object model into one
//! export-safe packet. Each [`M5ContentObjectRow`] names one governed wording or
//! content object — the safety-critical UI string, the controlled glossary term,
//! the action-label pattern, the error/recovery block, the AI copy guardrail, the
//! count/scope phrase set, the content-ops metadata artifact, and the
//! commercial-boundary wording review — and binds it to its qualification class,
//! required fields, the controlled state vocabularies it carries, the concrete
//! vocabulary tokens it admits, evidence requirements, the proof packet that keeps
//! it current, downgrade triggers, rollback posture, source contracts, and
//! consumer-surface parity.
//!
//! The matrix is the single source of truth for whether claimed M5 user-facing
//! surfaces may publish wording claims. UI, CLI/help, docs, support exports,
//! release notes, screenshots/demos, AI surfaces, and commercial-boundary prompts
//! consume this packet rather than maintaining parallel copy lists: safety-critical
//! strings keep stable ids and controlled terms; action labels and counts stay
//! scope-honest; error copy explains failure, remaining capability, and the next
//! action; AI wording never overstates confidence or autonomy; content-ops
//! artifacts keep version/source metadata; and hosted/open/self-hosted/commercial
//! language cannot drift from the actual product boundary.
//!
//! The controlled vocabularies mirror the canonical tokens already owned by the
//! controlled glossary, the count/scope/freshness grammar, the AI copy guardrails
//! contract, the product truth vocabulary, and the deployment-profile register;
//! the matrix freezes them in one self-describing [`M5ContentVocabularySet`] rather
//! than minting parallel tokens. It references the upstream copy, AI, and
//! governance contracts by id. Raw message bodies, raw provider payloads,
//! credentials, secret material, and untranslated free-text prose stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/content/freeze-the-m5-content-design-controlled-vocabulary-content-ops-and-commercial-boundary-wording-matrix.schema.json`](../../../../schemas/content/freeze-the-m5-content-design-controlled-vocabulary-content-ops-and-commercial-boundary-wording-matrix.schema.json).
//! The contract doc is
//! [`docs/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md`](../../../../docs/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md).
//! The protected fixture directory is
//! [`fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/`](../../../../fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_content_wording_matrix, seeded_m5_content_wording_matrix_ai_guardrail_narrowed,
    seeded_m5_content_wording_matrix_commercial_boundary_held, M5_CONTENT_WORDING_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ContentWordingMatrixPacket`].
pub const M5_CONTENT_WORDING_MATRIX_RECORD_KIND: &str =
    "freeze_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix";

/// Schema version for M5 content-wording matrix records.
pub const M5_CONTENT_WORDING_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_CONTENT_WORDING_MATRIX_SCHEMA_REF: &str =
    "schemas/content/freeze-the-m5-content-design-controlled-vocabulary-content-ops-and-commercial-boundary-wording-matrix.schema.json";

/// Repo-relative path of the M5 content-wording matrix contract doc.
pub const M5_CONTENT_WORDING_MATRIX_DOC_REF: &str =
    "docs/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md";

/// Repo-relative path of the frozen UI copy contract (action labels, error copy,
/// AI copy guardrails).
pub const M5_CONTENT_UI_COPY_CONTRACT_REF: &str = "docs/copy/ui_copy_contract.md";

/// Repo-relative path of the frozen naming and state-label contract.
pub const M5_CONTENT_NAMING_LABEL_CONTRACT_REF: &str =
    "docs/copy/naming_and_state_label_contract.md";

/// Repo-relative path of the frozen count/scope/freshness grammar contract.
pub const M5_CONTENT_COUNT_SCOPE_GRAMMAR_REF: &str = "docs/copy/count_scope_freshness_grammar.md";

/// Repo-relative path of the frozen translation-safe content-ops contract.
pub const M5_CONTENT_CONTENT_OPS_CONTRACT_REF: &str =
    "docs/copy/translation_safe_content_ops_contract.md";

/// Repo-relative path of the frozen AI copy guardrails contract.
pub const M5_CONTENT_AI_COPY_GUARDRAILS_CONTRACT_REF: &str =
    "docs/ai/ai_copy_guardrails_contract.md";

/// Repo-relative path of the controlled glossary register.
pub const M5_CONTENT_CONTROLLED_GLOSSARY_REF: &str = "artifacts/copy/controlled_glossary.yaml";

/// Repo-relative path of the count/scope/freshness controlled term set.
pub const M5_CONTENT_COUNT_SCOPE_TERM_SET_REF: &str = "artifacts/copy/count_scope_term_set.yaml";

/// Repo-relative path of the product truth vocabulary register (lifecycle,
/// authority, retention, install/update, deployment-profile state classes).
pub const M5_CONTENT_PRODUCT_TRUTH_VOCABULARY_REF: &str =
    "artifacts/governance/product_truth_vocabulary.yaml";

/// Repo-relative path of the deployment-profile register that owns the
/// hosting-boundary and managed/local/self-hosted/open vocabulary.
pub const M5_CONTENT_DEPLOYMENT_PROFILES_REF: &str =
    "artifacts/governance/deployment_profiles.yaml";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONTENT_WORDING_MATRIX_FIXTURE_DIR: &str =
    "fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONTENT_WORDING_MATRIX_ARTIFACT_REF: &str =
    "artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CONTENT_WORDING_MATRIX_SUMMARY_REF: &str =
    "artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md";

/// One of the eight governed M5 content/wording objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentObjectKind {
    /// Safety-critical UI string with a stable message id and controlled terms.
    SafetyCriticalUiString,
    /// Controlled glossary / state-label term with a reserved meaning.
    GlossaryTerm,
    /// Verb-first, outcome-specific action-label pattern.
    ActionLabelPattern,
    /// Four-part error/recovery block (what failed, why, what still works, next
    /// safe action).
    ErrorRecoveryBlock,
    /// AI copy guardrail governing certainty, evidence, and autonomy language.
    AiCopyGuardrail,
    /// Count/scope/freshness phrase set that keeps counts scope-honest.
    CountScopePhraseSet,
    /// Content-ops metadata artifact carrying version/source metadata for docs,
    /// help, exports, and screenshots/demos.
    ContentOpsArtifact,
    /// Commercial-boundary wording review for hosted/open/self-hosted/managed
    /// language.
    CommercialBoundaryWording,
}

impl M5ContentObjectKind {
    /// Every governed object, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SafetyCriticalUiString,
        Self::GlossaryTerm,
        Self::ActionLabelPattern,
        Self::ErrorRecoveryBlock,
        Self::AiCopyGuardrail,
        Self::CountScopePhraseSet,
        Self::ContentOpsArtifact,
        Self::CommercialBoundaryWording,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafetyCriticalUiString => "safety_critical_ui_string",
            Self::GlossaryTerm => "glossary_term",
            Self::ActionLabelPattern => "action_label_pattern",
            Self::ErrorRecoveryBlock => "error_recovery_block",
            Self::AiCopyGuardrail => "ai_copy_guardrail",
            Self::CountScopePhraseSet => "count_scope_phrase_set",
            Self::ContentOpsArtifact => "content_ops_artifact",
            Self::CommercialBoundaryWording => "commercial_boundary_wording",
        }
    }

    /// Controlled state vocabularies this object kind MUST declare.
    pub fn required_state_vocabularies(self) -> &'static [M5ContentStateVocabulary] {
        use M5ContentStateVocabulary as V;
        match self {
            Self::SafetyCriticalUiString => &[
                V::LifecycleState,
                V::TrustClass,
                V::PolicyState,
                V::FreshnessState,
            ],
            Self::GlossaryTerm => &[V::LifecycleState, V::TrustClass, V::ClientScope],
            Self::ActionLabelPattern => &[V::PolicyState, V::ClientScope],
            Self::ErrorRecoveryBlock => &[V::PolicyState, V::FreshnessState],
            Self::AiCopyGuardrail => &[V::TrustClass, V::PolicyState, V::FreshnessState],
            Self::CountScopePhraseSet => &[V::FreshnessState, V::CompatibilityState],
            Self::ContentOpsArtifact => &[V::CompatibilityState, V::FreshnessState],
            Self::CommercialBoundaryWording => {
                &[V::HostingBoundary, V::EditionLabel, V::ClientScope]
            }
        }
    }
}

/// Qualification class for an M5 content/wording object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentQualificationClass {
    /// Object qualifies for the Stable claim.
    Stable,
    /// Object is narrowed to Beta.
    Beta,
    /// Object is narrowed to Preview.
    Preview,
    /// Object is experimental and not claimed.
    Experimental,
    /// Object is unavailable on this build.
    Unavailable,
    /// Object is held pending upstream resolution.
    Held,
}

impl M5ContentQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the object may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Names one of the controlled state vocabularies a content object carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentStateVocabulary {
    /// Capability lifecycle / release-stage state.
    LifecycleState,
    /// Source / destination authority (trust) class.
    TrustClass,
    /// Policy / entitlement gating state.
    PolicyState,
    /// Compatibility state between a claim and the active target.
    CompatibilityState,
    /// Freshness state of the underlying data or evidence.
    FreshnessState,
    /// Client-scope label (which client the wording applies to).
    ClientScope,
    /// Hosting / deployment-topology boundary.
    HostingBoundary,
    /// Managed/local/self-hosted/open edition label.
    EditionLabel,
}

impl M5ContentStateVocabulary {
    /// Every vocabulary, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LifecycleState,
        Self::TrustClass,
        Self::PolicyState,
        Self::CompatibilityState,
        Self::FreshnessState,
        Self::ClientScope,
        Self::HostingBoundary,
        Self::EditionLabel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleState => "lifecycle_state",
            Self::TrustClass => "trust_class",
            Self::PolicyState => "policy_state",
            Self::CompatibilityState => "compatibility_state",
            Self::FreshnessState => "freshness_state",
            Self::ClientScope => "client_scope",
            Self::HostingBoundary => "hosting_boundary",
            Self::EditionLabel => "edition_label",
        }
    }
}

/// Controlled lifecycle / release-stage state for a wording claim.
///
/// Mirrors the canonical lifecycle terms owned by the product truth vocabulary so
/// wording surfaces never mint a parallel release-stage synonym.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentLifecycleState {
    /// Experimental Labs capability.
    Labs,
    /// Preview capability, not yet broadly claimed.
    Preview,
    /// Beta capability.
    Beta,
    /// Stable, broadly claimed capability.
    Stable,
    /// Long-term-support-facing capability.
    LtsFacing,
    /// Deprecated capability scheduled for removal.
    Deprecated,
    /// Capability disabled by policy.
    DisabledByPolicy,
    /// Retired capability.
    Retired,
}

impl ContentLifecycleState {
    /// Every lifecycle state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Labs,
        Self::Preview,
        Self::Beta,
        Self::Stable,
        Self::LtsFacing,
        Self::Deprecated,
        Self::DisabledByPolicy,
        Self::Retired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::LtsFacing => "lts_facing",
            Self::Deprecated => "deprecated",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::Retired => "retired",
        }
    }
}

/// Controlled source / destination authority (trust) class for a wording claim.
///
/// Mirrors the canonical authority terms owned by the product truth vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentTrustClass {
    /// Official, publicly verifiable first-party source.
    OfficialPublic,
    /// Official, private (managed/tenant) first-party source.
    OfficialPrivate,
    /// Community source, not vendor-verified.
    Community,
    /// Third-party vendor source.
    ThirdPartyVendor,
}

impl ContentTrustClass {
    /// Every trust class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OfficialPublic,
        Self::OfficialPrivate,
        Self::Community,
        Self::ThirdPartyVendor,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialPrivate => "official_private",
            Self::Community => "community",
            Self::ThirdPartyVendor => "third_party_vendor",
        }
    }
}

/// Controlled policy / entitlement gating state for a wording claim.
///
/// Mirrors the trust/policy gating labels owned by the controlled glossary so a
/// `Trust required`, `Restricted`, or `Policy blocked` reason can never be softened
/// into generic unavailability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentPolicyState {
    /// The action is allowed.
    Allowed,
    /// Trust must be established before the action proceeds.
    TrustRequired,
    /// The action is restricted to a narrower scope.
    Restricted,
    /// The action requires explicit review before it proceeds.
    RequiresReview,
    /// The action is blocked by policy.
    PolicyBlocked,
}

impl ContentPolicyState {
    /// Every policy state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Allowed,
        Self::TrustRequired,
        Self::Restricted,
        Self::RequiresReview,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::TrustRequired => "trust_required",
            Self::Restricted => "restricted",
            Self::RequiresReview => "requires_review",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Controlled compatibility state between a wording claim and the active target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentCompatibilityState {
    /// The claim is compatible with the active target.
    Compatible,
    /// The claim is compatible within an accepted minor-skew window.
    MinorSkewCompatible,
    /// The claim is incompatible with the active target.
    Incompatible,
    /// Compatibility could not be verified.
    UnverifiedCompatibility,
    /// The claim refers to a deprecated path.
    DeprecatedPath,
}

impl ContentCompatibilityState {
    /// Every compatibility state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Compatible,
        Self::MinorSkewCompatible,
        Self::Incompatible,
        Self::UnverifiedCompatibility,
        Self::DeprecatedPath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::MinorSkewCompatible => "minor_skew_compatible",
            Self::Incompatible => "incompatible",
            Self::UnverifiedCompatibility => "unverified_compatibility",
            Self::DeprecatedPath => "deprecated_path",
        }
    }
}

/// Controlled freshness state for the data or evidence behind a wording claim.
///
/// Mirrors the count/scope/freshness grammar so `stale` always means the same
/// reserved state across UI, CLI, docs, exports, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentFreshnessState {
    /// Proven current for the declared scope and freshness basis.
    ProvenCurrent,
    /// Cached data shown with a disclosed cache posture.
    Cached,
    /// Data is warming and not yet complete.
    Warming,
    /// Prior data shown after its freshness floor or causal continuity was lost.
    Stale,
    /// Freshness could not be verified.
    Unverified,
}

impl ContentFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProvenCurrent,
        Self::Cached,
        Self::Warming,
        Self::Stale,
        Self::Unverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenCurrent => "proven_current",
            Self::Cached => "cached",
            Self::Warming => "warming",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }
}

/// Controlled client-scope label for a wording claim.
///
/// Mirrors the closed client-scope set owned by the controlled glossary so a
/// browser-companion surface can never imply full desktop parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentClientScope {
    /// Full desktop client.
    Desktop,
    /// Browser companion surface.
    BrowserCompanion,
    /// Desktop plus browser companion.
    DesktopPlusBrowserCompanion,
    /// Headless-only surface.
    HeadlessOnly,
    /// Local-only posture (no managed recall, sync, or hosted evidence).
    LocalOnly,
}

impl ContentClientScope {
    /// Every client scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Desktop,
        Self::BrowserCompanion,
        Self::DesktopPlusBrowserCompanion,
        Self::HeadlessOnly,
        Self::LocalOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::BrowserCompanion => "browser_companion",
            Self::DesktopPlusBrowserCompanion => "desktop_plus_browser_companion",
            Self::HeadlessOnly => "headless_only",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Controlled hosting / deployment-topology boundary for a wording claim.
///
/// Mirrors the frozen deployment-profile vocabulary so hosting language never
/// drifts from the actual deployment profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentHostingBoundary {
    /// Individual, local deployment.
    IndividualLocal,
    /// Self-hosted deployment.
    SelfHosted,
    /// Enterprise online deployment.
    EnterpriseOnline,
    /// Air-gapped deployment.
    AirGapped,
    /// Managed cloud deployment.
    ManagedCloud,
}

impl ContentHostingBoundary {
    /// Every hosting boundary, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::IndividualLocal,
        Self::SelfHosted,
        Self::EnterpriseOnline,
        Self::AirGapped,
        Self::ManagedCloud,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }
}

/// Controlled managed/local/self-hosted/open edition label for a wording claim.
///
/// Names the commercial edition a surface may claim. Open, local-independent
/// language can never be applied when managed recall, sync, or hosted services
/// participated, and commercial language can never be applied to an open-source
/// capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentEditionLabel {
    /// Open-source capability.
    OpenSource,
    /// Local-independent build with no managed dependency.
    LocalIndependent,
    /// Self-hosted edition.
    SelfHosted,
    /// Managed edition with optional or required hosted services.
    Managed,
    /// Commercial / paid edition or add-on.
    Commercial,
}

impl ContentEditionLabel {
    /// Every edition label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenSource,
        Self::LocalIndependent,
        Self::SelfHosted,
        Self::Managed,
        Self::Commercial,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSource => "open_source",
            Self::LocalIndependent => "local_independent",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::Commercial => "commercial",
        }
    }
}

/// Evidence requirement level for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentEvidenceRequirement {
    /// At least one proof packet is required.
    Required,
    /// Proof is recommended but not blocking.
    Recommended,
    /// Proof is optional.
    Optional,
    /// Not applicable for this object's current qualification.
    NotApplicable,
}

impl M5ContentEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow an object below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A label drifted from its controlled-glossary reserved meaning.
    ControlledTermDrift,
    /// A safety-critical string lost its stable message id.
    MessageIdUnstable,
    /// An overclaim of confidence, validation, freshness, or autonomy was found.
    OverclaimDetected,
    /// A count/scope phrase is no longer scope-honest.
    ScopeCountDishonest,
    /// The freshness window for the underlying data expired.
    FreshnessExpired,
    /// Commercial-boundary wording diverged from the actual product boundary.
    CommercialBoundaryDrift,
    /// A safety-critical string lost localization / message-id parity.
    LocalizationParityLost,
    /// A content-ops artifact is missing required version/source metadata.
    ContentOpsMetadataMissing,
    /// An upstream dependency object narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ContentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ControlledTermDrift,
        Self::MessageIdUnstable,
        Self::OverclaimDetected,
        Self::ScopeCountDishonest,
        Self::FreshnessExpired,
        Self::CommercialBoundaryDrift,
        Self::LocalizationParityLost,
        Self::ContentOpsMetadataMissing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ControlledTermDrift => "controlled_term_drift",
            Self::MessageIdUnstable => "message_id_unstable",
            Self::OverclaimDetected => "overclaim_detected",
            Self::ScopeCountDishonest => "scope_count_dishonest",
            Self::FreshnessExpired => "freshness_expired",
            Self::CommercialBoundaryDrift => "commercial_boundary_drift",
            Self::LocalizationParityLost => "localization_parity_lost",
            Self::ContentOpsMetadataMissing => "content_ops_metadata_missing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentRollbackPosture {
    /// The controlled term stays labeled and is never softened for tone.
    TermLabeledNeverSoftened,
    /// The stable message id is preserved; the safety string never becomes
    /// free-text.
    MessageIdStablePreserved,
    /// Overclaiming copy is blocked before it ships.
    OverclaimBlockedBeforeShip,
    /// Commercial-boundary wording stays matched to the actual product boundary.
    BoundaryWordingMatchesProduct,
    /// Counts and scopes stay honest; a narrowed scope is disclosed, never hidden.
    ScopeCountStaysHonest,
    /// Not applicable for the object's current qualification.
    NotApplicable,
}

impl M5ContentRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TermLabeledNeverSoftened => "term_labeled_never_softened",
            Self::MessageIdStablePreserved => "message_id_stable_preserved",
            Self::OverclaimBlockedBeforeShip => "overclaim_blocked_before_ship",
            Self::BoundaryWordingMatchesProduct => "boundary_wording_matches_product",
            Self::ScopeCountStaysHonest => "scope_count_stays_honest",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project a content object's qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentConsumerSurface {
    /// Product UI surface.
    ProductUi,
    /// CLI / help text surface.
    CliHelp,
    /// Documentation surface.
    Docs,
    /// Support / export packet.
    SupportExport,
    /// Release notes.
    ReleaseNotes,
    /// Screenshots and demo captures.
    ScreenshotsDemos,
    /// AI explain / answer surfaces.
    AiSurfaces,
    /// Onboarding / tour surface.
    Onboarding,
    /// Help / About surface.
    HelpAbout,
    /// Marketplace / extension storefront surface.
    Marketplace,
}

impl M5ContentConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::CliHelp => "cli_help",
            Self::Docs => "docs",
            Self::SupportExport => "support_export",
            Self::ReleaseNotes => "release_notes",
            Self::ScreenshotsDemos => "screenshots_demos",
            Self::AiSurfaces => "ai_surfaces",
            Self::Onboarding => "onboarding",
            Self::HelpAbout => "help_about",
            Self::Marketplace => "marketplace",
        }
    }
}

/// One row in the M5 content-wording matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentObjectRow {
    /// Governed content object.
    pub object_kind: M5ContentObjectKind,
    /// Qualification class earned by this object.
    pub qualification: M5ContentQualificationClass,
    /// Owner role accountable for keeping this object's wording governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required fields the object must carry.
    pub required_fields: Vec<String>,
    /// Controlled state vocabularies this object carries.
    pub state_vocabularies: Vec<M5ContentStateVocabulary>,
    /// Lifecycle states admitted by this object.
    pub lifecycle_states: Vec<ContentLifecycleState>,
    /// Trust classes admitted by this object.
    pub trust_classes: Vec<ContentTrustClass>,
    /// Policy states admitted by this object.
    pub policy_states: Vec<ContentPolicyState>,
    /// Compatibility states admitted by this object.
    pub compatibility_states: Vec<ContentCompatibilityState>,
    /// Freshness states admitted by this object.
    pub freshness_states: Vec<ContentFreshnessState>,
    /// Client scopes admitted by this object.
    pub client_scopes: Vec<ContentClientScope>,
    /// Hosting boundaries admitted by this object.
    pub hosting_boundaries: Vec<ContentHostingBoundary>,
    /// Edition labels admitted by this object.
    pub edition_labels: Vec<ContentEditionLabel>,
    /// Evidence requirement level.
    pub evidence_requirement: M5ContentEvidenceRequirement,
    /// Proof packet refs that keep this object current.
    pub required_proof_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this object.
    pub downgrade_triggers: Vec<M5ContentDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5ContentRollbackPosture,
    /// Source contract refs consumed by this object.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this object's qualification.
    pub consumer_surfaces: Vec<M5ContentConsumerSurface>,
}

impl M5ContentObjectRow {
    /// Returns true when the row declares the given vocabulary.
    fn declares(&self, vocab: M5ContentStateVocabulary) -> bool {
        self.state_vocabularies.contains(&vocab)
    }

    /// Returns true when the token vec for `vocab` is non-empty.
    fn vocab_tokens_present(&self, vocab: M5ContentStateVocabulary) -> bool {
        use M5ContentStateVocabulary as V;
        match vocab {
            V::LifecycleState => !self.lifecycle_states.is_empty(),
            V::TrustClass => !self.trust_classes.is_empty(),
            V::PolicyState => !self.policy_states.is_empty(),
            V::CompatibilityState => !self.compatibility_states.is_empty(),
            V::FreshnessState => !self.freshness_states.is_empty(),
            V::ClientScope => !self.client_scopes.is_empty(),
            V::HostingBoundary => !self.hosting_boundaries.is_empty(),
            V::EditionLabel => !self.edition_labels.is_empty(),
        }
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
///
/// Each field lists every canonical token for one controlled vocabulary, in
/// declaration order. The matrix validates each list against the typed `ALL`
/// arrays so the frozen vocabulary cannot silently drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentVocabularySet {
    /// Lifecycle-state tokens.
    pub lifecycle_states: Vec<String>,
    /// Trust-class tokens.
    pub trust_classes: Vec<String>,
    /// Policy-state tokens.
    pub policy_states: Vec<String>,
    /// Compatibility-state tokens.
    pub compatibility_states: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Client-scope tokens.
    pub client_scopes: Vec<String>,
    /// Hosting-boundary tokens.
    pub hosting_boundaries: Vec<String>,
    /// Edition-label tokens.
    pub edition_labels: Vec<String>,
}

impl M5ContentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            lifecycle_states: ContentLifecycleState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            trust_classes: ContentTrustClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            policy_states: ContentPolicyState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            compatibility_states: ContentCompatibilityState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            freshness_states: ContentFreshnessState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            client_scopes: ContentClientScope::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            hosting_boundaries: ContentHostingBoundary::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            edition_labels: ContentEditionLabel::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Trust and wording-honesty review block.
///
/// Every flag is a hard invariant; all must hold for the matrix to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentTrustReview {
    /// Safety-critical strings use stable message ids.
    pub safety_critical_strings_use_stable_ids: bool,
    /// Safety-critical strings use controlled terms.
    pub safety_critical_strings_use_controlled_terms: bool,
    /// Action labels and counts stay scope-honest.
    pub action_labels_and_counts_scope_honest: bool,
    /// Error copy explains failure, remaining capability, and the next action.
    pub error_copy_explains_failure_remaining_capability_and_next_action: bool,
    /// AI wording never overstates confidence or autonomy.
    pub ai_wording_never_overstates_confidence_or_autonomy: bool,
    /// Content-ops artifacts keep version/source metadata.
    pub content_ops_artifacts_keep_version_and_source_metadata: bool,
    /// Commercial-boundary wording matches the actual product boundary.
    pub commercial_boundary_wording_matches_product_boundary: bool,
    /// Controlled terms are never softened for tone.
    pub controlled_terms_never_softened_for_tone: bool,
    /// Every surface points at one controlled-term inventory, not parallel copy
    /// lists.
    pub one_controlled_term_inventory_not_parallel_copy_lists: bool,
    /// No speculative brand-refresh or marketing-campaign work is in scope.
    pub no_speculative_brand_or_marketing_campaign_scope: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified objects automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentConsumerProjection {
    /// Product UI consumes the shared object model.
    pub product_ui_consumes_object_model: bool,
    /// CLI / help text shows controlled terms.
    pub cli_help_shows_controlled_terms: bool,
    /// Docs show content-ops metadata.
    pub docs_shows_content_ops_metadata: bool,
    /// Support export shows the shared object model.
    pub support_export_shows_object_model: bool,
    /// Release notes use the controlled vocabulary.
    pub release_notes_use_controlled_vocabulary: bool,
    /// Screenshots and demos carry version/source metadata.
    pub screenshots_demos_carry_version_source_metadata: bool,
    /// AI surfaces honor the copy guardrails.
    pub ai_surfaces_honor_copy_guardrails: bool,
    /// Onboarding uses controlled terms.
    pub onboarding_uses_controlled_terms: bool,
    /// Help / About shows commercial-boundary truth.
    pub help_about_shows_commercial_boundary_truth: bool,
    /// Preview / Labs surfaces are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_objects: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the object.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the content-wording lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet for the lane.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every object.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every object.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`M5ContentWordingMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContentWordingMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5ContentObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ContentVocabularySet,
    /// Trust review block.
    pub trust_review: M5ContentTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContentProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5ContentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 content-wording matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentWordingMatrixPacket {
    /// Record kind; must equal [`M5_CONTENT_WORDING_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONTENT_WORDING_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5ContentObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ContentVocabularySet,
    /// Trust review block.
    pub trust_review: M5ContentTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContentProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5ContentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ContentWordingMatrixPacket {
    /// Builds an M5 content-wording matrix packet from stable-lane input.
    pub fn new(input: M5ContentWordingMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CONTENT_WORDING_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CONTENT_WORDING_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            object_rows: input.object_rows,
            vocabulary_set: input.vocabulary_set,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 content-wording matrix invariants.
    pub fn validate(&self) -> Vec<M5ContentWordingMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CONTENT_WORDING_MATRIX_RECORD_KIND {
            violations.push(M5ContentWordingMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONTENT_WORDING_MATRIX_SCHEMA_VERSION {
            violations.push(M5ContentWordingMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ContentWordingMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_object_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 content-wording matrix packet serializes"),
        ) {
            violations.push(M5ContentWordingMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 content-wording matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .object_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Content-Design, Controlled-Vocabulary, Content-Ops, and Commercial-Boundary Wording Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Objects: {} ({} stable)\n",
            self.object_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Objects\n\n");
        for row in &self.object_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.object_kind.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Vocabularies: {}\n",
                row.state_vocabularies
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 content-wording matrix export.
#[derive(Debug)]
pub enum M5ContentWordingMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ContentWordingMatrixViolation>),
}

impl fmt::Display for M5ContentWordingMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 content-wording matrix export parse failed: {error}"
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
                    "m5 content-wording matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ContentWordingMatrixArtifactError {}

/// Validation failures emitted by [`M5ContentWordingMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ContentWordingMatrixViolation {
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
    /// A required governed object is missing from the matrix.
    RequiredObjectMissing,
    /// An object row is incomplete.
    ObjectRowIncomplete,
    /// An object row omits a vocabulary its kind requires.
    RequiredVocabularyMissing,
    /// A declared vocabulary has no concrete tokens.
    DeclaredVocabularyHasNoTokens,
    /// A token vec is populated for a vocabulary the row does not declare.
    UndeclaredVocabularyHasTokens,
    /// An object claiming Stable is missing required proof packet refs.
    StableObjectMissingProof,
    /// An object has no downgrade triggers.
    DowngradeTriggersMissing,
    /// An object has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ContentWordingMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ObjectRowIncomplete => "object_row_incomplete",
            Self::RequiredVocabularyMissing => "required_vocabulary_missing",
            Self::DeclaredVocabularyHasNoTokens => "declared_vocabulary_has_no_tokens",
            Self::UndeclaredVocabularyHasTokens => "undeclared_vocabulary_has_tokens",
            Self::StableObjectMissingProof => "stable_object_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 content-wording matrix export.
pub fn current_stable_m5_content_wording_matrix_export(
) -> Result<M5ContentWordingMatrixPacket, M5ContentWordingMatrixArtifactError> {
    let packet: M5ContentWordingMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/support_export.json"
    )))
    .map_err(M5ContentWordingMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ContentWordingMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CONTENT_WORDING_MATRIX_SCHEMA_REF,
        M5_CONTENT_WORDING_MATRIX_DOC_REF,
        M5_CONTENT_UI_COPY_CONTRACT_REF,
        M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
        M5_CONTENT_COUNT_SCOPE_GRAMMAR_REF,
        M5_CONTENT_CONTENT_OPS_CONTRACT_REF,
        M5_CONTENT_AI_COPY_GUARDRAILS_CONTRACT_REF,
        M5_CONTENT_CONTROLLED_GLOSSARY_REF,
        M5_CONTENT_PRODUCT_TRUTH_VOCABULARY_REF,
        M5_CONTENT_DEPLOYMENT_PROFILES_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ContentWordingMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ContentWordingMatrixViolation::VocabularySetDrift);
    }
}

fn validate_object_rows(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    let present: BTreeSet<M5ContentObjectKind> = packet
        .object_rows
        .iter()
        .map(|row| row.object_kind)
        .collect();
    for required in M5ContentObjectKind::ALL {
        if !present.contains(&required) {
            violations.push(M5ContentWordingMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.object_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.required_fields.is_empty()
            || row.state_vocabularies.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5ContentWordingMatrixViolation::ObjectRowIncomplete);
        }

        for required_vocab in row.object_kind.required_state_vocabularies() {
            if !row.declares(*required_vocab) {
                violations.push(M5ContentWordingMatrixViolation::RequiredVocabularyMissing);
            }
        }

        for vocab in M5ContentStateVocabulary::ALL {
            let declared = row.declares(vocab);
            let has_tokens = row.vocab_tokens_present(vocab);
            if declared && !has_tokens {
                violations.push(M5ContentWordingMatrixViolation::DeclaredVocabularyHasNoTokens);
            }
            if !declared && has_tokens {
                violations.push(M5ContentWordingMatrixViolation::UndeclaredVocabularyHasTokens);
            }
        }

        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ContentWordingMatrixViolation::StableObjectMissingProof);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ContentWordingMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ContentWordingMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.safety_critical_strings_use_stable_ids,
        review.safety_critical_strings_use_controlled_terms,
        review.action_labels_and_counts_scope_honest,
        review.error_copy_explains_failure_remaining_capability_and_next_action,
        review.ai_wording_never_overstates_confidence_or_autonomy,
        review.content_ops_artifacts_keep_version_and_source_metadata,
        review.commercial_boundary_wording_matches_product_boundary,
        review.controlled_terms_never_softened_for_tone,
        review.one_controlled_term_inventory_not_parallel_copy_lists,
        review.no_speculative_brand_or_marketing_campaign_scope,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5ContentWordingMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.product_ui_consumes_object_model,
        projection.cli_help_shows_controlled_terms,
        projection.docs_shows_content_ops_metadata,
        projection.support_export_shows_object_model,
        projection.release_notes_use_controlled_vocabulary,
        projection.screenshots_demos_carry_version_source_metadata,
        projection.ai_surfaces_honor_copy_guardrails,
        projection.onboarding_uses_controlled_terms,
        projection.help_about_shows_commercial_boundary_truth,
        projection.preview_labs_label_for_unqualified_objects,
    ] {
        if !ok {
            violations.push(M5ContentWordingMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ContentWordingMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ContentWordingMatrixPacket,
    violations: &mut Vec<M5ContentWordingMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(M5ContentWordingMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
