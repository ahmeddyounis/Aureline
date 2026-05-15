//! Notice-digest preview row for support-bundle manifests.
//!
//! The row is metadata-only: it embeds package counts, fingerprints, source
//! refs, and review states from `aureline-notices`, not third-party license
//! text or raw package bytes.

use aureline_notices::NoticeBundle;

use super::manifest::SizeEstimate;
use super::preview::{PreviewItemSeed, SupportBundlePreviewBuilder};
use super::vocabulary::{ActionabilityImpactClass, DiagnosticDataClass, HighRiskContentClass};

/// Stable support-pack item id for the notice digest row.
pub const SUPPORT_ITEM_NOTICE_DIGEST: &str = "support.item.notice_digest";

/// Builds the metadata-only preview row that carries the notice digest.
pub fn notice_digest_preview_item_seed(bundle: &NoticeBundle) -> PreviewItemSeed {
    let digest_bytes = serde_json::to_vec(&bundle.notice_digest)
        .map(|bytes| bytes.len() as u64)
        .ok();
    PreviewItemSeed {
        support_pack_item_id: SUPPORT_ITEM_NOTICE_DIGEST.to_owned(),
        title: "Dependency notice digest".to_owned(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".to_owned(),
        artifact_kind_class: "notice_digest".to_owned(),
        manifest_path_ref: "preview_items[].notice_digest".to_owned(),
        bundle_member_path_ref: Some("manifest/notices/notice_digest.json".to_owned()),
        source_refs: bundle.source_refs.clone(),
        size_estimate: SizeEstimate {
            estimated_bytes: digest_bytes,
            confidence_class: if digest_bytes.is_some() {
                "exact".to_owned()
            } else {
                "unknown".to_owned()
            },
            display_label: digest_bytes
                .map(|bytes| format!("{bytes} B"))
                .unwrap_or_else(|| "unknown until export".to_owned()),
            size_source_class: if digest_bytes.is_some() {
                "precomputed_manifest".to_owned()
            } else {
                "unknown_until_export".to_owned()
            },
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary: bundle.summary_sentence(),
        notes: format!(
            "Metadata-only notice digest {}; no third-party license text is embedded.",
            bundle.notice_digest.digest_fingerprint
        ),
    }
}

/// Queues the notice digest row on a support-bundle preview builder.
pub fn add_notice_digest_preview_item<'a>(
    builder: &'a mut SupportBundlePreviewBuilder,
    bundle: &NoticeBundle,
) -> &'a mut SupportBundlePreviewBuilder {
    builder.add_item(notice_digest_preview_item_seed(bundle))
}
