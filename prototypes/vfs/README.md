# VFS / save prototype

This prototype validates the five-layer filesystem-identity model, the
conflict-aware save pipeline, the watcher-source / watcher-health contract,
and the root-capability envelope frozen in
[`docs/adr/0006-vfs-save-cache-identity.md`](../../docs/adr/0006-vfs-save-cache-identity.md)
and in the cross-surface vocabulary at
[`docs/filesystem/filesystem_identity_vocabulary.md`](../../docs/filesystem/filesystem_identity_vocabulary.md).
The goal is contract correctness: trigger the exact outcomes, counters, and
watcher frames the ADR names against a reviewable synthetic-root fixture
table, without taking a dependency on platform-specific filesystem quirks.

It is a **prototype**, not the production VFS. No mmap, no real watchers, no
real remote agent transport, no rename planner, no durable cache identity.
Those land with the production filesystem work; this prototype proves the
vocabulary, the save failure-case split, and the watcher-posture story with
one byte-stable save-plan record per scenario.

## Where the code lives

| Piece | Path |
|---|---|
| Identity (layers 1–4) | [`crates/aureline-vfs/src/identity.rs`](../../crates/aureline-vfs/src/identity.rs) |
| Root-capability envelope | [`crates/aureline-vfs/src/capabilities.rs`](../../crates/aureline-vfs/src/capabilities.rs) |
| Synthetic filesystem model | [`crates/aureline-vfs/src/synthetic.rs`](../../crates/aureline-vfs/src/synthetic.rs) |
| Watcher-source / watcher-health | [`crates/aureline-vfs/src/watcher.rs`](../../crates/aureline-vfs/src/watcher.rs) |
| Save-target token + conflict-aware save stub (layer 5) | [`crates/aureline-vfs/src/save.rs`](../../crates/aureline-vfs/src/save.rs) |
| Protected-hot-path hook counters | [`crates/aureline-vfs/src/hooks.rs`](../../crates/aureline-vfs/src/hooks.rs) |
| Smoke harness (frozen scenario table + JSON renderer) | [`crates/aureline-vfs/src/harness.rs`](../../crates/aureline-vfs/src/harness.rs) |
| Bench binary | [`crates/aureline-vfs/src/bin/vfs_proto.rs`](../../crates/aureline-vfs/src/bin/vfs_proto.rs) |
| Public re-exports (crate surface) | [`crates/aureline-vfs/src/lib.rs`](../../crates/aureline-vfs/src/lib.rs) |
| Corpus manifest + rename matrix | [`fixtures/fs/identity_corpus_manifest.yaml`](../../fixtures/fs/identity_corpus_manifest.yaml), [`fixtures/fs/case_only_rename_matrix.yaml`](../../fixtures/fs/case_only_rename_matrix.yaml) |
| Human-reviewable fixtures | [`fixtures/fs/save_truth_cases/`](../../fixtures/fs/save_truth_cases/) |
| Alias/symlink supplements | [`fixtures/fs/alias_and_symlink_cases/`](../../fixtures/fs/alias_and_symlink_cases/) |
| Machine-emitted save plans | [`artifacts/fs/save_plan_examples/`](../../artifacts/fs/save_plan_examples/) |
| Reviewer-facing coordination rows | [`artifacts/fs/save_coordination_examples/`](../../artifacts/fs/save_coordination_examples/) |
| Smoke wrapper | [`tools/vfs_proto.sh`](../../tools/vfs_proto.sh) |

## What the prototype models

- **Five-layer identity record.** Every scenario routes every file through
  `PresentationPath` (layer 1), `LogicalWorkspaceIdentity` (layer 2),
  `CanonicalFilesystemObject` (layer 3), and `AliasSet` (layer 4). Layer 5
  (the `SaveTargetToken`) is the only one the save pipeline issues; surfaces
  that cannot produce a token MUST NOT offer a save affordance.
- **Root-capability envelope.** Every synthetic root advertises its flag
  set, root class, strongest / fallback identity-token kinds, preferred and
  permitted save modes, watcher source, and optional mount-graph hash at
  attach time. `select_save_mode()` picks the mode the envelope allows and
  returns `blocked` for read-only / policy-constrained roots before any
  bytes move.
- **Synthetic filesystem.** `SyntheticRoot` owns canonical objects, alias
  presentations, generation counters, and permission snapshots — no real
  filesystem access. `apply_external_change` bumps the generation to model
  a sibling writer; `apply_commit` bumps on a successful save. This keeps
  the emitted save-plan JSON byte-stable across hosts.
- **Conflict-aware save stub.** `attempt_save` runs in the fixed order the
  ADR names: policy / read-only / review gate → participant run →
  compare-before-write vs. a fresh read of the strongest token → commit via
  the selected mode → manifest recorded. Every failure is a typed
  `SaveOutcome` from the ADR's failure-case taxonomy; no silent demotion.
- **Watcher posture.** `WatcherRegistry` tracks source and health per
  root and emits a `WatcherHealthFrame` on every transition. Health
  transitions toward `degraded` / `fallback_polling` / `unavailable` are
  what ADR 0005 `subscription_freshness_downgrade` frames cite with
  `stale_reason = watcher_dropped`. Watcher perfection is NOT a save
  correctness requirement; compare-before-write is.
- **Save-plan record.** Each scenario emits a `SavePlan` that bundles the
  identity record, the save-target token (with capability flags + write
  mode + permission snapshot), the manifest (with outcome + capability
  mode + failure detail), the watcher frames captured in the window, and
  a reviewer-notes list. The renderer is hand-rolled JSON: one record
  explains opened path vs. actual write target vs. rewrite class vs.
  degraded / unsupported conditions, and each exported scenario also
  carries the stable `corpus_case_id`, any `related_fixture_ids`, and
  any `rename_matrix_row_refs`.
- **Structural metrics only.** Counts, labels, synthetic monotonic tokens.
  No wall-clock latencies, no process IDs, no inode numbers from the host
  filesystem. The benchmark lab layers timing on top of these counts when
  it scores against protected-hot-path budgets.

## Scenario table

One row per ADR 0006 failure case the pipeline must name with its own
vocabulary. See `fixtures/fs/save_truth_cases/README.md` for the index and
`fixtures/fs/identity_corpus_manifest.yaml` for the stable case ids.

| Label | Outcome |
|---|---|
| `local_atomic_save_happy_path` | `committed` |
| `case_only_difference` | `committed` |
| `symlink_alias` | `committed` |
| `hardlink_sibling` | `committed` |
| `unicode_normalization_variant` | `committed` |
| `external_change_detected` | `external_change_detected` |
| `review_required_before_save` | `review_required_before_save` |
| `read_only_root_blocked` | `read_only_or_policy_blocked` |
| `remote_conditional_conflict` | `save_conflict` |
| `watcher_degradation` | `committed` (with degraded watcher frames) |
| `save_participant_failed` | `save_participant_failed` |

## How to run

From the repo root:

```
./tools/vfs_proto.sh                                     # aggregate -> stdout
./tools/vfs_proto.sh --emit-scenarios artifacts/fs/save_plan_examples
```

Flags:

- `--release` — build/run the release profile.
- `--emit PATH` — write the aggregate JSON to `PATH` instead of stdout.
- `--emit-scenarios DIR` — write one `<label>.json` per scenario plus
  `aggregate.json` into `DIR`.

The Rust crate has its own tests: `cargo test -p aureline-vfs` covers the
watcher health-state machine, the save-mode selector on the capability
envelope, the synthetic root's generation semantics, the full harness's
byte-stability across reruns, and that every alias / blocked / review /
conflict scenario reaches exactly the expected `SaveOutcome`.

## Known holes — carried forward, not hidden in comments

These are recorded here rather than left implicit in source. Every item
below is a visible carry-forward task; none is a silent capability of the
prototype.

1. **No real filesystem.** Every scenario uses a `SyntheticRoot`. The
   production VFS owns the platform-specific adapters (APFS, NTFS, ext4,
   network filesystems) and the case / normalization / symlink / bind-mount
   resolution rules each needs; the prototype's synthetic object model is
   the seam those adapters must match.
2. **No real watcher.** `WatcherRegistry` is a state-machine stub. The
   production build wires OS native watchers (FSEvents / inotify /
   ReadDirectoryChangesW), remote-agent streams, and scalable watcher
   integrations behind the same source / health / frame vocabulary.
3. **No mmap, no paged reader on save.** The save stub writes whole-buffer
   bytes through the synthetic root. Large-file save paths land with the
   large-file prototype / production buffer stack; this prototype names
   the modes but does not stream bytes.
4. **No durable cache identity.** `vfs_cache_lookup`,
   `vfs_cache_invalidate`, and `vfs_cache_rebuild` are named here as
   counters so downstream lanes do not invent synonyms; the durable cache
   lands with the cache / input-digest work.
5. **No mutation-journal wiring.** The save manifest is produced; writing
   it durably, rotating the journal, and replaying against a parent
   snapshot belongs to the mutation-journal workstream.
6. **No rename planner.** `vfs_rename_plan_previewed` / `vfs_rename_commit`
   are named but unused. Case-only, Unicode-normalization, and safety-
   reviewed renames land separately behind the same ADR 0006 planner
   contract.
7. **No review-workflow wiring.** `review_required_before_save` /
   `review_required_before_rename` route to typed outcomes; the banner,
   approval path, and reviewer surface land with the review workstream.
8. **Single save participant, single group.** `attempt_save` models one
   participant (textual normalisation) per save. Multi-participant ordering,
   save-participant-group rebase, and group failure rollback belong to the
   save-pipeline workstream. `vfs_save_stage` / `vfs_save_participant_run`
   are the seams those hooks consume.
9. **No real permission elevation.** `PermissionSnapshot` is a record.
   Production branches on it to decide whether in-place writes need sudo /
   UAC elevation; the prototype carries the snapshot for reviewers but
   does not act on it.
10. **No remote-agent transport.** The `remote_conditional_conflict`
    scenario models a revision-token bump through `apply_external_change`.
    The real conditional-remote-write wire protocol lands with the
    remote-agent workstream; the prototype names `rev:N` values so the
    mismatch is visible in the committed seed.
11. **No archive-view mount graph.** `ArchiveLikeView` is a root class the
    prototype only exercises with a read-only overlay fixture. Real archive
    traversal, inner-alias disclosure, and the archive-safety policy land
    with the archive / container workstream.
12. **No observability schema validation.** The emitted JSON is
    byte-stable but is not checked against a JSON-schema. Once
    `schemas/runtime/vfs_save_envelope.schema.json` stabilises, the
    harness should validate each record against its boundary schema.
13. **Synthetic monotonic tokens stand in for timestamps.** The
    prototype emits `mono:HHMM:SS:SS.FRAC` strings where the production
    pipeline uses producer-local monotonic timestamps. The benchmark lab
    attaches real timestamps on top of these.
14. **Aggregate is per-scenario; no multi-root orchestration.** Each
    scenario attaches one root. Workspaces with many roots (multi-root
    workspaces, submounts, detached roots) are the next carry-forward.
15. **No trust-boundary enforcement.** `TrustState::Restricted` is named
    but the pipeline does not refuse restricted-root saves beyond the
    `review_required_*` flags. The restricted-mode workflow lands with
    the workspace-trust workstream.

## Carry-forward items (what the next wave of work picks up)

- Replace `SyntheticRoot` with platform adapters behind the same
  `resolve` / `read_strongest_token` / `apply_commit` seams.
- Land the production save pipeline (real tempfile + atomic rename,
  real in-place write, real conditional remote write behind the same
  `atomic_write_mode` vocabulary).
- Wire the mutation journal so every `SaveManifest` durably writes an
  entry and a failed commit can replay without silent corruption.
- Land the rename planner (case-only, Unicode-normalization, and
  cross-alias) behind the `vfs_rename_plan_previewed` /
  `vfs_rename_commit` hook pair.
- Wire real filesystem watchers per platform and per root class;
  emit `WatcherHealthFrame` on every real transition.
- Attach each emitted save plan to the boundary schema at
  `schemas/runtime/vfs_save_envelope.schema.json` and assert boundary
  validity in harness tests.
- Grow the scenario table with multi-root workspaces, restricted-trust
  roots, decode-recovery saves, and migration-save cases so the
  benchmark lab has the coverage to score against the ADR's full
  failure-case taxonomy.
