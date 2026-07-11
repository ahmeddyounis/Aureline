# M5 kernel picker rows and kernel origin pills

The kernel picker row and the kernel origin pill are two of the eight governed
notebook-kernel-output components frozen by the
[M5 notebook-kernel-output component matrix](m5_notebook_kernel_output_component_matrix.md). This
lane implements those two families as two co-equal control vectors in one export-safe packet,
[`KernelPickerRowKernelOriginPillControlsPacket`](../../crates/aureline-notebook/src/implement_kernel_picker_rows_and_kernel_origin_pills_with_kernel_class_environment_identity_locality_trust_limits_exact_or_degraded_provenance_and_rerun_reattach_continuity_across_claimed_m5_notebook_surfaces/mod.rs),
so a claimed M5 notebook, kernel-manager, debug, review, or CLI surface can project a picker row and
an origin pill that make **kernel choice and kernel origin explicit before a user acts on a runtime
that differs from the notebook they opened** — with kernel choice truth and kernel origin truth kept
distinct, and exact continuity never implied when the environment fingerprint differs materially.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_kernel_picker_row`

Given a candidate's kernel selection state, the resolver derives a **choice state**:

- `selected` → `currently_selected` (the current kernel, selectable now)
- `recommended` → `recommended_choice` (selectable now)
- `available` → `available_choice` (selectable now)
- `needs_install` → `needs_setup_first` (must carry an explicit install note), not selectable now
- `incompatible` → `incompatible_choice` (must carry an explicit incompatible note), not selectable now
- `unavailable` → `unavailable_choice` (must carry an explicit unavailable note), not selectable now

A user can therefore always tell whether a candidate is the **current kernel, a recommended or
available choice, or blocked** before acting; an incompatible, unavailable, or install-first
candidate can never read as a clean, immediately-selectable choice. Each candidate keeps its kernel
class, environment identity/fingerprint, locality, compatibility state, trust/policy limits, and
last-seen availability visible, so a user can **choose another kernel without losing sight of
provenance, compatibility, or trust limits**.

### `resolve_kernel_origin_pill`

Given a pill's kernel origin trust state and environment fingerprint state, the resolver derives a
**provenance class** and whether **exact continuity** may be claimed:

- `trusted_origin` / `first_party` → `exact_provenance`
- `third_party` / `unverified_origin` → `degraded_provenance` (must carry an explicit degraded note)
- `restricted_origin` → `restricted_provenance` (must carry an explicit restricted note)
- `unknown_origin` → `unknown_provenance` (must carry an explicit unknown-origin note)

Exact continuity across reattach / rerun may be claimed **only** when the environment fingerprint
`matches` the last run **and** the provenance is exact. A `drifted`, `unknown`, or `not_evaluated`
fingerprint always carries its own drift note and blocks the exact-continuity claim — so a kernel
change never silently implies exact continuity when the environment fingerprint differs materially,
and a third-party, unverified, restricted, or unknown origin can never read as exact provenance.

## The two component vectors

Each **kernel picker row** names its candidate's kernel class (so local-interpreter, virtual-env,
conda-env, container, remote, and managed candidates never collapse into one badge), environment
identity, locality, compatibility state, trust/policy limits, and last-seen availability, and always
offers keyboard-complete **choose / inspect / view-compatibility** actions so a user can choose
another kernel without losing sight of provenance, compatibility, or trust limits.

Each **kernel origin pill** names where the current kernel physically runs (so local, SSH,
container, devcontainer, managed, and browser-bridge kernels never collapse into one unlabeled
badge), how trusted that origin is, its derived provenance class, its environment fingerprint state,
and whether reattaching / rerunning keeps exact continuity, and always offers keyboard-complete
**inspect / view-provenance / copy** actions so the kernel origin stays visible and copyable in
notebook tabs, headers, debug bridges, and support exports.

## Hard invariants

Every component keeps four hard invariants `false`, enforced by the Rust validator and mirrored as
`const false` in the schema:

- `collapses_kernel_origins_into_one_badge` — local / SSH / container / managed / browser-bridge
  kernels never collapse into one unlabeled badge.
- `implies_exact_continuity_on_material_drift` — a kernel change never implies exact continuity when
  the environment fingerprint differs materially.
- `hides_trust_or_compatibility_behind_hover_only` — trust and compatibility are never hidden behind
  a hover-only affordance.
- `overwrites_provenance_without_review` — resolved provenance is never overwritten with
  lower-confidence provenance without review.

Every next step names one stable **kernel-manager / notebook / docs / support** deep link rather
than an ephemeral overlay, and raw notebook payloads, pasted paths, credentials, and private
endpoints never cross the export boundary.

## Coverage and acceptance criteria

The seeded packet carries six kernel picker rows covering every kernel candidate kind, selection
state, and derived choice state, and six kernel origin pills covering every kernel origin class,
origin trust state, derived provenance class, and environment fingerprint state. The validator
asserts that coverage, that each derived class equals the resolved class, that a pill claims exact
continuity only when the derived truth allows it, and that each conditional note is present, so the
packet proves the acceptance criteria:

- Users can choose another kernel without losing sight of provenance, compatibility, or trust limits
  (per-candidate kernel class, environment identity, compatibility, and trust/policy limit fields,
  the mandatory `choose_kernel` / `inspect_candidate` / `view_compatibility` actions, and the
  `choose_another_kernel_without_losing_provenance` review flag).
- Kernel origin remains visible in notebook tabs, headers, debug bridges, and support exports
  (shared frozen origin vocabulary, the mandatory `copy_origin_identity` action, and the
  `kernel_origin_visible_in_tabs_headers_debug_support` review flag).
- Kernel changes do not silently imply exact continuity when the environment fingerprint differs
  materially (the `may_claim_exact_continuity` gate, the `drift_note`, and the
  `exact_continuity_never_implied_on_material_drift` review flag).

## Truth source and artifacts

The Rust validator in `crates/aureline-notebook` is the authoritative gate. The headless emitter
`aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive` is the only mint-from-truth
path for:

- the checked support export and matrix CSV under
  `artifacts/release/m5-kernel-picker-row-kernel-origin-pill-proof/`,
- the Markdown design report at
  `artifacts/design/m5-kernel-picker-row-kernel-origin-pill.md`, and
- the narrowed scenario fixtures under
  `fixtures/ui/m5-kernel-picker-row-kernel-origin-pill-controls/`.

The boundary schema is
[`schemas/ui/m5-kernel-picker-row-kernel-origin-pill-controls.schema.json`](../../schemas/ui/m5-kernel-picker-row-kernel-origin-pill-controls.schema.json).
