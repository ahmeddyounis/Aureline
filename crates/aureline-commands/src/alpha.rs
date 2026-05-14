//! Alpha command-registry publication and parity validation.
//!
//! This module loads the alpha launch-wedge publication from
//! `artifacts/commands/alpha_command_registry.yaml`, validates that it projects
//! from the seeded command registry, and exposes the bounded descriptor set to
//! shell and headless consumers.

use std::collections::BTreeSet;
use std::fmt;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::registry::{seeded_registry, CommandRegistry};

/// Error returned when an alpha command publication cannot be trusted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaRegistryError {
    /// The checked-in JSON payload could not be parsed.
    Json(String),
    /// A top-level or nested alpha record failed a structural invariant.
    Invalid(String),
    /// A claimed alpha command does not exist in the seeded command registry.
    UnknownCommand(String),
    /// The alpha packet disagrees with the seeded descriptor.
    DescriptorDrift {
        /// Command whose projected descriptor field drifted.
        command_id: String,
        /// Field that drifted.
        field: String,
        /// Value in the seeded registry.
        expected: String,
        /// Value in the alpha publication.
        actual: String,
    },
    /// A fixture referenced by the alpha packet could not be parsed.
    InvalidFixture {
        /// Repository-relative fixture path.
        path: String,
        /// Parse or validation detail.
        detail: String,
    },
}

impl fmt::Display for AlphaRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(err) => write!(f, "failed to parse alpha command registry JSON: {err}"),
            Self::Invalid(detail) => write!(f, "invalid alpha command registry: {detail}"),
            Self::UnknownCommand(command_id) => {
                write!(
                    f,
                    "alpha registry references unknown command_id: {command_id}"
                )
            }
            Self::DescriptorDrift {
                command_id,
                field,
                expected,
                actual,
            } => write!(
                f,
                "alpha registry drift for {command_id}.{field}: expected {expected}, got {actual}"
            ),
            Self::InvalidFixture { path, detail } => {
                write!(f, "invalid alpha fixture {path}: {detail}")
            }
        }
    }
}

impl std::error::Error for AlphaRegistryError {}

impl From<serde_json::Error> for AlphaRegistryError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value.to_string())
    }
}

/// Published alpha registry packet for launch-wedge commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaCommandRegistryRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Alpha publication schema version.
    pub alpha_command_registry_schema_version: u32,
    /// Stable packet id.
    pub registry_id: String,
    /// Timestamp or build ref that generated the packet.
    pub generated_at: String,
    /// Registry artifact this alpha packet projects from.
    pub source_registry_artifact_ref: String,
    /// Canonical descriptor schema referenced by the source registry.
    pub source_command_descriptor_schema_ref: String,
    /// Alpha descriptor publication schema.
    pub descriptor_schema_ref: String,
    /// Alpha invocation-session schema.
    pub invocation_session_schema_ref: String,
    /// Parity report artifact for this publication.
    pub parity_report_ref: String,
    /// Disabled-reason cause-family fixture manifest.
    pub disabled_reason_vocabulary_ref: String,
    /// Result and invocation fixtures that anchor the baseline.
    #[serde(default)]
    pub result_contract_baseline_refs: Vec<String>,
    /// Surface families included in the alpha parity packet.
    #[serde(default)]
    pub surface_family_set: Vec<String>,
    /// Alpha cause-family mapping for disabled commands.
    #[serde(default)]
    pub disabled_reason_cause_families: Vec<AlphaDisabledReasonCauseFamily>,
    /// Claimed commands in this alpha publication.
    #[serde(default)]
    pub claimed_commands: Vec<AlphaCommandClaimRecord>,
}

impl AlphaCommandRegistryRecord {
    /// Parses an alpha registry record from the checked-in JSON payload.
    pub fn from_json(payload: &str) -> Result<Self, AlphaRegistryError> {
        Ok(serde_json::from_str(payload)?)
    }

    /// Returns the claim for a canonical command id.
    pub fn claim_for_command(&self, command_id: &str) -> Option<&AlphaCommandClaimRecord> {
        self.claimed_commands
            .iter()
            .find(|claim| claim.command_id == command_id)
    }

    /// Returns whether the alpha packet contains a surface-family row.
    pub fn contains_surface_family(&self, surface_family: &str) -> bool {
        self.claimed_commands.iter().any(|claim| {
            claim
                .surface_parity
                .iter()
                .any(|row| row.surface_family == surface_family)
        })
    }

    /// Returns all discoverability consumer refs matching the requested class.
    pub fn discoverability_consumers(
        &self,
        consumer_class: &str,
    ) -> Vec<&AlphaDiscoverabilityConsumerRef> {
        self.claimed_commands
            .iter()
            .flat_map(|claim| claim.discoverability_record.consumer_refs.iter())
            .filter(|consumer| consumer.consumer_class == consumer_class)
            .collect()
    }

    /// Validates that the alpha publication is a faithful projection of the seed.
    pub fn validate_against_registry(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), AlphaRegistryError> {
        if self.record_kind != "alpha_command_registry_record" {
            return Err(AlphaRegistryError::Invalid(
                "record_kind must be alpha_command_registry_record".to_string(),
            ));
        }
        if self.alpha_command_registry_schema_version != 1 {
            return Err(AlphaRegistryError::Invalid(
                "unsupported alpha_command_registry_schema_version".to_string(),
            ));
        }
        if self.claimed_commands.is_empty() {
            return Err(AlphaRegistryError::Invalid(
                "claimed_commands must be non-empty".to_string(),
            ));
        }

        self.validate_required_surface_families()?;
        self.validate_required_disabled_cause_families()?;
        self.validate_required_discoverability_consumers()?;

        let mut command_ids = BTreeSet::new();
        for claim in &self.claimed_commands {
            if !command_ids.insert(claim.command_id.as_str()) {
                return Err(AlphaRegistryError::Invalid(format!(
                    "duplicate alpha command claim {}",
                    claim.command_id
                )));
            }
            claim.validate_against_registry(registry)?;
        }

        Ok(())
    }

    fn validate_required_surface_families(&self) -> Result<(), AlphaRegistryError> {
        for required in [
            "command_palette",
            "menu_or_button",
            "keybinding_help",
            "cli_headless",
            "ai_tool_surface",
            "recipe_or_onboarding_generated",
            "browser_or_voice_descriptor",
        ] {
            if !self
                .surface_family_set
                .iter()
                .any(|value| value == required)
            {
                return Err(AlphaRegistryError::Invalid(format!(
                    "surface_family_set missing {required}"
                )));
            }
            if !self.contains_surface_family(required) {
                return Err(AlphaRegistryError::Invalid(format!(
                    "claimed_commands missing surface family {required}"
                )));
            }
        }
        Ok(())
    }

    fn validate_required_disabled_cause_families(&self) -> Result<(), AlphaRegistryError> {
        let observed = self
            .disabled_reason_cause_families
            .iter()
            .map(|row| row.cause_family.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "focus",
            "selection",
            "lifecycle_state",
            "missing_dependency",
            "policy",
            "entitlement",
            "remote_or_host_mismatch",
        ] {
            if !observed.contains(required) {
                return Err(AlphaRegistryError::Invalid(format!(
                    "disabled_reason_cause_families missing {required}"
                )));
            }
        }
        Ok(())
    }

    fn validate_required_discoverability_consumers(&self) -> Result<(), AlphaRegistryError> {
        for required in [
            "start_center_card",
            "onboarding_hint",
            "keymap_bridge",
            "help_search_result",
            "migration_guidance",
        ] {
            if self.discoverability_consumers(required).is_empty() {
                return Err(AlphaRegistryError::Invalid(format!(
                    "discoverability consumers missing {required}"
                )));
            }
        }
        Ok(())
    }
}

/// Disabled-reason cause-family row in the alpha registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaDisabledReasonCauseFamily {
    /// Human-facing cause family for alpha review.
    pub cause_family: String,
    /// Canonical machine reason code used by command packets.
    pub canonical_disabled_reason_code: String,
    /// Copy contract explaining how surfaces should disclose the condition.
    pub surface_copy_contract: String,
}

/// One command claim in the alpha registry publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaCommandClaimRecord {
    /// Stable alpha claim id.
    pub claim_id: String,
    /// Canonical command id.
    pub command_id: String,
    /// Descriptor revision projected by the alpha packet.
    pub command_revision_ref: String,
    /// Lifecycle state projected by the seeded descriptor.
    pub lifecycle_state: String,
    /// Default authority class used by shell-originating rows.
    pub authority_class: String,
    /// Capability scope projected by the seeded descriptor.
    pub capability_scope_class: String,
    /// Dominant side-effect cue projected by registry consumers.
    pub dominant_side_effect_class: String,
    /// Preview class projected by all claimed surfaces.
    pub preview_class: String,
    /// Approval posture projected by all claimed surfaces.
    pub approval_posture_class: String,
    /// Result contract class projected by result consumers.
    pub result_contract_class: String,
    /// Required evidence classes from the seeded descriptor.
    #[serde(default)]
    pub evidence_ref_class_required: Vec<String>,
    /// Automation labels disclosed for the alpha row.
    #[serde(default)]
    pub automation_labels: Vec<String>,
    /// Discoverability record and consumer proof rows.
    pub discoverability_record: AlphaDiscoverabilityRecord,
    /// Cross-surface parity rows.
    #[serde(default)]
    pub surface_parity: Vec<AlphaSurfaceParityRecord>,
    /// Invocation or result packet fixtures that exercise this command.
    #[serde(default)]
    pub invocation_packet_fixture_refs: Vec<String>,
}

impl AlphaCommandClaimRecord {
    /// Validates this alpha command claim against the seeded registry entry.
    pub fn validate_against_registry(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), AlphaRegistryError> {
        let entry = registry
            .get(&self.command_id)
            .ok_or_else(|| AlphaRegistryError::UnknownCommand(self.command_id.clone()))?;

        self.expect_field(
            "command_revision_ref",
            &entry.descriptor.command_revision_ref,
            &self.command_revision_ref,
        )?;
        self.expect_field(
            "lifecycle_state",
            &entry.descriptor.lifecycle_state,
            &self.lifecycle_state,
        )?;
        self.expect_field(
            "capability_scope_class",
            &entry.descriptor.capability_scope_class,
            &self.capability_scope_class,
        )?;
        self.expect_field(
            "dominant_side_effect_class",
            &entry.dominant_side_effect_class,
            &self.dominant_side_effect_class,
        )?;
        self.expect_field(
            "preview_class",
            &entry.descriptor.preview_class,
            &self.preview_class,
        )?;
        self.expect_field(
            "approval_posture_class",
            &entry.descriptor.approval_posture_class,
            &self.approval_posture_class,
        )?;
        self.expect_field(
            "result_contract_class",
            &entry.descriptor.result_contract.result_contract_class,
            &self.result_contract_class,
        )?;
        self.expect_string_sets(
            "automation_labels",
            &entry.automation_labels,
            &self.automation_labels,
        )?;
        self.expect_string_sets(
            "evidence_ref_class_required",
            &entry.descriptor.result_contract.evidence_ref_class_required,
            &self.evidence_ref_class_required,
        )?;

        if self.surface_parity.is_empty() {
            return Err(AlphaRegistryError::Invalid(format!(
                "{} has no surface parity rows",
                self.command_id
            )));
        }

        for consumer in &self.discoverability_record.consumer_refs {
            if consumer.command_id != self.command_id {
                return Err(AlphaRegistryError::Invalid(format!(
                    "{} consumer {} points at {}",
                    self.command_id, consumer.consumer_ref, consumer.command_id
                )));
            }
            if !consumer.preserves_preview_apply_semantics {
                return Err(AlphaRegistryError::Invalid(format!(
                    "{} consumer {} does not preserve preview/apply semantics",
                    self.command_id, consumer.consumer_ref
                )));
            }
            if consumer.disabled_reason_mode != "typed_reason_required_when_unavailable" {
                return Err(AlphaRegistryError::Invalid(format!(
                    "{} consumer {} uses unsupported disabled_reason_mode {}",
                    self.command_id, consumer.consumer_ref, consumer.disabled_reason_mode
                )));
            }
        }

        for row in &self.surface_parity {
            row.validate_for_claim(self)?;
        }

        Ok(())
    }

    fn expect_field(
        &self,
        field: &str,
        expected: &str,
        actual: &str,
    ) -> Result<(), AlphaRegistryError> {
        if expected != actual {
            return Err(AlphaRegistryError::DescriptorDrift {
                command_id: self.command_id.clone(),
                field: field.to_string(),
                expected: expected.to_string(),
                actual: actual.to_string(),
            });
        }
        Ok(())
    }

    fn expect_string_sets(
        &self,
        field: &str,
        expected: &[String],
        actual: &[String],
    ) -> Result<(), AlphaRegistryError> {
        let expected = expected.iter().map(String::as_str).collect::<BTreeSet<_>>();
        let actual = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
        if expected != actual {
            return Err(AlphaRegistryError::DescriptorDrift {
                command_id: self.command_id.clone(),
                field: field.to_string(),
                expected: expected.into_iter().collect::<Vec<_>>().join("+"),
                actual: actual.into_iter().collect::<Vec<_>>().join("+"),
            });
        }
        Ok(())
    }
}

/// Discoverability record projected by the alpha claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaDiscoverabilityRecord {
    /// Primary label ref from the seeded descriptor.
    pub primary_label_ref: String,
    /// Alias ids disclosed to consumers.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Category refs consumed by search, onboarding, and help.
    #[serde(default)]
    pub category_refs: Vec<String>,
    /// Docs/help anchor ref that owns command help.
    pub docs_help_anchor_ref: String,
    /// Consumer rows that prove discoverability round trips.
    #[serde(default)]
    pub consumer_refs: Vec<AlphaDiscoverabilityConsumerRef>,
}

/// One surface that consumes a discoverability record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaDiscoverabilityConsumerRef {
    /// Consumer class such as Start Center, onboarding, keymap bridge, or help search.
    pub consumer_class: String,
    /// Stable consumer ref inside the owning surface.
    pub consumer_ref: String,
    /// Canonical command id the consumer resolves to.
    pub command_id: String,
    /// Keyboard, voice, or intent route shown by the consumer.
    pub keyboard_route: String,
    /// Docs/help anchor shown by the consumer.
    pub descriptor_anchor_ref: String,
    /// Invocation or result packet fixture proving the route, when runnable.
    pub invocation_packet_ref: Option<String>,
    /// Whether preview/apply semantics are preserved.
    pub preserves_preview_apply_semantics: bool,
    /// Disabled-reason handling mode used by the consumer.
    pub disabled_reason_mode: String,
    /// Exact reopen ref used by onboarding/help/support surfaces.
    pub exact_reopen_ref: Option<String>,
}

/// Cross-surface parity row for one command and one surface family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaSurfaceParityRecord {
    /// Surface family being compared.
    pub surface_family: String,
    /// Concrete surface class or descriptor row.
    pub surface_class: String,
    /// Coverage status for this surface.
    pub coverage_status: String,
    /// Command id projected by the surface.
    pub projected_command_id: String,
    /// Authority class reported by this surface.
    pub authority_class: String,
    /// Preview class preserved by this surface.
    pub preview_class: String,
    /// Approval posture preserved by this surface.
    pub approval_posture_class: String,
    /// Disabled-reason handling mode.
    pub disabled_reason_mode: String,
    /// Automation support label disclosed by this surface.
    pub automation_support: String,
    /// Client-scope or handoff disclosure.
    pub client_scope_disclosure: String,
    /// Result contract class preserved by this surface.
    pub result_contract_class: String,
    /// Optional explicit-narrowing reason.
    pub narrowing_reason: Option<String>,
}

impl AlphaSurfaceParityRecord {
    fn validate_for_claim(
        &self,
        claim: &AlphaCommandClaimRecord,
    ) -> Result<(), AlphaRegistryError> {
        if self.coverage_status == "unknown_gap" {
            return Err(AlphaRegistryError::Invalid(format!(
                "{} has unknown high-risk gap on {}",
                claim.command_id, self.surface_family
            )));
        }
        if self.projected_command_id != claim.command_id {
            return Err(AlphaRegistryError::DescriptorDrift {
                command_id: claim.command_id.clone(),
                field: format!("{}.projected_command_id", self.surface_family),
                expected: claim.command_id.clone(),
                actual: self.projected_command_id.clone(),
            });
        }
        for (field, expected, actual) in [
            ("preview_class", &claim.preview_class, &self.preview_class),
            (
                "approval_posture_class",
                &claim.approval_posture_class,
                &self.approval_posture_class,
            ),
            (
                "result_contract_class",
                &claim.result_contract_class,
                &self.result_contract_class,
            ),
        ] {
            if expected != actual {
                return Err(AlphaRegistryError::DescriptorDrift {
                    command_id: claim.command_id.clone(),
                    field: format!("{}.{}", self.surface_family, field),
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            }
        }
        if self.disabled_reason_mode != "typed_reason_required_when_unavailable" {
            return Err(AlphaRegistryError::Invalid(format!(
                "{} surface {} uses disabled_reason_mode {}",
                claim.command_id, self.surface_family, self.disabled_reason_mode
            )));
        }
        Ok(())
    }
}

/// Manifest containing alpha disabled-reason fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaDisabledReasonManifest {
    /// Record discriminator.
    pub record_kind: String,
    /// Manifest schema version.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Fixture cases contained in the manifest.
    #[serde(default)]
    pub cases: Vec<AlphaDisabledReasonFixtureRecord>,
}

/// One alpha disabled-reason fixture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaDisabledReasonFixtureRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Fixture schema version.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Reviewer-facing cause family.
    pub cause_family: String,
    /// Canonical command id under test.
    pub command_id: String,
    /// Canonical disabled-reason code.
    pub disabled_reason_code: String,
    /// Repair hook surfaced with the disabled reason.
    pub repair_hook_ref: crate::descriptor::RepairHookRef,
    /// Export-safe disclosure expected from product surfaces.
    pub surface_disclosure: String,
}

/// Summary block inside the alpha parity report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaParityReportSummary {
    /// Number of commands in the report.
    pub claimed_command_count: usize,
    /// Number of surface rows checked.
    pub surface_rows_checked: usize,
    /// Number of unknown high-risk gaps.
    pub unknown_high_risk_gaps: usize,
    /// Number of blocking findings.
    pub blocking_findings: usize,
    /// Pass/fail status token.
    pub status: String,
}

/// Alpha parity report packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaParityReportRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Report schema version.
    pub alpha_command_parity_report_schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Source registry artifact.
    pub source_registry_ref: String,
    /// Summary fields used by CLI/headless consumers.
    pub summary: AlphaParityReportSummary,
}

const ALPHA_COMMAND_REGISTRY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/commands/alpha_command_registry.yaml"
));

static ALPHA_COMMAND_REGISTRY: OnceLock<AlphaCommandRegistryRecord> = OnceLock::new();

/// Returns the checked-in alpha command registry publication.
pub fn alpha_command_registry() -> &'static AlphaCommandRegistryRecord {
    ALPHA_COMMAND_REGISTRY.get_or_init(|| {
        let registry = AlphaCommandRegistryRecord::from_json(ALPHA_COMMAND_REGISTRY_JSON)
            .expect("alpha command registry must parse");
        registry
            .validate_against_registry(seeded_registry())
            .expect("alpha command registry must validate against seeded registry");
        registry
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::invocation::{CommandResultPacketRecord, InvocationSessionPacketRecord};

    fn repo_file(path: &str) -> String {
        let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
        std::fs::read_to_string(format!("{base}{path}")).expect("repo fixture must read")
    }

    #[test]
    fn alpha_registry_loads_and_validates_against_seeded_registry() {
        let registry = alpha_command_registry();
        assert_eq!(registry.record_kind, "alpha_command_registry_record");
        assert_eq!(registry.claimed_commands.len(), 6);
        assert!(registry
            .claim_for_command("cmd:workspace.open_folder")
            .is_some());
        assert!(registry
            .claim_for_command("cmd:workspace.import_profile")
            .is_some());
    }

    #[test]
    fn alpha_registry_covers_required_non_desktop_lanes() {
        let registry = alpha_command_registry();
        for family in [
            "cli_headless",
            "ai_tool_surface",
            "recipe_or_onboarding_generated",
            "browser_or_voice_descriptor",
        ] {
            assert!(
                registry.contains_surface_family(family),
                "alpha registry should cover {family}"
            );
        }
        assert!(registry
            .claimed_commands
            .iter()
            .flat_map(|claim| claim.surface_parity.iter())
            .any(|row| row.coverage_status == "voice_addressable"));
        assert!(registry
            .claimed_commands
            .iter()
            .flat_map(|claim| claim.surface_parity.iter())
            .any(|row| row.coverage_status == "browser_handoff_only"));
        assert!(registry
            .claimed_commands
            .iter()
            .flat_map(|claim| claim.surface_parity.iter())
            .any(|row| row.coverage_status == "not_surfaced_on_this_client"));
    }

    #[test]
    fn alpha_disabled_reason_manifest_covers_required_cause_families() {
        let payload = repo_file("fixtures/commands/disabled_reason_alpha/manifest.json");
        let manifest: AlphaDisabledReasonManifest =
            serde_json::from_str(&payload).expect("disabled reason manifest parses");
        assert_eq!(
            manifest.record_kind,
            "disabled_reason_alpha_manifest_record"
        );
        let observed = manifest
            .cases
            .iter()
            .map(|case| case.cause_family.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "focus",
            "selection",
            "lifecycle_state",
            "missing_dependency",
            "policy",
            "entitlement",
            "remote_or_host_mismatch",
        ] {
            assert!(observed.contains(required), "missing cause {required}");
        }
        for case in &manifest.cases {
            assert!(alpha_command_registry()
                .claim_for_command(&case.command_id)
                .is_some());
            assert!(!case.repair_hook_ref.hook_id.trim().is_empty());
        }
    }

    #[test]
    fn alpha_invocation_session_fixtures_parse_and_match_registry() {
        let registry = alpha_command_registry();
        for path in [
            "fixtures/commands/invocation_session_alpha/open_folder_success.invocation.json",
            "fixtures/commands/invocation_session_alpha/clone_repository_dependency_disabled.invocation.json",
            "fixtures/commands/invocation_session_alpha/restore_checkpoint_preview_failed.invocation.json",
        ] {
            let payload = repo_file(path);
            let raw: serde_json::Value = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("failed to parse raw fixture {path}: {err}"));
            let record: InvocationSessionPacketRecord =
                serde_json::from_str(&payload).unwrap_or_else(|err| {
                    panic!("failed to parse invocation fixture {path}: {err}")
                });
            assert_eq!(
                raw.pointer("/route_policy_source/policy_epoch")
                    .and_then(|value| value.as_str()),
                Some(record.policy_context.policy_epoch.as_str()),
                "fixture {path} must preserve route/policy source"
            );
            assert!(
                raw.pointer("/route_policy_source/route_class")
                    .and_then(|value| value.as_str())
                    .is_some_and(|route| !route.trim().is_empty()),
                "fixture {path} must preserve route class"
            );
            let claim = registry
                .claim_for_command(&record.command_id)
                .unwrap_or_else(|| panic!("fixture {path} references unknown command"));
            assert_eq!(record.command_revision_ref, claim.command_revision_ref);
            assert_eq!(
                record.preview_posture.preview_class_declared,
                claim.preview_class
            );
            assert_eq!(
                record.approval_posture.approval_posture_class_declared,
                claim.approval_posture_class
            );
        }

        let payload = repo_file(
            "fixtures/commands/invocation_session_alpha/import_profile_preview_success.result.json",
        );
        let result: CommandResultPacketRecord =
            serde_json::from_str(&payload).expect("result fixture parses");
        let claim = registry
            .claim_for_command(&result.invocation.canonical_command_id)
            .expect("result command exists");
        assert_eq!(
            result.invocation.command_revision_ref,
            claim.command_revision_ref
        );
        assert_eq!(
            result.invocation.preview_posture.preview_class_declared,
            claim.preview_class
        );
        assert_eq!(
            result
                .invocation
                .approval_posture
                .approval_posture_class_declared,
            claim.approval_posture_class
        );
        assert_eq!(result.result.outcome_code, "succeeded");
        assert!(result.no_bypass_guards.preview_path_preserved);
        assert!(result.no_bypass_guards.approval_path_preserved);
        assert!(result
            .result
            .checkpoint_refs
            .iter()
            .any(|checkpoint| checkpoint.checkpoint_ref.is_some()));
        assert_eq!(
            result.result.rollback_handle_ref.rollback_handle_posture,
            "available"
        );
    }

    #[test]
    fn alpha_parity_report_has_no_blocking_findings() {
        let payload = repo_file("artifacts/commands/alpha_command_parity_report.yaml");
        let report: AlphaParityReportRecord =
            serde_json::from_str(&payload).expect("parity report parses");
        assert_eq!(report.record_kind, "alpha_command_parity_report_record");
        assert_eq!(
            report.source_registry_ref,
            "artifacts/commands/alpha_command_registry.yaml"
        );
        assert_eq!(report.summary.unknown_high_risk_gaps, 0);
        assert_eq!(report.summary.blocking_findings, 0);
        assert_eq!(report.summary.status, "pass");
    }
}
