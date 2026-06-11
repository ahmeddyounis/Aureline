//! Shared ecosystem compatibility scorecards for marketplace, migration,
//! workflow-bundle, support-export, and release-claim surfaces.
//!
//! This crate defines one machine-readable scorecard row that stable-facing
//! ecosystem claims must resolve through. The row keeps parity band, freshness,
//! evidence source, supported deployment and runtime profiles, reference
//! workspace linkage, downgrade rules, and migration or replacement guidance in
//! one canonical record so consumer surfaces reuse the same truth instead of
//! drifting into prose.
//!
//! The stable line relies on three rules:
//!
//! - stable-facing rows must cite reference-workspace lineage and current
//!   evidence;
//! - consumer surfaces must project directly from a canonical row rather than
//!   retyping parity, freshness, or linkage state; and
//! - stale evidence, narrowed bridge parity, missing evidence, or expired
//!   certification must automatically narrow the effective parity band.
//!
//! The
//! [`freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix`]
//! module owns the canonical M5 ecosystem install-governance matrix. It freezes one
//! row per marketed M5 artifact family — first-party framework packs, docs packs,
//! local-model packs, signed recipe packs, template artifacts, bridge-backed
//! packages, side-loaded packages, and mirrored/private-registry variants — each
//! naming its own source class, runtime origin, compatibility label, permission-
//! manifest state, activation-budget band, lifecycle state, evidence freshness, and
//! rollback posture. A non-inheriting promotion gate recomputes the support class a
//! family may publish, so a family with unverified provenance, stale evidence, an
//! unreviewed permission expansion, an exceeded activation budget, an unsupported
//! target, an incomplete rollback, or an active quarantine narrows or fails
//! promotion automatically instead of inheriting trust from an adjacent first-party
//! family.
//!
//! The [`m5_marketplace_fact_views`] module projects that governance truth into the
//! marketplace presentation layer: source-aware result rows, per-listing detail fact
//! grids, and side-by-side compare views that keep package kind, source class,
//! lifecycle state, support class, evidence freshness, runtime origin, bridge/native
//! state, and mirror/private-registry posture explicit across the public registry,
//! enterprise mirror, private registry, and manual-import discovery flows. Its
//! disclosure level is recomputed from each row's facts so reduced provenance widens
//! warnings rather than collapsing fields.
//!
//! The [`m5_install_review`] module turns install and update from a generic download
//! action into one reviewed change model. Each sheet compares a package's current
//! effective revision with the proposed one and makes permission deltas, transitive
//! capability widening, runtime-origin and host-class changes, publisher continuity,
//! compatibility-floor regressions, restart/open-work implications, and a rollback
//! plan explicit on one surface. Its commit disposition is recomputed from those
//! facts, so newly widened permissions, a changed publisher, or a changed runtime
//! origin always force the unified review sheet rather than a one-click commit, and
//! workspace-, profile-, and global-scope actions stay visibly distinct.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub mod freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix;
pub mod m5_install_review;
pub mod m5_marketplace_fact_views;

/// Supported schema version for ecosystem compatibility packets and projections.
pub const ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION: u32 = 1;

/// Canonical schema path cited by packets.
pub const ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_REF: &str =
    "schemas/ecosystem/compatibility_scorecard.schema.json";

const PACKET_RECORD_KIND: &str = "ecosystem_compatibility_scorecard_packet";
const ROW_RECORD_KIND: &str = "ecosystem_compatibility_scorecard_row";
const CLAIM_PROJECTION_RECORD_KIND: &str = "ecosystem_compatibility_claim_projection";
const INSPECTION_RECORD_KIND: &str = "ecosystem_compatibility_scorecard_inspection";

const CLAIMED_SURFACE_CLASSES: &[&str] = &[
    "imported_extension_claim",
    "compatibility_bridge_claim",
    "workflow_bundle_claim",
    "reference_workspace_claim",
];

const SUBJECT_CLASSES: &[&str] = &[
    "imported_extension",
    "bridge_class",
    "workflow_bundle",
    "reference_workspace",
];

const PARITY_BAND_CLASSES: &[&str] = &[
    "stable",
    "limited",
    "preview",
    "retest_pending",
    "unsupported",
];

const BRIDGE_PARITY_CLASSES: &[&str] = &[
    "exact",
    "partial",
    "approximate",
    "unsupported",
    "not_applicable",
];

const FRESHNESS_CLASSES: &[&str] = &["current", "aging", "stale", "expired", "missing"];

const EVIDENCE_SOURCE_CLASSES: &[&str] = &[
    "reference_workspace_report",
    "archetype_certification",
    "bridge_matrix",
    "conformance_suite",
    "migration_fixture",
];

const REFERENCE_WORKSPACE_CERTIFICATION_STATE_CLASSES: &[&str] =
    &["current", "expired", "missing", "not_required"];

const KNOWN_GAP_STATE_CLASSES: &[&str] = &["none", "disclosed", "unknown"];

const DOWNGRADE_RULE_CLASSES: &[&str] = &[
    "evidence_freshness_expired",
    "evidence_missing",
    "bridge_parity_narrowed",
    "bridge_unsupported",
    "reference_workspace_certification_expired",
    "reference_workspace_certification_missing",
    "known_gap_disclosed",
    "known_gap_unknown",
];

const DOWNGRADE_STATE_CLASSES: &[&str] = &[
    "none",
    "freshness_expired",
    "bridge_narrowed",
    "reference_workspace_narrowed",
    "known_gap_narrowed",
    "unsupported",
];

const CONSUMER_SURFACE_CLASSES: &[&str] = &[
    "marketplace_card",
    "migration_center_report",
    "bridge_detail_view",
    "bundle_detail_view",
    "support_export",
    "release_claim_manifest",
];

/// Input describing one ecosystem compatibility packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemCompatibilityPacketInput {
    /// Stable packet identifier used by fixtures and consumer projections.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the packet content.
    pub scorecard_revision: u32,
    /// Canonical scorecard rows that back ecosystem claims.
    pub rows: Vec<EcosystemCompatibilityRowInput>,
    /// Consumer claims that must project directly from canonical rows.
    pub consumer_claims: Vec<EcosystemConsumerClaimInput>,
    /// Reviewable packet summary safe for docs, release, and support surfaces.
    pub summary_label: String,
}

/// Input describing one canonical compatibility scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemCompatibilityRowInput {
    /// Stable row identifier.
    pub row_id: String,
    /// Claimed surface family protected by this row.
    pub claimed_surface_class: String,
    /// Subject family the row speaks for.
    pub subject_class: String,
    /// Stable subject reference safe to log and export.
    pub subject_ref: String,
    /// Human-readable subject label.
    pub subject_label: String,
    /// Claimed parity band before downgrade automation is applied.
    pub claimed_parity_band_class: String,
    /// Bridge parity that caps the effective parity band.
    pub bridge_parity_class: String,
    /// Evidence freshness state that caps the effective parity band.
    pub freshness_class: String,
    /// Evidence source family cited by the row.
    pub evidence_source_class: String,
    /// Opaque reference to the evidence source.
    pub evidence_source_ref: String,
    /// Supported deployment profile rows reused across consumer surfaces.
    #[serde(default)]
    pub supported_deployment_profile_refs: Vec<String>,
    /// Supported runtime profile rows reused across consumer surfaces.
    #[serde(default)]
    pub supported_runtime_profile_refs: Vec<String>,
    /// Reference workspace identifiers backing the row.
    #[serde(default)]
    pub reference_workspace_ids: Vec<String>,
    /// Reference-workspace lineage or certification refs backing the row.
    #[serde(default)]
    pub reference_workspace_lineage_refs: Vec<String>,
    /// Certification state for the linked reference workspace.
    pub reference_workspace_certification_state_class: String,
    /// Known-gap state for the row.
    pub known_gap_state_class: String,
    /// Known-gap refs linked from the row.
    #[serde(default)]
    pub known_gap_refs: Vec<String>,
    /// Closed downgrade rules that may narrow the row.
    #[serde(default)]
    pub downgrade_rule_classes: Vec<String>,
    /// Replacement guidance for a narrowed or unsupported claim, when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_guidance_ref: Option<String>,
    /// Migration guidance for a switching or import claim, when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_guidance_ref: Option<String>,
    /// Workflow bundles that inherit this row.
    #[serde(default)]
    pub linked_bundle_refs: Vec<String>,
    /// Imported-user handoff bundles that inherit this row.
    #[serde(default)]
    pub linked_handoff_bundle_refs: Vec<String>,
    /// Reviewable summary safe for product and export surfaces.
    pub summary_label: String,
}

/// Input describing one consumer claim projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemConsumerClaimInput {
    /// Stable projection identifier.
    pub claim_id: String,
    /// Consumer surface that renders this projection.
    pub consumer_surface_class: String,
    /// Canonical row the consumer must resolve through.
    pub row_ref: String,
    /// Reviewable summary safe for UI, docs, and support surfaces.
    pub summary_label: String,
}

/// Canonical ecosystem compatibility packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemCompatibilityPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the packet content.
    pub scorecard_revision: u32,
    /// Source schemas cited by the packet.
    pub source_schema_refs: Vec<String>,
    /// Canonical scorecard rows that back ecosystem claims.
    pub rows: Vec<EcosystemCompatibilityRow>,
    /// Consumer projections that must mirror canonical rows exactly.
    pub consumer_claims: Vec<EcosystemCompatibilityClaimProjection>,
    /// Compact inspection row for release and fixture validation.
    pub inspection: EcosystemCompatibilityInspection,
    /// Reviewable packet summary safe for docs, release, and support surfaces.
    pub summary_label: String,
}

impl EcosystemCompatibilityPacket {
    /// Builds a packet from input and derives all consumer projections.
    ///
    /// # Errors
    ///
    /// Returns [`EcosystemCompatibilityError`] when the input violates scorecard
    /// or consumer-projection invariants.
    pub fn from_input(
        input: EcosystemCompatibilityPacketInput,
    ) -> Result<Self, EcosystemCompatibilityError> {
        validate_input(&input)?;

        let rows: Vec<EcosystemCompatibilityRow> = input
            .rows
            .iter()
            .map(EcosystemCompatibilityRow::from_input)
            .collect();
        let row_map: BTreeMap<&str, &EcosystemCompatibilityRow> =
            rows.iter().map(|row| (row.row_id.as_str(), row)).collect();
        let consumer_claims: Vec<EcosystemCompatibilityClaimProjection> = input
            .consumer_claims
            .iter()
            .map(|claim| {
                let row = row_map
                    .get(claim.row_ref.as_str())
                    .copied()
                    .ok_or_else(|| err("consumer claim row_ref must resolve to a canonical row"))?;
                EcosystemCompatibilityClaimProjection::from_input(claim, row)
            })
            .collect::<Result<_, _>>()?;
        let inspection = EcosystemCompatibilityInspection::from_rows(&rows, &consumer_claims);

        let packet = Self {
            record_kind: PACKET_RECORD_KIND.to_string(),
            schema_version: ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            scorecard_revision: input.scorecard_revision,
            source_schema_refs: vec![ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_REF.to_string()],
            rows,
            consumer_claims,
            inspection,
            summary_label: input.summary_label,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates the packet and re-derives inspection and claim projections.
    ///
    /// # Errors
    ///
    /// Returns [`EcosystemCompatibilityError`] when a packet violates the
    /// shared scorecard contract.
    pub fn validate(&self) -> Result<(), EcosystemCompatibilityError> {
        ensure_eq(self.record_kind.as_str(), PACKET_RECORD_KIND, "record_kind")?;
        ensure_eq_u32(
            self.schema_version,
            ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;
        ensure_nonempty(&self.summary_label, "summary_label")?;
        if self.scorecard_revision == 0 {
            return Err(err("scorecard_revision must be at least 1"));
        }
        if !self
            .source_schema_refs
            .iter()
            .any(|entry| entry == ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_REF)
        {
            return Err(err("packet must cite the ecosystem compatibility schema"));
        }

        if self.rows.is_empty() {
            return Err(err("packet must contain at least one canonical row"));
        }
        if self.consumer_claims.is_empty() {
            return Err(err("packet must contain at least one consumer claim"));
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            row.validate()?;
            if !row_ids.insert(row.row_id.as_str()) {
                return Err(err("row_id values must be unique"));
            }
        }

        let row_map: BTreeMap<&str, &EcosystemCompatibilityRow> = self
            .rows
            .iter()
            .map(|row| (row.row_id.as_str(), row))
            .collect();
        let mut claim_ids = BTreeSet::new();
        for claim in &self.consumer_claims {
            claim.validate()?;
            if !claim_ids.insert(claim.claim_id.as_str()) {
                return Err(err("claim_id values must be unique"));
            }
            let row = row_map
                .get(claim.row_ref.as_str())
                .copied()
                .ok_or_else(|| err("consumer claim row_ref must resolve to a canonical row"))?;
            claim.validate_against_row(row)?;
        }

        for required_surface in CONSUMER_SURFACE_CLASSES {
            if !self
                .consumer_claims
                .iter()
                .any(|claim| claim.consumer_surface_class == *required_surface)
            {
                return Err(err(
                    "packet must prove marketplace, migration, bridge, bundle, support, and release consumers",
                ));
            }
        }

        let derived =
            EcosystemCompatibilityInspection::from_rows(&self.rows, &self.consumer_claims);
        if derived != self.inspection {
            return Err(err(
                "stored inspection row does not match the canonical rows and consumer claims",
            ));
        }
        Ok(())
    }
}

/// One canonical ecosystem compatibility scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemCompatibilityRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable row identifier.
    pub row_id: String,
    /// Claimed surface family protected by this row.
    pub claimed_surface_class: String,
    /// Subject family the row speaks for.
    pub subject_class: String,
    /// Stable subject reference safe to log and export.
    pub subject_ref: String,
    /// Human-readable subject label.
    pub subject_label: String,
    /// Claimed parity band before downgrade automation is applied.
    pub claimed_parity_band_class: String,
    /// Effective parity band after freshness, bridge, certification, and
    /// known-gap caps are applied.
    pub effective_parity_band_class: String,
    /// Bridge parity that caps the effective parity band.
    pub bridge_parity_class: String,
    /// Evidence freshness state that caps the effective parity band.
    pub freshness_class: String,
    /// Evidence source family cited by the row.
    pub evidence_source_class: String,
    /// Opaque reference to the evidence source.
    pub evidence_source_ref: String,
    /// Supported deployment profile rows reused across consumer surfaces.
    pub supported_deployment_profile_refs: Vec<String>,
    /// Supported runtime profile rows reused across consumer surfaces.
    pub supported_runtime_profile_refs: Vec<String>,
    /// Reference workspace identifiers backing the row.
    pub reference_workspace_ids: Vec<String>,
    /// Reference-workspace lineage or certification refs backing the row.
    pub reference_workspace_lineage_refs: Vec<String>,
    /// Certification state for the linked reference workspace.
    pub reference_workspace_certification_state_class: String,
    /// Known-gap state for the row.
    pub known_gap_state_class: String,
    /// Known-gap refs linked from the row.
    pub known_gap_refs: Vec<String>,
    /// Closed downgrade rules that may narrow the row.
    pub downgrade_rule_classes: Vec<String>,
    /// Effective downgrade state derived from the row inputs.
    pub downgrade_state_class: String,
    /// Specific reasons that narrowed the row below its claimed parity band.
    pub downgrade_reasons: Vec<String>,
    /// Replacement guidance for a narrowed or unsupported claim, when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_guidance_ref: Option<String>,
    /// Migration guidance for a switching or import claim, when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_guidance_ref: Option<String>,
    /// Workflow bundles that inherit this row.
    pub linked_bundle_refs: Vec<String>,
    /// Imported-user handoff bundles that inherit this row.
    pub linked_handoff_bundle_refs: Vec<String>,
    /// Reviewable summary safe for product and export surfaces.
    pub summary_label: String,
}

impl EcosystemCompatibilityRow {
    fn from_input(input: &EcosystemCompatibilityRowInput) -> Self {
        let derived = derive_effective_row_state(input);
        Self {
            record_kind: ROW_RECORD_KIND.to_string(),
            schema_version: ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            row_id: input.row_id.clone(),
            claimed_surface_class: input.claimed_surface_class.clone(),
            subject_class: input.subject_class.clone(),
            subject_ref: input.subject_ref.clone(),
            subject_label: input.subject_label.clone(),
            claimed_parity_band_class: input.claimed_parity_band_class.clone(),
            effective_parity_band_class: derived.effective_parity_band_class,
            bridge_parity_class: input.bridge_parity_class.clone(),
            freshness_class: input.freshness_class.clone(),
            evidence_source_class: input.evidence_source_class.clone(),
            evidence_source_ref: input.evidence_source_ref.clone(),
            supported_deployment_profile_refs: input.supported_deployment_profile_refs.clone(),
            supported_runtime_profile_refs: input.supported_runtime_profile_refs.clone(),
            reference_workspace_ids: input.reference_workspace_ids.clone(),
            reference_workspace_lineage_refs: input.reference_workspace_lineage_refs.clone(),
            reference_workspace_certification_state_class: input
                .reference_workspace_certification_state_class
                .clone(),
            known_gap_state_class: input.known_gap_state_class.clone(),
            known_gap_refs: input.known_gap_refs.clone(),
            downgrade_rule_classes: input.downgrade_rule_classes.clone(),
            downgrade_state_class: derived.downgrade_state_class,
            downgrade_reasons: derived.downgrade_reasons,
            replacement_guidance_ref: input.replacement_guidance_ref.clone(),
            migration_guidance_ref: input.migration_guidance_ref.clone(),
            linked_bundle_refs: input.linked_bundle_refs.clone(),
            linked_handoff_bundle_refs: input.linked_handoff_bundle_refs.clone(),
            summary_label: input.summary_label.clone(),
        }
    }

    /// Returns true when the row still backs a stable-facing claim.
    pub fn backs_stable_claim(&self) -> bool {
        self.effective_parity_band_class == "stable"
    }

    /// Validates the row against the canonical ecosystem scorecard contract.
    ///
    /// # Errors
    ///
    /// Returns [`EcosystemCompatibilityError`] when a row violates the shared
    /// compatibility contract.
    pub fn validate(&self) -> Result<(), EcosystemCompatibilityError> {
        ensure_eq(
            self.record_kind.as_str(),
            ROW_RECORD_KIND,
            "row.record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            "row.schema_version",
        )?;
        ensure_nonempty(&self.row_id, "row.row_id")?;
        ensure_token(
            CLAIMED_SURFACE_CLASSES,
            &self.claimed_surface_class,
            "row.claimed_surface_class",
        )?;
        ensure_token(SUBJECT_CLASSES, &self.subject_class, "row.subject_class")?;
        ensure_nonempty(&self.subject_ref, "row.subject_ref")?;
        ensure_nonempty(&self.subject_label, "row.subject_label")?;
        ensure_token(
            PARITY_BAND_CLASSES,
            &self.claimed_parity_band_class,
            "row.claimed_parity_band_class",
        )?;
        ensure_token(
            PARITY_BAND_CLASSES,
            &self.effective_parity_band_class,
            "row.effective_parity_band_class",
        )?;
        ensure_token(
            BRIDGE_PARITY_CLASSES,
            &self.bridge_parity_class,
            "row.bridge_parity_class",
        )?;
        ensure_token(
            FRESHNESS_CLASSES,
            &self.freshness_class,
            "row.freshness_class",
        )?;
        ensure_token(
            EVIDENCE_SOURCE_CLASSES,
            &self.evidence_source_class,
            "row.evidence_source_class",
        )?;
        ensure_nonempty(&self.evidence_source_ref, "row.evidence_source_ref")?;
        ensure_token(
            REFERENCE_WORKSPACE_CERTIFICATION_STATE_CLASSES,
            &self.reference_workspace_certification_state_class,
            "row.reference_workspace_certification_state_class",
        )?;
        ensure_token(
            KNOWN_GAP_STATE_CLASSES,
            &self.known_gap_state_class,
            "row.known_gap_state_class",
        )?;
        ensure_token(
            DOWNGRADE_STATE_CLASSES,
            &self.downgrade_state_class,
            "row.downgrade_state_class",
        )?;
        ensure_nonempty(&self.summary_label, "row.summary_label")?;
        ensure_no_blank_entries(
            &self.supported_deployment_profile_refs,
            "row.supported_deployment_profile_refs",
        )?;
        ensure_no_blank_entries(
            &self.supported_runtime_profile_refs,
            "row.supported_runtime_profile_refs",
        )?;
        ensure_no_blank_entries(&self.reference_workspace_ids, "row.reference_workspace_ids")?;
        ensure_no_blank_entries(
            &self.reference_workspace_lineage_refs,
            "row.reference_workspace_lineage_refs",
        )?;
        ensure_no_blank_entries(&self.known_gap_refs, "row.known_gap_refs")?;
        ensure_no_blank_entries(&self.linked_bundle_refs, "row.linked_bundle_refs")?;
        ensure_no_blank_entries(
            &self.linked_handoff_bundle_refs,
            "row.linked_handoff_bundle_refs",
        )?;
        for rule in &self.downgrade_rule_classes {
            ensure_token(DOWNGRADE_RULE_CLASSES, rule, "row.downgrade_rule_classes")?;
        }
        for reason in &self.downgrade_reasons {
            ensure_token(DOWNGRADE_RULE_CLASSES, reason, "row.downgrade_reasons")?;
        }
        if self.claimed_parity_band_class == "stable" && self.reference_workspace_ids.is_empty() {
            return Err(err(
                "stable claimed rows must cite at least one reference workspace id",
            ));
        }
        if !self.reference_workspace_ids.is_empty()
            && self.reference_workspace_lineage_refs.is_empty()
        {
            return Err(err(
                "reference workspace ids must carry lineage or certification refs",
            ));
        }
        if self.subject_class == "workflow_bundle" && self.linked_bundle_refs.is_empty() {
            return Err(err(
                "workflow bundle rows must cite the bundle refs that inherit them",
            ));
        }
        if self.claimed_surface_class == "imported_extension_claim"
            && self.migration_guidance_ref.is_none()
        {
            return Err(err(
                "imported extension claims must cite migration guidance",
            ));
        }
        let derived = derive_effective_row_state(&EcosystemCompatibilityRowInput {
            row_id: self.row_id.clone(),
            claimed_surface_class: self.claimed_surface_class.clone(),
            subject_class: self.subject_class.clone(),
            subject_ref: self.subject_ref.clone(),
            subject_label: self.subject_label.clone(),
            claimed_parity_band_class: self.claimed_parity_band_class.clone(),
            bridge_parity_class: self.bridge_parity_class.clone(),
            freshness_class: self.freshness_class.clone(),
            evidence_source_class: self.evidence_source_class.clone(),
            evidence_source_ref: self.evidence_source_ref.clone(),
            supported_deployment_profile_refs: self.supported_deployment_profile_refs.clone(),
            supported_runtime_profile_refs: self.supported_runtime_profile_refs.clone(),
            reference_workspace_ids: self.reference_workspace_ids.clone(),
            reference_workspace_lineage_refs: self.reference_workspace_lineage_refs.clone(),
            reference_workspace_certification_state_class: self
                .reference_workspace_certification_state_class
                .clone(),
            known_gap_state_class: self.known_gap_state_class.clone(),
            known_gap_refs: self.known_gap_refs.clone(),
            downgrade_rule_classes: self.downgrade_rule_classes.clone(),
            replacement_guidance_ref: self.replacement_guidance_ref.clone(),
            migration_guidance_ref: self.migration_guidance_ref.clone(),
            linked_bundle_refs: self.linked_bundle_refs.clone(),
            linked_handoff_bundle_refs: self.linked_handoff_bundle_refs.clone(),
            summary_label: self.summary_label.clone(),
        });
        if derived.effective_parity_band_class != self.effective_parity_band_class {
            return Err(err(
                "stored effective parity band does not match the derived ecosystem scorecard state",
            ));
        }
        if derived.downgrade_state_class != self.downgrade_state_class {
            return Err(err(
                "stored downgrade state does not match the derived ecosystem scorecard state",
            ));
        }
        let stored: BTreeSet<&str> = self.downgrade_reasons.iter().map(String::as_str).collect();
        let expected: BTreeSet<&str> = derived
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the derived ecosystem scorecard state",
            ));
        }
        Ok(())
    }
}

/// Consumer projection derived from a canonical ecosystem scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemCompatibilityClaimProjection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection identifier.
    pub claim_id: String,
    /// Consumer surface that renders this projection.
    pub consumer_surface_class: String,
    /// Canonical row the consumer resolved through.
    pub row_ref: String,
    /// Subject family the projection speaks for.
    pub subject_class: String,
    /// Stable subject reference safe to log and export.
    pub subject_ref: String,
    /// Effective parity band the consumer must render.
    pub effective_parity_band_class: String,
    /// Evidence freshness state the consumer must render.
    pub freshness_class: String,
    /// Evidence source family the consumer must cite.
    pub evidence_source_class: String,
    /// Supported deployment profile rows preserved across surfaces.
    pub supported_deployment_profile_refs: Vec<String>,
    /// Supported runtime profile rows preserved across surfaces.
    pub supported_runtime_profile_refs: Vec<String>,
    /// Reference workspace identifiers preserved across surfaces.
    pub reference_workspace_ids: Vec<String>,
    /// Reference-workspace lineage or certification refs preserved across surfaces.
    pub reference_workspace_lineage_refs: Vec<String>,
    /// Effective downgrade state the consumer must render.
    pub downgrade_state_class: String,
    /// Known-gap state the consumer must render.
    pub known_gap_state_class: String,
    /// Replacement guidance for a narrowed or unsupported claim, when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_guidance_ref: Option<String>,
    /// Migration guidance for a switching or import claim, when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_guidance_ref: Option<String>,
    /// Workflow bundles that inherit this row.
    pub linked_bundle_refs: Vec<String>,
    /// Imported-user handoff bundles that inherit this row.
    pub linked_handoff_bundle_refs: Vec<String>,
    /// Reviewable summary safe for UI, docs, and support surfaces.
    pub summary_label: String,
}

impl EcosystemCompatibilityClaimProjection {
    fn from_input(
        input: &EcosystemConsumerClaimInput,
        row: &EcosystemCompatibilityRow,
    ) -> Result<Self, EcosystemCompatibilityError> {
        ensure_token(
            CONSUMER_SURFACE_CLASSES,
            &input.consumer_surface_class,
            "claim.consumer_surface_class",
        )?;
        ensure_nonempty(&input.claim_id, "claim.claim_id")?;
        ensure_nonempty(&input.row_ref, "claim.row_ref")?;
        ensure_nonempty(&input.summary_label, "claim.summary_label")?;
        Ok(Self {
            record_kind: CLAIM_PROJECTION_RECORD_KIND.to_string(),
            schema_version: ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            claim_id: input.claim_id.clone(),
            consumer_surface_class: input.consumer_surface_class.clone(),
            row_ref: row.row_id.clone(),
            subject_class: row.subject_class.clone(),
            subject_ref: row.subject_ref.clone(),
            effective_parity_band_class: row.effective_parity_band_class.clone(),
            freshness_class: row.freshness_class.clone(),
            evidence_source_class: row.evidence_source_class.clone(),
            supported_deployment_profile_refs: row.supported_deployment_profile_refs.clone(),
            supported_runtime_profile_refs: row.supported_runtime_profile_refs.clone(),
            reference_workspace_ids: row.reference_workspace_ids.clone(),
            reference_workspace_lineage_refs: row.reference_workspace_lineage_refs.clone(),
            downgrade_state_class: row.downgrade_state_class.clone(),
            known_gap_state_class: row.known_gap_state_class.clone(),
            replacement_guidance_ref: row.replacement_guidance_ref.clone(),
            migration_guidance_ref: row.migration_guidance_ref.clone(),
            linked_bundle_refs: row.linked_bundle_refs.clone(),
            linked_handoff_bundle_refs: row.linked_handoff_bundle_refs.clone(),
            summary_label: input.summary_label.clone(),
        })
    }

    /// Validates the projection against its structural invariants.
    ///
    /// # Errors
    ///
    /// Returns [`EcosystemCompatibilityError`] when a projection violates the
    /// cross-surface contract.
    pub fn validate(&self) -> Result<(), EcosystemCompatibilityError> {
        ensure_eq(
            self.record_kind.as_str(),
            CLAIM_PROJECTION_RECORD_KIND,
            "claim.record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            "claim.schema_version",
        )?;
        ensure_nonempty(&self.claim_id, "claim.claim_id")?;
        ensure_token(
            CONSUMER_SURFACE_CLASSES,
            &self.consumer_surface_class,
            "claim.consumer_surface_class",
        )?;
        ensure_nonempty(&self.row_ref, "claim.row_ref")?;
        ensure_token(SUBJECT_CLASSES, &self.subject_class, "claim.subject_class")?;
        ensure_nonempty(&self.subject_ref, "claim.subject_ref")?;
        ensure_token(
            PARITY_BAND_CLASSES,
            &self.effective_parity_band_class,
            "claim.effective_parity_band_class",
        )?;
        ensure_token(
            FRESHNESS_CLASSES,
            &self.freshness_class,
            "claim.freshness_class",
        )?;
        ensure_token(
            EVIDENCE_SOURCE_CLASSES,
            &self.evidence_source_class,
            "claim.evidence_source_class",
        )?;
        ensure_token(
            DOWNGRADE_STATE_CLASSES,
            &self.downgrade_state_class,
            "claim.downgrade_state_class",
        )?;
        ensure_token(
            KNOWN_GAP_STATE_CLASSES,
            &self.known_gap_state_class,
            "claim.known_gap_state_class",
        )?;
        ensure_nonempty(&self.summary_label, "claim.summary_label")?;
        Ok(())
    }

    fn validate_against_row(
        &self,
        row: &EcosystemCompatibilityRow,
    ) -> Result<(), EcosystemCompatibilityError> {
        if self.row_ref != row.row_id
            || self.subject_class != row.subject_class
            || self.subject_ref != row.subject_ref
            || self.effective_parity_band_class != row.effective_parity_band_class
            || self.freshness_class != row.freshness_class
            || self.evidence_source_class != row.evidence_source_class
            || self.supported_deployment_profile_refs != row.supported_deployment_profile_refs
            || self.supported_runtime_profile_refs != row.supported_runtime_profile_refs
            || self.reference_workspace_ids != row.reference_workspace_ids
            || self.reference_workspace_lineage_refs != row.reference_workspace_lineage_refs
            || self.downgrade_state_class != row.downgrade_state_class
            || self.known_gap_state_class != row.known_gap_state_class
            || self.replacement_guidance_ref != row.replacement_guidance_ref
            || self.migration_guidance_ref != row.migration_guidance_ref
            || self.linked_bundle_refs != row.linked_bundle_refs
            || self.linked_handoff_bundle_refs != row.linked_handoff_bundle_refs
        {
            return Err(err(
                "consumer projection drifted from the canonical ecosystem scorecard row",
            ));
        }
        if self.consumer_surface_class == "bundle_detail_view" && self.linked_bundle_refs.is_empty()
        {
            return Err(err(
                "bundle detail projections must preserve bundle inheritance refs",
            ));
        }
        if self.consumer_surface_class == "migration_center_report"
            && self.migration_guidance_ref.is_none()
        {
            return Err(err(
                "migration center projections must preserve migration guidance refs",
            ));
        }
        Ok(())
    }
}

/// Compact inspection row for fixture and release validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemCompatibilityInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of canonical rows in the packet.
    pub row_count: usize,
    /// Number of consumer projections in the packet.
    pub consumer_claim_count: usize,
    /// Number of rows that still back stable claims.
    pub stable_row_count: usize,
    /// Number of rows narrowed below their claimed parity band.
    pub narrowed_row_count: usize,
    /// Number of rows carrying retest-pending parity.
    pub retest_pending_row_count: usize,
    /// True when every consumer claim resolves through a canonical row.
    pub all_consumer_claims_row_backed: bool,
    /// True when every stable row cites reference-workspace lineage.
    pub stable_rows_reference_workspace_backed: bool,
    /// True when every protected consumer surface is represented.
    pub protected_surface_coverage_complete: bool,
}

impl EcosystemCompatibilityInspection {
    fn from_rows(
        rows: &[EcosystemCompatibilityRow],
        consumer_claims: &[EcosystemCompatibilityClaimProjection],
    ) -> Self {
        let stable_row_count = rows.iter().filter(|row| row.backs_stable_claim()).count();
        let narrowed_row_count = rows
            .iter()
            .filter(|row| row.claimed_parity_band_class != row.effective_parity_band_class)
            .count();
        let retest_pending_row_count = rows
            .iter()
            .filter(|row| row.effective_parity_band_class == "retest_pending")
            .count();
        let all_consumer_claims_row_backed = consumer_claims.iter().all(|claim| {
            rows.iter().any(|row| {
                claim.row_ref == row.row_id
                    && claim.effective_parity_band_class == row.effective_parity_band_class
                    && claim.reference_workspace_ids == row.reference_workspace_ids
                    && claim.reference_workspace_lineage_refs
                        == row.reference_workspace_lineage_refs
            })
        });
        let stable_rows_reference_workspace_backed = rows.iter().all(|row| {
            row.effective_parity_band_class != "stable"
                || (!row.reference_workspace_ids.is_empty()
                    && !row.reference_workspace_lineage_refs.is_empty())
        });
        let protected_surface_coverage_complete = CONSUMER_SURFACE_CLASSES.iter().all(|surface| {
            consumer_claims
                .iter()
                .any(|claim| claim.consumer_surface_class == *surface)
        });

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_string(),
            schema_version: ECOSYSTEM_COMPATIBILITY_SCORECARD_SCHEMA_VERSION,
            row_count: rows.len(),
            consumer_claim_count: consumer_claims.len(),
            stable_row_count,
            narrowed_row_count,
            retest_pending_row_count,
            all_consumer_claims_row_backed,
            stable_rows_reference_workspace_backed,
            protected_surface_coverage_complete,
        }
    }
}

/// Validation error for ecosystem compatibility packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcosystemCompatibilityError {
    message: String,
}

impl fmt::Display for EcosystemCompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for EcosystemCompatibilityError {}

struct DerivedRowState {
    effective_parity_band_class: String,
    downgrade_state_class: String,
    downgrade_reasons: Vec<String>,
}

fn derive_effective_row_state(input: &EcosystemCompatibilityRowInput) -> DerivedRowState {
    let mut effective = input.claimed_parity_band_class.clone();
    let mut reasons = Vec::new();

    match input.freshness_class.as_str() {
        "stale" | "expired" => {
            effective = min_parity_band(effective, "retest_pending");
            reasons.push("evidence_freshness_expired".to_string());
        }
        "missing" => {
            effective = min_parity_band(effective, "preview");
            reasons.push("evidence_missing".to_string());
        }
        _ => {}
    }

    match input.bridge_parity_class.as_str() {
        "partial" | "approximate" => {
            effective = min_parity_band(effective, "limited");
            reasons.push("bridge_parity_narrowed".to_string());
        }
        "unsupported" => {
            effective = min_parity_band(effective, "unsupported");
            reasons.push("bridge_unsupported".to_string());
        }
        _ => {}
    }

    match input.reference_workspace_certification_state_class.as_str() {
        "expired" => {
            effective = min_parity_band(effective, "retest_pending");
            reasons.push("reference_workspace_certification_expired".to_string());
        }
        "missing" => {
            effective = min_parity_band(effective, "preview");
            reasons.push("reference_workspace_certification_missing".to_string());
        }
        _ => {}
    }

    match input.known_gap_state_class.as_str() {
        "disclosed" => {
            effective = min_parity_band(effective, "limited");
            reasons.push("known_gap_disclosed".to_string());
        }
        "unknown" => {
            effective = min_parity_band(effective, "preview");
            reasons.push("known_gap_unknown".to_string());
        }
        _ => {}
    }

    reasons.sort();
    reasons.dedup();

    let downgrade_state_class = if reasons.is_empty() {
        "none"
    } else if reasons.iter().any(|reason| reason == "bridge_unsupported") {
        "unsupported"
    } else if reasons
        .iter()
        .any(|reason| reason == "bridge_parity_narrowed")
    {
        "bridge_narrowed"
    } else if reasons.iter().any(|reason| {
        matches!(
            reason.as_str(),
            "reference_workspace_certification_expired"
                | "reference_workspace_certification_missing"
        )
    }) {
        "reference_workspace_narrowed"
    } else if reasons.iter().any(|reason| {
        matches!(
            reason.as_str(),
            "evidence_freshness_expired" | "evidence_missing"
        )
    }) {
        "freshness_expired"
    } else {
        "known_gap_narrowed"
    };

    DerivedRowState {
        effective_parity_band_class: effective,
        downgrade_state_class: downgrade_state_class.to_string(),
        downgrade_reasons: reasons,
    }
}

fn parity_rank(value: &str) -> u8 {
    match value {
        "stable" => 4,
        "limited" => 3,
        "preview" => 2,
        "retest_pending" => 1,
        "unsupported" => 0,
        _ => 0,
    }
}

fn min_parity_band(current: String, candidate: &str) -> String {
    if parity_rank(candidate) < parity_rank(&current) {
        candidate.to_string()
    } else {
        current
    }
}

fn validate_input(
    input: &EcosystemCompatibilityPacketInput,
) -> Result<(), EcosystemCompatibilityError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;
    if input.scorecard_revision == 0 {
        return Err(err("scorecard_revision must be at least 1"));
    }
    if input.rows.is_empty() {
        return Err(err("rows must not be empty"));
    }
    if input.consumer_claims.is_empty() {
        return Err(err("consumer_claims must not be empty"));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), EcosystemCompatibilityError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_eq(actual: &str, expected: &str, field: &str) -> Result<(), EcosystemCompatibilityError> {
    if actual != expected {
        return Err(err(format!("{field} must equal {expected}")));
    }
    Ok(())
}

fn ensure_eq_u32(
    actual: u32,
    expected: u32,
    field: &str,
) -> Result<(), EcosystemCompatibilityError> {
    if actual != expected {
        return Err(err(format!("{field} must equal {expected}")));
    }
    Ok(())
}

fn ensure_token(
    vocabulary: &[&str],
    value: &str,
    field: &str,
) -> Result<(), EcosystemCompatibilityError> {
    if vocabulary.contains(&value) {
        Ok(())
    } else {
        Err(err(format!(
            "{field} must be one of: {}",
            vocabulary.join(", ")
        )))
    }
}

fn ensure_no_blank_entries(
    values: &[String],
    field: &str,
) -> Result<(), EcosystemCompatibilityError> {
    if values.iter().any(|value| value.trim().is_empty()) {
        return Err(err(format!("{field} must not contain blank entries")));
    }
    Ok(())
}

fn err(message: impl Into<String>) -> EcosystemCompatibilityError {
    EcosystemCompatibilityError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct FixtureCase {
        case_name: String,
        packet_input: EcosystemCompatibilityPacketInput,
        expected: FixtureExpectation,
    }

    #[derive(Debug, Deserialize)]
    struct FixtureExpectation {
        stable_row_count: usize,
        narrowed_row_count: usize,
        retest_pending_row_count: usize,
        rows: Vec<ExpectedRow>,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedRow {
        row_id: String,
        effective_parity_band_class: String,
        downgrade_state_class: String,
        downgrade_reasons: Vec<String>,
    }

    fn fixtures() -> Vec<FixtureCase> {
        let raw_cases: &[&str] = &[
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ecosystem/m4/stabilize-ecosystem-compatibility-scorecards-reference-workspace/stable-current.json"
            )),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ecosystem/m4/stabilize-ecosystem-compatibility-scorecards-reference-workspace/stale-evidence.json"
            )),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ecosystem/m4/stabilize-ecosystem-compatibility-scorecards-reference-workspace/bridge-narrowed.json"
            )),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ecosystem/m4/stabilize-ecosystem-compatibility-scorecards-reference-workspace/certification-expired.json"
            )),
        ];
        raw_cases
            .iter()
            .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
            .collect()
    }

    #[test]
    fn fixtures_build_validate_and_preserve_consumer_truth() {
        let fixtures = fixtures();
        assert_eq!(
            fixtures.len(),
            4,
            "all canonical ecosystem fixtures must load"
        );

        for fixture in fixtures {
            let packet = EcosystemCompatibilityPacket::from_input(fixture.packet_input)
                .unwrap_or_else(|e| panic!("fixture {} must build: {e}", fixture.case_name));
            packet
                .validate()
                .unwrap_or_else(|e| panic!("fixture {} must validate: {e}", fixture.case_name));

            assert!(
                packet.inspection.all_consumer_claims_row_backed,
                "{}",
                fixture.case_name
            );
            assert!(
                packet.inspection.stable_rows_reference_workspace_backed,
                "{}",
                fixture.case_name
            );
            assert!(
                packet.inspection.protected_surface_coverage_complete,
                "{}",
                fixture.case_name
            );

            assert_eq!(
                packet.inspection.stable_row_count, fixture.expected.stable_row_count,
                "{}",
                fixture.case_name
            );
            assert_eq!(
                packet.inspection.narrowed_row_count, fixture.expected.narrowed_row_count,
                "{}",
                fixture.case_name
            );
            assert_eq!(
                packet.inspection.retest_pending_row_count,
                fixture.expected.retest_pending_row_count,
                "{}",
                fixture.case_name
            );

            for expected in fixture.expected.rows {
                let row = packet
                    .rows
                    .iter()
                    .find(|row| row.row_id == expected.row_id)
                    .unwrap_or_else(|| panic!("missing expected row {}", expected.row_id));
                assert_eq!(
                    row.effective_parity_band_class, expected.effective_parity_band_class,
                    "{}",
                    fixture.case_name
                );
                assert_eq!(
                    row.downgrade_state_class, expected.downgrade_state_class,
                    "{}",
                    fixture.case_name
                );
                let mut got = row.downgrade_reasons.clone();
                got.sort();
                let mut want = expected.downgrade_reasons.clone();
                want.sort();
                assert_eq!(got, want, "{}", fixture.case_name);

                for claim in packet
                    .consumer_claims
                    .iter()
                    .filter(|claim| claim.row_ref == row.row_id)
                {
                    assert_eq!(
                        claim.effective_parity_band_class, row.effective_parity_band_class,
                        "{}",
                        fixture.case_name
                    );
                    assert_eq!(
                        claim.reference_workspace_ids, row.reference_workspace_ids,
                        "{}",
                        fixture.case_name
                    );
                    assert_eq!(
                        claim.reference_workspace_lineage_refs,
                        row.reference_workspace_lineage_refs,
                        "{}",
                        fixture.case_name
                    );
                }
            }
        }
    }
}
