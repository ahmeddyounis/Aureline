# Onboarding pack-state examples

Worked fixtures for the onboarding docs-pack locale, cached /
not-installed state, and offline disclosure contract frozen in
[`/docs/docsops/onboarding_pack_state_contract.md`](../../../docs/docsops/onboarding_pack_state_contract.md).
Every fixture here conforms to
[`/schemas/docs/onboarding_pack_state.schema.json`](../../../schemas/docs/onboarding_pack_state.schema.json).

The fixtures exist so the Start Center, first-run tour, glossary
card, guided-tour step, in-product help overlay, onboarding portability
audit, and support-export surfaces can write against a shared corpus
without inventing their own onboarding-state dialect. Each file carries
a `__fixture__` section summarizing the scenario, the axes it exercises,
and the contract sections it illustrates. The top-level record itself
conforms to the schema so tooling can validate the file as an
integration check.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/docs/onboarding_pack_state.schema.json`](../../../schemas/docs/onboarding_pack_state.schema.json).
- **Cross-contract audit corpus.** Parity audits between onboarding
  surfaces (Start Center, first-run tour, welcome banner, glossary
  card, help overlay), the docs-pack manifest registry, the
  destination-descriptor table, the guided-surface state table, and
  the onboarding-portability manifest compare emitted rows against the
  onboarding-pack state record the row projects from. These fixtures
  are the reference states that audit reads.
- **Surface development.** Onboarding / glossary / tour / help-overlay
  surfaces write against these fixtures so every surface resolves the
  same install-state / locale-presence / freshness / route / account
  prerequisite / denial fields without interpretation drift.

## Fixtures

- [`local_only_starter_docs.json`](./local_only_starter_docs.json) —
  the baseline local-only starter case: `install_state =
  local_only_starter`, `freshness_class = authoritative_live`,
  `browser_handoff_policy = no_handoff_local_only`,
  `account_prerequisite_class = no_account_required`,
  `reset_class = not_resettable_packaged_with_binary`.
- [`cached_but_stale_help.json`](./cached_but_stale_help.json) — the
  cached-but-stale help-overlay case: `install_state =
  cached_snapshot_stale`, `freshness_class = stale`, typed
  `cached_snapshot_expired` denial, embedded surface with a
  screenshot-safe system-browser fallback.
- [`missing_locale_pack.json`](./missing_locale_pack.json) — the
  missing-locale-pack case: primary `en` locale is reviewed and
  installed, the user requested `pt-BR`, the `pt-BR` locale pack is
  not installed, and the surface suppresses the row with a typed
  `locale_missing_not_installed` denial plus an `install_locale_pack`
  repair hook.
- [`account_optional_onboarding.json`](./account_optional_onboarding.json)
  — the account-optional onboarding-guidance case:
  `pack_role = account_optional_onboarding_pack` holds the
  `no_account_required` account prerequisite under every envelope and
  composes with an onboarding-portability state record that pins tour
  progress as portable profile state.
- [`air_gapped_offline_onboarding.json`](./air_gapped_offline_onboarding.json)
  — the air-gapped case: `install_state = mirror_only_verified`,
  `offline_posture = air_gapped_signed_bundle`, applicable only to
  self-hosted / air-gapped deployment profiles,
  `account_prerequisite_class = no_account_required`.
- [`locale_row_missing_fallback.json`](./locale_row_missing_fallback.json)
  — a standalone `onboarding_pack_locale_row_record` projecting the
  `pt-BR` locale row from the missing-locale fixture, the shape parity
  audits use when enumerating locale gaps across onboarding packs
  without holding a full state record.

## Required fields (per the contract)

A publishable onboarding-pack-state record MUST carry:

- `pack_id`, `pack_revision_ref`, `docs_pack_manifest_ref`,
  `destination_descriptor_ref`,
- `pack_role`, `install_state`, `freshness_class`,
  `version_match_state`, `running_build_identity_ref`,
- `primary_locale`, `requested_locale`, `effective_locale`,
  `locale_presence_class`, `locale_fallback_disclosure_class`,
  a non-empty `locale_rows` list (one per advertised locale, including
  the primary locale),
- `offline_posture`, `browser_handoff_policy`,
  `account_prerequisite_class`, `embedded_route_policy`,
- a non-empty `applicable_deployment_profiles`,
- `publishable = true` with an empty `publishable_denial_reasons`,
- `reset_class`, `policy_context`, `redaction_class`, `minted_at`.

A non-publishable record MUST carry at least one
`publishable_denial_reason` and a non-null `repair_hook_ref`.

## Related schemas and artifacts

- [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json)
  — the upstream docs-pack manifest whose `pack_revision_ref`,
  `source_class`, `freshness_class`, signing / mirror-lineage, and
  publishable-blocking-reason fields this record projects.
- [`/schemas/docs/destination_descriptor.schema.json`](../../../schemas/docs/destination_descriptor.schema.json)
  — the destination descriptor this record's
  `destination_descriptor_ref` pins; supplies preferred / fallback
  route classes, external-open policy, disclosure-mode, and
  `browser_handoff_reason` subset.
- [`/schemas/ux/guided_surface_state.schema.json`](../../../schemas/ux/guided_surface_state.schema.json)
  — the guided-surface state the optional `guided_surface_state_ref`
  pins; supplies suppression cause, dismissal / reset / progress-export
  class, and surface-kind vocabulary for glossary cards, guided-tour
  steps, onboarding cards, architecture explainers, contextual tips,
  exercise steps, and speaker-note / teaching-session adjuncts.
- [`/schemas/ux/onboarding_portability_state.schema.json`](../../../schemas/ux/onboarding_portability_state.schema.json)
  — the onboarding-portability state the optional
  `onboarding_portability_state_ref` pins; supplies entry-surface
  family, account-prompt class, state-portability class, and reset /
  export / profile-scope class.
