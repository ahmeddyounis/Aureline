# Parser Substrate, Syntax-Tree Identity, And Coordinate Normalization ADR Seed

- **Decision id:** pending formal register row
- **Status:** Accepted
- **Decision date:** 2026-04-26
- **Owner:** `@ahmeddyounis`
- **Forum:** architecture_council
- **Related requirement ids:** `none`

## Context

Aureline's product sources already choose Tree-sitter as the syntax and
structure backbone for syntax highlighting, folding, structural
selection, breadcrumbs, bracket cues, local symbols, and syntax-aware
edits. They also require the editor to own byte offsets, line starts,
and grapheme boundaries independently from LSP, DAP, notebook, or other
external protocol coordinates.

The gap this seed closes is the boundary between those two facts. If
syntax trees, protocol ranges, refactors, diagnostics, folds, minimap
markers, and support exports each choose their own offsets, freshness
labels, or fallback rules, later features will appear to work while
silently disagreeing about where source truth lives. Parser failures,
grammar mismatches, and decode-recovery files are especially risky
because they can still produce useful text views while structural
features should narrow or stop.

This ADR freezes one parser substrate contract and two machine-readable
schemas:

- [`/schemas/language/parse_session.schema.json`](../../schemas/language/parse_session.schema.json)
  defines the parse-session, grammar-provenance, syntax-tree identity,
  parse freshness, cache, budget, failure, and parse-derived cue record.
- [`/schemas/language/coordinate_mapping.schema.json`](../../schemas/language/coordinate_mapping.schema.json)
  defines the coordinate-normalization record used when bytes, code
  points, graphemes, UTF-16 protocol offsets, syntax points, visual
  columns, and editor-native positions cross a boundary.
- [`/fixtures/language/parse_cases/`](../../fixtures/language/parse_cases/)
  contains worked records for exact parsing, parse errors, missing
  grammar, grammar mismatch, and decode-recovery coordinate blocking.

This contract composes with:

- [`/docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md)
  for buffer snapshots, internal UTF-8 text, undo classes, decode
  recovery, and source fidelity.
- [`/docs/verification/text_fidelity_packet.md`](../verification/text_fidelity_packet.md)
  for Unicode coordinate projections and save/decode-recovery
  vocabulary.
- [`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md)
  for syntax provider attribution and semantic downgrade language.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  for anchor remap, freshness, and mutation-safety disclosure.

If this ADR disagrees with the PRD, technical architecture, technical
design, UI/UX spec, or the linked buffer/text-fidelity contracts, those
documents win and this ADR plus its companion schemas update in the same
change.

## Decision

Aureline will use Tree-sitter as the default parser substrate for
source-code syntax trees. A different parser may serve a language only
when it exposes the same parse-session, syntax-tree identity,
freshness, coordinate, degradation, and export contract defined here.

The editor core owns the canonical text coordinate model. Parser and
protocol coordinate systems are projections from buffer snapshots, never
the editor's source of truth. Tree-sitter byte offsets and points are
accepted as parser-native inputs and outputs, but they cannot leak into
editor commands, diagnostics, refactors, support exports, or protocol
bridges without a `coordinate_mapping_record`.

## Grammar Packaging And Provenance

Grammar packages are reusable but governed artifacts.

| Field | Rule |
|---|---|
| `grammar_id` | Stable opaque id for a language grammar family. It is not a display name and is not inferred from file extension alone. |
| `language_id` | Canonical language id selected by file type, shebang, modeline, notebook cell metadata, or workspace override. |
| `grammar_source_class` | One of bundled curated upstream, extension pack, workspace pinned, remote-agent mirror, or not applicable. |
| `grammar_version_ref` | Exact package or commit identity used to build the grammar. |
| `grammar_abi_ref` | Runtime ABI or parser compatibility identity. ABI mismatch is a typed failure, not a best-effort load. |
| `query_pack_ref` | Highlight, fold, indent, outline, injection, and local-symbol query pack identity. Query changes invalidate derived cues even when the tree shape is unchanged. |
| `signature_ref` | Signature, checksum, or mirror provenance for the shipped grammar artifact. |
| `upstream_ref` | Upstream project and revision when the grammar is reused or locally patched. |
| `local_patch_ref` | Required for local forks. Every fork needs an exit path back to upstream or a named ownership row. |

Packaging rules:

1. Bundled grammars are signed, versioned, and cache-keyed by grammar
   id, language id, ABI, query pack, and artifact hash.
2. Extension-provided grammars load through the extension trust and
   capability path before they may run in the local parser host.
3. A workspace-pinned grammar can override a bundled grammar only after
   trust policy admits it. The parse session records the override and
   the affected scope.
4. Remote-agent mirrors may parse remote files, but their records still
   carry the same grammar and coordinate fields. Remote parser truth is
   not stronger than local parser truth unless the buffer snapshot and
   grammar provenance match the admitted workspace epoch.
5. Grammar updates, query-pack updates, ABI changes, trust changes, and
   language-id remaps invalidate cached trees and all parse-derived
   cues.

## Parser Host Placement

Parsing is part of the language platform, not the input/render hot path.

| Host class | Allowed work | Forbidden work |
|---|---|---|
| `editor_process_foreground_worker` | Bounded incremental parse slices for the visible buffer after a text snapshot is committed. | Blocking input dispatch, frame submission, grammar downloads, untrusted grammar loading, full workspace scans. |
| `local_sidecar_worker` | Grammar loading, extension grammar isolation, full-file parse work, background reparses, cache warming, and query-pack execution. | Owning unsaved editor state or mutating shell view state directly. |
| `workspace_remote_agent` | Remote workspace parse work under the same contracts when the source snapshot is remote-owned. | Returning raw parser offsets without coordinate mapping or hiding remote freshness. |
| `imported_snapshot` | Support or index replay of previously sealed parse metadata. | Claiming current editor freshness. |
| `not_scheduled` | Explicit degraded states where no parser may run. | Silent syntax-derived UI. |

The visible editor path may request a foreground incremental parse, but
publishing the result is asynchronous. If the parser yields or is
superseded, the editor keeps the previous current tree if it is still
honestly reusable, narrows to cached or lexical cues, or suppresses the
structural cue.

## Parse-Session Lifecycle

Every parse is represented by one `parse_session_record`.

1. **Request admitted.** The caller declares the buffer snapshot,
   language id, requested cue set, parse request class, budget policy,
   trust state, and cancellation token.
2. **Grammar resolved.** The grammar resolver produces grammar
   provenance or a typed missing/mismatch/blocked state.
3. **Coordinate profile attached.** The parse session names the
   buffer's coordinate profile and decode state before parser-native
   offsets can be consumed.
4. **Old tree selected.** Incremental parses may reuse one previous tree
   only when buffer lineage, grammar identity, ABI, query pack, and
   language id match.
5. **Parse executed or yielded.** The parser runs within its budget. It
   must observe cancellation when a newer buffer version is committed.
6. **Tree identity minted.** A successful or partial parse publishes a
   syntax-tree identity. Pointer identity is never exported.
7. **Derived cues projected.** Highlighting, folds, indent guides,
   structural selection, breadcrumbs, outline rows, minimap markers,
   bracket cues, and local symbols each publish their own cue posture.
8. **Cache and freshness recorded.** Cache status, invalidation reason,
   freshness class, and degraded/failure states remain visible.

## Incremental Parse Budget And Yield Rules

Budget values are policy seeds. Implementations may tune the numeric
thresholds with benchmark evidence, but they may not remove
cancellation, yielding, or typed degradation.

| `budget_policy_class` | Intended use | Slice budget | Yield rule | Degraded publication |
|---|---|---:|---|---|
| `visible_edit_interactive` | Active buffer after a committed edit | 3 ms soft, 8 ms burst ceiling | Yield after one slice, on newer edit, or on input/render pressure. | Keep previous tree as `warm_cached` if ranges can be remapped; otherwise narrow cues. |
| `foreground_visible_file` | Visible file open, reveal, or language switch | 10 ms slices | Yield between slices and before blocking shell responsiveness. | Publish partial tree or lexical fallback with affected cues. |
| `background_workspace` | Workspace warming, index/search feeds, graph ingest | 25 ms slices | Yield to higher-priority visible parse work and cancellation. | Mark structural providers warming or stale. |
| `large_file_reduced` | Files beyond normal edit thresholds or hostile/minified files | 2 ms visible slices | Prefer lexical/bracket scanner; do not attempt full tree unless policy allows. | Suppress expensive cues and label syntax as reduced. |
| `export_replay_bounded` | Support/export or imported parse replay | 25 ms slices | Stop on stale or missing source epoch. | Export metadata with unverified freshness, not current syntax truth. |

Rules:

1. A parse may be cancelled after any committed buffer version newer
   than the requested version.
2. A yielded parse is not failure. It publishes
   `yielded_budget_exhausted` until completed, cancelled, or superseded.
3. A partial tree with parser error nodes may support highlighting and
   bracket cues, but broad structural selection, refactor anchors, and
   semantic graph ingest must narrow or block according to the cue
   posture.
4. Query-pack execution shares the same budget posture as parsing.
   Expensive query packs cannot bypass parser scheduling.

## Syntax-Tree Identity And Freshness

Syntax-tree identity is value identity, not process memory identity.

A syntax tree identity contains:

- `syntax_tree_id`
- `tree_epoch_ref`
- `document_ref`
- `buffer_id`
- `buffer_version`
- `buffer_content_hash_ref`
- `decoded_text_hash_ref`
- `parser_substrate_class`
- `grammar_id`
- `language_id`
- `grammar_version_ref`
- `grammar_abi_ref`
- `query_pack_ref`
- `parser_host_class`
- `parse_session_id`
- `parse_started_at`
- `parse_completed_at`
- `parse_quality_class`
- `freshness_class`

Freshness rules:

1. `current_buffer_version` means the tree was produced from the
   currently admitted buffer version with matching grammar and query
   identities.
2. `warm_cached` means a previous tree is intentionally reused while a
   newer parse is pending and affected ranges are known.
3. `stale_buffer_version` means the tree belongs to older text and must
   not host mutation or exact anchors.
4. `stale_grammar_version` means grammar, ABI, or query-pack identity
   changed after the tree was produced.
5. `unverified_imported` means parse metadata came from replay/import
   and may support inspection only.

No consumer may treat stale or imported syntax as current editor truth.

## Coordinate Normalization

The coordinate model is governed by
`coordinate_mapping_record`.

Rules:

1. Internal buffer coordinates are UTF-8 byte offsets plus zero-based
   line-start indexes and grapheme boundaries.
2. Editor-native positions are line plus grapheme-aligned column. They
   may carry view-local visual columns, but visual columns are never
   saved as source truth.
3. Ranges are half-open: start included, end excluded.
4. Tree-sitter points are zero-based rows plus byte columns. They are
   parser-native coordinates and must be translated before crossing into
   editor, protocol, diagnostic, refactor, or export records.
5. Protocol adapters project outward on demand. LSP UTF-16 positions,
   DAP UTF-8 positions, notebook cell positions, search-result spans,
   clipboard spans, and support-export spans cannot become internal
   editor coordinates.
6. A projection that would split a surrogate pair, grapheme cluster,
   combining sequence, ZWJ sequence, regional-indicator pair, or
   unresolved decode-recovery region is blocked and labeled. It is not
   rounded to the nearest position.
7. Code-point offsets and grapheme indices are useful projections but
   not internal authority. They are recomputed from the admitted buffer
   snapshot.
8. Mixed-encoding or decode-recovery buffers may expose resolved ranges.
   Projections over unresolved raw-byte ranges return
   `unavailable_decode_recovery`.

## Fallback And Degraded States

Parser failures are typed product states.

| State | Required behavior |
|---|---|
| `degraded_no_grammar` | Syntax highlighting may use lexical/plain-text fallback. Structural selection, outline, breadcrumbs, semantic graph ingest, and refactor anchors are unavailable unless another governed provider supplies them. |
| `failed` with `grammar_abi_mismatch` | Do not load the grammar. Invalidate cached trees for that grammar identity and surface grammar mismatch in status/export rows. |
| `completed` with `partial_tree_with_errors` | Highlighting and local bracket cues may remain available. Folds, structural selection, minimap markers, and local symbols must declare partial posture. |
| `degraded_decode_recovery` | Preserve raw bytes and block parser projections over unresolved regions. Structural cues over clean ranges may be partial only when range mapping is exact. |
| `stale_awaiting_reparse` | Cached cues may remain visible only with stale or cached posture. Mutation, exact anchors, and refactor previews must not rely on the stale tree. |
| `cancelled_superseded` | The record remains useful for debugging and export, but no product surface may present it as current. |

Derived cue posture is per cue:

- `available_exact`
- `available_partial`
- `cached_only`
- `fallback_heuristic`
- `suppressed_due_to_degradation`
- `blocked`

A parse session can therefore support exact highlighting, partial folds,
blocked structural selection, and suppressed minimap markers at the same
time without hiding why those cues differ.

## Cache Invalidation

Parser caches are derived state. They may accelerate the product but
cannot become source truth.

Cache keys include:

- document ref and buffer identity;
- buffer version or content hash;
- encoding/decode state;
- language id;
- grammar id, version, ABI, and artifact hash;
- query pack ref;
- parser substrate class;
- parser host class where host behavior affects output;
- trust or policy epoch when it admits or blocks a grammar.

Invalidate parse trees and derived cues on:

- buffer edit outside a remappable incremental range;
- language id change;
- grammar, ABI, query pack, or package signature change;
- trust, restricted-mode, or extension-capability change;
- encoding override, decode-recovery resolution, BOM/newline conversion
  that changes byte layout;
- cache corruption or failed hash check;
- remote workspace epoch change; or
- explicit user or support action to clear derived state.

## Export And Support Rules

Parse records are export-safe metadata by default.

Exports may include:

- parse session id, syntax tree id, grammar provenance, freshness,
  lifecycle state, failure/degradation class, cue posture, and coordinate
  mapping summaries;
- opaque buffer/document refs, content hash refs, epoch refs, and
  fixture/corpus refs; and
- range summaries only after they have been normalized through
  `coordinate_mapping_record`.

Exports must not include raw source text, raw notebook payloads, raw
grammar binaries, raw parser logs, raw extension process arguments, raw
absolute paths, hostnames, URLs, or secret material unless a separate
high-trust export policy explicitly admits them.

Parse-derived cues exported for folds, indent guides, structural
selection, breadcrumbs, minimap markers, diagnostics, or refactor
previews must carry:

- cue class;
- cue posture;
- producer parse session id;
- syntax tree id when present;
- freshness class;
- coordinate mapping id when a range crosses a boundary; and
- degraded or blocked reason when the cue is not exact.

## Alternatives Considered

- **Parser-specific offsets everywhere.** Rejected because Tree-sitter
  byte columns, LSP UTF-16 positions, visual columns, and editor
  grapheme positions would leak into unrelated surfaces and make exact
  refactors or diagnostics unreviewable.
- **Language-server syntax as the only structure source.** Rejected
  because language servers vary widely and do not reliably provide the
  low-latency local structure needed for highlighting, folds, and
  structural selection.
- **Silent plain-text fallback.** Rejected because missing grammars and
  parse failures still affect user trust. Fallbacks are allowed only
  when their narrower authority is typed and visible.
- **Parser cache as truth.** Rejected because derived trees are
  invalidated by text, grammar, query, trust, encoding, and remote epoch
  changes. The buffer snapshot remains authoritative.

## Source Anchors

- `.t2/docs/Aureline_PRD.md:152` - Tree-sitter is the syntax and
  structure backbone while LSP remains a compatibility layer.
- `.t2/docs/Aureline_PRD.md:855` - the buffer model is UTF-8 first with
  lossless handling for legacy encodings and binary-safe views.
- `.t2/docs/Aureline_PRD.md:870` - Tree-sitter feeds tokenization,
  syntax trees, folds, structure views, breadcrumbs, selection
  expansion, and syntax-aware edits.
- `.t2/docs/Aureline_PRD.md:1379` - internal byte offsets, line starts,
  and grapheme boundaries are first-class editor data.
- `.t2/docs/Aureline_PRD.md:1380` - protocol adapters translate to
  external UTF encodings at the boundary.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:404` - the
  architecture decision summary selects Tree-sitter for the syntax
  engine.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1411` -
  efficient mapping among byte offsets, line starts, graphemes, and
  protocol coordinates is required.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1419` -
  internal editor positions stay independent from external protocol
  coordinate systems.
- `.t2/docs/Aureline_Technical_Design_Document.md:14135` - parser,
  encoding, Unicode, generated-file guards, anchors, parser state, and
  freshness are first-party editor structural truth.
