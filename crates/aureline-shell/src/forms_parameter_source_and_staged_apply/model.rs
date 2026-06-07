//! Governed structured-input record for source, validation, and apply truth.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Stable record-kind tag carried in serialized form truth records.
pub const FORM_TRUTH_RECORD_KIND: &str = "forms_parameter_source_and_staged_apply_record";

/// Schema version for [`FormTruthPacketRecord`] payloads.
pub const FORM_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every stable structured-input surface.
pub const FORM_TRUTH_SHARED_CONTRACT_REF: &str = "shell:forms_parameter_source_staged_apply:v1";

/// Reviewer-facing notice rendered on every governed structured-input surface.
pub const FORM_TRUTH_NOTICE: &str =
    "Structured input truth: every stable form row shows label, value, requirement, \
     source, validation, action, apply timing, side effects, and export posture; \
     parameter precedence keeps defaults, detected values, imports, workspace \
     values, policy, user overrides, and secret references distinct; async \
     validation preserves last-known results while pending and invalidates stale \
     results when targets or dependencies change; staged and preview-first flows \
     preserve dirty state, checkpoint, review, revert, and exact final action \
     wording; secret, path, object-reference, and code-backed fields disclose \
     storage mode, basis, stable identity, preview path, and redaction defaults.";

/// Surface family rendering the structured-input contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormSurfaceClass {
    /// Stable settings editor or effective-value inspector.
    SettingsEditor,
    /// First-run or setup card.
    SetupCard,
    /// Template, starter, scaffold, or migration wizard.
    ScaffoldWizard,
    /// Provider/account connection or install flow.
    ProviderAccountFlow,
    /// Policy or entitlement review form.
    PolicyReview,
    /// Publish, mutation-review, or generated-comment sheet.
    PublishReview,
    /// Recovery, repair, or support handoff form.
    RecoveryReview,
}

impl FormSurfaceClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsEditor => "settings_editor",
            Self::SetupCard => "setup_card",
            Self::ScaffoldWizard => "scaffold_wizard",
            Self::ProviderAccountFlow => "provider_account_flow",
            Self::PolicyReview => "policy_review",
            Self::PublishReview => "publish_review",
            Self::RecoveryReview => "recovery_review",
        }
    }
}

/// Client authority context where the form is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScope {
    /// Full desktop product authority.
    Desktop,
    /// Remote workspace authority with negotiated target state.
    RemoteWorkspace,
    /// Managed workspace under organization policy.
    ManagedWorkspace,
    /// Browser companion with reduced authority.
    BrowserCompanion,
    /// Restricted client with unsupported or read-only fields.
    RestrictedClient,
    /// Degraded or offline context using cached evidence.
    OfflineDegraded,
}

impl ClientScope {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::RemoteWorkspace => "remote_workspace",
            Self::ManagedWorkspace => "managed_workspace",
            Self::BrowserCompanion => "browser_companion",
            Self::RestrictedClient => "restricted_client",
            Self::OfflineDegraded => "offline_degraded",
        }
    }

    fn requires_limitation_disclosure(self) -> bool {
        matches!(self, Self::BrowserCompanion | Self::RestrictedClient)
    }
}

/// Parameter source vocabulary used on source rows and precedence audits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePrecedence {
    /// Built-in or descriptor default.
    Default,
    /// Value detected from host, runtime, repository, or provider state.
    Detected,
    /// Value imported from migration, bundle, or previous IDE state.
    Imported,
    /// Workspace-visible configuration value.
    Workspace,
    /// Policy-enforced or organization-managed value.
    Policy,
    /// User-authored override.
    UserOverride,
    /// Secret handle, keychain item, delegated identity, or vault reference.
    SecretReference,
    /// Narrow runtime prompt source used when the value is collected per run.
    RuntimePrompt,
}

impl SourcePrecedence {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Detected => "detected",
            Self::Imported => "imported",
            Self::Workspace => "workspace",
            Self::Policy => "policy",
            Self::UserOverride => "user_override",
            Self::SecretReference => "secret_reference",
            Self::RuntimePrompt => "runtime_prompt",
        }
    }

    /// Returns the controlled display label for this source.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Detected => "Detected",
            Self::Imported => "Imported",
            Self::Workspace => "Workspace value",
            Self::Policy => "Policy",
            Self::UserOverride => "User override",
            Self::SecretReference => "Secret reference",
            Self::RuntimePrompt => "Runtime prompt",
        }
    }
}

/// Requirement state displayed beside a field label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementState {
    /// Required unconditionally.
    Required,
    /// Optional and omission has declared behavior.
    Optional,
    /// Required only because another field, target, or policy state requires it.
    Conditional,
}

/// Field kind used to apply additional truth requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldKind {
    /// Ordinary typed value.
    Plain,
    /// Secret or credential reference field.
    Secret,
    /// Local, remote, container, or workspace path field.
    Path,
    /// Object reference with user label and stable identity.
    ObjectReference,
    /// Code-backed config, policy, or generated-artifact field.
    CodeBacked,
}

/// Apply timing displayed by the field and form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyTiming {
    /// Applies immediately in the current scope.
    Immediate,
    /// Staged until explicit save or apply.
    Staged,
    /// Requires preview or dry run before commit.
    PreviewFirst,
    /// Effective value is managed and inspect-only.
    PolicyLocked,
}

impl ApplyTiming {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Staged => "staged",
            Self::PreviewFirst => "preview_first",
            Self::PolicyLocked => "policy_locked",
        }
    }
}

/// Validation class disclosed beside async and synchronous checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationClass {
    /// Local syntax or parser check.
    LocalSyntax,
    /// Schema, model, or cross-field dependency check.
    SchemaModel,
    /// Host, container, runtime, binary, or path probe.
    EnvironmentProbe,
    /// Remote provider, identity, auth, or account check.
    RemoteAuth,
    /// Policy or entitlement decision.
    PolicyEntitlement,
    /// Dry-run or preview validation.
    DryRunPreview,
}

impl ValidationClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSyntax => "local_syntax",
            Self::SchemaModel => "schema_model",
            Self::EnvironmentProbe => "environment_probe",
            Self::RemoteAuth => "remote_auth",
            Self::PolicyEntitlement => "policy_entitlement",
            Self::DryRunPreview => "dry_run_preview",
        }
    }
}

/// Field validation state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationState {
    /// Untouched or not yet resolved.
    Pristine,
    /// Required input is missing.
    Incomplete,
    /// Value violates a rule.
    Invalid,
    /// Value is allowed but risky or lossy.
    Warning,
    /// Policy, client scope, or dependency prevents use.
    Blocked,
    /// Current value passes checks in current context.
    Ready,
    /// Newer validation is running while last-known truth remains visible.
    Pending,
    /// Previous result is no longer valid for the current dependency or target.
    Stale,
}

impl ValidationState {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pristine => "pristine",
            Self::Incomplete => "incomplete",
            Self::Invalid => "invalid",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
            Self::Ready => "ready",
            Self::Pending => "pending",
            Self::Stale => "stale",
        }
    }
}

/// User action made available from a field row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldActionClass {
    /// Reset the row to its effective default or winning source.
    Reset,
    /// Clear the row value.
    Clear,
    /// Open detail, policy, source, or validation explanation.
    OpenDetails,
    /// Reveal a masked value after friction.
    Reveal,
    /// Copy a safe representation.
    Copy,
    /// Revoke, disconnect, or clear a secret reference.
    Revoke,
}

/// One action rendered on a field row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldAction {
    /// Action class used by keyboard, help, and test projections.
    pub action_class: FieldActionClass,
    /// Exact visible label for the action.
    pub label: String,
}

/// One candidate source considered during precedence resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedenceCandidate {
    /// Source class for this candidate.
    pub source_class: SourcePrecedence,
    /// Controlled or product-visible label for this source.
    pub source_label: String,
    /// Redacted or safe value summary.
    pub value_summary: String,
    /// Rank used by the resolver for this field.
    pub precedence_rank: u32,
    /// Whether this candidate won the effective value.
    pub effective: bool,
    /// Explanation when this candidate did not win.
    pub superseded_by: Option<String>,
}

/// Validation result for one field row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Validation class currently displayed.
    pub validation_class: ValidationClass,
    /// Current validation state.
    pub state: ValidationState,
    /// Exact user-facing rule, result, or blocked reason.
    pub message: String,
    /// Whether this validation blocks final submission.
    pub blocks_submit: bool,
    /// Last known state preserved while a newer validation is pending.
    pub last_known_state: Option<ValidationState>,
    /// Last known message preserved while a newer validation is pending.
    pub last_known_message: Option<String>,
    /// Monotonic validation epoch for stale-result rejection.
    pub validation_epoch: u64,
    /// Monotonic target epoch the validation was computed against.
    pub target_epoch: u64,
    /// Field or target dependencies that invalidate this result when changed.
    pub dependent_inputs: Vec<String>,
    /// Whether target or dependency change automatically invalidates this result.
    pub invalidates_on_target_change: bool,
}

/// Storage and export truth for secret-bearing fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretFieldTruth {
    /// Storage mode disclosed to the user.
    pub storage_mode: SecretStorageMode,
    /// Reveal friction copy or mechanism.
    pub reveal_friction: String,
    /// Copy warning shown before exposing sensitive material.
    pub copy_warning: String,
    /// Clear, revoke, or disconnect action label.
    pub clear_or_revoke_action: String,
    /// Export behavior used by support and share flows.
    pub export_behavior: SecretExportBehavior,
}

/// Secret storage mode vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretStorageMode {
    /// Kept only for the current session.
    SessionOnly,
    /// Stored in the operating-system keychain.
    OsKeychain,
    /// Supplied by managed policy.
    PolicyProvided,
    /// Stored as an external vault reference.
    ExternalVaultReference,
    /// Stored as a delegated identity or provider handle.
    DelegatedIdentity,
}

/// Redaction behavior for secret fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretExportBehavior {
    /// Raw value is never exported; only the reference is exported.
    ReferenceOnly,
    /// Value is redacted by default in exports.
    RedactedByDefault,
    /// Secret is policy-owned and export omits raw material.
    PolicyOwnedOmitted,
}

/// Path basis vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathBasis {
    /// Relative to the current workspace root.
    WorkspaceRelative,
    /// Absolute path on the active host.
    Absolute,
}

/// Location basis vocabulary for path fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathLocationClass {
    /// Local host path.
    Local,
    /// Remote host path.
    Remote,
    /// Container filesystem path.
    Container,
    /// Workspace logical path.
    Workspace,
}

/// Truth required for path fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathFieldTruth {
    /// Whether the displayed path is relative or absolute.
    pub basis: PathBasis,
    /// Where the path is resolved.
    pub location_class: PathLocationClass,
    /// Safe displayed path value.
    pub displayed_path: String,
    /// Stable basis or target identity ref used for review/export.
    pub basis_ref: String,
}

/// Truth required for object-reference fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceFieldTruth {
    /// Human-readable display label.
    pub display_label: String,
    /// Stable ID path or object ref.
    pub stable_id_path: String,
}

/// Truth required for code-backed fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeBackedFieldTruth {
    /// Reviewable diff or preview path.
    pub diff_preview_ref: String,
    /// Whether comments are preserved.
    pub preserves_comments: bool,
    /// Whether unknown fields are preserved.
    pub preserves_unknown_fields: bool,
    /// Broad-impact disclosure shown before writes.
    pub broad_impact_disclosure: String,
}

/// One canonical field row rendered by stable forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldRowContract {
    /// Stable field id.
    pub field_id: String,
    /// Visible label, never placeholder-only.
    pub label: String,
    /// Redacted or safe current value summary.
    pub current_value_summary: String,
    /// Required, optional, or conditional state.
    pub requirement: RequirementState,
    /// Field kind used for additional disclosure requirements.
    pub field_kind: FieldKind,
    /// Winning source class for the current effective value.
    pub source_class: SourcePrecedence,
    /// Visible source label.
    pub source_label: String,
    /// Validation result shown beside the field.
    pub validation: ValidationResult,
    /// Apply timing shown beside the field.
    pub apply_timing: ApplyTiming,
    /// Reset, clear, details, reveal, copy, or revoke actions.
    pub actions: Vec<FieldAction>,
    /// Secret-specific truth when [`FieldKind::Secret`].
    pub secret_truth: Option<SecretFieldTruth>,
    /// Path-specific truth when [`FieldKind::Path`].
    pub path_truth: Option<PathFieldTruth>,
    /// Reference-specific truth when [`FieldKind::ObjectReference`].
    pub reference_truth: Option<ReferenceFieldTruth>,
    /// Code-backed truth when [`FieldKind::CodeBacked`].
    pub code_backed_truth: Option<CodeBackedFieldTruth>,
}

/// Source precedence audit for one field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcePrecedenceAudit {
    /// Field id this audit explains.
    pub field_id: String,
    /// Winning effective source.
    pub effective_source: SourcePrecedence,
    /// Redacted or safe effective value summary.
    pub effective_value_summary: String,
    /// Plain-language reason the source won.
    pub winning_reason: String,
    /// Ordered source candidates considered by the resolver.
    pub candidates: Vec<PrecedenceCandidate>,
}

/// Side-effect classes shown before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// Editor-only preference or visual state.
    EditorOnly,
    /// File write.
    FileWrite,
    /// Process spawn or task run.
    ProcessSpawn,
    /// Network call.
    NetworkCall,
    /// Remote provider or infrastructure mutation.
    RemoteMutation,
    /// Policy or entitlement change.
    PolicyChange,
}

/// Broad impact class for code-backed or generated changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildImpactClass {
    /// No write occurs.
    None,
    /// Single setting or local preference.
    NarrowLocal,
    /// Multiple workspace files or settings.
    WorkspaceWide,
    /// Remote, provider, or infrastructure impact.
    RemoteOrProvider,
    /// Policy-managed or entitlement impact.
    PolicyManaged,
}

/// Apply packet that describes timing, dirty state, review, checkpoint, and revert truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedApplyPacket {
    /// Form-level apply timing.
    pub apply_timing: ApplyTiming,
    /// Whether the form currently has unsaved or staged changes.
    pub dirty: bool,
    /// Dirty scopes shown in the UI.
    pub dirty_scopes: Vec<String>,
    /// Events across which dirty state remains durable.
    pub dirty_state_persisted_across: Vec<String>,
    /// Review sheet or preview ref when required.
    pub review_sheet_ref: Option<String>,
    /// Checkpoint ref when a reversible boundary exists.
    pub checkpoint_ref: Option<String>,
    /// Revert or rollback action label.
    pub revert_action_label: Option<String>,
    /// Exact target scope shown before commit.
    pub target_scope: String,
    /// Side effects disclosed before apply.
    pub side_effects: Vec<SideEffectClass>,
    /// Broad impact class used by release and support evidence.
    pub build_impact_class: BuildImpactClass,
    /// Exact final submit label.
    pub final_submit_label: String,
    /// Whether save-and-resume is available and disclosed.
    pub save_and_resume_disclosed: bool,
}

/// Multi-step flow state shown in wizard headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardStepState {
    /// One-based current step.
    pub current_step: u32,
    /// Total step count.
    pub total_steps: u32,
    /// Blocked prerequisite summaries with field anchors.
    pub blocked_prerequisites: Vec<String>,
    /// Whether save-and-resume posture is visible.
    pub save_and_resume_visible: bool,
    /// Exact final submit label.
    pub final_submit_label: String,
}

/// Unsupported or reduced-authority disclosure for restricted clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientLimitation {
    /// Affected field id.
    pub field_id: String,
    /// Unsupported or reduced-authority reason.
    pub reason: String,
    /// Whether the limitation is disclosed before the final step.
    pub disclosed_before_final_step: bool,
}

/// Accessibility review evidence for dense form surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityReview {
    /// Keyboard-only operation was reviewed.
    pub keyboard_operable: bool,
    /// Screen-reader labels and inline links were reviewed.
    pub screen_reader_operable: bool,
    /// IME composition does not break validation or focus.
    pub ime_safe: bool,
    /// RTL layout and narration were reviewed.
    pub rtl_safe: bool,
    /// Reduced-motion mode preserves state and focus return.
    pub reduced_motion_safe: bool,
    /// Focus return target after validation or staged-review navigation.
    pub focus_return_target: String,
}

/// Canonical packet consumed by stable form surfaces and evidence exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormTruthPacketRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Timestamp when the packet was minted.
    pub as_of: String,
    /// Surface family.
    pub surface_class: FormSurfaceClass,
    /// Client authority context.
    pub client_scope: ClientScope,
    /// Form title or legend.
    pub title: String,
    /// Affected scope summary.
    pub affected_scope: String,
    /// Canonical field rows.
    pub fields: Vec<FieldRowContract>,
    /// Per-field source precedence audits.
    pub precedence_audits: Vec<SourcePrecedenceAudit>,
    /// Apply timing, review, checkpoint, and revert truth.
    pub staged_apply: StagedApplyPacket,
    /// Wizard or step-flow state when applicable.
    pub wizard_state: Option<WizardStepState>,
    /// Reduced-authority disclosures for restricted clients.
    pub client_limitations: Vec<ClientLimitation>,
    /// Accessibility review evidence.
    pub accessibility_review: AccessibilityReview,
    /// Redaction-safe support export lines.
    pub support_export_lines: Vec<String>,
}

impl FormTruthPacketRecord {
    /// Returns redaction-safe support export lines for this packet.
    pub fn support_export_lines(&self) -> Vec<String> {
        self.support_export_lines.clone()
    }
}

/// One validation defect returned by [`validate_form_truth_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormTruthValidationError {
    /// Packet id where the defect was found.
    pub packet_id: String,
    /// Field id when the defect is field-scoped.
    pub field_id: Option<String>,
    /// Human-readable defect summary for test and release evidence.
    pub message: String,
}

impl FormTruthValidationError {
    fn packet(packet: &FormTruthPacketRecord, message: impl Into<String>) -> Self {
        Self {
            packet_id: packet.packet_id.clone(),
            field_id: None,
            message: message.into(),
        }
    }

    fn field(packet: &FormTruthPacketRecord, field_id: &str, message: impl Into<String>) -> Self {
        Self {
            packet_id: packet.packet_id.clone(),
            field_id: Some(field_id.to_owned()),
            message: message.into(),
        }
    }
}

/// Validates one form truth packet against the stable honesty invariants.
pub fn validate_form_truth_packet(
    packet: &FormTruthPacketRecord,
) -> Result<(), Vec<FormTruthValidationError>> {
    let mut errors = Vec::new();
    if packet.record_kind != FORM_TRUTH_RECORD_KIND {
        errors.push(FormTruthValidationError::packet(
            packet,
            "record_kind mismatch",
        ));
    }
    if packet.schema_version != FORM_TRUTH_SCHEMA_VERSION {
        errors.push(FormTruthValidationError::packet(
            packet,
            "schema_version mismatch",
        ));
    }
    if packet.shared_contract_ref != FORM_TRUTH_SHARED_CONTRACT_REF {
        errors.push(FormTruthValidationError::packet(
            packet,
            "shared_contract_ref mismatch",
        ));
    }
    if packet.title.trim().is_empty() || packet.affected_scope.trim().is_empty() {
        errors.push(FormTruthValidationError::packet(
            packet,
            "title and affected_scope must be visible",
        ));
    }
    validate_fields(packet, &mut errors);
    validate_precedence(packet, &mut errors);
    validate_apply_packet(packet, &mut errors);
    validate_client_and_accessibility(packet, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_fields(packet: &FormTruthPacketRecord, errors: &mut Vec<FormTruthValidationError>) {
    let mut seen = BTreeSet::new();
    for field in &packet.fields {
        if !seen.insert(field.field_id.clone()) {
            errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "duplicate field id",
            ));
        }
        if field.label.trim().is_empty()
            || field.current_value_summary.trim().is_empty()
            || field.source_label.trim().is_empty()
        {
            errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "field row must carry label, current value, and source label",
            ));
        }
        if field.actions.is_empty() {
            errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "field row must expose reset, clear, details, or equivalent action",
            ));
        }
        validate_validation_result(packet, field, errors);
        validate_kind_specific_truth(packet, field, errors);
    }
}

fn validate_validation_result(
    packet: &FormTruthPacketRecord,
    field: &FieldRowContract,
    errors: &mut Vec<FormTruthValidationError>,
) {
    let validation = &field.validation;
    if validation.message.trim().is_empty() {
        errors.push(FormTruthValidationError::field(
            packet,
            &field.field_id,
            "validation message must name the rule, result, or blocked reason",
        ));
    }
    if validation.state == ValidationState::Pending
        && (validation.last_known_state.is_none() || validation.last_known_message.is_none())
    {
        errors.push(FormTruthValidationError::field(
            packet,
            &field.field_id,
            "pending validation must preserve last-known result",
        ));
    }
    if (!validation.dependent_inputs.is_empty() || validation.state == ValidationState::Stale)
        && !validation.invalidates_on_target_change
    {
        errors.push(FormTruthValidationError::field(
            packet,
            &field.field_id,
            "dependent or stale validation must invalidate on target change",
        ));
    }
    if validation.state == ValidationState::Blocked && !validation.blocks_submit {
        errors.push(FormTruthValidationError::field(
            packet,
            &field.field_id,
            "blocked validation must block submit",
        ));
    }
}

fn validate_kind_specific_truth(
    packet: &FormTruthPacketRecord,
    field: &FieldRowContract,
    errors: &mut Vec<FormTruthValidationError>,
) {
    match field.field_kind {
        FieldKind::Plain => {}
        FieldKind::Secret => match &field.secret_truth {
            Some(secret) => {
                if secret.reveal_friction.trim().is_empty()
                    || secret.copy_warning.trim().is_empty()
                    || secret.clear_or_revoke_action.trim().is_empty()
                {
                    errors.push(FormTruthValidationError::field(
                        packet,
                        &field.field_id,
                        "secret fields must disclose reveal friction, copy warning, and clear/revoke action",
                    ));
                }
            }
            None => errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "secret field missing storage/export truth",
            )),
        },
        FieldKind::Path => match &field.path_truth {
            Some(path) => {
                if path.displayed_path.trim().is_empty() || path.basis_ref.trim().is_empty() {
                    errors.push(FormTruthValidationError::field(
                        packet,
                        &field.field_id,
                        "path fields must disclose displayed path and stable basis ref",
                    ));
                }
            }
            None => errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "path field missing basis truth",
            )),
        },
        FieldKind::ObjectReference => match &field.reference_truth {
            Some(reference) => {
                if reference.display_label.trim().is_empty()
                    || reference.stable_id_path.trim().is_empty()
                {
                    errors.push(FormTruthValidationError::field(
                        packet,
                        &field.field_id,
                        "reference fields must disclose display label and stable id path",
                    ));
                }
            }
            None => errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "object-reference field missing identity truth",
            )),
        },
        FieldKind::CodeBacked => match &field.code_backed_truth {
            Some(code) => {
                if code.diff_preview_ref.trim().is_empty()
                    || code.broad_impact_disclosure.trim().is_empty()
                    || !code.preserves_comments
                    || !code.preserves_unknown_fields
                {
                    errors.push(FormTruthValidationError::field(
                        packet,
                        &field.field_id,
                        "code-backed fields must disclose preview and preservation expectations",
                    ));
                }
            }
            None => errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "code-backed field missing preview truth",
            )),
        },
    }
}

fn validate_precedence(packet: &FormTruthPacketRecord, errors: &mut Vec<FormTruthValidationError>) {
    for field in &packet.fields {
        let Some(audit) = packet
            .precedence_audits
            .iter()
            .find(|audit| audit.field_id == field.field_id)
        else {
            errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "field missing source precedence audit",
            ));
            continue;
        };
        let winners: Vec<_> = audit
            .candidates
            .iter()
            .filter(|candidate| candidate.effective)
            .collect();
        if winners.len() != 1 || audit.effective_source != field.source_class {
            errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "source audit must identify exactly one winner matching the field source",
            ));
        }
        if audit.winning_reason.trim().is_empty() || audit.effective_value_summary.trim().is_empty()
        {
            errors.push(FormTruthValidationError::field(
                packet,
                &field.field_id,
                "source audit must explain why the effective value won",
            ));
        }
    }
}

fn validate_apply_packet(
    packet: &FormTruthPacketRecord,
    errors: &mut Vec<FormTruthValidationError>,
) {
    let apply = &packet.staged_apply;
    if apply.final_submit_label.trim().is_empty()
        || matches!(
            apply.final_submit_label.trim(),
            "Continue" | "Next" | "Submit"
        )
    {
        errors.push(FormTruthValidationError::packet(
            packet,
            "final submit label must name the exact resulting action",
        ));
    }
    if apply.target_scope.trim().is_empty() {
        errors.push(FormTruthValidationError::packet(
            packet,
            "apply packet must disclose target scope",
        ));
    }
    match apply.apply_timing {
        ApplyTiming::Immediate => {
            if apply.side_effects.is_empty() || apply.revert_action_label.is_none() {
                errors.push(FormTruthValidationError::packet(
                    packet,
                    "immediate forms must disclose side effects and undo/revert path",
                ));
            }
        }
        ApplyTiming::Staged => {
            let required = ["tab_change", "reconnect", "client_handoff"];
            if !apply.dirty
                || apply.checkpoint_ref.is_none()
                || apply.revert_action_label.is_none()
                || !apply.save_and_resume_disclosed
                || !required.iter().all(|required| {
                    apply
                        .dirty_state_persisted_across
                        .iter()
                        .any(|v| v == required)
                })
            {
                errors.push(FormTruthValidationError::packet(
                    packet,
                    "staged forms must preserve dirty state, checkpoint, revert, and save/resume",
                ));
            }
        }
        ApplyTiming::PreviewFirst => {
            if apply.review_sheet_ref.is_none()
                || apply.checkpoint_ref.is_none()
                || apply.side_effects.is_empty()
            {
                errors.push(FormTruthValidationError::packet(
                    packet,
                    "preview-first forms must disclose review sheet, checkpoint, and side effects",
                ));
            }
        }
        ApplyTiming::PolicyLocked => {
            if apply.review_sheet_ref.is_none()
                || apply.build_impact_class != BuildImpactClass::PolicyManaged
            {
                errors.push(FormTruthValidationError::packet(
                    packet,
                    "policy-locked forms must provide inspectable policy details",
                ));
            }
        }
    }
    if let Some(wizard) = &packet.wizard_state {
        if wizard.current_step == 0
            || wizard.current_step > wizard.total_steps
            || !wizard.save_and_resume_visible
            || wizard.final_submit_label != apply.final_submit_label
        {
            errors.push(FormTruthValidationError::packet(
                packet,
                "wizard state must preserve step, save/resume, and final submit semantics",
            ));
        }
    }
}

fn validate_client_and_accessibility(
    packet: &FormTruthPacketRecord,
    errors: &mut Vec<FormTruthValidationError>,
) {
    if packet.client_scope.requires_limitation_disclosure()
        && (packet.client_limitations.is_empty()
            || packet
                .client_limitations
                .iter()
                .any(|limitation| !limitation.disclosed_before_final_step))
    {
        errors.push(FormTruthValidationError::packet(
            packet,
            "restricted clients must disclose unsupported fields before final step",
        ));
    }
    let a11y = &packet.accessibility_review;
    if !a11y.keyboard_operable
        || !a11y.screen_reader_operable
        || !a11y.ime_safe
        || !a11y.rtl_safe
        || !a11y.reduced_motion_safe
        || a11y.focus_return_target.trim().is_empty()
    {
        errors.push(FormTruthValidationError::packet(
            packet,
            "accessibility review must cover keyboard, screen reader, IME, RTL, reduced motion, and focus return",
        ));
    }
}
