//! Typed parameter-review sheets and their first consumers.
//!
//! The automation contract baseline in
//! [`crate::m5_automation_contract_baseline`] froze *what* a pre-apply
//! parameter-review sheet is ([`ParameterReviewSheet`]) and the reused
//! argument-inspection, verdict, and redaction vocabularies every surface reads.
//! This module makes the sheet concrete: a live, mutable
//! [`ParameterReviewBuilder`] that reviews each runtime input as a **typed,
//! provenance-bearing field** — it carries the field type, the source layer the
//! value came from, whether the active value is a default or an override, the
//! save-to-scope choice, typed validation, and, for secret-bearing values, an
//! opaque secret reference instead of a raw value.
//!
//! The builder never holds a raw secret or an ambiguous form control:
//! [`ParameterReviewBuilder::to_sheet_record`] projects the live builder back
//! onto the frozen [`ParameterReviewSheet`] so every consumer reads the same
//! verdict truth, while the live [`ReviewedParameter`] keeps the extra
//! dimensions (field type, source layer, default/override state, save scope, and
//! secret reference) that make the input reviewable before run or share. A
//! secret value becomes a [`SecretReference`] (a broker handle plus a redaction
//! class); it is never stored as a literal, so it cannot leak into a recipe file
//! or a default run-history record.
//!
//! [`ParameterReviewFirstConsumersPacket`] binds the first M5 automation
//! families that gather runtime input — notebook, task/test/debug, request/API,
//! package, incident, and the AI assistant — each to a seeded sheet, and
//! [`ParameterReviewFirstConsumersPacket::validate`] enforces the freeze
//! mechanically: every entrypoint binds a non-empty sheet, every parameter is
//! typed with an explicit source layer, every secret value is a reference, every
//! save scope is in its allowed set, and the frozen projection stays consistent
//! with the live parameters. A dropped entrypoint, an untyped or
//! ambiguous-source parameter, a raw secret, a disallowed save scope, an
//! inconsistent projection, or a violated invariant *blocks stable*.
//!
//! The reviewer-facing landing page is
//! [`/docs/m5/parameter-review-and-secret-references.md`]; the cross-tool
//! boundary schema is [`/schemas/automation/parameter-review.schema.json`]; the
//! reused frozen-sheet schema is
//! [`/schemas/automation/recipe-builder.schema.json`].
//!
//! [`/docs/m5/parameter-review-and-secret-references.md`]: ../../../docs/m5/parameter-review-and-secret-references.md
//! [`/schemas/automation/parameter-review.schema.json`]: ../../../schemas/automation/parameter-review.schema.json
//! [`/schemas/automation/recipe-builder.schema.json`]: ../../../schemas/automation/recipe-builder.schema.json

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    ArgumentInspectionKind, AutomationBaselinePromotionState, ParameterReviewRow,
    ParameterReviewSheet, ParameterReviewVerdictClass, RECIPE_BUILDER_SCHEMA_REF,
};
use crate::recipe_builder::RecipeBuilderEntrypoint;

/// Stable record-kind tag for [`ParameterReviewFirstConsumersPacket`].
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_RECORD_KIND: &str =
    "m5_parameter_review_first_consumers_packet";

/// Stable record-kind tag for [`ParameterReviewFirstConsumersSupportExport`].
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_parameter_review_first_consumers_support_export";

/// Stable record-kind tag for [`ParameterReviewFirstConsumersCliHeadlessView`].
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_parameter_review_first_consumers_cli_headless";

/// Stable record-kind tag for [`ParameterReviewExport`].
pub const PARAMETER_REVIEW_EXPORT_RECORD_KIND: &str = "parameter_review_export_record";

/// Stable record-kind tag the builder mints for the frozen sheet projection.
///
/// Identical to the record kind frozen in the automation contract baseline, so
/// the projection is the same `parameter_review_sheet_record` every surface reads.
pub const PARAMETER_REVIEW_SHEET_RECORD_KIND: &str = "parameter_review_sheet_record";

/// Integer schema version for the parameter-review first-consumers family.
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the first-consumers boundary schema.
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/automation/parameter-review.schema.json";

/// Repo-relative path of the frozen parameter-review-sheet boundary schema.
pub const PARAMETER_REVIEW_SHEET_SCHEMA_REF: &str = RECIPE_BUILDER_SCHEMA_REF;

/// Repo-relative path of the reviewer contract doc for the parameter-review lane.
pub const PARAMETER_REVIEW_DOC_REF: &str = "docs/m5/parameter-review-and-secret-references.md";

/// Repo-relative path of the checked-in first-consumers packet artifact.
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/parameter-review/packet.json";

/// Repo-relative root the worked-example parameter-review fixtures live under.
pub const PARAMETER_REVIEW_FIXTURE_DIR: &str = "fixtures/automation/m5/parameter-review";

/// Stable packet id minted by the seed.
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_ID: &str =
    "automation:m5:parameter-review-first-consumers:v1";

/// Stable support-export id minted by the seed inspector.
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:parameter-review-first-consumers";

/// Stable CLI/headless view id minted by the seed inspector.
pub const PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:parameter-review-first-consumers";

// ---------------------------------------------------------------------------
// Typed field
// ---------------------------------------------------------------------------

/// The declared type of a reviewed parameter.
///
/// Every reviewable input is typed; a generic untyped form control is not
/// admissible. A [`ParameterFieldType::SecretReference`] field must carry a
/// [`SecretReference`] rather than a literal value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterFieldType {
    /// A free-text scalar value.
    Text,
    /// A signed integer value.
    Integer,
    /// A boolean toggle.
    Boolean,
    /// A choice from a closed enumeration.
    Enumeration,
    /// A workspace-relative path reference (never a raw absolute path).
    PathReference,
    /// A URL reference resolved against an environment profile.
    UrlReference,
    /// A secret-bearing value held as an opaque broker reference.
    SecretReference,
    /// A duration in milliseconds.
    DurationMs,
    /// A reference to a saved environment profile.
    EnvironmentProfileRef,
}

impl ParameterFieldType {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ParameterFieldType::Text => "text",
            ParameterFieldType::Integer => "integer",
            ParameterFieldType::Boolean => "boolean",
            ParameterFieldType::Enumeration => "enumeration",
            ParameterFieldType::PathReference => "path_reference",
            ParameterFieldType::UrlReference => "url_reference",
            ParameterFieldType::SecretReference => "secret_reference",
            ParameterFieldType::DurationMs => "duration_ms",
            ParameterFieldType::EnvironmentProfileRef => "environment_profile_ref",
        }
    }

    /// Whether this field type carries a secret-bearing value.
    pub fn is_secret(self) -> bool {
        matches!(self, ParameterFieldType::SecretReference)
    }
}

// ---------------------------------------------------------------------------
// Source layer
// ---------------------------------------------------------------------------

/// Where a reviewed parameter's value came from.
///
/// The source layer is the parameter's provenance: it makes "where did this
/// value come from" explicit instead of leaving it to a generic form control.
/// Every layer except [`ParameterSourceLayer::UnspecifiedGenericControl`] maps
/// to a frozen [`ArgumentInspectionKind`], which is how provenance reuse stays
/// mechanically checkable. The unspecified variant is the inadmissible state the
/// gate refuses: an ambiguous value hiding in a generic control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterSourceLayer {
    /// The value is the descriptor's declared default.
    DescriptorDefault,
    /// The value is remembered at workspace scope.
    WorkspaceSaved,
    /// The value is remembered at user scope.
    UserSaved,
    /// The value is supplied by the recipe manifest.
    RecipeSupplied,
    /// The value is bound to the active selection.
    SelectionBacked,
    /// The value is bound to the focused context (active file or target).
    FocusedContextBacked,
    /// The value is proposed by the AI assistant.
    AiProposed,
    /// The value is pinned by admin policy and cannot be overridden.
    PolicyPinned,
    /// The value is resolved through an opaque secret-broker handle.
    SecretBroker,
    /// The value's origin is unspecified — an ambiguous generic control.
    UnspecifiedGenericControl,
}

impl ParameterSourceLayer {
    /// Every source layer in canonical order.
    pub const ALL: [ParameterSourceLayer; 10] = [
        ParameterSourceLayer::DescriptorDefault,
        ParameterSourceLayer::WorkspaceSaved,
        ParameterSourceLayer::UserSaved,
        ParameterSourceLayer::RecipeSupplied,
        ParameterSourceLayer::SelectionBacked,
        ParameterSourceLayer::FocusedContextBacked,
        ParameterSourceLayer::AiProposed,
        ParameterSourceLayer::PolicyPinned,
        ParameterSourceLayer::SecretBroker,
        ParameterSourceLayer::UnspecifiedGenericControl,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ParameterSourceLayer::DescriptorDefault => "descriptor_default",
            ParameterSourceLayer::WorkspaceSaved => "workspace_saved",
            ParameterSourceLayer::UserSaved => "user_saved",
            ParameterSourceLayer::RecipeSupplied => "recipe_supplied",
            ParameterSourceLayer::SelectionBacked => "selection_backed",
            ParameterSourceLayer::FocusedContextBacked => "focused_context_backed",
            ParameterSourceLayer::AiProposed => "ai_proposed",
            ParameterSourceLayer::PolicyPinned => "policy_pinned",
            ParameterSourceLayer::SecretBroker => "secret_broker",
            ParameterSourceLayer::UnspecifiedGenericControl => "unspecified_generic_control",
        }
    }

    /// The frozen argument-inspection kind this layer maps to, if any.
    ///
    /// Returns `None` only for [`ParameterSourceLayer::UnspecifiedGenericControl`],
    /// which has no admissible provenance and is rejected by the gate.
    pub fn explicit_inspection_kind(self) -> Option<ArgumentInspectionKind> {
        Some(match self {
            ParameterSourceLayer::DescriptorDefault => {
                ArgumentInspectionKind::DefaultFromDescriptorArgumentRef
            }
            ParameterSourceLayer::WorkspaceSaved | ParameterSourceLayer::UserSaved => {
                ArgumentInspectionKind::TypedArgumentSlotRef
            }
            ParameterSourceLayer::RecipeSupplied => {
                ArgumentInspectionKind::AutomationRecipeSuppliedArgumentRef
            }
            ParameterSourceLayer::SelectionBacked => {
                ArgumentInspectionKind::SelectionBackedArgumentRef
            }
            ParameterSourceLayer::FocusedContextBacked => {
                ArgumentInspectionKind::FocusedContextBackedArgumentRef
            }
            ParameterSourceLayer::AiProposed => ArgumentInspectionKind::AiProposedArgumentRef,
            ParameterSourceLayer::PolicyPinned => ArgumentInspectionKind::PolicyPinnedArgumentRef,
            ParameterSourceLayer::SecretBroker => {
                ArgumentInspectionKind::CredentialHandleArgumentRef
            }
            ParameterSourceLayer::UnspecifiedGenericControl => return None,
        })
    }

    /// The inspection kind used for the frozen-sheet projection.
    ///
    /// Falls back to [`ArgumentInspectionKind::TypedArgumentSlotRef`] for the
    /// unspecified layer so the projection still type-checks; validation rejects
    /// the ambiguity separately via [`Self::explicit_inspection_kind`].
    pub fn inspection_kind(self) -> ArgumentInspectionKind {
        self.explicit_inspection_kind()
            .unwrap_or(ArgumentInspectionKind::TypedArgumentSlotRef)
    }
}

// ---------------------------------------------------------------------------
// Default / override state
// ---------------------------------------------------------------------------

/// Whether the parameter's active value is a default or an override.
///
/// This keeps default-versus-override state visible: a reviewer can always tell
/// whether the value in play is the unchanged default, an override the user
/// applied, a value still awaiting input, or a value pinned by policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterValueState {
    /// The active value is the unchanged default from the source layer.
    DefaultValue,
    /// The user overrode the default for this run.
    Overridden,
    /// A required value has not been provided yet.
    AwaitingInput,
    /// The value is fixed by admin policy and cannot be edited.
    PolicyPinned,
}

impl ParameterValueState {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ParameterValueState::DefaultValue => "default_value",
            ParameterValueState::Overridden => "overridden",
            ParameterValueState::AwaitingInput => "awaiting_input",
            ParameterValueState::PolicyPinned => "policy_pinned",
        }
    }

    /// Whether the active value is an override of the default.
    pub fn is_override(self) -> bool {
        matches!(self, ParameterValueState::Overridden)
    }
}

// ---------------------------------------------------------------------------
// Save-to-scope
// ---------------------------------------------------------------------------

/// The scope a remembered value would affect.
///
/// Save-to-scope is always explicit and portable: a reviewer sees, per
/// parameter, whether a value is used once or remembered for the workspace, the
/// user, or pinned by organization policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveToScope {
    /// Used for this run only; not remembered.
    RunOnly,
    /// Remembered for this workspace.
    Workspace,
    /// Remembered for this user across workspaces.
    User,
    /// Pinned by organization policy; read-only, cannot be saved into.
    OrganizationPolicy,
}

impl SaveToScope {
    /// Every save scope in canonical order.
    pub const ALL: [SaveToScope; 4] = [
        SaveToScope::RunOnly,
        SaveToScope::Workspace,
        SaveToScope::User,
        SaveToScope::OrganizationPolicy,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            SaveToScope::RunOnly => "run_only",
            SaveToScope::Workspace => "workspace",
            SaveToScope::User => "user",
            SaveToScope::OrganizationPolicy => "organization_policy",
        }
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// The typed constraint applied to a reviewed parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterConstraintKind {
    /// No constraint beyond the field type.
    None,
    /// The value must be non-empty.
    NonEmpty,
    /// An integer within an inclusive range.
    IntegerRange,
    /// A member of a closed enumeration.
    EnumMembership,
    /// A workspace-relative path.
    WorkspaceRelativePath,
    /// A URL with an allowed scheme.
    UrlScheme,
    /// A present, resolvable secret-broker handle.
    SecretBrokerHandlePresent,
    /// A resolvable environment-profile reference.
    EnvironmentProfileResolvable,
}

impl ParameterConstraintKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ParameterConstraintKind::None => "none",
            ParameterConstraintKind::NonEmpty => "non_empty",
            ParameterConstraintKind::IntegerRange => "integer_range",
            ParameterConstraintKind::EnumMembership => "enum_membership",
            ParameterConstraintKind::WorkspaceRelativePath => "workspace_relative_path",
            ParameterConstraintKind::UrlScheme => "url_scheme",
            ParameterConstraintKind::SecretBrokerHandlePresent => "secret_broker_handle_present",
            ParameterConstraintKind::EnvironmentProfileResolvable => {
                "environment_profile_resolvable"
            }
        }
    }
}

/// The typed validation applied to one reviewed parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterValidation {
    /// The typed constraint applied.
    pub constraint_kind: ParameterConstraintKind,
    /// Whether the constraint is currently satisfied.
    pub satisfied: bool,
    /// Reviewable summary of the constraint (never the raw value).
    pub constraint_summary: String,
}

impl ParameterValidation {
    /// A satisfied validation with the given constraint and summary.
    pub fn satisfied(
        constraint_kind: ParameterConstraintKind,
        constraint_summary: impl Into<String>,
    ) -> Self {
        ParameterValidation {
            constraint_kind,
            satisfied: true,
            constraint_summary: constraint_summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Secret reference
// ---------------------------------------------------------------------------

/// An opaque reference to a secret-bearing value.
///
/// A secret value never appears as a literal. It is held as a broker handle plus
/// the redaction class that governs the handle, so the value can be reviewed,
/// rerun, exported, and supported without the secret leaking into a recipe file
/// or a default run-history record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretReference {
    /// Opaque secret-broker handle; never the secret itself.
    pub broker_handle_ref: String,
    /// Re-exported redaction class governing the handle.
    pub redaction_class: String,
}

/// Redaction classes that admit a secret-bearing reference.
const SECRET_BEARING_REDACTION_CLASSES: [&str; 3] = [
    "redaction_required_with_secret_broker_handles",
    "signing_evidence_only",
    "operator_only_restricted",
];

// ---------------------------------------------------------------------------
// Reviewed parameter
// ---------------------------------------------------------------------------

/// One typed, provenance-bearing reviewed parameter.
///
/// A reviewed parameter is the unit of the parameter-review sheet: it carries its
/// field type, the source layer it came from, whether the value is a default or
/// an override, the save-to-scope choice and its allowed set, typed validation,
/// and — for secret-bearing fields — an opaque [`SecretReference`] instead of a
/// raw value. The review verdict is derived, never asserted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewedParameter {
    /// Snake_case parameter name.
    pub parameter_name: String,
    /// The declared field type.
    pub field_type: ParameterFieldType,
    /// The source layer the value came from.
    pub source_layer: ParameterSourceLayer,
    /// Whether the active value is a default or an override.
    pub value_state: ParameterValueState,
    /// Whether the parameter is required before apply.
    pub required: bool,
    /// Re-exported redaction class governing the value's sensitivity.
    pub sensitivity_class: String,
    /// Opaque secret reference for a secret-bearing field, or `null`.
    pub secret_reference: Option<SecretReference>,
    /// The save scope chosen for this parameter.
    pub chosen_save_scope: SaveToScope,
    /// The save scopes this parameter may be saved to.
    pub available_save_scopes: Vec<SaveToScope>,
    /// The typed validation applied.
    pub validation: ParameterValidation,
    /// Reviewable summary sentence (never the raw value).
    pub summary: String,
}

impl ReviewedParameter {
    /// The derived review verdict for this parameter.
    ///
    /// A failed constraint drives `blocked`; a value pinned by policy drives
    /// `policy_pinned`; a value still awaiting input drives `needs_input`; a
    /// secret reference held behind a broker handle drives
    /// `sensitive_held_for_review`; otherwise the value is `resolved`.
    pub fn verdict_class(&self) -> ParameterReviewVerdictClass {
        if !self.validation.satisfied {
            return ParameterReviewVerdictClass::Blocked;
        }
        match self.value_state {
            ParameterValueState::PolicyPinned => ParameterReviewVerdictClass::PolicyPinned,
            ParameterValueState::AwaitingInput => ParameterReviewVerdictClass::NeedsInput,
            ParameterValueState::DefaultValue | ParameterValueState::Overridden => {
                if self.secret_reference.is_some() {
                    ParameterReviewVerdictClass::SensitiveHeldForReview
                } else {
                    ParameterReviewVerdictClass::Resolved
                }
            }
        }
    }

    /// Projects this parameter onto the frozen [`ParameterReviewRow`].
    pub fn to_review_row(&self) -> ParameterReviewRow {
        ParameterReviewRow {
            parameter_name: self.parameter_name.clone(),
            inspection_kind: self.source_layer.inspection_kind(),
            verdict_class: self.verdict_class(),
            required: self.required,
            sensitivity_class: self.sensitivity_class.clone(),
            summary: self.summary.clone(),
        }
    }

    /// Whether the parameter is a required value still awaiting input.
    pub fn is_unresolved_required(&self) -> bool {
        self.required && self.verdict_class() == ParameterReviewVerdictClass::NeedsInput
    }

    /// Whether the chosen save scope is in the allowed set.
    pub fn save_scope_allowed(&self) -> bool {
        self.available_save_scopes.contains(&self.chosen_save_scope)
    }

    /// Whether the secret-reference posture is consistent.
    ///
    /// A secret-bearing field must hold a reference (or be awaiting input) and
    /// carry a secret-bearing redaction class; a non-secret field must not smuggle
    /// a broker handle. A consistent posture is what keeps a secret value from
    /// appearing as a raw literal.
    pub fn secret_posture_consistent(&self) -> bool {
        match (self.field_type.is_secret(), self.secret_reference.as_ref()) {
            (true, Some(reference)) => {
                SECRET_BEARING_REDACTION_CLASSES.contains(&reference.redaction_class.as_str())
                    && SECRET_BEARING_REDACTION_CLASSES.contains(&self.sensitivity_class.as_str())
            }
            (true, None) => self.value_state == ParameterValueState::AwaitingInput,
            (false, Some(_)) => false,
            (false, None) => true,
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter-review builder
// ---------------------------------------------------------------------------

/// An error raised by a [`ParameterReviewBuilder`] mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterReviewError {
    /// No parameter with the given name is present.
    ParameterNotFound(String),
    /// A parameter with the given name is already present.
    DuplicateParameterName(String),
}

impl std::fmt::Display for ParameterReviewError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterReviewError::ParameterNotFound(name) => {
                write!(formatter, "no parameter named {name} is present")
            }
            ParameterReviewError::DuplicateParameterName(name) => {
                write!(formatter, "a parameter named {name} is already present")
            }
        }
    }
}

impl std::error::Error for ParameterReviewError {}

/// The live, mutable parameter-review authoring object.
///
/// The builder owns the ordered list of [`ReviewedParameter`]s; it derives each
/// parameter's verdict and the sheet's unresolved-required count, and projects
/// the frozen [`ParameterReviewSheet`] on demand. It holds no raw secret and no
/// untyped control: every projection reads back through the reviewed parameters,
/// so a consumer reviewing inputs reuses the same verdict truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewBuilder {
    /// Opaque sheet id.
    pub sheet_id: String,
    /// The M5 automation family this sheet reviews input for.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Opaque builder session id this sheet belongs to.
    pub builder_id: String,
    /// Opaque draft recipe revision ref this sheet reviews.
    pub draft_recipe_revision_ref: String,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// Ordered reviewed parameters.
    pub parameters: Vec<ReviewedParameter>,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

impl ParameterReviewBuilder {
    /// Opens an empty sheet for one entrypoint.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        entrypoint: RecipeBuilderEntrypoint,
        sheet_id: impl Into<String>,
        builder_id: impl Into<String>,
        draft_recipe_revision_ref: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        ParameterReviewBuilder {
            sheet_id: sheet_id.into(),
            entrypoint,
            builder_id: builder_id.into(),
            draft_recipe_revision_ref: draft_recipe_revision_ref.into(),
            title: title.into(),
            summary: summary.into(),
            parameters: Vec::new(),
            minted_at: minted_at.into(),
        }
    }

    /// Appends a reviewed parameter to the sheet.
    pub fn add_parameter(
        &mut self,
        parameter: ReviewedParameter,
    ) -> Result<(), ParameterReviewError> {
        if self.parameter(&parameter.parameter_name).is_some() {
            return Err(ParameterReviewError::DuplicateParameterName(
                parameter.parameter_name.clone(),
            ));
        }
        self.parameters.push(parameter);
        Ok(())
    }

    /// The parameter with the given name, if present.
    pub fn parameter(&self, parameter_name: &str) -> Option<&ReviewedParameter> {
        self.parameters
            .iter()
            .find(|parameter| parameter.parameter_name == parameter_name)
    }

    /// Marks a parameter as overridden for this run.
    ///
    /// Overriding preserves the parameter's type, source layer, and save scope;
    /// only the default-versus-override state changes, so the override stays
    /// visible.
    pub fn override_parameter(&mut self, parameter_name: &str) -> Result<(), ParameterReviewError> {
        let parameter = self
            .parameters
            .iter_mut()
            .find(|parameter| parameter.parameter_name == parameter_name)
            .ok_or_else(|| ParameterReviewError::ParameterNotFound(parameter_name.to_owned()))?;
        parameter.value_state = ParameterValueState::Overridden;
        Ok(())
    }

    /// Count of required parameters still awaiting input (apply-blocking when > 0).
    pub fn unresolved_required_count(&self) -> u32 {
        self.parameters
            .iter()
            .filter(|parameter| parameter.is_unresolved_required())
            .count() as u32
    }

    /// Count of parameters held as secret references.
    pub fn secret_reference_count(&self) -> u32 {
        self.parameters
            .iter()
            .filter(|parameter| parameter.secret_reference.is_some())
            .count() as u32
    }

    /// Whether every required parameter is resolved and none is blocked.
    pub fn is_apply_ready(&self) -> bool {
        self.unresolved_required_count() == 0
            && self
                .parameters
                .iter()
                .all(|parameter| parameter.verdict_class() != ParameterReviewVerdictClass::Blocked)
    }

    /// Per-parameter source-layer tokens, index-aligned with the parameters.
    pub fn source_layer_tokens(&self) -> Vec<String> {
        self.parameters
            .iter()
            .map(|parameter| parameter.source_layer.as_str().to_owned())
            .collect()
    }

    /// Per-parameter field-type tokens, index-aligned with the parameters.
    pub fn field_type_tokens(&self) -> Vec<String> {
        self.parameters
            .iter()
            .map(|parameter| parameter.field_type.as_str().to_owned())
            .collect()
    }

    /// Per-parameter chosen-save-scope tokens, index-aligned with the parameters.
    pub fn save_scope_tokens(&self) -> Vec<String> {
        self.parameters
            .iter()
            .map(|parameter| parameter.chosen_save_scope.as_str().to_owned())
            .collect()
    }

    /// Per-parameter redaction-class tokens, index-aligned with the parameters.
    pub fn redaction_class_tokens(&self) -> Vec<String> {
        self.parameters
            .iter()
            .map(|parameter| parameter.sensitivity_class.clone())
            .collect()
    }

    /// Projects the live builder onto the frozen parameter-review-sheet record.
    ///
    /// This is the proof the sheet reuses verdict truth: each emitted row quotes
    /// the same parameter name, inspection kind, verdict, requiredness, and
    /// sensitivity class the builder derives.
    pub fn to_sheet_record(&self) -> ParameterReviewSheet {
        ParameterReviewSheet {
            record_kind: PARAMETER_REVIEW_SHEET_RECORD_KIND.to_owned(),
            recipe_builder_schema_version: PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION,
            sheet_id: self.sheet_id.clone(),
            builder_id: self.builder_id.clone(),
            draft_recipe_revision_ref: self.draft_recipe_revision_ref.clone(),
            rows: self
                .parameters
                .iter()
                .map(ReviewedParameter::to_review_row)
                .collect(),
            unresolved_required_count: self.unresolved_required_count(),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Exports the sheet, preserving full parameter provenance and redaction posture.
    pub fn export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ParameterReviewExport {
        ParameterReviewExport {
            record_kind: PARAMETER_REVIEW_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            sheet_record: self.to_sheet_record(),
            builder: self.clone(),
            export_digest: fnv1a64(&self.digest_tokens()),
        }
    }

    /// Tokens hashed into the export digest, in parameter order.
    fn digest_tokens(&self) -> Vec<String> {
        let mut tokens = vec![self.sheet_id.clone(), self.builder_id.clone()];
        for parameter in &self.parameters {
            tokens.push(parameter.parameter_name.clone());
            tokens.push(parameter.field_type.as_str().to_owned());
            tokens.push(parameter.source_layer.as_str().to_owned());
            tokens.push(parameter.sensitivity_class.clone());
        }
        tokens
    }
}

/// A sheet exported for rerun review, sharing, or support bundles.
///
/// The export nests the whole [`ParameterReviewBuilder`] verbatim — so each
/// parameter's type, source layer, default/override state, save scope, secret
/// reference, and redaction class all survive — alongside the derived
/// frozen-sheet projection and an order-stable digest.
/// [`ParameterReviewExport::import`] reconstructs the identical builder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// The frozen sheet projection consumers read.
    pub sheet_record: ParameterReviewSheet,
    /// The builder, preserved verbatim for round-trip import.
    pub builder: ParameterReviewBuilder,
    /// Order-stable digest over the sheet's parameter provenance.
    pub export_digest: String,
}

impl ParameterReviewExport {
    /// Reconstructs the builder from the export.
    pub fn import(&self) -> ParameterReviewBuilder {
        self.builder.clone()
    }

    /// Whether the export preserves provenance and redaction posture.
    ///
    /// Every parameter must keep a non-empty source layer and redaction class,
    /// every secret-bearing field must hold a reference (no raw value), and the
    /// projection must stay aligned with the live parameters.
    pub fn provenance_preserved(&self) -> bool {
        !self.builder.parameters.is_empty()
            && self.builder.parameters.iter().all(|parameter| {
                !parameter.sensitivity_class.is_empty() && parameter.secret_posture_consistent()
            })
            && self.sheet_record.rows.len() == self.builder.parameters.len()
    }
}

// ---------------------------------------------------------------------------
// First-consumer bindings
// ---------------------------------------------------------------------------

/// One entrypoint binding: the seeded sheet a first consumer reviews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewConsumerBinding {
    /// The entrypoint this binding describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Opaque sheet id.
    pub sheet_id: String,
    /// Opaque builder id the sheet belongs to.
    pub builder_id: String,
    /// The frozen sheet record the consumer reuses.
    pub sheet_record: ParameterReviewSheet,
    /// The live reviewed parameters, carrying the typed dimensions.
    pub reviewed_parameters: Vec<ReviewedParameter>,
    /// Parameter count, carried for compact projections.
    pub parameter_count: u32,
    /// Count of required parameters still awaiting input.
    pub unresolved_required_count: u32,
    /// Count of parameters held as secret references.
    pub secret_reference_count: u32,
    /// Reviewable summary of what the consumer reviews.
    pub entry_summary: String,
}

impl ParameterReviewConsumerBinding {
    /// Builds a binding from a consumer's authored sheet.
    pub fn from_builder(builder: &ParameterReviewBuilder) -> Self {
        ParameterReviewConsumerBinding {
            entrypoint: builder.entrypoint,
            title: builder.entrypoint.title().to_owned(),
            sheet_id: builder.sheet_id.clone(),
            builder_id: builder.builder_id.clone(),
            sheet_record: builder.to_sheet_record(),
            reviewed_parameters: builder.parameters.clone(),
            parameter_count: builder.parameters.len() as u32,
            unresolved_required_count: builder.unresolved_required_count(),
            secret_reference_count: builder.secret_reference_count(),
            entry_summary: builder.summary.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Invariants and findings
// ---------------------------------------------------------------------------

/// Frozen invariants the first-consumers packet pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewInvariantsBlock {
    /// Every reviewed parameter carries an explicit field type.
    pub every_parameter_is_typed: bool,
    /// Every parameter's source layer is explicit, never a generic control.
    pub source_layer_is_explicit_for_every_parameter: bool,
    /// Default-versus-override state stays visible per parameter.
    pub default_or_override_state_is_visible: bool,
    /// Secret-bearing values are references, never raw literals.
    pub secret_values_are_references_not_raw: bool,
    /// Save-to-scope is explicit and within the allowed set.
    pub save_to_scope_is_explicit_and_allowed: bool,
    /// Review verdicts reuse the frozen verdict vocabulary.
    pub verdicts_reuse_the_frozen_vocabulary: bool,
    /// Provenance and redaction posture survive export and import.
    pub provenance_and_redaction_survive_export_import: bool,
}

impl ParameterReviewInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        ParameterReviewInvariantsBlock {
            every_parameter_is_typed: true,
            source_layer_is_explicit_for_every_parameter: true,
            default_or_override_state_is_visible: true,
            secret_values_are_references_not_raw: true,
            save_to_scope_is_explicit_and_allowed: true,
            verdicts_reuse_the_frozen_vocabulary: true,
            provenance_and_redaction_survive_export_import: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 7] {
        [
            ("every_parameter_is_typed", self.every_parameter_is_typed),
            (
                "source_layer_is_explicit_for_every_parameter",
                self.source_layer_is_explicit_for_every_parameter,
            ),
            (
                "default_or_override_state_is_visible",
                self.default_or_override_state_is_visible,
            ),
            (
                "secret_values_are_references_not_raw",
                self.secret_values_are_references_not_raw,
            ),
            (
                "save_to_scope_is_explicit_and_allowed",
                self.save_to_scope_is_explicit_and_allowed,
            ),
            (
                "verdicts_reuse_the_frozen_vocabulary",
                self.verdicts_reuse_the_frozen_vocabulary,
            ),
            (
                "provenance_and_redaction_survive_export_import",
                self.provenance_and_redaction_survive_export_import,
            ),
        ]
    }
}

/// Severity of a parameter-review validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterReviewFindingSeverity {
    /// Blocks the packet from stable.
    Blocker,
    /// Narrows the packet below stable.
    Warning,
}

/// Kind of a parameter-review validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterReviewFindingKind {
    /// A required first-consumer entrypoint is absent.
    MissingEntrypoint,
    /// An entrypoint binds a sheet with no parameters.
    EntrypointSheetEmpty,
    /// A parameter's source layer is unspecified — an ambiguous generic control.
    SourceLayerUnspecified,
    /// A secret-bearing value is not held as a reference.
    SecretValueNotReferenced,
    /// A chosen save scope is outside the allowed set.
    SaveScopeNotAllowed,
    /// The frozen sheet projection disagrees with the live parameters.
    SheetProjectionInconsistent,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl ParameterReviewFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ParameterReviewFindingKind::MissingEntrypoint => "missing_entrypoint",
            ParameterReviewFindingKind::EntrypointSheetEmpty => "entrypoint_sheet_empty",
            ParameterReviewFindingKind::SourceLayerUnspecified => "source_layer_unspecified",
            ParameterReviewFindingKind::SecretValueNotReferenced => "secret_value_not_referenced",
            ParameterReviewFindingKind::SaveScopeNotAllowed => "save_scope_not_allowed",
            ParameterReviewFindingKind::SheetProjectionInconsistent => {
                "sheet_projection_inconsistent"
            }
            ParameterReviewFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by the first-consumers gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewFinding {
    /// The finding kind.
    pub finding_kind: ParameterReviewFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: ParameterReviewFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl ParameterReviewFinding {
    fn blocker(
        finding_kind: ParameterReviewFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        ParameterReviewFinding {
            finding_kind,
            severity: ParameterReviewFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// First-consumers packet
// ---------------------------------------------------------------------------

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewFirstConsumersInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<ParameterReviewConsumerBinding>,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: ParameterReviewInvariantsBlock,
}

/// Canonical M5 parameter-review first-consumers packet.
///
/// The packet binds every first-consumer entrypoint to a seeded sheet and pins
/// the freeze invariants. [`ParameterReviewFirstConsumersPacket::validate`]
/// recomputes the findings so the fail-closed gate and the typed consumer agree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewFirstConsumersPacket {
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
    /// Reused frozen-sheet boundary schema ref.
    pub sheet_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<ParameterReviewConsumerBinding>,
    /// Frozen invariants block.
    pub invariants: ParameterReviewInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<ParameterReviewFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over entrypoint and parameter tokens.
    pub packet_digest: String,
}

impl ParameterReviewFirstConsumersPacket {
    /// Freezes an input into a packet, computing findings, promotion, and digest.
    pub fn materialize(input: ParameterReviewFirstConsumersInput) -> Self {
        let findings = validate_parts(&input.consumer_bindings, &input.invariants);
        let promotion_state = promotion_state_for_findings(&findings);
        let packet_digest = packet_digest(&input.consumer_bindings);
        ParameterReviewFirstConsumersPacket {
            record_kind: PARAMETER_REVIEW_FIRST_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            schema_ref: PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            sheet_schema_ref: PARAMETER_REVIEW_SHEET_SCHEMA_REF.to_owned(),
            doc_ref: PARAMETER_REVIEW_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            consumer_bindings: input.consumer_bindings,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            packet_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<ParameterReviewFinding> {
        validate_parts(&self.consumer_bindings, &self.invariants)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// The binding for one entrypoint, if present.
    pub fn binding(
        &self,
        entrypoint: RecipeBuilderEntrypoint,
    ) -> Option<&ParameterReviewConsumerBinding> {
        self.consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
    }

    /// Entrypoint tokens in the order the packet stores them.
    pub fn entrypoint_tokens(&self) -> Vec<&'static str> {
        self.consumer_bindings
            .iter()
            .map(|binding| binding.entrypoint.as_str())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ParameterReviewFirstConsumersSupportExport {
        ParameterReviewFirstConsumersSupportExport {
            record_kind: PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            packet_digest: self.packet_digest.clone(),
            promotion_state: self.promotion_state,
            consumer_rows: self
                .consumer_bindings
                .iter()
                .map(|binding| ParameterReviewSupportConsumerRow {
                    entrypoint: binding.entrypoint,
                    title: binding.title.clone(),
                    sheet_id: binding.sheet_id.clone(),
                    parameter_count: binding.parameter_count,
                    unresolved_required_count: binding.unresolved_required_count,
                    secret_reference_count: binding.secret_reference_count,
                    parameter_rows: binding
                        .reviewed_parameters
                        .iter()
                        .map(ParameterReviewSupportParameterRow::from_parameter)
                        .collect(),
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
    ) -> ParameterReviewFirstConsumersCliHeadlessView {
        ParameterReviewFirstConsumersCliHeadlessView {
            record_kind: PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            consumer_lines: self
                .consumer_bindings
                .iter()
                .map(|binding| {
                    format!(
                        "{} sheet={} params={} unresolved={} secrets={}",
                        binding.entrypoint.as_str(),
                        binding.sheet_id,
                        binding.parameter_count,
                        binding.unresolved_required_count,
                        binding.secret_reference_count,
                    )
                })
                .collect(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} consumers={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.consumer_bindings.len(),
            self.packet_digest,
        )];
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} sheet={} params={} unresolved={} secrets={}",
                binding.entrypoint.as_str(),
                binding.sheet_id,
                binding.parameter_count,
                binding.unresolved_required_count,
                binding.secret_reference_count,
            ));
            for parameter in &binding.reviewed_parameters {
                lines.push(format!(
                    "  param {} type={} source={} state={} scope={} verdict={} secret={}",
                    parameter.parameter_name,
                    parameter.field_type.as_str(),
                    parameter.source_layer.as_str(),
                    parameter.value_state.as_str(),
                    parameter.chosen_save_scope.as_str(),
                    parameter.verdict_class().as_str(),
                    parameter.secret_reference.is_some(),
                ));
            }
        }
        lines
    }
}

/// One support-export parameter row (redacted projection, no raw value).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewSupportParameterRow {
    /// Snake_case parameter name.
    pub parameter_name: String,
    /// Field type token.
    pub field_type: ParameterFieldType,
    /// Source layer token (provenance).
    pub source_layer: ParameterSourceLayer,
    /// Default-versus-override state.
    pub value_state: ParameterValueState,
    /// Derived review verdict.
    pub verdict_class: ParameterReviewVerdictClass,
    /// Chosen save scope.
    pub chosen_save_scope: SaveToScope,
    /// Re-exported redaction class.
    pub sensitivity_class: String,
    /// Whether the value is held as a secret reference.
    pub held_as_secret_reference: bool,
}

impl ParameterReviewSupportParameterRow {
    fn from_parameter(parameter: &ReviewedParameter) -> Self {
        ParameterReviewSupportParameterRow {
            parameter_name: parameter.parameter_name.clone(),
            field_type: parameter.field_type,
            source_layer: parameter.source_layer,
            value_state: parameter.value_state,
            verdict_class: parameter.verdict_class(),
            chosen_save_scope: parameter.chosen_save_scope,
            sensitivity_class: parameter.sensitivity_class.clone(),
            held_as_secret_reference: parameter.secret_reference.is_some(),
        }
    }
}

/// One support-export consumer row (redacted projection).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewSupportConsumerRow {
    /// The entrypoint this row describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Opaque sheet id.
    pub sheet_id: String,
    /// Parameter count.
    pub parameter_count: u32,
    /// Count of required parameters still awaiting input.
    pub unresolved_required_count: u32,
    /// Count of parameters held as secret references.
    pub secret_reference_count: u32,
    /// Per-parameter redacted rows.
    pub parameter_rows: Vec<ParameterReviewSupportParameterRow>,
}

/// Redacted support-export projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewFirstConsumersSupportExport {
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
    /// Consumer rows.
    pub consumer_rows: Vec<ParameterReviewSupportConsumerRow>,
    /// Frozen invariants block.
    pub invariants: ParameterReviewInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<ParameterReviewFindingKind>,
}

impl ParameterReviewFirstConsumersSupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary.
    ///
    /// A support export is safe only when no row carries a raw secret; secret
    /// values appear as references, never literals, so the export is structurally
    /// redacted.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.packet_digest.is_empty()
            && !self.consumer_rows.is_empty()
    }
}

/// Compact CLI / headless projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewFirstConsumersCliHeadlessView {
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
    /// One line per consumer entrypoint.
    pub consumer_lines: Vec<String>,
}

impl ParameterReviewFirstConsumersCliHeadlessView {
    /// Whether the view explains every entrypoint.
    pub fn every_entrypoint_explained(&self) -> bool {
        self.consumer_lines.len() == RecipeBuilderEntrypoint::ALL.len()
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_parts(
    consumer_bindings: &[ParameterReviewConsumerBinding],
    invariants: &ParameterReviewInvariantsBlock,
) -> Vec<ParameterReviewFinding> {
    let mut findings = Vec::new();

    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let Some(binding) = consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
        else {
            findings.push(ParameterReviewFinding::blocker(
                ParameterReviewFindingKind::MissingEntrypoint,
                Some(entrypoint.as_str().to_owned()),
                format!(
                    "the {} entrypoint binds no parameter-review sheet",
                    entrypoint.as_str()
                ),
            ));
            continue;
        };
        validate_binding(binding, &mut findings);
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(ParameterReviewFinding::blocker(
                ParameterReviewFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn validate_binding(
    binding: &ParameterReviewConsumerBinding,
    findings: &mut Vec<ParameterReviewFinding>,
) {
    let entrypoint = binding.entrypoint.as_str();
    let parameters = &binding.reviewed_parameters;

    if parameters.is_empty() {
        findings.push(ParameterReviewFinding::blocker(
            ParameterReviewFindingKind::EntrypointSheetEmpty,
            Some(entrypoint.to_owned()),
            format!("the {entrypoint} entrypoint binds a sheet with no parameters"),
        ));
        return;
    }

    // The frozen projection must stay aligned with the live parameters.
    if binding.sheet_record.rows.len() != parameters.len() {
        findings.push(ParameterReviewFinding::blocker(
            ParameterReviewFindingKind::SheetProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} sheet projects {} rows for {} parameters",
                binding.sheet_record.rows.len(),
                parameters.len()
            ),
        ));
    }
    let recomputed_unresolved = parameters
        .iter()
        .filter(|parameter| parameter.is_unresolved_required())
        .count() as u32;
    if binding.sheet_record.unresolved_required_count != recomputed_unresolved {
        findings.push(ParameterReviewFinding::blocker(
            ParameterReviewFindingKind::SheetProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} sheet reports {} unresolved required, recomputed {recomputed_unresolved}",
                binding.sheet_record.unresolved_required_count
            ),
        ));
    }

    for (index, parameter) in parameters.iter().enumerate() {
        let subject = format!("{entrypoint}:{}", parameter.parameter_name);

        if parameter.source_layer.explicit_inspection_kind().is_none() {
            findings.push(ParameterReviewFinding::blocker(
                ParameterReviewFindingKind::SourceLayerUnspecified,
                Some(subject.clone()),
                format!(
                    "parameter {} on {entrypoint} hides in a generic control with no source layer",
                    parameter.parameter_name
                ),
            ));
        }

        if !parameter.secret_posture_consistent() {
            findings.push(ParameterReviewFinding::blocker(
                ParameterReviewFindingKind::SecretValueNotReferenced,
                Some(subject.clone()),
                format!(
                    "parameter {} on {entrypoint} carries a secret-bearing value that is not held as a reference",
                    parameter.parameter_name
                ),
            ));
        }

        if !parameter.save_scope_allowed() {
            findings.push(ParameterReviewFinding::blocker(
                ParameterReviewFindingKind::SaveScopeNotAllowed,
                Some(subject.clone()),
                format!(
                    "parameter {} on {entrypoint} chooses save scope {} outside its allowed set",
                    parameter.parameter_name,
                    parameter.chosen_save_scope.as_str()
                ),
            ));
        }

        // The frozen row must quote the same verdict truth as the live parameter.
        if let Some(row) = binding.sheet_record.rows.get(index) {
            if row.parameter_name != parameter.parameter_name
                || row.inspection_kind != parameter.source_layer.inspection_kind()
                || row.verdict_class != parameter.verdict_class()
                || row.required != parameter.required
                || row.sensitivity_class != parameter.sensitivity_class
            {
                findings.push(ParameterReviewFinding::blocker(
                    ParameterReviewFindingKind::SheetProjectionInconsistent,
                    Some(subject.clone()),
                    format!(
                        "the projected row for {} on {entrypoint} disagrees with the reviewed parameter",
                        parameter.parameter_name
                    ),
                ));
            }
        }
    }
}

fn promotion_state_for_findings(
    findings: &[ParameterReviewFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == ParameterReviewFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == ParameterReviewFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn packet_digest(consumer_bindings: &[ParameterReviewConsumerBinding]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    for binding in consumer_bindings {
        tokens.push(binding.entrypoint.as_str().to_owned());
        for parameter in &binding.reviewed_parameters {
            tokens.push(parameter.parameter_name.clone());
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

fn s(value: &str) -> String {
    value.to_owned()
}

fn scopes(values: &[SaveToScope]) -> Vec<SaveToScope> {
    values.to_vec()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Builds a non-secret reviewed parameter.
#[allow(clippy::too_many_arguments)]
fn parameter(
    parameter_name: &str,
    field_type: ParameterFieldType,
    source_layer: ParameterSourceLayer,
    value_state: ParameterValueState,
    required: bool,
    sensitivity_class: &str,
    chosen_save_scope: SaveToScope,
    available_save_scopes: &[SaveToScope],
    validation: ParameterValidation,
    summary: &str,
) -> ReviewedParameter {
    ReviewedParameter {
        parameter_name: s(parameter_name),
        field_type,
        source_layer,
        value_state,
        required,
        sensitivity_class: s(sensitivity_class),
        secret_reference: None,
        chosen_save_scope,
        available_save_scopes: scopes(available_save_scopes),
        validation,
        summary: s(summary),
    }
}

/// Builds a secret-bearing reviewed parameter held behind a broker handle.
#[allow(clippy::too_many_arguments)]
fn secret_parameter(
    parameter_name: &str,
    source_layer: ParameterSourceLayer,
    required: bool,
    sensitivity_class: &str,
    broker_handle_ref: &str,
    chosen_save_scope: SaveToScope,
    available_save_scopes: &[SaveToScope],
    summary: &str,
) -> ReviewedParameter {
    ReviewedParameter {
        parameter_name: s(parameter_name),
        field_type: ParameterFieldType::SecretReference,
        source_layer,
        value_state: ParameterValueState::DefaultValue,
        required,
        sensitivity_class: s(sensitivity_class),
        secret_reference: Some(SecretReference {
            broker_handle_ref: s(broker_handle_ref),
            redaction_class: s(sensitivity_class),
        }),
        chosen_save_scope,
        available_save_scopes: scopes(available_save_scopes),
        validation: ParameterValidation::satisfied(
            ParameterConstraintKind::SecretBrokerHandlePresent,
            "a resolvable secret-broker handle is present",
        ),
        summary: s(summary),
    }
}

/// Existing contracts the first-consumers packet reuses instead of re-deciding.
pub fn canonical_reused_contract_refs() -> Vec<String> {
    strings(&[
        PARAMETER_REVIEW_SHEET_SCHEMA_REF,
        "schemas/automation/automation-contract-baseline.schema.json",
        "schemas/automation/recipe-builder-first-consumers.schema.json",
        "schemas/automation/recipe_manifest.schema.json",
        "schemas/commands/command_descriptor.schema.json",
        "schemas/commands/shareability_metadata.schema.json",
        "docs/m5/recipe-builder-and-macro-contract.md",
        "docs/automation/recipe_and_macro_contract.md",
    ])
}

/// Builds the seeded sheet one first consumer reviews.
pub fn seeded_consumer_sheet(entrypoint: RecipeBuilderEntrypoint) -> ParameterReviewBuilder {
    use ParameterConstraintKind::{
        EnumMembership, EnvironmentProfileResolvable, IntegerRange, NonEmpty, UrlScheme,
        WorkspaceRelativePath,
    };
    use ParameterFieldType::{
        Enumeration, EnvironmentProfileRef, Integer, PathReference, Text, UrlReference,
    };
    use ParameterSourceLayer::{
        AiProposed, DescriptorDefault, FocusedContextBacked, PolicyPinned, RecipeSupplied,
        SecretBroker, SelectionBacked, UserSaved, WorkspaceSaved,
    };
    use ParameterValueState::{
        AwaitingInput, DefaultValue, Overridden, PolicyPinned as PinnedState,
    };
    use SaveToScope::{OrganizationPolicy, RunOnly, User, Workspace};

    match entrypoint {
        RecipeBuilderEntrypoint::Notebook => {
            let mut sheet = ParameterReviewBuilder::new(
                entrypoint,
                "sheet:notebook-run-and-export:v1",
                "builder:notebook:run-and-export:v1",
                "recipe-rev:notebook-run-and-export:1",
                "Review notebook run-and-export inputs",
                "Reviews the kernel profile, export format, and output location before the notebook runs.",
                "2026-06-18T00:00:00Z",
            );
            sheet
                .add_parameter(parameter(
                    "kernel_profile",
                    Enumeration,
                    DescriptorDefault,
                    DefaultValue,
                    false,
                    "metadata_safe_default",
                    Workspace,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(
                        EnumMembership,
                        "one of the registered kernel profiles",
                    ),
                    "kernel_profile defaults to the workspace kernel profile",
                ))
                .expect("add kernel_profile");
            sheet
                .add_parameter(parameter(
                    "export_format",
                    Enumeration,
                    UserSaved,
                    Overridden,
                    false,
                    "metadata_safe_default",
                    User,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(EnumMembership, "one of html, pdf, or markdown"),
                    "export_format is overridden to html for this run",
                ))
                .expect("add export_format");
            sheet
                .add_parameter(parameter(
                    "output_dir",
                    PathReference,
                    FocusedContextBacked,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    Workspace,
                    &[RunOnly, Workspace],
                    ParameterValidation::satisfied(
                        WorkspaceRelativePath,
                        "a workspace-relative directory",
                    ),
                    "output_dir is bound to the focused notebook's folder",
                ))
                .expect("add output_dir");
            sheet
        }
        RecipeBuilderEntrypoint::TaskTestDebug => {
            let mut sheet = ParameterReviewBuilder::new(
                entrypoint,
                "sheet:task-test-then-rerun-failed:v1",
                "builder:task:test-then-rerun-failed:v1",
                "recipe-rev:task-test-then-rerun-failed:1",
                "Review test run inputs",
                "Reviews the test selector, parallelism, and coverage threshold before tests run.",
                "2026-06-18T00:00:00Z",
            );
            sheet
                .add_parameter(parameter(
                    "test_selector",
                    Text,
                    SelectionBacked,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    RunOnly,
                    &[RunOnly, Workspace],
                    ParameterValidation::satisfied(NonEmpty, "a non-empty test selector"),
                    "test_selector is bound to the selected tests",
                ))
                .expect("add test_selector");
            sheet
                .add_parameter(parameter(
                    "max_parallelism",
                    Integer,
                    WorkspaceSaved,
                    Overridden,
                    false,
                    "metadata_safe_default",
                    Workspace,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(IntegerRange, "an integer in 1..=64"),
                    "max_parallelism is overridden to 8 for this run",
                ))
                .expect("add max_parallelism");
            sheet
                .add_parameter(parameter(
                    "coverage_threshold",
                    Integer,
                    DescriptorDefault,
                    DefaultValue,
                    false,
                    "metadata_safe_default",
                    User,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(
                        IntegerRange,
                        "an integer percentage in 0..=100",
                    ),
                    "coverage_threshold defaults to the descriptor value",
                ))
                .expect("add coverage_threshold");
            sheet
        }
        RecipeBuilderEntrypoint::RequestApi => {
            let mut sheet = ParameterReviewBuilder::new(
                entrypoint,
                "sheet:request-send-and-save:v1",
                "builder:request:send-and-save:v1",
                "recipe-rev:request-send-and-save:1",
                "Review request send-and-save inputs",
                "Reviews the environment profile, URL, bearer token reference, and body variable before the request is sent.",
                "2026-06-18T00:00:00Z",
            );
            sheet
                .add_parameter(parameter(
                    "environment_profile",
                    EnvironmentProfileRef,
                    WorkspaceSaved,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    Workspace,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(
                        EnvironmentProfileResolvable,
                        "a resolvable environment profile",
                    ),
                    "environment_profile is the saved staging profile",
                ))
                .expect("add environment_profile");
            sheet
                .add_parameter(parameter(
                    "request_url",
                    UrlReference,
                    RecipeSupplied,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    RunOnly,
                    &[RunOnly, Workspace],
                    ParameterValidation::satisfied(
                        UrlScheme,
                        "an https URL resolved against the profile",
                    ),
                    "request_url is supplied by the recipe and resolved against the profile",
                ))
                .expect("add request_url");
            sheet
                .add_parameter(secret_parameter(
                    "bearer_token",
                    SecretBroker,
                    true,
                    "redaction_required_with_secret_broker_handles",
                    "secret-broker:request.bearer_token",
                    User,
                    &[RunOnly, Workspace, User],
                    "bearer_token is held as a secret-broker reference, never a raw value",
                ))
                .expect("add bearer_token");
            sheet
                .add_parameter(parameter(
                    "body_variable",
                    Text,
                    UserSaved,
                    AwaitingInput,
                    true,
                    "metadata_safe_default",
                    RunOnly,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(NonEmpty, "a non-empty body variable"),
                    "body_variable still needs input before the request can be sent",
                ))
                .expect("add body_variable");
            sheet
        }
        RecipeBuilderEntrypoint::Package => {
            let mut sheet = ParameterReviewBuilder::new(
                entrypoint,
                "sheet:package-audit-then-update:v1",
                "builder:package:audit-then-update:v1",
                "recipe-rev:package-audit-then-update:1",
                "Review dependency audit-and-update inputs",
                "Reviews the audit scope, the policy-pinned update channel, and the registry token reference.",
                "2026-06-18T00:00:00Z",
            );
            sheet
                .add_parameter(parameter(
                    "audit_scope",
                    Enumeration,
                    DescriptorDefault,
                    DefaultValue,
                    false,
                    "metadata_safe_default",
                    Workspace,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(
                        EnumMembership,
                        "one of direct, transitive, or all",
                    ),
                    "audit_scope defaults to direct dependencies",
                ))
                .expect("add audit_scope");
            sheet
                .add_parameter(parameter(
                    "update_channel",
                    Enumeration,
                    PolicyPinned,
                    PinnedState,
                    true,
                    "metadata_safe_default",
                    OrganizationPolicy,
                    &[OrganizationPolicy],
                    ParameterValidation::satisfied(
                        EnumMembership,
                        "the policy-pinned stable channel",
                    ),
                    "update_channel is pinned to stable by organization policy",
                ))
                .expect("add update_channel");
            sheet
                .add_parameter(secret_parameter(
                    "registry_token",
                    SecretBroker,
                    true,
                    "redaction_required_with_secret_broker_handles",
                    "secret-broker:package.registry_token",
                    Workspace,
                    &[RunOnly, Workspace, User],
                    "registry_token is held as a secret-broker reference for the private registry",
                ))
                .expect("add registry_token");
            sheet
        }
        RecipeBuilderEntrypoint::Incident => {
            let mut sheet = ParameterReviewBuilder::new(
                entrypoint,
                "sheet:incident-capture-evidence:v1",
                "builder:incident:capture-evidence:v1",
                "recipe-rev:incident-capture-evidence:1",
                "Review incident evidence-capture inputs",
                "Reviews the incident reference, redaction profile, and bundle destination before capture.",
                "2026-06-18T00:00:00Z",
            );
            sheet
                .add_parameter(parameter(
                    "incident_ref",
                    Text,
                    FocusedContextBacked,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    RunOnly,
                    &[RunOnly, Workspace],
                    ParameterValidation::satisfied(NonEmpty, "the focused incident reference"),
                    "incident_ref is bound to the focused incident",
                ))
                .expect("add incident_ref");
            sheet
                .add_parameter(parameter(
                    "redaction_profile",
                    Enumeration,
                    WorkspaceSaved,
                    DefaultValue,
                    false,
                    "internal_support_restricted",
                    Workspace,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(
                        EnumMembership,
                        "one of the registered redaction profiles",
                    ),
                    "redaction_profile defaults to the internal-support profile",
                ))
                .expect("add redaction_profile");
            sheet
                .add_parameter(parameter(
                    "bundle_destination",
                    PathReference,
                    UserSaved,
                    Overridden,
                    false,
                    "metadata_safe_default",
                    User,
                    &[RunOnly, Workspace, User],
                    ParameterValidation::satisfied(
                        WorkspaceRelativePath,
                        "a workspace-relative directory",
                    ),
                    "bundle_destination is overridden to the support folder for this run",
                ))
                .expect("add bundle_destination");
            sheet
        }
        RecipeBuilderEntrypoint::AiAssistant => {
            let mut sheet = ParameterReviewBuilder::new(
                entrypoint,
                "sheet:ai-apply-proposed-fix:v1",
                "builder:ai:apply-proposed-fix:v1",
                "recipe-rev:ai-apply-proposed-fix:1",
                "Review AI-proposed fix inputs",
                "Reviews the proposal id, apply mode, and signing-key reference before the AI-proposed fix is applied.",
                "2026-06-18T00:00:00Z",
            );
            sheet
                .add_parameter(parameter(
                    "proposal_id",
                    Text,
                    AiProposed,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    RunOnly,
                    &[RunOnly, Workspace],
                    ParameterValidation::satisfied(NonEmpty, "the AI-proposed proposal id"),
                    "proposal_id is proposed by the AI assistant",
                ))
                .expect("add proposal_id");
            sheet
                .add_parameter(parameter(
                    "apply_mode",
                    Enumeration,
                    AiProposed,
                    DefaultValue,
                    true,
                    "metadata_safe_default",
                    RunOnly,
                    &[RunOnly, Workspace],
                    ParameterValidation::satisfied(
                        EnumMembership,
                        "one of dry_run or apply_under_approval",
                    ),
                    "apply_mode is proposed as apply-under-approval",
                ))
                .expect("add apply_mode");
            sheet
                .add_parameter(secret_parameter(
                    "signing_key",
                    SecretBroker,
                    true,
                    "signing_evidence_only",
                    "secret-broker:ai.signing_key",
                    RunOnly,
                    &[RunOnly],
                    "signing_key is held as a signing-evidence-only secret reference",
                ))
                .expect("add signing_key");
            sheet
        }
    }
}

/// Builds the canonical stable first-consumers input.
pub fn current_parameter_review_first_consumers_input() -> ParameterReviewFirstConsumersInput {
    let consumer_bindings = RecipeBuilderEntrypoint::ALL
        .into_iter()
        .map(|entrypoint| {
            ParameterReviewConsumerBinding::from_builder(&seeded_consumer_sheet(entrypoint))
        })
        .collect();
    ParameterReviewFirstConsumersInput {
        packet_id: PARAMETER_REVIEW_FIRST_CONSUMERS_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        consumer_bindings,
        reused_contract_refs: canonical_reused_contract_refs(),
        invariants: ParameterReviewInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable first-consumers packet.
pub fn seeded_parameter_review_first_consumers_packet() -> ParameterReviewFirstConsumersPacket {
    ParameterReviewFirstConsumersPacket::materialize(
        current_parameter_review_first_consumers_input(),
    )
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_parameter_review_first_consumers_packet(
    packet: &ParameterReviewFirstConsumersPacket,
) -> Result<(), Vec<ParameterReviewFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Worked example: the request/API sheet exported for round-trip review.
///
/// The request sheet carries a held secret reference and an overridden value, so
/// the round-trip proves provenance and redaction posture survive export.
pub fn seeded_parameter_review_export_roundtrip() -> ParameterReviewExport {
    seeded_consumer_sheet(RecipeBuilderEntrypoint::RequestApi)
        .export("export:request-send-and-save:v1", "2026-06-18T00:01:00Z")
}

/// Worked example: the package sheet that holds a registry-token secret reference.
pub fn seeded_secret_reference_sheet() -> ParameterReviewBuilder {
    seeded_consumer_sheet(RecipeBuilderEntrypoint::Package)
}
