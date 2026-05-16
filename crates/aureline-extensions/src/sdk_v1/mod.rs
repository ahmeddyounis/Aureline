//! SDK v1 beta typed APIs, manifest authoring guides, and canonical
//! sample-extension pack.
//!
//! This module gives extension authors one inspectable starting point for
//! building, validating, publishing, and recovering an extension against
//! the M3 beta admission lane. It joins three previously-separate truth
//! sources into one record set:
//!
//! - **Typed API surfaces.** One [`SdkV1ApiSurfaceRecord`] per published
//!   SDK v1 API surface (Wasm component-model, Wasm core-module,
//!   external-host supervised, helper-binary, compatibility-bridge,
//!   remote-side-component). Each surface pins its host contract family,
//!   the WIT world refs / external host contract ref it covers, and its
//!   availability under the M3 beta lane.
//! - **Manifest authoring guides.** One
//!   [`SdkV1ManifestAuthoringGuideRecord`] per authored walkthrough that
//!   pairs a typed [`SdkV1ManifestGuideClass`] with the surfaces it
//!   covers and a non-empty repair-affordance label.
//! - **Canonical sample-extension pack.** One
//!   [`SamplePackExtensionRecord`] per checked-in sample whose typed
//!   [`SamplePackEntryClass`] and [`SamplePackValidationClass`] make the
//!   wasm and external-host claims real rather than implied. Every
//!   sample carries an opaque ref to its manifest baseline, permission
//!   manifest, runtime contract, and SDK release-bundle ref so the same
//!   row reviewers see in install / review chrome is the row authors
//!   build against.
//!
//! [`evaluate_sdk_v1_starter_pack`] combines the three sources into one
//! [`SdkV1StarterPackRecord`] under a closed
//! [`SdkV1StarterPackDecisionClass`] / [`SdkV1StarterPackReasonClass`]
//! pair, and refuses the pack closed when a claimed wasm or external-host
//! lane has no validated sample, when an authoring guide is missing for a
//! claimed surface, when sample / starter-pack identity is inconsistent,
//! or when the redaction class drifts off
//! [`RedactionClass::MetadataSafeDefault`].
//!
//! [`project_sdk_v1_starter_pack_support_export`] is the first consuming
//! surface — a metadata-safe support / partner export that quotes the
//! same closed tokens plus a `blocks_authoring` flag the install / review
//! chrome, partner packet template, and CLI / headless review lanes read
//! verbatim. Raw SDK release bytes, raw signing-key material, raw sample
//! source bytes, raw paths, raw tokens, and raw publisher-private data
//! MUST NOT appear; every field is an opaque ref or a closed vocabulary
//! value.
//!
//! Reviewer landing page:
//! [`/docs/extensions/m3/sdk_v1/`](../../../../docs/extensions/m3/sdk_v1/).
//! Checked-in fixtures:
//! [`/fixtures/extensions/m3/sample_pack/`](../../../../fixtures/extensions/m3/sample_pack/).
//! Boundary schema:
//! [`/schemas/extensions/sdk_v1_starter_pack.schema.json`](../../../../schemas/extensions/sdk_v1_starter_pack.schema.json).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::{HostContractFamilyClass, RedactionClass};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`SdkV1ApiSurfaceRecord`] payloads.
pub const SDK_V1_API_SURFACE_RECORD_KIND: &str = "sdk_v1_api_surface_record";

/// Record-kind tag carried on serialized
/// [`SdkV1ManifestAuthoringGuideRecord`] payloads.
pub const SDK_V1_MANIFEST_AUTHORING_GUIDE_RECORD_KIND: &str =
    "sdk_v1_manifest_authoring_guide_record";

/// Record-kind tag carried on serialized [`SamplePackExtensionRecord`] payloads.
pub const SAMPLE_PACK_EXTENSION_RECORD_KIND: &str = "sample_pack_extension_record";

/// Record-kind tag carried on serialized [`SdkV1StarterPackRecord`] payloads.
pub const SDK_V1_STARTER_PACK_RECORD_KIND: &str = "sdk_v1_starter_pack_record";

/// Record-kind tag carried on serialized
/// [`SdkV1StarterPackSupportExportRecord`] payloads.
pub const SDK_V1_STARTER_PACK_SUPPORT_EXPORT_RECORD_KIND: &str =
    "sdk_v1_starter_pack_support_export_record";

/// Schema version for the SDK v1 starter-pack payloads.
///
/// Bumped on breaking payload changes. Additive enum members or
/// optional fields are additive-minor and require consumers to keep
/// unknown-field preservation at their boundary.
pub const SDK_V1_STARTER_PACK_SCHEMA_VERSION: u32 = 1;

/// Closed SDK v1 typed-API surface vocabulary.
///
/// Mirrors the host-contract-family vocabulary from
/// [`HostContractFamilyClass`]: every API surface published under the
/// SDK v1 beta lane resolves to exactly one host contract family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdkV1ApiSurfaceClass {
    WasmComponentModelHostApi,
    WasmCoreModuleHostApi,
    ExternalHostSupervisedApi,
    HelperBinaryApi,
    CompatibilityBridgeApi,
    RemoteSideComponentApi,
}

/// Closed availability vocabulary for an SDK v1 API surface under the
/// M3 beta lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdkV1ApiAvailabilityClass {
    /// Surface is available under the M3 beta lane and authors can
    /// build against it without private internal hooks.
    AvailableInBeta,
    /// Surface is exposed under the M3 beta lane but documented as
    /// preview; authors MUST NOT ship a row that is not also covered
    /// by a passing conformance result.
    PreviewInBeta,
    /// Surface is not available until the M3 general lane; the
    /// starter pack lists it for transparency but the install /
    /// review chrome MUST NOT admit authors against it.
    NotAvailableUntilGeneral,
    /// Surface is retired pending a successor. Authors MUST migrate
    /// before the retirement window closes.
    RetiredPendingSuccessor,
}

/// Closed sample-pack entry vocabulary covering the canonical sample
/// patterns the M3 beta lane claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SamplePackEntryClass {
    /// Minimal Wasm component-model sample: one declared scope, no
    /// capability negotiation.
    WasmComponentMinimal,
    /// Wasm component-model sample exercising capability-world
    /// negotiation and narrowing.
    WasmComponentCapabilityNegotiated,
    /// Minimal external-host supervised sample: one external host
    /// process, kept within a single capability world.
    ExternalHostSupervisedMinimal,
    /// External-host supervised sample exercising capability-world
    /// negotiation, narrowing, and a recorded supervision posture.
    ExternalHostSupervisedCapabilityNegotiated,
    /// Short-lived helper-binary sample.
    HelperBinaryShortLived,
    /// A documentation-walkthrough row that the authoring guide
    /// references. Does not ship a runnable binary.
    ManifestAuthoringWalkthrough,
}

/// Closed sample-pack validation vocabulary mirroring the SDK
/// publication contract's `sample_validation_state` shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SamplePackValidationClass {
    /// Sample MUST compile under CI; no runtime validation.
    MustCompileInCi,
    /// Sample MUST validate under CI through the sample-validator kit
    /// (compiles + manifest validates + permission diff stable).
    MustValidateInCi,
    /// Sample MUST run under CI against a reference host and produce
    /// a passing conformance-result row.
    MustRunInCi,
    /// Sample is documentation-only and not validated as a runnable
    /// row. Reserved for [`SamplePackEntryClass::ManifestAuthoringWalkthrough`].
    DocumentationOnly,
}

/// Closed manifest-authoring guide vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdkV1ManifestGuideClass {
    /// Walkthrough for declaring permission scopes and rationale
    /// labels against the manifest baseline.
    PermissionDeclarationWalkthrough,
    /// Walkthrough for picking a host contract family and proving the
    /// claim against the runtime v1 beta contract.
    HostContractFamilyWalkthrough,
    /// Walkthrough for binding to a verified publisher identity.
    PublisherIdentityWalkthrough,
    /// Walkthrough for declaring activation / runtime budgets and the
    /// degraded-state class the row claims.
    RuntimeBudgetWalkthrough,
    /// Walkthrough for the update / rollback contract: how to ship a
    /// new version, how to be revoked, how to recover.
    UpdateAndRollbackWalkthrough,
    /// Walkthrough for the SDK publication contract: build identity,
    /// mirror availability, support window, badge.
    SdkPublicationWalkthrough,
}

/// Closed SDK v1 starter-pack decision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdkV1StarterPackDecisionClass {
    /// All claimed surfaces are available in beta, every claimed wasm /
    /// external-host lane has at least one validated sample, and every
    /// claimed surface has at least one authoring guide.
    ReadyForAuthors,
    /// At least one claimed surface is preview-only or not-available;
    /// the pack is still inspectable but the install / review chrome
    /// MUST disclose the preview posture verbatim.
    PartiallyReadyPreviewSurfacesOnly,
    /// The starter pack is refused for a structural / identity /
    /// completeness reason. Install / review chrome MUST hold and MUST
    /// NOT advertise the pack to authors.
    RefusedInconsistentInput,
}

/// Closed SDK v1 starter-pack reason class paired with
/// [`SdkV1StarterPackDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdkV1StarterPackReasonClass {
    AllClaimedSurfacesAvailableInBeta,
    SomeClaimedSurfacesPreviewInBeta,
    RefusedPackIdUnprefixed,
    RefusedSdkLineRefMissing,
    RefusedClaimedSurfacesEmpty,
    RefusedNoWasmSampleForClaimedWasmLane,
    RefusedNoExternalHostSampleForClaimedExternalHostLane,
    RefusedAuthoringGuideMissingForClaimedSurface,
    RefusedSampleValidationDocumentationOnlyOnRunnableEntry,
    RefusedSampleHostFamilyDisagreesWithApiSurface,
    RefusedSampleManifestRefUnprefixed,
    RefusedSamplePermissionManifestRefUnprefixed,
    RefusedSampleRuntimeContractRefUnprefixed,
    RefusedSurfaceAvailabilityRetired,
    RefusedRedactionClassNotMetadataSafe,
}

/// One inspectable SDK v1 typed-API surface row.
///
/// A surface row pins the host contract family the API targets, the WIT
/// world refs / external host contract ref it covers, and the typed
/// availability under the M3 beta lane. SDK release-bundle and
/// conformance refs come in by opaque ref so the SDK publication
/// contract remains the source of truth for build identity and badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkV1ApiSurfaceRecord {
    pub record_kind: String,
    pub sdk_v1_starter_pack_schema_version: u32,
    pub api_surface_id: String,
    pub api_surface_class: SdkV1ApiSurfaceClass,
    pub host_contract_family_class: HostContractFamilyClass,
    pub availability_class: SdkV1ApiAvailabilityClass,
    pub sdk_line_id: String,
    pub sdk_line_semver: String,
    pub sdk_release_bundle_ref: String,
    pub covered_wit_world_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_host_contract_ref: Option<String>,
    pub surface_summary_label: String,
    pub redaction_class: RedactionClass,
}

/// One inspectable manifest-authoring guide row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkV1ManifestAuthoringGuideRecord {
    pub record_kind: String,
    pub sdk_v1_starter_pack_schema_version: u32,
    pub manifest_guide_id: String,
    pub guide_class: SdkV1ManifestGuideClass,
    pub covered_api_surface_classes: Vec<SdkV1ApiSurfaceClass>,
    pub guide_summary_label: String,
    pub repair_affordance_label: String,
    pub doc_path_label: String,
    pub redaction_class: RedactionClass,
}

/// One inspectable sample-extension row.
///
/// A sample row pins its category, its host contract family, its
/// validation class, and opaque refs to the manifest baseline,
/// permission manifest, runtime contract, and SDK release-bundle ref it
/// builds against. The install / review surface, the partner packet
/// template, and the support export read these refs verbatim instead of
/// inventing a local "this sample compiles" string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SamplePackExtensionRecord {
    pub record_kind: String,
    pub sdk_v1_starter_pack_schema_version: u32,
    pub sample_pack_id: String,
    pub sample_entry_class: SamplePackEntryClass,
    pub sample_validation_class: SamplePackValidationClass,
    pub host_contract_family_class: HostContractFamilyClass,
    pub api_surface_class: SdkV1ApiSurfaceClass,
    pub manifest_baseline_ref: String,
    pub permission_manifest_ref: String,
    pub runtime_contract_ref: String,
    pub sdk_release_bundle_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_result_ref: Option<String>,
    pub sample_summary_label: String,
    pub repair_affordance_label: String,
    pub redaction_class: RedactionClass,
}

/// Input to evaluate one SDK v1 starter pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkV1StarterPackInput {
    pub starter_pack_id: String,
    pub sdk_line_id: String,
    pub sdk_line_semver: String,
    pub claimed_api_surfaces: Vec<SdkV1ApiSurfaceRecord>,
    pub authoring_guides: Vec<SdkV1ManifestAuthoringGuideRecord>,
    pub sample_pack_entries: Vec<SamplePackExtensionRecord>,
    pub computed_at: String,
}

/// One bundled, inspectable SDK v1 starter-pack record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkV1StarterPackRecord {
    pub record_kind: String,
    pub sdk_v1_starter_pack_schema_version: u32,
    pub starter_pack_id: String,
    pub sdk_line_id: String,
    pub sdk_line_semver: String,
    pub claimed_api_surfaces: Vec<SdkV1ApiSurfaceRecord>,
    pub authoring_guides: Vec<SdkV1ManifestAuthoringGuideRecord>,
    pub sample_pack_entries: Vec<SamplePackExtensionRecord>,
    pub claimed_api_surface_count: u32,
    pub available_in_beta_surface_count: u32,
    pub preview_in_beta_surface_count: u32,
    pub wasm_sample_count: u32,
    pub external_host_sample_count: u32,
    pub authoring_guide_count: u32,
    pub decision_class: SdkV1StarterPackDecisionClass,
    pub reason_class: SdkV1StarterPackReasonClass,
    pub decision_summary: String,
    pub computed_at: String,
    pub redaction_class: RedactionClass,
}

/// First consumer projection: a metadata-safe support / partner export
/// that quotes the starter-pack truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkV1StarterPackSupportExportRecord {
    pub record_kind: String,
    pub sdk_v1_starter_pack_schema_version: u32,
    pub export_id: String,
    pub starter_pack_ref: String,
    pub sdk_line_id: String,
    pub sdk_line_semver: String,
    pub decision_class: SdkV1StarterPackDecisionClass,
    pub reason_class: SdkV1StarterPackReasonClass,
    pub claimed_api_surface_count: u32,
    pub available_in_beta_surface_count: u32,
    pub preview_in_beta_surface_count: u32,
    pub wasm_sample_count: u32,
    pub external_host_sample_count: u32,
    pub authoring_guide_count: u32,
    pub blocks_authoring: bool,
    pub preview_disclosure_required: bool,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by the SDK v1 starter-pack
/// validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkV1StarterPackFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl SdkV1StarterPackFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Map an SDK v1 typed-API surface class to the host contract family it
/// targets. The mapping is closed and total over [`SdkV1ApiSurfaceClass`].
pub fn host_contract_family_for_api_surface(
    surface: SdkV1ApiSurfaceClass,
) -> HostContractFamilyClass {
    match surface {
        SdkV1ApiSurfaceClass::WasmComponentModelHostApi => HostContractFamilyClass::WasmComponentModel,
        SdkV1ApiSurfaceClass::WasmCoreModuleHostApi => HostContractFamilyClass::WasmCoreModule,
        SdkV1ApiSurfaceClass::ExternalHostSupervisedApi => HostContractFamilyClass::ExternalHostProcess,
        SdkV1ApiSurfaceClass::HelperBinaryApi => HostContractFamilyClass::HelperBinary,
        SdkV1ApiSurfaceClass::CompatibilityBridgeApi => HostContractFamilyClass::CompatibilityBridge,
        SdkV1ApiSurfaceClass::RemoteSideComponentApi => HostContractFamilyClass::RemoteSideComponent,
    }
}

/// Evaluate one [`SdkV1StarterPackInput`] into a typed
/// [`SdkV1StarterPackRecord`].
///
/// Decision precedence (most-restrictive first):
///
/// 1. Pack id / sdk-line / surface-list shape invariants refuse the pack
///    with [`SdkV1StarterPackDecisionClass::RefusedInconsistentInput`].
/// 2. A claimed wasm API surface (component-model or core-module) without
///    at least one wasm sample row refuses with
///    [`SdkV1StarterPackReasonClass::RefusedNoWasmSampleForClaimedWasmLane`].
/// 3. A claimed external-host API surface without at least one external-
///    host sample row refuses with
///    [`SdkV1StarterPackReasonClass::RefusedNoExternalHostSampleForClaimedExternalHostLane`].
/// 4. A claimed API surface class with no authoring guide that covers it
///    refuses with
///    [`SdkV1StarterPackReasonClass::RefusedAuthoringGuideMissingForClaimedSurface`].
/// 5. A runnable sample with
///    [`SamplePackValidationClass::DocumentationOnly`] refuses with
///    [`SdkV1StarterPackReasonClass::RefusedSampleValidationDocumentationOnlyOnRunnableEntry`].
/// 6. A sample row whose `host_contract_family_class` disagrees with its
///    `api_surface_class` refuses with
///    [`SdkV1StarterPackReasonClass::RefusedSampleHostFamilyDisagreesWithApiSurface`].
/// 7. Any retired surface refuses with
///    [`SdkV1StarterPackReasonClass::RefusedSurfaceAvailabilityRetired`].
/// 8. If any claimed surface is preview-only or not-available-until-general,
///    the pack resolves to
///    [`SdkV1StarterPackDecisionClass::PartiallyReadyPreviewSurfacesOnly`].
/// 9. Otherwise the pack resolves to
///    [`SdkV1StarterPackDecisionClass::ReadyForAuthors`].
pub fn evaluate_sdk_v1_starter_pack(input: SdkV1StarterPackInput) -> SdkV1StarterPackRecord {
    let SdkV1StarterPackInput {
        starter_pack_id,
        sdk_line_id,
        sdk_line_semver,
        claimed_api_surfaces,
        authoring_guides,
        sample_pack_entries,
        computed_at,
    } = input;

    let claimed_api_surface_count = claimed_api_surfaces.len() as u32;
    let available_in_beta_surface_count = claimed_api_surfaces
        .iter()
        .filter(|s| s.availability_class == SdkV1ApiAvailabilityClass::AvailableInBeta)
        .count() as u32;
    let preview_in_beta_surface_count = claimed_api_surfaces
        .iter()
        .filter(|s| s.availability_class == SdkV1ApiAvailabilityClass::PreviewInBeta)
        .count() as u32;
    // Counts only validated runnable samples; documentation
    // walkthrough rows are excluded so the wasm / external-host
    // coverage claim is backed by buildable rows.
    let wasm_sample_count = sample_pack_entries
        .iter()
        .filter(|e| {
            !matches!(
                e.sample_entry_class,
                SamplePackEntryClass::ManifestAuthoringWalkthrough
            ) && matches!(
                e.host_contract_family_class,
                HostContractFamilyClass::WasmComponentModel | HostContractFamilyClass::WasmCoreModule
            )
        })
        .count() as u32;
    let external_host_sample_count = sample_pack_entries
        .iter()
        .filter(|e| {
            !matches!(
                e.sample_entry_class,
                SamplePackEntryClass::ManifestAuthoringWalkthrough
            ) && e.host_contract_family_class == HostContractFamilyClass::ExternalHostProcess
        })
        .count() as u32;
    let authoring_guide_count = authoring_guides.len() as u32;

    let (decision_class, reason_class, decision_summary) = decide_starter_pack(
        &starter_pack_id,
        &sdk_line_id,
        &claimed_api_surfaces,
        &authoring_guides,
        &sample_pack_entries,
    );

    SdkV1StarterPackRecord {
        record_kind: SDK_V1_STARTER_PACK_RECORD_KIND.to_string(),
        sdk_v1_starter_pack_schema_version: SDK_V1_STARTER_PACK_SCHEMA_VERSION,
        starter_pack_id,
        sdk_line_id,
        sdk_line_semver,
        claimed_api_surfaces,
        authoring_guides,
        sample_pack_entries,
        claimed_api_surface_count,
        available_in_beta_surface_count,
        preview_in_beta_surface_count,
        wasm_sample_count,
        external_host_sample_count,
        authoring_guide_count,
        decision_class,
        reason_class,
        decision_summary,
        computed_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a starter-pack record into the first consumer support /
/// partner export.
pub fn project_sdk_v1_starter_pack_support_export(
    record: &SdkV1StarterPackRecord,
    export_id: &str,
) -> SdkV1StarterPackSupportExportRecord {
    let blocks_authoring = matches!(
        record.decision_class,
        SdkV1StarterPackDecisionClass::RefusedInconsistentInput
    );
    let preview_disclosure_required = matches!(
        record.decision_class,
        SdkV1StarterPackDecisionClass::PartiallyReadyPreviewSurfacesOnly
    );

    let export_safe_summary = format!(
        "{} claimed API surfaces ({} available, {} preview); {} wasm samples, {} external-host samples, {} authoring guides; decision={:?}",
        record.claimed_api_surface_count,
        record.available_in_beta_surface_count,
        record.preview_in_beta_surface_count,
        record.wasm_sample_count,
        record.external_host_sample_count,
        record.authoring_guide_count,
        record.decision_class,
    );

    SdkV1StarterPackSupportExportRecord {
        record_kind: SDK_V1_STARTER_PACK_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        sdk_v1_starter_pack_schema_version: SDK_V1_STARTER_PACK_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        starter_pack_ref: record.starter_pack_id.clone(),
        sdk_line_id: record.sdk_line_id.clone(),
        sdk_line_semver: record.sdk_line_semver.clone(),
        decision_class: record.decision_class,
        reason_class: record.reason_class,
        claimed_api_surface_count: record.claimed_api_surface_count,
        available_in_beta_surface_count: record.available_in_beta_surface_count,
        preview_in_beta_surface_count: record.preview_in_beta_surface_count,
        wasm_sample_count: record.wasm_sample_count,
        external_host_sample_count: record.external_host_sample_count,
        authoring_guide_count: record.authoring_guide_count,
        blocks_authoring,
        preview_disclosure_required,
        export_safe_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate the structural invariants of an [`SdkV1ApiSurfaceRecord`].
pub fn validate_sdk_v1_api_surface_record(
    record: &SdkV1ApiSurfaceRecord,
) -> Vec<SdkV1StarterPackFinding> {
    let mut findings = Vec::new();
    if record.record_kind != SDK_V1_API_SURFACE_RECORD_KIND {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.record_kind_wrong",
            format!(
                "record_kind must be '{SDK_V1_API_SURFACE_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.sdk_v1_starter_pack_schema_version != SDK_V1_STARTER_PACK_SCHEMA_VERSION {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.schema_version_wrong",
            format!(
                "sdk_v1_starter_pack_schema_version must be {SDK_V1_STARTER_PACK_SCHEMA_VERSION}; got {}",
                record.sdk_v1_starter_pack_schema_version
            ),
        ));
    }
    if !record.api_surface_id.starts_with("sdk_v1_api_surface:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.id_unprefixed",
            "api_surface_id must start with 'sdk_v1_api_surface:'",
        ));
    }
    if !record.sdk_release_bundle_ref.starts_with("sdk_release_bundle:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.sdk_release_bundle_ref_unprefixed",
            "sdk_release_bundle_ref must start with 'sdk_release_bundle:'",
        ));
    }
    let expected_family = host_contract_family_for_api_surface(record.api_surface_class);
    if record.host_contract_family_class != expected_family {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.host_contract_family_mismatch",
            format!(
                "api_surface_class={:?} must carry host_contract_family_class={:?}; got {:?}",
                record.api_surface_class, expected_family, record.host_contract_family_class
            ),
        ));
    }
    if record.surface_summary_label.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.surface_summary_label_required",
            "surface_summary_label must be a non-empty string",
        ));
    }
    if record.sdk_line_id.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.sdk_line_id_required",
            "sdk_line_id must be a non-empty kebab-case identifier",
        ));
    }
    if record.sdk_line_semver.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.sdk_line_semver_required",
            "sdk_line_semver must be a non-empty semver string",
        ));
    }
    if matches!(
        record.api_surface_class,
        SdkV1ApiSurfaceClass::WasmComponentModelHostApi
            | SdkV1ApiSurfaceClass::WasmCoreModuleHostApi
    ) && record.covered_wit_world_refs.is_empty()
    {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.wasm_surface_must_cite_wit_world_refs",
            "wasm API surface rows must cite at least one covered WIT world ref",
        ));
    }
    if matches!(
        record.api_surface_class,
        SdkV1ApiSurfaceClass::ExternalHostSupervisedApi | SdkV1ApiSurfaceClass::HelperBinaryApi
    ) && record.external_host_contract_ref.is_none()
    {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.external_host_surface_must_cite_contract_ref",
            "external-host or helper-binary API surface rows must cite an external_host_contract_ref",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_api_surface.redaction_class_must_be_metadata_safe",
            "sdk_v1_api_surface records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    findings
}

/// Validate the structural invariants of an
/// [`SdkV1ManifestAuthoringGuideRecord`].
pub fn validate_sdk_v1_manifest_authoring_guide_record(
    record: &SdkV1ManifestAuthoringGuideRecord,
) -> Vec<SdkV1StarterPackFinding> {
    let mut findings = Vec::new();
    if record.record_kind != SDK_V1_MANIFEST_AUTHORING_GUIDE_RECORD_KIND {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.record_kind_wrong",
            format!(
                "record_kind must be '{SDK_V1_MANIFEST_AUTHORING_GUIDE_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.sdk_v1_starter_pack_schema_version != SDK_V1_STARTER_PACK_SCHEMA_VERSION {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.schema_version_wrong",
            format!(
                "sdk_v1_starter_pack_schema_version must be {SDK_V1_STARTER_PACK_SCHEMA_VERSION}; got {}",
                record.sdk_v1_starter_pack_schema_version
            ),
        ));
    }
    if !record.manifest_guide_id.starts_with("manifest_guide:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.id_unprefixed",
            "manifest_guide_id must start with 'manifest_guide:'",
        ));
    }
    if record.covered_api_surface_classes.is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.covered_api_surface_classes_required",
            "covered_api_surface_classes must list at least one SDK v1 API surface class",
        ));
    }
    if record.guide_summary_label.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.guide_summary_label_required",
            "guide_summary_label must be a non-empty string",
        ));
    }
    if record.repair_affordance_label.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.repair_affordance_label_required",
            "repair_affordance_label must be a non-empty string",
        ));
    }
    if record.doc_path_label.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.doc_path_label_required",
            "doc_path_label must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_manifest_authoring_guide.redaction_class_must_be_metadata_safe",
            "sdk_v1_manifest_authoring_guide records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    findings
}

/// Validate the structural invariants of a [`SamplePackExtensionRecord`].
pub fn validate_sample_pack_extension_record(
    record: &SamplePackExtensionRecord,
) -> Vec<SdkV1StarterPackFinding> {
    let mut findings = Vec::new();
    if record.record_kind != SAMPLE_PACK_EXTENSION_RECORD_KIND {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.record_kind_wrong",
            format!(
                "record_kind must be '{SAMPLE_PACK_EXTENSION_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.sdk_v1_starter_pack_schema_version != SDK_V1_STARTER_PACK_SCHEMA_VERSION {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.schema_version_wrong",
            format!(
                "sdk_v1_starter_pack_schema_version must be {SDK_V1_STARTER_PACK_SCHEMA_VERSION}; got {}",
                record.sdk_v1_starter_pack_schema_version
            ),
        ));
    }
    if !record.sample_pack_id.starts_with("sample_pack:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.id_unprefixed",
            "sample_pack_id must start with 'sample_pack:'",
        ));
    }
    if !record.manifest_baseline_ref.starts_with("manifest_baseline:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.manifest_baseline_ref_unprefixed",
            "manifest_baseline_ref must start with 'manifest_baseline:'",
        ));
    }
    if !record.permission_manifest_ref.starts_with("permission_manifest:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.permission_manifest_ref_unprefixed",
            "permission_manifest_ref must start with 'permission_manifest:'",
        ));
    }
    if !record.runtime_contract_ref.starts_with("runtime_v1_beta:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.runtime_contract_ref_unprefixed",
            "runtime_contract_ref must start with 'runtime_v1_beta:'",
        ));
    }
    if !record.sdk_release_bundle_ref.starts_with("sdk_release_bundle:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.sdk_release_bundle_ref_unprefixed",
            "sdk_release_bundle_ref must start with 'sdk_release_bundle:'",
        ));
    }
    let expected_family = host_contract_family_for_api_surface(record.api_surface_class);
    if record.host_contract_family_class != expected_family {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.host_contract_family_mismatch",
            format!(
                "api_surface_class={:?} must carry host_contract_family_class={:?}; got {:?}",
                record.api_surface_class, expected_family, record.host_contract_family_class
            ),
        ));
    }
    if record.sample_summary_label.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.sample_summary_label_required",
            "sample_summary_label must be a non-empty string",
        ));
    }
    if record.repair_affordance_label.trim().is_empty() {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.repair_affordance_label_required",
            "repair_affordance_label must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.redaction_class_must_be_metadata_safe",
            "sample_pack_extension records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    // A runnable sample entry MUST NOT be tagged DocumentationOnly.
    if !matches!(
        record.sample_entry_class,
        SamplePackEntryClass::ManifestAuthoringWalkthrough
    ) && record.sample_validation_class == SamplePackValidationClass::DocumentationOnly
    {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.runnable_sample_must_not_be_documentation_only",
            "runnable sample entries must carry a must_compile_in_ci / must_validate_in_ci / must_run_in_ci validation class",
        ));
    }
    // A documentation-walkthrough entry MUST be DocumentationOnly.
    if matches!(
        record.sample_entry_class,
        SamplePackEntryClass::ManifestAuthoringWalkthrough
    ) && record.sample_validation_class != SamplePackValidationClass::DocumentationOnly
    {
        findings.push(SdkV1StarterPackFinding::new(
            "sample_pack_extension.walkthrough_entry_must_be_documentation_only",
            "manifest_authoring_walkthrough entries must carry sample_validation_class = documentation_only",
        ));
    }
    findings
}

/// Validate the structural invariants of an [`SdkV1StarterPackRecord`].
pub fn validate_sdk_v1_starter_pack_record(
    record: &SdkV1StarterPackRecord,
) -> Vec<SdkV1StarterPackFinding> {
    let mut findings = Vec::new();
    if record.record_kind != SDK_V1_STARTER_PACK_RECORD_KIND {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.record_kind_wrong",
            format!(
                "record_kind must be '{SDK_V1_STARTER_PACK_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.sdk_v1_starter_pack_schema_version != SDK_V1_STARTER_PACK_SCHEMA_VERSION {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.schema_version_wrong",
            format!(
                "sdk_v1_starter_pack_schema_version must be {SDK_V1_STARTER_PACK_SCHEMA_VERSION}; got {}",
                record.sdk_v1_starter_pack_schema_version
            ),
        ));
    }
    if !record.starter_pack_id.starts_with("sdk_v1_starter_pack:") {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.id_unprefixed",
            "starter_pack_id must start with 'sdk_v1_starter_pack:'",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.redaction_class_must_be_metadata_safe",
            "sdk_v1_starter_pack records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    // Cross-check derived counters.
    let expected_claimed = record.claimed_api_surfaces.len() as u32;
    if record.claimed_api_surface_count != expected_claimed {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.claimed_api_surface_count_inconsistent",
            "claimed_api_surface_count must equal claimed_api_surfaces.len()",
        ));
    }
    let expected_available = record
        .claimed_api_surfaces
        .iter()
        .filter(|s| s.availability_class == SdkV1ApiAvailabilityClass::AvailableInBeta)
        .count() as u32;
    if record.available_in_beta_surface_count != expected_available {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.available_in_beta_surface_count_inconsistent",
            "available_in_beta_surface_count must equal the count of AvailableInBeta surfaces",
        ));
    }
    let expected_preview = record
        .claimed_api_surfaces
        .iter()
        .filter(|s| s.availability_class == SdkV1ApiAvailabilityClass::PreviewInBeta)
        .count() as u32;
    if record.preview_in_beta_surface_count != expected_preview {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.preview_in_beta_surface_count_inconsistent",
            "preview_in_beta_surface_count must equal the count of PreviewInBeta surfaces",
        ));
    }
    let expected_wasm = record
        .sample_pack_entries
        .iter()
        .filter(|e| {
            !matches!(
                e.sample_entry_class,
                SamplePackEntryClass::ManifestAuthoringWalkthrough
            ) && matches!(
                e.host_contract_family_class,
                HostContractFamilyClass::WasmComponentModel | HostContractFamilyClass::WasmCoreModule
            )
        })
        .count() as u32;
    if record.wasm_sample_count != expected_wasm {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.wasm_sample_count_inconsistent",
            "wasm_sample_count must equal the count of validated runnable wasm-host samples (documentation walkthroughs excluded)",
        ));
    }
    let expected_external = record
        .sample_pack_entries
        .iter()
        .filter(|e| {
            !matches!(
                e.sample_entry_class,
                SamplePackEntryClass::ManifestAuthoringWalkthrough
            ) && e.host_contract_family_class == HostContractFamilyClass::ExternalHostProcess
        })
        .count() as u32;
    if record.external_host_sample_count != expected_external {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.external_host_sample_count_inconsistent",
            "external_host_sample_count must equal the count of validated runnable external-host samples (documentation walkthroughs excluded)",
        ));
    }
    if record.authoring_guide_count != record.authoring_guides.len() as u32 {
        findings.push(SdkV1StarterPackFinding::new(
            "sdk_v1_starter_pack.authoring_guide_count_inconsistent",
            "authoring_guide_count must equal authoring_guides.len()",
        ));
    }
    findings
}

fn decide_starter_pack(
    starter_pack_id: &str,
    sdk_line_id: &str,
    claimed_api_surfaces: &[SdkV1ApiSurfaceRecord],
    authoring_guides: &[SdkV1ManifestAuthoringGuideRecord],
    sample_pack_entries: &[SamplePackExtensionRecord],
) -> (
    SdkV1StarterPackDecisionClass,
    SdkV1StarterPackReasonClass,
    String,
) {
    if !starter_pack_id.starts_with("sdk_v1_starter_pack:") {
        return (
            SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
            SdkV1StarterPackReasonClass::RefusedPackIdUnprefixed,
            "Refused: starter_pack_id must start with 'sdk_v1_starter_pack:'.".to_string(),
        );
    }
    if sdk_line_id.trim().is_empty() {
        return (
            SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
            SdkV1StarterPackReasonClass::RefusedSdkLineRefMissing,
            "Refused: sdk_line_id must be a non-empty kebab-case identifier.".to_string(),
        );
    }
    if claimed_api_surfaces.is_empty() {
        return (
            SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
            SdkV1StarterPackReasonClass::RefusedClaimedSurfacesEmpty,
            "Refused: claimed_api_surfaces must list at least one SDK v1 API surface.".to_string(),
        );
    }
    for surface in claimed_api_surfaces {
        if !surface.sdk_release_bundle_ref.starts_with("sdk_release_bundle:") {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSdkLineRefMissing,
                format!(
                    "Refused: claimed API surface '{}' is missing the 'sdk_release_bundle:' prefix on sdk_release_bundle_ref.",
                    surface.api_surface_id,
                ),
            );
        }
        if surface.availability_class == SdkV1ApiAvailabilityClass::RetiredPendingSuccessor {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSurfaceAvailabilityRetired,
                format!(
                    "Refused: claimed API surface '{}' is retired_pending_successor.",
                    surface.api_surface_id,
                ),
            );
        }
    }
    for sample in sample_pack_entries {
        if !sample.manifest_baseline_ref.starts_with("manifest_baseline:") {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSampleManifestRefUnprefixed,
                format!(
                    "Refused: sample '{}' manifest_baseline_ref is missing the 'manifest_baseline:' prefix.",
                    sample.sample_pack_id,
                ),
            );
        }
        if !sample.permission_manifest_ref.starts_with("permission_manifest:") {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSamplePermissionManifestRefUnprefixed,
                format!(
                    "Refused: sample '{}' permission_manifest_ref is missing the 'permission_manifest:' prefix.",
                    sample.sample_pack_id,
                ),
            );
        }
        if !sample.runtime_contract_ref.starts_with("runtime_v1_beta:") {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSampleRuntimeContractRefUnprefixed,
                format!(
                    "Refused: sample '{}' runtime_contract_ref is missing the 'runtime_v1_beta:' prefix.",
                    sample.sample_pack_id,
                ),
            );
        }
        let expected_family = host_contract_family_for_api_surface(sample.api_surface_class);
        if sample.host_contract_family_class != expected_family {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSampleHostFamilyDisagreesWithApiSurface,
                format!(
                    "Refused: sample '{}' host_contract_family_class={:?} disagrees with api_surface_class={:?}.",
                    sample.sample_pack_id, sample.host_contract_family_class, sample.api_surface_class,
                ),
            );
        }
        let runnable = !matches!(
            sample.sample_entry_class,
            SamplePackEntryClass::ManifestAuthoringWalkthrough
        );
        if runnable
            && sample.sample_validation_class == SamplePackValidationClass::DocumentationOnly
        {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedSampleValidationDocumentationOnlyOnRunnableEntry,
                format!(
                    "Refused: runnable sample '{}' carries sample_validation_class=documentation_only.",
                    sample.sample_pack_id,
                ),
            );
        }
    }
    let claims_wasm = claimed_api_surfaces.iter().any(|s| {
        matches!(
            s.api_surface_class,
            SdkV1ApiSurfaceClass::WasmComponentModelHostApi
                | SdkV1ApiSurfaceClass::WasmCoreModuleHostApi
        )
    });
    let claims_external_host = claimed_api_surfaces.iter().any(|s| {
        matches!(
            s.api_surface_class,
            SdkV1ApiSurfaceClass::ExternalHostSupervisedApi
        )
    });
    let has_wasm_sample = sample_pack_entries.iter().any(|e| {
        !matches!(
            e.sample_entry_class,
            SamplePackEntryClass::ManifestAuthoringWalkthrough
        ) && matches!(
            e.host_contract_family_class,
            HostContractFamilyClass::WasmComponentModel | HostContractFamilyClass::WasmCoreModule
        )
    });
    let has_external_host_sample = sample_pack_entries.iter().any(|e| {
        !matches!(
            e.sample_entry_class,
            SamplePackEntryClass::ManifestAuthoringWalkthrough
        ) && e.host_contract_family_class == HostContractFamilyClass::ExternalHostProcess
    });
    if claims_wasm && !has_wasm_sample {
        return (
            SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
            SdkV1StarterPackReasonClass::RefusedNoWasmSampleForClaimedWasmLane,
            "Refused: starter pack claims a wasm API surface but ships no validated wasm sample row."
                .to_string(),
        );
    }
    if claims_external_host && !has_external_host_sample {
        return (
            SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
            SdkV1StarterPackReasonClass::RefusedNoExternalHostSampleForClaimedExternalHostLane,
            "Refused: starter pack claims an external-host API surface but ships no validated external-host sample row.".to_string(),
        );
    }
    for surface in claimed_api_surfaces {
        let covered = authoring_guides
            .iter()
            .any(|g| g.covered_api_surface_classes.contains(&surface.api_surface_class));
        if !covered {
            return (
                SdkV1StarterPackDecisionClass::RefusedInconsistentInput,
                SdkV1StarterPackReasonClass::RefusedAuthoringGuideMissingForClaimedSurface,
                format!(
                    "Refused: claimed API surface '{}' (class={:?}) has no authoring guide covering it.",
                    surface.api_surface_id, surface.api_surface_class,
                ),
            );
        }
    }
    let any_preview_or_not_available = claimed_api_surfaces.iter().any(|s| {
        matches!(
            s.availability_class,
            SdkV1ApiAvailabilityClass::PreviewInBeta
                | SdkV1ApiAvailabilityClass::NotAvailableUntilGeneral
        )
    });
    if any_preview_or_not_available {
        return (
            SdkV1StarterPackDecisionClass::PartiallyReadyPreviewSurfacesOnly,
            SdkV1StarterPackReasonClass::SomeClaimedSurfacesPreviewInBeta,
            "Partially ready: at least one claimed API surface is preview-only or not available until general; install / review chrome MUST disclose the preview posture verbatim.".to_string(),
        );
    }
    (
        SdkV1StarterPackDecisionClass::ReadyForAuthors,
        SdkV1StarterPackReasonClass::AllClaimedSurfacesAvailableInBeta,
        "Ready: every claimed API surface is available_in_beta, every claimed wasm / external-host lane has at least one validated sample, and every claimed surface has at least one authoring guide.".to_string(),
    )
}
