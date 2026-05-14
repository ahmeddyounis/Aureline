//! Alpha route-origin support/export consumer.
//!
//! This module consumes the checked-in route-origin matrix, alpha
//! transport-decision schema companion fixtures, and command-route
//! reconstruction packet. It projects every protected fixture into a
//! support-bundle [`ActionReconstructionContext`] so support exports can
//! reconstruct command, target, route, traffic origin, policy, actor,
//! timestamp, outcome, and fallback truth without scraping UI text.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::bundle::{
    ActionPolicySourceContext, ActionReconstructionSeed, ActionabilityImpactClass,
    DiagnosticDataClass, ExactBuildCapture, HighRiskContentClass, PreviewItemSeed, SizeEstimate,
    SupportBundlePreview, SupportBundlePreviewBuilder,
};

/// Stable record kind for the route-origin matrix.
pub const ALPHA_ROUTE_ORIGIN_MATRIX_RECORD_KIND: &str = "alpha_route_origin_matrix";

/// Stable record kind for the support reconstruction packet.
pub const COMMAND_ROUTE_RECONSTRUCTION_RECORD_KIND: &str =
    "command_route_reconstruction_alpha_packet";

/// Stable record kind for fixture transport decisions.
pub const TRANSPORT_DECISION_ALPHA_RECORD_KIND: &str = "transport_decision_alpha_record";

/// Repository-relative path to the checked-in matrix.
pub const ALPHA_ROUTE_ORIGIN_MATRIX_PATH: &str = "artifacts/routes/alpha_route_origin_matrix.yaml";

/// Repository-relative path to the checked-in reconstruction packet.
pub const COMMAND_ROUTE_RECONSTRUCTION_PACKET_PATH: &str =
    "artifacts/support/command_route_reconstruction_alpha.yaml";

/// Repository-relative path to the checked-in fixture manifest.
pub const ROUTE_ORIGIN_ALPHA_FIXTURE_MANIFEST_PATH: &str =
    "fixtures/runtime/route_origin_alpha/manifest.yaml";

const ALPHA_ROUTE_ORIGIN_MATRIX_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/routes/alpha_route_origin_matrix.yaml"
));

const COMMAND_ROUTE_RECONSTRUCTION_PACKET_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/command_route_reconstruction_alpha.yaml"
));

const ROUTE_ORIGIN_ALPHA_FIXTURE_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/runtime/route_origin_alpha/manifest.yaml"
));

const ROUTE_ORIGIN_FIXTURES: [(&str, &str); 8] = [
    (
        "fixtures/runtime/route_origin_alpha/local_task_debug.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/local_task_debug.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/provider_preflight.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/provider_preflight.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/browser_handoff_publish.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/browser_handoff_publish.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/publish_pipeline.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/publish_pipeline.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/wrong_target_denied.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/wrong_target_denied.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/wrong_origin_denied.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/wrong_origin_denied.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/hidden_relay_denied.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/hidden_relay_denied.yaml"
        )),
    ),
    (
        "fixtures/runtime/route_origin_alpha/hidden_fallback_denied.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/runtime/route_origin_alpha/hidden_fallback_denied.yaml"
        )),
    ),
];

/// Loads the checked-in alpha route-origin matrix.
///
/// # Errors
///
/// Returns a YAML parse error when the matrix no longer matches
/// [`AlphaRouteOriginMatrix`].
pub fn current_alpha_route_origin_matrix() -> Result<AlphaRouteOriginMatrix, serde_yaml::Error> {
    serde_yaml::from_str(ALPHA_ROUTE_ORIGIN_MATRIX_YAML)
}

/// Loads the checked-in command-route reconstruction packet.
///
/// # Errors
///
/// Returns a YAML parse error when the packet no longer matches
/// [`CommandRouteReconstructionPacket`].
pub fn current_command_route_reconstruction_packet(
) -> Result<CommandRouteReconstructionPacket, serde_yaml::Error> {
    serde_yaml::from_str(COMMAND_ROUTE_RECONSTRUCTION_PACKET_YAML)
}

/// Loads the checked-in route-origin fixture manifest.
///
/// # Errors
///
/// Returns a YAML parse error when the manifest no longer matches
/// [`RouteOriginFixtureManifest`].
pub fn current_route_origin_fixture_manifest(
) -> Result<RouteOriginFixtureManifest, serde_yaml::Error> {
    serde_yaml::from_str(ROUTE_ORIGIN_ALPHA_FIXTURE_MANIFEST_YAML)
}

/// Loads every checked-in route-origin transport decision fixture.
///
/// # Errors
///
/// Returns a YAML parse error when any fixture no longer matches
/// [`TransportDecisionAlphaRecord`].
pub fn current_route_origin_fixture_corpus() -> Result<RouteOriginFixtureCorpus, serde_yaml::Error>
{
    let entries = ROUTE_ORIGIN_FIXTURES
        .into_iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str(yaml).map(|decision| RouteOriginFixtureEntry {
                fixture_ref: fixture_ref.to_owned(),
                decision,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RouteOriginFixtureCorpus { entries })
}

/// Builds a support-bundle preview from the checked-in alpha route-origin
/// packet and fixtures.
///
/// # Errors
///
/// Returns [`RouteOriginAlphaError::ContractViolations`] when the checked-in
/// matrix, packet, and fixtures no longer line up, or propagates support
/// preview builder errors.
pub fn current_route_origin_support_preview(
    exact_build: ExactBuildCapture,
    generated_at: impl Into<String>,
) -> Result<SupportBundlePreview, RouteOriginAlphaError> {
    let matrix = current_alpha_route_origin_matrix()?;
    let packet = current_command_route_reconstruction_packet()?;
    let fixture_manifest = current_route_origin_fixture_manifest()?;
    let corpus = current_route_origin_fixture_corpus()?;
    let violations = matrix
        .validate_packet_and_corpus(&packet, &fixture_manifest, &corpus)
        .into_iter()
        .chain(packet.validate_with_corpus(&corpus))
        .collect::<Vec<_>>();
    if !violations.is_empty() {
        return Err(RouteOriginAlphaError::ContractViolations(violations));
    }

    build_support_preview_from_corpus(exact_build, generated_at, &packet, &corpus)
}

fn build_support_preview_from_corpus(
    exact_build: ExactBuildCapture,
    generated_at: impl Into<String>,
    packet: &CommandRouteReconstructionPacket,
    corpus: &RouteOriginFixtureCorpus,
) -> Result<SupportBundlePreview, RouteOriginAlphaError> {
    let mut decisions_by_ref = corpus
        .entries
        .iter()
        .map(|entry| (entry.fixture_ref.as_str(), &entry.decision))
        .collect::<BTreeMap<_, _>>();
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:route-origin-alpha:0001",
        "Alpha route-origin support reconstruction",
        generated_at.into(),
        exact_build,
    );

    for projection in &packet.projection_rows {
        let Some(decision) = decisions_by_ref.remove(projection.fixture_ref.as_str()) else {
            continue;
        };

        builder.add_item(preview_item_for_projection(projection, decision));
        builder
            .add_action_reconstruction_context(action_context_for_decision(projection, decision));
    }

    builder.build().map_err(RouteOriginAlphaError::Preview)
}

fn preview_item_for_projection(
    projection: &ProjectionRow,
    decision: &TransportDecisionAlphaRecord,
) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: projection.support_item_id.clone(),
        title: format!("Alpha route-origin reconstruction: {}", projection.projection_id),
        data_class: DiagnosticDataClass::EnvironmentAdjacent,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "route_and_execution_truth".into(),
        artifact_kind_class: "command_route_reconstruction_alpha_packet".into(),
        manifest_path_ref: format!("projection_rows.{}", projection.projection_id),
        bundle_member_path_ref: Some(format!(
            "manifest/route-origin/{}.json",
            projection.projection_id
        )),
        source_refs: vec![
            projection.fixture_ref.clone(),
            projection.matrix_row_ref.clone(),
            COMMAND_ROUTE_RECONSTRUCTION_PACKET_PATH.into(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(4096),
            confidence_class: "estimated".into(),
            display_label: "4 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary: format!(
            "Without this row, support cannot reconstruct {} route truth for {}.",
            decision.decision_result.route_truth_state, decision.command.command_id
        ),
        notes: "Metadata-only route/origin reconstruction; raw endpoints, command bodies, and provider payloads are excluded.".into(),
    }
}

fn action_context_for_decision(
    projection: &ProjectionRow,
    decision: &TransportDecisionAlphaRecord,
) -> ActionReconstructionSeed {
    ActionReconstructionSeed {
        support_pack_item_id: projection.support_item_id.clone(),
        command_id: decision.command.command_id.clone(),
        command_descriptor_ref: decision.command.command_descriptor_ref.clone(),
        invocation_session_id: decision.command.invocation_session_id.clone(),
        target_identity_ref: decision.target.target_identity_ref.clone(),
        action_route_packet_ref: Some(decision.decision_id.clone()),
        transport_decision_ref: Some(decision.decision_id.clone()),
        action_origin_class: decision.origin.origin_scope.clone(),
        action_target_class: decision.target.target_class.clone(),
        action_route_class: decision.route.route_choice.clone(),
        action_exposure_class: decision.route.exposure_posture.clone(),
        origin_scope: Some(decision.origin.origin_scope.clone()),
        traffic_origin: Some(decision.origin.traffic_origin.clone()),
        endpoint_class: Some(decision.target.endpoint_class.clone()),
        transport_target_class: Some(decision.target.target_class.clone()),
        route_choice: Some(decision.route.route_choice.clone()),
        egress_class: Some(decision.route.egress_class.clone()),
        decision_outcome: Some(decision.decision_result.decision_outcome.clone()),
        route_truth_state: Some(decision.decision_result.route_truth_state.clone()),
        fallback_posture: Some(decision.fallback.fallback_posture.clone()),
        actor_ref: Some(decision.actor.actor_ref.clone()),
        occurred_at: Some(decision.timestamps.decided_at.clone()),
        policy_source: ActionPolicySourceContext {
            policy_source_ref: decision.policy.policy_source_ref.clone(),
            policy_epoch: decision.policy.policy_epoch.clone(),
            trust_state: trust_state_for_decision(decision).into(),
            policy_bundle_ref: decision.policy.policy_bundle_ref.clone(),
            source_class: policy_source_class_for_support(&decision.policy.policy_source_class)
                .into(),
        },
        route_summary: decision.route.route_summary.clone(),
        reviewed_enforcement_ref: Some(projection.matrix_row_ref.clone()),
        redaction_class: decision.redaction_class.clone(),
    }
}

fn policy_source_class_for_support(policy_source_class: &str) -> &'static str {
    match policy_source_class {
        "local_default" => "local_default_policy",
        "org_admin_policy" | "emergency_policy" => "admin_policy",
        "unknown_policy_source" => "unknown_policy_source",
        _ => "invocation_policy_context",
    }
}

fn trust_state_for_decision(decision: &TransportDecisionAlphaRecord) -> &'static str {
    if decision
        .decision_result
        .decision_outcome
        .starts_with("deny_")
    {
        "restricted"
    } else {
        "trusted"
    }
}

fn push_violation(
    violations: &mut Vec<RouteOriginViolation>,
    check_id: impl Into<String>,
    subject: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(RouteOriginViolation {
        check_id: check_id.into(),
        subject: subject.into(),
        message: message.into(),
    });
}

/// Parsed alpha route-origin matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaRouteOriginMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Current matrix status.
    pub status: String,
    /// Owning support or runtime reviewer.
    pub owner_dri: String,
    /// Source artifact refs consumed by the matrix.
    pub source_refs: BTreeMap<String, String>,
    /// First consumer refs that must stay real and inspectable.
    pub consumer_refs: BTreeMap<String, String>,
    /// Frozen vocabulary buckets used by the matrix.
    pub canonical_vocabularies: BTreeMap<String, Vec<String>>,
    /// Fields every export must preserve.
    pub required_export_fields: Vec<String>,
    /// Route rows covered by the matrix.
    pub route_rows: Vec<RouteOriginMatrixRow>,
    /// Validation requirements for the protected corpus.
    pub validation_contract: MatrixValidationContract,
}

impl AlphaRouteOriginMatrix {
    /// Validates this matrix against the reconstruction packet and fixture
    /// corpus.
    pub fn validate_packet_and_corpus(
        &self,
        packet: &CommandRouteReconstructionPacket,
        fixture_manifest: &RouteOriginFixtureManifest,
        corpus: &RouteOriginFixtureCorpus,
    ) -> Vec<RouteOriginViolation> {
        let mut violations = Vec::new();
        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "matrix.schema_version",
                &self.matrix_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != ALPHA_ROUTE_ORIGIN_MATRIX_RECORD_KIND {
            push_violation(
                &mut violations,
                "matrix.record_kind",
                &self.matrix_id,
                "record_kind is not alpha_route_origin_matrix",
            );
        }

        let fixture_refs = corpus
            .entries
            .iter()
            .map(|entry| entry.fixture_ref.as_str())
            .collect::<BTreeSet<_>>();
        let projection_refs = packet
            .projection_rows
            .iter()
            .map(|row| row.fixture_ref.as_str())
            .collect::<BTreeSet<_>>();
        let manifest_files = fixture_manifest
            .case_files
            .iter()
            .map(|case| format!("fixtures/runtime/route_origin_alpha/{}", case.file))
            .collect::<BTreeSet<_>>();

        for row in &self.route_rows {
            row.validate_vocabularies(self, &mut violations);
            if !fixture_refs.contains(row.fixture_ref.as_str()) {
                push_violation(
                    &mut violations,
                    "matrix.fixture_ref",
                    &row.row_id,
                    format!("fixture_ref {} is not loaded", row.fixture_ref),
                );
            }
            if !projection_refs.contains(row.fixture_ref.as_str()) {
                push_violation(
                    &mut violations,
                    "matrix.support_projection_ref",
                    &row.row_id,
                    "no reconstruction packet projection consumes this fixture",
                );
            }
            if !manifest_files.contains(row.fixture_ref.as_str()) {
                push_violation(
                    &mut violations,
                    "matrix.fixture_manifest",
                    &row.row_id,
                    "fixture manifest does not list this route row fixture",
                );
            }
        }

        let states = self
            .route_rows
            .iter()
            .map(|row| row.route_truth_state.as_str())
            .collect::<BTreeSet<_>>();
        for required in &self.validation_contract.required_route_truth_states {
            if !states.contains(required.as_str()) {
                push_violation(
                    &mut violations,
                    "matrix.required_route_truth_states",
                    required,
                    "required route truth state is not covered by route rows",
                );
            }
        }

        violations
    }
}

/// One route row in the alpha matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteOriginMatrixRow {
    /// Stable row id.
    pub row_id: String,
    /// Reviewer-facing row title.
    pub title: String,
    /// Command family exercised by this row.
    pub command_family: String,
    /// Declared origin scope.
    pub origin_scope: String,
    /// Declared traffic origin.
    pub traffic_origin: String,
    /// Declared target class.
    pub target_class: String,
    /// Declared endpoint class.
    pub endpoint_class: String,
    /// Declared route choice.
    pub route_choice: String,
    /// Declared egress class.
    pub egress_class: String,
    /// Declared route truth state.
    pub route_truth_state: String,
    /// Declared decision outcome.
    pub decision_outcome: String,
    /// Declared fallback posture.
    pub fallback_posture: String,
    /// Declared policy source class.
    pub policy_source_class: String,
    /// Declared exposure posture.
    pub exposure_posture: String,
    /// Support packet family that consumes this row.
    pub support_packet_family: String,
    /// Support packet projection ref.
    pub support_projection_ref: String,
    /// Protected fixture ref.
    pub fixture_ref: String,
    /// Fields support export must carry for this row.
    pub required_support_fields: Vec<String>,
    /// Fallback or route postures forbidden for this row.
    #[serde(default)]
    pub forbidden_postures: Vec<String>,
}

impl RouteOriginMatrixRow {
    fn validate_vocabularies(
        &self,
        matrix: &AlphaRouteOriginMatrix,
        violations: &mut Vec<RouteOriginViolation>,
    ) {
        for (field, value) in [
            ("origin_scope", &self.origin_scope),
            ("traffic_origin", &self.traffic_origin),
            ("target_class", &self.target_class),
            ("endpoint_class", &self.endpoint_class),
            ("egress_class", &self.egress_class),
            ("route_choice", &self.route_choice),
            ("route_truth_state", &self.route_truth_state),
            ("decision_outcome", &self.decision_outcome),
            ("fallback_posture", &self.fallback_posture),
            ("policy_source_class", &self.policy_source_class),
        ] {
            let Some(allowed) = matrix.canonical_vocabularies.get(field) else {
                push_violation(
                    violations,
                    "matrix.vocabulary.missing",
                    field,
                    "matrix is missing a canonical vocabulary",
                );
                continue;
            };
            if !allowed.contains(value) {
                push_violation(
                    violations,
                    "matrix.vocabulary.value",
                    &self.row_id,
                    format!("{field} value {value} is not in the canonical vocabulary"),
                );
            }
        }
    }
}

/// Protected matrix validation contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixValidationContract {
    /// Route truth states that must be covered.
    pub required_route_truth_states: Vec<String>,
    /// Route choices that must be covered.
    pub required_route_choices: Vec<String>,
    /// Consumer fields every projection must preserve.
    pub required_consumer_fields: Vec<String>,
    /// Fixture manifest ref for protected coverage.
    pub protected_fixture_manifest_ref: String,
}

/// Parsed command-route reconstruction packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandRouteReconstructionPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Current packet status.
    pub status: String,
    /// Owning support reviewer.
    pub owner_dri: String,
    /// Artifacts consumed by this packet.
    pub consumes: BTreeMap<String, String>,
    /// First consumer refs.
    pub first_consumers: BTreeMap<String, String>,
    /// Packet-level support bundle contract.
    pub packet_contract: PacketContract,
    /// Per-fixture projection rows.
    pub projection_rows: Vec<ProjectionRow>,
}

impl CommandRouteReconstructionPacket {
    /// Validates projection rows against the fixture corpus.
    pub fn validate_with_corpus(
        &self,
        corpus: &RouteOriginFixtureCorpus,
    ) -> Vec<RouteOriginViolation> {
        let mut violations = Vec::new();
        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "packet.schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != COMMAND_ROUTE_RECONSTRUCTION_RECORD_KIND {
            push_violation(
                &mut violations,
                "packet.record_kind",
                &self.packet_id,
                "record_kind is not command_route_reconstruction_alpha_packet",
            );
        }

        let decisions = corpus
            .entries
            .iter()
            .map(|entry| (entry.fixture_ref.as_str(), &entry.decision))
            .collect::<BTreeMap<_, _>>();
        for row in &self.projection_rows {
            let Some(decision) = decisions.get(row.fixture_ref.as_str()) else {
                push_violation(
                    &mut violations,
                    "packet.fixture_ref",
                    &row.projection_id,
                    "projection row fixture_ref is not in the loaded corpus",
                );
                continue;
            };
            if row.expected_route_truth_state != decision.decision_result.route_truth_state {
                push_violation(
                    &mut violations,
                    "packet.expected_route_truth_state",
                    &row.projection_id,
                    "projection expected state does not match fixture",
                );
            }
            if row.expected_outcome != decision.decision_result.decision_outcome {
                push_violation(
                    &mut violations,
                    "packet.expected_outcome",
                    &row.projection_id,
                    "projection expected outcome does not match fixture",
                );
            }
            decision.validate(&mut violations);
        }

        violations
    }
}

/// Packet-level support bundle contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketContract {
    /// Base support item id.
    pub support_item_id: String,
    /// Preview title for the support row.
    pub preview_title: String,
    /// Bundle section class.
    pub preview_bundle_section_class: String,
    /// Artifact kind class.
    pub preview_artifact_kind_class: String,
    /// Diagnostic data class.
    pub data_class: String,
    /// High-risk content class.
    pub high_risk_content_class: String,
    /// Default redaction state.
    pub default_redaction_state: String,
    /// Whether raw content may be exported.
    pub raw_content_export_allowed: bool,
    /// Required manifest fields for support reconstruction.
    pub required_manifest_fields: Vec<String>,
    /// Fields that must use typed absence when unavailable.
    pub absent_fields_must_be_typed: Vec<String>,
}

/// One projection row in the reconstruction packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionRow {
    /// Stable projection id.
    pub projection_id: String,
    /// Matrix row ref.
    pub matrix_row_ref: String,
    /// Command family.
    pub command_family: String,
    /// Support context.
    pub support_context: String,
    /// Support item id for the preview row.
    pub support_item_id: String,
    /// Fixture ref consumed by this row.
    pub fixture_ref: String,
    /// Expected route truth state.
    pub expected_route_truth_state: String,
    /// Expected decision outcome.
    pub expected_outcome: String,
    /// Whether support reconstructs without UI text.
    pub reconstructs_without_ui_text: bool,
    /// Route fields projected into support/export packets.
    pub route_fields: Vec<String>,
}

/// Parsed fixture manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteOriginFixtureManifest {
    /// Manifest schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Current manifest status.
    pub status: String,
    /// Contract refs used by the manifest.
    pub contract_refs: BTreeMap<String, String>,
    /// Required truth states covered by case files.
    pub required_truth_states: Vec<String>,
    /// Fixture case files.
    pub case_files: Vec<FixtureManifestCase>,
}

/// One fixture manifest case row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureManifestCase {
    /// Fixture file basename.
    pub file: String,
    /// Matrix row id covered by this fixture.
    pub matrix_row_id: String,
    /// Command family covered by this fixture.
    pub command_family: String,
    /// Expected route truth state.
    pub expected_route_truth_state: String,
    /// Expected decision outcome.
    pub expected_outcome: String,
    /// Expected fallback posture.
    pub expected_fallback_posture: String,
}

/// Loaded fixture corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteOriginFixtureCorpus {
    /// Loaded fixture entries.
    pub entries: Vec<RouteOriginFixtureEntry>,
}

/// One loaded fixture with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteOriginFixtureEntry {
    /// Repository-relative fixture path.
    pub fixture_ref: String,
    /// Parsed transport decision.
    pub decision: TransportDecisionAlphaRecord,
}

/// Parsed alpha transport decision record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionAlphaRecord {
    /// Transport decision schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable decision id.
    pub decision_id: String,
    /// Matrix row ref this decision exercises.
    pub matrix_row_ref: String,
    /// Transport policy snapshot ref.
    pub transport_policy_snapshot_ref: Option<String>,
    /// Command linkage.
    pub command: TransportCommand,
    /// Actor linkage.
    pub actor: TransportActor,
    /// Origin and traffic-origin linkage.
    pub origin: TransportOrigin,
    /// Target linkage.
    pub target: TransportTarget,
    /// Route choice linkage.
    pub route: TransportRoute,
    /// Policy source linkage.
    pub policy: TransportPolicy,
    /// Decision outcome and route truth state.
    pub decision_result: TransportDecisionResult,
    /// Fallback posture.
    pub fallback: TransportFallback,
    /// Transport decision timestamps.
    pub timestamps: TransportTimestamps,
    /// Export projection field contract.
    pub export_projection: TransportExportProjection,
    /// Redaction class.
    pub redaction_class: String,
    /// Optional trace refs.
    #[serde(default)]
    pub trace_refs: Vec<String>,
}

impl TransportDecisionAlphaRecord {
    fn validate(&self, violations: &mut Vec<RouteOriginViolation>) {
        if self.schema_version != 1 {
            push_violation(
                violations,
                "fixture.schema_version",
                &self.decision_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != TRANSPORT_DECISION_ALPHA_RECORD_KIND {
            push_violation(
                violations,
                "fixture.record_kind",
                &self.decision_id,
                "record_kind is not transport_decision_alpha_record",
            );
        }
        if !self.export_projection.raw_material_excluded {
            push_violation(
                violations,
                "fixture.raw_material_excluded",
                &self.decision_id,
                "support projection must exclude raw material",
            );
        }
        match self.decision_result.decision_outcome.as_str() {
            "deny_wrong_target" => {
                if self.target.target_match_state != "wrong_target_detected"
                    || self.fallback.fallback_posture != "wrong_target_denied"
                {
                    push_violation(
                        violations,
                        "fixture.wrong_target",
                        &self.decision_id,
                        "wrong-target denial must preserve target mismatch and fallback posture",
                    );
                }
            }
            "deny_wrong_origin" => {
                if self.origin.origin_match_state != "wrong_origin_detected"
                    || self.origin.expected_origin_scope.is_none()
                    || self.fallback.fallback_posture != "wrong_origin_denied"
                {
                    push_violation(
                        violations,
                        "fixture.wrong_origin",
                        &self.decision_id,
                        "wrong-origin denial must preserve expected origin and fallback posture",
                    );
                }
            }
            "deny_hidden_relay" => {
                if self.route.route_choice != "managed_relay"
                    || self.fallback.declared_to_user
                    || !self.fallback.hidden_fallback_detected
                    || self.fallback.fallback_posture != "hidden_relay_denied"
                {
                    push_violation(
                        violations,
                        "fixture.hidden_relay",
                        &self.decision_id,
                        "hidden relay denial must expose undeclared managed relay state",
                    );
                }
            }
            "deny_hidden_fallback" => {
                if self.route.prior_route_choice.is_none()
                    || self.fallback.direct_fallback_allowed
                    || !self.fallback.hidden_fallback_detected
                    || self.fallback.fallback_posture != "hidden_fallback_denied"
                {
                    push_violation(
                        violations,
                        "fixture.hidden_fallback",
                        &self.decision_id,
                        "hidden fallback denial must preserve prior route and forbid direct fallback",
                    );
                }
            }
            "degraded_route_changed" => {
                if self.route.route_choice == "browser_handoff"
                    && self.fallback.fallback_posture != "browser_handoff_required"
                {
                    push_violation(
                        violations,
                        "fixture.browser_handoff",
                        &self.decision_id,
                        "browser handoff must use browser_handoff_required fallback posture",
                    );
                }
            }
            _ => {}
        }
    }
}

/// Command linkage on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportCommand {
    /// Stable command id.
    pub command_id: String,
    /// Command descriptor ref.
    pub command_descriptor_ref: String,
    /// Invocation session id.
    pub invocation_session_id: String,
    /// Command family.
    pub command_family: String,
}

/// Actor linkage on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportActor {
    /// Actor class.
    pub actor_class: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Issuing product surface.
    pub issuing_surface: String,
}

/// Origin linkage on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportOrigin {
    /// Origin scope.
    pub origin_scope: String,
    /// Actual traffic origin.
    pub traffic_origin: String,
    /// Expected origin scope, if a policy constrained it.
    pub expected_origin_scope: Option<String>,
    /// Origin match state.
    pub origin_match_state: String,
    /// Origin ref.
    pub origin_ref: String,
}

/// Target linkage on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportTarget {
    /// Target class.
    pub target_class: String,
    /// Endpoint class.
    pub endpoint_class: String,
    /// Target identity ref.
    pub target_identity_ref: String,
    /// Intended target ref.
    pub intended_target_ref: Option<String>,
    /// Observed target ref.
    pub observed_target_ref: Option<String>,
    /// Target match state.
    pub target_match_state: String,
}

/// Route linkage on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportRoute {
    /// Route choice.
    pub route_choice: String,
    /// Route label.
    pub route_label: String,
    /// Traffic origin used by this route.
    pub traffic_origin: String,
    /// Egress class.
    pub egress_class: String,
    /// Auth posture.
    pub auth_posture: String,
    /// Whether the route changed from a planned route.
    pub route_changed: bool,
    /// Route-change reason code.
    pub route_change_reason_code: String,
    /// Prior route choice when the route changed.
    pub prior_route_choice: Option<String>,
    /// Prior target ref when the route changed.
    pub prior_target_ref: Option<String>,
    /// Exposure posture.
    pub exposure_posture: String,
    /// Redaction-safe route summary.
    pub route_summary: String,
}

/// Policy linkage on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicy {
    /// Policy source class.
    pub policy_source_class: String,
    /// Policy source ref.
    pub policy_source_ref: String,
    /// Policy epoch.
    pub policy_epoch: String,
    /// Optional policy bundle ref.
    pub policy_bundle_ref: Option<String>,
    /// Redaction-safe policy summary.
    pub policy_summary: String,
}

/// Decision result on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionResult {
    /// Decision outcome.
    pub decision_outcome: String,
    /// Route truth state.
    pub route_truth_state: String,
    /// Optional denial reason code.
    pub denial_reason_code: Option<String>,
    /// Redaction-safe outcome summary.
    pub outcome_summary: String,
    /// Optional repair hook ref.
    pub repair_hook_ref: Option<String>,
}

/// Fallback posture on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportFallback {
    /// Fallback posture.
    pub fallback_posture: String,
    /// Whether fallback was declared to the user.
    pub declared_to_user: bool,
    /// Whether direct fallback was allowed.
    pub direct_fallback_allowed: bool,
    /// Whether hidden fallback was detected.
    pub hidden_fallback_detected: bool,
    /// Redaction-safe fallback summary.
    pub fallback_summary: String,
}

/// Timestamps on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportTimestamps {
    /// Planned timestamp.
    pub planned_at: String,
    /// Decision timestamp.
    pub decided_at: String,
    /// Completion timestamp, if the action completed.
    pub completed_at: Option<String>,
}

/// Export projection block on a transport decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportExportProjection {
    /// Consumer surfaces that read this record.
    pub consumer_surfaces: Vec<String>,
    /// Support packet field refs.
    pub support_packet_field_refs: Vec<String>,
    /// Support-bundle preview field refs.
    pub support_bundle_preview_field_refs: Vec<String>,
    /// Whether raw material is excluded.
    pub raw_material_excluded: bool,
}

/// Validation finding for route-origin artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteOriginViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject that failed validation.
    pub subject: String,
    /// Reviewer-facing violation message.
    pub message: String,
}

/// Errors raised while building the route-origin support projection.
#[derive(Debug)]
pub enum RouteOriginAlphaError {
    /// YAML parsing failed.
    Yaml(serde_yaml::Error),
    /// Support preview building failed.
    Preview(crate::bundle::SupportBundlePreviewError),
    /// The checked-in contract set did not validate.
    ContractViolations(Vec<RouteOriginViolation>),
}

impl std::fmt::Display for RouteOriginAlphaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "route-origin YAML parse error: {err}"),
            Self::Preview(err) => write!(f, "route-origin support preview error: {err}"),
            Self::ContractViolations(violations) => {
                write!(f, "route-origin contract violations: {}", violations.len())
            }
        }
    }
}

impl std::error::Error for RouteOriginAlphaError {}

impl From<serde_yaml::Error> for RouteOriginAlphaError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Yaml(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::ReleaseChannelClass;

    fn fixture_capture() -> ExactBuildCapture {
        ExactBuildCapture::for_fixture(
            "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
            "0.0.0",
            ReleaseChannelClass::DevLocal,
        )
    }

    #[test]
    fn checked_in_contracts_validate() {
        let matrix = current_alpha_route_origin_matrix().expect("matrix parses");
        let packet = current_command_route_reconstruction_packet().expect("packet parses");
        let manifest = current_route_origin_fixture_manifest().expect("manifest parses");
        let corpus = current_route_origin_fixture_corpus().expect("fixtures parse");

        let violations = matrix
            .validate_packet_and_corpus(&packet, &manifest, &corpus)
            .into_iter()
            .chain(packet.validate_with_corpus(&corpus))
            .collect::<Vec<_>>();

        assert_eq!(violations, Vec::new());
    }

    #[test]
    fn support_preview_carries_transport_decision_fields() {
        let preview =
            current_route_origin_support_preview(fixture_capture(), "2026-05-14T16:10:00Z")
                .expect("preview builds");

        assert_eq!(preview.manifest.preview_items.len(), 8);
        assert_eq!(preview.manifest.action_reconstruction_contexts.len(), 8);
        assert!(preview.manifest.has_exact_build_identity());
        assert!(preview
            .manifest
            .action_reconstruction_contexts
            .iter()
            .all(|context| {
                context.transport_decision_ref.is_some()
                    && context.origin_scope.is_some()
                    && context.traffic_origin.is_some()
                    && context.route_choice.is_some()
                    && context.decision_outcome.is_some()
                    && context.fallback_posture.is_some()
                    && !context.raw_content_exported
            }));

        let hidden_fallback = preview
            .manifest
            .action_reconstruction_contexts
            .iter()
            .find(|context| context.decision_outcome.as_deref() == Some("deny_hidden_fallback"))
            .expect("hidden fallback context");
        assert_eq!(
            hidden_fallback.fallback_posture.as_deref(),
            Some("hidden_fallback_denied")
        );
    }
}
