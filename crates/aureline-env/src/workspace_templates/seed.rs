//! The checked-in workspace-template corpus this lane freezes: one
//! canonical template per source class plus the degraded variants that
//! exercise the inspector's composition narrowing, inherited capsule
//! narrowing, and warm-start downgrade.

use crate::capsules::{seeded_environment_capsules, CapsuleDigest, EnvironmentCapsule};
use crate::m5_env_governance::EvidenceState;

use super::{
    inspect_template, ArchetypeDefault, CompositionGuardrails, DocsOnboardingRef, DocsRefKind,
    MirrorClass, SignerClass, SupportClass, SupportPosture, TemplateIdentity, TemplateSourceClass,
    TemplateTrust, WorkflowBundleRef, WorkspaceTemplate, WorkspaceTemplateFixture,
    WORKSPACE_TEMPLATE_FIXTURE_RECORD_KIND, WORKSPACE_TEMPLATE_RECORD_KIND,
    WORKSPACE_TEMPLATE_SCHEMA_VERSION,
};

const WORKFLOW_BUNDLE_MANIFESTS_REF: &str =
    "artifacts/workspace/m5/m5-workflow-bundle-manifests.json";
const WORKFLOW_BUNDLE_IDS_REF: &str = "artifacts/qe/workflow_bundle_ids.yaml";
const ARCHETYPE_CONFIDENCE_REF: &str = "artifacts/workspace/archetype_confidence_rows.yaml";
const START_CENTER_DOC_REF: &str = "docs/help/m5-start-center-and-switcher.md";
const CAPSULE_DOC_REF: &str = "docs/env/environment-capsule.md";

/// Deterministic 64-hex placeholder digest derived from a stable label.
/// These are metadata tokens standing in for real content digests, never
/// the bodies they would digest.
fn dg(label: &str) -> CapsuleDigest {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in label.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let chunk = format!("{hash:016x}");
    CapsuleDigest {
        algorithm: "sha256".to_owned(),
        value: chunk.repeat(4),
    }
}

fn capsule(capsule_id: &str) -> EnvironmentCapsule {
    seeded_environment_capsules()
        .into_iter()
        .find(|capsule| capsule.identity.capsule_id == capsule_id)
        .unwrap_or_else(|| panic!("seeded capsule {capsule_id} must exist"))
}

struct TemplateSpec {
    template_id: &'static str,
    capsule_id: &'static str,
    label: &'static str,
    source_class: TemplateSourceClass,
    signer_class: SignerClass,
    mirror_class: MirrorClass,
    support_class: SupportClass,
    /// Evidence state of the trust attestation backing the template.
    attestation_state: EvidenceState,
    /// Freshness of the template against its upstream.
    freshness_state: EvidenceState,
    consumer_ref: &'static str,
    why: &'static str,
    notes: &'static str,
}

const SPECS: &[TemplateSpec] = &[
    TemplateSpec {
        template_id: "env.template.first_party",
        capsule_id: "env.capsule.local",
        label: "First-party local starter template",
        source_class: TemplateSourceClass::FirstParty,
        signer_class: SignerClass::FirstPartySigned,
        mirror_class: MirrorClass::FirstPartyOrigin,
        support_class: SupportClass::FirstPartySupported,
        attestation_state: EvidenceState::Current,
        freshness_state: EvidenceState::Current,
        consumer_ref: "crates/aureline-shell/src/start_center/mod.rs",
        why: "This template composes a first-party, signed local capsule with the workflow bundles, certified-archetype defaults, and onboarding docs it expects; hydration reuses the same capsule object as a direct local run rather than forking the execution model.",
        notes: "A first-party template is signed and supported; it hydrates the embedded local capsule and certifies at the capsule's claim.",
    },
    TemplateSpec {
        template_id: "env.template.managed_approved",
        capsule_id: "env.capsule.managed_workspace",
        label: "Managed-approved workspace template",
        source_class: TemplateSourceClass::ManagedApproved,
        signer_class: SignerClass::MirrorCountersigned,
        mirror_class: MirrorClass::ManagedMirror,
        support_class: SupportClass::ManagedSupported,
        attestation_state: EvidenceState::Current,
        freshness_state: EvidenceState::Current,
        consumer_ref: "crates/aureline-support/src/bundle/mod.rs",
        why: "This template composes a managed-approved capsule mirror-countersigned through the managed review; its workflow-bundle references and archetype defaults are mirrored so install/update/remove review the same composed layers support reads.",
        notes: "A managed-approved template is countersigned by the managed mirror; it certifies at the embedded managed-workspace capsule's beta claim with full warm reuse.",
    },
    TemplateSpec {
        template_id: "env.template.community",
        capsule_id: "env.capsule.vm",
        label: "Community starter template",
        source_class: TemplateSourceClass::Community,
        signer_class: SignerClass::CommunitySigned,
        mirror_class: MirrorClass::CommunityMirror,
        support_class: SupportClass::CommunitySupported,
        attestation_state: EvidenceState::Current,
        freshness_state: EvidenceState::Current,
        consumer_ref: "crates/aureline-templates/src/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/mod.rs",
        why: "This template composes a community-signed starter capsule with referenced workflow bundles and archetype defaults; the source, signer, and mirror class stay visible so a community template never silently presents itself as first-party.",
        notes: "A community template certifies at its capsule's claim only while its attestation is current; partial attestation narrows the claim.",
    },
    TemplateSpec {
        template_id: "env.template.local_draft",
        capsule_id: "env.capsule.local",
        label: "Local draft template",
        source_class: TemplateSourceClass::LocalDraft,
        signer_class: SignerClass::Unsigned,
        mirror_class: MirrorClass::LocalOnly,
        support_class: SupportClass::Unsupported,
        attestation_state: EvidenceState::NotApplicable,
        freshness_state: EvidenceState::Current,
        consumer_ref: "crates/aureline-scaffold/src/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/mod.rs",
        why: "This template is an unsigned local draft the user is still editing; it composes the same capsule object and layers as a published template, but its unsigned, unsupported, local-only posture stays visible for review.",
        notes: "A local draft is unsigned and local-only; attestation does not apply, so it certifies at its capsule's claim while staying clearly unpublished.",
    },
];

fn workflow_bundle_refs(template_id: &str) -> Vec<WorkflowBundleRef> {
    vec![
        WorkflowBundleRef {
            bundle_id: "bundle.app".to_owned(),
            reference: WORKFLOW_BUNDLE_MANIFESTS_REF.to_owned(),
            digest: dg(&format!("{template_id}:bundle:app")),
            coverage: EvidenceState::Current,
            widens_execution_scope: false,
            summary: "The application workflow bundle the template expects.".to_owned(),
        },
        WorkflowBundleRef {
            bundle_id: "bundle.checks".to_owned(),
            reference: WORKFLOW_BUNDLE_IDS_REF.to_owned(),
            digest: dg(&format!("{template_id}:bundle:checks")),
            coverage: EvidenceState::Current,
            widens_execution_scope: false,
            summary: "The checks / health workflow bundle the template expects.".to_owned(),
        },
    ]
}

fn archetype_defaults(template_id: &str) -> Vec<ArchetypeDefault> {
    vec![ArchetypeDefault {
        archetype_id: "archetype.web_service".to_owned(),
        reference: ARCHETYPE_CONFIDENCE_REF.to_owned(),
        digest: dg(&format!("{template_id}:archetype:web_service")),
        coverage: EvidenceState::Current,
        summary: "The certified web-service archetype default the template seeds.".to_owned(),
    }]
}

fn docs_refs() -> Vec<DocsOnboardingRef> {
    vec![
        DocsOnboardingRef {
            docs_id: "docs.start_center".to_owned(),
            kind: DocsRefKind::StartCenter,
            reference: START_CENTER_DOC_REF.to_owned(),
            coverage: EvidenceState::Current,
            summary: "The start-center and switcher onboarding reference.".to_owned(),
        },
        DocsOnboardingRef {
            docs_id: "docs.environment".to_owned(),
            kind: DocsRefKind::Reference,
            reference: CAPSULE_DOC_REF.to_owned(),
            coverage: EvidenceState::Current,
            summary: "The environment-capsule reference documentation.".to_owned(),
        },
    ]
}

fn base_template(spec: &TemplateSpec) -> WorkspaceTemplate {
    let id = spec.template_id;
    let embedded = capsule(spec.capsule_id);
    let claimed_maturity = embedded.claimed_maturity;
    let claimed_warm_start_posture = embedded.claimed_warm_start_posture;
    WorkspaceTemplate {
        record_kind: WORKSPACE_TEMPLATE_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TEMPLATE_SCHEMA_VERSION,
        identity: TemplateIdentity {
            template_id: id.to_owned(),
            template_version: 1,
            label: spec.label.to_owned(),
            template_digest: dg(&format!("{id}:template")),
            summary: format!("Identity of the {} template.", spec.source_class.as_str()),
        },
        environment_capsule: embedded,
        workflow_bundle_refs: workflow_bundle_refs(id),
        archetype_defaults: archetype_defaults(id),
        docs_refs: docs_refs(),
        trust: TemplateTrust {
            source_class: spec.source_class,
            signer_class: spec.signer_class,
            mirror_class: spec.mirror_class,
            attestation_state: spec.attestation_state,
            summary: format!(
                "Trust posture: {} source, {} signer, {} mirror.",
                spec.source_class.as_str(),
                spec.signer_class.as_str(),
                spec.mirror_class.as_str()
            ),
        },
        support: SupportPosture {
            support_class: spec.support_class,
            freshness_state: spec.freshness_state,
            summary: format!("Support posture: {}.", spec.support_class.as_str()),
        },
        guardrails: CompositionGuardrails {
            injects_proprietary_service_dependence: false,
            introduces_ungated_lifecycle_hooks: false,
            widens_bundle_or_runtime_scope: false,
            summary: "The template composes referenced layers without injecting proprietary service dependence, ungated hooks, or bundle/runtime widening.".to_owned(),
        },
        claimed_maturity,
        claimed_warm_start_posture,
        why_this_template: spec.why.to_owned(),
        notes: spec.notes.to_owned(),
    }
}

/// The canonical template objects this lane freezes, one per source class.
pub fn seeded_workspace_templates() -> Vec<WorkspaceTemplate> {
    SPECS.iter().map(base_template).collect()
}

fn spec_for(source_class: TemplateSourceClass) -> &'static TemplateSpec {
    SPECS
        .iter()
        .find(|spec| spec.source_class == source_class)
        .expect("every source class has a spec")
}

fn fixture(
    fixture_id: &str,
    source_class: TemplateSourceClass,
    template: WorkspaceTemplate,
    consumer_ref: &str,
    notes: &str,
) -> WorkspaceTemplateFixture {
    let inspection = inspect_template(&template);
    WorkspaceTemplateFixture {
        record_kind: WORKSPACE_TEMPLATE_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TEMPLATE_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        source_class,
        template,
        expected_verdict: inspection.verdict,
        expected_effective_maturity: inspection.effective_maturity,
        expected_warm_start_posture: inspection.effective_warm_start_posture,
        expected_narrow_reason_tokens: inspection.narrow_reason_tokens,
        expected_composition_narrow_tokens: inspection.composition_narrow_tokens,
        expected_warm_start_downgrade_tokens: inspection.warm_start_downgrade_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

/// The checked-in fixture corpus: one certified template per source class
/// plus the degraded variants that drive the inspector's composition
/// narrowing, withholding, inherited capsule narrowing, and warm-start
/// downgrade.
pub fn seeded_workspace_template_fixtures() -> Vec<WorkspaceTemplateFixture> {
    let mut fixtures = Vec::new();

    for spec in SPECS {
        fixtures.push(fixture(
            &format!(
                "fixture.workspace_template.{}_certified",
                spec.source_class.as_str()
            ),
            spec.source_class,
            base_template(spec),
            spec.consumer_ref,
            "A fully current template certifies at its embedded capsule's claim and warm-start posture.",
        ));
    }

    // Community: partial trust attestation narrows the claim through the
    // composition layer without touching warm start.
    let community = spec_for(TemplateSourceClass::Community);
    let mut partial_attestation = base_template(community);
    partial_attestation.trust.attestation_state = EvidenceState::Partial;
    partial_attestation.trust.summary =
        "Trust posture: community attestation covers only part of the published artifact."
            .to_owned();
    fixtures.push(fixture(
        "fixture.workspace_template.community_attestation_partial",
        TemplateSourceClass::Community,
        partial_attestation,
        community.consumer_ref,
        "Partial trust attestation narrows the community template claim to beta through the composition layer.",
    ));

    // Local draft: missing attestation withholds the template entirely.
    let local_draft = spec_for(TemplateSourceClass::LocalDraft);
    let mut missing_attestation = base_template(local_draft);
    missing_attestation.trust.attestation_state = EvidenceState::Missing;
    missing_attestation.trust.summary =
        "Trust posture: the local draft has no attestation backing it.".to_owned();
    fixtures.push(fixture(
        "fixture.workspace_template.local_draft_attestation_missing",
        TemplateSourceClass::LocalDraft,
        missing_attestation,
        local_draft.consumer_ref,
        "A local draft with missing attestation is withheld rather than presented as installable.",
    ));

    // Managed-approved: a stale workflow-bundle reference narrows the claim
    // through the composition layer.
    let managed = spec_for(TemplateSourceClass::ManagedApproved);
    let mut stale_bundle = base_template(managed);
    if let Some(bundle) = stale_bundle.workflow_bundle_refs.first_mut() {
        bundle.coverage = EvidenceState::Stale;
        bundle.summary =
            "The referenced workflow bundle trails its current manifest digest.".to_owned();
    }
    fixtures.push(fixture(
        "fixture.workspace_template.managed_workflow_bundle_stale",
        TemplateSourceClass::ManagedApproved,
        stale_bundle,
        managed.consumer_ref,
        "A stale workflow-bundle reference narrows the managed template claim to preview through the composition layer.",
    ));

    // Community over a stale-fingerprint capsule: the template inherits the
    // capsule's narrowing and warm-start downgrade rather than forking them.
    let mut inherited_downgrade = base_template(community);
    inherited_downgrade.environment_capsule = capsule("env.capsule.container");
    inherited_downgrade
        .environment_capsule
        .compatibility_fingerprint
        .coverage = EvidenceState::Stale;
    inherited_downgrade
        .environment_capsule
        .compatibility_fingerprint
        .summary =
        "Fingerprint trails the current source digest; warm reuse is no longer trustworthy."
            .to_owned();
    inherited_downgrade.claimed_maturity = inherited_downgrade.environment_capsule.claimed_maturity;
    inherited_downgrade.claimed_warm_start_posture = inherited_downgrade
        .environment_capsule
        .claimed_warm_start_posture;
    fixtures.push(fixture(
        "fixture.workspace_template.community_capsule_fingerprint_stale",
        TemplateSourceClass::Community,
        inherited_downgrade,
        community.consumer_ref,
        "A stale embedded-capsule fingerprint narrows the template and forces a cold build, proving the template inherits the capsule's downgrade instead of forking it.",
    ));

    fixtures
}
