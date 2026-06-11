# M5 install/update review sheets

This document describes the canonical packet that freezes the **M5 install/update
review sheets** — one reviewed change model per install or update of a marketed M5
artifact family. It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-install-review.json` and the typed model in the
`aureline-ecosystem` crate (`m5_install_review`).

Where the
[`M5 ecosystem install-governance matrix`](m5-ecosystem-install-governance-matrix.md)
freezes one governance row per marketed artifact family and the
[`M5 marketplace fact-views`](m5-marketplace-fact-views.md) project that truth into
the storefront, this packet freezes how an install or update is **reviewed before
commit**. It turns install and update from a generic download action into one
coherent surface that compares a package's current effective revision with the
proposed one and makes every consequential change explicit.

## What each sheet compares

A review sheet pairs a **current** revision (absent for a fresh `install`) with a
**proposed** revision and surfaces the delta between them. Each revision pins its
`runtime_origin`, `host_class`, `source_class`, `publisher_ref`, `signing_root_ref`,
`namespace_ref`, `compatibility_label`, and `support_class`. On top of the pair, the
sheet records:

- **Permission deltas** — a list of capability deltas, each marked
  `required` versus `optional`, `direct` versus `transitive`, and `added`,
  `removed`, `unchanged`, `widened`, or `narrowed`. Transitive capability widening
  is named (`origin: transitive`, `change: added`/`widened`) rather than buried in a
  dependency.
- **Runtime-origin and host-class changes** — moving a package from a signed build
  to a bridge or local-model runtime, or from a `local` host to a `managed_workspace`,
  `remote_host`, or `container` host, is computed from the current/proposed pair.
- **Publisher continuity** — a `publisher_transfer` state (`same_publisher`,
  `transferred_verified`, `transferred_unverified`, `publisher_unknown`,
  `not_applicable`), a `signing_root_continuity` (`continuous`, `rotated_disclosed`,
  `root_changed`, `unsigned`, `not_applicable`), and a `namespace_state` (`stable`,
  `renamed`, `orphaned`, `reclaimed`, `not_applicable`).
- **Compatibility-floor change** — `unchanged`, `relaxed`, `regressed`,
  `regressed_to_unsupported`, or `initial` (fresh install), computed from the current
  and proposed `compatibility_label`.
- **Restart/open-work implications** — a `restart_impact` (`no_restart`,
  `reattach_required`, `host_restart_required`, `app_restart_required`) and an
  `open_work_impact` (`no_impact`, `background_reindex`, `open_editors_affected`,
  `active_session_interrupted`).
- **Rollback** — a `rollback` plan with a durable `checkpoint_handle_ref`, a
  `rollback_posture`, a current-package `fallback_package_ref`, and whether a
  `checkpoint_created` checkpoint exists, so recovery is intentional rather than an
  ad hoc reinstall.

Every sheet carries a `governance_family_ref` that resolves to its row in the
install-governance matrix, so the review sheet builds on the same family truth the
marketplace surfaces use.

## The commit gate is recomputed, not stored by hand

The `commit_disposition` a sheet publishes — `one_click_allowed`,
`unified_review_required`, or `blocked` — is **not** hand-entered. It and the
`review_triggers` set are recomputed from the sheet's facts, and the stored values
must equal that recomputation or validation fails. Each trigger forces a minimum
disposition and the sheet takes the strictest:

- **`permissions_widened`** (forces `unified_review_required`) — any capability delta
  is `added` or `widened`.
- **`publisher_discontinuity`** (forces `unified_review_required`) — the publisher
  transferred, the signing root changed, or the namespace was orphaned or reclaimed.
- **`runtime_origin_changed`** / **`host_class_changed`** (force
  `unified_review_required`) — the proposed runtime origin or host class differs from
  the current one.
- **`compatibility_floor_regressed`** (forces `unified_review_required`) — the
  compatibility floor regresses but stays supported.
- **`compatibility_unsupported`** (forces `blocked`) — the proposed revision is
  unsupported on the target.
- **`restart_or_reattach_required`** (forces `unified_review_required`) — committing
  needs a restart or reattach.
- **`open_work_impacted`** (forces `unified_review_required`) — committing affects
  open editors or interrupts an active session.
- **`rollback_not_established`** (forces `unified_review_required`) — a state-changing
  commit has an unverified or irreversible rollback, or no checkpoint.

## The guardrail: no one-click commit on widening, publisher, or runtime change

This is the lane guardrail. Because widened permissions, a publisher discontinuity,
and a runtime-origin change each force at least `unified_review_required`, a one-click
install/update can never be offered on any of them — the unified review sheet must be
acknowledged first. A regression to an unsupported target is stronger still: it
`blocked`s the commit, and the packet refuses to expose an enabled `commit` action on
a blocked sheet.

## Scopes stay visibly distinct

Each sheet declares a `scope` of `workspace`, `profile`, or `global`, and **every**
action on the sheet must carry that same scope. This keeps a local troubleshooting
moment from silently widening or narrowing the wrong scope: a workspace-scoped retry
can never apply at global scope. Rollback checkpoint creation, current-package
fallback, scope-specific disable, and retry are all offered through the same reviewed
model rather than separate hidden dialogs.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each sheet's package
kind, change kind, scope, commit disposition, review-trigger tokens, compatibility
floor change, and whether the change widens permissions or allows a one-click commit,
plus `unified_review_required_count` and `blocked_count`. Support bundles and
docs/help should ingest this projection directly rather than re-describing install
state by hand, so the product, support exports, and release evidence all cite the
same sheets.

## Validation

`M5InstallReview::validate()` reports every violation, including an unsupported schema
version or record kind, non-canonical closed vocabularies, empty required fields,
duplicate sheet ids, a change kind that expects a current revision but has none (or a
fresh install that carries one), a publisher-transfer state that disagrees with the
publisher refs, a state-changing rollback plan missing a durable ref, an action
scoped outside its sheet's scope, a missing required `commit`/`cancel` action, a
blocked sheet that still enables its commit action, a compatibility-floor change,
review-trigger set, or commit disposition that disagrees with the recomputation, and
a summary block that disagrees with the sheets.
