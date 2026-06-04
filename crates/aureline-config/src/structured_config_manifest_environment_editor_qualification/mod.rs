//! Stable qualification records for structured config, manifest, environment,
//! and live-state editors.
//!
//! A [`StructuredEditorQualification`] records whether an artifact-class surface
//! is showing canonical source, an effective/resolved projection, a planned
//! preview, or observed live state; whether round-trip editing can preserve
//! comments, unknown fields, ordering, and extension namespaces; how environment
//! and policy layers won; how secrets are represented; and which write posture
//! is allowed. The same record is rendered by UI, CLI/headless inspect, Help,
//! docs, and support export.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized qualification records.
pub const STRUCTURED_EDITOR_RECORD_KIND: &str =
    "structured_config_manifest_environment_editor_qualification";

/// Schema version for [`StructuredEditorQualification`].
pub const STRUCTURED_EDITOR_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref every consuming surface must quote.
pub const STRUCTURED_EDITOR_SHARED_CONTRACT_REF: &str =
    "config:structured_editor_source_effective_live:v1";

/// Reviewer-facing notice rendered on UI, CLI, Help, and support surfaces.
pub const STRUCTURED_EDITOR_NOTICE: &str =
    "Structured editor qualification: claimed stable config, manifest, lockfile, environment, \
     and live-state surfaces must distinguish canonical source, effective or rendered \
     projection, planned preview, and observed live state; expose target context and \
     environment or policy source chains; preserve comments, unknown keys, ordering, and \
     extension namespaces where the parser supports it; downgrade to raw, compare-only, \
     source-only, or high-risk review before mutation where preservation is not proven; keep \
     secret values as references or redacted placeholders in copy/export; and block wrong-target, \
     unresolved, deferred, stale-validation, and unsafe round-trip edits before save or apply.";

/// Public claim class for the qualified surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClass {
    /// Replacement-grade stable.
    Stable,
    /// Below stable but available to beta users.
    Beta,
    /// Inspectable preview only.
    Preview,
    /// No claim is published.
    NotClaimed,
}

impl ClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }
}

/// Artifact family covered by a qualification record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    /// Human-edited JSONC, YAML, TOML, or HCL config.
    StructuredConfig,
    /// Package, extension, workspace, or deployment manifest.
    Manifest,
    /// Lockfile or generated resolver output.
    Lockfile,
    /// `.env`-style or CI environment file.
    EnvironmentFile,
    /// Live or observed target-state projection.
    LiveStateProjection,
}

impl ArtifactClass {
    /// Returns the stable string vocabulary for this artifact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredConfig => "structured_config",
            Self::Manifest => "manifest",
            Self::Lockfile => "lockfile",
            Self::EnvironmentFile => "environment_file",
            Self::LiveStateProjection => "live_state_projection",
        }
    }
}

/// Truth layer currently represented by a view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthLayer {
    /// Canonical authored source.
    Source,
    /// Rendered or generated projection derived from source.
    RenderedProjection,
    /// Effective or resolved value after precedence, schema, and policy.
    EffectiveValue,
    /// Planned preview, dry-run, or validation result.
    PlannedPreview,
    /// Observed target state.
    LiveObserved,
    /// Provider-owned overlay.
    ProviderOverlay,
}

impl TruthLayer {
    /// Returns the stable string vocabulary for this truth layer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::RenderedProjection => "rendered_projection",
            Self::EffectiveValue => "effective_value",
            Self::PlannedPreview => "planned_preview",
            Self::LiveObserved => "live_observed",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Round-trip risk assigned to a parser/writer path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundTripRisk {
    /// Preservation has fixture proof.
    Preserving,
    /// Only declared canonicalization is allowed.
    DeclaredCanonicalization,
    /// Write requires compare-before-save or explicit high-risk review.
    HighRiskReviewRequired,
    /// Surface may inspect or compare but not write structurally.
    CompareOnly,
    /// Surface falls back to canonical raw source editing.
    RawSourceOnly,
}

impl RoundTripRisk {
    /// Returns `true` when the risk does not allow direct structured writes.
    pub const fn requires_downgrade_before_write(self) -> bool {
        matches!(
            self,
            Self::HighRiskReviewRequired | Self::CompareOnly | Self::RawSourceOnly
        )
    }

    /// Returns the stable string vocabulary for this risk class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserving => "preserving",
            Self::DeclaredCanonicalization => "declared_canonicalization",
            Self::HighRiskReviewRequired => "high_risk_review_required",
            Self::CompareOnly => "compare_only",
            Self::RawSourceOnly => "raw_source_only",
        }
    }
}

/// Mutation posture offered by the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritePosture {
    /// Structured save can mutate the canonical source.
    StructuredWriteAllowed,
    /// Compare-before-save is mandatory.
    CompareBeforeSaveRequired,
    /// User edits the canonical source only.
    SourceOnlyEdit,
    /// Inspect and compare only.
    InspectOnly,
    /// Mutation is blocked.
    Blocked,
}

/// Apply timing vocabulary shared by setup sheets and review surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyTiming {
    /// Applies immediately after save.
    Immediate,
    /// Staged until a later action.
    Staged,
    /// Preview or dry-run is required before apply.
    PreviewFirst,
    /// Policy controls the value.
    PolicyLocked,
}

/// Target context in which the value is interpreted or observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetContextClass {
    /// Local desktop or local workspace.
    Local,
    /// Remote SSH or remote workspace.
    Remote,
    /// Containerized target.
    Container,
    /// Managed/policy-owned target.
    ManagedPolicy,
    /// Browser companion or provider-side companion.
    BrowserCompanion,
}

/// Source class for a parameter or environment value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterSourceClass {
    /// Built-in or channel default.
    Default,
    /// Workspace-visible source file.
    WorkspaceValue,
    /// User or profile override.
    UserOverride,
    /// Runtime prompt value.
    RuntimePrompt,
    /// Secret handle or reference.
    SecretReference,
    /// Policy-supplied or policy-narrowed value.
    PolicyInjected,
    /// Runtime-discovered value.
    RuntimeDiscovered,
}

/// Storage/export mode for a sensitive or path-like value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueStorageMode {
    /// Literal value can be copied after review.
    Literal,
    /// Stable reference or handle only.
    ReferenceHandle,
    /// Redacted placeholder.
    RedactedPlaceholder,
    /// Key path only.
    KeyPathOnly,
    /// Local-only value that is not portable.
    LocalOnly,
}

/// Validation class used by the shared form model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationClass {
    /// Local parse or syntax validation.
    LocalSyntax,
    /// Schema validation.
    Schema,
    /// Environment probe.
    EnvironmentProbe,
    /// Remote or authentication validation.
    RemoteAuth,
    /// Policy validation.
    Policy,
    /// Dry-run validation.
    DryRun,
}

impl ValidationClass {
    /// All validation classes required by the shared form model.
    pub const REQUIRED: [Self; 6] = [
        Self::LocalSyntax,
        Self::Schema,
        Self::EnvironmentProbe,
        Self::RemoteAuth,
        Self::Policy,
        Self::DryRun,
    ];
}

/// Result status for a validation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    /// Validation passed.
    Passed,
    /// Validation is stale because inputs changed.
    StaleInvalidated,
    /// Validation timed out.
    TimedOut,
    /// Validation blocks mutation.
    Blocked,
    /// Validation is not applicable for this artifact.
    NotApplicable,
}

/// Copy/export posture for a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyExportMode {
    /// Literal value is allowed.
    LiteralValue,
    /// Stable reference or handle only.
    ReferenceHandle,
    /// Redacted placeholder only.
    RedactedPlaceholder,
    /// Key path only.
    KeyPathOnly,
}

/// Surface that consumes the shared record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// First-party editor UI.
    EditorUi,
    /// CLI or headless inspect command.
    CliHeadlessInspect,
    /// Help or docs view.
    HelpDocs,
    /// Support export packet.
    SupportExport,
    /// Template, importer, AI suggestion, or review route.
    RoutedActionReview,
}

impl SurfaceClass {
    /// Required consumers for shared vocabulary parity.
    pub const REQUIRED: [Self; 5] = [
        Self::EditorUi,
        Self::CliHeadlessInspect,
        Self::HelpDocs,
        Self::SupportExport,
        Self::RoutedActionReview,
    ];
}

/// Preservation proof for a parser/writer path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservationProof {
    /// Whether comments survive.
    pub preserves_comments: bool,
    /// Whether unknown keys survive.
    pub preserves_unknown_fields: bool,
    /// Whether authored ordering survives.
    pub preserves_ordering: bool,
    /// Whether extension namespaces survive.
    pub preserves_extension_namespaces: bool,
    /// Fixture refs that prove the classification.
    pub fixture_refs: Vec<String>,
    /// Downgrade or review route used when preservation is not proven.
    pub fallback_posture: WritePosture,
}

impl PreservationProof {
    fn fully_preserving(&self) -> bool {
        self.preserves_comments
            && self.preserves_unknown_fields
            && self.preserves_ordering
            && self.preserves_extension_namespaces
    }
}

/// One value source in the effective/environment chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterSourceRow {
    /// Stable key path.
    pub key_path: String,
    /// Source class.
    pub source_class: ParameterSourceClass,
    /// Source layer label.
    pub layer_label: String,
    /// Whether this layer won precedence.
    pub wins_effective_value: bool,
    /// Storage/export mode.
    pub storage_mode: ValueStorageMode,
    /// Redaction-safe display summary.
    pub visible_value_summary: String,
    /// Whether remove/reset is scoped to this layer.
    pub layer_specific_reset: bool,
}

/// Secret/reference chip shown next to a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretReferenceChip {
    /// Stable key path.
    pub key_path: String,
    /// Source class for the secret.
    pub source_class: ParameterSourceClass,
    /// Handle or reference label.
    pub handle_label: String,
    /// Copy/export behavior.
    pub copy_export_mode: CopyExportMode,
    /// Whether raw secret material is excluded by default.
    pub raw_secret_export_blocked_by_default: bool,
}

/// Shared validation result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationRow {
    /// Validation class.
    pub validation_class: ValidationClass,
    /// Current status.
    pub status: ValidationStatus,
    /// Exact blocker or result reason.
    pub reason: String,
    /// Whether a target change invalidates this result.
    pub invalidates_on_target_change: bool,
    /// Whether dirty state invalidates this result.
    pub invalidates_on_dirty_change: bool,
}

/// Parity row for a consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes the shared contract.
    pub consumes_shared_contract: bool,
    /// Vocabulary terms shown by this surface.
    pub vocabulary_terms: Vec<String>,
    /// Redaction-safe export posture.
    pub redaction_safe: bool,
    /// Whether preview/apply/revert lineage is exposed.
    pub exposes_preview_apply_revert_lineage: bool,
}

/// Row proving a risky drill blocks or downgrades before mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskDrillRow {
    /// Drill id.
    pub drill_id: String,
    /// Drill class.
    pub drill_class: String,
    /// Whether mutation was blocked before save/apply.
    pub blocks_before_mutation: bool,
    /// User-visible reason.
    pub visible_reason: String,
    /// Repair or fallback route.
    pub repair_route: String,
}

/// Qualification result derived from the evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Qualification {
    /// Public claim class.
    pub claim_class: ClaimClass,
    /// Whether the scenario qualifies as stable.
    pub qualifies_stable: bool,
    /// Derived narrowing reasons.
    pub narrowing_reasons: Vec<String>,
}

/// Input used to build a qualification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationInput {
    /// Stable record id.
    pub record_id: String,
    /// Artifact class.
    pub artifact_class: ArtifactClass,
    /// Primary truth layer.
    pub primary_truth_layer: TruthLayer,
    /// Additional inspectable truth layers.
    pub available_truth_layers: Vec<TruthLayer>,
    /// Target context class.
    pub target_context: TargetContextClass,
    /// Round-trip risk.
    pub round_trip_risk: RoundTripRisk,
    /// Write posture.
    pub write_posture: WritePosture,
    /// Apply timing.
    pub apply_timing: ApplyTiming,
    /// Preservation proof.
    pub preservation: PreservationProof,
    /// Parameter and environment source chain.
    pub parameter_sources: Vec<ParameterSourceRow>,
    /// Secret/reference chips.
    pub secret_chips: Vec<SecretReferenceChip>,
    /// Validation rows.
    pub validations: Vec<ValidationRow>,
    /// Surface parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Risk drill rows.
    pub risk_drills: Vec<RiskDrillRow>,
    /// Support export reference.
    pub support_export_ref: String,
    /// Documentation reference.
    pub docs_ref: String,
}

/// Canonical qualification record consumed by UI, CLI, docs, and support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredEditorQualification {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable record id.
    pub record_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Artifact class.
    pub artifact_class: ArtifactClass,
    /// Primary truth layer.
    pub primary_truth_layer: TruthLayer,
    /// Additional inspectable truth layers.
    pub available_truth_layers: Vec<TruthLayer>,
    /// Target context class.
    pub target_context: TargetContextClass,
    /// Round-trip risk.
    pub round_trip_risk: RoundTripRisk,
    /// Write posture.
    pub write_posture: WritePosture,
    /// Apply timing.
    pub apply_timing: ApplyTiming,
    /// Preservation proof.
    pub preservation: PreservationProof,
    /// Parameter and environment source chain.
    pub parameter_sources: Vec<ParameterSourceRow>,
    /// Secret/reference chips.
    pub secret_chips: Vec<SecretReferenceChip>,
    /// Validation rows.
    pub validations: Vec<ValidationRow>,
    /// Surface parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Risk drill rows.
    pub risk_drills: Vec<RiskDrillRow>,
    /// Support export reference.
    pub support_export_ref: String,
    /// Documentation reference.
    pub docs_ref: String,
    /// Derived qualification result.
    pub qualification: Qualification,
}

impl StructuredEditorQualification {
    /// Builds a qualification record and derives its public claim from evidence.
    pub fn build(input: QualificationInput) -> Self {
        let narrowing_reasons = derive_narrowing_reasons(&input);
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            ClaimClass::Stable
        } else if input.primary_truth_layer == TruthLayer::LiveObserved {
            ClaimClass::Preview
        } else {
            ClaimClass::Beta
        };

        Self {
            record_kind: STRUCTURED_EDITOR_RECORD_KIND.to_owned(),
            schema_version: STRUCTURED_EDITOR_SCHEMA_VERSION,
            record_id: input.record_id,
            shared_contract_ref: STRUCTURED_EDITOR_SHARED_CONTRACT_REF.to_owned(),
            notice: STRUCTURED_EDITOR_NOTICE.to_owned(),
            artifact_class: input.artifact_class,
            primary_truth_layer: input.primary_truth_layer,
            available_truth_layers: input.available_truth_layers,
            target_context: input.target_context,
            round_trip_risk: input.round_trip_risk,
            write_posture: input.write_posture,
            apply_timing: input.apply_timing,
            preservation: input.preservation,
            parameter_sources: input.parameter_sources,
            secret_chips: input.secret_chips,
            validations: input.validations,
            surface_parity: input.surface_parity,
            risk_drills: input.risk_drills,
            support_export_ref: input.support_export_ref,
            docs_ref: input.docs_ref,
            qualification: Qualification {
                claim_class,
                qualifies_stable,
                narrowing_reasons,
            },
        }
    }

    /// Returns redaction-safe support export lines.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("record_id: {}", self.record_id),
            format!("artifact_class: {}", self.artifact_class.as_str()),
            format!("truth_layer: {}", self.primary_truth_layer.as_str()),
            format!("target_context: {:?}", self.target_context),
            format!("round_trip_risk: {}", self.round_trip_risk.as_str()),
            format!("write_posture: {:?}", self.write_posture),
            format!("apply_timing: {:?}", self.apply_timing),
            format!("claim: {}", self.qualification.claim_class.as_str()),
            format!(
                "raw_secret_export: {}",
                if self
                    .secret_chips
                    .iter()
                    .all(|chip| chip.raw_secret_export_blocked_by_default)
                {
                    "blocked_by_default"
                } else {
                    "needs_review"
                }
            ),
        ]
    }
}

fn derive_narrowing_reasons(input: &QualificationInput) -> Vec<String> {
    let mut reasons = Vec::new();
    let available: BTreeSet<TruthLayer> = input.available_truth_layers.iter().copied().collect();
    for required in [
        TruthLayer::Source,
        TruthLayer::EffectiveValue,
        TruthLayer::PlannedPreview,
        TruthLayer::LiveObserved,
    ] {
        if !available.contains(&required) {
            reasons.push(format!("missing_truth_layer:{}", required.as_str()));
        }
    }

    if input.round_trip_risk.requires_downgrade_before_write()
        && input.write_posture == WritePosture::StructuredWriteAllowed
    {
        reasons.push("unsafe_round_trip_allows_structured_write".to_owned());
    }
    if input.round_trip_risk.requires_downgrade_before_write() {
        reasons.push(format!(
            "round_trip_downgraded:{}",
            input.round_trip_risk.as_str()
        ));
    }
    if !input.round_trip_risk.requires_downgrade_before_write()
        && !input.preservation.fully_preserving()
    {
        reasons.push("preserving_claim_without_full_preservation_proof".to_owned());
    }
    if input.parameter_sources.is_empty()
        || !input
            .parameter_sources
            .iter()
            .any(|row| row.wins_effective_value)
    {
        reasons.push("missing_effective_winner".to_owned());
    }
    if input
        .parameter_sources
        .iter()
        .any(|row| !row.layer_specific_reset)
    {
        reasons.push("reset_not_layer_specific".to_owned());
    }
    if input
        .secret_chips
        .iter()
        .any(|chip| !chip.raw_secret_export_blocked_by_default)
    {
        reasons.push("raw_secret_export_not_blocked".to_owned());
    }
    if missing_validation_classes(&input.validations) {
        reasons.push("shared_validation_model_incomplete".to_owned());
    }
    if input.validations.iter().any(|row| {
        matches!(
            row.status,
            ValidationStatus::StaleInvalidated | ValidationStatus::TimedOut
        ) && row.reason.is_empty()
    }) {
        reasons.push("stale_or_timeout_validation_without_exact_reason".to_owned());
    }
    if !surfaces_share_contract(&input.surface_parity) {
        reasons.push("surface_vocabulary_not_shared".to_owned());
    }
    if input
        .risk_drills
        .iter()
        .any(|row| !row.blocks_before_mutation)
    {
        reasons.push("risk_drill_does_not_block_before_mutation".to_owned());
    }
    if input.primary_truth_layer == TruthLayer::LiveObserved
        && input.write_posture != WritePosture::InspectOnly
    {
        reasons.push("live_state_projection_is_mutable".to_owned());
    }
    reasons.sort();
    reasons
}

fn missing_validation_classes(rows: &[ValidationRow]) -> bool {
    let present: BTreeSet<ValidationClass> = rows.iter().map(|row| row.validation_class).collect();
    ValidationClass::REQUIRED
        .iter()
        .any(|required| !present.contains(required))
}

fn surfaces_share_contract(rows: &[SurfaceParityRow]) -> bool {
    let present: BTreeSet<SurfaceClass> = rows.iter().map(|row| row.surface_class).collect();
    SurfaceClass::REQUIRED
        .iter()
        .all(|required| present.contains(required))
        && rows.iter().all(|row| {
            row.consumes_shared_contract
                && row.redaction_safe
                && row.exposes_preview_apply_revert_lineage
        })
}

/// One scenario in the deterministic qualification corpus.
#[derive(Debug, Clone)]
pub struct StructuredEditorScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// Fixture filename.
    pub fixture_filename: String,
    /// Expected claim class.
    pub expected_claim_class: ClaimClass,
    /// Expected stable verdict.
    pub expected_qualifies_stable: bool,
    record: StructuredEditorQualification,
}

impl StructuredEditorScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> StructuredEditorQualification {
        self.record.clone()
    }
}

/// Returns the deterministic structured-editor qualification corpus.
pub fn structured_editor_corpus() -> Vec<StructuredEditorScenario> {
    vec![
        scenario(
            "source_effective_live_preserving",
            ClaimClass::Stable,
            true,
            source_effective_live_preserving_input(),
        ),
        scenario(
            "manifest_compare_only_downgrade",
            ClaimClass::Beta,
            false,
            manifest_compare_only_input(),
        ),
        scenario(
            "wrong_target_blocks_apply",
            ClaimClass::Stable,
            true,
            wrong_target_blocks_apply_input(),
        ),
        scenario(
            "live_state_inspect_only",
            ClaimClass::Preview,
            false,
            live_state_inspect_only_input(),
        ),
        scenario(
            "unsafe_round_trip_overclaim",
            ClaimClass::Beta,
            false,
            unsafe_round_trip_overclaim_input(),
        ),
    ]
}

fn scenario(
    id: &'static str,
    expected_claim_class: ClaimClass,
    expected_qualifies_stable: bool,
    input: QualificationInput,
) -> StructuredEditorScenario {
    let record = StructuredEditorQualification::build(input);
    StructuredEditorScenario {
        scenario_id: id,
        fixture_filename: format!("{id}.json"),
        expected_claim_class,
        expected_qualifies_stable,
        record,
    }
}

fn base_input(record_id: &str, artifact_class: ArtifactClass) -> QualificationInput {
    QualificationInput {
        record_id: record_id.to_owned(),
        artifact_class,
        primary_truth_layer: TruthLayer::Source,
        available_truth_layers: vec![
            TruthLayer::Source,
            TruthLayer::EffectiveValue,
            TruthLayer::PlannedPreview,
            TruthLayer::LiveObserved,
        ],
        target_context: TargetContextClass::Local,
        round_trip_risk: RoundTripRisk::Preserving,
        write_posture: WritePosture::StructuredWriteAllowed,
        apply_timing: ApplyTiming::PreviewFirst,
        preservation: PreservationProof {
            preserves_comments: true,
            preserves_unknown_fields: true,
            preserves_ordering: true,
            preserves_extension_namespaces: true,
            fixture_refs: vec![
                "fixtures/config/m4/structured-config-manifest-environment-editor-qualification/source_effective_live_preserving.json".to_owned(),
            ],
            fallback_posture: WritePosture::StructuredWriteAllowed,
        },
        parameter_sources: parameter_sources(),
        secret_chips: secret_chips(),
        validations: validation_rows(),
        surface_parity: surface_parity_rows(true),
        risk_drills: risk_drills(true),
        support_export_ref: "artifacts/support/config/structured-editor-support-export".to_owned(),
        docs_ref: "docs/config/m4/structured-config-manifest-environment-editor-qualification.md".to_owned(),
    }
}

fn source_effective_live_preserving_input() -> QualificationInput {
    base_input(
        "config-qualification:source-effective-live-preserving",
        ArtifactClass::StructuredConfig,
    )
}

fn manifest_compare_only_input() -> QualificationInput {
    let mut input = base_input(
        "config-qualification:manifest-compare-only-downgrade",
        ArtifactClass::Manifest,
    );
    input.round_trip_risk = RoundTripRisk::CompareOnly;
    input.write_posture = WritePosture::InspectOnly;
    input.preservation.preserves_comments = false;
    input.preservation.fallback_posture = WritePosture::InspectOnly;
    input.preservation.fixture_refs = vec![
        "fixtures/config/m4/structured-config-manifest-environment-editor-qualification/manifest_compare_only_downgrade.json".to_owned(),
    ];
    input
}

fn wrong_target_blocks_apply_input() -> QualificationInput {
    let mut input = base_input(
        "config-qualification:wrong-target-blocks-apply",
        ArtifactClass::EnvironmentFile,
    );
    input.target_context = TargetContextClass::Container;
    input.write_posture = WritePosture::Blocked;
    input.apply_timing = ApplyTiming::PreviewFirst;
    input.risk_drills = risk_drills(true);
    input.validations.push(ValidationRow {
        validation_class: ValidationClass::RemoteAuth,
        status: ValidationStatus::Blocked,
        reason: "target changed from local desktop to container; preview token invalidated"
            .to_owned(),
        invalidates_on_target_change: true,
        invalidates_on_dirty_change: true,
    });
    input
}

fn live_state_inspect_only_input() -> QualificationInput {
    let mut input = base_input(
        "config-qualification:live-state-inspect-only",
        ArtifactClass::LiveStateProjection,
    );
    input.primary_truth_layer = TruthLayer::LiveObserved;
    input.write_posture = WritePosture::InspectOnly;
    input.round_trip_risk = RoundTripRisk::CompareOnly;
    input.preservation.preserves_comments = false;
    input.preservation.preserves_ordering = false;
    input.preservation.fallback_posture = WritePosture::InspectOnly;
    input
}

fn unsafe_round_trip_overclaim_input() -> QualificationInput {
    let mut input = base_input(
        "config-qualification:unsafe-round-trip-overclaim",
        ArtifactClass::Lockfile,
    );
    input.round_trip_risk = RoundTripRisk::HighRiskReviewRequired;
    input.write_posture = WritePosture::StructuredWriteAllowed;
    input.preservation.preserves_unknown_fields = false;
    input.surface_parity = surface_parity_rows(false);
    input.risk_drills = risk_drills(false);
    input
}

fn parameter_sources() -> Vec<ParameterSourceRow> {
    vec![
        ParameterSourceRow {
            key_path: "runtime.node.env.API_TOKEN".to_owned(),
            source_class: ParameterSourceClass::SecretReference,
            layer_label: "workspace env file".to_owned(),
            wins_effective_value: false,
            storage_mode: ValueStorageMode::ReferenceHandle,
            visible_value_summary: "secret ref: workspace/api-token".to_owned(),
            layer_specific_reset: true,
        },
        ParameterSourceRow {
            key_path: "runtime.node.env.API_TOKEN".to_owned(),
            source_class: ParameterSourceClass::PolicyInjected,
            layer_label: "managed policy".to_owned(),
            wins_effective_value: true,
            storage_mode: ValueStorageMode::ReferenceHandle,
            visible_value_summary: "policy-provided secret handle".to_owned(),
            layer_specific_reset: true,
        },
        ParameterSourceRow {
            key_path: "runtime.node.cwd".to_owned(),
            source_class: ParameterSourceClass::WorkspaceValue,
            layer_label: "workspace manifest".to_owned(),
            wins_effective_value: true,
            storage_mode: ValueStorageMode::Literal,
            visible_value_summary: "workspace-relative path".to_owned(),
            layer_specific_reset: true,
        },
    ]
}

fn secret_chips() -> Vec<SecretReferenceChip> {
    vec![SecretReferenceChip {
        key_path: "runtime.node.env.API_TOKEN".to_owned(),
        source_class: ParameterSourceClass::SecretReference,
        handle_label: "secret ref only".to_owned(),
        copy_export_mode: CopyExportMode::ReferenceHandle,
        raw_secret_export_blocked_by_default: true,
    }]
}

fn validation_rows() -> Vec<ValidationRow> {
    vec![
        ValidationRow {
            validation_class: ValidationClass::LocalSyntax,
            status: ValidationStatus::Passed,
            reason: "JSONC parse accepted with comments retained".to_owned(),
            invalidates_on_target_change: false,
            invalidates_on_dirty_change: true,
        },
        ValidationRow {
            validation_class: ValidationClass::Schema,
            status: ValidationStatus::Passed,
            reason: "stable schema loaded".to_owned(),
            invalidates_on_target_change: false,
            invalidates_on_dirty_change: true,
        },
        ValidationRow {
            validation_class: ValidationClass::EnvironmentProbe,
            status: ValidationStatus::Passed,
            reason: "local and target env probes agree".to_owned(),
            invalidates_on_target_change: true,
            invalidates_on_dirty_change: true,
        },
        ValidationRow {
            validation_class: ValidationClass::RemoteAuth,
            status: ValidationStatus::Passed,
            reason: "target auth fresh".to_owned(),
            invalidates_on_target_change: true,
            invalidates_on_dirty_change: false,
        },
        ValidationRow {
            validation_class: ValidationClass::Policy,
            status: ValidationStatus::Passed,
            reason: "policy source chain resolved".to_owned(),
            invalidates_on_target_change: true,
            invalidates_on_dirty_change: true,
        },
        ValidationRow {
            validation_class: ValidationClass::DryRun,
            status: ValidationStatus::Passed,
            reason: "preview computed; no mutation performed".to_owned(),
            invalidates_on_target_change: true,
            invalidates_on_dirty_change: true,
        },
    ]
}

fn surface_parity_rows(shared: bool) -> Vec<SurfaceParityRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface_class| SurfaceParityRow {
            surface_class,
            consumes_shared_contract: shared,
            vocabulary_terms: vec![
                "source".to_owned(),
                "effective_value".to_owned(),
                "planned_preview".to_owned(),
                "live_observed".to_owned(),
                "round_trip_risk".to_owned(),
            ],
            redaction_safe: true,
            exposes_preview_apply_revert_lineage: shared,
        })
        .collect()
}

fn risk_drills(blocks: bool) -> Vec<RiskDrillRow> {
    vec![
        RiskDrillRow {
            drill_id: "wrong-target".to_owned(),
            drill_class: "target_context_mismatch".to_owned(),
            blocks_before_mutation: blocks,
            visible_reason: "target changed; preview token invalidated before apply".to_owned(),
            repair_route: "switch target or regenerate preview".to_owned(),
        },
        RiskDrillRow {
            drill_id: "round-trip-unsafe".to_owned(),
            drill_class: "unknown_field_loss_risk".to_owned(),
            blocks_before_mutation: blocks,
            visible_reason:
                "unknown fields would be lost; source-only or compare-only route required"
                    .to_owned(),
            repair_route: "open raw source or compare-only review".to_owned(),
        },
    ]
}
