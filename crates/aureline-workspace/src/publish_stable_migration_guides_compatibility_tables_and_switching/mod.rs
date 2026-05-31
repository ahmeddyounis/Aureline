//! Publish stable migration guides, compatibility tables, and switching
//! known-limits for launch cohorts.
//!
//! The migration-wizard import-fidelity contract (see
//! [`crate::stabilize_migration_wizard_import_fidelity_for_editor_launch_paths`])
//! owns *per-item* import outcomes. This module owns the layer above it: the
//! **published, stable switching truth** a launch cohort reads before it adopts
//! Aureline — the migration guide, the machine-readable compatibility table, and
//! the explicit switching known-limits — and the **stability qualification** that
//! published truth is allowed to claim.
//!
//! The central rule mirrors the rest of the daily-driver line: a **stable**
//! switching claim may never be implied from prose alone. A guide that renders a
//! `Stable` switch-readiness badge must resolve to a *current* compatibility
//! table whose freshness still supports it, must carry no unsupported core
//! capability, no unresolved blocking known-limit, must be reversible, and must
//! be fully attributed about its provider/browser-handoff source. When any of
//! those fails, the visible tier is **automatically narrowed below Stable**
//! (to `beta` or `preview`) rather than left asserting switch-readiness the
//! evidence no longer backs. The checked-in packet is canonical: docs/help,
//! release packets, and the switch planner ingest it instead of cloning status
//! prose.
//!
//! The record family includes:
//!
//! - [`MigrationGuideIdentity`] — the guide identity shared across migration
//!   center, docs/help, release packets, and the switch planner: guide id,
//!   source tool, launch cohort, doc reference, evidence freshness, and the
//!   reversibility/previewability of the switch the guide describes.
//! - [`MigrationCompatibilityTable`] / [`MigrationCompatibilityTableRow`] — the
//!   machine-readable compatibility table linking each source capability area to
//!   an `exact` / `translated` / `partial` / `shimmed` / `unsupported` outcome
//!   generated from real imported artifacts, marking core capabilities.
//! - [`SwitchingKnownLimit`] — an explicitly disclosed switching limitation with
//!   a severity class and the workaround (native alternative, bridge, manual
//!   step, or none) so known gaps stay visible instead of collapsing to green.
//! - [`StableQualificationClaim`] — the claimed switch tier, the *effective*
//!   tier after the table and known-limits are applied, the support claim it may
//!   imply, and the reasons that narrowed it.
//! - [`ProviderHandoffDisclosure`] — explicit source, actor, freshness, target,
//!   and return path for any provider-linked or browser-handoff step so the
//!   stable line never hides hosted authority behind local chrome.
//! - [`MigrationSwitchingPublicationInspection`] — compact boolean projection for
//!   CLI/headless and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/publish-stable-migration-guides-compatibility-tables-and-switching.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/publish-stable-migration-guides-compatibility-tables-and-switching/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every migration-switching publication record.
pub const MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for [`MigrationSwitchingPublicationPacket`].
pub const MIGRATION_SWITCHING_PUBLICATION_PACKET_RECORD_KIND: &str =
    "migration_switching_publication_packet";

/// Record-kind tag for [`MigrationGuideIdentity`].
pub const MIGRATION_GUIDE_IDENTITY_RECORD_KIND: &str = "migration_guide_identity";

/// Record-kind tag for [`MigrationCompatibilityTable`].
pub const MIGRATION_COMPATIBILITY_TABLE_RECORD_KIND: &str = "migration_compatibility_table";

/// Record-kind tag for [`MigrationCompatibilityTableRow`].
pub const MIGRATION_COMPATIBILITY_TABLE_ROW_RECORD_KIND: &str = "migration_compatibility_table_row";

/// Record-kind tag for [`SwitchingKnownLimit`].
pub const SWITCHING_KNOWN_LIMIT_RECORD_KIND: &str = "switching_known_limit";

/// Record-kind tag for [`StableQualificationClaim`].
pub const STABLE_QUALIFICATION_CLAIM_RECORD_KIND: &str = "stable_qualification_claim";

/// Record-kind tag for [`ProviderHandoffDisclosure`].
pub const PROVIDER_HANDOFF_DISCLOSURE_RECORD_KIND: &str = "provider_handoff_disclosure";

/// Record-kind tag for [`MigrationSwitchingPublicationInspection`].
pub const MIGRATION_SWITCHING_PUBLICATION_INSPECTION_RECORD_KIND: &str =
    "migration_switching_publication_inspection";

/// Canonical schema path the packet cites.
pub const MIGRATION_SWITCHING_PUBLICATION_SCHEMA_REF: &str =
    "schemas/review/publish-stable-migration-guides-compatibility-tables-and-switching.schema.json";

/// Closed set of supported source migration tools/ecosystems.
pub const MIGRATION_SOURCE_TOOLS: &[&str] = &[
    "vs_code_code_oss",
    "jetbrains_family",
    "vim_neovim",
    "emacs",
];

/// Closed set of launch cohorts a guide is published for.
pub const LAUNCH_COHORTS: &[&str] = &[
    "solo_switcher",
    "team_pilot",
    "org_rollout",
    "design_partner",
    "imported_user",
];

/// Closed set of compatibility outcome labels generated from imported artifacts.
pub const COMPATIBILITY_OUTCOME_LABELS: &[&str] =
    &["exact", "translated", "partial", "shimmed", "unsupported"];

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* switch claim. These may only render when
/// backed by a current compatibility table with no stable-blocking gap.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed set of compatibility-table evidence freshness classes.
pub const GUIDE_EVIDENCE_FRESHNESS_CLASSES: &[&str] = &[
    "fresh_current",
    "aging_within_window",
    "stale_past_window",
    "evidence_unknown",
];

/// Closed set of claim-basis classes. `prose_only` may never back a stable tier.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "prose_only"];

/// Closed set of switching known-limit severity classes.
pub const KNOWN_LIMIT_SEVERITY_CLASSES: &[&str] = &["blocking", "major", "minor", "informational"];

/// Closed set of switching known-limit workaround classes.
pub const KNOWN_LIMIT_WORKAROUND_CLASSES: &[&str] = &[
    "native_alternative",
    "bridge_available",
    "manual_step",
    "no_workaround",
];

/// Closed set of provider/browser-handoff source classes.
pub const HANDOFF_SOURCE_CLASSES: &[&str] = &[
    "local_only",
    "hosted_provider",
    "browser_handoff",
    "mirror_offline",
];

/// Closed set of provider/browser-handoff actor classes.
pub const HANDOFF_ACTOR_CLASSES: &[&str] = &[
    "local_user",
    "aureline_runtime",
    "hosted_provider_service",
    "browser_session",
];

/// Closed set of provider/browser-handoff freshness classes.
pub const HANDOFF_FRESHNESS_CLASSES: &[&str] = &[
    "live_current",
    "cached_disclosed",
    "snapshot_dated",
    "freshness_unknown",
];

/// Closed set of support claim classes a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_switch_ready_claim",
    "beta_switch_partial_claim",
    "preview_switch_experimental_claim",
    "withdrawn_no_switch_claim",
];

/// Closed set of reasons that narrow a stable switch claim below Stable.
pub const SWITCH_DOWNGRADE_REASONS: &[&str] = &[
    "evidence_freshness_expired",
    "compatibility_table_unsupported_present",
    "known_limit_blocking_unresolved",
    "prose_only_claim",
    "missing_compatibility_table",
    "not_reversible",
    "attribution_incomplete",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const MIGRATION_SWITCHING_CONSUMER_SURFACES: &[&str] = &[
    "migration_center",
    "docs_help_surface",
    "release_packet",
    "switch_planner",
    "support_export",
    "about_surface",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a migration-switching publication packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSwitchingPublicationInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Guide identity input.
    pub identity: MigrationGuideIdentityInput,
    /// Compatibility table input.
    pub compatibility_table: MigrationCompatibilityTableInput,
    /// Switching known-limit inputs.
    #[serde(default)]
    pub known_limits: Vec<SwitchingKnownLimitInput>,
    /// Stability qualification claim input.
    pub claim: StableQualificationClaimInput,
    /// Provider/browser-handoff disclosure input.
    pub handoff_disclosure: ProviderHandoffDisclosureInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`MigrationGuideIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationGuideIdentityInput {
    /// Stable guide id (must match every surface).
    pub guide_id: String,
    /// Integer guide revision.
    pub guide_revision: u32,
    /// Source migration tool/ecosystem.
    pub source_tool: String,
    /// Launch cohort the guide is published for.
    pub launch_cohort: String,
    /// Canonical doc reference for the published guide.
    pub guide_ref: String,
    /// Evidence freshness class for the compatibility table backing the guide.
    pub evidence_freshness_class: String,
    /// Timestamp the evidence freshness expires.
    pub freshness_expires_at: String,
    /// True when the switch the guide describes is reversible to the prior tool.
    pub reversible: bool,
    /// True when the switch/import the guide describes is previewable first.
    pub previewable: bool,
}

/// Input for [`MigrationCompatibilityTable`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCompatibilityTableInput {
    /// Stable table id.
    pub table_id: String,
    /// Machine-readable table reference.
    pub table_ref: String,
    /// Timestamp the table was generated.
    pub generated_at: String,
    /// Table rows.
    pub rows: Vec<MigrationCompatibilityTableRowInput>,
}

/// Input for one [`MigrationCompatibilityTableRow`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCompatibilityTableRowInput {
    /// Stable row id.
    pub row_id: String,
    /// Capability area this row covers.
    pub capability_area: String,
    /// Source-feature reference covered by this row.
    pub source_feature_ref: String,
    /// Outcome label generated from imported artifacts.
    pub outcome_label: String,
    /// True when this capability is core to a stable switch claim.
    pub is_core_capability: bool,
    /// Reference to the imported artifact this outcome was derived from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from_artifact_ref: Option<String>,
    /// Reference to the guide section explaining this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guide_section_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for one [`SwitchingKnownLimit`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchingKnownLimitInput {
    /// Stable limit id.
    pub limit_id: String,
    /// Capability area the limit applies to.
    pub capability_area: String,
    /// Severity class of the limit.
    pub severity_class: String,
    /// Workaround class for the limit.
    pub workaround_class: String,
    /// Reference to a workaround, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workaround_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`StableQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableQualificationClaimInput {
    /// Switch tier claimed by the guide author.
    pub claimed_tier: String,
    /// Claim basis: evidence-backed vs prose only.
    pub claim_basis_class: String,
    /// Compatibility table this claim points to, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_table_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ProviderHandoffDisclosure`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHandoffDisclosureInput {
    /// Source class for any provider/browser-handoff step in the guide.
    pub source_class: String,
    /// Actor class performing the handoff.
    pub actor_class: String,
    /// Freshness class of provider/browser-handoff data.
    pub freshness_class: String,
    /// Public target label for the handoff destination.
    pub target_label: String,
    /// Public return-path label describing how the user comes back.
    pub return_path_label: String,
    /// True when ownership of the destination is disclosed.
    pub discloses_ownership: bool,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Guide identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationGuideIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable guide id.
    pub guide_id: String,
    /// Integer guide revision.
    pub guide_revision: u32,
    /// Source migration tool/ecosystem.
    pub source_tool: String,
    /// Launch cohort the guide is published for.
    pub launch_cohort: String,
    /// Canonical doc reference for the published guide.
    pub guide_ref: String,
    /// Evidence freshness class for the compatibility table backing the guide.
    pub evidence_freshness_class: String,
    /// Timestamp the evidence freshness expires.
    pub freshness_expires_at: String,
    /// True when the switch the guide describes is reversible to the prior tool.
    pub reversible: bool,
    /// True when the switch/import the guide describes is previewable first.
    pub previewable: bool,
}

impl MigrationGuideIdentity {
    /// Returns true when the backing evidence is current enough for a stable tier.
    pub fn evidence_is_current(&self) -> bool {
        matches!(
            self.evidence_freshness_class.as_str(),
            "fresh_current" | "aging_within_window"
        )
    }
}

/// Machine-readable compatibility table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCompatibilityTable {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable table id.
    pub table_id: String,
    /// Machine-readable table reference.
    pub table_ref: String,
    /// Timestamp the table was generated.
    pub generated_at: String,
    /// Table rows.
    pub rows: Vec<MigrationCompatibilityTableRow>,
}

impl MigrationCompatibilityTable {
    /// Returns true when a core capability row carries an `unsupported` outcome.
    pub fn has_unsupported_core_capability(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.is_core_capability && row.outcome_label == "unsupported")
    }
}

/// One row of a compatibility table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCompatibilityTableRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Capability area this row covers.
    pub capability_area: String,
    /// Source-feature reference covered by this row.
    pub source_feature_ref: String,
    /// Outcome label generated from imported artifacts.
    pub outcome_label: String,
    /// True when this capability is core to a stable switch claim.
    pub is_core_capability: bool,
    /// Reference to the imported artifact this outcome was derived from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from_artifact_ref: Option<String>,
    /// Reference to the guide section explaining this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guide_section_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// An explicitly disclosed switching known-limit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchingKnownLimit {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable limit id.
    pub limit_id: String,
    /// Capability area the limit applies to.
    pub capability_area: String,
    /// Severity class of the limit.
    pub severity_class: String,
    /// Workaround class for the limit.
    pub workaround_class: String,
    /// Reference to a workaround, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workaround_ref: Option<String>,
    /// True when the limit is blocking and has no workaround.
    pub blocks_stable_switch: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Stability qualification claim after the table and known-limits are applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Switch tier claimed by the guide author.
    pub claimed_tier: String,
    /// Effective switch tier after the table and known-limits are applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: evidence-backed vs prose only.
    pub claim_basis_class: String,
    /// Compatibility table this claim points to, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_table_ref: Option<String>,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Explicit provider/browser-handoff disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHandoffDisclosure {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Source class for any provider/browser-handoff step in the guide.
    pub source_class: String,
    /// Actor class performing the handoff.
    pub actor_class: String,
    /// Freshness class of provider/browser-handoff data.
    pub freshness_class: String,
    /// Public target label for the handoff destination.
    pub target_label: String,
    /// Public return-path label describing how the user comes back.
    pub return_path_label: String,
    /// True when ownership of the destination is disclosed.
    pub discloses_ownership: bool,
}

impl ProviderHandoffDisclosure {
    /// Returns true when source, actor, freshness, target, and return path are
    /// all present and ownership is disclosed — the attribution a stable claim
    /// depends on.
    pub fn is_complete(&self) -> bool {
        self.discloses_ownership
            && !self.source_class.trim().is_empty()
            && !self.actor_class.trim().is_empty()
            && !self.freshness_class.trim().is_empty()
            && !self.target_label.trim().is_empty()
            && !self.return_path_label.trim().is_empty()
    }
}

/// Compact inspection row for CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSwitchingPublicationInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Guide inspected by this row.
    pub guide_id_ref: String,
    /// Effective switch tier.
    pub effective_tier: String,
    /// True when the claim is a stable switch claim.
    pub stable_claim: bool,
    /// True when the effective stable claim resolves to a current table.
    pub resolves_to_current_compatibility_table: bool,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// True when no stable claim is backed by prose alone.
    pub no_prose_only_stable_claim: bool,
    /// True when the guide identity is internally consistent.
    pub identity_consistent: bool,
    /// True when the switch is reversible.
    pub reversible: bool,
    /// True when the switch is previewable.
    pub previewable: bool,
    /// True when provider/browser-handoff attribution is complete.
    pub attribution_complete: bool,
    /// Number of compatibility-table rows.
    pub compatibility_row_count: usize,
    /// Number of disclosed known-limits.
    pub known_limit_count: usize,
    /// Number of unresolved blocking known-limits.
    pub blocking_known_limit_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Migration-switching publication packet consumed by migration center,
/// docs/help, release packets, the switch planner, support export, and About.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSwitchingPublicationPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Guide identity.
    pub identity: MigrationGuideIdentity,
    /// Compatibility table.
    pub compatibility_table: MigrationCompatibilityTable,
    /// Disclosed switching known-limits.
    pub known_limits: Vec<SwitchingKnownLimit>,
    /// Stability qualification claim after the table is applied.
    pub claim: StableQualificationClaim,
    /// Provider/browser-handoff disclosure.
    pub handoff_disclosure: ProviderHandoffDisclosure,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a published guide cannot hide a provider mutation behind chrome.
    pub allows_hidden_provider_mutation: bool,
    /// False so a published guide cannot describe an unattributed handoff.
    pub allows_unattributed_handoff: bool,
    /// False so a stable guide cannot describe an irreversible switch silently.
    pub allows_irreversible_switch_without_disclosure: bool,
    /// Inspection row.
    pub inspection: MigrationSwitchingPublicationInspection,
}

impl MigrationSwitchingPublicationPacket {
    /// Builds a publication packet from input, applying the compatibility table
    /// and known-limits to the claimed tier so any required downgrade is
    /// automatic.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationSwitchingValidationError`] when the input violates an
    /// identity, table, claim, or disclosure invariant.
    pub fn from_input(
        input: MigrationSwitchingPublicationInput,
    ) -> Result<Self, MigrationSwitchingValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let compatibility_table = table_record(&input.compatibility_table);
        let known_limits: Vec<SwitchingKnownLimit> =
            input.known_limits.iter().map(known_limit_record).collect();
        let handoff_disclosure = handoff_record(&input.handoff_disclosure);
        let claim = claim_record(
            &input.claim,
            &identity,
            &compatibility_table,
            &known_limits,
            &handoff_disclosure,
        );
        let inspection = inspection_record(
            &identity,
            &compatibility_table,
            &known_limits,
            &claim,
            &handoff_disclosure,
        );

        let packet = Self {
            record_kind: MIGRATION_SWITCHING_PUBLICATION_PACKET_RECORD_KIND.to_string(),
            schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            compatibility_table,
            known_limits,
            claim,
            handoff_disclosure,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![MIGRATION_SWITCHING_PUBLICATION_SCHEMA_REF.to_string()],
            allows_hidden_provider_mutation: false,
            allows_unattributed_handoff: false,
            allows_irreversible_switch_without_disclosure: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the publication invariants.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationSwitchingValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), MigrationSwitchingValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            MIGRATION_SWITCHING_PUBLICATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_table(&self.compatibility_table)?;
        for limit in &self.known_limits {
            validate_known_limit(limit)?;
        }
        validate_claim(&self.claim)?;
        validate_handoff(&self.handoff_disclosure)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                MIGRATION_SWITCHING_CONSUMER_SURFACES,
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
            .any(|r| r == MIGRATION_SWITCHING_PUBLICATION_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No hidden authority or silent irreversibility may ride a published guide.
        if self.allows_hidden_provider_mutation
            || self.allows_unattributed_handoff
            || self.allows_irreversible_switch_without_disclosure
        {
            return Err(err(
                "a published guide must not allow hidden provider mutation, unattributed handoff, or silent irreversible switching",
            ));
        }

        // A claim pointing at a table ref must resolve to the published table.
        if let Some(table_ref) = &self.claim.compatibility_table_ref {
            if table_ref != &self.compatibility_table.table_ref {
                return Err(err(
                    "claim compatibility_table_ref must resolve to the published table",
                ));
            }
        }

        // Stable-claim binding: a stable effective tier must be evidence-backed,
        // resolve to a current table, carry no unsupported core capability or
        // unresolved blocking limit, be reversible, and be fully attributed.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if self.claim.claim_basis_class != "evidence_backed" {
                return Err(err(
                    "stable effective tier must be evidence-backed, not prose-only",
                ));
            }
            if self.claim.compatibility_table_ref.as_deref()
                != Some(self.compatibility_table.table_ref.as_str())
            {
                return Err(err(
                    "stable effective tier must point at the published compatibility table",
                ));
            }
            if !self.identity.evidence_is_current() {
                return Err(err(
                    "stable effective tier must resolve to a current compatibility table",
                ));
            }
            if self.compatibility_table.has_unsupported_core_capability() {
                return Err(err(
                    "stable effective tier must not carry an unsupported core capability",
                ));
            }
            if self.known_limits.iter().any(|l| l.blocks_stable_switch) {
                return Err(err(
                    "stable effective tier must not carry an unresolved blocking known-limit",
                ));
            }
            if !self.identity.reversible {
                return Err(err(
                    "stable effective tier must describe a reversible switch",
                ));
            }
            if !self.handoff_disclosure.is_complete() {
                return Err(err(
                    "stable effective tier must carry complete provider/handoff attribution",
                ));
            }
            if self.claim.downgraded {
                return Err(err(
                    "a stable effective tier must not also be marked downgraded",
                ));
            }
        }

        // Downgrade truth: a downgraded claim must carry at least one reason and
        // must never keep a stable effective tier.
        if self.claim.downgraded {
            if self.claim.downgrade_reasons.is_empty() {
                return Err(err("a downgraded claim must carry at least one reason"));
            }
            if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
                return Err(err("a downgraded claim must not keep a stable tier"));
            }
        }

        // Recompute the effective tier and re-derive the downgrade verdict so the
        // stored claim cannot drift from the table/known-limit truth.
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &self.identity,
            &self.compatibility_table,
            &self.known_limits,
            &self.handoff_disclosure,
        );
        if derived.effective_tier != self.claim.effective_tier {
            return Err(err(
                "stored effective tier does not match the table-derived tier",
            ));
        }
        if derived.downgraded != self.claim.downgraded {
            return Err(err(
                "stored downgrade flag does not match the table-derived verdict",
            ));
        }
        let stored: BTreeSet<&str> = self
            .claim
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<&str> = derived
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the table-derived reasons",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from prose alone.
    pub fn no_prose_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "evidence_backed";
        }
        true
    }

    /// Returns true when the effective stable claim resolves to a current table.
    pub fn resolves_to_current_compatibility_table(&self) -> bool {
        self.claim.compatibility_table_ref.as_deref()
            == Some(self.compatibility_table.table_ref.as_str())
            && self.identity.evidence_is_current()
            && !self.compatibility_table.has_unsupported_core_capability()
            && !self.known_limits.iter().any(|l| l.blocks_stable_switch)
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationSwitchingPublicationProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Guide id.
    pub guide_id: String,
    /// Source migration tool.
    pub source_tool: String,
    /// Launch cohort.
    pub launch_cohort: String,
    /// Claimed switch tier.
    pub claimed_tier: String,
    /// Effective switch tier after the table is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable switch claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when the effective stable claim resolves to a current table.
    pub resolves_to_current_compatibility_table: bool,
    /// Number of compatibility-table rows.
    pub compatibility_row_count: usize,
    /// Number of disclosed known-limits.
    pub known_limit_count: usize,
}

impl From<MigrationSwitchingPublicationPacket> for MigrationSwitchingPublicationProjection {
    fn from(packet: MigrationSwitchingPublicationPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            guide_id: packet.identity.guide_id,
            source_tool: packet.identity.source_tool,
            launch_cohort: packet.identity.launch_cohort,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            resolves_to_current_compatibility_table: packet
                .inspection
                .resolves_to_current_compatibility_table,
            compatibility_row_count: packet.compatibility_table.rows.len(),
            known_limit_count: packet.known_limits.len(),
        }
    }
}

/// Parses and validates a materialized publication packet.
///
/// # Errors
///
/// Returns [`MigrationSwitchingError`] when the payload fails to parse or
/// violates the publication invariants.
pub fn project_migration_switching_publication(
    payload: &str,
) -> Result<MigrationSwitchingPublicationProjection, MigrationSwitchingError> {
    let packet: MigrationSwitchingPublicationPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(MigrationSwitchingPublicationProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for publication operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationSwitchingError {
    /// Validation failed.
    Validation(MigrationSwitchingValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for MigrationSwitchingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for MigrationSwitchingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationSwitchingValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for MigrationSwitchingValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MigrationSwitchingValidationError {}

impl MigrationSwitchingValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for MigrationSwitchingError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(MigrationSwitchingValidationError {
            message: err.to_string(),
        })
    }
}

impl From<MigrationSwitchingValidationError> for MigrationSwitchingError {
    fn from(err: MigrationSwitchingValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Outcome of applying a compatibility table and known-limits to a claimed tier.
struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Applies the compatibility table and known-limits to a claimed tier,
/// narrowing automatically below Stable when the evidence no longer supports it.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    identity: &MigrationGuideIdentity,
    table: &MigrationCompatibilityTable,
    known_limits: &[SwitchingKnownLimit],
    handoff: &ProviderHandoffDisclosure,
) -> DerivedTier {
    // Non-stable claims are already honest; they pass through unchanged.
    if !STABLE_TIERS.contains(&claimed_tier) {
        return DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        };
    }

    let mut reasons: Vec<String> = Vec::new();

    if claim_basis != "evidence_backed" {
        reasons.push("prose_only_claim".to_string());
    }
    // A stable claim with no resolvable table cannot stand.
    if table.rows.is_empty() {
        reasons.push("missing_compatibility_table".to_string());
    }
    if !identity.evidence_is_current() {
        reasons.push("evidence_freshness_expired".to_string());
    }
    if table.has_unsupported_core_capability() {
        reasons.push("compatibility_table_unsupported_present".to_string());
    }
    if known_limits.iter().any(|l| l.blocks_stable_switch) {
        reasons.push("known_limit_blocking_unresolved".to_string());
    }
    if !identity.reversible {
        reasons.push("not_reversible".to_string());
    }
    if !handoff.is_complete() {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();

    if reasons.is_empty() {
        DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        let effective = narrow_tier_for(&reasons);
        DerivedTier {
            effective_tier: effective.to_string(),
            support_claim: support_claim_for(effective),
            downgraded: true,
            downgrade_reasons: reasons,
        }
    }
}

/// Picks the effective tier to render given the active narrowing reasons.
///
/// A guide whose only shortfall is aging evidence narrows to `beta`; any
/// structural shortfall (prose-only, missing table, unsupported core, unresolved
/// blocking limit, irreversibility, or incomplete attribution) narrows to the
/// more conservative `preview`.
fn narrow_tier_for(reasons: &[String]) -> &'static str {
    let only_freshness = reasons.iter().all(|r| r == "evidence_freshness_expired");
    if only_freshness {
        "beta"
    } else {
        "preview"
    }
}

/// Maps an effective tier to the support claim it may imply.
fn support_claim_for(tier: &str) -> String {
    match tier {
        "stable" => "stable_switch_ready_claim",
        "beta" => "beta_switch_partial_claim",
        "preview" => "preview_switch_experimental_claim",
        "withdrawn" => "withdrawn_no_switch_claim",
        _ => "preview_switch_experimental_claim",
    }
    .to_string()
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &MigrationGuideIdentityInput) -> MigrationGuideIdentity {
    MigrationGuideIdentity {
        record_kind: MIGRATION_GUIDE_IDENTITY_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        guide_id: input.guide_id.clone(),
        guide_revision: input.guide_revision,
        source_tool: input.source_tool.clone(),
        launch_cohort: input.launch_cohort.clone(),
        guide_ref: input.guide_ref.clone(),
        evidence_freshness_class: input.evidence_freshness_class.clone(),
        freshness_expires_at: input.freshness_expires_at.clone(),
        reversible: input.reversible,
        previewable: input.previewable,
    }
}

fn table_record(input: &MigrationCompatibilityTableInput) -> MigrationCompatibilityTable {
    MigrationCompatibilityTable {
        record_kind: MIGRATION_COMPATIBILITY_TABLE_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        table_id: input.table_id.clone(),
        table_ref: input.table_ref.clone(),
        generated_at: input.generated_at.clone(),
        rows: input.rows.iter().map(table_row_record).collect(),
    }
}

fn table_row_record(input: &MigrationCompatibilityTableRowInput) -> MigrationCompatibilityTableRow {
    MigrationCompatibilityTableRow {
        record_kind: MIGRATION_COMPATIBILITY_TABLE_ROW_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        row_id: input.row_id.clone(),
        capability_area: input.capability_area.clone(),
        source_feature_ref: input.source_feature_ref.clone(),
        outcome_label: input.outcome_label.clone(),
        is_core_capability: input.is_core_capability,
        derived_from_artifact_ref: input.derived_from_artifact_ref.clone(),
        guide_section_ref: input.guide_section_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn known_limit_record(input: &SwitchingKnownLimitInput) -> SwitchingKnownLimit {
    let blocks_stable_switch =
        input.severity_class == "blocking" && input.workaround_class == "no_workaround";
    SwitchingKnownLimit {
        record_kind: SWITCHING_KNOWN_LIMIT_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        limit_id: input.limit_id.clone(),
        capability_area: input.capability_area.clone(),
        severity_class: input.severity_class.clone(),
        workaround_class: input.workaround_class.clone(),
        workaround_ref: input.workaround_ref.clone(),
        blocks_stable_switch,
        summary_label: input.summary_label.clone(),
    }
}

fn handoff_record(input: &ProviderHandoffDisclosureInput) -> ProviderHandoffDisclosure {
    ProviderHandoffDisclosure {
        record_kind: PROVIDER_HANDOFF_DISCLOSURE_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        source_class: input.source_class.clone(),
        actor_class: input.actor_class.clone(),
        freshness_class: input.freshness_class.clone(),
        target_label: input.target_label.clone(),
        return_path_label: input.return_path_label.clone(),
        discloses_ownership: input.discloses_ownership,
    }
}

fn claim_record(
    input: &StableQualificationClaimInput,
    identity: &MigrationGuideIdentity,
    table: &MigrationCompatibilityTable,
    known_limits: &[SwitchingKnownLimit],
    handoff: &ProviderHandoffDisclosure,
) -> StableQualificationClaim {
    let derived = derive_effective_tier(
        &input.claimed_tier,
        &input.claim_basis_class,
        identity,
        table,
        known_limits,
        handoff,
    );
    StableQualificationClaim {
        record_kind: STABLE_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        compatibility_table_ref: input.compatibility_table_ref.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    identity: &MigrationGuideIdentity,
    table: &MigrationCompatibilityTable,
    known_limits: &[SwitchingKnownLimit],
    claim: &StableQualificationClaim,
    handoff: &ProviderHandoffDisclosure,
) -> MigrationSwitchingPublicationInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());
    let resolves_to_current_compatibility_table = claim.compatibility_table_ref.as_deref()
        == Some(table.table_ref.as_str())
        && identity.evidence_is_current()
        && !table.has_unsupported_core_capability()
        && !known_limits.iter().any(|l| l.blocks_stable_switch);
    let no_prose_only_stable_claim = !stable_claim || claim.claim_basis_class == "evidence_backed";
    let blocking_known_limit_count = known_limits
        .iter()
        .filter(|l| l.blocks_stable_switch)
        .count();

    MigrationSwitchingPublicationInspection {
        record_kind: MIGRATION_SWITCHING_PUBLICATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        guide_id_ref: identity.guide_id.clone(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        resolves_to_current_compatibility_table,
        downgraded: claim.downgraded,
        no_prose_only_stable_claim,
        identity_consistent: identity_is_consistent(identity),
        reversible: identity.reversible,
        previewable: identity.previewable,
        attribution_complete: handoff.is_complete(),
        compatibility_row_count: table.rows.len(),
        known_limit_count: known_limits.len(),
        blocking_known_limit_count,
        summary_label: claim.summary_label.clone(),
    }
}

fn identity_is_consistent(identity: &MigrationGuideIdentity) -> bool {
    !identity.guide_id.trim().is_empty()
        && contains(MIGRATION_SOURCE_TOOLS, &identity.source_tool)
        && contains(LAUNCH_COHORTS, &identity.launch_cohort)
        && contains(
            GUIDE_EVIDENCE_FRESHNESS_CLASSES,
            &identity.evidence_freshness_class,
        )
        && !identity.guide_ref.trim().is_empty()
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &MigrationSwitchingPublicationInput,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.guide_id, "identity.guide_id")?;
    ensure_token(
        MIGRATION_SOURCE_TOOLS,
        &id.source_tool,
        "identity.source_tool",
    )?;
    ensure_token(LAUNCH_COHORTS, &id.launch_cohort, "identity.launch_cohort")?;
    ensure_token(
        GUIDE_EVIDENCE_FRESHNESS_CLASSES,
        &id.evidence_freshness_class,
        "identity.evidence_freshness_class",
    )?;
    ensure_nonempty(&id.guide_ref, "identity.guide_ref")?;
    ensure_nonempty(&id.freshness_expires_at, "identity.freshness_expires_at")?;

    let table = &input.compatibility_table;
    ensure_nonempty(&table.table_id, "compatibility_table.table_id")?;
    ensure_nonempty(&table.table_ref, "compatibility_table.table_ref")?;
    if table.rows.is_empty() {
        return Err(err(
            "compatibility table must contain at least one row; a published guide may not ship an empty table",
        ));
    }
    let mut row_ids = BTreeSet::new();
    for row in &table.rows {
        ensure_nonempty(&row.row_id, "compatibility_table_row.row_id")?;
        if !row_ids.insert(&row.row_id) {
            return Err(err(format!(
                "duplicate compatibility row_id: {}",
                row.row_id
            )));
        }
        ensure_nonempty(
            &row.capability_area,
            "compatibility_table_row.capability_area",
        )?;
        ensure_nonempty(
            &row.source_feature_ref,
            "compatibility_table_row.source_feature_ref",
        )?;
        ensure_token(
            COMPATIBILITY_OUTCOME_LABELS,
            &row.outcome_label,
            "compatibility_table_row.outcome_label",
        )?;
    }

    let mut limit_ids = BTreeSet::new();
    for limit in &input.known_limits {
        ensure_nonempty(&limit.limit_id, "known_limit.limit_id")?;
        if !limit_ids.insert(&limit.limit_id) {
            return Err(err(format!(
                "duplicate known_limit limit_id: {}",
                limit.limit_id
            )));
        }
        ensure_nonempty(&limit.capability_area, "known_limit.capability_area")?;
        ensure_token(
            KNOWN_LIMIT_SEVERITY_CLASSES,
            &limit.severity_class,
            "known_limit.severity_class",
        )?;
        ensure_token(
            KNOWN_LIMIT_WORKAROUND_CLASSES,
            &limit.workaround_class,
            "known_limit.workaround_class",
        )?;
        // A non-blocking-workaround limit must name where its workaround lives.
        if limit.workaround_class != "no_workaround" && limit.workaround_ref.is_none() {
            return Err(err(format!(
                "known_limit {} names a workaround class but no workaround_ref",
                limit.limit_id
            )));
        }
    }

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim.claim_basis_class",
    )?;
    if let Some(table_ref) = &claim.compatibility_table_ref {
        if table_ref != &table.table_ref {
            return Err(err(
                "claim.compatibility_table_ref does not resolve to the published table",
            ));
        }
    }
    // A stable claimed tier must at least point at the table to be considered.
    if STABLE_TIERS.contains(&claim.claimed_tier.as_str())
        && claim.claim_basis_class == "evidence_backed"
        && claim.compatibility_table_ref.is_none()
    {
        return Err(err(
            "an evidence-backed stable claim must name a compatibility_table_ref",
        ));
    }

    let handoff = &input.handoff_disclosure;
    ensure_token(
        HANDOFF_SOURCE_CLASSES,
        &handoff.source_class,
        "handoff.source_class",
    )?;
    ensure_token(
        HANDOFF_ACTOR_CLASSES,
        &handoff.actor_class,
        "handoff.actor_class",
    )?;
    ensure_token(
        HANDOFF_FRESHNESS_CLASSES,
        &handoff.freshness_class,
        "handoff.freshness_class",
    )?;
    ensure_nonempty(&handoff.target_label, "handoff.target_label")?;
    ensure_nonempty(&handoff.return_path_label, "handoff.return_path_label")?;

    for surface in &input.consumer_surfaces {
        ensure_token(
            MIGRATION_SWITCHING_CONSUMER_SURFACES,
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
    identity: &MigrationGuideIdentity,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        MIGRATION_GUIDE_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        MIGRATION_SWITCHING_PUBLICATION_SCHEMA_VERSION,
        "identity schema_version",
    )?;
    ensure_token(
        MIGRATION_SOURCE_TOOLS,
        &identity.source_tool,
        "identity source_tool",
    )?;
    ensure_token(
        LAUNCH_COHORTS,
        &identity.launch_cohort,
        "identity launch_cohort",
    )?;
    ensure_token(
        GUIDE_EVIDENCE_FRESHNESS_CLASSES,
        &identity.evidence_freshness_class,
        "identity evidence_freshness_class",
    )?;
    Ok(())
}

fn validate_table(
    table: &MigrationCompatibilityTable,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_eq(
        table.record_kind.as_str(),
        MIGRATION_COMPATIBILITY_TABLE_RECORD_KIND,
        "table record_kind",
    )?;
    if table.rows.is_empty() {
        return Err(err("compatibility table must contain at least one row"));
    }
    for row in &table.rows {
        ensure_eq(
            row.record_kind.as_str(),
            MIGRATION_COMPATIBILITY_TABLE_ROW_RECORD_KIND,
            "table row record_kind",
        )?;
        ensure_token(
            COMPATIBILITY_OUTCOME_LABELS,
            &row.outcome_label,
            "table row outcome_label",
        )?;
    }
    Ok(())
}

fn validate_known_limit(
    limit: &SwitchingKnownLimit,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_eq(
        limit.record_kind.as_str(),
        SWITCHING_KNOWN_LIMIT_RECORD_KIND,
        "known_limit record_kind",
    )?;
    ensure_token(
        KNOWN_LIMIT_SEVERITY_CLASSES,
        &limit.severity_class,
        "known_limit severity_class",
    )?;
    ensure_token(
        KNOWN_LIMIT_WORKAROUND_CLASSES,
        &limit.workaround_class,
        "known_limit workaround_class",
    )?;
    let expected_blocking =
        limit.severity_class == "blocking" && limit.workaround_class == "no_workaround";
    if limit.blocks_stable_switch != expected_blocking {
        return Err(err(
            "known_limit blocks_stable_switch must reflect blocking severity with no workaround",
        ));
    }
    Ok(())
}

fn validate_claim(
    claim: &StableQualificationClaim,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        STABLE_QUALIFICATION_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim claimed_tier")?;
    ensure_token(
        STABILITY_TIERS,
        &claim.effective_tier,
        "claim effective_tier",
    )?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim claim_basis_class",
    )?;
    for reason in &claim.downgrade_reasons {
        ensure_token(SWITCH_DOWNGRADE_REASONS, reason, "claim downgrade_reason")?;
    }
    Ok(())
}

fn validate_handoff(
    handoff: &ProviderHandoffDisclosure,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_eq(
        handoff.record_kind.as_str(),
        PROVIDER_HANDOFF_DISCLOSURE_RECORD_KIND,
        "handoff record_kind",
    )?;
    ensure_token(
        HANDOFF_SOURCE_CLASSES,
        &handoff.source_class,
        "handoff source_class",
    )?;
    ensure_token(
        HANDOFF_ACTOR_CLASSES,
        &handoff.actor_class,
        "handoff actor_class",
    )?;
    ensure_token(
        HANDOFF_FRESHNESS_CLASSES,
        &handoff.freshness_class,
        "handoff freshness_class",
    )?;
    ensure_nonempty(&handoff.target_label, "handoff target_label")?;
    ensure_nonempty(&handoff.return_path_label, "handoff return_path_label")?;
    Ok(())
}

fn validate_inspection(
    inspection: &MigrationSwitchingPublicationInspection,
    packet: &MigrationSwitchingPublicationPacket,
) -> Result<(), MigrationSwitchingValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        MIGRATION_SWITCHING_PUBLICATION_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.guide_id_ref.as_str(),
        packet.identity.guide_id.as_str(),
        "inspection guide_id_ref",
    )?;
    ensure_eq(
        inspection.effective_tier.as_str(),
        packet.claim.effective_tier.as_str(),
        "inspection effective_tier",
    )?;
    if inspection.compatibility_row_count != packet.compatibility_table.rows.len() {
        return Err(err(
            "inspection compatibility_row_count must match table rows",
        ));
    }
    if inspection.known_limit_count != packet.known_limits.len() {
        return Err(err("inspection known_limit_count must match known-limits"));
    }
    if inspection.no_prose_only_stable_claim != packet.no_prose_only_stable_claim() {
        return Err(err("inspection no_prose_only_stable_claim is inconsistent"));
    }
    if inspection.resolves_to_current_compatibility_table
        != packet.resolves_to_current_compatibility_table()
    {
        return Err(err(
            "inspection resolves_to_current_compatibility_table is inconsistent",
        ));
    }
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    if inspection.reversible != packet.identity.reversible {
        return Err(err("inspection reversible is inconsistent"));
    }
    if inspection.attribution_complete != packet.handoff_disclosure.is_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> MigrationSwitchingValidationError {
    MigrationSwitchingValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), MigrationSwitchingValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), MigrationSwitchingValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), MigrationSwitchingValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), MigrationSwitchingValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
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

    fn row(id: &str, outcome: &str, core: bool) -> MigrationCompatibilityTableRowInput {
        MigrationCompatibilityTableRowInput {
            row_id: id.to_string(),
            capability_area: "keybindings".to_string(),
            source_feature_ref: "feature.keybindings".to_string(),
            outcome_label: outcome.to_string(),
            is_core_capability: core,
            derived_from_artifact_ref: Some("fixtures/migration/keybindings.json".to_string()),
            guide_section_ref: Some("docs#keybindings".to_string()),
            summary_label: "Keybindings row".to_string(),
        }
    }

    fn complete_handoff() -> ProviderHandoffDisclosureInput {
        ProviderHandoffDisclosureInput {
            source_class: "local_only".to_string(),
            actor_class: "local_user".to_string(),
            freshness_class: "live_current".to_string(),
            target_label: "Local Aureline workspace".to_string(),
            return_path_label: "Returns to migration center".to_string(),
            discloses_ownership: true,
        }
    }

    fn base_input() -> MigrationSwitchingPublicationInput {
        MigrationSwitchingPublicationInput {
            packet_id: "pkt1".to_string(),
            generated_at: "2026-05-31T10:00:00Z".to_string(),
            identity: MigrationGuideIdentityInput {
                guide_id: "guide.vscode.solo".to_string(),
                guide_revision: 1,
                source_tool: "vs_code_code_oss".to_string(),
                launch_cohort: "solo_switcher".to_string(),
                guide_ref: "docs/m4/publish-stable-migration-guides.md".to_string(),
                evidence_freshness_class: "fresh_current".to_string(),
                freshness_expires_at: "2026-12-31T00:00:00Z".to_string(),
                reversible: true,
                previewable: true,
            },
            compatibility_table: MigrationCompatibilityTableInput {
                table_id: "table.vscode.solo".to_string(),
                table_ref: "artifacts/compat/vscode_solo.json".to_string(),
                generated_at: "2026-05-31T09:00:00Z".to_string(),
                rows: vec![row("r1", "exact", true), row("r2", "translated", false)],
            },
            known_limits: vec![SwitchingKnownLimitInput {
                limit_id: "limit.minor".to_string(),
                capability_area: "themes".to_string(),
                severity_class: "minor".to_string(),
                workaround_class: "native_alternative".to_string(),
                workaround_ref: Some("docs#themes".to_string()),
                summary_label: "Minor theme difference".to_string(),
            }],
            claim: StableQualificationClaimInput {
                claimed_tier: "stable".to_string(),
                claim_basis_class: "evidence_backed".to_string(),
                compatibility_table_ref: Some("artifacts/compat/vscode_solo.json".to_string()),
                summary_label: "Stable switch for VS Code solo switchers".to_string(),
            },
            handoff_disclosure: complete_handoff(),
            consumer_surfaces: vec![
                "migration_center".to_string(),
                "docs_help_surface".to_string(),
                "release_packet".to_string(),
                "support_export".to_string(),
            ],
            summary_label: "VS Code solo switcher migration guide".to_string(),
        }
    }

    #[test]
    fn closed_vocabularies_hold_their_anchors() {
        assert!(COMPATIBILITY_OUTCOME_LABELS.contains(&"exact"));
        assert!(COMPATIBILITY_OUTCOME_LABELS.contains(&"translated"));
        assert!(COMPATIBILITY_OUTCOME_LABELS.contains(&"partial"));
        assert!(COMPATIBILITY_OUTCOME_LABELS.contains(&"shimmed"));
        assert!(COMPATIBILITY_OUTCOME_LABELS.contains(&"unsupported"));
        assert!(STABLE_TIERS.contains(&"stable"));
        // Stable tiers must be a strict subset of the tier set.
        assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
        assert!(STABILITY_TIERS.contains(&"beta"));
        assert!(STABILITY_TIERS.contains(&"preview"));
        assert!(SWITCH_DOWNGRADE_REASONS.contains(&"evidence_freshness_expired"));
        assert!(SWITCH_DOWNGRADE_REASONS.contains(&"compatibility_table_unsupported_present"));
    }

    #[test]
    fn stable_guide_with_current_table_holds() {
        let packet =
            MigrationSwitchingPublicationPacket::from_input(base_input()).expect("must project");
        assert_eq!(packet.claim.effective_tier, "stable");
        assert!(!packet.claim.downgraded);
        assert_eq!(
            packet.claim.support_claim_class,
            "stable_switch_ready_claim"
        );
        assert!(packet.inspection.stable_claim);
        assert!(packet.inspection.resolves_to_current_compatibility_table);
        assert!(packet.no_prose_only_stable_claim());
    }

    #[test]
    fn stale_evidence_narrows_to_beta() {
        let mut input = base_input();
        input.identity.evidence_freshness_class = "stale_past_window".to_string();
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert_eq!(packet.claim.effective_tier, "beta");
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"evidence_freshness_expired".to_string()));
        assert!(!packet.inspection.stable_claim);
    }

    #[test]
    fn unsupported_core_capability_narrows_to_preview() {
        let mut input = base_input();
        input.compatibility_table.rows[0].outcome_label = "unsupported".to_string();
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert_eq!(packet.claim.effective_tier, "preview");
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"compatibility_table_unsupported_present".to_string()));
    }

    #[test]
    fn blocking_known_limit_narrows_to_preview() {
        let mut input = base_input();
        input.known_limits.push(SwitchingKnownLimitInput {
            limit_id: "limit.blocking".to_string(),
            capability_area: "debugger".to_string(),
            severity_class: "blocking".to_string(),
            workaround_class: "no_workaround".to_string(),
            workaround_ref: None,
            summary_label: "No debugger bridge yet".to_string(),
        });
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert_eq!(packet.claim.effective_tier, "preview");
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"known_limit_blocking_unresolved".to_string()));
        assert_eq!(packet.inspection.blocking_known_limit_count, 1);
    }

    #[test]
    fn prose_only_stable_claim_is_narrowed() {
        let mut input = base_input();
        input.claim.claim_basis_class = "prose_only".to_string();
        input.claim.compatibility_table_ref = None;
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"prose_only_claim".to_string()));
    }

    #[test]
    fn irreversible_switch_narrows_below_stable() {
        let mut input = base_input();
        input.identity.reversible = false;
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"not_reversible".to_string()));
    }

    #[test]
    fn incomplete_attribution_narrows_below_stable() {
        let mut input = base_input();
        input.handoff_disclosure.discloses_ownership = false;
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert!(packet.claim.downgraded);
        assert!(packet
            .claim
            .downgrade_reasons
            .contains(&"attribution_incomplete".to_string()));
    }

    #[test]
    fn honest_beta_claim_passes_through() {
        let mut input = base_input();
        input.claim.claimed_tier = "beta".to_string();
        input.claim.compatibility_table_ref = None;
        input.compatibility_table.rows[1].outcome_label = "partial".to_string();
        let packet = MigrationSwitchingPublicationPacket::from_input(input).expect("must project");
        assert_eq!(packet.claim.effective_tier, "beta");
        assert!(!packet.claim.downgraded);
        assert_eq!(
            packet.claim.support_claim_class,
            "beta_switch_partial_claim"
        );
    }

    #[test]
    fn empty_table_is_rejected() {
        let mut input = base_input();
        input.compatibility_table.rows.clear();
        let result = MigrationSwitchingPublicationPacket::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn unknown_source_tool_is_rejected() {
        let mut input = base_input();
        input.identity.source_tool = "notepad".to_string();
        let result = MigrationSwitchingPublicationPacket::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn projection_roundtrips_through_json() {
        let packet =
            MigrationSwitchingPublicationPacket::from_input(base_input()).expect("must project");
        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection =
            project_migration_switching_publication(&payload).expect("must project from json");
        assert_eq!(projection.guide_id, "guide.vscode.solo");
        assert_eq!(projection.effective_tier, "stable");
        assert!(projection.resolves_to_current_compatibility_table);
        assert_eq!(projection.compatibility_row_count, 2);
    }

    #[test]
    fn known_limit_without_workaround_ref_is_rejected() {
        let mut input = base_input();
        input.known_limits[0].workaround_ref = None;
        let result = MigrationSwitchingPublicationPacket::from_input(input);
        assert!(result.is_err());
    }
}
