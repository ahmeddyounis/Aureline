# Path-truth chip, alias-inspector view, and save-target review packet

This packet freezes the cross-surface object model that every open,
save, rename-preview, editor-tab, breadcrumb, CLI save row, support
export, and alias-disclosure surface uses when it has to make three
questions answerable on difficult filesystem fixtures instead of
assumed:

1. **Which path did the user open?**
2. **Which canonical filesystem object will the next write land on?**
3. **Which object is the save / review action actually targeting,
   and does that target differ from what the user selected?**

The product has to be honest about those three questions on the hard
cases — case-only rename on an insensitive-preserving root,
symlink target shifts mid-session, overlay or managed-provider
roots, archive mounts, bind-mount aliases, whole-file rewrite
fallback — or its explanations collapse into screenshots. This
packet lands the object model, the chip contract, and the review
fixtures before the full editor, explorer, and save-target-review
surfaces are implemented, so later work does not have to reconcile
a tab tooltip that says one thing, a save dialog that says another,
and a support bundle that says a third.

The ADR of record for the underlying filesystem-identity, watcher,
save-pipeline, root-capability, and cache-identity rules is
[`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
The cross-surface identity and semantic-readiness vocabulary is
frozen in
[`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md).
The machine-readable identity / readiness boundary is
[`schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json).
This packet is additive over those sources and reuses their frozen
tokens verbatim; it does not mint parallel identity, alias-kind,
readiness, or safe-next-action vocabularies. Where this packet
disagrees with ADR 0006 or the identity vocabulary, those sources
win and this packet is updated in the same change.

Companion artifacts:

- [`/artifacts/fs/path_truth_examples/`](../../artifacts/fs/path_truth_examples/)
  — one seed `path_truth_chip_record` per difficult fixture.
- [`/artifacts/fs/alias_inspector_examples/`](../../artifacts/fs/alias_inspector_examples/)
  — one seed `alias_inspector_view_record` per difficult fixture,
  keyed to the same case ids the chip examples quote.
- [`/fixtures/fs/difficult_save_review_cases/`](../../fixtures/fs/difficult_save_review_cases/)
  — reviewer-facing fixtures for the hard save-target-review cases
  (case-only rename with collision, symlink target shift, managed
  overlay, whole-file rewrite fallback, archive inner alias,
  bind-mount canonical drift).
- [`/fixtures/fs/identity_corpus_manifest.yaml`](../../fixtures/fs/identity_corpus_manifest.yaml)
  — cross-corpus join index; the difficult-save fixtures register
  their case ids there alongside the existing save-truth and
  alias/symlink corpora.

## Why freeze this now

The save-truth corpus in
[`/fixtures/fs/save_truth_cases/`](../../fixtures/fs/save_truth_cases/)
and the alias / symlink corpus in
[`/fixtures/fs/alias_and_symlink_cases/`](../../fixtures/fs/alias_and_symlink_cases/)
describe what the save pipeline does on one canonical happy or
failing path per scenario. That is enough for the VFS prototype's
byte-stability tests, but it is not enough for the surfaces that
have to *explain* the result.

The hardest filesystem fixtures fail on explanation, not on bytes.
A case-only rename that silently became a no-op; a symlink that
repointed after the user opened the tab; a managed overlay where
the save dialog says "Save" but the write is actually a request
that will be reviewed by the provider; an archive mount where the
affordance looks writable because the tree looks familiar; a
bind-mount alias that makes the canonical object drift into a
foreign root — in each case, the save-truth fixture says "did the
bytes land" while the user wants to know "did my bytes land on the
file I thought I was editing".

Freezing the chip, the inspector view, and the difficult-case
fixtures here means:

- every tab, breadcrumb, save dialog, rename preview, and log line
  quotes one `path_truth_chip_record` against the same alias set
  and the same degraded-save hints;
- support handoff, AI context inspectors, and future editor or
  search surfaces reuse the same fields and never collapse a
  canonical shift into a generic "path changed";
- the difficult fixtures can be reviewed — in YAML — without
  running a UI, and the review statement preserves the distinction
  between the path opened, the object resolved, and the target
  written.

## Scope

This packet freezes three record kinds inside one packet:

1. `path_truth_chip_record` — the compact chip projection every
   file-facing surface renders. Quotes the ADR-0006 identity
   layers by name, references alias cues and degraded-save hints,
   and marks the row as `exact`, `imported`, `heuristic`, or
   `stale` with respect to filesystem identity.
2. `alias_inspector_view_record` — a reviewer-facing data-view
   projection that expands the alias set, shows the resolution
   chain for each member, and names exactly why a save or open
   action may hit a different object than the user originally
   selected.
3. `save_target_review_case` — a fixture shape for the hard cases
   the chip and the inspector view must stay honest across.
   Lives under `/fixtures/fs/difficult_save_review_cases/`.

Out of scope:

- A new alias-kind, readiness-state, or safe-next-action vocabulary.
  Those are frozen in the identity vocabulary and in ADR 0006; this
  packet only projects them.
- The final explorer / tab / breadcrumb UI chrome. This packet is
  the object model and the corpus; the rendered strings live with
  the shell interaction-safety contract.
- The VFS prototype harness. The difficult-case fixtures are
  reviewer-facing and YAML-first; they register in the identity
  corpus manifest and may later be lifted into the harness.
- Exhaustive filesystem coverage. The fixture corpus covers one
  representative case per difficult category; adding more cases
  within a category is an additive-minor change.

## 1. Path-truth chip record

The chip is the compact projection every file-facing surface
renders next to the presentation path. A surface that needs to
explain "which file is this" at a glance reads exactly one
`path_truth_chip_record` and renders its fields verbatim.

### 1.1 Identity

Every chip carries:

- `chip_id` — stable opaque id scoped to the surface instance.
- `surface_class` — one value from the closed set in §1.2.
- `subject` — the filesystem-identity record the chip is about
  (see `filesystem_identity_record` in the identity schema).
  References `workspace_id`, `root_id`, and `logical_uri` by
  opaque handle; raw absolute paths are never inlined through
  this handle.
- `presentation_path` — ADR-0006 `presentation_path` (layer 1),
  unchanged.
- `canonical_filesystem_object` — ADR-0006
  `canonical_filesystem_object` (layer 3), unchanged.
- `corpus_case_ref` — optional id of the difficult-case fixture
  the chip illustrates (e.g., `corpus.fs.difficult.case_only_rename_with_collision`).
  Chips emitted against live files carry `null` here.

Rules (frozen):

1. A chip MUST NOT invent a new `alias_kind`, `root_class`,
   `atomic_write_mode`, `semantic_readiness_state`, or
   `safe_next_action` value. The chip quotes the identity
   vocabulary; a surface that needs a new token opens an
   additive-minor change against the vocabulary, not against this
   packet.
2. A chip's `presentation_path.uri` is never silently rewritten to
   `canonical_filesystem_object.canonical_uri`. When the two
   differ, the chip surfaces `canonical_differs_from_presentation
   = true` and the inspector view shows the resolution chain.
3. `corpus_case_ref` is the only field a reviewer uses to join the
   chip artifact to the fixture. Private surface-local case ids
   are forbidden.

### 1.2 Surface class

The surface class is closed; new surfaces are added by decision
row. Every listed surface renders the chip contract. A surface in
this family that mints its own path-truth string is non-conforming.

- `editor_tab` — tab tooltip and overflow chip.
- `breadcrumb` — breadcrumb segment chip.
- `save_dialog_header` — header chip on a save / save-as dialog.
- `rename_preview_card` — preview card rendered before a rename
  commit.
- `save_target_review_sheet` — reviewer sheet rendered before a
  save lands on a difficult fixture.
- `alias_inspector_header` — header chip on the alias-inspector
  detail view.
- `support_packet_header` — path-truth header on a support export.
- `ai_context_inspector_row` — one row per file the AI context
  inspector references.
- `cli_save_row` — one row per save / rename in the CLI output.
- `log_line` — path-truth entry on a mutation log line.

Rules (frozen):

1. The chip is the boundary every surface in this family shares.
   Adding a new surface class is additive-minor; removing one
   already in this list is breaking.
2. A chip on a `save_target_review_sheet` MUST carry a non-empty
   `save_target_review` block (§1.5). On other surfaces the
   block is optional.

### 1.3 Alias cues

Alias cues summarise the alias / overlay / case-fold posture
around the canonical object so a glance at the chip conveys what
is different about this path without scrolling into the inspector
view.

- `alias_kinds` — array; values from the frozen `alias_kind` set
  (`symlink`, `junction`, `hardlink_sibling`, `case_only_variant`,
  `unicode_normalization_variant`, `remote_alias`,
  `bind_mount_alias`, `container_mount_alias`,
  `archive_inner_alias`). Multiple values are allowed.
- `overlay_kind` — one of `none`, `managed_provider_overlay`,
  `container_overlay`, `bind_mount_overlay`, `archive_overlay`.
  Overlays describe the root posture, not the alias relationship.
- `case_fold_class` — one of `sensitive`, `insensitive_preserving`,
  `insensitive_non_preserving`. Pass-through of ADR-0006
  `capability_flags.case_sensitivity`.
- `normalization_form` — one of `none`, `nfc`, `nfd`,
  `mixed_observed`. Pass-through of the canonical object's form.
- `alias_count` — integer count of alias-set members. Support
  surfaces render "also reachable via N other paths" from this
  count.
- `canonical_differs_from_presentation` — boolean. True whenever
  `presentation_path.uri` resolves to a different URI than
  `canonical_filesystem_object.canonical_uri` after alias
  resolution.

Rules (frozen):

1. `alias_count` is never omitted. `0` is a legitimate value.
2. A chip with `alias_count >= 1` MUST expose at least the
   `open_alias_details` action in `offered_actions`.
3. `overlay_kind` `none` is legitimate. A root is not
   retroactively an overlay because the user is uncertain about
   the root type; the value reflects the root adapter's posture.

### 1.4 Degraded-save hints

The chip carries the compact degraded-save hint set so a surface
can render "Save is degraded" honestly without querying the save
pipeline. Full detail rides on the alias inspector view and on the
save-target-review fixture.

- `identity_assertion_state` — one of `exact`, `degraded`,
  `unsupported`. Pass-through of the corpus manifest's
  `identity_assertion_state_vocabulary`.
- `save_assertion_state` — one of `exact`, `degraded`,
  `unsupported`. Pass-through of the corpus manifest's
  `save_assertion_state_vocabulary`.
- `watcher_assertion_state` — one of `exact`, `degraded`,
  `unsupported`. Pass-through of the corpus manifest's
  `watcher_assertion_state_vocabulary`.
- `atomic_write_mode` — one of `atomic_replace`,
  `in_place_write`, `conditional_remote_write`,
  `whole_file_rewrite_fallback`, `blocked`. Extends the
  ADR-0006 save-mode set with the reviewer-facing
  `whole_file_rewrite_fallback` label so a chip can honestly
  surface when the adapter has degraded to full-content rewrite;
  the underlying save pipeline still records the actual
  primitive it used.
- `save_hint_codes` — ordered array of short, frozen codes. The
  seed set is:
  - `presentation_differs_from_canonical`
  - `alias_set_non_empty`
  - `case_only_rename_requires_preview`
  - `normalization_only_rename_requires_preview`
  - `symlink_target_shifted_mid_session`
  - `overlay_rewrite_only`
  - `managed_review_required`
  - `archive_inspect_only`
  - `bind_mount_canonical_drift`
  - `watcher_degraded`
  - `conditional_remote_write_required`
  - `write_affects_multiple_names`
  - `save_blocked_by_policy`

Rules (frozen):

1. A chip MUST render every non-`exact` assertion state. A chip
   that collapses `degraded` or `unsupported` into a generic
   "save failed" label is non-conforming.
2. `atomic_write_mode = blocked` MUST carry at least one
   `save_hint_code` that explains the block
   (`archive_inspect_only`, `managed_review_required`,
   `save_blocked_by_policy`, …).
3. `save_hint_codes` is an ordered array; the first code is the
   primary hint the surface renders compactly, the remaining codes
   ride on the alias-inspector view.
4. Adding a new `save_hint_code` is additive-minor. Repurposing
   one is breaking.

### 1.5 Row-truth state

Every chip carries exactly one `row_truth_state` that names how
trustworthy the chip's claim is with respect to filesystem
identity — independent of whether the bytes on disk changed. The
state set is a closed four-value subset of the identity
vocabulary's semantic-readiness states:

| State       | Meaning                                                                                                                                    |
|-------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| `exact`     | The canonical target, alias set, and capability flags on this chip were resolved from the root adapter for the exact object the chip names. |
| `imported`  | Parts of the chip were imported from an external source (replay, support-bundle excerpt, vendored probe). Never authoritative.              |
| `heuristic` | The chip fell back to a heuristic path (no adapter available, offline probe, inferred canonical path). Correctness is best-effort.          |
| `stale`     | The chip's claims were authoritative against inputs that have since changed (generation token rolled, symlink target shifted, overlay remapped); the chip is not yet refreshed. |

`row_truth_state` is the only row-truth vocabulary the chip
surfaces. Chips MUST NOT render `partial`, `unavailable`, or
`out_of_scope` against this field; those semantic-readiness values
belong on a separate producer frame (the graph, the search index,
the docs pack), not on the filesystem-identity row itself.

Rules (frozen):

1. A chip with `row_truth_state = stale` MUST cite one
   `row_truth_reason` from the closed set in §1.6.
2. A chip with `row_truth_state = imported` or `heuristic` MUST
   cite the appropriate `row_truth_reason` and MUST NOT ride a
   `save_assertion_state = exact` claim in the same record.
3. A chip with `row_truth_state = exact` MAY still carry
   `save_assertion_state = degraded` or `unsupported` — the root
   is honestly degraded even when the chip itself is authoritative.

### 1.6 Row-truth reason

Closed set, pass-through of the filesystem-identity-facing subset
of the identity vocabulary's `not_ready_reason`:

- `canonical_resolved_from_root` — default for `exact`; the
  adapter resolved the canonical target directly.
- `canonical_inherited_from_import` — for `imported`; the chip
  was built from an imported record.
- `canonical_inferred_heuristic` — for `heuristic`; the chip was
  inferred without a live adapter.
- `canonical_token_stale` — for `stale`; the compare-before-write
  generation token has been bumped.
- `canonical_target_shifted` — for `stale`; the symlink / alias
  target was repointed mid-session.
- `canonical_overlay_remapped` — for `stale`; a managed or
  container overlay remapped the path to a different object.
- `canonical_partial_enumeration` — for `heuristic`; the adapter
  has enumerated some but not all aliases.

Rules (frozen):

1. Exactly one `row_truth_reason` rides on every non-`exact` chip.
2. A chip with `row_truth_state = exact` MAY carry
   `canonical_resolved_from_root` or omit the reason.
3. Adding a reason code is additive-minor; repurposing one is
   breaking.

### 1.7 Save-target review

Optional on most surfaces, required on the
`save_target_review_sheet` and on any chip with
`atomic_write_mode = blocked`. Quotes the same shape the corpus
fixtures carry:

- `required` — boolean.
- `reason_codes` — ordered list of `save_hint_codes` that drive
  the review.
- `primary_action` — one of the closed action set in §1.8.
- `secondary_actions` — ordered list of additional actions.

### 1.8 Offered actions

The closed action set is the filesystem-identity-facing subset of
the identity vocabulary's `safe_next_action`:

- `open_alias_details`
- `open_save_target_review`
- `open_rename_preview`
- `save_with_compare_before_write`
- `save_as`
- `cancel`
- `review_and_approve`
- `copy_canonical_uri`
- `copy_workset_id`
- `open_support_bundle`
- `open_review_diff`
- `retry_now`
- `no_action_required`

Rules (frozen):

1. Every chip carries at least one action. A chip with no actions
   is denied with `chip_actions_missing`.
2. `review_and_approve` is required on any chip where
   `save_target_review.required = true` and the review is
   reviewer-blocking.
3. `open_alias_details` is required on any chip where
   `alias_count >= 1`.

## 2. Alias-inspector view record

The alias inspector is the keyboard-reachable detail surface the
chip opens through `open_alias_details`. It is not a screenshot
and not a free-text paragraph; it is a structured data view every
support, product, and CLI surface renders from the same record.

### 2.1 Shape

- `record_kind = alias_inspector_view_record`.
- `schema_version = 1`.
- `view_id` — stable opaque id.
- `subject_chip_ref` — `chip_id` the view was opened from.
- `subject` — same identity subject the chip references.
- `presentation_path` — verbatim from the chip.
- `canonical_filesystem_object` — verbatim from the chip.
- `alias_set_members` — array; one entry per alias-set member.
- `overlay_context` — optional block; present when
  `overlay_kind != none`.
- `divergence_summary` — block describing whether save and open
  may diverge and why.
- `rows` — ordered inspector rows projected for review.
- `row_truth_state` — pass-through of the chip state.
- `corpus_case_ref` — same join handle as the chip.

### 2.2 Alias-set member row

Each entry carries:

- `alias_uri` — opaque URI string.
- `alias_kind` — frozen alias-kind value.
- `resolution_chain` — ordered array of strings; pass-through of
  the identity vocabulary's `resolution_chain`.
- `shares_dirty_buffer_authority` — boolean. True when a write
  through this alias modifies the same dirty buffer as a write
  through the canonical path.
- `divergence_risk` — one of `none`, `case_fold`,
  `normalization_only`, `overlay_remap`, `target_shift`,
  `archive_inner`, `bind_mount`, `hardlink_sibling`. Names why a
  save through this alias may target a different object than the
  user thought they selected.
- `review_required_before_save` — boolean; pass-through of the
  save-target-review rule for this alias.

### 2.3 Overlay context

Optional; required when `overlay_kind != none`. Carries:

- `overlay_kind` — same value as on the chip.
- `provider_class` — one of `managed_provider`,
  `container_runtime`, `bind_mount_host`, `archive_reader`,
  `none`. Opaque provider reference is carried separately.
- `provider_ref` — opaque provider id; raw provider URLs are not
  inlined.
- `writeback_policy` — one of `direct_write`,
  `whole_file_rewrite_fallback`, `review_then_write`,
  `inspect_only`. Names what the overlay will actually do with a
  write attempt.
- `visible_layer_readonly` — boolean; true for archive and
  some container overlays.
- `notes` — short redaction-aware free text.

### 2.4 Divergence summary

- `save_and_open_may_diverge` — boolean. True when the open action
  and the save action may resolve to different canonical objects
  (symlink shifted, overlay remapped, archive inner alias, …).
- `divergence_reasons` — ordered list of `save_hint_codes`
  describing why.
- `what_user_selected` — short label for the object the user
  selected at open time (presentation path).
- `what_save_targets` — short label for the object the save
  action will actually target.
- `what_review_shows` — short label for the object a save-target
  review sheet would render.

Rules (frozen):

1. `save_and_open_may_diverge = false` requires
   `divergence_reasons` to be empty.
2. `save_and_open_may_diverge = true` requires at least one
   `divergence_reason`.
3. Inspector-view rows MAY NOT omit `divergence_summary`; the
   block is the reviewer's primary read.

### 2.5 Inspector rows

An ordered array of review rows over the identity layers, alias
cues, and degraded-save hints. Every row carries:

- `field` — one of a closed set: `presentation_path`,
  `canonical_uri`, `normalization_form`, `identity_token_kind`,
  `identity_token_value`, `alias_set_summary`, `overlay_kind`,
  `case_fold_class`, `atomic_write_mode`, `identity_assertion_state`,
  `save_assertion_state`, `watcher_assertion_state`,
  `row_truth_state`, `save_target_review_required`.
- `value_summary` — short string rendered verbatim.
- `authority` — one of `root_adapter`, `import_record`,
  `heuristic_fallback`, `overlay_provider`, `watcher`,
  `save_pipeline`.
- `divergence_note` — optional short string; present when the
  field contributes to the divergence summary.

Rules (frozen):

1. Adding a new `field` value is additive-minor; removing one is
   breaking.
2. A row with `authority = heuristic_fallback` MUST be consistent
   with `row_truth_state = heuristic` on the chip.
3. Raw credential material, raw provider URLs, raw policy bodies,
   and raw user paths outside the workspace root do not cross
   this boundary. Class labels, counts, opaque handles, and
   workspace-relative URIs do.

## 3. Save-target-review case fixtures

The difficult-case fixtures under
[`/fixtures/fs/difficult_save_review_cases/`](../../fixtures/fs/difficult_save_review_cases/)
are the reviewable inputs the chip and the inspector view have to
stay honest across. Every fixture is one YAML file describing one
case, joined to the rest of the corpus through the identity
manifest.

### 3.1 Shape

Every fixture declares:

- `schema_version: 1`.
- `case_id` — stable id under the `corpus.fs.difficult.*`
  namespace. Registered in `identity_corpus_manifest.yaml`.
- `case_family: save_target_review_case`.
- `title` — short reviewer-facing label.
- `summary` — 2–4 sentence narrative of what the surface has to
  explain on this case.
- `difficulty_axis` — one of `case_only_rename`,
  `symlink_target_shift`, `overlay_managed_root`,
  `whole_file_rewrite_fallback`, `archive_inner_alias`,
  `bind_mount_canonical_drift`.
- `root_profile` — pass-through of the root capability class.
- `presentation_path`, `canonical_filesystem_object`,
  `alias_set` — same layer-1/3/4 identity record the chip emits.
- `row_truth_state` / `row_truth_reason` — as declared on the
  chip.
- `degraded_save_hints` — chip-side hint block.
- `save_target_review` — required block, as on the chip.
- `expected_chip_artifact` — path to the seed chip example.
- `expected_inspector_artifact` — path to the seed alias
  inspector example.
- `client_can_still_assert` — non-empty list of honest claims.
- `client_must_not_assert` — non-empty list of claims the
  surface must refuse to make.
- `related_fixture_ids` — cross-corpus join ids.

### 3.2 Difficulty axes

The seed corpus covers six difficulty axes. Adding a new axis is
additive-minor (it extends the fixture-validator enum and the
corpus manifest); removing an axis is breaking.

| Axis                             | What the surface has to explain                                                                            |
|----------------------------------|------------------------------------------------------------------------------------------------------------|
| `case_only_rename`               | A case-only rename that may silently become a no-op on an insensitive-non-preserving root, or that needs a preview on an insensitive-preserving root with a case-fold collision. |
| `symlink_target_shift`           | The symlink target moved between open and save; the open action and the save action resolve to different canonical objects. |
| `overlay_managed_root`           | A managed provider overlay where save is a request, not a direct write; the user sees "Save" but a review gate fires. |
| `whole_file_rewrite_fallback`    | The adapter cannot atomic-replace; the save degrades to whole-file rewrite; the chip must disclose the fallback and the watcher implications. |
| `archive_inner_alias`            | An archive mount where the tree looks editable; the chip must render `inspect_only` and disclose the `archive_inner_alias`. |
| `bind_mount_canonical_drift`     | A bind-mount alias whose canonical target now lives in a foreign root; save would cross a root boundary unexpectedly. |

### 3.3 Rules

1. Every fixture registers in `identity_corpus_manifest.yaml`
   under a new `difficult_save_review_cases` section. The join
   fields (`case_id`, `primary_fixture`, assertion-state rows,
   related fixture ids) follow the manifest's existing pattern.
2. Each fixture has exactly one matching chip artifact under
   `/artifacts/fs/path_truth_examples/` and exactly one matching
   inspector-view artifact under
   `/artifacts/fs/alias_inspector_examples/`. The three files
   share a `corpus_case_id`.
3. Raw paths used in fixtures are illustrative (`file:///ws/...`,
   `agent://...`, `overlay://...`); no real user paths appear.
4. A fixture MUST carry non-empty `client_can_still_assert` and
   `client_must_not_assert` lists. The two lists are the
   reviewer's honesty guardrails.

## 4. How each surface uses the packet

### 4.1 Editor tab, breadcrumb, log line

- `editor_tab`, `breadcrumb`, and `log_line` surfaces read a chip
  and render `presentation_path.display_label`, the first
  `save_hint_code`, and the `alias_count`. They open the alias
  inspector on `open_alias_details`.
- A tab that would otherwise de-duplicate by path string MUST
  dedupe by `canonical_filesystem_object` (ADR-0006 rule). The
  chip is the source of truth for that comparison.
- A log line that cites a save event attaches the chip's
  `atomic_write_mode` and `save_hint_codes` verbatim.

### 4.2 Save dialog, rename preview, save-target-review sheet

- `save_dialog_header` renders the chip with the full
  `degraded_save_hints` block visible.
- `rename_preview_card` renders the chip against both the
  pre-rename and post-rename canonical targets.
- `save_target_review_sheet` renders the chip plus the alias
  inspector plus the `save_target_review` block. The sheet is
  the surface where the user decides whether to proceed on a
  difficult fixture.

### 4.3 Alias inspector

- `alias_inspector_header` renders the chip at the top of the
  inspector view.
- The inspector view is the canonical data-view projection. Every
  row in the view is drawn from the frozen `field` set (§2.5).

### 4.4 Support export, AI context inspector, CLI

- `support_packet_header` quotes the chip on the path-truth
  section of a support bundle. Triage replays the same alias
  set and degraded-save hints the reporter saw; no new identity
  is minted.
- `ai_context_inspector_row` attaches a chip per file the AI
  turn references. When the chip's `row_truth_state` is not
  `exact`, the inspector MUST render the reason.
- `cli_save_row` emits the chip fields in the CLI's structured
  save output (JSON / YAML). The human-facing CLI renders the
  same fields in a compact one-line form.

## 5. Cross-corpus join

Every difficult-case fixture, chip artifact, and inspector-view
artifact joins through `corpus_case_id`. The identity corpus
manifest is the authoritative index. The seed set adds a new
section:

```yaml
difficult_save_review_cases:
  - case_id: corpus.fs.difficult.case_only_rename_with_collision
    record_family: save_target_review_case
    primary_fixture: fixtures/fs/difficult_save_review_cases/case_only_rename_collision.yaml
    chip_artifact: artifacts/fs/path_truth_examples/case_only_rename_chip.json
    inspector_artifact: artifacts/fs/alias_inspector_examples/case_only_rename_alias_view.json
    identity_assertion_state: exact
    save_assertion_state: degraded
    watcher_assertion_state: exact
    related_fixture_ids:
      - corpus.fs.identity.case_only_difference
      - corpus.fs.alias.case_fold_collision_insensitive_root
    topics:
      - case_only_rename
      - case_fold_collision
      - save_target_review
```

Six seed rows cover the six difficulty axes (§3.2). Adding a row
is additive-minor; removing one is breaking.

## 6. Change management

- Adding a new `surface_class`, `overlay_kind`, `atomic_write_mode`
  fallback label, `save_hint_code`, `row_truth_reason`, inspector
  `field`, inspector `authority`, alias `divergence_risk`, or
  `difficulty_axis` is additive-minor and bumps the packet's seed
  revision.
- Repurposing an existing enum value is breaking and requires a
  new decision row.
- The chip's `alias_kind`, `semantic_readiness_state` subset, and
  `safe_next_action` subset are pass-through of ADR 0006 and the
  identity vocabulary. Any change to those vocabularies lands
  there first, not here.
- Removing a `surface_class` already in the seed set is breaking;
  surfaces in the seed family may not be silently unhooked.

## 7. References

- ADR 0006 — `docs/adr/0006-vfs-save-cache-identity.md`.
- Identity vocabulary — `docs/filesystem/filesystem_identity_vocabulary.md`.
- Identity schema — `schemas/filesystem/save_target_token.schema.json`.
- Save-truth corpus — `fixtures/fs/save_truth_cases/`.
- Alias / symlink corpus — `fixtures/fs/alias_and_symlink_cases/`.
- Corpus manifest — `fixtures/fs/identity_corpus_manifest.yaml`.
- Case-only rename matrix — `fixtures/fs/case_only_rename_matrix.yaml`.
- Save-coordination projection examples —
  `artifacts/fs/save_coordination_examples/`.
- Scope-truth chip packet (surface-parity precedent) —
  `docs/workspace/scope_truth_packet.md`.
