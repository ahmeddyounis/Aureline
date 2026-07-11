# M5 notebook document headers and kernel-state strips

The notebook document header and the kernel-state strip are two of the eight governed
notebook-kernel-output components frozen by the
[M5 notebook-kernel-output component matrix](m5_notebook_kernel_output_component_matrix.md). This
lane implements those two families as two co-equal control vectors in one export-safe packet,
[`NotebookDocumentHeaderKernelStateStripControlsPacket`](../../crates/aureline-notebook/src/implement_notebook_document_headers_and_kernel_state_strips_with_canonical_ipynb_source_selected_kernel_origin_busy_queued_offline_truth_and_no_kernel_edit_parity_across_claimed_m5_notebook_surfaces/mod.rs),
so a claimed M5 notebook, kernel-manager, debug, review, or CLI surface can project a document
header and a kernel-state strip that let a user orient to a notebook's canonical source and its
live kernel state **before they run, debug, review, or trust any output** — with document truth
and runtime truth kept distinct, and a kernel-free notebook kept explicitly editable rather than
blocked behind a setup-first wall.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_document_header`

Given a header's document source class and identity state, the resolver derives an **origin
class**:

- `local_ipynb` → `local_document` (settled canonical source)
- `remote_ipynb` → `remote_document` (settled canonical source)
- `managed_workspace_ipynb` → `managed_document` (settled canonical source)
- `imported_ipynb` → `imported_document` (must carry an explicit imported note), not canonical
- `scratch_untitled` → `scratch_document` (must carry an explicit scratch note), not canonical
- `unknown_source` → `unknown_document` (must carry an explicit unknown-source note), not canonical

Independently, a `conflicted`, `read_only`, `recovered`, or `unsaved_changes` identity always
carries its own note, so document state never hides behind a clean-looking header. A user can
therefore always tell whether they are looking at a **local, remote, managed, imported, scratch,
or unknown-source notebook**, and whether its identity is settled, before trusting it; an imported,
scratch, or unknown-source notebook can never read as a settled canonical source.

### `resolve_kernel_state`

Given a strip's kernel execution state and connection state, the resolver derives a **live class**:

- `dead_no_kernel` (or `idle_ready` while `never_connected`) → `no_kernel_editable` (must carry an
  explicit no-kernel note), not live, editing preserved
- `queued_pending` → `queued_live` (a live kernel)
- `busy_running` → `busy_live` (a live kernel)
- `idle_ready` over `connected_local` / `connected_remote` → `ready_live` (a live kernel)
- `interrupted` → `inspect_only` (must carry an explicit inspect-only note), not live
- `disconnected_reconnecting`, or `idle_ready` while `reconnecting` / `disconnected` /
  `connection_lost` → `disconnected_recoverable` (must carry an explicit reconnect note), not live

A user can therefore always tell whether the kernel is **ready, busy, queued, kernel-free,
disconnected-recoverable, or inspect-only** before trusting any output; a kernel-free,
disconnected, or interrupted notebook can never read as live, and a kernel-free notebook stays
explicitly editable, searchable, and reviewable.

## The two component vectors

Each **document header** names its canonical `.ipynb` identity, its source class and derived origin
class, its identity state, its paired export state, its current target / workspace context, and its
source-of-truth cue, and always offers keyboard-complete **open / export / review** actions so a
notebook stays editable, exportable, and reviewable even before any kernel is selected.

Each **kernel-state strip** names its selected kernel origin / class (so local, remote, container,
SSH, and managed kernels never collapse into one unlabeled badge), its execution and connection
state, its derived live class, its execution context, and its kernel-free edit parity, and always
offers keyboard-complete **select / inspect / continue-without-kernel** actions so no kernel-free
notebook is ever forced behind a setup-first blocker.

## Hard invariants

Every component keeps four hard invariants `false`, enforced by the Rust validator and mirrored as
`const false` in the schema:

- `pretends_kernel_free_is_live` — a kernel-free or disconnected notebook is never shown as live.
- `collapses_kernel_origins_into_one_badge` — local / remote / managed kernels never collapse into
  one unlabeled badge.
- `conflates_document_and_runtime_truth` — document truth and runtime truth stay distinct.
- `hides_state_behind_hover_only` — no governed state is hidden behind a hover-only affordance.

Every next step names one stable **notebook / kernel-manager / docs / support** deep link rather
than an ephemeral overlay, and raw notebook payloads, pasted paths, credentials, and private
endpoints never cross the export boundary.

## Coverage and acceptance criteria

The seeded packet carries six document headers covering every document source class, identity
state, and derived origin class, and six kernel-state strips covering every kernel execution state,
connection state, and derived live class. The validator asserts that coverage, that each derived
class equals the resolved class, and that each conditional note is present, so the packet proves the
acceptance criteria:

- Users can distinguish notebook document truth from runtime truth at a glance (distinct header vs
  strip field sets and the `document_and_runtime_truth_distinct` review flag).
- Kernel-free notebooks remain explicitly editable / searchable / reviewable without pretending to
  be live (`no_kernel_editable` live class, mandatory `continue_without_kernel` action, and the
  `kernel_free_edit_note`).
- Notebook headers and kernel-state strips remain consistent across edit, diff, debug, and support
  surfaces (shared frozen vocabulary and the `header_and_strip_consistent_across_surfaces` review
  flag).

## Truth source and artifacts

The Rust validator in `crates/aureline-notebook` is the authoritative gate. The headless emitter
`aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive` is the only
mint-from-truth path for:

- the checked support export and matrix CSV under
  `artifacts/release/m5-notebook-document-header-kernel-state-strip-proof/`,
- the Markdown design report at
  `artifacts/design/m5-notebook-document-header-kernel-state-strip.md`, and
- the narrowed scenario fixtures under
  `fixtures/ui/m5-notebook-document-header-kernel-state-strip-controls/`.

The boundary schema is
[`schemas/ui/m5-notebook-document-header-kernel-state-strip-controls.schema.json`](../../schemas/ui/m5-notebook-document-header-kernel-state-strip-controls.schema.json).
