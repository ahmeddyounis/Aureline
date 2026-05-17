# Beta update rollback packet

This packet is the beta source of truth for update rollback after a
failed or revoked beta update. It is generated from
[`/artifacts/release/m3/update_rollback/rollback_plan.json`](../../../artifacts/release/m3/update_rollback/rollback_plan.json)
and projects to
[`/artifacts/release/m3/update_rollback/support_export_projection.json`](../../../artifacts/release/m3/update_rollback/support_export_projection.json).
The headless gate is
[`/tools/ci/m3/update_rollback/`](../../../tools/ci/m3/update_rollback).

The current beta build is
`build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`.
The admitted rollback target is
`release_candidate:aureline.2_0_4_stable` with exact-build identity
`build-id:aureline:stable:2.0.4:aarch64-apple-darwin:release:1f40c9d2b4a1`.
Rollback claims must use these refs, not version strings alone.

## Rollback Vocabulary

UI, docs, support export, and migration surfaces quote the same plan
vocabulary:

- `retained_prior_artifact_set`
- `schema_rollback_hook`
- `downgrade_eligibility_state`
- `exact_build_identity_ref`

The shared vocabulary is intentionally mechanical. It lets support
compare a user-facing rollback sheet with the support export without
translating between marketing names, installer status strings, and
release-center refs.

## Retained Prior Artifact Set

The rollback target is admitted only because the plan retains the prior
coordinated artifact set as exact-build artifacts. The
`retained_prior_artifact_set` covers desktop shell, CLI, remote agent
tarball, update metadata, policy bundle, schema export, docs pack,
support runbook bundle, and release evidence packet. Every retained row
uses `exact_build_identity_ref` equal to
`build-id:aureline:stable:2.0.4:aarch64-apple-darwin:release:1f40c9d2b4a1`,
`retention_state = retained_exact_build`,
`verification_state = verified`, and `rollback_atom_member = true`.

Metadata-only retention is not enough. If package bytes, schema export,
docs/help truth, or support runbook refs are missing, the validator
downgrades the rollback claim instead of allowing a binary-only restore.

## Schema Rollback Hooks

The plan admits three `schema_rollback_hook` rows:

| Hook | State root | Compatibility | Reviewed flow | Checkpoint |
|---|---|---|---|---|
| `schema_hook:update.rollback.settings_profile` | `state.per_user_configuration_root.stable` | `additive_compatible` | `update_center_review` | `checkpoint.update.rollback_triggered` |
| `schema_hook:update.rollback.recovery_root` | `state.per_user_recovery_root.stable` | `backward_readable` | `support_assisted_review` | `checkpoint.update.rollback_completed` |
| `schema_hook:update.rollback.remote_agent_cache` | `state.per_user_derived_cache_root.stable` | `repair_required` | `headless_review` | `checkpoint.update.post_rollback_evidence_captured` |

Hooks are not generic cleanup scripts. They can be invoked only through
reviewed update checkpoints, and non-automatic compatibility carries a
repair transaction or a review-gated downgrade state.

## Downgrade Truth

The published `downgrade_eligibility_state` is
`eligible_with_review`. The support export repeats the caveats verbatim:

- schema rollback hooks are invoked only from update rollback
  checkpoints with review refs;
- derived remote-agent cache may rebuild after rollback and is not
  user-authored state loss;
- policy forced-minimum versions can block rollback even when retained
  artifacts exist.

User-authored configuration, recovery state, and managed policy roots
are preserved. Derived remote-agent cache is intentionally not restored
and may rebuild after rollback.

## Verification

```bash
python3 -m tools.ci.m3.update_rollback --repo-root . --check
cargo test -p aureline-install --test update_rollback_plan_beta
```
