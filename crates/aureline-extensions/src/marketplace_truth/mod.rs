//! Marketplace truth-row projection for claimed extension discovery rows.
//!
//! The catalog descriptor owns publisher, registry, moderation,
//! revocation, and mirror metadata. The generated compatibility report owns
//! the current support class and freshness posture for claimed beta rows.
//! This module joins those two sources into one row-level projection so
//! marketplace surfaces can render lifecycle badges, compatibility labels,
//! support-class chips, trust posture, and mirrorability before opening the
//! install-review sheet.

use serde::{Deserialize, Serialize};

use crate::compatibility_matrix::{
    ExtensionBridgeMatrix, ExtensionBridgeMatrixRow, ExtensionBridgeStateClass,
    ExtensionCompatibilityLabel,
};
use crate::install_review::{BridgeStateClass, CompatibilityClaimClass, CompatibilityLabel};
use crate::manifest_baseline::{PublisherTrustTierClass, RedactionClass};
use crate::registry::{
    CatalogDescriptorDecisionClass, CatalogDescriptorRecord, CatalogLifecycleStateClass,
    CatalogMirrorabilityClass, CatalogModerationStateClass, CatalogRegistrySourceClass,
    CatalogRevocationSnapshotAgeClass, CatalogTrustBadgeInheritanceRuleClass,
};
use crate::review_alpha::RevocationStateClass;

/// Record-kind tag carried by [`MarketplaceTruthRowRecord`] payloads.
pub const MARKETPLACE_TRUTH_ROW_RECORD_KIND: &str = "marketplace_truth_row_record";

/// Record-kind tag carried by [`MarketplaceTruthSupportExportRecord`] payloads.
pub const MARKETPLACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "marketplace_truth_support_export_record";

/// Schema version for marketplace truth-row payloads.
pub const MARKETPLACE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Parsed subset of the generated compatibility report consumed by marketplace rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReportSnapshot {
    /// Discriminator for the generated report.
    pub record_kind: String,
    /// Stable report id.
    pub report_id: String,
    /// Report revision.
    pub report_revision: u32,
    /// Release-channel scope covered by the report.
    pub release_channel_scope: String,
    /// Date string the report is valid as of.
    pub as_of: String,
    /// Generation timestamp for the report.
    pub generated_at: String,
    /// Rows in the generated compatibility report.
    pub rows: Vec<CompatibilityReportRow>,
}

impl CompatibilityReportSnapshot {
    /// Returns a generated report row by `row_id` or `report_row_id`.
    pub fn row_by_ref(&self, row_ref: &str) -> Option<&CompatibilityReportRow> {
        self.rows
            .iter()
            .find(|row| row.row_id == row_ref || row.report_row_id == row_ref)
    }
}

/// One generated compatibility report row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReportRow {
    /// Stable report-local row id.
    pub report_row_id: String,
    /// Stable compatibility row id.
    pub row_id: String,
    /// Scope family for the row.
    pub row_scope: String,
    /// Claimed surface protected by this row.
    pub claimed_surface: String,
    /// Support-class object generated for this row.
    pub support_class: CompatibilityReportSupportClass,
    /// Evidence refs cited by the generated report row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Support-class object generated for one compatibility report row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReportSupportClass {
    /// Declared support class from the report.
    pub declared: String,
    /// Effective support class after downgrade triggers.
    pub effective: String,
    /// Downgrade triggers fired by the report generator.
    #[serde(default)]
    pub downgrade_triggers_fired: Vec<String>,
    /// Optional scorecard ref for the generated row.
    #[serde(default)]
    pub scorecard_ref: Option<String>,
}

/// Controlled marketplace badge vocabulary for claimed beta rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceTruthBadgeClass {
    /// Early discoverable row with expected churn or pending moderation.
    Preview,
    /// Beta row or beta-versioned package with support intent but not stable proof.
    Beta,
    /// Stable row backed by current support evidence.
    Stable,
    /// Present but on a visible sunset path.
    Deprecated,
    /// Installable or reviewable only with narrower guarantees.
    Limited,
    /// Revoked row that cannot install or update.
    Revoked,
    /// Row is mirrorable or is served through an approved mirror/offline source.
    Mirrored,
    /// Current compatibility or certification evidence requires retest.
    RetestPending,
}

impl MarketplaceTruthBadgeClass {
    /// Returns every controlled badge class required by marketplace rows.
    pub const fn required_acceptance_states() -> [Self; 8] {
        [
            Self::Preview,
            Self::Beta,
            Self::Stable,
            Self::Deprecated,
            Self::Limited,
            Self::Revoked,
            Self::Mirrored,
            Self::RetestPending,
        ]
    }

    /// Returns the stable schema token for this badge.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::Limited => "limited",
            Self::Revoked => "revoked",
            Self::Mirrored => "mirrored",
            Self::RetestPending => "retest_pending",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::Deprecated => "Deprecated",
            Self::Limited => "Limited",
            Self::Revoked => "Revoked",
            Self::Mirrored => "Mirrored",
            Self::RetestPending => "Retest pending",
        }
    }
}

/// Compatibility label rendered on marketplace rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceCompatibilityLabelClass {
    /// Compatible on the current generated-report row and catalog target.
    Compatible,
    /// Compatible with narrower guarantees than the catalog row might imply.
    Limited,
    /// Requires a compatibility bridge or shim.
    NeedsBridge,
    /// Unsupported on the current report or catalog target.
    Unsupported,
    /// Evidence is stale or pending re-verification.
    RetestPending,
}

impl MarketplaceCompatibilityLabelClass {
    /// Returns the stable schema token for this label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Limited => "limited",
            Self::NeedsBridge => "needs_bridge",
            Self::Unsupported => "unsupported",
            Self::RetestPending => "retest_pending",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Compatible => "Compatible",
            Self::Limited => "Limited",
            Self::NeedsBridge => "Needs bridge",
            Self::Unsupported => "Unsupported",
            Self::RetestPending => "Retest pending",
        }
    }
}

/// Source used to compute the marketplace compatibility label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceCompatibilityLabelSourceClass {
    /// The label was derived from a generated compatibility report row.
    GeneratedCompatibilityReport,
}

/// Support-class chip rendered on marketplace rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceSupportChipClass {
    /// Certified by current report evidence.
    Certified,
    /// Supported by current report evidence.
    Supported,
    /// Supported only with narrower guarantees.
    Limited,
    /// Community-maintained or community-supported only.
    Community,
    /// Experimental support class from the current report.
    Experimental,
    /// Current report says retest is pending.
    RetestPending,
    /// Current report says evidence is stale.
    EvidenceStale,
    /// Unsupported by the current report.
    Unsupported,
}

impl MarketplaceSupportChipClass {
    /// Returns the stable schema token for this support chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Community => "community",
            Self::Experimental => "experimental",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Certified => "Certified",
            Self::Supported => "Supported",
            Self::Limited => "Limited",
            Self::Community => "Community",
            Self::Experimental => "Experimental",
            Self::RetestPending => "Retest pending",
            Self::EvidenceStale => "Evidence stale",
            Self::Unsupported => "Unsupported",
        }
    }
}

/// Trust or source posture chip rendered on marketplace rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceTrustChipClass {
    /// Publisher is verified at the origin.
    VerifiedPublisher,
    /// Publisher is organisationally managed.
    OrganisationalPublisher,
    /// Publisher is community-maintained.
    CommunityPublisher,
    /// Publisher identity is present but not verified.
    UnverifiedPublisher,
    /// Publisher cannot inherit trust because it is quarantined.
    QuarantinedPublisher,
    /// Source is the public registry.
    PublicRegistry,
    /// Source is an approved mirror.
    ApprovedMirror,
    /// Source is a private registry.
    PrivateRegistry,
    /// Source is an offline bundle.
    OfflineBundle,
    /// Source is a local archive or manual import.
    LocalArchive,
    /// Source is a quarantined local copy.
    QuarantinedLocalCopy,
    /// Trust badge is capped by source or mirror rules.
    TrustCappedBySource,
    /// Row carries mirrorable metadata.
    Mirrorable,
}

/// Input consumed to project one marketplace truth row.
pub struct MarketplaceTruthRowInput<'a> {
    /// Stable marketplace row id.
    pub row_id: &'a str,
    /// Catalog descriptor evaluated by the registry pipeline.
    pub catalog: &'a CatalogDescriptorRecord,
    /// Current generated compatibility report.
    pub compatibility_report: &'a CompatibilityReportSnapshot,
    /// Compatibility report row id or report-row id to bind.
    pub compatibility_report_row_ref: &'a str,
    /// Current extension bridge matrix.
    pub extension_bridge_matrix: &'a ExtensionBridgeMatrix,
    /// Extension bridge-matrix row id to bind.
    pub extension_bridge_matrix_row_ref: &'a str,
    /// Native install-review or mutation-review ref opened from the row.
    pub install_review_ref: &'a str,
    /// Timestamp for the projected marketplace row.
    pub generated_at: &'a str,
}

/// Marketplace row projection consumed by discovery, shell, CLI, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceTruthRowRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this record.
    pub marketplace_truth_schema_version: u32,
    /// Stable marketplace row id.
    pub row_id: String,
    /// Catalog descriptor ref that produced this row.
    pub catalog_descriptor_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Display name rendered on the marketplace row.
    pub display_name: String,
    /// Publisher display label rendered before install review.
    pub publisher_display_label: String,
    /// Controlled lifecycle and state badges rendered on the row.
    pub lifecycle_badges: Vec<MarketplaceTruthBadgeClass>,
    /// Compatibility label derived from the generated report and catalog state.
    pub compatibility_label_class: MarketplaceCompatibilityLabelClass,
    /// Source used to compute the compatibility label.
    pub compatibility_label_source_class: MarketplaceCompatibilityLabelSourceClass,
    /// Generated compatibility report revision.
    pub compatibility_report_revision: u32,
    /// Generated compatibility report row id.
    pub compatibility_report_row_id: String,
    /// Generated compatibility report timestamp.
    pub compatibility_report_generated_at: String,
    /// Extension bridge matrix id consumed by this marketplace row.
    pub extension_bridge_matrix_id: String,
    /// Extension bridge matrix row consumed by this marketplace row.
    pub extension_bridge_matrix_row_id: String,
    /// Bridge state quoted from the bridge matrix.
    pub extension_bridge_state_class: ExtensionBridgeStateClass,
    /// Compatibility label quoted from the bridge matrix.
    pub extension_bridge_compatibility_label: ExtensionCompatibilityLabel,
    /// Runtime compatibility window quoted from the bridge matrix.
    pub runtime_compatibility_window_id: String,
    /// SDK compatibility window quoted from the bridge matrix.
    pub sdk_compatibility_window_id: String,
    /// Manifest compatibility window quoted from the bridge matrix.
    pub manifest_compatibility_window_id: String,
    /// Bridge compatibility window quoted from the bridge matrix.
    pub bridge_compatibility_window_id: String,
    /// Downgrade support class quoted from the bridge matrix.
    pub bridge_downgrade_support_class: String,
    /// Bridge out-of-window posture quoted from the bridge matrix.
    pub bridge_out_of_window_posture: String,
    /// Known limits quoted from the bridge matrix.
    pub bridge_known_limits: Vec<String>,
    /// Export-safe bridge summary.
    pub bridge_summary: String,
    /// Support-class chips derived from the generated report row.
    pub support_chips: Vec<MarketplaceSupportChipClass>,
    /// Trust and source posture chips rendered before install review.
    pub trust_chips: Vec<MarketplaceTrustChipClass>,
    /// Effective support class string quoted from the generated report.
    pub report_effective_support_class: String,
    /// Declared support class string quoted from the generated report.
    pub report_declared_support_class: String,
    /// Downgrade triggers quoted from the generated report.
    pub report_downgrade_triggers_fired: Vec<String>,
    /// Catalog compatibility claim class.
    pub catalog_compatibility_claim_class: CompatibilityClaimClass,
    /// Catalog bridge state class.
    pub catalog_bridge_state_class: BridgeStateClass,
    /// Catalog-rendered compatibility label before generated-report narrowing.
    pub catalog_rendered_compatibility_label: CompatibilityLabel,
    /// Registry source class.
    pub registry_source_class: CatalogRegistrySourceClass,
    /// Catalog lifecycle state class.
    pub catalog_lifecycle_state_class: CatalogLifecycleStateClass,
    /// Catalog decision class.
    pub catalog_decision_class: CatalogDescriptorDecisionClass,
    /// Catalog mirrorability class.
    pub mirrorability_class: CatalogMirrorabilityClass,
    /// Native install-review or mutation-review ref opened from the row.
    pub install_review_ref: String,
    /// True when install or update should be blocked before native review.
    pub blocks_install_or_update: bool,
    /// Export-safe compatibility summary for UI, CLI, and support consumers.
    pub compatibility_summary: String,
    /// Export-safe trust summary for UI, CLI, and support consumers.
    pub trust_summary: String,
    /// Export-safe support summary for UI, CLI, and support consumers.
    pub support_summary: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support export derived from a marketplace truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceTruthSupportExportRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this record.
    pub marketplace_truth_schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Source marketplace row ref.
    pub row_ref: String,
    /// Source catalog descriptor ref.
    pub catalog_descriptor_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Lifecycle badges rendered on the row.
    pub lifecycle_badges: Vec<MarketplaceTruthBadgeClass>,
    /// Compatibility label rendered on the row.
    pub compatibility_label_class: MarketplaceCompatibilityLabelClass,
    /// Support-class chips rendered on the row.
    pub support_chips: Vec<MarketplaceSupportChipClass>,
    /// Trust chips rendered on the row.
    pub trust_chips: Vec<MarketplaceTrustChipClass>,
    /// Generated report row used by the projection.
    pub compatibility_report_row_id: String,
    /// Extension bridge matrix id used by the projection.
    pub extension_bridge_matrix_id: String,
    /// Extension bridge matrix row used by the projection.
    pub extension_bridge_matrix_row_id: String,
    /// Bridge state rendered on the source row.
    pub extension_bridge_state_class: ExtensionBridgeStateClass,
    /// Bridge known limits rendered on the source row.
    pub bridge_known_limits: Vec<String>,
    /// Native install-review or mutation-review ref opened from the row.
    pub install_review_ref: String,
    /// True when install or update is blocked before native review.
    pub blocks_install_or_update: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by marketplace truth-row validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceTruthFinding {
    /// Stable validation check id.
    pub check_id: String,
    /// Human-readable validation message.
    pub message: String,
}

impl MarketplaceTruthFinding {
    fn new(check_id: &str, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.to_string(),
            message: message.into(),
        }
    }
}

/// Projects a catalog descriptor and generated compatibility report into one row.
pub fn project_marketplace_truth_row(
    input: MarketplaceTruthRowInput<'_>,
) -> Result<MarketplaceTruthRowRecord, MarketplaceTruthFinding> {
    let report_row = input
        .compatibility_report
        .row_by_ref(input.compatibility_report_row_ref)
        .ok_or_else(|| {
            MarketplaceTruthFinding::new(
                "marketplace_truth.compatibility_report_row_missing",
                format!(
                    "compatibility report row {:?} was not found in {}",
                    input.compatibility_report_row_ref, input.compatibility_report.report_id
                ),
            )
        })?;

    let catalog_label = input
        .catalog
        .mirror
        .compatibility_labels
        .first()
        .ok_or_else(|| {
            MarketplaceTruthFinding::new(
                "marketplace_truth.catalog_compatibility_missing",
                "catalog descriptor must carry at least one compatibility label",
            )
        })?;
    let bridge_row = input
        .extension_bridge_matrix
        .row_by_ref(input.extension_bridge_matrix_row_ref)
        .ok_or_else(|| {
            MarketplaceTruthFinding::new(
                "marketplace_truth.extension_bridge_matrix_row_missing",
                format!(
                    "extension bridge matrix row {:?} was not found in {}",
                    input.extension_bridge_matrix_row_ref, input.extension_bridge_matrix.matrix_id
                ),
            )
        })?;

    let support_chip = support_chip_from_report(report_row);
    let retest_pending = report_row_retest_pending(report_row)
        || matches!(
            input.catalog.revocation.revocation_snapshot_age_class,
            CatalogRevocationSnapshotAgeClass::Stale
                | CatalogRevocationSnapshotAgeClass::UnverifiedNoSnapshot
        )
        || matches!(
            input.catalog.mirror.mirrorability_class,
            CatalogMirrorabilityClass::MirrorablePendingReverify
        );
    let row_revoked = row_revoked(input.catalog);
    let blocks_install_or_update = row_revoked
        || retest_pending
        || matches!(
            input.catalog.decision_class,
            CatalogDescriptorDecisionClass::Refused
        )
        || matches!(support_chip, MarketplaceSupportChipClass::Unsupported);

    let compatibility_label_class = compatibility_label_from_sources(
        support_chip,
        retest_pending,
        row_revoked,
        catalog_label.compatibility_claim_class,
        catalog_label.bridge_state_class,
        catalog_label.rendered_label,
        bridge_row,
    );
    let lifecycle_badges = lifecycle_badges_for(
        input.catalog,
        support_chip,
        retest_pending,
        row_revoked,
        &input.catalog.extension_version,
    );
    let trust_chips = trust_chips_for(input.catalog);
    let support_chips = vec![support_chip];

    let compatibility_summary = format!(
        "Compatibility label {} is derived from the current generated compatibility report row {} (effective support {}), catalog label {}, and extension bridge matrix row {}.",
        compatibility_label_class.label(),
        report_row.row_id,
        report_row.support_class.effective,
        catalog_label.rendered_label.label(),
        bridge_row.row_id
    );
    let bridge_summary = format!(
        "Bridge matrix {} row {} declares state {:?}, label {:?}, runtime window {}, SDK window {}, manifest window {}, bridge window {}, downgrade {} / {}.",
        input.extension_bridge_matrix.matrix_id,
        bridge_row.row_id,
        bridge_row.bridge_window.bridge_state_class,
        bridge_row.bridge_window.compatibility_label,
        bridge_row.runtime_window.window_id,
        bridge_row.sdk_window.window_id,
        bridge_row.manifest_window.window_id,
        bridge_row.bridge_window.window_id,
        bridge_row.downgrade_behavior.support_class,
        bridge_row.downgrade_behavior.out_of_window_posture
    );
    let trust_summary = format!(
        "Publisher {} is {:?}; source {:?}; mirrorability {:?}; trust inheritance {:?}.",
        input.catalog.publisher.publisher_display_label,
        input.catalog.publisher.publisher_trust_tier_class,
        input.catalog.lifecycle.source_registry_class,
        input.catalog.mirror.mirrorability_class,
        input.catalog.mirror.trust_badge_inheritance_rule_class
    );
    let support_summary = format!(
        "Support class comes from the current generated compatibility report revision {} row {}: declared={}, effective={}, triggers={}.",
        input.compatibility_report.report_revision,
        report_row.row_id,
        report_row.support_class.declared,
        report_row.support_class.effective,
        if report_row.support_class.downgrade_triggers_fired.is_empty() {
            "none".to_string()
        } else {
            report_row
                .support_class
                .downgrade_triggers_fired
                .join(",")
        }
    );

    Ok(MarketplaceTruthRowRecord {
        record_kind: MARKETPLACE_TRUTH_ROW_RECORD_KIND.to_string(),
        marketplace_truth_schema_version: MARKETPLACE_TRUTH_SCHEMA_VERSION,
        row_id: input.row_id.to_string(),
        catalog_descriptor_ref: input.catalog.descriptor_id.clone(),
        extension_identity: input.catalog.extension_identity.clone(),
        extension_version: input.catalog.extension_version.clone(),
        package_id: input.catalog.package_id.clone(),
        display_name: input.catalog.display_name.clone(),
        publisher_display_label: input.catalog.publisher.publisher_display_label.clone(),
        lifecycle_badges,
        compatibility_label_class,
        compatibility_label_source_class:
            MarketplaceCompatibilityLabelSourceClass::GeneratedCompatibilityReport,
        compatibility_report_revision: input.compatibility_report.report_revision,
        compatibility_report_row_id: report_row.row_id.clone(),
        compatibility_report_generated_at: input.compatibility_report.generated_at.clone(),
        extension_bridge_matrix_id: input.extension_bridge_matrix.matrix_id.clone(),
        extension_bridge_matrix_row_id: bridge_row.row_id.clone(),
        extension_bridge_state_class: bridge_row.bridge_window.bridge_state_class,
        extension_bridge_compatibility_label: bridge_row.bridge_window.compatibility_label,
        runtime_compatibility_window_id: bridge_row.runtime_window.window_id.clone(),
        sdk_compatibility_window_id: bridge_row.sdk_window.window_id.clone(),
        manifest_compatibility_window_id: bridge_row.manifest_window.window_id.clone(),
        bridge_compatibility_window_id: bridge_row.bridge_window.window_id.clone(),
        bridge_downgrade_support_class: bridge_row.downgrade_behavior.support_class.clone(),
        bridge_out_of_window_posture: bridge_row.downgrade_behavior.out_of_window_posture.clone(),
        bridge_known_limits: bridge_row.bridge_window.known_limits.clone(),
        bridge_summary,
        support_chips,
        trust_chips,
        report_effective_support_class: report_row.support_class.effective.clone(),
        report_declared_support_class: report_row.support_class.declared.clone(),
        report_downgrade_triggers_fired: report_row.support_class.downgrade_triggers_fired.clone(),
        catalog_compatibility_claim_class: catalog_label.compatibility_claim_class,
        catalog_bridge_state_class: catalog_label.bridge_state_class,
        catalog_rendered_compatibility_label: catalog_label.rendered_label,
        registry_source_class: input.catalog.lifecycle.source_registry_class,
        catalog_lifecycle_state_class: input.catalog.lifecycle.lifecycle_state_class,
        catalog_decision_class: input.catalog.decision_class,
        mirrorability_class: input.catalog.mirror.mirrorability_class,
        install_review_ref: input.install_review_ref.to_string(),
        blocks_install_or_update,
        compatibility_summary,
        trust_summary,
        support_summary,
        generated_at: input.generated_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    })
}

/// Projects a marketplace truth row into metadata-safe support export.
pub fn project_marketplace_truth_support_export(
    row: &MarketplaceTruthRowRecord,
    export_id: &str,
) -> MarketplaceTruthSupportExportRecord {
    MarketplaceTruthSupportExportRecord {
        record_kind: MARKETPLACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        marketplace_truth_schema_version: MARKETPLACE_TRUTH_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        row_ref: row.row_id.clone(),
        catalog_descriptor_ref: row.catalog_descriptor_ref.clone(),
        extension_identity: row.extension_identity.clone(),
        extension_version: row.extension_version.clone(),
        lifecycle_badges: row.lifecycle_badges.clone(),
        compatibility_label_class: row.compatibility_label_class,
        support_chips: row.support_chips.clone(),
        trust_chips: row.trust_chips.clone(),
        compatibility_report_row_id: row.compatibility_report_row_id.clone(),
        extension_bridge_matrix_id: row.extension_bridge_matrix_id.clone(),
        extension_bridge_matrix_row_id: row.extension_bridge_matrix_row_id.clone(),
        extension_bridge_state_class: row.extension_bridge_state_class,
        bridge_known_limits: row.bridge_known_limits.clone(),
        install_review_ref: row.install_review_ref.clone(),
        blocks_install_or_update: row.blocks_install_or_update,
        export_safe_summary: format!(
            "{} {} badges={:?}; compatibility={:?}; support={:?}; trust={:?}; report_row={}; bridge_row={}; install_review={}",
            row.extension_identity,
            row.extension_version,
            row.lifecycle_badges,
            row.compatibility_label_class,
            row.support_chips,
            row.trust_chips,
            row.compatibility_report_row_id,
            row.extension_bridge_matrix_row_id,
            row.install_review_ref
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validates structural invariants for a marketplace truth row.
pub fn validate_marketplace_truth_row(
    row: &MarketplaceTruthRowRecord,
) -> Vec<MarketplaceTruthFinding> {
    let mut findings = Vec::new();

    if row.record_kind != MARKETPLACE_TRUTH_ROW_RECORD_KIND {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.record_kind_wrong",
            format!(
                "record_kind must be '{MARKETPLACE_TRUTH_ROW_RECORD_KIND}'; got {:?}",
                row.record_kind
            ),
        ));
    }
    if row.marketplace_truth_schema_version != MARKETPLACE_TRUTH_SCHEMA_VERSION {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.schema_version_wrong",
            format!(
                "marketplace_truth_schema_version must be {MARKETPLACE_TRUTH_SCHEMA_VERSION}; got {}",
                row.marketplace_truth_schema_version
            ),
        ));
    }
    if !row.row_id.starts_with("marketplace_truth_row:") {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.row_id_unprefixed",
            "row_id must start with 'marketplace_truth_row:'",
        ));
    }
    if row.lifecycle_badges.is_empty() {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.lifecycle_badges_missing",
            "marketplace row must render at least one controlled lifecycle badge",
        ));
    }
    if row.support_chips.is_empty() {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.support_chips_missing",
            "marketplace row must render a support-class chip before install review",
        ));
    }
    if row.trust_chips.is_empty() {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.trust_chips_missing",
            "marketplace row must render trust/source chips before install review",
        ));
    }
    if row.install_review_ref.trim().is_empty() {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.install_review_ref_missing",
            "marketplace row must cite the native install-review or mutation-review ref",
        ));
    }
    if row.compatibility_report_row_id.trim().is_empty() {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.compatibility_report_row_missing",
            "marketplace row must cite the generated compatibility report row",
        ));
    }
    if !row
        .extension_bridge_matrix_id
        .starts_with("extension_bridge_matrix:")
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.extension_bridge_matrix_missing",
            "marketplace row must cite the extension bridge matrix",
        ));
    }
    if !row
        .extension_bridge_matrix_row_id
        .starts_with("extension_bridge_row:")
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.extension_bridge_matrix_row_missing",
            "marketplace row must cite an extension bridge matrix row",
        ));
    }
    if row.runtime_compatibility_window_id.trim().is_empty()
        || row.sdk_compatibility_window_id.trim().is_empty()
        || row.manifest_compatibility_window_id.trim().is_empty()
        || row.bridge_compatibility_window_id.trim().is_empty()
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.extension_window_refs_missing",
            "marketplace row must cite runtime, SDK, manifest, and bridge windows",
        ));
    }
    if row
        .extension_bridge_state_class
        .requires_non_parity_disclosure()
        && row.bridge_known_limits.is_empty()
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.bridge_limits_missing",
            "bridge, shimmed, partial, and unsupported marketplace rows must carry known limits",
        ));
    }
    if row
        .extension_bridge_state_class
        .requires_non_parity_disclosure()
        && row.compatibility_label_class == MarketplaceCompatibilityLabelClass::Compatible
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.bridge_overclaims_compatible",
            "bridge, shimmed, partial, and unsupported rows must not render the Compatible marketplace label",
        ));
    }
    if row.compatibility_label_source_class
        != MarketplaceCompatibilityLabelSourceClass::GeneratedCompatibilityReport
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.compatibility_not_report_derived",
            "marketplace compatibility labels must be derived from the generated report",
        ));
    }
    if row.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.redaction_not_metadata_safe",
            "marketplace truth rows must be metadata-safe by default",
        ));
    }
    if row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::RetestPending)
        && row.compatibility_label_class != MarketplaceCompatibilityLabelClass::RetestPending
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.retest_badge_label_drift",
            "Retest pending badge must render the Retest pending compatibility label",
        ));
    }
    if row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Revoked)
        && !row.blocks_install_or_update
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.revoked_not_blocked",
            "Revoked marketplace rows must block install and update",
        ));
    }
    if row.support_chips.iter().any(|chip| {
        matches!(
            chip,
            MarketplaceSupportChipClass::Experimental
                | MarketplaceSupportChipClass::Limited
                | MarketplaceSupportChipClass::Community
                | MarketplaceSupportChipClass::RetestPending
                | MarketplaceSupportChipClass::EvidenceStale
                | MarketplaceSupportChipClass::Unsupported
        )
    }) && row.compatibility_label_class == MarketplaceCompatibilityLabelClass::Compatible
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth.compatibility_overclaims_support",
            "non-supported report support classes must not render a Compatible label",
        ));
    }

    findings
}

/// Validates structural invariants for a marketplace truth support export.
pub fn validate_marketplace_truth_support_export(
    export: &MarketplaceTruthSupportExportRecord,
) -> Vec<MarketplaceTruthFinding> {
    let mut findings = Vec::new();

    if export.record_kind != MARKETPLACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.record_kind_wrong",
            format!(
                "record_kind must be '{MARKETPLACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                export.record_kind
            ),
        ));
    }
    if export.marketplace_truth_schema_version != MARKETPLACE_TRUTH_SCHEMA_VERSION {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.schema_version_wrong",
            format!(
                "marketplace_truth_schema_version must be {MARKETPLACE_TRUTH_SCHEMA_VERSION}; got {}",
                export.marketplace_truth_schema_version
            ),
        ));
    }
    if !export
        .export_id
        .starts_with("marketplace_truth_support_export:")
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.export_id_unprefixed",
            "export_id must start with 'marketplace_truth_support_export:'",
        ));
    }
    if !export.row_ref.starts_with("marketplace_truth_row:") {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.row_ref_unprefixed",
            "row_ref must start with 'marketplace_truth_row:'",
        ));
    }
    if !export
        .extension_bridge_matrix_id
        .starts_with("extension_bridge_matrix:")
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.extension_bridge_matrix_missing",
            "support export must cite the extension bridge matrix",
        ));
    }
    if !export
        .extension_bridge_matrix_row_id
        .starts_with("extension_bridge_row:")
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.extension_bridge_matrix_row_missing",
            "support export must cite the extension bridge matrix row",
        ));
    }
    if export
        .extension_bridge_state_class
        .requires_non_parity_disclosure()
        && export.bridge_known_limits.is_empty()
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.bridge_limits_missing",
            "bridge, shimmed, partial, and unsupported support exports must carry known limits",
        ));
    }
    if export
        .extension_bridge_state_class
        .requires_non_parity_disclosure()
        && export.compatibility_label_class == MarketplaceCompatibilityLabelClass::Compatible
    {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.bridge_overclaims_compatible",
            "bridge, shimmed, partial, and unsupported support exports must not render the Compatible compatibility label",
        ));
    }
    if export.export_safe_summary.trim().is_empty() {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.summary_missing",
            "support export must carry a metadata-safe summary",
        ));
    }
    if export.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(MarketplaceTruthFinding::new(
            "marketplace_truth_export.redaction_not_metadata_safe",
            "marketplace truth support exports must be metadata-safe by default",
        ));
    }

    findings
}

fn support_chip_from_report(row: &CompatibilityReportRow) -> MarketplaceSupportChipClass {
    let effective = row.support_class.effective.trim().to_ascii_lowercase();
    match effective.as_str() {
        "certified" => MarketplaceSupportChipClass::Certified,
        "supported" => MarketplaceSupportChipClass::Supported,
        "limited" => MarketplaceSupportChipClass::Limited,
        "community" => MarketplaceSupportChipClass::Community,
        "experimental" => MarketplaceSupportChipClass::Experimental,
        "retest_pending" | "retest pending" => MarketplaceSupportChipClass::RetestPending,
        "evidence_stale" | "evidence stale" => MarketplaceSupportChipClass::EvidenceStale,
        "unsupported" | "explicitly_unsupported" => MarketplaceSupportChipClass::Unsupported,
        other if other.contains("unsupported") => MarketplaceSupportChipClass::Unsupported,
        _ => MarketplaceSupportChipClass::Limited,
    }
}

fn report_row_retest_pending(row: &CompatibilityReportRow) -> bool {
    let effective = row.support_class.effective.trim().to_ascii_lowercase();
    if matches!(
        effective.as_str(),
        "retest_pending" | "retest pending" | "evidence_stale" | "evidence stale"
    ) {
        return true;
    }
    let triggers = &row.support_class.downgrade_triggers_fired;
    !triggers.is_empty()
        && triggers
            .iter()
            .any(|trigger| !trigger.trim().eq_ignore_ascii_case("none"))
}

fn compatibility_label_from_sources(
    support_chip: MarketplaceSupportChipClass,
    retest_pending: bool,
    row_revoked: bool,
    compatibility_claim_class: CompatibilityClaimClass,
    bridge_state_class: BridgeStateClass,
    catalog_label: CompatibilityLabel,
    bridge_row: &ExtensionBridgeMatrixRow,
) -> MarketplaceCompatibilityLabelClass {
    if retest_pending {
        return MarketplaceCompatibilityLabelClass::RetestPending;
    }
    if row_revoked
        || matches!(support_chip, MarketplaceSupportChipClass::Unsupported)
        || matches!(
            compatibility_claim_class,
            CompatibilityClaimClass::IncompatibleBlockedOnPolicy
        )
        || matches!(catalog_label, CompatibilityLabel::Unsupported)
    {
        return MarketplaceCompatibilityLabelClass::Unsupported;
    }
    if matches!(
        bridge_row.bridge_window.bridge_state_class,
        ExtensionBridgeStateClass::Unsupported
    ) {
        return MarketplaceCompatibilityLabelClass::Unsupported;
    }
    if matches!(
        bridge_row.bridge_window.bridge_state_class,
        ExtensionBridgeStateClass::Bridge | ExtensionBridgeStateClass::Shimmed
    ) {
        return MarketplaceCompatibilityLabelClass::NeedsBridge;
    }
    if matches!(
        bridge_row.bridge_window.bridge_state_class,
        ExtensionBridgeStateClass::Partial
    ) {
        return MarketplaceCompatibilityLabelClass::Limited;
    }
    if matches!(
        compatibility_claim_class,
        CompatibilityClaimClass::CompatibilityBridgeRequired
    ) || matches!(
        bridge_state_class,
        BridgeStateClass::BridgeRequiredCompatibilityBridgeProfile
            | BridgeStateClass::BridgeRequiredCapabilityWorldSubsetOnly
            | BridgeStateClass::BridgeRequiredHostContractFamilySubsetOnly
    ) || matches!(
        catalog_label,
        CompatibilityLabel::Translated | CompatibilityLabel::Shimmed
    ) {
        return MarketplaceCompatibilityLabelClass::NeedsBridge;
    }
    if matches!(
        support_chip,
        MarketplaceSupportChipClass::Certified | MarketplaceSupportChipClass::Supported
    ) && matches!(
        compatibility_claim_class,
        CompatibilityClaimClass::CompatibleOnAllDeclaredTargets
    ) && matches!(catalog_label, CompatibilityLabel::Exact)
    {
        return MarketplaceCompatibilityLabelClass::Compatible;
    }
    MarketplaceCompatibilityLabelClass::Limited
}

fn lifecycle_badges_for(
    catalog: &CatalogDescriptorRecord,
    support_chip: MarketplaceSupportChipClass,
    retest_pending: bool,
    row_revoked: bool,
    extension_version: &str,
) -> Vec<MarketplaceTruthBadgeClass> {
    let mut badges = Vec::new();

    if row_revoked {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Revoked);
    } else if matches!(
        catalog.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Deprecated
    ) {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Deprecated);
    } else if matches!(
        catalog.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Staged
    ) || matches!(
        catalog.moderation.moderation_state_class,
        CatalogModerationStateClass::PendingReview
    ) {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Preview);
    } else if version_is_beta(extension_version) {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Beta);
    } else if matches!(
        support_chip,
        MarketplaceSupportChipClass::Certified | MarketplaceSupportChipClass::Supported
    ) {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Stable);
    } else {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Limited);
    }

    if row_limited(catalog, support_chip) {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Limited);
    }
    if row_mirrored(catalog) {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::Mirrored);
    }
    if retest_pending {
        push_badge(&mut badges, MarketplaceTruthBadgeClass::RetestPending);
    }

    badges
}

fn trust_chips_for(catalog: &CatalogDescriptorRecord) -> Vec<MarketplaceTrustChipClass> {
    let mut chips = Vec::new();
    chips.push(match catalog.publisher.publisher_trust_tier_class {
        PublisherTrustTierClass::VerifiedPublisher => MarketplaceTrustChipClass::VerifiedPublisher,
        PublisherTrustTierClass::OrganisationalPublisher => {
            MarketplaceTrustChipClass::OrganisationalPublisher
        }
        PublisherTrustTierClass::CommunityPublisher => {
            MarketplaceTrustChipClass::CommunityPublisher
        }
        PublisherTrustTierClass::UnverifiedPublisher
        | PublisherTrustTierClass::AnonymousPublisherClass => {
            MarketplaceTrustChipClass::UnverifiedPublisher
        }
        PublisherTrustTierClass::QuarantinedPublisher => {
            MarketplaceTrustChipClass::QuarantinedPublisher
        }
    });
    chips.push(match catalog.lifecycle.source_registry_class {
        CatalogRegistrySourceClass::PublicRegistry => MarketplaceTrustChipClass::PublicRegistry,
        CatalogRegistrySourceClass::ApprovedMirror => MarketplaceTrustChipClass::ApprovedMirror,
        CatalogRegistrySourceClass::PrivateRegistry => MarketplaceTrustChipClass::PrivateRegistry,
        CatalogRegistrySourceClass::OfflineBundle => MarketplaceTrustChipClass::OfflineBundle,
        CatalogRegistrySourceClass::LocalArchive => MarketplaceTrustChipClass::LocalArchive,
        CatalogRegistrySourceClass::QuarantinedLocalCopy => {
            MarketplaceTrustChipClass::QuarantinedLocalCopy
        }
    });
    if !matches!(
        catalog.mirror.trust_badge_inheritance_rule_class,
        CatalogTrustBadgeInheritanceRuleClass::InheritsOriginTier
    ) {
        chips.push(MarketplaceTrustChipClass::TrustCappedBySource);
    }
    if row_mirrored(catalog) {
        chips.push(MarketplaceTrustChipClass::Mirrorable);
    }
    chips
}

fn row_revoked(catalog: &CatalogDescriptorRecord) -> bool {
    matches!(
        catalog.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Revoked
    ) || matches!(
        catalog.moderation.moderation_state_class,
        CatalogModerationStateClass::Revoked
    ) || matches!(
        catalog.revocation.revocation_state_class,
        RevocationStateClass::Revoked
            | RevocationStateClass::EmergencyDisabled
            | RevocationStateClass::MirrorPromotionRevoked
    )
}

fn row_limited(
    catalog: &CatalogDescriptorRecord,
    support_chip: MarketplaceSupportChipClass,
) -> bool {
    matches!(
        catalog.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Limited | CatalogLifecycleStateClass::Deprecated
    ) || matches!(
        catalog.decision_class,
        CatalogDescriptorDecisionClass::Limited
    ) || matches!(
        support_chip,
        MarketplaceSupportChipClass::Limited
            | MarketplaceSupportChipClass::Community
            | MarketplaceSupportChipClass::Experimental
            | MarketplaceSupportChipClass::RetestPending
            | MarketplaceSupportChipClass::EvidenceStale
            | MarketplaceSupportChipClass::Unsupported
    )
}

fn row_mirrored(catalog: &CatalogDescriptorRecord) -> bool {
    matches!(
        catalog.lifecycle.source_registry_class,
        CatalogRegistrySourceClass::ApprovedMirror
            | CatalogRegistrySourceClass::PrivateRegistry
            | CatalogRegistrySourceClass::OfflineBundle
    ) || matches!(
        catalog.mirror.mirrorability_class,
        CatalogMirrorabilityClass::MirrorableVerified
            | CatalogMirrorabilityClass::MirrorableCappedTrust
            | CatalogMirrorabilityClass::MirrorablePendingReverify
    )
}

fn version_is_beta(version: &str) -> bool {
    let version = version.to_ascii_lowercase();
    version.contains("beta") || version.contains("-rc") || version.contains("preview")
}

fn push_badge(badges: &mut Vec<MarketplaceTruthBadgeClass>, badge: MarketplaceTruthBadgeClass) {
    if !badges.contains(&badge) {
        badges.push(badge);
    }
}

#[cfg(test)]
mod tests;
