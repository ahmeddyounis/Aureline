# Warm-start freshness and bypass-safety report (M3 beta)

**Corpus:** `workspace.warm_start_and_live_resume.beta`
**Boundary:** `aureline_shell::start_center::warm_start_choice::WarmStartChoiceCard`
(validated by `validate_warm_start_choice_card`)
**Replayed by:** `cargo test -p aureline-qe --test warm_start_live_resume_conformance`
**Manifest (single source of truth):** `fixtures/workspace/m3/warm_start_and_live_resume/manifest.json`

This operations/deployment validation report is published by CI for every claimed
beta warm-start, prebuild, or live-resume row. It records — per row — the source
class, the snapshot freshness/age and any invalidation reason, the entry lanes the
card offers, the environment-starter setup location, and the bypass/defer routes
that keep a same-weight local path. The conformance test asserts this report names
every positive and negative drill in the corpus, so the report cannot drift from
the validated card truth.

## Why this lane exists

Warm-start is where a beta promise is easiest to break quietly: a stale snapshot
rendered as a live resume, a managed/remote attach dressed up as a local open, an
"open minimal" escape hatch that quietly fetches over the network, or a setup task
that runs before the user reviews it. The corpus turns each of those into a
regression-gated drill. Two cross-cutting guarantees are re-checked on **every**
positive row:

1. **Local-safe default and a same-weight Open-minimal path.** The default action
   resolves to a local-safe lane that never widens trust or runs networked work,
   and every row keeps an Open-minimal lane so the user can open without the
   starter or prebuild. Local-first rows additionally keep both Open-minimal and
   Set-up-later at the same weight.
2. **No silent side effects.** Any lane that fetches over the network, widens
   trust, runs setup tasks, or attaches a managed/remote runtime is gated behind
   review (never immediately available), and a stale or invalidated snapshot never
   backs a takeable live resume.

## Freshness and bypass matrix (positive rows)

| Drill | Source | Snapshot freshness / age | Invalidation | Setup location | Bypass / defer |
| --- | --- | --- | --- | --- | --- |
| `template.first_party_certified.local` | workspace_template | — | — | local_host | open_minimal / set_up_later |
| `template.local_only.offline` | workspace_template | — | — | local_host | open_minimal / set_up_later |
| `template.team_managed.devcontainer` | workspace_template | — | — | devcontainer | open_minimal / set_up_later |
| `template.community.experimental` | workspace_template | — | — | local_host | open_minimal / set_up_later |
| `template.policy_narrowed.blocked` | workspace_template | — | — (lane blocked_by_policy) | local_host | open_minimal / set_up_later |
| `local_only.folder_open` | local_folder | — | — | no_setup | (no starter to bypass) |
| `live_resume.managed_requires_reauth` | live_workspace | cached / within_days | — | managed_cloud | open_minimal, start_from_snapshot / set_up_later |
| `live_resume.ssh_suspended` | live_workspace | — | — | ssh_host | open_minimal / set_up_later |
| `snapshot.fresh_local_resume` | prebuild_snapshot | fresh / within_hours | — | local_host | open_minimal, start_from_snapshot / set_up_later |
| `snapshot.stale_capsule_drift` | prebuild_snapshot | stale / beyond_review_window | capsule_drift | devcontainer | open_minimal, start_from_snapshot / set_up_later |
| `snapshot.invalidated_lockfile` | prebuild_snapshot | invalidated / within_weeks | dependency_lockfile_changed | local_host | open_minimal, start_from_snapshot / set_up_later |
| `snapshot.mirror_only.unverified` | prebuild_snapshot | unverified / within_days | — (verify before use) | local_host | open_minimal, start_from_snapshot / set_up_later |
| `clone_fresh.remote_repository` | remote_repository | — | — | local_host | open_minimal / set_up_later |
| `clone_fresh.offline_cached_fallback` | remote_repository | cached / within_days | — | local_host | open_minimal, start_from_snapshot / set_up_later |

### Degradation notes

- **Stale / invalidated snapshots downgrade precisely.** `snapshot.stale_capsule_drift`
  and `snapshot.invalidated_lockfile` disable the live-resume lane
  (`unavailable_stale_snapshot`), name the invalidation reason, and still offer a
  read-only inspection of the snapshot plus a rebuild — never a generic setup
  failure.
- **Mirror-only catalogs stay honest.** `snapshot.mirror_only.unverified` marks the
  prebuild freshness `unverified`, withholds the live-resume lane until the mirror
  artifact is verified, and keeps read-only inspection and rebuild available.
- **Offline keeps local continuity.** `clone_fresh.offline_cached_fallback` keeps
  the cached copy usable read-only while clone-fresh stays clearly marked as
  needing the network.
- **Policy narrowing is explicit.** `template.policy_narrowed.blocked` marks the
  generate lane `blocked_by_policy` and lights the honesty marker while preserving
  the local escape hatches.

## Negative drills (rejected regressions)

Each negative drill applies a typed tamper to a contract-valid base card; the
warm-start contract MUST reject it.

| Drill | Tamper proves the contract rejects |
| --- | --- |
| `negative.stale_snapshot_resume_takeable` | a stale snapshot offering a takeable live resume |
| `negative.stale_snapshot_missing_reason` | a stale snapshot that omits its invalidation reason |
| `negative.remote_lane_masquerades_as_local` | a networked lane advertising a local-safe side effect |
| `negative.escape_hatch_has_side_effect` | an Open-minimal lane acquiring a network side effect |
| `negative.safest_action_not_local_safe` | a default action that is not local-safe |
| `negative.default_widens_trust` | a default action that widens trust |
| `negative.local_first_escape_hatch_not_same_weight` | Set-up-later losing same weight on a local-first card |
| `negative.environment_starter_missing_bypass` | a starter that runs setup without a bypass route |
| `negative.environment_starter_missing_defer` | a starter that runs setup without a defer route |
| `negative.managed_attach_undisclosed` | a managed/remote attach hidden from the side-effect summary |
| `negative.source_class_token_drift` | a source-class token drifting from the source class |
| `negative.honesty_marker_inconsistent` | an honesty marker cleared on a card with a stale snapshot |

## Redaction

Every fixture and every validated card is scanned for forbidden raw-content tokens
(private keys, absolute home paths, cloud keys, bearer tokens). Only typed labels,
opaque `sha256:` fingerprint references, and reviewable sentences cross the
warm-start boundary.
