//! Canonical lifecycle-telemetry emission and export-parity conformance for every long-lived M5
//! object.
//!
//! The [frozen lifecycle matrix][matrix] already binds each long-lived M5 object family to an
//! explicit state machine, one visible primary status surface, one exportable status code, one
//! controlled last-failure reason, one named recovery affordance, and an ordered inventory of
//! milestone checkpoints. This lane is the **certification capstone** that certifies, for every one
//! of those thirteen object families, that the same controlled lifecycle vocabulary **survives the
//! machine paths** — telemetry, structured logs, dashboards, and support-packet exports — so M5 state
//! truth is diagnosable from one shared contract in logs, dashboards, and packets rather than drifting
//! by surface or disappearing in export paths.
//!
//! For every object family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the object **emits its stable lifecycle and checkpoint enums into every telemetry sink** —
//!   telemetry, structured logs, dashboards, and support exports — rather than emitting local prose
//!   or dropping a sink ([`TelemetryEnumEmissionState`]);
//! - the object **emits controlled transition events attributed to a controlled actor or subsystem**
//!   rather than firing anonymous or missing transition events ([`TransitionEventEmissionState`]);
//! - the **UI and export paths agree on lifecycle naming and required fields** — the conformance
//!   suite fails the moment a status code, last-failure reason, recovery affordance, or checkpoint
//!   boundary is named or shaped differently by the two paths ([`UiExportParityState`]);
//! - and Support Center, diagnostics, and claim-publication tooling **consume the one shared
//!   lifecycle contract** rather than restating it as local prose ([`SharedContractConsumptionState`]).
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`TelemetryConformanceRow`]): one row per
//!   [`M5LifecycleObjectFamily`] naming the four telemetry sinks it emits its enums into (drawn from
//!   the [`M5LifecycleTelemetrySink`] vocabulary), the mandatory fields it keeps conformant across UI
//!   and export (drawn from the [`M5LifecycleMandatoryField`] vocabulary), the frozen primary status
//!   surface, status-code export field, and last-failure-reason field it emits, its
//!   enum-emission / transition-event / ui-export-parity / shared-contract-consumption posture,
//!   whether the same state-truth vocabulary survives headless/companion-adjacent execution, the
//!   consumer surfaces it evaluated, any active waiver, and a derived green/yellow/red
//!   [`TelemetryConformanceStatus`].
//! - the release **certification packet** ([`TelemetryConformancePacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   conformance causes ([`TelemetryConformanceCause`]), and the blocking findings the lane refuses
//!   to ship with.
//! - the **certification dashboard** ([`TelemetryConformanceDashboard`]): a light projection the
//!   Shiproom / Support Center / product UI / CLI / diagnostics / telemetry automation reads to
//!   auto-narrow a family's telemetry-conformance claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment an
//! object discloses a reduced telemetry-sink set, discloses coarse transition events, keeps a
//! disclosed, waivered export-field narrowing, or discloses a partial shared-contract adoption; it
//! drops to `red` if an object emits local prose or drops its enums from a sink, fires missing or
//! anonymous transition events, lets the UI and export paths drift on lifecycle naming or required
//! fields, replaces the shared contract with local prose, loses the same state-truth vocabulary in a
//! headless/companion-adjacent execution, fails to emit into all four telemetry sinks, fails to keep
//! all three mandatory fields conformant, or fails to certify every consumer surface the matrix
//! declares for the family. That derivation is the auto-narrowing the acceptance criteria require, and
//! the consumer-surface, telemetry-sink, and mandatory-field completeness checks are the conformance
//! lints that fail when a surface diverges from the controlled state vocabulary or skips a mandatory
//! field like the last-failure reason, recovery affordance, or checkpoint boundary.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary,
//! counts, refs, and short labels. The object-family, checkpoint, state, recovery-affordance,
//! last-failure-reason, primary-status-surface, consumer-surface, downgrade-trigger, journey, and
//! qualification vocabulary is re-exported by reference from the already frozen [matrix], and every
//! family's driving journey, explicit state machine, primary status surface, status-code export field,
//! last-failure-reason field, recovery affordance, checkpoint lineage, and applicable triggers are
//! pulled straight from that matrix's seeded packet, so this lane mints no parallel lifecycle
//! vocabulary and cannot certify a family the matrix does not anchor. Only the telemetry-conformance
//! -specific vocabulary ([`M5LifecycleTelemetrySink`], [`M5LifecycleMandatoryField`],
//! [`M5LifecycleTelemetryDimension`], [`TelemetryConformanceStatus`], [`TelemetryEnumEmissionState`],
//! [`TransitionEventEmissionState`], [`UiExportParityState`], [`SharedContractConsumptionState`],
//! [`TelemetryConformanceWaiver`], [`TelemetryConformanceCause`], [`TelemetryConformanceFinding`]) is
//! new.
//!
//! [matrix]: crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix as matrix;

pub use matrix::{
    M5CriticalJourney, M5JourneyCheckpoint, M5LastFailureReasonClass, M5LifecycleConsumerSurface,
    M5LifecycleDowngradeTrigger, M5LifecycleObjectFamily, M5LifecycleQualificationClass,
    M5LifecycleState, M5PrimaryStatusSurface, M5RecoveryAffordance,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_lifecycle_telemetry_conformance_packet,
    seeded_m5_lifecycle_telemetry_conformance_packet_ai_shared_contract_local_prose_blocked,
    seeded_m5_lifecycle_telemetry_conformance_packet_data_ui_export_drift_blocked,
    seeded_m5_lifecycle_telemetry_conformance_packet_extension_headless_parity_lost_blocked,
    seeded_m5_lifecycle_telemetry_conformance_packet_notebook_enums_absent_blocked,
    seeded_m5_lifecycle_telemetry_conformance_packet_remote_transition_events_missing_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SHARED_CONTRACT_REF: &str =
    "lifecycle:m5_lifecycle_telemetry_conformance:v1";

/// Stable record kind for [`TelemetryConformancePacket`] payloads.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_telemetry_conformance_packet_record";

/// Stable record kind for [`TelemetryConformanceDashboard`] payloads.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_telemetry_conformance_dashboard_record";

/// Stable record kind for [`TelemetryConformanceSupportExport`] payloads.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_telemetry_conformance_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_ID: &str =
    "m5-lifecycle-telemetry-conformance:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_DASHBOARD_ID: &str =
    "m5-lifecycle-telemetry-conformance-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-lifecycle-telemetry-conformance:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-lifecycle-telemetry-conformance.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md";

/// Published certification-packet artifact ref.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-lifecycle-telemetry-conformance-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-lifecycle-telemetry-conformance-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-lifecycle-telemetry-conformance-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-lifecycle-telemetry-conformance-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_lifecycle_telemetry_conformance_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_MATRIX_DOC_REF: &str =
    matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for the driving object families.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the mandatory-field binding.
pub const M5_LIFECYCLE_TELEMETRY_CONFORMANCE_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every object family the certification must cover, in canonical order. A certification that covers
/// fewer regresses into a partial view and blocks.
pub const REQUIRED_OBJECT_FAMILIES: [M5LifecycleObjectFamily; 13] = M5LifecycleObjectFamily::ALL;

/// Every telemetry dimension each family row certifies, in canonical order.
pub const REQUIRED_TELEMETRY_DIMENSIONS: [M5LifecycleTelemetryDimension; 4] =
    M5LifecycleTelemetryDimension::ALL;

/// Every telemetry sink each family row must emit its stable enums into, in canonical order.
pub const REQUIRED_TELEMETRY_SINKS: [M5LifecycleTelemetrySink; 4] = M5LifecycleTelemetrySink::ALL;

/// Every mandatory field each family row must keep conformant across UI and export, in canonical
/// order.
pub const REQUIRED_MANDATORY_FIELDS: [M5LifecycleMandatoryField; 3] = M5LifecycleMandatoryField::ALL;

/// One of the four telemetry sinks a family's stable lifecycle and checkpoint enums must be emitted
/// into so M5 state truth survives logs, dashboards, and packets rather than living only in a live UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleTelemetrySink {
    /// Telemetry event stream.
    Telemetry,
    /// Structured application logs.
    StructuredLogs,
    /// Operator / Shiproom dashboards.
    Dashboards,
    /// Support-packet exports.
    SupportExports,
}

impl M5LifecycleTelemetrySink {
    /// Every telemetry sink, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Telemetry,
        Self::StructuredLogs,
        Self::Dashboards,
        Self::SupportExports,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Telemetry => "telemetry",
            Self::StructuredLogs => "structured_logs",
            Self::Dashboards => "dashboards",
            Self::SupportExports => "support_exports",
        }
    }
}

/// One of the three mandatory lifecycle fields the conformance suite requires the UI and export paths
/// to keep — the exact fields the spec names: the last-failure reason, the recovery affordance, and
/// the checkpoint boundary. A row that skips one blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleMandatoryField {
    /// The controlled last-failure reason field.
    LastFailureReason,
    /// The named recovery affordance field.
    RecoveryAffordance,
    /// The milestone checkpoint-boundary field.
    CheckpointBoundary,
}

impl M5LifecycleMandatoryField {
    /// Every mandatory field, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::LastFailureReason,
        Self::RecoveryAffordance,
        Self::CheckpointBoundary,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastFailureReason => "last_failure_reason",
            Self::RecoveryAffordance => "recovery_affordance",
            Self::CheckpointBoundary => "checkpoint_boundary",
        }
    }
}

/// One of the four telemetry dimensions each object-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a
/// long-lived M5 object keep its state truth diagnosable from one shared contract: it emits its stable
/// lifecycle and checkpoint enums into every telemetry sink; it emits controlled, attributed
/// transition events; its UI and export paths agree on lifecycle naming and required fields; and
/// Support Center, diagnostics, and claim tooling consume the shared contract rather than local prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleTelemetryDimension {
    /// Stable lifecycle and checkpoint enums are emitted into every telemetry sink.
    EnumEmission,
    /// Controlled, attributed transition events are emitted.
    TransitionEvent,
    /// UI and export paths agree on lifecycle naming and required fields.
    UiExportParity,
    /// Support Center, diagnostics, and claim tooling consume the shared contract, not local prose.
    SharedContractConsumption,
}

impl M5LifecycleTelemetryDimension {
    /// Every telemetry dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::EnumEmission,
        Self::TransitionEvent,
        Self::UiExportParity,
        Self::SharedContractConsumption,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnumEmission => "enum_emission",
            Self::TransitionEvent => "transition_event",
            Self::UiExportParity => "ui_export_parity",
            Self::SharedContractConsumption => "shared_contract_consumption",
        }
    }
}

/// The derived telemetry-conformance certification light an object family carries.
///
/// `green` means the object emits its stable lifecycle and checkpoint enums into all four telemetry
/// sinks, emits controlled attributed transition events, keeps its UI and export paths in agreement on
/// lifecycle naming and required fields, and has Support Center / diagnostics / claim tooling consume
/// the shared contract — across every declared consumer surface and with the same state-truth
/// vocabulary surviving a headless/companion-adjacent execution. `yellow` is a disclosed narrowing (a
/// disclosed reduced telemetry-sink set, disclosed coarse transition events, a waivered export-field
/// narrowing, or a disclosed partial shared-contract adoption). `red` is blocked: enums absent or
/// local prose emitted, transition events missing or anonymous, UI/export lifecycle naming or fields
/// drifted, the shared contract replaced by local prose, a headless/companion-adjacent vocabulary
/// loss, an incomplete telemetry-sink or mandatory-field set, or a row that did not certify every
/// declared consumer surface — and it may not keep a telemetry-conformance claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryConformanceStatus {
    /// Full standing: all four telemetry dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl TelemetryConformanceStatus {
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

/// How the object emits its stable lifecycle and checkpoint enums into the telemetry sinks.
///
/// `stable_enums_emitted_to_every_sink` means every sink — telemetry, structured logs, dashboards,
/// and support exports — carries the object's stable lifecycle and checkpoint enum tokens, not local
/// prose. `disclosed_reduced_enum_sink_set` means the object emits its stable enums into a disclosed
/// reduced set of sinks on a constrained build — for example collapsing the structured-log emission
/// into the telemetry stream while still emitting stable enums into telemetry, dashboards, and support
/// exports (a yellow narrowing). `enums_absent_or_local_prose_emitted` means the object dropped a sink
/// or emitted human prose instead of the stable enum tokens, so logs, dashboards, or packets cannot be
/// pivoted on the controlled vocabulary — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryEnumEmissionState {
    /// Stable enums are emitted into every telemetry sink.
    StableEnumsEmittedToEverySink,
    /// The object emits its stable enums into a disclosed reduced sink set.
    DisclosedReducedEnumSinkSet,
    /// The object dropped a sink or emitted local prose instead of stable enums — a blocker.
    EnumsAbsentOrLocalProseEmitted,
}

impl TelemetryEnumEmissionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableEnumsEmittedToEverySink => "stable_enums_emitted_to_every_sink",
            Self::DisclosedReducedEnumSinkSet => "disclosed_reduced_enum_sink_set",
            Self::EnumsAbsentOrLocalProseEmitted => "enums_absent_or_local_prose_emitted",
        }
    }

    /// `true` when stable enums are emitted at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::StableEnumsEmittedToEverySink)
    }

    /// `true` when the object took a disclosed reduced-sink narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedEnumSinkSet)
    }
}

/// How the object emits controlled transition events for its state machine.
///
/// `transition_events_emitted_with_attribution` means every state transition emits a controlled event
/// carrying the from/to states and the controlled actor or subsystem that drove it.
/// `disclosed_coarse_transition_events` means the object emits a disclosed coarse-grained transition
/// event on a constrained build — for example emitting one event per checkpoint boundary rather than
/// per intermediate transition while still attributing it (a yellow narrowing).
/// `transition_events_missing_or_anonymous` means the object fires no transition event, or one with no
/// controlled actor/subsystem attribution, so a state change appears in the machine paths as an
/// anonymous jump — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionEventEmissionState {
    /// Transition events are emitted with controlled actor/subsystem attribution.
    TransitionEventsEmittedWithAttribution,
    /// The object emits disclosed coarse-grained transition events.
    DisclosedCoarseTransitionEvents,
    /// The object fires missing or anonymous transition events — a blocker.
    TransitionEventsMissingOrAnonymous,
}

impl TransitionEventEmissionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransitionEventsEmittedWithAttribution => {
                "transition_events_emitted_with_attribution"
            }
            Self::DisclosedCoarseTransitionEvents => "disclosed_coarse_transition_events",
            Self::TransitionEventsMissingOrAnonymous => "transition_events_missing_or_anonymous",
        }
    }

    /// `true` when transition events are emitted at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::TransitionEventsEmittedWithAttribution)
    }

    /// `true` when the object took a disclosed coarse-event narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCoarseTransitionEvents)
    }
}

/// How the object keeps its UI and export paths in agreement on lifecycle naming and required fields.
///
/// `ui_and_export_naming_and_fields_agree` means the status code, last-failure reason, recovery
/// affordance, and checkpoint boundary the UI shows are named and shaped identically in the export
/// path. `disclosed_export_field_narrowing` means the export path carries a disclosed reduced field
/// detail on a compact export — for example collapsing an intermediate checkpoint boundary while still
/// exporting the terminal status code, last-failure reason, and recovery affordance under the same
/// names (a yellow narrowing that reduces exported detail, so it **requires an active waiver**).
/// `ui_export_lifecycle_naming_or_fields_drifted` means the UI and export paths disagree on a
/// lifecycle name or drop a required field, so the same state reads differently in the UI than in a
/// log, dashboard, or packet — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UiExportParityState {
    /// The UI and export paths agree on lifecycle naming and required fields.
    UiAndExportNamingAndFieldsAgree,
    /// The export path carries a disclosed reduced field detail.
    DisclosedExportFieldNarrowing,
    /// The UI and export paths drifted on lifecycle naming or dropped a required field — a blocker.
    UiExportLifecycleNamingOrFieldsDrifted,
}

impl UiExportParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiAndExportNamingAndFieldsAgree => "ui_and_export_naming_and_fields_agree",
            Self::DisclosedExportFieldNarrowing => "disclosed_export_field_narrowing",
            Self::UiExportLifecycleNamingOrFieldsDrifted => {
                "ui_export_lifecycle_naming_or_fields_drifted"
            }
        }
    }

    /// `true` when the UI and export paths agree at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::UiAndExportNamingAndFieldsAgree)
    }

    /// `true` when the object took a disclosed export-field narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedExportFieldNarrowing)
    }
}

/// How Support Center, diagnostics, and claim-publication tooling consume the shared lifecycle
/// contract.
///
/// `shared_contract_consumed_no_local_prose` means every downstream consumer resolves the object's
/// lifecycle state through the one shared contract rather than restating it. `disclosed_partial_contract_adoption`
/// means a downstream consumer takes a disclosed partial adoption on a legacy surface — for example a
/// diagnostics view resolving the status code from the shared contract while still rendering a
/// disclosed local label for one legacy field (a yellow narrowing). `local_prose_replaces_shared_contract`
/// means a consumer replaced the shared contract with local prose, so Shiproom and Support Center can
/// no longer diagnose state truth from one contract — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedContractConsumptionState {
    /// Downstream consumers consume the shared contract with no local prose.
    SharedContractConsumedNoLocalProse,
    /// A downstream consumer takes a disclosed partial adoption.
    DisclosedPartialContractAdoption,
    /// A consumer replaced the shared contract with local prose — a blocker.
    LocalProseReplacesSharedContract,
}

impl SharedContractConsumptionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedContractConsumedNoLocalProse => "shared_contract_consumed_no_local_prose",
            Self::DisclosedPartialContractAdoption => "disclosed_partial_contract_adoption",
            Self::LocalProseReplacesSharedContract => "local_prose_replaces_shared_contract",
        }
    }

    /// `true` when the shared contract is consumed at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::SharedContractConsumedNoLocalProse)
    }

    /// `true` when the object took a disclosed partial-adoption narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialContractAdoption)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather
/// than blocked — never lets absent enums, missing transition events, a UI/export drift, or a shared
/// contract replaced by local prose hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformanceWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The object family the waiver applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl TelemetryConformanceWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked an object family's telemetry-conformance certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformanceCause {
    /// The object family the cause applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl TelemetryConformanceCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One object family, certified across its enum-emission, transition-event, ui-export-parity, and
/// shared-contract-consumption dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformanceRow {
    /// The object family being certified.
    pub object_family: M5LifecycleObjectFamily,
    /// Short reviewer-facing family label.
    pub object_label: String,
    /// The frozen matrix journey this family emits its telemetry through. Pulled from the matrix.
    pub matrix_journey: M5CriticalJourney,
    /// Qualification class the matrix earned for the object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this family's telemetry governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The controlled states the object's explicit state machine admits. Pulled from the matrix.
    pub admitted_states: Vec<M5LifecycleState>,
    /// The one visible primary status surface the enums are emitted from. Pulled from the matrix.
    pub primary_status_surface: M5PrimaryStatusSurface,
    /// The one exportable status-code field emitted into every sink. Pulled from the matrix.
    pub status_code_export_field: String,
    /// The one last-failure-reason field emitted into every sink. Pulled from the matrix.
    pub last_failure_reason_field: String,
    /// The one named recovery affordance the mandatory-field conformance anchors on. Pulled from the
    /// matrix.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Controlled last-failure reason classes this family reports. Pulled from the matrix.
    pub last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    /// The ordered milestone checkpoints the transition events replay over. Pulled from the matrix
    /// journey row.
    pub checkpoint_lineage: Vec<M5JourneyCheckpoint>,
    /// The four telemetry sinks this row emits its stable enums into (must be all four).
    pub emitted_telemetry_sinks: Vec<M5LifecycleTelemetrySink>,
    /// The three mandatory fields this row keeps conformant across UI and export (must be all three).
    pub conformant_mandatory_fields: Vec<M5LifecycleMandatoryField>,
    /// Consumer surfaces the matrix declares the object must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Enum-emission posture.
    pub enum_emission: TelemetryEnumEmissionState,
    /// Transition-event posture.
    pub transition_event: TransitionEventEmissionState,
    /// UI/export-parity posture.
    pub ui_export_parity: UiExportParityState,
    /// Shared-contract-consumption posture.
    pub shared_contract_consumption: SharedContractConsumptionState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed export-field narrowing is in force.
    pub active_waiver: Option<TelemetryConformanceWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: TelemetryConformanceStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<TelemetryConformanceCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl TelemetryConformanceRow {
    /// `true` when the row certified every consumer surface the matrix declares for the object — no
    /// declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when the row emits its stable enums into every one of the four telemetry sinks — the
    /// structural proof that lifecycle enums appear in telemetry, logs, dashboards, and support
    /// exports.
    pub fn telemetry_sinks_complete(&self) -> bool {
        let mut emitted: Vec<&str> = self
            .emitted_telemetry_sinks
            .iter()
            .map(|sink| sink.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_TELEMETRY_SINKS
            .iter()
            .map(|sink| sink.as_str())
            .collect();
        emitted.sort_unstable();
        emitted.dedup();
        required.sort_unstable();
        emitted == required
    }

    /// `true` when the row keeps every one of the three mandatory fields conformant — the structural
    /// proof that the conformance suite fails when a row skips the last-failure reason, recovery
    /// affordance, or checkpoint boundary.
    pub fn mandatory_fields_complete(&self) -> bool {
        let mut conformant: Vec<&str> = self
            .conformant_mandatory_fields
            .iter()
            .map(|field| field.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_MANDATORY_FIELDS
            .iter()
            .map(|field| field.as_str())
            .collect();
        conformant.sort_unstable();
        conformant.dedup();
        required.sort_unstable();
        conformant == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.telemetry_sinks_complete() {
            return true;
        }
        if !self.mandatory_fields_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.enum_emission,
            TelemetryEnumEmissionState::EnumsAbsentOrLocalProseEmitted
        ) {
            return true;
        }
        if matches!(
            self.transition_event,
            TransitionEventEmissionState::TransitionEventsMissingOrAnonymous
        ) {
            return true;
        }
        if matches!(
            self.ui_export_parity,
            UiExportParityState::UiExportLifecycleNamingOrFieldsDrifted
        ) {
            return true;
        }
        if matches!(
            self.shared_contract_consumption,
            SharedContractConsumptionState::LocalProseReplacesSharedContract
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.enum_emission.is_disclosed_narrowing()
            || self.transition_event.is_disclosed_narrowing()
            || self.ui_export_parity.is_disclosed_narrowing()
            || self.shared_contract_consumption.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the telemetry posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> TelemetryConformanceStatus {
        if self.has_hard_blocker() {
            TelemetryConformanceStatus::Red
        } else if self.has_narrowing() {
            TelemetryConformanceStatus::Yellow
        } else {
            TelemetryConformanceStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (enum emission,
    /// transition event, ui/export parity, shared-contract consumption, then structural completeness
    /// and headless parity).
    pub fn recompute_causes(&self) -> Vec<TelemetryConformanceCause> {
        let mut causes = Vec::new();
        match self.enum_emission {
            TelemetryEnumEmissionState::StableEnumsEmittedToEverySink => {}
            TelemetryEnumEmissionState::DisclosedReducedEnumSinkSet => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object emits its stable lifecycle and checkpoint enums into a \
                             disclosed reduced set of telemetry sinks on a constrained build — for \
                             example folding the structured-log emission into the telemetry stream \
                             while still emitting stable enums into telemetry, dashboards, and \
                             support exports — so the sink coverage is narrowed and disclosed rather \
                             than dropping the controlled vocabulary."
                        .to_owned(),
                });
            }
            TelemetryEnumEmissionState::EnumsAbsentOrLocalProseEmitted => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "The object dropped a telemetry sink or emitted human-readable prose \
                             instead of the stable lifecycle and checkpoint enum tokens, so logs, \
                             dashboards, and packets can no longer be pivoted on the controlled state \
                             vocabulary."
                        .to_owned(),
                });
            }
        }
        match self.transition_event {
            TransitionEventEmissionState::TransitionEventsEmittedWithAttribution => {}
            TransitionEventEmissionState::DisclosedCoarseTransitionEvents => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object emits disclosed coarse-grained transition events on a \
                             constrained build — one event per checkpoint boundary rather than per \
                             intermediate transition — while still attributing each event to a \
                             controlled actor or subsystem, so the transition telemetry is narrowed \
                             and disclosed rather than anonymous."
                        .to_owned(),
                });
            }
            TransitionEventEmissionState::TransitionEventsMissingOrAnonymous => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::AnonymousCheckpoint,
                    disclosed: false,
                    detail: "The object fired no transition event, or one carrying no controlled \
                             actor or subsystem attribution, so a state change appears in the machine \
                             paths as an anonymous jump with no attributable checkpoint boundary."
                        .to_owned(),
                });
            }
        }
        match self.ui_export_parity {
            UiExportParityState::UiAndExportNamingAndFieldsAgree => {}
            UiExportParityState::DisclosedExportFieldNarrowing => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The export path carries a disclosed, waivered reduced field detail on a \
                             compact export — collapsing one intermediate checkpoint boundary while \
                             still exporting the terminal status code, last-failure reason, and \
                             recovery affordance under the same names the UI shows — so the export \
                             parity is narrowed and disclosed rather than drifted."
                        .to_owned(),
                });
            }
            UiExportParityState::UiExportLifecycleNamingOrFieldsDrifted => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StatusCodeUnexportable,
                    disclosed: false,
                    detail: "The UI and export paths disagree on a lifecycle name or the export path \
                             dropped a required field, so the same state reads differently in the UI \
                             than in a log, dashboard, or packet and the status code is no longer \
                             exportable identically across paths."
                        .to_owned(),
                });
            }
        }
        match self.shared_contract_consumption {
            SharedContractConsumptionState::SharedContractConsumedNoLocalProse => {}
            SharedContractConsumptionState::DisclosedPartialContractAdoption => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A downstream consumer takes a disclosed partial adoption of the shared \
                             lifecycle contract on a legacy surface — resolving the status code from \
                             the shared contract while still rendering a disclosed local label for \
                             one legacy field — so the contract consumption is narrowed and disclosed \
                             rather than replaced by local prose."
                        .to_owned(),
                });
            }
            SharedContractConsumptionState::LocalProseReplacesSharedContract => {
                causes.push(TelemetryConformanceCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "A downstream consumer replaced the shared lifecycle contract with local \
                             prose, so Shiproom, Support Center, diagnostics, and claim tooling can no \
                             longer diagnose this object's state truth from one shared contract."
                        .to_owned(),
                });
            }
        }
        if !self.telemetry_sinks_complete() {
            causes.push(TelemetryConformanceCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StatusCodeUnexportable,
                disclosed: false,
                detail: "The object does not emit its stable enums into all four telemetry sinks — \
                         telemetry, structured logs, dashboards, and support exports — so its \
                         lifecycle state does not appear in every machine path."
                    .to_owned(),
            });
        }
        if !self.mandatory_fields_complete() {
            causes.push(TelemetryConformanceCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::LastFailureReasonMissing,
                disclosed: false,
                detail: "The object does not keep all three mandatory fields conformant — the \
                         last-failure reason, the recovery affordance, and the checkpoint boundary — \
                         so an emission can skip a mandatory field the conformance suite requires."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(TelemetryConformanceCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail: "A headless or companion-adjacent execution of this object lost the shared \
                         state-truth vocabulary for its telemetry emission, so the same object \
                         reports a different lifecycle and transition language depending on how it \
                         runs."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed export-field narrowing may only stay yellow (rather than red) when a waiver
    /// discloses it — reducing the detail carried into the export path is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.ui_export_parity,
            UiExportParityState::DisclosedExportFieldNarrowing
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<TelemetryConformanceFinding> {
        let mut findings = Vec::new();
        let family = self.object_family.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(TelemetryConformanceFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.telemetry_sinks_complete() {
            findings.push(TelemetryConformanceFinding::TelemetrySinksIncomplete {
                family: family.clone(),
            });
        }
        if !self.mandatory_fields_complete() {
            findings.push(TelemetryConformanceFinding::MandatoryFieldsIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(TelemetryConformanceFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.enum_emission,
            TelemetryEnumEmissionState::EnumsAbsentOrLocalProseEmitted
        ) {
            findings.push(TelemetryConformanceFinding::EnumsAbsentOrLocalProse {
                family: family.clone(),
            });
        }
        if matches!(
            self.transition_event,
            TransitionEventEmissionState::TransitionEventsMissingOrAnonymous
        ) {
            findings.push(TelemetryConformanceFinding::TransitionEventsMissing {
                family: family.clone(),
            });
        }
        if matches!(
            self.ui_export_parity,
            UiExportParityState::UiExportLifecycleNamingOrFieldsDrifted
        ) {
            findings.push(TelemetryConformanceFinding::UiExportDrift {
                family: family.clone(),
            });
        }
        if matches!(
            self.shared_contract_consumption,
            SharedContractConsumptionState::LocalProseReplacesSharedContract
        ) {
            findings.push(TelemetryConformanceFinding::SharedContractLocalProse {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, TelemetryConformanceStatus::Green) && !self.has_reason() {
            findings.push(TelemetryConformanceFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(TelemetryConformanceFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_family != self.object_family {
                findings.push(TelemetryConformanceFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(TelemetryConformanceFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(TelemetryConformanceFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(TelemetryConformanceFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} enum={} transition={} ui_export={} contract={} headless={} sinks={} fields={} surfaces={} waiver={}",
            self.object_family.as_str(),
            self.derived_status.as_str(),
            self.enum_emission.as_str(),
            self.transition_event.as_str(),
            self.ui_export_parity.as_str(),
            self.shared_contract_consumption.as_str(),
            self.headless_parity_preserved,
            self.emitted_telemetry_sinks.len(),
            self.conformant_mandatory_fields.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the telemetry-conformance certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum TelemetryConformanceFinding {
    /// An object family has no certification row.
    ObjectFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not emit its stable enums into all four telemetry sinks.
    TelemetrySinksIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not keep all three mandatory fields conformant.
    MandatoryFieldsIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// The object dropped a sink or emitted local prose instead of stable enums.
    EnumsAbsentOrLocalProse {
        /// The family token.
        family: String,
    },
    /// The object fired missing or anonymous transition events.
    TransitionEventsMissing {
        /// The family token.
        family: String,
    },
    /// The UI and export paths drifted on lifecycle naming or a required field.
    UiExportDrift {
        /// The family token.
        family: String,
    },
    /// A downstream consumer replaced the shared contract with local prose.
    SharedContractLocalProse {
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
    /// The declared conformance causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl TelemetryConformanceFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectFamilyMissing { .. } => "object_family_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::TelemetrySinksIncomplete { .. } => "telemetry_sinks_incomplete",
            Self::MandatoryFieldsIncomplete { .. } => "mandatory_fields_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::EnumsAbsentOrLocalProse { .. } => "enums_absent_or_local_prose",
            Self::TransitionEventsMissing { .. } => "transition_events_missing",
            Self::UiExportDrift { .. } => "ui_export_drift",
            Self::SharedContractLocalProse { .. } => "shared_contract_local_prose",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ObjectFamilyMissing { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::TelemetrySinksIncomplete { family }
            | Self::MandatoryFieldsIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::EnumsAbsentOrLocalProse { family }
            | Self::TransitionEventsMissing { family }
            | Self::UiExportDrift { family }
            | Self::SharedContractLocalProse { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release telemetry-conformance certification packet shared by the Shiproom / Support Center /
/// product UI / CLI / diagnostics / telemetry automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformancePacket {
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
    /// The frozen lifecycle matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen lifecycle object-state schema.
    pub object_state_schema_ref: String,
    /// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
    pub journey_checkpoint_schema_ref: String,
    /// Frozen lifecycle-matrix contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// State-object inventory this proof mirrors for the driving object families.
    pub state_object_inventory_ref: String,
    /// State-class recovery reference this proof mirrors for the mandatory-field binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four telemetry dimensions every family row certifies.
    pub required_telemetry_dimensions: Vec<String>,
    /// The four telemetry sinks every family row must emit its stable enums into.
    pub required_telemetry_sinks: Vec<String>,
    /// The three mandatory fields every family row must keep conformant.
    pub required_mandatory_fields: Vec<String>,
    /// The thirteen object families the certification must cover.
    pub required_object_families: Vec<String>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<TelemetryConformanceRow>,
    /// Object families certified, in canonical (sorted) order.
    pub covered_object_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-conformance) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<TelemetryConformanceWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<TelemetryConformanceCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<TelemetryConformanceFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Lifecycle / release automation refs that consume this packet to auto-narrow object families.
    pub lifecycle_automation_refs: Vec<String>,
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

impl TelemetryConformancePacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5LifecycleObjectFamily) -> Option<&TelemetryConformanceRow> {
        self.rows.iter().find(|row| row.object_family == family)
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
                waiver.object_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.conformance_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.object_family.as_str(),
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

    /// Projects the light certification dashboard the lifecycle automation consumes.
    pub fn dashboard(&self) -> TelemetryConformanceDashboard {
        TelemetryConformanceDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 telemetry-conformance packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per object family naming its
    /// status, the four telemetry postures, headless parity, the telemetry-sink and mandatory-field
    /// counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_family,status,enum_emission,transition_event,ui_export_parity,shared_contract_consumption,headless_parity,telemetry_sinks,mandatory_fields,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_family.as_str(),
                row.derived_status.as_str(),
                row.enum_emission.as_str(),
                row.transition_event.as_str(),
                row.ui_export_parity.as_str(),
                row.shared_contract_consumption.as_str(),
                row.headless_parity_preserved,
                row.emitted_telemetry_sinks.len(),
                row.conformant_mandatory_fields.len(),
                row.evaluated_consumer_surfaces.len(),
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
        out.push_str(
            "# M5 lifecycle telemetry conformance: stable lifecycle enums, transition events, and export parity across logs, dashboards, and packets\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_lifecycle_telemetry_conformance`](../../crates/aureline-shell/src/m5_lifecycle_telemetry_conformance/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- markdown > \\\n  artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md\n",
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
        out.push_str(&format!(
            "- Required telemetry dimensions: {}\n",
            self.required_telemetry_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Object families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full conformance): {}\n",
            self.green_row_count
        ));
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

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Object family | Status | Enum emission | Transition event | UI/export parity | Shared contract | Headless | Waiver |\n\
             | ------------- | ------ | ------------- | ---------------- | ---------------- | --------------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.object_label,
                row.derived_status.as_str(),
                row.enum_emission.as_str(),
                row.transition_event.as_str(),
                row.ui_export_parity.as_str(),
                row.shared_contract_consumption.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&TelemetryConformanceRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, TelemetryConformanceStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every long-lived M5 object emits its stable lifecycle and checkpoint enums into telemetry, structured logs, dashboards, and support exports, emits controlled attributed transition events, keeps its UI and export paths in agreement on lifecycle naming and required fields, and has Support Center, diagnostics, and claim tooling consume the one shared contract across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.object_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact conformance causes\n\n");
        if self.conformance_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.conformance_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.object_family.as_str(),
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
                    waiver.object_family.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_lifecycle_telemetry_conformance_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformanceDashboardRow {
    /// The object family.
    pub object_family: M5LifecycleObjectFamily,
    /// Short family label.
    pub object_label: String,
    /// The matrix journey the family drives.
    pub matrix_journey: M5CriticalJourney,
    /// Derived green/yellow/red status.
    pub status: TelemetryConformanceStatus,
    /// Number of telemetry sinks the stable enums are emitted into.
    pub telemetry_sink_count: usize,
    /// Number of mandatory fields kept conformant.
    pub mandatory_field_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Enum-emission posture.
    pub enum_emission: TelemetryEnumEmissionState,
    /// Transition-event posture.
    pub transition_event: TransitionEventEmissionState,
    /// UI/export-parity posture.
    pub ui_export_parity: UiExportParityState,
    /// Shared-contract-consumption posture.
    pub shared_contract_consumption: SharedContractConsumptionState,
    /// `true` when headless/companion-adjacent parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the Shiproom / Support Center / product UI / CLI / diagnostics /
/// telemetry automation reads to auto-narrow an object family's telemetry-conformance claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformanceDashboard {
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
    pub rows: Vec<TelemetryConformanceDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Lifecycle / release automation refs that consume the dashboard.
    pub lifecycle_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TelemetryConformanceDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &TelemetryConformancePacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| TelemetryConformanceDashboardRow {
                object_family: row.object_family,
                object_label: row.object_label.clone(),
                matrix_journey: row.matrix_journey,
                status: row.derived_status,
                telemetry_sink_count: row.emitted_telemetry_sinks.len(),
                mandatory_field_count: row.conformant_mandatory_fields.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                enum_emission: row.enum_emission,
                transition_event: row.transition_event,
                ui_export_parity: row.ui_export_parity,
                shared_contract_consumption: row.shared_contract_consumption,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .conformance_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SCHEMA_VERSION,
            dashboard_id: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            lifecycle_automation_refs: packet.lifecycle_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 telemetry-conformance dashboard serializes")
    }
}

/// Support-export wrapper for the telemetry-conformance certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConformanceSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: TelemetryConformancePacket,
    /// Dashboard quoted in full.
    pub dashboard: TelemetryConformanceDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl TelemetryConformanceSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the lifecycle automation — can name
    /// the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: TelemetryConformancePacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.object_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SCHEMA_VERSION,
            shared_contract_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_lifecycle_telemetry_conformance_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryConformanceInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<TelemetryConformanceRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
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

/// Builds a [`TelemetryConformancePacket`] from the exact build identity, the frozen matrix ref, and
/// the per-family certification rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_lifecycle_telemetry_conformance_packet(
    input: TelemetryConformanceInput,
) -> TelemetryConformancePacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<TelemetryConformanceRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<TelemetryConformanceFinding> = Vec::new();

    // Every object family must carry a certification row.
    let present: BTreeSet<M5LifecycleObjectFamily> =
        rows.iter().map(|row| row.object_family).collect();
    for family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(TelemetryConformanceFinding::ObjectFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_object_families: Vec<String> = {
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
        .filter(|row| matches!(row.derived_status, TelemetryConformanceStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TelemetryConformanceStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TelemetryConformanceStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(TelemetryConformanceFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<TelemetryConformanceWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<TelemetryConformanceCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_telemetry_dimensions: Vec<String> = REQUIRED_TELEMETRY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_telemetry_sinks: Vec<String> = REQUIRED_TELEMETRY_SINKS
        .iter()
        .map(|sink| sink.as_str().to_owned())
        .collect();
    let required_mandatory_fields: Vec<String> = REQUIRED_MANDATORY_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    let required_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = TelemetryConformancePacket {
        record_kind: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SCHEMA_VERSION,
        shared_contract_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_ID.to_owned(),
        source_schema_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Lifecycle telemetry enums, transition events, and export parity on every \
                   long-lived M5 object: each of the thirteen governed object families certified so \
                   its stable lifecycle and checkpoint enums are emitted into telemetry, structured \
                   logs, dashboards, and support exports; its state transitions emit controlled, \
                   attributed transition events; its UI and export paths agree on lifecycle naming \
                   and the mandatory last-failure-reason, recovery-affordance, and checkpoint-boundary \
                   fields; and Support Center, diagnostics, and claim tooling consume the one shared \
                   contract rather than local prose — across every declared consumer surface, with \
                   the same state-truth vocabulary preserved in headless and companion-adjacent \
                   execution — and each family's green/yellow/red claim auto-narrowed from its four \
                   telemetry postures."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_OBJECT_STATE_SCHEMA_REF
            .to_owned(),
        journey_checkpoint_schema_ref:
            M5_LIFECYCLE_TELEMETRY_CONFORMANCE_JOURNEY_CHECKPOINT_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_STATE_OBJECT_INVENTORY_REF
            .to_owned(),
        state_class_recovery_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_STATE_CLASS_RECOVERY_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_telemetry_dimensions,
        required_telemetry_sinks,
        required_mandatory_fields,
        required_object_families,
        rows,
        covered_object_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        conformance_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.telemetry_conformance_registry".to_owned(),
            "release_automation.auto_narrow.telemetry_conformance_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.telemetry_conformance".to_owned(),
            M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-lifecycle-telemetry-conformance".to_owned()],
        published_report_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_DASHBOARD_REF
            .to_owned(),
        published_doc_ref: M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(TelemetryConformanceFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_lifecycle_telemetry_conformance_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum TelemetryConformanceValidationError {
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
    /// The declared required telemetry dimensions do not match the lane constants.
    RequiredTelemetryDimensionsStale,
    /// The declared required telemetry sinks do not match the lane constants.
    RequiredTelemetrySinksStale,
    /// The declared required mandatory fields do not match the lane constants.
    RequiredMandatoryFieldsStale,
    /// The declared required object families do not match the lane constants.
    RequiredObjectFamiliesStale,
    /// The rows do not cover all thirteen object families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared conformance causes do not match the recomputed causes.
    ConformanceCausesStale,
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

/// Validates a packet against the telemetry-conformance certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every object family carries a
/// current certification row; each row's status is the derived auto-narrowed value, never asserted; a
/// green row cannot keep a claim while it emits local prose or drops a sink, fires missing or
/// anonymous transition events, drifts its UI and export paths, replaces the shared contract with
/// local prose, loses headless/companion-adjacent parity, fails to emit into all four telemetry sinks,
/// fails to keep all three mandatory fields conformant, or fails to certify every declared consumer
/// surface; and a disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_lifecycle_telemetry_conformance_packet(
    packet: &TelemetryConformancePacket,
) -> Result<(), Vec<TelemetryConformanceValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(TelemetryConformanceValidationError::NoRows);
    }
    if packet.record_kind != M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_RECORD_KIND {
        errors.push(TelemetryConformanceValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SCHEMA_VERSION {
        errors.push(TelemetryConformanceValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(TelemetryConformanceValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(TelemetryConformanceValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_TELEMETRY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_telemetry_dimensions != expected_dimensions {
        errors.push(TelemetryConformanceValidationError::RequiredTelemetryDimensionsStale);
    }
    let expected_sinks: Vec<String> = REQUIRED_TELEMETRY_SINKS
        .iter()
        .map(|sink| sink.as_str().to_owned())
        .collect();
    if packet.required_telemetry_sinks != expected_sinks {
        errors.push(TelemetryConformanceValidationError::RequiredTelemetrySinksStale);
    }
    let expected_fields: Vec<String> = REQUIRED_MANDATORY_FIELDS
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect();
    if packet.required_mandatory_fields != expected_fields {
        errors.push(TelemetryConformanceValidationError::RequiredMandatoryFieldsStale);
    }
    let expected_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_object_families != expected_families {
        errors.push(TelemetryConformanceValidationError::RequiredObjectFamiliesStale);
    }

    let present: BTreeSet<M5LifecycleObjectFamily> =
        packet.rows.iter().map(|row| row.object_family).collect();
    let coverage_complete = REQUIRED_OBJECT_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_FAMILIES.len() {
        errors.push(TelemetryConformanceValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_object_families {
        errors.push(TelemetryConformanceValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TelemetryConformanceStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TelemetryConformanceStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TelemetryConformanceStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(TelemetryConformanceValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<TelemetryConformanceWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(TelemetryConformanceValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<TelemetryConformanceCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(TelemetryConformanceValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<TelemetryConformanceFinding> = Vec::new();
    for family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(TelemetryConformanceFinding::ObjectFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(TelemetryConformanceFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(TelemetryConformanceFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(TelemetryConformanceValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(TelemetryConformanceValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(TelemetryConformanceValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(TelemetryConformanceValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(TelemetryConformanceValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(TelemetryConformanceValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
