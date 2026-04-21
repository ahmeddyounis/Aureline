# Docs-pack manifest examples

Worked fixtures for the docs-pack manifest contract frozen in
[`/docs/docs/docs_pack_manifest_contract.md`](../../../docs/docs/docs_pack_manifest_contract.md).
Every fixture here conforms to
[`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json).

The fixtures exist so the docs-browser, Help / About, service-health,
AI-explanation, onboarding, support-export, and citation lanes can
write against a shared corpus without inventing their own manifest
dialect. Each file carries a `__fixture__` section summarizing the
scenario, the axes it exercises, and the contract sections it
illustrates. The top-level record itself conforms to the schema so
tooling can validate the file as an integration check.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json).
  A fixture that fails validation is a bug in the fixture, not in
  the schema.
- **Parity-audit corpus.** A later parity audit between Help /
  About, docs panes, the docs browser, the service-health view, and
  the support summary compares emitted `help_status_badge_record`
  instances against the manifest a row points into. These fixtures
  are the reference packs that audit reads.
- **Surface development.** Docs-browser, Help / About,
  service-health, AI-overlay, onboarding, and support-export
  surfaces write against these fixtures so every surface resolves
  the same source / version / freshness / client-scope / locale /
  citation / backlink fields without interpretation drift.

## Required fields (per the contract)

A publishable manifest MUST carry:

- `pack_id`, `pack_revision_ref`, `source_class`, `publisher_class`,
  `publisher_id`, `display_version`, `semver_version`, `pre_release`,
  `target_running_build_identity_ref`,
- a `signing` block whose `signature_status = signed_and_verified`,
- a `mirror_lineage` block (with `mirror_chain_status` = one of
  `continuous` / `not_applicable`; `predecessor_missing` and
  `signing_chain_broken` are publishable blockers),
- `primary_locale`, a non-empty `available_locales`, and
  `locale_coverage` rows for every available locale,
- `declared_freshness_class`, `refresh_window_seconds`, a non-empty
  `client_scopes`,
- `citation_posture` (with a non-empty
  `required_citation_anchor_kinds` when `citation_required`),
- `backlink_posture`,
- an `example_summary` with `stable_examples_exceed_threshold =
  false`,
- `publishable_state = publishable` with an empty
  `publishable_blocking_reasons`,
- `policy_context`, `redaction_class`, and `minted_at`.

A non-publishable manifest MUST carry at least one
`publishable_blocking_reason` and a non-null `repair_hook_ref`.

## When a docs pack is not publishable

The closed set of publishable-blocking reasons is frozen in the
[contract](../../../docs/docs/docs_pack_manifest_contract.md#when-a-pack-is-not-publishable).
A pack is not publishable when any of the following apply:

- `signature_unverified`, `mirror_continuity_broken`, or
  `pack_quarantined` (signing / mirror / quarantine gates fail).
- `source_class_unresolved`, `client_scope_empty`,
  `locale_set_empty`, or `missing_target_build_identity`
  (required axes cannot be pinned).
- `required_citation_anchors_missing` (citation posture unmet).
- `stale_examples_exceed_threshold` (pack's stale-example ratio
  crossed the publisher's threshold).
- `contract_version_unknown` (schema version lag).
- `backlink_unresolvable` (anchors will not round-trip).
- `publisher_not_permitted` or `policy_blocked` (admin-policy
  narrowing applied).

Surfaces that encounter a non-publishable manifest deny render with
the typed degraded-state cause from
[ADR 0013](../../../docs/adr/0013-docs-help-service-health-truth.md)
and route to the pack's `repair_hook_ref`. Silent rendering as
available is forbidden.

## Fixtures

- [`project_docs_fresh.json`](./project_docs_fresh.json) — the baseline
  publishable case: `source_class = project_docs`,
  `version_match_state = exact_build_match`,
  `freshness_class = authoritative_live`, zero blocking reasons.
- [`mirrored_official_docs_offline.json`](./mirrored_official_docs_offline.json) —
  the required offline / mirrored example: air-gapped signed-bundle
  import, continuous mirror chain, `backlink_deferred`, monotonic
  `offline_expiration_at`.
- [`curated_knowledge_pack_partially_stale.json`](./curated_knowledge_pack_partially_stale.json) —
  the required partially-stale example: two stable examples, one
  stale (`renamed_target_symbol` with a `superseding_example_id`),
  one needs-review (`setting_default_changed`), stale ratio below
  threshold so the pack stays publishable.
- [`project_docs_mixed_locale_coverage.json`](./project_docs_mixed_locale_coverage.json) —
  the required mixed-locale example: four locales with four coverage
  classes (`complete`, `partial`, `machine_assisted`, `stale_copy`).
- [`support_runbook_newer_than_client.json`](./support_runbook_newer_than_client.json) —
  the pack-newer-than-client case: pack targets `1.1.x` while the
  running build is `1.0.x`; the registry marks the manifest blocked
  with a `missing_target_build_identity` reason and an
  `upgrade_release_channel` repair hook.
- [`generated_reference_non_publishable.json`](./generated_reference_non_publishable.json) —
  the non-publishable case: `signature_status = signed_but_unverified`
  and required citation anchors cannot resolve; the manifest is
  blocked with `signature_unverified` and
  `required_citation_anchors_missing` reasons and a `contact_support`
  repair hook.
- [`stale_example_record.json`](./stale_example_record.json) — a
  standalone `docs_pack_example_record` projecting the
  rename-refactor stale example from the partially-stale fixture,
  the shape parity audits use when they enumerate examples across
  packs without holding a full manifest.

## Related schemas and artifacts

- [`/schemas/docs/help_status_badge.schema.json`](../../../schemas/docs/help_status_badge.schema.json)
  — the `help_status_badge_record`, `citation_anchor_record`, and
  `docs_help_service_health_audit_event_record` shapes the manifest's
  `pack_revision_ref`, `source_class`, `freshness_class`, and
  `client_scopes` project into.
- [`/artifacts/docs/help_badge_vocabulary.yaml`](../../../artifacts/docs/help_badge_vocabulary.yaml)
  — worked badge-vocabulary examples that point into pack revisions
  the manifests in this folder describe.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../../schemas/governance/capability_lifecycle.schema.json)
  — `freshness_class`, `client_scope`, `repair_hook_ref`,
  `redaction_class` vocabularies re-exported in the manifest.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
  — the browser-handoff envelope rendering surfaces quote when a
  pack row needs to hand off to a browser (ADR 0010 / ADR 0013).
