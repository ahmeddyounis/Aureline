//! Alpha runtime fault-domain and supervisor health-event consumer.
//!
//! This module parses the checked-in runtime artifacts at
//! `/artifacts/runtime/fault_domain_taxonomy_alpha.yaml` and
//! `/artifacts/runtime/supervisor_health_events_alpha.yaml`, validates their
//! restart-budget and fail-closed invariants, and projects the same rows into
//! support/export packet form. It references repair transactions by id and
//! schema ref; repair preview, checkpoint, and reversal semantics remain owned
//! by the support repair contract.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the alpha runtime fault-domain taxonomy.
pub const RUNTIME_FAULT_DOMAIN_TAXONOMY_RECORD_KIND: &str = "runtime_fault_domain_taxonomy_alpha";

/// Stable record-kind tag for the alpha supervisor health-event catalog.
pub const SUPERVISOR_HEALTH_EVENTS_RECORD_KIND: &str = "supervisor_health_events_alpha";

/// Stable record-kind tag for the protected drill manifest.
pub const RUNTIME_FAULT_DOMAIN_DRILL_MANIFEST_RECORD_KIND: &str =
    "runtime_fault_domain_drill_manifest";

/// Stable record-kind tag for one protected drill case.
pub const RUNTIME_FAULT_DOMAIN_DRILL_CASE_RECORD_KIND: &str = "runtime_fault_domain_drill_case";

/// Stable record-kind tag for support/export projection packets.
pub const RUNTIME_FAULT_DOMAIN_SUPPORT_PACKET_RECORD_KIND: &str =
    "runtime_fault_domain_support_packet";

/// Repository-relative path for the checked-in alpha taxonomy artifact.
pub const CURRENT_ALPHA_TAXONOMY_PATH: &str = "artifacts/runtime/fault_domain_taxonomy_alpha.yaml";

/// Repository-relative path for the supervisor health-event catalog.
pub const CURRENT_ALPHA_HEALTH_EVENTS_PATH: &str =
    "artifacts/runtime/supervisor_health_events_alpha.yaml";

/// Repository-relative path for the protected drill manifest.
pub const CURRENT_ALPHA_DRILL_MANIFEST_PATH: &str =
    "fixtures/runtime/fault_domain_drills_alpha/manifest.yaml";

const CURRENT_ALPHA_TAXONOMY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/runtime/fault_domain_taxonomy_alpha.yaml"
));

const CURRENT_ALPHA_HEALTH_EVENTS_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/runtime/supervisor_health_events_alpha.yaml"
));

const CURRENT_ALPHA_DRILL_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/runtime/fault_domain_drills_alpha/manifest.yaml"
));

const DRILL_FIXTURES: [(&str, &str); 3] = [
    (
        "knowledge_worker_budget_exhaustion.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/fault_domain_drills_alpha/knowledge_worker_budget_exhaustion.yaml"
        )),
    ),
    (
        "extension_host_quarantine_recovery.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/fault_domain_drills_alpha/extension_host_quarantine_recovery.yaml"
        )),
    ),
    (
        "remote_connector_offline_recovery.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/fault_domain_drills_alpha/remote_connector_offline_recovery.yaml"
        )),
    ),
];

const REQUIRED_TRANSITIONS: [&str; 5] = ["start", "degrade", "restart", "quarantine", "recover"];

const REQUIRED_PROJECTION_FIELDS: [&str; 6] = [
    "lane_id",
    "fault_domain_id",
    "host_class_id",
    "current_state_class",
    "restart_budget_ref",
    "forensic_packet_ref",
];

/// Loads the checked-in alpha runtime fault-domain taxonomy.
///
/// # Errors
///
/// Returns a YAML parse error when the artifact does not match
/// [`RuntimeFaultDomainTaxonomy`].
pub fn current_alpha_fault_domain_taxonomy() -> Result<RuntimeFaultDomainTaxonomy, serde_yaml::Error>
{
    serde_yaml::from_str(CURRENT_ALPHA_TAXONOMY_YAML)
}

/// Loads the checked-in alpha supervisor health-event catalog.
///
/// # Errors
///
/// Returns a YAML parse error when the artifact does not match
/// [`SupervisorHealthEventCatalog`].
pub fn current_alpha_supervisor_health_events(
) -> Result<SupervisorHealthEventCatalog, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_HEALTH_EVENTS_YAML)
}

/// Loads the protected fault-domain drill manifest.
///
/// # Errors
///
/// Returns a YAML parse error when the manifest is malformed.
pub fn current_alpha_fault_domain_drill_manifest(
) -> Result<RuntimeFaultDomainDrillManifest, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_DRILL_MANIFEST_YAML)
}

/// Loads the protected fault-domain drill corpus with fixture references.
///
/// # Errors
///
/// Returns a YAML parse error when the manifest or any fixture is malformed.
pub fn current_alpha_fault_domain_drill_corpus(
) -> Result<RuntimeFaultDomainDrillCorpus, serde_yaml::Error> {
    let manifest = current_alpha_fault_domain_drill_manifest()?;
    let fixtures_by_name = DRILL_FIXTURES
        .into_iter()
        .collect::<BTreeMap<&'static str, &'static str>>();
    let entries = manifest
        .case_files
        .iter()
        .map(|case_file| {
            let yaml = fixtures_by_name
                .get(case_file.as_str())
                .copied()
                .unwrap_or_default();
            serde_yaml::from_str(yaml).map(|case| RuntimeFaultDomainDrillEntry {
                fixture_ref: format!("fixtures/runtime/fault_domain_drills_alpha/{case_file}"),
                case,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(RuntimeFaultDomainDrillCorpus { manifest, entries })
}

/// Parsed alpha runtime fault-domain taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainTaxonomy {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable taxonomy id.
    pub taxonomy_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Owning runtime/support lane.
    pub owner_lane: String,
    /// Source contracts consumed by this taxonomy.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Companion artifact refs for docs, events, and drills.
    pub companion_artifacts: RuntimeFaultDomainCompanionArtifacts,
    /// First consumer and projection contract.
    pub consumer_contract: RuntimeFaultDomainConsumerContract,
    /// Closed vocabularies used by lanes and events.
    pub closed_vocabularies: RuntimeFaultDomainClosedVocabularies,
    /// Runtime lanes covered by the alpha taxonomy.
    pub alpha_runtime_lanes: Vec<RuntimeLane>,
    /// Acceptance counters embedded in the artifact.
    pub acceptance_proof: RuntimeTaxonomyAcceptanceProof,
    /// UTC timestamp when the artifact was emitted.
    pub emitted_at: String,
}

impl RuntimeFaultDomainTaxonomy {
    /// Validates lane coverage, fail-closed state, and consumer projection refs.
    pub fn validate(
        &self,
        event_catalog: &SupervisorHealthEventCatalog,
    ) -> Vec<RuntimeFaultDomainViolation> {
        let mut violations = Vec::new();

        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "taxonomy.schema_version",
                &self.taxonomy_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != RUNTIME_FAULT_DOMAIN_TAXONOMY_RECORD_KIND {
            push_violation(
                &mut violations,
                "taxonomy.record_kind",
                &self.taxonomy_id,
                "record_kind must be runtime_fault_domain_taxonomy_alpha",
            );
        }
        if self.consumer_contract.first_consumer_ref
            != "crates/aureline-support/src/runtime_health_alpha/mod.rs"
        {
            push_violation(
                &mut violations,
                "taxonomy.consumer.first_consumer_ref",
                &self.taxonomy_id,
                "first consumer must be the support runtime-health alpha module",
            );
        }
        if self.consumer_contract.default_redaction_class != "metadata_safe_default" {
            push_violation(
                &mut violations,
                "taxonomy.consumer.default_redaction_class",
                &self.taxonomy_id,
                "default redaction must be metadata_safe_default",
            );
        }
        if self.consumer_contract.support_packet_record_kind
            != RUNTIME_FAULT_DOMAIN_SUPPORT_PACKET_RECORD_KIND
        {
            push_violation(
                &mut violations,
                "taxonomy.consumer.support_packet_record_kind",
                &self.taxonomy_id,
                "support packet record kind must match the support projection",
            );
        }

        for required_ref in [
            "restart_budget_contract",
            "forensic_packet_schema",
            "repair_transaction_schema",
            "repair_preview_alpha",
            "support_bundle_contract",
            "incident_workspace_alpha",
        ] {
            if !self.source_contract_refs.contains_key(required_ref) {
                push_violation(
                    &mut violations,
                    "taxonomy.source_contract_refs",
                    &self.taxonomy_id,
                    format!("missing source contract ref {required_ref}"),
                );
            }
        }

        let declared_transitions = self
            .closed_vocabularies
            .transition_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for transition in REQUIRED_TRANSITIONS {
            if !declared_transitions.contains(transition) {
                push_violation(
                    &mut violations,
                    "taxonomy.closed_vocabularies.transition_classes",
                    &self.taxonomy_id,
                    format!("missing transition class {transition}"),
                );
            }
        }

        let mut lane_ids = BTreeSet::new();
        let mut host_class_ids = BTreeSet::new();
        let explicit_recovery_states = self
            .closed_vocabularies
            .explicit_recovery_state_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let escalation_states = self
            .closed_vocabularies
            .escalation_state_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();

        for lane in &self.alpha_runtime_lanes {
            validate_lane(
                lane,
                &mut violations,
                &mut lane_ids,
                &mut host_class_ids,
                &explicit_recovery_states,
                &escalation_states,
            );
        }

        let restartable_lane_count = self
            .alpha_runtime_lanes
            .iter()
            .filter(|lane| lane.strike_budget.automatic_restarts_in_window > 0)
            .count() as u32;
        if self.acceptance_proof.lane_count != self.alpha_runtime_lanes.len() as u32 {
            push_violation(
                &mut violations,
                "taxonomy.acceptance_proof.lane_count",
                &self.taxonomy_id,
                "acceptance lane count must match alpha_runtime_lanes",
            );
        }
        if self.acceptance_proof.restartable_lane_count != restartable_lane_count {
            push_violation(
                &mut violations,
                "taxonomy.acceptance_proof.restartable_lane_count",
                &self.taxonomy_id,
                "restartable lane count must match lanes with automatic restarts",
            );
        }
        if !self.acceptance_proof.named_fault_domain_per_lane
            || !self.acceptance_proof.strike_budget_per_lane
            || !self.acceptance_proof.visible_fail_closed_state_per_lane
            || !self.acceptance_proof.support_bundle_projection_per_lane
            || !self.acceptance_proof.incident_packet_projection_per_lane
            || !self.acceptance_proof.ui_state_copy_projection_per_lane
        {
            push_violation(
                &mut violations,
                "taxonomy.acceptance_proof",
                &self.taxonomy_id,
                "all acceptance proof booleans must be true",
            );
        }

        let event_violations = event_catalog.validate_against_taxonomy(self);
        violations.extend(event_violations);
        violations
    }

    /// Returns the lane with the given id.
    pub fn lane(&self, lane_id: &str) -> Option<&RuntimeLane> {
        self.alpha_runtime_lanes
            .iter()
            .find(|lane| lane.lane_id == lane_id)
    }

    /// Projects the taxonomy into a support/export packet.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        drill_corpus: &RuntimeFaultDomainDrillCorpus,
    ) -> RuntimeFaultDomainSupportPacket {
        let drill_refs_by_lane = drill_corpus.drill_refs_by_lane();
        let rows = self
            .alpha_runtime_lanes
            .iter()
            .map(|lane| RuntimeFaultDomainSupportRow {
                lane_id: lane.lane_id.clone(),
                lane_label: lane.lane_label.clone(),
                fault_domain_id: lane.fault_domain_id.clone(),
                host_class_id: lane.host_class_id.clone(),
                restart_budget_ref: lane.restart_budget_ref.clone(),
                strike_window_class: lane.strike_budget.strike_window_class.clone(),
                window_seconds: lane.strike_budget.window_seconds,
                automatic_restarts_in_window: lane.strike_budget.automatic_restarts_in_window,
                degraded_state_class: lane.visible_state_contract.degraded_state_class.clone(),
                quarantine_state_class: lane.visible_state_contract.quarantine_state_class.clone(),
                fail_closed_state_class: lane
                    .visible_state_contract
                    .fail_closed_state_class
                    .clone(),
                repair_transaction_ref: lane.repair_handoff.repair_transaction_ref.clone(),
                support_bundle_item_id: lane.projection_contract.support_bundle_item_id.clone(),
                incident_packet_item_id: lane.projection_contract.incident_packet_item_id.clone(),
                ui_state_copy_ref: lane.projection_contract.ui_state_copy_ref.clone(),
                drill_case_refs: drill_refs_by_lane
                    .get(lane.lane_id.as_str())
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect();

        RuntimeFaultDomainSupportPacket {
            schema_version: 1,
            record_kind: RUNTIME_FAULT_DOMAIN_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            taxonomy_ref: CURRENT_ALPHA_TAXONOMY_PATH.to_owned(),
            supervisor_health_event_catalog_ref: CURRENT_ALPHA_HEALTH_EVENTS_PATH.to_owned(),
            redaction_class: self.consumer_contract.default_redaction_class.clone(),
            rows,
            export_safe_summary:
                "Runtime fault-domain rows are metadata-only and omit raw process payloads.".into(),
        }
    }
}

/// Companion artifact references for the taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainCompanionArtifacts {
    /// Supervisor event catalog ref.
    pub supervisor_health_events_ref: String,
    /// Human restart-budget alpha doc ref.
    pub restart_budget_alpha_doc_ref: String,
    /// Protected drill manifest ref.
    pub drill_manifest_ref: String,
}

/// Consumer contract for support, incident, and UI projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainConsumerContract {
    /// First source consumer for this artifact.
    pub first_consumer_ref: String,
    /// Support packet record kind.
    pub support_packet_record_kind: String,
    /// Support-bundle item id.
    pub support_bundle_item_id: String,
    /// UI copy projection ref.
    pub ui_state_copy_ref: String,
    /// Incident packet join ref.
    pub incident_packet_join_ref: String,
    /// Default redaction class.
    pub default_redaction_class: String,
}

/// Closed vocabularies used by the alpha fault-domain packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainClosedVocabularies {
    /// Transition classes in supervisor health events.
    pub transition_classes: Vec<String>,
    /// Runtime escalation state classes.
    pub escalation_state_classes: Vec<String>,
    /// Consumer context classes.
    pub support_context_classes: Vec<String>,
    /// States that explicitly stop hidden restart after budget exhaustion.
    pub explicit_recovery_state_classes: Vec<String>,
}

/// One alpha runtime lane bound to a fault domain and restart budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeLane {
    /// Stable lane id.
    pub lane_id: String,
    /// Human-readable lane label.
    pub lane_label: String,
    /// Runtime host class id.
    pub host_class_id: String,
    /// Fault domain id.
    pub fault_domain_id: String,
    /// Supervisor host class that owns restart decisions.
    pub supervision_owner_host_class: Option<String>,
    /// Reference into the canonical restart-budget artifact.
    pub restart_budget_ref: String,
    /// Strike-window and restart-budget row.
    pub strike_budget: RuntimeStrikeBudget,
    /// Visible state projection for this lane.
    pub visible_state_contract: RuntimeVisibleStateContract,
    /// Repair transaction handoff metadata.
    pub repair_handoff: RuntimeRepairHandoff,
    /// UI/support/incident projection metadata.
    pub projection_contract: RuntimeProjectionContract,
    /// Export-safe implementation notes.
    pub notes: String,
}

/// Strike-window and restart-budget values for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeStrikeBudget {
    /// Strike-window class.
    pub strike_window_class: String,
    /// Window length in seconds.
    pub window_seconds: u32,
    /// Automatic restarts admitted inside the window.
    pub automatic_restarts_in_window: u32,
    /// Backoff profile class.
    pub backoff_profile_class: String,
    /// Behavior once the budget is exhausted.
    pub budget_exhaustion_behavior: String,
}

/// Visible state contract for a runtime lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeVisibleStateContract {
    /// Nominal state class.
    pub nominal_state_class: String,
    /// Degraded state class.
    pub degraded_state_class: String,
    /// Quarantine or equivalent fail-closed state class.
    pub quarantine_state_class: String,
    /// State projected when budget exhaustion stops automatic restart.
    pub fail_closed_state_class: String,
    /// State that indicates successful recovery.
    pub recovery_state_class: String,
    /// Export-safe summary for UI copy and packets.
    pub user_visible_summary: String,
}

/// Repair transaction handoff metadata for a runtime lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeRepairHandoff {
    /// Optional repair transaction id owned by the repair contract.
    pub repair_transaction_ref: Option<String>,
    /// Repair transaction schema ref.
    pub repair_transaction_contract_ref: String,
    /// Whether mutation requires a repair preview.
    pub preview_required_before_mutation: bool,
    /// Reason no repair transaction is offered.
    pub no_repair_reason_class: Option<String>,
}

/// Projection metadata shared by UI copy, support bundles, and incident packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProjectionContract {
    /// Support-bundle item id.
    pub support_bundle_item_id: String,
    /// Incident-packet item id.
    pub incident_packet_item_id: String,
    /// UI state-copy ref.
    pub ui_state_copy_ref: String,
    /// Fields every projection must carry.
    pub required_projection_fields: Vec<String>,
    /// Redaction class for this row.
    pub redaction_class: String,
}

/// Acceptance proof counters embedded in the taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeTaxonomyAcceptanceProof {
    /// Count of lane rows.
    pub lane_count: u32,
    /// Count of lanes with non-zero automatic restart budgets.
    pub restartable_lane_count: u32,
    /// Whether each lane declares a fault domain.
    pub named_fault_domain_per_lane: bool,
    /// Whether each lane declares a strike budget.
    pub strike_budget_per_lane: bool,
    /// Whether each lane declares a visible fail-closed state.
    pub visible_fail_closed_state_per_lane: bool,
    /// Whether each lane projects to support bundles.
    pub support_bundle_projection_per_lane: bool,
    /// Whether each lane projects to incident packets.
    pub incident_packet_projection_per_lane: bool,
    /// Whether each lane projects to UI state copy.
    pub ui_state_copy_projection_per_lane: bool,
}

/// Parsed supervisor health-event catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisorHealthEventCatalog {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable event catalog id.
    pub event_catalog_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Source contracts consumed by this catalog.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Consumer route declarations.
    pub consumer_routes: SupervisorHealthConsumerRoutes,
    /// Required transition classes.
    pub required_transition_classes: Vec<String>,
    /// Event class rows.
    pub event_classes: Vec<SupervisorHealthEventClass>,
    /// Policy for budget exhaustion.
    pub outside_budget_policy: OutsideBudgetPolicy,
    /// UTC timestamp when the artifact was emitted.
    pub emitted_at: String,
}

impl SupervisorHealthEventCatalog {
    /// Validates event classes, consumer routes, and budget-exhaustion policy.
    pub fn validate_against_taxonomy(
        &self,
        taxonomy: &RuntimeFaultDomainTaxonomy,
    ) -> Vec<RuntimeFaultDomainViolation> {
        let mut violations = Vec::new();
        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "health_events.schema_version",
                &self.event_catalog_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != SUPERVISOR_HEALTH_EVENTS_RECORD_KIND {
            push_violation(
                &mut violations,
                "health_events.record_kind",
                &self.event_catalog_id,
                "record_kind must be supervisor_health_events_alpha",
            );
        }
        if self
            .source_contract_refs
            .get("fault_domain_taxonomy_alpha")
            .map(String::as_str)
            != Some(CURRENT_ALPHA_TAXONOMY_PATH)
        {
            push_violation(
                &mut violations,
                "health_events.source_contract_refs.fault_domain_taxonomy_alpha",
                &self.event_catalog_id,
                "event catalog must reference the alpha taxonomy",
            );
        }

        let required_transitions = self
            .required_transition_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let event_transitions = self
            .event_classes
            .iter()
            .map(|event| event.transition_class.as_str())
            .collect::<BTreeSet<_>>();
        for transition in REQUIRED_TRANSITIONS {
            if !required_transitions.contains(transition) {
                push_violation(
                    &mut violations,
                    "health_events.required_transition_classes",
                    &self.event_catalog_id,
                    format!("missing required transition {transition}"),
                );
            }
            if !event_transitions.contains(transition) {
                push_violation(
                    &mut violations,
                    "health_events.event_classes.transition_class",
                    &self.event_catalog_id,
                    format!("no event class covers transition {transition}"),
                );
            }
        }

        if !self.consumer_routes.ui_state_copy.consumable
            || !self.consumer_routes.support_bundle.consumable
            || !self.consumer_routes.incident_packet.consumable
        {
            push_violation(
                &mut violations,
                "health_events.consumer_routes",
                &self.event_catalog_id,
                "UI, support, and incident routes must all be consumable",
            );
        }

        for event in &self.event_classes {
            validate_event_class(event, &mut violations);
        }

        if self
            .outside_budget_policy
            .automatic_restart_after_budget_allowed
        {
            push_violation(
                &mut violations,
                "health_events.outside_budget_policy",
                &self.event_catalog_id,
                "automatic restart after budget exhaustion must be forbidden",
            );
        }
        if !self.outside_budget_policy.fail_closed_required {
            push_violation(
                &mut violations,
                "health_events.outside_budget_policy.fail_closed_required",
                &self.event_catalog_id,
                "budget exhaustion must fail closed",
            );
        }
        if !self
            .outside_budget_policy
            .explicit_recovery_state_classes
            .iter()
            .any(|state| state == "quarantined")
        {
            push_violation(
                &mut violations,
                "health_events.outside_budget_policy.explicit_recovery_state_classes",
                &self.event_catalog_id,
                "quarantined must be an explicit fail-closed state",
            );
        }

        let taxonomy_recovery_states = taxonomy
            .closed_vocabularies
            .explicit_recovery_state_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for state in &self.outside_budget_policy.explicit_recovery_state_classes {
            if !taxonomy_recovery_states.contains(state.as_str()) {
                push_violation(
                    &mut violations,
                    "health_events.outside_budget_policy.explicit_recovery_state_classes",
                    state,
                    "outside-budget state must exist in the taxonomy recovery vocabulary",
                );
            }
        }
        violations
    }

    /// Returns the event class row with the given stable event-class id.
    pub fn event_class(&self, event_class: &str) -> Option<&SupervisorHealthEventClass> {
        self.event_classes
            .iter()
            .find(|event| event.event_class == event_class)
    }
}

/// Consumer routes for supervisor health events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisorHealthConsumerRoutes {
    /// UI state-copy route.
    pub ui_state_copy: SupervisorHealthConsumerRoute,
    /// Support-bundle route.
    pub support_bundle: SupervisorHealthConsumerRoute,
    /// Incident-packet route.
    pub incident_packet: SupervisorHealthConsumerRoute,
}

/// One consumer route for supervisor health events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisorHealthConsumerRoute {
    /// Route ref.
    pub route_ref: String,
    /// Whether the route is active for alpha.
    pub consumable: bool,
    /// Fields required by this route.
    pub required_fields: Vec<String>,
}

/// One event class in the supervisor health-event catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisorHealthEventClass {
    /// Stable event-class id.
    pub event_class: String,
    /// Transition class.
    pub transition_class: String,
    /// Export-safe event summary.
    pub summary: String,
    /// State classes that may appear before the transition.
    pub state_before_classes: Vec<String>,
    /// State classes that may appear after the transition.
    pub state_after_classes: Vec<String>,
    /// Fields required on concrete event rows.
    pub required_fields: Vec<String>,
    /// Fields exported to consumers.
    pub consumer_payload_fields: Vec<String>,
    /// Support-bundle projection metadata.
    pub support_bundle_projection: EventProjection,
    /// Incident-packet projection metadata.
    pub incident_packet_projection: EventProjection,
    /// UI state-copy projection metadata.
    pub ui_state_copy_projection: UiEventProjection,
}

/// Consumer projection metadata for one event class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventProjection {
    /// Whether the projection is active for alpha.
    pub consumable: bool,
    /// Item id used by the target packet.
    pub item_id: String,
}

/// UI state-copy projection metadata for one event class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiEventProjection {
    /// Whether the projection is active for alpha.
    pub consumable: bool,
    /// UI state-copy class.
    pub state_copy_class: String,
}

/// Restart-budget exhaustion policy for supervisor health.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutsideBudgetPolicy {
    /// Whether automatic restart may continue after budget exhaustion.
    pub automatic_restart_after_budget_allowed: bool,
    /// Whether a fail-closed state is required.
    pub fail_closed_required: bool,
    /// Explicit states used when automatic restart stops.
    pub explicit_recovery_state_classes: Vec<String>,
    /// Whether mutating recovery must route through repair preview.
    pub repair_preview_required_for_mutating_recovery: bool,
    /// Repair transaction contract ref.
    pub repair_transaction_contract_ref: String,
}

/// Protected fault-domain drill manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainDrillManifest {
    /// Manifest schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Taxonomy artifact ref.
    pub taxonomy_ref: String,
    /// Supervisor health-event catalog ref.
    pub supervisor_health_event_catalog_ref: String,
    /// Repair transaction schema ref.
    pub repair_transaction_contract_ref: String,
    /// Case file names under the drill directory.
    pub case_files: Vec<String>,
    /// Acceptance-state booleans asserted by the corpus.
    pub acceptance_states: BTreeMap<String, bool>,
}

/// Protected fault-domain drill corpus with resolved fixture refs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFaultDomainDrillCorpus {
    /// Parsed manifest.
    pub manifest: RuntimeFaultDomainDrillManifest,
    /// Parsed drill entries.
    pub entries: Vec<RuntimeFaultDomainDrillEntry>,
}

impl RuntimeFaultDomainDrillCorpus {
    /// Validates drill rows against the taxonomy and health-event catalog.
    pub fn validate(
        &self,
        taxonomy: &RuntimeFaultDomainTaxonomy,
        event_catalog: &SupervisorHealthEventCatalog,
    ) -> Vec<RuntimeFaultDomainViolation> {
        let mut violations = Vec::new();
        if self.manifest.schema_version != 1 {
            push_violation(
                &mut violations,
                "drills.manifest.schema_version",
                &self.manifest.manifest_id,
                "schema_version must be 1",
            );
        }
        if self.manifest.record_kind != RUNTIME_FAULT_DOMAIN_DRILL_MANIFEST_RECORD_KIND {
            push_violation(
                &mut violations,
                "drills.manifest.record_kind",
                &self.manifest.manifest_id,
                "record_kind must be runtime_fault_domain_drill_manifest",
            );
        }
        if self.manifest.taxonomy_ref != CURRENT_ALPHA_TAXONOMY_PATH {
            push_violation(
                &mut violations,
                "drills.manifest.taxonomy_ref",
                &self.manifest.manifest_id,
                "manifest must reference the alpha taxonomy artifact",
            );
        }
        if self.manifest.supervisor_health_event_catalog_ref != CURRENT_ALPHA_HEALTH_EVENTS_PATH {
            push_violation(
                &mut violations,
                "drills.manifest.supervisor_health_event_catalog_ref",
                &self.manifest.manifest_id,
                "manifest must reference the alpha event catalog",
            );
        }
        if !self
            .manifest
            .acceptance_states
            .values()
            .all(|asserted| *asserted)
        {
            push_violation(
                &mut violations,
                "drills.manifest.acceptance_states",
                &self.manifest.manifest_id,
                "all manifest acceptance states must be true",
            );
        }
        if self.manifest.case_files.len() != self.entries.len() {
            push_violation(
                &mut violations,
                "drills.manifest.case_files",
                &self.manifest.manifest_id,
                "case_files must match loaded drill entries",
            );
        }

        let mut covered_transitions = BTreeSet::new();
        let explicit_recovery_states = event_catalog
            .outside_budget_policy
            .explicit_recovery_state_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for entry in &self.entries {
            entry.case.validate(
                &entry.fixture_ref,
                taxonomy,
                event_catalog,
                &explicit_recovery_states,
                &mut covered_transitions,
                &mut violations,
            );
        }
        for transition in REQUIRED_TRANSITIONS {
            if !covered_transitions.contains(transition) {
                push_violation(
                    &mut violations,
                    "drills.transition_coverage",
                    &self.manifest.manifest_id,
                    format!("drill corpus does not cover transition {transition}"),
                );
            }
        }
        violations
    }

    /// Groups drill fixture refs by runtime lane id.
    pub fn drill_refs_by_lane(&self) -> BTreeMap<&str, Vec<String>> {
        let mut refs: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for entry in &self.entries {
            refs.entry(entry.case.lane_id.as_str())
                .or_default()
                .push(entry.fixture_ref.clone());
        }
        refs
    }
}

/// One parsed drill case with its fixture reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFaultDomainDrillEntry {
    /// Repository-relative fixture ref.
    pub fixture_ref: String,
    /// Parsed drill case.
    pub case: RuntimeFaultDomainDrillCase,
}

/// One protected fault-domain drill case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainDrillCase {
    /// Case schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable case id.
    pub case_id: String,
    /// Runtime lane id.
    pub lane_id: String,
    /// Fault-domain id.
    pub fault_domain_id: String,
    /// Host-class id.
    pub host_class_id: String,
    /// Export-safe scenario summary.
    pub scenario_summary: String,
    /// Drill budget snapshot.
    pub budget: DrillBudget,
    /// Expected final state for the drill.
    pub expected_final_state_class: String,
    /// Ordered supervisor health events.
    pub health_events: Vec<DrillHealthEvent>,
    /// Projection assertions.
    pub projection_assertions: DrillProjectionAssertions,
    /// Repair handoff assertions.
    pub repair_handoff: DrillRepairHandoff,
    /// Security and privacy assertions.
    pub security_privacy_assertions: DrillSecurityPrivacyAssertions,
}

impl RuntimeFaultDomainDrillCase {
    fn validate(
        &self,
        fixture_ref: &str,
        taxonomy: &RuntimeFaultDomainTaxonomy,
        event_catalog: &SupervisorHealthEventCatalog,
        explicit_recovery_states: &BTreeSet<&str>,
        covered_transitions: &mut BTreeSet<String>,
        violations: &mut Vec<RuntimeFaultDomainViolation>,
    ) {
        if self.schema_version != 1 {
            push_violation(
                violations,
                "drill.schema_version",
                &self.case_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != RUNTIME_FAULT_DOMAIN_DRILL_CASE_RECORD_KIND {
            push_violation(
                violations,
                "drill.record_kind",
                &self.case_id,
                "record_kind must be runtime_fault_domain_drill_case",
            );
        }
        let Some(lane) = taxonomy.lane(&self.lane_id) else {
            push_violation(
                violations,
                "drill.lane_id",
                &self.case_id,
                format!("{fixture_ref} references unknown lane {}", self.lane_id),
            );
            return;
        };
        if lane.fault_domain_id != self.fault_domain_id {
            push_violation(
                violations,
                "drill.fault_domain_id",
                &self.case_id,
                "drill fault domain must match taxonomy lane",
            );
        }
        if lane.host_class_id != self.host_class_id {
            push_violation(
                violations,
                "drill.host_class_id",
                &self.case_id,
                "drill host class must match taxonomy lane",
            );
        }
        if self.budget.automatic_restarts_in_window
            != lane.strike_budget.automatic_restarts_in_window
            || self.budget.strike_window_class != lane.strike_budget.strike_window_class
        {
            push_violation(
                violations,
                "drill.budget",
                &self.case_id,
                "drill budget must match taxonomy lane budget",
            );
        }
        if self.health_events.is_empty() {
            push_violation(
                violations,
                "drill.health_events",
                &self.case_id,
                "drill must include health events",
            );
        }
        if self.budget.budget_exhausted {
            if self.budget.automatic_restart_admitted_after_budget {
                push_violation(
                    violations,
                    "drill.budget.automatic_restart_admitted_after_budget",
                    &self.case_id,
                    "budget exhaustion must not admit restart after budget",
                );
            }
            let has_fail_closed_event = self
                .health_events
                .iter()
                .any(|event| event.transition_class == "quarantine");
            if !has_fail_closed_event {
                push_violation(
                    violations,
                    "drill.health_events.fail_closed",
                    &self.case_id,
                    "budget exhaustion must include a fail-closed quarantine transition",
                );
            }
            let fail_closed_state_seen = self.health_events.iter().any(|event| {
                event.transition_class == "quarantine"
                    && explicit_recovery_states.contains(event.state_after.as_str())
            });
            if !fail_closed_state_seen {
                push_violation(
                    violations,
                    "drill.health_events.state_after",
                    &self.case_id,
                    "fail-closed transition must end in an explicit recovery state",
                );
            }
        }

        let final_state = self
            .health_events
            .last()
            .map(|event| event.state_after.as_str());
        if final_state != Some(self.expected_final_state_class.as_str()) {
            push_violation(
                violations,
                "drill.expected_final_state_class",
                &self.case_id,
                "expected_final_state_class must match the last event state_after",
            );
        }

        for event in &self.health_events {
            covered_transitions.insert(event.transition_class.clone());
            validate_drill_event(event, event_catalog, explicit_recovery_states, violations);
        }

        for required in REQUIRED_PROJECTION_FIELDS {
            if !self
                .projection_assertions
                .required_projection_fields
                .iter()
                .any(|field| field == required)
            {
                push_violation(
                    violations,
                    "drill.projection_assertions.required_projection_fields",
                    &self.case_id,
                    format!("projection is missing required field {required}"),
                );
            }
        }
        if self.projection_assertions.support_bundle_item_id
            != taxonomy.consumer_contract.support_bundle_item_id
        {
            push_violation(
                violations,
                "drill.projection_assertions.support_bundle_item_id",
                &self.case_id,
                "drill support item id must match taxonomy consumer contract",
            );
        }
        if self.repair_handoff.repair_transaction_contract_ref
            != "schemas/support/repair_transaction.schema.json"
        {
            push_violation(
                violations,
                "drill.repair_handoff.repair_transaction_contract_ref",
                &self.case_id,
                "drill must bind to the repair transaction schema",
            );
        }
        if !self.repair_handoff.preview_required_before_mutation {
            push_violation(
                violations,
                "drill.repair_handoff.preview_required_before_mutation",
                &self.case_id,
                "repair handoff must require preview before mutation",
            );
        }
        if self.security_privacy_assertions.redaction_class != "metadata_safe_default"
            || self.security_privacy_assertions.raw_payload_included
        {
            push_violation(
                violations,
                "drill.security_privacy_assertions",
                &self.case_id,
                "drill must remain metadata-safe and exclude raw payloads",
            );
        }
    }
}

/// Budget snapshot used by one drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillBudget {
    /// Strike-window class.
    pub strike_window_class: String,
    /// Window length in seconds.
    pub window_seconds: u32,
    /// Automatic restart budget.
    pub automatic_restarts_in_window: u32,
    /// Counted strikes in the drill.
    pub counted_strikes: u32,
    /// Whether the drill exhausts the budget.
    pub budget_exhausted: bool,
    /// Whether restart was admitted after budget exhaustion.
    pub automatic_restart_admitted_after_budget: bool,
}

/// One supervisor health event inside a protected drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillHealthEvent {
    /// Stable event id.
    pub event_id: String,
    /// Event class.
    pub event_class: String,
    /// Transition class.
    pub transition_class: String,
    /// State before the event.
    pub state_before: Option<String>,
    /// State after the event.
    pub state_after: String,
    /// Strike count observed at the event.
    pub strike_count_at_event: u32,
    /// Restart attempt count inside the window.
    pub restart_attempt_in_window: u32,
    /// Event timestamp.
    pub observed_at: String,
    /// Optional forensic packet ref.
    pub forensic_packet_ref: Option<String>,
    /// Support packet ref.
    pub support_packet_ref: String,
    /// Incident packet ref.
    pub incident_packet_ref: String,
    /// Optional repair transaction ref.
    pub repair_transaction_ref: Option<String>,
    /// Export-safe event summary.
    pub summary: String,
}

/// Projection assertions for a drill case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillProjectionAssertions {
    /// UI state-copy ref.
    pub ui_state_copy_ref: String,
    /// Support-bundle item id.
    pub support_bundle_item_id: String,
    /// Incident-packet item id.
    pub incident_packet_item_id: String,
    /// Required projection fields.
    pub required_projection_fields: Vec<String>,
}

/// Repair handoff assertions for a drill case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillRepairHandoff {
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Repair transaction schema ref.
    pub repair_transaction_contract_ref: String,
    /// Whether preview is required before mutation.
    pub preview_required_before_mutation: bool,
}

/// Security and privacy assertions for a drill case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillSecurityPrivacyAssertions {
    /// Redaction class.
    pub redaction_class: String,
    /// Whether any raw payload is included.
    pub raw_payload_included: bool,
    /// Whether exact-build identity is required by the packet.
    pub exact_build_identity_required: bool,
}

/// Support/export packet projected from the runtime taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainSupportPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Timestamp when the packet was generated.
    pub generated_at: String,
    /// Taxonomy artifact ref.
    pub taxonomy_ref: String,
    /// Supervisor health-event catalog ref.
    pub supervisor_health_event_catalog_ref: String,
    /// Redaction class for the packet.
    pub redaction_class: String,
    /// Lane rows included in this packet.
    pub rows: Vec<RuntimeFaultDomainSupportRow>,
    /// Export-safe packet summary.
    pub export_safe_summary: String,
}

impl RuntimeFaultDomainSupportPacket {
    /// Returns true when the packet is metadata-only and all rows have ids.
    pub fn is_export_safe(&self) -> bool {
        self.redaction_class == "metadata_safe_default"
            && !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(RuntimeFaultDomainSupportRow::is_export_safe)
    }
}

/// One support/export row for a runtime lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFaultDomainSupportRow {
    /// Runtime lane id.
    pub lane_id: String,
    /// Human-readable lane label.
    pub lane_label: String,
    /// Fault-domain id.
    pub fault_domain_id: String,
    /// Host-class id.
    pub host_class_id: String,
    /// Restart-budget artifact ref.
    pub restart_budget_ref: String,
    /// Strike-window class.
    pub strike_window_class: String,
    /// Window length in seconds.
    pub window_seconds: u32,
    /// Automatic restart budget.
    pub automatic_restarts_in_window: u32,
    /// Degraded state class.
    pub degraded_state_class: String,
    /// Quarantine state class.
    pub quarantine_state_class: String,
    /// Fail-closed state class.
    pub fail_closed_state_class: String,
    /// Optional repair transaction ref.
    pub repair_transaction_ref: Option<String>,
    /// Support-bundle item id.
    pub support_bundle_item_id: String,
    /// Incident-packet item id.
    pub incident_packet_item_id: String,
    /// UI state-copy ref.
    pub ui_state_copy_ref: String,
    /// Drill case refs that exercise this lane.
    pub drill_case_refs: Vec<String>,
}

impl RuntimeFaultDomainSupportRow {
    /// Returns true when the row has stable export refs and no raw payload.
    pub fn is_export_safe(&self) -> bool {
        !self.lane_id.is_empty()
            && !self.fault_domain_id.is_empty()
            && !self.host_class_id.is_empty()
            && !self.restart_budget_ref.is_empty()
            && self.support_bundle_item_id == "support.item.runtime_fault_domains_alpha"
            && self
                .incident_packet_item_id
                .starts_with("incident.item.runtime_fault_domain.")
            && self
                .ui_state_copy_ref
                .starts_with("docs/runtime/restart_budget_alpha.md#")
    }
}

/// One validation issue found in runtime fault-domain alpha artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFaultDomainViolation {
    /// Field path or invariant id.
    pub field_path: String,
    /// Subject id that failed validation.
    pub subject_ref: String,
    /// Reviewer-facing explanation.
    pub notes: String,
}

fn validate_lane(
    lane: &RuntimeLane,
    violations: &mut Vec<RuntimeFaultDomainViolation>,
    lane_ids: &mut BTreeSet<String>,
    host_class_ids: &mut BTreeSet<String>,
    explicit_recovery_states: &BTreeSet<&str>,
    escalation_states: &BTreeSet<&str>,
) {
    if lane.lane_id.trim().is_empty()
        || lane.host_class_id.trim().is_empty()
        || lane.fault_domain_id.trim().is_empty()
    {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.identity",
            &lane.lane_id,
            "lane id, host class, and fault domain must be non-empty",
        );
    }
    if !lane_ids.insert(lane.lane_id.clone()) {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.duplicate_lane_id",
            &lane.lane_id,
            "lane ids must be unique",
        );
    }
    if !host_class_ids.insert(lane.host_class_id.clone()) {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.duplicate_host_class_id",
            &lane.host_class_id,
            "host classes must appear in exactly one alpha lane",
        );
    }
    if !lane
        .restart_budget_ref
        .starts_with("artifacts/runtime/restart_budgets.yaml#/fault_domains/")
    {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.restart_budget_ref",
            &lane.lane_id,
            "lane must reference the canonical restart budgets artifact",
        );
    }
    if lane.strike_budget.window_seconds == 0 {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.strike_budget.window_seconds",
            &lane.lane_id,
            "strike budget window must be positive",
        );
    }
    for (field, state) in [
        (
            "visible_state_contract.nominal_state_class",
            lane.visible_state_contract.nominal_state_class.as_str(),
        ),
        (
            "visible_state_contract.degraded_state_class",
            lane.visible_state_contract.degraded_state_class.as_str(),
        ),
        (
            "visible_state_contract.quarantine_state_class",
            lane.visible_state_contract.quarantine_state_class.as_str(),
        ),
        (
            "visible_state_contract.fail_closed_state_class",
            lane.visible_state_contract.fail_closed_state_class.as_str(),
        ),
        (
            "visible_state_contract.recovery_state_class",
            lane.visible_state_contract.recovery_state_class.as_str(),
        ),
    ] {
        if !escalation_states.contains(state) {
            push_violation(
                violations,
                field,
                &lane.lane_id,
                format!("state {state} is not in the escalation vocabulary"),
            );
        }
    }
    if !explicit_recovery_states
        .contains(lane.visible_state_contract.fail_closed_state_class.as_str())
    {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.visible_state_contract.fail_closed_state_class",
            &lane.lane_id,
            "fail-closed state must be explicit recovery state",
        );
    }
    if lane.repair_handoff.repair_transaction_contract_ref
        != "schemas/support/repair_transaction.schema.json"
    {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.repair_handoff.repair_transaction_contract_ref",
            &lane.lane_id,
            "repair handoff must reference the repair transaction schema",
        );
    }
    if !lane.repair_handoff.preview_required_before_mutation {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.repair_handoff.preview_required_before_mutation",
            &lane.lane_id,
            "repair handoff must require preview before mutation",
        );
    }
    if let Some(repair_ref) = &lane.repair_handoff.repair_transaction_ref {
        if !repair_ref.starts_with("repair_transaction:") {
            push_violation(
                violations,
                "taxonomy.alpha_runtime_lanes.repair_handoff.repair_transaction_ref",
                &lane.lane_id,
                "repair transaction refs must use the repair_transaction prefix",
            );
        }
    }
    if lane.projection_contract.redaction_class != "metadata_safe_default" {
        push_violation(
            violations,
            "taxonomy.alpha_runtime_lanes.projection_contract.redaction_class",
            &lane.lane_id,
            "projection redaction must be metadata_safe_default",
        );
    }
    for required in REQUIRED_PROJECTION_FIELDS {
        if !lane
            .projection_contract
            .required_projection_fields
            .iter()
            .any(|field| field == required)
        {
            push_violation(
                violations,
                "taxonomy.alpha_runtime_lanes.projection_contract.required_projection_fields",
                &lane.lane_id,
                format!("projection is missing required field {required}"),
            );
        }
    }
}

fn validate_event_class(
    event: &SupervisorHealthEventClass,
    violations: &mut Vec<RuntimeFaultDomainViolation>,
) {
    if event.event_class.trim().is_empty() || event.transition_class.trim().is_empty() {
        push_violation(
            violations,
            "health_events.event_classes.identity",
            &event.event_class,
            "event_class and transition_class must be non-empty",
        );
    }
    if !event.support_bundle_projection.consumable
        || !event.incident_packet_projection.consumable
        || !event.ui_state_copy_projection.consumable
    {
        push_violation(
            violations,
            "health_events.event_classes.projections",
            &event.event_class,
            "every event class must project to UI, support, and incident consumers",
        );
    }
    for required in [
        "event_id",
        "lane_id",
        "fault_domain_id",
        "host_class_id",
        "state_after",
    ] {
        if !event.required_fields.iter().any(|field| field == required) {
            push_violation(
                violations,
                "health_events.event_classes.required_fields",
                &event.event_class,
                format!("event class is missing required field {required}"),
            );
        }
    }
    if event.support_bundle_projection.item_id != "support.item.runtime_fault_domains_alpha" {
        push_violation(
            violations,
            "health_events.event_classes.support_bundle_projection.item_id",
            &event.event_class,
            "support projection must use the runtime fault-domain item id",
        );
    }
    if !event
        .incident_packet_projection
        .item_id
        .starts_with("incident.item.runtime_fault_domain.")
    {
        push_violation(
            violations,
            "health_events.event_classes.incident_packet_projection.item_id",
            &event.event_class,
            "incident projection must use runtime fault-domain item ids",
        );
    }
}

fn validate_drill_event(
    event: &DrillHealthEvent,
    event_catalog: &SupervisorHealthEventCatalog,
    explicit_recovery_states: &BTreeSet<&str>,
    violations: &mut Vec<RuntimeFaultDomainViolation>,
) {
    let Some(catalog_event) = event_catalog.event_class(&event.event_class) else {
        push_violation(
            violations,
            "drill.health_events.event_class",
            &event.event_id,
            "drill event references an unknown event class",
        );
        return;
    };
    if catalog_event.transition_class != event.transition_class {
        push_violation(
            violations,
            "drill.health_events.transition_class",
            &event.event_id,
            "drill event transition must match catalog event class",
        );
    }
    if !catalog_event
        .state_after_classes
        .iter()
        .any(|state| state == &event.state_after)
    {
        push_violation(
            violations,
            "drill.health_events.state_after",
            &event.event_id,
            "drill event state_after is not allowed by catalog event class",
        );
    }
    if event.transition_class == "quarantine"
        && !explicit_recovery_states.contains(event.state_after.as_str())
    {
        push_violation(
            violations,
            "drill.health_events.quarantine_state_after",
            &event.event_id,
            "quarantine transition must end in an explicit recovery state",
        );
    }
    if event.transition_class != "start" && event.forensic_packet_ref.is_none() {
        push_violation(
            violations,
            "drill.health_events.forensic_packet_ref",
            &event.event_id,
            "non-start drill events must carry a forensic packet ref",
        );
    }
    if event.support_packet_ref.trim().is_empty() || event.incident_packet_ref.trim().is_empty() {
        push_violation(
            violations,
            "drill.health_events.packet_refs",
            &event.event_id,
            "support and incident packet refs must be non-empty",
        );
    }
}

fn push_violation(
    violations: &mut Vec<RuntimeFaultDomainViolation>,
    field_path: impl Into<String>,
    subject_ref: impl Into<String>,
    notes: impl Into<String>,
) {
    violations.push(RuntimeFaultDomainViolation {
        field_path: field_path.into(),
        subject_ref: subject_ref.into(),
        notes: notes.into(),
    });
}
