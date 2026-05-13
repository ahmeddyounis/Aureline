//! Fixture-backed coverage for launch-language accessibility parity.
//!
//! The packet under `fixtures/accessibility/m2_language_surfaces/` is the
//! canonical truth source for diagnostics, completion assistance, and
//! refactor-preview accessibility review.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_shell::help::language_surface_accessibility::{
    build_language_surface_accessibility_summary_lines,
    validate_language_surface_accessibility_packet, AccessibilityDimensionClass,
    LanguageAssistSurfaceClass, LanguageSurfaceAccessibilityPacket,
};

const LANGUAGE_SURFACE_KNOWN_LIMIT: &str =
    "known_limit:external_alpha.language_surface_accessibility_synthetic_only";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn packet_path() -> PathBuf {
    repo_root().join("fixtures/accessibility/m2_language_surfaces/language_surface_parity.yaml")
}

fn load_packet() -> LanguageSurfaceAccessibilityPacket {
    let path = packet_path();
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn language_surface_accessibility_packet_passes_contract_validator() {
    let packet = load_packet();
    let report = validate_language_surface_accessibility_packet(&packet);

    assert!(report.passed, "{report:#?}");
    assert_eq!(report.checked_surface_count, 3);
    assert!(
        report.findings.is_empty(),
        "passing fixture should not emit findings: {report:#?}"
    );
    assert!(
        packet
            .known_limit_refs
            .iter()
            .any(|known_limit| known_limit == LANGUAGE_SURFACE_KNOWN_LIMIT),
        "packet should cite the language-surface accessibility known limit"
    );

    let surface_classes: BTreeSet<LanguageAssistSurfaceClass> =
        packet.rows.iter().map(|row| row.surface_class).collect();
    for required in [
        LanguageAssistSurfaceClass::Diagnostics,
        LanguageAssistSurfaceClass::CompletionAssist,
        LanguageAssistSurfaceClass::RefactorPreview,
    ] {
        assert!(
            surface_classes.contains(&required),
            "packet should cover {required:?}"
        );
    }

    let required_dimensions: BTreeSet<AccessibilityDimensionClass> =
        packet.required_dimension_classes.iter().copied().collect();
    for required in [
        AccessibilityDimensionClass::Keyboard,
        AccessibilityDimensionClass::ScreenReader,
        AccessibilityDimensionClass::ReducedMotion,
    ] {
        assert!(
            required_dimensions.contains(&required),
            "packet should require {required:?}"
        );
    }

    for row in &packet.rows {
        assert_eq!(
            row.keyboard.pointer_only_action_count, 0,
            "{} must not rely on pointer-only actions",
            row.surface_id
        );
        assert!(
            row.keyboard.route_same_as_pointer
                && row.keyboard.focus_visible
                && row.keyboard.escape_or_cancel_path,
            "{} must keep the keyboard route complete",
            row.surface_id
        );
        assert!(
            row.screen_reader.announces_source_label
                && row.screen_reader.announces_preview_or_apply_posture
                && row.screen_reader.no_color_only_state,
            "{} must expose source, preview posture, and non-color state",
            row.surface_id
        );
        assert!(
            row.reduced_motion.meaning_preserved_without_motion
                && row.reduced_motion.no_motion_only_cue
                && !row.reduced_motion.layout_shift_required,
            "{} must preserve meaning without motion",
            row.surface_id
        );
        assert!(
            row.content_integrity.no_silent_multi_file_mutation,
            "{} must block silent broad mutation",
            row.surface_id
        );
        assert!(
            row.known_limit_refs
                .iter()
                .any(|known_limit| known_limit == LANGUAGE_SURFACE_KNOWN_LIMIT),
            "{} should carry the active language-surface known limit",
            row.surface_id
        );
    }
}

#[test]
fn language_surface_packet_refs_resolve_and_known_limits_are_published() {
    let packet = load_packet();
    let root = repo_root();

    assert_repo_ref_exists(&root, &packet.review_packet_ref);
    assert_repo_ref_exists(&root, &packet.docs_ref);
    assert_repo_ref_exists(&root, &packet.consumer_ref);
    assert_repo_refs_exist(&root, &packet.source_refs);

    for row in &packet.rows {
        assert_repo_refs_exist(&root, &row.upstream_contract_refs);
        assert_repo_refs_exist(&root, &row.upstream_fixture_refs);
        assert_repo_refs_exist(&root, &row.content_integrity.safe_preview_refs);
        assert_repo_refs_exist(&root, &row.content_integrity.preview_or_review_refs);
    }

    let markdown_path = root.join("artifacts/feedback/external_alpha_known_limits.md");
    let markdown = std::fs::read_to_string(&markdown_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", markdown_path.display()));
    let yaml_path = root.join("artifacts/milestones/m2/known_limits_alpha.yaml");
    let yaml = std::fs::read_to_string(&yaml_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", yaml_path.display()));

    for known_limit in &packet.known_limit_refs {
        assert!(
            markdown.contains(known_limit),
            "{} should publish {known_limit}",
            markdown_path.display()
        );
        assert!(
            yaml.contains(known_limit),
            "{} should publish {known_limit}",
            yaml_path.display()
        );
    }

    let summary = build_language_surface_accessibility_summary_lines(&packet);
    assert!(
        summary
            .iter()
            .any(|line| line.contains("Language surface accessibility alpha - passed")),
        "summary should expose pass state: {summary:#?}"
    );
    assert!(
        summary.iter().any(|line| line.contains("Diagnostics")),
        "summary should list the diagnostics surface: {summary:#?}"
    );
    assert!(
        summary
            .iter()
            .any(|line| line.contains("Rename and refactor preview")),
        "summary should list the refactor-preview surface: {summary:#?}"
    );
}

fn assert_repo_refs_exist(root: &Path, refs: &[String]) {
    for reference in refs {
        if let Some(path_ref) = repo_path_ref(reference) {
            assert_repo_ref_exists(root, path_ref);
        }
    }
}

fn assert_repo_ref_exists(root: &Path, reference: &str) {
    let path_ref = repo_path_ref(reference).unwrap_or(reference);
    let path = root.join(path_ref);
    assert!(
        path.exists(),
        "repo ref {reference} should resolve to {}",
        path.display()
    );
}

fn repo_path_ref(reference: &str) -> Option<&str> {
    let path_ref = reference.split('#').next().unwrap_or(reference);
    if path_ref.starts_with("artifacts/")
        || path_ref.starts_with("crates/")
        || path_ref.starts_with("docs/")
        || path_ref.starts_with("fixtures/")
        || path_ref.starts_with("schemas/")
    {
        Some(path_ref)
    } else {
        None
    }
}
