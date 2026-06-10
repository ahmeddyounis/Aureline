//! Experiment run identities, environment fingerprints, dataset cards, and
//! artifact lineage.
//!
//! This module materializes the typed experiment lineage layer that sits
//! between notebook execution and reproducibility claims. It produces
//! [`ExperimentRunIdentity`] records, [`ExperimentEnvironmentFingerprint`]
//! records, [`DatasetCard`] records, [`ArtifactLineage`] records, and the
//! [`ExperimentLineagePacket`] checked-in artifact that downstream docs,
//! help, support, and CI surfaces ingest instead of cloning status text.
//!
//! The records enforce local-first experiment traceability: run identity,
//! dataset provenance, artifact lineage, and environment fingerprint stay
//! inspectable and portable without requiring a hosted tracking product.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`ExperimentRunIdentity`] payloads.
pub const EXPERIMENT_RUN_IDENTITY_RECORD_KIND: &str = "notebook_experiment_run_identity";

/// Stable record-kind tag for serialized [`ExperimentEnvironmentFingerprint`]
/// payloads.
pub const EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND: &str =
    "notebook_experiment_environment_fingerprint";

/// Stable record-kind tag for serialized [`DatasetCard`] payloads.
pub const DATASET_CARD_RECORD_KIND: &str = "notebook_dataset_card";

/// Stable record-kind tag for serialized [`ArtifactLineage`] payloads.
pub const ARTIFACT_LINEAGE_RECORD_KIND: &str = "notebook_artifact_lineage";

/// Stable record-kind tag for the checked-in [`ExperimentLineagePacket`].
pub const EXPERIMENT_LINEAGE_PACKET_RECORD_KIND: &str = "notebook_experiment_lineage_packet";

/// Repo-relative path to the checked-in experiment-lineage packet JSON.
pub const EXPERIMENT_LINEAGE_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.json";

/// Embedded checked-in experiment-lineage packet JSON.
pub const EXPERIMENT_LINEAGE_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Outcome class for an experiment run. Distinguishes successful,
    /// failed, cancelled, partial, and policy-blocked outcomes so the run
    /// identity never silently claims reproducibility it cannot back.
    ExperimentRunOutcomeClass {
        Success => "success",
        Failure => "failure",
        Cancelled => "cancelled",
        Partial => "partial",
        PolicyBlocked => "policy_blocked",
    }
);

closed_vocab!(
    /// Freshness class for an experiment environment fingerprint.
    /// Distinguishes current, stale, unresolved, and policy-blocked states
    /// so the fingerprint never silently claims reproducibility it cannot
    /// back.
    ExperimentEnvironmentFingerprintFreshnessClass {
        Fresh => "fresh",
        Stale => "stale",
        Unresolved => "unresolved",
        PolicyBlocked => "policy_blocked",
    }
);

closed_vocab!(
    /// Source class for a dataset. Distinguishes local files, remote URLs,
    /// databases, API endpoints, versioned stores, generated outputs, and
    /// unknown sources so dataset provenance stays honest.
    DatasetSourceClass {
        LocalFile => "local_file",
        RemoteUrl => "remote_url",
        Database => "database",
        ApiEndpoint => "api_endpoint",
        VersionedStore => "versioned_store",
        Generated => "generated",
        Unknown => "unknown",
    }
);

closed_vocab!(
    /// Sensitivity and redaction class for a dataset card. Distinguishes
    /// public, internal, confidential, redacted-preview, and blocked states
    /// so sharing surfaces default to safe postures.
    DatasetSensitivityRedactionClass {
        Public => "public",
        Internal => "internal",
        Confidential => "confidential",
        RedactedPreview => "redacted_preview",
        Blocked => "blocked",
    }
);

closed_vocab!(
    /// Location class for a dataset. Distinguishes local workspace,
    /// remote storage, managed cache, and provider-only locations so the
    /// dataset card never implies data is accessible when it is not.
    DatasetLocationClass {
        LocalWorkspace => "local_workspace",
        RemoteStorage => "remote_storage",
        ManagedCache => "managed_cache",
        ProviderOnly => "provider_only",
    }
);

closed_vocab!(
    /// Save location class for an artifact. Distinguishes local workspace,
    /// remote storage, managed artifact store, and export buffer locations
    /// so artifact lineage stays honest about where artifacts live.
    ArtifactSaveLocationClass {
        LocalWorkspace => "local_workspace",
        RemoteStorage => "remote_storage",
        ManagedArtifactStore => "managed_artifact_store",
        ExportBuffer => "export_buffer",
    }
);

closed_vocab!(
    /// State class for an artifact lineage entry. Distinguishes current,
    /// stale, diverged, orphaned, and imported states so consumers know
    /// whether an artifact can be trusted as originating from the claimed
    /// run.
    ArtifactLineageStateClass {
        Current => "current",
        Stale => "stale",
        Diverged => "diverged",
        Orphaned => "orphaned",
        Imported => "imported",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentLineageFinding {
    /// Stable check id (e.g. `experiment_run_identity.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, run id, fingerprint id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl ExperimentLineageFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for an [`ExperimentRunIdentity`].
pub type ExperimentRunIdentityFinding = ExperimentLineageFinding;

/// Typed validation finding for an [`ExperimentEnvironmentFingerprint`].
pub type ExperimentEnvironmentFingerprintFinding = ExperimentLineageFinding;

/// Typed validation finding for a [`DatasetCard`].
pub type DatasetCardFinding = ExperimentLineageFinding;

/// Typed validation finding for an [`ArtifactLineage`].
pub type ArtifactLineageFinding = ExperimentLineageFinding;

/// Typed validation finding for an [`ExperimentLineagePacket`].
pub type ExperimentLineagePacketFinding = ExperimentLineageFinding;

/// Canonical experiment run identity record. Carries human-readable run
/// identity, source reference, outcome, revision provenance, execution
/// origin, and environment fingerprint reference so experiment rows stay
/// traceable without a hosted tracker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentRunIdentity {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_experiment_lineage_schema_version: u32,
    /// Stable opaque run id.
    pub run_id: String,
    /// Human-readable run title.
    pub title: String,
    /// Opaque ref to the originating notebook, script, task, or test.
    pub source_ref: String,
    /// UTC start timestamp.
    pub started_at: String,
    /// UTC end timestamp when the run is terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Outcome class for the run.
    pub outcome_class: ExperimentRunOutcomeClass,
    /// Opaque ref to the commit or workspace revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_or_revision_ref: Option<String>,
    /// Human-readable execution origin label (e.g. `local_host`,
    /// `managed_workspace:gpu-pool`).
    pub execution_origin_label: String,
    /// Opaque ref to the [`ExperimentEnvironmentFingerprint`] for this run.
    pub environment_fingerprint_ref: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl ExperimentRunIdentity {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<ExperimentRunIdentityFinding> {
        let mut findings = Vec::new();
        let subject = self.run_id.as_str();

        if self.record_kind != EXPERIMENT_RUN_IDENTITY_RECORD_KIND {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.record_kind",
                subject,
                format!(
                    "record_kind must be '{EXPERIMENT_RUN_IDENTITY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_experiment_lineage_schema_version != NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION
        {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION}, found {}",
                    self.notebook_experiment_lineage_schema_version
                ),
            ));
        }

        if self.title.trim().is_empty() {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.title",
                subject,
                "title must not be empty",
            ));
        }
        if self.source_ref.trim().is_empty() {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.source_ref",
                subject,
                "source_ref must not be empty",
            ));
        }
        if self.execution_origin_label.trim().is_empty() {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.execution_origin_label",
                subject,
                "execution_origin_label must not be empty",
            ));
        }
        if self.environment_fingerprint_ref.trim().is_empty() {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.environment_fingerprint_ref",
                subject,
                "environment_fingerprint_ref must not be empty",
            ));
        }

        if matches!(self.outcome_class, ExperimentRunOutcomeClass::Success)
            && self.ended_at.is_none()
        {
            findings.push(ExperimentRunIdentityFinding::new(
                "experiment_run_identity.success_requires_ended_at",
                subject,
                "success outcome requires an ended_at timestamp",
            ));
        }

        findings
    }
}

/// Canonical experiment environment fingerprint record. Communicates
/// human-readable environment identity, interpreter or kernel label,
/// package and toolchain summary, target origin, policy epoch, and
/// freshness so reproducibility claims remain inspectable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentEnvironmentFingerprint {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_experiment_lineage_schema_version: u32,
    /// Stable opaque fingerprint id.
    pub fingerprint_id: String,
    /// Human-readable environment identity label
    /// (e.g. `python3.12 + pandas2.3 + torch2.8`).
    pub environment_identity_label: String,
    /// Human-readable interpreter or kernel label.
    pub interpreter_kernel_label: String,
    /// Human-readable package and toolchain summary label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_toolchain_summary: Option<String>,
    /// Human-readable target origin label.
    pub target_origin_label: String,
    /// Opaque ref to the policy epoch in force when the fingerprint was
    /// captured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
    /// Freshness class.
    pub freshness_class: ExperimentEnvironmentFingerprintFreshnessClass,
    /// Last-known-good wall-clock timestamp; null when unresolved or never
    /// verified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_at: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl ExperimentEnvironmentFingerprint {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<ExperimentEnvironmentFingerprintFinding> {
        let mut findings = Vec::new();
        let subject = self.fingerprint_id.as_str();

        if self.record_kind != EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.record_kind",
                subject,
                format!(
                    "record_kind must be '{EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_experiment_lineage_schema_version != NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION
        {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION}, found {}",
                    self.notebook_experiment_lineage_schema_version
                ),
            ));
        }

        if self.environment_identity_label.trim().is_empty() {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.environment_identity_label",
                subject,
                "environment_identity_label must not be empty",
            ));
        }
        if self.interpreter_kernel_label.trim().is_empty() {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.interpreter_kernel_label",
                subject,
                "interpreter_kernel_label must not be empty",
            ));
        }
        if self.target_origin_label.trim().is_empty() {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.target_origin_label",
                subject,
                "target_origin_label must not be empty",
            ));
        }

        if matches!(self.freshness_class, ExperimentEnvironmentFingerprintFreshnessClass::Fresh)
            && self.last_known_good_at.is_none()
        {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.fresh_requires_last_known_good",
                subject,
                "fresh fingerprints must carry a last_known_good_at timestamp",
            ));
        }
        if matches!(
            self.freshness_class,
            ExperimentEnvironmentFingerprintFreshnessClass::PolicyBlocked
        ) && self.policy_epoch_ref.is_none()
        {
            findings.push(ExperimentEnvironmentFingerprintFinding::new(
                "experiment_environment_fingerprint.policy_blocked_requires_epoch",
                subject,
                "policy_blocked fingerprints must carry a policy_epoch_ref",
            ));
        }

        findings
    }
}

/// Canonical dataset card record. Carries dataset identity, source class,
/// version or snapshot note, size estimate, sensitivity and redaction
/// state, and location class so dataset provenance stays honest and
/// sharing surfaces default to metadata-only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetCard {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_experiment_lineage_schema_version: u32,
    /// Stable opaque dataset id.
    pub dataset_id: String,
    /// Human-readable dataset label.
    pub dataset_label: String,
    /// Source class for the dataset.
    pub source_class: DatasetSourceClass,
    /// Version or snapshot label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_version_label: Option<String>,
    /// Human-readable size estimate label (e.g. `1.2M rows`, `4.5 GB`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_estimate_label: Option<String>,
    /// Sensitivity and redaction class.
    pub sensitivity_redaction_class: DatasetSensitivityRedactionClass,
    /// Location class for the dataset.
    pub location_class: DatasetLocationClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl DatasetCard {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<DatasetCardFinding> {
        let mut findings = Vec::new();
        let subject = self.dataset_id.as_str();

        if self.record_kind != DATASET_CARD_RECORD_KIND {
            findings.push(DatasetCardFinding::new(
                "dataset_card.record_kind",
                subject,
                format!(
                    "record_kind must be '{DATASET_CARD_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_experiment_lineage_schema_version != NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION
        {
            findings.push(DatasetCardFinding::new(
                "dataset_card.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION}, found {}",
                    self.notebook_experiment_lineage_schema_version
                ),
            ));
        }

        if self.dataset_label.trim().is_empty() {
            findings.push(DatasetCardFinding::new(
                "dataset_card.dataset_label",
                subject,
                "dataset_label must not be empty",
            ));
        }

        if matches!(
            self.sensitivity_redaction_class,
            DatasetSensitivityRedactionClass::RedactedPreview | DatasetSensitivityRedactionClass::Blocked
        ) && self.size_estimate_label.is_none()
        {
            findings.push(DatasetCardFinding::new(
                "dataset_card.redacted_requires_size_estimate",
                subject,
                "redacted_preview or blocked datasets must carry a size_estimate_label",
            ));
        }

        findings
    }
}

/// Canonical artifact lineage record. Carries artifact identity, producing
/// run reference, generator step label, environment fingerprint reference,
/// save location class, and lineage state so consumers know whether an
/// artifact can be trusted as originating from the claimed run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLineage {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_experiment_lineage_schema_version: u32,
    /// Stable opaque artifact id.
    pub artifact_id: String,
    /// Opaque ref to the producing [`ExperimentRunIdentity`].
    pub producing_run_ref: String,
    /// Human-readable generator step label (e.g. `cell_3_output`,
    /// `model_export_step_7`).
    pub generator_step_label: String,
    /// Opaque ref to the [`ExperimentEnvironmentFingerprint`] relevant to
    /// the artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_fingerprint_ref: Option<String>,
    /// Save location class for the artifact.
    pub save_location_class: ArtifactSaveLocationClass,
    /// Lineage state class.
    pub lineage_state_class: ArtifactLineageStateClass,
    /// Human-readable stale or diverged note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_diverged_note: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl ArtifactLineage {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<ArtifactLineageFinding> {
        let mut findings = Vec::new();
        let subject = self.artifact_id.as_str();

        if self.record_kind != ARTIFACT_LINEAGE_RECORD_KIND {
            findings.push(ArtifactLineageFinding::new(
                "artifact_lineage.record_kind",
                subject,
                format!(
                    "record_kind must be '{ARTIFACT_LINEAGE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_experiment_lineage_schema_version != NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION
        {
            findings.push(ArtifactLineageFinding::new(
                "artifact_lineage.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION}, found {}",
                    self.notebook_experiment_lineage_schema_version
                ),
            ));
        }

        if self.producing_run_ref.trim().is_empty() {
            findings.push(ArtifactLineageFinding::new(
                "artifact_lineage.producing_run_ref",
                subject,
                "producing_run_ref must not be empty",
            ));
        }
        if self.generator_step_label.trim().is_empty() {
            findings.push(ArtifactLineageFinding::new(
                "artifact_lineage.generator_step_label",
                subject,
                "generator_step_label must not be empty",
            ));
        }

        if matches!(
            self.lineage_state_class,
            ArtifactLineageStateClass::Stale | ArtifactLineageStateClass::Diverged
        ) && self.stale_diverged_note.is_none()
        {
            findings.push(ArtifactLineageFinding::new(
                "artifact_lineage.stale_diverged_requires_note",
                subject,
                "stale or diverged lineage must carry a stale_diverged_note",
            ));
        }

        if matches!(self.lineage_state_class, ArtifactLineageStateClass::Orphaned)
            && self.producing_run_ref != "orphaned"
        {
            findings.push(ArtifactLineageFinding::new(
                "artifact_lineage.orphaned_run_ref",
                subject,
                "orphaned lineage must set producing_run_ref to 'orphaned'",
            ));
        }

        findings
    }
}

/// Checked-in experiment-lineage packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentLineagePacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: experiment run outcome classes.
    pub experiment_run_outcome_classes: Vec<ExperimentRunOutcomeClass>,
    /// Closed vocabulary: experiment environment fingerprint freshness classes.
    pub experiment_environment_fingerprint_freshness_classes: Vec<ExperimentEnvironmentFingerprintFreshnessClass>,
    /// Closed vocabulary: dataset source classes.
    pub dataset_source_classes: Vec<DatasetSourceClass>,
    /// Closed vocabulary: dataset sensitivity and redaction classes.
    pub dataset_sensitivity_redaction_classes: Vec<DatasetSensitivityRedactionClass>,
    /// Closed vocabulary: dataset location classes.
    pub dataset_location_classes: Vec<DatasetLocationClass>,
    /// Closed vocabulary: artifact save location classes.
    pub artifact_save_location_classes: Vec<ArtifactSaveLocationClass>,
    /// Closed vocabulary: artifact lineage state classes.
    pub artifact_lineage_state_classes: Vec<ArtifactLineageStateClass>,
    /// Worked example experiment run identities.
    pub example_experiment_run_identities: Vec<ExperimentRunIdentity>,
    /// Worked example experiment environment fingerprints.
    pub example_experiment_environment_fingerprints: Vec<ExperimentEnvironmentFingerprint>,
    /// Worked example dataset cards.
    pub example_dataset_cards: Vec<DatasetCard>,
    /// Worked example artifact lineages.
    pub example_artifact_lineages: Vec<ArtifactLineage>,
    /// Export-safe summary line.
    pub summary: String,
}

impl ExperimentLineagePacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ExperimentLineagePacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != EXPERIMENT_LINEAGE_PACKET_RECORD_KIND {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{EXPERIMENT_LINEAGE_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.experiment_run_outcome_classes.len() != ExperimentRunOutcomeClass::ALL.len() {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.outcome_classes_coverage",
                subject,
                "experiment_run_outcome_classes must list every variant",
            ));
        }
        if self.experiment_environment_fingerprint_freshness_classes.len()
            != ExperimentEnvironmentFingerprintFreshnessClass::ALL.len()
        {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.freshness_classes_coverage",
                subject,
                "experiment_environment_fingerprint_freshness_classes must list every variant",
            ));
        }
        if self.dataset_source_classes.len() != DatasetSourceClass::ALL.len() {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.dataset_source_classes_coverage",
                subject,
                "dataset_source_classes must list every variant",
            ));
        }
        if self.dataset_sensitivity_redaction_classes.len()
            != DatasetSensitivityRedactionClass::ALL.len()
        {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.sensitivity_classes_coverage",
                subject,
                "dataset_sensitivity_redaction_classes must list every variant",
            ));
        }
        if self.dataset_location_classes.len() != DatasetLocationClass::ALL.len() {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.location_classes_coverage",
                subject,
                "dataset_location_classes must list every variant",
            ));
        }
        if self.artifact_save_location_classes.len() != ArtifactSaveLocationClass::ALL.len() {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.save_location_classes_coverage",
                subject,
                "artifact_save_location_classes must list every variant",
            ));
        }
        if self.artifact_lineage_state_classes.len() != ArtifactLineageStateClass::ALL.len() {
            findings.push(ExperimentLineagePacketFinding::new(
                "experiment_lineage_packet.lineage_state_classes_coverage",
                subject,
                "artifact_lineage_state_classes must list every variant",
            ));
        }

        for run in &self.example_experiment_run_identities {
            findings.extend(run.validate().into_iter().map(|f| {
                ExperimentLineagePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for fp in &self.example_experiment_environment_fingerprints {
            findings.extend(fp.validate().into_iter().map(|f| {
                ExperimentLineagePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for ds in &self.example_dataset_cards {
            findings.extend(ds.validate().into_iter().map(|f| {
                ExperimentLineagePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for art in &self.example_artifact_lineages {
            findings.extend(art.validate().into_iter().map(|f| {
                ExperimentLineagePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl ExperimentRunOutcomeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Success,
        Self::Failure,
        Self::Cancelled,
        Self::Partial,
        Self::PolicyBlocked,
    ];
}

impl ExperimentEnvironmentFingerprintFreshnessClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Fresh,
        Self::Stale,
        Self::Unresolved,
        Self::PolicyBlocked,
    ];
}

impl DatasetSourceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalFile,
        Self::RemoteUrl,
        Self::Database,
        Self::ApiEndpoint,
        Self::VersionedStore,
        Self::Generated,
        Self::Unknown,
    ];
}

impl DatasetSensitivityRedactionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Public,
        Self::Internal,
        Self::Confidential,
        Self::RedactedPreview,
        Self::Blocked,
    ];
}

impl DatasetLocationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalWorkspace,
        Self::RemoteStorage,
        Self::ManagedCache,
        Self::ProviderOnly,
    ];
}

impl ArtifactSaveLocationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalWorkspace,
        Self::RemoteStorage,
        Self::ManagedArtifactStore,
        Self::ExportBuffer,
    ];
}

impl ArtifactLineageStateClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Stale,
        Self::Diverged,
        Self::Orphaned,
        Self::Imported,
    ];
}

/// Parses the checked-in experiment-lineage packet JSON.
pub fn current_experiment_lineage_packet() -> Result<ExperimentLineagePacket, serde_json::Error> {
    serde_json::from_str(EXPERIMENT_LINEAGE_PACKET_JSON)
}

#[cfg(test)]
mod tests;
