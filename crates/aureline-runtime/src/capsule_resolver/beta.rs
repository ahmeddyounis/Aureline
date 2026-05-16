//! Beta extension to the environment-capsule resolver.
//!
//! The alpha resolver in [`super`] inspects metadata-only signals (Node and
//! Python detector reports) and never reads devcontainer, Nix, or Compose
//! bodies. The beta layer extends that resolver so users can inspect how
//! Aureline derived runtime truth from declarative inputs and how conflicts
//! between those inputs were resolved.
//!
//! The beta resolver:
//!
//! * Detects and parses `devcontainer.json`, `docker-compose.yml`, and Nix
//!   files (`flake.nix`, `shell.nix`, `default.nix`) without executing any
//!   repository-owned hook.
//! * Labels every parsed source with one of three confidence classes —
//!   `imported` (clean structured parse), `heuristic` (partial parse or
//!   inferred body), `unsupported` (file class the contract does not parse).
//! * Picks a single primary source through a closed precedence ladder
//!   (devcontainer > compose > nix > node/python detector) and records every
//!   precedence row alongside the result so reviewers can replay the
//!   decision.
//! * Mints a capsule reference whose hash is the digest of the source-set
//!   plus the parsed-field tokens, so a content change advances the hash and
//!   a downstream ticket-drift evaluator invalidates stale stored bindings.
//! * Exposes a typed [`evaluate_capsule_drift`] evaluator that compares a
//!   stored source-set digest against a freshly resolved beta resolution and
//!   classifies the drift (`stale_inputs` / `manually_diverged`).
//!
//! The resolver is read-only: it never spawns a container, it never runs a
//! Nix evaluator, and it never executes a lifecycle hook. Raw command lines
//! and raw secret values do not cross the beta boundary; only structured
//! tokens, content digests, and source-class labels do.
//!
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/environment_capsules_beta.md`](../../../../docs/runtime/m3/environment_capsules_beta.md).
//! The cross-tool boundary schema is
//! [`/schemas/runtime/environment_capsule_beta.schema.json`](../../../../schemas/runtime/environment_capsule_beta.schema.json).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{
    digest_token, EnvironmentCapsuleHint, EnvironmentCapsuleResolution, EnvironmentCapsuleResolver,
    EnvironmentCapsuleResolverConfig, ProjectArchetypeHint,
};
use crate::execution_context::{CapsuleDriftState, EnvironmentCapsuleRef, PrebuildReuseState};

/// Stable record-kind tag for the beta capsule resolution payload.
pub const ENVIRONMENT_CAPSULE_BETA_RESOLUTION_RECORD_KIND: &str =
    "environment_capsule_beta_resolution_record";

/// Stable record-kind tag for the beta drift evaluation record.
pub const ENVIRONMENT_CAPSULE_BETA_DRIFT_RECORD_KIND: &str =
    "environment_capsule_beta_drift_record";

/// Stable record-kind tag for the beta support-export packet.
pub const ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "environment_capsule_beta_support_export_record";

/// Stable record-kind tag for the beta source-coverage manifest.
pub const ENVIRONMENT_CAPSULE_BETA_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "environment_capsule_beta_coverage_manifest_record";

/// Schema version for the beta resolver records.
pub const ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION: u32 = 1;

/// Beta resolver implementation token recorded on every resolution.
pub const ENVIRONMENT_CAPSULE_BETA_RESOLVER_VERSION: &str =
    "environment_capsule_resolver.beta.v1";

/// Closed source vocabulary the beta resolver classifies declarative inputs
/// against. Every source the resolver inspects projects onto exactly one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaSourceClass {
    /// `devcontainer.json` (or `.devcontainer/devcontainer.json`).
    Devcontainer,
    /// `docker-compose.yml` / `compose.yml` body.
    DockerCompose,
    /// `flake.nix` declarative input.
    NixFlake,
    /// `shell.nix` declarative input.
    NixShell,
    /// `default.nix` declarative input.
    NixDefault,
    /// Node manifest (`package.json` plus lockfile family).
    NodeManifest,
    /// Python manifest (`pyproject.toml`, `.python-version`).
    PythonManifest,
}

impl CapsuleBetaSourceClass {
    /// All source classes the beta resolver knows about.
    pub const ALL: [Self; 7] = [
        Self::Devcontainer,
        Self::DockerCompose,
        Self::NixFlake,
        Self::NixShell,
        Self::NixDefault,
        Self::NodeManifest,
        Self::PythonManifest,
    ];

    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Devcontainer => "devcontainer",
            Self::DockerCompose => "docker_compose",
            Self::NixFlake => "nix_flake",
            Self::NixShell => "nix_shell",
            Self::NixDefault => "nix_default",
            Self::NodeManifest => "node_manifest",
            Self::PythonManifest => "python_manifest",
        }
    }

    /// Precedence rank — lower wins. Devcontainer is the most explicit
    /// declarative input and wins over Compose, Nix, and detector signals.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::Devcontainer => 0,
            Self::DockerCompose => 1,
            Self::NixFlake => 2,
            Self::NixShell => 3,
            Self::NixDefault => 4,
            Self::NodeManifest => 5,
            Self::PythonManifest => 6,
        }
    }

    /// Default confidence assigned by the parser when a source is recognised
    /// but the beta contract does not promise full body parsing for it.
    /// Nix files fall here because the resolver does not embed a Nix
    /// evaluator.
    pub const fn default_confidence(self) -> CapsuleBetaSourceConfidence {
        match self {
            Self::Devcontainer | Self::DockerCompose => CapsuleBetaSourceConfidence::Imported,
            Self::NixFlake | Self::NixShell | Self::NixDefault => {
                CapsuleBetaSourceConfidence::Unsupported
            }
            Self::NodeManifest | Self::PythonManifest => CapsuleBetaSourceConfidence::Imported,
        }
    }
}

/// Confidence label stamped on every parsed source so reviewers can tell at a
/// glance whether the resolver ingested a clean structured body, a heuristic
/// fallback, or a file class the contract intentionally does not parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaSourceConfidence {
    /// Body parsed cleanly into structured tokens.
    Imported,
    /// Body parsed but at least one field had to fall back to a heuristic
    /// because the body was malformed or the contract does not promise the
    /// field shape.
    Heuristic,
    /// File class is recognised but the contract does not parse the body.
    /// Drift detection still tracks the content digest.
    Unsupported,
}

impl CapsuleBetaSourceConfidence {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Closed reasons a parsed source carries a `heuristic` or `unsupported`
/// confidence label. Empty when the source parsed cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaSourceNote {
    /// Body could not be parsed against the expected JSON / YAML grammar.
    BodyUnparseable,
    /// Body parsed but required field was missing or empty.
    RequiredFieldMissing,
    /// Body referenced a sibling source the resolver could not locate.
    DependentSourceMissing,
    /// Beta contract does not parse this source body; drift tracking still
    /// applies via the content digest.
    UnsupportedBodyParse,
    /// Source body conflicted with another higher-precedence source; only the
    /// higher-precedence body shaped the primary resolution.
    OverriddenByHigherPrecedence,
    /// Body declared a feature outside the beta vocabulary.
    UnknownFieldKept,
}

impl CapsuleBetaSourceNote {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BodyUnparseable => "body_unparseable",
            Self::RequiredFieldMissing => "required_field_missing",
            Self::DependentSourceMissing => "dependent_source_missing",
            Self::UnsupportedBodyParse => "unsupported_body_parse",
            Self::OverriddenByHigherPrecedence => "overridden_by_higher_precedence",
            Self::UnknownFieldKept => "unknown_field_kept",
        }
    }
}

/// Structured body of a parsed devcontainer source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DevcontainerParsedFields {
    /// Pinned image reference, if declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Dockerfile body referenced by the profile, if declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dockerfile_ref: Option<String>,
    /// Compose file referenced by the profile, if declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose_file_ref: Option<String>,
    /// Compose service the profile selects, if declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose_service: Option<String>,
    /// Names of declared features (no values).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_keys: Vec<String>,
    /// Number of forwarded ports declared.
    pub forward_port_count: u32,
    /// Lifecycle hook keys declared (no command bodies).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifecycle_hook_keys: Vec<String>,
}

/// Structured body of a parsed Compose source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposeParsedFields {
    /// Names of services declared.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_keys: Vec<String>,
    /// Whether the compose body declares at least one image-based service.
    pub has_image_service: bool,
    /// Whether the compose body declares at least one build-based service.
    pub has_build_service: bool,
}

/// Structured body of a Nix source. The beta contract does not embed a Nix
/// evaluator, so the body remains opaque; the digest still pins the source
/// for drift detection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NixParsedFields {
    /// Stable variant token (`flake`, `shell`, `default`).
    pub variant_token: String,
}

/// Structured body of a Node manifest source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeParsedFields {
    /// Whether `package.json` was present at the workspace root.
    pub has_package_json: bool,
    /// Lockfile families discovered (e.g. `package-lock.json`, `pnpm-lock.yaml`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lockfile_refs: Vec<String>,
}

/// Structured body of a Python manifest source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonParsedFields {
    /// Whether `pyproject.toml` was present.
    pub has_pyproject: bool,
    /// Whether `.python-version` was present.
    pub has_python_version: bool,
    /// Lockfile families discovered (e.g. `uv.lock`, `poetry.lock`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lockfile_refs: Vec<String>,
}

/// Tagged union of structured parsed-field bodies. The variant is keyed by
/// the source class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CapsuleBetaParsedFields {
    Devcontainer(DevcontainerParsedFields),
    DockerCompose(ComposeParsedFields),
    Nix(NixParsedFields),
    NodeManifest(NodeParsedFields),
    PythonManifest(PythonParsedFields),
}

/// One parsed source row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSourceParse {
    /// Source class.
    pub source_class: CapsuleBetaSourceClass,
    /// Stable source-class token.
    pub source_class_token: String,
    /// Workspace-relative reference to the parsed body.
    pub source_ref: String,
    /// Content digest of the body bytes at parse time.
    pub content_digest: String,
    /// Confidence label.
    pub confidence: CapsuleBetaSourceConfidence,
    /// Stable confidence token.
    pub confidence_token: String,
    /// Closed-vocabulary notes describing why the confidence label was set
    /// (empty when `confidence` is `imported` and no body field was missing).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<CapsuleBetaSourceNote>,
    /// Structured parsed-field body.
    pub parsed_fields: CapsuleBetaParsedFields,
}

/// One row in the precedence ladder the resolver consulted while resolving a
/// workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaPrecedenceRow {
    /// Source class.
    pub source_class: CapsuleBetaSourceClass,
    /// Stable source-class token.
    pub source_class_token: String,
    /// Precedence rank — lower wins.
    pub rank: u8,
    /// Whether the resolver actually parsed this source for this workspace.
    pub source_present: bool,
    /// Whether this source shaped the primary capsule binding.
    pub winner: bool,
}

/// Beta capsule resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleBetaResolution {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Beta resolver implementation token.
    pub resolver_version: String,
    /// Workspace root inspected by this report.
    pub workspace_root_ref: String,
    /// Caller-supplied project archetype hint.
    pub archetype_hint: ProjectArchetypeHint,
    /// Underlying alpha resolution carried verbatim so reviewers can compare
    /// alpha and beta outputs side-by-side without re-resolving.
    pub alpha_resolution: EnvironmentCapsuleResolution,
    /// Sources discovered, parsed, and stamped with confidence labels.
    pub sources: Vec<CapsuleBetaSourceParse>,
    /// Precedence ladder consulted while picking a primary source.
    pub precedence: Vec<CapsuleBetaPrecedenceRow>,
    /// Source class that shaped the primary capsule binding (empty when no
    /// source was present and the alpha NoSignal capsule is the primary).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_source: Option<CapsuleBetaSourceClass>,
    /// Stable token for [`Self::primary_source`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_source_token: Option<String>,
    /// Closed-vocabulary conflict notes recorded when more than one source
    /// claimed authority for the same field family.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflict_notes: Vec<CapsuleBetaSourceNote>,
    /// Aggregated digest over the parsed-source set.
    pub source_set_digest: String,
    /// Capsule drift state — beta resolver folds parse-time presence and
    /// digests into one of the canonical drift labels so downstream
    /// consumers do not invent their own.
    pub drift_state: CapsuleDriftState,
    /// Beta capsule reference. The hash is bound to the source-set digest so
    /// any content change advances the hash and a downstream ticket-drift
    /// evaluator invalidates the stored binding.
    pub environment_capsule_ref: EnvironmentCapsuleRef,
    /// Prebuild reuse state — `Candidate` when at least one source was
    /// parsed, `NotApplicable` when no source was present.
    pub prebuild_reuse_state: PrebuildReuseState,
}

/// Closed-vocabulary outcomes [`evaluate_capsule_drift`] returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaDriftOutcome {
    /// Stored source-set digest matches the freshly resolved digest.
    InSync,
    /// At least one source body changed content.
    StaleInputs,
    /// Sources were added or removed since the stored snapshot.
    ManuallyDiverged,
    /// Stored snapshot referenced no sources; the freshly resolved snapshot
    /// has no prior baseline to compare against.
    UnknownLineage,
}

impl CapsuleBetaDriftOutcome {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::StaleInputs => "stale_inputs",
            Self::ManuallyDiverged => "manually_diverged",
            Self::UnknownLineage => "unknown_lineage",
        }
    }

    /// Project the outcome onto the canonical [`CapsuleDriftState`].
    pub const fn to_capsule_drift_state(self) -> CapsuleDriftState {
        match self {
            Self::InSync => CapsuleDriftState::InSync,
            Self::StaleInputs => CapsuleDriftState::StaleInputs,
            Self::ManuallyDiverged => CapsuleDriftState::ManuallyDiverged,
            Self::UnknownLineage => CapsuleDriftState::UnknownLineage,
        }
    }
}

/// Per-source row recorded by the drift evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaDriftRow {
    /// Source class.
    pub source_class: CapsuleBetaSourceClass,
    /// Stable source-class token.
    pub source_class_token: String,
    /// Stored content digest, when the source was present in the baseline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stored_content_digest: Option<String>,
    /// Fresh content digest, when the source is present today.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fresh_content_digest: Option<String>,
}

/// Stored baseline a caller compares against the fresh resolution. Persisting
/// this projection alongside an approval ticket or rerun snapshot lets the
/// runtime invalidate stale bindings the moment a declarative input changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSourceBaseline {
    /// Stored aggregated digest over the source set.
    pub source_set_digest: String,
    /// Per-source rows captured at baseline time.
    pub source_rows: Vec<CapsuleBetaDriftRow>,
}

impl CapsuleBetaSourceBaseline {
    /// Capture the baseline from a beta resolution.
    pub fn from_resolution(resolution: &EnvironmentCapsuleBetaResolution) -> Self {
        let source_rows = resolution
            .sources
            .iter()
            .map(|src| CapsuleBetaDriftRow {
                source_class: src.source_class,
                source_class_token: src.source_class_token.clone(),
                stored_content_digest: Some(src.content_digest.clone()),
                fresh_content_digest: Some(src.content_digest.clone()),
            })
            .collect();
        Self {
            source_set_digest: resolution.source_set_digest.clone(),
            source_rows,
        }
    }
}

/// Drift evaluation record. Replays into the support-export packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleBetaDriftEvaluation {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stored aggregated digest.
    pub stored_source_set_digest: String,
    /// Fresh aggregated digest.
    pub fresh_source_set_digest: String,
    /// Outcome.
    pub outcome: CapsuleBetaDriftOutcome,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Per-source drift rows (only sources that differ are present).
    pub drift_rows: Vec<CapsuleBetaDriftRow>,
    /// Source classes added in the fresh snapshot.
    pub added_sources: Vec<CapsuleBetaSourceClass>,
    /// Source classes removed in the fresh snapshot.
    pub removed_sources: Vec<CapsuleBetaSourceClass>,
}

impl EnvironmentCapsuleBetaDriftEvaluation {
    /// True when the outcome is anything other than `InSync`.
    pub fn is_drifted(&self) -> bool {
        !matches!(self.outcome, CapsuleBetaDriftOutcome::InSync)
    }
}

/// One coverage row in the beta source manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSourceCoverageRow {
    /// Source class.
    pub source_class: CapsuleBetaSourceClass,
    /// Stable source-class token.
    pub source_class_token: String,
    /// Precedence rank.
    pub rank: u8,
    /// Default confidence.
    pub default_confidence: CapsuleBetaSourceConfidence,
    /// Stable token for [`Self::default_confidence`].
    pub default_confidence_token: String,
}

/// Coverage manifest pinning the canonical beta source vocabulary and
/// precedence rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleBetaCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Canonical source-class rows ordered by precedence rank.
    pub source_classes: Vec<CapsuleBetaSourceCoverageRow>,
}

impl EnvironmentCapsuleBetaCoverageManifest {
    /// Builds the canonical coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        let mut source_classes: Vec<_> = CapsuleBetaSourceClass::ALL
            .into_iter()
            .map(|class| CapsuleBetaSourceCoverageRow {
                source_class: class,
                source_class_token: class.as_str().to_owned(),
                rank: class.precedence_rank(),
                default_confidence: class.default_confidence(),
                default_confidence_token: class.default_confidence().as_str().to_owned(),
            })
            .collect();
        source_classes.sort_by_key(|row| row.rank);
        Self {
            record_kind: ENVIRONMENT_CAPSULE_BETA_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            source_classes,
        }
    }

    /// True when every source class declared in the canonical vocabulary is
    /// represented by a coverage row.
    pub fn covers_every_source_class(&self) -> bool {
        for class in CapsuleBetaSourceClass::ALL {
            if !self.source_classes.iter().any(|row| row.source_class == class) {
                return false;
            }
        }
        true
    }
}

/// Beta support-export packet projecting the canonical coverage manifest, the
/// resolved beta resolution, and any drift evaluations the support flow
/// attached. Raw env values, raw command lines, and raw secrets are out of
/// scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Coverage manifest at export time.
    pub coverage_manifest: EnvironmentCapsuleBetaCoverageManifest,
    /// Resolution being exported.
    pub resolution: EnvironmentCapsuleBetaResolution,
    /// Drift evaluations carried alongside the export.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drift_evaluations: Vec<EnvironmentCapsuleBetaDriftEvaluation>,
}

impl EnvironmentCapsuleBetaSupportExport {
    /// Builds the support-export packet.
    pub fn new(
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
        resolution: EnvironmentCapsuleBetaResolution,
        drift_evaluations: Vec<EnvironmentCapsuleBetaDriftEvaluation>,
    ) -> Self {
        let manifest_id_owned = manifest_id.into();
        let generated_at_owned = generated_at.into();
        Self {
            record_kind: ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id_owned.clone(),
            generated_at: generated_at_owned.clone(),
            coverage_manifest: EnvironmentCapsuleBetaCoverageManifest::canonical(
                manifest_id_owned,
                generated_at_owned,
            ),
            resolution,
            drift_evaluations,
        }
    }
}

/// Caller-provided configuration for [`EnvironmentCapsuleBetaResolver`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentCapsuleBetaResolverConfig {
    /// Underlying alpha resolver configuration.
    pub alpha_config: EnvironmentCapsuleResolverConfig,
    /// Beta resolver implementation token recorded on the result.
    pub beta_resolver_version: String,
}

impl Default for EnvironmentCapsuleBetaResolverConfig {
    fn default() -> Self {
        Self {
            alpha_config: EnvironmentCapsuleResolverConfig::default(),
            beta_resolver_version: ENVIRONMENT_CAPSULE_BETA_RESOLVER_VERSION.to_owned(),
        }
    }
}

/// Read-only resolver that extends the alpha capsule resolver with
/// devcontainer, Nix, and Compose parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentCapsuleBetaResolver {
    inner: EnvironmentCapsuleResolver,
    config: EnvironmentCapsuleBetaResolverConfig,
}

impl EnvironmentCapsuleBetaResolver {
    /// Creates a beta resolver with the supplied configuration.
    pub fn new(config: EnvironmentCapsuleBetaResolverConfig) -> Self {
        let inner = EnvironmentCapsuleResolver::new(config.alpha_config.clone());
        Self { inner, config }
    }

    /// Creates a beta resolver with the default alpha configuration.
    pub fn default_read_only() -> Self {
        Self::new(EnvironmentCapsuleBetaResolverConfig::default())
    }

    /// Resolves a workspace into a beta resolution.
    pub fn resolve_workspace(
        &self,
        workspace_root: &Path,
        archetype_hint: ProjectArchetypeHint,
    ) -> EnvironmentCapsuleBetaResolution {
        let alpha = self
            .inner
            .resolve_workspace(workspace_root, archetype_hint);
        let mut sources = parse_workspace_sources(workspace_root);
        sources.sort_by(|a, b| a.source_class.precedence_rank().cmp(&b.source_class.precedence_rank()));

        let primary_source = sources
            .iter()
            .map(|src| src.source_class)
            .min_by_key(|class| class.precedence_rank());
        let conflict_notes = compute_conflict_notes(&sources, primary_source);
        if let Some(primary) = primary_source {
            for src in sources.iter_mut() {
                if src.source_class != primary && !src.notes.contains(&CapsuleBetaSourceNote::OverriddenByHigherPrecedence) {
                    src.notes
                        .push(CapsuleBetaSourceNote::OverriddenByHigherPrecedence);
                }
            }
        }

        let precedence = build_precedence_ladder(&sources, primary_source);
        let source_set_digest = compute_source_set_digest(&sources);
        let drift_state = if sources.is_empty() {
            CapsuleDriftState::UnknownLineage
        } else {
            CapsuleDriftState::InSync
        };
        let prebuild_reuse_state = if sources.is_empty() {
            PrebuildReuseState::NotApplicable
        } else {
            PrebuildReuseState::Candidate
        };

        let capsule_id = capsule_id_for_primary(primary_source, alpha.capsule_hint, archetype_hint);
        let capsule_hash = digest_token(&[
            "capsule.beta",
            capsule_id.as_str(),
            source_set_digest.as_str(),
            archetype_hint.as_str(),
        ]);
        let environment_capsule_ref = EnvironmentCapsuleRef {
            capsule_id: capsule_id.clone(),
            capsule_hash,
            resolved_schema_version: self.config.alpha_config.resolved_schema_version.clone(),
            drift_state,
        };

        EnvironmentCapsuleBetaResolution {
            record_kind: ENVIRONMENT_CAPSULE_BETA_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
            resolver_version: self.config.beta_resolver_version.clone(),
            workspace_root_ref: workspace_root.display().to_string(),
            archetype_hint,
            alpha_resolution: alpha,
            sources,
            precedence,
            primary_source,
            primary_source_token: primary_source.map(|s| s.as_str().to_owned()),
            conflict_notes,
            source_set_digest,
            drift_state,
            environment_capsule_ref,
            prebuild_reuse_state,
        }
    }
}

impl Default for EnvironmentCapsuleBetaResolver {
    fn default() -> Self {
        Self::default_read_only()
    }
}

/// Evaluates whether a stored capsule baseline is still in sync with a fresh
/// resolution. The outcome is closed and projects onto [`CapsuleDriftState`]
/// via [`CapsuleBetaDriftOutcome::to_capsule_drift_state`].
pub fn evaluate_capsule_drift(
    stored: &CapsuleBetaSourceBaseline,
    fresh: &EnvironmentCapsuleBetaResolution,
) -> EnvironmentCapsuleBetaDriftEvaluation {
    let mut drift_rows: Vec<CapsuleBetaDriftRow> = Vec::new();
    let mut added: Vec<CapsuleBetaSourceClass> = Vec::new();
    let mut removed: Vec<CapsuleBetaSourceClass> = Vec::new();

    let stored_by_class: BTreeMap<CapsuleBetaSourceClass, &CapsuleBetaDriftRow> = stored
        .source_rows
        .iter()
        .map(|row| (row.source_class, row))
        .collect();
    let fresh_by_class: BTreeMap<CapsuleBetaSourceClass, &CapsuleBetaSourceParse> = fresh
        .sources
        .iter()
        .map(|src| (src.source_class, src))
        .collect();

    for (class, src) in &fresh_by_class {
        match stored_by_class.get(class) {
            None => {
                added.push(*class);
                drift_rows.push(CapsuleBetaDriftRow {
                    source_class: *class,
                    source_class_token: class.as_str().to_owned(),
                    stored_content_digest: None,
                    fresh_content_digest: Some(src.content_digest.clone()),
                });
            }
            Some(stored_row) => {
                if stored_row.stored_content_digest.as_deref()
                    != Some(src.content_digest.as_str())
                {
                    drift_rows.push(CapsuleBetaDriftRow {
                        source_class: *class,
                        source_class_token: class.as_str().to_owned(),
                        stored_content_digest: stored_row.stored_content_digest.clone(),
                        fresh_content_digest: Some(src.content_digest.clone()),
                    });
                }
            }
        }
    }
    for (class, stored_row) in &stored_by_class {
        if !fresh_by_class.contains_key(class) {
            removed.push(*class);
            drift_rows.push(CapsuleBetaDriftRow {
                source_class: *class,
                source_class_token: class.as_str().to_owned(),
                stored_content_digest: stored_row.stored_content_digest.clone(),
                fresh_content_digest: None,
            });
        }
    }
    drift_rows.sort_by_key(|row| row.source_class.precedence_rank());
    added.sort_by_key(|c| c.precedence_rank());
    removed.sort_by_key(|c| c.precedence_rank());

    let outcome = if stored.source_rows.is_empty() && fresh.sources.is_empty() {
        CapsuleBetaDriftOutcome::UnknownLineage
    } else if !added.is_empty() || !removed.is_empty() {
        CapsuleBetaDriftOutcome::ManuallyDiverged
    } else if !drift_rows.is_empty() {
        CapsuleBetaDriftOutcome::StaleInputs
    } else {
        CapsuleBetaDriftOutcome::InSync
    };

    EnvironmentCapsuleBetaDriftEvaluation {
        record_kind: ENVIRONMENT_CAPSULE_BETA_DRIFT_RECORD_KIND.to_owned(),
        schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
        stored_source_set_digest: stored.source_set_digest.clone(),
        fresh_source_set_digest: fresh.source_set_digest.clone(),
        outcome,
        outcome_token: outcome.as_str().to_owned(),
        drift_rows,
        added_sources: added,
        removed_sources: removed,
    }
}

fn parse_workspace_sources(root: &Path) -> Vec<CapsuleBetaSourceParse> {
    let mut sources = Vec::new();
    if let Some(parse) = parse_devcontainer(root) {
        sources.push(parse);
    }
    if let Some(parse) = parse_compose(root) {
        sources.push(parse);
    }
    for parse in parse_nix(root) {
        sources.push(parse);
    }
    if let Some(parse) = parse_node(root) {
        sources.push(parse);
    }
    if let Some(parse) = parse_python(root) {
        sources.push(parse);
    }
    sources
}

fn parse_devcontainer(root: &Path) -> Option<CapsuleBetaSourceParse> {
    for candidate in ["devcontainer.json", ".devcontainer/devcontainer.json"] {
        let path = root.join(candidate);
        if path.is_file() {
            let bytes = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let digest = digest_token(&[
                "devcontainer.body",
                candidate,
                bytes_digest(&bytes).as_str(),
            ]);
            let body = String::from_utf8_lossy(&bytes).into_owned();
            let stripped = strip_jsonc_comments(&body);
            let mut notes = Vec::new();
            let (parsed_fields, confidence) = match serde_json::from_str::<serde_json::Value>(&stripped) {
                Ok(value) => {
                    let parsed = parse_devcontainer_value(&value, &mut notes);
                    let confidence = if notes.is_empty() {
                        CapsuleBetaSourceConfidence::Imported
                    } else {
                        CapsuleBetaSourceConfidence::Heuristic
                    };
                    (parsed, confidence)
                }
                Err(_) => {
                    notes.push(CapsuleBetaSourceNote::BodyUnparseable);
                    (DevcontainerParsedFields::default(), CapsuleBetaSourceConfidence::Heuristic)
                }
            };
            if parsed_fields.compose_file_ref.is_some() {
                let compose_path = root.join(parsed_fields.compose_file_ref.as_deref().unwrap());
                if !compose_path.is_file() {
                    notes.push(CapsuleBetaSourceNote::DependentSourceMissing);
                }
            }
            return Some(CapsuleBetaSourceParse {
                source_class: CapsuleBetaSourceClass::Devcontainer,
                source_class_token: CapsuleBetaSourceClass::Devcontainer.as_str().to_owned(),
                source_ref: candidate.to_owned(),
                content_digest: digest,
                confidence,
                confidence_token: confidence.as_str().to_owned(),
                notes,
                parsed_fields: CapsuleBetaParsedFields::Devcontainer(parsed_fields),
            });
        }
    }
    None
}

fn parse_devcontainer_value(
    value: &serde_json::Value,
    notes: &mut Vec<CapsuleBetaSourceNote>,
) -> DevcontainerParsedFields {
    let mut parsed = DevcontainerParsedFields::default();
    let object = match value.as_object() {
        Some(map) => map,
        None => {
            notes.push(CapsuleBetaSourceNote::RequiredFieldMissing);
            return parsed;
        }
    };
    if let Some(image) = object.get("image").and_then(|v| v.as_str()) {
        parsed.image_ref = Some(image.to_owned());
    }
    if let Some(dockerfile) = object.get("dockerFile").and_then(|v| v.as_str()) {
        parsed.dockerfile_ref = Some(dockerfile.to_owned());
    } else if let Some(build) = object.get("build").and_then(|v| v.as_object()) {
        if let Some(dockerfile) = build.get("dockerfile").and_then(|v| v.as_str()) {
            parsed.dockerfile_ref = Some(dockerfile.to_owned());
        }
    }
    if let Some(compose) = object.get("dockerComposeFile") {
        if let Some(path) = compose.as_str() {
            parsed.compose_file_ref = Some(path.to_owned());
        } else if let Some(arr) = compose.as_array() {
            if let Some(first) = arr.first().and_then(|v| v.as_str()) {
                parsed.compose_file_ref = Some(first.to_owned());
            }
        }
    }
    if let Some(service) = object.get("service").and_then(|v| v.as_str()) {
        parsed.compose_service = Some(service.to_owned());
    }
    if let Some(features) = object.get("features").and_then(|v| v.as_object()) {
        parsed.feature_keys = features.keys().cloned().collect();
        parsed.feature_keys.sort();
    } else if let Some(features) = object.get("features").and_then(|v| v.as_array()) {
        for feature in features {
            if let Some(s) = feature.as_str() {
                parsed.feature_keys.push(s.to_owned());
            }
        }
        parsed.feature_keys.sort();
    }
    if let Some(forward_ports) = object.get("forwardPorts").and_then(|v| v.as_array()) {
        parsed.forward_port_count = forward_ports.len() as u32;
    }
    for hook in [
        "onCreateCommand",
        "postCreateCommand",
        "postStartCommand",
        "postAttachCommand",
        "updateContentCommand",
        "initializeCommand",
        "waitFor",
    ] {
        if object.contains_key(hook) {
            parsed.lifecycle_hook_keys.push(hook.to_owned());
        }
    }
    parsed.lifecycle_hook_keys.sort();
    if parsed.image_ref.is_none()
        && parsed.dockerfile_ref.is_none()
        && parsed.compose_file_ref.is_none()
    {
        notes.push(CapsuleBetaSourceNote::RequiredFieldMissing);
    }
    parsed
}

fn parse_compose(root: &Path) -> Option<CapsuleBetaSourceParse> {
    for candidate in [
        "docker-compose.yml",
        "docker-compose.yaml",
        "compose.yml",
        "compose.yaml",
    ] {
        let path = root.join(candidate);
        if path.is_file() {
            let bytes = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let digest = digest_token(&[
                "compose.body",
                candidate,
                bytes_digest(&bytes).as_str(),
            ]);
            let body = String::from_utf8_lossy(&bytes).into_owned();
            let mut notes = Vec::new();
            let parsed = parse_compose_body(&body, &mut notes);
            let confidence = if parsed.service_keys.is_empty() {
                CapsuleBetaSourceConfidence::Heuristic
            } else if notes.is_empty() {
                CapsuleBetaSourceConfidence::Imported
            } else {
                CapsuleBetaSourceConfidence::Heuristic
            };
            return Some(CapsuleBetaSourceParse {
                source_class: CapsuleBetaSourceClass::DockerCompose,
                source_class_token: CapsuleBetaSourceClass::DockerCompose.as_str().to_owned(),
                source_ref: candidate.to_owned(),
                content_digest: digest,
                confidence,
                confidence_token: confidence.as_str().to_owned(),
                notes,
                parsed_fields: CapsuleBetaParsedFields::DockerCompose(parsed),
            });
        }
    }
    None
}

fn parse_compose_body(body: &str, notes: &mut Vec<CapsuleBetaSourceNote>) -> ComposeParsedFields {
    let mut parsed = ComposeParsedFields::default();
    let mut in_services = false;
    let mut services_indent: Option<usize> = None;
    let mut current_service: Option<String> = None;
    let mut current_service_indent: Option<usize> = None;
    for line in body.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() || trimmed.trim_start().starts_with('#') {
            continue;
        }
        let indent = trimmed.len() - trimmed.trim_start().len();
        let content = trimmed.trim_start();

        if !in_services {
            if content.starts_with("services:") && indent == 0 {
                in_services = true;
            }
            continue;
        }

        // Exiting services block when a sibling top-level key shows up.
        if indent == 0 && !content.starts_with("services:") {
            in_services = false;
            current_service = None;
            current_service_indent = None;
            continue;
        }
        if services_indent.is_none() && indent > 0 {
            services_indent = Some(indent);
        }
        let service_indent = services_indent.unwrap_or(2);
        if indent == service_indent {
            // New service entry: "<name>:"
            if let Some(name) = content.strip_suffix(':') {
                let name = name.trim();
                if !name.is_empty() {
                    parsed.service_keys.push(name.to_owned());
                    current_service = Some(name.to_owned());
                    current_service_indent = Some(indent);
                }
            }
            continue;
        }
        if let (Some(_service), Some(svc_indent)) = (&current_service, current_service_indent) {
            if indent > svc_indent {
                if content.starts_with("image:") {
                    parsed.has_image_service = true;
                } else if content.starts_with("build:") {
                    parsed.has_build_service = true;
                }
            } else if indent <= svc_indent {
                current_service = None;
                current_service_indent = None;
            }
        }
    }
    parsed.service_keys.sort();
    parsed.service_keys.dedup();
    if parsed.service_keys.is_empty() {
        notes.push(CapsuleBetaSourceNote::RequiredFieldMissing);
    }
    parsed
}

fn parse_nix(root: &Path) -> Vec<CapsuleBetaSourceParse> {
    let mut parsed = Vec::new();
    for (file, class, variant) in [
        ("flake.nix", CapsuleBetaSourceClass::NixFlake, "flake"),
        ("shell.nix", CapsuleBetaSourceClass::NixShell, "shell"),
        ("default.nix", CapsuleBetaSourceClass::NixDefault, "default"),
    ] {
        let path = root.join(file);
        if path.is_file() {
            let bytes = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let digest = digest_token(&["nix.body", file, bytes_digest(&bytes).as_str()]);
            let confidence = class.default_confidence();
            parsed.push(CapsuleBetaSourceParse {
                source_class: class,
                source_class_token: class.as_str().to_owned(),
                source_ref: file.to_owned(),
                content_digest: digest,
                confidence,
                confidence_token: confidence.as_str().to_owned(),
                notes: vec![CapsuleBetaSourceNote::UnsupportedBodyParse],
                parsed_fields: CapsuleBetaParsedFields::Nix(NixParsedFields {
                    variant_token: variant.to_owned(),
                }),
            });
        }
    }
    parsed
}

fn parse_node(root: &Path) -> Option<CapsuleBetaSourceParse> {
    let manifest_path = root.join("package.json");
    let lockfiles: Vec<String> = ["package-lock.json", "pnpm-lock.yaml", "yarn.lock", "npm-shrinkwrap.json"]
        .into_iter()
        .filter(|name| root.join(name).is_file())
        .map(|name| name.to_owned())
        .collect();
    if !manifest_path.is_file() && lockfiles.is_empty() {
        return None;
    }
    let mut digest_inputs: Vec<PathBuf> = Vec::new();
    let mut has_package_json = false;
    if manifest_path.is_file() {
        digest_inputs.push(manifest_path.clone());
        has_package_json = true;
    }
    for lock in &lockfiles {
        digest_inputs.push(root.join(lock));
    }
    let digest = aggregate_path_digest("node.body", &digest_inputs);
    let parsed = NodeParsedFields {
        has_package_json,
        lockfile_refs: lockfiles,
    };
    let confidence = if has_package_json {
        CapsuleBetaSourceConfidence::Imported
    } else {
        CapsuleBetaSourceConfidence::Heuristic
    };
    Some(CapsuleBetaSourceParse {
        source_class: CapsuleBetaSourceClass::NodeManifest,
        source_class_token: CapsuleBetaSourceClass::NodeManifest.as_str().to_owned(),
        source_ref: "package.json".to_owned(),
        content_digest: digest,
        confidence,
        confidence_token: confidence.as_str().to_owned(),
        notes: Vec::new(),
        parsed_fields: CapsuleBetaParsedFields::NodeManifest(parsed),
    })
}

fn parse_python(root: &Path) -> Option<CapsuleBetaSourceParse> {
    let pyproject_path = root.join("pyproject.toml");
    let python_version_path = root.join(".python-version");
    let lockfiles: Vec<String> = ["uv.lock", "poetry.lock", "Pipfile.lock"]
        .into_iter()
        .filter(|name| root.join(name).is_file())
        .map(|name| name.to_owned())
        .collect();
    let has_pyproject = pyproject_path.is_file();
    let has_python_version = python_version_path.is_file();
    if !has_pyproject && !has_python_version && lockfiles.is_empty() {
        return None;
    }
    let mut digest_inputs: Vec<PathBuf> = Vec::new();
    if has_pyproject {
        digest_inputs.push(pyproject_path.clone());
    }
    if has_python_version {
        digest_inputs.push(python_version_path.clone());
    }
    for lock in &lockfiles {
        digest_inputs.push(root.join(lock));
    }
    let digest = aggregate_path_digest("python.body", &digest_inputs);
    let parsed = PythonParsedFields {
        has_pyproject,
        has_python_version,
        lockfile_refs: lockfiles,
    };
    let confidence = if has_pyproject || has_python_version {
        CapsuleBetaSourceConfidence::Imported
    } else {
        CapsuleBetaSourceConfidence::Heuristic
    };
    Some(CapsuleBetaSourceParse {
        source_class: CapsuleBetaSourceClass::PythonManifest,
        source_class_token: CapsuleBetaSourceClass::PythonManifest.as_str().to_owned(),
        source_ref: if has_pyproject {
            "pyproject.toml".to_owned()
        } else if has_python_version {
            ".python-version".to_owned()
        } else {
            "python.manifest".to_owned()
        },
        content_digest: digest,
        confidence,
        confidence_token: confidence.as_str().to_owned(),
        notes: Vec::new(),
        parsed_fields: CapsuleBetaParsedFields::PythonManifest(parsed),
    })
}

fn aggregate_path_digest(label: &str, paths: &[PathBuf]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    tokens.push(label.to_owned());
    let mut sorted_paths: Vec<&PathBuf> = paths.iter().collect();
    sorted_paths.sort();
    for path in sorted_paths {
        let bytes = fs::read(path).unwrap_or_default();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        tokens.push(name);
        tokens.push(bytes_digest(&bytes));
    }
    let token_views: Vec<&str> = tokens.iter().map(String::as_str).collect();
    digest_token(&token_views)
}

fn bytes_digest(bytes: &[u8]) -> String {
    // Reuse the alpha digest function over a stable string view of the bytes.
    let view = String::from_utf8_lossy(bytes);
    digest_token(&["bytes", view.as_ref()])
}

fn build_precedence_ladder(
    sources: &[CapsuleBetaSourceParse],
    primary_source: Option<CapsuleBetaSourceClass>,
) -> Vec<CapsuleBetaPrecedenceRow> {
    let mut rows: Vec<CapsuleBetaPrecedenceRow> = CapsuleBetaSourceClass::ALL
        .into_iter()
        .map(|class| {
            let present = sources.iter().any(|src| src.source_class == class);
            CapsuleBetaPrecedenceRow {
                source_class: class,
                source_class_token: class.as_str().to_owned(),
                rank: class.precedence_rank(),
                source_present: present,
                winner: primary_source == Some(class),
            }
        })
        .collect();
    rows.sort_by_key(|row| row.rank);
    rows
}

fn compute_conflict_notes(
    sources: &[CapsuleBetaSourceParse],
    primary: Option<CapsuleBetaSourceClass>,
) -> Vec<CapsuleBetaSourceNote> {
    let mut notes = Vec::new();
    let primary = match primary {
        Some(p) => p,
        None => return notes,
    };
    let losers: Vec<&CapsuleBetaSourceParse> = sources
        .iter()
        .filter(|s| s.source_class != primary)
        .collect();
    if !losers.is_empty() {
        notes.push(CapsuleBetaSourceNote::OverriddenByHigherPrecedence);
    }
    notes
}

fn compute_source_set_digest(sources: &[CapsuleBetaSourceParse]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    tokens.push("source-set.beta".to_owned());
    let mut sorted: Vec<&CapsuleBetaSourceParse> = sources.iter().collect();
    sorted.sort_by_key(|s| s.source_class.precedence_rank());
    for src in sorted {
        tokens.push(src.source_class.as_str().to_owned());
        tokens.push(src.source_ref.clone());
        tokens.push(src.content_digest.clone());
        tokens.push(src.confidence.as_str().to_owned());
    }
    let views: Vec<&str> = tokens.iter().map(String::as_str).collect();
    digest_token(&views)
}

fn capsule_id_for_primary(
    primary: Option<CapsuleBetaSourceClass>,
    capsule_hint: EnvironmentCapsuleHint,
    archetype_hint: ProjectArchetypeHint,
) -> String {
    match primary {
        Some(CapsuleBetaSourceClass::Devcontainer) => {
            "capsule.beta.devcontainer.parsed".to_owned()
        }
        Some(CapsuleBetaSourceClass::DockerCompose) => "capsule.beta.compose.parsed".to_owned(),
        Some(CapsuleBetaSourceClass::NixFlake) => "capsule.beta.nix_flake.metadata".to_owned(),
        Some(CapsuleBetaSourceClass::NixShell) => "capsule.beta.nix_shell.metadata".to_owned(),
        Some(CapsuleBetaSourceClass::NixDefault) => "capsule.beta.nix_default.metadata".to_owned(),
        Some(CapsuleBetaSourceClass::NodeManifest) => match archetype_hint {
            ProjectArchetypeHint::WebApplication
            | ProjectArchetypeHint::WebFrontendLibrary
            | ProjectArchetypeHint::DocumentationSite => {
                "capsule.beta.node.web.metadata".to_owned()
            }
            _ => "capsule.beta.node.metadata".to_owned(),
        },
        Some(CapsuleBetaSourceClass::PythonManifest) => "capsule.beta.python.metadata".to_owned(),
        None => match capsule_hint {
            EnvironmentCapsuleHint::NoSignal => "capsule.beta.unknown.uncertain".to_owned(),
            EnvironmentCapsuleHint::Polyglot => "capsule.beta.polyglot.metadata".to_owned(),
            EnvironmentCapsuleHint::Node => "capsule.beta.node.metadata".to_owned(),
            EnvironmentCapsuleHint::Python => "capsule.beta.python.metadata".to_owned(),
        },
    }
}

fn strip_jsonc_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut iter = input.chars().peekable();
    let mut in_string = false;
    let mut escape = false;
    while let Some(ch) = iter.next() {
        if in_string {
            out.push(ch);
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
            out.push(ch);
            continue;
        }
        if ch == '/' {
            match iter.peek() {
                Some('/') => {
                    iter.next();
                    while let Some(&peek) = iter.peek() {
                        if peek == '\n' {
                            break;
                        }
                        iter.next();
                    }
                    continue;
                }
                Some('*') => {
                    iter.next();
                    let mut prev = '\0';
                    for nc in iter.by_ref() {
                        if prev == '*' && nc == '/' {
                            break;
                        }
                        prev = nc;
                    }
                    continue;
                }
                _ => {}
            }
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_workspace(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "aureline-capsule-beta-{label}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("create temp workspace");
        root
    }

    #[test]
    fn devcontainer_only_workspace_resolves_to_imported_devcontainer_source() {
        let root = temp_workspace("devcontainer");
        fs::write(
            root.join("devcontainer.json"),
            r#"{
  // Pinned image
  "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {}
  },
  "forwardPorts": [3000, 5173],
  "postCreateCommand": "echo hello"
}
"#,
        )
        .expect("write devcontainer.json");

        let resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::WebApplication);

        assert_eq!(resolution.primary_source, Some(CapsuleBetaSourceClass::Devcontainer));
        assert_eq!(resolution.primary_source_token.as_deref(), Some("devcontainer"));
        assert_eq!(resolution.drift_state, CapsuleDriftState::InSync);
        assert_eq!(resolution.prebuild_reuse_state, PrebuildReuseState::Candidate);
        let devcontainer = resolution
            .sources
            .iter()
            .find(|s| s.source_class == CapsuleBetaSourceClass::Devcontainer)
            .expect("devcontainer source");
        assert_eq!(devcontainer.confidence, CapsuleBetaSourceConfidence::Imported);
        assert!(devcontainer.notes.is_empty());
        match &devcontainer.parsed_fields {
            CapsuleBetaParsedFields::Devcontainer(parsed) => {
                assert_eq!(parsed.image_ref.as_deref(), Some("mcr.microsoft.com/devcontainers/base:ubuntu"));
                assert!(parsed.feature_keys.contains(&"ghcr.io/devcontainers/features/node:1".to_owned()));
                assert_eq!(parsed.forward_port_count, 2);
                assert!(parsed.lifecycle_hook_keys.contains(&"postCreateCommand".to_owned()));
            }
            _ => panic!("unexpected parsed fields"),
        }
        assert!(resolution
            .precedence
            .iter()
            .any(|row| row.source_class == CapsuleBetaSourceClass::Devcontainer && row.winner));
        assert_eq!(
            resolution.environment_capsule_ref.capsule_id,
            "capsule.beta.devcontainer.parsed"
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn devcontainer_overrides_compose_when_both_present() {
        let root = temp_workspace("devcontainer-compose");
        fs::write(
            root.join("devcontainer.json"),
            r#"{
  "dockerComposeFile": "docker-compose.yml",
  "service": "app"
}
"#,
        )
        .expect("write devcontainer.json");
        fs::write(
            root.join("docker-compose.yml"),
            "services:\n  app:\n    image: nginx:1.25\n  worker:\n    build: .\n",
        )
        .expect("write docker-compose.yml");

        let resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::BackendService);

        assert_eq!(resolution.primary_source, Some(CapsuleBetaSourceClass::Devcontainer));
        assert!(resolution
            .conflict_notes
            .contains(&CapsuleBetaSourceNote::OverriddenByHigherPrecedence));
        let compose = resolution
            .sources
            .iter()
            .find(|s| s.source_class == CapsuleBetaSourceClass::DockerCompose)
            .expect("compose source");
        assert!(compose
            .notes
            .contains(&CapsuleBetaSourceNote::OverriddenByHigherPrecedence));
        match &compose.parsed_fields {
            CapsuleBetaParsedFields::DockerCompose(parsed) => {
                assert_eq!(parsed.service_keys, vec!["app".to_owned(), "worker".to_owned()]);
                assert!(parsed.has_image_service);
                assert!(parsed.has_build_service);
            }
            _ => panic!("unexpected parsed fields"),
        }

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn nix_only_workspace_marks_source_unsupported_but_tracks_drift() {
        let root = temp_workspace("nix");
        fs::write(
            root.join("flake.nix"),
            "{ description = \"test\"; outputs = inputs: {}; }\n",
        )
        .expect("write flake.nix");

        let resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::LibraryOrSdk);
        assert_eq!(resolution.primary_source, Some(CapsuleBetaSourceClass::NixFlake));
        let nix = resolution
            .sources
            .iter()
            .find(|s| s.source_class == CapsuleBetaSourceClass::NixFlake)
            .expect("nix source");
        assert_eq!(nix.confidence, CapsuleBetaSourceConfidence::Unsupported);
        assert!(nix.notes.contains(&CapsuleBetaSourceNote::UnsupportedBodyParse));
        assert_eq!(resolution.drift_state, CapsuleDriftState::InSync);
        assert_eq!(
            resolution.environment_capsule_ref.capsule_id,
            "capsule.beta.nix_flake.metadata"
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn empty_workspace_marks_lineage_unknown() {
        let root = temp_workspace("empty");
        let resolution = EnvironmentCapsuleBetaResolver::default_read_only().resolve_workspace(
            &root,
            ProjectArchetypeHint::ArchetypeClassUnknownRequiresReview,
        );
        assert!(resolution.primary_source.is_none());
        assert_eq!(resolution.drift_state, CapsuleDriftState::UnknownLineage);
        assert_eq!(
            resolution.environment_capsule_ref.capsule_id,
            "capsule.beta.unknown.uncertain"
        );
        assert_eq!(resolution.prebuild_reuse_state, PrebuildReuseState::NotApplicable);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn drift_evaluator_marks_stale_inputs_after_devcontainer_edit() {
        let root = temp_workspace("drift-stale");
        fs::write(
            root.join("devcontainer.json"),
            r#"{ "image": "ghcr.io/example/runtime:1" }"#,
        )
        .expect("write");
        let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
        let baseline_resolution = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let baseline = CapsuleBetaSourceBaseline::from_resolution(&baseline_resolution);

        fs::write(
            root.join("devcontainer.json"),
            r#"{ "image": "ghcr.io/example/runtime:2" }"#,
        )
        .expect("rewrite");
        let fresh = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);

        let evaluation = evaluate_capsule_drift(&baseline, &fresh);
        assert!(evaluation.is_drifted());
        assert_eq!(evaluation.outcome, CapsuleBetaDriftOutcome::StaleInputs);
        assert!(evaluation
            .drift_rows
            .iter()
            .any(|row| row.source_class == CapsuleBetaSourceClass::Devcontainer));
        assert!(evaluation.added_sources.is_empty());
        assert!(evaluation.removed_sources.is_empty());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn drift_evaluator_marks_manually_diverged_after_source_added() {
        let root = temp_workspace("drift-add");
        fs::write(
            root.join("devcontainer.json"),
            r#"{ "image": "ghcr.io/example/runtime:1" }"#,
        )
        .expect("write");
        let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
        let baseline_resolution = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let baseline = CapsuleBetaSourceBaseline::from_resolution(&baseline_resolution);

        fs::write(
            root.join("docker-compose.yml"),
            "services:\n  app:\n    image: nginx:1.25\n",
        )
        .expect("write");
        let fresh = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let evaluation = evaluate_capsule_drift(&baseline, &fresh);
        assert_eq!(evaluation.outcome, CapsuleBetaDriftOutcome::ManuallyDiverged);
        assert!(evaluation.added_sources.contains(&CapsuleBetaSourceClass::DockerCompose));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn coverage_manifest_pins_full_source_vocabulary() {
        let manifest = EnvironmentCapsuleBetaCoverageManifest::canonical(
            "environment-capsule-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert!(manifest.covers_every_source_class());
        assert_eq!(manifest.source_classes.len(), CapsuleBetaSourceClass::ALL.len());
        for (idx, row) in manifest.source_classes.iter().enumerate() {
            assert_eq!(row.rank as usize, idx);
        }
    }

    #[test]
    fn support_export_round_trips_through_serde() {
        let root = temp_workspace("support");
        fs::write(
            root.join("devcontainer.json"),
            r#"{ "image": "ghcr.io/example/runtime:1" }"#,
        )
        .expect("write");
        let resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let packet = EnvironmentCapsuleBetaSupportExport::new(
            "environment-capsule-beta:packet",
            "2026-05-15T00:00:00Z",
            resolution.clone(),
            Vec::new(),
        );
        let json = serde_json::to_string(&packet).expect("serialize");
        let round: EnvironmentCapsuleBetaSupportExport =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, packet);
        assert_eq!(
            round.record_kind,
            ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn jsonc_comment_stripping_preserves_string_bodies() {
        let stripped = strip_jsonc_comments(
            r#"{ "image": "// not a comment", /* block */ "service": "app" } // trailing"#,
        );
        let value: serde_json::Value = serde_json::from_str(&stripped).expect("parse");
        assert_eq!(value["image"], "// not a comment");
        assert_eq!(value["service"], "app");
    }

    #[test]
    fn capsule_hash_advances_when_source_content_changes() {
        let root = temp_workspace("hash-advance");
        fs::write(
            root.join("devcontainer.json"),
            r#"{ "image": "ghcr.io/example/runtime:1" }"#,
        )
        .expect("write");
        let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
        let first = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);

        fs::write(
            root.join("devcontainer.json"),
            r#"{ "image": "ghcr.io/example/runtime:2" }"#,
        )
        .expect("rewrite");
        let second = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);

        assert_ne!(
            first.environment_capsule_ref.capsule_hash,
            second.environment_capsule_ref.capsule_hash
        );
        assert_ne!(first.source_set_digest, second.source_set_digest);
        fs::remove_dir_all(root).ok();
    }
}
