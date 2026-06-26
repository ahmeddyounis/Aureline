# Action-Label and Count/Scope-Language Parity Catalog

This document is the contract for the action-label and count/scope-language parity
catalog. The catalog is the single source of truth for the verb-first action labels
and count/scope phrases Aureline renders when an action approves, reruns, exports,
installs, applies, deletes, or publishes objects. UI buttons, batch action bars,
review sheets, toast/activity rows, CLI/help summaries, export/report headings,
confirmation dialogs, support exports, docs, and narrated surfaces resolve their
labels and disclosures through this catalog rather than inlining a literal verb or
scope word.

Where the
[count/scope/freshness microcopy grammar](../../copy/count_scope_freshness_grammar.md)
freezes the controlled scope terms (`selected`, `visible`, `loaded`, `all matching`,
`hidden by policy`, `outside current workset`) and the
[safety-critical string catalog](./m5_safety_critical_string_catalog.md) freezes the
trust/policy/runtime wording, this catalog freezes the *actual* verb-first action
labels and the disclosures that state how many objects each scope holds — so a
primary action can never hide its scope, side effect, or selection class behind a
vague verb such as `Continue`, `Accept`, or `Submit`.

- Record kind: `m5_action_label_scope_catalog`
- Schema: [`schemas/content/m5-action-label-scope.schema.json`](../../../schemas/content/m5-action-label-scope.schema.json)
- Canonical support export: [`artifacts/content/m5-action-label-proof/support_export.json`](../../../artifacts/content/m5-action-label-proof/support_export.json)
- Summary artifact: [`artifacts/content/m5-action-label-proof/m5_action_label_scope_parity.md`](../../../artifacts/content/m5-action-label-proof/m5_action_label_scope_parity.md)
- Fixtures: [`fixtures/content/m5-action-label-scope/`](../../../fixtures/content/m5-action-label-scope/)
- Producer: `aureline_shell::m5_action_label_scope_parity::current_action_label_scope_catalog_export`
- Headless emitter: `aureline_shell_m5_action_label_scope`

## Controlled registries

The catalog carries three controlled registries. Labels and disclosures reference
their members by locale-neutral id and never inline a verb, scope, or noun as a
literal string.

- **Scopes** — one [`ScopeDefinition`] per scope class. The scope ids match the
  controlled count/scope term ids the grammar owns: `selected`, `visible`, `loaded`,
  `all_matching`, `hidden_by_policy`, `outside_current_workset`, plus a
  `single_object` scope for single-target actions. Each definition agrees with its
  class on whether it is `actionable` (may be an action target), whether it is an
  `is_exclusion` (disclosed but never acted on — `hidden_by_policy` and
  `outside_current_workset`), and whether it `requires_count`.
- **Verbs** — one [`ActionVerb`] per imperative verb: `approve`, `rerun`, `apply`,
  `delete`, `export`, `install`, `publish`. Each verb carries a reversibility class
  (`reversible`, `undoable_window`, `irreversible`) and a default mutation class.
- **Objects** — one [`ActionObject`] per noun class, with a singular and plural
  label, so every action narrows the object class it mutates.

## Action labels

An [`ActionLabel`] is verb-first. Its `reference_label` is a template built from
these placeholders, which resolve against the registries:

- `{verb}` — the verb's canonical imperative label.
- `{count:<name>}` — a named count slot, kept as `{<name>}` for the renderer to fill.
- `{scope:<scope_id>}` — the scope's canonical phrase.
- `{object_one}` / `{object_many}` — the object's singular / plural noun.

Every label is checked against these hard invariants:

- **Verb-first** — the first template segment is `{verb}`.
- **No ambiguous default** — the verb is never one of the banned tokens
  (`continue`, `accept`, `submit`, `ok`, `confirm`, `proceed`, `done`, `go`, `yes`,
  `next`, `finish`), and the rendered first word is never one either.
- **Scope declared** — an actionable multi-object scope (`selected`, `visible`,
  `loaded`, `all_matching`) names its scope phrase, unless
  `scope_unambiguous_in_sheet` is true because the surrounding review sheet already
  shows the scope. The narrated `screen_reader_label` always names the scope even
  when the visible button relies on the sheet.
- **Object narrowed** — the template names the object class; a verb alone is never a
  complete label.
- **Count present** — a counted scope carries a `count_var`, and the template's
  `{count:...}` slot agrees with it.
- **Review state carried** — approval and batch-mutation labels declare a review
  state other than `no_review_needed`, so a broad approval cannot hide that it runs
  over unreviewed objects.
- **Side effect disclosed** — destructive, publish, and install labels disclose
  their side effect.

So `Approve 5 selected changes`, `Rerun 12 visible tasks`,
`Delete 3 selected files`, and `Approve all matching changes` are valid, while a
bare `Continue`, `Accept`, `Submit`, `Apply`, or `OK` on the same surface is not.

## Count/scope disclosures

A [`ScopeDisclosure`] states how many objects each scope holds, reusing the same
controlled scope phrases. It declares a primary scope, any disclosed (typically
excluded) scopes, a `count_status` (`exact`, `approximate`, `partial`, `cached`,
`stale`, `streaming`, `warming`, `unknown`), and the named counts its template
fills. A disclosure must name every scope and count it declares, so it can never
claim a population it does not show:

```text
3 selected changes (exact); 2 hidden by policy, 1 outside current workset not included.
1,240 all matching findings (approx.); 12 hidden by policy withheld.
84 loaded of 1,240 all matching results (partial).
```

The same phrase set spans the batch action bar, review sheet, toast/activity row,
CLI/help summary, and export/report heading, so a count cannot mean one thing on a
bar and another in an export.

## Cross-surface reuse and consumer parity

The `shared_scope_phrase_ids` set names the scope phrases that must appear on at
least three distinct surfaces; the catalog proves this with `cross_surface_reuse`.
The `consumer_projection` block asserts that product UI, CLI/help, docs, support
export, screen readers, and the activity feed all reuse the catalog labels, and the
catalog requires enough labels to carry product-UI, docs, and support-export parity
together so the language docs and exports show is the language the product used at
runtime.

## Localization and offline parity

Machine-facing identity stays locale-neutral. Label ids, verb/scope/object ids,
count-variable names, and the `{...}` placeholders are lowercase ascii
(`[a-z0-9_.]`); only the canonical verb labels, scope phrases, object nouns, and
templates carry human prose. The localized overlay fixture pseudo-localizes that
prose while keeping every id and placeholder byte-for-byte identical, and the
offline-mirror fixture keeps the labels and disclosures intact, so a translation or
an air-gapped mirror can never fork the meaning of a scope or hide the object class
an action mutates.

## Validation and proof

`ActionLabelScopeCatalog::validate` returns the full set of violations (see
[`mod.rs`](../../../crates/aureline-shell/src/m5_action_label_scope_parity/mod.rs)).
The inline tests, the checked support export, and the two fixtures are all minted
from one seed builder, so the in-code catalog, the artifact, and the fixtures cannot
drift. The proof-freshness block auto-narrows the catalog claim when its proof falls
out of the freshness SLO.
