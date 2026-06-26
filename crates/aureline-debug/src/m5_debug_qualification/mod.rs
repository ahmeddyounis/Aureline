//! Debugger qualification rows, claim publications, and downgrade rules: the canonical M5
//! record that certifies every claimed debugger-facing surface against the shared debug
//! object model and its evidence corpus, and that narrows the published claim automatically
//! when debugger evidence is stale, partial, or failing.
//!
//! The earlier debug lanes materialize the *object model*: the
//! [`m5_debug_contracts`](crate::m5_debug_contracts) matrix names the ten governed debugger
//! object families and one shared vocabulary; the session-descriptor, breakpoint-spec,
//! frame/variable, evaluate/REPL, chronology/replay/notebook-parity, and dump/mapping/restore
//! lanes materialize those families as typed truth packets. This lane does *not* re-express
//! that truth. It binds each claimed M5 debugger-facing row to (a) the
//! [`DebugObjectClass`] families it claims and (b) the proof packets that keep it current,
//! computes one [`DebugQualificationStatus`] from the row's evidence freshness and
//! completeness, derives the [`DebugClaimMaturity`] the product is actually allowed to
//! publish from that status plus the disclosed support/mapping/parity truth, and republishes
//! that derived maturity to the claim board, About / help / service-health, support exports,
//! and release packets — so no debugger surface stays greener than its evidence.
//!
//! Qualification truth stays explicit and self-narrowing:
//!
//! - **One row, one computed status, one derived maturity.** Every [`DebugQualificationRow`]
//!   carries the disclosed truth dimensions (evidence freshness and completeness, support
//!   class, mapping fidelity, notebook parity, replay support, policy posture) and a stored
//!   [`status`](DebugQualificationRow::status) and
//!   [`published_maturity`](DebugQualificationRow::published_maturity). Both are *computed*
//!   by [`DebugQualificationRow::derive_status`] and
//!   [`DebugQualificationRow::derive_published_maturity`], so an inconsistent edit flips a
//!   field and fails the freeze gate rather than silently publishing a stale claim.
//! - **Stable is earned, not asserted.** A row publishes
//!   [`DebugClaimMaturity::Stable`] only when its status is
//!   [`DebugQualificationStatus::Certified`] *and* its disclosed truth is a supported
//!   backend with an exact, exact-build mapping; any aging, stale, partial, missing, or
//!   policy-blocked evidence — or any disclosed support/mapping/parity degradation in the
//!   row's own claimed scope — narrows the published maturity below stable.
//! - **Publications republish the floor.** Every [`DebugClaimPublication`] (claim board,
//!   About / help / service-health, support export, release packet) republishes the
//!   *narrowest* derived maturity across the rows it speaks for, never a wider claim, so
//!   product, docs, and release prose read one current maturity instead of duplicated
//!   green language.
//! - **Downgrade rules explain the narrowing.** Every [`DebugDowngradeRule`] names one
//!   [`DowngradeTrigger`] and the maturity it floors to; the set guarantees every row that
//!   exhibits a trigger is listed by the active rule and is narrowed at least that far, so a
//!   reviewer can read *why* a claim narrowed without re-deriving it.
//!
//! [`m5_debug_qualification_set`] is the canonical binding: it builds the set
//! deterministically and computes each [`DebugQualificationInvariant`]'s `holds` flag from
//! the built records, so the checked-in fixture and the freeze gate freeze the contract
//! byte-for-byte. The record carries no source bodies, value bodies, raw paths, provider
//! payloads, URLs, hostnames, or credentials — only opaque object refs, stable tokens, and
//! short reviewable sentences — so it is safe for support export.
//!
//! The cross-tool boundary schema is at
//! [`/schemas/debug/m5_debug_qualification.schema.json`](../../../schemas/debug/m5_debug_qualification.schema.json).
//! The checked-in stable packet is at
//! [`/fixtures/debug/m5_debug_qualification/canonical_set.json`](../../../fixtures/debug/m5_debug_qualification/canonical_set.json).
//! The reviewer-facing contract is at
//! [`/docs/debug/m5_debug_qualification.md`](../../../docs/debug/m5_debug_qualification.md).

use serde::{Deserialize, Serialize};

use crate::m5_chronology_replay_parity::{DebugSupportClass, NotebookParityClass};
use crate::m5_debug_contracts::{DebugConsumer, DebugObjectClass};
use crate::m5_dump_mapping_restore::DebugMappingFidelity;

#[cfg(test)]
mod tests;

/// Schema version for the M5 debug qualification set.
pub const M5_DEBUG_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the M5 debug qualification set.
pub const M5_DEBUG_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/debug/m5_debug_qualification.schema.json";

/// Stable record-kind tag for the qualification set.
pub const M5_DEBUG_QUALIFICATION_RECORD_KIND: &str = "m5_debug_qualification_set";

/// Stable id for the canonical qualification set.
pub const M5_DEBUG_QUALIFICATION_SET_ID: &str = "m5-debug-qualification:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical binding stays
/// deterministic and the fixture freezes byte-for-byte.
pub const M5_DEBUG_QUALIFICATION_AS_OF: &str = "2026-06-26T00:00:00Z";

/// The freeze gate that keeps the qualification set current. Stable promotion runs this
/// gate; it fails when the in-code set drifts from the checked-in fixture or any invariant
/// flips.
pub const M5_DEBUG_QUALIFICATION_FREEZE_GATE_REF: &str =
    "crates/aureline-debug/tests/m5_debug_qualification.rs";

/// The checked-in canonical qualification-set fixture.
pub const M5_DEBUG_QUALIFICATION_FIXTURE_REF: &str =
    "fixtures/debug/m5_debug_qualification/canonical_set.json";

/// The contract narrative document.
pub const M5_DEBUG_QUALIFICATION_DOC_REF: &str = "docs/debug/m5_debug_qualification.md";

/// The human-readable evidence companion artifact.
pub const M5_DEBUG_QUALIFICATION_ARTIFACT_REF: &str = "artifacts/debug/m5_debug_qualification.md";

// ---------------------------------------------------------------------------
// Vocabularies.
// ---------------------------------------------------------------------------

/// The four debugger-facing row families this lane qualifies.
///
/// The spec names notebook, profiler/replay, incident/support, and general runtime-heavy
/// rows as the claimed M5 debugger surfaces; each row carries exactly one category so the
/// qualification dashboard can group claims by the surface that renders them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugRowCategory {
    /// The core debugger UI rows: launch/attach sessions, breakpoints, frames, variables,
    /// evaluate, and console.
    CoreRuntime,
    /// The notebook debug rows: kernel bridge, frame-to-cell parity, restart consequence.
    Notebook,
    /// The profiler / trace / replay rows: replay sessions and chronology capture.
    ProfilerReplay,
    /// The incident / crash / support-export rows: symbolicated dumps and support packets.
    IncidentSupport,
}

impl DebugRowCategory {
    /// All categories, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::CoreRuntime,
        Self::Notebook,
        Self::ProfilerReplay,
        Self::IncidentSupport,
    ];

    /// Stable snake_case token for this category.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreRuntime => "core_runtime",
            Self::Notebook => "notebook",
            Self::ProfilerReplay => "profiler_replay",
            Self::IncidentSupport => "incident_support",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CoreRuntime => "Core runtime",
            Self::Notebook => "Notebook",
            Self::ProfilerReplay => "Profiler / replay",
            Self::IncidentSupport => "Incident / support",
        }
    }
}

/// How fresh a row's proof packets are relative to its freshness SLO.
///
/// Freshness is a stored, reviewable input rather than a wall-clock read, so the canonical
/// binding stays deterministic; the freeze gate freezes the freshness each row claims and the
/// derived status it produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Evidence is current and within the freshness SLO.
    Fresh,
    /// Evidence is aging toward the freshness SLO and a retest is due.
    Aging,
    /// Evidence has aged past the freshness SLO.
    Stale,
    /// No current evidence packet is available.
    Missing,
}

impl EvidenceFreshness {
    /// All freshness states, in canonical order.
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Aging, Self::Stale, Self::Missing];

    /// Stable snake_case token for this freshness state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// The qualification status computed for one claimed debugger-facing row.
///
/// Exactly one status holds for a row, derived from its evidence freshness, completeness, and
/// policy posture by [`DebugQualificationRow::derive_status`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugQualificationStatus {
    /// Current, complete evidence within the freshness SLO; the row may be published.
    Certified,
    /// Evidence is aging toward the freshness SLO; a retest is pending.
    RetestPending,
    /// Evidence has aged past the freshness SLO.
    Stale,
    /// Evidence is present but incomplete for the row's claimed scope.
    Partial,
    /// No current evidence; the claim cannot stand.
    Failing,
    /// The claim is blocked by an explicit policy rule.
    PolicyBlocked,
}

impl DebugQualificationStatus {
    /// All statuses, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::RetestPending,
        Self::Stale,
        Self::Partial,
        Self::Failing,
        Self::PolicyBlocked,
    ];

    /// Stable snake_case token for this status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::RetestPending => "retest_pending",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Failing => "failing",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Certified => "Certified",
            Self::RetestPending => "Retest pending",
            Self::Stale => "Stale",
            Self::Partial => "Partial",
            Self::Failing => "Failing",
            Self::PolicyBlocked => "Policy blocked",
        }
    }

    /// Whether this status allows the row to be published at its claimed maturity.
    pub const fn allows_publication(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Whether this status forces the published claim to narrow below its claimed maturity.
    pub const fn triggers_narrowing(self) -> bool {
        !matches!(self, Self::Certified)
    }
}

/// The maturity the product is allowed to *publish* for a row or aggregated surface.
///
/// Ranked from widest ([`Stable`](Self::Stable)) to narrowest ([`Withdrawn`](Self::Withdrawn))
/// by [`rank`](Self::rank); aggregating surfaces republish the narrowest (highest-rank)
/// maturity across the rows they speak for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugClaimMaturity {
    /// Current proof for a supported, exact-mapping claim; may be called stable.
    Stable,
    /// Visible but below stable: certified yet limited, approximate, or partial.
    Preview,
    /// May inspect metadata only; live control or replay is unavailable or policy-blocked.
    InspectOnly,
    /// Evidence is aging or stale; a retest must run before the claim is republished.
    RetestPending,
    /// The claim cannot stand and is withdrawn from the product surface.
    Withdrawn,
}

impl DebugClaimMaturity {
    /// All maturities, in canonical (widest-first) order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Preview,
        Self::InspectOnly,
        Self::RetestPending,
        Self::Withdrawn,
    ];

    /// Stable snake_case token for this maturity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::InspectOnly => "inspect_only",
            Self::RetestPending => "retest_pending",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Stable => "Stable",
            Self::Preview => "Preview",
            Self::InspectOnly => "Inspect-only",
            Self::RetestPending => "Retest pending",
            Self::Withdrawn => "Withdrawn",
        }
    }

    /// Narrowing rank: widest (`stable`) is `0`, narrowest (`withdrawn`) is `4`. A higher rank
    /// is a narrower, more conservative claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Stable => 0,
            Self::Preview => 1,
            Self::InspectOnly => 2,
            Self::RetestPending => 3,
            Self::Withdrawn => 4,
        }
    }

    /// Whether this maturity is the stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns the narrower (higher-rank) of two maturities.
    pub fn narrower(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// One reason a claim is automatically narrowed.
///
/// Each [`DebugQualificationRow`] computes the set of triggers it exhibits; each active
/// [`DebugDowngradeRule`] names exactly one trigger and the maturity it floors to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTrigger {
    /// Evidence is aging toward its freshness SLO.
    EvidenceAging,
    /// Evidence has aged past its freshness SLO.
    EvidenceStale,
    /// Evidence is incomplete for the row's claimed scope.
    EvidencePartial,
    /// No current evidence is available.
    EvidenceMissing,
    /// The row's own backend support class is below `supported`.
    SupportClassDegraded,
    /// The row's mapping fidelity does not preserve an exact, exact-build source mapping.
    MappingFidelityDegraded,
    /// A notebook row's frame-to-cell parity is divergent or unsupported.
    NotebookParityLost,
    /// A replay-claiming row's replay support is below `supported`.
    ReplayEvidenceLost,
    /// The claim is blocked by an explicit policy rule.
    PolicyBlocked,
}

impl DowngradeTrigger {
    /// All triggers, in canonical order.
    pub const ALL: [Self; 9] = [
        Self::EvidenceAging,
        Self::EvidenceStale,
        Self::EvidencePartial,
        Self::EvidenceMissing,
        Self::SupportClassDegraded,
        Self::MappingFidelityDegraded,
        Self::NotebookParityLost,
        Self::ReplayEvidenceLost,
        Self::PolicyBlocked,
    ];

    /// Stable snake_case token for this trigger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceAging => "evidence_aging",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidencePartial => "evidence_partial",
            Self::EvidenceMissing => "evidence_missing",
            Self::SupportClassDegraded => "support_class_degraded",
            Self::MappingFidelityDegraded => "mapping_fidelity_degraded",
            Self::NotebookParityLost => "notebook_parity_lost",
            Self::ReplayEvidenceLost => "replay_evidence_lost",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The channels that republish debugger qualification truth instead of restating it.
///
/// These are the four surfaces the spec names: the claim publication board, About / help /
/// service-health, support exports, and release packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPublicationChannel {
    /// The product claim publication board.
    ClaimBoard,
    /// About, in-product help, and service-health surfaces.
    AboutHelpServiceHealth,
    /// Support bundle / export packets.
    SupportExport,
    /// Release evidence packets.
    ReleasePacket,
}

impl ClaimPublicationChannel {
    /// All channels, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ClaimBoard,
        Self::AboutHelpServiceHealth,
        Self::SupportExport,
        Self::ReleasePacket,
    ];

    /// Stable snake_case token for this channel.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimBoard => "claim_board",
            Self::AboutHelpServiceHealth => "about_help_service_health",
            Self::SupportExport => "support_export",
            Self::ReleasePacket => "release_packet",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ClaimBoard => "Claim board",
            Self::AboutHelpServiceHealth => "About / help / service-health",
            Self::SupportExport => "Support export",
            Self::ReleasePacket => "Release packet",
        }
    }
}

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// One claimed M5 debugger-facing row, qualified against the shared debug object model.
///
/// The disclosed truth dimensions are the inputs; [`status`](Self::status),
/// [`published_maturity`](Self::published_maturity), [`narrowed`](Self::narrowed), and
/// [`narrowing_reason`](Self::narrowing_reason) are computed from them and frozen, so an
/// inconsistent edit flips a field and fails [`DebugQualificationSet::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugQualificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Human-readable title.
    pub title: String,
    /// The claimed M5 row reference this qualifies.
    pub m5_row_ref: String,
    /// The debugger surface family this row belongs to.
    pub category: DebugRowCategory,
    /// The debugger object families this row claims, tying it to the shared contract.
    pub claimed_object_classes: Vec<DebugObjectClass>,
    /// The maturity the row historically claimed before qualification.
    pub claimed_maturity: DebugClaimMaturity,
    /// How fresh the row's proof packets are.
    pub evidence_freshness: EvidenceFreshness,
    /// Whether the evidence covers the row's full claimed scope.
    pub evidence_complete: bool,
    /// The row's own backend support class for its claimed capability.
    pub support_class: DebugSupportClass,
    /// The row's mapping fidelity.
    pub mapping_fidelity: DebugMappingFidelity,
    /// The notebook frame-to-cell parity disclosed for this row (`not_applicable` off
    /// notebook surfaces).
    pub notebook_parity: NotebookParityClass,
    /// The replay support disclosed for this row (`supported` when no replay is claimed).
    pub replay_support: DebugSupportClass,
    /// Whether the claim is blocked by an explicit policy rule.
    pub policy_blocked: bool,
    /// The freshness SLO, in days, the evidence is measured against.
    pub freshness_slo_days: u32,
    /// The timestamp the row was last certified.
    pub last_certified_at: String,
    /// The computed qualification status.
    pub status: DebugQualificationStatus,
    /// The computed maturity the product is allowed to publish.
    pub published_maturity: DebugClaimMaturity,
    /// Whether the published maturity is narrower than the claimed maturity.
    pub narrowed: bool,
    /// One reviewable sentence explaining the narrowing, empty when not narrowed.
    pub narrowing_reason: String,
    /// The proof packets that keep this row current.
    pub evidence_refs: Vec<String>,
    /// One reviewable sentence summarizing the row.
    pub summary: String,
}

impl DebugQualificationRow {
    /// Whether the row claims a replay or chronology capability, so replay support is in scope.
    pub fn claims_replay(&self) -> bool {
        self.claimed_object_classes.iter().any(|c| {
            matches!(
                c,
                DebugObjectClass::ReplaySession | DebugObjectClass::ChronologyCapability
            )
        })
    }

    /// Whether the row claims notebook-debug parity, so notebook parity is in scope.
    pub fn claims_notebook(&self) -> bool {
        self.category == DebugRowCategory::Notebook
            || self
                .claimed_object_classes
                .contains(&DebugObjectClass::NotebookDebugParity)
    }

    /// Computes the qualification status from evidence freshness, completeness, and policy.
    ///
    /// Precedence, widest narrowing last: policy-blocked, missing (failing), stale, partial,
    /// aging (retest-pending), otherwise certified.
    pub fn derive_status(&self) -> DebugQualificationStatus {
        if self.policy_blocked {
            return DebugQualificationStatus::PolicyBlocked;
        }
        match self.evidence_freshness {
            EvidenceFreshness::Missing => return DebugQualificationStatus::Failing,
            EvidenceFreshness::Stale => return DebugQualificationStatus::Stale,
            EvidenceFreshness::Aging | EvidenceFreshness::Fresh => {}
        }
        if !self.evidence_complete {
            return DebugQualificationStatus::Partial;
        }
        if self.evidence_freshness == EvidenceFreshness::Aging {
            return DebugQualificationStatus::RetestPending;
        }
        DebugQualificationStatus::Certified
    }

    /// The maturity floor a *certified* row earns from its disclosed support/mapping/parity
    /// truth within its own claimed scope.
    fn certified_maturity_floor(&self) -> DebugClaimMaturity {
        let mut floor = DebugClaimMaturity::Stable;
        floor = floor.narrower(match self.support_class {
            DebugSupportClass::Supported => DebugClaimMaturity::Stable,
            DebugSupportClass::Limited => DebugClaimMaturity::Preview,
            DebugSupportClass::PolicyBlocked => DebugClaimMaturity::Preview,
            DebugSupportClass::Unavailable => DebugClaimMaturity::InspectOnly,
        });
        if !self.mapping_fidelity.preserves_exact_source() {
            floor = floor.narrower(DebugClaimMaturity::Preview);
        }
        if self.claims_notebook() {
            floor = floor.narrower(match self.notebook_parity {
                NotebookParityClass::Mirrored | NotebookParityClass::NotApplicable => {
                    DebugClaimMaturity::Stable
                }
                NotebookParityClass::Divergent => DebugClaimMaturity::Preview,
                NotebookParityClass::Unsupported => DebugClaimMaturity::InspectOnly,
            });
        }
        if self.claims_replay() {
            floor = floor.narrower(match self.replay_support {
                DebugSupportClass::Supported => DebugClaimMaturity::Stable,
                DebugSupportClass::Limited | DebugSupportClass::PolicyBlocked => {
                    DebugClaimMaturity::Preview
                }
                DebugSupportClass::Unavailable => DebugClaimMaturity::InspectOnly,
            });
        }
        floor
    }

    /// Computes the maturity the product is allowed to publish for this row.
    ///
    /// A degraded status forces a narrowed maturity outright; a certified status is tempered
    /// by the disclosed support/mapping/parity truth so an honest-but-limited row never
    /// publishes stable.
    pub fn derive_published_maturity(&self) -> DebugClaimMaturity {
        match self.derive_status() {
            DebugQualificationStatus::PolicyBlocked | DebugQualificationStatus::Failing => {
                DebugClaimMaturity::Withdrawn
            }
            DebugQualificationStatus::Stale => DebugClaimMaturity::RetestPending,
            DebugQualificationStatus::RetestPending => DebugClaimMaturity::RetestPending,
            DebugQualificationStatus::Partial => DebugClaimMaturity::Preview,
            DebugQualificationStatus::Certified => self.certified_maturity_floor(),
        }
    }

    /// Whether the published maturity is strictly narrower than the claimed maturity.
    pub fn derive_narrowed(&self) -> bool {
        self.derive_published_maturity().rank() > self.claimed_maturity.rank()
    }

    /// The set of downgrade triggers this row exhibits, in canonical trigger order.
    pub fn degradations(&self) -> Vec<DowngradeTrigger> {
        let mut out = Vec::new();
        match self.evidence_freshness {
            EvidenceFreshness::Aging => out.push(DowngradeTrigger::EvidenceAging),
            EvidenceFreshness::Stale => out.push(DowngradeTrigger::EvidenceStale),
            EvidenceFreshness::Missing => out.push(DowngradeTrigger::EvidenceMissing),
            EvidenceFreshness::Fresh => {}
        }
        if !self.evidence_complete {
            out.push(DowngradeTrigger::EvidencePartial);
        }
        if self.support_class != DebugSupportClass::Supported {
            out.push(DowngradeTrigger::SupportClassDegraded);
        }
        if !self.mapping_fidelity.preserves_exact_source() {
            out.push(DowngradeTrigger::MappingFidelityDegraded);
        }
        if self.claims_notebook()
            && matches!(
                self.notebook_parity,
                NotebookParityClass::Divergent | NotebookParityClass::Unsupported
            )
        {
            out.push(DowngradeTrigger::NotebookParityLost);
        }
        if self.claims_replay() && self.replay_support != DebugSupportClass::Supported {
            out.push(DowngradeTrigger::ReplayEvidenceLost);
        }
        if self.policy_blocked {
            out.push(DowngradeTrigger::PolicyBlocked);
        }
        out.sort();
        out.dedup();
        out
    }

    /// Computes the narrowing reason: empty when not narrowed, otherwise a reviewable
    /// sentence citing the dominant cause.
    pub fn derive_narrowing_reason(&self) -> String {
        if !self.derive_narrowed() {
            return String::new();
        }
        match self.derive_status() {
            DebugQualificationStatus::PolicyBlocked => {
                "Claim withdrawn: this debugger surface is blocked by policy.".to_owned()
            }
            DebugQualificationStatus::Failing => {
                "Claim withdrawn: required debugger evidence is missing.".to_owned()
            }
            DebugQualificationStatus::Stale => {
                "Claim narrowed to retest-pending: debugger evidence aged past its freshness SLO."
                    .to_owned()
            }
            DebugQualificationStatus::RetestPending => {
                "Claim narrowed to retest-pending: debugger evidence is aging toward its \
                 freshness SLO."
                    .to_owned()
            }
            DebugQualificationStatus::Partial => {
                "Claim narrowed to preview: debugger evidence is partial for the claimed scope."
                    .to_owned()
            }
            DebugQualificationStatus::Certified => {
                "Claim narrowed: disclosed support or mapping truth is below the claimed maturity."
                    .to_owned()
            }
        }
    }

    /// Whether the stored computed fields agree with the derivations from the inputs.
    pub fn flags_consistent(&self) -> bool {
        self.status == self.derive_status()
            && self.published_maturity == self.derive_published_maturity()
            && self.narrowed == self.derive_narrowed()
            && self.narrowing_reason == self.derive_narrowing_reason()
    }
}

/// One claim publication that republishes the floor of the rows it speaks for to one channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugClaimPublication {
    /// Stable publication id.
    pub publication_id: String,
    /// Human-readable title.
    pub title: String,
    /// The channel this publication targets.
    pub channel: ClaimPublicationChannel,
    /// The qualification rows this publication republishes.
    pub row_refs: Vec<String>,
    /// The maturity the channel historically claimed.
    pub claimed_maturity: DebugClaimMaturity,
    /// The computed maturity republished (the floor of the referenced rows, never wider than
    /// the claimed maturity).
    pub published_maturity: DebugClaimMaturity,
    /// Whether the published maturity is narrower than the claimed maturity.
    pub narrowed: bool,
    /// One reviewable sentence explaining the narrowing, empty when not narrowed.
    pub narrowing_reason: String,
    /// Whether the channel shows each row's qualification status.
    pub shows_status: bool,
    /// Whether the channel shows the evidence refs backing the claim.
    pub shows_evidence_refs: bool,
    /// One reviewable sentence summarizing the publication.
    pub summary: String,
}

impl DebugClaimPublication {
    /// Computes the published maturity: the narrowest maturity across the referenced rows.
    /// An empty reference set republishes the claimed maturity unchanged.
    pub fn derive_published_maturity(&self, rows: &[DebugQualificationRow]) -> DebugClaimMaturity {
        let mut floor = self.claimed_maturity;
        for row_id in &self.row_refs {
            if let Some(row) = rows.iter().find(|r| &r.row_id == row_id) {
                floor = floor.narrower(row.published_maturity);
            }
        }
        floor
    }

    /// Whether the published maturity is strictly narrower than the claimed maturity.
    pub fn derive_narrowed(&self, rows: &[DebugQualificationRow]) -> bool {
        self.derive_published_maturity(rows).rank() > self.claimed_maturity.rank()
    }

    /// Computes the narrowing reason: empty when not narrowed, otherwise a reviewable sentence.
    pub fn derive_narrowing_reason(&self, rows: &[DebugQualificationRow]) -> String {
        if !self.derive_narrowed(rows) {
            return String::new();
        }
        let published = self.derive_published_maturity(rows);
        format!(
            "{} narrowed to {}: republishing the narrowest qualified debugger row it covers.",
            self.channel.label(),
            published.label()
        )
    }

    /// Whether the stored computed fields agree with the derivations from the rows.
    pub fn flags_consistent(&self, rows: &[DebugQualificationRow]) -> bool {
        self.published_maturity == self.derive_published_maturity(rows)
            && self.narrowed == self.derive_narrowed(rows)
            && self.narrowing_reason == self.derive_narrowing_reason(rows)
    }
}

/// One downgrade rule: the declarative policy that narrows every row exhibiting a trigger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugDowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The single trigger that activates this rule.
    pub trigger: DowngradeTrigger,
    /// The maturity a triggered row is floored to (at minimum).
    pub resulting_maturity: DebugClaimMaturity,
    /// The rows this rule currently floors.
    pub affected_row_refs: Vec<String>,
    /// Whether the rule is active.
    pub active: bool,
    /// Whether the rule is shown on certification surfaces.
    pub shows_rule: bool,
    /// One reviewable sentence summarizing the rule.
    pub summary: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugQualificationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen, typed M5 debug qualification set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugQualificationSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_debug_qualification_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the set current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The boundary schemas this set binds as truth sources.
    pub source_schema_refs: Vec<String>,
    /// The crate modules that already produce the consumed truth.
    pub producer_refs: Vec<String>,
    /// The surfaces that consume the qualification rows and publications.
    pub consumer_surfaces: Vec<DebugConsumer>,
    /// The qualification rows.
    pub qualification_rows: Vec<DebugQualificationRow>,
    /// The claim publications.
    pub claim_publications: Vec<DebugClaimPublication>,
    /// The downgrade rules.
    pub downgrade_rules: Vec<DebugDowngradeRule>,
    /// The computed invariants.
    pub invariants: Vec<DebugQualificationInvariant>,
    /// Whether raw payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the qualification set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugQualificationSetValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for DebugQualificationSetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m5 debug qualification set invalid: {}", self.reason)
    }
}

impl std::error::Error for DebugQualificationSetValidationError {}

impl DebugQualificationSet {
    /// Returns the qualification row with the given id, if present.
    pub fn row(&self, row_id: &str) -> Option<&DebugQualificationRow> {
        self.qualification_rows.iter().find(|r| r.row_id == row_id)
    }

    /// Returns the publication for the given channel, if present.
    pub fn publication_for_channel(
        &self,
        channel: ClaimPublicationChannel,
    ) -> Option<&DebugClaimPublication> {
        self.claim_publications
            .iter()
            .find(|p| p.channel == channel)
    }

    /// Returns the first qualification row in the given category, if present.
    pub fn row_in_category(&self, category: DebugRowCategory) -> Option<&DebugQualificationRow> {
        self.qualification_rows
            .iter()
            .find(|r| r.category == category)
    }

    /// Returns the first qualification row carrying the given status, if present.
    pub fn row_with_status(
        &self,
        status: DebugQualificationStatus,
    ) -> Option<&DebugQualificationRow> {
        self.qualification_rows.iter().find(|r| r.status == status)
    }

    /// Whether every object class is claimed by at least one row.
    pub fn covers_object_class(&self, class: DebugObjectClass) -> bool {
        self.qualification_rows
            .iter()
            .any(|r| r.claimed_object_classes.contains(&class))
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are excluded and
    /// every ref is a repo-relative object ref, never a URL, host, credential, or absolute
    /// path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_set = self
            .source_schema_refs
            .iter()
            .map(String::as_str)
            .chain(self.producer_refs.iter().map(String::as_str))
            .chain(std::iter::once(self.freeze_gate_ref.as_str()));
        let from_rows = self
            .qualification_rows
            .iter()
            .flat_map(|r| r.evidence_refs.iter().map(String::as_str));
        from_set.chain(from_rows)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    ///
    /// # Errors
    ///
    /// Returns a [`DebugQualificationSetValidationError`] when an identifier, a ref, a
    /// computed flag, a cross-reference, a downgrade rule, or an invariant is inconsistent.
    pub fn validate(&self) -> Result<(), DebugQualificationSetValidationError> {
        let fail = |reason: String| Err(DebugQualificationSetValidationError { reason });

        if self.record_kind != M5_DEBUG_QUALIFICATION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_DEBUG_QUALIFICATION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.m5_debug_qualification_schema_version != M5_DEBUG_QUALIFICATION_SCHEMA_VERSION {
            return fail("unexpected schema version".to_owned());
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.qualification_rows.is_empty() {
            return fail("no qualification rows".to_owned());
        }
        if self.claim_publications.is_empty() {
            return fail("no claim publications".to_owned());
        }
        if self.downgrade_rules.is_empty() {
            return fail("no downgrade rules".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.qualification_rows.iter().map(|r| r.row_id.as_str())) {
            return fail("row ids are not unique".to_owned());
        }
        if !all_unique(
            self.claim_publications
                .iter()
                .map(|p| p.publication_id.as_str()),
        ) {
            return fail("publication ids are not unique".to_owned());
        }
        if !all_unique(self.downgrade_rules.iter().map(|r| r.rule_id.as_str())) {
            return fail("rule ids are not unique".to_owned());
        }

        // Per-row consistency.
        for r in &self.qualification_rows {
            if r.row_id.is_empty()
                || r.title.is_empty()
                || r.m5_row_ref.is_empty()
                || r.last_certified_at.is_empty()
                || r.summary.is_empty()
            {
                return fail(format!("row {} has an empty required field", r.row_id));
            }
            if r.claimed_object_classes.is_empty() {
                return fail(format!("row {} claims no object class", r.row_id));
            }
            if r.evidence_refs.is_empty() {
                return fail(format!("row {} cites no evidence", r.row_id));
            }
            if r.freshness_slo_days == 0 {
                return fail(format!("row {} has a zero freshness SLO", r.row_id));
            }
            if !r.flags_consistent() {
                return fail(format!(
                    "row {} computed status/maturity/narrowing disagrees with its evidence",
                    r.row_id
                ));
            }
            // Stable is earned, never asserted.
            if r.published_maturity.is_stable()
                && (r.status != DebugQualificationStatus::Certified
                    || r.support_class != DebugSupportClass::Supported
                    || !r.mapping_fidelity.preserves_exact_source())
            {
                return fail(format!(
                    "row {} publishes stable without certified, supported, exact-mapping evidence",
                    r.row_id
                ));
            }
            // A narrowed row must carry a reason; an un-narrowed row must not.
            if r.narrowed == r.narrowing_reason.is_empty() {
                return fail(format!(
                    "row {} narrowing reason disagrees with its narrowed flag",
                    r.row_id
                ));
            }
        }

        // Per-publication consistency.
        for p in &self.claim_publications {
            if p.publication_id.is_empty() || p.title.is_empty() || p.summary.is_empty() {
                return fail(format!(
                    "publication {} has an empty required field",
                    p.publication_id
                ));
            }
            if p.row_refs.is_empty() {
                return fail(format!(
                    "publication {} republishes no rows",
                    p.publication_id
                ));
            }
            for row_ref in &p.row_refs {
                if self.row(row_ref).is_none() {
                    return fail(format!(
                        "publication {} references unknown row {}",
                        p.publication_id, row_ref
                    ));
                }
            }
            if !p.shows_status || !p.shows_evidence_refs {
                return fail(format!(
                    "publication {} must show qualification status and evidence refs",
                    p.publication_id
                ));
            }
            if !p.flags_consistent(&self.qualification_rows) {
                return fail(format!(
                    "publication {} computed maturity/narrowing disagrees with its rows",
                    p.publication_id
                ));
            }
        }

        // Every channel is materialized exactly once.
        for channel in ClaimPublicationChannel::ALL {
            let count = self
                .claim_publications
                .iter()
                .filter(|p| p.channel == channel)
                .count();
            if count != 1 {
                return fail(format!(
                    "channel {} must be published exactly once, found {}",
                    channel.as_str(),
                    count
                ));
            }
        }

        // Per-rule consistency and the trigger cross-check.
        for rule in &self.downgrade_rules {
            if rule.rule_id.is_empty() || rule.title.is_empty() || rule.summary.is_empty() {
                return fail(format!("rule {} has an empty required field", rule.rule_id));
            }
            if rule.active && !rule.shows_rule {
                return fail(format!("active rule {} must be visible", rule.rule_id));
            }
            for row_ref in &rule.affected_row_refs {
                match self.row(row_ref) {
                    None => {
                        return fail(format!(
                            "rule {} affects unknown row {}",
                            rule.rule_id, row_ref
                        ))
                    }
                    Some(row) => {
                        if !row.degradations().contains(&rule.trigger) {
                            return fail(format!(
                                "rule {} lists row {} which does not exhibit trigger {}",
                                rule.rule_id,
                                row_ref,
                                rule.trigger.as_str()
                            ));
                        }
                        if row.published_maturity.rank() < rule.resulting_maturity.rank() {
                            return fail(format!(
                                "rule {} mandates {} but row {} only published {}",
                                rule.rule_id,
                                rule.resulting_maturity.as_str(),
                                row_ref,
                                row.published_maturity.as_str()
                            ));
                        }
                    }
                }
            }
            // An active rule must list every row that exhibits its trigger.
            if rule.active {
                for row in &self.qualification_rows {
                    if row.degradations().contains(&rule.trigger)
                        && !rule.affected_row_refs.contains(&row.row_id)
                    {
                        return fail(format!(
                            "active rule {} omits row {} which exhibits trigger {}",
                            rule.rule_id,
                            row.row_id,
                            rule.trigger.as_str()
                        ));
                    }
                }
            }
        }

        // Every object class is covered.
        for class in DebugObjectClass::ALL {
            if !self.covers_object_class(class) {
                return fail(format!("object class {} is not covered", class.as_str()));
            }
        }

        // Every invariant recomputes to its stored value and holds.
        let recomputed = compute_invariants(
            &self.qualification_rows,
            &self.claim_publications,
            &self.downgrade_rules,
        );
        if recomputed != self.invariants {
            return fail("invariants drifted from their computed values".to_owned());
        }
        for i in &self.invariants {
            if !i.holds {
                return fail(format!("invariant {} does not hold", i.invariant_id));
            }
        }

        if !self.is_support_export_safe() {
            return fail("set is not support-export safe".to_owned());
        }

        Ok(())
    }
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque `aureline://`
/// handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds one qualification row, computing its status/maturity/narrowing from the inputs.
#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    title: &str,
    m5_row_ref: &str,
    category: DebugRowCategory,
    claimed_object_classes: &[DebugObjectClass],
    claimed_maturity: DebugClaimMaturity,
    evidence_freshness: EvidenceFreshness,
    evidence_complete: bool,
    support_class: DebugSupportClass,
    mapping_fidelity: DebugMappingFidelity,
    notebook_parity: NotebookParityClass,
    replay_support: DebugSupportClass,
    policy_blocked: bool,
    last_certified_at: &str,
    evidence_refs: &[&str],
    summary: &str,
) -> DebugQualificationRow {
    let mut r = DebugQualificationRow {
        row_id: row_id.to_owned(),
        title: title.to_owned(),
        m5_row_ref: m5_row_ref.to_owned(),
        category,
        claimed_object_classes: claimed_object_classes.to_vec(),
        claimed_maturity,
        evidence_freshness,
        evidence_complete,
        support_class,
        mapping_fidelity,
        notebook_parity,
        replay_support,
        policy_blocked,
        freshness_slo_days: 30,
        last_certified_at: last_certified_at.to_owned(),
        // Filled below from the derivations.
        status: DebugQualificationStatus::Certified,
        published_maturity: DebugClaimMaturity::Stable,
        narrowed: false,
        narrowing_reason: String::new(),
        evidence_refs: strvec(evidence_refs),
        summary: summary.to_owned(),
    };
    r.status = r.derive_status();
    r.published_maturity = r.derive_published_maturity();
    r.narrowed = r.derive_narrowed();
    r.narrowing_reason = r.derive_narrowing_reason();
    r
}

fn build_rows() -> Vec<DebugQualificationRow> {
    use DebugClaimMaturity::*;
    use DebugMappingFidelity as Map;
    use DebugObjectClass as Obj;
    use DebugRowCategory as Cat;
    use DebugSupportClass as Sup;
    use EvidenceFreshness as Fresh;
    use NotebookParityClass as Par;

    vec![
        row(
            "debug.qual:core_session_attach:0001",
            "Launch / attach session and target descriptors",
            "m5_debug_session",
            Cat::CoreRuntime,
            &[Obj::DebugSession, Obj::AttachTarget],
            Stable,
            Fresh::Fresh,
            true,
            Sup::Supported,
            Map::Exact,
            Par::NotApplicable,
            Sup::Supported,
            false,
            "2026-06-25",
            &[
                "fixtures/debug/m5_debug_session_descriptors/canonical_set.json",
                "fixtures/debug/m5_debug_contracts/canonical_matrix.json",
            ],
            "Launch, attach, core-file, replay, and inspect-only session modes stay distinct \
             with fresh descriptor evidence; published stable.",
        ),
        row(
            "debug.qual:core_breakpoints_frames:0002",
            "Breakpoint specs and frame mapping",
            "m5_debug_breakpoints",
            Cat::CoreRuntime,
            &[Obj::BreakpointSpec, Obj::FrameMapping],
            Stable,
            Fresh::Fresh,
            true,
            Sup::Supported,
            Map::Exact,
            Par::NotApplicable,
            Sup::Supported,
            false,
            "2026-06-25",
            &[
                "fixtures/debug/m5_breakpoint_specs/canonical_set.json",
                "fixtures/debug/m5_frame_variable_snapshots/canonical_set.json",
            ],
            "Verified breakpoints and exact frame mappings carry current spec evidence; \
             published stable.",
        ),
        row(
            "debug.qual:core_variables_evaluate:0003",
            "Variables, watches, evaluate, and console",
            "m5_debug_variables_evaluate",
            Cat::CoreRuntime,
            &[
                Obj::VariableWatchSnapshot,
                Obj::EvaluateRequestResult,
                Obj::ConsoleEmission,
            ],
            Stable,
            Fresh::Aging,
            true,
            Sup::Supported,
            Map::Exact,
            Par::NotApplicable,
            Sup::Supported,
            false,
            "2026-05-20",
            &[
                "fixtures/debug/m5_frame_variable_snapshots/canonical_set.json",
                "fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json",
            ],
            "Variable/watch and evaluate/console evidence is aging toward its SLO; the stable \
             claim narrows to retest-pending until a fresh capture lands.",
        ),
        row(
            "debug.qual:notebook_debug_bridge:0004",
            "Notebook debugger bridge and frame-to-cell parity",
            "m5_notebook_debug_bridge",
            Cat::Notebook,
            &[Obj::NotebookDebugParity, Obj::BreakpointSpec],
            Preview,
            Fresh::Fresh,
            true,
            Sup::Limited,
            Map::Exact,
            Par::Divergent,
            Sup::Supported,
            false,
            "2026-06-24",
            &[
                "artifacts/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.json",
                "fixtures/debug/m5_chronology_replay_parity/canonical_set.json",
            ],
            "The notebook bridge offers a disclosed-divergent subset of debug parity; \
             published preview honestly, not narrowed.",
        ),
        row(
            "debug.qual:notebook_unsupported_kernel:0005",
            "Notebook debug support states on unsupported kernels",
            "m5_notebook_debug_support_states",
            Cat::Notebook,
            &[Obj::NotebookDebugParity, Obj::ChronologyCapability],
            Preview,
            Fresh::Stale,
            true,
            Sup::Unavailable,
            Map::SymbolOnly,
            Par::Unsupported,
            Sup::Unavailable,
            false,
            "2026-04-15",
            &[
                "artifacts/notebook/m5/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues.json",
                "fixtures/debug/m5_chronology_replay_parity/canonical_set.json",
            ],
            "Unsupported-kernel debug-state evidence is stale; the preview claim narrows to \
             retest-pending and discloses no notebook parity or chronology.",
        ),
        row(
            "debug.qual:profiler_replay_session:0006",
            "Profiler replay session and chronology capture",
            "m5_profiler_replay",
            Cat::ProfilerReplay,
            &[
                Obj::ReplaySession,
                Obj::ChronologyCapability,
                Obj::FrameMapping,
            ],
            Preview,
            Fresh::Fresh,
            true,
            Sup::Limited,
            Map::Approximate,
            Par::NotApplicable,
            Sup::Limited,
            false,
            "2026-06-22",
            &[
                "artifacts/perf/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.json",
                "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json",
            ],
            "Replay sessions are inspect-only with limited reverse-step support and \
             approximate frame mapping; published preview honestly, not narrowed.",
        ),
        row(
            "debug.qual:profiler_replay_imported:0007",
            "Imported replay capture with build mismatch",
            "m5_profiler_imported_replay",
            Cat::ProfilerReplay,
            &[Obj::ReplaySession, Obj::FrameMapping],
            Stable,
            Fresh::Fresh,
            false,
            Sup::Limited,
            Map::MismatchedBuild,
            Par::NotApplicable,
            Sup::Limited,
            false,
            "2026-06-21",
            &[
                "artifacts/perf/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.json",
                "fixtures/debug/m5_dump_mapping_restore/canonical_set.json",
            ],
            "An imported replay capture maps against a mismatched build with partial evidence; \
             the stable claim narrows to preview.",
        ),
        row(
            "debug.qual:incident_crash_symbolication:0008",
            "Incident crash dump symbolication",
            "m5_incident_crash_symbolication",
            Cat::IncidentSupport,
            &[Obj::FrameMapping, Obj::DebugSession],
            Preview,
            Fresh::Missing,
            false,
            Sup::Unavailable,
            Map::Unresolved,
            Par::NotApplicable,
            Sup::Supported,
            false,
            "2026-03-30",
            &[
                "artifacts/support/crash_artifact_retention_seed.json",
                "fixtures/debug/symbolication/unresolved_mismatch_report.json",
            ],
            "No current symbolication evidence is available and mapping is unresolved; the \
             claim is withdrawn until a resolved report lands.",
        ),
        row(
            "debug.qual:incident_support_export:0009",
            "Support-export of variables, console, and session",
            "m5_incident_support_export",
            Cat::IncidentSupport,
            &[
                Obj::VariableWatchSnapshot,
                Obj::ConsoleEmission,
                Obj::DebugSession,
            ],
            Stable,
            Fresh::Fresh,
            true,
            Sup::PolicyBlocked,
            Map::Exact,
            Par::NotApplicable,
            Sup::Supported,
            true,
            "2026-06-23",
            &[
                "artifacts/tooling/m5-problems-output-evidence-certification/support_export.json",
                "fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json",
            ],
            "Support export of debugger state is blocked by policy; the claim is withdrawn \
             from the product surface.",
        ),
        row(
            "debug.qual:profiler_replay_inspect_only:0010",
            "Inspect-only replay of an unavailable-control capture",
            "m5_profiler_inspect_only_replay",
            Cat::ProfilerReplay,
            &[Obj::ReplaySession, Obj::VariableWatchSnapshot],
            InspectOnly,
            Fresh::Fresh,
            true,
            Sup::Unavailable,
            Map::Exact,
            Par::NotApplicable,
            Sup::Unavailable,
            false,
            "2026-06-20",
            &[
                "fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json",
                "fixtures/debug/m5_chronology_replay_parity/canonical_set.json",
            ],
            "Live control and replay verbs are unavailable for this capture; published \
             inspect-only honestly, not narrowed.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn publication(
    publication_id: &str,
    title: &str,
    channel: ClaimPublicationChannel,
    row_refs: &[&str],
    claimed_maturity: DebugClaimMaturity,
    rows: &[DebugQualificationRow],
    shows_status: bool,
    shows_evidence_refs: bool,
    summary: &str,
) -> DebugClaimPublication {
    let mut p = DebugClaimPublication {
        publication_id: publication_id.to_owned(),
        title: title.to_owned(),
        channel,
        row_refs: strvec(row_refs),
        claimed_maturity,
        published_maturity: claimed_maturity,
        narrowed: false,
        narrowing_reason: String::new(),
        shows_status,
        shows_evidence_refs,
        summary: summary.to_owned(),
    };
    p.published_maturity = p.derive_published_maturity(rows);
    p.narrowed = p.derive_narrowed(rows);
    p.narrowing_reason = p.derive_narrowing_reason(rows);
    p
}

fn build_publications(rows: &[DebugQualificationRow]) -> Vec<DebugClaimPublication> {
    use ClaimPublicationChannel::*;
    use DebugClaimMaturity::*;

    vec![
        publication(
            "debug.pub:claim_board:0001",
            "Core debugger claim board",
            ClaimBoard,
            &[
                "debug.qual:core_session_attach:0001",
                "debug.qual:core_breakpoints_frames:0002",
                "debug.qual:core_variables_evaluate:0003",
            ],
            Stable,
            rows,
            true,
            true,
            "The claim board republishes the narrowest core-debugger row; aging \
             variable/evaluate evidence narrows the board to retest-pending.",
        ),
        publication(
            "debug.pub:about_help:0002",
            "About / help / service-health debugger claim",
            AboutHelpServiceHealth,
            &[
                "debug.qual:notebook_debug_bridge:0004",
                "debug.qual:notebook_unsupported_kernel:0005",
                "debug.qual:profiler_replay_session:0006",
                "debug.qual:profiler_replay_inspect_only:0010",
            ],
            Preview,
            rows,
            true,
            true,
            "About, help, and service-health republish the narrowest notebook/replay row; \
             stale unsupported-kernel evidence narrows the surface to retest-pending.",
        ),
        publication(
            "debug.pub:support_export:0003",
            "Support-export debugger claim",
            SupportExport,
            &[
                "debug.qual:profiler_replay_imported:0007",
                "debug.qual:incident_crash_symbolication:0008",
                "debug.qual:incident_support_export:0009",
            ],
            Preview,
            rows,
            true,
            true,
            "Support exports republish the narrowest incident/support row; a withdrawn \
             symbolication or policy-blocked export narrows the surface to withdrawn.",
        ),
        publication(
            "debug.pub:release_packet:0004",
            "Release packet debugger claim",
            ReleasePacket,
            &[
                "debug.qual:core_session_attach:0001",
                "debug.qual:core_breakpoints_frames:0002",
            ],
            Stable,
            rows,
            true,
            true,
            "The release packet republishes only the ship-required core rows; both hold \
             stable so the packet stays stable.",
        ),
    ]
}

fn downgrade_rule(
    rule_id: &str,
    title: &str,
    trigger: DowngradeTrigger,
    resulting_maturity: DebugClaimMaturity,
    rows: &[DebugQualificationRow],
    summary: &str,
) -> DebugDowngradeRule {
    let affected_row_refs: Vec<String> = rows
        .iter()
        .filter(|r| r.degradations().contains(&trigger))
        .map(|r| r.row_id.clone())
        .collect();
    DebugDowngradeRule {
        rule_id: rule_id.to_owned(),
        title: title.to_owned(),
        trigger,
        resulting_maturity,
        affected_row_refs,
        active: true,
        shows_rule: true,
        summary: summary.to_owned(),
    }
}

fn build_downgrade_rules(rows: &[DebugQualificationRow]) -> Vec<DebugDowngradeRule> {
    use DebugClaimMaturity::*;
    use DowngradeTrigger::*;

    vec![
        downgrade_rule(
            "debug.rule:evidence_aging:0001",
            "Aging evidence narrows to retest-pending",
            EvidenceAging,
            RetestPending,
            rows,
            "A row whose evidence is aging toward its freshness SLO narrows to retest-pending.",
        ),
        downgrade_rule(
            "debug.rule:evidence_stale:0002",
            "Stale evidence narrows to retest-pending",
            EvidenceStale,
            RetestPending,
            rows,
            "A row whose evidence aged past its freshness SLO narrows to retest-pending.",
        ),
        downgrade_rule(
            "debug.rule:evidence_partial:0003",
            "Partial evidence narrows to preview",
            EvidencePartial,
            Preview,
            rows,
            "A row with incomplete evidence for its claimed scope narrows to preview.",
        ),
        downgrade_rule(
            "debug.rule:evidence_missing:0004",
            "Missing evidence withdraws the claim",
            EvidenceMissing,
            Withdrawn,
            rows,
            "A row with no current evidence is withdrawn from the product surface.",
        ),
        downgrade_rule(
            "debug.rule:support_degraded:0005",
            "Degraded support class narrows to preview",
            SupportClassDegraded,
            Preview,
            rows,
            "A row whose backend support is below supported narrows at least to preview.",
        ),
        downgrade_rule(
            "debug.rule:mapping_degraded:0006",
            "Degraded mapping fidelity narrows to preview",
            MappingFidelityDegraded,
            Preview,
            rows,
            "A row without an exact, exact-build mapping narrows at least to preview.",
        ),
        downgrade_rule(
            "debug.rule:notebook_parity_lost:0007",
            "Lost notebook parity narrows to preview",
            NotebookParityLost,
            Preview,
            rows,
            "A notebook row with divergent or unsupported parity narrows at least to preview.",
        ),
        downgrade_rule(
            "debug.rule:replay_evidence_lost:0008",
            "Lost replay evidence narrows to preview",
            ReplayEvidenceLost,
            Preview,
            rows,
            "A replay-claiming row with below-supported replay narrows at least to preview.",
        ),
        downgrade_rule(
            "debug.rule:policy_blocked:0009",
            "Policy-blocked claim is withdrawn",
            PolicyBlocked,
            Withdrawn,
            rows,
            "A row blocked by an explicit policy rule is withdrawn from the product surface.",
        ),
    ]
}

fn invariant(invariant_id: &str, statement: &str, holds: bool) -> DebugQualificationInvariant {
    DebugQualificationInvariant {
        invariant_id: invariant_id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    rows: &[DebugQualificationRow],
    publications: &[DebugClaimPublication],
    rules: &[DebugDowngradeRule],
) -> Vec<DebugQualificationInvariant> {
    let every_object_class_covered = DebugObjectClass::ALL
        .iter()
        .all(|c| rows.iter().any(|r| r.claimed_object_classes.contains(c)));

    let every_category_present = DebugRowCategory::ALL
        .iter()
        .all(|cat| rows.iter().any(|r| r.category == *cat));

    let every_status_present = DebugQualificationStatus::ALL
        .iter()
        .all(|s| rows.iter().any(|r| r.status == *s));

    let rows_consistent = rows.iter().all(DebugQualificationRow::flags_consistent);

    let stable_is_earned = rows.iter().all(|r| {
        !r.published_maturity.is_stable()
            || (r.status == DebugQualificationStatus::Certified
                && r.support_class == DebugSupportClass::Supported
                && r.mapping_fidelity.preserves_exact_source())
    });

    let narrowing_disclosed = rows
        .iter()
        .all(|r| r.narrowed != r.narrowing_reason.is_empty());

    let publications_consistent = publications.iter().all(|p| p.flags_consistent(rows));

    let every_channel_published = ClaimPublicationChannel::ALL
        .iter()
        .all(|ch| publications.iter().filter(|p| p.channel == *ch).count() == 1);

    let rules_cover_triggers = rules.iter().filter(|r| r.active).all(|rule| {
        rows.iter().all(|row| {
            if row.degradations().contains(&rule.trigger) {
                rule.affected_row_refs.contains(&row.row_id)
                    && row.published_maturity.rank() >= rule.resulting_maturity.rank()
            } else {
                true
            }
        })
    });

    let degraded_rows_are_narrowed = rows.iter().all(|r| {
        if r.status.triggers_narrowing() {
            r.published_maturity.rank() > DebugClaimMaturity::Stable.rank()
        } else {
            true
        }
    });

    vec![
        invariant(
            "debug.qual.inv:object_classes_covered",
            "Every governed debugger object family is claimed by at least one qualification row.",
            every_object_class_covered,
        ),
        invariant(
            "debug.qual.inv:categories_present",
            "Every debugger surface category — core runtime, notebook, profiler/replay, and \
             incident/support — is materialized.",
            every_category_present,
        ),
        invariant(
            "debug.qual.inv:statuses_present",
            "Every qualification status is materialized, so the narrowing machinery is exercised.",
            every_status_present,
        ),
        invariant(
            "debug.qual.inv:rows_consistent",
            "Every row's stored status, published maturity, and narrowing agree with the \
             derivation from its disclosed evidence.",
            rows_consistent,
        ),
        invariant(
            "debug.qual.inv:stable_is_earned",
            "No row publishes stable unless its status is certified with a supported backend \
             and an exact, exact-build mapping.",
            stable_is_earned,
        ),
        invariant(
            "debug.qual.inv:narrowing_disclosed",
            "Every narrowed row carries a reviewable narrowing reason and every un-narrowed \
             row carries none.",
            narrowing_disclosed,
        ),
        invariant(
            "debug.qual.inv:degraded_rows_narrowed",
            "Every row whose status triggers narrowing publishes below stable.",
            degraded_rows_are_narrowed,
        ),
        invariant(
            "debug.qual.inv:publications_floor",
            "Every claim publication republishes the narrowest maturity across the rows it \
             covers and never a wider claim.",
            publications_consistent,
        ),
        invariant(
            "debug.qual.inv:channels_published",
            "Every publication channel — claim board, About/help/service-health, support \
             export, and release packet — is published exactly once.",
            every_channel_published,
        ),
        invariant(
            "debug.qual.inv:rules_cover_triggers",
            "Every active downgrade rule lists every row exhibiting its trigger and narrows \
             each at least to its resulting maturity.",
            rules_cover_triggers,
        ),
    ]
}

/// Builds the canonical M5 debug qualification set.
///
/// Deterministic: the same bytes every call. Each row's status/maturity/narrowing and each
/// invariant's `holds` flag is computed from the built records, so an inconsistent edit flips
/// a field rather than silently passing.
pub fn m5_debug_qualification_set() -> DebugQualificationSet {
    let qualification_rows = build_rows();
    let claim_publications = build_publications(&qualification_rows);
    let downgrade_rules = build_downgrade_rules(&qualification_rows);
    let invariants = compute_invariants(&qualification_rows, &claim_publications, &downgrade_rules);

    DebugQualificationSet {
        record_kind: M5_DEBUG_QUALIFICATION_RECORD_KIND.to_owned(),
        m5_debug_qualification_schema_version: M5_DEBUG_QUALIFICATION_SCHEMA_VERSION,
        schema_ref: M5_DEBUG_QUALIFICATION_SCHEMA_REF.to_owned(),
        set_id: M5_DEBUG_QUALIFICATION_SET_ID.to_owned(),
        as_of: M5_DEBUG_QUALIFICATION_AS_OF.to_owned(),
        freeze_gate_ref: M5_DEBUG_QUALIFICATION_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed set that certifies every claimed M5 debugger-facing row \
                  against the shared debug object model and its evidence corpus, and narrows the \
                  published claim automatically when debugger evidence is stale, partial, or \
                  failing. Each qualification row binds the debugger object families it claims to \
                  the proof packets that keep it current, computes one qualification status from \
                  evidence freshness and completeness, and derives the maturity the product is \
                  allowed to publish — stable only when certified with a supported, exact-mapping \
                  backend. Claim publications for the claim board, About/help/service-health, \
                  support exports, and release packets republish the narrowest maturity across \
                  the rows they cover, and downgrade rules name why each claim narrowed."
            .to_owned(),
        source_schema_refs: strvec(&[
            "schemas/debug/m5_debug_contracts.schema.json",
            "schemas/debug/m5_chronology_replay_parity.schema.json",
            "schemas/debug/m5_dump_mapping_restore.schema.json",
        ]),
        producer_refs: strvec(&[
            "crates/aureline-debug/src/m5_debug_qualification/mod.rs",
            "crates/aureline-debug/src/m5_debug_contracts/mod.rs",
            "crates/aureline-profiler/src/certify_profiler_trace_replay_and_imported_versus_live_truth_on_all_claimed_m5_rows/mod.rs",
        ]),
        consumer_surfaces: vec![
            DebugConsumer::CoreDebugger,
            DebugConsumer::NotebookDebug,
            DebugConsumer::Profiler,
            DebugConsumer::IncidentReview,
            DebugConsumer::SupportExport,
            DebugConsumer::AiContext,
            DebugConsumer::ReviewWorkspace,
            DebugConsumer::CliHeadless,
            DebugConsumer::DocsHelp,
        ],
        qualification_rows,
        claim_publications,
        downgrade_rules,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Projects the qualification set to a human-readable, line-oriented summary.
pub fn m5_debug_qualification_lines(set: &DebugQualificationSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "M5 debug qualification — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Rows: {}  Publications: {}  Downgrade rules: {}  Invariants: {}",
        set.qualification_rows.len(),
        set.claim_publications.len(),
        set.downgrade_rules.len(),
        set.invariants.len(),
    ));

    lines.push("Qualification rows:".to_owned());
    for r in &set.qualification_rows {
        lines.push(format!(
            "  - {} [{}] status={} claimed={} published={} narrowed={}",
            r.row_id,
            r.category.as_str(),
            r.status.as_str(),
            r.claimed_maturity.as_str(),
            r.published_maturity.as_str(),
            r.narrowed,
        ));
        lines.push(format!(
            "      support={} mapping={} parity={} replay={} freshness={} complete={}",
            r.support_class.as_str(),
            r.mapping_fidelity.as_str(),
            r.notebook_parity.as_str(),
            r.replay_support.as_str(),
            r.evidence_freshness.as_str(),
            r.evidence_complete,
        ));
        if !r.narrowing_reason.is_empty() {
            lines.push(format!("      narrowed: {}", r.narrowing_reason));
        }
        lines.push(format!("      {}", r.summary));
    }

    lines.push("Claim publications:".to_owned());
    for p in &set.claim_publications {
        lines.push(format!(
            "  - {} [{}] claimed={} published={} narrowed={} rows={}",
            p.publication_id,
            p.channel.as_str(),
            p.claimed_maturity.as_str(),
            p.published_maturity.as_str(),
            p.narrowed,
            p.row_refs.len(),
        ));
        lines.push(format!("      {}", p.summary));
    }

    lines.push("Downgrade rules:".to_owned());
    for rule in &set.downgrade_rules {
        lines.push(format!(
            "  - {} trigger={} -> {} active={} rows={}",
            rule.rule_id,
            rule.trigger.as_str(),
            rule.resulting_maturity.as_str(),
            rule.active,
            rule.affected_row_refs.len(),
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
