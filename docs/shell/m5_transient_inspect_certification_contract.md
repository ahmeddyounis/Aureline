# M5 tooltip, hovercard & peek-panel representation, promotion, reach & stale-labeling contract

This lane is the **transient-inspect certification capstone** on top of the frozen
[M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the transient
inspect primitives — the tooltip, the hovercard, the peek panel, and the
pinned-preview promotion, with their representation classes, promotion states,
source/provider/freshness labels, accessibility routes, and mandatory labels — this
lane *certifies* that, in every claimed M5 inspect context, a tooltip, hovercard, or
peek panel preserves canonical target identity, source/provider class,
freshness/mapping quality, and representation label; that pinning or promoting a peek
keeps that same identity and state without dropping its representation or provenance
truth; that no glanceable information is hover-only or pointer-only but stays
reachable through keyboard focus, an explicit context action, or an info affordance on
touch/pen and compact layouts; and that stale, cached, or approximate preview content
stays visibly labeled before and after pinning and is reconstructable from a support
export.

The lane exists so that M5 can honestly claim mature shell quality: glanceable
metadata never mutates into a hidden second application, users never mistake a
stale/cached preview for live canonical content, and no critical instruction is
trapped behind a hover-only tooltip.

## Governed inspect contexts

The certification proof covers exactly seven claimed M5 inspect contexts, and refuses
to ship if any is missing:

- `search_results` — Search results tooltips & peek
- `docs_help` — Docs / help hovercards
- `review_change` — Review / change hovercards & peek
- `editor` — Editor symbol tooltips & peek
- `data_grid` — Data grid cell hovercards & peek
- `profiler` — Profiler flame-graph peek
- `operator` — Operator console tooltips & peek

## Per-context certification row

Each row names the transient inspect primitives it drives (`tooltip`, `hovercard`,
`peek_panel`, and `pinned_preview_promotion`) and — pulled straight from the union of
the frozen matrix's four transient rows — the representation classes, promotion
states, freshness labels, required labels, accessibility routes, consumer surfaces,
and downgrade triggers. It is certified across four posture axes:

- **representation truth** — `identity_source_freshness_representation_labeled`
  (green), `disclosed_reduced_representation_detail` (yellow: a compact-width
  hovercard falls back to a shorter form while identity/source/freshness stay
  labeled), or `source_provider_or_freshness_hidden` (red: a cached/stale value can
  read as live canonical content).
- **promotion continuity** — `pin_open_paths_preserve_identity_and_state` (green),
  `disclosed_reduced_promotion_path` (yellow: a waivered deferral of one promotion
  path while pin/open still preserve identity and state), or
  `promotion_drops_identity_or_representation` (red: a promotion or pin drops the
  target identity or representation, or a peek promotes without disclosing its
  preview-only versus live-editable posture).
- **non-hover reach** — `keyboard_focus_context_reachable` (green),
  `disclosed_reduced_reach_route` (yellow: one route reduced, at least one non-hover
  route remains, disclosed), or `information_hover_or_pointer_only` (red).
- **stale-preview labeling** — `stale_labeled_and_export_reconstructable` (green),
  `disclosed_partial_capture` (yellow), or
  `stale_reads_as_live_or_absent_from_capture` (red: a stale preview reads as live or
  a pinned preview is absent from capture).

Each row also carries the hard invariant `tooltip_never_sole_critical_instruction`;
`false` is a blocker (a tooltip carrying the sole instruction for an action, reachable
only through pointer hover).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow`
when it discloses a reduced representation detail, a reduced promotion path (backed by
a waiver), a reduced non-hover reach route, or a partial support-export capture. It
drops to `red` when any axis reaches its blocked state, a tooltip carries the sole
critical instruction, or its representation classes / promotion states / required
labels / stale labels are incomplete. Those structural lints —
`representation_classes_complete`, `promotion_states_complete`,
`required_labels_complete`, `stale_labels_present` — are what prevent a later transient
surface from shipping without its full tooltip/hovercard/peek/pinned/provenance/
truncated representation vocabulary, its full transient → pinned → promoted → detached
→ demoted → dismissed transition set, its identity/state/keyboard-route/source-provider/
freshness/reopen-path labels, or its stale/cached freshness labels. The Rust validator
in `crates/aureline-shell/src/m5_transient_inspect_certification` is the authoritative
gate.

A narrowed (non-green) row must disclose a reason; a
`disclosed_reduced_promotion_path` narrowing must additionally carry an active,
matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status,
  aggregate green/yellow/red counts, active waivers, the exact certification causes,
  and the blocking findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / attention router /
  release automation reads to auto-narrow a claimed inspect context when its
  certification proof falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id,
  matrix ref, build id, each context, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short
labels — never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or
credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification`)
is the only mint-from-truth path for:

- `artifacts/release/m5-transient-inspect-certification-proof/packet.json`
- `artifacts/release/m5-transient-inspect-certification-proof/dashboard.json`
- `artifacts/release/m5-transient-inspect-certification-proof/support_export.json`
- `artifacts/release/m5-transient-inspect-certification-proof/matrix.csv`
- `artifacts/shell/m5-transient-inspect-certification.md` (this report's rendered
  companion)
- `fixtures/ui/m5-transient-inspect-certification/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-transient-inspect-certification.schema.json`](../../schemas/shell/m5-transient-inspect-certification.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification -- validate
cargo test -p aureline-shell --test m5_transient_inspect_certification_fixtures
cargo test -p aureline-shell m5_transient_inspect_certification
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification --"
$BIN packet         > artifacts/release/m5-transient-inspect-certification-proof/packet.json
$BIN dashboard      > artifacts/release/m5-transient-inspect-certification-proof/dashboard.json
$BIN support-export > artifacts/release/m5-transient-inspect-certification-proof/support_export.json
$BIN csv            > artifacts/release/m5-transient-inspect-certification-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-transient-inspect-certification.md
$BIN packet         > fixtures/ui/m5-transient-inspect-certification/packet.json
$BIN dashboard      > fixtures/ui/m5-transient-inspect-certification/dashboard.json
$BIN support-export > fixtures/ui/m5-transient-inspect-certification/support_export.json
$BIN compact        > fixtures/ui/m5-transient-inspect-certification/compact.txt
```
