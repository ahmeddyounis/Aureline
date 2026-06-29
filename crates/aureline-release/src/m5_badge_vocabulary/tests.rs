//! Inline tests for the M5 badge vocabulary.

use super::*;

fn canonical() -> M5BadgeVocabulary {
    seeded_m5_badge_vocabulary()
}

#[test]
fn canonical_packet_validates() {
    let packet = canonical();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BADGE_VOCABULARY_PACKET_ID);
    assert_eq!(packet.record_kind, M5_BADGE_VOCABULARY_RECORD_KIND);
    assert_eq!(packet.families.len(), BadgeFamily::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
    assert!(packet.disclosure.all_render());
}

#[test]
fn every_badge_family_has_entries_and_maps_back() {
    let packet = canonical();
    for family in BadgeFamily::ALL {
        let group = packet.family_group(family).expect("family group");
        assert_eq!(group.descriptor_family, family.descriptor_family());
        assert!(!group.entries.is_empty());
        for entry in &group.entries {
            assert_eq!(entry.badge_family, family);
            assert_eq!(entry.dimension.badge_family(), family);
            assert!(entry.validate().is_empty(), "{:?}", entry.validate());
        }
    }
}

#[test]
fn every_dimension_renders_its_controlled_enum() {
    let packet = canonical();
    for dimension in BadgeDimension::ALL {
        let count = packet
            .all_entries()
            .iter()
            .filter(|e| e.dimension == dimension)
            .count();
        assert!(count > 0, "dimension {} has no badges", dimension.as_str());
    }
    // The four descriptor families are all rendered.
    assert_eq!(
        packet.summary.total_dimensions,
        BadgeDimension::ALL.len() as u32
    );
}

#[test]
fn every_required_user_facing_term_renders_exactly_one_badge() {
    let packet = canonical();
    for term in REQUIRED_USER_FACING_TERMS {
        let matches: Vec<&BadgeVocabularyEntry> = packet
            .all_entries()
            .into_iter()
            .filter(|e| e.label == term)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "term `{term}` must render exactly one badge"
        );
        assert!(packet.required_term_coverage.iter().any(|c| c.term == term));
    }
    assert_eq!(
        packet.summary.required_terms_covered,
        REQUIRED_USER_FACING_TERMS.len() as u32
    );
    assert!(packet.conformance.every_required_term_present);
}

#[test]
fn badge_ids_are_unique_and_export_safe() {
    let packet = canonical();
    let mut ids = std::collections::BTreeSet::new();
    for entry in packet.all_entries() {
        assert!(
            ids.insert(entry.badge_id.clone()),
            "duplicate badge id `{}`",
            entry.badge_id
        );
        assert_eq!(
            entry.badge_id,
            format!("{}.{}", entry.dimension.as_str(), entry.value_token)
        );
        assert!(entry.message_id.starts_with(M5_BADGE_MESSAGE_ID_PREFIX));
    }
}

#[test]
fn weaker_origins_are_first_class_badges() {
    let packet = canonical();
    for token in ["mirror", "offline_bundle", "side_loaded", "not_provided"] {
        let entry = packet
            .all_entries()
            .into_iter()
            .find(|e| e.dimension == BadgeDimension::SourceOrigin && e.value_token == token)
            .unwrap_or_else(|| panic!("source-origin vocabulary dropped `{token}`"));
        assert!(!entry.label.is_empty());
    }
    assert!(packet.conformance.weaker_origins_never_omitted);
}

#[test]
fn tone_and_claim_effect_stay_consistent() {
    let packet = canonical();
    for entry in packet.all_entries() {
        assert_eq!(entry.signal, entry.tone.signal());
        match entry.tone {
            BadgeTone::Authoritative | BadgeTone::Informational => {
                assert_eq!(entry.claim_effect, BadgeClaimEffect::None);
                assert_eq!(entry.signal, DescriptorSignal::Green);
            }
            BadgeTone::Caution => {
                assert_eq!(entry.claim_effect, BadgeClaimEffect::Narrows);
                assert_eq!(entry.signal, DescriptorSignal::Yellow);
            }
            BadgeTone::Blocking => {
                assert_eq!(entry.claim_effect, BadgeClaimEffect::Blocks);
                assert_eq!(entry.signal, DescriptorSignal::Red);
            }
        }
    }
    assert!(packet.conformance.weaker_badges_carry_claim_effect);
}

#[test]
fn exactly_one_authoritative_badge_per_dimension() {
    let packet = canonical();
    for dimension in BadgeDimension::ALL {
        let authoritative = packet
            .all_entries()
            .into_iter()
            .filter(|e| e.dimension == dimension && e.tone == BadgeTone::Authoritative)
            .count();
        assert_eq!(
            authoritative,
            1,
            "dimension {} must have exactly one authoritative badge",
            dimension.as_str()
        );
    }
}

#[test]
fn family_drawer_message_id_links_back_to_the_matrix() {
    let packet = canonical();
    for group in &packet.families {
        // The family drawer id is the same id the descriptor matrix points at.
        assert_eq!(
            group.family_drawer_message_id,
            format!(
                "{}drawer.{}",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                group.descriptor_family.as_str()
            )
        );
    }
}

#[test]
fn export_is_deterministic_and_carries_no_raw_material() {
    let packet = canonical();
    let first = packet.export_safe_json();
    let second = seeded_m5_badge_vocabulary().export_safe_json();
    assert_eq!(first, second);
    let value: serde_json::Value = serde_json::from_str(&first).expect("valid json");
    // Round-trips back to an equal packet.
    let parsed: M5BadgeVocabulary = serde_json::from_value(value).expect("round trip");
    assert_eq!(parsed, packet);
    assert!(packet.validate().is_empty());
}

#[test]
fn lookup_helpers_resolve_badges() {
    let packet = canonical();
    let official = packet.badge_for_term("Official").expect("official badge");
    assert_eq!(official.value_token, "first_party_signed");
    assert_eq!(official.badge_family, BadgeFamily::ProvenanceBadge);
    let by_id = packet.badge(&official.badge_id).expect("by id");
    assert_eq!(by_id, official);

    let stale = packet
        .badge_for_term("Evidence stale")
        .expect("evidence stale");
    assert_eq!(stale.claim_effect, BadgeClaimEffect::Narrows);
    assert_eq!(stale.badge_family, BadgeFamily::FreshnessBadge);
}

#[test]
fn markdown_summary_lists_every_badge() {
    let packet = canonical();
    let md = packet.render_markdown_summary();
    assert!(md.contains("# M5 Badge Vocabulary And Explanation Drawers"));
    for entry in packet.all_entries() {
        assert!(md.contains(&entry.badge_id), "missing `{}`", entry.badge_id);
    }
    for term in REQUIRED_USER_FACING_TERMS {
        assert!(md.contains(term), "missing term `{term}`");
    }
}
