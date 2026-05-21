# v1.0 support-class ledger, certified-archetype manifest, and downgrade automation

This document is the reviewer-facing companion for the gated v1.0 support-class
ledger:

- [`/artifacts/release/support_class_ledger.json`](../../artifacts/release/support_class_ledger.json)
- schema: [`/schemas/release/support_class_ledger.schema.json`](../../schemas/release/support_class_ledger.schema.json)
- proof packet:
  [`/artifacts/release/m4/support_class_ledger_proof_packet.md`](../../artifacts/release/m4/support_class_ledger_proof_packet.md)

The ledger is the **canonical truth** for which support class each subject
publishes for the v1.0 release train. It is the publication layer on top of the
stable claim matrix
([`/docs/release/stable_claim_matrix.md`](./stable_claim_matrix.md)): the matrix
decides which surfaces may publish as *Stable*; this ledger decides what *support
class* each subject publishes and narrows that class automatically when its
backing thins out. Downstream dashboards, docs, Help/About surfaces, release
packets, and support exports MUST ingest this ledger by `entry_id` rather than
cloning its status text.

The ledger does not re-mint the support-class taxonomy. The eight support classes
and their evidence-path, recovery, and badge rules live in
[`/artifacts/release/support_class_rows.yaml`](../../artifacts/release/support_class_rows.yaml)
(`taxonomy_ref`); this ledger decides which subjects have *earned* each class and
what happens when they have not.

## The certified cutline

The cutline fixes what *Certified* requires. The class vocabulary is four
positive classes (strongest first) and four distinct refusal classes:

```
certified > supported > community > experimental   |   refusal classes (rank 0)
```

- **Positive classes:** `certified`, `supported`, `community`, `experimental`. A
  ledger entry is always *put forward as* a positive class (`claimed_class`).
- **Refusal classes:** `not_certified_in_this_mode`, `not_configured`,
  `disabled_by_policy`, `not_supported`. A narrowed entry may drop to one of
  these. They are terminal (rank `0`) and never a published positive claim.

*Certified* is reserved for subjects that reference a fresh, owner-signed
certified-archetype manifest entry whose status is `certified`. An entry's
`effective_class` is what it actually publishes after narrowing; it may never be
**wider** (stronger) than its `claimed_class` — narrowing is always admissible,
widening is forbidden.

## The certified-archetype manifest

`certified_archetypes` is the manifest of scope envelopes the product is
certified for. Each entry names the `client_class`, `os_family`,
`deployment_mode`, and `locality_mode` it covers, a `certification` block (report
ref, capture date, freshness window, owner sign-off), and a
`certification_status` of `certified` or `decertified`. A Certified ledger entry
must reference one manifest entry by id; it may only **publish** Certified when
that manifest entry is `certified` and fresh.

## Ledger entries and states

Every entry carries a `claimed_class`, a `backing_stable_claim_ref` into the
stable claim matrix, an `evidence` block (refs, the required evidence path, a
freshness window, an optional `waiver`, and an `owner_signoff`), the
`ledger_state` it earned, the `active_downgrade_reasons`, and the
`effective_class` it publishes.

The `ledger_state` is the verdict for that entry:

- `published` — full, current evidence with owner sign-off; publishes the claimed
  class.
- `provisional_on_waiver` — publishes the claimed class only because an active,
  unexpired waiver covers a recorded gap.
- `narrowed_unqualified` — required backing is absent; the class must narrow.
- `narrowed_stale` — evidence exists but its freshness window expired; the class
  must narrow.
- `narrowed_waiver_expired` — the entry relied on a waiver that has expired; the
  class must narrow.

An entry whose state forces narrowing MUST drop strictly below its claimed class
and name at least one active downgrade reason. An entry that publishes its claim
MUST have current, proof-backed, owner-signed evidence with **no** active
downgrade reason, and — when Certified — must point at a certified manifest
entry.

## Downgrade reasons and the downgrade automation

The closed reason vocabulary (mirrored in the schema and the typed model) is:

- `certified_archetype_unmanifested`
- `certified_archetype_evidence_stale`
- `certified_archetype_decertified`
- `support_evidence_missing`
- `support_evidence_stale`
- `backing_stable_claim_narrowed`
- `waiver_expired`
- `owner_signoff_missing`

Each `downgrade_rule` names one reason as its `trigger_reason`, the classes it
watches (`applies_to_classes`), a `default_action`
(`narrow_published_class`, `refresh_certified_archetype`, `hold_publication`,
`request_owner_signoff`, `route_to_supported_mode`), and whether it
`blocks_publication`. A rule **fires** when any claimed entry in its watch set
carries its trigger reason. Every downgrade reason has a rule watching for it, so
a narrowing reason can never fire without a corresponding publication gate.

The automation is what makes the ledger ingest the stable line rather than
restate it. The CI gate reads the stable claim matrix named by `claim_matrix_ref`
and, for each entry with a `backing_stable_claim_ref`, fails when:

- the entry still publishes a class while its backing stable claim is narrowed
  below the stable cutline,
- the entry's backing claim is narrowed but it does not carry the
  `backing_stable_claim_narrowed` reason, or
- the entry names `backing_stable_claim_narrowed` while its backing claim still
  holds.

## Publication verdict

The `publication` block records the verdict for the v1.0 support line. It is
`hold` when any blocking downgrade rule fires and `proceed` otherwise. The
`blocking_rule_ids` and `blocking_entry_ids` enumerate the firing rules and the
entries that triggered them. The gate recomputes all three and fails on any
drift, so the verdict can never overstate readiness.

At this revision the ledger carries three subjects narrowed below their claimed
class (a decertified archetype, stale evidence, an expired waiver, and a narrowed
backing stable claim), so four blocking downgrade rules fire and the v1.0 support
publication is held. That is the honest posture: the repository is
pre-implementation and most subjects have not yet earned a published support
class.

## CI gate

Run:

```sh
python3 ci/check_support_class_ledger.py --repo-root .
```

The gate fails when a closed vocabulary or the cutline drifts; when a Certified
entry references no manifest entry or publishes against a decertified archetype;
when an entry that is not qualified does not narrow below its claimed class; when
a published entry carries an active downgrade reason or lacks evidence or owner
sign-off; when an entry overstates its posture against the `as_of` date (rides an
expired waiver, stale evidence, or a stale certified-archetype report); when an
entry's posture disagrees with its backing stable claim; when the publication
verdict or blocking sets disagree with the firing rules; when the summary counts
drift; or when a referenced artifact does not exist. It also runs negative drills
and the checked-in fixture cases under
[`/fixtures/release/support_class_ledger/`](../../fixtures/release/support_class_ledger/),
and writes a validation capture to
[`/artifacts/release/captures/support_class_ledger_validation_capture.json`](../../artifacts/release/captures/support_class_ledger_validation_capture.json).

Shiproom and release tooling can fail publication directly from this artifact:

```sh
python3 ci/check_support_class_ledger.py --repo-root . --require-proceed
```

This exits non-zero (code 2) whenever the recomputed publication verdict is
`hold`, distinct from an invalid-artifact failure (code 1).

The typed Rust consumer
(`aureline_release::support_class_ledger::current_support_class_ledger`) reads
the same ledger and runs the same structural cross-check, and exposes a
redaction-safe `support_export_projection()` for Help/About and support surfaces,
so `cargo test -p aureline-release` enforces these invariants without a cargo
build in CI.

## Update rules

1. Land qualification evidence, refreshed certified-archetype reports, and
   waivers first; point each entry's `evidence_refs`, `proof_index_ref`, and
   `certified_archetype_ref` at the canonical packets.
2. Set each entry's `ledger_state`, `active_downgrade_reasons`, and
   `effective_class` to the honest posture. An entry whose backing stable claim
   narrowed, whose evidence is stale, or whose archetype is decertified narrows
   below its claimed class.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_support_class_ledger.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower support claim than planned, narrow the claim and
   update the ledger — do not paper over the gap with prose.
