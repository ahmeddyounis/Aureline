# Attention-routing qualification

This document describes the **certification** that decides whether Aureline's shell,
companion, and operator surfaces may advertise their attention-routing claims — and narrows
those claims automatically the moment the underlying evidence goes stale or failing. It turns
attention quality from prose that surfaces restate by hand into a **derived, governed claim**
bound to the proof packets that keep it true.

Where the [attention-routing matrix](./m5-attention-routing.md) *names and freezes the object
model* and each sibling lane — the [notification envelope](./m5-envelope-routing.md), the
durable [activity object](./m5-activity-objects.md), the
[action/retention engine](./m5-attention-actions.md), the
[quiet-hours / suppression policy](./m5-quiet-hours-suppression.md), the
[badge aggregate](./m5-badge-aggregates.md), and the [fanout receipt](./m5-fanout-receipts.md)
— *proves one attention family* with a checked-in fixture and a freeze gate, this lane records
**which claims depend on which families, and what happens to a claim when one family's proof is
no longer current.**

The track invariant this lane protects: **no claimed shell, companion, operator, or managed row
stays green when notification-routing, activity-object, badge, fanout, or quiet-hours/privacy
evidence is stale or failing.** A profile cannot stay green because an independent surface (OS
notifications, an operator dashboard, chronology reuse) happens to exist while the underlying
routing, privacy, or dedupe semantics are stale.

If this document, the companion schema, and the worked fixture disagree, the normative sources
in `.t2/docs/` win and this document plus its companions update in the same change.

## One qualification row per claimed family

[`attention_qualification_bundle`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs)
publishes a [`FamilyQualificationRow`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs)
for each of the seven governed attention families. Each row names the family's published proof
packet — the boundary schema, the checked-in fixture, and the freeze gate that keeps it current
— the [release-evidence rows](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs)
it covers, and its current [`EvidenceState`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs).
The family identifiers, record kinds, schema refs, and freeze-gate refs are reused directly from
the sibling lanes' own public constants, so a family that renames its bundle id breaks this
certification's build in the same change.

The canonical bundle freezes every family `fresh`. Release automation overrides the state from
the live freeze-gate result: `stale` when the proof is older than its freshness window,
`failing` when the gate fails, and `missing` when the proof packet is absent.

## Claims are derived, never asserted

Each claimed [`ClaimedProfile`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs)
— shell, companion, and operator — declares the families it depends on. Its
[`ClaimState`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs) is computed
by [`evaluate_profile_claim`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs)
as the worst of its dependencies' evidence severities:

| Worst dependency | Claim state | Meaning |
| --- | --- | --- |
| all `fresh` | `full` | advertised at full strength |
| any `stale` | `narrowed` | degraded claim while the underlying objects still exist |
| any `failing` / `missing` | `withdrawn` | claim withdrawn until the proof is restored |

`narrowed_by` names every dependency whose evidence is not fresh, in family order, so a narrowed
or withdrawn claim always names its cause. Because the published row equals a fresh
re-evaluation, the bundle's `validate()` rejects any hand-edit that asserts a claim wider than
its evidence — a claim can only be derived.

The shell depends on every family; the companion and operator depend on the subset they actually
render. The routing matrix, the quiet-hours/suppression policy, and the fanout receipt are shared
spines every profile depends on, so **routing, privacy, or fanout** proof going stale narrows
every claim — the acceptance-criteria trio.

## Wiring into release automation

[`recompute_profiles`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs) is the
release-automation entry point. Given the family rows and a set of live evidence states, it
returns each profile's derived claim row — `full`, `narrowed`, or `withdrawn`. Release evidence
packets carry one row per family plus the derived per-profile claim, so a stale or failing
attention proof automatically narrows the affected shell, companion, or operator claim without a
human restating it.

The narrowing and withdrawal behavior is itself frozen as computed invariants
(`attention_qualification.stale_dependency_narrows`,
`attention_qualification.failing_dependency_withdraws`): the bundle exercises the evaluator over
every `(profile, family)` pair and confirms that perturbing one family moves exactly the
dependent profiles and leaves the rest at full strength.

## One projection for every consumer

[`projection()`](../../crates/aureline-activity/src/m5_attention_qualification/mod.rs) is the
support-export-safe view that About/Help, the activity center, support export, the compatibility
report, and release and public-truth surfaces all read instead of restating attention quality
claims. It lists each profile's derived claim and published sentence, and each family's evidence
state and proof refs — opaque object refs and stable tokens only, never message bodies or
payloads.

## What this lane does not do

Certification narrows a marketable claim; it never silences a security advisory. Every family
preserves the security-escalation escape (`attention_qualification.security_never_silenced`), and
the underlying suppression, badge, and fanout lanes keep that guarantee. This lane stays inside
M5 attention-routing, durable-job, and reopen-truth for already-claimed rows; it does not widen
into a messaging platform, a chat system, or a general-purpose mobile-notification product.
