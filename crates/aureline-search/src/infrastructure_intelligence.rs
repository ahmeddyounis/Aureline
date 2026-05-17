//! Search-lane adapter for provider-owned infrastructure intelligence.
//!
//! Search consumes [`aureline_provider::InfrastructureIntelligenceAlphaPage`]
//! directly and projects search rows from the packet. This keeps source,
//! freshness, truth-layer, and partiality labels aligned with review and
//! support exports.

pub use aureline_provider::{
    InfrastructureIntelligenceAlphaPage, InfrastructureSearchProjection,
    InfrastructureSearchResultRow,
};

/// Projects read-only infrastructure relationships into search result rows.
pub fn project_infrastructure_relationships_for_search(
    page: &InfrastructureIntelligenceAlphaPage,
) -> InfrastructureSearchProjection {
    page.search_projection()
}

#[cfg(test)]
mod tests {
    use aureline_provider::{seeded_infrastructure_intelligence_alpha_page, RedactionClass};

    use super::project_infrastructure_relationships_for_search;

    #[test]
    fn search_projection_uses_provider_owned_packet() {
        let page = seeded_infrastructure_intelligence_alpha_page();
        let projection = project_infrastructure_relationships_for_search(&page);
        assert_eq!(projection.source_page_id, page.page_id);
        assert_eq!(projection.result_rows.len(), page.resources.len());
        assert_eq!(
            projection.redaction_class,
            RedactionClass::MetadataSafeDefault
        );
        assert!(projection
            .result_rows
            .iter()
            .any(|row| !row.relationship_refs.is_empty()));
    }
}
