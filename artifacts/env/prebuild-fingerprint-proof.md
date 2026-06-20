# Prebuild-fingerprint proof

This is the human-readable proof summary for the prebuild-snapshot
compatibility-fingerprint lane. The machine-readable packet is
[`artifacts/env/prebuild-fingerprint-packet.json`](prebuild-fingerprint-packet.json);
the canonical implementation is
[`crates/aureline-env/src/prebuilds/mod.rs`](../../crates/aureline-env/src/prebuilds/mod.rs)
and the reviewer doc is
[`docs/env/prebuild-fingerprint.md`](../../docs/env/prebuild-fingerprint.md). The
packet, fixtures, and this report are regenerated from the seeded projection, so
they cannot disagree with the engine.

## Fingerprint keys and invalidation rules

A prebuild fingerprint is keyed on six inputs; each key's drift floor is fixed by
its compatibility class, and the rules are derived from the class so they can
never drift from `evaluate_prebuild_reuse`.

| Key | Class | Drift | Absent |
| --- | --- | --- | --- |
| `source_tree_identity` | identity | cold | cold |
| `capsule_hash` | identity | cold | cold |
| `platform_arch` | invalidating | invalidated | invalidated |
| `policy_epoch` | invalidating | invalidated | invalidated |
| `extension_lock_digest` | layered | partially warm | cold |
| `toolchain_digest` | layered | partially warm | cold |

Artifact integrity: losing a non-critical layer → partially warm; losing a
critical layer (`base_image`, `toolchain`) → cold.

## Canonical cases

Every outcome — warm, partially warm, cold, and invalidated — is exercised by a
checked-in case whose stamped decision replays from the engine.

| Case | Outcome | Reason | Gated actions |
| --- | --- | --- | --- |
| `case.prebuild.full_match` | warm | full_match | — |
| `case.devcontainer.extension_lock_drift` | partially warm | extension_lock_drift | language_intel |
| `case.remote_container.partial_artifact_loss` | partially warm | partial_artifact_loss | search_index, language_intel |
| `case.starter.source_drift` | cold | source_drift | build_run, search_index, language_intel, services |
| `case.managed_workspace.platform_drift` | invalidated | platform_drift | all |
| `case.prebuild.policy_drift` | invalidated | policy_drift | all |

## Fixture corpus

Ten fixtures cover the warm baseline, every key-drift class, both artifact-loss
classes, and an unrecorded (absent) key. Each fixture's expected outcome,
reuse/invalidated flags, warm-start posture, reason, reason tokens, and gated
actions are computed from the engine.

| Fixture | Outcome | Reused | Invalidated |
| --- | --- | --- | --- |
| `full_match_warm` | warm | yes | no |
| `extension_lock_drift_partial` | partially warm | yes | no |
| `toolchain_drift_partial` | partially warm | yes | no |
| `partial_artifact_loss_partial` | partially warm | yes | no |
| `source_drift_cold` | cold | no | no |
| `capsule_drift_cold` | cold | no | no |
| `critical_artifact_loss_cold` | cold | no | no |
| `source_tree_absent_cold` | cold | no | no |
| `policy_drift_invalidated` | invalidated | no | yes |
| `platform_drift_invalidated` | invalidated | no | yes |

## Acceptance evidence

- **Claimed warm-start paths can explain why a prebuild was reused, rejected, or
  downgraded.** Every decision carries the dominant `PrebuildReason`, per-key and
  per-artifact evaluation, and a review-safe headline.
- **Users and support tooling can distinguish warm, partially warm, cold, and
  invalidated starts.** The corpus distinguishes all four `StartOutcome`s; the
  reuse flag is set only for warm and partially warm starts and the invalidated
  flag only for invalidated ones.
- **Prebuild reuse no longer silently outruns source/policy/platform drift.** The
  source-, policy-, and platform-drift fixtures and drills all reject or
  invalidate the snapshot; none reuse it.

See [`artifacts/env/prebuild-reuse-drills.md`](prebuild-reuse-drills.md) for the
failure / recovery drills.
