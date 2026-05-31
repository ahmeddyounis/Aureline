//! Certify launch bundles, imported-user handoff bundles, and org-approved
//! bundles against archetype reports.
//!
//! The workflow-bundle review packet (see [`crate::bundles`]) owns install,
//! update, drift, and rollback truth. This module owns the layer above it: the
//! **certification claim** a bundle is allowed to make on the stable line, and
//! the **machine-readable compatibility scorecard** that claim must resolve to.
//!
//! The central rule is that a stable bundle claim — `Certified` or
//! `Managed approved` — may never be implied from prose alone. Any claimed
//! stable handoff must point to a *current* scorecard row whose freshness,
//! bridge state, downgrade state, and known-gap state still support it. When the
//! scorecard row goes stale, the bridge parity narrows, a gap is detected, or
//! the row is missing entirely, the visible badge is **automatically
//! downgraded** rather than left asserting parity the evidence no longer backs.
//!
//! The record family includes:
//!
//! - [`CertifiedBundleIdentity`] — the bundle identity that must match across
//!   Start Center, diagnostics, CLI/headless install, export packets, and docs:
//!   bundle id, signer/source, archetype class, compatible Aureline range, and
//!   certification state.
//! - [`CompatibilityScorecard`] / [`CompatibilityScorecardRow`] — the
//!   machine-readable scorecard linking bundle id, imported-extension class,
//!   bridge state, supported deployment/profile rows, and certified
//!   reference-workspace ids, with explicit freshness, downgrade, and
//!   known-gap state.
//! - [`BundleCertificationClaim`] — the claimed badge, the *effective* badge
//!   after the scorecard is applied, the support claim it is allowed to imply,
//!   and the downgrade reasons that narrowed it.
//! - [`ImportedHandoffReport`] — migration report and unsupported-item list
//!   preserved for imported-user handoff bundles so post-import help stays
//!   traceable instead of collapsing into one green banner.
//! - [`BundleArchetypeCertificationInspection`] — compact boolean projection
//!   for CLI/headless and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/certify-launch-bundles-imported-user-handoff-bundles-and.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/certify-launch-bundles-imported-user-handoff-bundles-and/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every bundle-archetype certification record.
pub const BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for [`BundleArchetypeCertificationPacket`].
pub const BUNDLE_ARCHETYPE_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "bundle_archetype_certification_packet";

/// Record-kind tag for [`CertifiedBundleIdentity`].
pub const CERTIFIED_BUNDLE_IDENTITY_RECORD_KIND: &str = "certified_bundle_identity";

/// Record-kind tag for [`BundleCertificationClaim`].
pub const BUNDLE_CERTIFICATION_CLAIM_RECORD_KIND: &str = "bundle_certification_claim";

/// Record-kind tag for [`CompatibilityScorecard`].
pub const COMPATIBILITY_SCORECARD_RECORD_KIND: &str = "bundle_compatibility_scorecard";

/// Record-kind tag for [`CompatibilityScorecardRow`].
pub const COMPATIBILITY_SCORECARD_ROW_RECORD_KIND: &str = "bundle_compatibility_scorecard_row";

/// Record-kind tag for [`ImportedHandoffReport`].
pub const IMPORTED_HANDOFF_REPORT_RECORD_KIND: &str = "imported_handoff_report";

/// Record-kind tag for [`BundleArchetypeCertificationInspection`].
pub const BUNDLE_ARCHETYPE_CERTIFICATION_INSPECTION_RECORD_KIND: &str =
    "bundle_archetype_certification_inspection";

/// Canonical schema path the export cites.
pub const BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/review/certify-launch-bundles-imported-user-handoff-bundles-and.schema.json";

/// Closed set of product bundle classes.
pub const BUNDLE_CLASSES: &[&str] = &[
    "launch_bundle",
    "imported_user_bundle",
    "org_approved_bundle",
    "design_partner_bundle",
    "local_draft_bundle",
];

/// Closed set of bundle source classes kept distinct on every surface.
pub const BUNDLE_SOURCE_CLASSES: &[&str] = &[
    "certified",
    "managed_approved",
    "community",
    "imported",
    "local_draft",
];

/// Closed set of archetype classes a bundle can certify against.
pub const BUNDLE_ARCHETYPE_CLASSES: &[&str] = &[
    "tsjs_web",
    "python_service",
    "rust_systems",
    "polyglot_monorepo",
    "data_notebook",
    "imported_generic",
    "local_unclassified",
];

/// Closed set of certification state classes carried by identity.
pub const CERTIFICATION_STATE_CLASSES: &[&str] = &[
    "certified_current",
    "managed_approved_current",
    "community_unverified",
    "imported_pending_review",
    "local_draft",
    "retest_pending",
    "certification_stale",
    "status_unknown",
];

/// Closed set of effective badge classes after the scorecard is applied.
pub const EFFECTIVE_BADGE_CLASSES: &[&str] = &[
    "certified",
    "managed_approved",
    "community",
    "imported",
    "local_draft",
    "retest_pending",
    "limited",
    "status_unknown",
];

/// Closed set of support claim classes the badge may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_launch_wedge_claim",
    "managed_org_claim",
    "community_no_certification_claim",
    "imported_pending_review_claim",
    "local_draft_no_claim",
    "limited_retest_pending_claim",
];

/// Closed set of claim-basis classes. `prose_only` may never back a stable claim.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["scorecard_row_backed", "prose_only"];

/// Closed set of scorecard freshness classes.
pub const SCORECARD_FRESHNESS_CLASSES: &[&str] = &[
    "fresh_current",
    "aging_within_window",
    "stale_past_window",
    "imported_evidence",
    "evidence_unknown",
];

/// Closed set of bridge state classes for a scorecard row.
pub const BRIDGE_STATE_CLASSES: &[&str] = &[
    "exact_bridge",
    "partial_bridge",
    "approximate_bridge",
    "unsupported_bridge",
    "bridge_unknown",
];

/// Closed set of downgrade-state classes a scorecard row can carry.
pub const SCORECARD_DOWNGRADE_STATE_CLASSES: &[&str] = &[
    "no_downgrade",
    "downgraded_freshness_expired",
    "downgraded_bridge_narrowed",
    "downgraded_gap_detected",
];

/// Closed set of known-gap state classes a scorecard row can carry.
pub const KNOWN_GAP_STATE_CLASSES: &[&str] =
    &["no_known_gaps", "known_gaps_disclosed", "gaps_unknown"];

/// Closed set of imported-extension classes for a scorecard row.
pub const IMPORTED_EXTENSION_CLASSES: &[&str] = &[
    "mapped_native",
    "bridged_compatibility",
    "approximate_shim",
    "unsupported_extension",
    "not_applicable",
];

/// Closed set of bundle distribution classes. Every channel uses the same
/// scorecard vocabulary; offline/mirror never degrade to opaque archive import.
pub const BUNDLE_DISTRIBUTION_CLASSES: &[&str] =
    &["public_registry", "mirror_first", "offline_archive"];

/// Closed set of downgrade reasons that narrow a stable claim.
pub const BUNDLE_CERTIFICATION_DOWNGRADE_REASONS: &[&str] = &[
    "scorecard_freshness_expired",
    "bridge_parity_narrowed",
    "known_gap_detected",
    "missing_scorecard_row",
    "scorecard_row_bundle_mismatch",
    "prose_only_claim",
    "reference_workspace_missing",
    "certification_state_stale",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const BUNDLE_CERTIFICATION_CONSUMER_SURFACES: &[&str] = &[
    "start_center_bundle_card",
    "bundle_detail_page",
    "diagnostics_lane",
    "cli_headless_install",
    "support_export",
    "docs_surface",
    "about_surface",
];

/// Badge classes that count as a *stable* handoff claim. These may only render
/// when backed by a current scorecard row.
pub const STABLE_BADGE_CLASSES: &[&str] = &["certified", "managed_approved"];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a bundle-archetype certification packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleArchetypeCertificationInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Bundle identity input.
    pub identity: CertifiedBundleIdentityInput,
    /// Compatibility scorecard input.
    pub scorecard: CompatibilityScorecardInput,
    /// Certification claim input.
    pub claim: BundleCertificationClaimInput,
    /// Optional imported-user handoff report input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_handoff: Option<ImportedHandoffReportInput>,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`CertifiedBundleIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedBundleIdentityInput {
    /// Stable bundle id (must match every surface).
    pub bundle_id: String,
    /// Integer bundle revision.
    pub bundle_revision: u32,
    /// Product bundle class.
    pub bundle_class: String,
    /// Bundle source class.
    pub bundle_source_class: String,
    /// Archetype class the bundle certifies against.
    pub archetype_class: String,
    /// Signer source class from the manifest.
    pub signer_source_class: String,
    /// Stable signer reference safe for export.
    pub signer_ref: String,
    /// Compatible Aureline version range copied to every surface.
    pub compatible_aureline_range: String,
    /// Certification state at packet mint time.
    pub certification_state_class: String,
    /// Distribution class (public registry, mirror-first, offline archive).
    pub distribution_class: String,
}

/// Input for [`CompatibilityScorecard`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityScorecardInput {
    /// Stable scorecard id.
    pub scorecard_id: String,
    /// Machine-readable scorecard reference.
    pub scorecard_ref: String,
    /// Timestamp the scorecard was generated.
    pub generated_at: String,
    /// Scorecard rows.
    pub rows: Vec<CompatibilityScorecardRowInput>,
}

/// Input for one [`CompatibilityScorecardRow`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityScorecardRowInput {
    /// Stable row id.
    pub row_id: String,
    /// Bundle id this row certifies.
    pub bundle_id_ref: String,
    /// Imported-extension class for this row.
    pub imported_extension_class: String,
    /// Bridge state for this row.
    pub bridge_state_class: String,
    /// Supported deployment rows referenced by this scorecard row.
    #[serde(default)]
    pub supported_deployment_refs: Vec<String>,
    /// Supported profile rows referenced by this scorecard row.
    #[serde(default)]
    pub supported_profile_refs: Vec<String>,
    /// Certified reference-workspace ids backing this row.
    #[serde(default)]
    pub certified_reference_workspace_ids: Vec<String>,
    /// Freshness class for this row.
    pub freshness_class: String,
    /// Timestamp the row freshness expires.
    pub freshness_expires_at: String,
    /// Downgrade state for this row.
    pub downgrade_state_class: String,
    /// Known-gap state for this row.
    pub known_gap_state_class: String,
    /// Known-gap references disclosed by this row.
    #[serde(default)]
    pub known_gap_refs: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BundleCertificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleCertificationClaimInput {
    /// Badge class claimed by the bundle source.
    pub claimed_badge_class: String,
    /// Claim basis: scorecard-row backed vs prose only.
    pub claim_basis_class: String,
    /// Scorecard row this claim points to, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scorecard_row_ref: Option<String>,
    /// True when the claim asserts a certified reference workspace.
    pub asserts_reference_workspace: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ImportedHandoffReport`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedHandoffReportInput {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Migration report reference preserved post-import.
    pub migration_report_ref: String,
    /// Unsupported-item references preserved for post-import help.
    #[serde(default)]
    pub unsupported_item_refs: Vec<String>,
    /// Partial-mapping references preserved for post-import help.
    #[serde(default)]
    pub partial_item_refs: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Bundle identity that must match across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedBundleIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Integer bundle revision.
    pub bundle_revision: u32,
    /// Product bundle class.
    pub bundle_class: String,
    /// Bundle source class.
    pub bundle_source_class: String,
    /// Archetype class the bundle certifies against.
    pub archetype_class: String,
    /// Signer source class from the manifest.
    pub signer_source_class: String,
    /// Stable signer reference safe for export.
    pub signer_ref: String,
    /// Compatible Aureline version range copied to every surface.
    pub compatible_aureline_range: String,
    /// Certification state at packet mint time.
    pub certification_state_class: String,
    /// Distribution class (public registry, mirror-first, offline archive).
    pub distribution_class: String,
}

/// Machine-readable compatibility scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityScorecard {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable scorecard id.
    pub scorecard_id: String,
    /// Machine-readable scorecard reference.
    pub scorecard_ref: String,
    /// Timestamp the scorecard was generated.
    pub generated_at: String,
    /// Scorecard rows.
    pub rows: Vec<CompatibilityScorecardRow>,
}

/// One row of a compatibility scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityScorecardRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Bundle id this row certifies.
    pub bundle_id_ref: String,
    /// Imported-extension class for this row.
    pub imported_extension_class: String,
    /// Bridge state for this row.
    pub bridge_state_class: String,
    /// Supported deployment rows referenced by this scorecard row.
    pub supported_deployment_refs: Vec<String>,
    /// Supported profile rows referenced by this scorecard row.
    pub supported_profile_refs: Vec<String>,
    /// Certified reference-workspace ids backing this row.
    pub certified_reference_workspace_ids: Vec<String>,
    /// Freshness class for this row.
    pub freshness_class: String,
    /// Timestamp the row freshness expires.
    pub freshness_expires_at: String,
    /// Downgrade state for this row.
    pub downgrade_state_class: String,
    /// Known-gap state for this row.
    pub known_gap_state_class: String,
    /// Known-gap references disclosed by this row.
    pub known_gap_refs: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl CompatibilityScorecardRow {
    /// Returns true when this row is current enough to back a stable claim.
    pub fn is_current(&self) -> bool {
        matches!(
            self.freshness_class.as_str(),
            "fresh_current" | "aging_within_window"
        ) && self.downgrade_state_class == "no_downgrade"
    }

    /// Returns true when this row's bridge state still supports exact/partial
    /// parity (not approximate or unsupported).
    pub fn bridge_supports_parity(&self) -> bool {
        matches!(
            self.bridge_state_class.as_str(),
            "exact_bridge" | "partial_bridge"
        )
    }
}

/// Certification claim after the scorecard is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleCertificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Badge class claimed by the bundle source.
    pub claimed_badge_class: String,
    /// Effective badge class after the scorecard is applied.
    pub effective_badge_class: String,
    /// Support claim the effective badge is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: scorecard-row backed vs prose only.
    pub claim_basis_class: String,
    /// Scorecard row this claim points to, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scorecard_row_ref: Option<String>,
    /// True when the claim asserts a certified reference workspace.
    pub asserts_reference_workspace: bool,
    /// True when the claimed badge was downgraded by the scorecard.
    pub downgraded: bool,
    /// Downgrade reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Imported-user handoff report preserved post-import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedHandoffReport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable handoff id.
    pub handoff_id: String,
    /// Migration report reference preserved post-import.
    pub migration_report_ref: String,
    /// Unsupported-item references preserved for post-import help.
    pub unsupported_item_refs: Vec<String>,
    /// Partial-mapping references preserved for post-import help.
    pub partial_item_refs: Vec<String>,
    /// True when the migration report and unsupported list are preserved (never
    /// collapsed into a single green banner).
    pub preserved: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleArchetypeCertificationInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Bundle inspected by this row.
    pub bundle_id_ref: String,
    /// Effective badge class.
    pub effective_badge_class: String,
    /// True when the claim is a stable handoff claim.
    pub stable_claim: bool,
    /// True when the effective stable claim resolves to a current scorecard row.
    pub resolves_to_current_scorecard_row: bool,
    /// True when the claimed badge was downgraded by the scorecard.
    pub downgraded: bool,
    /// True when no stable claim is backed by prose alone.
    pub no_prose_only_stable_claim: bool,
    /// True when the bundle identity is internally consistent.
    pub identity_consistent: bool,
    /// True when imported-user handoff context is preserved (when applicable).
    pub imported_handoff_preserved: bool,
    /// True when offline/mirror distribution keeps full scorecard vocabulary.
    pub offline_parity_preserved: bool,
    /// True when no hidden trust/egress/provider widening is implied.
    pub no_hidden_authority_widening: bool,
    /// Number of scorecard rows.
    pub scorecard_row_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Bundle-archetype certification packet consumed by Start Center, diagnostics,
/// CLI/headless install, support export, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleArchetypeCertificationPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Bundle identity.
    pub identity: CertifiedBundleIdentity,
    /// Compatibility scorecard.
    pub scorecard: CompatibilityScorecard,
    /// Certification claim after the scorecard is applied.
    pub claim: BundleCertificationClaim,
    /// Optional imported-user handoff report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_handoff: Option<ImportedHandoffReport>,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so hidden imperative setup hooks cannot ride a certified bundle.
    pub allows_hidden_setup_hooks: bool,
    /// False so secret injection cannot ride bundle application.
    pub allows_secret_injection: bool,
    /// False so silent trust/egress/provider widening cannot ride a bundle.
    pub allows_silent_trust_widening: bool,
    /// Inspection row.
    pub inspection: BundleArchetypeCertificationInspection,
}

impl BundleArchetypeCertificationPacket {
    /// Builds a certification packet from input, applying the scorecard to the
    /// claimed badge so any required downgrade is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`BundleCertificationValidationError`] when the input violates an
    /// identity, scorecard, claim, or handoff invariant.
    pub fn from_input(
        input: BundleArchetypeCertificationInput,
    ) -> Result<Self, BundleCertificationValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let scorecard = scorecard_record(&input.scorecard);
        let claim = claim_record(&input.claim, &identity, &scorecard);
        let imported_handoff = input
            .imported_handoff
            .as_ref()
            .map(|h| handoff_record(h, &identity));
        let inspection = inspection_record(&identity, &scorecard, &claim, imported_handoff.as_ref());

        let packet = Self {
            record_kind: BUNDLE_ARCHETYPE_CERTIFICATION_PACKET_RECORD_KIND.to_string(),
            schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            scorecard,
            claim,
            imported_handoff,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_REF.to_string()],
            allows_hidden_setup_hooks: false,
            allows_secret_injection: false,
            allows_silent_trust_widening: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the certification invariants.
    ///
    /// # Errors
    ///
    /// Returns [`BundleCertificationValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), BundleCertificationValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            BUNDLE_ARCHETYPE_CERTIFICATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_scorecard(&self.scorecard)?;
        validate_claim(&self.claim)?;
        if let Some(handoff) = &self.imported_handoff {
            validate_handoff(handoff)?;
        }

        for surface in &self.consumer_surfaces {
            ensure_token(
                BUNDLE_CERTIFICATION_CONSUMER_SURFACES,
                surface,
                "consumer_surface",
            )?;
        }
        if self.consumer_surfaces.is_empty() {
            return Err(err("packet must bind at least one consumer surface"));
        }
        if !self
            .source_schema_refs
            .iter()
            .any(|r| r == BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No hidden authority may ride bundle application, even when certified.
        if self.allows_hidden_setup_hooks
            || self.allows_secret_injection
            || self.allows_silent_trust_widening
        {
            return Err(err(
                "bundle application must not allow hidden setup hooks, secret injection, or silent trust widening",
            ));
        }

        // Scorecard rows that reference this bundle must use the canonical id.
        for row in &self.scorecard.rows {
            if row.bundle_id_ref == self.identity.bundle_id {
                continue;
            }
            // Rows for other bundles are allowed (a shared scorecard), but the
            // referenced row must belong to this bundle (checked below).
        }

        // Stable-claim binding: a stable effective badge must resolve to a
        // current scorecard row that belongs to this bundle.
        if STABLE_BADGE_CLASSES.contains(&self.claim.effective_badge_class.as_str()) {
            if self.claim.claim_basis_class != "scorecard_row_backed" {
                return Err(err(
                    "stable effective badge must be backed by a scorecard row, not prose",
                ));
            }
            let row = self.resolved_scorecard_row().ok_or_else(|| {
                err("stable effective badge must resolve to a known scorecard row")
            })?;
            if row.bundle_id_ref != self.identity.bundle_id {
                return Err(err(
                    "stable claim scorecard row must belong to this bundle id",
                ));
            }
            if !row.is_current() {
                return Err(err(
                    "stable effective badge must resolve to a current scorecard row",
                ));
            }
            if !row.bridge_supports_parity() {
                return Err(err(
                    "stable effective badge must resolve to a row with exact or partial bridge parity",
                ));
            }
            if self.claim.asserts_reference_workspace
                && row.certified_reference_workspace_ids.is_empty()
            {
                return Err(err(
                    "stable claim asserting a reference workspace must resolve to a row that names one",
                ));
            }
            if self.claim.downgraded {
                return Err(err(
                    "a stable effective badge must not also be marked downgraded",
                ));
            }
        }

        // Downgrade truth: a downgraded claim must carry at least one reason and
        // must never keep a stable effective badge.
        if self.claim.downgraded {
            if self.claim.downgrade_reasons.is_empty() {
                return Err(err("a downgraded claim must carry at least one reason"));
            }
            if STABLE_BADGE_CLASSES.contains(&self.claim.effective_badge_class.as_str()) {
                return Err(err("a downgraded claim must not keep a stable badge"));
            }
        }

        // Recompute the effective badge and re-derive the downgrade verdict so
        // the stored claim cannot drift from the scorecard truth.
        let resolved = self.resolved_scorecard_row();
        let derived = derive_effective_badge(
            &self.claim.claimed_badge_class,
            &self.claim.claim_basis_class,
            self.claim.asserts_reference_workspace,
            &self.identity,
            resolved,
        );
        if derived.effective_badge != self.claim.effective_badge_class {
            return Err(err(
                "stored effective badge does not match the scorecard-derived badge",
            ));
        }
        if derived.downgraded != self.claim.downgraded {
            return Err(err(
                "stored downgrade flag does not match the scorecard-derived verdict",
            ));
        }
        let stored: BTreeSet<&str> =
            self.claim.downgrade_reasons.iter().map(String::as_str).collect();
        let expected: BTreeSet<&str> = derived.downgrade_reasons.iter().map(String::as_str).collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the scorecard-derived reasons",
            ));
        }

        // Imported-user handoff bundles must preserve a migration report.
        if self.identity.bundle_class == "imported_user_bundle" {
            let handoff = self.imported_handoff.as_ref().ok_or_else(|| {
                err("imported_user_bundle must carry a preserved imported-handoff report")
            })?;
            if !handoff.preserved {
                return Err(err(
                    "imported_user_bundle handoff report must be preserved, not collapsed",
                ));
            }
            if handoff.migration_report_ref.trim().is_empty() {
                return Err(err(
                    "imported_user_bundle must preserve a migration report reference",
                ));
            }
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns the scorecard row referenced by the claim, when it resolves.
    pub fn resolved_scorecard_row(&self) -> Option<&CompatibilityScorecardRow> {
        let row_ref = self.claim.scorecard_row_ref.as_deref()?;
        self.scorecard.rows.iter().find(|r| r.row_id == row_ref)
    }

    /// Returns true when no stable claim is implied from prose alone.
    pub fn no_prose_only_stable_claim(&self) -> bool {
        if STABLE_BADGE_CLASSES.contains(&self.claim.effective_badge_class.as_str()) {
            return self.claim.claim_basis_class == "scorecard_row_backed";
        }
        true
    }

    /// Returns true when offline/mirror distribution keeps full scorecard
    /// vocabulary instead of degrading to opaque archive import.
    pub fn offline_parity_preserved(&self) -> bool {
        // Every distribution class must carry a populated, machine-readable
        // scorecard. Offline and mirror flows are not allowed a shortcut.
        !self.scorecard.rows.is_empty() && !self.scorecard.scorecard_ref.trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleArchetypeCertificationProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Bundle id.
    pub bundle_id: String,
    /// Product bundle class.
    pub bundle_class: String,
    /// Archetype class.
    pub archetype_class: String,
    /// Claimed badge class.
    pub claimed_badge_class: String,
    /// Effective badge class after the scorecard is applied.
    pub effective_badge_class: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable handoff claim.
    pub stable_claim: bool,
    /// True when the claimed badge was downgraded.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when the effective stable claim resolves to a current scorecard row.
    pub resolves_to_current_scorecard_row: bool,
    /// Number of scorecard rows.
    pub scorecard_row_count: usize,
    /// True when imported-user handoff context is preserved (when applicable).
    pub imported_handoff_preserved: bool,
}

impl From<BundleArchetypeCertificationPacket> for BundleArchetypeCertificationProjection {
    fn from(packet: BundleArchetypeCertificationPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            bundle_id: packet.identity.bundle_id,
            bundle_class: packet.identity.bundle_class,
            archetype_class: packet.identity.archetype_class,
            claimed_badge_class: packet.claim.claimed_badge_class,
            effective_badge_class: packet.claim.effective_badge_class,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            resolves_to_current_scorecard_row: packet.inspection.resolves_to_current_scorecard_row,
            scorecard_row_count: packet.scorecard.rows.len(),
            imported_handoff_preserved: packet.inspection.imported_handoff_preserved,
        }
    }
}

/// Parses and validates a materialized certification packet.
///
/// # Errors
///
/// Returns [`BundleCertificationError`] when the payload fails to parse or
/// violates the certification invariants.
pub fn project_bundle_archetype_certification(
    payload: &str,
) -> Result<BundleArchetypeCertificationProjection, BundleCertificationError> {
    let packet: BundleArchetypeCertificationPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(BundleArchetypeCertificationProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for certification operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundleCertificationError {
    /// Validation failed.
    Validation(BundleCertificationValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for BundleCertificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for BundleCertificationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for certification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleCertificationValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for BundleCertificationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BundleCertificationValidationError {}

impl BundleCertificationValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for BundleCertificationError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(BundleCertificationValidationError {
            message: err.to_string(),
        })
    }
}

impl From<BundleCertificationValidationError> for BundleCertificationError {
    fn from(err: BundleCertificationValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-badge derivation (the automatic downgrade)
// ---------------------------------------------------------------------------

/// Outcome of applying a scorecard row to a claimed badge.
struct DerivedBadge {
    effective_badge: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Applies the scorecard row (or its absence) to a claimed badge, downgrading
/// automatically when the evidence no longer supports a stable claim.
fn derive_effective_badge(
    claimed_badge: &str,
    claim_basis: &str,
    asserts_reference_workspace: bool,
    identity: &CertifiedBundleIdentity,
    row: Option<&CompatibilityScorecardRow>,
) -> DerivedBadge {
    // Non-stable claims are already honest; they pass through unchanged.
    if !STABLE_BADGE_CLASSES.contains(&claimed_badge) {
        let support_claim = support_claim_for(claimed_badge, false);
        return DerivedBadge {
            effective_badge: claimed_badge.to_string(),
            support_claim,
            downgraded: false,
            downgrade_reasons: Vec::new(),
        };
    }

    let mut reasons: Vec<String> = Vec::new();

    if claim_basis != "scorecard_row_backed" {
        reasons.push("prose_only_claim".to_string());
    }
    if identity.certification_state_class == "certification_stale" {
        reasons.push("certification_state_stale".to_string());
    }

    match row {
        None => reasons.push("missing_scorecard_row".to_string()),
        Some(row) => {
            if row.bundle_id_ref != identity.bundle_id {
                reasons.push("scorecard_row_bundle_mismatch".to_string());
            }
            if row.freshness_class == "stale_past_window"
                || row.downgrade_state_class == "downgraded_freshness_expired"
            {
                reasons.push("scorecard_freshness_expired".to_string());
            }
            if !row.bridge_supports_parity()
                || row.downgrade_state_class == "downgraded_bridge_narrowed"
            {
                reasons.push("bridge_parity_narrowed".to_string());
            }
            if row.known_gap_state_class == "known_gaps_disclosed"
                || row.downgrade_state_class == "downgraded_gap_detected"
            {
                reasons.push("known_gap_detected".to_string());
            }
            if asserts_reference_workspace && row.certified_reference_workspace_ids.is_empty() {
                reasons.push("reference_workspace_missing".to_string());
            }
        }
    }

    reasons.sort();
    reasons.dedup();

    if reasons.is_empty() {
        // The stable badge holds.
        DerivedBadge {
            effective_badge: claimed_badge.to_string(),
            support_claim: support_claim_for(claimed_badge, false),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        // Choose the safest visible badge for the failure mode.
        let effective = downgrade_badge_for(&reasons);
        DerivedBadge {
            effective_badge: effective.to_string(),
            support_claim: support_claim_for(effective, true),
            downgraded: true,
            downgrade_reasons: reasons,
        }
    }
}

/// Picks the effective badge to render given the active downgrade reasons.
fn downgrade_badge_for(reasons: &[String]) -> &'static str {
    // Freshness expiry maps to an explicit retest-pending badge; every other
    // failure mode renders the conservative "limited" badge.
    if reasons.iter().any(|r| r == "scorecard_freshness_expired")
        || reasons.iter().any(|r| r == "certification_state_stale")
    {
        "retest_pending"
    } else {
        "limited"
    }
}

/// Maps an effective badge to the support claim it may imply.
fn support_claim_for(badge: &str, downgraded: bool) -> String {
    if downgraded {
        return "limited_retest_pending_claim".to_string();
    }
    match badge {
        "certified" => "stable_launch_wedge_claim",
        "managed_approved" => "managed_org_claim",
        "community" => "community_no_certification_claim",
        "imported" => "imported_pending_review_claim",
        "local_draft" => "local_draft_no_claim",
        _ => "limited_retest_pending_claim",
    }
    .to_string()
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &CertifiedBundleIdentityInput) -> CertifiedBundleIdentity {
    CertifiedBundleIdentity {
        record_kind: CERTIFIED_BUNDLE_IDENTITY_RECORD_KIND.to_string(),
        schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        bundle_id: input.bundle_id.clone(),
        bundle_revision: input.bundle_revision,
        bundle_class: input.bundle_class.clone(),
        bundle_source_class: input.bundle_source_class.clone(),
        archetype_class: input.archetype_class.clone(),
        signer_source_class: input.signer_source_class.clone(),
        signer_ref: input.signer_ref.clone(),
        compatible_aureline_range: input.compatible_aureline_range.clone(),
        certification_state_class: input.certification_state_class.clone(),
        distribution_class: input.distribution_class.clone(),
    }
}

fn scorecard_record(input: &CompatibilityScorecardInput) -> CompatibilityScorecard {
    CompatibilityScorecard {
        record_kind: COMPATIBILITY_SCORECARD_RECORD_KIND.to_string(),
        schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        scorecard_id: input.scorecard_id.clone(),
        scorecard_ref: input.scorecard_ref.clone(),
        generated_at: input.generated_at.clone(),
        rows: input.rows.iter().map(scorecard_row_record).collect(),
    }
}

fn scorecard_row_record(input: &CompatibilityScorecardRowInput) -> CompatibilityScorecardRow {
    CompatibilityScorecardRow {
        record_kind: COMPATIBILITY_SCORECARD_ROW_RECORD_KIND.to_string(),
        schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        row_id: input.row_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        imported_extension_class: input.imported_extension_class.clone(),
        bridge_state_class: input.bridge_state_class.clone(),
        supported_deployment_refs: input.supported_deployment_refs.clone(),
        supported_profile_refs: input.supported_profile_refs.clone(),
        certified_reference_workspace_ids: input.certified_reference_workspace_ids.clone(),
        freshness_class: input.freshness_class.clone(),
        freshness_expires_at: input.freshness_expires_at.clone(),
        downgrade_state_class: input.downgrade_state_class.clone(),
        known_gap_state_class: input.known_gap_state_class.clone(),
        known_gap_refs: input.known_gap_refs.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &BundleCertificationClaimInput,
    identity: &CertifiedBundleIdentity,
    scorecard: &CompatibilityScorecard,
) -> BundleCertificationClaim {
    let row = input
        .scorecard_row_ref
        .as_deref()
        .and_then(|r| scorecard.rows.iter().find(|row| row.row_id == r));
    let derived = derive_effective_badge(
        &input.claimed_badge_class,
        &input.claim_basis_class,
        input.asserts_reference_workspace,
        identity,
        row,
    );
    BundleCertificationClaim {
        record_kind: BUNDLE_CERTIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        claimed_badge_class: input.claimed_badge_class.clone(),
        effective_badge_class: derived.effective_badge,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        scorecard_row_ref: input.scorecard_row_ref.clone(),
        asserts_reference_workspace: input.asserts_reference_workspace,
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn handoff_record(
    input: &ImportedHandoffReportInput,
    _identity: &CertifiedBundleIdentity,
) -> ImportedHandoffReport {
    ImportedHandoffReport {
        record_kind: IMPORTED_HANDOFF_REPORT_RECORD_KIND.to_string(),
        schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        handoff_id: input.handoff_id.clone(),
        migration_report_ref: input.migration_report_ref.clone(),
        unsupported_item_refs: input.unsupported_item_refs.clone(),
        partial_item_refs: input.partial_item_refs.clone(),
        preserved: !input.migration_report_ref.trim().is_empty(),
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    identity: &CertifiedBundleIdentity,
    scorecard: &CompatibilityScorecard,
    claim: &BundleCertificationClaim,
    handoff: Option<&ImportedHandoffReport>,
) -> BundleArchetypeCertificationInspection {
    let stable_claim = STABLE_BADGE_CLASSES.contains(&claim.effective_badge_class.as_str());
    let resolved_row = claim
        .scorecard_row_ref
        .as_deref()
        .and_then(|r| scorecard.rows.iter().find(|row| row.row_id == r));
    // True only when the referenced row genuinely belongs to this bundle and is
    // still current with parity-supporting bridge state — the condition a stable
    // claim depends on. A downgrade flips this to false, which is the signal.
    let resolves_to_current_scorecard_row = resolved_row
        .map(|row| {
            row.bundle_id_ref == identity.bundle_id
                && row.is_current()
                && row.bridge_supports_parity()
        })
        .unwrap_or(false);
    let no_prose_only_stable_claim = !stable_claim || claim.claim_basis_class == "scorecard_row_backed";
    let imported_handoff_preserved = if identity.bundle_class == "imported_user_bundle" {
        handoff.map(|h| h.preserved).unwrap_or(false)
    } else {
        true
    };

    BundleArchetypeCertificationInspection {
        record_kind: BUNDLE_ARCHETYPE_CERTIFICATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        bundle_id_ref: identity.bundle_id.clone(),
        effective_badge_class: claim.effective_badge_class.clone(),
        stable_claim,
        resolves_to_current_scorecard_row,
        downgraded: claim.downgraded,
        no_prose_only_stable_claim,
        identity_consistent: identity_is_consistent(identity),
        imported_handoff_preserved,
        offline_parity_preserved: !scorecard.rows.is_empty()
            && !scorecard.scorecard_ref.trim().is_empty(),
        no_hidden_authority_widening: true,
        scorecard_row_count: scorecard.rows.len(),
        summary_label: claim.summary_label.clone(),
    }
}

fn identity_is_consistent(identity: &CertifiedBundleIdentity) -> bool {
    !identity.bundle_id.trim().is_empty()
        && contains(BUNDLE_CLASSES, &identity.bundle_class)
        && contains(BUNDLE_SOURCE_CLASSES, &identity.bundle_source_class)
        && contains(BUNDLE_ARCHETYPE_CLASSES, &identity.archetype_class)
        && contains(CERTIFICATION_STATE_CLASSES, &identity.certification_state_class)
        && !identity.compatible_aureline_range.trim().is_empty()
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &BundleArchetypeCertificationInput,
) -> Result<(), BundleCertificationValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.bundle_id, "identity.bundle_id")?;
    ensure_token(BUNDLE_CLASSES, &id.bundle_class, "identity.bundle_class")?;
    ensure_token(
        BUNDLE_SOURCE_CLASSES,
        &id.bundle_source_class,
        "identity.bundle_source_class",
    )?;
    ensure_token(
        BUNDLE_ARCHETYPE_CLASSES,
        &id.archetype_class,
        "identity.archetype_class",
    )?;
    ensure_token(
        CERTIFICATION_STATE_CLASSES,
        &id.certification_state_class,
        "identity.certification_state_class",
    )?;
    ensure_token(
        BUNDLE_DISTRIBUTION_CLASSES,
        &id.distribution_class,
        "identity.distribution_class",
    )?;
    ensure_nonempty(&id.signer_ref, "identity.signer_ref")?;
    ensure_nonempty(
        &id.compatible_aureline_range,
        "identity.compatible_aureline_range",
    )?;

    let sc = &input.scorecard;
    ensure_nonempty(&sc.scorecard_id, "scorecard.scorecard_id")?;
    ensure_nonempty(&sc.scorecard_ref, "scorecard.scorecard_ref")?;
    if sc.rows.is_empty() {
        return Err(err(
            "scorecard must contain at least one row; offline/mirror flows may not degrade to opaque import",
        ));
    }
    let mut row_ids = BTreeSet::new();
    for row in &sc.rows {
        ensure_nonempty(&row.row_id, "scorecard_row.row_id")?;
        if !row_ids.insert(&row.row_id) {
            return Err(err(format!("duplicate scorecard row_id: {}", row.row_id)));
        }
        ensure_nonempty(&row.bundle_id_ref, "scorecard_row.bundle_id_ref")?;
        ensure_token(
            IMPORTED_EXTENSION_CLASSES,
            &row.imported_extension_class,
            "scorecard_row.imported_extension_class",
        )?;
        ensure_token(
            BRIDGE_STATE_CLASSES,
            &row.bridge_state_class,
            "scorecard_row.bridge_state_class",
        )?;
        ensure_token(
            SCORECARD_FRESHNESS_CLASSES,
            &row.freshness_class,
            "scorecard_row.freshness_class",
        )?;
        ensure_token(
            SCORECARD_DOWNGRADE_STATE_CLASSES,
            &row.downgrade_state_class,
            "scorecard_row.downgrade_state_class",
        )?;
        ensure_token(
            KNOWN_GAP_STATE_CLASSES,
            &row.known_gap_state_class,
            "scorecard_row.known_gap_state_class",
        )?;
        ensure_nonempty(&row.freshness_expires_at, "scorecard_row.freshness_expires_at")?;
        // A disclosed gap must list at least one gap reference.
        if row.known_gap_state_class == "known_gaps_disclosed" && row.known_gap_refs.is_empty() {
            return Err(err(format!(
                "scorecard row {} discloses gaps but lists none",
                row.row_id
            )));
        }
    }

    let claim = &input.claim;
    ensure_token(
        EFFECTIVE_BADGE_CLASSES,
        &claim.claimed_badge_class,
        "claim.claimed_badge_class",
    )?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim.claim_basis_class")?;
    if let Some(row_ref) = &claim.scorecard_row_ref {
        if !sc.rows.iter().any(|r| &r.row_id == row_ref) {
            return Err(err(format!(
                "claim.scorecard_row_ref {row_ref} does not resolve to a scorecard row"
            )));
        }
    }
    // A stable claimed badge must at least point at a row to be considered.
    if STABLE_BADGE_CLASSES.contains(&claim.claimed_badge_class.as_str())
        && claim.claim_basis_class == "scorecard_row_backed"
        && claim.scorecard_row_ref.is_none()
    {
        return Err(err(
            "a scorecard-backed stable claim must name a scorecard_row_ref",
        ));
    }

    if let Some(handoff) = &input.imported_handoff {
        ensure_nonempty(&handoff.handoff_id, "imported_handoff.handoff_id")?;
    }
    if id.bundle_class == "imported_user_bundle" {
        let handoff = input.imported_handoff.as_ref().ok_or_else(|| {
            err("imported_user_bundle must carry an imported-handoff report")
        })?;
        ensure_nonempty(
            &handoff.migration_report_ref,
            "imported_handoff.migration_report_ref",
        )?;
    }

    for surface in &input.consumer_surfaces {
        ensure_token(
            BUNDLE_CERTIFICATION_CONSUMER_SURFACES,
            surface,
            "consumer_surface",
        )?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(
    identity: &CertifiedBundleIdentity,
) -> Result<(), BundleCertificationValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        CERTIFIED_BUNDLE_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        BUNDLE_ARCHETYPE_CERTIFICATION_SCHEMA_VERSION,
        "identity schema_version",
    )?;
    ensure_token(BUNDLE_CLASSES, &identity.bundle_class, "identity bundle_class")?;
    ensure_token(
        BUNDLE_SOURCE_CLASSES,
        &identity.bundle_source_class,
        "identity bundle_source_class",
    )?;
    ensure_token(
        BUNDLE_ARCHETYPE_CLASSES,
        &identity.archetype_class,
        "identity archetype_class",
    )?;
    ensure_token(
        CERTIFICATION_STATE_CLASSES,
        &identity.certification_state_class,
        "identity certification_state_class",
    )?;
    ensure_token(
        BUNDLE_DISTRIBUTION_CLASSES,
        &identity.distribution_class,
        "identity distribution_class",
    )?;
    Ok(())
}

fn validate_scorecard(
    scorecard: &CompatibilityScorecard,
) -> Result<(), BundleCertificationValidationError> {
    ensure_eq(
        scorecard.record_kind.as_str(),
        COMPATIBILITY_SCORECARD_RECORD_KIND,
        "scorecard record_kind",
    )?;
    if scorecard.rows.is_empty() {
        return Err(err("scorecard must contain at least one row"));
    }
    for row in &scorecard.rows {
        ensure_eq(
            row.record_kind.as_str(),
            COMPATIBILITY_SCORECARD_ROW_RECORD_KIND,
            "scorecard row record_kind",
        )?;
        ensure_token(
            IMPORTED_EXTENSION_CLASSES,
            &row.imported_extension_class,
            "scorecard row imported_extension_class",
        )?;
        ensure_token(
            BRIDGE_STATE_CLASSES,
            &row.bridge_state_class,
            "scorecard row bridge_state_class",
        )?;
        ensure_token(
            SCORECARD_FRESHNESS_CLASSES,
            &row.freshness_class,
            "scorecard row freshness_class",
        )?;
        ensure_token(
            SCORECARD_DOWNGRADE_STATE_CLASSES,
            &row.downgrade_state_class,
            "scorecard row downgrade_state_class",
        )?;
        ensure_token(
            KNOWN_GAP_STATE_CLASSES,
            &row.known_gap_state_class,
            "scorecard row known_gap_state_class",
        )?;
    }
    Ok(())
}

fn validate_claim(
    claim: &BundleCertificationClaim,
) -> Result<(), BundleCertificationValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        BUNDLE_CERTIFICATION_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(
        EFFECTIVE_BADGE_CLASSES,
        &claim.claimed_badge_class,
        "claim claimed_badge_class",
    )?;
    ensure_token(
        EFFECTIVE_BADGE_CLASSES,
        &claim.effective_badge_class,
        "claim effective_badge_class",
    )?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim claim_basis_class")?;
    for reason in &claim.downgrade_reasons {
        ensure_token(
            BUNDLE_CERTIFICATION_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_handoff(
    handoff: &ImportedHandoffReport,
) -> Result<(), BundleCertificationValidationError> {
    ensure_eq(
        handoff.record_kind.as_str(),
        IMPORTED_HANDOFF_REPORT_RECORD_KIND,
        "handoff record_kind",
    )?;
    ensure_nonempty(&handoff.handoff_id, "handoff handoff_id")?;
    if handoff.preserved && handoff.migration_report_ref.trim().is_empty() {
        return Err(err(
            "a preserved handoff report must keep its migration_report_ref",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &BundleArchetypeCertificationInspection,
    packet: &BundleArchetypeCertificationPacket,
) -> Result<(), BundleCertificationValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        BUNDLE_ARCHETYPE_CERTIFICATION_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.bundle_id_ref.as_str(),
        packet.identity.bundle_id.as_str(),
        "inspection bundle_id_ref",
    )?;
    ensure_eq(
        inspection.effective_badge_class.as_str(),
        packet.claim.effective_badge_class.as_str(),
        "inspection effective_badge_class",
    )?;
    if inspection.scorecard_row_count != packet.scorecard.rows.len() {
        return Err(err("inspection scorecard_row_count must match scorecard rows"));
    }
    if inspection.no_prose_only_stable_claim != packet.no_prose_only_stable_claim() {
        return Err(err("inspection no_prose_only_stable_claim is inconsistent"));
    }
    if inspection.offline_parity_preserved != packet.offline_parity_preserved() {
        return Err(err("inspection offline_parity_preserved is inconsistent"));
    }
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> BundleCertificationValidationError {
    BundleCertificationValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), BundleCertificationValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), BundleCertificationValidationError> {
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), BundleCertificationValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), BundleCertificationValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!("{field} must be one of {tokens:?}, got {value}")));
    }
    Ok(())
}

fn contains(tokens: &[&str], value: &str) -> bool {
    tokens.contains(&value)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: &str, bundle: &str) -> CompatibilityScorecardRowInput {
        CompatibilityScorecardRowInput {
            row_id: id.to_string(),
            bundle_id_ref: bundle.to_string(),
            imported_extension_class: "mapped_native".to_string(),
            bridge_state_class: "exact_bridge".to_string(),
            supported_deployment_refs: vec!["deploy.local".to_string()],
            supported_profile_refs: vec!["profile.default".to_string()],
            certified_reference_workspace_ids: vec!["refws.tsjs.alpha".to_string()],
            freshness_class: "fresh_current".to_string(),
            freshness_expires_at: "2026-12-31T00:00:00Z".to_string(),
            downgrade_state_class: "no_downgrade".to_string(),
            known_gap_state_class: "no_known_gaps".to_string(),
            known_gap_refs: vec![],
            summary_label: "Row".to_string(),
        }
    }

    fn identity(class: &str, source: &str, state: &str) -> CertifiedBundleIdentityInput {
        CertifiedBundleIdentityInput {
            bundle_id: "bundle.launch.tsjs".to_string(),
            bundle_revision: 1,
            bundle_class: class.to_string(),
            bundle_source_class: source.to_string(),
            archetype_class: "tsjs_web".to_string(),
            signer_source_class: "aureline_certified".to_string(),
            signer_ref: "signer.aureline".to_string(),
            compatible_aureline_range: ">=0.4, <0.5".to_string(),
            certification_state_class: state.to_string(),
            distribution_class: "public_registry".to_string(),
        }
    }

    fn base_input() -> BundleArchetypeCertificationInput {
        BundleArchetypeCertificationInput {
            packet_id: "pkt1".to_string(),
            generated_at: "2026-05-31T10:00:00Z".to_string(),
            identity: identity("launch_bundle", "certified", "certified_current"),
            scorecard: CompatibilityScorecardInput {
                scorecard_id: "sc1".to_string(),
                scorecard_ref: "artifacts/compat/bundle_scorecard_alpha.json".to_string(),
                generated_at: "2026-05-31T09:00:00Z".to_string(),
                rows: vec![row("r1", "bundle.launch.tsjs")],
            },
            claim: BundleCertificationClaimInput {
                claimed_badge_class: "certified".to_string(),
                claim_basis_class: "scorecard_row_backed".to_string(),
                scorecard_row_ref: Some("r1".to_string()),
                asserts_reference_workspace: true,
                summary_label: "Claim".to_string(),
            },
            imported_handoff: None,
            consumer_surfaces: vec![
                "start_center_bundle_card".to_string(),
                "support_export".to_string(),
            ],
            summary_label: "Certified launch bundle".to_string(),
        }
    }

    #[test]
    fn closed_vocabularies_hold_their_anchors() {
        assert!(BUNDLE_CLASSES.contains(&"launch_bundle"));
        assert!(BUNDLE_CLASSES.contains(&"imported_user_bundle"));
        assert!(BUNDLE_CLASSES.contains(&"org_approved_bundle"));
        assert!(BUNDLE_SOURCE_CLASSES.contains(&"certified"));
        assert!(BUNDLE_SOURCE_CLASSES.contains(&"managed_approved"));
        assert!(EFFECTIVE_BADGE_CLASSES.contains(&"limited"));
        assert!(EFFECTIVE_BADGE_CLASSES.contains(&"retest_pending"));
        assert!(STABLE_BADGE_CLASSES.contains(&"certified"));
        assert!(STABLE_BADGE_CLASSES.contains(&"managed_approved"));
        // Stable badges must be a strict subset of the effective badge set.
        assert!(STABLE_BADGE_CLASSES
            .iter()
            .all(|b| EFFECTIVE_BADGE_CLASSES.contains(b)));
        assert!(BUNDLE_CERTIFICATION_DOWNGRADE_REASONS.contains(&"scorecard_freshness_expired"));
        assert!(BUNDLE_CERTIFICATION_DOWNGRADE_REASONS.contains(&"bridge_parity_narrowed"));
        assert!(BRIDGE_STATE_CLASSES.contains(&"approximate_bridge"));
    }

    #[test]
    fn certified_claim_with_current_row_holds() {
        let packet = BundleArchetypeCertificationPacket::from_input(base_input())
            .expect("must project");
        assert_eq!(packet.claim.effective_badge_class, "certified");
        assert!(!packet.claim.downgraded);
        assert_eq!(packet.claim.support_claim_class, "stable_launch_wedge_claim");
        assert!(packet.inspection.resolves_to_current_scorecard_row);
        assert!(packet.no_prose_only_stable_claim());
    }

    #[test]
    fn stale_row_downgrades_automatically() {
        let mut input = base_input();
        input.scorecard.rows[0].freshness_class = "stale_past_window".to_string();
        let packet = BundleArchetypeCertificationPacket::from_input(input)
            .expect("must project even when downgraded");
        assert!(packet.claim.downgraded);
        assert_eq!(packet.claim.effective_badge_class, "retest_pending");
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"scorecard_freshness_expired".to_string()));
        assert!(!packet.inspection.stable_claim);
    }

    #[test]
    fn narrowed_bridge_downgrades_automatically() {
        let mut input = base_input();
        input.scorecard.rows[0].bridge_state_class = "approximate_bridge".to_string();
        let packet = BundleArchetypeCertificationPacket::from_input(input)
            .expect("must project");
        assert!(packet.claim.downgraded);
        assert_eq!(packet.claim.effective_badge_class, "limited");
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"bridge_parity_narrowed".to_string()));
    }

    #[test]
    fn prose_only_stable_claim_is_downgraded() {
        let mut input = base_input();
        input.claim.claim_basis_class = "prose_only".to_string();
        input.claim.scorecard_row_ref = None;
        let packet = BundleArchetypeCertificationPacket::from_input(input)
            .expect("must project");
        assert!(packet.claim.downgraded);
        assert!(!STABLE_BADGE_CLASSES.contains(&packet.claim.effective_badge_class.as_str()));
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"prose_only_claim".to_string()));
    }

    #[test]
    fn missing_row_downgrades_stable_claim() {
        let mut input = base_input();
        input.claim.scorecard_row_ref = Some("r1".to_string());
        // Point the claim at a row id that is not present is rejected at input;
        // instead drop the row's bundle binding to force a mismatch downgrade.
        input.scorecard.rows[0].bundle_id_ref = "bundle.other".to_string();
        let packet = BundleArchetypeCertificationPacket::from_input(input)
            .expect("must project");
        assert!(packet.claim.downgraded);
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"scorecard_row_bundle_mismatch".to_string()));
    }

    #[test]
    fn imported_user_bundle_requires_preserved_handoff() {
        let mut input = base_input();
        input.identity = identity("imported_user_bundle", "imported", "imported_pending_review");
        input.claim.claimed_badge_class = "imported".to_string();
        input.claim.claim_basis_class = "scorecard_row_backed".to_string();
        input.claim.scorecard_row_ref = Some("r1".to_string());
        input.claim.asserts_reference_workspace = false;
        // No handoff -> input validation rejects.
        let result = BundleArchetypeCertificationPacket::from_input(input.clone());
        assert!(result.is_err());

        input.imported_handoff = Some(ImportedHandoffReportInput {
            handoff_id: "ho1".to_string(),
            migration_report_ref: "fixtures/migration/report.json".to_string(),
            unsupported_item_refs: vec!["ext.unsupported".to_string()],
            partial_item_refs: vec![],
            summary_label: "Handoff".to_string(),
        });
        let packet = BundleArchetypeCertificationPacket::from_input(input).expect("must project");
        assert!(packet.imported_handoff.as_ref().unwrap().preserved);
        assert!(packet.inspection.imported_handoff_preserved);
    }

    #[test]
    fn offline_distribution_keeps_scorecard_vocabulary() {
        let mut input = base_input();
        input.identity.distribution_class = "offline_archive".to_string();
        let packet = BundleArchetypeCertificationPacket::from_input(input).expect("must project");
        assert!(packet.offline_parity_preserved());
        assert!(packet.inspection.offline_parity_preserved);
    }

    #[test]
    fn empty_scorecard_is_rejected() {
        let mut input = base_input();
        input.scorecard.rows.clear();
        let result = BundleArchetypeCertificationPacket::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn managed_approved_claim_holds_with_current_row() {
        let mut input = base_input();
        input.identity = identity("org_approved_bundle", "managed_approved", "managed_approved_current");
        input.claim.claimed_badge_class = "managed_approved".to_string();
        let packet = BundleArchetypeCertificationPacket::from_input(input).expect("must project");
        assert_eq!(packet.claim.effective_badge_class, "managed_approved");
        assert_eq!(packet.claim.support_claim_class, "managed_org_claim");
        assert!(!packet.claim.downgraded);
    }

    #[test]
    fn projection_roundtrips_through_json() {
        let packet = BundleArchetypeCertificationPacket::from_input(base_input())
            .expect("must project");
        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection =
            project_bundle_archetype_certification(&payload).expect("must project from json");
        assert_eq!(projection.bundle_id, "bundle.launch.tsjs");
        assert_eq!(projection.effective_badge_class, "certified");
        assert!(projection.resolves_to_current_scorecard_row);
    }

    #[test]
    fn reference_workspace_missing_downgrades() {
        let mut input = base_input();
        input.scorecard.rows[0].certified_reference_workspace_ids.clear();
        let packet = BundleArchetypeCertificationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"reference_workspace_missing".to_string()));
    }
}
