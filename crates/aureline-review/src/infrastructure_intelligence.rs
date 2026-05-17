//! Review-lane adapter for provider-owned infrastructure intelligence.
//!
//! Review anchors are projected from
//! [`aureline_provider::InfrastructureIntelligenceAlphaPage`] so stale,
//! partial, provider-overlay, and source-freshness labels stay identical to
//! search and support exports.

pub use aureline_provider::{
    InfrastructureIntelligenceAlphaPage, InfrastructureReviewAnchorRow,
    InfrastructureReviewProjection,
};

/// Projects infrastructure relationships into review anchors.
pub fn project_infrastructure_relationships_for_review(
    page: &InfrastructureIntelligenceAlphaPage,
) -> InfrastructureReviewProjection {
    page.review_projection()
}

#[cfg(test)]
mod tests {
    use aureline_provider::{seeded_infrastructure_intelligence_alpha_page, RedactionClass};

    use super::project_infrastructure_relationships_for_review;

    #[test]
    fn review_projection_uses_provider_owned_packet() {
        let page = seeded_infrastructure_intelligence_alpha_page();
        let projection = project_infrastructure_relationships_for_review(&page);
        assert_eq!(projection.source_page_id, page.page_id);
        assert_eq!(projection.anchor_rows.len(), page.relationships.len());
        assert_eq!(
            projection.redaction_class,
            RedactionClass::MetadataSafeDefault
        );
        assert!(projection
            .anchor_rows
            .iter()
            .all(|row| !row.relationship_ref.is_empty()));
    }
}
