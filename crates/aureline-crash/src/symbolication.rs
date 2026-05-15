//! Exact-build local symbolication for crash envelopes.
//!
//! The alpha symbolicator consumes an already loaded in-tree symbol/source-map
//! record. It does not call platform debuggers, read raw dump bytes, or guess
//! at neighboring builds. Every lookup is tied to the crash envelope's primary
//! exact-build identity.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::incident_trail::{
    CrashDumpManifest, CrashEnvelope, CrashFrame, CrashModule, SymbolicatedModuleResult,
    SymbolicationReport,
};

/// Record-kind tag emitted by the alpha local symbolicator.
pub const SYMBOLICATION_REPORT_RECORD_KIND: &str = "symbolication_smoke_report";

/// In-tree symbol/source-map file used for exact-build symbolication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InTreeSymbolFile {
    /// Symbol file schema version.
    pub schema_version: u32,
    /// Producer record-kind tag.
    pub record_kind: String,
    /// Optional fixture id used by tests and support drills.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixture_id: Option<String>,
    /// Stable symbolication report ref to emit when this file resolves a crash.
    pub symbolication_report_ref: String,
    /// RFC 3339 UTC timestamp to place on the emitted report.
    pub generated_at: String,
    /// Runtime exact-build identity this symbol file was generated for.
    pub runtime_identity_ref: String,
    /// Support bundle ref selected by the local symbolication path.
    pub support_bundle_ref: String,
    /// Optional release evidence packet ref for support/release joins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_evidence_packet_ref: Option<String>,
    /// Optional release claim refs for support/release joins.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claim_row_refs: Vec<String>,
    /// Optional retention seed ref shared with support/export manifests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_seed_ref: Option<String>,
    /// Per-module symbol or source-map entries.
    pub modules: Vec<InTreeSymbolModule>,
}

/// Per-module symbol/source-map entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InTreeSymbolModule {
    /// Stable module id that must match the crash envelope.
    pub module_id: String,
    /// Module class such as `native_binary` or `web_bundle`.
    pub module_kind: String,
    /// Exact-build identity for the symbol/source-map artifact.
    pub symbolication_identity_ref: String,
    /// Optional crash-symbol archive identity for native modules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_archive_identity_ref: Option<String>,
    /// Symbol tag matched by the local symbolicator.
    pub matched_symbol_tag: String,
    /// Resolvable frames for this module.
    pub frames: Vec<InTreeSymbolFrame>,
}

/// One frame entry in an in-tree symbol/source-map file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InTreeSymbolFrame {
    /// Stable frame index within the captured stack.
    pub frame_index: u32,
    /// Native instruction address for binary frames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Generated source location for source-map frames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_location: Option<String>,
    /// Symbol name after resolution.
    pub symbol_name: String,
    /// Original source location after source-map resolution.
    pub source_location: String,
    /// Redaction-safe frame summary safe for support bundles.
    pub resolved_frame_summary: String,
}

/// Inputs for one exact-build local symbolication run.
#[derive(Debug, Clone, Copy)]
pub struct ExactBuildSymbolicationInput<'a> {
    /// Synthetic or captured crash envelope.
    pub crash_envelope: &'a CrashEnvelope,
    /// Metadata-only dump manifest for the same crash.
    pub crash_dump_manifest: &'a CrashDumpManifest,
    /// In-tree symbol/source-map file to resolve frames.
    pub symbol_file: &'a InTreeSymbolFile,
}

/// Errors raised by exact-build local symbolication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactBuildSymbolicationError {
    /// The crash envelope did not name the symbol file's runtime identity.
    PrimaryIdentityMismatch {
        /// Runtime exact-build identity expected by the symbol file.
        expected: String,
        /// Exact-build identity found in the crash envelope.
        actual: String,
    },
    /// The dump manifest did not name the crash envelope's runtime identity.
    DumpIdentityMismatch {
        /// Exact-build identity expected by the crash envelope.
        expected: String,
        /// Exact-build identity found in the dump manifest.
        actual: String,
    },
    /// A crash module did not belong to the runtime exact-build family.
    ModuleIdentityMismatch {
        /// Module id whose identity was rejected.
        module_id: String,
        /// Runtime exact-build family expected by the symbol file.
        expected_family: String,
        /// Exact-build identity found on the module.
        actual: String,
    },
    /// A symbol/source-map module did not belong to the runtime family.
    SymbolFileIdentityMismatch {
        /// Module id whose symbol file identity was rejected.
        module_id: String,
        /// Runtime exact-build family expected by the symbol file.
        expected_family: String,
        /// Exact-build identity found on the symbol file module.
        actual: String,
    },
    /// A crash module had no matching symbol/source-map module.
    MissingModuleSymbols {
        /// Module id missing from the in-tree symbol file.
        module_id: String,
    },
    /// A symbol/source-map module had a different module kind.
    ModuleKindMismatch {
        /// Module id with a kind mismatch.
        module_id: String,
        /// Module kind in the crash envelope.
        envelope_kind: String,
        /// Module kind in the symbol/source-map file.
        symbol_kind: String,
    },
    /// A crash module carried no faulting frames.
    MissingFaultingFrames {
        /// Module id with no captured frames.
        module_id: String,
    },
    /// A faulting frame had no exact entry in the symbol/source-map file.
    MissingFrameMapping {
        /// Module id whose frame could not be resolved.
        module_id: String,
        /// Frame index that could not be resolved.
        frame_index: u32,
    },
    /// A symbol tag did not match the module identity captured in the crash.
    SymbolTagMismatch {
        /// Module id whose symbol tag was rejected.
        module_id: String,
        /// Expected symbol tag suffix derived from the crash module identity.
        expected_suffix: String,
        /// Symbol tag found in the symbol/source-map file.
        actual: String,
    },
}

impl fmt::Display for ExactBuildSymbolicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrimaryIdentityMismatch { expected, actual } => write!(
                f,
                "crash envelope exact-build identity {actual} does not match runtime identity {expected}"
            ),
            Self::DumpIdentityMismatch { expected, actual } => write!(
                f,
                "crash dump exact-build identity {actual} does not match crash envelope identity {expected}"
            ),
            Self::ModuleIdentityMismatch {
                module_id,
                expected_family,
                actual,
            } => write!(
                f,
                "module {module_id} exact-build identity {actual} is outside runtime family {expected_family}"
            ),
            Self::SymbolFileIdentityMismatch {
                module_id,
                expected_family,
                actual,
            } => write!(
                f,
                "symbol file for module {module_id} uses identity {actual} outside runtime family {expected_family}"
            ),
            Self::MissingModuleSymbols { module_id } => {
                write!(f, "no in-tree symbols exist for module {module_id}")
            }
            Self::ModuleKindMismatch {
                module_id,
                envelope_kind,
                symbol_kind,
            } => write!(
                f,
                "module {module_id} kind mismatch: envelope={envelope_kind} symbols={symbol_kind}"
            ),
            Self::MissingFaultingFrames { module_id } => {
                write!(f, "module {module_id} has no captured faulting frames")
            }
            Self::MissingFrameMapping {
                module_id,
                frame_index,
            } => write!(
                f,
                "module {module_id} frame {frame_index} has no exact in-tree symbol mapping"
            ),
            Self::SymbolTagMismatch {
                module_id,
                expected_suffix,
                actual,
            } => write!(
                f,
                "module {module_id} symbol tag {actual} does not end with expected suffix {expected_suffix}"
            ),
        }
    }
}

impl std::error::Error for ExactBuildSymbolicationError {}

/// Symbolicate a crash envelope using exact-build in-tree symbols.
///
/// # Errors
///
/// Returns [`ExactBuildSymbolicationError`] when the crash, dump, or symbol
/// file identities do not match exactly, when a module cannot be resolved, or
/// when a captured frame is absent from the in-tree symbol map.
pub fn symbolicate_exact_build(
    input: ExactBuildSymbolicationInput<'_>,
) -> Result<SymbolicationReport, ExactBuildSymbolicationError> {
    let crash_envelope = input.crash_envelope;
    let crash_dump_manifest = input.crash_dump_manifest;
    let symbol_file = input.symbol_file;

    if crash_envelope.primary_exact_build_identity_ref != symbol_file.runtime_identity_ref {
        return Err(ExactBuildSymbolicationError::PrimaryIdentityMismatch {
            expected: symbol_file.runtime_identity_ref.clone(),
            actual: crash_envelope.primary_exact_build_identity_ref.clone(),
        });
    }

    if crash_dump_manifest.primary_exact_build_identity_ref
        != crash_envelope.primary_exact_build_identity_ref
    {
        return Err(ExactBuildSymbolicationError::DumpIdentityMismatch {
            expected: crash_envelope.primary_exact_build_identity_ref.clone(),
            actual: crash_dump_manifest.primary_exact_build_identity_ref.clone(),
        });
    }

    let symbol_modules = symbol_file
        .modules
        .iter()
        .map(|module| (module.module_id.as_str(), module))
        .collect::<BTreeMap<_, _>>();
    let mut module_results = Vec::with_capacity(crash_envelope.modules.len());

    for module in &crash_envelope.modules {
        ensure_module_identity_matches(module, &symbol_file.runtime_identity_ref)?;
        let symbol_module = symbol_modules
            .get(module.module_id.as_str())
            .ok_or_else(|| ExactBuildSymbolicationError::MissingModuleSymbols {
                module_id: module.module_id.clone(),
            })?;
        ensure_symbol_module_matches(module, symbol_module, &symbol_file.runtime_identity_ref)?;
        ensure_symbol_tag_matches(module, symbol_module)?;

        if module.faulting_frames.is_empty() {
            return Err(ExactBuildSymbolicationError::MissingFaultingFrames {
                module_id: module.module_id.clone(),
            });
        }

        let mut resolved_frame_summary = Vec::with_capacity(module.faulting_frames.len());
        for frame in &module.faulting_frames {
            let symbol_frame = matching_symbol_frame(module, frame, symbol_module)?;
            resolved_frame_summary.push(symbol_frame.resolved_frame_summary.clone());
        }

        module_results.push(SymbolicatedModuleResult {
            module_id: module.module_id.clone(),
            module_kind: module.module_kind.clone(),
            mapping_state: "exact".into(),
            runtime_identity_ref: symbol_file.runtime_identity_ref.clone(),
            symbolication_identity_ref: Some(symbol_module.symbolication_identity_ref.clone()),
            support_archive_identity_ref: symbol_module.support_archive_identity_ref.clone(),
            matched_symbol_tag: Some(symbol_module.matched_symbol_tag.clone()),
            unresolved_reason: None,
            resolved_frame_summary,
        });
    }

    Ok(SymbolicationReport {
        schema_version: 1,
        record_kind: SYMBOLICATION_REPORT_RECORD_KIND.to_owned(),
        fixture_id: symbol_file.fixture_id.clone(),
        symbolication_report_ref: symbol_file.symbolication_report_ref.clone(),
        generated_at: symbol_file.generated_at.clone(),
        crash_envelope_ref: crash_envelope.crash_envelope_ref.clone(),
        primary_exact_build_identity_ref: symbol_file.runtime_identity_ref.clone(),
        result_state: "exact_match".into(),
        module_results,
        crash_dump_ref: crash_dump_manifest.crash_dump_ref.clone(),
        support_bundle_ref: symbol_file.support_bundle_ref.clone(),
        release_evidence_packet_ref: symbol_file.release_evidence_packet_ref.clone(),
        claim_row_refs: symbol_file.claim_row_refs.clone(),
        retention_seed_ref: symbol_file.retention_seed_ref.clone(),
        notes:
            "Exact-build in-tree symbols resolved every captured frame; raw dump bytes were not read."
                .into(),
    })
}

fn ensure_module_identity_matches(
    module: &CrashModule,
    runtime_identity_ref: &str,
) -> Result<(), ExactBuildSymbolicationError> {
    if exact_build_family_matches(runtime_identity_ref, &module.exact_build_identity_ref) {
        return Ok(());
    }

    Err(ExactBuildSymbolicationError::ModuleIdentityMismatch {
        module_id: module.module_id.clone(),
        expected_family: runtime_identity_ref.to_owned(),
        actual: module.exact_build_identity_ref.clone(),
    })
}

fn ensure_symbol_module_matches(
    module: &CrashModule,
    symbol_module: &InTreeSymbolModule,
    runtime_identity_ref: &str,
) -> Result<(), ExactBuildSymbolicationError> {
    if module.module_kind != symbol_module.module_kind {
        return Err(ExactBuildSymbolicationError::ModuleKindMismatch {
            module_id: module.module_id.clone(),
            envelope_kind: module.module_kind.clone(),
            symbol_kind: symbol_module.module_kind.clone(),
        });
    }

    if exact_build_family_matches(
        runtime_identity_ref,
        &symbol_module.symbolication_identity_ref,
    ) && symbol_module
        .support_archive_identity_ref
        .as_ref()
        .map_or(true, |identity_ref| {
            exact_build_family_matches(runtime_identity_ref, identity_ref)
        })
    {
        return Ok(());
    }

    let actual = symbol_module
        .support_archive_identity_ref
        .clone()
        .filter(|identity_ref| !exact_build_family_matches(runtime_identity_ref, identity_ref))
        .unwrap_or_else(|| symbol_module.symbolication_identity_ref.clone());
    Err(ExactBuildSymbolicationError::SymbolFileIdentityMismatch {
        module_id: module.module_id.clone(),
        expected_family: runtime_identity_ref.to_owned(),
        actual,
    })
}

fn ensure_symbol_tag_matches(
    module: &CrashModule,
    symbol_module: &InTreeSymbolModule,
) -> Result<(), ExactBuildSymbolicationError> {
    let expected_suffix = module.module_identity.as_ref().and_then(|identity| {
        identity
            .build_id
            .as_ref()
            .or(identity.source_map_digest.as_ref())
    });

    let Some(expected_suffix) = expected_suffix else {
        return Ok(());
    };

    if symbol_module.matched_symbol_tag.ends_with(expected_suffix) {
        return Ok(());
    }

    Err(ExactBuildSymbolicationError::SymbolTagMismatch {
        module_id: module.module_id.clone(),
        expected_suffix: expected_suffix.clone(),
        actual: symbol_module.matched_symbol_tag.clone(),
    })
}

fn matching_symbol_frame<'a>(
    module: &CrashModule,
    frame: &CrashFrame,
    symbol_module: &'a InTreeSymbolModule,
) -> Result<&'a InTreeSymbolFrame, ExactBuildSymbolicationError> {
    symbol_module
        .frames
        .iter()
        .find(|symbol_frame| frame_matches(frame, symbol_frame))
        .ok_or_else(|| ExactBuildSymbolicationError::MissingFrameMapping {
            module_id: module.module_id.clone(),
            frame_index: frame.frame_index,
        })
}

fn frame_matches(frame: &CrashFrame, symbol_frame: &InTreeSymbolFrame) -> bool {
    if frame.frame_index != symbol_frame.frame_index {
        return false;
    }

    if let Some(address) = &frame.address {
        return symbol_frame.address.as_ref() == Some(address);
    }

    if let Some(generated_location) = &frame.generated_location {
        return symbol_frame.generated_location.as_ref() == Some(generated_location);
    }

    frame.symbol_hint == symbol_frame.symbol_name
}

fn exact_build_family_matches(primary_exact_build_ref: &str, candidate: &str) -> bool {
    candidate == primary_exact_build_ref
        || candidate
            .strip_prefix(primary_exact_build_ref)
            .is_some_and(|suffix| suffix.starts_with(':'))
}
