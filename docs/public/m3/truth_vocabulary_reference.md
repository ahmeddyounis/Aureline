# Truth-vocabulary reference

This is the public, human-readable reference for the **product truth
vocabulary**: the trust-bearing and release-bearing state words every
user-visible channel must use. It is the readable companion to the governed
registry at `artifacts/governance/product_truth_vocabulary.yaml` and the
diffable parity report at
`artifacts/release/m3/truth_vocabulary_parity_report.md`.

## Why one vocabulary

Product UI, Help/About, docs, release notes, admin / CLI / headless inspect
output, and support bundles all answer the same questions about a row: what
lifecycle stage is it in, how ready is it, who is the source of truth behind
it, how long is its data retained, how is it installed and updated, which
deployment profile does it sit in, and what is its service-health state when a
boundary is impaired.

When each surface forks a private synonym, one surface can call a row `Stable`
while another calls it `Beta`, one can say a destination is `Official` while
another says `Community`, and a generic `service down` can hide that local work
is still safe. The registry closes those gaps: every surface resolves to one
canonical word per state class, and the parity gate fails closed before a beta
widens or a stable promotes on a divergent vocabulary.

This vocabulary is deliberately narrow. It governs the seven state classes
below only. Narrative docs, marketing copy, and microcopy are out of scope
except where they quote one of these controlled state words.

## How terms resolve

Each surface usage of a state word resolves to exactly one of:

- a **canonical term** — the controlled word, used as-is;
- an **allowed alias** — a product display label or accepted synonym that
  resolves silently to a canonical term (for example `Stable` → `stable`,
  `desktop_local_first` → `individual_local`);
- a **deprecated alias** — still recognized, but raises an *alias migration*
  finding so it cannot persist silently; past its `migrate_by` date it becomes
  a **blocker**;
- a **forbidden alias** — a banned parallel synonym; quoting it on a protected
  surface is a **blocker**;
- **unknown** — an off-vocabulary word; a **blocker** on a protected surface,
  a **warning** on an advisory one.

Two surfaces describing the same subject with conflicting resolved vocabulary
is a **cross-surface conflict** blocker.

## Surface classes

A vocabulary class is *protected* on a surface when its state words are
claim-bearing there. A mismatch in a protected pairing is a blocker; the same
mismatch on an advisory (non-protected) pairing is a warning.

| Surface | Protected vocabulary classes |
|---|---|
| `product_ui` | all seven |
| `help_about` | all seven |
| `docs` | `lifecycle`, `readiness`, `authority`, `retention`, `deployment_profile` |
| `release_notes` | `lifecycle`, `readiness`, `install_update`, `deployment_profile` |
| `cli_headless_inspect` | `lifecycle`, `readiness`, `install_update`, `deployment_profile`, `outage_boundary` |
| `support_bundle` | all seven |
| `admin_export` | all seven |

## Vocabulary classes

Every axis below mirrors a single upstream source of truth verbatim; the
parity gate fails closed if the registry drifts from it.

### `lifecycle` — capability lifecycle / release-stage state

Axis `lifecycle_state`, mirrored from
`schemas/governance/capability_lifecycle.schema.json`.

Canonical terms: `labs`, `preview`, `beta`, `stable`, `lts_facing`,
`deprecated`, `disabled_by_policy`, `retired`.

Display aliases: `Labs`, `Preview`, `Beta`, `Stable`, `LTS`, `Deprecated`,
`Disabled by policy`, `Retired`. Forbidden synonyms include `ga`,
`generally_available`, `alpha`, `early_access`, and `end_of_life`. `sunset`
is deprecated in favor of `deprecated`.

### `readiness` — qualification / support readiness class

Axis `support_class`, mirrored from
`artifacts/release/m3/claim_manifest_matrix.yaml`.

Canonical terms: `certified`, `supported`, `limited`, `experimental`,
`community`, `retest_pending`, `evidence_stale`, `unsupported`.

Forbidden synonyms include `production_ready` and `not_supported`.
`fully_supported` and `best_effort` are deprecated in favor of `supported` and
`limited`.

### `authority` — source / destination trust class

Axis `destination_class`, mirrored from
`schemas/public/about_destination.schema.json`.

Canonical terms: `official_public`, `official_private`, `community`,
`third_party_vendor`.

`third_party` is forbidden (use `third_party_vendor`). `official`,
`first_party`, and `vendor` are deprecated in favor of their precise forms.

### `retention` — data-retention posture class

Axis `retention_class`, mirrored from
`artifacts/governance/deployment_profiles.yaml`.

Canonical terms: `no_retention_beyond_local_disk`, `workspace_repo_retained`,
`customer_retention_window`, `vendor_retention_window_with_customer_policy`,
`vendor_retention_window_default`, `retention_not_applicable`.

`not_retained` is forbidden. `ephemeral` and `vendor_default` are deprecated.

### `install_update` — install-mode and update-channel class

Axis `install_mode`, mirrored from
`schemas/install/install_topology_truth_row.schema.json`.

Canonical terms: `per_user_installed`, `per_machine_installed`, `portable`,
`offline_bundle`, `managed_deployed`, `side_by_side_preview`.

Axis `update_channel`, mirrored from the same schema.

Canonical terms: `stable`, `preview`, `beta`, `lts`, `portable_stable`,
`portable_preview`.

`mdm_deployed`, `ga`, and `nightly` are forbidden. `user_install`,
`enterprise_deployed`, and `release` are deprecated.

### `deployment_profile` — deployment / topology profile

Axis `deployment_profile`, mirrored from
`artifacts/governance/deployment_profiles.yaml`.

Canonical terms: `individual_local`, `self_hosted`, `enterprise_online`,
`air_gapped`, `managed_cloud`.

Product-facing labels are allowed aliases: `desktop_local_first`,
`self_hosted_sovereign`, `hybrid_remote_attach`, `air_gapped_mirror_only`,
`browser_companion_handoff_default_home`. `on_premise` and `offline` are
forbidden; `saas`, `hybrid`, and `on_prem` are deprecated.

### `outage_boundary` — service-health contract state and boundary class

Axis `service_contract_state`, mirrored from
`schemas/ops/service_contract_state.schema.json`.

Canonical terms: `ready`, `degraded`, `local_only`, `stale`,
`contract_mismatch`, `policy_blocked`, `unavailable`.

Axis `boundary_class`, mirrored from
`artifacts/release/m3/service_health_contract_matrix.json`.

Canonical terms: `local_only`, `vendor_provider`,
`local_with_remote_required`, `local_with_remote_optional`, `hosted`.

Generic outage copy is forbidden: `service_down`, `service_degraded`, and
`broken`. `outage`, `remote_required`, and `cloud_only` are deprecated.

## How to confirm parity

```bash
scripts/ci/run_truth_vocabulary_parity.sh
```

Regenerate the parity report after a registry or upstream change:

```bash
scripts/ci/run_truth_vocabulary_parity.sh --write
```
