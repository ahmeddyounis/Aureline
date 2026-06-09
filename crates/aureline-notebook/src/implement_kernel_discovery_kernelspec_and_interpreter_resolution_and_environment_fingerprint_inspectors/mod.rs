//! Kernel discovery, kernelspec, interpreter resolution, and environment
//! fingerprint inspectors.
//!
//! This module materializes the typed discovery, resolution, and fingerprinting
//! layer that sits between the notebook document model and the kernel session
//! runtime. It produces [`Kernelspec`] records, [`InterpreterResolution`]
//! records, [`EnvironmentFingerprint`] records, and [`KernelDiscoveryEntry`]
//! candidates that the notebook header, kernel bar, and runtime truth consume.
//!
//! The module exposes:
//!
//! - the [`Kernelspec`] record that carries a stable kernelspec identity,
//!   display name, language label, and opaque refs to launch commands,
//!   resources, and metadata so raw paths never cross the boundary;
//! - the [`InterpreterResolution`] record that captures interpreter version,
//!   environment manager class, and source manifest provenance;
//! - the [`EnvironmentFingerprint`] record that communicates human-readable
//!   environment identity, package and toolchain summaries, target origin,
//!   policy epoch, and freshness state;
//! - the [`KernelDiscoveryEntry`] record that binds a discovered kernelspec to
//!   its discovery source, interpreter resolution, environment fingerprint, and
//!   compatibility / availability state;
//! - the [`KernelDiscoveryPacket`] checked-in artifact that downstream docs,
//!   help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`Kernelspec`] payloads.
pub const KERNELSPEC_RECORD_KIND: &str = "notebook_kernelspec";

/// Stable record-kind tag for serialized [`InterpreterResolution`] payloads.
pub const INTERPRETER_RESOLUTION_RECORD_KIND: &str = "notebook_interpreter_resolution";

/// Stable record-kind tag for serialized [`EnvironmentFingerprint`] payloads.
pub const ENVIRONMENT_FINGERPRINT_RECORD_KIND: &str = "notebook_environment_fingerprint";

/// Stable record-kind tag for serialized [`KernelDiscoveryEntry`] payloads.
pub const KERNEL_DISCOVERY_ENTRY_RECORD_KIND: &str = "notebook_kernel_discovery_entry";

/// Stable record-kind tag for the checked-in [`KernelDiscoveryPacket`].
pub const KERNEL_DISCOVERY_PACKET_RECORD_KIND: &str = "notebook_kernel_discovery_packet";

/// Repo-relative path to the checked-in kernel-discovery packet JSON.
pub const KERNEL_DISCOVERY_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.json";

/// Embedded checked-in kernel-discovery packet JSON.
pub const KERNEL_DISCOVERY_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.json"
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
    /// Where a kernelspec was discovered. Distinguishes local Jupyter data
    /// directories, Conda environments, virtual environments, system paths,
    /// remote registries, managed workspace pools, and remote-agent
    /// enumerations so the user always knows the provenance of a kernel
    /// candidate.
    KernelspecDiscoverySourceClass {
        JupyterDataDir => "jupyter_data_dir",
        CondaEnv => "conda_env",
        VirtualEnv => "virtual_env",
        SystemPath => "system_path",
        RemoteRegistry => "remote_registry",
        ManagedWorkspace => "managed_workspace",
        RemoteAgent => "remote_agent",
        BrowserBridge => "browser_bridge",
    }
);

impl KernelspecDiscoverySourceClass {
    /// True for any discovery source that is not on the local filesystem.
    pub const fn is_remote_source(self) -> bool {
        matches!(
            self,
            Self::RemoteRegistry | Self::ManagedWorkspace | Self::RemoteAgent | Self::BrowserBridge
        )
    }
}

closed_vocab!(
    /// Environment manager class that resolved the interpreter. Mirrors the
    /// manager vocabulary the runtime detector already uses so resolution
    /// records do not invent parallel terms.
    InterpreterManagerClass {
        Uv => "uv",
        Venv => "venv",
        Poetry => "poetry",
        Conda => "conda",
        Pyenv => "pyenv",
        System => "system",
        Unknown => "unknown",
    }
);

closed_vocab!(
    /// Freshness class for an environment fingerprint. Distinguishes current,
    /// stale, unresolved, and policy-blocked states so the fingerprint never
    /// silently claims reproducibility it cannot back.
    EnvironmentFingerprintFreshnessClass {
        Fresh => "fresh",
        Stale => "stale",
        Unresolved => "unresolved",
        PolicyBlocked => "policy_blocked",
    }
);

closed_vocab!(
    /// Compatibility class for a discovered kernel relative to a notebook.
    /// Distinguishes fully compatible kernels from language mismatches,
    /// version mismatches, policy-narrowed candidates, and unresolved
    /// dependency states.
    KernelDiscoveryCompatibilityClass {
        Compatible => "compatible",
        IncompatibleLanguage => "incompatible_language",
        IncompatibleVersion => "incompatible_version",
        PolicyNarrowed => "policy_narrowed",
        UnresolvedDependencies => "unresolved_dependencies",
    }
);

closed_vocab!(
    /// Availability class for a discovered kernel candidate. Distinguishes
    /// ready-to-launch kernels from starting, busy, unavailable, and
    /// policy-blocked states.
    KernelDiscoveryAvailabilityClass {
        Available => "available",
        Starting => "starting",
        Busy => "busy",
        Unavailable => "unavailable",
        PolicyBlocked => "policy_blocked",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelDiscoveryFinding {
    /// Stable check id (e.g. `kernelspec.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, kernelspec id, fingerprint id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl KernelDiscoveryFinding {
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

/// Typed validation finding for a [`Kernelspec`].
pub type KernelspecFinding = KernelDiscoveryFinding;

/// Typed validation finding for an [`InterpreterResolution`].
pub type InterpreterResolutionFinding = KernelDiscoveryFinding;

/// Typed validation finding for an [`EnvironmentFingerprint`].
pub type EnvironmentFingerprintFinding = KernelDiscoveryFinding;

/// Typed validation finding for a [`KernelDiscoveryEntry`].
pub type KernelDiscoveryEntryFinding = KernelDiscoveryFinding;

/// Typed validation finding for a [`KernelDiscoveryPacket`].
pub type KernelDiscoveryPacketFinding = KernelDiscoveryFinding;

/// Canonical kernelspec record. Carries human-readable identity and opaque
/// refs to launch commands, resources, and metadata so raw filesystem paths
/// never cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Kernelspec {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_kernel_discovery_schema_version: u32,
    /// Stable opaque kernelspec id.
    pub kernelspec_id: String,
    /// Human-readable display name (e.g. `Python 3.12`).
    pub display_name_label: String,
    /// Canonical language label (e.g. `python`, `r`, `julia`).
    pub language_label: String,
    /// Opaque ref to the resolved launch-command template.
    pub launch_command_template_ref: String,
    /// Opaque ref to the kernelspec resource directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_dir_ref: Option<String>,
    /// Opaque ref to extended kernelspec metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl Kernelspec {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<KernelspecFinding> {
        let mut findings = Vec::new();
        let subject = self.kernelspec_id.as_str();

        if self.record_kind != KERNELSPEC_RECORD_KIND {
            findings.push(KernelspecFinding::new(
                "kernelspec.record_kind",
                subject,
                format!(
                    "record_kind must be '{KERNELSPEC_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_kernel_discovery_schema_version != NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION
        {
            findings.push(KernelspecFinding::new(
                "kernelspec.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION}, found {}",
                    self.notebook_kernel_discovery_schema_version
                ),
            ));
        }

        if self.display_name_label.trim().is_empty() {
            findings.push(KernelspecFinding::new(
                "kernelspec.display_name_label",
                subject,
                "display_name_label must not be empty",
            ));
        }
        if self.language_label.trim().is_empty() {
            findings.push(KernelspecFinding::new(
                "kernelspec.language_label",
                subject,
                "language_label must not be empty",
            ));
        }
        if self.launch_command_template_ref.trim().is_empty() {
            findings.push(KernelspecFinding::new(
                "kernelspec.launch_command_template_ref",
                subject,
                "launch_command_template_ref must not be empty",
            ));
        }

        findings
    }
}

/// Canonical interpreter resolution record. Captures the interpreter version,
/// environment manager class, and source manifest provenance discovered for a
/// notebook kernel candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterpreterResolution {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_kernel_discovery_schema_version: u32,
    /// Stable opaque resolution id.
    pub resolution_id: String,
    /// Opaque VFS path-identity token for the interpreter executable.
    pub interpreter_path_token_ref: String,
    /// Human-readable version label (e.g. `3.12.4`).
    pub version_label: String,
    /// Canonical language label.
    pub language_label: String,
    /// Environment manager class that produced this resolution.
    pub manager_class: InterpreterManagerClass,
    /// Opaque ref to the source manifest that drove resolution
    /// (e.g. `pyproject.toml`, `.python-version`, `environment.yml`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_manifest_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl InterpreterResolution {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<InterpreterResolutionFinding> {
        let mut findings = Vec::new();
        let subject = self.resolution_id.as_str();

        if self.record_kind != INTERPRETER_RESOLUTION_RECORD_KIND {
            findings.push(InterpreterResolutionFinding::new(
                "interpreter_resolution.record_kind",
                subject,
                format!(
                    "record_kind must be '{INTERPRETER_RESOLUTION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_kernel_discovery_schema_version != NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION
        {
            findings.push(InterpreterResolutionFinding::new(
                "interpreter_resolution.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION}, found {}",
                    self.notebook_kernel_discovery_schema_version
                ),
            ));
        }

        if self.interpreter_path_token_ref.trim().is_empty() {
            findings.push(InterpreterResolutionFinding::new(
                "interpreter_resolution.interpreter_path_token_ref",
                subject,
                "interpreter_path_token_ref must not be empty",
            ));
        }
        if self.language_label.trim().is_empty() {
            findings.push(InterpreterResolutionFinding::new(
                "interpreter_resolution.language_label",
                subject,
                "language_label must not be empty",
            ));
        }

        findings
    }
}

/// Canonical environment fingerprint record. Communicates human-readable
/// environment identity, package and toolchain summaries, target origin,
/// policy epoch, and freshness so reproducibility claims remain inspectable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentFingerprint {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_kernel_discovery_schema_version: u32,
    /// Stable opaque fingerprint id.
    pub fingerprint_id: String,
    /// Human-readable environment identity label
    /// (e.g. `python3.12 + pandas2.3 + torch2.8`).
    pub environment_identity_label: String,
    /// Opaque ref to the [`InterpreterResolution`] that produced this
    /// fingerprint.
    pub interpreter_resolution_ref: String,
    /// Human-readable package summary label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_summary_label: Option<String>,
    /// Human-readable toolchain summary label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toolchain_summary_label: Option<String>,
    /// Human-readable target origin label (e.g. `local_host`,
    /// `managed_workspace:gpu-pool`).
    pub target_origin_label: String,
    /// Opaque ref to the policy epoch in force when the fingerprint was
    /// captured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
    /// Freshness class.
    pub freshness_class: EnvironmentFingerprintFreshnessClass,
    /// Last-known-good wall-clock timestamp; null when unresolved or never
    /// verified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_at: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl EnvironmentFingerprint {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<EnvironmentFingerprintFinding> {
        let mut findings = Vec::new();
        let subject = self.fingerprint_id.as_str();

        if self.record_kind != ENVIRONMENT_FINGERPRINT_RECORD_KIND {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.record_kind",
                subject,
                format!(
                    "record_kind must be '{ENVIRONMENT_FINGERPRINT_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_kernel_discovery_schema_version != NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION
        {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION}, found {}",
                    self.notebook_kernel_discovery_schema_version
                ),
            ));
        }

        if self.environment_identity_label.trim().is_empty() {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.environment_identity_label",
                subject,
                "environment_identity_label must not be empty",
            ));
        }
        if self.interpreter_resolution_ref.trim().is_empty() {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.interpreter_resolution_ref",
                subject,
                "interpreter_resolution_ref must not be empty",
            ));
        }
        if self.target_origin_label.trim().is_empty() {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.target_origin_label",
                subject,
                "target_origin_label must not be empty",
            ));
        }

        if matches!(
            self.freshness_class,
            EnvironmentFingerprintFreshnessClass::Fresh
        ) && self.last_known_good_at.is_none()
        {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.fresh_requires_last_known_good",
                subject,
                "fresh fingerprints must carry a last_known_good_at timestamp",
            ));
        }
        if matches!(
            self.freshness_class,
            EnvironmentFingerprintFreshnessClass::PolicyBlocked
        ) && self.policy_epoch_ref.is_none()
        {
            findings.push(EnvironmentFingerprintFinding::new(
                "environment_fingerprint.policy_blocked_requires_epoch",
                subject,
                "policy_blocked fingerprints must carry a policy_epoch_ref",
            ));
        }

        findings
    }
}

/// Canonical kernel discovery entry. Binds a discovered kernelspec to its
/// discovery source, interpreter resolution, environment fingerprint, and
/// compatibility / availability state so the kernel-selection UI can present
/// an honest, attributed list of candidates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelDiscoveryEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_kernel_discovery_schema_version: u32,
    /// Stable opaque discovery-entry id.
    pub entry_id: String,
    /// Discovered kernelspec record.
    pub kernelspec: Kernelspec,
    /// Where this kernelspec was discovered.
    pub discovery_source_class: KernelspecDiscoverySourceClass,
    /// Opaque ref to the [`InterpreterResolution`] for this candidate.
    pub interpreter_resolution_ref: String,
    /// Opaque ref to the [`EnvironmentFingerprint`] for this candidate.
    pub environment_fingerprint_ref: String,
    /// Compatibility class relative to the requesting notebook.
    pub compatibility_class: KernelDiscoveryCompatibilityClass,
    /// Availability class for this candidate.
    pub availability_class: KernelDiscoveryAvailabilityClass,
    /// Human-readable target origin label.
    pub target_origin_label: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl KernelDiscoveryEntry {
    /// Returns typed truth-rule findings; an empty vector means the entry is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<KernelDiscoveryEntryFinding> {
        let mut findings = Vec::new();
        let subject = self.entry_id.as_str();

        if self.record_kind != KERNEL_DISCOVERY_ENTRY_RECORD_KIND {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.record_kind",
                subject,
                format!(
                    "record_kind must be '{KERNEL_DISCOVERY_ENTRY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_kernel_discovery_schema_version != NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION
        {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION}, found {}",
                    self.notebook_kernel_discovery_schema_version
                ),
            ));
        }

        findings.extend(
            self.kernelspec
                .validate()
                .into_iter()
                .map(|f| KernelDiscoveryEntryFinding::new(&f.check_id, &f.subject_ref, &f.message)),
        );

        if self.interpreter_resolution_ref.trim().is_empty() {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.interpreter_resolution_ref",
                subject,
                "interpreter_resolution_ref must not be empty",
            ));
        }
        if self.environment_fingerprint_ref.trim().is_empty() {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.environment_fingerprint_ref",
                subject,
                "environment_fingerprint_ref must not be empty",
            ));
        }
        if self.target_origin_label.trim().is_empty() {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.target_origin_label",
                subject,
                "target_origin_label must not be empty",
            ));
        }

        if matches!(
            self.availability_class,
            KernelDiscoveryAvailabilityClass::PolicyBlocked
        ) && matches!(
            self.compatibility_class,
            KernelDiscoveryCompatibilityClass::Compatible
        ) {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.policy_blocked_compatible",
                subject,
                "policy_blocked availability is incompatible with compatible compatibility_class",
            ));
        }

        if self.discovery_source_class.is_remote_source()
            && self.target_origin_label == "local_host"
        {
            findings.push(KernelDiscoveryEntryFinding::new(
                "kernel_discovery_entry.remote_source_local_origin",
                subject,
                "remote discovery sources must not claim local_host target origin",
            ));
        }

        findings
    }
}

/// Checked-in kernel-discovery packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelDiscoveryPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: kernelspec discovery source classes.
    pub kernelspec_discovery_source_classes: Vec<KernelspecDiscoverySourceClass>,
    /// Closed vocabulary: interpreter manager classes.
    pub interpreter_manager_classes: Vec<InterpreterManagerClass>,
    /// Closed vocabulary: environment fingerprint freshness classes.
    pub environment_fingerprint_freshness_classes: Vec<EnvironmentFingerprintFreshnessClass>,
    /// Closed vocabulary: kernel discovery compatibility classes.
    pub kernel_discovery_compatibility_classes: Vec<KernelDiscoveryCompatibilityClass>,
    /// Closed vocabulary: kernel discovery availability classes.
    pub kernel_discovery_availability_classes: Vec<KernelDiscoveryAvailabilityClass>,
    /// Worked example kernelspecs.
    pub example_kernelspecs: Vec<Kernelspec>,
    /// Worked example interpreter resolutions.
    pub example_interpreter_resolutions: Vec<InterpreterResolution>,
    /// Worked example environment fingerprints.
    pub example_environment_fingerprints: Vec<EnvironmentFingerprint>,
    /// Worked example kernel discovery entries.
    pub example_kernel_discovery_entries: Vec<KernelDiscoveryEntry>,
    /// Export-safe summary line.
    pub summary: String,
}

impl KernelDiscoveryPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<KernelDiscoveryPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != KERNEL_DISCOVERY_PACKET_RECORD_KIND {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{KERNEL_DISCOVERY_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.kernelspec_discovery_source_classes.len()
            != KernelspecDiscoverySourceClass::ALL.len()
        {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.discovery_source_classes_coverage",
                subject,
                "kernelspec_discovery_source_classes must list every variant",
            ));
        }
        if self.interpreter_manager_classes.len() != InterpreterManagerClass::ALL.len() {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.interpreter_manager_classes_coverage",
                subject,
                "interpreter_manager_classes must list every variant",
            ));
        }
        if self.environment_fingerprint_freshness_classes.len()
            != EnvironmentFingerprintFreshnessClass::ALL.len()
        {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.freshness_classes_coverage",
                subject,
                "environment_fingerprint_freshness_classes must list every variant",
            ));
        }
        if self.kernel_discovery_compatibility_classes.len()
            != KernelDiscoveryCompatibilityClass::ALL.len()
        {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.compatibility_classes_coverage",
                subject,
                "kernel_discovery_compatibility_classes must list every variant",
            ));
        }
        if self.kernel_discovery_availability_classes.len()
            != KernelDiscoveryAvailabilityClass::ALL.len()
        {
            findings.push(KernelDiscoveryPacketFinding::new(
                "kernel_discovery_packet.availability_classes_coverage",
                subject,
                "kernel_discovery_availability_classes must list every variant",
            ));
        }

        for ks in &self.example_kernelspecs {
            findings.extend(ks.validate().into_iter().map(|f| {
                KernelDiscoveryPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for ir in &self.example_interpreter_resolutions {
            findings.extend(ir.validate().into_iter().map(|f| {
                KernelDiscoveryPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for ef in &self.example_environment_fingerprints {
            findings.extend(ef.validate().into_iter().map(|f| {
                KernelDiscoveryPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for entry in &self.example_kernel_discovery_entries {
            findings.extend(entry.validate().into_iter().map(|f| {
                KernelDiscoveryPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl KernelspecDiscoverySourceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::JupyterDataDir,
        Self::CondaEnv,
        Self::VirtualEnv,
        Self::SystemPath,
        Self::RemoteRegistry,
        Self::ManagedWorkspace,
        Self::RemoteAgent,
        Self::BrowserBridge,
    ];
}

impl InterpreterManagerClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Uv,
        Self::Venv,
        Self::Poetry,
        Self::Conda,
        Self::Pyenv,
        Self::System,
        Self::Unknown,
    ];
}

impl EnvironmentFingerprintFreshnessClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Fresh,
        Self::Stale,
        Self::Unresolved,
        Self::PolicyBlocked,
    ];
}

impl KernelDiscoveryCompatibilityClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Compatible,
        Self::IncompatibleLanguage,
        Self::IncompatibleVersion,
        Self::PolicyNarrowed,
        Self::UnresolvedDependencies,
    ];
}

impl KernelDiscoveryAvailabilityClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Available,
        Self::Starting,
        Self::Busy,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];
}

/// Parses the checked-in kernel-discovery packet JSON.
pub fn current_kernel_discovery_packet() -> Result<KernelDiscoveryPacket, serde_json::Error> {
    serde_json::from_str(KERNEL_DISCOVERY_PACKET_JSON)
}

#[cfg(test)]
mod tests;
