//! Generated-artifact lineage truth.
//!
//! This module is the canonical home for the rules that decide whether a
//! workspace-relative path is a *generated* artifact (lockfile, build output,
//! generated source sibling, vendored snapshot) and, when known, which
//! source-canonical artifact it derives from.
//!
//! Explorer and search surfaces consume the published [`LineageHintRecord`] so
//! they never imply that a generated artifact is the canonical edit target
//! when the lineage is known. The detection vocabulary is intentionally
//! narrow: M1 only covers cases the workspace can detect *safely* from the
//! relative path alone — there is no build-graph reconstruction here, and a
//! missing rule must produce no hint rather than a guess.
//!
//! Failure-drill rule: when no rule matches, the detector returns `None`. The
//! shell explorer/search surfaces MUST keep rendering the row exactly as they
//! did before — the absence of a hint is not a license to invent one.
//!
//! Primary sources:
//! - `docs/workspace/generated_artifact_lineage.md`
//! - `fixtures/workspace/generated_artifact_cases/*.json`

pub mod lineage;

pub use lineage::{
    default_catalog, detect_lineage, GeneratedArtifactCatalog, GeneratedArtifactClass,
    GeneratedArtifactRule, LineageFreshnessClass, LineageHintRecord, LineageHintRecordKind,
    LineageHintSchemaVersion, RuleMatcher, SourceCanonicalLink,
};
