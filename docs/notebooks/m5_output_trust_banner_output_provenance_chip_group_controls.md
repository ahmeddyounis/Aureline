# M5 output trust banners and output provenance chip groups

The output trust banner and the output provenance chip group are two of the eight governed
notebook-kernel-output components frozen by the
[M5 notebook-kernel-output component matrix](m5_notebook_kernel_output_component_matrix.md). This
lane implements those two families as two co-equal control vectors in one export-safe packet,
[`OutputTrustBannerOutputProvenanceChipGroupControlsPacket`](../../crates/aureline-notebook/src/implement_output_trust_banners_and_output_provenance_chip_groups_with_plaintext_sanitizedrich_trustedlocalactive_isolatedremoteactive_class_stale_output_honesty_and_copy_export_choice_across_claimed_m5_notebook_outputs/mod.rs),
so a claimed M5 notebook, output-viewer, AI-context, review, or support / export surface can project
a banner and a chip group that make **output trust explicit before a user copies, shares, or acts on
a rich result** — with output trust truth and output provenance truth kept distinct, and stale
output never presented as live.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_output_trust_banner`

Given a banner's output trust class, the resolver derives a **presentation class**:

- `trusted_output` → `trusted_local_active` (trusted, local active content)
- `sanitized_output` → `sanitized_rich` (rich content with active content stripped)
- `sandboxed_output` → `isolated_remote_active` (active content isolated in a sandbox)
- `raw_active_output` → `plain_text` (a raw / untrusted output shown inert as plain text, never run)
- `blocked_output` → `blocked_content` (withheld by policy; raw available behind review)
- `unknown_trust` → `unknown_content` (trust class undetermined; treat as untrusted)

A user can therefore always tell whether an output is **plain text, sanitized rich content, trusted
local active content, or isolated remote active content** before they copy, share, or act on it, and
a raw or untrusted output never renders as trusted active content.

Given the banner's freshness state, the resolver derives whether the output **may present as live**:
an output may present as live **only** when its freshness is `live_output`. A `stale_output`,
`cached_output`, `cleared_output`, `superseded_output`, or `no_output` banner is never live and each
non-live state carries its own note, so a stale output **remains visibly stale after notebook edits,
kernel restart, or target / environment drift** and can never read as live truth.

### `resolve_output_provenance_chip_group`

Given a chip group's output provenance kind, the resolver derives an **origin class**:

- `produced_by_cell` → `cell_produced` (internal)
- `produced_by_run` → `run_produced` (internal)
- `imported_output` → `imported_origin` (external; must carry an explicit external note)
- `restored_output` → `restored_origin` (external; must carry an explicit external note)
- `external_output` → `external_origin` (external; must carry an explicit external note)
- `unknown_provenance` → `unknown_origin` (external; must carry an explicit external note)

Given the provenance state, the resolver derives a **lineage resolution** and whether a **current
pinned lineage** may be claimed. A current, pinned lineage may be claimed **only** when the state is
`provenance_complete` (`fully_resolved`) or `execution_count_pinned` (`lineage_pinned`); a
`provenance_partial`, `provenance_missing`, `execution_count_drifted`, or `provenance_stale` lineage
can never read as a current pinned lineage and carries its own note — so an imported, restored,
external, or unknown output never reads as internally produced, and a partial, missing, drifted, or
stale lineage never reads as current.

## The two component vectors

Each **output trust banner** names its output's trust class (so a trust class is never hidden behind
a hover-only affordance), its raw-versus-rendered representation, and its live-versus-stale
freshness, and always offers keyboard-complete **open-raw / export / copy** actions so copy and
export preserve the trust class and the raw-versus-rendered representation instead of flattening the
output into ambiguous evidence.

Each **output provenance chip group** names its output's cell / run identity, its origin class (so an
imported, restored, external, or unknown output never reads as internally produced), its attached
artifacts, and its persistence / retention cues, and always offers keyboard-complete **inspect /
view-artifacts / copy** actions so an output's producing run and lineage stay visible and copyable in
notebook, AI-context, and support exports.

## Hard invariants

Every component keeps four hard invariants `false`, enforced by the Rust validator and mirrored as
`const false` in the schema:

- `presents_stale_output_as_live` — a stale, cached, cleared, superseded, or absent output is never
  presented as live truth.
- `hides_trust_class_behind_hover_only` — the raw / sanitized / active trust class is never hidden
  behind a hover-only affordance.
- `flattens_output_into_ambiguous_evidence` — copy / export never flattens the output into ambiguous
  evidence; the trust class and the raw-versus-rendered representation are always preserved.
- `severs_output_provenance` — an output's canonical provenance is never severed.

Every next step names one stable **notebook / output-viewer / docs / support** deep link rather than
an ephemeral overlay, and raw notebook payloads, pasted paths, credentials, and private endpoints
never cross the export boundary.

## Coverage and acceptance criteria

The seeded packet carries six output trust banners covering every output trust class, freshness
state, and derived presentation class, and six output provenance chip groups covering every output
provenance kind, provenance state, derived origin class, and derived lineage resolution. The
validator asserts that coverage, that each derived class equals the resolved class, that a banner
claims live only when the derived truth allows it, that a chip group claims a current lineage only
when the derived truth allows it, and that each conditional note is present, so the packet proves the
acceptance criteria:

- Users can tell whether notebook output is plain text, sanitized rich content, trusted local active
  content, or isolated remote active content (the derived presentation class, the always-present
  `trust_class_label`, and the `banner_shows_trust_class` review flag).
- Stale outputs remain visibly stale after notebook edits, kernel restart, or target / environment
  drift (the `may_present_as_live` gate, the `stale_note`, and the
  `stale_output_never_presented_as_live` review flag).
- Copy / export actions preserve the trust class and representation choice instead of flattening
  output into ambiguous evidence (the mandatory `open_raw` / `export_output` / `copy_output`
  actions, the `copy_export_choice_note`, and the
  `copy_export_preserves_trust_and_representation` review flag).

## Truth source and artifacts

The Rust validator in `crates/aureline-notebook` is the authoritative gate. The headless emitter
`aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive` is the only
mint-from-truth path for:

- the checked support export and matrix CSV under
  `artifacts/release/m5-output-trust-banner-output-provenance-chip-group-proof/`,
- the Markdown design report at
  `artifacts/design/m5-output-trust-banner-output-provenance-chip-group.md`, and
- the narrowed scenario fixtures under
  `fixtures/ui/m5-output-trust-banner-output-provenance-chip-group-controls/`.

The boundary schema is
[`schemas/ui/m5-output-trust-banner-output-provenance-chip-group-controls.schema.json`](../../schemas/ui/m5-output-trust-banner-output-provenance-chip-group-controls.schema.json).
