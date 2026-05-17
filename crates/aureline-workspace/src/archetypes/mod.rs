//! Workspace archetype catalog anchors shared by detection and support surfaces.
//!
//! The detector implementation lives in [`crate::archetype_detection`]. This
//! module exposes the governed artifact references and closed baseline
//! vocabulary so shell, support, and future CLI surfaces do not invent parallel
//! catalog paths or state names.

/// Detector-facing catalog that joins marker detection to scorecards and bundles.
pub const ARCHETYPE_DETECTION_MATRIX_REF: &str =
    "artifacts/compat/m3/archetype_detection_matrix.yaml";

/// Scorecard index that owns archetype support and evidence freshness.
pub const ARCHETYPE_SCORECARD_INDEX_REF: &str =
    "artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml";

/// Detection outcomes that the workspace admission route must preserve.
pub const REQUIRED_DETECTION_OUTCOMES: &[&str] = &[
    "certified_archetype_match",
    "probable_archetype",
    "mixed_or_ambiguous_workspace",
    "unknown_or_generic_workspace",
    "restricted_or_policy_blocked",
    "missing_prerequisite",
];

/// Readiness buckets that must remain structurally distinct.
pub const REQUIRED_READINESS_BUCKETS: &[&str] =
    &["blocking_now", "recommended_soon", "optional_later"];

/// Boundary choices required when competing roots or stacks are detected.
pub const MIXED_WORKSPACE_BOUNDARY_CHOICES: &[&str] = &[
    "open_whole_repo",
    "open_probable_project",
    "open_current_folder_only",
    "create_workset_or_slice",
];

/// Same-weight bypasses required when detection recommends setup.
pub const SETUP_RECOMMENDATION_BYPASSES: &[&str] =
    &["set_up_later", "open_minimal", "dismiss_recommendation"];
