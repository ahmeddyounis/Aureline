# Qualification Matrix — Desktop Local, Remote/Helper, Provider-Linked, State/Schema, and Accessibility — Stable Packet

- Packet: `remote:qualification_matrix:desktop:default`
- Schema version: `1`
- Contract ref: `remote:qualification_matrix:desktop:v1`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (22)

## Surface × profile matrix (16 rows)

| Surface | Profile | Dependency | Local-core | No-account | Failure downgrade | Qualification |
|---|---|---|---|---|---|---|
| `desktop_local` | `local_oss` | `local_only` | ✓ | ✓ | `not_applicable` | `stable` |
| `desktop_local` | `self_hosted` | `managed` | ✓ | — | `local_core_unaffected` | `stable` |
| `desktop_local` | `managed` | `managed` | ✓ | — | `local_core_unaffected` | `stable` |
| `desktop_local` | `air_gapped` | `local_only` | ✓ | ✓ | `not_applicable` | `stable` |
| `remote_helper` | `local_oss` | `network` | ✓ | ✓ | `degraded_features` | `stable` |
| `remote_helper` | `self_hosted` | `managed` | ✓ | — | `degraded_features` | `stable` |
| `remote_helper` | `managed` | `managed` | ✓ | — | `degraded_features` | `stable` |
| `remote_helper` | `air_gapped` | `air_gapped` | ✓ | ✓ | `mirror_fallback` | `stable` |
| `provider_linked` | `local_oss` | `network` | ✓ | ✓ | `degraded_features` | `stable` |
| `provider_linked` | `self_hosted` | `managed` | ✓ | — | `degraded_features` | `stable` |
| `provider_linked` | `managed` | `managed` | ✓ | — | `degraded_features` | `stable` |
| `provider_linked` | `air_gapped` | `air_gapped` | ✓ | ✓ | `mirror_fallback` | `stable` |
| `state_schema` | `local_oss` | `local_only` | ✓ | ✓ | `not_applicable` | `stable` |
| `state_schema` | `self_hosted` | `managed` | ✓ | — | `offline_grace` | `stable` |
| `state_schema` | `managed` | `managed` | ✓ | — | `offline_grace` | `stable` |
| `state_schema` | `air_gapped` | `local_only` | ✓ | ✓ | `not_applicable` | `stable` |

## Accessibility feature matrix (6 rows)

| Feature | Dependency | No-account | Failure downgrade | Qualification |
|---|---|---|---|---|
| `keyboard` | `local_only` | ✓ | `not_applicable` | `stable` |
| `screen_reader` | `local_only` | ✓ | `not_applicable` | `stable` |
| `ime_grapheme_bidi` | `local_only` | ✓ | `not_applicable` | `stable` |
| `zoom` | `local_only` | ✓ | `not_applicable` | `stable` |
| `high_contrast` | `local_only` | ✓ | `not_applicable` | `stable` |
| `reduced_motion` | `local_only` | ✓ | `not_applicable` | `stable` |

## Key invariants verified

1. All 22 required rows are covered (16 surface × profile + 6 accessibility-feature rows).
2. No raw private material is exposed on any row record (`raw_private_material_excluded: true` on all records).
3. Every row explicitly declares `local_core_continuity_allowed: true`; local editing is never blocked by managed or network-dependent row failures.
4. Every row carries an explicit `dependency_class_token` (`local_only`, `network`, `managed`, or `air_gapped`).
5. Every `local_oss`, `air_gapped`, and accessibility row declares `no_account_local_compatible: true`.
6. Every row carries a typed `failure_downgrade_token` so downgrade behavior is reconstructable from typed records.

## Hard guardrail — withdrawal condition

The following forces `Withdrawn` immediately and cannot be overridden:

- Any row record with `raw_private_material_excluded: false`
  (narrow reason: `raw_private_material_exposed`).

## Local-core continuity guarantee

Enterprise, managed, and self-hosted features are additive; no profile may
block local editing, local state access, or local buffer/LSP/keybinding
functionality when its managed or network-dependent capabilities are
unavailable. The `local_core_continuity_allowed: true` field on every row
enforces this guarantee.

## Failure / recovery drill coverage

| Scenario | Surface(s) | Expected behavior |
|---|---|---|
| Managed endpoint unreachable | `desktop_local:managed`, `state_schema:managed` | Local editing continues; managed feature flags and sync degrade; offline-grace window applies to state/schema |
| Self-hosted endpoint unreachable | `remote_helper:self_hosted`, `provider_linked:self_hosted` | Remote and provider features degrade; local repo and workspace continue unblocked |
| No internet, no mirror | `remote_helper:local_oss`, `provider_linked:local_oss` | Remote and provider features degrade gracefully; local core is unaffected |
| Air-gapped, declared mirror | `remote_helper:air_gapped`, `provider_linked:air_gapped` | All connections routed to declared signed mirror; connections outside mirror set are blocked; local core unaffected |
| Raw credential on any row record | any | Immediate `Withdrawn` qualification; `raw_private_material_exposed` defect |
| Missing required row | any required | Narrows to `Preview`; `required_row_missing` defect for each absent row |
| Accessibility feature unvalidated | any accessibility | Narrows to `Beta`; surface must be re-validated before claiming stable |

## Canonical paths

- Doc: `docs/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper.md`
- Runtime owner: `aureline_remote::finalize_qualification_rows_for_desktop_local_remote_helper`
- Fixtures: `fixtures/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper/`
- Schema: `schemas/enterprise/finalize-qualification-rows-for-desktop-local-remote-helper.schema.json`
