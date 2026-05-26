//! Mutation-journal and generated-artifact lineage: the governed,
//! export-safe projection that proves how Aureline tracks mutations
//! across editor, formatter, AI, build, lockfile, and preview paths
//! and how it labels the generated and mirrored artifacts those paths
//! produce.
//!
//! Where the recovery-ladder lineage proves the ordered recovery
//! sequence and the cache / storage-class lineage proves the storage
//! layer under caches and durable state, this projection proves the
//! *mutation surface* on top of both: which mutation paths exist, how
//! each path's journal entry is no-rerun safe, which generated or
//! mirrored artifact classes are governed, which canonical source ref
//! each artifact carries, which generator or signer identity signed
//! it, which output digest pins it, which drift state and default
//! edit posture it claims, and which surfaces (tree, breadcrumb,
//! search, review, AI-context, save, support) label it so the user
//! never sees a generated artifact treated as the canonical edit
//! target.
//!
//! The projection ingests a live
//! [`MutationAndGeneratedArtifactInputs`] envelope verbatim (one
//! [`MutationPathObservation`] per mutation path, one
//! [`GeneratedArtifactObservation`] per governed artifact, plus the
//! controlled inspection-hook table) and produces a lineage record
//! that proves the contract claims the stable line is anchored on:
//!
//! - **Mutation-path coverage truth.** Every governed mutation path
//!   ships a row bound to one closed [`MutationPathKind`] (editor,
//!   formatter, ai_apply, build_runner, lockfile_resolver,
//!   preview_runtime). Missing required paths narrow the corpus.
//! - **Generated-artifact coverage truth.** Every required generated
//!   or mirrored artifact class ships a row bound to one closed
//!   [`GeneratedArtifactKind`] (build_output, generated_source_sibling,
//!   structured_lockfile, notebook_output, preview_snapshot). The
//!   optional mirrored / design-snapshot artifact classes ride on top
//!   without changing the required set.
//! - **Canonical-lineage truth.** Every artifact row references a
//!   non-empty canonical source ref, a generator or signer identity,
//!   and an output digest so consumers can pin the canonical sibling
//!   and the regeneration provenance before re-running.
//! - **Drift truth.** Every artifact row declares one closed
//!   [`DriftStateClass`]; an artifact that overrode the default edit
//!   posture must be in `drifted_from_generator` and reference a
//!   recovery / regenerate guidance disclosure id.
//! - **Edit-posture honesty.** Every non-authoritative artifact
//!   declares one closed [`DefaultEditPostureClass`]; only artifact
//!   classes that explicitly support round-trip-safe editing may
//!   declare `round_trip_safe`. Diverged artifacts must reference both
//!   the override disclosure id and the recovery / regenerate
//!   guidance.
//! - **Labeling-surface coverage truth.** Every required labeling
//!   surface (tree, breadcrumb, search, review, ai_context, save,
//!   support) is reachable across the corpus and every artifact
//!   declares the surfaces that label it.
//! - **No-rerun honesty.** Every mutation-path row declares one
//!   closed [`MutationNoRerunPosture`]; privileged paths
//!   (`ai_apply`, `build_runner`, `lockfile_resolver`,
//!   `preview_runtime`) must declare `explicit_user_action_required`
//!   (or `terminal_no_further_run`) with both a commit action id and
//!   a commit disclosure id so resume / reconnect / recovery cannot
//!   silently replay AI apply, build runs, lockfile resolves, or
//!   preview regenerations.
//! - **Support-export honesty.** Each row's support-export projection
//!   preserves the mutation path or artifact class, the canonical
//!   source ref, the generator or signer identity, the output digest,
//!   the drift state, the default edit posture, the labeling
//!   surfaces, and the disclosure ids while excluding raw secrets,
//!   approval tickets, delegated credentials, and live authority
//!   handles.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks
//!   (`inspect_lineage`, `compare_canonical`, `regenerate`,
//!   `export_before_override`, `rollback_override`, `export`,
//!   `repair`) is reachable so destructive overrides and regenerations
//!   stay reviewable.
//! - **Producer attribution.** Each record carries the producer ref,
//!   the schema version, the capture timestamp, and an integrity hash
//!   derived from the input identities so replay and support
//!   pipelines can pin the source before applying.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to the
//!   source corpus, workspace, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`MutationAndGeneratedArtifactLineageRecord`].
pub const MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the mutation / generated-artifact lineage
/// record.
pub const MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/mutation_and_generated_artifact_lineage.schema.json";

/// Stable record-kind tag for the mutation / generated-artifact
/// lineage record.
pub const MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_RECORD_KIND: &str =
    "mutation_and_generated_artifact_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the mutation paths Aureline journals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationPathKind {
    /// Editor refactor / rename / multi-cursor edit mutation.
    Editor,
    /// Formatter or save-participant formatting mutation.
    Formatter,
    /// AI patch apply after a reviewed proposal.
    AiApply,
    /// Build runner writing compiler / bundler / packager output.
    BuildRunner,
    /// Resolver-created lockfile refresh or write.
    LockfileResolver,
    /// Preview runtime regenerating a derived render snapshot.
    PreviewRuntime,
}

impl MutationPathKind {
    /// Returns the stable snake_case token for this mutation path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Formatter => "formatter",
            Self::AiApply => "ai_apply",
            Self::BuildRunner => "build_runner",
            Self::LockfileResolver => "lockfile_resolver",
            Self::PreviewRuntime => "preview_runtime",
        }
    }

    /// True when this mutation path touches privileged / mutating
    /// surfaces (AI apply, build runs, lockfile resolves, preview
    /// regenerations) and therefore must declare a safe no-rerun
    /// posture before resume / reconnect can fire.
    pub const fn touches_privileged_surface(self) -> bool {
        matches!(
            self,
            Self::AiApply
                | Self::BuildRunner
                | Self::LockfileResolver
                | Self::PreviewRuntime
        )
    }
}

/// Closed list of mutation paths every lineage record must seed.
pub const REQUIRED_MUTATION_PATHS: [MutationPathKind; 6] = [
    MutationPathKind::Editor,
    MutationPathKind::Formatter,
    MutationPathKind::AiApply,
    MutationPathKind::BuildRunner,
    MutationPathKind::LockfileResolver,
    MutationPathKind::PreviewRuntime,
];

/// Closed vocabulary for the generated / mirrored artifact classes
/// the lineage covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedArtifactKind {
    /// Build output (compiler / bundler / packager artifact).
    BuildOutput,
    /// Generated source file co-located with a hand-authored sibling
    /// (e.g. `*.gen.rs`, `*_pb.rs`).
    GeneratedSourceSibling,
    /// Structured lockfile resolved from a manifest.
    StructuredLockfile,
    /// Notebook output cell (computed output not part of user source).
    NotebookOutput,
    /// Preview runtime snapshot of a render / live preview.
    PreviewSnapshot,
    /// Design-tool snapshot mirrored into the workspace.
    DesignSnapshot,
    /// Mirrored documentation artifact (e.g. published API docs).
    MirroredDocArtifact,
    /// Mirrored schema artifact (e.g. generated OpenAPI / GraphQL
    /// schema).
    MirroredSchemaArtifact,
    /// Mirrored model artifact (e.g. exported ML model snapshot).
    MirroredModelArtifact,
    /// Mirrored registry artifact (e.g. snapshot of a package /
    /// container registry).
    MirroredRegistryArtifact,
}

impl GeneratedArtifactKind {
    /// Returns the stable snake_case token for this artifact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildOutput => "build_output",
            Self::GeneratedSourceSibling => "generated_source_sibling",
            Self::StructuredLockfile => "structured_lockfile",
            Self::NotebookOutput => "notebook_output",
            Self::PreviewSnapshot => "preview_snapshot",
            Self::DesignSnapshot => "design_snapshot",
            Self::MirroredDocArtifact => "mirrored_doc_artifact",
            Self::MirroredSchemaArtifact => "mirrored_schema_artifact",
            Self::MirroredModelArtifact => "mirrored_model_artifact",
            Self::MirroredRegistryArtifact => "mirrored_registry_artifact",
        }
    }

    /// True when the artifact class is part of the required set every
    /// Stable corpus must seed.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::BuildOutput
                | Self::GeneratedSourceSibling
                | Self::StructuredLockfile
                | Self::NotebookOutput
                | Self::PreviewSnapshot
        )
    }

    /// True when the artifact class is allowed to declare
    /// `round_trip_safe` as its default edit posture (i.e. direct
    /// editing safely round-trips through the generator).
    pub const fn supports_round_trip_safe_editing(self) -> bool {
        // Structured lockfiles support certain narrow round-trip
        // edits (e.g. pinning a version under tooling guidance);
        // every other generated artifact must default to blocking
        // direct writes.
        matches!(self, Self::StructuredLockfile)
    }
}

/// Closed list of generated-artifact classes every lineage record
/// must seed.
pub const REQUIRED_GENERATED_ARTIFACT_CLASSES: [GeneratedArtifactKind; 5] = [
    GeneratedArtifactKind::BuildOutput,
    GeneratedArtifactKind::GeneratedSourceSibling,
    GeneratedArtifactKind::StructuredLockfile,
    GeneratedArtifactKind::NotebookOutput,
    GeneratedArtifactKind::PreviewSnapshot,
];

/// Closed default-edit-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultEditPostureClass {
    /// Direct writes are blocked by default; the user must pivot to
    /// the canonical source or explicitly override.
    BlockWritesDefault,
    /// The artifact class supports round-trip-safe direct editing.
    RoundTripSafe,
    /// The user overrode the default edit posture; the artifact is
    /// in a visible diverged state and must surface recovery /
    /// regenerate guidance.
    DivergedFromGenerator,
}

impl DefaultEditPostureClass {
    /// Returns the stable snake_case token for this edit posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockWritesDefault => "block_writes_default",
            Self::RoundTripSafe => "round_trip_safe",
            Self::DivergedFromGenerator => "diverged_from_generator",
        }
    }

    /// True when this posture requires a recovery / regenerate
    /// guidance disclosure id.
    pub const fn requires_recovery_disclosure(self) -> bool {
        matches!(self, Self::DivergedFromGenerator)
    }
}

/// Closed drift-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftStateClass {
    /// Artifact digest matches the most recent generator run.
    InSync,
    /// Artifact diverges from its generator and must surface
    /// recovery / regenerate guidance.
    DriftedFromGenerator,
    /// Regeneration is in flight; the artifact may transiently
    /// disagree with its canonical source.
    RegenerationPending,
    /// Drift state could not be determined from the captured inputs.
    UnknownDrift,
}

impl DriftStateClass {
    /// Returns the stable snake_case token for this drift state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::DriftedFromGenerator => "drifted_from_generator",
            Self::RegenerationPending => "regeneration_pending",
            Self::UnknownDrift => "unknown_drift",
        }
    }
}

/// Closed labeling-surface vocabulary — the surfaces that must
/// disclose a generated or mirrored artifact's nature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelingSurfaceKind {
    /// Explorer tree row.
    Tree,
    /// Editor breadcrumb / chrome row.
    Breadcrumb,
    /// Search result row.
    Search,
    /// Review / diff surface row.
    Review,
    /// AI-context inclusion hint (so the model never assumes a
    /// generated artifact is the canonical edit target).
    AiContext,
    /// Save / save-participant surface.
    Save,
    /// Support / diagnostic export surface.
    Support,
}

impl LabelingSurfaceKind {
    /// Returns the stable snake_case token for this labeling surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::Breadcrumb => "breadcrumb",
            Self::Search => "search",
            Self::Review => "review",
            Self::AiContext => "ai_context",
            Self::Save => "save",
            Self::Support => "support",
        }
    }
}

/// Closed list of labeling surfaces every governed artifact must
/// disclose itself on.
pub const REQUIRED_LABELING_SURFACES: [LabelingSurfaceKind; 7] = [
    LabelingSurfaceKind::Tree,
    LabelingSurfaceKind::Breadcrumb,
    LabelingSurfaceKind::Search,
    LabelingSurfaceKind::Review,
    LabelingSurfaceKind::AiContext,
    LabelingSurfaceKind::Save,
    LabelingSurfaceKind::Support,
];

/// Closed no-rerun posture vocabulary for mutation paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationNoRerunPosture {
    /// The mutation re-runs only after an explicit user action with
    /// a disclosure.
    ExplicitUserActionRequired,
    /// The mutation only re-runs deterministically against a captured
    /// rollback checkpoint; safe for non-privileged surfaces only.
    DeterministicReplayAfterCheckpoint,
    /// The mutation is terminal — no further automatic re-run will
    /// fire.
    TerminalNoFurtherRun,
}

impl MutationNoRerunPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserActionRequired => "explicit_user_action_required",
            Self::DeterministicReplayAfterCheckpoint => "deterministic_replay_after_checkpoint",
            Self::TerminalNoFurtherRun => "terminal_no_further_run",
        }
    }

    /// True when the posture is safe for a privileged mutation path
    /// (only `explicit_user_action_required` and
    /// `terminal_no_further_run` are safe).
    pub const fn safe_for_privileged_path(self) -> bool {
        matches!(
            self,
            Self::ExplicitUserActionRequired | Self::TerminalNoFurtherRun
        )
    }
}

/// Closed support-export posture vocabulary for the mutation /
/// generated-artifact lineage rows. Local-only postures are refused
/// when the row touches user-authored material, so every governed
/// row ships either `metadata_safe_export` or `held_record`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationSupportExportPosture {
    /// Row ships a metadata-safe projection in the support packet.
    MetadataSafeExport,
    /// Row withholds its state until manual review.
    HeldRecord,
}

impl MutationSupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Class of pre-action inspection / repair hook offered before any
/// destructive override or regenerate commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationAndGeneratedArtifactInspectionHookClass {
    /// Open the lineage inspector with the current mutation path,
    /// artifact rows, and drift summary.
    InspectLineage,
    /// Compare the artifact against its canonical source before any
    /// override commits.
    CompareCanonical,
    /// Regenerate the artifact from its canonical source rather than
    /// silently writing through.
    Regenerate,
    /// Export the artifact state before overriding the default edit
    /// posture so the diverged state can be restored.
    ExportBeforeOverride,
    /// Roll a `diverged_from_generator` artifact back to its
    /// generator-aligned state.
    RollbackOverride,
    /// Export the lineage record itself (support-safe).
    Export,
    /// Open the typed repair sheet for the current drift / override.
    Repair,
}

impl MutationAndGeneratedArtifactInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectLineage => "inspect_lineage",
            Self::CompareCanonical => "compare_canonical",
            Self::Regenerate => "regenerate",
            Self::ExportBeforeOverride => "export_before_override",
            Self::RollbackOverride => "rollback_override",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-action inspection / repair hook offered before a
/// mutation / artifact override commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAndGeneratedArtifactInspectionHook {
    /// Hook class.
    pub hook_class: MutationAndGeneratedArtifactInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-action inspection / repair hook table.
pub fn default_mutation_and_generated_artifact_inspection_hooks(
) -> Vec<MutationAndGeneratedArtifactInspectionHook> {
    vec![
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::InspectLineage,
            action_id: "mutation_and_generated_artifact.inspect_lineage".to_owned(),
            label: "Inspect mutation and artifact lineage".to_owned(),
            available: true,
            disclosure:
                "Opens the lineage inspector with the current mutation paths, governed artifacts, drift state, and labeling surfaces before any override commits."
                    .to_owned(),
        },
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::CompareCanonical,
            action_id: "mutation_and_generated_artifact.compare_canonical".to_owned(),
            label: "Compare against canonical source".to_owned(),
            available: true,
            disclosure:
                "Renders the diff between the generated artifact and its canonical source so the user can review divergence before overriding the default edit posture."
                    .to_owned(),
        },
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::Regenerate,
            action_id: "mutation_and_generated_artifact.regenerate".to_owned(),
            label: "Regenerate from canonical source".to_owned(),
            available: true,
            disclosure:
                "Regenerates the artifact from its canonical source rather than silently writing through the generated layer."
                    .to_owned(),
        },
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::ExportBeforeOverride,
            action_id: "mutation_and_generated_artifact.export_before_override".to_owned(),
            label: "Export before override".to_owned(),
            available: true,
            disclosure:
                "Exports the current artifact and lineage state before the user overrides the default edit posture so the diverged state can be restored."
                    .to_owned(),
        },
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::RollbackOverride,
            action_id: "mutation_and_generated_artifact.rollback_override".to_owned(),
            label: "Roll back override".to_owned(),
            available: true,
            disclosure:
                "Reverts a diverged-from-generator artifact back to its generator-aligned state once the user finishes inspecting the override."
                    .to_owned(),
        },
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::Export,
            action_id: "mutation_and_generated_artifact.export".to_owned(),
            label: "Export mutation / artifact lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this mutation / generated-artifact lineage record for support without raw secrets, approval tickets, or delegated credentials."
                    .to_owned(),
        },
        MutationAndGeneratedArtifactInspectionHook {
            hook_class: MutationAndGeneratedArtifactInspectionHookClass::Repair,
            action_id: "mutation_and_generated_artifact.repair".to_owned(),
            label: "Open typed repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the typed repair sheet for the current drift / override and surfaces the preview, blast-radius disclosure, and reversal semantics rather than firing the repair as a shortcut."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a mutation /
/// artifact row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationSupportExportInputs {
    pub posture: MutationSupportExportPosture,
    pub includes_path_or_class: bool,
    pub includes_canonical_source_ref: bool,
    pub includes_generator_identity: bool,
    pub includes_output_digest: bool,
    pub includes_drift_state: bool,
    pub includes_default_edit_posture: bool,
    pub includes_labeling_surfaces: bool,
    pub includes_disclosure_ids: bool,
    pub raw_secrets_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl MutationSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: MutationSupportExportPosture) -> Self {
        Self {
            posture,
            includes_path_or_class: true,
            includes_canonical_source_ref: true,
            includes_generator_identity: true,
            includes_output_digest: true,
            includes_drift_state: true,
            includes_default_edit_posture: true,
            includes_labeling_surfaces: true,
            includes_disclosure_ids: true,
            raw_secrets_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// One observation of a governed mutation path at a captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPathObservation {
    /// Stable mutation-path id (e.g. `mutation_path.ai_apply`).
    pub path_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed mutation-path kind.
    pub path_kind: MutationPathKind,
    /// Stable id of the journal entry the path produces.
    pub journal_entry_id: String,
    /// Declared no-rerun posture.
    pub no_rerun_posture: MutationNoRerunPosture,
    /// Stable id of the action that commits this mutation (required
    /// for `explicit_user_action_required` paths).
    pub commit_action_id: String,
    /// Stable id of the disclosure paired with the commit action
    /// (required for `explicit_user_action_required` paths).
    pub commit_disclosure_id: String,
    /// Whether this mutation path touches privileged / mutating
    /// surfaces beyond the editor buffer (resolved automatically from
    /// the path kind but mirrored on the observation so support
    /// exports surface it).
    pub touches_privileged_surface: bool,
    /// Support-export projection for the mutation-path row.
    pub support_export: MutationSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One observation of a governed generated / mirrored artifact at a
/// captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactObservation {
    /// Stable artifact id (e.g. `artifact.cargo_lockfile`).
    pub artifact_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed artifact class.
    pub artifact_kind: GeneratedArtifactKind,
    /// Stable canonical source ref (e.g. `manifest.cargo`,
    /// `proto.users_v1`).
    pub canonical_source_ref: String,
    /// Stable generator / signer identity (e.g.
    /// `generator:cargo_lockfile@1.78`).
    pub generator_identity: String,
    /// Output digest pinning the artifact to its generator run.
    pub output_digest: String,
    /// Declared drift state.
    pub drift_state: DriftStateClass,
    /// Declared default edit posture.
    pub default_edit_posture: DefaultEditPostureClass,
    /// Stable id of the override disclosure (required when the
    /// posture is `diverged_from_generator`).
    pub override_disclosure_id: String,
    /// Stable id of the recovery / regenerate guidance disclosure
    /// (required when the posture is `diverged_from_generator` or
    /// the drift state is `drifted_from_generator`).
    pub recovery_guidance_disclosure_id: String,
    /// Surfaces the artifact is labeled on. Each governed artifact
    /// must label itself on every required surface.
    pub labeled_in_surfaces: BTreeSet<LabelingSurfaceKind>,
    /// Support-export projection for the artifact row.
    pub support_export: MutationSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAndGeneratedArtifactInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured mutation-path observations.
    pub mutation_paths: Vec<MutationPathObservation>,
    /// Captured generated / mirrored artifact observations.
    pub generated_artifacts: Vec<GeneratedArtifactObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a mutation / generated-artifact lineage record
/// narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationAndGeneratedArtifactLineageNarrowReason {
    /// The captured input had no mutation paths or no generated
    /// artifacts.
    CorpusEmpty,
    /// A required mutation-path kind is missing from the corpus.
    RequiredMutationPathMissing,
    /// A required generated-artifact class is missing from the
    /// corpus.
    RequiredGeneratedArtifactClassMissing,
    /// A generated-artifact row is missing its canonical source ref.
    CanonicalSourceRefMissing,
    /// A generated-artifact row is missing its generator / signer
    /// identity.
    GeneratorIdentityMissing,
    /// A generated-artifact row is missing its output digest.
    OutputDigestMissing,
    /// A generated-artifact row declared `drifted_from_generator`
    /// drift but did not reference a recovery / regenerate guidance
    /// disclosure id.
    DriftDisclosureMissing,
    /// A non-round-trip-safe artifact declared `round_trip_safe` as
    /// its default edit posture.
    EditPostureUnsafeDefault,
    /// A `diverged_from_generator` artifact is missing either the
    /// override disclosure id or the recovery / regenerate guidance
    /// disclosure id.
    DivergedDisclosureMissing,
    /// A governed artifact is not labeled on one of the required
    /// labeling surfaces.
    LabelingSurfaceMissing,
    /// A privileged mutation path declared a no-rerun posture other
    /// than `explicit_user_action_required` or
    /// `terminal_no_further_run`.
    MutationNoRerunPostureUnsafe,
    /// An `explicit_user_action_required` mutation path is missing
    /// the commit action id or commit disclosure id.
    ExplicitActionMetadataMissing,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, approval tickets, delegated credentials, or live
    /// authority handles slipped into a support-export projection.
    SupportExportRedactionUnsafe,
    /// Producer attribution is incomplete (producer ref or
    /// captured-at empty).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl MutationAndGeneratedArtifactLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredMutationPathMissing => "required_mutation_path_missing",
            Self::RequiredGeneratedArtifactClassMissing => {
                "required_generated_artifact_class_missing"
            }
            Self::CanonicalSourceRefMissing => "canonical_source_ref_missing",
            Self::GeneratorIdentityMissing => "generator_identity_missing",
            Self::OutputDigestMissing => "output_digest_missing",
            Self::DriftDisclosureMissing => "drift_disclosure_missing",
            Self::EditPostureUnsafeDefault => "edit_posture_unsafe_default",
            Self::DivergedDisclosureMissing => "diverged_disclosure_missing",
            Self::LabelingSurfaceMissing => "labeling_surface_missing",
            Self::MutationNoRerunPostureUnsafe => "mutation_no_rerun_posture_unsafe",
            Self::ExplicitActionMetadataMissing => "explicit_action_metadata_missing",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a mutation / generated-artifact
/// lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAndGeneratedArtifactLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<MutationAndGeneratedArtifactLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One mutation-path row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPathRow {
    /// Stable mutation-path id.
    pub path_id: String,
    /// Mutation-path title.
    pub title: String,
    /// Mutation-path kind.
    pub path_kind: MutationPathKind,
    /// Stable id of the journal entry the path produces.
    pub journal_entry_id: String,
    /// No-rerun posture.
    pub no_rerun_posture: MutationNoRerunPosture,
    /// Commit action id.
    pub commit_action_id: String,
    /// Commit disclosure id.
    pub commit_disclosure_id: String,
    /// Whether the mutation touches privileged / mutating surfaces.
    pub touches_privileged_surface: bool,
    /// Re-derived value of `touches_privileged_surface` from the
    /// path kind (so support exports can detect mis-claimed rows).
    pub canonical_touches_privileged_surface: bool,
    /// True when declared and canonical privileged-surface flags
    /// agree.
    pub privileged_surface_matches: bool,
    /// Support-export posture.
    pub support_export_posture: MutationSupportExportPosture,
    /// True when this mutation path is required by the contract.
    pub is_required: bool,
}

/// One generated / mirrored artifact row carried in the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactRow {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Artifact title.
    pub title: String,
    /// Artifact kind.
    pub artifact_kind: GeneratedArtifactKind,
    /// Canonical source ref.
    pub canonical_source_ref: String,
    /// Generator / signer identity.
    pub generator_identity: String,
    /// Output digest.
    pub output_digest: String,
    /// Drift state.
    pub drift_state: DriftStateClass,
    /// Default edit posture.
    pub default_edit_posture: DefaultEditPostureClass,
    /// Override disclosure id.
    pub override_disclosure_id: String,
    /// Recovery / regenerate guidance disclosure id.
    pub recovery_guidance_disclosure_id: String,
    /// Labeling surfaces the artifact discloses itself on.
    pub labeled_in_surfaces: BTreeSet<LabelingSurfaceKind>,
    /// Labeling surfaces still required for the artifact (empty when
    /// all required surfaces are present).
    pub missing_labeling_surfaces: BTreeSet<LabelingSurfaceKind>,
    /// True when the artifact class supports round-trip-safe editing.
    pub supports_round_trip_safe_editing: bool,
    /// Support-export posture.
    pub support_export_posture: MutationSupportExportPosture,
    /// True when this artifact class is required by the contract.
    pub is_required: bool,
}

/// Mutation-path coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPathCoverageSummary {
    /// All mutation-path rows carried by the corpus.
    pub mutation_path_rows: Vec<MutationPathRow>,
    /// True when every required mutation path is present.
    pub all_required_paths_present: bool,
}

/// Generated-artifact coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactCoverageSummary {
    /// All generated / mirrored artifact rows carried by the corpus.
    pub generated_artifact_rows: Vec<GeneratedArtifactRow>,
    /// True when every required artifact class is present.
    pub all_required_artifact_classes_present: bool,
}

/// Canonical-lineage truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalLineageTruthSummary {
    /// True when every artifact row carries a non-empty canonical
    /// source ref.
    pub all_artifacts_have_canonical_source_ref: bool,
    /// True when every artifact row carries a non-empty generator /
    /// signer identity.
    pub all_artifacts_have_generator_identity: bool,
    /// True when every artifact row carries a non-empty output
    /// digest.
    pub all_artifacts_have_output_digest: bool,
}

/// Drift truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthSummary {
    /// Number of artifact rows declared `drifted_from_generator`.
    pub drifted_artifact_count: usize,
    /// True when every drifted artifact references a recovery /
    /// regenerate guidance disclosure id.
    pub all_drifted_artifacts_have_disclosure: bool,
}

/// Edit-posture honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditPostureHonestySummary {
    /// True when only artifact classes that explicitly support
    /// round-trip-safe editing declare `round_trip_safe`.
    pub all_round_trip_safe_claims_supported: bool,
    /// Number of artifact rows declared `diverged_from_generator`.
    pub diverged_artifact_count: usize,
    /// True when every diverged artifact references both an
    /// override disclosure id and a recovery / regenerate guidance
    /// disclosure id.
    pub all_diverged_artifacts_have_disclosures: bool,
}

/// Labeling-surface coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelingSurfaceCoverageSummary {
    /// True when every governed artifact is labeled on every
    /// required surface.
    pub all_artifacts_labeled_on_required_surfaces: bool,
}

/// Mutation no-rerun honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationNoRerunHonestySummary {
    /// True when every privileged mutation path declares a safe
    /// no-rerun posture.
    pub all_privileged_paths_safe: bool,
    /// True when every `explicit_user_action_required` mutation path
    /// references both a commit action id and a commit disclosure id.
    pub all_explicit_paths_have_metadata: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationSupportExportHonestySummary {
    /// True when every row's support-export projection preserves the
    /// required fields.
    pub all_rows_preserve_fields: bool,
    /// True when every row redacts raw secrets.
    pub all_rows_redact_raw_secrets: bool,
    /// True when every row excludes approval tickets.
    pub all_rows_exclude_approval_tickets: bool,
    /// True when every row excludes delegated credentials.
    pub all_rows_exclude_delegated_credentials: bool,
    /// True when every row excludes live authority handles.
    pub all_rows_exclude_live_authority_handles: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAndGeneratedArtifactProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Schema version pinned by the input.
    pub schema_version: u32,
    /// Opaque integrity hash derived from the input identities.
    pub integrity_hash: String,
    /// Input capture timestamp.
    pub captured_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe mutation / generated-artifact lineage record
/// per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAndGeneratedArtifactLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub mutation_and_generated_artifact_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque ref to the corpus the projection ingested.
    pub corpus_ref: String,
    /// Producer attribution pillar.
    pub producer_attribution: MutationAndGeneratedArtifactProducerAttributionSummary,
    /// Mutation-path coverage pillar.
    pub mutation_path_coverage: MutationPathCoverageSummary,
    /// Generated-artifact coverage pillar.
    pub generated_artifact_coverage: GeneratedArtifactCoverageSummary,
    /// Canonical-lineage truth pillar.
    pub canonical_lineage_truth: CanonicalLineageTruthSummary,
    /// Drift truth pillar.
    pub drift_truth: DriftTruthSummary,
    /// Edit-posture honesty pillar.
    pub edit_posture_honesty: EditPostureHonestySummary,
    /// Labeling-surface coverage pillar.
    pub labeling_surface_coverage: LabelingSurfaceCoverageSummary,
    /// Mutation no-rerun honesty pillar.
    pub mutation_no_rerun_honesty: MutationNoRerunHonestySummary,
    /// Support-export honesty pillar.
    pub support_export_honesty: MutationSupportExportHonestySummary,
    /// Pre-action inspection / repair hooks.
    pub inspection_hooks: Vec<MutationAndGeneratedArtifactInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: MutationAndGeneratedArtifactLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl MutationAndGeneratedArtifactLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF
            && self.record_kind == MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: MutationAndGeneratedArtifactInspectionHookClass,
    ) -> Option<&MutationAndGeneratedArtifactInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed mutation / generated-artifact lineage record
/// from a live [`MutationAndGeneratedArtifactInputs`] envelope using
/// the default inspection-hook set.
pub fn project_mutation_and_generated_artifact_lineage(
    posture_id: impl Into<String>,
    inputs: &MutationAndGeneratedArtifactInputs,
) -> MutationAndGeneratedArtifactLineageRecord {
    project_mutation_and_generated_artifact_lineage_with_hooks(
        posture_id,
        inputs,
        default_mutation_and_generated_artifact_inspection_hooks(),
    )
}

/// Like [`project_mutation_and_generated_artifact_lineage`] but with
/// an explicit inspection-hook set (for testing degraded-hook
/// postures).
pub fn project_mutation_and_generated_artifact_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &MutationAndGeneratedArtifactInputs,
    inspection_hooks: Vec<MutationAndGeneratedArtifactInspectionHook>,
) -> MutationAndGeneratedArtifactLineageRecord {
    let posture_id: String = posture_id.into();

    let mutation_path_coverage = project_mutation_path_coverage(inputs);
    let generated_artifact_coverage = project_generated_artifact_coverage(inputs);
    let canonical_lineage_truth = project_canonical_lineage_truth(&generated_artifact_coverage);
    let drift_truth = project_drift_truth(&generated_artifact_coverage);
    let edit_posture_honesty = project_edit_posture_honesty(&generated_artifact_coverage);
    let labeling_surface_coverage =
        project_labeling_surface_coverage(&generated_artifact_coverage);
    let mutation_no_rerun_honesty = project_mutation_no_rerun_honesty(&mutation_path_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.mutation_paths.is_empty() || inputs.generated_artifacts.is_empty() {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::CorpusEmpty);
    }
    if !mutation_path_coverage.all_required_paths_present {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::RequiredMutationPathMissing,
        );
    }
    if !generated_artifact_coverage.all_required_artifact_classes_present {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::RequiredGeneratedArtifactClassMissing,
        );
    }
    if !canonical_lineage_truth.all_artifacts_have_canonical_source_ref {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::CanonicalSourceRefMissing,
        );
    }
    if !canonical_lineage_truth.all_artifacts_have_generator_identity {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::GeneratorIdentityMissing);
    }
    if !canonical_lineage_truth.all_artifacts_have_output_digest {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::OutputDigestMissing);
    }
    if !drift_truth.all_drifted_artifacts_have_disclosure {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::DriftDisclosureMissing);
    }
    if !edit_posture_honesty.all_round_trip_safe_claims_supported {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::EditPostureUnsafeDefault);
    }
    if !edit_posture_honesty.all_diverged_artifacts_have_disclosures {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::DivergedDisclosureMissing);
    }
    if !labeling_surface_coverage.all_artifacts_labeled_on_required_surfaces {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::LabelingSurfaceMissing);
    }
    if !mutation_no_rerun_honesty.all_privileged_paths_safe {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::MutationNoRerunPostureUnsafe,
        );
    }
    if !mutation_no_rerun_honesty.all_explicit_paths_have_metadata {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::ExplicitActionMetadataMissing,
        );
    }

    let required_hooks = [
        MutationAndGeneratedArtifactInspectionHookClass::InspectLineage,
        MutationAndGeneratedArtifactInspectionHookClass::CompareCanonical,
        MutationAndGeneratedArtifactInspectionHookClass::Regenerate,
        MutationAndGeneratedArtifactInspectionHookClass::ExportBeforeOverride,
        MutationAndGeneratedArtifactInspectionHookClass::RollbackOverride,
        MutationAndGeneratedArtifactInspectionHookClass::Export,
        MutationAndGeneratedArtifactInspectionHookClass::Repair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::InspectionHookUnavailable);
    }

    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::ProducerAttributionIncomplete,
        );
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = MutationAndGeneratedArtifactLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &mutation_path_coverage,
        &generated_artifact_coverage,
        &drift_truth,
        &edit_posture_honesty,
        &stable_qualification,
    );

    MutationAndGeneratedArtifactLineageRecord {
        record_kind: MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
        mutation_and_generated_artifact_lineage_schema_version:
            MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION,
        schema_ref: MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        mutation_path_coverage,
        generated_artifact_coverage,
        canonical_lineage_truth,
        drift_truth,
        edit_posture_honesty,
        labeling_surface_coverage,
        mutation_no_rerun_honesty,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_mutation_path_coverage(
    inputs: &MutationAndGeneratedArtifactInputs,
) -> MutationPathCoverageSummary {
    let mutation_path_rows: Vec<MutationPathRow> = inputs
        .mutation_paths
        .iter()
        .map(project_mutation_path_row)
        .collect();
    let observed: BTreeSet<_> = mutation_path_rows
        .iter()
        .map(|row| row.path_kind)
        .collect();
    let all_required_paths_present = REQUIRED_MUTATION_PATHS
        .iter()
        .all(|required| observed.contains(required));
    MutationPathCoverageSummary {
        mutation_path_rows,
        all_required_paths_present,
    }
}

fn project_mutation_path_row(observation: &MutationPathObservation) -> MutationPathRow {
    let canonical_touches_privileged_surface =
        observation.path_kind.touches_privileged_surface();
    let privileged_surface_matches =
        observation.touches_privileged_surface == canonical_touches_privileged_surface;
    MutationPathRow {
        path_id: observation.path_id.clone(),
        title: observation.title.clone(),
        path_kind: observation.path_kind,
        journal_entry_id: observation.journal_entry_id.clone(),
        no_rerun_posture: observation.no_rerun_posture,
        commit_action_id: observation.commit_action_id.clone(),
        commit_disclosure_id: observation.commit_disclosure_id.clone(),
        touches_privileged_surface: observation.touches_privileged_surface,
        canonical_touches_privileged_surface,
        privileged_surface_matches,
        support_export_posture: observation.support_export.posture,
        is_required: REQUIRED_MUTATION_PATHS.contains(&observation.path_kind),
    }
}

fn project_generated_artifact_coverage(
    inputs: &MutationAndGeneratedArtifactInputs,
) -> GeneratedArtifactCoverageSummary {
    let generated_artifact_rows: Vec<GeneratedArtifactRow> = inputs
        .generated_artifacts
        .iter()
        .map(project_generated_artifact_row)
        .collect();
    let observed: BTreeSet<_> = generated_artifact_rows
        .iter()
        .map(|row| row.artifact_kind)
        .collect();
    let all_required_artifact_classes_present = REQUIRED_GENERATED_ARTIFACT_CLASSES
        .iter()
        .all(|required| observed.contains(required));
    GeneratedArtifactCoverageSummary {
        generated_artifact_rows,
        all_required_artifact_classes_present,
    }
}

fn project_generated_artifact_row(
    observation: &GeneratedArtifactObservation,
) -> GeneratedArtifactRow {
    let required: BTreeSet<LabelingSurfaceKind> =
        REQUIRED_LABELING_SURFACES.iter().copied().collect();
    let missing_labeling_surfaces: BTreeSet<LabelingSurfaceKind> = required
        .difference(&observation.labeled_in_surfaces)
        .copied()
        .collect();
    GeneratedArtifactRow {
        artifact_id: observation.artifact_id.clone(),
        title: observation.title.clone(),
        artifact_kind: observation.artifact_kind,
        canonical_source_ref: observation.canonical_source_ref.clone(),
        generator_identity: observation.generator_identity.clone(),
        output_digest: observation.output_digest.clone(),
        drift_state: observation.drift_state,
        default_edit_posture: observation.default_edit_posture,
        override_disclosure_id: observation.override_disclosure_id.clone(),
        recovery_guidance_disclosure_id: observation.recovery_guidance_disclosure_id.clone(),
        labeled_in_surfaces: observation.labeled_in_surfaces.clone(),
        missing_labeling_surfaces,
        supports_round_trip_safe_editing: observation
            .artifact_kind
            .supports_round_trip_safe_editing(),
        support_export_posture: observation.support_export.posture,
        is_required: observation.artifact_kind.is_required(),
    }
}

fn project_canonical_lineage_truth(
    coverage: &GeneratedArtifactCoverageSummary,
) -> CanonicalLineageTruthSummary {
    let mut all_artifacts_have_canonical_source_ref = true;
    let mut all_artifacts_have_generator_identity = true;
    let mut all_artifacts_have_output_digest = true;
    for row in &coverage.generated_artifact_rows {
        if row.canonical_source_ref.trim().is_empty() {
            all_artifacts_have_canonical_source_ref = false;
        }
        if row.generator_identity.trim().is_empty() {
            all_artifacts_have_generator_identity = false;
        }
        if row.output_digest.trim().is_empty() {
            all_artifacts_have_output_digest = false;
        }
    }
    CanonicalLineageTruthSummary {
        all_artifacts_have_canonical_source_ref,
        all_artifacts_have_generator_identity,
        all_artifacts_have_output_digest,
    }
}

fn project_drift_truth(coverage: &GeneratedArtifactCoverageSummary) -> DriftTruthSummary {
    let mut drifted_artifact_count = 0usize;
    let mut all_drifted_artifacts_have_disclosure = true;
    for row in &coverage.generated_artifact_rows {
        if row.drift_state == DriftStateClass::DriftedFromGenerator {
            drifted_artifact_count += 1;
            if row.recovery_guidance_disclosure_id.trim().is_empty() {
                all_drifted_artifacts_have_disclosure = false;
            }
        }
    }
    DriftTruthSummary {
        drifted_artifact_count,
        all_drifted_artifacts_have_disclosure,
    }
}

fn project_edit_posture_honesty(
    coverage: &GeneratedArtifactCoverageSummary,
) -> EditPostureHonestySummary {
    let mut all_round_trip_safe_claims_supported = true;
    let mut diverged_artifact_count = 0usize;
    let mut all_diverged_artifacts_have_disclosures = true;
    for row in &coverage.generated_artifact_rows {
        if row.default_edit_posture == DefaultEditPostureClass::RoundTripSafe
            && !row.supports_round_trip_safe_editing
        {
            all_round_trip_safe_claims_supported = false;
        }
        if row.default_edit_posture == DefaultEditPostureClass::DivergedFromGenerator {
            diverged_artifact_count += 1;
            if row.override_disclosure_id.trim().is_empty()
                || row.recovery_guidance_disclosure_id.trim().is_empty()
            {
                all_diverged_artifacts_have_disclosures = false;
            }
        }
    }
    EditPostureHonestySummary {
        all_round_trip_safe_claims_supported,
        diverged_artifact_count,
        all_diverged_artifacts_have_disclosures,
    }
}

fn project_labeling_surface_coverage(
    coverage: &GeneratedArtifactCoverageSummary,
) -> LabelingSurfaceCoverageSummary {
    let all_artifacts_labeled_on_required_surfaces = coverage
        .generated_artifact_rows
        .iter()
        .all(|row| row.missing_labeling_surfaces.is_empty());
    LabelingSurfaceCoverageSummary {
        all_artifacts_labeled_on_required_surfaces,
    }
}

fn project_mutation_no_rerun_honesty(
    coverage: &MutationPathCoverageSummary,
) -> MutationNoRerunHonestySummary {
    let mut all_privileged_paths_safe = true;
    let mut all_explicit_paths_have_metadata = true;
    for row in &coverage.mutation_path_rows {
        if row.canonical_touches_privileged_surface
            && !row.no_rerun_posture.safe_for_privileged_path()
        {
            all_privileged_paths_safe = false;
        }
        if row.no_rerun_posture == MutationNoRerunPosture::ExplicitUserActionRequired
            && (row.commit_action_id.trim().is_empty()
                || row.commit_disclosure_id.trim().is_empty())
        {
            all_explicit_paths_have_metadata = false;
        }
    }
    MutationNoRerunHonestySummary {
        all_privileged_paths_safe,
        all_explicit_paths_have_metadata,
    }
}

fn project_support_export_honesty(
    inputs: &MutationAndGeneratedArtifactInputs,
) -> MutationSupportExportHonestySummary {
    let mut all_rows_preserve_fields = true;
    let mut all_rows_redact_raw_secrets = true;
    let mut all_rows_exclude_approval_tickets = true;
    let mut all_rows_exclude_delegated_credentials = true;
    let mut all_rows_exclude_live_authority_handles = true;

    let mutation_supports = inputs.mutation_paths.iter().map(|p| p.support_export);
    let artifact_supports = inputs.generated_artifacts.iter().map(|a| a.support_export);

    for support in mutation_supports.chain(artifact_supports) {
        if !(support.includes_path_or_class
            && support.includes_canonical_source_ref
            && support.includes_generator_identity
            && support.includes_output_digest
            && support.includes_drift_state
            && support.includes_default_edit_posture
            && support.includes_labeling_surfaces
            && support.includes_disclosure_ids)
        {
            all_rows_preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            all_rows_redact_raw_secrets = false;
        }
        if !support.approval_tickets_excluded {
            all_rows_exclude_approval_tickets = false;
        }
        if !support.delegated_credentials_excluded {
            all_rows_exclude_delegated_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            all_rows_exclude_live_authority_handles = false;
        }
    }

    MutationSupportExportHonestySummary {
        all_rows_preserve_fields,
        all_rows_redact_raw_secrets,
        all_rows_exclude_approval_tickets,
        all_rows_exclude_delegated_credentials,
        all_rows_exclude_live_authority_handles,
    }
}

fn project_producer_attribution(
    inputs: &MutationAndGeneratedArtifactInputs,
) -> MutationAndGeneratedArtifactProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    MutationAndGeneratedArtifactProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_support_export_narrows(
    summary: &MutationSupportExportHonestySummary,
    narrow_reasons: &mut Vec<MutationAndGeneratedArtifactLineageNarrowReason>,
) {
    if !summary.all_rows_preserve_fields {
        narrow_reasons
            .push(MutationAndGeneratedArtifactLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !(summary.all_rows_redact_raw_secrets
        && summary.all_rows_exclude_approval_tickets
        && summary.all_rows_exclude_delegated_credentials
        && summary.all_rows_exclude_live_authority_handles)
    {
        narrow_reasons.push(
            MutationAndGeneratedArtifactLineageNarrowReason::SupportExportRedactionUnsafe,
        );
    }
}

fn compute_integrity_hash(inputs: &MutationAndGeneratedArtifactInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for path in &inputs.mutation_paths {
        for byte in path.path_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(path.path_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(path.no_rerun_posture.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for artifact in &inputs.generated_artifacts {
        for byte in artifact.artifact_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(artifact.artifact_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(artifact.drift_state.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(artifact.default_edit_posture.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("mga:{hash:016x}")
}

fn hook_available(
    hooks: &[MutationAndGeneratedArtifactInspectionHook],
    class: MutationAndGeneratedArtifactInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    mutation_path_coverage: &MutationPathCoverageSummary,
    generated_artifact_coverage: &GeneratedArtifactCoverageSummary,
    drift_truth: &DriftTruthSummary,
    edit_posture_honesty: &EditPostureHonestySummary,
    qualification: &MutationAndGeneratedArtifactLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Mutation/artifact lineage proven Stable: paths={paths} artifacts={artifacts} drifted={drifted} diverged={diverged}.",
            paths = mutation_path_coverage.mutation_path_rows.len(),
            artifacts = generated_artifact_coverage.generated_artifact_rows.len(),
            drifted = drift_truth.drifted_artifact_count,
            diverged = edit_posture_honesty.diverged_artifact_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Mutation/artifact lineage narrowed below Stable (paths={paths} artifacts={artifacts}): {reasons}.",
            paths = mutation_path_coverage.mutation_path_rows.len(),
            artifacts = generated_artifact_coverage.generated_artifact_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a mutation /
/// generated-artifact lineage record. The same projection is consumed
/// by the workspace mutation-lineage status surface, the headless CLI
/// emitter, Help/About, and support export.
pub fn mutation_and_generated_artifact_lineage_lines(
    record: &MutationAndGeneratedArtifactLineageRecord,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Mutation/artifact lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
    ));
    lines.push(format!(
        "mutation_path_coverage: paths={} required_present={}",
        record.mutation_path_coverage.mutation_path_rows.len(),
        record.mutation_path_coverage.all_required_paths_present,
    ));
    lines.push("Mutation paths:".to_owned());
    for row in &record.mutation_path_coverage.mutation_path_rows {
        lines.push(format!(
            "  - {kind} {id} journal={journal} no_rerun={no_rerun} privileged={privileged} canonical_privileged={canonical_privileged} privileged_matches={privileged_matches} required={required} support_export={posture}",
            kind = row.path_kind.as_str(),
            id = row.path_id,
            journal = row.journal_entry_id,
            no_rerun = row.no_rerun_posture.as_str(),
            privileged = row.touches_privileged_surface,
            canonical_privileged = row.canonical_touches_privileged_surface,
            privileged_matches = row.privileged_surface_matches,
            required = row.is_required,
            posture = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "generated_artifact_coverage: artifacts={} required_present={}",
        record
            .generated_artifact_coverage
            .generated_artifact_rows
            .len(),
        record
            .generated_artifact_coverage
            .all_required_artifact_classes_present,
    ));
    lines.push("Generated artifacts:".to_owned());
    for row in &record.generated_artifact_coverage.generated_artifact_rows {
        let surfaces: Vec<&str> = row
            .labeled_in_surfaces
            .iter()
            .map(|s| s.as_str())
            .collect();
        let missing: Vec<&str> = row
            .missing_labeling_surfaces
            .iter()
            .map(|s| s.as_str())
            .collect();
        lines.push(format!(
            "  - {kind} {id} canonical={canonical} generator={generator} digest={digest} drift={drift} posture={posture} round_trip_safe_class={round_trip_safe_class} required={required} support_export={support} labeled=[{labeled}] missing=[{missing}]",
            kind = row.artifact_kind.as_str(),
            id = row.artifact_id,
            canonical = row.canonical_source_ref,
            generator = row.generator_identity,
            digest = row.output_digest,
            drift = row.drift_state.as_str(),
            posture = row.default_edit_posture.as_str(),
            round_trip_safe_class = row.supports_round_trip_safe_editing,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
            labeled = surfaces.join(","),
            missing = missing.join(","),
        ));
    }
    lines.push(format!(
        "Canonical-lineage truth: canonical_refs={c} generator_identity={g} output_digest={d}",
        c = record
            .canonical_lineage_truth
            .all_artifacts_have_canonical_source_ref,
        g = record
            .canonical_lineage_truth
            .all_artifacts_have_generator_identity,
        d = record
            .canonical_lineage_truth
            .all_artifacts_have_output_digest,
    ));
    lines.push(format!(
        "Drift truth: drifted_artifacts={count} all_have_disclosure={disclosure}",
        count = record.drift_truth.drifted_artifact_count,
        disclosure = record.drift_truth.all_drifted_artifacts_have_disclosure,
    ));
    lines.push(format!(
        "Edit-posture honesty: round_trip_safe_supported={rts} diverged_artifacts={diverged} all_diverged_have_disclosures={dd}",
        rts = record
            .edit_posture_honesty
            .all_round_trip_safe_claims_supported,
        diverged = record.edit_posture_honesty.diverged_artifact_count,
        dd = record
            .edit_posture_honesty
            .all_diverged_artifacts_have_disclosures,
    ));
    lines.push(format!(
        "Labeling-surface coverage: all_labeled={}",
        record
            .labeling_surface_coverage
            .all_artifacts_labeled_on_required_surfaces,
    ));
    lines.push(format!(
        "Mutation no-rerun honesty: all_privileged_safe={safe} all_explicit_have_metadata={meta}",
        safe = record.mutation_no_rerun_honesty.all_privileged_paths_safe,
        meta = record
            .mutation_no_rerun_honesty
            .all_explicit_paths_have_metadata,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} redact_secrets={secrets} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority}",
        fields = record.support_export_honesty.all_rows_preserve_fields,
        secrets = record.support_export_honesty.all_rows_redact_raw_secrets,
        approvals = record
            .support_export_honesty
            .all_rows_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
