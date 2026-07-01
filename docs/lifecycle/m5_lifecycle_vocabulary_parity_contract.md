# M5 lifecycle-vocabulary parity contract

This lane is the **controlled-vocabulary parity capstone** on top of the frozen
[M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes the controlled M5 lifecycle-state vocabulary — the fifteen state terms every long-lived M5
object must speak — and names the consumer surfaces that must project that vocabulary. This lane
certifies that each controlled term keeps **one meaning across every consumer surface**, stays
**semantically distinct** rather than collapsing into a generic failure, exports **one stable status
code identically** on every export path, and narrows its **published release/docs/help copy
automatically** when evidence or support state changes — and that the same state-truth vocabulary
survives a headless or companion-adjacent execution.

The lane exists so that M5 can honestly ship its growing mix of notebook, data/API, AI, remote,
preview, operator, docs, and release surfaces without controlled lifecycle wording drifting by
surface or disappearing in export paths. It closes the gap between "individual flows have states"
and "Aureline has one controlled lifecycle language users, support, automation, and docs all share":
`retest_pending`, `experimental`, `policy_blocked`, and `read_only_degraded` must never be rephrased
into a vague generic error, and no surface may quietly use legacy wording while the claim publishes
as if every surface agreed.

## Controlled terms

The certification covers exactly the fifteen controlled lifecycle-state terms the matrix freezes,
and refuses to ship if any is missing:

- `ready` — Ready
- `warming` — Warming
- `partial` — Partial
- `stale` — Stale
- `rebuilding` — Rebuilding
- `restricted` — Restricted
- `policy_blocked` — Policy blocked
- `reconnecting` — Reconnecting
- `degraded` — Degraded
- `read_only_degraded` — Read-only degraded
- `unavailable` — Unavailable
- `rollback_available` — Rollback available
- `deprecated` — Deprecated
- `experimental` — Experimental
- `retest_pending` — Retest pending

Each term row's grounding — the object families that admit the term, the required consumer surfaces,
and the applicable downgrade triggers — is pulled straight from the frozen matrix's seeded packet,
so this lane mints no parallel lifecycle vocabulary and cannot certify a term the matrix does not
freeze. The required consumer-surface set is **derived** as the union of every consumer surface the
matrix declares on any governed object family (`product_ui`, `cli`, `docs_help`, `diagnostics`,
`support_export`, `telemetry`, `claim_tooling`, `release_notes`).

## Certified parity dimensions

Each row is certified across the four parity dimensions (`cross_surface_term`,
`semantic_distinction`, `export_code_parity`, `published_copy_narrowing`):

- **cross-surface term** — `term_stable_across_all_surfaces` (green), a disclosed
  `disclosed_surface_paraphrase` where one surface presents a disclosed, waivered friendlier label
  mapped to the same controlled token (yellow), or `term_meaning_drifted_across_surfaces` (red: the
  term means different things on different surfaces).
- **semantic distinction** — `distinct_meaning_preserved` (green), a disclosed
  `disclosed_grouped_presentation` where a compact surface groups the term under a disclosed family
  header while still naming it individually (yellow), or `collapsed_into_generic_failure` (red: the
  term reads as a generic failure).
- **export-code parity** — `code_exports_identically_all_paths` (green), a disclosed
  `disclosed_partial_export` where the status code exports in a reduced form on a subset of surfaces
  while still naming the same controlled state (yellow), or `status_code_unexportable` (red: the code
  stopped exporting on an export path).
- **published-copy narrowing** — `copy_auto_narrows_on_state_change` (green), a disclosed
  `disclosed_manual_narrowing` where published copy narrows only through a disclosed manual publish
  step (yellow), or `stale_copy_overclaims` (red: published copy stayed stale and overclaims after
  the state changed).

In addition, every row carries `headless_parity_preserved`: a hard invariant that the same
state-truth vocabulary survives a headless or companion-adjacent execution. A row that loses it
**blocks** (a `state_vocabulary_drift` cause), because a headless run must not report a different
state language than the in-product surface.

## Derived status and the parity lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when any of the
four dimensions takes a disclosed narrowing. It drops to `red` when a term's meaning drifts across
surfaces, it collapses into a generic failure, its status code stops exporting, its published copy
stays stale and overclaims, headless/companion-adjacent parity is lost, or the row fails to certify
every declared consumer surface. The consumer-surface completeness check is the lint that prevents a
certification from silently regressing into a partial, single-surface view — the exact regression
that lets one surface keep legacy or vague wording while the claim publishes as if every surface
agreed. The Rust validator in `crates/aureline-shell/src/m5_lifecycle_vocabulary_parity` is the
authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_surface_paraphrase` narrowing must
additionally carry an active, matching, unexpired waiver.

## Records

- **Parity packet** — the full set of rows with derived per-row status, aggregate green/yellow/red
  counts, active waivers, the exact term causes, and the blocking findings the lane refuses to ship
  with.
- **Parity dashboard** — a light projection the product UI / CLI / diagnostics / support / telemetry
  automation reads to auto-narrow a controlled term's published wording when its parity falls out of
  policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref, build
  id, each controlled term, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never raw
URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity`) is the only
mint-from-truth path for:

- `artifacts/release/m5-lifecycle-vocabulary-parity-proof/packet.json`
- `artifacts/release/m5-lifecycle-vocabulary-parity-proof/dashboard.json`
- `artifacts/release/m5-lifecycle-vocabulary-parity-proof/support_export.json`
- `artifacts/release/m5-lifecycle-vocabulary-parity-proof/matrix.csv`
- `artifacts/lifecycle/m5-lifecycle-vocabulary-parity.md` (this report's rendered companion)
- `fixtures/state/m5-lifecycle-vocabulary-parity/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/lifecycle/m5-lifecycle-vocabulary-parity.schema.json`](../../schemas/lifecycle/m5-lifecycle-vocabulary-parity.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity -- validate
cargo test -p aureline-shell --test m5_lifecycle_vocabulary_parity_fixtures
cargo test -p aureline-shell m5_lifecycle_vocabulary_parity
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity --"
$BIN packet         > artifacts/release/m5-lifecycle-vocabulary-parity-proof/packet.json
$BIN dashboard      > artifacts/release/m5-lifecycle-vocabulary-parity-proof/dashboard.json
$BIN support-export > artifacts/release/m5-lifecycle-vocabulary-parity-proof/support_export.json
$BIN csv            > artifacts/release/m5-lifecycle-vocabulary-parity-proof/matrix.csv
$BIN markdown       > artifacts/lifecycle/m5-lifecycle-vocabulary-parity.md
$BIN packet         > fixtures/state/m5-lifecycle-vocabulary-parity/packet.json
$BIN dashboard      > fixtures/state/m5-lifecycle-vocabulary-parity/dashboard.json
$BIN support-export > fixtures/state/m5-lifecycle-vocabulary-parity/support_export.json
$BIN compact        > fixtures/state/m5-lifecycle-vocabulary-parity/compact.txt
```
