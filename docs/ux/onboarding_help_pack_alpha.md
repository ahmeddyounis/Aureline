# Onboarding Help-Pack Alpha

This contract defines the first shared onboarding/help/glossary pack consumed by Start Center onboarding, help search, and support export reconstruction.

## Contract

- Pack identity is stable: every pack has a `pack_id`, `pack_revision_ref`, source class, source version, freshness class, version-match state, install state, offline posture, source locale, available locales, and citation availability.
- Item identity is stable: every Start Center card, contextual hint, keymap bridge, migration hint, glossary entry, and future guided-learning reference has an `item_id` that stays valid across locale overlays and future learning surfaces.
- Command metadata is canonical: user-action hints carry `command_id`, command revision when registry-backed, help anchor, keyboard route, route kind, and metadata source.
- Locale fallback is visible: requested locale, effective locale, fallback class, stale-translation marker, and source-language escape hatch are recorded on the item, not inferred from prose.
- Offline posture is explicit: local-only, cached, mirror-verified, and not-installed states are represented as typed pack posture and item render state.
- Citations and exact reopen are preserved: renderable items carry citation refs, source-strip refs, citation-drawer refs, support-export item IDs, and exact reopen refs with pack revision and locale.
- Progress stays user-owned: dismissals, bookmarks, resume points, and deferred installs live in portable user profile state with repo mutation, repo read, and telemetry read disabled by default.

## First Consumers

The shell Start Center onboarding projection reads `artifacts/docs/onboarding_help_pack_alpha.yaml` before falling back to the older docs-pack projection. Help-search rows produced from the pack retain command IDs, keyboard routes, citations, source-language fallback state, and exact reopen refs.

Support export reconstruction uses `support_export_identity` on each item so an exported packet can reconstruct what the user saw without exporting raw docs bodies.

## Protected Proofs

The fixture corpus under `fixtures/docs/onboarding_help_pack_alpha/` covers:

- source-language fallback for a locale-varied keymap bridge;
- local-only, cached, and not-installed pack posture;
- glossary item identity that can be referenced by future guided-learning surfaces;
- support export reconstruction without raw body export.

The alpha pack is a substrate for later guided tours and glossary cards. It is not a tour runtime state store, and it does not give repo packs, docs packs, or extensions hidden access to progress state.
