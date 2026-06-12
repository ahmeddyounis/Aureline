//! Docs-pack content projection for the docs/help browser.
//!
//! The content pipeline consumes [`aureline_docs::DocsPack`] records and
//! materializes the same [`DocsBrowserRowCard`] shape that boundary-card backed
//! embedded docs surfaces already render. Pack content owns docs-node identity;
//! the shell context owns client-scope and browser-handoff labels.

use aureline_docs::{
    CitationLocalityClass, CitationSourceClass, DocsDerivedExplanationKind,
    DocsExampleValidationClass, DocsExternalOpenFallback, DocsFreshnessClass,
    DocsKnowledgeObjectKind, DocsKnowledgeSurfaceKind, DocsKnowledgeSurfaceProjection,
    DocsKnowledgeSurfaceProjectionInput, DocsMirrorOfflinePosture, DocsNodeIdentity,
    DocsNodeProvenance, DocsNodeProvenanceInput, DocsPack, VersionMatchState,
};
use serde::{Deserialize, Serialize};

use crate::embedded::boundary_card::{
    BrowserFallbackPostureClass, FallbackTargetClass, IdentityMode, TrustState,
};

use super::state::{
    DocsBrowserBrowserHandoffRow, DocsBrowserClientScopeRow, DocsBrowserFreshnessRow,
    DocsBrowserRowCard, DocsBrowserSourceRow, DocsBrowserVersionRow,
};

/// Shell-owned context needed to render docs-pack content in the docs browser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserContentContext {
    /// User-visible owner label for the docs surface.
    pub owner_label: String,
    /// User-visible publisher or service label.
    pub publisher_or_service_label: String,
    /// User-visible origin label.
    pub origin_label: String,
    /// Host, domain, or local pack label.
    pub host_or_domain_label: String,
    /// Data boundary disclosed in the client-scope row.
    pub data_boundary_label: String,
    /// Boundary state disclosed in the client-scope row.
    pub boundary_state_label: String,
    /// Identity mode active when the rows were projected.
    pub identity_mode: IdentityMode,
    /// Workspace trust state active when the rows were projected.
    pub trust_state: TrustState,
    /// Policy epoch ref active when the rows were projected.
    pub policy_epoch_ref: String,
    /// Browser-handoff action label.
    pub action_label: String,
    /// Browser-handoff packet ref when external opening is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Browser fallback summary shown when external opening is degraded.
    pub fallback_summary_label: String,
    /// Browser fallback posture.
    pub posture_class: BrowserFallbackPostureClass,
    /// Browser fallback target.
    pub fallback_target_class: FallbackTargetClass,
}

impl DocsBrowserContentContext {
    /// Builds the default local docs-pack context for a loaded pack.
    pub fn local_pack(pack: &DocsPack) -> Self {
        Self {
            owner_label: pack.pack_label.clone(),
            publisher_or_service_label: "Aureline docs pack".to_owned(),
            origin_label: "Local docs pack".to_owned(),
            host_or_domain_label: pack.pack_id.clone(),
            data_boundary_label: "Local docs pack within the current workspace boundary."
                .to_owned(),
            boundary_state_label: if pack.source_truth.freshness_class.lowers_certainty() {
                "Docs pack truth is degraded".to_owned()
            } else {
                "Docs pack verified for the current scope".to_owned()
            },
            identity_mode: IdentityMode::AccountFreeLocal,
            trust_state: TrustState::Trusted,
            policy_epoch_ref: format!("epoch.docs-pack.{}", sanitize_id(&pack.pack_revision_ref)),
            action_label: "Open in browser".to_owned(),
            browser_handoff_packet_ref: pack.source_truth.browser_handoff_packet_ref.clone(),
            fallback_summary_label: "System browser is the fallback for docs handoff.".to_owned(),
            posture_class: BrowserFallbackPostureClass::SystemBrowserFirst,
            fallback_target_class: FallbackTargetClass::SystemBrowserHandoffPacket,
        }
    }
}

/// Materializes docs-pack nodes into docs-browser row cards.
pub fn docs_browser_row_cards_from_pack(
    pack: &DocsPack,
    context: &DocsBrowserContentContext,
) -> Vec<DocsBrowserRowCard> {
    pack.nodes
        .iter()
        .map(|node| docs_browser_row_card_from_node(pack, context, &node.docs_node))
        .collect()
}

fn docs_browser_row_card_from_node(
    pack: &DocsPack,
    context: &DocsBrowserContentContext,
    docs_node: &DocsNodeIdentity,
) -> DocsBrowserRowCard {
    DocsBrowserRowCard {
        surface_id_ref: docs_node.docs_node_id.clone(),
        owner_label: context.owner_label.clone(),
        publisher_or_service_label: context.publisher_or_service_label.clone(),
        origin_label: context.origin_label.clone(),
        host_or_domain_label: context.host_or_domain_label.clone(),
        knowledge_surface_projection: Some(docs_browser_projection_from_node(
            pack, context, docs_node,
        )),
        source_row: DocsBrowserSourceRow {
            class_token: docs_node.source_class.as_str().to_owned(),
            label: source_class_label(docs_node.source_class).to_owned(),
            snapshot_age_label: pack.source_truth.snapshot_age_label.clone(),
            help_status_badge_ref: pack.source_truth.help_status_badge_ref.clone(),
        },
        version_row: DocsBrowserVersionRow {
            state_token: docs_node.version_match_state.as_str().to_owned(),
            label: version_match_state_label(docs_node.version_match_state).to_owned(),
            running_build_identity_ref: pack.source_truth.running_build_identity_ref.clone(),
        },
        freshness_row: DocsBrowserFreshnessRow {
            class_token: docs_node.freshness_class.as_str().to_owned(),
            label: freshness_class_label(docs_node.freshness_class).to_owned(),
            degraded: docs_node.freshness_class.lowers_certainty(),
        },
        client_scope_row: DocsBrowserClientScopeRow {
            data_boundary_label: context.data_boundary_label.clone(),
            boundary_state_label: context.boundary_state_label.clone(),
            identity_mode_token: identity_mode_token(context.identity_mode).to_owned(),
            trust_state_token: trust_state_token(context.trust_state).to_owned(),
            policy_epoch_ref: context.policy_epoch_ref.clone(),
        },
        browser_handoff_row: DocsBrowserBrowserHandoffRow {
            available: context
                .browser_handoff_packet_ref
                .as_deref()
                .map_or(false, |packet| !packet.trim().is_empty()),
            action_label: context.action_label.clone(),
            browser_handoff_packet_ref: context.browser_handoff_packet_ref.clone(),
            fallback_summary_label: context.fallback_summary_label.clone(),
            posture_class_token: serialize_token(&context.posture_class),
            fallback_target_class_token: serialize_token(&context.fallback_target_class),
        },
    }
}

fn docs_browser_projection_from_node(
    pack: &DocsPack,
    context: &DocsBrowserContentContext,
    docs_node: &DocsNodeIdentity,
) -> DocsKnowledgeSurfaceProjection {
    let external_open = context
        .browser_handoff_packet_ref
        .as_ref()
        .filter(|packet| !packet.trim().is_empty())
        .map(|packet| {
            DocsExternalOpenFallback::available(
                format!(
                    "action:docs-browser-open-source:{}",
                    sanitize_id(&docs_node.docs_node_id)
                ),
                context.action_label.clone(),
                packet.clone(),
            )
        })
        .unwrap_or_else(|| {
            if matches!(
                docs_node.locality_class,
                CitationLocalityClass::VendorLive | CitationLocalityClass::NotInstalled
            ) {
                DocsExternalOpenFallback::blocked_by_policy(
                    "External docs source is not available in this context.",
                )
            } else {
                DocsExternalOpenFallback::not_required()
            }
        });
    let provenance = DocsNodeProvenance::new(DocsNodeProvenanceInput {
        provenance_id: format!(
            "docs-browser-provenance:{}",
            sanitize_id(&docs_node.docs_node_id)
        ),
        docs_node: docs_node.clone(),
        knowledge_object_kind: DocsKnowledgeObjectKind::from(docs_node.doc_kind),
        derived_explanation_kind: (docs_node.source_class
            == CitationSourceClass::DerivedExplanation)
            .then_some(DocsDerivedExplanationKind::Generated),
        source_build_at: pack
            .source_truth
            .source_build_at
            .clone()
            .unwrap_or_else(|| pack.pack_revision_ref.clone()),
        running_build_identity_ref: pack.source_truth.running_build_identity_ref.clone(),
        mirror_offline_posture: DocsMirrorOfflinePosture::from_locality(docs_node.locality_class),
        external_open,
        example_validation: DocsExampleValidationClass::NotApplicable,
        citation_drawer_ref: Some(format!(
            "citation-drawer:{}",
            sanitize_id(&docs_node.docs_node_id)
        )),
        infrastructure_lineage: None,
        surface_refs: vec![format!(
            "surface:docs-browser:{}",
            sanitize_id(&docs_node.docs_node_id)
        )],
    });
    DocsKnowledgeSurfaceProjection::new(DocsKnowledgeSurfaceProjectionInput {
        surface_kind: DocsKnowledgeSurfaceKind::DocsBrowser,
        surface_id_ref: docs_node.docs_node_id.clone(),
        provenance,
        citation_inspection_action_ref: format!(
            "action:docs-browser-inspect-citations:{}",
            sanitize_id(&docs_node.docs_node_id)
        ),
        open_supporting_source_action_ref: Some(format!(
            "action:docs-browser-open-source:{}",
            sanitize_id(&docs_node.docs_node_id)
        )),
        keyboard_accessible_actions: true,
        export_packet_refs: vec![format!(
            "docs-evidence-packet:docs-browser:{}",
            sanitize_id(&docs_node.docs_node_id)
        )],
    })
}

fn source_class_label(value: CitationSourceClass) -> &'static str {
    match value {
        CitationSourceClass::ProjectDocs => "Project docs (this build's authoritative pack)",
        CitationSourceClass::GeneratedReference => "Generated reference",
        CitationSourceClass::MirroredOfficialDocs => "Mirrored official docs",
        CitationSourceClass::CuratedKnowledgePack => "Curated knowledge pack",
        CitationSourceClass::VendorProviderDocs => "Vendor / provider docs",
        CitationSourceClass::SupportRunbook => "Support runbook",
        CitationSourceClass::DerivedExplanation => "Derived explanation (not authoritative)",
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

fn freshness_class_label(value: DocsFreshnessClass) -> &'static str {
    match value {
        DocsFreshnessClass::AuthoritativeLive => "Authoritative (live)",
        DocsFreshnessClass::WarmCached => "Warm cached",
        DocsFreshnessClass::DegradedCached => "Degraded cached",
        DocsFreshnessClass::Stale => "Stale",
        DocsFreshnessClass::Unverified => "Unverified",
    }
}

fn identity_mode_token(value: IdentityMode) -> &'static str {
    match value {
        IdentityMode::AccountFreeLocal => "account_free_local",
        IdentityMode::SelfHostedOrg => "self_hosted_org",
        IdentityMode::ManagedWorkspace => "managed_workspace",
    }
}

fn trust_state_token(value: TrustState) -> &'static str {
    match value {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
    }
}

fn serialize_token<T: Serialize + std::fmt::Debug>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(ToString::to_string))
        .unwrap_or_else(|| format!("{value:?}"))
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}
