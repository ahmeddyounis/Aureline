# Semantic-readiness projection matrix and readiness-inspector contract

This document freezes how the **semantic-readiness** vocabulary is projected
into shell, search, diagnostics, migration, and support/export surfaces so
readiness truth is one reusable contract instead of surface-local prose.

It composes with:

- Frozen vocabulary: [`docs/filesystem/filesystem_identity_vocabulary.md`](filesystem_identity_vocabulary.md)
- Canonical schema: [`schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)

Where this document disagrees with the frozen vocabulary or the canonical
schema, those sources win and this projection matrix updates in the same
change.

Machine-readable companions:

- View/inspector contract: [`schemas/filesystem/semantic_readiness_view.schema.json`](../../schemas/filesystem/semantic_readiness_view.schema.json)
- Explainer copy id register: [`artifacts/filesystem/readiness_explainer_copy_ids.yaml`](../../artifacts/filesystem/readiness_explainer_copy_ids.yaml)
- Parity matrix: [`artifacts/filesystem/semantic_readiness_parity_matrix.yaml`](../../artifacts/filesystem/semantic_readiness_parity_matrix.yaml)
- Fixture corpus: [`fixtures/filesystem/semantic_readiness_cases/`](../../fixtures/filesystem/semantic_readiness_cases/)
- Edge-case corpus: [`fixtures/filesystem/semantic_readiness_edge_cases/`](../../fixtures/filesystem/semantic_readiness_edge_cases/)

## Scope

Frozen here:

- the projection requirements each named surface must satisfy;
- the **minimum parity field set** that must remain identical across live UI,
  screenshot-safe surfaces, machine output, and exported evidence; and
- the readiness-inspector **why-not-ready** record that adds structure
  (blocking subsystem, freshness source, confidence class, escalation link)
  without redefining the underlying readiness vocabulary.

Out of scope:

- implementing a readiness inspector UI, CLI commands, or readiness producers;
- defining new readiness states or renaming the frozen vocabulary; and
- re-litigating the filesystem identity model or watcher/save contracts.

## Canonical readiness record

The canonical record every surface consumes is the `semantic_readiness` frame
and its wrapper `semantic_readiness_record` in the filesystem identity schema.

Surfaces MUST treat the following as the authoritative values and MUST NOT
re-label them with surface-local synonyms:

- `semantic_readiness.state`
- `semantic_readiness.not_ready_reason` (required when `state != exact`)
- `semantic_readiness.safe_next_action` (primary routing action)
- `semantic_readiness.safe_next_actions[]` (ordered action menu)
- `semantic_readiness.producer_id`, `producer_version`, `observed_at`
- `semantic_readiness.explainer` (optional short text)
- `semantic_readiness.not_ready_explainer.*` (when present)

Any surface that cannot preserve these fields must project an explicit omission
reason rather than compressing into a generic “unavailable” label.

## Minimum parity fields (cross-surface identical)

The following fields are the **parity floor**. When a surface renders readiness
for the same subject, these values MUST remain identical byte-for-byte across:

- live UI;
- screenshot-safe renders;
- shell/CLI machine output;
- migration reports; and
- support/export evidence.

Parity floor:

- `subject` (workspace/object/producer scope, using stable ids)
- `semantic_readiness.state`
- `semantic_readiness.not_ready_reason` (when `state != exact`)
- `semantic_readiness.safe_next_action`
- `semantic_readiness.producer_id`
- `semantic_readiness.producer_version`
- `semantic_readiness.observed_at`
- `semantic_readiness.not_ready_explainer.title` + `.body` (when present)
- `semantic_readiness.support_export.packet_family` + `.redaction_policy`
  (when exported)
- `semantic_readiness.support_export.parity_signature` (when present)

Surfaces MAY add additional local fields, but they MUST NOT replace or rename
any parity-floor field and MUST NOT widen readiness by omission.

## Surface projection matrix

This matrix names what each surface must preserve when it projects a readiness
record. “Required projection” is the minimum; “May omit” requires an explicit
omission reason on the consuming surface or exported packet.

| Surface | Required projection | May omit (with omission reason) |
|---|---|---|
| Status chip (tabs, breadcrumbs, headers) | `semantic_readiness.state`; when `state != exact`, `not_ready_reason`; a reachable “details” affordance (command id or deep link) | `producer_version`, `observed_at`, full `safe_next_actions[]` |
| Search / quick-open result rows | `state`, `not_ready_reason` (when non-exact), and any ranking-impact disclosure keyed off readiness | `safe_next_actions[]` (only if a details action is present) |
| Diagnostics panel / attention center | full parity floor + at least one safe-next-action row | none (diagnostics is the “full” projection inside product) |
| Shell / CLI (machine output) | parity floor fields encoded without loss (state, reason, action, producer id, timestamps) | localized labels; decorative iconography |
| Shell / CLI (human text) | state label + reason label (when non-exact) + primary safe next action label | raw ids (if a stable reference is shown instead) |
| Migration report (dry-run + export) | parity floor + explicitly stated consequence of non-exact readiness (what the migration could not prove) | `safe_next_actions[]` beyond the primary action |
| Support bundle / evidence export | full `semantic_readiness_record` or view contract record, including support/export projection fields | none |

Rules (frozen):

1. No surface may collapse `partial`, `stale`, `imported`, `heuristic`,
   `unavailable`, and `out_of_scope` into one generic label.
2. When `state != exact`, the surface MUST show one `not_ready_reason` label
   and MUST offer at least one safe-next-action.
3. Compact surfaces MAY omit `producer_version`/`observed_at` inline, but MUST
   provide a keyboard-reachable route to the full record.
4. Support/export is never “best effort”: a support packet that drops the
   parity-floor fields is non-conforming.

## State projection defaults (labels and safety posture)

These defaults define how a state is described when no producer-specific copy
is present. Producers MAY specialise the explainer copy, but the state token
and reason token remain unchanged.

| `semantic_readiness.state` | Default label | Default safety posture |
|---|---|---|
| `exact` | Ready | Claims admissible as authoritative for the producer scope. |
| `imported` | Imported | Never promote to authoritative; always disclose import origin. |
| `heuristic` | Heuristic | Best-effort; must disclose fallback and cap confidence. |
| `stale` | Stale | Inputs changed since last run; must offer refresh/rebuild actions. |
| `partial` | Partial | Coverage incomplete; must disclose which scope is covered vs missing. |
| `unavailable` | Unavailable | Producer cannot serve claims; must disclose why and what still works. |
| `out_of_scope` | Out of scope | Not a failure; must disclose the scope boundary and how to widen it (if allowed). |

## Why-not-ready record (readiness inspector)

The canonical readiness record intentionally keeps the core vocabulary small.
The readiness inspector adds a typed **why-not-ready** record that:

- does not replace `state` / `not_ready_reason`;
- routes users to safe actions deterministically; and
- preserves escalation and docs linkback uniformly across surfaces.

The inspector record is defined by
[`schemas/filesystem/semantic_readiness_view.schema.json`](../../schemas/filesystem/semantic_readiness_view.schema.json)
and carries:

- `blocking_subsystem` — which lane is currently gating the claim (watcher,
  lexical index, semantic graph, generator/lineage, policy/entitlement, remote).
- `freshness_source` — which signal asserted staleness/partiality/unavailability
  (input digests, watcher health, imported snapshot, policy scope, remote link).
- `confidence_class` — `high|medium|low|unknown` aligned with the confidence
  vocabulary used by graph-backed surfaces.
- `safe_next_actions[]` — ordered list of safe next actions with command ids.
- `escalation_target` — a docs reference or support/export route when local
  repair is not possible.

Copy for the not-ready explainer is referenced through stable `copy_id`s in
[`artifacts/filesystem/readiness_explainer_copy_ids.yaml`](../../artifacts/filesystem/readiness_explainer_copy_ids.yaml)
so screenshot-safe, machine output, and export surfaces can prove they quoted
the same approved wording.

## Fixture corpus

The fixture corpus under
[`fixtures/filesystem/semantic_readiness_cases/`](../../fixtures/filesystem/semantic_readiness_cases/)
anchors the minimum acceptance set:

- cold index (warming, partial);
- stale graph (inputs changed, stale);
- degraded watcher (watch fidelity degraded);
- moved root (root identity remapped, stale);
- generated/imported hybrid content (lineage not authoritative);
- detached remote target (offline/unreachable, unavailable); and
- partial semantic hydration (enumeration incomplete, partial).

Each fixture MUST remain render-stable across shell, search, diagnostics,
migration, and support projections by consuming the same parity-floor fields.

The edge-case corpus under
[`fixtures/filesystem/semantic_readiness_edge_cases/`](../../fixtures/filesystem/semantic_readiness_edge_cases/)
adds path-identity drift and save-target mismatch scenarios that require
presentation-vs-canonical evidence and export parity discipline.

## References

- Filesystem identity + readiness vocabulary:
  `docs/filesystem/filesystem_identity_vocabulary.md`
- Filesystem identity schema:
  `schemas/filesystem/save_target_token.schema.json`
- Watch fidelity and downgrade semantics:
  `docs/files/save_fallback_and_watch_fidelity_contract.md`
- Path truth chip precedent (projection parity):
  `docs/fs/path_truth_packet.md`
- Conformance packet and parity audit:
  `docs/verification/semantic_readiness_conformance_packet.md`
