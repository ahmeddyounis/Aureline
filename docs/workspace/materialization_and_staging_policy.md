# Workspace target materialization and staging policy

This document freezes the cross-surface vocabulary Aureline uses when
clone, import, archive-inspection, handoff, restore, and support/export
flows must stay honest about **where bytes live**, **whether the
location is durable**, and **whether a location is already the workspace
root** or only a temporary / staged materialization on the path to a
durable target.

Temporary and staged materialization is allowed. What is forbidden is
letting a temporary or staged location masquerade as durable workspace
state in UI chrome, support exports, recovery flows, or recents.

The machine-readable schema that exports this vocabulary and the
disclosure contract lives at:

- [`/schemas/workspace/materialization_class.schema.json`](../../schemas/workspace/materialization_class.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/staging_cases/`](../../fixtures/workspace/staging_cases/)

This contract composes with (and does not replace):

- [`/docs/workspace/entry_restore_object_model.md`](./entry_restore_object_model.md)
  for `entry_verb`, `resulting_mode`, `destination_disposition`, and the
  inspect-only vs. write-capable entry invariants.
- [`/docs/ux/clone_review_contract.md`](../ux/clone_review_contract.md) and
  [`/docs/ux/import_handoff_review_contract.md`](../ux/import_handoff_review_contract.md)
  for the UX-layer review sheets that project this policy into clone and
  import surfaces.
- [`/docs/workspace/bootstrap_packet_contract.md`](./bootstrap_packet_contract.md)
  for resumability and rollback vocabulary when staging or temporary
  materialization exists after an interrupted acquisition.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or design-system style guide, those documents win and this
document plus its schema and fixtures MUST be updated in the same
change.

## Why freeze this now

Project entry and handoff are protected journeys. If Aureline cannot
name where bytes live and whether that location is durable, then:

- a clone can silently land in a temp directory and later “become” a
  workspace, leaving users unable to find their files or reason about
  cleanup;
- an import can unpack into staging but later appear in recents as if it
  were already a durable workspace, breaking support/export and rollback
  expectations;
- support bundles can claim to snapshot a workspace while actually
  exporting only a staged extraction root (or worse: an OS temp path);
- recovery flows can offer “restore last session” against a target that
  was never committed to durable state.

Freezing the materialization class vocabulary and disclosure rules
before clone/import/archive flows harden keeps “inspect-only” and
“write-capable” paths visibly distinct and prevents hidden temp-dir
behavior on launch-bearing entry paths (Start Center, OS file open,
drag/drop, CLI/headless entry).

## Definitions

- **Target materialization** — The concrete byte-bearing location
  Aureline is currently using for a target: a durable workspace root, a
  user-chosen destination pending admission, labelled non-durable
  staging, a process-scoped temporary inspection root, a remote-backed
  root, a generated-only preview, or cached evidence without bytes.
- **Durable workspace state** — A location Aureline may treat as the
  workspace root for recents, recovery, support exports, and mutation
  flows. Durable workspace state is not “anything that exists on disk”;
  it is “a location the user has explicitly admitted as the workspace.”
- **Non-durable staging** — A labelled location on disk used to land
  bytes before the user admits a durable workspace root. Staging is
  disposable by default and MUST stay visibly labelled as non-durable.
- **Temporary inspection** — An inspection-only materialization that is
  process-scoped (in memory, tmpfs, or OS temp) and disappears on exit,
  cancel, or failure unless an explicit retention posture is chosen.
- **Promotion** — The explicit, reviewed step that moves or rebinds
  bytes from a non-durable or uncommitted location into a durable
  workspace root (or admits an already-chosen durable destination as the
  workspace root). Promotion MUST NOT be silent.
- **Collision** — Any conflict between the destination/promotion target
  and existing bytes or prior workspace state. Collisions MUST route to
  a typed collision review; they MUST NOT resolve by silently picking a
  different temp directory or silently overwriting bytes.

## Target materialization class (closed)

Surfaces that need to disclose “where bits live” use
`target_materialization_class` from the schema. The set is closed.

- `local_durable_workspace` — A local durable workspace root that the
  user has admitted.
- `local_destination_pending_admission` — Bytes are in a user-chosen
  destination, but the location is not yet admitted as a durable
  workspace root.
- `labelled_non_durable_staging` — Bytes exist on disk in labelled,
  non-durable staging.
- `temporary_inspection` — Bytes are in a process-scoped temporary
  inspection root; inspect-only posture.
- `staged_import_extraction` — Bytes exist on disk in labelled,
  non-durable extraction staging used for import/archive flows.
- `remote_backed_workspace` — The canonical root is remote (SSH,
  container, managed). Local caches are disposable and must be disclosed
  as such.
- `generated_only` — Content is generated or synthesized but not yet
  written into a durable target (preview-only until admitted).
- `cached_evidence_only` — Only evidence/metadata is present (digests,
  manifests, summaries). No byte-bearing root is available.
- `materialization_failed_partial` — A materialization attempt failed
  leaving partial bytes in a non-durable or uncommitted location; cleanup
  and recovery routes must be explicit.

## Disclosure rules (temp-location and staging contract)

### 1) Inspect-only vs. write-capable is never ambiguous

1. If an entry path is inspect-only, it MUST remain inspect-only even if
   Aureline uses temporary inspection or staging under the hood.
2. If bytes are being written (even to non-durable staging), the surface
   MUST disclose that write posture explicitly before commit.

A surface that collapses inspect-only and write-capable paths into one
generic “opening workspace” status is non-conforming.

### 2) Temporary and staged locations are always labelled

When `target_materialization_class` is
`labelled_non_durable_staging`, `staged_import_extraction`,
`local_destination_pending_admission`, or `materialization_failed_partial`:

- the non-durable / uncommitted label MUST be visible (not hidden behind
  generic progress copy);
- the location MUST be disclosed via a redaction-safe label (never a raw
  absolute path by default);
- the user MUST have a typed next step that either promotes/adopts the
  location into a durable workspace, or discards/rolls it back.

### 3) Durable-target promotion is a separate reviewed step

Promotion from staging/temporary/uncommitted to a durable workspace root:

- MUST be explicit (a distinct reviewed step);
- MUST name what is being promoted/admitted and what will become durable;
- MUST NOT be implied by successful extraction/clone completion.

### 4) Cleanup posture is explicit and preserved across surfaces

Every temporary/staged materialization carries a cleanup posture:

- `cleanup_on_cancel` / `cleanup_on_failure` means staged bytes are
  removed on cancel/failure.
- `retain_for_review` means staged bytes remain available for inspection
  until the user discards them.
- `retain_until_durable_promotion` means staged bytes persist until the
  user promotes/adopts them, after which cleanup is expected.

If staging is retained, it MUST remain labelled non-durable in recents,
support exports, and recovery flows until promotion/admission.

### 5) Collisions never resolve by hidden temp-dir behavior

If promotion/admission would collide with existing bytes or existing
workspace state, Aureline MUST route to an explicit collision review. A
collision resolver that silently picks a different temporary directory,
silently overwrites bytes, or silently reuses an existing target is
non-conforming.

## Where this shows up (UI, export, recovery invariants)

The materialization class and disclosure rules above constrain multiple
truth surfaces:

- **UI chrome / recents** MUST render staging/temporary/uncommitted
  states as visibly distinct from durable workspaces.
- **Support exports** MUST NOT label temporary/staged/uncommitted roots
  as “workspace state”; exports may reference them or include redacted
  evidence, but they must preserve the staging label and posture.
- **Recovery flows** MUST NOT offer “restore last session” or durable
  rollback actions for targets that were never admitted as a durable
  workspace root; staged/temporary targets offer inspect-only or discard
  routes instead.

## Worked examples

See the fixtures under
[`/fixtures/workspace/staging_cases/`](../../fixtures/workspace/staging_cases/)
for concrete records that:

- distinguish temporary inspection from labelled staging;
- keep non-durable staging distinct from a pending user destination;
- show explicit promotion/admission and cleanup postures;
- model cached-evidence-only and generated-only disclosure without
  pretending bytes are present.

