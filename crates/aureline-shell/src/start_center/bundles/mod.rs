//! Start Center workflow-bundle lifecycle projections.
//!
//! This module joins the checked-in launch bundle manifests with the
//! compatibility scorecard and drift packet artifacts so Start Center can show
//! bundle lifecycle truth without re-deriving status, review, drift, rollback,
//! template, or support-export meaning locally.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserialize;

use super::{build_alpha_bundle_gallery_rows, AlphaBundleGalleryError};

const SCORECARD_PACKET: (&str, &str) = (
    "artifacts/compat/workflow_bundle_scorecard_sample.json",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/compat/workflow_bundle_scorecard_sample.json"
    )),
);

const DRIFT_PACKET: (&str, &str) = (
    "artifacts/compat/bundle_drift_packet_sample.json",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/compat/bundle_drift_packet_sample.json"
    )),
);

/// Start Center lifecycle projection for one alpha workflow bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterBundleLifecycleRow {
    /// Stable workflow bundle id.
    pub bundle_id: String,
    /// Persona and stack label inherited from the Start Center bundle gallery.
    pub persona_or_stack_label: String,
    /// Signer label inherited from the launch-bundle manifest.
    pub signer_label: String,
    /// Source label inherited from the launch-bundle manifest.
    pub source_label: String,
    /// Bundle release channel.
    pub channel: String,
    /// Compatible Aureline version range.
    pub compatible_aureline_range: String,
    /// Machine-readable scorecard status class.
    pub scorecard_status_class: String,
    /// Display label paired with [`Self::scorecard_status_class`].
    pub scorecard_display_label: String,
    /// Archetype rows backing this scorecard.
    pub archetype_row_refs: Vec<String>,
    /// Install-preview record ref shown before first apply.
    pub install_preview_ref: String,
    /// Update-preview record ref shown before upgrade or rebase.
    pub update_preview_ref: String,
    /// Remove-bundle review ref shown before removal.
    pub remove_review_ref: String,
    /// Rollback checkpoint policy for apply/update/remove/rebase.
    pub rollback_checkpoint_policy: String,
    /// Drift states currently visible for this bundle.
    pub drift_states: Vec<String>,
    /// Lifecycle actions visible across review and drift surfaces.
    pub lifecycle_actions: Vec<String>,
    /// Explicit template or scaffold refs kept mirrorable in the scorecard.
    pub template_scaffold_refs: Vec<String>,
    /// Whether template/scaffold refs are explicit, mirrorable, and free of opaque generation behavior.
    pub template_refs_explicit_and_mirrorable: bool,
    /// Support-export refs that can reconstruct this lifecycle row.
    pub support_export_refs: Vec<String>,
    /// Whether the support-safe lifecycle packet would export raw user content.
    pub raw_content_export_allowed: bool,
    /// Mirror or offline packaging posture from the scorecard row.
    pub mirror_or_offline_packaging_posture: String,
}

/// Error returned when workflow-bundle lifecycle artifacts cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleLifecycleProjectionError {
    source_ref: &'static str,
    message: String,
}

impl BundleLifecycleProjectionError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or projection failure.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(source_ref: &'static str, message: impl Into<String>) -> Self {
        Self {
            source_ref,
            message: message.into(),
        }
    }
}

impl fmt::Display for BundleLifecycleProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for BundleLifecycleProjectionError {}

impl From<AlphaBundleGalleryError> for BundleLifecycleProjectionError {
    fn from(value: AlphaBundleGalleryError) -> Self {
        Self::new(value.manifest_ref(), value.message().to_string())
    }
}

/// Builds Start Center lifecycle rows from launch-bundle manifests, scorecards, and drift packets.
///
/// # Errors
///
/// Returns [`BundleLifecycleProjectionError`] when one of the checked-in
/// artifacts cannot be parsed or lacks the fields required by the projection.
pub fn build_alpha_bundle_lifecycle_rows(
) -> Result<Vec<StartCenterBundleLifecycleRow>, BundleLifecycleProjectionError> {
    let gallery_rows = build_alpha_bundle_gallery_rows()?;
    let scorecard_packet: ScorecardPacketDoc = serde_json::from_str(SCORECARD_PACKET.1)
        .map_err(|err| BundleLifecycleProjectionError::new(SCORECARD_PACKET.0, err.to_string()))?;
    let drift_packet: DriftPacketDoc = serde_json::from_str(DRIFT_PACKET.1)
        .map_err(|err| BundleLifecycleProjectionError::new(DRIFT_PACKET.0, err.to_string()))?;

    let scorecards_by_bundle = scorecard_packet
        .rows
        .iter()
        .map(|row| (row.bundle.bundle_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();

    let mut rows = Vec::new();
    for gallery in gallery_rows {
        let scorecard = scorecards_by_bundle
            .get(gallery.bundle_id.as_str())
            .ok_or_else(|| {
                BundleLifecycleProjectionError::new(
                    SCORECARD_PACKET.0,
                    format!("missing scorecard row for {}", gallery.bundle_id),
                )
            })?;
        if scorecard.compatible_aureline_range != gallery.compatible_aureline_range {
            return Err(BundleLifecycleProjectionError::new(
                SCORECARD_PACKET.0,
                format!(
                    "{} compatible range differs between manifest and scorecard",
                    gallery.bundle_id
                ),
            ));
        }

        let matching_drift_rows = drift_packet
            .drift_rows
            .iter()
            .filter(|row| row.bundle.bundle_id == gallery.bundle_id)
            .collect::<Vec<_>>();
        let matching_remove_reviews = drift_packet
            .remove_reviews
            .iter()
            .filter(|review| review.bundle.bundle_id == gallery.bundle_id)
            .collect::<Vec<_>>();

        let mut lifecycle_actions = gallery
            .available_actions
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for drift in &matching_drift_rows {
            lifecycle_actions.extend(drift.visible_actions.iter().cloned());
        }
        if !matching_remove_reviews.is_empty() {
            lifecycle_actions.insert("remove_bundle".to_string());
        }

        let support_export_refs = scorecard
            .support_export
            .export_packet_refs
            .iter()
            .chain(drift_packet.support_export.export_packet_refs.iter())
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let template_scaffold_refs = scorecard
            .template_scaffold_refs
            .iter()
            .map(|template| template.template_ref.clone())
            .collect::<Vec<_>>();
        let template_refs_explicit_and_mirrorable = !template_scaffold_refs.is_empty()
            && scorecard.template_scaffold_refs.iter().all(|template| {
                template.mirrorable
                    && !template.opaque_generation_behavior_allowed
                    && !template.template_manifest_ref.trim().is_empty()
                    && !template.generated_lineage_contract_ref.trim().is_empty()
            });

        rows.push(StartCenterBundleLifecycleRow {
            bundle_id: gallery.bundle_id,
            persona_or_stack_label: gallery.persona_or_stack_label,
            signer_label: gallery.signer_label,
            source_label: gallery.source_label,
            channel: gallery.channel,
            compatible_aureline_range: gallery.compatible_aureline_range,
            scorecard_status_class: scorecard.compatibility_status_class.clone(),
            scorecard_display_label: scorecard.display_status_label.clone(),
            archetype_row_refs: scorecard
                .archetype_bindings
                .iter()
                .map(|binding| binding.archetype_row_ref.clone())
                .collect(),
            install_preview_ref: scorecard.review_refs.install_preview_ref.clone(),
            update_preview_ref: scorecard.review_refs.update_preview_ref.clone(),
            remove_review_ref: scorecard.review_refs.remove_review_ref.clone(),
            rollback_checkpoint_policy: scorecard.review_refs.rollback_checkpoint_policy.clone(),
            drift_states: matching_drift_rows
                .iter()
                .map(|row| row.drift_state_class.clone())
                .collect(),
            lifecycle_actions: lifecycle_actions.into_iter().collect(),
            template_scaffold_refs,
            template_refs_explicit_and_mirrorable,
            support_export_refs,
            raw_content_export_allowed: scorecard.support_export.raw_content_export_allowed
                || drift_packet.support_export.raw_content_export_allowed
                || matching_remove_reviews
                    .iter()
                    .any(|review| review.raw_user_content_exported),
            mirror_or_offline_packaging_posture: scorecard
                .mirror_or_offline_packaging_posture
                .clone(),
        });
    }
    Ok(rows)
}

/// Renders the alpha workflow-bundle lifecycle projection as deterministic plaintext.
///
/// # Errors
///
/// Returns [`BundleLifecycleProjectionError`] when
/// [`build_alpha_bundle_lifecycle_rows`] cannot project the checked-in
/// artifacts.
pub fn render_alpha_bundle_lifecycle_plaintext() -> Result<String, BundleLifecycleProjectionError> {
    let rows = build_alpha_bundle_lifecycle_rows()?;
    let mut lines = vec![
        "Workflow bundle lifecycle alpha".to_string(),
        "bundle_id | scorecard | reviews | drift | actions | support_export".to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | install={}, update={}, remove={} | {} | {} | {}",
            row.bundle_id,
            row.scorecard_display_label,
            row.install_preview_ref,
            row.update_preview_ref,
            row.remove_review_ref,
            if row.drift_states.is_empty() {
                "none".to_string()
            } else {
                row.drift_states.join(",")
            },
            row.lifecycle_actions.join(","),
            row.support_export_refs.join(",")
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

#[derive(Debug, Deserialize)]
struct ScorecardPacketDoc {
    rows: Vec<ScorecardRowDoc>,
}

#[derive(Debug, Deserialize)]
struct ScorecardRowDoc {
    bundle: BundleIdentityDoc,
    compatibility_status_class: String,
    display_status_label: String,
    compatible_aureline_range: String,
    archetype_bindings: Vec<ArchetypeBindingDoc>,
    review_refs: ReviewRefsDoc,
    template_scaffold_refs: Vec<TemplateScaffoldRefDoc>,
    mirror_or_offline_packaging_posture: String,
    support_export: SupportExportDoc,
}

#[derive(Debug, Deserialize)]
struct ArchetypeBindingDoc {
    archetype_row_ref: String,
}

#[derive(Debug, Deserialize)]
struct ReviewRefsDoc {
    install_preview_ref: String,
    update_preview_ref: String,
    remove_review_ref: String,
    rollback_checkpoint_policy: String,
}

#[derive(Debug, Deserialize)]
struct TemplateScaffoldRefDoc {
    template_ref: String,
    template_manifest_ref: String,
    generated_lineage_contract_ref: String,
    mirrorable: bool,
    opaque_generation_behavior_allowed: bool,
}

#[derive(Debug, Deserialize)]
struct SupportExportDoc {
    export_packet_refs: Vec<String>,
    raw_content_export_allowed: bool,
}

#[derive(Debug, Deserialize)]
struct DriftPacketDoc {
    drift_rows: Vec<DriftRowDoc>,
    remove_reviews: Vec<RemoveReviewDoc>,
    support_export: SupportExportDoc,
}

#[derive(Debug, Deserialize)]
struct DriftRowDoc {
    bundle: BundleIdentityDoc,
    drift_state_class: String,
    visible_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RemoveReviewDoc {
    bundle: BundleIdentityDoc,
    raw_user_content_exported: bool,
}

#[derive(Debug, Deserialize)]
struct BundleIdentityDoc {
    bundle_id: String,
}
