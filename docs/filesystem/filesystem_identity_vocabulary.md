# Filesystem identity, alias-set, save-target-token, and semantic-readiness vocabulary

This document freezes the cross-surface vocabulary the product, CLI,
companion surfaces, support/export, and later graph/search work all
rely on when they have to name "which file is this, really?" or
"how ready is this workspace?" It is the seed the VFS prototype, the
supportability docs, and the M0 architecture pack reuse; it is also
the seed later readiness-inspector / support-packet surfaces reuse so
they do not invent a parallel readiness dialect.

The ADR of record for the underlying filesystem-identity, watcher,
save-pipeline, root-capability, and cache-identity rules is
[ADR 0006](../adr/0006-vfs-save-cache-identity.md). This document
does not restate ADR 0006; it pins the names every non-VFS surface
must use when it renders, logs, exports, or explains the same
objects. The ADR wins on any disagreement; this file and the
companion schema under
[`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
are updated in the same change.

## Why freeze this now

A workspace IDE answers four questions about every file hundreds of
times per second: *which logical object am I looking at*, *which
real filesystem object will my next write land on*, *which known
aliases also point at that object*, and *how much of what I claim
about the object is currently trustworthy*. Left implicit, each
surface answers them slightly differently — the tab title says one
thing, the breadcrumb says another, the CLI renders a third, the
support bundle exports a fourth, and the AI surface acts on a
fifth. The goal here is one frozen vocabulary so every surface tells
the same story and later readiness / not-ready / safe-next-action
work composes over the same fields rather than re-deriving them.

## Scope

- Freeze the presentation path, logical workspace identity,
  canonical filesystem object, alias set, and save-target token
  names, and the relationships between them.
- Freeze the **semantic-readiness** state vocabulary that labels how
  much a surface can claim about an object beyond "it exists on
  disk". Every graph-backed, index-backed, generator-backed, or
  imported-fact surface reads this vocabulary verbatim.
- Reserve the projection fields that later readiness chips,
  not-ready explainers, safe-next-action rows, and support/export
  packets will fill, so those packets do not invent alternate
  readiness dialects.
- Provide worked examples for path-case changes, symlink / junction
  aliases, moved files, and partially-ready workspaces.
- Seed the machine-readable schema at
  [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json).

## Out of scope

- The full remote-agent implementation, watcher-adapter internals,
  and platform-specific durability calls — owned by later decision
  rows and by the VFS crate adapters.
- Final persisted lifecycle of every readiness transition — this
  document freezes the vocabulary and its projection fields; the
  concrete state machine lives with the producer lane (graph,
  search, docs pack, generator, AI apply, review, support export).
- Pricing / packaging / boundary classification — owned by
  [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md).

## How this document relates to ADR 0006

ADR 0006 freezes the identity layers, watcher posture, save
pipeline, root-capability envelope, cache-identity rules, failure
cases, and protected-hot-path hooks. Those are all normative for
the VFS lane and for any surface that calls into save, rename, or
cache.

This document layers one contract on top of ADR 0006: **semantic
readiness**. Semantic readiness is the vocabulary every non-VFS
surface (CLI, support bundles, graph, search, docs pack, AI apply,
review, product chrome) uses to explain *why a claim about an
object is or is not trustworthy right now* — independent of whether
the bytes on disk changed. ADR 0006 answers "which object and what
did save do?"; this document answers "how ready is what we said
about that object?"

## 1. Identity vocabulary

Every surface that names a file by any path, shows a file in chrome,
logs a save, exports a support packet, or attributes a mutation
uses the following five terms and nothing else.

### 1.1 `presentation_path`

The path the user opened, verbatim. Tabs, breadcrumbs, command-
palette entries, copy / paste payloads, support bundle entries, and
CLI output preserve this string where it is safe to do so. A
surface may not silently rewrite `presentation_path` to the
canonical URI; if the surface needs to display the canonical form,
it renders the canonical URI *alongside* `presentation_path` with
an explicit alias disclosure, not by replacement.

Required fields: `uri`, `display_label`, `root_badge`. A full
schema definition is inherited from ADR 0006's
`vfs_save_envelope.schema.json#/$defs/presentation_path`.

### 1.2 `logical_workspace_identity`

The workspace object the product is tracking. `workspace_id` and
`root_id` are stable across rename, alias change, or canonical-path
renormalisation. `logical_uri` is the workspace-relative logical
address that search, history, CLI, AI, and review all address. The
logical URI does not encode a specific alias; it survives a
case-only rename or a symlink-target swap that leaves the canonical
object unchanged.

### 1.3 `canonical_filesystem_object`

The real underlying object, named by the strongest identity token
the root can provide (see ADR 0006's `strongest_identity_token`
enum) plus optional fallback tokens. Save and external-change
decisions key on this object first, the presentation path second.

A surface that needs to explain "same file, different alias" quotes
`canonical_filesystem_object` and `alias_set` together; it does not
invent its own "these look like the same file" heuristic.

### 1.4 `alias_set`

All known alternative paths that resolve to the same
`canonical_filesystem_object`, each labelled with a frozen
`alias_kind` from the set `{symlink, junction, hardlink_sibling,
case_only_variant, unicode_normalization_variant, remote_alias,
bind_mount_alias, container_mount_alias, archive_inner_alias}` and
an optional `resolution_chain`. `alias_set` is authoritative for
duplicate-tab prevention and for the alias disclosure every non-VFS
surface renders; no surface may dedupe tabs by path-string
comparison alone.

### 1.5 `save_target_token`

The token the editor, AI apply, review apply, or CLI pins at open
time and that the save pipeline compares on write. Carries the
capability flags, the `atomic_write_mode` the token authorises, the
generation token observed at open, the permission snapshot, and
the review-before-save / review-before-rename gates. A surface
that cannot produce a `save_target_token` may not offer a save
affordance.

## 2. Semantic-readiness vocabulary

Semantic readiness labels how much a surface can honestly claim
about an object right now, independent of whether the bytes on
disk changed. It rides alongside the identity layers; every
surface that displays, exports, or reasons about an object renders
`semantic_readiness` verbatim. Later readiness-inspector and
support-packet work reads the same fields; it does not invent a
parallel vocabulary.

### 2.1 Frozen state set

| State           | Meaning                                                                                                                       |
|-----------------|-------------------------------------------------------------------------------------------------------------------------------|
| `exact`         | The producer derived its claims from the authoritative inputs for this exact object and has not lost fidelity.                |
| `imported`      | The claims were imported from an external source (vendored index, replayed capture, third-party bundle). Never authoritative. |
| `heuristic`     | The claims were produced by a fallback path (regex, ctags, filename heuristic, language-agnostic scan). Correctness is best-effort. |
| `stale`         | The producer last derived authoritative claims against inputs that have since changed; the claims have not been refreshed.   |
| `partial`       | The producer covered some of the inputs for this object but not all (partial enumeration, scope reduced by policy, incremental producer still warming). |
| `unavailable`   | The producer cannot serve any claim for this object right now (producer disabled, entitlement missing, policy blocked, adapter not loaded, offline). |
| `out_of_scope`  | The object lies outside the producer's declared scope (not indexed, ignored by policy, excluded by workset). Not a failure; a scope statement. |

`exact` is the only state that may ride with `freshness =
authoritative` on an ADR-0005 subscription frame. Every other state
MUST downgrade `freshness` accordingly.

### 2.2 Not-ready reason vocabulary

When a surface renders any state other than `exact`, it cites
exactly one `not_ready_reason` from the frozen set below. Support
bundles, attention-center rows, and readiness chips quote this
verbatim; it is the language the user and support share.

| Reason code                     | When it applies                                                                                                          |
|---------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| `producer_warming`              | Producer is still attaching or performing initial enumeration.                                                           |
| `producer_restart`              | Producer restarted; `disposable` caches were dropped (ADR 0006); the surface is waiting for a rebuild.                   |
| `watcher_degraded`              | The VFS watcher reported `degraded`, `fallback_polling`, or `unavailable`; external-change honesty is reduced.           |
| `input_digest_stale`            | One or more `input_digest_set` entries changed since the last producer run (ADR 0006 `stale_inputs`).                    |
| `generator_changed`             | The generator or producer version changed; prior output is no longer lineage-matched (ADR 0006 `generator_changed`).     |
| `manually_diverged`             | A managed / generated artifact was edited directly (ADR 0006 `manually_diverged`).                                       |
| `unknown_lineage`               | The producer cannot attribute the current output to any `input_digest_set` it recognises (ADR 0006 `unknown_lineage`).   |
| `imported_source`               | The claim was imported (replay, vendor bundle, third-party index) and MUST NOT be promoted to authoritative.             |
| `heuristic_fallback`            | The producer fell back to a heuristic path because the authoritative adapter was unavailable.                            |
| `partial_enumeration`           | Only part of the required inputs have been enumerated yet.                                                               |
| `scope_excluded`                | The object is outside the producer's declared scope (workset boundary, ignore pattern, policy bundle).                   |
| `entitlement_missing`           | The producer requires an entitlement the current identity / session does not carry.                                      |
| `policy_blocked`                | The producer is allowed to run but is prevented from publishing claims by local / organisation policy.                   |
| `offline_unreachable`           | The producer depends on a reachable service that is currently unreachable; local-core behaviour continues unchanged.     |
| `review_required`               | Publishing the claim requires a review step that has not completed.                                                      |
| `large_input_deferred`          | Producer declined to enumerate a large input within the current budget; the surface may retry on demand.                 |
| `corrupt_input_quarantined`     | An input was observed corrupt or unreadable and has been quarantined; surface explains the quarantine.                   |

Adding a new reason is an additive-minor change; repurposing an
existing code is breaking and requires a new decision row.

### 2.3 Safe-next-action vocabulary

Every `not_ready_reason` row carries a frozen `safe_next_action`
code so the product, CLI, and support all route users to the same
affordance. The set is intentionally small:

`retry_now`, `wait`, `refresh_input_digests`, `regenerate`,
`restore_from_backup`, `review_and_approve`, `widen_scope`,
`reduce_scope`, `reauthenticate`, `acquire_entitlement`,
`open_offline_details`, `open_support_bundle`, `open_alias_details`,
`open_review_diff`, `save_as`, `cancel`, `no_action_required`.

The mapping from `not_ready_reason` to the recommended
`safe_next_action` lives as machine-readable rows on the producer
(not in this document) so per-producer specialisation is possible
without forking the vocabulary. What this document freezes is the
action set itself.

### 2.4 Support / export parity

Every readiness record exports with the same fields it shows in
the UI. A support bundle, mutation-journal entry, or replay capture
includes at minimum:

- `semantic_readiness` (one state from §2.1)
- `not_ready_reason` (one code from §2.2, required when state is
  not `exact`)
- `safe_next_action` (one code from §2.3)
- `producer_id`, `producer_version`, `observed_at`
- The full identity-layer record (§1.1–§1.4) for the object the
  readiness claim is about
- Optional `explainer` free-text field bounded to 240 graphemes,
  redaction-aware; the product surface and the support bundle render
  the same explainer.

A surface may render fewer fields in compact chrome (e.g. a chip
shows state + reason only), but it MUST be able to reach the full
record through a keyboard-reachable detail affordance so users,
admins, and support see the same truth the exporter records.

## 3. Projection fields reserved for future packets

The following projection fields are reserved on every readiness
record. They are optional at the foundations milestone; M0
conformance packets MUST NOT invent alternate names for them.

| Field                              | Purpose                                                                                                               |
|------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| `readiness_chip.state`             | Compact UI state rendered on tabs, breadcrumbs, status items, activity rows, and CLI badges.                          |
| `readiness_chip.detail_target`     | Command ID or deep link the chip opens; resolves through the command plane (see ADR 0006 linked lanes).               |
| `not_ready_explainer.title`        | Short title (< 64 graphemes) that leads the not-ready detail surface.                                                 |
| `not_ready_explainer.body`         | Longer explanation (≤ 1024 graphemes) quoted verbatim by the support bundle and by assistive-tech announcements.      |
| `not_ready_explainer.what_still_works` | Narrowed capability list the user can still rely on; aligned with the PRD local-core continuity rules.             |
| `safe_next_action.command_id`      | Canonical command id the affordance invokes. Must resolve in the command plane.                                       |
| `safe_next_action.consequence`     | One-sentence description of what the action will do; support bundle quotes verbatim.                                  |
| `support_export.packet_family`     | Named packet family (`filesystem_identity`, `semantic_readiness`, `mutation_journal_entry`, `replay_capture`, …).      |
| `support_export.redaction_policy`  | Redaction policy id applied to the exported record; never overridden per-surface.                                     |
| `support_export.parity_signature`  | Content digest that lets the support surface and the in-product surface prove they are showing the same record.       |

The projection fields are not a state machine; they are slots. A
packet that needs more precision opens an additive-minor change
under the ADR-0006 schema-of-record posture.

## 4. Worked examples

These examples show the vocabulary in use for the four categories
the spec calls out. Each example references a companion fixture
under [`/fixtures/filesystem/`](../../fixtures/filesystem/).

### 4.1 Path-case change on a case-insensitive root

A user renames `Readme.md` to `README.md` on an APFS workspace
(case-insensitive, case-preserving). The canonical object does not
change — `strongest_identity_token` is still the same
`device_inode_generation` value — but the `presentation_path` and
one member of the `alias_set` do.

- `presentation_path.uri` updates from `…/Readme.md` to `…/README.md`.
- `canonical_filesystem_object` is unchanged.
- `alias_set` gains a `case_only_variant` alias for the prior spelling
  until the next VFS sweep confirms the old spelling is gone.
- Every surface that had the old presentation path open converges
  onto the new spelling through `vfs_alias_converge` (ADR 0006); no
  surface may treat the two spellings as separate mutable files.
- Semantic readiness: the object itself remains `exact` for any
  producer whose `input_digest_set` is keyed on
  `canonical_filesystem_object`. Path-keyed producers (none are
  permitted on the authoritative path) would have reported `stale`;
  ADR 0006 forbids that posture.

See [`/fixtures/filesystem/path_case_change.json`](../../fixtures/filesystem/path_case_change.json).

### 4.2 Symlink / junction alias

A user opens `docs/current-readme.md`, a symlink to
`/repo/README.md`. Two presentation paths, one canonical object.
The `alias_set` records the symlink and the canonical path; the VFS
converges both tabs onto one dirty-buffer authority. A save through
either presentation path targets the canonical object. A Windows
junction produces the analogous `alias_kind = junction` record
under `local_windows_like`.

Semantic readiness: the symlinked view and the canonical view share
the same readiness record; a producer that indexes the canonical
object does not recount the symlinked alias as separate coverage.

See [`/fixtures/filesystem/symlink_alias.json`](../../fixtures/filesystem/symlink_alias.json).

### 4.3 Moved file (`git mv` or VFS rename)

A user moves `src/lib/foo.rs` to `src/foo.rs`. The canonical object
is unchanged on POSIX (same inode); only the `canonical_uri` and
`presentation_path.uri` update. On Windows, the `windows_object_id`
is also stable across rename. `alias_set` does not accumulate a
stale alias — the VFS prunes the old path on rename commit
(`vfs_rename_commit`, ADR 0006).

Semantic readiness: producers keyed on the strongest identity token
remain `exact` after the rename. Producers keyed only on the logical
URI may briefly report `partial` with reason
`partial_enumeration` while they redrive the mapping; they resolve
back to `exact` on the next producer sweep.

See [`/fixtures/filesystem/moved_file.json`](../../fixtures/filesystem/moved_file.json).

### 4.4 Partially ready workspace

A user opens a freshly cloned monorepo. The VFS is attached and the
file tree is `exact`; the lexical search index is `partial` with
reason `producer_warming`; the semantic graph is `partial` with
reason `partial_enumeration` (only `payments` and `auth` roots
enumerated so far); a vendor-imported type index is `imported` with
reason `imported_source`; a policy-restricted infra root is
`unavailable` with reason `policy_blocked`.

The workspace chrome shows one composite readiness chip (`Partially
ready`) with a detail surface that lists each producer's state and
reason. Every label in that detail surface is drawn from §2.1–§2.3;
no producer invents a private dialect. The support bundle exports
the same record; the CLI's `doctor` equivalent prints the same
fields.

See [`/fixtures/filesystem/partially_ready_workspace.json`](../../fixtures/filesystem/partially_ready_workspace.json).

## 5. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about the objects defined in §1 or the states in §2.

1. **Preserve `presentation_path`.** Tabs, breadcrumbs, copy / paste,
   command-palette entries, CLI output, and recent-file lists show
   the path the user opened. The canonical URI is rendered
   *alongside* when disclosure is required; it does not replace the
   presentation path.
2. **Disclose alias relationships.** A surface that shows one
   presentation path for a canonical object with more than one
   known alias MUST surface at least the alias count (e.g. "also
   reachable via 2 other paths") and offer a keyboard-reachable
   detail affordance.
3. **Never dedupe by path string.** Tab-deduplication, dirty-state
   merging, and external-change detection key on
   `canonical_filesystem_object`, not on the presentation path.
4. **Readiness labels come from §2.1 only.** No surface ships a
   custom state like `Indexing…`, `Half ready`, or `Maybe stale`
   when one of the frozen states fits. `Partially ready` is the
   only composite surface label permitted, and only over records
   whose individual states come from §2.1.
5. **Quote `not_ready_reason` verbatim.** A chip may render the
   reason's short label; the detail surface MUST render the code
   so the user, the CLI, and the support bundle can compare them.
6. **Support parity.** Every readiness record exports through the
   support-bundle / replay / mutation-journal families with the
   same fields it shows in chrome (§2.4). A surface that needs to
   hide a field does so through a redaction policy, not by omitting
   the field.
7. **Safe-next-action routing.** A not-ready surface MUST expose at
   least one `safe_next_action` from §2.3. `no_action_required` is
   a valid action; silence is not.
8. **One canonical explainer per record.** The product, the CLI,
   the support bundle, and the exported replay capture share the
   same explainer text; assistive-tech announces that text. Local
   paraphrases are forbidden.

## 6. Machine-readable schema seed

The machine-readable schema lives at
[`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json).
It imports the ADR-0006 `vfs_save_envelope.schema.json` identity
records by reference, adds the `semantic_readiness` vocabulary
(§2.1–§2.3) as a first-class record, and reserves the projection
fields (§3).

The schema is the *export* boundary; Rust types in the VFS crate
(`crates/aureline-vfs`) are the schema of record, mirroring
ADR 0006's posture. A breaking change bumps
`filesystem_identity_schema_version`; additive-optional changes do
not. There is no external IDL or code-generator toolchain at this
milestone.

## 7. Using this vocabulary

Producers, consumers, and exporters all read this document and the
companion schema. No lane fork is permitted.

| Lane                                  | Reads vocabulary for                                                                                       |
|---------------------------------------|------------------------------------------------------------------------------------------------------------|
| VFS prototype (`crates/aureline-vfs`) | Identity records, save-target token issuance, alias convergence, semantic-readiness frame construction.    |
| Buffer / editor                       | Canonical-object dedupe, dirty-buffer convergence, save-target pinning, readiness chip on the active tab.  |
| Graph / search / docs pack            | Producer attribution, `input_digest_set` derivation, `not_ready_reason` selection, partial / stale labels. |
| AI apply / review                     | Save-target pinning through the participant chain; readiness gating on apply preview and on result review. |
| CLI / headless flows                  | `presentation_path` preservation, readiness states rendered verbatim, `safe_next_action` routing.          |
| Support export / mutation journal / replay | Packet family, redaction policy, parity signature; identity-layer and readiness-record fields quoted verbatim. |
| Product / UX docs                     | Readiness chip copy rules, not-ready explainer template, alias-disclosure rendering.                       |

## 8. Acceptance

- The identity and readiness terms in §1–§2 are reused by the VFS
  prototype, the supportability docs, and the M0 architecture
  pack. No lane invents its own.
- The machine-readable schema at
  [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
  validates the four worked-example fixtures under
  [`/fixtures/filesystem/`](../../fixtures/filesystem/).
- Product / UX docs describe the same terms by citation rather
  than by local redefinition.
- The projection fields in §3 are present (even if unfilled) on
  every later readiness packet so M0 conformance packets compose
  without retrofit.

## 9. Changing this vocabulary

- **Additive-minor** changes (new `not_ready_reason`, new
  `safe_next_action`, new projection slot, new alias kind, new
  readiness-producer lane) land here and in the companion schema in
  the same change. No ADR is required, but the change must cite
  the motivating fixture or packet.
- **Repurposing** an existing state, reason, or projection field
  is breaking. It opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The ADR wins on any disagreement with ADR 0006; this document
  and the schema are updated in the same change when that happens.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1108` — "5.17 Virtual file system and
  file watching".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1628` —
  "12.2.1 Real filesystem identity, canonical path, and save-
  coordination architecture".
- `.t2/docs/Aureline_Technical_Design_Document.md:1478` — "7.2.3
  Filesystem identity".
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:2013` — "use
  **Partially ready** when work can continue with reduced
  completeness".
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:17614` — "EA.2
  Index-readiness chip".
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md:1125` —
  lifecycle state copy rules (`Partially ready`, `Degraded`,
  `Read-only degraded`).
- `.t2/docs/Aureline_Milestones_Document.md:3498` — "canonical
  filesystem-identity, alias-set, save-target-token, and
  semantic-readiness vocabulary seed shared across product, CLI,
  and support".
- `.t2/docs/Aureline_Milestones_Document.md:5159` — "Workspace-
  truth, lineage, and semantic-readiness promotion gate".

## Linked artifacts

- ADR of record: [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- Boundary schema: [`schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json).
- Companion envelope schema (identity + save + cache): [`schemas/runtime/vfs_save_envelope.schema.json`](../../schemas/runtime/vfs_save_envelope.schema.json).
- Worked-example fixtures: [`fixtures/filesystem/`](../../fixtures/filesystem/).
- Reactive-truth envelope: [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- Root-capability matrix: [`artifacts/io/root_capability_matrix.yaml`](../../artifacts/io/root_capability_matrix.yaml).
- Affected lanes: `crates/aureline-vfs`, `crates/aureline-buffer`,
  `crates/aureline-rpc`, `crates/aureline-telemetry`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:support_export`.
