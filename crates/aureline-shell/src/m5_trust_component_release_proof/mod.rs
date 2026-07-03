//! Settings-row, capability-sheet, evidence-chronology, and chronology-export truth certified as a
//! single release-evidence bundle across every claimed M5 trust, config, activity, and support
//! surface.
//!
//! The [frozen trust-chronology component matrix][matrix] already freezes Aureline's highest-trust
//! reusable components — the settings row, the permission/capability sheet, the event/history row,
//! the timeline group, the narrative summary card, and the chronology export preview — into one
//! export-safe packet: their settings-row states and source pills, the capability consequence
//! classes and scope states, the chronology verbs and provenance badges, the chronology detail
//! states and export fields, the non-visual accessibility routes, the mandatory labels every
//! component must be able to show, and the downgrade triggers that narrow them below a claim. This
//! lane is the **release-evidence certification capstone** that closes the B88 component-contract
//! lane on top of that matrix: for every governed component family it certifies, per-family and
//! against the family's own frozen matrix row, that the component's contract truth
//! (effective-versus-configured value with source and lock state, consequence-grouped scope with
//! re-consent, or stable verb / provenance / portable export) stays certified on every claimed M5
//! surface; that the same reusable component reads with one row grammar across every surface rather
//! than reinventing itself off the primary surface; that a support export plus screenshot/golden
//! baselines reconstruct the component's truth without a live screenshot; and that the exported
//! proof stays fresh so a consumer that drifts off the shared contract auto-narrows rather than
//! keeping a stale claim.
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`TrustReleaseProofRow`]): one row per
//!   [`M5TrustComponentFamily`] naming the truth pillars it certifies, the settings-row states /
//!   source pills / consequence classes / scope states / chronology verbs / provenance badges /
//!   detail states / export fields / accessibility routes / required labels / shell zone /
//!   responsive classes / window classes / surface families / consumer surfaces / downgrade triggers
//!   pulled from the frozen matrix row for that family, its contract-truth / cross-surface-parity /
//!   support-export-proof / proof-freshness posture, any active waiver, and a derived green/yellow/red
//!   [`TrustReleaseProofStatus`].
//! - the release **certification packet** ([`TrustReleaseProofPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   certification causes ([`TrustReleaseProofCause`]), and the blocking findings the lane refuses to
//!   ship with — including the packet-level proof that the union of certified truth pillars covers
//!   the whole track invariant.
//! - the **certification dashboard** ([`TrustReleaseProofDashboard`]): a light projection the shell /
//!   release automation / support export reads to auto-narrow a claimed component family when its
//!   component truth or proof freshness falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment it
//! discloses a reduced contract-truth detail, a reduced cross-surface projection (behind a waiver), a
//! partial support-export capture, or a partial proof refresh; it drops to `red` if a component's
//! contract truth collapses or drifts, its row grammar diverges off the primary surface, its truth is
//! absent from the support-export capture, its exported proof is stale or divergent, it drops audit /
//! support truth, it fails to certify every claimed M5 surface family, or it declares no truth
//! pillars. That derivation is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary,
//! counts, refs, and short labels. The component vocabulary is re-exported by reference from the
//! already frozen [matrix]; each row pulls its component bindings straight from the matrix's seeded
//! per-family component row, so this lane mints no parallel component vocabulary and cannot certify a
//! release posture the matrix does not freeze. Only the certification-specific vocabulary
//! ([`M5TrustComponentTruthPillar`], [`M5TrustReleaseProofDimension`], [`TrustReleaseProofStatus`],
//! [`ComponentContractTruthState`], [`CrossSurfaceParityState`], [`SupportExportProofState`],
//! [`ProofFreshnessState`], [`TrustReleaseProofWaiver`], [`TrustReleaseProofCause`],
//! [`TrustReleaseProofFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix as matrix;

pub use matrix::{
    M5CapabilityConsequenceClass, M5CapabilityScopeState, M5ChronologyDetailState,
    M5ChronologyExportField, M5ChronologyVerb, M5ProvenanceBadge, M5ResponsiveClass,
    M5SettingSourcePill, M5SettingsRowState, M5ShellConsumerSurface, M5ShellSurfaceFamily,
    M5ShellZoneSlot, M5TrustAccessibilityRoute, M5TrustComponentDowngradeTrigger,
    M5TrustComponentFamily, M5TrustQualificationClass, M5TrustRequiredLabel, M5WindowClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_trust_component_release_proof_packet,
    seeded_m5_trust_component_release_proof_packet_capability_sheet_row_grammar_diverged_blocked,
    seeded_m5_trust_component_release_proof_packet_event_history_row_capture_absent_blocked,
    seeded_m5_trust_component_release_proof_packet_narrative_summary_card_audit_truth_dropped_blocked,
    seeded_m5_trust_component_release_proof_packet_settings_row_contract_truth_collapsed_blocked,
    seeded_m5_trust_component_release_proof_packet_timeline_group_proof_stale_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_TRUST_RELEASE_PROOF_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_TRUST_RELEASE_PROOF_SHARED_CONTRACT_REF: &str =
    "shell:m5_trust_component_release_proof:v1";

/// Stable record kind for [`TrustReleaseProofPacket`] payloads.
pub const M5_TRUST_RELEASE_PROOF_PACKET_RECORD_KIND: &str =
    "shell_m5_trust_component_release_proof_packet_record";

/// Stable record kind for [`TrustReleaseProofDashboard`] payloads.
pub const M5_TRUST_RELEASE_PROOF_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_trust_component_release_proof_dashboard_record";

/// Stable record kind for [`TrustReleaseProofSupportExport`] payloads.
pub const M5_TRUST_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_trust_component_release_proof_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_TRUST_RELEASE_PROOF_PACKET_ID: &str = "m5-trust-component-release-proof:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_TRUST_RELEASE_PROOF_DASHBOARD_ID: &str =
    "m5-trust-component-release-proof-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_TRUST_RELEASE_PROOF_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-trust-component-release-proof:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_TRUST_RELEASE_PROOF_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-trust-component-release-proof.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_TRUST_RELEASE_PROOF_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-trust-component-release-proof.md";

/// Published certification-packet artifact ref.
pub const M5_TRUST_RELEASE_PROOF_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-trust-component-release-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_TRUST_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-trust-component-release-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_TRUST_RELEASE_PROOF_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-trust-component-release-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_TRUST_RELEASE_PROOF_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-trust-component-release-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_TRUST_RELEASE_PROOF_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_trust_component_release_proof_contract.md";

/// Repo-relative ref to the frozen trust-chronology component matrix schema.
pub const M5_TRUST_RELEASE_PROOF_MATRIX_SCHEMA_REF: &str = matrix::M5_TRUST_COMPONENTS_SCHEMA_REF;

/// One of the three high-trust component-truth pillars named by the lane's track invariant. Every
/// certified component family declares at least one; the union across the six families must cover the
/// full set so the release bundle certifies the whole track invariant, not one feature family's slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustComponentTruthPillar {
    /// Settings pillar: effective-versus-configured value carried with its source pill and lock
    /// state.
    EffectiveValueSourceAndLock,
    /// Capability pillar: consequence-grouped permission scope with transitive scope and re-consent.
    ConsequenceScopeAndReconsent,
    /// Chronology pillar: stable verbs, provenance badges, and portable detail / export.
    ChronologyVerbProvenanceAndExport,
}

impl M5TrustComponentTruthPillar {
    /// Every truth pillar, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::EffectiveValueSourceAndLock,
        Self::ConsequenceScopeAndReconsent,
        Self::ChronologyVerbProvenanceAndExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveValueSourceAndLock => "effective_value_source_and_lock",
            Self::ConsequenceScopeAndReconsent => "consequence_scope_and_reconsent",
            Self::ChronologyVerbProvenanceAndExport => "chronology_verb_provenance_and_export",
        }
    }
}

/// The truth pillars a governed component family carries. Settings rows carry the effective-value
/// pillar, capability sheets carry the consequence-scope pillar, and every chronology component
/// (event row, timeline group, narrative card, export preview) carries the chronology pillar.
pub fn applicable_truth_pillars(
    family: M5TrustComponentFamily,
) -> Vec<M5TrustComponentTruthPillar> {
    match family {
        M5TrustComponentFamily::SettingsRow => {
            vec![M5TrustComponentTruthPillar::EffectiveValueSourceAndLock]
        }
        M5TrustComponentFamily::CapabilitySheet => {
            vec![M5TrustComponentTruthPillar::ConsequenceScopeAndReconsent]
        }
        M5TrustComponentFamily::EventHistoryRow
        | M5TrustComponentFamily::TimelineGroup
        | M5TrustComponentFamily::NarrativeSummaryCard
        | M5TrustComponentFamily::ChronologyExportPreview => {
            vec![M5TrustComponentTruthPillar::ChronologyVerbProvenanceAndExport]
        }
    }
}

/// The frozen downgrade trigger a family reaches for when its core contract truth collapses. Each
/// value is present in the family's own frozen matrix downgrade-trigger list, so a cause never mints
/// a parallel reason synonym.
pub fn primary_contract_trigger(
    family: M5TrustComponentFamily,
) -> M5TrustComponentDowngradeTrigger {
    match family {
        M5TrustComponentFamily::SettingsRow => {
            M5TrustComponentDowngradeTrigger::EffectiveConfiguredConflated
        }
        M5TrustComponentFamily::CapabilitySheet => {
            M5TrustComponentDowngradeTrigger::ConsequenceGroupingDropped
        }
        M5TrustComponentFamily::EventHistoryRow
        | M5TrustComponentFamily::TimelineGroup
        | M5TrustComponentFamily::NarrativeSummaryCard => {
            M5TrustComponentDowngradeTrigger::VerbVocabularyDrift
        }
        M5TrustComponentFamily::ChronologyExportPreview => {
            M5TrustComponentDowngradeTrigger::ExportFieldDropped
        }
    }
}

/// Short reviewer-facing label for a governed component family.
pub const fn component_family_label(family: M5TrustComponentFamily) -> &'static str {
    match family {
        M5TrustComponentFamily::SettingsRow => "Settings row",
        M5TrustComponentFamily::CapabilitySheet => "Capability sheet",
        M5TrustComponentFamily::EventHistoryRow => "Event / history row",
        M5TrustComponentFamily::TimelineGroup => "Timeline group",
        M5TrustComponentFamily::NarrativeSummaryCard => "Narrative summary card",
        M5TrustComponentFamily::ChronologyExportPreview => "Chronology export preview",
    }
}

/// One of the four certification dimensions each component family is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustReleaseProofDimension {
    /// The component's core contract truth (effective value / scope / verb-provenance-export) stays
    /// certified on every claimed surface.
    ComponentContractTruth,
    /// The same reusable component reads with one row grammar across every claimed surface.
    CrossSurfaceParity,
    /// A support export plus screenshot/golden baselines reconstruct the component truth.
    SupportExportProof,
    /// The exported proof stays fresh so a drifted consumer auto-narrows.
    ProofFreshness,
}

impl M5TrustReleaseProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ComponentContractTruth,
        Self::CrossSurfaceParity,
        Self::SupportExportProof,
        Self::ProofFreshness,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentContractTruth => "component_contract_truth",
            Self::CrossSurfaceParity => "cross_surface_parity",
            Self::SupportExportProof => "support_export_proof",
            Self::ProofFreshness => "proof_freshness",
        }
    }
}

/// The derived certification light a governed component family carries.
///
/// `green` means the component's contract truth stays certified on every claimed M5 surface, reads
/// with one row grammar everywhere, reconstructs from a support export plus screenshot/golden
/// baselines, and keeps its exported proof fresh. `yellow` is a disclosed narrowing (a reduced
/// contract-truth detail, a waivered reduced cross-surface projection, a partial support-export
/// capture, or a partial proof refresh). `red` is blocked: the contract truth collapsed or drifted,
/// the row grammar diverged off the primary surface, the truth is absent from capture, the exported
/// proof is stale or divergent, audit / support truth is dropped, a claimed M5 surface family is not
/// certified, or the family declares no truth pillars — and the family may not keep a
/// trust-component-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustReleaseProofStatus {
    /// Full standing: certified, parity-stable, reconstructable, fresh.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl TrustReleaseProofStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the family keeps its core contract truth certified on every claimed M5 surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentContractTruthState {
    /// The component's effective-value / scope / verb-provenance-export contract truth is certified
    /// on every claimed surface: nothing is conflated, dropped, or paraphrased.
    ContractTruthCertifiedEverySurface,
    /// A contract-truth detail is disclosedly reduced (a rarely-seen source pill or a low-frequency
    /// verb is summarized on a secondary surface) while the core truth stays certified and the
    /// reduction is disclosed.
    DisclosedReducedContractTruth,
    /// The component's contract truth collapses into a generic value or drifts from the frozen
    /// vocabulary on a claimed surface — always a blocker.
    ContractTruthCollapsedOrDrifted,
}

impl ComponentContractTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContractTruthCertifiedEverySurface => "contract_truth_certified_every_surface",
            Self::DisclosedReducedContractTruth => "disclosed_reduced_contract_truth",
            Self::ContractTruthCollapsedOrDrifted => "contract_truth_collapsed_or_drifted",
        }
    }

    /// `true` when the contract truth is certified everywhere.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::ContractTruthCertifiedEverySurface)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedContractTruth)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ContractTruthCollapsedOrDrifted)
    }
}

/// How the family keeps one row grammar across every claimed surface rather than reinventing itself
/// off the primary surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossSurfaceParityState {
    /// The reusable component reads with the same row grammar, state vocabulary, and reopen path on
    /// every claimed surface.
    ParityCertifiedAcrossSurfaces,
    /// A surface projection is disclosedly reduced (a compact secondary surface shows a summarized
    /// projection) while the shared row grammar is preserved and the reduction is disclosed and
    /// waivered.
    DisclosedReducedSurfaceProjection,
    /// The component reinvents a second row grammar off the primary surface, so the same truth reads
    /// differently across surfaces — always a blocker.
    RowGrammarDivergedOffPrimarySurface,
}

impl CrossSurfaceParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityCertifiedAcrossSurfaces => "parity_certified_across_surfaces",
            Self::DisclosedReducedSurfaceProjection => "disclosed_reduced_surface_projection",
            Self::RowGrammarDivergedOffPrimarySurface => "row_grammar_diverged_off_primary_surface",
        }
    }

    /// `true` when parity is certified across surfaces.
    pub const fn is_parity(self) -> bool {
        matches!(self, Self::ParityCertifiedAcrossSurfaces)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedSurfaceProjection)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::RowGrammarDivergedOffPrimarySurface)
    }
}

/// How the family's component truth survives copied evidence, saved packets, and screenshot/golden
/// baselines — reconstructable from a support export without a live screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportProofState {
    /// The support export plus the screenshot/golden baselines reconstruct the component truth — its
    /// source / scope / chronology language and its export fields — in the same language shown
    /// in-product, so a component regression can be diagnosed without a live screenshot.
    ReconstructableInExportAndScreenshot,
    /// The support export reconstructs the component truth and discloses a partial capture (some
    /// low-priority component detail is trimmed) while the reduction is disclosed.
    DisclosedPartialCapture,
    /// The family's component truth is absent from the support-export capture, so a component
    /// regression cannot be explained without a live screenshot — always a blocker.
    ComponentTruthAbsentFromCapture,
}

impl SupportExportProofState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableInExportAndScreenshot => {
                "reconstructable_in_export_and_screenshot"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::ComponentTruthAbsentFromCapture => "component_truth_absent_from_capture",
        }
    }

    /// `true` when the export and baselines reconstruct the component truth.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::ReconstructableInExportAndScreenshot)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ComponentTruthAbsentFromCapture)
    }
}

/// How the family keeps its exported proof current so a consumer that drifts off the shared contract
/// auto-narrows rather than keeping a stale claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofFreshnessState {
    /// The exported proof reflects the current component contract and is within the freshness SLO, so
    /// the claim is backed by a current proof.
    ExportedProofFreshAndCurrent,
    /// The exported proof is refreshed and discloses a partial refresh (a low-priority slice awaits
    /// the next refresh) while the current claim stays backed and the reduction is disclosed.
    DisclosedPartialRefresh,
    /// The exported proof is stale or divergent from the current component contract, so the claim is
    /// no longer backed by a current proof — always a blocker.
    ExportedProofStaleOrDivergent,
}

impl ProofFreshnessState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportedProofFreshAndCurrent => "exported_proof_fresh_and_current",
            Self::DisclosedPartialRefresh => "disclosed_partial_refresh",
            Self::ExportedProofStaleOrDivergent => "exported_proof_stale_or_divergent",
        }
    }

    /// `true` when the exported proof is fresh and current.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::ExportedProofFreshAndCurrent)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialRefresh)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ExportedProofStaleOrDivergent)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red reduced cross-surface projection stay
/// yellow rather than blocked — never lets a collapsed contract truth, a diverged row grammar, a
/// missing export, or a stale proof hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed component family the waiver applies to.
    pub component_family: M5TrustComponentFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl TrustReleaseProofWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's certification.
///
/// The trigger token mirrors the frozen [`M5TrustComponentDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym. Contract-truth causes reach for the family's own primary
/// contract trigger; cross-surface and audit-truth causes use `AuditTruthLostOffPrimarySurface`;
/// support-export and proof-freshness causes use `ProofStale`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofCause {
    /// The governed component family the cause applies to.
    pub component_family: M5TrustComponentFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5TrustComponentDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl TrustReleaseProofCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed component family, certified across contract truth, cross-surface parity,
/// support-export proof, and proof freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofRow {
    /// The governed component family being certified.
    pub component_family: M5TrustComponentFamily,
    /// The frozen qualification class for this family. Pulled from the matrix.
    pub matrix_qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this family certified. Pulled from the matrix.
    pub owner_role: String,
    /// Short component-family label.
    pub family_label: String,
    /// Human-readable scope summary. Pulled from the matrix.
    pub scope_summary: String,
    /// Truth pillars this family certifies (family-specific).
    pub certified_truth_pillars: Vec<M5TrustComponentTruthPillar>,
    /// Canonical shell zone this component attaches to. Pulled from the matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component survives. Pulled from the matrix.
    pub certified_responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across. Pulled from the matrix.
    pub certified_window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render this component. Pulled from the matrix.
    pub certified_surface_families: Vec<M5ShellSurfaceFamily>,
    /// Settings-row states this family projects (settings only). Pulled from the matrix.
    pub certified_settings_row_states: Vec<M5SettingsRowState>,
    /// Source pills this family shows (settings only). Pulled from the matrix.
    pub certified_source_pills: Vec<M5SettingSourcePill>,
    /// Capability consequence classes this family groups by (capability only). Pulled from the
    /// matrix.
    pub certified_consequence_classes: Vec<M5CapabilityConsequenceClass>,
    /// Capability scope states this family honours (capability only). Pulled from the matrix.
    pub certified_capability_scope_states: Vec<M5CapabilityScopeState>,
    /// Chronology verbs this family uses (chronology only). Pulled from the matrix.
    pub certified_chronology_verbs: Vec<M5ChronologyVerb>,
    /// Provenance badges this family attributes (chronology only). Pulled from the matrix.
    pub certified_provenance_badges: Vec<M5ProvenanceBadge>,
    /// Chronology detail states this family honours (grouping only). Pulled from the matrix.
    pub certified_chronology_detail_states: Vec<M5ChronologyDetailState>,
    /// Chronology export fields this family promises (export only). Pulled from the matrix.
    pub certified_chronology_export_fields: Vec<M5ChronologyExportField>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Mandatory labels this family must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5TrustRequiredLabel>,
    /// Shell subsystems that consume this family's projection. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// Contract-truth posture.
    pub component_contract_truth: ComponentContractTruthState,
    /// Cross-surface-parity posture.
    pub cross_surface_parity: CrossSurfaceParityState,
    /// Support-export-proof posture.
    pub support_export_proof: SupportExportProofState,
    /// Proof-freshness posture.
    pub proof_freshness: ProofFreshnessState,
    /// Hard invariant: no audit / support truth is dropped off the primary surface. `false` is a
    /// blocker.
    pub never_drops_audit_or_support_truth: bool,
    /// Active waiver, when a disclosed reduced cross-surface projection is in force.
    pub active_waiver: Option<TrustReleaseProofWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: TrustReleaseProofStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<TrustReleaseProofCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl TrustReleaseProofRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the family certifies every claimed M5 surface family — the lint that prevents a
    /// family from keeping a claim while a claimed M5 surface is left uncertified. This is the
    /// per-surface-family auto-narrowing the acceptance criteria require.
    pub fn surface_families_complete(&self) -> bool {
        let present: BTreeSet<M5ShellSurfaceFamily> =
            self.certified_surface_families.iter().copied().collect();
        M5ShellSurfaceFamily::ALL
            .iter()
            .all(|family| present.contains(family))
    }

    /// `true` when the family declares exactly its applicable truth pillars — the lint that prevents
    /// a family from shipping without naming the component-truth pillar it carries.
    pub fn truth_pillars_declared(&self) -> bool {
        !self.certified_truth_pillars.is_empty()
            && self.certified_truth_pillars == applicable_truth_pillars(self.component_family)
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.component_contract_truth.is_blocked()
            || self.cross_surface_parity.is_blocked()
            || self.support_export_proof.is_blocked()
            || self.proof_freshness.is_blocked()
            || !self.never_drops_audit_or_support_truth
            || !self.surface_families_complete()
            || !self.truth_pillars_declared()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.component_contract_truth.is_disclosed()
            || self.cross_surface_parity.is_disclosed()
            || self.support_export_proof.is_disclosed()
            || self.proof_freshness.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the audit-truth invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> TrustReleaseProofStatus {
        if self.has_hard_blocker() {
            TrustReleaseProofStatus::Red
        } else if self.has_narrowing() {
            TrustReleaseProofStatus::Yellow
        } else {
            TrustReleaseProofStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order (contract truth,
    /// cross-surface parity, support-export, proof freshness, audit-truth invariant).
    pub fn recompute_causes(&self) -> Vec<TrustReleaseProofCause> {
        let mut causes = Vec::new();
        if !self.component_contract_truth.is_certified() {
            causes.push(TrustReleaseProofCause {
                component_family: self.component_family,
                trigger: primary_contract_trigger(self.component_family),
                disclosed: self.component_contract_truth.is_disclosed(),
                detail: if self.component_contract_truth.is_disclosed() {
                    "A contract-truth detail is disclosedly reduced (a rarely-seen source pill or a \
                     low-frequency verb is summarized on a secondary surface) while the core truth \
                     stays certified; the reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The component's contract truth collapses into a generic value or drifts from the \
                     frozen vocabulary on a claimed surface."
                        .to_owned()
                },
            });
        }
        if !self.cross_surface_parity.is_parity() {
            causes.push(TrustReleaseProofCause {
                component_family: self.component_family,
                trigger: M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
                disclosed: self.cross_surface_parity.is_disclosed(),
                detail: if self.cross_surface_parity.is_disclosed() {
                    "A surface projection is disclosedly reduced (a compact secondary surface shows a \
                     summarized projection) while the shared row grammar is preserved; the reduction \
                     is disclosed behind a waiver and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The component reinvents a second row grammar off the primary surface, so the \
                     same truth reads differently across surfaces."
                        .to_owned()
                },
            });
        }
        if !self.support_export_proof.is_reconstructable() {
            causes.push(TrustReleaseProofCause {
                component_family: self.component_family,
                trigger: M5TrustComponentDowngradeTrigger::ProofStale,
                disclosed: self.support_export_proof.is_disclosed(),
                detail: if self.support_export_proof.is_disclosed() {
                    "The support export reconstructs the component truth and discloses a partial \
                     capture (some low-priority component detail is trimmed) while the reduction is \
                     disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The family's component truth is absent from the support-export capture, so a \
                     component regression cannot be explained without a live screenshot."
                        .to_owned()
                },
            });
        }
        if !self.proof_freshness.is_fresh() {
            causes.push(TrustReleaseProofCause {
                component_family: self.component_family,
                trigger: M5TrustComponentDowngradeTrigger::ProofStale,
                disclosed: self.proof_freshness.is_disclosed(),
                detail: if self.proof_freshness.is_disclosed() {
                    "The exported proof is refreshed and discloses a partial refresh (a low-priority \
                     slice awaits the next refresh) while the current claim stays backed; the \
                     reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The exported proof is stale or divergent from the current component contract, so \
                     the claim is no longer backed by a current proof."
                        .to_owned()
                },
            });
        }
        if !self.never_drops_audit_or_support_truth {
            causes.push(TrustReleaseProofCause {
                component_family: self.component_family,
                trigger: M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
                disclosed: false,
                detail: "The component drops audit / support truth off the primary surface, so a \
                         setting, capability, or chronology truth cannot be reconstructed off the \
                         surface that rendered it."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced cross-surface projection may only stay yellow (rather than red) when a
    /// waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.cross_surface_parity.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<TrustReleaseProofFinding> {
        let mut findings = Vec::new();
        let family = self.component_family.as_str().to_owned();

        if self.component_contract_truth.is_blocked() {
            findings.push(TrustReleaseProofFinding::ContractTruthCollapsedOrDrifted {
                family: family.clone(),
            });
        }
        if self.cross_surface_parity.is_blocked() {
            findings.push(
                TrustReleaseProofFinding::RowGrammarDivergedOffPrimarySurface {
                    family: family.clone(),
                },
            );
        }
        if self.support_export_proof.is_blocked() {
            findings.push(TrustReleaseProofFinding::ComponentTruthAbsentFromCapture {
                family: family.clone(),
            });
        }
        if self.proof_freshness.is_blocked() {
            findings.push(TrustReleaseProofFinding::ExportedProofStaleOrDivergent {
                family: family.clone(),
            });
        }
        if !self.never_drops_audit_or_support_truth {
            findings.push(TrustReleaseProofFinding::AuditOrSupportTruthDropped {
                family: family.clone(),
            });
        }
        if !self.surface_families_complete() {
            findings.push(TrustReleaseProofFinding::SurfaceFamiliesIncomplete {
                family: family.clone(),
            });
        }
        if !self.truth_pillars_declared() {
            findings.push(TrustReleaseProofFinding::TruthPillarsUndeclared {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, TrustReleaseProofStatus::Green) && !self.has_reason() {
            findings.push(TrustReleaseProofFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(TrustReleaseProofFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.component_family != self.component_family {
                findings.push(TrustReleaseProofFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(TrustReleaseProofFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(TrustReleaseProofFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(TrustReleaseProofFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} contract={} parity={} export={} freshness={} no_dropped_audit_truth={} waiver={}",
            self.component_family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.component_contract_truth.as_str(),
            self.cross_surface_parity.as_str(),
            self.support_export_proof.as_str(),
            self.proof_freshness.as_str(),
            self.never_drops_audit_or_support_truth,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the release-proof certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum TrustReleaseProofFinding {
    /// A governed component family has no certification row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A family's contract truth collapses or drifts on a claimed surface.
    ContractTruthCollapsedOrDrifted {
        /// The family token.
        family: String,
    },
    /// A family reinvents a second row grammar off the primary surface.
    RowGrammarDivergedOffPrimarySurface {
        /// The family token.
        family: String,
    },
    /// A family's component truth is absent from the support-export capture.
    ComponentTruthAbsentFromCapture {
        /// The family token.
        family: String,
    },
    /// A family's exported proof is stale or divergent.
    ExportedProofStaleOrDivergent {
        /// The family token.
        family: String,
    },
    /// A family drops audit / support truth off the primary surface (hard invariant).
    AuditOrSupportTruthDropped {
        /// The family token.
        family: String,
    },
    /// A family does not certify every claimed M5 surface family.
    SurfaceFamiliesIncomplete {
        /// The family token.
        family: String,
    },
    /// A family declares no (or the wrong) truth pillars.
    TruthPillarsUndeclared {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// The union of certified truth pillars does not cover the whole track invariant.
    TruthPillarCoverageIncomplete,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl TrustReleaseProofFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::ContractTruthCollapsedOrDrifted { .. } => "contract_truth_collapsed_or_drifted",
            Self::RowGrammarDivergedOffPrimarySurface { .. } => {
                "row_grammar_diverged_off_primary_surface"
            }
            Self::ComponentTruthAbsentFromCapture { .. } => "component_truth_absent_from_capture",
            Self::ExportedProofStaleOrDivergent { .. } => "exported_proof_stale_or_divergent",
            Self::AuditOrSupportTruthDropped { .. } => "audit_or_support_truth_dropped",
            Self::SurfaceFamiliesIncomplete { .. } => "surface_families_incomplete",
            Self::TruthPillarsUndeclared { .. } => "truth_pillars_undeclared",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::TruthPillarCoverageIncomplete => "truth_pillar_coverage_incomplete",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::FamilyMissing { family }
            | Self::ContractTruthCollapsedOrDrifted { family }
            | Self::RowGrammarDivergedOffPrimarySurface { family }
            | Self::ComponentTruthAbsentFromCapture { family }
            | Self::ExportedProofStaleOrDivergent { family }
            | Self::AuditOrSupportTruthDropped { family }
            | Self::SurfaceFamiliesIncomplete { family }
            | Self::TruthPillarsUndeclared { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::TruthPillarCoverageIncomplete => "truth_pillars",
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / release automation / support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen trust-chronology component matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen trust-chronology component matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The certification dimensions every family is certified across.
    pub required_proof_dimensions: Vec<M5TrustReleaseProofDimension>,
    /// The truth pillars the release bundle must cover across the families.
    pub required_truth_pillars: Vec<M5TrustComponentTruthPillar>,
    /// The claimed M5 surface families every family must certify.
    pub required_surface_families: Vec<M5ShellSurfaceFamily>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<TrustReleaseProofRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Truth pillars certified across the whole bundle, in canonical (sorted) order.
    pub covered_truth_pillars: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<TrustReleaseProofWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<TrustReleaseProofCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<TrustReleaseProofFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed families.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TrustReleaseProofPacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5TrustComponentFamily) -> Option<&TrustReleaseProofRow> {
        self.rows.iter().find(|row| row.component_family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.component_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.component_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the shell automation consumes.
    pub fn dashboard(&self) -> TrustReleaseProofDashboard {
        TrustReleaseProofDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 trust-component release-proof packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per component family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,status,qualification,component_contract_truth,cross_surface_parity,support_export_proof,proof_freshness,never_drops_audit_or_support_truth,truth_pillars,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.component_contract_truth.as_str(),
                row.cross_surface_parity.as_str(),
                row.support_export_proof.as_str(),
                row.proof_freshness.as_str(),
                row.never_drops_audit_or_support_truth,
                join_tokens(&row.certified_truth_pillars, |p| p.as_str()),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 trust-component release proof\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_trust_component_release_proof`](../../crates/aureline-shell/src/m5_trust_component_release_proof/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_release_proof -- markdown > \\\n  artifacts/shell/m5-trust-component-release-proof.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!("- Green: {}\n", self.green_row_count));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification dimensions\n\n");
        for dimension in &self.required_proof_dimensions {
            out.push_str(&format!("- `{}`\n", dimension.as_str()));
        }
        out.push('\n');

        out.push_str("## Truth pillars\n\n");
        for pillar in &self.required_truth_pillars {
            out.push_str(&format!("- `{}`\n", pillar.as_str()));
        }
        out.push('\n');

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Component family | Status | Qualification | Contract truth | Cross-surface parity | Support-export | Proof freshness | No-dropped-audit-truth | Waiver |\n\
             | ---------------- | ------ | ------------- | -------------- | -------------------- | -------------- | --------------- | ---------------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.family_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.component_contract_truth.as_str(),
                row.cross_surface_parity.as_str(),
                row.support_export_proof.as_str(),
                row.proof_freshness.as_str(),
                row.never_drops_audit_or_support_truth,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&TrustReleaseProofRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, TrustReleaseProofStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed component family is certified at full standing.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.component_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact certification causes\n\n");
        if self.certification_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.certification_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.component_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.component_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_release_proof -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_trust_component_release_proof_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofDashboardRow {
    /// The governed component family.
    pub component_family: M5TrustComponentFamily,
    /// Short family label.
    pub family_label: String,
    /// Derived green/yellow/red status.
    pub status: TrustReleaseProofStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5TrustQualificationClass,
    /// Contract-truth posture.
    pub component_contract_truth: ComponentContractTruthState,
    /// Cross-surface-parity posture.
    pub cross_surface_parity: CrossSurfaceParityState,
    /// Support-export-proof posture.
    pub support_export_proof: SupportExportProofState,
    /// Proof-freshness posture.
    pub proof_freshness: ProofFreshnessState,
    /// `true` when no audit / support truth is dropped.
    pub never_drops_audit_or_support_truth: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / release automation / support export reads to
/// auto-narrow claimed component families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<TrustReleaseProofDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Shell / release automation refs that consume the dashboard.
    pub shell_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TrustReleaseProofDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &TrustReleaseProofPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| TrustReleaseProofDashboardRow {
                component_family: row.component_family,
                family_label: row.family_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                component_contract_truth: row.component_contract_truth,
                cross_surface_parity: row.cross_surface_parity,
                support_export_proof: row.support_export_proof,
                proof_freshness: row.proof_freshness,
                never_drops_audit_or_support_truth: row.never_drops_audit_or_support_truth,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .certification_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_TRUST_RELEASE_PROOF_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_TRUST_RELEASE_PROOF_SCHEMA_VERSION,
            dashboard_id: M5_TRUST_RELEASE_PROOF_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            shell_automation_refs: packet.shell_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 trust-component release-proof dashboard serializes")
    }
}

/// Support-export wrapper for the release-proof certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustReleaseProofSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: TrustReleaseProofPacket,
    /// Dashboard quoted in full.
    pub dashboard: TrustReleaseProofDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl TrustReleaseProofSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each active waiver
    /// id is quoted as a case id so a support reviewer — or the shell automation — can name the same
    /// family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: TrustReleaseProofPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.component_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_TRUST_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_TRUST_RELEASE_PROOF_SCHEMA_VERSION,
            shared_contract_ref: M5_TRUST_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_trust_component_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustReleaseProofInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen trust-chronology component matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<TrustReleaseProofRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// The truth pillars certified across the whole bundle, sorted, from the rows.
fn covered_truth_pillars(rows: &[TrustReleaseProofRow]) -> Vec<String> {
    let mut present: BTreeSet<M5TrustComponentTruthPillar> = BTreeSet::new();
    for row in rows {
        present.extend(row.certified_truth_pillars.iter().copied());
    }
    let mut tokens: Vec<String> = present.iter().map(|p| p.as_str().to_owned()).collect();
    tokens.sort();
    tokens
}

/// `true` when the union of certified truth pillars across the rows covers the whole track
/// invariant.
fn truth_pillar_coverage_complete(rows: &[TrustReleaseProofRow]) -> bool {
    let present: BTreeSet<M5TrustComponentTruthPillar> = rows
        .iter()
        .flat_map(|row| row.certified_truth_pillars.iter().copied())
        .collect();
    M5TrustComponentTruthPillar::ALL
        .iter()
        .all(|pillar| present.contains(pillar))
}

/// Builds a [`TrustReleaseProofPacket`] from the exact build identity, the frozen matrix ref, and
/// the per-family certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active waivers, and
/// the blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_trust_component_release_proof_packet(
    input: TrustReleaseProofInput,
) -> TrustReleaseProofPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<TrustReleaseProofRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<TrustReleaseProofFinding> = Vec::new();

    // Every governed family must carry a certification row.
    let present: BTreeSet<M5TrustComponentFamily> =
        rows.iter().map(|row| row.component_family).collect();
    for family in M5TrustComponentFamily::ALL {
        if !present.contains(&family) {
            blocking_findings.push(TrustReleaseProofFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }
    if !truth_pillar_coverage_complete(&rows) {
        blocking_findings.push(TrustReleaseProofFinding::TruthPillarCoverageIncomplete);
    }

    let covered_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TrustReleaseProofStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TrustReleaseProofStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TrustReleaseProofStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(TrustReleaseProofFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<TrustReleaseProofWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<TrustReleaseProofCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = TrustReleaseProofPacket {
        record_kind: M5_TRUST_RELEASE_PROOF_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_TRUST_RELEASE_PROOF_SCHEMA_VERSION,
        shared_contract_ref: M5_TRUST_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_TRUST_RELEASE_PROOF_PACKET_ID.to_owned(),
        source_schema_ref: M5_TRUST_RELEASE_PROOF_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Settings-row, capability-sheet, evidence-chronology, and chronology-export truth \
                   certified as one release-evidence bundle across every claimed M5 trust, config, \
                   activity, and support surface: each governed component family keeps its contract \
                   truth certified on every claimed surface, reads with one row grammar everywhere, \
                   reconstructs from a support export plus screenshot/golden baselines, and keeps its \
                   exported proof fresh — with each row's green/yellow/red claim auto-narrowed from \
                   its contract-truth, cross-surface-parity, support-export-proof, and proof-freshness \
                   posture so a family that drifts off the shared component contract narrows rather \
                   than keeping a stale claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_TRUST_RELEASE_PROOF_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5TrustReleaseProofDimension::ALL.to_vec(),
        required_truth_pillars: M5TrustComponentTruthPillar::ALL.to_vec(),
        required_surface_families: M5ShellSurfaceFamily::ALL.to_vec(),
        covered_truth_pillars: covered_truth_pillars(&rows),
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        certification_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.release_automation.trust_component_release_proof_registry".to_owned(),
            "release_automation.auto_narrow.trust_component_release_proof_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.trust_component_release_proof".to_owned(),
            "artifacts/release/m5-trust-component-release-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_TRUST_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-trust-component-release-proof".to_owned()],
        published_report_ref: M5_TRUST_RELEASE_PROOF_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_TRUST_RELEASE_PROOF_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_TRUST_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_TRUST_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(TrustReleaseProofFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_trust_component_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum TrustReleaseProofValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The rows do not cover all six governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The declared covered truth pillars do not match the rows.
    CoveredTruthPillarsStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required truth pillars are not the canonical set.
    RequiredTruthPillarsStale,
    /// The declared required surface families are not the canonical set.
    RequiredSurfaceFamiliesStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared certification causes do not match the recomputed causes.
    CertificationCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the release-proof certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed component family
/// carries a current certification row; each row's status is the derived auto-narrowed value, never
/// asserted; a green row cannot keep a claim while a component's contract truth collapses or drifts,
/// its row grammar diverges off the primary surface, its truth is absent from capture, its exported
/// proof is stale or divergent, it drops audit / support truth, it fails to certify every claimed M5
/// surface family, or it declares no truth pillars; the bundle's truth pillars cover the whole track
/// invariant; and a disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_trust_component_release_proof_packet(
    packet: &TrustReleaseProofPacket,
) -> Result<(), Vec<TrustReleaseProofValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(TrustReleaseProofValidationError::NoRows);
    }
    if packet.record_kind != M5_TRUST_RELEASE_PROOF_PACKET_RECORD_KIND {
        errors.push(TrustReleaseProofValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_TRUST_RELEASE_PROOF_SCHEMA_VERSION {
        errors.push(TrustReleaseProofValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(TrustReleaseProofValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(TrustReleaseProofValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5TrustReleaseProofDimension::ALL {
        errors.push(TrustReleaseProofValidationError::RequiredDimensionsStale);
    }
    if packet.required_truth_pillars != M5TrustComponentTruthPillar::ALL {
        errors.push(TrustReleaseProofValidationError::RequiredTruthPillarsStale);
    }
    if packet.required_surface_families != M5ShellSurfaceFamily::ALL {
        errors.push(TrustReleaseProofValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5TrustComponentFamily> =
        packet.rows.iter().map(|row| row.component_family).collect();
    let coverage_complete = M5TrustComponentFamily::ALL
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != M5TrustComponentFamily::ALL.len() {
        errors.push(TrustReleaseProofValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_families {
        errors.push(TrustReleaseProofValidationError::CoverageStale);
    }
    if covered_truth_pillars(&packet.rows) != packet.covered_truth_pillars {
        errors.push(TrustReleaseProofValidationError::CoveredTruthPillarsStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TrustReleaseProofStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TrustReleaseProofStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TrustReleaseProofStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(TrustReleaseProofValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<TrustReleaseProofWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(TrustReleaseProofValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<TrustReleaseProofCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(TrustReleaseProofValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<TrustReleaseProofFinding> = Vec::new();
    for family in M5TrustComponentFamily::ALL {
        if !present.contains(&family) {
            recomputed.push(TrustReleaseProofFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if !truth_pillar_coverage_complete(&packet.rows) {
        recomputed.push(TrustReleaseProofFinding::TruthPillarCoverageIncomplete);
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(TrustReleaseProofFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(TrustReleaseProofFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(TrustReleaseProofValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(TrustReleaseProofValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(TrustReleaseProofValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(TrustReleaseProofValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(TrustReleaseProofValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(TrustReleaseProofValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
