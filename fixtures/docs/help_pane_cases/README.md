# Help-pane state cases

Worked YAML fixtures for the docs / help pane contract frozen in
[`/docs/docs/docs_help_pane_contract.md`](../../../docs/docs/docs_help_pane_contract.md).
Every fixture here conforms to
[`/schemas/docs/help_pane_state.schema.json`](../../../schemas/docs/help_pane_state.schema.json).

The fixtures exist so the in-product docs pane, embedded
docs-browser body, Help / About pane, service-health pane,
support-summary pane, onboarding help overlay, AI-explanation
pane, and release-notice pane can write against a shared corpus
without inventing their own pane vocabulary. Each file carries a
`__fixture__` section summarizing the scenario, the axes it
exercises, and the contract sections it illustrates. The
top-level record itself conforms to the schema so tooling can
validate the file as an integration check.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/docs/help_pane_state.schema.json`](../../../schemas/docs/help_pane_state.schema.json).
  A fixture that fails validation is a bug in the fixture, not
  in the schema.
- **Parity-audit corpus.** A later parity audit between Help,
  About, the docs pane, the docs-browser, the service-health
  view, support summaries, and onboarding overlays compares
  emitted `help_pane_state_record` instances field-for-field
  against these fixtures.
- **Surface development.** Embedded docs / help / About /
  service-health / support-summary / onboarding / AI-explanation
  / release-notice surfaces write against these fixtures so every
  pane resolves the same source / version / freshness / cache /
  install / locale / external-open / policy / stale-example /
  copy-rule fields without interpretation drift.

## Required state coverage

The fixture set in this folder covers every state class the
contract names. A pane that needs to test a state not listed
here MUST add a new fixture rather than overload an existing one.

- [`live_authoritative_baseline.yaml`](./live_authoritative_baseline.yaml)
  — baseline `live_authoritative_truth_copy` rendering against
  the project's own docs pack at exact build match.
- [`cached_only_owner_unreachable.yaml`](./cached_only_owner_unreachable.yaml)
  — cache class `cached_snapshot_only_owner_unreachable` with
  an optional `required_fallback_when_in_product_unavailable`
  browser route.
- [`mirror_only_verified_offline.yaml`](./mirror_only_verified_offline.yaml)
  — air-gapped signed-mirror copy with continuous mirror chain;
  no browser handoff is permitted in the envelope.
- [`expired_cached_requires_refresh.yaml`](./expired_cached_requires_refresh.yaml)
  — cached copy past its `offline_expiration_at`; pane suppressed
  with `cached_snapshot_expired` and `freshness_floor_unmet`
  denial reasons and a `refresh_freshness` repair hook.
- [`not_installed_pack.yaml`](./not_installed_pack.yaml)
  — pack referenced but no copy resident; pane suppressed with
  `pack_not_installed` and an `import_offline_onboarding_bundle`
  repair hook.
- [`unavailable_locale_not_installed.yaml`](./unavailable_locale_not_installed.yaml)
  — requested locale missing and not installed; pane suppressed
  with `locale_missing_not_installed` and an `install_locale_pack`
  repair hook (silent fallback to primary locale forbidden).
- [`stale_example_disclosed_inline.yaml`](./stale_example_disclosed_inline.yaml)
  — partially-stale curated pack stays renderable but surfaces
  `stale_examples_disclosed_inline` with the typed disclosure on
  the primary surface.
- [`version_mismatched.yaml`](./version_mismatched.yaml)
  — runbook pack targets a newer minor than the running build;
  pane suppressed with `incompatible_drift_detected` and an
  `upgrade_release_channel` repair hook.
- [`policy_limited_external_open_blocked.yaml`](./policy_limited_external_open_blocked.yaml)
  — admin policy forbids browser handoff; pane stays renderable
  but `external_open_path = not_permitted` and the policy
  narrowing renders on the primary surface.
- [`external_open_required_fallback.yaml`](./external_open_required_fallback.yaml)
  — embedded docs-browser body offers a required system-browser
  fallback for the same logical object with the screenshot-safe
  / export-safe `external_docs_or_runbook` reason.
- [`support_export_known_limits_about_pane.yaml`](./support_export_known_limits_about_pane.yaml)
  — Help / About pane projects the about-packet's known-limits
  row with `not_build_bound` applicability and an
  `optional_same_object` browser route.

## Related schemas and artifacts

- [`/schemas/docs/help_status_badge.schema.json`](../../../schemas/docs/help_status_badge.schema.json)
  — chip vocabulary the pane projects on the primary surface.
- [`/schemas/docs/destination_descriptor.schema.json`](../../../schemas/docs/destination_descriptor.schema.json)
  — destination route / trust / boundary / external-open
  vocabulary the pane quotes via `destination_descriptor_ref`.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json)
  — docs-pack manifest the pane projects via
  `docs_pack_manifest_ref`.
- [`/schemas/docs/onboarding_pack_state.schema.json`](../../../schemas/docs/onboarding_pack_state.schema.json)
  — onboarding-pack state for panes consumed inside an
  onboarding help overlay.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../../schemas/governance/capability_lifecycle.schema.json)
  — `freshness_class`, `client_scope`, `repair_hook_ref`,
  `redaction_class` vocabularies re-exported in the pane state.
