//! Cross-surface representation-labeled copy and export validation.
//!
//! This module consumes the shell-wide interaction-safety copy/export
//! vocabulary and projects it onto the protected alpha surfaces where raw,
//! plain-text, rendered, and context-bearing transfers can otherwise drift:
//! diff, review, search, and package/install review.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version for representation copy/export alpha packets.
pub const REPRESENTATION_COPY_EXPORT_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema version for the shell interaction-safety contract this module consumes.
pub const INTERACTION_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RepresentationCopyExportAlphaPacket`].
pub const REPRESENTATION_COPY_EXPORT_ALPHA_PACKET_RECORD_KIND: &str =
    "representation_copy_export_alpha_packet";

/// Stable record-kind tag for [`RepresentationCopyExportValidationReport`].
pub const REPRESENTATION_COPY_EXPORT_VALIDATION_REPORT_RECORD_KIND: &str =
    "representation_copy_export_validation_report";

/// Stable record-kind tag for [`InteractionSafetyCopyExportRecord`].
pub const INTERACTION_SAFETY_COPY_EXPORT_RECORD_KIND: &str = "copy_export_representation_record";

/// Protected surfaces covered by the cross-surface copy/export alpha proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedCopySurfaceKind {
    /// Local Git diff rows and hunks.
    Diff,
    /// Review anchors, review packets, and review-thread rows.
    Review,
    /// Search result rows, snippets, and query snapshots.
    Search,
    /// Package or install-review rows.
    Package,
}

impl ProtectedCopySurfaceKind {
    /// Stable token used in fixtures and packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diff => "diff",
            Self::Review => "review",
            Self::Search => "search",
            Self::Package => "package",
        }
    }

    /// Shell interaction-safety surface class consumed by this projection.
    pub const fn interaction_surface_class(self) -> &'static str {
        match self {
            Self::Diff | Self::Review => "review_and_diff_canvas",
            Self::Search => "palette_and_search_canvas",
            Self::Package => "install_update_attach_canvas",
        }
    }
}

/// All surfaces required by the alpha proof.
pub const PROTECTED_COPY_EXPORT_SURFACES: [ProtectedCopySurfaceKind; 4] = [
    ProtectedCopySurfaceKind::Diff,
    ProtectedCopySurfaceKind::Review,
    ProtectedCopySurfaceKind::Search,
    ProtectedCopySurfaceKind::Package,
];

/// Copy or export action kind from the interaction-safety schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyExportActionKind {
    /// A clipboard transfer.
    Copy,
    /// A durable export or support/review artifact transfer.
    Export,
}

impl CopyExportActionKind {
    /// Stable token used by `copy_export_representation_record`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Export => "export",
        }
    }
}

/// Representation class from the interaction-safety contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionRepresentationClass {
    /// Exact source bytes or exact source text.
    Raw,
    /// Rendered representation of the current surface.
    Rendered,
    /// Source representation with metacharacters escaped.
    Escaped,
    /// Static sanitized representation.
    Sanitized,
    /// Representation confined to a sandbox boundary.
    Sandboxed,
    /// Generated content with citation anchors where authoritative material is quoted.
    Generated,
    /// Metadata envelope with raw body withheld.
    BlockedMetadataOnly,
}

impl InteractionRepresentationClass {
    /// Stable token used by `copy_export_representation_record`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Rendered => "rendered",
            Self::Escaped => "escaped",
            Self::Sanitized => "sanitized",
            Self::Sandboxed => "sandboxed",
            Self::Generated => "generated",
            Self::BlockedMetadataOnly => "blocked_metadata_only",
        }
    }
}

/// Structured label class for copy/export actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyExportLabelClass {
    /// Label explicitly says raw copy.
    CopyRaw,
    /// Label explicitly says plain text copy.
    CopyPlainText,
    /// Label explicitly says escaped copy.
    CopyEscaped,
    /// Label explicitly says rendered copy.
    CopyRendered,
    /// Label explicitly says context-bearing copy.
    CopyWithContext,
    /// Label explicitly says the clipboard receives an export packet.
    CopyExportPacket,
    /// Label explicitly says sanitized snapshot export.
    ExportSanitizedSnapshot,
    /// Label explicitly says metadata-only export.
    ExportMetadataOnly,
}

impl CopyExportLabelClass {
    /// Stable token used in fixtures and validation reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyRaw => "copy_raw",
            Self::CopyPlainText => "copy_plain_text",
            Self::CopyEscaped => "copy_escaped",
            Self::CopyRendered => "copy_rendered",
            Self::CopyWithContext => "copy_with_context",
            Self::CopyExportPacket => "copy_export_packet",
            Self::ExportSanitizedSnapshot => "export_sanitized_snapshot",
            Self::ExportMetadataOnly => "export_metadata_only",
        }
    }
}

/// Local transfer mode used to validate plain text versus richer copies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyPayloadMode {
    /// Plain text with surface chrome stripped.
    PlainText,
    /// Exact raw source or stable raw identifier.
    Raw,
    /// Rendered representation.
    Rendered,
    /// Context-bearing copy that includes provenance, target, or hunk/query context.
    WithContext,
    /// Export packet copied to the clipboard.
    ExportPacket,
    /// Sanitized export body.
    SanitizedSnapshot,
    /// Metadata-only envelope.
    MetadataOnly,
}

impl CopyPayloadMode {
    /// Stable token used in fixtures and packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::Raw => "raw",
            Self::Rendered => "rendered",
            Self::WithContext => "with_context",
            Self::ExportPacket => "export_packet",
            Self::SanitizedSnapshot => "sanitized_snapshot",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Policy context copied into interaction-safety records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionPolicyContext {
    /// Policy epoch in force when the record was minted.
    pub policy_epoch: String,
    /// Workspace trust state at mint time.
    pub trust_state: String,
    /// Optional execution-context id when the transfer originated in a live execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
}

/// Interaction-safety copy/export record emitted for one transfer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionSafetyCopyExportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the interaction-safety contract.
    pub interaction_safety_schema_version: u32,
    /// Stable copy/export id.
    pub copy_export_id: String,
    /// Copy or export action kind.
    pub action_kind: String,
    /// Representation class from the interaction-safety contract.
    pub representation_class: String,
    /// Protected shell surface class that emitted the transfer.
    pub source_surface_class: String,
    /// Opaque target identity for the source row or object.
    pub source_target_identity_ref: String,
    /// Citation anchors backing generated or docs/help transfers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub citation_anchor_refs: Vec<String>,
    /// Redaction class applied before bytes leave the product.
    pub redaction_class: String,
    /// Policy context at mint time.
    pub policy_context: InteractionPolicyContext,
    /// Timestamp supplied by the caller or fixture.
    pub minted_at: String,
}

/// Input action supplied by a protected fixture or first consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportActionInput {
    /// Stable action id on the source surface.
    pub action_id: String,
    /// Copy or export action kind.
    pub action_kind: CopyExportActionKind,
    /// Structured label class, used instead of scraping visible text.
    pub label_class: CopyExportLabelClass,
    /// Human-facing label shown by the source surface.
    pub visible_label: String,
    /// Interaction-safety representation class.
    pub representation_class: InteractionRepresentationClass,
    /// Local payload mode.
    pub payload_mode: CopyPayloadMode,
    /// True when this is the default transfer action for the surface.
    pub default_for_surface: bool,
    /// True when this action includes path, hunk, query, review, package, or support context.
    pub includes_context: bool,
    /// True when this action can move sensitive values to the clipboard/export.
    pub carries_sensitive_value: bool,
    /// True when a preview or label-first sheet is required before clipboard commit.
    pub preview_required_before_clipboard: bool,
    /// True when trust-class labels are retained on the transfer.
    pub preserves_trust_class: bool,
    /// True when provenance refs are retained on the transfer.
    pub preserves_provenance: bool,
    /// True when the representation label is retained on the transfer.
    pub preserves_representation_label: bool,
    /// Optional cross-surface reconciliation group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_group_ref: Option<String>,
    /// Citation anchors for generated or docs/help transfer rows.
    #[serde(default)]
    pub citation_anchor_refs: Vec<String>,
    /// Redaction class applied to the transfer.
    pub redaction_class: String,
}

/// Surface input supplied by a protected fixture or first consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportSurfaceInput {
    /// Protected surface kind.
    pub surface: ProtectedCopySurfaceKind,
    /// Stable surface ref.
    pub surface_ref: String,
    /// Opaque target identity ref for the surface.
    pub source_target_identity_ref: String,
    /// Trust class label visible on this surface.
    pub trust_class: String,
    /// Target boundary carried into copy/export validation.
    pub target_boundary_ref: String,
    /// Opaque provenance refs that must survive transfer.
    pub provenance_refs: Vec<String>,
    /// Sensitive value classes present on this surface.
    #[serde(default)]
    pub sensitive_value_classes: Vec<String>,
    /// Inspect or reveal paths offered before risky transfer.
    pub inspect_or_reveal_paths: Vec<String>,
    /// Reopen, history, or recovery affordances for this surface.
    pub recovery_affordances: Vec<String>,
    /// Copy/export actions offered by the surface.
    pub actions: Vec<RepresentationCopyExportActionInput>,
}

/// Case input consumed by the alpha projection and CLI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportCase {
    /// Stable case id.
    pub case_id: String,
    /// Timestamp used for deterministic fixture output.
    pub minted_at: String,
    /// Source contract refs consumed by this proof.
    pub source_contract_refs: Vec<String>,
    /// Policy context copied into each interaction-safety record.
    pub policy_context: InteractionPolicyContext,
    /// Protected surface inputs.
    pub surfaces: Vec<RepresentationCopyExportSurfaceInput>,
}

/// Projected action after interaction-safety record minting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportActionProjection {
    /// Stable action id on the source surface.
    pub action_id: String,
    /// Copy or export action kind.
    pub action_kind: String,
    /// Structured label class.
    pub label_class: String,
    /// Human-facing label shown by the source surface.
    pub visible_label: String,
    /// Interaction-safety representation class.
    pub representation_class: String,
    /// Local payload mode.
    pub payload_mode: String,
    /// True when this is the default transfer action for the surface.
    pub default_for_surface: bool,
    /// True when this action includes target or provenance context.
    pub includes_context: bool,
    /// True when this action can move sensitive values.
    pub carries_sensitive_value: bool,
    /// True when preview or label-first review is required before clipboard commit.
    pub preview_required_before_clipboard: bool,
    /// True when trust class survives the transfer.
    pub preserves_trust_class: bool,
    /// True when provenance survives the transfer.
    pub preserves_provenance: bool,
    /// True when representation label survives the transfer.
    pub preserves_representation_label: bool,
    /// Optional cross-surface reconciliation group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_group_ref: Option<String>,
    /// Interaction-safety copy/export record.
    pub interaction_copy_export_record: InteractionSafetyCopyExportRecord,
}

/// Projected surface after interaction-safety record minting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportSurfaceProjection {
    /// Protected surface kind token.
    pub surface: String,
    /// Stable surface ref.
    pub surface_ref: String,
    /// Shell interaction-safety surface class.
    pub interaction_surface_class: String,
    /// Opaque target identity ref for the surface.
    pub source_target_identity_ref: String,
    /// Trust class label visible on this surface.
    pub trust_class: String,
    /// Target boundary carried into copy/export validation.
    pub target_boundary_ref: String,
    /// Opaque provenance refs that must survive transfer.
    pub provenance_refs: Vec<String>,
    /// Sensitive value classes present on this surface.
    pub sensitive_value_classes: Vec<String>,
    /// Inspect or reveal paths offered before risky transfer.
    pub inspect_or_reveal_paths: Vec<String>,
    /// Reopen, history, or recovery affordances for this surface.
    pub recovery_affordances: Vec<String>,
    /// Copy/export actions offered by the surface.
    pub actions: Vec<RepresentationCopyExportActionProjection>,
}

/// Cross-surface packet consumed by support, review, and validation surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha packet.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Source contract refs consumed by this proof.
    pub source_contract_refs: Vec<String>,
    /// Protected surface projections.
    pub surfaces: Vec<RepresentationCopyExportSurfaceProjection>,
    /// Timestamp used for deterministic fixture output.
    pub minted_at: String,
}

impl RepresentationCopyExportAlphaPacket {
    /// Builds a packet from a fixture or first-consumer input.
    pub fn from_case(case: RepresentationCopyExportCase) -> Self {
        let surfaces = case
            .surfaces
            .into_iter()
            .map(|surface| project_surface(surface, &case.policy_context, &case.minted_at))
            .collect();
        Self {
            record_kind: REPRESENTATION_COPY_EXPORT_ALPHA_PACKET_RECORD_KIND.to_string(),
            schema_version: REPRESENTATION_COPY_EXPORT_ALPHA_SCHEMA_VERSION,
            case_id: case.case_id,
            source_contract_refs: case.source_contract_refs,
            surfaces,
            minted_at: case.minted_at,
        }
    }

    /// Returns true when diff, review, search, and package surfaces are present exactly once.
    pub fn covers_protected_surfaces(&self) -> bool {
        let present = self
            .surfaces
            .iter()
            .map(|surface| surface.surface.as_str())
            .collect::<BTreeSet<_>>();
        PROTECTED_COPY_EXPORT_SURFACES
            .iter()
            .all(|surface| present.contains(surface.as_str()))
            && present.len() == PROTECTED_COPY_EXPORT_SURFACES.len()
    }

    /// Validates the packet against the cross-surface alpha invariants.
    pub fn validate(&self) -> RepresentationCopyExportValidationReport {
        let mut violations = Vec::new();

        validate_surface_coverage(self, &mut violations);
        for surface in &self.surfaces {
            validate_surface(surface, &mut violations);
        }
        validate_cross_surface_reconciliation(self, &mut violations);

        let reconciled_groups = reconciled_groups(self);
        let status = if violations.is_empty() {
            "passed"
        } else {
            "failed"
        };

        RepresentationCopyExportValidationReport {
            record_kind: REPRESENTATION_COPY_EXPORT_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: REPRESENTATION_COPY_EXPORT_ALPHA_SCHEMA_VERSION,
            case_id: self.case_id.clone(),
            status: status.to_string(),
            violations,
            reconciled_groups,
            validated_surface_count: self.surfaces.len(),
        }
    }
}

/// One validation violation surfaced by the cross-surface proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportViolation {
    /// Stable violation id.
    pub violation_id: String,
    /// Surface token, if the violation is surface-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Action id, if the violation is action-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// Reviewable summary.
    pub summary: String,
}

/// Validation report emitted by [`RepresentationCopyExportAlphaPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCopyExportValidationReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha report.
    pub schema_version: u32,
    /// Case id validated by this report.
    pub case_id: String,
    /// `passed` when no violations were found.
    pub status: String,
    /// Violations found during validation.
    pub violations: Vec<RepresentationCopyExportViolation>,
    /// Cross-surface reconciliation groups proven by structure.
    pub reconciled_groups: Vec<String>,
    /// Count of protected surface projections validated.
    pub validated_surface_count: usize,
}

impl RepresentationCopyExportValidationReport {
    /// Returns true when the report contains no violations.
    pub fn passed(&self) -> bool {
        self.violations.is_empty() && self.status == "passed"
    }

    /// Returns the violation ids present in this report.
    pub fn violation_ids(&self) -> BTreeSet<&str> {
        self.violations
            .iter()
            .map(|violation| violation.violation_id.as_str())
            .collect()
    }
}

fn project_surface(
    surface: RepresentationCopyExportSurfaceInput,
    policy_context: &InteractionPolicyContext,
    minted_at: &str,
) -> RepresentationCopyExportSurfaceProjection {
    let interaction_surface_class = surface.surface.interaction_surface_class().to_string();
    let actions = surface
        .actions
        .into_iter()
        .map(|action| {
            let copy_export_id = format!(
                "copy-export:{}:{}",
                sanitize_id(&surface.surface_ref),
                sanitize_id(&action.action_id)
            );
            let interaction_copy_export_record = InteractionSafetyCopyExportRecord {
                record_kind: INTERACTION_SAFETY_COPY_EXPORT_RECORD_KIND.to_string(),
                interaction_safety_schema_version: INTERACTION_SAFETY_SCHEMA_VERSION,
                copy_export_id,
                action_kind: action.action_kind.as_str().to_string(),
                representation_class: action.representation_class.as_str().to_string(),
                source_surface_class: interaction_surface_class.clone(),
                source_target_identity_ref: surface.source_target_identity_ref.clone(),
                citation_anchor_refs: action.citation_anchor_refs.clone(),
                redaction_class: action.redaction_class.clone(),
                policy_context: policy_context.clone(),
                minted_at: minted_at.to_string(),
            };

            RepresentationCopyExportActionProjection {
                action_id: action.action_id,
                action_kind: action.action_kind.as_str().to_string(),
                label_class: action.label_class.as_str().to_string(),
                visible_label: action.visible_label,
                representation_class: action.representation_class.as_str().to_string(),
                payload_mode: action.payload_mode.as_str().to_string(),
                default_for_surface: action.default_for_surface,
                includes_context: action.includes_context,
                carries_sensitive_value: action.carries_sensitive_value,
                preview_required_before_clipboard: action.preview_required_before_clipboard,
                preserves_trust_class: action.preserves_trust_class,
                preserves_provenance: action.preserves_provenance,
                preserves_representation_label: action.preserves_representation_label,
                reconciliation_group_ref: action.reconciliation_group_ref,
                interaction_copy_export_record,
            }
        })
        .collect();

    RepresentationCopyExportSurfaceProjection {
        surface: surface.surface.as_str().to_string(),
        surface_ref: surface.surface_ref,
        interaction_surface_class,
        source_target_identity_ref: surface.source_target_identity_ref,
        trust_class: surface.trust_class,
        target_boundary_ref: surface.target_boundary_ref,
        provenance_refs: surface.provenance_refs,
        sensitive_value_classes: surface.sensitive_value_classes,
        inspect_or_reveal_paths: surface.inspect_or_reveal_paths,
        recovery_affordances: surface.recovery_affordances,
        actions,
    }
}

fn validate_surface_coverage(
    packet: &RepresentationCopyExportAlphaPacket,
    violations: &mut Vec<RepresentationCopyExportViolation>,
) {
    let counts =
        packet
            .surfaces
            .iter()
            .fold(BTreeMap::<&str, usize>::new(), |mut counts, surface| {
                *counts.entry(surface.surface.as_str()).or_default() += 1;
                counts
            });

    for required in PROTECTED_COPY_EXPORT_SURFACES {
        if !counts.contains_key(required.as_str()) {
            violations.push(violation(
                "protected_surface_missing",
                Some(required.as_str()),
                None,
                "protected copy/export surface is missing",
            ));
        }
    }

    for (surface, count) in counts {
        if count > 1 {
            violations.push(violation(
                "protected_surface_duplicated",
                Some(surface),
                None,
                "protected copy/export surface appears more than once",
            ));
        }
    }
}

fn validate_surface(
    surface: &RepresentationCopyExportSurfaceProjection,
    violations: &mut Vec<RepresentationCopyExportViolation>,
) {
    if surface.target_boundary_ref.trim().is_empty() {
        violations.push(violation(
            "target_boundary_missing",
            Some(&surface.surface),
            None,
            "surface did not name a target boundary",
        ));
    }
    if surface.provenance_refs.is_empty() {
        violations.push(violation(
            "provenance_missing",
            Some(&surface.surface),
            None,
            "surface did not carry provenance refs",
        ));
    }
    if surface.recovery_affordances.is_empty() {
        violations.push(violation(
            "recovery_affordance_missing",
            Some(&surface.surface),
            None,
            "surface did not expose a reopen/history/recovery affordance",
        ));
    }
    if surface.actions.is_empty() {
        violations.push(violation(
            "copy_export_action_missing",
            Some(&surface.surface),
            None,
            "surface did not expose any copy/export actions",
        ));
        return;
    }

    let defaults = surface
        .actions
        .iter()
        .filter(|action| action.default_for_surface)
        .collect::<Vec<_>>();
    if defaults.len() != 1 {
        violations.push(violation(
            "default_copy_action_count_invalid",
            Some(&surface.surface),
            None,
            "surface must expose exactly one default copy action",
        ));
    }

    for action in &surface.actions {
        validate_action(surface, action, violations);
    }

    let risky_or_ambiguous = surface_has_unsafe_or_ambiguous_transfer(surface);
    if risky_or_ambiguous && surface.inspect_or_reveal_paths.is_empty() {
        violations.push(violation(
            "inspect_or_reveal_path_missing",
            Some(&surface.surface),
            None,
            "unsafe or ambiguous surface did not expose inspect or reveal paths before transfer",
        ));
    }
}

fn validate_action(
    surface: &RepresentationCopyExportSurfaceProjection,
    action: &RepresentationCopyExportActionProjection,
    violations: &mut Vec<RepresentationCopyExportViolation>,
) {
    if action.default_for_surface && !default_action_is_raw_or_plain_safe(action) {
        violations.push(violation(
            "default_copy_not_raw_or_plain_safe",
            Some(&surface.surface),
            Some(&action.action_id),
            "default copy action is not a raw or plain-text safe representation",
        ));
    }

    if action_is_richer_or_context_bearing(action) && !action_has_explicit_richer_label(action) {
        violations.push(violation(
            "richer_copy_label_missing",
            Some(&surface.surface),
            Some(&action.action_id),
            "rendered, context-bearing, or export-packet transfer lacks an explicit label class",
        ));
    }

    if action.carries_sensitive_value && !action.preview_required_before_clipboard {
        violations.push(violation(
            "sensitive_copy_preview_missing",
            Some(&surface.surface),
            Some(&action.action_id),
            "sensitive copy/export action can reach the clipboard without a preview",
        ));
    }

    if !action.preserves_trust_class {
        violations.push(violation(
            "trust_class_lost_on_transfer",
            Some(&surface.surface),
            Some(&action.action_id),
            "copy/export action does not preserve trust class",
        ));
    }
    if !action.preserves_provenance {
        violations.push(violation(
            "provenance_lost_on_transfer",
            Some(&surface.surface),
            Some(&action.action_id),
            "copy/export action does not preserve provenance",
        ));
    }
    if !action.preserves_representation_label {
        violations.push(violation(
            "representation_label_lost_on_transfer",
            Some(&surface.surface),
            Some(&action.action_id),
            "copy/export action does not preserve representation label",
        ));
    }

    let record = &action.interaction_copy_export_record;
    if record.record_kind != INTERACTION_SAFETY_COPY_EXPORT_RECORD_KIND
        || record.interaction_safety_schema_version != INTERACTION_SAFETY_SCHEMA_VERSION
        || record.source_surface_class != surface.interaction_surface_class
        || record.source_target_identity_ref != surface.source_target_identity_ref
        || record.representation_class != action.representation_class
        || record.action_kind != action.action_kind
    {
        violations.push(violation(
            "interaction_copy_export_record_mismatch",
            Some(&surface.surface),
            Some(&action.action_id),
            "action does not reconcile with its interaction-safety copy/export record",
        ));
    }
}

fn validate_cross_surface_reconciliation(
    packet: &RepresentationCopyExportAlphaPacket,
    violations: &mut Vec<RepresentationCopyExportViolation>,
) {
    let groups = reconciliation_groups(packet);
    let mut valid_group_count = 0usize;

    for (group, entries) in groups {
        let unique_surfaces = entries
            .iter()
            .map(|entry| entry.surface.surface.as_str())
            .collect::<BTreeSet<_>>();
        if unique_surfaces.len() < 2 {
            continue;
        }

        let mut group_valid = true;
        for entry in &entries {
            if entry.surface.target_boundary_ref.trim().is_empty()
                || entry.surface.recovery_affordances.is_empty()
                || !entry.action.preserves_trust_class
                || !entry.action.preserves_provenance
                || !entry.action.preserves_representation_label
            {
                group_valid = false;
            }
        }

        if group_valid {
            valid_group_count += 1;
        } else {
            violations.push(violation(
                "cross_surface_reconciliation_incomplete",
                None,
                Some(&group),
                "reconciliation group did not preserve representation, target boundary, provenance, and recovery across surfaces",
            ));
        }
    }

    if valid_group_count == 0 {
        violations.push(violation(
            "cross_surface_reconciliation_missing",
            None,
            None,
            "no reconciliation group covered at least two protected surfaces",
        ));
    }
}

struct ReconciliationEntry<'a> {
    surface: &'a RepresentationCopyExportSurfaceProjection,
    action: &'a RepresentationCopyExportActionProjection,
}

fn reconciliation_groups<'a>(
    packet: &'a RepresentationCopyExportAlphaPacket,
) -> BTreeMap<String, Vec<ReconciliationEntry<'a>>> {
    let mut groups: BTreeMap<String, Vec<ReconciliationEntry<'a>>> = BTreeMap::new();
    for surface in &packet.surfaces {
        for action in &surface.actions {
            if let Some(group) = &action.reconciliation_group_ref {
                groups
                    .entry(group.clone())
                    .or_default()
                    .push(ReconciliationEntry { surface, action });
            }
        }
    }
    groups
}

fn reconciled_groups(packet: &RepresentationCopyExportAlphaPacket) -> Vec<String> {
    reconciliation_groups(packet)
        .into_iter()
        .filter_map(|(group, entries)| {
            let unique_surfaces = entries
                .iter()
                .map(|entry| entry.surface.surface.as_str())
                .collect::<BTreeSet<_>>();
            (unique_surfaces.len() >= 2).then_some(group)
        })
        .collect()
}

fn surface_has_unsafe_or_ambiguous_transfer(
    surface: &RepresentationCopyExportSurfaceProjection,
) -> bool {
    !surface.sensitive_value_classes.is_empty()
        || surface.actions.iter().any(|action| {
            action.carries_sensitive_value || action_is_richer_or_context_bearing(action)
        })
}

fn default_action_is_raw_or_plain_safe(action: &RepresentationCopyExportActionProjection) -> bool {
    let representation = action.representation_class.as_str();
    let payload = action.payload_mode.as_str();
    let representation_safe = representation == InteractionRepresentationClass::Raw.as_str()
        || representation == InteractionRepresentationClass::Escaped.as_str();
    let payload_safe =
        payload == CopyPayloadMode::PlainText.as_str() || payload == CopyPayloadMode::Raw.as_str();
    representation_safe && payload_safe && !action.includes_context
}

fn action_is_richer_or_context_bearing(action: &RepresentationCopyExportActionProjection) -> bool {
    action.includes_context
        || matches!(
            action.payload_mode.as_str(),
            "rendered" | "with_context" | "export_packet" | "sanitized_snapshot" | "metadata_only"
        )
        || matches!(
            action.representation_class.as_str(),
            "rendered" | "sanitized" | "sandboxed" | "generated" | "blocked_metadata_only"
        )
}

fn action_has_explicit_richer_label(action: &RepresentationCopyExportActionProjection) -> bool {
    matches!(
        action.label_class.as_str(),
        "copy_rendered"
            | "copy_with_context"
            | "copy_export_packet"
            | "export_sanitized_snapshot"
            | "export_metadata_only"
    )
}

fn violation(
    violation_id: &str,
    surface: Option<&str>,
    action_id: Option<&str>,
    summary: &str,
) -> RepresentationCopyExportViolation {
    RepresentationCopyExportViolation {
        violation_id: violation_id.to_string(),
        surface: surface.map(str::to_string),
        action_id: action_id.map(str::to_string),
        summary: summary.to_string(),
    }
}

fn sanitize_id(value: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_sep = false;
            continue;
        }
        if last_sep {
            continue;
        }
        out.push('-');
        last_sep = true;
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "root".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_case() -> RepresentationCopyExportCase {
        RepresentationCopyExportCase {
            case_id: "case:unit:representation-copy-export".to_string(),
            minted_at: "2026-05-13T00:00:00Z".to_string(),
            source_contract_refs: vec!["schemas/ux/interaction_safety.schema.json".to_string()],
            policy_context: InteractionPolicyContext {
                policy_epoch: "policy:unit".to_string(),
                trust_state: "trusted".to_string(),
                execution_context_id: None,
            },
            surfaces: PROTECTED_COPY_EXPORT_SURFACES
                .into_iter()
                .map(surface_input)
                .collect(),
        }
    }

    fn surface_input(surface: ProtectedCopySurfaceKind) -> RepresentationCopyExportSurfaceInput {
        let token = surface.as_str();
        RepresentationCopyExportSurfaceInput {
            surface,
            surface_ref: format!("surface:{token}:unit"),
            source_target_identity_ref: format!("target:{token}:unit"),
            trust_class: "RawText".to_string(),
            target_boundary_ref: format!("boundary:{token}:unit"),
            provenance_refs: vec![format!("provenance:{token}:unit")],
            sensitive_value_classes: Vec::new(),
            inspect_or_reveal_paths: vec![format!("inspect:{token}:raw")],
            recovery_affordances: vec![format!("reopen:{token}:unit")],
            actions: vec![
                RepresentationCopyExportActionInput {
                    action_id: format!("copy_plain:{token}"),
                    action_kind: CopyExportActionKind::Copy,
                    label_class: CopyExportLabelClass::CopyPlainText,
                    visible_label: "Copy plain text".to_string(),
                    representation_class: InteractionRepresentationClass::Raw,
                    payload_mode: CopyPayloadMode::PlainText,
                    default_for_surface: true,
                    includes_context: false,
                    carries_sensitive_value: false,
                    preview_required_before_clipboard: false,
                    preserves_trust_class: true,
                    preserves_provenance: true,
                    preserves_representation_label: true,
                    reconciliation_group_ref: (surface == ProtectedCopySurfaceKind::Diff
                        || surface == ProtectedCopySurfaceKind::Review)
                        .then(|| "reconcile:diff-review-plain-copy".to_string()),
                    citation_anchor_refs: Vec::new(),
                    redaction_class: "metadata_safe_default".to_string(),
                },
                RepresentationCopyExportActionInput {
                    action_id: format!("copy_with_context:{token}"),
                    action_kind: CopyExportActionKind::Copy,
                    label_class: CopyExportLabelClass::CopyWithContext,
                    visible_label: "Copy with context".to_string(),
                    representation_class: InteractionRepresentationClass::Rendered,
                    payload_mode: CopyPayloadMode::WithContext,
                    default_for_surface: false,
                    includes_context: true,
                    carries_sensitive_value: false,
                    preview_required_before_clipboard: false,
                    preserves_trust_class: true,
                    preserves_provenance: true,
                    preserves_representation_label: true,
                    reconciliation_group_ref: None,
                    citation_anchor_refs: Vec::new(),
                    redaction_class: "metadata_safe_default".to_string(),
                },
            ],
        }
    }

    #[test]
    fn validates_happy_path_packet() {
        let packet = RepresentationCopyExportAlphaPacket::from_case(base_case());
        let report = packet.validate();

        assert!(packet.covers_protected_surfaces());
        assert!(report.passed(), "{:?}", report.violations);
        assert_eq!(
            report.reconciled_groups,
            vec!["reconcile:diff-review-plain-copy".to_string()]
        );
    }

    #[test]
    fn rejects_rendered_default_copy() {
        let mut case = base_case();
        case.surfaces[0].actions[0].representation_class = InteractionRepresentationClass::Rendered;
        case.surfaces[0].actions[0].payload_mode = CopyPayloadMode::Rendered;

        let report = RepresentationCopyExportAlphaPacket::from_case(case).validate();

        assert!(report
            .violation_ids()
            .contains("default_copy_not_raw_or_plain_safe"));
    }

    #[test]
    fn rejects_sensitive_copy_without_preview() {
        let mut case = base_case();
        case.surfaces[3].sensitive_value_classes = vec!["private_registry_handle".to_string()];
        case.surfaces[3].actions[1].carries_sensitive_value = true;

        let report = RepresentationCopyExportAlphaPacket::from_case(case).validate();

        assert!(report
            .violation_ids()
            .contains("sensitive_copy_preview_missing"));
    }
}
