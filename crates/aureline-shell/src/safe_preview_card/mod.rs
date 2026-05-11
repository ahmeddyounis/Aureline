//! Representation-labeled safe-preview and copy/export card.
//!
//! The card is the live shell consumer of the bounded
//! [`aureline_preview::SafePreviewRecord`] wedge. It is the surface a user
//! opens to ask "what am I looking at, what does each copy or export action
//! actually leave with, and what is the wedge refusing to claim?" without
//! trusting the chrome to be truthful on its own.
//!
//! The projection is intentionally a thin shell: every value comes verbatim
//! from the canonical preview record, the section ordering and row ids are
//! stable, and the deterministic [`SafePreviewCardSnapshot::render_plaintext`]
//! block is the same payload a support export quotes.
//!
//! Out of scope (deliberately):
//!
//! - Live mutation, AI apply, or share / publish boundary moves. The card is
//!   read-only; chrome routes "Copy" / "Export" actions back through the
//!   wedge's typed action ids.
//! - Synthesizing fields the wedge cannot prove. A row that has no value
//!   reads "(none)" rather than inventing a default.

use serde::{Deserialize, Serialize};

use aureline_preview::{CopyExportOption, SafePreviewInvariantViolation, SafePreviewRecord};

/// Stable record-kind tag carried in serialized snapshots.
pub const SAFE_PREVIEW_CARD_RECORD_KIND: &str = "safe_preview_card_snapshot_record";

/// Schema version for the [`SafePreviewCardSnapshot`] payload shape.
pub const SAFE_PREVIEW_CARD_SCHEMA_VERSION: u32 = 1;

/// Stable inspector section ids the card renders. The order is the canonical
/// reading order; chrome MUST render in this order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewSectionId {
    /// Prototype label chip and bounded-wedge honesty marker.
    PrototypeLabel,
    /// Content-class / trust-class / origin-class header.
    Header,
    /// Currently visible representation chip and transforms badge.
    CurrentlyVisible,
    /// Body extent: total bytes, visible bytes, line counts, finding counts.
    BodyExtent,
    /// Paired copy/export options with their representation labels.
    CopyExportOptions,
    /// Claim limits (workspace-local only, bounded prototype, etc).
    ClaimLimits,
    /// Representation-honesty invariants. Empty list means the preview is
    /// clean; any row here is an addressable violation the chrome MUST
    /// surface.
    Invariants,
}

impl SafePreviewSectionId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrototypeLabel => "prototype_label",
            Self::Header => "header",
            Self::CurrentlyVisible => "currently_visible",
            Self::BodyExtent => "body_extent",
            Self::CopyExportOptions => "copy_export_options",
            Self::ClaimLimits => "claim_limits",
            Self::Invariants => "invariants",
        }
    }

    pub const fn heading(self) -> &'static str {
        match self {
            Self::PrototypeLabel => "Prototype wedge",
            Self::Header => "Preview header",
            Self::CurrentlyVisible => "Currently visible",
            Self::BodyExtent => "Body extent",
            Self::CopyExportOptions => "Copy / export options",
            Self::ClaimLimits => "Claim limits",
            Self::Invariants => "Representation invariants",
        }
    }
}

/// Stable per-row honesty marker quoted verbatim on the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewRowStatus {
    /// Row is informational only (chip, header, claim limit).
    Informational,
    /// Row carries a live representation-bearing value.
    Live,
    /// Row carries a typed invariant violation the chrome MUST surface.
    Blocked,
    /// Row is an addressable copy or export action.
    Action,
}

impl SafePreviewRowStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Live => "live",
            Self::Blocked => "blocked",
            Self::Action => "action",
        }
    }
}

/// Addressable target for a row. The chrome reads this to route the
/// "Inspect" / "Copy" / "Export" / "Resolve" buttons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SafePreviewRowAddress {
    /// Row addresses a copy/export option by id.
    CopyExportOption { option_id: String, action_id: String },
    /// Row addresses a typed invariant violation by token.
    Invariant { violation_token: String },
    /// Row has no addressable target.
    None,
}

/// One card row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewRow {
    pub row_id: String,
    pub label: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
    pub status: SafePreviewRowStatus,
    pub address: SafePreviewRowAddress,
}

impl SafePreviewRow {
    fn descriptive(row_id: &str, label: &str, value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            row_id: row_id.to_owned(),
            label: label.to_owned(),
            value_token: Some(value.clone()),
            value,
            status: SafePreviewRowStatus::Informational,
            address: SafePreviewRowAddress::None,
        }
    }

    fn live(row_id: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            status: SafePreviewRowStatus::Live,
            ..Self::descriptive(row_id, label, value)
        }
    }
}

/// One card section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewSection {
    pub section_id: SafePreviewSectionId,
    pub heading: String,
    pub rows: Vec<SafePreviewRow>,
}

impl SafePreviewSection {
    fn new(section_id: SafePreviewSectionId, rows: Vec<SafePreviewRow>) -> Self {
        Self {
            section_id,
            heading: section_id.heading().to_owned(),
            rows,
        }
    }
}

/// Safe-preview card snapshot.
///
/// The snapshot is the canonical record the chrome renders, a support export
/// quotes, and a fixture replays. Every section is always present; a
/// degraded snapshot is never silently smaller than a green snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewCardSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub preview_id: String,
    pub source_subject_ref: String,
    pub prototype_label_token: String,
    pub prototype_label_text: String,
    pub content_class_token: String,
    pub content_class_display: String,
    pub trust_class_token: String,
    pub origin_class_token: String,
    pub currently_visible_representation_token: String,
    pub sections: Vec<SafePreviewSection>,
    /// True when at least one representation-honesty invariant is violated.
    /// The chrome MUST render the invariants section as Blocked when this
    /// reads true.
    pub has_invariant_violations: bool,
    /// Number of distinct copy/export options surfaced.
    pub copy_export_option_count: u32,
}

impl SafePreviewCardSnapshot {
    /// Project a snapshot from a [`SafePreviewRecord`].
    pub fn project(record: &SafePreviewRecord) -> Self {
        let violations = record.validate();
        let sections = vec![
            project_prototype_section(record),
            project_header_section(record),
            project_currently_visible_section(record),
            project_body_extent_section(record),
            project_copy_export_section(record),
            project_claim_limits_section(record),
            project_invariants_section(&violations),
        ];
        Self {
            record_kind: SAFE_PREVIEW_CARD_RECORD_KIND.to_owned(),
            schema_version: SAFE_PREVIEW_CARD_SCHEMA_VERSION,
            preview_id: record.preview_id.clone(),
            source_subject_ref: record.source_subject_ref.clone(),
            prototype_label_token: record.prototype_label_token.clone(),
            prototype_label_text: record.prototype_label_display.clone(),
            content_class_token: record.content_class_token.clone(),
            content_class_display: record.content_class_display.clone(),
            trust_class_token: record.trust_class_token.clone(),
            origin_class_token: record.origin_class_token.clone(),
            currently_visible_representation_token: record
                .currently_visible_representation_token
                .clone(),
            sections,
            has_invariant_violations: !violations.is_empty(),
            copy_export_option_count: record.copy_export_options.len() as u32,
        }
    }

    /// Locate one section by id.
    pub fn section(&self, id: SafePreviewSectionId) -> Option<&SafePreviewSection> {
        self.sections.iter().find(|s| s.section_id == id)
    }

    /// Render a deterministic plaintext block downstream consumers quote
    /// verbatim (copy-card action, support exports, fixture replays).
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Safe preview & copy/export card\n");
        out.push_str(&format!(
            "Prototype: [{ptoken}] {prototype}\nPreview: {preview}\nSubject: {subject}\n\
             Content: {content} / trust: {trust} / origin: {origin}\nCurrently visible: {visible}\n\n",
            ptoken = self.prototype_label_token,
            prototype = self.prototype_label_text,
            preview = self.preview_id,
            subject = self.source_subject_ref,
            content = self.content_class_token,
            trust = self.trust_class_token,
            origin = self.origin_class_token,
            visible = self.currently_visible_representation_token,
        ));
        for section in &self.sections {
            out.push_str(&format!("[{}]\n", section.heading));
            if section.rows.is_empty() {
                out.push_str("  (none)\n");
            }
            for row in &section.rows {
                out.push_str(&format!(
                    "  {}: {}  [{}]\n",
                    row.label,
                    row.value,
                    row.status.as_str()
                ));
            }
            out.push('\n');
        }
        out
    }
}

fn project_prototype_section(record: &SafePreviewRecord) -> SafePreviewSection {
    let row = SafePreviewRow {
        row_id: "prototype_chip".to_owned(),
        label: "Wedge label".to_owned(),
        value: record.prototype_label_display.clone(),
        value_token: Some(record.prototype_label_token.clone()),
        status: SafePreviewRowStatus::Informational,
        address: SafePreviewRowAddress::None,
    };
    SafePreviewSection::new(SafePreviewSectionId::PrototypeLabel, vec![row])
}

fn project_header_section(record: &SafePreviewRecord) -> SafePreviewSection {
    let rows = vec![
        SafePreviewRow::live("preview_id", "Preview id", record.preview_id.clone()),
        SafePreviewRow::live(
            "source_subject",
            "Source subject",
            record.source_subject_ref.clone(),
        ),
        SafePreviewRow::live(
            "content_class",
            "Content class",
            record.content_class_display.clone(),
        ),
        SafePreviewRow::live(
            "trust_class",
            "Trust class",
            record.trust_class_token.clone(),
        ),
        SafePreviewRow::live(
            "origin_class",
            "Origin",
            record.origin_class_token.clone(),
        ),
        SafePreviewRow::live(
            "source_surface_family",
            "Source surface",
            record.source_surface_family.clone(),
        ),
        SafePreviewRow::live("summary", "Summary", record.summary_line.clone()),
    ];
    SafePreviewSection::new(SafePreviewSectionId::Header, rows)
}

fn project_currently_visible_section(record: &SafePreviewRecord) -> SafePreviewSection {
    let row = SafePreviewRow {
        row_id: "currently_visible".to_owned(),
        label: "Currently visible representation".to_owned(),
        value: record.currently_visible_representation_label.clone(),
        value_token: Some(record.currently_visible_representation_token.clone()),
        status: SafePreviewRowStatus::Live,
        address: SafePreviewRowAddress::None,
    };
    SafePreviewSection::new(SafePreviewSectionId::CurrentlyVisible, vec![row])
}

fn project_body_extent_section(record: &SafePreviewRecord) -> SafePreviewSection {
    let mut rows = Vec::new();
    if let Some(total) = record.total_byte_count {
        rows.push(SafePreviewRow::live(
            "total_bytes",
            "Total bytes",
            total.to_string(),
        ));
    }
    if let Some(visible) = record.visible_byte_count {
        rows.push(SafePreviewRow::live(
            "visible_bytes",
            "Visible bytes",
            visible.to_string(),
        ));
    }
    if let Some(visible_lines) = record.visible_line_count {
        rows.push(SafePreviewRow::live(
            "visible_lines",
            "Visible lines",
            visible_lines.to_string(),
        ));
    }
    rows.push(SafePreviewRow::live(
        "suspicious_finding_count",
        "Suspicious findings",
        record.suspicious_finding_count.to_string(),
    ));
    if rows.is_empty() {
        rows.push(SafePreviewRow::descriptive(
            "no_body_extent",
            "Body extent",
            "(not applicable)",
        ));
    }
    SafePreviewSection::new(SafePreviewSectionId::BodyExtent, rows)
}

fn project_copy_export_section(record: &SafePreviewRecord) -> SafePreviewSection {
    let rows = record
        .copy_export_options
        .iter()
        .map(project_copy_export_option_row)
        .collect();
    SafePreviewSection::new(SafePreviewSectionId::CopyExportOptions, rows)
}

fn project_copy_export_option_row(option: &CopyExportOption) -> SafePreviewRow {
    let value = format!(
        "{label} — representation={representation}, scope={scope}, transforms=[{transforms}], share_safe(issue/support)={issue}/{support}",
        label = option.label,
        representation = option.representation_class,
        scope = option.scope_class,
        transforms = option.transforms_applied.join(","),
        issue = option.share_safety.safe_for_issue_report,
        support = option.share_safety.safe_for_support_bundle,
    );
    SafePreviewRow {
        row_id: option.option_id.clone(),
        label: option.action_id.clone(),
        value,
        value_token: Some(option.action_id.clone()),
        status: SafePreviewRowStatus::Action,
        address: SafePreviewRowAddress::CopyExportOption {
            option_id: option.option_id.clone(),
            action_id: option.action_id.clone(),
        },
    }
}

fn project_claim_limits_section(record: &SafePreviewRecord) -> SafePreviewSection {
    let rows = record
        .claim_limits
        .iter()
        .map(|limit| SafePreviewRow {
            row_id: format!("claim_limit:{}", limit.token),
            label: limit.token.clone(),
            value: limit.label.clone(),
            value_token: Some(limit.token.clone()),
            status: SafePreviewRowStatus::Informational,
            address: SafePreviewRowAddress::None,
        })
        .collect();
    SafePreviewSection::new(SafePreviewSectionId::ClaimLimits, rows)
}

fn project_invariants_section(
    violations: &[SafePreviewInvariantViolation],
) -> SafePreviewSection {
    if violations.is_empty() {
        let row = SafePreviewRow {
            row_id: "no_violations".to_owned(),
            label: "Representation invariants".to_owned(),
            value: "All representation-honesty invariants satisfied.".to_owned(),
            value_token: Some("no_violations".to_owned()),
            status: SafePreviewRowStatus::Informational,
            address: SafePreviewRowAddress::None,
        };
        return SafePreviewSection::new(SafePreviewSectionId::Invariants, vec![row]);
    }
    let rows = violations
        .iter()
        .map(|violation| SafePreviewRow {
            row_id: format!("invariant:{}", violation.token()),
            label: violation.token().to_owned(),
            value: invariant_description(violation),
            value_token: Some(violation.token().to_owned()),
            status: SafePreviewRowStatus::Blocked,
            address: SafePreviewRowAddress::Invariant {
                violation_token: violation.token().to_owned(),
            },
        })
        .collect();
    SafePreviewSection::new(SafePreviewSectionId::Invariants, rows)
}

fn invariant_description(violation: &SafePreviewInvariantViolation) -> String {
    match violation {
        SafePreviewInvariantViolation::NoCopyExportOptions => {
            "Preview has no copy / export options.".to_owned()
        }
        SafePreviewInvariantViolation::ActionKindMismatch {
            option_id,
            expected_kind,
            actual_kind,
        } => format!(
            "Option {option_id} reports action_kind={actual_kind} but action_id requires {expected_kind}."
        ),
        SafePreviewInvariantViolation::MissingRepresentationLabel { option_id } => {
            format!("Option {option_id} does not advertise the representation_label disclosure.")
        }
        SafePreviewInvariantViolation::MissingPairedAction {
            content_class,
            missing_action_id,
        } => format!(
            "Content class {content_class} requires action {missing_action_id} but the preview does not offer it."
        ),
        SafePreviewInvariantViolation::UnpairedRiskyTextAction {
            option_id,
            expected_peer,
        } => format!(
            "Option {option_id} does not pair the required peer action {expected_peer} in must_offer_also."
        ),
        SafePreviewInvariantViolation::UnlabeledRenderedCopy { option_id } => format!(
            "Option {option_id} offers copy_rendered but does not label the payload as a rendered representation."
        ),
        SafePreviewInvariantViolation::GeneratedOriginMismatch { actual_origin } => format!(
            "Generated preview reports origin {actual_origin} but origin must be 'generated'."
        ),
        SafePreviewInvariantViolation::GeneratedVisibleMismatch {
            actual_representation,
        } => format!(
            "Generated preview reports currently visible representation {actual_representation} but it must be 'generated'."
        ),
        SafePreviewInvariantViolation::GeneratedCopyRawWithoutCitation { option_id } => format!(
            "Generated preview offers copy_raw on {option_id} without a citation_anchor backing."
        ),
        SafePreviewInvariantViolation::OversizedMissingWindowTransform => {
            "Oversized preview reports a windowed body but no option carries truncated_or_windowed.".to_owned()
        }
        SafePreviewInvariantViolation::OversizedScopeOverclaim => {
            "Oversized preview reports a windowed body but at least one copy option still claims loaded_materialized_set scope.".to_owned()
        }
        SafePreviewInvariantViolation::OversizedMissingOmittedBytes => {
            "Oversized preview reports a windowed body but no option publishes a non-zero omitted_bytes_estimate.".to_owned()
        }
    }
}

#[cfg(test)]
mod tests;
