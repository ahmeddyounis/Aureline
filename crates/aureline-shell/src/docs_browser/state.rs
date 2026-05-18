//! Live docs/help browser state and shell-side render projection.
//!
//! [`DocsBrowserSurfaceState`] wraps an [`EmbeddedBoundaryCardRecord`] and
//! materializes a [`DocsBrowserRowCard`] for the chrome to render. The row
//! card is byte-replayable: every label is present even when the upstream
//! truth degrades to `unknown_target_build` / `unverified` / missing
//! `source_truth`, so the docs/help pane never blanks out provenance.

use serde::{Deserialize, Serialize};

use crate::embedded::boundary_card::{
    BoundaryActionId, EmbeddedBoundaryCardRecord, FreshnessClass, SourceClass, VersionMatchState,
};

/// Live docs/help browser surface state.
///
/// The surface is owned by the shell and projects the upstream embedded
/// boundary card into a focused docs-truth row card. Other owners (a11y,
/// accessibility tree, support bundle) may also consume the projection.
#[derive(Debug, Clone)]
pub struct DocsBrowserSurfaceState {
    card: EmbeddedBoundaryCardRecord,
}

impl DocsBrowserSurfaceState {
    /// Construct a docs/help browser surface from a boundary card record.
    pub fn from_boundary_card(card: EmbeddedBoundaryCardRecord) -> Self {
        Self { card }
    }

    /// Borrow the underlying boundary card record. Tests and the host
    /// chrome may consume the rest of the embedded substrate (action
    /// partition, layout constraints) directly through this borrow.
    pub fn boundary_card(&self) -> &EmbeddedBoundaryCardRecord {
        &self.card
    }

    /// Materialize a render-ready docs/help browser row card.
    pub fn render_row_card(&self) -> DocsBrowserRowCard {
        let card = &self.card;

        let source_row = match card.source_truth.as_ref() {
            Some(truth) => DocsBrowserSourceRow {
                class_token: source_class_token(truth.source_class).to_string(),
                label: source_class_label(truth.source_class).to_string(),
                snapshot_age_label: truth.snapshot_age_label.clone(),
                help_status_badge_ref: truth.help_status_badge_ref.clone(),
            },
            None => DocsBrowserSourceRow::unknown(),
        };

        let version_row = match card.source_truth.as_ref() {
            Some(truth) => DocsBrowserVersionRow {
                state_token: version_match_state_token(truth.version_match_state).to_string(),
                label: version_match_state_label(truth.version_match_state).to_string(),
                running_build_identity_ref: truth.running_build_identity_ref.clone(),
            },
            None => DocsBrowserVersionRow::unknown(),
        };

        let freshness_row = match card.source_truth.as_ref() {
            Some(truth) => DocsBrowserFreshnessRow {
                class_token: freshness_class_token(truth.freshness_class).to_string(),
                label: freshness_class_label(truth.freshness_class).to_string(),
                degraded: freshness_is_degraded(truth.freshness_class),
            },
            None => DocsBrowserFreshnessRow::unknown(),
        };

        let client_scope_row = DocsBrowserClientScopeRow {
            data_boundary_label: card.data_boundary_label.clone(),
            boundary_state_label: card.boundary_state_label.clone(),
            identity_mode_token: identity_mode_token(&card.policy_context.identity_mode)
                .to_string(),
            trust_state_token: trust_state_token(&card.policy_context.trust_state).to_string(),
            policy_epoch_ref: card.policy_context.policy_epoch.clone(),
        };

        let browser_handoff_row = DocsBrowserBrowserHandoffRow::from_card(card);

        DocsBrowserRowCard {
            surface_id_ref: card.surface_id_ref.clone(),
            owner_label: card.owner_identity.label.clone(),
            publisher_or_service_label: card.publisher_or_service_identity.label.clone(),
            origin_label: card.origin_identity.origin_label.clone(),
            host_or_domain_label: card.origin_identity.host_or_domain_label.clone(),
            knowledge_surface_projection: None,
            source_row,
            version_row,
            freshness_row,
            client_scope_row,
            browser_handoff_row,
        }
    }
}

/// Render-ready projection of the docs/help browser skeleton.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserRowCard {
    pub surface_id_ref: String,
    pub owner_label: String,
    pub publisher_or_service_label: String,
    pub origin_label: String,
    pub host_or_domain_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knowledge_surface_projection: Option<aureline_docs::DocsKnowledgeSurfaceProjection>,
    pub source_row: DocsBrowserSourceRow,
    pub version_row: DocsBrowserVersionRow,
    pub freshness_row: DocsBrowserFreshnessRow,
    pub client_scope_row: DocsBrowserClientScopeRow,
    pub browser_handoff_row: DocsBrowserBrowserHandoffRow,
}

impl DocsBrowserRowCard {
    /// Render a stable list of `Label: value` strings the host chrome can
    /// paint into the docs/help pane without re-deriving labels from
    /// closed vocabularies.
    pub fn render_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(12);
        lines.push(format!("Owner: {}", self.owner_label));
        lines.push(format!("Publisher: {}", self.publisher_or_service_label));
        lines.push(format!(
            "Origin: {} ({})",
            self.origin_label, self.host_or_domain_label
        ));
        lines.push(format!("Source: {}", self.source_row.label));
        if let Some(age) = self.source_row.snapshot_age_label.as_deref() {
            lines.push(format!("Snapshot age: {age}"));
        }
        lines.push(format!("Version: {}", self.version_row.label));
        lines.push(format!(
            "Build: {}",
            self.version_row.running_build_identity_ref
        ));
        lines.push(format!("Freshness: {}", self.freshness_row.label));
        if let Some(projection) = &self.knowledge_surface_projection {
            lines.push(format!(
                "Source build: {}",
                projection.source_strip.source_build_at
            ));
            lines.push(format!(
                "Locality: {} / {}",
                projection.source_strip.locality_class_token,
                projection.source_strip.mirror_offline_posture_token
            ));
            lines.push(format!(
                "Citations: {} [{}]",
                projection.source_strip.citation_availability_token,
                projection.citation_inspection_action_ref
            ));
            lines.push(format!(
                "Docs truth: {}",
                projection.source_strip.truth_label
            ));
        }
        lines.push(format!(
            "Client scope: {}",
            self.client_scope_row.data_boundary_label
        ));
        lines.push(format!(
            "Boundary: {}  [identity: {}, trust: {}]",
            self.client_scope_row.boundary_state_label,
            self.client_scope_row.identity_mode_token,
            self.client_scope_row.trust_state_token
        ));
        lines.push(self.browser_handoff_row.render_line());
        lines
    }
}

/// Source-of-truth class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserSourceRow {
    pub class_token: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_age_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_status_badge_ref: Option<String>,
}

impl DocsBrowserSourceRow {
    fn unknown() -> Self {
        Self {
            class_token: "unknown_source_truth".to_string(),
            label: "Unknown source (no source-of-truth disclosed)".to_string(),
            snapshot_age_label: None,
            help_status_badge_ref: None,
        }
    }
}

/// Version match-state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserVersionRow {
    pub state_token: String,
    pub label: String,
    pub running_build_identity_ref: String,
}

impl DocsBrowserVersionRow {
    fn unknown() -> Self {
        Self {
            state_token: "unknown_target_build".to_string(),
            label: "Unknown target build".to_string(),
            running_build_identity_ref: "id:build:aureline:unknown".to_string(),
        }
    }
}

/// Freshness row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserFreshnessRow {
    pub class_token: String,
    pub label: String,
    pub degraded: bool,
}

impl DocsBrowserFreshnessRow {
    fn unknown() -> Self {
        Self {
            class_token: "unverified".to_string(),
            label: "Unverified".to_string(),
            degraded: true,
        }
    }
}

/// Client-scope row, named so users can tell which workspace / identity
/// boundary the docs page is anchored to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserClientScopeRow {
    pub data_boundary_label: String,
    pub boundary_state_label: String,
    pub identity_mode_token: String,
    pub trust_state_token: String,
    pub policy_epoch_ref: String,
}

/// Browser handoff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserBrowserHandoffRow {
    pub available: bool,
    pub action_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    pub fallback_summary_label: String,
    pub posture_class_token: String,
    pub fallback_target_class_token: String,
}

impl DocsBrowserBrowserHandoffRow {
    fn from_card(card: &EmbeddedBoundaryCardRecord) -> Self {
        let action = card.open_in_browser_action();
        let action_label = action
            .map(|row| row.action_label.clone())
            .unwrap_or_else(|| "Open in browser (unavailable)".to_string());
        let action_packet_ref = action.and_then(|row| row.browser_handoff_packet_ref.clone());
        let fallback = &card.browser_fallback;
        let packet_ref = action_packet_ref.or_else(|| fallback.browser_handoff_packet_ref.clone());

        Self {
            available: action.is_some()
                && action
                    .and_then(|row| row.browser_handoff_packet_ref.as_deref())
                    .map(|s| !s.is_empty())
                    .unwrap_or(false),
            action_label,
            browser_handoff_packet_ref: packet_ref,
            fallback_summary_label: fallback.summary_label.clone(),
            posture_class_token: serialize_token(&fallback.posture_class),
            fallback_target_class_token: serialize_token(&fallback.fallback_target_class),
        }
    }

    fn render_line(&self) -> String {
        let packet = self
            .browser_handoff_packet_ref
            .as_deref()
            .unwrap_or("missing");
        format!(
            "Handoff: {}  [posture: {}, target: {}, packet: {}]",
            self.action_label, self.posture_class_token, self.fallback_target_class_token, packet
        )
    }
}

fn source_class_token(value: SourceClass) -> &'static str {
    match value {
        SourceClass::ProjectDocs => "project_docs",
        SourceClass::GeneratedReference => "generated_reference",
        SourceClass::MirroredOfficialDocs => "mirrored_official_docs",
        SourceClass::CuratedKnowledgePack => "curated_knowledge_pack",
        SourceClass::DerivedExplanation => "derived_explanation",
        SourceClass::VendorProviderDocs => "vendor_provider_docs",
        SourceClass::SupportRunbook => "support_runbook",
        SourceClass::ExternalStatusFeed => "external_status_feed",
    }
}

fn source_class_label(value: SourceClass) -> &'static str {
    match value {
        SourceClass::ProjectDocs => "Project docs (this build's authoritative pack)",
        SourceClass::GeneratedReference => "Generated reference",
        SourceClass::MirroredOfficialDocs => "Mirrored official docs",
        SourceClass::CuratedKnowledgePack => "Curated knowledge pack",
        SourceClass::DerivedExplanation => "Derived explanation (not authoritative)",
        SourceClass::VendorProviderDocs => "Vendor / provider docs",
        SourceClass::SupportRunbook => "Support runbook",
        SourceClass::ExternalStatusFeed => "External status feed",
    }
}

fn version_match_state_token(value: VersionMatchState) -> &'static str {
    match value {
        VersionMatchState::ExactBuildMatch => "exact_build_match",
        VersionMatchState::CompatibleMinorDrift => "compatible_minor_drift",
        VersionMatchState::IncompatibleDriftDetected => "incompatible_drift_detected",
        VersionMatchState::PreReleaseUnverified => "pre_release_unverified",
        VersionMatchState::UnknownTargetBuild => "unknown_target_build",
    }
}

fn version_match_state_label(value: VersionMatchState) -> &'static str {
    match value {
        VersionMatchState::ExactBuildMatch => "Exact build match",
        VersionMatchState::CompatibleMinorDrift => "Compatible (minor drift)",
        VersionMatchState::IncompatibleDriftDetected => "Incompatible drift detected",
        VersionMatchState::PreReleaseUnverified => "Pre-release (unverified)",
        VersionMatchState::UnknownTargetBuild => "Unknown target build",
    }
}

fn freshness_class_token(value: FreshnessClass) -> &'static str {
    match value {
        FreshnessClass::AuthoritativeLive => "authoritative_live",
        FreshnessClass::WarmCached => "warm_cached",
        FreshnessClass::DegradedCached => "degraded_cached",
        FreshnessClass::Stale => "stale",
        FreshnessClass::Unverified => "unverified",
    }
}

fn freshness_class_label(value: FreshnessClass) -> &'static str {
    match value {
        FreshnessClass::AuthoritativeLive => "Authoritative (live)",
        FreshnessClass::WarmCached => "Warm cached",
        FreshnessClass::DegradedCached => "Degraded cached",
        FreshnessClass::Stale => "Stale",
        FreshnessClass::Unverified => "Unverified",
    }
}

fn freshness_is_degraded(value: FreshnessClass) -> bool {
    !matches!(
        value,
        FreshnessClass::AuthoritativeLive | FreshnessClass::WarmCached
    )
}

fn identity_mode_token(value: &crate::embedded::boundary_card::IdentityMode) -> &'static str {
    use crate::embedded::boundary_card::IdentityMode;
    match value {
        IdentityMode::AccountFreeLocal => "account_free_local",
        IdentityMode::SelfHostedOrg => "self_hosted_org",
        IdentityMode::ManagedWorkspace => "managed_workspace",
    }
}

fn trust_state_token(value: &crate::embedded::boundary_card::TrustState) -> &'static str {
    use crate::embedded::boundary_card::TrustState;
    match value {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
    }
}

fn serialize_token<T: Serialize + std::fmt::Debug>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(ToString::to_string))
        .unwrap_or_else(|| format!("{value:?}"))
}

/// Confirm that the given action partition row is the open-in-browser
/// handoff, and return its packet ref. The docs/help skeleton must never
/// surface a non-product-owned-handoff partition role as the open-in-browser
/// row, so the helper rejects anything else.
pub fn docs_browser_open_in_browser_packet_ref(card: &EmbeddedBoundaryCardRecord) -> Option<&str> {
    let action = card.open_in_browser_action()?;
    if action.action_id != BoundaryActionId::OpenInSystemBrowser {
        return None;
    }
    action.browser_handoff_packet_ref.as_deref()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    fn fixture_path(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/help/docs_browser_cases")
            .join(format!("{name}.json"))
    }

    fn load_fixture(name: &str) -> EmbeddedBoundaryCardRecord {
        let raw = std::fs::read_to_string(fixture_path(name))
            .unwrap_or_else(|err| panic!("read fixture {name}: {err}"));
        let value: JsonValue =
            serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse fixture {name}: {err}"));
        let mut object = value
            .as_object()
            .cloned()
            .unwrap_or_else(|| panic!("fixture {name} must be a JSON object"));
        object.remove("$schema");
        object.remove("__fixture__");
        serde_json::from_value(JsonValue::Object(object))
            .unwrap_or_else(|err| panic!("deserialize fixture {name}: {err}"))
    }

    #[test]
    fn live_verified_fixture_renders_full_truth_rows() {
        let card = load_fixture("project_docs_live_verified");
        let surface = DocsBrowserSurfaceState::from_boundary_card(card);
        let row_card = surface.render_row_card();

        assert_eq!(row_card.source_row.class_token, "project_docs");
        assert_eq!(
            row_card.source_row.label,
            "Project docs (this build's authoritative pack)"
        );
        assert_eq!(row_card.version_row.state_token, "exact_build_match");
        assert_eq!(row_card.version_row.label, "Exact build match");
        assert_eq!(row_card.freshness_row.class_token, "authoritative_live");
        assert!(!row_card.freshness_row.degraded);
        assert_eq!(
            row_card.client_scope_row.identity_mode_token,
            "account_free_local"
        );
        assert_eq!(row_card.client_scope_row.trust_state_token, "trusted");
        assert!(row_card.browser_handoff_row.available);
        assert_eq!(
            row_card
                .browser_handoff_row
                .browser_handoff_packet_ref
                .as_deref(),
            Some("id:browser-handoff:docs-help:project-docs")
        );
        assert_eq!(
            row_card.browser_handoff_row.posture_class_token,
            "system_browser_first"
        );
    }

    #[test]
    fn stale_snapshot_fixture_keeps_drift_truth_rows_explicit() {
        let card = load_fixture("mirrored_docs_stale_snapshot");
        let surface = DocsBrowserSurfaceState::from_boundary_card(card);
        let row_card = surface.render_row_card();

        assert_eq!(row_card.source_row.class_token, "mirrored_official_docs");
        assert_eq!(
            row_card.source_row.snapshot_age_label.as_deref(),
            Some("8 days behind upstream")
        );
        assert_eq!(
            row_card.version_row.state_token,
            "incompatible_drift_detected"
        );
        assert_eq!(row_card.version_row.label, "Incompatible drift detected");
        assert_eq!(row_card.freshness_row.class_token, "stale");
        assert!(row_card.freshness_row.degraded);
        assert_eq!(
            row_card
                .browser_handoff_row
                .browser_handoff_packet_ref
                .as_deref(),
            Some("id:browser-handoff:docs-help:acme-cloud-upstream")
        );
        let lines = row_card.render_lines();
        assert!(
            lines
                .iter()
                .any(|l| l.contains("Snapshot age: 8 days behind upstream")),
            "snapshot age row must render: {lines:#?}"
        );
        assert!(
            lines.iter().any(|l| l.starts_with("Freshness: Stale")),
            "freshness row must render the stale label: {lines:#?}"
        );
    }

    #[test]
    fn unknown_metadata_fixture_keeps_rows_explicit_for_failure_drill() {
        let card = load_fixture("unknown_metadata_unverified");
        let surface = DocsBrowserSurfaceState::from_boundary_card(card);
        let row_card = surface.render_row_card();

        assert_eq!(row_card.source_row.class_token, "derived_explanation");
        assert_eq!(row_card.version_row.state_token, "unknown_target_build");
        assert_eq!(row_card.version_row.label, "Unknown target build");
        assert_eq!(row_card.freshness_row.class_token, "unverified");
        assert_eq!(row_card.freshness_row.label, "Unverified");
        assert!(row_card.freshness_row.degraded);
        assert_eq!(row_card.client_scope_row.trust_state_token, "restricted");
        assert!(
            row_card.browser_handoff_row.available,
            "browser handoff must remain available even when metadata is unknown",
        );
        let lines = row_card.render_lines();
        for label in [
            "Source: ",
            "Version: ",
            "Freshness: ",
            "Client scope: ",
            "Handoff: ",
        ] {
            assert!(
                lines.iter().any(|l| l.starts_with(label)),
                "expected explicit row for label {label:?}: {lines:#?}",
            );
        }
    }

    #[test]
    fn missing_source_truth_falls_back_to_unknown_rows() {
        let mut card = load_fixture("project_docs_live_verified");
        card.source_truth = None;
        let surface = DocsBrowserSurfaceState::from_boundary_card(card);
        let row_card = surface.render_row_card();

        assert_eq!(row_card.source_row.class_token, "unknown_source_truth");
        assert_eq!(row_card.version_row.state_token, "unknown_target_build");
        assert_eq!(row_card.freshness_row.class_token, "unverified");
        assert!(row_card.freshness_row.degraded);
        let lines = row_card.render_lines();
        assert!(lines
            .iter()
            .any(|l| l.contains("Unknown source (no source-of-truth disclosed)")));
    }

    #[test]
    fn open_in_browser_packet_ref_round_trips_for_known_action() {
        let card = load_fixture("project_docs_live_verified");
        let packet_ref = docs_browser_open_in_browser_packet_ref(&card)
            .expect("open in browser packet ref must resolve for live verified case");
        assert_eq!(packet_ref, "id:browser-handoff:docs-help:project-docs");
    }
}
