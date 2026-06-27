//! Inline tests for the M5 host-rendered primitive library.

use std::collections::BTreeSet;

use super::*;

use crate::m5_component_manifest::{seeded_m5_component_manifest_package, M5ComponentKind};
use crate::m5_foundation_package::seeded_m5_foundation_package;

fn canonical() -> M5HostPrimitiveLibrary {
    seeded_m5_host_primitive_library()
}

#[test]
fn canonical_library_validates() {
    let library = canonical();
    assert!(library.validate().is_empty(), "{:?}", library.validate());
    assert_eq!(library.record_kind, M5_HOST_PRIMITIVE_LIBRARY_RECORD_KIND);
    assert_eq!(library.library_id, M5_HOST_PRIMITIVE_LIBRARY_ID);
    assert_eq!(library.library_version, M5_HOST_PRIMITIVE_LIBRARY_VERSION);
}

#[test]
fn library_publishes_one_primitive_per_component_kind() {
    let library = canonical();
    for kind in M5ComponentKind::ALL {
        let primitive = library
            .primitive(kind)
            .unwrap_or_else(|| panic!("missing {}", kind.as_str()));
        assert_eq!(primitive.component_kind, kind);
        assert_eq!(
            primitive.primitive_id,
            format!("design-system:primitive:{}", kind.as_str())
        );
        assert_eq!(
            primitive.component_id,
            format!("design-system:component:{}", kind.as_str())
        );
    }
    assert_eq!(library.primitives.len(), M5ComponentKind::ALL.len());
}

#[test]
fn every_primitive_renders_the_full_canonical_state_set() {
    let canonical_states: BTreeSet<CanonicalStateClass> =
        CanonicalStateClass::required().iter().copied().collect();
    for primitive in &canonical().primitives {
        let rendered: BTreeSet<CanonicalStateClass> = primitive
            .state_render_plans
            .iter()
            .map(|p| p.state)
            .collect();
        assert_eq!(
            rendered, canonical_states,
            "{} does not render the full canonical state set",
            primitive.primitive_id
        );
        assert!(
            !primitive.mandatory_state_plans().is_empty(),
            "{} has no mandatory render plan",
            primitive.primitive_id
        );
    }
}

#[test]
fn every_render_plan_stays_labelled_and_non_color_only() {
    for primitive in &canonical().primitives {
        for plan in &primitive.state_render_plans {
            assert!(
                plan.non_color_cues.contains(&NonColorCueClass::LabelText),
                "{} {:?} is not labelled",
                primitive.primitive_id,
                plan.state
            );
            assert!(
                plan.non_color_cues.len() >= 2,
                "{} {:?} carries no non-color cue beyond the label",
                primitive.primitive_id,
                plan.state
            );
            assert!(!plan.rendered_parts.is_empty());
        }
    }
}

#[test]
fn blocked_state_names_its_constraint_with_a_lock_or_shield() {
    // Blocked states never read as a generic spinner; they carry the lock/shield metaphor so the
    // constraint is named, consistently across every primitive.
    for primitive in &canonical().primitives {
        let blocked = primitive
            .state_plan(CanonicalStateClass::Blocked)
            .expect("blocked plan present");
        assert!(
            blocked
                .non_color_cues
                .contains(&NonColorCueClass::LockOrShieldGlyph),
            "{} blocked state lacks a lock/shield glyph",
            primitive.primitive_id
        );
    }
}

#[test]
fn every_primitive_preserves_the_shared_appearance_vocabulary() {
    let foundation = seeded_m5_foundation_package();
    let density: BTreeSet<&str> = foundation.density_tokens().into_iter().collect();
    let high_contrast: BTreeSet<&str> = foundation.high_contrast_tokens().into_iter().collect();

    for primitive in &canonical().primitives {
        let primitive_density: BTreeSet<&str> = primitive
            .appearance
            .density_classes
            .iter()
            .map(|d| d.token())
            .collect();
        assert_eq!(
            primitive_density, density,
            "{} does not honor the full density vocabulary",
            primitive.primitive_id
        );

        let primitive_contrast: BTreeSet<&str> = primitive
            .appearance
            .contrast_classes
            .iter()
            .map(|c| c.token())
            .collect();
        for token in &high_contrast {
            assert!(
                primitive_contrast.contains(token),
                "{} does not honor high-contrast token {token}",
                primitive.primitive_id
            );
        }

        assert!(primitive.appearance.honors_focus_order);
        assert!(primitive.appearance.honors_keyboard_model);
        assert!(primitive.appearance.honors_high_contrast);
        assert!(primitive.appearance.honors_reduced_motion);
    }
}

#[test]
fn primitives_are_aligned_with_their_component_manifests() {
    let library = canonical();
    let manifests = seeded_m5_component_manifest_package();
    let findings = audit_primitive_manifest_alignment(&library, &manifests);
    assert!(findings.is_empty(), "{findings:?}");
}

#[test]
fn token_references_resolve_to_published_foundation_entries() {
    // Every primitive renders from foundation token references the foundation package publishes, so
    // the primitive wires to the shared tokens rather than feature-local styling.
    let foundation = seeded_m5_foundation_package();
    let published: BTreeSet<String> = foundation
        .families
        .iter()
        .flat_map(|f| f.entries.iter().map(|e| e.entry_id.clone()))
        .collect();
    for primitive in &canonical().primitives {
        for token in &primitive.token_references {
            assert!(
                published.contains(token),
                "{} references unpublished foundation token {token}",
                primitive.primitive_id
            );
        }
    }
}

#[test]
fn every_required_surface_routes_through_exactly_one_primitive() {
    let library = canonical();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (_, consumer) in library.all_consumers() {
        assert!(
            seen.insert(consumer.surface_class.as_str()),
            "{} is served by more than one primitive",
            consumer.surface_class
        );
    }
    for required in REQUIRED_CONSUMER_SURFACES {
        assert!(
            seen.contains(required),
            "required surface {required} does not route through any primitive"
        );
    }
}

#[test]
fn embedded_consumers_inherit_or_carry_a_partial_badge() {
    for primitive in &canonical().primitives {
        for consumer in &primitive.consumers {
            match consumer.posture {
                M5ConformancePosture::InheritedHostRendered => {
                    assert!(consumer.partial_badge_message_id.is_none());
                }
                M5ConformancePosture::ReducedWithPartialBadge => {
                    assert!(
                        consumer.consumer_class.is_embedded_or_extension(),
                        "{} is first-party but declares a reduced posture",
                        consumer.surface_class
                    );
                    let badge = consumer
                        .partial_badge_message_id
                        .as_deref()
                        .expect("reduced consumer carries a partial badge");
                    assert!(badge.starts_with(M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX));
                }
            }
        }
    }
}

#[test]
fn export_import_round_trips_and_revalidates() {
    let library = canonical();
    let json = library.export_safe_json();
    let imported = M5HostPrimitiveLibrary::from_json(&json).expect("imports");
    assert_eq!(imported, library);
    assert!(imported.validate().is_empty());
}

#[test]
fn release_packet_projects_one_summary_per_primitive() {
    let library = canonical();
    let release = library.release_packet();
    assert_eq!(release.library_version, "1.0.0");
    assert_eq!(release.total_primitives, M5ComponentKind::ALL.len() as u32);
    assert_eq!(
        release.primitive_summaries.len(),
        M5ComponentKind::ALL.len()
    );
    for (primitive, summary) in library.primitives.iter().zip(&release.primitive_summaries) {
        assert_eq!(summary.component_kind, primitive.component_kind);
        assert_eq!(summary.primitive_id, primitive.primitive_id);
        assert_eq!(summary.component_id, primitive.component_id);
        assert_eq!(
            summary.state_plan_count,
            primitive.state_render_plans.len() as u32
        );
        assert_eq!(
            summary.mandatory_state_count,
            primitive.mandatory_state_plans().len() as u32
        );
        assert_eq!(
            summary.token_reference_count,
            primitive.token_references.len() as u32
        );
        assert_eq!(summary.consumer_count, primitive.consumers.len() as u32);
        assert_eq!(
            summary.inherited_consumer_count,
            primitive.inherited_consumers().len() as u32
        );
        assert_eq!(
            summary.reduced_consumer_count,
            primitive.reduced_consumers().len() as u32
        );
    }
    let total_consumers: u32 = library
        .primitives
        .iter()
        .map(|p| p.consumers.len() as u32)
        .sum();
    assert_eq!(release.total_consumers, total_consumers);
}

#[test]
fn validation_rejects_bad_library_version() {
    let mut library = canonical();
    library.library_version = "1.0".to_owned();
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::BadLibraryVersion));
}

#[test]
fn validation_rejects_duplicate_primitive_kind() {
    let mut library = canonical();
    let extra = library.primitives[0].clone();
    library.primitives.push(extra);
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::DuplicatePrimitiveKind));
}

#[test]
fn validation_rejects_missing_primitive_kind() {
    let mut library = canonical();
    library.primitives.pop();
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::RequiredPrimitiveKindMissing));
}

#[test]
fn validation_rejects_incomplete_state_plan_coverage() {
    let mut library = canonical();
    library.primitives[0].state_render_plans.pop();
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::StatePlansIncomplete));
}

#[test]
fn validation_rejects_a_state_plan_without_a_label_cue() {
    let mut library = canonical();
    let plan = &mut library.primitives[0].state_render_plans[0];
    plan.non_color_cues
        .retain(|c| *c != NonColorCueClass::LabelText);
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::RenderPlanIncomplete));
}

#[test]
fn validation_rejects_appearance_missing_high_contrast() {
    let mut library = canonical();
    library.primitives[0]
        .appearance
        .contrast_classes
        .retain(|c| *c != ThemeClass::HighContrastDark);
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::AppearanceIncomplete));
}

#[test]
fn validation_rejects_appearance_dropping_reduced_motion_guarantee() {
    let mut library = canonical();
    library.primitives[0].appearance.honors_reduced_motion = false;
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::AppearanceIncomplete));
}

#[test]
fn validation_rejects_reduced_consumer_without_partial_badge() {
    // The masquerade guard: an extension consumer with a reduced posture must carry a partial badge.
    let mut library = canonical();
    let primitive = library
        .primitive(M5ComponentKind::BoundaryBar)
        .expect("boundary bar present")
        .component_kind;
    let target = library
        .primitives
        .iter_mut()
        .find(|p| p.component_kind == primitive)
        .unwrap();
    let reduced = target
        .consumers
        .iter_mut()
        .find(|c| c.posture == M5ConformancePosture::ReducedWithPartialBadge)
        .expect("a reduced consumer exists");
    reduced.partial_badge_message_id = None;
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::PartialBadgeMissing));
}

#[test]
fn validation_rejects_first_party_reduced_posture() {
    let mut library = canonical();
    let first_party = library
        .primitives
        .iter_mut()
        .flat_map(|p| p.consumers.iter_mut())
        .find(|c| c.consumer_class == M5ConsumerClass::FirstParty)
        .expect("a first-party consumer exists");
    first_party.posture = M5ConformancePosture::ReducedWithPartialBadge;
    first_party.partial_badge_message_id = Some(format!(
        "{}consumer.test.partial_badge",
        M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX
    ));
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::FirstPartyCannotReduce));
}

#[test]
fn validation_rejects_duplicate_consumer_surface() {
    let mut library = canonical();
    let dup = library.primitives[1].consumers[0].clone();
    library.primitives[0].consumers.push(dup);
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::DuplicateConsumerSurface));
}

#[test]
fn validation_rejects_missing_required_consumer_surface() {
    let mut library = canonical();
    library.primitives[0].consumers.remove(0);
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::RequiredConsumerSurfaceMissing));
}

#[test]
fn validation_rejects_missing_source_contracts() {
    let mut library = canonical();
    library.source_contract_refs = vec!["schemas/design-system/wrong.schema.json".to_owned()];
    assert!(library
        .validate()
        .contains(&M5HostPrimitiveViolation::MissingSourceContracts));
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
fn checked_library_fixture_matches_seed_and_validates() {
    let from_disk = current_stable_m5_host_primitive_library().expect("checked library validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked host-primitive library drifted from the seed builder"
    );
}

#[test]
fn checked_per_primitive_fixtures_match_seed() {
    let library = canonical();
    let fixtures: &[(M5ComponentKind, &str)] = &[
        (
            M5ComponentKind::PlaceholderCard,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-placeholder_card.json"
            )),
        ),
        (
            M5ComponentKind::StateBlock,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-state_block.json"
            )),
        ),
        (
            M5ComponentKind::ReviewSheet,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-review_sheet.json"
            )),
        ),
        (
            M5ComponentKind::JobRow,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-job_row.json"
            )),
        ),
        (
            M5ComponentKind::BoundaryBar,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-boundary_bar.json"
            )),
        ),
        (
            M5ComponentKind::FormControl,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-form_control.json"
            )),
        ),
        (
            M5ComponentKind::DenseCollection,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-component-gallery/host-primitive-dense_collection.json"
            )),
        ),
    ];
    for (kind, raw) in fixtures {
        let from_disk: M5HostPrimitive =
            serde_json::from_str(raw).expect("per-primitive fixture parses");
        let seeded = library.primitive(*kind).expect("primitive present");
        assert_eq!(
            &from_disk,
            seeded,
            "checked primitive fixture for {} drifted from the seed",
            kind.as_str()
        );
    }
}

#[test]
fn checked_release_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/host-primitive-release.json"
    ));
    let from_disk: M5HostPrimitiveReleasePacket =
        serde_json::from_str(raw).expect("release packet parses");
    assert_eq!(
        from_disk,
        canonical().release_packet(),
        "checked release packet drifted from the computed release packet"
    );
}
