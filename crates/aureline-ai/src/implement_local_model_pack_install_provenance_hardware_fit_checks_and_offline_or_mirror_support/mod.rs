//! Local-model pack install, provenance, hardware-fit checks, and offline or
//! mirror support.
//!
//! This module materializes the local-model pack catalogue into one export-safe
//! truth packet whose unit of truth is a [`LocalModelPackRow`]: a single local
//! model pack that binds its publisher, model, version, install state,
//! provenance class, source channel, hardware-fit class, footprint tier, and
//! accelerator requirement into one inspectable record. The packet is the
//! canonical local-model-pack source for shell, docs, support export, and
//! release tooling; consumers project it instead of re-deriving install or
//! provenance state by hand.
//!
//! The packet refuses to present a pack as ready when its provenance or
//! hardware fit cannot back the claim. A pack that is installed or verified must
//! carry verified provenance — a signed, publisher-verified or key-pinned chain
//! — so an unsigned or unverified pack can never reach an active install state.
//! A pack whose provenance failed (a signature mismatch or a policy block) must
//! be quarantined rather than presented as installed. A pack that does not fit
//! the device (insufficient memory or disk, or an unsupported accelerator) may
//! not be presented as active, and a claimed pack may not hide its hardware fit
//! behind an unverified posture. Offline and mirror channels are held to the
//! same provenance bar as a direct download: a claimed pack sourced from a
//! mirror, offline bundle, air-gapped sideload, or local cache must still carry
//! verified provenance, so offline support never becomes a way to waive signing.
//! Every claimed pack carries a closed set of downgrade rules that narrow the
//! claim instead of hiding the pack, reusing the qualification and
//! downgrade-trigger vocabularies frozen by the M5 AI workflow matrix lane so no
//! pack row may stay greener than its evidence.
//!
//! Raw download URLs, mirror endpoints, credential bodies, raw signature blobs,
//! raw checksums, and exact byte sizes stay outside the support boundary; the
//! packet carries classes, tiers, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json`](../../../../schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support.md`](../../../../docs/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/`](../../../../fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::PROVIDER_MODEL_REGISTRY_SCHEMA_REF;

/// Stable record-kind tag carried by [`LocalModelPackInstallPacket`].
pub const LOCAL_MODEL_PACK_RECORD_KIND: &str =
    "implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support";

/// Schema version for local-model pack install records.
pub const LOCAL_MODEL_PACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const LOCAL_MODEL_PACK_SCHEMA_REF: &str =
    "schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json";

/// Repo-relative path of the local-model pack contract doc.
pub const LOCAL_MODEL_PACK_DOC_REF: &str =
    "docs/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support.md";

/// Repo-relative path of the protected fixture directory.
pub const LOCAL_MODEL_PACK_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support";

/// Repo-relative path of the checked support-export artifact.
pub const LOCAL_MODEL_PACK_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const LOCAL_MODEL_PACK_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support.md";

/// Install lifecycle state disclosed for one local-model pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalPackInstallState {
    /// The pack is catalogued but not present on the device.
    NotInstalled,
    /// The pack is downloading or importing.
    Acquiring,
    /// The pack is present and its provenance was checked.
    Installed,
    /// The pack is present, provenance-verified, and hardware-fit-confirmed.
    Verified,
    /// The pack is held aside after a provenance or fit failure.
    Quarantined,
    /// Installation failed and left no usable pack.
    InstallFailed,
    /// The pack was present and has been removed.
    Removed,
}

impl LocalPackInstallState {
    /// Every install state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotInstalled,
        Self::Acquiring,
        Self::Installed,
        Self::Verified,
        Self::Quarantined,
        Self::InstallFailed,
        Self::Removed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotInstalled => "not_installed",
            Self::Acquiring => "acquiring",
            Self::Installed => "installed",
            Self::Verified => "verified",
            Self::Quarantined => "quarantined",
            Self::InstallFailed => "install_failed",
            Self::Removed => "removed",
        }
    }

    /// Whether the pack is presented as present and usable on the device.
    pub const fn is_active_install(self) -> bool {
        matches!(self, Self::Installed | Self::Verified)
    }

    /// Whether the state holds the pack aside rather than presenting it.
    pub const fn is_held_aside(self) -> bool {
        matches!(
            self,
            Self::NotInstalled | Self::Quarantined | Self::InstallFailed | Self::Removed
        )
    }
}

/// Provenance posture proven for a pack's bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackProvenanceClass {
    /// Signed by a publisher whose identity is verified.
    SignedPublisherVerified,
    /// Signed with a pinned key trusted by the operator.
    SignedKeyPinned,
    /// Only a checksum matched; the pack is not signed.
    ChecksumOnlyUnsigned,
    /// No provenance has been established yet.
    UnverifiedSource,
    /// A signature was present but did not verify.
    SignatureMismatch,
    /// Provenance is blocked by policy.
    PolicyBlocked,
}

impl PackProvenanceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedPublisherVerified => "signed_publisher_verified",
            Self::SignedKeyPinned => "signed_key_pinned",
            Self::ChecksumOnlyUnsigned => "checksum_only_unsigned",
            Self::UnverifiedSource => "unverified_source",
            Self::SignatureMismatch => "signature_mismatch",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether the provenance chain is verified (signed and trusted).
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::SignedPublisherVerified | Self::SignedKeyPinned)
    }

    /// Whether the provenance check actively failed.
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::SignatureMismatch | Self::PolicyBlocked)
    }
}

/// Channel a pack's bytes were sourced from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSourceChannelClass {
    /// Downloaded directly from the vendor.
    DirectVendorDownload,
    /// Pulled from an operator-configured mirror.
    ConfiguredMirror,
    /// Imported from an offline bundle.
    OfflineBundleImport,
    /// Sideloaded into an air-gapped environment.
    AirGappedSideload,
    /// Reused from a verified local cache.
    LocalCacheReuse,
}

impl PackSourceChannelClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectVendorDownload => "direct_vendor_download",
            Self::ConfiguredMirror => "configured_mirror",
            Self::OfflineBundleImport => "offline_bundle_import",
            Self::AirGappedSideload => "air_gapped_sideload",
            Self::LocalCacheReuse => "local_cache_reuse",
        }
    }

    /// Whether this channel is an offline or mirror path rather than a direct
    /// vendor download.
    pub const fn is_offline_or_mirror(self) -> bool {
        matches!(
            self,
            Self::ConfiguredMirror
                | Self::OfflineBundleImport
                | Self::AirGappedSideload
                | Self::LocalCacheReuse
        )
    }
}

/// Result of checking a pack against the device's capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardwareFitClass {
    /// The device comfortably exceeds the pack's requirements.
    FitsComfortably,
    /// The device meets the requirements with little headroom.
    FitsConstrained,
    /// The device lacks the memory the pack requires.
    InsufficientMemory,
    /// The device lacks the disk the pack requires.
    InsufficientDisk,
    /// The device lacks a required accelerator.
    UnsupportedAccelerator,
    /// The fit has not been checked.
    UnknownUnverified,
}

impl HardwareFitClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitsComfortably => "fits_comfortably",
            Self::FitsConstrained => "fits_constrained",
            Self::InsufficientMemory => "insufficient_memory",
            Self::InsufficientDisk => "insufficient_disk",
            Self::UnsupportedAccelerator => "unsupported_accelerator",
            Self::UnknownUnverified => "unknown_unverified",
        }
    }

    /// Whether the pack actually fits the device.
    pub const fn fits(self) -> bool {
        matches!(self, Self::FitsComfortably | Self::FitsConstrained)
    }

    /// Whether the fit posture is concretely disclosed.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::UnknownUnverified)
    }
}

/// Disclosed on-device footprint tier for a pack.
///
/// Tiers stand in for exact byte sizes so the footprint is never hidden behind
/// generic language but no raw size crosses the support boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackFootprintTierClass {
    /// Smallest packs (roughly sub-gigabyte).
    Tiny,
    /// Small packs.
    Small,
    /// Mid-sized packs.
    Medium,
    /// Large packs.
    Large,
    /// Largest packs requiring substantial disk and memory.
    ExtraLarge,
}

impl PackFootprintTierClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra_large",
        }
    }
}

/// Accelerator a pack requires or can use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackAcceleratorClass {
    /// Runs on CPU only.
    CpuOnly,
    /// Uses a GPU if present, otherwise CPU.
    GpuOptional,
    /// Requires a GPU.
    GpuRequired,
    /// Uses an NPU if present, otherwise CPU.
    NpuOptional,
}

impl PackAcceleratorClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CpuOnly => "cpu_only",
            Self::GpuOptional => "gpu_optional",
            Self::GpuRequired => "gpu_required",
            Self::NpuOptional => "npu_optional",
        }
    }

    /// Whether the pack mandates a dedicated accelerator.
    pub const fn requires_accelerator(self) -> bool {
        matches!(self, Self::GpuRequired)
    }
}

/// One downgrade rule that narrows a pack's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalPackDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the pack narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One local-model pack row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalModelPackRow {
    /// Stable pack id.
    pub pack_id: String,
    /// Model identity token.
    pub model_id: String,
    /// Publisher identity token (no raw endpoint URL).
    pub publisher_id: String,
    /// Pack version token.
    pub pack_version: String,
    /// Qualification class claimed for this pack.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Install lifecycle state.
    pub install_state: LocalPackInstallState,
    /// Provenance posture for the pack's bytes.
    pub provenance: PackProvenanceClass,
    /// Channel the pack was sourced from.
    pub source_channel: PackSourceChannelClass,
    /// Hardware-fit posture against the device.
    pub hardware_fit: HardwareFitClass,
    /// Disclosed on-device footprint tier.
    pub footprint_tier: PackFootprintTierClass,
    /// Accelerator the pack requires or can use.
    pub accelerator: PackAcceleratorClass,
    /// Review-safe provenance label shown to users.
    pub provenance_label: String,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<LocalPackDowngradeRule>,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl LocalModelPackRow {
    /// Whether this pack carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Qualification this pack narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this pack.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Pack `{}`\n", self.pack_id));
        out.push_str(&format!("- Model: `{}`\n", self.model_id));
        out.push_str(&format!("- Publisher: `{}`\n", self.publisher_id));
        out.push_str(&format!("- Version: `{}`\n", self.pack_version));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!(
            "- Install state: `{}`\n",
            self.install_state.as_str()
        ));
        out.push_str(&format!(
            "- Provenance: `{}` ({})\n",
            self.provenance.as_str(),
            self.provenance_label
        ));
        out.push_str(&format!(
            "- Source channel: `{}`\n",
            self.source_channel.as_str()
        ));
        out.push_str(&format!(
            "- Hardware fit: `{}`\n",
            self.hardware_fit.as_str()
        ));
        out.push_str(&format!(
            "- Footprint: `{}`\n",
            self.footprint_tier.as_str()
        ));
        out.push_str(&format!("- Accelerator: `{}`\n", self.accelerator.as_str()));
        out
    }
}

/// Proof freshness block for the pack packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalModelPackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed packs.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`LocalModelPackInstallPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalModelPackInstallPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Local-model pack rows.
    pub packs: Vec<LocalModelPackRow>,
    /// Proof freshness block.
    pub proof_freshness: LocalModelPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe local-model pack install packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalModelPackInstallPacket {
    /// Record kind; must equal [`LOCAL_MODEL_PACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LOCAL_MODEL_PACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Local-model pack rows.
    pub packs: Vec<LocalModelPackRow>,
    /// Proof freshness block.
    pub proof_freshness: LocalModelPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl LocalModelPackInstallPacket {
    /// Builds a local-model pack install packet from stable-lane input.
    pub fn new(input: LocalModelPackInstallPacketInput) -> Self {
        Self {
            record_kind: LOCAL_MODEL_PACK_RECORD_KIND.to_owned(),
            schema_version: LOCAL_MODEL_PACK_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            packs: input.packs,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the local-model pack invariants.
    pub fn validate(&self) -> Vec<LocalModelPackViolation> {
        let mut violations = Vec::new();

        if self.record_kind != LOCAL_MODEL_PACK_RECORD_KIND {
            violations.push(LocalModelPackViolation::WrongRecordKind);
        }
        if self.schema_version != LOCAL_MODEL_PACK_SCHEMA_VERSION {
            violations.push(LocalModelPackViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(LocalModelPackViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_packs_present(self, &mut violations);
        for pack in &self.packs {
            validate_pack(pack, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("local-model pack packet serializes"),
        ) {
            violations.push(LocalModelPackViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of packs presented as actively installed (installed or verified).
    pub fn installed_pack_count(&self) -> usize {
        self.packs
            .iter()
            .filter(|pack| pack.install_state.is_active_install())
            .count()
    }

    /// Count of packs in the verified install state.
    pub fn verified_pack_count(&self) -> usize {
        self.packs
            .iter()
            .filter(|pack| pack.install_state == LocalPackInstallState::Verified)
            .count()
    }

    /// Count of packs sourced from an offline or mirror channel.
    pub fn offline_or_mirror_pack_count(&self) -> usize {
        self.packs
            .iter()
            .filter(|pack| pack.source_channel.is_offline_or_mirror())
            .count()
    }

    /// Count of packs held aside in quarantine.
    pub fn quarantined_pack_count(&self) -> usize {
        self.packs
            .iter()
            .filter(|pack| pack.install_state == LocalPackInstallState::Quarantined)
            .count()
    }

    /// Returns the pack row for `pack_id`, if present.
    pub fn pack(&self, pack_id: &str) -> Option<&LocalModelPackRow> {
        self.packs.iter().find(|pack| pack.pack_id == pack_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("local-model pack packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Local Model Pack Install And Provenance\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Packs: {} ({} installed, {} verified, {} offline/mirror, {} quarantined)\n",
            self.packs.len(),
            self.installed_pack_count(),
            self.verified_pack_count(),
            self.offline_or_mirror_pack_count(),
            self.quarantined_pack_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Pack inspectors\n\n");
        for pack in &self.packs {
            out.push_str(&pack.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in local-model pack export.
#[derive(Debug)]
pub enum LocalModelPackArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LocalModelPackViolation>),
}

impl fmt::Display for LocalModelPackArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "local-model pack export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "local-model pack export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for LocalModelPackArtifactError {}

/// Validation failures emitted by [`LocalModelPackInstallPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalModelPackViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no packs.
    NoPacks,
    /// A pack id appears more than once.
    DuplicatePack,
    /// A pack row is missing a required identity or label field.
    PackRowIncomplete,
    /// An active-install pack lacks verified provenance.
    ActiveInstallWithoutVerifiedProvenance,
    /// A pack whose provenance failed is not held aside.
    FailedProvenanceNotQuarantined,
    /// An active-install pack does not actually fit the device.
    ActiveInstallHardwareUnfit,
    /// A claimed pack hides its hardware fit behind an unverified posture.
    UndisclosedHardwareFit,
    /// A claimed offline or mirror pack lacks verified provenance.
    OfflineOrMirrorWithoutVerifiedProvenance,
    /// A claimed pack is missing required evidence packet refs.
    ClaimedPackMissingEvidence,
    /// A pack has no downgrade rules.
    DowngradeRulesMissing,
    /// A pack's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl LocalModelPackViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoPacks => "no_packs",
            Self::DuplicatePack => "duplicate_pack",
            Self::PackRowIncomplete => "pack_row_incomplete",
            Self::ActiveInstallWithoutVerifiedProvenance => {
                "active_install_without_verified_provenance"
            }
            Self::FailedProvenanceNotQuarantined => "failed_provenance_not_quarantined",
            Self::ActiveInstallHardwareUnfit => "active_install_hardware_unfit",
            Self::UndisclosedHardwareFit => "undisclosed_hardware_fit",
            Self::OfflineOrMirrorWithoutVerifiedProvenance => {
                "offline_or_mirror_without_verified_provenance"
            }
            Self::ClaimedPackMissingEvidence => "claimed_pack_missing_evidence",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in local-model pack export.
pub fn current_local_model_pack_install_export(
) -> Result<LocalModelPackInstallPacket, LocalModelPackArtifactError> {
    let packet: LocalModelPackInstallPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/support_export.json"
    )))
    .map_err(LocalModelPackArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LocalModelPackArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

fn validate_source_contracts(
    packet: &LocalModelPackInstallPacket,
    violations: &mut Vec<LocalModelPackViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        LOCAL_MODEL_PACK_SCHEMA_REF,
        LOCAL_MODEL_PACK_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(LocalModelPackViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_packs_present(
    packet: &LocalModelPackInstallPacket,
    violations: &mut Vec<LocalModelPackViolation>,
) {
    if packet.packs.is_empty() {
        violations.push(LocalModelPackViolation::NoPacks);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for pack in &packet.packs {
        if !seen.insert(pack.pack_id.as_str()) {
            violations.push(LocalModelPackViolation::DuplicatePack);
        }
    }
}

fn validate_pack(pack: &LocalModelPackRow, violations: &mut Vec<LocalModelPackViolation>) {
    if pack.pack_id.trim().is_empty()
        || pack.model_id.trim().is_empty()
        || pack.publisher_id.trim().is_empty()
        || pack.pack_version.trim().is_empty()
        || pack.provenance_label.trim().is_empty()
    {
        violations.push(LocalModelPackViolation::PackRowIncomplete);
    }

    if pack.install_state.is_active_install() && !pack.provenance.is_verified() {
        violations.push(LocalModelPackViolation::ActiveInstallWithoutVerifiedProvenance);
    }

    if pack.provenance.is_failed() && !pack.install_state.is_held_aside() {
        violations.push(LocalModelPackViolation::FailedProvenanceNotQuarantined);
    }

    if pack.install_state.is_active_install() && !pack.hardware_fit.fits() {
        violations.push(LocalModelPackViolation::ActiveInstallHardwareUnfit);
    }

    if pack.is_claimed() && !pack.hardware_fit.is_disclosed() {
        violations.push(LocalModelPackViolation::UndisclosedHardwareFit);
    }

    if pack.is_claimed()
        && pack.source_channel.is_offline_or_mirror()
        && !pack.provenance.is_verified()
    {
        violations.push(LocalModelPackViolation::OfflineOrMirrorWithoutVerifiedProvenance);
    }

    if pack.is_claimed() && pack.evidence_packet_refs.is_empty() {
        violations.push(LocalModelPackViolation::ClaimedPackMissingEvidence);
    }

    validate_downgrade_rules(pack, violations);
}

fn validate_downgrade_rules(
    pack: &LocalModelPackRow,
    violations: &mut Vec<LocalModelPackViolation>,
) {
    if pack.downgrade_rules.is_empty() {
        violations.push(LocalModelPackViolation::DowngradeRulesMissing);
        return;
    }

    if !pack
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(LocalModelPackViolation::DowngradeRuleMissingProofStale);
    }

    let claimed_rank = qualification_rank(pack.claimed_qualification);
    for rule in &pack.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(LocalModelPackViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &LocalModelPackInstallPacket,
    violations: &mut Vec<LocalModelPackViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(LocalModelPackViolation::ProofFreshnessIncomplete);
    }
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
                || lower.contains("https://")
                || lower.contains("http://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
