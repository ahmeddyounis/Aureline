//! Mirror/offline import-export, schema-version compare, and
//! materialization diagnostics for environment capsules, templates,
//! prebuilds, and runtimes.
//!
//! The sibling [`crate::capsules`], [`crate::workspace_templates`],
//! [`crate::prebuilds`], and [`crate::runtime_materialization`] modules
//! each materialize one typed environment object and one metadata-first
//! export — [`crate::capsules::CapsuleExport`],
//! [`crate::workspace_templates::TemplateExport`],
//! [`crate::prebuilds::PrebuildExport`], and
//! [`crate::runtime_materialization::RuntimeExport`]. What they leave
//! implicit is a *portable contract* that carries those exports together
//! across online, mirrored, and offline profiles and a *diagnostics
//! engine* that explains, in one vocabulary, why a capsule, template,
//! prebuild, or runtime could not be trusted, hydrated, or reused.
//!
//! This module adds that lane. It composes the existing exports — never a
//! private clone — into one [`EnvArtifactBundle`] stamped with an
//! [`ArtifactProvenance`]: the env-artifacts schema version, the producing
//! surface and build, the redaction class, and — the load-bearing addition
//! — the [`SourceChannel`] (online, mirror, or offline) and a review-safe
//! `source_truth` label. The bundle is the unit that
//! [`import_env_bundle`] validates, [`compare_env_bundles`] diffs across
//! schema versions and source channels, and [`diagnose_bundle`] folds into
//! one [`EnvDiagnosticsReport`].
//!
//! The diagnostics engine reuses the verdicts the upstream engines already
//! computed — a capsule inspection's [`crate::m5_env_governance::RowVerdict`],
//! a prebuild's [`crate::prebuilds::StartOutcome`], and a runtime's
//! [`crate::runtime_materialization::RuntimeParity`] — and maps each onto one
//! [`FindingCode`] and one [`HydrationOutcome`], so a withheld capsule, a
//! cold prebuild, or a wrong-target runtime downgrades visibly and
//! identically whether it arrived over the first-party network, a managed
//! mirror, or a sealed offline import. Share is blocked exactly when an
//! artifact is [`HydrationOutcome::Untrusted`]; a [`ReviewState`] keeps the
//! metadata-first, review-before-share posture explicit.
//!
//! Every surface reads the **same** report object:
//! [`desktop_env_diagnostics`], [`headless_env_diagnostics`], and
//! [`support_env_diagnostics`] return one [`EnvDiagnosticsReport`], and
//! [`doctor_env_probes`] projects it into Project-Doctor-shaped
//! [`EnvDoctorProbe`]s with finding codes, evidence refs, and exact recovery
//! paths — so Doctor and support explain an environment-hydration failure
//! from one source of truth instead of cloning environment prose.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::capsules::{
    CapsuleExport, RedactionClass, ENVIRONMENT_CAPSULE_DOC_REF, ENVIRONMENT_CAPSULE_PROOF_REF,
    ENVIRONMENT_CAPSULE_SCHEMA_REF, ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
};
use crate::m5_env_governance::{RowVerdict, ValidationReport, ValidationViolation};
use crate::prebuilds::{
    PrebuildExport, StartOutcome, PREBUILD_FINGERPRINT_DOC_REF, PREBUILD_FINGERPRINT_PROOF_REF,
    PREBUILD_FINGERPRINT_SCHEMA_REF, PREBUILD_FINGERPRINT_SCHEMA_VERSION,
};
use crate::runtime_materialization::{
    RuntimeExport, RuntimeParity, RUNTIME_MATERIALIZATION_DOC_REF,
    RUNTIME_MATERIALIZATION_PROOF_REF, RUNTIME_MATERIALIZATION_SCHEMA_REF,
    RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
};
use crate::workspace_templates::{
    MirrorClass, SupportClass, TemplateExport, WORKSPACE_TEMPLATE_DOC_REF,
    WORKSPACE_TEMPLATE_PROOF_REF, WORKSPACE_TEMPLATE_SCHEMA_REF, WORKSPACE_TEMPLATE_SCHEMA_VERSION,
};

/// Schema version stamped onto bundles, reports, comparisons, and fixtures.
pub const ENV_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by an [`EnvArtifactBundle`].
pub const ENV_ARTIFACT_BUNDLE_RECORD_KIND: &str = "env_artifact_bundle_record";

/// Stable record-kind tag carried by an [`EnvDiagnosticsReport`].
pub const ENV_DIAGNOSTICS_REPORT_RECORD_KIND: &str = "env_diagnostics_report_record";

/// Stable record-kind tag carried by an [`EnvBundleComparison`].
pub const ENV_BUNDLE_COMPARISON_RECORD_KIND: &str = "env_bundle_comparison_record";

/// Stable record-kind tag carried by an [`EnvDoctorProbe`].
pub const ENV_DOCTOR_PROBE_RECORD_KIND: &str = "env_doctor_probe_record";

/// Stable record-kind tag carried by an [`EnvDiagnosticsFixture`].
pub const ENV_DIAGNOSTICS_FIXTURE_RECORD_KIND: &str = "env_diagnostics_fixture_record";

/// Repo-relative schema ref for the env-artifacts contract and fixtures.
pub const ENV_DIAGNOSTICS_SCHEMA_REF: &str = "schemas/env/env-artifacts.schema.json";

/// Repo-relative reviewer doc ref.
pub const ENV_DIAGNOSTICS_DOC_REF: &str = "docs/env/env-diagnostics.md";

/// Repo-relative operator runbook ref.
pub const ENV_DIAGNOSTICS_RUNBOOK_REF: &str = "artifacts/env/env-diagnostics-runbook.md";

/// Repo-relative fixture directory.
pub const ENV_DIAGNOSTICS_FIXTURE_DIR: &str = "fixtures/env/env-diagnostics";

/// Repo-relative fixture manifest.
pub const ENV_DIAGNOSTICS_FIXTURE_MANIFEST_REF: &str = "fixtures/env/env-diagnostics/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// Where an environment-artifact bundle was sourced from. The channel is
/// carried on every bundle, comparison, and diagnostic so mirror and
/// offline profiles reuse the same vocabulary instead of an opaque import
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceChannel {
    /// Resolved from the first-party origin over a reachable vendor network.
    Online,
    /// Resolved from a managed or community mirror with vendor network
    /// access absent.
    Mirror,
    /// Imported fully offline from a sealed bundle; no network was reached.
    Offline,
}

impl SourceChannel {
    /// Every source channel in canonical order.
    pub const ALL: [Self; 3] = [Self::Online, Self::Mirror, Self::Offline];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Mirror => "mirror",
            Self::Offline => "offline",
        }
    }

    /// Whether the channel resolved without first-party vendor network
    /// access. Mirror and offline channels carry the same environment
    /// vocabulary but a different provenance posture.
    pub const fn is_network_absent(self) -> bool {
        matches!(self, Self::Mirror | Self::Offline)
    }
}

/// The surface that produced an environment-artifact bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProducerSurface {
    /// The desktop shell.
    Desktop,
    /// The CLI / headless surface.
    Headless,
    /// The support-export surface.
    Support,
}

impl ProducerSurface {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Headless => "headless",
            Self::Support => "support",
        }
    }
}

/// The family of environment artifact a diagnostic or delta is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    /// A typed environment capsule.
    Capsule,
    /// A workspace template that composes a capsule.
    Template,
    /// A prebuild reuse decision over a snapshot.
    Prebuild,
    /// A runtime materialization of a capsule.
    Runtime,
}

impl ArtifactKind {
    /// Every artifact kind in canonical order.
    pub const ALL: [Self; 4] = [Self::Capsule, Self::Template, Self::Prebuild, Self::Runtime];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Capsule => "capsule",
            Self::Template => "template",
            Self::Prebuild => "prebuild",
            Self::Runtime => "runtime",
        }
    }

    /// The upstream env schema version supported for this kind.
    const fn supported_schema_version(self) -> u32 {
        match self {
            Self::Capsule => ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
            Self::Template => WORKSPACE_TEMPLATE_SCHEMA_VERSION,
            Self::Prebuild => PREBUILD_FINGERPRINT_SCHEMA_VERSION,
            Self::Runtime => RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
        }
    }
}

/// The trust/hydration disposition the diagnostics engine reaches for one
/// artifact. Share is blocked exactly when an artifact is
/// [`HydrationOutcome::Untrusted`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HydrationOutcome {
    /// The artifact is fully trusted; it hydrates or warm-reuses as claimed.
    Trusted,
    /// The artifact is usable but narrowed; the downgrade is visible.
    Degraded,
    /// A warm-reuse path is not trustworthy, but the environment still
    /// hydrates via a rebuild (a cold or invalidated prebuild).
    Unreusable,
    /// The artifact cannot be trusted to hydrate as claimed and must not be
    /// shared until repaired.
    Untrusted,
}

impl HydrationOutcome {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Degraded => "degraded",
            Self::Unreusable => "unreusable",
            Self::Untrusted => "untrusted",
        }
    }

    /// Whether this outcome blocks sharing the bundle.
    pub const fn blocks_share(self) -> bool {
        matches!(self, Self::Untrusted)
    }
}

/// A stable diagnostic finding code naming why an artifact reached its
/// outcome. The codes are locale-invariant machine meanings; the human
/// prose lives in [`FindingCode::explanation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingCode {
    /// Every governing dimension is current; the artifact is trusted.
    Trusted,
    /// The claim narrowed below its claimed maturity but still holds.
    MaturityNarrowed,
    /// Warm-start reuse narrowed below the claimed posture.
    WarmStartDowngraded,
    /// A required dimension cannot be proven; the claim is withheld.
    ClaimWithheld,
    /// A prebuild snapshot is only partially reusable; a layer is rebuilt.
    PrebuildPartialReuse,
    /// A prebuild snapshot is not reusable for current content; the
    /// environment is rebuilt cold.
    PrebuildColdRebuild,
    /// A prebuild snapshot is incompatible or untrusted and is evicted.
    PrebuildInvalidated,
    /// A runtime materialized the right target but part of the stack is not
    /// fully up.
    MaterializationDegraded,
    /// A runtime materialized a different target or namespace than declared.
    MaterializationMismatch,
    /// A mirrored template's source or support is not first-party verified.
    MirrorSourceUnverified,
    /// The artifact declares an env schema version this build cannot read.
    SchemaVersionUnsupported,
    /// The artifact crosses the boundary with a non-metadata redaction class.
    RedactionViolation,
}

impl FindingCode {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::MaturityNarrowed => "maturity_narrowed",
            Self::WarmStartDowngraded => "warm_start_downgraded",
            Self::ClaimWithheld => "claim_withheld",
            Self::PrebuildPartialReuse => "prebuild_partial_reuse",
            Self::PrebuildColdRebuild => "prebuild_cold_rebuild",
            Self::PrebuildInvalidated => "prebuild_invalidated",
            Self::MaterializationDegraded => "materialization_degraded",
            Self::MaterializationMismatch => "materialization_mismatch",
            Self::MirrorSourceUnverified => "mirror_source_unverified",
            Self::SchemaVersionUnsupported => "schema_version_unsupported",
            Self::RedactionViolation => "redaction_violation",
        }
    }

    /// The hydration outcome this finding maps to.
    pub const fn outcome(self) -> HydrationOutcome {
        match self {
            Self::Trusted => HydrationOutcome::Trusted,
            Self::MaturityNarrowed
            | Self::WarmStartDowngraded
            | Self::PrebuildPartialReuse
            | Self::MaterializationDegraded
            | Self::MirrorSourceUnverified => HydrationOutcome::Degraded,
            Self::PrebuildColdRebuild | Self::PrebuildInvalidated => HydrationOutcome::Unreusable,
            Self::ClaimWithheld
            | Self::MaterializationMismatch
            | Self::SchemaVersionUnsupported
            | Self::RedactionViolation => HydrationOutcome::Untrusted,
        }
    }

    /// Whether the finding blocks sharing the bundle.
    pub const fn blocks_share(self) -> bool {
        self.outcome().blocks_share()
    }

    /// Review-safe explanation of the finding.
    pub const fn explanation(self) -> &'static str {
        match self {
            Self::Trusted => {
                "Every governing dimension is current; the artifact hydrates or warm-reuses as claimed."
            }
            Self::MaturityNarrowed => {
                "Partial or stale evidence narrowed the claim below its claimed maturity; the artifact is usable but downgraded."
            }
            Self::WarmStartDowngraded => {
                "Warm-start reuse narrowed below the claimed posture because the prebuild fingerprint outran its source digest."
            }
            Self::ClaimWithheld => {
                "A required dimension — for example an ungated lifecycle hook — cannot be proven, so the environment claim is withheld."
            }
            Self::PrebuildPartialReuse => {
                "Only part of the snapshot is reusable; an affected layer is rebuilt while the rest stays warm."
            }
            Self::PrebuildColdRebuild => {
                "No reuse is trustworthy for current content, so the environment is rebuilt cold; the snapshot is a benign cache miss."
            }
            Self::PrebuildInvalidated => {
                "The snapshot is incompatible or untrusted and is evicted; the environment is rebuilt from source."
            }
            Self::MaterializationDegraded => {
                "The runtime materialized the declared target, but part of the stack is not fully up — a service, mount, port, or secret projection is pending."
            }
            Self::MaterializationMismatch => {
                "The runtime materialized a different target or namespace than the capsule declared: code ran somewhere other than where the environment said it would."
            }
            Self::MirrorSourceUnverified => {
                "The artifact resolved from a community mirror or an unsupported source, so its provenance is not first-party verified."
            }
            Self::SchemaVersionUnsupported => {
                "The artifact declares an environment schema version this build cannot read, so it cannot be trusted or hydrated."
            }
            Self::RedactionViolation => {
                "The artifact crossed the boundary with a non-metadata redaction class, so it cannot be shared."
            }
        }
    }

    /// Exact recovery or escalation path for the finding.
    pub const fn recovery_path(self) -> &'static str {
        match self {
            Self::Trusted => "No action required; export or hydrate the artifact as-is.",
            Self::MaturityNarrowed => {
                "Refresh the stale or partial source evidence named in the reason tokens, then re-export the capsule."
            }
            Self::WarmStartDowngraded => {
                "Rebuild the prebuild snapshot against the current source digest to restore warm reuse, or accept the colder posture."
            }
            Self::ClaimWithheld => {
                "Review and gate the lifecycle hook (or the missing dimension) named in the reason tokens before hydrating."
            }
            Self::PrebuildPartialReuse => {
                "Allow the affected layer to rebuild; the rest of the snapshot stays warm. No manual action is required."
            }
            Self::PrebuildColdRebuild => {
                "Allow the cold rebuild to complete; rebuild the snapshot to restore warm start on the next launch."
            }
            Self::PrebuildInvalidated => {
                "Discard the evicted snapshot and rebuild the environment; investigate the platform, policy, or critical-artifact drift in the reason tokens."
            }
            Self::MaterializationDegraded => {
                "Wait for the pending service, mount, port, or secret projection named in the reason tokens, or open the runtime inspector to repair it."
            }
            Self::MaterializationMismatch => {
                "Stop the wrong-target run and re-materialize on the declared target before trusting the environment."
            }
            Self::MirrorSourceUnverified => {
                "Confirm the mirror or community source is acceptable for this workspace, or switch to a first-party origin."
            }
            Self::SchemaVersionUnsupported => {
                "Update the reader to a build that supports the artifact's env schema version, or re-export from a compatible producer."
            }
            Self::RedactionViolation => {
                "Re-export the artifact through the metadata-first projection so no raw bodies cross the boundary."
            }
        }
    }
}

/// The review-before-share posture of a bundle or report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    /// The bundle carries no blocking findings but still awaits human
    /// review before it is shared.
    PendingReview,
    /// At least one artifact is untrusted; the bundle is blocked from share
    /// until the finding is repaired.
    Blocked,
}

impl ReviewState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::Blocked => "blocked",
        }
    }
}

/// Project-Doctor severity for one environment-diagnostics probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeSeverity {
    /// The artifact is healthy and trusted.
    Healthy,
    /// The artifact is degraded or not reusable but the environment still
    /// hydrates; surfaced as a notice.
    Notice,
    /// The artifact is untrusted; the probe blocks share and hydration.
    Blocking,
}

impl ProbeSeverity {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Notice => "notice",
            Self::Blocking => "blocking",
        }
    }

    /// The probe severity for a hydration outcome.
    const fn for_outcome(outcome: HydrationOutcome) -> Self {
        match outcome {
            HydrationOutcome::Trusted => Self::Healthy,
            HydrationOutcome::Degraded | HydrationOutcome::Unreusable => Self::Notice,
            HydrationOutcome::Untrusted => Self::Blocking,
        }
    }
}

// ---------------------------------------------------------------------------
// Provenance and bundle.
// ---------------------------------------------------------------------------

/// The provenance stamped onto an environment-artifact bundle: schema
/// version, producing surface and build, redaction class, and — the
/// load-bearing fields — the source channel and a review-safe source-truth
/// label. All fields are metadata; no body, secret, or raw payload crosses
/// the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactProvenance {
    /// Env-artifacts schema version the bundle was produced against.
    pub schema_version: u32,
    /// Surface that produced the bundle.
    pub producer_surface: ProducerSurface,
    /// Build-identity reference of the producer (metadata ref, not a body).
    pub producer_build_ref: String,
    /// Where the artifacts were sourced from.
    pub source_channel: SourceChannel,
    /// Review-safe label describing the source (e.g. "first-party origin",
    /// "managed mirror snapshot", "sealed offline import").
    pub source_truth: String,
    /// Metadata ref to the mirror / origin manifest. Required when the
    /// channel is [`SourceChannel::Mirror`]; may be empty for a local
    /// online capture.
    pub mirror_origin_ref: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
    /// Metadata ref to the capture event or record that produced the bundle.
    pub captured_ref: String,
}

/// A portable, metadata-first bundle of environment artifacts. It composes
/// the existing capsule, template, prebuild, and runtime exports under one
/// [`ArtifactProvenance`] so the same artifacts can be exported, imported,
/// compared, and diagnosed across online, mirror, and offline profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvArtifactBundle {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Bundle provenance.
    pub provenance: ArtifactProvenance,
    /// Capsule exports carried by the bundle.
    pub capsules: Vec<CapsuleExport>,
    /// Template exports carried by the bundle.
    pub templates: Vec<TemplateExport>,
    /// Prebuild exports carried by the bundle.
    pub prebuilds: Vec<PrebuildExport>,
    /// Runtime exports carried by the bundle.
    pub runtimes: Vec<RuntimeExport>,
    /// Review-safe summary of the bundle.
    pub summary: String,
}

impl EnvArtifactBundle {
    /// Total number of artifacts across every family.
    pub fn artifact_count(&self) -> usize {
        self.capsules.len() + self.templates.len() + self.prebuilds.len() + self.runtimes.len()
    }
}

/// Assembles a metadata-first bundle from already-projected exports under
/// one provenance. This is the export flow: callers project raw environment
/// objects through their own `export_*` functions, then hand the exports
/// here. The provenance's schema version and redaction class are normalized
/// to the lane's contract.
pub fn assemble_env_bundle(
    bundle_id: impl Into<String>,
    mut provenance: ArtifactProvenance,
    capsules: Vec<CapsuleExport>,
    templates: Vec<TemplateExport>,
    prebuilds: Vec<PrebuildExport>,
    runtimes: Vec<RuntimeExport>,
) -> EnvArtifactBundle {
    let bundle_id = bundle_id.into();
    provenance.schema_version = ENV_DIAGNOSTICS_SCHEMA_VERSION;
    provenance.redaction_class = RedactionClass::MetadataOnly;
    let summary = format!(
        "Metadata-first environment-artifact bundle {} from the {} channel: {} capsule(s), {} template(s), {} prebuild(s), {} runtime(s).",
        bundle_id,
        provenance.source_channel.as_str(),
        capsules.len(),
        templates.len(),
        prebuilds.len(),
        runtimes.len(),
    );
    EnvArtifactBundle {
        record_kind: ENV_ARTIFACT_BUNDLE_RECORD_KIND.to_owned(),
        schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
        bundle_id,
        provenance,
        capsules,
        templates,
        prebuilds,
        runtimes,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Diagnostics engine.
// ---------------------------------------------------------------------------

/// One per-artifact hydration diagnostic: the single object that explains
/// why a capsule, template, prebuild, or runtime could not be trusted,
/// hydrated, or reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvHydrationDiagnostic {
    /// Family of the artifact.
    pub artifact_kind: ArtifactKind,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Env schema version the artifact declared.
    pub artifact_schema_version: u32,
    /// Source channel the artifact arrived over.
    pub source_channel: SourceChannel,
    /// The dominant finding code.
    pub finding_code: FindingCode,
    /// The hydration outcome the finding maps to.
    pub outcome: HydrationOutcome,
    /// True when the diagnostic blocks sharing the bundle.
    pub blocks_share: bool,
    /// Stable reason tokens drawn from the upstream env engine.
    pub reason_tokens: Vec<String>,
    /// Metadata refs backing the diagnostic.
    pub evidence_refs: Vec<String>,
    /// Exact recovery or escalation path.
    pub next_step: String,
    /// Review-safe headline.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// The folded diagnostics report for one bundle: the single object desktop,
/// headless, and support surfaces read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvDiagnosticsReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Bundle id under diagnosis.
    pub bundle_id: String,
    /// Source channel of the diagnosed bundle.
    pub source_channel: SourceChannel,
    /// Bundle provenance, echoed for the report consumer.
    pub provenance: ArtifactProvenance,
    /// Per-artifact diagnostics, in capsule/template/prebuild/runtime order.
    pub diagnostics: Vec<EnvHydrationDiagnostic>,
    /// Count of trusted artifacts.
    pub trusted_count: usize,
    /// Count of degraded artifacts.
    pub degraded_count: usize,
    /// Count of artifacts whose warm reuse failed but still hydrate.
    pub unreusable_count: usize,
    /// Count of untrusted artifacts.
    pub untrusted_count: usize,
    /// Stable tokens naming every artifact that blocks share.
    pub blocking_artifact_tokens: Vec<String>,
    /// True when any artifact is untrusted.
    pub share_blocked: bool,
    /// Review-before-share posture.
    pub review_state: ReviewState,
    /// Review-safe headline.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

fn dedup_tokens(tokens: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for token in tokens {
        if token.trim().is_empty() {
            continue;
        }
        if seen.insert(token.clone()) {
            out.push(token);
        }
    }
    out
}

fn evidence_refs_for(kind: ArtifactKind) -> Vec<String> {
    let (schema, doc, proof) = match kind {
        ArtifactKind::Capsule => (
            ENVIRONMENT_CAPSULE_SCHEMA_REF,
            ENVIRONMENT_CAPSULE_DOC_REF,
            ENVIRONMENT_CAPSULE_PROOF_REF,
        ),
        ArtifactKind::Template => (
            WORKSPACE_TEMPLATE_SCHEMA_REF,
            WORKSPACE_TEMPLATE_DOC_REF,
            WORKSPACE_TEMPLATE_PROOF_REF,
        ),
        ArtifactKind::Prebuild => (
            PREBUILD_FINGERPRINT_SCHEMA_REF,
            PREBUILD_FINGERPRINT_DOC_REF,
            PREBUILD_FINGERPRINT_PROOF_REF,
        ),
        ArtifactKind::Runtime => (
            RUNTIME_MATERIALIZATION_SCHEMA_REF,
            RUNTIME_MATERIALIZATION_DOC_REF,
            RUNTIME_MATERIALIZATION_PROOF_REF,
        ),
    };
    vec![
        schema.to_owned(),
        doc.to_owned(),
        proof.to_owned(),
        ENV_DIAGNOSTICS_SCHEMA_REF.to_owned(),
        ENV_DIAGNOSTICS_RUNBOOK_REF.to_owned(),
    ]
}

fn schema_version_supported(kind: ArtifactKind, declared: u32) -> bool {
    declared == kind.supported_schema_version()
}

fn build_diagnostic(
    kind: ArtifactKind,
    artifact_id: String,
    artifact_schema_version: u32,
    channel: SourceChannel,
    finding_code: FindingCode,
    reason_tokens: Vec<String>,
) -> EnvHydrationDiagnostic {
    let outcome = finding_code.outcome();
    let headline = format!(
        "{} {} on the {} channel: {}",
        kind.as_str(),
        artifact_id,
        channel.as_str(),
        finding_code.explanation(),
    );
    EnvHydrationDiagnostic {
        artifact_kind: kind,
        artifact_id,
        artifact_schema_version,
        source_channel: channel,
        finding_code,
        outcome,
        blocks_share: finding_code.blocks_share(),
        reason_tokens: dedup_tokens(reason_tokens),
        evidence_refs: evidence_refs_for(kind),
        next_step: finding_code.recovery_path().to_owned(),
        headline,
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// Diagnoses one capsule export against the bundle's source channel.
pub fn diagnose_capsule_export(
    export: &CapsuleExport,
    channel: SourceChannel,
) -> EnvHydrationDiagnostic {
    let kind = ArtifactKind::Capsule;
    let inspection = &export.inspection;
    let reason_tokens = dedup_tokens(
        inspection
            .narrow_reason_tokens
            .iter()
            .cloned()
            .chain(inspection.warm_start_downgrade_tokens.iter().cloned())
            .chain(inspection.stale_or_missing_dimension_tokens.iter().cloned()),
    );
    let finding = if !schema_version_supported(kind, export.schema_version) {
        FindingCode::SchemaVersionUnsupported
    } else if export.redaction_class != RedactionClass::MetadataOnly {
        FindingCode::RedactionViolation
    } else {
        match inspection.verdict {
            RowVerdict::Withheld => FindingCode::ClaimWithheld,
            RowVerdict::Narrowed if inspection.warm_start_downgraded => {
                FindingCode::WarmStartDowngraded
            }
            RowVerdict::Narrowed => FindingCode::MaturityNarrowed,
            RowVerdict::Certified => FindingCode::Trusted,
        }
    };
    build_diagnostic(
        kind,
        export.capsule_id.clone(),
        export.schema_version,
        channel,
        finding,
        reason_tokens,
    )
}

/// Diagnoses one template export against the bundle's source channel.
pub fn diagnose_template_export(
    export: &TemplateExport,
    channel: SourceChannel,
) -> EnvHydrationDiagnostic {
    let kind = ArtifactKind::Template;
    let inspection = &export.inspection;
    let reason_tokens = dedup_tokens(
        inspection
            .narrow_reason_tokens
            .iter()
            .cloned()
            .chain(inspection.warm_start_downgrade_tokens.iter().cloned())
            .chain(inspection.stale_or_missing_tokens.iter().cloned()),
    );
    let mirror_unverified = matches!(export.mirror_class, MirrorClass::CommunityMirror)
        || matches!(export.support_class, SupportClass::Unsupported);
    let finding = if !schema_version_supported(kind, export.schema_version) {
        FindingCode::SchemaVersionUnsupported
    } else if export.redaction_class != RedactionClass::MetadataOnly {
        FindingCode::RedactionViolation
    } else {
        match inspection.verdict {
            RowVerdict::Withheld => FindingCode::ClaimWithheld,
            RowVerdict::Narrowed if inspection.warm_start_downgraded => {
                FindingCode::WarmStartDowngraded
            }
            RowVerdict::Narrowed => FindingCode::MaturityNarrowed,
            RowVerdict::Certified if mirror_unverified => FindingCode::MirrorSourceUnverified,
            RowVerdict::Certified => FindingCode::Trusted,
        }
    };
    build_diagnostic(
        kind,
        export.template_id.clone(),
        export.schema_version,
        channel,
        finding,
        reason_tokens,
    )
}

/// Diagnoses one prebuild export against the bundle's source channel.
pub fn diagnose_prebuild_export(
    export: &PrebuildExport,
    channel: SourceChannel,
) -> EnvHydrationDiagnostic {
    let kind = ArtifactKind::Prebuild;
    let reason_tokens = dedup_tokens(
        export
            .reason_tokens
            .iter()
            .cloned()
            .chain(export.gated_action_tokens.iter().cloned()),
    );
    let finding = if !schema_version_supported(kind, export.schema_version) {
        FindingCode::SchemaVersionUnsupported
    } else if export.redaction_class != RedactionClass::MetadataOnly {
        FindingCode::RedactionViolation
    } else {
        match export.outcome {
            StartOutcome::Warm => FindingCode::Trusted,
            StartOutcome::PartiallyWarm => FindingCode::PrebuildPartialReuse,
            StartOutcome::Cold => FindingCode::PrebuildColdRebuild,
            StartOutcome::Invalidated => FindingCode::PrebuildInvalidated,
        }
    };
    build_diagnostic(
        kind,
        export.snapshot_id.clone(),
        export.schema_version,
        channel,
        finding,
        reason_tokens,
    )
}

/// Diagnoses one runtime export against the bundle's source channel. This
/// is the materialization-diagnostics path: a wrong-target run is reported
/// as a mismatch rather than collapsed into a generic "workspace started".
pub fn diagnose_runtime_export(
    export: &RuntimeExport,
    channel: SourceChannel,
) -> EnvHydrationDiagnostic {
    let kind = ArtifactKind::Runtime;
    let reason_tokens = dedup_tokens(
        export
            .reason_tokens
            .iter()
            .cloned()
            .chain(export.degraded_facet_tokens.iter().cloned())
            .chain(export.unready_service_tokens.iter().cloned()),
    );
    let finding = if !schema_version_supported(kind, export.schema_version) {
        FindingCode::SchemaVersionUnsupported
    } else if export.redaction_class != RedactionClass::MetadataOnly {
        FindingCode::RedactionViolation
    } else {
        match export.parity {
            RuntimeParity::Aligned => FindingCode::Trusted,
            RuntimeParity::Degraded => FindingCode::MaterializationDegraded,
            RuntimeParity::Mismatched => FindingCode::MaterializationMismatch,
        }
    };
    build_diagnostic(
        kind,
        export.instance_id.clone(),
        export.schema_version,
        channel,
        finding,
        reason_tokens,
    )
}

/// Folds a bundle into one diagnostics report. Every contained export is
/// diagnosed in capsule/template/prebuild/runtime order against the
/// bundle's source channel, and the rollups, blocking tokens, and review
/// state are derived from the per-artifact outcomes.
pub fn diagnose_bundle(bundle: &EnvArtifactBundle) -> EnvDiagnosticsReport {
    let channel = bundle.provenance.source_channel;
    let mut diagnostics = Vec::with_capacity(bundle.artifact_count());
    diagnostics.extend(
        bundle
            .capsules
            .iter()
            .map(|export| diagnose_capsule_export(export, channel)),
    );
    diagnostics.extend(
        bundle
            .templates
            .iter()
            .map(|export| diagnose_template_export(export, channel)),
    );
    diagnostics.extend(
        bundle
            .prebuilds
            .iter()
            .map(|export| diagnose_prebuild_export(export, channel)),
    );
    diagnostics.extend(
        bundle
            .runtimes
            .iter()
            .map(|export| diagnose_runtime_export(export, channel)),
    );

    let mut trusted_count = 0;
    let mut degraded_count = 0;
    let mut unreusable_count = 0;
    let mut untrusted_count = 0;
    let mut blocking_artifact_tokens = Vec::new();
    for diagnostic in &diagnostics {
        match diagnostic.outcome {
            HydrationOutcome::Trusted => trusted_count += 1,
            HydrationOutcome::Degraded => degraded_count += 1,
            HydrationOutcome::Unreusable => unreusable_count += 1,
            HydrationOutcome::Untrusted => untrusted_count += 1,
        }
        if diagnostic.blocks_share {
            blocking_artifact_tokens.push(format!(
                "{}:{}",
                diagnostic.artifact_kind.as_str(),
                diagnostic.artifact_id
            ));
        }
    }

    let share_blocked = untrusted_count > 0;
    let review_state = if share_blocked {
        ReviewState::Blocked
    } else {
        ReviewState::PendingReview
    };
    let headline = if share_blocked {
        format!(
            "Bundle {} ({} channel) is blocked: {} untrusted, {} degraded, {} not reusable, {} trusted across {} artifact(s).",
            bundle.bundle_id,
            channel.as_str(),
            untrusted_count,
            degraded_count,
            unreusable_count,
            trusted_count,
            diagnostics.len(),
        )
    } else {
        format!(
            "Bundle {} ({} channel) is clean and pending review: {} degraded, {} not reusable, {} trusted across {} artifact(s).",
            bundle.bundle_id,
            channel.as_str(),
            degraded_count,
            unreusable_count,
            trusted_count,
            diagnostics.len(),
        )
    };

    EnvDiagnosticsReport {
        record_kind: ENV_DIAGNOSTICS_REPORT_RECORD_KIND.to_owned(),
        schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
        bundle_id: bundle.bundle_id.clone(),
        source_channel: channel,
        provenance: bundle.provenance.clone(),
        diagnostics,
        trusted_count,
        degraded_count,
        unreusable_count,
        untrusted_count,
        blocking_artifact_tokens,
        share_blocked,
        review_state,
        headline,
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// The desktop diagnostics surface. Desktop reads the same
/// [`EnvDiagnosticsReport`] as every other surface.
pub fn desktop_env_diagnostics(bundle: &EnvArtifactBundle) -> EnvDiagnosticsReport {
    diagnose_bundle(bundle)
}

/// The headless / CLI diagnostics surface. Headless reads the same
/// [`EnvDiagnosticsReport`] as every other surface.
pub fn headless_env_diagnostics(bundle: &EnvArtifactBundle) -> EnvDiagnosticsReport {
    diagnose_bundle(bundle)
}

/// The support diagnostics surface. The report is already metadata-first,
/// so support reads the same object desktop and headless read, preserving
/// the review-before-share posture.
pub fn support_env_diagnostics(bundle: &EnvArtifactBundle) -> EnvDiagnosticsReport {
    diagnose_bundle(bundle)
}

// ---------------------------------------------------------------------------
// Project Doctor probe projection.
// ---------------------------------------------------------------------------

/// One Project-Doctor-shaped probe projected from a hydration diagnostic.
/// Doctor reads these to explain an environment-hydration failure directly,
/// from the same finding code and evidence the support report carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvDoctorProbe {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable probe id (one family per artifact kind).
    pub probe_id: String,
    /// The finding code the probe reports.
    pub finding_code: FindingCode,
    /// Family of the affected artifact.
    pub artifact_kind: ArtifactKind,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Source channel the artifact arrived over.
    pub source_channel: SourceChannel,
    /// Probe severity.
    pub severity: ProbeSeverity,
    /// Metadata refs backing the probe.
    pub evidence_refs: Vec<String>,
    /// Review-safe explanation of the finding.
    pub explanation: String,
    /// Exact recovery or escalation path.
    pub recovery_path: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

fn probe_from_diagnostic(diagnostic: &EnvHydrationDiagnostic) -> EnvDoctorProbe {
    EnvDoctorProbe {
        record_kind: ENV_DOCTOR_PROBE_RECORD_KIND.to_owned(),
        schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
        probe_id: format!("probe.env.{}.hydration", diagnostic.artifact_kind.as_str()),
        finding_code: diagnostic.finding_code,
        artifact_kind: diagnostic.artifact_kind,
        artifact_id: diagnostic.artifact_id.clone(),
        source_channel: diagnostic.source_channel,
        severity: ProbeSeverity::for_outcome(diagnostic.outcome),
        evidence_refs: diagnostic.evidence_refs.clone(),
        explanation: diagnostic.finding_code.explanation().to_owned(),
        recovery_path: diagnostic.next_step.clone(),
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// Projects a bundle's diagnostics into Project-Doctor probes, one per
/// artifact, in the same order as the report.
pub fn doctor_env_probes(bundle: &EnvArtifactBundle) -> Vec<EnvDoctorProbe> {
    diagnose_bundle(bundle)
        .diagnostics
        .iter()
        .map(probe_from_diagnostic)
        .collect()
}

// ---------------------------------------------------------------------------
// Schema-version / source-channel comparison.
// ---------------------------------------------------------------------------

/// How an artifact changed between two bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvArtifactChangeKind {
    /// The artifact exists only in the target bundle.
    Added,
    /// The artifact exists only in the base bundle.
    Removed,
    /// A field changed between the two bundles.
    Changed,
}

impl EnvArtifactChangeKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

/// One artifact-level delta between two bundles. Values are metadata tokens
/// (ids, digests, schema versions, state tokens), never bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvArtifactDelta {
    /// Family of the artifact.
    pub artifact_kind: ArtifactKind,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Kind of change.
    pub change_kind: EnvArtifactChangeKind,
    /// Name of the changed field (e.g. `schema_version`, `digest`, `state`).
    pub field: String,
    /// Metadata token before the change (empty for additions).
    pub before: String,
    /// Metadata token after the change (empty for removals).
    pub after: String,
}

/// The comparison between two environment-artifact bundles, across schema
/// versions and source channels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvBundleComparison {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Base bundle id.
    pub base_bundle_id: String,
    /// Target bundle id.
    pub target_bundle_id: String,
    /// Base source channel.
    pub base_source_channel: SourceChannel,
    /// Target source channel.
    pub target_source_channel: SourceChannel,
    /// True when the two bundles were produced on different channels.
    pub source_channel_changed: bool,
    /// Base env-artifacts schema version.
    pub base_schema_version: u32,
    /// Target env-artifacts schema version.
    pub target_schema_version: u32,
    /// True when the two bundles share a compatible env-artifacts schema
    /// version.
    pub schema_version_compatible: bool,
    /// True when the two bundles are artifact-identical.
    pub identical: bool,
    /// Ordered artifact-level deltas.
    pub deltas: Vec<EnvArtifactDelta>,
    /// Review-safe summary of the comparison.
    pub summary: String,
}

struct ArtifactDescriptor {
    kind: ArtifactKind,
    id: String,
    schema_version: u32,
    digest_token: String,
    state_token: String,
}

fn bundle_descriptors(bundle: &EnvArtifactBundle) -> Vec<ArtifactDescriptor> {
    let mut out = Vec::with_capacity(bundle.artifact_count());
    for export in &bundle.capsules {
        out.push(ArtifactDescriptor {
            kind: ArtifactKind::Capsule,
            id: export.capsule_id.clone(),
            schema_version: export.schema_version,
            digest_token: export.capsule_digest.value.clone(),
            state_token: export.inspection.verdict.as_str().to_owned(),
        });
    }
    for export in &bundle.templates {
        out.push(ArtifactDescriptor {
            kind: ArtifactKind::Template,
            id: export.template_id.clone(),
            schema_version: export.schema_version,
            digest_token: export.template_digest.value.clone(),
            state_token: export.inspection.verdict.as_str().to_owned(),
        });
    }
    for export in &bundle.prebuilds {
        out.push(ArtifactDescriptor {
            kind: ArtifactKind::Prebuild,
            id: export.snapshot_id.clone(),
            schema_version: export.schema_version,
            digest_token: export.warm_start_posture.as_str().to_owned(),
            state_token: export.outcome.as_str().to_owned(),
        });
    }
    for export in &bundle.runtimes {
        out.push(ArtifactDescriptor {
            kind: ArtifactKind::Runtime,
            id: export.instance_id.clone(),
            schema_version: export.schema_version,
            digest_token: export.observed_target_class.as_str().to_owned(),
            state_token: export.parity.as_str().to_owned(),
        });
    }
    out
}

fn push_delta(
    deltas: &mut Vec<EnvArtifactDelta>,
    kind: ArtifactKind,
    id: &str,
    field: &str,
    before: String,
    after: String,
) {
    if before != after {
        deltas.push(EnvArtifactDelta {
            artifact_kind: kind,
            artifact_id: id.to_owned(),
            change_kind: EnvArtifactChangeKind::Changed,
            field: field.to_owned(),
            before,
            after,
        });
    }
}

/// Compares two bundles across schema versions and source channels,
/// surfacing added, removed, and changed artifacts as metadata tokens. This
/// is the versioned-compare path: a schema-version drift or a channel
/// change is explicit rather than implied by an opaque re-import.
pub fn compare_env_bundles(
    base: &EnvArtifactBundle,
    target: &EnvArtifactBundle,
) -> EnvBundleComparison {
    let base_descriptors = bundle_descriptors(base);
    let target_descriptors = bundle_descriptors(target);
    let target_keys: BTreeSet<(ArtifactKind, &str)> = target_descriptors
        .iter()
        .map(|d| (d.kind, d.id.as_str()))
        .collect();
    let base_keys: BTreeSet<(ArtifactKind, &str)> = base_descriptors
        .iter()
        .map(|d| (d.kind, d.id.as_str()))
        .collect();

    let mut deltas = Vec::new();
    for base_descriptor in &base_descriptors {
        match target_descriptors
            .iter()
            .find(|d| d.kind == base_descriptor.kind && d.id == base_descriptor.id)
        {
            Some(target_descriptor) => {
                push_delta(
                    &mut deltas,
                    base_descriptor.kind,
                    &base_descriptor.id,
                    "schema_version",
                    base_descriptor.schema_version.to_string(),
                    target_descriptor.schema_version.to_string(),
                );
                push_delta(
                    &mut deltas,
                    base_descriptor.kind,
                    &base_descriptor.id,
                    "digest",
                    base_descriptor.digest_token.clone(),
                    target_descriptor.digest_token.clone(),
                );
                push_delta(
                    &mut deltas,
                    base_descriptor.kind,
                    &base_descriptor.id,
                    "state",
                    base_descriptor.state_token.clone(),
                    target_descriptor.state_token.clone(),
                );
            }
            None => deltas.push(EnvArtifactDelta {
                artifact_kind: base_descriptor.kind,
                artifact_id: base_descriptor.id.clone(),
                change_kind: EnvArtifactChangeKind::Removed,
                field: "artifact".to_owned(),
                before: base_descriptor.state_token.clone(),
                after: String::new(),
            }),
        }
    }
    for target_descriptor in &target_descriptors {
        if !base_keys.contains(&(target_descriptor.kind, target_descriptor.id.as_str())) {
            deltas.push(EnvArtifactDelta {
                artifact_kind: target_descriptor.kind,
                artifact_id: target_descriptor.id.clone(),
                change_kind: EnvArtifactChangeKind::Added,
                field: "artifact".to_owned(),
                before: String::new(),
                after: target_descriptor.state_token.clone(),
            });
        }
    }
    let _ = target_keys;

    let base_channel = base.provenance.source_channel;
    let target_channel = target.provenance.source_channel;
    let schema_version_compatible =
        base.provenance.schema_version == target.provenance.schema_version;
    let identical = deltas.is_empty() && schema_version_compatible;
    let summary = if identical {
        format!(
            "Bundles {} and {} are artifact-identical at env-artifacts schema version {}.",
            base.bundle_id, target.bundle_id, base.provenance.schema_version
        )
    } else {
        format!(
            "{} artifact delta(s) between bundles {} ({} channel) and {} ({} channel); schema versions {} -> {} are {}.",
            deltas.len(),
            base.bundle_id,
            base_channel.as_str(),
            target.bundle_id,
            target_channel.as_str(),
            base.provenance.schema_version,
            target.provenance.schema_version,
            if schema_version_compatible {
                "compatible"
            } else {
                "incompatible"
            },
        )
    };

    EnvBundleComparison {
        record_kind: ENV_BUNDLE_COMPARISON_RECORD_KIND.to_owned(),
        schema_version: ENV_DIAGNOSTICS_SCHEMA_VERSION,
        base_bundle_id: base.bundle_id.clone(),
        target_bundle_id: target.bundle_id.clone(),
        base_source_channel: base_channel,
        target_source_channel: target_channel,
        source_channel_changed: base_channel != target_channel,
        base_schema_version: base.provenance.schema_version,
        target_schema_version: target.provenance.schema_version,
        schema_version_compatible,
        identical,
        deltas,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Import / validation.
// ---------------------------------------------------------------------------

fn violation(report: &mut ValidationReport, check_id: &'static str, message: impl Into<String>) {
    report.violations.push(ValidationViolation {
        check_id,
        message: message.into(),
    });
}

/// Validates a bundle's structural contract: record kind, schema version,
/// provenance completeness, the metadata-first redaction posture, and that
/// it carries at least one artifact. A mirror channel must name its origin.
pub fn validate_env_artifact_bundle(bundle: &EnvArtifactBundle) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if bundle.record_kind != ENV_ARTIFACT_BUNDLE_RECORD_KIND {
        violation(
            &mut report,
            "bundle.record_kind",
            "bundle record_kind does not match the frozen token",
        );
    }
    if bundle.schema_version != ENV_DIAGNOSTICS_SCHEMA_VERSION {
        violation(
            &mut report,
            "bundle.schema_version",
            "bundle schema_version must be 1",
        );
    }
    if bundle.bundle_id.trim().is_empty() {
        violation(&mut report, "bundle.id", "bundle must carry a stable id");
    }
    if bundle.artifact_count() == 0 {
        violation(
            &mut report,
            "bundle.artifacts",
            "bundle must carry at least one artifact",
        );
    }

    let provenance = &bundle.provenance;
    if provenance.schema_version != ENV_DIAGNOSTICS_SCHEMA_VERSION {
        violation(
            &mut report,
            "bundle.provenance.schema_version",
            "provenance schema_version must match the bundle",
        );
    }
    if provenance.redaction_class != RedactionClass::MetadataOnly {
        violation(
            &mut report,
            "bundle.provenance.redaction_class",
            "provenance must declare a metadata-only redaction class",
        );
    }
    if provenance.producer_build_ref.trim().is_empty() {
        violation(
            &mut report,
            "bundle.provenance.producer_build_ref",
            "provenance must cite a producer build ref",
        );
    }
    if provenance.source_truth.trim().is_empty() {
        violation(
            &mut report,
            "bundle.provenance.source_truth",
            "provenance must carry a review-safe source-truth label",
        );
    }
    if provenance.captured_ref.trim().is_empty() {
        violation(
            &mut report,
            "bundle.provenance.captured_ref",
            "provenance must cite a capture ref",
        );
    }
    if provenance.source_channel == SourceChannel::Mirror
        && provenance.mirror_origin_ref.trim().is_empty()
    {
        violation(
            &mut report,
            "bundle.provenance.mirror_origin_ref",
            "a mirror-channel bundle must name its mirror origin ref",
        );
    }

    // Every contained export must be metadata-first.
    for export in &bundle.capsules {
        if export.redaction_class != RedactionClass::MetadataOnly {
            violation(
                &mut report,
                "bundle.capsule.redaction_class",
                format!("capsule {} export must be metadata-only", export.capsule_id),
            );
        }
    }
    for export in &bundle.templates {
        if export.redaction_class != RedactionClass::MetadataOnly {
            violation(
                &mut report,
                "bundle.template.redaction_class",
                format!(
                    "template {} export must be metadata-only",
                    export.template_id
                ),
            );
        }
    }
    for export in &bundle.prebuilds {
        if export.redaction_class != RedactionClass::MetadataOnly {
            violation(
                &mut report,
                "bundle.prebuild.redaction_class",
                format!(
                    "prebuild {} export must be metadata-only",
                    export.snapshot_id
                ),
            );
        }
    }
    for export in &bundle.runtimes {
        if export.redaction_class != RedactionClass::MetadataOnly {
            violation(
                &mut report,
                "bundle.runtime.redaction_class",
                format!(
                    "runtime {} export must be metadata-only",
                    export.instance_id
                ),
            );
        }
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Imports a bundle: validates its contract, then folds it into a
/// diagnostics report so the importer immediately sees why each artifact is
/// or is not trusted. The same vocabulary is used regardless of whether the
/// bundle arrived online, over a mirror, or fully offline.
pub fn import_env_bundle(
    bundle: &EnvArtifactBundle,
) -> Result<EnvDiagnosticsReport, ValidationReport> {
    validate_env_artifact_bundle(bundle)?;
    Ok(diagnose_bundle(bundle))
}

// ---------------------------------------------------------------------------
// Fixture record.
// ---------------------------------------------------------------------------

/// One checked-in fixture: a bundle of a given source channel plus the
/// diagnosis outcome the engine must reach for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvDiagnosticsFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Source channel the fixture exercises.
    pub source_channel: SourceChannel,
    /// The bundle under test.
    pub bundle: EnvArtifactBundle,
    /// Expected per-artifact finding codes, in report order.
    pub expected_finding_codes: Vec<FindingCode>,
    /// Expected share-blocked flag.
    pub expected_share_blocked: bool,
    /// Expected review state.
    pub expected_review_state: ReviewState,
    /// One consumer surface that ingests this bundle.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// Validates a checked-in fixture: the bundle itself, and that the recorded
/// expectations equal what the diagnostics engine computes.
pub fn validate_env_diagnostics_fixture(
    fixture: &EnvDiagnosticsFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != ENV_DIAGNOSTICS_FIXTURE_RECORD_KIND {
        violation(
            &mut report,
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != ENV_DIAGNOSTICS_SCHEMA_VERSION {
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
    if fixture.source_channel != fixture.bundle.provenance.source_channel {
        violation(
            &mut report,
            "fixture.source_channel",
            format!(
                "fixture {} source channel disagrees with the bundle provenance",
                fixture.fixture_id
            ),
        );
    }

    if let Err(bundle_report) = validate_env_artifact_bundle(&fixture.bundle) {
        for inner in bundle_report.violations {
            report.violations.push(inner);
        }
    }

    let computed = diagnose_bundle(&fixture.bundle);
    let computed_codes: Vec<FindingCode> = computed
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.finding_code)
        .collect();
    if fixture.expected_finding_codes != computed_codes {
        violation(
            &mut report,
            "fixture.expected_finding_codes",
            format!(
                "fixture {} expected finding codes disagree with the engine",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_share_blocked != computed.share_blocked {
        violation(
            &mut report,
            "fixture.expected_share_blocked",
            format!(
                "fixture {} expected share-blocked {} disagrees with the engine ({})",
                fixture.fixture_id, fixture.expected_share_blocked, computed.share_blocked
            ),
        );
    }
    if fixture.expected_review_state != computed.review_state {
        violation(
            &mut report,
            "fixture.expected_review_state",
            format!(
                "fixture {} expected review state {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_review_state.as_str(),
                computed.review_state.as_str()
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

pub use seed::{
    seeded_env_artifact_bundles, seeded_env_diagnostics_fixtures, seeded_env_diagnostics_reports,
};

#[cfg(test)]
mod tests;
