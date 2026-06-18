//! Cross-surface automation-label parity for claimed M5 commands.
//!
//! The automation contract baseline in [`crate::m5_automation_contract_baseline`]
//! froze the controlled safety-label vocabulary ([`AutomationSafetyLabelId`]) that
//! every automation surface reads. This module proves the harder property the
//! newer UX docs require: that the *same* labels, with the *same* stable ids and
//! the *same* canonical display tokens, are projected consistently wherever a
//! claimed M5 command is surfaced or exported — the command palette row, the
//! recipe builder, the macro recorder, docs/help, CLI/headless inspect, the
//! support/export packet, and the release/public-truth artifact.
//!
//! For each claimed command, [`CommandLabelParityRow`] holds one *source* label
//! set (the labels the command graph owns) and one [`SurfaceLabelProjection`] per
//! surface. [`LabelParityPacket::validate`] enforces the freeze mechanically:
//! every command projects to every surface, every surface projects the same
//! stable-id set as the source, no surface invents a synonym display token or
//! drops an effect-disclosure (side-effect) label, the stable ids survive
//! localization / export / downgrade, and the closed vocabulary stays covered.
//! A dropped surface, a drifted label set, a synonym token, a dropped side-effect
//! class, a lost stable id, or a violated invariant *blocks stable*.
//!
//! The reviewer-facing landing page is [`/docs/m5/automation-safety-labels.md`];
//! the cross-surface boundary schema is
//! [`/schemas/automation/automation-labels.schema.json`]; the reused vocabulary
//! axis is the controlled-automation-label set frozen in
//! [`/schemas/automation/automation-manifest.schema.json`].
//!
//! [`/docs/m5/automation-safety-labels.md`]: ../../../docs/m5/automation-safety-labels.md
//! [`/schemas/automation/automation-labels.schema.json`]: ../../../schemas/automation/automation-labels.schema.json
//! [`/schemas/automation/automation-manifest.schema.json`]: ../../../schemas/automation/automation-manifest.schema.json

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    canonical_safety_labels, AutomationBaselinePromotionState, AutomationSafetyLabel,
    AutomationSafetyLabelId, SafetyLabelKind, AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF,
    CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
};

/// Stable record-kind tag for [`LabelParityPacket`].
pub const LABEL_PARITY_RECORD_KIND: &str = "m5_automation_label_parity_packet";

/// Stable record-kind tag for [`LabelParitySupportExport`].
pub const LABEL_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_automation_label_parity_support_export";

/// Stable record-kind tag for [`LabelParityCliHeadlessView`].
pub const LABEL_PARITY_CLI_HEADLESS_RECORD_KIND: &str = "m5_automation_label_parity_cli_headless";

/// Integer schema version for the label-parity packet family.
pub const LABEL_PARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the cross-surface boundary schema.
pub const LABEL_PARITY_SCHEMA_REF: &str = "schemas/automation/automation-labels.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const LABEL_PARITY_DOC_REF: &str = "docs/m5/automation-safety-labels.md";

/// Repo-relative path of the checked-in label-parity packet artifact.
pub const LABEL_PARITY_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/label-parity/packet.json";

/// Repo-relative root the worked-example label-parity fixtures live under.
pub const LABEL_PARITY_FIXTURE_DIR: &str = "fixtures/automation/m5/label-parity";

/// Stable packet id minted by the seed.
pub const LABEL_PARITY_ID: &str = "automation:m5:label-parity:v1";

/// Stable support-export id minted by the seed inspector.
pub const LABEL_PARITY_SUPPORT_EXPORT_ID: &str = "support-export:automation:m5:label-parity";

/// Stable CLI/headless view id minted by the seed inspector.
pub const LABEL_PARITY_CLI_HEADLESS_ID: &str = "cli-headless:automation:m5:label-parity";

// ---------------------------------------------------------------------------
// Surfaces
// ---------------------------------------------------------------------------

/// One M5 surface that projects automation safety labels for a command.
///
/// The set is closed: every claimed command must project its label set to every
/// surface here, and a later surface that wants to show automation posture reuses
/// this projection instead of inventing surface-local synonyms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelSurfaceClass {
    /// The command palette / launcher row.
    CommandPaletteRow,
    /// The declarative recipe builder.
    RecipeBuilder,
    /// The macro recorder and replay surface.
    MacroRecorder,
    /// Docs and in-app help.
    DocsHelp,
    /// The CLI / headless `inspect` projection.
    CliHeadlessInspect,
    /// The redacted support / export packet.
    SupportExport,
    /// The release notes and public-truth artifact.
    ReleasePublicTruth,
}

impl LabelSurfaceClass {
    /// Every surface in canonical (declaration) order.
    pub const ALL: [LabelSurfaceClass; 7] = [
        LabelSurfaceClass::CommandPaletteRow,
        LabelSurfaceClass::RecipeBuilder,
        LabelSurfaceClass::MacroRecorder,
        LabelSurfaceClass::DocsHelp,
        LabelSurfaceClass::CliHeadlessInspect,
        LabelSurfaceClass::SupportExport,
        LabelSurfaceClass::ReleasePublicTruth,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            LabelSurfaceClass::CommandPaletteRow => "command_palette_row",
            LabelSurfaceClass::RecipeBuilder => "recipe_builder",
            LabelSurfaceClass::MacroRecorder => "macro_recorder",
            LabelSurfaceClass::DocsHelp => "docs_help",
            LabelSurfaceClass::CliHeadlessInspect => "cli_headless_inspect",
            LabelSurfaceClass::SupportExport => "support_export",
            LabelSurfaceClass::ReleasePublicTruth => "release_public_truth",
        }
    }

    /// Reviewable title.
    pub fn title(self) -> &'static str {
        match self {
            LabelSurfaceClass::CommandPaletteRow => "Command palette row",
            LabelSurfaceClass::RecipeBuilder => "Recipe builder",
            LabelSurfaceClass::MacroRecorder => "Macro recorder",
            LabelSurfaceClass::DocsHelp => "Docs and help",
            LabelSurfaceClass::CliHeadlessInspect => "CLI and headless inspect",
            LabelSurfaceClass::SupportExport => "Support export",
            LabelSurfaceClass::ReleasePublicTruth => "Release and public truth",
        }
    }
}

// ---------------------------------------------------------------------------
// Projected label
// ---------------------------------------------------------------------------

/// One safety label as a single surface renders it.
///
/// The label carries both its stable id token (the localization- and
/// export-stable identity) and its canonical display token (what the user reads).
/// Parity requires the stable id token to equal [`AutomationSafetyLabelId::as_str`]
/// and the display token to equal [`AutomationSafetyLabelId::display_token`]: a
/// surface that renames the display token has invented a synonym, and a surface
/// whose stable id token drifts has lost the cross-surface identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedLabel {
    /// The label this projection renders.
    pub label_id: AutomationSafetyLabelId,
    /// The localization- and export-stable id token.
    pub stable_id_token: String,
    /// The canonical, user-facing display token.
    pub display_token: String,
    /// Whether the label is an admissibility cue or an effect disclosure.
    pub label_kind: SafetyLabelKind,
}

impl ProjectedLabel {
    /// Materializes the canonical projection of one label.
    pub fn canonical(label_id: AutomationSafetyLabelId) -> Self {
        ProjectedLabel {
            label_id,
            stable_id_token: label_id.as_str().to_owned(),
            display_token: label_id.display_token().to_owned(),
            label_kind: label_id.kind(),
        }
    }

    /// Whether the stable id token matches the canonical id for the label.
    pub fn stable_id_matches(&self) -> bool {
        self.stable_id_token == self.label_id.as_str()
    }

    /// Whether the display token matches the canonical display for the label.
    pub fn display_token_matches(&self) -> bool {
        self.display_token == self.label_id.display_token()
    }
}

// ---------------------------------------------------------------------------
// Surface projection
// ---------------------------------------------------------------------------

/// How one surface projects a command's source label set.
///
/// The projection carries the ordered labels the surface renders plus the three
/// guarantees that the stable ids survive a localization swap, an export, and a
/// downgrade — the states in which a careless surface would otherwise drop the
/// stable id and leave only a translated string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceLabelProjection {
    /// The surface this projection describes.
    pub surface: LabelSurfaceClass,
    /// The labels the surface renders, in canonical order.
    pub projected_labels: Vec<ProjectedLabel>,
    /// The stable ids survive a localization swap of the display token.
    pub preserves_stable_ids_on_localization: bool,
    /// The stable ids survive export to a packet or bundle.
    pub preserves_stable_ids_on_export: bool,
    /// The stable ids survive a downgrade or claim-narrowing state.
    pub preserves_stable_ids_on_downgrade: bool,
}

impl SurfaceLabelProjection {
    /// Projects the source label set onto one surface canonically.
    ///
    /// The source labels are filtered into canonical order, every label projects
    /// from the frozen vocabulary, and every state-preservation guarantee holds.
    pub fn canonical(
        surface: LabelSurfaceClass,
        source_labels: &[AutomationSafetyLabelId],
    ) -> Self {
        let projected_labels = AutomationSafetyLabelId::ALL
            .into_iter()
            .filter(|label| source_labels.contains(label))
            .map(ProjectedLabel::canonical)
            .collect();
        SurfaceLabelProjection {
            surface,
            projected_labels,
            preserves_stable_ids_on_localization: true,
            preserves_stable_ids_on_export: true,
            preserves_stable_ids_on_downgrade: true,
        }
    }

    /// The stable id tokens the surface renders, in projection order.
    pub fn stable_id_tokens(&self) -> Vec<String> {
        self.projected_labels
            .iter()
            .map(|label| label.stable_id_token.clone())
            .collect()
    }

    /// Whether every state-preservation guarantee holds.
    pub fn preserves_stable_ids(&self) -> bool {
        self.preserves_stable_ids_on_localization
            && self.preserves_stable_ids_on_export
            && self.preserves_stable_ids_on_downgrade
    }
}

// ---------------------------------------------------------------------------
// Command row
// ---------------------------------------------------------------------------

/// One claimed M5 command and the labels it projects across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandLabelParityRow {
    /// Opaque, stable command id.
    pub command_id: String,
    /// Opaque command revision ref.
    pub command_revision_ref: String,
    /// Dotted snake_case canonical verb.
    pub canonical_verb: String,
    /// Reviewable command title.
    pub title: String,
    /// Ref to the command-graph field that owns this command's label set.
    pub source_label_ref: String,
    /// The one source label set the command graph owns, in canonical order.
    pub source_labels: Vec<AutomationSafetyLabelId>,
    /// One projection per surface.
    pub surface_projections: Vec<SurfaceLabelProjection>,
}

impl CommandLabelParityRow {
    /// Builds a row that projects one source label set to every surface.
    pub fn from_source(
        command_id: impl Into<String>,
        command_revision_ref: impl Into<String>,
        canonical_verb: impl Into<String>,
        title: impl Into<String>,
        source_labels: &[AutomationSafetyLabelId],
    ) -> Self {
        let canonical_labels: Vec<AutomationSafetyLabelId> = AutomationSafetyLabelId::ALL
            .into_iter()
            .filter(|label| source_labels.contains(label))
            .collect();
        let canonical_verb = canonical_verb.into();
        let surface_projections = LabelSurfaceClass::ALL
            .into_iter()
            .map(|surface| SurfaceLabelProjection::canonical(surface, &canonical_labels))
            .collect();
        CommandLabelParityRow {
            command_id: command_id.into(),
            command_revision_ref: command_revision_ref.into(),
            canonical_verb: canonical_verb.clone(),
            title: title.into(),
            source_label_ref: format!(
                "schemas/commands/command_descriptor.schema.json#/$defs/automation_labels:{canonical_verb}"
            ),
            source_labels: canonical_labels,
            surface_projections,
        }
    }

    /// The source stable id tokens in canonical order.
    pub fn source_stable_id_tokens(&self) -> Vec<String> {
        self.source_labels
            .iter()
            .map(|label| label.as_str().to_owned())
            .collect()
    }

    /// The projection for one surface, if present.
    pub fn projection(&self, surface: LabelSurfaceClass) -> Option<&SurfaceLabelProjection> {
        self.surface_projections
            .iter()
            .find(|projection| projection.surface == surface)
    }
}

// ---------------------------------------------------------------------------
// Invariants
// ---------------------------------------------------------------------------

/// Frozen invariants the label-parity packet pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelParityInvariantsBlock {
    /// Every surface projects from one label source, not a surface-local set.
    pub all_surfaces_project_from_one_label_source: bool,
    /// No surface renames a label into a surface-local synonym.
    pub no_surface_invents_synonyms: bool,
    /// Effect-disclosure (side-effect) labels are never dropped on a surface.
    pub effect_disclosure_labels_never_dropped: bool,
    /// Stable ids survive localization, export, and downgrade states.
    pub stable_ids_survive_localization_export_downgrade: bool,
    /// The safety-label vocabulary is closed and frozen.
    pub vocabulary_is_closed_and_frozen: bool,
    /// Every claimed command projects its labels to every surface.
    pub every_claimed_command_projects_to_every_surface: bool,
}

impl LabelParityInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        LabelParityInvariantsBlock {
            all_surfaces_project_from_one_label_source: true,
            no_surface_invents_synonyms: true,
            effect_disclosure_labels_never_dropped: true,
            stable_ids_survive_localization_export_downgrade: true,
            vocabulary_is_closed_and_frozen: true,
            every_claimed_command_projects_to_every_surface: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 6] {
        [
            (
                "all_surfaces_project_from_one_label_source",
                self.all_surfaces_project_from_one_label_source,
            ),
            (
                "no_surface_invents_synonyms",
                self.no_surface_invents_synonyms,
            ),
            (
                "effect_disclosure_labels_never_dropped",
                self.effect_disclosure_labels_never_dropped,
            ),
            (
                "stable_ids_survive_localization_export_downgrade",
                self.stable_ids_survive_localization_export_downgrade,
            ),
            (
                "vocabulary_is_closed_and_frozen",
                self.vocabulary_is_closed_and_frozen,
            ),
            (
                "every_claimed_command_projects_to_every_surface",
                self.every_claimed_command_projects_to_every_surface,
            ),
        ]
    }
}

// ---------------------------------------------------------------------------
// Findings
// ---------------------------------------------------------------------------

/// Severity of a label-parity validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityFindingSeverity {
    /// Blocks the packet from stable.
    Blocker,
    /// Narrows the packet below stable.
    Warning,
}

/// Kind of a label-parity validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityFindingKind {
    /// A command does not project its labels to a required surface.
    MissingSurfaceProjection,
    /// A surface's projected stable-id set differs from the command's source set.
    SurfaceLabelSetDrift,
    /// A surface renders a label with a non-canonical synonym display token.
    SynonymDisplayToken,
    /// A surface renders a label whose stable id token drifted from the canonical id.
    StableIdTokenDrift,
    /// A surface dropped an effect-disclosure (side-effect) label the source has.
    EffectDisclosureDropped,
    /// A surface does not preserve stable ids across localization, export, or downgrade.
    StableIdNotPreservedAcrossStates,
    /// A surface renders a label outside the frozen vocabulary.
    LabelOutsideVocabulary,
    /// The packet's vocabulary block does not cover the frozen label set.
    VocabularyCoverageIncomplete,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl LabelParityFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            LabelParityFindingKind::MissingSurfaceProjection => "missing_surface_projection",
            LabelParityFindingKind::SurfaceLabelSetDrift => "surface_label_set_drift",
            LabelParityFindingKind::SynonymDisplayToken => "synonym_display_token",
            LabelParityFindingKind::StableIdTokenDrift => "stable_id_token_drift",
            LabelParityFindingKind::EffectDisclosureDropped => "effect_disclosure_dropped",
            LabelParityFindingKind::StableIdNotPreservedAcrossStates => {
                "stable_id_not_preserved_across_states"
            }
            LabelParityFindingKind::LabelOutsideVocabulary => "label_outside_vocabulary",
            LabelParityFindingKind::VocabularyCoverageIncomplete => {
                "vocabulary_coverage_incomplete"
            }
            LabelParityFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by the parity gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelParityFinding {
    /// The finding kind.
    pub finding_kind: LabelParityFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: LabelParityFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl LabelParityFinding {
    fn blocker(
        finding_kind: LabelParityFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        LabelParityFinding {
            finding_kind,
            severity: LabelParityFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelParityInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// The closed safety-label vocabulary rows.
    pub vocabulary: Vec<AutomationSafetyLabel>,
    /// One row per claimed M5 command.
    pub command_rows: Vec<CommandLabelParityRow>,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: LabelParityInvariantsBlock,
}

/// Canonical M5 automation-label parity packet.
///
/// The packet binds every claimed command to a source label set and a projection
/// per surface, lists the closed vocabulary, and pins the freeze invariants.
/// [`LabelParityPacket::validate`] recomputes the findings so the fail-closed gate
/// and the typed consumer agree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelParityPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Boundary schema ref for this packet.
    pub schema_ref: String,
    /// Reused automation-contract-baseline schema ref.
    pub contract_baseline_schema_ref: String,
    /// Reused controlled-automation-label vocabulary axis ref.
    pub label_vocabulary_axis_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// The closed safety-label vocabulary rows.
    pub vocabulary: Vec<AutomationSafetyLabel>,
    /// One row per claimed M5 command.
    pub command_rows: Vec<CommandLabelParityRow>,
    /// Frozen invariants block.
    pub invariants: LabelParityInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<LabelParityFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over command and label tokens.
    pub packet_digest: String,
}

impl LabelParityPacket {
    /// Freezes an input into a packet, computing findings, promotion, and digest.
    pub fn materialize(input: LabelParityInput) -> Self {
        let findings = validate_parts(&input.vocabulary, &input.command_rows, &input.invariants);
        let promotion_state = promotion_state_for_findings(&findings);
        let packet_digest = packet_digest(&input.command_rows);
        LabelParityPacket {
            record_kind: LABEL_PARITY_RECORD_KIND.to_owned(),
            schema_version: LABEL_PARITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            schema_ref: LABEL_PARITY_SCHEMA_REF.to_owned(),
            contract_baseline_schema_ref: AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF.to_owned(),
            label_vocabulary_axis_ref: format!(
                "{CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF}#/$defs/controlled_automation_label"
            ),
            doc_ref: LABEL_PARITY_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            vocabulary: input.vocabulary,
            command_rows: input.command_rows,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            packet_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<LabelParityFinding> {
        validate_parts(&self.vocabulary, &self.command_rows, &self.invariants)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// The row for one canonical verb, if present.
    pub fn row(&self, canonical_verb: &str) -> Option<&CommandLabelParityRow> {
        self.command_rows
            .iter()
            .find(|row| row.canonical_verb == canonical_verb)
    }

    /// Canonical verbs in the order the packet stores them.
    pub fn command_verbs(&self) -> Vec<&str> {
        self.command_rows
            .iter()
            .map(|row| row.canonical_verb.as_str())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> LabelParitySupportExport {
        LabelParitySupportExport {
            record_kind: LABEL_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LABEL_PARITY_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            packet_digest: self.packet_digest.clone(),
            promotion_state: self.promotion_state,
            vocabulary_tokens: self
                .vocabulary
                .iter()
                .map(|label| label.label_id.as_str().to_owned())
                .collect(),
            command_rows: self
                .command_rows
                .iter()
                .map(|row| SupportCommandRow {
                    command_id: row.command_id.clone(),
                    canonical_verb: row.canonical_verb.clone(),
                    title: row.title.clone(),
                    source_label_tokens: row.source_stable_id_tokens(),
                    surface_count: row.surface_projections.len() as u32,
                })
                .collect(),
            invariants: self.invariants.clone(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind)
                .collect(),
        }
    }

    /// Builds the compact CLI / headless projection.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> LabelParityCliHeadlessView {
        LabelParityCliHeadlessView {
            record_kind: LABEL_PARITY_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: LABEL_PARITY_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            command_lines: self
                .command_rows
                .iter()
                .map(|row| {
                    format!(
                        "{} surfaces={} labels=[{}]",
                        row.canonical_verb,
                        row.surface_projections.len(),
                        row.source_labels
                            .iter()
                            .map(|label| label.display_token())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                })
                .collect(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} commands={} surfaces={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.command_rows.len(),
            LabelSurfaceClass::ALL.len(),
            self.packet_digest,
        )];
        for row in &self.command_rows {
            lines.push(format!(
                "command {} surfaces={} labels={}",
                row.canonical_verb,
                row.surface_projections.len(),
                row.source_labels
                    .iter()
                    .map(|label| label.as_str())
                    .collect::<Vec<_>>()
                    .join("|"),
            ));
        }
        lines
    }
}

/// One support-export command row (redacted projection).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCommandRow {
    /// Opaque command id.
    pub command_id: String,
    /// Dotted snake_case canonical verb.
    pub canonical_verb: String,
    /// Reviewable command title.
    pub title: String,
    /// Source stable-id label tokens, in canonical order.
    pub source_label_tokens: Vec<String>,
    /// Number of surfaces the command projects to.
    pub surface_count: u32,
}

/// Redacted support-export projection of the label-parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelParitySupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// Packet id this export was minted from.
    pub packet_id: String,
    /// Packet digest carried for verification.
    pub packet_digest: String,
    /// Promotion state of the source packet.
    pub promotion_state: AutomationBaselinePromotionState,
    /// The closed vocabulary stable-id tokens carried for support review.
    pub vocabulary_tokens: Vec<String>,
    /// Command rows.
    pub command_rows: Vec<SupportCommandRow>,
    /// Frozen invariants block.
    pub invariants: LabelParityInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<LabelParityFindingKind>,
}

impl LabelParitySupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.packet_digest.is_empty()
            && !self.command_rows.is_empty()
    }
}

/// Compact CLI / headless projection of the label-parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelParityCliHeadlessView {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Monotonic generation timestamp.
    pub generated_at: String,
    /// Packet id this view was minted from.
    pub packet_id: String,
    /// Promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// One line per command.
    pub command_lines: Vec<String>,
}

impl LabelParityCliHeadlessView {
    /// Whether the view explains every command in the packet.
    pub fn explains_command_count(&self, expected: usize) -> bool {
        self.command_lines.len() == expected
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_parts(
    vocabulary: &[AutomationSafetyLabel],
    command_rows: &[CommandLabelParityRow],
    invariants: &LabelParityInvariantsBlock,
) -> Vec<LabelParityFinding> {
    let mut findings = Vec::new();

    validate_vocabulary(vocabulary, &mut findings);

    let vocabulary_tokens: Vec<String> = AutomationSafetyLabelId::ALL
        .iter()
        .map(|label| label.as_str().to_owned())
        .collect();

    for row in command_rows {
        validate_command_row(row, &vocabulary_tokens, &mut findings);
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(LabelParityFinding::blocker(
                LabelParityFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn validate_vocabulary(
    vocabulary: &[AutomationSafetyLabel],
    findings: &mut Vec<LabelParityFinding>,
) {
    let canonical = canonical_safety_labels();
    if vocabulary != canonical.as_slice() {
        findings.push(LabelParityFinding::blocker(
            LabelParityFindingKind::VocabularyCoverageIncomplete,
            None,
            "the packet vocabulary does not match the frozen, ordered safety-label set",
        ));
    }
}

fn validate_command_row(
    row: &CommandLabelParityRow,
    vocabulary_tokens: &[String],
    findings: &mut Vec<LabelParityFinding>,
) {
    let verb = row.canonical_verb.as_str();
    let source_tokens = sorted(row.source_stable_id_tokens());
    let source_effect_tokens: Vec<String> = row
        .source_labels
        .iter()
        .filter(|label| label.kind() == SafetyLabelKind::EffectDisclosure)
        .map(|label| label.as_str().to_owned())
        .collect();

    for surface in LabelSurfaceClass::ALL {
        let Some(projection) = row.projection(surface) else {
            findings.push(LabelParityFinding::blocker(
                LabelParityFindingKind::MissingSurfaceProjection,
                Some(format!("{verb}:{}", surface.as_str())),
                format!(
                    "command {verb} does not project its labels to the {} surface",
                    surface.as_str()
                ),
            ));
            continue;
        };
        validate_projection(
            verb,
            projection,
            &source_tokens,
            &source_effect_tokens,
            vocabulary_tokens,
            findings,
        );
    }
}

fn validate_projection(
    verb: &str,
    projection: &SurfaceLabelProjection,
    source_tokens: &[String],
    source_effect_tokens: &[String],
    vocabulary_tokens: &[String],
    findings: &mut Vec<LabelParityFinding>,
) {
    let surface = projection.surface.as_str();
    let subject = format!("{verb}:{surface}");
    let projected_tokens = sorted(projection.stable_id_tokens());

    // A dropped effect-disclosure (side-effect) label is the most specific
    // failure; report it on its own so the "never omit the side-effect class"
    // guardrail is unambiguous, and skip the generic set-drift finding.
    let dropped_effect: Vec<&String> = source_effect_tokens
        .iter()
        .filter(|token| !projected_tokens.contains(*token))
        .collect();
    if !dropped_effect.is_empty() {
        findings.push(LabelParityFinding::blocker(
            LabelParityFindingKind::EffectDisclosureDropped,
            Some(subject.clone()),
            format!(
                "the {surface} surface for {verb} drops the effect-disclosure label(s) {}",
                dropped_effect
                    .iter()
                    .map(|token| token.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    } else if projected_tokens != source_tokens {
        findings.push(LabelParityFinding::blocker(
            LabelParityFindingKind::SurfaceLabelSetDrift,
            Some(subject.clone()),
            format!(
                "the {surface} surface for {verb} projects a label set that differs from the command source"
            ),
        ));
    }

    for label in &projection.projected_labels {
        if !vocabulary_tokens.contains(&label.stable_id_token) {
            findings.push(LabelParityFinding::blocker(
                LabelParityFindingKind::LabelOutsideVocabulary,
                Some(subject.clone()),
                format!(
                    "the {surface} surface for {verb} renders {} which is outside the frozen vocabulary",
                    label.stable_id_token
                ),
            ));
            continue;
        }
        if !label.stable_id_matches() {
            findings.push(LabelParityFinding::blocker(
                LabelParityFindingKind::StableIdTokenDrift,
                Some(subject.clone()),
                format!(
                    "the {surface} surface for {verb} renders {} with a drifted stable id token {}",
                    label.label_id.as_str(),
                    label.stable_id_token
                ),
            ));
        }
        if !label.display_token_matches() {
            findings.push(LabelParityFinding::blocker(
                LabelParityFindingKind::SynonymDisplayToken,
                Some(subject.clone()),
                format!(
                    "the {surface} surface for {verb} renders {} with the synonym display token {:?}",
                    label.label_id.as_str(),
                    label.display_token
                ),
            ));
        }
    }

    if !projection.preserves_stable_ids() {
        findings.push(LabelParityFinding::blocker(
            LabelParityFindingKind::StableIdNotPreservedAcrossStates,
            Some(subject),
            format!(
                "the {surface} surface for {verb} does not preserve stable ids across localization, export, or downgrade"
            ),
        ));
    }
}

fn promotion_state_for_findings(
    findings: &[LabelParityFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == LabelParityFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == LabelParityFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn sorted(mut tokens: Vec<String>) -> Vec<String> {
    tokens.sort_unstable();
    tokens
}

fn packet_digest(command_rows: &[CommandLabelParityRow]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    for row in command_rows {
        tokens.push(row.canonical_verb.clone());
        for label in &row.source_labels {
            tokens.push(format!("{}:{}", row.canonical_verb, label.as_str()));
        }
    }
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[String]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

// ---------------------------------------------------------------------------
// Seeds
// ---------------------------------------------------------------------------

/// Existing contracts the label-parity packet reuses instead of re-deciding.
pub fn canonical_reused_contract_refs() -> Vec<String> {
    [
        "schemas/automation/automation-contract-baseline.schema.json",
        "schemas/automation/automation-manifest.schema.json",
        "schemas/automation/recipe-builder-first-consumers.schema.json",
        "schemas/automation/macro-recorder.schema.json",
        "schemas/automation/run-history.schema.json",
        "schemas/commands/command_descriptor.schema.json",
        "docs/m5/recipe-builder-and-macro-contract.md",
        "docs/automation/recipe_and_macro_contract.md",
    ]
    .iter()
    .map(|value| (*value).to_owned())
    .collect()
}

/// The claimed M5 commands the parity packet binds, with their source labels.
///
/// The set is chosen so the union of every command's labels covers the full
/// frozen vocabulary, including a UI-only command (whose label still projects to
/// every surface, where it reads as not admissible) and a remote-mutating command.
pub fn seeded_command_rows() -> Vec<CommandLabelParityRow> {
    use AutomationSafetyLabelId::{
        ApprovalRequired, HeadlessSafe, MacroSafe, NetworkCall, RecipeSafe, RemoteMutation,
        RunsProcess, UiOnly, WritesFiles,
    };
    vec![
        CommandLabelParityRow::from_source(
            "command:notebook.run_all_cells",
            "command-rev:notebook.run_all_cells:4",
            "notebook.run_all_cells",
            "Run all notebook cells",
            &[RecipeSafe, HeadlessSafe, RunsProcess, WritesFiles],
        ),
        CommandLabelParityRow::from_source(
            "command:task.run_tests",
            "command-rev:task.run_tests:9",
            "task.run_tests",
            "Run the test task",
            &[MacroSafe, RecipeSafe, HeadlessSafe, RunsProcess],
        ),
        CommandLabelParityRow::from_source(
            "command:request.send_saved",
            "command-rev:request.send_saved:6",
            "request.send_saved",
            "Send the saved request",
            &[RecipeSafe, HeadlessSafe, NetworkCall],
        ),
        CommandLabelParityRow::from_source(
            "command:package.apply_safe_updates",
            "command-rev:package.apply_safe_updates:4",
            "package.apply_safe_updates",
            "Apply safe dependency updates",
            &[RecipeSafe, ApprovalRequired, NetworkCall, WritesFiles],
        ),
        CommandLabelParityRow::from_source(
            "command:remote.deploy_release",
            "command-rev:remote.deploy_release:2",
            "remote.deploy_release",
            "Deploy the release to the remote target",
            &[RecipeSafe, ApprovalRequired, NetworkCall, RemoteMutation],
        ),
        CommandLabelParityRow::from_source(
            "command:editor.toggle_minimap",
            "command-rev:editor.toggle_minimap:1",
            "editor.toggle_minimap",
            "Toggle the editor minimap",
            &[UiOnly],
        ),
    ]
}

/// Builds the canonical stable label-parity input.
pub fn current_label_parity_input() -> LabelParityInput {
    LabelParityInput {
        packet_id: LABEL_PARITY_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        vocabulary: canonical_safety_labels(),
        command_rows: seeded_command_rows(),
        reused_contract_refs: canonical_reused_contract_refs(),
        invariants: LabelParityInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable label-parity packet.
pub fn seeded_label_parity_packet() -> LabelParityPacket {
    LabelParityPacket::materialize(current_label_parity_input())
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_label_parity_packet(
    packet: &LabelParityPacket,
) -> Result<(), Vec<LabelParityFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}
