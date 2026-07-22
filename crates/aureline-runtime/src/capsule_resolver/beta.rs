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
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    digest_token, EnvironmentCapsuleHint, EnvironmentCapsuleResolution, EnvironmentCapsuleResolver,
    EnvironmentCapsuleResolverConfig, ProjectArchetypeHint,
    ENVIRONMENT_CAPSULE_RESOLUTION_SCHEMA_VERSION,
};
use crate::digest::{sha256_framed_token, sha256_token};
use crate::discovery::bounded_file::{
    read_bounded_workspace_bytes, workspace_regular_file_exists, BoundedWorkspaceReadError,
};
use crate::execution_context::{CapsuleDriftState, EnvironmentCapsuleRef, PrebuildReuseState};

const MAX_CAPSULE_SOURCE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_CAPSULE_SOURCE_SET_BYTES: u64 = 32 * 1024 * 1024;
const MAX_SUPPORT_SOURCES: usize = CapsuleBetaSourceClass::ALL.len();
const MAX_SUPPORT_DRIFT_EVALUATIONS: usize = 32;
const MAX_SUPPORT_SOURCE_NOTES: usize = 9;
const MAX_SUPPORT_HASH_INPUT_BYTES: usize = 4 * 1024;

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
pub const ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION: u32 = 2;

/// Schema version for the default-redacted beta support-export projection.
///
/// Resolution, drift, and coverage records remain on schema version 2. The
/// support record advances independently because v3 deliberately removes the
/// verbatim resolution body from the export boundary.
pub const ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 3;

/// Beta resolver implementation token recorded on every resolution.
pub const ENVIRONMENT_CAPSULE_BETA_RESOLVER_VERSION: &str = "environment_capsule_resolver.beta.v2";

/// Governed default profile applied by the beta support projection.
pub const ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF: &str =
    "support.redaction.local_first_default";

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

/// Whether the resolver obtained the complete byte body for a source row.
///
/// Missing files do not produce rows. A present-but-unreadable or oversized
/// file does produce a row so degraded evidence cannot be mistaken for source
/// absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaSourceReadState {
    /// The complete source body was read under the per-file and aggregate caps.
    Complete,
    /// The source was present but failed containment, identity, type, or I/O checks.
    Unavailable,
    /// The source exceeded the per-file or aggregate byte budget.
    ResourceLimitExceeded,
}

impl CapsuleBetaSourceReadState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Unavailable => "unavailable",
            Self::ResourceLimitExceeded => "resource_limit_exceeded",
        }
    }

    const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
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
    /// The source was present but could not be read through the bounded,
    /// identity-checked workspace reader.
    SourceReadUnavailable,
    /// The source exceeded the per-file or aggregate resolver byte budget.
    SourceResourceLimitExceeded,
    /// The complete body was read and digested, but it was not valid UTF-8 and
    /// therefore could not be parsed as JSON / YAML text.
    BodyInvalidUtf8,
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
            Self::SourceReadUnavailable => "source_read_unavailable",
            Self::SourceResourceLimitExceeded => "source_resource_limit_exceeded",
            Self::BodyInvalidUtf8 => "body_invalid_utf8",
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
    /// SHA-256 digest of the exact body bytes (or a framed multi-file body)
    /// when [`Self::read_state`] is complete.
    pub content_digest: Option<String>,
    /// Bounded-read outcome for this source.
    pub read_state: CapsuleBetaSourceReadState,
    /// Stable token for [`Self::read_state`].
    pub read_state_token: String,
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
    /// Stored bounded-read state, when the source was present in the baseline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stored_read_state: Option<CapsuleBetaSourceReadState>,
    /// Fresh bounded-read state, when the source is present today.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fresh_read_state: Option<CapsuleBetaSourceReadState>,
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
                stored_content_digest: src.content_digest.clone(),
                fresh_content_digest: src.content_digest.clone(),
                stored_read_state: Some(src.read_state),
                fresh_read_state: Some(src.read_state),
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
            if !self
                .source_classes
                .iter()
                .any(|row| row.source_class == class)
            {
                return false;
            }
        }
        true
    }
}

/// How a digest in the support projection was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaSupportDigestProjectionClass {
    /// The producer supplied a syntactically valid SHA-256 token.
    ValidatedSha256,
    /// An invalid claim was replaced with a profile-scoped SHA-256 digest of
    /// the claim so comparison remains possible without repeating it.
    RedactedInvalidInputRehash,
}

/// Digest admitted to the default-redacted support boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportDigest {
    pub value: String,
    pub projection_class: CapsuleBetaSupportDigestProjectionClass,
}

/// Redaction-safe summary of a parsed capsule source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CapsuleBetaSupportParsedSummary {
    Devcontainer {
        image_declared: bool,
        dockerfile_declared: bool,
        compose_file_declared: bool,
        compose_service_declared: bool,
        feature_count: u64,
        lifecycle_hook_count: u64,
    },
    DockerCompose {
        service_count: u64,
        has_image_service: bool,
        has_build_service: bool,
    },
    NixMetadataOnly,
    NodeManifest {
        has_package_json: bool,
        lockfile_count: u64,
    },
    PythonManifest {
        has_pyproject: bool,
        has_python_version: bool,
        lockfile_count: u64,
    },
    /// The raw record paired a source class with an impossible parsed shape.
    ShapeMismatch,
}

/// One bounded, redaction-safe source row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportSourceProjection {
    pub source_class: CapsuleBetaSourceClass,
    pub precedence_rank: u8,
    pub primary: bool,
    pub source_ref_digest: String,
    pub content_digest: Option<CapsuleBetaSupportDigest>,
    pub read_state: CapsuleBetaSourceReadState,
    pub confidence: CapsuleBetaSourceConfidence,
    pub notes: Vec<CapsuleBetaSourceNote>,
    pub parsed_summary: CapsuleBetaSupportParsedSummary,
}

/// Metadata-only summary of the alpha Node/Python detector bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportDetectorSummary {
    pub alpha_schema_version: u32,
    pub node_detection_present: bool,
    pub node_detector_failure: bool,
    pub node_has_fallback: bool,
    pub node_unresolved_ambiguity_count: u64,
    pub python_detection_present: bool,
    pub python_detector_failure: bool,
    pub python_has_fallback: bool,
    pub python_unresolved_ambiguity_count: u64,
}

/// Canonical source-class row in the support coverage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportCoverageRow {
    pub source_class: CapsuleBetaSourceClass,
    pub precedence_rank: u8,
    pub default_confidence: CapsuleBetaSourceConfidence,
}

/// Redaction-safe projection of the canonical coverage vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportCoverageProjection {
    pub record_kind: String,
    pub core_schema_version: u32,
    pub source_class_count: u64,
    pub source_classes: Vec<CapsuleBetaSupportCoverageRow>,
}

/// Redaction-safe projection of one beta resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportResolutionProjection {
    pub core_record_kind: String,
    pub core_schema_version: u32,
    pub resolver_version_digest: String,
    pub resolver_is_current: bool,
    pub archetype_hint: ProjectArchetypeHint,
    pub capsule_hint: EnvironmentCapsuleHint,
    pub primary_source: Option<CapsuleBetaSourceClass>,
    pub source_count_observed: u64,
    pub source_count_exported: u64,
    pub source_count_omitted: u64,
    pub conflict_notes: Vec<CapsuleBetaSourceNote>,
    pub projected_source_set_digest: String,
    pub capsule_id_digest: String,
    pub projected_capsule_binding_digest: String,
    pub drift_state: CapsuleDriftState,
    pub prebuild_reuse_state: PrebuildReuseState,
    pub detector_summary: CapsuleBetaSupportDetectorSummary,
    pub source_projections: Vec<CapsuleBetaSupportSourceProjection>,
}

/// Redaction-safe per-source drift row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportDriftRowProjection {
    pub source_class: CapsuleBetaSourceClass,
    pub stored_content_digest: Option<CapsuleBetaSupportDigest>,
    pub fresh_content_digest: Option<CapsuleBetaSupportDigest>,
    pub stored_read_state: Option<CapsuleBetaSourceReadState>,
    pub fresh_read_state: Option<CapsuleBetaSourceReadState>,
}

/// Bounded, redaction-safe drift evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportDriftEvaluationProjection {
    pub core_record_kind: String,
    pub core_schema_version: u32,
    pub stored_source_set_digest: CapsuleBetaSupportDigest,
    pub fresh_source_set_digest: CapsuleBetaSupportDigest,
    pub outcome: CapsuleBetaDriftOutcome,
    pub drift_row_count_observed: u64,
    pub drift_row_count_exported: u64,
    pub drift_row_count_omitted: u64,
    pub drift_rows: Vec<CapsuleBetaSupportDriftRowProjection>,
    pub added_source_count_observed: u64,
    pub added_source_count_exported: u64,
    pub added_source_count_omitted: u64,
    pub added_sources: Vec<CapsuleBetaSourceClass>,
    pub removed_source_count_observed: u64,
    pub removed_source_count_exported: u64,
    pub removed_source_count_omitted: u64,
    pub removed_sources: Vec<CapsuleBetaSourceClass>,
}

/// Why a raw field is present or absent from the support projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleBetaSupportFieldDispositionClass {
    RecordedAndPresent,
    NotRecordedByDesign,
    OmittedByRedaction,
    OmittedByExpiry,
    OmittedByPolicy,
    OmittedByLegalHold,
    UnavailableSource,
    OutsidePlatformScope,
}

/// One field-level privacy disposition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportFieldDisposition {
    pub field_path: String,
    pub disposition: CapsuleBetaSupportFieldDispositionClass,
    pub evidence_window_ref: String,
    pub redaction_profile_ref: Option<String>,
    pub note: String,
}

/// Default interpretation for fields absent from the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleBetaSupportAbsenceSummary {
    pub missing_field_default: String,
    pub requires_disposition_for_export: bool,
    pub disposition_count: u64,
}

/// Default-redacted beta support packet.
///
/// This intentionally does not embed [`EnvironmentCapsuleBetaResolution`] or
/// [`EnvironmentCapsuleBetaDriftEvaluation`]. Their path, detector, parsed
/// string, and caller-token fields are private runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub manifest_id_digest: String,
    pub generated_at: String,
    pub purpose: String,
    pub data_class: String,
    pub redaction_class: String,
    pub redaction_profile_ref: String,
    pub export_posture: String,
    pub raw_private_material_exported: bool,
    pub coverage_projection: CapsuleBetaSupportCoverageProjection,
    pub resolution_projection: CapsuleBetaSupportResolutionProjection,
    pub drift_evaluation_count_observed: u64,
    pub drift_evaluation_count_exported: u64,
    pub drift_evaluation_count_omitted: u64,
    pub drift_evaluations: Vec<CapsuleBetaSupportDriftEvaluationProjection>,
    pub absence_summary: CapsuleBetaSupportAbsenceSummary,
    pub field_dispositions: Vec<CapsuleBetaSupportFieldDisposition>,
}

impl EnvironmentCapsuleBetaSupportExport {
    /// Builds a bounded, default-redacted support packet from private runtime
    /// evidence. The constructor re-derives all tokens and counts that cross
    /// the support boundary.
    pub fn new(
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
        resolution: EnvironmentCapsuleBetaResolution,
        drift_evaluations: Vec<EnvironmentCapsuleBetaDriftEvaluation>,
    ) -> Self {
        let manifest_id = manifest_id.into();
        let (generated_at, timestamp_valid) = normalize_support_timestamp(generated_at.into());
        let resolution_projection = project_support_resolution(&resolution);
        let drift_evaluation_count_observed = count_u64(drift_evaluations.len());
        let drift_evaluations: Vec<_> = drift_evaluations
            .iter()
            .take(MAX_SUPPORT_DRIFT_EVALUATIONS)
            .map(project_support_drift_evaluation)
            .collect();
        let drift_evaluation_count_exported = count_u64(drift_evaluations.len());
        let drift_evaluation_count_omitted =
            drift_evaluation_count_observed.saturating_sub(drift_evaluation_count_exported);
        let bounded_tail_omitted = resolution_projection.source_count_omitted > 0
            || drift_evaluation_count_omitted > 0
            || drift_evaluations.iter().any(|drift| {
                drift.drift_row_count_omitted > 0
                    || drift.added_source_count_omitted > 0
                    || drift.removed_source_count_omitted > 0
            });
        let field_dispositions = support_field_dispositions(timestamp_valid, bounded_tail_omitted);
        let absence_summary = CapsuleBetaSupportAbsenceSummary {
            missing_field_default: "unknown_until_field_disposition_present".to_owned(),
            requires_disposition_for_export: true,
            disposition_count: count_u64(field_dispositions.len()),
        };

        Self {
            record_kind: ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_SCHEMA_VERSION,
            manifest_id_digest: profile_scoped_digest("manifest_id", &manifest_id),
            generated_at,
            purpose: "environment_capsule_resolution_support".to_owned(),
            data_class: "environment_adjacent".to_owned(),
            redaction_class: "metadata_safe_default".to_owned(),
            redaction_profile_ref: ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF
                .to_owned(),
            export_posture: "included_metadata_only".to_owned(),
            raw_private_material_exported: false,
            coverage_projection: canonical_support_coverage(),
            resolution_projection,
            drift_evaluation_count_observed,
            drift_evaluation_count_exported,
            drift_evaluation_count_omitted,
            drift_evaluations,
            absence_summary,
            field_dispositions,
        }
    }
}

fn canonical_support_coverage() -> CapsuleBetaSupportCoverageProjection {
    CapsuleBetaSupportCoverageProjection {
        record_kind: "environment_capsule_beta_support_coverage_projection".to_owned(),
        core_schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
        source_class_count: count_u64(CapsuleBetaSourceClass::ALL.len()),
        source_classes: CapsuleBetaSourceClass::ALL
            .into_iter()
            .map(|source_class| CapsuleBetaSupportCoverageRow {
                source_class,
                precedence_rank: source_class.precedence_rank(),
                default_confidence: source_class.default_confidence(),
            })
            .collect(),
    }
}

fn project_support_resolution(
    resolution: &EnvironmentCapsuleBetaResolution,
) -> CapsuleBetaSupportResolutionProjection {
    let source_count_observed = count_u64(resolution.sources.len());
    let mut unique_sources = BTreeMap::new();
    for source in &resolution.sources {
        unique_sources.entry(source.source_class).or_insert(source);
        if unique_sources.len() == MAX_SUPPORT_SOURCES {
            break;
        }
    }
    let source_projections: Vec<_> = unique_sources
        .into_values()
        .take(MAX_SUPPORT_SOURCES)
        .map(|source| project_support_source(source, resolution.primary_source))
        .collect();
    let source_count_exported = count_u64(source_projections.len());
    let source_count_omitted = source_count_observed.saturating_sub(source_count_exported);
    let projected_source_set_digest = digest_safe_projection(
        b"environment_capsule_beta_support_source_set_v1",
        &source_projections,
    );
    let capsule_id_digest =
        profile_scoped_digest("capsule_id", &resolution.environment_capsule_ref.capsule_id);
    let projected_capsule_binding_digest = sha256_framed_token(&[
        ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF.as_bytes(),
        b"environment_capsule_beta_support_binding_v1",
        projected_source_set_digest.as_bytes(),
        capsule_id_digest.as_bytes(),
        resolution.drift_state.as_str().as_bytes(),
        resolution.prebuild_reuse_state.as_str().as_bytes(),
    ]);
    let mut conflict_notes: Vec<_> = resolution
        .conflict_notes
        .iter()
        .copied()
        .take(MAX_SUPPORT_SOURCE_NOTES)
        .collect();
    conflict_notes.sort();
    conflict_notes.dedup();
    conflict_notes.truncate(MAX_SUPPORT_SOURCE_NOTES);

    CapsuleBetaSupportResolutionProjection {
        core_record_kind: ENVIRONMENT_CAPSULE_BETA_RESOLUTION_RECORD_KIND.to_owned(),
        core_schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
        resolver_version_digest: profile_scoped_digest(
            "resolver_version",
            &resolution.resolver_version,
        ),
        resolver_is_current: resolution.record_kind
            == ENVIRONMENT_CAPSULE_BETA_RESOLUTION_RECORD_KIND
            && resolution.schema_version == ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION
            && resolution.resolver_version == ENVIRONMENT_CAPSULE_BETA_RESOLVER_VERSION,
        archetype_hint: resolution.archetype_hint,
        capsule_hint: resolution.alpha_resolution.capsule_hint,
        primary_source: resolution.primary_source.filter(|primary| {
            source_projections
                .iter()
                .any(|source| source.source_class == *primary)
        }),
        source_count_observed,
        source_count_exported,
        source_count_omitted,
        conflict_notes,
        projected_source_set_digest,
        capsule_id_digest,
        projected_capsule_binding_digest,
        drift_state: resolution.drift_state,
        prebuild_reuse_state: resolution.prebuild_reuse_state,
        detector_summary: project_detector_summary(&resolution.alpha_resolution),
        source_projections,
    }
}

fn project_support_source(
    source: &CapsuleBetaSourceParse,
    primary_source: Option<CapsuleBetaSourceClass>,
) -> CapsuleBetaSupportSourceProjection {
    let valid_complete_digest = source.read_state == CapsuleBetaSourceReadState::Complete
        && source
            .content_digest
            .as_deref()
            .is_some_and(is_sha256_token);
    let read_state =
        if source.read_state == CapsuleBetaSourceReadState::Complete && !valid_complete_digest {
            CapsuleBetaSourceReadState::Unavailable
        } else {
            source.read_state
        };
    let content_digest = if valid_complete_digest {
        source.content_digest.as_deref().map(project_support_digest)
    } else {
        None
    };
    let mut notes: Vec<_> = source
        .notes
        .iter()
        .copied()
        .take(MAX_SUPPORT_SOURCE_NOTES)
        .collect();
    if source.read_state == CapsuleBetaSourceReadState::Complete && !valid_complete_digest {
        notes.push(CapsuleBetaSourceNote::SourceReadUnavailable);
    }
    notes.sort();
    notes.dedup();
    notes.truncate(MAX_SUPPORT_SOURCE_NOTES);

    CapsuleBetaSupportSourceProjection {
        source_class: source.source_class,
        precedence_rank: source.source_class.precedence_rank(),
        primary: primary_source == Some(source.source_class),
        source_ref_digest: profile_scoped_digest("source_ref", &source.source_ref),
        content_digest,
        read_state,
        confidence: source.confidence,
        notes,
        parsed_summary: project_parsed_summary(source),
    }
}

fn project_parsed_summary(source: &CapsuleBetaSourceParse) -> CapsuleBetaSupportParsedSummary {
    match (source.source_class, &source.parsed_fields) {
        (CapsuleBetaSourceClass::Devcontainer, CapsuleBetaParsedFields::Devcontainer(parsed)) => {
            CapsuleBetaSupportParsedSummary::Devcontainer {
                image_declared: parsed.image_ref.is_some(),
                dockerfile_declared: parsed.dockerfile_ref.is_some(),
                compose_file_declared: parsed.compose_file_ref.is_some(),
                compose_service_declared: parsed.compose_service.is_some(),
                feature_count: count_u64(parsed.feature_keys.len()),
                lifecycle_hook_count: count_u64(parsed.lifecycle_hook_keys.len()),
            }
        }
        (CapsuleBetaSourceClass::DockerCompose, CapsuleBetaParsedFields::DockerCompose(parsed)) => {
            CapsuleBetaSupportParsedSummary::DockerCompose {
                service_count: count_u64(parsed.service_keys.len()),
                has_image_service: parsed.has_image_service,
                has_build_service: parsed.has_build_service,
            }
        }
        (
            CapsuleBetaSourceClass::NixFlake
            | CapsuleBetaSourceClass::NixShell
            | CapsuleBetaSourceClass::NixDefault,
            CapsuleBetaParsedFields::Nix(_),
        ) => CapsuleBetaSupportParsedSummary::NixMetadataOnly,
        (CapsuleBetaSourceClass::NodeManifest, CapsuleBetaParsedFields::NodeManifest(parsed)) => {
            CapsuleBetaSupportParsedSummary::NodeManifest {
                has_package_json: parsed.has_package_json,
                lockfile_count: count_u64(parsed.lockfile_refs.len()),
            }
        }
        (
            CapsuleBetaSourceClass::PythonManifest,
            CapsuleBetaParsedFields::PythonManifest(parsed),
        ) => CapsuleBetaSupportParsedSummary::PythonManifest {
            has_pyproject: parsed.has_pyproject,
            has_python_version: parsed.has_python_version,
            lockfile_count: count_u64(parsed.lockfile_refs.len()),
        },
        _ => CapsuleBetaSupportParsedSummary::ShapeMismatch,
    }
}

fn project_detector_summary(
    resolution: &EnvironmentCapsuleResolution,
) -> CapsuleBetaSupportDetectorSummary {
    let node = resolution.node_toolchain_detection.as_ref();
    let python = resolution.python_environment_detection.as_ref();
    CapsuleBetaSupportDetectorSummary {
        alpha_schema_version: ENVIRONMENT_CAPSULE_RESOLUTION_SCHEMA_VERSION,
        node_detection_present: node.is_some(),
        node_detector_failure: node.is_some_and(|report| report.has_detector_failure()),
        node_has_fallback: node.is_some_and(|report| report.has_fallback()),
        node_unresolved_ambiguity_count: node.map_or(0, |report| {
            let count = count_u64(report.unresolved_ambiguities.len());
            if report.has_unresolved_ambiguity() {
                count.max(1)
            } else {
                0
            }
        }),
        python_detection_present: python.is_some(),
        python_detector_failure: python.is_some_and(|report| report.has_detector_failure()),
        python_has_fallback: python.is_some_and(|report| report.has_fallback()),
        python_unresolved_ambiguity_count: python.map_or(0, |report| {
            let count = count_u64(report.unresolved_ambiguities.len());
            if report.has_unresolved_ambiguity() {
                count.max(1)
            } else {
                0
            }
        }),
    }
}

fn project_support_drift_evaluation(
    drift: &EnvironmentCapsuleBetaDriftEvaluation,
) -> CapsuleBetaSupportDriftEvaluationProjection {
    let drift_row_count_observed = count_u64(drift.drift_rows.len());
    let mut unique_rows = BTreeMap::new();
    for row in &drift.drift_rows {
        unique_rows.entry(row.source_class).or_insert(row);
        if unique_rows.len() == MAX_SUPPORT_SOURCES {
            break;
        }
    }
    let drift_rows: Vec<_> = unique_rows
        .into_values()
        .take(MAX_SUPPORT_SOURCES)
        .map(|row| CapsuleBetaSupportDriftRowProjection {
            source_class: row.source_class,
            stored_content_digest: row
                .stored_content_digest
                .as_deref()
                .map(project_support_digest),
            fresh_content_digest: row
                .fresh_content_digest
                .as_deref()
                .map(project_support_digest),
            stored_read_state: row.stored_read_state,
            fresh_read_state: row.fresh_read_state,
        })
        .collect();
    let drift_row_count_exported = count_u64(drift_rows.len());

    let added_source_count_observed = count_u64(drift.added_sources.len());
    let mut added_sources: Vec<_> = drift
        .added_sources
        .iter()
        .copied()
        .take(MAX_SUPPORT_SOURCES)
        .collect();
    added_sources.sort();
    added_sources.dedup();
    added_sources.truncate(MAX_SUPPORT_SOURCES);
    let added_source_count_exported = count_u64(added_sources.len());

    let removed_source_count_observed = count_u64(drift.removed_sources.len());
    let mut removed_sources: Vec<_> = drift
        .removed_sources
        .iter()
        .copied()
        .take(MAX_SUPPORT_SOURCES)
        .collect();
    removed_sources.sort();
    removed_sources.dedup();
    removed_sources.truncate(MAX_SUPPORT_SOURCES);
    let removed_source_count_exported = count_u64(removed_sources.len());

    CapsuleBetaSupportDriftEvaluationProjection {
        core_record_kind: ENVIRONMENT_CAPSULE_BETA_DRIFT_RECORD_KIND.to_owned(),
        core_schema_version: ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
        stored_source_set_digest: project_support_digest(&drift.stored_source_set_digest),
        fresh_source_set_digest: project_support_digest(&drift.fresh_source_set_digest),
        outcome: drift.outcome,
        drift_row_count_observed,
        drift_row_count_exported,
        drift_row_count_omitted: drift_row_count_observed.saturating_sub(drift_row_count_exported),
        drift_rows,
        added_source_count_observed,
        added_source_count_exported,
        added_source_count_omitted: added_source_count_observed
            .saturating_sub(added_source_count_exported),
        added_sources,
        removed_source_count_observed,
        removed_source_count_exported,
        removed_source_count_omitted: removed_source_count_observed
            .saturating_sub(removed_source_count_exported),
        removed_sources,
    }
}

fn project_support_digest(value: &str) -> CapsuleBetaSupportDigest {
    if is_sha256_token(value) {
        CapsuleBetaSupportDigest {
            value: value.to_owned(),
            projection_class: CapsuleBetaSupportDigestProjectionClass::ValidatedSha256,
        }
    } else {
        CapsuleBetaSupportDigest {
            value: profile_scoped_digest("invalid_digest_claim", value),
            projection_class: CapsuleBetaSupportDigestProjectionClass::RedactedInvalidInputRehash,
        }
    }
}

fn is_sha256_token(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn profile_scoped_digest(field: &str, value: &str) -> String {
    let value_bytes = value.as_bytes();
    let retained = &value_bytes[..value_bytes.len().min(MAX_SUPPORT_HASH_INPUT_BYTES)];
    let observed_len = count_u64(value_bytes.len()).to_be_bytes();
    sha256_framed_token(&[
        ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF.as_bytes(),
        field.as_bytes(),
        &observed_len,
        retained,
    ])
}

fn digest_safe_projection<T: Serialize>(domain: &[u8], value: &T) -> String {
    let encoded = serde_json::to_vec(value)
        .unwrap_or_else(|_| b"support_projection_serialization_unavailable".to_vec());
    sha256_framed_token(&[
        ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF.as_bytes(),
        domain,
        &encoded,
    ])
}

fn count_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn normalize_support_timestamp(value: String) -> (String, bool) {
    if is_strict_utc_timestamp(&value) {
        (value, true)
    } else {
        ("1970-01-01T00:00:00Z".to_owned(), false)
    }
}

fn is_strict_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
        || bytes.iter().enumerate().any(|(index, byte)| {
            !matches!(index, 4 | 7 | 10 | 13 | 16 | 19) && !byte.is_ascii_digit()
        })
    {
        return false;
    }
    let number = |start: usize, width: usize| -> u32 {
        bytes[start..start + width]
            .iter()
            .fold(0_u32, |value, byte| value * 10 + u32::from(*byte - b'0'))
    };
    let year = number(0, 4);
    let month = number(5, 2);
    let day = number(8, 2);
    let hour = number(11, 2);
    let minute = number(14, 2);
    let second = number(17, 2);
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    (1..=max_day).contains(&day) && hour < 24 && minute < 60 && second < 60
}

fn support_field_dispositions(
    timestamp_valid: bool,
    bounded_tail_omitted: bool,
) -> Vec<CapsuleBetaSupportFieldDisposition> {
    use CapsuleBetaSupportFieldDispositionClass::{
        OmittedByRedaction, RecordedAndPresent, UnavailableSource,
    };

    let redaction_profile =
        || Some(ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF.to_owned());
    let row = |field_path: &str,
               disposition: CapsuleBetaSupportFieldDispositionClass,
               redaction_profile_ref: Option<String>,
               note: &str| CapsuleBetaSupportFieldDisposition {
        field_path: field_path.to_owned(),
        disposition,
        evidence_window_ref: "support.evidence_window.current_export".to_owned(),
        redaction_profile_ref,
        note: note.to_owned(),
    };

    vec![
        row(
            "coverage_projection",
            RecordedAndPresent,
            None,
            "The canonical seven-class coverage projection is present.",
        ),
        row(
            "resolution_projection.metadata",
            RecordedAndPresent,
            None,
            "Closed-vocabulary resolution metadata is present.",
        ),
        row(
            "drift_evaluations.projection",
            RecordedAndPresent,
            None,
            "Bounded metadata-only drift evidence is present.",
        ),
        row(
            "input.manifest_id",
            OmittedByRedaction,
            redaction_profile(),
            "The caller manifest id is replaced by a profile-scoped SHA-256 digest.",
        ),
        row(
            "input.generated_at",
            if timestamp_valid {
                RecordedAndPresent
            } else {
                UnavailableSource
            },
            None,
            if timestamp_valid {
                "The timestamp matched the strict UTC export shape."
            } else {
                "The input timestamp was invalid; the export uses the fixed epoch sentinel."
            },
        ),
        row(
            "resolution.workspace_root_ref",
            OmittedByRedaction,
            redaction_profile(),
            "Raw workspace paths never cross the support boundary.",
        ),
        row(
            "resolution.alpha_resolution.raw_payload",
            OmittedByRedaction,
            redaction_profile(),
            "Alpha detector paths, requirements, candidates, values, and summaries become counts and booleans.",
        ),
        row(
            "resolution.sources[].source_ref",
            OmittedByRedaction,
            redaction_profile(),
            "Each source ref is replaced by a profile-scoped SHA-256 digest.",
        ),
        row(
            "resolution.sources[].parsed_fields.devcontainer_strings_and_forward_ports",
            OmittedByRedaction,
            redaction_profile(),
            "Private devcontainer strings and forward-port values are omitted; presence and bounded counts remain.",
        ),
        row(
            "resolution.sources[].parsed_fields.docker_compose.service_keys",
            OmittedByRedaction,
            redaction_profile(),
            "Compose service keys are replaced by a re-derived count.",
        ),
        row(
            "resolution.sources[].parsed_fields.nix.variant_token",
            OmittedByRedaction,
            redaction_profile(),
            "The raw variant token is withheld because the closed source class already identifies the Nix variant.",
        ),
        row(
            "resolution.sources[].parsed_fields.node_manifest.lockfile_refs",
            OmittedByRedaction,
            redaction_profile(),
            "Node lockfile refs are replaced by a re-derived count.",
        ),
        row(
            "resolution.sources[].parsed_fields.python_manifest.lockfile_refs",
            OmittedByRedaction,
            redaction_profile(),
            "Python lockfile refs are replaced by a re-derived count.",
        ),
        row(
            "resolution.precedence_and_tokens",
            OmittedByRedaction,
            redaction_profile(),
            "Caller-provided ranks, primary state, and tokens are withheld and regenerated from enums.",
        ),
        row(
            "resolution.raw_digest_and_capsule_claims",
            OmittedByRedaction,
            redaction_profile(),
            "Unvalidated digest, capsule, and resolver claims are not copied; safe digests are validated or re-derived.",
        ),
        row(
            "bounded_collection_tails",
            if bounded_tail_omitted {
                OmittedByRedaction
            } else {
                RecordedAndPresent
            },
            if bounded_tail_omitted {
                redaction_profile()
            } else {
                None
            },
            if bounded_tail_omitted {
                "Collection tails are withheld by the bounded redaction profile; observed, exported, and omitted counts disclose truncation."
            } else {
                "Observed, exported, and omitted counts show that no bounded collection tail was omitted."
            },
        ),
    ]
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
        let alpha = self.inner.resolve_workspace(workspace_root, archetype_hint);
        let mut sources = parse_workspace_sources(workspace_root);
        sources.sort_by(|a, b| {
            a.source_class
                .precedence_rank()
                .cmp(&b.source_class.precedence_rank())
        });

        let primary_source = sources
            .iter()
            .map(|src| src.source_class)
            .min_by_key(|class| class.precedence_rank());
        let conflict_notes = compute_conflict_notes(&sources, primary_source);
        if let Some(primary) = primary_source {
            for src in sources.iter_mut() {
                if src.source_class != primary
                    && !src
                        .notes
                        .contains(&CapsuleBetaSourceNote::OverriddenByHigherPrecedence)
                {
                    src.notes
                        .push(CapsuleBetaSourceNote::OverriddenByHigherPrecedence);
                }
            }
        }

        let precedence = build_precedence_ladder(&sources, primary_source);
        let source_set_digest = compute_source_set_digest(&sources);
        let all_source_reads_complete =
            sources.iter().all(|source| source.read_state.is_complete());
        let drift_state = if sources.is_empty() || !all_source_reads_complete {
            CapsuleDriftState::UnknownLineage
        } else {
            CapsuleDriftState::InSync
        };
        let prebuild_reuse_state = if sources.is_empty() {
            PrebuildReuseState::NotApplicable
        } else if !all_source_reads_complete {
            PrebuildReuseState::RejectedDrift
        } else {
            PrebuildReuseState::Candidate
        };

        let primary_read_state = primary_source.and_then(|primary| {
            sources
                .iter()
                .find(|source| source.source_class == primary)
                .map(|source| source.read_state)
        });
        let capsule_id = capsule_id_for_primary(
            primary_source,
            primary_read_state,
            alpha.capsule_hint,
            archetype_hint,
        );
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
                    fresh_content_digest: src.content_digest.clone(),
                    stored_read_state: None,
                    fresh_read_state: Some(src.read_state),
                });
            }
            Some(stored_row) => {
                if stored_row.stored_content_digest != src.content_digest
                    || stored_row.stored_read_state != Some(src.read_state)
                {
                    drift_rows.push(CapsuleBetaDriftRow {
                        source_class: *class,
                        source_class_token: class.as_str().to_owned(),
                        stored_content_digest: stored_row.stored_content_digest.clone(),
                        fresh_content_digest: src.content_digest.clone(),
                        stored_read_state: stored_row.stored_read_state,
                        fresh_read_state: Some(src.read_state),
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
                stored_read_state: stored_row.stored_read_state,
                fresh_read_state: None,
            });
        }
    }
    drift_rows.sort_by_key(|row| row.source_class.precedence_rank());
    added.sort_by_key(|c| c.precedence_rank());
    removed.sort_by_key(|c| c.precedence_rank());

    let has_unknown_read_lineage = fresh
        .sources
        .iter()
        .any(|source| !source.read_state.is_complete())
        || stored.source_rows.iter().any(|row| {
            !matches!(
                row.stored_read_state,
                Some(CapsuleBetaSourceReadState::Complete)
            )
        });
    let outcome = if (stored.source_rows.is_empty() && fresh.sources.is_empty())
        || has_unknown_read_lineage
    {
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

#[derive(Debug)]
struct CapsuleSourceReadBudget {
    remaining_bytes: u64,
}

impl CapsuleSourceReadBudget {
    fn new() -> Self {
        Self {
            remaining_bytes: MAX_CAPSULE_SOURCE_SET_BYTES,
        }
    }

    fn read(
        &mut self,
        root: &Path,
        relative_ref: &str,
    ) -> Result<Option<Vec<u8>>, CapsuleBetaSourceReadState> {
        let limit = MAX_CAPSULE_SOURCE_BYTES.min(self.remaining_bytes);
        match read_bounded_workspace_bytes(root, Path::new(relative_ref), limit) {
            Ok(Some(bytes)) => {
                self.remaining_bytes = self.remaining_bytes.saturating_sub(bytes.len() as u64);
                Ok(Some(bytes))
            }
            Ok(None) => Ok(None),
            Err(BoundedWorkspaceReadError::TooLarge) => {
                Err(CapsuleBetaSourceReadState::ResourceLimitExceeded)
            }
            Err(_) => Err(CapsuleBetaSourceReadState::Unavailable),
        }
    }
}

fn read_state_note(state: CapsuleBetaSourceReadState) -> CapsuleBetaSourceNote {
    match state {
        CapsuleBetaSourceReadState::Complete => {
            unreachable!("complete reads do not carry a read-failure note")
        }
        CapsuleBetaSourceReadState::Unavailable => CapsuleBetaSourceNote::SourceReadUnavailable,
        CapsuleBetaSourceReadState::ResourceLimitExceeded => {
            CapsuleBetaSourceNote::SourceResourceLimitExceeded
        }
    }
}

fn parse_workspace_sources(root: &Path) -> Vec<CapsuleBetaSourceParse> {
    let Ok(canonical_root) = root.canonicalize() else {
        return Vec::new();
    };
    if !canonical_root.is_dir() {
        return Vec::new();
    }

    let mut budget = CapsuleSourceReadBudget::new();
    let mut sources = Vec::new();
    if let Some(parse) = parse_devcontainer(&canonical_root, &mut budget) {
        sources.push(parse);
    }
    if let Some(parse) = parse_compose(&canonical_root, &mut budget) {
        sources.push(parse);
    }
    for parse in parse_nix(&canonical_root, &mut budget) {
        sources.push(parse);
    }
    if let Some(parse) = parse_node(&canonical_root, &mut budget) {
        sources.push(parse);
    }
    if let Some(parse) = parse_python(&canonical_root, &mut budget) {
        sources.push(parse);
    }
    sources
}

fn parse_devcontainer(
    root: &Path,
    budget: &mut CapsuleSourceReadBudget,
) -> Option<CapsuleBetaSourceParse> {
    for candidate in ["devcontainer.json", ".devcontainer/devcontainer.json"] {
        let bytes = match budget.read(root, candidate) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => continue,
            Err(read_state) => {
                let confidence = CapsuleBetaSourceConfidence::Heuristic;
                return Some(CapsuleBetaSourceParse {
                    source_class: CapsuleBetaSourceClass::Devcontainer,
                    source_class_token: CapsuleBetaSourceClass::Devcontainer.as_str().to_owned(),
                    source_ref: candidate.to_owned(),
                    content_digest: None,
                    read_state,
                    read_state_token: read_state.as_str().to_owned(),
                    confidence,
                    confidence_token: confidence.as_str().to_owned(),
                    notes: vec![read_state_note(read_state)],
                    parsed_fields: CapsuleBetaParsedFields::Devcontainer(
                        DevcontainerParsedFields::default(),
                    ),
                });
            }
        };
        let digest = sha256_token(&bytes);
        let mut notes = Vec::new();
        let (parsed_fields, confidence) = match String::from_utf8(bytes) {
            Ok(body) => {
                let stripped = strip_jsonc_comments(&body);
                match serde_json::from_str::<serde_json::Value>(&stripped) {
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
                        (
                            DevcontainerParsedFields::default(),
                            CapsuleBetaSourceConfidence::Heuristic,
                        )
                    }
                }
            }
            Err(_) => {
                notes.push(CapsuleBetaSourceNote::BodyInvalidUtf8);
                (
                    DevcontainerParsedFields::default(),
                    CapsuleBetaSourceConfidence::Heuristic,
                )
            }
        };
        if let Some(compose_ref) = parsed_fields.compose_file_ref.as_deref() {
            if !workspace_regular_file_exists(root, Path::new(compose_ref)) {
                notes.push(CapsuleBetaSourceNote::DependentSourceMissing);
            }
        }
        return Some(CapsuleBetaSourceParse {
            source_class: CapsuleBetaSourceClass::Devcontainer,
            source_class_token: CapsuleBetaSourceClass::Devcontainer.as_str().to_owned(),
            source_ref: candidate.to_owned(),
            content_digest: Some(digest),
            read_state: CapsuleBetaSourceReadState::Complete,
            read_state_token: CapsuleBetaSourceReadState::Complete.as_str().to_owned(),
            confidence,
            confidence_token: confidence.as_str().to_owned(),
            notes,
            parsed_fields: CapsuleBetaParsedFields::Devcontainer(parsed_fields),
        });
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

fn parse_compose(
    root: &Path,
    budget: &mut CapsuleSourceReadBudget,
) -> Option<CapsuleBetaSourceParse> {
    for candidate in [
        "docker-compose.yml",
        "docker-compose.yaml",
        "compose.yml",
        "compose.yaml",
    ] {
        let bytes = match budget.read(root, candidate) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => continue,
            Err(read_state) => {
                let confidence = CapsuleBetaSourceConfidence::Heuristic;
                return Some(CapsuleBetaSourceParse {
                    source_class: CapsuleBetaSourceClass::DockerCompose,
                    source_class_token: CapsuleBetaSourceClass::DockerCompose.as_str().to_owned(),
                    source_ref: candidate.to_owned(),
                    content_digest: None,
                    read_state,
                    read_state_token: read_state.as_str().to_owned(),
                    confidence,
                    confidence_token: confidence.as_str().to_owned(),
                    notes: vec![read_state_note(read_state)],
                    parsed_fields: CapsuleBetaParsedFields::DockerCompose(
                        ComposeParsedFields::default(),
                    ),
                });
            }
        };
        let digest = sha256_token(&bytes);
        let mut notes = Vec::new();
        let parsed = match String::from_utf8(bytes) {
            Ok(body) => parse_compose_body(&body, &mut notes),
            Err(_) => {
                notes.push(CapsuleBetaSourceNote::BodyInvalidUtf8);
                ComposeParsedFields::default()
            }
        };
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
            content_digest: Some(digest),
            read_state: CapsuleBetaSourceReadState::Complete,
            read_state_token: CapsuleBetaSourceReadState::Complete.as_str().to_owned(),
            confidence,
            confidence_token: confidence.as_str().to_owned(),
            notes,
            parsed_fields: CapsuleBetaParsedFields::DockerCompose(parsed),
        });
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

fn parse_nix(root: &Path, budget: &mut CapsuleSourceReadBudget) -> Vec<CapsuleBetaSourceParse> {
    let mut parsed = Vec::new();
    for (file, class, variant) in [
        ("flake.nix", CapsuleBetaSourceClass::NixFlake, "flake"),
        ("shell.nix", CapsuleBetaSourceClass::NixShell, "shell"),
        ("default.nix", CapsuleBetaSourceClass::NixDefault, "default"),
    ] {
        let read = budget.read(root, file);
        let (content_digest, read_state, confidence, notes) = match read {
            Ok(Some(bytes)) => (
                Some(sha256_token(&bytes)),
                CapsuleBetaSourceReadState::Complete,
                class.default_confidence(),
                vec![CapsuleBetaSourceNote::UnsupportedBodyParse],
            ),
            Ok(None) => continue,
            Err(read_state) => (
                None,
                read_state,
                CapsuleBetaSourceConfidence::Heuristic,
                vec![read_state_note(read_state)],
            ),
        };
        parsed.push(CapsuleBetaSourceParse {
            source_class: class,
            source_class_token: class.as_str().to_owned(),
            source_ref: file.to_owned(),
            content_digest,
            read_state,
            read_state_token: read_state.as_str().to_owned(),
            confidence,
            confidence_token: confidence.as_str().to_owned(),
            notes,
            parsed_fields: CapsuleBetaParsedFields::Nix(NixParsedFields {
                variant_token: variant.to_owned(),
            }),
        });
    }
    parsed
}

#[derive(Debug)]
struct AggregateSourceRead {
    present_refs: Vec<String>,
    bodies: Vec<(String, Vec<u8>)>,
    read_state: CapsuleBetaSourceReadState,
    notes: Vec<CapsuleBetaSourceNote>,
}

fn read_source_group(
    root: &Path,
    candidate_refs: &[&str],
    budget: &mut CapsuleSourceReadBudget,
) -> AggregateSourceRead {
    let mut result = AggregateSourceRead {
        present_refs: Vec::new(),
        bodies: Vec::new(),
        read_state: CapsuleBetaSourceReadState::Complete,
        notes: Vec::new(),
    };
    for relative_ref in candidate_refs {
        match budget.read(root, relative_ref) {
            Ok(Some(bytes)) => {
                result.present_refs.push((*relative_ref).to_owned());
                result.bodies.push(((*relative_ref).to_owned(), bytes));
            }
            Ok(None) => {}
            Err(read_state) => {
                result.present_refs.push((*relative_ref).to_owned());
                if read_state == CapsuleBetaSourceReadState::ResourceLimitExceeded
                    || result.read_state == CapsuleBetaSourceReadState::Complete
                {
                    result.read_state = read_state;
                }
                let note = read_state_note(read_state);
                if !result.notes.contains(&note) {
                    result.notes.push(note);
                }
            }
        }
    }
    result
}

fn aggregate_body_digest(label: &str, bodies: &[(String, Vec<u8>)]) -> String {
    let mut sorted = bodies.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| left.0.cmp(&right.0));
    let mut parts = Vec::<&[u8]>::with_capacity(1 + sorted.len() * 2);
    parts.push(label.as_bytes());
    for (relative_ref, bytes) in sorted {
        parts.push(relative_ref.as_bytes());
        parts.push(bytes.as_slice());
    }
    sha256_framed_token(&parts)
}

fn parse_node(root: &Path, budget: &mut CapsuleSourceReadBudget) -> Option<CapsuleBetaSourceParse> {
    let read = read_source_group(
        root,
        &[
            "package.json",
            "package-lock.json",
            "pnpm-lock.yaml",
            "yarn.lock",
            "npm-shrinkwrap.json",
        ],
        budget,
    );
    if read.present_refs.is_empty() {
        return None;
    }
    let has_package_json = read.present_refs.iter().any(|item| item == "package.json");
    let lockfile_refs = read
        .present_refs
        .iter()
        .filter(|item| item.as_str() != "package.json")
        .cloned()
        .collect::<Vec<_>>();
    let mut notes = read.notes;
    if !has_package_json && !notes.contains(&CapsuleBetaSourceNote::RequiredFieldMissing) {
        notes.push(CapsuleBetaSourceNote::RequiredFieldMissing);
    }
    let confidence = if read.read_state.is_complete() && has_package_json {
        CapsuleBetaSourceConfidence::Imported
    } else {
        CapsuleBetaSourceConfidence::Heuristic
    };
    let content_digest = read
        .read_state
        .is_complete()
        .then(|| aggregate_body_digest("node.body.v2", &read.bodies));
    let source_ref = if has_package_json {
        "package.json".to_owned()
    } else {
        lockfile_refs[0].clone()
    };
    Some(CapsuleBetaSourceParse {
        source_class: CapsuleBetaSourceClass::NodeManifest,
        source_class_token: CapsuleBetaSourceClass::NodeManifest.as_str().to_owned(),
        source_ref,
        content_digest,
        read_state: read.read_state,
        read_state_token: read.read_state.as_str().to_owned(),
        confidence,
        confidence_token: confidence.as_str().to_owned(),
        notes,
        parsed_fields: CapsuleBetaParsedFields::NodeManifest(NodeParsedFields {
            has_package_json,
            lockfile_refs,
        }),
    })
}

fn parse_python(
    root: &Path,
    budget: &mut CapsuleSourceReadBudget,
) -> Option<CapsuleBetaSourceParse> {
    let read = read_source_group(
        root,
        &[
            "pyproject.toml",
            ".python-version",
            "uv.lock",
            "poetry.lock",
            "Pipfile.lock",
        ],
        budget,
    );
    if read.present_refs.is_empty() {
        return None;
    }
    let has_pyproject = read
        .present_refs
        .iter()
        .any(|item| item == "pyproject.toml");
    let has_python_version = read
        .present_refs
        .iter()
        .any(|item| item == ".python-version");
    let lockfile_refs = read
        .present_refs
        .iter()
        .filter(|item| !matches!(item.as_str(), "pyproject.toml" | ".python-version"))
        .cloned()
        .collect::<Vec<_>>();
    let mut notes = read.notes;
    if !has_pyproject
        && !has_python_version
        && !notes.contains(&CapsuleBetaSourceNote::RequiredFieldMissing)
    {
        notes.push(CapsuleBetaSourceNote::RequiredFieldMissing);
    }
    let confidence = if read.read_state.is_complete() && (has_pyproject || has_python_version) {
        CapsuleBetaSourceConfidence::Imported
    } else {
        CapsuleBetaSourceConfidence::Heuristic
    };
    let content_digest = read
        .read_state
        .is_complete()
        .then(|| aggregate_body_digest("python.body.v2", &read.bodies));
    let source_ref = if has_pyproject {
        "pyproject.toml".to_owned()
    } else if has_python_version {
        ".python-version".to_owned()
    } else {
        lockfile_refs[0].clone()
    };
    Some(CapsuleBetaSourceParse {
        source_class: CapsuleBetaSourceClass::PythonManifest,
        source_class_token: CapsuleBetaSourceClass::PythonManifest.as_str().to_owned(),
        source_ref,
        content_digest,
        read_state: read.read_state,
        read_state_token: read.read_state.as_str().to_owned(),
        confidence,
        confidence_token: confidence.as_str().to_owned(),
        notes,
        parsed_fields: CapsuleBetaParsedFields::PythonManifest(PythonParsedFields {
            has_pyproject,
            has_python_version,
            lockfile_refs,
        }),
    })
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
        tokens.push(
            src.content_digest
                .clone()
                .unwrap_or_else(|| "content_digest.unavailable".to_owned()),
        );
        tokens.push(src.read_state.as_str().to_owned());
        tokens.push(src.confidence.as_str().to_owned());
        let mut notes = src.notes.clone();
        notes.sort();
        tokens.extend(notes.into_iter().map(|note| note.as_str().to_owned()));
    }
    let views: Vec<&str> = tokens.iter().map(String::as_str).collect();
    digest_token(&views)
}

fn capsule_id_for_primary(
    primary: Option<CapsuleBetaSourceClass>,
    primary_read_state: Option<CapsuleBetaSourceReadState>,
    capsule_hint: EnvironmentCapsuleHint,
    archetype_hint: ProjectArchetypeHint,
) -> String {
    if matches!(
        primary_read_state,
        Some(
            CapsuleBetaSourceReadState::Unavailable
                | CapsuleBetaSourceReadState::ResourceLimitExceeded
        )
    ) {
        return match primary {
            Some(source) => format!("capsule.beta.{}.unavailable", source.as_str()),
            None => "capsule.beta.unknown.uncertain".to_owned(),
        };
    }
    match primary {
        Some(CapsuleBetaSourceClass::Devcontainer) => "capsule.beta.devcontainer.parsed".to_owned(),
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

        assert_eq!(
            resolution.primary_source,
            Some(CapsuleBetaSourceClass::Devcontainer)
        );
        assert_eq!(
            resolution.primary_source_token.as_deref(),
            Some("devcontainer")
        );
        assert_eq!(resolution.drift_state, CapsuleDriftState::InSync);
        assert_eq!(
            resolution.prebuild_reuse_state,
            PrebuildReuseState::Candidate
        );
        let devcontainer = resolution
            .sources
            .iter()
            .find(|s| s.source_class == CapsuleBetaSourceClass::Devcontainer)
            .expect("devcontainer source");
        assert_eq!(
            devcontainer.confidence,
            CapsuleBetaSourceConfidence::Imported
        );
        assert!(devcontainer.notes.is_empty());
        match &devcontainer.parsed_fields {
            CapsuleBetaParsedFields::Devcontainer(parsed) => {
                assert_eq!(
                    parsed.image_ref.as_deref(),
                    Some("mcr.microsoft.com/devcontainers/base:ubuntu")
                );
                assert!(parsed
                    .feature_keys
                    .contains(&"ghcr.io/devcontainers/features/node:1".to_owned()));
                assert_eq!(parsed.forward_port_count, 2);
                assert!(parsed
                    .lifecycle_hook_keys
                    .contains(&"postCreateCommand".to_owned()));
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

        assert_eq!(
            resolution.primary_source,
            Some(CapsuleBetaSourceClass::Devcontainer)
        );
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
                assert_eq!(
                    parsed.service_keys,
                    vec!["app".to_owned(), "worker".to_owned()]
                );
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
        assert_eq!(
            resolution.primary_source,
            Some(CapsuleBetaSourceClass::NixFlake)
        );
        let nix = resolution
            .sources
            .iter()
            .find(|s| s.source_class == CapsuleBetaSourceClass::NixFlake)
            .expect("nix source");
        assert_eq!(nix.confidence, CapsuleBetaSourceConfidence::Unsupported);
        assert!(nix
            .notes
            .contains(&CapsuleBetaSourceNote::UnsupportedBodyParse));
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
        assert_eq!(
            resolution.prebuild_reuse_state,
            PrebuildReuseState::NotApplicable
        );
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
        let baseline_resolution =
            resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);
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
        let baseline_resolution =
            resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let baseline = CapsuleBetaSourceBaseline::from_resolution(&baseline_resolution);

        fs::write(
            root.join("docker-compose.yml"),
            "services:\n  app:\n    image: nginx:1.25\n",
        )
        .expect("write");
        let fresh = resolver.resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let evaluation = evaluate_capsule_drift(&baseline, &fresh);
        assert_eq!(
            evaluation.outcome,
            CapsuleBetaDriftOutcome::ManuallyDiverged
        );
        assert!(evaluation
            .added_sources
            .contains(&CapsuleBetaSourceClass::DockerCompose));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn invalid_utf8_bodies_keep_exact_distinct_content_digests() {
        let first_root = temp_workspace("invalid-utf8-first");
        let second_root = temp_workspace("invalid-utf8-second");
        fs::write(first_root.join("devcontainer.json"), [0x80_u8]).expect("first body");
        fs::write(second_root.join("devcontainer.json"), [0x81_u8]).expect("second body");

        let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
        let first = resolver.resolve_workspace(&first_root, ProjectArchetypeHint::BackendService);
        let second = resolver.resolve_workspace(&second_root, ProjectArchetypeHint::BackendService);
        let first_source = first
            .sources
            .iter()
            .find(|source| source.source_class == CapsuleBetaSourceClass::Devcontainer)
            .expect("first source");
        let second_source = second
            .sources
            .iter()
            .find(|source| source.source_class == CapsuleBetaSourceClass::Devcontainer)
            .expect("second source");

        assert_eq!(
            first_source.read_state,
            CapsuleBetaSourceReadState::Complete
        );
        assert!(first_source
            .notes
            .contains(&CapsuleBetaSourceNote::BodyInvalidUtf8));
        assert_ne!(first_source.content_digest, second_source.content_digest);
        assert_eq!(first_source.content_digest, Some(sha256_token(&[0x80])));
        assert_eq!(second_source.content_digest, Some(sha256_token(&[0x81])));

        fs::remove_dir_all(first_root).ok();
        fs::remove_dir_all(second_root).ok();
    }

    #[test]
    fn oversized_sources_remain_visible_and_reject_reuse() {
        let root = temp_workspace("oversized");
        fs::write(
            root.join("devcontainer.json"),
            vec![b' '; MAX_CAPSULE_SOURCE_BYTES as usize + 1],
        )
        .expect("oversized body");

        let resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let source = resolution
            .sources
            .iter()
            .find(|source| source.source_class == CapsuleBetaSourceClass::Devcontainer)
            .expect("degraded source remains visible");

        assert_eq!(
            source.read_state,
            CapsuleBetaSourceReadState::ResourceLimitExceeded
        );
        assert!(source.content_digest.is_none());
        assert!(source
            .notes
            .contains(&CapsuleBetaSourceNote::SourceResourceLimitExceeded));
        assert_eq!(resolution.drift_state, CapsuleDriftState::UnknownLineage);
        assert_eq!(
            resolution.prebuild_reuse_state,
            PrebuildReuseState::RejectedDrift
        );
        assert_eq!(
            resolution.environment_capsule_ref.capsule_id,
            "capsule.beta.devcontainer.unavailable"
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn aggregate_budget_is_enforced_across_separate_sources() {
        let root = temp_workspace("aggregate-budget");
        fs::write(root.join("one"), [1_u8, 2]).expect("first source");
        fs::write(root.join("two"), [3_u8, 4]).expect("second source");
        let mut budget = CapsuleSourceReadBudget { remaining_bytes: 3 };

        assert_eq!(budget.read(&root, "one"), Ok(Some(vec![1_u8, 2])));
        assert_eq!(
            budget.read(&root, "two"),
            Err(CapsuleBetaSourceReadState::ResourceLimitExceeded)
        );

        fs::remove_dir_all(root).ok();
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_sources_remain_visible_as_unavailable_evidence() {
        use std::os::unix::fs::symlink;

        let root = temp_workspace("symlink-source");
        fs::write(root.join("real.json"), r#"{"image":"example:1"}"#).expect("real body");
        symlink("real.json", root.join("devcontainer.json")).expect("source symlink");

        let resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        let source = resolution
            .sources
            .iter()
            .find(|source| source.source_class == CapsuleBetaSourceClass::Devcontainer)
            .expect("degraded source remains visible");

        assert_eq!(source.read_state, CapsuleBetaSourceReadState::Unavailable);
        assert!(source.content_digest.is_none());
        assert!(source
            .notes
            .contains(&CapsuleBetaSourceNote::SourceReadUnavailable));
        assert_eq!(resolution.drift_state, CapsuleDriftState::UnknownLineage);
        assert_eq!(
            resolution.prebuild_reuse_state,
            PrebuildReuseState::RejectedDrift
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn coverage_manifest_pins_full_source_vocabulary() {
        let manifest = EnvironmentCapsuleBetaCoverageManifest::canonical(
            "environment-capsule-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert!(manifest.covers_every_source_class());
        assert_eq!(
            manifest.source_classes.len(),
            CapsuleBetaSourceClass::ALL.len()
        );
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
        assert_eq!(
            round.schema_version,
            ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_SCHEMA_VERSION
        );
        assert!(!round.raw_private_material_exported);
        assert_eq!(round.resolution_projection.source_count_observed, 1);
        assert_eq!(round.resolution_projection.source_count_exported, 1);
        assert_eq!(round.resolution_projection.source_count_omitted, 0);
        let serialized = serde_json::to_string(&round).expect("serialize projection");
        assert!(!serialized.contains(root.to_string_lossy().as_ref()));
        assert!(!serialized.contains("ghcr.io/example/runtime:1"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn support_export_redacts_private_fields_and_bounds_untrusted_collections() {
        let root = temp_workspace("support-redaction");
        fs::write(
            root.join("devcontainer.json"),
            r#"{
                "image": "registry.example/private/team/runtime:secret",
                "features": {"private.example/feature": {}},
                "postCreateCommand": "printenv PRIVATE_TOKEN"
            }"#,
        )
        .expect("write");
        let mut resolution = EnvironmentCapsuleBetaResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::BackendService);
        resolution.sources[0].content_digest = Some("private-invalid-digest".to_owned());
        let duplicate_source = resolution.sources[0].clone();
        for _ in 0..10 {
            resolution.sources.push(duplicate_source.clone());
        }
        let manifest_id = "manifest:/private/operator/name";
        let packet = EnvironmentCapsuleBetaSupportExport::new(
            manifest_id,
            "invalid timestamp and private note",
            resolution,
            Vec::new(),
        );
        let json = serde_json::to_string(&packet).expect("serialize");

        assert_eq!(packet.generated_at, "1970-01-01T00:00:00Z");
        assert_ne!(packet.manifest_id_digest, manifest_id);
        assert!(!json.contains(manifest_id));
        assert!(!json.contains(root.to_string_lossy().as_ref()));
        assert!(!json.contains("registry.example"));
        assert!(!json.contains("private.example"));
        assert!(!json.contains("PRIVATE_TOKEN"));
        assert!(!json.contains("private-invalid-digest"));
        assert!(!json.contains("\"workspace_root_ref\":"));
        assert!(!json.contains("\"source_ref\":"));
        assert!(!json.contains("\"alpha_resolution\":"));
        assert_eq!(
            packet.resolution_projection.source_projections[0].read_state,
            CapsuleBetaSourceReadState::Unavailable
        );
        assert_eq!(packet.resolution_projection.source_count_observed, 11);
        assert_eq!(packet.resolution_projection.source_count_exported, 1);
        assert_eq!(packet.resolution_projection.source_count_omitted, 10);
        assert!(packet.resolution_projection.source_projections[0]
            .content_digest
            .is_none());
        assert!(packet.field_dispositions.iter().any(|row| {
            row.field_path == "input.generated_at"
                && row.disposition == CapsuleBetaSupportFieldDispositionClass::UnavailableSource
        }));
        assert!(packet.field_dispositions.iter().any(|row| {
            row.field_path == "bounded_collection_tails"
                && row.disposition == CapsuleBetaSupportFieldDispositionClass::OmittedByRedaction
                && row.redaction_profile_ref.as_deref()
                    == Some(ENVIRONMENT_CAPSULE_BETA_SUPPORT_REDACTION_PROFILE_REF)
        }));
        assert!(packet.field_dispositions.iter().all(|row| {
            !matches!(
                row.disposition,
                CapsuleBetaSupportFieldDispositionClass::NotRecordedByDesign
                    | CapsuleBetaSupportFieldDispositionClass::OmittedByPolicy
            )
        }));
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
