//! Canonical seed builders for the M5 client-scope-card registry.
//!
//! These builders are the single producer of the checked-in client-scope-card registry, the
//! release-grade parity proof, and the per-surface consumer fixtures. The headless emitter and the
//! inline tests both call them so the in-code cards, the artifacts, and the fixtures never drift.
//! Each card is built from one [`ClientScopeSubDescriptor`](crate::m5_descriptor_object::ClientScopeSubDescriptor)
//! over the shared, frozen client-scope vocabulary, so the desktop / browser-companion / headless /
//! unsupported cards span every authority class and handoff requirement and prove a narrowed client
//! can never imply desktop parity.

use super::*;

/// Stable registry id for the canonical client-scope-card registry.
pub const M5_CLIENT_SCOPE_CARD_REGISTRY_ID: &str = "m5-client-scope-card-registry:stable:0001";

/// Mint timestamp for the canonical cards.
const SEED_MINTED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// Builds the desktop scope sub-descriptor: the full desktop product at full authority, no handoff.
fn desktop_full_scope() -> ClientScopeSubDescriptor {
    ClientScopeSubDescriptor {
        client_kind: ClientScope::DesktopFull,
        authority_class: AuthorityClass::FullAuthority,
        handoff_requirement: HandoffRequirement::NotRequired,
    }
}

/// The desktop card: the full desktop product surface. No narrowing, no caveat, no blocked action —
/// the one card that carries full capability and authority parity.
pub fn seeded_desktop_full_card() -> ClientScopeCard {
    ClientScopeCard::new(
        "client-scope-card:desktop-full",
        "Desktop full-authority surface",
        SurfaceClass::Desktop,
        desktop_full_scope(),
    )
}

/// The browser-companion card: a mobile / browser companion with bounded, host-relayed scope that
/// must hand off privileged actions to the desktop. It can observe and edit in place but cannot
/// approve or administer without a desktop handoff.
pub fn seeded_browser_companion_card() -> ClientScopeCard {
    ClientScopeCard::new(
        "client-scope-card:browser-companion",
        "Browser / mobile companion surface",
        SurfaceClass::BrowserCompanion,
        ClientScopeSubDescriptor {
            client_kind: ClientScope::MobileCompanion,
            authority_class: AuthorityClass::ScopedAuthority,
            handoff_requirement: HandoffRequirement::DesktopHandoffRequired,
        },
    )
}

/// The browser-reference card: a read-only browser reference surface whose privileged actions pivot
/// to an out-of-plane console. It can only observe; every mutate / approve / administer action is
/// blocked and recovers via a console handoff.
pub fn seeded_browser_reference_card() -> ClientScopeCard {
    ClientScopeCard::new(
        "client-scope-card:browser-reference",
        "Browser reference (read-only) surface",
        SurfaceClass::BrowserCompanion,
        ClientScopeSubDescriptor {
            client_kind: ClientScope::BrowserReference,
            authority_class: AuthorityClass::ReferenceOnly,
            handoff_requirement: HandoffRequirement::ConsoleHandoffRequired,
        },
    )
}

/// The headless card: a CLI / automation client with bounded, host-relayed scope that must hand off
/// privileged actions to the desktop. It can observe and edit in place but cannot approve or
/// administer in place.
pub fn seeded_headless_card() -> ClientScopeCard {
    ClientScopeCard::new(
        "client-scope-card:headless",
        "Headless / automation client",
        SurfaceClass::Headless,
        ClientScopeSubDescriptor {
            client_kind: ClientScope::CompanionScoped,
            authority_class: AuthorityClass::ScopedAuthority,
            handoff_requirement: HandoffRequirement::DesktopHandoffRequired,
        },
    )
}

/// The unsupported handoff-only card: a surface that can only originate or open a desktop handoff
/// and carries no in-place authority at all. Every capability is blocked and recovers on the
/// desktop.
pub fn seeded_unsupported_handoff_card() -> ClientScopeCard {
    ClientScopeCard::new(
        "client-scope-card:unsupported-handoff",
        "Unsupported handoff-only surface",
        SurfaceClass::Unsupported,
        ClientScopeSubDescriptor {
            client_kind: ClientScope::HandoffOnly,
            authority_class: AuthorityClass::HandoffOnly,
            handoff_requirement: HandoffRequirement::DesktopHandoffRequired,
        },
    )
}

/// The unsupported not-provided card: an unsupported surface whose authority and handoff posture
/// were not provided. Every absent value stays explicit — authority `not_provided`, handoff
/// `not_provided` — and the card still blocks every action rather than reading at parity by omission.
pub fn seeded_unsupported_not_provided_card() -> ClientScopeCard {
    ClientScopeCard::new(
        "client-scope-card:unsupported-not-provided",
        "Unsupported not-provided surface",
        SurfaceClass::Unsupported,
        ClientScopeSubDescriptor {
            client_kind: ClientScope::HandoffOnly,
            authority_class: AuthorityClass::NotProvided,
            handoff_requirement: HandoffRequirement::NotProvided,
        },
    )
}

/// The canonical client-scope-card registry: the six seed cards spanning the four surface classes,
/// every authority class, and every handoff requirement, the controlled vocabulary, the consumer
/// set, the conformance review, and the summary.
pub fn seeded_m5_client_scope_card_registry() -> M5ClientScopeCardRegistry {
    M5ClientScopeCardRegistry::new(M5ClientScopeCardRegistryInput {
        registry_id: M5_CLIENT_SCOPE_CARD_REGISTRY_ID.to_owned(),
        report_label: "M5 client-scope card parity across discovery, deep-link, handoff, and companion surfaces"
            .to_owned(),
        cards: vec![
            seeded_desktop_full_card(),
            seeded_browser_companion_card(),
            seeded_browser_reference_card(),
            seeded_headless_card(),
            seeded_unsupported_handoff_card(),
            seeded_unsupported_not_provided_card(),
        ],
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}
