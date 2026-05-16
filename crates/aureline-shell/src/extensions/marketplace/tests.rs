//! Unit coverage for shell marketplace truth page projection.

use aureline_extensions::{MarketplaceCompatibilityLabelClass, MarketplaceSupportChipClass};

use super::*;

#[test]
fn seeded_marketplace_truth_page_validates() {
    let page = seeded_marketplace_truth_page();

    validate_marketplace_truth_page(&page).expect("seeded marketplace truth page must validate");
    assert_eq!(page.rows.len(), page.support_rows.len());
    assert_eq!(
        page.controlled_badge_vocabulary,
        MarketplaceTruthBadgeClass::required_acceptance_states().to_vec()
    );
}

#[test]
fn public_beta_row_uses_generated_report_support_not_catalog_copy() {
    let page = seeded_marketplace_truth_page();
    let row = page
        .rows
        .iter()
        .find(|row| row.row_id.ends_with(":public-beta"))
        .expect("seed row exists");

    assert_eq!(
        row.support_chips,
        vec![MarketplaceSupportChipClass::Experimental]
    );
    assert_eq!(
        row.compatibility_label_class,
        MarketplaceCompatibilityLabelClass::Limited
    );
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Beta));
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Mirrored));
    assert!(row
        .support_summary
        .contains("current generated compatibility report"));
}

#[test]
fn seeded_rows_cover_preinstall_trust_states() {
    let page = seeded_marketplace_truth_page();
    for badge in [
        MarketplaceTruthBadgeClass::Preview,
        MarketplaceTruthBadgeClass::Beta,
        MarketplaceTruthBadgeClass::Limited,
        MarketplaceTruthBadgeClass::Revoked,
        MarketplaceTruthBadgeClass::Mirrored,
        MarketplaceTruthBadgeClass::RetestPending,
    ] {
        assert!(
            page.rows
                .iter()
                .any(|row| row.lifecycle_badges.contains(&badge)),
            "seeded rows should cover {:?}",
            badge
        );
    }
    assert_eq!(page.summary.row_count, 4);
    assert!(page.summary.blocked_install_or_update_count >= 2);
}

#[test]
fn support_export_rows_quote_source_rows() {
    let page = seeded_marketplace_truth_page();

    for row in &page.rows {
        let export = page
            .support_rows
            .iter()
            .find(|export| export.row_ref == row.row_id)
            .expect("support row exists for marketplace row");
        assert_eq!(export.lifecycle_badges, row.lifecycle_badges);
        assert_eq!(
            export.compatibility_label_class,
            row.compatibility_label_class
        );
        assert_eq!(export.support_chips, row.support_chips);
        assert_eq!(export.trust_chips, row.trust_chips);
        assert_eq!(export.install_review_ref, row.install_review_ref);
    }
}

#[test]
fn validator_detects_support_export_drift() {
    let mut page = seeded_marketplace_truth_page();
    page.support_rows[0].support_chips.clear();

    let errors = validate_marketplace_truth_page(&page).expect_err("drift must fail validation");
    assert!(errors.iter().any(|error| matches!(
        error,
        MarketplaceTruthPageValidationError::SupportExportParityDrift { field, .. }
            if field == "support_chips"
    )));
}
