//! Crash envelope + exact-build symbol manifest binding (alpha).
//!
//! This module owns the alpha contract that joins a runtime
//! [`crate::CrashEnvelope`] to a release-side [`SymbolManifest`] and
//! projects one [`CrashEnvelopeSymbolBinding`] record that downstream
//! support, recovery, and export surfaces can inspect without parsing
//! raw dump bytes, raw stack bodies, or absolute paths.
//!
//! The boundary schema for both records lives at
//! [`SYMBOL_MANIFEST_SCHEMA_REF`] and the reviewer doc lives at
//! [`SYMBOL_MANIFEST_DOC_REF`]. Both refs are quoted verbatim on every
//! emitted binding so the headless export, the support chrome, and the
//! release reviewer share one truth.
//!
//! ## What this module owns
//!
//! - The [`SymbolManifest`] release-side declaration of per-module
//!   symbol identities, support-archive identities, build/debug ids
//!   for native binaries, and source-map digests for source-map
//!   bundles. The manifest is metadata-only: it carries refs and
//!   identities, never symbol bytes, dwarf tables, or absolute paths.
//! - The [`CrashEnvelopeSymbolBinding`] support-side projection that
//!   joins one [`CrashEnvelope`] (and the optional in-tree
//!   [`SymbolicationReport`]) to a [`SymbolManifest`] and labels the
//!   result as one of [`SymbolBindingState::Linked`],
//!   [`SymbolBindingState::Partial`],
//!   [`SymbolBindingState::MissingManifest`], or
//!   [`SymbolBindingState::BuildMismatch`].
//! - The [`bind_crash_envelope`] entry point and the
//!   [`CrashEnvelopeBindingInputs`] container the support pipeline
//!   uses to produce one binding from already loaded fixture records.
//!
//! ## What this module does NOT own
//!
//! - Reading raw minidump bytes, raw memory pages, or platform-debugger
//!   output. The binding is computed from already typed envelope,
//!   manifest, and optional symbolication-report records.
//! - Uploading crash evidence. Every binding pins
//!   `raw_dump_exported = false` and the support export posture defaults
//!   to `metadata_only_default`; raw dump bodies remain governed by
//!   `support.item.crash_dump_or_core` and stay local-only unless an
//!   explicit reviewed upload path is approved.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    incident_trail::{CrashEnvelope, SymbolicationReport},
    symbolication::InTreeSymbolModule,
};

/// Stable record-kind tag for a symbol manifest record.
pub const SYMBOL_MANIFEST_RECORD_KIND: &str = "symbol_manifest_record";

/// Stable record-kind tag for a crash-envelope symbol binding record.
pub const CRASH_ENVELOPE_SYMBOL_BINDING_RECORD_KIND: &str = "crash_envelope_symbol_binding_record";

/// Frozen schema version for the alpha symbol-manifest contract.
pub const SYMBOL_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema both records mirror.
pub const SYMBOL_MANIFEST_SCHEMA_REF: &str =
    "schemas/support/crash_symbolication_manifest_alpha.schema.json";

/// Reviewer doc ref quoted verbatim by every emitted binding.
pub const SYMBOL_MANIFEST_DOC_REF: &str = "docs/support/m3/crash_symbolication_alpha.md";

/// Closed module-kind vocabulary for crash and symbol-manifest rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestModuleKind {
    /// Native executable or dynamic library shipped with the IDE.
    NativeBinary,
    /// Web/source-map bundle shipped with the renderer surface.
    WebBundle,
}

impl ManifestModuleKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeBinary => "native_binary",
            Self::WebBundle => "web_bundle",
        }
    }
}

/// Closed artifact-family vocabulary the manifest admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestArtifactFamilyClass {
    /// The shipped IDE shell or helper binary.
    IdeBinary,
    /// The shipped command-line binary.
    CliBinary,
    /// A support helper binary used by recovery flows.
    SupportHelperBinary,
    /// A source-map bundle for a renderer / web surface.
    SourceMapBundle,
}

impl ManifestArtifactFamilyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdeBinary => "ide_binary",
            Self::CliBinary => "cli_binary",
            Self::SupportHelperBinary => "support_helper_binary",
            Self::SourceMapBundle => "source_map_bundle",
        }
    }
}

/// Closed release-channel vocabulary for the symbol manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannelClass {
    /// Alpha channel.
    Alpha,
    /// Preview/design-partner channel.
    Preview,
    /// Beta channel.
    Beta,
}

impl ReleaseChannelClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Alpha => "alpha",
            Self::Preview => "preview",
            Self::Beta => "beta",
        }
    }
}

/// Closed storage-class vocabulary; the manifest never embeds symbol bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestStorageClass {
    /// Manifest carries identities and refs only — never symbol bytes.
    MetadataOnlyNoSymbolBytes,
}

impl ManifestStorageClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnlyNoSymbolBytes => "metadata_only_no_symbol_bytes",
        }
    }
}

/// Closed redaction-class vocabulary admitted on the manifest itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestRedactionClass {
    /// Reviewer-safe defaults: no raw private material, no ambient authority.
    MetadataSafeDefault,
}

impl ManifestRedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
        }
    }
}

/// Closed support-export posture vocabulary on the binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportPostureClass {
    /// Metadata-only default: bindings can be exported, raw bytes cannot.
    MetadataOnlyDefault,
    /// Local-inspection-only: bindings stay on the local device pending review.
    LocalInspectionOnly,
}

impl SupportExportPostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnlyDefault => "metadata_only_default",
            Self::LocalInspectionOnly => "local_inspection_only",
        }
    }
}

/// Overall binding state for a crash envelope joined to a symbol manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolBindingState {
    /// Every envelope module is covered by a manifest module with matching
    /// exact-build identity.
    Linked,
    /// At least one envelope module is missing from the manifest, or at
    /// least one manifest entry lacks an envelope module, but no identity
    /// mismatch was observed.
    Partial,
    /// No manifest was supplied; binding remains honest without claiming
    /// any symbol coverage.
    MissingManifest,
    /// One or more module identities (envelope vs. manifest, or
    /// manifest vs. report) disagree on exact-build family; the binding
    /// refuses to label the crash as exact.
    BuildMismatch,
}

impl SymbolBindingState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Partial => "partial",
            Self::MissingManifest => "missing_manifest",
            Self::BuildMismatch => "build_mismatch",
        }
    }
}

/// Per-module binding state for a crash module against a manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleBindingState {
    /// Manifest entry and envelope module share the exact-build family.
    Linked,
    /// Envelope named a module that the manifest does not declare.
    MissingFromManifest,
    /// Manifest declared a module that the envelope did not name.
    ExtraInEnvelope,
    /// Identities disagree between envelope and manifest for the module.
    IdentityMismatch,
}

impl ModuleBindingState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::MissingFromManifest => "missing_from_manifest",
            Self::ExtraInEnvelope => "extra_in_envelope",
            Self::IdentityMismatch => "identity_mismatch",
        }
    }
}

/// One symbol manifest module row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolManifestModule {
    /// Stable module id; must match the crash envelope's `module_id`.
    pub module_id: String,
    /// Closed module kind.
    pub module_kind: ManifestModuleKind,
    /// Closed artifact-family class.
    pub artifact_family_class: ManifestArtifactFamilyClass,
    /// Exact-build identity of the artifact this row maps to.
    pub exact_build_identity_ref: String,
    /// Exact-build identity of the symbol/source-map artifact.
    pub symbolication_identity_ref: String,
    /// Optional support-archive identity for native modules
    /// (the crash-symbol archive ref carried into support bundles).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_archive_identity_ref: Option<String>,
    /// Optional code file name for native binaries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_file_name: Option<String>,
    /// Optional native build id (e.g. GNU build-id) for native binaries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// Optional native debug id for native binaries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_id: Option<String>,
    /// Optional renderer bundle revision ref for source-map bundles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_revision_ref: Option<String>,
    /// Optional source-map digest for source-map bundles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_map_digest: Option<String>,
    /// Optional generated asset ref for source-map bundles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_asset_ref: Option<String>,
    /// Closed storage class; manifests never embed symbol bytes.
    pub storage_class: ManifestStorageClass,
    /// Reviewer-safe note for this row.
    pub notes: String,
}

/// Release-side declaration of per-module symbol identities tied to one
/// exact-build identity. This is the metadata-only sibling of the crash
/// envelope: it tells support what symbols are *supposed* to exist for
/// a given build, so a runtime crash can be bound to a release artifact
/// set or labeled `build_mismatch` honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolManifest {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable manifest id.
    pub manifest_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Release channel this manifest was generated for.
    pub release_channel_class: ReleaseChannelClass,
    /// Primary exact-build identity for the manifest.
    pub primary_exact_build_identity_ref: String,
    /// Workspace version pinned by the manifest.
    pub workspace_version: String,
    /// Target triple pinned by the manifest.
    pub target_triple: String,
    /// Build profile pinned by the manifest.
    pub profile: String,
    /// Short commit hash pinned by the manifest.
    pub commit_short: String,
    /// Optional release evidence packet ref this manifest belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_evidence_packet_ref: Option<String>,
    /// Optional retention seed ref shared with support/export manifests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_seed_ref: Option<String>,
    /// Module rows. MUST be non-empty.
    pub modules: Vec<SymbolManifestModule>,
    /// Redaction class governing the manifest body.
    pub redaction_class: ManifestRedactionClass,
    /// Pinned to true: the manifest excludes raw private material.
    pub raw_private_material_excluded: bool,
    /// Pinned to true: the manifest excludes ambient authority.
    pub ambient_authority_excluded: bool,
    /// Reviewer-safe summary note.
    pub notes: String,
}

/// One row in a crash-envelope binding report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleBindingRow {
    /// Stable module id.
    pub module_id: String,
    /// Module kind reported by the envelope (or manifest, for
    /// `extra_in_envelope`).
    pub module_kind: ManifestModuleKind,
    /// Exact-build identity carried by the envelope row.
    pub envelope_exact_build_identity_ref: String,
    /// Optional manifest symbolication identity ref for this module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_symbolication_identity_ref: Option<String>,
    /// Optional manifest support-archive identity ref for this module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_support_archive_identity_ref: Option<String>,
    /// Per-module binding state.
    pub binding_state: ModuleBindingState,
    /// Optional mismatch reason for partial / mismatched rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mismatch_reason: Option<String>,
}

/// Inputs the support pipeline gives to [`bind_crash_envelope`].
#[derive(Debug, Clone)]
pub struct CrashEnvelopeBindingInputs<'a> {
    /// Stable binding id.
    pub binding_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Crash envelope captured locally.
    pub crash_envelope: &'a CrashEnvelope,
    /// Optional release-side symbol manifest. When `None`, the binding
    /// reports [`SymbolBindingState::MissingManifest`] without claiming
    /// any symbol coverage.
    pub symbol_manifest: Option<&'a SymbolManifest>,
    /// Optional in-tree symbolication report produced by the local
    /// symbolicator.
    pub symbolication_report: Option<&'a SymbolicationReport>,
    /// Optional support-bundle manifest ref the binding rolls into.
    pub support_bundle_manifest_ref: Option<String>,
    /// Support-export posture for this binding.
    pub support_export_posture: SupportExportPostureClass,
}

/// Crash envelope joined to a symbol manifest and (optionally) a
/// symbolication report. The binding is metadata-safe: no raw dump
/// bytes, no raw stack bodies, no absolute paths, no ambient authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashEnvelopeSymbolBinding {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable binding id.
    pub binding_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Crash envelope ref the binding was computed from.
    pub crash_envelope_ref: String,
    /// Optional symbol manifest ref. Present when a manifest was bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_manifest_ref: Option<String>,
    /// Optional local symbolication report ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbolication_report_ref: Option<String>,
    /// Optional support-bundle manifest ref this binding rolls into.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_bundle_manifest_ref: Option<String>,
    /// Primary exact-build identity from the crash envelope.
    pub primary_exact_build_identity_ref: String,
    /// Optional primary exact-build identity from the bound manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_primary_exact_build_identity_ref: Option<String>,
    /// Overall binding state.
    pub binding_state: SymbolBindingState,
    /// Per-module binding rows.
    pub module_bindings: Vec<ModuleBindingRow>,
    /// Support-export posture for this binding.
    pub support_export_posture: SupportExportPostureClass,
    /// Pinned to true: the binding excludes raw private material.
    pub raw_private_material_excluded: bool,
    /// Pinned to true: the binding excludes ambient authority.
    pub ambient_authority_excluded: bool,
    /// Pinned to false: raw dump bytes are never exported by default.
    pub raw_dump_exported: bool,
    /// Reviewer doc ref quoted verbatim.
    pub doc_ref: String,
    /// Boundary schema ref quoted verbatim.
    pub schema_ref: String,
    /// Honesty notes about partial, missing, or mismatched evidence.
    #[serde(default)]
    pub honesty_notes: Vec<String>,
    /// Reviewer-safe summary note.
    pub notes: String,
}

impl CrashEnvelopeSymbolBinding {
    /// True when the binding fully links every envelope module to the
    /// manifest with matching exact-build identities.
    pub fn is_linked(&self) -> bool {
        self.binding_state == SymbolBindingState::Linked
    }

    /// True when the binding refuses to claim exact symbolication
    /// because some identity disagreed.
    pub fn is_build_mismatch(&self) -> bool {
        self.binding_state == SymbolBindingState::BuildMismatch
    }

    /// True when no manifest was bound to the envelope.
    pub fn is_missing_manifest(&self) -> bool {
        self.binding_state == SymbolBindingState::MissingManifest
    }

    /// True when raw private material and ambient authority are
    /// excluded and raw dump bytes are not exported. Every alpha
    /// binding satisfies this property by construction.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.raw_dump_exported
    }
}

/// Build one [`CrashEnvelopeSymbolBinding`] from already loaded
/// envelope, manifest, and optional symbolication-report records.
pub fn bind_crash_envelope(inputs: CrashEnvelopeBindingInputs<'_>) -> CrashEnvelopeSymbolBinding {
    let envelope = inputs.crash_envelope;
    let manifest = inputs.symbol_manifest;
    let report = inputs.symbolication_report;

    let primary_envelope_identity = envelope.primary_exact_build_identity_ref.clone();
    let manifest_primary_identity = manifest.map(|m| m.primary_exact_build_identity_ref.clone());

    let primary_mismatch = manifest
        .map(|m| m.primary_exact_build_identity_ref != envelope.primary_exact_build_identity_ref)
        .unwrap_or(false);

    let report_primary_mismatch = report
        .map(|r| {
            r.primary_exact_build_identity_ref != envelope.primary_exact_build_identity_ref
                || r.crash_envelope_ref != envelope.crash_envelope_ref
        })
        .unwrap_or(false);

    let manifest_by_id = manifest
        .map(|m| {
            m.modules
                .iter()
                .map(|module| (module.module_id.clone(), module))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let envelope_module_ids = envelope
        .modules
        .iter()
        .map(|module| module.module_id.clone())
        .collect::<BTreeSet<_>>();

    let mut module_bindings = Vec::new();
    let mut module_mismatch_observed = false;
    let mut module_missing_observed = false;

    for envelope_module in &envelope.modules {
        let row = match manifest_by_id.get(&envelope_module.module_id) {
            Some(manifest_module) => {
                let identity_matches = exact_build_family_matches(
                    &envelope.primary_exact_build_identity_ref,
                    &envelope_module.exact_build_identity_ref,
                ) && exact_build_family_matches(
                    &envelope.primary_exact_build_identity_ref,
                    &manifest_module.exact_build_identity_ref,
                ) && exact_build_family_matches(
                    &envelope.primary_exact_build_identity_ref,
                    &manifest_module.symbolication_identity_ref,
                ) && manifest_module
                    .support_archive_identity_ref
                    .as_deref()
                    .map_or(true, |identity| {
                        exact_build_family_matches(
                            &envelope.primary_exact_build_identity_ref,
                            identity,
                        )
                    });

                let kind_matches =
                    manifest_module_kind_matches(envelope_module.module_kind.as_str())
                        == Some(manifest_module.module_kind);

                if identity_matches && kind_matches && !primary_mismatch {
                    ModuleBindingRow {
                        module_id: envelope_module.module_id.clone(),
                        module_kind: manifest_module.module_kind,
                        envelope_exact_build_identity_ref: envelope_module
                            .exact_build_identity_ref
                            .clone(),
                        manifest_symbolication_identity_ref: Some(
                            manifest_module.symbolication_identity_ref.clone(),
                        ),
                        manifest_support_archive_identity_ref: manifest_module
                            .support_archive_identity_ref
                            .clone(),
                        binding_state: ModuleBindingState::Linked,
                        mismatch_reason: None,
                    }
                } else {
                    module_mismatch_observed = true;
                    let reason = if !kind_matches {
                        format!(
                            "module_kind_mismatch: envelope={} manifest={}",
                            envelope_module.module_kind,
                            manifest_module.module_kind.as_str()
                        )
                    } else if primary_mismatch {
                        "primary_exact_build_identity_mismatch".to_owned()
                    } else {
                        "module_exact_build_identity_outside_runtime_family".to_owned()
                    };
                    ModuleBindingRow {
                        module_id: envelope_module.module_id.clone(),
                        module_kind: manifest_module.module_kind,
                        envelope_exact_build_identity_ref: envelope_module
                            .exact_build_identity_ref
                            .clone(),
                        manifest_symbolication_identity_ref: Some(
                            manifest_module.symbolication_identity_ref.clone(),
                        ),
                        manifest_support_archive_identity_ref: manifest_module
                            .support_archive_identity_ref
                            .clone(),
                        binding_state: ModuleBindingState::IdentityMismatch,
                        mismatch_reason: Some(reason),
                    }
                }
            }
            None => {
                module_missing_observed = manifest.is_some();
                let module_kind =
                    manifest_module_kind_matches(envelope_module.module_kind.as_str())
                        .unwrap_or(ManifestModuleKind::NativeBinary);
                ModuleBindingRow {
                    module_id: envelope_module.module_id.clone(),
                    module_kind,
                    envelope_exact_build_identity_ref: envelope_module
                        .exact_build_identity_ref
                        .clone(),
                    manifest_symbolication_identity_ref: None,
                    manifest_support_archive_identity_ref: None,
                    binding_state: if manifest.is_some() {
                        ModuleBindingState::MissingFromManifest
                    } else {
                        ModuleBindingState::ExtraInEnvelope
                    },
                    mismatch_reason: if manifest.is_some() {
                        Some("module_absent_from_symbol_manifest".to_owned())
                    } else {
                        Some("no_symbol_manifest_bound".to_owned())
                    },
                }
            }
        };
        module_bindings.push(row);
    }

    if let Some(manifest_record) = manifest {
        for manifest_module in &manifest_record.modules {
            if !envelope_module_ids.contains(&manifest_module.module_id) {
                module_missing_observed = true;
                module_bindings.push(ModuleBindingRow {
                    module_id: manifest_module.module_id.clone(),
                    module_kind: manifest_module.module_kind,
                    envelope_exact_build_identity_ref: String::new(),
                    manifest_symbolication_identity_ref: Some(
                        manifest_module.symbolication_identity_ref.clone(),
                    ),
                    manifest_support_archive_identity_ref: manifest_module
                        .support_archive_identity_ref
                        .clone(),
                    binding_state: ModuleBindingState::ExtraInEnvelope,
                    mismatch_reason: Some("manifest_module_absent_from_crash_envelope".to_owned()),
                });
            }
        }
    }

    if let Some(report_record) = report {
        report_in_tree_mismatch_observed(
            report_record,
            &envelope.primary_exact_build_identity_ref,
            &mut module_bindings,
        );
    }

    let binding_state = if manifest.is_none() {
        SymbolBindingState::MissingManifest
    } else if primary_mismatch
        || report_primary_mismatch
        || module_mismatch_observed
        || module_bindings
            .iter()
            .any(|row| row.binding_state == ModuleBindingState::IdentityMismatch)
    {
        SymbolBindingState::BuildMismatch
    } else if module_missing_observed
        || module_bindings.iter().any(|row| {
            matches!(
                row.binding_state,
                ModuleBindingState::MissingFromManifest | ModuleBindingState::ExtraInEnvelope
            )
        })
    {
        SymbolBindingState::Partial
    } else {
        SymbolBindingState::Linked
    };

    let honesty_notes = honesty_notes_for(binding_state, manifest.is_some(), report.is_some());
    let notes = notes_for(binding_state);

    CrashEnvelopeSymbolBinding {
        schema_version: SYMBOL_MANIFEST_SCHEMA_VERSION,
        record_kind: CRASH_ENVELOPE_SYMBOL_BINDING_RECORD_KIND.to_owned(),
        binding_id: inputs.binding_id,
        generated_at: inputs.generated_at,
        crash_envelope_ref: envelope.crash_envelope_ref.clone(),
        symbol_manifest_ref: manifest.map(|m| m.manifest_id.clone()),
        symbolication_report_ref: report.map(|r| r.symbolication_report_ref.clone()),
        support_bundle_manifest_ref: inputs.support_bundle_manifest_ref,
        primary_exact_build_identity_ref: primary_envelope_identity,
        manifest_primary_exact_build_identity_ref: manifest_primary_identity,
        binding_state,
        module_bindings,
        support_export_posture: inputs.support_export_posture,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        raw_dump_exported: false,
        doc_ref: SYMBOL_MANIFEST_DOC_REF.to_owned(),
        schema_ref: SYMBOL_MANIFEST_SCHEMA_REF.to_owned(),
        honesty_notes,
        notes,
    }
}

/// Cross-check that the in-tree symbolication report agrees with the
/// envelope on exact-build family; downgrade rows that disagree.
fn report_in_tree_mismatch_observed(
    report: &SymbolicationReport,
    primary_envelope_identity: &str,
    module_bindings: &mut [ModuleBindingRow],
) {
    let report_by_id: BTreeMap<&str, &crate::incident_trail::SymbolicatedModuleResult> = report
        .module_results
        .iter()
        .map(|module| (module.module_id.as_str(), module))
        .collect();

    for row in module_bindings.iter_mut() {
        let Some(report_module) = report_by_id.get(row.module_id.as_str()) else {
            continue;
        };
        let mut runtime_ok = exact_build_family_matches(
            primary_envelope_identity,
            &report_module.runtime_identity_ref,
        );
        if let Some(identity) = &report_module.symbolication_identity_ref {
            runtime_ok =
                runtime_ok && exact_build_family_matches(primary_envelope_identity, identity);
        }
        if let Some(identity) = &report_module.support_archive_identity_ref {
            runtime_ok =
                runtime_ok && exact_build_family_matches(primary_envelope_identity, identity);
        }
        if !runtime_ok && row.binding_state == ModuleBindingState::Linked {
            row.binding_state = ModuleBindingState::IdentityMismatch;
            row.mismatch_reason =
                Some("symbolication_report_identity_outside_runtime_family".to_owned());
        }
    }
}

/// Match the [`InTreeSymbolModule`] kind against the manifest vocabulary
/// so the symbol-manifest module-kind check is consistent across both
/// in-tree symbol files and the release manifest.
pub fn match_in_tree_module_kind(module: &InTreeSymbolModule) -> Option<ManifestModuleKind> {
    manifest_module_kind_matches(&module.module_kind)
}

fn manifest_module_kind_matches(token: &str) -> Option<ManifestModuleKind> {
    match token {
        "native_binary" => Some(ManifestModuleKind::NativeBinary),
        "web_bundle" => Some(ManifestModuleKind::WebBundle),
        _ => None,
    }
}

fn exact_build_family_matches(primary_exact_build_ref: &str, candidate: &str) -> bool {
    candidate == primary_exact_build_ref
        || candidate
            .strip_prefix(primary_exact_build_ref)
            .is_some_and(|suffix| suffix.starts_with(':'))
}

fn honesty_notes_for(
    binding_state: SymbolBindingState,
    manifest_bound: bool,
    report_bound: bool,
) -> Vec<String> {
    let mut notes = Vec::new();
    match binding_state {
        SymbolBindingState::Linked => notes.push(
            "Crash envelope and symbol manifest agree on exact-build identity for every module."
                .to_owned(),
        ),
        SymbolBindingState::Partial => notes.push(
            "Crash envelope and symbol manifest agree on identity, but at least one module is missing from the manifest or extra in the envelope.".to_owned(),
        ),
        SymbolBindingState::MissingManifest => notes.push(
            "No symbol manifest is bound to this crash envelope; the binding preserves crash refs without implying symbols are available.".to_owned(),
        ),
        SymbolBindingState::BuildMismatch => notes.push(
            "Crash envelope and symbol manifest disagree on exact-build identity; the binding refuses to label the crash as exact and the symbolication report cannot be trusted as exact-build.".to_owned(),
        ),
    }
    if !manifest_bound {
        notes.push(
            "Release-side symbol manifest is absent. Support handoff MUST publish or bind a manifest before claiming exact-build symbolication.".to_owned(),
        );
    }
    if !report_bound {
        notes.push(
            "Local symbolication report is absent. The binding still pins build identity and manifest refs so reviewers can re-run the local symbolicator later.".to_owned(),
        );
    }
    notes
}

fn notes_for(binding_state: SymbolBindingState) -> String {
    format!(
        "Crash envelope symbol binding recorded with binding_state={}; raw dump bytes are not exported by default.",
        binding_state.as_str()
    )
}
