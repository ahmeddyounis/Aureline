# ADR 0006 — Workspace VFS identity, file watching, save semantics, and cache identity

- **Decision id:** D-0003 (see `artifacts/governance/decision_index.yaml#D-0003`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-07-15
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** none

## Context

Workspace / VFS truth is the fastest place an IDE can quietly fracture. The
same bytes can appear through multiple logical paths, remote aliases,
symlinks, junctions, case-only variants, Unicode-normalization variants, or
generated overlays. A write that looks correct at the string level can land
on a different real object — or on no object at all — when the root is
remote, virtual, cloud-backed, or policy-constrained. A watcher that drops
events without telling the surface can make the editor offer a "safe" save
against on-disk state that has already drifted. Every other lane (buffer,
editor, search, graph, review, AI apply, mutation journal, support-bundle
exporter, replay) trusts the VFS to resolve identity once and to coordinate
save honestly; if those rules are settled implicitly per feature, the
product's truth claim — the editor, the breadcrumb, the file tree, and the
CLI agree on what the workspace currently contains — cannot be defended.

The freeze matters because later work cannot land honestly on top of an
undecided VFS contract: the buffer lane (ADR 0003) cannot promise that save
participants compose with conditional-write semantics without a shared
save-target token; the subscription lane (ADR 0005) cannot label
`workspace_vfs` frames `authoritative` without a stable canonical identity;
the eventual mutation-journal and replay lanes cannot reconstruct "which
real object was saved at 14:07 local" without a frozen identity record; and
remote / container adaptation cannot ship without a capability envelope
that tells each surface which guarantees hold for the current root.

This ADR closes `D-0003` (Workspace VFS path identity and watcher model)
ahead of its `2026-07-15` freeze so the buffer, editor, search, graph,
review, AI apply, mutation-journal, support-export, CLI, and remote-agent
lanes can start instrumenting against one identity and save contract. It is
scoped to the **foundations seed contract** — the identity layers, watcher
posture, save pipeline, root capabilities, and cache-identity rules every
protected surface MUST agree on today. The full remote-agent implementation,
the precise platform-specific `fsync` strategy, and the final generated-
artifact lifecycle are out of scope; this ADR freezes only the vocabulary
and invariants those later lanes will reuse.

The VFS identity record rides alongside the ADR-0004 event envelope and the
ADR-0005 subscription envelope (both for `workspace_vfs` frames and for
derived producers that name VFS input digests in `producer_refs`). This ADR
does not redefine transport or reactive truth; it defines the filesystem-
identity, watcher, save, and cache fields those envelopes refer to.

## Decision

Aureline freezes a single **filesystem-identity model** (five layers —
presentation path, logical workspace identity, canonical filesystem object,
alias set, save-target token), one **watcher-source / watcher-health
contract**, one **save pipeline** (compare-before-write on the strongest
available generation token, atomic-replace preferred with named degraded
classes, save-participant composition that never silently downgrades), one
**root-capability envelope** (local POSIX-like, local Windows-like, remote
agent mount, virtual / generated, with named capability flags for
case-only rename, Unicode normalization change, symlink / junction escape,
read-only / policy-constrained, remote / container adaptation, and review-
before-save / review-before-rename), one **cache-identity and
versioning** rule (content digest and snapshot lineage, durable vs
disposable state boundary), and a set of **protected-hot-path hooks** the
benchmark lab, the support-bundle exporter, and the eventual mutation-
journal / replay lanes instrument against.

All are stated in terms of contract, vocabulary, and hook names rather than
specific crates so dependency refresh is a hygiene change, not a re-
litigation.

### Filesystem-identity layers

Every file-open, rename, save, autosave, compare, restore, and AI / apply
flow MUST carry all five identity layers where the underlying root can
provide them. No lane MAY invent parallel names.

| Layer                          | What is stored                                                                                                              | Purpose                                                                                                       |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|
| `presentation_path`            | Original opened URI plus display label and root badge.                                                                      | Tabs, breadcrumbs, copy / paste, and user-visible messages preserve the path the user chose where safe.       |
| `logical_workspace_identity`   | `workspace_id`, `root_id`, logical URI, trust state, policy scope.                                                          | Search, history, AI, review, and CLI address one workspace object even if the visible path changes.           |
| `canonical_filesystem_object`  | Strongest available: device + inode + generation (POSIX-like); file ID / object ID (Windows-like); provider object ID or remote revision token (remote); logical document id plus source refs (virtual / generated). Plus canonical URI and Unicode normalization form. | Save and external-change decisions target the real underlying object, not only the last rendered string.      |
| `alias_set`                    | All known alternative paths to the same canonical object plus the resolution chain (symlink path, junction path, hardlink sibling, case-only variant, normalization variant, remote alias, bind-mount alias, container-mount alias). | Duplicate tabs and wrong-target saves are prevented or explained rather than discovered after corruption.     |
| `save_target_token`            | `capability_flags`, `atomic_write_mode`, `compare_before_write_generation_token`, `permission_snapshot`, `review_required_before_save`, `review_required_before_rename`. | The product can state exactly what object a save is about to replace and which guarantees hold.               |

**Identity rules (frozen).**

1. All file-open, rename, save, autosave, compare, restore, external-
   change, and AI / apply flows MUST carry both `presentation_path` and
   `canonical_filesystem_object` when the root can provide the latter.
2. If multiple `presentation_path`s resolve to the same
   `canonical_filesystem_object`, the VFS MUST converge them onto one
   dirty-buffer authority and surface the alias relationship. A surface
   MAY NOT silently treat aliases as separate mutable files.
3. Case-only renames on case-insensitive roots, Unicode-normalization
   changes, and symlink / junction boundary crossings require an explicit
   capability check and a previewable rename plan. A root that lacks the
   capability MUST NOT be asked to simulate it.
4. External-change detection and merge prompts key on
   `canonical_filesystem_object` first and on the path string only second.
   "The same file through a different alias" MUST NOT look like an
   unrelated overwrite.
5. No feature-specific direct filesystem crawl or ad hoc path
   normalization is allowed outside the VFS contract. Raw platform paths
   are adapter concerns, not surface concerns.
6. Support bundles, recovery manifests, and mutation-journal entries MUST
   include enough identity metadata to explain which real object was
   opened or saved without requiring the user to reconstruct symlink,
   junction, case-fold, or normalization behaviour by hand.

### Watcher-source and watcher-health contract

Watcher behaviour is normalised through the VFS. No surface MAY wire its
own filesystem watcher.

**Watcher sources (frozen set).**

| Source                         | Used for                                                                          |
|--------------------------------|-----------------------------------------------------------------------------------|
| `os_native_watcher`            | Local roots where the operating system provides change notifications.            |
| `remote_agent_watcher_stream`  | Remote and container roots, delivered through the remote-agent transport.        |
| `scalable_watcher_integration` | Opt-in Watchman-class integration for very large local repositories.             |
| `polling_fallback`             | Degraded environments where no notification source is available or all have lost fidelity. |

Adding a new watcher source requires a new decision row.

**Watcher-health taxonomy (frozen).** Every surface that renders watcher-
derived truth reads `watcher_health` and surfaces the degraded states
visibly.

| Health             | Meaning                                                                                                   |
|--------------------|-----------------------------------------------------------------------------------------------------------|
| `healthy`          | Primary source is active; no known lost events.                                                           |
| `warming`          | Primary source is still attaching or replaying an initial enumeration.                                    |
| `degraded`         | Primary source is active but has lost fidelity (overflow, missed subtree, coalesced batch).               |
| `fallback_polling` | Primary source failed; the VFS has switched to polling. Save / rename costs are unchanged; event latency is worse. |
| `unavailable`      | No source is usable for the root. Surfaces MUST label this state; save is not disabled, but honesty about external change is.|

**Watcher guarantees (frozen).**

- Event de-duplication and cause attribution where the source supports it.
- No feature-specific hidden watchers.
- Stale or degraded watcher states surface visibly to the user and to
  support tooling, quoting `watcher_health` verbatim.
- Save and external-change behaviour MUST NOT depend on watcher
  perfection. `compare_before_write` is the floor; the watcher is a
  latency optimisation, not a correctness guarantee.
- A watcher-health transition toward less healthy emits a named frame on
  the VFS subscription (ADR 0005 `subscription_freshness_downgrade` with
  `stale_reason = watcher_dropped`) and fires the
  `vfs_watcher_health_changed` hook.

### Save pipeline

Saves and autosaves run through one pipeline. Feature code MAY NOT bypass
it with ad hoc writes.

1. **Stage.** The editor (or AI apply, or review apply) stages a buffer
   snapshot against a `save_target_token` pinned at open time. The token
   carries the generation token observed at open.
2. **Save-participant chain.** Eligible save participants run on the
   staged content in a frozen order (text normalisation → formatter →
   imports / code-actions → AI apply → user-registered participants). The
   chain composes under ADR 0003's save-participant-group undo class. A
   failing participant MUST NOT silently downgrade the save to an unsafe
   write-back; the pipeline fails closed with a typed
   `save_participant_failed` reason.
3. **Compare-before-write.** Before any byte reaches the root, the VFS
   re-reads the `canonical_filesystem_object`'s strongest available
   generation token and compares it to the token on the
   `save_target_token`. A mismatch is an external-change conflict; the
   pipeline emits a typed `external_change_detected` frame with diff
   metadata and MUST NOT overwrite without user or policy resolution.
4. **Atomic replace (preferred).** On roots whose
   `atomic_write_mode = atomic_replace`, the VFS writes to a sibling
   temporary file on the same volume, `fsync`s the content and the
   directory entry (platform-specific details deferred), then renames
   over the target. The rename is the commit point; crash recovery reads
   the journal (ADR 0003) rather than the temporary file.
5. **Degraded save classes.** When `atomic_replace` is not available, the
   root advertises one of the named degraded classes below; the VFS uses
   that class and the surface labels the degraded guarantee visibly.
6. **Journal and manifest.** On commit, the VFS records a save manifest
   carrying `presentation_path`, `canonical_filesystem_object`,
   `generation_token`, `capability_mode`, `save_participant_group_id`,
   and the resulting checkpoint ref. The ADR-0003 recovery journal and
   the mutation journal (deferred appendix) read this manifest.

**Save-mode taxonomy (frozen).**

| Save mode                    | Meaning                                                                                                         |
|------------------------------|-----------------------------------------------------------------------------------------------------------------|
| `atomic_replace`             | Temp-write + fsync + atomic rename on the same volume. Preferred on any root that can provide it.               |
| `in_place_write`             | Direct write to the target; used only when a tool requires it or when the root cannot rename (see capability matrix). Surface labels the degraded guarantee. |
| `conditional_remote_write`   | Write with a remote revision token / ETag precondition; a stale token yields a conditional-write conflict rather than a blind overwrite. |
| `blocked`                    | The root is read-only, policy-constrained, or the save target requires review; no write is issued. The surface offers review / alternate-target affordances only. |

A save-target that cannot name one of these modes MUST NOT be offered a
save affordance.

**Conflict path (frozen).** `external_change_detected` is the only conflict
outcome. The pipeline MUST surface one of: `review_diff`, `overwrite`,
`merge`, `save_as`, or `cancel`. Silent overwrite is never a resolution.

**Autosave.** Autosave runs the same pipeline with the same
`compare_before_write` and the same save manifest; it MAY skip save
participants that the user has deferred to explicit save. Autosave MUST
NOT degrade to a write that would not be legal under explicit save.

### Root-capability envelope

Every root advertises a capability envelope at attach time. The envelope
is the input to every save-mode and rename decision. The full matrix
lives in `artifacts/io/root_capability_matrix.yaml`; the frozen capability
flags are:

| Flag                                  | Meaning                                                                                                  |
|---------------------------------------|----------------------------------------------------------------------------------------------------------|
| `supports_atomic_replace`             | Root supports temp-write + rename on the same volume as the target.                                      |
| `supports_in_place_write`             | Root accepts direct in-place writes.                                                                     |
| `supports_conditional_remote_write`   | Root provides a revision token / ETag usable as a write precondition.                                    |
| `case_sensitivity`                    | One of `sensitive`, `insensitive_preserving`, `insensitive_non_preserving`.                              |
| `unicode_normalization`               | One of `none`, `nfc`, `nfd`, `mixed_observed`. Controls whether a normalization-changing rename is legal.|
| `supports_case_only_rename`           | Root can perform a case-only rename atomically (requires explicit check + preview).                      |
| `supports_unicode_normalization_rename`| Root can perform a normalization-only rename atomically (requires explicit check + preview).            |
| `symlink_escape_policy`               | One of `allow`, `warn`, `block`. Governs writes whose canonical path resolves outside the root.          |
| `read_only`                           | Root disallows writes; save-mode is `blocked` and rename is refused.                                     |
| `policy_constrained`                  | Root permits writes only under policy (trust mode, admin, review). Save-mode is `blocked` until policy admits. |
| `review_required_before_save`         | Writes require a review step before the pipeline can reach step 4.                                       |
| `review_required_before_rename`       | Renames require a review step before they are executed.                                                  |
| `remote_container_adaptation`         | Root is served via remote agent or container; the agent / container envelope carries extra flags (remote revision token, mount graph hash, path-semantics metadata, watcher model metadata). |
| `strongest_identity_token`            | Declares the strongest identity token the root can provide: `file_id_generation`, `device_inode_generation`, `windows_object_id`, `provider_object_id_revision`, `logical_document_id_source_refs`, or `content_hash_only`. |
| `fallback_identity_tokens`            | Ordered list of fallback tokens the root guarantees, drawn from `{device_inode, inode_mtime_size, content_hash, remote_revision_token}`. |

A root that cannot honestly advertise any identity token above
`content_hash_only` MUST NOT be offered save affordances that require
exact-object promises; the surface downgrades to an export / diff
affordance.

### Cache identity and versioning

All on-disk caches (search shards, graph indexes, docs packs, symbol
bundles, preview runtimes, thumbnail caches, imported-index materialisations,
support-bundle exports, replay captures) key by cache identity, not by
path. A path may change (rename, alias, case-only rewrite) without
invalidating a cache whose inputs have not changed.

**Cache-identity fields (frozen).** Every cache entry records:

- `cache_class` — one of `ephemeral_projection`, `durable_local_materialization`,
  `exportable_snapshot`, `managed_replicated_view` (matches ADR-0005
  `view_class`). The class governs persistence and invalidation.
- `cache_schema_version` — integer; bumped on breaking payload changes.
- `producer_id` / `producer_version` — producer attribution.
- `input_digest_set` — one or more named inputs, each carrying a content
  digest. `canonical_filesystem_object` digests MUST be canonical content
  hashes or the root's strongest identity token, never a path string.
- `output_digest` — content digest of the cache payload.
- `generated_at` — producer-local monotonic timestamp.
- `drift_state` — one of `in_sync`, `stale_inputs`, `generator_changed`,
  `manually_diverged`, `unknown_lineage` (matches Appendix DE.2 of the
  TAD).
- `durability_class` — `durable` (survives process restart and may be
  republished after a supersession check) or `disposable` (recomputable
  and always invalidated on producer restart).

**Invalidation rules (frozen).**

1. A cache is stale when any entry in `input_digest_set` is observed
   different from the source. The cache is not stale merely because a
   watcher dropped; the producer re-reads the source before declaring
   staleness.
2. `durable` caches survive process restart only if a supersession check
   on `input_digest_set` succeeds. A mismatch forces a rebuild; the
   producer MAY NOT serve `freshness = authoritative` from a cache whose
   inputs have changed.
3. `disposable` caches are dropped on producer restart. Surfaces that
   depend on them label `freshness = warming` during rebuild.
4. A cache that can be reconstructed from authority is `durable` only if
   the producer also commits to bounded rebuild cost under the benchmark
   lab's corpus. Otherwise the cache is `disposable`.
5. Imported and replayed caches (ADR 0005 `freshness = imported` or
   `replayed`) carry a source identifier in `producer_refs` and MUST
   NOT be promoted to authoritative under any local policy.

**Durable vs disposable state boundary.** The VFS and save pipeline own
all authority for files, canonical identities, save tokens, trust state,
and watcher health (ADR 0005 `workspace_vfs` authority class). Downstream
caches MAY project this authority but MAY NOT invent it. A cache whose
producer disappears does not silently become authority.

### Failure cases

The pipeline distinguishes the failure classes below; each has a typed
reason that support bundles and diagnostics quote verbatim.

| Failure case                          | What it means                                                                                                | How the pipeline behaves                                                                                         |
|---------------------------------------|--------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| `external_change_detected`            | Compare-before-write observed a generation-token mismatch. The on-disk object drifted between open and save. | Fail closed; emit diff metadata; offer `review_diff` / `overwrite` / `merge` / `save_as` / `cancel`. No blind overwrite. |
| `watcher_uncertainty`                 | Watcher health transitioned to `degraded`, `fallback_polling`, or `unavailable`. Saves remain legal; external-change honesty is downgraded. | Emit `subscription_freshness_downgrade` with `stale_reason = watcher_dropped`; compare-before-write still runs at save time. |
| `save_conflict`                       | `compare_before_write` or a remote conditional write reported a stale precondition.                          | Emit `save_conflict`; refuse to commit; offer the same conflict affordances as `external_change_detected`.       |
| `wrong_target_prevention`             | The save-target's `canonical_filesystem_object` or alias set does not match the editor's buffer authority.   | Fail closed with `wrong_target_prevented`; surface the alias relationship; require the user to reselect target. |
| `save_participant_failed`             | A save participant raised an error.                                                                          | Fail closed; do not write; preserve the unformatted buffer; surface which participant failed and why.            |
| `degraded_guarantee_declared`         | The root cannot provide `atomic_replace` and the chosen mode is degraded (`in_place_write`, `conditional_remote_write`, or `blocked`). | Save proceeds under the declared mode; surface labels the degraded class visibly; support bundle records the class. |
| `generated_or_managed_write_blocked`  | The save target is a generated / managed file (build output, codegen output, lockfile, notebook output, preview snapshot, mirrored pack) whose artifact-class policy blocks direct edits. | Fail closed with the Appendix-DE drift-state taxonomy; offer `regenerate` / `restore` / `override-with-divergence-badge` as the artifact class allows. |
| `read_only_or_policy_blocked`         | Root is `read_only` or `policy_constrained`; no write is legal.                                              | Save-mode is `blocked`; surface offers review, `save_as`, or export affordances only.                            |
| `review_required_before_save`         | Root requires a review step before commit.                                                                   | Pipeline halts at step 3; surface routes to review; commit resumes only on review acceptance.                    |
| `review_required_before_rename`       | Root requires a review step before rename.                                                                   | Rename is refused until review accepts.                                                                          |

### Protected-hot-path hooks

The VFS and save pipeline expose the following named hooks. They are the
canonical instrumentation surface for the buffer, editor, search, review,
AI apply, mutation-journal, support-export, replay, and benchmark lanes.
No lane MAY invent alternative names for the same measurement. These hooks
sit above ADR-0004 transport hooks and ADR-0005 subscription hooks and do
not replace them.

| Hook id                              | Fires when                                                                                                    | Protected hot-path budget |
|--------------------------------------|---------------------------------------------------------------------------------------------------------------|---------------------------|
| `vfs_root_attach`                    | A workspace root is attached and its capability envelope is registered.                                       | yes                       |
| `vfs_root_detach`                    | A workspace root is detached.                                                                                 | yes                       |
| `vfs_canonicalize`                   | The VFS resolves a presentation path to a canonical filesystem object.                                        | yes                       |
| `vfs_alias_converge`                 | Two presentation paths are detected to resolve to the same canonical object and are converged onto one buffer authority. | yes                       |
| `vfs_watcher_event`                  | A primary or fallback watcher emits an event.                                                                 | yes                       |
| `vfs_watcher_health_changed`         | `watcher_health` transitions (either direction).                                                              | no (observability only)   |
| `vfs_external_change_detected`       | The VFS observes a canonical-object change from an external source.                                           | yes                       |
| `vfs_save_stage`                     | The save pipeline stages a buffer snapshot against a save-target token.                                       | yes                       |
| `vfs_save_participant_run`           | A save participant starts running on staged content.                                                          | yes                       |
| `vfs_save_participant_failed`        | A save participant fails; the pipeline halts.                                                                 | yes                       |
| `vfs_save_compare_before_write`      | The VFS re-reads the target's generation token and compares it to the save-target token.                      | yes                       |
| `vfs_save_conflict`                  | Compare-before-write or a conditional-write precondition yields a conflict.                                   | yes                       |
| `vfs_save_atomic_commit`             | An `atomic_replace` save commits via rename.                                                                  | yes                       |
| `vfs_save_in_place_commit`           | An `in_place_write` save commits.                                                                             | yes                       |
| `vfs_save_remote_conditional_commit` | A `conditional_remote_write` save commits.                                                                    | yes                       |
| `vfs_save_blocked`                   | A save-mode is `blocked` for the attempted target.                                                            | no (observability only)   |
| `vfs_save_manifest_record`           | The VFS writes the save manifest (post-commit).                                                               | yes                       |
| `vfs_rename_plan_previewed`          | The VFS produces a rename plan preview (case-only, normalization, or symlink / junction escape).              | no (observability only)   |
| `vfs_rename_commit`                  | A rename commits.                                                                                             | yes                       |
| `vfs_cache_lookup`                   | A cache consumer requests a cache entry.                                                                      | yes                       |
| `vfs_cache_invalidate`               | A cache entry is invalidated because an `input_digest_set` member changed.                                    | yes                       |
| `vfs_cache_rebuild`                  | A `durable` cache supersession check fails and a rebuild begins.                                              | no (observability only)   |
| `vfs_degraded_guarantee_declared`    | A save commits under a degraded class (`in_place_write`, `conditional_remote_write`).                         | no (observability only)   |

The benchmark lab reports every hot-path hook against its protected budget
on the claimed corpora (local POSIX-like root, local Windows-like root,
remote agent mount, container mount, very-large-repo Watchman-class
integration, polling fallback) alongside the ADR-0004 transport hooks and
the ADR-0005 subscription hooks. Non-hot-path hooks are observability-only
and do not gate release.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- The full remote-agent implementation, its transport choice beyond the
  ADR-0004 seam, and its operational lifecycle. This ADR only names the
  capability envelope the agent must carry.
- The precise platform-specific `fsync` strategy (per-file, per-directory,
  `F_BARRIERFSYNC`, `FlushFileBuffers`, io_uring barriers). The ADR
  freezes the correctness floor (temp-write → durability barrier →
  rename-as-commit); the specific calls are a platform-adapter concern
  tracked under a later decision row.
- The final generated-artifact lifecycle. This ADR adopts the
  Appendix DE artifact-class vocabulary (`InSync`, `StaleInputs`,
  `GeneratorChanged`, `ManuallyDiverged`, `UnknownLineage`) but does not
  freeze the per-class mutation pipeline.
- Multi-root sparse-slice / workset policy. This ADR freezes the
  per-root identity and save contract; the sparse-slice policy opens a
  separate decision row.
- A CRDT-style cross-root merge. The save pipeline's floor is
  compare-before-write on a single canonical object; collaborative
  merge opens a new decision row.
- Public-SDK stability of the save-target token or the capability
  envelope. Both are internal at the foundations milestone; the public
  SDK surface lands behind a separate decision row.
- Any implicit per-surface save heuristic ("it looked fine so we wrote
  it"). Surfaces use the frozen save-mode taxonomy; they do not invent
  one.

These lines move only by opening a new decision row, not by editing this
ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/vfs_tradeoff_rows.yaml`. Headline summary:

| Axis                                  | Chosen stack                                                                                                      | Best rejected alternative                                               | Why chosen wins                                                                                       |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| **Identity layering**                 | Five-layer identity: presentation, logical, canonical, alias set, save-target token                               | Path-as-identity (a string is the file)                                 | Path-as-identity fractures on symlinks, case-folds, normalisation, remote aliases; the five-layer rule is what lets save be honest about "same file, different alias" |
| **Watcher correctness**               | `compare_before_write` is the correctness floor; watcher is a latency optimisation                                | "Trust the watcher" (skip comparison when the watcher says nothing changed) | Watchers drop events under overflow, replay, remote-agent reconnect; trusting them silently corrupts files |
| **Save mode**                         | `atomic_replace` preferred; named degraded classes (`in_place_write`, `conditional_remote_write`, `blocked`) with visible labels | Always-in-place write; or always-atomic (fail where it is not possible) | Always-in-place loses crash safety on local; always-atomic refuses legitimate remote / policy-constrained roots; named degraded classes keep honesty |
| **Root capability envelope**          | Per-root capability envelope declared at attach; surfaces read flags                                              | One global FS posture                                                   | One global posture forces the product to either assume POSIX everywhere (breaks Windows / remote) or pessimise everywhere (breaks performance on POSIX) |
| **Alias handling**                    | Alias sets converge onto one dirty-buffer authority with disclosure                                               | Each path is a separate mutable file                                    | Two dirty buffers for the same file is a corruption path; disclosure + convergence is the only honest posture |
| **Cache identity**                    | Content-digest plus input-digest-set keys, `durable` vs `disposable` boundary, producer-owned invalidation        | Path-keyed caches invalidated by mtime alone                            | Path-keyed caches survive path changes that should invalidate them and invalidate on path changes that should not; digest keys match the inputs that actually changed |
| **Conflict posture**                  | `external_change_detected` + typed affordances; never a silent overwrite                                          | Last-writer-wins default                                                | Last-writer-wins silently loses the user's other session's work; typed affordances are the only auditable outcome |
| **Schema of record**                  | Rust types in the VFS crate are the source of truth; JSON Schema export at `schemas/runtime/vfs_save_envelope.schema.json` | External IDL + codegen at this milestone                                | No second-language consumer yet; the boundary schema reserves a clean integration point |

Each row carries reopen triggers in the YAML. A benchmark finding that
`vfs_save_compare_before_write` exceeds its budget on a claimed corpus,
or a support-bundle export that cannot reconstruct which real object was
saved from the labelled frames, reopens the relevant row.

### Decision-example fixtures

A small corpus of decision-example fixtures lives under
`fixtures/runtime/vfs_decision_examples/`. They are short, reviewable
scenarios (atomic save on a local POSIX root, in-place degraded save on a
cross-volume target, remote conditional-write conflict, external-change
detected via compare-before-write, case-only rename on a case-insensitive
root, symlink escape blocked by policy, alias convergence onto one dirty
buffer, generated-file write blocked by artifact class, read-only root
blocked, watcher fallback to polling) used by the VFS, buffer, editor,
search, review, AI apply, mutation-journal, and support-export lanes to
anchor the hook names, the identity layers, the save-mode taxonomy, the
capability flags, and the failure-case reasons to concrete inputs and
observable outcomes. They are not a test suite; they are the language the
ADR's hook list and taxonomy refer to.

## Consequences

- **Frozen:** the five identity layers (`presentation_path`,
  `logical_workspace_identity`, `canonical_filesystem_object`,
  `alias_set`, `save_target_token`), the identity rules, the save-mode
  taxonomy (`atomic_replace`, `in_place_write`, `conditional_remote_write`,
  `blocked`), the watcher-source set (`os_native_watcher`,
  `remote_agent_watcher_stream`, `scalable_watcher_integration`,
  `polling_fallback`), the watcher-health taxonomy (`healthy`, `warming`,
  `degraded`, `fallback_polling`, `unavailable`), the root-capability
  flag set, the failure-case reason set, and the protected-hot-path
  hook names.
- **Frozen:** the cache-identity fields (`cache_class`,
  `cache_schema_version`, `producer_id`, `producer_version`,
  `input_digest_set`, `output_digest`, `generated_at`, `drift_state`,
  `durability_class`) and the durable-vs-disposable boundary.
- **Frozen:** the schema of record is Rust types in the VFS crate; the
  boundary schema lives at
  `schemas/runtime/vfs_save_envelope.schema.json`; there is no external
  IDL or codegen toolchain at this milestone. This mirrors ADR 0004 and
  ADR 0005.
- **Frozen:** `compare_before_write` is the save correctness floor.
  No surface MAY skip it on the assertion that the watcher reported no
  change.
- **Frozen:** feature code MAY NOT bypass the save pipeline with ad hoc
  filesystem writes. Tooling integrations that need direct writes land
  as save participants or as declared adapters behind the capability
  envelope.
- **Permitted:** adding a new watcher source, a new capability flag, or
  a new failure-case reason is an additive-minor change recorded in the
  capability matrix and the tradeoff register; repurposing any existing
  value is breaking and requires a new decision row.
- **Permitted:** the platform adapter chooses the specific durability
  barrier call (`fsync`, `F_BARRIERFSYNC`, `FlushFileBuffers`, io_uring
  barriers). The ADR only freezes that a barrier runs between content
  write and rename.
- **Follow-up:** the buffer, editor, search, review, AI apply, mutation-
  journal, and support-export lanes instrument every hot-path hook before
  claiming save or external-change guarantees. The benchmark lab
  stabilises traces against the hooks on the claimed corpora.
- **Follow-up:** the mutation-journal lane (a later decision row) adopts
  the save-manifest fields above as its filesystem-mutation journal
  schema; it does not invent a second identity record.
- **Follow-up:** the remote-agent lane (a later decision row) ships the
  capability envelope above as its attach-time handshake payload.
- **Ratifies:** the ADR-0005 `workspace_vfs` authority class, its
  producer refs, and its `stale_reason = watcher_dropped` /
  `upstream_input_stale` codes now refer to the hooks, save manifest,
  and watcher-health taxonomy frozen here.

## Alternatives considered

- **Path-as-identity.** Treat the file path string as the file's
  identity. Rejected: symlinks, junctions, case-insensitive roots,
  Unicode normalisation, remote aliases, and bind / container mounts
  all create cases where two paths point at one object or one path
  points at different objects over time. A save pipeline built on path
  identity cannot honestly explain what it wrote.
- **Trust the watcher.** Skip `compare_before_write` when the watcher
  reports no change. Rejected: every watcher source loses events under
  overflow, replay, or remote-agent reconnect. Trusting the watcher
  silently corrupts files whenever the watcher is wrong; the benchmark
  lab cannot instrument a correctness floor that depends on a perfect
  watcher.
- **Always in-place writes.** Skip atomic replace everywhere so the
  pipeline is one code path. Rejected: on local roots this loses crash
  safety that atomic replace provides for free; the support bundle
  cannot promise that a saved file is either the old content or the new
  content.
- **Always atomic replace.** Refuse any root that cannot provide atomic
  replace. Rejected: remote, container, policy-constrained, and some
  read-only-overlay roots legitimately cannot provide it; refusing them
  would break the workspace-on-remote-agent use case and the cloud-
  backed file use case.
- **Path-keyed caches invalidated by mtime.** Rejected: path-keyed
  caches survive path changes that should invalidate them (a rename
  that swaps two files silently keeps the old cache), and they
  invalidate on path changes that should not (a case-only rewrite on a
  case-insensitive root blows away a valid cache). Digest-keyed caches
  match the inputs that actually changed.
- **One global filesystem posture.** Pick one capability set and apply
  it everywhere. Rejected: forces the product to either assume POSIX
  everywhere (breaks Windows, remote, and virtual roots) or to
  pessimise everywhere (breaks performance on POSIX). The per-root
  capability envelope is the only posture that stays honest.
- **Last-writer-wins conflict resolution.** Silently overwrite on
  external change. Rejected: the user's other session's work is lost
  without trace; the support bundle cannot explain who wrote what when.
- **External IDL + generator for the save / capability payload.**
  Adopt a separate IDL. Rejected: same argument ADR 0004 and ADR 0005
  make — an IDL without a second-language consumer costs more than it
  buys; the JSON Schema export reserves the integration point.
- **Defer to a later milestone.** Leave `D-0003` open and let the
  narrowing default apply on `2026-07-15`. Rejected: the narrowing
  default ("single-root, canonical-path, polling-fallback watcher
  model; defer multi-root and platform-specific watcher
  optimisations") would block remote / container adaptation and the
  review-required-before-save policy class exactly when later lanes
  need the frozen vocabulary; the mutation-journal and replay lanes
  would land with incompatible identity assumptions the lane would
  then have to reconcile.

The `D-0003` `narrow` default-if-unresolved posture would have locked
the VFS to a single-root, canonical-path, polling-fallback watcher model
until an ADR landed. Accepting this ADR replaces that narrowing with
the frozen identity model, watcher-source and health contract, save
pipeline, root-capability envelope, cache-identity rules, failure-case
reasons, and hook list above; the narrowing default does not apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1108` — "5.17 Virtual file system and file
  watching".
- `.t2/docs/Aureline_PRD.md:1114` — "path identity must track case
  sensitivity per root, symlink policy, inode/mtime fingerprints".
- `.t2/docs/Aureline_PRD.md:1115` — "save flow defaults to atomic write
  via temp-file plus rename".
- `.t2/docs/Aureline_PRD.md:1116` — "watchers should prefer OS-native
  notifications for local files and agent-side notifications for remote
  files".
- `.t2/docs/Aureline_PRD.md:1117` — "very large local repositories may
  optionally integrate with a file-watching service such as Watchman".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:43` — TOC:
  "Real filesystem identity, canonical path, and save-coordination
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1597` — "VFS
  responsibilities".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1608` — "12.2
  Watcher model".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1614` — "OS-native
  local watchers; remote-agent watcher streams".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1621` — "event
  de-duplication and cause attribution where possible".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1624` — "save and
  external-change behavior do not depend on watcher perfection".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1628` — "12.2.1
  Real filesystem identity, canonical path, and save-coordination
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1630` — "the path
  the user opened, the logical workspace object, and the canonical
  filesystem object".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1641` — "all
  file-open, rename, save, external-change, compare, restore, and
  AI/apply flows must carry both the presentation path and the canonical
  object identity".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1642` — "if
  multiple paths resolve to the same canonical object, Aureline should
  converge them onto one dirty-buffer authority".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1643` —
  "case-only renames on case-insensitive roots, Unicode-normalization
  changes, and symlink/junction boundary crossings require explicit
  capability checks and previewable rename plans".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1644` — "save
  and autosave flows must use compare-before-write semantics".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1645` — "atomic
  replace is the preferred save mode, but roots that cannot provide it
  must advertise the degraded guarantee explicitly".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1646` —
  "external-change detection and merge prompts key on canonical object
  identity first and only path string second".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1647` — "no
  feature-specific direct filesystem crawl or ad hoc path normalization
  is allowed outside the VFS contract".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1648` — "support
  bundles, recovery manifests, and mutation-journal entries must include
  enough identity metadata".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9674` —
  "Appendix DC — Filesystem Identity, Canonical Path, and
  Save-Coordination Matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9678` — "DC.1
  Root and identity matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9685` — "DC.2
  Alias and rename edge cases".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9695` — "DC.3
  Save-coordination rules".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9697` — "save
  manifests record presentation path, canonical object, generation
  token, capability mode, and resulting checkpoint".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9699` — "feature
  code must never bypass VFS save coordination with ad hoc file writes".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9708` —
  Filesystem mutation journal fields: "canonical identity, alias set,
  save token, affected path set".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9740` —
  "DE.2 Drift-state matrix".
- `.t2/docs/Aureline_Technical_Design_Document.md:1446` — "7.2
  Workspace, VFS, state, and persistence".
- `.t2/docs/Aureline_Technical_Design_Document.md:1467` — "7.2.2
  Watcher model".
- `.t2/docs/Aureline_Technical_Design_Document.md:1476` — "Every
  degraded watcher state must be visible to the user and to support
  tooling".
- `.t2/docs/Aureline_Technical_Design_Document.md:1478` — "7.2.3
  Filesystem identity".
- `.t2/docs/Aureline_Technical_Design_Document.md:1488` — "This is
  required to prevent duplicate dirty states, wrong-target saves,
  case-only surprises, and symlink confusion".

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0003`
- RFC: none.
- Tradeoff register (machine form):
  `artifacts/architecture/vfs_tradeoff_rows.yaml`.
- Root-capability matrix (machine form):
  `artifacts/io/root_capability_matrix.yaml`.
- Boundary schema (machine form):
  `schemas/runtime/vfs_save_envelope.schema.json`.
- Decision-example fixtures:
  `fixtures/runtime/vfs_decision_examples/`.
- Transport contract the save pipeline rides:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Reactive-truth contract the `workspace_vfs` authority class rides:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- Buffer / undo contract the save-participant chain composes with:
  `docs/adr/0003-buffer-undo-large-file.md`.
- Affected lanes: `crates/aureline-vfs`, `crates/aureline-buffer`,
  `crates/aureline-rpc`, `crates/aureline-telemetry`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:benchmark_lab`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:support_export`.

## Supersession history

First acceptance. No supersession.
