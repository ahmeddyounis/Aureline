//! Inline tests for the M5 component-gallery evidence pack.

use std::collections::BTreeSet;

use super::*;

use crate::m5_component_manifest::{seeded_m5_component_manifest_package, M5ComponentKind};
use crate::m5_host_primitive::seeded_m5_host_primitive_library;

fn canonical() -> M5EvidencePack {
    seeded_m5_evidence_pack()
}

#[test]
fn canonical_pack_validates() {
    let pack = canonical();
    assert!(pack.validate().is_empty(), "{:?}", pack.validate());
    assert_eq!(pack.record_kind, M5_EVIDENCE_PACK_RECORD_KIND);
    assert_eq!(pack.pack_id, M5_EVIDENCE_PACK_ID);
    assert_eq!(pack.pack_version, M5_EVIDENCE_PACK_VERSION);
}

#[test]
fn pack_publishes_one_component_per_kind() {
    let pack = canonical();
    for kind in M5ComponentKind::ALL {
        let component = pack
            .component(kind)
            .unwrap_or_else(|| panic!("missing {}", kind.as_str()));
        assert_eq!(component.component_kind, kind);
        assert_eq!(
            component.component_id,
            format!("design-system:component:{}", kind.as_str())
        );
        assert_eq!(
            component.primitive_id,
            format!("design-system:primitive:{}", kind.as_str())
        );
    }
    assert_eq!(pack.components.len(), M5ComponentKind::ALL.len());
}

#[test]
fn every_component_renders_the_full_canonical_state_set() {
    let canonical_states: BTreeSet<CanonicalStateClass> =
        CanonicalStateClass::required().iter().copied().collect();
    for component in &canonical().components {
        let rendered: BTreeSet<CanonicalStateClass> =
            component.scenes.iter().map(|s| s.state).collect();
        assert_eq!(
            rendered, canonical_states,
            "{} does not render the full canonical state set",
            component.component_id
        );
        assert!(
            component.scenes.iter().any(|s| s.mandatory),
            "{} has no mandatory scene",
            component.component_id
        );
        assert!(component.coverage_complete());
    }
}

#[test]
fn every_scene_captures_all_appearance_variants_in_one_pack() {
    // High-contrast, reduced-motion, and zoom evidence live in the same pack as the normal-theme
    // baseline — never in a separate, easily-stale folder.
    for component in &canonical().components {
        for scene in &component.scenes {
            let kinds: BTreeSet<M5EvidenceVariantKind> =
                scene.variants.iter().map(|v| v.variant_kind).collect();
            for required in M5EvidenceVariantKind::ALL {
                assert!(
                    kinds.contains(&required),
                    "{} {:?} is missing the {} variant",
                    component.component_id,
                    scene.state,
                    required.as_str()
                );
            }
            assert_eq!(scene.high_contrast_variants().len(), 2);
            assert!(scene
                .variant(M5EvidenceVariantKind::ReducedMotion)
                .is_some());
            assert!(scene
                .variants
                .iter()
                .any(|v| v.variant_kind.is_zoom() && v.zoom_percent > 100));
        }
    }
}

#[test]
fn every_variant_axis_matches_its_kind() {
    for component in &canonical().components {
        for scene in &component.scenes {
            for variant in &scene.variants {
                assert_eq!(variant.theme_class, variant.variant_kind.theme_class());
                assert_eq!(
                    variant.motion_posture,
                    variant.variant_kind.motion_posture()
                );
                assert_eq!(variant.zoom_percent, variant.variant_kind.zoom_percent());
            }
        }
    }
}

#[test]
fn high_contrast_variants_use_high_contrast_themes() {
    use aureline_ui::tokens::ThemeClass;
    for component in &canonical().components {
        for scene in &component.scenes {
            let hc_dark = scene
                .variant(M5EvidenceVariantKind::HighContrastDark)
                .expect("hc dark present");
            assert_eq!(hc_dark.theme_class, ThemeClass::HighContrastDark);
            let hc_light = scene
                .variant(M5EvidenceVariantKind::HighContrastLight)
                .expect("hc light present");
            assert_eq!(hc_light.theme_class, ThemeClass::HighContrastLight);
        }
    }
}

#[test]
fn reduced_motion_variant_uses_reduced_posture() {
    use aureline_ui::themes::AccessibilityPostureClass;
    for component in &canonical().components {
        for scene in &component.scenes {
            let reduced = scene
                .variant(M5EvidenceVariantKind::ReducedMotion)
                .expect("reduced-motion present");
            assert_eq!(
                reduced.motion_posture,
                AccessibilityPostureClass::MotionReduced
            );
        }
    }
}

#[test]
fn every_scene_stays_labelled_and_non_color_only() {
    for component in &canonical().components {
        for scene in &component.scenes {
            assert!(
                scene.non_color_cues.contains(&NonColorCueClass::LabelText),
                "{} {:?} is not labelled",
                component.component_id,
                scene.state
            );
            for variant in &scene.variants {
                assert!(variant.non_color_meaning_present);
            }
        }
    }
}

#[test]
fn scenes_are_rendered_from_the_host_primitive_render_plans() {
    // The evidence is reproducible from the checked-in primitive library: every scene's parts, cues,
    // and interactivity come straight from the primitive's render plan for that state.
    let library = seeded_m5_host_primitive_library();
    for component in &canonical().components {
        let primitive = library
            .primitive(component.component_kind)
            .expect("primitive present");
        for scene in &component.scenes {
            let plan = primitive.state_plan(scene.state).expect("plan present");
            assert_eq!(scene.rendered_parts, plan.rendered_parts);
            assert_eq!(scene.non_color_cues, plan.non_color_cues);
            assert_eq!(scene.interactive, plan.interactive);
            assert_eq!(scene.mandatory, plan.mandatory);
        }
    }
}

#[test]
fn owning_identity_comes_from_the_component_manifest() {
    let manifests = seeded_m5_component_manifest_package();
    for component in &canonical().components {
        let manifest = manifests
            .manifest(component.component_kind)
            .expect("manifest present");
        assert_eq!(component.component_id, manifest.component_id);
        assert_eq!(component.owner_role, manifest.lifecycle.owner_role);
        assert!(!component.owner_role.trim().is_empty());
    }
}

#[test]
fn baseline_digests_are_deterministic_and_unique_per_variant() {
    let pack = canonical();
    let again = canonical();
    // Deterministic: the same descriptor mints the same digest every run.
    assert_eq!(pack, again);

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for component in &pack.components {
        for scene in &component.scenes {
            for variant in &scene.variants {
                assert!(variant.baseline_digest.starts_with("fnv1a64:"));
                assert!(
                    seen.insert(variant.baseline_digest.as_str()),
                    "duplicate baseline digest {}",
                    variant.baseline_digest
                );
            }
        }
    }
}

#[test]
fn a_content_change_changes_the_baseline_digest() {
    // The digest is a real visual-diff baseline: mutating a rendered part changes it, so a checked-in
    // baseline cannot silently drift from the scene it certifies.
    let mut pack = canonical();
    let scene = &mut pack.components[0].scenes[0];
    let before = scene.variants[0].baseline_digest.clone();
    scene.rendered_parts.push("injected_part".to_owned());
    // Recompute via validate, which compares stored digests against the canonical descriptor.
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::VariantIncomplete));
    // The recomputed digest differs from the stored one.
    let input = SceneDigestInput {
        state: pack.components[0].scenes[0].state,
        status_message_id: &pack.components[0].scenes[0].status_message_id,
        rendered_parts: &pack.components[0].scenes[0].rendered_parts,
        non_color_cues: &pack.components[0].scenes[0].non_color_cues,
        interactive: pack.components[0].scenes[0].interactive,
    };
    let recomputed = variant_baseline_digest(
        &pack.components[0].component_id,
        &input,
        M5EvidenceVariantKind::NormalDark,
    );
    assert_ne!(before, recomputed);
}

#[test]
fn canonical_pack_is_all_current_and_certified() {
    let pack = canonical();
    for component in &pack.components {
        assert_eq!(
            component.freshness,
            M5EvidenceFreshness::Current,
            "{} is not current",
            component.component_id
        );
        assert_eq!(component.claim_gate, M5EvidenceClaimGate::Certified);
    }
    assert!(pack.stale_components().is_empty());
    assert!(pack.narrowed_components().is_empty());
    assert_eq!(pack.pack_claim_gate(), M5EvidenceClaimGate::Certified);
}

#[test]
fn stale_evidence_narrows_the_owning_components_claim() {
    // Re-evaluating at a later date narrows the older components (captured first) while the
    // freshly-captured ones stay certified — narrowing is per owning component, not all-or-nothing.
    let stale = seeded_m5_evidence_pack_stale_narrowed();
    assert!(stale.validate().is_empty(), "{:?}", stale.validate());

    assert!(
        !stale.narrowed_components().is_empty(),
        "stale pack should narrow at least one component"
    );
    assert!(
        stale
            .components
            .iter()
            .any(|c| c.claim_gate == M5EvidenceClaimGate::Certified),
        "stale pack should keep at least one component certified"
    );
    for component in &stale.components {
        match component.freshness {
            M5EvidenceFreshness::Stale => {
                assert_eq!(component.claim_gate, M5EvidenceClaimGate::Narrowed);
            }
            M5EvidenceFreshness::Current => {
                assert_eq!(component.claim_gate, M5EvidenceClaimGate::Certified);
            }
        }
    }
    assert_eq!(stale.pack_claim_gate(), M5EvidenceClaimGate::Narrowed);
}

#[test]
fn reevaluate_recomputes_freshness_without_touching_digests() {
    let pack = canonical();
    let later = pack.reevaluate(M5_EVIDENCE_PACK_STALE_EVALUATED_AT);
    for (before, after) in pack.components.iter().zip(&later.components) {
        assert_eq!(before.captured_at, after.captured_at);
        assert_eq!(before.scenes, after.scenes, "digests/scenes must not move");
        assert_eq!(after.evaluated_at, M5_EVIDENCE_PACK_STALE_EVALUATED_AT);
    }
}

#[test]
fn evidence_freshness_is_computed_from_dates_and_window() {
    assert_eq!(
        evidence_freshness("2026-06-01T00:00:00Z", "2026-06-30T00:00:00Z", 90),
        M5EvidenceFreshness::Current
    );
    assert_eq!(
        evidence_freshness("2026-01-01T00:00:00Z", "2026-06-30T00:00:00Z", 90),
        M5EvidenceFreshness::Stale
    );
    // Boundary: exactly the window is still current.
    assert_eq!(
        evidence_freshness("2026-01-01", "2026-04-01", 90),
        M5EvidenceFreshness::Current
    );
    // One day past the window is stale.
    assert_eq!(
        evidence_freshness("2026-01-01", "2026-04-02", 90),
        M5EvidenceFreshness::Stale
    );
    // Evidence captured after the evaluation date is current, not stale.
    assert_eq!(
        evidence_freshness("2026-07-01", "2026-06-01", 90),
        M5EvidenceFreshness::Current
    );
    // Unparseable dates are conservatively stale.
    assert_eq!(
        evidence_freshness("not-a-date", "2026-06-01", 90),
        M5EvidenceFreshness::Stale
    );
}

#[test]
fn focus_is_visible_in_every_variant_of_an_interactive_scene() {
    for component in &canonical().components {
        for scene in &component.scenes {
            if scene.interactive {
                for variant in &scene.variants {
                    assert!(
                        variant.focus_visible,
                        "{} interactive {:?} variant {} lacks focus",
                        component.component_id,
                        scene.state,
                        variant.variant_kind.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn export_import_round_trips_and_revalidates() {
    let pack = canonical();
    let json = pack.export_safe_json();
    let imported = M5EvidencePack::from_json(&json).expect("imports");
    assert_eq!(imported, pack);
    assert!(imported.validate().is_empty());
}

#[test]
fn release_packet_projects_one_summary_per_component() {
    let pack = canonical();
    let release = pack.release_packet();
    assert_eq!(release.pack_version, "1.0.0");
    assert_eq!(release.total_components, M5ComponentKind::ALL.len() as u32);
    assert_eq!(
        release.component_summaries.len(),
        M5ComponentKind::ALL.len()
    );
    assert_eq!(release.total_scenes, pack.total_scenes() as u32);
    assert_eq!(release.total_variants, pack.total_variants() as u32);
    assert_eq!(
        release.certified_component_count,
        M5ComponentKind::ALL.len() as u32
    );
    assert_eq!(release.narrowed_component_count, 0);
    assert_eq!(release.blocked_component_count, 0);
    assert_eq!(release.pack_claim_gate, M5EvidenceClaimGate::Certified);
    for (component, summary) in pack.components.iter().zip(&release.component_summaries) {
        assert_eq!(summary.component_kind, component.component_kind);
        assert_eq!(summary.component_id, component.component_id);
        assert_eq!(summary.owner_role, component.owner_role);
        assert_eq!(summary.scene_count, component.scenes.len() as u32);
        assert_eq!(summary.variant_count, component.total_variants() as u32);
        assert_eq!(
            summary.high_contrast_variant_count,
            2 * component.scenes.len() as u32
        );
        assert_eq!(
            summary.reduced_motion_variant_count,
            component.scenes.len() as u32
        );
        assert_eq!(
            summary.zoom_variant_count,
            2 * component.scenes.len() as u32
        );
        assert_eq!(summary.freshness, component.freshness);
        assert_eq!(summary.claim_gate, component.claim_gate);
    }
}

#[test]
fn release_packet_carries_stale_narrowing_for_review() {
    let release = seeded_m5_evidence_pack_stale_narrowed().release_packet();
    assert!(release.narrowed_component_count > 0);
    assert_eq!(release.pack_claim_gate, M5EvidenceClaimGate::Narrowed);
    assert_eq!(
        release.narrowed_component_count + release.certified_component_count,
        release.total_components
    );
}

#[test]
fn validation_rejects_bad_pack_version() {
    let mut pack = canonical();
    pack.pack_version = "1.0".to_owned();
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::BadPackVersion));
}

#[test]
fn validation_rejects_duplicate_component_kind() {
    let mut pack = canonical();
    let extra = pack.components[0].clone();
    pack.components.push(extra);
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::DuplicateComponentKind));
}

#[test]
fn validation_rejects_missing_component_kind() {
    let mut pack = canonical();
    pack.components.pop();
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::RequiredComponentKindMissing));
}

#[test]
fn validation_rejects_incomplete_scene_coverage() {
    let mut pack = canonical();
    pack.components[0].scenes.pop();
    // Removing a scene leaves the component's claim gate stale relative to coverage.
    let violations = pack.validate();
    assert!(violations.contains(&M5EvidencePackViolation::SceneCoverageIncomplete));
}

#[test]
fn validation_rejects_missing_appearance_variant() {
    let mut pack = canonical();
    // Drop the high-contrast dark variant from one scene.
    pack.components[0].scenes[0]
        .variants
        .retain(|v| v.variant_kind != M5EvidenceVariantKind::HighContrastDark);
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::VariantCoverageIncomplete));
}

#[test]
fn validation_rejects_a_scene_without_a_label_cue() {
    let mut pack = canonical();
    pack.components[0].scenes[0]
        .non_color_cues
        .retain(|c| *c != NonColorCueClass::LabelText);
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::SceneIncomplete));
}

#[test]
fn validation_rejects_a_tampered_baseline_digest() {
    let mut pack = canonical();
    pack.components[0].scenes[0].variants[0].baseline_digest =
        "fnv1a64:0000000000000000".to_owned();
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::VariantIncomplete));
}

#[test]
fn validation_rejects_a_freshness_that_disagrees_with_the_dates() {
    let mut pack = canonical();
    pack.components[0].freshness = M5EvidenceFreshness::Stale;
    let violations = pack.validate();
    assert!(violations.contains(&M5EvidencePackViolation::FreshnessMismatch));
}

#[test]
fn validation_rejects_a_claim_gate_that_disagrees_with_freshness() {
    let mut pack = canonical();
    pack.components[0].claim_gate = M5EvidenceClaimGate::Narrowed;
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::ClaimGateMismatch));
}

#[test]
fn validation_rejects_missing_source_contracts() {
    let mut pack = canonical();
    pack.source_contract_refs = vec!["schemas/design-system/wrong.schema.json".to_owned()];
    assert!(pack
        .validate()
        .contains(&M5EvidencePackViolation::MissingSourceContracts));
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = canonical().export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("authorization"));
    assert!(!json.contains("bearer "));
}

#[test]
fn checked_pack_fixture_matches_seed_and_validates() {
    let from_disk = current_stable_m5_evidence_pack().expect("checked pack validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked evidence pack drifted from the seed builder"
    );
}

#[test]
fn checked_per_component_fixtures_match_seed() {
    let pack = canonical();
    let fixtures: &[(M5ComponentKind, &str)] = &[
        (
            M5ComponentKind::PlaceholderCard,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-placeholder_card.json"
            )),
        ),
        (
            M5ComponentKind::StateBlock,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-state_block.json"
            )),
        ),
        (
            M5ComponentKind::ReviewSheet,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-review_sheet.json"
            )),
        ),
        (
            M5ComponentKind::JobRow,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-job_row.json"
            )),
        ),
        (
            M5ComponentKind::BoundaryBar,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-boundary_bar.json"
            )),
        ),
        (
            M5ComponentKind::FormControl,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-form_control.json"
            )),
        ),
        (
            M5ComponentKind::DenseCollection,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/evidence-dense_collection.json"
            )),
        ),
    ];
    for (kind, raw) in fixtures {
        let from_disk: M5ComponentEvidence =
            serde_json::from_str(raw).expect("per-component fixture parses");
        let seeded = pack.component(*kind).expect("component present");
        assert_eq!(
            &from_disk,
            seeded,
            "checked component fixture for {} drifted from the seed",
            kind.as_str()
        );
    }
}

#[test]
fn checked_release_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/evidence-pack-release.json"
    ));
    let from_disk: M5EvidencePackReleasePacket =
        serde_json::from_str(raw).expect("release packet parses");
    assert_eq!(
        from_disk,
        canonical().release_packet(),
        "checked release packet drifted from the computed release packet"
    );
}
