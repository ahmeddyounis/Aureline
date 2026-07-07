# M5 publication-component-consumer fixtures

Protected narrowed-variant fixtures for the M5 publication-component-consumer
adoption lane. Each is minted only by the headless emitter and asserted
byte-aligned with its seed builder by
`checked_narrowed_fixtures_validate_and_match_seed_builders`.

- `about_help_handoff_narrowed.json` — the About/help consumer held at `beta`
  pending reduced-scope-banner parity across every browser-handoff path; every
  consumer stays visible.
- `docs_mirror_offline_narrowed.json` — the docs portal narrowed to `preview`
  pending mirror/offline caveat-parity across every snapshot path; every consumer
  stays visible.

Regenerate via the subcommands documented in
[`docs/release/m5_publication_component_consumer_contract.md`](../../../docs/release/m5_publication_component_consumer_contract.md).
