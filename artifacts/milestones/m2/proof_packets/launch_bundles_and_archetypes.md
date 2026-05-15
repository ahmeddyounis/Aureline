# External Alpha Launch Bundles and Archetype Seeds

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:24:31Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_launch_bundle_contract_set@2026-05-15
validator: ci/check_alpha_launch_bundles.py
validation_capture: artifacts/milestones/m2/captures/launch_bundle_validation_capture.json
claim_change_state: no_claim_widening
```

This packet binds the two external-alpha launch bundles to their archetype seed
rows, benchmark fixture rows, visible Start Center projection, and validator.
It is evidence for the setup contract only; it does not promote either row to a
certified or replacement-grade claim.

## Canonical Artifacts

- TypeScript and JavaScript bundle: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Python bundle: `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Archetype seed rows: `artifacts/certification/m2_archetype_seed_rows.yaml`
- Validator: `ci/check_alpha_launch_bundles.py`
- Latest capture: `artifacts/milestones/m2/captures/launch_bundle_validation_capture.json`
- Start Center consumer: `crates/aureline-shell/src/start_center/mod.rs`

## TypeScript / JavaScript Web App Or Service

| Field | Value |
|---|---|
| Bundle | `launch_bundle:typescript_web_app.seed` |
| Archetype seed row | `archetype_certification_seed:ts_web_app_or_service` |
| Benchmark fixture row | `fixture_register:external_alpha.ts_web_app_reference` |
| Current claim | `experimental` / `alpha_limited` |
| Evidence state | `current_seed` |
| Badge action | opens this packet or the manifest row above |

Acceptance notes:

- The bundle manifest names extensions, profile settings including keymap mode,
  tasks and recipes, docs/tour packs, scaffold/template references, trust notes,
  and rollback checkpoint creation.
- The install/update review uses one coherent diff section list for online,
  mirror, and offline review.
- Drift choices include `apply`, `compare`, `dismiss`, `keep_local`,
  `adopt_bundle`, and `compare_again_later`.
- Remove and rebase paths preserve user-created files, imported mappings, local
  history, and non-bundle-owned artifacts.

## Python Service Or Data App

| Field | Value |
|---|---|
| Bundle | `launch_bundle:python_service_or_data_app.seed` |
| Archetype seed row | `archetype_certification_seed:python_service_or_data_app` |
| Benchmark fixture row | `fixture_register:external_alpha.python_service_data_reference` |
| Current claim | `experimental` / `alpha_limited` |
| Evidence state | `current_seed` |
| Badge action | opens this packet or the manifest row above |

Acceptance notes:

- The bundle manifest names extensions, profile settings including keymap mode,
  tasks and recipes, docs/tour packs, scaffold/template references, trust notes,
  and rollback checkpoint creation.
- Notebook behavior is a handoff disclosure only; full notebook parity remains
  a known limit.
- The mirror/offline review path uses the same section names as online install
  review.

## Validator

Run:

```sh
python3 ci/check_alpha_launch_bundles.py --repo-root .
```

Render the CLI gallery projection:

```sh
python3 ci/check_alpha_launch_bundles.py --repo-root . --render-gallery
```

Refresh the checked-in validation capture:

```sh
python3 ci/check_alpha_launch_bundles.py --repo-root . --report artifacts/milestones/m2/captures/launch_bundle_validation_capture.json
```

The validator checks:

- both expected bundle manifests exist and match the alpha wedge matrix;
- each bundle links to exactly one archetype seed row and one benchmark fixture
  register row;
- install/update review carries one coherent diff over extensions, profile and
  keymap presets, tasks/recipes, docs/tour packs, scaffold/template refs,
  trust/permission notes, and rollback checkpoint creation;
- drift/recommendation choices preserve user agency and local artifacts;
- current support and certification states remain seed-limited; and
- bundle and archetype badges open the underlying evidence packet.
