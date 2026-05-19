//! Fixture-replay tests for capability records and artifact
//! dependency markers.
//!
//! The integration test loads every fixture under
//! `fixtures/capabilities/m3/dependency_markers/` and proves that
//! each persisted artifact round-trips through
//! [`validate_artifact_markers`] without a defect.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_capabilities::{
    catalog_default_capabilities, project_marker_for_host_surface, validate_artifact_markers,
    ArtifactClass, ArtifactDependencyMarker, CapabilityRecord, DependencyClass, HostSurface,
    SupportPromise,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FixtureArtifact {
    artifact_ref: String,
    #[serde(default)]
    artifact_class: Option<String>,
    markers: Vec<ArtifactDependencyMarker>,
    #[serde(default)]
    capability_records: Vec<CapabilityRecord>,
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("capabilities")
        .join("m3")
        .join("dependency_markers")
}

fn read_fixture(path: &Path) -> FixtureArtifact {
    let bytes = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("read fixture {}: {e}", path.display()));
    serde_json::from_str(&bytes)
        .unwrap_or_else(|e| panic!("parse fixture {}: {e}", path.display()))
}

fn fixture_paths() -> Vec<PathBuf> {
    let root = fixture_root();
    assert!(
        root.is_dir(),
        "fixture root missing: {}",
        root.display()
    );
    let mut out = Vec::new();
    for entry in fs::read_dir(&root).expect("read fixture dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out.sort();
    assert!(!out.is_empty(), "no fixtures found under {}", root.display());
    out
}

#[test]
fn every_fixture_validates_against_the_seed_catalog() {
    let catalog = catalog_default_capabilities();
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        let mut combined = catalog.clone();
        combined.extend(fixture.capability_records.iter().cloned());

        validate_artifact_markers(&fixture.artifact_ref, &fixture.markers, &combined)
            .unwrap_or_else(|errors| {
                panic!(
                    "fixture {} failed validation:\n{}",
                    path.display(),
                    errors
                        .iter()
                        .map(|e| format!("  - {e}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            });
    }
}

#[test]
fn fixtures_cover_every_artifact_class_and_dependency_class() {
    let mut seen_artifact_classes = BTreeMap::new();
    let mut seen_dependency_classes = BTreeMap::new();
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        for marker in &fixture.markers {
            seen_artifact_classes
                .entry(marker.artifact_class.as_str())
                .and_modify(|c| *c += 1)
                .or_insert(1usize);
            seen_dependency_classes
                .entry(marker.dependency_class.as_str())
                .and_modify(|c| *c += 1)
                .or_insert(1usize);
        }
        if let Some(expected) = fixture.artifact_class.as_ref() {
            for marker in &fixture.markers {
                assert_eq!(
                    marker.artifact_class.as_str(),
                    expected.as_str(),
                    "{} carries marker for wrong artifact_class",
                    path.display()
                );
            }
        }
    }
    for class in [
        ArtifactClass::SettingsExport,
        ArtifactClass::Profile,
        ArtifactClass::WorkflowBundle,
        ArtifactClass::PortableStatePackage,
        ArtifactClass::Recipe,
        ArtifactClass::SavedView,
        ArtifactClass::MigrationPacket,
        ArtifactClass::SupportExport,
        ArtifactClass::SyncArtifact,
    ] {
        assert!(
            seen_artifact_classes.contains_key(class.as_str()),
            "no fixture exercised artifact_class {}",
            class.as_str()
        );
    }
    for class in [
        DependencyClass::Labs,
        DependencyClass::Preview,
        DependencyClass::BetaOnly,
        DependencyClass::PolicyGated,
        DependencyClass::HostSpecific,
    ] {
        assert!(
            seen_dependency_classes.contains_key(class.as_str()),
            "no fixture exercised dependency_class {}",
            class.as_str()
        );
    }
}

#[test]
fn projections_share_vocabulary_across_every_host_surface() {
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        for marker in &fixture.markers {
            let mut tokens = Vec::new();
            for surface in [
                HostSurface::SettingsInspector,
                HostSurface::ImportReviewSheet,
                HostSurface::BundleDetailPage,
                HostSurface::DowngradeFlow,
                HostSurface::HeadlessCliInspect,
                HostSurface::DocsHelpPage,
            ] {
                let projection = project_marker_for_host_surface(marker, surface);
                tokens.push((
                    projection.dependency_class.clone(),
                    projection.required_lifecycle_state.clone(),
                    projection.support_promise.clone(),
                    projection.effect_on_import.clone(),
                ));
                assert!(
                    projection.user_authored_data_preserved,
                    "{}: marker {} dropped user data on {}",
                    path.display(),
                    marker.marker_id,
                    surface.as_str()
                );
            }
            let first = tokens[0].clone();
            for token in &tokens[1..] {
                assert_eq!(
                    token, &first,
                    "{}: marker {} drifted vocabulary across surfaces",
                    path.display(),
                    marker.marker_id
                );
            }
        }
    }
}

#[test]
fn fixtures_only_carry_supported_support_promises() {
    let allowed = [
        SupportPromise::BestEffort,
        SupportPromise::CommunitySupported,
        SupportPromise::StandardSupport,
        SupportPromise::ExtendedSupport,
        SupportPromise::OperatorOnly,
        SupportPromise::NoSupport,
    ];
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        for marker in &fixture.markers {
            assert!(
                allowed.contains(&marker.support_promise),
                "{}: marker {} carries unsupported support_promise {}",
                path.display(),
                marker.marker_id,
                marker.support_promise.as_str()
            );
        }
    }
}
