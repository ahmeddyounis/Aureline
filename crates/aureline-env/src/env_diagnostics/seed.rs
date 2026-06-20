//! Seeded environment-artifact bundles, diagnostics reports, and fixtures.
//!
//! The corpus assembles three bundles from the **same** seeded capsule,
//! template, prebuild, and runtime objects the sibling modules ship, one
//! per source channel:
//!
//! - `env.bundle.local_online` — a first-party online capture where every
//!   artifact is trusted;
//! - `env.bundle.remote_mirror` — a managed-mirror capture whose capsule,
//!   template, prebuild, and runtime each downgrade visibly; and
//! - `env.bundle.offline_sealed` — a sealed offline import that is blocked
//!   by an ungated capsule hook and a wrong-target runtime.
//!
//! Every fixture's expectations are re-derived through [`diagnose_bundle`]
//! so the recorded outcomes can never drift from the engine.

use super::{
    assemble_env_bundle, diagnose_bundle, ArtifactProvenance, EnvArtifactBundle,
    EnvDiagnosticsFixture, EnvDiagnosticsReport, ProducerSurface, SourceChannel,
    ENV_DIAGNOSTICS_FIXTURE_RECORD_KIND, ENV_DIAGNOSTICS_SCHEMA_VERSION,
};
use crate::capsules::{
    export_capsule_metadata, seeded_environment_capsules, CapsuleExport, EnvironmentCapsule,
    RedactionClass, TrustGateState,
};
use crate::m5_env_governance::EvidenceState;
use crate::prebuilds::{
    export_prebuild_decision, seeded_prebuild_fingerprint_packet, PrebuildExport,
};
use crate::runtime_materialization::{
    export_runtime_materialization, materialize_runtime, seeded_runtime_materialization_fixtures,
    seeded_runtime_materializations, RuntimeExport,
};
use crate::workspace_templates::{
    export_template_metadata, seeded_workspace_templates, TemplateExport,
};

const BUILD_IDENTITY_REF: &str = "artifacts/build/build_identity.json";
const SHELL_CONSUMER: &str = "crates/aureline-shell/src/environment_inspector/mod.rs";
const SUPPORT_CONSUMER: &str = "crates/aureline-support/src/bundle/mod.rs";
const DOCTOR_CONSUMER: &str = "crates/aureline-doctor/src/probes/mod.rs";

fn capsule(capsule_id: &str) -> EnvironmentCapsule {
    seeded_environment_capsules()
        .into_iter()
        .find(|capsule| capsule.identity.capsule_id == capsule_id)
        .unwrap_or_else(|| panic!("capsule {capsule_id} exists"))
}

fn capsule_export(capsule_id: &str) -> CapsuleExport {
    export_capsule_metadata(&capsule(capsule_id))
}

/// A local capsule whose first lifecycle hook is ungated, so the capsule is
/// withheld and the bundle is blocked.
fn ungated_capsule_export() -> CapsuleExport {
    let mut capsule = capsule("env.capsule.local");
    capsule.trust_hooks[0].gate_state = TrustGateState::Ungated;
    export_capsule_metadata(&capsule)
}

/// A container capsule whose prebuild fingerprint is stale, so warm reuse
/// downgrades to a cold build.
fn stale_fingerprint_capsule_export() -> CapsuleExport {
    let mut capsule = capsule("env.capsule.container");
    capsule.compatibility_fingerprint.coverage = EvidenceState::Stale;
    export_capsule_metadata(&capsule)
}

fn template_export(template_id: &str) -> TemplateExport {
    let template = seeded_workspace_templates()
        .into_iter()
        .find(|template| template.identity.template_id == template_id)
        .unwrap_or_else(|| panic!("template {template_id} exists"));
    export_template_metadata(&template)
}

fn prebuild_export(case_id: &str) -> PrebuildExport {
    let case = seeded_prebuild_fingerprint_packet()
        .cases
        .into_iter()
        .find(|case| case.case_id == case_id)
        .unwrap_or_else(|| panic!("prebuild case {case_id} exists"));
    export_prebuild_decision(&case.decision)
}

/// The first aligned runtime materialization (the local target class).
fn aligned_runtime_export() -> RuntimeExport {
    let materialization = seeded_runtime_materializations()
        .into_iter()
        .next()
        .expect("an aligned runtime materialization exists");
    export_runtime_materialization(&materialization)
}

/// Re-derives the runtime materialization for one seeded scenario fixture
/// and exports it, so the degraded and mismatched runtimes flow through the
/// same engine the runtime lane uses.
fn runtime_export_for_fixture(fixture_id: &str) -> RuntimeExport {
    let fixture = seeded_runtime_materialization_fixtures()
        .into_iter()
        .find(|fixture| fixture.fixture_id == fixture_id)
        .unwrap_or_else(|| panic!("runtime fixture {fixture_id} exists"));
    let materialization = materialize_runtime(&fixture.capsule, &fixture.instance);
    export_runtime_materialization(&materialization)
}

fn provenance(
    producer_surface: ProducerSurface,
    source_channel: SourceChannel,
    source_truth: &str,
    mirror_origin_ref: &str,
    captured_ref: &str,
) -> ArtifactProvenance {
    ArtifactProvenance {
        schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
        producer_surface,
        producer_build_ref: BUILD_IDENTITY_REF.to_owned(),
        source_channel,
        source_truth: source_truth.to_owned(),
        mirror_origin_ref: mirror_origin_ref.to_owned(),
        redaction_class: RedactionClass::MetadataOnly,
        captured_ref: captured_ref.to_owned(),
    }
}

/// A first-party online capture: every artifact is trusted.
fn local_online_bundle() -> EnvArtifactBundle {
    assemble_env_bundle(
        "env.bundle.local_online",
        provenance(
            ProducerSurface::Desktop,
            SourceChannel::Online,
            "First-party origin reached over the vendor network",
            "",
            "artifacts/env/environment-capsule-proof.md",
        ),
        vec![capsule_export("env.capsule.local")],
        vec![template_export("env.template.first_party")],
        vec![prebuild_export("case.prebuild.full_match")],
        vec![aligned_runtime_export()],
    )
}

/// A managed-mirror capture: each artifact downgrades visibly, but nothing
/// is untrusted, so the bundle stays shareable pending review.
fn remote_mirror_bundle() -> EnvArtifactBundle {
    assemble_env_bundle(
        "env.bundle.remote_mirror",
        provenance(
            ProducerSurface::Headless,
            SourceChannel::Mirror,
            "Managed mirror snapshot with the vendor network absent",
            "artifacts/templates/workspace_template_seed.yaml",
            "artifacts/env/m5-env-proof-packet.json",
        ),
        vec![stale_fingerprint_capsule_export()],
        vec![template_export("env.template.community")],
        vec![prebuild_export("case.devcontainer.extension_lock_drift")],
        vec![runtime_export_for_fixture(
            "runtime_container_partial_service",
        )],
    )
}

/// A sealed offline import: an ungated capsule hook and a wrong-target
/// runtime block the bundle from being shared.
fn offline_sealed_bundle() -> EnvArtifactBundle {
    assemble_env_bundle(
        "env.bundle.offline_sealed",
        provenance(
            ProducerSurface::Support,
            SourceChannel::Offline,
            "Sealed offline import; no network was reached",
            "",
            "artifacts/env/hook-review-and-repair.md",
        ),
        vec![ungated_capsule_export()],
        Vec::new(),
        vec![prebuild_export("case.managed_workspace.platform_drift")],
        vec![runtime_export_for_fixture("runtime_container_wrong_target")],
    )
}

/// The seeded environment-artifact bundles, one per source channel.
pub fn seeded_env_artifact_bundles() -> Vec<EnvArtifactBundle> {
    vec![
        local_online_bundle(),
        remote_mirror_bundle(),
        offline_sealed_bundle(),
    ]
}

/// The diagnostics report for every seeded bundle.
pub fn seeded_env_diagnostics_reports() -> Vec<EnvDiagnosticsReport> {
    seeded_env_artifact_bundles()
        .iter()
        .map(diagnose_bundle)
        .collect()
}

fn fixture(
    fixture_id: &str,
    bundle: EnvArtifactBundle,
    consumer_ref: &str,
    notes: &str,
) -> EnvDiagnosticsFixture {
    let report = diagnose_bundle(&bundle);
    EnvDiagnosticsFixture {
        record_kind: ENV_DIAGNOSTICS_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        source_channel: bundle.provenance.source_channel,
        expected_finding_codes: report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.finding_code)
            .collect(),
        expected_share_blocked: report.share_blocked,
        expected_review_state: report.review_state,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
        bundle,
    }
}

/// The checked-in fixture corpus: one bundle per source channel, covering a
/// fully trusted online capture, a degraded mirror capture, and a blocked
/// offline import.
pub fn seeded_env_diagnostics_fixtures() -> Vec<EnvDiagnosticsFixture> {
    vec![
        fixture(
            "local_online_trusted",
            local_online_bundle(),
            SHELL_CONSUMER,
            "A first-party online capture whose capsule, template, prebuild, and runtime are all trusted.",
        ),
        fixture(
            "remote_mirror_degraded",
            remote_mirror_bundle(),
            SUPPORT_CONSUMER,
            "A managed-mirror capture: stale fingerprint, community template, partial prebuild, and degraded runtime all downgrade visibly but stay shareable.",
        ),
        fixture(
            "offline_sealed_blocked",
            offline_sealed_bundle(),
            DOCTOR_CONSUMER,
            "A sealed offline import blocked by an ungated capsule hook and a wrong-target runtime, with an invalidated prebuild surfaced as not reusable.",
        ),
    ]
}
