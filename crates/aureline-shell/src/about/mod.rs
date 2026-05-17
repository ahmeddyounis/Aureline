//! Help / About "release truth" card projection.
//!
//! Help / About already renders a build identity, install mode, client
//! scope, and docs/help truth section out of the [`crate::help_about`]
//! seed. This module adds the **release-truth card** the same surface
//! reads: a deterministic projection of the generated M3 claim manifest
//! that quotes the same provenance, freshness, support-class, claim
//! posture, and lifecycle vocabulary as the
//! [`crate::service_health::ServiceHealthBetaSurface`].
//!
//! Help / About reads only the rows whose manifest `help_about` channel
//! projection binds with `binding_status = "required"`; the chrome MUST
//! not invent its own filter. Each row is rendered with a short
//! `display_summary` that joins the row's `headline`, effective claim
//! posture, effective support class, lifecycle label, and freshness state
//! so the chrome can drop the row into a single line.
//!
//! The card carries the manifest envelope so a reviewer can quote the
//! `manifest_id`, `manifest_revision`, `manifest_state`, and
//! `release_channel_scope` while reading Help / About. The chrome's
//! "Copy About card for support export" action MUST include the
//! plaintext block this card renders so support exports and Help / About
//! quote the same beta truth.

use serde::{Deserialize, Serialize};

use crate::service_health::{
    BetaRowKindClass, ClaimPostureClass, FreshnessBadgeClass, FreshnessStateClass,
    LifecycleLabelClass, ManifestStateClass, ProvenanceLabelClass, ReleaseChannelScopeClass,
    ServiceHealthBetaRow, ServiceHealthBetaSurface, ServiceHealthChannelProjection,
};

/// Stable record-kind tag carried in serialized help-about release-truth
/// payloads.
pub const HELP_ABOUT_RELEASE_TRUTH_CARD_RECORD_KIND: &str = "help_about_release_truth_card_record";

/// Schema version for the [`HelpAboutReleaseTruthCard`] payload shape.
pub const HELP_ABOUT_RELEASE_TRUTH_CARD_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every card.
pub const HELP_ABOUT_RELEASE_TRUTH_NOTICE: &str =
    "Help / About release-truth card: every row, badge, and counter is projected verbatim from the \
     generated M3 governed claim manifest. Refresh the manifest to update what users see in-product.";

/// One row rendered on the Help / About release-truth card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutReleaseTruthRow {
    pub row_id: String,
    pub row_kind: BetaRowKindClass,
    pub row_kind_token: String,
    pub headline: String,
    pub claim_family: String,
    /// Compatibility row refs this Help/About row resolves through before
    /// presenting the claim as current release truth.
    pub compatibility_row_refs: Vec<String>,
    pub claim_posture_effective: ClaimPostureClass,
    pub claim_posture_effective_token: String,
    pub claim_posture_downgraded: bool,
    pub support_effective_token: String,
    pub support_downgraded: bool,
    pub lifecycle_label: LifecycleLabelClass,
    pub lifecycle_label_token: String,
    pub freshness_badge: FreshnessBadgeClass,
    pub freshness_badge_token: String,
    pub freshness_state: FreshnessStateClass,
    pub freshness_state_token: String,
    pub evidence_date: String,
    pub review_window_days: u32,
    pub provenance_label: ProvenanceLabelClass,
    pub provenance_label_token: String,
    pub evidence_owner: String,
    /// Verbatim `binding_status` quoted from the manifest's
    /// `channel_projections[help_about]` entry.
    pub help_about_binding_status: String,
    /// Verbatim `projection_kind` quoted from the same entry.
    pub help_about_projection_kind: String,
    /// Verbatim `copy_field` quoted from the same entry.
    pub help_about_copy_field: String,
    /// Verbatim `surface_ref` quoted from the same entry.
    pub help_about_surface_ref: String,
    /// One-line summary the chrome can drop into a chip.
    pub display_summary: String,
    /// True when this row should light its own honest-warning chip.
    pub honesty_marker_present: bool,
}

impl HelpAboutReleaseTruthRow {
    fn from_row(row: &ServiceHealthBetaRow) -> Option<Self> {
        let help_about = row.help_about_projection.as_ref()?;
        let display_summary = compose_display_summary(row);
        Some(Self {
            row_id: row.row_id.clone(),
            row_kind: row.row_kind,
            row_kind_token: row.row_kind_token.clone(),
            headline: row.headline.clone(),
            claim_family: row.claim_family.clone(),
            compatibility_row_refs: row.compatibility_row_refs.clone(),
            claim_posture_effective: row.claim_posture.effective,
            claim_posture_effective_token: row.claim_posture.effective_token.clone(),
            claim_posture_downgraded: row.claim_posture.downgraded,
            support_effective_token: row.support.effective_token.clone(),
            support_downgraded: row.support.downgraded,
            lifecycle_label: row.lifecycle_label,
            lifecycle_label_token: row.lifecycle_label_token.clone(),
            freshness_badge: row.freshness.badge_class,
            freshness_badge_token: row.freshness.badge_token.clone(),
            freshness_state: row.freshness.state,
            freshness_state_token: row.freshness.state_token.clone(),
            evidence_date: row.freshness.evidence_date.clone(),
            review_window_days: row.freshness.review_window_days,
            provenance_label: row.provenance.label,
            provenance_label_token: row.provenance.label_token.clone(),
            evidence_owner: row.provenance.evidence_owner.clone(),
            help_about_binding_status: help_about.binding_status.clone(),
            help_about_projection_kind: help_about.projection_kind.clone(),
            help_about_copy_field: help_about.copy_field.clone(),
            help_about_surface_ref: help_about.surface_ref.clone(),
            display_summary,
            honesty_marker_present: row.honesty_marker_present,
        })
    }
}

fn compose_display_summary(row: &ServiceHealthBetaRow) -> String {
    let posture_chip = if row.claim_posture.downgraded {
        format!(
            "{} (declared {})",
            row.claim_posture.effective_token, row.claim_posture.declared_token
        )
    } else {
        row.claim_posture.effective_token.clone()
    };
    let support_chip = if row.support.downgraded {
        format!(
            "{} (declared {})",
            row.support.effective_token, row.support.declared_token
        )
    } else {
        row.support.effective_token.clone()
    };
    format!(
        "{headline} — posture: {posture}, support: {support}, lifecycle: {lifecycle}, freshness: {freshness}",
        headline = row.headline,
        posture = posture_chip,
        support = support_chip,
        lifecycle = row.lifecycle_label_token,
        freshness = row.freshness.state_token,
    )
}

/// Help / About release-truth card record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutReleaseTruthCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub manifest_id: String,
    pub manifest_revision: u32,
    pub manifest_state: ManifestStateClass,
    pub manifest_state_token: String,
    pub release_channel_scope: ReleaseChannelScopeClass,
    pub release_channel_scope_token: String,
    pub release_channel_scope_label: String,
    pub milestone_id: String,
    pub as_of: String,
    pub as_of_for_freshness_evaluation: String,
    pub rows: Vec<HelpAboutReleaseTruthRow>,
    pub downgraded_claim_row_count: u32,
    pub downgraded_support_row_count: u32,
    pub evidence_stale_row_count: u32,
    pub evidence_expired_row_count: u32,
    pub required_projection_missing_row_count: u32,
    pub copy_field_drift_row_count: u32,
    pub honesty_marker_present: bool,
}

impl HelpAboutReleaseTruthCard {
    /// Project the card from an upstream [`ServiceHealthBetaSurface`].
    /// Only rows whose `help_about` channel projection binds with
    /// `binding_status = "required"` are quoted; the chrome MUST not
    /// invent its own filter.
    pub fn project(surface: &ServiceHealthBetaSurface) -> Self {
        let mut rows = Vec::new();
        let mut downgraded_claim_row_count = 0u32;
        let mut downgraded_support_row_count = 0u32;
        let mut evidence_stale_row_count = 0u32;
        let mut evidence_expired_row_count = 0u32;
        let mut required_projection_missing_row_count = 0u32;
        let mut copy_field_drift_row_count = 0u32;

        for row in &surface.rows {
            let Some(help_about) = row.help_about_projection.as_ref() else {
                continue;
            };
            if help_about.binding_status != "required" {
                continue;
            }
            if let Some(card_row) = HelpAboutReleaseTruthRow::from_row(row) {
                if row.claim_posture.downgraded {
                    downgraded_claim_row_count += 1;
                }
                if row.support.downgraded {
                    downgraded_support_row_count += 1;
                }
                if row.freshness.state.is_stale() {
                    evidence_stale_row_count += 1;
                }
                if row.freshness.state.is_expired() {
                    evidence_expired_row_count += 1;
                }
                if row.required_projection_missing {
                    required_projection_missing_row_count += 1;
                }
                if row.copy_field_drifts_between_help_about_and_service_health {
                    copy_field_drift_row_count += 1;
                }
                rows.push(card_row);
            }
        }

        let honesty_marker_present = downgraded_claim_row_count > 0
            || downgraded_support_row_count > 0
            || evidence_stale_row_count > 0
            || evidence_expired_row_count > 0
            || required_projection_missing_row_count > 0
            || copy_field_drift_row_count > 0;

        Self {
            record_kind: HELP_ABOUT_RELEASE_TRUTH_CARD_RECORD_KIND.to_owned(),
            schema_version: HELP_ABOUT_RELEASE_TRUTH_CARD_SCHEMA_VERSION,
            notice: HELP_ABOUT_RELEASE_TRUTH_NOTICE.to_owned(),
            manifest_id: surface.manifest_id.clone(),
            manifest_revision: surface.manifest_revision,
            manifest_state: surface.manifest_state,
            manifest_state_token: surface.manifest_state_token.clone(),
            release_channel_scope: surface.release_channel_scope,
            release_channel_scope_token: surface.release_channel_scope_token.clone(),
            release_channel_scope_label: surface.release_channel_scope_label.clone(),
            milestone_id: surface.milestone_id.clone(),
            as_of: surface.as_of.clone(),
            as_of_for_freshness_evaluation: surface.as_of_for_freshness_evaluation.clone(),
            rows,
            downgraded_claim_row_count,
            downgraded_support_row_count,
            evidence_stale_row_count,
            evidence_expired_row_count,
            required_projection_missing_row_count,
            copy_field_drift_row_count,
            honesty_marker_present,
        }
    }

    /// Render a deterministic plaintext block for the
    /// "Copy About card for support export" action and reviewer-facing
    /// previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Help / About — release truth\n");
        out.push_str(&format!(
            "Manifest: {} (rev {})\n",
            self.manifest_id, self.manifest_revision
        ));
        out.push_str(&format!(
            "Manifest state: {} ({})\n",
            self.manifest_state.label(),
            self.manifest_state_token,
        ));
        out.push_str(&format!(
            "Release channel: {} ({})\n",
            self.release_channel_scope_label, self.release_channel_scope_token,
        ));
        out.push_str(&format!("Milestone: {}\n", self.milestone_id));
        out.push_str(&format!("As of: {}\n", self.as_of));
        out.push_str(&format!(
            "Evaluated at: {}\n",
            self.as_of_for_freshness_evaluation
        ));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        out.push('\n');
        out.push_str(&format!(
            "Summary: rows={} downgraded_claim={} downgraded_support={} stale={} expired={} projection_missing={} copy_field_drift={}\n\n",
            self.rows.len(),
            self.downgraded_claim_row_count,
            self.downgraded_support_row_count,
            self.evidence_stale_row_count,
            self.evidence_expired_row_count,
            self.required_projection_missing_row_count,
            self.copy_field_drift_row_count,
        ));

        for row in &self.rows {
            out.push_str(&format!("- {}\n", row.display_summary));
            out.push_str(&format!(
                "    row_id={} provenance={} evidence_owner={} evidence_date={} window={} days\n",
                row.row_id,
                row.provenance_label_token,
                row.evidence_owner,
                row.evidence_date,
                row.review_window_days,
            ));
            if !row.compatibility_row_refs.is_empty() {
                out.push_str(&format!(
                    "    compatibility rows: {}\n",
                    row.compatibility_row_refs.join(", "),
                ));
            }
            out.push_str(&format!(
                "    help_about channel: binding={} projection_kind={} copy_field={} surface_ref={}\n",
                row.help_about_binding_status,
                row.help_about_projection_kind,
                row.help_about_copy_field,
                row.help_about_surface_ref,
            ));
        }
        out
    }
}

/// Convenience: inspect the `help_about` projection on a row directly.
pub fn help_about_projection<'a>(
    row: &'a ServiceHealthBetaRow,
) -> Option<&'a ServiceHealthChannelProjection> {
    row.help_about_projection.as_ref()
}

#[cfg(test)]
mod tests;
