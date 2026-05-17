//! Shell consumer for the beta policy-pack inspection page.
//!
//! The shell does not mint a parallel policy model. It consumes the
//! auth-owned [`aureline_auth::seeded_policy_pack_beta_page`] projection,
//! adds a compact rendering summary, and exposes the same records to the
//! headless inspector, support export, admin/settings center, and reviewer
//! fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_policy_pack_beta_page, seeded_policy_pack_beta_page, validate_policy_pack_beta_page,
    PolicyPackApplyStateClass, PolicyPackBetaDefect, PolicyPackBetaDefectKind,
    PolicyPackBetaDenialTrace, PolicyPackBetaDiff, PolicyPackBetaDiffEntry,
    PolicyPackBetaImportReceipt, PolicyPackBetaPack, PolicyPackBetaPage,
    PolicyPackBetaProfileClass, PolicyPackBetaRule, PolicyPackBetaSummary,
    PolicyPackBetaSupportExport, PolicyPackDiffEntryKind, PolicyPackProvenance,
    PolicyPackRuleEffectClass, PolicyPackSignatureStateClass, PolicyPackSourceClass,
    POLICY_PACK_BETA_DEFECT_RECORD_KIND, POLICY_PACK_BETA_DENIAL_TRACE_RECORD_KIND,
    POLICY_PACK_BETA_DIFF_ENTRY_RECORD_KIND, POLICY_PACK_BETA_DIFF_RECORD_KIND,
    POLICY_PACK_BETA_IMPORT_RECEIPT_RECORD_KIND, POLICY_PACK_BETA_PACK_RECORD_KIND,
    POLICY_PACK_BETA_PAGE_RECORD_KIND, POLICY_PACK_BETA_RULE_RECORD_KIND,
    POLICY_PACK_BETA_SCHEMA_VERSION, POLICY_PACK_BETA_SHARED_CONTRACT_REF,
    POLICY_PACK_BETA_SUMMARY_RECORD_KIND, POLICY_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

/// Stable record kind for [`PolicyPackBetaRenderSummary`] payloads.
pub const POLICY_PACK_BETA_RENDER_RECORD_KIND: &str = "shell_policy_pack_beta_render_record";

/// Shell-facing rendering summary for the beta policy-pack page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of packs rendered.
    pub pack_count: usize,
    /// Number of packs whose apply state is `effective`.
    pub effective_pack_count: usize,
    /// Number of mirror or manual-import receipts rendered.
    pub mirror_or_import_receipt_count: usize,
    /// Number of diff records rendered.
    pub diff_count: usize,
    /// Number of denial traces rendered.
    pub denial_trace_count: usize,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Source tokens rendered by the shell.
    pub source_tokens_present: Vec<String>,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl PolicyPackBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &PolicyPackBetaPage) -> Self {
        Self {
            record_kind: POLICY_PACK_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: POLICY_PACK_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_PACK_BETA_SHARED_CONTRACT_REF.to_owned(),
            pack_count: page.summary.pack_count,
            effective_pack_count: page.summary.effective_pack_count,
            mirror_or_import_receipt_count: page.summary.mirror_or_import_receipt_count,
            diff_count: page.summary.diff_count,
            denial_trace_count: page.summary.denial_trace_count,
            profiles_present: page.summary.profiles_present.clone(),
            source_tokens_present: page.summary.source_tokens_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_profiles_and_sources() {
        let page = seeded_policy_pack_beta_page();
        validate_policy_pack_beta_page(&page).expect("seeded page validates");
        let summary = PolicyPackBetaRenderSummary::from_page(&page);
        assert_eq!(summary.pack_count, page.packs.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.profiles_present.contains(&"mirror_only".to_owned()));
        assert!(summary.profiles_present.contains(&"offline".to_owned()));
        assert!(summary
            .profiles_present
            .contains(&"enterprise_managed".to_owned()));
        assert!(summary
            .source_tokens_present
            .contains(&"signed_mirror_origin".to_owned()));
        assert!(summary
            .source_tokens_present
            .contains(&"manual_signed_file_import".to_owned()));
        assert!(summary
            .source_tokens_present
            .contains(&"air_gapped_signed_transfer".to_owned()));
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_policy_pack_beta_page();
        let export = PolicyPackBetaSupportExport::from_page(
            "support-export:policy-pack-beta:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
