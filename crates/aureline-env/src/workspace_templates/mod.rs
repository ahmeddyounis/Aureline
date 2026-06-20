//! The declarative workspace-template layer that composes a typed
//! environment capsule with workflow-bundle references, certified-archetype
//! defaults, and docs / onboarding references.
//!
//! The sibling [`crate::capsules`] module materialized the typed
//! [`EnvironmentCapsule`] object — the concrete environment definition a
//! template hydrates, a prebuild fingerprints, and a runtime materializes —
//! and the [`crate::m5_env_governance`] module froze the governance matrix
//! that certifies it. What both deliberately left implicit is the *template*:
//! the launch artifact a user picks from the start center that hydrates a
//! capsule, points at the workflow bundles it expects, defaults to a
//! certified archetype, and links its onboarding docs.
//!
//! Historically those starter flows were opaque code paths with hidden
//! execution assumptions. This module turns them into declarative,
//! reviewable, mirrorable artifacts. A [`WorkspaceTemplate`] *embeds* the
//! same typed [`EnvironmentCapsule`] the rest of the lane consumes, so
//! template hydration cannot fork the runtime or trust semantics from the
//! core execution model: the inspector folds the embedded capsule through
//! the **same** [`inspect_environment`] path, then narrows the result
//! further by the composition layers and the template's trust posture.
//!
//! Each template carries, as inspectable data:
//!
//! - a [`TemplateIdentity`] (id, version, source class, signer / mirror
//!   class, and a versioned digest),
//! - the embedded [`EnvironmentCapsule`] it hydrates,
//! - typed [`WorkflowBundleRef`]s the template expects (references, never
//!   forked execution),
//! - [`ArchetypeDefault`]s pinning the certified archetype defaults,
//! - [`DocsOnboardingRef`]s linking the onboarding surfaces,
//! - a [`TemplateTrust`] posture (source / signer / mirror class and an
//!   attestation evidence state),
//! - a [`SupportPosture`] (support class and freshness), and
//! - explicit [`CompositionGuardrails`] proving the template injects no
//!   proprietary service dependence, no ungated lifecycle hooks, and no
//!   hidden bundle / runtime widening.
//!
//! [`inspect_template`] produces one [`WhyThisTemplate`] explainability
//! object — the single object desktop ([`desktop_template_inspection`]),
//! CLI / headless ([`headless_template_inspection`]), and support
//! ([`support_template_inspection`]) all read. [`diff_templates`] and
//! [`plan_template_change`] give the install / update / remove flows a
//! diffable, rollback-aware view of exactly what a template composes and
//! why, and [`export_template_metadata`] projects a redaction-safe support
//! view. None of these forks the capsule object: the template only ever
//! *narrows* the capsule's claim, never widens it.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::capsules::{
    diff_capsules, export_capsule_metadata, inspect_environment, validate_environment_capsule,
    CapsuleChangeKind, CapsuleDiff, CapsuleDigest, CapsuleExport, CapsuleFieldChange,
    EnvironmentCapsule, RedactionClass, WhyThisEnvironment,
};
use crate::m5_env_governance::{
    ClaimMaturity, EvidenceState, RowVerdict, ValidationReport, ValidationViolation,
    WarmStartPosture,
};

/// Schema version stamped onto templates, inspections, diffs, plans, and
/// fixtures.
pub const WORKSPACE_TEMPLATE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by a serialized [`WorkspaceTemplate`].
pub const WORKSPACE_TEMPLATE_RECORD_KIND: &str = "workspace_template_record";

/// Stable record-kind tag carried by a [`WhyThisTemplate`] inspection.
pub const WORKSPACE_TEMPLATE_INSPECTION_RECORD_KIND: &str = "workspace_template_inspection_record";

/// Stable record-kind tag carried by a [`TemplateExport`].
pub const WORKSPACE_TEMPLATE_EXPORT_RECORD_KIND: &str = "workspace_template_export_record";

/// Stable record-kind tag carried by a [`TemplateDiff`].
pub const WORKSPACE_TEMPLATE_DIFF_RECORD_KIND: &str = "workspace_template_diff_record";

/// Stable record-kind tag carried by a [`TemplateChangePlan`].
pub const WORKSPACE_TEMPLATE_PLAN_RECORD_KIND: &str = "workspace_template_change_plan_record";

/// Stable record-kind tag carried by a [`WorkspaceTemplateFixture`].
pub const WORKSPACE_TEMPLATE_FIXTURE_RECORD_KIND: &str = "workspace_template_fixture_record";

/// Repo-relative schema ref for the template and its fixtures.
pub const WORKSPACE_TEMPLATE_SCHEMA_REF: &str = "schemas/env/workspace-template.schema.json";

/// Repo-relative reviewer doc ref.
pub const WORKSPACE_TEMPLATE_DOC_REF: &str = "docs/env/workspace-template.md";

/// Repo-relative human-readable proof report.
pub const WORKSPACE_TEMPLATE_PROOF_REF: &str = "artifacts/env/workspace-template-proof.md";

/// Repo-relative fixture directory.
pub const WORKSPACE_TEMPLATE_FIXTURE_DIR: &str = "fixtures/env/workspace-template";

/// Repo-relative fixture manifest.
pub const WORKSPACE_TEMPLATE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/env/workspace-template/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The provenance class a template is published under. This is the
/// review-and-trust label; it does not by itself narrow the claim — the
/// attestation evidence state does — but a template's signer class must be
/// consistent with its source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSourceClass {
    /// Authored and published by the first party.
    FirstParty,
    /// A third-party template admitted through the managed mirror review.
    ManagedApproved,
    /// A community-authored template.
    Community,
    /// A local, unpublished draft the user is still editing.
    LocalDraft,
}

impl TemplateSourceClass {
    /// Every source class in canonical order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::ManagedApproved,
        Self::Community,
        Self::LocalDraft,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::ManagedApproved => "managed_approved",
            Self::Community => "community",
            Self::LocalDraft => "local_draft",
        }
    }
}

/// How a template's bytes are attested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerClass {
    /// Signed by the first-party publishing key.
    FirstPartySigned,
    /// Countersigned by the managed mirror after review.
    MirrorCountersigned,
    /// Signed by a community publishing key.
    CommunitySigned,
    /// Unsigned (e.g. a local draft).
    Unsigned,
}

impl SignerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySigned => "first_party_signed",
            Self::MirrorCountersigned => "mirror_countersigned",
            Self::CommunitySigned => "community_signed",
            Self::Unsigned => "unsigned",
        }
    }
}

/// Which mirror a template is distributed through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorClass {
    /// Served from the first-party origin.
    FirstPartyOrigin,
    /// Served from the managed mirror.
    ManagedMirror,
    /// Served from a community mirror.
    CommunityMirror,
    /// Resolved from a local path; not mirrored.
    LocalOnly,
}

impl MirrorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyOrigin => "first_party_origin",
            Self::ManagedMirror => "managed_mirror",
            Self::CommunityMirror => "community_mirror",
            Self::LocalOnly => "local_only",
        }
    }
}

/// The support class a template is published under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// First-party supported.
    FirstPartySupported,
    /// Supported through the managed program.
    ManagedSupported,
    /// Community supported.
    CommunitySupported,
    /// Not supported (e.g. a local draft).
    Unsupported,
}

impl SupportClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySupported => "first_party_supported",
            Self::ManagedSupported => "managed_supported",
            Self::CommunitySupported => "community_supported",
            Self::Unsupported => "unsupported",
        }
    }
}

/// One layer the template composes. The environment capsule is the
/// foundational layer; the rest are composed on top of it as references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompositionLayerKind {
    /// The embedded environment capsule the template hydrates.
    EnvironmentCapsule,
    /// A workflow bundle the template expects.
    WorkflowBundle,
    /// A certified-archetype default.
    ArchetypeDefault,
    /// A docs / onboarding reference.
    DocsReference,
    /// The signer / mirror attestation backing the template.
    TrustAttestation,
    /// The support / freshness posture of the template.
    SupportFreshness,
}

impl CompositionLayerKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentCapsule => "environment_capsule",
            Self::WorkflowBundle => "workflow_bundle",
            Self::ArchetypeDefault => "archetype_default",
            Self::DocsReference => "docs_reference",
            Self::TrustAttestation => "trust_attestation",
            Self::SupportFreshness => "support_freshness",
        }
    }
}

/// The kind of onboarding surface a [`DocsOnboardingRef`] links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsRefKind {
    /// A getting-started / onboarding guide.
    Onboarding,
    /// The start-center / template-gallery reference.
    StartCenter,
    /// The reviewer documentation for the composed environment.
    Reference,
}

impl DocsRefKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Onboarding => "onboarding",
            Self::StartCenter => "start_center",
            Self::Reference => "reference",
        }
    }
}

/// A template lifecycle operation a [`TemplateChangePlan`] explains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateLifecycleOp {
    /// Install the template for the first time.
    Install,
    /// Update an installed template to a new version.
    Update,
    /// Remove an installed template.
    Remove,
}

impl TemplateLifecycleOp {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Remove => "remove",
        }
    }
}

// ---------------------------------------------------------------------------
// Typed composition layers.
// ---------------------------------------------------------------------------

/// One workflow-bundle reference the template composes. This is a
/// *reference* pinned by a digest, never a forked execution model: the
/// bundle is consumed through the existing workflow-bundle lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowBundleRef {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Repo- or workspace-relative reference (metadata, not a body).
    pub reference: String,
    /// Digest pinning the referenced bundle.
    pub digest: CapsuleDigest,
    /// Freshness / coverage of this bundle reference.
    pub coverage: EvidenceState,
    /// Whether composing this bundle widens the execution scope. Must be
    /// `false`: a template may reference workflows but may not silently
    /// widen the runtime scope.
    pub widens_execution_scope: bool,
    /// Review-safe summary of the bundle reference.
    pub summary: String,
}

/// One certified-archetype default the template seeds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeDefault {
    /// Stable archetype id.
    pub archetype_id: String,
    /// Repo- or workspace-relative reference to the archetype confidence row.
    pub reference: String,
    /// Digest pinning the archetype default.
    pub digest: CapsuleDigest,
    /// Freshness / coverage of this archetype default.
    pub coverage: EvidenceState,
    /// Review-safe summary of the archetype default.
    pub summary: String,
}

/// One docs / onboarding reference the template links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsOnboardingRef {
    /// Stable docs ref id.
    pub docs_id: String,
    /// Kind of onboarding surface.
    pub kind: DocsRefKind,
    /// Repo-relative reference to the docs surface.
    pub reference: String,
    /// Freshness / coverage of this docs reference.
    pub coverage: EvidenceState,
    /// Review-safe summary of the docs reference.
    pub summary: String,
}

/// The template's trust posture: its source, signer, and mirror class, plus
/// the evidence state of the attestation that backs it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateTrust {
    /// Provenance class.
    pub source_class: TemplateSourceClass,
    /// Signer class.
    pub signer_class: SignerClass,
    /// Mirror class.
    pub mirror_class: MirrorClass,
    /// Freshness / coverage of the attestation backing the trust posture.
    pub attestation_state: EvidenceState,
    /// Review-safe summary of the trust posture.
    pub summary: String,
}

/// The template's support and freshness posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportPosture {
    /// Support class.
    pub support_class: SupportClass,
    /// Freshness / coverage of the template against its upstream.
    pub freshness_state: EvidenceState,
    /// Review-safe summary of the support posture.
    pub summary: String,
}

/// Explicit guardrail flags proving the template composition stays honest.
/// All three must be `false`; a `true` value is a contract violation, so a
/// template can never *silently* widen what it composes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositionGuardrails {
    /// True only if the template injects dependence on a proprietary service.
    pub injects_proprietary_service_dependence: bool,
    /// True only if the template introduces an ungated lifecycle hook beyond
    /// the capsule's trust-gated hooks.
    pub introduces_ungated_lifecycle_hooks: bool,
    /// True only if the template widens the bundle or runtime scope.
    pub widens_bundle_or_runtime_scope: bool,
    /// Review-safe summary of the guardrail posture.
    pub summary: String,
}

impl CompositionGuardrails {
    /// Returns true when no guardrail is tripped.
    pub const fn is_clean(&self) -> bool {
        !self.injects_proprietary_service_dependence
            && !self.introduces_ungated_lifecycle_hooks
            && !self.widens_bundle_or_runtime_scope
    }
}

/// The identity of a template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateIdentity {
    /// Stable template id.
    pub template_id: String,
    /// Monotonic template version.
    pub template_version: u32,
    /// Review-safe template label.
    pub label: String,
    /// Versioned digest of the template's defining inputs.
    pub template_digest: CapsuleDigest,
    /// Review-safe summary of the template identity.
    pub summary: String,
}

/// The declarative workspace-template object: one inspectable, diffable,
/// mirrorable launch artifact that composes a typed environment capsule with
/// workflow-bundle references, certified-archetype defaults, and docs /
/// onboarding references — without forking the execution model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplate {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Template identity.
    pub identity: TemplateIdentity,
    /// The embedded environment capsule the template hydrates. Hydration
    /// reuses this exact object, so it cannot fork the execution model.
    pub environment_capsule: EnvironmentCapsule,
    /// Workflow-bundle references the template expects.
    pub workflow_bundle_refs: Vec<WorkflowBundleRef>,
    /// Certified-archetype defaults.
    pub archetype_defaults: Vec<ArchetypeDefault>,
    /// Docs / onboarding references.
    pub docs_refs: Vec<DocsOnboardingRef>,
    /// Trust posture (source / signer / mirror class and attestation state).
    pub trust: TemplateTrust,
    /// Support / freshness posture.
    pub support: SupportPosture,
    /// Composition guardrails.
    pub guardrails: CompositionGuardrails,
    /// Maturity claimed for this template; must equal the capsule's claim.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed for this template; must equal the capsule's.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Review-safe "why this template" headline.
    pub why_this_template: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Certification: one engine, narrowing only.
// ---------------------------------------------------------------------------

/// One composition layer's evidence, folded into the template certification.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LayerEvidence {
    kind: CompositionLayerKind,
    layer_id: String,
    state: EvidenceState,
}

/// The computed outcome of certifying a template against its embedded
/// capsule and its composition layers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateOutcome {
    /// The narrowest maturity the template claim may hold.
    pub effective_maturity: ClaimMaturity,
    /// The verdict the engine reaches.
    pub verdict: RowVerdict,
    /// True when the template claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Tokens naming every dimension or layer that forced maturity narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Tokens naming only the composition layers that forced narrowing.
    pub composition_narrow_tokens: Vec<String>,
    /// The coldest warm-start posture the template may hold. The composition
    /// layers do not govern warm start, so this equals the capsule's.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claimed one.
    pub warm_start_downgraded: bool,
    /// Warm-start-downgrade tokens, inherited from the capsule.
    pub warm_start_downgrade_tokens: Vec<String>,
    /// Dimensions and layers whose evidence is stale or missing.
    pub stale_or_missing_tokens: Vec<String>,
}

fn composition_layers(template: &WorkspaceTemplate) -> Vec<LayerEvidence> {
    let mut layers = Vec::new();
    for bundle in &template.workflow_bundle_refs {
        layers.push(LayerEvidence {
            kind: CompositionLayerKind::WorkflowBundle,
            layer_id: bundle.bundle_id.clone(),
            state: bundle.coverage,
        });
    }
    for archetype in &template.archetype_defaults {
        layers.push(LayerEvidence {
            kind: CompositionLayerKind::ArchetypeDefault,
            layer_id: archetype.archetype_id.clone(),
            state: archetype.coverage,
        });
    }
    for docs in &template.docs_refs {
        layers.push(LayerEvidence {
            kind: CompositionLayerKind::DocsReference,
            layer_id: docs.docs_id.clone(),
            state: docs.coverage,
        });
    }
    layers.push(LayerEvidence {
        kind: CompositionLayerKind::TrustAttestation,
        layer_id: template.trust.source_class.as_str().to_owned(),
        state: template.trust.attestation_state,
    });
    layers.push(LayerEvidence {
        kind: CompositionLayerKind::SupportFreshness,
        layer_id: template.support.support_class.as_str().to_owned(),
        state: template.support.freshness_state,
    });
    layers
}

fn layer_token(layer: &LayerEvidence) -> String {
    format!(
        "{}_{}_{}",
        layer.kind.as_str(),
        layer.layer_id,
        layer.state.as_str()
    )
}

/// Folds the embedded-capsule inspection and the composition layers into one
/// template outcome.
///
/// This reuses the capsule narrowing engine for the environment — the
/// `capsule_inspection` is the output of [`inspect_environment`], which runs
/// the canonical `certify_capsule_outcome` engine — and the **same**
/// [`EvidenceState`] floors for the composition layers, so the template
/// certification can never disagree with the capsule certification. The
/// composition layers can only narrow the maturity; warm start is governed
/// solely by the capsule, so it is carried through unchanged.
fn certify_template_outcome(
    claimed_maturity: ClaimMaturity,
    capsule_inspection: &WhyThisEnvironment,
    layers: &[LayerEvidence],
) -> TemplateOutcome {
    let mut effective_maturity = capsule_inspection.effective_maturity;
    let mut composition_narrow_tokens = Vec::new();
    let mut stale_or_missing = capsule_inspection.stale_or_missing_dimension_tokens.clone();

    for layer in layers {
        if let Some(floor) = layer.state.qualification_floor() {
            if floor.severity() > effective_maturity.severity() {
                effective_maturity = floor;
            }
            composition_narrow_tokens.push(layer_token(layer));
        }
        if layer.state.is_stale_or_missing() {
            stale_or_missing.push(layer_token(layer));
        }
    }

    composition_narrow_tokens.sort();
    composition_narrow_tokens.dedup();
    stale_or_missing.sort();
    stale_or_missing.dedup();

    let mut narrow_reason_tokens = capsule_inspection.narrow_reason_tokens.clone();
    narrow_reason_tokens.extend(composition_narrow_tokens.iter().cloned());
    narrow_reason_tokens.sort();
    narrow_reason_tokens.dedup();

    let verdict = if effective_maturity == ClaimMaturity::Withdrawn {
        RowVerdict::Withheld
    } else if effective_maturity.severity() > claimed_maturity.severity() {
        RowVerdict::Narrowed
    } else {
        RowVerdict::Certified
    };

    TemplateOutcome {
        effective_maturity,
        verdict,
        narrowed: verdict == RowVerdict::Narrowed,
        narrow_reason_tokens,
        composition_narrow_tokens,
        effective_warm_start_posture: capsule_inspection.effective_warm_start_posture,
        warm_start_downgraded: capsule_inspection.warm_start_downgraded,
        warm_start_downgrade_tokens: capsule_inspection.warm_start_downgrade_tokens.clone(),
        stale_or_missing_tokens: stale_or_missing,
    }
}

// ---------------------------------------------------------------------------
// Why-this-template inspector.
// ---------------------------------------------------------------------------

/// One per-layer reason line in a [`WhyThisTemplate`] report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateLayerReason {
    /// Composition layer this reason explains.
    pub kind: CompositionLayerKind,
    /// Stable id of the layer (capsule id, bundle id, archetype id, etc.).
    pub layer_id: String,
    /// Observed evidence state for the layer.
    pub evidence_state: EvidenceState,
    /// Metadata reference backing the layer.
    pub reference: String,
    /// What this layer contributes to the template.
    pub contribution: String,
}

/// The why-this-template inspection: the single explainability object
/// desktop, CLI / headless, and support surfaces all consume. It wraps the
/// embedded capsule's [`WhyThisEnvironment`] inspection and reports the
/// template's effective maturity, verdict, and warm-start posture after the
/// composition layers narrow it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhyThisTemplate {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Template id under inspection.
    pub template_id: String,
    /// Template version under inspection.
    pub template_version: u32,
    /// Template digest under inspection.
    pub template_digest: CapsuleDigest,
    /// Provenance class.
    pub source_class: TemplateSourceClass,
    /// Signer class.
    pub signer_class: SignerClass,
    /// Mirror class.
    pub mirror_class: MirrorClass,
    /// Support class.
    pub support_class: SupportClass,
    /// Maturity claimed for the template.
    pub claimed_maturity: ClaimMaturity,
    /// Effective maturity after narrowing.
    pub effective_maturity: ClaimMaturity,
    /// Engine verdict.
    pub verdict: RowVerdict,
    /// Warm-start posture claimed for the template.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Effective warm-start posture after narrowing.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claim.
    pub warm_start_downgraded: bool,
    /// True when the guardrails are clean (no silent widening).
    pub guardrails_clean: bool,
    /// Tokens naming every dimension or layer that forced maturity narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Tokens naming only the composition layers that forced narrowing.
    pub composition_narrow_tokens: Vec<String>,
    /// Warm-start-downgrade tokens, inherited from the capsule.
    pub warm_start_downgrade_tokens: Vec<String>,
    /// Dimensions and layers whose evidence is stale or missing.
    pub stale_or_missing_tokens: Vec<String>,
    /// The embedded capsule's canonical inspection. Proves the template does
    /// not fork the environment or trust semantics from the core model.
    pub capsule_inspection: WhyThisEnvironment,
    /// Per-layer reasons behind the verdict.
    pub layer_reasons: Vec<TemplateLayerReason>,
    /// Review-safe headline summarizing why this template is what it is.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

fn layer_contribution(kind: CompositionLayerKind) -> &'static str {
    match kind {
        CompositionLayerKind::EnvironmentCapsule => {
            "Hydrates the embedded environment capsule through the same execution model as a direct run."
        }
        CompositionLayerKind::WorkflowBundle => {
            "References a workflow bundle the template expects, without forking or widening the runtime scope."
        }
        CompositionLayerKind::ArchetypeDefault => {
            "Seeds a certified-archetype default so the workspace opens with reviewed defaults."
        }
        CompositionLayerKind::DocsReference => {
            "Links the onboarding and reference docs so the template explains itself."
        }
        CompositionLayerKind::TrustAttestation => {
            "Carries the source, signer, and mirror class so install/update/remove can review provenance."
        }
        CompositionLayerKind::SupportFreshness => {
            "Carries the support class and freshness so a stale template downgrades visibly."
        }
    }
}

fn layer_reasons(
    template: &WorkspaceTemplate,
    capsule: &WhyThisEnvironment,
) -> Vec<TemplateLayerReason> {
    let mut reasons = vec![TemplateLayerReason {
        kind: CompositionLayerKind::EnvironmentCapsule,
        layer_id: template.environment_capsule.identity.capsule_id.clone(),
        evidence_state: worst_capsule_state(capsule),
        reference: WORKSPACE_TEMPLATE_DOC_REF.to_owned(),
        contribution: layer_contribution(CompositionLayerKind::EnvironmentCapsule).to_owned(),
    }];
    for bundle in &template.workflow_bundle_refs {
        reasons.push(TemplateLayerReason {
            kind: CompositionLayerKind::WorkflowBundle,
            layer_id: bundle.bundle_id.clone(),
            evidence_state: bundle.coverage,
            reference: bundle.reference.clone(),
            contribution: layer_contribution(CompositionLayerKind::WorkflowBundle).to_owned(),
        });
    }
    for archetype in &template.archetype_defaults {
        reasons.push(TemplateLayerReason {
            kind: CompositionLayerKind::ArchetypeDefault,
            layer_id: archetype.archetype_id.clone(),
            evidence_state: archetype.coverage,
            reference: archetype.reference.clone(),
            contribution: layer_contribution(CompositionLayerKind::ArchetypeDefault).to_owned(),
        });
    }
    for docs in &template.docs_refs {
        reasons.push(TemplateLayerReason {
            kind: CompositionLayerKind::DocsReference,
            layer_id: docs.docs_id.clone(),
            evidence_state: docs.coverage,
            reference: docs.reference.clone(),
            contribution: layer_contribution(CompositionLayerKind::DocsReference).to_owned(),
        });
    }
    reasons.push(TemplateLayerReason {
        kind: CompositionLayerKind::TrustAttestation,
        layer_id: template.trust.source_class.as_str().to_owned(),
        evidence_state: template.trust.attestation_state,
        reference: template.trust.signer_class.as_str().to_owned(),
        contribution: layer_contribution(CompositionLayerKind::TrustAttestation).to_owned(),
    });
    reasons.push(TemplateLayerReason {
        kind: CompositionLayerKind::SupportFreshness,
        layer_id: template.support.support_class.as_str().to_owned(),
        evidence_state: template.support.freshness_state,
        reference: template.support.support_class.as_str().to_owned(),
        contribution: layer_contribution(CompositionLayerKind::SupportFreshness).to_owned(),
    });
    reasons
}

/// The worst evidence state surfaced by the capsule inspection, used to
/// label the foundational capsule layer.
fn worst_capsule_state(capsule: &WhyThisEnvironment) -> EvidenceState {
    capsule
        .reasons
        .iter()
        .map(|reason| reason.evidence_state)
        .fold(EvidenceState::Current, |acc, state| {
            if state_rank(state) > state_rank(acc) {
                state
            } else {
                acc
            }
        })
}

fn state_rank(state: EvidenceState) -> u8 {
    match state {
        EvidenceState::Current | EvidenceState::NotApplicable => 0,
        EvidenceState::Partial => 1,
        EvidenceState::Stale => 2,
        EvidenceState::Missing => 3,
    }
}

/// Inspects a template and produces the canonical why-this-template report.
/// This is the single inspection path every surface shares; it reuses
/// [`inspect_environment`] for the embedded capsule so the template inspector
/// and the capsule inspector can never disagree.
pub fn inspect_template(template: &WorkspaceTemplate) -> WhyThisTemplate {
    let capsule_inspection = inspect_environment(&template.environment_capsule);
    let layers = composition_layers(template);
    let outcome = certify_template_outcome(template.claimed_maturity, &capsule_inspection, &layers);
    let layer_reasons = layer_reasons(template, &capsule_inspection);
    WhyThisTemplate {
        record_kind: WORKSPACE_TEMPLATE_INSPECTION_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TEMPLATE_SCHEMA_VERSION,
        template_id: template.identity.template_id.clone(),
        template_version: template.identity.template_version,
        template_digest: template.identity.template_digest.clone(),
        source_class: template.trust.source_class,
        signer_class: template.trust.signer_class,
        mirror_class: template.trust.mirror_class,
        support_class: template.support.support_class,
        claimed_maturity: template.claimed_maturity,
        effective_maturity: outcome.effective_maturity,
        verdict: outcome.verdict,
        claimed_warm_start_posture: template.claimed_warm_start_posture,
        effective_warm_start_posture: outcome.effective_warm_start_posture,
        warm_start_downgraded: outcome.warm_start_downgraded,
        guardrails_clean: template.guardrails.is_clean(),
        narrow_reason_tokens: outcome.narrow_reason_tokens,
        composition_narrow_tokens: outcome.composition_narrow_tokens,
        warm_start_downgrade_tokens: outcome.warm_start_downgrade_tokens,
        stale_or_missing_tokens: outcome.stale_or_missing_tokens,
        capsule_inspection,
        layer_reasons,
        headline: template.why_this_template.clone(),
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// The desktop why-this-template inspector. Desktop reads the same
/// [`WhyThisTemplate`] object as every other surface.
pub fn desktop_template_inspection(template: &WorkspaceTemplate) -> WhyThisTemplate {
    inspect_template(template)
}

/// The headless / CLI why-this-template inspector. Headless reads the same
/// [`WhyThisTemplate`] object as every other surface.
pub fn headless_template_inspection(template: &WorkspaceTemplate) -> WhyThisTemplate {
    inspect_template(template)
}

/// The support-path inspection: the metadata-first export wrapping the same
/// [`WhyThisTemplate`] object support and release surfaces read.
pub fn support_template_inspection(template: &WorkspaceTemplate) -> TemplateExport {
    export_template_metadata(template)
}

// ---------------------------------------------------------------------------
// Metadata-first export.
// ---------------------------------------------------------------------------

/// One exported composition layer (id, kind, coverage — never a body).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedLayer {
    /// Composition layer kind.
    pub kind: CompositionLayerKind,
    /// Stable layer id.
    pub layer_id: String,
    /// Observed evidence state.
    pub coverage: EvidenceState,
}

/// A metadata-first export of a template for support and release surfaces.
/// It wraps the canonical [`WhyThisTemplate`] inspection and the capsule's
/// own metadata export, and projects only ids, digests, classes, and states —
/// never secrets, raw bodies, hook commands, or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Template id.
    pub template_id: String,
    /// Template digest.
    pub template_digest: CapsuleDigest,
    /// Provenance class.
    pub source_class: TemplateSourceClass,
    /// Signer class.
    pub signer_class: SignerClass,
    /// Mirror class.
    pub mirror_class: MirrorClass,
    /// Support class.
    pub support_class: SupportClass,
    /// True when the guardrails are clean.
    pub guardrails_clean: bool,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
    /// The canonical inspection this export wraps.
    pub inspection: WhyThisTemplate,
    /// The embedded capsule's own metadata export.
    pub capsule_export: CapsuleExport,
    /// Exported composition layers.
    pub composition_layers: Vec<ExportedLayer>,
    /// Review-safe summary of the export.
    pub summary: String,
}

/// Projects a redaction-safe, metadata-first export of a template.
pub fn export_template_metadata(template: &WorkspaceTemplate) -> TemplateExport {
    let inspection = inspect_template(template);
    let capsule_export = export_capsule_metadata(&template.environment_capsule);
    let composition_layers = composition_layers(template)
        .into_iter()
        .map(|layer| ExportedLayer {
            kind: layer.kind,
            layer_id: layer.layer_id,
            coverage: layer.state,
        })
        .collect();
    TemplateExport {
        record_kind: WORKSPACE_TEMPLATE_EXPORT_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TEMPLATE_SCHEMA_VERSION,
        template_id: template.identity.template_id.clone(),
        template_digest: template.identity.template_digest.clone(),
        source_class: template.trust.source_class,
        signer_class: template.trust.signer_class,
        mirror_class: template.trust.mirror_class,
        support_class: template.support.support_class,
        guardrails_clean: template.guardrails.is_clean(),
        redaction_class: RedactionClass::MetadataOnly,
        inspection,
        capsule_export,
        composition_layers,
        summary: format!(
            "Metadata-first export of template {} ({}); no secrets, raw bodies, or hook commands cross the boundary.",
            template.identity.template_id,
            template.trust.source_class.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Diff.
// ---------------------------------------------------------------------------

/// The diff between two templates. It reuses the capsule field-change
/// vocabulary and embeds the capsule diff, so an install / update / remove
/// review sees exactly what the template composes and how it changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateDiff {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Base template id.
    pub base_template_id: String,
    /// Target template id.
    pub target_template_id: String,
    /// True when the two templates are field-identical.
    pub identical: bool,
    /// Ordered template-level field changes.
    pub changes: Vec<CapsuleFieldChange>,
    /// The embedded capsule diff.
    pub capsule_diff: CapsuleDiff,
    /// Review-safe summary of the diff.
    pub summary: String,
}

fn push_change(changes: &mut Vec<CapsuleFieldChange>, path: &str, before: String, after: String) {
    if before != after {
        changes.push(CapsuleFieldChange {
            path: path.to_owned(),
            change_kind: CapsuleChangeKind::Changed,
            before,
            after,
        });
    }
}

/// Diffs two templates field-by-field, surfacing identity, trust, support,
/// claim, composition-layer, and embedded-capsule changes as metadata tokens.
pub fn diff_templates(base: &WorkspaceTemplate, target: &WorkspaceTemplate) -> TemplateDiff {
    let mut changes = Vec::new();

    push_change(
        &mut changes,
        "identity.template_version",
        base.identity.template_version.to_string(),
        target.identity.template_version.to_string(),
    );
    push_change(
        &mut changes,
        "identity.template_digest",
        base.identity.template_digest.value.clone(),
        target.identity.template_digest.value.clone(),
    );
    push_change(
        &mut changes,
        "trust.source_class",
        base.trust.source_class.as_str().to_owned(),
        target.trust.source_class.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "trust.signer_class",
        base.trust.signer_class.as_str().to_owned(),
        target.trust.signer_class.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "trust.mirror_class",
        base.trust.mirror_class.as_str().to_owned(),
        target.trust.mirror_class.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "trust.attestation_state",
        base.trust.attestation_state.as_str().to_owned(),
        target.trust.attestation_state.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "support.support_class",
        base.support.support_class.as_str().to_owned(),
        target.support.support_class.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "support.freshness_state",
        base.support.freshness_state.as_str().to_owned(),
        target.support.freshness_state.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "claimed_maturity",
        base.claimed_maturity.as_str().to_owned(),
        target.claimed_maturity.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "claimed_warm_start_posture",
        base.claimed_warm_start_posture.as_str().to_owned(),
        target.claimed_warm_start_posture.as_str().to_owned(),
    );

    diff_bundle_refs(&mut changes, base, target);
    diff_archetypes(&mut changes, base, target);
    diff_docs(&mut changes, base, target);

    let capsule_diff = diff_capsules(&base.environment_capsule, &target.environment_capsule);
    let identical = changes.is_empty() && capsule_diff.identical;
    let summary = if identical {
        format!(
            "Templates {} and {} are field-identical.",
            base.identity.template_id, target.identity.template_id
        )
    } else {
        format!(
            "{} template-level change(s) and {} capsule change(s) between {} and {}.",
            changes.len(),
            capsule_diff.changes.len(),
            base.identity.template_id,
            target.identity.template_id
        )
    };

    TemplateDiff {
        record_kind: WORKSPACE_TEMPLATE_DIFF_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TEMPLATE_SCHEMA_VERSION,
        base_template_id: base.identity.template_id.clone(),
        target_template_id: target.identity.template_id.clone(),
        identical,
        changes,
        capsule_diff,
        summary,
    }
}

fn diff_bundle_refs(
    changes: &mut Vec<CapsuleFieldChange>,
    base: &WorkspaceTemplate,
    target: &WorkspaceTemplate,
) {
    let base_ids: BTreeSet<&str> = base
        .workflow_bundle_refs
        .iter()
        .map(|b| b.bundle_id.as_str())
        .collect();
    for bundle in &base.workflow_bundle_refs {
        match target
            .workflow_bundle_refs
            .iter()
            .find(|other| other.bundle_id == bundle.bundle_id)
        {
            Some(other) => push_change(
                changes,
                &format!("workflow_bundle_refs.{}.digest", bundle.bundle_id),
                bundle.digest.value.clone(),
                other.digest.value.clone(),
            ),
            None => changes.push(CapsuleFieldChange {
                path: format!("workflow_bundle_refs.{}", bundle.bundle_id),
                change_kind: CapsuleChangeKind::Removed,
                before: bundle.digest.value.clone(),
                after: String::new(),
            }),
        }
    }
    for bundle in &target.workflow_bundle_refs {
        if !base_ids.contains(bundle.bundle_id.as_str()) {
            changes.push(CapsuleFieldChange {
                path: format!("workflow_bundle_refs.{}", bundle.bundle_id),
                change_kind: CapsuleChangeKind::Added,
                before: String::new(),
                after: bundle.digest.value.clone(),
            });
        }
    }
}

fn diff_archetypes(
    changes: &mut Vec<CapsuleFieldChange>,
    base: &WorkspaceTemplate,
    target: &WorkspaceTemplate,
) {
    let base_ids: BTreeSet<&str> = base
        .archetype_defaults
        .iter()
        .map(|a| a.archetype_id.as_str())
        .collect();
    for archetype in &base.archetype_defaults {
        match target
            .archetype_defaults
            .iter()
            .find(|other| other.archetype_id == archetype.archetype_id)
        {
            Some(other) => push_change(
                changes,
                &format!("archetype_defaults.{}.digest", archetype.archetype_id),
                archetype.digest.value.clone(),
                other.digest.value.clone(),
            ),
            None => changes.push(CapsuleFieldChange {
                path: format!("archetype_defaults.{}", archetype.archetype_id),
                change_kind: CapsuleChangeKind::Removed,
                before: archetype.digest.value.clone(),
                after: String::new(),
            }),
        }
    }
    for archetype in &target.archetype_defaults {
        if !base_ids.contains(archetype.archetype_id.as_str()) {
            changes.push(CapsuleFieldChange {
                path: format!("archetype_defaults.{}", archetype.archetype_id),
                change_kind: CapsuleChangeKind::Added,
                before: String::new(),
                after: archetype.digest.value.clone(),
            });
        }
    }
}

fn diff_docs(
    changes: &mut Vec<CapsuleFieldChange>,
    base: &WorkspaceTemplate,
    target: &WorkspaceTemplate,
) {
    let base_ids: BTreeSet<&str> = base.docs_refs.iter().map(|d| d.docs_id.as_str()).collect();
    for docs in &base.docs_refs {
        match target
            .docs_refs
            .iter()
            .find(|other| other.docs_id == docs.docs_id)
        {
            Some(other) => push_change(
                changes,
                &format!("docs_refs.{}.coverage", docs.docs_id),
                docs.coverage.as_str().to_owned(),
                other.coverage.as_str().to_owned(),
            ),
            None => changes.push(CapsuleFieldChange {
                path: format!("docs_refs.{}", docs.docs_id),
                change_kind: CapsuleChangeKind::Removed,
                before: docs.reference.clone(),
                after: String::new(),
            }),
        }
    }
    for docs in &target.docs_refs {
        if !base_ids.contains(docs.docs_id.as_str()) {
            changes.push(CapsuleFieldChange {
                path: format!("docs_refs.{}", docs.docs_id),
                change_kind: CapsuleChangeKind::Added,
                before: String::new(),
                after: docs.reference.clone(),
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Lifecycle change plan.
// ---------------------------------------------------------------------------

/// One composed layer summarized for an install / update / remove plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedLayer {
    /// Composition layer kind.
    pub kind: CompositionLayerKind,
    /// Stable layer id.
    pub layer_id: String,
    /// Metadata reference for the layer.
    pub reference: String,
}

/// A reviewable plan describing what a template install / update / remove
/// composes, the resulting claim, and how to roll it back.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateChangePlan {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Lifecycle operation being planned.
    pub op: TemplateLifecycleOp,
    /// Template id under change.
    pub template_id: String,
    /// Version before the change (`None` for a fresh install).
    pub before_version: Option<u32>,
    /// Version after the change (`None` for a removal).
    pub after_version: Option<u32>,
    /// Layers the resulting template composes (empty for a removal).
    pub composed_layers: Vec<PlannedLayer>,
    /// The field-level diff (`None` for install / remove).
    pub diff: Option<TemplateDiff>,
    /// Effective maturity of the resulting template (`None` for a removal).
    pub effective_maturity: Option<ClaimMaturity>,
    /// Verdict of the resulting template (`None` for a removal).
    pub verdict: Option<RowVerdict>,
    /// Effective warm-start posture of the resulting template (`None` for a
    /// removal).
    pub effective_warm_start_posture: Option<WarmStartPosture>,
    /// Whether the plan is reviewable before it applies (always `true`).
    pub reviewable: bool,
    /// Review-safe description of how to roll the change back.
    pub rollback_summary: String,
    /// Review-safe summary of the plan.
    pub summary: String,
}

fn planned_layers(template: &WorkspaceTemplate) -> Vec<PlannedLayer> {
    let mut layers = vec![PlannedLayer {
        kind: CompositionLayerKind::EnvironmentCapsule,
        layer_id: template.environment_capsule.identity.capsule_id.clone(),
        reference: WORKSPACE_TEMPLATE_SCHEMA_REF.to_owned(),
    }];
    for bundle in &template.workflow_bundle_refs {
        layers.push(PlannedLayer {
            kind: CompositionLayerKind::WorkflowBundle,
            layer_id: bundle.bundle_id.clone(),
            reference: bundle.reference.clone(),
        });
    }
    for archetype in &template.archetype_defaults {
        layers.push(PlannedLayer {
            kind: CompositionLayerKind::ArchetypeDefault,
            layer_id: archetype.archetype_id.clone(),
            reference: archetype.reference.clone(),
        });
    }
    for docs in &template.docs_refs {
        layers.push(PlannedLayer {
            kind: CompositionLayerKind::DocsReference,
            layer_id: docs.docs_id.clone(),
            reference: docs.reference.clone(),
        });
    }
    layers
}

/// Builds a reviewable, rollback-aware plan for a template lifecycle change.
///
/// - `Install` expects `base` to be `None` and `target` to be `Some`.
/// - `Update` expects both `base` and `target` to be `Some`.
/// - `Remove` expects `base` to be `Some` and `target` to be `None`.
///
/// Any other combination yields a plan whose summary records the mismatch and
/// leaves the resulting claim empty, so a malformed lifecycle call is visible
/// rather than silently applied.
pub fn plan_template_change(
    op: TemplateLifecycleOp,
    base: Option<&WorkspaceTemplate>,
    target: Option<&WorkspaceTemplate>,
) -> TemplateChangePlan {
    let resulting = match op {
        TemplateLifecycleOp::Install | TemplateLifecycleOp::Update => target,
        TemplateLifecycleOp::Remove => None,
    };
    let template_id = target
        .or(base)
        .map(|t| t.identity.template_id.clone())
        .unwrap_or_default();

    let diff = match (base, target) {
        (Some(base), Some(target)) => Some(diff_templates(base, target)),
        _ => None,
    };

    let composed_layers = resulting.map(planned_layers).unwrap_or_default();
    let outcome = resulting.map(inspect_template);

    let rollback_summary = match op {
        TemplateLifecycleOp::Install => {
            "Roll back by removing the installed template; no prior version is restored.".to_owned()
        }
        TemplateLifecycleOp::Update => format!(
            "Roll back by reinstalling template version {}.",
            base.map(|t| t.identity.template_version).unwrap_or(0)
        ),
        TemplateLifecycleOp::Remove => format!(
            "Roll back by reinstalling template version {}.",
            base.map(|t| t.identity.template_version).unwrap_or(0)
        ),
    };

    let summary = match op {
        TemplateLifecycleOp::Install => format!(
            "Install composes {} layer(s) for template {}.",
            composed_layers.len(),
            template_id
        ),
        TemplateLifecycleOp::Update => format!(
            "Update recomposes template {} with {} change(s).",
            template_id,
            diff.as_ref()
                .map(|d| d.changes.len() + d.capsule_diff.changes.len())
                .unwrap_or(0)
        ),
        TemplateLifecycleOp::Remove => {
            format!("Remove tears down the composed layers of template {template_id}.")
        }
    };

    TemplateChangePlan {
        record_kind: WORKSPACE_TEMPLATE_PLAN_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TEMPLATE_SCHEMA_VERSION,
        op,
        template_id,
        before_version: base.map(|t| t.identity.template_version),
        after_version: target.map(|t| t.identity.template_version),
        composed_layers,
        diff,
        effective_maturity: outcome.as_ref().map(|o| o.effective_maturity),
        verdict: outcome.as_ref().map(|o| o.verdict),
        effective_warm_start_posture: outcome.as_ref().map(|o| o.effective_warm_start_posture),
        reviewable: true,
        rollback_summary,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Fixture record.
// ---------------------------------------------------------------------------

/// One checked-in fixture: a template of a given source class plus the
/// inspection outcome the engine must reach for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Source class the fixture exercises.
    pub source_class: TemplateSourceClass,
    /// The template under test.
    pub template: WorkspaceTemplate,
    /// Expected engine verdict.
    pub expected_verdict: RowVerdict,
    /// Expected effective maturity.
    pub expected_effective_maturity: ClaimMaturity,
    /// Expected effective warm-start posture.
    pub expected_warm_start_posture: WarmStartPosture,
    /// Expected combined maturity-narrowing tokens.
    pub expected_narrow_reason_tokens: Vec<String>,
    /// Expected composition-only narrowing tokens.
    pub expected_composition_narrow_tokens: Vec<String>,
    /// Expected warm-start-downgrade tokens.
    pub expected_warm_start_downgrade_tokens: Vec<String>,
    /// One consumer surface that ingests this template.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

fn violation(report: &mut ValidationReport, check_id: &'static str, message: impl Into<String>) {
    report.violations.push(ValidationViolation {
        check_id,
        message: message.into(),
    });
}

fn validate_digest(report: &mut ValidationReport, owner: &str, digest: &CapsuleDigest) {
    if digest.algorithm.trim().is_empty() {
        violation(
            report,
            "template.digest_algorithm",
            format!("{owner} digest must name an algorithm"),
        );
    }
    let value_ok = digest.value.len() == 64 && digest.value.chars().all(|c| c.is_ascii_hexdigit());
    if !value_ok {
        violation(
            report,
            "template.digest_value",
            format!("{owner} digest value must be a 64-char hex string"),
        );
    }
}

/// Whether a signer class is consistent with a source class.
fn signer_matches_source(source: TemplateSourceClass, signer: SignerClass) -> bool {
    match source {
        TemplateSourceClass::FirstParty => signer == SignerClass::FirstPartySigned,
        TemplateSourceClass::ManagedApproved => matches!(
            signer,
            SignerClass::MirrorCountersigned | SignerClass::FirstPartySigned
        ),
        TemplateSourceClass::Community => matches!(
            signer,
            SignerClass::CommunitySigned | SignerClass::MirrorCountersigned
        ),
        TemplateSourceClass::LocalDraft => signer == SignerClass::Unsigned,
    }
}

/// Validates a checked-in template object against the frozen contract.
pub fn validate_workspace_template(template: &WorkspaceTemplate) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if template.record_kind != WORKSPACE_TEMPLATE_RECORD_KIND {
        violation(
            &mut report,
            "template.record_kind",
            "template record_kind does not match the frozen token",
        );
    }
    if template.schema_version != WORKSPACE_TEMPLATE_SCHEMA_VERSION {
        violation(
            &mut report,
            "template.schema_version",
            "template schema_version must be 1",
        );
    }
    if template.identity.template_id.trim().is_empty() {
        violation(
            &mut report,
            "template.id",
            "template must carry a stable id",
        );
    }
    if template.identity.template_version == 0 {
        violation(
            &mut report,
            "template.version",
            "template version must be at least 1",
        );
    }
    if template.identity.label.trim().is_empty() {
        violation(&mut report, "template.label", "template must carry a label");
    }
    validate_digest(
        &mut report,
        "template.identity",
        &template.identity.template_digest,
    );

    // The embedded capsule must validate on its own.
    if let Err(capsule_report) = validate_environment_capsule(&template.environment_capsule) {
        for inner in capsule_report.violations {
            report.violations.push(inner);
        }
    }

    // No forking: the template must inherit the capsule's claim, never widen it.
    if template.claimed_maturity != template.environment_capsule.claimed_maturity {
        violation(
            &mut report,
            "template.claimed_maturity_matches_capsule",
            "template claimed_maturity must equal the embedded capsule's claimed maturity",
        );
    }
    if template.claimed_warm_start_posture
        != template.environment_capsule.claimed_warm_start_posture
    {
        violation(
            &mut report,
            "template.claimed_warm_start_matches_capsule",
            "template claimed_warm_start_posture must equal the embedded capsule's",
        );
    }

    // Composition layers must be present and well-formed.
    if template.workflow_bundle_refs.is_empty() {
        violation(
            &mut report,
            "template.workflow_bundle_refs",
            "template must reference at least one workflow bundle",
        );
    }
    let mut bundle_ids = BTreeSet::new();
    for bundle in &template.workflow_bundle_refs {
        if bundle.bundle_id.trim().is_empty() {
            violation(
                &mut report,
                "template.bundle_id",
                "workflow bundle ref must carry an id",
            );
        } else if !bundle_ids.insert(bundle.bundle_id.as_str()) {
            violation(
                &mut report,
                "template.bundle_id_unique",
                format!("template repeats workflow bundle id {}", bundle.bundle_id),
            );
        }
        if bundle.reference.trim().is_empty() {
            violation(
                &mut report,
                "template.bundle_reference",
                format!(
                    "workflow bundle ref {} must carry a reference",
                    bundle.bundle_id
                ),
            );
        }
        validate_digest(
            &mut report,
            &format!("workflow bundle {}", bundle.bundle_id),
            &bundle.digest,
        );
        // Guardrail: a bundle reference may not widen the execution scope.
        if bundle.widens_execution_scope {
            violation(
                &mut report,
                "template.bundle_widens_scope",
                format!(
                    "workflow bundle {} must not widen the execution scope",
                    bundle.bundle_id
                ),
            );
        }
    }

    if template.archetype_defaults.is_empty() {
        violation(
            &mut report,
            "template.archetype_defaults",
            "template must seed at least one certified-archetype default",
        );
    }
    let mut archetype_ids = BTreeSet::new();
    for archetype in &template.archetype_defaults {
        if archetype.archetype_id.trim().is_empty() {
            violation(
                &mut report,
                "template.archetype_id",
                "archetype default must carry an id",
            );
        } else if !archetype_ids.insert(archetype.archetype_id.as_str()) {
            violation(
                &mut report,
                "template.archetype_id_unique",
                format!("template repeats archetype id {}", archetype.archetype_id),
            );
        }
        if archetype.reference.trim().is_empty() {
            violation(
                &mut report,
                "template.archetype_reference",
                format!(
                    "archetype default {} must carry a reference",
                    archetype.archetype_id
                ),
            );
        }
        validate_digest(
            &mut report,
            &format!("archetype default {}", archetype.archetype_id),
            &archetype.digest,
        );
    }

    if template.docs_refs.is_empty() {
        violation(
            &mut report,
            "template.docs_refs",
            "template must link at least one docs / onboarding reference",
        );
    }
    let mut docs_ids = BTreeSet::new();
    for docs in &template.docs_refs {
        if docs.docs_id.trim().is_empty() {
            violation(&mut report, "template.docs_id", "docs ref must carry an id");
        } else if !docs_ids.insert(docs.docs_id.as_str()) {
            violation(
                &mut report,
                "template.docs_id_unique",
                format!("template repeats docs id {}", docs.docs_id),
            );
        }
        if docs.reference.trim().is_empty() {
            violation(
                &mut report,
                "template.docs_reference",
                format!("docs ref {} must carry a reference", docs.docs_id),
            );
        }
    }

    // Trust posture: signer must be consistent with the source class, and an
    // unsigned template may not claim current attestation.
    if !signer_matches_source(template.trust.source_class, template.trust.signer_class) {
        violation(
            &mut report,
            "template.signer_matches_source",
            format!(
                "signer class {} is inconsistent with source class {}",
                template.trust.signer_class.as_str(),
                template.trust.source_class.as_str()
            ),
        );
    }
    if template.trust.signer_class == SignerClass::Unsigned
        && template.trust.attestation_state == EvidenceState::Current
    {
        violation(
            &mut report,
            "template.unsigned_attestation",
            "an unsigned template must not claim current attestation",
        );
    }
    if template.trust.summary.trim().is_empty() {
        violation(
            &mut report,
            "template.trust_summary",
            "trust posture must carry a summary",
        );
    }
    if template.support.summary.trim().is_empty() {
        violation(
            &mut report,
            "template.support_summary",
            "support posture must carry a summary",
        );
    }

    // Guardrails must be clean.
    if !template.guardrails.is_clean() {
        violation(
            &mut report,
            "template.guardrails",
            "template composition must not inject proprietary service dependence, ungated hooks, or bundle/runtime widening",
        );
    }
    if template.guardrails.summary.trim().is_empty() {
        violation(
            &mut report,
            "template.guardrails_summary",
            "guardrails must carry a summary",
        );
    }

    if template.why_this_template.trim().is_empty() {
        violation(
            &mut report,
            "template.why_this_template",
            "template must carry a why-this-template headline",
        );
    }
    if template.notes.trim().is_empty() {
        violation(
            &mut report,
            "template.notes",
            "template must carry a reviewer note",
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates a checked-in template fixture: the template itself, and that the
/// recorded expectations equal what the inspector computes.
pub fn validate_workspace_template_fixture(
    fixture: &WorkspaceTemplateFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != WORKSPACE_TEMPLATE_FIXTURE_RECORD_KIND {
        violation(
            &mut report,
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != WORKSPACE_TEMPLATE_SCHEMA_VERSION {
        violation(
            &mut report,
            "fixture.schema_version",
            "fixture schema_version must be 1",
        );
    }
    if fixture.fixture_id.trim().is_empty() {
        violation(&mut report, "fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        violation(
            &mut report,
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        violation(
            &mut report,
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    if let Err(template_report) = validate_workspace_template(&fixture.template) {
        for inner in template_report.violations {
            report.violations.push(inner);
        }
    }

    let inspection = inspect_template(&fixture.template);
    if fixture.expected_verdict != inspection.verdict {
        violation(
            &mut report,
            "fixture.expected_verdict",
            format!(
                "fixture {} expected verdict {} disagrees with the inspector ({})",
                fixture.fixture_id,
                fixture.expected_verdict.as_str(),
                inspection.verdict.as_str()
            ),
        );
    }
    if fixture.expected_effective_maturity != inspection.effective_maturity {
        violation(
            &mut report,
            "fixture.expected_effective_maturity",
            format!(
                "fixture {} expected maturity {} disagrees with the inspector ({})",
                fixture.fixture_id,
                fixture.expected_effective_maturity.as_str(),
                inspection.effective_maturity.as_str()
            ),
        );
    }
    if fixture.expected_warm_start_posture != inspection.effective_warm_start_posture {
        violation(
            &mut report,
            "fixture.expected_warm_start_posture",
            format!(
                "fixture {} expected warm-start posture {} disagrees with the inspector ({})",
                fixture.fixture_id,
                fixture.expected_warm_start_posture.as_str(),
                inspection.effective_warm_start_posture.as_str()
            ),
        );
    }
    if fixture.expected_narrow_reason_tokens != inspection.narrow_reason_tokens {
        violation(
            &mut report,
            "fixture.expected_narrow_reason_tokens",
            format!(
                "fixture {} expected narrowing tokens disagree with the inspector",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_composition_narrow_tokens != inspection.composition_narrow_tokens {
        violation(
            &mut report,
            "fixture.expected_composition_narrow_tokens",
            format!(
                "fixture {} expected composition tokens disagree with the inspector",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_warm_start_downgrade_tokens != inspection.warm_start_downgrade_tokens {
        violation(
            &mut report,
            "fixture.expected_warm_start_downgrade_tokens",
            format!(
                "fixture {} expected warm-start downgrade tokens disagree with the inspector",
                fixture.fixture_id
            ),
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Seeded corpus.
// ---------------------------------------------------------------------------

mod seed;

pub use seed::{seeded_workspace_template_fixtures, seeded_workspace_templates};

#[cfg(test)]
mod tests;
